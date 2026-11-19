// Copyright © 2018-2019, VideoLAN and dav1d authors
// Copyright © 2018-2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! The 1-D inverse transforms of `upstream/dav1d/src/itx_1d.c`, on vectors.
//!
//! Each lane is one independent row or column; a function takes the `N`
//! inputs of a transform as `N` vectors and returns its `N` outputs. The
//! arithmetic is the C's, statement by statement and with its names,
//! including the C's rearranged products
//! (`((a * 1567 - b * (3784 - 4096) + 2048) >> 12) - b`), so that it stays
//! free of overflow where the C is, at every bit depth.
//!
//! # Two lane types
//!
//! A transform is written once, generic over the lanes `V` that hold its
//! *stored* values, and `V::Wide`, 32-bit lanes that hold the rest:
//!
//! - stored (`V`): the inputs, and every result of the C's `CLIP`;
//! - wide (`V::Wide`): every product (`(a * c1 + b * c2 + rnd) >> s`, the
//!   `* 181` steps) and the outputs the C does not clip.
//!
//! The notation, against the C:
//!
//! | here              | the C                          |
//! |-------------------|--------------------------------|
//! | `wide([a, b])`    | `a`, `b` as operands of a product |
//! | `w(a)`            | the same for one value         |
//! | `clip(x)`         | `CLIP(x)`, `x` a wide expression |
//! | `add(a, b)`       | `CLIP(a + b)`, `a`, `b` stored |
//! | `sub(a, b)`       | `CLIP(a - b)`, `a`, `b` stored |
//!
//! Above 8 bits `V` is itself a vector of `i32` (`V::Wide` is `V`), and all
//! of this is the C's `int` arithmetic as in the 32-bit pilot. At 8 bits the
//! 8-lane passes store in `i16` lanes: the ranges below show that every
//! stored value of the C fits there and that each operation on it is exact.
//!
//! # The ranges at 8 bpc (`BITDEPTH == 8`)
//!
//! `inv_txfm_add_c` (itx_tmpl.c) clips both passes to `INT16_MIN..=INT16_MAX`
//! (`row_clip_*` and `col_clip_*` are the range of `int16_t` at 8 bits).
//! For every input, conformant or not (all `int16_t` coefficients):
//!
//! 1. *Inputs.* A coefficient is an `int16_t`. The `is_rect2` scaling
//!    `(c * 181 + 128) >> 8` has `|result| <= 23170`: stored (computed
//!    wide).
//! 2. *Clipped values.* `CLIP` maps to `INT16_MIN..=INT16_MAX`, the range of
//!    `i16`: every `CLIP` result is stored. `CLIP(x)` of a wide `x` is the
//!    saturating narrowing of `x`; `CLIP(a + b)` of stored `a`, `b` (the C
//!    forms `a + b` exactly in `int`) is the saturating `i16` addition.
//!    Both are exactly the C's clip, because its bounds are those of `i16`.
//! 3. *Products.* Their operands are stored values (`|v| <= 2^15`) or a sum
//!    or difference of two (`(t6a - t5a) * 181`, `|v| <= 2^16`), formed in
//!    the wide lanes; the coefficients are below 4096, at most four products
//!    are summed (`adst4`), so every intermediate is below `2^30` and the
//!    32-bit lanes compute the C's `int` values exactly (its rearranged forms
//!    included). The results do not fit 16 bits for every input: a rotation
//!    reaches about `2^15 * sqrt(2)`, `((in0 + in2) * 181 + 128) >> 8`
//!    reaches 46341. They stay wide until the C clips them. (In a
//!    conformant stream they fit 16 bits; the assembly stores them in 16
//!    bits there, and wraps or saturates beyond it.)
//! 4. *Outputs.* The DCT's outputs are `CLIP`s: stored. The ADST's are
//!    `CLIP`s, negated `CLIP`s (up to `+32768`) and `* 181` products; the
//!    4-point ADST's and the identities' are not clipped at all (the 32-point
//!    identity reaches `4 * 32767`): wide (`round_wide` of `transform`).
//! 5. *Between the passes* the C computes `iclip((v + rnd) >> shift,
//!    col_clip_min, col_clip_max)`: stored. For a stored `v` the rounded
//!    shift stays within `i16`; it is computed in `i16` as
//!    `(v >> shift) + ((v >> (shift - 1)) & 1)`, which equals
//!    `(v + (1 << (shift - 1))) >> shift` for every `v` and has no
//!    intermediate overflow.
//! 6. *To the pixels* the C adds `(c + 8) >> 4`. The second pass's outputs
//!    are below `2^17` in magnitude (the 32-point identity of a stored value
//!    is the largest), so `|(c + 8) >> 4| <= 8192`: stored, and
//!    `dst + residual` is exact in `i16` before the clip to the pixel range.
//!
//! So 32 bits are needed exactly where the C forms a value beyond `int16_t`
//! that it has not clipped yet: the products, the sums that feed them, and
//! the unclipped outputs until the rounding of the pass. Nothing else is
//! widened, and no step saturates where the C does not clip. Not on 16-bit
//! lanes, for other reasons: the 4-lane passes (blocks 4 wide or tall;
//! `fearless_simd` has no 4-lane `i16` vector), the lossless WHT (no
//! clipping: its second pass exceeds `int16_t` for non-conformant input)
//! and 10 and 12 bits (`row_clip` is 18 and 20 bits wide there).
//!
//! The C's `tx64` variants of the 4- to 32-point DCTs (used by the 64-point
//! DCT, whose inputs 32..63 are zero) are the same formulas with the zero
//! inputs dropped: `dct64` passes zeros and the compiler drops them.

use core::ops::Neg;

use fearless_simd::prelude::*;
use fearless_simd::{i16x16, i16x8, i32x16, i32x4, i32x8, i64x2, u16x16, u16x8};

use super::blocks::Residual;

/// 32-bit lanes: the wide values of the transforms, and the lanes of the
/// paths that stay on 32 bits.
pub trait Int32<S: Simd>: SimdInt<S, Element = i32> + Neg<Output = Self> {
    /// Stores the lanes (each in `0..=u16::MAX`) as `u16`s.
    fn store_u16(self, out: &mut [u16]);

    /// Stores the lanes (each in `0..=u8::MAX`) as `u8`s.
    fn store_u8(self, out: &mut [u8]);
}

/// The lanes that hold a transform's stored values (module documentation).
pub trait Lanes<S: Simd>: SimdInt<S> {
    /// The 32-bit lanes of the same values.
    type Wide: Int32<S>;

    /// The `i16` lanes that carry these values to the pixels.
    type Residual: Residual<S>;

    /// The clipping range of a pass, as wide vectors.
    fn range(simd: S, bounds: (i32, i32)) -> Clip<Self::Wide>;

    /// The lanes as 32-bit lanes.
    fn wide(self) -> Self::Wide;

    /// The C's `CLIP(x)`: `iclip(x, min, max)`, stored.
    fn clip(x: Self::Wide, range: Clip<Self::Wide>) -> Self;

    /// A wide value that is within the stored range, stored.
    fn stored(x: Self::Wide) -> Self;

    /// `CLIP(self + other)`.
    fn clip_add(self, other: Self, range: Clip<Self::Wide>) -> Self;

    /// `CLIP(self - other)`.
    fn clip_sub(self, other: Self, range: Clip<Self::Wide>) -> Self;

    /// `iclip(self, min, max)` of a stored value.
    fn clip_stored(self, range: Clip<Self::Wide>) -> Self;

    /// `(self + ((1 << shift) >> 1)) >> shift`.
    fn round_shift(self, shift: u32) -> Self;

    /// Transposes the square block `block` (`Self::LEN` vectors).
    fn transpose(block: &mut [Self]);

    /// The lanes (each in the range of `i16`) as `i16` lanes, in order.
    fn residual(self) -> Self::Residual;
}

/// The bounds of the C's `CLIP` (`min`, `max` of an `itx_1d_fn`).
#[derive(Clone, Copy)]
pub struct Clip<W> {
    pub min: W,
    pub max: W,
}

