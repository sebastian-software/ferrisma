//! Safe, narrow still-image encoder boundary over the workspace libavif port.
//! Input rows are straight-alpha sRGB8 RGBA; output uses CICP 1/13/6 and full range.
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
/// alpha bytes; true treats them as straight alpha. `threads` is clamped to
/// 1–256. Quality and speed use libavif's scales.
#[derive(Clone, Copy, Debug)]
pub struct Config {
    pub width: u32,
    pub height: u32,
    pub has_alpha: bool,
    pub quality: f32,
    pub speed: u8,
    pub threads: usize,
    pub chroma: Chroma,
    pub depth: u8,
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
const AVIF_RGB_FORMAT_RGBA: u32 = 1;
const AVIF_RANGE_FULL: u32 = 1;
const AVIF_COLOR_PRIMARIES_BT709: u16 = 1;
const AVIF_TRANSFER_CHARACTERISTICS_SRGB: u16 = 13;
const AVIF_MATRIX_COEFFICIENTS_BT601: u16 = 6;
/// libavif's speed range, `AVIF_SPEED_SLOWEST` to `AVIF_SPEED_FASTEST`.
const MAX_SPEED: u8 = 10;
/// Ferrisma caps encoder threads at 256 as well.
const MAX_THREADS: usize = 256;
/// At most this many row bands convert RGB to YUV at once.
const MAX_BANDS: usize = 8;
/// Bands shorter than this cost more in thread start-up than they save.
const MIN_BAND_ROWS: u32 = 64;

/// Validate the request and return the simultaneous RGBA and YUV(A) sample bytes.
/// This excludes allocator overhead, codec scratch and compressed output.
///
/// # Errors
/// Returns `InvalidOptions` for unsupported settings or size overflow.
pub fn required_frame_bytes(request: &Config) -> Result<usize, EncodeError> {
    let invalid = |message: &str| EncodeError::new(ErrorKind::InvalidOptions, message);
    if !(request.depth == 8 || request.depth == 10) {
        return Err(invalid("libavif-rs backend depth must be 8 or 10"));
    }
    if !request.quality.is_finite() || !(0.0..=100.0).contains(&request.quality) {
        return Err(invalid("libavif quality must be between 0 and 100"));
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
    frame_bytes(request).ok_or_else(|| invalid("output frame size overflows"))
}

/// An owned frame accepting each output row exactly once from top to bottom.
pub struct Frame {
    request: Config,
    rgba: Vec<u8>,
    next: u32,
}

impl Frame {
    /// Allocate the full RGBA frame after validating the request. Callers that
    /// budget pixels should reserve `required_frame_bytes` before this call.
    ///
    /// # Errors
    /// Returns an error for invalid options or failed allocation.
    pub fn new(request: Config) -> Result<Self, EncodeError> {
        required_frame_bytes(&request)?;
        let frame = request.width as usize * request.height as usize * 4;
        let mut rgba = Vec::new();
        rgba.try_reserve_exact(frame).map_err(|_| {
            EncodeError::new(
                ErrorKind::ResourceExhausted,
                "cannot allocate the RGBA frame for libavif",
            )
        })?;
        Ok(Self {
            request,
            rgba,
            next: 0,
        })
    }
    /// Append exactly one full RGBA8 row in increasing y order.
    ///
    /// # Errors
    /// Returns an error for an out-of-order row or an incorrect row length.
    pub fn write_row(&mut self, y: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        let row_bytes = 4 * self.request.width as usize;
        if y != self.next || y >= self.request.height || rgba.len() != row_bytes {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend received rows out of order or of the wrong length",
            ));
        }
        self.rgba.extend_from_slice(rgba);
        self.next += 1;
        Ok(())
    }
    /// Encode the complete frame. Codec scratch and compressed bytes are not
    /// covered by `required_frame_bytes`.
    ///
    /// # Errors
    /// Returns an error for incomplete input, failed encoding or allocation.
    pub fn finish(self) -> Result<Vec<u8>, EncodeError> {
        if self.next != self.request.height {
            return Err(EncodeError::new(
                ErrorKind::Failed,
                "libavif-rs backend finished before the last row",
            ));
        }
        encode(&self.request, self.rgba)
    }
}

