//! Safe Rust API over the decoder of the translation; no upstream unit.
//!
//! Wraps the translated types and functions in validated Rust types and adds
//! no codec behavior of its own; see `docs/adr/`.
//!
//! Work package: WP16 (safe decoder API).
//!
//! The decision record is `docs/adr/0007-safe-decoder-api.md`. Every decoding
//! calls the translation with the stored options: [`decode`] and
//! [`decode_into`] call the translated `WebPDecode` ([`webp_decode`]),
//! [`IncrementalDecoder`] the translated incremental decoder and
//! [`AnimationDecoder`] the translated `WebPAnimDecoder`, so the samples are
//! those of libwebp v1.6.0 with the same options. The API validates crops,
//! scales, strides and buffer lengths first, so that caller input never
//! reaches a panic of the translation, and reports failures as
//! [`DecodeError`].
//!
//! # Example
//!
//! ```
//! use webp::api::decode::{decode, ColorMode, Config, Features, Format};
//! # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
//! # let rgba = [200_u8, 100, 50, 255].repeat(6 * 4);
//! # let rows = PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 6, height: 4, stride: 24 };
//! # let file = encode_lossless(rows)?;
//!
//! // `file` holds a lossless 6 by 4 WebP image.
//! let features = Features::parse(&file)?;
//! assert_eq!((features.width, features.height, features.format), (6, 4, Some(Format::Lossless)));
//!
//! let config = Config::new(ColorMode::Rgba).crop(2, 0, 4, 4);
//! let image = decode(&file, &config)?;
//! let rows = image.rows().expect("an RGB mode");
//! assert_eq!((image.width(), image.height(), rows.len()), (4, 4, 4));
//! assert_eq!(rows.row(0), Some(&[200_u8, 100, 50, 255].repeat(4)[..]));
//! # Ok::<(), Box<dyn std::error::Error>>(())
//! ```
//!
//! # Memory
//!
//! [`decode`] returns an [`Image`] in memory that libwebp allocates; the image
//! owns it and hands it over without a copy ([`Image::into_samples`]).
//! [`decode_into`] and [`IncrementalDecoder::with_output`] write into the
//! caller's rows or planes ([`Output`]), so that the caller can budget that
//! memory; every stride counts bytes. [`Config::output_size`] gives the size
//! of the output before decoding. [`AnimationDecoder::new`] allocates two
//! canvases of `4 * canvas_width * canvas_height` bytes and zeroes them.
//!
//! # Semantics
//!
//! - **Lossy color.** A `VP8 ` bitstream holds `Y'CbCr` 4:2:0 after ITU-R BT.601
//!   in limited range (Y 16 to 235, Cb and Cr 16 to 240). The RGB modes convert
//!   with the fixed-point matrix of libwebp's `src/dsp/yuv.h` (`VP8YUVToR`,
//!   `VP8YUVToG`, `VP8YUVToB`), with fancy upsampling of the chroma unless
//!   [`Config::fancy_upsampling`] is off or the output is scaled (the rescaler
//!   then upsamples). The in-loop filter runs unless
//!   [`Config::bypass_filtering`] is set or the output is scaled below 3/4 in
//!   both directions. [`ColorMode::Yuv`] and [`ColorMode::Yuva`] return the
//!   decoded planes as stored, so their caller converts with the same
//!   limited-range matrix. [`Config::dithering`] adds libwebp's dither to
//!   lossy output.
//! - **Lossless color.** A `VP8L` bitstream stores ARGB exactly; the RGB modes
//!   reorder its bytes (the 16-bit modes keep the high bits), and the YUV
//!   modes convert it with `WebPConvertARGBToY` and `WebPConvertARGBToUV` (the
//!   same limited-range matrix, chroma from 2 by 2 averages).
//! - **Alpha** is straight in [`ColorMode::Rgba`], [`ColorMode::Bgra`],
//!   [`ColorMode::Argb`], [`ColorMode::Rgba4444`] and the alpha plane of
//!   [`ColorMode::Yuva`], and premultiplied in the four `Premultiplied` modes.
//!   The alpha of a lossy image comes from its `ALPH` chunk;
//!   [`Config::alpha_dithering`] smooths quantized alpha. Modes without alpha
//!   drop it.
//! - **Precision.** Every mode has 8 bits per channel at most;
//!   [`ColorMode::Rgba4444`] and [`ColorMode::Rgb565`] pack 4 or 5 and 6 bits
//!   per channel into two bytes, red first.
//! - **Color signal and orientation.** WebP carries no color signal besides an
//!   `ICCP` chunk and no orientation besides an `EXIF` chunk. [`Container`]
//!   returns the raw payloads and parses neither; without a profile, readers
//!   assume sRGB.
//!
//! # Cropping
//!
//! A crop decodes the rows from the top of the image down to the crop's
//! bottom (with 2 or 8 rows of filter margin for lossy images), so it saves
//! the rows below the crop and nothing above it; a horizontal crop saves
//! nothing, and the incremental decoder parses every macroblock row of a lossy
//! image whatever the crop. Scaling runs after decoding at full resolution.
//! `docs/status/wp12.md` ("Cropping measurement for Ferrixel") has the counts.

use std::borrow::Cow;
use std::fmt;

use crate::dec::idec_dec::{
    WebPIDecoder, webp_i_append, webp_i_dec_get_rgb, webp_i_dec_get_yuva, webp_i_decoded_area,
    webp_i_new_decoder, webp_i_new_rgb, webp_i_new_yuva, webp_i_update,
};
use crate::dec::webp_dec::{webp_check_crop_dimensions, webp_decode, webp_validate_decoder_config};
use crate::demux::anim_decode::{
    WebPAnimDecoder, anim_decoder_new_bytes, webp_anim_decoder_get_info,
    webp_anim_decoder_get_next, webp_anim_decoder_has_more_frames, webp_anim_decoder_reset,
};
use crate::demux::demux::{
    WebPDemuxer, demux_bytes, webp_demux_get_chunk, webp_demux_get_i, webp_demux_next_chunk,
};
use crate::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPBitstreamFeatures, WebPDecBuffer, WebPDecBufferPlanes,
    WebPDecoderConfig, WebPDecoderOptions, WebPRGBABuffer, WebPYUVABuffer, webp_get_features,
    webp_init_decoder_config,
};
use crate::webp::demux::{
    WEBP_DEMUX_ABI_VERSION, WebPAnimDecoderOptions, WebPChunkIterator, WebPDemuxState,
    WebPFormatFeature, webp_anim_decoder_options_init,
};
use crate::webp::mux_types::WebPFeatureFlags;
use crate::webp::types::{PlaneOffset, PlaneStorage};

/// The largest scaled width or height that the API accepts: libwebp's
/// `WEBP_MAX_DIMENSION`.
const MAX_SCALED_DIMENSION: u32 = 16383;

//------------------------------------------------------------------------------
// Errors

/// Which output a stride or buffer error is about.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode_into, ColorMode, Config, DecodeError, Output, Plane};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [1_u8, 2, 3, 255].repeat(4);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 2, stride: 8 })?;
///
/// // A 2 by 2 image needs 8 + 8 bytes of RGBA rows.
/// let mut rows = [0_u8; 15];
/// let output = Output::Rows { data: &mut rows, stride: 8 };
/// let error = decode_into(&file, &Config::new(ColorMode::Rgba), output).unwrap_err();
/// assert_eq!(error, DecodeError::BufferTooShort { plane: Plane::Rows, len: 15, needed: 16 });
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Plane {
    /// The packed rows of [`Output::Rows`].
    Rows,
    /// The luma plane of [`YuvaPlanes`].
    Y,
    /// The U chroma plane of [`YuvaPlanes`].
    U,
    /// The V chroma plane of [`YuvaPlanes`].
    V,
    /// The alpha plane of [`YuvaPlanes`].
    A,
}

/// Why a decoding or one of its inputs failed.
///
/// The variants up to [`DecodeError::NotEnoughData`] are libwebp's
/// `VP8StatusCode` values 1 to 7; [`DecodeError::code`] returns the C code.
/// [`DecodeError::FrameFailed`] is a failed animation frame, for which libwebp
/// reports no status. The others are the API's own validation, which runs
/// before the translation is called.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{ColorMode, Config, DecodeError};
///
/// let error = Config::new(ColorMode::Rgb).crop(1, 0, 8, 8).validate().unwrap_err();
/// assert_eq!(error, DecodeError::InvalidCrop { left: 1, top: 0, width: 8, height: 8 });
/// assert_eq!(error.code(), None);
/// assert_eq!(DecodeError::BitstreamError.to_string(), "invalid bitstream");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum DecodeError {
    /// Memory could not be allocated (C: `VP8_STATUS_OUT_OF_MEMORY`).
    OutOfMemory,
    /// libwebp refused a parameter (C: `VP8_STATUS_INVALID_PARAM`): for the
    /// incremental decoder, a caller output that does not fit the image, or
    /// a crop outside it, which it learns from the header; mixed calls of
    /// [`IncrementalDecoder::append`] and [`IncrementalDecoder::update`], or a
    /// shorter buffer passed to `update`.
    InvalidParam,
    /// The bitstream is invalid or truncated (C: `VP8_STATUS_BITSTREAM_ERROR`).
    BitstreamError,
    /// The bitstream uses a feature that this decoder does not support, such
    /// as an animation given to [`decode`] (C: `VP8_STATUS_UNSUPPORTED_FEATURE`).
    UnsupportedFeature,
    /// The incremental decoding has not completed: the error of
    /// [`IncrementalDecoder::into_image`] before that (C:
    /// `VP8_STATUS_SUSPENDED`, which the decoding steps report as
    /// [`Status::NeedsMoreData`]).
    Suspended,
    /// The decoding was aborted by an I/O hook (C: `VP8_STATUS_USER_ABORT`);
    /// the API sets no hooks, so it does not return this code.
    UserAbort,
    /// The data ends before the information asked for (C:
    /// `VP8_STATUS_NOT_ENOUGH_DATA`): read more and try again.
    NotEnoughData,
    /// The animation decoder failed on a frame; libwebp's
    /// `WebPAnimDecoderGetNext` reports no status.
    FrameFailed,
    /// The crop has a zero size, an odd `left` or `top`, a value above
    /// `i32::MAX`, or does not lie inside the image.
    InvalidCrop {
        /// The left edge given.
        left: u32,
        /// The top edge given.
        top: u32,
        /// The width given.
        width: u32,
        /// The height given.
        height: u32,
    },
    /// A scaled width or height is 0 or above 16383.
    InvalidScale {
        /// The width given.
        width: u32,
        /// The height given.
        height: u32,
    },
    /// A stride is 0, shorter than a row of the output, longer than
    /// `i32::MAX`, or so long that the plane's extent does not fit in `usize`.
    InvalidStride {
        /// The output with the stride.
        plane: Plane,
        /// The stride given, in bytes.
        stride: usize,
    },
    /// A buffer is shorter than its rows: `(rows - 1) * stride + row length`
    /// bytes, or empty.
    BufferTooShort {
        /// The output that is too short.
        plane: Plane,
        /// Its length, in bytes.
        len: usize,
        /// The bytes that its rows and stride address.
        needed: usize,
    },
    /// The output does not match the color mode: [`Output::Rows`] for a YUV
    /// mode, [`Output::Planes`] for an RGB mode, an alpha plane missing for
    /// [`ColorMode::Yuva`] or given for [`ColorMode::Yuv`], or caller memory
    /// asked to become an [`Image`].
    OutputMismatch {
        /// The color mode of the configuration.
        mode: ColorMode,
    },
    /// A parameter is outside the range the API accepts: `dithering` or
    /// `alpha_dithering` above 100.
    InvalidParameter {
        /// The parameter's name.
        name: &'static str,
    },
}

impl DecodeError {
    /// The `VP8StatusCode` of this error, or `None` for [`DecodeError::FrameFailed`]
    /// and the API's own validation errors.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::DecodeError;
    /// use webp::webp::decode::VP8StatusCode;
    ///
    /// assert_eq!(DecodeError::NotEnoughData.code(), Some(VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA));
    /// assert_eq!(DecodeError::FrameFailed.code(), None);
    /// ```
    #[must_use]
    pub const fn code(&self) -> Option<VP8StatusCode> {
        Some(match self {
            Self::OutOfMemory => VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY,
            Self::InvalidParam => VP8StatusCode::VP8_STATUS_INVALID_PARAM,
            Self::BitstreamError => VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR,
            Self::UnsupportedFeature => VP8StatusCode::VP8_STATUS_UNSUPPORTED_FEATURE,
            Self::Suspended => VP8StatusCode::VP8_STATUS_SUSPENDED,
            Self::UserAbort => VP8StatusCode::VP8_STATUS_USER_ABORT,
            Self::NotEnoughData => VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA,
            Self::FrameFailed
            | Self::InvalidCrop { .. }
            | Self::InvalidScale { .. }
            | Self::InvalidStride { .. }
            | Self::BufferTooShort { .. }
            | Self::OutputMismatch { .. }
            | Self::InvalidParameter { .. } => return None,
        })
    }

