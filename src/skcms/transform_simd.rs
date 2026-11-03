// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Four-lane baseline stages from pinned src/Transform_inl.h. The ARM NEON and
//! x86 SSE arithmetic stays vector arithmetic, including eager branch operands.

use super::curve::Curve;
use super::profile::{A2B, B2A};
use super::transfer_function::TransferFunction as Tf;
use super::transform_types::Op;
use super::types::{Matrix3x3, Matrix3x4};
use std::ops::{Add, Div, Mul, Sub};
use std::ptr;
#[cfg(target_arch = "aarch64")]
#[path = "transform_neon.rs"]
mod neon;

#[derive(Clone, Copy)]
struct F([f32; 4]);
impl F {
    const ZERO: Self = Self([0.0; 4]);
    const ONE: Self = Self([1.0; 4]);
    fn splat(v: f32) -> Self {
        Self([v; 4])
    }
    fn bits(self) -> [u32; 4] {
        self.0.map(f32::to_bits)
    }
    fn from_bits(v: [u32; 4]) -> Self {
        Self(v.map(f32::from_bits))
    }
    fn less(self, rhs: Self) -> [bool; 4] {
        std::array::from_fn(|i| self.0[i] < rhs.0[i])
    }
    fn greater(self, rhs: Self) -> [bool; 4] {
        rhs.less(self)
    }
    fn equal(self, rhs: Self) -> [bool; 4] {
        std::array::from_fn(|i| self.0[i] == rhs.0[i])
    }
    fn select(c: [bool; 4], t: Self, e: Self) -> Self {
        Self(std::array::from_fn(|i| if c[i] { t.0[i] } else { e.0[i] }))
    }

    #[expect(
        clippy::needless_return,
        reason = "Keep target-selected intrinsic branches explicit."
    )]
    fn i32(self) -> [i32; 4] {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: all bit patterns of four binary32 lanes are valid intrinsic inputs.
        unsafe {
            return std::mem::transmute::<std::arch::aarch64::int32x4_t, [i32; 4]>(
                std::arch::aarch64::vcvtq_s32_f32(std::mem::transmute::<
                    [f32; 4],
                    std::arch::aarch64::float32x4_t,
                >(self.0)),
            );
        }
        #[cfg(target_arch = "x86_64")]
        // SAFETY: SSE2 is a required x86_64 baseline feature; conversion retains its native result.
        unsafe {
            return std::mem::transmute::<std::arch::x86_64::__m128i, [i32; 4]>(
                std::arch::x86_64::_mm_cvttps_epi32(std::mem::transmute::<
                    [f32; 4],
                    std::arch::x86_64::__m128,
                >(self.0)),
            );
        }
        #[cfg(not(any(target_arch = "aarch64", target_arch = "x86_64")))]
        self.0.map(|v| v as i32)
    }
    #[cfg_attr(
        target_arch = "aarch64",
        expect(
            clippy::needless_return,
            reason = "Keep target-selected intrinsic branches explicit."
        )
    )]
    fn u32(self) -> [u32; 4] {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: NEON conversion has no memory preconditions.
        unsafe {
            return std::mem::transmute::<std::arch::aarch64::uint32x4_t, [u32; 4]>(
                std::arch::aarch64::vcvtq_u32_f32(std::mem::transmute::<
                    [f32; 4],
                    std::arch::aarch64::float32x4_t,
                >(self.0)),
            );
        }
        #[cfg(not(target_arch = "aarch64"))]
        self.0.map(|v| v as u32)
    }
    fn fixed(self) -> [u32; 4] {
        (self + 0.5).i32().map(|v| v as u32)
    }
    #[cfg_attr(
        target_arch = "aarch64",
        expect(
            clippy::needless_return,
            reason = "Keep target-selected intrinsic branches explicit."
        )
    )]
    fn floor(self) -> Self {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: NEON floor accepts every binary32 lane without memory access.
        unsafe {
            return Self(std::mem::transmute::<
                std::arch::aarch64::float32x4_t,
                [f32; 4],
            >(std::arch::aarch64::vrndmq_f32(
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(self.0),
            )));
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            let roundtrip = Self(self.i32().map(|v| v as f32));
            roundtrip - Self::select(roundtrip.greater(self), Self::ONE, Self::ZERO)
        }
    }
    #[cfg_attr(
        target_arch = "aarch64",
        expect(
            clippy::needless_return,
            reason = "Keep target-selected intrinsic branches explicit."
        )
    )]
    fn min(self, y: Self) -> Self {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: selected baseline source uses vminq_f32 for these four lanes.
        unsafe {
            return Self(std::mem::transmute::<
                std::arch::aarch64::float32x4_t,
                [f32; 4],
            >(std::arch::aarch64::vminq_f32(
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(self.0),
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(y.0),
            )));
        }
        #[cfg(not(target_arch = "aarch64"))]
        Self::select(self.greater(y), y, self)
    }
    #[cfg_attr(
        target_arch = "aarch64",
        expect(
            clippy::needless_return,
            reason = "Keep target-selected intrinsic branches explicit."
        )
    )]
    fn max(self, y: Self) -> Self {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: selected baseline source uses vmaxq_f32 for these four lanes.
        unsafe {
            return Self(std::mem::transmute::<
                std::arch::aarch64::float32x4_t,
                [f32; 4],
            >(std::arch::aarch64::vmaxq_f32(
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(self.0),
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(y.0),
            )));
        }
        #[cfg(not(target_arch = "aarch64"))]
        Self::select(self.less(y), y, self)
    }
    #[cfg_attr(
        target_arch = "aarch64",
        expect(
            clippy::needless_return,
            reason = "Keep target-selected intrinsic branches explicit."
        )
    )]
    fn madd(self, b: Self, c: Self) -> Self {
        #[cfg(target_arch = "aarch64")]
        // SAFETY: NEON fused arithmetic implements the original contracted source expression.
        unsafe {
            return Self(std::mem::transmute::<
                std::arch::aarch64::float32x4_t,
                [f32; 4],
            >(std::arch::aarch64::vfmaq_f32(
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(c.0),
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(self.0),
                std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(b.0),
            )));
        }
        #[cfg(not(target_arch = "aarch64"))]
        {
            self * b + c
        }
    }
}
macro_rules! arithmetic {
    ($trait:ident,$method:ident,$arm:ident,$sse:ident,$op:tt) => {
        impl $trait for F {type Output=Self; #[inline] fn $method(self,rhs:Self)->Self {
            #[cfg(target_arch="aarch64")]
            // SAFETY: NEON operates on initialized four-lane values and accesses no memory.
            unsafe {return Self(std::mem::transmute::<std::arch::aarch64::float32x4_t, [f32; 4]>(std::arch::aarch64::$arm(std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(self.0),std::mem::transmute::<[f32; 4], std::arch::aarch64::float32x4_t>(rhs.0))));}
            #[cfg(target_arch="x86_64")]
            // SAFETY: SSE is required on x86_64; both operands have exactly 128 bits.
            unsafe {return Self(std::mem::transmute::<std::arch::x86_64::__m128, [f32; 4]>(std::arch::x86_64::$sse(std::mem::transmute::<[f32; 4], std::arch::x86_64::__m128>(self.0),std::mem::transmute::<[f32; 4], std::arch::x86_64::__m128>(rhs.0))));}
            #[cfg(not(any(target_arch="aarch64",target_arch="x86_64")))]
            Self(std::array::from_fn(|i|self.0[i] $op rhs.0[i]))
        }}
        impl $trait<f32> for F {type Output=Self; fn $method(self,rhs:f32)->Self {self $op Self::splat(rhs)}}
        impl $trait<F> for f32 {type Output=F; fn $method(self,rhs:F)->F {F::splat(self) $op rhs}}
    }
}
arithmetic!(Add,add,vaddq_f32,_mm_add_ps,+);
arithmetic!(Sub,sub,vsubq_f32,_mm_sub_ps,-);
arithmetic!(Mul,mul,vmulq_f32,_mm_mul_ps,*);
arithmetic!(Div,div,vdivq_f32,_mm_div_ps,/);

