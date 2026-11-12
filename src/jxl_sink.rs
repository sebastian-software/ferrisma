//! Still JPEG XL output through the workspace libjxl-rs encoder (ADR-0039).
//!
//! The port's public encoder API admits no frame encode yet; its only
//! complete, oracle-verified image path is the fixed lossless profile
//! `LosslessThunderConfig`: Modular, effort 2 (libjxl "thunder"), level-10
//! parameters, one-shot buffering, a raw codestream, enum sRGB color without
//! ICC, 8-bit RGB with optional unassociated 8-bit alpha, orientation 1. This
//! sink admits exactly that profile and rejects every other request with a
//! typed error instead of falling back.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};

use crate::encoder_backend::EncodedImage;
use crate::error::Error;
use crate::geometry::Size;
use crate::memory::{ExternalReservation, ManagedBuffer, MemoryBudget};
use crate::pipeline::{ChromaSubsampling, JpegXlOptions, OutputFormat, TransformOptions};

/// Level 5 dimension and area limits (ISO/IEC 18181-2, Annex A). The raw
/// codestream carries no `jxll` level box, so admitted images must also fit
/// the default level a decoder assumes.
const MAX_LEVEL5_DIMENSION: usize = 1 << 18;
const MAX_LEVEL5_PIXELS: usize = 1 << 28;

/// Codec-allocated planes that exist at the same time as the input frame in
/// the admitted one-shot path (`ComputeEncodingData` and
/// `compute_encoding_data_lossless_thunder` in the port): an `Image3<f32>`
/// color frame padded to whole 8x8 blocks, one `f32` plane per extra
/// channel, and the Modular `i32` channels of every color and alpha plane.
/// This is a lower bound from the source path: row padding, Modular
/// transforms, tree learning and tokens are opaque.
pub(crate) fn known_codec_frame_bytes(
    width: usize,
    height: usize,
    alpha: bool,
) -> Result<usize, Error> {
    let padded = width
        .checked_next_multiple_of(8)
        .zip(height.checked_next_multiple_of(8))
        .and_then(|(w, h)| w.checked_mul(h))
        .ok_or(Error::GeometryOverflow)?;
    let pixels = width.checked_mul(height).ok_or(Error::GeometryOverflow)?;
    let planes = 3 + usize::from(alpha);
    let color = padded.checked_mul(3 * 4);
    let extra = pixels.checked_mul(usize::from(alpha) * 4);
    let modular = pixels.checked_mul(planes * 4);
    color
        .zip(extra)
        .zip(modular)
        .and_then(|((color, extra), modular)| color.checked_add(extra)?.checked_add(modular))
        .ok_or(Error::GeometryOverflow)
}

pub(crate) struct JxlSink {
    width: usize,
    height: usize,
    next_y: usize,
    /// Straight-alpha RGBA8 rows, compacted to RGB8 in place when opaque.
    frame: ManagedBuffer<u8>,
    budget: MemoryBudget,
    has_alpha: bool,
}

impl JxlSink {
    /// Check the size-independent settings against the admitted profile;
    /// the facade calls this before it opens the input.
    pub(crate) fn validate(options: &TransformOptions) -> Result<(), Error> {
        if options.jpeg_xl != (JpegXlOptions::Lossless { effort: 2 }) {
            return Err(Error::UnsupportedEncoderProfile(
                "only lossless effort 2 is admitted by the workspace encoder",
            ));
        }
        // The port has no parallel runner (encoder step 13); it is serial.
        if options.encoder_threads != 1 {
            return Err(Error::UnsupportedEncoderProfile(
                "the workspace encoder is serial; set encoder_threads to 1",
            ));
        }
        if options.chroma_subsampling != ChromaSubsampling::Yuv444 || options.max_output_bytes == 0
        {
            return Err(Error::InvalidEncodeOptions);
        }
        Ok(())
    }

