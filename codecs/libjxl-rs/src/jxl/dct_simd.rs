// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The SIMD (I)DCT of lib/jxl/dct-inl.h with the block adapters of
//! dct_block-inl.h and the transposes of transpose-inl.h, for the NEON and
//! AVX2 targets (port ADR 0004), written once against [`SimdF32`]. Compare
//! it only with the original's SIMD build at the same target.
//!
//! The original runs every one-dimensional transform over `SZ` columns at
//! once with a `HWY_CAPPED(float, SZ)` descriptor: `SZ = M` when the block
//! has at most `Lanes` columns (`fit`), otherwise full vectors over column
//! groups. [`Capped`] is that descriptor: [`Full`] is the target's full
//! vector, [`Part`] a partial vector of 1, 2 or 4 lanes (NEON's 64-bit and
//! 32-bit vectors, AVX2's 128-bit vector). Every operation in these kernels
//! (`Add`, `Sub`, `Mul`, the fused `MulAdd`/`NegMulAdd`, loads and stores) is
//! lane-wise, so [`Part`] computes each lane with the binary32 operation the
//! partial vector performs on it (a single-rounding fused multiply-add for
//! `MulAdd`, as `vfma`/`vfmadd` do). Scratch memory keeps the original's
//! `SZ`-strided layout, so the transforms write the same scratch words.
//! The transposes move words only; their result does not depend on the
//! original's 4x4 (NEON) or 8x8 (AVX2) vector blocking.
use super::dct_scales::{SQRT2, wc_multipliers};
use super::hwy::SimdF32;

/// `HWY_CAPPED(float, SZ)` for the operations of dct-inl.h.
pub(crate) trait Capped: Copy {
    type V: Copy;
    const SZ: usize;
    fn load(self, src: &[f32]) -> Self::V;
    fn store(self, v: Self::V, dst: &mut [f32]);
    fn set(self, v: f32) -> Self::V;
    fn add(self, a: Self::V, b: Self::V) -> Self::V;
    fn sub(self, a: Self::V, b: Self::V) -> Self::V;
    fn mul(self, a: Self::V, b: Self::V) -> Self::V;
    /// `MulAdd(mul, x, add)`, fused.
    fn mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V;
    /// `NegMulAdd(mul, x, add) = add - mul * x`, fused.
    fn neg_mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V;
}

/// The target's full vector, `HWY_CAPPED(float, Lanes)`.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Full<D>(pub D);
impl<D: SimdF32> Capped for Full<D> {
    type V = D::V;
    const SZ: usize = D::LANES;
    #[inline(always)]
    fn load(self, src: &[f32]) -> Self::V {
        self.0.load(src)
    }
    #[inline(always)]
    fn store(self, v: Self::V, dst: &mut [f32]) {
        self.0.store(v, dst)
    }
    #[inline(always)]
    fn set(self, v: f32) -> Self::V {
        self.0.set(v)
    }
    #[inline(always)]
    fn add(self, a: Self::V, b: Self::V) -> Self::V {
        self.0.add(a, b)
    }
    #[inline(always)]
    fn sub(self, a: Self::V, b: Self::V) -> Self::V {
        self.0.sub(a, b)
    }
    #[inline(always)]
    fn mul(self, a: Self::V, b: Self::V) -> Self::V {
        self.0.mul(a, b)
    }
    #[inline(always)]
    fn mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        self.0.mul_add(mul, x, add)
    }
    #[inline(always)]
    fn neg_mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        self.0.neg_mul_add(mul, x, add)
    }
}

