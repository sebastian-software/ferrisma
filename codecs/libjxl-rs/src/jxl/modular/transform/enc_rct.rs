// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Existing scalar forward RCT from modular/transform/enc_rct.cc. Separate
//! Rust entry points retain the source's in-place and disjoint channel calls.
//! Only the original null-pool path is translated; custom runners/SIMD remain
//! unported. Source scalar forward arithmetic requires nonoverflowing i32
//! intermediates, unlike the wrapping Highway operations in the inverse.

use super::super::modular_image::{Channel, Image};
use super::rct::{Rows, SeparateRows, image_rows};
use super::transform::check_equal_channels;
use crate::jxl::base::status::{Status, StatusCode};

fn fwd_rct_row<const TRANSFORM: usize>(w: usize, rows: &mut impl Rows) -> bool {
    assert!(TRANSFORM <= 6);
    let second = TRANSFORM >> 1;
    let third = TRANSFORM & 1;
    if TRANSFORM == 6 {
        for x in 0..w {
            let [r, g, b] = rows.get3(x);
            let o1 = r - b;
            let tmp = b + (o1 >> 1);
            let o2 = g - tmp;
            rows.set3(x, [tmp + (o2 >> 1), o1, o2]);
        }
        return true;
    }
    for x in 0..w {
        let [first, mut second_value, mut third_value] = rows.get3(x);
        if second == 1 {
            second_value -= first;
        } else if second == 2 {
            second_value -= (first + third_value) >> 1;
        }
        if third != 0 {
            third_value -= first;
        }
        rows.set3(x, [first, second_value, third_value]);
    }
    true
}
fn permutation_indices(permutation: usize) -> [usize; 3] {
    [
        permutation % 3,
        (permutation + 1 + permutation / 3) % 3,
        (permutation + 2 - permutation / 3) % 3,
    ]
}
fn in_place<const TRANSFORM: usize>(
    image: &mut Image,
    begin: usize,
    indices: [usize; 3],
    w: usize,
    h: usize,
) -> Status {
    for task in 0..h as u32 {
        // The forward source callback takes int, then Row converts it to size_t.
        let y = (task as i32) as usize;
        let mut rows = image_rows(image, begin, y, indices, [0, 1, 2]);
        if !fwd_rct_row::<TRANSFORM>(w, &mut rows) {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}
fn separate<const TRANSFORM: usize>(input: [&Channel; 3], output: [&mut Channel; 3]) -> Status {
    let w = output[0].w;
    let h = output[0].h;
    let [a, b, c] = output;
    for task in 0..h as u32 {
        let y = (task as i32) as usize;
        let mut rows = SeparateRows {
            input: [input[0].row(y), input[1].row(y), input[2].row(y)],
            output: [a.row_mut(y), b.row_mut(y), c.row_mut(y)],
        };
        if !fwd_rct_row::<TRANSFORM>(w, &mut rows) {
            return Err(StatusCode::GenericError);
        }
    }
    Ok(())
}
pub fn fwd_rct_channels(
    input: [&Channel; 3],
    output: [&mut Channel; 3],
    rct_type: usize,
) -> Status {
    let indices = permutation_indices(rct_type / 7);
    let input = [input[indices[0]], input[indices[1]], input[indices[2]]];
    match rct_type % 7 {
        0 => separate::<0>(input, output),
        1 => separate::<1>(input, output),
        2 => separate::<2>(input, output),
        3 => separate::<3>(input, output),
        4 => separate::<4>(input, output),
        5 => separate::<5>(input, output),
        _ => separate::<6>(input, output),
    }
}
pub fn fwd_rct(image: &mut Image, begin: usize, rct_type: usize) -> Status {
    check_equal_channels(image, begin as u32, begin.wrapping_add(2) as u32)?;
    if rct_type == 0 {
        return Err(StatusCode::GenericError);
    }
    let indices = permutation_indices(rct_type / 7);
    let w = image.channel[begin].w;
    let h = image.channel[begin].h;
    match rct_type % 7 {
        0 => in_place::<0>(image, begin, indices, w, h),
        1 => in_place::<1>(image, begin, indices, w, h),
        2 => in_place::<2>(image, begin, indices, w, h),
        3 => in_place::<3>(image, begin, indices, w, h),
        4 => in_place::<4>(image, begin, indices, w, h),
        5 => in_place::<5>(image, begin, indices, w, h),
        _ => in_place::<6>(image, begin, indices, w, h),
    }
}
