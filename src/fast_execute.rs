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
//! - the horizontal and vertical resample run on `ferrixel-kernels`
//!   (runtime-dispatched NEON or AVX2, ADR-0020), on four `f32` channels
//!   per pixel (the fourth is unused for opaque RGB);
//! - output samples are quantized by an exact `f32` to sRGB8 encoder
//!   (bucket table plus thresholds) instead of `powf`.
//!
//! The reference executor stays compiled and is the oracle of the
//! differential tests in `fast_execute_tests`. Memory accounting, the single
//! decoder request, cancellation and the zero-full-frame contract are the
//! same as in the reference.

use std::sync::OnceLock;
use std::time::{Duration, Instant};

use ferrixel_kernels::{KernelError, Kernels, Taps};

use crate::color::{AlphaMode, ColorEncoding};
use crate::decoder::{ChannelLayout, DecodeSource, DecodedRow};
use crate::error::Error;
use crate::geometry::Rect;
use crate::graph::{Graph, NodeId};
use crate::memory::MemoryBudget;
use crate::quality::taps;
use crate::quality_execute::{QualityReport, prepare};
use crate::working_color::{
    decode_srgb, decode_straight_to_premultiplied_linear_srgb, decode_to_linear_srgb,
    encode_linear_srgb8, p3_to_srgb_matrix,
};

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
enum Primaries {
    Srgb,
    DisplayP3,
}

/// Flat tap tables: taps of output position `i` are
/// `index[start[i]..start[i + 1]]` with matching `weight`s.
struct TapTable {
    start: Vec<usize>,
    index: Vec<u32>,
    weight: Vec<f32>,
}

