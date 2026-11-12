// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! One-lane Highway recursion and block scheduling from lib/jxl/dct-inl.h,
//! lib/jxl/dct_block-inl.h, and the scalar lib/jxl/transpose-inl.h branch.
//! Slice splits express the original temporary pointers, including the in-place
//! recursive calls. Inputs, outputs, and scratch owners are disjoint as required
//! by the original restrict contract. Target-specific vector kernels are absent.
//! As in the original's DCT1DImpl<N, 1>/IDCT1DImpl<N, 1> templates and its
//! ComputeScaled(I)DCT<ROWS, COLS> instantiations, every transform size and
//! block shape is a separate instantiation with constant loop bounds, strides,
//! and multiplier tables; one dispatch per block replaces the former
//! runtime-size recursion. Operations, operands, and scratch writes are
//! unchanged.
//!
//! Inlining policy. Without fast-math, LLVM may still swap the operands of a
//! commutative addition (an unrolled four-point column showed `fadd %b, %a`
//! for the source's `a + b`), and AArch64 FADD returns its first operand's
//! NaN when both inputs are NaN. Which levels are inlined therefore decides
//! competing NaN payloads. The out-of-line boundaries below (`idct_add`,
//! `idct2_column`, `prefix_sum_backward`, and every forward DCT level) keep
//! the former recursion's payload selection: every ordered pair of competing
//! payloads over all shapes of up to 256 values, the LowestFrequencies blocks
//! of up to 64 blocks and the small strategies, and a randomized corpus over
//! every shape match the former implementation word for word in the port's
//! scalar profile and in the root release profile (2026-11-12). Inlining the
//! two-point DCT together with the four-point level, the four-point DCT level
//! alone (2x4 shape), the eight-point DCT level, or the unrolled IDCT prefix
//! sum from 16 points up changed selected payloads. Re-check this mapping
//! after compiler or toolchain changes.
//!
//! This mapping is established for AArch64 only. Other targets keep the
//! former runtime-size recursion unchanged: an x86_64 build of the
//! instantiated form, run under Rosetta 2, changed selected payloads against
//! the former x86_64 build.
use super::dct_scales::{SQRT2, wc_multipliers};

// Highway scalar float multiplication widens to double, multiplies, then
// narrows. Its MulAdd/NegMulAdd subsequently add/subtract in binary32.
#[inline]
pub(crate) fn scalar_mul(a: f32, b: f32) -> f32 {
    (f64::from(a) * f64::from(b)) as f32
}

// The selected original IDCT objects retain the addend's payload when it
// competes with the product. Keep that operand order out of line in both the
// native AArch64 and x86_64 configurations.
#[inline(never)]
fn idct_add(addend: f32, product: f32) -> f32 {
    addend + product
}

#[cfg(target_arch = "aarch64")]
mod instantiated {
    use super::*;

    // IDCT1DImpl<2, 1> on one strided column, out of line for its payload order.
    #[inline(never)]
    fn idct2_column(from: &[f32], x: usize, from_stride: usize, to: &mut [f32], to_stride: usize) {
        let a = from[x];
        let b = from[x + from_stride];
        to[x] = a + b;
        to[x + to_stride] = a - b;
    }

    // The BTranspose running sum of the IDCT's odd half, kept as a rolled loop for
    // its payload order once the half has eight or more values.
    #[inline(never)]
    fn prefix_sum_backward(v: &mut [f32]) {
        for i in (1..v.len()).rev() {
            v[i] += v[i - 1];
        }
    }

    /// One transform size. Each method re-borrows its slices at their fixed
    /// lengths, so constant indices need no runtime checks. Arithmetic operations,
    /// operands, and scratch write positions are those of the original recursion.
    trait Transform1d {
        const N: usize;
        /// DCT1DImpl<N, 1>(mem, tmp): `mem` holds N values, `scratch` is `tmp`.
        fn dct(mem: &mut [f32], scratch: &mut [f32]);
        /// IDCT1DImpl<N, 1> with `from == to` and unit strides, as in the
        /// original's in-place recursive subcalls; `scratch` is `tmp`.
        fn idct_in_place(mem: &mut [f32], scratch: &mut [f32]);
        /// The recursion between the even/odd gather and the final butterflies:
        /// `tmp` holds the N gathered values and `scratch` is `tmp + N`.
        fn idct_finish(tmp: &mut [f32], scratch: &mut [f32]);
    }

    struct Size1;
    struct Size2;

