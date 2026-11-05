//! ADR-0021 precision experiment.
//!
//! Standalone evidence program: no dependency on the `ferrixel` crate. It
//! measures the error of the proposed lookup tables, the f32 working type and
//! the 16-bit fixed-point path against the f64 scalar reference, over
//! exhaustive 8-bit inputs, dense 16-bit and f32 sweeps, and complete
//! `decode -> premultiply -> horizontal -> vertical -> sRGB8` chains on
//! synthetic adversarial images.
//!
//! The reference functions below mirror `src/working_color.rs` and
//! `src/quality.rs` of Ferrixel at commit 17f3f46 (2026-11-05). They are
//! copied by hand on purpose so that this program stays buildable without
//! the engine crate; keep them in sync when the reference changes.
//!
//! Run with `cargo run --release` inside this directory. See README.md.

use std::fmt::Write as _;
use std::time::Instant;

/// Fractional bits of the Q1.14 working format: 1.0 == 16384, range [-2, 2).
const Q: u32 = 14;
const ONE_Q14: i32 = 1 << Q;
/// Hard cap on coefficient precision (`--p-cap`). The per-axis interval
/// bound below usually decides the precision before this cap is reached.
static P_CAP: std::sync::atomic::AtomicU32 = std::sync::atomic::AtomicU32::new(16);

fn p_cap() -> u32 {
    P_CAP.load(std::sync::atomic::Ordering::Relaxed)
}

/// Number of buckets of the exact f32 encoder's coarse index table.
const ENC_BUCKETS: usize = 4096;
/// The sRGB8 code step at the dark end, in linear light: 1 / (255 * 12.92).
const DARK_CODE_STEP: f64 = 1.0 / (255.0 * 12.92);

// --------------------------------------------------------------------------
// f64 reference, mirrors src/quality.rs and src/working_color.rs
// --------------------------------------------------------------------------

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Filter {
    Area,
    Lanczos3,
}

