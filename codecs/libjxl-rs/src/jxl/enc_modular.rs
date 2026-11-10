// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Sample conversion and transform/tree helpers from lib/jxl/enc_modular.cc.
//! Frame stream state uses the null-pool configuration. Pixel preparation is
//! translated for enc_frame::LosslessThunderConfig's fixed source path; general
//! pixel preparation and VarDCT entry points remain unported.
//! Owning vectors replace their C++ owners;
//! borrowed custom parameter views retain their original lifetime obligations.
//! Disjoint slices replace the source's row pointers. Source-undefined floating
//! conversions and shifts remain caller preconditions, checked with assertions.
//! A failed custom-float conversion retains its already written output prefix.

use super::base::bits::ceil_log2_nonzero;
use super::base::float::{min_f32, mul_add_f32, mul_add_f64};
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::common::SpeedTier;
use super::dec_ans::HybridUintConfig;
use super::dec_modular::ModularStreamId;
use super::enc_ans::{
    EntropyEncodingData, Token, build_and_encode_histograms, write_tokens_with_allotment,
};
use super::enc_ans_params::{Histogram, HistogramParams};
use super::enc_aux_out::{AuxOut, LayerType};
use super::enc_bit_writer::BitWriter;
use super::enc_frame::LosslessThunderConfig;
use super::enc_modular_simd::estimate_cost;
use super::enc_params::CompressParams;
use super::enc_progressive_split::ProgressiveSplitter;
use super::fields::bundle;
use super::frame_dimensions::FrameDimensions;
use super::frame_header::{ColorTransform, FrameHeader};
use super::image::{Image3, Plane};
use super::modular::encoding::context_predict::{
    NUM_NONREF_PROPERTIES, predictor_has_weighted, weighted,
};
use super::modular::encoding::dec_ma::{PropertyDecisionNode, Tree};
use super::modular::encoding::enc_encoding::{
    learn_tree, modular_compress, modular_generic_compress, predefined_tree,
};
use super::modular::encoding::enc_ma::tokenize_tree;
use super::modular::encoding::encoding::{GroupHeader, validate_channel_dimensions};
use super::modular::encoding::ma_common::NUM_TREE_CONTEXTS;
use super::modular::modular_image::{Channel, Image};
use super::modular::options::{
    ModularMultiplierInfo, ModularOptions, Predictor, TreeKind, TreeMode, UNDEFINED_PREDICTOR,
};
use super::modular::transform::enc_rct::fwd_rct_channels;
use super::modular::transform::enc_transform::{compute_minmax, transform_forward};
use super::modular::transform::transform::{Transform, TransformId};
use super::pack_signed::pack_signed;

/// Internal source state; these fields are exposed for differential fixtures,
/// like other translated codec-internal storage. This is not the C API.
pub struct ModularFrameEncoder<'a> {
    pub ac_metadata_size: Vec<usize>,
    pub extra_dc_precision: Vec<u8>,
    pub stream_images: Vec<Image>,
    pub stream_options: Vec<ModularOptions>,
    pub quants: Vec<u32>,
    pub tree: Tree,
    pub tree_tokens: Vec<Vec<Token>>,
    pub stream_headers: Vec<GroupHeader>,
    pub tokens: Vec<Vec<Token>>,
    pub code: EntropyEncodingData,
    pub context_map: Vec<u8>,
    pub frame_dim: FrameDimensions,
    pub cparams: CompressParams<'a>,
    pub tree_splits: Vec<usize>,
    pub gi_channel: Vec<Vec<u32>>,
    pub image_widths: Vec<usize>,
    pub stream_params: Vec<GroupParams>,
}

#[derive(Clone, Copy)]
pub struct GroupParams {
    pub rect: Rect,
    pub min_shift: i32,
    pub max_shift: i32,
    pub id: ModularStreamId,
}

impl<'a> ModularFrameEncoder<'a> {
    pub fn create(
        frame_header: &FrameHeader<'_>,
        cparams_orig: &CompressParams<'a>,
        streaming_mode: bool,
    ) -> StatusOr<Self> {
        let mut result = Self {
            ac_metadata_size: Vec::new(),
            extra_dc_precision: Vec::new(),
            stream_images: Vec::new(),
            stream_options: Vec::new(),
            quants: Vec::new(),
            tree: Vec::new(),
            tree_tokens: Vec::new(),
            stream_headers: Vec::new(),
            tokens: Vec::new(),
            code: EntropyEncodingData::default(),
            context_map: Vec::new(),
            frame_dim: FrameDimensions::default(),
            cparams: CompressParams::default(),
            tree_splits: Vec::new(),
            gi_channel: Vec::new(),
            image_widths: Vec::new(),
            stream_params: Vec::new(),
        };
        result.init(frame_header, cparams_orig, streaming_mode)?;
        Ok(result)
    }