/// A partial vector of `N` lanes (fewer than the target's), computed lane by
/// lane with the binary32 operation the original's partial vector performs.
#[derive(Clone, Copy, Debug)]
pub(crate) struct Part<const N: usize>;
impl<const N: usize> Capped for Part<N> {
    type V = [f32; N];
    const SZ: usize = N;
    #[inline(always)]
    fn load(self, src: &[f32]) -> Self::V {
        src[..N].try_into().unwrap()
    }
    #[inline(always)]
    fn store(self, v: Self::V, dst: &mut [f32]) {
        dst[..N].copy_from_slice(&v)
    }
    #[inline(always)]
    fn set(self, v: f32) -> Self::V {
        [v; N]
    }
    #[inline(always)]
    fn add(self, a: Self::V, b: Self::V) -> Self::V {
        core::array::from_fn(|i| a[i] + b[i])
    }
    #[inline(always)]
    fn sub(self, a: Self::V, b: Self::V) -> Self::V {
        core::array::from_fn(|i| a[i] - b[i])
    }
    #[inline(always)]
    fn mul(self, a: Self::V, b: Self::V) -> Self::V {
        core::array::from_fn(|i| a[i] * b[i])
    }
    #[inline(always)]
    fn mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        core::array::from_fn(|i| mul[i].mul_add(x[i], add[i]))
    }
    // Negating a factor is exact, so this is the single-rounding add - mul * x.
    #[inline(always)]
    fn neg_mul_add(self, mul: Self::V, x: Self::V, add: Self::V) -> Self::V {
        core::array::from_fn(|i| (-mul[i]).mul_add(x[i], add[i]))
    }
}

#[inline(always)]
fn ld<C: Capped>(c: C, s: &[f32], i: usize) -> C::V {
    c.load(&s[i * C::SZ..])
}
#[inline(always)]
fn st<C: Capped>(c: C, v: C::V, s: &mut [f32], i: usize) {
    c.store(v, &mut s[i * C::SZ..])
}

/// One transform size of DCT1DImpl<N, SZ>/IDCT1DImpl<N, SZ>. Buffers hold
/// `N` vectors of `SZ` lanes (`i * SZ` is vector `i`), as the original's
/// `mem`/`tmp`; `scratch` is the original's `tmp + N * SZ`.
pub(crate) trait Size {
    const N: usize;
    /// DCT1DImpl<N, SZ>(mem, tmp).
    fn dct<C: Capped>(c: C, mem: &mut [f32], tmp: &mut [f32]);
    /// IDCT1DImpl<N, SZ>(mem, SZ, mem, SZ, tmp): the in-place recursive call.
    fn idct_in_place<C: Capped>(c: C, mem: &mut [f32], tmp: &mut [f32]);
    /// The steps of IDCT1DImpl<N, SZ> between ForwardEvenOdd and
    /// MultiplyAndAdd on the gathered `tmp` (N vectors).
    fn idct_finish<C: Capped>(c: C, tmp: &mut [f32], scratch: &mut [f32]);
}

pub(crate) struct Size1;
pub(crate) struct Size2;
impl Size for Size1 {
    const N: usize = 1;
    #[inline(always)]
    fn dct<C: Capped>(_c: C, _mem: &mut [f32], _tmp: &mut [f32]) {}
    // StoreU(LoadU(d, from), d, to) with from == to.
    #[inline(always)]
    fn idct_in_place<C: Capped>(_c: C, _mem: &mut [f32], _tmp: &mut [f32]) {}
    fn idct_finish<C: Capped>(_c: C, _tmp: &mut [f32], _scratch: &mut [f32]) {
        unreachable!("IDCT1DImpl<1> has no recursion body")
    }
}
impl Size for Size2 {
    const N: usize = 2;
    #[inline(always)]
    fn dct<C: Capped>(c: C, mem: &mut [f32], _tmp: &mut [f32]) {
        let in1 = ld(c, mem, 0);
        let in2 = ld(c, mem, 1);
        st(c, c.add(in1, in2), mem, 0);
        st(c, c.sub(in1, in2), mem, 1);
    }
    #[inline(always)]
    fn idct_in_place<C: Capped>(c: C, mem: &mut [f32], _tmp: &mut [f32]) {
        let in1 = ld(c, mem, 0);
        let in2 = ld(c, mem, 1);
        st(c, c.add(in1, in2), mem, 0);
        st(c, c.sub(in1, in2), mem, 1);
    }
    fn idct_finish<C: Capped>(_c: C, _tmp: &mut [f32], _scratch: &mut [f32]) {
        unreachable!("IDCT1DImpl<2> has no recursion body")
    }
}