impl Filter {
    fn name(self) -> &'static str {
        match self {
            Filter::Area => "Area",
            Filter::Lanczos3 => "Lanczos3",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Color {
    Srgb,
    DisplayP3,
}

#[derive(Clone, Copy, Debug)]
struct Tap {
    index: u32,
    weight: f64,
}

fn sinc(x: f64) -> f64 {
    if x.abs() < 1.0e-12 {
        1.0
    } else {
        let p = x * std::f64::consts::PI;
        p.sin() / p
    }
}

/// Verbatim port of `quality::taps` (errors turned into panics).
fn taps(input: u32, output: u32, at: u32, filter: Filter) -> Vec<Tap> {
    assert!(input > 0 && output > 0 && at < output);
    let ratio = f64::from(input) / f64::from(output);
    let center = (f64::from(at) + 0.5) * ratio - 0.5;
    let scale = ratio.max(1.0);
    let radius = match filter {
        Filter::Area => scale * 0.5,
        Filter::Lanczos3 => scale * 3.0,
    };
    let first = (center - radius - 1.0).floor().max(0.0) as u32;
    let last = (center + radius + 1.0).ceil().min(f64::from(input - 1)) as u32;
    let mut weights = Vec::new();
    let mut sum = 0.0;
    for index in first..=last {
        let distance = f64::from(index) - center;
        let weight = match filter {
            Filter::Area => ((f64::from(index) + 0.5).min(center + radius)
                - (f64::from(index) - 0.5).max(center - radius))
            .max(0.0),
            Filter::Lanczos3 => {
                let d = distance / scale;
                if d.abs() < 3.0 {
                    sinc(d) * sinc(d / 3.0)
                } else {
                    0.0
                }
            }
        };
        if weight.abs() > f64::EPSILON {
            weights.push(Tap { index, weight });
            sum += weight;
        }
    }
    assert!(!weights.is_empty() && sum.is_finite() && sum.abs() >= f64::EPSILON);
    for tap in &mut weights {
        tap.weight /= sum;
    }
    weights
}

fn axis(input: u32, output: u32, filter: Filter) -> Vec<Vec<Tap>> {
    (0..output)
        .map(|at| taps(input, output, at, filter))
        .collect()
}

/// Extended sRGB EOTF, sign preserving (working_color::decode_srgb).
fn decode_srgb(encoded: f64) -> f64 {
    let magnitude = encoded.abs();
    if magnitude <= 0.040_45 {
        encoded / 12.92
    } else {
        encoded.signum() * ((magnitude + 0.055) / 1.055).powf(2.4)
    }
}

/// working_color::encode_linear_srgb8 without the error path.
fn encode_linear_srgb8(linear: f64) -> u8 {
    let linear = linear.clamp(0.0, 1.0);
    let encoded = if linear <= 0.003_130_8 {
        12.92 * linear
    } else {
        1.055 * linear.powf(1.0 / 2.4) - 0.055
    };
    (encoded * 255.0).round() as u8
}

fn transform(matrix: [[f64; 3]; 3], rgb: [f64; 3]) -> [f64; 3] {
    matrix.map(|row| row[0].mul_add(rgb[0], row[1].mul_add(rgb[1], row[2] * rgb[2])))
}

const P3_TO_XYZ_D65: [[f64; 3]; 3] = [
    [
        608_311.0 / 1_250_200.0,
        189_793.0 / 714_400.0,
        198_249.0 / 1_000_160.0,
    ],
    [
        35_783.0 / 156_275.0,
        247_089.0 / 357_200.0,
        198_249.0 / 2_500_400.0,
    ],
    [0.0, 32_229.0 / 714_400.0, 5_220_557.0 / 5_000_800.0],
];

const XYZ_D65_TO_SRGB: [[f64; 3]; 3] = [
    [12_831.0 / 3959.0, -329.0 / 214.0, -1974.0 / 3959.0],
    [
        -851_781.0 / 878_810.0,
        1_648_619.0 / 878_810.0,
        36_519.0 / 878_810.0,
    ],
    [705.0 / 12_673.0, -2585.0 / 12_673.0, 705.0 / 667.0],
];

fn decode_rgb_f64(color: Color, encoded: [f64; 3]) -> [f64; 3] {
    let linear = encoded.map(decode_srgb);
    match color {
        Color::Srgb => linear,
        Color::DisplayP3 => transform(XYZ_D65_TO_SRGB, transform(P3_TO_XYZ_D65, linear)),
    }
}

/// quality_execute::final_rgba without the error path.
fn final_rgba(acc: [f64; 4], opaque: bool) -> [u8; 4] {
    let alpha = if opaque { 1.0 } else { acc[3].clamp(0.0, 1.0) };
    let mut rgba = [0_u8; 4];
    if alpha > 1.0e-12 {
        for channel in 0..3 {
            rgba[channel] = encode_linear_srgb8(acc[channel] / alpha);
        }
    }
    rgba[3] = (alpha * 255.0).round() as u8;
    rgba
}

/// XYZ_D65_TO_SRGB x P3_TO_XYZ_D65 as one matrix (used only to build tables).
fn folded_p3_matrix() -> [[f64; 3]; 3] {
    let mut m = [[0.0; 3]; 3];
    for (o, row) in m.iter_mut().enumerate() {
        for (i, cell) in row.iter_mut().enumerate() {
            for k in 0..3 {
                *cell += XYZ_D65_TO_SRGB[o][k] * P3_TO_XYZ_D65[k][i];
            }
        }
    }
    m
}

// --------------------------------------------------------------------------
// Proposed tables
// --------------------------------------------------------------------------

struct Tables {
    /// 256-entry sRGB8 -> linear, f32, correctly rounded from the f64 reference.
    dec_f32: [f32; 256],
    /// 256-entry sRGB8 -> linear Q1.14, rounded from the f64 reference.
    dec_q14: [i16; 256],
    /// 16385-entry Q0.14 linear -> sRGB8, exact by construction.
    enc_q14: Vec<u8>,
    /// Display P3 decode folded with the P3 -> sRGB matrix: per code, per
    /// (output, input) channel the product `m[o][i] * decode(code)` in Q.30.
    p3_q30: Vec<[[i32; 3]; 3]>,
    /// The same products in f32.
    p3_f32: Vec<[[f32; 3]; 3]>,
    /// The folded matrix in f32 (for F32 Display P3 input).
    p3_m32: [[f32; 3]; 3],
    /// Code at the lower edge of each 1/4096 bucket of [0, 1] (4097 entries).
    enc_bucket: Vec<u8>,
    /// Smallest f32 whose reference encoding is >= code (257 entries).
    enc_thresh: [f32; 257],
    /// Linear sRGB value range of decoded Display P3 8-bit input. The map is
    /// linear in the per-channel decoded values, so the extremes lie on the
    /// corners of the unit cube.
    p3_range: (f64, f64),
    /// The same range in Q1.14 integer units as decoded by the fixed-point
    /// tables, widened by one LSB for their rounding.
    p3_range_q14: (i64, i64),
}

impl Tables {
    fn build() -> Self {
        let mut dec_f32 = [0.0_f32; 256];
        let mut dec_q14 = [0_i16; 256];
        for code in 0..256 {
            let linear = decode_srgb(code as f64 / 255.0);
            dec_f32[code] = linear as f32;
            dec_q14[code] = (linear * f64::from(ONE_Q14)).round() as i16;
        }
        let enc_q14 = (0..=ONE_Q14)
            .map(|q| encode_linear_srgb8(f64::from(q) / f64::from(ONE_Q14)))
            .collect();

        let m = folded_p3_matrix();
        let mut p3_q30 = Vec::with_capacity(256);
        let mut p3_f32 = Vec::with_capacity(256);
        for code in 0..256 {
            let linear = decode_srgb(code as f64 / 255.0);
            let mut q = [[0_i32; 3]; 3];
            let mut f = [[0.0_f32; 3]; 3];
            for o in 0..3 {
                for i in 0..3 {
                    let product = m[o][i] * linear;
                    let scaled = (product * (1_u64 << 30) as f64).round();
                    assert!(scaled.abs() <= f64::from(i32::MAX));
                    q[o][i] = scaled as i32;
                    f[o][i] = product as f32;
                }
            }
            p3_q30.push(q);
            p3_f32.push(f);
        }
        let p3_m32 = m.map(|row| row.map(|v| v as f32));

        let enc_bucket: Vec<u8> = (0..=ENC_BUCKETS)
            .map(|b| encode_linear_srgb8(b as f64 / ENC_BUCKETS as f64))
            .collect();
        for b in 0..ENC_BUCKETS {
            assert!(
                enc_bucket[b + 1] - enc_bucket[b] <= 1,
                "bucket {b} spans more than one code; the exact f32 encoder needs finer buckets"
            );
        }
        let mut enc_thresh = [0.0_f32; 257];
        enc_thresh[256] = f32::INFINITY;
        for code in 1..=255_u8 {
            let start = decode_srgb((f64::from(code) - 0.5) / 255.0);
            let mut x = start as f32;
            while encode_linear_srgb8(f64::from(x)) < code {
                x = x.next_up();
            }
            while x > 0.0 && encode_linear_srgb8(f64::from(x.next_down())) >= code {
                x = x.next_down();
            }
            enc_thresh[usize::from(code)] = x;
        }

        let mut p3_range = (0.0_f64, 1.0_f64);
        for corner in 0..8_u8 {
            let rgb = [corner & 1, (corner >> 1) & 1, (corner >> 2) & 1].map(f64::from);
            for v in decode_rgb_f64(Color::DisplayP3, rgb) {
                p3_range.0 = p3_range.0.min(v);
                p3_range.1 = p3_range.1.max(v);
            }
        }
        let p3_range_q14 = (
            (p3_range.0 * f64::from(ONE_Q14)).floor() as i64 - 1,
            (p3_range.1 * f64::from(ONE_Q14)).ceil() as i64 + 1,
        );

        Self {
            dec_f32,
            dec_q14,
            enc_q14,
            p3_q30,
            p3_f32,
            p3_m32,
            enc_bucket,
            enc_thresh,
            p3_range,
            p3_range_q14,
        }
    }

    /// Exact linear -> sRGB8 for f32: one coarse lookup and one threshold compare.
    #[inline]
    fn enc_f32_exact(&self, x: f32) -> u8 {
        let x = x.clamp(0.0, 1.0);
        let bucket = (x * ENC_BUCKETS as f32) as usize;
        let mut code = self.enc_bucket[bucket];
        if code < 255 && x >= self.enc_thresh[usize::from(code) + 1] {
            code += 1;
        }
        code
    }

    /// Linear -> sRGB8 through a plain index table with `k` fractional bits
    /// (alternative design, measured for comparison only).
    fn enc_index_table(k: u32) -> Vec<u8> {
        (0..=(1_u32 << k))
            .map(|q| encode_linear_srgb8(f64::from(q) / f64::from(1_u32 << k)))
            .collect()
    }

    #[inline]
    fn decode_fixed(&self, color: Color, rgb: [u8; 3]) -> [i16; 3] {
        match color {
            Color::Srgb => rgb.map(|c| self.dec_q14[usize::from(c)]),
            Color::DisplayP3 => {
                let mut out = [0_i16; 3];
                for (o, slot) in out.iter_mut().enumerate() {
                    let mut sum = 0_i64;
                    for (i, &code) in rgb.iter().enumerate() {
                        sum += i64::from(self.p3_q30[usize::from(code)][o][i]);
                    }
                    assert!(
                        sum.abs() <= i64::from(i32::MAX),
                        "P3 Q.30 sum overflows i32"
                    );
                    let q14 = (sum + (1 << 15)) >> 16;
                    assert!(q14.abs() <= i64::from(i16::MAX));
                    *slot = q14 as i16;
                }
                out
            }
        }
    }

    #[inline]
    fn decode_f32_u8(&self, color: Color, rgb: [u8; 3]) -> [f32; 3] {
        match color {
            Color::Srgb => rgb.map(|c| self.dec_f32[usize::from(c)]),
            Color::DisplayP3 => {
                let mut out = [0.0_f32; 3];
                for (o, slot) in out.iter_mut().enumerate() {
                    let mut sum = 0.0_f32;
                    for (i, &code) in rgb.iter().enumerate() {
                        sum += self.p3_f32[usize::from(code)][o][i];
                    }
                    *slot = sum;
                }
                out
            }
        }
    }

    #[inline]
    fn decode_f32_float(&self, color: Color, rgb: [f32; 3]) -> [f32; 3] {
        let linear = rgb.map(decode_srgb_f32);
        match color {
            Color::Srgb => linear,
            Color::DisplayP3 => {
                let m = self.p3_m32;
                let mut out = [0.0_f32; 3];
                for (o, slot) in out.iter_mut().enumerate() {
                    *slot = m[o][0] * linear[0] + m[o][1] * linear[1] + m[o][2] * linear[2];
                }
                out
            }
        }
    }
}

fn decode_srgb_f32(encoded: f32) -> f32 {
    let magnitude = encoded.abs();
    if magnitude <= 0.040_45 {
        encoded / 12.92
    } else {
        encoded.signum() * ((magnitude + 0.055) / 1.055).powf(2.4)
    }
}

// --------------------------------------------------------------------------
// Weight quantization
// --------------------------------------------------------------------------

struct FixedAxis {
    precision: u32,
    rows: Vec<Vec<(u32, i16)>>,
    /// Largest |accumulator| the integer interval bound allows, as a fraction of 2^31.
    bound: f64,
}

/// Real-valued interval propagation of one separable pass, used only for
/// the worst-case illustration in the report: an input range [lo, hi] maps
/// to [lo * G - hi * N, hi * G - lo * N] where G is the largest positive
/// weight sum and N the largest negative weight sum of any output position.
fn propagate(input: (f64, f64), gain: f64, negative: f64) -> (f64, f64) {
    (
        input.0 * gain - input.1 * negative,
        input.1 * gain - input.0 * negative,
    )
}

/// Round each weight to `1 << precision` units and fix the largest tap so
/// that every row sums to exactly `1 << precision`. None if a weight does
/// not fit i16.
fn quantize_at(axis: &[Vec<Tap>], precision: u32) -> Option<Vec<Vec<(u32, i16)>>> {
    let scale = f64::from(1_u32 << precision);
    let mut rows = Vec::with_capacity(axis.len());
    for row in axis {
        let mut ints: Vec<i32> = row
            .iter()
            .map(|t| (t.weight * scale).round() as i32)
            .collect();
        let diff = (1_i32 << precision) - ints.iter().sum::<i32>();
        let largest = row
            .iter()
            .enumerate()
            .max_by(|a, b| a.1.weight.abs().partial_cmp(&b.1.weight.abs()).unwrap())
            .map(|(i, _)| i)
            .unwrap();
        ints[largest] += diff;
        let mut out = Vec::with_capacity(row.len());
        for (t, v) in row.iter().zip(ints) {
            if v.abs() > i32::from(i16::MAX) {
                return None;
            }
            out.push((t.index, v as i16));
        }
        rows.push(out);
    }
    Some(rows)
}

/// Exact interval bound of the accumulator over all output positions and
/// all inputs in the integer range `input` (Q1.14 units): the smallest and
/// largest possible accumulator value, in units of 2^-(14 + p).
fn accumulator_interval(rows: &[Vec<(u32, i16)>], input: (i64, i64)) -> (i64, i64) {
    let mut lo_acc = i64::MAX;
    let mut hi_acc = i64::MIN;
    for row in rows {
        let mut lo = 0_i64;
        let mut hi = 0_i64;
        for &(_, w) in row {
            let w = i64::from(w);
            if w >= 0 {
                lo += w * input.0;
                hi += w * input.1;
            } else {
                lo += w * input.1;
                hi += w * input.0;
            }
        }
        lo_acc = lo_acc.min(lo);
        hi_acc = hi_acc.max(hi);
    }
    (lo_acc, hi_acc)
}

/// The planner rule. Pillow-SIMD style adaptive precision, bounded exactly:
/// start from the largest `p <= cap` whose largest |weight| fits i16, and
/// lower `p` (not below 14) until the accumulator interval computed from the
/// quantized integer weights fits i32. `input` is the integer Q1.14 range of
/// the pass input; `stores_rows` says whether the pass output is stored as
/// Q1.14 i16 rows (horizontal) or clamped straight from the accumulator
/// (vertical). Returns None when the plan must use f32 instead: the row
/// range does not fit i16, or no precision >= 14 keeps the accumulator in
/// i32. The second value is the integer output range of the pass including
/// its rounding, i.e. the input range of the next pass.
fn quantize_axis(
    axis: &[Vec<Tap>],
    input: (i64, i64),
    stores_rows: bool,
) -> Option<(FixedAxis, (i64, i64))> {
    let max_weight = axis
        .iter()
        .flatten()
        .map(|t| t.weight.abs())
        .fold(0.0_f64, f64::max);
    let mut p_fit = 0;
    while p_fit < p_cap()
        && (max_weight * f64::from(1_u32 << (p_fit + 1))).round() <= f64::from(i16::MAX)
    {
        p_fit += 1;
    }
    for precision in (Q..=p_fit.max(Q)).rev() {
        let Some(rows) = quantize_at(axis, precision) else {
            continue;
        };
        let (lo_acc, hi_acc) = accumulator_interval(&rows, input);
        if lo_acc < -i64::from(i32::MAX) || hi_acc > i64::from(i32::MAX) {
            continue;
        }
        let half = 1_i64 << (precision - 1);
        let output = ((lo_acc + half) >> precision, (hi_acc + half) >> precision);
        if stores_rows && (output.0 < -i64::from(i16::MAX) || output.1 > i64::from(i16::MAX)) {
            // Rows are Q1.14 whatever p is; a lower p cannot help.
            return None;
        }
        let bound = lo_acc.unsigned_abs().max(hi_acc.unsigned_abs()) as f64 / (1_u64 << 31) as f64;
        return Some((
            FixedAxis {
                precision,
                rows,
                bound,
            },
            output,
        ));
    }
    None
}

fn f32_axis(axis: &[Vec<Tap>]) -> Vec<Vec<(u32, f32)>> {
    axis.iter()
        .map(|row| row.iter().map(|t| (t.index, t.weight as f32)).collect())
        .collect()
}

// --------------------------------------------------------------------------
// Images and chains
// --------------------------------------------------------------------------

enum Samples {
    U8(Vec<u8>),
    F32(Vec<f32>),
}

struct Image {
    width: u32,
    height: u32,
    channels: usize,
    color: Color,
    samples: Samples,
}

impl Image {
    fn kind(&self) -> &'static str {
        match (&self.samples, self.channels, self.color) {
            (Samples::U8(_), 3, Color::Srgb) => "U8 RGB sRGB",
            (Samples::U8(_), 3, Color::DisplayP3) => "U8 RGB Display P3",
            (Samples::U8(_), 4, Color::Srgb) => "U8 RGBA straight sRGB",
            (Samples::U8(_), 4, Color::DisplayP3) => "U8 RGBA straight Display P3",
            (Samples::F32(_), 3, Color::Srgb) => "F32 RGB sRGB",
            (Samples::F32(_), 3, Color::DisplayP3) => "F32 RGB Display P3",
            _ => "F32 RGBA",
        }
    }
}

/// f64 reference chain (decode once per source sample; numerically the same
/// values as the per-tap decode in quality_execute.rs).
fn run_f64(img: &Image, out_w: u32, out_h: u32, filter: Filter) -> Vec<[u8; 4]> {
    let cols = axis(img.width, out_w, filter);
    let rows = axis(img.height, out_h, filter);
    let (w, h, ch) = (img.width as usize, img.height as usize, img.channels);
    let opaque = ch == 3;
    let mut linear: Vec<[f64; 4]> = Vec::with_capacity(w * h);
    for p in 0..w * h {
        let comp = |c: usize| match &img.samples {
            Samples::U8(v) => f64::from(v[p * ch + c]) / 255.0,
            Samples::F32(v) => f64::from(v[p * ch + c]),
        };
        let rgb = decode_rgb_f64(img.color, [comp(0), comp(1), comp(2)]);
        let alpha = if opaque { 1.0 } else { comp(3) };
        linear.push([rgb[0] * alpha, rgb[1] * alpha, rgb[2] * alpha, alpha]);
    }
    let ow = out_w as usize;
    let mut horizontal = vec![[0.0_f64; 4]; ow * h];
    for y in 0..h {
        for (col, taps) in cols.iter().enumerate() {
            let mut acc = [0.0_f64; 4];
            for t in taps {
                let s = linear[y * w + t.index as usize];
                for c in 0..4 {
                    acc[c] += s[c] * t.weight;
                }
            }
            horizontal[y * ow + col] = acc;
        }
    }
    let mut out = Vec::with_capacity(ow * out_h as usize);
    for taps in &rows {
        for col in 0..ow {
            let mut acc = [0.0_f64; 4];
            for t in taps {
                let s = horizontal[t.index as usize * ow + col];
                for c in 0..4 {
                    acc[c] += s[c] * t.weight;
                }
            }
            out.push(final_rgba(acc, opaque));
        }
    }
    out
}

/// f32 chain: LUT or f32 powf decode, f32 weights and sums, exact encoder.
fn run_f32(tables: &Tables, img: &Image, out_w: u32, out_h: u32, filter: Filter) -> Vec<[u8; 4]> {
    let cols = f32_axis(&axis(img.width, out_w, filter));
    let rows = f32_axis(&axis(img.height, out_h, filter));
    let (w, h, ch) = (img.width as usize, img.height as usize, img.channels);
    let opaque = ch == 3;
    let mut linear: Vec<[f32; 4]> = Vec::with_capacity(w * h);
    for p in 0..w * h {
        let (rgb, alpha) = match &img.samples {
            Samples::U8(v) => {
                let rgb =
                    tables.decode_f32_u8(img.color, [v[p * ch], v[p * ch + 1], v[p * ch + 2]]);
                let alpha = if opaque {
                    1.0
                } else {
                    f32::from(v[p * ch + 3]) / 255.0
                };
                (rgb, alpha)
            }
            Samples::F32(v) => {
                let rgb =
                    tables.decode_f32_float(img.color, [v[p * ch], v[p * ch + 1], v[p * ch + 2]]);
                let alpha = if opaque { 1.0 } else { v[p * ch + 3] };
                (rgb, alpha)
            }
        };
        linear.push([rgb[0] * alpha, rgb[1] * alpha, rgb[2] * alpha, alpha]);
    }
    let ow = out_w as usize;
    let mut horizontal = vec![[0.0_f32; 4]; ow * h];
    for y in 0..h {
        for (col, taps) in cols.iter().enumerate() {
            let mut acc = [0.0_f32; 4];
            for &(index, weight) in taps {
                let s = linear[y * w + index as usize];
                for c in 0..4 {
                    acc[c] += s[c] * weight;
                }
            }
            horizontal[y * ow + col] = acc;
        }
    }
    let mut out = Vec::with_capacity(ow * out_h as usize);
    for taps in &rows {
        for col in 0..ow {
            let mut acc = [0.0_f32; 4];
            for &(index, weight) in taps {
                let s = horizontal[index as usize * ow + col];
                for c in 0..4 {
                    acc[c] += s[c] * weight;
                }
            }
            let alpha = if opaque { 1.0 } else { acc[3].clamp(0.0, 1.0) };
            let mut rgba = [0_u8; 4];
            if alpha > 1.0e-12 {
                for c in 0..3 {
                    rgba[c] = tables.enc_f32_exact(acc[c] / alpha);
                }
            }
            rgba[3] = (alpha * 255.0).round() as u8;
            out.push(rgba);
        }
    }
    out
}

struct FixedRun {
    out: Vec<[u8; 4]>,
    precision_h: u32,
    precision_v: u32,
    /// Largest |accumulator| observed, as a fraction of 2^31.
    peak_acc: f64,
    /// Largest |intermediate row value| observed, as a fraction of 32767.
    peak_mid: f64,
    /// Largest |accumulator| the interval bound allows, as a fraction of 2^31.
    bound_acc: f64,
    /// Largest |intermediate row value| the interval bound allows, as a fraction of 32767.
    bound_mid: f64,
}

/// 16-bit fixed-point chain: Q1.14 samples and intermediate rows, i16
/// adaptive-precision weights, i32 accumulation (modeled in i64 and checked),
/// round-half-up shifts, exact 16385-entry encoder.
fn run_fixed(tables: &Tables, img: &Image, out_w: u32, out_h: u32, filter: Filter) -> FixedRun {
    let Samples::U8(samples) = &img.samples else {
        panic!("fixed point is defined for U8 input only");
    };
    let input_range = match img.color {
        Color::Srgb => (0, i64::from(ONE_Q14)),
        Color::DisplayP3 => tables.p3_range_q14,
    };
    let (cols, mid_range) = quantize_axis(&axis(img.width, out_w, filter), input_range, true)
        .expect("the planner rule rejects this geometry for fixed point; f32 would be used");
    let (rows, _) = quantize_axis(&axis(img.height, out_h, filter), mid_range, false)
        .expect("the planner rule rejects this geometry for fixed point; f32 would be used");
    let bound_mid =
        mid_range.0.unsigned_abs().max(mid_range.1.unsigned_abs()) as f64 / f64::from(i16::MAX);
    let (w, h, ch) = (img.width as usize, img.height as usize, img.channels);
    let opaque = ch == 3;
    let mut peak_acc = 0_i64;
    let mut peak_mid = 0_i32;
    let mut linear: Vec<[i16; 4]> = Vec::with_capacity(w * h);
    for p in 0..w * h {
        let rgb = tables.decode_fixed(
            img.color,
            [samples[p * ch], samples[p * ch + 1], samples[p * ch + 2]],
        );
        if opaque {
            linear.push([rgb[0], rgb[1], rgb[2], ONE_Q14 as i16]);
        } else {
            let a = i32::from(samples[p * ch + 3]);
            let pre = rgb.map(|v| ((i32::from(v) * a + 127).div_euclid(255)) as i16);
            let alpha = ((a * ONE_Q14 + 127) / 255) as i16;
            linear.push([pre[0], pre[1], pre[2], alpha]);
        }
    }
    let ow = out_w as usize;
    let half_h = 1_i64 << (cols.precision - 1);
    let mut horizontal = vec![[0_i16; 4]; ow * h];
    for y in 0..h {
        for (col, taps) in cols.rows.iter().enumerate() {
            let mut acc = [0_i64; 4];
            for &(index, weight) in taps {
                let s = linear[y * w + index as usize];
                for c in 0..4 {
                    acc[c] += i64::from(s[c]) * i64::from(weight);
                }
            }
            let mut row = [0_i16; 4];
            for c in 0..4 {
                peak_acc = peak_acc.max(acc[c].abs());
                assert!(
                    acc[c].abs() <= i64::from(i32::MAX),
                    "horizontal i32 accumulator overflow"
                );
                let r = (acc[c] + half_h) >> cols.precision;
                assert!(
                    r.abs() <= i64::from(i16::MAX),
                    "intermediate row overflows i16"
                );
                peak_mid = peak_mid.max(r.unsigned_abs() as i32);
                row[c] = r as i16;
            }
            horizontal[y * ow + col] = row;
        }
    }
    let half_v = 1_i64 << (rows.precision - 1);
    let mut out = Vec::with_capacity(ow * out_h as usize);
    for taps in &rows.rows {
        for col in 0..ow {
            let mut acc = [0_i64; 4];
            for &(index, weight) in taps {
                let s = horizontal[index as usize * ow + col];
                for c in 0..4 {
                    acc[c] += i64::from(s[c]) * i64::from(weight);
                }
            }
            let mut q = [0_i64; 4];
            for c in 0..4 {
                peak_acc = peak_acc.max(acc[c].abs());
                assert!(
                    acc[c].abs() <= i64::from(i32::MAX),
                    "vertical i32 accumulator overflow"
                );
                q[c] = (acc[c] + half_v) >> rows.precision;
            }
            let mut rgba = [0_u8; 4];
            if opaque {
                for c in 0..3 {
                    rgba[c] = tables.enc_q14[q[c].clamp(0, i64::from(ONE_Q14)) as usize];
                }
                rgba[3] = 255;
            } else {
                let alpha = q[3].clamp(0, i64::from(ONE_Q14));
                if alpha > 0 {
                    for c in 0..3 {
                        let ratio = ((q[c] << Q) + alpha / 2).div_euclid(alpha);
                        rgba[c] = tables.enc_q14[ratio.clamp(0, i64::from(ONE_Q14)) as usize];
                    }
                }
                rgba[3] = ((alpha * 255 + (1 << (Q - 1))) >> Q) as u8;
            }
            out.push(rgba);
        }
    }
    FixedRun {
        out,
        precision_h: cols.precision,
        precision_v: rows.precision,
        peak_acc: peak_acc as f64 / (1_u64 << 31) as f64,
        peak_mid: f64::from(peak_mid) / f64::from(i16::MAX),
        bound_acc: cols.bound.max(rows.bound),
        bound_mid,
    }
}

// --------------------------------------------------------------------------
// Statistics
// --------------------------------------------------------------------------

#[derive(Default, Clone, Copy)]
struct Stats {
    n: u64,
    exact: u64,
    off1: u64,
    off2: u64,
    max: u32,
    sum: u64,
}

impl Stats {
    fn add(&mut self, d: u32) {
        self.n += 1;
        match d {
            0 => self.exact += 1,
            1 => self.off1 += 1,
            _ => self.off2 += 1,
        }
        self.max = self.max.max(d);
        self.sum += u64::from(d);
    }
    fn merge(&mut self, o: &Stats) {
        self.n += o.n;
        self.exact += o.exact;
        self.off1 += o.off1;
        self.off2 += o.off2;
        self.max = self.max.max(o.max);
        self.sum += o.sum;
    }
    fn exact_pct(&self) -> f64 {
        if self.n == 0 {
            100.0
        } else {
            100.0 * self.exact as f64 / self.n as f64
        }
    }
    fn mean(&self) -> f64 {
        if self.n == 0 {
            0.0
        } else {
            self.sum as f64 / self.n as f64
        }
    }
    fn cell(&self) -> String {
        if self.n == 0 {
            return "-".to_string();
        }
        format!(
            "max {} / {:.4}% exact / {} off-1 / {} off-2+",
            self.max,
            self.exact_pct(),
            self.off1,
            self.off2
        )
    }
}

struct Comparison {
    /// RGB where the reference output alpha is > 0 (all samples when opaque).
    rgb: Stats,
    alpha: Stats,
    /// RGB stats by reference output alpha bucket: 1-25, 26-127, 128-254, 255.
    buckets: [Stats; 4],
    /// RGB samples that differ where the reference alpha is 0 (invisible).
    invisible: u64,
}

fn compare(reference: &[[u8; 4]], test: &[[u8; 4]], opaque: bool) -> Comparison {
    assert_eq!(reference.len(), test.len());
    let mut c = Comparison {
        rgb: Stats::default(),
        alpha: Stats::default(),
        buckets: [Stats::default(); 4],
        invisible: 0,
    };
    for (r, t) in reference.iter().zip(test) {
        let visible = opaque || r[3] > 0;
        for ch in 0..3 {
            let d = u32::from(r[ch].abs_diff(t[ch]));
            if visible {
                c.rgb.add(d);
                if !opaque {
                    let b = match r[3] {
                        1..=25 => 0,
                        26..=127 => 1,
                        128..=254 => 2,
                        _ => 3,
                    };
                    c.buckets[b].add(d);
                }
            } else if d != 0 {
                c.invisible += 1;
            }
        }
        if !opaque {
            c.alpha.add(u32::from(r[3].abs_diff(t[3])));
        }
    }
    c
}

// --------------------------------------------------------------------------
// Synthetic images
// --------------------------------------------------------------------------

struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }
    fn u8(&mut self) -> u8 {
        (self.next() >> 56) as u8
    }
    fn range_u8(&mut self, lo: u8, hi: u8) -> u8 {
        lo + (self.next() % u64::from(hi - lo + 1)) as u8
    }
    fn below(&mut self, n: usize) -> usize {
        (self.next() % n as u64) as usize
    }
    fn unit(&mut self) -> f64 {
        (self.next() >> 11) as f64 / (1_u64 << 53) as f64
    }
}