    impl Transform1d for Size1 {
        const N: usize = 1;
        #[inline(always)]
        fn dct(_mem: &mut [f32], _scratch: &mut [f32]) {}
        #[inline(always)]
        fn idct_in_place(_mem: &mut [f32], _scratch: &mut [f32]) {}
        fn idct_finish(_tmp: &mut [f32], _scratch: &mut [f32]) {
            unreachable!("IDCT1DImpl<1> has no recursion body")
        }
    }

    impl Transform1d for Size2 {
        const N: usize = 2;
        // Out of line for its payload order (module note).
        #[inline(never)]
        fn dct(mem: &mut [f32], _scratch: &mut [f32]) {
            let mem = &mut mem[..2];
            let a = mem[0];
            let b = mem[1];
            mem[0] = a + b;
            mem[1] = a - b;
        }
        #[inline(always)]
        fn idct_in_place(mem: &mut [f32], _scratch: &mut [f32]) {
            let mem = &mut mem[..2];
            let a = mem[0];
            let b = mem[1];
            mem[0] = a + b;
            mem[1] = a - b;
        }
        fn idct_finish(_tmp: &mut [f32], _scratch: &mut [f32]) {
            unreachable!("IDCT1DImpl<2> has no recursion body")
        }
    }

    macro_rules! recursive_transform {
        ($name:ident, $n:literal, $half:ident, $dct:meta, $idct:meta, $finish:meta) => {
            struct $name;
            impl Transform1d for $name {
                const N: usize = $n;
                #[$dct]
                fn dct(mem: &mut [f32], scratch: &mut [f32]) {
                    const N: usize = $n;
                    const HALF: usize = N / 2;
                    let mem = &mut mem[..N];
                    let (tmp, rest) = scratch.split_at_mut(N);
                    let wc = &wc_multipliers(N)[..HALF];
                    for i in 0..HALF {
                        tmp[i] = mem[i] + mem[N - i - 1];
                    }
                    $half::dct(&mut tmp[..HALF], rest);
                    for i in 0..HALF {
                        tmp[HALF + i] = mem[i] - mem[N - i - 1];
                    }
                    for i in 0..HALF {
                        tmp[HALF + i] = scalar_mul(tmp[HALF + i], wc[i]);
                    }
                    $half::dct(&mut tmp[HALF..], rest);
                    tmp[HALF] = scalar_mul(tmp[HALF], SQRT2) + tmp[HALF + 1];
                    // Empty for the four-point level, as CoeffBundle<2, 1>::B.
                    #[allow(clippy::reversed_empty_ranges)]
                    for i in 1..HALF - 1 {
                        tmp[HALF + i] += tmp[HALF + i + 1];
                    }
                    for i in 0..HALF {
                        mem[2 * i] = tmp[i];
                    }
                    for i in HALF..N {
                        mem[2 * (i - HALF) + 1] = tmp[i];
                    }
                }
                #[$idct]
                fn idct_in_place(mem: &mut [f32], scratch: &mut [f32]) {
                    const N: usize = $n;
                    const HALF: usize = N / 2;
                    let mem = &mut mem[..N];
                    let (tmp, rest) = scratch.split_at_mut(N);
                    for i in 0..HALF {
                        tmp[i] = mem[2 * i];
                    }
                    for i in HALF..N {
                        tmp[i] = mem[2 * (i - HALF) + 1];
                    }
                    Self::idct_finish(tmp, rest);
                    let wc = &wc_multipliers(N)[..HALF];
                    for i in 0..HALF {
                        let a = tmp[i];
                        let b = tmp[HALF + i];
                        let product = scalar_mul(wc[i], b);
                        mem[i] = idct_add(a, product);
                        mem[N - i - 1] = a - product;
                    }
                }
                #[$finish]
                fn idct_finish(tmp: &mut [f32], scratch: &mut [f32]) {
                    const N: usize = $n;
                    const HALF: usize = N / 2;
                    let tmp = &mut tmp[..N];
                    $half::idct_in_place(&mut tmp[..HALF], scratch);
                    if N >= 16 {
                        prefix_sum_backward(&mut tmp[HALF..]);
                    } else {
                        for i in (1..HALF).rev() {
                            tmp[HALF + i] += tmp[HALF + i - 1];
                        }
                    }
                    tmp[HALF] = scalar_mul(tmp[HALF], SQRT2);
                    $half::idct_in_place(&mut tmp[HALF..], scratch);
                }
            }
        };
    }