    /// The error for a status that a failed call of the translation returned.
    /// The translation returns `Err` only with codes 1 to 7; a code outside
    /// them would be a bitstream error.
    const fn from_status(status: VP8StatusCode) -> Self {
        match status {
            VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY => Self::OutOfMemory,
            VP8StatusCode::VP8_STATUS_INVALID_PARAM => Self::InvalidParam,
            VP8StatusCode::VP8_STATUS_UNSUPPORTED_FEATURE => Self::UnsupportedFeature,
            VP8StatusCode::VP8_STATUS_SUSPENDED => Self::Suspended,
            VP8StatusCode::VP8_STATUS_USER_ABORT => Self::UserAbort,
            VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA => Self::NotEnoughData,
            _ => Self::BitstreamError,
        }
    }
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("out of memory"),
            Self::InvalidParam => f.write_str("invalid parameter"),
            Self::BitstreamError => f.write_str("invalid bitstream"),
            Self::UnsupportedFeature => f.write_str("unsupported feature"),
            Self::Suspended => f.write_str("the decoding has not completed"),
            Self::UserAbort => f.write_str("aborted"),
            Self::NotEnoughData => f.write_str("not enough data"),
            Self::FrameFailed => f.write_str("the animation frame could not be decoded"),
            Self::InvalidCrop {
                left,
                top,
                width,
                height,
            } => write!(f, "invalid crop of {width} by {height} at {left},{top}"),
            Self::InvalidScale { width, height } => write!(
                f,
                "invalid scale to {width} by {height}: each must be 1 to {MAX_SCALED_DIMENSION}"
            ),
            Self::InvalidStride { plane, stride } => {
                write!(f, "invalid stride {stride} for the {plane:?} output")
            }
            Self::BufferTooShort { plane, len, needed } => write!(
                f,
                "the {plane:?} output holds {len} bytes, its rows need {needed}"
            ),
            Self::OutputMismatch { mode } => {
                write!(f, "the output does not match the color mode {mode:?}")
            }
            Self::InvalidParameter { name } => write!(f, "invalid parameter: {name}"),
        }
    }
}

impl std::error::Error for DecodeError {}

/// The `Result` of a call of the translation as the API's result.
fn status(result: Result<(), VP8StatusCode>) -> Result<(), DecodeError> {
    result.map_err(DecodeError::from_status)
}

//------------------------------------------------------------------------------
// Features

/// The compression of a still image or of an animation's first frame
/// (C: `WebPBitstreamFeatures::format` 1 and 2).
///
/// # Examples
///
/// ```
/// use webp::api::decode::{Features, Format};
/// # use webp::api::encode::{encode_lossy, PixelLayout, PixelRows};
/// # let rgb = [9_u8, 99, 199].repeat(4);
/// # let file = encode_lossy(PixelRows { layout: PixelLayout::Rgb, data: &rgb, width: 2, height: 2, stride: 6 }, 75.0)?;
///
/// assert_eq!(Features::parse(&file)?.format, Some(Format::Lossy));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Format {
    /// A `VP8 ` bitstream.
    Lossy,
    /// A `VP8L` bitstream.
    Lossless,
}

/// What a WebP file says about itself before decoding (C:
/// `WebPBitstreamFeatures`).
///
/// # Examples
///
/// ```
/// use webp::api::decode::{DecodeError, Features};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [1_u8, 2, 3, 128].repeat(15);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 5, height: 3, stride: 20 })?;
///
/// let features = Features::parse(&file)?;
/// assert_eq!((features.width, features.height), (5, 3));
/// assert!(features.has_alpha && !features.has_animation);
///
/// // A prefix that ends before the dimensions asks for more data.
/// assert_eq!(Features::parse(&file[..10]), Err(DecodeError::NotEnoughData));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Features {
    /// The width in pixels: the canvas of an extended (`VP8X`) file.
    pub width: u32,
    /// The height in pixels: the canvas of an extended (`VP8X`) file.
    pub height: u32,
    /// Whether the file declares alpha (the `VP8X` flag, an `ALPH` chunk or a
    /// lossless bitstream's alpha bit).
    pub has_alpha: bool,
    /// Whether the file is an animation; decode it with [`AnimationDecoder`].
    pub has_animation: bool,
    /// The compression of the image, or `None` when the parse did not reach
    /// a bitstream header (libwebp reports 0, "undefined"): for an animation,
    /// whose frames each carry their own format, and for an extended still
    /// image probed with a prefix that ends before its image chunk. Use
    /// [`has_animation`](Self::has_animation) to tell the two apart.
    pub format: Option<Format>,
}

impl Features {
    /// Reads the features from the start of a WebP file (C:
    /// `WebPGetFeatures`); a prefix is enough once it reaches the bitstream's
    /// header, or the `VP8X` chunk of an extended file.
    ///
    /// After the `VP8X` chunk of an extended still image the result can carry
    /// `format: None`, with the canvas as the size, until the prefix reaches
    /// the image chunk's header; a caller who needs the format reads more
    /// until it is `Some`.
    ///
    /// # Errors
    ///
    /// [`DecodeError::NotEnoughData`] when the prefix ends too early, and
    /// [`DecodeError::BitstreamError`] for a file that is not a WebP file
    /// libwebp decodes.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{DecodeError, Features};
    ///
    /// assert_eq!(Features::parse(b"RIFF"), Err(DecodeError::NotEnoughData));
    /// assert_eq!(Features::parse(b"GIF89a, not a WebP file"), Err(DecodeError::BitstreamError));
    /// ```
    ///
    /// A prefix of an extended still image that ends after the `VP8X` chunk
    /// gives the canvas but no format yet:
    ///
    /// ```
    /// use webp::api::decode::{Features, Format};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # fn chunk(tag: &[u8], payload: &[u8]) -> Vec<u8> {
    /// #     let mut bytes = tag.to_vec();
    /// #     bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    /// #     bytes.extend_from_slice(payload);
    /// #     if payload.len() % 2 == 1 { bytes.push(0); }
    /// #     bytes
    /// # }
    /// # let rgba = [0_u8, 0, 0, 255].repeat(4);
    /// # let image = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 2, stride: 8 })?;
    /// # let vp8x = chunk(b"VP8X", &[0x20 | 0x08, 0, 0, 0, 1, 0, 0, 1, 0, 0]);
    /// # let body = [vp8x, chunk(b"ICCP", b"profile"), image[12..].to_vec(), chunk(b"EXIF", b"Exif\0\0MM")].concat();
    /// # let mut file = b"RIFF".to_vec();
    /// # file.extend_from_slice(&(body.len() as u32 + 4).to_le_bytes());
    /// # file.extend_from_slice(b"WEBP");
    /// # file.extend_from_slice(&body);
    ///
    /// // `file` is an extended 2 by 2 still image: a 12-byte RIFF header, an
    /// // 18-byte `VP8X` chunk, then `ICCP`, `VP8L` and `EXIF` chunks.
    /// let prefix = Features::parse(&file[..30])?;
    /// assert_eq!((prefix.width, prefix.height), (2, 2));
    /// assert_eq!((prefix.format, prefix.has_animation), (None, false));
    ///
    /// assert_eq!(Features::parse(&file)?.format, Some(Format::Lossless));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn parse(data: &[u8]) -> Result<Self, DecodeError> {
        let mut raw = WebPBitstreamFeatures::default();
        status(webp_get_features(data, &mut raw))?;
        Ok(Self {
            width: raw.width.cast_unsigned(),
            height: raw.height.cast_unsigned(),
            has_alpha: raw.has_alpha != 0,
            has_animation: raw.has_animation != 0,
            format: match raw.format {
                1 => Some(Format::Lossy),
                2 => Some(Format::Lossless),
                _ => None,
            },
        })
    }
}

//------------------------------------------------------------------------------
// Color modes

/// The layout of the decoded samples (C: `WEBP_CSP_MODE`, in its order).
///
/// Names give the byte order of a pixel; the `Premultiplied` modes multiply
/// red, green and blue by alpha.
///
/// # Examples
///
/// ```
/// use webp::api::decode::ColorMode;
///
/// assert_eq!(ColorMode::Bgra.bytes_per_pixel(), Some(4));
/// assert_eq!(ColorMode::Rgb565.bytes_per_pixel(), Some(2));
/// assert_eq!(ColorMode::Yuva.bytes_per_pixel(), None);
/// assert!(ColorMode::PremultipliedArgb.is_premultiplied());
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorMode {
    /// Red, green, blue (C: `MODE_RGB`).
    Rgb,
    /// Red, green, blue, straight alpha (C: `MODE_RGBA`).
    Rgba,
    /// Blue, green, red (C: `MODE_BGR`).
    Bgr,
    /// Blue, green, red, straight alpha (C: `MODE_BGRA`).
    Bgra,
    /// Straight alpha, red, green, blue (C: `MODE_ARGB`).
    Argb,
    /// Red and green, then blue and straight alpha, 4 bits each, in two bytes
    /// (C: `MODE_RGBA_4444`).
    Rgba4444,
    /// Red (5 bits) and green (6 bits), then blue (5 bits), in two bytes
    /// (C: `MODE_RGB_565`).
    Rgb565,
    /// [`ColorMode::Rgba`] with premultiplied alpha (C: `MODE_rgbA`).
    PremultipliedRgba,
    /// [`ColorMode::Bgra`] with premultiplied alpha (C: `MODE_bgrA`).
    PremultipliedBgra,
    /// [`ColorMode::Argb`] with premultiplied alpha (C: `MODE_Argb`).
    PremultipliedArgb,
    /// [`ColorMode::Rgba4444`] with premultiplied alpha (C: `MODE_rgbA_4444`).
    PremultipliedRgba4444,
    /// Y, U and V planes, 4:2:0, limited range (C: `MODE_YUV`).
    Yuv,
    /// Y, U, V and straight alpha planes (C: `MODE_YUVA`).
    Yuva,
}

impl ColorMode {
    /// The C colorspace.
    const fn raw(self) -> WEBP_CSP_MODE {
        match self {
            Self::Rgb => WEBP_CSP_MODE::MODE_RGB,
            Self::Rgba => WEBP_CSP_MODE::MODE_RGBA,
            Self::Bgr => WEBP_CSP_MODE::MODE_BGR,
            Self::Bgra => WEBP_CSP_MODE::MODE_BGRA,
            Self::Argb => WEBP_CSP_MODE::MODE_ARGB,
            Self::Rgba4444 => WEBP_CSP_MODE::MODE_RGBA_4444,
            Self::Rgb565 => WEBP_CSP_MODE::MODE_RGB_565,
            Self::PremultipliedRgba => WEBP_CSP_MODE::MODE_rgbA,
            Self::PremultipliedBgra => WEBP_CSP_MODE::MODE_bgrA,
            Self::PremultipliedArgb => WEBP_CSP_MODE::MODE_Argb,
            Self::PremultipliedRgba4444 => WEBP_CSP_MODE::MODE_rgbA_4444,
            Self::Yuv => WEBP_CSP_MODE::MODE_YUV,
            Self::Yuva => WEBP_CSP_MODE::MODE_YUVA,
        }
    }

    /// The bytes of a pixel in the RGB modes (libwebp's `kModeBpp`), or
    /// `None` for the planar YUV modes.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::ColorMode;
    ///
    /// assert_eq!(ColorMode::Rgb.bytes_per_pixel(), Some(3));
    /// assert_eq!(ColorMode::Yuv.bytes_per_pixel(), None);
    /// ```
    #[must_use]
    pub const fn bytes_per_pixel(self) -> Option<usize> {
        match self {
            Self::Rgb | Self::Bgr => Some(3),
            Self::Rgba
            | Self::Bgra
            | Self::Argb
            | Self::PremultipliedRgba
            | Self::PremultipliedBgra
            | Self::PremultipliedArgb => Some(4),
            Self::Rgba4444 | Self::Rgb565 | Self::PremultipliedRgba4444 => Some(2),
            Self::Yuv | Self::Yuva => None,
        }
    }

    /// Whether the output holds alpha (C: `WebPIsAlphaMode`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::ColorMode;
    ///
    /// assert!(ColorMode::Yuva.has_alpha());
    /// assert!(!ColorMode::Rgb565.has_alpha());
    /// ```
    #[must_use]
    pub const fn has_alpha(self) -> bool {
        !matches!(self, Self::Rgb | Self::Bgr | Self::Rgb565 | Self::Yuv)
    }

