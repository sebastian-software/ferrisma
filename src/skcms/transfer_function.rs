// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Transfer functions from pinned skcms.cc:44-300,1980-2112 and
//! src/skcms_public.h. Profile parsing and pixel transforms are separate units.
//!
//! The pinned native Apple Clang build contracts multiply/add expressions on
//! AArch64. Explicit contraction below retains those original operations;
//! the baseline x86-64 source uses separate operations. Native parity remains
//! required for each compiler configuration before exposing CMS integration.

#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
pub struct TransferFunction {
    pub g: f32,
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum TfType {
    Invalid = 0,
    Srgbish = 1,
    Pqish = 2,
    Hlgish = 3,
    HlgInvish = 4,
    Pq = 5,
    Hlg = 6,
}

#[inline]
fn madd(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        a * b + c
    }
}
#[inline]
fn addmul(c: f32, a: f32, b: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        a.mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        c + a * b
    }
}
#[inline]
fn msub(a: f32, b: f32, c: f32) -> f32 {
    #[cfg(target_arch = "aarch64")]
    {
        (-a).mul_add(b, c)
    }
    #[cfg(not(target_arch = "aarch64"))]
    {
        c - a * b
    }
}
#[expect(
    clippy::excessive_precision,
    reason = "Preserve original skcms literals."
)]
pub(crate) fn log2f(x: f32) -> f32 {
    let bits = x.to_bits() as i32;
    let e = bits as f32 * (1.0 / (1 << 23) as f32);
    let m = f32::from_bits(((bits & 0x007fffff) | 0x3f000000) as u32);
    msub(1.498030302, m, e - 124.225514990) - 1.725879990 / (0.3520887068 + m)
}
#[expect(
    clippy::excessive_precision,
    clippy::approx_constant,
    reason = "Preserve original skcms literal."
)]
pub(crate) fn logf(x: f32) -> f32 {
    0.69314718 * log2f(x)
}
#[expect(
    clippy::excessive_precision,
    reason = "Preserve original skcms literals."
)]
fn exp2f(mut x: f32) -> f32 {
    if x > 128.0 {
        return f32::INFINITY;
    } else if x < -127.0 {
        return 0.0;
    }
    // Original floorf_ casts to int before adjusting downward. This reached
    // interval is in range; NaN has no defined original cast result.
    assert!(!x.is_nan(), "original skcms float-to-int precondition");
    let roundtrip = (x as i32) as f32;
    let floor = if roundtrip > x {
        roundtrip - 1.0
    } else {
        roundtrip
    };
    let fract = x - floor;
    let fbits = (1 << 23) as f32
        * (msub(1.490129070, fract, x + 121.274057500) + 27.728023300 / (4.84252568 - fract));
    if fbits >= i32::MAX as f32 {
        return f32::INFINITY;
    } else if fbits < 0.0 {
        return 0.0;
    }
    assert!(!fbits.is_nan(), "original skcms float-to-int precondition");
    x = f32::from_bits((fbits as i32) as u32);
    x
}
pub fn powf(x: f32, y: f32) -> f32 {
    if x <= 0.0 {
        return 0.0;
    }
    if x == 1.0 {
        return 1.0;
    }
    exp2f(log2f(x) * y)
}
#[expect(
    clippy::excessive_precision,
    clippy::approx_constant,
    reason = "Preserve original skcms literal."
)]
fn expf(x: f32) -> f32 {
    exp2f(1.4426950408889634074 * x)
}
fn finite(x: f32) -> bool {
    0.0 == x * 0.0
}
fn fabs(x: f32) -> f32 {
    if x < 0.0 { -x } else { x }
}
fn marker(kind: TfType) -> f32 {
    -(kind as i32 as f32)
}

/// Fieldwise original classifier. Only source-reached fields must be initialized.
///
/// # Safety
/// Pointer has aligned function storage and every field reached by the original
/// short circuit is initialized. This includes all words on the nonnegative path.
pub(crate) unsafe fn classify_raw(tf: *const TransferFunction) -> TfType {
    // SAFETY: raw field places are read only after the original short circuits;
    // no reference or full value is formed from unknown inactive words.
    unsafe {
        if (*tf).g < 0.0 {
            if (*tf).g < -128.0 {
                return TfType::Invalid;
            }
            let kind = -((*tf).g as i32);
            if (-kind) as f32 != (*tf).g {
                return TfType::Invalid;
            }
            return match kind {
                2 => TfType::Pqish,
                3 => TfType::Hlgish,
                4 => TfType::HlgInvish,
                5 if (*tf).b == 0.0
                    && (*tf).c == 0.0
                    && (*tf).d == 0.0
                    && (*tf).e == 0.0
                    && (*tf).f == 0.0 =>
                {
                    TfType::Pq
                }
                6 if (*tf).d == 0.0 && (*tf).e == 0.0 && (*tf).f == 0.0 => TfType::Hlg,
                _ => TfType::Invalid,
            };
        }
        if finite((*tf).a + (*tf).b + (*tf).c + (*tf).d + (*tf).e + (*tf).f + (*tf).g)
            && (*tf).a >= 0.0
            && (*tf).c >= 0.0
            && (*tf).d >= 0.0
            && (*tf).g >= 0.0
            && madd((*tf).a, (*tf).d, (*tf).b) >= 0.0
        {
            TfType::Srgbish
        } else {
            TfType::Invalid
        }
    }
}

