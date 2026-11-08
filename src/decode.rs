//! The safe decoding API (ADR 0006): a row-streaming decoder over the
//! translated libjpeg API, for Rust programs. Not an upstream file.
//!
//! [`Decoder::new`] reads the header of a JPEG image held in memory and keeps
//! its EXIF data and ICC profile. [`Decoder::start`] selects the output
//! format, scale and DCT method and starts decompression;
//! [`Scanlines`] then crops columns, skips rows and reads rows in order.
//! Every libjpeg error and warning comes back as a [`Message`] with
//! upstream's code and text; nothing is printed.
//!
//! ```
//! use jpeg::{Decoder, OutputOptions, PixelFormat, Scale};
//!
//! /// Decodes the rows `top..bottom` of `data` at half size as RGB.
//! fn rows(data: &[u8], top: u32, bottom: u32) -> Result<Vec<u8>, jpeg::Error> {
//!     let mut decoder = Decoder::new(data)?;
//!     let _icc = decoder.icc_profile()?;
//!     let _exif = decoder.exif();
//!     let mut scanlines = decoder.start(&OutputOptions {
//!         format: Some(PixelFormat::Rgb),
//!         scale: Scale { num: 1, denom: 2 },
//!         ..OutputOptions::default()
//!     })?;
//!     let bottom = bottom.min(scanlines.info().height);
//!     scanlines.skip_rows(top)?;
//!     let mut buffer = vec![0; scanlines.info().row_bytes() * (bottom - top) as usize];
//!     scanlines.read_rows(&mut buffer)?;
//!     Ok(buffer)
//! }
//! ```
//!
//! The calls are those of an application of the libjpeg API, in upstream's
//! order: `jpeg_create_decompress`, `jpeg_mem_src`, `jpeg_save_markers` for
//! APP1 and APP2, `jpeg_read_header`, `jpeg_read_icc_profile`, the output
//! parameters, `jpeg_calc_output_dimensions`, `jpeg_start_decompress`,
//! `jpeg_crop_scanline`, `jpeg_skip_scanlines`, `jpeg_read_scanlines`,
//! `jpeg_finish_decompress` and `jpeg_destroy_decompress`.

use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

use crate::jdapimin::{
    jpeg_create_decompress, jpeg_destroy_decompress, jpeg_finish_decompress, jpeg_read_header,
};
use crate::jdapistd::{
    jpeg_crop_scanline, jpeg_read_scanlines, jpeg_skip_scanlines, jpeg_start_decompress,
};
use crate::jdatasrc::jpeg_mem_src;
use crate::jdicc::jpeg_read_icc_profile;
use crate::jdmarker::jpeg_save_markers;
use crate::jdmaster::jpeg_calc_output_dimensions;
use crate::jerror::{
    ErrorExit, J_MESSAGE_CODE, format_message, jpeg_error_methods, jpeg_std_error,
};
use crate::jpeglib::{
    J_COLOR_SPACE, J_DCT_METHOD, JMSG_LENGTH_MAX, JPEG_APP0, RowListMut, jpeg_decompress_struct,
    jpeg_error_mgr,
};

/// The most warnings a decoder keeps; [`Report::warning_count`] counts all.
pub const MAX_KEPT_WARNINGS: usize = 64;

/// The length limit of the saved APP1 and APP2 markers: whole segments, as
/// `djpeg` saves APP2 for ICC profiles.
const MARKER_LENGTH_LIMIT: u32 = 0xFFFF;

/// The identifier that starts the APP1 segment holding EXIF data.
const EXIF_IDENTIFIER: &[u8] = b"Exif\0\0";

/// A libjpeg error or warning: upstream's message code and formatted text.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    /// The message code, for example `J_MESSAGE_CODE::JWRN_JPEG_EOF`.
    pub code: J_MESSAGE_CODE,
    /// The text `format_message` produces, without a newline.
    pub text: String,
}

impl fmt::Display for Message {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.text)
    }
}

