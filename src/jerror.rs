// libjpeg-turbo 3.2.1, src/jerror.h:
//
// /*
//  * jerror.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1997, Thomas G. Lane.
//  * Modified 1997-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2014, 2017, 2021-2023, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file defines the error and message codes for the JPEG library.
//  * Edit this file to add new codes, or to translate the message strings to
//  * some other language.
//  * A set of error-reporting macros are defined too.  Some applications using
//  * the JPEG library may wish to include this file to get the error codes
//  * and/or the macros.
//  */
// libjpeg-turbo 3.2.1, src/jerror.c:
//
// /*
//  * jerror.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1998, Thomas G. Lane.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2022, 2024, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains simple error-reporting and trace-message routines.
//  * These are suitable for Unix-like systems and others where writing to
//  * stderr is the right thing to do.  Many applications will want to replace
//  * some or all of these routines.
//  *
//  * If you define USE_WINDOWS_MESSAGEBOX in jconfig.h or in the makefile,
//  * you get a Windows-specific hack to display error messages in a dialog box.
//  * It ain't much, but it beats dropping error messages into the bit bucket,
//  * which is what happens to output to stderr under most Windows C compilers.
//  *
//  * These routines are used by both the compression and decompression code.
//  */
//! Translation of `src/jerror.h` and `src/jerror.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! The message list is `jerror.h` as compiled with `JPEG_LIB_VERSION` 62 and
//! arithmetic coding enabled (`jconfig.h`), which fixes every code's value.
//! The macros are functions of the same names (ADR 0005, "Error exits and
//! warnings"): `ERREXIT` and its variants store the code and parameters as the
//! macros do, call the `error_exit` hook and return the [`ErrorExit`] marker
//! for the caller to return as `Err`; `WARNMS` and `TRACEMS` and their
//! variants call `emit_message` and propagate its result.
//!
//! The default methods of [`jpeg_error_methods`] are upstream's functions,
//! which are also free functions here so that an application's override can
//! still call them, as a C application keeps the pointer that
//! `jpeg_std_error` installed. The default `error_exit` prints the message and
//! returns; it neither destroys the object nor exits (ADR 0005, "The default
//! error manager"). `USE_WINDOWS_MESSAGEBOX` is not defined on the CI hosts.

use std::io::Write;

use crate::jpeglib::{
    JMSG_LENGTH_MAX, JMSG_STR_PARM_MAX, j_common_ptr, jpeg_decompress_struct, jpeg_error_mgr,
};
use crate::jversion::{JCOPYRIGHT, JVERSION};

// upstream: src/jerror.h J_MESSAGE_CODE
/// The message codes of the JPEG library.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct J_MESSAGE_CODE(pub u32);

