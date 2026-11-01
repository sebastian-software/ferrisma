// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ModularStreamId storage, constructors, and index mapping from
//! lib/jxl/dec_modular.h, and sample conversion from lib/jxl/dec_modular.cc.
//! ModularFrameDecoder storage, Init, DecodeGlobalInfo, and MaybeDropFullImage
//! retain their complete source control flow. DecodeGroup, image-to-pipeline
//! conversion, and FinalizeDecoding use the original null-pool/simple-pipeline
//! configuration. Full decoder state orchestration and debug descriptions remain
//! unported. The three Highway
//! kernels retain the one-lane scalar configuration; disjoint row slices replace
//! restrict pointers. Custom-float shifts retain source caller preconditions.
//! Kind retains raw fixed-underlying-type enum values: the source switch leaves
//! an unknown kind's initialized index at zero. All size_t arithmetic wraps.

use super::base::bits::ceil_log2_nonzero;
use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::dec_ans::{ANSCode, decode_histograms};
use super::dec_bit_reader::BitReader;
use super::frame_dimensions::FrameDimensions;
use super::frame_header::{ColorTransform, FLAG_NOISE, FrameEncoding, FrameHeader};
use super::image::Plane;
use super::image_ops::{copy_rect_to, zero_fill_image};
use super::modular::encoding::dec_ma::{Tree, decode_tree};
use super::modular::encoding::encoding::{GroupHeader, modular_generic_decompress};
use super::modular::modular_image::{Channel, Image};
use super::modular::options::ModularOptions;
use super::modular::transform::transform::{Transform, TransformId};
use super::passes_state::PassesSharedState;
use super::quant_weights::NUM_QUANT_TABLES;
use super::render_pipeline::simple_render_pipeline::{RenderPipelineInput, SimpleRenderPipeline};

fn shift_right(n: usize, shift: i32) -> usize {
    assert!(
        (0..usize::BITS as i32).contains(&shift),
        "source shift precondition"
    );
    n >> shift
}
fn div_ceil_shift(n: usize, shift: i32) -> usize {
    assert!((0..31).contains(&shift), "source int shift precondition");
    let d = 1usize << shift;
    n.wrapping_add(d - 1) / d
}

pub fn multiply_sum(
    xsize: usize,
    row_in: &[i32],
    row_in_y: &[i32],
    factor: f32,
    row_out: &mut [f32],
) {
    assert!(row_in.len() >= xsize && row_in_y.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        let input = row_in[x].wrapping_add(row_in_y[x]);
        row_out[x] = input as f32 * factor;
    }
}

pub fn rgb_from_single(
    xsize: usize,
    row_in: &[i32],
    factor: f32,
    out_r: &mut [f32],
    out_g: &mut [f32],
    out_b: &mut [f32],
) {
    assert!(
        row_in.len() >= xsize
            && out_r.len() >= xsize
            && out_g.len() >= xsize
            && out_b.len() >= xsize
    );
    for x in 0..xsize {
        let output = row_in[x] as f32 * factor;
        out_r[x] = output;
        out_g[x] = output;
        out_b[x] = output;
    }
}

pub fn single_from_single(xsize: usize, row_in: &[i32], factor: f32, row_out: &mut [f32]) {
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        row_out[x] = row_in[x] as f32 * factor;
    }
}

pub fn single_from_single_accurate(xsize: usize, row_in: &[i32], factor: f64, row_out: &mut [f32]) {
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        row_out[x] = (row_in[x] as f64 * factor) as f32;
    }
}

