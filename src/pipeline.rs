//! First practical M1 facade. Codec implementations are opt-in and remain
//! private. Output is SDR AVIF or WebP; unsupported color fails closed.

use std::fmt;
use std::time::{Duration, Instant};

#[cfg(feature = "avif-write")]
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
#[cfg(feature = "webp-write")]
use crate::webp_sink::WebpSink;

pub use crate::encoder_backend::{
    EncodeError, EncodeErrorKind, EncodeRequest, EncodedImage, EncoderBackend, RowEncoder,
};
pub use crate::quality::Filter as ResizeFilter;
pub use ferrixel_kernels::{IsaLevel as KernelLevel, LevelReason as KernelLevelReason};

/// Chroma layout for AVIF output and AVIF encoder backends.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum ChromaSubsampling {
    /// Full-resolution chroma, the AVIF default.
    #[default]
    Yuv444,
    /// Half-resolution chroma in both directions, centered 2×2 average.
    Yuv420,
}

/// The encoded container format. WebP has no selectable chroma layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub enum OutputFormat {
    #[default]
    Avif,
    WebP,
}

/// Settings for the built-in still WebP encoder.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum WebpOptions {
    /// Quality is 1–100; speed is 1 (slowest) through 10 (fastest).
    Lossy { quality: f32, speed: u8 },
    /// Level 0 is fastest; 9 gives the smallest lossless files.
    Lossless { level: u8 },
}

impl Default for WebpOptions {
    fn default() -> Self {
        Self::Lossy {
            quality: 75.0,
            speed: 6,
        }
    }
}

/// Accepted input codecs. Compile the corresponding Cargo features to enable
/// each adapter; no transitive codec dependencies are required by the core.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ImageFormat {
    Jpeg,
    WebP,
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

/// Select the JPEG XL input backend explicitly. The interim reader remains
/// the default until the reference port meets the selection gates (ADR-0032).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum JpegXlDecoder {
    #[default]
    JxlOxide,
    LibjxlRs,
}

/// Explicit output geometry, quality, profile assumption and memory budget.
/// Encoder scratch allocations are opaque and not constrained by the budget.
#[derive(Clone, Copy, Debug)]
pub struct TransformOptions {
    pub output_format: OutputFormat,
    pub webp: WebpOptions,
    pub output_width: u32,
    pub output_height: u32,
    pub crop: Option<Crop>,
    pub filter: ResizeFilter,
    pub precision: WorkingPrecision,
    /// Highest SIMD kernel level to use; `None` picks the best the machine
    /// supports. Lowering it never changes the result beyond the ADR-0021
    /// bound; it exists for verification, benchmarks and bug reports.
    pub kernel_level_ceiling: Option<KernelLevel>,
    /// Threads for the decode-transform row pipeline (ADR-0022): 0 means
    /// all available cores, 1 the sequential path. The output is
    /// byte-identical for every value.
    pub transform_threads: usize,
    pub profile_policy: InputProfilePolicy,
    pub jxl_decoder: JpegXlDecoder,
    pub quality: f32,
    pub speed: u8,
    /// AV1 encoder threads: 0 means all available cores. 1 (the default)
    /// encodes one tile; any other value splits the image into tiles derived
    /// from its size alone, so the bytes are the same for every thread count
    /// other than 1 and on every machine. Tiles cost a few percent of bytes;
    /// measurements in `docs/benchmarks/M2-baseline.md`.
    pub encoder_threads: usize,
    /// Chroma layout of the AVIF output; 4:4:4 by default.
    pub chroma_subsampling: ChromaSubsampling,
    pub max_managed_bytes: usize,
    /// Maximum compressed input size, checked before initializing any codec.
    pub max_input_bytes: usize,
    /// Maximum decoded input pixels; AVIF preflights coded frame dimensions.
    pub max_input_pixels: u64,
    /// Maximum retained encoded output bytes. WebP checks while writing;
    /// AVIF and external backends are checked after their encoder returns.
    pub max_output_bytes: usize,
}