impl J_MESSAGE_CODE {
    pub const JMSG_NOMESSAGE: J_MESSAGE_CODE = J_MESSAGE_CODE(0);
    pub const JERR_ARITH_NOTIMPL: J_MESSAGE_CODE = J_MESSAGE_CODE(1);
    pub const JERR_BAD_ALIGN_TYPE: J_MESSAGE_CODE = J_MESSAGE_CODE(2);
    pub const JERR_BAD_ALLOC_CHUNK: J_MESSAGE_CODE = J_MESSAGE_CODE(3);
    pub const JERR_BAD_BUFFER_MODE: J_MESSAGE_CODE = J_MESSAGE_CODE(4);
    pub const JERR_BAD_COMPONENT_ID: J_MESSAGE_CODE = J_MESSAGE_CODE(5);
    pub const JERR_BAD_DCT_COEF: J_MESSAGE_CODE = J_MESSAGE_CODE(6);
    pub const JERR_BAD_DCTSIZE: J_MESSAGE_CODE = J_MESSAGE_CODE(7);
    pub const JERR_BAD_HUFF_TABLE: J_MESSAGE_CODE = J_MESSAGE_CODE(8);
    pub const JERR_BAD_IN_COLORSPACE: J_MESSAGE_CODE = J_MESSAGE_CODE(9);
    pub const JERR_BAD_J_COLORSPACE: J_MESSAGE_CODE = J_MESSAGE_CODE(10);
    pub const JERR_BAD_LENGTH: J_MESSAGE_CODE = J_MESSAGE_CODE(11);
    pub const JERR_BAD_LIB_VERSION: J_MESSAGE_CODE = J_MESSAGE_CODE(12);
    pub const JERR_BAD_MCU_SIZE: J_MESSAGE_CODE = J_MESSAGE_CODE(13);
    pub const JERR_BAD_POOL_ID: J_MESSAGE_CODE = J_MESSAGE_CODE(14);
    pub const JERR_BAD_PRECISION: J_MESSAGE_CODE = J_MESSAGE_CODE(15);
    pub const JERR_BAD_PROGRESSION: J_MESSAGE_CODE = J_MESSAGE_CODE(16);
    pub const JERR_BAD_PROG_SCRIPT: J_MESSAGE_CODE = J_MESSAGE_CODE(17);
    pub const JERR_BAD_SAMPLING: J_MESSAGE_CODE = J_MESSAGE_CODE(18);
    pub const JERR_BAD_SCAN_SCRIPT: J_MESSAGE_CODE = J_MESSAGE_CODE(19);
    pub const JERR_BAD_STATE: J_MESSAGE_CODE = J_MESSAGE_CODE(20);
    pub const JERR_BAD_STRUCT_SIZE: J_MESSAGE_CODE = J_MESSAGE_CODE(21);
    pub const JERR_BAD_VIRTUAL_ACCESS: J_MESSAGE_CODE = J_MESSAGE_CODE(22);
    pub const JERR_BUFFER_SIZE: J_MESSAGE_CODE = J_MESSAGE_CODE(23);
    pub const JERR_CANT_SUSPEND: J_MESSAGE_CODE = J_MESSAGE_CODE(24);
    pub const JERR_CCIR601_NOTIMPL: J_MESSAGE_CODE = J_MESSAGE_CODE(25);
    pub const JERR_COMPONENT_COUNT: J_MESSAGE_CODE = J_MESSAGE_CODE(26);
    pub const JERR_CONVERSION_NOTIMPL: J_MESSAGE_CODE = J_MESSAGE_CODE(27);
    pub const JERR_DAC_INDEX: J_MESSAGE_CODE = J_MESSAGE_CODE(28);
    pub const JERR_DAC_VALUE: J_MESSAGE_CODE = J_MESSAGE_CODE(29);
    pub const JERR_DHT_INDEX: J_MESSAGE_CODE = J_MESSAGE_CODE(30);
    pub const JERR_DQT_INDEX: J_MESSAGE_CODE = J_MESSAGE_CODE(31);
    pub const JERR_EMPTY_IMAGE: J_MESSAGE_CODE = J_MESSAGE_CODE(32);
    pub const JERR_EMS_READ: J_MESSAGE_CODE = J_MESSAGE_CODE(33);
    pub const JERR_EMS_WRITE: J_MESSAGE_CODE = J_MESSAGE_CODE(34);
    pub const JERR_EOI_EXPECTED: J_MESSAGE_CODE = J_MESSAGE_CODE(35);
    pub const JERR_FILE_READ: J_MESSAGE_CODE = J_MESSAGE_CODE(36);
    pub const JERR_FILE_WRITE: J_MESSAGE_CODE = J_MESSAGE_CODE(37);
    pub const JERR_FRACT_SAMPLE_NOTIMPL: J_MESSAGE_CODE = J_MESSAGE_CODE(38);
    pub const JERR_HUFF_CLEN_OVERFLOW: J_MESSAGE_CODE = J_MESSAGE_CODE(39);
    pub const JERR_HUFF_MISSING_CODE: J_MESSAGE_CODE = J_MESSAGE_CODE(40);
    pub const JERR_IMAGE_TOO_BIG: J_MESSAGE_CODE = J_MESSAGE_CODE(41);
    pub const JERR_INPUT_EMPTY: J_MESSAGE_CODE = J_MESSAGE_CODE(42);
    pub const JERR_INPUT_EOF: J_MESSAGE_CODE = J_MESSAGE_CODE(43);
    pub const JERR_MISMATCHED_QUANT_TABLE: J_MESSAGE_CODE = J_MESSAGE_CODE(44);
    pub const JERR_MISSING_DATA: J_MESSAGE_CODE = J_MESSAGE_CODE(45);
    pub const JERR_MODE_CHANGE: J_MESSAGE_CODE = J_MESSAGE_CODE(46);
    pub const JERR_NOTIMPL: J_MESSAGE_CODE = J_MESSAGE_CODE(47);
    pub const JERR_NOT_COMPILED: J_MESSAGE_CODE = J_MESSAGE_CODE(48);
    pub const JERR_NO_BACKING_STORE: J_MESSAGE_CODE = J_MESSAGE_CODE(49);
    pub const JERR_NO_HUFF_TABLE: J_MESSAGE_CODE = J_MESSAGE_CODE(50);
    pub const JERR_NO_IMAGE: J_MESSAGE_CODE = J_MESSAGE_CODE(51);
    pub const JERR_NO_QUANT_TABLE: J_MESSAGE_CODE = J_MESSAGE_CODE(52);
    pub const JERR_NO_SOI: J_MESSAGE_CODE = J_MESSAGE_CODE(53);
    pub const JERR_OUT_OF_MEMORY: J_MESSAGE_CODE = J_MESSAGE_CODE(54);
    pub const JERR_QUANT_COMPONENTS: J_MESSAGE_CODE = J_MESSAGE_CODE(55);
    pub const JERR_QUANT_FEW_COLORS: J_MESSAGE_CODE = J_MESSAGE_CODE(56);
    pub const JERR_QUANT_MANY_COLORS: J_MESSAGE_CODE = J_MESSAGE_CODE(57);
    pub const JERR_SOF_DUPLICATE: J_MESSAGE_CODE = J_MESSAGE_CODE(58);
    pub const JERR_SOF_NO_SOS: J_MESSAGE_CODE = J_MESSAGE_CODE(59);
    pub const JERR_SOF_UNSUPPORTED: J_MESSAGE_CODE = J_MESSAGE_CODE(60);
    pub const JERR_SOI_DUPLICATE: J_MESSAGE_CODE = J_MESSAGE_CODE(61);
    pub const JERR_SOS_NO_SOF: J_MESSAGE_CODE = J_MESSAGE_CODE(62);
    pub const JERR_TFILE_CREATE: J_MESSAGE_CODE = J_MESSAGE_CODE(63);
    pub const JERR_TFILE_READ: J_MESSAGE_CODE = J_MESSAGE_CODE(64);
    pub const JERR_TFILE_SEEK: J_MESSAGE_CODE = J_MESSAGE_CODE(65);
    pub const JERR_TFILE_WRITE: J_MESSAGE_CODE = J_MESSAGE_CODE(66);
    pub const JERR_TOO_LITTLE_DATA: J_MESSAGE_CODE = J_MESSAGE_CODE(67);
    pub const JERR_UNKNOWN_MARKER: J_MESSAGE_CODE = J_MESSAGE_CODE(68);
    pub const JERR_VIRTUAL_BUG: J_MESSAGE_CODE = J_MESSAGE_CODE(69);
    pub const JERR_WIDTH_OVERFLOW: J_MESSAGE_CODE = J_MESSAGE_CODE(70);
    pub const JERR_XMS_READ: J_MESSAGE_CODE = J_MESSAGE_CODE(71);
    pub const JERR_XMS_WRITE: J_MESSAGE_CODE = J_MESSAGE_CODE(72);
    pub const JMSG_COPYRIGHT: J_MESSAGE_CODE = J_MESSAGE_CODE(73);
    pub const JMSG_VERSION: J_MESSAGE_CODE = J_MESSAGE_CODE(74);
    pub const JTRC_16BIT_TABLES: J_MESSAGE_CODE = J_MESSAGE_CODE(75);
    pub const JTRC_ADOBE: J_MESSAGE_CODE = J_MESSAGE_CODE(76);
    pub const JTRC_APP0: J_MESSAGE_CODE = J_MESSAGE_CODE(77);
    pub const JTRC_APP14: J_MESSAGE_CODE = J_MESSAGE_CODE(78);
    pub const JTRC_DAC: J_MESSAGE_CODE = J_MESSAGE_CODE(79);
    pub const JTRC_DHT: J_MESSAGE_CODE = J_MESSAGE_CODE(80);
    pub const JTRC_DQT: J_MESSAGE_CODE = J_MESSAGE_CODE(81);
    pub const JTRC_DRI: J_MESSAGE_CODE = J_MESSAGE_CODE(82);
    pub const JTRC_EMS_CLOSE: J_MESSAGE_CODE = J_MESSAGE_CODE(83);
    pub const JTRC_EMS_OPEN: J_MESSAGE_CODE = J_MESSAGE_CODE(84);
    pub const JTRC_EOI: J_MESSAGE_CODE = J_MESSAGE_CODE(85);
    pub const JTRC_HUFFBITS: J_MESSAGE_CODE = J_MESSAGE_CODE(86);
    pub const JTRC_JFIF: J_MESSAGE_CODE = J_MESSAGE_CODE(87);
    pub const JTRC_JFIF_BADTHUMBNAILSIZE: J_MESSAGE_CODE = J_MESSAGE_CODE(88);
    pub const JTRC_JFIF_EXTENSION: J_MESSAGE_CODE = J_MESSAGE_CODE(89);
    pub const JTRC_JFIF_THUMBNAIL: J_MESSAGE_CODE = J_MESSAGE_CODE(90);
    pub const JTRC_MISC_MARKER: J_MESSAGE_CODE = J_MESSAGE_CODE(91);
    pub const JTRC_PARMLESS_MARKER: J_MESSAGE_CODE = J_MESSAGE_CODE(92);
    pub const JTRC_QUANTVALS: J_MESSAGE_CODE = J_MESSAGE_CODE(93);
    pub const JTRC_QUANT_3_NCOLORS: J_MESSAGE_CODE = J_MESSAGE_CODE(94);
    pub const JTRC_QUANT_NCOLORS: J_MESSAGE_CODE = J_MESSAGE_CODE(95);
    pub const JTRC_QUANT_SELECTED: J_MESSAGE_CODE = J_MESSAGE_CODE(96);
    pub const JTRC_RECOVERY_ACTION: J_MESSAGE_CODE = J_MESSAGE_CODE(97);
    pub const JTRC_RST: J_MESSAGE_CODE = J_MESSAGE_CODE(98);
    pub const JTRC_SMOOTH_NOTIMPL: J_MESSAGE_CODE = J_MESSAGE_CODE(99);
    pub const JTRC_SOF: J_MESSAGE_CODE = J_MESSAGE_CODE(100);
    pub const JTRC_SOF_COMPONENT: J_MESSAGE_CODE = J_MESSAGE_CODE(101);
    pub const JTRC_SOI: J_MESSAGE_CODE = J_MESSAGE_CODE(102);
    pub const JTRC_SOS: J_MESSAGE_CODE = J_MESSAGE_CODE(103);
    pub const JTRC_SOS_COMPONENT: J_MESSAGE_CODE = J_MESSAGE_CODE(104);
    pub const JTRC_SOS_PARAMS: J_MESSAGE_CODE = J_MESSAGE_CODE(105);
    pub const JTRC_TFILE_CLOSE: J_MESSAGE_CODE = J_MESSAGE_CODE(106);
    pub const JTRC_TFILE_OPEN: J_MESSAGE_CODE = J_MESSAGE_CODE(107);
    pub const JTRC_THUMB_JPEG: J_MESSAGE_CODE = J_MESSAGE_CODE(108);
    pub const JTRC_THUMB_PALETTE: J_MESSAGE_CODE = J_MESSAGE_CODE(109);
    pub const JTRC_THUMB_RGB: J_MESSAGE_CODE = J_MESSAGE_CODE(110);
    pub const JTRC_UNKNOWN_IDS: J_MESSAGE_CODE = J_MESSAGE_CODE(111);
    pub const JTRC_XMS_CLOSE: J_MESSAGE_CODE = J_MESSAGE_CODE(112);
    pub const JTRC_XMS_OPEN: J_MESSAGE_CODE = J_MESSAGE_CODE(113);
    pub const JWRN_ADOBE_XFORM: J_MESSAGE_CODE = J_MESSAGE_CODE(114);
    pub const JWRN_BOGUS_PROGRESSION: J_MESSAGE_CODE = J_MESSAGE_CODE(115);
    pub const JWRN_EXTRANEOUS_DATA: J_MESSAGE_CODE = J_MESSAGE_CODE(116);
    pub const JWRN_HIT_MARKER: J_MESSAGE_CODE = J_MESSAGE_CODE(117);
    pub const JWRN_HUFF_BAD_CODE: J_MESSAGE_CODE = J_MESSAGE_CODE(118);
    pub const JWRN_JFIF_MAJOR: J_MESSAGE_CODE = J_MESSAGE_CODE(119);
    pub const JWRN_JPEG_EOF: J_MESSAGE_CODE = J_MESSAGE_CODE(120);
    pub const JWRN_MUST_RESYNC: J_MESSAGE_CODE = J_MESSAGE_CODE(121);
    pub const JWRN_NOT_SEQUENTIAL: J_MESSAGE_CODE = J_MESSAGE_CODE(122);
    pub const JWRN_TOO_MUCH_DATA: J_MESSAGE_CODE = J_MESSAGE_CODE(123);
    pub const JERR_BAD_CROP_SPEC: J_MESSAGE_CODE = J_MESSAGE_CODE(124);
    pub const JERR_NO_ARITH_TABLE: J_MESSAGE_CODE = J_MESSAGE_CODE(125);
    pub const JWRN_ARITH_BAD_CODE: J_MESSAGE_CODE = J_MESSAGE_CODE(126);
    pub const JWRN_BOGUS_ICC: J_MESSAGE_CODE = J_MESSAGE_CODE(127);
    pub const JERR_BAD_DROP_SAMPLING: J_MESSAGE_CODE = J_MESSAGE_CODE(128);
    pub const JERR_BAD_RESTART: J_MESSAGE_CODE = J_MESSAGE_CODE(129);
    /// One more than the last message code.
    pub const JMSG_LASTMSGCODE: J_MESSAGE_CODE = J_MESSAGE_CODE(130);
}