/// The `Lanes` of a vector of `i32`: stored and wide are the same lanes.
macro_rules! int32_lanes {
    ($t:ident, $transpose:ident, |$v:ident| $residual:expr) => {
        impl<S: Simd> Lanes<S> for $t<S> {
            type Wide = Self;
            type Residual = i16x8<S>;

            #[inline(always)]
            fn range(simd: S, (min, max): (i32, i32)) -> Clip<Self> {
                Clip { min: Self::splat(simd, min), max: Self::splat(simd, max) }
            }

            #[inline(always)]
            fn wide(self) -> Self {
                self
            }

            #[inline(always)]
            fn clip(x: Self, range: Clip<Self>) -> Self {
                x.max(range.min).min(range.max)
            }

            #[inline(always)]
            fn stored(x: Self) -> Self {
                x
            }

            #[inline(always)]
            fn clip_add(self, other: Self, range: Clip<Self>) -> Self {
                Self::clip(self + other, range)
            }

            #[inline(always)]
            fn clip_sub(self, other: Self, range: Clip<Self>) -> Self {
                Self::clip(self - other, range)
            }

            #[inline(always)]
            fn clip_stored(self, range: Clip<Self>) -> Self {
                Self::clip(self, range)
            }

            #[inline(always)]
            fn round_shift(self, shift: u32) -> Self {
                (self + ((1 << shift) >> 1)) >> shift
            }

            #[inline(always)]
            fn transpose(block: &mut [Self]) {
                $transpose(block);
            }

            /// The first lanes of an `i16x8` (the values are within `i16`).
            #[inline(always)]
            fn residual(self) -> i16x8<S> {
                let $v = self;
                $residual
            }
        }
    };
}

int32_lanes!(i32x4, transpose_i32x4, |v| v.relaxed_narrow(v));
int32_lanes!(i32x8, transpose_i32x8, |v| v.relaxed_narrow(v).split().0);

/// 16-bit lanes for 8 bpc (module documentation): 8 of them (`i16x8`, the
/// natural width of NEON) or 16 (`i16x16`, of AVX2), with 32-bit lanes of
/// the same count as the wide lanes.
macro_rules! int16_lanes {
    ($t:ident, $wide:ident, $transpose:ident) => {
        impl<S: Simd> Lanes<S> for $t<S> {
            type Wide = $wide<S>;
            type Residual = Self;

            /// The C clips to the range of `int16_t` at 8 bpc, the only depth
            /// that uses these lanes (`int16_t` coefficients, `blocks::Coef`);
            /// every clip below relies on it.
            #[inline(always)]
            fn range(simd: S, (min, max): (i32, i32)) -> Clip<$wide<S>> {
                debug_assert!(min == i16::MIN.into() && max == i16::MAX.into(), "16-bit lanes: the C clips to int16_t only at 8 bpc");
                Clip { min: $wide::splat(simd, min), max: $wide::splat(simd, max) }
            }

            #[inline(always)]
            fn wide(self) -> $wide<S> {
                let (low, high) = self.widen();
                low.combine(high)
            }

            /// Saturation to `i16` is the clip to `INT16_MIN..=INT16_MAX`.
            #[inline(always)]
            fn clip(x: $wide<S>, _range: Clip<$wide<S>>) -> Self {
                let (low, high) = x.split();
                low.saturating_narrow(high)
            }

            #[inline(always)]
            fn stored(x: $wide<S>) -> Self {
                let (low, high) = x.split();
                low.saturating_narrow(high)
            }

            /// `a + b` of two `int16_t` values clipped to their range.
            #[inline(always)]
            fn clip_add(self, other: Self, _range: Clip<$wide<S>>) -> Self {
                self.saturating_add(other)
            }

            #[inline(always)]
            fn clip_sub(self, other: Self, _range: Clip<$wide<S>>) -> Self {
                self.saturating_sub(other)
            }

            /// Every `i16` is within `INT16_MIN..=INT16_MAX`.
            #[inline(always)]
            fn clip_stored(self, _range: Clip<$wide<S>>) -> Self {
                self
            }

            /// `(v >> s) + bit s - 1 of v`, which is `(v + (1 << (s - 1))) >> s`
            /// without the overflow of the addition.
            #[inline(always)]
            fn round_shift(self, shift: u32) -> Self {
                if shift == 0 {
                    return self;
                }
                (self >> shift) + ((self >> (shift - 1)) & 1)
            }

            #[inline(always)]
            fn transpose(block: &mut [Self]) {
                $transpose(block);
            }

            #[inline(always)]
            fn residual(self) -> Self {
                self
            }
        }
    };
}

int16_lanes!(i16x8, i32x8, transpose_i16x8);
int16_lanes!(i16x16, i32x16, transpose_i16x16);

// The transposes. Each is written with the library's lane-ordered
// interleaves; the wider ones as quadrants of the narrower ones. They are
// functions, not closures: a closure called out of line would not carry the
// target features of the `vectorize` region (x86-64).

/// 4x4 lanes of `i32`.
#[inline(always)]
fn transpose_i32x4<S: Simd>(block: &mut [i32x4<S>]) {
    let [a, b, c, d]: &mut [i32x4<S>; 4] = block.try_into().unwrap();
    // [a0 b0 a1 b1], [a2 b2 a3 b3], [c0 d0 c1 d1], [c2 d2 c3 d3]
    let (ab0, ab2) = (a.zip_low(*b), a.zip_high(*b));
    let (cd0, cd2) = (c.zip_low(*d), c.zip_high(*d));
    // Pairs of lanes as one 64-bit lane: [a0 b0 c0 d0], [a1 b1 c1 d1], ...
    let pairs = |v: i32x4<S>| v.bitcast::<i64x2<S>>();
    let lanes = |v: i64x2<S>| v.bitcast::<i32x4<S>>();
    *a = lanes(pairs(ab0).zip_low(pairs(cd0)));
    *b = lanes(pairs(ab0).zip_high(pairs(cd0)));
    *c = lanes(pairs(ab2).zip_low(pairs(cd2)));
    *d = lanes(pairs(ab2).zip_high(pairs(cd2)));
}

/// The `i`-th 4- or 8-lane block of `N` vectors, as an array to transpose:
/// vectors `first..first + N`, each split into halves, `high` picks one.
macro_rules! quadrant {
    ($block:expr, $first:expr, $high:expr, $n:literal) => {{
        let mut q = [$block[$first].split().0; $n];
        for (i, v) in q.iter_mut().enumerate() {
            let (low_lanes, high_lanes) = $block[$first + i].split();
            *v = if $high { high_lanes } else { low_lanes };
        }
        q
    }};
}

/// 8x8 lanes of `i32`: each 4x4 quadrant transposed, the two off the
/// diagonal exchanged.
#[inline(always)]
fn transpose_i32x8<S: Simd>(block: &mut [i32x8<S>]) {
    let mut q = [quadrant!(block, 0, false, 4), quadrant!(block, 4, false, 4), quadrant!(block, 0, true, 4), quadrant!(block, 4, true, 4)];
    for quadrant in &mut q {
        transpose_i32x4(quadrant);
    }
    let [rows0_lanes0, rows4_lanes0, rows0_lanes4, rows4_lanes4] = q;
    for i in 0..4 {
        block[i] = rows0_lanes0[i].combine(rows4_lanes0[i]);
        block[i + 4] = rows0_lanes4[i].combine(rows4_lanes4[i]);
    }
}

/// 8x8 lanes of `i16`: as four 2x2 blocks of 32-bit pairs, then two of
/// 64-bit quadruples.
#[inline(always)]
fn transpose_i16x8<S: Simd>(block: &mut [i16x8<S>]) {
    let [r0, r1, r2, r3, r4, r5, r6, r7]: [i16x8<S>; 8] = (&*block).try_into().unwrap();
    // [r0_0 r1_0 r0_1 r1_1 r0_2 r1_2 r0_3 r1_3], ...
    let pairs = |a: i16x8<S>, b: i16x8<S>| (a.zip_low(b).bitcast::<i32x4<S>>(), a.zip_high(b).bitcast::<i32x4<S>>());
    let (a0, a1) = pairs(r0, r1);
    let (a2, a3) = pairs(r2, r3);
    let (a4, a5) = pairs(r4, r5);
    let (a6, a7) = pairs(r6, r7);
    // [r0_0 r1_0 r2_0 r3_0 r0_1 r1_1 r2_1 r3_1], ...
    let quads = |a: i32x4<S>, b: i32x4<S>| (a.zip_low(b).bitcast::<i64x2<S>>(), a.zip_high(b).bitcast::<i64x2<S>>());
    let (b0, b1) = quads(a0, a2);
    let (b2, b3) = quads(a1, a3);
    let (b4, b5) = quads(a4, a6);
    let (b6, b7) = quads(a5, a7);
    // Column x: the quadruple of rows 0..4 and of rows 4..8.
    for (x, (low, high)) in [(b0, b4), (b1, b5), (b2, b6), (b3, b7)].into_iter().enumerate() {
        block[2 * x] = low.zip_low(high).bitcast();
        block[2 * x + 1] = low.zip_high(high).bitcast();
    }
}

