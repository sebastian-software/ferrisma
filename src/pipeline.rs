//! First practical M1 facade. Codec implementations are opt-in and remain
//! private. Output is SDR AVIF; HDR and unsupported ICC/profiles fail closed.

use std::fmt;
use std::time::{Duration, Instant};

use crate::avif_sink::{AvifSink, EncodeSettings};
use crate::color::ColorEncoding;
use crate::decoder::{DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo};
use crate::error::Error;
use crate::fast_execute::execute_fast_srgb8;
use crate::geometry::{Rect, Size};
use crate::graph::Graph;
use crate::memory::MemoryBudget;
use crate::orientation::OrientedSource;
use crate::quality_execute::execute_quality_srgb8;

pub use crate::quality::Filter as ResizeFilter;

/// Accepted input codecs. Compile the corresponding Cargo features to enable
/// each adapter; no transitive codec dependencies are required by the core.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    JpegXl,
    Avif,
}

/// Source color handling policy; the default never guesses a color profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum InputProfilePolicy {
    #[default]
    Strict,
    /// Explicitly assume sRGB only when the decoder reports *no* output ICC
    /// and *no* output CICP. This must be an informed caller decision.
    AssumeSrgbIfMissing,
    /// Explicitly use the decoder-provided *rendered output* ICC as authoritative.
    /// Requires the optional `icc` feature and currently supports RGB only.
    UseRenderedIcc,
}

