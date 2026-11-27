//! M2 work package 1: the scalar `f32` executor for the M1 chain
//! `decoder -> crop -> linear sRGB -> separable resample -> sRGB8`.
//!
//! It computes the same image as the `f64` reference in `quality_execute`,
//! within the bound of ADR-0021, but restructures the work:
//!
//! - `f32` working precision instead of `f64` (ADR-0021 rule 1);
//! - each source sample is linearized **once**, through an exact 256-entry
//!   table for 8-bit input, instead of once per filter tap;
//! - filter taps are precomputed into flat index/weight tables;
//! - horizontally filtered rows live in a ring buffer addressed in O(1);
//! - the horizontal and vertical resample run on `ferrisma-kernels`
//!   (runtime-dispatched NEON or AVX2, ADR-0020), on four `f32` channels
//!   per pixel (the fourth is unused for opaque RGB);
//! - output samples are quantized by an exact `f32` to sRGB8 encoder
//!   (bucket table plus thresholds) instead of `powf`.
//!
//! The reference executor stays compiled and is the oracle of the
//! differential tests in `fast_execute_tests`. Memory accounting, the single
//! decoder request, cancellation and the zero-full-frame contract are the
//! same as in the reference.

use std::sync::{Arc, OnceLock};
use std::time::{Duration, Instant};

use ferrisma_kernels::{KernelError, Kernels, Taps};

use crate::color::{AlphaMode, ColorEncoding};
use crate::decoder::{
    ChannelLayout, DecodeReport, DecodeSource, DecodedRow, RowConversion, SampleType,
};
use crate::error::Error;
use crate::geometry::Rect;
use crate::graph::{Graph, NodeId};
use crate::memory::{MemoryBudget, try_filled_vec, try_vec};
use crate::output_depth::{EIGHT, OutputBuffer, OutputRow, max_code};
use crate::quality::taps_into;
use crate::quality_execute::{
    Prepared, QualityReport, prepare, reserve_resize_plan, tap_entry_bound,
};
use crate::working_color::{
    decode_srgb, display_p3_linear_to_srgb_linear, encode_linear_srgb8, p3_to_srgb_matrix,
};

/// Pixels processed per stack-buffered chunk in `linearize_row` and
/// `quantize_row_deep` (64 * 3 = 192 `f32` = 768 bytes, L1-resident with zero
/// heap allocation).
const TRANSFER_CHUNK_PIXELS: usize = 64;

/// Stack chunk size for vertical tap row slices passed to
/// `Kernels::vertical_rgba` / `Kernels::vertical_rgba_add` with zero heap
/// allocation for any tap count.
pub(crate) const VERTICAL_STACK_TAPS: usize = 64;

/// Conservative absolute error factor for `display_p3_linear_to_srgb_linear`
/// applied to polynomial-decoded in-range channels: L1 row norm (`<= 1.45`)
/// times `1.4e-10` is `< 2.1e-10`, widened to `6.0e-10`.
const P3_ERR_GUARD: f64 = 6.0e-10;

/// Guard half-width in `f64` ULPs around the `f32` rounding midpoint
/// (`low29 == 0x1000_0000`) for `Kernels::srgb_to_linear` on `[-1, 1]`.
/// Max relative error is `< 1.4e-10` (plus `< 1e-14` for unfused `Scalar` and
/// `f64` alpha multiply), which is `< 2 * 2.0e-10 * 2^52 = 1,801,440 < 0x001C_0000`
/// `f64` ULPs; we use `0x0020_0000` (`2,097,152` ULPs, > `2.3e-10` relative).
const SRGB_F32_TIE_GUARD: u32 = 0x0020_0000;

#[cold]
#[inline(never)]
fn refine_srgb_channel(raw: f32, value: f64, alpha: f64) -> f32 {
    // `srgb_to_linear` already evaluated exact `f64::powf` outside `[-1, 1]`.
    if raw.abs() > 1.0 {
        #[allow(clippy::cast_possible_truncation)]
        return value as f32;
    }
    // Tiny nonzero alpha can produce subnormal `f32` products whose error
    // interval does not straddle a subnormal rounding tie; avoid `powf` then.
    let eps = value.abs() * 2.5e-10;
    #[allow(clippy::cast_possible_truncation)]
    let (lo, hi) = ((value - eps) as f32, (value + eps) as f32);
    if lo.to_bits() == hi.to_bits() {
        return lo;
    }
    #[allow(clippy::cast_possible_truncation)]
    {
        (decode_srgb(f64::from(raw)) * alpha) as f32
    }
}

/// Narrow `decoded * alpha` to `f32`, guaranteeing bit-identity with exact
/// `(decode_srgb(f64::from(raw)) * alpha) as f32` for both opaque (`alpha = 1.0`)
/// and straight-alpha sRGB.
///
/// For normal `f32` magnitudes (`f64` biased exponent `897..=1150`), the exact
/// `f32` round-to-nearest-even tie lies at `low29 == 0x1000_0000`. Any value
/// further than `SRGB_F32_TIE_GUARD` `f64` ULPs from that tie is guaranteed to
/// round to the same `f32` as `decode_srgb`; both `+0.0` and `-0.0` preserve
/// exact sign bits directly on the fast path, and only the ~0.8% of values
/// inside the tie band (plus subnormals) enter `refine_srgb_channel`, which
/// also skips re-decoding extended-range (`|raw| > 1.0`) channels.
#[inline]
fn narrow_srgb_channel(raw: f32, decoded: f64, alpha: f64) -> f32 {
    let value = decoded * alpha;
    let bits = value.to_bits();
    #[allow(clippy::cast_possible_truncation)]
    let exp = ((bits >> 52) & 0x7FF) as u32;
    #[allow(clippy::cast_possible_truncation)]
    let low29 = (bits as u32) & 0x1FFF_FFFF;
    if value == 0.0
        || (exp.wrapping_sub(897) <= (1150 - 897)
            && low29.wrapping_sub(0x1000_0000 - SRGB_F32_TIE_GUARD) >= 2 * SRGB_F32_TIE_GUARD)
    {
        #[allow(clippy::cast_possible_truncation)]
        return value as f32;
    }
    refine_srgb_channel(raw, value, alpha)
}

/// Decode a Display P3 pixel once with exact `f64` `decode_srgb`, apply the
/// exact two-step reference matrix, premultiply by `alpha`, and narrow once to
/// `f32`.
#[inline]
fn decode_p3_exact(pixel: [f32; 3], alpha: f64) -> [f32; 3] {
    let exact = [
        decode_srgb(f64::from(pixel[0])),
        decode_srgb(f64::from(pixel[1])),
        decode_srgb(f64::from(pixel[2])),
    ];
    let rgb = display_p3_linear_to_srgb_linear(exact);
    #[allow(clippy::cast_possible_truncation)]
    [
        (rgb[0] * alpha) as f32,
        (rgb[1] * alpha) as f32,
        (rgb[2] * alpha) as f32,
    ]
}

#[cold]
#[inline(never)]
fn refine_p3_pixel(pixel: [f32; 3], alpha: f64) -> [f32; 3] {
    decode_p3_exact(pixel, alpha)
}

/// Convert an in-range (`|pixel[c]| <= 1.0`) Display P3 linear `f64` triple
/// through the exact two-step reference matrix and premultiply by `alpha`,
/// refining with exact `decode_srgb` whenever the polynomial error interval
/// could cross an `f32` rounding boundary.
#[inline]
fn narrow_p3_in_range(pixel: [f32; 3], decoded: [f64; 3], alpha: f64) -> [f32; 3] {
    let rgb = display_p3_linear_to_srgb_linear(decoded);
    let peak = decoded[0].abs().max(decoded[1].abs()).max(decoded[2].abs());
    if peak == 0.0 {
        // Signed zeros are decoded exactly; preserve their exact sign bits
        // without triggering `refine_p3_pixel`.
        #[allow(clippy::cast_possible_truncation)]
        return [
            (rgb[0] * alpha) as f32,
            (rgb[1] * alpha) as f32,
            (rgb[2] * alpha) as f32,
        ];
    }
    let eps_unscaled = peak * P3_ERR_GUARD;
    if alpha == 0.0 {
        if rgb[0].abs() <= eps_unscaled
            || rgb[1].abs() <= eps_unscaled
            || rgb[2].abs() <= eps_unscaled
        {
            return refine_p3_pixel(pixel, alpha);
        }
        #[allow(clippy::cast_possible_truncation)]
        return [
            (rgb[0] * alpha) as f32,
            (rgb[1] * alpha) as f32,
            (rgb[2] * alpha) as f32,
        ];
    }
    let eps = eps_unscaled * alpha;
    let v0 = rgb[0] * alpha;
    let v1 = rgb[1] * alpha;
    let v2 = rgb[2] * alpha;
    #[allow(clippy::cast_possible_truncation)]
    let (lo0, hi0) = ((v0 - eps) as f32, (v0 + eps) as f32);
    #[allow(clippy::cast_possible_truncation)]
    let (lo1, hi1) = ((v1 - eps) as f32, (v1 + eps) as f32);
    #[allow(clippy::cast_possible_truncation)]
    let (lo2, hi2) = ((v2 - eps) as f32, (v2 + eps) as f32);
    if lo0.to_bits() == hi0.to_bits()
        && lo1.to_bits() == hi1.to_bits()
        && lo2.to_bits() == hi2.to_bits()
    {
        [lo0, lo1, lo2]
    } else {
        refine_p3_pixel(pixel, alpha)
    }
}

