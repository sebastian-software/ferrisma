//! An independent linear-light resize oracle for the engine's Area and
//! Lanczos3 filters (issue #18, ADR-0013/0014/0016/0021).
//!
//! This file is written from the definitions below and shares no code with
//! `src/quality.rs`, `src/working_color.rs`, `src/quality_execute.rs` or
//! `src/fast_execute.rs`. It differs from the engine's own `f64` reference in
//! structure on purpose:
//!
//! - **Coordinates.** Source pixel `i` is the cell `[i, i + 1)`; output pixel
//!   `o` of a `n -> m` resize covers `[o * n / m, (o + 1) * n / m)`, so its
//!   center is `(o + 0.5) * n / m`. The kernel is stretched by
//!   `s = max(n / m, 1)` (antialiasing on downscale only).
//! - **Lanczos3.** `L(t) = sinc(t) * sinc(t / 3)` for `|t| < 3`, else 0, with
//!   `sinc(t) = sin(pi t) / (pi t)`. Tap weight `L((i + 0.5 - c) / s)`,
//!   evaluated for *every* source index (no support pre-window, no epsilon
//!   dropping of small taps).
//! - **Area.** A box of width `s` centered on the output center, integrated
//!   exactly over each source cell: the weight is the length of the overlap
//!   of `[c - s/2, c + s/2]` and `[i, i + 1)`. On downscale this is the
//!   pixel-area average; on upscale (`s = 1`) the overlap is `1 - |i + 0.5 - c|`,
//!   i.e. a tent (bilinear interpolation).
//! - **Borders.** Taps outside the image do not exist; each output's weights
//!   are renormalized to sum to one (ImageMagick's `resize.c` and the
//!   engine use the same convention; it is checked against ImageMagick).
//! - **Two dimensions.** The 2D result is the direct double sum
//!   `sum_j sum_i wy[j] * wx[i] * p(i, j)` over the premultiplied samples,
//!   not two sequential 1D passes with an intermediate image.
//! - **Color.** sRGB-coded samples are decoded with the IEC 61966-2-1 EOTF.
//!   Display P3 (D65, sRGB transfer) is converted to linear sRGB by a matrix
//!   derived here from the published primaries and white point, not the
//!   CSS Color 4 rational matrices the engine uses.
//! - **Alpha.** Straight alpha is associated in linear light before
//!   filtering. The filter result is kept associated (premultiplied) and
//!   unclamped, which is convention-free and is what is compared against
//!   ImageMagick.
//! - **Output step** (`quantize`), the documented engine contract: the
//!   output alpha is `clamp(a, 0, 1)` and its byte `round(255 * alpha)`;
//!   behind alpha byte 0, RGB is 0; otherwise the associated color is
//!   divided by the *clamped* output alpha, clamped to `[0, 1]` and
//!   sRGB-encoded. ImageMagick instead divides by the unclamped alpha, which
//!   differs only where Lanczos overshoot pushes alpha above one; see
//!   `tests/fixtures/resize/README.md`.

#![allow(dead_code)]

/// FNV-1a 64; mirrors `fnv1a64` in `tests/fixtures/resize/tools/generate.py`.
pub fn fnv1a64(data: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for byte in data {
        hash ^= u64::from(*byte);
        hash = hash.wrapping_mul(0x0000_0100_0000_01b3);
    }
    hash
}

