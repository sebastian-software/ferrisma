//! Kernel bodies, written once against [`Lanes`]. Safe Rust: slices are
//! bounds-checked, and the `Lanes` methods take fixed-size arrays.

use crate::Taps;
use crate::lanes::{Lanes, LanesF64, Scalar};

#[inline(always)]
fn horizontal_single<L: Lanes>(run: &[f32], weights: &[f32]) -> [f32; 4] {
    if L::SLOTS == 1 && L::HORIZONTAL_PIXELS > 1 {
        let mut acc = L::zero();
        let quads = weights.len() / 4;
        let q_taps = quads * 4;
        for (v, w) in run[..q_taps * 4]
            .as_chunks::<16>()
            .0
            .iter()
            .zip(weights[..q_taps].as_chunks::<4>().0.iter())
        {
            let wv = L::load(w);
            acc = L::fma_lane::<0>(acc, L::load(&v[0..4]), wv);
            acc = L::fma_lane::<1>(acc, L::load(&v[4..8]), wv);
            acc = L::fma_lane::<2>(acc, L::load(&v[8..12]), wv);
            acc = L::fma_lane::<3>(acc, L::load(&v[12..16]), wv);
        }
        for (v, &w) in run[q_taps * 4..]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(&weights[q_taps..])
        {
            acc = L::fma(acc, L::load(v), L::splat(w));
        }
        return L::reduce_slots(acc);
    }
    let full = weights.len() / L::SLOTS * L::SLOTS;
    let mut sum = if full > 0 {
        let mut acc = L::zero();
        for (values, tap_weights) in run[..full * 4]
            .chunks_exact(L::SLOTS * 4)
            .zip(weights[..full].chunks_exact(L::SLOTS))
        {
            acc = L::fma(acc, L::load(values), L::slot_weights(tap_weights));
        }
        L::reduce_slots(acc)
    } else {
        [0.0; 4]
    };
    for (values, &weight) in run[full * 4..]
        .as_chunks::<4>()
        .0
        .iter()
        .zip(&weights[full..])
    {
        for (channel, value) in sum.iter_mut().enumerate() {
            *value = values[channel].mul_add(weight, *value);
        }
    }
    sum
}

/// Pipeline four independent output pixels on `SLOTS == 1` (`Neon`) while
/// accumulating every pixel's taps in exact sequential order `0, 1, ..., n-1`
/// (bit-identical to `Scalar`, 0 ULP).
#[inline(always)]
#[allow(clippy::too_many_lines)] // Four pipelined pixel lanes kept flat for register allocation.
fn horizontal_lane_4px<L: Lanes>(
    source: &[f32],
    taps: &Taps,
    base: usize,
    pixels: &mut [[f32; 4]],
) {
    let starts = &taps.starts[base..base + 4];
    let offsets = &taps.offsets[base..base + 5];
    let (s0, s1, s2, s3) = (
        starts[0] as usize * 4,
        starts[1] as usize * 4,
        starts[2] as usize * 4,
        starts[3] as usize * 4,
    );
    let (o0, o1, o2, o3, o4) = (
        offsets[0] as usize,
        offsets[1] as usize,
        offsets[2] as usize,
        offsets[3] as usize,
        offsets[4] as usize,
    );
    let w0 = &taps.weights[o0..o1];
    let w1 = &taps.weights[o1..o2];
    let w2 = &taps.weights[o2..o3];
    let w3 = &taps.weights[o3..o4];
    let r0 = &source[s0..s0 + w0.len() * 4];
    let r1 = &source[s1..s1 + w1.len() * 4];
    let r2 = &source[s2..s2 + w2.len() * 4];
    let r3 = &source[s3..s3 + w3.len() * 4];

    let common_len = w0.len().min(w1.len()).min(w2.len()).min(w3.len());
    if common_len < 4 {
        pixels[0] = horizontal_single::<L>(r0, w0);
        pixels[1] = horizontal_single::<L>(r1, w1);
        pixels[2] = horizontal_single::<L>(r2, w2);
        pixels[3] = horizontal_single::<L>(r3, w3);
        return;
    }
    let common_taps = common_len / 4 * 4;
    let common_vals = common_taps * 4;

    let mut acc0 = L::zero();
    let mut acc1 = L::zero();
    let mut acc2 = L::zero();
    let mut acc3 = L::zero();

    for (((v0, sw0), (v1, sw1)), ((v2, sw2), (v3, sw3))) in r0[..common_vals]
        .as_chunks::<16>()
        .0
        .iter()
        .zip(w0[..common_taps].as_chunks::<4>().0.iter())
        .zip(
            r1[..common_vals]
                .as_chunks::<16>()
                .0
                .iter()
                .zip(w1[..common_taps].as_chunks::<4>().0.iter()),
        )
        .zip(
            r2[..common_vals]
                .as_chunks::<16>()
                .0
                .iter()
                .zip(w2[..common_taps].as_chunks::<4>().0.iter())
                .zip(
                    r3[..common_vals]
                        .as_chunks::<16>()
                        .0
                        .iter()
                        .zip(w3[..common_taps].as_chunks::<4>().0.iter()),
                ),
        )
    {
        let wv0 = L::load(sw0);
        let wv1 = L::load(sw1);
        let wv2 = L::load(sw2);
        let wv3 = L::load(sw3);

        acc0 = L::fma_lane::<0>(acc0, L::load(&v0[0..4]), wv0);
        acc1 = L::fma_lane::<0>(acc1, L::load(&v1[0..4]), wv1);
        acc2 = L::fma_lane::<0>(acc2, L::load(&v2[0..4]), wv2);
        acc3 = L::fma_lane::<0>(acc3, L::load(&v3[0..4]), wv3);

        acc0 = L::fma_lane::<1>(acc0, L::load(&v0[4..8]), wv0);
        acc1 = L::fma_lane::<1>(acc1, L::load(&v1[4..8]), wv1);
        acc2 = L::fma_lane::<1>(acc2, L::load(&v2[4..8]), wv2);
        acc3 = L::fma_lane::<1>(acc3, L::load(&v3[4..8]), wv3);

        acc0 = L::fma_lane::<2>(acc0, L::load(&v0[8..12]), wv0);
        acc1 = L::fma_lane::<2>(acc1, L::load(&v1[8..12]), wv1);
        acc2 = L::fma_lane::<2>(acc2, L::load(&v2[8..12]), wv2);
        acc3 = L::fma_lane::<2>(acc3, L::load(&v3[8..12]), wv3);

        acc0 = L::fma_lane::<3>(acc0, L::load(&v0[12..16]), wv0);
        acc1 = L::fma_lane::<3>(acc1, L::load(&v1[12..16]), wv1);
        acc2 = L::fma_lane::<3>(acc2, L::load(&v2[12..16]), wv2);
        acc3 = L::fma_lane::<3>(acc3, L::load(&v3[12..16]), wv3);
    }

    for (((v0, &sw0), (v1, &sw1)), ((v2, &sw2), (v3, &sw3))) in r0[common_vals..common_len * 4]
        .as_chunks::<4>()
        .0
        .iter()
        .zip(&w0[common_taps..common_len])
        .zip(
            r1[common_vals..common_len * 4]
                .as_chunks::<4>()
                .0
                .iter()
                .zip(&w1[common_taps..common_len]),
        )
        .zip(
            r2[common_vals..common_len * 4]
                .as_chunks::<4>()
                .0
                .iter()
                .zip(&w2[common_taps..common_len])
                .zip(
                    r3[common_vals..common_len * 4]
                        .as_chunks::<4>()
                        .0
                        .iter()
                        .zip(&w3[common_taps..common_len]),
                ),
        )
    {
        acc0 = L::fma(acc0, L::load(v0), L::splat(sw0));
        acc1 = L::fma(acc1, L::load(v1), L::splat(sw1));
        acc2 = L::fma(acc2, L::load(v2), L::splat(sw2));
        acc3 = L::fma(acc3, L::load(v3), L::splat(sw3));
    }

    if w0.len() != common_len
        || w1.len() != common_len
        || w2.len() != common_len
        || w3.len() != common_len
    {
        for (v, &w) in r0[common_len * 4..]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(&w0[common_len..])
        {
            acc0 = L::fma(acc0, L::load(v), L::splat(w));
        }
        for (v, &w) in r1[common_len * 4..]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(&w1[common_len..])
        {
            acc1 = L::fma(acc1, L::load(v), L::splat(w));
        }
        for (v, &w) in r2[common_len * 4..]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(&w2[common_len..])
        {
            acc2 = L::fma(acc2, L::load(v), L::splat(w));
        }
        for (v, &w) in r3[common_len * 4..]
            .as_chunks::<4>()
            .0
            .iter()
            .zip(&w3[common_len..])
        {
            acc3 = L::fma(acc3, L::load(v), L::splat(w));
        }
    }

    L::store(&mut pixels[0], acc0);
    L::store(&mut pixels[1], acc1);
    L::store(&mut pixels[2], acc2);
    L::store(&mut pixels[3], acc3);
}

