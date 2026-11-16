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
use crate::pipeline::{JpegXlOptions, OutputFormat, OutputSettings, TransformOptions};

/// Level 5 dimension and area limits (ISO/IEC 18181-2, Annex A). The raw
/// codestream carries no `jxll` level box, so admitted images must also fit
/// the default level a decoder assumes.
const MAX_LEVEL5_DIMENSION: usize = 1 << 18;
const MAX_LEVEL5_PIXELS: usize = 1 << 28;

/// Pixels of one DC group (2048x2048), the unit the streaming encoder works
/// on one at a time.
const DC_GROUP_PIXELS: usize = 2048 * 2048;

/// The measured peak memory of one encode through the bridge, the call this
/// sink makes, as an upper envelope in bytes (ADR-0004, ADR-0042 D3;
/// issue #223). Source: `docs/benchmarks/v1.0-jxl-encoder-memory-2026-11-16.md`:
/// the maximum resident set size of a fresh process (one at a time; runs
/// whose footprint showed compressed or swapped pages repeated) that holds
/// what this sink holds, its frame of four samples per pixel (compacted in
/// place to three when opaque), and calls the bridge as [`JxlSink::finish`]
/// does, on macOS `AArch64`, for a photograph, a 16-bit photograph and
/// uniform noise (the content that compresses worst) from 0.49 to 24 MP,
/// lossless and at lossy distances 0.05, 1.0 and 2.9, plus the records of
/// 2026-11-15. Every recorded peak is at least 10 % below the model
/// (`jxl_output_tests::model_covers_every_measured_peak_with_margin`). The
/// frame is part of the peak; the codec reservation is the peak minus the
/// frame. It is not a bound for other hosts or allocators.
///
/// Peak resident memory is not the live heap: the allocator keeps memory
/// that was freed, so the peak varies between runs of the same encode by up
/// to about 7 % and depends on the shape of the DC groups, which is why
/// the second DC group has a term of its own.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct PeakModel {
    /// Bytes that do not scale with the image: process, allocator and the
    /// encoder's fixed tables (they dominate below a megapixel).
    pub(crate) constant: usize,
    /// One-shot path (at most 8 groups of 256x256, where the original does
    /// not stream): bytes per pixel of everything else.
    pub(crate) one_shot: usize,
    /// Streaming path (more than 8 groups): bytes per pixel of the whole
    /// frame, the buffers that live through the whole encode: this sink's
    /// frame, the encoder's copy of it (`JxlEncoderAddImageFrame`), the
    /// frame-wide DC-resolution and section data, and the encoded output.
    /// Fitted as the steepest measured slope above two DC groups, of the
    /// peaks or of the live heap, whichever is larger.
    pub(crate) frame: usize,
    /// Streaming: bytes per pixel of the first DC group (the frame if it is
    /// smaller), the working set the encoder allocates for the one DC group
    /// it encodes at a time.
    pub(crate) dc_group: usize,
    /// Streaming: bytes per pixel of the second DC group (the pixels beyond
    /// the first, at most one DC group): resident memory the allocator kept
    /// from the first group's working set while the second group, usually
    /// of another shape, allocated its own. Later groups reuse what the
    /// first two left, so the term stops after one DC group. The model of
    /// 2026-11-15 had no such term and was up to 10 % below measured
    /// 12.2 MP RGBA peaks.
    pub(crate) retained: usize,
}

impl PeakModel {
    /// The modeled peak for a `width` x `height` frame on the streaming or
    /// the one-shot path.
    fn peak_bytes(&self, width: usize, height: usize, streams: bool) -> Result<usize, Error> {
        let pixels = width.checked_mul(height).ok_or(Error::GeometryOverflow)?;
        let term =
            |count: usize, bytes: usize| count.checked_mul(bytes).ok_or(Error::GeometryOverflow);
        let scaled = if streams {
            let first = pixels.min(DC_GROUP_PIXELS);
            let second = pixels.saturating_sub(DC_GROUP_PIXELS).min(DC_GROUP_PIXELS);
            term(pixels, self.frame)?
                .checked_add(term(first, self.dc_group)?)
                .and_then(|sum| sum.checked_add(term(second, self.retained).ok()?))
                .ok_or(Error::GeometryOverflow)?
        } else {
            term(pixels, self.one_shot)?
        };
        scaled
            .checked_add(self.constant)
            .ok_or(Error::GeometryOverflow)
    }
}

/// The lossless model (effort 2, the default buffering), indexed
/// `[depth above 8 bits][alpha]`. Above 8 bits the samples are `u16` (the
/// widest token and plane case, measured at 16 bits) and cover 9 to 15 bits,
/// whose buffers are the same.
pub(crate) const LOSSLESS_PEAK: [[PeakModel; 2]; 2] = [
    [
        PeakModel {
            constant: 12 << 20,
            one_shot: 87,
            frame: 14,
            dc_group: 51,
            retained: 38,
        },
        PeakModel {
            constant: 12 << 20,
            one_shot: 119,
            frame: 16,
            dc_group: 69,
            retained: 67,
        },
    ],
    [
        PeakModel {
            constant: 12 << 20,
            one_shot: 111,
            frame: 24,
            dc_group: 58,
            retained: 56,
        },
        PeakModel {
            constant: 12 << 20,
            one_shot: 135,
            frame: 34,
            dc_group: 71,
            retained: 69,
        },
    ],
];