/// Whether a Display P3 pixel should bypass the SIMD polynomial and decode
/// once via [`decode_p3_exact`]:
/// - any channel is extended-range (`|c| > 1.0`), avoiding unused polynomial
///   work on mixed-range pixels, or
/// - blue-only (`r == 0.0 && g == 0.0 && b != 0.0`), whose two-step matrix
///   produces near-zero red/green terms whose polynomial error band straddles
///   zero and would otherwise double-decode via `refine_p3_pixel`.
#[inline]
fn p3_needs_direct_exact(pixel: [f32; 3]) -> bool {
    let p_peak = (pixel[0].to_bits() & 0x7FFF_FFFF)
        .max(pixel[1].to_bits() & 0x7FFF_FFFF)
        .max(pixel[2].to_bits() & 0x7FFF_FFFF);
    p_peak > 0x3F80_0000 || (pixel[0] == 0.0 && pixel[1] == 0.0 && pixel[2] != 0.0)
}

/// Linearize an opaque Display P3 chunk that contains extended-range or
/// blue-only pixels: route those pixels directly through exact
/// [`decode_p3_exact`] once, and compact any remaining in-range pixels in the
/// chunk for one SIMD `srgb_to_linear` call.
fn linearize_p3_direct_chunk_rgb(
    chunk_in: &[[f32; 3]],
    kernels: &Kernels,
    linear_buf: &mut [f64; TRANSFER_CHUNK_PIXELS * 3],
    chunk_out: &mut [[f32; 4]],
) -> Result<(), Error> {
    let mut in_range_encoded = [0.0_f32; TRANSFER_CHUNK_PIXELS * 3];
    let mut in_range_idx = [0_usize; TRANSFER_CHUNK_PIXELS];
    let mut count = 0_usize;
    for (i, pixel) in chunk_in.iter().enumerate() {
        if p3_needs_direct_exact(*pixel) {
            let rgb = decode_p3_exact(*pixel, 1.0);
            chunk_out[i] = [rgb[0], rgb[1], rgb[2], 0.0];
        } else {
            in_range_encoded[count * 3] = pixel[0];
            in_range_encoded[count * 3 + 1] = pixel[1];
            in_range_encoded[count * 3 + 2] = pixel[2];
            in_range_idx[count] = i;
            count += 1;
        }
    }
    if count > 0 {
        let rgb_len = count * 3;
        kernels
            .srgb_to_linear(&in_range_encoded[..rgb_len], &mut linear_buf[..rgb_len])
            .map_err(kernel_error)?;
        for (k, &out_idx) in in_range_idx[..count].iter().enumerate() {
            let decoded = [
                linear_buf[k * 3],
                linear_buf[k * 3 + 1],
                linear_buf[k * 3 + 2],
            ];
            let rgb = narrow_p3_in_range(chunk_in[out_idx], decoded, 1.0);
            chunk_out[out_idx] = [rgb[0], rgb[1], rgb[2], 0.0];
        }
    }
    Ok(())
}

/// Linearize a straight-alpha Display P3 chunk that contains extended-range or
/// blue-only pixels without evaluating unused polynomial approximations.
fn linearize_p3_direct_chunk_rgba(
    chunk_in: &[[f32; 4]],
    kernels: &Kernels,
    encoded_buf: &mut [f32; TRANSFER_CHUNK_PIXELS * 3],
    linear_buf: &mut [f64; TRANSFER_CHUNK_PIXELS * 3],
    chunk_out: &mut [[f32; 4]],
) -> Result<(), Error> {
    let mut in_range_idx = [0_usize; TRANSFER_CHUNK_PIXELS];
    let mut count = 0_usize;
    for (i, pixel) in chunk_in.iter().enumerate() {
        let rgb_in = [pixel[0], pixel[1], pixel[2]];
        if p3_needs_direct_exact(rgb_in) {
            let alpha = pixel[3];
            let rgb = decode_p3_exact(rgb_in, f64::from(alpha));
            chunk_out[i] = [rgb[0], rgb[1], rgb[2], alpha];
        } else {
            encoded_buf[count * 3] = pixel[0];
            encoded_buf[count * 3 + 1] = pixel[1];
            encoded_buf[count * 3 + 2] = pixel[2];
            in_range_idx[count] = i;
            count += 1;
        }
    }
    if count > 0 {
        let rgb_len = count * 3;
        kernels
            .srgb_to_linear(&encoded_buf[..rgb_len], &mut linear_buf[..rgb_len])
            .map_err(kernel_error)?;
        for (k, &out_idx) in in_range_idx[..count].iter().enumerate() {
            let pixel = chunk_in[out_idx];
            let alpha = pixel[3];
            let decoded = [
                linear_buf[k * 3],
                linear_buf[k * 3 + 1],
                linear_buf[k * 3 + 2],
            ];
            let rgb = narrow_p3_in_range([pixel[0], pixel[1], pixel[2]], decoded, f64::from(alpha));
            chunk_out[out_idx] = [rgb[0], rgb[1], rgb[2], alpha];
        }
    }
    Ok(())
}

/// Largest unsigned magnitude bit pattern across `samples`. In IEEE-754
/// binary32, every finite sample has bits `< 0x7F80_0000` (`f32::INFINITY`),
/// and every sample in `[-1.0, 1.0]` has bits `<= 0x3F80_0000` (`1.0_f32`).
/// Branchless so LLVM auto-vectorizes it with `umax`.
#[inline]
fn max_abs_bits(samples: &[f32]) -> u32 {
    let mut max_bits = 0_u32;
    for &sample in samples {
        max_bits = max_bits.max(sample.to_bits() & 0x7FFF_FFFF);
    }
    max_bits
}

/// Bucket count of the encoder table; every bucket spans at most one code
/// step on the sRGB curve (the steepest slope is 12.92 * 255 codes per unit).
const ENCODER_BUCKETS: usize = 4096;

pub(crate) struct Tables {
    /// `decode_srgb(code / 255)` rounded to `f32`.
    decode_u8: [f32; 256],
    /// `encode_linear_srgb8(bucket / 4096)` for bucket `0..=4096`.
    bucket_code: Vec<u8>,
    /// `threshold[c]` is the smallest `f32` in `[0, 1]` that the reference
    /// encodes to at least `c`; `threshold[256]` is infinity.
    threshold: [f32; 257],
}

impl Tables {
    fn build() -> Self {
        let mut decode_u8 = [0.0_f32; 256];
        for (code, value) in decode_u8.iter_mut().enumerate() {
            #[allow(clippy::cast_possible_truncation)]
            {
                *value =
                    decode_srgb(f64::from(u8::try_from(code).unwrap_or(u8::MAX)) / 255.0) as f32;
            }
        }
        let reference = |value: f32| encode_linear_srgb8(f64::from(value)).unwrap_or(u8::MAX);
        let mut threshold = [0.0_f32; 257];
        threshold[256] = f32::INFINITY;
        let one = 1.0_f32.to_bits();
        for code in 1..=255_u16 {
            // Non-negative f32 values are ordered like their bit patterns.
            let (mut low, mut high) = (0_u32, one);
            while low < high {
                let middle = low + (high - low) / 2;
                if u16::from(reference(f32::from_bits(middle))) >= code {
                    high = middle;
                } else {
                    low = middle + 1;
                }
            }
            threshold[usize::from(code)] = f32::from_bits(low);
        }
        let bucket_code = (0..=ENCODER_BUCKETS)
            .map(|bucket| {
                #[allow(clippy::cast_precision_loss)]
                reference(bucket as f32 / ENCODER_BUCKETS as f32)
            })
            .collect();
        Self {
            decode_u8,
            bucket_code,
            threshold,
        }
    }