/// Why decoding stopped.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// libjpeg raised an error. The decoder cannot continue.
    Library(Message),
    /// libjpeg emitted a corrupt-data warning while
    /// [`DecoderOptions::fail_on_warning`] was set. The decoder cannot
    /// continue.
    Warning(Message),
    /// An argument of this API was not acceptable; libjpeg was not called.
    InvalidArgument(&'static str),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Library(message) => write!(f, "JPEG error: {message}"),
            Error::Warning(message) => write!(f, "JPEG warning treated as error: {message}"),
            Error::InvalidArgument(reason) => write!(f, "invalid argument: {reason}"),
        }
    }
}

impl std::error::Error for Error {}

/// Options that apply from the start of decoding.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub struct DecoderOptions {
    /// `max_memory_to_use` in bytes, rounded down to libjpeg's `long`. It
    /// limits the whole-image buffers of progressive and buffered decoding;
    /// a file that needs more fails with `JERR_NO_BACKING_STORE`. `None`
    /// keeps libjpeg's default: no limit, or the `JPEGMEM` environment
    /// variable.
    pub max_memory: Option<usize>,
    /// Stop at the first corrupt-data warning with [`Error::Warning`]
    /// instead of continuing as libjpeg does.
    pub fail_on_warning: bool,
}

/// The color space of the JPEG data, `jpeg_color_space`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum ColorSpace {
    /// `JCS_UNKNOWN`.
    Unknown,
    /// `JCS_GRAYSCALE`.
    Grayscale,
    /// `JCS_RGB`.
    Rgb,
    /// `JCS_YCbCr`.
    YCbCr,
    /// `JCS_CMYK`.
    Cmyk,
    /// `JCS_YCCK`.
    Ycck,
}

impl ColorSpace {
    fn from_jcs(space: J_COLOR_SPACE) -> Self {
        match space {
            J_COLOR_SPACE::JCS_GRAYSCALE => ColorSpace::Grayscale,
            J_COLOR_SPACE::JCS_RGB => ColorSpace::Rgb,
            J_COLOR_SPACE::JCS_YCbCr => ColorSpace::YCbCr,
            J_COLOR_SPACE::JCS_CMYK => ColorSpace::Cmyk,
            J_COLOR_SPACE::JCS_YCCK => ColorSpace::Ycck,
            _ => ColorSpace::Unknown,
        }
    }
}

/// The layout of output samples, `out_color_space`. The `x` channels of the
/// padded formats and the alpha channels are filled with 255.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PixelFormat {
    /// One gray sample, `JCS_GRAYSCALE`.
    Grayscale,
    /// Red, green, blue, `JCS_RGB`.
    Rgb,
    /// `JCS_EXT_RGBX`.
    Rgbx,
    /// `JCS_EXT_BGR`.
    Bgr,
    /// `JCS_EXT_BGRX`.
    Bgrx,
    /// `JCS_EXT_XBGR`.
    Xbgr,
    /// `JCS_EXT_XRGB`.
    Xrgb,
    /// `JCS_EXT_RGBA`.
    Rgba,
    /// `JCS_EXT_BGRA`.
    Bgra,
    /// `JCS_EXT_ABGR`.
    Abgr,
    /// `JCS_EXT_ARGB`.
    Argb,
    /// Luma and chroma as stored, `JCS_YCbCr`.
    YCbCr,
    /// Cyan, magenta, yellow, black, `JCS_CMYK`; for CMYK and YCCK files.
    Cmyk,
    /// `JCS_YCCK`, for YCCK files.
    Ycck,
}

