//! `djpeg` on the port: the library calls that upstream's `src/djpeg.c` makes
//! and the PPM/PGM output of `src/wrppm.c`, translated as test code for the
//! bit tests of `tests/wp7_bittests.rs` (ADR 0002). Include it next to
//! `support/oracle.rs`.
//!
//! The translation covers what an 8-bit decode to PPM or PGM does. The
//! switches that select another output format, color quantization, RGB565,
//! another precision, a progress monitor (`-report`, `-maxscans`), a memory
//! limit or `-strict` are outside it and make [`djpeg`] fail with a message;
//! no in-scope bit test uses them. `djpeg.c` reads the input with
//! `jpeg_stdio_src` unless `-memsrc` is given; the port has no stdio source
//! (ADR 0001), so the input is always attached with `jpeg_mem_src`, which
//! delivers the same data for a complete file. Where `djpeg.c` prints to
//! `stderr` and exits, [`djpeg`] returns the message as an error, and so it
//! does for an error exit of the library, with the message code.
//! `DJPEG_MULTILINE` is undefined, as in upstream's build, so the writer
//! emits one row per call.

#![allow(dead_code, reason = "each test binary uses part of the support")]

use jpeg::jdapimin::{
    jpeg_create_decompress, jpeg_destroy_decompress, jpeg_finish_decompress, jpeg_read_header,
};
use jpeg::jdapistd::{
    jpeg_crop_scanline, jpeg_read_scanlines, jpeg_skip_scanlines, jpeg_start_decompress,
};
use jpeg::jdatasrc::jpeg_mem_src;
use jpeg::jdicc::jpeg_read_icc_profile;
use jpeg::jdmarker::{jpeg_save_markers, jpeg_set_marker_processor};
use jpeg::jdmaster::jpeg_calc_output_dimensions;
use jpeg::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE, jpeg_std_error};
use jpeg::jmorecfg::{
    JDIMENSION, JPEG_MAX_DIMENSION, JSAMPLE, rgb_blue, rgb_green, rgb_pixelsize, rgb_red,
};
use jpeg::jpegint::IsExtRGB;
use jpeg::jpeglib::{
    J_COLOR_SPACE, J_DCT_METHOD, J_DITHER_MODE, JDCT_FASTEST, JPEG_APP0, JPEG_COM, RowListMut,
    call_fill_input_buffer, jpeg_decompress_struct, jpeg_error_mgr, jpeg_marker_processor,
};

/// What a `djpeg` run produced.
#[derive(Debug)]
pub struct Output {
    /// The PPM or PGM file.
    pub image: Vec<u8>,
    /// The ICC profile that `-icc FILE` writes, with the file name.
    pub icc: Option<(String, Vec<u8>)>,
    /// `jerr.num_warnings`, which makes `djpeg` exit with `EXIT_WARNING`.
    pub num_warnings: i64,
}

// upstream: src/cdjpeg.c keymatch
/// Case-insensitive matching of possibly-abbreviated keyword switches.
/// `keyword` is the constant keyword (must be lower case already),
/// `minchars` is length of minimum legal abbreviation.
fn keymatch(arg: &str, keyword: &str, minchars: usize) -> bool {
    let mut keyword = keyword.bytes();
    let mut nmatched = 0;

    for ca in arg.bytes() {
        let Some(ck) = keyword.next() else {
            return false; /* arg longer than keyword, no good */
        };
        if ca.to_ascii_lowercase() != ck {
            return false; /* no good */
        }
        nmatched += 1; /* count matched characters */
    }
    /* reached end of argument; fail if it's too short for unique abbrev */
    nmatched >= minchars
}

/// `djpeg.c`'s switch variables.
#[derive(Default)]
struct Switches {
    icc_filename: Option<String>,
    skip: Option<(JDIMENSION, JDIMENSION)>,
    crop: Option<(JDIMENSION, JDIMENSION, JDIMENSION, JDIMENSION)>,
}

/// `usage()`, which prints the switches and exits.
fn usage(arg: &str) -> String {
    format!("djpeg: bad switch or argument {arg:?}")
}

