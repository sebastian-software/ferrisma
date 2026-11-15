//! Still JPEG XL output through the workspace libjxl-rs encoder (ADR-0039).
//!
//! Two admitted profiles, selected by [`JpegXlOptions`]:
//!
//! - **Lossless:** the port's public encoder API through the safe
//!   `ferrisma-jxl-encoder` bridge (`encode_lossless`): libjxl's own
//!   lossless setting at effort 2 (Modular, raw codestream, enum sRGB
//!   without ICC, 8-bit RGB with optional unassociated 8-bit alpha) with the
//!   **default buffering**, so that more than 8 groups take the original's
//!   chunked encode (`EncodeFrameStreaming`, one 2048x2048 DC group at a time;
//!   issue #223, ADR-0039 2026-11-15). Its bytes are the original's default
//!   `cjxl -d 0 -e 2 -x color_space=RGB_D65_SRG_Rel_SRG` (issue #249;
//!   `jxl_output_tests` compares them). Until 2026-11-15 the sink used the
//!   port's fixed one-shot `LosslessThunderConfig`, the `--buffering=0`
//!   bytes; at most 8 groups both are the same file.
//! - **Lossy:** the port's public encoder API through the safe
//!   `ferrisma-jxl-encoder` bridge: `VarDCT` in XYB at effort 7, distance
//!   0.05 to below 3.0, patches off, 8-bit RGB with enum sRGB and, when any
//!   alpha sample is below 255, one straight 8-bit alpha channel coded
//!   losslessly; one frame, raw codestream, serial. The result equals
//!   `cjxl --patches=0 -e 7 -d <distance> -x color_space=RGB_D65_SRG_Rel_SRG`
//!   on the finished rows (byte-identical with pinned libjxl in the port's
//!   own oracle tests; the color hint selects the relative rendering intent
//!   that `JxlColorEncodingSetToSRGB` signals, ADR-0039 2026-11-15).
//!
//! Patches are always off in lossy mode (ADR-0039): the port does not encode
//! the patch reference frame yet, and with the search off every image encodes
//! deterministically, without a content-dependent retry. Fully opaque alpha
//! is omitted by this sink in both modes before the encoder sees it; the
//! encoder's own opaque-alpha stripping is not relied on. Every request
//! outside the admitted profiles is rejected with a typed error instead of
//! falling back; alpha is never dropped silently.

use ferrisma_jxl_encoder as bridge;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};

use crate::encoder_backend::EncodedImage;
use crate::error::Error;
use crate::geometry::Size;
use crate::memory::{ExternalReservation, ManagedBuffer, MemoryBudget};
use crate::output_depth::OutputRow;

/// Bytes of one frame sample: 1 at 8 bits, 2 (`u16`) above.
fn sample_bytes(depth: u8) -> usize {
    if depth > 8 { 2 } else { 1 }
}
use crate::pipeline::{AvifOptions, JpegXlOptions, OutputFormat, TransformOptions};

/// Level 5 dimension and area limits (ISO/IEC 18181-2, Annex A). The raw
/// codestream carries no `jxll` level box, so admitted images must also fit
/// the default level a decoder assumes.
const MAX_LEVEL5_DIMENSION: usize = 1 << 18;
const MAX_LEVEL5_PIXELS: usize = 1 << 28;