impl PixelFormat {
    fn to_jcs(self) -> J_COLOR_SPACE {
        match self {
            PixelFormat::Grayscale => J_COLOR_SPACE::JCS_GRAYSCALE,
            PixelFormat::Rgb => J_COLOR_SPACE::JCS_RGB,
            PixelFormat::Rgbx => J_COLOR_SPACE::JCS_EXT_RGBX,
            PixelFormat::Bgr => J_COLOR_SPACE::JCS_EXT_BGR,
            PixelFormat::Bgrx => J_COLOR_SPACE::JCS_EXT_BGRX,
            PixelFormat::Xbgr => J_COLOR_SPACE::JCS_EXT_XBGR,
            PixelFormat::Xrgb => J_COLOR_SPACE::JCS_EXT_XRGB,
            PixelFormat::Rgba => J_COLOR_SPACE::JCS_EXT_RGBA,
            PixelFormat::Bgra => J_COLOR_SPACE::JCS_EXT_BGRA,
            PixelFormat::Abgr => J_COLOR_SPACE::JCS_EXT_ABGR,
            PixelFormat::Argb => J_COLOR_SPACE::JCS_EXT_ARGB,
            PixelFormat::YCbCr => J_COLOR_SPACE::JCS_YCbCr,
            PixelFormat::Cmyk => J_COLOR_SPACE::JCS_CMYK,
            PixelFormat::Ycck => J_COLOR_SPACE::JCS_YCCK,
        }
    }

    /// `None` for the out-of-scope `JCS_RGB565` and for values no output
    /// color space has.
    fn from_jcs(space: J_COLOR_SPACE) -> Option<Self> {
        Some(match space {
            J_COLOR_SPACE::JCS_GRAYSCALE => PixelFormat::Grayscale,
            J_COLOR_SPACE::JCS_RGB | J_COLOR_SPACE::JCS_EXT_RGB => PixelFormat::Rgb,
            J_COLOR_SPACE::JCS_EXT_RGBX => PixelFormat::Rgbx,
            J_COLOR_SPACE::JCS_EXT_BGR => PixelFormat::Bgr,
            J_COLOR_SPACE::JCS_EXT_BGRX => PixelFormat::Bgrx,
            J_COLOR_SPACE::JCS_EXT_XBGR => PixelFormat::Xbgr,
            J_COLOR_SPACE::JCS_EXT_XRGB => PixelFormat::Xrgb,
            J_COLOR_SPACE::JCS_EXT_RGBA => PixelFormat::Rgba,
            J_COLOR_SPACE::JCS_EXT_BGRA => PixelFormat::Bgra,
            J_COLOR_SPACE::JCS_EXT_ABGR => PixelFormat::Abgr,
            J_COLOR_SPACE::JCS_EXT_ARGB => PixelFormat::Argb,
            J_COLOR_SPACE::JCS_YCbCr => PixelFormat::YCbCr,
            J_COLOR_SPACE::JCS_CMYK => PixelFormat::Cmyk,
            J_COLOR_SPACE::JCS_YCCK => PixelFormat::Ycck,
            _ => return None,
        })
    }
}

/// The inverse DCT, `dct_method`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
pub enum DctMethod {
    /// Accurate integer method, `JDCT_ISLOW`; libjpeg's default.
    #[default]
    IntSlow,
    /// Less accurate integer method, `JDCT_IFAST`.
    IntFast,
    /// Floating-point method, `JDCT_FLOAT`.
    Float,
}

impl DctMethod {
    fn to_jdct(self) -> J_DCT_METHOD {
        match self {
            DctMethod::IntSlow => J_DCT_METHOD::JDCT_ISLOW,
            DctMethod::IntFast => J_DCT_METHOD::JDCT_IFAST,
            DctMethod::Float => J_DCT_METHOD::JDCT_FLOAT,
        }
    }
}

/// The scaling factor `num / denom`, `scale_num` and `scale_denom`. libjpeg
/// decodes at `N/8` for `N` from 1 to 16, choosing the smallest `N/8` not
/// below `num / denom`.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Scale {
    /// `scale_num`.
    pub num: u32,
    /// `scale_denom`.
    pub denom: u32,
}

impl Scale {
    /// Full size.
    pub const ONE: Scale = Scale { num: 1, denom: 1 };
}

impl Default for Scale {
    fn default() -> Self {
        Scale::ONE
    }
}

/// Output parameters, set between the header and the start of decompression.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputOptions {
    /// The output layout; `None` keeps libjpeg's default for the file:
    /// grayscale for grayscale files, RGB for YCbCr and RGB files, CMYK for
    /// CMYK and YCCK files.
    pub format: Option<PixelFormat>,
    /// The scaling factor; full size by default.
    pub scale: Scale,
    /// The inverse DCT; `JDCT_ISLOW` by default.
    pub dct_method: DctMethod,
    /// `do_fancy_upsampling`; on by default.
    pub fancy_upsampling: bool,
    /// `do_block_smoothing` for progressive files; on by default.
    pub block_smoothing: bool,
}

