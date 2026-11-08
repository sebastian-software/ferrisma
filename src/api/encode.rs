//! Safe Rust API over the encoder of the translation; no upstream unit.
//!
//! Wraps the translated types and functions in validated Rust types and adds
//! no codec behavior of its own; see `docs/adr/`.
//!
//! Work package: WP17 (safe encoder API).
//!
//! The decision record is `docs/adr/0006-safe-encoder-api.md`. Every encoding
//! calls the translated `WebPEncode` ([`webp_encode`]) with the stored
//! [`WebPConfig`], so its output is byte-identical to libwebp v1.6.0 with the
//! same configuration and samples. The API validates dimensions, strides,
//! buffer lengths and floating-point parameters first, so that caller input
//! never reaches a panic of the translation, and reports failures as
//! [`EncodeError`].
//!
//! # Example
//!
//! ```
//! use webp::api::encode::{Config, Encoder, Picture, PixelLayout, PixelRows, SampleForm};
//!
//! // A 4 by 2 RGBA picture, rows packed.
//! let rgba = [255_u8, 0, 0, 255].repeat(8);
//! let rows = PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 4, height: 2, stride: 16 };
//! let mut picture = Picture::import(rows, SampleForm::Argb)?;
//! let config = Config::new().quality(80.0).method(4);
//! let webp = Encoder::new(&config).encode(&mut picture)?;
//! assert_eq!(&webp[..4], b"RIFF");
//! assert_eq!(&webp[8..12], b"WEBP");
//! # Ok::<(), webp::api::encode::EncodeError>(())
//! ```
//!
//! # Pictures and memory
//!
//! [`Picture::import`] copies strided rows into planes that the picture
//! allocates. [`Picture::from_argb`] and [`Picture::from_yuva`] borrow planes
//! that the caller allocated, exclusively for the picture's lifetime, so that
//! the caller can account for that memory; libwebp writes into them where the
//! semantics below say so. Strides count elements of the slice's type: bytes
//! for `&[u8]` rows and planes, pixels for the `&mut [u32]` ARGB plane. The
//! encoder converts what it needs: a lossy encoding of an ARGB picture
//! allocates YUVA planes, a lossless encoding of a YUVA picture an ARGB plane.
//! Those planes stay with the picture and [`Picture::owned_sample_bytes`]
//! counts them; the encoder's internal state is not observable.
//!
//! # Semantics
//!
//! - **Lossy samples** are ITU-R BT.601 `Y'CbCr` 4:2:0 in limited range (Y 16 to
//!   235, Cb and Cr 16 to 240), converted with the fixed-point constants of
//!   libwebp's `src/dsp/yuv.h` and gamma-corrected chroma averaging, which is
//!   weighted by alpha in blocks with transparency; a decoder converts back
//!   with the same matrix. Sharp YUV ([`Config::sharp_yuv`]) and dithering
//!   (`preprocessing` bit 2, through [`Config::raw_mut`]) apply only when the
//!   picture holds ARGB samples at encoding time ([`SampleForm::Argb`] or
//!   [`Picture::from_argb`]); [`SampleForm::Yuva`] converts during the import,
//!   without them, as libwebp does. Planes from [`Picture::from_yuva`] are
//!   encoded as they are.
//! - **Alpha** is straight (not premultiplied); the encoder has no
//!   premultiplied input. Unless [`Config::exact`] is set, the encoder rewrites
//!   the samples under fully transparent areas in the picture's planes, the
//!   caller's included: in 8 by 8 blocks of the Y, U and V planes for a lossy
//!   encoding, and every transparent ARGB pixel becomes 0 for a lossless one.
//!   `show_compressed` (through [`Config::raw_mut`]) writes the decoded samples
//!   back into the Y, U and V planes.
//! - **Metadata**: the output is a still WebP file with no `ICCP`, `EXIF` or
//!   `XMP` chunk; WebP carries no other color or orientation signal. Metadata
//!   and animation belong to the muxer (WP15).

use std::fmt;

use crate::enc::config_enc::{webp_config_lossless_preset, webp_validate_config};
use crate::enc::picture_csp_enc::{
    webp_picture_import_bgr, webp_picture_import_bgra, webp_picture_import_bgrx,
    webp_picture_import_rgb, webp_picture_import_rgba, webp_picture_import_rgbx,
};
use crate::enc::picture_enc::{
    LOSSLESS_DEFAULT_QUALITY, webp_memory_write, webp_memory_writer_init,
};
use crate::enc::webp_enc::webp_encode;
pub use crate::webp::encode::WebPAuxStats;
use crate::webp::encode::{
    WEBP_MAX_DIMENSION, WebPConfig, WebPEncCSP, WebPEncodingError, WebPImageHint, WebPMemoryWriter,
    WebPPicture, WebPPreset, webp_config_init, webp_config_preset, webp_picture_init,
};
use crate::webp::types::{PlaneOffset, PlaneStorage};

//------------------------------------------------------------------------------
// Enumerations

/// A predefined configuration set, tuned for a kind of content (C:
/// `WebPPreset`).
///
/// # Examples
///
/// ```
/// use webp::api::encode::{Config, Preset};
///
/// let config = Config::preset(Preset::Photo, 80.0)?;
/// assert_eq!(config.as_raw().quality, 80.0);
/// # Ok::<(), webp::api::encode::EncodeError>(())
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum Preset {
    /// The default preset (C: `WEBP_PRESET_DEFAULT`).
    #[default]
    Default,
    /// A digital picture, such as a portrait or an indoor shot
    /// (C: `WEBP_PRESET_PICTURE`).
    Picture,
    /// An outdoor photograph with natural lighting (C: `WEBP_PRESET_PHOTO`).
    Photo,
    /// A hand or line drawing with high-contrast details
    /// (C: `WEBP_PRESET_DRAWING`).
    Drawing,
    /// A small, colorful image (C: `WEBP_PRESET_ICON`).
    Icon,
    /// Text-like content (C: `WEBP_PRESET_TEXT`).
    Text,
}

impl Preset {
    const fn raw(self) -> WebPPreset {
        match self {
            Self::Default => WebPPreset::WEBP_PRESET_DEFAULT,
            Self::Picture => WebPPreset::WEBP_PRESET_PICTURE,
            Self::Photo => WebPPreset::WEBP_PRESET_PHOTO,
            Self::Drawing => WebPPreset::WEBP_PRESET_DRAWING,
            Self::Icon => WebPPreset::WEBP_PRESET_ICON,
            Self::Text => WebPPreset::WEBP_PRESET_TEXT,
        }
    }
}

/// A hint about the content that the lossless encoder uses
/// (C: `WebPImageHint`).
///
/// # Examples
///
/// ```
/// use webp::api::encode::{Config, ImageHint};
///
/// let config = Config::new().lossless(true).image_hint(ImageHint::Graph);
/// assert_eq!(config.as_raw().image_hint.0, 3);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum ImageHint {
    /// No hint (C: `WEBP_HINT_DEFAULT`).
    #[default]
    Default,
    /// A digital picture, such as a portrait or an indoor shot
    /// (C: `WEBP_HINT_PICTURE`).
    Picture,
    /// An outdoor photograph with natural lighting (C: `WEBP_HINT_PHOTO`).
    Photo,
    /// A discrete-tone image such as a graph or map (C: `WEBP_HINT_GRAPH`).
    Graph,
}

impl ImageHint {
    const fn raw(self) -> WebPImageHint {
        match self {
            Self::Default => WebPImageHint::WEBP_HINT_DEFAULT,
            Self::Picture => WebPImageHint::WEBP_HINT_PICTURE,
            Self::Photo => WebPImageHint::WEBP_HINT_PHOTO,
            Self::Graph => WebPImageHint::WEBP_HINT_GRAPH,
        }
    }
}