    /// Exact for every `f32` in `[0, 1]`: identical to
    /// `encode_linear_srgb8(f64::from(value))`. The caller clamps and rejects
    /// non-finite values first.
    #[inline]
    pub(crate) fn encode(&self, value: f32) -> u8 {
        #[allow(
            clippy::cast_possible_truncation,
            clippy::cast_sign_loss,
            clippy::cast_precision_loss
        )]
        let bucket = ((value * ENCODER_BUCKETS as f32) as usize).min(ENCODER_BUCKETS);
        let mut code = usize::from(self.bucket_code[bucket]);
        while value >= self.threshold[code + 1] {
            code += 1;
        }
        u8::try_from(code).unwrap_or(u8::MAX)
    }
}

pub(crate) fn tables() -> &'static Tables {
    static TABLES: OnceLock<Tables> = OnceLock::new();
    TABLES.get_or_init(Tables::build)
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub(crate) enum Primaries {
    Srgb,
    DisplayP3,
    /// Already linear sRGB (CICP transfer 8): no transfer decode.
    LinearSrgb,
}

/// Flat tap tables: taps of output position `i` are
/// `index[start[i]..start[i + 1]]` with matching `weight`s.
pub(crate) struct TapTable {
    start: Vec<usize>,
    index: Vec<u32>,
    weight: Vec<f32>,
}

impl TapTable {
    pub(crate) fn build(
        input: u32,
        output: u32,
        first: u32,
        count: u32,
        filter: crate::quality::Filter,
        origin: u32,
    ) -> Result<Self, Error> {
        let mut table = Self {
            start: try_vec(
                usize::try_from(count)
                    .map_err(|_| Error::GeometryOverflow)?
                    .checked_add(1)
                    .ok_or(Error::GeometryOverflow)?,
            )?,
            index: try_vec(tap_entry_bound(input, output, count, filter)?)?,
            weight: try_vec(tap_entry_bound(input, output, count, filter)?)?,
        };
        table.start.push(0);
        let mut scratch = Vec::new();
        for offset in 0..count {
            let at = first.checked_add(offset).ok_or(Error::GeometryOverflow)?;
            taps_into(input, output, at, filter, &mut scratch)?;
            for tap in &scratch {
                table
                    .index
                    .push(tap.index.checked_sub(origin).ok_or(Error::OutOfBounds)?);
                #[allow(clippy::cast_possible_truncation)]
                table.weight.push(tap.weight as f32);
            }
            table.start.push(table.index.len());
        }
        Ok(table)
    }

    fn taps(&self, position: usize) -> (&[u32], &[f32]) {
        let range = self.start[position]..self.start[position + 1];
        (&self.index[range.clone()], &self.weight[range])
    }

    /// Contiguous runs for the SIMD kernels: each output position starts at
    /// its first tap, and gaps (taps the filter drops as zero) get weight 0,
    /// which adds exactly nothing.
    fn contiguous(&self, source_pixels: usize) -> Result<Taps, Error> {
        let outputs = self.start.len() - 1;
        let mut starts = try_vec(outputs)?;
        let mut offsets = try_vec(outputs.checked_add(1).ok_or(Error::GeometryOverflow)?)?;
        let weight_count = (0..outputs).try_fold(0_usize, |total, position| {
            let (indices, _) = self.taps(position);
            let first = indices.first().ok_or(Error::IncompleteDecoderOutput)?;
            let last = indices.last().ok_or(Error::IncompleteDecoderOutput)?;
            let span = usize::try_from(last - first + 1).map_err(|_| Error::GeometryOverflow)?;
            total.checked_add(span).ok_or(Error::GeometryOverflow)
        })?;
        let mut weights = try_vec(weight_count)?;
        offsets.push(0_u32);
        for position in 0..outputs {
            let (indices, values) = self.taps(position);
            let first = *indices.first().ok_or(Error::IncompleteDecoderOutput)?;
            starts.push(first);
            let mut next = first;
            for (&index, &weight) in indices.iter().zip(values) {
                while next < index {
                    weights.push(0.0);
                    next += 1;
                }
                weights.push(weight);
                next = index + 1;
            }
            offsets.push(u32::try_from(weights.len()).map_err(|_| Error::GeometryOverflow)?);
        }
        Taps::new(source_pixels, starts, offsets, weights).map_err(kernel_error)
    }
}

pub(crate) fn kernel_error(error: KernelError) -> Error {
    match error {
        KernelError::InvalidTaps => Error::OutOfBounds,
        KernelError::NonFiniteSample => Error::NonFinitePixelSample,
        KernelError::LengthMismatch | KernelError::UnsupportedLevel(_) => {
            Error::IncompleteDecoderOutput
        }
    }
}

/// Fast `f32` counterpart of `quality_execute::execute_quality_srgb8`, with
/// the same graph contract, sink contract and report: [`execute_fast`] at
/// 8 bits.
#[allow(clippy::too_many_arguments)] // The reference signature plus the kernel table.
#[cfg(test)] // The 8-bit contract of the executor tests.
pub(crate) fn execute_fast_srgb8(
    graph: &Graph,
    target: NodeId,
    bound_source: NodeId,
    source: &mut dyn DecodeSource,
    requested: Rect,
    budget: &MemoryBudget,
    kernels: &Kernels,
    threads: usize,
    sink: impl FnMut(u32, &[u8]) -> Result<(), Error>,
) -> Result<QualityReport, Error> {
    execute_fast(
        graph,
        target,
        bound_source,
        source,
        requested,
        budget,
        kernels,
        threads,
        EIGHT,
        crate::output_depth::eight_bit_sink(sink),
    )
}

/// Fast `f32` counterpart of `quality_execute::execute_quality`: the same
/// graph contract, sink contract, report and output `depth` (8 to 16 bits;
/// 8-bit rows are `u8`, deeper rows `u16`, ADR-0021 amendment of
/// 2026-11-15). The resample runs on `kernels` (ADR-0020); every pixel is
/// stored as four `f32` channels. With `threads > 1` the rows run on a
/// pipeline of workers (ADR-0022) that produces byte-identical output.
#[allow(clippy::too_many_arguments)] // The reference signature plus the kernel table.
pub(crate) fn execute_fast(
    graph: &Graph,
    target: NodeId,
    bound_source: NodeId,
    source: &mut dyn DecodeSource,
    requested: Rect,
    budget: &MemoryBudget,
    kernels: &Kernels,
    threads: usize,
    depth: u8,
    sink: impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
) -> Result<QualityReport, Error> {
    if !(EIGHT..=crate::output_depth::MAX_DEPTH).contains(&depth) {
        return Err(Error::InvalidEncodeOptions);
    }
    let info = source.info().clone();
    match (info.layout, info.alpha) {
        (ChannelLayout::Rgb, AlphaMode::Opaque) => run::<3>(
            graph,
            target,
            bound_source,
            source,
            requested,
            budget,
            kernels,
            threads,
            depth,
            sink,
        ),
        (ChannelLayout::Rgba, AlphaMode::Straight) => run::<4>(
            graph,
            target,
            bound_source,
            source,
            requested,
            budget,
            kernels,
            threads,
            depth,
            sink,
        ),
        (ChannelLayout::Rgb | ChannelLayout::Rgba, _) => {
            // Same order of checks as the reference: reservation and binding first.
            let _plan_reservation = reserve_resize_plan(graph, target, requested, budget)?;
            prepare(graph, target, bound_source, source, requested)?;
            Err(Error::UnsupportedAlphaMode)
        }
        _ => {
            let _plan_reservation = reserve_resize_plan(graph, target, requested, budget)?;
            prepare(graph, target, bound_source, source, requested)?;
            Err(Error::UnsupportedPixelOperation)
        }
    }
}