/// The error a function returns after `ERREXIT` has run: the error itself is
/// in `cinfo.err.msg_code` and `cinfo.err.msg_parm`, where C keeps it (ADR
/// 0005, "Errors return"). Not an upstream item.
#[must_use]
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct ErrorExit;

// upstream: src/jerror.c jpeg_std_message_table
/// The message string table, built from the master message list in
/// `jerror.h`, with a final null entry.
#[expect(non_upper_case_globals, reason = "C table name")]
pub static jpeg_std_message_table: [Option<&str>; J_MESSAGE_CODE::JMSG_LASTMSGCODE.0 as usize + 1] = [
    // JMSG_NOMESSAGE
    Some("Bogus message code %d"),
    // JERR_ARITH_NOTIMPL
    Some("Sorry, arithmetic coding is not implemented"),
    // JERR_BAD_ALIGN_TYPE
    Some("ALIGN_TYPE is wrong, please fix"),
    // JERR_BAD_ALLOC_CHUNK
    Some("MAX_ALLOC_CHUNK is wrong, please fix"),
    // JERR_BAD_BUFFER_MODE
    Some("Bogus buffer control mode"),
    // JERR_BAD_COMPONENT_ID
    Some("Invalid component ID %d in SOS"),
    // JERR_BAD_DCT_COEF
    Some("DCT coefficient (lossy) or spatial difference (lossless) out of range"),
    // JERR_BAD_DCTSIZE
    Some("IDCT output block size %d not supported"),
    // JERR_BAD_HUFF_TABLE
    Some("Bogus Huffman table definition"),
    // JERR_BAD_IN_COLORSPACE
    Some("Bogus input colorspace"),
    // JERR_BAD_J_COLORSPACE
    Some("Bogus JPEG colorspace"),
    // JERR_BAD_LENGTH
    Some("Bogus marker length"),
    // JERR_BAD_LIB_VERSION
    Some("Wrong JPEG library version: library is %d, caller expects %d"),
    // JERR_BAD_MCU_SIZE
    Some("Sampling factors too large for interleaved scan"),
    // JERR_BAD_POOL_ID
    Some("Invalid memory pool code %d"),
    // JERR_BAD_PRECISION
    Some("Unsupported JPEG data precision %d"),
    // JERR_BAD_PROGRESSION
    Some("Invalid progressive/lossless parameters Ss=%d Se=%d Ah=%d Al=%d"),
    // JERR_BAD_PROG_SCRIPT
    Some("Invalid progressive/lossless parameters at scan script entry %d"),
    // JERR_BAD_SAMPLING
    Some("Bogus sampling factors"),
    // JERR_BAD_SCAN_SCRIPT
    Some("Invalid scan script at entry %d"),
    // JERR_BAD_STATE
    Some("Improper call to JPEG library in state %d"),
    // JERR_BAD_STRUCT_SIZE
    Some("JPEG parameter struct mismatch: library thinks size is %u, caller expects %u"),
    // JERR_BAD_VIRTUAL_ACCESS
    Some("Bogus virtual array access"),
    // JERR_BUFFER_SIZE
    Some("Buffer passed to JPEG library is too small"),
    // JERR_CANT_SUSPEND
    Some("Suspension not allowed here"),
    // JERR_CCIR601_NOTIMPL
    Some("CCIR601 sampling not implemented yet"),
    // JERR_COMPONENT_COUNT
    Some("Too many color components: %d, max %d"),
    // JERR_CONVERSION_NOTIMPL
    Some("Unsupported color conversion request"),
    // JERR_DAC_INDEX
    Some("Bogus DAC index %d"),
    // JERR_DAC_VALUE
    Some("Bogus DAC value 0x%x"),
    // JERR_DHT_INDEX
    Some("Bogus DHT index %d"),
    // JERR_DQT_INDEX
    Some("Bogus DQT index %d"),
    // JERR_EMPTY_IMAGE
    Some("Empty JPEG image (DNL not supported)"),
    // JERR_EMS_READ
    Some("Read from EMS failed"),
    // JERR_EMS_WRITE
    Some("Write to EMS failed"),
    // JERR_EOI_EXPECTED
    Some("Didn't expect more than one scan"),
    // JERR_FILE_READ
    Some("Input file read error"),
    // JERR_FILE_WRITE
    Some("Output file write error --- out of disk space?"),
    // JERR_FRACT_SAMPLE_NOTIMPL
    Some("Fractional sampling not implemented yet"),
    // JERR_HUFF_CLEN_OVERFLOW
    Some("Huffman code size table overflow"),
    // JERR_HUFF_MISSING_CODE
    Some("Missing Huffman code table entry"),
    // JERR_IMAGE_TOO_BIG
    Some("Maximum supported image dimension is %u pixels"),
    // JERR_INPUT_EMPTY
    Some("Empty input file"),
    // JERR_INPUT_EOF
    Some("Premature end of input file"),
    // JERR_MISMATCHED_QUANT_TABLE
    Some("Cannot transcode due to multiple use of quantization table %d"),
    // JERR_MISSING_DATA
    Some("Scan script does not transmit all data"),
    // JERR_MODE_CHANGE
    Some("Invalid color quantization mode change"),
    // JERR_NOTIMPL
    Some("Requested features are incompatible"),
    // JERR_NOT_COMPILED
    Some("Requested feature was omitted at compile time"),
    // JERR_NO_BACKING_STORE
    Some("Memory limit exceeded"),
    // JERR_NO_HUFF_TABLE
    Some("Huffman table 0x%02x was not defined"),
    // JERR_NO_IMAGE
    Some("JPEG datastream contains no image"),
    // JERR_NO_QUANT_TABLE
    Some("Quantization table 0x%02x was not defined"),
    // JERR_NO_SOI
    Some("Not a JPEG file: starts with 0x%02x 0x%02x"),
    // JERR_OUT_OF_MEMORY
    Some("Insufficient memory (case %d)"),
    // JERR_QUANT_COMPONENTS
    Some("Cannot quantize more than %d color components"),
    // JERR_QUANT_FEW_COLORS
    Some("Cannot quantize to fewer than %d colors"),
    // JERR_QUANT_MANY_COLORS
    Some("Cannot quantize to more than %d colors"),
    // JERR_SOF_DUPLICATE
    Some("Invalid JPEG file structure: two SOF markers"),
    // JERR_SOF_NO_SOS
    Some("Invalid JPEG file structure: missing SOS marker"),
    // JERR_SOF_UNSUPPORTED
    Some("Unsupported JPEG process: SOF type 0x%02x"),
    // JERR_SOI_DUPLICATE
    Some("Invalid JPEG file structure: two SOI markers"),
    // JERR_SOS_NO_SOF
    Some("Invalid JPEG file structure: SOS before SOF"),
    // JERR_TFILE_CREATE
    Some("Failed to create temporary file %s"),
    // JERR_TFILE_READ
    Some("Read failed on temporary file"),
    // JERR_TFILE_SEEK
    Some("Seek failed on temporary file"),
    // JERR_TFILE_WRITE
    Some("Write failed on temporary file --- out of disk space?"),
    // JERR_TOO_LITTLE_DATA
    Some("Application transferred too few scanlines"),
    // JERR_UNKNOWN_MARKER
    Some("Unsupported marker type 0x%02x"),
    // JERR_VIRTUAL_BUG
    Some("Virtual array controller messed up"),
    // JERR_WIDTH_OVERFLOW
    Some("Image too wide for this implementation"),
    // JERR_XMS_READ
    Some("Read from XMS failed"),
    // JERR_XMS_WRITE
    Some("Write to XMS failed"),
    // JMSG_COPYRIGHT
    Some(JCOPYRIGHT),
    // JMSG_VERSION
    Some(JVERSION),
    // JTRC_16BIT_TABLES
    Some("Caution: quantization tables are too coarse for baseline JPEG"),
    // JTRC_ADOBE
    Some("Adobe APP14 marker: version %d, flags 0x%04x 0x%04x, transform %d"),
    // JTRC_APP0
    Some("Unknown APP0 marker (not JFIF), length %u"),
    // JTRC_APP14
    Some("Unknown APP14 marker (not Adobe), length %u"),
    // JTRC_DAC
    Some("Define Arithmetic Table 0x%02x: 0x%02x"),
    // JTRC_DHT
    Some("Define Huffman Table 0x%02x"),
    // JTRC_DQT
    Some("Define Quantization Table %d  precision %d"),
    // JTRC_DRI
    Some("Define Restart Interval %u"),
    // JTRC_EMS_CLOSE
    Some("Freed EMS handle %u"),
    // JTRC_EMS_OPEN
    Some("Obtained EMS handle %u"),
    // JTRC_EOI
    Some("End Of Image"),
    // JTRC_HUFFBITS
    Some("        %3d %3d %3d %3d %3d %3d %3d %3d"),
    // JTRC_JFIF
    Some("JFIF APP0 marker: version %d.%02d, density %dx%d  %d"),
    // JTRC_JFIF_BADTHUMBNAILSIZE
    Some("Warning: thumbnail image size does not match data length %u"),
    // JTRC_JFIF_EXTENSION
    Some("JFIF extension marker: type 0x%02x, length %u"),
    // JTRC_JFIF_THUMBNAIL
    Some("    with %d x %d thumbnail image"),
    // JTRC_MISC_MARKER
    Some("Miscellaneous marker 0x%02x, length %u"),
    // JTRC_PARMLESS_MARKER
    Some("Unexpected marker 0x%02x"),
    // JTRC_QUANTVALS
    Some("        %4u %4u %4u %4u %4u %4u %4u %4u"),
    // JTRC_QUANT_3_NCOLORS
    Some("Quantizing to %d = %d*%d*%d colors"),
    // JTRC_QUANT_NCOLORS
    Some("Quantizing to %d colors"),
    // JTRC_QUANT_SELECTED
    Some("Selected %d colors for quantization"),
    // JTRC_RECOVERY_ACTION
    Some("At marker 0x%02x, recovery action %d"),
    // JTRC_RST
    Some("RST%d"),
    // JTRC_SMOOTH_NOTIMPL
    Some("Smoothing not supported with nonstandard sampling ratios"),
    // JTRC_SOF
    Some("Start Of Frame 0x%02x: width=%u, height=%u, components=%d"),
    // JTRC_SOF_COMPONENT
    Some("    Component %d: %dhx%dv q=%d"),
    // JTRC_SOI
    Some("Start of Image"),
    // JTRC_SOS
    Some("Start Of Scan: %d components"),
    // JTRC_SOS_COMPONENT
    Some("    Component %d: dc=%d ac=%d"),
    // JTRC_SOS_PARAMS
    Some("  Ss=%d, Se=%d, Ah=%d, Al=%d"),
    // JTRC_TFILE_CLOSE
    Some("Closed temporary file %s"),
    // JTRC_TFILE_OPEN
    Some("Opened temporary file %s"),
    // JTRC_THUMB_JPEG
    Some("JFIF extension marker: JPEG-compressed thumbnail image, length %u"),
    // JTRC_THUMB_PALETTE
    Some("JFIF extension marker: palette thumbnail image, length %u"),
    // JTRC_THUMB_RGB
    Some("JFIF extension marker: RGB thumbnail image, length %u"),
    // JTRC_UNKNOWN_IDS
    Some("Unrecognized component IDs %d %d %d, assuming YCbCr (lossy) or RGB (lossless)"),
    // JTRC_XMS_CLOSE
    Some("Freed XMS handle %u"),
    // JTRC_XMS_OPEN
    Some("Obtained XMS handle %u"),
    // JWRN_ADOBE_XFORM
    Some("Unknown Adobe color transform code %d"),
    // JWRN_BOGUS_PROGRESSION
    Some("Inconsistent progression sequence for component %d coefficient %d"),
    // JWRN_EXTRANEOUS_DATA
    Some("Corrupt JPEG data: %u extraneous bytes before marker 0x%02x"),
    // JWRN_HIT_MARKER
    Some("Corrupt JPEG data: premature end of data segment"),
    // JWRN_HUFF_BAD_CODE
    Some("Corrupt JPEG data: bad Huffman code"),
    // JWRN_JFIF_MAJOR
    Some("Warning: unknown JFIF revision number %d.%02d"),
    // JWRN_JPEG_EOF
    Some("Premature end of JPEG file"),
    // JWRN_MUST_RESYNC
    Some("Corrupt JPEG data: found marker 0x%02x instead of RST%d"),
    // JWRN_NOT_SEQUENTIAL
    Some("Invalid SOS parameters for sequential JPEG"),
    // JWRN_TOO_MUCH_DATA
    Some("Application transferred too many scanlines"),
    // JERR_BAD_CROP_SPEC
    Some("Invalid crop request"),
    // JERR_NO_ARITH_TABLE
    Some("Arithmetic table 0x%02x was not defined"),
    // JWRN_ARITH_BAD_CODE
    Some("Corrupt JPEG data: bad arithmetic code"),
    // JWRN_BOGUS_ICC
    Some("Corrupt JPEG data: bad ICC marker"),
    // JERR_BAD_DROP_SAMPLING
    Some("Component index %d: mismatching sampling ratio %d:%d, %d:%d, %c"),
    // JERR_BAD_RESTART
    Some(
        "Invalid restart interval %d; must be an integer multiple of the number of MCUs in an MCU row (%d)",
    ),
    None,
];