/// The predictive filter of the alpha plane in a lossy encoding
/// (C: `WebPConfig::alpha_filtering` 0, 1 and 2).
///
/// # Examples
///
/// ```
/// use webp::api::encode::{AlphaFilter, Config};
///
/// let config = Config::new().alpha_filter(AlphaFilter::Best);
/// assert_eq!(config.as_raw().alpha_filtering, 2);
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum AlphaFilter {
    /// No filter (C: 0).
    None,
    /// The fast filter, the default (C: 1).
    #[default]
    Fast,
    /// The best filter, slower (C: 2).
    Best,
}

impl AlphaFilter {
    const fn raw(self) -> i32 {
        match self {
            Self::None => 0,
            Self::Fast => 1,
            Self::Best => 2,
        }
    }
}

/// The order of the channels of packed rows, as libwebp's six importers read
/// them (C: `WebPPictureImportRGB` and its siblings).
///
/// # Examples
///
/// ```
/// use webp::api::encode::PixelLayout;
///
/// assert_eq!(PixelLayout::Bgr.bytes_per_pixel(), 3);
/// assert_eq!(PixelLayout::Rgbx.bytes_per_pixel(), 4);
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PixelLayout {
    /// Red, green, blue (C: `WebPPictureImportRGB`).
    Rgb,
    /// Red, green, blue, straight alpha (C: `WebPPictureImportRGBA`).
    Rgba,
    /// Red, green, blue and an ignored byte (C: `WebPPictureImportRGBX`).
    Rgbx,
    /// Blue, green, red (C: `WebPPictureImportBGR`).
    Bgr,
    /// Blue, green, red, straight alpha (C: `WebPPictureImportBGRA`).
    Bgra,
    /// Blue, green, red and an ignored byte (C: `WebPPictureImportBGRX`).
    Bgrx,
}

impl PixelLayout {
    /// The bytes of one pixel: 3 or 4.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::PixelLayout;
    ///
    /// assert_eq!(PixelLayout::Rgba.bytes_per_pixel(), 4);
    /// ```
    #[must_use]
    pub const fn bytes_per_pixel(self) -> usize {
        match self {
            Self::Rgb | Self::Bgr => 3,
            Self::Rgba | Self::Rgbx | Self::Bgra | Self::Bgrx => 4,
        }
    }

    fn importer(self) -> fn(&mut WebPPicture<'_>, &[u8], i32) -> bool {
        match self {
            Self::Rgb => webp_picture_import_rgb,
            Self::Rgba => webp_picture_import_rgba,
            Self::Rgbx => webp_picture_import_rgbx,
            Self::Bgr => webp_picture_import_bgr,
            Self::Bgra => webp_picture_import_bgra,
            Self::Bgrx => webp_picture_import_bgrx,
        }
    }
}

/// The samples that an imported picture keeps (C: `WebPPicture::use_argb`).
///
/// # Examples
///
/// ```
/// use webp::api::encode::{Picture, PixelLayout, PixelRows, SampleForm};
///
/// let rgb = [10_u8, 20, 30].repeat(4);
/// let rows = PixelRows { layout: PixelLayout::Rgb, data: &rgb, width: 2, height: 2, stride: 6 };
/// let yuva = Picture::import(rows, SampleForm::Yuva)?;
/// let argb = Picture::import(rows, SampleForm::Argb)?;
/// assert_eq!(yuva.owned_sample_bytes(), 4 + 1 + 1);
/// assert_eq!(argb.owned_sample_bytes(), 4 * 4);
/// # Ok::<(), webp::api::encode::EncodeError>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum SampleForm {
    /// Y, U and V planes (4:2:0), and an alpha plane when the rows are not
    /// opaque: what the lossy encoder reads. The conversion happens during the
    /// import, without sharp YUV or dithering.
    Yuva,
    /// One plane of ARGB pixels: what the lossless encoder reads. A lossy
    /// encoding converts it at encoding time, with sharp YUV or dithering
    /// when the configuration asks for them.
    Argb,
}

/// The input that an [`EncodeError`] refers to.
///
/// # Examples
///
/// ```
/// use webp::api::encode::{EncodeError, Picture, Plane};
///
/// let mut argb = vec![0_u32; 3];
/// let error = Picture::from_argb(&mut argb, 2, 2, 2).unwrap_err();
/// assert_eq!(error, EncodeError::BufferTooShort { plane: Plane::Argb, len: 3, needed: 4 });
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Plane {
    /// The packed rows of [`PixelRows`].
    Pixels,
    /// The luma plane of [`YuvaPlanes`].
    Y,
    /// The U chroma plane of [`YuvaPlanes`].
    U,
    /// The V chroma plane of [`YuvaPlanes`].
    V,
    /// The alpha plane of [`YuvaPlanes`].
    A,
    /// The ARGB plane of [`Picture::from_argb`].
    Argb,
}

//------------------------------------------------------------------------------
// Errors

/// Why an encoding or one of its inputs failed.
///
/// The variants up to [`EncodeError::Unreported`] are the codes that libwebp
/// records in `WebPPicture::error_code`, with C's rule that the first recorded
/// error is kept; [`EncodeError::code`] returns the C code. The others are the
/// API's own validation, which runs before the translation is called.
///
/// # Examples
///
/// ```
/// use webp::api::encode::{Config, EncodeError};
///
/// let error = Config::new().quality(f32::NAN).validate().unwrap_err();
/// assert_eq!(error, EncodeError::InvalidParameter { name: "quality" });
/// assert_eq!(error.code(), None);
/// assert_eq!(error.to_string(), "invalid parameter: quality");
/// ```
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[non_exhaustive]
pub enum EncodeError {
    /// Memory for the picture or the encoder could not be allocated
    /// (C: `VP8_ENC_ERROR_OUT_OF_MEMORY`).
    OutOfMemory,
    /// Memory for the bitstream could not be allocated
    /// (C: `VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY`).
    BitstreamOutOfMemory,
    /// A required plane is missing (C: `VP8_ENC_ERROR_NULL_PARAMETER`); the
    /// picture of an encoding whose callback panicked has no planes.
    NullParameter,
    /// The configuration fails libwebp's validation, or the encoder rejects a
    /// combination (C: `VP8_ENC_ERROR_INVALID_CONFIGURATION`).
    InvalidConfiguration,
    /// The picture's dimensions are rejected (C: `VP8_ENC_ERROR_BAD_DIMENSION`).
    BadDimension,
    /// The first partition exceeds 512 KiB (C:
    /// `VP8_ENC_ERROR_PARTITION0_OVERFLOW`); a lower quality or more segments
    /// help.
    Partition0Overflow,
    /// A token partition exceeds 16 MiB (C: `VP8_ENC_ERROR_PARTITION_OVERFLOW`).
    PartitionOverflow,
    /// The writer returned `false`, or the memory writer could not grow
    /// (C: `VP8_ENC_ERROR_BAD_WRITE`).
    BadWrite,
    /// The file would exceed 4 GiB (C: `VP8_ENC_ERROR_FILE_TOO_BIG`).
    FileTooBig,
    /// The progress hook returned `false` (C: `VP8_ENC_ERROR_USER_ABORT`).
    UserAbort,
    /// The encoder failed without recording a code (C returns 0 with
    /// `VP8_ENC_OK`; libwebp does so when `QuantizeLevels` runs out of memory).
    Unreported,
    /// The width or height is 0 or above 16383 (`WEBP_MAX_DIMENSION`).
    InvalidDimensions {
        /// The width given.
        width: u32,
        /// The height given.
        height: u32,
    },
    /// A stride is shorter than a row, longer than `i32::MAX`, or so long that
    /// the plane's extent does not fit in `usize`.
    InvalidStride {
        /// The input with the stride.
        plane: Plane,
        /// The stride given, in elements of the input's type.
        stride: usize,
    },
    /// A buffer is shorter than its rows: `(rows - 1) * stride + row length`
    /// elements.
    BufferTooShort {
        /// The input that is too short.
        plane: Plane,
        /// Its length, in elements.
        len: usize,
        /// The elements that its dimensions and stride address.
        needed: usize,
    },
    /// A parameter is outside the range the API accepts: `quality` (finite,
    /// 0 to 100), `target_psnr` (finite) or `lossless_level` (0 to 9).
    InvalidParameter {
        /// The parameter's name.
        name: &'static str,
    },
}