/// 16x16 lanes of `i16`: each 8x8 quadrant transposed (`transpose_i16x8`),
/// the two off the diagonal exchanged. Where a quadrant's vectors are zero
/// (the padded square of an 8-wide block, `blocks::rows_pass`), the compiler
/// drops its transpose.
#[inline(always)]
fn transpose_i16x16<S: Simd>(block: &mut [i16x16<S>]) {
    let mut q = [quadrant!(block, 0, false, 8), quadrant!(block, 8, false, 8), quadrant!(block, 0, true, 8), quadrant!(block, 8, true, 8)];
    for quadrant in &mut q {
        transpose_i16x8(quadrant);
    }
    let [rows0_lanes0, rows8_lanes0, rows0_lanes8, rows8_lanes8] = q;
    for i in 0..8 {
        block[i] = rows0_lanes0[i].combine(rows8_lanes0[i]);
        block[i + 8] = rows0_lanes8[i].combine(rows8_lanes8[i]);
    }
}

impl<S: Simd> Int32<S> for i32x4<S> {
    #[inline(always)]
    fn store_u16(self, out: &mut [u16]) {
        let narrowed: u16x8<S> = self.narrow(self).bitcast();
        out.copy_from_slice(&narrowed.as_slice()[..4]);
    }

    #[inline(always)]
    fn store_u8(self, out: &mut [u8]) {
        let narrowed: u16x8<S> = self.narrow(self).bitcast();
        out.copy_from_slice(&narrowed.narrow(narrowed).as_slice()[..4]);
    }
}

impl<S: Simd> Int32<S> for i32x8<S> {
    #[inline(always)]
    fn store_u16(self, out: &mut [u16]) {
        let narrowed: u16x16<S> = self.narrow(self).bitcast();
        out.copy_from_slice(&narrowed.as_slice()[..8]);
    }

    #[inline(always)]
    fn store_u8(self, out: &mut [u8]) {
        let narrowed: u16x16<S> = self.narrow(self).bitcast();
        out.copy_from_slice(&narrowed.narrow(narrowed).as_slice()[..8]);
    }
}

/// The wide lanes of `i16x16`; stored to pixels only as two halves.
impl<S: Simd> Int32<S> for i32x16<S> {
    #[inline(always)]
    fn store_u16(self, out: &mut [u16]) {
        let (low, high) = self.split();
        low.store_u16(&mut out[..8]);
        high.store_u16(&mut out[8..]);
    }

    #[inline(always)]
    fn store_u8(self, out: &mut [u8]) {
        let (low, high) = self.split();
        low.store_u8(&mut out[..8]);
        high.store_u8(&mut out[8..]);
    }
}

/// `enum Tx1dType` (src/levels.h).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Dct,
    Adst,
    FlipAdst,
    Identity,
}

/// A 1-D transform as a type: each pass is compiled for its transform.
pub trait Tx1d {
    const KIND: Kind;
}

/// The types of the `Kind`s.
pub mod tx1d {
    use super::{Kind, Tx1d};

    pub struct Dct;
    pub struct Adst;
    pub struct FlipAdst;
    pub struct Identity;

    impl Tx1d for Dct {
        const KIND: Kind = Kind::Dct;
    }
    impl Tx1d for Adst {
        const KIND: Kind = Kind::Adst;
    }
    impl Tx1d for FlipAdst {
        const KIND: Kind = Kind::FlipAdst;
    }
    impl Tx1d for Identity {
        const KIND: Kind = Kind::Identity;
    }
}

