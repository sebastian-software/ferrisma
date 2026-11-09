# Wrap the decoder translation in a safe, validated Rust API

- Status: accepted
- Created: 2026-11-09
- Updated: 2026-11-09

## Context

The translation exposes libwebp's decoder in C's shape ([ADR 0001](0001-mechanical-port.md),
[ADR 0003](0003-rust-representation.md)): `webp_get_features` filling a
`WebPBitstreamFeatures` of `i32` fields, `webp_decode` with a
`WebPDecoderConfig<'a>` whose `WebPDecBuffer<'a>` describes planes by offsets,
strides and sizes over a `PlaneStorage`, the incremental decoder
`WebPIDecoder<'a, 'b>` that borrows its configuration, `VP8StatusCode` results
in which `VP8_STATUS_SUSPENDED` is the normal state of an incremental
decoding, the demuxer `WebPDemuxer<'a>` and the animation decoder
`WebPAnimDecoder<'a>`, both of which borrow a `WebPData` value. Functions panic
where C has a precondition: a buffer description that does not match its
slices, or a flipped plane outside its region. Ferrixel's `webp-read` source
(WP21) needs a safe API under its ADR-0012 contract (describe the source, then
stream rows of a region into memory the engine budgets) and its ADR-0018
adoption gates, and the port plan (Phase 6) lists what the decoder half must
offer. WP16 has no upstream unit: it adds no behavior, so every decoding must
equal `webp_decode` (or the incremental decoder, or the animation decoder)
with the same configuration, and therefore the C oracle. [ADR 0006](0006-safe-encoder-api.md)
is the model.

## Decision