impl EncodeError {
    /// The `WebPEncodingError` that libwebp recorded for this error, or `None`
    /// for an error of the API's own validation. [`EncodeError::Unreported`]
    /// returns `VP8_ENC_OK`, the code C leaves in that case.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::EncodeError;
    /// use webp::webp::encode::WebPEncodingError;
    ///
    /// assert_eq!(EncodeError::UserAbort.code(), Some(WebPEncodingError::VP8_ENC_ERROR_USER_ABORT));
    /// assert_eq!(EncodeError::InvalidDimensions { width: 0, height: 1 }.code(), None);
    /// ```
    #[must_use]
    pub const fn code(&self) -> Option<WebPEncodingError> {
        Some(match self {
            Self::OutOfMemory => WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
            Self::BitstreamOutOfMemory => WebPEncodingError::VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY,
            Self::NullParameter => WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER,
            Self::InvalidConfiguration => WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION,
            Self::BadDimension => WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION,
            Self::Partition0Overflow => WebPEncodingError::VP8_ENC_ERROR_PARTITION0_OVERFLOW,
            Self::PartitionOverflow => WebPEncodingError::VP8_ENC_ERROR_PARTITION_OVERFLOW,
            Self::BadWrite => WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE,
            Self::FileTooBig => WebPEncodingError::VP8_ENC_ERROR_FILE_TOO_BIG,
            Self::UserAbort => WebPEncodingError::VP8_ENC_ERROR_USER_ABORT,
            Self::Unreported => WebPEncodingError::VP8_ENC_OK,
            Self::InvalidDimensions { .. }
            | Self::InvalidStride { .. }
            | Self::BufferTooShort { .. }
            | Self::InvalidParameter { .. } => return None,
        })
    }

    /// The error for the code that a failed call of the translation recorded.
    const fn from_code(code: WebPEncodingError) -> Self {
        match code {
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY => Self::OutOfMemory,
            WebPEncodingError::VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY => Self::BitstreamOutOfMemory,
            WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER => Self::NullParameter,
            WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION => Self::InvalidConfiguration,
            WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION => Self::BadDimension,
            WebPEncodingError::VP8_ENC_ERROR_PARTITION0_OVERFLOW => Self::Partition0Overflow,
            WebPEncodingError::VP8_ENC_ERROR_PARTITION_OVERFLOW => Self::PartitionOverflow,
            WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE => Self::BadWrite,
            WebPEncodingError::VP8_ENC_ERROR_FILE_TOO_BIG => Self::FileTooBig,
            WebPEncodingError::VP8_ENC_ERROR_USER_ABORT => Self::UserAbort,
            _ => Self::Unreported,
        }
    }
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OutOfMemory => f.write_str("out of memory"),
            Self::BitstreamOutOfMemory => f.write_str("out of memory for the bitstream"),
            Self::NullParameter => f.write_str("a required plane is missing"),
            Self::InvalidConfiguration => f.write_str("invalid configuration"),
            Self::BadDimension => f.write_str("invalid picture dimensions"),
            Self::Partition0Overflow => f.write_str("the first partition exceeds 512 KiB"),
            Self::PartitionOverflow => f.write_str("a token partition exceeds 16 MiB"),
            Self::BadWrite => f.write_str("the writer failed"),
            Self::FileTooBig => f.write_str("the file would exceed 4 GiB"),
            Self::UserAbort => f.write_str("aborted by the progress hook"),
            Self::Unreported => f.write_str("the encoder failed without an error code"),
            Self::InvalidDimensions { width, height } => write!(
                f,
                "invalid dimensions {width} by {height}: each must be 1 to {WEBP_MAX_DIMENSION}"
            ),
            Self::InvalidStride { plane, stride } => {
                write!(f, "invalid stride {stride} for the {plane:?} plane")
            }
            Self::BufferTooShort { plane, len, needed } => write!(
                f,
                "the {plane:?} plane holds {len} elements, its rows need {needed}"
            ),
            Self::InvalidParameter { name } => write!(f, "invalid parameter: {name}"),
        }
    }
}

impl std::error::Error for EncodeError {}

//------------------------------------------------------------------------------
// Configuration

/// The API's own check of a quality: finite and in 0 to 100.
fn check_quality(quality: f32) -> Result<(), EncodeError> {
    if quality.is_finite() && (0.0..=100.0).contains(&quality) {
        Ok(())
    } else {
        Err(EncodeError::InvalidParameter { name: "quality" })
    }
}

