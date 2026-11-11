//! M1 reference executor: decoder -> color transform -> antialiased
//! separable resampling -> explicit SDR output quantization.
//! A single source region is decoded only once, including JPEG's codec-level
//! ROI request. The reusable output buffer and cached *horizontal* source rows
//! are charged to the shared managed-memory budget. The final encoder may
//! separately require an entire output frame; this executor does not.

use std::collections::VecDeque;
use std::time::{Duration, Instant};

use crate::color::{AlphaMode, ColorEncoding};
use crate::decoder::{ChannelLayout, DecodeReport, DecodeSource, DecodedRow, SampleType};
use crate::error::Error;
use crate::geometry::Rect;
use crate::graph::{Graph, NodeId, NodeKind, ResizeDomain};
use crate::memory::{ExternalReservation, ManagedBuffer, MemoryBudget, try_filled_vec, try_vec};
use crate::quality::{Filter, Tap, taps};
use crate::surface::PixelFormat;
use crate::working_color::{
    decode_straight_to_premultiplied_linear_srgb, decode_to_linear_srgb, encode_linear_srgb8,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) struct QualityReport {
    pub decoder: DecodeReport,
    pub output_rows: u64,
    pub decoder_requests: u32,
    pub peak_engine_managed_bytes: usize,
    pub explicit_full_frame_intermediates: u32,
    /// Wall-clock time inside the source's region reader, excluding the
    /// per-row transform callback: codec decode plus any source wrapper work
    /// such as reorientation or ICC conversion.
    pub source_time: Duration,
    /// Wall-clock time spent in the per-row transform callback: transfer
    /// decode, resampling, quantization and the caller's row sink.
    pub transform_time: Duration,
}

struct CachedRow {
    y: u32,
    samples: ManagedBuffer<f64>,
}

// Charge all components together although the f64 and f32 executors do
// not run concurrently. Flat tables reserve at most the entry bound and
// contiguous SIMD weights reserve their exact span, so neither grows while
// populated. The extra tap copy covers footprint/construction scratch.
const TAP_STORAGE: usize = 2 * std::mem::size_of::<Tap>();
const FLAT_AND_SIMD: usize =
    2 * (std::mem::size_of::<u32>() + std::mem::size_of::<f32>()) + std::mem::size_of::<f32>();
const RANK_AND_NEEDED: usize = 2 * std::mem::size_of::<usize>() + 2 * std::mem::size_of::<u32>();
// Leave a second descriptor per needed row for queue capacity rounding.
const REFERENCE_CACHE: usize = 2 * std::mem::size_of::<CachedRow>();
// Allow a second descriptor-sized margin for Arc bookkeeping. Rust does
// not promise Arc's layout; allocator and runtime internals remain outside
// this charge, as do the separately budgeted pixel payloads.
const SHARED_ROW: usize =
    2 * (std::mem::size_of::<ManagedBuffer<f32>>() + 2 * std::mem::size_of::<usize>());
const PARALLEL_REFERENCES: usize =
    std::mem::size_of::<Option<std::sync::Arc<ManagedBuffer<f32>>>>()
        + std::mem::size_of::<(std::sync::Arc<ManagedBuffer<f32>>, f32)>();
const ENTRY_BYTES: usize = TAP_STORAGE
    + FLAT_AND_SIMD
    + RANK_AND_NEEDED
    + REFERENCE_CACHE
    + SHARED_ROW
    + PARALLEL_REFERENCES;
// Outer f64 tap vectors, f32 table offsets, SIMD offsets and
// ready/future-min arrays are bounded by output positions.
const POSITION_BYTES: usize = std::mem::size_of::<Vec<Tap>>()
    + 2 * std::mem::size_of::<usize>()
    + 4 * std::mem::size_of::<u32>();

/// The validated chain shared by the `f64` reference executor and the fast
/// executor (`fast_execute`): decoder region, resize input footprint, resize
/// output region, resize input/output sizes and filter.
#[derive(Clone, Copy)]
pub(crate) struct Prepared {
    pub source: Rect,
    pub footprint: Rect,
    pub resize: Rect,
    pub input: crate::geometry::Size,
    pub output: crate::geometry::Size,
    pub filter: Filter,
}

