//! First practical M1 facade. Codec implementations are opt-in and remain
//! private. Output is SDR AVIF, WebP or lossless JPEG XL; unsupported color
//! fails closed.

use std::fmt;
use std::time::{Duration, Instant};

use crate::color::ColorEncoding;
use crate::decoder::{DecodeAccess, DecodeReport, DecodeSource, DecodedRow, DecoderInfo};
use crate::error::Error;
use crate::fast_execute::execute_fast;
use crate::geometry::{Rect, Size};
use crate::graph::Graph;
#[cfg(feature = "jpeg-xl-write")]
use crate::jxl_sink::JxlSink;
use crate::memory::{ExternalReservation, MemoryBudget};
use crate::orientation::OrientedSource;
use crate::output_depth::OutputRow;
use crate::quality_execute::execute_quality;
#[cfg(feature = "webp-write")]
use crate::webp_sink::WebpSink;

pub use crate::decoder::WorkCount;
pub use crate::discarded::{Discarded, DiscardedSet};
pub use crate::encoder_backend::{
    EncodeError, EncodeErrorKind, EncodeRequest, EncodedImage, EncoderBackend, RowEncoder,
};
#[cfg(feature = "avif-write")]
pub use crate::own_avif::LibavifBackend;
pub use crate::quality::Filter as ResizeFilter;
pub use crate::resize_geometry::{ResizeFit, ResizeOptions};
pub use ferrisma_kernels::{IsaLevel as KernelLevel, LevelReason as KernelLevelReason};

/// Chroma layout of AVIF output ([`AvifOptions::chroma`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum ChromaSubsampling {
    /// Full-resolution chroma, the AVIF default.
    #[default]
    Yuv444,
    /// Half-resolution chroma in both directions, centered 2×2 average.
    Yuv420,
}

/// The encoded container format. WebP and JPEG XL have no selectable chroma
/// layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum OutputFormat {
    /// Still AVIF; requires `avif-write` (the workspace libavif/AOM writer) or
    /// an AVIF backend.
    #[default]
    Avif,
    /// Still WebP; requires `webp-write` or a WebP backend.
    WebP,
    /// A raw JPEG XL codestream (ADR-0039); requires `jpeg-xl-write`.
    JpegXl,
}

/// Settings for the built-in JPEG XL encoder (ADR-0039). The workspace
/// encoder admits two profiles today; every other value is rejected with
/// [`ProcessingErrorKind::InvalidOptions`] before any pixel is decoded, and
/// the encoder never falls back to another profile.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum JpegXlOptions {
    /// Lossless Modular encoding. Only effort 2 (libjxl's "thunder") is
    /// admitted by the workspace encoder.
    Lossless {
        /// libjxl effort level; only `2` is admitted today.
        effort: u8,
        /// Bits per sample, as [`AvifOptions::depth`]: `None` (the default)
        /// follows the input (8 to 16, `cjxl`'s rule), `Some(8..=16)` is
        /// explicit. Lossless color above 12 bits makes libjxl require
        /// codestream level 10, and the output is then the ISOBMFF container
        /// with a `jxll` box, as `cjxl` writes it. Added 2026-11-15; before,
        /// every output was 8-bit.
        depth: Option<u8>,
    },
    /// Lossy `VarDCT` encoding in XYB at libjxl effort 7 ("squirrel"),
    /// with libjxl's patch search always off (the workspace encoder cannot
    /// write the patch reference frame yet), so text-like content encodes
    /// too.
    ///
    /// Images with alpha below the largest code are admitted: the alpha
    /// channel is written as one straight (unassociated) extra channel at
    /// the color depth, coded losslessly, and color under fully transparent
    /// pixels is not kept (libjxl's `keep_invisible` default). Fully opaque
    /// alpha is omitted. Alpha above 12 bits makes the output a container
    /// (codestream level 10).
    Lossy {
        /// Butteraugli distance, libjxl's quality target: the maximum
        /// perceptual difference the encoder aims for, in units of a just
        /// noticeable difference. `1.0` is libjxl's default ("visually
        /// lossless"); lower is closer to the input and larger. Admitted:
        /// `0.05 <= distance < 3.0` (dot detection starts at 3.0 and is not
        /// translated yet).
        distance: f32,
        /// Bits per sample, as for [`Self::Lossless`]: `None` follows the
        /// input (8 to 16), `Some(8..=16)` is explicit.
        depth: Option<u8>,
    },
}

impl JpegXlOptions {
    /// Smallest admitted lossy distance (libjxl's `kMinButteraugliDistance`).
    pub const MIN_LOSSY_DISTANCE: f32 = 0.05;
    /// Exclusive upper bound of the admitted lossy distance.
    pub const MAX_LOSSY_DISTANCE: f32 = 3.0;

    /// The depth setting of either profile.
    #[must_use]
    pub fn depth(&self) -> Option<u8> {
        match *self {
            Self::Lossless { depth, .. } | Self::Lossy { depth, .. } => depth,
        }
    }

    /// The same profile with another depth setting.
    #[must_use]
    pub fn with_depth(self, depth: Option<u8>) -> Self {
        match self {
            Self::Lossless { effort, .. } => Self::Lossless { effort, depth },
            Self::Lossy { distance, .. } => Self::Lossy { distance, depth },
        }
    }
}

impl Default for JpegXlOptions {
    /// Lossy at distance 1.0, libjxl's own default (ADR-0039; pending owner
    /// confirmation).
    fn default() -> Self {
        Self::Lossy {
            distance: 1.0,
            depth: None,
        }
    }
}

/// Settings for the built-in still WebP encoder, on libwebp's own scales
/// (ADR-0043 D4: no scale is mapped onto another encoder's). Each libwebp
/// concept has one control: lossy output takes libwebp's `quality` and
/// `method` (`cwebp -q`, `-m`); lossless output takes libwebp's lossless
/// preset level (`cwebp -z`), which sets libwebp's method and quality
/// itself. Must stay at the default for other outputs.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum WebpOptions {
    /// Lossy VP8 at libwebp's quality and method.
    Lossy {
        /// libwebp quality, 1 through 100; default 75 (libwebp's).
        quality: f32,
        /// libwebp method, 0 (fastest) through 6 (slowest, smallest);
        /// default 4 (libwebp's `WebPConfigInit`, `cwebp -m`). Every value
        /// is passed unchanged. Until 2026-11-15 this was a `speed` 1–10
        /// mapped onto the methods, which gave three pairs of speeds the
        /// same bytes.
        method: u8,
    },
    /// Lossless VP8L at libwebp's preset level (`WebPConfigLosslessPreset`):
    /// 0 is fastest, 9 gives the smallest files.
    Lossless {
        /// Preset level from 0 through 9.
        level: u8,
    },
}

impl WebpOptions {
    /// libwebp's default lossy method (`WebPConfigInit`).
    pub const DEFAULT_METHOD: u8 = 4;
    /// The slowest lossy method libwebp has.
    pub const MAX_METHOD: u8 = 6;
}

impl Default for WebpOptions {
    /// Lossy at libwebp's defaults: quality 75, method 4.
    fn default() -> Self {
        Self::Lossy {
            quality: 75.0,
            method: Self::DEFAULT_METHOD,
        }
    }
}

