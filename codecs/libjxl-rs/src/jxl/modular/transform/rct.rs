// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Existing Highway scalar inverse RCT from modular/transform/rct.cc.
//! Rows run through RunOnPool: a null pool runs them in ascending task order;
//! a runner's tasks are serialized by `run_on_pool_exclusive`. SIMD targets
//! remain unported. Disjoint row borrows retain the source's
//! load-all-inputs-before-stores rule for in-place permutations.

use super::super::modular_image::{Channel, Image, Pixel};
use super::transform::check_equal_channels;
use crate::jxl::base::data_parallel::{ThreadPool, run_on_pool_exclusive_no_init};
use crate::jxl::base::status::{Status, StatusCode};

pub(super) trait Rows {
    fn get3(&self, x: usize) -> [Pixel; 3];
    fn set3(&mut self, x: usize, values: [Pixel; 3]);
}
pub(super) struct InPlaceRows<'a> {
    rows: [&'a mut [Pixel]; 3],
    input: [usize; 3],
    output: [usize; 3],
}
// Both accessors run once per sample; inline them into the row loop so the
// permuted row selection is not a call per sample (integer code only).
impl Rows for InPlaceRows<'_> {
    #[inline(always)]
    fn get3(&self, x: usize) -> [Pixel; 3] {
        [
            self.rows[self.input[0]][x],
            self.rows[self.input[1]][x],
            self.rows[self.input[2]][x],
        ]
    }
    #[inline(always)]
    fn set3(&mut self, x: usize, values: [Pixel; 3]) {
        self.rows[self.output[0]][x] = values[0];
        self.rows[self.output[1]][x] = values[1];
        self.rows[self.output[2]][x] = values[2];
    }
}
pub(super) fn image_rows(
    image: &mut Image,
    begin: usize,
    y: usize,
    input: [usize; 3],
    output: [usize; 3],
) -> InPlaceRows<'_> {
    let [a, b, c] = &mut image.channel[begin..begin + 3] else {
        unreachable!()
    };
    InPlaceRows {
        rows: [a.row_mut(y), b.row_mut(y), c.row_mut(y)],
        input,
        output,
    }
}
pub(super) struct SeparateRows<'a> {
    pub input: [&'a [Pixel]; 3],
    pub output: [&'a mut [Pixel]; 3],
}
impl Rows for SeparateRows<'_> {
    #[inline(always)]
    fn get3(&self, x: usize) -> [Pixel; 3] {
        [self.input[0][x], self.input[1][x], self.input[2][x]]
    }
    #[inline(always)]
    fn set3(&mut self, x: usize, values: [Pixel; 3]) {
        self.output[0][x] = values[0];
        self.output[1][x] = values[1];
        self.output[2][x] = values[2];
    }
}
fn inv_rct_row<const TRANSFORM: usize>(rows: &mut impl Rows, w: usize) {
    assert!(TRANSFORM < 7);
    let second = TRANSFORM >> 1;
    let third = TRANSFORM & 1;
    // The scalar Highway loop has one lane and consumes every valid sample;
    // its scalar tail is therefore empty. Highway Add/Sub wrap at 32 bits.
    for x in 0..w {
        let [mut first, mut second_value, mut third_value] = rows.get3(x);
        if TRANSFORM == 6 {
            first = first.wrapping_sub(third_value >> 1);
            let g = third_value.wrapping_add(first);
            first = first.wrapping_sub(second_value >> 1);
            let r = first.wrapping_add(second_value);
            rows.set3(x, [r, g, first]);
        } else {
            if third != 0 {
                third_value = third_value.wrapping_add(first);
            }
            if second == 1 {
                second_value = second_value.wrapping_add(first);
            } else if second == 2 {
                second_value = second_value.wrapping_add(first.wrapping_add(third_value) >> 1);
            }
            rows.set3(x, [first, second_value, third_value]);
        }
    }
}
fn inverse_rows<const TRANSFORM: usize>(
    image: &mut Image,
    begin: usize,
    dest: [usize; 3],
    w: usize,
    h: usize,
    pool: Option<&ThreadPool>,
) -> Status {
    let process_row = |image: &mut Image, task: u32, _thread: usize| -> Status {
        let y = task as usize;
        let mut rows = image_rows(image, begin, y, [0, 1, 2], dest);
        inv_rct_row::<TRANSFORM>(&mut rows, w);
        Ok(())
    };
    // RunOnPool narrows its range end to uint32_t in the source.
    // SAFETY: The row body borrows only the image's channel owners, which
    // hold no thread-affine values, and the image stays exclusively borrowed
    // by this call until it returns.
    unsafe { run_on_pool_exclusive_no_init(pool, 0, h as u32, image, process_row) }
}
fn permute_channels(channels: &mut [Channel], permutation: i32) {
    // Express the original three ownership moves with swaps; never allocate
    // replacement channels or clone planes to move out of a borrowed slice.
    match permutation {
        0 => {}
        1 => {
            channels.swap(0, 2);
            channels.swap(1, 2);
        }
        2 => {
            channels.swap(0, 1);
            channels.swap(1, 2);
        }
        3 => channels.swap(1, 2),
        4 => channels.swap(0, 1),
        5 => channels.swap(0, 2),
        _ => panic!("invalid source RCT permutation"),
    }
}
pub fn inv_rct(
    image: &mut Image,
    begin: usize,
    rct_type: usize,
    pool: Option<&ThreadPool>,
) -> Status {
    check_equal_channels(image, begin as u32, begin.wrapping_add(2) as u32)?;
    let w = image.channel[begin].w;
    let h = image.channel[begin].h;
    if rct_type == 0 {
        return Ok(());
    }
    let permutation = (rct_type / 7) as i32;
    if permutation >= 6 {
        return Err(StatusCode::GenericError);
    }
    let custom = rct_type % 7;
    if custom == 0 {
        permute_channels(&mut image.channel[begin..begin + 3], permutation);
        return Ok(());
    }
    let dest = [
        (permutation % 3) as usize,
        ((permutation + 1 + permutation / 3) % 3) as usize,
        ((permutation + 2 - permutation / 3) % 3) as usize,
    ];
    match custom {
        0 => inverse_rows::<0>(image, begin, dest, w, h, pool)?,
        1 => inverse_rows::<1>(image, begin, dest, w, h, pool)?,
        2 => inverse_rows::<2>(image, begin, dest, w, h, pool)?,
        3 => inverse_rows::<3>(image, begin, dest, w, h, pool)?,
        4 => inverse_rows::<4>(image, begin, dest, w, h, pool)?,
        5 => inverse_rows::<5>(image, begin, dest, w, h, pool)?,
        _ => inverse_rows::<6>(image, begin, dest, w, h, pool)?,
    }
    Ok(())
}
