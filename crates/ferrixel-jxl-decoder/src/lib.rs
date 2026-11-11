//! Narrow, owned safe boundary to the local libjxl decoder.
//! The public decoder has no crop request; all reads reconstruct a full frame.
#![deny(clippy::undocumented_unsafe_blocks)]

use jxl::jxl::codestream_header::JxlBasicInfo;
use jxl::jxl::decode::{
    self as api, JxlColorProfileTarget, JxlDecoder, JxlDecoderStatus as Status,
};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::marker::PhantomData;
use std::ptr::NonNull;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Decode,
    Unsupported,
    PixelLimit { pixels: u64, limit: u64 },
    BufferSize,
    Allocation,
}

#[derive(Clone, Debug)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub channels: u32,
    pub alpha_premultiplied: bool,
    /// The DATA profile describes the decoder's emitted float samples.
    pub rendered_icc: Vec<u8>,
}

impl ImageInfo {
    pub fn sample_count(&self) -> Result<usize, Error> {
        usize::try_from(self.width)
            .ok()
            .and_then(|w| w.checked_mul(self.height as usize))
            .and_then(|n| n.checked_mul(self.channels as usize))
            .ok_or(Error::BufferSize)
    }

    fn format(&self) -> JxlPixelFormat {
        JxlPixelFormat {
            num_channels: self.channels,
            data_type: JxlDataType::FLOAT,
            endianness: JxlEndianness::NATIVE,
            align: 0,
        }
    }
}

/// Owns a decoder while its borrowed encoded input is live. Its Drop runs
/// before that input can be freed; caller pixel buffers are never retained
/// past `decode_into`.
struct Decoder<'a> {
    raw: NonNull<JxlDecoder>,
    _input: PhantomData<&'a [u8]>,
}

impl<'a> Decoder<'a> {
    fn new(input: &'a [u8]) -> Result<Self, Error> {
        // SAFETY: A null manager selects libjxl's default allocator. The
        // returned pointer is uniquely owned here and destroyed in Drop.
        let raw = NonNull::new(unsafe { api::jxl_decoder_create(std::ptr::null()) })
            .ok_or(Error::Allocation)?;
        let mut decoder = Self {
            raw,
            _input: PhantomData,
        };
        let events =
            (Status::BASIC_INFO.0 | Status::COLOR_ENCODING.0 | Status::FULL_IMAGE.0) as i32;
        if api::jxl_decoder_subscribe_events(decoder.get(), events) != Status::SUCCESS {
            return Err(Error::Decode);
        }
        // Default keep_orientation=false makes dimensions and samples upright.
        // SAFETY: `input` is immutable and remains live through this decoder's
        // lifetime; a zero length input still has a valid slice pointer.
        if unsafe { api::jxl_decoder_set_input(decoder.get(), input.as_ptr(), input.len()) }
            != Status::SUCCESS
        {
            return Err(Error::Decode);
        }
        api::jxl_decoder_close_input(decoder.get());
        Ok(decoder)
    }

    fn get(&mut self) -> &mut JxlDecoder {
        // SAFETY: `raw` is uniquely owned, live until Drop, and no other
        // reference to the decoder crosses this call.
        unsafe { self.raw.as_mut() }
    }

    fn next(&mut self) -> Status {
        // SAFETY: The borrowed input is live and immutable. No output buffer
        // is registered until decode_into, where it remains live through the
        // FULL_IMAGE event and decoder destruction.
        unsafe { api::jxl_decoder_process_input(self.get()) }
    }

    fn info(&mut self, max_pixels: u64) -> Result<ImageInfo, Error> {
        let mut basic = JxlBasicInfo::zeroed();
        if api::jxl_decoder_get_basic_info(self.get(), Some(&mut basic)) != Status::SUCCESS {
            return Err(Error::Decode);
        }
        let pixels = u64::from(basic.xsize) * u64::from(basic.ysize);
        if pixels == 0 {
            return Err(Error::Decode);
        }
        if pixels > max_pixels {
            return Err(Error::PixelLimit {
                pixels,
                limit: max_pixels,
            });
        }
        if basic.exponent_bits_per_sample != 0
            || basic.alpha_exponent_bits != 0
            || basic.have_animation != 0
            || basic.num_color_channels != 1 && basic.num_color_channels != 3
        {
            return Err(Error::Unsupported);
        }
        let alpha = basic.alpha_bits != 0;
        if basic.num_extra_channels != u32::from(alpha) {
            return Err(Error::Unsupported);
        }
        let channels = basic.num_color_channels + u32::from(alpha);
        Ok(ImageInfo {
            width: basic.xsize,
            height: basic.ysize,
            channels,
            alpha_premultiplied: alpha && basic.alpha_premultiplied != 0,
            rendered_icc: Vec::new(),
        })
    }

    fn data_icc(&mut self) -> Result<Vec<u8>, Error> {
        let mut size = 0;
        if api::jxl_decoder_get_icc_profile_size(
            self.get(),
            JxlColorProfileTarget::DATA,
            Some(&mut size),
        ) != Status::SUCCESS
            || size > 16 * 1024 * 1024
        {
            return Err(Error::Unsupported);
        }
        let mut profile = Vec::new();
        profile
            .try_reserve_exact(size)
            .map_err(|_| Error::Allocation)?;
        profile.resize(size, 0);
        // SAFETY: The vector is writable for `size` bytes and lives through
        // the getter; the decoder cannot retain this output pointer.
        if unsafe {
            api::jxl_decoder_get_color_as_icc_profile(
                self.get(),
                JxlColorProfileTarget::DATA,
                profile.as_mut_ptr(),
                size,
            )
        } != Status::SUCCESS
        {
            return Err(Error::Decode);
        }
        Ok(profile)
    }
}