/// The methods of an error manager (ADR 0005, "Public hooks"). Each default
/// is upstream's function. While a method runs, the trait object is taken
/// out of `cinfo.err` and the error manager's fields stay in it, so a method
/// reaches other methods through `self` and the fields through `cinfo.err`,
/// as C reaches both through `cinfo->err`.
#[expect(non_camel_case_types, reason = "named after the C struct")]
pub trait jpeg_error_methods {
    /// Error exit handler. Upstream's never returns; here it returns, and the
    /// caller returns `Err(ErrorExit)` (ADR 0005).
    fn error_exit(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        error_exit(self, cinfo);
    }

    /// Conditionally emits a trace or warning message.
    ///
    /// # Errors
    ///
    /// An application may stop decoding here, where a C application could
    /// `longjmp`; the default never fails.
    fn emit_message(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        msg_level: i32,
    ) -> Result<(), ErrorExit> {
        emit_message(self, cinfo, msg_level)
    }

    /// Actually outputs a trace or error message.
    fn output_message(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        output_message(self, cinfo);
    }

    /// Formats a message string for the most recent JPEG error or message.
    fn format_message(
        &mut self,
        cinfo: &mut jpeg_decompress_struct<'_>,
        buffer: &mut [u8; JMSG_LENGTH_MAX],
    ) {
        format_message(cinfo, buffer);
    }