/// Original inversion for a possibly partial source and initialized local output.
///
/// # Safety
/// Source initializes exactly the fields reached by original classification and
/// inversion. The initialized destination is disjoint from source storage;
/// callers use a local destination before any external output assignment.
pub(crate) unsafe fn invert_raw(
    source: *const TransferFunction,
    destination: &mut TransferFunction,
) -> bool {
    // SAFETY: original immediate-invalid / unsupported paths read only their
    // classifier fields. Every remaining inversion reaches all seven words.
    unsafe {
        let kind = classify_raw(source);
        if matches!(kind, TfType::Invalid | TfType::Pq | TfType::Hlg) {
            return false;
        }
        let value = source.read();
        value.invert_classified(kind, destination)
    }
}

/// Original scalar evaluator with lazy classifier/source-field requirements.
///
/// # Safety
/// Source initializes every field reached by original classification/evaluation.
/// Immediate-invalid markers may leave a..f unknown. Built-in PQ/HLG markers
/// require only classifier-tested fields; their standard formulas ignore other
/// function parameters. Reached scalar math retains its source preconditions.
pub(crate) unsafe fn eval_raw(source: *const TransferFunction, mut x: f32) -> f32 {
    let sign = if x < 0.0 { -1.0 } else { 1.0 };
    x *= sign;
    // SAFETY: preserve source input arithmetic before the lazy classifier.
    // Only branches that actually reach all seven fields materialize a value.
    unsafe {
        let kind = classify_raw(source);
        match kind {
            TfType::Invalid => 0.0,
            TfType::Pq | TfType::Hlg => TransferFunction::default().eval_classified(kind, x, sign),
            _ => source.read().eval_classified(kind, x, sign),
        }
    }
}

impl TransferFunction {
    pub fn get_type(&self) -> TfType {
        // SAFETY: a safe function value initializes all seven fields, which
        // covers every possible original classifier path.
        unsafe { classify_raw(self) }
    }

    pub fn is_srgbish(&self) -> bool {
        self.get_type() == TfType::Srgbish
    }
    pub fn is_pqish(&self) -> bool {
        self.get_type() == TfType::Pqish
    }
    pub fn is_hlgish(&self) -> bool {
        self.get_type() == TfType::Hlgish
    }
    pub fn is_pq(&self) -> bool {
        self.get_type() == TfType::Pq
    }
    pub fn is_hlg(&self) -> bool {
        self.get_type() == TfType::Hlg
    }

    pub fn make_pqish(&mut self, a: f32, b: f32, c: f32, d: f32, e: f32, f: f32) -> bool {
        *self = Self {
            g: marker(TfType::Pqish),
            a,
            b,
            c,
            d,
            e,
            f,
        };
        true
    }
    pub fn make_scaled_hlgish(&mut self, k: f32, r: f32, g: f32, a: f32, b: f32, c: f32) -> bool {
        *self = Self {
            g: marker(TfType::Hlgish),
            a: r,
            b: g,
            c: a,
            d: b,
            e: c,
            f: k - 1.0,
        };
        true
    }
    pub fn make_hlgish(&mut self, r: f32, g: f32, a: f32, b: f32, c: f32) -> bool {
        self.make_scaled_hlgish(1.0, r, g, a, b, c)
    }
    pub fn make_pq(&mut self, white: f32) {
        *self = Self {
            g: marker(TfType::Pq),
            a: white,
            b: 0.0,
            c: 0.0,
            d: 0.0,
            e: 0.0,
            f: 0.0,
        };
    }
    pub fn make_hlg(&mut self, white: f32, peak: f32, gamma: f32) {
        *self = Self {
            g: marker(TfType::Hlg),
            a: white,
            b: peak,
            c: gamma,
            d: 0.0,
            e: 0.0,
            f: 0.0,
        };
    }

