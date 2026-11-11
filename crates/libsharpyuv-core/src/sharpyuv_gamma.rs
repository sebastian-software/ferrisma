// libwebp v1.6.0, sharpyuv/sharpyuv_gamma.c:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Gamma correction utilities.

// libwebp v1.6.0, sharpyuv/sharpyuv_gamma.h:
//
// Copyright 2022 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Gamma correction utilities.

//! Translation of `sharpyuv/sharpyuv_gamma.c` and `sharpyuv/sharpyuv_gamma.h` from libwebp v1.6.0.
//!
//! Work package: WP5 (`SharpYUV`).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Gamma tables.** `static` tables filled once by
//!    `SharpYuvInitGammaTables` become `LazyLock` statics with the same
//!    computation and values (ADR 0003). The function forces the computation.
//!    Its values are compared through the transfer functions
//!    (`tests/wp5_gamma.rs`), since the C tables are not visible outside their
//!    file.
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. **Gamma sweep of 65,536 for non-sRGB transfers.** `SharpYuvLinearToGamma`
//!    at 16 bits converts a value above 65,535 from float to `uint16_t`, which
//!    is undefined; the probe observed a wrap to 0 for the inverse of the IEC
//!    61966 transfer, and Rust saturates. The sweep stops at 65,535 for these
//!    transfers. The conversion never reaches this case, because it calls the
//!    functions with bit depths up to 14.
//! 2. **Transfer functions 0, 2, 3 and 19.** The C `default:` branch asserts
//!    and returns a linear value of 0. The Rust branch does the same with
//!    `debug_assert!`. These inputs are not swept.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Floating point across hosts.** Gamma and matrix computations use
//!    `float` and libm. The results are equal on the development host (macOS
//!    ARM64); other hosts (the Linux x86-64 runner) are not claimed.

use std::sync::LazyLock;

use super::sharpyuv::SharpYuvTransferFunctionType;

// upstream: sharpyuv/sharpyuv_gamma.c GAMMA_TO_LINEAR_TAB_BITS
const GAMMA_TO_LINEAR_TAB_BITS: i32 = 10;
// upstream: sharpyuv/sharpyuv_gamma.c GAMMA_TO_LINEAR_TAB_SIZE
const GAMMA_TO_LINEAR_TAB_SIZE: usize = 1 << GAMMA_TO_LINEAR_TAB_BITS;
// upstream: sharpyuv/sharpyuv_gamma.c LINEAR_TO_GAMMA_TAB_BITS
const LINEAR_TO_GAMMA_TAB_BITS: i32 = 9;
// upstream: sharpyuv/sharpyuv_gamma.c LINEAR_TO_GAMMA_TAB_SIZE
const LINEAR_TO_GAMMA_TAB_SIZE: usize = 1 << LINEAR_TO_GAMMA_TAB_BITS;
// upstream: sharpyuv/sharpyuv_gamma.c GAMMA_TO_LINEAR_BITS
const GAMMA_TO_LINEAR_BITS: i32 = 16;
// upstream: sharpyuv/sharpyuv_gamma.c kGammaF
const GAMMA_F: f64 = 1.0 / 0.45;

// upstream: sharpyuv/sharpyuv_gamma.c kGammaToLinearTabS, filled by SharpYuvInitGammaTables
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
static GAMMA_TO_LINEAR_TAB_S: LazyLock<[u32; GAMMA_TO_LINEAR_TAB_SIZE + 2]> = LazyLock::new(|| {
    let mut tab = [0_u32; GAMMA_TO_LINEAR_TAB_SIZE + 2];
    let a = 0.099_296_826_809_44_f64;
    let thresh = 0.018_053_968_510_807_f64;
    let final_scale = f64::from(1 << GAMMA_TO_LINEAR_BITS);
    let norm = 1.0 / GAMMA_TO_LINEAR_TAB_SIZE as f64;
    let a_rec = 1.0 / (1.0 + a);
    for (v, entry) in tab
        .iter_mut()
        .enumerate()
        .take(GAMMA_TO_LINEAR_TAB_SIZE + 1)
    {
        let g = norm * v as f64;
        let value = if g <= thresh * 4.5 {
            g / 4.5
        } else {
            (a_rec * (g + a)).powf(GAMMA_F)
        };
        *entry = (value * final_scale + 0.5) as u32;
    }
    // to prevent small rounding errors to cause read-overflow:
    tab[GAMMA_TO_LINEAR_TAB_SIZE + 1] = tab[GAMMA_TO_LINEAR_TAB_SIZE];
    tab
});