    // Attributes: DCT level, in-place IDCT level, IDCT recursion (see the module
    // note on payload order).
    recursive_transform!(
        Size4,
        4,
        Size2,
        inline(never),
        inline(always),
        inline(always)
    );
    recursive_transform!(
        Size8,
        8,
        Size4,
        inline(never),
        inline(always),
        inline(always)
    );
    recursive_transform!(
        Size16,
        16,
        Size8,
        inline(never),
        inline(always),
        inline(always)
    );
    recursive_transform!(
        Size32,
        32,
        Size16,
        inline(never),
        inline(always),
        inline(always)
    );
    recursive_transform!(
        Size64,
        64,
        Size32,
        inline(never),
        inline(never),
        inline(never)
    );
    recursive_transform!(
        Size128,
        128,
        Size64,
        inline(never),
        inline(never),
        inline(never)
    );
    recursive_transform!(
        Size256,
        256,
        Size128,
        inline(never),
        inline(never),
        inline(never)
    );

    // DCT1DWrapper<N, 1> over the M columns of a block: gather a column into tmp,
    // transform it with tmp + N as scratch, then scale (raw copy for N = 1) into
    // the target. The target stride is always M; the source stride is M except
    // for the caller's pixel rows.
    #[inline(always)]
    fn dct_columns<S: Transform1d, M: Transform1d, const FROM_STRIDE_IS_M: bool>(
        from: &[f32],
        from_stride: usize,
        to: &mut [f32],
        tmp: &mut [f32],
    ) {
        let n = S::N;
        let m = M::N;
        let fs = if FROM_STRIDE_IS_M { m } else { from_stride };
        let from = if FROM_STRIDE_IS_M {
            &from[..n * m]
        } else {
            from
        };
        let to = &mut to[..n * m];
        let (col, scratch) = tmp[..3 * n].split_at_mut(n);
        let mul = 1.0f32 / n as f32;
        for x in 0..m {
            for y in 0..n {
                col[y] = from[y * fs + x];
            }
            S::dct(col, scratch);
            for y in 0..n {
                // The original N=1 specialization folds its constant *1 to a
                // raw copy in both selected objects, retaining signaling bits.
                to[y * m + x] = if n == 1 {
                    col[y]
                } else {
                    scalar_mul(mul, col[y])
                };
            }
        }
    }

    // IDCT1DWrapper<N, 1> over the M columns of a block with the original strided
    // even/odd gather and MultiplyAndAdd scatter; N = 1 and N = 2 are the
    // original's direct specializations. The source stride is always M; the target
    // stride is M except for the caller's pixel rows.
    #[inline(always)]
    fn idct_columns<S: Transform1d, M: Transform1d, const TO_STRIDE_IS_M: bool>(
        from: &[f32],
        to: &mut [f32],
        to_stride: usize,
        tmp: &mut [f32],
    ) {
        let n = S::N;
        let m = M::N;
        let half = n / 2;
        let from = &from[..n * m];
        let ts = if TO_STRIDE_IS_M { m } else { to_stride };
        let to = if TO_STRIDE_IS_M { &mut to[..n * m] } else { to };
        let (col, scratch) = tmp[..2 * n].split_at_mut(n);
        for x in 0..m {
            if n == 1 {
                to[x] = from[x];
                continue;
            }
            if n == 2 {
                idct2_column(from, x, m, to, ts);
                continue;
            }
            for i in 0..half {
                col[i] = from[x + 2 * i * m];
            }
            for i in half..n {
                col[i] = from[x + (2 * (i - half) + 1) * m];
            }
            S::idct_finish(col, scratch);
            let wc = &wc_multipliers(n)[..half];
            for i in 0..half {
                let a = col[i];
                let b = col[half + i];
                let product = scalar_mul(wc[i], b);
                to[x + i * ts] = idct_add(a, product);
                to[x + (n - i - 1) * ts] = a - product;
            }
        }
    }

    // Scalar GenericTransposeBlock of a ROWS x COLS block with dense rows.
    #[inline(always)]
    fn transpose<R: Transform1d, C: Transform1d>(from: &[f32], to: &mut [f32]) {
        let (rows, cols) = (R::N, C::N);
        let from = &from[..rows * cols];
        let to = &mut to[..rows * cols];
        for y in 0..rows {
            for x in 0..cols {
                to[x * rows + y] = from[y * cols + x];
            }
        }
    }