    pub fn eval(&self, mut x: f32) -> f32 {
        let sign = if x < 0.0 { -1.0 } else { 1.0 };
        x *= sign;
        self.eval_classified(self.get_type(), x, sign)
    }
    #[expect(clippy::excessive_precision, reason = "Preserve original HLG literal.")]
    fn eval_classified(&self, kind: TfType, mut x: f32, sign: f32) -> f32 {
        match kind {
            TfType::Invalid => 0.0,
            TfType::Hlg => {
                sign * if x <= 0.5 {
                    x * x / 3.0
                } else {
                    (expf((x - 0.55991073) / 0.17883277) + 0.28466892) / 12.0
                }
            }
            TfType::Hlgish => {
                let k = self.f + 1.0;
                k * sign
                    * if x * self.a <= 1.0 {
                        powf(x * self.a, self.b)
                    } else {
                        expf((x - self.e) * self.c) + self.d
                    }
            }
            TfType::HlgInvish => {
                x /= self.f + 1.0;
                sign * if x <= 1.0 {
                    self.a * powf(x, self.b)
                } else {
                    madd(self.c, logf(x - self.d), self.e)
                }
            }
            TfType::Srgbish => {
                sign * if x < self.d {
                    madd(self.c, x, self.f)
                } else {
                    powf(madd(self.a, x, self.b), self.g) + self.e
                }
            }
            TfType::Pq => {
                let p = powf(x, 1.0 / (2523.0 / 32.0));
                powf(
                    (p - 107.0 / 128.0) / msub(2392.0 / 128.0, p, 2413.0 / 128.0),
                    1.0 / (1305.0 / 8192.0),
                )
            }
            TfType::Pqish => {
                sign * powf(
                    addmul(self.a, self.b, powf(x, self.c))
                        / addmul(self.d, self.e, powf(x, self.c)),
                    self.f,
                )
            }
        }
    }

    pub fn invert_into(&self, dst: &mut Self) -> bool {
        self.invert_classified(self.get_type(), dst)
    }
    fn invert_classified(&self, kind: TfType, dst: &mut Self) -> bool {
        match kind {
            TfType::Invalid | TfType::Pq | TfType::Hlg => return false,
            TfType::Pqish => {
                *dst = Self {
                    g: marker(TfType::Pqish),
                    a: -self.a,
                    b: self.d,
                    c: 1.0 / self.f,
                    d: self.b,
                    e: -self.e,
                    f: 1.0 / self.c,
                };
                return true;
            }
            kind @ (TfType::Hlgish | TfType::HlgInvish) => {
                let kind = if kind == TfType::Hlgish {
                    TfType::HlgInvish
                } else {
                    TfType::Hlgish
                };
                *dst = Self {
                    g: marker(kind),
                    a: 1.0 / self.a,
                    b: 1.0 / self.b,
                    c: 1.0 / self.c,
                    d: self.d,
                    e: self.e,
                    f: self.f,
                };
                return true;
            }
            TfType::Srgbish => {}
        }
        let mut inv = Self::default();
        let d_l = madd(self.c, self.d, self.f);
        let d_r = powf(madd(self.a, self.d, self.b), self.g) + self.e;
        if fabs(d_l - d_r) > 1.0 / 512.0 {
            return false;
        }
        inv.d = d_l;
        if inv.d > 0.0 {
            inv.c = 1.0 / self.c;
            inv.f = -self.f / self.c;
        }
        let k = powf(self.a, -self.g);
        inv.g = 1.0 / self.g;
        inv.a = k;
        inv.b = -k * self.e;
        inv.e = -self.b / self.a;
        if inv.a < 0.0 {
            return false;
        }
        if madd(inv.a, inv.d, inv.b) < 0.0 {
            inv.b = -inv.a * inv.d;
        }
        if inv.get_type() != TfType::Srgbish {
            return false;
        }
        let mut s = self.eval(1.0);
        if !finite(s) {
            return false;
        }
        let sign = if s < 0.0 { -1.0 } else { 1.0 };
        s *= sign;
        if s < inv.d {
            inv.f = msub(sign * inv.c, s, 1.0);
        } else {
            inv.e = msub(sign, powf(madd(inv.a, s, inv.b), inv.g), 1.0);
        }
        *dst = inv;
        dst.get_type() == TfType::Srgbish
    }

    /// Source `src == dst` call. All seven input words are initialized; their
    /// value snapshot avoids conflicting Rust references. The source likewise
    /// finishes reading input parameters before its first output assignment.
    pub fn invert_in_place(&mut self) -> bool {
        let source = *self;
        source.invert_into(self)
    }
}