/// Compression parameters: a [`WebPConfig`] with typed setters.
///
/// The setters store their value in the matching C field and never fail;
/// [`Config::validate`], which every encoding calls first, reports a value
/// that is out of range. [`Config::raw_mut`] reaches the fields without a
/// setter (segments, filters, passes, partitions and the like).
///
/// # Examples
///
/// ```
/// use webp::api::encode::{AlphaFilter, Config, ImageHint, Preset};
///
/// let lossy = Config::preset(Preset::Picture, 82.0)?
///     .method(6)
///     .alpha_quality(90)
///     .alpha_filter(AlphaFilter::Best)
///     .sharp_yuv(true);
/// lossy.validate()?;
///
/// let lossless = Config::new().lossless_level(9)?.exact(true).image_hint(ImageHint::Photo);
/// assert_eq!(lossless.as_raw().lossless, 1);
/// # Ok::<(), webp::api::encode::EncodeError>(())
/// ```
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Config {
    raw: WebPConfig,
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

impl From<WebPConfig> for Config {
    /// Wraps a configuration of the translation as it is; [`Config::validate`]
    /// checks it.
    fn from(raw: WebPConfig) -> Self {
        Self { raw }
    }
}

impl Config {
    /// The default configuration: the default preset at quality 75, lossy
    /// (C: `WebPConfigInit`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!((config.as_raw().quality, config.as_raw().method), (75.0, 4));
    /// ```
    #[must_use]
    pub fn new() -> Self {
        let mut raw = WebPConfig::default();
        let valid = webp_config_init(&mut raw);
        debug_assert!(valid, "the default configuration is valid");
        Self { raw }
    }

    /// The configuration of `preset` at `quality` (C: `WebPConfigPreset`).
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidParameter`] when `quality` is not finite or
    /// outside 0 to 100.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, EncodeError, Preset};
    ///
    /// let config = Config::preset(Preset::Text, 90.0)?;
    /// assert_eq!(config.as_raw().quality, 90.0);
    /// assert_eq!(
    ///     Config::preset(Preset::Text, 100.5),
    ///     Err(EncodeError::InvalidParameter { name: "quality" })
    /// );
    /// # Ok::<(), EncodeError>(())
    /// ```
    pub fn preset(preset: Preset, quality: f32) -> Result<Self, EncodeError> {
        check_quality(quality)?;
        let mut raw = WebPConfig::default();
        if webp_config_preset(&mut raw, preset.raw(), quality) {
            Ok(Self { raw })
        } else {
            Err(EncodeError::InvalidConfiguration)
        }
    }

    /// Switches to lossless encoding with the method and quality of `level`,
    /// from 0 (fastest) to 9 (smallest), as `cwebp -z` does
    /// (C: `WebPConfigLosslessPreset`).
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidParameter`] when `level` is above 9.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// let config = Config::new().lossless_level(6)?;
    /// assert_eq!((config.as_raw().lossless, config.as_raw().method), (1, 4));
    /// assert!(Config::new().lossless_level(10).is_err());
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    pub fn lossless_level(mut self, level: u8) -> Result<Self, EncodeError> {
        if webp_config_lossless_preset(&mut self.raw, i32::from(level)) {
            Ok(self)
        } else {
            Err(EncodeError::InvalidParameter {
                name: "lossless_level",
            })
        }
    }

    /// Lossless (`true`) or lossy (`false`, the default) encoding
    /// (C: `lossless`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().lossless(true).as_raw().lossless, 1);
    /// ```
    #[must_use]
    pub fn lossless(mut self, on: bool) -> Self {
        self.raw.lossless = i32::from(on);
        self
    }

    /// The quality, 0 to 100 (C: `quality`). Lossy: from small and poor to
    /// large and good. Lossless: from fast to slow and small.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().quality(30.0).as_raw().quality, 30.0);
    /// ```
    #[must_use]
    pub fn quality(mut self, quality: f32) -> Self {
        self.raw.quality = quality;
        self
    }

    /// The trade-off between speed and size, 0 (fast) to 6 (slow, smaller)
    /// (C: `method`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().method(6).as_raw().method, 6);
    /// ```
    #[must_use]
    pub fn method(mut self, method: u8) -> Self {
        self.raw.method = i32::from(method);
        self
    }

    /// A hint about the content for the lossless encoder (C: `image_hint`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, ImageHint};
    ///
    /// assert_eq!(Config::new().image_hint(ImageHint::Photo).as_raw().image_hint.0, 2);
    /// ```
    #[must_use]
    pub fn image_hint(mut self, hint: ImageHint) -> Self {
        self.raw.image_hint = hint.raw();
        self
    }

    /// A target size in bytes for a lossy encoding, 0 for none
    /// (C: `target_size`). The C field is an `int`, so the value saturates at
    /// `i32::MAX`, which no picture within the dimension limit reaches.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// let config = Config::new().target_size(20_000);
    /// assert_eq!(config.as_raw().target_size, 20_000);
    /// assert_eq!(Config::new().target_size(u32::MAX).as_raw().target_size, i32::MAX);
    /// ```
    #[must_use]
    pub fn target_size(mut self, bytes: u32) -> Self {
        self.raw.target_size = i32::try_from(bytes).unwrap_or(i32::MAX);
        self
    }

    /// A target PSNR in dB for a lossy encoding, 0 for none (C: `target_PSNR`);
    /// it takes precedence over [`Config::target_size`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().target_psnr(42.0).as_raw().target_psnr, 42.0);
    /// ```
    #[must_use]
    pub fn target_psnr(mut self, db: f32) -> Self {
        self.raw.target_psnr = db;
        self
    }

    /// Lossless compression of the alpha plane in a lossy encoding (`true`,
    /// the default) or no compression (C: `alpha_compression`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().alpha_compression(false).as_raw().alpha_compression, 0);
    /// ```
    #[must_use]
    pub fn alpha_compression(mut self, on: bool) -> Self {
        self.raw.alpha_compression = i32::from(on);
        self
    }

    /// The predictive filter of the alpha plane (C: `alpha_filtering`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{AlphaFilter, Config};
    ///
    /// assert_eq!(Config::new().alpha_filter(AlphaFilter::None).as_raw().alpha_filtering, 0);
    /// ```
    #[must_use]
    pub fn alpha_filter(mut self, filter: AlphaFilter) -> Self {
        self.raw.alpha_filtering = filter.raw();
        self
    }

    /// The quality of the alpha plane, 0 (smallest) to 100 (lossless, the
    /// default) (C: `alpha_quality`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().alpha_quality(50).as_raw().alpha_quality, 50);
    /// ```
    #[must_use]
    pub fn alpha_quality(mut self, quality: u8) -> Self {
        self.raw.alpha_quality = i32::from(quality);
        self
    }

    /// Keeps the samples under fully transparent areas (C: `exact`); see the
    /// module documentation for what the encoder writes otherwise.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().exact(true).as_raw().exact, 1);
    /// ```
    #[must_use]
    pub fn exact(mut self, on: bool) -> Self {
        self.raw.exact = i32::from(on);
        self
    }

    /// Near-lossless preprocessing of a lossless encoding, 0 (most loss) to 100
    /// (off, the default) (C: `near_lossless`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().lossless(true).near_lossless(60).as_raw().near_lossless, 60);
    /// ```
    #[must_use]
    pub fn near_lossless(mut self, level: u8) -> Self {
        self.raw.near_lossless = i32::from(level);
        self
    }

    /// The sharp (iterative, slower) RGB-to-YUV conversion of a lossy encoding
    /// of ARGB samples (C: `use_sharp_yuv`).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().sharp_yuv(true).as_raw().use_sharp_yuv, 1);
    /// ```
    #[must_use]
    pub fn sharp_yuv(mut self, on: bool) -> Self {
        self.raw.use_sharp_yuv = i32::from(on);
        self
    }

    /// Lets the encoder use one more thread inside the call (C:
    /// `thread_level`). The output depends on this setting, not on the
    /// machine; the writer and the progress hook stay on the calling thread.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().multithreaded(true).as_raw().thread_level, 1);
    /// ```
    #[must_use]
    pub fn multithreaded(mut self, on: bool) -> Self {
        self.raw.thread_level = i32::from(on);
        self
    }

    /// Checks the configuration as every encoding does before it starts:
    /// first the API's own rules (`quality` finite and in 0 to 100,
    /// `target_psnr` finite, which C does not check), then libwebp's
    /// validation (C: `WebPValidateConfig`).
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidParameter`] for the API's rules,
    /// [`EncodeError::InvalidConfiguration`] for libwebp's.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, EncodeError};
    ///
    /// assert_eq!(Config::new().validate(), Ok(()));
    /// assert_eq!(Config::new().method(7).validate(), Err(EncodeError::InvalidConfiguration));
    /// ```
    pub fn validate(&self) -> Result<(), EncodeError> {
        check_quality(self.raw.quality)?;
        if !self.raw.target_psnr.is_finite() {
            return Err(EncodeError::InvalidParameter {
                name: "target_psnr",
            });
        }
        if webp_validate_config(&self.raw) {
            Ok(())
        } else {
            Err(EncodeError::InvalidConfiguration)
        }
    }

    /// The configuration of the translation.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// assert_eq!(Config::new().as_raw().segments, 4);
    /// ```
    #[must_use]
    pub const fn as_raw(&self) -> &WebPConfig {
        &self.raw
    }

    /// The configuration of the translation, mutably, for the fields without a
    /// setter; [`Config::validate`] checks what is written.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Config;
    ///
    /// let mut config = Config::new();
    /// config.raw_mut().segments = 1;
    /// config.raw_mut().pass = 6;
    /// config.validate()?;
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    pub const fn raw_mut(&mut self) -> &mut WebPConfig {
        &mut self.raw
    }
}

//------------------------------------------------------------------------------
// Input validation

/// Checks a width and height against 1 to `WEBP_MAX_DIMENSION`.
fn check_dimensions(width: u32, height: u32) -> Result<(i32, i32), EncodeError> {
    let limit = WEBP_MAX_DIMENSION.unsigned_abs();
    if width == 0 || height == 0 || width > limit || height > limit {
        return Err(EncodeError::InvalidDimensions { width, height });
    }
    let invalid = EncodeError::InvalidDimensions { width, height };
    Ok((
        i32::try_from(width).map_err(|_| invalid)?,
        i32::try_from(height).map_err(|_| invalid)?,
    ))
}

/// Checks the stride and length of a plane of `rows` rows of `columns`
/// elements, and returns the stride as C's `int`.
fn check_plane(
    plane: Plane,
    len: usize,
    columns: usize,
    rows: usize,
    stride: usize,
) -> Result<i32, EncodeError> {
    let invalid = EncodeError::InvalidStride { plane, stride };
    if stride < columns {
        return Err(invalid);
    }
    let c_stride = i32::try_from(stride).map_err(|_| invalid)?;
    let needed = rows
        .checked_sub(1)
        .and_then(|rows| rows.checked_mul(stride))
        .and_then(|start| start.checked_add(columns))
        .ok_or(invalid)?;
    if len < needed {
        return Err(EncodeError::BufferTooShort { plane, len, needed });
    }
    Ok(c_stride)
}

/// A dimension that `check_dimensions` accepted, as `usize`.
fn count(value: i32) -> usize {
    usize::try_from(value).unwrap_or(0)
}

//------------------------------------------------------------------------------
// Pictures

/// Strided rows of packed pixels, the input of [`Picture::import`] and of the
/// simple functions.
///
/// `stride` counts elements of the slice's type, which for `data: &[u8]` are
/// bytes: at least `width * layout.bytes_per_pixel()`, at most `i32::MAX`.
/// `data` holds at least `(height - 1) * stride + width * bytes_per_pixel`
/// bytes; the bytes between rows are not read.
///
/// # Examples
///
/// ```
/// use webp::api::encode::{PixelLayout, PixelRows};
///
/// // 3 by 2 BGR pixels with rows padded to 12 bytes.
/// let data = [0_u8; 12 + 9];
/// let rows = PixelRows { layout: PixelLayout::Bgr, data: &data, width: 3, height: 2, stride: 12 };
/// assert_eq!(rows.stride, 12);
/// ```
#[derive(Clone, Copy)]
pub struct PixelRows<'p> {
    /// The order of the channels.
    pub layout: PixelLayout,
    /// The rows, top to bottom.
    pub data: &'p [u8],
    /// The pixels of a row, 1 to 16383.
    pub width: u32,
    /// The rows, 1 to 16383.
    pub height: u32,
    /// The distance from the start of a row to the next, in bytes.
    pub stride: usize,
}