    fn init(
        &mut self,
        frame_header: &FrameHeader<'_>,
        cparams_orig: &CompressParams<'a>,
        streaming_mode: bool,
    ) -> Status {
        self.frame_dim = frame_header.to_frame_dimensions();
        self.cparams = cparams_orig.clone();
        let num_streams =
            ModularStreamId::num(&self.frame_dim, frame_header.passes.num_passes as usize);
        let p = &mut self.cparams;
        if p.responsive == 1 && p.is_lossless() && p.decoding_speed_tier == 1 {
            p.decoding_speed_tier = 2;
        }
        #[expect(
            clippy::collapsible_if,
            reason = "Preserve the source's nested progressive-lossless policy."
        )]
        if p.responsive == 1 && p.is_lossless() {
            if p.colorspace < 0 {
                p.colorspace = 6;
            }
        }
        if p.modular_part_is_lossless() {
            let disable_wp = |p: &mut CompressParams<'_>| {
                p.options.wp_tree_mode = TreeMode::NoWP;
                if p.options.predictor == Predictor::WEIGHTED {
                    p.options.predictor = Predictor::GRADIENT;
                }
            };
            match p.decoding_speed_tier {
                0 => p.options.fast_decode_multiplier = 1.001,
                1 => {
                    p.options.fast_decode_multiplier = 1.005;
                    disable_wp(p);
                }
                2 => {
                    p.options.fast_decode_multiplier = 1.015;
                    disable_wp(p);
                }
                3 => {
                    p.options.wp_tree_mode = TreeMode::GradientOnly;
                    p.options.predictor = Predictor::GRADIENT;
                }
                _ => {
                    p.options.wp_tree_mode = TreeMode::GradientOnly;
                    p.options.predictor = Predictor::GRADIENT;
                    p.options.nb_repeats = 0.0;
                }
            }
        }
        for _ in 0..num_streams {
            self.stream_images.push(Image::default());
        }
        if p.responsive < 0 {
            p.responsive = if p.modular_part_is_lossless() { 0 } else { 1 };
        }
        p.options.splitting_heuristics_node_threshold = ((75 + 14 * p.speed_tier as i32) as usize)
            .wrapping_add(10usize.wrapping_mul(p.decoding_speed_tier))
            as f32;
        {
            let mut prop_order = if p.responsive != 0 {
                vec![0, 1, 4, 5, 6, 7, 8, 15, 9, 10, 11, 12, 13, 14, 2, 3]
            } else {
                vec![0, 1, 15, 9, 10, 11, 12, 13, 14, 2, 3, 4, 5, 6, 7, 8]
            };
            if p.responsive == 0
                && num_streams < 30
                && p.speed_tier > SpeedTier::Tortoise
                && cparams_orig.modular_part_is_lossless()
            {
                prop_order.remove(1);
            }
            let metadata = &frame_header
                .nonserialized_metadata
                .expect("source Init requires metadata")
                .m;
            let max_properties = p.options.max_properties.min(
                (metadata.num_extra_channels as i32)
                    .checked_add(if metadata.color_encoding.is_gray() {
                        0
                    } else {
                        2
                    })
                    .expect("source property count must fit int"),
            );
            match p.speed_tier {
                SpeedTier::Hare => {
                    p.options.splitting_heuristics_properties = prop_order[..4].to_vec();
                    p.options.max_property_values = 48;
                    p.options.nb_repeats *= 0.5;
                }
                SpeedTier::Wombat => {
                    p.options.splitting_heuristics_properties = prop_order[..5].to_vec();
                    p.options.max_property_values = 64;
                    p.options.nb_repeats *= 0.7;
                }
                SpeedTier::Squirrel => {
                    p.options.splitting_heuristics_properties = prop_order[..7].to_vec();
                    p.options.max_property_values = 96;
                }
                SpeedTier::Kitten => {
                    p.options.splitting_heuristics_properties = prop_order[..10].to_vec();
                    p.options.max_property_values = 128;
                    p.options.nb_repeats *= 1.1;
                }
                SpeedTier::Glacier | SpeedTier::Tortoise => {
                    p.options.splitting_heuristics_properties = prop_order;
                    p.options.max_property_values = 256;
                    p.options.nb_repeats *= 1.3;
                }
                _ => {
                    p.options.splitting_heuristics_properties = prop_order[..3].to_vec();
                    p.options.max_property_values = 32;
                    p.options.nb_repeats *= 0.3;
                }
            }
            if p.speed_tier > SpeedTier::Tortoise {
                for i in 0..max_properties {
                    p.options
                        .splitting_heuristics_properties
                        .push((NUM_NONREF_PROPERTIES + i as usize * 4 + 3) as u32);
                }
            } else {
                for i in 0..max_properties
                    .checked_mul(4)
                    .expect("source property count must fit int")
                {
                    p.options
                        .splitting_heuristics_properties
                        .push((NUM_NONREF_PROPERTIES + i as usize) as u32);
                }
            }
        }
        p.options.nb_repeats = min_f32(1.0, p.options.nb_repeats);
        if matches!(
            p.options.predictor,
            Predictor::AVERAGE0
                | Predictor::AVERAGE1
                | Predictor::AVERAGE2
                | Predictor::AVERAGE3
                | Predictor::AVERAGE4
                | Predictor::WEIGHTED
        ) && !p.modular_part_is_lossless()
        {
            p.options.predictor = UNDEFINED_PREDICTOR;
        }
        if p.options.predictor == UNDEFINED_PREDICTOR {
            if (p.speed_tier <= SpeedTier::Glacier || !p.modular_mode)
                && p.is_lossless()
                && p.responsive == 0
            {
                p.options.predictor = Predictor::VARIABLE;
            } else if p.responsive != 0 || p.lossy_palette {
                p.options.predictor = Predictor::ZERO;
            } else if !p.is_lossless() {
                p.options.predictor = Predictor::GRADIENT;
            } else if p.speed_tier < SpeedTier::Falcon {
                p.options.predictor = Predictor::BEST;
            } else if p.speed_tier == SpeedTier::Falcon {
                p.options.predictor = Predictor::WEIGHTED;
            } else if p.speed_tier > SpeedTier::Falcon {
                p.options.predictor = Predictor::GRADIENT;
            }
        } else if p.lossy_palette {
            p.options.predictor = Predictor::ZERO;
        }
        if !p.modular_part_is_lossless()
            && matches!(
                p.options.predictor,
                Predictor::WEIGHTED | Predictor::VARIABLE | Predictor::BEST
            )
        {
            p.options.predictor = Predictor::ZERO;
        }
        self.tree_splits.push(0);
        if !p.modular_mode {
            let qt0 = ModularStreamId::quant_table(0)?;
            p.options.fast_decode_multiplier = 1.0;
            self.tree_splits.extend([
                ModularStreamId::var_dct_dc(0).id(&self.frame_dim),
                ModularStreamId::modular_dc(0).id(&self.frame_dim),
                ModularStreamId::ac_metadata(0).id(&self.frame_dim),
                qt0.id(&self.frame_dim),
                ModularStreamId::modular_ac(0, 0).id(&self.frame_dim),
            ]);
            self.ac_metadata_size
                .resize(self.frame_dim.num_dc_groups, 0);
            self.extra_dc_precision
                .resize(self.frame_dim.num_dc_groups, 0);
        }
        self.tree_splits.push(num_streams);
        p.options.max_chan_size = self.frame_dim.group_dim;
        p.options.group_dim = self.frame_dim.group_dim;
        self.stream_options.resize(num_streams, p.options.clone());
        self.stream_options[0] = p.options.clone();
        if p.speed_tier == SpeedTier::Falcon {
            self.stream_options[0].tree_kind = TreeKind::WPFixedDC;
        } else if p.speed_tier == SpeedTier::Thunder {
            self.stream_options[0].tree_kind = TreeKind::GradientFixedDC;
        }
        self.stream_options[0].histogram_params =
            HistogramParams::for_modular(p, &[], streaming_mode);
        Ok(())
    }

    /// ComputeEncodingData's reached branches for the sealed configuration.
    /// The owning frame alone can call this method; its private parameters and
    /// metadata fix do_color=true, groupwise=false, integer samples, no effects,
    /// optional same-depth alpha, no XYB, no quantization, and no responsive squeeze.
    pub(super) fn compute_encoding_data_lossless_thunder(
        &mut self,
        config: &LosslessThunderConfig,
        frame_header: &FrameHeader<'_>,
        splitter: &ProgressiveSplitter,
        color: &Image3<f32>,
        extra_channels: &[Plane<f32>],
    ) -> Status {
        let metadata = &config.metadata().m;
        let patch_dim = frame_header.to_frame_dimensions();
        let xsize = patch_dim.xsize;
        let ysize = patch_dim.ysize;
        let group_rect = Rect::new(0, 0, color.xsize(), color.ysize());
        let frame_area_rect = Rect::new(0, 0, xsize, ysize);
        assert!(color.xsize() >= xsize && color.ysize() >= ysize);
        let color_channels = if metadata.color_encoding.is_gray() {
            1
        } else {
            3
        };
        let nb_chans = color_channels + extra_channels.len();
        let mut max_bitdepth = metadata.bit_depth.bits_per_sample as i32 + 1;
        self.stream_images[0] = Image::create(
            xsize,
            ysize,
            metadata.bit_depth.bits_per_sample as i32,
            nb_chans as i32,
        )?;
        let mut maxval = (1u32 << self.stream_images[0].bitdepth) as i32 - 1;
        for c in 0..3 {
            if metadata.color_encoding.is_gray() && c != 0 {
                continue;
            }
            let ch = &mut self.stream_images[0].channel[c];
            let factor = f64::from(maxval);
            let bits = metadata.bit_depth.bits_per_sample;
            let exp_bits = metadata.bit_depth.exponent_bits_per_sample;
            ch.hshift = frame_header.chroma_subsampling.h_shift(c) as i32;
            ch.vshift = frame_header.chroma_subsampling.v_shift(c) as i32;
            let xsize_shifted = xsize.div_ceil(1 << ch.hshift);
            let ysize_shifted = ysize.div_ceil(1 << ch.vshift);
            ch.shrink_to(xsize_shifted as i32, ysize_shifted as i32)?;
            for y in 0..ysize_shifted {
                let row_in = &color.plane_row(c, y + group_rect.y0())[group_rect.x0()..];
                float_to_int(
                    row_in,
                    ch.row_mut(y),
                    xsize_shifted,
                    bits,
                    exp_bits,
                    false,
                    factor,
                )?;
            }
        }
        let mut c = color_channels;
        for (ec, extra) in extra_channels.iter().enumerate() {
            let eci = &metadata.extra_channel_info[ec];
            let ecups = frame_header.extra_channel_upsampling[ec] as usize;
            let ch = &mut self.stream_images[0].channel[c];
            ch.shrink_to(
                patch_dim.xsize_upsampled.div_ceil(ecups) as i32,
                patch_dim.ysize_upsampled.div_ceil(ecups) as i32,
            )?;
            ch.hshift =
                ceil_log2_nonzero(ecups) as i32 - ceil_log2_nonzero(frame_header.upsampling) as i32;
            ch.vshift = ch.hshift;
            let bits = eci.bit_depth.bits_per_sample;
            let exp_bits = eci.bit_depth.exponent_bits_per_sample;
            let fp = eci.bit_depth.floating_point_sample;
            let factor = if fp {
                1.0
            } else {
                f64::from((1u32 << bits) - 1)
            };
            max_bitdepth = max_bitdepth.max(bits as i32 + i32::from(!fp));
            for y in 0..ch.plane.ysize() {
                let row_in = &extra.row(y + group_rect.y0())[group_rect.x0()..];
                let width = ch.plane.xsize();
                float_to_int(row_in, ch.row_mut(y), width, bits, exp_bits, fp, factor)?;
            }
            c += 1;
        }
        if c != nb_chans {
            return Err(StatusCode::GenericError);
        }
        let level_max_bitdepth = if self.cparams.level == 5 { 16 } else { 32 };
        if max_bitdepth > level_max_bitdepth {
            return Err(StatusCode::GenericError);
        }
        // Both source palette guards are true for this fixed configuration.
        // Preserve default palette/compaction policy and all transform attempts.
        try_palettes(
            &mut self.stream_images[0],
            &mut max_bitdepth,
            &mut maxval,
            &self.cparams,
            self.cparams.channel_colors_pre_transform_percent,
        )?;
        let gi = &mut self.stream_images[0];
        #[expect(
            clippy::collapsible_if,
            reason = "Preserve the source's separate RCT eligibility and selection checks."
        )]
        if gi.channel.len() - gi.nb_meta_channels >= 3 && max_bitdepth + 1 < level_max_bitdepth {
            if self.cparams.colorspace < 0 {
                let mut ycocg = Transform::new(TransformId::RCT);
                ycocg.rct_type = 6;
                ycocg.begin_c = gi.nb_meta_channels as u32;
                do_transform(gi, &ycocg, &weighted::Header::default(), false);
                max_bitdepth += 1;
            }
        }
        if max_bitdepth + 1 > level_max_bitdepth {
            self.cparams.colorspace = 0;
        }
        if max_bitdepth > level_max_bitdepth {
            return Err(StatusCode::GenericError);
        }

        for group_id in 0..patch_dim.num_dc_groups {
            let rgx = group_id % patch_dim.xsize_dc_groups;
            let rgy = group_id / patch_dim.xsize_dc_groups;
            let rect = Rect::new(
                rgx * patch_dim.dc_group_dim,
                rgy * patch_dim.dc_group_dim,
                patch_dim.dc_group_dim,
                patch_dim.dc_group_dim,
            );
            let gx = rgx + frame_area_rect.x0() / 2048;
            let gy = rgy + frame_area_rect.y0() / 2048;
            let real_group_id = gy * self.frame_dim.xsize_dc_groups + gx;
            self.stream_params.push(GroupParams {
                rect,
                min_shift: 3,
                max_shift: 1000,
                id: ModularStreamId::modular_dc(real_group_id),
            });
        }
        for group_id in 0..patch_dim.num_groups {
            let rgx = group_id % patch_dim.xsize_groups;
            let rgy = group_id / patch_dim.xsize_groups;
            let rect = Rect::new(
                rgx * patch_dim.group_dim,
                rgy * patch_dim.group_dim,
                patch_dim.group_dim,
                patch_dim.group_dim,
            );
            let gx = rgx + frame_area_rect.x0() / self.frame_dim.group_dim;
            let gy = rgy + frame_area_rect.y0() / self.frame_dim.group_dim;
            let real_group_id = gy * self.frame_dim.xsize_groups + gx;
            for pass in 0..splitter.get_num_passes() {
                let (min_shift, max_shift) = frame_header.passes.get_downsampling_bracket(pass);
                self.stream_params.push(GroupParams {
                    rect,
                    min_shift,
                    max_shift,
                    id: ModularStreamId::modular_ac(real_group_id, pass),
                });
            }
        }
        if self.stream_params.len() == 2 {
            self.stream_params.push(GroupParams {
                rect: Rect::new(0, 0, xsize, ysize),
                min_shift: 0,
                max_shift: 1000,
                id: ModularStreamId::global(),
            });
        }
        self.gi_channel
            .resize_with(self.stream_images.len(), Vec::new);
        for i in 0..self.stream_params.len() {
            let group = self.stream_params[i];
            let stream = group.id.id(&self.frame_dim);
            if stream != 0 {
                self.stream_options[stream] = self.stream_options[0].clone();
            }
            self.prepare_stream_params(
                &group.rect,
                group.min_shift,
                group.max_shift,
                &group.id,
                true,
                false,
            )?;
        }
        let gi = &mut self.stream_images[0];
        let mut ch = gi.nb_meta_channels;
        while ch < gi.channel.len() {
            let fc = &gi.channel[ch];
            if fc.w > self.frame_dim.group_dim || fc.h > self.frame_dim.group_dim {
                break;
            }
            ch += 1;
        }
        for channel in &mut gi.channel[ch..] {
            // Source releases the plane while retaining logical channel sizes.
            channel.plane = Plane::default();
        }
        validate_channel_dimensions(gi, &self.stream_options[0])
    }

    /// The source's private caller always passes its own cparams member.
    /// Borrow that field directly so the same owner need not be aliased or copied.
    pub fn prepare_stream_params(
        &mut self,
        rect: &Rect,
        min_shift: i32,
        max_shift: i32,
        stream: &ModularStreamId,
        do_color: bool,
        groupwise: bool,
    ) -> Status {
        let cparams = &self.cparams;
        let s = stream.id(&self.frame_dim);
        if s == 0 && self.frame_dim.num_groups != 1 {
            return Ok(());
        }
        if s > 0 {
            // The group owner is disjoint from the global image owner.
            let (before, after) = self.stream_images.split_at_mut(s);
            let full_image = &before[0];
            let gi = &mut after[0];
            *gi = Image::create(rect.xsize(), rect.ysize(), full_image.bitdepth, 0)?;
            let mut c = full_image.nb_meta_channels;
            if !groupwise {
                while c < full_image.channel.len() {
                    let fc = &full_image.channel[c];
                    if fc.w > self.frame_dim.group_dim || fc.h > self.frame_dim.group_dim {
                        break;
                    }
                    c += 1;
                }
            }
            for c in c..full_image.channel.len() {
                let fc = &full_image.channel[c];
                let shift = fc.hshift.min(fc.vshift);
                if shift > max_shift {
                    continue;
                }
                if shift < min_shift {
                    continue;
                }
                let r = Rect::new_clamped(
                    rect.x0() >> fc.hshift,
                    rect.y0() >> fc.vshift,
                    rect.xsize() >> fc.hshift,
                    rect.ysize() >> fc.vshift,
                    fc.w,
                    fc.h,
                );
                if r.xsize() == 0 || r.ysize() == 0 {
                    continue;
                }
                self.gi_channel[s].push(c as u32);
                let mut gc = Channel::create(r.xsize(), r.ysize())?;
                gc.hshift = fc.hshift;
                gc.vshift = fc.vshift;
                for y in 0..r.ysize() {
                    gc.row_mut(y)[..r.xsize()]
                        .copy_from_slice(&fc.row(y + r.y0())[r.x0()..r.x0() + r.xsize()]);
                }
                gi.channel.push(gc);
            }
            if gi.channel.is_empty() {
                return Ok(());
            }
            if self.cparams.modular_part_is_lossless()
                && cparams.responsive == 0
                && !cparams.lossy_palette
                && cparams.speed_tier < SpeedTier::Cheetah
            {
                let (mut max_bitdepth, mut maxval) = (0, 0);
                try_palettes(
                    gi,
                    &mut max_bitdepth,
                    &mut maxval,
                    cparams,
                    cparams.channel_colors_percent,
                )?;
            }
        }
        let gi = &mut self.stream_images[s];
        if cparams.color_transform == ColorTransform::NONE
            && cparams.is_lossless()
            && cparams.colorspace < 0
            && gi.channel.len().wrapping_sub(gi.nb_meta_channels) >= 3
            && cparams.responsive == 0
            && do_color
            && cparams.speed_tier <= SpeedTier::Hare
        {
            let mut nb_rcts_to_try = match cparams.speed_tier {
                SpeedTier::Lightning
                | SpeedTier::Thunder
                | SpeedTier::Falcon
                | SpeedTier::Cheetah => 0,
                SpeedTier::Hare => 4,
                SpeedTier::Wombat => 5,
                SpeedTier::Squirrel => 7,
                SpeedTier::Kitten => 9,
                SpeedTier::TectonicPlate | SpeedTier::Glacier | SpeedTier::Tortoise => 19,
            };
            let mut best_cost = f32::MAX;
            let mut best_rct = 0usize;
            let need_to_restore = nb_rcts_to_try > 1;
            let mut orig = Vec::with_capacity(3);
            for rct_type in [
                0usize, 6, 5, 10, 26, 40, 12, 19, 8, 4, 9, 15, 16, 17, 32, 33, 2, 1, 3,
            ] {
                if nb_rcts_to_try == 0 {
                    break;
                }
                nb_rcts_to_try -= 1;
                if rct_type == 0 {
                    best_cost = estimate_cost(gi)?;
                    for c in 0..3 {
                        let genuine = &mut gi.channel[gi.nb_meta_channels + c];
                        orig.push(Channel::create_with_shifts(
                            genuine.w,
                            genuine.h,
                            genuine.hshift,
                            genuine.vshift,
                        )?);
                        std::mem::swap(&mut genuine.plane, &mut orig[c].plane);
                    }
                } else {
                    let begin = gi.nb_meta_channels;
                    let [r, g, b] = &mut gi.channel[begin..begin + 3] else {
                        unreachable!("source RCT search has three channels")
                    };
                    fwd_rct_channels([&orig[0], &orig[1], &orig[2]], [r, g, b], rct_type)?;
                    let cost = estimate_cost(gi)?;
                    if cost < best_cost {
                        best_rct = rct_type;
                        best_cost = cost;
                    }
                }
            }
            if need_to_restore {
                #[expect(
                    clippy::needless_range_loop,
                    reason = "Preserve source triplet indices and plane-swap order."
                )]
                for c in 0..3 {
                    std::mem::swap(
                        &mut gi.channel[gi.nb_meta_channels + c].plane,
                        &mut orig[c].plane,
                    );
                }
            }
            if best_rct != 0 {
                let mut sg = Transform::new(TransformId::RCT);
                sg.begin_c = gi.nb_meta_channels as u32;
                sg.rct_type = best_rct as u32;
                do_transform(gi, &sg, &weighted::Header::default(), false);
            }
        }
        let nb_wp_modes = if cparams.speed_tier <= SpeedTier::Tortoise {
            5
        } else if cparams.speed_tier <= SpeedTier::Kitten {
            2
        } else {
            1
        };
        if nb_wp_modes > 1
            && self.cparams.custom_fixed_tree.is_empty()
            && predictor_has_weighted(self.stream_options[s].predictor)
        {
            let mut best_cost = f32::MAX;
            self.stream_options[s].wp_mode = 0;
            for i in 0..nb_wp_modes {
                let cost = estimate_wp_cost(gi, i);
                if cost < best_cost {
                    best_cost = cost;
                    self.stream_options[s].wp_mode = i as i32;
                }
            }
        }
        Ok(())
    }

    pub fn compute_tree(&mut self) -> Status {
        let mut multiplier_info: Vec<ModularMultiplierInfo> = Vec::new();
        if !self.quants.is_empty() {
            for stream_id in 0..self.stream_images.len() as u32 {
                let s = stream_id as usize;
                if stream_id > 0 && self.gi_channel[s].is_empty() {
                    continue;
                }
                let image = &self.stream_images[s];
                let options = &self.stream_options[s];
                for i in image.nb_meta_channels as u32..image.channel.len() as u32 {
                    let c = i as usize;
                    if image.channel[c].w > options.max_chan_size
                        || image.channel[c].h > options.max_chan_size
                    {
                        continue;
                    }
                    if stream_id > 0 && self.gi_channel[s].is_empty() {
                        continue;
                    }
                    let ch_id = if stream_id == 0 {
                        c
                    } else {
                        self.gi_channel[s][c - image.nb_meta_channels] as usize
                    };
                    let q = self.quants[ch_id];
                    if multiplier_info
                        .last()
                        .is_none_or(|last| last.range[1][0] != stream_id || last.multiplier != q)
                    {
                        multiplier_info.push(ModularMultiplierInfo {
                            range: [
                                [i, i.wrapping_add(1)],
                                [stream_id, stream_id.wrapping_add(1)],
                            ],
                            multiplier: q,
                        });
                    } else {
                        multiplier_info.last_mut().unwrap().range[0][1] = i.wrapping_add(1);
                    }
                }
            }
            super::base::libcpp_sort::sort_by(&mut multiplier_info, |a, b| {
                (a.range, a.multiplier) < (b.range, b.multiplier)
            });
            let mut new_num = 1;
            for i in 1..multiplier_info.len() {
                let cur = multiplier_info[i];
                let prev = &mut multiplier_info[new_num - 1];
                if prev.range[0] == cur.range[0]
                    && prev.multiplier == cur.multiplier
                    && prev.range[1][1] == cur.range[1][0]
                {
                    prev.range[1][1] = cur.range[1][1];
                } else {
                    multiplier_info[new_num] = cur;
                    new_num += 1;
                }
            }
            multiplier_info.resize(new_num, ModularMultiplierInfo::default());
        }
        if !self.cparams.custom_fixed_tree.is_empty() {
            self.tree = self.cparams.custom_fixed_tree.clone();
        } else if self.cparams.speed_tier < SpeedTier::Falcon || !self.cparams.modular_mode {
            let mut useful_splits = Vec::with_capacity(self.tree_splits.len());
            for chunk in 0..self.tree_splits.len() - 1 {
                let mut has_pixels = false;
                for i in self.tree_splits[chunk]..self.tree_splits[chunk + 1] {
                    if !self.stream_images[i].empty() {
                        has_pixels = true;
                    }
                }
                if has_pixels {
                    useful_splits.push(self.tree_splits[chunk]);
                }
            }
            if useful_splits.is_empty() {
                return Ok(());
            }
            useful_splits.push(*self.tree_splits.last().unwrap());
            let mut trees = vec![Tree::new(); useful_splits.len() - 1];
            for chunk in 0..(useful_splits.len() - 1) as u32 {
                let chunk = chunk as usize;
                let mut start = useful_splits[chunk] as u32;
                let mut stop = useful_splits[chunk + 1] as u32;
                while start < stop && self.stream_images[start as usize].empty() {
                    start += 1;
                }
                while start < stop && self.stream_images[(stop - 1) as usize].empty() {
                    stop -= 1;
                }
                if self.stream_options[start as usize].tree_kind == TreeKind::Learn {
                    trees[chunk] = learn_tree(
                        &self.stream_images,
                        &self.stream_options,
                        start,
                        stop,
                        &multiplier_info,
                    )?;
                } else {
                    let mut total_pixels = 0usize;
                    for i in start..stop {
                        for ch in &self.stream_images[i as usize].channel {
                            total_pixels = total_pixels.wrapping_add(ch.w.wrapping_mul(ch.h));
                        }
                    }
                    total_pixels = total_pixels.max(1);
                    trees[chunk] = predefined_tree(
                        self.stream_options[start as usize].tree_kind,
                        total_pixels,
                        8,
                        0,
                    );
                }
            }
            self.tree.clear();
            merge_trees(
                &trees,
                &useful_splits,
                0,
                useful_splits.len() - 1,
                &mut self.tree,
            )?;
        } else {
            let mut total_pixels = 0usize;
            let mut max_bitdepth = 0;
            for img in &self.stream_images {
                max_bitdepth = max_bitdepth.max(img.bitdepth);
                for ch in &img.channel {
                    total_pixels = total_pixels.wrapping_add(ch.w.wrapping_mul(ch.h));
                }
            }
            self.tree = if self.cparams.speed_tier <= SpeedTier::Falcon {
                predefined_tree(
                    TreeKind::WPFixedDC,
                    total_pixels,
                    max_bitdepth,
                    self.stream_options[0].max_properties,
                )
            } else if self.cparams.speed_tier <= SpeedTier::Thunder {
                predefined_tree(
                    TreeKind::GradientFixedDC,
                    total_pixels,
                    max_bitdepth,
                    self.stream_options[0].max_properties,
                )
            } else {
                vec![PropertyDecisionNode::leaf(Predictor::GRADIENT, 0, 1)]
            };
        }
        self.tree_tokens.resize_with(1, Vec::new);
        self.tree_tokens[0].clear();
        let mut decoded_tree = Tree::new();
        tokenize_tree(&self.tree, &mut self.tree_tokens[0], &mut decoded_tree)?;
        if self.tree.len() != decoded_tree.len() {
            return Err(StatusCode::GenericError);
        }
        self.tree = decoded_tree;
        Ok(())
    }

    pub fn compute_tokens(&mut self) -> Status {
        let num_streams = self.stream_images.len();
        self.stream_headers
            .resize_with(num_streams, GroupHeader::default);
        self.tokens.resize_with(num_streams, Vec::new);
        self.image_widths.resize(num_streams, 0);
        for stream_id in 0..num_streams as u32 {
            let s = stream_id as usize;
            self.tokens[s].clear();
            modular_compress(
                &self.stream_images[s],
                &self.stream_options[s],
                s,
                &self.tree,
                &mut self.stream_headers[s],
                &mut self.tokens[s],
                &mut self.image_widths[s],
            )?;
        }
        Ok(())
    }

    pub fn encode_global_info(
        &mut self,
        streaming_mode: bool,
        writer: &mut BitWriter,
        mut aux: Option<&mut AuxOut>,
    ) -> Status {
        let mut skip_rest = false;
        writer.with_max_bits(
            1,
            LayerType::ModularTree,
            aux.as_deref_mut(),
            |w, _| {
                if self.tree_tokens.is_empty() || self.tree_tokens[0].is_empty() {
                    w.write(1, 0);
                    skip_rest = true;
                } else {
                    w.write(1, 1);
                }
                Ok(())
            },
            false,
        )?;
        if skip_rest {
            return Ok(());
        }
        let mut params =
            HistogramParams::for_modular(&self.cparams, &self.extra_dc_precision, streaming_mode);
        {
            let mut tree_code = EntropyEncodingData::default();
            build_and_encode_histograms(
                &params,
                NUM_TREE_CONTEXTS,
                &mut self.tree_tokens,
                &mut tree_code,
                Some(writer),
                LayerType::ModularTree,
                aux.as_deref_mut(),
            )?;
            write_tokens_with_allotment(
                &self.tree_tokens[0],
                &tree_code,
                0,
                writer,
                LayerType::ModularTree,
                aux.as_deref_mut(),
            )?;
        }
        params.streaming_mode = streaming_mode;
        params.add_missing_symbols = streaming_mode;
        params.image_widths = self.image_widths.clone();
        build_and_encode_histograms(
            &params,
            self.tree.len().wrapping_add(1) / 2,
            &mut self.tokens,
            &mut self.code,
            Some(writer),
            LayerType::ModularGlobal,
            aux,
        )?;
        Ok(())
    }

    pub fn encode_stream(
        &mut self,
        writer: &mut BitWriter,
        mut aux: Option<&mut AuxOut>,
        layer: LayerType,
        stream: &ModularStreamId,
    ) -> Status {
        let s = stream.id(&self.frame_dim);
        if self.stream_images[s].channel.is_empty() {
            return Ok(());
        }
        if self.tokens.is_empty() {
            modular_generic_compress(
                &self.stream_images[s],
                &self.stream_options[s],
                writer,
                aux,
                layer,
                s,
            )?;
        } else {
            bundle::write(
                &mut self.stream_headers[s],
                writer,
                layer,
                aux.as_deref_mut(),
            )?;
            write_tokens_with_allotment(&self.tokens[s], &self.code, 0, writer, layer, aux)?;
        }
        Ok(())
    }

    pub fn clear_stream_data(&mut self, stream: &ModularStreamId) {
        let mut empty_image = Image::default();
        std::mem::swap(
            &mut self.stream_images[stream.id(&self.frame_dim)],
            &mut empty_image,
        );
    }
    pub fn clear_modular_stream_data(&mut self) {
        for i in 0..self.stream_params.len() {
            let stream = self.stream_params[i].id;
            self.clear_stream_data(&stream);
        }
        self.stream_params.clear();
    }
    pub fn compute_streaming_absolute_ac_group_id(
        &self,
        dc_group_id: usize,
        ac_group_id: usize,
        patch_dim: &FrameDimensions,
    ) -> usize {
        let dc_group_x = dc_group_id % self.frame_dim.xsize_dc_groups;
        let dc_group_y = dc_group_id / self.frame_dim.xsize_dc_groups;
        let ac_group_x = ac_group_id % patch_dim.xsize_groups;
        let ac_group_y = ac_group_id / patch_dim.xsize_groups;
        dc_group_x
            .wrapping_mul(8)
            .wrapping_add(ac_group_x)
            .wrapping_add(
                dc_group_y
                    .wrapping_mul(8)
                    .wrapping_add(ac_group_y)
                    .wrapping_mul(self.frame_dim.xsize_groups),
            )
    }
}

