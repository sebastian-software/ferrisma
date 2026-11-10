# Wrap the encoder translation in a safe, validated Rust API

- Status: accepted
- Created: 2026-11-08
- Updated: 2026-11-10

## Context

The translation exposes libwebp's encoder in C's shape ([ADR
0001](0001-mechanical-port.md), [ADR 0003](0003-rust-representation.md)):
`WebPConfig` with `i32` flags, `WebPPicture<'a>` with plane offsets,
`PlaneStorage` and boxed callbacks, `webp_encode` returning `bool` with the
code in `pic.error_code`, and functions that panic on inputs that are
preconditions in C (negative strides, item 4 of `# Deviations from upstream` in
`src/enc/picture_enc.rs`; short buffers index out of bounds). Ferrixel's
`webp-write` sink (WP22) needs a safe API under the adoption gates of its
ADR-0018, with buffers it owns and budgets (its ADR-0004), and Ferrixel's port
plan (Phase 6) lists what the encoder half must offer. WP17 has no upstream
unit: it adds no behavior, so every encoding must stay byte-identical to
`webp_encode` with the same `WebPConfig`, and therefore to the C oracle.

## Decision

**Module.** The API lives in `src/api/encode.rs` as `webp::api::encode`, next
to WP16's `src/api/decode.rs`. `src/api/mod.rs` declares both; `src/lib.rs`
gains `pub mod api;` under the crate's `#![deny(unsafe_code)]`;
`tools/units.py` and `tools/units.json` learn the new top-level directory and
its two non-translation modules (owners WP16 and WP17). The translation's
types and functions stay where they are, with their names; the API wraps them
and re-exports only `WebPAuxStats`. Names follow the Rust API guidelines, not
`tools/rust_name.py`.

**Public items.**

```rust
pub use crate::webp::encode::WebPAuxStats;

pub enum Preset { Default, Picture, Photo, Drawing, Icon, Text }
pub enum ImageHint { Default, Picture, Photo, Graph }
pub enum AlphaFilter { None, Fast, Best }
pub enum PixelLayout { Rgb, Rgba, Rgbx, Bgr, Bgra, Bgrx }   // bytes_per_pixel()
pub enum SampleForm { Yuva, Argb }

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config { /* WebPConfig */ }
impl Config {
    pub fn new() -> Config;                                    // also Default
    pub fn preset(preset: Preset, quality: f32) -> Result<Config, EncodeError>;
    pub fn lossless_level(self, level: u8) -> Result<Config, EncodeError>;
    // infallible, chainable setters, checked by validate():
    pub fn lossless(self, on: bool) -> Config;
    pub fn quality(self, quality: f32) -> Config;
    pub fn method(self, method: u8) -> Config;
    pub fn image_hint(self, hint: ImageHint) -> Config;
    pub fn target_size(self, bytes: u32) -> Config;
    pub fn target_psnr(self, db: f32) -> Config;
    pub fn alpha_compression(self, on: bool) -> Config;
    pub fn alpha_filter(self, filter: AlphaFilter) -> Config;
    pub fn alpha_quality(self, quality: u8) -> Config;
    pub fn exact(self, on: bool) -> Config;
    pub fn near_lossless(self, level: u8) -> Config;
    pub fn sharp_yuv(self, on: bool) -> Config;
    pub fn multithreaded(self, on: bool) -> Config;
    pub fn validate(&self) -> Result<(), EncodeError>;
    pub fn as_raw(&self) -> &WebPConfig;
    pub fn raw_mut(&mut self) -> &mut WebPConfig;              // every other field
}
impl From<WebPConfig> for Config;

pub struct PixelRows<'p> {
    pub layout: PixelLayout, pub data: &'p [u8],
    pub width: u32, pub height: u32, pub stride: usize,      // stride in bytes
}
pub struct YuvaPlanes<'a> {
    pub y: &'a mut [u8], pub y_stride: usize,
    pub u: &'a mut [u8], pub v: &'a mut [u8], pub uv_stride: usize,
    pub a: Option<&'a mut [u8]>, pub a_stride: usize,
}

#[derive(Debug)]                                              // Send + Sync
pub struct Picture<'a> { /* specifications and two PlaneStorage<'a, _> */ }
impl Picture<'static> {
    pub fn import(rows: PixelRows<'_>, form: SampleForm) -> Result<Self, EncodeError>;
}
impl<'a> Picture<'a> {
    pub fn from_argb(argb: &'a mut [u32], width: u32, height: u32, stride: usize)
        -> Result<Self, EncodeError>;                          // stride in pixels
    pub fn from_yuva(width: u32, height: u32, planes: YuvaPlanes<'a>)
        -> Result<Self, EncodeError>;
    pub fn width(&self) -> u32;
    pub fn height(&self) -> u32;
    pub fn owned_sample_bytes(&self) -> usize;
}

pub struct Encoder<'e> { /* &'e Config, optional hook and statistics */ }
impl<'e> Encoder<'e> {
    pub fn new(config: &'e Config) -> Self;
    pub fn progress(self, hook: impl FnMut(i32) -> bool + 'e) -> Self;
    pub fn stats(self, stats: &'e mut WebPAuxStats) -> Self;
    pub fn encode(self, picture: &mut Picture<'_>) -> Result<Vec<u8>, EncodeError>;
    pub fn encode_to(self, picture: &mut Picture<'_>,
                     writer: impl FnMut(&[u8]) -> bool) -> Result<(), EncodeError>;
}

pub fn encode_lossy(rows: PixelRows<'_>, quality: f32) -> Result<Vec<u8>, EncodeError>;
pub fn encode_lossless(rows: PixelRows<'_>) -> Result<Vec<u8>, EncodeError>;

#[non_exhaustive]
pub enum EncodeError {
    OutOfMemory, BitstreamOutOfMemory, NullParameter, InvalidConfiguration,
    BadDimension, Partition0Overflow, PartitionOverflow, BadWrite, FileTooBig,
    UserAbort, Unreported,
    InvalidDimensions { width: u32, height: u32 },
    InvalidStride { plane: Plane, stride: usize },
    BufferTooShort { plane: Plane, len: usize, needed: usize },
    InvalidParameter { name: &'static str },
}
impl EncodeError { pub fn code(&self) -> Option<WebPEncodingError>; }
pub enum Plane { Pixels, Y, U, V, A, Argb }
```

