// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The Highway `FwdRctRow` of modular/transform/enc_rct.cc for the NEON and
//! AVX2 targets (port ADR 0004, "Encoder path"), written once against
//! [`SimdF32`]'s signed lanes (`HWY_FULL(int32_t)` has the float vector's
//! lane count). `enc_rct.rs` keeps the scalar (`HWY_SCALAR`) translation and
//! calls [`fwd_rct_simd`] / [`fwd_rct_channels_simd`] when the encoder
//! selected a SIMD target, as `HWY_DYNAMIC_DISPATCH(FwdRctImpl)` does.
//!
//! The vector loop is the source's `for (x = 0; x < w; x += Lanes(di))`
//! without a scalar tail: the last step loads and stores a whole vector, so
//! the samples up to `RoundUpTo(w, Lanes)` are read and written inside the
//! rows' padding (a channel row is at least `RoundUpTo(w, 32)` samples, 128
//! bytes, long). `Add`/`Sub` wrap, `ShiftRight` is arithmetic. Within one
//! vector step all three loads precede the stores, and steps cover disjoint
//! samples, so the in-place permutations of `FwdRct(Image&, ...)` read every
//! input before it is overwritten, as in the source.

use super::super::modular_image::{Channel, Image, Pixel};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};

/// The rows of one `FwdRctRow` call: `load(k)` is `ink + x`, `store(k)` is
/// `outk + x`.
trait RctRows {
    fn load<D: SimdF32>(&self, d: D, k: usize, x: usize) -> D::I;
    fn store<D: SimdF32>(&mut self, d: D, v: D::I, k: usize, x: usize);
}

/// `FwdRct(Image&, ...)`: the inputs are the permuted output channels.
struct InPlace<'a> {
    rows: [&'a mut [Pixel]; 3],
    input: [usize; 3],
}
impl RctRows for InPlace<'_> {
    #[inline(always)]
    fn load<D: SimdF32>(&self, d: D, k: usize, x: usize) -> D::I {
        d.load_i32(&self.rows[self.input[k]][x..])
    }
    #[inline(always)]
    fn store<D: SimdF32>(&mut self, d: D, v: D::I, k: usize, x: usize) {
        d.store_i32(v, &mut self.rows[k][x..]);
    }
}

/// `FwdRct(in, out, ...)` with disjoint input and output channels.
struct Separate<'a> {
    input: [&'a [Pixel]; 3],
    output: [&'a mut [Pixel]; 3],
}
impl RctRows for Separate<'_> {
    #[inline(always)]
    fn load<D: SimdF32>(&self, d: D, k: usize, x: usize) -> D::I {
        d.load_i32(&self.input[k][x..])
    }
    #[inline(always)]
    fn store<D: SimdF32>(&mut self, d: D, v: D::I, k: usize, x: usize) {
        d.store_i32(v, &mut self.output[k][x..]);
    }
}

struct FwdRctRow<R, const TRANSFORM: usize> {
    w: usize,
    rows: R,
}
impl<R: RctRows, const TRANSFORM: usize> Kernel for FwdRctRow<R, TRANSFORM> {
    type Output = bool;
    #[inline(always)]
    fn run<D: SimdF32>(self, di: D) -> bool {
        assert!(TRANSFORM <= 6, "Invalid RCT transform");
        let Self { w, mut rows } = self;
        let second = TRANSFORM >> 1;
        let third = TRANSFORM & 1;

        if TRANSFORM == 6 {
            let mut x = 0;
            while x < w {
                let r = rows.load(di, 0, x);
                let g = rows.load(di, 1, x);
                let b = rows.load(di, 2, x);
                let o1 = di.sub_i32(r, b);
                let tmp = di.add_i32(b, di.shift_right_i32::<1>(o1));
                let o2 = di.sub_i32(g, tmp);
                let o0 = di.add_i32(tmp, di.shift_right_i32::<1>(o2));
                rows.store(di, o0, 0, x);
                rows.store(di, o1, 1, x);
                rows.store(di, o2, 2, x);
                x += D::LANES;
            }
            return true;
        }

        let mut x = 0;
        while x < w {
            let i0 = rows.load(di, 0, x);
            let i1 = rows.load(di, 1, x);
            let i2 = rows.load(di, 2, x);
            let mut o1 = i1;
            if second == 1 {
                o1 = di.sub_i32(o1, i0);
            } else if second == 2 {
                o1 = di.sub_i32(o1, di.shift_right_i32::<1>(di.add_i32(i0, i2)));
            }
            let mut o2 = i2;
            if third != 0 {
                o2 = di.sub_i32(o2, i0);
            }
            rows.store(di, i0, 0, x);
            rows.store(di, o1, 1, x);
            rows.store(di, o2, 2, x);
            x += D::LANES;
        }
        true
    }
}

/// `RctPermute`'s channel indices.
fn permutation_indices(permutation: usize) -> [usize; 3] {
    [
        permutation % 3,
        (permutation + 1 + permutation / 3) % 3,
        (permutation + 2 - permutation / 3) % 3,
    ]
}

fn row<R: RctRows>(target: Target, transform: usize, w: usize, rows: R) -> bool {
    match transform {
        0 => dispatch(target, FwdRctRow::<R, 0> { w, rows }),
        1 => dispatch(target, FwdRctRow::<R, 1> { w, rows }),
        2 => dispatch(target, FwdRctRow::<R, 2> { w, rows }),
        3 => dispatch(target, FwdRctRow::<R, 3> { w, rows }),
        4 => dispatch(target, FwdRctRow::<R, 4> { w, rows }),
        5 => dispatch(target, FwdRctRow::<R, 5> { w, rows }),
        _ => dispatch(target, FwdRctRow::<R, 6> { w, rows }),
    }
}

/// `FwdRctImpl` at a SIMD target for disjoint channels (`FwdRct(in, out,
/// rct_type, nullptr)`); `w` and `h` are those of `output[0]`. Only the
/// null-pool path is translated, as in `enc_rct.rs`.
pub fn fwd_rct_channels_simd(
    target: Target,
    input: [&Channel; 3],
    output: [&mut Channel; 3],
    rct_type: usize,
) -> Status {
    let indices = permutation_indices(rct_type / 7);
    let input = [input[indices[0]], input[indices[1]], input[indices[2]]];
    let transform = rct_type % 7;
    let w = output[0].w;
    let h = output[0].h;
    let [a, b, c] = output;
    for task in 0..h as u32 {
        // The source callback takes int, then Row converts it to size_t.
        let y = (task as i32) as usize;
        let rows = Separate {
            input: [input[0].row(y), input[1].row(y), input[2].row(y)],
            output: [a.row_mut(y), b.row_mut(y), c.row_mut(y)],
        };
        if !row(target, transform, w, rows) {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}

/// `FwdRctImpl` at a SIMD target as `FwdRct(Image&, begin_c, rct_type,
/// nullptr)` calls it, after its channel checks: the inputs are the
/// permuted channels `begin_c..begin_c + 3`, the outputs those channels.
pub fn fwd_rct_simd(target: Target, image: &mut Image, begin: usize, rct_type: usize) -> Status {
    let indices = permutation_indices(rct_type / 7);
    let transform = rct_type % 7;
    let w = image.channel[begin].w;
    let h = image.channel[begin].h;
    for task in 0..h as u32 {
        let y = (task as i32) as usize;
        let [a, b, c] = &mut image.channel[begin..begin + 3] else {
            unreachable!()
        };
        let rows = InPlace {
            rows: [a.row_mut(y), b.row_mut(y), c.row_mut(y)],
            input: indices,
        };
        if !row(target, transform, w, rows) {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}