const SATURATED: [[u8; 3]; 8] = [
    [0, 0, 0],
    [255, 255, 255],
    [255, 0, 0],
    [0, 255, 0],
    [0, 0, 255],
    [255, 255, 0],
    [0, 255, 255],
    [255, 0, 255],
];

fn u8_image(
    w: u32,
    h: u32,
    ch: usize,
    color: Color,
    mut f: impl FnMut(u32, u32) -> [u8; 4],
) -> Image {
    let mut data = Vec::with_capacity(w as usize * h as usize * ch);
    for y in 0..h {
        for x in 0..w {
            let px = f(x, y);
            data.extend_from_slice(&px[..ch]);
        }
    }
    Image {
        width: w,
        height: h,
        channels: ch,
        color,
        samples: Samples::U8(data),
    }
}

fn f32_image(w: u32, h: u32, color: Color, mut f: impl FnMut(u32, u32) -> [f32; 3]) -> Image {
    let mut data = Vec::with_capacity(w as usize * h as usize * 3);
    for y in 0..h {
        for x in 0..w {
            data.extend_from_slice(&f(x, y));
        }
    }
    Image {
        width: w,
        height: h,
        channels: 3,
        color,
        samples: Samples::F32(data),
    }
}

fn rectangles(
    rng: &mut Rng,
    w: u32,
    h: u32,
    count: usize,
    palette: &[[u8; 3]],
) -> Vec<(u32, u32, u32, u32, [u8; 3])> {
    (0..count)
        .map(|_| {
            let rw = 4 + rng.below(w as usize / 3) as u32;
            let rh = 4 + rng.below(h as usize / 3) as u32;
            let x = rng.below((w - rw) as usize) as u32;
            let y = rng.below((h - rh) as usize) as u32;
            (x, y, rw, rh, palette[rng.below(palette.len())])
        })
        .collect()
}