/// Settings for the built-in AVIF writer, libavif with its AOM encoder
/// (ADR-0037), on libavif's own scales (ADR-0043 D4). The defaults are those
/// of the original tool, `avifenc` of the pinned libavif 1.4.2 (ADR-0037,
/// 2026-11-15). Must stay at the default for other outputs. External AVIF backends receive it in
/// [`EncodeRequest::avif`] and validate it on their own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AvifOptions {
    /// Color quality, libavif's `avifEncoder::quality`: 0 (worst) through
    /// 100 (lossless quantizer); default 60 (`avifenc`).
    pub quality: u8,
    /// Alpha quality, libavif's `avifEncoder::qualityAlpha`, on the same
    /// scale; used only when an alpha sample is below 255; default 60.
    /// Explicit: `avifenc` copies the color quality into it when it is not
    /// given, so its default is the color default, and this field does not
    /// follow `quality` when only that one is changed.
    pub alpha_quality: u8,
    /// AOM speed (`cpu-used`), 0 (slowest) through 10 (fastest); default 6
    /// (`avifenc`).
    pub speed: u8,
    /// AV1 bit depth: `None` (the default) follows the input, as `avifenc`
    /// does: the largest of 8, 10 and 12 that is at most the input's depth
    /// (a 16-bit input becomes 12, AVIF's largest, as `avifenc` writes a
    /// 16-bit PNG). `Some(8 | 10 | 12)` is explicit; another value is
    /// refused. The samples are quantized at that depth by the engine (owner
    /// decision of 2026-11-15, ADR-0021 amendment): a deeper input keeps
    /// its precision, and a higher explicit depth carries correctly scaled
    /// content. [`Discarded::PrecisionReduced`] is reported exactly when the
    /// output depth is below the input's. Until 2026-11-15 this was a `u8`,
    /// 8 or 10 (default 8), and every AVIF carried 8-bit content.
    pub depth: Option<u8>,
    /// Chroma layout; default 4:4:4 (`avifenc` for RGB input).
    pub chroma: ChromaSubsampling,
}

impl Default for AvifOptions {
    fn default() -> Self {
        Self {
            quality: 60,
            alpha_quality: 60,
            speed: 6,
            depth: None,
            chroma: ChromaSubsampling::Yuv444,
        }
    }
}

/// Accepted input codecs. Compile the corresponding Cargo features to enable
/// each adapter; no transitive codec dependencies are required by the core.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ImageFormat {
    /// JPEG; requires `jpeg-read`.
    Jpeg,
    /// Still WebP; requires `webp-read`.
    WebP,
    /// JPEG XL; requires `jpeg-xl-read` (the workspace libjxl-rs reader).
    JpegXl,
    /// Still AVIF; requires `avif-read`.
    Avif,
}

/// Source color handling policy; the default never guesses a color profile.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum InputProfilePolicy {
    /// Require unambiguous decoder-reported sRGB; never assume or convert.
    #[default]
    Strict,
    /// Explicitly assume sRGB only when the decoder reports *no* output ICC
    /// and *no* output CICP. This must be an informed caller decision.
    AssumeSrgbIfMissing,
    /// Explicitly use the decoder-provided *rendered output* ICC as authoritative.
    /// Requires the optional `icc` feature and supports opaque RGB and RGB
    /// with straight alpha (alpha passes through the color transform
    /// unchanged; premultiplied or unspecified alpha is refused). Input
    /// without a profile whose typed signal is sRGB, Display P3 or linear
    /// sRGB (AVIF `nclx`, JPEG XL enum encoding) is converted as under
    /// [`Self::Strict`] (since 2026-11-15).
    UseRenderedIcc,
}

/// Working precision of the resample chain (ADR-0021). Both produce the same
/// image within ADR-0021's bound; the reference exists for verification and
/// before/after measurements, not for production use.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
#[non_exhaustive]
pub enum WorkingPrecision {
    /// Scalar `f32` executor with table-driven transfer functions.
    #[default]
    F32,
    /// The M1 `f64` reference executor, the oracle of the differential tests.
    F64Reference,
}

/// A crop rectangle in upright display coordinates, after orientation.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Crop {
    /// Left edge in pixels.
    pub x: u32,
    /// Top edge in pixels.
    pub y: u32,
    /// Width in pixels; must be nonzero.
    pub width: u32,
    /// Height in pixels; must be nonzero.
    pub height: u32,
}

/// Explicit output geometry, quality, profile assumption and memory budget.
/// Encoder scratch allocations are opaque and not constrained by the budget.
#[derive(Clone, Copy, Debug)]
pub struct TransformOptions {
    /// Encoded output format. Each format has its own settings below, on
    /// its encoder's own scales; the settings of the other formats must
    /// stay at their defaults, or the transcode is refused before decoding
    /// (until 2026-11-15 the AVIF settings were the format-independent
    /// looking `quality`, `speed`, `chroma_subsampling` and `avif_depth`).
    pub output_format: OutputFormat,
    /// AVIF settings; must stay at the default for other formats.
    pub avif: AvifOptions,
    /// WebP settings; must stay at the default for other formats.
    pub webp: WebpOptions,
    /// JPEG XL settings; must stay at the default for other formats.
    pub jpeg_xl: JpegXlOptions,
    /// Exact output width. [`transcode_resized`] and
    /// [`transcode_resized_with`] take the size from [`ResizeOptions`] and
    /// refuse a value other than the default (1) with
    /// [`ProcessingErrorKind::InvalidOptions`] instead of ignoring it.
    pub output_width: u32,
    /// Exact output height; as `output_width`, refused by the resize entry
    /// points unless it is the default (1).
    pub output_height: u32,
    /// Optional crop in upright display coordinates, applied before resizing.
    pub crop: Option<Crop>,
    /// Resampling filter.
    pub filter: ResizeFilter,
    /// Working precision of the resample chain.
    pub precision: WorkingPrecision,
    /// Highest SIMD kernel level to use; `None` picks the best the machine
    /// supports up to AVX2 on x86-64 and NEON on aarch64
    /// (`ferrisma_kernels::DEFAULT_CEILING`, decision D6 of ADR-0020), so
    /// the default output does not depend on AVX-512. `Some(KernelLevel::Avx512)`
    /// is the explicit opt-in to AVX-512. Changing it never changes the
    /// result beyond the ADR-0021 bound; it exists for verification,
    /// benchmarks and bug reports.
    pub kernel_level_ceiling: Option<KernelLevel>,
    /// Threads for the decode-transform row pipeline (ADR-0022): 0 means
    /// all available cores (capped at 256), 1 the sequential path; explicit
    /// values must be at most 256. The output is
    /// byte-identical for every value.
    pub transform_threads: usize,
    /// How missing or rendered ICC color is handled.
    pub profile_policy: InputProfilePolicy,
    /// AV1 encoder threads: 0 means available cores (capped at 256), and
    /// explicit values must be at most 256. libavif uses automatic tiling and
    /// its encoded bytes can depend on the selected thread count. The
    /// built-in WebP and JPEG XL encoders are serial and require 1.
    pub encoder_threads: usize,
    /// Engine-managed memory budget in bytes. Opaque codec scratch memory is
    /// not bounded by it; see [`ProcessingReport`].
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
            avif: AvifOptions::default(),
            webp: WebpOptions::default(),
            jpeg_xl: JpegXlOptions::default(),
            output_width: 1,
            output_height: 1,
            crop: None,
            filter: ResizeFilter::Lanczos3,
            precision: WorkingPrecision::F32,
            kernel_level_ceiling: None,
            transform_threads: 0,
            profile_policy: InputProfilePolicy::Strict,
            encoder_threads: 1,
            // 1 GiB since 2026-11-15 (owner decision, ADR-0004 amendment);
            // 256 MiB before.
            max_managed_bytes: 1024 * 1024 * 1024,
            max_input_bytes: 64 * 1024 * 1024,
            max_input_pixels: 100_000_000,
            max_output_bytes: 256 * 1024 * 1024,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
/// How the decoder delivered the requested source region.
pub enum SourceAccess {
    /// The decoder produced the full frame; the engine selected the region.
    FullFrameFallback,
    /// The decoder honored a region request.
    CodecRegionRequest,
}

/// Wall-clock time per pipeline stage, measured with `std::time::Instant`.
/// These are observations of one run on one machine, not throughput claims;
/// the stages overlap only where documented on each field.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
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

/// What the decoder was asked for, what it returned and which work it
/// necessarily performed (ADR-0012, ADR-0023). Counts are pixels and rows,
/// not time. Only [`WorkCount::Exact`] values bound the work from above: a
/// codec that returns a crop may still have decoded far more.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct DecoderWork {
    /// The upright source crop that feeds the resize, in display
    /// coordinates; the whole image when no crop was requested.
    pub crop: Crop,
    /// The upright rectangle the executor requested from the source. Filters
    /// clamp at the crop's edges, so it equals `crop`: there is no filter
    /// halo beyond the crop and a difference would be a planning defect.
    pub requested: Crop,
    /// The rectangle whose samples the codec adapter delivers, in the codec's
    /// own frame: stored order when the engine reorients (JPEG, WebP, AVIF;
    /// ADR-0019), upright display coordinates when the decoder applies the
    /// orientation itself (both JPEG XL backends). Context an adapter adds
    /// to its codec call, such as the JPEG adapter's extra column on each
    /// side for chroma upsampling, is not part of it; it appears in
    /// `codec_output` and therefore in `overfetch_pixels`.
    pub codec_request: Crop,
    /// The rectangle the codec actually wrote, in the same frame as
    /// `codec_request`, including alignment, filter context and full-frame
    /// fallbacks.
    pub codec_output: Crop,
    /// `codec_output` pixels beyond `codec_request`: the source overfetch.
    pub overfetch_pixels: u64,
    /// Pixels the codec reconstructed, as far as the adapter can establish.
    pub reconstructed_pixels: WorkCount,
    /// Stored rows whose compressed data the codec parsed.
    pub entropy_rows: WorkCount,
    /// Rows the source handed to the executor. Equal to `requested.height`
    /// when no source row is decoded or delivered twice.
    pub delivered_rows: u64,
    /// Sample bytes copied between engine buffers by source wrappers; the
    /// reorientation fill and gather each copy the region once (ADR-0019).
    pub wrapper_copied_bytes: u64,
}