/// Decode one source row to linear-light (premultiplied for `C == 4`) `f32`
/// samples, once per sample, four values per pixel (the fourth is 0 for
/// opaque RGB).
pub(crate) fn linearize_row<const C: usize>(
    row: DecodedRow<'_>,
    primaries: Primaries,
    p3: &[[f32; 3]; 3],
    kernels: &Kernels,
    out: &mut [f32],
) -> Result<(), Error> {
    let lut = &tables().decode_u8;
    match row {
        // Linear input (the ICC adapter's output): copy, check, premultiply.
        DecodedRow::F32(samples) if primaries == Primaries::LinearSrgb => {
            for (pixel, linear) in samples
                .as_chunks::<C>()
                .0
                .iter()
                .zip(out.as_chunks_mut::<4>().0)
            {
                if pixel.iter().any(|value| !value.is_finite()) {
                    return Err(Error::NonFinitePixelSample);
                }
                if C == 4 {
                    let alpha = pixel[3];
                    if !(0.0..=1.0).contains(&alpha) {
                        return Err(Error::InvalidAlphaValue);
                    }
                    *linear = [pixel[0] * alpha, pixel[1] * alpha, pixel[2] * alpha, alpha];
                } else {
                    *linear = [pixel[0], pixel[1], pixel[2], 0.0];
                }
            }
        }
        DecodedRow::U8(samples) if primaries == Primaries::LinearSrgb => {
            for (pixel, linear) in samples
                .as_chunks::<C>()
                .0
                .iter()
                .zip(out.as_chunks_mut::<4>().0)
            {
                let rgb = [pixel[0], pixel[1], pixel[2]].map(|v| f32::from(v) / 255.0);
                if C == 4 {
                    let alpha = f32::from(pixel[3]) / 255.0;
                    *linear = [rgb[0] * alpha, rgb[1] * alpha, rgb[2] * alpha, alpha];
                } else {
                    *linear = [rgb[0], rgb[1], rgb[2], 0.0];
                }
            }
        }
        DecodedRow::U8(samples) => {
            for (pixel, linear) in samples
                .as_chunks::<C>()
                .0
                .iter()
                .zip(out.as_chunks_mut::<4>().0)
            {
                let mut rgb = [
                    lut[usize::from(pixel[0])],
                    lut[usize::from(pixel[1])],
                    lut[usize::from(pixel[2])],
                ];
                if primaries == Primaries::DisplayP3 {
                    rgb = p3.map(|m| m[0].mul_add(rgb[0], m[1].mul_add(rgb[1], m[2] * rgb[2])));
                }
                if C == 4 {
                    let alpha = f32::from(pixel[3]) / 255.0;
                    linear[0] = rgb[0] * alpha;
                    linear[1] = rgb[1] * alpha;
                    linear[2] = rgb[2] * alpha;
                    linear[3] = alpha;
                } else {
                    *linear = [rgb[0], rgb[1], rgb[2], 0.0];
                }
            }
        }
        DecodedRow::F32(samples) => {
            linearize_f32_row::<C>(samples, primaries, kernels, out)?;
        }
    }
    Ok(())
}

/// Float input (JPEG XL and >8-bit AVIF): decode once through the SIMD
/// `f64`-lane sRGB EOTF kernel into L1-resident `f64` chunks, apply any
/// Display P3 two-step primaries matrix and straight-alpha premultiplication
/// in `f64`, and narrow the final working pixel once to `f32`. Rejects
/// non-finite input samples, out-of-range alpha, and unrepresentable working
/// pixels identically to the scalar reference.
fn linearize_f32_row<const C: usize>(
    samples: &[f32],
    primaries: Primaries,
    kernels: &Kernels,
    out: &mut [f32],
) -> Result<(), Error> {
    if C == 3 {
        linearize_f32_rgb_row(samples, primaries, kernels, out)
    } else {
        linearize_f32_rgba_row(samples, primaries, kernels, out)
    }
}

fn linearize_f32_rgb_row(
    samples: &[f32],
    primaries: Primaries,
    kernels: &Kernels,
    out: &mut [f32],
) -> Result<(), Error> {
    let peak = max_abs_bits(samples);
    if peak >= 0x7F80_0000 {
        return Err(Error::NonFinitePixelSample);
    }
    let has_extended = peak > 0x3F80_0000;
    let mut linear_buf = [0.0_f64; TRANSFER_CHUNK_PIXELS * 3];
    let in_pixels = samples.as_chunks::<3>().0;
    let out_pixels = out.as_chunks_mut::<4>().0;
    for (chunk_in, chunk_out) in in_pixels
        .chunks(TRANSFER_CHUNK_PIXELS)
        .zip(out_pixels.chunks_mut(TRANSFER_CHUNK_PIXELS))
    {
        if primaries == Primaries::DisplayP3
            && ((has_extended && max_abs_bits(chunk_in.as_flattened()) > 0x3F80_0000)
                || chunk_in
                    .iter()
                    .any(|p| p[0] == 0.0 && p[1] == 0.0 && p[2] != 0.0))
        {
            linearize_p3_direct_chunk_rgb(chunk_in, kernels, &mut linear_buf, chunk_out)?;
            continue;
        }
        let rgb_len = chunk_in.len() * 3;
        kernels
            .srgb_to_linear(chunk_in.as_flattened(), &mut linear_buf[..rgb_len])
            .map_err(kernel_error)?;
        if primaries == Primaries::DisplayP3 {
            for ((pixel, decoded), linear) in chunk_in
                .iter()
                .zip(linear_buf[..rgb_len].as_chunks::<3>().0)
                .zip(chunk_out.iter_mut())
            {
                let rgb = narrow_p3_in_range(*pixel, *decoded, 1.0);
                *linear = [rgb[0], rgb[1], rgb[2], 0.0];
            }
        } else {
            for ((pixel, decoded), linear) in chunk_in
                .iter()
                .zip(linear_buf[..rgb_len].as_chunks::<3>().0)
                .zip(chunk_out.iter_mut())
            {
                *linear = [
                    narrow_srgb_channel(pixel[0], decoded[0], 1.0),
                    narrow_srgb_channel(pixel[1], decoded[1], 1.0),
                    narrow_srgb_channel(pixel[2], decoded[2], 1.0),
                    0.0,
                ];
            }
        }
    }
    if has_extended && max_abs_bits(out) >= 0x7F80_0000 {
        return Err(Error::NonFinitePixelSample);
    }
    Ok(())
}

fn linearize_f32_rgba_row(
    samples: &[f32],
    primaries: Primaries,
    kernels: &Kernels,
    out: &mut [f32],
) -> Result<(), Error> {
    let mut encoded_buf = [0.0_f32; TRANSFER_CHUNK_PIXELS * 3];
    let mut linear_buf = [0.0_f64; TRANSFER_CHUNK_PIXELS * 3];
    let in_pixels = samples.as_chunks::<4>().0;
    let out_pixels = out.as_chunks_mut::<4>().0;
    let mut has_extended = false;
    for (chunk_in, chunk_out) in in_pixels
        .chunks(TRANSFER_CHUNK_PIXELS)
        .zip(out_pixels.chunks_mut(TRANSFER_CHUNK_PIXELS))
    {
        let rgb_len = chunk_in.len() * 3;
        let mut peak_rgb = 0_u32;
        let mut or_bits = 0_u32;
        let mut has_blue_only = false;
        for (i, pixel) in chunk_in.iter().enumerate() {
            let alpha = pixel[3];
            if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
                return Err(Error::InvalidAlphaValue);
            }
            let r_bits = pixel[0].to_bits();
            let g_bits = pixel[1].to_bits();
            let b_bits = pixel[2].to_bits();
            let p_peak = (r_bits & 0x7FFF_FFFF)
                .max(g_bits & 0x7FFF_FFFF)
                .max(b_bits & 0x7FFF_FFFF);
            if p_peak >= 0x7F80_0000 {
                return Err(Error::NonFinitePixelSample);
            }
            peak_rgb = peak_rgb.max(p_peak);
            or_bits |= r_bits | g_bits | b_bits | alpha.to_bits();
            has_blue_only |= pixel[0] == 0.0 && pixel[1] == 0.0 && pixel[2] != 0.0;
            encoded_buf[i * 3] = pixel[0];
            encoded_buf[i * 3 + 1] = pixel[1];
            encoded_buf[i * 3 + 2] = pixel[2];
        }
        if or_bits == 0 {
            chunk_out.fill([0.0, 0.0, 0.0, 0.0]);
            continue;
        }
        if peak_rgb > 0x3F80_0000 {
            has_extended = true;
        }
        if primaries == Primaries::DisplayP3 && (peak_rgb > 0x3F80_0000 || has_blue_only) {
            linearize_p3_direct_chunk_rgba(
                chunk_in,
                kernels,
                &mut encoded_buf,
                &mut linear_buf,
                chunk_out,
            )?;
            continue;
        }
        kernels
            .srgb_to_linear(&encoded_buf[..rgb_len], &mut linear_buf[..rgb_len])
            .map_err(kernel_error)?;
        if primaries == Primaries::DisplayP3 {
            for ((pixel, decoded), linear) in chunk_in
                .iter()
                .zip(linear_buf[..rgb_len].as_chunks::<3>().0)
                .zip(chunk_out.iter_mut())
            {
                let alpha = pixel[3];
                let rgb =
                    narrow_p3_in_range([pixel[0], pixel[1], pixel[2]], *decoded, f64::from(alpha));
                *linear = [rgb[0], rgb[1], rgb[2], alpha];
            }
        } else {
            for ((pixel, decoded), linear) in chunk_in
                .iter()
                .zip(linear_buf[..rgb_len].as_chunks::<3>().0)
                .zip(chunk_out.iter_mut())
            {
                let alpha = pixel[3];
                let alpha_f64 = f64::from(alpha);
                *linear = [
                    narrow_srgb_channel(pixel[0], decoded[0], alpha_f64),
                    narrow_srgb_channel(pixel[1], decoded[1], alpha_f64),
                    narrow_srgb_channel(pixel[2], decoded[2], alpha_f64),
                    alpha,
                ];
            }
        }
    }
    if has_extended && max_abs_bits(out) >= 0x7F80_0000 {
        return Err(Error::NonFinitePixelSample);
    }
    Ok(())
}