fn cases(w: u32, h: u32) -> Vec<(&'static str, Image)> {
    let mut rng = Rng(0x9E37_79B9_7F4A_7C15);
    let mut out = Vec::new();

    out.push((
        "noise",
        u8_image(w, h, 3, Color::Srgb, |_, _| {
            [rng.u8(), rng.u8(), rng.u8(), 255]
        }),
    ));
    out.push((
        "dark-noise (codes 0-24)",
        u8_image(w, h, 3, Color::Srgb, |_, _| {
            [
                rng.range_u8(0, 24),
                rng.range_u8(0, 24),
                rng.range_u8(0, 24),
                255,
            ]
        }),
    ));
    out.push((
        "bright-noise (codes 231-255)",
        u8_image(w, h, 3, Color::Srgb, |_, _| {
            [
                rng.range_u8(231, 255),
                rng.range_u8(231, 255),
                rng.range_u8(231, 255),
                255,
            ]
        }),
    ));
    out.push((
        "gradient",
        u8_image(w, h, 3, Color::Srgb, |x, y| {
            let r = (x * 255 / (w - 1).max(1)) as u8;
            let g = (y * 255 / (h - 1).max(1)) as u8;
            let b = ((x + y) * 255 / (w + h - 2).max(1)) as u8;
            [r, g, b, 255]
        }),
    ));
    {
        let mut palette = Vec::new();
        for _ in 0..64 {
            palette.push([rng.u8(), rng.u8(), rng.u8()]);
        }
        out.push((
            "flat-blocks (32px constant blocks)",
            u8_image(w, h, 3, Color::Srgb, |x, y| {
                let p = palette[((x / 32) * 7 + (y / 32) * 13) as usize % palette.len()];
                [p[0], p[1], p[2], 255]
            }),
        ));
    }
    out.push((
        "checker-1px",
        u8_image(w, h, 3, Color::Srgb, |x, y| {
            let v = if (x + y) % 2 == 0 { 255 } else { 0 };
            [v, v, v, 255]
        }),
    ));
    out.push((
        "checker-2px",
        u8_image(w, h, 3, Color::Srgb, |x, y| {
            let v = if ((x / 2) + (y / 2)) % 2 == 0 { 255 } else { 0 };
            [v, v, v, 255]
        }),
    ));
    {
        let rects = rectangles(&mut rng, w, h, 48, &SATURATED);
        out.push((
            "saturated-edges",
            u8_image(w, h, 3, Color::Srgb, |x, y| {
                let mut px = if (x / 64 + y / 64) % 2 == 0 {
                    [0, 0, 0]
                } else {
                    [255, 255, 255]
                };
                for &(rx, ry, rw, rh, c) in &rects {
                    if x >= rx && x < rx + rw && y >= ry && y < ry + rh {
                        px = c;
                    }
                }
                [px[0], px[1], px[2], 255]
            }),
        ));
    }
    {
        let rects = rectangles(&mut rng, w, h, 48, &SATURATED);
        out.push((
            "p3-primaries (out-of-gamut edges)",
            u8_image(w, h, 3, Color::DisplayP3, |x, y| {
                let mut px = if (x / 48 + y / 48) % 2 == 0 {
                    [0, 0, 0]
                } else {
                    [255, 255, 255]
                };
                for &(rx, ry, rw, rh, c) in &rects {
                    if x >= rx && x < rx + rw && y >= ry && y < ry + rh {
                        px = c;
                    }
                }
                [px[0], px[1], px[2], 255]
            }),
        ));
    }
    out.push((
        "p3-checker-1px (red/green)",
        u8_image(w, h, 3, Color::DisplayP3, |x, y| {
            if (x + y) % 2 == 0 {
                [255, 0, 0, 255]
            } else {
                [0, 255, 0, 255]
            }
        }),
    ));
    out.push((
        "p3-noise",
        u8_image(w, h, 3, Color::DisplayP3, |_, _| {
            [rng.u8(), rng.u8(), rng.u8(), 255]
        }),
    ));
    {
        let rects = rectangles(&mut rng, w, h, 40, &SATURATED);
        let mut dark = Vec::new();
        for _ in 0..16 {
            dark.push([
                rng.range_u8(0, 40),
                rng.range_u8(0, 40),
                rng.range_u8(0, 40),
            ]);
        }
        let dark_rects = rectangles(&mut rng, w, h, 24, &dark);
        out.push((
            "alpha-edges (opaque shapes on transparent)",
            u8_image(w, h, 4, Color::Srgb, |x, y| {
                // Transparent background carries garbage color on purpose.
                let mut px = if (x / 96) % 2 == 0 {
                    [255, 0, 255, 0]
                } else {
                    [0, 0, 0, 0]
                };
                for &(rx, ry, rw, rh, c) in rects.iter().chain(&dark_rects) {
                    if x >= rx && x < rx + rw && y >= ry && y < ry + rh {
                        px = [c[0], c[1], c[2], 255];
                    }
                }
                px
            }),
        ));
    }
    out.push((
        "alpha-noise",
        u8_image(w, h, 4, Color::Srgb, |_, _| {
            [rng.u8(), rng.u8(), rng.u8(), rng.u8()]
        }),
    ));
    out.push((
        "f32-noise",
        f32_image(w, h, Color::Srgb, |_, _| {
            [rng.unit() as f32, rng.unit() as f32, rng.unit() as f32]
        }),
    ));
    out.push((
        "f32-dark (0-0.01)",
        f32_image(w, h, Color::Srgb, |_, _| {
            [
                (rng.unit() * 0.01) as f32,
                (rng.unit() * 0.01) as f32,
                (rng.unit() * 0.01) as f32,
            ]
        }),
    ));
    out.push((
        "f32-extended (-0.2 to 1.5)",
        f32_image(w, h, Color::Srgb, |_, _| {
            [
                (rng.unit() * 1.7 - 0.2) as f32,
                (rng.unit() * 1.7 - 0.2) as f32,
                (rng.unit() * 1.7 - 0.2) as f32,
            ]
        }),
    ));
    out.push((
        "f32-p3-noise",
        f32_image(w, h, Color::DisplayP3, |_, _| {
            [rng.unit() as f32, rng.unit() as f32, rng.unit() as f32]
        }),
    ));
    out
}