fn public_rect(rect: Rect) -> Crop {
    Crop {
        x: rect.x,
        y: rect.y,
        width: rect.width,
        height: rect.height,
    }
}

/// The color signal that described the decoded samples when the engine
/// converted them into its linear-sRGB working space (ADR-0015, issue
/// #212). ICC and CICP are never ranked against each other: a conversion
/// either uses one signal, or uses the rendered ICC profile after checking
/// that the CICP beside it agrees.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum ColorSignal {
    /// The decoder's typed signal alone (sRGB, Display P3 or linear sRGB):
    /// AVIF `nclx` or a JPEG XL enum color encoding; the decoder reported no
    /// ICC profile. The same under every policy.
    Cicp,
    /// The decoder reported neither ICC nor CICP, and
    /// [`InputProfilePolicy::AssumeSrgbIfMissing`] supplied sRGB.
    AssumedSrgb,
    /// [`InputProfilePolicy::UseRenderedIcc`]: the decoder's rendered ICC
    /// profile was converted; the decoder reported no CICP beside it. A
    /// well-formed `cicp` tag inside the profile is checked as part of the
    /// profile.
    RenderedIcc,
    /// [`InputProfilePolicy::UseRenderedIcc`]: the rendered ICC profile was
    /// converted after the decoder's CICP beside it was checked and found to
    /// agree with the profile transform. Disagreement is refused with
    /// [`RefusalReason::CicpBesideIcc`]; neither signal takes precedence.
    /// No built-in reader reports CICP beside a profile since 2026-11-15
    /// (AVIF follows MIAF: the profile governs, owner decision), so this
    /// signal is kept for a reader that does.
    RenderedIccWithAgreeingCicp,
}

/// Report known materializations separately from opaque codec memory.
#[derive(Clone, Copy, Debug)]
#[non_exhaustive]
pub struct ProcessingReport {
    /// How the source region was obtained.
    pub input_access: SourceAccess,
    /// Number of region requests sent to the decoder.
    pub decoder_requests: u32,
    /// Known full decoded frames materialized by the decoder adapter.
    pub decoder_full_frame_buffers: u32,
    /// Full-frame intermediates materialized by the engine.
    pub engine_full_frame_intermediates: u32,
    /// Engine-managed bytes materialized to reorient the input; zero for
    /// upright input.
    pub orientation_buffer_bytes: usize,
    /// Known pixel-frame bytes materialized by the selected encoder. libavif
    /// reports its concurrent RGBA and YUV(A) frames. Known bytes are charged to the managed budget.
    pub encoder_output_frame_bytes: usize,
    /// Peak engine-managed bytes charged against `max_managed_bytes`.
    pub peak_managed_bytes: usize,
    /// The decoder allocated memory that the engine cannot observe or bound.
    pub opaque_decoder_memory: bool,
    /// The encoder allocated memory that the engine cannot observe or bound.
    pub opaque_encoder_memory: bool,
    /// Requested versus decoded source regions and the decoder's work.
    pub decoder_work: DecoderWork,
    /// The color signal the conversion into the working space used.
    pub color_signal: ColorSignal,
    /// The executor that produced the pixels.
    pub working_precision: WorkingPrecision,
    /// SIMD level of the resample kernels and why it was chosen; the `f64`
    /// reference executor always reports `Scalar`.
    pub kernel_level: KernelLevel,
    /// Why `kernel_level` was selected.
    pub kernel_level_reason: KernelLevelReason,
    /// Encoder threads actually used (resolved when the option was 0).
    pub encoder_threads: usize,
    /// Transform pipeline threads actually used; 1 for the sequential path
    /// and the `f64` reference.
    pub transform_threads: usize,
    /// The decoder that read the input and produced the source pixels
    /// (issue #217). Built-in readers: `"libjpeg-turbo-rs"` (JPEG),
    /// `"libwebp-rs"` (WebP), `"libjxl-rs"` (JPEG XL) and `"libavif-rs/aom"`
    /// (AVIF), one per format. The strings are observations; new readers add
    /// new values.
    pub decoder_backend: &'static str,
    /// The encoder that wrote the output: `"libavif-rs/aom"` (AVIF),
    /// `"libwebp-rs"` (WebP), `"libjxl-rs"` (JPEG XL), or the
    /// [`EncoderBackend::name`] of an external backend (ADR-0024).
    pub encoder_backend: &'static str,
    /// Bits per sample of the decoded input, color and alpha (the larger):
    /// 8 for JPEG and WebP, 8, 10 or 12 for AVIF, 1 to 16 for JPEG XL
    /// (since 2026-11-15, ADR-0021 amendment).
    pub input_depth: u8,
    /// Bits per sample written: the input's by default, capped at what the
    /// output admits (AVIF 12, JPEG XL 16, WebP 8), or the explicit depth
    /// setting. When it is below `input_depth`, `discarded` names
    /// [`Discarded::PrecisionReduced`] (since 2026-11-15).
    pub output_depth: u8,
    /// The classes of input information this transcode left out of a
    /// correct picture (ADR-0042 D2); empty when nothing was left out.
    /// Experimental; see [`Discarded`].
    pub discarded: DiscardedSet,
    /// Wall-clock observations per stage; not a performance claim.
    pub timings: StageTimings,
}

/// One encoded image and its processing report.
#[derive(Debug)]
#[non_exhaustive]
pub struct Output {
    /// Format of `bytes`.
    pub format: OutputFormat,
    /// The complete encoded file.
    pub bytes: Vec<u8>,
    /// Actual encoded dimensions, which may be smaller than requested.
    pub width: u32,
    /// Actual encoded height.
    pub height: u32,
    /// Materialization, memory, backend, and timing observations.
    pub report: ProcessingReport,
}