impl Default for OutputOptions {
    /// libjpeg's defaults.
    fn default() -> Self {
        OutputOptions {
            format: None,
            scale: Scale::ONE,
            dct_method: DctMethod::IntSlow,
            fancy_upsampling: true,
            block_smoothing: true,
        }
    }
}

/// What `jpeg_read_header` found.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Header {
    /// `image_width`.
    pub width: u32,
    /// `image_height`.
    pub height: u32,
    /// `num_components`.
    pub components: u32,
    /// `jpeg_color_space`.
    pub color_space: ColorSpace,
    /// `data_precision`. Only 8 decodes (ADR 0001); other precisions fail
    /// at [`Decoder::start`] with `JERR_BAD_PRECISION`.
    pub precision: u32,
    /// `progressive_mode`.
    pub progressive: bool,
    /// `arith_code`.
    pub arithmetic: bool,
}

/// The output image: `output_width`, `output_height`, `output_components`
/// and `out_color_space`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OutputInfo {
    /// `output_width`; after [`Scanlines::crop`], the cropped width.
    pub width: u32,
    /// `output_height`.
    pub height: u32,
    /// `output_components`, the samples per pixel.
    pub components: u32,
    /// `out_color_space`.
    pub format: PixelFormat,
}

impl OutputInfo {
    /// The bytes of one output row.
    #[must_use]
    pub fn row_bytes(&self) -> usize {
        let width = usize::try_from(self.width).unwrap_or(usize::MAX);
        let components = usize::try_from(self.components).unwrap_or(usize::MAX);
        width.saturating_mul(components)
    }
}

/// The warnings of a decode.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Report {
    /// The first [`MAX_KEPT_WARNINGS`] warnings, in order.
    pub warnings: Vec<Message>,
    /// `num_warnings`: every warning, including those not kept.
    pub warning_count: u64,
}

/// What the error manager recorded.
#[derive(Default)]
struct Log {
    error: Option<Error>,
    warnings: Vec<Message>,
}

/// The error manager of this API (ADR 0005, decision 4): records errors and
/// warnings instead of printing them. Trace messages are dropped.
struct Recorder {
    log: Rc<RefCell<Log>>,
    fail_on_warning: bool,
}

impl Recorder {
    /// The current message, as upstream's `output_message` would show it.
    fn message(cinfo: &mut jpeg_decompress_struct<'_>) -> Message {
        let mut buffer = [0; JMSG_LENGTH_MAX];
        format_message(cinfo, &mut buffer);
        let length = buffer
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(JMSG_LENGTH_MAX);
        Message {
            code: J_MESSAGE_CODE(cinfo.err.msg_code.cast_unsigned()),
            text: String::from_utf8_lossy(&buffer[..length]).into_owned(),
        }
    }
}

impl jpeg_error_methods for Recorder {
    fn error_exit(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        let message = Recorder::message(cinfo);
        self.log.borrow_mut().error = Some(Error::Library(message));
    }

    fn emit_message(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        msg_level: i32,
    ) -> Result<(), ErrorExit> {
        if msg_level >= 0 {
            return Ok(());
        }
        /* Always count warnings in num_warnings, as upstream's emit_message. */
        cinfo.err.num_warnings += 1;
        let message = Recorder::message(cinfo);
        let mut log = self.log.borrow_mut();
        if self.fail_on_warning {
            log.error = Some(Error::Warning(message));
            return Err(ErrorExit);
        }
        if log.warnings.len() < MAX_KEPT_WARNINGS {
            log.warnings.push(message);
        }
        Ok(())
    }

    fn output_message(&mut self, _cinfo: &mut jpeg_decompress_struct<'_>) {}
}

/// A decompression object that `jpeg_destroy_decompress` releases on drop.
struct Object<'a> {
    cinfo: jpeg_decompress_struct<'a>,
    log: Rc<RefCell<Log>>,
}

