//! Safe, narrow still-image encoder boundary over the workspace libavif port.
//! Input rows are straight-alpha sRGB RGBA, 8-bit or (since 2026-11-15) `u16`
//! at the image depth; output uses CICP 1/13/6 and full range.
//!
//! Internal to `ferrisma`: this crate follows no SemVer of its own,
//! `ferrisma` requires exactly the version released with it (an `=`
//! requirement), and any release may change this API. It is not for
//! direct use; enable the matching `ferrisma` feature instead (ADR-0040,
//! problem 7).
#![deny(unsafe_op_in_unsafe_fn, clippy::undocumented_unsafe_blocks)]

use avif::{avif as image_api, rawdata, reformat, write};
use std::ffi::CStr;

/// Output chroma layout.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Chroma {
    Yuv444,
    Yuv420,
}

/// Configuration for one still image. `has_alpha` false ignores the row's
/// alpha samples; true treats them as straight alpha. `threads` is clamped
/// to 1–256. Quality, alpha quality and speed use libavif's integer scales and
/// are passed unchanged (`avifEncoder::quality`, `qualityAlpha`, `speed`).
/// `depth` is the AV1 depth, 8, 10 or 12 (12 since 2026-11-15).
/// `sample_depth` is the depth of the RGB rows libavif converts
/// (`avifRGBImage::depth`): 8 (`write_row`, `u8` samples, which libavif
/// scales to `depth`) or `depth` itself (`write_row_u16`, samples
/// `0..=2^depth - 1`), the engine's deep rows (ADR-0021, amendment of
/// 2026-11-15).
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub has_alpha: bool,
    pub quality: u8,
    pub alpha_quality: u8,
    pub speed: u8,
    pub threads: usize,
    pub chroma: Chroma,
    pub depth: u8,
    pub sample_depth: u8,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorKind {
    InvalidOptions,
    ResourceExhausted,
    Failed,
}

#[derive(Debug)]
pub struct EncodeError {
    kind: ErrorKind,
    message: String,
}
impl EncodeError {
    fn new(kind: ErrorKind, message: impl Into<String>) -> Self {
        Self {
            kind,
            message: message.into(),
        }
    }
    pub fn into_parts(self) -> (ErrorKind, String) {
        (self.kind, self.message)
    }
}
impl std::fmt::Display for EncodeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.message)
    }
}
impl std::error::Error for EncodeError {}

// Values from libavif's `avif.h`; the generated bindings keep C enums as
// plain integers.
const AVIF_RESULT_OK: u32 = 0;
const AVIF_RESULT_INVALID_ARGUMENT: u32 = 24;
const AVIF_RESULT_OUT_OF_MEMORY: u32 = 26;
const AVIF_PIXEL_FORMAT_YUV444: u32 = 1;
const AVIF_PIXEL_FORMAT_YUV420: u32 = 3;
const AVIF_RGB_FORMAT_RGB: u32 = 0;
const AVIF_RGB_FORMAT_RGBA: u32 = 1;
const AVIF_RANGE_FULL: u32 = 1;
const AVIF_COLOR_PRIMARIES_BT709: u16 = 1;
const AVIF_TRANSFER_CHARACTERISTICS_SRGB: u16 = 13;
const AVIF_MATRIX_COEFFICIENTS_BT601: u16 = 6;
/// libavif's quality range ends at `AVIF_QUALITY_BEST` (lossless quantizer).
const MAX_QUALITY: u8 = 100;
/// libavif's speed range, `AVIF_SPEED_SLOWEST` to `AVIF_SPEED_FASTEST`.
const MAX_SPEED: u8 = 10;
/// Ferrisma caps encoder threads at 256 as well.
const MAX_THREADS: usize = 256;
/// At most this many row bands convert RGB to YUV at once.
const MAX_BANDS: usize = 8;
/// Bands shorter than this cost more in thread start-up than they save.
const MIN_BAND_ROWS: u32 = 64;
/// Bands smaller than this pixel area cost more in thread start-up than they save.
const MIN_PIXELS_PER_BAND: u64 = 65_536;

/// Validate the request and return the simultaneous RGB(A) conversion strip
/// and YUV(A) plane sample bytes for [`Frame::new`].
/// This excludes allocator overhead, codec scratch and compressed output.
///
/// # Errors
/// Returns `InvalidOptions` for unsupported settings or size overflow.
pub fn required_frame_bytes(request: &Config) -> Result<usize, EncodeError> {
    required_frame_bytes_with_mode(request, false)
}

/// Validate the request and return the simultaneous RGB(A) conversion strip
/// and YUV(A) plane sample bytes for [`Frame::new_serial_conversion`], which
/// keeps row-intake conversion single-threaded so callers with an active
/// transform or orientation worker pool do not spawn a second concurrent pool
/// during `write_row` / `write_row_u16`.
///
/// # Errors
/// Returns `InvalidOptions` for unsupported settings or size overflow.
pub fn required_frame_bytes_serial_conversion(request: &Config) -> Result<usize, EncodeError> {
    required_frame_bytes_with_mode(request, true)
}

fn required_frame_bytes_with_mode(
    request: &Config,
    serial_conversion: bool,
) -> Result<usize, EncodeError> {
    let invalid = |message: &str| EncodeError::new(ErrorKind::InvalidOptions, message);
    if !matches!(request.depth, 8 | 10 | 12) {
        return Err(invalid("libavif-rs backend depth must be 8, 10 or 12"));
    }
    if request.sample_depth != 8 && request.sample_depth != request.depth {
        return Err(invalid(
            "libavif-rs backend rows must be 8-bit or at the image depth",
        ));
    }
    if request.quality > MAX_QUALITY {
        return Err(invalid("libavif quality must be between 0 and 100"));
    }
    if request.alpha_quality > MAX_QUALITY {
        return Err(invalid("libavif alpha quality must be between 0 and 100"));
    }
    if request.speed > MAX_SPEED {
        return Err(invalid("libavif speed must be between 0 and 10"));
    }
    if request.width == 0 || request.height == 0 {
        return Err(invalid("output dimensions must be nonzero"));
    }
    request
        .width
        .checked_mul(4)
        .ok_or_else(|| invalid("output row size overflows"))?;
    rgb_row_bytes(request).ok_or_else(|| invalid("output row size overflows"))?;
    let bytes = frame_bytes(request, serial_conversion)
        .ok_or_else(|| invalid("output frame size overflows"))?;
    let rgba_stream_samples = usize::try_from(request.width)
        .ok()
        .and_then(|w| w.checked_mul(usize::try_from(request.height).ok()?))
        .and_then(|px| px.checked_mul(4));
    if isize::try_from(bytes).is_err()
        || rgba_stream_samples.is_none_or(|n| isize::try_from(n).is_err())
    {
        return Err(invalid("output frame size overflows"));
    }
    Ok(bytes)
}

/// Native-depth RGB (opaque) or RGBA (alpha) sample storage. Storing
/// `sample_depth > 8` in `Vec<u16>` guarantees `align_of::<u16>()` alignment
/// for `avifImageRGBToYUV`'s `*const u16` reads (both 3-channel RGB at 6 bytes
/// per pixel and 4-channel RGBA at 8 bytes per pixel stay 2-byte aligned) and
/// avoids per-sample byte serialization.
enum FrameSamples {
    U8(Vec<u8>),
    U16(Vec<u16>),
}

impl FrameSamples {
    fn clear(&mut self) {
        match self {
            Self::U8(samples) => samples.clear(),
            Self::U16(samples) => samples.clear(),
        }
    }

    fn as_band_mut(&mut self) -> BandSamples<'_> {
        match self {
            Self::U8(samples) => BandSamples::U8(samples),
            Self::U16(samples) => BandSamples::U16(samples),
        }
    }
}

/// Mutable view over one or more RGB(A) rows in native sample storage.
enum BandSamples<'a> {
    U8(&'a mut [u8]),
    U16(&'a mut [u16]),
}

impl BandSamples<'_> {
    fn split_at_mut(self, mid: usize) -> (Self, Self) {
        match self {
            Self::U8(slice) => {
                let (head, tail) = slice.split_at_mut(mid);
                (Self::U8(head), Self::U8(tail))
            }
            Self::U16(slice) => {
                let (head, tail) = slice.split_at_mut(mid);
                (Self::U16(head), Self::U16(tail))
            }
        }
    }
}

