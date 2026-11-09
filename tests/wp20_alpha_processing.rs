//! WP20 parity test: the NEON variants of `src/dsp/alpha_processing_neon.c`
//! against the C translations they replace (`src/dsp/alpha_processing.rs`),
//! Rust against Rust, and the dispatch functions against the NEON entry
//! points. Buffers end exactly at the last byte the C function touches, and
//! start at every offset 0 to 3 of a larger allocation, so an access outside
//! the C function's range panics.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::{Rng, widths};
use webp::dsp::alpha_processing as c;
use webp::dsp::alpha_processing_neon as neon;

/// A buffer of `len` bytes after `offset` bytes of padding, from `rng`.
fn padded(rng: &mut Rng, offset: usize, len: usize) -> Vec<u8> {
    rng.edge_bytes(offset + len)
}

#[test]
fn apply_alpha_multiply_covers_every_alpha_and_value() {
    // Row a holds the 256 values x with alpha a, as RGBA and as ARGB.
    for alpha_first in [false, true] {
        let mut plane = vec![0_u8; 256 * 256 * 4];
        for a in 0..256 {
            for x in 0..256 {
                let pixel = &mut plane[(a * 256 + x) * 4..][..4];
                pixel.fill(x as u8);
                pixel[if alpha_first { 0 } else { 3 }] = a as u8;
            }
        }
        let mut expected = plane.clone();
        c::apply_alpha_multiply_c(&mut expected, alpha_first, 256, 256, 1024);
        neon::apply_alpha_multiply(&mut plane, alpha_first, 256, 256, 1024);
        assert_eq!(plane, expected, "alpha_first={alpha_first}");
    }
}

#[test]
fn apply_alpha_multiply_matches_c() {
    let mut rng = Rng(0x2020_0001);
    let mut cases = 0;
    for alpha_first in [false, true] {
        for w in widths() {
            for h in [1_usize, 2, 3] {
                for pad in [0, 5] {
                    let stride = 4 * w + pad;
                    for offset in 0..4 {
                        let len = if h == 0 { 0 } else { (h - 1) * stride + 4 * w };
                        let buffer = padded(&mut rng, offset, len);
                        let mut expected = buffer.clone();
                        let mut actual = buffer.clone();
                        let mut dispatched = buffer;
                        let (w, h, s) = (w as i32, h as i32, stride as i32);
                        c::apply_alpha_multiply_c(&mut expected[offset..], alpha_first, w, h, s);
                        neon::apply_alpha_multiply(&mut actual[offset..], alpha_first, w, h, s);
                        c::webp_apply_alpha_multiply(
                            &mut dispatched[offset..],
                            alpha_first,
                            w,
                            h,
                            s,
                        );
                        assert_eq!(actual, expected, "w={w} h={h} stride={s} offset={offset}");
                        assert_eq!(dispatched, expected);
                        cases += 1;
                    }
                }
            }
        }
    }
    assert!(cases > 1000);
}

