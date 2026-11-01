// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_strategy::{AcStrategyImage, AcStrategyType};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::chroma_from_luma::{COLOR_FACTOR_DIST, ColorCorrelation, ColorCorrelationMap};
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
use jxl::jxl::image::{Image3, Plane, Sample};
use jxl::jxl::quant_weights::DequantMatrices;
use jxl::jxl::quantizer::Quantizer;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("ss-")
}
fn corr(out: &mut String, c: &ColorCorrelation) {
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
fn plane<T: Sample>(out: &mut String, p: &Plane<T>) {
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
        corr(&mut out, c.base());
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
        corr(&mut out, c.base());
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
    let status = (|| -> Status {
        frame.allocate_frame_fields()?;
        let mut color = Image3::<f32>::create(w.div_ceil(8) * 8, h.div_ceil(8) * 8)?;
        color.shrink_to(w, h)?;
        let mut alpha = false;
        config.copy_color_channels(&input, Rect::new(0, 0, w, h), &mut color, &mut alpha)?;
        jxl::jxl::image_ops::pad_image_to_block_multiple_in_place(&mut color, 8)?;
        frame.prepare_modular(&color)?;
        frame.compute_modular_tokens()?;
        frame.encode_groups(&mut codes, if has_aux { Some(&mut aux) } else { None })
    })();
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
    corr(&mut out, fields.cmap.base());
    plane(&mut out, &fields.quant_dc);
    for c in 0..3 {
        plane(&mut out, fields.dc_storage.plane(c));
    }
    quant(&mut out, frame.quantizer(), frame.matrices(), 3);
    code_output(&mut out, &codes);
    aux_stats::output(&mut out, &aux);
    let full = match status {
        Ok(()) => frame.finalize_groups(&codes, if has_aux { Some(&mut aux) } else { None }),
        Err(s) => Err(s),
    };
    write!(
        out,
        " F {} {}",
        full.as_ref().err().map_or(0, |s| *s as i32),
        full.as_ref()
            .map_or_else(|_| "-".to_owned(), |w| hex(w.get_span()))
    )
    .unwrap();
    aux_stats::output(&mut out, &aux);
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