/// Broad failure categories a caller can match on. The message carries the
/// details; new categories may be added before a stable release.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProcessingErrorKind {
    /// Options are invalid: an encoder setting outside its scale or for
    /// another output, zero limits, output geometry, a crop outside the
    /// image, or a geometry calculation that overflows.
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
    /// An internal invariant failed; this indicates a Ferrisma bug.
    Internal,
}

/// Why an input was refused, beyond its [`ProcessingErrorKind`] (issue
/// #212, ADR-0017 amendments of 2026-11-14 and 2026-11-15). A closed
/// vocabulary by refusal class, not one value per message: each variant
/// names one class of well-formed but unsupported input that the engine
/// detects itself, and callers match on it instead of on message text.
/// Malformed data, limits, I/O, invalid options and internal failures carry
/// no reason. New reasons may be added; an existing reason keeps its
/// meaning. Where one input meets several classes, the reason names the
/// first check that refused it (the order is documented in
/// `docs/supported-matrix.md`), not every class it belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum RefusalReason {
    /// The input is animated; only still images are decoded and there is no
    /// first-frame fallback. Kind: [`ProcessingErrorKind::UnsupportedInput`].
    Animation,
    /// The decoder reports neither an ICC profile nor CICP for its output,
    /// and the policy is [`InputProfilePolicy::Strict`]. A caller who knows
    /// the input is sRGB may choose
    /// [`InputProfilePolicy::AssumeSrgbIfMissing`]. Kind:
    /// [`ProcessingErrorKind::UnsupportedColor`].
    ColorSignalMissing,
    /// The decoder describes its output with an ICC profile, and the policy
    /// is not [`InputProfilePolicy::UseRenderedIcc`] (which requires the
    /// `icc` feature). Kind: [`ProcessingErrorKind::UnsupportedColor`].
    RenderedIccRequiresOptIn,
    /// Under [`InputProfilePolicy::UseRenderedIcc`], CICP beside the
    /// rendered ICC profile (the profile's own `cicp` tag, or a decoder's
    /// CICP beside it) signals HDR, cannot be checked, or disagrees with
    /// the profile; the profile is not given precedence (ADR-0015 rule 4).
    /// AVIF `nclx` beside a profile is not such evidence since 2026-11-15:
    /// MIAF gives the profile the primaries and transfer. Kind:
    /// [`ProcessingErrorKind::UnsupportedColor`].
    CicpBesideIcc,
    /// The decoder signals a PQ or HLG transfer (JPEG XL enum encoding,
    /// AVIF CICP), which needs reference-white and tone-mapping decisions
    /// the engine does not make (ADR-0016 rule 7). Refused under every
    /// policy. A PQ/HLG `cicp` beside a rendered ICC profile is
    /// [`Self::CicpBesideIcc`] instead. Kind:
    /// [`ProcessingErrorKind::UnsupportedColor`].
    HdrTransfer,
    /// The decoder describes its output with an SDR color encoding the
    /// engine cannot convert: CICP outside sRGB, Display P3 and linear sRGB
    /// without an ICC profile. Kind:
    /// [`ProcessingErrorKind::UnsupportedColor`].
    UnsupportedColorEncoding,
    /// The policy is [`InputProfilePolicy::UseRenderedIcc`], but the
    /// decoder reports no ICC profile: untagged input, or a typed signal
    /// outside sRGB, Display P3 and linear sRGB (those three are converted
    /// as under the strict policy since 2026-11-15). Kind:
    /// [`ProcessingErrorKind::UnsupportedColor`].
    RenderedIccMissing,
    /// The AVIF container's color signals (`nclx`, ICC) contradict the AV1
    /// sequence header or libavif's parsed image. Kind:
    /// [`ProcessingErrorKind::UnsupportedColor`].
    ContradictoryColorSignals,
    /// The channel layout, sample format or YUV matrix is outside the
    /// engine's contract: CMYK/YCCK JPEG, constant-luminance BT.2020 AVIF,
    /// CMYK, float or extra channels in JPEG XL. (Grayscale JPEG XL and
    /// monochrome AVIF are accepted since 2026-11-15 and expanded to RGB,
    /// as grayscale JPEG is.) Kind: [`ProcessingErrorKind::UnsupportedInput`].
    UnsupportedLayout,
    /// The alpha channel is premultiplied (associated) or its association
    /// is unspecified; only straight alpha is processed. Kind:
    /// [`ProcessingErrorKind::UnsupportedInput`].
    AlphaAssociation,
    /// A geometric transform the engine does not apply: AVIF clean aperture
    /// (`clap`, essential or not), `imir` associated before `irot`, or an
    /// EXIF orientation outside 1 to 8. Kind:
    /// [`ProcessingErrorKind::UnsupportedInput`].
    GeometricTransform,
    /// The container marks a property of the image as essential (ISO/IEC
    /// 23008-12 section 10.2.1: a reader must understand it to process the
    /// image) and the engine does not understand it; today an unknown
    /// essential property of an AVIF item the reader uses (the primary
    /// item, grid cells, the alpha plane; ADR-0042 D1). The file is well-formed,
    /// so the kind is [`ProcessingErrorKind::UnsupportedInput`], not
    /// `MalformedInput`.
    UnsupportedEssentialProperty,
    /// Under [`InputProfilePolicy::UseRenderedIcc`], the rendered ICC
    /// profile describes another color space than the decoded channels: an
    /// RGB profile beside gray samples, or a gray (`GRAY`) profile beside
    /// RGB samples (since 2026-11-15). A gray profile beside gray samples is
    /// converted. Kind: [`ProcessingErrorKind::UnsupportedColor`].
    IccColorSpaceMismatch,
}