fn source_i32(value: f64) -> i32 {
    assert!(
        value.trunc() >= i32::MIN as f64 && value.trunc() <= i32::MAX as f64,
        "source palette float-to-int conversion must be defined"
    );
    value as i32
}

pub fn merge_trees(
    trees: &[Tree],
    tree_splits: &[usize],
    begin: usize,
    end: usize,
    tree: &mut Tree,
) -> Status {
    if trees.len().wrapping_add(1) != tree_splits.len() || end <= begin || end > trees.len() {
        return Err(StatusCode::GenericError);
    }
    if end == begin.wrapping_add(1) {
        let sz = tree.len();
        tree.extend_from_slice(&trees[begin]);
        for node in &mut tree[sz..] {
            node.lchild = (node.lchild as usize).wrapping_add(sz) as u32;
            node.rchild = (node.rchild as usize).wrapping_add(sz) as u32;
        }
        return Ok(());
    }
    let mid = begin.wrapping_add(end) / 2;
    let splitval = tree_splits[mid].wrapping_sub(1);
    let cur = tree.len();
    tree.push(PropertyDecisionNode::new(
        1,
        splitval as i32,
        0,
        0,
        Predictor::ZERO,
        0,
        1,
    ));
    tree[cur].lchild = tree.len() as u32;
    merge_trees(trees, tree_splits, mid, end, tree)?;
    tree[cur].rchild = tree.len() as u32;
    merge_trees(trees, tree_splits, begin, mid, tree)?;
    Ok(())
}