/// Working precision of the resample chain (ADR-0021). Both produce the same
/// image within ADR-0021's bound; the reference exists for verification and
/// before/after measurements, not for production use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum WorkingPrecision {
    /// Scalar `f32` executor with table-driven transfer functions.
    #[default]
    F32,
    /// The M1 `f64` reference executor, the oracle of the differential tests.
    F64Reference,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crop {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

/// Explicit output geometry, quality, profile assumption and memory budget.
/// Encoder scratch allocations are opaque and not constrained by the budget.
#[derive(Clone, Copy, Debug)]
pub struct TransformOptions {
    pub output_width: u32,
    pub output_height: u32,
    pub crop: Option<Crop>,
    pub filter: ResizeFilter,
    pub precision: WorkingPrecision,
    pub profile_policy: InputProfilePolicy,
    pub quality: f32,
    pub speed: u8,
    pub max_managed_bytes: usize,
    /// Maximum compressed input size, checked before initializing any codec.
    pub max_input_bytes: usize,
    /// Maximum decoded input pixels; AVIF preflights coded frame dimensions.
    pub max_input_pixels: u64,
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self {
            output_width: 1,
            output_height: 1,
            crop: None,
            filter: ResizeFilter::Lanczos3,
            precision: WorkingPrecision::F32,
            profile_policy: InputProfilePolicy::Strict,
            quality: 75.0,
            speed: 6,
            max_managed_bytes: 256 * 1024 * 1024,
            max_input_bytes: 64 * 1024 * 1024,
            max_input_pixels: 100_000_000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SourceAccess {
    FullFrameFallback,
    CodecRegionRequest,
}

/// Wall-clock time per pipeline stage, measured with `std::time::Instant`.
/// These are observations of one run on one machine, not throughput claims;
/// the stages overlap only where documented on each field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct StageTimings {
    /// Option validation, container and header parsing, orientation and ICC
    /// setup, graph construction: everything before the first decoded pixel.
    pub open: Duration,
    /// Time inside the source's region reader excluding the transform
    /// callback: codec decode, reorientation and ICC conversion.
    pub decode: Duration,
    /// Transfer decode, resampling, quantization and copying rows into the
    /// encoder surface.
    pub transform: Duration,
    /// Encoding the materialized output frame.
    pub encode: Duration,
    /// From entry to return; at least the sum of the other stages.
    pub total: Duration,
}

/// Report known materializations separately from opaque codec memory.
#[derive(Clone, Copy, Debug)]
pub struct ProcessingReport {
    pub input_access: SourceAccess,
    pub decoder_requests: u32,
    pub decoder_full_frame_buffers: u32,
    pub engine_full_frame_intermediates: u32,
    /// Engine-managed bytes materialized to reorient the input; zero for
    /// upright input.
    pub orientation_buffer_bytes: usize,
    pub encoder_output_frame_bytes: usize,
    pub peak_managed_bytes: usize,
    pub opaque_decoder_memory: bool,
    pub opaque_encoder_memory: bool,
    /// The executor that produced the pixels.
    pub working_precision: WorkingPrecision,
    pub timings: StageTimings,
}

#[derive(Debug)]
pub struct Output {
    pub avif_bytes: Vec<u8>,
    pub report: ProcessingReport,
}

/// Broad failure categories a caller can match on. The message carries the
/// details; new categories may be added before a stable release.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProcessingErrorKind {
    /// Options are invalid: quality, speed, zero limits, output geometry, a
    /// crop outside the image, or a geometry calculation that overflows.
    InvalidOptions,
    /// The compressed size or decoded pixel count exceeds a configured limit.
    InputLimitExceeded,
    /// The codec or color-management Cargo feature is not compiled in.
    FeatureDisabled,
    /// The input is truncated, malformed or rejected by its decoder.
    MalformedInput,
    /// The input uses a layout, alpha mode or geometric transform that is
    /// not supported yet.
    UnsupportedInput,
    /// The rendered color is missing, ambiguous, contradictory, HDR or needs
    /// a color-management transform this build cannot perform.
    UnsupportedColor,
    /// The managed-memory budget is exhausted or an allocation failed.
    ResourceExhausted,
    /// Processing was cancelled by the output sink.
    Cancelled,
    /// The encoder failed.
    EncodeFailed,
    /// An internal invariant failed; this indicates a Ferrixel bug.
    Internal,
}

/// Public error that never exposes backend-specific types.
#[derive(Debug)]
pub struct ProcessingError {
    kind: ProcessingErrorKind,
    message: String,
}
impl ProcessingError {
    fn new(kind: ProcessingErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// The failure category, for programmatic handling.
    #[must_use]
    pub fn kind(&self) -> ProcessingErrorKind {
        self.kind
    }
}
impl fmt::Display for ProcessingError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}
impl std::error::Error for ProcessingError {}
impl From<Error> for ProcessingError {
    fn from(value: Error) -> Self {
        use ProcessingErrorKind as Kind;
        let kind = match &value {
            Error::InvalidDimensions | Error::GeometryOverflow | Error::OutOfBounds => {
                Kind::InvalidOptions
            }
            #[cfg(feature = "avif-write")]
            Error::InvalidEncodeOptions => Kind::InvalidOptions,
            #[cfg(feature = "avif-read")]
            Error::InputPixelLimitExceeded { .. } => Kind::InputLimitExceeded,
            Error::DecodeFailed
            | Error::IncompleteDecoderOutput
            | Error::NonFinitePixelSample
            | Error::InvalidAlphaValue => Kind::MalformedInput,
            Error::UnsupportedAlphaMode
            | Error::UnsupportedCodecLayout
            | Error::UnsupportedPixelOperation
            | Error::ChromaPhaseRequiresResampling => Kind::UnsupportedInput,
            Error::UnsupportedImageTransform => Kind::UnsupportedInput,
            Error::UnsupportedColorEncoding => Kind::UnsupportedColor,
            #[cfg(feature = "avif-read")]
            Error::ContradictoryColorSignals => Kind::UnsupportedColor,
            Error::AllocationFailed | Error::BudgetExceeded { .. } => Kind::ResourceExhausted,
            Error::Cancelled => Kind::Cancelled,
            Error::EncodeFailed => Kind::EncodeFailed,
            Error::IncompleteOutput
            | Error::UnknownNode
            | Error::PixelFormatMismatch
            | Error::SourceBindingMismatch
            | Error::InvalidPlaneStride
            | Error::InvalidPlaneDimensions
            | Error::InsufficientPlaneStorage => Kind::Internal,
        };
        Self::new(kind, value.to_string())
    }
}

struct ExplicitSrgb<'a> {
    decoder: Box<dyn DecodeSource + 'a>,
    info: DecoderInfo,
}
impl DecodeSource for ExplicitSrgb<'_> {
    fn info(&self) -> &DecoderInfo {
        &self.info
    }
    fn read_region(
        &mut self,
        rect: Rect,
        budget: &MemoryBudget,
        sink: &mut dyn FnMut(u32, DecodedRow<'_>) -> Result<(), Error>,
    ) -> Result<DecodeReport, Error> {
        self.decoder.read_region(rect, budget, sink)
    }
}

