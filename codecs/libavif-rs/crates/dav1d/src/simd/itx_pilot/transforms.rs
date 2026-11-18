// Copyright © 2018-2019, VideoLAN and dav1d authors
// Copyright © 2018-2019, Two Orioles, LLC
// All rights reserved. BSD-2-Clause; see LICENSES/ in this crate.

//! The 1-D inverse transforms of `upstream/dav1d/src/itx_1d.c`, on vectors.
//!
//! Each lane is one independent row or column; a function takes the `N`
//! inputs of a transform as `N` vectors and returns its `N` outputs. The
//! arithmetic is the C's, statement by statement and with its names, in
//! 32-bit lanes (the C's `int`), including the C's rearranged products
//! (`(a * 1567 - b * (3784 - 4096) + 2048) >> 12) - b`), so that it stays
//! free of overflow where the C is, at every bit depth.
//!
//! The C's `tx64` variants of the 4- to 32-point DCTs (used by the 64-point
//! DCT, whose inputs 32..63 are zero) are the same formulas with the zero
//! inputs dropped: `dct64` passes zeros and the compiler drops them.

use core::ops::Neg;

use fearless_simd::prelude::*;
use fearless_simd::{i16x8, i32x4, i32x8, i64x2, u16x16, u16x8};

/// A vector of `i32` lanes the transforms run on: `i32x4` (blocks 4 wide
/// or 4 tall) or `i32x8`.
pub trait Lanes<S: Simd>: SimdInt<S, Element = i32> + Neg<Output = Self> {
    /// Transposes the square block `block` (`Self::LEN` vectors).
    fn transpose(block: &mut [Self]);

    /// The C's `CLIP`: `iclip(v, min, max)`.
    #[inline(always)]
    fn clip(self, range: Clip<Self>) -> Self {
        self.max(range.min).min(range.max)
    }

    /// Stores the lanes (each in `0..=u16::MAX`) as `u16`s.
    fn store_u16(self, out: &mut [u16]);

    /// Stores the lanes (each in `0..=u8::MAX`) as `u8`s.
    fn store_u8(self, out: &mut [u8]);

    /// The lanes (each in the range of `i16`) as the first lanes of an
    /// `i16x8`.
    fn to_i16x8(self) -> i16x8<S>;
}

impl<S: Simd> Lanes<S> for i32x4<S> {
    #[inline(always)]
    fn transpose(block: &mut [Self]) {
        let [a, b, c, d]: &mut [Self; 4] = block.try_into().unwrap();
        // [a0 b0 a1 b1], [a2 b2 a3 b3], [c0 d0 c1 d1], [c2 d2 c3 d3]
        let (ab0, ab2) = (a.zip_low(*b), a.zip_high(*b));
        let (cd0, cd2) = (c.zip_low(*d), c.zip_high(*d));
        // Pairs of lanes as one 64-bit lane: [a0 b0 c0 d0], [a1 b1 c1 d1], ...
        let pairs = |v: Self| v.bitcast::<i64x2<S>>();
        let lanes = |v: i64x2<S>| v.bitcast::<Self>();
        *a = lanes(pairs(ab0).zip_low(pairs(cd0)));
        *b = lanes(pairs(ab0).zip_high(pairs(cd0)));
        *c = lanes(pairs(ab2).zip_low(pairs(cd2)));
        *d = lanes(pairs(ab2).zip_high(pairs(cd2)));
    }

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

    #[inline(always)]
    fn to_i16x8(self) -> i16x8<S> {
        self.narrow(self)
    }
}

impl<S: Simd> Lanes<S> for i32x8<S> {
    /// As four 4x4 blocks: each quadrant transposed, the two off the
    /// diagonal exchanged.
    #[inline(always)]
    fn transpose(block: &mut [Self]) {
        // A function, not a closure: a closure called out of line would not
        // carry the target features of the `vectorize` region.
        #[inline(always)]
        fn quadrant<S: Simd>(block: &[i32x8<S>], first: usize, high: bool) -> [i32x4<S>; 4] {
            // Vectors `first..first + 4`, lanes 0..4 or (`high`) 4..8, transposed.
            let mut q = [block[first].split().0; 4];
            for (i, v) in q.iter_mut().enumerate() {
                let (low_lanes, high_lanes) = block[first + i].split();
                *v = if high { high_lanes } else { low_lanes };
            }
            i32x4::transpose(&mut q);
            q
        }
        let (rows0_lanes0, rows4_lanes0) = (quadrant(block, 0, false), quadrant(block, 4, false));
        let (rows0_lanes4, rows4_lanes4) = (quadrant(block, 0, true), quadrant(block, 4, true));
        for i in 0..4 {
            block[i] = rows0_lanes0[i].combine(rows4_lanes0[i]);
            block[i + 4] = rows0_lanes4[i].combine(rows4_lanes4[i]);
        }
    }

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