/// Quantize the complete plane, preserving source division and valid signed
/// arithmetic preconditions, even when the channel's logical dimensions differ.
pub fn quantize_channel(ch: &mut Channel, q: i32) {
    if q == 1 {
        return;
    }
    for y in 0..ch.plane.ysize() {
        for x in 0..ch.plane.xsize() {
            let value = &mut ch.plane.row_mut(y)[x];
            *value = if *value < 0 {
                -((-*value + q / 2) / q) * q
            } else {
                ((*value + q / 2) / q) * q
            };
        }
    }
}

pub fn estimate_wp_cost(img: &Image, i: usize) -> f32 {
    let mut extra_bits = 0usize;
    let mut histo_cost = 0.0f32;
    let config = HybridUintConfig::default();
    const CUTOFFS: [i32; 33] = [
        -500, -392, -255, -191, -127, -95, -63, -47, -31, -23, -15, -11, -7, -4, -3, -1, 0, 1, 3,
        5, 7, 11, 15, 23, 31, 47, 63, 95, 127, 191, 255, 392, 500,
    ];
    let mut histo: [Histogram; 34] = std::array::from_fn(|_| Histogram::default());
    let mut wp_header = weighted::Header::default();
    weighted::predictor_mode(i as i32, &mut wp_header);
    for ch in &img.channel {
        let mut wp_state = weighted::State::new(&wp_header, ch.w, ch.h);
        let mut properties = vec![0];
        let mut unhealthy = false;
        for y in 0..ch.h {
            let row = ch.row(y);
            for x in 0..ch.w {
                let left = if x != 0 {
                    row[x - 1]
                } else if y != 0 {
                    ch.row(y - 1)[x]
                } else {
                    0
                } as i64;
                let top = if y != 0 {
                    ch.row(y - 1)[x] as i64
                } else {
                    left
                };
                let topleft = if x != 0 && y != 0 {
                    ch.row(y - 1)[x - 1] as i64
                } else {
                    left
                };
                let topright = if x + 1 < ch.w && y != 0 {
                    ch.row(y - 1)[x + 1] as i64
                } else {
                    top
                };
                let toptop = if y > 1 { ch.row(y - 2)[x] as i64 } else { top };
                let guess = wp_state.predict::<true>(
                    x,
                    y,
                    ch.w,
                    top,
                    left,
                    topright,
                    topleft,
                    toptop,
                    Some(&mut properties),
                    0,
                ) as i32;
                let mut ctx = 0;
                for c in CUTOFFS {
                    ctx += usize::from(c >= properties[0]);
                }
                let (res, overflow) = row[x].overflowing_sub(guess);
                unhealthy |= overflow;
                let (token, nbits, _) = config.encode(pack_signed(res));
                histo[ctx].add(token as usize);
                extra_bits = extra_bits.wrapping_add(nbits as usize);
                wp_state.update_errors(row[x] as i64, x, y, ch.w);
            }
        }
        if unhealthy {
            return f32::MAX;
        }
        for h in &mut histo {
            histo_cost += h.shannon_entropy();
            h.clear();
        }
    }
    histo_cost + extra_bits as f32
}