/// Pipeline two independent output pixels on `SLOTS >= 2` (`Avx2`, `Avx512`)
/// while preserving each pixel's exact `SLOTS`-strided accumulation order and
/// `reduce_slots` tail (bit-identical to single-pixel evaluation at the same
/// ISA level).
#[inline(always)]
fn horizontal_2px<L: Lanes>(source: &[f32], taps: &Taps, base: usize, pixels: &mut [[f32; 4]]) {
    let starts = &taps.starts[base..base + 2];
    let offsets = &taps.offsets[base..base + 3];
    let (s0, s1) = (starts[0] as usize * 4, starts[1] as usize * 4);
    let (o0, o1, o2) = (
        offsets[0] as usize,
        offsets[1] as usize,
        offsets[2] as usize,
    );
    let w0 = &taps.weights[o0..o1];
    let w1 = &taps.weights[o1..o2];
    let r0 = &source[s0..s0 + w0.len() * 4];
    let r1 = &source[s1..s1 + w1.len() * 4];

    let full0 = w0.len() / L::SLOTS * L::SLOTS;
    let full1 = w1.len() / L::SLOTS * L::SLOTS;
    let common = full0.min(full1);
    if common == 0 {
        pixels[0] = horizontal_single::<L>(r0, w0);
        pixels[1] = horizontal_single::<L>(r1, w1);
        return;
    }
    let step = L::SLOTS * 4;

    let mut acc0 = L::zero();
    let mut acc1 = L::zero();
    for ((v0, tw0), (v1, tw1)) in r0[..common * 4]
        .chunks_exact(step)
        .zip(w0[..common].chunks_exact(L::SLOTS))
        .zip(
            r1[..common * 4]
                .chunks_exact(step)
                .zip(w1[..common].chunks_exact(L::SLOTS)),
        )
    {
        acc0 = L::fma(acc0, L::load(v0), L::slot_weights(tw0));
        acc1 = L::fma(acc1, L::load(v1), L::slot_weights(tw1));
    }
    if full0 != common {
        for (v0, tw0) in r0[common * 4..full0 * 4]
            .chunks_exact(step)
            .zip(w0[common..full0].chunks_exact(L::SLOTS))
        {
            acc0 = L::fma(acc0, L::load(v0), L::slot_weights(tw0));
        }
    }
    if full1 != common {
        for (v1, tw1) in r1[common * 4..full1 * 4]
            .chunks_exact(step)
            .zip(w1[common..full1].chunks_exact(L::SLOTS))
        {
            acc1 = L::fma(acc1, L::load(v1), L::slot_weights(tw1));
        }
    }

    let mut sum0 = L::reduce_slots(acc0);
    for (values, &weight) in r0[full0 * 4..].as_chunks::<4>().0.iter().zip(&w0[full0..]) {
        for (channel, value) in sum0.iter_mut().enumerate() {
            *value = values[channel].mul_add(weight, *value);
        }
    }
    let mut sum1 = L::reduce_slots(acc1);
    for (values, &weight) in r1[full1 * 4..].as_chunks::<4>().0.iter().zip(&w1[full1..]) {
        for (channel, value) in sum1.iter_mut().enumerate() {
            *value = values[channel].mul_add(weight, *value);
        }
    }
    pixels[0] = sum0;
    pixels[1] = sum1;
}

