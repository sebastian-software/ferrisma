//! A private, capability-reporting boundary between graph planning and codecs.
//! A requested ROI is NOT proof that the underlying codec avoided other work.
use crate::color::{AlphaMode, ColorEncoding, ImageSemantics};
use crate::color_contract::{ColorSignal, inspect_rendered_color};
use crate::error::Error;
use crate::geometry::{Rect, Size};
use crate::graph::{Graph, NodeId, NodeKind};
use crate::memory::MemoryBudget;
use crate::orientation::Orientation;
use crate::plan::plan_region;
use crate::surface::PixelFormat;

// Preserve the decoder module's import path while keeping channel meaning
// in the shared surface contract used by both graph and codec adapters.
pub(crate) use crate::surface::ChannelLayout;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SampleType {
    U8,
    F32,
}

// A no-codec build can validate region planning without a full-frame adapter.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum DecodeAccess {
    /// Complete decoded frame required even if the sink needs only a crop.
    FullFrameFallback,
    /// Region requested through the codec API; actual skipped work is unknown.
    CodecRegionRequest,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct DecoderInfo {
    pub size: Size,
    pub layout: ChannelLayout,
    pub sample: SampleType,
    pub access: DecodeAccess,
    /// Alpha association of emitted samples, not of the encoded bitstream.
    pub alpha: AlphaMode,
    /// Describes *emitted* samples. None means unknown, not assumed sRGB.
    pub rendered_icc: Option<Vec<u8>>,
    pub rendered_cicp: Option<[u8; 4]>,
}
impl DecoderInfo {
    /// Inspect actual emitted samples' profile evidence without copying ICC
    /// bytes; input-file tags must not be substituted after decoder conversion.
    #[cfg_attr(
        not(test),
        allow(dead_code, reason = "used by test-only architecture experiments")
    )]
    pub(crate) fn rendered_color_signal(&self) -> ColorSignal {
        inspect_rendered_color(self.rendered_icc.as_deref(), self.rendered_cicp)
    }

    pub(crate) fn semantics(&self) -> ImageSemantics {
        ImageSemantics {
            color: ColorEncoding::from_metadata(self.rendered_icc.as_deref(), self.rendered_cicp),
            alpha: self.alpha,
        }
    }

    pub(crate) fn pixel_format(&self) -> PixelFormat {
        match self.sample {
            SampleType::U8 => PixelFormat::InterleavedU8(self.layout),
            SampleType::F32 => PixelFormat::InterleavedF32(self.layout),
        }
    }
}

// Float rows are only constructed by the optional JPEG XL adapter.
#[allow(dead_code)]
#[derive(Clone, Copy, Debug)]
pub(crate) enum DecodedRow<'a> {
    U8(&'a [u8]),
    F32(&'a [f32]),
}
impl DecodedRow<'_> {
    pub(crate) fn len(self) -> usize {
        match self {
            Self::U8(r) => r.len(),
            Self::F32(r) => r.len(),
        }
    }
    pub(crate) fn sample_type(self) -> SampleType {
        match self {
            Self::U8(_) => SampleType::U8,
            Self::F32(_) => SampleType::F32,
        }
    }
}
/// A decoder work count that is either known exactly or only bounded from
/// below. An adapter reports `AtLeast` whenever the codec's internal skipping
/// is not observed: a cropped returned buffer is never evidence of an upper
/// bound (ADR-0012, ADR-0023).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum WorkCount {
    /// The adapter controls or observes this amount of work exactly.
    Exact(u64),
    /// The codec did at least this much work; more is possible and unmeasured.
    AtLeast(u64),
}

impl WorkCount {
    /// The proven lower bound of the work.
    #[must_use]
    pub fn at_least(self) -> u64 {
        match self {
            Self::Exact(n) | Self::AtLeast(n) => n,
        }
    }
}

impl std::fmt::Display for WorkCount {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Exact(n) => write!(f, "{n}"),
            Self::AtLeast(n) => write!(f, ">={n}"),
        }
    }
}

/// What one codec call produced and which work it necessarily performed, in
/// stored (pre-orientation) coordinates. `request` and `codec_output` are
/// exact; `reconstructed_pixels` and `entropy_rows` are exact only where the
/// adapter can establish them, otherwise lower bounds derived from the codec
/// output and from work the format cannot skip.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct CodecWork {
    /// The stored rectangle the adapter requested from the codec.
    pub request: Rect,
    /// The stored rectangle of samples the codec wrote for the adapter. Its
    /// area minus `request`'s is the source overfetch.
    pub codec_output: Rect,
    /// Pixels the codec reconstructed (prediction or inverse transform,
    /// upsampling and color conversion), including discarded context rows.
    pub reconstructed_pixels: WorkCount,
    /// Stored rows whose compressed data the codec parsed.
    pub entropy_rows: WorkCount,
}