/// An owned frame accepting each output row exactly once from top to bottom.
/// Rows stream into a bounded RGB (opaque) or RGBA (alpha) strip and convert
/// directly into pre-allocated `avifImage` YUV(A) planes via
/// `avifImageRGBToYUV` as each strip fills, avoiding a full RGB(A) frame
/// allocation while preserving bit-identical samples.
pub struct Frame {
    request: Config,
    /// Pre-allocated `avifImage` receiving converted YUV (and optional alpha)
    /// planes strip by strip.
    image: Image,
    /// Rolling strip of pending RGB (opaque) or RGBA (alpha) samples: `u8` at
    /// `sample_depth` 8, native `u16` above so the buffer satisfies
    /// `align_of::<u16>()` for `avifImageRGBToYUV`.
    strip: FrameSamples,
    /// Maximum rows buffered before flushing to `image` (always even unless
    /// `request.height == 1`).
    strip_rows: u32,
    /// First image row currently buffered in `strip` (always even, as 4:2:0
    /// `avifImageSetViewRect` requires).
    strip_y: u32,
    /// Number of rows currently buffered in `strip`.
    buffered: u32,
    /// Next expected row index `y`.
    next: u32,
    /// Set permanently after a failed strip flush so callers cannot retry rows
    /// or call `finish()` on partially uninitialized YUV(A) planes.
    poisoned: bool,
    /// When true, strip RGB-to-YUV conversion stays on the calling thread
    /// during `write_row` / `write_row_u16` while `finish` still uses
    /// `request.threads` for `avifEncoderWrite`.
    serial_conversion: bool,
}

impl Frame {
    /// Allocate the YUV(A) planes and bounded RGB(A) conversion strip after
    /// validating the request. Callers that budget pixels should reserve
    /// `required_frame_bytes` before this call.
    ///
    /// # Errors
    /// Returns an error for invalid options or failed allocation.
    pub fn new(request: Config) -> Result<Self, EncodeError> {
        Self::new_with_mode(request, false)
    }

    /// Like [`Frame::new`], but keeps row-intake RGB-to-YUV strip conversion
    /// single-threaded on the calling thread (spawning zero worker threads
    /// during `write_row` / `write_row_u16`), while [`Frame::finish`] still
    /// runs `avifEncoderWrite` with `request.threads`. Callers that budget
    /// pixels should reserve [`required_frame_bytes_serial_conversion`] before
    /// this call.
    ///
    /// # Errors
    /// Returns an error for invalid options or failed allocation.
    pub fn new_serial_conversion(request: Config) -> Result<Self, EncodeError> {
        Self::new_with_mode(request, true)
    }

    fn new_with_mode(request: Config, serial_conversion: bool) -> Result<Self, EncodeError> {
        required_frame_bytes_with_mode(&request, serial_conversion)?;
        let format = match request.chroma {
            Chroma::Yuv444 => AVIF_PIXEL_FORMAT_YUV444,
            Chroma::Yuv420 => AVIF_PIXEL_FORMAT_YUV420,
        };
        let image = new_image(&request, format)?;
        allocate_planes(&image, request.has_alpha)?;
        let strip_rows = strip_rows(&request, serial_conversion);
        let samples = request.width as usize * strip_rows as usize * rgb_channels(&request);
        let strip = if request.sample_depth > 8 {
            let mut buf = Vec::new();
            buf.try_reserve_exact(samples).map_err(|_| {
                EncodeError::new(
                    ErrorKind::ResourceExhausted,
                    "cannot allocate the RGB conversion strip for libavif",
                )
            })?;
            FrameSamples::U16(buf)
        } else {
            let mut buf = Vec::new();
            buf.try_reserve_exact(samples).map_err(|_| {
                EncodeError::new(
                    ErrorKind::ResourceExhausted,
                    "cannot allocate the RGB conversion strip for libavif",
                )
            })?;
            FrameSamples::U8(buf)
        };
        Ok(Self {
            request,
            image,
            strip,
            strip_rows,
            strip_y: 0,
            buffered: 0,
            next: 0,
            poisoned: false,
            serial_conversion,
        })
    }

    fn check_not_poisoned(&self) -> Result<(), EncodeError> {
        if self.poisoned {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs frame failed during a previous strip flush",
            ));
        }
        Ok(())
    }

    /// Append exactly one full RGBA8 row in increasing y order.
    ///
    /// # Errors
    /// Returns an error for a poisoned frame, an out-of-order row, an
    /// incorrect row length, or a failed strip conversion.
    pub fn write_row(&mut self, y: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.check_not_poisoned()?;
        self.check_row(y, rgba.len(), 8)?;
        let FrameSamples::U8(buf) = &mut self.strip else {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend received rows out of order, of the wrong length or depth",
            ));
        };
        if self.request.has_alpha {
            buf.extend_from_slice(rgba);
        } else {
            for pixel in rgba.as_chunks::<4>().0 {
                buf.extend_from_slice(&pixel[..3]);
            }
        }
        self.finish_row(y)
    }

    /// Append exactly one full RGBA row of `u16` samples at the image depth
    /// (`sample_depth` = `depth` above 8), in increasing y order.
    ///
    /// # Errors
    /// Returns an error for a poisoned frame, an out-of-order row, an
    /// incorrect row length, an 8-bit frame, a sample above `2^depth - 1`, or
    /// a failed strip conversion.
    pub fn write_row_u16(&mut self, y: u32, rgba: &[u16]) -> Result<(), EncodeError> {
        self.check_not_poisoned()?;
        self.check_row(y, rgba.len(), self.request.depth.max(9))?;
        let max = (1_u32 << self.request.depth) - 1;
        if rgba.iter().any(|&sample| u32::from(sample) > max) {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend received a sample above the image depth",
            ));
        }
        let FrameSamples::U16(buf) = &mut self.strip else {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend received rows out of order, of the wrong length or depth",
            ));
        };
        if self.request.has_alpha {
            buf.extend_from_slice(rgba);
        } else {
            for pixel in rgba.as_chunks::<4>().0 {
                buf.extend_from_slice(&pixel[..3]);
            }
        }
        self.finish_row(y)
    }

    fn finish_row(&mut self, y: u32) -> Result<(), EncodeError> {
        self.buffered += 1;
        if self.buffered == self.strip_rows || y + 1 == self.request.height {
            let count = self.buffered;
            let y_offset = self.strip_y;
            let row_bytes = match rgb_row_bytes(&self.request) {
                Some(bytes) => bytes,
                None => {
                    self.poisoned = true;
                    self.strip.clear();
                    self.buffered = 0;
                    return Err(EncodeError::new(
                        ErrorKind::InvalidOptions,
                        "row size overflows",
                    ));
                }
            };
            let bands = conversion_bands(
                self.request.width,
                count,
                strip_conversion_threads(&self.request, self.serial_conversion),
            );
            let result = rgb_to_yuv_range(
                &self.image,
                &self.request,
                self.strip.as_band_mut(),
                y_offset,
                count,
                row_bytes,
                bands,
            );
            self.strip.clear();
            self.buffered = 0;
            if let Err(err) = result {
                self.poisoned = true;
                return Err(err);
            }
            self.strip_y += count;
        }
        self.next = y + 1;
        Ok(())
    }

    fn check_row(&self, y: u32, samples: usize, depth: u8) -> Result<(), EncodeError> {
        if y != self.next
            || y >= self.request.height
            || samples != 4 * self.request.width as usize
            || (self.request.sample_depth > 8) != (depth > 8)
        {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend received rows out of order, of the wrong length or depth",
            ));
        }
        Ok(())
    }

    /// Encode the complete frame. Codec scratch and compressed bytes are not
    /// covered by `required_frame_bytes`.
    ///
    /// # Errors
    /// Returns an error for a poisoned frame, incomplete input, failed
    /// encoding or allocation.
    pub fn finish(self) -> Result<Vec<u8>, EncodeError> {
        self.check_not_poisoned()?;
        if self.next != self.request.height || self.buffered != 0 {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend finished before the last row",
            ));
        }
        drop(self.strip);
        encode_image(&self.request, &self.image)
    }
}

/// Channels stored in the RGB(A) conversion strip: 3 for opaque frames
/// (`AVIF_RGB_FORMAT_RGB`), 4 when alpha is encoded (`AVIF_RGB_FORMAT_RGBA`).
fn rgb_channels(request: &Config) -> usize {
    if request.has_alpha { 4 } else { 3 }
}

/// Bytes per RGB(A) sample of the strip: 1 at 8 bits, 2 (`uint16_t`) above.
fn rgb_sample_bytes(request: &Config) -> usize {
    if request.sample_depth > 8 { 2 } else { 1 }
}

/// Byte stride of one RGB(A) row in the conversion strip.
fn rgb_row_bytes(request: &Config) -> Option<u32> {
    let pixel_bytes = u32::try_from(rgb_channels(request) * rgb_sample_bytes(request)).ok()?;
    request.width.checked_mul(pixel_bytes)
}

/// Thread budget for row-intake strip conversion: 1 when the caller selected
/// serial conversion (avoiding nested thread pools during `write_row`),
/// otherwise `request.threads` clamped to `1..=MAX_THREADS`.
fn strip_conversion_threads(request: &Config, serial_conversion: bool) -> usize {
    if serial_conversion {
        1
    } else {
        request.threads.clamp(1, MAX_THREADS)
    }
}