/// Public error that never exposes backend-specific types.
#[derive(Debug)]
pub struct ProcessingError {
    kind: ProcessingErrorKind,
    reason: Option<RefusalReason>,
    message: String,
}
impl ProcessingError {
    fn new(kind: ProcessingErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            reason: None,
            message: message.into(),
        }
    }

    fn refused(
        kind: ProcessingErrorKind,
        reason: Option<RefusalReason>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            reason,
            ..Self::new(kind, message)
        }
    }

    /// The failure category, for programmatic handling.
    #[must_use]
    pub fn kind(&self) -> ProcessingErrorKind {
        self.kind
    }

    /// The reason of a refusal, when the engine assigns one of the
    /// documented [`RefusalReason`]s; `None` for every other error and for
    /// refusals that have no stable reason yet (message text is never
    /// stable).
    #[must_use]
    pub fn refusal_reason(&self) -> Option<RefusalReason> {
        self.reason
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
            #[cfg(any(feature = "avif-read", feature = "webp-read", feature = "jpeg-xl-read"))]
            Error::InputPixelLimitExceeded { .. } => Kind::InputLimitExceeded,
            Error::DecodeFailed
            | Error::IncompleteDecoderOutput
            | Error::NonFinitePixelSample
            | Error::InvalidAlphaValue => Kind::MalformedInput,
            Error::UnsupportedAlphaMode
            | Error::UnsupportedCodecLayout
            | Error::UnsupportedLayout
            | Error::UnsupportedPixelOperation
            | Error::ChromaPhaseRequiresResampling => Kind::UnsupportedInput,
            Error::UnsupportedAnimation => Kind::UnsupportedInput,
            Error::UnsupportedImageTransform => Kind::UnsupportedInput,
            Error::UnsupportedColorEncoding | Error::HdrTransfer => Kind::UnsupportedColor,
            #[cfg(feature = "icc")]
            Error::CicpBesideIcc(_) | Error::RenderedIccMissing | Error::IccColorSpaceMismatch => {
                Kind::UnsupportedColor
            }
            #[cfg(feature = "avif-read")]
            Error::ContradictoryColorSignals => Kind::UnsupportedColor,
            #[cfg(feature = "avif-read")]
            Error::UnsupportedEssentialProperty(_) => Kind::UnsupportedInput,
            Error::AllocationFailed | Error::ThreadSpawnFailed | Error::BudgetExceeded { .. } => {
                Kind::ResourceExhausted
            }
            Error::Cancelled => Kind::Cancelled,
            Error::EncodeFailed => Kind::EncodeFailed,
            Error::IncompleteOutput => Kind::Internal,
            #[cfg(feature = "jpeg-xl-write")]
            Error::UnsupportedEncoderProfile(_) => Kind::InvalidOptions,
            Error::UnknownNode
            | Error::PixelFormatMismatch
            | Error::SourceBindingMismatch
            | Error::InvalidPlaneStride
            | Error::InvalidPlaneDimensions
            | Error::InsufficientPlaneStorage => Kind::Internal,
        };
        let reason = match &value {
            Error::UnsupportedAnimation => Some(RefusalReason::Animation),
            #[cfg(feature = "icc")]
            Error::CicpBesideIcc(_) => Some(RefusalReason::CicpBesideIcc),
            #[cfg(feature = "icc")]
            Error::RenderedIccMissing => Some(RefusalReason::RenderedIccMissing),
            #[cfg(feature = "icc")]
            Error::IccColorSpaceMismatch => Some(RefusalReason::IccColorSpaceMismatch),
            Error::HdrTransfer => Some(RefusalReason::HdrTransfer),
            #[cfg(feature = "avif-read")]
            Error::ContradictoryColorSignals => Some(RefusalReason::ContradictoryColorSignals),
            #[cfg(feature = "avif-read")]
            Error::UnsupportedEssentialProperty(_) => {
                Some(RefusalReason::UnsupportedEssentialProperty)
            }
            Error::UnsupportedLayout => Some(RefusalReason::UnsupportedLayout),
            Error::UnsupportedAlphaMode => Some(RefusalReason::AlphaAssociation),
            Error::UnsupportedImageTransform => Some(RefusalReason::GeometricTransform),
            // `UnsupportedColorEncoding` and `UnsupportedCodecLayout` remain
            // for profile, transform and wrapper failures without a
            // documented class; the engine-internal pixel-operation errors
            // are not refusals of an input.
            _ => None,
        };
        Self::refused(kind, reason, value.to_string())
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
    #[cfg(not(any(feature = "avif-read", feature = "webp-read", feature = "jpeg-xl-read")))]
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
        ImageFormat::JpegXl => {
            #[cfg(feature = "jpeg-xl-read")]
            {
                Ok(Box::new(crate::codecs::jxl::JxlSource::new_bounded(
                    bytes, max_pixels,
                )?))
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

/// The name reported in [`ProcessingReport::decoder_backend`] for the reader
/// that [`open_source`] opens for `format`; only called after that reader
/// was opened, so the name always belongs to a compiled backend.
fn decoder_backend(format: ImageFormat) -> &'static str {
    match format {
        ImageFormat::Jpeg => "libjpeg-turbo-rs",
        ImageFormat::WebP => "libwebp-rs",
        ImageFormat::JpegXl => "libjxl-rs",
        ImageFormat::Avif => "libavif-rs/aom",
    }
}

/// Limits applied before and after parsing a JPEG or WebP header.
#[derive(Clone, Copy, Debug)]
pub struct InspectionLimits {
    /// Maximum compressed input size in bytes; must be nonzero.
    pub max_input_bytes: usize,
    /// Maximum stored pixel count; must be nonzero.
    pub max_input_pixels: u64,
}

impl Default for InspectionLimits {
    fn default() -> Self {
        let options = TransformOptions::default();
        Self {
            max_input_bytes: options.max_input_bytes,
            max_input_pixels: options.max_input_pixels,
        }
    }
}

/// Metadata for samples emitted by a JPEG or WebP decoder, before any
/// profile assumption or color transform. Dimensions distinguish stored from
/// upright display coordinates; ICC and CICP are evidence, not a conversion.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ImageMetadata {
    /// Coded width before orientation.
    pub stored_width: u32,
    /// Coded height before orientation.
    pub stored_height: u32,
    /// Upright display width after orientation.
    pub width: u32,
    /// Upright display height after orientation.
    pub height: u32,
    /// The decoder emits an alpha channel.
    pub has_alpha: bool,
    /// ICC profile describing the decoder's emitted samples, if any.
    pub rendered_icc: Option<Vec<u8>>,
    /// CICP (primaries, transfer, matrix, full range) of the emitted samples, if any.
    pub rendered_cicp: Option<[u8; 4]>,
}

/// Inspect JPEG or WebP headers without requesting pixel decoding. Limits do
/// not bound opaque codec allocations, and a valid header does not guarantee
/// a complete decodable image or a supported output color.
///
/// # Errors
/// Returns an error for invalid limits, unsupported formats, missing codec
/// features, malformed headers, or an exceeded input limit; a caught panic
/// as [`ProcessingErrorKind::Internal`], as for [`transcode`].
pub fn inspect(
    encoded: &[u8],
    format: ImageFormat,
    limits: InspectionLimits,
) -> Result<ImageMetadata, ProcessingError> {
    contain_panic(|| inspect_headers(encoded, format, limits))
}

fn inspect_headers(
    encoded: &[u8],
    format: ImageFormat,
    limits: InspectionLimits,
) -> Result<ImageMetadata, ProcessingError> {
    if limits.max_input_bytes == 0 || limits.max_input_pixels == 0 {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "input byte/pixel limits must be nonzero",
        ));
    }
    if encoded.len() > limits.max_input_bytes {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InputLimitExceeded,
            "compressed image exceeds max_input_bytes",
        ));
    }
    if !matches!(format, ImageFormat::Jpeg | ImageFormat::WebP) {
        return Err(ProcessingError::new(
            ProcessingErrorKind::UnsupportedInput,
            "header inspection supports JPEG and WebP",
        ));
    }
    let source = open_source(encoded, format, limits.max_input_pixels)?;
    let info = source.info();
    let stored = info.size;
    if u64::from(stored.width) * u64::from(stored.height) > limits.max_input_pixels {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InputLimitExceeded,
            "image exceeds max_input_pixels",
        ));
    }
    let display = source.orientation().display_size(stored)?;
    Ok(ImageMetadata {
        stored_width: stored.width,
        stored_height: stored.height,
        width: display.width,
        height: display.height,
        has_alpha: info.layout == crate::decoder::ChannelLayout::Rgba,
        rendered_icc: info.rendered_icc.clone(),
        rendered_cicp: info.rendered_cicp,
    })
}

/// Convert one real input to SDR AVIF. Quality and profile constraints are
/// enforced before executing a pixel transform, and only *one* decoder region
/// is requested for the output. The encoder's known input frames are
/// reserved in the managed budget and reported.
///
/// # Errors
/// Returns an error for unsupported codec features, ambiguous profiles,
/// invalid geometry/options, memory exhaustion or encoder failures. A panic
/// inside the engine, a codec or an encoder backend does not unwind out of
/// this function or any other entry point: it is returned as
/// [`ProcessingErrorKind::Internal`] (since 2026-11-16; with
/// `panic = "abort"` the process aborts instead).
pub fn transcode(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
) -> Result<Output, ProcessingError> {
    guarded_run(encoded, format, options, None, None)
}

/// [`transcode`] with another encoder (ADR-0024): the same decode, color,
/// crop, resample and quantization, then the output rows go to `backend`
/// instead of the built-in sink. Every encoder setting (`avif`, `webp`,
/// `jpeg_xl`, `encoder_threads`) is passed to the backend in an
/// [`EncodeRequest`]; the backend validates it on its own scale and refuses
/// what it cannot honor (since 2026-11-15 the AVIF depth is part of the
/// request, in [`AvifOptions`]).
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
    guarded_run(encoded, format, options, Some(backend), None)
}