/// Per-thread buffers for a deferred source conversion; empty without one or
/// when the conversion fuses directly into `linear`.
pub(crate) struct ConversionScratch {
    scratch: crate::memory::ManagedBuffer<f32>,
    converted: crate::memory::ManagedBuffer<f32>,
}

impl ConversionScratch {
    pub(crate) fn new(plan: &RowPlan, budget: &MemoryBudget) -> Result<Self, Error> {
        let (scratch_len, converted_len) = match &plan.conversion {
            Some(conversion) => {
                let pixels = plan.linear_samples / 4;
                let (scratch_ch, converted_ch) = conversion.scratch_channels();
                (
                    pixels
                        .checked_mul(scratch_ch)
                        .ok_or(Error::GeometryOverflow)?,
                    pixels
                        .checked_mul(converted_ch)
                        .ok_or(Error::GeometryOverflow)?,
                )
            }
            None => (0, 0),
        };
        Ok(Self {
            scratch: budget.allocate_typed::<f32>(scratch_len)?,
            converted: budget.allocate_typed::<f32>(converted_len)?,
        })
    }
}

/// Apply the plan's deferred source conversion, if any, then linearize.
pub(crate) fn source_row_to_linear<const C: usize>(
    plan: &RowPlan,
    row: DecodedRow<'_>,
    buffers: &mut ConversionScratch,
    kernels: &Kernels,
    linear: &mut [f32],
) -> Result<(), Error> {
    let row = match &plan.conversion {
        Some(conversion) => {
            if conversion.convert_to_linear_rgba(row, linear)? {
                return Ok(());
            }
            conversion.convert(
                row,
                buffers.scratch.as_mut_slice(),
                buffers.converted.as_mut_slice(),
            )?;
            DecodedRow::F32(buffers.converted.as_slice())
        }
        None => row,
    };
    linearize_row::<C>(row, plan.primaries, &plan.p3, kernels, linear)
}

/// Encode one row of filtered linear samples to straight sRGB8 RGBA.
pub(crate) fn quantize_row<const C: usize>(
    acc: &[f32],
    output: &mut [u8],
    tables: &Tables,
) -> Result<(), Error> {
    for (pixel, rgba) in acc
        .as_chunks::<4>()
        .0
        .iter()
        .zip(output.as_chunks_mut::<4>().0)
    {
        if pixel.iter().any(|sample| !sample.is_finite()) {
            return Err(Error::NonFinitePixelSample);
        }
        if C == 3 {
            for channel in 0..3 {
                rgba[channel] = tables.encode(pixel[channel].clamp(0.0, 1.0));
            }
            rgba[3] = u8::MAX;
        } else {
            let alpha = pixel[3].clamp(0.0, 1.0);
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            let alpha_byte = (alpha * 255.0).round() as u8;
            rgba[3] = alpha_byte;
            // ADR-0014: alpha byte 0 is transparent black.
            for channel in 0..3 {
                rgba[channel] = if alpha_byte == 0 {
                    0
                } else {
                    tables.encode((pixel[channel] / alpha).clamp(0.0, 1.0))
                };
            }
        }
    }
    Ok(())
}

/// Encode one row of filtered linear samples to straight sRGB RGBA at
/// `depth` bits (above 8), scaled to `0..=2^depth - 1`. The same steps as
/// [`quantize_row`] in the same order (clamp, alpha rounded in `f32`,
/// division by alpha in `f32`, transparent black at alpha code 0), using
/// `kernels.linear_to_srgb_u16` (`f64`-lane minimax OETF with exact-OETF
/// half-code tie resolution, returning the exact code of
/// `working_color::encode_linear_srgb` for every `f32`).
pub(crate) fn quantize_row_deep<const C: usize>(
    acc: &[f32],
    output: &mut [u16],
    depth: u8,
    kernels: &Kernels,
) -> Result<(), Error> {
    if max_abs_bits(acc) >= 0x7F80_0000 {
        return Err(Error::NonFinitePixelSample);
    }
    let max = max_code(depth);
    let scale = f32::from(max);
    let mut rgb_in = [0.0_f32; TRANSFER_CHUNK_PIXELS * 3];
    let mut rgb_out = [0_u16; TRANSFER_CHUNK_PIXELS * 3];
    let in_pixels = acc.as_chunks::<4>().0;
    let out_pixels = output.as_chunks_mut::<4>().0;
    for (chunk_in, chunk_out) in in_pixels
        .chunks(TRANSFER_CHUNK_PIXELS)
        .zip(out_pixels.chunks_mut(TRANSFER_CHUNK_PIXELS))
    {
        let rgb_len = chunk_in.len() * 3;
        if C == 3 {
            for (i, pixel) in chunk_in.iter().enumerate() {
                rgb_in[i * 3] = pixel[0];
                rgb_in[i * 3 + 1] = pixel[1];
                rgb_in[i * 3 + 2] = pixel[2];
            }
            kernels
                .linear_to_srgb_u16(&rgb_in[..rgb_len], max, &mut rgb_out[..rgb_len])
                .map_err(kernel_error)?;
            for (encoded, rgba) in rgb_out[..rgb_len]
                .as_chunks::<3>()
                .0
                .iter()
                .zip(chunk_out.iter_mut())
            {
                *rgba = [encoded[0], encoded[1], encoded[2], max];
            }
        } else {
            let mut visible_idx = [0_u8; TRANSFER_CHUNK_PIXELS];
            let mut visible = 0_usize;
            for ((i, pixel), rgba) in chunk_in.iter().enumerate().zip(chunk_out.iter_mut()) {
                let alpha = pixel[3].clamp(0.0, 1.0);
                #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                let alpha_code = (alpha * scale).round() as u16;
                // ADR-0014: alpha code 0 is transparent black (`oetf(0) == 0`).
                if alpha_code == 0 {
                    *rgba = [0, 0, 0, 0];
                } else {
                    rgba[3] = alpha_code;
                    // Clamp after unpremultiplication so finite working pixels
                    // whose `c / alpha` overflows `f32` to `±inf` still saturate
                    // to `0` or `max` identically to `quantize_row` and `f64`.
                    rgb_in[visible * 3] = (pixel[0] / alpha).clamp(0.0, 1.0);
                    rgb_in[visible * 3 + 1] = (pixel[1] / alpha).clamp(0.0, 1.0);
                    rgb_in[visible * 3 + 2] = (pixel[2] / alpha).clamp(0.0, 1.0);
                    #[allow(clippy::cast_possible_truncation)]
                    {
                        visible_idx[visible] = i as u8;
                    }
                    visible += 1;
                }
            }
            if visible == 0 {
                continue;
            }
            let visible_rgb = visible * 3;
            kernels
                .linear_to_srgb_u16(&rgb_in[..visible_rgb], max, &mut rgb_out[..visible_rgb])
                .map_err(kernel_error)?;
            let encoded_pixels = rgb_out[..visible_rgb].as_chunks::<3>().0;
            if visible == chunk_in.len() {
                for (encoded, rgba) in encoded_pixels.iter().zip(chunk_out.iter_mut()) {
                    rgba[0] = encoded[0];
                    rgba[1] = encoded[1];
                    rgba[2] = encoded[2];
                }
            } else {
                for (encoded, &out_idx) in encoded_pixels.iter().zip(&visible_idx[..visible]) {
                    let rgba = &mut chunk_out[usize::from(out_idx)];
                    rgba[0] = encoded[0];
                    rgba[1] = encoded[1];
                    rgba[2] = encoded[2];
                }
            }
        }
    }
    Ok(())
}

/// Quantize one row into `output` at its depth.
pub(crate) fn quantize_into<const C: usize>(
    acc: &[f32],
    output: &mut OutputBuffer,
    depth: u8,
    tables: &Tables,
    kernels: &Kernels,
) -> Result<(), Error> {
    match output {
        OutputBuffer::U8(bytes) => quantize_row::<C>(acc, bytes.as_mut_slice(), tables),
        OutputBuffer::U16(samples) => {
            quantize_row_deep::<C>(acc, samples.as_mut_slice(), depth, kernels)
        }
    }
}