/// CoeffBundle<N, SZ>::ForwardEvenOdd: gather even, then odd input rows.
#[inline(always)]
fn forward_even_odd<C: Capped>(c: C, n: usize, from: &[f32], from_stride: usize, out: &mut [f32]) {
    for i in 0..n / 2 {
        st(c, c.load(&from[2 * i * from_stride..]), out, i);
    }
    for i in n / 2..n {
        st(
            c,
            c.load(&from[(2 * (i - n / 2) + 1) * from_stride..]),
            out,
            i,
        );
    }
}

/// CoeffBundle<N, SZ>::MultiplyAndAdd: the final butterflies into `out`.
#[inline(always)]
fn multiply_and_add<C: Capped>(c: C, n: usize, coeff: &[f32], out: &mut [f32], out_stride: usize) {
    let wc = wc_multipliers(n);
    for i in 0..n / 2 {
        let mul = c.set(wc[i]);
        let in1 = ld(c, coeff, i);
        let in2 = ld(c, coeff, n / 2 + i);
        let out1 = c.mul_add(mul, in2, in1);
        let out2 = c.neg_mul_add(mul, in2, in1);
        c.store(out1, &mut out[i * out_stride..]);
        c.store(out2, &mut out[(n - i - 1) * out_stride..]);
    }
}