    /// Whether red, green and blue are multiplied by alpha (C:
    /// `WebPIsPremultipliedMode`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::ColorMode;
    ///
    /// assert!(ColorMode::PremultipliedRgba4444.is_premultiplied());
    /// assert!(!ColorMode::Rgba.is_premultiplied());
    /// ```
    #[must_use]
    pub const fn is_premultiplied(self) -> bool {
        matches!(
            self,
            Self::PremultipliedRgba
                | Self::PremultipliedBgra
                | Self::PremultipliedArgb
                | Self::PremultipliedRgba4444
        )
    }

    /// Whether the output is planar `Y'CbCr` ([`ColorMode::Yuv`],
    /// [`ColorMode::Yuva`]) rather than packed RGB rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::ColorMode;
    ///
    /// assert!(ColorMode::Yuv.is_yuv());
    /// assert!(!ColorMode::Argb.is_yuv());
    /// ```
    #[must_use]
    pub const fn is_yuv(self) -> bool {
        matches!(self, Self::Yuv | Self::Yuva)
    }
}

//------------------------------------------------------------------------------
// Configuration

/// `value` as C's `int`, saturated; the API refuses values above `i32::MAX`
/// before the translation sees them.
fn to_int(value: u32) -> i32 {
    i32::try_from(value).unwrap_or(i32::MAX)
}

/// Decoding options and the color mode: a `WebPDecoderOptions` with typed
/// setters, and the output colorspace.
///
/// The setters store their value and never fail; [`Config::validate`], which
/// every decoding calls first, reports a value that the API refuses, and
/// [`Config::output_size`] checks the crop against an image.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{ColorMode, Config};
///
/// let config = Config::new(ColorMode::PremultipliedBgra)
///     .crop(64, 32, 640, 480)
///     .scale(320, 240)
///     .multithreaded(true)
///     .dithering(50);
/// config.validate()?;
/// assert_eq!(config.as_raw().use_cropping, 1);
/// # Ok::<(), webp::api::decode::DecodeError>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Config {
    mode: ColorMode,
    options: WebPDecoderOptions,
    crop: Option<[u32; 4]>,
    scale: Option<[u32; 2]>,
}

impl Default for Config {
    /// [`ColorMode::Rgb`] without options, as `WebPInitDecoderConfig` leaves
    /// a configuration.
    fn default() -> Self {
        Self::new(ColorMode::Rgb)
    }
}

impl Config {
    /// A configuration for `mode` with libwebp's default options: no crop,
    /// scale, flip or dithering, the in-loop filter and fancy upsampling on,
    /// one thread (C: `WebPInitDecoderConfig`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// let config = Config::new(ColorMode::Yuva);
    /// assert_eq!(config.mode(), ColorMode::Yuva);
    /// assert_eq!(config.as_raw().no_fancy_upsampling, 0);
    /// ```
    #[must_use]
    pub fn new(mode: ColorMode) -> Self {
        let mut raw = WebPDecoderConfig::default();
        let initialized = webp_init_decoder_config(&mut raw);
        debug_assert!(initialized, "the crate's own ABI version matches");
        Self {
            mode,
            options: raw.options,
            crop: None,
            scale: None,
        }
    }

    /// Decodes only the rectangle of `width` by `height` pixels at `left`,
    /// `top` (C: `use_cropping`, `crop_left`, `crop_top`, `crop_width`,
    /// `crop_height`). `left` and `top` must be even: libwebp moves an odd
    /// lossy crop to the even position, so the API refuses odd offsets for
    /// every image. To read a rectangle at an odd position, crop one column or
    /// row earlier and skip it. The crop applies before the scale.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, DecodeError};
    ///
    /// assert!(Config::new(ColorMode::Rgb).crop(2, 4, 3, 5).validate().is_ok());
    /// assert!(Config::new(ColorMode::Rgb).crop(3, 4, 3, 5).validate().is_err());
    /// ```
    #[must_use]
    pub fn crop(mut self, left: u32, top: u32, width: u32, height: u32) -> Self {
        self.crop = Some([left, top, width, height]);
        self.options.use_cropping = 1;
        self.options.crop_left = to_int(left);
        self.options.crop_top = to_int(top);
        self.options.crop_width = to_int(width);
        self.options.crop_height = to_int(height);
        self
    }

    /// Scales the (cropped) image to `width` by `height` pixels, each 1 to
    /// 16383 (C: `use_scaling`, `scaled_width`, `scaled_height`). libwebp's
    /// rescaler disables fancy upsampling, and a scale below 3/4 in both
    /// directions skips the in-loop filter. libwebp derives a side given as 0;
    /// the API needs both, so that it can bound the output before decoding.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, DecodeError};
    ///
    /// assert!(Config::new(ColorMode::Rgb).scale(100, 50).validate().is_ok());
    /// assert_eq!(
    ///     Config::new(ColorMode::Rgb).scale(100, 0).validate(),
    ///     Err(DecodeError::InvalidScale { width: 100, height: 0 })
    /// );
    /// ```
    #[must_use]
    pub fn scale(mut self, width: u32, height: u32) -> Self {
        self.scale = Some([width, height]);
        self.options.use_scaling = 1;
        self.options.scaled_width = to_int(width);
        self.options.scaled_height = to_int(height);
        self
    }

    /// Flips the output vertically (C: `flip`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// assert_eq!(Config::new(ColorMode::Rgb).flip(true).as_raw().flip, 1);
    /// ```
    #[must_use]
    pub fn flip(mut self, on: bool) -> Self {
        self.options.flip = i32::from(on);
        self
    }

    /// Skips the in-loop filter of lossy images, which is faster and changes
    /// the samples (C: `bypass_filtering`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// assert_eq!(Config::new(ColorMode::Rgb).bypass_filtering(true).as_raw().bypass_filtering, 1);
    /// ```
    #[must_use]
    pub fn bypass_filtering(mut self, on: bool) -> Self {
        self.options.bypass_filtering = i32::from(on);
        self
    }

    /// Fancy upsampling of the chroma of lossy images in the RGB modes, on
    /// by default; off uses the faster point upsampler (C: the negation of
    /// `no_fancy_upsampling`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// let config = Config::new(ColorMode::Rgb).fancy_upsampling(false);
    /// assert_eq!(config.as_raw().no_fancy_upsampling, 1);
    /// ```
    #[must_use]
    pub fn fancy_upsampling(mut self, on: bool) -> Self {
        self.options.no_fancy_upsampling = i32::from(!on);
        self
    }

    /// Decodes lossy images with a second thread for filtering and output
    /// (C: `use_threads`); the samples do not change.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// assert_eq!(Config::new(ColorMode::Rgb).multithreaded(true).as_raw().use_threads, 1);
    /// ```
    #[must_use]
    pub fn multithreaded(mut self, on: bool) -> Self {
        self.options.use_threads = i32::from(on);
        self
    }

    /// The strength of the dither that libwebp adds to lossy output, 0 (off,
    /// the default) to 100 (C: `dithering_strength`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, DecodeError};
    ///
    /// assert!(Config::new(ColorMode::Rgb).dithering(100).validate().is_ok());
    /// assert_eq!(
    ///     Config::new(ColorMode::Rgb).dithering(101).validate(),
    ///     Err(DecodeError::InvalidParameter { name: "dithering" })
    /// );
    /// ```
    #[must_use]
    pub fn dithering(mut self, strength: u8) -> Self {
        self.options.dithering_strength = i32::from(strength);
        self
    }

    /// The strength of the smoothing of quantized alpha, 0 (off, the default)
    /// to 100 (C: `alpha_dithering_strength`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// let config = Config::new(ColorMode::Rgba).alpha_dithering(100);
    /// assert_eq!(config.as_raw().alpha_dithering_strength, 100);
    /// ```
    #[must_use]
    pub fn alpha_dithering(mut self, strength: u8) -> Self {
        self.options.alpha_dithering_strength = i32::from(strength);
        self
    }

    /// The color mode of the output.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// assert_eq!(Config::default().mode(), ColorMode::Rgb);
    /// ```
    #[must_use]
    pub const fn mode(&self) -> ColorMode {
        self.mode
    }

    /// Checks what does not depend on the image: a crop of at least 1 by 1
    /// pixel at even `left` and `top`, every value at most `i32::MAX`; a scale
    /// of 1 to 16383 on each side; dithering strengths of at most 100; then
    /// libwebp's `WebPValidateDecoderConfig`.
    ///
    /// # Errors
    ///
    /// [`DecodeError::InvalidCrop`], [`DecodeError::InvalidScale`] or
    /// [`DecodeError::InvalidParameter`] for the API's rules, and
    /// [`DecodeError::InvalidParam`] if libwebp refused the configuration
    /// after them.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, DecodeError};
    ///
    /// assert_eq!(
    ///     Config::new(ColorMode::Rgb).crop(0, 0, 0, 8).validate(),
    ///     Err(DecodeError::InvalidCrop { left: 0, top: 0, width: 0, height: 8 })
    /// );
    /// ```
    pub fn validate(&self) -> Result<(), DecodeError> {
        if let Some([left, top, width, height]) = self.crop {
            let too_large = [left, top, width, height]
                .iter()
                .any(|&value| i32::try_from(value).is_err());
            if width == 0 || height == 0 || left % 2 != 0 || top % 2 != 0 || too_large {
                return Err(DecodeError::InvalidCrop {
                    left,
                    top,
                    width,
                    height,
                });
            }
        }
        if let Some([width, height]) = self.scale {
            let range = 1..=MAX_SCALED_DIMENSION;
            if !range.contains(&width) || !range.contains(&height) {
                return Err(DecodeError::InvalidScale { width, height });
            }
        }
        if self.options.dithering_strength > 100 {
            return Err(DecodeError::InvalidParameter { name: "dithering" });
        }
        if self.options.alpha_dithering_strength > 100 {
            return Err(DecodeError::InvalidParameter {
                name: "alpha_dithering",
            });
        }
        // A backstop: the checks above already refuse every configuration
        // that `WebPValidateDecoderConfig` refuses without features.
        if webp_validate_decoder_config(&self.raw_config()) {
            Ok(())
        } else {
            Err(DecodeError::InvalidParam)
        }
    }

    /// The width and height of the output for an image with `features`: the
    /// crop's size, then the scale's, after [`Config::validate`] and a check
    /// that the crop lies inside the image (C: `WebPCheckCropDimensions`).
    ///
    /// # Errors
    ///
    /// The errors of [`Config::validate`], and [`DecodeError::InvalidCrop`]
    /// for a crop that does not lie inside the image.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, Features};
    ///
    /// let features = Features { width: 640, height: 480, has_alpha: false, has_animation: false, format: None };
    /// let config = Config::new(ColorMode::Rgb).crop(320, 0, 320, 480).scale(160, 240);
    /// assert_eq!(config.output_size(&features)?, (160, 240));
    /// assert!(Config::new(ColorMode::Rgb).crop(322, 0, 320, 480).output_size(&features).is_err());
    /// # Ok::<(), webp::api::decode::DecodeError>(())
    /// ```
    pub fn output_size(&self, features: &Features) -> Result<(u32, u32), DecodeError> {
        self.validate()?;
        let (mut width, mut height) = (features.width, features.height);
        if let Some([left, top, crop_width, crop_height]) = self.crop {
            if !webp_check_crop_dimensions(
                to_int(features.width),
                to_int(features.height),
                to_int(left),
                to_int(top),
                to_int(crop_width),
                to_int(crop_height),
            ) {
                return Err(DecodeError::InvalidCrop {
                    left,
                    top,
                    width: crop_width,
                    height: crop_height,
                });
            }
            (width, height) = (crop_width, crop_height);
        }
        if let Some([scaled_width, scaled_height]) = self.scale {
            (width, height) = (scaled_width, scaled_height);
        }
        Ok((width, height))
    }

    /// The options as libwebp reads them (C: `WebPDecoderOptions`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config};
    ///
    /// let config = Config::new(ColorMode::Rgb).crop(0, 2, 10, 20);
    /// let raw = config.as_raw();
    /// assert_eq!((raw.crop_top, raw.crop_width, raw.crop_height), (2, 10, 20));
    /// ```
    #[must_use]
    pub const fn as_raw(&self) -> &WebPDecoderOptions {
        &self.options
    }

    /// A `WebPDecoderConfig` with this colorspace and these options, and an
    /// output buffer without memory.
    fn raw_config<'a>(&self) -> WebPDecoderConfig<'a> {
        let mut raw = WebPDecoderConfig::default();
        let initialized = webp_init_decoder_config(&mut raw);
        debug_assert!(initialized, "the crate's own ABI version matches");
        raw.output.colorspace = self.mode.raw();
        raw.options = self.options;
        raw
    }
}

//------------------------------------------------------------------------------
// Caller memory