/// Transcode using aspect-aware thumbnail geometry. The size comes from
/// `resize`; the `output_width` and `output_height` fields of `options` must
/// stay at their defaults. All other options apply. See [`ResizeOptions`]
/// and ADR-0034 for integer rounding and crop semantics.
///
/// # Errors
/// Returns the same errors as [`transcode`], including invalid resize
/// geometry, and [`ProcessingErrorKind::InvalidOptions`] when
/// `output_width` or `output_height` differs from its default (until
/// 2026-11-15 they were ignored).
pub fn transcode_resized(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
    resize: ResizeOptions,
) -> Result<Output, ProcessingError> {
    guarded_run(encoded, format, options, None, Some(resize))
}

/// [`transcode_resized`] with a caller-provided encoder backend.
///
/// # Errors
/// Returns the same errors as [`transcode_with`] and [`transcode_resized`].
pub fn transcode_resized_with(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
    resize: ResizeOptions,
    backend: &dyn EncoderBackend,
) -> Result<Output, ProcessingError> {
    guarded_run(encoded, format, options, Some(backend), Some(resize))
}

/// The panic boundary of the public entry points (ADR-0040 amendment of
/// 2026-11-16, working decision, delegated, revisable by the owner): a panic
/// inside `body` is returned as [`ProcessingErrorKind::Internal`] instead of
/// unwinding into the caller. A panic is still a defect; this only keeps it
/// from taking the caller's thread (or a whole batch) with it. Under
/// `panic = "abort"` nothing unwinds and the process aborts, as before.
///
/// Why `AssertUnwindSafe` is sound here, that is, why nothing observed after
/// the catch can be inconsistent:
/// - Everything `run` and `inspect` mutate is created inside the call and
///   dropped while unwinding: the `MemoryBudget` (one per call, never shared
///   between calls or batch items), its reservations (released by `Drop`),
///   the graph, the sink, the decoder and encoder contexts, and the scoped
///   transform workers (`std::thread::scope` joins them and re-raises a
///   worker's panic on this thread, where it is caught). No budget accounting
///   outlives the call, so a later call starts from a fresh budget.
/// - What the caller passes in is read-only to the engine: the input bytes,
///   the `Copy` options and `&dyn EncoderBackend`. The engine holds no lock
///   or counter of its own across the boundary that a panic could leave
///   poisoned or half-updated (the batch's result slots are written only
///   after the catch, with the item's result).
/// - Process-wide state is one-time initialization only: `OnceLock` tables
///   (the resample tables, the kernels' detected ISA level), which a panic
///   during initialization leaves uninitialized for the next caller to
///   retry, and the codec ports' once-initialized dispatch tables. A panic
///   that would have to cross the AVIF port's C-ABI `pthread_once` does not
///   unwind through it; the process aborts there instead of being caught.
/// - Not covered: the state of a caller-supplied [`EncoderBackend`] after
///   its own panic is the backend's concern; in a batch the same backend
///   serves the other items.
fn contain_panic<T>(
    body: impl FnOnce() -> Result<T, ProcessingError>,
) -> Result<T, ProcessingError> {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(body)).unwrap_or_else(|payload| {
        let detail = payload
            .downcast_ref::<&str>()
            .copied()
            .or_else(|| payload.downcast_ref::<String>().map(String::as_str));
        Err(ProcessingError::new(
            ProcessingErrorKind::Internal,
            match detail {
                Some(detail) => {
                    format!(
                        "a panic was caught inside ferrisma (a defect, please report it): {detail}"
                    )
                }
                None => {
                    "a panic was caught inside ferrisma (a defect, please report it)".to_owned()
                }
            },
        ))
    })
}

/// [`run`] behind the panic boundary of the public entry points.
fn guarded_run(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
    backend: Option<&dyn EncoderBackend>,
    resize: Option<ResizeOptions>,
) -> Result<Output, ProcessingError> {
    contain_panic(|| run(encoded, format, options, backend, resize))
}

/// Thread counts above 256 are invalid everywhere, in a batch too (owner
/// decision of 2026-11-15: an option is refused rather than ignored).
fn validate_thread_counts(options: &TransformOptions) -> Result<(), ProcessingError> {
    if options.transform_threads > crate::thread_spawn::MAX_WORKERS
        || options.encoder_threads > crate::thread_spawn::MAX_WORKERS
    {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "transform and encoder threads must be between 0 and 256",
        ));
    }
    Ok(())
}

/// The output side of one transcode: the built-in sink or a backend's.
enum Sink<'b> {
    #[cfg(feature = "webp-write")]
    WebP(Box<WebpSink>),
    #[cfg(feature = "jpeg-xl-write")]
    JpegXl(Box<JxlSink>),
    Backend {
        encoder: Box<dyn RowEncoder + 'b>,
        /// The backend error that stopped the pipeline, if any; the
        /// executor only carries the internal `Error`.
        failure: Option<EncodeError>,
        // Kept after the encoder so dropping a partial sink releases its
        // pixel storage before its reservation.
        frame_reservation: ExternalReservation,
    },
}

impl Sink<'_> {
    fn write_row(&mut self, y: u32, row: OutputRow<'_>) -> Result<(), Error> {
        match (self, row) {
            #[cfg(feature = "webp-write")]
            (Self::WebP(sink), OutputRow::U8(row)) => sink.write_row(y, row),
            #[cfg(feature = "jpeg-xl-write")]
            (Self::JpegXl(sink), row) => sink.write_output_row(y, row),
            (
                Self::Backend {
                    encoder, failure, ..
                },
                row,
            ) => match row {
                OutputRow::U8(row) => encoder.write_row(y, row),
                OutputRow::U16(row) => encoder.write_row_u16(y, row),
            }
            .map_err(|error| {
                *failure = Some(error);
                Error::EncodeFailed
            }),
            // A built-in sink receives rows only at a depth it admits.
            #[allow(unreachable_patterns)]
            _ => Err(Error::IncompleteOutput),
        }
    }

    /// The backend error behind `error`, if the backend caused it.
    fn explain(&mut self, error: Error) -> ProcessingError {
        match self {
            Self::Backend { failure, .. } if failure.is_some() => failure
                .take()
                .map_or_else(|| error.into(), ProcessingError::from),
            Self::Backend { .. } => error.into(),
            #[cfg(feature = "webp-write")]
            Self::WebP(_) => error.into(),
            #[cfg(feature = "jpeg-xl-write")]
            Self::JpegXl(_) => error.into(),
        }
    }

    fn finish(self) -> Result<EncodedImage, ProcessingError> {
        match self {
            #[cfg(feature = "webp-write")]
            Self::WebP(sink) => Ok(sink.finish()?),
            #[cfg(feature = "jpeg-xl-write")]
            Self::JpegXl(sink) => Ok(sink.finish()?),
            Self::Backend {
                encoder,
                frame_reservation,
                ..
            } => {
                let image = encoder.finish()?;
                drop(frame_reservation);
                Ok(image)
            }
        }
    }
}