#[test]
fn dispatch_and_extract_alpha_match_c() {
    let mut rng = Rng(0x2020_0002);
    for w in widths() {
        for h in [1_usize, 2, 4] {
            for pad in [0, 3] {
                // The alpha byte is first (Argb) or last (rgbA) in the pixel;
                // `dst` starts at it and ends at the last row's alpha byte.
                for alpha_offset in [0, 3] {
                    for opaque in [false, true] {
                        let dst_stride = 4 * w + 4 * pad;
                        let alpha_stride = w + pad;
                        let dst_len = if w == 0 {
                            0
                        } else {
                            (h - 1) * dst_stride + 4 * (w - 1) + 1
                        };
                        let alpha_len = if w == 0 {
                            0
                        } else {
                            (h - 1) * alpha_stride + w
                        };
                        let alpha = if opaque {
                            vec![0xff; alpha_len]
                        } else {
                            rng.edge_bytes(alpha_len)
                        };
                        let dst = padded(&mut rng, alpha_offset, dst_len);
                        let args = (w as i32, h as i32, alpha_stride as i32, dst_stride as i32);
                        let mut expected = dst.clone();
                        let mut actual = dst.clone();
                        let mut dispatched = dst;
                        let r_c = c::dispatch_alpha_c(
                            &alpha,
                            args.2,
                            args.0,
                            args.1,
                            &mut expected[alpha_offset..],
                            args.3,
                        );
                        let r_n = neon::dispatch_alpha(
                            &alpha,
                            args.2,
                            args.0,
                            args.1,
                            &mut actual[alpha_offset..],
                            args.3,
                        );
                        let r_d = c::webp_dispatch_alpha(
                            &alpha,
                            args.2,
                            args.0,
                            args.1,
                            &mut dispatched[alpha_offset..],
                            args.3,
                        );
                        assert_eq!(
                            (r_n, &actual),
                            (r_c, &expected),
                            "dispatch w={w} h={h} pad={pad}"
                        );
                        assert_eq!((r_d, &dispatched), (r_c, &expected));

                        // Extract the alpha bytes of the plane just written.
                        let mut alpha_c = rng.bytes(alpha_len);
                        let mut alpha_n = alpha_c.clone();
                        let mut alpha_d = alpha_c.clone();
                        let src = &expected[alpha_offset..];
                        let r_c =
                            c::extract_alpha_c(src, args.3, args.0, args.1, &mut alpha_c, args.2);
                        let r_n =
                            neon::extract_alpha(src, args.3, args.0, args.1, &mut alpha_n, args.2);
                        let r_d = c::webp_extract_alpha(
                            src,
                            args.3,
                            args.0,
                            args.1,
                            &mut alpha_d,
                            args.2,
                        );
                        assert_eq!(
                            (r_n, &alpha_n),
                            (r_c, &alpha_c),
                            "extract w={w} h={h} pad={pad}"
                        );
                        assert_eq!((r_d, &alpha_d), (r_c, &alpha_c));
                    }
                }
            }
        }
    }
}

#[test]
fn dispatch_alpha_to_green_and_extract_green_match_c() {
    let mut rng = Rng(0x2020_0003);
    for w in widths() {
        for h in [1_usize, 2, 3] {
            for pad in [0, 1, 7] {
                let dst_stride = w + pad;
                let alpha_stride = w + 2 * pad;
                let dst_len = if w == 0 { 0 } else { (h - 1) * dst_stride + w };
                let alpha_len = if w == 0 {
                    0
                } else {
                    (h - 1) * alpha_stride + w
                };
                let alpha = rng.edge_bytes(alpha_len);
                let dst = rng.words(dst_len);
                let mut expected = dst.clone();
                let mut actual = dst.clone();
                let mut dispatched = dst;
                let (wi, hi, a_s, d_s) =
                    (w as i32, h as i32, alpha_stride as i32, dst_stride as i32);
                c::dispatch_alpha_to_green_c(&alpha, a_s, wi, hi, &mut expected, d_s);
                neon::dispatch_alpha_to_green(&alpha, a_s, wi, hi, &mut actual, d_s);
                c::webp_dispatch_alpha_to_green(&alpha, a_s, wi, hi, &mut dispatched, d_s);
                assert_eq!(actual, expected, "w={w} h={h} pad={pad}");
                assert_eq!(dispatched, expected);
            }
        }
    }
    for size in widths() {
        for offset in 0..4 {
            let argb = rng.words(offset + size);
            let mut expected = rng.bytes(size);
            let mut actual = expected.clone();
            let mut dispatched = expected.clone();
            c::extract_green_c(&argb[offset..], &mut expected, size as i32);
            neon::extract_green(&argb[offset..], &mut actual, size as i32);
            c::webp_extract_green(&argb[offset..], &mut dispatched, size as i32);
            assert_eq!(actual, expected, "size={size} offset={offset}");
            assert_eq!(dispatched, expected);
        }
    }
}