/// Measured peak memory of the lossless encode, issue #223 (ADR-0004,
/// ADR-0039 2026-11-15). Source: `docs/benchmarks/v1.0-jxl-lossless-chunked-memory-2026-11-15.md`:
/// peak RSS of a fresh process (`/usr/bin/time -l`, the larger of two
/// runs) of the port's public encoder with the default buffering, the path
/// this sink takes, at 0.49 to 24 MP, opaque RGB and RGBA, on macOS `AArch64`.
/// The measured process holds the input once (3 B/px RGB, 4 B/px RGBA)
/// besides the encoder; the reservation below subtracts exactly that, so
/// that the engine's 4 B/px frame plus the reservation is at least the
/// measured peak. Each term is indexed `[opaque RGB, RGBA]`.
///
/// One-shot path (at most 8 groups of 256x256, where the original does not
/// stream at any buffering): bytes per pixel. Measured 75.3 (RGB) and 96.9
/// (RGBA) B/px at 12 and 24 MP, 85.8 and 113.3 at 512x1024 (8 groups),
/// where the constant term below dominates. The RGBA term replaces the 4/3
/// scaling of 2026-11-13, which was derived, not measured.
pub(crate) const LOSSLESS_ONE_SHOT_BYTES_PER_PIXEL: [usize; 2] = [76, 97];
/// The one-shot term for samples above 8 bits (`u16` input), measured on
/// 2026-11-15 with 16-bit photographs (`docs/benchmarks/v1.0-jxl-deep-memory-2026-11-15.md`):
/// 111.7 (RGB) and 130.5 (RGBA) B/px RSS at 512x1024, the constant below
/// included; 16 bits is the deepest and widest token case and covers 9 to
/// 15 bits, whose buffers are the same `u16` and `i32` planes.
pub(crate) const LOSSLESS_ONE_SHOT_DEEP_BYTES_PER_PIXEL: [usize; 2] = [88, 107];
/// Chunked (streaming) path, more than 8 groups: bytes per pixel of the
/// whole frame (the encoder's input copy, tokens and section writers kept
/// until output, the output) ...
pub(crate) const LOSSLESS_CHUNKED_BYTES_PER_PIXEL: [usize; 2] = [16, 20];
/// The chunked whole-frame term for samples above 8 bits, measured as above:
/// the slope between 12.2 and 24 MP, 25.0 (RGB) and 31.7 (RGBA) B/px.
pub(crate) const LOSSLESS_CHUNKED_DEEP_BYTES_PER_PIXEL: [usize; 2] = [25, 32];
/// ... plus bytes per pixel of one DC group (2048x2048 pixels, or the frame
/// if smaller): the working set the original allocates for one DC group at
/// a time. Measured RSS: RGB 52.3 B/px at 2048x2048, 33.0 at 12 MP, 19.9 at
/// 24 MP; RGBA 70.0, 42.3, 28.9.
pub(crate) const LOSSLESS_CHUNKED_DC_GROUP_BYTES_PER_PIXEL: [usize; 2] = [53, 70];
/// The DC-group term for samples above 8 bits, measured as above: fitted
/// through the 12.2 and 24 MP peaks (70.3 and 98.9 B/px of one DC group),
/// rounded up; every measured deep peak fits.
pub(crate) const LOSSLESS_CHUNKED_DC_GROUP_DEEP_BYTES_PER_PIXEL: [usize; 2] = [71, 99];
/// Constant term of both paths: process and allocator overhead that does
/// not scale with the image (it dominates at a few hundred thousand pixels).
pub(crate) const LOSSLESS_PEAK_CONSTANT_BYTES: usize = 12 << 20;
/// Pixels of one DC group (2048x2048).
const DC_GROUP_PIXELS: usize = 2048 * 2048;

/// Whether the encoder streams this frame: more than 8 groups of 256x256
/// (`CanDoStreamingEncoding` at the default buffering; frames of at most
/// 400x400 pixels, whose Modular groups can be larger, have at most four).
fn lossless_streams(width: usize, height: usize) -> bool {
    width.div_ceil(256).saturating_mul(height.div_ceil(256)) > 8
}