fn open_source<'a>(
    bytes: &'a [u8],
    format: ImageFormat,
    max_pixels: u64,
) -> Result<Box<dyn DecodeSource + 'a>, ProcessingError> {
    #[cfg(not(feature = "avif-read"))]
    let _ = max_pixels;
    match format {
        ImageFormat::Jpeg => {
            #[cfg(feature = "jpeg-read")]
            {
                Ok(Box::new(crate::codecs::jpeg::JpegSource::new(bytes)?))
            }
            #[cfg(not(feature = "jpeg-read"))]
            {
                let _ = bytes;
                Err(ProcessingError::new(
                    ProcessingErrorKind::FeatureDisabled,
                    "enable the jpeg-read Cargo feature",
                ))
            }
        }
        ImageFormat::JpegXl => {
            #[cfg(feature = "jpeg-xl-read")]
            {
                Ok(Box::new(crate::codecs::jxl::JxlSource::new(bytes)?))
            }
            #[cfg(not(feature = "jpeg-xl-read"))]
            {
                let _ = bytes;
                Err(ProcessingError::new(
                    ProcessingErrorKind::FeatureDisabled,
                    "enable the jpeg-xl-read Cargo feature",
                ))
            }
        }
        ImageFormat::Avif => {
            #[cfg(feature = "avif-read")]
            {
                Ok(Box::new(crate::codecs::avif::AvifSource::new_bounded(
                    bytes, max_pixels,
                )?))
            }
            #[cfg(not(feature = "avif-read"))]
            {
                let _ = bytes;
                Err(ProcessingError::new(
                    ProcessingErrorKind::FeatureDisabled,
                    "enable the avif-read Cargo feature",
                ))
            }
        }
    }
}