    /// Resets error state variables at the start of a new image.
    fn reset_error_mgr(&mut self, cinfo: &mut jpeg_decompress_struct<'_>) {
        reset_error_mgr(cinfo);
    }
}

// upstream: src/jerror.c error_exit
/// Displays the message. Upstream then calls `jpeg_destroy` and
/// `exit(EXIT_FAILURE)`; a library may not exit the process, so the default
/// returns and leaves the object to its owner (ADR 0005, "The default error
/// manager").
pub fn error_exit<M: jpeg_error_methods + ?Sized>(
    methods: &mut M,
    cinfo: &mut jpeg_decompress_struct<'_>,
) {
    /* Always display the message */
    methods.output_message(cinfo);
}

// upstream: src/jerror.c output_message
/// Formats the message and sends it to standard error, adding a newline.
pub fn output_message<M: jpeg_error_methods + ?Sized>(
    methods: &mut M,
    cinfo: &mut jpeg_decompress_struct<'_>,
) {
    let mut buffer = [0; JMSG_LENGTH_MAX];

    /* Create the message */
    methods.format_message(cinfo, &mut buffer);

    /* Send it to stderr, adding a newline */
    let length = buffer
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(JMSG_LENGTH_MAX);
    let mut line = Vec::with_capacity(length + 1);
    line.extend_from_slice(&buffer[..length]);
    line.push(b'\n');
    // fprintf's result is ignored upstream too.
    let _ = std::io::stderr().lock().write_all(&line);
}

// upstream: src/jerror.c emit_message
/// Decides whether to emit a trace or warning message. `msg_level` is -1 for
/// a recoverable corrupt-data warning, 0 for an important advisory message
/// and 1 and up for successively more detailed tracing.
///
/// # Errors
///
/// Never; the `Result` lets an override stop decoding (ADR 0005).
pub fn emit_message<M: jpeg_error_methods + ?Sized>(
    methods: &mut M,
    cinfo: &mut jpeg_decompress_struct<'_>,
    msg_level: i32,
) -> Result<(), ErrorExit> {
    if msg_level < 0 {
        /* It's a warning message.  Since corrupt files may generate many warnings,
         * the policy implemented here is to show only the first warning,
         * unless trace_level >= 3.
         */
        if cinfo.err.num_warnings == 0 || cinfo.err.trace_level >= 3 {
            methods.output_message(cinfo);
        }
        /* Always count warnings in num_warnings. */
        cinfo.err.num_warnings += 1;
    } else {
        /* It's a trace message.  Show it if trace_level >= msg_level. */
        if cinfo.err.trace_level >= msg_level {
            methods.output_message(cinfo);
        }
    }
    Ok(())
}

// upstream: src/jerror.c format_message
/// Formats the message for `cinfo.err.msg_code` into `buffer`, without a
/// newline and NUL-terminated, truncated to `JMSG_LENGTH_MAX - 1` bytes as
/// `snprintf` truncates.
///
/// # Panics
///
/// When the bogus-code fallback finds no entry 0 in `jpeg_message_table`,
/// where C would dereference a null pointer.
#[expect(
    clippy::cast_sign_loss,
    reason = "the C indexes with an int that the range checks keep nonnegative"
)]
pub fn format_message(cinfo: &mut jpeg_decompress_struct<'_>, buffer: &mut [u8; JMSG_LENGTH_MAX]) {
    let err = &mut *cinfo.err;
    let msg_code = err.msg_code;
    let mut msgtext: Option<&str> = None;

    /* Look up message string in proper table */
    if msg_code > 0 && msg_code <= err.last_jpeg_message {
        msgtext = err
            .jpeg_message_table
            .get(msg_code as usize)
            .copied()
            .flatten();
    } else if let Some(addon_message_table) = err.addon_message_table
        && msg_code >= err.first_addon_message
        && msg_code <= err.last_addon_message
    {
        msgtext = addon_message_table
            .get((msg_code - err.first_addon_message) as usize)
            .copied()
            .flatten();
    }

    /* Defend against bogus message number */
    let msgtext = if let Some(msgtext) = msgtext {
        msgtext
    } else {
        err.msg_parm.set_i(0, msg_code);
        err.jpeg_message_table[0].expect("jpeg_message_table[0] is NULL")
    };

    /* Check for string parameter, as indicated by %s in the message text */
    let mut isstring = false;
    let bytes = msgtext.as_bytes();
    if let Some(percent) = bytes.iter().position(|&ch| ch == b'%')
        && bytes.get(percent + 1) == Some(&b's')
    {
        isstring = true;
    }

    /* Format the message into the passed buffer */
    if isstring {
        let s = err.msg_parm.s();
        let length = s
            .iter()
            .position(|&byte| byte == 0)
            .unwrap_or(JMSG_STR_PARM_MAX);
        SNPRINTF(buffer, msgtext, PrintfArgs::Str(&s[..length]));
    } else {
        let parm = &err.msg_parm;
        SNPRINTF(
            buffer,
            msgtext,
            PrintfArgs::Ints([
                parm.i(0),
                parm.i(1),
                parm.i(2),
                parm.i(3),
                parm.i(4),
                parm.i(5),
                parm.i(6),
                parm.i(7),
            ]),
        );
    }
}