/// The part of `sscanf(text, "%u/%u")`, `"%d,%d"` and the crop format that a
/// well-formed argument uses: an unsigned decimal number at the start of
/// `text`, and the rest. Not an upstream item.
fn number(text: &str) -> Option<(u64, &str)> {
    let digits = text.bytes().take_while(u8::is_ascii_digit).count();
    let value = text[..digits].parse().ok()?;
    Some((value, &text[digits..]))
}

// upstream: src/djpeg.c parse_switches
/// Parses optional switches into `cinfo` and the switch variables. `args`
/// holds the switches of a bit test, without the program name, `-outfile`
/// and the input file. `for_real` is false on the first (dummy) pass.
#[expect(clippy::too_many_lines, reason = "djpeg.c's switch chain")]
fn parse_switches(
    cinfo: &mut jpeg_decompress_struct<'_>,
    args: &[&str],
    for_real: bool,
) -> Result<Switches, String> {
    let _ = for_real; /* only -map, which is outside the translation, uses it */

    /* Set up default JPEG parameters. */
    let mut switches = Switches::default();
    cinfo.err.trace_level = 0;

    /* Scan command line options, adjust parameters */
    let mut argn = 0;
    while argn < args.len() {
        let Some(arg) = args[argn].strip_prefix('-') else {
            return Err(usage(args[argn])); /* the bit tests pass no file names here */
        };
        let next = |argn: &mut usize| -> Result<&str, String> {
            *argn += 1; /* advance to next argument */
            args.get(*argn).copied().ok_or_else(|| usage(arg))
        };

        let outside = |name: &str| format!("djpeg -{name} is outside the translated driver");
        if keymatch(arg, "bmp", 1) {
            return Err(outside("bmp"));
        } else if keymatch(arg, "colors", 1)
            || keymatch(arg, "colours", 1)
            || keymatch(arg, "quantize", 1)
            || keymatch(arg, "quantise", 1)
        {
            return Err(outside("colors"));
        } else if keymatch(arg, "dct", 2) {
            /* Select IDCT algorithm. */
            let value = next(&mut argn)?;
            if keymatch(value, "int", 1) {
                cinfo.dct_method = J_DCT_METHOD::JDCT_ISLOW;
            } else if keymatch(value, "fast", 2) {
                cinfo.dct_method = J_DCT_METHOD::JDCT_IFAST;
            } else if keymatch(value, "float", 2) {
                cinfo.dct_method = J_DCT_METHOD::JDCT_FLOAT;
            } else {
                return Err(usage(value));
            }
        } else if keymatch(arg, "dither", 2) {
            return Err(outside("dither"));
        } else if keymatch(arg, "debug", 1) || keymatch(arg, "verbose", 1) {
            /* Enable debug printouts. */
            cinfo.err.trace_level += 1;
        } else if keymatch(arg, "version", 4) {
            return Err(outside("version"));
        } else if keymatch(arg, "fast", 1) {
            /* Select recommended processing options for quick-and-dirty output. */
            cinfo.two_pass_quantize = false;
            cinfo.dither_mode = J_DITHER_MODE::JDITHER_ORDERED;
            if !cinfo.quantize_colors {
                /* don't override an earlier -colors */
                cinfo.desired_number_of_colors = 216;
            }
            cinfo.dct_method = JDCT_FASTEST;
            cinfo.do_fancy_upsampling = false;
        } else if keymatch(arg, "gif", 1) || keymatch(arg, "gif0", 4) {
            return Err(outside("gif"));
        } else if keymatch(arg, "grayscale", 2) || keymatch(arg, "greyscale", 2) {
            /* Force monochrome output. */
            cinfo.out_color_space = J_COLOR_SPACE::JCS_GRAYSCALE;
        } else if keymatch(arg, "rgb", 2) {
            /* Force RGB output. */
            cinfo.out_color_space = J_COLOR_SPACE::JCS_RGB;
        } else if keymatch(arg, "rgb565", 2) {
            return Err(outside("rgb565"));
        } else if keymatch(arg, "icc", 1) {
            /* Set ICC filename. */
            switches.icc_filename = Some(next(&mut argn)?.to_owned());
            jpeg_save_markers(cinfo, JPEG_APP0 + 2, 0xFFFF).map_err(|_| library_error(cinfo))?;
        } else if keymatch(arg, "noicc", 3) {
            /* noicc only matters for PNG output */
        } else if keymatch(arg, "map", 3) {
            return Err(outside("map"));
        } else if keymatch(arg, "maxmemory", 3) {
            return Err(outside("maxmemory"));
        } else if keymatch(arg, "maxscans", 4) {
            return Err(outside("maxscans"));
        } else if keymatch(arg, "nosmooth", 3) {
            /* Suppress fancy upsampling */
            cinfo.do_fancy_upsampling = false;
        } else if keymatch(arg, "onepass", 3) {
            /* Use fast one-pass quantization. */
            cinfo.two_pass_quantize = false;
        } else if keymatch(arg, "os2", 3) {
            return Err(outside("os2"));
        } else if keymatch(arg, "outfile", 4) {
            return Err(usage(arg)); /* the runner returns the output */
        } else if keymatch(arg, "nooverwrite", 3) {
            /* only matters for an output file */
        } else if keymatch(arg, "memsrc", 2) {
            /* the runner always uses the in-memory source manager */
        } else if keymatch(arg, "pnm", 1) || keymatch(arg, "ppm", 1) {
            /* PPM/PGM output format. */
        } else if keymatch(arg, "png", 3) {
            return Err(outside("png"));
        } else if keymatch(arg, "precision", 2) {
            return Err(outside("precision"));
        } else if keymatch(arg, "report", 2) {
            return Err(outside("report"));
        } else if keymatch(arg, "scale", 1) {
            /* Scale the output image by a fraction M/N. */
            let value = next(&mut argn)?;
            let parsed = number(value).and_then(|(num, rest)| {
                let (denom, _) = number(rest.strip_prefix('/')?)?;
                Some((u32::try_from(num).ok()?, u32::try_from(denom).ok()?))
            });
            let (num, denom) = parsed.ok_or_else(|| usage(value))?;
            cinfo.scale_num = num;
            cinfo.scale_denom = denom;
        } else if keymatch(arg, "skip", 2) {
            let value = next(&mut argn)?;
            let parsed = number(value).and_then(|(start, rest)| {
                let (end, _) = number(rest.strip_prefix(',')?)?;
                Some((start, end))
            });
            let (start, end) = parsed.ok_or_else(|| usage(value))?;
            if start > end || end > u64::from(i32::MAX.cast_unsigned()) {
                return Err(usage(value));
            }
            switches.skip = Some((
                JDIMENSION::try_from(start).unwrap(),
                JDIMENSION::try_from(end).unwrap(),
            ));
        } else if keymatch(arg, "crop", 2) {
            let value = next(&mut argn)?;
            let parsed = number(value).and_then(|(width, rest)| {
                let rest = rest.strip_prefix(['X', 'x'])?;
                let (height, rest) = number(rest)?;
                let (x, rest) = number(rest.strip_prefix('+')?)?;
                let (y, _) = number(rest.strip_prefix('+')?)?;
                Some((width, height, x, y))
            });
            let (width, height, x, y) = parsed.ok_or_else(|| usage(value))?;
            if width < 1 || height < 1 {
                return Err(usage(value));
            }
            let dimension = |v: u64| JDIMENSION::try_from(v).map_err(|_| usage(value));
            switches.crop = Some((
                dimension(width)?,
                dimension(height)?,
                dimension(x)?,
                dimension(y)?,
            ));
        } else if keymatch(arg, "strict", 2) {
            return Err(outside("strict"));
        } else if keymatch(arg, "targa", 1) {
            return Err(outside("targa"));
        } else {
            return Err(usage(arg)); /* bogus switch */
        }
        argn += 1;
    }

    Ok(switches)
}