fn geometries(w: u32, h: u32) -> Vec<(String, u32, u32)> {
    let g = |label: &str, ow: u32, oh: u32| (label.to_string(), ow.max(1), oh.max(1));
    vec![
        g("identity", w, h),
        g("up 1.5x", w * 3 / 2, h * 3 / 2),
        g("down 1.5x", w * 2 / 3, h * 2 / 3),
        g("down 2x", w / 2, h / 2),
        g(
            "down 2.37x",
            (f64::from(w) / 2.37) as u32,
            (f64::from(h) / 2.37) as u32,
        ),
        g("down 8x", w / 8, h / 8),
        g("down 16x", w / 16, h / 16),
        g("down 32x", w / 32, h / 32),
        g("down 64x", w / 64, h / 64),
        g("down 128x", w / 128, h / 128),
        g("down to 2x2", 2, 2),
        g("aniso 1x/16x", w, h / 16),
    ]
}

// --------------------------------------------------------------------------
// Table experiments
// --------------------------------------------------------------------------

fn experiment_decode_tables(t: &Tables, report: &mut String) {
    let mut max_f32_rel = 0.0_f64;
    let mut max_q14_abs = 0.0_f64;
    let mut max_q14_rel = 0.0_f64;
    let mut round_trip_failures = 0;
    for code in 0..256 {
        let reference = decode_srgb(code as f64 / 255.0);
        if reference > 0.0 {
            max_f32_rel =
                max_f32_rel.max(((f64::from(t.dec_f32[code]) - reference) / reference).abs());
            let q = f64::from(t.dec_q14[code]) / f64::from(ONE_Q14);
            max_q14_abs = max_q14_abs.max((q - reference).abs());
            max_q14_rel = max_q14_rel.max(((q - reference) / reference).abs());
        }
        let back = t.enc_q14[t.dec_q14[code] as usize];
        if usize::from(back) != code {
            round_trip_failures += 1;
        }
    }
    let _ = writeln!(report, "## E1 Decode tables (exhaustive, 256 codes)\n");
    let _ = writeln!(report, "| Table | Result |\n| --- | --- |");
    let _ = writeln!(
        report,
        "| f32 256-entry sRGB8 -> linear | max relative error vs f64 {max_f32_rel:.3e} (f32 rounding only) |"
    );
    let _ = writeln!(
        report,
        "| Q1.14 256-entry sRGB8 -> linear | max abs error {max_q14_abs:.3e} linear = {:.4} dark-end code steps; max relative error {max_q14_rel:.3e} |",
        max_q14_abs / DARK_CODE_STEP
    );
    let _ = writeln!(
        report,
        "| Q1.14 decode then exact Q0.14 encode | round-trip failures: {round_trip_failures} of 256 |"
    );
    let _ = writeln!(
        report,
        "| Distinct Q1.14 values for 256 codes | {} (codes 0 and 1 map to {} and {}) |\n",
        {
            let mut v: Vec<i16> = t.dec_q14.to_vec();
            v.sort_unstable();
            v.dedup();
            v.len()
        },
        t.dec_q14[0],
        t.dec_q14[1]
    );
}