// upstream: src/jerror.c reset_error_mgr
/// Resets the error state variables at the start of a new image, without
/// losing any application-specific methods.
pub fn reset_error_mgr(cinfo: &mut jpeg_decompress_struct<'_>) {
    cinfo.err.num_warnings = 0;
    /* trace_level is not reset since it is an application-supplied parameter */
    cinfo.err.msg_code = 0; /* may be useful as a flag for "no error" */
}

/// The standard methods that `jpeg_std_error` installs: every default of
/// [`jpeg_error_methods`]. Not an upstream item.
#[derive(Clone, Copy, Debug, Default)]
pub struct StdErrorMethods;

impl jpeg_error_methods for StdErrorMethods {}

// upstream: src/jerror.c jpeg_std_error
/// Fills in the standard error-handling methods in an error manager, after
/// which the application may override some of them.
pub fn jpeg_std_error<'a, 'src>(err: &'a mut jpeg_error_mgr<'src>) -> &'a mut jpeg_error_mgr<'src> {
    *err = jpeg_error_mgr::default();

    err.methods = Some(Box::new(StdErrorMethods));

    /* Initialize message table pointers */
    err.jpeg_message_table = &jpeg_std_message_table;
    err.last_jpeg_message = J_MESSAGE_CODE::JMSG_LASTMSGCODE.0.cast_signed() - 1;

    err
}

/// `(*cinfo->err->error_exit) (cinfo)`: takes the methods out of
/// `cinfo.err`, calls the method and puts them back unless the method
/// installed others. Not an upstream item.
///
/// # Panics
///
/// When the error manager has no methods, where C would call a null pointer.
pub fn call_error_exit(cinfo: &mut jpeg_decompress_struct<'_>) {
    let mut methods = cinfo
        .err
        .methods
        .take()
        .expect("cinfo->err->error_exit is NULL");
    methods.error_exit(cinfo);
    if cinfo.err.methods.is_none() {
        cinfo.err.methods = Some(methods);
    }
}

/// `(*cinfo->err->emit_message) (cinfo, msg_level)`; see [`call_error_exit`].
///
/// # Errors
///
/// What the method returns.
///
/// # Panics
///
/// When the error manager has no methods.
pub fn call_emit_message(
    cinfo: &mut jpeg_decompress_struct<'_>,
    msg_level: i32,
) -> Result<(), ErrorExit> {
    let mut methods = cinfo
        .err
        .methods
        .take()
        .expect("cinfo->err->emit_message is NULL");
    let result = methods.emit_message(cinfo, msg_level);
    if cinfo.err.methods.is_none() {
        cinfo.err.methods = Some(methods);
    }
    result
}

/// `(*cinfo->err->output_message) (cinfo)`; see [`call_error_exit`].
///
/// # Panics
///
/// When the error manager has no methods.
pub fn call_output_message(cinfo: &mut jpeg_decompress_struct<'_>) {
    let mut methods = cinfo
        .err
        .methods
        .take()
        .expect("cinfo->err->output_message is NULL");
    methods.output_message(cinfo);
    if cinfo.err.methods.is_none() {
        cinfo.err.methods = Some(methods);
    }
}

/// `(*cinfo->err->format_message) (cinfo, buffer)`; see
/// [`call_error_exit`].
///
/// # Panics
///
/// When the error manager has no methods.
pub fn call_format_message(
    cinfo: &mut jpeg_decompress_struct<'_>,
    buffer: &mut [u8; JMSG_LENGTH_MAX],
) {
    let mut methods = cinfo
        .err
        .methods
        .take()
        .expect("cinfo->err->format_message is NULL");
    methods.format_message(cinfo, buffer);
    if cinfo.err.methods.is_none() {
        cinfo.err.methods = Some(methods);
    }
}

/// `(*cinfo->err->reset_error_mgr) (cinfo)`; see [`call_error_exit`].
///
/// # Panics
///
/// When the error manager has no methods.
pub fn call_reset_error_mgr(cinfo: &mut jpeg_decompress_struct<'_>) {
    let mut methods = cinfo
        .err
        .methods
        .take()
        .expect("cinfo->err->reset_error_mgr is NULL");
    methods.reset_error_mgr(cinfo);
    if cinfo.err.methods.is_none() {
        cinfo.err.methods = Some(methods);
    }
}

#[inline]
fn set_code(cinfo: &mut jpeg_decompress_struct<'_>, code: J_MESSAGE_CODE) {
    cinfo.err.msg_code = code.0.cast_signed();
}

#[inline]
fn set_parms(cinfo: &mut jpeg_decompress_struct<'_>, parms: &[i32]) {
    for (index, &parm) in parms.iter().enumerate() {
        cinfo.err.msg_parm.set_i(index, parm);
    }
}

/// C's `strncpy`: copies the C string `source` (up to its first NUL, or all
/// of it) into `dest`, at most `count` bytes, and fills the rest of the first
/// `count` bytes with NULs.
fn strncpy(dest: &mut [u8], source: &[u8], count: usize) {
    let length = source
        .iter()
        .position(|&byte| byte == 0)
        .unwrap_or(source.len());
    let copied = length.min(count);
    dest[..copied].copy_from_slice(&source[..copied]);
    dest[copied..count].fill(0);
}

// upstream: src/jerror.h ERREXIT
#[expect(non_snake_case, reason = "C macro name")]
pub fn ERREXIT(cinfo: j_common_ptr<'_, '_>, code: J_MESSAGE_CODE) -> ErrorExit {
    set_code(cinfo, code);
    call_error_exit(cinfo);
    ErrorExit
}

// upstream: src/jerror.h ERREXIT1
#[expect(non_snake_case, reason = "C macro name")]
pub fn ERREXIT1(cinfo: j_common_ptr<'_, '_>, code: J_MESSAGE_CODE, p1: i32) -> ErrorExit {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1]);
    call_error_exit(cinfo);
    ErrorExit
}

// upstream: src/jerror.h ERREXIT2
#[expect(non_snake_case, reason = "C macro name")]
pub fn ERREXIT2(cinfo: j_common_ptr<'_, '_>, code: J_MESSAGE_CODE, p1: i32, p2: i32) -> ErrorExit {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1, p2]);
    call_error_exit(cinfo);
    ErrorExit
}

// upstream: src/jerror.h ERREXIT3
#[expect(non_snake_case, reason = "C macro name")]
pub fn ERREXIT3(
    cinfo: j_common_ptr<'_, '_>,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
    p3: i32,
) -> ErrorExit {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1, p2, p3]);
    call_error_exit(cinfo);
    ErrorExit
}