/// The message of a library error: `djpeg` prints it through the standard
/// error manager and exits. Not an upstream item.
fn library_error(cinfo: &jpeg_decompress_struct<'_>) -> String {
    format!("djpeg: library error, message code {}", cinfo.err.msg_code)
}

// upstream: src/djpeg.c jpeg_getc
/// Reads the next byte; the source must not suspend.
fn jpeg_getc(cinfo: &mut jpeg_decompress_struct<'_>) -> Result<u32, ErrorExit> {
    if cinfo.src_mut().bytes_in_buffer == 0 && !call_fill_input_buffer(cinfo)? {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CANT_SUSPEND));
    }
    let datasrc = cinfo.src_mut();
    datasrc.bytes_in_buffer -= 1;
    let byte = datasrc.buffer.bytes()[datasrc.next_input_byte];
    datasrc.next_input_byte += 1;
    Ok(u32::from(byte))
}

// upstream: src/djpeg.c print_text_marker
/// Marker processor for COM and interesting APPn markers. This replaces the
/// library's built-in processor, which just skips the marker. We want to
/// print out the marker as text, to the extent possible. Note this code
/// relies on a non-suspending data source.
struct PrintTextMarker;

impl jpeg_marker_processor for PrintTextMarker {
    fn process_marker(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
    ) -> Result<bool, ErrorExit> {
        let traceit = cinfo.err.trace_level >= 1;
        let mut lastch: u32 = 0;

        let mut length = i64::from(jpeg_getc(cinfo)? << 8);
        length += i64::from(jpeg_getc(cinfo)?);
        length -= 2; /* discount the length word itself */

        if traceit {
            if cinfo.unread_marker == JPEG_COM {
                eprintln!("Comment, length {length}:");
            } else {
                /* assume it is an APPn otherwise */
                eprintln!("APP{}, length {length}:", cinfo.unread_marker - JPEG_APP0);
            }
        }

        loop {
            length -= 1;
            if length < 0 {
                break;
            }
            let ch = jpeg_getc(cinfo)?;
            if traceit {
                /* Emit the character in a readable form.
                 * Nonprintables are converted to \nnn form,
                 * while \ is converted to \\.
                 * Newlines in CR, CR/LF, or LF form will be printed as one newline.
                 */
                if ch == u32::from(b'\r') {
                    eprintln!();
                } else if ch == u32::from(b'\n') {
                    if lastch != u32::from(b'\r') {
                        eprintln!();
                    }
                } else if ch == u32::from(b'\\') {
                    eprint!("\\\\");
                } else if (0x20..0x7F).contains(&ch) {
                    eprint!("{}", char::from(u8::try_from(ch).unwrap()));
                } else {
                    eprint!("\\{ch:03o}");
                }
                lastch = ch;
            }
        }

        if traceit {
            eprintln!();
        }

        Ok(true)
    }
}