#[expect(
    clippy::excessive_precision,
    reason = "Retain original baseline literals."
)]
fn approx_log2(x: F) -> F {
    let bits = x.bits();
    let e = F(bits.map(|v| (v as i32) as f32)) * (1.0 / (1 << 23) as f32);
    let m = F::from_bits(bits.map(|v| (v & 0x007fffff) | 0x3f000000));
    F::splat(-1.498030302).madd(m, e - 124.225514990) - 1.725879990 / (0.3520887068 + m)
}
#[expect(
    clippy::excessive_precision,
    reason = "Retain original baseline literals."
)]
fn approx_exp2(x: F) -> F {
    let fract = x - x.floor();
    let fbits = (1 << 23) as f32
        * (F::splat(-1.490129070).madd(fract, x + 121.274057500)
            + 27.728023300 / (4.84252568 - fract));
    F::from_bits(
        fbits
            .max(F::ZERO)
            .min(F::splat(0x7f800000u32 as f32))
            .i32()
            .map(|v| v as u32),
    )
}
fn approx_pow(x: F, y: f32) -> F {
    let z = x.equal(F::ZERO);
    let o = x.equal(F::ONE);
    F::select(
        std::array::from_fn(|i| z[i] || o[i]),
        x,
        approx_exp2(approx_log2(x) * y),
    )
}
fn strip_sign(x: F) -> (F, [u32; 4]) {
    let bits = x.bits();
    (
        F::from_bits(bits.map(|v| v & 0x7fffffff)),
        bits.map(|v| v & 0x80000000),
    )
}
fn apply_sign(x: F, s: [u32; 4]) -> F {
    let bits = x.bits();
    F::from_bits(std::array::from_fn(|i| bits[i] | s[i]))
}
fn apply_tf(tf: Tf, x: F) -> F {
    let (x, s) = strip_sign(x);
    apply_sign(
        F::select(
            x.less(F::splat(tf.d)),
            F::splat(tf.c).madd(x, F::splat(tf.f)),
            approx_pow(F::splat(tf.a).madd(x, F::splat(tf.b)), tf.g) + tf.e,
        ),
        s,
    )
}
fn apply_gamma(tf: Tf, x: F) -> F {
    let (x, s) = strip_sign(x);
    apply_sign(approx_pow(x, tf.g), s)
}
fn apply_pq(tf: Tf, x: F) -> F {
    let (x, s) = strip_sign(x);
    let p = approx_pow(x, tf.c);
    apply_sign(
        approx_pow(
            F::splat(tf.b).madd(p, F::splat(tf.a)).max(F::ZERO)
                / F::splat(tf.e).madd(p, F::splat(tf.d)),
            tf.f,
        ),
        s,
    )
}
#[expect(
    clippy::excessive_precision,
    reason = "Retain original baseline literals."
)]
#[expect(
    clippy::approx_constant,
    reason = "Retain original baseline approximation constant."
)]
fn apply_hlg(tf: Tf, x: F) -> F {
    let (x, s) = strip_sign(x);
    let xr = x * tf.a;
    let v = F::select(
        std::array::from_fn(|i| xr.0[i] <= 1.0),
        approx_pow(xr, tf.b),
        approx_exp2(1.4426950408889634074 * ((x - tf.e) * tf.c)) + tf.d,
    );
    (tf.f + 1.0) * apply_sign(v, s)
}
#[expect(
    clippy::excessive_precision,
    reason = "Retain original baseline literals."
)]
#[expect(
    clippy::approx_constant,
    reason = "Retain original baseline approximation constant."
)]
fn apply_hlginv(tf: Tf, x: F) -> F {
    let (x, s) = strip_sign(x);
    let x = x / (tf.f + 1.0);
    apply_sign(
        F::select(
            std::array::from_fn(|i| x.0[i] <= 1.0),
            tf.a * approx_pow(x, tf.b),
            F::splat(tf.c).madd(0.69314718 * approx_log2(x - tf.d), F::splat(tf.e)),
        ),
        s,
    )
}
#[expect(
    clippy::excessive_precision,
    reason = "Retain original baseline literals."
)]
fn compute_y(x: F, y: F, z: F) -> F {
    #[cfg(target_arch = "aarch64")]
    {
        // Actual baseline NEON rounds G*Yg, then contracts R*Yr and B*Yb
        // in source order (both original forward and inverse OOTF stages).
        F::splat(0.02102382).madd(z, F::splat(-0.02831655).madd(x, 1.00995452 * y))
    }
    #[cfg(not(target_arch = "aarch64"))]
    F::splat(0.02102382).madd(z, F::splat(1.00995452).madd(y, -0.02831655 * x))
}
fn minus_ulp(v: F) -> F {
    F::from_bits(v.bits().map(|v| v.wrapping_sub(1)))
}