/// The caller's Y, U, V and optional alpha planes, an [`Output`] for
/// [`ColorMode::Yuv`] and [`ColorMode::Yuva`].
///
/// Every stride counts bytes. The Y and A planes have `height` rows of
/// `width` samples, the U and V planes `(height + 1) / 2` rows of
/// `(width + 1) / 2` samples; each stride is at least its row length and at
/// most `i32::MAX`, and each plane holds at least
/// `(rows - 1) * stride + row length` bytes. `a` is given exactly for
/// [`ColorMode::Yuva`]; `a_stride` is ignored without it.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode_into, ColorMode, Config, Output, YuvaPlanes};
/// # use webp::api::encode::{encode_lossy, PixelLayout, PixelRows};
/// # let rgb = [90_u8, 60, 30].repeat(15);
/// # let file = encode_lossy(PixelRows { layout: PixelLayout::Rgb, data: &rgb, width: 5, height: 3, stride: 15 }, 90.0)?;
///
/// // A 5 by 3 image: chroma planes of 3 by 2, rows padded to 8 bytes.
/// let (mut y, mut u, mut v) = (vec![0_u8; 8 * 3], vec![0_u8; 8 * 2], vec![0_u8; 8 * 2]);
/// let planes = YuvaPlanes {
///     y: &mut y, y_stride: 8,
///     u: &mut u, v: &mut v, uv_stride: 8,
///     a: None, a_stride: 0,
/// };
/// decode_into(&file, &Config::new(ColorMode::Yuv), Output::Planes(planes))?;
/// assert!(y[..5].iter().all(|&luma| (16..=235).contains(&luma)));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct YuvaPlanes<'a> {
    /// The luma plane.
    pub y: &'a mut [u8],
    /// The distance between luma rows, in bytes.
    pub y_stride: usize,
    /// The U (Cb) chroma plane.
    pub u: &'a mut [u8],
    /// The V (Cr) chroma plane.
    pub v: &'a mut [u8],
    /// The distance between rows of both chroma planes, in bytes.
    pub uv_stride: usize,
    /// The alpha plane, straight alpha, for [`ColorMode::Yuva`].
    pub a: Option<&'a mut [u8]>,
    /// The distance between alpha rows, in bytes.
    pub a_stride: usize,
}

impl fmt::Debug for YuvaPlanes<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("YuvaPlanes")
            .field("y_len", &self.y.len())
            .field("y_stride", &self.y_stride)
            .field("u_len", &self.u.len())
            .field("v_len", &self.v.len())
            .field("uv_stride", &self.uv_stride)
            .field("a_len", &self.a.as_ref().map(|a| a.len()))
            .field("a_stride", &self.a_stride)
            .finish()
    }
}

/// Caller memory that a decoding writes into.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode_into, ColorMode, Config, Output};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [7_u8, 8, 9, 255].repeat(12);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 4, height: 3, stride: 16 })?;
///
/// // 4 by 3 BGR pixels in rows of 16 bytes (12 used).
/// let mut rows = vec![0_u8; 16 * 2 + 12];
/// let size = decode_into(&file, &Config::new(ColorMode::Bgr), Output::Rows { data: &mut rows, stride: 16 })?;
/// assert_eq!(size, (4, 3));
/// assert_eq!(&rows[..3], &[9, 8, 7]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub enum Output<'a> {
    /// Packed rows for the RGB modes: at least
    /// `(height - 1) * stride + width * bytes_per_pixel` bytes.
    Rows {
        /// The rows, top to bottom.
        data: &'a mut [u8],
        /// The distance from the start of a row to the next, in bytes: at
        /// least `width * bytes_per_pixel` and at most `i32::MAX`.
        stride: usize,
    },
    /// Planes for [`ColorMode::Yuv`] and [`ColorMode::Yuva`].
    Planes(YuvaPlanes<'a>),
}

impl fmt::Debug for Output<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Rows { data, stride } => f
                .debug_struct("Rows")
                .field("data_len", &data.len())
                .field("stride", stride)
                .finish(),
            Self::Planes(planes) => f.debug_tuple("Planes").field(planes).finish(),
        }
    }
}

/// The API's check of one output plane: `rows` rows of `row` bytes at
/// `stride`, inside `len` bytes.
fn check_plane(
    plane: Plane,
    len: usize,
    stride: usize,
    row: usize,
    rows: usize,
) -> Result<(), DecodeError> {
    if stride < row || i32::try_from(stride).is_err() {
        return Err(DecodeError::InvalidStride { plane, stride });
    }
    let needed = rows
        .saturating_sub(1)
        .checked_mul(stride)
        .and_then(|extent| extent.checked_add(row))
        .ok_or(DecodeError::InvalidStride { plane, stride })?;
    if len < needed {
        return Err(DecodeError::BufferTooShort { plane, len, needed });
    }
    Ok(())
}

/// Whether `output` has the shape that `mode` needs: rows for the RGB modes,
/// planes for the YUV modes, with an alpha plane exactly for `Yuva`.
fn check_shape(mode: ColorMode, output: &Output<'_>) -> Result<(), DecodeError> {
    let matches = match output {
        Output::Rows { .. } => !mode.is_yuv(),
        Output::Planes(planes) => mode.is_yuv() && planes.a.is_some() == (mode == ColorMode::Yuva),
    };
    if matches {
        Ok(())
    } else {
        Err(DecodeError::OutputMismatch { mode })
    }
}

/// The checks of `output` against an output of `width` by `height` in
/// `mode` (C: `CheckDecBuffer`, with the API's own errors).
fn check_output(
    mode: ColorMode,
    width: u32,
    height: u32,
    output: &Output<'_>,
) -> Result<(), DecodeError> {
    check_shape(mode, output)?;
    let width = usize::try_from(width).unwrap_or(usize::MAX);
    let height = usize::try_from(height).unwrap_or(usize::MAX);
    match output {
        Output::Rows { data, stride } => {
            let bpp = mode.bytes_per_pixel().unwrap_or(1);
            let row = width.checked_mul(bpp).ok_or(DecodeError::InvalidStride {
                plane: Plane::Rows,
                stride: *stride,
            })?;
            check_plane(Plane::Rows, data.len(), *stride, row, height)
        }
        Output::Planes(planes) => {
            let (uv_width, uv_height) = (width.div_ceil(2), height.div_ceil(2));
            check_plane(Plane::Y, planes.y.len(), planes.y_stride, width, height)?;
            check_plane(
                Plane::U,
                planes.u.len(),
                planes.uv_stride,
                uv_width,
                uv_height,
            )?;
            check_plane(
                Plane::V,
                planes.v.len(),
                planes.uv_stride,
                uv_width,
                uv_height,
            )?;
            if let Some(a) = &planes.a {
                check_plane(Plane::A, a.len(), planes.a_stride, width, height)?;
            }
            Ok(())
        }
    }
}

/// The checks of `output` that do not depend on the image: the shape, and
/// strides of 1 to `i32::MAX` bytes over non-empty slices.
fn check_output_shape(mode: ColorMode, output: &Output<'_>) -> Result<(), DecodeError> {
    check_shape(mode, output)?;
    let check = |plane, len: usize, stride: usize| {
        if stride == 0 || i32::try_from(stride).is_err() {
            Err(DecodeError::InvalidStride { plane, stride })
        } else if len == 0 {
            Err(DecodeError::BufferTooShort {
                plane,
                len,
                needed: 1,
            })
        } else {
            Ok(())
        }
    };
    match output {
        Output::Rows { data, stride } => check(Plane::Rows, data.len(), *stride),
        Output::Planes(planes) => {
            check(Plane::Y, planes.y.len(), planes.y_stride)?;
            check(Plane::U, planes.u.len(), planes.uv_stride)?;
            check(Plane::V, planes.v.len(), planes.uv_stride)?;
            if let Some(a) = &planes.a {
                check(Plane::A, a.len(), planes.a_stride)?;
            }
            Ok(())
        }
    }
}

/// A stride that the checks above accepted, as C's `int`.
fn stride_int(stride: usize) -> i32 {
    i32::try_from(stride).unwrap_or(i32::MAX)
}

/// Lends the caller's memory to `buffer` as `PlaneStorage::External`, with
/// each `size` equal to its slice's length (C: `is_external_memory` 1 and
/// the plane pointers, strides and sizes).
fn attach<'a>(buffer: &mut WebPDecBuffer<'a>, output: Output<'a>) {
    let plane = |region| PlaneOffset { region, offset: 0 };
    match output {
        Output::Rows { data, stride } => {
            buffer.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
                rgba: plane(0),
                stride: stride_int(stride),
                size: data.len(),
            });
            buffer.storage = PlaneStorage::External(vec![data]);
        }
        Output::Planes(planes) => {
            let YuvaPlanes {
                y,
                y_stride,
                u,
                v,
                uv_stride,
                a,
                a_stride,
            } = planes;
            buffer.u = WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
                y: plane(0),
                u: plane(1),
                v: plane(2),
                a: a.is_some().then(|| plane(3)),
                y_stride: stride_int(y_stride),
                u_stride: stride_int(uv_stride),
                v_stride: stride_int(uv_stride),
                a_stride: a.as_ref().map_or(0, |_| stride_int(a_stride)),
                y_size: y.len(),
                u_size: u.len(),
                v_size: v.len(),
                a_size: a.as_ref().map_or(0, |a| a.len()),
            });
            let mut slices = vec![y, u, v];
            slices.extend(a);
            buffer.storage = PlaneStorage::External(slices);
        }
    }
    buffer.slow_memory = false;
}

//------------------------------------------------------------------------------
// Decoded rows

/// Rows of one plane: a region of memory, the position of the first row, a
/// signed stride in bytes and the number of rows.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode, ColorMode, Config};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let mut rgba = [0_u8, 0, 0, 255].repeat(6);
/// # rgba[..4].copy_from_slice(&[255, 0, 0, 255]);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 })?;
///
/// let image = decode(&file, &Config::new(ColorMode::Rgb))?;
/// let rows = image.rows().expect("an RGB mode");
/// assert_eq!((rows.len(), rows.row_len(), rows.stride()), (2, 9, 9));
/// assert_eq!(&rows.row(0).unwrap()[..3], &[255, 0, 0]);
/// assert_eq!(rows.iter().count(), 2);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy)]
pub struct Rows<'s> {
    data: &'s [u8],
    first: usize,
    stride: isize,
    row_len: usize,
    count: u32,
}

impl fmt::Debug for Rows<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Rows")
            .field("region_len", &self.data.len())
            .field("first", &self.first)
            .field("stride", &self.stride)
            .field("row_len", &self.row_len)
            .field("count", &self.count)
            .finish()
    }
}

impl<'s> Rows<'s> {
    /// The rows from `first` at `stride` (C's `int`) in `data`.
    fn new(data: &'s [u8], first: usize, stride: i32, row_len: usize, count: i32) -> Self {
        Self {
            data,
            first,
            stride: isize::try_from(stride).unwrap_or(0),
            row_len,
            count: u32::try_from(count).unwrap_or(0),
        }
    }

    /// The number of rows.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 4 * 5];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 4, height: 5, stride: 16 })?;
    ///
    /// assert_eq!(decode(&file, &Config::new(ColorMode::Rgba))?.rows().unwrap().len(), 5);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn len(&self) -> u32 {
        self.count
    }

    /// Whether there are no rows, as before an incremental decoding has
    /// decoded any.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert!(!decode(&file, &Config::new(ColorMode::Rgba))?.rows().unwrap().is_empty());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn is_empty(&self) -> bool {
        self.count == 0
    }

    /// Row `index`, `row_len` bytes from the first row plus `index` strides,
    /// or `None` for an index at or past [`Rows::len`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [5_u8, 6, 7, 255, 1, 2, 3, 255];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 2, stride: 4 })?;
    ///
    /// let image = decode(&file, &Config::new(ColorMode::Rgb))?;
    /// let rows = image.rows().unwrap();
    /// assert_eq!(rows.row(1), Some(&[1_u8, 2, 3][..]));
    /// assert_eq!(rows.row(2), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn row(&self, index: u32) -> Option<&'s [u8]> {
        if index >= self.count {
            return None;
        }
        let offset = isize::try_from(index).ok()?.checked_mul(self.stride)?;
        let start = self.first.checked_add_signed(offset)?;
        self.data.get(start..start.checked_add(self.row_len)?)
    }

    /// The rows in order: row 0, then row 1, and so on.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [9_u8; 4 * 6];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 3, stride: 8 })?;
    ///
    /// let image = decode(&file, &Config::new(ColorMode::Rgba))?;
    /// assert!(image.rows().unwrap().iter().all(|row| row == [9_u8; 8]));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn iter(&self) -> impl Iterator<Item = &'s [u8]> + 's {
        let rows = *self;
        (0..rows.count).filter_map(move |index| rows.row(index))
    }

    /// The bytes of a row: the width times the bytes per pixel, or the width
    /// of a plane.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 5];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 5, height: 1, stride: 20 })?;
    ///
    /// assert_eq!(decode(&file, &Config::new(ColorMode::Rgb565))?.rows().unwrap().row_len(), 10);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn row_len(&self) -> usize {
        self.row_len
    }

    /// The distance from a row to the next, in bytes; negative while a
    /// flipped incremental decoding runs, whose row 0 is then the bottom row
    /// of the output.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 3 * 2];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 })?;
    ///
    /// assert_eq!(decode(&file, &Config::new(ColorMode::Argb))?.rows().unwrap().stride(), 12);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn stride(&self) -> isize {
        self.stride
    }
}