// upstream: sharpyuv/sharpyuv_gamma.c kLinearToGammaTabS, filled by SharpYuvInitGammaTables
#[expect(
    clippy::cast_precision_loss,
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
static LINEAR_TO_GAMMA_TAB_S: LazyLock<[u32; LINEAR_TO_GAMMA_TAB_SIZE + 2]> = LazyLock::new(|| {
    let mut tab = [0_u32; LINEAR_TO_GAMMA_TAB_SIZE + 2];
    let a = 0.099_296_826_809_44_f64;
    let thresh = 0.018_053_968_510_807_f64;
    let final_scale = f64::from(1 << GAMMA_TO_LINEAR_BITS);
    let scale = 1.0 / LINEAR_TO_GAMMA_TAB_SIZE as f64;
    for (v, entry) in tab
        .iter_mut()
        .enumerate()
        .take(LINEAR_TO_GAMMA_TAB_SIZE + 1)
    {
        let g = scale * v as f64;
        let value = if g <= thresh {
            4.5 * g
        } else {
            (1.0 + a) * g.powf(1.0 / GAMMA_F) - a
        };
        *entry = (final_scale * value + 0.5) as u32;
    }
    // to prevent small rounding errors to cause read-overflow:
    tab[LINEAR_TO_GAMMA_TAB_SIZE + 1] = tab[LINEAR_TO_GAMMA_TAB_SIZE];
    tab
});

// upstream: sharpyuv/sharpyuv_gamma.c SharpYuvInitGammaTables
// The tables are computed once, on first use, by the LazyLock statics above, so
// the function only forces that computation.
pub fn sharp_yuv_init_gamma_tables() {
    LazyLock::force(&GAMMA_TO_LINEAR_TAB_S);
    LazyLock::force(&LINEAR_TO_GAMMA_TAB_S);
}

