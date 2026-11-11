//! Ferrixel's row contract over the independent libavif writer boundary.

use crate::encoder_backend::{
    EncodeError, EncodeErrorKind, EncodeRequest, EncodedImage, EncoderBackend, RowEncoder,
};
use crate::pipeline::{ChromaSubsampling, OutputFormat};
use ferrixel_avif_write as writer;

/// Workspace libavif with its AOM encoder, available with `avif-own-write`.
///
/// Quality 0–100 and speed 0–10 use libavif's scales, not Rav1e's.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LibavifBackend {
    /// AV1 bit depth: 8 or 10.
    pub depth: u8,
}

impl Default for LibavifBackend {
    fn default() -> Self {
        Self { depth: 10 }
    }
}

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
    fn config(self, request: &EncodeRequest) -> Result<writer::Config, EncodeError> {
        if request.format != OutputFormat::Avif {
            return Err(EncodeError::new(
                EncodeErrorKind::InvalidOptions,
                "libavif-rs backend requires AVIF output",
            ));
        }
        Ok(writer::Config {
            width: request.width,
            height: request.height,
            has_alpha: request.has_alpha,
            quality: request.quality,
            speed: request.speed,
            threads: request.threads,
            chroma: match request.chroma {
                ChromaSubsampling::Yuv444 => writer::Chroma::Yuv444,
                ChromaSubsampling::Yuv420 => writer::Chroma::Yuv420,
            },
            depth: self.depth,
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
    fn required_frame_bytes(&self, request: &EncodeRequest) -> Result<usize, EncodeError> {
        writer::required_frame_bytes(&self.config(request)?).map_err(map_error)
    }
    fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        let config = self.config(request)?;
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