/// The budget reservation for the lossless codec, on top of the engine's
/// RGBA input frame (4 B/px at 8 bits, 8 B/px above, reserved when the sink
/// is created): the measured-peak model above minus the input the measured
/// process held, so that frame plus reservation cover the measured peak
/// (issue #223; the deep terms measured for `u16` input on 2026-11-15, not
/// derived from the 8-bit ones). For one-shot frames the planes known from
/// the source path (`known_codec_frame_bytes`) stay a floor.
pub(crate) fn lossless_codec_reservation_bytes(
    width: usize,
    height: usize,
    alpha: bool,
    depth: u8,
) -> Result<usize, Error> {
    let pixels = width.checked_mul(height).ok_or(Error::GeometryOverflow)?;
    let i = usize::from(alpha);
    let deep = depth > 8;
    let per_pixel = |bytes: usize| pixels.checked_mul(bytes).ok_or(Error::GeometryOverflow);
    let streams = lossless_streams(width, height);
    let (one_shot, chunked, dc_group) = if deep {
        (
            LOSSLESS_ONE_SHOT_DEEP_BYTES_PER_PIXEL,
            LOSSLESS_CHUNKED_DEEP_BYTES_PER_PIXEL,
            LOSSLESS_CHUNKED_DC_GROUP_DEEP_BYTES_PER_PIXEL,
        )
    } else {
        (
            LOSSLESS_ONE_SHOT_BYTES_PER_PIXEL,
            LOSSLESS_CHUNKED_BYTES_PER_PIXEL,
            LOSSLESS_CHUNKED_DC_GROUP_BYTES_PER_PIXEL,
        )
    };
    let scaled = if streams {
        per_pixel(chunked[i])?
            .checked_add(
                pixels
                    .min(DC_GROUP_PIXELS)
                    .checked_mul(dc_group[i])
                    .ok_or(Error::GeometryOverflow)?,
            )
            .ok_or(Error::GeometryOverflow)?
    } else {
        per_pixel(one_shot[i])?
    };
    let peak = scaled
        .checked_add(LOSSLESS_PEAK_CONSTANT_BYTES)
        .ok_or(Error::GeometryOverflow)?;
    // The measured process held its input once: 3 or 4 samples per pixel
    // of 1 or 2 bytes.
    let measured_input = per_pixel((3 + i) * sample_bytes(depth))?;
    let measured = peak.saturating_sub(measured_input);
    if streams {
        Ok(measured)
    } else {
        Ok(measured.max(known_codec_frame_bytes(width, height, alpha)?))
    }
}

/// Codec-allocated planes that exist at the same time as the input frame in
/// the admitted lossless one-shot path (`ComputeEncodingData` and
/// `compute_encoding_data_lossless_thunder` in the port): an `Image3<f32>`
/// color frame padded to whole 8x8 blocks, one `f32` plane per extra
/// channel, and the Modular `i32` channels of every color and alpha plane.
/// This is a lower bound from the source path: row padding, Modular
/// transforms, tree learning and tokens are not in it. It is the floor of
/// `lossless_codec_reservation_bytes` on the one-shot path, no longer the
/// reservation itself (issue #223: about 28 B/px reserved against about
/// 72 B/px measured).
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