impl fmt::Debug for PixelRows<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PixelRows")
            .field("layout", &self.layout)
            .field("data_len", &self.data.len())
            .field("width", &self.width)
            .field("height", &self.height)
            .field("stride", &self.stride)
            .finish()
    }
}

/// The caller's Y, U, V and optional A planes, the input of
/// [`Picture::from_yuva`].
///
/// Strides count elements of the slices' type, which are bytes. The Y and A
/// planes have `height` rows of `width` samples, the U and V planes
/// `(height + 1) / 2` rows of `(width + 1) / 2` samples; each stride is at
/// least its row length and at most `i32::MAX`, and each plane holds at least
/// `(rows - 1) * stride + row length` samples. `a_stride` is ignored when `a`
/// is `None`.
///
/// # Examples
///
/// ```
/// use webp::api::encode::{Picture, YuvaPlanes};
///
/// // 3 by 3: chroma planes of 2 by 2, alpha omitted.
/// let (mut y, mut u, mut v) = (vec![128_u8; 9], vec![128_u8; 4], vec![128_u8; 4]);
/// let planes = YuvaPlanes {
///     y: &mut y, y_stride: 3,
///     u: &mut u, v: &mut v, uv_stride: 2,
///     a: None, a_stride: 0,
/// };
/// let picture = Picture::from_yuva(3, 3, planes)?;
/// assert_eq!(picture.owned_sample_bytes(), 0);
/// # Ok::<(), webp::api::encode::EncodeError>(())
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
    /// The alpha plane, straight alpha, or `None` for an opaque picture.
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

/// The fields of a `WebPPicture` that describe its samples, without the
/// storages and the per-call callbacks.
#[derive(Clone, Copy, Default)]
struct Specs {
    use_argb: i32,
    colorspace: WebPEncCSP,
    width: i32,
    height: i32,
    y: Option<PlaneOffset>,
    u: Option<PlaneOffset>,
    v: Option<PlaneOffset>,
    a: Option<PlaneOffset>,
    y_stride: i32,
    uv_stride: i32,
    a_stride: i32,
    argb: Option<PlaneOffset>,
    argb_stride: i32,
}

impl Specs {
    fn of(pic: &WebPPicture<'_>) -> Self {
        Self {
            use_argb: pic.use_argb,
            colorspace: pic.colorspace,
            width: pic.width,
            height: pic.height,
            y: pic.y,
            u: pic.u,
            v: pic.v,
            a: pic.a,
            y_stride: pic.y_stride,
            uv_stride: pic.uv_stride,
            a_stride: pic.a_stride,
            argb: pic.argb,
            argb_stride: pic.argb_stride,
        }
    }

    fn apply(self, pic: &mut WebPPicture<'_>) {
        pic.use_argb = self.use_argb;
        pic.colorspace = self.colorspace;
        pic.width = self.width;
        pic.height = self.height;
        pic.y = self.y;
        pic.u = self.u;
        pic.v = self.v;
        pic.a = self.a;
        pic.y_stride = self.y_stride;
        pic.uv_stride = self.uv_stride;
        pic.a_stride = self.a_stride;
        pic.argb = self.argb;
        pic.argb_stride = self.argb_stride;
    }
}

/// The plane at the start of storage region `region`.
const fn region_start(region: usize) -> PlaneOffset {
    PlaneOffset { region, offset: 0 }
}

/// A storage for one call: owned samples move out (and come back through
/// [`Returned`]), the caller's slices are reborrowed.
fn lend<'s, T>(storage: &'s mut PlaneStorage<'_, T>) -> PlaneStorage<'s, T> {
    match storage {
        PlaneStorage::Internal(samples) => PlaneStorage::Internal(std::mem::take(samples)),
        PlaneStorage::External(slices) => {
            PlaneStorage::External(slices.iter_mut().map(|slice| &mut **slice).collect())
        }
        // Not produced by this API; a view's regions are shared references.
        PlaneStorage::Shared(slices) => PlaneStorage::Shared(slices.clone()),
    }
}

/// The owned samples of a storage after a call; the caller's slices of an
/// `External` storage are still held by the picture.
fn owned<T>(storage: PlaneStorage<'_, T>) -> Option<Vec<T>> {
    match storage {
        PlaneStorage::Internal(samples) => Some(samples),
        PlaneStorage::External(_) | PlaneStorage::Shared(_) => None,
    }
}

/// What a call leaves in its `WebPPicture`, without the call's lifetime.
struct Returned {
    specs: Specs,
    yuva: Option<Vec<u8>>,
    argb: Option<Vec<u32>>,
    error: WebPEncodingError,
}

impl Returned {
    fn take(pic: WebPPicture<'_>) -> Self {
        Self {
            specs: Specs::of(&pic),
            error: pic.error_code,
            yuva: owned(pic.yuva),
            argb: owned(pic.argb_storage),
        }
    }
}

