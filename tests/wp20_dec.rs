//! WP20 parity test: the NEON variants of `src/dsp/dec_neon.c` against the C
//! translations they replace (`src/dsp/dec.rs`), Rust against Rust, and the
//! dispatch functions against the C translations.
//!
//! Every buffer ends exactly at the last byte that the NEON function reads or
//! writes (the C function touches the same bytes or fewer), and starts at
//! offsets 0 to 3 of a larger allocation, so an access outside the range
//! panics. The inputs cover the range the decoder passes for streams that an
//! encoder produces: transform coefficients within ±2048 (the input range that
//! the C `TransformOne` documents; every sign pattern of the bounds), every
//! `int16_t` for the DC and Walsh-Hadamard transforms, filter thresholds 0 to
//! 254 (the decoder passes at most 193), interior limits and
//! high-edge-variance thresholds 0 to 254, flat, noisy, stepped and random
//! planes, and pixels at 0 and 255 for the saturating predictors.
//!
//! Outside those ranges upstream's NEON functions themselves differ from its C
//! functions (`TransformOne_NEON` saturates in 16 bits where `TransformOne_C`
//! computes in `int`, and the filters convert thresholds above 255 to
//! `uint8_t`); `src/dsp/dec_neon.rs` records it, and the differential tests of
//! WP6 against the `release-simd` oracle show that the translation reproduces
//! upstream's NEON bytes there.

#![cfg(all(feature = "simd", target_arch = "aarch64"))]

#[path = "support/wp20.rs"]
mod support;

use support::Rng;
use webp::dsp::dec as c;
use webp::dsp::dec_neon as neon;

const BPS: usize = 32;

/// A value in `low..=high`.
fn range(rng: &mut Rng, low: i64, high: i64) -> i64 {
    low + rng.below((high - low + 1) as u64) as i64
}

/// A decoder coefficient: mostly zero or small, sometimes at the bound.
fn coefficient(rng: &mut Rng, bound: i64) -> i16 {
    match rng.below(8) {
        0..=2 => 0,
        3..=4 => range(rng, -16, 16) as i16,
        5 => {
            if rng.below(2) == 0 {
                bound as i16
            } else {
                (-bound) as i16
            }
        }
        _ => range(rng, -bound, bound) as i16,
    }
}

/// Bytes at 0 or 255 a quarter of the time, random otherwise.
fn pixels(rng: &mut Rng, count: usize) -> Vec<u8> {
    match rng.below(4) {
        0 => {
            // Flat or nearly flat neighbors, as in smooth pictures.
            let base = rng.byte();
            (0..count)
                .map(|_| base.wrapping_add(range(rng, -2, 2) as u8))
                .collect()
        }
        _ => rng.edge_bytes(count),
    }
}

//------------------------------------------------------------------------------
// Transforms

/// The 4x4 blocks that a transform adds to: rows of `BPS` bytes, the last row
/// ending after `width` bytes, at offset `offset` of the allocation.
fn block(rng: &mut Rng, offset: usize, width: usize) -> Vec<u8> {
    pixels(rng, offset + 3 * BPS + width)
}

fn transform_case(rng: &mut Rng, bound: i64, ac3: bool) -> [i16; 32] {
    let mut coeffs = [0i16; 32];
    for (i, coeff) in coeffs.iter_mut().enumerate() {
        if !ac3 || [0, 1, 4].contains(&i) {
            *coeff = coefficient(rng, bound);
        }
    }
    coeffs
}