pub(crate) fn prepare(
    graph: &Graph,
    target: NodeId,
    source_node: NodeId,
    decoder: &dyn DecodeSource,
    request: Rect,
) -> Result<Prepared, Error> {
    let stages = crate::plan::plan_region(graph, target, request)?;
    let first = stages.first().ok_or(Error::UnknownNode)?;
    let source = graph.node(first.node)?;
    let info = decoder.info();
    if first.node != source_node
        || source.kind != NodeKind::DecoderSource
        || source.size != info.size
        || source.format != info.pixel_format()
        || source.semantics != info.semantics()
    {
        return Err(Error::SourceBindingMismatch);
    }
    if !matches!(info.layout, ChannelLayout::Rgb | ChannelLayout::Rgba) {
        return Err(Error::UnsupportedPixelOperation);
    }
    if !matches!(info.sample, SampleType::U8 | SampleType::F32) {
        return Err(Error::UnsupportedPixelOperation);
    }

    let mut linearized = false;
    let mut resized = None;
    let mut quantized = false;
    for (index, step) in stages.iter().enumerate().skip(1) {
        match graph.node(step.node)?.kind {
            NodeKind::Crop { .. } => {}
            NodeKind::ToLinearSrgb { .. } if !linearized && resized.is_none() => {
                linearized = true;
            }
            NodeKind::Resize {
                domain: ResizeDomain::WorkingLinear(filter),
                ..
            } if linearized && resized.is_none() => {
                resized = Some((index, filter));
            }
            NodeKind::ToSrgb8 { .. } if resized.is_some() && !quantized => {
                quantized = true;
            }
            _ => return Err(Error::UnsupportedPixelOperation),
        }
    }
    let (index, filter) = resized.ok_or(Error::UnsupportedPixelOperation)?;
    if !linearized || !quantized {
        return Err(Error::UnsupportedPixelOperation);
    }
    if graph.node(target)?.format != PixelFormat::InterleavedU8(info.layout)
        || graph.node(target)?.semantics.color != ColorEncoding::Cicp([1, 13, 0, 1])
    {
        return Err(Error::PixelFormatMismatch);
    }
    let resize = stages[index];
    let foot = stages[index - 1];
    Ok(Prepared {
        source: first.region,
        footprint: foot.region,
        resize: resize.region,
        input: graph.node(foot.node)?.size,
        output: graph.node(resize.node)?.size,
        filter,
    })
}

/// Reserve a conservative upper bound for resize tap and scheduling storage
/// before `plan_region` constructs even its first temporary tap vector.
/// Retaining the reservation through execution also covers the row-plan
/// metadata that lives alongside the pixel buffers. Allocator overhead,
/// thread stacks, graph nodes, channels and codec memory are excluded.
pub(crate) fn reserve_resize_plan(
    graph: &Graph,
    target: NodeId,
    request: Rect,
    budget: &MemoryBudget,
) -> Result<ExternalReservation, Error> {
    if !request.fits(graph.node(target)?.size) {
        return Err(Error::OutOfBounds);
    }
    let mut current = target;
    let mut resize = None;
    loop {
        let node = graph.node(current)?;
        current = match node.kind {
            NodeKind::ToLinearSrgb { input }
            | NodeKind::ToSrgb8 { input }
            | NodeKind::Crop { input, .. } => input,
            NodeKind::Resize {
                input,
                domain: ResizeDomain::WorkingLinear(filter),
            } => {
                if resize.is_some() {
                    return Err(Error::UnsupportedPixelOperation);
                }
                resize = Some((graph.node(input)?.size, node.size, filter));
                input
            }
            NodeKind::Resize { .. } => return Err(Error::UnsupportedPixelOperation),
            NodeKind::Synthetic | NodeKind::Planar10Source | NodeKind::DecoderSource => break,
        };
    }
    let (input, output, filter) = resize.ok_or(Error::UnsupportedPixelOperation)?;
    let x = tap_entry_bound(input.width, output.width, request.width, filter)?;
    let y = tap_entry_bound(input.height, output.height, request.height, filter)?;
    let positions = usize::try_from(u64::from(request.width) + u64::from(request.height) + 2)
        .map_err(|_| Error::GeometryOverflow)?;
    let bytes = x
        .checked_add(y)
        .and_then(|entries| entries.checked_mul(ENTRY_BYTES))
        .and_then(|entries| {
            positions
                .checked_mul(POSITION_BYTES)
                .and_then(|slots| entries.checked_add(slots))
        })
        .ok_or(Error::GeometryOverflow)?;
    budget.reserve_external(bytes)
}

pub(crate) fn tap_entry_bound(
    input: u32,
    output: u32,
    count: u32,
    filter: Filter,
) -> Result<usize, Error> {
    let ratio = u64::from(input).div_ceil(u64::from(output)).max(1);
    let span = ratio
        .checked_mul(match filter {
            Filter::Area => 1,
            Filter::Lanczos3 => 6,
        })
        .and_then(|value| value.checked_add(4))
        .ok_or(Error::GeometryOverflow)?
        .min(u64::from(input));
    let total = span
        .checked_mul(u64::from(count))
        .ok_or(Error::GeometryOverflow)?;
    usize::try_from(total).map_err(|_| Error::GeometryOverflow)
}