// upstream: src/jerror.h ERREXIT4
#[expect(non_snake_case, reason = "C macro name")]
pub fn ERREXIT4(
    cinfo: j_common_ptr<'_, '_>,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: i32,
) -> ErrorExit {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1, p2, p3, p4]);
    call_error_exit(cinfo);
    ErrorExit
}

// upstream: src/jerror.h ERREXIT6
#[expect(non_snake_case, reason = "C macro name")]
#[expect(clippy::too_many_arguments, reason = "the C macro's parameters")]
pub fn ERREXIT6(
    cinfo: j_common_ptr<'_, '_>,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: i32,
    p5: i32,
    p6: i32,
) -> ErrorExit {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1, p2, p3, p4, p5, p6]);
    call_error_exit(cinfo);
    ErrorExit
}

// upstream: src/jerror.h ERREXITS
/// `str` is a C string: it ends at its first NUL or at the end of the slice.
#[expect(non_snake_case, reason = "C macro name")]
pub fn ERREXITS(cinfo: j_common_ptr<'_, '_>, code: J_MESSAGE_CODE, str: &[u8]) -> ErrorExit {
    set_code(cinfo, code);
    strncpy(cinfo.err.msg_parm.s_mut(), str, JMSG_STR_PARM_MAX - 1);
    cinfo.err.msg_parm.s_mut()[JMSG_STR_PARM_MAX - 1] = 0;
    call_error_exit(cinfo);
    ErrorExit
}

// upstream: src/jerror.h WARNMS
/// A nonfatal error: we can keep going, but the data is probably corrupt.
///
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn WARNMS(cinfo: j_common_ptr<'_, '_>, code: J_MESSAGE_CODE) -> Result<(), ErrorExit> {
    set_code(cinfo, code);
    call_emit_message(cinfo, -1)
}

// upstream: src/jerror.h WARNMS1
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn WARNMS1(
    cinfo: j_common_ptr<'_, '_>,
    code: J_MESSAGE_CODE,
    p1: i32,
) -> Result<(), ErrorExit> {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1]);
    call_emit_message(cinfo, -1)
}

// upstream: src/jerror.h WARNMS2
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn WARNMS2(
    cinfo: j_common_ptr<'_, '_>,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
) -> Result<(), ErrorExit> {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1, p2]);
    call_emit_message(cinfo, -1)
}

// upstream: src/jerror.h TRACEMS
/// An informational or debugging message.
///
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn TRACEMS(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
) -> Result<(), ErrorExit> {
    set_code(cinfo, code);
    call_emit_message(cinfo, lvl)
}

// upstream: src/jerror.h TRACEMS1
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn TRACEMS1(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
    p1: i32,
) -> Result<(), ErrorExit> {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1]);
    call_emit_message(cinfo, lvl)
}

// upstream: src/jerror.h TRACEMS2
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn TRACEMS2(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
) -> Result<(), ErrorExit> {
    set_code(cinfo, code);
    set_parms(cinfo, &[p1, p2]);
    call_emit_message(cinfo, lvl)
}

// upstream: src/jerror.h TRACEMS3
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn TRACEMS3(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
    p3: i32,
) -> Result<(), ErrorExit> {
    set_parms(cinfo, &[p1, p2, p3]);
    set_code(cinfo, code);
    call_emit_message(cinfo, lvl)
}

// upstream: src/jerror.h TRACEMS4
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn TRACEMS4(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: i32,
) -> Result<(), ErrorExit> {
    set_parms(cinfo, &[p1, p2, p3, p4]);
    set_code(cinfo, code);
    call_emit_message(cinfo, lvl)
}

// upstream: src/jerror.h TRACEMS5
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
#[expect(clippy::too_many_arguments, reason = "the C macro's parameters")]
pub fn TRACEMS5(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: i32,
    p5: i32,
) -> Result<(), ErrorExit> {
    set_parms(cinfo, &[p1, p2, p3, p4, p5]);
    set_code(cinfo, code);
    call_emit_message(cinfo, lvl)
}

// upstream: src/jerror.h TRACEMS8
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
#[expect(clippy::too_many_arguments, reason = "the C macro's parameters")]
pub fn TRACEMS8(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
    p1: i32,
    p2: i32,
    p3: i32,
    p4: i32,
    p5: i32,
    p6: i32,
    p7: i32,
    p8: i32,
) -> Result<(), ErrorExit> {
    set_parms(cinfo, &[p1, p2, p3, p4, p5, p6, p7, p8]);
    set_code(cinfo, code);
    call_emit_message(cinfo, lvl)
}

// upstream: src/jerror.h TRACEMSS
/// `str` is a C string: it ends at its first NUL or at the end of the slice.
///
/// # Errors
///
/// What `emit_message` returns.
#[expect(non_snake_case, reason = "C macro name")]
pub fn TRACEMSS(
    cinfo: j_common_ptr<'_, '_>,
    lvl: i32,
    code: J_MESSAGE_CODE,
    str: &[u8],
) -> Result<(), ErrorExit> {
    set_code(cinfo, code);
    strncpy(cinfo.err.msg_parm.s_mut(), str, JMSG_STR_PARM_MAX);
    cinfo.err.msg_parm.s_mut()[JMSG_STR_PARM_MAX - 1] = 0;
    call_emit_message(cinfo, lvl)
}

/// The variable arguments `format_message` passes to `snprintf`: the eight
/// `int` parameters, or the string parameter. Not an upstream item.
#[derive(Clone, Copy, Debug)]
pub enum PrintfArgs<'a> {
    /// `msg_parm.i[0]` to `msg_parm.i[7]`.
    Ints([i32; 8]),
    /// `msg_parm.s`, up to its NUL.
    Str(&'a [u8]),
}

// upstream: src/jinclude.h SNPRINTF
/// C's `snprintf` for the conversions that message strings use (ADR 0005,
/// "Messages"): `d`, `i`, `o`, `u`, `x`, `X`, `c`, `s` and `%%`, with the
/// flags `-`, `+`, space, `#` and `0`, a field width and a precision (each
/// may be `*`), and the length modifiers `hh` and `h`. Writes at most
/// `buffer.len() - 1` bytes and a NUL, as `snprintf` truncates.
///
/// A conversion that has no defined meaning with these arguments, which is
/// undefined behavior in C (a conversion of another type, a length modifier
/// other than `hh` or `h`, a conversion beyond the arguments passed, an
/// integer conversion in a string message or `%s` in an integer message), is
/// copied to the output as written.
#[expect(non_snake_case, reason = "C macro name")]
pub fn SNPRINTF(buffer: &mut [u8], format: &str, args: PrintfArgs<'_>) {
    let mut output = Vec::with_capacity(buffer.len());
    vformat(&mut output, format.as_bytes(), args);
    let Some(limit) = buffer.len().checked_sub(1) else {
        return;
    };
    let length = output.len().min(limit);
    buffer[..length].copy_from_slice(&output[..length]);
    buffer[length] = 0;
}

#[expect(clippy::struct_excessive_bools, reason = "the printf flags")]
#[derive(Default)]
struct Spec {
    left: bool,
    plus: bool,
    space: bool,
    alternate: bool,
    zero: bool,
    width: usize,
    precision: Option<usize>,
}

struct Args<'a> {
    list: PrintfArgs<'a>,
    next: usize,
    string_used: bool,
}

impl Args<'_> {
    fn int(&mut self) -> Option<i32> {
        match self.list {
            PrintfArgs::Ints(ints) => {
                let value = ints.get(self.next).copied();
                self.next += 1;
                value
            }
            PrintfArgs::Str(_) => None,
        }
    }

    fn string(&mut self) -> Option<&[u8]> {
        match self.list {
            PrintfArgs::Str(string) if !self.string_used => {
                self.string_used = true;
                Some(string)
            }
            _ => None,
        }
    }
}