/// See `Kernels::horizontal_rgba`. The caller has validated the lengths.
///
/// Pipelines `L::HORIZONTAL_PIXELS` independent output pixels (`4` on NEON
/// using `vld1q_f32` + `vfmaq_laneq_f32` in exact sequential tap order, `2` on
/// AVX2 and AVX-512 preserving each pixel's exact `SLOTS`-strided accumulation
/// order and `reduce_slots` tail), staying bit-identical to single-pixel
/// evaluation at every ISA level (and bit-identical to `Scalar` on `Neon`).
#[inline(always)]
pub(crate) fn horizontal_rgba<L: Lanes>(source: &[f32], taps: &Taps, out: &mut [f32]) {
    let pixels = out.as_chunks_mut::<4>().0;
    if L::SLOTS == 1 && L::HORIZONTAL_PIXELS > 1 && taps.max_run < 4 {
        for ((&start_px, bounds), pixel) in taps
            .starts
            .iter()
            .zip(taps.offsets.windows(2))
            .zip(pixels.iter_mut())
        {
            let start = start_px as usize * 4;
            let weights = &taps.weights[bounds[0] as usize..bounds[1] as usize];
            let run = &source[start..start + weights.len() * 4];
            let acc = match weights.len() {
                0 => L::zero(),
                1 => L::fma(L::zero(), L::load(&run[..4]), L::splat(weights[0])),
                2 => {
                    let a = L::fma(L::zero(), L::load(&run[..4]), L::splat(weights[0]));
                    L::fma(a, L::load(&run[4..8]), L::splat(weights[1]))
                }
                3 => {
                    let a = L::fma(L::zero(), L::load(&run[..4]), L::splat(weights[0]));
                    let a = L::fma(a, L::load(&run[4..8]), L::splat(weights[1]));
                    L::fma(a, L::load(&run[8..12]), L::splat(weights[2]))
                }
                _ => unreachable!(),
            };
            L::store(pixel, acc);
        }
        return;
    }
    let mut pos = 0;
    if L::SLOTS == 1 && L::HORIZONTAL_PIXELS == 4 {
        while pos + 4 <= pixels.len() {
            horizontal_lane_4px::<L>(source, taps, pos, &mut pixels[pos..pos + 4]);
            pos += 4;
        }
    } else if L::SLOTS >= 2
        && L::HORIZONTAL_PIXELS == 2
        && usize::try_from(taps.max_run).unwrap_or(usize::MAX) >= L::SLOTS
    {
        while pos + 2 <= pixels.len() {
            horizontal_2px::<L>(source, taps, pos, &mut pixels[pos..pos + 2]);
            pos += 2;
        }
    }
    for (offset, pixel) in pixels[pos..].iter_mut().enumerate() {
        let position = pos + offset;
        let start = taps.starts[position] as usize * 4;
        let weights =
            &taps.weights[taps.offsets[position] as usize..taps.offsets[position + 1] as usize];
        let run = &source[start..start + weights.len() * 4];
        *pixel = horizontal_single::<L>(run, weights);
    }
}

/// See `Kernels::axpy`. The caller has validated the lengths.
#[inline(always)]
pub(crate) fn axpy<L: Lanes>(acc: &mut [f32], row: &[f32], weight: f32) {
    let width = L::SLOTS * 4;
    let full = acc.len() / width * width;
    let splat = L::splat(weight);
    let (acc_body, acc_tail) = acc.split_at_mut(full);
    let (row_body, row_tail) = row.split_at(full);
    for (a, r) in acc_body
        .chunks_exact_mut(width)
        .zip(row_body.chunks_exact(width))
    {
        L::store(a, L::fma(L::load(a), L::load(r), splat));
    }
    for (a, &r) in acc_tail.iter_mut().zip(row_tail) {
        *a = r.mul_add(weight, *a);
    }
}

/// Column strip width in `f32` elements (`8 KiB`) used when `rows.len() > 4`.
/// Divisible by `8 * 4` (NEON), `4 * 8` (AVX2), and `4 * 16` (AVX-512),
/// bounding concurrent streams per pass to 5 (4 input strips + 1 output strip,
/// `40 KiB` working set across the 5 strips) to reduce intermediate output
/// read-modify-write traffic by up to 4× and improve locality on wide rows and
/// power-of-two contiguous ring strides (e.g. 8192-pixel = 128 KiB rows in
/// `fast_execute::run_sequential`).
const VERTICAL_COL_TILE: usize = 2048;

#[inline(always)]
#[allow(clippy::too_many_lines)] // Register-blocked 8/4/2/1 unroll tiers kept flat for LLVM.
fn vertical_strip<L: Lanes, const FIRST: bool>(
    rows: &[&[f32]],
    weights: &[f32],
    out: &mut [f32],
    col_start: usize,
    col_end: usize,
) {
    let w = L::SLOTS * 4;
    let mut col = col_start;

    if L::VERTICAL_UNROLL == 8 {
        let block = w * 8;
        while col + block <= col_end {
            let (mut a0, mut a1, mut a2, mut a3, mut a4, mut a5, mut a6, mut a7) = if FIRST {
                (
                    L::zero(),
                    L::zero(),
                    L::zero(),
                    L::zero(),
                    L::zero(),
                    L::zero(),
                    L::zero(),
                    L::zero(),
                )
            } else {
                let o = &out[col..col + block];
                (
                    L::load(&o[0..w]),
                    L::load(&o[w..w * 2]),
                    L::load(&o[w * 2..w * 3]),
                    L::load(&o[w * 3..w * 4]),
                    L::load(&o[w * 4..w * 5]),
                    L::load(&o[w * 5..w * 6]),
                    L::load(&o[w * 6..w * 7]),
                    L::load(&o[w * 7..w * 8]),
                )
            };
            for (row, &weight) in rows.iter().zip(weights) {
                let wv = L::splat(weight);
                let r = &row[col..col + block];
                a0 = L::fma(a0, L::load(&r[0..w]), wv);
                a1 = L::fma(a1, L::load(&r[w..w * 2]), wv);
                a2 = L::fma(a2, L::load(&r[w * 2..w * 3]), wv);
                a3 = L::fma(a3, L::load(&r[w * 3..w * 4]), wv);
                a4 = L::fma(a4, L::load(&r[w * 4..w * 5]), wv);
                a5 = L::fma(a5, L::load(&r[w * 5..w * 6]), wv);
                a6 = L::fma(a6, L::load(&r[w * 6..w * 7]), wv);
                a7 = L::fma(a7, L::load(&r[w * 7..w * 8]), wv);
            }
            let o = &mut out[col..col + block];
            L::store(&mut o[0..w], a0);
            L::store(&mut o[w..w * 2], a1);
            L::store(&mut o[w * 2..w * 3], a2);
            L::store(&mut o[w * 3..w * 4], a3);
            L::store(&mut o[w * 4..w * 5], a4);
            L::store(&mut o[w * 5..w * 6], a5);
            L::store(&mut o[w * 6..w * 7], a6);
            L::store(&mut o[w * 7..w * 8], a7);
            col += block;
        }
    }

    if L::VERTICAL_UNROLL >= 4 {
        let block = w * 4;
        while col + block <= col_end {
            let (mut a0, mut a1, mut a2, mut a3) = if FIRST {
                (L::zero(), L::zero(), L::zero(), L::zero())
            } else {
                let o = &out[col..col + block];
                (
                    L::load(&o[0..w]),
                    L::load(&o[w..w * 2]),
                    L::load(&o[w * 2..w * 3]),
                    L::load(&o[w * 3..w * 4]),
                )
            };
            for (row, &weight) in rows.iter().zip(weights) {
                let wv = L::splat(weight);
                let r = &row[col..col + block];
                a0 = L::fma(a0, L::load(&r[0..w]), wv);
                a1 = L::fma(a1, L::load(&r[w..w * 2]), wv);
                a2 = L::fma(a2, L::load(&r[w * 2..w * 3]), wv);
                a3 = L::fma(a3, L::load(&r[w * 3..w * 4]), wv);
            }
            let o = &mut out[col..col + block];
            L::store(&mut o[0..w], a0);
            L::store(&mut o[w..w * 2], a1);
            L::store(&mut o[w * 2..w * 3], a2);
            L::store(&mut o[w * 3..w * 4], a3);
            col += block;
        }
    } else if L::VERTICAL_UNROLL == 2 {
        let block = w * 2;
        while col + block <= col_end {
            let (mut a0, mut a1) = if FIRST {
                (L::zero(), L::zero())
            } else {
                let o = &out[col..col + block];
                (L::load(&o[0..w]), L::load(&o[w..w * 2]))
            };
            for (row, &weight) in rows.iter().zip(weights) {
                let wv = L::splat(weight);
                let r = &row[col..col + block];
                a0 = L::fma(a0, L::load(&r[0..w]), wv);
                a1 = L::fma(a1, L::load(&r[w..w * 2]), wv);
            }
            let o = &mut out[col..col + block];
            L::store(&mut o[0..w], a0);
            L::store(&mut o[w..w * 2], a1);
            col += block;
        }
    }

    while col + w <= col_end {
        let mut a0 = if FIRST {
            L::zero()
        } else {
            L::load(&out[col..col + w])
        };
        for (row, &weight) in rows.iter().zip(weights) {
            a0 = L::fma(a0, L::load(&row[col..col + w]), L::splat(weight));
        }
        L::store(&mut out[col..col + w], a0);
        col += w;
    }

    while col < col_end {
        let mut a = if FIRST { 0.0 } else { out[col] };
        for (row, &weight) in rows.iter().zip(weights) {
            a = row[col].mul_add(weight, a);
        }
        out[col] = a;
        col += 1;
    }
}