impl Default for TransformOptions {
    fn default() -> Self {
        Self {
            output_format: OutputFormat::Avif,
            webp: WebpOptions::default(),
            output_width: 1,
            output_height: 1,
            crop: None,
            filter: ResizeFilter::Lanczos3,
            precision: WorkingPrecision::F32,
            kernel_level_ceiling: None,
            transform_threads: 0,
            profile_policy: InputProfilePolicy::Strict,
            jxl_decoder: JpegXlDecoder::JxlOxide,
            quality: 75.0,
            speed: 6,
            encoder_threads: 1,
            chroma_subsampling: ChromaSubsampling::Yuv444,
            max_managed_bytes: 256 * 1024 * 1024,
            max_input_bytes: 64 * 1024 * 1024,
            max_input_pixels: 100_000_000,
            max_output_bytes: 256 * 1024 * 1024,
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
    /// Bytes of the encoder's YUV input frame, allocated by `rav1e` on the
    /// sink's request and charged to the managed budget (the larger of the
    /// color and alpha frames; they are not alive at the same time).
    pub encoder_output_frame_bytes: usize,
    pub peak_managed_bytes: usize,
    pub opaque_decoder_memory: bool,
    pub opaque_encoder_memory: bool,
    /// The executor that produced the pixels.
    pub working_precision: WorkingPrecision,
    /// SIMD level of the resample kernels and why it was chosen; the `f64`
    /// reference executor always reports `Scalar`.
    pub kernel_level: KernelLevel,
    pub kernel_level_reason: KernelLevelReason,
    /// Encoder threads actually used (resolved when the option was 0).
    pub encoder_threads: usize,
    /// Transform pipeline threads actually used; 1 for the sequential path
    /// and the `f64` reference.
    pub transform_threads: usize,
    /// The encoder that wrote the output: `"rav1e"` for `transcode`, the
    /// backend's name for `transcode_with` (ADR-0024).
    pub encoder_backend: &'static str,
    pub timings: StageTimings,
}

#[derive(Debug)]
pub struct Output {
    pub format: OutputFormat,
    pub bytes: Vec<u8>,
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
impl From<EncodeError> for ProcessingError {
    fn from(value: EncodeError) -> Self {
        let kind = match value.kind() {
            EncodeErrorKind::InvalidOptions => ProcessingErrorKind::InvalidOptions,
            EncodeErrorKind::ResourceExhausted => ProcessingErrorKind::ResourceExhausted,
            EncodeErrorKind::Failed => ProcessingErrorKind::EncodeFailed,
        };
        Self::new(kind, format!("encoder backend: {value}"))
    }
}
impl From<Error> for ProcessingError {
    fn from(value: Error) -> Self {
        use ProcessingErrorKind as Kind;
        let kind = match &value {
            Error::InvalidDimensions | Error::GeometryOverflow | Error::OutOfBounds => {
                Kind::InvalidOptions
            }
            Error::InvalidEncodeOptions => Kind::InvalidOptions,
            #[cfg(any(
                feature = "avif-read",
                feature = "webp-read",
                feature = "jpeg-xl-own-read"
            ))]
            Error::InputPixelLimitExceeded { .. } => Kind::InputLimitExceeded,
            Error::DecodeFailed
            | Error::IncompleteDecoderOutput
            | Error::NonFinitePixelSample
            | Error::InvalidAlphaValue => Kind::MalformedInput,
            Error::UnsupportedAlphaMode
            | Error::UnsupportedCodecLayout
            | Error::UnsupportedPixelOperation
            | Error::ChromaPhaseRequiresResampling => Kind::UnsupportedInput,
            #[cfg(feature = "webp-read")]
            Error::UnsupportedAnimation => Kind::UnsupportedInput,
            Error::UnsupportedImageTransform => Kind::UnsupportedInput,
            Error::UnsupportedColorEncoding => Kind::UnsupportedColor,
            #[cfg(feature = "avif-read")]
            Error::ContradictoryColorSignals => Kind::UnsupportedColor,
            Error::AllocationFailed | Error::BudgetExceeded { .. } => Kind::ResourceExhausted,
            Error::Cancelled => Kind::Cancelled,
            Error::EncodeFailed => Kind::EncodeFailed,
            #[cfg(any(feature = "avif-write", feature = "webp-write"))]
            Error::IncompleteOutput => Kind::Internal,
            Error::UnknownNode
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
    jxl_decoder: JpegXlDecoder,
) -> Result<Box<dyn DecodeSource + 'a>, ProcessingError> {
    #[cfg(not(any(
        feature = "avif-read",
        feature = "webp-read",
        feature = "jpeg-xl-own-read"
    )))]
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
        ImageFormat::WebP => {
            #[cfg(feature = "webp-read")]
            {
                Ok(Box::new(crate::codecs::webp::WebpSource::new_bounded(
                    bytes, max_pixels,
                )?))
            }
            #[cfg(not(feature = "webp-read"))]
            {
                let _ = bytes;
                Err(ProcessingError::new(
                    ProcessingErrorKind::FeatureDisabled,
                    "enable the webp-read Cargo feature",
                ))
            }
        }
        ImageFormat::JpegXl => match jxl_decoder {
            JpegXlDecoder::JxlOxide => {
                #[cfg(feature = "jpeg-xl-read")]
                {
                    Ok(Box::new(crate::codecs::jxl_oxide::JxlSource::new(bytes)?))
                }
                #[cfg(not(feature = "jpeg-xl-read"))]
                {
                    Err(ProcessingError::new(
                        ProcessingErrorKind::FeatureDisabled,
                        "enable the jpeg-xl-read Cargo feature for jxl-oxide",
                    ))
                }
            }
            JpegXlDecoder::LibjxlRs => {
                #[cfg(feature = "jpeg-xl-own-read")]
                {
                    Ok(Box::new(crate::codecs::jxl::JxlSource::new_bounded(
                        bytes, max_pixels,
                    )?))
                }
                #[cfg(not(feature = "jpeg-xl-own-read"))]
                {
                    Err(ProcessingError::new(
                        ProcessingErrorKind::FeatureDisabled,
                        "enable the jpeg-xl-own-read Cargo feature for libjxl-rs",
                    ))
                }
            }
        },
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
/// is requested for the output. The encoder's input frame is explicitly
/// materialized for `rav1e`, so memory is not O(output width).
///
/// # Errors
/// Returns an error for unsupported codec features, ambiguous profiles,
/// invalid geometry/options, memory exhaustion or encoder failures.
pub fn transcode(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
) -> Result<Output, ProcessingError> {
    run(encoded, format, options, None)
}