macro_rules! recursive_size {
    ($name:ident, $n:literal, $half:ident, $inline:meta) => {
        pub(crate) struct $name;
        impl Size for $name {
            const N: usize = $n;
            #[$inline]
            fn dct<C: Capped>(c: C, mem: &mut [f32], tmp: &mut [f32]) {
                const N: usize = $n;
                const H: usize = N / 2;
                let sz = C::SZ;
                let mem = &mut mem[..N * sz];
                let (tmp, rest) = tmp.split_at_mut(N * sz);
                // CoeffBundle<N / 2, SZ>::AddReverse(mem, mem + N / 2 * SZ, tmp).
                for i in 0..H {
                    let in1 = ld(c, mem, i);
                    let in2 = ld(c, mem, H + (H - i - 1));
                    st(c, c.add(in1, in2), tmp, i);
                }
                $half::dct(c, &mut tmp[..H * sz], rest);
                // SubReverse(mem, mem + N / 2 * SZ, tmp + N / 2 * SZ).
                for i in 0..H {
                    let in1 = ld(c, mem, i);
                    let in2 = ld(c, mem, H + (H - i - 1));
                    st(c, c.sub(in1, in2), tmp, H + i);
                }
                // CoeffBundle<N, SZ>::Multiply(tmp).
                let wc = wc_multipliers(N);
                for i in 0..H {
                    let in1 = ld(c, tmp, H + i);
                    let mul = c.set(wc[i]);
                    st(c, c.mul(in1, mul), tmp, H + i);
                }
                $half::dct(c, &mut tmp[H * sz..], rest);
                // CoeffBundle<N / 2, SZ>::B(tmp + N / 2 * SZ).
                {
                    let coeff = &mut tmp[H * sz..];
                    let sqrt2 = c.set(SQRT2);
                    let in1_0 = ld(c, coeff, 0);
                    let in2_0 = ld(c, coeff, 1);
                    st(c, c.mul_add(in1_0, sqrt2, in2_0), coeff, 0);
                    #[allow(clippy::reversed_empty_ranges)]
                    for i in 1..H - 1 {
                        let in1 = ld(c, coeff, i);
                        let in2 = ld(c, coeff, i + 1);
                        st(c, c.add(in1, in2), coeff, i);
                    }
                }
                // CoeffBundle<N, SZ>::InverseEvenOdd(tmp, mem).
                for i in 0..H {
                    st(c, ld(c, tmp, i), mem, 2 * i);
                }
                for i in H..N {
                    st(c, ld(c, tmp, i), mem, 2 * (i - H) + 1);
                }
            }
            #[$inline]
            fn idct_in_place<C: Capped>(c: C, mem: &mut [f32], tmp: &mut [f32]) {
                const N: usize = $n;
                let sz = C::SZ;
                let mem = &mut mem[..N * sz];
                let (tmp, rest) = tmp.split_at_mut(N * sz);
                forward_even_odd(c, N, mem, sz, tmp);
                Self::idct_finish(c, tmp, rest);
                multiply_and_add(c, N, tmp, mem, sz);
            }
            #[$inline]
            fn idct_finish<C: Capped>(c: C, tmp: &mut [f32], scratch: &mut [f32]) {
                const N: usize = $n;
                const H: usize = N / 2;
                let sz = C::SZ;
                let tmp = &mut tmp[..N * sz];
                $half::idct_in_place(c, &mut tmp[..H * sz], scratch);
                // CoeffBundle<N / 2, SZ>::BTranspose(tmp + N / 2 * SZ).
                {
                    let coeff = &mut tmp[H * sz..];
                    for i in (1..H).rev() {
                        let in1 = ld(c, coeff, i);
                        let in2 = ld(c, coeff, i - 1);
                        st(c, c.add(in1, in2), coeff, i);
                    }
                    let sqrt2 = c.set(SQRT2);
                    let in1 = ld(c, coeff, 0);
                    st(c, c.mul(in1, sqrt2), coeff, 0);
                }
                $half::idct_in_place(c, &mut tmp[H * sz..], scratch);
            }
        }
    };
}
// Levels up to 32 points inline into their callers, as the original's
// templates do and as the scalar translation (dct.rs) does; larger levels are
// left to the compiler (code size).
recursive_size!(Size4, 4, Size2, inline(always));
recursive_size!(Size8, 8, Size4, inline(always));
recursive_size!(Size16, 16, Size8, inline(always));
recursive_size!(Size32, 32, Size16, inline(always));
recursive_size!(Size64, 64, Size32, inline);
recursive_size!(Size128, 128, Size64, inline);
recursive_size!(Size256, 256, Size128, inline);

/// IDCT1DImpl<N, SZ>(from, from_stride, to, to_stride, tmp) with disjoint
/// `from` and `to`.
#[inline(always)]
fn idct1d_impl<C: Capped, S: Size>(
    c: C,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    to_stride: usize,
    tmp: &mut [f32],
) {
    match S::N {
        1 => c.store(c.load(from), to),
        2 => {
            let in1 = c.load(from);
            let in2 = c.load(&from[from_stride..]);
            c.store(c.add(in1, in2), to);
            c.store(c.sub(in1, in2), &mut to[to_stride..]);
        }
        n => {
            let (t, rest) = tmp.split_at_mut(n * C::SZ);
            forward_even_odd(c, n, from, from_stride, t);
            S::idct_finish(c, t, rest);
            multiply_and_add(c, n, t, to, to_stride);
        }
    }
}

/// DCT1DWrapper's body for the column group at `from`/`to`: LoadFromBlock,
/// DCT1DImpl<N, SZ>, StoreToBlockAndScale.
#[inline(always)]
fn dct1d_group<C: Capped, S: Size>(
    c: C,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    to_stride: usize,
    tmp: &mut [f32],
) {
    let n = S::N;
    for i in 0..n {
        st(c, c.load(&from[i * from_stride..]), tmp, i);
    }
    let (t, rest) = tmp.split_at_mut(n * C::SZ);
    S::dct(c, t, rest);
    let mul = c.set(1.0f32 / n as f32);
    for i in 0..n {
        c.store(c.mul(mul, ld(c, t, i)), &mut to[i * to_stride..]);
    }
}

