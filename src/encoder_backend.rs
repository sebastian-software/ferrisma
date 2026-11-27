//! Pluggable output encoders (ADR-0024).
//!
//! `transcode` writes AVIF with the built-in workspace libavif/AOM writer,
//! itself an [`EncoderBackend`] (`avif-write`). The trait lets a separate
//! crate supply another encoder, such as the libavif-rs adapter in
//! `crates/ferrisma-libavif`, without adding its
//! dependencies, toolchain or `unsafe` code to this crate. The engine hands
//! the backend finished output rows; everything up to and including the
//! final quantization, at 8 bits or at a depth the backend admits
//! ([`EncoderBackend::row_depths`]), stays Ferrisma's.

use std::fmt;

use crate::pipeline::{OutputFormat, OutputSettings};

/// What one image's encoder is asked to produce: the output settings of
/// the transcode, as the caller set them, and the frame the rows describe.
/// A backend validates the settings of its format on its own scale and
/// refuses, with [`EncodeErrorKind::InvalidOptions`], a setting it cannot
/// honor instead of ignoring it (ADR-0042). Compare backends at matched
/// quality, not equal numeric settings.
///
/// Since 2026-11-16 the settings are one [`OutputSettings`] value, so the
/// request carries only the requested format's settings (before: a format
/// and the settings of all three formats, the others at their defaults).
/// Not `Copy`, so that a later field need not be.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct EncodeRequest {
    /// The requested output format and its encoder settings: quality,
    /// speed, depth and chroma layout for AVIF ([`AvifOptions`]), the
    /// profile for WebP and JPEG XL.
    ///
    /// [`AvifOptions`]: crate::pipeline::AvifOptions
    pub output: OutputSettings,
    /// Output width in pixels.
    pub width: u32,
    /// Output height in pixels.
    pub height: u32,
    /// The rows carry straight alpha that may be below the largest code;
    /// when false every alpha sample is the largest code (255 at 8 bits).
    pub has_alpha: bool,
    /// Bits per sample of the rows: 8 (`write_row`, `u8`) or one of the
    /// backend's other [`EncoderBackend::row_depths`] (`write_row_u16`,
    /// samples scaled to `0..=2^depth - 1`). The engine picks it by the
    /// depth rule (ADR-0021, amendment of 2026-11-15): the input's depth,
    /// capped at what the backend admits, or the format's explicit depth
    /// setting. Added 2026-11-15; before, every row was 8-bit.
    pub depth: u8,
    /// Encoder threads, resolved (at least 1); 1 means the caller asked
    /// for single-threaded encoding.
    pub threads: usize,
    /// Internal hint that the engine's transform stage will keep its own
    /// worker pool alive while delivering rows to `write_row` / `write_row_u16`,
    /// so row-intake conversion stays serial instead of spawning a second
    /// concurrent thread pool.
    #[cfg_attr(not(feature = "avif-write"), allow(dead_code))]
    pub(crate) concurrent_row_producer: bool,
}

impl PartialEq for EncodeRequest {
    fn eq(&self, other: &Self) -> bool {
        self.output == other.output
            && self.width == other.width
            && self.height == other.height
            && self.has_alpha == other.has_alpha
            && self.depth == other.depth
            && self.threads == other.threads
    }
}

impl EncodeRequest {
    /// A request for `width` by `height` opaque 8-bit rows in one thread,
    /// with `output`'s settings: the constructor for a backend author's own
    /// tests of [`EncoderBackend::begin`] and
    /// [`EncoderBackend::required_frame_bytes`] (since 2026-11-16; the
    /// struct is `#[non_exhaustive]`, so it cannot be built as a literal).
    /// The other fields are public and can be changed after construction.
    /// The engine builds its requests itself; this constructor does not
    /// validate anything.
    ///
    /// ```
    /// use ferrisma::pipeline::{AvifOptions, EncodeRequest, OutputFormat, OutputSettings};
    ///
    /// let mut request = EncodeRequest::new(OutputSettings::Avif(AvifOptions::default()), 64, 48);
    /// request.has_alpha = true;
    /// request.depth = 10;
    /// assert_eq!(request.format(), OutputFormat::Avif);
    /// assert_eq!(request.threads, 1);
    /// ```
    #[must_use]
    pub fn new(output: OutputSettings, width: u32, height: u32) -> Self {
        Self {
            output,
            width,
            height,
            has_alpha: false,
            depth: 8,
            threads: 1,
            concurrent_row_producer: false,
        }
    }

    /// The requested output format, the format of [`Self::output`].
    #[must_use]
    pub fn format(&self) -> OutputFormat {
        self.output.format()
    }
}

/// The encoded image and what the backend knows about its memory.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct EncodedImage {
    /// Format of `bytes`; must match the request.
    pub format: OutputFormat,
    /// The complete encoded file.
    pub bytes: Vec<u8>,
    /// Bytes of input frames the backend materialized (for example a
    /// full RGBA or YUV frame), reported as `encoder_output_frame_bytes`.
    pub frame_materialization_bytes: usize,
    /// Whether the backend's own allocations are outside any bound
    /// Ferrisma can report; reported as `opaque_encoder_memory`.
    pub opaque_memory: bool,
}