/// How `wrppm.c` emits a row. Not an upstream item.
#[derive(Clone, Copy)]
enum PutPixelRows {
    /// `put_pixel_rows`: write the decompressor's output buffer as is.
    Direct,
    /// `copy_pixel_rows`.
    Copy,
    /// `put_rgb`.
    Rgb,
    /// `put_cmyk`.
    Cmyk,
}

// upstream: src/wrppm.c ppm_dest_struct
/// Private version of data destination object.
struct PpmDest {
    /// The output file.
    output_file: Vec<u8>,
    /// fwrite's I/O buffer; also the decompressor output buffer in the
    /// direct case.
    iobuffer: Vec<JSAMPLE>,
    /// The decompressor output buffer where it differs from `iobuffer`.
    buffer: Option<Vec<JSAMPLE>>,
    /// Width of I/O buffer.
    buffer_width: usize,
    /// Samples per output row.
    samples_per_row: JDIMENSION,
    put_pixel_rows: PutPixelRows,
}

/// `PPM_MAXVAL` for 8-bit samples.
fn ppm_maxval(cinfo: &jpeg_decompress_struct<'_>) -> i32 {
    (1 << cinfo.data_precision) - 1
}

// upstream: src/cmyk.h cmyk_to_rgb
/// Converts CMYK to RGB.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's conversion of a rounded double to JSAMPLE"
)]
fn cmyk_to_rgb(maxval: i32, c: JSAMPLE, m: JSAMPLE, y: JSAMPLE, k: JSAMPLE) -> [JSAMPLE; 3] {
    let maxval = f64::from(maxval);
    let k = f64::from(k);
    [
        (f64::from(c) * k / maxval + 0.5) as JSAMPLE,
        (f64::from(m) * k / maxval + 0.5) as JSAMPLE,
        (f64::from(y) * k / maxval + 0.5) as JSAMPLE,
    ]
}