#[test]
fn transforms_match_c() {
    let mut rng = Rng(0x2020_dec0_0001);
    let mut cases = 0;
    for bound in [16, 255, 2048] {
        for _ in 0..3000 {
            let offset = rng.below(4) as usize;
            for do_two in [false, true] {
                let coeffs = transform_case(&mut rng, bound, false);
                let width = if do_two { 8 } else { 4 };
                let dst = block(&mut rng, offset, width);
                let (mut expected, mut actual, mut dispatched) = (dst.clone(), dst.clone(), dst);
                c::transform_two_c(&coeffs, &mut expected[offset..], do_two);
                neon::transform_two(&coeffs, &mut actual[offset..], do_two);
                c::vp8_transform(&coeffs, &mut dispatched[offset..], do_two);
                assert_eq!(actual, expected, "TransformTwo do_two={do_two} {coeffs:?}");
                assert_eq!(dispatched, expected);
                cases += 1;
            }

            let coeffs = transform_case(&mut rng, bound, true);
            let dst = block(&mut rng, offset, 4);
            let (mut expected, mut actual, mut dispatched) = (dst.clone(), dst.clone(), dst);
            c::transform_ac3_c(&coeffs, &mut expected[offset..]);
            neon::transform_ac3(&coeffs, &mut actual[offset..]);
            c::vp8_transform_ac3(&coeffs, &mut dispatched[offset..]);
            assert_eq!(actual, expected, "TransformAC3 {coeffs:?}");
            assert_eq!(dispatched, expected);

            let coeffs = transform_case(&mut rng, bound, false);
            let dst = block(&mut rng, offset, 4);
            let (mut expected, mut actual, mut dispatched) = (dst.clone(), dst.clone(), dst);
            c::transform_dc_c(&coeffs, &mut expected[offset..]);
            neon::transform_dc(&coeffs, &mut actual[offset..]);
            c::vp8_transform_dc(&coeffs, &mut dispatched[offset..]);
            assert_eq!(actual, expected, "TransformDC {}", coeffs[0]);
            assert_eq!(dispatched, expected);
            cases += 2;
        }
    }
    // Every sign pattern of the bounds -2048 and 2047 in one block, the
    // extremes of the first and second passes.
    for pattern in 0..=u16::MAX {
        let mut coeffs = [0i16; 32];
        for (i, coeff) in coeffs.iter_mut().take(16).enumerate() {
            *coeff = if pattern >> i & 1 == 0 { -2048 } else { 2047 };
        }
        let dst = block(&mut rng, 0, 4);
        let (mut expected, mut actual) = (dst.clone(), dst);
        c::transform_two_c(&coeffs, &mut expected, false);
        neon::transform_two(&coeffs, &mut actual, false);
        assert_eq!(actual, expected, "TransformTwo sign pattern {pattern:#06x}");
        let ac3 = [coeffs[0], coeffs[1], 0, 0, coeffs[4]];
        let dst = block(&mut rng, 0, 4);
        let (mut expected, mut actual) = (dst.clone(), dst);
        c::transform_ac3_c(&ac3, &mut expected);
        neon::transform_ac3(&ac3, &mut actual);
        assert_eq!(actual, expected, "TransformAC3 sign pattern {pattern:#06x}");
        cases += 2;
    }
    // The DC transform over every coefficient a decoded block can hold.
    for dc in i16::MIN..=i16::MAX {
        let mut rng_dst = Rng(u64::from(dc as u16));
        let dst = block(&mut rng_dst, 0, 4);
        let coeffs = [dc; 1];
        let (mut expected, mut actual) = (dst.clone(), dst);
        c::transform_dc_c(&coeffs, &mut expected);
        neon::transform_dc(&coeffs, &mut actual);
        assert_eq!(actual, expected, "TransformDC {dc}");
        cases += 1;
    }
    eprintln!("transforms: {cases} cases");
}

