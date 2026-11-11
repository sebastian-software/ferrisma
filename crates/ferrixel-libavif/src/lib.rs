//! libavif-rs as an output encoder for Ferrixel (ADR-0024).
//!
//! [`LibavifBackend`] implements Ferrixel's [`EncoderBackend`]: it collects
//! the finished straight-alpha sRGB8 rows into one RGBA frame, converts it
//! with libavif's `avifImageRGBToYUV` and encodes it with `avifEncoderWrite`
//! through libavif-rs, the mechanical Rust port of libavif with its Rust AOM
//! encoder. The output carries the same signals as Ferrixel's built-in
//! `rav1e` sink: CICP 1/13/6 (BT.709 primaries, sRGB transfer, BT.601
//! matrix), full range, 10 bits per sample by default.
//!
//! libavif-rs exposes the original C ABI, so this crate is where the
//! `unsafe` lives; Ferrixel's core keeps `#![forbid(unsafe_code)]`. Every
//! `unsafe` block states why the call is sound, and every libavif object is
//! owned by a guard that frees it on all paths.

#![deny(unsafe_op_in_unsafe_fn, clippy::undocumented_unsafe_blocks)]

use std::ffi::CStr;

use avif::{avif as image_api, rawdata, reformat, write};
use ferrixel::pipeline::{
    ChromaSubsampling, EncodeError, EncodeErrorKind, EncodeRequest, EncodedImage, EncoderBackend,
    OutputFormat, RowEncoder,
};

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
/// Ferrixel caps encoder threads at 256 as well.
const MAX_THREADS: usize = 256;

/// libavif-rs with its Rust AOM encoder.
///
/// `quality` (0–100) and `speed` (0–10, libavif's `AVIF_SPEED_*` scale) are
/// libavif's own settings, not `rav1e`'s: compare the two backends at
/// matched quality. `threads` sets `maxThreads`; any value other than 1
/// also enables libavif's automatic tiling, mirroring Ferrixel's `rav1e`
/// sink, which encodes one tile only on one thread.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LibavifBackend {
    /// Bits per sample of the AV1 image: 8 or 10.
    pub depth: u8,
}

impl Default for LibavifBackend {
    /// 10 bits per sample, as Ferrixel's `rav1e` sink writes.
    fn default() -> Self {
        Self { depth: 10 }
    }
}

impl EncoderBackend for LibavifBackend {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }
    fn name(&self) -> &'static str {
        "libavif-rs/aom"
    }

    fn begin(&self, request: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        let invalid = |message: &str| EncodeError::new(EncodeErrorKind::InvalidOptions, message);
        if !(self.depth == 8 || self.depth == 10) {
            return Err(invalid("libavif-rs backend depth must be 8 or 10"));
        }
        if !request.quality.is_finite() || !(0.0..=100.0).contains(&request.quality) {
            return Err(invalid("libavif quality must be between 0 and 100"));
        }
        if request.speed > MAX_SPEED {
            return Err(invalid("libavif speed must be between 0 and 10"));
        }
        let frame = usize::try_from(request.width)
            .ok()
            .zip(usize::try_from(request.height).ok())
            .and_then(|(width, height)| width.checked_mul(height))
            .and_then(|pixels| pixels.checked_mul(4))
            .ok_or_else(|| invalid("output frame size overflows"))?;
        let mut rgba = Vec::new();
        rgba.try_reserve_exact(frame).map_err(|_| {
            EncodeError::new(
                EncodeErrorKind::ResourceExhausted,
                "cannot allocate the RGBA frame for libavif",
            )
        })?;
        Ok(Box::new(Frame {
            request: *request,
            depth: self.depth,
            rgba,
            next: 0,
        }))
    }
}

/// One image: rows collected into a full RGBA frame, encoded at the end.
struct Frame {
    request: EncodeRequest,
    depth: u8,
    rgba: Vec<u8>,
    next: u32,
}

impl RowEncoder for Frame {
    fn write_row(&mut self, y: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        let row_bytes = 4 * self.request.width as usize;
        if y != self.next || y >= self.request.height || rgba.len() != row_bytes {
            return Err(EncodeError::new(
                EncodeErrorKind::Failed,
                "libavif-rs backend received rows out of order or of the wrong length",
            ));
        }
        self.rgba.extend_from_slice(rgba);
        self.next += 1;
        Ok(())
    }

    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        if self.next != self.request.height {
            return Err(EncodeError::new(
                EncodeErrorKind::Failed,
                "libavif-rs backend finished before the last row",
            ));
        }
        let bytes = encode(&self.request, self.depth, self.rgba)?;
        Ok(EncodedImage::new(
            OutputFormat::Avif,
            bytes,
            frame_bytes(&self.request, self.depth),
            true,
        ))
    }
}