/// [`transcode`] with another encoder (ADR-0024): the same decode, color,
/// crop, resample and quantization, then the output rows go to `backend`
/// instead of the built-in `rav1e` sink. `quality`, `speed`,
/// `encoder_threads` and `chroma_subsampling` are passed to the backend in
/// an [`EncodeRequest`]; the backend validates them on its own scale.
///
/// # Errors
/// As [`transcode`], plus the backend's errors, mapped to their
/// [`ProcessingErrorKind`].
pub fn transcode_with(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
    backend: &dyn EncoderBackend,
) -> Result<Output, ProcessingError> {
    run(encoded, format, options, Some(backend))
}

/// The output side of one transcode: the built-in sink or a backend's.
enum Sink<'b> {
    #[cfg(feature = "avif-write")]
    Rav1e(Box<AvifSink>),
    #[cfg(feature = "webp-write")]
    WebP(Box<WebpSink>),
    Backend {
        encoder: Box<dyn RowEncoder + 'b>,
        /// The backend error that stopped the pipeline, if any; the
        /// executor only carries the internal `Error`.
        failure: Option<EncodeError>,
    },
}

impl Sink<'_> {
    fn write_row(&mut self, y: u32, row: &[u8]) -> Result<(), Error> {
        match self {
            #[cfg(feature = "avif-write")]
            Self::Rav1e(sink) => sink.write_row(y, row),
            #[cfg(feature = "webp-write")]
            Self::WebP(sink) => sink.write_row(y, row),
            Self::Backend { encoder, failure } => encoder.write_row(y, row).map_err(|error| {
                *failure = Some(error);
                Error::EncodeFailed
            }),
        }
    }

    /// The backend error behind `error`, if the backend caused it.
    fn explain(&mut self, error: Error) -> ProcessingError {
        match self {
            Self::Backend { failure, .. } if failure.is_some() => failure
                .take()
                .map_or_else(|| error.into(), ProcessingError::from),
            Self::Backend { .. } => error.into(),
            #[cfg(feature = "avif-write")]
            Self::Rav1e(_) => error.into(),
            #[cfg(feature = "webp-write")]
            Self::WebP(_) => error.into(),
        }
    }

    #[allow(unused_variables)] // Only the built-in AVIF sink consumes the budget on finish.
    fn finish(self, budget: &MemoryBudget) -> Result<EncodedImage, ProcessingError> {
        match self {
            #[cfg(feature = "avif-write")]
            Self::Rav1e(sink) => {
                let avif = sink.finish(budget)?;
                Ok(EncodedImage::new(
                    OutputFormat::Avif,
                    avif.bytes,
                    avif.frame_materialization_bytes,
                    avif.opaque_encoder_memory_unbounded,
                ))
            }
            #[cfg(feature = "webp-write")]
            Self::WebP(sink) => Ok(sink.finish()?),
            Self::Backend { encoder, .. } => Ok(encoder.finish()?),
        }
    }
}