/// Rows buffered in one streaming RGB-to-YUV strip. Single-threaded, narrow, or
/// serial-conversion encodes buffer at most two rows (the 2x2 block height of
/// 4:2:0); standalone multi-threaded encodes whose strips meet
/// `MIN_PIXELS_PER_BAND` buffer enough rows per active conversion band (at
/// least `MIN_BAND_ROWS`, capped at `MIN_BAND_ROWS * min(threads, MAX_BANDS)`)
/// so parallel conversion amortizes worker start-up without materializing a
/// full RGB frame. Always even (unless `height == 1`) so every strip starts on
/// an even row as 4:2:0 `avifImageSetViewRect` requires.
fn strip_rows(request: &Config, serial_conversion: bool) -> u32 {
    let threads = strip_conversion_threads(request, serial_conversion);
    let max_strip = MIN_BAND_ROWS
        .saturating_mul(u32::try_from(threads.min(MAX_BANDS)).unwrap_or(1))
        .min(request.height);
    let bands = conversion_bands(request.width, max_strip, threads);
    if bands == 1 {
        request.height.min(2)
    } else {
        let width = u64::from(request.width).max(1);
        let rows_for_pixels =
            u32::try_from(MIN_PIXELS_PER_BAND.div_ceil(width)).unwrap_or(u32::MAX);
        let rows_per_band = MIN_BAND_ROWS.max(rows_for_pixels).next_multiple_of(2);
        let bands = u32::try_from(bands).unwrap_or(1);
        rows_per_band.saturating_mul(bands).min(max_strip)
    }
}

/// The active RGB(A) strip plus libavif's YUV(A) planes, which are alive
/// together while rows stream into the encoder.
fn frame_bytes(request: &Config, serial_conversion: bool) -> Option<usize> {
    let width = usize::try_from(request.width).ok()?;
    let height = usize::try_from(request.height).ok()?;
    let pixels = width.checked_mul(height)?;
    let sample: usize = if request.depth > 8 { 2 } else { 1 };
    let chroma = match request.chroma {
        Chroma::Yuv444 => pixels.checked_mul(2)?,
        Chroma::Yuv420 => width
            .div_ceil(2)
            .checked_mul(height.div_ceil(2))?
            .checked_mul(2)?,
    };
    let alpha = if request.has_alpha { pixels } else { 0 };
    let planes = pixels
        .checked_add(chroma)?
        .checked_add(alpha)?
        .checked_mul(sample)?;
    let strip_height = usize::try_from(strip_rows(request, serial_conversion)).ok()?;
    let strip_bytes = width
        .checked_mul(strip_height)?
        .checked_mul(rgb_channels(request) * rgb_sample_bytes(request))?;
    strip_bytes.checked_add(planes)
}

fn failure(result: u32, during: &str) -> EncodeError {
    // SAFETY: `avifResultToString` returns a pointer to a static,
    // NUL-terminated string for every value, including unknown ones.
    let text = unsafe { CStr::from_ptr(image_api::avifResultToString(result)) }
        .to_string_lossy()
        .into_owned();
    let kind = match result {
        AVIF_RESULT_OUT_OF_MEMORY => ErrorKind::ResourceExhausted,
        AVIF_RESULT_INVALID_ARGUMENT => ErrorKind::InvalidOptions,
        _ => ErrorKind::Failed,
    };
    EncodeError::new(kind, format!("libavif {during}: {text}"))
}

/// Owns an `avifImage` from `avifImageCreate`.
struct Image(*mut image_api::avifImage);
// SAFETY: `Image` uniquely owns its `avifImage` and its `avifAlloc`-allocated
// YUV/alpha planes; `avifImage` carries only plain metadata and heap pointers
// with no thread-affine state, and `View`s never outlive a synchronous
// conversion call.
unsafe impl Send for Image {}
impl Drop for Image {
    fn drop(&mut self) {
        // SAFETY: the pointer came from `avifImageCreate`, is not null
        // (checked at construction) and is destroyed exactly once, here.
        unsafe { image_api::avifImageDestroy(self.0) }
    }
}

fn allocate_planes(image: &Image, has_alpha: bool) -> Result<(), EncodeError> {
    let planes = if has_alpha {
        image_api::AVIF_PLANES_ALL
    } else {
        image_api::AVIF_PLANES_YUV
    };
    // SAFETY: `image.0` is a valid image with nonzero size and a supported
    // depth and format; the planes it allocates belong to `image` and are
    // freed by `Image`.
    let result = unsafe { image_api::avifImageAllocatePlanes(image.0, planes) };
    if result != AVIF_RESULT_OK {
        return Err(failure(result, "plane allocation"));
    }
    Ok(())
}

fn create_view(image: &Image, width: u32, y: u32, height: u32) -> Result<View, EncodeError> {
    // SAFETY: no arguments; a null result is handled below.
    let view = unsafe { image_api::avifImageCreateEmpty() };
    if view.is_null() {
        return Err(EncodeError::new(
            ErrorKind::ResourceExhausted,
            "libavif could not create a band view",
        ));
    }
    let view = View(view);
    let rect = image_api::avifCropRect {
        x: 0,
        y,
        width,
        height,
    };
    // SAFETY: `view.0` and `image.0` are distinct valid images; the rectangle
    // lies inside the image and starts on an even row, as 4:2:0 requires. The
    // view only points into `image`'s pre-allocated planes, which outlive it.
    let result = unsafe { image_api::avifImageSetViewRect(view.0, image.0, &raw const rect) };
    if result != AVIF_RESULT_OK {
        return Err(failure(result, "band view"));
    }
    Ok(view)
}

/// Owns an `avifEncoder` from `avifEncoderCreate`.
struct Encoder(*mut write::avifEncoder);
impl Drop for Encoder {
    fn drop(&mut self) {
        // SAFETY: as for `Image`: non-null, from `avifEncoderCreate`,
        // destroyed once.
        unsafe { write::avifEncoderDestroy(self.0) }
    }
}

/// Owns the output buffer libavif allocates in `avifEncoderWrite`.
struct Output(write::avifRWData);
impl Drop for Output {
    fn drop(&mut self) {
        // SAFETY: the struct is zero-initialized (null data, size 0) or
        // filled by `avifEncoderWrite`; `avifRWDataFree` accepts both and
        // resets it. `rawdata::avifRWData` and `write::avifRWData` are the
        // same C struct, generated once per translation unit.
        unsafe { rawdata::avifRWDataFree((&raw mut self.0).cast()) }
    }
}

/// Row bands for the RGB-to-YUV conversion: one per encoder thread, at most
/// `MAX_BANDS`, each at least `MIN_BAND_ROWS` rows and at least
/// `MIN_PIXELS_PER_BAND` pixels. One thread (or a small/narrow strip) keeps
/// the serial conversion path without spawning worker threads.
fn conversion_bands(width: u32, height: u32, threads: usize) -> usize {
    let by_rows = usize::try_from(height / MIN_BAND_ROWS).unwrap_or(1);
    let pixels = u64::from(width).saturating_mul(u64::from(height));
    let by_pixels = usize::try_from(pixels / MIN_PIXELS_PER_BAND).unwrap_or(usize::MAX);
    threads.min(MAX_BANDS).min(by_rows).min(by_pixels).max(1)
}

/// The first row and row count of each band when `height` rows are split
/// into at most `bands` bands. Every band but the last has an even height,
/// so every band starts on an even row: a 4:2:0 view must start on a chroma
/// row, and each 2x2 chroma average then reads the same two rows as in the
/// whole image.
fn band_rows(height: u32, bands: usize) -> Vec<(u32, u32)> {
    let bands = u32::try_from(bands.max(1)).unwrap_or(1);
    let step = height.div_ceil(bands).next_multiple_of(2).max(2);
    let mut rows = Vec::new();
    let mut y = 0;
    while y < height {
        let count = step.min(height - y);
        rows.push((y, count));
        y += count;
    }
    rows
}

/// A view of one row band of an image's planes, from
/// `avifImageSetViewRect`. It points into the parent's planes and never
/// frees them.
struct View(*mut image_api::avifImage);
// SAFETY: a `View` is an `avifImage` struct used by one band thread only.
// Its plane pointers address rows of the parent's planes that no other band
// touches (`band_rows` gives disjoint rows; 4:2:0 bands start on even rows,
// so their chroma rows are disjoint too). The built-in path of
// `avifImageRGBToYUV` keeps its state in locals and per-call allocations,
// not in globals.
unsafe impl Send for View {}
impl Drop for View {
    fn drop(&mut self) {
        // SAFETY: `self.0` came from `avifImageCreateEmpty`, is not null and
        // is destroyed exactly once, here. `avifImageRGBToYUV` marks a view's
        // planes as owned when it "allocates" them (they are already set, so
        // it allocates nothing); clearing the pointers and flags first keeps
        // `avifImageDestroy` from freeing the parent's memory.
        unsafe {
            (*self.0).yuvPlanes = [std::ptr::null_mut(); 3];
            (*self.0).imageOwnsYUVPlanes = 0;
            (*self.0).alphaPlane = std::ptr::null_mut();
            (*self.0).imageOwnsAlphaPlane = 0;
            image_api::avifImageDestroy(self.0);
        }
    }
}