pub fn do_transform(
    image: &mut Image,
    tr: &Transform,
    wp_header: &weighted::Header,
    force_jxlart: bool,
) -> bool {
    let mut t = tr.clone();
    let did_it = if force_jxlart {
        if t.meta_apply(image).is_err() {
            return false;
        }
        true
    } else {
        transform_forward(&mut t, image, wp_header).is_ok()
    };
    if did_it {
        image.transform.push(t);
    }
    did_it
}

pub fn maybe_do_transform(
    image: &mut Image,
    tr: &Transform,
    cparams: &CompressParams<'_>,
    wp_header: &weighted::Header,
    cost_before: f32,
    force_jxlart: bool,
) -> StatusOr<bool> {
    if force_jxlart || cparams.speed_tier >= SpeedTier::Squirrel {
        return Ok(do_transform(image, tr, wp_header, force_jxlart));
    }
    let mut did_it = do_transform(image, tr, wp_header, false);
    if did_it {
        let cost_after = estimate_cost(image)?;
        if cost_after > cost_before {
            let t = image
                .transform
                .last()
                .expect("successful source transform pushed an entry")
                .clone();
            if t.inverse(image, wp_header, None).is_err() {
                return Ok(false);
            }
            image.transform.pop();
            did_it = false;
        }
    }
    Ok(did_it)
}