**Module.** The API lives in `src/api/decode.rs` as `webp::api::decode`, which
`src/api/mod.rs` already declares. The translation's types and functions stay
where they are; the API wraps them and re-exports none (each translation type
it would expose carries C's integer fields or padding). Names follow the Rust
API guidelines, not `tools/rust_name.py`.

**Public items.**

```rust
pub enum Format { Lossy, Lossless }
pub struct Features {                                          // Copy, Eq
    pub width: u32, pub height: u32,
    pub has_alpha: bool, pub has_animation: bool,
    pub format: Option<Format>,                                // None: an animation
}
impl Features { pub fn parse(data: &[u8]) -> Result<Features, DecodeError>; }

pub enum ColorMode {
    Rgb, Rgba, Bgr, Bgra, Argb, Rgba4444, Rgb565,
    PremultipliedRgba, PremultipliedBgra, PremultipliedArgb, PremultipliedRgba4444,
    Yuv, Yuva,
}
impl ColorMode {
    pub const fn bytes_per_pixel(self) -> Option<usize>;      // None for Yuv, Yuva
    pub const fn has_alpha(self) -> bool;
    pub const fn is_premultiplied(self) -> bool;
    pub const fn is_yuv(self) -> bool;
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config { /* WEBP_CSP_MODE and WebPDecoderOptions */ }
impl Config {
    pub fn new(mode: ColorMode) -> Config;                     // Default: Rgb, as C
    // infallible, chainable setters, checked by validate():
    pub fn crop(self, left: u32, top: u32, width: u32, height: u32) -> Config;
    pub fn scale(self, width: u32, height: u32) -> Config;
    pub fn flip(self, on: bool) -> Config;
    pub fn bypass_filtering(self, on: bool) -> Config;
    pub fn fancy_upsampling(self, on: bool) -> Config;         // on by default
    pub fn multithreaded(self, on: bool) -> Config;
    pub fn dithering(self, strength: u8) -> Config;            // 0 to 100
    pub fn alpha_dithering(self, strength: u8) -> Config;      // 0 to 100
    pub fn mode(&self) -> ColorMode;
    pub fn validate(&self) -> Result<(), DecodeError>;
    pub fn output_size(&self, features: &Features) -> Result<(u32, u32), DecodeError>;
    pub fn as_raw(&self) -> &WebPDecoderOptions;
}

pub struct YuvaPlanes<'a> {                                   // as encode::YuvaPlanes
    pub y: &'a mut [u8], pub y_stride: usize,
    pub u: &'a mut [u8], pub v: &'a mut [u8], pub uv_stride: usize,
    pub a: Option<&'a mut [u8]>, pub a_stride: usize,
}
pub enum Output<'a> {
    Rows { data: &'a mut [u8], stride: usize },               // the RGB modes
    Planes(YuvaPlanes<'a>),                                   // Yuv, Yuva
}

pub struct Image { /* mode, size, the library memory and its plane layout */ }
impl Image {
    pub fn mode(&self) -> ColorMode;
    pub fn width(&self) -> u32;
    pub fn height(&self) -> u32;
    pub fn rows(&self) -> Option<Rows<'_>>;                   // the RGB modes
    pub fn planes(&self) -> Option<YuvaRows<'_>>;             // Yuv, Yuva
    pub fn samples(&self) -> &[u8];
    pub fn into_samples(self) -> Vec<u8>;
}
pub struct Rows<'s> { /* region, first row, signed stride, row length, count */ }
impl<'s> Rows<'s> {
    pub fn len(&self) -> u32;
    pub fn is_empty(&self) -> bool;
    pub fn row(&self, index: u32) -> Option<&'s [u8]>;
    pub fn iter(&self) -> impl Iterator<Item = &'s [u8]>;
    pub fn row_len(&self) -> usize;
    pub fn stride(&self) -> isize;
}
pub struct YuvaRows<'s> { pub y: Rows<'s>, pub u: Rows<'s>, pub v: Rows<'s>, pub a: Option<Rows<'s>> }

pub fn decode(data: &[u8], config: &Config) -> Result<Image, DecodeError>;
pub fn decode_into(data: &[u8], config: &Config, output: Output<'_>)
    -> Result<(u32, u32), DecodeError>;

pub enum Status { NeedsMoreData, Complete }
pub struct IncrementalDecoder<'a> { /* WebPIDecoder<'a, 'a> */ }
impl<'a> IncrementalDecoder<'a> {
    pub fn new(config: &'a Config) -> Result<Self, DecodeError>;
    pub fn with_output(config: &'a Config, output: Output<'a>) -> Result<Self, DecodeError>;
    pub fn append(&mut self, data: &[u8]) -> Result<Status, DecodeError>;
    pub fn update(&mut self, data: &[u8]) -> Result<Status, DecodeError>;
    pub fn output_size(&self) -> Option<(u32, u32)>;
    pub fn decoded_rows(&self) -> u32;
    pub fn rows(&self) -> Option<Rows<'_>>;
    pub fn planes(&self) -> Option<YuvaRows<'_>>;
    pub fn into_image(self) -> Result<Image, DecodeError>;
}

pub struct Container<'d> { /* WebPDemuxer<'d> */ }
impl<'d> Container<'d> {
    pub fn new(data: &'d [u8]) -> Result<Self, DecodeError>;
    pub fn partial(data: &'d [u8]) -> Result<Self, DecodeError>;
    pub fn is_complete(&self) -> bool;
    pub fn canvas_width(&self) -> u32;
    pub fn canvas_height(&self) -> u32;
    pub fn has_alpha(&self) -> bool;
    pub fn has_animation(&self) -> bool;
    pub fn loop_count(&self) -> u32;
    pub fn background_color(&self) -> u32;                     // 0xAARRGGBB
    pub fn frame_count(&self) -> u32;
    pub fn icc_profile(&self) -> Option<&[u8]>;
    pub fn exif(&self) -> Option<&[u8]>;
    pub fn xmp(&self) -> Option<&[u8]>;
    pub fn chunks(&self, fourcc: [u8; 4]) -> Chunks<'_>;
}
pub struct Chunks<'s> { /* WebPChunkIterator<'s> */ }       // Iterator<Item = &'s [u8]>

pub enum AnimationMode { Rgba, Bgra, PremultipliedRgba, PremultipliedBgra }
pub struct AnimationOptions { pub mode: AnimationMode, pub multithreaded: bool } // Default
pub struct AnimationInfo {
    pub canvas_width: u32, pub canvas_height: u32,
    pub loop_count: u32, pub background_color: u32,           // 0xAARRGGBB
    pub frame_count: u32,
}
pub struct Frame<'s> { pub canvas: &'s [u8], pub stride: usize, pub timestamp_ms: i32 }
pub struct AnimationDecoder<'d> { /* WebPAnimDecoder<'d> */ }
impl<'d> AnimationDecoder<'d> {
    pub fn new(data: &'d [u8], options: AnimationOptions) -> Result<Self, DecodeError>;
    pub fn info(&self) -> AnimationInfo;
    pub fn has_more_frames(&self) -> bool;
    pub fn next_frame(&mut self) -> Result<Option<Frame<'_>>, DecodeError>;
    pub fn reset(&mut self);
}

#[non_exhaustive]
pub enum DecodeError {
    OutOfMemory, InvalidParam, BitstreamError, UnsupportedFeature, Suspended,
    UserAbort, NotEnoughData,
    FrameFailed,
    InvalidCrop { left: u32, top: u32, width: u32, height: u32 },
    InvalidScale { width: u32, height: u32 },
    InvalidStride { plane: Plane, stride: usize },
    BufferTooShort { plane: Plane, len: usize, needed: usize },
    OutputMismatch { mode: ColorMode },
    InvalidParameter { name: &'static str },
}
impl DecodeError { pub const fn code(&self) -> Option<VP8StatusCode>; }
pub enum Plane { Rows, Y, U, V, A }
```

`DecodeError` implements `Display` and `std::error::Error`; every enum derives
`Clone`, `Copy`, `Debug`, `PartialEq`, `Eq` and `Hash`.

**Mapping to the translation.**

| API | Translation |
| --- | --- |
| `Features::parse`, `Format` | `webp_get_features` (`src/webp/decode.rs`); `format` 1 and 2 are `Some(Lossy)` and `Some(Lossless)`; 0 is `None`, which a successful parse reports only for an animation (`ParseHeadersInternal` returns at the animation flag, and a prefix that ends before the image chunk is `NOT_ENOUGH_DATA`), whose frames carry their own format |
| `ColorMode` | `WEBP_CSP_MODE::MODE_RGB` to `MODE_YUVA` in C's order; `bytes_per_pixel` is `kModeBpp` |
| `Config::new`, setters | `webp_init_decoder_config`, then the `WebPDecoderOptions` fields `use_cropping`, `crop_*`, `use_scaling`, `scaled_*`, `flip`, `bypass_filtering`, `no_fancy_upsampling` (negated), `use_threads`, `dithering_strength`, `alpha_dithering_strength`, and `output.colorspace` |
| `Config::validate` | the API's checks below, then `webp_validate_decoder_config` without features |
| `Config::output_size` | `webp_check_crop_dimensions` (`src/dec/webp_dec.rs`) against the features, then the scaled size |
| `decode` | `webp_decode` with `PlaneStorage::Internal` output; the `Image` takes the buffer's memory and plane layout |
| `decode_into`, `Output`, `YuvaPlanes` | `webp_decode` with `PlaneStorage::External` (one region for `Rows`, Y, U, V and A regions for `Planes`), each `size` field equal to its slice's length, `slow_memory` false |
| `IncrementalDecoder::new`, `with_output` | `webp_i_new_rgb` (RGB modes) or `webp_i_new_yuva` (`Planes`) or `webp_i_new_decoder(None)` with the colorspace set on its own buffer (YUV in library memory), then `set_options`, a crate-visible accessor of `src/dec/idec_dec.rs` that does what `WebPIDecode` does (`idec->params.options = &config->options`) |
| `append`, `update`, `Status` | `webp_i_append`, `webp_i_update`; `Ok` is `Complete`, `VP8_STATUS_SUSPENDED` is `NeedsMoreData`; an empty `append` before any data is answered as `NeedsMoreData` without the call (libwebp asserts against it) |
| `output_size`, `decoded_rows`, `rows`, `planes` | `webp_i_decoded_area` (the size, `None` while it is 0, and `last_y`), `webp_i_dec_get_rgb`, `webp_i_dec_get_yuva` (the planes and strides) |
| `into_image` | the decoder's own buffer after `STATE_DONE` |
| `Container::new`, `partial` | `webp_demux_internal` with `allow_partial` false or true (`webp_demux`, `webp_demux_partial`), through `demux_bytes`, a crate-visible form of it in `src/demux/demux.rs` that takes the bytes instead of a `WebPData` |
| `Container` accessors | `webp_demux_get_i` with `WEBP_FF_CANVAS_WIDTH`, `_HEIGHT`, `_LOOP_COUNT`, `_BACKGROUND_COLOR`, `_FRAME_COUNT`; `has_alpha` and `has_animation` are the `ALPHA_FLAG` and `ANIMATION_FLAG` bits of `WEBP_FF_FORMAT_FLAGS` (the API does not expose C's bit set; `ICCP`, `EXIF` and `XMP ` presence is their accessors'); `webp_demux_get_chunk(…, 1)` for `ICCP`, `EXIF`, `XMP `; `chunks` walks `webp_demux_get_chunk` and `webp_demux_next_chunk` |
| `AnimationDecoder::new`, `AnimationMode`, `AnimationOptions` | `webp_anim_decoder_new` through `anim_decoder_new_bytes`, a crate-visible form of `webp_anim_decoder_new_internal` in `src/demux/anim_decode.rs` that takes the bytes, with `color_mode` `MODE_RGBA`, `MODE_BGRA`, `MODE_rgbA`, `MODE_bgrA` and `use_threads` |
| `info`, `has_more_frames`, `next_frame`, `reset` | `webp_anim_decoder_get_info`, `webp_anim_decoder_has_more_frames`, `webp_anim_decoder_get_next`, `webp_anim_decoder_reset` |
| `DecodeError` | `VP8StatusCode` 1 to 7 (`code()` maps back), `FrameFailed` for a failed `get_next`, and the API's validation |

**Interfaces of merged modules.** Three crate-visible items, each its own
commit with no behavior change, make the borrows above expressible in safe
Rust: `WebPIDecoder::set_options` in `src/dec/idec_dec.rs` (WP12), because the
translation sets the options only through `webp_i_decode`, which also borrows
the configuration's output buffer, so a decoder that owns its buffer could not
take options; `demux_bytes` in `src/demux/demux.rs` (WP8) and
`anim_decoder_new_bytes` in `src/demux/anim_decode.rs` (WP14), because the
demuxer borrows the `WebPData` value rather than its bytes, so an object built
from `&'d [u8]` inside the API could not outlive the call. The translated
functions call the new forms with `&data.bytes` and keep their `// upstream:`
markers.

**Ownership and lifetimes.** `Config`, `Features`, `Image` and the errors own
their data and are `Send` and `Sync`. `decode` returns an `Image` that owns
the library memory (`samples`, `into_samples` hand it over without a copy).
`decode_into` borrows the caller's slices for the call: Ferrixel's budgeted
rows or planes (its ADR-0004). `IncrementalDecoder<'a>` borrows the `Config`
shared and the caller's slices exclusively for its lifetime; the rows it lends
(`rows`, `planes`) borrow the decoder, so they end before the next `append`.
`Container<'d>` and `AnimationDecoder<'d>` borrow the caller's bytes; chunk
payloads borrow the container, and a frame's canvas borrows the animation
decoder until the next call. `IncrementalDecoder` is neither `Send` nor `Sync`,
because the translation's decoder holds a `dyn VP8IoHooks` box.

**Validation.** The API checks before it calls the translation, so caller
input never reaches a panic:

- `Config::validate`: a crop with `width` and `height` at least 1, `left` and
  `top` even, each value at most `i32::MAX`, else `InvalidCrop` (libwebp moves
  an odd crop of a lossy image to the even position, as `WebPIoInitFromOptions`
  snaps when the source is YUV, and crops a lossless image exactly; the API
  has one rule for both, and a caller crops one column or row earlier and
  skips it); a scale with both sides in 1 to 16383, else `InvalidScale`
  (libwebp derives a side given as 0 and accepts larger outputs);
  dithering strengths at most 100, else `InvalidParameter` (`WebPDecode`
  clamps them; `WebPValidateDecoderConfig` refuses them). Every decoder
  entry point calls it first.
- `Config::output_size`: the crop inside the picture
  (`left + width <= picture width`, the same for the height), else
  `InvalidCrop`; the output is the crop's size, then the scale's.
- `decode`, `decode_into`: after `Config::validate`, `Features::parse`; its
  error is the one `webp_decode` returns (`NotEnoughData` becomes `BitstreamError`, as in C); an
  animation is `UnsupportedFeature`, as libwebp reports it, before the
  configuration is checked against the picture; then `output_size`; then the
  output: `Rows` only for the RGB modes and `Planes` only for `Yuv` and
  `Yuva`, with `a` given exactly for `Yuva`, else `OutputMismatch`; strides at
  least a row (`width * bytes_per_pixel`, `width`, `(width + 1) / 2`) and at
  most `i32::MAX`, else `InvalidStride`; each slice at least
  `(rows - 1) * stride + row length` bytes, else `BufferTooShort`.
- `IncrementalDecoder::append`: an empty piece before any data returns
  `NeedsMoreData` without calling the decoder, because libwebp asserts that
  data was stored before (`WebPIAppend` of zero bytes first); release builds
  of C and of the translation return `SUSPENDED` there.
- `IncrementalDecoder::with_output`: the picture is unknown until its header
  arrives, so the API checks what does not depend on it (the plane matching
  above, strides at least 1 and at most `i32::MAX`, slices not empty) and
  libwebp checks the rest at the header, returning `InvalidParam`, as C does.
  A caller that knows the features checks first with `output_size` and the
  formulas above.
- Every stride counts bytes. Negative strides and offsets cannot be expressed
  (`u32`, `usize`); a flipped output is `flip(true)`, not a negative stride.

**Errors and status.** `DecodeError` has one variant per `VP8StatusCode` from
1 to 7, so `code()` returns C's code for those and `None` for the API's own.
The incremental decoder reports `VP8_STATUS_SUSPENDED` as
`Ok(Status::NeedsMoreData)`; `Suspended` is the error of `into_image` before
the decoding has completed. `Features::parse` returns `NotEnoughData` for a
prefix that is too short, which is the signal to read more. A failed
`next_frame` is `FrameFailed`, because libwebp's `WebPAnimDecoderGetNext`
returns no status; `AnimationDecoder::new` and `Container::new` report the
error of `Features::parse` first (libwebp's demuxer reports a file that is not
WebP as a header in progress), then a container that does not parse
(`NotEnoughData` when `webp_demux_partial` says the data ends early,
`BitstreamError` otherwise), and `AnimationDecoder::new` otherwise
`OutOfMemory`, the only other way `WebPAnimDecoderNew` fails with a valid
mode. A whole-image decoding of a truncated file returns libwebp's status:
`BitstreamError` before the dimensions (`WebPDecode` maps the features'
`NOT_ENOUGH_DATA`), `NotEnoughData` or `BitstreamError` later. `UserAbort`
is never returned (the API sets no I/O hooks) and is kept for `code()`.

**Semantics (Ferrixel ADR-0018, gate e).**

- **Lossy color.** `VP8 ` frames hold Y'CbCr 4:2:0 after ITU-R BT.601 in
  limited range (Y 16 to 235, Cb and Cr 16 to 240). The RGB modes convert with
  the fixed-point matrix of `src/dsp/yuv.h` (`VP8YUVToR`, `VP8YUVToG`,
  `VP8YUVToB`), with fancy upsampling of the chroma unless
  `fancy_upsampling(false)` or a scale (which upsamples in the rescaler); the
  in-loop filter runs unless `bypass_filtering` or a scale below 3/4 in both
  directions. `Yuv` and `Yuva` return the decoded planes as stored, so a caller
  of those modes converts with the same limited-range matrix itself.
  `dithering` adds libwebp's dither to lossy output.
- **Lossless color.** `VP8L` stores ARGB exactly; the RGB modes reorder its
  bytes (the 16-bit modes keep the high bits), and `Yuv`/`Yuva` convert it with `WebPConvertARGBToY`
  and `WebPConvertARGBToUV` (the same limited-range matrix, chroma from 2 by 2
  averages).
- **Alpha.** Straight in `Rgba`, `Bgra`, `Argb`, `Rgba4444` and the `a` plane
  of `Yuva`; premultiplied in the four `Premultiplied` modes (libwebp's
  `WebPApplyAlphaMultiply`). The alpha of a lossy image comes from its `ALPH`
  chunk; `alpha_dithering` smooths quantized alpha. Modes without alpha drop
  it; `Rgb565` and the opaque RGB modes do not blend.
- **16-bit modes.** `Rgba4444` and `Rgb565` pack 4 or 5/6 bits per channel in
  two bytes, red first (`WEBP_SWAP_16BIT_CSP` off, the reference
  configuration). Every mode has 8-bit precision at most.
- **Color signal and orientation.** WebP carries no color signal besides an
  `ICCP` chunk and no orientation besides an `EXIF` chunk. The API returns the
  raw chunk payloads and parses neither; RGB values are as stored, so readers
  assume sRGB without a profile.
- **Animation.** Each `next_frame` returns the whole canvas after blending and
  disposal, in the chosen mode (premultiplied modes blend premultiplied
  pixels); the canvas starts transparent black and `background_color` is
  reported, not applied, as in libwebp, as `0xAARRGGBB` (the `ANIM` chunk's
  bytes read as a little-endian `u32`, the `u32` pixel convention of ADR 0006);
  timestamps are the end times of the frames in milliseconds. A still image is
  one frame with timestamp 0, loop count 1 and background `0xFFFFFFFF`.

**Cropping.** The crop is libwebp's (`use_cropping` and the four values) and
saves what WP12 measured (`docs/status/wp12.md`, "Cropping measurement for
Ferrixel"): for `VP8 ` and `VP8L` images, `decode` and `decode_into` decode
only the rows down to the crop's bottom (plus 2 or 8 rows of filter margin for
lossy), never fewer rows above it, and nothing is saved by a horizontal crop;
the incremental decoder parses every macroblock row of a lossy image whatever
the crop; scaling runs after decoding at full resolution. The API promises
these facts as they are measured, not a cost model.

**Out of scope.** Encoding is WP17's (ADR 0006); editing and writing
containers and animations are WP15's. Not in the API: I/O hooks
(`WebPISetIOHooks`), slow external memory (`slow_memory`, which only delays
the writes into caller memory for premultiplied output and costs a temporary
buffer of the whole image), `WebPCopyDecBuffer`
and `WebPGrabDecBuffer`, the frame iterators of the demuxer, and the simple
functions (`WebPDecodeRGBA` and friends are `decode` with `Config::new(mode)`
and default options, which libwebp decodes identically).

## Consequences

Callers get typed configuration, validated buffers and `Result` errors
without `unsafe`, and each output follows from calling the verified
translation with the stored options. The incremental decoder and the views
borrow rather than own what libwebp borrows, so their lifetimes are visible in
the types. The API covers less than the C ABI; the translation remains public
for the rest.

## Review triggers

Revisit when WP21 needs a form or option that is missing (slow memory, a
`Send` incremental decoder, a derived scale side, odd crop offsets for
lossless images, a memory limit for animation canvases), when WP15 changes the
demuxer, or when the upstream pin advances.