#[allow(clippy::too_many_lines)] // The stages stay visible in a small facade.
fn run(
    encoded: &[u8],
    format: ImageFormat,
    options: TransformOptions,
    backend: Option<&dyn EncoderBackend>,
    resize: Option<ResizeOptions>,
) -> Result<Output, ProcessingError> {
    let started = Instant::now();
    if options.max_output_bytes == 0 {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "max_output_bytes must be nonzero",
        ));
    }
    // A setting of another output is refused, never ignored (ADR-0043 D4).
    let avif = options.avif == AvifOptions::default();
    let webp = options.webp == WebpOptions::default();
    let jpeg_xl = options.jpeg_xl == JpegXlOptions::default();
    if !match options.output_format {
        OutputFormat::Avif => webp && jpeg_xl,
        OutputFormat::WebP => avif && jpeg_xl,
        OutputFormat::JpegXl => avif && webp,
    } {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "encoder options do not match the selected output format",
        ));
    }
    validate_thread_counts(&options)?;
    // The built-in WebP encoder is serial, as is the JPEG XL one; 0 would
    // mean "available cores" and be reported as such.
    if backend.is_none()
        && options.output_format == OutputFormat::WebP
        && options.encoder_threads != 1
    {
        return Err(ProcessingError::new(
            ProcessingErrorKind::InvalidOptions,
            "the WebP encoder is serial; set encoder_threads to 1",
        ));
    }
    // The built-in AVIF writer is the workspace libavif/AOM backend; it
    // reads every AVIF setting from the request.
    #[cfg(feature = "avif-write")]
    let own = (backend.is_none() && options.output_format == OutputFormat::Avif)
        .then_some(LibavifBackend);
    #[cfg(feature = "avif-write")]
    let backend = backend.or(own.as_ref().map(|encoder| encoder as &dyn EncoderBackend));
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
            .min(crate::thread_spawn::MAX_WORKERS),
        threads => threads,
    };
    #[cfg(feature = "jpeg-xl-write")]
    if backend.is_none() && options.output_format == OutputFormat::JpegXl {
        JxlSink::validate(&options)?;
    }
    if let Some(resize) = resize {
        // Owner decision of 2026-11-15 (ADR-0042): an option that would be
        // ignored is refused. The aspect-aware entry points take their
        // size from `ResizeOptions`; a changed exact size is a mistake.
        let default = TransformOptions::default();
        if (options.output_width, options.output_height)
            != (default.output_width, default.output_height)
        {
            return Err(ProcessingError::new(
                ProcessingErrorKind::InvalidOptions,
                "output_width and output_height are not used with ResizeOptions; leave them at their defaults",
            ));
        }
        crate::resize_geometry::validate(resize)?;
    } else {
        Size::new(options.output_width, options.output_height)?;
    }
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
        (WorkingPrecision::F32, 0) => std::thread::available_parallelism()
            .map_or(1, std::num::NonZero::get)
            .min(crate::thread_spawn::MAX_WORKERS),
        (WorkingPrecision::F32, threads) => threads,
    };
    let mut source = open_source(encoded, format, options.max_input_pixels)?;
    // Alpha is processed straight (ADR-0014); an association the reader
    // did not convert is refused under every policy, before color, rather
    // than guessed (ADR-0015).
    if matches!(
        source.info().layout,
        crate::decoder::ChannelLayout::Rgba | crate::decoder::ChannelLayout::GrayAlpha
    ) && source.info().alpha != crate::color::AlphaMode::Straight
    {
        return Err(Error::UnsupportedAlphaMode.into());
    }
    let decoder_backend = decoder_backend(format);
    source.set_decode_threads(transform_threads);
    // ADR-0042 D2: what the codec reads past, from its headers, and hidden
    // color under zero alpha, noticed in the rows it delivers.
    let mut discarded = source.discarded();
    let hidden_color = std::sync::Arc::new(std::sync::atomic::AtomicBool::new(false));
    let source: Box<dyn DecodeSource + '_> = if matches!(
        source.info().layout,
        crate::decoder::ChannelLayout::Rgba | crate::decoder::ChannelLayout::GrayAlpha
    ) {
        Box::new(crate::discarded::HiddenColorProbe::new(
            source,
            hidden_color.clone(),
        ))
    } else {
        source
    };
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
    let mut color_signal = ColorSignal::Cicp;
    // A typed signal the engine converts itself (sRGB, Display P3, linear
    // sRGB) without a profile beside it takes the same route under every
    // policy: `UseRenderedIcc` permits a profile conversion, it does not
    // require one (working decision 2026-11-15, delegated, revisable by the
    // owner; ADR-0043 D1 for JPEG XL enum and AVIF `nclx`, ADR-0015).
    let typed = source.info().rendered_icc.is_none()
        && source
            .info()
            .rendered_cicp
            .is_some_and(crate::color_contract::is_typed_working_family);
    let source: Box<dyn DecodeSource + '_> =
        if options.profile_policy == InputProfilePolicy::UseRenderedIcc && !typed {
            #[cfg(feature = "icc")]
            {
                color_signal = if source.info().rendered_cicp.is_some() {
                    ColorSignal::RenderedIccWithAgreeingCicp
                } else {
                    ColorSignal::RenderedIcc
                };
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
    let actual: Box<dyn DecodeSource + '_> = if source.info().rendered_icc.is_none()
        && source.info().rendered_cicp.is_none()
        && options.profile_policy == InputProfilePolicy::AssumeSrgbIfMissing
    {
        let mut info = source.info().clone();
        info.rendered_cicp = Some([1, 13, 0, 1]);
        color_signal = ColorSignal::AssumedSrgb;
        Box::new(ExplicitSrgb {
            decoder: source,
            info,
        })
    } else {
        source
    };
    let color = actual.info().semantics().color;
    if !matches!(color, ColorEncoding::Cicp(cicp) if crate::color_contract::is_typed_working_family(cicp))
    {
        // Under `UseRenderedIcc` the ICC stage emits linear sRGB, so ICC
        // reaches this check only under the other two policies.
        let reason = match color {
            ColorEncoding::Unspecified => Some(RefusalReason::ColorSignalMissing),
            ColorEncoding::Icc(_) | ColorEncoding::IccAndCicp { .. } => {
                Some(RefusalReason::RenderedIccRequiresOptIn)
            }
            ColorEncoding::Cicp([_, 16 | 18, ..]) => Some(RefusalReason::HdrTransfer),
            ColorEncoding::Cicp(_) => Some(RefusalReason::UnsupportedColorEncoding),
        };
        return Err(ProcessingError::refused(
            ProcessingErrorKind::UnsupportedColor,
            reason,
            "unsupported or ambiguous rendered color profile; a color-management transform is required",
        ));
    }
    // Gray reaches the graph as RGB: the one expansion of every format
    // (ADR-0043 D2), after color handling (a gray ICC profile was already
    // converted to RGB by the rendered-ICC stage).
    let mut actual = crate::gray::GrayToRgb::wrap(actual);
    let size = actual.info().size;
    let crop = if let Some(crop) = options.crop {
        Rect::new(crop.x, crop.y, crop.width, crop.height)?
    } else {
        Rect::new(0, 0, size.width, size.height)?
    };
    if !crop.fits(size) {
        return Err(Error::OutOfBounds.into());
    }
    let (crop, target_size) = if let Some(resize) = resize {
        crate::resize_geometry::resolve(crop, resize)?
    } else {
        (
            crop,
            Size::new(options.output_width, options.output_height)?,
        )
    };

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
    // The depth rule (ADR-0021 amendment, ADR-0043 decided application of
    // 2026-11-15), one for every output: the input's depth, capped at what
    // the output admits; a reduction is the pipeline's to report.
    let input_depth = actual.info().depth;
    let admitted: &[u8] = match (backend, options.output_format) {
        (Some(backend), _) => backend.row_depths(),
        // cjxl's integer depths (ADR-0021 amendment of 2026-11-15).
        (None, OutputFormat::JpegXl) => &[8, 9, 10, 11, 12, 13, 14, 15, 16],
        // WebP is 8-bit by its format.
        (None, _) => &[crate::output_depth::EIGHT],
    };
    let requested = match options.output_format {
        OutputFormat::Avif => options.avif.depth,
        OutputFormat::JpegXl => options.jpeg_xl.depth(),
        OutputFormat::WebP => None,
    };
    let output_depth =
        crate::output_depth::resolve(requested, input_depth, admitted).map_err(|_| {
            ProcessingError::new(
                ProcessingErrorKind::InvalidOptions,
                "the requested output depth is not one this encoder writes",
            )
        })?;
    discarded.insert_if(output_depth < input_depth, Discarded::PrecisionReduced);
    let mut output_sink = match backend {
        None => match options.output_format {
            // With `avif-write` the AOM writer is the backend above.
            OutputFormat::Avif => {
                return Err(ProcessingError::new(
                    ProcessingErrorKind::FeatureDisabled,
                    "enable the avif-write Cargo feature",
                ));
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
            OutputFormat::JpegXl => {
                #[cfg(feature = "jpeg-xl-write")]
                {
                    Sink::JpegXl(Box::new(JxlSink::with_depth(
                        target_size,
                        options,
                        &budget,
                        output_depth,
                    )?))
                }
                #[cfg(not(feature = "jpeg-xl-write"))]
                {
                    return Err(ProcessingError::new(
                        ProcessingErrorKind::FeatureDisabled,
                        "enable the jpeg-xl-write Cargo feature",
                    ));
                }
            }
        },
        Some(backend) => {
            let request = EncodeRequest {
                format: options.output_format,
                avif: options.avif,
                webp: options.webp,
                jpeg_xl: options.jpeg_xl,
                width: target_size.width,
                height: target_size.height,
                has_alpha: transparent,
                depth: output_depth,
                threads: encoder_threads,
            };
            let frame_reservation =
                budget.reserve_external(backend.required_frame_bytes(&request)?)?;
            Sink::Backend {
                encoder: backend.begin(&request)?,
                failure: None,
                frame_reservation,
            }
        }
    };
    let encoder_backend = backend.map_or(
        match options.output_format {
            OutputFormat::Avif => "libavif-rs/aom",
            OutputFormat::WebP => "libwebp-rs",
            OutputFormat::JpegXl => "libjxl-rs",
        },
        EncoderBackend::name,
    );
    let open = started.elapsed();
    let request = Rect::new(0, 0, target_size.width, target_size.height)?;
    let kernels = match options.kernel_level_ceiling {
        Some(ceiling) => ferrisma_kernels::Kernels::with_ceiling(ceiling),
        None => ferrisma_kernels::Kernels::best(),
    };
    let (kernel_level, kernel_level_reason) = match options.precision {
        WorkingPrecision::F32 => (kernels.level(), kernels.reason()),
        WorkingPrecision::F64Reference => (KernelLevel::Scalar, KernelLevelReason::Ceiling),
    };
    let processed = match options.precision {
        WorkingPrecision::F32 => execute_fast(
            &graph,
            finished,
            input,
            &mut *actual,
            request,
            &budget,
            &kernels,
            transform_threads,
            output_depth,
            |y, row| output_sink.write_row(y, row),
        ),
        WorkingPrecision::F64Reference => execute_quality(
            &graph,
            finished,
            input,
            &mut *actual,
            request,
            &budget,
            output_depth,
            |y, row| output_sink.write_row(y, row),
        ),
    };
    let processed = processed.map_err(|error| output_sink.explain(error))?;
    let encode_started = Instant::now();
    let encoded_image = output_sink.finish()?;
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
    discarded.insert_if(
        hidden_color.load(std::sync::atomic::Ordering::Relaxed),
        Discarded::HiddenColor,
    );
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
        decoder_work: DecoderWork {
            crop: public_rect(crop),
            requested: public_rect(processed.decoder.requested),
            codec_request: public_rect(processed.decoder.work.request),
            codec_output: public_rect(processed.decoder.work.codec_output),
            overfetch_pixels: processed.decoder.work.overfetch_pixels(),
            reconstructed_pixels: processed.decoder.work.reconstructed_pixels,
            entropy_rows: processed.decoder.work.entropy_rows,
            delivered_rows: processed.decoder.rows,
            wrapper_copied_bytes: processed.decoder.engine_copied_bytes,
        },
        color_signal,
        working_precision: options.precision,
        kernel_level,
        kernel_level_reason,
        encoder_threads,
        transform_threads,
        decoder_backend,
        encoder_backend,
        input_depth,
        output_depth,
        discarded,
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
        width: target_size.width,
        height: target_size.height,
        report,
    })
}