pub fn try_palettes(
    gi: &mut Image,
    max_bitdepth: &mut i32,
    maxval: &mut i32,
    cparams: &CompressParams<'_>,
    channel_colors_percent: f32,
) -> Status {
    let mut cost_before = 0.0f32;
    let mut did_palette = 0usize;
    let nb_pixels = gi.channel[0].w.wrapping_mul(gi.channel[0].h) as f32;
    let nb_chans = gi.channel.len().wrapping_sub(gi.nb_meta_channels) as i32;
    let arbitrary_bpp_estimate = 0.2f32 * gi.bitdepth as f32 * nb_chans as f32;
    if cparams.palette_colors != 0 || cparams.lossy_palette {
        cost_before = if cparams.speed_tier <= SpeedTier::Squirrel {
            estimate_cost(gi)?
        } else {
            nb_pixels * arbitrary_bpp_estimate
        };
        if nb_chans > 1 {
            let mut t = Transform::new(TransformId::PALETTE);
            t.begin_c = gi.nb_meta_channels as u32;
            t.num_c = nb_chans as u32;
            assert_ne!(
                cparams.palette_colors,
                i32::MIN,
                "source palette abs must be defined"
            );
            t.nb_colors =
                source_i32((mul_add_f32(cost_before, 0.0005, nb_pixels / 128.0) + 128.0) as f64)
                    .min(cparams.palette_colors.abs()) as u32;
            t.ordered_palette = cparams.palette_colors >= 0;
            t.lossy_palette = cparams.lossy_palette && t.num_c == 3;
            if t.lossy_palette {
                t.predictor = Predictor::AVERAGE4;
            }
            did_palette = usize::from(maybe_do_transform(
                gi,
                &t,
                cparams,
                &weighted::Header::default(),
                cost_before,
                cparams.options.zero_tokens,
            )?);
        }
        if did_palette == 0 && nb_chans > 3 {
            let mut t = Transform::new(TransformId::PALETTE);
            t.begin_c = gi.nb_meta_channels as u32;
            t.num_c = (nb_chans - 1) as u32;
            assert_ne!(
                cparams.palette_colors,
                i32::MIN,
                "source palette abs must be defined"
            );
            t.nb_colors =
                source_i32((mul_add_f32(cost_before, 0.0005, nb_pixels / 128.0) + 128.0) as f64)
                    .min(cparams.palette_colors.abs()) as u32;
            t.ordered_palette = cparams.palette_colors >= 0;
            t.lossy_palette = cparams.lossy_palette;
            if t.lossy_palette {
                t.predictor = Predictor::AVERAGE4;
            }
            did_palette = usize::from(maybe_do_transform(
                gi,
                &t,
                cparams,
                &weighted::Header::default(),
                cost_before,
                cparams.options.zero_tokens,
            )?);
        }
    }
    if channel_colors_percent > 0.0 {
        let nb_channels = gi
            .channel
            .len()
            .wrapping_sub(gi.nb_meta_channels)
            .wrapping_sub(did_palette);
        let orig_bitdepth = *max_bitdepth;
        *max_bitdepth = 0;
        if nb_channels > 0 && (did_palette != 0 || cost_before == 0.0) {
            cost_before = if cparams.speed_tier < SpeedTier::Squirrel {
                estimate_cost(gi)?
            } else {
                0.0
            };
        }
        for i in did_palette..nb_channels.wrapping_add(did_palette) {
            let (mut min, mut max) = (0, 0);
            compute_minmax(
                &gi.channel[gi.nb_meta_channels + i],
                Some(&mut min),
                Some(&mut max),
            );
            let colors = max as i64 - min as i64 + 1;
            let mut t = Transform::new(TransformId::PALETTE);
            t.begin_c = i.wrapping_add(gi.nb_meta_channels) as u32;
            t.num_c = 1;
            t.nb_colors = source_i32((nb_pixels / 16.0) as f64).min(source_i32(
                channel_colors_percent as f64 / 100.0 * colors as f64,
            )) as u32;
            let did_ch_palette = maybe_do_transform(
                gi,
                &t,
                cparams,
                &weighted::Header::default(),
                cost_before,
                false,
            )?;
            if did_ch_palette {
                compute_minmax(
                    &gi.channel[gi.nb_meta_channels + i],
                    Some(&mut min),
                    Some(&mut max),
                );
                if max < *maxval {
                    *maxval = max;
                }
                let ch_bitdepth = if max > 0 {
                    ceil_log2_nonzero(max as u32) as i32
                } else {
                    0
                };
                if ch_bitdepth > *max_bitdepth {
                    *max_bitdepth = ch_bitdepth;
                }
            } else {
                *max_bitdepth = orig_bitdepth;
            }
        }
    }
    Ok(())
}