#[allow(clippy::too_many_lines)] // The stages stay visible in a small facade.
fn run(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
    backend: Option<&dyn EncoderBackend>,
) -> Result<Output, ProcessingError> {
    let started = Instant::now();
    if options.max_output_bytes == 0 {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "max_output_bytes must be nonzero",
        ));
    }
    if match options.output_format {
        OutputFormat::Avif => options.webp != WebpOptions::default(),
        OutputFormat::WebP => {
            options.quality.to_bits() != 75.0_f32.to_bits()
                || options.speed != 6
                || options.chroma_subsampling != ChromaSubsampling::Yuv444
        }
    } {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "encoder options do not match the selected output format",
        ));
    }
    if let Some(backend) = backend
        && backend.format() != options.output_format
    {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "output format does not match encoder backend",
        ));
    }
    let encoder_threads = match options.encoder_threads {
        0 => std::thread::available_parallelism()
            .map_or(1, std::num::NonZero::get)
            .min(256),
        threads => threads,
    };
    #[cfg(feature = "avif-write")]
    let encode_settings = EncodeSettings {
        quality: options.quality,
        speed: options.speed,
        threads: encoder_threads,
        parallel_tiles: options.encoder_threads != 1,
        chroma: options.chroma_subsampling,
    };
    #[cfg(feature = "avif-write")]
    if backend.is_none() && options.output_format == OutputFormat::Avif {
        encode_settings.validate()?;
    }
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
    let transform_threads = match (options.precision, options.transform_threads) {
        (WorkingPrecision::F64Reference, _) => 1,
        (WorkingPrecision::F32, 0) => {
            std::thread::available_parallelism().map_or(1, std::num::NonZero::get)
        }
        (WorkingPrecision::F32, threads) => threads,
    };
    let source = open_source(
        encoded,
        format,
        options.max_input_pixels,
        options.jxl_decoder,
    )?;
    // Present an upright image to the graph; crops are display coordinates.
    let source: Box<dyn DecodeSource + '_> = if source.orientation().is_identity() {
        source
    } else {
        Box::new(OrientedSource::new(source)?.with_threads(transform_threads))
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
        ColorEncoding::Cicp([1 | 12, 13, 0, 1] | [1, 8, 0, 1])
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
    let transparent = actual.info().layout == crate::decoder::ChannelLayout::Rgba;
    let mut output_sink = match backend {
        None => match options.output_format {
            OutputFormat::Avif => {
                #[cfg(feature = "avif-write")]
                {
                    Sink::Rav1e(Box::new(AvifSink::new(
                        target_size,
                        transparent,
                        encode_settings,
                        &budget,
                    )?))
                }
                #[cfg(not(feature = "avif-write"))]
                {
                    return Err(ProcessingError::new(
                        ProcessingErrorKind::FeatureDisabled,
                        "enable the avif-write Cargo feature",
                    ));
                }
            }
            OutputFormat::WebP => {
                #[cfg(feature = "webp-write")]
                {
                    Sink::WebP(Box::new(WebpSink::new(target_size, options, &budget)?))
                }
                #[cfg(not(feature = "webp-write"))]
                {
                    return Err(ProcessingError::new(
                        ProcessingErrorKind::FeatureDisabled,
                        "enable the webp-write Cargo feature",
                    ));
                }
            }
        },
        Some(backend) => Sink::Backend {
            encoder: backend.begin(&EncodeRequest {
                format: options.output_format,
                webp: options.webp,
                width: target_size.width,
                height: target_size.height,
                has_alpha: transparent,
                quality: options.quality,
                speed: options.speed,
                threads: encoder_threads,
                chroma: options.chroma_subsampling,
            })?,
            failure: None,
        },
    };
    let encoder_backend = backend.map_or(
        match options.output_format {
            OutputFormat::Avif => "rav1e",
            OutputFormat::WebP => "libwebp-rs",
        },
        EncoderBackend::name,
    );
    let open = started.elapsed();
    let request = Rect::new(0, 0, target_size.width, target_size.height)?;
    let kernels = match options.kernel_level_ceiling {
        Some(ceiling) => ferrixel_kernels::Kernels::with_ceiling(ceiling),
        None => ferrixel_kernels::Kernels::best(),
    };
    let (kernel_level, kernel_level_reason) = match options.precision {
        WorkingPrecision::F32 => (kernels.level(), kernels.reason()),
        WorkingPrecision::F64Reference => (KernelLevel::Scalar, KernelLevelReason::Ceiling),
    };
    let processed = match options.precision {
        WorkingPrecision::F32 => execute_fast_srgb8(
            &graph,
            finished,
            input,
            &mut *actual,
            request,
            &budget,
            &kernels,
            transform_threads,
            |y, row| output_sink.write_row(y, row),
        ),
        WorkingPrecision::F64Reference => execute_quality_srgb8(
            &graph,
            finished,
            input,
            &mut *actual,
            request,
            &budget,
            |y, row| output_sink.write_row(y, row),
        ),
    };
    let processed = processed.map_err(|error| output_sink.explain(error))?;
    let encode_started = Instant::now();
    let encoded_image = output_sink.finish(&budget)?;
    if encoded_image.format != options.output_format {
        return Err(ProcessingError::new(
            ProcessingErrorKind::EncodeFailed,
            "encoder backend returned a different output format",
        ));
    }
    if encoded_image.bytes.len() > options.max_output_bytes {
        return Err(ProcessingError::new(
            ProcessingErrorKind::ResourceExhausted,
            "encoded output exceeds max_output_bytes",
        ));
    }
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
        encoder_output_frame_bytes: encoded_image.frame_materialization_bytes,
        peak_managed_bytes: budget.snapshot().peak,
        opaque_decoder_memory: processed.decoder.opaque_codec_memory_unbounded,
        opaque_encoder_memory: encoded_image.opaque_memory,
        working_precision: options.precision,
        kernel_level,
        kernel_level_reason,
        encoder_threads,
        transform_threads,
        encoder_backend,
        timings: StageTimings {
            open,
            decode: processed.source_time,
            transform: processed.transform_time,
            encode,
            total: started.elapsed(),
        },
    };
    Ok(Output {
        format: options.output_format,
        bytes: encoded_image.bytes,
        report,
    })
}