/// Four-row fused vertical strip with hoisted weights and pre-sliced rows
/// (`r0..r3`, `out` all have identical length). Bounding each pass to four live
/// input streams plus `out` improves locality on wide rows and power-of-two
/// contiguous ring strides (e.g. 8192-pixel = 128 KiB rows), eliminates
/// per-block weight broadcasts (`splat`) and bounds checks, and accumulates
/// taps `0, 1, 2, 3` in exact order.
#[inline(always)]
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn vertical_strip_4<L: Lanes, const FIRST: bool>(
    r0: &[f32],
    r1: &[f32],
    r2: &[f32],
    r3: &[f32],
    w0: f32,
    w1: f32,
    w2: f32,
    w3: f32,
    out: &mut [f32],
) {
    let w = L::SLOTS * 4;
    let (wv0, wv1, wv2, wv3) = (L::splat(w0), L::splat(w1), L::splat(w2), L::splat(w3));
    let len = out.len();
    let mut col = 0;

    let block = w * 4;
    while col + block <= len {
        let (mut a0, mut a1, mut a2, mut a3) = if FIRST {
            (L::zero(), L::zero(), L::zero(), L::zero())
        } else {
            let o = &out[col..col + block];
            (
                L::load(&o[0..w]),
                L::load(&o[w..w * 2]),
                L::load(&o[w * 2..w * 3]),
                L::load(&o[w * 3..w * 4]),
            )
        };
        let s0 = &r0[col..col + block];
        a0 = L::fma(a0, L::load(&s0[0..w]), wv0);
        a1 = L::fma(a1, L::load(&s0[w..w * 2]), wv0);
        a2 = L::fma(a2, L::load(&s0[w * 2..w * 3]), wv0);
        a3 = L::fma(a3, L::load(&s0[w * 3..w * 4]), wv0);
        let s1 = &r1[col..col + block];
        a0 = L::fma(a0, L::load(&s1[0..w]), wv1);
        a1 = L::fma(a1, L::load(&s1[w..w * 2]), wv1);
        a2 = L::fma(a2, L::load(&s1[w * 2..w * 3]), wv1);
        a3 = L::fma(a3, L::load(&s1[w * 3..w * 4]), wv1);
        let s2 = &r2[col..col + block];
        a0 = L::fma(a0, L::load(&s2[0..w]), wv2);
        a1 = L::fma(a1, L::load(&s2[w..w * 2]), wv2);
        a2 = L::fma(a2, L::load(&s2[w * 2..w * 3]), wv2);
        a3 = L::fma(a3, L::load(&s2[w * 3..w * 4]), wv2);
        let s3 = &r3[col..col + block];
        a0 = L::fma(a0, L::load(&s3[0..w]), wv3);
        a1 = L::fma(a1, L::load(&s3[w..w * 2]), wv3);
        a2 = L::fma(a2, L::load(&s3[w * 2..w * 3]), wv3);
        a3 = L::fma(a3, L::load(&s3[w * 3..w * 4]), wv3);
        let o = &mut out[col..col + block];
        L::store(&mut o[0..w], a0);
        L::store(&mut o[w..w * 2], a1);
        L::store(&mut o[w * 2..w * 3], a2);
        L::store(&mut o[w * 3..w * 4], a3);
        col += block;
    }

    while col + w <= len {
        let mut a0 = if FIRST {
            L::zero()
        } else {
            L::load(&out[col..col + w])
        };
        a0 = L::fma(a0, L::load(&r0[col..col + w]), wv0);
        a0 = L::fma(a0, L::load(&r1[col..col + w]), wv1);
        a0 = L::fma(a0, L::load(&r2[col..col + w]), wv2);
        a0 = L::fma(a0, L::load(&r3[col..col + w]), wv3);
        L::store(&mut out[col..col + w], a0);
        col += w;
    }

    while col < len {
        let mut a = if FIRST { 0.0 } else { out[col] };
        a = r0[col].mul_add(w0, a);
        a = r1[col].mul_add(w1, a);
        a = r2[col].mul_add(w2, a);
        a = r3[col].mul_add(w3, a);
        out[col] = a;
        col += 1;
    }
}

