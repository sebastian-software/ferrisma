//! Still WebP output through the workspace port's safe encoder API.

use std::borrow::Cow;

use webp::api::encode::{Config, EncodeError as WebpError, Encoder, Picture, Preset};
use webp::mux::muxedit::{webp_mux_assemble, webp_mux_set_chunk, webp_mux_set_image};
use webp::webp::mux::webp_mux_new;
use webp::webp::mux_types::WebPData;

use crate::encoder_backend::EncodedImage;
use crate::error::Error;
use crate::geometry::Size;
use crate::memory::{ExternalReservation, ManagedBuffer, MemoryBudget};
use crate::pipeline::{OutputFormat, OutputSettings, Threads, TransformOptions, WebpOptions};

// The unchanged ICC sRGB2014 profile (assets/icc/README.md) describes the
// emitted sRGB8 samples. It is a package-local copy so the published crate
// does not depend on the libavif upstream snapshot.
const SRGB_ICC: &[u8] = include_bytes!("../assets/icc/sRGB2014.icc");
const ICC_CHUNK_BYTES: usize = 8 + SRGB_ICC.len().next_multiple_of(2);

/// Parse the total padded RIFF byte length (`8 + padded_payload`) from the
/// first WebP chunk when it carries a RIFF/WEBP header.
fn riff_total_bytes(first_chunk: &[u8]) -> Option<usize> {
    if first_chunk.len() < 12 || &first_chunk[..4] != b"RIFF" || &first_chunk[8..12] != b"WEBP" {
        return None;
    }
    let payload = usize::try_from(u32::from_le_bytes([
        first_chunk[4],
        first_chunk[5],
        first_chunk[6],
        first_chunk[7],
    ]))
    .ok()?;
    let padded = payload.checked_add(payload & 1)?;
    8_usize.checked_add(padded)
}

struct WebpChunkWriter {
    bytes: Vec<u8>,
    max_output_bytes: usize,
    max_codec_bytes: usize,
    failure: Option<Error>,
}

impl WebpChunkWriter {
    fn new(max_output_bytes: usize) -> Self {
        Self {
            bytes: Vec::new(),
            max_output_bytes,
            max_codec_bytes: max_output_bytes.saturating_sub(ICC_CHUNK_BYTES),
            failure: None,
        }
    }

    fn push_chunk(&mut self, chunk: &[u8]) -> bool {
        let Some(new_len) = self.bytes.len().checked_add(chunk.len()) else {
            self.failure = Some(Error::GeometryOverflow);
            return false;
        };
        // The ICC chunk is always added. VP8X may already be present.
        if new_len > self.max_codec_bytes {
            // The ICC chunk is added after the codec's chunks.
            self.failure = Some(Error::OutputLimitExceeded {
                needed: new_len.saturating_add(ICC_CHUNK_BYTES),
                limit: self.max_output_bytes,
            });
            return false;
        }
        if self.bytes.is_empty()
            && let Some(riff_total) = riff_total_bytes(chunk)
        {
            if riff_total > self.max_codec_bytes {
                self.failure = Some(Error::OutputLimitExceeded {
                    needed: riff_total.saturating_add(ICC_CHUNK_BYTES),
                    limit: self.max_output_bytes,
                });
                return false;
            }
            if self
                .bytes
                .try_reserve_exact(riff_total.max(chunk.len()))
                .is_err()
            {
                self.failure = Some(Error::AllocationFailed);
                return false;
            }
        } else if self.bytes.try_reserve(chunk.len()).is_err() {
            self.failure = Some(Error::AllocationFailed);
            return false;
        }
        self.bytes.extend_from_slice(chunk);
        true
    }
}

/// Encode `picture` into a `Vec<u8>`, reserving the checked padded RIFF size
/// from the first header chunk so subsequent chunks (including a trailing
/// 1-byte RIFF pad byte) do not reallocate.
fn encode_webp_chunks_observed(
    config: &Config,
    picture: &mut Picture<'_>,
    max_output_bytes: usize,
    mut on_chunk: impl FnMut(&[u8], &Vec<u8>),
) -> Result<Vec<u8>, Error> {
    let mut writer = WebpChunkWriter::new(max_output_bytes);
    let result = Encoder::new(config).encode_to(picture, |chunk| {
        let ok = writer.push_chunk(chunk);
        if ok {
            on_chunk(chunk, &writer.bytes);
        }
        ok
    });
    if let Some(error) = writer.failure {
        return Err(error);
    }
    result.map_err(|error| match error {
        WebpError::OutOfMemory | WebpError::BitstreamOutOfMemory => Error::AllocationFailed,
        WebpError::InvalidConfiguration
        | WebpError::InvalidParameter { .. }
        | WebpError::InvalidDimensions { .. }
        | WebpError::BadDimension => Error::InvalidEncodeOptions,
        WebpError::UserAbort => Error::Cancelled,
        _ => Error::EncodeFailed,
    })?;
    Ok(writer.bytes)
}

#[cfg(all(test, feature = "webp-read"))]
pub(crate) fn encode_webp_chunks_for_test(
    config: &Config,
    picture: &mut Picture<'_>,
    max_output_bytes: usize,
    on_chunk: impl FnMut(&[u8], &Vec<u8>),
) -> Result<Vec<u8>, Error> {
    encode_webp_chunks_observed(config, picture, max_output_bytes, on_chunk)
}

pub(crate) struct WebpSink {
    width: usize,
    height: usize,
    next_y: usize,
    frame: ManagedBuffer<u32>,
    budget: MemoryBudget,
    config: Config,
    lossy: bool,
    has_alpha: bool,
    max_output_bytes: usize,
}