/// Convert a custom floating-point bit pattern to binary32 without rounding.
///
/// Both rows must contain `xsize` samples. Except for binary32, the source
/// requires positive width, 1..=8 exponent bits, and 0..=23 mantissa bits.
pub fn int_to_float(
    row_in: &[i32],
    row_out: &mut [f32],
    xsize: usize,
    bits: i32,
    exp_bits: i32,
) -> Status {
    if bits == 32 {
        if exp_bits != 8 {
            return Err(StatusCode::GenericError);
        }
        assert!(row_in.len() >= xsize && row_out.len() >= xsize);
        for x in 0..xsize {
            row_out[x] = f32::from_bits(row_in[x] as u32);
        }
        return Ok(());
    }
    assert!(
        (1..=31).contains(&bits)
            && (1..=8).contains(&exp_bits)
            && bits > exp_bits
            && bits - exp_bits - 1 <= 23
    );
    let exp_bias = (1 << (exp_bits - 1)) - 1;
    let sign_shift = bits - 1;
    let mant_bits = bits - exp_bits - 1;
    let mant_shift = 23 - mant_bits;
    assert!(row_in.len() >= xsize && row_out.len() >= xsize);
    for x in 0..xsize {
        let mut f = row_in[x] as u32;
        let signbit = f >> sign_shift;
        f &= (1 << sign_shift) - 1;
        if f == 0 {
            row_out[x] = if signbit != 0 { -0.0 } else { 0.0 };
            continue;
        }
        let mut exp = (f >> mant_bits) as i32;
        let mut mantissa = (f & ((1 << mant_bits) - 1)) as i32;
        if exp == (1 << exp_bits) - 1 {
            f = if signbit != 0 { 0x8000_0000 } else { 0 };
            f |= 0b1111_1111 << 23;
            f |= (mantissa << mant_shift) as u32;
            row_out[x] = f32::from_bits(f);
            continue;
        }
        mantissa <<= mant_shift;
        if exp == 0 && exp_bits < 8 {
            while mantissa & 0x80_0000 == 0 {
                mantissa <<= 1;
                exp -= 1;
            }
            exp += 1;
            mantissa &= 0x7f_ffff;
        }
        exp -= exp_bias;
        exp += 127;
        if exp < 0 {
            return Err(StatusCode::GenericError);
        }
        f = if signbit != 0 { 0x8000_0000 } else { 0 };
        f |= (exp << 23) as u32;
        f |= mantissa as u32;
        row_out[x] = f32::from_bits(f);
    }
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct ModularStreamKind(pub i32);
impl ModularStreamKind {
    pub const GLOBAL_DATA: Self = Self(0);
    pub const VAR_DCT_DC: Self = Self(1);
    pub const MODULAR_DC: Self = Self(2);
    pub const AC_METADATA: Self = Self(3);
    pub const QUANT_TABLE: Self = Self(4);
    pub const MODULAR_AC: Self = Self(5);
}
#[derive(Clone, Copy, Debug)]
pub struct ModularStreamId {
    pub kind: ModularStreamKind,
    pub quant_table_id: usize,
    pub group_id: usize,
    pub pass_id: usize,
}
impl ModularStreamId {
    pub fn id(&self, frame_dim: &FrameDimensions) -> usize {
        match self.kind {
            ModularStreamKind::GLOBAL_DATA => 0,
            ModularStreamKind::VAR_DCT_DC => 1usize.wrapping_add(self.group_id),
            ModularStreamKind::MODULAR_DC => 1usize
                .wrapping_add(frame_dim.num_dc_groups)
                .wrapping_add(self.group_id),
            ModularStreamKind::AC_METADATA => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(2))
                .wrapping_add(self.group_id),
            ModularStreamKind::QUANT_TABLE => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(3))
                .wrapping_add(self.quant_table_id),
            ModularStreamKind::MODULAR_AC => 1usize
                .wrapping_add(frame_dim.num_dc_groups.wrapping_mul(3))
                .wrapping_add(NUM_QUANT_TABLES as usize)
                .wrapping_add(frame_dim.num_groups.wrapping_mul(self.pass_id))
                .wrapping_add(self.group_id),
            _ => 0,
        }
    }
    pub fn global() -> Self {
        Self {
            kind: ModularStreamKind::GLOBAL_DATA,
            quant_table_id: 0,
            group_id: 0,
            pass_id: 0,
        }
    }
    pub fn var_dct_dc(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::VAR_DCT_DC,
            group_id,
            ..Self::global()
        }
    }
    pub fn modular_dc(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::MODULAR_DC,
            group_id,
            ..Self::global()
        }
    }
    pub fn ac_metadata(group_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::AC_METADATA,
            group_id,
            ..Self::global()
        }
    }
    pub fn quant_table(quant_table_id: usize) -> StatusOr<Self> {
        if quant_table_id >= NUM_QUANT_TABLES as usize {
            return Err(StatusCode::GenericError);
        }
        Ok(Self {
            kind: ModularStreamKind::QUANT_TABLE,
            quant_table_id,
            ..Self::global()
        })
    }
    pub fn modular_ac(group_id: usize, pass_id: usize) -> Self {
        Self {
            kind: ModularStreamKind::MODULAR_AC,
            group_id,
            pass_id,
            ..Self::global()
        }
    }
    pub fn num(frame_dim: &FrameDimensions, passes: usize) -> usize {
        Self::modular_ac(0, passes).id(frame_dim)
    }
}