impl Drop for Decoder<'_> {
    fn drop(&mut self) {
        // SAFETY: This is the sole owner of a pointer returned by create;
        // destruction also releases any registered input or output pointer.
        unsafe { api::jxl_decoder_destroy(self.raw.as_ptr()) };
    }
}

/// Parse dimensions and emitted color metadata before allocating pixels.
pub fn inspect(input: &[u8], max_pixels: u64) -> Result<ImageInfo, Error> {
    let mut decoder = Decoder::new(input)?;
    let mut info = None;
    loop {
        match decoder.next() {
            Status::BASIC_INFO => info = Some(decoder.info(max_pixels)?),
            Status::COLOR_ENCODING => {
                let mut info = info.ok_or(Error::Decode)?;
                info.rendered_icc = decoder.data_icc()?;
                return Ok(info);
            }
            Status::ERROR | Status::NEED_MORE_INPUT | Status::SUCCESS => return Err(Error::Decode),
            _ => return Err(Error::Decode),
        }
    }
}

/// Reconstruct the complete upright image into a caller-owned F32 buffer.
/// The pixel limit is checked again before registering that buffer.
pub fn decode_into(
    input: &[u8],
    expected: &ImageInfo,
    output: &mut [f32],
    max_pixels: u64,
) -> Result<(), Error> {
    if output.len() != expected.sample_count()? {
        return Err(Error::BufferSize);
    }
    let mut decoder = Decoder::new(input)?;
    let mut checked = false;
    let mut color = false;
    let mut full = false;
    loop {
        match decoder.next() {
            Status::BASIC_INFO => {
                let actual = decoder.info(max_pixels)?;
                if actual.width != expected.width
                    || actual.height != expected.height
                    || actual.channels != expected.channels
                    || actual.alpha_premultiplied != expected.alpha_premultiplied
                {
                    return Err(Error::Decode);
                }
                checked = true;
            }
            Status::COLOR_ENCODING => {
                if !checked || decoder.data_icc()? != expected.rendered_icc {
                    return Err(Error::Decode);
                }
                color = true;
            }
            Status::NEED_IMAGE_OUT_BUFFER => {
                if !color || full {
                    return Err(Error::Decode);
                }
                let format = expected.format();
                let mut bytes = 0;
                if api::jxl_decoder_image_out_buffer_size(
                    decoder.get(),
                    Some(&format),
                    Some(&mut bytes),
                ) != Status::SUCCESS
                    || Some(bytes) != output.len().checked_mul(4)
                {
                    return Err(Error::BufferSize);
                }
                // SAFETY: output is aligned F32 storage of the exact decoder
                // size; it remains exclusive and live until decoder Drop.
                if unsafe {
                    api::jxl_decoder_set_image_out_buffer(
                        decoder.get(),
                        Some(&format),
                        output.as_mut_ptr().cast(),
                        bytes,
                    )
                } != Status::SUCCESS
                {
                    return Err(Error::Decode);
                }
            }
            Status::FULL_IMAGE => full = true,
            Status::SUCCESS if full => return Ok(()),
            Status::ERROR | Status::NEED_MORE_INPUT | Status::SUCCESS => return Err(Error::Decode),
            _ => return Err(Error::Unsupported),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Error, decode_into, inspect};

    const SAMPLE: &[u8] = include_bytes!("../../../tests/fixtures/sample.jxl");

    #[test]
    fn checks_pixel_limit_before_output() {
        assert_eq!(
            inspect(SAMPLE, 34).unwrap_err(),
            Error::PixelLimit {
                pixels: 35,
                limit: 34
            }
        );
        let info = inspect(SAMPLE, 35).unwrap();
        assert_eq!((info.width, info.height, info.channels), (7, 5, 3));
        let mut short = [0f32; 1];
        assert_eq!(
            decode_into(SAMPLE, &info, &mut short, 35),
            Err(Error::BufferSize)
        );
    }

    #[test]
    fn rejects_missing_tail_before_returning_pixels() {
        let info = inspect(SAMPLE, 35).unwrap();
        let mut output = vec![0f32; info.sample_count().unwrap()];
        assert_eq!(
            decode_into(&SAMPLE[..SAMPLE.len() / 2], &info, &mut output, 35),
            Err(Error::Decode)
        );
        decode_into(SAMPLE, &info, &mut output, 35).unwrap();
        assert!(output.iter().all(|sample| sample.is_finite()));
    }

    #[test]
    fn preserves_grayscale_and_alpha_association() {
        let gray =
            include_bytes!("../../../codecs/libjxl-rs/tests/data/jpeg-reconstruction/gray.jxl");
        let gray = inspect(gray, 1_000).unwrap();
        assert_eq!(gray.channels, 1);
        let gray_alpha =
            include_bytes!("../../../codecs/libjxl-rs/tests/data/effects_gray_alpha16.jxl");
        let gray_alpha = inspect(gray_alpha, 1_000).unwrap();
        assert_eq!(gray_alpha.channels, 2);
        let associated = include_bytes!(
            "../../../codecs/libjxl-rs/tests/data/extra-channel-alpha-1-up-1-shift-0.jxl"
        );
        let associated = inspect(associated, 1_000).unwrap();
        assert_eq!(associated.channels, 4);
        assert!(associated.alpha_premultiplied);
        let straight = include_bytes!(
            "../../../codecs/libjxl-rs/tests/data/extra-channel-alpha-0-up-1-shift-0.jxl"
        );
        let straight = inspect(straight, 1_000).unwrap();
        assert_eq!(straight.channels, 4);
        assert!(!straight.alpha_premultiplied);
    }
}