/// Shared body for `vertical_rgba` (`INIT = true`, starting from `0.0`) and
/// `vertical_rgba_add` (`INIT = false`, accumulating into existing `out`).
#[inline(always)]
fn vertical_rgba_inner<L: Lanes, const INIT: bool>(
    rows: &[&[f32]],
    weights: &[f32],
    out: &mut [f32],
) {
    if rows.len() < 4 {
        vertical_strip::<L, INIT>(rows, weights, out, 0, out.len());
        return;
    }
    if rows.len() == 4 {
        vertical_strip_4::<L, INIT>(
            rows[0], rows[1], rows[2], rows[3], weights[0], weights[1], weights[2], weights[3], out,
        );
        return;
    }
    let mut col_start = 0;
    while col_start < out.len() {
        let col_end = (col_start + VERTICAL_COL_TILE).min(out.len());
        let out_strip = &mut out[col_start..col_end];
        let mut tap = 0;
        let mut first = INIT;
        while tap + 4 <= rows.len() {
            let (r0, r1, r2, r3) = (
                &rows[tap][col_start..col_end],
                &rows[tap + 1][col_start..col_end],
                &rows[tap + 2][col_start..col_end],
                &rows[tap + 3][col_start..col_end],
            );
            if first {
                vertical_strip_4::<L, true>(
                    r0,
                    r1,
                    r2,
                    r3,
                    weights[tap],
                    weights[tap + 1],
                    weights[tap + 2],
                    weights[tap + 3],
                    out_strip,
                );
                first = false;
            } else {
                vertical_strip_4::<L, false>(
                    r0,
                    r1,
                    r2,
                    r3,
                    weights[tap],
                    weights[tap + 1],
                    weights[tap + 2],
                    weights[tap + 3],
                    out_strip,
                );
            }
            tap += 4;
        }
        if tap < rows.len() {
            if first {
                vertical_strip::<L, true>(&rows[tap..], &weights[tap..], out, col_start, col_end);
            } else {
                vertical_strip::<L, false>(&rows[tap..], &weights[tap..], out, col_start, col_end);
            }
        }
        col_start = col_end;
    }
}

/// See `Kernels::vertical_rgba`. The caller has validated that `rows.len() ==
/// weights.len()`, every `row.len() == out.len()`, and `rows` is non-empty.
///
/// Each element accumulates taps in exact order `0..rows.len()` starting from
/// `0.0` via fused multiply-add, so results are bit-identical to zeroing `out`
/// and running `axpy` per tap at every ISA level.
#[inline(always)]
pub(crate) fn vertical_rgba<L: Lanes>(rows: &[&[f32]], weights: &[f32], out: &mut [f32]) {
    vertical_rgba_inner::<L, true>(rows, weights, out);
}

/// See `Kernels::vertical_rgba_add`. Accumulates `rows * weights` into
/// existing `acc` in exact tap order `0..rows.len()`.
#[inline(always)]
pub(crate) fn vertical_rgba_add<L: Lanes>(rows: &[&[f32]], weights: &[f32], acc: &mut [f32]) {
    vertical_rgba_inner::<L, false>(rows, weights, acc);
}

/// Degree-8 Remez minimax coefficients for `u^1.6` on
/// `[((0.04045 + 0.055) / 1.055)^0.25, 1.0]`, weighted by `1 / u^1.6`
/// (max relative error `1.39e-10`). With `t = (|x| + 0.055) / 1.055` and
/// `u = sqrt(sqrt(t))`, `t^2 * P(u)` matches `((|x| + 0.055) / 1.055)^2.4`
/// to `< 1.4e-10` relative error (430x finer than 1 ULP of `f32`).
const EOTF_C0: f64 = -6.335_193_213_440_115e-03;
const EOTF_C1: f64 = 1.838_822_428_917_036e-01;
const EOTF_C2: f64 = 1.306_064_415_403_507_3e+00;
const EOTF_C3: f64 = -1.004_001_397_013_697_2e+00;
const EOTF_C4: f64 = 9.792_986_212_645_558e-01;
const EOTF_C5: f64 = -7.356_100_235_950_327e-01;
const EOTF_C6: f64 = 3.759_463_673_632_575e-01;
const EOTF_C7: f64 = -1.151_308_532_324_695e-01;
const EOTF_C8: f64 = 1.588_582_027_037_883_2e-02;

#[inline(always)]
fn srgb_to_linear_unit<L: LanesF64>(x: L::V) -> L::V {
    let ax = L::abs(x);
    let linear_seg = L::mul(ax, L::splat(1.0 / 12.92));
    let t = L::mul_add(L::splat(0.055 / 1.055), ax, L::splat(1.0 / 1.055));
    let u = L::sqrt(L::sqrt(t));
    let u2 = L::mul(u, u);
    let u4 = L::mul(u2, u2);
    let u8 = L::mul(u4, u4);
    let p01 = L::mul_add(L::splat(EOTF_C0), L::splat(EOTF_C1), u);
    let p23 = L::mul_add(L::splat(EOTF_C2), L::splat(EOTF_C3), u);
    let p45 = L::mul_add(L::splat(EOTF_C4), L::splat(EOTF_C5), u);
    let p67 = L::mul_add(L::splat(EOTF_C6), L::splat(EOTF_C7), u);
    let p03 = L::mul_add(p01, p23, u2);
    let p47 = L::mul_add(p45, p67, u2);
    let p07 = L::mul_add(p03, p47, u4);
    let poly = L::mul_add(p07, L::splat(EOTF_C8), u8);
    let curved = L::mul(L::mul(t, t), poly);
    let mag = L::select_le(ax, L::splat(0.040_45), linear_seg, curved);
    L::copysign(mag, x)
}

/// Exact `f32` magnitude bit pattern for the sRGB linear-segment threshold
/// `0.04045_f32` (`f64::from(0.04045_f32) < 0.04045_f64`, while the next `f32`
/// up exceeds `0.04045_f64`).
const EOTF_LINEAR_THRESHOLD_BITS: u32 = 0.040_45_f32.to_bits();
/// Exact `f32` magnitude bit pattern for `1.0_f32`.
const UNIT_BITS: u32 = 1.0_f32.to_bits();

/// Scalar counterpart of [`srgb_to_linear_unit`] using `L::scalar_mul_add`, so
/// tails and mixed-range vectors execute the exact same fused or unfused
/// arithmetic as `L`'s vector path.
#[inline(always)]
fn srgb_to_linear_unit_scalar<L: LanesF64>(x: f64) -> f64 {
    let ax = x.abs();
    if ax <= 0.040_45 {
        return x * (1.0 / 12.92);
    }
    let t = L::scalar_mul_add(0.055 / 1.055, ax, 1.0 / 1.055);
    let u = t.sqrt().sqrt();
    let u2 = u * u;
    let u4 = u2 * u2;
    let u8 = u4 * u4;
    let p01 = L::scalar_mul_add(EOTF_C0, EOTF_C1, u);
    let p23 = L::scalar_mul_add(EOTF_C2, EOTF_C3, u);
    let p45 = L::scalar_mul_add(EOTF_C4, EOTF_C5, u);
    let p67 = L::scalar_mul_add(EOTF_C6, EOTF_C7, u);
    let p03 = L::scalar_mul_add(p01, p23, u2);
    let p47 = L::scalar_mul_add(p45, p67, u2);
    let p07 = L::scalar_mul_add(p03, p47, u4);
    let poly = L::scalar_mul_add(p07, EOTF_C8, u8);
    let curved = (t * t) * poly;
    curved.copysign(x)
}