    // ComputeScaledDCT<ROWS, COLS>: one instantiation per block shape.
    #[inline(never)]
    fn scaled_dct<R: Transform1d, C: Transform1d>(
        from: &[f32],
        from_stride: usize,
        to: &mut [f32],
        scratch: &mut [f32],
    ) {
        let (block, tmp) = scratch.split_at_mut(R::N * C::N);
        if R::N < C::N {
            dct_columns::<R, C, false>(from, from_stride, block, tmp);
            transpose::<R, C>(block, to);
            dct_columns::<C, R, true>(to, 0, block, tmp);
            transpose::<C, R>(block, to);
        } else {
            dct_columns::<R, C, false>(from, from_stride, to, tmp);
            transpose::<R, C>(to, block);
            dct_columns::<C, R, true>(block, 0, to, tmp);
        }
    }

    // ComputeScaledIDCT<ROWS, COLS>: one instantiation per block shape.
    #[inline(never)]
    fn scaled_idct<R: Transform1d, C: Transform1d>(
        from: &mut [f32],
        to: &mut [f32],
        to_stride: usize,
        scratch: &mut [f32],
    ) {
        let (block, tmp) = scratch.split_at_mut(R::N * C::N);
        if R::N < C::N {
            transpose::<R, C>(from, block);
            idct_columns::<C, R, true>(block, from, 0, tmp);
            transpose::<C, R>(from, block);
            idct_columns::<R, C, false>(block, to, to_stride, tmp);
        } else {
            idct_columns::<C, R, true>(from, block, 0, tmp);
            transpose::<C, R>(block, from);
            idct_columns::<R, C, false>(from, to, to_stride, tmp);
        }
    }

    // The original selects the template instantiation statically; one dispatch per
    // block replaces it here.
    macro_rules! by_cols {
    ($f:ident, $r:ty, $cols:expr, $($arg:expr),*) => {
        match $cols {
            1 => $f::<$r, Size1>($($arg),*),
            2 => $f::<$r, Size2>($($arg),*),
            4 => $f::<$r, Size4>($($arg),*),
            8 => $f::<$r, Size8>($($arg),*),
            16 => $f::<$r, Size16>($($arg),*),
            32 => $f::<$r, Size32>($($arg),*),
            64 => $f::<$r, Size64>($($arg),*),
            128 => $f::<$r, Size128>($($arg),*),
            256 => $f::<$r, Size256>($($arg),*),
            _ => unreachable!("transform sizes are checked powers of two up to 256"),
        }
    };
}
    macro_rules! by_shape {
    ($f:ident, $rows:expr, $cols:expr, $($arg:expr),*) => {
        match $rows {
            1 => by_cols!($f, Size1, $cols, $($arg),*),
            2 => by_cols!($f, Size2, $cols, $($arg),*),
            4 => by_cols!($f, Size4, $cols, $($arg),*),
            8 => by_cols!($f, Size8, $cols, $($arg),*),
            16 => by_cols!($f, Size16, $cols, $($arg),*),
            32 => by_cols!($f, Size32, $cols, $($arg),*),
            64 => by_cols!($f, Size64, $cols, $($arg),*),
            128 => by_cols!($f, Size128, $cols, $($arg),*),
            256 => by_cols!($f, Size256, $cols, $($arg),*),
            _ => unreachable!("transform sizes are checked powers of two up to 256"),
        }
    };
}

    pub(super) fn scaled_dct_dispatch(
        rows: usize,
        cols: usize,
        from: &[f32],
        from_stride: usize,
        to: &mut [f32],
        scratch: &mut [f32],
    ) {
        by_shape!(scaled_dct, rows, cols, from, from_stride, to, scratch)
    }
    pub(super) fn scaled_idct_dispatch(
        rows: usize,
        cols: usize,
        from: &mut [f32],
        to: &mut [f32],
        to_stride: usize,
        scratch: &mut [f32],
    ) {
        by_shape!(scaled_idct, rows, cols, from, to, to_stride, scratch)
    }
}

#[cfg(not(target_arch = "aarch64"))]
mod runtime {
    use super::*;

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
    pub(super) fn scaled_dct_dispatch(
        rows: usize,
        cols: usize,
        from: &[f32],
        from_stride: usize,
        to: &mut [f32],
        scratch: &mut [f32],
    ) {
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
    pub(super) fn scaled_idct_dispatch(
        rows: usize,
        cols: usize,
        from: &mut [f32],
        to: &mut [f32],
        to_stride: usize,
        scratch: &mut [f32],
    ) {
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
}

#[cfg(target_arch = "aarch64")]
use instantiated::{scaled_dct_dispatch, scaled_idct_dispatch};
#[cfg(not(target_arch = "aarch64"))]
use runtime::{scaled_dct_dispatch, scaled_idct_dispatch};

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
    scaled_dct_dispatch(rows, cols, from, from_stride, to, scratch);
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
    scaled_idct_dispatch(rows, cols, from, to, to_stride, scratch);
}