fn experiment_p3(t: &Tables, report: &mut String) -> (f64, f64) {
    let start = Instant::now();
    let mut lin = [0.0_f64; 256];
    for (code, slot) in lin.iter_mut().enumerate() {
        *slot = decode_srgb(code as f64 / 255.0);
    }
    let mut max_linear = [f64::MIN; 3];
    let mut min_linear = [f64::MAX; 3];
    let mut fixed_off1 = 0_u64;
    let mut fixed_off2 = 0_u64;
    let mut max_fixed_lsb = 0.0_f64;
    let mut max_f32_abs = 0.0_f64;
    for r in 0..=255_u8 {
        for g in 0..=255_u8 {
            for b in 0..=255_u8 {
                let reference = transform(
                    XYZ_D65_TO_SRGB,
                    transform(
                        P3_TO_XYZ_D65,
                        [
                            lin[usize::from(r)],
                            lin[usize::from(g)],
                            lin[usize::from(b)],
                        ],
                    ),
                );
                let fixed = t.decode_fixed(Color::DisplayP3, [r, g, b]);
                let f32v = t.decode_f32_u8(Color::DisplayP3, [r, g, b]);
                for c in 0..3 {
                    max_linear[c] = max_linear[c].max(reference[c]);
                    min_linear[c] = min_linear[c].min(reference[c]);
                    let exact_lsb = reference[c] * f64::from(ONE_Q14);
                    let d = (f64::from(fixed[c]) - exact_lsb).abs();
                    max_fixed_lsb = max_fixed_lsb.max(d);
                    let d_round = (i32::from(fixed[c]) - exact_lsb.round() as i32).abs();
                    match d_round {
                        0 => {}
                        1 => fixed_off1 += 1,
                        _ => fixed_off2 += 1,
                    }
                    max_f32_abs = max_f32_abs.max((f64::from(f32v[c]) - reference[c]).abs());
                }
            }
        }
    }
    let _ = writeln!(
        report,
        "## E2 Display P3 8-bit -> linear sRGB (exhaustive, 2^24 triples, {:.2} s)\n",
        start.elapsed().as_secs_f64()
    );
    let _ = writeln!(
        report,
        "| Quantity | R | G | B |\n| --- | --- | --- | --- |"
    );
    let _ = writeln!(
        report,
        "| max linear sRGB value | {:.4} | {:.4} | {:.4} |",
        max_linear[0], max_linear[1], max_linear[2]
    );
    let _ = writeln!(
        report,
        "| min linear sRGB value | {:.4} | {:.4} | {:.4} |\n",
        min_linear[0], min_linear[1], min_linear[2]
    );
    let _ = writeln!(report, "| Path | Result |\n| --- | --- |");
    let _ = writeln!(
        report,
        "| Q.30 folded product tables, summed, rounded once to Q1.14 | max distance to the unrounded f64 value {max_fixed_lsb:.4} LSB; entries not equal to round(f64 * 2^14): {fixed_off1} off by 1, {fixed_off2} off by 2+ (of {}) |",
        3 * (1_u64 << 24)
    );
    let _ = writeln!(
        report,
        "| f32 folded product tables, summed in f32 | max abs error vs f64 {max_f32_abs:.3e} linear |\n"
    );
    (
        max_linear.iter().copied().fold(f64::MIN, f64::max),
        min_linear.iter().copied().fold(f64::MAX, f64::min),
    )
}

fn experiment_encode_sweeps(t: &Tables, report: &mut String) {
    let start = Instant::now();
    const N: u32 = 1 << 24;
    let sweep: Vec<f64> = (0..N).map(|i| f64::from(i) / f64::from(N - 1)).collect();
    let dark: Vec<f64> = (0..(1_u32 << 20))
        .map(|i| f64::from(i) / f64::from((1_u32 << 20) - 1) * 0.01)
        .collect();
    let reference_sweep: Vec<u8> = sweep.iter().map(|&x| encode_linear_srgb8(x)).collect();
    let reference_dark: Vec<u8> = dark.iter().map(|&x| encode_linear_srgb8(x)).collect();
    let _ = writeln!(
        report,
        "## E3 Linear -> sRGB8 encoder shapes (dense sweeps)\n"
    );
    let _ = writeln!(
        report,
        "Full sweep: 2^24 points uniformly spaced in [0, 1]. Dark sweep: 2^20 points in [0, 0.01] (codes 0 to about 25).\n"
    );
    let _ = writeln!(
        report,
        "| Encoder | Full sweep mismatches | Full max | Dark sweep mismatches | Dark max |\n| --- | --- | --- | --- | --- |"
    );
    for k in [12_u32, 14, 16] {
        let table = Tables::enc_index_table(k);
        let scale = f64::from(1_u32 << k);
        let mut full = Stats::default();
        for (x, &r) in sweep.iter().zip(&reference_sweep) {
            let v = table[(x * scale).round() as usize];
            full.add(u32::from(v.abs_diff(r)));
        }
        let mut dk = Stats::default();
        for (x, &r) in dark.iter().zip(&reference_dark) {
            let v = table[(x * scale).round() as usize];
            dk.add(u32::from(v.abs_diff(r)));
        }
        let _ = writeln!(
            report,
            "| nearest index, 2^{k}+1 entries ({} KiB), no interpolation | {} ({:.4}%) | {} | {} ({:.4}%) | {} |",
            ((1_u32 << k) + 1).div_ceil(1024),
            full.n - full.exact,
            100.0 - full.exact_pct(),
            full.max,
            dk.n - dk.exact,
            100.0 - dk.exact_pct(),
            dk.max
        );
    }
    {
        let mut full = Stats::default();
        for &x in &sweep {
            let xf = x as f32;
            full.add(u32::from(
                t.enc_f32_exact(xf)
                    .abs_diff(encode_linear_srgb8(f64::from(xf))),
            ));
        }
        let mut dk = Stats::default();
        for &x in &dark {
            let xf = x as f32;
            dk.add(u32::from(
                t.enc_f32_exact(xf)
                    .abs_diff(encode_linear_srgb8(f64::from(xf))),
            ));
        }
        let _ = writeln!(
            report,
            "| exact f32: 4097-entry bucket table + 257 f32 thresholds (5 KiB) | {} | {} | {} | {} |",
            full.n - full.exact,
            full.max,
            dk.n - dk.exact,
            dk.max
        );
    }
    let _ = writeln!(
        report,
        "\nSweeps took {:.2} s.\n",
        start.elapsed().as_secs_f64()
    );
}

