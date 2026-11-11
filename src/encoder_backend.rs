//! Pluggable output encoders (ADR-0024).
//!
//! `transcode` writes AVIF with the built-in `rav1e` sink. An
//! [`EncoderBackend`] lets a separate crate supply another encoder, such as
//! the libavif-rs adapter in `crates/ferrixel-libavif`, without adding its
//! dependencies, toolchain or `unsafe` code to this crate. The engine hands
//! the backend finished output rows; everything up to and including the
//! final sRGB8 quantization stays Ferrixel's.

use std::fmt;

use crate::pipeline::{ChromaSubsampling, OutputFormat, WebpOptions};

/// What one image's encoder is asked to produce. AVIF quality and speed are
/// passed through as the caller set them; WebP settings are in `webp`.
/// Compare backends at matched quality, not equal numeric settings.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct EncodeRequest {
    pub format: OutputFormat,
    /// Format-specific settings when `format` is WebP.
    pub webp: WebpOptions,
    pub width: u32,
    pub height: u32,
    /// The rows carry straight alpha that may be below 255; when false
    /// every alpha byte is 255.
    pub has_alpha: bool,
    /// AVIF quality; for WebP use `webp`.
    pub quality: f32,
    /// AVIF speed; for WebP use `webp`.
    pub speed: u8,
    /// Encoder threads, resolved (at least 1); 1 means the caller asked
    /// for single-threaded encoding.
    pub threads: usize,
    pub chroma: ChromaSubsampling,
}

/// The encoded image and what the backend knows about its memory.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
#[non_exhaustive]
pub struct EncodedImage {
    pub format: OutputFormat,
    pub bytes: Vec<u8>,
    /// Bytes of input frames the backend materialized (for example a
    /// full RGBA or YUV frame), reported as `encoder_output_frame_bytes`.
    pub frame_materialization_bytes: usize,
    /// Whether the backend's own allocations are outside any bound
    /// Ferrixel can report; reported as `opaque_encoder_memory`.
    pub opaque_memory: bool,
}

impl EncodedImage {
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
    /// The request is outside what the backend supports (quality, speed,
    /// geometry, alpha or chroma layout).
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
    #[must_use]
    pub fn new(kind: EncodeErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }

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
pub trait RowEncoder {
    /// `rgba` is one output row of straight-alpha sRGB8 RGBA samples,
    /// `4 * width` bytes, with alpha 255 for opaque images.
    ///
    /// # Errors
    /// Returning an error stops the pipeline; the transcode fails with it.
    fn write_row(&mut self, y: u32, rgba: &[u8]) -> Result<(), EncodeError>;

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
    /// Short name for reports and diagnostics, such as `"libavif-rs/aom"`.
    fn name(&self) -> &'static str;

    /// Start one image. Called after the input and options were validated
    /// and before the first pixel is decoded.
    ///
    /// # Errors
    /// The transcode fails with the returned error before decoding.
    fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError>;
}
