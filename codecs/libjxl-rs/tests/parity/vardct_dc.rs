// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
use jxl::jxl::ac_context::BlockCtxMap;
use jxl::jxl::ac_strategy::{AcStrategyImage, AcStrategyType};
use jxl::jxl::base::rect::Rect;
use jxl::jxl::compressed_dc::{adaptive_dc_smoothing, dequant_dc};
use jxl::jxl::dec_cache::PassesDecoderState;
use jxl::jxl::dec_modular::ModularFrameDecoder;
use jxl::jxl::frame_header::{FrameEncoding, FrameHeader, YCbCrChromaSubsampling};
use jxl::jxl::image::{Image3, Plane, Sample};
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::image_ops::fill_image;
use jxl::jxl::modular::modular_image::Image;
use jxl::jxl::passes_state::initialize_passes_shared_state;
use std::sync::atomic::Ordering;

pub(super) fn handles(kind: &str) -> bool {
    kind.starts_with("vdc-")
}
fn chroma(mode: u32) -> YCbCrChromaSubsampling {
    let mut result = YCbCrChromaSubsampling::default();
    if mode != 0 {
        result
            .set(&[2, 1, 1], &[if mode == 2 { 2 } else { 1 }, 1, 1])
            .unwrap();
    }
    result
}
trait Observation: Sample {
    fn word(self) -> String;
}
impl Observation for f32 {
    fn word(self) -> String {
        self.to_bits().to_string()
    }
}
impl Observation for i32 {
    fn word(self) -> String {
        self.to_string()
    }
}
impl Observation for u8 {
    fn word(self) -> String {
        self.to_string()
    }
}
impl Observation for i8 {
    fn word(self) -> String {
        self.to_string()
    }
}
fn plane<T: Observation>(out: &mut String, p: &Plane<T>) {
    write!(out, " {} {}", p.xsize(), p.ysize()).unwrap();
    for y in 0..p.ysize() {
        for x in 0..p.xsize() {
            write!(out, " {}", p.row(y)[x].word()).unwrap();
        }
    }
}
fn image(out: &mut String, p: &Image3<f32>) {
    for c in 0..3 {
        plane(out, p.plane(c));
    }
}
fn strategy(out: &mut String, s: &AcStrategyImage) {
    out.push_str(" S");
    for y in 0..s.ysize() {
        for x in 0..s.xsize() {
            if !s.is_valid(x, y) {
                out.push_str(" 255");
            } else {
                let ac = s.const_row(y, 0).get(x);
                write!(
                    out,
                    " {}",
                    (ac.raw_strategy() << 1) | u8::from(ac.is_first_block())
                )
                .unwrap();
            }
        }
    }
}
fn state(out: &mut String, s: &PassesDecoderState<'_, '_>) {
    write!(out, " U {}", s.used_acs.load(Ordering::SeqCst)).unwrap();
    let p = &s.shared_storage;
    image(out, &p.dc_storage);
    plane(out, &p.quant_dc);
    plane(out, &p.raw_quant_field);
    plane(out, &p.epf_sharpness);
    plane(out, &p.cmap.ytox_map);
    plane(out, &p.cmap.ytob_map);
    strategy(out, &p.ac_strategy);
    if s.sigma.borrow().xsize() != 0 {
        plane(out, &s.sigma.borrow());
    }
}
fn float(args: &mut std::str::SplitWhitespace<'_>) -> f32 {
    f32::from_bits(number(args))
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut out = String::new();
    if kind == "vdc-smooth" {
        let w = number(args);
        let h = number(args);
        let mode = number(args);
        let factors = [float(args), float(args), float(args)];
        let mut p = Image3::create(w, h).unwrap();
        for c in 0..3 {
            for y in 0..h {
                for x in 0..w {
                    p.plane_row_mut(c, y)[x] = float(args);
                }
            }
        }
        let status = adaptive_dc_smoothing(&factors, &mut p, &chroma(mode), None);
        write!(out, "{}", status_code(&status)).unwrap();
        for c in 0..3 {
            let cs = chroma(mode);
            let width = if w > 2 && h > 2 && !cs.is_444() {
                w >> cs.h_shift(c)
            } else {
                w
            };
            let height = if w > 2 && h > 2 && !cs.is_444() {
                h >> cs.v_shift(c)
            } else {
                h
            };
            write!(out, " {width} {height}").unwrap();
            for y in 0..height {
                for x in 0..width {
                    write!(out, " {}", p.plane_row(c, y)[x].to_bits()).unwrap();
                }
            }
        }
    } else if kind == "vdc-strategy" {
        let w = number(args);
        let h = number(args);
        let n = number(args);
        let mut p = AcStrategyImage::create(w, h).unwrap();
        p.fill_invalid();
        for _ in 0..n {
            let x = number(args);
            let y = number(args);
            let raw = AcStrategyType(number(args));
            let kind = number::<u32>(args);
            let check = number::<u32>(args) != 0;
            let status = if kind != 0 {
                p.set(x, y, raw)
            } else {
                p.set_no_bounds_check(x, y, raw, check)
            };
            write!(out, " {}", status_code(&status)).unwrap();
        }
        strategy(&mut out, &p);
    } else if kind == "vdc-dequant" {
        let x = number(args);
        let y = number(args);
        let w = number(args);
        let h = number(args);
        let iw = number(args);
        let ih = number(args);
        let cs = chroma(number(args));
        let factors = [float(args), float(args), float(args)];
        let mul = float(args);
        let cfl = [float(args), 0.0, float(args)];
        let mut b = BlockCtxMap {
            num_dc_ctxs: number(args),
            ..BlockCtxMap::default()
        };
        for t in &mut b.dc_thresholds {
            let n = number(args);
            *t = (0..n).map(|_| number(args)).collect();
        }
        let mut dc = Image3::create(iw, ih).unwrap();
        let mut q = Plane::create(iw, ih).unwrap();
        let mut input = Image::create(w, h, 8, 3).unwrap();
        for c in 0..3 {
            fill_image(-19.0 - c as f32, dc.plane_mut(c));
            let ch = &mut input.channel[if c < 2 { c ^ 1 } else { c }];
            ch.w >>= cs.h_shift(c);
            ch.h >>= cs.v_shift(c);
            ch.shrink().unwrap();
            for y in 0..ch.h {
                for x in 0..ch.w {
                    ch.row_mut(y)[x] = number(args);
                }
            }
        }
        fill_image(199, &mut q);
        dequant_dc(
            Rect::new(x, y, w, h),
            &mut dc,
            &mut q,
            &input,
            &factors,
            mul,
            &cfl,
            &cs,
            &b,
        );
        out.push('D');
        image(&mut out, &dc);
        plane(&mut out, &q);
    } else {
        assert_eq!(kind, "vdc-read");
        let w = number::<usize>(args);
        let h = number::<usize>(args);
        let group = number(args);
        let kind = number::<u32>(args);
        let cs = chroma(number(args));
        let epf = number(args);
        let offset = number(args);
        let repeat = number(args);
        let global = unhex(args.next().unwrap());
        let payload = unhex(args.next().unwrap());
        let mut m = CodecMetadata::default();
        m.size.set(w * 8, h * 8).unwrap();
        m.m.set_uint_samples(8);
        let mut header = FrameHeader::new(Some(&m));
        header.encoding = FrameEncoding::VAR_DCT;
        header.chroma_subsampling = cs;
        header.loop_filter.epf_iters = epf;
        let dim = header.to_frame_dimensions();
        let mut s = PassesDecoderState::new();
        initialize_passes_shared_state(&header, &mut s.shared_storage, false).unwrap();
        for c in 0..3 {
            fill_image(
                [-19.0, -23.0, -29.0][c],
                s.shared_storage.dc_storage.plane_mut(c),
            );
        }
        fill_image(199, &mut s.shared_storage.quant_dc);
        fill_image(-73, &mut s.shared_storage.raw_quant_field);
        fill_image(201, &mut s.shared_storage.epf_sharpness);
        fill_image(-117, &mut s.shared_storage.cmap.ytox_map);
        fill_image(121, &mut s.shared_storage.cmap.ytob_map);
        s.shared_storage.ac_strategy.fill_invalid();
        s.used_acs.store(0x8000_0000, Ordering::SeqCst);
        if epf != 0 {
            *s.sigma.borrow_mut() =
                Plane::create(dim.xsize_blocks + 4, dim.ysize_blocks + 4).unwrap();
            fill_image(-17.0, &mut s.sigma.borrow_mut());
        }
        let mut d = ModularFrameDecoder::new();
        d.init(&dim);
        let mut gr = BitReader::new(&global);
        d.decode_global_info(&mut gr, &header, false).unwrap();
        gr.close().unwrap();
        for _ in 0..repeat {
            let mut br = BitReader::new(&payload);
            br.skip_bits(offset);
            let status = if kind != 0 {
                d.decode_ac_metadata(&header, group, &mut br, &mut s)
            } else {
                d.decode_var_dct_dc(&header, group, &mut br, &mut s)
            };
            let bits = br.total_bits_consumed();
            let close = br.close();
            write!(
                out,
                " R {} {bits} {}",
                status_code(&status),
                status_code(&close)
            )
            .unwrap();
            state(&mut out, &s);
        }
    }
    out
}
fn append(out: &mut String, value: impl std::fmt::Display) {
    write!(out, " {value}").unwrap();
}
#[test]
fn dc_smoothing_complete_scalar_rows_borders_subsampling_and_exceptional_values() {
    let mut cases = Vec::new();
    for mode in 0..3 {
        for (w, h) in [
            (1, 1),
            (2, 9),
            (9, 2),
            (3, 3),
            (4, 4),
            (7, 5),
            (8, 8),
            (17, 9),
        ] {
            for pattern in 0..10 {
                let mut request = format!(
                    "vdc-smooth {w} {h} {mode} {} {} {}",
                    0.125f32.to_bits(),
                    0.5f32.to_bits(),
                    2.0f32.to_bits()
                );
                for c in 0..3 {
                    for y in 0..h {
                        for x in 0..w {
                            let v = match pattern {
                                0 => 0.0,
                                1 => -0.0,
                                2 => (x + y + c) as f32 / 1024.0,
                                3 => ((x * 17 + y * 31 + c * 13) % 23) as f32 / 512.0,
                                4 => f32::from_bits(if x == w / 2 && y == h / 2 {
                                    0x7fc12345
                                } else {
                                    0x3a012345
                                }),
                                5 => f32::from_bits(if x == w / 2 && y == h / 2 {
                                    0x7f800000
                                } else {
                                    0x3a012345
                                }),
                                6 => f32::from_bits(if x == w / 2 && y == h / 2 {
                                    0x7f812345
                                } else {
                                    0x3a012345
                                }),
                                7 => f32::from_bits(if x == w / 2 && y == h / 2 {
                                    0xffc98765
                                } else if x == w / 2 + 1 {
                                    0x7fc12345
                                } else {
                                    0x3a012345
                                }),
                                8 => f32::from_bits(if x == w / 2 && y == h / 2 {
                                    0xff812345
                                } else if y == h / 2 + 1 {
                                    0x7fc98765
                                } else {
                                    0x3a012345
                                }),
                                _ => f32::from_bits(((x + y + c) % 7) as u32),
                            };
                            append(&mut request, v.to_bits());
                        }
                    }
                }
                cases.push(request);
            }
        }
    }
    compare(cases);
}
#[test]
fn dc_dequantization_channel_order_cfl_buckets_rectangles_and_subsampling() {
    let mut cases = Vec::new();
    for mode in 0..3 {
        for custom in [false, true] {
            for precision in 0..4 {
                for (w, h, x, y) in [(2, 2, 0, 0), (4, 4, 2, 2), (8, 6, 2, 2)] {
                    let mut request = format!(
                        "vdc-dequant {x} {y} {w} {h} {} {} {mode} {} {} {} {} {} {} {}",
                        w + x + 2,
                        h + y + 2,
                        0.25f32.to_bits(),
                        0.5f32.to_bits(),
                        1.75f32.to_bits(),
                        (1.0f32 / (1 << precision) as f32).to_bits(),
                        (-0.75f32).to_bits(),
                        1.25f32.to_bits(),
                        if custom { 24 } else { 1 }
                    );
                    request.push_str(if custom {
                        " 2 -7 13 1 0 3 -19 -3 17"
                    } else {
                        " 0 0 0"
                    });
                    let cs = chroma(mode);
                    for c in 0..3 {
                        for iy in 0..h >> cs.v_shift(c) {
                            for ix in 0..w >> cs.h_shift(c) {
                                append(&mut request, (ix * 19 + iy * 7 + c * 11) as i32 - 29);
                            }
                        }
                    }
                    cases.push(request);
                }
            }
        }
    }
    compare(cases);
}
#[test]
fn strategy_setters_rows_overlap_partial_mutations_and_unchecked_overwrite() {
    let mut cases = Vec::new();
    for raw in 0..27 {
        let base = format!("vdc-strategy 40 40 3 1 1 {raw} 0 1 0 0 5 0 1 1 1 0 1 0");
        cases.push(base);
        cases.push(format!("vdc-strategy 40 40 2 0 0 {raw} 1 0 0 0 0 1 0"));
    }
    compare(cases);
}
fn prefixed(source: &str, offset: usize) -> String {
    if offset == 0 {
        return source.into();
    }
    let bytes = unhex(source);
    let mut shifted = vec![0; (bytes.len() * 8 + offset).div_ceil(8)];
    shifted[0] = (1 << offset) - 1;
    for (i, b) in bytes.into_iter().enumerate() {
        shifted[(8 * i + offset) / 8] |= b << offset;
        shifted[(8 * i + offset) / 8 + 1] |= b >> (8 - offset);
    }
    hex(&shifted)
}
#[test]
fn vardct_dc_groups_precision_and_ac_metadata_all_strategies_failure_and_reuse() {
    let global = original_observations(&["vdc-global".into()]).remove(0);
    let mut cases = Vec::new();
    for precision in 0..4 {
        for (w, h) in [(1, 1), (3, 5), (8, 8), (32, 32)] {
            let source =
                original_observations(&[format!("vdc-source {w} {h} 0 0 0 {precision}")]).remove(0);
            for len in [0, 1, source.len() / 4, source.len() / 2] {
                let payload = if len == 0 { "-" } else { &source[..len * 2] };
                cases.push(format!("vdc-read {w} {h} 0 0 0 0 0 2 {global} {payload}"));
            }
        }
    }
    for raw in 0..27 {
        for mutation in 0..5 {
            let source = original_observations(&[format!("vdc-source 32 32 1 {raw} {mutation} 0")])
                .remove(0);
            for epf in [0, 1] {
                cases.push(format!("vdc-read 32 32 0 1 0 {epf} 0 2 {global} {source}"));
            }
            for len in [0, 1, source.len() / 4] {
                let payload = if len == 0 { "-" } else { &source[..len * 2] };
                cases.push(format!("vdc-read 32 32 0 1 0 0 0 1 {global} {payload}"));
            }
        }
    }
    for mode in 1..3 {
        for precision in 0..4 {
            let source =
                original_observations(&[format!("vdc-source 8 8 0 0 0 {precision} {mode}")])
                    .remove(0);
            for offset in [0, 3, 7] {
                cases.push(format!(
                    "vdc-read 8 8 0 0 {mode} 0 {offset} 2 {global} {}",
                    prefixed(&source, offset)
                ));
            }
        }
        for raw in [0, 4, 5, 26] {
            let source =
                original_observations(&[format!("vdc-source 32 32 1 {raw} 0 0")]).remove(0);
            cases.push(format!("vdc-read 32 32 0 1 {mode} 0 0 1 {global} {source}"));
        }
    }
    for raw in [0, 4, 26] {
        let source = original_observations(&[format!("vdc-source 32 32 1 {raw} 0 0")]).remove(0);
        for offset in [3, 7] {
            cases.push(format!(
                "vdc-read 32 32 0 1 0 1 {offset} 1 {global} {}",
                prefixed(&source, offset)
            ));
        }
    }
    for (w, h, sw, sh) in [(257, 8, 1, 8), (8, 257, 8, 1)] {
        for kind in 0..2 {
            let source =
                original_observations(&[format!("vdc-source {sw} {sh} {kind} 0 0 2")]).remove(0);
            cases.push(format!(
                "vdc-read {w} {h} 1 {kind} 0 {kind} 0 2 {global} {source}"
            ));
        }
    }
    compare(cases);
}