/// A picture to encode: its dimensions and its samples, either planes it
/// allocated ([`Picture::import`]) or the caller's planes, borrowed
/// exclusively for `'a` ([`Picture::from_argb`], [`Picture::from_yuva`]).
///
/// A picture holds no callbacks, so it is `Send` and `Sync`; the writer, the
/// progress hook and the statistics belong to one [`Encoder`] call.
///
/// # Examples
///
/// ```
/// use webp::api::encode::{Config, Encoder, Picture};
///
/// // Caller-owned ARGB pixels (0xAARRGGBB), 2 by 2 with one pixel of padding.
/// let mut argb = vec![0xff20_4060_u32; 2 * 3];
/// let mut picture = Picture::from_argb(&mut argb, 2, 2, 3)?;
/// assert_eq!((picture.width(), picture.height()), (2, 2));
/// let config = Config::new().lossless(true);
/// let webp = Encoder::new(&config).encode(&mut picture)?;
/// assert_eq!(&webp[12..16], b"VP8L");
/// # Ok::<(), webp::api::encode::EncodeError>(())
/// ```
pub struct Picture<'a> {
    specs: Specs,
    yuva: PlaneStorage<'a, u8>,
    argb_storage: PlaneStorage<'a, u32>,
}

impl fmt::Debug for Picture<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Picture")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("use_argb", &self.specs.use_argb)
            .field("owned_sample_bytes", &self.owned_sample_bytes())
            .finish_non_exhaustive()
    }
}

impl Picture<'static> {
    /// Copies `rows` into planes that the picture allocates, in the form that
    /// `form` selects (C: `WebPPictureImportRGB` and its siblings, with
    /// `use_argb`). [`SampleForm::Yuva`] converts to YUV at once, and adds an
    /// alpha plane only when some pixel is not opaque.
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidDimensions`], [`EncodeError::InvalidStride`] or
    /// [`EncodeError::BufferTooShort`] for rows outside the rules of
    /// [`PixelRows`]; [`EncodeError::OutOfMemory`] when the planes cannot be
    /// allocated.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Picture, PixelLayout, PixelRows, SampleForm};
    ///
    /// let rgba = [1_u8, 2, 3, 128].repeat(6);
    /// let rows = PixelRows { layout: PixelLayout::Rgba, data: &rgba, width: 3, height: 2, stride: 12 };
    /// let picture = Picture::import(rows, SampleForm::Yuva)?;
    /// // Y 3 by 2, U and V 2 by 1, and alpha, since the pixels are translucent.
    /// assert_eq!(picture.owned_sample_bytes(), 6 + 2 + 2 + 6);
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    pub fn import(rows: PixelRows<'_>, form: SampleForm) -> Result<Self, EncodeError> {
        let (width, height) = check_dimensions(rows.width, rows.height)?;
        let columns = count(width)
            .checked_mul(rows.layout.bytes_per_pixel())
            .ok_or(EncodeError::InvalidDimensions {
                width: rows.width,
                height: rows.height,
            })?;
        let stride = check_plane(
            Plane::Pixels,
            rows.data.len(),
            columns,
            count(height),
            rows.stride,
        )?;
        let mut pic = WebPPicture::default();
        let initialized = webp_picture_init(&mut pic);
        debug_assert!(initialized, "the picture's version is the crate's");
        pic.width = width;
        pic.height = height;
        pic.use_argb = i32::from(form == SampleForm::Argb);
        if !(rows.layout.importer())(&mut pic, rows.data, stride) {
            // A refusal without a code is the stride test, which the checks
            // above already passed.
            return Err(if pic.error_code == WebPEncodingError::VP8_ENC_OK {
                EncodeError::InvalidStride {
                    plane: Plane::Pixels,
                    stride: rows.stride,
                }
            } else {
                EncodeError::from_code(pic.error_code)
            });
        }
        Ok(Self {
            specs: Specs::of(&pic),
            yuva: pic.yuva,
            argb_storage: pic.argb_storage,
        })
    }
}

impl<'a> Picture<'a> {
    /// Borrows the caller's ARGB pixels, `0xAARRGGBB` in each `u32` with
    /// straight alpha, as the picture's samples (C: `use_argb` with the
    /// caller's `argb` plane).
    ///
    /// `stride` counts elements of the slice's type, which for `&mut [u32]`
    /// are pixels, not bytes: at least `width`, at most `i32::MAX`. `argb`
    /// holds at least `(height - 1) * stride + width` pixels.
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidDimensions`], [`EncodeError::InvalidStride`] or
    /// [`EncodeError::BufferTooShort`] (with [`Plane::Argb`]).
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Picture;
    ///
    /// let mut argb = vec![0x80ff_0000_u32; 4 * 4];
    /// let picture = Picture::from_argb(&mut argb, 4, 4, 4)?;
    /// assert_eq!(picture.owned_sample_bytes(), 0);
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    pub fn from_argb(
        argb: &'a mut [u32],
        width: u32,
        height: u32,
        stride: usize,
    ) -> Result<Self, EncodeError> {
        let (c_width, c_height) = check_dimensions(width, height)?;
        let argb_stride = check_plane(
            Plane::Argb,
            argb.len(),
            count(c_width),
            count(c_height),
            stride,
        )?;
        Ok(Self {
            specs: Specs {
                use_argb: 1,
                width: c_width,
                height: c_height,
                argb: Some(region_start(0)),
                argb_stride,
                ..Specs::default()
            },
            yuva: PlaneStorage::default(),
            argb_storage: PlaneStorage::External(vec![argb]),
        })
    }

    /// Borrows the caller's Y, U, V and optional alpha planes as the picture's
    /// samples, taken as they are (C: `use_argb` 0 with the caller's planes,
    /// colorspace `WEBP_YUV420` or, with alpha, `WEBP_YUV420A`). See
    /// [`YuvaPlanes`] for the stride and length rules.
    ///
    /// # Errors
    ///
    /// [`EncodeError::InvalidDimensions`], [`EncodeError::InvalidStride`] or
    /// [`EncodeError::BufferTooShort`], naming the plane.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, Encoder, Picture, YuvaPlanes};
    ///
    /// let (mut y, mut u, mut v, mut a) = (vec![100_u8; 16], vec![90_u8; 4], vec![160_u8; 4], vec![255_u8; 16]);
    /// let planes = YuvaPlanes {
    ///     y: &mut y, y_stride: 4,
    ///     u: &mut u, v: &mut v, uv_stride: 2,
    ///     a: Some(&mut a), a_stride: 4,
    /// };
    /// let mut picture = Picture::from_yuva(4, 4, planes)?;
    /// let webp = Encoder::new(&Config::new()).encode(&mut picture)?;
    /// assert_eq!(&webp[12..16], b"VP8 ");
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    pub fn from_yuva(width: u32, height: u32, planes: YuvaPlanes<'a>) -> Result<Self, EncodeError> {
        let (c_width, c_height) = check_dimensions(width, height)?;
        let (columns, rows) = (count(c_width), count(c_height));
        let (uv_columns, uv_rows) = (count((c_width + 1) >> 1), count((c_height + 1) >> 1));
        let y_stride = check_plane(Plane::Y, planes.y.len(), columns, rows, planes.y_stride)?;
        let uv_stride = check_plane(
            Plane::U,
            planes.u.len(),
            uv_columns,
            uv_rows,
            planes.uv_stride,
        )?;
        check_plane(
            Plane::V,
            planes.v.len(),
            uv_columns,
            uv_rows,
            planes.uv_stride,
        )?;
        let a_stride = match &planes.a {
            Some(a) => check_plane(Plane::A, a.len(), columns, rows, planes.a_stride)?,
            None => 0,
        };
        let has_alpha = planes.a.is_some();
        let mut regions = vec![planes.y, planes.u, planes.v];
        regions.extend(planes.a);
        Ok(Self {
            specs: Specs {
                use_argb: 0,
                colorspace: if has_alpha {
                    WebPEncCSP::WEBP_YUV420A
                } else {
                    WebPEncCSP::WEBP_YUV420
                },
                width: c_width,
                height: c_height,
                y: Some(region_start(0)),
                u: Some(region_start(1)),
                v: Some(region_start(2)),
                a: has_alpha.then_some(region_start(3)),
                y_stride,
                uv_stride,
                a_stride,
                ..Specs::default()
            },
            yuva: PlaneStorage::External(regions),
            argb_storage: PlaneStorage::default(),
        })
    }

    /// The width in pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Picture;
    ///
    /// let mut argb = vec![0_u32; 6];
    /// assert_eq!(Picture::from_argb(&mut argb, 3, 2, 3)?.width(), 3);
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    #[must_use]
    pub const fn width(&self) -> u32 {
        self.specs.width.unsigned_abs()
    }

    /// The height in pixels.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::Picture;
    ///
    /// let mut argb = vec![0_u32; 6];
    /// assert_eq!(Picture::from_argb(&mut argb, 3, 2, 3)?.height(), 2);
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    #[must_use]
    pub const fn height(&self) -> u32 {
        self.specs.height.unsigned_abs()
    }

    /// The bytes of the samples that the picture allocated itself: the
    /// imported planes, and the planes that an encoding's conversion added.
    /// The caller's planes are not counted, nor is the encoder's internal
    /// state.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, Encoder, Picture};
    ///
    /// let mut argb = vec![0xff00_ff00_u32; 4 * 4];
    /// let mut picture = Picture::from_argb(&mut argb, 4, 4, 4)?;
    /// assert_eq!(picture.owned_sample_bytes(), 0);
    /// // A lossy encoding converts the ARGB pixels into YUV planes it keeps.
    /// Encoder::new(&Config::new()).encode(&mut picture)?;
    /// assert_eq!(picture.owned_sample_bytes(), 16 + 4 + 4);
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    #[must_use]
    pub fn owned_sample_bytes(&self) -> usize {
        let yuva = match &self.yuva {
            PlaneStorage::Internal(samples) => samples.len(),
            PlaneStorage::External(_) | PlaneStorage::Shared(_) => 0,
        };
        let argb = match &self.argb_storage {
            PlaneStorage::Internal(samples) => samples.len() * std::mem::size_of::<u32>(),
            PlaneStorage::External(_) | PlaneStorage::Shared(_) => 0,
        };
        yuva + argb
    }

    /// Encodes the picture with the translated `WebPEncode` in a `WebPPicture`
    /// that lives for this call. The planes leave the picture for the call and
    /// come back afterwards, so a panicking callback leaves it without planes.
    fn encode_with<'c>(
        &'c mut self,
        config: &WebPConfig,
        writer: &'c mut dyn FnMut(&[u8]) -> bool,
        progress: Option<Box<dyn FnMut(i32) -> bool + 'c>>,
        stats: Option<&'c mut WebPAuxStats>,
    ) -> Result<(), EncodeError> {
        let specs = self.specs;
        self.specs = Specs {
            y: None,
            u: None,
            v: None,
            a: None,
            argb: None,
            ..specs
        };
        let mut call = WebPPicture::default();
        let initialized = webp_picture_init(&mut call);
        debug_assert!(initialized, "the picture's version is the crate's");
        specs.apply(&mut call);
        call.yuva = lend(&mut self.yuva);
        call.argb_storage = lend(&mut self.argb_storage);
        call.writer = Some(Box::new(move |data: &[u8]| writer(data)));
        call.progress_hook = progress;
        call.stats = stats;

        let ok = webp_encode(config, &mut call);

        let returned = Returned::take(call);
        self.specs = returned.specs;
        if let Some(samples) = returned.yuva {
            self.yuva = PlaneStorage::Internal(samples);
        }
        if let Some(samples) = returned.argb {
            self.argb_storage = PlaneStorage::Internal(samples);
        }
        if ok {
            Ok(())
        } else {
            Err(EncodeError::from_code(returned.error))
        }
    }
}