/// The RGBA frame plus libavif's YUV(A) planes, which are alive together.
fn frame_bytes(request: &Config) -> Option<usize> {
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
    pixels.checked_mul(4)?.checked_add(planes)
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
impl Drop for Image {
    fn drop(&mut self) {
        // SAFETY: the pointer came from `avifImageCreate`, is not null
        // (checked at construction) and is destroyed exactly once, here.
        unsafe { image_api::avifImageDestroy(self.0) }
    }
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
/// `MAX_BANDS`, each at least `MIN_BAND_ROWS` rows. One thread keeps the
/// serial whole-image call.
fn conversion_bands(height: u32, threads: usize) -> usize {
    let by_rows = usize::try_from(height / MIN_BAND_ROWS).unwrap_or(1);
    threads.min(MAX_BANDS).min(by_rows).max(1)
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

/// Convert one band of RGBA rows into `target`'s planes.
fn convert_band(
    target: *mut image_api::avifImage,
    request: &Config,
    band: &mut [u8],
    row_bytes: u32,
) -> Result<(), EncodeError> {
    // SAFETY: an all-zero `avifRGBImage` is a valid value of the C struct
    // (integers and a null pointer); it is fully initialized just below.
    let mut rgb: reformat::avifRGBImage = unsafe { std::mem::zeroed() };
    // SAFETY: `rgb` is a valid, writable `avifRGBImage` and `target` a valid
    // image. The `avif` and `reformat` modules translate the same C struct,
    // so the pointer casts keep its layout.
    unsafe { image_api::avifRGBImageSetDefaults((&raw mut rgb).cast(), target.cast()) };
    rgb.format = AVIF_RGB_FORMAT_RGBA;
    rgb.depth = 8;
    rgb.ignoreAlpha = i32::from(!request.has_alpha);
    rgb.alphaPremultiplied = 0;
    rgb.pixels = band.as_mut_ptr();
    rgb.rowBytes = row_bytes;
    // SAFETY: `rgb.pixels` points to `rowBytes * height` initialized bytes
    // of `band` (the target's rows), which outlives the call and is not
    // otherwise accessed during it; libavif only reads the RGB image here.
    // For the whole image it allocates the planes, which `Image` frees; a
    // view's planes are already set and are written in place.
    let result = unsafe { reformat::avifImageRGBToYUV(target.cast(), &raw const rgb) };
    if result == AVIF_RESULT_OK {
        Ok(())
    } else {
        Err(failure(result, "RGB to YUV conversion"))
    }
}

/// Convert the RGBA frame into `image`'s planes in up to `bands` row bands
/// on scoped threads; one band is libavif's serial call on the whole image.
/// libavif's built-in conversion works per pixel for 4:4:4 and per 2x2
/// block for 4:2:0, so the samples are the same for every band count
/// (`tests::conversion_bands_do_not_change_samples`).
fn rgb_to_yuv(
    image: &Image,
    request: &Config,
    rgba: &mut [u8],
    row_bytes: u32,
    bands: usize,
) -> Result<(), EncodeError> {
    let rows = band_rows(request.height, bands);
    if rows.len() == 1 {
        return convert_band(image.0, request, rgba, row_bytes);
    }
    let planes = if request.has_alpha {
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
    let mut jobs = Vec::with_capacity(rows.len());
    let mut rest = rgba;
    for &(y, count) in &rows {
        let (band, tail) = rest.split_at_mut(row_bytes as usize * count as usize);
        rest = tail;
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
            width: request.width,
            height: count,
        };
        // SAFETY: `view.0` and `image.0` are distinct valid images; the
        // rectangle lies inside the image and starts on an even row
        // (`band_rows`), as 4:2:0 requires. The view only points into
        // `image`'s planes, which outlive it.
        let result = unsafe { image_api::avifImageSetViewRect(view.0, image.0, &raw const rect) };
        if result != AVIF_RESULT_OK {
            return Err(failure(result, "band view"));
        }
        jobs.push((view, band));
    }
    std::thread::scope(|scope| {
        let handles: Vec<_> = jobs
            .into_iter()
            .map(|(view, band)| {
                scope.spawn(move || {
                    // Own the whole view, so it is destroyed on this thread
                    // after the conversion.
                    let view = view;
                    convert_band(view.0, request, band, row_bytes)
                })
            })
            .collect();
        handles.into_iter().try_for_each(|handle| {
            handle.join().unwrap_or_else(|_| {
                Err(EncodeError::new(
                    ErrorKind::Failed,
                    "libavif RGB to YUV band panicked",
                ))
            })
        })
    })
}

fn encode(request: &Config, mut rgba: Vec<u8>) -> Result<Vec<u8>, EncodeError> {
    let format = match request.chroma {
        Chroma::Yuv444 => AVIF_PIXEL_FORMAT_YUV444,
        Chroma::Yuv420 => AVIF_PIXEL_FORMAT_YUV420,
    };
    let row_bytes = request
        .width
        .checked_mul(4)
        .ok_or_else(|| EncodeError::new(ErrorKind::InvalidOptions, "row size overflows"))?;
    if rgba.len() != row_bytes as usize * request.height as usize {
        return Err(EncodeError::new(
            ErrorKind::Failed,
            "libavif-rs backend frame is incomplete",
        ));
    }
    let image = new_image(request, format)?;
    let bands = conversion_bands(request.height, request.threads.clamp(1, MAX_THREADS));
    rgb_to_yuv(&image, request, &mut rgba, row_bytes, bands)?;
    drop(rgba);
    encode_image(request, &image)
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
    let quality = request.quality.round();
    #[allow(clippy::cast_possible_truncation)] // 0..=100, checked in `begin`.
    let quality = quality as i32;
    let threads = request.threads.clamp(1, MAX_THREADS);
    // SAFETY: `encoder.0` is a valid, exclusively owned `avifEncoder`; these
    // are plain fields of it.
    unsafe {
        // Select the reference encoder explicitly even if Cargo feature
        // unification enables additional libavif codecs.
        (*encoder.0).codecChoice = write::AVIF_CODEC_CHOICE_AOM;
        (*encoder.0).quality = quality;
        (*encoder.0).qualityAlpha = quality;
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
                for depth in [8, 10] {
                    for has_alpha in [false, true] {
                        let request = Config {
                            width,
                            height,
                            has_alpha,
                            quality: 75.0,
                            speed: 6,
                            threads: 1,
                            chroma,
                            depth,
                        };
                        let format = match chroma {
                            Chroma::Yuv444 => AVIF_PIXEL_FORMAT_YUV444,
                            Chroma::Yuv420 => AVIF_PIXEL_FORMAT_YUV420,
                        };
                        let convert = |bands| {
                            let image = new_image(&request, format).unwrap();
                            let mut frame = rgba.clone();
                            rgb_to_yuv(&image, &request, &mut frame, width * 4, bands).unwrap();
                            planes(&image, &request)
                        };
                        let serial = convert(1);
                        for bands in [2, 3, 8] {
                            assert!(band_rows(height, bands).len() > 1);
                            assert_eq!(
                                convert(bands),
                                serial,
                                "{width}x{height} {chroma:?} {depth}-bit alpha {has_alpha} bands {bands}"
                            );
                        }
                    }
                }
            }
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
        assert_eq!(conversion_bands(1067, 1), 1);
        assert_eq!(conversion_bands(1067, 20), 8);
        assert_eq!(conversion_bands(100, 20), 1);
        assert_eq!(conversion_bands(130, 20), 2);
    }
}