/// IDCT1D<N, M>: the `fit` wrapper with `HWY_CAPPED(float, M)` when `M` is at
/// most `Lanes`, otherwise full vectors over the column groups.
#[inline(always)]
fn idct1d<D: SimdF32, S: Size>(
    d: D,
    m: usize,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    to_stride: usize,
    tmp: &mut [f32],
) {
    if m >= D::LANES {
        for i in (0..m).step_by(D::LANES) {
            idct1d_impl::<_, S>(
                Full(d),
                &from[i..],
                from_stride,
                &mut to[i..],
                to_stride,
                tmp,
            );
        }
        return;
    }
    match m {
        1 => idct1d_impl::<_, S>(Part::<1>, from, from_stride, to, to_stride, tmp),
        2 => idct1d_impl::<_, S>(Part::<2>, from, from_stride, to, to_stride, tmp),
        4 => idct1d_impl::<_, S>(Part::<4>, from, from_stride, to, to_stride, tmp),
        _ => unreachable!("block widths are powers of two"),
    }
}

/// DCT1D<N, M>, with the same descriptor choice as [`idct1d`].
#[inline(always)]
fn dct1d<D: SimdF32, S: Size>(
    d: D,
    m: usize,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    to_stride: usize,
    tmp: &mut [f32],
) {
    if m >= D::LANES {
        for i in (0..m).step_by(D::LANES) {
            dct1d_group::<_, S>(
                Full(d),
                &from[i..],
                from_stride,
                &mut to[i..],
                to_stride,
                tmp,
            );
        }
        return;
    }
    match m {
        1 => dct1d_group::<_, S>(Part::<1>, from, from_stride, to, to_stride, tmp),
        2 => dct1d_group::<_, S>(Part::<2>, from, from_stride, to, to_stride, tmp),
        4 => dct1d_group::<_, S>(Part::<4>, from, from_stride, to, to_stride, tmp),
        _ => unreachable!("block widths are powers of two"),
    }
}

/// Transpose<N, M>::Run(DCTFrom(from, from_stride), DCTTo(to, to_stride)):
/// `to(m, n) = from(n, m)`.
#[inline(always)]
fn transpose(
    rows: usize,
    cols: usize,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    to_stride: usize,
) {
    for n in 0..rows {
        for m in 0..cols {
            to[m * to_stride + n] = from[n * from_stride + m];
        }
    }
}

/// ComputeScaledIDCT<ROWS, COLS>: `from` (ROWS * COLS coefficients) is
/// mutated as in the original; `scratch` holds the block and `tmp`.
#[inline]
fn scaled_idct<D: SimdF32, R: Size, C: Size>(
    d: D,
    from: &mut [f32],
    to: &mut [f32],
    to_stride: usize,
    scratch: &mut [f32],
) {
    let (rows, cols) = (R::N, C::N);
    let (block, tmp) = scratch.split_at_mut(rows * cols);
    if rows < cols {
        transpose(rows, cols, from, cols, block, rows);
        idct1d::<D, C>(d, rows, block, rows, from, rows, tmp);
        transpose(cols, rows, from, rows, block, cols);
        idct1d::<D, R>(d, cols, block, cols, to, to_stride, tmp);
    } else {
        idct1d::<D, C>(d, rows, from, rows, block, rows, tmp);
        transpose(cols, rows, block, rows, from, cols);
        idct1d::<D, R>(d, cols, from, cols, to, to_stride, tmp);
    }
}