#[test]
fn transform_wht_matches_c() {
    let mut rng = Rng(0x2020_dec0_0002);
    let mut cases = 0;
    for _ in 0..20_000 {
        let input: Vec<i16> = (0..16)
            .map(|_| match rng.below(4) {
                0 => rng.next_u64() as i16,
                1 => [i16::MIN, i16::MAX, 0, -1][rng.below(4) as usize],
                _ => coefficient(&mut rng, 2048),
            })
            .collect();
        let offset = rng.below(4) as usize;
        // The output covers the DC position of each of the 16 blocks.
        let out: Vec<i16> = (0..offset + 15 * 16 + 1)
            .map(|_| rng.next_u64() as i16)
            .collect();
        let (mut expected, mut actual, mut dispatched) = (out.clone(), out.clone(), out);
        c::transform_wht_c(&input, &mut expected[offset..]);
        neon::transform_wht(&input, &mut actual[offset..]);
        c::vp8_transform_wht(&input, &mut dispatched[offset..]);
        assert_eq!(actual, expected, "TransformWHT {input:?}");
        assert_eq!(dispatched, expected);
        cases += 1;
    }
    eprintln!("wht: {cases} cases");
}

//------------------------------------------------------------------------------
// Predictors

type Pred = fn(&mut [u8], usize);

/// Runs `neon_fn` and `c_fn` (and the dispatch function for `mode`) on a block
/// at offset `offset` whose buffer starts one row and one sample before it
/// and ends at `after` bytes past it.
fn predictor_case(
    rng: &mut Rng,
    neon_fn: Pred,
    c_fn: Pred,
    dispatch: fn(i32, &mut [u8], usize),
    mode: i32,
    after: usize,
) {
    let offset = rng.below(4) as usize;
    let dst = offset + BPS + 1;
    let buf = pixels(rng, dst + after);
    let (mut expected, mut actual, mut dispatched) = (buf.clone(), buf.clone(), buf);
    c_fn(&mut expected, dst);
    neon_fn(&mut actual, dst);
    dispatch(mode, &mut dispatched, dst);
    assert_eq!(actual, expected, "mode {mode} offset {offset}");
    assert_eq!(dispatched, expected, "dispatch mode {mode}");
}

#[test]
fn predictors_match_c() {
    let mut rng = Rng(0x2020_dec0_0003);
    // The bytes past `dst` that the NEON loads reach: the left column's
    // 8-byte loads end 7 bytes into the last row.
    let after4 = 3 * BPS + 7;
    let after8 = 7 * BPS + 8;
    let after16 = 15 * BPS + 16;
    let luma4: [(i32, Pred, Pred); 5] = [
        (0, neon::dc4, c::dc4_c),
        (1, neon::tm4, c::tm4_c),
        (2, neon::ve4, c::ve4_c),
        (4, neon::rd4, c::rd4_c),
        (6, neon::ld4, c::ld4_c),
    ];
    let luma16: [(i32, Pred, Pred); 7] = [
        (0, neon::dc16_top_left, c::dc16_c),
        (1, neon::tm16, c::tm16_c),
        (2, neon::ve16, c::ve16_c),
        (3, neon::he16, c::he16_c),
        (4, neon::dc16_no_top, c::dc16_no_top_c),
        (5, neon::dc16_no_left, c::dc16_no_left_c),
        (6, neon::dc16_no_top_left, c::dc16_no_top_left_c),
    ];
    let chroma8: [(i32, Pred, Pred); 7] = [
        (0, neon::dc8uv, c::dc8uv_c),
        (1, neon::tm8uv, c::tm8uv_c),
        (2, neon::ve8uv, c::ve8uv_c),
        (3, neon::he8uv, c::he8uv_c),
        (4, neon::dc8uv_no_top, c::dc8uv_no_top_c),
        (5, neon::dc8uv_no_left, c::dc8uv_no_left_c),
        (6, neon::dc8uv_no_top_left, c::dc8uv_no_top_left_c),
    ];
    let mut cases = 0;
    for _ in 0..2000 {
        for &(mode, n, f) in &luma4 {
            predictor_case(&mut rng, n, f, c::vp8_pred_luma4, mode, after4);
            cases += 1;
        }
        for &(mode, n, f) in &luma16 {
            predictor_case(&mut rng, n, f, c::vp8_pred_luma16, mode, after16);
            cases += 1;
        }
        for &(mode, n, f) in &chroma8 {
            predictor_case(&mut rng, n, f, c::vp8_pred_chroma8, mode, after8);
            cases += 1;
        }
    }
    eprintln!("predictors: {cases} cases");
}