    #[inline(always)]
    fn to_i16x8(self) -> i16x8<S> {
        self.narrow(self).split().0
    }
}

/// The bounds of the C's `CLIP` (`min`, `max` of an `itx_1d_fn`).
#[derive(Clone, Copy)]
pub struct Clip<V> {
    pub min: V,
    pub max: V,
}

/// `enum Tx1dType` (src/levels.h).
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    Dct,
    Adst,
    FlipAdst,
    Identity,
}

/// The `N`-point transform `kind` (`dav1d_tx1d_fns`); for `N` = 64 only the
/// first 32 inputs are read.
#[inline(always)]
pub fn transform<S: Simd, V: Lanes<S>, const N: usize>(kind: Kind, c: [V; N], range: Clip<V>) -> [V; N] {
    // `[V; N]` as `[V; M]` where the match below has fixed N = M.
    #[inline(always)]
    fn fixed<V: Copy, const M: usize>(c: &[V]) -> [V; M] {
        c[..M].try_into().unwrap()
    }
    let mut out = match (kind, N) {
        (Kind::Dct, 4) => fixed(&dct4(fixed(&c), range)),
        (Kind::Dct, 8) => fixed(&dct8(fixed(&c), range)),
        (Kind::Dct, 16) => fixed(&dct16(fixed(&c), range)),
        (Kind::Dct, 32) => fixed(&dct32(fixed(&c), range)),
        (Kind::Dct, 64) => fixed(&dct64(fixed(&c), range)),
        (Kind::Adst | Kind::FlipAdst, 4) => fixed(&adst4(fixed(&c))),
        (Kind::Adst | Kind::FlipAdst, 8) => fixed(&adst8(fixed(&c), range)),
        (Kind::Adst | Kind::FlipAdst, 16) => fixed(&adst16(fixed(&c), range)),
        // inv_identity*_1d_c (no clipping, as in the C).
        (Kind::Identity, 4) => each(c, |v| v + ((v * 1697 + 2048) >> 12)),
        (Kind::Identity, 8) => each(c, |v| v * 2),
        (Kind::Identity, 16) => each(c, |v| v * 2 + ((v * 1697 + 1024) >> 11)),
        (Kind::Identity, 32) => each(c, |v| v * 4),
        _ => panic!("no {N}-point transform of this kind in AV1"),
    };
    // inv_flipadst*_1d_c: the ADST written from the last output up.
    if kind == Kind::FlipAdst {
        out.reverse();
    }
    out
}

// Arrays of vectors are built and changed with loops, not with
// `array::map` or `array::from_fn`: those may stay calls, and a call leaves
// the `vectorize` region and its target features (AVX2 on x86-64).

