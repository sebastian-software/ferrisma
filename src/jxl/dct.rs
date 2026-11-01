// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! One-lane Highway recursion and block scheduling from lib/jxl/dct-inl.h,
//! lib/jxl/dct_block-inl.h, and the scalar lib/jxl/transpose-inl.h branch.
//! Slice offsets express the original temporary pointers, including the in-place
//! recursive calls. Inputs, outputs, and scratch owners are disjoint as required
//! by the original restrict contract. Target-specific vector kernels are absent.
use super::dct_scales::{SQRT2, wc_multipliers};

// Highway scalar float multiplication widens to double, multiplies, then
// narrows. Its MulAdd/NegMulAdd subsequently add/subtract in binary32.
#[inline]
pub(crate) fn scalar_mul(a: f32, b: f32) -> f32 {
    (f64::from(a) * f64::from(b)) as f32
}

// The selected original AArch64 IDCT object's FADD places the addend before
// the product. Keep that observed competing-NaN operand order out of line.
#[inline(never)]
fn idct_add(addend: f32, product: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        addend + product
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        product + addend
    }
}

fn dct_1d(n: usize, mem: &mut [f32], at: usize, tmp: usize) {
    if n == 1 {
        return;
    }
    if n == 2 {
        let a = mem[at];
        let b = mem[at + 1];
        mem[at] = a + b;
        mem[at + 1] = a - b;
        return;
    }
    let half = n / 2;
    for i in 0..half {
        mem[tmp + i] = mem[at + i] + mem[at + n - i - 1];
    }
    dct_1d(half, mem, tmp, tmp + n);
    for i in 0..half {
        mem[tmp + half + i] = mem[at + i] - mem[at + n - i - 1];
    }
    for i in 0..half {
        mem[tmp + half + i] = scalar_mul(mem[tmp + half + i], wc_multipliers(n)[i]);
    }
    dct_1d(half, mem, tmp + half, tmp + n);
    mem[tmp + half] = scalar_mul(mem[tmp + half], SQRT2) + mem[tmp + half + 1];
    for i in 1..half - 1 {
        mem[tmp + half + i] += mem[tmp + half + i + 1];
    }
    for i in 0..half {
        mem[at + 2 * i] = mem[tmp + i];
    }
    for i in half..n {
        mem[at + 2 * (i - half) + 1] = mem[tmp + i];
    }
}