fn half_to_f(half: [u16; 4]) -> F {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: FCVTL implements the selected original NEON half conversion. The
    // register is an initialized 64-bit vector, with no memory operands.
    unsafe {
        let input: std::arch::aarch64::float32x2_t = std::mem::transmute(half);
        let output: std::arch::aarch64::float32x4_t;
        std::arch::asm!("fcvtl {output:v}.4s, {input:v}.4h",input=in(vreg) input,output=out(vreg) output,options(pure,nomem,nostack));
        F(std::mem::transmute::<
            std::arch::aarch64::float32x4_t,
            [f32; 4],
        >(output))
    }
    #[cfg(not(target_arch = "aarch64"))]
    F::from_bits(half.map(|v| {
        let s = (v as u32) & 0x8000;
        let em = v as u32 ^ s;
        if em < 0x400 {
            0
        } else {
            (s << 16) + (em << 13) + ((127 - 15) << 23)
        }
    }))
}
fn f_to_half(f: F) -> [u16; 4] {
    #[cfg(target_arch = "aarch64")]
    // SAFETY: FCVTN is the original NEON narrowing conversion; every binary32
    // input is valid, and no memory is accessed by the instruction.
    unsafe {
        let input: std::arch::aarch64::float32x4_t = std::mem::transmute(f.0);
        let output: std::arch::aarch64::float32x2_t;
        std::arch::asm!("fcvtn {output:v}.4h, {input:v}.4s",input=in(vreg) input,output=out(vreg) output,options(pure,nomem,nostack));
        std::mem::transmute(output)
    }
    #[cfg(not(target_arch = "aarch64"))]
    f.bits().map(|v| {
        let s = v & 0x80000000;
        let em = v ^ s;
        if em < 0x38800000 {
            0
        } else {
            ((s >> 16)
                .wrapping_add(em >> 13)
                .wrapping_sub((127 - 15) << 10)) as u16
        }
    })
}