impl PpmDest {
    /// `dest->pub.buffer` as the row list of `jpeg_read_scanlines`, with
    /// `buffer_height` (always 1) rows.
    fn rows(&mut self) -> &mut [JSAMPLE] {
        match &mut self.buffer {
            Some(buffer) => buffer,
            None => &mut self.iobuffer,
        }
    }

    // upstream: src/wrppm.c put_pixel_rows
    // upstream: src/wrppm.c copy_pixel_rows
    // upstream: src/wrppm.c put_rgb
    // upstream: src/wrppm.c put_cmyk
    /// Writes some pixel data. In this module `rows_supplied` will always be
    /// 1, and as in C, a row is written even when it is 0.
    #[expect(clippy::cast_sign_loss, reason = "C's nonnegative indexes")]
    fn put_pixel_rows(&mut self, cinfo: &jpeg_decompress_struct<'_>, _rows_supplied: JDIMENSION) {
        match self.put_pixel_rows {
            PutPixelRows::Direct => {}
            PutPixelRows::Copy => {
                let ptr = self.buffer.as_ref().expect("separate buffer");
                self.iobuffer[..self.samples_per_row as usize]
                    .copy_from_slice(&ptr[..self.samples_per_row as usize]);
            }
            PutPixelRows::Rgb => {
                let space = cinfo.out_color_space.0 as usize;
                let (rindex, gindex, bindex, ps) = (
                    rgb_red[space] as usize,
                    rgb_green[space] as usize,
                    rgb_blue[space] as usize,
                    rgb_pixelsize[space] as usize,
                );
                let ptr = self.buffer.as_ref().expect("separate buffer");
                for col in 0..cinfo.output_width as usize {
                    let pixel = &ptr[col * ps..];
                    self.iobuffer[col * 3] = pixel[rindex];
                    self.iobuffer[col * 3 + 1] = pixel[gindex];
                    self.iobuffer[col * 3 + 2] = pixel[bindex];
                }
            }
            PutPixelRows::Cmyk => {
                let maxval = ppm_maxval(cinfo);
                let ptr = self.buffer.as_ref().expect("separate buffer");
                for col in 0..cinfo.output_width as usize {
                    let p = &ptr[col * 4..col * 4 + 4];
                    let rgb = cmyk_to_rgb(maxval, p[0], p[1], p[2], p[3]);
                    self.iobuffer[col * 3..col * 3 + 3].copy_from_slice(&rgb);
                }
            }
        }
        self.output_file
            .extend_from_slice(&self.iobuffer[..self.buffer_width]);
    }

    // upstream: src/wrppm.c start_output_ppm
    /// Startup: writes the file header.
    fn start_output(&mut self, cinfo: &jpeg_decompress_struct<'_>) -> Result<(), String> {
        /* Emit file header */
        use J_COLOR_SPACE as CS;
        let kind = match cinfo.out_color_space {
            /* emit header for raw PGM format */
            CS::JCS_GRAYSCALE => "P5",
            CS::JCS_RGB
            | CS::JCS_EXT_RGB
            | CS::JCS_EXT_RGBX
            | CS::JCS_EXT_BGR
            | CS::JCS_EXT_BGRX
            | CS::JCS_EXT_XBGR
            | CS::JCS_EXT_XRGB
            | CS::JCS_EXT_RGBA
            | CS::JCS_EXT_BGRA
            | CS::JCS_EXT_ABGR
            | CS::JCS_EXT_ARGB
            | CS::JCS_CMYK => {
                if !IsExtRGB(cinfo.out_color_space) && cinfo.quantize_colors {
                    return Err("djpeg: JERR_PPM_COLORSPACE".to_owned());
                }
                /* emit header for raw PPM format */
                "P6"
            }
            _ => return Err("djpeg: JERR_PPM_COLORSPACE".to_owned()),
        };
        self.output_file.extend_from_slice(
            format!(
                "{kind}\n{} {}\n{}\n",
                cinfo.output_width,
                cinfo.output_height,
                ppm_maxval(cinfo)
            )
            .as_bytes(),
        );
        Ok(())
    }

