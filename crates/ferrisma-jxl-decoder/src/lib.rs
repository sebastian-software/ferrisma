//! Narrow, owned safe boundary to the local libjxl decoder.
//! The public decoder has no crop request; all reads reconstruct a full frame.
//! [`decode_region_into`] retains only a requested upright region of that
//! frame through the decoder's image-out callback.
//!
//! Internal to `ferrisma`: this crate follows no SemVer of its own,
//! `ferrisma` requires exactly the version released with it (an `=`
//! requirement), and any release may change this API. It is not for
//! direct use; enable the matching `ferrisma` feature instead (ADR-0040,
//! problem 7).
#![deny(clippy::undocumented_unsafe_blocks)]

use jxl::jxl::codestream_header::JxlBasicInfo;
use jxl::jxl::color_encoding::{self as enc, JxlColorEncoding, TransferFunction as RawTransfer};
use jxl::jxl::decode::{
    self as api, JxlColorProfileTarget, JxlDecoder, JxlDecoderStatus as Status,
};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::cell::Cell;
use std::ffi::c_void;
use std::marker::PhantomData;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::NonNull;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Error {
    Decode,
    Unsupported,
    /// The DATA ICC profile is larger than 16 MiB; reported apart from the
    /// layout refusals of [`Self::Unsupported`].
    ProfileTooLarge,
    /// The codestream is an animation; only still images are decoded.
    Animation,
    PixelLimit {
        pixels: u64,
        limit: u64,
    },
    BufferSize,
    Allocation,
}

#[derive(Clone, Debug)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub channels: u32,
    /// The codestream's alpha is associated (premultiplied). The decoded
    /// samples are straight all the same: every decode asks libjxl to
    /// un-premultiply (`JxlDecoderSetUnpremultiplyAlpha`).
    pub alpha_premultiplied: bool,
    /// The DATA profile describes the decoder's emitted float samples.
    pub rendered_icc: Vec<u8>,
    /// Transfer function of the DATA color encoding when it is enum-signalled;
    /// `None` when the emitted samples are described only by an ICC profile.
    pub data_transfer: Option<Transfer>,
    /// The structured (enum) DATA color encoding, from libjxl's
    /// `JxlDecoderGetColorAsEncodedProfile`; `None` when the emitted samples
    /// are described only by an ICC profile.
    pub data_encoding: Option<EncodedColor>,
    /// The codestream signals its color by an embedded ICC profile
    /// (`want_icc`), not by an enum encoding: libjxl's
    /// `JxlDecoderGetColorAsEncodedProfile` fails for the ORIGINAL target.
    /// For an XYB image the DATA encoding can still be an enum (the decoder
    /// renders into linear sRGB then), so this is the only evidence of the
    /// embedded profile.
    pub original_icc: bool,
    /// Bits per color sample in the codestream header.
    pub bits_per_sample: u32,
    /// Bits per alpha sample; 0 without alpha.
    pub alpha_bits: u32,
    /// The codestream has a preview frame, which is not decoded.
    pub have_preview: bool,
}

/// Transfer function of an enum-signalled color encoding.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Transfer {
    /// SMPTE ST 2084 perceptual quantizer (HDR).
    Pq,
    /// ARIB STD-B67 hybrid log-gamma (HDR).
    Hlg,
    /// Any other transfer function (sRGB, BT.709, linear, DCI, gamma, ...).
    Other,
}

/// A structured JPEG XL color encoding (`JxlColorEncoding`), reduced to
/// the fields that name a color space. Custom chromaticities and the gamma
/// exponent are not carried: an encoding that needs them has no H.273 code
/// point, and its exact description is the synthesized DATA ICC profile.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EncodedColor {
    pub color_space: ColorSpace,
    pub white_point: WhitePoint,
    pub primaries: Primaries,
    pub transfer: TransferFunction,
}

/// `JxlColorSpace`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ColorSpace {
    Rgb,
    Gray,
    Xyb,
    Unknown,
}

/// `JxlWhitePoint`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WhitePoint {
    D65,
    Dci,
    E,
    Custom,
}

/// `JxlPrimaries`; meaningless for gray.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Primaries {
    Srgb,
    Bt2100,
    P3,
    Custom,
}

/// `JxlTransferFunction`.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum TransferFunction {
    Bt709,
    Linear,
    Srgb,
    Pq,
    Dci,
    Hlg,
    Gamma,
    Unknown,
}