// upstream: sharpyuv/sharpyuv_gamma.c Shift
fn shift(v: i32, shift_amount: i32) -> i32 {
    if shift_amount >= 0 {
        v << shift_amount
    } else {
        v >> -shift_amount
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FixedPointInterpolation
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn fixed_point_interpolation(
    v: i32,
    tab: &[u32],
    tab_pos_shift_right: i32,
    tab_value_shift: i32,
) -> u32 {
    let tab_pos = shift(v, -tab_pos_shift_right) as u32;
    // fractional part, in 'tab_pos_shift' fixed-point precision
    let x = (v as u32).wrapping_sub(tab_pos << tab_pos_shift_right);
    // v0 / v1 are in kGammaToLinearBits fixed-point precision (range [0..1])
    let v0 = shift(tab[tab_pos as usize] as i32, tab_value_shift) as u32;
    let v1 = shift(tab[tab_pos as usize + 1] as i32, tab_value_shift) as u32;
    // Final interpolation.
    let v2 = v1.wrapping_sub(v0).wrapping_mul(x); // note: v1 >= v0.
    let half = if tab_pos_shift_right > 0 {
        1_i32 << (tab_pos_shift_right - 1)
    } else {
        0
    };
    v0.wrapping_add(v2.wrapping_add(half as u32) >> tab_pos_shift_right)
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearSrgb
fn to_linear_srgb(v: u16, bit_depth: i32) -> u32 {
    to_linear_srgb_tab(v, bit_depth, &*GAMMA_TO_LINEAR_TAB_S)
}

fn to_linear_srgb_tab(v: u16, bit_depth: i32, tab: &[u32]) -> u32 {
    let shift_amount = GAMMA_TO_LINEAR_TAB_BITS - bit_depth;
    if shift_amount > 0 {
        return tab[usize::from(v) << shift_amount];
    }
    fixed_point_interpolation(i32::from(v), tab, -shift_amount, 0)
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearSrgb
fn from_linear_srgb(value: u32, bit_depth: i32) -> u16 {
    from_linear_srgb_tab(value, bit_depth, &*LINEAR_TO_GAMMA_TAB_S)
}

#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn from_linear_srgb_tab(value: u32, bit_depth: i32, tab: &[u32]) -> u16 {
    fixed_point_interpolation(
        value as i32,
        tab,
        GAMMA_TO_LINEAR_BITS - LINEAR_TO_GAMMA_TAB_BITS,
        bit_depth - GAMMA_TO_LINEAR_BITS,
    ) as u16
}

/// Per-conversion lookup context. The sRGB tables are forced once and then
/// borrowed directly by the hot loop; public gamma functions retain their API.
pub(crate) struct GammaContext {
    transfer_type: SharpYuvTransferFunctionType,
    to_linear: Option<&'static [u32]>,
    from_linear: Option<&'static [u32]>,
}

impl GammaContext {
    #[must_use]
    pub(crate) fn new(transfer_type: SharpYuvTransferFunctionType) -> Self {
        let (to_linear, from_linear) =
            if transfer_type == SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb {
                (
                    Some(&GAMMA_TO_LINEAR_TAB_S[..]),
                    Some(&LINEAR_TO_GAMMA_TAB_S[..]),
                )
            } else {
                (None, None)
            };
        Self {
            transfer_type,
            to_linear,
            from_linear,
        }
    }

    #[must_use]
    pub(crate) fn to_linear(&self, value: u16, bit_depth: i32) -> u32 {
        if let Some(table) = self.to_linear {
            to_linear_srgb_tab(value, bit_depth, table)
        } else {
            sharp_yuv_gamma_to_linear(value, bit_depth, self.transfer_type)
        }
    }

    #[must_use]
    pub(crate) fn to_gamma(&self, value: u32, bit_depth: i32) -> u16 {
        if let Some(table) = self.from_linear {
            from_linear_srgb_tab(value, bit_depth, table)
        } else {
            sharp_yuv_linear_to_gamma(value, bit_depth, self.transfer_type)
        }
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c CLAMP
fn clamp(x: f32, low: f32, high: f32) -> f32 {
    if x < low {
        low
    } else if high < x {
        high
    } else {
        x
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c MIN
fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else { b }
}

// upstream: sharpyuv/sharpyuv_gamma.c MAX
fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else { b }
}

// upstream: sharpyuv/sharpyuv_gamma.c Roundf
#[expect(
    clippy::cast_possible_truncation,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn roundf(x: f32) -> f32 {
    if x < 0.0 {
        f64::from(x - 0.5).ceil() as f32
    } else {
        f64::from(x + 0.5).floor() as f32
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c Powf
#[expect(
    clippy::cast_possible_truncation,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn powf(base: f32, exp: f32) -> f32 {
    f64::from(base).powf(f64::from(exp)) as f32
}

// upstream: sharpyuv/sharpyuv_gamma.c Log10f
#[expect(
    clippy::cast_possible_truncation,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
fn log10f(x: f32) -> f32 {
    f64::from(x).log10() as f32
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinear709
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn to_linear_709(gamma: f32) -> f32 {
    if gamma < 0.0 {
        0.0
    } else if gamma < 4.5_f32 * 0.018_053_968_510_807_f32 {
        gamma / 4.5
    } else if gamma < 1.0 {
        powf(
            (gamma + 0.099_296_826_809_44_f32) / 1.099_296_826_809_44_f32,
            1.0_f32 / 0.45_f32,
        )
    } else {
        1.0
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinear709
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn from_linear_709(linear: f32) -> f32 {
    if linear < 0.0 {
        0.0
    } else if linear < 0.018_053_968_510_807_f32 {
        linear * 4.5
    } else if linear < 1.0 {
        1.099_296_826_809_44_f32 * powf(linear, 0.45_f32) - 0.099_296_826_809_44_f32
    } else {
        1.0
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinear470M
fn to_linear_470m(gamma: f32) -> f32 {
    powf(clamp(gamma, 0.0, 1.0), 2.2_f32)
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinear470M
fn from_linear_470m(linear: f32) -> f32 {
    powf(clamp(linear, 0.0, 1.0), 1.0_f32 / 2.2_f32)
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinear470Bg
fn to_linear_470bg(gamma: f32) -> f32 {
    powf(clamp(gamma, 0.0, 1.0), 2.8_f32)
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinear470Bg
fn from_linear_470bg(linear: f32) -> f32 {
    powf(clamp(linear, 0.0, 1.0), 1.0_f32 / 2.8_f32)
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearSmpte240
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn to_linear_smpte240(gamma: f32) -> f32 {
    if gamma < 0.0 {
        0.0
    } else if gamma < 4.0_f32 * 0.022_821_585_529_445_f32 {
        gamma / 4.0
    } else if gamma < 1.0 {
        powf(
            (gamma + 0.111_572_195_921_731_f32) / 1.111_572_195_921_731_f32,
            1.0_f32 / 0.45_f32,
        )
    } else {
        1.0
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearSmpte240
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn from_linear_smpte240(linear: f32) -> f32 {
    if linear < 0.0 {
        0.0
    } else if linear < 0.022_821_585_529_445_f32 {
        linear * 4.0
    } else if linear < 1.0 {
        1.111_572_195_921_731_f32 * powf(linear, 0.45_f32) - 0.111_572_195_921_731_f32
    } else {
        1.0
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearLog100
fn to_linear_log100(gamma: f32) -> f32 {
    // The function is non-bijective so choose the middle of [0, 0.01].
    let mid_interval = 0.01_f32 / 2.0;
    if gamma <= 0.0 {
        mid_interval
    } else {
        powf(10.0, 2.0 * (min(gamma, 1.0) - 1.0))
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearLog100
fn from_linear_log100(linear: f32) -> f32 {
    if linear < 0.01_f32 {
        0.0
    } else {
        1.0 + log10f(min(linear, 1.0)) / 2.0
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearLog100Sqrt10
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn to_linear_log100_sqrt10(gamma: f32) -> f32 {
    // The function is non-bijective so choose the middle of [0, 0.00316227766f[.
    let mid_interval = 0.003_162_277_66_f32 / 2.0;
    if gamma <= 0.0 {
        mid_interval
    } else {
        powf(10.0, 2.5 * (min(gamma, 1.0) - 1.0))
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearLog100Sqrt10
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn from_linear_log100_sqrt10(linear: f32) -> f32 {
    if linear < 0.003_162_277_66_f32 {
        0.0
    } else {
        1.0 + log10f(min(linear, 1.0)) / 2.5
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearIec61966
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn to_linear_iec61966(gamma: f32) -> f32 {
    if gamma <= -4.5_f32 * 0.018_053_968_510_807_f32 {
        powf(
            (-gamma + 0.099_296_826_809_44_f32) / -1.099_296_826_809_44_f32,
            1.0_f32 / 0.45_f32,
        )
    } else if gamma < 4.5_f32 * 0.018_053_968_510_807_f32 {
        gamma / 4.5
    } else {
        powf(
            (gamma + 0.099_296_826_809_44_f32) / 1.099_296_826_809_44_f32,
            1.0_f32 / 0.45_f32,
        )
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearIec61966
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn from_linear_iec61966(linear: f32) -> f32 {
    if linear <= -0.018_053_968_510_807_f32 {
        -1.099_296_826_809_44_f32 * powf(-linear, 0.45_f32) + 0.099_296_826_809_44_f32
    } else if linear < 0.018_053_968_510_807_f32 {
        linear * 4.5
    } else {
        1.099_296_826_809_44_f32 * powf(linear, 0.45_f32) - 0.099_296_826_809_44_f32
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearBt1361
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn to_linear_bt1361(gamma: f32) -> f32 {
    if gamma < -0.25 {
        -0.25
    } else if gamma < 0.0 {
        powf(
            (gamma - 0.024_824_206_702_36_f32) / -0.274_824_206_702_36_f32,
            1.0_f32 / 0.45_f32,
        ) / -4.0
    } else if gamma < 4.5_f32 * 0.018_053_968_510_807_f32 {
        gamma / 4.5
    } else if gamma < 1.0 {
        powf(
            (gamma + 0.099_296_826_809_44_f32) / 1.099_296_826_809_44_f32,
            1.0_f32 / 0.45_f32,
        )
    } else {
        1.0
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearBt1361
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn from_linear_bt1361(linear: f32) -> f32 {
    if linear < -0.25 {
        -0.25
    } else if linear < 0.0 {
        -0.274_824_206_702_36_f32 * powf(-4.0 * linear, 0.45_f32) + 0.024_824_206_702_36_f32
    } else if linear < 0.018_053_968_510_807_f32 {
        linear * 4.5
    } else if linear < 1.0 {
        1.099_296_826_809_44_f32 * powf(linear, 0.45_f32) - 0.099_296_826_809_44_f32
    } else {
        1.0
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearPq
fn to_linear_pq(gamma: f32) -> f32 {
    if gamma > 0.0 {
        let pow_gamma = powf(gamma, 32.0_f32 / 2523.0_f32);
        let num = max(pow_gamma - 107.0_f32 / 128.0_f32, 0.0);
        let den = max(
            2413.0_f32 / 128.0_f32 - 2392.0_f32 / 128.0_f32 * pow_gamma,
            f32::MIN_POSITIVE,
        );
        return powf(num / den, 4096.0_f32 / 653.0_f32);
    }
    0.0
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearPq
fn from_linear_pq(linear: f32) -> f32 {
    if linear > 0.0 {
        let pow_linear = powf(linear, 653.0_f32 / 4096.0_f32);
        let num = 107.0_f32 / 128.0_f32 + 2413.0_f32 / 128.0_f32 * pow_linear;
        let den = 1.0_f32 + 2392.0_f32 / 128.0_f32 * pow_linear;
        return powf(num / den, 2523.0_f32 / 32.0_f32);
    }
    0.0
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearSmpte428
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn to_linear_smpte428(gamma: f32) -> f32 {
    powf(max(gamma, 0.0), 2.6_f32) / 0.916_555_279_740_309_34_f32
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearSmpte428
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn from_linear_smpte428(linear: f32) -> f32 {
    powf(
        0.916_555_279_740_309_34_f32 * max(linear, 0.0),
        1.0_f32 / 2.6_f32,
    )
}

// upstream: sharpyuv/sharpyuv_gamma.c ToLinearHlg
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn to_linear_hlg(gamma: f32) -> f32 {
    if gamma < 0.0 {
        0.0
    } else if gamma <= 0.5 {
        powf((gamma * gamma) * (1.0_f32 / 3.0_f32), 1.2_f32)
    } else {
        powf(
            (((gamma - 0.559_910_73_f32) / 0.178_832_77_f32).exp() + 0.284_668_92_f32) / 12.0_f32,
            1.2_f32,
        )
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c FromLinearHlg
#[expect(
    clippy::excessive_precision,
    reason = "C float literals, rounded to f32 as in C"
)]
fn from_linear_hlg(linear: f32) -> f32 {
    let linear = powf(linear, 1.0_f32 / 1.2_f32);
    if linear < 0.0 {
        0.0
    } else if linear <= 1.0_f32 / 12.0_f32 {
        (3.0_f32 * linear).sqrt()
    } else {
        0.178_832_77_f32 * (12.0_f32 * linear - 0.284_668_92_f32).ln() + 0.559_910_73_f32
    }
}

// upstream: sharpyuv/sharpyuv_gamma.c SharpYuvGammaToLinear
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
#[must_use]
pub fn sharp_yuv_gamma_to_linear(
    v: u16,
    bit_depth: i32,
    transfer_type: SharpYuvTransferFunctionType,
) -> u32 {
    if transfer_type == SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb {
        return to_linear_srgb(v, bit_depth);
    }
    let v_float = f32::from(v) / ((1 << bit_depth) - 1) as f32;
    let linear = match transfer_type {
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt709
        | SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt601
        | SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_10Bit
        | SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_12Bit => {
            to_linear_709(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470M => to_linear_470m(v_float),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470Bg => to_linear_470bg(v_float),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte240 => {
            to_linear_smpte240(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLinear => return u32::from(v),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100 => to_linear_log100(v_float),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100_Sqrt10 => {
            to_linear_log100_sqrt10(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionIec61966 => {
            to_linear_iec61966(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt1361 => to_linear_bt1361(v_float),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte2084 => to_linear_pq(v_float),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte428 => {
            to_linear_smpte428(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionHlg => to_linear_hlg(v_float),
        _ => {
            // upstream: assert(0) in the default branch; the value is undefined
            debug_assert!(false, "unknown SharpYuv transfer function");
            0.0
        }
    };
    roundf(linear * 65535.0) as u32
}

// upstream: sharpyuv/sharpyuv_gamma.c SharpYuvLinearToGamma
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_precision_loss,
    clippy::cast_sign_loss,
    reason = "C integer and float conversions, kept as in the source (ADR 0003)"
)]
#[must_use]
pub fn sharp_yuv_linear_to_gamma(
    v: u32,
    bit_depth: i32,
    transfer_type: SharpYuvTransferFunctionType,
) -> u16 {
    if transfer_type == SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSrgb {
        return from_linear_srgb(v, bit_depth);
    }
    let v_float = v as f32 / 65535.0;
    let linear = match transfer_type {
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt709
        | SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt601
        | SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_10Bit
        | SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt2020_12Bit => {
            from_linear_709(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470M => from_linear_470m(v_float),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt470Bg => {
            from_linear_470bg(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte240 => {
            from_linear_smpte240(v_float)
        }
        // The value is truncated to 16 bits, as the C return type does.
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLinear => return v as u16,
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100 => {
            from_linear_log100(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionLog100_Sqrt10 => {
            from_linear_log100_sqrt10(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionIec61966 => {
            from_linear_iec61966(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionBt1361 => {
            from_linear_bt1361(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte2084 => from_linear_pq(v_float),
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionSmpte428 => {
            from_linear_smpte428(v_float)
        }
        SharpYuvTransferFunctionType::kSharpYuvTransferFunctionHlg => from_linear_hlg(v_float),
        _ => {
            // upstream: assert(0) in the default branch; the value is undefined
            debug_assert!(false, "unknown SharpYuv transfer function");
            0.0
        }
    };
    roundf(linear * ((1 << bit_depth) - 1) as f32) as u16
}