#[inline(always)]
fn srgb_to_linear_scalar<L: LanesF64>(sample: f32) -> f64 {
    let value = f64::from(sample);
    if value.abs() <= 1.0 {
        srgb_to_linear_unit_scalar::<L>(value)
    } else {
        let magnitude = value.abs();
        value.signum() * ((magnitude + 0.055) / 1.055).powf(2.4)
    }
}

/// Largest unsigned magnitude bit pattern across `samples`. Branchless so
/// LLVM auto-vectorizes it with `umax` / `vpmaxud`.
#[inline(always)]
fn max_abs_bits(samples: &[f32]) -> u32 {
    let mut max_bits = 0_u32;
    for &sample in samples {
        max_bits = max_bits.max(sample.to_bits() & 0x7FFF_FFFF);
    }
    max_bits
}

/// See `Kernels::srgb_to_linear`. The caller has validated the lengths.
#[inline(always)]
pub(crate) fn srgb_to_linear<L: LanesF64>(src: &[f32], dst: &mut [f64]) {
    let width = L::WIDTH;
    let full = src.len() / width * width;
    let (src_body, src_tail) = src.split_at(full);
    let (dst_body, dst_tail) = dst.split_at_mut(full);
    let peak = max_abs_bits(src_body);
    if peak <= EOTF_LINEAR_THRESHOLD_BITS {
        let inv = L::splat(1.0 / 12.92);
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            L::store_f64(d, L::mul(L::load_f32(s), inv));
        }
    } else if peak <= UNIT_BITS {
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            L::store_f64(d, srgb_to_linear_unit::<L>(L::load_f32(s)));
        }
    } else {
        let one = L::splat(1.0);
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            let x = L::load_f32(s);
            if L::all_le(L::abs(x), one) {
                L::store_f64(d, srgb_to_linear_unit::<L>(x));
            } else {
                for (out, &sample) in d.iter_mut().zip(s) {
                    *out = srgb_to_linear_scalar::<L>(sample);
                }
            }
        }
    }
    for (out, &sample) in dst_tail.iter_mut().zip(src_tail) {
        *out = srgb_to_linear_scalar::<L>(sample);
    }
}

/// Maximum distance from the rounded integer (`|scaled - round(scaled)|`) for
/// which the polynomial OETF on the curved segment cannot straddle a half-code
/// rounding threshold `k + 0.5`. The polynomial error on `(0.0031308, 1.0]` is
/// `< 6e-11` (`< 4e-6` of a 16-bit code step, including `a * b + c` vs FMA
/// differences `< 1e-14`); a `1e-4` guard band is >25x wider while keeping
/// over 99.98% of curved lanes on the SIMD fast path. Linear-segment lanes
/// (`<= 0.0031308`) are exact (`12.92 * clamped`) and pass limit `1.0` so they
/// never trigger refinement.
const HALF_CODE_LIMIT: f64 = 0.5 - 1.0e-4;

/// Largest `f32` value whose widened `f64` is `<= 0.0031308_f64`.
const OETF_LINEAR_THRESHOLD_F32: f32 = {
    let x = 0.003_130_8_f32;
    if (x as f64) <= 0.003_130_8_f64 {
        x
    } else {
        x.next_down()
    }
};

/// `true` iff every finite sample in `samples` falls in the exact linear OETF
/// segment (`<= 0.0031308_f64`, including negative values clamped to `0.0`).
/// Short-circuits on the first bright sample so normal images pay ~0 overhead.
#[inline(always)]
fn all_oetf_linear(samples: &[f32]) -> bool {
    samples
        .iter()
        .all(|&sample| sample <= OETF_LINEAR_THRESHOLD_F32)
}

/// Degree-8 Remez minimax approximation of `1.055 * z^(10/3) - 0.055` on
/// `[0.0031308^0.125, 1.0]` with `z = sqrt(sqrt(sqrt(x)))` (max absolute error
/// `5.91e-11`, or `3.9e-6` of a 16-bit code step). Returns `(encoded, limit)`
/// where linear-segment lanes (`<= 0.0031308`) carry `limit = 1.0` (exact, no
/// refinement needed) and curved lanes carry `limit = HALF_CODE_LIMIT`.
#[inline(always)]
fn linear_to_srgb_unit<L: LanesF64>(x: L::V) -> (L::V, L::V) {
    const C0: f64 = -5.542_793_273_847_191_5e-02;
    const C1: f64 = 7.009_144_686_315_251e-03;
    const C2: f64 = -6.102_448_520_459_556e-02;
    const C3: f64 = 6.882_975_397_802_682e-01;
    const C4: f64 = 6.016_079_414_773_584e-01;
    const C5: f64 = -2.670_754_412_189_171_3e-01;
    const C6: f64 = 1.148_725_112_899_608_6e-01;
    const C7: f64 = -3.259_848_748_051_394e-02;
    const C8: f64 = 4.339_209_467_645_605_5e-03;

    let clamped = L::clamp01(x);
    let threshold = L::splat(0.003_130_8);
    let linear_seg = L::mul(clamped, L::splat(12.92));
    let z = L::sqrt(L::sqrt(L::sqrt(clamped)));
    let z2 = L::mul(z, z);
    let z4 = L::mul(z2, z2);
    let z8 = L::mul(z4, z4);
    let p01 = L::mul_add(L::splat(C0), L::splat(C1), z);
    let p23 = L::mul_add(L::splat(C2), L::splat(C3), z);
    let p45 = L::mul_add(L::splat(C4), L::splat(C5), z);
    let p67 = L::mul_add(L::splat(C6), L::splat(C7), z);
    let p03 = L::mul_add(p01, p23, z2);
    let p47 = L::mul_add(p45, p67, z2);
    let p07 = L::mul_add(p03, p47, z4);
    let curved = L::mul_add(p07, L::splat(C8), z8);
    let encoded = L::select_le(clamped, threshold, linear_seg, curved);
    let limit = L::select_le(clamped, threshold, L::splat(1.0), L::splat(HALF_CODE_LIMIT));
    (encoded, limit)
}

/// Exact `f64` sRGB OETF fallback for only the curved-segment lanes whose
/// polynomial output landed inside the half-code guard band around `k + 0.5`
/// (indicated by set bits in `mask`). Keeping this out-of-line avoids spilling
/// SIMD registers in the fast path.
#[cold]
#[inline(never)]
fn refine_linear_to_srgb_u16_masked(src: &[f32], max: u16, dst: &mut [u16], mut mask: u8) {
    let scale = f64::from(max);
    while mask != 0 {
        let lane = mask.trailing_zeros() as usize;
        mask &= mask - 1;
        #[allow(clippy::manual_clamp)]
        let clamped = f64::from(src[lane]).max(0.0).min(1.0);
        let encoded = if clamped <= 0.003_130_8 {
            12.92 * clamped
        } else {
            1.055 * clamped.powf(1.0 / 2.4) - 0.055
        };
        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
        {
            dst[lane] = (encoded * scale).round() as u16;
        }
    }
}