/// The bridge's description of the frame (lossy and lossless): RGB, or
/// RGBA with straight alpha, enum sRGB, at `depth` bits (8 to 16; alpha at
/// the same depth, as `cjxl` writes a PAM).
fn bridge_image(
    width: usize,
    height: usize,
    alpha: bool,
    depth: u8,
) -> Result<bridge::ImageInfo, Error> {
    let width = u32::try_from(width).map_err(|_| Error::GeometryOverflow)?;
    let height = u32::try_from(height).map_err(|_| Error::GeometryOverflow)?;
    Ok(if alpha {
        bridge::ImageInfo::srgba8(width, height)
    } else {
        bridge::ImageInfo::srgb8(width, height)
    }
    .with_bits(depth))
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
    depth: u8,
) -> Result<usize, Error> {
    bridge::estimated_codec_bytes(&bridge_image(width, height, alpha, depth)?)
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
    /// Output bits per sample, 8 to 16 (ADR-0021 amendment of 2026-11-15).
    depth: u8,
    /// Straight-alpha RGBA rows (`u8`, or native-endian `u16` bytes above 8
    /// bits), compacted to RGB in place when opaque.
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
        // AVIF settings, the chroma layout among them (the facade refuses
        // them first).
        if options.avif != AvifOptions::default() || options.max_output_bytes == 0 {
            return Err(Error::InvalidEncodeOptions);
        }
        Ok(())
    }

    fn mode(options: &TransformOptions) -> Result<Mode, Error> {
        match options.jpeg_xl {
            JpegXlOptions::Lossless { effort: 2, .. } => Ok(Mode::Lossless),
            JpegXlOptions::Lossless { .. } => Err(Error::UnsupportedEncoderProfile(
                "only lossless effort 2 is admitted by the workspace encoder",
            )),
            // `contains` is false for NaN.
            JpegXlOptions::Lossy { distance, .. }
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
    /// An 8-bit sink (the contract of the 8-bit tests).
    #[cfg(test)]
    pub(crate) fn new(
        size: Size,
        options: TransformOptions,
        budget: &MemoryBudget,
    ) -> Result<Self, Error> {
        Self::with_depth(size, options, budget, crate::output_depth::EIGHT)
    }

    /// A sink at `depth` bits per sample (8 to 16).
    pub(crate) fn with_depth(
        size: Size,
        options: TransformOptions,
        budget: &MemoryBudget,
        depth: u8,
    ) -> Result<Self, Error> {
        if !(8..=16).contains(&depth) {
            return Err(Error::InvalidEncodeOptions);
        }
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
        let bytes = pixels
            .checked_mul(4 * sample_bytes(depth))
            .ok_or(Error::GeometryOverflow)?;
        Ok(Self {
            width,
            height,
            next_y: 0,
            depth,
            frame: budget.allocate_typed::<u8>(bytes)?,
            budget: budget.clone(),
            has_alpha: false,
            mode,
            max_output_bytes: options.max_output_bytes,
        })
    }

    /// One 8-bit row (the contract of the 8-bit tests).
    #[cfg(test)]
    pub(crate) fn write_row(&mut self, y: u32, bytes: &[u8]) -> Result<(), Error> {
        self.write_output_row(y, OutputRow::U8(bytes))
    }

    /// One row at the sink's depth.
    pub(crate) fn write_output_row(&mut self, y: u32, row: OutputRow<'_>) -> Result<(), Error> {
        let samples = self.width.checked_mul(4).ok_or(Error::GeometryOverflow)?;
        let deep = self.depth > 8;
        if usize::try_from(y).map_err(|_| Error::GeometryOverflow)? != self.next_y
            || self.next_y >= self.height
            || match row {
                OutputRow::U8(bytes) => deep || bytes.len() != samples,
                OutputRow::U16(values) => !deep || values.len() != samples,
            }
        {
            return Err(Error::IncompleteOutput);
        }
        let row_bytes = samples * sample_bytes(self.depth);
        let start = self.next_y * row_bytes;
        let target = &mut self.frame.as_mut_slice()[start..start + row_bytes];
        let transparent = match row {
            OutputRow::U8(bytes) => {
                target.copy_from_slice(bytes);
                bytes.as_chunks::<4>().0.iter().any(|rgba| rgba[3] != 255)
            }
            OutputRow::U16(values) => {
                for (out, value) in target.as_chunks_mut::<2>().0.iter_mut().zip(values) {
                    *out = value.to_ne_bytes();
                }
                let max = crate::output_depth::max_code(self.depth);
                values.as_chunks::<4>().0.iter().any(|rgba| rgba[3] != max)
            }
        };
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
        let sample = sample_bytes(self.depth);
        let frame = self.frame.as_mut_slice();
        if !self.has_alpha {
            // Opaque output carries no alpha channel. Compact in place; each
            // destination index is at most its source index.
            for pixel in 0..pixels {
                frame.copy_within(
                    pixel * 4 * sample..(pixel * 4 + 3) * sample,
                    pixel * 3 * sample,
                );
            }
        }
        let bytes = pixels * channels * sample;
        match self.mode {
            Mode::Lossless => self.finish_lossless(bytes),
            Mode::Lossy { distance } => self.finish_lossy(bytes, distance),
        }
    }

    fn finish_lossless(&self, samples: usize) -> Result<EncodedImage, Error> {
        let codec_bytes =
            lossless_codec_reservation_bytes(self.width, self.height, self.has_alpha, self.depth)?;
        let _codec_frames: ExternalReservation = self.budget.reserve_external(codec_bytes)?;
        let image = bridge_image(self.width, self.height, self.has_alpha, self.depth)?;
        let mut bytes = Vec::new();
        // No limit here: as before, the facade checks `max_output_bytes`
        // after the encoder returns ("encoded output exceeds
        // max_output_bytes").
        bridge::encode_lossless(
            image,
            &self.frame.as_slice()[..samples],
            &mut bytes,
            usize::MAX,
        )
        .map_err(|error| match error {
            // Not reachable for the admitted request (effort 2, 8-bit RGB
            // with optional straight alpha, enum sRGB); typed, never a
            // fallback.
            bridge::Error::NotAdmitted(_) => Error::UnsupportedEncoderProfile(
                "the workspace encoder does not admit this lossless request",
            ),
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

    fn finish_lossy(&self, samples: usize, distance: f32) -> Result<EncodedImage, Error> {
        let codec_bytes =
            estimated_lossy_codec_bytes(self.width, self.height, self.has_alpha, self.depth)?;
        let _codec_frames: ExternalReservation = self.budget.reserve_external(codec_bytes)?;
        let image = bridge_image(self.width, self.height, self.has_alpha, self.depth)?;
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