    /// Validate the request against the admitted profile and reserve the
    /// engine-owned input frame before any pixel is decoded.
    pub(crate) fn new(
        size: Size,
        options: TransformOptions,
        budget: &MemoryBudget,
    ) -> Result<Self, Error> {
        Self::validate(&options)?;
        let width = usize::try_from(size.width).map_err(|_| Error::GeometryOverflow)?;
        let height = usize::try_from(size.height).map_err(|_| Error::GeometryOverflow)?;
        let pixels = width.checked_mul(height).ok_or(Error::GeometryOverflow)?;
        if width > MAX_LEVEL5_DIMENSION
            || height > MAX_LEVEL5_DIMENSION
            || pixels > MAX_LEVEL5_PIXELS
        {
            return Err(Error::UnsupportedEncoderProfile(
                "output exceeds the JPEG XL level 5 dimension or pixel limit",
            ));
        }
        // Size-header admission happens in the codec's own metadata setter.
        LosslessThunderConfig::new(width, height, false, LosslessSampleDepth::Uint8)
            .map_err(|_| Error::UnsupportedEncoderProfile("the encoder rejected the image size"))?;
        let samples = pixels.checked_mul(4).ok_or(Error::GeometryOverflow)?;
        Ok(Self {
            width,
            height,
            next_y: 0,
            frame: budget.allocate_typed::<u8>(samples)?,
            budget: budget.clone(),
            has_alpha: false,
        })
    }

    pub(crate) fn write_row(&mut self, y: u32, bytes: &[u8]) -> Result<(), Error> {
        let row_bytes = self.width.checked_mul(4).ok_or(Error::GeometryOverflow)?;
        if usize::try_from(y).map_err(|_| Error::GeometryOverflow)? != self.next_y
            || self.next_y >= self.height
            || bytes.len() != row_bytes
        {
            return Err(Error::IncompleteOutput);
        }
        let start = self.next_y * row_bytes;
        self.frame.as_mut_slice()[start..start + row_bytes].copy_from_slice(bytes);
        self.has_alpha |= bytes.as_chunks::<4>().0.iter().any(|rgba| rgba[3] != 255);
        self.next_y += 1;
        Ok(())
    }

    pub(crate) fn finish(mut self) -> Result<EncodedImage, Error> {
        if self.next_y != self.height {
            return Err(Error::IncompleteOutput);
        }
        let pixels = self.width * self.height;
        let channels = if self.has_alpha { 4 } else { 3 };
        let frame = self.frame.as_mut_slice();
        if !self.has_alpha {
            // Opaque output carries no alpha channel. Compact in place; each
            // destination index is at most its source index.
            for pixel in 0..pixels {
                frame.copy_within(pixel * 4..pixel * 4 + 3, pixel * 3);
            }
        }
        let codec_bytes = known_codec_frame_bytes(self.width, self.height, self.has_alpha)?;
        let _codec_frames: ExternalReservation = self.budget.reserve_external(codec_bytes)?;
        let mut config = if self.has_alpha {
            LosslessThunderConfig::new_with_alpha(
                self.width,
                self.height,
                false,
                LosslessSampleDepth::Uint8,
            )
        } else {
            LosslessThunderConfig::new(self.width, self.height, false, LosslessSampleDepth::Uint8)
        }
        .map_err(|_| Error::UnsupportedEncoderProfile("the encoder rejected the image size"))?;
        let encoded = config
            .encode_pixels(&frame[..pixels * channels], None)
            .map_err(|status| match status {
                StatusCode::Unsupported => {
                    Error::UnsupportedEncoderProfile("the encoder reported an unsupported request")
                }
                StatusCode::GenericError | StatusCode::NotEnoughBytes => Error::EncodeFailed,
            })?;
        let mut bytes = Vec::new();
        bytes
            .try_reserve_exact(encoded.len())
            .map_err(|_| Error::AllocationFailed)?;
        bytes.extend_from_slice(encoded.as_slice());
        Ok(EncodedImage::new(
            OutputFormat::JpegXl,
            bytes,
            self.frame.as_slice().len() + codec_bytes,
            true,
        ))
    }
}