/// # Safety
/// The active table count/pointers and every reached source index must be
/// initialized and readable. All inactive union bytes remain unobserved.
unsafe fn table(curve: *const Curve, v: F) -> F {
    // SAFETY: caller establishes the active table branch and its live backing.
    unsafe {
        let entries = curve.cast::<u32>().read();
        let ix = F::ZERO.max(v.min(F::ONE)) * (entries - 1) as f32;
        let lo = ix.i32();
        let hi = minus_ulp(ix + 1.0).i32();
        let t = ix - F(lo.map(|v| v as f32));
        let p8 = ptr::addr_of!((*curve).table.table_8).read();
        let (l, h) = if !p8.is_null() {
            (
                F(std::array::from_fn(|i| *p8.offset(lo[i] as isize) as f32)) * (1.0 / 255.0),
                F(std::array::from_fn(|i| *p8.offset(hi[i] as isize) as f32)) * (1.0 / 255.0),
            )
        } else {
            let p = ptr::addr_of!((*curve).table.table_16).read();
            (
                F(std::array::from_fn(|i| {
                    p.offset(2 * lo[i] as isize)
                        .cast::<u16>()
                        .read_unaligned()
                        .swap_bytes() as f32
                })) * (1.0 / 65535.0),
                F(std::array::from_fn(|i| {
                    p.offset(2 * hi[i] as isize)
                        .cast::<u16>()
                        .read_unaligned()
                        .swap_bytes() as f32
                })) * (1.0 / 65535.0),
            )
        };
        (h - l).madd(t, l)
    }
}

/// # Safety
/// Grid dimensions and pointers have the source's initialized active shape and
/// readable backing, including the original one/two-byte gather look-behind.
unsafe fn clut(
    dim: usize,
    outputs: usize,
    points: *const u8,
    grid8: *const u8,
    grid16: *const u8,
    rgba: &mut [F; 4],
) {
    let mut index = [[0i32; 4]; 8];
    let mut weight = [F::ZERO; 8];
    let inputs = *rgba;
    let mut stride = 1;
    for i in (0..dim).rev() {
        // SAFETY: original caller guarantees exactly the reached dimension prefix.
        let point = unsafe { points.add(i).read() };
        let x = inputs[i] * (point as i32 - 1) as f32;
        let lo = x.i32();
        let hi = minus_ulp(x + 1.0).i32();
        index[i] = lo.map(|v| v * stride);
        index[i + 4] = hi.map(|v| v * stride);
        stride *= point as i32;
        let t = x - F(lo.map(|v| v as f32));
        weight[i] = 1.0 - t;
        weight[i + 4] = t;
    }
    rgba[0] = F::ZERO;
    rgba[1] = F::ZERO;
    rgba[2] = F::ZERO;
    if outputs == 4 {
        rgba[3] = F::ZERO;
    }
    for combo in 0..1usize << dim {
        let mut ix = index[(combo & 1) * 4];
        let mut w = weight[(combo & 1) * 4];
        for d in (1..dim).rev() {
            let at = d + ((combo >> d) & 1) * 4;
            ix = std::array::from_fn(|i| ix[i] + index[at][i]);
            w = w * weight[at];
        }
        let mut sample = [F::ZERO; 4];
        // SAFETY: caller guarantees source gather look-behind and complete
        // records at all reached indices, with unaligned memcpy-style loads.
        unsafe {
            if outputs == 3 && !grid8.is_null() {
                let words: [u32; 4] = std::array::from_fn(|i| {
                    grid8
                        .offset(3 * ix[i] as isize - 1)
                        .cast::<u32>()
                        .read_unaligned()
                        >> 8
                });
                for (c, out) in sample.iter_mut().enumerate().take(3) {
                    *out = F(words.map(|v| ((v >> (8 * c)) & 255) as f32)) * (1.0 / 255.0);
                }
            } else if outputs == 3 {
                let words: [u64; 4] = std::array::from_fn(|i| {
                    grid16
                        .offset(6 * ix[i] as isize - 2)
                        .cast::<u64>()
                        .read_unaligned()
                        >> 16
                });
                for (c, out) in sample.iter_mut().enumerate().take(3) {
                    *out = F(words.map(|v| ((v >> (16 * c)) as u16).swap_bytes() as f32))
                        * (1.0 / 65535.0);
                }
            } else if !grid8.is_null() {
                let words: [u32; 4] = std::array::from_fn(|i| {
                    grid8
                        .offset(4 * ix[i] as isize)
                        .cast::<u32>()
                        .read_unaligned()
                });
                for (c, out) in sample.iter_mut().enumerate() {
                    *out = F(words.map(|v| ((v >> (8 * c)) & 255) as f32)) * (1.0 / 255.0);
                }
            } else {
                for (c, out) in sample.iter_mut().enumerate() {
                    *out = F(std::array::from_fn(|i| {
                        grid16
                            .offset(2 * (4 * ix[i] as isize + c as isize))
                            .cast::<u16>()
                            .read_unaligned()
                            .swap_bytes() as f32
                    })) * (1.0 / 65535.0);
                }
            }
        }
        for c in 0..4 {
            rgba[c] = w.madd(sample[c], rgba[c]);
        }
    }
}