/// ComputeScaledDCT<ROWS, COLS>: the maybe-transposed scaled DCT of the
/// ROWS x COLS samples at `from` into `to`.
#[inline]
fn scaled_dct<D: SimdF32, R: Size, C: Size>(
    d: D,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    scratch: &mut [f32],
) {
    let (rows, cols) = (R::N, C::N);
    let (block, tmp) = scratch.split_at_mut(rows * cols);
    if rows < cols {
        dct1d::<D, R>(d, cols, from, from_stride, block, cols, tmp);
        transpose(rows, cols, block, cols, to, rows);
        dct1d::<D, C>(d, rows, to, rows, block, rows, tmp);
        transpose(cols, rows, block, rows, to, cols);
    } else {
        dct1d::<D, R>(d, cols, from, from_stride, to, cols, tmp);
        transpose(rows, cols, to, cols, block, rows);
        dct1d::<D, C>(d, rows, block, rows, to, rows, tmp);
    }
}

macro_rules! by_cols {
    ($f:ident, $d:expr, $r:ty, $cols:expr, $($arg:expr),*) => {
        match $cols {
            1 => $f::<_, $r, Size1>($d, $($arg),*),
            2 => $f::<_, $r, Size2>($d, $($arg),*),
            4 => $f::<_, $r, Size4>($d, $($arg),*),
            8 => $f::<_, $r, Size8>($d, $($arg),*),
            16 => $f::<_, $r, Size16>($d, $($arg),*),
            32 => $f::<_, $r, Size32>($d, $($arg),*),
            64 => $f::<_, $r, Size64>($d, $($arg),*),
            128 => $f::<_, $r, Size128>($d, $($arg),*),
            256 => $f::<_, $r, Size256>($d, $($arg),*),
            _ => unreachable!("transform sizes are checked powers of two up to 256"),
        }
    };
}
macro_rules! by_shape {
    ($f:ident, $d:expr, $rows:expr, $cols:expr, $($arg:expr),*) => {
        match $rows {
            1 => by_cols!($f, $d, Size1, $cols, $($arg),*),
            2 => by_cols!($f, $d, Size2, $cols, $($arg),*),
            4 => by_cols!($f, $d, Size4, $cols, $($arg),*),
            8 => by_cols!($f, $d, Size8, $cols, $($arg),*),
            16 => by_cols!($f, $d, Size16, $cols, $($arg),*),
            32 => by_cols!($f, $d, Size32, $cols, $($arg),*),
            64 => by_cols!($f, $d, Size64, $cols, $($arg),*),
            128 => by_cols!($f, $d, Size128, $cols, $($arg),*),
            256 => by_cols!($f, $d, Size256, $cols, $($arg),*),
            _ => unreachable!("transform sizes are checked powers of two up to 256"),
        }
    };
}

fn dimensions(rows: usize, cols: usize) {
    assert!(rows.is_power_of_two() && cols.is_power_of_two() && rows <= 256 && cols <= 256);
}

/// ComputeScaledDCT<rows, cols> for the descriptor's target; the same
/// contract as [`super::dct::compute_scaled_dct`].
#[inline]
pub(crate) fn compute_scaled_dct_simd<D: SimdF32>(
    d: D,
    rows: usize,
    cols: usize,
    from: &[f32],
    from_stride: usize,
    to: &mut [f32],
    scratch: &mut [f32],
) {
    dimensions(rows, cols);
    assert!(from_stride >= cols);
    by_shape!(scaled_dct, d, rows, cols, from, from_stride, to, scratch)
}

/// ComputeScaledIDCT<rows, cols> for the descriptor's target; the same
/// contract as [`super::dct::compute_scaled_idct`].
#[inline]
pub(crate) fn compute_scaled_idct_simd<D: SimdF32>(
    d: D,
    rows: usize,
    cols: usize,
    from: &mut [f32],
    to: &mut [f32],
    to_stride: usize,
    scratch: &mut [f32],
) {
    dimensions(rows, cols);
    assert!(to_stride >= cols);
    by_shape!(scaled_idct, d, rows, cols, from, to, to_stride, scratch)
}