/// The rows of the Y, U, V and, for [`ColorMode::Yuva`], alpha planes.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode, ColorMode, Config};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [40_u8, 80, 120, 200].repeat(15);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 5, height: 3, stride: 20 })?;
///
/// let image = decode(&file, &Config::new(ColorMode::Yuva))?;
/// let planes = image.planes().expect("a YUV mode");
/// assert_eq!((planes.y.len(), planes.y.row_len()), (3, 5));
/// assert_eq!((planes.u.len(), planes.u.row_len()), (2, 3));
/// assert_eq!(planes.a.unwrap().row(0), Some(&[200_u8; 5][..]));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug)]
pub struct YuvaRows<'s> {
    /// The luma rows.
    pub y: Rows<'s>,
    /// The U (Cb) rows.
    pub u: Rows<'s>,
    /// The V (Cr) rows.
    pub v: Rows<'s>,
    /// The alpha rows, for [`ColorMode::Yuva`].
    pub a: Option<Rows<'s>>,
}

/// Where a plane of an [`Image`] lies in its memory.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct PlaneLayout {
    offset: usize,
    stride: i32,
}

/// The planes of an [`Image`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Layout {
    Packed(PlaneLayout),
    Planar {
        y: PlaneLayout,
        u: PlaneLayout,
        v: PlaneLayout,
        a: Option<PlaneLayout>,
    },
}

/// A decoded image in memory that libwebp allocated, which the image owns.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode, ColorMode, Config};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [11_u8, 22, 33, 44].repeat(8);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 4, height: 2, stride: 16 })?;
///
/// let image = decode(&file, &Config::new(ColorMode::Bgra))?;
/// assert_eq!((image.mode(), image.width(), image.height()), (ColorMode::Bgra, 4, 2));
/// assert_eq!(&image.samples()[..4], &[33, 22, 11, 44]);
/// let samples: Vec<u8> = image.into_samples();
/// assert_eq!(samples.len(), 4 * 4 * 2);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, PartialEq, Eq)]
pub struct Image {
    mode: ColorMode,
    width: u32,
    height: u32,
    samples: Vec<u8>,
    layout: Layout,
}

impl fmt::Debug for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Image")
            .field("mode", &self.mode)
            .field("width", &self.width)
            .field("height", &self.height)
            .field("samples_len", &self.samples.len())
            .field("layout", &self.layout)
            .finish()
    }
}

impl Image {
    /// The image in a buffer whose memory libwebp allocated: its memory and
    /// plane layout. A buffer that holds caller memory or no memory is not an
    /// image; the API never passes one.
    fn from_buffer(mode: ColorMode, buffer: WebPDecBuffer<'_>) -> Result<Self, DecodeError> {
        let width = u32::try_from(buffer.width).map_err(|_| DecodeError::InvalidParam)?;
        let height = u32::try_from(buffer.height).map_err(|_| DecodeError::InvalidParam)?;
        let PlaneStorage::Internal(samples) = buffer.storage else {
            return Err(DecodeError::OutputMismatch { mode });
        };
        if samples.is_empty() {
            // A backstop: both callers pass a buffer only after a complete
            // decoding, which allocated the memory.
            return Err(DecodeError::Suspended);
        }
        let plane = |offset: PlaneOffset, stride: i32| PlaneLayout {
            offset: offset.offset,
            stride,
        };
        let layout = match buffer.u {
            WebPDecBufferPlanes::RGBA(rgba) => Layout::Packed(plane(rgba.rgba, rgba.stride)),
            WebPDecBufferPlanes::YUVA(yuva) => Layout::Planar {
                y: plane(yuva.y, yuva.y_stride),
                u: plane(yuva.u, yuva.u_stride),
                v: plane(yuva.v, yuva.v_stride),
                a: yuva.a.map(|a| plane(a, yuva.a_stride)),
            },
        };
        Ok(Self {
            mode,
            width,
            height,
            samples,
            layout,
        })
    }

    /// The color mode of the samples.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(decode(&file, &Config::new(ColorMode::Yuv))?.mode(), ColorMode::Yuv);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn mode(&self) -> ColorMode {
        self.mode
    }

    /// The width in pixels, after crop and scale.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 8 * 8];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 8, height: 8, stride: 32 })?;
    ///
    /// let image = decode(&file, &Config::new(ColorMode::Rgb).scale(3, 2))?;
    /// assert_eq!((image.width(), image.height()), (3, 2));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.width
    }

    /// The height in pixels, after crop and scale.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 8 * 8];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 8, height: 8, stride: 32 })?;
    ///
    /// assert_eq!(decode(&file, &Config::new(ColorMode::Rgb).crop(0, 2, 8, 4))?.height(), 4);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.height
    }

    /// The rows of an RGB mode, top to bottom, or `None` for a YUV mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [1_u8, 2, 3, 4, 5, 6, 7, 8];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 2, stride: 4 })?;
    ///
    /// let flipped = decode(&file, &Config::new(ColorMode::Rgba).flip(true))?;
    /// assert_eq!(flipped.rows().unwrap().row(0), Some(&[5_u8, 6, 7, 8][..]));
    /// assert!(decode(&file, &Config::new(ColorMode::Yuva))?.rows().is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn rows(&self) -> Option<Rows<'_>> {
        let Layout::Packed(plane) = self.layout else {
            return None;
        };
        let row_len = usize::try_from(self.width)
            .ok()?
            .checked_mul(self.mode.bytes_per_pixel()?)?;
        Some(Rows::new(
            &self.samples,
            plane.offset,
            plane.stride,
            row_len,
            i32::try_from(self.height).ok()?,
        ))
    }

    /// The planes of a YUV mode, or `None` for an RGB mode.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [255_u8; 4 * 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 2, stride: 8 })?;
    ///
    /// let image = decode(&file, &Config::new(ColorMode::Yuv))?;
    /// let planes = image.planes().unwrap();
    /// assert_eq!(planes.y.row(0), Some(&[235_u8, 235][..])); // white, limited range
    /// assert!(planes.a.is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn planes(&self) -> Option<YuvaRows<'_>> {
        let Layout::Planar { y, u, v, a } = self.layout else {
            return None;
        };
        let width = usize::try_from(self.width).ok()?;
        let height = i32::try_from(self.height).ok()?;
        let (uv_width, uv_height) = (width.div_ceil(2), (height + 1) / 2);
        let rows = |plane: PlaneLayout, row_len, count| {
            Rows::new(&self.samples, plane.offset, plane.stride, row_len, count)
        };
        Some(YuvaRows {
            y: rows(y, width, height),
            u: rows(u, uv_width, uv_height),
            v: rows(v, uv_width, uv_height),
            a: a.map(|a| rows(a, width, height)),
        })
    }

    /// All the memory of the image: the rows of an RGB mode, or the Y, U, V
    /// and alpha planes one after the other, as libwebp lays them out.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 3 * 3];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 3, stride: 12 })?;
    ///
    /// // 3 by 3 luma, 2 by 2 U and V, 3 by 3 alpha.
    /// assert_eq!(decode(&file, &Config::new(ColorMode::Yuva))?.samples().len(), 9 + 4 + 4 + 9);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn samples(&self) -> &[u8] {
        &self.samples
    }

    /// The memory of the image, without a copy; see [`Image::samples`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{decode, ColorMode, Config};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [3_u8; 4 * 2];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 1, stride: 8 })?;
    ///
    /// assert_eq!(decode(&file, &Config::new(ColorMode::Rgba))?.into_samples(), vec![3_u8; 8]);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn into_samples(self) -> Vec<u8> {
        self.samples
    }
}

//------------------------------------------------------------------------------
// Decoding

/// The steps before `WebPDecode` that the API adds: the configuration, the
/// features (with `WebPDecode`'s status on failure), the refusal of an
/// animation that libwebp would report, and the output size.
fn prepare(data: &[u8], config: &Config) -> Result<(u32, u32), DecodeError> {
    config.validate()?;
    let features = match Features::parse(data) {
        Ok(features) => features,
        // WebPDecode treats not enough data as a bitstream error.
        Err(DecodeError::NotEnoughData) => return Err(DecodeError::BitstreamError),
        Err(error) => return Err(error),
    };
    if features.has_animation {
        return Err(DecodeError::UnsupportedFeature);
    }
    config.output_size(&features)
}

/// Decodes a still WebP image into memory that libwebp allocates (C:
/// `WebPDecode` with an output buffer without memory). `WebPDecodeRGBA` and
/// the other simple functions of libwebp are this function with
/// `Config::new(mode)`.
///
/// # Errors
///
/// The errors of [`Config::validate`] and [`Config::output_size`] first; then
/// libwebp's status: [`DecodeError::BitstreamError`] for an invalid file or
/// one that ends before the image's dimensions, [`DecodeError::NotEnoughData`]
/// or [`DecodeError::BitstreamError`] for one that ends later,
/// [`DecodeError::UnsupportedFeature`] for an animation (use
/// [`AnimationDecoder`]), [`DecodeError::OutOfMemory`].
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode, ColorMode, Config, DecodeError};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [10_u8, 20, 30, 255].repeat(16);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 4, height: 4, stride: 16 })?;
///
/// let image = decode(&file, &Config::new(ColorMode::Rgb))?;
/// assert_eq!(image.rows().unwrap().row(3), Some(&[10_u8, 20, 30].repeat(4)[..]));
/// // A prefix without the image's dimensions.
/// assert_eq!(decode(&file[..10], &Config::new(ColorMode::Rgb)), Err(DecodeError::BitstreamError));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn decode(data: &[u8], config: &Config) -> Result<Image, DecodeError> {
    prepare(data, config)?;
    let mut raw = config.raw_config();
    status(webp_decode(data, &mut raw))?;
    Image::from_buffer(config.mode, raw.output)
}

/// Decodes a still WebP image into the caller's memory (C: `WebPDecode` with
/// `is_external_memory` 1), which is checked against the output size first.
/// Returns the output's width and height. On failure, the rows that were
/// decoded stay in the caller's memory.
///
/// # Errors
///
/// The errors of [`decode`], and [`DecodeError::OutputMismatch`],
/// [`DecodeError::InvalidStride`] or [`DecodeError::BufferTooShort`] for
/// memory that does not fit the output.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{decode_into, ColorMode, Config, Features, Output};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [1_u8, 2, 3, 4].repeat(64);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 16, height: 4, stride: 64 })?;
///
/// // Budget exactly the rows of the cropped output.
/// let config = Config::new(ColorMode::Rgba).crop(4, 2, 8, 2);
/// let (width, height) = config.output_size(&Features::parse(&file)?)?;
/// let stride = width as usize * 4;
/// let mut rows = vec![0_u8; stride * height as usize];
/// decode_into(&file, &config, Output::Rows { data: &mut rows, stride })?;
/// assert_eq!(rows, [1_u8, 2, 3, 4].repeat(16));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub fn decode_into(
    data: &[u8],
    config: &Config,
    output: Output<'_>,
) -> Result<(u32, u32), DecodeError> {
    let (width, height) = prepare(data, config)?;
    check_output(config.mode, width, height, &output)?;
    let mut raw = config.raw_config();
    attach(&mut raw.output, output);
    status(webp_decode(data, &mut raw))?;
    Ok((width, height))
}

//------------------------------------------------------------------------------
// Incremental decoding

/// How far an incremental decoding has come.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{ColorMode, Config, IncrementalDecoder, Status};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [0_u8; 4 * 8 * 8];
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 8, height: 8, stride: 32 })?;
///
/// let config = Config::new(ColorMode::Rgba);
/// let mut decoder = IncrementalDecoder::new(&config)?;
/// assert_eq!(decoder.append(&file[..20])?, Status::NeedsMoreData);
/// assert_eq!(decoder.append(&file[20..])?, Status::Complete);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Status {
    /// The image is incomplete: pass more data (C: `VP8_STATUS_SUSPENDED`).
    NeedsMoreData,
    /// The image is decoded (C: `VP8_STATUS_OK`).
    Complete,
}