/// See `Kernels::linear_to_srgb_u16`. The caller has validated the lengths and
/// finiteness of `linear`.
#[inline(always)]
pub(crate) fn linear_to_srgb_u16<L: LanesF64>(linear: &[f32], max: u16, out: &mut [u16]) {
    let width = L::WIDTH;
    let full = linear.len() / width * width;
    let (src_body, src_tail) = linear.split_at(full);
    let (dst_body, dst_tail) = out.split_at_mut(full);
    let scale = L::splat(f64::from(max));
    let scale_scalar = f64::from(max);
    if all_oetf_linear(linear) {
        let slope = L::splat(12.92);
        let one = L::splat(1.0);
        for (s, d) in src_body
            .chunks_exact(width)
            .zip(dst_body.chunks_exact_mut(width))
        {
            let encoded = L::mul(L::clamp01(L::load_f32(s)), slope);
            let _ = L::store_u16_round_checked(d, L::mul(encoded, scale), one);
        }
        for (&s, d) in src_tail.iter().zip(dst_tail) {
            let clamped = Scalar::clamp01(f64::from(s));
            let _ = Scalar::store_u16_round_checked(
                std::slice::from_mut(d),
                (clamped * 12.92) * scale_scalar,
                1.0,
            );
        }
        return;
    }
    for (s, d) in src_body
        .chunks_exact(width)
        .zip(dst_body.chunks_exact_mut(width))
    {
        let (encoded, limit) = linear_to_srgb_unit::<L>(L::load_f32(s));
        let mask = L::store_u16_round_checked(d, L::mul(encoded, scale), limit);
        if mask != 0 {
            refine_linear_to_srgb_u16_masked(s, max, d, mask);
        }
    }
    for (&s, d) in src_tail.iter().zip(dst_tail) {
        let clamped = Scalar::clamp01(f64::from(s));
        if clamped <= 0.003_130_8 {
            let _ = Scalar::store_u16_round_checked(
                std::slice::from_mut(d),
                (clamped * 12.92) * scale_scalar,
                1.0,
            );
        } else {
            let (encoded, limit) = linear_to_srgb_unit::<Scalar>(clamped);
            let mask = Scalar::store_u16_round_checked(
                std::slice::from_mut(d),
                encoded * scale_scalar,
                limit,
            );
            if mask != 0 {
                refine_linear_to_srgb_u16_masked(
                    std::slice::from_ref(&s),
                    max,
                    std::slice::from_mut(d),
                    mask,
                );
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Portable `SLOTS = 2, HORIZONTAL_PIXELS = 2` model exercising the exact
    /// `Avx2` kernel body paths (`horizontal_2px`, `horizontal_single`,
    /// `vertical_rgba`, `vertical_rgba_add`) on any host architecture.
    struct MockAvx2;

    impl Lanes for MockAvx2 {
        const SLOTS: usize = 2;
        const HORIZONTAL_PIXELS: usize = 2;
        const VERTICAL_UNROLL: usize = 4;
        type V = [[f32; 4]; 2];
        fn zero() -> Self::V {
            [[0.0; 4]; 2]
        }
        fn load(values: &[f32]) -> Self::V {
            let v = values.first_chunk::<8>().expect("eight values");
            [[v[0], v[1], v[2], v[3]], [v[4], v[5], v[6], v[7]]]
        }
        fn store(values: &mut [f32], vector: Self::V) {
            let out = values.first_chunk_mut::<8>().expect("eight values");
            out[0..4].copy_from_slice(&vector[0]);
            out[4..8].copy_from_slice(&vector[1]);
        }
        fn splat(value: f32) -> Self::V {
            [[value; 4]; 2]
        }
        fn slot_weights(weights: &[f32]) -> Self::V {
            [[weights[0]; 4], [weights[1]; 4]]
        }
        fn fma(acc: Self::V, a: Self::V, b: Self::V) -> Self::V {
            std::array::from_fn(|s| std::array::from_fn(|c| a[s][c].mul_add(b[s][c], acc[s][c])))
        }
        fn reduce_slots(vector: Self::V) -> [f32; 4] {
            std::array::from_fn(|c| vector[0][c] + vector[1][c])
        }
    }

    /// Portable `SLOTS = 4, HORIZONTAL_PIXELS = 2` model exercising the exact
    /// `Avx512` kernel body paths (`horizontal_2px`, `horizontal_single`,
    /// `vertical_rgba`, `vertical_rgba_add`) on hosts without AVX-512.
    struct MockAvx512;

    impl Lanes for MockAvx512 {
        const SLOTS: usize = 4;
        const HORIZONTAL_PIXELS: usize = 2;
        const VERTICAL_UNROLL: usize = 2;
        type V = [[f32; 4]; 4];
        fn zero() -> Self::V {
            [[0.0; 4]; 4]
        }
        fn load(values: &[f32]) -> Self::V {
            let v = values.first_chunk::<16>().expect("sixteen values");
            std::array::from_fn(|s| [v[s * 4], v[s * 4 + 1], v[s * 4 + 2], v[s * 4 + 3]])
        }
        fn store(values: &mut [f32], vector: Self::V) {
            let out = values.first_chunk_mut::<16>().expect("sixteen values");
            for s in 0..4 {
                out[s * 4..(s + 1) * 4].copy_from_slice(&vector[s]);
            }
        }
        fn splat(value: f32) -> Self::V {
            [[value; 4]; 4]
        }
        fn slot_weights(weights: &[f32]) -> Self::V {
            std::array::from_fn(|s| [weights[s]; 4])
        }
        fn fma(acc: Self::V, a: Self::V, b: Self::V) -> Self::V {
            std::array::from_fn(|s| std::array::from_fn(|c| a[s][c].mul_add(b[s][c], acc[s][c])))
        }
        fn reduce_slots(vector: Self::V) -> [f32; 4] {
            std::array::from_fn(|c| ((vector[0][c] + vector[1][c]) + vector[2][c]) + vector[3][c])
        }
    }

    fn assert_pipelined_matches_single<L: Lanes>(source: &[f32], taps: &Taps, label: &str) {
        let mut multi = vec![f32::NAN; taps.outputs() * 4];
        horizontal_rgba::<L>(source, taps, &mut multi);
        for p in 0..taps.outputs() {
            let start = taps.starts[p] as usize * 4;
            let weights = &taps.weights[taps.offsets[p] as usize..taps.offsets[p + 1] as usize];
            let single = horizontal_single::<L>(&source[start..start + weights.len() * 4], weights);
            for c in 0..4 {
                assert_eq!(
                    multi[p * 4 + c].to_bits(),
                    single[c].to_bits(),
                    "{label} pos={p} c={c}"
                );
            }
        }
    }

    #[test]
    fn pipelined_avx2_and_avx512_match_single_pixel_bit_for_bit() {
        // Variable border tap counts across output widths 1..=17.
        let source_pixels = 64;
        let source: Vec<f32> = (0..source_pixels * 4)
            .map(|i| {
                #[allow(clippy::cast_precision_loss)]
                let v = ((i * 37 + 11) % 257) as f32 / 128.0 - 0.5;
                v
            })
            .collect();
        for outputs in 1..=17_usize {
            let mut starts = Vec::with_capacity(outputs);
            let mut offsets = vec![0_u32];
            let mut weights = Vec::new();
            for p in 0..outputs {
                let count = match p {
                    0 => 3,
                    1 => 5,
                    _ if p + 1 == outputs => 4,
                    _ => 7 + (p % 5),
                };
                let start = (p * 2) % (source_pixels - count);
                starts.push(u32::try_from(start).unwrap());
                for k in 0..count {
                    #[allow(clippy::cast_precision_loss)]
                    let w = ((k * 13 + p * 7) % 19) as f32 / 10.0 - 0.4;
                    weights.push(w);
                }
                offsets.push(u32::try_from(weights.len()).unwrap());
            }
            let taps = Taps::new(source_pixels, starts, offsets, weights).unwrap();
            assert_pipelined_matches_single::<MockAvx2>(&source, &taps, "MockAvx2");
            assert_pipelined_matches_single::<MockAvx512>(&source, &taps, "MockAvx512");
        }

        // Short / uneven 3x-Area-style tap tables (mixing 1-tap and 2-tap runs,
        // plus 1-tap-only tables) exercise the `max_run` short-tap bypass and
        // `common == 0` fallback on every ISA width.
        for pattern in [&[1_usize, 1, 2, 2, 1, 2, 2, 1, 2][..], &[1, 1, 1, 1, 1, 1]] {
            let mut starts = Vec::with_capacity(pattern.len());
            let mut offsets = vec![0_u32];
            let mut weights = Vec::new();
            for (p, &count) in pattern.iter().enumerate() {
                starts.push(u32::try_from(p % (source_pixels - count)).unwrap());
                for k in 0..count {
                    #[allow(clippy::cast_precision_loss)]
                    weights.push((k + 1) as f32 / (count * (count + 1) / 2) as f32);
                }
                offsets.push(u32::try_from(weights.len()).unwrap());
            }
            let taps = Taps::new(source_pixels, starts, offsets, weights).unwrap();
            assert_pipelined_matches_single::<Scalar>(&source, &taps, "Scalar-short");
            assert_pipelined_matches_single::<MockAvx2>(&source, &taps, "MockAvx2-short");
            assert_pipelined_matches_single::<MockAvx512>(&source, &taps, "MockAvx512-short");
        }
    }

    #[test]
    fn pipelined_avx2_and_avx512_preserve_cancellation_and_uniform_sums() {
        // 64 uniform 0.1_f32 taps across 9 outputs: pipelined == single-pixel bit-for-bit.
        let uniform_src = vec![0.1_f32; 72 * 4];
        let uniform_taps = Taps::new(
            72,
            (0..9_u32).collect(),
            (0..=9_u32).map(|i| i * 64).collect(),
            vec![1.0_f32 / 64.0; 9 * 64],
        )
        .unwrap();
        assert_pipelined_matches_single::<MockAvx2>(&uniform_src, &uniform_taps, "MockAvx2-u64");
        assert_pipelined_matches_single::<MockAvx512>(
            &uniform_src,
            &uniform_taps,
            "MockAvx512-u64",
        );

        // Both large opposite-sign cancellation patterns across 9 outputs.
        let patterns = [
            [
                67_108_864.0_f32,
                0.0,
                -67_108_864.0,
                0.0,
                1.0,
                0.0,
                0.0,
                0.0,
            ],
            [
                67_108_864.0_f32,
                1.0,
                0.0,
                0.0,
                -67_108_864.0,
                0.0,
                0.0,
                0.0,
            ],
        ];
        let cancel_taps = Taps::new(
            72,
            (0..9_u32).map(|i| i * 8).collect(),
            (0..=9_u32).map(|i| i * 8).collect(),
            vec![0.125_f32; 9 * 8],
        )
        .unwrap();
        for red_pattern in patterns {
            let mut cancel_src = Vec::with_capacity(9 * 8 * 4);
            for _ in 0..9 {
                for &r in &red_pattern {
                    cancel_src.extend_from_slice(&[r, 0.0, 0.0, 0.0]);
                }
            }
            assert_pipelined_matches_single::<MockAvx2>(&cancel_src, &cancel_taps, "MockAvx2-c");
            assert_pipelined_matches_single::<MockAvx512>(
                &cancel_src,
                &cancel_taps,
                "MockAvx512-c",
            );
            let mut c2 = vec![f32::NAN; 9 * 4];
            horizontal_rgba::<MockAvx2>(&cancel_src, &cancel_taps, &mut c2);
            for p in 0..9 {
                assert_eq!(c2[p * 4].to_bits(), 0.125_f32.to_bits());
            }
        }
    }

    #[test]
    fn packed_avx512_vertical_add_is_bit_identical_to_scalar() {
        // Vertical init + add chunking on MockAvx512.
        for length in [0, 1, 15, 16, 17, 31, 32, 33, 65, 257] {
            let storage: Vec<Vec<f32>> = (0..70)
                .map(|t| {
                    (0..length)
                        .map(|i| {
                            #[allow(clippy::cast_precision_loss)]
                            let v = ((t * 29 + i * 7) % 101) as f32 / 50.0 - 0.5;
                            v
                        })
                        .collect()
                })
                .collect();
            let weights: Vec<f32> = (0..70)
                .map(|t| {
                    #[allow(clippy::cast_precision_loss)]
                    let w = ((t * 17) % 31) as f32 / 20.0 - 0.3;
                    w
                })
                .collect();
            let row_slices: Vec<&[f32]> = storage.iter().map(Vec::as_slice).collect();
            let mut exp = vec![0.0_f32; length];
            vertical_rgba::<Scalar>(&row_slices, &weights, &mut exp);
            let mut actual = vec![f32::NAN; length];
            vertical_rgba::<MockAvx512>(&row_slices[..35], &weights[..35], &mut actual);
            vertical_rgba_add::<MockAvx512>(&row_slices[35..], &weights[35..], &mut actual);
            for i in 0..length {
                assert_eq!(actual[i].to_bits(), exp[i].to_bits());
            }
        }
    }
}
