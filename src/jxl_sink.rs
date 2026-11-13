//! Still JPEG XL output through the workspace libjxl-rs encoder (ADR-0039).
//!
//! Two admitted profiles, selected by [`JpegXlOptions`]:
//!
//! - **Lossless:** the port's fixed, oracle-verified `LosslessThunderConfig`
//!   (Modular, effort 2, level-10 parameters, one-shot, raw codestream, enum
//!   sRGB without ICC, 8-bit RGB with optional unassociated 8-bit alpha),
//!   reached through its safe entry point.
//! - **Lossy:** the port's public encoder API through the safe
//!   `ferrisma-jxl-encoder` bridge: `VarDCT` in XYB at effort 7, distance
//!   0.05 to below 3.0, patches off, 8-bit RGB with enum sRGB and, when any
//!   alpha sample is below 255, one straight 8-bit alpha channel coded
//!   losslessly; one frame, raw codestream, serial. The result equals
//!   `cjxl --patches=0 -e 7 -d <distance>` (byte-identical with pinned
//!   libjxl in the port's own oracle tests).
//!
//! Patches are always off in lossy mode (ADR-0039): the port does not encode
//! the patch reference frame yet, and with the search off every image encodes
//! deterministically, without a content-dependent retry. Fully opaque alpha
//! is omitted by this sink in both modes before the encoder sees it; the
//! encoder's own opaque-alpha stripping is not relied on. Every request
//! outside the admitted profiles is rejected with a typed error instead of
//! falling back; alpha is never dropped silently.

use ferrisma_jxl_encoder as bridge;
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

/// Measured peak memory of the admitted lossless one-shot profile, in bytes
/// per pixel of opaque 8-bit RGB input, **including** the engine's input
/// frame (issue #223, ADR-0004 and ADR-0039).
///
/// Source: `docs/benchmarks/v1.0-jxl-performance-analysis-2026-11-12.md`,
/// section 3 "Lossless encode": the port alone peaked at 69.7 B/px heap and
/// 75.8 B/px RSS (flower, 3.4 MP), the engine CLI at 71-72 B/px RSS
/// (1,707-1,719 MB, mountain JPEG to JPEG XL, 24 MP), and C++ `cjxl
/// --buffering=0` in the same one-shot mode at 79 B/px. The largest
/// measurement of this port, 75.8 B/px, is rounded up. What dominates it:
/// one 8-byte token per sample for every group (about 24 B/px), the `f32`
/// and Modular `i32` planes, and a second copy of the input. The figures
/// were taken on a loaded host; they are memory, not timing, but they are
/// one machine and two images, not a bound.
pub(crate) const LOSSLESS_ONE_SHOT_PEAK_BYTES_PER_RGB_PIXEL: usize = 76;

/// The budget reservation for the lossless codec, on top of the engine's
/// RGBA8 input frame (4 B/px, reserved when the sink is created): the
/// measured one-shot peak minus that frame, so frame plus reservation equal
/// the measured peak (issue #223).
///
/// The measurement is opaque RGB. With alpha the per-sample costs (tokens,
/// planes, input copy) apply to four samples instead of three, so the peak
/// is scaled by 4/3; that scaling is derived, **not measured**. For small
/// images, per-image costs dominate and the per-pixel figure does not hold,
/// so the planes known from the source path (`known_codec_frame_bytes`)
/// stay a floor.
pub(crate) fn lossless_codec_reservation_bytes(
    width: usize,
    height: usize,
    alpha: bool,
) -> Result<usize, Error> {
    let pixels = width.checked_mul(height).ok_or(Error::GeometryOverflow)?;
    let samples = 3 + usize::from(alpha);
    let peak = pixels
        .checked_mul(LOSSLESS_ONE_SHOT_PEAK_BYTES_PER_RGB_PIXEL * samples)
        .map(|bytes| bytes.div_ceil(3))
        .ok_or(Error::GeometryOverflow)?;
    let frame = pixels.checked_mul(4).ok_or(Error::GeometryOverflow)?;
    let measured = peak.saturating_sub(frame);
    Ok(measured.max(known_codec_frame_bytes(width, height, alpha)?))
}

/// Codec-allocated planes that exist at the same time as the input frame in
/// the admitted lossless one-shot path (`ComputeEncodingData` and
/// `compute_encoding_data_lossless_thunder` in the port): an `Image3<f32>`
/// color frame padded to whole 8x8 blocks, one `f32` plane per extra
/// channel, and the Modular `i32` channels of every color and alpha plane.
/// This is a lower bound from the source path: row padding, Modular
/// transforms, tree learning and tokens are not in it. It is the floor of
/// `lossless_codec_reservation_bytes`, no longer the reservation itself
/// (issue #223: about 28 B/px reserved against about 72 B/px measured).
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

/// The bridge's description of the lossy frame: RGB8, or RGBA8 with
/// straight alpha, enum sRGB.
fn lossy_image(width: usize, height: usize, alpha: bool) -> Result<bridge::ImageInfo, Error> {
    let width = u32::try_from(width).map_err(|_| Error::GeometryOverflow)?;
    let height = u32::try_from(height).map_err(|_| Error::GeometryOverflow)?;
    Ok(if alpha {
        bridge::ImageInfo::srgba8(width, height)
    } else {
        bridge::ImageInfo::srgb8(width, height)
    })
}