impl Object<'_> {
    /// The error that made a libjpeg call return `Err`.
    fn error(&mut self) -> Error {
        if let Some(error) = self.log.borrow_mut().error.take() {
            return error;
        }
        // A method returned `Err` without the error manager seeing why; the
        // translated library raises every error through `error_exit`.
        Error::Library(Recorder::message(&mut self.cinfo))
    }

    fn check<T>(&mut self, result: Result<T, ErrorExit>) -> Result<T, Error> {
        result.map_err(|ErrorExit| self.error())
    }

    fn report(&self) -> Report {
        Report {
            warnings: self.log.borrow().warnings.clone(),
            warning_count: u64::try_from(self.cinfo.err.num_warnings).unwrap_or(0),
        }
    }

    fn apply(&mut self, options: &OutputOptions) {
        let cinfo = &mut self.cinfo;
        if let Some(format) = options.format {
            cinfo.out_color_space = format.to_jcs();
        }
        cinfo.scale_num = options.scale.num;
        cinfo.scale_denom = options.scale.denom;
        cinfo.dct_method = options.dct_method.to_jdct();
        cinfo.do_fancy_upsampling = options.fancy_upsampling;
        cinfo.do_block_smoothing = options.block_smoothing;
    }

    fn output_info(&self) -> Result<OutputInfo, Error> {
        let cinfo = &self.cinfo;
        let format = PixelFormat::from_jcs(cinfo.out_color_space).ok_or(Error::InvalidArgument(
            "the output color space is out of scope",
        ))?;
        Ok(OutputInfo {
            width: cinfo.output_width,
            height: cinfo.output_height,
            components: u32::try_from(cinfo.output_components).unwrap_or(0),
            format,
        })
    }
}

impl Drop for Object<'_> {
    fn drop(&mut self) {
        // jpeg_destroy releases the pools; it raises no error once the
        // memory manager exists, and nothing remains to report.
        let _ = jpeg_destroy_decompress(&mut self.cinfo);
    }
}

/// A JPEG image whose header has been read.
pub struct Decoder<'a> {
    object: Object<'a>,
    header: Header,
}

impl<'a> Decoder<'a> {
    /// Reads the header of `data` with the default options.
    ///
    /// # Errors
    ///
    /// The libjpeg error of `jpeg_read_header`, for example `JERR_NO_SOI`
    /// for data that is not a JPEG image.
    pub fn new(data: &'a [u8]) -> Result<Self, Error> {
        Decoder::with_options(data, &DecoderOptions::default())
    }

    /// Reads the header of `data`, keeping its APP1 and APP2 markers for
    /// [`Decoder::exif`] and [`Decoder::icc_profile`].
    ///
    /// # Errors
    ///
    /// The libjpeg error of `jpeg_read_header`, for example `JERR_NO_SOI`
    /// for data that is not a JPEG image, or with
    /// [`DecoderOptions::fail_on_warning`], its first warning.
    pub fn with_options(data: &'a [u8], options: &DecoderOptions) -> Result<Self, Error> {
        let log = Rc::new(RefCell::new(Log::default()));
        let mut err = Box::new(jpeg_error_mgr::default());
        jpeg_std_error(&mut err);
        err.methods = Some(Box::new(Recorder {
            log: Rc::clone(&log),
            fail_on_warning: options.fail_on_warning,
        }));
        let mut object = Object {
            cinfo: jpeg_decompress_struct::new(err),
            log,
        };
        let result = jpeg_create_decompress(&mut object.cinfo);
        object.check(result)?;
        if let Some(max_memory) = options.max_memory {
            object.cinfo.mem_mut().pub_.max_memory_to_use =
                core::ffi::c_long::try_from(max_memory).unwrap_or(core::ffi::c_long::MAX);
        }
        let result = jpeg_mem_src(&mut object.cinfo, data);
        object.check(result)?;
        for marker in [JPEG_APP0 + 1, JPEG_APP0 + 2] {
            let result = jpeg_save_markers(&mut object.cinfo, marker, MARKER_LENGTH_LIMIT);
            object.check(result)?;
        }
        // require_image makes a tables-only stream an error, and the memory
        // source never suspends, so the header is complete.
        let result = jpeg_read_header(&mut object.cinfo, true);
        object.check(result)?;
        let cinfo = &object.cinfo;
        let header = Header {
            width: cinfo.image_width,
            height: cinfo.image_height,
            components: u32::try_from(cinfo.num_components).unwrap_or(0),
            color_space: ColorSpace::from_jcs(cinfo.jpeg_color_space),
            precision: u32::try_from(cinfo.data_precision).unwrap_or(0),
            progressive: cinfo.progressive_mode,
            arithmetic: cinfo.arith_code,
        };
        Ok(Decoder { object, header })
    }