/// # Safety
/// A complete four-pixel span, with original typed alignment, must be readable.
unsafe fn load_pixels(op: Op, src: *const u8, rgba: &mut [F; 4]) {
    use Op::*;
    #[cfg(target_arch = "aarch64")]
    // SAFETY: forwarded complete source span and original typed alignment.
    if unsafe { neon::load(op, src, rgba) } {
        return;
    }
    // SAFETY: caller establishes all reached four-pixel input bytes; the source
    // loads finish before any later stage can write overlapping output.
    unsafe {
        let mut packed = |bytes: usize, components: usize, decode: &dyn Fn(u64, usize) -> f32| {
            let words: [u64; 4] = std::array::from_fn(|i| match bytes {
                1 => *src.add(i) as u64,
                2 => src.add(i * 2).cast::<u16>().read_unaligned() as u64,
                4 => src.add(i * 4).cast::<u32>().read_unaligned() as u64,
                8 => src.add(i * 8).cast::<u64>().read_unaligned(),
                _ => unreachable!(),
            });
            for (c, out) in rgba.iter_mut().enumerate().take(components) {
                *out = F(words.map(|v| decode(v, c)));
            }
        };
        match op {
            load_a8 => rgba[3] = F(std::array::from_fn(|i| *src.add(i) as f32)) * (1.0 / 255.0),
            load_g8 => {
                let v = F(std::array::from_fn(|i| *src.add(i) as f32)) * (1.0 / 255.0);
                rgba[..3].fill(v);
            }
            load_ga88 => {
                let words: [u16; 4] =
                    std::array::from_fn(|i| src.add(i * 2).cast::<u16>().read_unaligned());
                rgba[..3].fill(F(words.map(|v| (v & 255) as f32)) * (1.0 / 255.0));
                rgba[3] = F(words.map(|v| (v >> 8) as f32)) * (1.0 / 255.0);
            }
            load_4444 => packed(2, 4, &|v, c| {
                ((v >> (12 - 4 * c)) & 15) as f32 * (1.0 / 15.0)
            }),
            load_565 => packed(2, 3, &|v, c| {
                let (mask, scale) = [
                    (31u64, 31.0),
                    (63 << 5, (63 << 5) as f32),
                    (31 << 11, (31 << 11) as f32),
                ][c];
                (v & mask) as f32 * (1.0 / scale)
            }),
            load_888 => {
                for (c, out) in rgba.iter_mut().enumerate().take(3) {
                    *out = F(std::array::from_fn(|i| *src.add(3 * i + c) as f32)) * (1.0 / 255.0);
                }
            }
            load_8888 => packed(4, 4, &|v, c| ((v >> (8 * c)) & 255) as f32 * (1.0 / 255.0)),
            load_1010102 => packed(4, 4, &|v, c| {
                if c == 3 {
                    ((v >> 30) & 3) as f32 * (1.0 / 3.0)
                } else {
                    ((v >> (10 * c)) & 1023) as f32 * (1.0 / 1023.0)
                }
            }),
            load_101010x_XR => packed(4, 3, &|v, c| {
                (((v as u32 >> (10 * c)) & 1023).wrapping_sub(384)) as f32 / 510.0
            }),
            load_10101010_XR => packed(8, 4, &|v, c| {
                (((v >> (16 * c + 6)) & 1023).wrapping_sub(384)) as f32 / 510.0
            }),
            load_161616LE | load_16161616LE | load_161616BE | load_16161616BE | load_hhh
            | load_hhhh => {
                let channels = if matches!(op, load_16161616LE | load_16161616BE | load_hhhh) {
                    4
                } else {
                    3
                };
                for (c, out) in rgba.iter_mut().enumerate().take(channels) {
                    let words: [u16; 4] = std::array::from_fn(|i| {
                        src.add(2 * (channels * i + c))
                            .cast::<u16>()
                            .read_unaligned()
                    });
                    *out = if matches!(op, load_hhh | load_hhhh) {
                        half_to_f(words)
                    } else {
                        F(words.map(|v| {
                            if matches!(op, load_161616BE | load_16161616BE) {
                                v.swap_bytes() as f32
                            } else {
                                v as f32
                            }
                        })) * (1.0 / 65535.0)
                    };
                }
            }
            load_fff | load_ffff => {
                let channels = if op == load_fff { 3 } else { 4 };
                for (c, out) in rgba.iter_mut().enumerate().take(channels) {
                    *out = F(std::array::from_fn(|i| {
                        src.add(4 * (channels * i + c))
                            .cast::<f32>()
                            .read_unaligned()
                    }));
                }
            }
            _ => unreachable!(),
        }
    }
}