/// A decoder that takes a still WebP file in pieces and lends the rows
/// decoded so far (C: `WebPIDecoder`).
///
/// It borrows the [`Config`] and, with [`IncrementalDecoder::with_output`],
/// the caller's memory, for its lifetime. [`IncrementalDecoder::append`]
/// copies each piece; [`IncrementalDecoder::update`] takes the whole data so
/// far each time, for a caller who keeps it. The two cannot be mixed.
/// The decoder is neither `Send` nor `Sync`.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{ColorMode, Config, IncrementalDecoder, Status};
/// # use webp::api::encode::{encode_lossy, PixelLayout, PixelRows};
/// # let rgb: Vec<u8> = (0..64 * 48 * 3).map(|i| (i % 251) as u8).collect();
/// # let file = encode_lossy(PixelRows { layout: PixelLayout::Rgb, data: &rgb, width: 64, height: 48, stride: 192 }, 80.0)?;
///
/// let config = Config::new(ColorMode::Rgb);
/// let mut decoder = IncrementalDecoder::new(&config)?;
/// let mut status = Status::NeedsMoreData;
/// for piece in file.chunks(100) {
///     status = decoder.append(piece)?;
///     // The rows decoded so far, lent until the next call.
///     if let Some(rows) = decoder.rows() {
///         assert_eq!(rows.len(), decoder.decoded_rows());
///     }
/// }
/// assert_eq!(status, Status::Complete);
/// let image = decoder.into_image()?;
/// assert_eq!((image.width(), image.height()), (64, 48));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct IncrementalDecoder<'a> {
    idec: WebPIDecoder<'a, 'a>,
    mode: ColorMode,
    library_memory: bool,
    complete: bool,
    /// Whether an `update`, or an `append` that did not fail with
    /// `OutOfMemory`, reached the translation, so that an empty `append` may
    /// reach it too.
    started: bool,
}

impl fmt::Debug for IncrementalDecoder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("IncrementalDecoder")
            .field("mode", &self.mode)
            .field("library_memory", &self.library_memory)
            .field("complete", &self.complete)
            .field("decoded_rows", &self.decoded_rows())
            .finish()
    }
}

impl<'a> IncrementalDecoder<'a> {
    /// An incremental decoding into memory that libwebp allocates when the
    /// header arrives (C: `WebPINewDecoder` with a buffer in the colorspace
    /// of `config`, and `config`'s options, as `WebPIDecode` sets them).
    ///
    /// # Errors
    ///
    /// The errors of [`Config::validate`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, IncrementalDecoder};
    ///
    /// let config = Config::new(ColorMode::Yuva).flip(true);
    /// let decoder = IncrementalDecoder::new(&config)?;
    /// assert_eq!(decoder.decoded_rows(), 0);
    /// # Ok::<(), webp::api::decode::DecodeError>(())
    /// ```
    pub fn new(config: &'a Config) -> Result<Self, DecodeError> {
        config.validate()?;
        let mut idec = webp_i_new_decoder(None).ok_or(DecodeError::OutOfMemory)?;
        idec.own_output_mut().colorspace = config.mode.raw();
        idec.set_options(&config.options);
        Ok(Self {
            idec,
            mode: config.mode,
            library_memory: true,
            complete: false,
            started: false,
        })
    }

    /// An incremental decoding into the caller's memory (C: `WebPINewRGB` or
    /// `WebPINewYUVA`, and `config`'s options). The image's size is not known
    /// yet, so the API checks only the shape of `output` (rows for the RGB
    /// modes, planes for the YUV modes, an alpha plane exactly for
    /// [`ColorMode::Yuva`]), strides of 1 to `i32::MAX` bytes and non-empty
    /// slices; libwebp checks the rest when the header arrives and returns
    /// [`DecodeError::InvalidParam`] for memory that does not fit. Check
    /// before with [`Config::output_size`] and the rules of [`Output`].
    ///
    /// # Errors
    ///
    /// The errors of [`Config::validate`], [`DecodeError::OutputMismatch`],
    /// [`DecodeError::InvalidStride`] and [`DecodeError::BufferTooShort`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, IncrementalDecoder, Output, Status};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [50_u8, 60, 70, 255].repeat(6);
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 })?;
    ///
    /// let config = Config::new(ColorMode::Rgb);
    /// let mut rows = vec![0_u8; 2 * 9];
    /// {
    ///     let output = Output::Rows { data: &mut rows, stride: 9 };
    ///     let mut decoder = IncrementalDecoder::with_output(&config, output)?;
    ///     assert_eq!(decoder.update(&file)?, Status::Complete);
    /// }
    /// assert_eq!(rows, [50_u8, 60, 70].repeat(6));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn with_output(config: &'a Config, output: Output<'a>) -> Result<Self, DecodeError> {
        config.validate()?;
        check_output_shape(config.mode, &output)?;
        let idec = match output {
            Output::Rows { data, stride } => {
                webp_i_new_rgb(config.mode.raw(), Some(data), stride_int(stride))
            }
            Output::Planes(planes) => {
                let a_stride = planes.a.as_ref().map_or(0, |_| stride_int(planes.a_stride));
                webp_i_new_yuva(
                    Some(planes.y),
                    stride_int(planes.y_stride),
                    Some(planes.u),
                    stride_int(planes.uv_stride),
                    Some(planes.v),
                    stride_int(planes.uv_stride),
                    planes.a,
                    a_stride,
                )
            }
        };
        let mut idec = idec.ok_or(DecodeError::InvalidParam)?;
        idec.set_options(&config.options);
        Ok(Self {
            idec,
            mode: config.mode,
            library_memory: false,
            complete: false,
            started: false,
        })
    }

    /// The step's result as the API reports it.
    fn step(&mut self, result: Result<(), VP8StatusCode>) -> Result<Status, DecodeError> {
        match result {
            Ok(()) => {
                self.complete = true;
                Ok(Status::Complete)
            }
            Err(VP8StatusCode::VP8_STATUS_SUSPENDED) => Ok(Status::NeedsMoreData),
            Err(code) => Err(DecodeError::from_status(code)),
        }
    }

    /// Copies the next piece of the file and decodes as far as it reaches
    /// (C: `WebPIAppend`). An empty piece before any data is
    /// [`Status::NeedsMoreData`]: libwebp asserts against that call, so the
    /// API answers it without calling the decoder.
    ///
    /// # Errors
    ///
    /// [`DecodeError::BitstreamError`] for an invalid file (and for every
    /// call after a failure), [`DecodeError::UnsupportedFeature`] for an
    /// animation, [`DecodeError::InvalidParam`] for caller memory or a crop
    /// that does not fit the image, or after [`IncrementalDecoder::update`],
    /// and [`DecodeError::OutOfMemory`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, DecodeError, IncrementalDecoder, Status};
    ///
    /// let config = Config::new(ColorMode::Rgb);
    /// let mut decoder = IncrementalDecoder::new(&config)?;
    /// assert_eq!(decoder.append(b"RIFF"), Ok(Status::NeedsMoreData));
    /// assert_eq!(decoder.append(b"\0\0\0\0JPEG"), Err(DecodeError::BitstreamError));
    /// # Ok::<(), DecodeError>(())
    /// ```
    pub fn append(&mut self, data: &[u8]) -> Result<Status, DecodeError> {
        if data.is_empty() && !self.started {
            // libwebp asserts that data was stored before (`WebPIAppend` of
            // zero bytes first); without the assertion it reports SUSPENDED.
            return Ok(Status::NeedsMoreData);
        }
        let result = webp_i_append(&mut self.idec, data);
        // An out-of-memory failure can leave the buffer empty: a piece above
        // `MAX_CHUNK_PAYLOAD` or a failed reservation stores nothing.
        if result != Err(VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY) {
            self.started = true;
        }
        self.step(result)
    }

    /// Decodes as far as `data`, the whole file so far, reaches, without
    /// copying it (C: `WebPIUpdate`). Each call passes the previous data and
    /// more; the data may move between calls.
    ///
    /// # Errors
    ///
    /// The errors of [`IncrementalDecoder::append`];
    /// [`DecodeError::InvalidParam`] also for data shorter than before, or
    /// after `append`.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, IncrementalDecoder, Status};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 16];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 4, height: 4, stride: 16 })?;
    ///
    /// let config = Config::new(ColorMode::Rgba);
    /// let mut decoder = IncrementalDecoder::new(&config)?;
    /// let mut received = Vec::new();
    /// for piece in file.chunks(7) {
    ///     received.extend_from_slice(piece);
    ///     let status = decoder.update(&received)?;
    ///     assert_eq!(status == Status::Complete, received.len() == file.len());
    /// }
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn update(&mut self, data: &[u8]) -> Result<Status, DecodeError> {
        self.started = true;
        let result = webp_i_update(&mut self.idec, data);
        self.step(result)
    }

    /// The width and height of the output, once libwebp has set the output up
    /// after the header (C: the width and height of the output buffer that
    /// `WebPIDecodedArea` returns, not its height out-parameter `last_y`,
    /// which [`decoded_rows`](Self::decoded_rows) reports);
    /// `None` before, including the moment after a lossless header when the
    /// decoded area exists with a width of 0.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, IncrementalDecoder};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 6 * 6];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 6, height: 6, stride: 24 })?;
    ///
    /// let config = Config::new(ColorMode::Rgb).scale(3, 2);
    /// let mut decoder = IncrementalDecoder::new(&config)?;
    /// assert_eq!(decoder.output_size(), None);
    /// decoder.append(&file)?;
    /// assert_eq!(decoder.output_size(), Some((3, 2)));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn output_size(&self) -> Option<(u32, u32)> {
        let buffer = webp_i_decoded_area(&self.idec, None, None, None, None)?;
        let width = u32::try_from(buffer.width)
            .ok()
            .filter(|&width| width > 0)?;
        let height = u32::try_from(buffer.height)
            .ok()
            .filter(|&height| height > 0)?;
        Some((width, height))
    }

    /// The number of output rows decoded so far, from the top of the
    /// unflipped output (C: `last_y`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, IncrementalDecoder};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 2 * 3];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 3, stride: 8 })?;
    ///
    /// let config = Config::new(ColorMode::Rgba);
    /// let mut decoder = IncrementalDecoder::new(&config)?;
    /// decoder.append(&file)?;
    /// assert_eq!(decoder.decoded_rows(), 3);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn decoded_rows(&self) -> u32 {
        let mut rows = 0;
        let _ = webp_i_decoded_area(&self.idec, None, None, None, Some(&mut rows));
        u32::try_from(rows).unwrap_or(0)
    }

    /// The rows of an RGB mode decoded so far, lent until the next call that
    /// takes `&mut self` (C: `WebPIDecGetRGB`); `None` for a YUV mode or before
    /// the output exists. Row 0 is the first decoded row: while a flipped
    /// decoding runs, that is the bottom row of the output and the stride is
    /// negative; once complete, the stride is positive and row 0 is the top.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, IncrementalDecoder};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [4_u8, 3, 2, 1].repeat(4);
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 2, stride: 8 })?;
    ///
    /// let config = Config::new(ColorMode::Rgba);
    /// let mut decoder = IncrementalDecoder::new(&config)?;
    /// assert!(decoder.rows().is_none());
    /// decoder.append(&file)?;
    /// assert_eq!(decoder.rows().unwrap().row(1), Some(&[4_u8, 3, 2, 1, 4, 3, 2, 1][..]));
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn rows(&self) -> Option<Rows<'_>> {
        let (mut last_y, mut width, mut stride) = (0, 0, 0);
        let (region, first) = webp_i_dec_get_rgb(
            &self.idec,
            Some(&mut last_y),
            Some(&mut width),
            None,
            Some(&mut stride),
        )?;
        let row_len = usize::try_from(width)
            .ok()?
            .checked_mul(self.mode.bytes_per_pixel()?)?;
        Some(Rows::new(region, first, stride, row_len, last_y))
    }

    /// The planes of a YUV mode decoded so far, lent like
    /// [`IncrementalDecoder::rows`] (C: `WebPIDecGetYUVA`); `None` for an RGB
    /// mode or before the output exists. Luma and alpha have
    /// [`IncrementalDecoder::decoded_rows`] rows, U and V half as many,
    /// rounded up; with a scale, libwebp writes chroma and alpha rows later
    /// than luma rows, so those are final only once the decoding completes.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, IncrementalDecoder};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8, 0, 0, 255].repeat(9);
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 3, stride: 12 })?;
    ///
    /// let config = Config::new(ColorMode::Yuv);
    /// let mut decoder = IncrementalDecoder::new(&config)?;
    /// decoder.append(&file)?;
    /// let planes = decoder.planes().unwrap();
    /// assert_eq!((planes.y.len(), planes.u.len()), (3, 2));
    /// assert_eq!(planes.y.row(0), Some(&[16_u8, 16, 16][..])); // black, limited range
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn planes(&self) -> Option<YuvaRows<'_>> {
        let (mut last_y, mut width, mut stride, mut uv_stride, mut a_stride) = (0, 0, 0, 0, 0);
        let (mut u, mut v, mut a) = (None, None, None);
        let (y_region, y_first) = webp_i_dec_get_yuva(
            &self.idec,
            Some(&mut last_y),
            Some(&mut u),
            Some(&mut v),
            Some(&mut a),
            Some(&mut width),
            None,
            Some(&mut stride),
            Some(&mut uv_stride),
            Some(&mut a_stride),
        )?;
        let width = usize::try_from(width).ok()?;
        let (uv_width, uv_rows) = (width.div_ceil(2), (last_y + 1) / 2);
        let ((u_region, u_first), (v_region, v_first)) = (u?, v?);
        Some(YuvaRows {
            y: Rows::new(y_region, y_first, stride, width, last_y),
            u: Rows::new(u_region, u_first, uv_stride, uv_width, uv_rows),
            v: Rows::new(v_region, v_first, uv_stride, uv_width, uv_rows),
            a: a.map(|(region, first)| Rows::new(region, first, a_stride, width, last_y)),
        })
    }

    /// The decoded image, in the memory that libwebp allocated for a decoder
    /// of [`IncrementalDecoder::new`], once the decoding has completed.
    ///
    /// # Errors
    ///
    /// [`DecodeError::Suspended`] before the decoding has completed, and
    /// [`DecodeError::OutputMismatch`] for a decoder of
    /// [`IncrementalDecoder::with_output`], whose samples are in the caller's
    /// memory.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{ColorMode, Config, DecodeError, IncrementalDecoder};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 2, stride: 8 })?;
    ///
    /// let config = Config::new(ColorMode::Rgba);
    /// let mut decoder = IncrementalDecoder::new(&config)?;
    /// decoder.append(&file[..file.len() / 2])?;
    /// assert_eq!(decoder.into_image().unwrap_err(), DecodeError::Suspended);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn into_image(mut self) -> Result<Image, DecodeError> {
        if !self.library_memory {
            return Err(DecodeError::OutputMismatch { mode: self.mode });
        }
        if !self.complete {
            return Err(DecodeError::Suspended);
        }
        let buffer = std::mem::take(self.idec.own_output_mut());
        Image::from_buffer(self.mode, buffer)
    }
}