/// The deterministic RGBA8 pattern of `generate.py` (`pattern`), bit for bit.
pub fn pattern(width: u32, height: u32, alpha: bool, seed: u64) -> Vec<u8> {
    let mut state = seed;
    let mut out = Vec::with_capacity(width as usize * height as usize * 4);
    for y in 0..height {
        for x in 0..width {
            let mut draws = [0_u8; 4];
            for draw in &mut draws {
                state = state
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                *draw = (state >> 56) as u8;
            }
            let region = (x * 4) / width;
            let mut rgb = match region {
                0 => [draws[0], draws[1], draws[2]],
                1 => {
                    let v = if (x + y) & 1 == 1 { 255 } else { 0 };
                    [v, v, v]
                }
                2 => {
                    if (x * 8) / width < 5 {
                        [255, 0, 0]
                    } else {
                        [0, 0, 255]
                    }
                }
                _ => [
                    ((x * 255) / (width - 1)) as u8,
                    ((y * 255) / (height - 1)) as u8,
                    (draws[0] & 1) * 255,
                ],
            };
            let mut a = 255;
            if alpha {
                match (x * 2) / width + 2 * ((y * 2) / height) {
                    1 => {
                        rgb = [0, 255, 0];
                        a = 0;
                    }
                    2 => a = if (x / 2) & 1 == 1 { 255 } else { 0 },
                    3 => a = draws[3],
                    _ => {}
                }
            }
            out.extend_from_slice(&[rgb[0], rgb[1], rgb[2], a]);
        }
    }
    out
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Kernel {
    Area,
    Lanczos3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Primaries {
    Srgb,
    DisplayP3,
}

/// IEC 61966-2-1 sRGB EOTF on a code value in `[0, 1]`.
pub fn srgb_eotf(v: f64) -> f64 {
    if v <= 0.040_45 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

/// IEC 61966-2-1 inverse EOTF, defined on `[0, 1]`.
pub fn srgb_oetf(l: f64) -> f64 {
    if l <= 0.003_130_8 {
        l * 12.92
    } else {
        1.055 * l.powf(1.0 / 2.4) - 0.055
    }
}

/// The explicit output step: clamp linear light, encode, round to 8 bits.
pub fn encode8(linear: f64) -> u8 {
    let clamped = linear.clamp(0.0, 1.0);
    (srgb_oetf(clamped) * 255.0).round() as u8
}

pub fn alpha8(alpha: f64) -> u8 {
    (alpha.clamp(0.0, 1.0) * 255.0).round() as u8
}

type Mat3 = [[f64; 3]; 3];

fn mul(a: Mat3, b: Mat3) -> Mat3 {
    let mut out = [[0.0; 3]; 3];
    for (r, row) in out.iter_mut().enumerate() {
        for (c, value) in row.iter_mut().enumerate() {
            *value = (0..3).map(|k| a[r][k] * b[k][c]).sum();
        }
    }
    out
}

fn inverse(m: Mat3) -> Mat3 {
    let det = m[0][0] * (m[1][1] * m[2][2] - m[1][2] * m[2][1])
        - m[0][1] * (m[1][0] * m[2][2] - m[1][2] * m[2][0])
        + m[0][2] * (m[1][0] * m[2][1] - m[1][1] * m[2][0]);
    let cof =
        |r0: usize, r1: usize, c0: usize, c1: usize| m[r0][c0] * m[r1][c1] - m[r0][c1] * m[r1][c0];
    [
        [
            cof(1, 2, 1, 2) / det,
            -cof(0, 2, 1, 2) / det,
            cof(0, 1, 1, 2) / det,
        ],
        [
            -cof(1, 2, 0, 2) / det,
            cof(0, 2, 0, 2) / det,
            -cof(0, 1, 0, 2) / det,
        ],
        [
            cof(1, 2, 0, 1) / det,
            -cof(0, 2, 0, 1) / det,
            cof(0, 1, 0, 1) / det,
        ],
    ]
}

/// RGB -> XYZ from xy chromaticities of the primaries and white point
/// (SMPTE RP 177 derivation).
fn rgb_to_xyz(primaries: [(f64, f64); 3], white: (f64, f64)) -> Mat3 {
    let column = |(x, y): (f64, f64)| [x / y, 1.0, (1.0 - x - y) / y];
    let p = primaries.map(column);
    let m = [
        [p[0][0], p[1][0], p[2][0]],
        [p[0][1], p[1][1], p[2][1]],
        [p[0][2], p[1][2], p[2][2]],
    ];
    let w = column(white);
    let inv = inverse(m);
    let s: [f64; 3] = std::array::from_fn(|r| (0..3).map(|k| inv[r][k] * w[k]).sum());
    std::array::from_fn(|r| std::array::from_fn(|c| m[r][c] * s[c]))
}

const D65: (f64, f64) = (0.3127, 0.3290);
const SRGB: [(f64, f64); 3] = [(0.64, 0.33), (0.30, 0.60), (0.15, 0.06)];
/// SMPTE EG 432-1 Display P3 primaries.
const P3: [(f64, f64); 3] = [(0.680, 0.320), (0.265, 0.690), (0.150, 0.060)];

/// Linear Display P3 to linear sRGB; both D65, so no adaptation.
pub fn p3_to_srgb_matrix() -> Mat3 {
    mul(inverse(rgb_to_xyz(SRGB, D65)), rgb_to_xyz(P3, D65))
}

/// Decode one RGB8 pixel to linear sRGB (possibly out of `[0, 1]` for P3).
pub fn decode_rgb(rgb: [u8; 3], primaries: Primaries, p3: &Mat3) -> [f64; 3] {
    let linear = rgb.map(|v| srgb_eotf(f64::from(v) / 255.0));
    match primaries {
        Primaries::Srgb => linear,
        Primaries::DisplayP3 => std::array::from_fn(|r| (0..3).map(|k| p3[r][k] * linear[k]).sum()),
    }
}

fn sinc(t: f64) -> f64 {
    if t == 0.0 {
        1.0
    } else {
        let x = std::f64::consts::PI * t;
        x.sin() / x
    }
}

/// Normalized weights of every source index `0..n` for output `o` of `m`.
pub fn axis_weights(kernel: Kernel, n: u32, m: u32, o: u32) -> Vec<f64> {
    let ratio = f64::from(n) / f64::from(m);
    let center = (f64::from(o) + 0.5) * ratio;
    let s = ratio.max(1.0);
    let mut w: Vec<f64> = (0..n)
        .map(|i| {
            let i = f64::from(i);
            match kernel {
                Kernel::Lanczos3 => {
                    let t = (i + 0.5 - center) / s;
                    if t.abs() < 3.0 {
                        sinc(t) * sinc(t / 3.0)
                    } else {
                        0.0
                    }
                }
                Kernel::Area => {
                    let lo = (center - s / 2.0).max(i);
                    let hi = (center + s / 2.0).min(i + 1.0);
                    (hi - lo).max(0.0)
                }
            }
        })
        .collect();
    let sum: f64 = w.iter().sum();
    assert!(sum.abs() > 1e-9, "degenerate weights");
    for v in &mut w {
        *v /= sum;
    }
    w
}

/// Linear-light result of the oracle before any clamping.
pub struct Linear {
    pub width: u32,
    pub height: u32,
    /// Interleaved RGBA; RGB is associated (premultiplied) with alpha.
    pub rgba: Vec<f64>,
}

pub struct Request {
    pub kernel: Kernel,
    pub primaries: Primaries,
    /// `(x, y, width, height)` inside the source.
    pub crop: (u32, u32, u32, u32),
    pub output: (u32, u32),
    /// The source carries meaningful alpha.
    pub alpha: bool,
    /// A deliberately *wrong* variant: clip decoded colors to `[0, 1]`
    /// before filtering. Used only to prove that a test can detect premature
    /// gamut clipping.
    pub clip_before_filter: bool,
}

/// Resize `rgba8` (interleaved straight RGBA8, `width` wide).
pub fn resize(rgba8: &[u8], width: u32, request: &Request) -> Linear {
    let (cx, cy, cw, ch) = request.crop;
    let (ow, oh) = request.output;
    let p3 = p3_to_srgb_matrix();
    // Premultiplied linear source.
    let mut source = vec![[0.0_f64; 4]; (cw * ch) as usize];
    for y in 0..ch {
        for x in 0..cw {
            let at = (((cy + y) * width + cx + x) * 4) as usize;
            let px = &rgba8[at..at + 4];
            let a = if request.alpha {
                f64::from(px[3]) / 255.0
            } else {
                1.0
            };
            let mut rgb = decode_rgb([px[0], px[1], px[2]], request.primaries, &p3);
            if request.clip_before_filter {
                rgb = rgb.map(|v| v.clamp(0.0, 1.0));
            }
            source[(y * cw + x) as usize] = [rgb[0] * a, rgb[1] * a, rgb[2] * a, a];
        }
    }
    let wx: Vec<Vec<f64>> = (0..ow)
        .map(|o| axis_weights(request.kernel, cw, ow, o))
        .collect();
    let wy: Vec<Vec<f64>> = (0..oh)
        .map(|o| axis_weights(request.kernel, ch, oh, o))
        .collect();
    let mut rgba = Vec::with_capacity((ow * oh * 4) as usize);
    for row in &wy {
        for column in &wx {
            let mut acc = [0.0_f64; 4];
            for (j, &wj) in row.iter().enumerate() {
                if wj == 0.0 {
                    continue;
                }
                for (i, &wi) in column.iter().enumerate() {
                    if wi == 0.0 {
                        continue;
                    }
                    let w = wj * wi;
                    let p = source[j * cw as usize + i];
                    for c in 0..4 {
                        acc[c] += w * p[c];
                    }
                }
            }
            if !request.alpha {
                // Opaque input: alpha is exactly one, not a weight sum.
                acc[3] = 1.0;
            }
            rgba.extend_from_slice(&acc);
        }
    }
    Linear {
        width: ow,
        height: oh,
        rgba,
    }
}

/// The explicit final step: straight RGBA8, RGB 0 behind alpha byte 0, the
/// associated color divided by the clamped output alpha.
pub fn quantize(linear: &Linear) -> Vec<u8> {
    linear
        .rgba
        .chunks_exact(4)
        .flat_map(|px| {
            let alpha = px[3].clamp(0.0, 1.0);
            let a = alpha8(alpha);
            if a == 0 {
                [0, 0, 0, 0]
            } else {
                [
                    encode8(px[0] / alpha),
                    encode8(px[1] / alpha),
                    encode8(px[2] / alpha),
                    a,
                ]
            }
        })
        .collect()
}

/// The same output step, but dividing by the *unclamped* alpha as
/// ImageMagick does; used only to quantify that convention difference.
pub fn quantize_unclamped_division(linear: &Linear) -> Vec<u8> {
    linear
        .rgba
        .chunks_exact(4)
        .flat_map(|px| {
            let a = alpha8(px[3]);
            if a == 0 {
                [0, 0, 0, 0]
            } else {
                [
                    encode8(px[0] / px[3]),
                    encode8(px[1] / px[3]),
                    encode8(px[2] / px[3]),
                    a,
                ]
            }
        })
        .collect()
}
