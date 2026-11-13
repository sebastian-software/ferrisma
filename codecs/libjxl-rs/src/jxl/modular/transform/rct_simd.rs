// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The Highway `InvRCTRow` of modular/transform/rct.cc for the NEON and AVX2
//! targets (port ADR 0004), written once against [`SimdF32`]'s signed lanes
//! (`HWY_FULL(pixel_type)` has the float vector's lanes). `rct.rs` keeps the
//! scalar (`HWY_SCALAR`) translation and its permutation and calls
//! [`inverse_rows_simd`] when the decoder selected a SIMD target, as
//! `HWY_DYNAMIC_DISPATCH(InvRCT)` does.
//!
//! The vector loop runs while `x + N - 1 < w`; the remaining samples take
//! the source's scalar tail with `PixelAdd` (wrapping 32-bit addition).
//! Vector `Add`/`Sub` wrap as well. Within one vector step all three loads
//! precede the stores, and steps cover disjoint samples, so the in-place
//! permutations read every input before it is overwritten, as in the source.

use super::super::modular_image::{Image, Pixel};
use crate::jxl::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use crate::jxl::base::status::Status;
use crate::jxl::hwy::{Kernel, SimdF32, Target, dispatch};

struct InvRctRow<'a, const TRANSFORM: usize> {
    rows: [&'a mut [Pixel]; 3],
    dest: [usize; 3],
    w: usize,
}
impl<const TRANSFORM: usize> Kernel for InvRctRow<'_, TRANSFORM> {
    type Output = ();
    #[inline(always)]
    fn run<D: SimdF32>(self, d: D) {
        assert!(TRANSFORM < 7, "Invalid transform type");
        let Self { rows, dest, w } = self;
        let second = TRANSFORM >> 1;
        let third = TRANSFORM & 1;
        let n = D::LANES;
        let mut x = 0;
        while x + n - 1 < w {
            if TRANSFORM == 6 {
                let mut y = d.load_i32(&rows[0][x..]);
                let co = d.load_i32(&rows[1][x..]);
                let cg = d.load_i32(&rows[2][x..]);
                y = d.sub_i32(y, d.shift_right_i32::<1>(cg));
                let g = d.add_i32(cg, y);
                y = d.sub_i32(y, d.shift_right_i32::<1>(co));
                let r = d.add_i32(y, co);
                d.store_i32(r, &mut rows[dest[0]][x..]);
                d.store_i32(g, &mut rows[dest[1]][x..]);
                d.store_i32(y, &mut rows[dest[2]][x..]);
            } else {
                let first = d.load_i32(&rows[0][x..]);
                let mut second_v = d.load_i32(&rows[1][x..]);
                let mut third_v = d.load_i32(&rows[2][x..]);
                if third != 0 {
                    third_v = d.add_i32(third_v, first);
                }
                if second == 1 {
                    second_v = d.add_i32(second_v, first);
                } else if second == 2 {
                    second_v =
                        d.add_i32(second_v, d.shift_right_i32::<1>(d.add_i32(first, third_v)));
                }
                d.store_i32(first, &mut rows[dest[0]][x..]);
                d.store_i32(second_v, &mut rows[dest[1]][x..]);
                d.store_i32(third_v, &mut rows[dest[2]][x..]);
            }
            x += n;
        }
        while x < w {
            let y = rows[0][x];
            let co = rows[1][x];
            let cg = rows[2][x];
            let values = if TRANSFORM == 6 {
                // PixelAdd(Y, -(Cg >> 1)) and its siblings: wrapping adds.
                let tmp = y.wrapping_add((cg >> 1).wrapping_neg());
                let g = cg.wrapping_add(tmp);
                let b = tmp.wrapping_add((co >> 1).wrapping_neg());
                let r = b.wrapping_add(co);
                [r, g, b]
            } else {
                let (first, mut second_v, mut third_v) = (y, co, cg);
                if third != 0 {
                    third_v = third_v.wrapping_add(first);
                }
                if second == 1 {
                    second_v = second_v.wrapping_add(first);
                } else if second == 2 {
                    second_v = second_v.wrapping_add(first.wrapping_add(third_v) >> 1);
                }
                [first, second_v, third_v]
            };
            rows[dest[0]][x] = values[0];
            rows[dest[1]][x] = values[1];
            rows[dest[2]][x] = values[2];
            x += 1;
        }
    }
}

fn row<const TRANSFORM: usize>(
    target: Target,
    image: &mut Image,
    begin: usize,
    dest: [usize; 3],
    w: usize,
    y: usize,
) {
    let [a, b, c] = &mut image.channel[begin..begin + 3] else {
        unreachable!()
    };
    // Rows of the three channels in source order; `dest` names the row each
    // result is stored to (the output channel pointers of the source).
    let rows = [a.row_mut(y), b.row_mut(y), c.row_mut(y)];
    dispatch(target, InvRctRow::<TRANSFORM> { rows, dest, w });
}

/// InvRCT's row loop at a SIMD target: `custom` is `rct_type % 7` (1-6),
/// `dest` the source's output channel offsets for the permutation.
#[expect(
    clippy::too_many_arguments,
    reason = "the source InvRCT row loop state plus the dispatch target"
)]
pub fn inverse_rows_simd(
    target: Target,
    image: &mut Image,
    begin: usize,
    custom: usize,
    dest: [usize; 3],
    w: usize,
    h: usize,
    pool: Option<&ThreadPool>,
) -> Status {
    let process_row = |image: &mut Image, task: u32, _thread: usize| -> Status {
        let y = task as usize;
        match custom {
            0 => row::<0>(target, image, begin, dest, w, y),
            1 => row::<1>(target, image, begin, dest, w, y),
            2 => row::<2>(target, image, begin, dest, w, y),
            3 => row::<3>(target, image, begin, dest, w, y),
            4 => row::<4>(target, image, begin, dest, w, y),
            5 => row::<5>(target, image, begin, dest, w, y),
            _ => row::<6>(target, image, begin, dest, w, y),
        }
        Ok(())
    };
    // RunOnPool narrows its range end to uint32_t in the source.
    // SAFETY: The row body borrows only the image's channel owners, which
    // hold no thread-affine values, and the image stays exclusively borrowed
    // by this call until it returns.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, h as u32, image, process_row) }
}