    /// The header.
    #[must_use]
    pub fn header(&self) -> &Header {
        &self.header
    }

    /// The EXIF data of the first APP1 segment that starts with
    /// `Exif\0\0`: the TIFF structure after that identifier.
    #[must_use]
    pub fn exif(&self) -> Option<&[u8]> {
        let mut marker = self.object.cinfo.marker_list.as_deref();
        while let Some(saved) = marker {
            if i32::from(saved.marker) == JPEG_APP0 + 1
                && let Some(data) = saved.data.get(..saved.data_length as usize)
                && let Some(tiff) = data.strip_prefix(EXIF_IDENTIFIER)
            {
                return Some(tiff);
            }
            marker = saved.next.as_deref();
        }
        None
    }

    /// The ICC profile that `jpeg_read_icc_profile` assembles from the APP2
    /// markers, or `None` without one. A malformed profile is `None` with
    /// the warning `JWRN_BOGUS_ICC`.
    ///
    /// # Errors
    ///
    /// The libjpeg error of `jpeg_read_icc_profile`, or with
    /// [`DecoderOptions::fail_on_warning`], `JWRN_BOGUS_ICC`.
    pub fn icc_profile(&mut self) -> Result<Option<Vec<u8>>, Error> {
        let mut data = None;
        let mut length = 0;
        let result =
            jpeg_read_icc_profile(&mut self.object.cinfo, Some(&mut data), Some(&mut length));
        if !self.object.check(result)? {
            return Ok(None);
        }
        Ok(data)
    }

    /// The output image that `options` give, from
    /// `jpeg_calc_output_dimensions`, without starting decompression.
    ///
    /// # Errors
    ///
    /// The libjpeg error of `jpeg_calc_output_dimensions`, for example
    /// `JERR_BAD_SCALE` or `JERR_CONVERSION_NOTIMPL`.
    pub fn output_info(&mut self, options: &OutputOptions) -> Result<OutputInfo, Error> {
        self.object.apply(options);
        let result = jpeg_calc_output_dimensions(&mut self.object.cinfo);
        self.object.check(result)?;
        self.object.output_info()
    }

    /// The warnings so far.
    #[must_use]
    pub fn report(&self) -> Report {
        self.object.report()
    }

    /// Sets `options` and starts decompression with `jpeg_start_decompress`.
    ///
    /// # Errors
    ///
    /// The libjpeg error of `jpeg_start_decompress`, for example
    /// `JERR_CONVERSION_NOTIMPL` for an output format the file's color space
    /// does not convert to, or `JERR_BAD_PRECISION` for a file of another
    /// precision than 8. For a progressive file this decodes every scan, so
    /// the errors and warnings of the image data can come here.
    pub fn start(mut self, options: &OutputOptions) -> Result<Scanlines<'a>, Error> {
        self.object.apply(options);
        // The memory source never suspends, so the start completes.
        let result = jpeg_start_decompress(&mut self.object.cinfo);
        self.object.check(result)?;
        let info = self.object.output_info()?;
        Ok(Scanlines {
            object: self.object,
            info,
        })
    }
}

/// A decompression in progress: rows in order, from the top.
pub struct Scanlines<'a> {
    object: Object<'a>,
    info: OutputInfo,
}

