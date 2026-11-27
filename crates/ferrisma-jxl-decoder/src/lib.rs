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
#[cfg(test)]
use jxl::threads::thread_parallel_runner::jxl_thread_parallel_runner_spawned_threads;
use jxl::threads::thread_parallel_runner::{
    RunnerError, jxl_thread_parallel_runner_create_lazy, jxl_thread_parallel_runner_destroy,
    jxl_thread_parallel_runner_last_error,
};
use std::ffi::c_void;
use std::marker::PhantomData;
use std::panic::{AssertUnwindSafe, catch_unwind};
use std::ptr::NonNull;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};

/// Upper bound on worker threads created for a single decode, matching the
/// workspace encoder bridges and keeping `num_worker_threads` safely within
/// libjxl's `i32` constructor parameter.
const MAX_THREADS: usize = 256;

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
    /// Starting the parallel decode worker pool failed.
    ThreadSpawn,
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

/// Owns a lazy libjxl thread parallel runner created by
/// `jxl_thread_parallel_runner_create_lazy`. Zero OS threads are spawned at
/// creation time; single-task ranges run on the calling thread, and multi-task
/// ranges grow the worker pool on demand up to `min(max_workers, num_tasks)`.
/// Held as a field of [`Decoder`] so `Decoder::drop` destroys the `JxlDecoder`
/// before this runner is dropped (Rust runs `Drop::drop` before dropping
/// struct fields).
struct ParallelRunner {
    raw: NonNull<c_void>,
}

impl ParallelRunner {
    fn map_runner_error(err: RunnerError) -> Error {
        match err {
            RunnerError::Allocation => Error::Allocation,
            RunnerError::ThreadSpawn => Error::ThreadSpawn,
            RunnerError::InvalidMemoryManager => Error::Decode,
        }
    }

    fn new(threads: usize) -> Result<Self, Error> {
        #[cfg(test)]
        if let Some(err) = FAIL_RUNNER_CREATE.with(std::cell::Cell::get) {
            return Err(Self::map_runner_error(err));
        }
        let workers = threads.clamp(1, MAX_THREADS);
        // SAFETY: A null manager selects libjxl's default allocator. The
        // returned pointer is uniquely owned here and destroyed in Drop;
        // errors distinguish allocation failure from OS thread-spawn failure.
        let raw = unsafe { jxl_thread_parallel_runner_create_lazy(std::ptr::null(), workers) }
            .map_err(Self::map_runner_error)?;
        let raw = NonNull::new(raw).ok_or(Error::Allocation)?;
        Ok(Self { raw })
    }

    fn last_error(&self) -> Option<Error> {
        // SAFETY: `self.raw` is a live runner owned by `self`.
        unsafe { jxl_thread_parallel_runner_last_error(self.raw.as_ptr()) }
            .map(Self::map_runner_error)
    }

    #[cfg(test)]
    fn spawned_threads(&self) -> usize {
        // SAFETY: `self.raw` is a live runner owned by `self`.
        unsafe { jxl_thread_parallel_runner_spawned_threads(self.raw.as_ptr()) }
    }
}

#[cfg(test)]
thread_local! {
    /// Test-only fault injection: simulate a `RunnerError` from runner creation
    /// on this test thread.
    static FAIL_RUNNER_CREATE: std::cell::Cell<Option<RunnerError>> =
        const { std::cell::Cell::new(None) };
    /// Test-only observation: worker threads spawned by the last `drive` call
    /// on this thread.
    static LAST_SPAWNED_THREADS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    /// Test-only observation: whether `region_callback` ran on at least one
    /// non-caller worker thread during the last `decode_region_into*` call.
    static LAST_CALLBACK_ON_WORKER: std::cell::Cell<bool> = const { std::cell::Cell::new(false) };
}

impl Drop for ParallelRunner {
    fn drop(&mut self) {
        // SAFETY: `self.raw` came from `jxl_thread_parallel_runner_create_lazy`,
        // is uniquely owned here, and any decoder referencing it has already
        // been destroyed in `Decoder::drop`.
        unsafe { jxl_thread_parallel_runner_destroy(self.raw.as_ptr()) };
    }
}

/// Owns a decoder while its borrowed encoded input is live. Its Drop runs
/// before its parallel runner and before that input can be freed; caller
/// pixel buffers are never retained past `decode_into`.
struct Decoder<'a> {
    raw: NonNull<JxlDecoder>,
    /// Kept alive strictly longer than `raw`: `Decoder::drop` destroys `raw`
    /// first, then Rust drops `_runner`.
    _runner: Option<ParallelRunner>,
    _input: PhantomData<&'a [u8]>,
}

impl<'a> Decoder<'a> {
    fn new(input: &'a [u8]) -> Result<Self, Error> {
        Self::new_with_threads(input, 1)
    }