/// Everything both executors need, derived once from the graph and source.
pub(crate) struct RowPlan {
    /// Sample type of the rows the source emits (before `conversion`).
    pub sample: SampleType,
    /// Output sample depth in bits (8: `u8` rows; 10 to 16: `u16` rows).
    pub depth: u8,
    /// A source conversion deferred to the executor (ADR-0023): applied to
    /// every needed row, on the worker that linearizes it.
    pub conversion: Option<Arc<dyn RowConversion>>,
    /// The decoder region the plan reads, once, top to bottom.
    pub source_region: Rect,
    pub source_y: u32,
    pub source_height: u32,
    pub footprint_y: u32,
    pub requested_y: u32,
    pub primaries: Primaries,
    pub p3: [[f32; 3]; 3],
    pub row_samples: usize,
    pub source_samples: usize,
    pub linear_samples: usize,
    pub output_rows: usize,
    pub column_taps: Taps,
    pub lines: TapTable,
    /// Source rows any output row needs, ascending; vertical taps refer to
    /// them by rank.
    pub needed: Vec<u32>,
    pub line_ranks: Vec<usize>,
    /// Output row `i` can be produced once source row `ready[i]` arrived.
    pub ready: Vec<u32>,
    /// Smallest source row any output row `>= i` still needs.
    pub future_min: Vec<u32>,
    /// Largest number of horizontal rows alive at once.
    pub capacity: usize,
}

fn primaries_from_color(color: &ColorEncoding) -> Result<Primaries, Error> {
    match color {
        ColorEncoding::Cicp([1, 13, 0, 1]) => Ok(Primaries::Srgb),
        ColorEncoding::Cicp([12, 13, 0, 1]) => Ok(Primaries::DisplayP3),
        ColorEncoding::Cicp([1, 8, 0, 1]) => Ok(Primaries::LinearSrgb),
        _ => Err(Error::UnsupportedColorEncoding),
    }
}

/// True when the resize stage is a 1:1 pixel-for-pixel identity on sRGB-encoded
/// samples with no deferred color conversion: every pixel maps directly from
/// its single source pixel without linearization, resampling or re-encoding.
fn is_identity_eligible(
    plan: &Prepared,
    requested: Rect,
    primaries: Primaries,
    conversion: Option<&Arc<dyn RowConversion>>,
) -> bool {
    conversion.is_none()
        && primaries == Primaries::Srgb
        && plan.input == plan.output
        && plan.footprint == plan.resize
        && plan.source.width == requested.width
        && plan.source.height == requested.height
}

impl RowPlan {
    fn from_prepared<const C: usize>(
        plan: Prepared,
        sample: SampleType,
        depth: u8,
        conversion: Option<Arc<dyn RowConversion>>,
        requested: Rect,
        primaries: Primaries,
    ) -> Result<Self, Error> {
        #[allow(clippy::cast_possible_truncation)]
        let p3 = p3_to_srgb_matrix().map(|row| row.map(|value| value as f32));

        let output_width = usize::try_from(requested.width).map_err(|_| Error::GeometryOverflow)?;
        let row_samples = output_width.checked_mul(4).ok_or(Error::GeometryOverflow)?;
        let source_width =
            usize::try_from(plan.source.width).map_err(|_| Error::GeometryOverflow)?;
        // Decoded rows carry `input_channels` samples per pixel (`C` unless a
        // deferred conversion expands gray 1/2 -> 3/4); linearized rows carry
        // four.
        let input_channels = conversion.as_ref().map_or(C, |conv| conv.input_channels());
        let source_samples = source_width
            .checked_mul(input_channels)
            .ok_or(Error::GeometryOverflow)?;
        let linear_samples = source_width.checked_mul(4).ok_or(Error::GeometryOverflow)?;

        let columns = TapTable::build(
            plan.input.width,
            plan.output.width,
            plan.resize.x,
            requested.width,
            plan.filter,
            plan.footprint.x,
        )?;
        if columns
            .index
            .iter()
            .any(|&x| usize::try_from(x).map_or(true, |x| x >= source_width))
        {
            return Err(Error::OutOfBounds);
        }
        let column_taps = columns.contiguous(source_width)?;
        let lines = TapTable::build(
            plan.input.height,
            plan.output.height,
            plan.resize.y,
            requested.height,
            plan.filter,
            0,
        )?;
        let output_rows = usize::try_from(requested.height).map_err(|_| Error::GeometryOverflow)?;

        let mut needed = try_vec(lines.index.len())?;
        needed.extend_from_slice(&lines.index);
        needed.sort_unstable();
        needed.dedup();
        let rank = |y: u32| {
            needed
                .binary_search(&y)
                .map_err(|_| Error::IncompleteDecoderOutput)
        };
        let mut line_ranks = try_vec(lines.index.len())?;
        for &y in &lines.index {
            line_ranks.push(rank(y)?);
        }

        // The running maximum of last taps keeps emission in order. A row is
        // dead once no remaining output row can reference it (Lanczos zero
        // crossings make the first tap nonmonotonic, see the reference).
        let mut ready = try_vec(output_rows)?;
        let mut running_max = 0_u32;
        for row in 0..output_rows {
            let (indices, _) = lines.taps(row);
            let last = *indices.iter().max().ok_or(Error::IncompleteDecoderOutput)?;
            running_max = running_max.max(last);
            ready.push(running_max);
        }
        let mut future_min = try_filled_vec(
            output_rows.checked_add(1).ok_or(Error::GeometryOverflow)?,
            u32::MAX,
        )?;
        for row in (0..output_rows).rev() {
            let (indices, _) = lines.taps(row);
            let first = *indices.iter().min().ok_or(Error::IncompleteDecoderOutput)?;
            future_min[row] = first.min(future_min[row + 1]);
        }
        let mut capacity = 1_usize;
        for row in 0..output_rows {
            capacity = capacity.max(rank(ready[row])? - rank(future_min[row])? + 1);
        }
        Ok(Self {
            sample,
            depth,
            conversion,
            source_region: plan.source,
            source_y: plan.source.y,
            source_height: plan.source.height,
            footprint_y: plan.footprint.y,
            requested_y: requested.y,
            primaries,
            p3,
            row_samples,
            source_samples,
            linear_samples,
            output_rows,
            column_taps,
            lines,
            needed,
            line_ranks,
            ready,
            future_min,
            capacity,
        })
    }

    /// Graph-space source row of decoder row `decoder_y`.
    pub(crate) fn semantic_y(&self, decoder_y: u32) -> Result<u32, Error> {
        decoder_y
            .checked_sub(self.source_y)
            .and_then(|n| n.checked_add(self.footprint_y))
            .ok_or(Error::GeometryOverflow)
    }

    pub(crate) fn output_y(&self, row: usize) -> Result<u32, Error> {
        u32::try_from(row)
            .ok()
            .and_then(|row| self.requested_y.checked_add(row))
            .ok_or(Error::GeometryOverflow)
    }

    /// Ranks and weights of output row `row`'s vertical taps.
    pub(crate) fn vertical_taps(&self, row: usize) -> (&[usize], &[f32]) {
        let range = self.lines.start[row]..self.lines.start[row + 1];
        (&self.line_ranks[range.clone()], &self.lines.weight[range])
    }
}

/// Below this many needed source rows, threads cost more than they save.
const PARALLEL_MIN_ROWS: usize = 64;

/// Partition `used_threads` between a transposing orientation gather and the
/// parallel row pipeline without nested oversubscription (issue #68, ADR-0019).
///
/// Only splits off gather threads when `OrientedSource` will actually spawn a
/// gather pool (`has_transposing_orientation`, `source_height >= PARALLEL_MIN_ROWS`,
/// and `gather >= 2`, i.e. `used_threads >= 6`). Whenever gathering stays
/// serial (`gather == 1`), all `used_threads` go to `parallel_execute` so
/// `ProcessingReport::transform_threads` reflects the actual worker count.
pub(crate) fn partition_transform_threads(
    used_threads: usize,
    has_transposing_orientation: bool,
    source_height: u32,
) -> (usize, usize) {
    if has_transposing_orientation
        && source_height >= crate::orientation::PARALLEL_MIN_ROWS
        && used_threads >= 6
    {
        let gather = (used_threads / 3).clamp(2, 4);
        (gather, used_threads - gather)
    } else {
        (1, used_threads)
    }
}