#[cfg(test)]
mod plan_bound_tests {
    use super::*;

    #[test]
    fn tap_bound_covers_real_spans_for_both_filters() {
        for filter in [Filter::Area, Filter::Lanczos3] {
            for input in 1..=32 {
                for output in 1..=32 {
                    let actual: usize = (0..output)
                        .map(|at| {
                            let taps = taps(input, output, at, filter).unwrap();
                            usize::try_from(
                                taps.last().unwrap().index - taps.first().unwrap().index + 1,
                            )
                            .unwrap()
                        })
                        .sum();
                    assert!(
                        actual <= tap_entry_bound(input, output, output, filter).unwrap(),
                        "input={input} output={output} filter={filter:?}"
                    );
                }
            }
        }
    }
}

fn sample_rgba(
    source: DecodedRow<'_>,
    pixel: usize,
    channels: usize,
    color: &ColorEncoding,
) -> Result<[f64; 4], Error> {
    let index = pixel.checked_mul(channels).ok_or(Error::GeometryOverflow)?;
    let component = |channel: usize| match source {
        DecodedRow::U8(row) => f64::from(row[index + channel]) / 255.0,
        DecodedRow::F32(row) => f64::from(row[index + channel]),
    };
    let rgb = [component(0), component(1), component(2)];
    if channels == 4 {
        let premultiplied = decode_straight_to_premultiplied_linear_srgb(color, rgb, component(3))?;
        Ok([
            premultiplied.rgb[0],
            premultiplied.rgb[1],
            premultiplied.rgb[2],
            premultiplied.alpha,
        ])
    } else {
        let linear = decode_to_linear_srgb(color, rgb)?;
        Ok([linear.rgb[0], linear.rgb[1], linear.rgb[2], 1.0])
    }
}

fn final_rgba(acc: [f64; 4], opaque: bool) -> Result<[u8; 4], Error> {
    if acc.iter().any(|sample| !sample.is_finite()) {
        return Err(Error::NonFinitePixelSample);
    }
    let alpha = if opaque { 1.0 } else { acc[3].clamp(0.0, 1.0) };
    let mut rgba = [0_u8; 4];
    #[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)]
    {
        rgba[3] = (alpha * 255.0).round() as u8;
    }
    // ADR-0014: an output pixel whose alpha *byte* is zero is transparent
    // black. Dividing by a tiny but nonzero alpha would emit arbitrary RGB
    // behind an invisible pixel and make fast paths disagree at random.
    if rgba[3] != 0 {
        for channel in 0..3 {
            rgba[channel] = encode_linear_srgb8(acc[channel] / alpha)?;
        }
    }
    Ok(rgba)
}