`EncodeError` implements `Display` and `std::error::Error`; every enum derives
`Clone`, `Copy`, `Debug`, `PartialEq`, `Eq` and `Hash`.

**Mapping to the translation.**

| API | Translation |
| --- | --- |
| `Config::new`, `Default` | `webp_config_init` (`src/webp/encode.rs`) |
| `Config::preset`, `Preset` | `webp_config_preset` with `WebPPreset::WEBP_PRESET_*` |
| `Config::lossless_level` | `webp_config_lossless_preset` (`src/enc/config_enc.rs`) |
| setters, `ImageHint`, `AlphaFilter` | the `WebPConfig` fields `lossless`, `quality`, `method`, `image_hint`, `target_size`, `target_psnr`, `alpha_compression`, `alpha_filtering` (0, 1, 2), `alpha_quality`, `exact`, `near_lossless`, `use_sharp_yuv`, `thread_level`; `target_size` saturates at `i32::MAX` |
| `Config::validate` | the API's own checks below, then `webp_validate_config` |
| `Picture::import`, `PixelLayout`, `SampleForm` | `webp_picture_init`, then `width`, `height`, `use_argb` and `webp_picture_import_{rgb,rgba,rgbx,bgr,bgra,bgrx}` (`src/enc/picture_csp_enc.rs`) |
| `Picture::from_argb` | `webp_picture_init`, `use_argb` 1, `argb_storage = PlaneStorage::External(vec![argb])` |
| `Picture::from_yuva` | `webp_picture_init`, `use_argb` 0, `colorspace` `WEBP_YUV420` or `WEBP_YUV420A`, `yuva = PlaneStorage::External` with regions Y, U, V, A |
| `Picture::owned_sample_bytes` | the bytes of the `PlaneStorage::Internal` storages |
| `Encoder::encode` | `webp_encode` with `webp_memory_write` into a `WebPMemoryWriter`, truncated to `size` |
| `Encoder::encode_to` | `webp_encode` with the caller's writer as `pic.writer` |
| `Encoder::progress`, `Encoder::stats` | `pic.progress_hook`, `pic.stats` for that call |
| `encode_lossy`, `encode_lossless` | the steps of C's `Encode` (`src/enc/picture_enc.c`): `WEBP_PRESET_DEFAULT` at `quality` (70 for lossless), `lossless`, `use_argb = lossless`, the importer, `webp_encode` into a `WebPMemoryWriter` |
| `EncodeError` | `pic.error_code` after a failed call (codes 1 to 10, C's first-error rule), or the API's validation |

**Ownership and lifetimes.** `Picture<'a>` keeps the picture's
specifications and its two storages, never callbacks, so it is `Send` and
`Sync`. `import` copies the rows into planes that the picture owns
(`Picture<'static>`); `from_argb` and `from_yuva` borrow the caller's planes
exclusively for `'a` (Ferrixel ADR-0004: the caller allocates and budgets
them). Each encoding builds a `WebPPicture` that lives for the call: owned
planes move into it and back, caller planes are reborrowed, and the writer,
hook and statistics are borrowed for the call only, so they need not outlive
`'a`. Planes that the call allocates (the ARGB-to-YUVA conversion of a lossy
encoding, the YUVA-to-ARGB conversion of a lossless one) stay with the picture
and are counted by `owned_sample_bytes`. A second encoding of the same picture
sees what the first left, as in C. If a callback panics, the panic propagates
and the picture is left without planes; a later encoding of it returns
`NullParameter`. Strides count elements of the slice's type: bytes for
`PixelRows` and `YuvaPlanes`, pixels for the `&mut [u32]` of `from_argb`.

**Validation.** The API checks before it calls the translation, so caller
input never reaches a panic:

- `width` and `height` in 1 to 16383 (`WEBP_MAX_DIMENSION`), else
  `InvalidDimensions`. C imports larger pictures and fails at
  `WebPEncode` with `VP8_ENC_ERROR_BAD_DIMENSION`; the API refuses them before
  it allocates.
- Rows: `stride >= width * bytes_per_pixel` (C requires only 3 bytes per pixel
  for the X layouts, which lets rows overlap) and `stride <= i32::MAX`, else
  `InvalidStride`; `data.len() >= (height - 1) * stride + width *
  bytes_per_pixel`, else `BufferTooShort`.
- ARGB: stride in pixels `>= width` and `<= i32::MAX`; `len >= (height - 1) *
  stride + width`.
- YUVA: `y_stride >= width`, `uv_stride >= (width + 1) / 2` (the odd-width
  case), `a_stride >= width` when `a` is given, all `<= i32::MAX`; each plane
  at least `(rows - 1) * stride + columns` samples for its rows and columns
  (chroma: `(height + 1) / 2` by `(width + 1) / 2`).
- An extent that overflows `usize` is `InvalidStride`. Negative strides and
  dimensions cannot be expressed (`u32`, `usize`), so bottom-up rows are not
  accepted; the caller reverses them.
- `Config`: `quality` finite and in 0 to 100, `target_psnr` finite (C accepts
  NaN and then converts it, which is undefined), `lossless_level` at most 9,
  each else `InvalidParameter`; then `webp_validate_config`, else
  `InvalidConfiguration`. `preset` and `lossless_level` check at once;
  `encode`, `encode_to` and the simple functions call `validate`.

**Output, progress, errors.** The writer receives the RIFF file in order, in
chunks, possibly empty ones; `false` stops the encoding with `BadWrite`, and
the bytes accepted so far stay with the writer. `Encoder::encode` uses C's
memory writer, so a refused allocation is `BadWrite` with C's growth and
`WEBP_MAX_ALLOCABLE_MEMORY` limit, and returns no bytes on failure. The hook
receives C's percentages; `false` stops with `UserAbort`, and the writer may
already hold bytes. libwebp ignores the answer at two places, the end of a
lossy analysis without segments (`ResetAllMBInfo`) and the statistics pass of
lossy methods 0 to 2 (`VP8EncLoop` drops `StatLoop`'s result): the encoding
goes on and can succeed with `USER_ABORT` recorded, which the API reports as
`Ok`, as C returns 1; a cancelling hook keeps returning `false`. Writer and hook
run on the calling thread, also with `multithreaded`, so they need not be
`Send`. Statistics are reset once the configuration and picture pass C's
validation and filled as C fills them, also on failure. Allocation failures
are `OutOfMemory` (the translation reserves fallibly); a failure that records
no code (C: `QuantizeLevels` running out of memory) is `Unreported`.
`EncodeError::code` returns the `WebPEncodingError` for codes C records and
`None` for the API's own errors.

**Semantics (Ferrixel ADR-0018, gate e).** Lossy samples are Y'CbCr 4:2:0
after ITU-R BT.601 in limited range (Y 16 to 235, Cb and Cr 16 to 240), with
the fixed-point constants of `src/dsp/yuv.h` (`VP8RGBToY`, `VP8RGBToU`,
`VP8RGBToV`) and gamma-corrected chroma averaging (alpha-weighted in blocks
with transparency). SharpYUV (`sharp_yuv`) and dithering (`preprocessing`
bit 2, through `raw_mut`) apply only when the picture holds ARGB samples at
encoding time (`import` with `SampleForm::Argb`, or `from_argb`); `import`
with `SampleForm::Yuva` converts at once without them, as in C. Planes from `from_yuva` are taken as
they are. Input alpha is straight; the encoder has no premultiplied input.
Unless `exact` is set, the encoder rewrites RGB or YUV under fully transparent
areas in the picture's planes, including the caller's (lossy: 8-by-8 blocks;
lossless: transparent pixels become 0); `show_compressed` (through `raw_mut`)
writes the decoded samples into the Y, U and V planes. WebP carries no color
signal besides an `ICCP` chunk and no orientation besides `EXIF`.

**Out of scope.** `ICCP`, `EXIF` and `XMP` chunks and animation are WP15's
muxer and animation encoder. Views, crop, rescale, distortion, alpha blending,
`extra_info` and the standalone conversions stay available in the translation
modules only.

## Consequences

Callers get typed configuration, validated input and `Result` errors without
`unsafe`, and the byte-identity of every encoding follows from calling
`webp_encode` with the stored `WebPConfig`, which WP13 proved against C. The
lending costs one small vector of slice references per call and no sample
copy. The API covers less than the C ABI; the translation remains public for
the rest.

## Review triggers

Revisit when WP22 needs a picture form or option that is missing (for example
caller-owned planes filled by libwebp's own RGB-to-YUV conversion, which needs
an importer that writes into `External` storage), when WP15 adds metadata and
animation, or when the upstream pin advances.