impl Scanlines<'_> {
    /// The output image.
    #[must_use]
    pub fn info(&self) -> &OutputInfo {
        &self.info
    }

    /// The next row to read, `output_scanline`.
    #[must_use]
    pub fn next_row(&self) -> u32 {
        self.object.cinfo.output_scanline
    }

    /// Restricts the output to the columns `x .. x + width` with
    /// `jpeg_crop_scanline`, before the first row is read or skipped.
    /// libjpeg widens the range to iMCU boundaries; the result is the range
    /// it decodes, and [`Scanlines::info`] reports the new width.
    ///
    /// # Errors
    ///
    /// `JERR_BAD_CROP_SPEC` for a range outside the image, and
    /// `JERR_BAD_STATE` once rows have been read.
    pub fn crop(&mut self, x: u32, width: u32) -> Result<(u32, u32), Error> {
        let (mut xoffset, mut cropped) = (x, width);
        let result = jpeg_crop_scanline(
            &mut self.object.cinfo,
            Some(&mut xoffset),
            Some(&mut cropped),
        );
        self.object.check(result)?;
        self.info = self.object.output_info()?;
        Ok((xoffset, cropped))
    }

    /// Skips `rows` rows with `jpeg_skip_scanlines` and returns how many it
    /// skipped: fewer only at the bottom of the image.
    ///
    /// # Errors
    ///
    /// The libjpeg errors and, with [`DecoderOptions::fail_on_warning`],
    /// warnings of decoding the skipped rows.
    pub fn skip_rows(&mut self, rows: u32) -> Result<u32, Error> {
        let result = jpeg_skip_scanlines(&mut self.object.cinfo, rows);
        self.object.check(result)
    }

    /// Reads rows into `buffer`, which holds whole rows of
    /// [`OutputInfo::row_bytes`] each, with `jpeg_read_scanlines` until the
    /// buffer is full or the image ends, and returns the number of rows read.
    /// At the bottom of the image it returns 0 without calling libjpeg.
    ///
    /// # Errors
    ///
    /// [`Error::InvalidArgument`] for a buffer that is empty or not a
    /// multiple of the row length, and the libjpeg errors and, with
    /// [`DecoderOptions::fail_on_warning`], warnings of decoding the rows.
    pub fn read_rows(&mut self, buffer: &mut [u8]) -> Result<u32, Error> {
        let row_bytes = self.info.row_bytes();
        if buffer.is_empty() || row_bytes == 0 || !buffer.len().is_multiple_of(row_bytes) {
            return Err(Error::InvalidArgument(
                "the buffer must hold one or more whole rows",
            ));
        }
        let remaining = self.info.height - self.object.cinfo.output_scanline;
        let wanted = u32::try_from(buffer.len() / row_bytes)
            .unwrap_or(u32::MAX)
            .min(remaining);
        let offsets: Vec<usize> = (0..wanted as usize).map(|row| row * row_bytes).collect();
        let mut read = 0;
        while read < wanted {
            let start = read as usize;
            let result = jpeg_read_scanlines(
                &mut self.object.cinfo,
                RowListMut::Shared {
                    storage: &mut buffer[start * row_bytes..],
                    rows: &offsets[..(wanted - read) as usize],
                },
                wanted - read,
            );
            let lines = self.object.check(result)?;
            if lines == 0 {
                break;
            }
            read += lines;
        }
        Ok(read)
    }

    /// The warnings so far.
    #[must_use]
    pub fn report(&self) -> Report {
        self.object.report()
    }

    /// Completes decompression with `jpeg_finish_decompress`, which reads the
    /// rest of the data up to the EOI marker, and returns the warnings. A
    /// decode that stops early drops the [`Scanlines`] instead.
    ///
    /// # Errors
    ///
    /// `JERR_TOO_LITTLE_DATA` when rows remain unread, and the libjpeg errors
    /// and warnings of the rest of the data.
    pub fn finish(mut self) -> Result<Report, Error> {
        let result = jpeg_finish_decompress(&mut self.object.cinfo);
        self.object.check(result)?;
        Ok(self.object.report())
    }
}