/// Convert one real input to SDR AVIF. Quality and profile constraints are
/// enforced before executing a pixel transform, and only *one* decoder region
/// is requested for the output. The output frame is explicitly materialized
/// for the current `ravif` encoder, so memory is not O(output width).
///
/// # Errors
/// Returns an error for unsupported codec features, ambiguous profiles,
/// invalid geometry/options, memory exhaustion or encoder failures.
#[allow(clippy::too_many_lines)] // The stages stay visible in a small facade.
pub fn transcode(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
) -> Result<Output, ProcessingError> {
    let started = Instant::now();
    EncodeSettings {
        quality: options.quality,
        speed: options.speed,
    }
    .validate()?;
    let target_size = Size::new(options.output_width, options.output_height)?;
    if options.max_input_bytes == 0 || options.max_input_pixels == 0 {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "input byte/pixel limits must be nonzero",
        ));
    }
    if encoded.len() > options.max_input_bytes {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InputLimitExceeded,
            "compressed image exceeds max_input_bytes",
        ));
    }
    let source = open_source(encoded, format, options.max_input_pixels)?;
    // Present an upright image to the graph; crops are display coordinates.
    let source: Box<dyn DecodeSource + '_> = if source.orientation().is_identity() {
        source
    } else {
        Box::new(OrientedSource::new(source)?)
    };
    let dimensions = source.info().size;
    let pixels = u64::from(dimensions.width) * u64::from(dimensions.height);
    if pixels > options.max_input_pixels {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InputLimitExceeded,
            format!(
                "image has {pixels} pixels; configured limit is {}",
                options.max_input_pixels
            ),
        ));
    }
    let source: Box<dyn DecodeSource + '_> =
        if options.profile_policy == InputProfilePolicy::UseRenderedIcc {
            #[cfg(feature = "icc")]
            {
                Box::new(crate::cms::IccSrgbSource::new(source)?)
            }
            #[cfg(not(feature = "icc"))]
            {
                return Err(ProcessingError::new(
                    ProcessingErrorKind::FeatureDisabled,
                    "enable the icc Cargo feature",
                ));
            }
        } else {
            source
        };
    let mut actual: Box<dyn DecodeSource + '_> = if source.info().rendered_icc.is_none()
        && source.info().rendered_cicp.is_none()
        && options.profile_policy == InputProfilePolicy::AssumeSrgbIfMissing
    {
        let mut info = source.info().clone();
        info.rendered_cicp = Some([1, 13, 0, 1]);
        Box::new(ExplicitSrgb {
            decoder: source,
            info,
        })
    } else {
        source
    };
    if !matches!(
        actual.info().semantics().color,
        ColorEncoding::Cicp([1 | 12, 13, 0, 1])
    ) {
        return Err(ProcessingError::new(
            ProcessingErrorKind::UnsupportedColor,
            "unsupported or ambiguous rendered color profile; a color-management transform is required",
        ));
    }
    let size = actual.info().size;
    let crop = if let Some(crop) = options.crop {
        Rect::new(crop.x, crop.y, crop.width, crop.height)?
    } else {
        Rect::new(0, 0, size.width, size.height)?
    };
    if !crop.fits(size) {
        return Err(Error::OutOfBounds.into());
    }

    let mut graph = Graph::new();
    let input = graph.decoded_source(
        size,
        actual.info().pixel_format(),
        actual.info().semantics(),
    )?;
    let cropped = graph.crop(input, crop)?;
    let linear = graph.add_linear_srgb(cropped)?;
    let resized = graph.resize_working(linear, target_size, options.filter)?;
    let finished = graph.add_srgb8(resized)?;

    let budget = MemoryBudget::new(options.max_managed_bytes);
    let mut output_sink = AvifSink::new(target_size, &budget)?;
    let open = started.elapsed();
    let request = Rect::new(0, 0, target_size.width, target_size.height)?;
    let processed = match options.precision {
        WorkingPrecision::F32 => execute_fast_srgb8(
            &graph,
            finished,
            input,
            &mut *actual,
            request,
            &budget,
            |y, row| output_sink.write_row(y, row),
        )?,
        WorkingPrecision::F64Reference => execute_quality_srgb8(
            &graph,
            finished,
            input,
            &mut *actual,
            request,
            &budget,
            |y, row| output_sink.write_row(y, row),
        )?,
    };
    let encode_started = Instant::now();
    let encoded_avif = output_sink.finish(EncodeSettings {
        quality: options.quality,
        speed: options.speed,
    })?;
    let encode = encode_started.elapsed();
    let input_access = match processed.decoder.access {
        DecodeAccess::FullFrameFallback => SourceAccess::FullFrameFallback,
        DecodeAccess::CodecRegionRequest => SourceAccess::CodecRegionRequest,
    };
    let report = ProcessingReport {
        input_access,
        decoder_requests: processed.decoder_requests,
        decoder_full_frame_buffers: processed.decoder.known_full_frame_buffers,
        engine_full_frame_intermediates: processed.explicit_full_frame_intermediates,
        orientation_buffer_bytes: processed.decoder.engine_buffer_bytes,
        encoder_output_frame_bytes: encoded_avif.frame_materialization_bytes,
        peak_managed_bytes: budget.snapshot().peak,
        opaque_decoder_memory: processed.decoder.opaque_codec_memory_unbounded,
        opaque_encoder_memory: encoded_avif.opaque_encoder_memory_unbounded,
        working_precision: options.precision,
        timings: StageTimings {
            open,
            decode: processed.source_time,
            transform: processed.transform_time,
            encode,
            total: started.elapsed(),
        },
    };
    Ok(Output {
        avif_bytes: encoded_avif.bytes,
        report,
    })
}