#[allow(clippy::too_many_arguments)] // Graph binding, budget, kernels, threads, depth and sink.
fn run<const C: usize>(
    graph: &Graph,
    target: NodeId,
    bound_source: NodeId,
    source: &mut dyn DecodeSource,
    requested: Rect,
    budget: &MemoryBudget,
    kernels: &Kernels,
    threads: usize,
    depth: u8,
    sink: impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
) -> Result<QualityReport, Error> {
    let prepared = prepare(graph, target, bound_source, source, requested)?;
    let info = source.info().clone();
    let color = info.semantics().color;
    let primaries = primaries_from_color(&color)?;
    // Take over the source's per-row conversion (the ICC transform) so that
    // it runs where the rows are linearized, on the workers when parallel.
    let conversion = source.defer_conversion();
    let sample = conversion
        .as_ref()
        .map_or(info.sample, |conv| conv.input_sample());

    if is_identity_eligible(&prepared, requested, primaries, conversion.as_ref()) {
        return run_identity::<C>(&prepared, sample, requested, source, budget, depth, sink);
    }

    let _plan_reservation = reserve_resize_plan(graph, target, requested, budget)?;
    let plan =
        RowPlan::from_prepared::<C>(prepared, sample, depth, conversion, requested, primaries)?;
    let used_threads = if threads > 1 && plan.needed.len() >= PARALLEL_MIN_ROWS {
        threads
    } else {
        1
    };
    let (report, source_time, transform_time, worker_threads) = if used_threads > 1 {
        let (gather_threads, worker_threads) = partition_transform_threads(
            used_threads,
            source.has_transposing_orientation(),
            plan.source_height,
        );
        source.set_orientation_threads(gather_threads);
        let (report, source_time, transform_time) = crate::parallel_execute::run::<C>(
            &plan,
            source,
            budget,
            kernels,
            worker_threads,
            sink,
        )?;
        (report, source_time, transform_time, worker_threads)
    } else {
        let (report, source_time, transform_time) =
            run_sequential::<C>(&plan, source, budget, kernels, sink)?;
        (report, source_time, transform_time, 1)
    };
    Ok(QualityReport {
        decoder: report,
        output_rows: u64::try_from(plan.output_rows).map_err(|_| Error::GeometryOverflow)?,
        decoder_requests: 1,
        peak_engine_managed_bytes: budget.snapshot().peak,
        explicit_full_frame_intermediates: 0,
        transform_threads: worker_threads,
        used_kernels: true,
        source_time,
        transform_time,
    })
}

/// Convert one 1:1 sRGB source row straight to the output row representation
/// without linearization, resampling or re-encoding. Preserves ADR-0014
/// transparent black (zero RGB when output alpha code is 0) and float sample
/// validation (`InvalidAlphaValue`, `NonFinitePixelSample`).
#[allow(clippy::too_many_lines)] // Four (input sample, output depth) arms kept explicit.
fn identity_row<const C: usize>(
    row: DecodedRow<'_>,
    output: &mut OutputBuffer,
    depth: u8,
    u8_to_deep: &[u16; 256],
) -> Result<(), Error> {
    match (row, output) {
        (DecodedRow::U8(src), OutputBuffer::U8(dst)) => {
            let dst = dst.as_mut_slice();
            if C == 3 {
                for (pixel, out) in src
                    .as_chunks::<3>()
                    .0
                    .iter()
                    .zip(dst.as_chunks_mut::<4>().0)
                {
                    *out = [pixel[0], pixel[1], pixel[2], u8::MAX];
                }
            } else {
                for (pixel, out) in src
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .zip(dst.as_chunks_mut::<4>().0)
                {
                    // ADR-0014: alpha byte 0 is transparent black.
                    let mask = u8::from(pixel[3] != 0).wrapping_neg();
                    *out = [pixel[0] & mask, pixel[1] & mask, pixel[2] & mask, pixel[3]];
                }
            }
            Ok(())
        }
        (DecodedRow::U8(src), OutputBuffer::U16(dst)) => {
            let max = max_code(depth);
            let dst = dst.as_mut_slice();
            if C == 3 {
                for (pixel, out) in src
                    .as_chunks::<3>()
                    .0
                    .iter()
                    .zip(dst.as_chunks_mut::<4>().0)
                {
                    *out = [
                        u8_to_deep[usize::from(pixel[0])],
                        u8_to_deep[usize::from(pixel[1])],
                        u8_to_deep[usize::from(pixel[2])],
                        max,
                    ];
                }
            } else {
                for (pixel, out) in src
                    .as_chunks::<4>()
                    .0
                    .iter()
                    .zip(dst.as_chunks_mut::<4>().0)
                {
                    // ADR-0014: alpha code 0 is transparent black. For every
                    // depth >= 8, `u8_to_deep[a] == 0` iff `a == 0`.
                    *out = if pixel[3] == 0 {
                        [0, 0, 0, 0]
                    } else {
                        [
                            u8_to_deep[usize::from(pixel[0])],
                            u8_to_deep[usize::from(pixel[1])],
                            u8_to_deep[usize::from(pixel[2])],
                            u8_to_deep[usize::from(pixel[3])],
                        ]
                    };
                }
            }
            Ok(())
        }
        (DecodedRow::F32(src), OutputBuffer::U8(dst)) => {
            let dst = dst.as_mut_slice();
            for (pixel, out) in src
                .as_chunks::<C>()
                .0
                .iter()
                .zip(dst.as_chunks_mut::<4>().0)
            {
                if C == 4 {
                    let alpha = pixel[3];
                    if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
                        return Err(Error::InvalidAlphaValue);
                    }
                    if !pixel[0].is_finite() || !pixel[1].is_finite() || !pixel[2].is_finite() {
                        return Err(Error::NonFinitePixelSample);
                    }
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let alpha_code = (f64::from(alpha) * 255.0).round() as u8;
                    if alpha_code == 0 {
                        *out = [0, 0, 0, 0];
                    } else {
                        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                        {
                            *out = [
                                (f64::from(pixel[0].clamp(0.0, 1.0)) * 255.0).round() as u8,
                                (f64::from(pixel[1].clamp(0.0, 1.0)) * 255.0).round() as u8,
                                (f64::from(pixel[2].clamp(0.0, 1.0)) * 255.0).round() as u8,
                                alpha_code,
                            ];
                        }
                    }
                } else {
                    if !pixel[0].is_finite() || !pixel[1].is_finite() || !pixel[2].is_finite() {
                        return Err(Error::NonFinitePixelSample);
                    }
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    {
                        *out = [
                            (f64::from(pixel[0].clamp(0.0, 1.0)) * 255.0).round() as u8,
                            (f64::from(pixel[1].clamp(0.0, 1.0)) * 255.0).round() as u8,
                            (f64::from(pixel[2].clamp(0.0, 1.0)) * 255.0).round() as u8,
                            u8::MAX,
                        ];
                    }
                }
            }
            Ok(())
        }
        (DecodedRow::F32(src), OutputBuffer::U16(dst)) => {
            let max = max_code(depth);
            let scale = f64::from(max);
            let dst = dst.as_mut_slice();
            for (pixel, out) in src
                .as_chunks::<C>()
                .0
                .iter()
                .zip(dst.as_chunks_mut::<4>().0)
            {
                if C == 4 {
                    let alpha = pixel[3];
                    if !alpha.is_finite() || !(0.0..=1.0).contains(&alpha) {
                        return Err(Error::InvalidAlphaValue);
                    }
                    if !pixel[0].is_finite() || !pixel[1].is_finite() || !pixel[2].is_finite() {
                        return Err(Error::NonFinitePixelSample);
                    }
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    let alpha_code = (f64::from(alpha) * scale).round() as u16;
                    if alpha_code == 0 {
                        *out = [0, 0, 0, 0];
                    } else {
                        #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                        {
                            *out = [
                                (f64::from(pixel[0].clamp(0.0, 1.0)) * scale).round() as u16,
                                (f64::from(pixel[1].clamp(0.0, 1.0)) * scale).round() as u16,
                                (f64::from(pixel[2].clamp(0.0, 1.0)) * scale).round() as u16,
                                alpha_code,
                            ];
                        }
                    }
                } else {
                    if !pixel[0].is_finite() || !pixel[1].is_finite() || !pixel[2].is_finite() {
                        return Err(Error::NonFinitePixelSample);
                    }
                    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
                    {
                        *out = [
                            (f64::from(pixel[0].clamp(0.0, 1.0)) * scale).round() as u16,
                            (f64::from(pixel[1].clamp(0.0, 1.0)) * scale).round() as u16,
                            (f64::from(pixel[2].clamp(0.0, 1.0)) * scale).round() as u16,
                            max,
                        ];
                    }
                }
            }
            Ok(())
        }
    }
}