    fn new_with_threads(input: &'a [u8], threads: usize) -> Result<Self, Error> {
        // Avoid spawning a worker pool for single-threaded decodes.
        let runner = if threads > 1 {
            Some(ParallelRunner::new(threads)?)
        } else {
            None
        };
        // SAFETY: A null manager selects libjxl's default allocator. The
        // returned pointer is uniquely owned here and destroyed in Drop.
        let raw = NonNull::new(unsafe { api::jxl_decoder_create(std::ptr::null()) })
            .ok_or(Error::Allocation)?;
        let runner_ptr = runner.as_ref().map(|r| r.raw.as_ptr());
        let mut decoder = Self {
            raw,
            _runner: runner,
            _input: PhantomData,
        };
        if let Some(runner_ptr) = runner_ptr {
            // SAFETY: `runner_ptr` comes from
            // `jxl_thread_parallel_runner_create_lazy`, obeys
            // `parallel_runner.h`, and `_runner` outlives `raw` because
            // `Decoder::drop` destroys `raw` before Rust drops `_runner`.
            // `jxl_decoder_set_lazy_thread_parallel_runner` caps the runner
            // from every frame's parsed group count (hidden and visible) at
            // `FrameStage::Toc`, before section decoding begins.
            if unsafe {
                api::jxl_decoder_set_lazy_thread_parallel_runner(decoder.get(), runner_ptr)
            } != Status::SUCCESS
            {
                return Err(Error::Decode);
            }
        }
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

    fn runner_error(&self) -> Option<Error> {
        self._runner.as_ref().and_then(ParallelRunner::last_error)
    }

    #[cfg(test)]
    fn spawned_threads(&self) -> usize {
        self._runner
            .as_ref()
            .map_or(0, ParallelRunner::spawned_threads)
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

/// Reconstruct the complete upright image into a caller-owned F32 buffer on
/// one thread. The pixel limit is checked again before registering that buffer.
pub fn decode_into(
    input: &[u8],
    expected: &ImageInfo,
    output: &mut [f32],
    max_pixels: u64,
) -> Result<(), Error> {
    decode_into_with_threads(input, expected, output, max_pixels, 1)
}

/// Reconstruct the complete upright image into a caller-owned F32 buffer,
/// using up to `threads` worker threads when `threads > 1` (and no parallel
/// runner when `threads <= 1`). Output samples are bit-identical across thread
/// counts.
pub fn decode_into_with_threads(
    input: &[u8],
    expected: &ImageInfo,
    output: &mut [f32],
    max_pixels: u64,
    threads: usize,
) -> Result<(), Error> {
    if output.len() != expected.sample_count()? {
        return Err(Error::BufferSize);
    }
    drive(
        input,
        expected,
        max_pixels,
        threads,
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
            // `drive` runs before returning. Concurrent runner tasks write
            // disjoint group-owned coordinates inside `output`.
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

/// Reconstruct the complete upright image on one thread, retaining only
/// `region` in a caller-owned F32 buffer of exactly `region` × channels
/// samples, row major. See [`decode_region_into_with_threads`].
pub fn decode_region_into(
    input: &[u8],
    expected: &ImageInfo,
    region: Region,
    output: &mut [f32],
    max_pixels: u64,
) -> Result<(), Error> {
    decode_region_into_with_threads(input, expected, region, output, max_pixels, 1)
}

/// Reconstruct the complete upright image using up to `threads` worker threads
/// when `threads > 1` (and no parallel runner when `threads <= 1`), retaining
/// only `region` in a caller-owned F32 buffer of exactly `region` × channels
/// samples, row major. The decoder still reconstructs the full frame (it has
/// no region request); only the retained output shrinks. The samples are
/// bit-identical to the same rectangle of [`decode_into`]'s output across all
/// thread counts.
///
/// # Callback contract
///
/// The rows arrive through `jxl_decoder_set_image_out_callback`, an
/// `unsafe extern "C"` function pointer plus an opaque pointer:
///
/// - **Ownership and lifetime.** The opaque pointer is a shared reference to
///   a stack-owned [`RegionSink`] declared before the decoder is created, and
///   the sink holds the only raw pointer derived from `output`. `drive`
///   destroys the decoder (and joins/destroys any parallel runner) before it
///   returns, so neither pointer is used after the sink and the caller's
///   borrow of `output` end. `output` is not touched through its slice while
///   the decoder exists.
/// - **Arguments.** The port's write stage calls it with upright coordinates
///   `(x, y)`, a pixel count and a pointer to that many initialized pixels in
///   the registered format (one pixel per call for transposing
///   orientations). The sink copies bytes, so it needs no source alignment,
///   and it rejects any run outside the upright image instead of trusting it.
/// - **Panics.** Unwinding out of an `extern "C"` function aborts the
///   process, and the port requires that callbacks never unwind. The body runs
///   under `catch_unwind`; a panic marks the sink failed via `AtomicBool`,
///   later calls are ignored, and the decode returns `Decode` after control
///   returns to Rust.
/// - **Reentrancy and threads.** When `threads <= 1`, no parallel runner is
///   registered and the callback runs synchronously on the calling thread.
///   When `threads > 1`, libjxl's thread parallel runner invokes the callback
///   concurrently from worker threads during `jxl_decoder_process_input` for
///   disjoint group-owned upright pixel rectangles (the same disjointness
///   invariant `WriteToOutputStage` relies on for its direct buffer writes).
///   The callback never calls into the decoder. All geometry fields on
///   [`RegionSink`] are immutable, concurrent byte writes target disjoint
///   ranges of `output`, and mutable progress/failure state lives in
///   [`AtomicU64`] and [`AtomicBool`].
/// - **Completeness.** The decode succeeds only after FULL_IMAGE and SUCCESS
///   and if the number of region pixels written equals the region's area; a
///   truncated or failed decode never returns partly written samples as
///   success.
pub fn decode_region_into_with_threads(
    input: &[u8],
    expected: &ImageInfo,
    region: Region,
    output: &mut [f32],
    max_pixels: u64,
    threads: usize,
) -> Result<(), Error> {
    decode_region_into_inner(
        input,
        expected,
        region,
        output,
        max_pixels,
        threads,
        #[cfg(test)]
        false,
    )
}

fn decode_region_into_inner(
    input: &[u8],
    expected: &ImageInfo,
    region: Region,
    output: &mut [f32],
    max_pixels: u64,
    threads: usize,
    #[cfg(test)] inject_panic: bool,
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
        written: AtomicU64::new(0),
        failed: AtomicBool::new(false),
        #[cfg(test)]
        inject_panic,
        #[cfg(test)]
        caller_thread: std::thread::current().id(),
        #[cfg(test)]
        saw_worker_thread: AtomicBool::new(false),
    };
    let opaque = std::ptr::from_ref(&sink).cast_mut().cast::<c_void>();
    let drive_result = drive(
        input,
        expected,
        max_pixels,
        threads,
        &mut |decoder, format| {
            // SAFETY: See the callback contract above: `opaque` points to
            // `sink`, which outlives the decoder (and its parallel runner)
            // that `drive` destroys before returning; the callback only reads
            // it as `&RegionSink`, never unwinds and never reenters the
            // decoder.
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
            if sink.failed.load(Ordering::Acquire) {
                Err(Error::Decode)
            } else {
                Ok(())
            }
        },
    );
    #[cfg(test)]
    LAST_CALLBACK_ON_WORKER.with(|flag| flag.set(sink.saw_worker_thread.load(Ordering::Acquire)));
    drive_result?;
    let pixels = u64::from(region.width) * u64::from(region.height);
    if sink.failed.load(Ordering::Acquire) || sink.written.load(Ordering::Acquire) != pixels {
        return Err(Error::Decode);
    }
    Ok(())
}

/// The retained region of one decode; see [`decode_region_into_with_threads`].
struct RegionSink {
    /// Derived once from the caller's `&mut [f32]`; writable for
    /// `output_bytes` bytes while the decoder exists. Concurrent runner
    /// callbacks write disjoint upright pixel ranges inside this buffer.
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
    written: AtomicU64,
    failed: AtomicBool,
    #[cfg(test)]
    inject_panic: bool,
    #[cfg(test)]
    caller_thread: std::thread::ThreadId,
    #[cfg(test)]
    saw_worker_thread: AtomicBool,
}

impl RegionSink {
    /// Copy the part of one emitted upright run that lies in the region.
    /// Returns false for a run the decoder cannot legitimately emit.
    fn accept(&self, x: usize, y: usize, count: usize, pixels: *const u8) -> bool {
        #[cfg(test)]
        if std::thread::current().id() != self.caller_thread {
            self.saw_worker_thread.store(true, Ordering::Relaxed);
        }
        #[cfg(test)]
        if self.inject_panic {
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
        // `output_bytes` (checked above) and disjoint from decoder scratch;
        // libjxl's render pipeline dispatches disjoint upright pixel
        // coordinates across runner threads, so concurrent calls never write
        // overlapping byte ranges. A byte copy needs no alignment of either
        // side.
        unsafe {
            std::ptr::copy_nonoverlapping(pixels.add(from), self.output.add(to), len);
        }
        self.written
            .fetch_add((stop - start) as u64, Ordering::Relaxed);
        true
    }
}

/// The image-out callback registered by [`decode_region_into_with_threads`].
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
    // SAFETY: The registration in decode_region_into_inner passes a pointer to
    // its stack sink, which outlives the decoder and its parallel runner; only
    // shared access is made, and every mutable field is atomic.
    let sink = unsafe { &*opaque.cast_const().cast::<RegionSink>() };
    if sink.failed.load(Ordering::Acquire) {
        return;
    }
    // Never unwind across this extern "C" boundary or the port's frames.
    let accepted =
        catch_unwind(AssertUnwindSafe(|| sink.accept(x, y, count, pixels.cast()))).unwrap_or(false);
    if !accepted {
        sink.failed.store(true, Ordering::Release);
    }
}

/// The event loop shared by both outputs: header and color checks against
/// `expected`, one output registration, and FULL_IMAGE then SUCCESS. `poll`
/// runs after every event so a failed callback stops the decode early. The
/// decoder (and any parallel runner) is destroyed before this returns.
fn drive(
    input: &[u8],
    expected: &ImageInfo,
    max_pixels: u64,
    threads: usize,
    register: &mut dyn FnMut(&mut JxlDecoder, &JxlPixelFormat) -> Result<(), Error>,
    poll: &dyn Fn() -> Result<(), Error>,
) -> Result<(), Error> {
    let mut decoder = Decoder::new_with_threads(input, threads)?;
    let mut checked = false;
    let mut color = false;
    let mut full = false;
    let result = loop {
        let status = decoder.next();
        if let Err(err) = poll() {
            break Err(err);
        }
        match status {
            Status::BASIC_INFO => {
                let actual = match decoder.info(max_pixels) {
                    Ok(actual) => actual,
                    Err(err) => break Err(err),
                };
                if actual.width != expected.width
                    || actual.height != expected.height
                    || actual.channels != expected.channels
                    || actual.alpha_premultiplied != expected.alpha_premultiplied
                {
                    break Err(Error::Decode);
                }
                checked = true;
            }
            Status::COLOR_ENCODING => {
                let icc = match decoder.data_icc() {
                    Ok(icc) => icc,
                    Err(err) => break Err(err),
                };
                if !checked || icc != expected.rendered_icc {
                    break Err(Error::Decode);
                }
                color = true;
            }
            Status::NEED_IMAGE_OUT_BUFFER => {
                if !color || full {
                    break Err(Error::Decode);
                }
                if let Err(err) = register(decoder.get(), &expected.format()) {
                    break Err(err);
                }
            }
            Status::FULL_IMAGE => full = true,
            Status::SUCCESS if full => break Ok(()),
            Status::ERROR => {
                if let Some(err) = decoder.runner_error() {
                    break Err(err);
                }
                break Err(Error::Decode);
            }
            Status::NEED_MORE_INPUT | Status::SUCCESS => break Err(Error::Decode),
            _ => break Err(Error::Unsupported),
        }
    };
    #[cfg(test)]
    LAST_SPAWNED_THREADS.with(|count| count.set(decoder.spawned_threads()));
    result
}

#[cfg(test)]
mod tests {
    use super::{
        Error, FAIL_RUNNER_CREATE, ImageInfo, LAST_CALLBACK_ON_WORKER, LAST_SPAWNED_THREADS,
        Region, RunnerError, Transfer, decode_into, decode_into_with_threads, decode_region_into,
        decode_region_into_inner, decode_region_into_with_threads, inspect,
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
        let multigroup = std::fs::read(format!(
            "{ROOT}tests/fixtures/jxl-deep/d10-2049x40-a0-d1.jxl"
        ))
        .unwrap();
        for (name, bytes) in [
            ("sample", SAMPLE),
            ("multigroup-vardct", multigroup.as_slice()),
        ] {
            let info = inspect(bytes, 1 << 24).unwrap();
            let crop = rect(1, 1, info.width - 2, info.height - 2);
            let mut output = vec![0f32; info.region_sample_count(crop).unwrap()];
            for threads in [1, 4] {
                let result = decode_region_into_inner(
                    bytes,
                    &info,
                    crop,
                    &mut output,
                    1 << 24,
                    threads,
                    true,
                );
                assert_eq!(result, Err(Error::Decode), "{name} threads={threads}");
                // The decoder (and runner) were destroyed normally; the next
                // decode succeeds cleanly.
                decode_region_into_with_threads(bytes, &info, crop, &mut output, 1 << 24, threads)
                    .unwrap();
                assert!(
                    output.iter().all(|sample| sample.is_finite()),
                    "{name} threads={threads}"
                );
            }
        }
    }

    #[test]
    fn parallel_decode_matches_single_thread_bit_for_bit() {
        let mut fixtures = callback_fixtures();
        for name in [
            "tests/fixtures/jxl-deep/d10-2049x40-a0-d1.jxl",
            "tests/fixtures/jxl-deep/d16-2049x40-a1-lossless.jxl",
            "tests/fixtures/jxl-lossless/rgba-2049x40.jxl",
            "tests/fixtures/jxl-lossless/rgb-2049x40.jxl",
            "tests/fixtures/jxl-lossless/rgb-300x260.jxl",
            "codecs/libjxl-rs/tests/data/jpeg-reconstruction/large-progressive.jxl",
            "codecs/libjxl-rs/tests/data/jpeg-reconstruction/chunked-baseline.jxl",
        ] {
            let bytes = std::fs::read(format!("{ROOT}{name}")).unwrap();
            fixtures.push((name.to_owned(), bytes));
        }
        let bits = |samples: &[f32]| samples.iter().map(|s| s.to_bits()).collect::<Vec<_>>();
        for (name, bytes) in fixtures {
            let info = inspect(&bytes, 1 << 24).unwrap();
            let mut serial_full = vec![0f32; info.sample_count().unwrap()];
            decode_into_with_threads(&bytes, &info, &mut serial_full, 1 << 24, 1).unwrap();
            let expected_full = bits(&serial_full);
            let crop = rect(
                info.width / 5,
                info.height / 5,
                (info.width / 2).max(1),
                (info.height / 2).max(1),
            );
            let mut serial_crop = vec![f32::NAN; info.region_sample_count(crop).unwrap()];
            decode_region_into_with_threads(&bytes, &info, crop, &mut serial_crop, 1 << 24, 1)
                .unwrap();
            let expected_crop = bits(&serial_crop);

            for threads in [0, 2, 4, 8] {
                let mut got_full = vec![f32::NAN; info.sample_count().unwrap()];
                decode_into_with_threads(&bytes, &info, &mut got_full, 1 << 24, threads).unwrap();
                assert_eq!(
                    bits(&got_full),
                    expected_full,
                    "{name}: decode_into threads={threads}"
                );
                let mut got_crop = vec![f32::NAN; info.region_sample_count(crop).unwrap()];
                decode_region_into_with_threads(
                    &bytes,
                    &info,
                    crop,
                    &mut got_crop,
                    1 << 24,
                    threads,
                )
                .unwrap();
                assert_eq!(
                    bits(&got_crop),
                    expected_crop,
                    "{name}: decode_region_into threads={threads}"
                );
            }
        }
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

    /// Encode a synthetic 8-bit sRGB JXL codestream in memory with the local
    /// libjxl-rs encoder so tests can exercise explicit orientations and
    /// modular group sizes without committing large binary fixtures.
    fn encode_test_jxl(
        width: u32,
        height: u32,
        orientation: jxl::jxl::codestream_header::JxlOrientation,
        lossless: bool,
        modular_group_size: Option<i64>,
    ) -> Vec<u8> {
        use jxl::jxl::codestream_header::JxlBasicInfo;
        use jxl::jxl::color_encoding::JxlColorEncoding;
        use jxl::jxl::encode::{
            self as enc_api, JxlEncoderFrameSettingId as Id, JxlEncoderStatus as EncStatus,
        };
        use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};

        let mut pixels = vec![0u8; width as usize * height as usize * 3];
        for y in 0..height as usize {
            for x in 0..width as usize {
                let idx = (y * width as usize + x) * 3;
                pixels[idx] = ((x * 7 + y * 3) & 0xff) as u8;
                pixels[idx + 1] = (((x * 5) ^ (y * 11)) & 0xff) as u8;
                pixels[idx + 2] = ((x.wrapping_mul(13).wrapping_add(y * 17)) & 0xff) as u8;
            }
        }
        // SAFETY: Null memory manager selects the default allocator; destroyed
        // before returning.
        let enc_ptr = unsafe { enc_api::jxl_encoder_create(std::ptr::null()) };
        assert!(!enc_ptr.is_null());
        struct EncGuard(*mut enc_api::JxlEncoder);
        impl Drop for EncGuard {
            fn drop(&mut self) {
                // SAFETY: Uniquely owned encoder from `jxl_encoder_create`.
                unsafe { enc_api::jxl_encoder_destroy(self.0) };
            }
        }
        let guard = EncGuard(enc_ptr);
        // SAFETY: Uniquely owned live encoder.
        let enc = unsafe { &mut *guard.0 };
        let mut basic = JxlBasicInfo::zeroed();
        enc_api::jxl_encoder_init_basic_info(&mut basic);
        basic.xsize = width;
        basic.ysize = height;
        basic.bits_per_sample = 8;
        basic.num_color_channels = 3;
        basic.uses_original_profile = i32::from(lossless);
        basic.orientation = orientation;
        assert_eq!(
            enc_api::jxl_encoder_set_basic_info(enc, &basic),
            EncStatus::SUCCESS
        );
        let mut color = JxlColorEncoding::default();
        enc_api::jxl_color_encoding_set_to_srgb(&mut color, 0);
        assert_eq!(
            enc_api::jxl_encoder_set_color_encoding(enc, &color),
            EncStatus::SUCCESS
        );
        // SAFETY: `enc` is a live encoder and a null source selects defaults.
        let settings = unsafe { enc_api::jxl_encoder_frame_settings_create(enc, std::ptr::null()) };
        assert!(!settings.is_null());
        // SAFETY: `settings` belongs to live `enc` and is not aliased.
        unsafe {
            if lossless {
                assert_eq!(
                    enc_api::jxl_encoder_frame_settings_set_option(settings, Id::EFFORT, 2),
                    EncStatus::SUCCESS
                );
                assert_eq!(
                    enc_api::jxl_encoder_set_frame_lossless(settings, 1),
                    EncStatus::SUCCESS
                );
            } else {
                assert_eq!(
                    enc_api::jxl_encoder_frame_settings_set_option(settings, Id::EFFORT, 7),
                    EncStatus::SUCCESS
                );
                assert_eq!(
                    enc_api::jxl_encoder_frame_settings_set_option(settings, Id::PATCHES, 0),
                    EncStatus::SUCCESS
                );
                assert_eq!(
                    enc_api::jxl_encoder_set_frame_distance(settings, 1.0),
                    EncStatus::SUCCESS
                );
            }
            if let Some(group_size) = modular_group_size {
                assert_eq!(
                    enc_api::jxl_encoder_frame_settings_set_option(
                        settings,
                        Id::MODULAR_GROUP_SIZE,
                        group_size,
                    ),
                    EncStatus::SUCCESS
                );
            }
            let format = JxlPixelFormat {
                num_channels: 3,
                data_type: JxlDataType::UINT8,
                endianness: JxlEndianness::NATIVE,
                align: 0,
            };
            assert_eq!(
                enc_api::jxl_encoder_add_image_frame(settings, &format, Some(&pixels)),
                EncStatus::SUCCESS
            );
        }
        enc_api::jxl_encoder_close_input(enc);
        let mut out = Vec::new();
        loop {
            out.reserve(64 * 1024);
            let start = out.len();
            let mut avail = out.spare_capacity_mut().len();
            let mut next = out.spare_capacity_mut().as_mut_ptr().cast::<u8>();
            // SAFETY: `next` points to `avail` writable bytes of spare capacity.
            let status =
                unsafe { enc_api::jxl_encoder_process_output(enc, &raw mut next, &raw mut avail) };
            let produced = out.spare_capacity_mut().len() - avail;
            // SAFETY: `jxl_encoder_process_output` initialized `produced` bytes.
            unsafe { out.set_len(start + produced) };
            match status {
                EncStatus::SUCCESS => return out,
                EncStatus::NEED_MORE_OUTPUT => {}
                other => panic!("unexpected encoder status {other:?}"),
            }
        }
    }

    #[test]
    fn lazy_runner_sizes_workers_from_actual_frame_work() {
        use jxl::jxl::codestream_header::JxlOrientation;

        let bits = |samples: &[f32]| samples.iter().map(|s| s.to_bits()).collect::<Vec<_>>();

        // 1. Single-group 7x5 fixture: requesting 20 threads spawns 0 workers.
        let small = inspect(SAMPLE, 35).unwrap();
        let mut small_out = vec![0f32; small.sample_count().unwrap()];
        decode_into_with_threads(SAMPLE, &small, &mut small_out, 35, 20).unwrap();
        assert_eq!(LAST_SPAWNED_THREADS.with(std::cell::Cell::get), 0);

        // 2. Single-group 128x128 VarDCT frame: even though AdaptiveDCSmoothing
        //    has 14 serialized DC rows, the frame has 1 group so 0 workers spawn.
        let small_vardct = encode_test_jxl(128, 128, JxlOrientation::IDENTITY, false, None);
        let info = inspect(&small_vardct, 1 << 24).unwrap();
        let mut serial = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&small_vardct, &info, &mut serial, 1 << 24, 1).unwrap();
        let mut parallel = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&small_vardct, &info, &mut parallel, 1 << 24, 20).unwrap();
        assert_eq!(
            LAST_SPAWNED_THREADS.with(std::cell::Cell::get),
            0,
            "single-group 128x128 VarDCT must spawn zero workers"
        );
        assert_eq!(bits(&parallel), bits(&serial));

        // 3. 300x300 lossless modular frame: libjxl selects group_size_shift=2
        //    (512x512 group -> 1 actual group despite exceeding 256x256), so
        //    requesting 20 threads spawns 0 workers instead of 4.
        let big_single_group = encode_test_jxl(300, 300, JxlOrientation::IDENTITY, true, None);
        let info = inspect(&big_single_group, 1 << 24).unwrap();
        let mut serial = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&big_single_group, &info, &mut serial, 1 << 24, 1).unwrap();
        let mut parallel = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&big_single_group, &info, &mut parallel, 1 << 24, 20).unwrap();
        assert_eq!(
            LAST_SPAWNED_THREADS.with(std::cell::Cell::get),
            0,
            "single 512x512 modular group must spawn zero worker threads"
        );
        assert_eq!(bits(&parallel), bits(&serial));

        // 4. 401x256 lossless modular frame: libjxl selects group_size_shift=1
        //    (256x256 groups -> 2 actual groups, 102,656 px >= 65,536), so
        //    requesting 20 threads spawns 2 workers rather than 20.
        let two_groups = encode_test_jxl(401, 256, JxlOrientation::IDENTITY, true, None);
        let info = inspect(&two_groups, 1 << 24).unwrap();
        let mut serial = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&two_groups, &info, &mut serial, 1 << 24, 1).unwrap();
        let mut parallel = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&two_groups, &info, &mut parallel, 1 << 24, 20).unwrap();
        assert_eq!(
            LAST_SPAWNED_THREADS.with(std::cell::Cell::get),
            2,
            "two-group modular frame must cap lazy workers at 2"
        );
        assert_eq!(bits(&parallel), bits(&serial));

        // 5. Small multi-group 320x64 VarDCT frame: crosses the 256 px group
        //    boundary (2 groups) but has only 20,480 px (< 65,536), so
        //    requesting 20 threads stays on the caller with 0 workers spawned.
        let small_multigroup = encode_test_jxl(320, 64, JxlOrientation::IDENTITY, false, None);
        let info = inspect(&small_multigroup, 1 << 24).unwrap();
        let mut serial = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&small_multigroup, &info, &mut serial, 1 << 24, 1).unwrap();
        let mut parallel = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&small_multigroup, &info, &mut parallel, 1 << 24, 20).unwrap();
        assert_eq!(
            LAST_SPAWNED_THREADS.with(std::cell::Cell::get),
            0,
            "small 320x64 multigroup VarDCT must spawn zero workers"
        );
        assert_eq!(bits(&parallel), bits(&serial));
    }

    #[test]
    fn oriented_multigroup_region_callback_runs_on_workers_bit_identically() {
        use jxl::jxl::codestream_header::JxlOrientation;

        let bits = |samples: &[f32]| samples.iter().map(|s| s.to_bits()).collect::<Vec<_>>();
        // Transposing orientations 6 (90 CW) and 8 (90 CCW) emit 1 pixel per
        // `region_callback` invocation; 300x300 has 4 groups so worker threads
        // invoke the callback concurrently across a group boundary.
        for orientation in [JxlOrientation::ROTATE_90_CW, JxlOrientation::ROTATE_90_CCW] {
            let bytes = encode_test_jxl(300, 300, orientation, false, None);
            let info = inspect(&bytes, 1 << 24).unwrap();
            assert_eq!((info.width, info.height), (300, 300));
            let crop = rect(180, 180, 110, 110);
            let mut serial = vec![f32::NAN; info.region_sample_count(crop).unwrap()];
            decode_region_into_with_threads(&bytes, &info, crop, &mut serial, 1 << 24, 1).unwrap();
            assert!(!LAST_CALLBACK_ON_WORKER.with(std::cell::Cell::get));
            let expected = bits(&serial);

            for threads in [4, 8] {
                let mut got = vec![f32::NAN; info.region_sample_count(crop).unwrap()];
                decode_region_into_with_threads(&bytes, &info, crop, &mut got, 1 << 24, threads)
                    .unwrap();
                assert_eq!(
                    bits(&got),
                    expected,
                    "orientation={orientation:?} threads={threads}"
                );
                assert_eq!(
                    LAST_SPAWNED_THREADS.with(std::cell::Cell::get),
                    4,
                    "300x300 4-group VarDCT should cap lazy workers at 4"
                );
                assert!(
                    LAST_CALLBACK_ON_WORKER.with(std::cell::Cell::get),
                    "oriented multi-group region_callback must execute on worker threads"
                );
            }
        }
    }

    #[test]
    fn runner_errors_distinguish_allocation_from_thread_spawn() {
        let multigroup = std::fs::read(format!(
            "{ROOT}tests/fixtures/jxl-deep/d10-2049x40-a0-d1.jxl"
        ))
        .unwrap();
        let info = inspect(&multigroup, 1 << 24).unwrap();
        struct Guard;
        impl Drop for Guard {
            fn drop(&mut self) {
                FAIL_RUNNER_CREATE.with(|flag| flag.set(None));
            }
        }
        let _guard = Guard;

        for (injected, expected) in [
            (RunnerError::Allocation, Error::Allocation),
            (RunnerError::ThreadSpawn, Error::ThreadSpawn),
            (RunnerError::InvalidMemoryManager, Error::Decode),
        ] {
            FAIL_RUNNER_CREATE.with(|flag| flag.set(Some(injected)));
            let mut output = vec![f32::NAN; info.sample_count().unwrap()];
            assert_eq!(
                decode_into_with_threads(&multigroup, &info, &mut output, 1 << 24, 4),
                Err(expected),
                "injected={injected:?}"
            );
            assert!(output.iter().all(|s| s.is_nan()));
        }

        // Single-threaded decode never creates a runner, so it still succeeds.
        FAIL_RUNNER_CREATE.with(|flag| flag.set(Some(RunnerError::ThreadSpawn)));
        let small = inspect(SAMPLE, 35).unwrap();
        let mut small_out = vec![0f32; small.sample_count().unwrap()];
        decode_into_with_threads(SAMPLE, &small, &mut small_out, 35, 1).unwrap();
        assert!(small_out.iter().all(|s| s.is_finite()));
    }

    /// Splice a valid two-frame still JXL codestream whose first frame is a
    /// hidden reference frame (`is_last = false`, `save_as_reference = 1`)
    /// built from `hidden_codestream` and whose second frame is the visible
    /// frame of `visible_codestream`.
    fn splice_hidden_and_visible_frames(
        hidden_codestream: &[u8],
        visible_codestream: &[u8],
    ) -> Vec<u8> {
        use jxl::jxl::dec_bit_reader::BitReader;
        use jxl::jxl::enc_bit_writer::BitWriter;
        use jxl::jxl::enc_fields::write_frame_header;
        use jxl::jxl::enc_toc::{write_toc_permutation, write_toc_sizes};
        use jxl::jxl::frame_header::{FrameHeader, FrameSize, read_frame_header};
        use jxl::jxl::headers::read_size_header;
        use jxl::jxl::image_metadata::{CodecMetadata, read_image_metadata};
        use jxl::jxl::memory_manager::JxlMemoryManager;
        use jxl::jxl::memory_manager_internal::MemoryManagerPtr;
        use jxl::jxl::toc::{num_toc_entries, read_toc};

        fn split_codestream(bytes: &[u8]) -> (CodecMetadata, usize) {
            assert_eq!(&bytes[..2], &[0xff, 0x0a]);
            let mut reader = BitReader::new(&bytes[2..]);
            let mut meta = CodecMetadata::default();
            read_size_header(&mut reader, &mut meta.size).unwrap();
            read_image_metadata(&mut reader, &mut meta.m).unwrap();
            meta.transform_data.nonserialized_xyb_encoded = meta.m.xyb_encoded;
            jxl::jxl::fields::bundle::read(&mut reader, &mut meta.transform_data).unwrap();
            reader.jump_to_byte_boundary().unwrap();
            let consumed = 2 + reader.total_bits_consumed() / 8;
            reader.close().unwrap();
            (meta, consumed)
        }

        let (hidden_meta, hidden_frame_offset) = split_codestream(hidden_codestream);
        let (visible_meta, visible_frame_offset) = split_codestream(visible_codestream);

        let hidden_frame_bytes = &hidden_codestream[hidden_frame_offset..];
        let mut reader = BitReader::new(hidden_frame_bytes);
        let mut fh = FrameHeader::new(Some(&hidden_meta));
        read_frame_header(&mut reader, &mut fh).unwrap();
        let dim = fh.to_frame_dimensions();
        let entries = num_toc_entries(
            dim.num_groups,
            dim.num_dc_groups,
            fh.passes.num_passes as usize,
        );
        let mut sizes = Vec::new();
        let mut perm = Vec::new();
        let mm = JxlMemoryManager::default();
        // SAFETY: `mm` is a live `JxlMemoryManager`; our single-group test
        // frame has no TOC permutation, so `read_toc` performs no allocation.
        let mm_ptr = unsafe { MemoryManagerPtr::from_raw(&raw const mm) };
        read_toc(mm_ptr, entries, &mut reader, &mut sizes, &mut perm).unwrap();
        assert!(perm.is_empty());
        let hidden_sections_offset = reader.total_bits_consumed() / 8;
        reader.close().unwrap();
        let hidden_sections = &hidden_frame_bytes[hidden_sections_offset..];

        // Rebuild Frame 0 against `visible_meta` as a hidden reference frame.
        let mut hidden_fh = FrameHeader::new(Some(&visible_meta));
        hidden_fh.encoding = fh.encoding;
        hidden_fh.flags = fh.flags;
        hidden_fh.color_transform = fh.color_transform;
        hidden_fh.loop_filter = fh.loop_filter.clone();
        hidden_fh.passes = fh.passes.clone();
        hidden_fh.is_last = false;
        hidden_fh.save_as_reference = 1;
        if hidden_meta.xsize() != visible_meta.xsize()
            || hidden_meta.ysize() != visible_meta.ysize()
        {
            hidden_fh.custom_size_or_origin = true;
            hidden_fh.frame_size = FrameSize {
                xsize: hidden_meta.xsize() as u32,
                ysize: hidden_meta.ysize() as u32,
            };
        }
        let mut writer = BitWriter::new();
        write_frame_header(&mut hidden_fh, &mut writer, None).unwrap();
        write_toc_permutation(&[], &mut writer, None).unwrap();
        let toc_sizes: Vec<usize> = sizes.iter().map(|&s| s as usize).collect();
        write_toc_sizes(&toc_sizes, &mut writer, None).unwrap();
        writer.zero_pad_to_byte();

        let mut combined = Vec::new();
        combined.extend_from_slice(&visible_codestream[..visible_frame_offset]);
        combined.extend_from_slice(writer.get_span());
        combined.extend_from_slice(hidden_sections);
        combined.extend_from_slice(&visible_codestream[visible_frame_offset..]);
        combined
    }

    #[test]
    fn hidden_reference_frame_respects_per_frame_group_worker_cap() {
        use jxl::jxl::codestream_header::JxlOrientation;

        let bits = |samples: &[f32]| samples.iter().map(|s| s.to_bits()).collect::<Vec<_>>();
        let single_128 = encode_test_jxl(128, 128, JxlOrientation::IDENTITY, false, None);

        // 1. Two single-group 128x128 VarDCT frames (hidden reference frame +
        //    visible frame): neither frame may spawn workers even though
        //    AdaptiveDCSmoothing has 14 rows on each frame.
        let hidden_then_single = splice_hidden_and_visible_frames(&single_128, &single_128);
        let info = inspect(&hidden_then_single, 1 << 24).unwrap();
        assert_eq!((info.width, info.height), (128, 128));
        let mut serial = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&hidden_then_single, &info, &mut serial, 1 << 24, 1).unwrap();
        let mut parallel = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&hidden_then_single, &info, &mut parallel, 1 << 24, 8).unwrap();
        assert_eq!(
            LAST_SPAWNED_THREADS.with(std::cell::Cell::get),
            0,
            "hidden single-group VarDCT reference frame must not spawn workers"
        );
        assert_eq!(bits(&parallel), bits(&serial));

        // 2. Hidden single-group 128x128 reference frame followed by visible
        //    4-group 300x300 VarDCT frame: frame 0 caps at 0, then frame 1
        //    raises the cap back to 4 (<= construction max 8) and matches
        //    serial bit-for-bit.
        let visible_300 = encode_test_jxl(300, 300, JxlOrientation::IDENTITY, false, None);
        let hidden_then_multi = splice_hidden_and_visible_frames(&single_128, &visible_300);
        let info = inspect(&hidden_then_multi, 1 << 24).unwrap();
        assert_eq!((info.width, info.height), (300, 300));
        let mut serial = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&hidden_then_multi, &info, &mut serial, 1 << 24, 1).unwrap();
        let mut parallel = vec![0f32; info.sample_count().unwrap()];
        decode_into_with_threads(&hidden_then_multi, &info, &mut parallel, 1 << 24, 8).unwrap();
        assert_eq!(
            LAST_SPAWNED_THREADS.with(std::cell::Cell::get),
            4,
            "visible 4-group frame after hidden 1-group frame must spawn 4 workers"
        );
        assert_eq!(bits(&parallel), bits(&serial));
    }
}