impl TapTable {
    fn build(
        input: u32,
        output: u32,
        first: u32,
        count: u32,
        filter: crate::quality::Filter,
        origin: u32,
    ) -> Result<Self, Error> {
        let mut table = Self {
            start: Vec::with_capacity(usize::try_from(count).unwrap_or(0) + 1),
            index: Vec::new(),
            weight: Vec::new(),
        };
        table.start.push(0);
        for offset in 0..count {
            let at = first.checked_add(offset).ok_or(Error::GeometryOverflow)?;
            for tap in taps(input, output, at, filter)? {
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
        let mut starts = Vec::with_capacity(outputs);
        let mut offsets = Vec::with_capacity(outputs + 1);
        let mut weights = Vec::with_capacity(self.weight.len());
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

fn kernel_error(error: KernelError) -> Error {
    match error {
        KernelError::InvalidTaps => Error::OutOfBounds,
        KernelError::LengthMismatch | KernelError::UnsupportedLevel(_) => {
            Error::IncompleteDecoderOutput
        }
    }
}

/// Fast `f32` counterpart of `quality_execute::execute_quality_srgb8`, with
/// the same graph contract, sink contract and report. The resample runs on
/// `kernels` (ADR-0020); every pixel is stored as four `f32` channels.
#[allow(clippy::too_many_arguments)] // The reference signature plus the kernel table.
pub(crate) fn execute_fast_srgb8(
    graph: &Graph,
    target: NodeId,
    bound_source: NodeId,
    source: &mut dyn DecodeSource,
    requested: Rect,
    budget: &MemoryBudget,
    kernels: &Kernels,
    sink: impl FnMut(u32, &[u8]) -> Result<(), Error>,
) -> Result<QualityReport, Error> {
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
            sink,
        ),
        (ChannelLayout::Rgb | ChannelLayout::Rgba, _) => {
            // Same order of checks as the reference: binding first.
            prepare(graph, target, bound_source, source, requested)?;
            Err(Error::UnsupportedAlphaMode)
        }
        _ => {
            prepare(graph, target, bound_source, source, requested)?;
            Err(Error::UnsupportedPixelOperation)
        }
    }
}

/// Decode one source row to linear-light (premultiplied for `C == 4`) `f32`
/// samples, once per sample, four values per pixel (the fourth is 0 for
/// opaque RGB).
fn linearize_row<const C: usize>(
    row: DecodedRow<'_>,
    primaries: Primaries,
    color: &ColorEncoding,
    p3: &[[f32; 3]; 3],
    out: &mut [f32],
) -> Result<(), Error> {
    let lut = &tables().decode_u8;
    match row {
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
            // Float input (JPEG XL) has no table; the reference transfer and
            // matrix run once per pixel, still not once per tap.
            for (pixel, linear) in samples
                .as_chunks::<C>()
                .0
                .iter()
                .zip(out.as_chunks_mut::<4>().0)
            {
                let rgb = [
                    f64::from(pixel[0]),
                    f64::from(pixel[1]),
                    f64::from(pixel[2]),
                ];
                #[allow(clippy::cast_possible_truncation)]
                if C == 4 {
                    let premultiplied = decode_straight_to_premultiplied_linear_srgb(
                        color,
                        rgb,
                        f64::from(pixel[3]),
                    )?;
                    for (out, value) in linear.iter_mut().zip(premultiplied.rgb) {
                        *out = value as f32;
                    }
                    linear[3] = premultiplied.alpha as f32;
                } else {
                    let decoded = decode_to_linear_srgb(color, rgb)?;
                    for (out, value) in linear.iter_mut().zip(decoded.rgb) {
                        *out = value as f32;
                    }
                    linear[3] = 0.0;
                }
            }
        }
    }
    Ok(())
}

#[allow(clippy::too_many_lines, clippy::too_many_arguments)] // Planning, ring sizing and the row loop stay together.
fn run<const C: usize>(
    graph: &Graph,
    target: NodeId,
    bound_source: NodeId,
    source: &mut dyn DecodeSource,
    requested: Rect,
    budget: &MemoryBudget,
    kernels: &Kernels,
    mut sink: impl FnMut(u32, &[u8]) -> Result<(), Error>,
) -> Result<QualityReport, Error> {
    let info = source.info().clone();
    let plan = prepare(graph, target, bound_source, source, requested)?;
    let color = info.semantics().color;
    let primaries = match color {
        ColorEncoding::Cicp([1, 13, 0, 1]) => Primaries::Srgb,
        ColorEncoding::Cicp([12, 13, 0, 1]) => Primaries::DisplayP3,
        _ => return Err(Error::UnsupportedColorEncoding),
    };
    #[allow(clippy::cast_possible_truncation)]
    let p3 = p3_to_srgb_matrix().map(|row| row.map(|value| value as f32));
    let tables = tables();

    let output_width = usize::try_from(requested.width).map_err(|_| Error::GeometryOverflow)?;
    let row_samples = output_width.checked_mul(4).ok_or(Error::GeometryOverflow)?;
    let source_width = usize::try_from(plan.source.width).map_err(|_| Error::GeometryOverflow)?;
    // Decoded rows carry `C` samples per pixel; linearized rows carry four.
    let source_samples = source_width.checked_mul(C).ok_or(Error::GeometryOverflow)?;
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

    // Source rows any output row needs, in ascending order; vertical taps are
    // stored as ranks into this list.
    let mut needed = lines.index.clone();
    needed.sort_unstable();
    needed.dedup();
    let rank = |y: u32| {
        needed
            .binary_search(&y)
            .map_err(|_| Error::IncompleteDecoderOutput)
    };
    let line_ranks = lines
        .index
        .iter()
        .map(|&y| rank(y))
        .collect::<Result<Vec<_>, _>>()?;

    // Output row i can be produced once source row ready[i] has arrived:
    // the running maximum of last taps keeps emission in order. A row is
    // dead once no remaining output row can reference it (Lanczos zero
    // crossings make the first tap nonmonotonic, see the reference).
    let mut ready = Vec::with_capacity(output_rows);
    let mut running_max = 0_u32;
    for row in 0..output_rows {
        let (indices, _) = lines.taps(row);
        let last = *indices.iter().max().ok_or(Error::IncompleteDecoderOutput)?;
        running_max = running_max.max(last);
        ready.push(running_max);
    }
    let mut future_min = vec![u32::MAX; output_rows + 1];
    for row in (0..output_rows).rev() {
        let (indices, _) = lines.taps(row);
        let first = *indices.iter().min().ok_or(Error::IncompleteDecoderOutput)?;
        future_min[row] = first.min(future_min[row + 1]);
    }
    // Rows alive while output row i is produced: ranks rank(future_min[i])
    // through rank(ready[i]). The largest such window sizes the ring.
    let mut capacity = 1_usize;
    for row in 0..output_rows {
        capacity = capacity.max(rank(ready[row])? - rank(future_min[row])? + 1);
    }

    let mut linear = budget.allocate_typed::<f32>(linear_samples)?;
    let mut ring = budget.allocate_typed::<f32>(
        capacity
            .checked_mul(row_samples)
            .ok_or(Error::GeometryOverflow)?,
    )?;
    let mut ring_rank = vec![usize::MAX; capacity];
    let mut accumulator = budget.allocate_typed::<f32>(row_samples)?;
    let mut output =
        budget.allocate_typed::<u8>(output_width.checked_mul(4).ok_or(Error::GeometryOverflow)?)?;

    let mut emitted = 0_usize;
    let mut decoded_rows = 0_u32;
    let mut expected_y = plan.source.y;
    let mut transform_time = Duration::ZERO;
    let read_started = Instant::now();
    let report = source.read_region(plan.source, budget, &mut |decoder_y, row| {
        let row_started = Instant::now();
        if decoder_y != expected_y
            || row.len() != source_samples
            || row.sample_type() != info.sample
        {
            return Err(Error::IncompleteDecoderOutput);
        }
        decoded_rows += 1;
        expected_y = expected_y.checked_add(1).ok_or(Error::GeometryOverflow)?;
        let semantic_y = decoder_y
            .checked_sub(plan.source.y)
            .and_then(|n| n.checked_add(plan.footprint.y))
            .ok_or(Error::GeometryOverflow)?;

        if let Ok(row_rank) = needed.binary_search(&semantic_y) {
            linearize_row::<C>(row, primaries, &color, &p3, linear.as_mut_slice())?;
            let slot = row_rank % capacity;
            ring_rank[slot] = row_rank;
            let target = &mut ring.as_mut_slice()[slot * row_samples..(slot + 1) * row_samples];
            kernels
                .horizontal_rgba(linear.as_slice(), &column_taps, target)
                .map_err(kernel_error)?;
        }

        while emitted < output_rows && ready[emitted] <= semantic_y {
            let acc = accumulator.as_mut_slice();
            acc.fill(0.0);
            let range = lines.start[emitted]..lines.start[emitted + 1];
            for (&row_rank, &weight) in line_ranks[range.clone()].iter().zip(&lines.weight[range]) {
                let slot = row_rank % capacity;
                if ring_rank[slot] != row_rank {
                    return Err(Error::IncompleteDecoderOutput);
                }
                let cached = &ring.as_slice()[slot * row_samples..(slot + 1) * row_samples];
                kernels.axpy(acc, cached, weight).map_err(kernel_error)?;
            }
            for (pixel, rgba) in acc
                .as_chunks::<4>()
                .0
                .iter()
                .zip(output.as_mut_slice().as_chunks_mut::<4>().0)
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
            let out_y =
                requested.y + u32::try_from(emitted).map_err(|_| Error::GeometryOverflow)?;
            sink(out_y, output.as_slice())?;
            emitted += 1;
        }
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
        output_rows: u64::try_from(emitted).map_err(|_| Error::GeometryOverflow)?,
        decoder_requests: 1,
        peak_engine_managed_bytes: budget.snapshot().peak,
        explicit_full_frame_intermediates: 0,
        source_time,
        transform_time,
    })
}