/// 1:1 sRGB pass-through: streams rows straight from the decoder to the sink
/// with only one output row buffer and no tap tables, linearization or ring
/// buffer. Byte-identical across thread counts.
fn run_identity<const C: usize>(
    plan: &Prepared,
    sample: SampleType,
    requested: Rect,
    source: &mut dyn DecodeSource,
    budget: &MemoryBudget,
    depth: u8,
    mut sink: impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
) -> Result<QualityReport, Error> {
    let output_width = usize::try_from(requested.width).map_err(|_| Error::GeometryOverflow)?;
    let row_samples = output_width.checked_mul(4).ok_or(Error::GeometryOverflow)?;
    let source_samples = output_width.checked_mul(C).ok_or(Error::GeometryOverflow)?;
    let output_rows = usize::try_from(requested.height).map_err(|_| Error::GeometryOverflow)?;

    let mut output = OutputBuffer::allocate(depth, row_samples, budget)?;
    let u8_to_deep: [u16; 256] = if sample == SampleType::U8 && depth > EIGHT {
        let max = f64::from(max_code(depth));
        std::array::from_fn(|code| {
            #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
            {
                ((f64::from(u8::try_from(code).unwrap_or(u8::MAX)) / 255.0) * max).round() as u16
            }
        })
    } else {
        [0; 256]
    };

    let mut emitted = 0_usize;
    let mut decoded_rows = 0_u32;
    let mut expected_y = plan.source.y;
    let mut transform_time = Duration::ZERO;
    let read_started = Instant::now();
    let report = source.read_region(plan.source, budget, &mut |decoder_y, row| {
        let row_started = Instant::now();
        if emitted >= output_rows
            || decoder_y != expected_y
            || row.len() != source_samples
            || row.sample_type() != sample
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        decoded_rows += 1;
        expected_y = expected_y.checked_add(1).ok_or(Error::GeometryOverflow)?;

        identity_row::<C>(row, &mut output, depth, &u8_to_deep)?;
        let out_y = u32::try_from(emitted)
            .ok()
            .and_then(|offset| requested.y.checked_add(offset))
            .ok_or(Error::GeometryOverflow)?;
        sink(out_y, output.row())?;
        emitted += 1;
        transform_time += row_started.elapsed();
        Ok(())
    })?;
    let source_time = read_started.elapsed().saturating_sub(transform_time);

    if decoded_rows != plan.source.height
        || emitted != output_rows
        || report.rows != u64::from(decoded_rows)
    {
        return Err(Error::IncompleteDecoderOutput);
    }
    Ok(QualityReport {
        decoder: report,
        output_rows: u64::try_from(output_rows).map_err(|_| Error::GeometryOverflow)?,
        decoder_requests: 1,
        peak_engine_managed_bytes: budget.snapshot().peak,
        explicit_full_frame_intermediates: 0,
        transform_threads: 1,
        used_kernels: false,
        source_time,
        transform_time,
    })
}

/// Borrowed view over the sequential horizontal ring buffer for vertical tap
/// lookup.
#[derive(Clone, Copy)]
struct RingView<'a> {
    samples: &'a [f32],
    ranks: &'a [usize],
    capacity: usize,
    row_samples: usize,
}

impl<'a> RingView<'a> {
    #[inline]
    fn row(self, row_rank: usize) -> Result<&'a [f32], Error> {
        let slot = row_rank % self.capacity;
        if self.ranks[slot] != row_rank {
            return Err(Error::IncompleteDecoderOutput);
        }
        Ok(&self.samples[slot * self.row_samples..(slot + 1) * self.row_samples])
    }
}

/// Accumulate a `> 4`-tap vertical filter from the horizontal ring buffer in
/// fixed-size stack chunks (`VERTICAL_STACK_TAPS = 64`), keeping the 1 KiB
/// slice array off the `0..=4`-tap hot path.
fn vertical_ring_chunked(
    kernels: &Kernels,
    ring: RingView<'_>,
    ranks: &[usize],
    weights: &[f32],
    acc: &mut [f32],
) -> Result<(), Error> {
    let mut stack_rows: [&[f32]; VERTICAL_STACK_TAPS] = [&[]; VERTICAL_STACK_TAPS];
    for (chunk_idx, (rank_chunk, weight_chunk)) in ranks
        .chunks(VERTICAL_STACK_TAPS)
        .zip(weights.chunks(VERTICAL_STACK_TAPS))
        .enumerate()
    {
        for (dst, &row_rank) in stack_rows.iter_mut().zip(rank_chunk) {
            *dst = ring.row(row_rank)?;
        }
        let slice = &stack_rows[..rank_chunk.len()];
        if chunk_idx == 0 {
            kernels
                .vertical_rgba(slice, weight_chunk, acc)
                .map_err(kernel_error)?;
        } else {
            kernels
                .vertical_rgba_add(slice, weight_chunk, acc)
                .map_err(kernel_error)?;
        }
    }
    Ok(())
}

/// One thread: linearize and filter each needed row on arrival, keep
/// horizontal rows in a ring addressed by rank, emit output rows in order.
fn run_sequential<const C: usize>(
    plan: &RowPlan,
    source: &mut dyn DecodeSource,
    budget: &MemoryBudget,
    kernels: &Kernels,
    mut sink: impl FnMut(u32, OutputRow<'_>) -> Result<(), Error>,
) -> Result<(DecodeReport, Duration, Duration), Error> {
    let tables = tables();
    let capacity = plan.capacity;
    let row_samples = plan.row_samples;
    let mut linear = budget.allocate_typed::<f32>(plan.linear_samples)?;
    let mut conversion = ConversionScratch::new(plan, budget)?;
    let mut ring = budget.allocate_typed::<f32>(
        capacity
            .checked_mul(row_samples)
            .ok_or(Error::GeometryOverflow)?,
    )?;
    let mut ring_rank = try_filled_vec(capacity, usize::MAX)?;
    let mut accumulator = budget.allocate_typed::<f32>(row_samples)?;
    let mut output = OutputBuffer::allocate(plan.depth, row_samples, budget)?;

    let mut emitted = 0_usize;
    let mut decoded_rows = 0_u32;
    let mut expected_y = plan.source_y;
    let mut transform_time = Duration::ZERO;
    let read_started = Instant::now();
    let report = source.read_region(plan.source_region, budget, &mut |decoder_y, row| {
        let row_started = Instant::now();
        if decoder_y != expected_y
            || row.len() != plan.source_samples
            || row.sample_type() != plan.sample
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        decoded_rows += 1;
        expected_y = expected_y.checked_add(1).ok_or(Error::GeometryOverflow)?;
        let semantic_y = plan.semantic_y(decoder_y)?;

        if let Ok(row_rank) = plan.needed.binary_search(&semantic_y) {
            source_row_to_linear::<C>(plan, row, &mut conversion, kernels, linear.as_mut_slice())?;
            let slot = row_rank % capacity;
            ring_rank[slot] = row_rank;
            let target = &mut ring.as_mut_slice()[slot * row_samples..(slot + 1) * row_samples];
            kernels
                .horizontal_rgba(linear.as_slice(), &plan.column_taps, target)
                .map_err(kernel_error)?;
        }

        while emitted < plan.output_rows && plan.ready[emitted] <= semantic_y {
            let acc = accumulator.as_mut_slice();
            let (ranks, weights) = plan.vertical_taps(emitted);
            let ring_view = RingView {
                samples: ring.as_slice(),
                ranks: &ring_rank,
                capacity,
                row_samples,
            };
            match ranks {
                [] => kernels.vertical_rgba(&[], &[], acc).map_err(kernel_error)?,
                &[r0] => kernels
                    .vertical_rgba(&[ring_view.row(r0)?], weights, acc)
                    .map_err(kernel_error)?,
                &[r0, r1] => kernels
                    .vertical_rgba(&[ring_view.row(r0)?, ring_view.row(r1)?], weights, acc)
                    .map_err(kernel_error)?,
                &[r0, r1, r2] => kernels
                    .vertical_rgba(
                        &[ring_view.row(r0)?, ring_view.row(r1)?, ring_view.row(r2)?],
                        weights,
                        acc,
                    )
                    .map_err(kernel_error)?,
                &[r0, r1, r2, r3] => kernels
                    .vertical_rgba(
                        &[
                            ring_view.row(r0)?,
                            ring_view.row(r1)?,
                            ring_view.row(r2)?,
                            ring_view.row(r3)?,
                        ],
                        weights,
                        acc,
                    )
                    .map_err(kernel_error)?,
                _ => vertical_ring_chunked(kernels, ring_view, ranks, weights, acc)?,
            }
            quantize_into::<C>(acc, &mut output, plan.depth, tables, kernels)?;
            sink(plan.output_y(emitted)?, output.row())?;
            emitted += 1;
        }
        transform_time += row_started.elapsed();
        Ok(())
    })?;
    let source_time = read_started.elapsed().saturating_sub(transform_time);

    if decoded_rows != plan.source_height
        || emitted != plan.output_rows
        || report.rows != u64::from(decoded_rows)
    {
        return Err(Error::IncompleteDecoderOutput);
    }
    Ok((report, source_time, transform_time))
}
