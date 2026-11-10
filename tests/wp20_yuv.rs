//! WP20 parity test: the NEON variants of `src/dsp/yuv_neon.c` against the C
//! translations they replace (`src/dsp/yuv.rs`), Rust against Rust, and the
//! dispatch functions against the C translations. Buffers end exactly at the
//! last element the C function touches and start at offsets 0 to 3 of a
//! larger allocation.
//!
//! `WebPConvertRGBA32ToUV` receives sums of four gamma-mapped 8-bit samples
//! (at most 1020, `picture_csp_enc.c`). `ConvertRGBA32ToUV_NEON` reads its
//! lanes as signed 16-bit values, so it equals the C function for components
//! below 32768 only; the test covers that whole range, and the module
//! documentation of `src/dsp/yuv_neon.rs` records the rest as upstream's.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::{Rng, widths};
use webp::dsp::yuv as c;
use webp::dsp::yuv_neon as neon;

#[test]
fn rgb_to_y_covers_every_red_green_pair() {
    // Rows of 256 pixels: red is the row, green the column, blue fixed.
    for (n, blue) in [0_u8, 1, 127, 128, 254, 255, 42].into_iter().enumerate() {
        let mut rgb = vec![0_u8; 256 * 256 * 3];
        for (i, pixel) in rgb.as_chunks_mut::<3>().0.iter_mut().enumerate() {
            *pixel = [(i >> 8) as u8, i as u8, blue];
        }
        let width = 256 * 256;
        let mut expected = vec![0_u8; width];
        let mut actual = vec![1_u8; width];
        c::convert_rgb24_to_y_c(&rgb, &mut expected, width as i32);
        neon::convert_rgb24_to_y(&rgb, &mut actual, width as i32);
        assert_eq!(actual, expected, "rgb blue={blue}");
        let mut actual = vec![2_u8; width];
        let mut expected = vec![3_u8; width];
        c::convert_bgr24_to_y_c(&rgb, &mut expected, width as i32);
        neon::convert_bgr24_to_y(&rgb, &mut actual, width as i32);
        assert_eq!(actual, expected, "bgr red={blue}");
        let argb: Vec<u32> = rgb
            .as_chunks::<3>()
            .0
            .iter()
            .map(|p| (u32::from(p[0]) << 16) | (u32::from(p[1]) << 8) | u32::from(p[2]))
            .map(|p| p | ((n as u32 * 37) << 24))
            .collect();
        let mut actual = vec![4_u8; width];
        let mut expected = vec![5_u8; width];
        c::convert_argb_to_y_c(&argb, &mut expected, width as i32);
        neon::convert_argb_to_y(&argb, &mut actual, width as i32);
        assert_eq!(actual, expected, "argb blue={blue}");
    }
}

#[test]
fn rgb_to_y_matches_c() {
    let mut rng = Rng(0x2020_0101);
    for width in widths() {
        for offset in 0..4 {
            let rgb = rng.edge_bytes(offset + 3 * width);
            let argb = rng.words(offset + width);
            let w = width as i32;
            for bgr in [false, true] {
                let mut expected = rng.bytes(width);
                let mut actual = expected.clone();
                let mut dispatched = expected.clone();
                if bgr {
                    c::convert_bgr24_to_y_c(&rgb[offset..], &mut expected, w);
                    neon::convert_bgr24_to_y(&rgb[offset..], &mut actual, w);
                    c::webp_convert_bgr24_to_y(&rgb[offset..], &mut dispatched, w);
                } else {
                    c::convert_rgb24_to_y_c(&rgb[offset..], &mut expected, w);
                    neon::convert_rgb24_to_y(&rgb[offset..], &mut actual, w);
                    c::webp_convert_rgb24_to_y(&rgb[offset..], &mut dispatched, w);
                }
                assert_eq!(actual, expected, "width={width} offset={offset} bgr={bgr}");
                assert_eq!(dispatched, expected);
            }
            let mut expected = rng.bytes(width);
            let mut actual = expected.clone();
            let mut dispatched = expected.clone();
            c::convert_argb_to_y_c(&argb[offset..], &mut expected, w);
            neon::convert_argb_to_y(&argb[offset..], &mut actual, w);
            c::webp_convert_argb_to_y(&argb[offset..], &mut dispatched, w);
            assert_eq!(actual, expected, "argb width={width} offset={offset}");
            assert_eq!(dispatched, expected);
        }
    }
}

#[test]
fn argb_to_uv_matches_c() {
    let mut rng = Rng(0x2020_0102);
    for src_width in widths() {
        for do_store in [0, 1] {
            for offset in 0..4 {
                let uv_width = src_width.div_ceil(2);
                // Edge channels: whole pixels of 0x00 or 0xff bytes, or random.
                let argb: Vec<u32> = (0..offset + src_width)
                    .map(|_| match rng.below(4) {
                        0 => 0,
                        1 => u32::MAX,
                        _ => rng.next_u64() as u32,
                    })
                    .collect();
                let u0 = rng.edge_bytes(uv_width);
                let v0 = rng.edge_bytes(uv_width);
                let (mut u_c, mut v_c) = (u0.clone(), v0.clone());
                let (mut u_n, mut v_n) = (u0.clone(), v0.clone());
                let (mut u_d, mut v_d) = (u0, v0);
                let w = src_width as i32;
                c::webp_convert_argb_to_uv_c(&argb[offset..], &mut u_c, &mut v_c, w, do_store);
                neon::convert_argb_to_uv(&argb[offset..], &mut u_n, &mut v_n, w, do_store);
                c::webp_convert_argb_to_uv(&argb[offset..], &mut u_d, &mut v_d, w, do_store);
                assert_eq!(
                    (&u_n, &v_n),
                    (&u_c, &v_c),
                    "width={src_width} store={do_store}"
                );
                assert_eq!((&u_d, &v_d), (&u_c, &v_c));
            }
        }
    }
}

#[test]
fn rgba32_to_uv_matches_c_below_32768() {
    let mut rng = Rng(0x2020_0103);
    const EDGES: [u16; 8] = [0, 1, 1020, 1023, 4095, 16384, 32766, 32767];
    for width in widths() {
        for offset in 0..4 {
            for range in [1021_u64, 32768] {
                let rgb: Vec<u16> = (0..4 * (offset + width))
                    .map(|_| {
                        if rng.below(4) == 0 {
                            EDGES[rng.below(EDGES.len() as u64) as usize]
                        } else {
                            rng.below(range) as u16
                        }
                    })
                    .collect();
                let rgb = &rgb[4 * offset..];
                let mut u_c = rng.bytes(width);
                let mut v_c = rng.bytes(width);
                let (mut u_n, mut v_n) = (u_c.clone(), v_c.clone());
                let (mut u_d, mut v_d) = (u_c.clone(), v_c.clone());
                let w = width as i32;
                c::webp_convert_rgba32_to_uv_c(rgb, &mut u_c, &mut v_c, w);
                neon::convert_rgba32_to_uv(rgb, &mut u_n, &mut v_n, w);
                c::webp_convert_rgba32_to_uv(rgb, &mut u_d, &mut v_d, w);
                assert_eq!((&u_n, &v_n), (&u_c, &v_c), "width={width} range={range}");
                assert_eq!((&u_d, &v_d), (&u_c, &v_c));
            }
        }
    }
}