//------------------------------------------------------------------------------
// Loop filters

/// A plane with smooth areas, edges and noise, so that the filters take every
/// branch, as in `tests/wp6_dsp.rs`.
fn filter_plane(rng: &mut Rng, size: usize, stride: usize) -> Vec<u8> {
    match rng.below(5) {
        0 => rng.edge_bytes(size),
        1 => {
            let base = range(rng, 0, 255);
            let spread = range(rng, 0, 8);
            (0..size)
                .map(|_| (base + range(rng, -spread, spread)).clamp(0, 255) as u8)
                .collect()
        }
        2 => {
            // Vertical steps between areas, with noise.
            let a = range(rng, 0, 255);
            let b = range(rng, 0, 255);
            let step = range(rng, 1, stride as i64) as usize;
            (0..size)
                .map(|i| {
                    let base = if (i % stride) < step { a } else { b };
                    (base + range(rng, -3, 3)).clamp(0, 255) as u8
                })
                .collect()
        }
        3 => {
            // Horizontal steps between areas, with noise.
            let a = range(rng, 0, 255);
            let b = range(rng, 0, 255);
            let row = range(rng, 1, 24) as usize;
            (0..size)
                .map(|i| {
                    let base = if i / stride < row { a } else { b };
                    (base + range(rng, -2, 2)).clamp(0, 255) as u8
                })
                .collect()
        }
        _ => {
            // Large steps that saturate the filter arithmetic.
            let spread = range(rng, 0, 40);
            (0..size)
                .map(|i| {
                    let base = if (i % stride + i / stride).is_multiple_of(2) {
                        0
                    } else {
                        255
                    };
                    (base + range(rng, -spread, spread)).clamp(0, 255) as u8
                })
                .collect()
        }
    }
}

/// A threshold in `0..=254`, half of the time in the decoder's range.
fn threshold(rng: &mut Rng, decoder_max: i64) -> i32 {
    if rng.below(2) == 0 {
        range(rng, 0, decoder_max) as i32
    } else {
        range(rng, 0, 254) as i32
    }
}

/// The bytes before and from `pos` that a filter touches with `stride`.
#[derive(Clone, Copy)]
enum Extent {
    /// Rows `-before_rows..after_rows` of `width` bytes from `pos`.
    Rows {
        before: usize,
        after: usize,
        width: usize,
    },
    /// Columns `-before..after` of `rows` rows.
    Cols {
        before: usize,
        after: usize,
        rows: usize,
    },
}

impl Extent {
    fn before(self, stride: usize) -> usize {
        match self {
            Extent::Rows { before, .. } => before * stride,
            Extent::Cols { before, .. } => before,
        }
    }

    fn after(self, stride: usize) -> usize {
        match self {
            Extent::Rows { after, width, .. } => (after - 1) * stride + width,
            Extent::Cols { after, rows, .. } => (rows - 1) * stride + after,
        }
    }
}

type Simple = fn(&mut [u8], usize, i32, i32);
type Luma = fn(&mut [u8], usize, i32, i32, i32, i32);
type Chroma = fn(&mut [u8], usize, &mut [u8], usize, i32, i32, i32, i32);

/// A plane of `stride` with `pos` placed so that the extent fits exactly.
fn filter_buffer(rng: &mut Rng, extent: Extent, stride: usize) -> (Vec<u8>, usize) {
    let offset = rng.below(4) as usize;
    let pos = offset + extent.before(stride);
    let buf = filter_plane(rng, pos + extent.after(stride), stride);
    (buf, pos)
}