    // upstream: src/wrppm.c calc_buffer_dimensions_ppm
    /// Re-calculates buffer dimensions based on output dimensions.
    #[expect(clippy::cast_sign_loss, reason = "C's nonnegative component count")]
    fn calc_buffer_dimensions(&mut self, cinfo: &jpeg_decompress_struct<'_>) {
        self.samples_per_row = if cinfo.out_color_space == J_COLOR_SPACE::JCS_GRAYSCALE {
            cinfo.output_width * cinfo.out_color_components as JDIMENSION
        } else {
            cinfo.output_width * 3
        };
        self.buffer_width = self.samples_per_row as usize; /* BYTESPERSAMPLE is 1 */
    }
}

// upstream: src/wrppm.c _jinit_write_ppm
/// The module selection routine for PPM format output, 8-bit instance.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative component count")]
fn jinit_write_ppm(cinfo: &mut jpeg_decompress_struct<'_>) -> Result<PpmDest, ErrorExit> {
    if cinfo.data_precision > 8 || cinfo.data_precision < 2 {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    /* Create module interface object: C allocates it from JPOOL_IMAGE */
    let mut dest = PpmDest {
        output_file: Vec::new(),
        iobuffer: Vec::new(),
        buffer: None,
        buffer_width: 0,
        samples_per_row: 0,
        put_pixel_rows: PutPixelRows::Direct,
    };

    /* Calculate output image dimensions so we can allocate space */
    if i64::from(cinfo.image_width) > JPEG_MAX_DIMENSION
        || i64::from(cinfo.image_height) > JPEG_MAX_DIMENSION
    {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_IMAGE_TOO_BIG,
            i32::try_from(JPEG_MAX_DIMENSION).unwrap(),
        ));
    }
    jpeg_calc_output_dimensions(cinfo)?;

    /* Create physical I/O buffer */
    dest.calc_buffer_dimensions(cinfo);
    dest.iobuffer = vec![0; dest.buffer_width];

    /* RGB_RED == 0, RGB_GREEN == 1, RGB_BLUE == 2 and RGB_PIXELSIZE == 3 */
    if cinfo.quantize_colors
        || (cinfo.out_color_space != J_COLOR_SPACE::JCS_EXT_RGB
            && cinfo.out_color_space != J_COLOR_SPACE::JCS_RGB
            && cinfo.out_color_space != J_COLOR_SPACE::JCS_GRAYSCALE)
    {
        /* When quantizing, we need an output buffer for colormap indexes
         * that's separate from the physical I/O buffer.  We also need a
         * separate buffer if pixel format translation must take place.
         */
        dest.buffer = Some(vec![
            0;
            (cinfo.output_width * cinfo.output_components as JDIMENSION)
                as usize
        ]);
        /* put_demapped_gray and put_demapped_rgb serve color quantization,
         * which parse_switches never requests */
        assert!(!cinfo.quantize_colors, "quantization is outside the driver");
        if IsExtRGB(cinfo.out_color_space) {
            dest.put_pixel_rows = PutPixelRows::Rgb;
        } else if cinfo.out_color_space == J_COLOR_SPACE::JCS_CMYK {
            dest.put_pixel_rows = PutPixelRows::Cmyk;
        } else {
            /* JCS_RGB565 (put_rgb565) is out of scope and rejected by
             * jpeg_start_decompress */
            dest.put_pixel_rows = PutPixelRows::Copy;
        }
    } else {
        /* We will fwrite() directly from decompressor output buffer. */
        dest.put_pixel_rows = PutPixelRows::Direct;
    }

    Ok(dest)
}