/// One image of a batch: compressed bytes, their format and the options.
#[derive(Clone, Copy, Debug)]
pub struct BatchItem<'a> {
    /// Compressed input bytes.
    pub encoded: &'a [u8],
    /// Input format.
    pub format: ImageFormat,
    /// Options; the engine replaces both thread counts with 1 and reports
    /// the 1. A count that [`transcode`] refuses (above 256) is refused for
    /// this item with [`ProcessingErrorKind::InvalidOptions`] (until
    /// 2026-11-15 it was accepted and replaced).
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
    /// An engine with up to 256 workers; 0 means available cores (also capped).
    #[must_use]
    pub fn new(threads: usize) -> Self {
        let threads = match threads {
            0 => std::thread::available_parallelism().map_or(1, std::num::NonZero::get),
            threads => threads,
        }
        .min(crate::thread_spawn::MAX_WORKERS);
        Self { threads }
    }

    /// Number of workers.
    #[must_use]
    pub fn threads(&self) -> usize {
        self.threads
    }

    /// Transcode every item; results come back in input order. A failing
    /// image does not stop the others, and neither does a panicking one: it
    /// gets [`ProcessingErrorKind::Internal`] (since 2026-11-16).
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

    if items.is_empty() {
        return Vec::new();
    }
    let next = AtomicUsize::new(0);
    let results: Vec<Mutex<Option<Result<Output, ProcessingError>>>> =
        items.iter().map(|_| Mutex::new(None)).collect();
    let workers = threads.min(items.len()).max(1);
    // No image starts until every worker is created. Closing this gate
    // after a partial spawn failure releases and joins all started workers.
    let (start, receiver) = std::sync::mpsc::channel::<()>();
    let receiver = Mutex::new(receiver);
    let started = std::thread::scope(|scope| {
        for _ in 0..workers {
            let receiver = &receiver;
            if crate::thread_spawn::spawn(scope, || {
                if receiver.lock().ok().and_then(|rx| rx.recv().ok()).is_none() {
                    return;
                }
                loop {
                    let index = next.fetch_add(1, Ordering::Relaxed);
                    let Some(item) = items.get(index) else { break };
                    // Counts that `transcode` refuses are refused here
                    // before they are replaced by 1.
                    // Each item has its own panic boundary: a panicking
                    // image gets `Internal` and the others keep their
                    // results (until 2026-11-16 the scope re-panicked).
                    let result = contain_panic(|| {
                        validate_thread_counts(&item.options)?;
                        let options = TransformOptions {
                            transform_threads: 1,
                            encoder_threads: 1,
                            ..item.options
                        };
                        run(item.encoded, item.format, options, backend, None)
                    });
                    if let Ok(mut slot) = results[index].lock() {
                        *slot = Some(result);
                    }
                }
            })
            .is_err()
            {
                drop(start);
                return false;
            }
        }
        for _ in 0..workers {
            if start.send(()).is_err() {
                return false;
            }
        }
        true
    });
    if !started {
        return items
            .iter()
            .map(|_| {
                Err(ProcessingError::new(
                    ProcessingErrorKind::ResourceExhausted,
                    "could not start a batch worker",
                ))
            })
            .collect();
    }
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

#[cfg(test)]
mod worker_limit_tests {
    use super::{Engine, ProcessingError, ProcessingErrorKind};
    use crate::error::Error;

    #[test]
    fn batch_worker_count_is_bounded_for_explicit_and_automatic_requests() {
        assert_eq!(Engine::new(1).threads(), 1);
        assert_eq!(Engine::new(256).threads(), 256);
        assert_eq!(Engine::new(usize::MAX).threads(), 256);
        assert!((1..=256).contains(&Engine::new(0).threads()));
        assert!(Engine::new(256).transcode_batch(&[]).is_empty());
        assert_eq!(
            ProcessingError::from(Error::ThreadSpawnFailed).kind(),
            ProcessingErrorKind::ResourceExhausted
        );
    }
}