impl CodecWork {
    /// A codec that reconstructs the whole stored frame for every request.
    #[cfg_attr(
        not(any(feature = "jpeg-xl-own-read", feature = "avif-read")),
        allow(dead_code, reason = "used by the full-frame codec adapters")
    )]
    pub(crate) fn full_frame(request: Rect, size: Size) -> Self {
        Self {
            request,
            codec_output: Rect {
                x: 0,
                y: 0,
                width: size.width,
                height: size.height,
            },
            reconstructed_pixels: WorkCount::AtLeast(
                u64::from(size.width) * u64::from(size.height),
            ),
            entropy_rows: WorkCount::AtLeast(u64::from(size.height)),
        }
    }

    /// A source that emits exactly the requested samples and whose internal
    /// work beyond them is not observed (synthetic sources and ROI codecs).
    pub(crate) fn region_at_least(request: Rect) -> Self {
        Self {
            request,
            codec_output: request,
            reconstructed_pixels: WorkCount::AtLeast(
                u64::from(request.width) * u64::from(request.height),
            ),
            entropy_rows: WorkCount::AtLeast(u64::from(request.height)),
        }
    }

    /// Codec output pixels beyond the request.
    pub(crate) fn overfetch_pixels(&self) -> u64 {
        let area = |r: Rect| u64::from(r.width) * u64::from(r.height);
        area(self.codec_output).saturating_sub(area(self.request))
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct DecodeReport {
    pub requested: Rect,
    pub access: DecodeAccess,
    pub rows: u64,
    pub output_sample_bytes: u64,
    /// Explicitly known, adapter-allocated full output frames.
    pub known_full_frame_buffers: u32,
    pub peak_engine_managed_bytes: usize,
    /// Codec-managed compressed data, render cache, and scratch are not capped.
    pub opaque_codec_memory_unbounded: bool,
    /// Engine-owned sample buffers materialized by source wrappers, such as
    /// the reorientation region (ADR-0019). Zero for plain decoders.
    pub engine_buffer_bytes: usize,
    /// Sample bytes that source wrappers copied between engine buffers, such
    /// as the reorientation's stored-region fill and upright gather. Zero for
    /// plain decoders.
    pub engine_copied_bytes: u64,
    /// The innermost codec call's request, output and work; wrappers keep it.
    pub work: CodecWork,
}

/// A stateless per-row conversion that a source wrapper applies to every
/// row it emits, and that it can hand to the executor instead, so that the
/// conversion runs on the executor's workers rather than on the thread that
/// drives the decoder (ADR-0023). The converted row is what the wrapper's
/// `info()` describes; deferred rows have `input_sample()` and the same
/// number of samples.
pub(crate) trait RowConversion: Send + Sync {
    /// Sample type of the rows the source emits once the conversion is
    /// deferred.
    fn input_sample(&self) -> SampleType;
    /// Convert one deferred row into `out` (same length), using `scratch`
    /// (same length) as working space.
    fn convert(
        &self,
        row: DecodedRow<'_>,
        scratch: &mut [f32],
        out: &mut [f32],
    ) -> Result<(), Error>;
}

pub(crate) trait DecodeSource {
    fn info(&self) -> &DecoderInfo;
    /// How emitted (stored-order) samples must be reoriented for display.
    fn orientation(&self) -> Orientation {
        Orientation::IDENTITY
    }
    /// Stop applying this source's per-row conversion and return it; the
    /// caller then applies it to every row before using the row. Called
    /// before `read_region`. `None`, the default, means rows arrive
    /// converted.
    fn defer_conversion(&mut self) -> Option<std::sync::Arc<dyn RowConversion>> {
        None
    }
    /// Borrows each output row only for the duration of the sink callback.
    fn read_region(
        &mut self,
        region: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error>;
}

/// Crop-only graph execution through the existing backwards region planner.
#[cfg_attr(
    not(test),
    allow(dead_code, reason = "used by test-only architecture experiments")
)]
/// A separate transform executor is required before resizing these layouts.
pub(crate) fn execute_decoded(
    graph: &Graph,
    target: NodeId,
    source_node: NodeId,
    source: &mut dyn DecodeSource,
    output: Rect,
    budget: &MemoryBudget,
    mut sink: impl FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
) -> Result<DecodeReport, Error> {
    let steps = plan_region(graph, target, output)?;
    let first = steps.first().ok_or(Error::UnknownNode)?;
    let node = graph.node(first.node)?;
    let info = source.info();
    if first.node != source_node
        || node.kind != NodeKind::DecoderSource
        || node.size != info.size
        || node.format != info.pixel_format()
        || node.semantics != info.semantics()
    {
        return Err(Error::SourceBindingMismatch);
    }
    if steps.iter().skip(1).any(
        |step| !matches!(graph.node(step.node), Ok(n) if matches!(n.kind, NodeKind::Crop { .. })),
    ) {
        return Err(Error::UnsupportedPixelOperation);
    }
    let expected_samples = usize::try_from(first.region.width)
        .map_err(|_| Error::GeometryOverflow)?
        .checked_mul(usize::from(info.layout.channels()))
        .ok_or(Error::GeometryOverflow)?;
    let expected_type = info.sample;
    let mut expected_y = first.region.y;
    let mut delivered = 0_u32;
    let report = source.read_region(first.region, budget, &mut |y, row| {
        if y != expected_y || row.len() != expected_samples || row.sample_type() != expected_type {
            return Err(Error::IncompleteDecoderOutput);
        }
        let output_y = output
            .y
            .checked_add(delivered)
            .ok_or(Error::GeometryOverflow)?;
        sink(output_y, row)?;
        delivered += 1;
        expected_y += 1;
        Ok(())
    })?;
    if delivered != output.height || report.rows != u64::from(delivered) {
        return Err(Error::IncompleteDecoderOutput);
    }
    Ok(report)
}