/// Convert normalized integer samples or exactly representable custom floats.
///
/// Both rows must contain `xsize` samples. Integer conversion requires finite
/// products whose rounded, truncated result fits i32. Custom floats other than
/// binary32 require 1..=8 exponent bits and 0..=23 mantissa bits.
pub fn float_to_int(
    row_in: &[f32],
    row_out: &mut [i32],
    xsize: usize,
    bits: u32,
    exp_bits: u32,
    fp: bool,
    dfactor: f64,
) -> Status {
    if bits > 32 {
        return Err(StatusCode::GenericError);
    }
    if !fp {
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        if bits > 22 {
            for x in 0..xsize {
                let value = mul_add_f64(
                    row_in[x] as f64,
                    dfactor,
                    if row_in[x] < 0.0 { -0.5 } else { 0.5 },
                );
                assert!(value.trunc() >= i32::MIN as f64 && value.trunc() <= i32::MAX as f64);
                row_out[x] = value as i32;
            }
        } else {
            let factor = dfactor as f32;
            for x in 0..xsize {
                let value =
                    mul_add_f32(row_in[x], factor, if row_in[x] < 0.0 { -0.5 } else { 0.5 });
                assert!(
                    value.trunc() as f64 >= i32::MIN as f64
                        && value.trunc() as f64 <= i32::MAX as f64
                );
                row_out[x] = value as i32;
            }
        }
        return Ok(());
    }
    if bits == 32 {
        if exp_bits != 8 {
            return Err(StatusCode::GenericError);
        }
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        for x in 0..xsize {
            row_out[x] = row_in[x].to_bits() as i32;
        }
        return Ok(());
    }
    if bits == 0 {
        return Err(StatusCode::GenericError);
    }
    assert!((1..=8).contains(&exp_bits) && bits > exp_bits && bits - exp_bits - 1 <= 23);
    let exp_bias = (1i32 << (exp_bits - 1)) - 1;
    let max_exp = (1i32 << exp_bits) - 1;
    let sign = 1u32 << (bits - 1);
    let mant_bits = (bits - exp_bits - 1) as i32;
    let mant_shift = 23 - mant_bits;
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        let mut f = row_in[x].to_bits();
        let signbit = f >> 31;
        f &= 0x7fff_ffff;
        if f == 0 {
            row_out[x] = if signbit != 0 { sign as i32 } else { 0 };
            continue;
        }
        let mut exp = (f >> 23) as i32 - 127;
        let mut mantissa = (f & 0x007f_ffff) as i32;
        if exp == 128 {
            f = if signbit != 0 { sign } else { 0 };
            f |= (((1i32 << exp_bits) - 1) << mant_bits) as u32;
            f |= (mantissa >> mant_shift) as u32;
            row_out[x] = f as i32;
            continue;
        }
        exp += exp_bias;
        if exp <= 0 {
            mantissa |= 0x0080_0000;
            if exp < -mant_bits {
                return Err(StatusCode::GenericError);
            }
            mantissa >>= 1 - exp;
            exp = 0;
        }
        if exp >= max_exp {
            return Err(StatusCode::GenericError);
        }
        if mantissa & ((1 << mant_shift) - 1) != 0 {
            return Err(StatusCode::GenericError);
        }
        mantissa >>= mant_shift;
        f = if signbit != 0 { sign } else { 0 };
        f |= (exp << mant_bits) as u32;
        f |= mantissa as u32;
        row_out[x] = f as i32;
    }
    Ok(())
}