/// One `jpeg_read_scanlines` call into the writer's buffer. Not an upstream
/// item.
fn read_scanlines(
    cinfo: &mut jpeg_decompress_struct<'_>,
    dest: &mut PpmDest,
    max_lines: JDIMENSION,
) -> Result<JDIMENSION, ErrorExit> {
    jpeg_read_scanlines(cinfo, RowListMut::Separate(&mut [dest.rows()]), max_lines)
}

// upstream: src/djpeg.c main
/// Runs `djpeg` with the switches `args` on the input `data` and returns its
/// output, or the message with which `djpeg` would exit.
///
/// # Errors
///
/// What `djpeg` prints before it exits with a failure.
#[expect(clippy::too_many_lines, reason = "djpeg.c's main")]
pub fn djpeg(args: &[&str], data: &[u8]) -> Result<Output, String> {
    let mut jerr = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut jerr);
    let mut cinfo = jpeg_decompress_struct::new(jerr);
    let fail = |cinfo: &jpeg_decompress_struct<'_>| library_error(cinfo);

    /* Initialize the JPEG decompression object with default error handling. */
    jpeg_create_decompress(&mut cinfo).map_err(|_| fail(&cinfo))?;
    /* The add-on message table of cderror.h serves the writers' own errors,
     * which this translation returns as messages. */

    /* Insert custom marker processor for COM and APP12. */
    jpeg_set_marker_processor(&mut cinfo, JPEG_COM, Box::new(PrintTextMarker))
        .map_err(|_| fail(&cinfo))?;
    jpeg_set_marker_processor(&mut cinfo, JPEG_APP0 + 12, Box::new(PrintTextMarker))
        .map_err(|_| fail(&cinfo))?;

    /* Scan command line to find file names. */
    parse_switches(&mut cinfo, args, false)?;

    /* Specify data source for decompression: jpeg_mem_src (see the module
     * documentation). */
    jpeg_mem_src(&mut cinfo, data).map_err(|_| fail(&cinfo))?;

    /* Read file header, set default decompression parameters */
    jpeg_read_header(&mut cinfo, true).map_err(|_| fail(&cinfo))?;

    /* Adjust default decompression parameters by re-parsing the options */
    let switches = parse_switches(&mut cinfo, args, true)?;

    /* Initialize the output module now to let it override any crucial
     * option settings (for instance, GIF wants to force color quantization).
     */
    let mut dest = jinit_write_ppm(&mut cinfo).map_err(|_| fail(&cinfo))?;

    /* Start decompressor */
    jpeg_start_decompress(&mut cinfo).map_err(|_| fail(&cinfo))?;

    if let Some((skip_start, skip_end)) = switches.skip {
        /* Skip rows */

        /* Check for valid skip_end.  We cannot check this value until after
         * jpeg_start_decompress() is called.  Note that we have already verified
         * that skip_start <= skip_end.
         */
        if skip_end > cinfo.output_height.wrapping_sub(1) {
            return Err(format!(
                "djpeg: skip region exceeds image height {}",
                cinfo.output_height
            ));
        }

        /* Write output file header.  This is a hack to ensure that the destination
         * manager creates an output image of the proper size.
         */
        let tmp = cinfo.output_height;
        cinfo.output_height -= skip_end - skip_start + 1;
        dest.start_output(&cinfo)?;
        cinfo.output_height = tmp;

        /* Process data: jinit_write_ppm has rejected every precision but 8,
         * so the branches for 12-bit samples and JERR_NOTIMPL never run */
        while cinfo.output_scanline < skip_start {
            let max_lines = 1; /* dest_mgr->buffer_height */
            let num_scanlines =
                read_scanlines(&mut cinfo, &mut dest, max_lines).map_err(|_| fail(&cinfo))?;
            dest.put_pixel_rows(&cinfo, num_scanlines);
        }
        let tmp =
            jpeg_skip_scanlines(&mut cinfo, skip_end - skip_start + 1).map_err(|_| fail(&cinfo))?;
        if tmp != skip_end - skip_start + 1 {
            return Err(format!(
                "djpeg: jpeg_skip_scanlines() returned {tmp} rather than {}",
                skip_end - skip_start + 1
            ));
        }
        while cinfo.output_scanline < cinfo.output_height {
            let num_scanlines =
                read_scanlines(&mut cinfo, &mut dest, 1).map_err(|_| fail(&cinfo))?;
            dest.put_pixel_rows(&cinfo, num_scanlines);
        }
    } else if let Some((mut crop_width, crop_height, mut crop_x, crop_y)) = switches.crop {
        /* Decompress a subregion */

        /* Check for valid crop dimensions.  We cannot check these values until
         * after jpeg_start_decompress() is called.
         */
        if u64::from(crop_x) + u64::from(crop_width) > u64::from(cinfo.output_width)
            || u64::from(crop_y) + u64::from(crop_height) > u64::from(cinfo.output_height)
        {
            return Err(format!(
                "djpeg: crop dimensions exceed image dimensions {} x {}",
                cinfo.output_width, cinfo.output_height
            ));
        }

        /* data_precision is 8 (see the skip case) */
        jpeg_crop_scanline(&mut cinfo, Some(&mut crop_x), Some(&mut crop_width))
            .map_err(|_| fail(&cinfo))?;
        dest.calc_buffer_dimensions(&cinfo);

        /* Write output file header.  This is a hack to ensure that the destination
         * manager creates an output image of the proper size.
         */
        let tmp = cinfo.output_height;
        cinfo.output_height = crop_height;
        dest.start_output(&cinfo)?;
        cinfo.output_height = tmp;

        /* Process data */
        let tmp = jpeg_skip_scanlines(&mut cinfo, crop_y).map_err(|_| fail(&cinfo))?;
        if tmp != crop_y {
            return Err(format!(
                "djpeg: jpeg_skip_scanlines() returned {tmp} rather than {crop_y}"
            ));
        }
        while cinfo.output_scanline < crop_y + crop_height {
            let num_scanlines =
                read_scanlines(&mut cinfo, &mut dest, 1).map_err(|_| fail(&cinfo))?;
            dest.put_pixel_rows(&cinfo, num_scanlines);
        }
        let rest = cinfo.output_height - crop_y - crop_height;
        let tmp = jpeg_skip_scanlines(&mut cinfo, rest).map_err(|_| fail(&cinfo))?;
        if tmp != rest {
            return Err(format!(
                "djpeg: jpeg_skip_scanlines() returned {tmp} rather than {rest}"
            ));
        }
    } else {
        /* Normal full-image decompress */

        /* Write output file header */
        dest.start_output(&cinfo)?;

        /* Process data (data_precision is 8, see the skip case) */
        while cinfo.output_scanline < cinfo.output_height {
            let num_scanlines =
                read_scanlines(&mut cinfo, &mut dest, 1).map_err(|_| fail(&cinfo))?;
            dest.put_pixel_rows(&cinfo, num_scanlines);
        }
    }

    let mut icc = None;
    if let Some(icc_filename) = switches.icc_filename {
        let mut icc_profile = None;
        let mut icc_len = 0;
        if jpeg_read_icc_profile(&mut cinfo, Some(&mut icc_profile), Some(&mut icc_len))
            .map_err(|_| fail(&cinfo))?
        {
            icc = Some((icc_filename, icc_profile.expect("ICC profile")));
        } else {
            if cinfo.err.msg_code != J_MESSAGE_CODE::JWRN_BOGUS_ICC.0.cast_signed() {
                eprintln!("djpeg: no ICC profile data in JPEG file");
            }
            /* C has opened the file, which stays empty */
            icc = Some((icc_filename, Vec::new()));
        }
    }

    /* Finish decompression and release memory.
     * I must do it in this order because output module has allocated memory
     * of lifespan JPOOL_IMAGE; it needs to finish before releasing memory.
     */
    /* finish_output_ppm only flushes the output file */
    jpeg_finish_decompress(&mut cinfo).map_err(|_| fail(&cinfo))?;
    let num_warnings = cinfo.err.num_warnings;
    jpeg_destroy_decompress(&mut cinfo).map_err(|_| fail(&cinfo))?;

    Ok(Output {
        image: dest.output_file,
        icc,
        num_warnings,
    })
}