/// # Safety
/// Destination has the source's writable four-pixel span and typed alignment.
unsafe fn store_pixels(op: Op, dst: *mut u8, rgba: [F; 4]) {
    use Op::*;
    #[cfg(target_arch = "aarch64")]
    // SAFETY: forwarded complete output span and original typed alignment.
    if unsafe { neon::store(op, dst, rgba) } {
        return;
    }
    let [r, g, b, a] = rgba;
    // SAFETY: caller establishes the complete output span. Stores follow the
    // original packed or channel-strided write order without forming references.
    unsafe {
        match op {
            store_a8 | store_g8 => {
                let q = (if op == store_a8 { a } else { g } * 255.0).fixed();
                for (i, v) in q.into_iter().enumerate() {
                    *dst.add(i) = v as u8;
                }
            }
            store_ga88 | store_4444 | store_565 | store_8888 | store_1010102 | store_101010x_XR
            | store_10101010_XR => {
                let mut words = [0u64; 4];
                let mut write = |v: F, scale: f32, shift: usize, mask: u64| {
                    let q = (v * scale).fixed();
                    for i in 0..4 {
                        words[i] |= ((q[i] as u64) & mask) << shift;
                    }
                };
                let bytes = match op {
                    store_ga88 => {
                        write(g, 255.0, 0, 65535);
                        write(a, 255.0, 8, 65535);
                        2
                    }
                    store_4444 => {
                        write(r, 15.0, 12, 65535);
                        write(g, 15.0, 8, 65535);
                        write(b, 15.0, 4, 65535);
                        write(a, 15.0, 0, 65535);
                        2
                    }
                    store_565 => {
                        write(r, 31.0, 0, 65535);
                        write(g, 63.0, 5, 65535);
                        write(b, 31.0, 11, 65535);
                        2
                    }
                    store_8888 => {
                        for (c, v) in rgba.into_iter().enumerate() {
                            write(v, 255.0, c * 8, u32::MAX as u64);
                        }
                        4
                    }
                    store_1010102 => {
                        write(r, 1023.0, 0, u32::MAX as u64);
                        write(g, 1023.0, 10, u32::MAX as u64);
                        write(b, 1023.0, 20, u32::MAX as u64);
                        write(a, 3.0, 30, u32::MAX as u64);
                        4
                    }
                    store_101010x_XR | store_10101010_XR => {
                        let channels = if op == store_101010x_XR { 3 } else { 4 };
                        for (c, v) in rgba.into_iter().enumerate().take(channels) {
                            let q = v.madd(F::splat(510.0), F::splat(384.0)).fixed();
                            for i in 0..4 {
                                words[i] |= (q[i] as u64)
                                    << (if channels == 3 { 10 * c } else { 16 * c + 6 });
                            }
                        }
                        if channels == 3 { 4 } else { 8 }
                    }
                    _ => unreachable!(),
                };
                for (i, v) in words.into_iter().enumerate() {
                    match bytes {
                        2 => dst.add(i * 2).cast::<u16>().write_unaligned(v as u16),
                        4 => dst.add(i * 4).cast::<u32>().write_unaligned(v as u32),
                        8 => dst.add(i * 8).cast::<u64>().write_unaligned(v),
                        _ => unreachable!(),
                    }
                }
            }
            store_888 => {
                for (c, v) in rgba.into_iter().enumerate().take(3) {
                    let q = (v * 255.0).fixed();
                    for (i, w) in q.into_iter().enumerate() {
                        *dst.add(3 * i + c) = w as u8;
                    }
                }
            }
            store_161616LE | store_16161616LE | store_161616BE | store_16161616BE | store_hhh
            | store_hhhh => {
                let channels = if matches!(op, store_16161616LE | store_16161616BE | store_hhhh) {
                    4
                } else {
                    3
                };
                let mut words = [[0u16; 4]; 4];
                for c in 0..channels {
                    words[c] = if matches!(op, store_hhh | store_hhhh) {
                        f_to_half(rgba[c])
                    } else {
                        let q = if cfg!(target_arch = "aarch64") {
                            (rgba[c] * 65535.0 + 0.5).u32()
                        } else {
                            (rgba[c] * 65535.0).fixed()
                        };
                        q.map(|v| {
                            if matches!(op, store_161616BE | store_16161616BE) {
                                (v as u16).swap_bytes()
                            } else {
                                v as u16
                            }
                        })
                    };
                }
                for (c, q) in words.iter().enumerate().take(channels) {
                    for (i, v) in q.iter().enumerate() {
                        dst.add(2 * (channels * i + c))
                            .cast::<u16>()
                            .write_unaligned(*v);
                    }
                }
            }
            store_fff | store_ffff => {
                let channels = if op == store_fff { 3 } else { 4 };
                for (c, q) in rgba.iter().enumerate().take(channels) {
                    for i in 0..4 {
                        dst.add(4 * (channels * i + c))
                            .cast::<f32>()
                            .write_unaligned(q.0[i]);
                    }
                }
            }
            _ => unreachable!(),
        }
    }
}