impl EncodedColor {
    fn from_raw(raw: &JxlColorEncoding) -> Self {
        let color_space = match raw.color_space {
            enc::ColorSpace::RGB => ColorSpace::Rgb,
            enc::ColorSpace::GRAY => ColorSpace::Gray,
            enc::ColorSpace::XYB => ColorSpace::Xyb,
            _ => ColorSpace::Unknown,
        };
        let white_point = match raw.white_point {
            enc::WhitePoint::D65 => WhitePoint::D65,
            enc::WhitePoint::DCI => WhitePoint::Dci,
            enc::WhitePoint::E => WhitePoint::E,
            _ => WhitePoint::Custom,
        };
        let primaries = match raw.primaries {
            enc::Primaries::SRGB => Primaries::Srgb,
            enc::Primaries::BT2100 => Primaries::Bt2100,
            enc::Primaries::P3 => Primaries::P3,
            _ => Primaries::Custom,
        };
        let transfer = match raw.transfer_function {
            RawTransfer::BT709 => TransferFunction::Bt709,
            RawTransfer::LINEAR => TransferFunction::Linear,
            RawTransfer::SRGB => TransferFunction::Srgb,
            RawTransfer::PQ => TransferFunction::Pq,
            RawTransfer::DCI => TransferFunction::Dci,
            RawTransfer::HLG => TransferFunction::Hlg,
            RawTransfer::GAMMA => TransferFunction::Gamma,
            _ => TransferFunction::Unknown,
        };
        Self {
            color_space,
            white_point,
            primaries,
            transfer,
        }
    }
}

impl Transfer {
    fn of(encoding: &JxlColorEncoding) -> Self {
        match encoding.transfer_function {
            RawTransfer::PQ => Self::Pq,
            RawTransfer::HLG => Self::Hlg,
            _ => Self::Other,
        }
    }
}

impl ImageInfo {
    pub fn sample_count(&self) -> Result<usize, Error> {
        usize::try_from(self.width)
            .ok()
            .and_then(|w| w.checked_mul(self.height as usize))
            .and_then(|n| n.checked_mul(self.channels as usize))
            .ok_or(Error::BufferSize)
    }

    /// Samples of `region`, or `BufferSize` if it is empty or does not fit
    /// the upright image.
    pub fn region_sample_count(&self, region: Region) -> Result<usize, Error> {
        if !region.fits(self) {
            return Err(Error::BufferSize);
        }
        usize::try_from(region.width)
            .ok()
            .and_then(|w| w.checked_mul(region.height as usize))
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

/// A non-empty rectangle of the upright (orientation-applied) image.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Region {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl Region {
    /// The whole upright image.
    pub fn full(info: &ImageInfo) -> Self {
        Self {
            x: 0,
            y: 0,
            width: info.width,
            height: info.height,
        }
    }

    fn fits(self, info: &ImageInfo) -> bool {
        self.width != 0
            && self.height != 0
            && self
                .x
                .checked_add(self.width)
                .is_some_and(|r| r <= info.width)
            && self
                .y
                .checked_add(self.height)
                .is_some_and(|b| b <= info.height)
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
        // Associated (premultiplied) alpha is converted to straight alpha
        // by libjxl's own step (`JxlDecoderSetUnpremultiplyAlpha`, ported
        // in the render pipeline's write stage), so the samples stay
        // bit-identical with the original's (owner decision 2026-11-15,
        // ADR-0043). It has no effect without associated alpha.
        if api::jxl_decoder_set_unpremultiply_alpha(decoder.get(), 1) != Status::SUCCESS {
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
        // or callback is registered until `drive` asks for one; the buffer or
        // the callback's sink then remains live through the FULL_IMAGE event
        // and decoder destruction, and the callback never unwinds or
        // reenters the decoder (see decode_region_into).
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
        if basic.have_animation != 0 {
            return Err(Error::Animation);
        }
        if basic.exponent_bits_per_sample != 0
            || basic.alpha_exponent_bits != 0
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
            data_transfer: None,
            data_encoding: None,
            original_icc: false,
            bits_per_sample: basic.bits_per_sample,
            alpha_bits: basic.alpha_bits,
            have_preview: basic.have_preview != 0,
        })
    }

