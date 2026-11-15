//! Ferrisma's row contract over the independent libavif writer boundary.

use crate::encoder_backend::{
    EncodeError, EncodeErrorKind, EncodeRequest, EncodedImage, EncoderBackend, RowEncoder,
};
use crate::pipeline::{ChromaSubsampling, OutputFormat};
use ferrisma_avif_write as writer;

/// Workspace libavif with its AOM encoder, available with `avif-write`; the
/// engine's built-in AVIF writer, also usable through `transcode_with`.
///
/// Every setting comes from the request's [`AvifOptions`] on libavif's own
/// scales (quality and alpha quality 0–100, speed 0–10); the depth is the
/// request's resolved [`EncodeRequest::depth`], 8, 10 or 12, at which the
/// rows arrive (since 2026-11-15; before, rows were always 8-bit and the
/// depth was the AV1 container's only).
/// Until 2026-11-15 the backend carried the depth itself
/// (`LibavifBackend { depth }`), a second control for the same setting.
///
/// [`AvifOptions`]: crate::pipeline::AvifOptions
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct LibavifBackend;

fn map_error(error: writer::EncodeError) -> EncodeError {
    let (error_kind, message) = error.into_parts();
    let kind = match error_kind {
        writer::ErrorKind::InvalidOptions => EncodeErrorKind::InvalidOptions,
        writer::ErrorKind::ResourceExhausted => EncodeErrorKind::ResourceExhausted,
        writer::ErrorKind::Failed => EncodeErrorKind::Failed,
    };
    EncodeError::new(kind, message)
}

impl LibavifBackend {
    fn config(request: &EncodeRequest) -> Result<writer::Config, EncodeError> {
        if request.format != OutputFormat::Avif {
            return Err(EncodeError::new(
                EncodeErrorKind::InvalidOptions,
                "libavif-rs backend requires AVIF output",
            ));
        }
        let avif = request.avif;
        Ok(writer::Config {
            width: request.width,
            height: request.height,
            has_alpha: request.has_alpha,
            quality: avif.quality,
            alpha_quality: avif.alpha_quality,
            speed: avif.speed,
            threads: request.threads,
            chroma: match avif.chroma {
                ChromaSubsampling::Yuv444 => writer::Chroma::Yuv444,
                ChromaSubsampling::Yuv420 => writer::Chroma::Yuv420,
            },
            // The rows arrive at the image depth, quantized by the engine
            // (ADR-0021 amendment of 2026-11-15): libavif converts RGB to
            // YUV at that depth and never scales 8-bit rows up.
            depth: request.depth,
            sample_depth: request.depth,
        })
    }
}

impl EncoderBackend for LibavifBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }
    fn name(&self) -> &'static str {
        "libavif-rs/aom"
    }
    fn row_depths(&self) -> &'static [u8] {
        &[8, 10, 12]
    }
    fn required_frame_bytes(&self, request: &EncodeRequest) -> Result<usize, EncodeError> {
        writer::required_frame_bytes(&Self::config(request)?).map_err(map_error)
    }
    fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        let config = Self::config(request)?;
        let bytes = writer::required_frame_bytes(&config).map_err(map_error)?;
        let frame = writer::Frame::new(config).map_err(map_error)?;
        Ok(Box::new(Frame { frame, bytes }))
    }
}

struct Frame {
    frame: writer::Frame,
    bytes: usize,
}
impl RowEncoder for Frame {
    fn write_row(&mut self, y: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.frame.write_row(y, rgba).map_err(map_error)
    }
    fn write_row_u16(&mut self, y: u32, rgba: &[u16]) -> Result<(), EncodeError> {
        self.frame.write_row_u16(y, rgba).map_err(map_error)
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        let bytes = self.frame.finish().map_err(map_error)?;
        Ok(EncodedImage::new(
            OutputFormat::Avif,
            bytes,
            self.bytes,
            true,
        ))
    }
}

/// Opaque 10-bit AVIF from the workspace writer at speed 10, one thread:
/// the test fixtures of the AVIF reader (the rav1e sink wrote them until
/// 2026-11-15). `row(y)` returns RGBA8 row `y`. `quality` stays an `f32`
/// of whole numbers, so the reader tests need no change; it is converted to
/// libavif's integer scale.
#[cfg(all(test, feature = "avif-read"))]
#[allow(clippy::cast_possible_truncation, clippy::cast_sign_loss)] // Test literals 0-100.
pub(crate) fn encode_for_test(
    width: u32,
    height: u32,
    chroma: ChromaSubsampling,
    quality: f32,
    row: impl Fn(u32) -> Vec<u8>,
) -> Vec<u8> {
    let mut frame = writer::Frame::new(writer::Config {
        width,
        height,
        has_alpha: false,
        quality: quality as u8,
        alpha_quality: quality as u8,
        speed: 10,
        threads: 1,
        chroma: match chroma {
            ChromaSubsampling::Yuv444 => writer::Chroma::Yuv444,
            ChromaSubsampling::Yuv420 => writer::Chroma::Yuv420,
        },
        depth: 10,
        sample_depth: 8,
    })
    .unwrap();
    for y in 0..height {
        frame.write_row(y, &row(y)).unwrap();
    }
    frame.finish().unwrap()
}