/// The lossy model (`VarDCT` at effort 7, patches off, any admitted
/// distance; the measured worst of 0.05, 1.0 and 2.9), indexed like
/// [`LOSSLESS_PEAK`]. It replaces the bridge's source-derived estimate
/// (`ferrisma_jxl_encoder::estimated_codec_bytes`), which covered less than a
/// tenth of the measured peak of a 4 MP frame: it counted one 256x256 group's
/// buffers where the encoder works on a whole DC group, and no tokens,
/// histograms or allocator retention.
pub(crate) const LOSSY_PEAK: [[PeakModel; 2]; 2] = [
    [
        PeakModel {
            constant: 20 << 20,
            one_shot: 100,
            frame: 9,
            dc_group: 102,
            retained: 62,
        },
        PeakModel {
            constant: 20 << 20,
            one_shot: 143,
            frame: 11,
            dc_group: 114,
            retained: 70,
        },
    ],
    [
        PeakModel {
            constant: 20 << 20,
            one_shot: 103,
            frame: 16,
            dc_group: 103,
            retained: 63,
        },
        PeakModel {
            constant: 20 << 20,
            one_shot: 143,
            frame: 25,
            dc_group: 114,
            retained: 70,
        },
    ],
];

/// The model for this request: mode, alpha and depth.
pub(crate) fn peak_model(lossless: bool, alpha: bool, depth: u8) -> PeakModel {
    let table = if lossless {
        &LOSSLESS_PEAK
    } else {
        &LOSSY_PEAK
    };
    table[usize::from(depth > 8)][usize::from(alpha)]
}

/// The engine's frame: four samples per pixel, 1 or 2 bytes each.
fn frame_bytes(width: usize, height: usize, depth: u8) -> Result<usize, Error> {
    width
        .checked_mul(height)
        .and_then(|pixels| pixels.checked_mul(4 * sample_bytes(depth)))
        .ok_or(Error::GeometryOverflow)
}

/// The modeled peak minus this sink's frame (reserved when the sink is
/// created): what the codec reservation must cover. The bridge decides
/// whether the frame streams ([`bridge::ImageInfo::streaming`]), so the
/// model follows the encoder's own mode switch.
fn codec_reservation_bytes(
    lossless: bool,
    width: usize,
    height: usize,
    alpha: bool,
    depth: u8,
) -> Result<usize, Error> {
    let streams = bridge_image(width, height, alpha, depth)?.streaming();
    let peak = peak_model(lossless, alpha, depth).peak_bytes(width, height, streams)?;
    Ok(peak.saturating_sub(frame_bytes(width, height, depth)?))
}

/// The budget reservation for the lossless codec, on top of the engine's
/// RGBA input frame (4 B/px at 8 bits, 8 B/px above, reserved when the sink
/// is created): the measured-peak model [`LOSSLESS_PEAK`] minus that frame.
/// For one-shot frames the planes known from the source path
/// (`known_codec_frame_bytes`) stay a floor.
pub(crate) fn lossless_codec_reservation_bytes(
    width: usize,
    height: usize,
    alpha: bool,
    depth: u8,
) -> Result<usize, Error> {
    let measured = codec_reservation_bytes(true, width, height, alpha, depth)?;
    if bridge_image(width, height, alpha, depth)?.streaming() {
        Ok(measured)
    } else {
        Ok(measured.max(known_codec_frame_bytes(width, height, alpha)?))
    }
}

/// The budget reservation for the lossy codec, on top of the engine's frame:
/// the measured-peak model [`LOSSY_PEAK`] minus that frame.
pub(crate) fn lossy_codec_reservation_bytes(
    width: usize,
    height: usize,
    alpha: bool,
    depth: u8,
) -> Result<usize, Error> {
    codec_reservation_bytes(false, width, height, alpha, depth)
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
/// 72 B/px measured on 2026-11-13).
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
        if options.encoder_threads() != 1 {
            return Err(Error::UnsupportedEncoderProfile(
                "the workspace encoder is serial; set encoder_threads to 1",
            ));
        }
        if options.max_output_bytes() == 0 {
            return Err(Error::InvalidEncodeOptions);
        }
        Ok(())
    }

    fn mode(options: &TransformOptions) -> Result<Mode, Error> {
        // Another output's settings never reach this sink: the facade
        // selects it by the settings' format.
        let OutputSettings::JpegXl(jpeg_xl) = options.output() else {
            return Err(Error::InvalidEncodeOptions);
        };
        match jpeg_xl {
            JpegXlOptions::Lossless(lossless) if lossless.effort() == 2 => Ok(Mode::Lossless),
            JpegXlOptions::Lossless(_) => Err(Error::UnsupportedEncoderProfile(
                "only lossless effort 2 is admitted by the workspace encoder",
            )),
            // `contains` is false for NaN.
            JpegXlOptions::Lossy(lossy)
                if (JpegXlOptions::MIN_LOSSY_DISTANCE..JpegXlOptions::MAX_LOSSY_DISTANCE)
                    .contains(&lossy.distance()) =>
            {
                Ok(Mode::Lossy {
                    distance: lossy.distance(),
                })
            }
            JpegXlOptions::Lossy(_) => Err(Error::UnsupportedEncoderProfile(
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
        options: &TransformOptions,
        budget: &MemoryBudget,
    ) -> Result<Self, Error> {
        Self::with_depth(size, options, budget, crate::output_depth::EIGHT)
    }

    /// A sink at `depth` bits per sample (8 to 16).
    pub(crate) fn with_depth(
        size: Size,
        options: &TransformOptions,
        budget: &MemoryBudget,
        depth: u8,
    ) -> Result<Self, Error> {
        if !(8..=16).contains(&depth) {
            return Err(Error::InvalidEncodeOptions);
        }
        Self::validate(options)?;
        let mode = Self::mode(options)?;
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
            max_output_bytes: options.max_output_bytes(),
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
            lossy_codec_reservation_bytes(self.width, self.height, self.has_alpha, self.depth)?;
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