//------------------------------------------------------------------------------
// Container

/// The payload of a chunk iterator: the demuxer always borrows it from the
/// input, which is what the API lends, so `None` is a backstop the API does
/// not reach.
fn payload<'s>(bytes: &Cow<'s, [u8]>) -> Option<&'s [u8]> {
    match bytes {
        Cow::Borrowed(bytes) => Some(bytes),
        Cow::Owned(_) => None,
    }
}

/// The error of a demuxer that did not parse `data`: the error of
/// [`Features::parse`] when it fails (libwebp's demuxer reports a file that
/// is not WebP as a header in progress), then [`DecodeError::NotEnoughData`]
/// when a partial parse succeeds or asks for more data, and
/// [`DecodeError::BitstreamError`] otherwise.
fn demux_error(data: &[u8]) -> DecodeError {
    if let Err(error) = Features::parse(data) {
        return error;
    }
    match demux_bytes(data, true, WEBP_DEMUX_ABI_VERSION) {
        (Some(_), _) | (None, WebPDemuxState::WEBP_DEMUX_PARSING_HEADER) => {
            DecodeError::NotEnoughData
        }
        (None, _) => DecodeError::BitstreamError,
    }
}

/// The container of a WebP file: its metadata chunks and the canvas and
/// animation parameters, through libwebp's demuxer (C: `WebPDemuxer`).
///
/// It borrows the file's bytes; the chunk payloads borrow the container.
///
/// # Examples
///
/// ```
/// use webp::api::decode::Container;
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # fn chunk(tag: &[u8], payload: &[u8]) -> Vec<u8> {
/// #     let mut bytes = tag.to_vec();
/// #     bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
/// #     bytes.extend_from_slice(payload);
/// #     if payload.len() % 2 == 1 { bytes.push(0); }
/// #     bytes
/// # }
/// # let rgba = [0_u8, 0, 0, 255].repeat(4);
/// # let image = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 2, stride: 8 })?;
/// # let vp8x = chunk(b"VP8X", &[0x20 | 0x08, 0, 0, 0, 1, 0, 0, 1, 0, 0]);
/// # let body = [vp8x, chunk(b"ICCP", b"profile"), image[12..].to_vec(), chunk(b"EXIF", b"Exif\0\0MM")].concat();
/// # let mut file = b"RIFF".to_vec();
/// # file.extend_from_slice(&(body.len() as u32 + 4).to_le_bytes());
/// # file.extend_from_slice(b"WEBP");
/// # file.extend_from_slice(&body);
///
/// // `file` is an extended 2 by 2 WebP file with an ICC profile and EXIF data.
/// let container = Container::new(&file)?;
/// assert_eq!((container.canvas_width(), container.canvas_height()), (2, 2));
/// assert_eq!(container.icc_profile(), Some(&b"profile"[..]));
/// assert_eq!(container.exif(), Some(&b"Exif\0\0MM"[..]));
/// assert_eq!(container.xmp(), None);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Container<'d> {
    demux: WebPDemuxer<'d>,
    state: WebPDemuxState,
}

impl fmt::Debug for Container<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Container")
            .field("complete", &self.is_complete())
            .field("canvas_width", &self.canvas_width())
            .field("canvas_height", &self.canvas_height())
            .field("frame_count", &self.frame_count())
            .finish_non_exhaustive()
    }
}

impl<'d> Container<'d> {
    /// Parses a complete WebP file (C: `WebPDemux`).
    ///
    /// # Errors
    ///
    /// The error of [`Features::parse`] for data that is not a WebP file or
    /// ends inside its header; then [`DecodeError::NotEnoughData`] for a file
    /// that ends early and [`DecodeError::BitstreamError`] for one that does
    /// not parse.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{Container, DecodeError};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 9];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 3, stride: 12 })?;
    ///
    /// assert!(Container::new(&file)?.is_complete());
    /// assert_eq!(Container::new(&file[..file.len() - 2]).unwrap_err(), DecodeError::NotEnoughData);
    /// assert_eq!(Container::new(b"not a WebP file").unwrap_err(), DecodeError::BitstreamError);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(data: &'d [u8]) -> Result<Self, DecodeError> {
        match demux_bytes(data, false, WEBP_DEMUX_ABI_VERSION) {
            (Some(demux), state) => Ok(Self { demux, state }),
            (None, _) => Err(demux_error(data)),
        }
    }

    /// Parses the start of a WebP file (C: `WebPDemuxPartial`); the chunks
    /// and frames that the data reaches are available, which is enough for an
    /// `ICCP` chunk, which precedes the image data.
    ///
    /// # Errors
    ///
    /// [`DecodeError::NotEnoughData`] when the data ends inside the file
    /// header, [`DecodeError::BitstreamError`] when it does not parse; as for
    /// [`Container::new`], the error of [`Features::parse`] comes first.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 9];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 3, stride: 12 })?;
    ///
    /// let container = Container::partial(&file[..file.len() - 2])?;
    /// assert!(!container.is_complete());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn partial(data: &'d [u8]) -> Result<Self, DecodeError> {
        match demux_bytes(data, true, WEBP_DEMUX_ABI_VERSION) {
            (Some(demux), state) => Ok(Self { demux, state }),
            (None, _) => Err(demux_error(data)),
        }
    }

    /// Whether the whole file was parsed (C: `WEBP_DEMUX_DONE`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert!(Container::partial(&file)?.is_complete());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn is_complete(&self) -> bool {
        self.state == WebPDemuxState::WEBP_DEMUX_DONE
    }

    /// One of the demuxer's values (C: `WebPDemuxGetI`).
    fn get(&self, feature: WebPFormatFeature) -> u32 {
        webp_demux_get_i(&self.demux, feature)
    }

    /// The canvas width in pixels: the image's for a simple file.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 6];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 })?;
    ///
    /// assert_eq!(Container::new(&file)?.canvas_width(), 3);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn canvas_width(&self) -> u32 {
        self.get(WebPFormatFeature::WEBP_FF_CANVAS_WIDTH)
    }

    /// The canvas height in pixels: the image's for a simple file.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 6];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 })?;
    ///
    /// assert_eq!(Container::new(&file)?.canvas_height(), 2);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn canvas_height(&self) -> u32 {
        self.get(WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT)
    }

    /// Whether the file declares alpha (C: `ALPHA_FLAG` of
    /// `WEBP_FF_FORMAT_FLAGS`, which the demuxer also sets for a simple
    /// file whose image has alpha).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8, 0, 0, 100];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert!(Container::new(&file)?.has_alpha());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn has_alpha(&self) -> bool {
        self.get(WebPFormatFeature::WEBP_FF_FORMAT_FLAGS) & WebPFeatureFlags::ALPHA_FLAG.0 != 0
    }

    /// Whether the file is an animation (C: `ANIMATION_FLAG` of
    /// `WEBP_FF_FORMAT_FLAGS`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert!(!Container::new(&file)?.has_animation());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn has_animation(&self) -> bool {
        self.get(WebPFormatFeature::WEBP_FF_FORMAT_FLAGS) & WebPFeatureFlags::ANIMATION_FLAG.0 != 0
    }

    /// The number of times an animation loops, 0 for forever (C:
    /// `WEBP_FF_LOOP_COUNT`); a still image reports 1, the demuxer's default.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(Container::new(&file)?.loop_count(), 1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn loop_count(&self) -> u32 {
        self.get(WebPFormatFeature::WEBP_FF_LOOP_COUNT)
    }

    /// The background color of an animation as `0xAARRGGBB`: the `ANIM`
    /// chunk's bytes (blue, green, red, alpha) read as a little-endian `u32`
    /// (C: `WEBP_FF_BACKGROUND_COLOR`). It is reported, not applied: the
    /// canvas of [`AnimationDecoder`] starts transparent black. A still image
    /// reports `0xFFFFFFFF`.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(Container::new(&file)?.background_color(), 0xFFFF_FFFF);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn background_color(&self) -> u32 {
        self.get(WebPFormatFeature::WEBP_FF_BACKGROUND_COLOR)
    }

    /// The number of frames: 1 for a still image (C: `WEBP_FF_FRAME_COUNT`);
    /// the frames parsed so far for a partial container.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(Container::new(&file)?.frame_count(), 1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn frame_count(&self) -> u32 {
        self.get(WebPFormatFeature::WEBP_FF_FRAME_COUNT)
    }

    /// The payload of the first chunk with `fourcc` (C: `WebPDemuxGetChunk`
    /// with chunk number 1).
    fn first_chunk(&self, fourcc: [u8; 4]) -> Option<&[u8]> {
        let iter = webp_demux_get_chunk(&self.demux, fourcc, 1)?;
        payload(&iter.chunk.bytes)
    }

    /// The raw ICC profile: the payload of the first `ICCP` chunk, which the
    /// demuxer keeps when the `VP8X` flags announce it.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// // A simple file has no metadata; readers then assume sRGB.
    /// assert_eq!(Container::new(&file)?.icc_profile(), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn icc_profile(&self) -> Option<&[u8]> {
        self.first_chunk(*b"ICCP")
    }

    /// The raw EXIF data: the payload of the first `EXIF` chunk, unparsed
    /// (the orientation, if any, is in it).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(Container::new(&file)?.exif(), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn exif(&self) -> Option<&[u8]> {
        self.first_chunk(*b"EXIF")
    }

    /// The raw XMP packet: the payload of the first `XMP ` chunk.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(Container::new(&file)?.xmp(), None);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn xmp(&self) -> Option<&[u8]> {
        self.first_chunk(*b"XMP ")
    }

    /// The payloads of every chunk with `fourcc` that the demuxer keeps
    /// (`ICCP`, `EXIF`, `XMP ` and unknown chunks), in file order (C:
    /// `WebPDemuxGetChunk` and `WebPDemuxNextChunk`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::Container;
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(Container::new(&file)?.chunks(*b"EXIF").count(), 0);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn chunks(&self, fourcc: [u8; 4]) -> Chunks<'_> {
        Chunks {
            iter: webp_demux_get_chunk(&self.demux, fourcc, 1),
            started: false,
        }
    }
}

/// The payloads of the chunks with one tag; see [`Container::chunks`].
///
/// # Examples
///
/// ```
/// use webp::api::decode::{Chunks, Container};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [0_u8; 4];
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
///
/// let container = Container::new(&file)?;
/// let mut chunks: Chunks<'_> = container.chunks(*b"XMP ");
/// assert_eq!(chunks.next(), None);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct Chunks<'s> {
    iter: Option<WebPChunkIterator<'s>>,
    started: bool,
}