/// Create a scoped worker without panicking when the OS cannot create a thread.
fn spawn_band<'scope, 'env, F, T>(
    scope: &'scope std::thread::Scope<'scope, 'env>,
    worker: F,
) -> std::io::Result<std::thread::ScopedJoinHandle<'scope, T>>
where
    F: FnOnce() -> T + Send + 'scope,
    T: Send + 'scope,
{
    #[cfg(test)]
    if let Some(action) = FAIL_SPAWN_AFTER.with(|remaining| match remaining.get() {
        Some((0, action)) => Some(action),
        Some((n, action)) => {
            remaining.set(Some((n - 1, action)));
            None
        }
        None => None,
    }) {
        match action {
            SpawnFault::Error => {
                return Err(std::io::Error::other(
                    "injected scoped thread spawn failure",
                ));
            }
            SpawnFault::Panic => panic!("injected scoped thread spawn panic"),
        }
    }
    std::thread::Builder::new().spawn_scoped(scope, worker)
}

#[cfg(test)]
#[derive(Clone, Copy)]
enum SpawnFault {
    Error,
    Panic,
}

#[cfg(test)]
thread_local! {
    static FAIL_SPAWN_AFTER: std::cell::Cell<Option<(usize, SpawnFault)>> =
        const { std::cell::Cell::new(None) };
}

/// Keep opaque 8-bit 4:4:4 RGB conversion on libavif's built-in converter even
/// when `libyuv` is linked: previously `AVIF_RGB_FORMAT_RGBA` fell back to the
/// built-in converter because `libyuv` has no full-range RGBA 4:4:4 path,
/// whereas `AVIF_RGB_FORMAT_RGB` would pick `RAWToJ444` with different
/// rounding.
fn should_avoid_libyuv(request: &Config) -> bool {
    request.sample_depth == 8 && request.chroma == Chroma::Yuv444 && !request.has_alpha
}

/// Configure an `avifRGBImage` header for one RGB (opaque) or RGBA (alpha) band.
///
/// # Safety
/// `target` must be a valid `avifImage` pointer.
unsafe fn configure_rgb_image(
    target: *mut image_api::avifImage,
    request: &Config,
    pixels: *mut u8,
    row_bytes: u32,
) -> reformat::avifRGBImage {
    // SAFETY: an all-zero `avifRGBImage` is a valid value of the C struct
    // (integers and a null pointer); it is fully initialized just below.
    let mut rgb: reformat::avifRGBImage = unsafe { std::mem::zeroed() };
    // SAFETY: `rgb` is a valid, writable `avifRGBImage` and `target` a valid
    // image. The `avif` and `reformat` modules translate the same C struct,
    // so the pointer casts keep its layout.
    unsafe { image_api::avifRGBImageSetDefaults((&raw mut rgb).cast(), target.cast()) };
    rgb.format = if request.has_alpha {
        AVIF_RGB_FORMAT_RGBA
    } else {
        AVIF_RGB_FORMAT_RGB
    };
    rgb.depth = u32::from(request.sample_depth);
    rgb.ignoreAlpha = i32::from(!request.has_alpha);
    rgb.alphaPremultiplied = 0;
    rgb.avoidLibYUV = i32::from(should_avoid_libyuv(request));
    rgb.pixels = pixels;
    rgb.rowBytes = row_bytes;
    rgb
}

/// Convert one band of RGB (opaque) or RGBA (alpha) rows into `target`'s planes.
fn convert_band(
    target: *mut image_api::avifImage,
    request: &Config,
    mut band: BandSamples<'_>,
    row_bytes: u32,
) -> Result<(), EncodeError> {
    let pixels = match (request.sample_depth > 8, &mut band) {
        (false, BandSamples::U8(slice)) => slice.as_mut_ptr(),
        (true, BandSamples::U16(slice)) => slice.as_mut_ptr().cast::<u8>(),
        _ => {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend sample storage does not match sample depth",
            ));
        }
    };
    // SAFETY: `target` is a valid live `avifImage` owned by the caller.
    let rgb = unsafe { configure_rgb_image(target, request, pixels, row_bytes) };
    // SAFETY: `rgb.pixels` points to `rowBytes * height` initialized bytes
    // of `band` (the target's rows), which outlives the call and is not
    // otherwise accessed during it; libavif only reads the RGB image here.
    // When `sample_depth > 8`, `band` is `BandSamples::U16` (2-byte aligned)
    // and `rowBytes` is a multiple of 6 (opaque RGB) or 8 (RGBA), so every
    // row and sample address is 2-byte aligned for libavif's `*const u16`
    // reads. `image`'s planes are already allocated and are written in place.
    let result = unsafe { reformat::avifImageRGBToYUV(target.cast(), &raw const rgb) };
    if result == AVIF_RESULT_OK {
        Ok(())
    } else {
        Err(failure(result, "RGB to YUV conversion"))
    }
}

/// Convert `count` RGB(A) rows starting at even image row `y_offset` into
/// `image`'s pre-allocated planes in up to `bands` row bands: band 0 runs on
/// the calling thread and bands `1..` run on fallibly spawned scoped threads.
/// libavif's built-in conversion works per pixel for 4:4:4 and per 2x2 block
/// for 4:2:0, so the samples are identical for every strip height and band
/// count (`tests::conversion_bands_do_not_change_samples` and
/// `tests::streaming_and_rgb_format_match_full_frame_rgba`).
fn rgb_to_yuv_range(
    image: &Image,
    request: &Config,
    rgba: BandSamples<'_>,
    y_offset: u32,
    count: u32,
    row_bytes: u32,
    bands: usize,
) -> Result<(), EncodeError> {
    let rows = band_rows(count, bands);
    if rows.len() == 1 {
        if y_offset == 0 && count == request.height {
            return convert_band(image.0, request, rgba, row_bytes);
        }
        let view = create_view(image, request.width, y_offset, count)?;
        return convert_band(view.0, request, rgba, row_bytes);
    }
    let row_samples = request.width as usize * rgb_channels(request);
    let mut jobs = Vec::with_capacity(rows.len());
    let mut rest = rgba;
    for &(local_y, band_count) in &rows {
        let (band, tail) = rest.split_at_mut(row_samples * band_count as usize);
        rest = tail;
        let view = create_view(image, request.width, y_offset + local_y, band_count)?;
        jobs.push((view, band));
    }
    let mut jobs_iter = jobs.into_iter();
    let Some((first_view, first_band)) = jobs_iter.next() else {
        return Ok(());
    };
    let gate = (
        std::sync::Mutex::new(None::<bool>),
        std::sync::Condvar::new(),
    );
    struct GateGuard<'a>(&'a (std::sync::Mutex<Option<bool>>, std::sync::Condvar));
    impl GateGuard<'_> {
        fn release(self, proceed: bool) {
            let (lock, condvar) = self.0;
            let mut state = lock
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            *state = Some(proceed);
            condvar.notify_all();
        }
    }
    impl Drop for GateGuard<'_> {
        fn drop(&mut self) {
            let (lock, condvar) = self.0;
            let mut state = lock
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner);
            if state.is_none() {
                *state = Some(false);
                condvar.notify_all();
            }
        }
    }
    std::thread::scope(|scope| {
        // Ensures already-spawned workers waiting on `gate` are cancelled and
        // woken even if the spawn loop unwinds before `release` runs.
        let gate_guard = GateGuard(&gate);
        let mut handles = Vec::with_capacity(rows.len() - 1);
        let mut spawn_error = None;
        for (view, band) in jobs_iter {
            let gate = &gate;
            match spawn_band(scope, move || {
                // Own the whole view, so it is destroyed on this thread
                // after the conversion or if startup is aborted.
                let view = view;
                let proceed = {
                    let (lock, condvar) = gate;
                    let mut state = lock
                        .lock()
                        .unwrap_or_else(std::sync::PoisonError::into_inner);
                    while state.is_none() {
                        state = condvar
                            .wait(state)
                            .unwrap_or_else(std::sync::PoisonError::into_inner);
                    }
                    state.unwrap_or(false)
                };
                if !proceed {
                    return Ok(());
                }
                convert_band(view.0, request, band, row_bytes)
            }) {
                Ok(handle) => handles.push(handle),
                Err(error) => {
                    spawn_error = Some(error);
                    break;
                }
            }
        }
        gate_guard.release(spawn_error.is_none());
        if let Some(error) = spawn_error {
            for handle in handles {
                let _ = handle.join();
            }
            drop(first_view);
            return Err(EncodeError::new(
                ErrorKind::ResourceExhausted,
                format!("libavif could not start an RGB to YUV conversion worker: {error}"),
            ));
        }
        let mut result = convert_band(first_view.0, request, first_band, row_bytes);
        drop(first_view);
        for handle in handles {
            let band_result = handle.join().unwrap_or_else(|_| {
                Err(EncodeError::new(
                    ErrorKind::Failed,
                    "libavif RGB to YUV band panicked",
                ))
            });
            if result.is_ok() {
                result = band_result;
            }
        }
        result
    })
}