// Same storage is input and output in the original recursive subcalls. Keep
// those offsets in one exclusive slice instead of manufacturing aliasing borrows.
fn idct_1d_in_place(n: usize, mem: &mut [f32], at: usize, tmp: usize) {
    if n == 1 {
        return;
    }
    if n == 2 {
        let a = mem[at];
        let b = mem[at + 1];
        mem[at] = a + b;
        mem[at + 1] = a - b;
        return;
    }
    for i in 0..n / 2 {
        mem[tmp + i] = mem[at + 2 * i];
    }
    for i in n / 2..n {
        mem[tmp + i] = mem[at + 2 * (i - n / 2) + 1];
    }
    idct_finish(n, mem, tmp);
    for i in 0..n / 2 {
        let mul = wc_multipliers(n)[i];
        let a = mem[tmp + i];
        let b = mem[tmp + n / 2 + i];
        let product = scalar_mul(mul, b);
        mem[at + i] = idct_add(a, product);
        mem[at + n - i - 1] = a - product;
    }
}
fn idct_finish(n: usize, tmp: &mut [f32], at: usize) {
    let half = n / 2;
    idct_1d_in_place(half, tmp, at, at + n);
    for i in (1..half).rev() {
        tmp[at + half + i] += tmp[at + half + i - 1];
    }
    tmp[at + half] = scalar_mul(tmp[at + half], SQRT2);
    idct_1d_in_place(half, tmp, at + half, at + n);
}
fn idct_1d(n: usize, from: &[f32], fs: usize, to: &mut [f32], ts: usize, tmp: &mut [f32]) {
    if n == 1 {
        to[0] = from[0];
        return;
    }
    if n == 2 {
        let a = from[0];
        let b = from[fs];
        to[0] = a + b;
        to[ts] = a - b;
        return;
    }
    for i in 0..n / 2 {
        tmp[i] = from[2 * i * fs];
    }
    for i in n / 2..n {
        tmp[i] = from[(2 * (i - n / 2) + 1) * fs];
    }
    idct_finish(n, tmp, 0);
    for i in 0..n / 2 {
        let mul = wc_multipliers(n)[i];
        let a = tmp[i];
        let b = tmp[n / 2 + i];
        let product = scalar_mul(mul, b);
        to[i * ts] = idct_add(a, product);
        to[(n - i - 1) * ts] = a - product;
    }
}
fn dct_columns(
    n: usize,
    m: usize,
    from: &[f32],
    fs: usize,
    to: &mut [f32],
    ts: usize,
    tmp: &mut [f32],
) {
    for x in 0..m {
        for y in 0..n {
            tmp[y] = from[y * fs + x];
        }
        dct_1d(n, tmp, 0, n);
        let mul = 1.0f32 / n as f32;
        for y in 0..n {
            // The original N=1 specialization folds its constant *1 to a
            // raw copy in both selected objects, retaining signaling bits.
            to[y * ts + x] = if n == 1 {
                tmp[y]
            } else {
                scalar_mul(mul, tmp[y])
            };
        }
    }
}
fn idct_columns(
    n: usize,
    m: usize,
    from: &[f32],
    fs: usize,
    to: &mut [f32],
    ts: usize,
    tmp: &mut [f32],
) {
    for x in 0..m {
        idct_1d(n, &from[x..], fs, &mut to[x..], ts, tmp);
    }
}
fn transpose(rows: usize, cols: usize, from: &[f32], fs: usize, to: &mut [f32], ts: usize) {
    for y in 0..rows {
        for x in 0..cols {
            to[x * ts + y] = from[y * fs + x];
        }
    }
}
fn dimensions(rows: usize, cols: usize) {
    assert!(rows.is_power_of_two() && cols.is_power_of_two() && rows <= 256 && cols <= 256);
}

/// Original scaled DCT with its maybe-transposed coefficient layout. Scratch
/// must hold rows*cols block values followed by at least 3*max(rows,cols) values.
/// The source's valid row width and disjoint-owner preconditions are retained.
pub fn compute_scaled_dct(
    rows: usize,
    cols: usize,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    scratch: &mut [f32],
) {
    dimensions(rows, cols);
    assert!(from_stride >= cols);
    let (block, tmp) = scratch.split_at_mut(rows * cols);
    if rows < cols {
        dct_columns(rows, cols, from, from_stride, block, cols, tmp);
        transpose(rows, cols, block, cols, to, rows);
        dct_columns(cols, rows, to, rows, block, rows, tmp);
        transpose(cols, rows, block, rows, to, cols);
    } else {
        dct_columns(rows, cols, from, from_stride, to, cols, tmp);
        transpose(rows, cols, to, cols, block, rows);
        dct_columns(cols, rows, block, rows, to, rows, tmp);
    }
}

/// Original inverse, including mutation of the coefficient block. Output row
/// padding and unused scratch values are preserved. Scratch has the same block
/// prefix and at least 2*max(rows,cols) trailing values as the original caller.
pub fn compute_scaled_idct(
    rows: usize,
    cols: usize,
    from: &mut [f32],
    to: &mut [f32],
    to_stride: usize,
    scratch: &mut [f32],
) {
    dimensions(rows, cols);
    assert!(to_stride >= cols);
    let (block, tmp) = scratch.split_at_mut(rows * cols);
    if rows < cols {
        transpose(rows, cols, from, cols, block, rows);
        idct_columns(cols, rows, block, rows, from, rows, tmp);
        transpose(cols, rows, from, rows, block, cols);
        idct_columns(rows, cols, block, cols, to, to_stride, tmp);
    } else {
        idct_columns(cols, rows, from, rows, block, rows, tmp);
        transpose(cols, rows, block, rows, from, cols);
        idct_columns(rows, cols, from, cols, to, to_stride, tmp);
    }
}