impl EncodedImage {
    /// Describe a finished encode.
    #[must_use]
    pub fn new(
        format: OutputFormat,
        bytes: Vec<u8>,
        frame_materialization_bytes: usize,
        opaque_memory: bool,
    ) -> Self {
        Self {
            format,
            bytes,
            frame_materialization_bytes,
            opaque_memory,
        }
    }
}

/// Why a backend failed.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum EncodeErrorKind {
    /// The request is outside what the backend supports (an encoder
    /// setting, geometry or alpha).
    InvalidOptions,
    /// The backend could not allocate memory.
    ResourceExhausted,
    /// The encoder itself failed.
    Failed,
}

/// A backend failure with a message for people.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct EncodeError {
    kind: EncodeErrorKind,
    message: String,
}

impl EncodeError {
    /// A failure of `kind` with a message for people.
    #[must_use]
    pub fn new(kind: EncodeErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

    /// The failure category.
    #[must_use]
    pub fn kind(&self) -> EncodeErrorKind {
        self.kind
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(&self.message)
    }
}

impl std::error::Error for EncodeError {}

/// The encoder of one image. It receives every output row exactly once,
/// top to bottom, then `finish` once; after an error from either, nothing
/// else is called.
///
/// `Send` since 2026-11-16 (before the first release, because adding a
/// supertrait later breaks every implementor): a later engine may hand the
/// encoder to another thread than the one that called
/// [`EncoderBackend::begin`], for example to encode while the next rows are
/// decoded or to write to a caller's output stream. An encoder that owns its
/// buffers (`Vec`s, a codec context behind an owned handle) is `Send`; one
/// that keeps an `Rc`, a `RefCell` shared with the backend, or a raw codec
/// pointer must wrap them (`Arc`, `Mutex`) or assert `Send` for a context the
/// codec documents as movable between threads. The built-in AVIF encoder
/// (`LibavifBackend`) owns its `avifImage` planes, bounded RGB(A) conversion
/// strip and settings, and creates the libavif encoder inside `finish`, so it
/// is `Send`.
///
/// **Rule for every new row axis.** Rows today are top to bottom, RGBA,
/// straight alpha, sRGB, at 8 bits or one deeper integer depth. Every axis a
/// later release adds (another depth or sample type such as float, other
/// channel layouts such as gray or no alpha, another color encoding, tiles or
/// another order) is an opt-in: a new method with a default that refuses or
/// never runs, and a capability on [`EncoderBackend`] whose default keeps the
/// conservative behavior (as [`EncoderBackend::row_depths`] and
/// [`RowEncoder::write_row_u16`] did for depth, ADR-0024 amendment of
/// 2026-11-16). An existing backend then compiles and behaves as before.
pub trait RowEncoder: Send {
    /// `rgba` is one output row of straight-alpha sRGB8 RGBA samples,
    /// `4 * width` bytes, with alpha 255 for opaque images.
    ///
    /// # Errors
    /// Returning an error stops the pipeline; the transcode fails with it.
    fn write_row(&mut self, y: u32, rgba: &[u8]) -> Result<(), EncodeError>;

    /// One output row of straight-alpha sRGB RGBA samples deeper than 8
    /// bits, `4 * width` native-endian `u16` values scaled to
    /// `0..=2^depth - 1` ([`EncodeRequest::depth`]), with the largest code as
    /// alpha for opaque images. Called instead of `write_row` only when the
    /// request's depth is above 8, which the engine chooses only for a
    /// depth in [`EncoderBackend::row_depths`]. The default refuses
    /// (added 2026-11-15; a backend without deeper rows needs no change).
    ///
    /// # Errors
    /// Returning an error stops the pipeline; the transcode fails with it.
    fn write_row_u16(&mut self, _y: u32, _rgba: &[u16]) -> Result<(), EncodeError> {
        Err(EncodeError::new(
            EncodeErrorKind::InvalidOptions,
            "this encoder backend accepts only 8-bit rows",
        ))
    }

    /// Encode after the last row.
    ///
    /// # Errors
    /// The transcode fails with the returned error.
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError>;
}

/// A source of encoders, shared by the images of a batch.
pub trait EncoderBackend: Sync {
    /// Format this backend emits. The pipeline checks it against the request.
    fn format(&self) -> OutputFormat;
    /// Short name for reports and diagnostics, such as `"libavif/aom"`.
    fn name(&self) -> &'static str;

    /// The row depths this backend encodes, ascending, 8 included: the
    /// engine's depth rule picks the largest one that does not exceed the
    /// input's depth (and reports the reduction when it is below), and an
    /// explicit depth outside this set is refused before decoding. The
    /// default, `[8]`, keeps every existing backend on 8-bit rows (added
    /// 2026-11-15, ADR-0021 amendment of that date).
    fn row_depths(&self) -> &'static [u8] {
        &[8]
    }

    /// Known pixel-frame bytes simultaneously materialized by this backend.
    ///
    /// The engine reserves this amount before `begin` and holds it through
    /// `finish`. The default of zero preserves existing backends, whose
    /// allocations may be opaque. This does not include encoded output,
    /// allocator overhead, or other codec scratch memory.
    ///
    /// # Errors
    /// Returns an error if the backend cannot represent the request.
    fn required_frame_bytes(&self, _request: &EncodeRequest) -> Result<usize, EncodeError> {
        Ok(0)
    }

    /// Start one image. Called after the input and options were validated
    /// and before the first pixel is decoded.
    ///
    /// # Errors
    /// The transcode fails with the returned error before decoding.
    fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError>;
}