#[test]
fn simple_filters_match_c() {
    let mut rng = Rng(0x2020_dec0_0004);
    let cases: [(&str, Simple, Simple, Simple, Extent); 4] = [
        (
            "SimpleVFilter16",
            neon::simple_v_filter16,
            c::simple_v_filter16_c,
            c::vp8_simple_v_filter16,
            Extent::Rows {
                before: 2,
                after: 2,
                width: 16,
            },
        ),
        (
            "SimpleHFilter16",
            neon::simple_h_filter16,
            c::simple_h_filter16_c,
            c::vp8_simple_h_filter16,
            Extent::Cols {
                before: 2,
                after: 2,
                rows: 16,
            },
        ),
        (
            "SimpleVFilter16i",
            neon::simple_v_filter16i,
            c::simple_v_filter16i_c,
            c::vp8_simple_v_filter16i,
            // Rows 2 to 13 from `pos`; the plane starts at `pos`.
            Extent::Rows {
                before: 0,
                after: 14,
                width: 16,
            },
        ),
        (
            "SimpleHFilter16i",
            neon::simple_h_filter16i,
            c::simple_h_filter16i_c,
            c::vp8_simple_h_filter16i,
            Extent::Cols {
                before: 0,
                after: 14,
                rows: 16,
            },
        ),
    ];
    let mut count = 0;
    for (name, n, f, d, extent) in cases {
        for _ in 0..4000 {
            let stride = range(&mut rng, 16, 48) as usize;
            let (buf, pos) = filter_buffer(&mut rng, extent, stride);
            let thresh = threshold(&mut rng, 193);
            let s = stride as i32;
            let (mut expected, mut actual, mut dispatched) = (buf.clone(), buf.clone(), buf);
            f(&mut expected, pos, s, thresh);
            n(&mut actual, pos, s, thresh);
            d(&mut dispatched, pos, s, thresh);
            assert_eq!(actual, expected, "{name} stride={s} thresh={thresh}");
            assert_eq!(dispatched, expected, "{name} dispatch");
            count += 1;
        }
    }
    eprintln!("simple filters: {count} cases");
}

#[test]
fn luma_filters_match_c() {
    let mut rng = Rng(0x2020_dec0_0005);
    let cases: [(&str, Luma, Luma, Luma, Extent); 4] = [
        (
            "VFilter16",
            neon::v_filter16,
            c::v_filter16_c,
            c::vp8v_filter16,
            Extent::Rows {
                before: 4,
                after: 4,
                width: 16,
            },
        ),
        (
            "HFilter16",
            neon::h_filter16,
            c::h_filter16_c,
            c::vp8h_filter16,
            Extent::Cols {
                before: 4,
                after: 4,
                rows: 16,
            },
        ),
        (
            "VFilter16i",
            neon::v_filter16i,
            c::v_filter16i_c,
            c::vp8v_filter16i,
            Extent::Rows {
                before: 0,
                after: 16,
                width: 16,
            },
        ),
        (
            "HFilter16i",
            neon::h_filter16i,
            c::h_filter16i_c,
            c::vp8h_filter16i,
            Extent::Cols {
                before: 0,
                after: 16,
                rows: 16,
            },
        ),
    ];
    let mut count = 0;
    for (name, n, f, d, extent) in cases {
        for _ in 0..4000 {
            let stride = range(&mut rng, 16, 48) as usize;
            let (buf, pos) = filter_buffer(&mut rng, extent, stride);
            let thresh = threshold(&mut rng, 193);
            let ithresh = threshold(&mut rng, 63);
            let hev = threshold(&mut rng, 2);
            let s = stride as i32;
            let (mut expected, mut actual, mut dispatched) = (buf.clone(), buf.clone(), buf);
            f(&mut expected, pos, s, thresh, ithresh, hev);
            n(&mut actual, pos, s, thresh, ithresh, hev);
            d(&mut dispatched, pos, s, thresh, ithresh, hev);
            assert_eq!(
                actual, expected,
                "{name} stride={s} thresh={thresh} ithresh={ithresh} hev={hev}"
            );
            assert_eq!(dispatched, expected, "{name} dispatch");
            count += 1;
        }
    }
    eprintln!("luma filters: {count} cases");
}