/// A scanline sink is borrowed and non-retaining. Cancelling the sink
/// propagates out immediately and releases all managed row reservations.
#[allow(clippy::too_many_lines)] // Separate planning and sink lifetime are explicit here.
pub(crate) fn execute_quality_srgb8(
    graph: &Graph,
    target: NodeId,
    bound_source: NodeId,
    source: &mut dyn DecodeSource,
    requested: Rect,
    budget: &MemoryBudget,
    mut sink: impl FnMut(u32, &[u8]) -> Result<(), Error>,
) -> Result<QualityReport, Error> {
    let info = source.info().clone();
    let _plan_reservation = reserve_resize_plan(graph, target, requested, budget)?;
    let plan = prepare(graph, target, bound_source, source, requested)?;
    if info.layout == ChannelLayout::Rgba && info.alpha != AlphaMode::Straight {
        return Err(Error::UnsupportedAlphaMode);
    }
    if info.layout == ChannelLayout::Rgb && info.alpha != AlphaMode::Opaque {
        return Err(Error::UnsupportedAlphaMode);
    }
    let channels = usize::from(info.layout.channels());
    let output_width = usize::try_from(requested.width).map_err(|_| Error::GeometryOverflow)?;
    let width_components = output_width.checked_mul(4).ok_or(Error::GeometryOverflow)?;
    let source_width = usize::try_from(plan.source.width).map_err(|_| Error::GeometryOverflow)?;
    let source_components = source_width
        .checked_mul(channels)
        .ok_or(Error::GeometryOverflow)?;

    let mut columns =
        try_vec(usize::try_from(requested.width).map_err(|_| Error::GeometryOverflow)?)?;
    for offset in 0..requested.width {
        let x = plan
            .resize
            .x
            .checked_add(offset)
            .ok_or(Error::GeometryOverflow)?;
        columns.push(taps(plan.input.width, plan.output.width, x, plan.filter)?);
    }
    let mut lines =
        try_vec(usize::try_from(requested.height).map_err(|_| Error::GeometryOverflow)?)?;
    for offset in 0..requested.height {
        let y = plan
            .resize
            .y
            .checked_add(offset)
            .ok_or(Error::GeometryOverflow)?;
        lines.push(taps(plan.input.height, plan.output.height, y, plan.filter)?);
    }
    let needed_capacity = lines.iter().try_fold(0_usize, |sum, row| {
        sum.checked_add(row.len()).ok_or(Error::GeometryOverflow)
    })?;
    let mut needed: Vec<u32> = try_vec(needed_capacity)?;
    needed.extend(lines.iter().flat_map(|row| row.iter().map(|tap| tap.index)));
    needed.sort_unstable();
    needed.dedup();

    // Lanczos can have exact zero crossings: the smallest *nonzero* source
    // tap need not increase monotonically with the destination row. Retain
    // each row until no remaining output row can reference it, rather than
    // only checking the immediately following row.
    let mut future_min_source = try_filled_vec(
        lines.len().checked_add(1).ok_or(Error::GeometryOverflow)?,
        u32::MAX,
    )?;
    for index in (0..lines.len()).rev() {
        let earliest = lines[index]
            .first()
            .ok_or(Error::IncompleteDecoderOutput)?
            .index;
        future_min_source[index] = earliest.min(future_min_source[index + 1]);
    }

    let mut output = budget.allocate_typed::<u8>(width_components)?;
    let mut cache: VecDeque<CachedRow> = VecDeque::new();
    cache
        .try_reserve_exact(needed.len())
        .map_err(|_| Error::AllocationFailed)?;
    let mut output_rows = 0_usize;
    let mut decoded_rows = 0_u32;
    let mut expected_y = plan.source.y;
    let mut transform_time = Duration::ZERO;
    let read_started = Instant::now();
    let report = source.read_region(plan.source, budget, &mut |decoder_y, row| {
        let row_started = Instant::now();
        if decoder_y != expected_y
            || row.len() != source_components
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
        if needed.binary_search(&semantic_y).is_ok() {
            let mut horizontal = budget.allocate_typed::<f64>(width_components)?;
            for (col, weights) in columns.iter().enumerate() {
                for tap in weights {
                    let source_x = usize::try_from(
                        tap.index
                            .checked_sub(plan.footprint.x)
                            .ok_or(Error::OutOfBounds)?,
                    )
                    .map_err(|_| Error::GeometryOverflow)?;
                    let pixel = sample_rgba(row, source_x, channels, &info.semantics().color)?;
                    for (channel, sample) in pixel.iter().enumerate() {
                        horizontal.as_mut_slice()[col * 4 + channel] += sample * tap.weight;
                    }
                }
            }
            cache.push_back(CachedRow {
                y: semantic_y,
                samples: horizontal,
            });
        }

        while output_rows < lines.len()
            && lines[output_rows]
                .last()
                .is_some_and(|tap| tap.index <= semantic_y)
        {
            let taps_y = &lines[output_rows];
            for col in 0..output_width {
                let mut acc = [0.0_f64; 4];
                for tap in taps_y {
                    let data = cache
                        .iter()
                        .find(|row| row.y == tap.index)
                        .ok_or(Error::IncompleteDecoderOutput)?;
                    for (channel, value) in acc.iter_mut().enumerate() {
                        *value += data.samples.as_slice()[col * 4 + channel] * tap.weight;
                    }
                }
                let pixel = final_rgba(acc, info.layout == ChannelLayout::Rgb)?;
                output.as_mut_slice()[col * 4..col * 4 + 4].copy_from_slice(&pixel);
            }
            let out_y =
                requested.y + u32::try_from(output_rows).map_err(|_| Error::GeometryOverflow)?;
            sink(out_y, output.as_slice())?;
            output_rows += 1;
            let earliest_future = future_min_source[output_rows];
            while cache.front().is_some_and(|row| row.y < earliest_future) {
                cache.pop_front();
            }
        }
        // Errors abort the whole run, so only the successful path is timed.
        transform_time += row_started.elapsed();
        Ok(())
    })?;
    let source_time = read_started.elapsed().saturating_sub(transform_time);

    if decoded_rows != plan.source.height
        || output_rows != lines.len()
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
        source_time,
        transform_time,
    })
}