/// One image of a batch: compressed bytes, their format and the options.
#[derive(Clone, Copy, Debug)]
pub struct BatchItem<'a> {
    pub encoded: &'a [u8],
    pub format: ImageFormat,
    pub options: TransformOptions,
}

/// Runs many images in parallel (ADR-0022). The engine schedules whole
/// images across its workers and runs each with one transform thread and
/// one encoder thread: across many images this keeps every core busy with
/// the smallest files (one AV1 tile per image), and the output of every
/// image is identical to `transcode` with those two options set to 1.
///
/// Each image keeps its own `max_managed_bytes` budget, so the engine's
/// managed memory is bounded by `threads` times the largest per-image budget.
#[derive(Clone, Copy, Debug)]
pub struct Engine {
    threads: usize,
}

impl Engine {
    /// An engine with `threads` workers; 0 means all available cores.
    #[must_use]
    pub fn new(threads: usize) -> Self {
        let threads = match threads {
            0 => std::thread::available_parallelism().map_or(1, std::num::NonZero::get),
            threads => threads,
        };
        Self { threads }
    }

    /// Number of workers.
    #[must_use]
    pub fn threads(&self) -> usize {
        self.threads
    }

    /// Transcode every item; results come back in input order. A failing
    /// image does not stop the others.
    #[must_use]
    pub fn transcode_batch(&self, items: &[BatchItem<'_>]) -> Vec<Result<Output, ProcessingError>> {
        batch(self.threads, items, None)
    }

    /// [`Engine::transcode_batch`] with another encoder (ADR-0024); every
    /// image gets its own encoder from `backend`.
    #[must_use]
    pub fn transcode_batch_with(
        &self,
        items: &[BatchItem<'_>],
        backend: &dyn EncoderBackend,
    ) -> Vec<Result<Output, ProcessingError>> {
        batch(self.threads, items, Some(backend))
    }
}

/// Schedule whole images over `threads` scoped workers.
fn batch(
    threads: usize,
    items: &[BatchItem<'_>],
    backend: Option<&dyn EncoderBackend>,
) -> Vec<Result<Output, ProcessingError>> {
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    let next = AtomicUsize::new(0);
    let results: Vec<Mutex<Option<Result<Output, ProcessingError>>>> =
        items.iter().map(|_| Mutex::new(None)).collect();
    let workers = threads.min(items.len()).max(1);
    std::thread::scope(|scope| {
        for _ in 0..workers {
            scope.spawn(|| {
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(item) = items.get(index) else { break };
                    let options = TransformOptions {
                        transform_threads: 1,
                        encoder_threads: 1,
                        ..item.options
                    };
                    let result = run(item.encoded, item.format, options, backend);
                    if let Ok(mut slot) = results[index].lock() {
                        *slot = Some(result);
                    }
                }
            });
        }
    });
    results
        .into_iter()
        .map(|slot| {
            slot.into_inner().ok().flatten().unwrap_or_else(|| {
                Err(ProcessingError::new(
                    ProcessingErrorKind::Internal,
                    "a batch worker stopped before finishing its image",
                ))
            })
        })
        .collect()
}