/// The lossy encoder's large buffers for an RGB8 or RGBA8 frame: an estimate
/// derived from the port's source (input copy, padded XYB planes, masking
/// plane, AC coefficients, and with alpha the extra-channel and Modular
/// planes; see `ferrisma_jxl_encoder::estimated_codec_bytes`), not a
/// measured bound.
pub(crate) fn estimated_lossy_codec_bytes(
    width: usize,
    height: usize,
    alpha: bool,
) -> Result<usize, Error> {
    bridge::estimated_codec_bytes(&lossy_image(width, height, alpha)?)
        .ok_or(Error::GeometryOverflow)
}

#[derive(Clone, Copy)]
enum Mode {
    Lossless,
    Lossy { distance: f32 },
}

pub(crate) struct JxlSink {
    width: usize,
    height: usize,
    next_y: usize,
    /// Straight-alpha RGBA8 rows, compacted to RGB8 in place when opaque.
    frame: ManagedBuffer<u8>,
    budget: MemoryBudget,
    has_alpha: bool,
    mode: Mode,
    max_output_bytes: usize,
}

impl JxlSink {
    /// Check the size-independent settings against the admitted profiles;
    /// the facade calls this before it opens the input.
    pub(crate) fn validate(options: &TransformOptions) -> Result<(), Error> {
        Self::mode(options)?;
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

    fn mode(options: &TransformOptions) -> Result<Mode, Error> {
        match options.jpeg_xl {
            JpegXlOptions::Lossless { effort: 2 } => Ok(Mode::Lossless),
            JpegXlOptions::Lossless { .. } => Err(Error::UnsupportedEncoderProfile(
                "only lossless effort 2 is admitted by the workspace encoder",
            )),
            // `contains` is false for NaN.
            JpegXlOptions::Lossy { distance }
                if (JpegXlOptions::MIN_LOSSY_DISTANCE..JpegXlOptions::MAX_LOSSY_DISTANCE)
                    .contains(&distance) =>
            {
                Ok(Mode::Lossy { distance })
            }
            JpegXlOptions::Lossy { .. } => Err(Error::UnsupportedEncoderProfile(
                "the lossy distance must be at least 0.05 and below 3.0",
            )),
        }
    }

    /// Validate the request against the admitted profile and reserve the
    /// engine-owned input frame before any pixel is decoded.
    pub(crate) fn new(
        size: Size,
        options: TransformOptions,
        budget: &MemoryBudget,
    ) -> Result<Self, Error> {
        Self::validate(&options)?;
        let mode = Self::mode(&options)?;
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
        if let Mode::Lossless = mode {
            // Size-header admission happens in the codec's own metadata setter.
            LosslessThunderConfig::new(width, height, false, LosslessSampleDepth::Uint8).map_err(
                |_| Error::UnsupportedEncoderProfile("the encoder rejected the image size"),
            )?;
        }
        let samples = pixels.checked_mul(4).ok_or(Error::GeometryOverflow)?;
        Ok(Self {
            width,
            height,
            next_y: 0,
            frame: budget.allocate_typed::<u8>(samples)?,
            budget: budget.clone(),
            has_alpha: false,
            mode,
            max_output_bytes: options.max_output_bytes,
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
        let transparent = bytes.as_chunks::<4>().0.iter().any(|rgba| rgba[3] != 255);
        let start = self.next_y * row_bytes;
        self.frame.as_mut_slice()[start..start + row_bytes].copy_from_slice(bytes);
        self.has_alpha |= transparent;
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
        match self.mode {
            Mode::Lossless => self.finish_lossless(pixels * channels),
            Mode::Lossy { distance } => self.finish_lossy(pixels * channels, distance),
        }
    }

    fn finish_lossless(&self, samples: usize) -> Result<EncodedImage, Error> {
        let codec_bytes =
            lossless_codec_reservation_bytes(self.width, self.height, self.has_alpha)?;
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
            .encode_pixels(&self.frame.as_slice()[..samples], None)
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

    fn finish_lossy(&self, samples: usize, distance: f32) -> Result<EncodedImage, Error> {
        let codec_bytes = estimated_lossy_codec_bytes(self.width, self.height, self.has_alpha)?;
        let _codec_frames: ExternalReservation = self.budget.reserve_external(codec_bytes)?;
        let image = lossy_image(self.width, self.height, self.has_alpha)?;
        let mut bytes = Vec::new();
        bridge::encode(
            image,
            // Patches off, always: see the module documentation.
            bridge::FrameOptions::with_distance(distance).with_patches(false),
            &self.frame.as_slice()[..samples],
            &mut bytes,
            self.max_output_bytes,
        )
        .map_err(|error| match error {
            // Not reachable for the admitted request (patches off, 8-bit
            // straight alpha, enum sRGB); kept typed, never a fallback.
            bridge::Error::NotAdmitted(_) => Error::UnsupportedEncoderProfile(
                "the workspace encoder does not admit this lossy request",
            ),
            bridge::Error::OutputLimit { limit } => Error::BudgetExceeded {
                requested: limit.saturating_add(1),
                available: limit,
            },
            bridge::Error::OutOfMemory => Error::AllocationFailed,
            _ => Error::EncodeFailed,
        })?;
        Ok(EncodedImage::new(
            OutputFormat::JpegXl,
            bytes,
            self.frame.as_slice().len() + codec_bytes,
            true,
        ))
    }
}
