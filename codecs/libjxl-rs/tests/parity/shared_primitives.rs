// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategyImage, AcStrategyType};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::{COLOR_FACTOR_DIST, ColorCorrelation, ColorCorrelationMap};
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
use jxl::jxl::image::{Plane, Sample};
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::Quantizer;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ss-")
}
pub(super) fn corr(out: &mut String, c: &ColorCorrelation) {
    write!(
        out,
        " C {} {} {} {} {} {} {} {}",
        c.raw_color_factor(),
        c.color_scale().to_bits(),
        c.color_factor().to_bits(),
        c.base_correlation_x().to_bits(),
        c.base_correlation_b().to_bits(),
        c.y_to_x_dc(),
        c.y_to_b_dc(),
        u32::from(c.is_jpeg_compatible())
    )
    .unwrap();
    for value in c.dc_factors() {
        write!(out, " {}", value.to_bits()).unwrap();
    }
}
fn quant(out: &mut String, q: &Quantizer, d: &DequantMatrices, slots: usize) {
    write!(
        out,
        " Q {} {} {} {} {}",
        q.global_scale(),
        q.raw_dc(),
        q.scale().to_bits(),
        q.inv_global_scale().to_bits(),
        q.inv_quant_dc().to_bits()
    )
    .unwrap();
    for c in 0..slots {
        write!(
            out,
            " {} {}",
            q.mul_dc()[c].to_bits(),
            q.inv_mul_dc()[c].to_bits()
        )
        .unwrap();
    }
    for value in q.zero_bias() {
        write!(out, " {}", value.to_bits()).unwrap();
    }
    for c in 0..3 {
        write!(
            out,
            " {} {}",
            q.dc_step(d, c).to_bits(),
            q.inv_dc_step(d, c).to_bits()
        )
        .unwrap();
    }
    for n in [0, 1, -1, 64, i32::MIN, i32::MAX] {
        write!(out, " {}", q.inv_quant_ac(n).to_bits()).unwrap();
    }
}
pub(super) fn plane<T: Sample>(out: &mut String, p: &Plane<T>) {
    write!(
        out,
        " P {} {} {} {}",
        p.xsize(),
        p.ysize(),
        p.bytes_per_row(),
        u32::from(p.bytes().is_some())
    )
    .unwrap();
}
fn signed_plane(out: &mut String, p: &Plane<i8>) {
    plane(out, p);
    for y in 0..p.ysize() {
        for value in p.row(y).iter().take(p.xsize()) {
            write!(out, " {value}").unwrap();
        }
    }
}
fn code_output(out: &mut String, codes: &[BitWriter]) {
    write!(out, " G {}", codes.len()).unwrap();
    for code in codes {
        let mut copy = BitWriter::new();
        copy.append_unaligned(code).unwrap();
        pad(&mut copy);
        write!(out, " {} {}", code.bits_written(), hex(copy.get_span())).unwrap();
    }
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    if kind == "ss-image" || kind == "ss-image-alpha" {
        return observe_image(args, kind == "ss-image-alpha");
    }
    if kind == "ss-owners" {
        return observe_owners(args);
    }
    if kind == "ss-acimage" {
        return observe_acimage(args);
    }
    if kind == "ss-quant" {
        let def = number::<u32>(args) != 0;
        let dc = number(args);
        let global = number(args);
        let custom = number::<u32>(args) != 0;
        let clear = number::<u32>(args) != 0;
        let first = std::array::from_fn(|_| f32::from_bits(number(args)));
        let second = std::array::from_fn(|_| f32::from_bits(number(args)));
        let mut d = DequantMatrices::default();
        if custom {
            d.set_dc_quant(&first);
        }
        let mut q = if def {
            Quantizer::new(&d)
        } else {
            Quantizer::with_values(&d, dc, global)
        };
        if clear {
            q.clear_dc_mul();
        }
        let mut out = String::new();
        let slots = if clear { 4 } else { 3 };
        quant(&mut out, &q, &d, slots);
        d.set_dc_quant(&second);
        quant(&mut out, &q, &d, slots);
        q.recompute_from_global_scale(&d);
        quant(&mut out, &q, &d, slots);
        return out;
    }
    if kind == "ss-corr" {
        let factor = number(args);
        let x = number(args);
        let b = number(args);
        let probe = number(args);
        let jpeg = number(args);
        let prefix = number(args);
        let bytes = unhex(args.next().unwrap());
        let mut c = ColorCorrelation::default();
        let mut out = String::new();
        corr(&mut out, &c);
        c.set_color_factor(factor);
        corr(&mut out, &c);
        c.set_y_to_x_dc(x);
        corr(&mut out, &c);
        c.set_y_to_b_dc(b);
        corr(&mut out, &c);
        write!(
            out,
            " R {} {} {}",
            c.y_to_x_ratio(probe).to_bits(),
            c.y_to_b_ratio(probe).to_bits(),
            ColorCorrelation::ratio_jpeg(jpeg)
        )
        .unwrap();
        let mut r = BitReader::new(&bytes);
        r.read_bits(prefix);
        let status = c.decode_dc(&mut r);
        let bits = r.total_bits_consumed();
        let closed = r.close();
        write!(
            out,
            " D {} {} {}",
            status_code(&status),
            bits,
            status_code(&closed)
        )
        .unwrap();
        corr(&mut out, &c);
        return out;
    }
    if kind == "ss-map" {
        let def = number::<u32>(args) != 0;
        let w = number(args);
        let h = number(args);
        let xyb = number::<u32>(args) != 0;
        let prefix = number(args);
        let bytes = unhex(args.next().unwrap());
        let result = if def {
            Ok(ColorCorrelationMap::default())
        } else {
            ColorCorrelationMap::create(w, h, xyb)
        };
        let mut out = result.as_ref().err().map_or(0, |s| *s as i32).to_string();
        let mut c = match result {
            Ok(c) => c,
            Err(_) => return out,
        };
        signed_plane(&mut out, &c.ytox_map);
        signed_plane(&mut out, &c.ytob_map);
        corr(&mut out, &c.base());
        let mut r = BitReader::new(&bytes);
        r.read_bits(prefix);
        let status = c.decode_dc(&mut r);
        let bits = r.total_bits_consumed();
        let closed = r.close();
        write!(
            out,
            " D {} {} {}",
            status_code(&status),
            bits,
            status_code(&closed)
        )
        .unwrap();
        corr(&mut out, &c.base());
        return out;
    }
    if kind == "ss-acs" {
        let def = number::<u32>(args) != 0;
        let w = number(args);
        let h = number(args);
        let mode = number::<u32>(args);
        let rx = number(args);
        let ry = number(args);
        let rw = number(args);
        let rh = number(args);
        let kind = AcStrategyType(number(args));
        let do_move = number::<u32>(args) != 0;
        let result = if def {
            Ok(AcStrategyImage::default())
        } else {
            AcStrategyImage::create(w, h)
        };
        let mut out = result.as_ref().err().map_or(0, |s| *s as i32).to_string();
        let mut c = match result {
            Ok(c) => c,
            Err(_) => return out,
        };
        if !def {
            c.fill_invalid();
            if mode == 1 {
                c.fill_dct8_full();
            }
            if mode == 2 {
                c.fill_dct8(Rect::new(rx, ry, rw, rh));
            }
        }
        if do_move {
            c = std::convert::identity(c);
        }
        write!(out, " {}", c.count_blocks(kind)).unwrap();
        plane(&mut out, c.layers());
        for y in 0..c.ysize() {
            for value in c.layers().row(y).iter().take(c.xsize()) {
                write!(out, " {value}").unwrap();
            }
        }
        for y in 0..c.ysize() {
            for x in 0..c.xsize() {
                write!(out, " {}", u32::from(c.is_valid(x, y))).unwrap();
            }
        }
        return out;
    }
    assert_eq!(kind, "ss-frame");
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let depth = number::<u32>(args);
    let null = number::<u32>(args) != 0;
    let has_aux = number::<u32>(args) != 0;
    let copy = number::<u32>(args) != 0;
    let data = unhex(args.next().unwrap());
    let config = LosslessThunderConfig::new(
        w,
        h,
        gray,
        if depth == 8 {
            LosslessSampleDepth::Uint8
        } else {
            LosslessSampleDepth::Uint16
        },
    )
    .unwrap();
    let mut frame = config.create_frame().unwrap();
    let mut input = JxlEncoderChunkedFrameAdapter::new(w, h, 0);
    let f = JxlPixelFormat {
        num_channels: if gray { 1 } else { 3 },
        data_type: if depth == 8 {
            JxlDataType::UINT8
        } else {
            JxlDataType::UINT16
        },
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    assert!(input.set_from_buffer(0, if null { None } else { Some(&data) }, data.len(), f));
    if copy {
        input.copy_buffers();
    }
    let mut aux = aux_stats::seed(7);
    let mut codes = Vec::new();
    let status = frame.compute_encoding_data(
        &input,
        &mut codes,
        if has_aux { Some(&mut aux) } else { None },
    );
    let fields = frame.fields();
    let mut out = format!(
        "{} {} {}",
        status_code(&status),
        frame.header().flags,
        fields.coeff_order_size
    );
    plane(&mut out, fields.ac_strategy.layers());
    plane(&mut out, &fields.raw_quant_field);
    plane(&mut out, &fields.epf_sharpness);
    signed_plane(&mut out, &fields.cmap.ytox_map);
    signed_plane(&mut out, &fields.cmap.ytob_map);
    corr(&mut out, &fields.cmap.base());
    plane(&mut out, &fields.quant_dc);
    for c in 0..3 {
        plane(&mut out, fields.dc_storage.plane(c));
    }
    quant(&mut out, frame.quantizer(), frame.matrices(), 3);
    code_output(&mut out, &codes);
    aux_stats::output(&mut out, &aux);
    let mut full_aux = aux_stats::seed(7);
    let full = config
        .create_frame()
        .unwrap()
        .encode_one_shot(&input, if has_aux { Some(&mut full_aux) } else { None });
    write!(
        out,
        " F {} {}",
        full.as_ref().err().map_or(0, |s| *s as i32),
        full.as_ref()
            .map_or_else(|_| "-".to_owned(), |w| hex(w.as_slice()))
    )
    .unwrap();
    aux_stats::output(&mut out, &full_aux);
    out
}

fn observe_image(args: &mut std::str::SplitWhitespace<'_>, alpha: bool) -> String {
    let w = number(args);
    let h = number(args);
    let gray = number::<u32>(args) != 0;
    let depth = number::<u32>(args);
    let has_aux = number::<u32>(args) != 0;
    let data = unhex(args.next().unwrap());
    let supplied = unhex(args.next().unwrap());
    let constructor = if alpha {
        LosslessThunderConfig::new_with_alpha
    } else {
        LosslessThunderConfig::new
    };
    let mut config = constructor(
        w,
        h,
        gray,
        if depth == 8 {
            LosslessSampleDepth::Uint8
        } else {
            LosslessSampleDepth::Uint16
        },
    )
    .unwrap();
    let mut aux = aux_stats::seed(7);
    let result = config.encode_pixels(&data, if has_aux { Some(&mut aux) } else { None });
    let mut out = format!(
        "{} {}",
        result.as_ref().err().map_or(0, |s| *s as i32),
        result
            .as_ref()
            .map_or_else(|_| "-".to_owned(), |bytes| hex(bytes.as_slice()))
    );
    aux_stats::output(&mut out, &aux);
    if let Ok(bytes) = result {
        assert_eq!(bytes.as_slice(), supplied);
        // Independent C++ decoders must recover the original lossless pixels
        // from both the source stream and the supplied native stream. Rust
        // complete-image decoding remains a separate, unfinished boundary.
        for _ in 0..2 {
            write!(
                out,
                " D 0 {w} {h} {depth} {} {}",
                if gray { 1 } else { 3 },
                hex(&data)
            )
            .unwrap();
        }
    }
    out
}

pub(super) fn observe_bundle(out: &mut String, b: &jxl::jxl::image_bundle::ImageBundle<'_>) {
    write!(
        out,
        " B {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        u32::from(b.have_memory_manager()),
        u32::from(b.jpeg_data.is_some()),
        b.color_transform.0,
        b.origin.x0,
        b.origin.y0,
        b.duration,
        b.timecode,
        u32::from(b.use_for_next_frame),
        u32::from(b.blend),
        b.blendmode.0,
        hex(&b.name),
        u32::from(b.metadata().is_some()),
        b.extra_channels().len(),
        b.decoded_bytes(),
        b.color().xsize()
    )
    .unwrap();
    for c in 0..3 {
        plane(out, b.color().plane(c));
    }
    frame::subsampling(out, &b.chroma_subsampling);
    metadata::color_bundle(out, b.c_current());
}

fn observe_owners(args: &mut std::str::SplitWhitespace<'_>) -> String {
    use jxl::jxl::{
        enc_cache::PassesEncoderState, image_bundle::ImageBundle, image_metadata::ImageMetadata,
        noise::NoiseParams,
    };
    let with_metadata = number::<u32>(args) != 0;
    let moves = number::<usize>(args);
    let decoded = number(args);
    let mut noise = NoiseParams {
        lut: std::array::from_fn(|_| f32::from_bits(number(args))),
    };
    let mut out = format!("N {}", u32::from(noise.has_any()));
    noise.clear();
    write!(out, " {}", u32::from(noise.has_any())).unwrap();
    for v in noise.lut {
        write!(out, " {}", v.to_bits()).unwrap();
    }
    let metadata = ImageMetadata::default();
    let mut bundle = ImageBundle::new(with_metadata.then_some(&metadata));
    bundle.set_decoded_bytes(decoded);
    for _ in 0..moves {
        bundle = std::convert::identity(bundle);
    }
    observe_bundle(&mut out, &bundle);
    let mut state = PassesEncoderState::new();
    for _ in 0..moves {
        state = std::convert::identity(state);
    }
    write!(
        out,
        " E {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        u32::from(state.have_memory_manager()),
        u32::from(state.streaming_mode),
        u32::from(state.initialize_global_state),
        state.dc_group_index,
        state.coeffs.len(),
        state.special_frames.len(),
        state.progressive_splitter.get_num_passes(),
        state.passes.len(),
        state.histogram_idx.len(),
        state.used_acs,
        state.used_orders.len(),
        state.x_qm_multiplier.to_bits(),
        state.b_qm_multiplier.to_bits(),
        state.shared.num_histograms
    )
    .unwrap();
    enc_frame::params(&mut out, &state.cparams);
    plane(&mut out, &state.initial_quant_masking1x1);
    let shared = &mut state.shared;
    write!(
        out,
        " S {} {} {} {}",
        shared.coeff_order_size,
        shared.coeff_orders.len(),
        u32::from(std::ptr::eq(&*shared.dc(), &shared.dc_storage)),
        shared.block_ctx_map.num_ctxs
    )
    .unwrap();
    write!(
        out,
        " {} {}",
        shared.block_ctx_map.num_dc_ctxs,
        hex(&shared.block_ctx_map.ctx_map)
    )
    .unwrap();
    for threshold in &shared.block_ctx_map.dc_thresholds {
        write!(out, " {}", threshold.len()).unwrap();
    }
    write!(out, " {}", shared.block_ctx_map.qf_thresholds.len()).unwrap();
    for reference in shared.reference_frames.borrow().iter() {
        write!(
            out,
            " R {} {}",
            u32::from(reference.frame.is_some()),
            u32::from(reference.ib_is_in_xyb)
        )
        .unwrap();
        observe_bundle(&mut out, reference.frame.as_deref().unwrap());
    }
    for dc in shared.dc_frames.borrow().iter() {
        for c in 0..3 {
            plane(&mut out, dc.plane(c));
        }
    }
    let features = &mut shared.image_features;
    write!(
        out,
        " H {} {}",
        u32::from(features.patches.borrow().have_memory_manager()),
        u32::from(features.patches.borrow().has_any())
    )
    .unwrap();
    for n in features.patches.borrow().storage_sizes() {
        write!(out, " {n}").unwrap();
    }
    shared.bind_own_reference_frames();
    write!(
        out,
        " {}",
        u32::from(
            shared
                .image_features
                .patches
                .borrow()
                .reference_frame_binding()
                .is_some()
        )
    )
    .unwrap();
    let mut splines = shared.image_features.splines.borrow_mut();
    for _ in 0..=moves {
        write!(
            out,
            " L {} {} {} {} {}",
            u32::from(splines.have_memory_manager()),
            u32::from(splines.has_any()),
            splines.quantization_adjustment(),
            splines.quantized_splines().len(),
            splines.starting_points().len()
        )
        .unwrap();
        for n in splines.storage_sizes() {
            write!(out, " {n}").unwrap();
        }
        write!(
            out,
            " {} {}",
            u32::from(splines.have_segment_indices()),
            u32::from(splines.have_segment_y_start())
        )
        .unwrap();
        splines.clear();
    }
    out
}

fn observe_acimage(args: &mut std::str::SplitWhitespace<'_>) -> String {
    use jxl::jxl::dct_util::{AcImage, AcPtrMut, AcType, ConstAcPtr};
    let kind = if number::<u32>(args) == 0 {
        AcType::I16
    } else {
        AcType::I32
    };
    let empty = number::<u32>(args) != 0;
    let w = number::<usize>(args);
    let h = number::<usize>(args);
    let mode = number::<usize>(args);
    let result = if empty {
        Ok(Box::new(AcImage::empty(kind)))
    } else {
        AcImage::make(kind, w, h)
    };
    let mut out = result.as_ref().err().map_or(0, |s| *s as i32).to_string();
    let mut img = match result {
        Ok(v) => v,
        Err(_) => return out,
    };
    if !empty {
        for c in 0..3 {
            for y in 0..h {
                // Seed physical padding as well; zero-fill must leave it unchanged.
                let count = img.pixels_per_row();
                for x in 0..count {
                    let value = (c * 1931 + y * 997 + x * 31337) as i32;
                    match img.plane_row_mut(c, y, x) {
                        AcPtrMut::I16(row) => row[0] = value as i16,
                        AcPtrMut::I32(row) => row[0] = value,
                    }
                }
            }
        }
    }
    if mode == 1 {
        img.zero_fill_plane(1);
    }
    if mode == 2 {
        img.zero_fill();
    }
    let img = std::convert::identity(img);
    write!(
        out,
        " {} {} {}",
        img.image_type() as i32,
        img.pixels_per_row(),
        u32::from(img.is_empty())
    )
    .unwrap();
    if !empty {
        for c in 0..3 {
            for y in 0..h {
                for x in 0..img.pixels_per_row() {
                    let value = match img.plane_row(c, y, x) {
                        ConstAcPtr::I16(row) => i32::from(row[0]),
                        ConstAcPtr::I32(row) => row[0],
                    };
                    write!(out, " {value}").unwrap();
                }
            }
        }
    }
    out
}

fn encoded_corr(
    prefix_bits: usize,
    default: bool,
    factor: u32,
    x: u16,
    b: u16,
    dcx: u8,
    dcb: u8,
) -> Vec<u8> {
    let mut w = BitWriter::new();
    prefix(&mut w, prefix_bits);
    w.with_max_bits(
        128,
        LayerType::Header,
        None,
        |w, _| {
            w.write(1, u64::from(default));
            if !default {
                jxl::jxl::enc_fields::u32_coder::write(COLOR_FACTOR_DIST, factor, w)?;
                w.write(16, u64::from(x));
                w.write(16, u64::from(b));
                w.write(8, u64::from(dcx));
                w.write(8, u64::from(dcb));
            }
            w.zero_pad_to_byte();
            Ok(())
        },
        false,
    )
    .unwrap();
    w.get_span().to_vec()
}
#[test]
fn quantizer_construction_matrix_reborrowing_mixed_precision_and_multiplier_clear_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let words = [
        0u32, 0x80000000, 1, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fa12345,
        0xffc98765, 0x7f7fffff,
    ];
    for (i, dc) in [0, 1, -1, 64, -64, i32::MIN, i32::MAX]
        .into_iter()
        .enumerate()
    {
        for (j, global) in [0, 1, -1, 1024, 65536, i32::MIN, i32::MAX]
            .into_iter()
            .enumerate()
        {
            for def in [0, 1] {
                for custom in [0, 1] {
                    for clear in [0, 1] {
                        let mut request = format!("ss-quant {def} {dc} {global} {custom} {clear}");
                        for k in 0..6 {
                            write!(request, " {}", words[(i * 3 + j + k) % words.len()]).unwrap();
                        }
                        cases.push(request);
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn color_correlation_setters_ratios_and_partial_decode_preserve_every_defined_float_bit_and_reader_status()
 {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let half = [
        0u16, 0x8000, 1, 0x3c00, 0x4400, 0x4401, 0xc401, 0x7c00, 0x7e01,
    ];
    for prefix_bits in [0, 1, 7] {
        for default in [false, true] {
            for (i, &x) in half.iter().enumerate() {
                for (j, &b) in half.iter().enumerate() {
                    let data = encoded_corr(
                        prefix_bits,
                        default,
                        [84, 256, 2, 257, 65793][(i + j) % 5],
                        x,
                        b,
                        (i * 37) as u8,
                        (j * 29) as u8,
                    );
                    for len in [0, data.len() / 2, data.len()] {
                        cases.push(format!(
                            "ss-corr {} {} {} {} {} {prefix_bits} {}",
                            [0u32, 1, 84, 256, u32::MAX][(i + j) % 5],
                            [0, 1, -128, i32::MIN, i32::MAX][i % 5],
                            [0, -1, 127, i32::MAX, i32::MIN][j % 5],
                            (i as i32 - 4) * 31337,
                            (j as i32 - 4) * 25113,
                            hex(&data[..len])
                        ));
                    }
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn color_map_construction_retains_default_vs_computed_dc_factors_tiles_zero_samples_and_delegated_decode()
 {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for w in [1usize, 63, 64, 65, 129, 513, 2049] {
        for h in [1usize, 63, 64, 65, 129] {
            for xyb in [0, 1] {
                for default in [false, true] {
                    let data = encoded_corr(3, default, 84, 0x3c00, 0xbc00, 127, 128);
                    cases.push(format!("ss-map 0 {w} {h} {xyb} 3 {}", hex(&data)));
                }
            }
        }
    }
    for def in [0, 1] {
        for (w, h) in [
            (0, 0),
            (usize::MAX, usize::MAX),
            ((u32::MAX as usize + 1) * 64, 1),
            (1, (u32::MAX as usize + 1) * 64),
        ] {
            cases.push(format!("ss-map {def} {w} {h} 1 0 01"));
        }
    }
    compare(cases);
}
#[test]
fn strategy_storage_fill_count_raw_type_promotion_cached_rows_and_moves_match_original() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for (w, h) in [(1usize, 1usize), (3, 2), (17, 9), (129, 3)] {
        for mode in 0..3 {
            for kind in [0u32, 1, 26, 27, 127, 128, 255, 256, u32::MAX] {
                for do_move in [0, 1] {
                    cases.push(format!(
                        "ss-acs 0 {w} {h} {mode} 0 0 {} {} {kind} {do_move}",
                        w.div_ceil(2),
                        h.div_ceil(2)
                    ));
                }
            }
        }
    }
    for kind in [0u32, 127, u32::MAX] {
        cases.push(format!("ss-acs 1 0 0 0 0 0 0 0 {kind} 0"));
    }
    for (w, h) in [(u32::MAX as usize + 1, 1), (1, u32::MAX as usize + 1)] {
        cases.push(format!("ss-acs 0 {w} {h} 0 0 0 0 0 0 0"));
    }
    compare(cases);
}
#[test]
fn pixel_components_and_allocated_fields_match_original_compute_encoding_data_and_complete_one_shot_frames()
 {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut random = 0x193b_610a_5981_e2cdu64;
    for (w, h) in [
        (1, 1),
        (17, 9),
        (129, 3),
        (257, 3),
        (401, 3),
        (513, 3),
        (2049, 3),
    ] {
        for gray in [0, 1] {
            for depth in [8, 16] {
                for mode in 0..3 {
                    for has_aux in [0, 1] {
                        for copy in [0, 1] {
                            let count = w * h * if gray == 1 { 1 } else { 3 };
                            let mut data = Vec::new();
                            for i in 0..count {
                                random ^= random << 13;
                                random ^= random >> 7;
                                random ^= random << 17;
                                let value = match mode {
                                    0 => 137,
                                    1 => i * 173 + 61,
                                    _ => random as usize,
                                };
                                if depth == 8 {
                                    data.push(value as u8);
                                } else {
                                    data.extend_from_slice(&(value as u16).to_ne_bytes());
                                }
                            }
                            cases.push(format!(
                                "ss-frame {w} {h} {gray} {depth} 0 {has_aux} {copy} {}",
                                hex(&data)
                            ));
                        }
                    }
                }
            }
        }
    }
    for gray in [0, 1] {
        for depth in [8, 16] {
            cases.push(format!(
                "ss-frame 17 9 {gray} {depth} 1 1 0 {}",
                hex(&vec![
                    0;
                    17 * 9
                        * if gray == 1 { 1 } else { 3 }
                        * if depth == 8 { 1 } else { 2 }
                ])
            ));
        }
    }
    compare(cases);
}

#[test]
fn complete_raw_codestreams_match_original_and_cpp_decodes_both_streams_to_exact_integer_samples() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let mut random = 0x7851_92fa_71c3_690bu64;
    for (w, h) in [
        (1, 1),
        (17, 9),
        (129, 3),
        (257, 3),
        (401, 3),
        (513, 3),
        (2049, 3),
        (65, 67),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in 0..3 {
                    let mut data = Vec::new();
                    for i in 0..w * h * if gray { 1 } else { 3 } {
                        let value = match mode {
                            0 => 137,
                            1 => i * 173 + 61,
                            _ => next(&mut random) as usize,
                        };
                        if depth == 8 {
                            data.push(value as u8);
                        } else {
                            data.extend_from_slice(&(value as u16).to_ne_bytes());
                        }
                    }
                    let mut config = LosslessThunderConfig::new(
                        w,
                        h,
                        gray,
                        if depth == 8 {
                            LosslessSampleDepth::Uint8
                        } else {
                            LosslessSampleDepth::Uint16
                        },
                    )
                    .unwrap();
                    let native = config.encode_pixels(&data, None).unwrap();
                    for has_aux in [0, 1] {
                        cases.push(format!(
                            "ss-image {w} {h} {} {depth} {has_aux} {} {}",
                            u32::from(gray),
                            hex(&data),
                            hex(native.as_slice())
                        ));
                    }
                }
            }
        }
    }
    for gray in [0, 1] {
        for depth in [8, 16] {
            let size = 17 * 9 * if gray == 1 { 1 } else { 3 } * if depth == 8 { 1 } else { 2 };
            for len in [0, 1, size / 2, size - 1] {
                for has_aux in [0, 1] {
                    cases.push(format!(
                        "ss-image 17 9 {gray} {depth} {has_aux} {} -",
                        hex(&vec![0; len])
                    ));
                }
            }
        }
    }
    compare(cases);
}

#[test]
fn shared_encoder_reference_feature_and_bundle_owners_match_original_initialized_state() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    let words = [
        0u32, 0x80000000, 1, 0x3a83126f, 0x3a831270, 0xba831270, 0x7f800000, 0xff800000,
        0x7fa12345, 0xffc98765,
    ];
    for metadata in [0, 1] {
        for moves in [0, 1, 3] {
            for decoded in [0, 1, usize::MAX] {
                for offset in 0..words.len() {
                    let mut request = format!("ss-owners {metadata} {moves} {decoded}");
                    for i in 0..8 {
                        write!(request, " {}", words[(offset + i) % words.len()]).unwrap();
                    }
                    cases.push(request);
                }
            }
        }
    }
    cases.push(
        "ss-owners 0 0 0 0 2147483648 1 981668463 981668463 0 2147483648 2141266757".to_owned(),
    );
    compare(cases);
}

#[test]
fn ac_image_both_types_keep_virtual_rows_stride_moves_zero_fill_and_physical_padding() {
    jxl::jxl::hwy::select_scalar_decoder_for_test();
    let mut cases = Vec::new();
    for kind in [0, 1] {
        for mode in 0..3 {
            cases.push(format!("ss-acimage {kind} 1 0 0 {mode}"));
            for (w, h) in [
                (0, 0),
                (1, 1),
                (17, 3),
                (64, 2),
                (129, 5),
                (u32::MAX as usize + 1, 1),
                (1, u32::MAX as usize + 1),
            ] {
                cases.push(format!("ss-acimage {kind} 0 {w} {h} {mode}"));
            }
        }
    }
    compare(cases);
}