/// A planeless image of the request's size, depth and format with the
/// output's CICP and range.
fn new_image(request: &Config, format: u32) -> Result<Image, EncodeError> {
    // SAFETY: plain arguments; a null result is handled below.
    let image = unsafe {
        image_api::avifImageCreate(
            request.width,
            request.height,
            u32::from(request.depth),
            format,
        )
    };
    if image.is_null() {
        return Err(EncodeError::new(
            ErrorKind::ResourceExhausted,
            "libavif could not create the image",
        ));
    }
    let image = Image(image);
    // SAFETY: `image.0` is a valid, exclusively owned `avifImage`; these are
    // plain fields of it.
    unsafe {
        (*image.0).colorPrimaries = AVIF_COLOR_PRIMARIES_BT709;
        (*image.0).transferCharacteristics = AVIF_TRANSFER_CHARACTERISTICS_SRGB;
        (*image.0).matrixCoefficients = AVIF_MATRIX_COEFFICIENTS_BT601;
        (*image.0).yuvRange = AVIF_RANGE_FULL;
    }
    Ok(image)
}

fn encode_image(request: &Config, image: &Image) -> Result<Vec<u8>, EncodeError> {
    // SAFETY: no arguments; a null result is handled below.
    let encoder = unsafe { write::avifEncoderCreate() };
    if encoder.is_null() {
        return Err(EncodeError::new(
            ErrorKind::ResourceExhausted,
            "libavif could not create the encoder",
        ));
    }
    let encoder = Encoder(encoder);
    let threads = request.threads.clamp(1, MAX_THREADS);
    // SAFETY: `encoder.0` is a valid, exclusively owned `avifEncoder`; these
    // are plain fields of it.
    unsafe {
        // Select the reference encoder explicitly even if Cargo feature
        // unification enables additional libavif codecs.
        (*encoder.0).codecChoice = write::AVIF_CODEC_CHOICE_AOM;
        (*encoder.0).quality = i32::from(request.quality);
        (*encoder.0).qualityAlpha = i32::from(request.alpha_quality);
        (*encoder.0).speed = i32::from(request.speed);
        (*encoder.0).maxThreads = i32::try_from(threads).unwrap_or(1);
        (*encoder.0).autoTiling = i32::from(threads != 1);
    }
    // SAFETY: an all-zero `avifRWData` is the empty buffer libavif expects.
    let mut output = Output(unsafe { std::mem::zeroed() });
    // SAFETY: `encoder.0` and `image.0` are valid; the `avif` and `write`
    // modules translate the same `avifImage` struct. libavif writes a newly
    // allocated buffer into `output`, which `Output` frees.
    let result = unsafe { write::avifEncoderWrite(encoder.0, image.0.cast(), &raw mut output.0) };
    if result != AVIF_RESULT_OK {
        return Err(failure(result, "encoding"));
    }
    if output.0.data.is_null() || output.0.size > isize::MAX as usize {
        return Err(EncodeError::new(
            ErrorKind::Failed,
            "libavif returned an invalid output buffer",
        ));
    }
    // SAFETY: on success libavif sets `data` to an allocation of `size`
    // initialized bytes, owned by `output` until it is dropped below.
    let source = unsafe { std::slice::from_raw_parts(output.0.data, output.0.size) };
    let mut bytes = Vec::new();
    bytes.try_reserve_exact(source.len()).map_err(|_| {
        EncodeError::new(
            ErrorKind::ResourceExhausted,
            "cannot allocate the encoded AVIF output",
        )
    })?;
    bytes.extend_from_slice(source);
    Ok(bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Every plane's sample bytes, row by row without padding.
    fn planes(image: &Image, request: &Config) -> Vec<Vec<u8>> {
        let sample = if request.depth > 8 { 2 } else { 1 };
        let (chroma_width, chroma_height) = match request.chroma {
            Chroma::Yuv444 => (request.width, request.height),
            Chroma::Yuv420 => (request.width.div_ceil(2), request.height.div_ceil(2)),
        };
        // SAFETY: `image.0` is a valid image whose planes `rgb_to_yuv`
        // allocated with these dimensions; the reads stay inside them.
        unsafe {
            let image = &*image.0;
            let mut planes: Vec<(*mut u8, u32, u32, u32)> = (0..3)
                .map(|plane| {
                    let (width, height) = if plane == 0 {
                        (request.width, request.height)
                    } else {
                        (chroma_width, chroma_height)
                    };
                    (
                        image.yuvPlanes[plane],
                        image.yuvRowBytes[plane],
                        width,
                        height,
                    )
                })
                .collect();
            if request.has_alpha {
                planes.push((
                    image.alphaPlane,
                    image.alphaRowBytes,
                    request.width,
                    request.height,
                ));
            } else {
                assert!(image.alphaPlane.is_null());
            }
            planes
                .into_iter()
                .map(|(base, row_bytes, width, height)| {
                    assert!(!base.is_null());
                    (0..height as usize)
                        .flat_map(|y| {
                            std::slice::from_raw_parts(
                                base.add(y * row_bytes as usize),
                                width as usize * sample,
                            )
                            .to_vec()
                        })
                        .collect()
                })
                .collect()
        }
    }

    /// Reference full-frame RGBA conversion (`AVIF_RGB_FORMAT_RGBA`), matching
    /// the pre-streaming baseline (`main` at `503007eb`).
    fn full_frame_rgba_image(request: &Config, rgba: &FrameSamples) -> Image {
        let format = match request.chroma {
            Chroma::Yuv444 => AVIF_PIXEL_FORMAT_YUV444,
            Chroma::Yuv420 => AVIF_PIXEL_FORMAT_YUV420,
        };
        let image = new_image(request, format).unwrap();
        let mut copy = match rgba {
            FrameSamples::U8(buf) => FrameSamples::U8(buf.clone()),
            FrameSamples::U16(buf) => FrameSamples::U16(buf.clone()),
        };
        let pixels = match &mut copy {
            FrameSamples::U8(buf) => buf.as_mut_ptr(),
            FrameSamples::U16(buf) => buf.as_mut_ptr().cast::<u8>(),
        };
        // SAFETY: zero-initialized `avifRGBImage` filled immediately below.
        let mut rgb: reformat::avifRGBImage = unsafe { std::mem::zeroed() };
        // SAFETY: `rgb` and `image.0` are valid; pointer casts keep C layout.
        unsafe { image_api::avifRGBImageSetDefaults((&raw mut rgb).cast(), image.0.cast()) };
        rgb.format = AVIF_RGB_FORMAT_RGBA;
        rgb.depth = u32::from(request.sample_depth);
        rgb.ignoreAlpha = i32::from(!request.has_alpha);
        rgb.alphaPremultiplied = 0;
        rgb.pixels = pixels;
        rgb.rowBytes = request.width * 4 * u32::try_from(rgb_sample_bytes(request)).unwrap();
        // SAFETY: `rgb.pixels` holds the complete RGBA frame for `image`.
        let result = unsafe { reformat::avifImageRGBToYUV(image.0.cast(), &raw const rgb) };
        assert_eq!(result, AVIF_RESULT_OK);
        image
    }

    #[test]
    fn conversion_bands_do_not_change_samples() {
        // Odd and even sizes, so that 4:2:0 has a partial last chroma row
        // and column; a pseudo-random frame with varying alpha.
        for (width, height) in [(67_u32, 131_u32), (130, 256), (33, 7)] {
            let mut state = 0x2545_f491_u32;
            let rgba: Vec<u8> = (0..width * height * 4)
                .map(|_| {
                    state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    (state >> 24) as u8
                })
                .collect();
            for chroma in [Chroma::Yuv444, Chroma::Yuv420] {
                for (depth, sample_depth) in [(8, 8), (10, 8), (10, 10), (12, 12)] {
                    for has_alpha in [false, true] {
                        let request = Config {
                            width,
                            height,
                            has_alpha,
                            quality: 75,
                            alpha_quality: 75,
                            speed: 6,
                            threads: 1,
                            chroma,
                            depth,
                            sample_depth,
                        };
                        let format = match chroma {
                            Chroma::Yuv444 => AVIF_PIXEL_FORMAT_YUV444,
                            Chroma::Yuv420 => AVIF_PIXEL_FORMAT_YUV420,
                        };
                        // Deep rows: native u16 samples masked to the depth.
                        let full_rgba: FrameSamples = if sample_depth > 8 {
                            FrameSamples::U16(
                                rgba.chunks(2)
                                    .cycle()
                                    .take(rgba.len())
                                    .map(|pair| {
                                        u16::from_ne_bytes([pair[0], pair[1 % pair.len()]])
                                            & ((1 << sample_depth) - 1)
                                    })
                                    .collect(),
                            )
                        } else {
                            FrameSamples::U8(rgba.clone())
                        };
                        // Packed strip storage: 3 channels for opaque, 4 for alpha.
                        let packed: FrameSamples = match &full_rgba {
                            FrameSamples::U8(buf) if !has_alpha => FrameSamples::U8(
                                buf.as_chunks::<4>()
                                    .0
                                    .iter()
                                    .flat_map(|p| [p[0], p[1], p[2]])
                                    .collect(),
                            ),
                            FrameSamples::U16(buf) if !has_alpha => FrameSamples::U16(
                                buf.as_chunks::<4>()
                                    .0
                                    .iter()
                                    .flat_map(|p| [p[0], p[1], p[2]])
                                    .collect(),
                            ),
                            FrameSamples::U8(buf) => FrameSamples::U8(buf.clone()),
                            FrameSamples::U16(buf) => FrameSamples::U16(buf.clone()),
                        };
                        let reference =
                            planes(&full_frame_rgba_image(&request, &full_rgba), &request);
                        let row_bytes = rgb_row_bytes(&request).unwrap();
                        let convert = |bands| {
                            let image = new_image(&request, format).unwrap();
                            allocate_planes(&image, has_alpha).unwrap();
                            let mut frame = match &packed {
                                FrameSamples::U8(buf) => FrameSamples::U8(buf.clone()),
                                FrameSamples::U16(buf) => FrameSamples::U16(buf.clone()),
                            };
                            rgb_to_yuv_range(
                                &image,
                                &request,
                                frame.as_band_mut(),
                                0,
                                height,
                                row_bytes,
                                bands,
                            )
                            .unwrap();
                            planes(&image, &request)
                        };
                        assert_eq!(
                            convert(1),
                            reference,
                            "{width}x{height} {chroma:?} {depth}-bit alpha {has_alpha} RGB vs RGBA"
                        );
                        for bands in [2, 3, 8] {
                            assert!(band_rows(height, bands).len() > 1);
                            assert_eq!(
                                convert(bands),
                                reference,
                                "{width}x{height} {chroma:?} {depth}-bit alpha {has_alpha} bands {bands}"
                            );
                        }
                        // Streaming row-by-row Frame (1, 2, 4, 8 threads) must
                        // produce bit-identical planes to full-frame RGBA.
                        for threads in [1, 2, 4, 8] {
                            let mut streamed = Frame::new(Config { threads, ..request }).unwrap();
                            match &full_rgba {
                                FrameSamples::U8(buf) => {
                                    for (y, row) in buf.chunks_exact(width as usize * 4).enumerate()
                                    {
                                        streamed.write_row(u32::try_from(y).unwrap(), row).unwrap();
                                    }
                                }
                                FrameSamples::U16(buf) => {
                                    for (y, row) in buf.chunks_exact(width as usize * 4).enumerate()
                                    {
                                        streamed
                                            .write_row_u16(u32::try_from(y).unwrap(), row)
                                            .unwrap();
                                    }
                                }
                            }
                            assert_eq!(
                                planes(&streamed.image, &request),
                                reference,
                                "{width}x{height} {chroma:?} {depth}-bit alpha {has_alpha} streamed threads={threads}"
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn streamed_encoded_avif_bytes_match_full_frame_rgba_baseline() {
        for (width, height) in [(1, 1), (33, 19), (67, 131)] {
            let mut state = 0x1234_5678_u32;
            let rgba: Vec<u8> = (0..width * height * 4)
                .map(|_| {
                    state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                    (state >> 24) as u8
                })
                .collect();
            for chroma in [Chroma::Yuv444, Chroma::Yuv420] {
                for depth in [8_u8, 10, 12] {
                    for has_alpha in [false, true] {
                        for threads in [1, 4] {
                            let request = Config {
                                width,
                                height,
                                has_alpha,
                                quality: 65,
                                alpha_quality: 65,
                                speed: 9,
                                threads,
                                chroma,
                                depth,
                                sample_depth: depth,
                            };
                            let full_rgba = if depth > 8 {
                                FrameSamples::U16(
                                    rgba.chunks(2)
                                        .cycle()
                                        .take(rgba.len())
                                        .map(|pair| {
                                            u16::from_ne_bytes([pair[0], pair[1 % pair.len()]])
                                                & ((1 << depth) - 1)
                                        })
                                        .collect(),
                                )
                            } else {
                                FrameSamples::U8(rgba.clone())
                            };
                            let ref_image = full_frame_rgba_image(&request, &full_rgba);
                            let expected_bytes = encode_image(&request, &ref_image).unwrap();
                            let mut frame = Frame::new(request).unwrap();
                            match &full_rgba {
                                FrameSamples::U8(buf) => {
                                    for (y, row) in buf.chunks_exact(width as usize * 4).enumerate()
                                    {
                                        frame.write_row(u32::try_from(y).unwrap(), row).unwrap();
                                    }
                                }
                                FrameSamples::U16(buf) => {
                                    for (y, row) in buf.chunks_exact(width as usize * 4).enumerate()
                                    {
                                        frame
                                            .write_row_u16(u32::try_from(y).unwrap(), row)
                                            .unwrap();
                                    }
                                }
                            }
                            let got_bytes = frame.finish().unwrap();
                            assert_eq!(
                                got_bytes, expected_bytes,
                                "{width}x{height} {chroma:?} {depth}-bit alpha={has_alpha} threads={threads}"
                            );
                        }
                    }
                }
            }
        }
    }

    /// The budget charge: the active RGB (opaque) or RGBA (alpha) conversion
    /// strip at 1 or 2 bytes per sample plus the YUV(A) planes at the depth.
    #[test]
    fn frame_bytes_count_deep_rows_and_planes() {
        let config = |depth, sample_depth, chroma, has_alpha, threads| Config {
            width: 10,
            height: 6,
            has_alpha,
            quality: 60,
            alpha_quality: 60,
            speed: 6,
            threads,
            chroma,
            depth,
            sample_depth,
        };
        let px = 60;
        // Single-threaded strip is 2 rows: 10 * 2 * 3 (opaque) or * 4 (alpha).
        assert_eq!(
            required_frame_bytes(&config(8, 8, Chroma::Yuv444, false, 1)).unwrap(),
            10 * 2 * 3 + px * 3
        );
        assert_eq!(
            required_frame_bytes(&config(10, 8, Chroma::Yuv444, true, 1)).unwrap(),
            10 * 2 * 4 + px * 4 * 2
        );
        assert_eq!(
            required_frame_bytes(&config(12, 12, Chroma::Yuv420, true, 1)).unwrap(),
            10 * 2 * 4 * 2 + (px + 5 * 3 * 2 + px) * 2
        );
        // Tall multi-threaded image uses a bounded 64 * bands strip (<= 512 rows).
        let tall = Config {
            width: 6000,
            height: 4000,
            has_alpha: false,
            quality: 60,
            alpha_quality: 60,
            speed: 6,
            threads: 8,
            chroma: Chroma::Yuv420,
            depth: 10,
            sample_depth: 10,
        };
        assert_eq!(
            required_frame_bytes(&tall).unwrap(),
            6000 * 512 * 3 * 2 + (6000 * 4000 + 3000 * 2000 * 2) * 2
        );
        assert!(required_frame_bytes(&config(12, 10, Chroma::Yuv444, false, 1)).is_err());
        assert!(required_frame_bytes(&config(16, 16, Chroma::Yuv444, false, 1)).is_err());
        let mut frame = Frame::new(config(10, 10, Chroma::Yuv444, false, 1)).unwrap();
        assert!(
            frame.write_row(0, &[0; 40]).is_err(),
            "8-bit row into a deep frame"
        );
        assert!(
            frame.write_row_u16(0, &[1024; 40]).is_err(),
            "above 10 bits"
        );
        assert!(frame.write_row_u16(0, &[1023; 40]).is_ok());
    }

    /// The test pattern of `tests/fixtures/avif-deep/rows16.py`.
    fn deep_pattern(width: u32, height: u32, depth: u8, alpha: bool) -> Vec<Vec<u16>> {
        let m = (1_u32 << depth) - 1;
        (0..height)
            .map(|y| {
                (0..width)
                    .flat_map(|x| {
                        let hash = x.wrapping_mul(2_654_435_761) ^ y.wrapping_mul(40_503);
                        let noise = (hash >> 13) & 0x3F;
                        let r = (x * m / (width - 1)) ^ noise;
                        let g = y * m / (height - 1);
                        let b = ((x + y) * 37) & m;
                        let a = if !alpha {
                            m
                        } else if x == 0 {
                            0
                        } else {
                            (x * 13 * (m / 255) + y * 5) % (m + 1)
                        };
                        [r, g, b, a].map(|v| u16::try_from(v).unwrap())
                    })
                    .collect()
            })
            .collect()
    }

    /// Deep rows give the C original's bytes (ADR-0041): every fixture in
    /// `tests/fixtures/avif-deep/` was written by `encode_deep.c` linked to
    /// the pinned C libavif 1.4.2 and the portable libaom reference, from
    /// the same samples and settings (README.md there). Never refreshed from
    /// this crate's output.
    #[test]
    fn deep_output_equals_the_c_original() {
        const DIR: &str = concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../tests/fixtures/avif-deep/"
        );
        let (width, height) = (67, 45);
        for (name, depth, chroma, alpha, quality) in [
            ("d10-444-q60", 10, Chroma::Yuv444, false, 60),
            ("d10-420-alpha-q60", 10, Chroma::Yuv420, true, 60),
            ("d10-444-q100", 10, Chroma::Yuv444, false, 100),
            ("d12-444-alpha-q60", 12, Chroma::Yuv444, true, 60),
            ("d12-420-q60", 12, Chroma::Yuv420, false, 60),
        ] {
            let mut frame = Frame::new(Config {
                width,
                height,
                has_alpha: alpha,
                quality,
                alpha_quality: quality,
                speed: 6,
                threads: 1,
                chroma,
                depth,
                sample_depth: depth,
            })
            .unwrap();
            for (y, row) in deep_pattern(width, height, depth, alpha).iter().enumerate() {
                frame.write_row_u16(u32::try_from(y).unwrap(), row).unwrap();
            }
            let bytes = frame.finish().unwrap();
            let expected = std::fs::read(format!("{DIR}{name}.avif")).unwrap();
            assert!(
                bytes == expected,
                "{name}: {} bytes, the C original wrote {}",
                bytes.len(),
                expected.len()
            );
        }
    }

    #[test]
    fn bands_cover_the_rows_once_and_start_on_even_rows() {
        for height in [1, 2, 3, 63, 64, 127, 128, 129, 1067, 4000] {
            for bands in 1..=9 {
                let rows = band_rows(height, bands);
                assert!(rows.len() <= bands.max(1));
                let mut next = 0;
                for &(y, count) in &rows {
                    assert_eq!(y, next);
                    assert_eq!(y % 2, 0);
                    assert!(count > 0);
                    next += count;
                }
                assert_eq!(next, height);
            }
        }
        assert_eq!(conversion_bands(1600, 1067, 1), 1);
        assert_eq!(conversion_bands(1600, 1067, 20), 8);
        assert_eq!(conversion_bands(1600, 100, 20), 1);
        assert_eq!(conversion_bands(1600, 130, 20), 2);
        // Narrow strips stay single-banded even with many rows and threads.
        assert_eq!(conversion_bands(16, 512, 8), 1);
        assert_eq!(conversion_bands(16, 8192, 8), 2);
        assert_eq!(conversion_bands(512, 512, 8), 4);
        assert_eq!(conversion_bands(4000, 512, 8), 8);
    }

    struct FailSpawnGuard(Option<(usize, SpawnFault)>);
    impl Drop for FailSpawnGuard {
        fn drop(&mut self) {
            FAIL_SPAWN_AFTER.with(|limit| limit.set(self.0));
        }
    }

    fn fail_spawn_after(successful: usize) -> FailSpawnGuard {
        let previous =
            FAIL_SPAWN_AFTER.with(|limit| limit.replace(Some((successful, SpawnFault::Error))));
        FailSpawnGuard(previous)
    }

    fn panic_spawn_after(successful: usize) -> FailSpawnGuard {
        let previous =
            FAIL_SPAWN_AFTER.with(|limit| limit.replace(Some((successful, SpawnFault::Panic))));
        FailSpawnGuard(previous)
    }

    #[test]
    fn deep_frame_uses_aligned_u16_storage() {
        for (depth, has_alpha, expected_channels) in [(10, true, 4), (12, true, 4), (10, false, 3)]
        {
            let mut frame = Frame::new(Config {
                width: 9,
                height: 5,
                has_alpha,
                quality: 60,
                alpha_quality: 60,
                speed: 6,
                threads: 1,
                chroma: Chroma::Yuv444,
                depth,
                sample_depth: depth,
            })
            .unwrap();
            let row = vec![(1_u16 << depth) - 1; 9 * 4];
            // Row 0 buffers into the 2-row strip without flushing yet.
            frame.write_row_u16(0, &row).unwrap();
            match &frame.strip {
                FrameSamples::U16(samples) => {
                    assert_eq!(samples.len(), 9 * expected_channels);
                    assert_eq!(samples.capacity(), 9 * 2 * expected_channels);
                    assert_eq!(
                        samples.as_ptr().align_offset(std::mem::align_of::<u16>()),
                        0
                    );
                }
                FrameSamples::U8(_) => panic!("deep frame must store native u16 samples"),
            }
            for y in 1..5 {
                frame.write_row_u16(y, &row).unwrap();
            }
            assert_eq!(frame.buffered, 0);
        }
    }

    #[test]
    fn band_spawn_failure_returns_resource_exhausted_and_band_zero_runs_on_caller() {
        let (width, height) = (1024_u32, 260_u32);
        let request = Config {
            width,
            height,
            has_alpha: true,
            quality: 60,
            alpha_quality: 60,
            speed: 6,
            threads: 4,
            chroma: Chroma::Yuv420,
            depth: 10,
            sample_depth: 10,
        };
        let bands = conversion_bands(width, height, request.threads);
        let actual_bands = band_rows(height, bands).len();
        assert_eq!(actual_bands, 4);
        let samples: Vec<u16> = (0..(width * height * 4) as usize)
            .map(|i| (i as u16).wrapping_mul(13) & 1023)
            .collect();
        let row_bytes = rgb_row_bytes(&request).unwrap();

        let serial_image = new_image(&request, AVIF_PIXEL_FORMAT_YUV420).unwrap();
        allocate_planes(&serial_image, request.has_alpha).unwrap();
        let mut serial_frame = FrameSamples::U16(samples.clone());
        rgb_to_yuv_range(
            &serial_image,
            &request,
            serial_frame.as_band_mut(),
            0,
            height,
            row_bytes,
            1,
        )
        .unwrap();
        let expected_planes = planes(&serial_image, &request);

        // Failing on spawn 0, 1, or 2 (out of 3 worker spawns) returns
        // ResourceExhausted without panicking or leaking views.
        for successful_spawns in 0..(actual_bands - 1) {
            let _guard = fail_spawn_after(successful_spawns);
            let image = new_image(&request, AVIF_PIXEL_FORMAT_YUV420).unwrap();
            allocate_planes(&image, request.has_alpha).unwrap();
            let mut frame = FrameSamples::U16(samples.clone());
            let err = rgb_to_yuv_range(
                &image,
                &request,
                frame.as_band_mut(),
                0,
                height,
                row_bytes,
                bands,
            )
            .unwrap_err();
            let (kind, message) = err.into_parts();
            assert_eq!(
                kind,
                ErrorKind::ResourceExhausted,
                "spawn failure after {successful_spawns} workers must report ResourceExhausted: {message}"
            );
        }

        // Allowing exactly `actual_bands - 1` spawns succeeds because band 0
        // runs on the calling thread instead of spawning a dedicated worker.
        let _guard = fail_spawn_after(actual_bands - 1);
        let image = new_image(&request, AVIF_PIXEL_FORMAT_YUV420).unwrap();
        allocate_planes(&image, request.has_alpha).unwrap();
        let mut frame = FrameSamples::U16(samples.clone());
        rgb_to_yuv_range(
            &image,
            &request,
            frame.as_band_mut(),
            0,
            height,
            row_bytes,
            bands,
        )
        .unwrap();
        assert_eq!(planes(&image, &request), expected_planes);

        // If the spawn loop unwinds after at least one worker is waiting on the
        // startup gate, the drop guard cancels and wakes it so `thread::scope`
        // joins without deadlocking.
        let panicked = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            let _guard = panic_spawn_after(1);
            let image = new_image(&request, AVIF_PIXEL_FORMAT_YUV420).unwrap();
            allocate_planes(&image, request.has_alpha).unwrap();
            let mut frame = FrameSamples::U16(samples);
            let _ = rgb_to_yuv_range(
                &image,
                &request,
                frame.as_band_mut(),
                0,
                height,
                row_bytes,
                bands,
            );
        }));
        assert!(panicked.is_err());
    }

    #[test]
    fn failed_strip_flush_poisons_frame_against_retry_and_finish() {
        let (width, height) = (1024_u32, 260_u32);
        for (depth, has_alpha) in [(8_u8, false), (10_u8, true)] {
            let request = Config {
                width,
                height,
                has_alpha,
                quality: 60,
                alpha_quality: 60,
                speed: 9,
                threads: 4,
                chroma: Chroma::Yuv420,
                depth,
                sample_depth: depth,
            };
            let mut frame = Frame::new(request).unwrap();
            assert_eq!(frame.strip_rows, 256);
            let row_u8 = vec![128_u8; width as usize * 4];
            let row_u16 = vec![512_u16; width as usize * 4];

            // Buffer rows 0..255 without flushing yet.
            for y in 0..255 {
                if depth > 8 {
                    frame.write_row_u16(y, &row_u16).unwrap();
                } else {
                    frame.write_row(y, &row_u8).unwrap();
                }
            }

            // Fail worker spawn on row 255, which triggers the 256-row strip flush.
            let flush_err = {
                let _guard = fail_spawn_after(0);
                if depth > 8 {
                    frame.write_row_u16(255, &row_u16).unwrap_err()
                } else {
                    frame.write_row(255, &row_u8).unwrap_err()
                }
            };
            assert_eq!(flush_err.into_parts().0, ErrorKind::ResourceExhausted);
            assert!(frame.poisoned);

            // Even after spawn failures are cleared, retrying row 255, advancing
            // to row 256, or calling finish() must fail without encoding
            // uninitialized plane rows.
            for retry_y in [255_u32, 256_u32] {
                let retry_err = if depth > 8 {
                    frame.write_row_u16(retry_y, &row_u16).unwrap_err()
                } else {
                    frame.write_row(retry_y, &row_u8).unwrap_err()
                };
                let (kind, msg) = retry_err.into_parts();
                assert_eq!(kind, ErrorKind::Failed, "depth={depth} retry_y={retry_y}");
                assert!(msg.contains("previous strip flush"), "{msg}");
            }
            let (kind, msg) = frame.finish().unwrap_err().into_parts();
            assert_eq!(kind, ErrorKind::Failed);
            assert!(msg.contains("previous strip flush"), "{msg}");
        }
    }

    #[test]
    fn opaque_8bit_444_rgb_avoids_libyuv_for_conversion_parity() {
        for (depth, chroma, has_alpha, expect_avoid) in [
            (8_u8, Chroma::Yuv444, false, true),
            (8_u8, Chroma::Yuv444, true, false),
            (10_u8, Chroma::Yuv444, false, false),
            (8_u8, Chroma::Yuv420, false, false),
            (8_u8, Chroma::Yuv420, true, false),
        ] {
            let request = Config {
                width: 8,
                height: 8,
                has_alpha,
                quality: 60,
                alpha_quality: 60,
                speed: 9,
                threads: 1,
                chroma,
                depth,
                sample_depth: depth,
            };
            let pixel_format = match chroma {
                Chroma::Yuv420 => AVIF_PIXEL_FORMAT_YUV420,
                Chroma::Yuv444 => AVIF_PIXEL_FORMAT_YUV444,
            };
            let image = new_image(&request, pixel_format).unwrap();
            let mut dummy = vec![0_u8; 8 * 8 * 8];
            // SAFETY: `image.0` is a valid live `avifImage`.
            let rgb = unsafe { configure_rgb_image(image.0, &request, dummy.as_mut_ptr(), 32) };
            assert_eq!(
                rgb.avoidLibYUV,
                i32::from(expect_avoid),
                "depth={depth} chroma={chroma:?} has_alpha={has_alpha}"
            );
            assert_eq!(
                rgb.format,
                if has_alpha {
                    AVIF_RGB_FORMAT_RGBA
                } else {
                    AVIF_RGB_FORMAT_RGB
                }
            );
        }
    }

    #[test]
    fn tall_narrow_frame_stays_single_band_and_encodes_identically_across_threads() {
        assert_eq!(conversion_bands(16, 512, 8), 1);
        assert_eq!(conversion_bands(4000, 512, 8), 8);
        let make_config = |threads| Config {
            width: 16,
            height: 8192,
            has_alpha: false,
            quality: 60,
            alpha_quality: 60,
            speed: 10,
            threads,
            chroma: Chroma::Yuv444,
            depth: 8,
            sample_depth: 8,
        };
        assert_eq!(strip_rows(&make_config(8), false), 2);

        let encode = |threads| {
            let mut frame = Frame::new(make_config(threads)).unwrap();
            let mut row = vec![0_u8; 16 * 4];
            for y in 0..8192_u32 {
                for x in 0..16_u32 {
                    let idx = x as usize * 4;
                    row[idx] = ((x * 17 + y) & 0xff) as u8;
                    row[idx + 1] = ((y.wrapping_mul(3) ^ x) & 0xff) as u8;
                    row[idx + 2] = (((x + y) * 5) & 0xff) as u8;
                    row[idx + 3] = 255;
                }
                frame.write_row(y, &row).unwrap();
            }
            frame.finish().unwrap()
        };

        let serial = encode(1);
        // Any conversion worker spawn would fail immediately under `fail_spawn_after(0)`.
        let parallel = {
            let _guard = fail_spawn_after(0);
            encode(8)
        };
        assert_eq!(parallel, serial);
    }

    #[test]
    fn concurrent_row_producer_keeps_strip_conversion_serial_and_byte_identical() {
        let wide_config = Config {
            width: 4000,
            height: 512,
            has_alpha: false,
            quality: 60,
            alpha_quality: 60,
            speed: 10,
            threads: 8,
            chroma: Chroma::Yuv444,
            depth: 8,
            sample_depth: 8,
        };
        // Standalone multi-threaded config uses full 512-row 8-band strips;
        // serial-conversion mode keeps a compact 2-row serial strip.
        assert_eq!(strip_rows(&wide_config, false), 512);
        assert_eq!(strip_rows(&wide_config, true), 2);
        assert!(
            required_frame_bytes_serial_conversion(&wide_config).unwrap()
                < required_frame_bytes(&wide_config).unwrap()
        );

        // Full 4000x512 strip intake under `fail_spawn_after(0)` proves zero
        // conversion workers spawn during `write_row` when `new_serial_conversion`
        // is used, while producing identical YUV planes.
        let mut standalone_wide = Frame::new(wide_config).unwrap();
        let mut producer_wide = Frame::new_serial_conversion(wide_config).unwrap();
        let mut wide_row = vec![0_u8; 4000 * 4];
        for y in 0..512_u32 {
            for x in 0..4000_u32 {
                let idx = x as usize * 4;
                wide_row[idx] = ((x.wrapping_mul(7) + y) & 0xff) as u8;
                wide_row[idx + 1] = ((y.wrapping_mul(11) ^ x) & 0xff) as u8;
                wide_row[idx + 2] = (((x + y) * 3) & 0xff) as u8;
                wide_row[idx + 3] = 255;
            }
            standalone_wide.write_row(y, &wide_row).unwrap();
            let _guard = fail_spawn_after(0);
            producer_wide.write_row(y, &wide_row).unwrap();
        }
        assert_eq!(
            planes(&producer_wide.image, &wide_config),
            planes(&standalone_wide.image, &wide_config),
        );
        drop(standalone_wide);
        drop(producer_wide);

        // Full encode (including multi-threaded `avifEncoderWrite` in `finish`)
        // on a 1024x260 multi-band frame is 100% byte-identical between
        // standalone parallel conversion and `new_serial_conversion`.
        let encode = |serial_conversion| {
            let config = Config {
                width: 1024,
                height: 260,
                has_alpha: false,
                quality: 60,
                alpha_quality: 60,
                speed: 10,
                threads: 8,
                chroma: Chroma::Yuv444,
                depth: 8,
                sample_depth: 8,
            };
            let mut frame = if serial_conversion {
                Frame::new_serial_conversion(config).unwrap()
            } else {
                Frame::new(config).unwrap()
            };
            let mut row = vec![0_u8; 1024 * 4];
            for y in 0..260_u32 {
                for x in 0..1024_u32 {
                    let idx = x as usize * 4;
                    row[idx] = ((x.wrapping_mul(5) + y) & 0xff) as u8;
                    row[idx + 1] = ((y.wrapping_mul(9) ^ x) & 0xff) as u8;
                    row[idx + 2] = (((x + y) * 13) & 0xff) as u8;
                    row[idx + 3] = 255;
                }
                if serial_conversion {
                    let _guard = fail_spawn_after(0);
                    frame.write_row(y, &row).unwrap();
                } else {
                    frame.write_row(y, &row).unwrap();
                }
            }
            frame.finish().unwrap()
        };
        let standalone = encode(false);
        let concurrent = encode(true);
        assert_eq!(concurrent, standalone);
    }
}