/// # Safety
/// Program/context pointers, source/output spans and active context objects
/// satisfy all original stage preconditions. Context pointees remain live for
/// execution; source and destination aliases are permitted as in the source.
unsafe fn exec_stages(program: &[Op], contexts: &[*const ()], src: *const u8, dst: *mut u8) {
    use Op::*;
    let mut v = [F::ZERO, F::ZERO, F::ZERO, F::ONE];
    for (&op, &arg) in program.iter().zip(contexts) {
        if (op as i32) < swap_rb as i32 {
            // SAFETY: the caller establishes this complete input group.
            unsafe { load_pixels(op, src, &mut v) };
        } else if (op as i32) >= store_a8 as i32 {
            // SAFETY: caller establishes this output group, and a store terminates execution.
            unsafe { store_pixels(op, dst, v) };
            return;
        } else {
            let [r, g, b, a] = v;
            match op {
                swap_rb => v.swap(0, 2),
                clamp => {
                    for x in &mut v {
                        *x = F::ZERO.max(x.min(F::ONE));
                    }
                }
                invert => {
                    for x in &mut v {
                        *x = 1.0 - *x;
                    }
                }
                force_opaque => v[3] = F::ONE,
                premul => {
                    for x in &mut v[..3] {
                        *x = *x * a;
                    }
                }
                unpremul => {
                    let inv = 1.0 / a;
                    let scale = F::select(inv.less(F::splat(f32::INFINITY)), inv, F::ZERO);
                    for x in &mut v[..3] {
                        *x = *x * scale;
                    }
                }
                matrix_3x3 | matrix_3x4 => {
                    // SAFETY: this opcode selects the live matrix context of its matching size.
                    let rows: [F; 3] = unsafe {
                        if op == matrix_3x3 {
                            let m = arg.cast::<Matrix3x3>();
                            std::array::from_fn(|c| {
                                let q = (*m).vals[c];
                                F::splat(q[2]).madd(b, F::splat(q[0]).madd(r, q[1] * g))
                            })
                        } else {
                            let m = arg.cast::<Matrix3x4>();
                            std::array::from_fn(|c| {
                                let q = (*m).vals[c];
                                F::splat(q[2]).madd(b, F::splat(q[0]).madd(r, q[1] * g)) + q[3]
                            })
                        }
                    };
                    v[..3].copy_from_slice(&rows);
                }
                lab_to_xyz => {
                    let l = r * 100.0;
                    let aa = g.madd(F::splat(255.0), F::splat(-128.0));
                    let bb = b.madd(F::splat(255.0), F::splat(-128.0));
                    let y = (l + 16.0) * (1.0 / 116.0);
                    let x = aa.madd(F::splat(1.0 / 500.0), y);
                    let z = F::splat(-1.0 / 200.0).madd(bb, y);
                    let f = |v: F| {
                        let cube = v * v * v;
                        F::select(
                            cube.greater(F::splat(0.008856)),
                            cube,
                            (v - (16.0 / 116.0)) * (1.0 / 7.787),
                        )
                    };
                    v[0] = f(x) * 0.9642;
                    v[1] = f(y);
                    v[2] = f(z) * 0.8249;
                }
                xyz_to_lab => {
                    let f = |x: F| {
                        F::select(
                            x.greater(F::splat(0.008856)),
                            approx_pow(x, 1.0 / 3.0),
                            x.madd(F::splat(7.787), F::splat(16.0 / 116.0)),
                        )
                    };
                    let x = f(r * (1.0 / 0.9642));
                    let y = f(g);
                    let z = f(b * (1.0 / 0.8249));
                    v[0] = y.madd(F::splat(116.0), F::splat(-16.0)) * (1.0 / 100.0);
                    v[1] = ((x - y) * 500.0 + 128.0) * (1.0 / 255.0);
                    v[2] = ((y - z) * 200.0 + 128.0) * (1.0 / 255.0);
                }
                gamma_r | gamma_g | gamma_b | gamma_a | gamma_rgb | tf_r | tf_g | tf_b | tf_a
                | tf_rgb | pq_r | pq_g | pq_b | pq_a | pq_rgb | hlg_r | hlg_g | hlg_b | hlg_a
                | hlg_rgb | hlginv_r | hlginv_g | hlginv_b | hlginv_a | hlginv_rgb => {
                    // SAFETY: every transfer stage receives its seven-word active function context.
                    let gamma_only = (op as i32) <= gamma_rgb as i32;
                    let generic_tf = (op as i32) >= tf_r as i32 && (op as i32) <= tf_rgb as i32;
                    // SAFETY: the original gamma stage reads only g; generic
                    // TF reads all seven fields, while PQ/HLG never inspect g.
                    // Construct a local carrier from only those reached words.
                    let tf = unsafe {
                        let p = arg.cast::<Tf>();
                        if gamma_only {
                            Tf {
                                g: ptr::addr_of!((*p).g).read(),
                                ..Tf::default()
                            }
                        } else if generic_tf {
                            p.read()
                        } else {
                            Tf {
                                g: 0.0,
                                a: ptr::addr_of!((*p).a).read(),
                                b: ptr::addr_of!((*p).b).read(),
                                c: ptr::addr_of!((*p).c).read(),
                                d: ptr::addr_of!((*p).d).read(),
                                e: ptr::addr_of!((*p).e).read(),
                                f: ptr::addr_of!((*p).f).read(),
                            }
                        }
                    };
                    let (base, apply): (i32, fn(Tf, F) -> F) = if (op as i32) <= gamma_rgb as i32 {
                        (gamma_r as i32, apply_gamma)
                    } else if (op as i32) <= tf_rgb as i32 {
                        (tf_r as i32, apply_tf)
                    } else if (op as i32) <= pq_rgb as i32 {
                        (pq_r as i32, apply_pq)
                    } else if (op as i32) <= hlg_rgb as i32 {
                        (hlg_r as i32, apply_hlg)
                    } else {
                        (hlginv_r as i32, apply_hlginv)
                    };
                    let channel = op as i32 - base;
                    if channel == 4 {
                        for x in &mut v[..3] {
                            *x = apply(tf, *x);
                        }
                    } else {
                        v[channel as usize] = apply(tf, v[channel as usize]);
                    }
                }
                hlg_ootf_scale => {
                    let (y, sign) = strip_sign(compute_y(r, g, b));
                    let scale = apply_sign(approx_pow(y, 0.2), sign);
                    for x in &mut v[..3] {
                        *x = (*x * scale) * (1000.0 / 203.0);
                    }
                }
                hlginv_ootf_scale => {
                    for x in &mut v[..3] {
                        *x = *x * (203.0 / 1000.0);
                    }
                    let (y, sign) = strip_sign(compute_y(v[0], v[1], v[2]));
                    let scale = apply_sign(approx_pow(y, 1.0 / 1.2 - 1.0), sign);
                    for x in &mut v[..3] {
                        *x = *x * scale;
                    }
                }
                table_r | table_g | table_b | table_a => {
                    let c = (op as i32 - table_r as i32) as usize;
                    // SAFETY: selected curve stage has the initialized active table and live backing.
                    v[c] = unsafe { table(arg.cast(), v[c]) };
                }
                clut_A2B => {
                    // SAFETY: the opcode's source profile provides this active A2B shape and backing.
                    unsafe {
                        let p = arg.cast::<A2B>();
                        let mut tmp = v;
                        clut(
                            (*p).input_channels as usize,
                            (*p).output_channels as usize,
                            ptr::addr_of!((*p).grid_points).cast::<u8>(),
                            (*p).grid_8,
                            (*p).grid_16,
                            &mut tmp,
                        );
                        v[..3].copy_from_slice(&tmp[..3]);
                        if (*p).input_channels == 4 {
                            v[3] = F::ONE;
                        }
                    }
                }
                clut_B2A => {
                    // SAFETY: the opcode's destination profile provides this active B2A shape and backing.
                    unsafe {
                        let p = arg.cast::<B2A>();
                        clut(
                            (*p).input_channels as usize,
                            (*p).output_channels as usize,
                            ptr::addr_of!((*p).grid_points).cast::<u8>(),
                            (*p).grid_8,
                            (*p).grid_16,
                            &mut v,
                        );
                    }
                }
                _ => unreachable!(),
            }
        }
    }
}