/// `f` applied to each vector of `c`.
#[inline(always)]
pub fn each<V: Copy, const N: usize>(mut c: [V; N], f: impl Fn(V) -> V) -> [V; N] {
    for v in &mut c {
        *v = f(*v);
    }
    c
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

/// `dav1d_inv_wht4_1d_c` (lossless blocks; no clipping, as in the C).
#[inline(always)]
pub fn wht4<S: Simd, V: Lanes<S>>(c: [V; 4]) -> [V; 4] {
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
pub fn dct4<S: Simd, V: Lanes<S>>(c: [V; 4], range: Clip<V>) -> [V; 4] {
    let clip = |v: V| v.clip(range);
    let [in0, in1, in2, in3] = c;
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
pub fn dct8<S: Simd, V: Lanes<S>>(c: [V; 8], range: Clip<V>) -> [V; 8] {
    let clip = |v: V| v.clip(range);
    let [t0, t1, t2, t3] = dct4(evens(c), range);
    let [in1, in3, in5, in7] = odds(c);
    let t4a = ((in1 *   799         - in7 * (4017 - 4096) + 2048) >> 12) - in7;
    let t5a =  (in5 *  1703         - in3 *  1138         + 1024) >> 11;
    let t6a =  (in5 *  1138         + in3 *  1703         + 1024) >> 11;
    let t7a = ((in1 * (4017 - 4096) + in7 *  799          + 2048) >> 12) + in1;
    let t4  = clip(t4a + t5a);
    let t5a = clip(t4a - t5a);
    let t7  = clip(t7a + t6a);
    let t6a = clip(t7a - t6a);
    let t5  = ((t6a - t5a) * 181 + 128) >> 8;
    let t6  = ((t6a + t5a) * 181 + 128) >> 8;
    [
        clip(t0 + t7), clip(t1 + t6), clip(t2 + t5), clip(t3 + t4),
        clip(t3 - t4), clip(t2 - t5), clip(t1 - t6), clip(t0 - t7),
    ]
}

/// `inv_dct16_1d_internal_c`.
#[inline(always)]
pub fn dct16<S: Simd, V: Lanes<S>>(c: [V; 16], range: Clip<V>) -> [V; 16] {
    let clip = |v: V| v.clip(range);
    let [t0, t1, t2, t3, t4, t5, t6, t7] = dct8(evens(c), range);
    let [in1, in3, in5, in7, in9, in11, in13, in15] = odds(c);
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
    let t9a  = ((  t14 *  1567         - t9  * (3784 - 4096)  + 2048) >> 12) - t9;
    let t14a = ((  t14 * (3784 - 4096) + t9  *  1567          + 2048) >> 12) + t14;
    let t10a = ((-(t13 * (3784 - 4096) + t10 *  1567)         + 2048) >> 12) - t13;
    let t13a = ((  t13 *  1567         - t10 * (3784 - 4096)  + 2048) >> 12) - t10;
    let t8a  = clip(t8   + t11);
    let t9   = clip(t9a  + t10a);
    let t10  = clip(t9a  - t10a);
    let t11a = clip(t8   - t11);
    let t12a = clip(t15  - t12);
    let t13  = clip(t14a - t13a);
    let t14  = clip(t14a + t13a);
    let t15a = clip(t15  + t12);
    let t10a = ((t13  - t10)  * 181 + 128) >> 8;
    let t13a = ((t13  + t10)  * 181 + 128) >> 8;
    let t11  = ((t12a - t11a) * 181 + 128) >> 8;
    let t12  = ((t12a + t11a) * 181 + 128) >> 8;
    [
        clip(t0 + t15a), clip(t1 + t14), clip(t2 + t13a), clip(t3 + t12),
        clip(t4 + t11), clip(t5 + t10a), clip(t6 + t9), clip(t7 + t8a),
        clip(t7 - t8a), clip(t6 - t9), clip(t5 - t10a), clip(t4 - t11),
        clip(t3 - t12), clip(t2 - t13a), clip(t1 - t14), clip(t0 - t15a),
    ]
}

/// `inv_dct32_1d_internal_c`.
#[inline(always)]
pub fn dct32<S: Simd, V: Lanes<S>>(c: [V; 32], range: Clip<V>) -> [V; 32] {
    let clip = |v: V| v.clip(range);
    let [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15] = dct16(evens(c), range);
    let [in1, in3, in5, in7, in9, in11, in13, in15, in17, in19, in21, in23, in25, in27, in29, in31] = odds(c);
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
    let t17a = ((  t30 *   799         - t17 * (4017 - 4096)  + 2048) >> 12) - t17;
    let t30a = ((  t30 * (4017 - 4096) + t17 *   799          + 2048) >> 12) + t30;
    let t18a = ((-(t29 * (4017 - 4096) + t18 *   799)         + 2048) >> 12) - t29;
    let t29a = ((  t29 *   799         - t18 * (4017 - 4096)  + 2048) >> 12) - t18;
    let t21a =  (  t26 *  1703         - t21 *  1138          + 1024) >> 11;
    let t26a =  (  t26 *  1138         + t21 *  1703          + 1024) >> 11;
    let t22a =  (-(t25 *  1138         + t22 *  1703) + 1024) >> 11;
    let t25a =  (  t25 *  1703         - t22 *  1138          + 1024) >> 11;
    let t16a = clip(t16  + t19);
    let t17  = clip(t17a + t18a);
    let t18  = clip(t17a - t18a);
    let t19a = clip(t16  - t19);
    let t20a = clip(t23  - t20);
    let t21  = clip(t22a - t21a);
    let t22  = clip(t22a + t21a);
    let t23a = clip(t23  + t20);
    let t24a = clip(t24  + t27);
    let t25  = clip(t25a + t26a);
    let t26  = clip(t25a - t26a);
    let t27a = clip(t24  - t27);
    let t28a = clip(t31  - t28);
    let t29  = clip(t30a - t29a);
    let t30  = clip(t30a + t29a);
    let t31a = clip(t31  + t28);
    let t18a = ((  t29  *  1567         - t18  * (3784 - 4096)  + 2048) >> 12) - t18;
    let t29a = ((  t29  * (3784 - 4096) + t18  *  1567          + 2048) >> 12) + t29;
    let t19  = ((  t28a *  1567         - t19a * (3784 - 4096)  + 2048) >> 12) - t19a;
    let t28  = ((  t28a * (3784 - 4096) + t19a *  1567          + 2048) >> 12) + t28a;
    let t20  = ((-(t27a * (3784 - 4096) + t20a *  1567)         + 2048) >> 12) - t27a;
    let t27  = ((  t27a *  1567         - t20a * (3784 - 4096)  + 2048) >> 12) - t20a;
    let t21a = ((-(t26  * (3784 - 4096) + t21  *  1567)         + 2048) >> 12) - t26;
    let t26a = ((  t26  *  1567         - t21  * (3784 - 4096)  + 2048) >> 12) - t21;
    let t16  = clip(t16a + t23a);
    let t17a = clip(t17  + t22);
    let t18  = clip(t18a + t21a);
    let t19a = clip(t19  + t20);
    let t20a = clip(t19  - t20);
    let t21  = clip(t18a - t21a);
    let t22a = clip(t17  - t22);
    let t23  = clip(t16a - t23a);
    let t24  = clip(t31a - t24a);
    let t25a = clip(t30  - t25);
    let t26  = clip(t29a - t26a);
    let t27a = clip(t28  - t27);
    let t28a = clip(t28  + t27);
    let t29  = clip(t29a + t26a);
    let t30a = clip(t30  + t25);
    let t31  = clip(t31a + t24a);
    let t20  = ((t27a - t20a) * 181 + 128) >> 8;
    let t27  = ((t27a + t20a) * 181 + 128) >> 8;
    let t21a = ((t26  - t21) * 181 + 128) >> 8;
    let t26a = ((t26  + t21) * 181 + 128) >> 8;
    let t22  = ((t25a - t22a) * 181 + 128) >> 8;
    let t25  = ((t25a + t22a) * 181 + 128) >> 8;
    let t23a = ((t24  - t23) * 181 + 128) >> 8;
    let t24a = ((t24  + t23) * 181 + 128) >> 8;
    [
        clip(t0  + t31), clip(t1  + t30a), clip(t2  + t29), clip(t3  + t28a),
        clip(t4  + t27), clip(t5  + t26a), clip(t6  + t25), clip(t7  + t24a),
        clip(t8  + t23a), clip(t9  + t22), clip(t10 + t21a), clip(t11 + t20),
        clip(t12 + t19a), clip(t13 + t18), clip(t14 + t17a), clip(t15 + t16),
        clip(t15 - t16), clip(t14 - t17a), clip(t13 - t18), clip(t12 - t19a),
        clip(t11 - t20), clip(t10 - t21a), clip(t9  - t22), clip(t8  - t23a),
        clip(t7  - t24a), clip(t6  - t25), clip(t5  - t26a), clip(t4  - t27),
        clip(t3  - t28a), clip(t2  - t29), clip(t1  - t30a), clip(t0  - t31),
    ]
}

/// `inv_dct64_1d_c`: of the 64 inputs only the first 32 can be nonzero
/// (AV1 codes at most 32x32 coefficients), so it takes those; the 32-point
/// DCT of the even inputs gets zeros for the upper half (module documentation).
#[inline(always)]
pub fn dct64<S: Simd, V: Lanes<S>>(c: [V; 32], range: Clip<V>) -> [V; 64] {
    let clip = |v: V| v.clip(range);
    let zero = V::splat(range.min.token(), 0);
    let mut even = [zero; 32];
    even[..16].copy_from_slice(&evens::<_, 32, 16>(c));
    let [t0, t1, t2, t3, t4, t5, t6, t7, t8, t9, t10, t11, t12, t13, t14, t15, t16, t17, t18, t19, t20, t21, t22, t23, t24, t25, t26, t27, t28, t29, t30, t31] = dct32(even, range);
    let [in1, in3, in5, in7, in9, in11, in13, in15, in17, in19, in21, in23, in25, in27, in29, in31] = odds(c);
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
    let t33a = ((t33 * (4096 - 4076) + t62 *   401         + 2048) >> 12) - t33;
    let t34a = ((t34 *  -401         + t61 * (4096 - 4076) + 2048) >> 12) - t61;
    let t37a =  (t37 * -1299         + t58 *  1583         + 1024) >> 11;
    let t38a =  (t38 * -1583         + t57 * -1299         + 1024) >> 11;
    let t41a = ((t41 * (4096 - 3612) + t54 *  1931         + 2048) >> 12) - t41;
    let t42a = ((t42 * -1931         + t53 * (4096 - 3612) + 2048) >> 12) - t53;
    let t45a = ((t45 * -1189         + t50 * (3920 - 4096) + 2048) >> 12) + t50;
    let t46a = ((t46 * (4096 - 3920) + t49 * -1189         + 2048) >> 12) - t46;
    let t49a = ((t46 * -1189         + t49 * (3920 - 4096) + 2048) >> 12) + t49;
    let t50a = ((t45 * (3920 - 4096) + t50 *  1189         + 2048) >> 12) + t45;
    let t53a = ((t42 * (4096 - 3612) + t53 *  1931         + 2048) >> 12) - t42;
    let t54a = ((t41 *  1931         + t54 * (3612 - 4096) + 2048) >> 12) + t54;
    let t57a =  (t38 * -1299         + t57 *  1583         + 1024) >> 11;
    let t58a =  (t37 *  1583         + t58 *  1299         + 1024) >> 11;
    let t61a = ((t34 * (4096 - 4076) + t61 *   401         + 2048) >> 12) - t34;
    let t62a = ((t33 *   401         + t62 * (4076 - 4096) + 2048) >> 12) + t62;
    let t32a = clip(t32  + t35);
    let t33  = clip(t33a + t34a);
    let t34  = clip(t33a - t34a);
    let t35a = clip(t32  - t35);
    let t36a = clip(t39  - t36);
    let t37  = clip(t38a - t37a);
    let t38  = clip(t38a + t37a);
    let t39a = clip(t39  + t36);
    let t40a = clip(t40  + t43);
    let t41  = clip(t41a + t42a);
    let t42  = clip(t41a - t42a);
    let t43a = clip(t40  - t43);
    let t44a = clip(t47  - t44);
    let t45  = clip(t46a - t45a);
    let t46  = clip(t46a + t45a);
    let t47a = clip(t47  + t44);
    let t48a = clip(t48  + t51);
    let t49  = clip(t49a + t50a);
    let t50  = clip(t49a - t50a);
    let t51a = clip(t48  - t51);
    let t52a = clip(t55  - t52);
    let t53  = clip(t54a - t53a);
    let t54  = clip(t54a + t53a);
    let t55a = clip(t55  + t52);
    let t56a = clip(t56  + t59);
    let t57  = clip(t57a + t58a);
    let t58  = clip(t57a - t58a);
    let t59a = clip(t56  - t59);
    let t60a = clip(t63  - t60);
    let t61  = clip(t62a - t61a);
    let t62  = clip(t62a + t61a);
    let t63a = clip(t63  + t60);
    let t34a = ((t34  * (4096 - 4017) + t61  *   799         + 2048) >> 12) - t34;
    let t35  = ((t35a * (4096 - 4017) + t60a *   799         + 2048) >> 12) - t35a;
    let t36  = ((t36a *  -799         + t59a * (4096 - 4017) + 2048) >> 12) - t59a;
    let t37a = ((t37  *  -799         + t58  * (4096 - 4017) + 2048) >> 12) - t58;
    let t42a =  (t42  * -1138         + t53  *  1703         + 1024) >> 11;
    let t43  =  (t43a * -1138         + t52a *  1703         + 1024) >> 11;
    let t44  =  (t44a * -1703         + t51a * -1138         + 1024) >> 11;
    let t45a =  (t45  * -1703         + t50  * -1138         + 1024) >> 11;
    let t50a =  (t45  * -1138         + t50  *  1703         + 1024) >> 11;
    let t51  =  (t44a * -1138         + t51a *  1703         + 1024) >> 11;
    let t52  =  (t43a *  1703         + t52a *  1138         + 1024) >> 11;
    let t53a =  (t42  *  1703         + t53  *  1138         + 1024) >> 11;
    let t58a = ((t37  * (4096 - 4017) + t58  *   799         + 2048) >> 12) - t37;
    let t59  = ((t36a * (4096 - 4017) + t59a *   799         + 2048) >> 12) - t36a;
    let t60  = ((t35a *   799         + t60a * (4017 - 4096) + 2048) >> 12) + t60a;
    let t61a = ((t34  *   799         + t61  * (4017 - 4096) + 2048) >> 12) + t61;
    let t32  = clip(t32a + t39a);
    let t33a = clip(t33  + t38);
    let t34  = clip(t34a + t37a);
    let t35a = clip(t35  + t36);
    let t36a = clip(t35  - t36);
    let t37  = clip(t34a - t37a);
    let t38a = clip(t33  - t38);
    let t39  = clip(t32a - t39a);
    let t40  = clip(t47a - t40a);
    let t41a = clip(t46  - t41);
    let t42  = clip(t45a - t42a);
    let t43a = clip(t44  - t43);
    let t44a = clip(t44  + t43);
    let t45  = clip(t45a + t42a);
    let t46a = clip(t46  + t41);
    let t47  = clip(t47a + t40a);
    let t48  = clip(t48a + t55a);
    let t49a = clip(t49  + t54);
    let t50  = clip(t50a + t53a);
    let t51a = clip(t51  + t52);
    let t52a = clip(t51  - t52);
    let t53  = clip(t50a - t53a);
    let t54a = clip(t49  - t54);
    let t55  = clip(t48a - t55a);
    let t56  = clip(t63a - t56a);
    let t57a = clip(t62  - t57);
    let t58  = clip(t61a - t58a);
    let t59a = clip(t60  - t59);
    let t60a = clip(t60  + t59);
    let t61  = clip(t61a + t58a);
    let t62a = clip(t62  + t57);
    let t63  = clip(t63a + t56a);
    let t36  = ((t36a * (4096 - 3784) + t59a *  1567         + 2048) >> 12) - t36a;
    let t37a = ((t37  * (4096 - 3784) + t58  *  1567         + 2048) >> 12) - t37;
    let t38  = ((t38a * (4096 - 3784) + t57a *  1567         + 2048) >> 12) - t38a;
    let t39a = ((t39  * (4096 - 3784) + t56  *  1567         + 2048) >> 12) - t39;
    let t40a = ((t40  * -1567         + t55  * (4096 - 3784) + 2048) >> 12) - t55;
    let t41  = ((t41a * -1567         + t54a * (4096 - 3784) + 2048) >> 12) - t54a;
    let t42a = ((t42  * -1567         + t53  * (4096 - 3784) + 2048) >> 12) - t53;
    let t43  = ((t43a * -1567         + t52a * (4096 - 3784) + 2048) >> 12) - t52a;
    let t52  = ((t43a * (4096 - 3784) + t52a *  1567         + 2048) >> 12) - t43a;
    let t53a = ((t42  * (4096 - 3784) + t53  *  1567         + 2048) >> 12) - t42;
    let t54  = ((t41a * (4096 - 3784) + t54a *  1567         + 2048) >> 12) - t41a;
    let t55a = ((t40  * (4096 - 3784) + t55  *  1567         + 2048) >> 12) - t40;
    let t56a = ((t39  *  1567         + t56  * (3784 - 4096) + 2048) >> 12) + t56;
    let t57  = ((t38a *  1567         + t57a * (3784 - 4096) + 2048) >> 12) + t57a;
    let t58a = ((t37  *  1567         + t58  * (3784 - 4096) + 2048) >> 12) + t58;
    let t59  = ((t36a *  1567         + t59a * (3784 - 4096) + 2048) >> 12) + t59a;
    let t32a = clip(t32  + t47);
    let t33  = clip(t33a + t46a);
    let t34a = clip(t34  + t45);
    let t35  = clip(t35a + t44a);
    let t36a = clip(t36  + t43);
    let t37  = clip(t37a + t42a);
    let t38a = clip(t38  + t41);
    let t39  = clip(t39a + t40a);
    let t40  = clip(t39a - t40a);
    let t41a = clip(t38  - t41);
    let t42  = clip(t37a - t42a);
    let t43a = clip(t36  - t43);
    let t44  = clip(t35a - t44a);
    let t45a = clip(t34  - t45);
    let t46  = clip(t33a - t46a);
    let t47a = clip(t32  - t47);
    let t48a = clip(t63  - t48);
    let t49  = clip(t62a - t49a);
    let t50a = clip(t61  - t50);
    let t51  = clip(t60a - t51a);
    let t52a = clip(t59  - t52);
    let t53  = clip(t58a - t53a);
    let t54a = clip(t57  - t54);
    let t55  = clip(t56a - t55a);
    let t56  = clip(t56a + t55a);
    let t57a = clip(t57  + t54);
    let t58  = clip(t58a + t53a);
    let t59a = clip(t59  + t52);
    let t60  = clip(t60a + t51a);
    let t61a = clip(t61  + t50);
    let t62  = clip(t62a + t49a);
    let t63a = clip(t63  + t48);
    let t40a = ((t55  - t40) * 181 + 128) >> 8;
    let t41  = ((t54a - t41a) * 181 + 128) >> 8;
    let t42a = ((t53  - t42) * 181 + 128) >> 8;
    let t43  = ((t52a - t43a) * 181 + 128) >> 8;
    let t44a = ((t51  - t44) * 181 + 128) >> 8;
    let t45  = ((t50a - t45a) * 181 + 128) >> 8;
    let t46a = ((t49  - t46) * 181 + 128) >> 8;
    let t47  = ((t48a - t47a) * 181 + 128) >> 8;
    let t48  = ((t47a + t48a) * 181 + 128) >> 8;
    let t49a = ((t46  + t49) * 181 + 128) >> 8;
    let t50  = ((t45a + t50a) * 181 + 128) >> 8;
    let t51a = ((t44  + t51) * 181 + 128) >> 8;
    let t52  = ((t43a + t52a) * 181 + 128) >> 8;
    let t53a = ((t42  + t53) * 181 + 128) >> 8;
    let t54  = ((t41a + t54a) * 181 + 128) >> 8;
    let t55a = ((t40  + t55) * 181 + 128) >> 8;
    [
        clip(t0  + t63a), clip(t1  + t62), clip(t2  + t61a), clip(t3  + t60),
        clip(t4  + t59a), clip(t5  + t58), clip(t6  + t57a), clip(t7  + t56),
        clip(t8  + t55a), clip(t9  + t54), clip(t10 + t53a), clip(t11 + t52),
        clip(t12 + t51a), clip(t13 + t50), clip(t14 + t49a), clip(t15 + t48),
        clip(t16 + t47), clip(t17 + t46a), clip(t18 + t45), clip(t19 + t44a),
        clip(t20 + t43), clip(t21 + t42a), clip(t22 + t41), clip(t23 + t40a),
        clip(t24 + t39), clip(t25 + t38a), clip(t26 + t37), clip(t27 + t36a),
        clip(t28 + t35), clip(t29 + t34a), clip(t30 + t33), clip(t31 + t32a),
        clip(t31 - t32a), clip(t30 - t33), clip(t29 - t34a), clip(t28 - t35),
        clip(t27 - t36a), clip(t26 - t37), clip(t25 - t38a), clip(t24 - t39),
        clip(t23 - t40a), clip(t22 - t41), clip(t21 - t42a), clip(t20 - t43),
        clip(t19 - t44a), clip(t18 - t45), clip(t17 - t46a), clip(t16 - t47),
        clip(t15 - t48), clip(t14 - t49a), clip(t13 - t50), clip(t12 - t51a),
        clip(t11 - t52), clip(t10 - t53a), clip(t9  - t54), clip(t8  - t55a),
        clip(t7  - t56), clip(t6  - t57a), clip(t5  - t58), clip(t4  - t59a),
        clip(t3  - t60), clip(t2  - t61a), clip(t1  - t62), clip(t0  - t63a),
    ]
}

/// `inv_adst4_1d_internal_c` (no clipping, as in the C).
#[inline(always)]
pub fn adst4<S: Simd, V: Lanes<S>>(c: [V; 4]) -> [V; 4] {
    let [in0, in1, in2, in3] = c;
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
pub fn adst8<S: Simd, V: Lanes<S>>(c: [V; 8], range: Clip<V>) -> [V; 8] {
    let clip = |v: V| v.clip(range);
    let [in0, in1, in2, in3, in4, in5, in6, in7] = c;
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
    let t4a = ((t4 * (3784 - 4096) + t5 * 1567 + 2048) >> 12) + t4;
    let t5a = ((t4 * 1567 - t5 * (3784 - 4096) + 2048) >> 12) - t5;
    let t6a = ((t7 * (3784 - 4096) - t6 * 1567 + 2048) >> 12) + t7;
    let t7a = ((t7 * 1567 + t6 * (3784 - 4096) + 2048) >> 12) + t6;
    let out0 = clip(t0 + t2);
    let out7 = -clip(t1 + t3);
    let t2 = clip(t0 - t2);
    let t3 = clip(t1 - t3);
    let out1 = -clip(t4a + t6a);
    let out6 = clip(t5a + t7a);
    let t6 = clip(t4a - t6a);
    let t7 = clip(t5a - t7a);
    let out3 = -(((t2 + t3) * 181 + 128) >> 8);
    let out4 = ((t2 - t3) * 181 + 128) >> 8;
    let out2 = ((t6 + t7) * 181 + 128) >> 8;
    let out5 = -(((t6 - t7) * 181 + 128) >> 8);
    [
        out0, out1, out2, out3,
        out4, out5, out6, out7,
    ]
}

/// `inv_adst16_1d_internal_c`.
#[inline(always)]
pub fn adst16<S: Simd, V: Lanes<S>>(c: [V; 16], range: Clip<V>) -> [V; 16] {
    let clip = |v: V| v.clip(range);
    let [in0, in1, in2, in3, in4, in5, in6, in7, in8, in9, in10, in11, in12, in13, in14, in15] = c;
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
    let t8   = ((t8a  * (4017 - 4096) + t9a  *   799         + 2048) >> 12) + t8a;
    let t9   = ((t8a  *   799         - t9a  * (4017 - 4096) + 2048) >> 12) - t9a;
    let t10  = ((t10a *  2276         + t11a * (3406 - 4096) + 2048) >> 12) + t11a;
    let t11  = ((t10a * (3406 - 4096) - t11a *  2276         + 2048) >> 12) + t10a;
    let t12  = ((t13a * (4017 - 4096) - t12a *   799         + 2048) >> 12) + t13a;
    let t13  = ((t13a *   799         + t12a * (4017 - 4096) + 2048) >> 12) + t12a;
    let t14  = ((t15a *  2276         - t14a * (3406 - 4096) + 2048) >> 12) - t14a;
    let t15  = ((t15a * (3406 - 4096) + t14a *  2276         + 2048) >> 12) + t15a;
    let t0   = clip(t0a + t4a);
    let t1   = clip(t1a + t5a);
    let t2   = clip(t2a + t6a);
    let t3   = clip(t3a + t7a);
    let t4   = clip(t0a - t4a);
    let t5   = clip(t1a - t5a);
    let t6   = clip(t2a - t6a);
    let t7   = clip(t3a - t7a);
    let t8a  = clip(t8  + t12);
    let t9a  = clip(t9  + t13);
    let t10a = clip(t10 + t14);
    let t11a = clip(t11 + t15);
    let t12a = clip(t8  - t12);
    let t13a = clip(t9  - t13);
    let t14a = clip(t10 - t14);
    let t15a = clip(t11 - t15);
    let t4a  = ((t4   * (3784 - 4096) + t5   *  1567         + 2048) >> 12) + t4;
    let t5a  = ((t4   *  1567         - t5   * (3784 - 4096) + 2048) >> 12) - t5;
    let t6a  = ((t7   * (3784 - 4096) - t6   *  1567         + 2048) >> 12) + t7;
    let t7a  = ((t7   *  1567         + t6   * (3784 - 4096) + 2048) >> 12) + t6;
    let t12  = ((t12a * (3784 - 4096) + t13a *  1567         + 2048) >> 12) + t12a;
    let t13  = ((t12a *  1567         - t13a * (3784 - 4096) + 2048) >> 12) - t13a;
    let t14  = ((t15a * (3784 - 4096) - t14a *  1567         + 2048) >> 12) + t15a;
    let t15  = ((t15a *  1567         + t14a * (3784 - 4096) + 2048) >> 12) + t14a;
    let out0 = clip(t0  + t2);
    let out15 = -clip(t1  + t3);
    let t2a             =  clip(t0  - t2);
    let t3a             =  clip(t1  - t3);
    let out3 = -clip(t4a + t6a);
    let out12 = clip(t5a + t7a);
    let t6              =  clip(t4a - t6a);
    let t7              =  clip(t5a - t7a);
    let out1 = -clip(t8a + t10a);
    let out14 = clip(t9a + t11a);
    let t10             =  clip(t8a - t10a);
    let t11             =  clip(t9a - t11a);
    let out2 = clip(t12 + t14);
    let out13 = -clip(t13 + t15);
    let t14a            =  clip(t12 - t14);
    let t15a            =  clip(t13 - t15);
    let out7 = -(((t2a  + t3a)  * 181 + 128) >> 8);
    let out8 = ((t2a  - t3a)  * 181 + 128) >> 8;
    let out4 = ((t6   + t7)   * 181 + 128) >> 8;
    let out11 = -(((t6   - t7)   * 181 + 128) >> 8);
    let out6 = ((t10  + t11)  * 181 + 128) >> 8;
    let out9 = -(((t10  - t11)  * 181 + 128) >> 8);
    let out5 = -(((t14a + t15a) * 181 + 128) >> 8);
    let out10 = ((t14a - t15a) * 181 + 128) >> 8;
    [
        out0, out1, out2, out3,
        out4, out5, out6, out7,
        out8, out9, out10, out11,
        out12, out13, out14, out15,
    ]
}