/// The RGBA frame plus libavif's YUV(A) planes, which are alive together.
fn frame_bytes(request: &EncodeRequest, depth: u8) -> usize {
    let pixels = request.width as usize * request.height as usize;
    let sample = if depth > 8 { 2 } else { 1 };
    let chroma = match request.chroma {
        ChromaSubsampling::Yuv444 => 2 * pixels,
        ChromaSubsampling::Yuv420 => {
            2 * (request.width as usize).div_ceil(2) * (request.height as usize).div_ceil(2)
        }
    };
    let alpha = if request.has_alpha { pixels } else { 0 };
    4 * pixels + sample * (pixels + chroma + alpha)
}

fn failure(result: u32, during: &str) -> EncodeError {
    // SAFETY: `avifResultToString` returns a pointer to a static,
    // NUL-terminated string for every value, including unknown ones.
    let text = unsafe { CStr::from_ptr(image_api::avifResultToString(result)) }
        .to_string_lossy()
        .into_owned();
    let kind = match result {
        AVIF_RESULT_OUT_OF_MEMORY => EncodeErrorKind::ResourceExhausted,
        AVIF_RESULT_INVALID_ARGUMENT => EncodeErrorKind::InvalidOptions,
        _ => EncodeErrorKind::Failed,
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

fn encode(request: &EncodeRequest, depth: u8, mut rgba: Vec<u8>) -> Result<Vec<u8>, EncodeError> {
    let format = match request.chroma {
        ChromaSubsampling::Yuv444 => AVIF_PIXEL_FORMAT_YUV444,
        ChromaSubsampling::Yuv420 => AVIF_PIXEL_FORMAT_YUV420,
    };
    let row_bytes = request
        .width
        .checked_mul(4)
        .ok_or_else(|| EncodeError::new(EncodeErrorKind::InvalidOptions, "row size overflows"))?;
    if rgba.len() != row_bytes as usize * request.height as usize {
        return Err(EncodeError::new(
            EncodeErrorKind::Failed,
            "libavif-rs backend frame is incomplete",
        ));
    }
    // SAFETY: plain arguments; a null result is handled below.
    let image = unsafe {
        image_api::avifImageCreate(request.width, request.height, u32::from(depth), format)
    };
    if image.is_null() {
        return Err(EncodeError::new(
            EncodeErrorKind::ResourceExhausted,
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

    // SAFETY: an all-zero `avifRGBImage` is a valid value of the C struct
    // (integers and a null pointer); it is fully initialized just below.
    let mut rgb: reformat::avifRGBImage = unsafe { std::mem::zeroed() };
    // SAFETY: `rgb` is a valid, writable `avifRGBImage` and `image.0` a valid
    // image. The `avif` and `reformat` modules translate the same C struct,
    // so the pointer casts keep its layout.
    unsafe { image_api::avifRGBImageSetDefaults((&raw mut rgb).cast(), image.0.cast()) };
    rgb.format = AVIF_RGB_FORMAT_RGBA;
    rgb.depth = 8;
    rgb.ignoreAlpha = i32::from(!request.has_alpha);
    rgb.alphaPremultiplied = 0;
    rgb.pixels = rgba.as_mut_ptr();
    rgb.rowBytes = row_bytes;
    // SAFETY: `rgb.pixels` points to `rowBytes * height` initialized bytes
    // in `rgba`, which outlives the call and is not otherwise accessed
    // during it; libavif only reads the RGB image here. It allocates the
    // image's planes, which `Image` frees.
    let result = unsafe { reformat::avifImageRGBToYUV(image.0.cast(), &raw const rgb) };
    if result != AVIF_RESULT_OK {
        return Err(failure(result, "RGB to YUV conversion"));
    }
    drop(rgba);

    // SAFETY: no arguments; a null result is handled below.
    let encoder = unsafe { write::avifEncoderCreate() };
    if encoder.is_null() {
        return Err(EncodeError::new(
            EncodeErrorKind::ResourceExhausted,
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
    if output.0.data.is_null() {
        return Err(EncodeError::new(
            EncodeErrorKind::Failed,
            "libavif returned no output",
        ));
    }
    // SAFETY: on success libavif sets `data` to an allocation of `size`
    // initialized bytes, owned by `output` until it is dropped below.
    let bytes = unsafe { std::slice::from_raw_parts(output.0.data, output.0.size) }.to_vec();
    Ok(bytes)
}