//------------------------------------------------------------------------------
// Encoding

/// One encoding: a configuration, and optionally a progress hook and
/// statistics, applied to a [`Picture`] with [`Encoder::encode`] or
/// [`Encoder::encode_to`].
///
/// # Examples
///
/// ```
/// use webp::api::encode::{Config, Encoder, Picture, WebPAuxStats};
///
/// let mut argb = vec![0xff40_8020_u32; 32 * 32];
/// let mut picture = Picture::from_argb(&mut argb, 32, 32, 32)?;
/// let config = Config::new().quality(60.0);
/// let mut stats = WebPAuxStats::default();
/// let mut reports = Vec::new();
/// let webp = Encoder::new(&config)
///     .progress(|percent| {
///         reports.push(percent);
///         true
///     })
///     .stats(&mut stats)
///     .encode(&mut picture)?;
/// assert_eq!(stats.coded_size as usize, webp.len());
/// assert!(!reports.is_empty());
/// # Ok::<(), webp::api::encode::EncodeError>(())
/// ```
pub struct Encoder<'e> {
    config: &'e Config,
    progress: Option<Box<dyn FnMut(i32) -> bool + 'e>>,
    stats: Option<&'e mut WebPAuxStats>,
}

impl fmt::Debug for Encoder<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Encoder")
            .field("config", self.config)
            .field("progress", &self.progress.is_some())
            .field("stats", &self.stats.is_some())
            .finish()
    }
}

impl<'e> Encoder<'e> {
    /// An encoding with `config`, no progress hook and no statistics.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, Encoder};
    ///
    /// let config = Config::new();
    /// let encoder = Encoder::new(&config);
    /// # let _ = encoder;
    /// ```
    #[must_use]
    pub const fn new(config: &'e Config) -> Self {
        Self {
            config,
            progress: None,
            stats: None,
        }
    }

    /// Reports progress to `hook` (C: `WebPPicture::progress_hook`). It
    /// receives libwebp's percentages, 0 to 100, each time the value changes,
    /// on the calling thread. Returning `false` stops the encoding with
    /// [`EncodeError::UserAbort`]; the writer keeps what it received.
    ///
    /// libwebp ignores the answer at two places, the end of a lossy analysis
    /// without segments (C: `ResetAllMBInfo`) and the statistics pass of lossy
    /// methods 0 to 2 (C: `VP8EncLoop` drops the result of `StatLoop`): the
    /// encoding then goes on and can succeed. A hook that cancels should
    /// therefore keep returning `false`.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, EncodeError, Encoder, Picture};
    ///
    /// let mut argb = vec![0xff00_0000_u32; 64 * 64];
    /// let mut picture = Picture::from_argb(&mut argb, 64, 64, 64)?;
    /// let config = Config::new();
    /// let result = Encoder::new(&config).progress(|percent| percent < 50).encode(&mut picture);
    /// assert_eq!(result, Err(EncodeError::UserAbort));
    /// # Ok::<(), EncodeError>(())
    /// ```
    #[must_use]
    pub fn progress(mut self, hook: impl FnMut(i32) -> bool + 'e) -> Self {
        self.progress = Some(Box::new(hook));
        self
    }