fn digits(value: u64, base: u64, upper: bool) -> Vec<u8> {
    const LOWER: &[u8; 16] = b"0123456789abcdef";
    const UPPER: &[u8; 16] = b"0123456789ABCDEF";
    let table = if upper { UPPER } else { LOWER };
    let mut reversed = Vec::new();
    let mut rest = value;
    loop {
        #[expect(clippy::cast_possible_truncation, reason = "a digit below 16")]
        reversed.push(table[(rest % base) as usize]);
        rest /= base;
        if rest == 0 {
            break;
        }
    }
    reversed.reverse();
    reversed
}

fn pad(output: &mut Vec<u8>, spec: &Spec, body: &[u8]) {
    let fill = spec.width.saturating_sub(body.len());
    if spec.left {
        output.extend_from_slice(body);
        output.resize(output.len() + fill, b' ');
    } else {
        output.resize(output.len() + fill, b' ');
        output.extend_from_slice(body);
    }
}

/// One integer conversion: `prefix` (a sign or `0x`) comes before the zero
/// padding, `magnitude` is the value's digits, and `octal_alternate` forces a
/// leading zero digit as `%#o` does.
fn integer(
    output: &mut Vec<u8>,
    spec: &Spec,
    prefix: &[u8],
    magnitude: &[u8],
    is_zero: bool,
    octal_alternate: bool,
) {
    let mut body_digits = if is_zero && spec.precision == Some(0) {
        Vec::new()
    } else {
        magnitude.to_vec()
    };
    if let Some(precision) = spec.precision
        && body_digits.len() < precision
    {
        let mut padded = vec![b'0'; precision - body_digits.len()];
        padded.extend_from_slice(&body_digits);
        body_digits = padded;
    }
    if octal_alternate && body_digits.first() != Some(&b'0') {
        body_digits.insert(0, b'0');
    }
    let length = prefix.len() + body_digits.len();
    if !spec.left && spec.zero && spec.precision.is_none() && spec.width > length {
        output.extend_from_slice(prefix);
        output.resize(output.len() + spec.width - length, b'0');
        output.extend_from_slice(&body_digits);
    } else {
        let mut body = prefix.to_vec();
        body.extend_from_slice(&body_digits);
        pad(output, spec, &body);
    }
}

fn number(format: &[u8], index: &mut usize) -> usize {
    let mut value: usize = 0;
    while let Some(&digit) = format.get(*index)
        && digit.is_ascii_digit()
    {
        value = value
            .saturating_mul(10)
            .saturating_add(usize::from(digit - b'0'));
        *index += 1;
    }
    value
}

/// Parses the flags, field width, precision and length modifier of a
/// conversion specification after its `%`, consuming the `*` arguments.
/// Returns the specification, the number of `h` modifiers, and whether the
/// specification has a defined meaning with these arguments.
#[expect(
    clippy::cast_sign_loss,
    reason = "printf's conversion of a nonnegative int width or precision"
)]
fn parse_spec(format: &[u8], index: &mut usize, args: &mut Args<'_>) -> (Spec, usize, bool) {
    let mut spec = Spec::default();
    while let Some(&flag) = format.get(*index) {
        match flag {
            b'-' => spec.left = true,
            b'+' => spec.plus = true,
            b' ' => spec.space = true,
            b'#' => spec.alternate = true,
            b'0' => spec.zero = true,
            _ => break,
        }
        *index += 1;
    }
    let mut defined = true;
    if format.get(*index) == Some(&b'*') {
        *index += 1;
        match args.int() {
            Some(width) if width < 0 => {
                spec.left = true;
                spec.width = width.unsigned_abs() as usize;
            }
            Some(width) => spec.width = width as usize,
            None => defined = false,
        }
    } else {
        spec.width = number(format, index);
    }
    if format.get(*index) == Some(&b'.') {
        *index += 1;
        if format.get(*index) == Some(&b'*') {
            *index += 1;
            match args.int() {
                Some(precision) if precision < 0 => spec.precision = None,
                Some(precision) => spec.precision = Some(precision as usize),
                None => defined = false,
            }
        } else {
            spec.precision = Some(number(format, index));
        }
    }
    let mut length = 0;
    while let Some(&modifier) = format.get(*index) {
        match modifier {
            b'h' => length += 1,
            b'l' | b'j' | b'z' | b't' | b'L' => defined = false,
            _ => break,
        }
        *index += 1;
    }
    (spec, length, defined && length <= 2)
}

/// Writes one conversion; `false` when it has no defined meaning with these
/// arguments, in which case nothing is written.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "printf's conversions of the int argument"
)]
fn convert(
    output: &mut Vec<u8>,
    conversion: u8,
    spec: &Spec,
    length: usize,
    args: &mut Args<'_>,
) -> bool {
    match conversion {
        b'%' => output.push(b'%'),
        b'd' | b'i' => {
            let Some(value) = args.int() else {
                return false;
            };
            let value = match length {
                2 => i32::from(value as i8),
                1 => i32::from(value as i16),
                _ => value,
            };
            let sign: &[u8] = if value < 0 {
                b"-"
            } else if spec.plus {
                b"+"
            } else if spec.space {
                b" "
            } else {
                b""
            };
            let magnitude = digits(u64::from(value.unsigned_abs()), 10, false);
            integer(output, spec, sign, &magnitude, value == 0, false);
        }
        b'o' | b'u' | b'x' | b'X' => {
            let Some(value) = args.int() else {
                return false;
            };
            let value = match length {
                2 => u32::from(value as u8),
                1 => u32::from(value as u16),
                _ => value as u32,
            };
            let (base, upper) = match conversion {
                b'o' => (8, false),
                b'u' => (10, false),
                b'x' => (16, false),
                _ => (16, true),
            };
            let magnitude = digits(u64::from(value), base, upper);
            let prefix: &[u8] = match conversion {
                b'x' if spec.alternate && value != 0 => b"0x",
                b'X' if spec.alternate && value != 0 => b"0X",
                _ => b"",
            };
            let octal_alternate = spec.alternate && conversion == b'o';
            integer(
                output,
                spec,
                prefix,
                &magnitude,
                value == 0,
                octal_alternate,
            );
        }
        b'c' if length == 0 => {
            let Some(value) = args.int() else {
                return false;
            };
            pad(output, spec, &[value as u8]);
        }
        b's' if length == 0 => {
            let Some(string) = args.string() else {
                return false;
            };
            let shown = spec.precision.map_or(string.len(), |p| p.min(string.len()));
            let body = string[..shown].to_vec();
            pad(output, spec, &body);
        }
        _ => return false,
    }
    true
}

fn vformat(output: &mut Vec<u8>, format: &[u8], args: PrintfArgs<'_>) {
    let mut args = Args {
        list: args,
        next: 0,
        string_used: false,
    };
    let mut index = 0;
    while index < format.len() {
        let byte = format[index];
        if byte != b'%' {
            output.push(byte);
            index += 1;
            continue;
        }
        let start = index;
        index += 1;
        let (spec, length, defined) = parse_spec(format, &mut index, &mut args);
        let Some(&conversion) = format.get(index) else {
            output.extend_from_slice(&format[start..]);
            break;
        };
        index += 1;
        if !(defined && convert(output, conversion, &spec, length, &mut args)) {
            output.extend_from_slice(&format[start..index]);
        }
    }
}