/// The `N`-point transform `kind` (`dav1d_tx1d_fns`), each output rounded
/// by the pass: `round` for the outputs the C clips (the DCT's, stored),
/// `round_wide` for those it leaves unclipped (the ADST's and the
/// identities', wide). For `N` = 64 only the first 32 inputs are read.
///
/// The rounding is applied here, so that the wide outputs are not returned
/// (an enum of them, 2 KiB for 32 outputs of `i32x16`, was moved with
/// `memcpy`).
#[inline(always)]
pub fn transform<S: Simd, V: Lanes<S>, const N: usize>(
    kind: Kind,
    c: [V; N],
    range: Clip<V::Wide>,
    round: impl Fn(V) -> V,
    round_wide: impl Fn(V::Wide) -> V,
) -> [V; N] {
    // `[T; N]` as `[T; M]` where the match below has fixed N = M.
    #[inline(always)]
    fn fixed<T: Copy, const M: usize>(c: &[T]) -> [T; M] {
        c[..M].try_into().unwrap()
    }
    let mut out = match (kind, N) {
        (Kind::Dct, 4) => fixed(&each(dct4(fixed(&c), range), &round)),
        (Kind::Dct, 8) => fixed(&each(dct8(fixed(&c), range), &round)),
        (Kind::Dct, 16) => fixed(&each(dct16(fixed(&c), range), &round)),
        (Kind::Dct, 32) => fixed(&each(dct32(fixed(&c), range), &round)),
        (Kind::Dct, 64) => fixed(&each(dct64(fixed(&c), range), &round)),
        (Kind::Adst | Kind::FlipAdst, 4) => fixed(&each(adst4(fixed(&c)), &round_wide)),
        (Kind::Adst | Kind::FlipAdst, 8) => fixed(&each(adst8(fixed(&c), range), &round_wide)),
        (Kind::Adst | Kind::FlipAdst, 16) => fixed(&each(adst16(fixed(&c), range), &round_wide)),
        // inv_identity*_1d_c (no clipping, as in the C). Called through
        // closures inlined by attribute: a function passed as `impl Fn` is
        // called through a shim that may stay out of line, and out of line
        // it leaves the region's target features (x86-64 IDTX 16x16 called
        // AVX2 additions as functions). Rounding the 32-point identity
        // output by output, with no wide array, cut x86-64 8 bpc IDTX 32x32
        // by a quarter but cost ARM64 10 bpc IDTX 32xN up to 44%: kept as an
        // array like the others.
        (Kind::Identity, 4) => each(each(wide(c), #[inline(always)] |v| identity4(v)), &round_wide),
        (Kind::Identity, 8) => each(each(wide(c), #[inline(always)] |v| identity8(v)), &round_wide),
        (Kind::Identity, 16) => each(each(wide(c), #[inline(always)] |v| identity16(v)), &round_wide),
        (Kind::Identity, 32) => each(each(wide(c), #[inline(always)] |v| identity32(v)), &round_wide),
        _ => panic!("no {N}-point transform of this kind in AV1"),
    };
    // inv_flipadst*_1d_c: the ADST written from the last output up (the
    // rounding is per output, so it can come first).
    if kind == Kind::FlipAdst {
        out.reverse();
    }
    out
}

/// The 4-point identity of `inv_identity4_1d_c`.
#[inline(always)]
fn identity4<S: Simd, W: Int32<S>>(v: W) -> W {
    v + ((v * 1697 + 2048) >> 12)
}

/// The 8-point identity of `inv_identity8_1d_c`.
#[inline(always)]
fn identity8<S: Simd, W: Int32<S>>(v: W) -> W {
    v * 2
}

/// The 32-point identity of `inv_identity32_1d_c`.
#[inline(always)]
fn identity32<S: Simd, W: Int32<S>>(v: W) -> W {
    v * 4
}

/// The 16-point identity of `inv_identity16_1d_c`.
#[inline(always)]
fn identity16<S: Simd, W: Int32<S>>(v: W) -> W {
    v * 2 + ((v * 1697 + 1024) >> 11)
}

// Arrays of vectors are built and changed with loops, not with
// `array::map` or `array::from_fn`: those may stay calls, and a call leaves
// the `vectorize` region and its target features (AVX2 on x86-64).

/// `f` applied to each vector of `c`.
#[inline(always)]
pub fn each<A: Copy, B: Copy, const N: usize>(c: [A; N], f: impl Fn(A) -> B) -> [B; N] {
    let mut out = [f(c[0]); N];
    for i in 1..N {
        out[i] = f(c[i]);
    }
    out
}

/// Stored values as operands of products.
#[inline(always)]
pub fn wide<S: Simd, V: Lanes<S>, const N: usize>(c: [V; N]) -> [V::Wide; N] {
    each(c, V::wide)
}

/// The inputs `0, 2, 4, ...`: the half the C passes on with `stride << 1`.
#[inline(always)]
fn evens<V: Copy, const N: usize, const HALF: usize>(c: [V; N]) -> [V; HALF] {
    let mut half = [c[0]; HALF];
    for (i, v) in half.iter_mut().enumerate() {
        *v = c[2 * i];
    }
    half
}

/// The inputs `1, 3, 5, ...`.
#[inline(always)]
fn odds<V: Copy, const N: usize, const HALF: usize>(c: [V; N]) -> [V; HALF] {
    let mut half = [c[1]; HALF];
    for (i, v) in half.iter_mut().enumerate() {
        *v = c[2 * i + 1];
    }
    half
}

/// `dav1d_inv_wht4_1d_c` (lossless blocks; no clipping, as in the C), on
/// 32-bit lanes.
#[inline(always)]
pub fn wht4<S: Simd, V: Int32<S>>(c: [V; 4]) -> [V; 4] {
    let [in0, in1, in2, in3] = c;
    let t0 = in0 + in1;
    let t2 = in2 - in3;
    let t4 = (t0 - t2) >> 1;
    let t3 = t4 - in3;
    let t1 = t4 - in1;
    [t0 - t3, t3, t1, t2 + t1]
}

/// `inv_dct4_1d_internal_c`.
#[inline(always)]
pub fn dct4<S: Simd, V: Lanes<S>>(c: [V; 4], range: Clip<V::Wide>) -> [V; 4] {
    let clip = |x| V::clip(x, range);
    let [in0, in1, in2, in3] = wide(c);
    let t0 = ((in0 + in2) * 181 + 128) >> 8;
    let t1 = ((in0 - in2) * 181 + 128) >> 8;
    let t2 = ((in1 *  1567         - in3 * (3784 - 4096) + 2048) >> 12) - in3;
    let t3 = ((in1 * (3784 - 4096) + in3 *  1567         + 2048) >> 12) + in1;
    [
        clip(t0 + t3), clip(t1 + t2), clip(t1 - t2), clip(t0 - t3),
    ]
}

/// `inv_dct8_1d_internal_c`.
#[inline(always)]
pub fn dct8<S: Simd, V: Lanes<S>>(c: [V; 8], range: Clip<V::Wide>) -> [V; 8] {
    let clip = |x| V::clip(x, range);
    let add = |a: V, b| a.clip_add(b, range);
    let sub = |a: V, b| a.clip_sub(b, range);
    let w = V::wide;
    let [t0, t1, t2, t3] = dct4(evens(c), range);
    let [in1, in3, in5, in7] = wide(odds(c));
    let t4a = ((in1 *   799         - in7 * (4017 - 4096) + 2048) >> 12) - in7;
    let t5a =  (in5 *  1703         - in3 *  1138         + 1024) >> 11;
    let t6a =  (in5 *  1138         + in3 *  1703         + 1024) >> 11;
    let t7a = ((in1 * (4017 - 4096) + in7 *  799          + 2048) >> 12) + in1;
    let t4  = clip(t4a + t5a);
    let t5a = clip(t4a - t5a);
    let t7  = clip(t7a + t6a);
    let t6a = clip(t7a - t6a);
    let t5  = ((w(t6a) - w(t5a)) * 181 + 128) >> 8;
    let t6  = ((w(t6a) + w(t5a)) * 181 + 128) >> 8;
    [
        add(t0, t7), clip(w(t1) + t6), clip(w(t2) + t5), add(t3, t4),
        sub(t3, t4), clip(w(t2) - t5), clip(w(t1) - t6), sub(t0, t7),
    ]
}

/// `inv_dct16_1d_internal_c`.
#[inline(always)]
pub fn dct16<S: Simd, V: Lanes<S>>(c: [V; 16], range: Clip<V::Wide>) -> [V; 16] {
    let clip = |x| V::clip(x, range);
    let add = |a: V, b| a.clip_add(b, range);
    let sub = |a: V, b| a.clip_sub(b, range);
    let w = V::wide;
    let [t0, t1, t2, t3, t4, t5, t6, t7] = dct8(evens(c), range);
    let [in1, in3, in5, in7, in9, in11, in13, in15] = wide(odds(c));
    let t8a  = ((in1  *   401         - in15 * (4076 - 4096) + 2048) >> 12) - in15;
    let t9a  =  (in9  *  1583         - in7  *  1299         + 1024) >> 11;
    let t10a = ((in5  *  1931         - in11 * (3612 - 4096) + 2048) >> 12) - in11;
    let t11a = ((in13 * (3920 - 4096) - in3  *  1189         + 2048) >> 12) + in13;
    let t12a = ((in13 *  1189         + in3  * (3920 - 4096) + 2048) >> 12) + in3;
    let t13a = ((in5  * (3612 - 4096) + in11 *  1931         + 2048) >> 12) + in5;
    let t14a =  (in9  *  1299         + in7  *  1583         + 1024) >> 11;
    let t15a = ((in1  * (4076 - 4096) + in15 *   401         + 2048) >> 12) + in1;
    let t8  = clip(t8a  + t9a);
    let t9  = clip(t8a  - t9a);
    let t10 = clip(t11a - t10a);
    let t11 = clip(t11a + t10a);
    let t12 = clip(t12a + t13a);
    let t13 = clip(t12a - t13a);
    let t14 = clip(t15a - t14a);
    let t15 = clip(t15a + t14a);
    let t9a  = ((  w(t14) *  1567         - w(t9)  * (3784 - 4096)  + 2048) >> 12) - w(t9);
    let t14a = ((  w(t14) * (3784 - 4096) + w(t9)  *  1567          + 2048) >> 12) + w(t14);
    let t10a = ((-(w(t13) * (3784 - 4096) + w(t10) *  1567)         + 2048) >> 12) - w(t13);
    let t13a = ((  w(t13) *  1567         - w(t10) * (3784 - 4096)  + 2048) >> 12) - w(t10);
    let t8a  = add(t8, t11);
    let t9   = clip(t9a  + t10a);
    let t10  = clip(t9a  - t10a);
    let t11a = sub(t8, t11);
    let t12a = sub(t15, t12);
    let t13  = clip(t14a - t13a);
    let t14  = clip(t14a + t13a);
    let t15a = add(t15, t12);
    let t10a = ((w(t13)  - w(t10))  * 181 + 128) >> 8;
    let t13a = ((w(t13)  + w(t10))  * 181 + 128) >> 8;
    let t11  = ((w(t12a) - w(t11a)) * 181 + 128) >> 8;
    let t12  = ((w(t12a) + w(t11a)) * 181 + 128) >> 8;
    [
        add(t0, t15a), add(t1, t14), clip(w(t2) + t13a), clip(w(t3) + t12),
        clip(w(t4) + t11), clip(w(t5) + t10a), add(t6, t9), add(t7, t8a),
        sub(t7, t8a), sub(t6, t9), clip(w(t5) - t10a), clip(w(t4) - t11),
        clip(w(t3) - t12), clip(w(t2) - t13a), sub(t1, t14), sub(t0, t15a),
    ]
}

/// `inv_dct32_1d_internal_c`.
#[inline(always)]
pub fn dct32<S: Simd, V: Lanes<S>>(c: [V; 32], range: Clip<V::Wide>) -> [V; 32] {
    let clip = |x| V::clip(x, range);
    let add = |a: V, b| a.clip_add(b, range);
    let sub = |a: V, b| a.clip_sub(b, range);
    let w = V::wide;
    let [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15] = dct16(evens(c), range);
    let [in1, in3, in5, in7, in9, in11, in13, in15, in17, in19, in21, in23, in25, in27, in29, in31] = wide(odds(c));
    let t16a = ((in1  *   201         - in31 * (4091 - 4096) + 2048) >> 12) - in31;
    let t17a = ((in17 * (3035 - 4096) - in15 *  2751         + 2048) >> 12) + in17;
    let t18a = ((in9  *  1751         - in23 * (3703 - 4096) + 2048) >> 12) - in23;
    let t19a = ((in25 * (3857 - 4096) - in7  *  1380         + 2048) >> 12) + in25;
    let t20a = ((in5  *   995         - in27 * (3973 - 4096) + 2048) >> 12) - in27;
    let t21a = ((in21 * (3513 - 4096) - in11 *  2106         + 2048) >> 12) + in21;
    let t22a =  (in13 *  1220         - in19 *  1645         + 1024) >> 11;
    let t23a = ((in29 * (4052 - 4096) - in3  *   601         + 2048) >> 12) + in29;
    let t24a = ((in29 *   601         + in3  * (4052 - 4096) + 2048) >> 12) + in3;
    let t25a =  (in13 *  1645         + in19 *  1220         + 1024) >> 11;
    let t26a = ((in21 *  2106         + in11 * (3513 - 4096) + 2048) >> 12) + in11;
    let t27a = ((in5  * (3973 - 4096) + in27 *   995         + 2048) >> 12) + in5;
    let t28a = ((in25 *  1380         + in7  * (3857 - 4096) + 2048) >> 12) + in7;
    let t29a = ((in9  * (3703 - 4096) + in23 *  1751         + 2048) >> 12) + in9;
    let t30a = ((in17 *  2751         + in15 * (3035 - 4096) + 2048) >> 12) + in15;
    let t31a = ((in1  * (4091 - 4096) + in31 *   201         + 2048) >> 12) + in1;
    let t16 = clip(t16a + t17a);
    let t17 = clip(t16a - t17a);
    let t18 = clip(t19a - t18a);
    let t19 = clip(t19a + t18a);
    let t20 = clip(t20a + t21a);
    let t21 = clip(t20a - t21a);
    let t22 = clip(t23a - t22a);
    let t23 = clip(t23a + t22a);
    let t24 = clip(t24a + t25a);
    let t25 = clip(t24a - t25a);
    let t26 = clip(t27a - t26a);
    let t27 = clip(t27a + t26a);
    let t28 = clip(t28a + t29a);
    let t29 = clip(t28a - t29a);
    let t30 = clip(t31a - t30a);
    let t31 = clip(t31a + t30a);
    let t17a = ((  w(t30) *   799         - w(t17) * (4017 - 4096)  + 2048) >> 12) - w(t17);
    let t30a = ((  w(t30) * (4017 - 4096) + w(t17) *   799          + 2048) >> 12) + w(t30);
    let t18a = ((-(w(t29) * (4017 - 4096) + w(t18) *   799)         + 2048) >> 12) - w(t29);
    let t29a = ((  w(t29) *   799         - w(t18) * (4017 - 4096)  + 2048) >> 12) - w(t18);
    let t21a =  (  w(t26) *  1703         - w(t21) *  1138          + 1024) >> 11;
    let t26a =  (  w(t26) *  1138         + w(t21) *  1703          + 1024) >> 11;
    let t22a =  (-(w(t25) *  1138         + w(t22) *  1703) + 1024) >> 11;
    let t25a =  (  w(t25) *  1703         - w(t22) *  1138          + 1024) >> 11;
    let t16a = add(t16, t19);
    let t17  = clip(t17a + t18a);
    let t18  = clip(t17a - t18a);
    let t19a = sub(t16, t19);
    let t20a = sub(t23, t20);
    let t21  = clip(t22a - t21a);
    let t22  = clip(t22a + t21a);
    let t23a = add(t23, t20);
    let t24a = add(t24, t27);
    let t25  = clip(t25a + t26a);
    let t26  = clip(t25a - t26a);
    let t27a = sub(t24, t27);
    let t28a = sub(t31, t28);
    let t29  = clip(t30a - t29a);
    let t30  = clip(t30a + t29a);
    let t31a = add(t31, t28);
    let t18a = ((  w(t29)  *  1567         - w(t18)  * (3784 - 4096)  + 2048) >> 12) - w(t18);
    let t29a = ((  w(t29)  * (3784 - 4096) + w(t18)  *  1567          + 2048) >> 12) + w(t29);
    let t19  = ((  w(t28a) *  1567         - w(t19a) * (3784 - 4096)  + 2048) >> 12) - w(t19a);
    let t28  = ((  w(t28a) * (3784 - 4096) + w(t19a) *  1567          + 2048) >> 12) + w(t28a);
    let t20  = ((-(w(t27a) * (3784 - 4096) + w(t20a) *  1567)         + 2048) >> 12) - w(t27a);
    let t27  = ((  w(t27a) *  1567         - w(t20a) * (3784 - 4096)  + 2048) >> 12) - w(t20a);
    let t21a = ((-(w(t26)  * (3784 - 4096) + w(t21)  *  1567)         + 2048) >> 12) - w(t26);
    let t26a = ((  w(t26)  *  1567         - w(t21)  * (3784 - 4096)  + 2048) >> 12) - w(t21);
    let t16  = add(t16a, t23a);
    let t17a = add(t17, t22);
    let t18  = clip(t18a + t21a);
    let t19a = clip(t19  + t20);
    let t20a = clip(t19  - t20);
    let t21  = clip(t18a - t21a);
    let t22a = sub(t17, t22);
    let t23  = sub(t16a, t23a);
    let t24  = sub(t31a, t24a);
    let t25a = sub(t30, t25);
    let t26  = clip(t29a - t26a);
    let t27a = clip(t28  - t27);
    let t28a = clip(t28  + t27);
    let t29  = clip(t29a + t26a);
    let t30a = add(t30, t25);
    let t31  = add(t31a, t24a);
    let t20  = ((w(t27a) - w(t20a)) * 181 + 128) >> 8;
    let t27  = ((w(t27a) + w(t20a)) * 181 + 128) >> 8;
    let t21a = ((w(t26)  - w(t21)) * 181 + 128) >> 8;
    let t26a = ((w(t26)  + w(t21)) * 181 + 128) >> 8;
    let t22  = ((w(t25a) - w(t22a)) * 181 + 128) >> 8;
    let t25  = ((w(t25a) + w(t22a)) * 181 + 128) >> 8;
    let t23a = ((w(t24)  - w(t23)) * 181 + 128) >> 8;
    let t24a = ((w(t24)  + w(t23)) * 181 + 128) >> 8;
    [
        add(t0, t31), add(t1, t30a), add(t2, t29), add(t3, t28a),
        clip(w(t4)  + t27), clip(w(t5)  + t26a), clip(w(t6)  + t25), clip(w(t7)  + t24a),
        clip(w(t8)  + t23a), clip(w(t9)  + t22), clip(w(t10) + t21a), clip(w(t11) + t20),
        add(t12, t19a), add(t13, t18), add(t14, t17a), add(t15, t16),
        sub(t15, t16), sub(t14, t17a), sub(t13, t18), sub(t12, t19a),
        clip(w(t11) - t20), clip(w(t10) - t21a), clip(w(t9)  - t22), clip(w(t8)  - t23a),
        clip(w(t7)  - t24a), clip(w(t6)  - t25), clip(w(t5)  - t26a), clip(w(t4)  - t27),
        sub(t3, t28a), sub(t2, t29), sub(t1, t30a), sub(t0, t31),
    ]
}

/// `inv_dct64_1d_c`: of the 64 inputs only the first 32 can be nonzero
/// (AV1 codes at most 32x32 coefficients), so it takes those; the 32-point
/// DCT of the even inputs gets zeros for the upper half (module documentation).
#[inline(always)]
pub fn dct64<S: Simd, V: Lanes<S>>(c: [V; 32], range: Clip<V::Wide>) -> [V; 64] {
    let clip = |x| V::clip(x, range);
    let add = |a: V, b| a.clip_add(b, range);
    let sub = |a: V, b| a.clip_sub(b, range);
    let w = V::wide;
    let zero = V::splat(range.min.token(), Default::default());
    let mut even = [zero; 32];
    even[..16].copy_from_slice(&evens::<_, 32, 16>(c));
    let [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31] = dct32(even, range);
    let [in1, in3, in5, in7, in9, in11, in13, in15, in17, in19, in21, in23, in25, in27, in29, in31] = wide(odds(c));
    let t32a = (in1  *   101 + 2048) >> 12;
    let t33a = (in31 * -2824 + 2048) >> 12;
    let t34a = (in17 *  1660 + 2048) >> 12;
    let t35a = (in15 * -1474 + 2048) >> 12;
    let t36a = (in9  *   897 + 2048) >> 12;
    let t37a = (in23 * -2191 + 2048) >> 12;
    let t38a = (in25 *  2359 + 2048) >> 12;
    let t39a = (in7  *  -700 + 2048) >> 12;
    let t40a = (in5  *   501 + 2048) >> 12;
    let t41a = (in27 * -2520 + 2048) >> 12;
    let t42a = (in21 *  2019 + 2048) >> 12;
    let t43a = (in11 * -1092 + 2048) >> 12;
    let t44a = (in13 *  1285 + 2048) >> 12;
    let t45a = (in19 * -1842 + 2048) >> 12;
    let t46a = (in29 *  2675 + 2048) >> 12;
    let t47a = (in3  *  -301 + 2048) >> 12;
    let t48a = (in3  *  4085 + 2048) >> 12;
    let t49a = (in29 *  3102 + 2048) >> 12;
    let t50a = (in19 *  3659 + 2048) >> 12;
    let t51a = (in13 *  3889 + 2048) >> 12;
    let t52a = (in11 *  3948 + 2048) >> 12;
    let t53a = (in21 *  3564 + 2048) >> 12;
    let t54a = (in27 *  3229 + 2048) >> 12;
    let t55a = (in5  *  4065 + 2048) >> 12;
    let t56a = (in7  *  4036 + 2048) >> 12;
    let t57a = (in25 *  3349 + 2048) >> 12;
    let t58a = (in23 *  3461 + 2048) >> 12;
    let t59a = (in9  *  3996 + 2048) >> 12;
    let t60a = (in15 *  3822 + 2048) >> 12;
    let t61a = (in17 *  3745 + 2048) >> 12;
    let t62a = (in31 *  2967 + 2048) >> 12;
    let t63a = (in1  *  4095 + 2048) >> 12;
    let t32 = clip(t32a + t33a);
    let t33 = clip(t32a - t33a);
    let t34 = clip(t35a - t34a);
    let t35 = clip(t35a + t34a);
    let t36 = clip(t36a + t37a);
    let t37 = clip(t36a - t37a);
    let t38 = clip(t39a - t38a);
    let t39 = clip(t39a + t38a);
    let t40 = clip(t40a + t41a);
    let t41 = clip(t40a - t41a);
    let t42 = clip(t43a - t42a);
    let t43 = clip(t43a + t42a);
    let t44 = clip(t44a + t45a);
    let t45 = clip(t44a - t45a);
    let t46 = clip(t47a - t46a);
    let t47 = clip(t47a + t46a);
    let t48 = clip(t48a + t49a);
    let t49 = clip(t48a - t49a);
    let t50 = clip(t51a - t50a);
    let t51 = clip(t51a + t50a);
    let t52 = clip(t52a + t53a);
    let t53 = clip(t52a - t53a);
    let t54 = clip(t55a - t54a);
    let t55 = clip(t55a + t54a);
    let t56 = clip(t56a + t57a);
    let t57 = clip(t56a - t57a);
    let t58 = clip(t59a - t58a);
    let t59 = clip(t59a + t58a);
    let t60 = clip(t60a + t61a);
    let t61 = clip(t60a - t61a);
    let t62 = clip(t63a - t62a);
    let t63 = clip(t63a + t62a);
    let t33a = ((w(t33) * (4096 - 4076) + w(t62) *   401         + 2048) >> 12) - w(t33);
    let t34a = ((w(t34) *  -401         + w(t61) * (4096 - 4076) + 2048) >> 12) - w(t61);
    let t37a =  (w(t37) * -1299         + w(t58) *  1583         + 1024) >> 11;
    let t38a =  (w(t38) * -1583         + w(t57) * -1299         + 1024) >> 11;
    let t41a = ((w(t41) * (4096 - 3612) + w(t54) *  1931         + 2048) >> 12) - w(t41);
    let t42a = ((w(t42) * -1931         + w(t53) * (4096 - 3612) + 2048) >> 12) - w(t53);
    let t45a = ((w(t45) * -1189         + w(t50) * (3920 - 4096) + 2048) >> 12) + w(t50);
    let t46a = ((w(t46) * (4096 - 3920) + w(t49) * -1189         + 2048) >> 12) - w(t46);
    let t49a = ((w(t46) * -1189         + w(t49) * (3920 - 4096) + 2048) >> 12) + w(t49);
    let t50a = ((w(t45) * (3920 - 4096) + w(t50) *  1189         + 2048) >> 12) + w(t45);
    let t53a = ((w(t42) * (4096 - 3612) + w(t53) *  1931         + 2048) >> 12) - w(t42);
    let t54a = ((w(t41) *  1931         + w(t54) * (3612 - 4096) + 2048) >> 12) + w(t54);
    let t57a =  (w(t38) * -1299         + w(t57) *  1583         + 1024) >> 11;
    let t58a =  (w(t37) *  1583         + w(t58) *  1299         + 1024) >> 11;
    let t61a = ((w(t34) * (4096 - 4076) + w(t61) *   401         + 2048) >> 12) - w(t34);
    let t62a = ((w(t33) *   401         + w(t62) * (4076 - 4096) + 2048) >> 12) + w(t62);
    let t32a = add(t32, t35);
    let t33  = clip(t33a + t34a);
    let t34  = clip(t33a - t34a);
    let t35a = sub(t32, t35);
    let t36a = sub(t39, t36);
    let t37  = clip(t38a - t37a);
    let t38  = clip(t38a + t37a);
    let t39a = add(t39, t36);
    let t40a = add(t40, t43);
    let t41  = clip(t41a + t42a);
    let t42  = clip(t41a - t42a);
    let t43a = sub(t40, t43);
    let t44a = sub(t47, t44);
    let t45  = clip(t46a - t45a);
    let t46  = clip(t46a + t45a);
    let t47a = add(t47, t44);
    let t48a = add(t48, t51);
    let t49  = clip(t49a + t50a);
    let t50  = clip(t49a - t50a);
    let t51a = sub(t48, t51);
    let t52a = sub(t55, t52);
    let t53  = clip(t54a - t53a);
    let t54  = clip(t54a + t53a);
    let t55a = add(t55, t52);
    let t56a = add(t56, t59);
    let t57  = clip(t57a + t58a);
    let t58  = clip(t57a - t58a);
    let t59a = sub(t56, t59);
    let t60a = sub(t63, t60);
    let t61  = clip(t62a - t61a);
    let t62  = clip(t62a + t61a);
    let t63a = add(t63, t60);
    let t34a = ((w(t34)  * (4096 - 4017) + w(t61)  *   799         + 2048) >> 12) - w(t34);
    let t35  = ((w(t35a) * (4096 - 4017) + w(t60a) *   799         + 2048) >> 12) - w(t35a);
    let t36  = ((w(t36a) *  -799         + w(t59a) * (4096 - 4017) + 2048) >> 12) - w(t59a);
    let t37a = ((w(t37)  *  -799         + w(t58)  * (4096 - 4017) + 2048) >> 12) - w(t58);
    let t42a =  (w(t42)  * -1138         + w(t53)  *  1703         + 1024) >> 11;
    let t43  =  (w(t43a) * -1138         + w(t52a) *  1703         + 1024) >> 11;
    let t44  =  (w(t44a) * -1703         + w(t51a) * -1138         + 1024) >> 11;
    let t45a =  (w(t45)  * -1703         + w(t50)  * -1138         + 1024) >> 11;
    let t50a =  (w(t45)  * -1138         + w(t50)  *  1703         + 1024) >> 11;
    let t51  =  (w(t44a) * -1138         + w(t51a) *  1703         + 1024) >> 11;
    let t52  =  (w(t43a) *  1703         + w(t52a) *  1138         + 1024) >> 11;
    let t53a =  (w(t42)  *  1703         + w(t53)  *  1138         + 1024) >> 11;
    let t58a = ((w(t37)  * (4096 - 4017) + w(t58)  *   799         + 2048) >> 12) - w(t37);
    let t59  = ((w(t36a) * (4096 - 4017) + w(t59a) *   799         + 2048) >> 12) - w(t36a);
    let t60  = ((w(t35a) *   799         + w(t60a) * (4017 - 4096) + 2048) >> 12) + w(t60a);
    let t61a = ((w(t34)  *   799         + w(t61)  * (4017 - 4096) + 2048) >> 12) + w(t61);
    let t32  = add(t32a, t39a);
    let t33a = add(t33, t38);
    let t34  = clip(t34a + t37a);
    let t35a = clip(t35  + t36);
    let t36a = clip(t35  - t36);
    let t37  = clip(t34a - t37a);
    let t38a = sub(t33, t38);
    let t39  = sub(t32a, t39a);
    let t40  = sub(t47a, t40a);
    let t41a = sub(t46, t41);
    let t42  = clip(t45a - t42a);
    let t43a = clip(t44  - t43);
    let t44a = clip(t44  + t43);
    let t45  = clip(t45a + t42a);
    let t46a = add(t46, t41);
    let t47  = add(t47a, t40a);
    let t48  = add(t48a, t55a);
    let t49a = add(t49, t54);
    let t50  = clip(t50a + t53a);
    let t51a = clip(t51  + t52);
    let t52a = clip(t51  - t52);
    let t53  = clip(t50a - t53a);
    let t54a = sub(t49, t54);
    let t55  = sub(t48a, t55a);
    let t56  = sub(t63a, t56a);
    let t57a = sub(t62, t57);
    let t58  = clip(t61a - t58a);
    let t59a = clip(t60  - t59);
    let t60a = clip(t60  + t59);
    let t61  = clip(t61a + t58a);
    let t62a = add(t62, t57);
    let t63  = add(t63a, t56a);
    let t36  = ((w(t36a) * (4096 - 3784) + w(t59a) *  1567         + 2048) >> 12) - w(t36a);
    let t37a = ((w(t37)  * (4096 - 3784) + w(t58)  *  1567         + 2048) >> 12) - w(t37);
    let t38  = ((w(t38a) * (4096 - 3784) + w(t57a) *  1567         + 2048) >> 12) - w(t38a);
    let t39a = ((w(t39)  * (4096 - 3784) + w(t56)  *  1567         + 2048) >> 12) - w(t39);
    let t40a = ((w(t40)  * -1567         + w(t55)  * (4096 - 3784) + 2048) >> 12) - w(t55);
    let t41  = ((w(t41a) * -1567         + w(t54a) * (4096 - 3784) + 2048) >> 12) - w(t54a);
    let t42a = ((w(t42)  * -1567         + w(t53)  * (4096 - 3784) + 2048) >> 12) - w(t53);
    let t43  = ((w(t43a) * -1567         + w(t52a) * (4096 - 3784) + 2048) >> 12) - w(t52a);
    let t52  = ((w(t43a) * (4096 - 3784) + w(t52a) *  1567         + 2048) >> 12) - w(t43a);
    let t53a = ((w(t42)  * (4096 - 3784) + w(t53)  *  1567         + 2048) >> 12) - w(t42);
    let t54  = ((w(t41a) * (4096 - 3784) + w(t54a) *  1567         + 2048) >> 12) - w(t41a);
    let t55a = ((w(t40)  * (4096 - 3784) + w(t55)  *  1567         + 2048) >> 12) - w(t40);
    let t56a = ((w(t39)  *  1567         + w(t56)  * (3784 - 4096) + 2048) >> 12) + w(t56);
    let t57  = ((w(t38a) *  1567         + w(t57a) * (3784 - 4096) + 2048) >> 12) + w(t57a);
    let t58a = ((w(t37)  *  1567         + w(t58)  * (3784 - 4096) + 2048) >> 12) + w(t58);
    let t59  = ((w(t36a) *  1567         + w(t59a) * (3784 - 4096) + 2048) >> 12) + w(t59a);
    let t32a = add(t32, t47);
    let t33  = add(t33a, t46a);
    let t34a = add(t34, t45);
    let t35  = add(t35a, t44a);
    let t36a = clip(t36  + t43);
    let t37  = clip(t37a + t42a);
    let t38a = clip(t38  + t41);
    let t39  = clip(t39a + t40a);
    let t40  = clip(t39a - t40a);
    let t41a = clip(t38  - t41);
    let t42  = clip(t37a - t42a);
    let t43a = clip(t36  - t43);
    let t44  = sub(t35a, t44a);
    let t45a = sub(t34, t45);
    let t46  = sub(t33a, t46a);
    let t47a = sub(t32, t47);
    let t48a = sub(t63, t48);
    let t49  = sub(t62a, t49a);
    let t50a = sub(t61, t50);
    let t51  = sub(t60a, t51a);
    let t52a = clip(t59  - t52);
    let t53  = clip(t58a - t53a);
    let t54a = clip(t57  - t54);
    let t55  = clip(t56a - t55a);
    let t56  = clip(t56a + t55a);
    let t57a = clip(t57  + t54);
    let t58  = clip(t58a + t53a);
    let t59a = clip(t59  + t52);
    let t60  = add(t60a, t51a);
    let t61a = add(t61, t50);
    let t62  = add(t62a, t49a);
    let t63a = add(t63, t48);
    let t40a = ((w(t55)  - w(t40)) * 181 + 128) >> 8;
    let t41  = ((w(t54a) - w(t41a)) * 181 + 128) >> 8;
    let t42a = ((w(t53)  - w(t42)) * 181 + 128) >> 8;
    let t43  = ((w(t52a) - w(t43a)) * 181 + 128) >> 8;
    let t44a = ((w(t51)  - w(t44)) * 181 + 128) >> 8;
    let t45  = ((w(t50a) - w(t45a)) * 181 + 128) >> 8;
    let t46a = ((w(t49)  - w(t46)) * 181 + 128) >> 8;
    let t47  = ((w(t48a) - w(t47a)) * 181 + 128) >> 8;
    let t48  = ((w(t47a) + w(t48a)) * 181 + 128) >> 8;
    let t49a = ((w(t46)  + w(t49)) * 181 + 128) >> 8;
    let t50  = ((w(t45a) + w(t50a)) * 181 + 128) >> 8;
    let t51a = ((w(t44)  + w(t51)) * 181 + 128) >> 8;
    let t52  = ((w(t43a) + w(t52a)) * 181 + 128) >> 8;
    let t53a = ((w(t42)  + w(t53)) * 181 + 128) >> 8;
    let t54  = ((w(t41a) + w(t54a)) * 181 + 128) >> 8;
    let t55a = ((w(t40)  + w(t55)) * 181 + 128) >> 8;
    [
        add(t0, t63a), add(t1, t62), add(t2, t61a), add(t3, t60),
        add(t4, t59a), add(t5, t58), add(t6, t57a), add(t7, t56),
        clip(w(t8)  + t55a), clip(w(t9)  + t54), clip(w(t10) + t53a), clip(w(t11) + t52),
        clip(w(t12) + t51a), clip(w(t13) + t50), clip(w(t14) + t49a), clip(w(t15) + t48),
        clip(w(t16) + t47), clip(w(t17) + t46a), clip(w(t18) + t45), clip(w(t19) + t44a),
        clip(w(t20) + t43), clip(w(t21) + t42a), clip(w(t22) + t41), clip(w(t23) + t40a),
        add(t24, t39), add(t25, t38a), add(t26, t37), add(t27, t36a),
        add(t28, t35), add(t29, t34a), add(t30, t33), add(t31, t32a),
        sub(t31, t32a), sub(t30, t33), sub(t29, t34a), sub(t28, t35),
        sub(t27, t36a), sub(t26, t37), sub(t25, t38a), sub(t24, t39),
        clip(w(t23) - t40a), clip(w(t22) - t41), clip(w(t21) - t42a), clip(w(t20) - t43),
        clip(w(t19) - t44a), clip(w(t18) - t45), clip(w(t17) - t46a), clip(w(t16) - t47),
        clip(w(t15) - t48), clip(w(t14) - t49a), clip(w(t13) - t50), clip(w(t12) - t51a),
        clip(w(t11) - t52), clip(w(t10) - t53a), clip(w(t9)  - t54), clip(w(t8)  - t55a),
        sub(t7, t56), sub(t6, t57a), sub(t5, t58), sub(t4, t59a),
        sub(t3, t60), sub(t2, t61a), sub(t1, t62), sub(t0, t63a),
    ]
}

/// `inv_adst4_1d_internal_c` (no clipping, as in the C).
#[inline(always)]
pub fn adst4<S: Simd, V: Lanes<S>>(c: [V; 4]) -> [V::Wide; 4] {
    let [in0, in1, in2, in3] = wide(c);
    let out0 = ((in0 * 1321 + in2 * (3803 - 4096) + in3 * (2482 - 4096) + in1 * (3344 - 4096) + 2048) >> 12) + in2 + in3 + in1;
    let out1 = ((in0 * (2482 - 4096) - in2 * 1321 - in3 * (3803 - 4096) + in1 * (3344 - 4096) + 2048) >> 12) + in0 - in3 + in1;
    let out2 = ((in0 - in2 + in3) * 209 + 128) >> 8;
    let out3 = ((in0 * (3803 - 4096) + in2 * (2482 - 4096) - in3 * 1321 - in1 * (3344 - 4096) + 2048) >> 12) + in0 + in2 - in1;
    [
        out0, out1, out2, out3,
    ]
}

/// `inv_adst8_1d_internal_c`.
#[inline(always)]
pub fn adst8<S: Simd, V: Lanes<S>>(c: [V; 8], range: Clip<V::Wide>) -> [V::Wide; 8] {
    let clip = |x| V::clip(x, range);
    let add = |a: V, b| a.clip_add(b, range);
    let sub = |a: V, b| a.clip_sub(b, range);
    let w = V::wide;
    let [in0, in1, in2, in3, in4, in5, in6, in7] = wide(c);
    let t0a = ((in7 * (4076 - 4096) + in0 * 401 + 2048) >> 12) + in7;
    let t1a = ((in7 * 401 - in0 * (4076 - 4096) + 2048) >> 12) - in0;
    let t2a = ((in5 * (3612 - 4096) + in2 * 1931 + 2048) >> 12) + in5;
    let t3a = ((in5 * 1931 - in2 * (3612 - 4096) + 2048) >> 12) - in2;
    let t4a = (in3 * 1299 + in4 * 1583 + 1024) >> 11;
    let t5a = (in3 * 1583 - in4 * 1299 + 1024) >> 11;
    let t6a = ((in1 * 1189 + in6 * (3920 - 4096) + 2048) >> 12) + in6;
    let t7a = ((in1 * (3920 - 4096) - in6 * 1189 + 2048) >> 12) + in1;
    let t0 = clip(t0a + t4a);
    let t1 = clip(t1a + t5a);
    let t2 = clip(t2a + t6a);
    let t3 = clip(t3a + t7a);
    let t4 = clip(t0a - t4a);
    let t5 = clip(t1a - t5a);
    let t6 = clip(t2a - t6a);
    let t7 = clip(t3a - t7a);
    let t4a = ((w(t4) * (3784 - 4096) + w(t5) * 1567 + 2048) >> 12) + w(t4);
    let t5a = ((w(t4) * 1567 - w(t5) * (3784 - 4096) + 2048) >> 12) - w(t5);
    let t6a = ((w(t7) * (3784 - 4096) - w(t6) * 1567 + 2048) >> 12) + w(t7);
    let t7a = ((w(t7) * 1567 + w(t6) * (3784 - 4096) + 2048) >> 12) + w(t6);
    let out0 = add(t0, t2);
    let out7 = -w(add(t1, t3));
    let t2 = sub(t0, t2);
    let t3 = sub(t1, t3);
    let out1 = -w(clip(t4a + t6a));
    let out6 = clip(t5a + t7a);
    let t6 = clip(t4a - t6a);
    let t7 = clip(t5a - t7a);
    let out3 = -(((w(t2) + w(t3)) * 181 + 128) >> 8);
    let out4 = ((w(t2) - w(t3)) * 181 + 128) >> 8;
    let out2 = ((w(t6) + w(t7)) * 181 + 128) >> 8;
    let out5 = -(((w(t6) - w(t7)) * 181 + 128) >> 8);
    [
        w(out0), out1, out2, out3,
        out4, out5, w(out6), out7,
    ]
}

/// `inv_adst16_1d_internal_c`.
#[inline(always)]
pub fn adst16<S: Simd, V: Lanes<S>>(c: [V; 16], range: Clip<V::Wide>) -> [V::Wide; 16] {
    let clip = |x| V::clip(x, range);
    let add = |a: V, b| a.clip_add(b, range);
    let sub = |a: V, b| a.clip_sub(b, range);
    let w = V::wide;
    let [in0, in1, in2, in3, in4, in5, in6, in7, in8, in9, in10, in11, in12, in13, in14, in15] = wide(c);
    let t0  = ((in15 * (4091 - 4096) + in0  *   201         + 2048) >> 12) + in15;
    let t1  = ((in15 *   201         - in0  * (4091 - 4096) + 2048) >> 12) - in0;
    let t2  = ((in13 * (3973 - 4096) + in2  *   995         + 2048) >> 12) + in13;
    let t3  = ((in13 *   995         - in2  * (3973 - 4096) + 2048) >> 12) - in2;
    let t4  = ((in11 * (3703 - 4096) + in4  *  1751         + 2048) >> 12) + in11;
    let t5  = ((in11 *  1751         - in4  * (3703 - 4096) + 2048) >> 12) - in4;
    let t6  =  (in9  *  1645         + in6  *  1220         + 1024) >> 11;
    let t7  =  (in9  *  1220         - in6  *  1645         + 1024) >> 11;
    let t8  = ((in7  *  2751         + in8  * (3035 - 4096) + 2048) >> 12) + in8;
    let t9  = ((in7  * (3035 - 4096) - in8  *  2751         + 2048) >> 12) + in7;
    let t10 = ((in5  *  2106         + in10 * (3513 - 4096) + 2048) >> 12) + in10;
    let t11 = ((in5  * (3513 - 4096) - in10 *  2106         + 2048) >> 12) + in5;
    let t12 = ((in3  *  1380         + in12 * (3857 - 4096) + 2048) >> 12) + in12;
    let t13 = ((in3  * (3857 - 4096) - in12 *  1380         + 2048) >> 12) + in3;
    let t14 = ((in1  *   601         + in14 * (4052 - 4096) + 2048) >> 12) + in14;
    let t15 = ((in1  * (4052 - 4096) - in14 *   601         + 2048) >> 12) + in1;
    let t0a  = clip(t0 + t8);
    let t1a  = clip(t1 + t9);
    let t2a  = clip(t2 + t10);
    let t3a  = clip(t3 + t11);
    let t4a  = clip(t4 + t12);
    let t5a  = clip(t5 + t13);
    let t6a  = clip(t6 + t14);
    let t7a  = clip(t7 + t15);
    let t8a  = clip(t0 - t8);
    let t9a  = clip(t1 - t9);
    let t10a = clip(t2 - t10);
    let t11a = clip(t3 - t11);
    let t12a = clip(t4 - t12);
    let t13a = clip(t5 - t13);
    let t14a = clip(t6 - t14);
    let t15a = clip(t7 - t15);
    let t8   = ((w(t8a)  * (4017 - 4096) + w(t9a)  *   799         + 2048) >> 12) + w(t8a);
    let t9   = ((w(t8a)  *   799         - w(t9a)  * (4017 - 4096) + 2048) >> 12) - w(t9a);
    let t10  = ((w(t10a) *  2276         + w(t11a) * (3406 - 4096) + 2048) >> 12) + w(t11a);
    let t11  = ((w(t10a) * (3406 - 4096) - w(t11a) *  2276         + 2048) >> 12) + w(t10a);
    let t12  = ((w(t13a) * (4017 - 4096) - w(t12a) *   799         + 2048) >> 12) + w(t13a);
    let t13  = ((w(t13a) *   799         + w(t12a) * (4017 - 4096) + 2048) >> 12) + w(t12a);
    let t14  = ((w(t15a) *  2276         - w(t14a) * (3406 - 4096) + 2048) >> 12) - w(t14a);
    let t15  = ((w(t15a) * (3406 - 4096) + w(t14a) *  2276         + 2048) >> 12) + w(t15a);
    let t0   = add(t0a, t4a);
    let t1   = add(t1a, t5a);
    let t2   = add(t2a, t6a);
    let t3   = add(t3a, t7a);
    let t4   = sub(t0a, t4a);
    let t5   = sub(t1a, t5a);
    let t6   = sub(t2a, t6a);
    let t7   = sub(t3a, t7a);
    let t8a  = clip(t8  + t12);
    let t9a  = clip(t9  + t13);
    let t10a = clip(t10 + t14);
    let t11a = clip(t11 + t15);
    let t12a = clip(t8  - t12);
    let t13a = clip(t9  - t13);
    let t14a = clip(t10 - t14);
    let t15a = clip(t11 - t15);
    let t4a  = ((w(t4)   * (3784 - 4096) + w(t5)   *  1567         + 2048) >> 12) + w(t4);
    let t5a  = ((w(t4)   *  1567         - w(t5)   * (3784 - 4096) + 2048) >> 12) - w(t5);
    let t6a  = ((w(t7)   * (3784 - 4096) - w(t6)   *  1567         + 2048) >> 12) + w(t7);
    let t7a  = ((w(t7)   *  1567         + w(t6)   * (3784 - 4096) + 2048) >> 12) + w(t6);
    let t12  = ((w(t12a) * (3784 - 4096) + w(t13a) *  1567         + 2048) >> 12) + w(t12a);
    let t13  = ((w(t12a) *  1567         - w(t13a) * (3784 - 4096) + 2048) >> 12) - w(t13a);
    let t14  = ((w(t15a) * (3784 - 4096) - w(t14a) *  1567         + 2048) >> 12) + w(t15a);
    let t15  = ((w(t15a) *  1567         + w(t14a) * (3784 - 4096) + 2048) >> 12) + w(t14a);
    let out0 = add(t0, t2);
    let out15 = -w(add(t1, t3));
    let t2a  = sub(t0, t2);
    let t3a  = sub(t1, t3);
    let out3 = -w(clip(t4a + t6a));
    let out12 = clip(t5a + t7a);
    let t6   = clip(t4a - t6a);
    let t7   = clip(t5a - t7a);
    let out1 = -w(add(t8a, t10a));
    let out14 = add(t9a, t11a);
    let t10  = sub(t8a, t10a);
    let t11  = sub(t9a, t11a);
    let out2 = clip(t12 + t14);
    let out13 = -w(clip(t13 + t15));
    let t14a = clip(t12 - t14);
    let t15a = clip(t13 - t15);
    let out7 = -(((w(t2a)  + w(t3a))  * 181 + 128) >> 8);
    let out8 = ((w(t2a)  - w(t3a))  * 181 + 128) >> 8;
    let out4 = ((w(t6)   + w(t7))   * 181 + 128) >> 8;
    let out11 = -(((w(t6)   - w(t7))   * 181 + 128) >> 8);
    let out6 = ((w(t10)  + w(t11))  * 181 + 128) >> 8;
    let out9 = -(((w(t10)  - w(t11))  * 181 + 128) >> 8);
    let out5 = -(((w(t14a) + w(t15a)) * 181 + 128) >> 8);
    let out10 = ((w(t14a) - w(t15a)) * 181 + 128) >> 8;
    [
        w(out0), out1, w(out2), out3,
        out4, out5, out6, out7,
        out8, out9, out10, out11,
        w(out12), out13, w(out14), out15,
    ]
}