fn experiment_exhaustive_f32_encoder(t: &Tables, report: &mut String) {
    let start = Instant::now();
    let last = 1.0_f32.to_bits();
    let threads = std::thread::available_parallelism().map_or(1, |n| n.get());
    let chunk = (last / threads as u32) + 1;
    let mismatches: u64 = std::thread::scope(|scope| {
        let handles: Vec<_> = (0..threads as u32)
            .map(|i| {
                let lo = i * chunk;
                let hi = ((i + 1) * chunk).min(last + 1);
                scope.spawn(move || {
                    let mut bad = 0_u64;
                    for bits in lo..hi {
                        let x = f32::from_bits(bits);
                        if t.enc_f32_exact(x) != encode_linear_srgb8(f64::from(x)) {
                            bad += 1;
                        }
                    }
                    bad
                })
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).sum()
    });
    let _ = writeln!(
        report,
        "## E4 Exact f32 encoder, exhaustive over every f32 in [0, 1]\n"
    );
    let _ = writeln!(
        report,
        "{} values (bit patterns 0x00000000..=0x3F800000), {} threads, {:.1} s: **{mismatches} mismatches** against `encode_linear_srgb8(x as f64)`.\n",
        u64::from(last) + 1,
        threads,
        start.elapsed().as_secs_f64()
    );
}

fn experiment_f32_decode(report: &mut String) {
    let mut max_rel = 0.0_f64;
    let mut max_abs = 0.0_f64;
    const N: u32 = 1 << 24;
    for i in 0..N {
        let x = (f64::from(i) / f64::from(N - 1) * 1.7 - 0.2) as f32;
        let reference = decode_srgb(f64::from(x));
        let fast = f64::from(decode_srgb_f32(x));
        let d = (fast - reference).abs();
        max_abs = max_abs.max(d);
        if reference.abs() > 1e-9 {
            max_rel = max_rel.max(d / reference.abs());
        }
    }
    let _ = writeln!(
        report,
        "## E5 f32 `powf` decode for F32 input (2^24 points in [-0.2, 1.5])\n"
    );
    let _ = writeln!(
        report,
        "max abs error {max_abs:.3e} linear ({:.5} dark-end code steps), max relative error {max_rel:.3e}.\n",
        max_abs / DARK_CODE_STEP
    );
}

fn experiment_premultiply(t: &Tables, report: &mut String) {
    let mut max_div = 0_i32;
    let mut max_trick = 0_i32;
    let mut max_alpha = 0_i32;
    for (code, &q14) in t.dec_q14.iter().enumerate() {
        let linear = decode_srgb(code as f64 / 255.0);
        for a in 0..256_i32 {
            let exact = (linear * f64::from(a) / 255.0 * f64::from(ONE_Q14)).round() as i32;
            let q = i32::from(q14);
            let div = (q * a + 127).div_euclid(255);
            let trick = (q * (a * 257) + (1 << 15)) >> 16;
            max_div = max_div.max((div - exact).abs());
            max_trick = max_trick.max((trick - exact).abs());
            let alpha_exact = (f64::from(a) / 255.0 * f64::from(ONE_Q14)).round() as i32;
            let alpha_fixed = (a * ONE_Q14 + 127) / 255;
            max_alpha = max_alpha.max((alpha_fixed - alpha_exact).abs());
        }
    }
    let _ = writeln!(
        report,
        "## E6 Fixed-point premultiply (exhaustive, 256 codes x 256 alphas)\n"
    );
    let _ = writeln!(
        report,
        "| Operation | max |error| vs round(f64 * 2^14) |\n| --- | --- |"
    );
    let _ = writeln!(report, "| `(q14 * a + 127) div 255` | {max_div} LSB |");
    let _ = writeln!(
        report,
        "| `(q14 * (a * 257) + 2^15) >> 16` (shift instead of divide) | {max_trick} LSB |"
    );
    let _ = writeln!(
        report,
        "| alpha `(a * 2^14 + 127) / 255` | {max_alpha} LSB |\n"
    );
}

fn experiment_weight_gain(report: &mut String) -> (f64, f64, f64) {
    let _ = writeln!(report, "## E7 Filter gain and coefficient precision\n");
    let _ = writeln!(
        report,
        "Gain G = largest sum of positive normalized weights of any output position and N the largest sum of negative weights (together they bound the overshoot of one pass); max |w| and the accumulator bound decide the adaptive coefficient precision p (cap {}).\n",
        p_cap()
    );
    let mut worst = [(0.0_f64, 0.0_f64, 0.0_f64); 2];
    let _ = writeln!(
        report,
        "| Filter | Sizes | G (max positive sum) | N (max negative sum) | max |w| | where |\n| --- | --- | --- | --- | --- | --- |"
    );
    for (fi, filter) in [Filter::Lanczos3, Filter::Area].into_iter().enumerate() {
        let mut g = 0.0_f64;
        let mut neg = 0.0_f64;
        let mut max_w = 0.0_f64;
        let mut at = (0, 0, 0);
        for input in 1..=96_u32 {
            for output in 1..=96_u32 {
                for (position, row) in axis(input, output, filter).iter().enumerate() {
                    let pos: f64 = row.iter().map(|t| t.weight.max(0.0)).sum();
                    let n: f64 = row.iter().map(|t| (-t.weight).max(0.0)).sum();
                    if pos > g {
                        g = pos;
                        at = (input, output, position);
                    }
                    neg = neg.max(n);
                    max_w = max_w.max(row.iter().map(|t| t.weight.abs()).fold(0.0, f64::max));
                }
            }
        }
        let _ = writeln!(
            report,
            "| {} | all input x output sizes in 1..=96 | {g:.5} | {neg:.5} | {max_w:.5} | G at input {}, output {}, position {} |",
            filter.name(),
            at.0,
            at.1,
            at.2
        );
        worst[fi] = (g, neg, max_w);
    }
    let _ = writeln!(
        report,
        "\n| Filter | Ratio (8192 -> n) | taps per output | G | max |w| | adaptive p |\n| --- | --- | --- | --- | --- | --- |"
    );
    for filter in [Filter::Lanczos3, Filter::Area] {
        for output in [4096_u32, 2048, 1024, 512, 256, 128, 64, 32, 16, 8, 4, 2, 1] {
            let rows = axis(8192, output, filter);
            let g = rows
                .iter()
                .map(|r| r.iter().map(|t| t.weight.max(0.0)).sum::<f64>())
                .fold(0.0, f64::max);
            let max_w = rows
                .iter()
                .flatten()
                .map(|t| t.weight.abs())
                .fold(0.0, f64::max);
            let taps_max = rows.iter().map(Vec::len).max().unwrap();
            let p = quantize_axis(&rows, (0, i64::from(ONE_Q14)), true)
                .expect("sRGB fits")
                .0
                .precision;
            let _ = writeln!(
                report,
                "| {} | {}x | {taps_max} | {g:.5} | {max_w:.5} | {p} |",
                filter.name(),
                8192 / output
            );
            let (wg, wn, ww) = &mut worst[usize::from(filter == Filter::Area)];
            *wg = wg.max(g);
            *wn = wn.max(
                rows.iter()
                    .map(|r| r.iter().map(|t| (-t.weight).max(0.0)).sum::<f64>())
                    .fold(0.0, f64::max),
            );
            *ww = ww.max(max_w);
        }
    }
    let _ = writeln!(report);
    (worst[0].0, worst[0].1, worst[1].0)
}

// --------------------------------------------------------------------------
// Chain experiments
// --------------------------------------------------------------------------

struct RunResult {
    case: &'static str,
    kind: &'static str,
    opaque: bool,
    is_u8: bool,
    filter: Filter,
    geometry: String,
    f32: Comparison,
    fixed: Option<(FixedRun, Comparison)>,
    seconds: f64,
}

fn run_case(
    tables: &Tables,
    case: &'static str,
    img: &Image,
    geoms: &[(String, u32, u32)],
) -> Vec<RunResult> {
    let mut results = Vec::new();
    let opaque = img.channels == 3;
    let is_u8 = matches!(img.samples, Samples::U8(_));
    for filter in [Filter::Lanczos3, Filter::Area] {
        for (label, ow, oh) in geoms {
            let start = Instant::now();
            let reference = run_f64(img, *ow, *oh, filter);
            let f32_out = run_f32(tables, img, *ow, *oh, filter);
            let f32 = compare(&reference, &f32_out, opaque);
            let fixed = if is_u8 {
                let run = run_fixed(tables, img, *ow, *oh, filter);
                let cmp = compare(&reference, &run.out, opaque);
                Some((run, cmp))
            } else {
                None
            };
            results.push(RunResult {
                case,
                kind: img.kind(),
                opaque,
                is_u8,
                filter,
                geometry: label.clone(),
                f32,
                fixed,
                seconds: start.elapsed().as_secs_f64(),
            });
        }
    }
    results
}

fn write_chain_report(results: &[Vec<RunResult>], report: &mut String) {
    let _ = writeln!(report, "## E8 Full chains against the f64 reference\n");
    let _ = writeln!(
        report,
        "Every cell compares sRGB8 output samples per channel with the f64 reference: max |delta| in codes, share of exactly equal samples, count off by 1, count off by 2 or more. For RGBA input, RGB is compared where the reference output alpha is > 0; alpha is compared separately; RGB differences under reference alpha 0 are counted as invisible.\n"
    );
    for case_results in results {
        let first = &case_results[0];
        let _ = writeln!(report, "### {} ({})\n", first.case, first.kind);
        if first.is_u8 {
            let _ = writeln!(
                report,
                "| Filter | Geometry | f32 RGB | fixed p (h, v) | fixed RGB | acc bound / 2^31 | peak acc / 2^31 | row bound / 32767 | peak row / 32767 | s |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |"
            );
        } else {
            let _ = writeln!(
                report,
                "| Filter | Geometry | f32 RGB | s |\n| --- | --- | --- | --- |"
            );
        }
        for r in case_results {
            if let Some((run, cmp)) = &r.fixed {
                let _ = writeln!(
                    report,
                    "| {} | {} | {} | {}, {} | {} | {:.3} | {:.3} | {:.3} | {:.3} | {:.2} |",
                    r.filter.name(),
                    r.geometry,
                    r.f32.rgb.cell(),
                    run.precision_h,
                    run.precision_v,
                    cmp.rgb.cell(),
                    run.bound_acc,
                    run.peak_acc,
                    run.bound_mid,
                    run.peak_mid,
                    r.seconds
                );
            } else {
                let _ = writeln!(
                    report,
                    "| {} | {} | {} | {:.2} |",
                    r.filter.name(),
                    r.geometry,
                    r.f32.rgb.cell(),
                    r.seconds
                );
            }
        }
        if !first.opaque {
            let _ = writeln!(
                report,
                "\nAlpha channel and RGB by reference output alpha (buckets 1-25, 26-127, 128-254, 255):\n"
            );
            let _ = writeln!(
                report,
                "| Filter | Geometry | Path | alpha | RGB a=1-25 | RGB a=26-127 | RGB a=128-254 | RGB a=255 | invisible RGB diffs |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- |"
            );
            for r in case_results {
                let row = |path: &str, c: &Comparison| {
                    format!(
                        "| {} | {} | {} | {} | {} | {} | {} | {} | {} |",
                        r.filter.name(),
                        r.geometry,
                        path,
                        c.alpha.cell(),
                        c.buckets[0].cell(),
                        c.buckets[1].cell(),
                        c.buckets[2].cell(),
                        c.buckets[3].cell(),
                        c.invisible
                    )
                };
                let _ = writeln!(report, "{}", row("f32", &r.f32));
                if let Some((_, cmp)) = &r.fixed {
                    let _ = writeln!(report, "{}", row("fixed", cmp));
                }
            }
        }
        let _ = writeln!(report);
    }
}

fn write_summary(results: &[Vec<RunResult>], report: &mut String) {
    let _ = writeln!(
        report,
        "## E9 Summary: worst case per geometry over all opaque U8 cases (sRGB and Display P3)\n"
    );
    let _ = writeln!(
        report,
        "| Filter | Geometry | f32 worst max | f32 min exact % | f32 mean |delta| | fixed worst max | fixed min exact % | fixed mean |delta| | fixed off-2+ total |\n| --- | --- | --- | --- | --- | --- | --- | --- | --- |"
    );
    let geoms: Vec<String> = results[0]
        .iter()
        .filter(|r| r.filter == Filter::Lanczos3)
        .map(|r| r.geometry.clone())
        .collect();
    for filter in [Filter::Lanczos3, Filter::Area] {
        for g in &geoms {
            let mut f32_max = 0;
            let mut f32_min_exact = 100.0_f64;
            let mut f32_total = Stats::default();
            let mut fx_max = 0;
            let mut fx_min_exact = 100.0_f64;
            let mut fx_total = Stats::default();
            for r in results.iter().flatten() {
                if r.filter != filter || &r.geometry != g || !r.opaque || !r.is_u8 {
                    continue;
                }
                f32_max = f32_max.max(r.f32.rgb.max);
                f32_min_exact = f32_min_exact.min(r.f32.rgb.exact_pct());
                f32_total.merge(&r.f32.rgb);
                if let Some((_, cmp)) = &r.fixed {
                    fx_max = fx_max.max(cmp.rgb.max);
                    fx_min_exact = fx_min_exact.min(cmp.rgb.exact_pct());
                    fx_total.merge(&cmp.rgb);
                }
            }
            let _ = writeln!(
                report,
                "| {} | {} | {} | {:.4} | {:.5} | {} | {:.4} | {:.5} | {} |",
                filter.name(),
                g,
                f32_max,
                f32_min_exact,
                f32_total.mean(),
                fx_max,
                fx_min_exact,
                fx_total.mean(),
                fx_total.off2
            );
        }
    }
    let _ = writeln!(
        report,
        "\nWorst case per geometry over the F32 input cases (f32 path only):\n"
    );
    let _ = writeln!(
        report,
        "| Filter | Geometry | f32 worst max | f32 min exact % |\n| --- | --- | --- | --- |"
    );
    for filter in [Filter::Lanczos3, Filter::Area] {
        for g in &geoms {
            let mut f32_max = 0;
            let mut f32_min_exact = 100.0_f64;
            for r in results.iter().flatten() {
                if r.filter != filter || &r.geometry != g || r.is_u8 {
                    continue;
                }
                f32_max = f32_max.max(r.f32.rgb.max);
                f32_min_exact = f32_min_exact.min(r.f32.rgb.exact_pct());
            }
            let _ = writeln!(
                report,
                "| {} | {} | {} | {:.4} |",
                filter.name(),
                g,
                f32_max,
                f32_min_exact
            );
        }
    }
    let _ = writeln!(
        report,
        "\nPer input, all geometries and both filters pooled (RGB where the reference alpha is nonzero; alpha channel separately):\n"
    );
    let _ = writeln!(
        report,
        "| Input | samples | f32 RGB | f32 alpha | fixed RGB | fixed alpha |\n| --- | --- | --- | --- | --- | --- |"
    );
    for case_results in results {
        let mut f32_rgb = Stats::default();
        let mut f32_alpha = Stats::default();
        let mut fx_rgb = Stats::default();
        let mut fx_alpha = Stats::default();
        for r in case_results {
            f32_rgb.merge(&r.f32.rgb);
            f32_alpha.merge(&r.f32.alpha);
            if let Some((_, cmp)) = &r.fixed {
                fx_rgb.merge(&cmp.rgb);
                fx_alpha.merge(&cmp.alpha);
            }
        }
        let cell = |s: &Stats| {
            if s.n == 0 {
                "-".to_string()
            } else {
                format!(
                    "max {} / {:.4}% exact / mean {:.5}",
                    s.max,
                    s.exact_pct(),
                    s.mean()
                )
            }
        };
        let _ = writeln!(
            report,
            "| {} | {} | {} | {} | {} | {} |",
            case_results[0].case,
            f32_rgb.n,
            cell(&f32_rgb),
            cell(&f32_alpha),
            cell(&fx_rgb),
            cell(&fx_alpha)
        );
    }
    let mut peak_acc = 0.0_f64;
    let mut peak_mid = 0.0_f64;
    let mut bound_acc = 0.0_f64;
    let mut bound_mid = 0.0_f64;
    let mut precisions = std::collections::BTreeMap::new();
    for r in results.iter().flatten() {
        if let Some((run, _)) = &r.fixed {
            peak_acc = peak_acc.max(run.peak_acc);
            peak_mid = peak_mid.max(run.peak_mid);
            bound_acc = bound_acc.max(run.bound_acc);
            bound_mid = bound_mid.max(run.bound_mid);
            *precisions
                .entry((run.precision_h, run.precision_v))
                .or_insert(0_u32) += 1;
        }
    }
    let _ = writeln!(
        report,
        "\nLargest fixed-point accumulator observed in any chain: {peak_acc:.4} of 2^31 (largest planner bound {bound_acc:.4}); largest intermediate row value: {peak_mid:.4} of 32767 (largest planner bound {bound_mid:.4}). Precision pairs (h, v) chosen: {precisions:?}.\n"
    );
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mut width = 1024_u32;
    let mut height = 768_u32;
    let mut exhaustive_f32 = true;
    let mut i = 0;
    while i < args.len() {
        match args[i].as_str() {
            "--width" => {
                width = args[i + 1].parse().expect("--width N");
                i += 1;
            }
            "--height" => {
                height = args[i + 1].parse().expect("--height N");
                i += 1;
            }
            "--skip-exhaustive-f32" => exhaustive_f32 = false,
            "--p-cap" => {
                P_CAP.store(
                    args[i + 1].parse().expect("--p-cap N"),
                    std::sync::atomic::Ordering::Relaxed,
                );
                i += 1;
            }
            other => panic!("unknown argument {other}"),
        }
        i += 1;
    }
    let total = Instant::now();
    let mut report = String::new();
    let _ = writeln!(report, "# ADR-0021 precision experiment\n");
    let _ = writeln!(
        report,
        "Parameters: chain images {width}x{height}, Q1.{Q} working format, coefficient precision cap {}, exact-encoder buckets {ENC_BUCKETS}, exhaustive f32 encoder check {}.\n",
        p_cap(),
        if exhaustive_f32 { "on" } else { "off" }
    );

    let start = Instant::now();
    let tables = Tables::build();
    let _ = writeln!(
        report,
        "Tables built in {:.3} s.\n",
        start.elapsed().as_secs_f64()
    );

    experiment_decode_tables(&tables, &mut report);
    let (p3_max, p3_min) = experiment_p3(&tables, &mut report);
    experiment_encode_sweeps(&tables, &mut report);
    if exhaustive_f32 {
        experiment_exhaustive_f32_encoder(&tables, &mut report);
    }
    experiment_f32_decode(&mut report);
    experiment_premultiply(&tables, &mut report);
    let (gain_lanczos, negative_lanczos, gain_area) = experiment_weight_gain(&mut report);

    // Headroom derivation from the measured numbers: interval propagation
    // through both passes with the worst Lanczos3 gain seen above.
    let _ = writeln!(
        report,
        "### Derived headroom (Lanczos3 worst gain G = {gain_lanczos:.5}, N = {negative_lanczos:.5}; Area gain {gain_area:.4})\n"
    );
    let _ = writeln!(
        report,
        "Exhaustive E2 range of decoded Display P3: [{p3_min:.4}, {p3_max:.4}]; corner-derived range used by the planner rule: [{:.4}, {:.4}] = Q1.14 [{}, {}]. This table applies the worst gain of any geometry to both passes; the planner rule in E8 instead bounds each plan exactly from its own quantized integer weights.\n",
        tables.p3_range.0, tables.p3_range.1, tables.p3_range_q14.0, tables.p3_range_q14.1
    );
    let _ = writeln!(
        report,
        "| Input | decoded range | after horizontal pass | max |row| in Q1.14 (limit 32767) | after vertical pass | max |acc| / 2^31 at p = 16 | at p = 15 |\n| --- | --- | --- | --- | --- | --- | --- |"
    );
    for (name, range) in [
        ("sRGB 8-bit", (0.0, 1.0)),
        ("Display P3 8-bit", tables.p3_range),
    ] {
        let h = propagate(range, gain_lanczos, negative_lanczos);
        let v = propagate(h, gain_lanczos, negative_lanczos);
        let h_mag = h.0.abs().max(h.1.abs());
        let v_mag = v.0.abs().max(v.1.abs());
        let _ = writeln!(
            report,
            "| {name} | [{:.4}, {:.4}] | [{:.4}, {:.4}] | {:.0} | [{:.4}, {:.4}] | {:.4} | {:.4} |",
            range.0,
            range.1,
            h.0,
            h.1,
            h_mag * f64::from(ONE_Q14),
            v.0,
            v.1,
            v_mag * (1_u64 << (Q + 16)) as f64 / (1_u64 << 31) as f64,
            v_mag * (1_u64 << (Q + 15)) as f64 / (1_u64 << 31) as f64
        );
    }
    let _ = writeln!(report);

    let start = Instant::now();
    let images = cases(width, height);
    let geoms = geometries(width, height);
    let results: Vec<Vec<RunResult>> = std::thread::scope(|scope| {
        let handles: Vec<_> = images
            .iter()
            .map(|(name, img)| {
                let tables = &tables;
                let geoms = &geoms;
                scope.spawn(move || run_case(tables, name, img, geoms))
            })
            .collect();
        handles.into_iter().map(|h| h.join().unwrap()).collect()
    });
    write_chain_report(&results, &mut report);
    write_summary(&results, &mut report);
    let _ = writeln!(
        report,
        "Chains took {:.1} s on {} threads; whole run {:.1} s.",
        start.elapsed().as_secs_f64(),
        std::thread::available_parallelism().map_or(1, |n| n.get()),
        total.elapsed().as_secs_f64()
    );
    print!("{report}");
}
