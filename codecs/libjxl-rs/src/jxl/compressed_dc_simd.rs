// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! AdaptiveDCSmoothing and DequantDC of lib/jxl/compressed_dc.cc for the
//! NEON and AVX2 targets (port ADR 0004), written once against
//! [`SimdF32`]. Compare them only with the original's SIMD build at the
//! same target.
//!
//! The original smooths the first and last pixels of a row with
//! `HWY_CAPPED(float, 1)` and the rest with full vectors; every operation is
//! lane-wise, so a one-lane pixel is computed here in a full vector whose
//! lane 0 holds it. DequantDC stores full vectors up to the next lane
//! multiple of the rectangle width, from quantized samples past the
//! channel's width that the original never initialized; those words are
//! not comparable, and the port stores only the rectangle's samples.
use super::ac_context::BlockCtxMap;
use super::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use super::base::rect::Rect;
use super::base::status::Status;
use super::frame_header::YCbCrChromaSubsampling;
use super::hwy::{Kernel, SimdF32, Target, dispatch};
use super::image::{Image3, Plane};
use super::memory_manager_internal::MemoryManagerPtr;
use super::modular::modular_image::Image;

const W1: f32 = 0.203_451_4;
const W2: f32 = 0.033_482_92;
const W0: f32 = 1.0 - 4.0 * (W1 + W2);

/// ComputePixelChannel at `x` of the three rows (`x - 1 .. x + LANES + 1`).
#[inline(always)]
fn pixel_channel<D: SimdF32>(
    d: D,
    dc_factor: f32,
    rows: [&[f32]; 3],
    gap: &mut D::V,
    x: usize,
) -> (D::V, D::V) {
    let [top, row, bottom] = rows;
    let tl = d.load(&top[x - 1..]);
    let tc = d.load(&top[x..]);
    let tr = d.load(&top[x + 1..]);
    let ml = d.load(&row[x - 1..]);
    let mc = d.load(&row[x..]);
    let mr = d.load(&row[x + 1..]);
    let bl = d.load(&bottom[x - 1..]);
    let bc = d.load(&bottom[x..]);
    let br = d.load(&bottom[x + 1..]);
    let corner = d.add(d.add(tl, tr), d.add(bl, br));
    let side = d.add(d.add(ml, mr), d.add(tc, bc));
    let sm = d.mul_add(
        corner,
        d.set(W2),
        d.mul_add(side, d.set(W1), d.mul(mc, d.set(W0))),
    );
    *gap = d.max(*gap, d.abs(d.div(d.sub(mc, sm), d.set(dc_factor))));
    (mc, sm)
}

/// ComputePixel (`channels` = 3) or ComputePixelSingleChannel (1) at `x`.
#[inline(always)]
fn pixel<D: SimdF32>(
    d: D,
    dc_factors: &[f32],
    rows: &[[&[f32]; 3]],
    out: &mut [Vec<f32>],
    x: usize,
    out_x: usize,
) {
    let mut gap = d.set(0.5);
    let mut mc = [d.zero(); 3];
    let mut sm = [d.zero(); 3];
    for c in 0..rows.len() {
        (mc[c], sm[c]) = pixel_channel(d, dc_factors[c], rows[c], &mut gap, x);
    }
    let factor = d.zero_if_negative(d.mul_add(d.set(-4.0), gap, d.set(3.0)));
    for c in 0..rows.len() {
        let value = d.mul_add(d.sub(sm[c], mc[c]), factor, mc[c]);
        d.store(value, &mut out[c][out_x..]);
    }
}

/// One smoothed row of `rows.len()` channels into `out` (xsize samples
/// each, borders copied).
struct SmoothRow<'a> {
    dc_factors: &'a [f32],
    rows: &'a [[&'a [f32]; 3]],
    xsize: usize,
    out: &'a mut [Vec<f32>],
}
impl Kernel for SmoothRow<'_> {
    type Output = ();
    #[inline(always)]
    #[expect(
        clippy::int_plus_one,
        reason = "Keep the source's `x + N <= xsize - 1` loop bound."
    )]
    fn run<D: SimdF32>(self, d: D) {
        let n = D::LANES;
        let xsize = self.xsize;
        // One-lane pixels: lane 0 of a vector over a copy of the neighbors.
        let one = |x: usize, out: &mut [Vec<f32>]| {
            let local: Vec<[Vec<f32>; 3]> = self
                .rows
                .iter()
                .map(|r| {
                    r.map(|row| {
                        let mut v = vec![0.0; n + 2];
                        v[..3].copy_from_slice(&row[x - 1..x + 2]);
                        v
                    })
                })
                .collect();
            let local_rows: Vec<[&[f32]; 3]> = local
                .iter()
                .map(|r| [&r[0][..], &r[1][..], &r[2][..]])
                .collect();
            let mut lane: Vec<Vec<f32>> = vec![vec![0.0; n]; self.rows.len()];
            pixel(d, self.dc_factors, &local_rows, &mut lane, 1, 0);
            for (o, l) in out.iter_mut().zip(&lane) {
                o[x] = l[0];
            }
        };
        let mut x = 1;
        while x < n.min(xsize - 1) {
            one(x, self.out);
            x += 1;
        }
        while x + n <= xsize - 1 {
            pixel(d, self.dc_factors, self.rows, self.out, x, x);
            x += n;
        }
        while x < xsize - 1 {
            one(x, self.out);
            x += 1;
        }
    }
}