/// Original frame decoder owners. Safe initial values for source-indeterminate
/// fields do not give them source meaning before Init/DecodeGlobalInfo assigns
/// them. Native allocation uses the existing valid memory-manager context.
pub struct ModularFrameDecoder {
    memory_manager_present: bool,
    full_image: Image,
    global_transform: Vec<Transform>,
    frame_dim: FrameDimensions,
    do_color: bool,
    have_something: bool,
    use_full_image: bool,
    all_same_shift: bool,
    tree: Tree,
    code: ANSCode,
    context_map: Vec<u8>,
    global_header: GroupHeader,
}
impl Default for ModularFrameDecoder {
    fn default() -> Self {
        Self::new()
    }
}
impl ModularFrameDecoder {
    pub fn new() -> Self {
        Self {
            memory_manager_present: true,
            full_image: Image::default(),
            global_transform: Vec::new(),
            frame_dim: FrameDimensions::default(),
            do_color: false,
            have_something: false,
            use_full_image: true,
            all_same_shift: false,
            tree: Vec::new(),
            code: ANSCode::default(),
            context_map: Vec::new(),
            global_header: GroupHeader::default(),
        }
    }
    pub fn init(&mut self, new_frame_dim: &FrameDimensions) {
        self.frame_dim = *new_frame_dim;
    }
    pub fn decode_global_info(
        &mut self,
        reader: &mut BitReader<'_>,
        frame_header: &FrameHeader<'_>,
        allow_truncated_group: bool,
    ) -> Status {
        let decode_color = frame_header.encoding == FrameEncoding::MODULAR;
        let metadata = &frame_header
            .nonserialized_metadata
            .expect("source requires frame metadata")
            .m;
        let is_gray = metadata.color_encoding.is_gray();
        let mut nb_chans = 3usize;
        if is_gray && frame_header.color_transform == ColorTransform::NONE {
            nb_chans = 1;
        }
        self.do_color = decode_color;
        let nb_extra = metadata.extra_channel_info.len();
        let has_tree = reader.read_bits(1) != 0;
        #[expect(
            clippy::collapsible_if,
            reason = "Keep the original truncation guard outside the tree-presence branch."
        )]
        if !allow_truncated_group
            || reader.total_bits_consumed() < reader.total_bytes().wrapping_mul(8)
        {
            if has_tree {
                let tree_size_limit = (1usize << 22).min(
                    1024usize.wrapping_add(
                        self.frame_dim
                            .xsize
                            .wrapping_mul(self.frame_dim.ysize)
                            .wrapping_mul(nb_chans.wrapping_add(nb_extra))
                            / 16,
                    ),
                );
                decode_tree(reader, &mut self.tree, tree_size_limit)?;
                decode_histograms(
                    reader,
                    self.tree.len().wrapping_add(1) / 2,
                    &mut self.code,
                    &mut self.context_map,
                    false,
                )?;
            }
        }
        if !self.do_color {
            nb_chans = 0;
        }
        let fp = metadata.bit_depth.floating_point_sample;
        #[expect(
            clippy::if_same_then_else,
            reason = "Preserve the distinct original uint32 and greater-than-32 rejection checkpoints."
        )]
        if metadata.bit_depth.bits_per_sample >= 32
            && self.do_color
            && frame_header.color_transform != ColorTransform::XYB
        {
            if metadata.bit_depth.bits_per_sample == 32 && !fp {
                return Err(StatusCode::GenericError);
            } else if metadata.bit_depth.bits_per_sample > 32 {
                return Err(StatusCode::GenericError);
            }
        }
        let mut gi = Image::create(
            self.frame_dim.xsize,
            self.frame_dim.ysize,
            metadata.bit_depth.bits_per_sample as i32,
            nb_chans.wrapping_add(nb_extra) as i32,
        )?;
        self.all_same_shift = true;
        if frame_header.color_transform == ColorTransform::YCBCR {
            for c in 0..nb_chans {
                gi.channel[c].hshift = frame_header.chroma_subsampling.h_shift(c) as i32;
                gi.channel[c].vshift = frame_header.chroma_subsampling.v_shift(c) as i32;
                let xsize_shifted = self
                    .frame_dim
                    .xsize
                    .wrapping_add((1usize << gi.channel[c].hshift) - 1)
                    / (1usize << gi.channel[c].hshift);
                let ysize_shifted = self
                    .frame_dim
                    .ysize
                    .wrapping_add((1usize << gi.channel[c].vshift) - 1)
                    / (1usize << gi.channel[c].vshift);
                gi.channel[c].shrink_to(xsize_shifted as i32, ysize_shifted as i32)?;
                if gi.channel[c].hshift != gi.channel[0].hshift
                    || gi.channel[c].vshift != gi.channel[0].vshift
                {
                    self.all_same_shift = false;
                }
            }
        }
        for ec in 0..nb_extra {
            let c = nb_chans + ec;
            let ecups = frame_header.extra_channel_upsampling[ec] as usize;
            let w = self.frame_dim.xsize_upsampled.wrapping_add(ecups - 1) / ecups;
            let h = self.frame_dim.ysize_upsampled.wrapping_add(ecups - 1) / ecups;
            gi.channel[c].shrink_to(w as i32, h as i32)?;
            let shift = (ceil_log2_nonzero(ecups) as i32)
                .wrapping_sub(ceil_log2_nonzero(frame_header.upsampling as usize) as i32);
            gi.channel[c].hshift = shift;
            gi.channel[c].vshift = shift;
            if gi.channel[c].hshift != gi.channel[0].hshift
                || gi.channel[c].vshift != gi.channel[0].vshift
            {
                self.all_same_shift = false;
            }
        }
        let mut options = ModularOptions {
            max_chan_size: self.frame_dim.group_dim,
            group_dim: self.frame_dim.group_dim,
            ..ModularOptions::default()
        };
        let dec_status = modular_generic_decompress(
            reader,
            &mut gi,
            Some(&mut self.global_header),
            ModularStreamId::global().id(&self.frame_dim),
            &mut options,
            false,
            Some(&self.tree),
            Some(&self.code),
            Some(&self.context_map),
            allow_truncated_group,
        );
        if !allow_truncated_group {
            dec_status?;
        }
        if dec_status.is_err_and(StatusCode::is_fatal_error) {
            return Err(StatusCode::GenericError);
        }
        self.have_something = false;
        for (c, gic) in gi.channel.iter().enumerate() {
            if c >= gi.nb_meta_channels
                && gic.w <= self.frame_dim.group_dim
                && gic.h <= self.frame_dim.group_dim
            {
                self.have_something = true;
            }
        }
        if !self.have_something
            && self.all_same_shift
            && gi.transform.len() == 1
            && gi.transform[0].id == TransformId::RCT
        {
            self.global_transform = gi.transform.clone();
            gi.transform.clear();
        }
        self.full_image = gi;
        dec_status
    }
    pub fn maybe_drop_full_image(&mut self) {
        if self.full_image.transform.is_empty() && !self.have_something && self.all_same_shift {
            self.use_full_image = false;
            for ch in &mut self.full_image.channel {
                ch.plane = Plane::default();
            }
        }
    }
    /// Complete original DecodeGroup. The optional decoder-state pointer is
    /// reified as its actual const shared-state dependency; pipeline storage
    /// is borrowed separately. No PassesDecoderState constructor is substituted.
    #[expect(
        clippy::too_many_arguments,
        reason = "Keep original group, shift, reader, state, and output checkpoint parameters."
    )]
    pub fn decode_group(
        &mut self,
        header: &FrameHeader<'_>,
        rect: Rect,
        reader: Option<&mut BitReader<'_>>,
        min_shift: i32,
        max_shift: i32,
        stream: &ModularStreamId,
        zerofill: bool,
        shared: Option<&PassesSharedState<'_>>,
        input: Option<&mut RenderPipelineInput<'_, '_>>,
        allow_truncated: bool,
        should_run_pipeline: Option<&mut bool>,
    ) -> Status {
        if stream.kind != ModularStreamKind::MODULAR_DC
            && stream.kind != ModularStreamKind::MODULAR_AC
        {
            return Err(StatusCode::GenericError);
        }
        let mut gi = Image::create(rect.xsize(), rect.ysize(), self.full_image.bitdepth, 0)?;
        let mut c = self.full_image.nb_meta_channels;
        while c < self.full_image.channel.len() {
            let fc = &self.full_image.channel[c];
            if fc.w > self.frame_dim.group_dim || fc.h > self.frame_dim.group_dim {
                break;
            }
            c += 1;
        }
        let beginc = c;
        for fc in self.full_image.channel.iter_mut().skip(c) {
            let shift = fc.hshift.min(fc.vshift);
            if shift > max_shift {
                continue;
            }
            if shift < min_shift {
                continue;
            }
            let r = Rect::new_clamped(
                shift_right(rect.x0(), fc.hshift),
                shift_right(rect.y0(), fc.vshift),
                shift_right(rect.xsize(), fc.hshift),
                shift_right(rect.ysize(), fc.vshift),
                fc.w,
                fc.h,
            );
            if r.xsize() == 0 || r.ysize() == 0 {
                continue;
            }
            if zerofill && self.use_full_image {
                for y in 0..r.ysize() {
                    fc.plane.row_mut(r.y0() + y)[r.x0()..r.x1()].fill(0);
                }
            } else {
                let mut gc = Channel::create(r.xsize(), r.ysize())?;
                if zerofill {
                    zero_fill_image(&mut gc.plane);
                }
                gc.hshift = fc.hshift;
                gc.vshift = fc.vshift;
                gi.channel.push(gc);
            }
        }
        if zerofill && self.use_full_image {
            return Ok(());
        }
        if gi.channel.is_empty() {
            if let (Some(_), Some(should)) = (shared, should_run_pipeline) {
                if self.do_color
                    || header
                        .nonserialized_metadata
                        .expect("source requires metadata")
                        .m
                        .num_extra_channels
                        > 0
                {
                    *should = false;
                }
            }
            return Ok(());
        }
        let mut options = ModularOptions::default();
        if !zerofill {
            let status = modular_generic_decompress(
                reader.expect("source requires a reader for nonempty encoded groups"),
                &mut gi,
                None,
                stream.id(&self.frame_dim),
                &mut options,
                true,
                Some(&self.tree),
                Some(&self.code),
                Some(&self.context_map),
                allow_truncated,
            );
            if !allow_truncated {
                status?;
            }
            if status.is_err_and(StatusCode::is_fatal_error) {
                return status;
            }
        }
        if !self.use_full_image {
            let input = input.ok_or(StatusCode::GenericError)?;
            for t in &self.global_transform {
                t.inverse(&mut gi, &self.global_header.wp_header)?;
            }
            let r = Rect::new(0, 0, gi.w, gi.h);
            self.modular_image_to_decoded_rect(header, &gi, shared, input, r)?;
            return Ok(());
        }
        let mut gic = 0usize;
        for fc in self.full_image.channel.iter_mut().skip(beginc) {
            let shift = fc.hshift.min(fc.vshift);
            if shift > max_shift {
                continue;
            }
            if shift < min_shift {
                continue;
            }
            let r = Rect::new_clamped(
                shift_right(rect.x0(), fc.hshift),
                shift_right(rect.y0(), fc.vshift),
                shift_right(rect.xsize(), fc.hshift),
                shift_right(rect.ysize(), fc.vshift),
                fc.w,
                fc.h,
            );
            if r.xsize() == 0 || r.ysize() == 0 {
                continue;
            }
            if !self.use_full_image {
                return Err(StatusCode::GenericError);
            }
            copy_rect_to(
                Rect::new(0, 0, r.xsize(), r.ysize()),
                &gi.channel[gic].plane,
                r,
                &mut fc.plane,
            )?;
            gic += 1;
        }
        Ok(())
    }
    /// Original ModularImageToDecodedRect with a null pool. Its only decoder
    /// state read is shared->matrices; the actual shared owner is borrowed.
    /// Source input image rows are only read, so gi has an immutable borrow.
    pub fn modular_image_to_decoded_rect(
        &self,
        header: &FrameHeader<'_>,
        gi: &Image,
        shared: Option<&PassesSharedState<'_>>,
        input: &mut RenderPipelineInput<'_, '_>,
        modular_rect: Rect,
    ) -> Status {
        let metadata = header
            .nonserialized_metadata
            .expect("source requires metadata");
        if !gi.transform.is_empty() {
            return Err(StatusCode::GenericError);
        }
        let mut c = 0usize;
        if self.do_color {
            let rgb_from_gray = metadata.m.color_encoding.is_gray()
                && header.color_transform == ColorTransform::NONE;
            let fp = metadata.m.bit_depth.floating_point_sample
                && header.color_transform != ColorTransform::XYB;
            while c < 3 {
                let mut factor = if self.full_image.bitdepth < 32 {
                    assert!(self.full_image.bitdepth >= 0, "source shift precondition");
                    1.0 / f64::from((1u32 << self.full_image.bitdepth) - 1)
                } else {
                    0.0
                };
                let mut c_in = c;
                if header.color_transform == ColorTransform::XYB {
                    factor = f64::from(
                        shared
                            .expect("source XYB conversion requires decoder shared state")
                            .matrices
                            .dc_quants()[c],
                    );
                    if c < 2 {
                        c_in = 1 - c;
                    }
                } else if rgb_from_gray {
                    c_in = 0;
                }
                if c_in >= gi.channel.len() {
                    return Err(StatusCode::GenericError);
                }
                let ch = &gi.channel[c_in];
                if ch.w == 0 || ch.h == 0 {
                    return Err(StatusCode::GenericError);
                }
                if ch.hshift > 3 || ch.vshift > 3 {
                    return Err(StatusCode::GenericError);
                }
                let r = input.get_buffer(c).1;
                let mr = Rect::new(
                    shift_right(modular_rect.x0(), ch.hshift),
                    shift_right(modular_rect.y0(), ch.vshift),
                    div_ceil_shift(modular_rect.xsize(), ch.hshift),
                    div_ceil_shift(modular_rect.ysize(), ch.vshift),
                )
                .crop(ch.plane.xsize(), ch.plane.ysize());
                let width = r.xsize();
                let height = r.ysize();
                if r.ysize() != mr.ysize() || r.xsize() != mr.xsize() {
                    return Err(StatusCode::GenericError);
                }
                if header.color_transform == ColorTransform::XYB && c == 2 {
                    if fp {
                        return Err(StatusCode::GenericError);
                    }
                    for task in 0..height as u32 {
                        let y = task as usize;
                        let row_in = &ch.row(mr.y0() + y)[mr.x0()..];
                        let row_y = &gi.channel[0].row(mr.y0() + y)[mr.x0()..];
                        let (out, rect) = input.get_buffer_mut(c);
                        let row_out = &mut out.row_mut(rect.y0() + y)[rect.x0()..];
                        multiply_sum(width, row_in, row_y, factor as f32, row_out);
                    }
                } else if fp {
                    let bits = metadata.m.bit_depth.bits_per_sample as i32;
                    let exp = metadata.m.bit_depth.exponent_bits_per_sample as i32;
                    for task in 0..height as u32 {
                        let y = task as usize;
                        let row_in = &ch.row(mr.y0() + y)[mr.x0()..];
                        if rgb_from_gray {
                            for cc in 0..3 {
                                let (out, rect) = input.get_buffer_mut(cc);
                                int_to_float(
                                    row_in,
                                    &mut out.row_mut(rect.y0() + y)[rect.x0()..],
                                    width,
                                    bits,
                                    exp,
                                )
                                .map_err(|_| StatusCode::GenericError)?;
                            }
                        } else {
                            let (out, rect) = input.get_buffer_mut(c);
                            int_to_float(
                                row_in,
                                &mut out.row_mut(rect.y0() + y)[rect.x0()..],
                                width,
                                bits,
                                exp,
                            )
                            .map_err(|_| StatusCode::GenericError)?;
                        }
                    }
                } else {
                    for task in 0..height as u32 {
                        let y = task as usize;
                        let row_in = &ch.row(mr.y0() + y)[mr.x0()..];
                        if rgb_from_gray {
                            if self.full_image.bitdepth < 23 {
                                let mut buffers = input.get_buffers_mut();
                                let (r, rr) = buffers.next().expect("source RGB buffer");
                                let (g, gr) = buffers.next().expect("source RGB buffer");
                                let (b, br) = buffers.next().expect("source RGB buffer");
                                rgb_from_single(
                                    width,
                                    row_in,
                                    factor as f32,
                                    &mut r.row_mut(rr.y0() + y)[rr.x0()..],
                                    &mut g.row_mut(gr.y0() + y)[gr.x0()..],
                                    &mut b.row_mut(br.y0() + y)[br.x0()..],
                                );
                            } else {
                                for cc in 0..3 {
                                    let (out, rect) = input.get_buffer_mut(cc);
                                    single_from_single_accurate(
                                        width,
                                        row_in,
                                        factor,
                                        &mut out.row_mut(rect.y0() + y)[rect.x0()..],
                                    );
                                }
                            }
                        } else {
                            let (out, rect) = input.get_buffer_mut(c);
                            let row_out = &mut out.row_mut(rect.y0() + y)[rect.x0()..];
                            if self.full_image.bitdepth < 23 {
                                single_from_single(width, row_in, factor as f32, row_out);
                            } else {
                                single_from_single_accurate(width, row_in, factor, row_out);
                            }
                        }
                    }
                }
                if rgb_from_gray {
                    break;
                }
                c += 1;
            }
            if rgb_from_gray {
                c = 1;
            }
        }
        for ec in 0..metadata.m.num_extra_channels as usize {
            let eci = &metadata.m.extra_channel_info[ec];
            let bits = eci.bit_depth.bits_per_sample as i32;
            let exp = eci.bit_depth.exponent_bits_per_sample as i32;
            let fp = eci.bit_depth.floating_point_sample;
            if !fp && bits >= 32 {
                return Err(StatusCode::GenericError);
            }
            let factor = if fp {
                0.0
            } else {
                assert!((0..32).contains(&bits), "source shift precondition");
                1.0 / f64::from((1u32 << bits) - 1)
            };
            if c >= gi.channel.len() {
                return Err(StatusCode::GenericError);
            }
            let ch = &gi.channel[c];
            let r = input.get_buffer(3 + ec).1;
            let mr = Rect::new(
                shift_right(modular_rect.x0(), ch.hshift),
                shift_right(modular_rect.y0(), ch.vshift),
                div_ceil_shift(modular_rect.xsize(), ch.hshift),
                div_ceil_shift(modular_rect.ysize(), ch.vshift),
            )
            .crop(ch.plane.xsize(), ch.plane.ysize());
            if r.ysize() != mr.ysize() || r.xsize() != mr.xsize() {
                return Err(StatusCode::GenericError);
            }
            let (out, rect) = input.get_buffer_mut(3 + ec);
            for y in 0..r.ysize() {
                let row_in = &ch.row(mr.y0() + y)[mr.x0()..];
                let row_out = &mut out.row_mut(rect.y0() + y)[rect.x0()..];
                if fp {
                    int_to_float(row_in, row_out, r.xsize(), bits, exp)?;
                } else if self.full_image.bitdepth < 23 {
                    single_from_single(r.xsize(), row_in, factor as f32, row_out);
                } else {
                    single_from_single_accurate(r.xsize(), row_in, factor, row_out);
                }
            }
            c += 1;
        }
        Ok(())
    }
    /// FinalizeDecoding's original full-image move/clone, transforms, and group
    /// rendering with a null pool and the original simple implementation.
    /// The shared/pipeline field borrows replace the containing decoder pointer.
    pub fn finalize_decoding(
        &mut self,
        header: &FrameHeader<'_>,
        shared: &PassesSharedState<'_>,
        pipeline: &mut SimpleRenderPipeline<'_>,
        inplace: bool,
    ) -> Status {
        if !self.use_full_image {
            return Ok(());
        }
        let mut gi = if inplace {
            Image::move_from(&mut self.full_image)
        } else {
            Image::clone_image(&self.full_image)?
        };
        let _xsize = gi.w;
        let _ysize = gi.h;
        // The source's small-image guard sets an already-null pool to null.
        let _force_null_pool = _xsize.wrapping_mul(_ysize)
            < self
                .frame_dim
                .group_dim
                .wrapping_mul(self.frame_dim.group_dim);
        gi.undo_transforms(&self.global_header.wp_header);
        if !self.global_transform.is_empty() {
            return Err(StatusCode::GenericError);
        }
        if gi.error {
            return Err(StatusCode::GenericError);
        }
        for i in 0..shared.frame_dim.num_groups {
            pipeline.clear_done(i);
        }
        let end = shared.frame_dim.num_groups as u32;
        if end != 0 {
            let use_group_ids =
                header.encoding == FrameEncoding::VAR_DCT || (header.flags & FLAG_NOISE) != 0;
            pipeline
                .prepare_for_threads(1, use_group_ids)
                .map_err(|_| StatusCode::GenericError)?;
            for group in 0..end {
                let mut input = pipeline.get_input_buffers(group as usize, 0);
                self.modular_image_to_decoded_rect(
                    header,
                    &gi,
                    Some(shared),
                    &mut input,
                    shared.frame_dim.group_rect(group as usize),
                )
                .map_err(|_| StatusCode::GenericError)?;
                input.done().map_err(|_| StatusCode::GenericError)?;
            }
        }
        Ok(())
    }
    pub fn have_dc(&self) -> bool {
        self.have_something
    }
    pub fn uses_full_image(&self) -> bool {
        self.use_full_image
    }
    pub fn have_memory_manager(&self) -> bool {
        self.memory_manager_present
    }
    pub fn full_image(&self) -> &Image {
        &self.full_image
    }
    pub fn global_transforms(&self) -> &[Transform] {
        &self.global_transform
    }
    pub fn frame_dimensions(&self) -> &FrameDimensions {
        &self.frame_dim
    }
    pub fn do_color(&self) -> bool {
        self.do_color
    }
    pub fn all_same_shift(&self) -> bool {
        self.all_same_shift
    }
    pub fn tree(&self) -> &Tree {
        &self.tree
    }
    pub fn code(&self) -> &ANSCode {
        &self.code
    }
    pub fn context_map(&self) -> &[u8] {
        &self.context_map
    }
    pub fn global_header(&self) -> &GroupHeader {
        &self.global_header
    }
}