impl WebpSink {
    pub(crate) fn new(
        size: Size,
        options: &TransformOptions,
        budget: &MemoryBudget,
    ) -> Result<Self, Error> {
        let OutputSettings::WebP(webp) = options.output() else {
            return Err(Error::InvalidEncodeOptions);
        };
        if size.width > 16_383
            || size.height > 16_383
            || options.encoder_threads() != Threads::ONE
            || options.max_output_bytes() == 0
        {
            return Err(Error::InvalidEncodeOptions);
        }
        let width = usize::try_from(size.width).map_err(|_| Error::GeometryOverflow)?;
        let height = usize::try_from(size.height).map_err(|_| Error::GeometryOverflow)?;
        let pixels = width.checked_mul(height).ok_or(Error::GeometryOverflow)?;
        let config = match webp {
            WebpOptions::Lossless(lossless) if lossless.level() <= 9 => {
                Config::new().lossless_level(lossless.level())
            }
            // libwebp's own method, passed unchanged (ADR-0043 D4).
            WebpOptions::Lossy(lossy)
                if lossy.quality().is_finite()
                    && (1.0..=100.0).contains(&lossy.quality())
                    && lossy.method() <= WebpOptions::MAX_METHOD =>
            {
                Config::preset(Preset::Default, lossy.quality())
                    .map(|config| config.method(lossy.method()))
            }
            _ => return Err(Error::InvalidEncodeOptions),
        }
        .map_err(|_| Error::InvalidEncodeOptions)?
        .exact(true);
        Ok(Self {
            width,
            height,
            next_y: 0,
            frame: budget.allocate_typed::<u32>(pixels)?,
            budget: budget.clone(),
            config,
            lossy: matches!(webp, WebpOptions::Lossy(_)),
            has_alpha: false,
            max_output_bytes: options.max_output_bytes(),
        })
    }

    pub(crate) fn write_row(&mut self, y: u32, bytes: &[u8]) -> Result<(), Error> {
        if usize::try_from(y).map_err(|_| Error::GeometryOverflow)? != self.next_y
            || self.next_y >= self.height
            || bytes.len() != self.width.checked_mul(4).ok_or(Error::GeometryOverflow)?
        {
            return Err(Error::IncompleteOutput);
        }
        let start = self.next_y * self.width;
        for (out, rgba) in self.frame.as_mut_slice()[start..start + self.width]
            .iter_mut()
            .zip(bytes.as_chunks::<4>().0)
        {
            self.has_alpha |= rgba[3] != 255;
            *out = u32::from_be_bytes([rgba[3], rgba[0], rgba[1], rgba[2]]);
        }
        self.next_y += 1;
        Ok(())
    }

    #[allow(clippy::too_many_lines)] // Reservations, encode, and mux stay in one lifetime scope.
    pub(crate) fn finish(mut self) -> Result<EncodedImage, Error> {
        if self.next_y != self.height {
            return Err(Error::IncompleteOutput);
        }
        let input_bytes = self
            .width
            .checked_mul(self.height)
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or(Error::GeometryOverflow)?;
        // libwebp converts ARGB to YUV420(A) for lossy output. Reserve those
        // predictable owned planes before asking the codec to allocate them.
        let converted_bytes = if self.lossy {
            let pixels = self.width * self.height;
            pixels
                .checked_add(
                    self.width
                        .div_ceil(2)
                        .checked_mul(self.height.div_ceil(2))
                        .and_then(|chroma| chroma.checked_mul(2))
                        .ok_or(Error::GeometryOverflow)?,
                )
                .and_then(|samples| samples.checked_add(if self.has_alpha { pixels } else { 0 }))
                .ok_or(Error::GeometryOverflow)?
        } else {
            0
        };
        let _converted: ExternalReservation = self.budget.reserve_external(converted_bytes)?;
        let mut picture = Picture::from_argb(
            self.frame.as_mut_slice(),
            u32::try_from(self.width).map_err(|_| Error::GeometryOverflow)?,
            u32::try_from(self.height).map_err(|_| Error::GeometryOverflow)?,
            self.width,
        )
        .map_err(|_| Error::InvalidEncodeOptions)?;
        let bytes = encode_webp_chunks_observed(
            &self.config,
            &mut picture,
            self.max_output_bytes,
            |_, _| {},
        )?;
        let actual_converted = picture.owned_sample_bytes();
        let vp8x_bytes = if bytes.get(12..16) == Some(b"VP8X".as_slice()) {
            0
        } else {
            18
        };
        let needed = bytes
            .len()
            .checked_add(ICC_CHUNK_BYTES + vp8x_bytes)
            .ok_or(Error::GeometryOverflow)?;
        if needed > self.max_output_bytes {
            return Err(Error::OutputLimitExceeded {
                needed,
                limit: self.max_output_bytes,
            });
        }
        let image = WebPData {
            bytes: Cow::Borrowed(&bytes),
        };
        let profile = WebPData {
            bytes: Cow::Borrowed(SRGB_ICC),
        };
        let mut mux = webp_mux_new().ok_or(Error::AllocationFailed)?;
        webp_mux_set_image(&mut mux, &image, 0).map_err(|_| Error::EncodeFailed)?;
        webp_mux_set_chunk(&mut mux, b"ICCP", &profile, 0).map_err(|_| Error::EncodeFailed)?;
        let encoded = webp_mux_assemble(&mut mux).map_err(|_| Error::EncodeFailed)?;
        if encoded.bytes.len() > self.max_output_bytes {
            return Err(Error::OutputLimitExceeded {
                needed: encoded.bytes.len(),
                limit: self.max_output_bytes,
            });
        }
        Ok(EncodedImage::new(
            OutputFormat::WebP,
            encoded.bytes.into_owned(),
            input_bytes + actual_converted,
            true,
        ))
    }
}