impl fmt::Debug for Chunks<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Chunks")
            .field("chunk_num", &self.iter.as_ref().map(|iter| iter.chunk_num))
            .field(
                "num_chunks",
                &self.iter.as_ref().map(|iter| iter.num_chunks),
            )
            .finish()
    }
}

impl<'s> Iterator for Chunks<'s> {
    type Item = &'s [u8];

    fn next(&mut self) -> Option<&'s [u8]> {
        let iter = self.iter.as_mut()?;
        if self.started && !webp_demux_next_chunk(iter) {
            self.iter = None;
            return None;
        }
        self.started = true;
        payload(&iter.chunk.bytes)
    }
}

//------------------------------------------------------------------------------
// Animation

/// The color mode of an animation's canvas: the four modes that libwebp's
/// animation decoder supports.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{AnimationMode, AnimationOptions};
///
/// assert_eq!(AnimationOptions::default().mode, AnimationMode::Rgba);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AnimationMode {
    /// Red, green, blue, straight alpha (C: `MODE_RGBA`).
    #[default]
    Rgba,
    /// Blue, green, red, straight alpha (C: `MODE_BGRA`).
    Bgra,
    /// Red, green, blue, premultiplied alpha (C: `MODE_rgbA`); the frames are
    /// blended premultiplied.
    PremultipliedRgba,
    /// Blue, green, red, premultiplied alpha (C: `MODE_bgrA`).
    PremultipliedBgra,
}

impl AnimationMode {
    /// The C colorspace.
    const fn raw(self) -> WEBP_CSP_MODE {
        match self {
            Self::Rgba => WEBP_CSP_MODE::MODE_RGBA,
            Self::Bgra => WEBP_CSP_MODE::MODE_BGRA,
            Self::PremultipliedRgba => WEBP_CSP_MODE::MODE_rgbA,
            Self::PremultipliedBgra => WEBP_CSP_MODE::MODE_bgrA,
        }
    }
}

/// The options of an [`AnimationDecoder`] (C: `WebPAnimDecoderOptions`).
///
/// # Examples
///
/// ```
/// use webp::api::decode::{AnimationMode, AnimationOptions};
///
/// let options = AnimationOptions { mode: AnimationMode::PremultipliedBgra, multithreaded: true };
/// assert!(options.multithreaded);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub struct AnimationOptions {
    /// The color mode of the canvas, [`AnimationMode::Rgba`] by default.
    pub mode: AnimationMode,
    /// Decodes lossy frames with a second thread (C: `use_threads`); the
    /// canvases do not change.
    pub multithreaded: bool,
}

/// The parameters of an animation (C: `WebPAnimInfo`).
///
/// # Examples
///
/// ```
/// use webp::api::decode::{AnimationDecoder, AnimationOptions};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [0_u8; 4 * 6];
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 })?;
///
/// // A still image is an animation of one frame.
/// let info = AnimationDecoder::new(&file, AnimationOptions::default())?.info();
/// assert_eq!((info.canvas_width, info.canvas_height, info.frame_count), (3, 2, 1));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct AnimationInfo {
    /// The canvas width in pixels.
    pub canvas_width: u32,
    /// The canvas height in pixels.
    pub canvas_height: u32,
    /// The number of times the animation loops, 0 for forever (1 for a still
    /// image).
    pub loop_count: u32,
    /// The background color as `0xAARRGGBB` (the `ANIM` chunk's bytes read as
    /// a little-endian `u32`), reported and not applied; see
    /// [`Container::background_color`].
    pub background_color: u32,
    /// The number of frames.
    pub frame_count: u32,
}

/// A decoded frame: the whole canvas after the frame was blended and the
/// previous one disposed, and the frame's end time.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{AnimationDecoder, AnimationOptions, Frame};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # let rgba = [1_u8, 2, 3, 4].repeat(2);
/// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 1, stride: 8 })?;
///
/// let mut decoder = AnimationDecoder::new(&file, AnimationOptions::default())?;
/// let frame: Frame<'_> = decoder.next_frame()?.expect("one frame");
/// assert_eq!((frame.canvas, frame.stride, frame.timestamp_ms), (&[1_u8, 2, 3, 4, 1, 2, 3, 4][..], 8, 0));
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
#[derive(Clone, Copy)]
pub struct Frame<'s> {
    /// The canvas: `canvas_height` rows of `canvas_width` pixels of 4 bytes.
    pub canvas: &'s [u8],
    /// The distance between canvas rows, in bytes: `4 * canvas_width`.
    pub stride: usize,
    /// The time at which the frame ends, in milliseconds: the sum of the
    /// durations so far (0 for a still image).
    pub timestamp_ms: i32,
}

impl fmt::Debug for Frame<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Frame")
            .field("canvas_len", &self.canvas.len())
            .field("stride", &self.stride)
            .field("timestamp_ms", &self.timestamp_ms)
            .finish()
    }
}

/// A decoder of a WebP animation into full canvases (C: `WebPAnimDecoder`).
///
/// It borrows the file's bytes. A still image is an animation of one frame.
/// Each [`AnimationDecoder::next_frame`] lends the canvas until the next call.
/// The canvas starts transparent black; the background color is reported in
/// [`AnimationInfo`], not applied.
///
/// # Examples
///
/// ```
/// use webp::api::decode::{AnimationDecoder, AnimationOptions};
/// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
/// # fn chunk(tag: &[u8], payload: &[u8]) -> Vec<u8> {
/// #     let mut bytes = tag.to_vec();
/// #     bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
/// #     bytes.extend_from_slice(payload);
/// #     if payload.len() % 2 == 1 { bytes.push(0); }
/// #     bytes
/// # }
/// # fn frame(red: u8, duration: u8) -> Vec<u8> {
/// #     let rgba = [red, 0, 0, 255].repeat(4);
/// #     let image = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 2, height: 2, stride: 8 }).unwrap();
/// #     let mut payload = vec![0, 0, 0, 0, 0, 0, 1, 0, 0, 1, 0, 0, duration, 0, 0, 0];
/// #     payload.extend_from_slice(&image[12..]);
/// #     chunk(b"ANMF", &payload)
/// # }
/// # let vp8x = chunk(b"VP8X", &[0x02, 0, 0, 0, 1, 0, 0, 1, 0, 0]);
/// # let anim = chunk(b"ANIM", &[0x10, 0x20, 0x30, 0xff, 3, 0]);
/// # let body = [vp8x, anim, frame(100, 40), frame(200, 60)].concat();
/// # let mut file = b"RIFF".to_vec();
/// # file.extend_from_slice(&(body.len() as u32 + 4).to_le_bytes());
/// # file.extend_from_slice(b"WEBP");
/// # file.extend_from_slice(&body);
///
/// // `file` is a 2 by 2 animation of a red frame for 40 ms and a brighter one
/// // for 60 ms, looping 3 times.
/// let mut decoder = AnimationDecoder::new(&file, AnimationOptions::default())?;
/// let info = decoder.info();
/// assert_eq!((info.frame_count, info.loop_count, info.background_color), (2, 3, 0xff30_2010));
/// let mut frames = Vec::new();
/// while let Some(frame) = decoder.next_frame()? {
///     frames.push((frame.canvas[0], frame.timestamp_ms));
/// }
/// assert_eq!(frames, [(100, 40), (200, 100)]);
/// # Ok::<(), Box<dyn std::error::Error>>(())
/// ```
pub struct AnimationDecoder<'d> {
    dec: WebPAnimDecoder<'d>,
}

impl fmt::Debug for AnimationDecoder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnimationDecoder")
            .field("info", &self.info())
            .field("has_more_frames", &self.has_more_frames())
            .finish_non_exhaustive()
    }
}

impl<'d> AnimationDecoder<'d> {
    /// Parses the file and prepares its decoding (C: `WebPAnimDecoderNew`).
    ///
    /// This allocates two canvases of `4 * canvas_width * canvas_height` bytes
    /// and zeroes them; a small file can declare a canvas of up to 2^32
    /// pixels. To budget that memory before it is allocated, read the canvas
    /// size first with [`Features::parse`] or [`Container::canvas_width`] and
    /// [`Container::canvas_height`].
    ///
    /// # Errors
    ///
    /// The errors of [`Features::parse`]; [`DecodeError::NotEnoughData`] for
    /// a file that ends early and [`DecodeError::BitstreamError`] for one that
    /// does not parse; [`DecodeError::OutOfMemory`] when the canvases cannot
    /// be allocated.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{AnimationDecoder, AnimationMode, AnimationOptions, Features};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4 * 6];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 })?;
    ///
    /// let features = Features::parse(&file)?;
    /// let canvases = 2 * 4 * u64::from(features.width) * u64::from(features.height);
    /// assert!(canvases <= 1 << 20, "within the budget");
    /// let options = AnimationOptions { mode: AnimationMode::Bgra, multithreaded: false };
    /// let decoder = AnimationDecoder::new(&file, options)?;
    /// # let _ = decoder;
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn new(data: &'d [u8], options: AnimationOptions) -> Result<Self, DecodeError> {
        let mut raw = WebPAnimDecoderOptions::default();
        let initialized = webp_anim_decoder_options_init(&mut raw);
        debug_assert!(initialized, "the crate's own ABI version matches");
        raw.color_mode = options.mode.raw();
        raw.use_threads = i32::from(options.multithreaded);
        if let Some(dec) = anim_decoder_new_bytes(data, Some(&raw), WEBP_DEMUX_ABI_VERSION) {
            return Ok(Self { dec });
        }
        // libwebp reports no status: the features, then the demuxer, then the
        // canvases can fail, the options being valid.
        Features::parse(data)?;
        if demux_bytes(data, false, WEBP_DEMUX_ABI_VERSION).0.is_none() {
            return Err(demux_error(data));
        }
        Err(DecodeError::OutOfMemory)
    }

    /// The animation's parameters (C: `WebPAnimDecoderGetInfo`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{AnimationDecoder, AnimationOptions};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// assert_eq!(AnimationDecoder::new(&file, AnimationOptions::default())?.info().loop_count, 1);
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn info(&self) -> AnimationInfo {
        let info = webp_anim_decoder_get_info(&self.dec);
        AnimationInfo {
            canvas_width: info.canvas_width,
            canvas_height: info.canvas_height,
            loop_count: info.loop_count,
            background_color: info.bgcolor,
            frame_count: info.frame_count,
        }
    }

    /// Whether frames remain to be decoded (C:
    /// `WebPAnimDecoderHasMoreFrames`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{AnimationDecoder, AnimationOptions};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// let mut decoder = AnimationDecoder::new(&file, AnimationOptions::default())?;
    /// assert!(decoder.has_more_frames());
    /// decoder.next_frame()?;
    /// assert!(!decoder.has_more_frames());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    #[must_use]
    pub fn has_more_frames(&self) -> bool {
        webp_anim_decoder_has_more_frames(&self.dec)
    }

    /// Decodes the next frame and lends the canvas until the next call (C:
    /// `WebPAnimDecoderGetNext`); `Ok(None)` after the last frame.
    ///
    /// # Errors
    ///
    /// [`DecodeError::FrameFailed`] when the frame cannot be decoded; the
    /// decoder stays at that frame.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{AnimationDecoder, AnimationOptions};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [8_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// let mut decoder = AnimationDecoder::new(&file, AnimationOptions::default())?;
    /// assert_eq!(decoder.next_frame()?.map(|frame| frame.canvas.to_vec()), Some(vec![8_u8; 4]));
    /// assert!(decoder.next_frame()?.is_none());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn next_frame(&mut self) -> Result<Option<Frame<'_>>, DecodeError> {
        if !self.has_more_frames() {
            return Ok(None);
        }
        let stride = usize::try_from(self.info().canvas_width)
            .unwrap_or(usize::MAX)
            .saturating_mul(4);
        match webp_anim_decoder_get_next(&mut self.dec) {
            Some((canvas, timestamp_ms)) => Ok(Some(Frame {
                canvas,
                stride,
                timestamp_ms,
            })),
            None => Err(DecodeError::FrameFailed),
        }
    }

    /// Restarts from the first frame (C: `WebPAnimDecoderReset`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::decode::{AnimationDecoder, AnimationOptions};
    /// # use webp::api::encode::{encode_lossless, PixelLayout, PixelRows};
    /// # let rgba = [0_u8; 4];
    /// # let file = encode_lossless(PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 1, height: 1, stride: 4 })?;
    ///
    /// let mut decoder = AnimationDecoder::new(&file, AnimationOptions::default())?;
    /// decoder.next_frame()?;
    /// decoder.reset();
    /// assert!(decoder.has_more_frames());
    /// # Ok::<(), Box<dyn std::error::Error>>(())
    /// ```
    pub fn reset(&mut self) {
        webp_anim_decoder_reset(&mut self.dec);
    }
}