#[test]
fn chroma_filters_match_c() {
    let mut rng = Rng(0x2020_dec0_0006);
    let cases: [(&str, Chroma, Chroma, Chroma, Extent); 4] = [
        (
            "VFilter8",
            neon::v_filter8,
            c::v_filter8_c,
            c::vp8v_filter8,
            Extent::Rows {
                before: 4,
                after: 4,
                width: 8,
            },
        ),
        (
            "HFilter8",
            neon::h_filter8,
            c::h_filter8_c,
            c::vp8h_filter8,
            Extent::Cols {
                before: 4,
                after: 4,
                rows: 8,
            },
        ),
        (
            "VFilter8i",
            neon::v_filter8i,
            c::v_filter8i_c,
            c::vp8v_filter8i,
            Extent::Rows {
                before: 0,
                after: 8,
                width: 8,
            },
        ),
        (
            "HFilter8i",
            neon::h_filter8i,
            c::h_filter8i_c,
            c::vp8h_filter8i,
            Extent::Cols {
                before: 0,
                after: 8,
                rows: 8,
            },
        ),
    ];
    let mut count = 0;
    for (name, n, f, d, extent) in cases {
        for _ in 0..4000 {
            let stride = range(&mut rng, 8, 40) as usize;
            let (u, u_pos) = filter_buffer(&mut rng, extent, stride);
            let (v, v_pos) = filter_buffer(&mut rng, extent, stride);
            let thresh = threshold(&mut rng, 193);
            let ithresh = threshold(&mut rng, 63);
            let hev = threshold(&mut rng, 2);
            let s = stride as i32;
            let (mut ue, mut ua, mut ud) = (u.clone(), u.clone(), u);
            let (mut ve, mut va, mut vd) = (v.clone(), v.clone(), v);
            f(&mut ue, u_pos, &mut ve, v_pos, s, thresh, ithresh, hev);
            n(&mut ua, u_pos, &mut va, v_pos, s, thresh, ithresh, hev);
            d(&mut ud, u_pos, &mut vd, v_pos, s, thresh, ithresh, hev);
            assert_eq!(
                (&ua, &va),
                (&ue, &ve),
                "{name} stride={s} thresh={thresh} ithresh={ithresh} hev={hev}"
            );
            assert_eq!((&ud, &vd), (&ue, &ve), "{name} dispatch");
            count += 1;
        }
    }
    eprintln!("chroma filters: {count} cases");
}

/// Every threshold from 0 to 254 on one plane per filter family, so that each
/// comparison boundary of the NEON masks is reached.
#[test]
fn every_threshold_matches_c() {
    let mut rng = Rng(0x2020_dec0_0007);
    let mut count = 0;
    for thresh in 0..=254 {
        let stride = 24;
        let extent = Extent::Cols {
            before: 4,
            after: 4,
            rows: 16,
        };
        let (buf, pos) = filter_buffer(&mut rng, extent, stride);
        let (mut expected, mut actual) = (buf.clone(), buf);
        c::simple_h_filter16_c(&mut expected, pos + 2, stride as i32, thresh);
        neon::simple_h_filter16(&mut actual, pos + 2, stride as i32, thresh);
        assert_eq!(actual, expected, "SimpleHFilter16 thresh={thresh}");
        for ithresh in [0, 1, 9, 63, 128, 254] {
            for hev in [0, 1, 2, 3, 128, 254] {
                let (buf, pos) = filter_buffer(&mut rng, extent, stride);
                let (mut expected, mut actual) = (buf.clone(), buf);
                c::h_filter16_c(&mut expected, pos, stride as i32, thresh, ithresh, hev);
                neon::h_filter16(&mut actual, pos, stride as i32, thresh, ithresh, hev);
                assert_eq!(
                    actual, expected,
                    "HFilter16 thresh={thresh} ithresh={ithresh} hev={hev}"
                );
                count += 1;
            }
        }
    }
    eprintln!("thresholds: {count} cases");
}