    /// Fills `stats` with the encoding's statistics (C:
    /// `WebPPicture::stats`). They are reset once the configuration and the
    /// picture pass libwebp's validation and filled as libwebp fills them, on
    /// failure too.
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, Encoder, Picture, WebPAuxStats};
    ///
    /// let mut argb = vec![0xff80_8080_u32; 16 * 16];
    /// let mut picture = Picture::from_argb(&mut argb, 16, 16, 16)?;
    /// let config = Config::new().quality(90.0);
    /// let mut stats = WebPAuxStats::default();
    /// let webp = Encoder::new(&config).stats(&mut stats).encode(&mut picture)?;
    /// assert_eq!(stats.coded_size as usize, webp.len());
    /// assert!(stats.psnr[3] > 40.0); // Y, U and V together, in dB
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    #[must_use]
    pub fn stats(mut self, stats: &'e mut WebPAuxStats) -> Self {
        self.stats = Some(stats);
        self
    }

    /// Encodes `picture` into memory, through libwebp's memory writer
    /// (C: `WebPEncode` with `WebPMemoryWrite`), and returns the WebP file.
    ///
    /// The encoder may write into the picture's planes, as the module
    /// documentation describes, and leaves the planes of its conversions with
    /// the picture. If the progress hook panics, the panic propagates and the
    /// picture is left without planes: a later encoding of it returns
    /// [`EncodeError::NullParameter`].
    ///
    /// # Errors
    ///
    /// The errors of [`Config::validate`], and the code libwebp records:
    /// [`EncodeError::BadWrite`] when the output cannot grow,
    /// [`EncodeError::UserAbort`] from the progress hook, and the others of
    /// [`EncodeError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, Encoder, Picture, PixelLayout, PixelRows, SampleForm};
    ///
    /// let rgb = [200_u8, 100, 50].repeat(16 * 16);
    /// let rows = PixelRows { layout: PixelLayout::Rgb, data: &rgb, width: 16, height: 16, stride: 48 };
    /// let mut picture = Picture::import(rows, SampleForm::Yuva)?;
    /// let webp = Encoder::new(&Config::new()).encode(&mut picture)?;
    /// assert_eq!(&webp[..4], b"RIFF");
    /// # Ok::<(), webp::api::encode::EncodeError>(())
    /// ```
    pub fn encode(self, picture: &mut Picture<'_>) -> Result<Vec<u8>, EncodeError> {
        let mut memory = WebPMemoryWriter::default();
        webp_memory_writer_init(&mut memory);
        self.encode_to(picture, |data| webp_memory_write(&mut memory, data))?;
        let mut bytes = memory.mem;
        bytes.truncate(memory.size);
        Ok(bytes)
    }

    /// Encodes `picture` through `writer` (C: `WebPEncode` with
    /// `WebPPicture::writer`). The writer receives the WebP file in order, in
    /// chunks that may be empty, on the calling thread; returning `false`
    /// stops the encoding with [`EncodeError::BadWrite`], and the bytes it
    /// accepted stay with it.
    ///
    /// The encoder may write into the picture's planes, as the module
    /// documentation describes, and leaves the planes of its conversions with
    /// the picture. If the writer or the progress hook panics, the panic
    /// propagates and the picture is left without planes: a later encoding of
    /// it returns [`EncodeError::NullParameter`].
    ///
    /// # Errors
    ///
    /// The errors of [`Config::validate`], and the code libwebp records:
    /// [`EncodeError::BadWrite`] from the writer, [`EncodeError::UserAbort`]
    /// from the progress hook, and the others of [`EncodeError`].
    ///
    /// # Examples
    ///
    /// ```
    /// use webp::api::encode::{Config, EncodeError, Encoder, Picture};
    ///
    /// let mut argb = vec![0xff12_3456_u32; 10 * 10];
    /// let mut picture = Picture::from_argb(&mut argb, 10, 10, 10)?;
    /// let config = Config::new();
    /// let mut file = Vec::new();
    /// Encoder::new(&config).encode_to(&mut picture, |chunk| {
    ///     file.extend_from_slice(chunk);
    ///     true
    /// })?;
    /// assert_eq!(&file[..4], b"RIFF");
    ///
    /// let refused = Encoder::new(&config).encode_to(&mut picture, |_| false);
    /// assert_eq!(refused, Err(EncodeError::BadWrite));
    /// # Ok::<(), EncodeError>(())
    /// ```
    pub fn encode_to(
        self,
        picture: &mut Picture<'_>,
        mut writer: impl FnMut(&[u8]) -> bool,
    ) -> Result<(), EncodeError> {
        self.config.validate()?;
        picture.encode_with(&self.config.raw, &mut writer, self.progress, self.stats)
    }
}

//------------------------------------------------------------------------------
// Simple functions

/// The steps of C's `Encode` (`src/enc/picture_enc.c`): the default preset at
/// `quality`, `lossless`, ARGB samples for a lossless encoding, the importer,
/// and `WebPEncode` into the memory writer.
fn encode_simple(
    rows: PixelRows<'_>,
    quality: f32,
    lossless: bool,
) -> Result<Vec<u8>, EncodeError> {
    let config = Config::preset(Preset::Default, quality)?.lossless(lossless);
    let form = if lossless {
        SampleForm::Argb
    } else {
        SampleForm::Yuva
    };
    let mut picture = Picture::import(rows, form)?;
    Encoder::new(&config).encode(&mut picture)
}

/// Encodes `rows` lossily at `quality` with the default preset, as libwebp's
/// simple API does (C: `WebPEncodeRGB`, `WebPEncodeBGR`, `WebPEncodeRGBA` and
/// `WebPEncodeBGRA`; the X layouts take the same steps).
///
/// # Errors
///
/// [`EncodeError::InvalidParameter`] for a quality that is not finite or not
/// in 0 to 100, the errors of [`Picture::import`], and the code libwebp
/// records.
///
/// # Examples
///
/// ```
/// use webp::api::encode::{PixelLayout, PixelRows, encode_lossy};
///
/// let bgra = [30_u8, 60, 90, 255].repeat(8 * 8);
/// let rows = PixelRows { layout: PixelLayout::Bgra, data: &bgra, width: 8, height: 8, stride: 32 };
/// let webp = encode_lossy(rows, 75.0)?;
/// assert_eq!(&webp[12..16], b"VP8 ");
/// # Ok::<(), webp::api::encode::EncodeError>(())
/// ```
pub fn encode_lossy(rows: PixelRows<'_>, quality: f32) -> Result<Vec<u8>, EncodeError> {
    encode_simple(rows, quality, false)
}

/// Encodes `rows` losslessly with the default preset at quality 70, as
/// libwebp's simple API does (C: `WebPEncodeLosslessRGB`,
/// `WebPEncodeLosslessBGR`, `WebPEncodeLosslessRGBA` and
/// `WebPEncodeLosslessBGRA`; the X layouts take the same steps).
///
/// # Errors
///
/// The errors of [`Picture::import`], and the code libwebp records.
///
/// # Examples
///
/// ```
/// use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};
///
/// let rgbx = [1_u8, 2, 3, 0].repeat(5 * 4);
/// let rows = PixelRows { layout: PixelLayout::Rgbx, data: &rgbx, width: 5, height: 4, stride: 20 };
/// let webp = encode_lossless(rows)?;
/// assert_eq!(&webp[12..16], b"VP8L");
/// # Ok::<(), webp::api::encode::EncodeError>(())
/// ```
pub fn encode_lossless(rows: PixelRows<'_>) -> Result<Vec<u8>, EncodeError> {
    encode_simple(rows, LOSSLESS_DEFAULT_QUALITY, true)
}