/// `AdaptiveDCSmoothing` instantiated for one SIMD target; the contract of
/// [`super::compressed_dc::adaptive_dc_smoothing_in`].
pub fn adaptive_dc_smoothing_simd(
    target: Target,
    memory_manager: MemoryManagerPtr,
    dc_factors: &[f32],
    dc: &mut Image3<f32>,
    chroma: &YCbCrChromaSubsampling,
    pool: Option<&ThreadPool>,
) -> Status {
    let xsize = dc.xsize();
    let ysize = dc.ysize();
    if ysize <= 2 || xsize <= 2 {
        return Ok(());
    }
    let channel_sets: Vec<(Vec<usize>, usize, usize)> = if chroma.is_444() {
        vec![(vec![0, 1, 2], xsize, ysize)]
    } else {
        (0..3)
            .map(|c| {
                (
                    vec![c],
                    xsize >> chroma.h_shift(c),
                    ysize >> chroma.v_shift(c),
                )
            })
            .filter(|(_, w, h)| *h > 2 && *w > 2)
            .collect()
    };
    // The source allocates an Image3F (4:4:4) or one ImageF reused, through
    // swaps, for every channel.
    let mut smoothed: Vec<Plane<f32>> = Vec::with_capacity(3);
    for _ in 0..if chroma.is_444() { 3 } else { 1 } {
        smoothed.push(Plane::create_in(xsize, ysize, 0, memory_manager)?);
    }
    for (channels, width, height) in channel_sets {
        for (s, &c) in smoothed.iter_mut().zip(&channels) {
            for y in [0, height - 1] {
                s.row_mut(y)[..width].copy_from_slice(&dc.plane_row(c, y)[..width]);
            }
        }
        let factors: Vec<f32> = channels.iter().map(|&c| dc_factors[c]).collect();
        let dc_ref = &*dc;
        let process_row = |smoothed: &mut Vec<Plane<f32>>, y: u32, _thread: usize| -> Status {
            let y = y as usize;
            let rows: Vec<[&[f32]; 3]> = channels
                .iter()
                .map(|&c| {
                    [
                        &dc_ref.plane_row(c, y - 1)[..width],
                        &dc_ref.plane_row(c, y)[..width],
                        &dc_ref.plane_row(c, y + 1)[..width],
                    ]
                })
                .collect();
            let mut out: Vec<Vec<f32>> = rows.iter().map(|r| r[1].to_vec()).collect();
            dispatch(
                target,
                SmoothRow {
                    dc_factors: &factors,
                    rows: &rows,
                    xsize: width,
                    out: &mut out,
                },
            );
            for (s, o) in smoothed.iter_mut().zip(&out) {
                s.row_mut(y)[..width].copy_from_slice(o);
            }
            Ok(())
        };
        // SAFETY: as in the scalar AdaptiveDCSmoothing, each task writes only
        // its own row `y` of the exclusively borrowed `smoothed` owners.
        unsafe {
            run_on_pool_exclusive_no_init(pool, 1, (height - 1) as u32, &mut smoothed, process_row)
        }?;
        for (s, &c) in smoothed.iter_mut().zip(&channels) {
            dc.plane_mut(c).swap(s);
        }
    }
    Ok(())
}

#[inline(always)]
fn load_i32<D: SimdF32>(d: D, values: &[i32]) -> D::I {
    let mut bits = [0u32; 16];
    for (b, &v) in bits.iter_mut().zip(&values[..D::LANES]) {
        *b = v as u32;
    }
    d.bitcast_to_i32(d.load_bits(&bits[..D::LANES]))
}