    /// The enum encoding of `target`, if it has one (libjxl's
    /// `JxlDecoderGetColorAsEncodedProfile`, which fails for an ICC-signalled
    /// encoding).
    fn encoded(&mut self, target: JxlColorProfileTarget) -> Option<JxlColorEncoding> {
        let mut encoding = JxlColorEncoding::default();
        (api::jxl_decoder_get_color_as_encoded_profile(self.get(), target, Some(&mut encoding))
            == Status::SUCCESS)
            .then_some(encoding)
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
            return Err(Error::ProfileTooLarge);
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
                let data = decoder.encoded(JxlColorProfileTarget::DATA);
                info.data_transfer = data.as_ref().map(Transfer::of);
                info.data_encoding = data.as_ref().map(EncodedColor::from_raw);
                info.original_icc = decoder.encoded(JxlColorProfileTarget::ORIGINAL).is_none();
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
    drive(
        input,
        expected,
        max_pixels,
        &mut |decoder, format| {
            let mut bytes = 0;
            if api::jxl_decoder_image_out_buffer_size(decoder, Some(format), Some(&mut bytes))
                != Status::SUCCESS
                || Some(bytes) != output.len().checked_mul(4)
            {
                return Err(Error::BufferSize);
            }
            // SAFETY: output is aligned F32 storage of the exact decoder
            // size; it remains exclusive and live until decoder Drop, which
            // `drive` runs before returning.
            if unsafe {
                api::jxl_decoder_set_image_out_buffer(
                    decoder,
                    Some(format),
                    output.as_mut_ptr().cast(),
                    bytes,
                )
            } != Status::SUCCESS
            {
                return Err(Error::Decode);
            }
            Ok(())
        },
        &|| Ok(()),
    )
}

/// Reconstruct the complete upright image, but retain only `region` of it in
/// a caller-owned F32 buffer of exactly `region` × channels samples, row
/// major. The decoder still reconstructs the full frame (it has no region
/// request); only the retained output shrinks. The samples are bit-identical
/// to the same rectangle of [`decode_into`]'s output.
///
/// # Callback contract
///
/// The rows arrive through `jxl_decoder_set_image_out_callback`, an
/// `unsafe extern "C"` function pointer plus an opaque pointer:
///
/// - **Ownership and lifetime.** The opaque pointer is a shared reference to
///   a stack-owned [`RegionSink`] declared before the decoder is created, and
///   the sink holds the only raw pointer derived from `output`. `drive`
///   destroys the decoder before it returns, so neither pointer is used after
///   the sink and the caller's borrow of `output` end. `output` is not
///   touched through its slice while the decoder exists.
/// - **Arguments.** The port's write stage calls it with upright coordinates
///   `(x, y)`, a pixel count and a pointer to that many initialized pixels in
///   the registered format (one pixel per call for transposing
///   orientations). The sink copies bytes, so it needs no source alignment,
///   and it rejects any run outside the upright image instead of trusting it.
/// - **Panics.** Unwinding out of an `extern "C"` function aborts the
///   process, and the port requires that callbacks never unwind. The body runs
///   under `catch_unwind`; a panic marks the sink failed, later calls are
///   ignored, and the decode returns `Decode` after control is back in Rust.
/// - **Reentrancy and threads.** No parallel runner is registered, so the
///   port calls the callback synchronously on this thread from inside
///   `jxl_decoder_process_input`, one call at a time. The callback never
///   calls into the decoder. The sink's state is in `Cell`s because the
///   wrapper reads it between `process_input` calls through the same shared
///   reference.
/// - **Completeness.** The decode succeeds only after FULL_IMAGE and SUCCESS
///   and if the number of region pixels written equals the region's area; a
///   truncated or failed decode never returns partly written samples as
///   success.
pub fn decode_region_into(
    input: &[u8],
    expected: &ImageInfo,
    region: Region,
    output: &mut [f32],
    max_pixels: u64,
) -> Result<(), Error> {
    if output.len() != expected.region_sample_count(region)? {
        return Err(Error::BufferSize);
    }
    let pixel_bytes = expected.channels as usize * 4;
    let sink = RegionSink {
        output: output.as_mut_ptr().cast(),
        output_bytes: output.len() * 4,
        width: expected.width as usize,
        height: expected.height as usize,
        x0: region.x as usize,
        y0: region.y as usize,
        x1: region.x as usize + region.width as usize,
        y1: region.y as usize + region.height as usize,
        pixel_bytes,
        row_bytes: region.width as usize * pixel_bytes,
        written: Cell::new(0),
        failed: Cell::new(false),
    };
    let opaque = std::ptr::from_ref(&sink).cast_mut().cast::<c_void>();
    drive(
        input,
        expected,
        max_pixels,
        &mut |decoder, format| {
            // SAFETY: See the callback contract above: `opaque` points to
            // `sink`, which outlives the decoder that `drive` destroys before
            // returning; the callback only reads it as `&RegionSink`, never
            // unwinds and never reenters the decoder.
            if unsafe {
                api::jxl_decoder_set_image_out_callback(
                    decoder,
                    Some(format),
                    Some(region_callback),
                    opaque,
                )
            } != Status::SUCCESS
            {
                return Err(Error::Decode);
            }
            Ok(())
        },
        &|| {
            if sink.failed.get() {
                Err(Error::Decode)
            } else {
                Ok(())
            }
        },
    )?;
    let pixels = u64::from(region.width) * u64::from(region.height);
    if sink.failed.get() || sink.written.get() != pixels {
        return Err(Error::Decode);
    }
    Ok(())
}

/// The retained region of one decode; see [`decode_region_into`].
struct RegionSink {
    /// Derived once from the caller's `&mut [f32]`; writable for
    /// `output_bytes` bytes while the decoder exists.
    output: *mut u8,
    output_bytes: usize,
    width: usize,
    height: usize,
    x0: usize,
    y0: usize,
    x1: usize,
    y1: usize,
    pixel_bytes: usize,
    row_bytes: usize,
    written: Cell<u64>,
    failed: Cell<bool>,
}

#[cfg(test)]
thread_local! {
    /// Test-only fault injection: panic inside the callback body.
    static PANIC_IN_CALLBACK: Cell<bool> = const { Cell::new(false) };
}

impl RegionSink {
    /// Copy the part of one emitted upright run that lies in the region.
    /// Returns false for a run the decoder cannot legitimately emit.
    fn accept(&self, x: usize, y: usize, count: usize, pixels: *const u8) -> bool {
        #[cfg(test)]
        if PANIC_IN_CALLBACK.with(Cell::get) {
            panic!("injected image-out callback panic");
        }
        let Some(end) = x.checked_add(count) else {
            return false;
        };
        if pixels.is_null() || y >= self.height || end > self.width {
            return false;
        }
        if y < self.y0 || y >= self.y1 {
            return true;
        }
        let start = x.max(self.x0);
        let stop = end.min(self.x1);
        if start >= stop {
            return true;
        }
        let from = (start - x) * self.pixel_bytes;
        let to = (y - self.y0) * self.row_bytes + (start - self.x0) * self.pixel_bytes;
        let len = (stop - start) * self.pixel_bytes;
        if to + len > self.output_bytes {
            return false;
        }
        // SAFETY: The port passes `count` initialized pixels of
        // `pixel_bytes` each and `from + len <= count * pixel_bytes`.
        // `output` is the caller's exclusive F32 buffer, writable for
        // `output_bytes` (checked above) and disjoint from decoder scratch.
        // A byte copy needs no alignment of either side.
        unsafe {
            std::ptr::copy_nonoverlapping(pixels.add(from), self.output.add(to), len);
        }
        self.written.set(self.written.get() + (stop - start) as u64);
        true
    }
}

/// The image-out callback registered by [`decode_region_into`].
///
/// # Safety
/// `opaque` is the `&RegionSink` registered with the decoder and is live;
/// `pixels` points to `count` initialized pixels of the registered format.
unsafe extern "C" fn region_callback(
    opaque: *mut c_void,
    x: usize,
    y: usize,
    count: usize,
    pixels: *const c_void,
) {
    // SAFETY: The registration in decode_region_into passes a pointer to its
    // stack sink, which outlives the decoder; only shared access is made.
    let sink = unsafe { &*opaque.cast_const().cast::<RegionSink>() };
    if sink.failed.get() {
        return;
    }
    // Never unwind across this extern "C" boundary or the port's frames.
    let accepted =
        catch_unwind(AssertUnwindSafe(|| sink.accept(x, y, count, pixels.cast()))).unwrap_or(false);
    if !accepted {
        sink.failed.set(true);
    }
}

/// The event loop shared by both outputs: header and color checks against
/// `expected`, one output registration, and FULL_IMAGE then SUCCESS. `poll`
/// runs after every event so a failed callback stops the decode early. The
/// decoder is destroyed before this returns.
fn drive(
    input: &[u8],
    expected: &ImageInfo,
    max_pixels: u64,
    register: &mut dyn FnMut(&mut JxlDecoder, &JxlPixelFormat) -> Result<(), Error>,
    poll: &dyn Fn() -> Result<(), Error>,
) -> Result<(), Error> {
    let mut decoder = Decoder::new(input)?;
    let mut checked = false;
    let mut color = false;
    let mut full = false;
    loop {
        let status = decoder.next();
        poll()?;
        match status {
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
                register(decoder.get(), &expected.format())?;
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
    use super::{
        Error, ImageInfo, PANIC_IN_CALLBACK, Region, Transfer, decode_into, decode_region_into,
        inspect,
    };

    const SAMPLE: &[u8] = include_bytes!("../../../tests/fixtures/sample.jxl");
    const ROOT: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../../");

    fn full(bytes: &[u8]) -> (ImageInfo, Vec<f32>) {
        let info = inspect(bytes, 1 << 24).unwrap();
        let mut output = vec![0f32; info.sample_count().unwrap()];
        decode_into(bytes, &info, &mut output, 1 << 24).unwrap();
        (info, output)
    }

    fn region(bytes: &[u8], info: &ImageInfo, region: Region) -> Result<Vec<f32>, Error> {
        let mut output = vec![f32::NAN; info.region_sample_count(region)?];
        decode_region_into(bytes, info, region, &mut output, 1 << 24)?;
        Ok(output)
    }

    fn rect(x: u32, y: u32, width: u32, height: u32) -> Region {
        Region {
            x,
            y,
            width,
            height,
        }
    }

    /// Lossless and lossy fixtures with every EXIF orientation (the
    /// transposing ones reach the callback one pixel per call), gray, gray
    /// with alpha, straight and associated alpha, and XYB.
    fn callback_fixtures() -> Vec<(String, Vec<u8>)> {
        let mut names = vec!["tests/fixtures/sample.jxl".to_owned()];
        for n in 1..=8 {
            names.push(format!("tests/fixtures/orientation/jxl-{n}.jxl"));
            names.push(format!(
                "codecs/libjxl-rs/tests/data/decoder-geometry/orientation-{n}.jxl"
            ));
        }
        for name in [
            "codecs/libjxl-rs/tests/data/jpeg-reconstruction/gray.jxl",
            "codecs/libjxl-rs/tests/data/effects_gray_alpha16.jxl",
            "codecs/libjxl-rs/tests/data/effects_rgb_alpha8.jxl",
            "codecs/libjxl-rs/tests/data/extra-channel-alpha-1-up-1-shift-0.jxl",
            "codecs/libjxl-rs/tests/data/extra-channel-alpha-0-up-1-shift-0.jxl",
            "tests/fixtures/jxl-color/lossless-p3.jxl",
            "tests/fixtures/jxl-color/xyb-p3.jxl",
        ] {
            names.push(name.to_owned());
        }
        names
            .into_iter()
            .map(|name| {
                let bytes = std::fs::read(format!("{ROOT}{name}")).unwrap();
                (name, bytes)
            })
            .collect()
    }

    #[test]
    fn region_callback_matches_the_buffer_path_bit_for_bit() {
        for (name, bytes) in callback_fixtures() {
            let (info, expected) = full(&bytes);
            let (w, h) = (info.width, info.height);
            let channels = info.channels as usize;
            let bits = |samples: &[f32]| samples.iter().map(|s| s.to_bits()).collect::<Vec<_>>();
            let whole = region(&bytes, &info, Region::full(&info)).unwrap();
            assert_eq!(bits(&whole), bits(&expected), "{name}: full region");
            // Corners, 1-pixel crops, odd offsets, the far edges, a single
            // row and a single column.
            let mut regions = vec![
                rect(0, 0, 1, 1),
                rect(w - 1, h - 1, 1, 1),
                rect(w - 1, 0, 1, 1),
                rect(0, h - 1, 1, 1),
                rect(w - 1, 0, 1, h),
                rect(0, h - 1, w, 1),
                rect(0, h / 2, w, 1),
                rect(w / 2, 0, 1, h),
            ];
            if w >= 4 && h >= 4 {
                regions.push(rect(1, 3, w - 2, h - 3));
                // Odd offsets near the center, to the far edges.
                let (x, y) = ((w / 2) | 1, (h / 2) | 1);
                regions.push(rect(x, y, w - x, h - y));
                regions.push(rect(3.min(w - 2), 1, 1.max(w / 3), 1.max(h / 3)));
            }
            for crop in regions {
                let got = region(&bytes, &info, crop).unwrap();
                let mut want = Vec::new();
                for y in crop.y..crop.y + crop.height {
                    let start = (y * w + crop.x) as usize * channels;
                    want.extend_from_slice(
                        &expected[start..start + crop.width as usize * channels],
                    );
                }
                assert_eq!(bits(&got), bits(&want), "{name}: {crop:?}");
            }
        }
    }

    #[test]
    fn region_requests_are_checked_before_decoding() {
        let info = inspect(SAMPLE, 35).unwrap();
        for crop in [
            rect(0, 0, 0, 1),
            rect(0, 0, 1, 0),
            rect(7, 0, 1, 1),
            rect(0, 5, 1, 1),
            rect(6, 0, 2, 1),
            rect(u32::MAX, 0, 2, 1),
            rect(0, u32::MAX, 1, 2),
        ] {
            assert_eq!(info.region_sample_count(crop), Err(Error::BufferSize));
            let mut output = vec![f32::NAN; 3];
            assert_eq!(
                decode_region_into(SAMPLE, &info, crop, &mut output, 35),
                Err(Error::BufferSize),
                "{crop:?}"
            );
        }
        let mut wrong = vec![f32::NAN; 5];
        assert_eq!(
            decode_region_into(SAMPLE, &info, rect(1, 1, 2, 1), &mut wrong, 35),
            Err(Error::BufferSize)
        );
        assert!(wrong.iter().all(|s| s.is_nan()));
    }

    #[test]
    fn region_decode_rejects_truncation_and_foreign_info() {
        let info = inspect(SAMPLE, 35).unwrap();
        let crop = rect(2, 1, 3, 2);
        let mut output = vec![f32::NAN; info.region_sample_count(crop).unwrap()];
        assert_eq!(
            decode_region_into(&SAMPLE[..SAMPLE.len() / 2], &info, crop, &mut output, 35),
            Err(Error::Decode)
        );
        let other = include_bytes!("../../../codecs/libjxl-rs/tests/data/effects_rgb_alpha8.jxl");
        let mut output = vec![f32::NAN; info.region_sample_count(crop).unwrap()];
        assert_eq!(
            decode_region_into(other, &info, crop, &mut output, 1_000),
            Err(Error::Decode)
        );
        // The mismatch is found at basic info, before any callback.
        assert!(output.iter().all(|sample| sample.is_nan()));
        assert_eq!(
            decode_region_into(SAMPLE, &info, crop, &mut output, 34),
            Err(Error::PixelLimit {
                pixels: 35,
                limit: 34
            })
        );
    }

    #[test]
    fn a_panicking_callback_fails_the_decode_without_unwinding_through_the_port() {
        let info = inspect(SAMPLE, 35).unwrap();
        let crop = rect(1, 1, 5, 3);
        let mut output = vec![0f32; info.region_sample_count(crop).unwrap()];
        PANIC_IN_CALLBACK.with(|flag| flag.set(true));
        let result = decode_region_into(SAMPLE, &info, crop, &mut output, 35);
        PANIC_IN_CALLBACK.with(|flag| flag.set(false));
        assert_eq!(result, Err(Error::Decode));
        // The decoder was destroyed normally; the next decode succeeds.
        decode_region_into(SAMPLE, &info, crop, &mut output, 35).unwrap();
        assert!(output.iter().all(|sample| sample.is_finite()));
    }

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
    fn rejects_layouts_outside_the_engine_contract_before_pixels() {
        const DATA: &str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../codecs/libjxl-rs/tests/data/"
        );
        // Float alpha (exponent bits), spot colors and several alpha/spot
        // channels are each reported as Unsupported. The external conformance
        // run covers animation and CMYK rejection.
        for name in [
            "modular-rgb-alpha-float24e7.jxl",
            "extra-channel-spot-signed-extreme-binary-name.jxl",
            "extra-channel-multiple-alpha-spots-float.jxl",
        ] {
            let bytes = std::fs::read(format!("{DATA}{name}")).unwrap();
            assert_eq!(
                inspect(&bytes, 1 << 30).unwrap_err(),
                Error::Unsupported,
                "{name}"
            );
        }
    }

    #[test]
    fn reports_animation_as_its_own_reason() {
        let animated = include_bytes!(
            "../../../codecs/libjxl-rs/tests/data/decoder-geometry/sequence-reference-overwrite-animation-1.jxl"
        );
        assert_eq!(inspect(animated, 1 << 30).unwrap_err(), Error::Animation);
    }

    #[test]
    fn exposes_the_data_transfer_of_enum_encodings() {
        const COLOR: &str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/jxl-color/"
        );
        for (name, transfer) in [
            ("lossless-p3", Transfer::Other),
            ("lossless-pq", Transfer::Pq),
            ("lossless-hlg", Transfer::Hlg),
            ("lossless-custom-pq", Transfer::Pq),
            ("lossless-custom-hlg", Transfer::Hlg),
            ("lossless-e-white-pq", Transfer::Pq),
            ("xyb-custom-pq", Transfer::Pq),
        ] {
            let bytes = std::fs::read(format!("{COLOR}{name}.jxl")).unwrap();
            let info = inspect(&bytes, 1 << 20).unwrap();
            assert_eq!(info.data_transfer, Some(transfer), "{name}");
        }
    }

    /// Which of libjxl's two color routes a file has (#222): an embedded
    /// profile (`original_icc`, no enum DATA encoding) or an enum encoding,
    /// exposed field by field.
    #[test]
    fn exposes_the_embedded_profile_signal_and_the_enum_data_encoding() {
        use super::{ColorSpace, EncodedColor, Primaries, TransferFunction, WhitePoint};
        let read = |path: &str| std::fs::read(format!("{ROOT}{path}")).unwrap();
        let icc = inspect(&read("tests/fixtures/parity/rgb-p3-icc.jxl"), 1 << 20).unwrap();
        assert!(icc.original_icc);
        assert_eq!(icc.data_encoding, None);
        assert_eq!(icc.data_transfer, None);
        for (path, expected) in [
            (
                "tests/fixtures/jxl-color/lossless-p3.jxl",
                EncodedColor {
                    color_space: ColorSpace::Rgb,
                    white_point: WhitePoint::D65,
                    primaries: Primaries::P3,
                    transfer: TransferFunction::Srgb,
                },
            ),
            (
                "tests/fixtures/jxl-color/lossless-dci-p3.jxl",
                EncodedColor {
                    color_space: ColorSpace::Rgb,
                    white_point: WhitePoint::Dci,
                    primaries: Primaries::P3,
                    transfer: TransferFunction::Dci,
                },
            ),
            (
                "tests/fixtures/sample.jxl",
                EncodedColor {
                    color_space: ColorSpace::Rgb,
                    white_point: WhitePoint::D65,
                    primaries: Primaries::Srgb,
                    transfer: TransferFunction::Srgb,
                },
            ),
        ] {
            let info = inspect(&read(path), 1 << 20).unwrap();
            assert!(!info.original_icc, "{path}");
            assert_eq!(info.data_encoding, Some(expected), "{path}");
        }
        let gray = inspect(
            &read("codecs/libjxl-rs/tests/data/jpeg-reconstruction/gray.jxl"),
            1 << 20,
        )
        .unwrap();
        let gray = gray.data_encoding.unwrap();
        assert_eq!(
            (gray.color_space, gray.white_point, gray.transfer),
            (ColorSpace::Gray, WhitePoint::D65, TransferFunction::Srgb)
        );
    }

    #[test]
    fn decode_rejects_info_from_another_image_without_registering_output() {
        let info = inspect(SAMPLE, 35).unwrap();
        let other = include_bytes!("../../../codecs/libjxl-rs/tests/data/effects_rgb_alpha8.jxl");
        let mut output = vec![f32::NAN; info.sample_count().unwrap()];
        assert_eq!(
            decode_into(other, &info, &mut output, 1_000),
            Err(Error::Decode)
        );
        // The mismatch is found at basic info, before any output buffer.
        assert!(output.iter().all(|sample| sample.is_nan()));
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
