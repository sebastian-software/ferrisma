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
//! Inlining policy. One implementation for every target. Levels up to 32
//! points are inlined into their callers, as the original's templates are;
//! nothing is forced out of line. Inlining lets LLVM commute the operands of
//! an addition, which can change which of two competing NaN payloads
//! survives; NaN payloads are not part of the gate (decision D8, port ADR
//! 0002), while every finite result, infinity and signed zero is unchanged.
use super::dct_scales::{SQRT2, wc_multipliers};

// Highway scalar float multiplication widens to double, multiplies, then
// narrows. Its MulAdd/NegMulAdd subsequently add/subtract in binary32.
#[inline]
pub(crate) fn scalar_mul(a: f32, b: f32) -> f32 {
    (f64::from(a) * f64::from(b)) as f32
}

// The original IDCT's MultiplyAndAdd: the addend plus the product.
#[inline]
fn idct_add(addend: f32, product: f32) -> f32 {
    addend + product
}

mod instantiated {
    use super::*;

    // IDCT1DImpl<2, 1> on one strided column.
    #[inline]
    fn idct2_column(from: &[f32], x: usize, from_stride: usize, to: &mut [f32], to_stride: usize) {
        let a = from[x];
        let b = from[x + from_stride];
        to[x] = a + b;
        to[x + to_stride] = a - b;
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
        #[inline(always)]
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
                    for i in (1..HALF).rev() {
                        tmp[HALF + i] += tmp[HALF + i - 1];
                    }
                    tmp[HALF] = scalar_mul(tmp[HALF], SQRT2);
                    $half::idct_in_place(&mut tmp[HALF..], scratch);
                }
            }
        };
    }

    // Attributes: DCT level, in-place IDCT level, IDCT recursion. Levels up to
    // 32 points inline into their callers; larger levels are left to the
    // compiler (code size).
    recursive_transform!(
        Size4,
        4,
        Size2,
        inline(always),
        inline(always),
        inline(always)
    );
    recursive_transform!(
        Size8,
        8,
        Size4,
        inline(always),
        inline(always),
        inline(always)
    );
    recursive_transform!(
        Size16,
        16,
        Size8,
        inline(always),
        inline(always),
        inline(always)
    );
    recursive_transform!(
        Size32,
        32,
        Size16,
        inline(always),
        inline(always),
        inline(always)
    );
    recursive_transform!(Size64, 64, Size32, inline, inline, inline);
    recursive_transform!(Size128, 128, Size64, inline, inline, inline);
    recursive_transform!(Size256, 256, Size128, inline, inline, inline);

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

use instantiated::{scaled_dct_dispatch, scaled_idct_dispatch};

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