/// One DequantDC row: `quant` rows (y, x, b order as `out`), `n` samples.
struct DequantRow<'a> {
    cfl: Option<[f32; 2]>,
    factors: [f32; 3],
    quant: [&'a [i32]; 3],
    out: [&'a mut [f32]; 3],
    n: usize,
    channels: usize,
}
impl Kernel for DequantRow<'_> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        let lanes = D::LANES;
        let full = self.n / lanes * lanes;
        let mut padded: [Vec<i32>; 3] = Default::default();
        for (p, q) in padded.iter_mut().zip(self.quant).take(self.channels) {
            *p = q[..self.n].to_vec();
            p.resize(full + 2 * lanes, 0);
        }
        let mut results: [Vec<f32>; 3] = Default::default();
        for r in results.iter_mut().take(self.channels) {
            *r = vec![0.0; full + lanes];
        }
        let mut x = 0;
        while x < self.n {
            let fac: [D::V; 3] = self.factors.map(|f| d.set(f));
            match self.cfl {
                Some([cfl_x, cfl_b]) => {
                    // Channel order of the source: y, x, b in `quant`.
                    let in_y = d.mul(d.convert_to_f32(load_i32(d, &padded[0][x..])), fac[0]);
                    let in_x = d.mul(d.convert_to_f32(load_i32(d, &padded[1][x..])), fac[1]);
                    let in_b = d.mul(d.convert_to_f32(load_i32(d, &padded[2][x..])), fac[2]);
                    d.store(in_y, &mut results[0][x..]);
                    d.store(d.mul_add(in_y, d.set(cfl_x), in_x), &mut results[1][x..]);
                    d.store(d.mul_add(in_y, d.set(cfl_b), in_b), &mut results[2][x..]);
                }
                None => {
                    let v = d.mul(d.convert_to_f32(load_i32(d, &padded[0][x..])), fac[0]);
                    d.store(v, &mut results[0][x..]);
                }
            }
            x += lanes;
        }
        for (out, r) in self.out.into_iter().zip(&results).take(self.channels) {
            out[..self.n].copy_from_slice(&r[..self.n]);
        }
    }
}

/// `DequantDC` instantiated for one SIMD target; the contract of
/// [`super::compressed_dc::dequant_dc`] for the rectangle's samples.
#[expect(
    clippy::too_many_arguments,
    reason = "Preserve the original DequantDC argument boundary."
)]
pub fn dequant_dc_simd(
    target: Target,
    r: Rect,
    dc: &mut Image3<f32>,
    quant_dc: &mut Plane<u8>,
    input: &Image,
    dc_factors: &[f32],
    multiplier: f32,
    cfl_factors: &[f32],
    chroma: &YCbCrChromaSubsampling,
    block_context: &BlockCtxMap,
) {
    if chroma.is_444() {
        // Scalar statements: dc_factors[c] * mul.
        let factors = [
            dc_factors[1] * multiplier,
            dc_factors[0] * multiplier,
            dc_factors[2] * multiplier,
        ];
        for y in 0..r.ysize() {
            let [p0, p1, p2] = dc.planes_mut();
            let x0 = r.x0();
            let n = r.xsize();
            dispatch(
                target,
                DequantRow {
                    cfl: Some([cfl_factors[0], cfl_factors[2]]),
                    factors,
                    quant: [
                        input.channel[0].plane.row(y),
                        input.channel[1].plane.row(y),
                        input.channel[2].plane.row(y),
                    ],
                    out: [
                        &mut p1.row_mut(r.y0() + y)[x0..x0 + n],
                        &mut p0.row_mut(r.y0() + y)[x0..x0 + n],
                        &mut p2.row_mut(r.y0() + y)[x0..x0 + n],
                    ],
                    n,
                    channels: 3,
                },
            );
        }
    } else {
        for c in [1, 0, 2] {
            let rect = Rect::new(
                r.x0() >> chroma.h_shift(c),
                r.y0() >> chroma.v_shift(c),
                r.xsize() >> chroma.h_shift(c),
                r.ysize() >> chroma.v_shift(c),
            );
            let factor = dc_factors[c] * multiplier;
            let ch = &input.channel[if c < 2 { c ^ 1 } else { c }];
            for y in 0..rect.ysize() {
                let n = rect.xsize();
                let row = &mut dc.plane_row_mut(c, rect.y0() + y)[rect.x0()..rect.x0() + n];
                dispatch(
                    target,
                    DequantRow {
                        cfl: None,
                        factors: [factor, 0.0, 0.0],
                        quant: [ch.plane.row(y), &[], &[]],
                        out: [row, &mut [], &mut []],
                        n,
                        channels: 1,
                    },
                );
            }
        }
    }
    super::compressed_dc::quant_dc_buckets(r, quant_dc, input, chroma, block_context);
}