/// Original baseline program traversal, including its zero-filled four-pixel
/// temporary for the final partial group.
///
/// # Safety
/// The program ends in a store stage. Each context has its matching initialized
/// source object and all reached backing bytes. Input/output spans, alignment,
/// aliasing and dimensions satisfy the source program's accessed extents.
pub(crate) unsafe fn run_program(
    program: &[Op],
    contexts: &[*const ()],
    src: *const u8,
    dst: *mut u8,
    mut n: i32,
    src_bpp: usize,
    dst_bpp: usize,
) {
    let mut i = 0usize;
    while n >= 4 {
        // SAFETY: caller establishes the complete four-pixel group spans.
        unsafe {
            exec_stages(
                program,
                contexts,
                src.add(i * src_bpp),
                dst.add(i * dst_bpp),
            );
        }
        i += 4;
        n -= 4;
    }
    if n > 0 {
        #[repr(align(16))]
        struct Tmp([u8; 64]);
        let mut tmp = Tmp([0; 64]);
        // SAFETY: source retains exactly these memcpy extents. Temporary has
        // the original 4*4*N bytes and alignment sufficient for all formats.
        unsafe {
            ptr::copy_nonoverlapping(
                src.add(i * src_bpp),
                tmp.0.as_mut_ptr(),
                n as usize * src_bpp,
            );
            exec_stages(program, contexts, tmp.0.as_ptr(), tmp.0.as_mut_ptr());
            ptr::copy_nonoverlapping(tmp.0.as_ptr(), dst.add(i * dst_bpp), n as usize * dst_bpp);
        }
    }
}
