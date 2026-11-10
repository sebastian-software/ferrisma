// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Original N=4 NEON load/store branches from pinned src/Transform_inl.h.
use super::{F, f_to_half, half_to_f};
use crate::skcms::transform_types::Op;
use std::arch::aarch64::*;
use std::mem::transmute;

fn from_native(v: float32x4_t) -> F {
    // SAFETY: these are the same four initialized binary32 lane representations.
    F(unsafe { transmute::<float32x4_t, [f32; 4]>(v) })
}
fn native(v: F) -> float32x4_t {
    // SAFETY: these are the same four initialized binary32 lane representations.
    unsafe { transmute::<[f32; 4], float32x4_t>(v.0) }
}
fn from_u16(v: uint16x4_t) -> F {
    // SAFETY: both widening and conversion accept every initialized integer lane.
    from_native(unsafe { vcvtq_f32_u32(vmovl_u16(v)) })
}
fn from_u32(v: uint32x4_t) -> F {
    // SAFETY: native unsigned conversion accepts every initialized integer lane.
    from_native(unsafe { vcvtq_f32_u32(v) })
}
fn fixed(v: F, scale: f32) -> uint32x4_t {
    // SAFETY: to_fixed produces four initialized unsigned lane representations.
    unsafe { transmute::<[u32; 4], uint32x4_t>((v * scale).fixed()) }
}
fn normalized_u16(v: F) -> uint16x4_t {
    // SAFETY: the source converts the original fused 65535*v+.5 expression to
    // unsigned words, then truncates to sixteen bits without saturating.
    unsafe {
        vmovn_u32(transmute::<[u32; 4], uint32x4_t>(
            v.madd(F::splat(65535.0), F::splat(0.5)).u32(),
        ))
    }
}
fn halves(v: F) -> uint16x4_t {
    // SAFETY: half conversion returns the exact four initialized 16-bit words.
    unsafe { transmute::<[u16; 4], uint16x4_t>(f_to_half(v)) }
}
fn from_halves(v: uint16x4_t) -> F {
    // SAFETY: the original half intrinsic consumes every 16-bit representation.
    half_to_f(unsafe { transmute::<uint16x4_t, [u16; 4]>(v) })
}
/// # Safety
/// Source covers four complete pixels with the original typed alignment.
pub(super) unsafe fn load(op: Op, src: *const u8, out: &mut [F; 4]) -> bool {
    use Op::*;
    // SAFETY: caller supplies every original source span. Each interleaved
    // intrinsic has the original lane/element width and reads no extra pixels.
    unsafe {
        match op {
            load_a8 | load_g8 => {
                let bytes = vcreate_u8(src.cast::<u32>().read_unaligned() as u64);
                let value = from_u16(vget_low_u16(vmovl_u8(bytes))) * (1.0 / 255.0);
                if op == load_a8 {
                    out[3] = value;
                } else {
                    out[..3].fill(value);
                }
            }
            load_ga88 => {
                let words = vld1_u16(src.cast());
                out[..3].fill(from_u16(vand_u16(words, vdup_n_u16(255))) * (1.0 / 255.0));
                out[3] = from_u16(vshr_n_u16::<8>(words)) * (1.0 / 255.0);
            }
            load_4444 => {
                let w = vld1_u16(src.cast());
                out[0] = from_u16(vshr_n_u16::<12>(w)) * (1.0 / 15.0);
                out[1] = from_u16(vand_u16(vshr_n_u16::<8>(w), vdup_n_u16(15))) * (1.0 / 15.0);
                out[2] = from_u16(vand_u16(vshr_n_u16::<4>(w), vdup_n_u16(15))) * (1.0 / 15.0);
                out[3] = from_u16(vand_u16(w, vdup_n_u16(15))) * (1.0 / 15.0);
            }
            load_565 => {
                let w = vld1_u16(src.cast());
                out[0] = from_u16(vand_u16(w, vdup_n_u16(31))) * (1.0 / 31.0);
                out[1] = from_u16(vand_u16(w, vdup_n_u16(63 << 5))) * (1.0 / (63 << 5) as f32);
                out[2] = from_u16(vand_u16(w, vdup_n_u16(31 << 11))) * (1.0 / (31 << 11) as f32);
            }
            load_888 => {
                let mut v = uint8x8x3_t(vdup_n_u8(0), vdup_n_u8(0), vdup_n_u8(0));
                v = vld3_lane_u8::<0>(src, v);
                v = vld3_lane_u8::<2>(src.add(3), v);
                v = vld3_lane_u8::<4>(src.add(6), v);
                v = vld3_lane_u8::<6>(src.add(9), v);
                out[0] = from_u16(vreinterpret_u16_u8(v.0)) * (1.0 / 255.0);
                out[1] = from_u16(vreinterpret_u16_u8(v.1)) * (1.0 / 255.0);
                out[2] = from_u16(vreinterpret_u16_u8(v.2)) * (1.0 / 255.0);
            }
            load_8888 => {
                let w = vld1q_u32(src.cast());
                out[0] = from_u32(vandq_u32(w, vdupq_n_u32(255))) * (1.0 / 255.0);
                out[1] = from_u32(vandq_u32(vshrq_n_u32::<8>(w), vdupq_n_u32(255))) * (1.0 / 255.0);
                out[2] =
                    from_u32(vandq_u32(vshrq_n_u32::<16>(w), vdupq_n_u32(255))) * (1.0 / 255.0);
                out[3] = from_u32(vshrq_n_u32::<24>(w)) * (1.0 / 255.0);
            }
            load_1010102 | load_101010x_XR => {
                let w = vld1q_u32(src.cast());
                let mask = vdupq_n_u32(1023);
                let channels = [
                    vandq_u32(w, mask),
                    vandq_u32(vshrq_n_u32::<10>(w), mask),
                    vandq_u32(vshrq_n_u32::<20>(w), mask),
                ];
                for (out, c) in out[..3].iter_mut().zip(channels) {
                    *out = if op == load_1010102 {
                        from_u32(c) * (1.0 / 1023.0)
                    } else {
                        from_u32(vsubq_u32(c, vdupq_n_u32(384))) / 510.0
                    };
                }
                if op == load_1010102 {
                    out[3] = from_u32(vshrq_n_u32::<30>(w)) * (1.0 / 3.0);
                }
            }
            load_161616LE | load_161616BE | load_hhh => {
                let v = vld3_u16(src.cast());
                let words = [v.0, v.1, v.2];
                for (out, mut w) in out[..3].iter_mut().zip(words) {
                    if op == load_hhh {
                        *out = from_halves(w);
                    } else {
                        if op == load_161616BE {
                            w = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(w)));
                        }
                        *out = from_u16(w) * (1.0 / 65535.0);
                    }
                }
            }
            load_16161616LE | load_16161616BE | load_hhhh => {
                let v = vld4_u16(src.cast());
                let words = [v.0, v.1, v.2, v.3];
                for (out, mut w) in out.iter_mut().zip(words) {
                    if op == load_hhhh {
                        *out = from_halves(w);
                    } else {
                        if op == load_16161616BE {
                            w = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(w)));
                        }
                        *out = from_u16(w) * (1.0 / 65535.0);
                    }
                }
            }
            load_fff => {
                let v = vld3q_f32(src.cast());
                out[0] = from_native(v.0);
                out[1] = from_native(v.1);
                out[2] = from_native(v.2);
            }
            load_ffff => {
                let v = vld4q_f32(src.cast());
                out[0] = from_native(v.0);
                out[1] = from_native(v.1);
                out[2] = from_native(v.2);
                out[3] = from_native(v.3);
            }
            _ => return false,
        }
    }
    true
}
/// # Safety
/// Destination covers four complete pixels with original typed alignment.
pub(super) unsafe fn store(op: Op, dst: *mut u8, rgba: [F; 4]) -> bool {
    use Op::*;
    let [r, g, b, a] = rgba;
    // SAFETY: caller establishes every original output span. Vector truncation
    // is deliberately non-saturating; interleaved stores preserve source lanes.
    unsafe {
        match op {
            store_a8 | store_g8 => {
                let q = fixed(if op == store_a8 { a } else { g }, 255.0);
                let bytes = vmovn_u16(vcombine_u16(vmovn_u32(q), vdup_n_u16(0)));
                dst.cast::<u32>()
                    .write_unaligned(vget_lane_u32::<0>(vreinterpret_u32_u8(bytes)));
            }
            store_ga88 => {
                vst1_u16(
                    dst.cast(),
                    vorr_u16(
                        vmovn_u32(fixed(g, 255.0)),
                        vshl_n_u16::<8>(vmovn_u32(fixed(a, 255.0))),
                    ),
                );
            }
            store_4444 => {
                vst1_u16(
                    dst.cast(),
                    vorr_u16(
                        vorr_u16(
                            vshl_n_u16::<12>(vmovn_u32(fixed(r, 15.0))),
                            vshl_n_u16::<8>(vmovn_u32(fixed(g, 15.0))),
                        ),
                        vorr_u16(
                            vshl_n_u16::<4>(vmovn_u32(fixed(b, 15.0))),
                            vmovn_u32(fixed(a, 15.0)),
                        ),
                    ),
                );
            }
            store_565 => {
                vst1_u16(
                    dst.cast(),
                    vorr_u16(
                        vorr_u16(
                            vmovn_u32(fixed(r, 31.0)),
                            vshl_n_u16::<5>(vmovn_u32(fixed(g, 63.0))),
                        ),
                        vshl_n_u16::<11>(vmovn_u32(fixed(b, 31.0))),
                    ),
                );
            }
            store_888 => {
                let v = uint8x8x3_t(
                    vreinterpret_u8_u16(vmovn_u32(fixed(r, 255.0))),
                    vreinterpret_u8_u16(vmovn_u32(fixed(g, 255.0))),
                    vreinterpret_u8_u16(vmovn_u32(fixed(b, 255.0))),
                );
                vst3_lane_u8::<0>(dst, v);
                vst3_lane_u8::<2>(dst.add(3), v);
                vst3_lane_u8::<4>(dst.add(6), v);
                vst3_lane_u8::<6>(dst.add(9), v);
            }
            store_8888 => {
                vst1q_u32(
                    dst.cast(),
                    vorrq_u32(
                        vorrq_u32(fixed(r, 255.0), vshlq_n_u32::<8>(fixed(g, 255.0))),
                        vorrq_u32(
                            vshlq_n_u32::<16>(fixed(b, 255.0)),
                            vshlq_n_u32::<24>(fixed(a, 255.0)),
                        ),
                    ),
                );
            }
            store_1010102 => {
                vst1q_u32(
                    dst.cast(),
                    vorrq_u32(
                        vorrq_u32(fixed(r, 1023.0), vshlq_n_u32::<10>(fixed(g, 1023.0))),
                        vorrq_u32(
                            vshlq_n_u32::<20>(fixed(b, 1023.0)),
                            vshlq_n_u32::<30>(fixed(a, 3.0)),
                        ),
                    ),
                );
            }
            store_161616LE | store_161616BE | store_hhh => {
                let mut words = rgba[..3].iter().copied().map(|x| {
                    if op == store_hhh {
                        halves(x)
                    } else {
                        normalized_u16(x)
                    }
                });
                let mut r = words.next().unwrap();
                let mut g = words.next().unwrap();
                let mut b = words.next().unwrap();
                if op == store_161616BE {
                    r = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(r)));
                    g = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(g)));
                    b = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(b)));
                }
                vst3_u16(dst.cast(), uint16x4x3_t(r, g, b));
            }
            store_16161616LE | store_16161616BE | store_hhhh => {
                let mut words = rgba.into_iter().map(|x| {
                    if op == store_hhhh {
                        halves(x)
                    } else {
                        normalized_u16(x)
                    }
                });
                let mut r = words.next().unwrap();
                let mut g = words.next().unwrap();
                let mut b = words.next().unwrap();
                let mut a = words.next().unwrap();
                if op == store_16161616BE {
                    r = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(r)));
                    g = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(g)));
                    b = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(b)));
                    a = vreinterpret_u16_u8(vrev16_u8(vreinterpret_u8_u16(a)));
                }
                vst4_u16(dst.cast(), uint16x4x4_t(r, g, b, a));
            }
            store_fff => vst3q_f32(dst.cast(), float32x4x3_t(native(r), native(g), native(b))),
            store_ffff => vst4q_f32(
                dst.cast(),
                float32x4x4_t(native(r), native(g), native(b), native(a)),
            ),
            _ => return false,
        }
    }
    true
}
