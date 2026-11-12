// libjpeg-turbo 3.2.1, src/jdmarker.c:
//
// /*
//  * jdmarker.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1998, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2012, 2015, 2022, 2024, 2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains routines to decode JPEG datastream markers.
//  * Most of the complexity arises from our desire to support input
//  * suspension: if not all of the data for a marker is available,
//  * we must exit back to the application.  On resumption, we reprocess
//  * the marker.
//  */
//! Translation of `src/jdmarker.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
//!
//! The input macros (`INPUT_VARS` to `INPUT_2BYTES`) are functions over a
//! copy of the source position, and suspend by returning `false` or `None`
//! where the macros take their action. A marker processor is
//! [`jpeg_marker_parser_method`]: a library routine or an application's
//! processor. The saved-marker list is a linked list of boxes, and
//! `master.marker_list_end` holds the position of its tail.

use core::ffi::c_long;

use crate::jcomapi::{jpeg_alloc_huff_table, jpeg_alloc_quant_table};
use crate::jerror::{
    ERREXIT, ERREXIT1, ERREXIT2, ErrorExit, J_MESSAGE_CODE, TRACEMS, TRACEMS1, TRACEMS2, TRACEMS3,
    TRACEMS4, TRACEMS5, TRACEMS8, WARNMS2,
};
use crate::jmemmgr::{alloc_large_vec, alloc_small_box, alloc_small_vec};
use crate::jmorecfg::{C_INT, C_POINTER, CLayout, JOCTET, UINT8, UINT16};
use crate::jpegint::{JLONG, jpeg_marker_reader};
use crate::jpeglib::{
    DCTSIZE2, J_COLOR_SPACE, JPEG_REACHED_EOI, JPEG_REACHED_SOS, JPEG_SUSPENDED, JPOOL_IMAGE,
    JPOOL_PERMANENT, MAX_COMPS_IN_SCAN, NUM_ARITH_TBLS, NUM_HUFF_TBLS, NUM_QUANT_TBLS,
    SIZEOF_jpeg_component_info, SIZEOF_jpeg_marker_struct, call_fill_input_buffer,
    call_resync_to_restart, call_skip_input_data, j_decompress_ptr, jpeg_component_info,
    jpeg_decompress_struct, jpeg_marker_parser_method, jpeg_marker_processor, jpeg_marker_struct,
    jpeg_saved_marker_ptr,
};
use crate::jutils::jpeg_natural_order;

// upstream: src/jdmarker.c JPEG_MARKER
/// JPEG marker codes.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct JPEG_MARKER(pub u32);

impl JPEG_MARKER {
    pub const M_SOF0: JPEG_MARKER = JPEG_MARKER(0xc0);
    pub const M_SOF1: JPEG_MARKER = JPEG_MARKER(0xc1);
    pub const M_SOF2: JPEG_MARKER = JPEG_MARKER(0xc2);
    pub const M_SOF3: JPEG_MARKER = JPEG_MARKER(0xc3);
    pub const M_SOF5: JPEG_MARKER = JPEG_MARKER(0xc5);
    pub const M_SOF6: JPEG_MARKER = JPEG_MARKER(0xc6);
    pub const M_SOF7: JPEG_MARKER = JPEG_MARKER(0xc7);
    pub const M_JPG: JPEG_MARKER = JPEG_MARKER(0xc8);
    pub const M_SOF9: JPEG_MARKER = JPEG_MARKER(0xc9);
    pub const M_SOF10: JPEG_MARKER = JPEG_MARKER(0xca);
    pub const M_SOF11: JPEG_MARKER = JPEG_MARKER(0xcb);
    pub const M_SOF13: JPEG_MARKER = JPEG_MARKER(0xcd);
    pub const M_SOF14: JPEG_MARKER = JPEG_MARKER(0xce);
    pub const M_SOF15: JPEG_MARKER = JPEG_MARKER(0xcf);
    pub const M_DHT: JPEG_MARKER = JPEG_MARKER(0xc4);
    pub const M_DAC: JPEG_MARKER = JPEG_MARKER(0xcc);
    pub const M_RST0: JPEG_MARKER = JPEG_MARKER(0xd0);
    pub const M_RST1: JPEG_MARKER = JPEG_MARKER(0xd1);
    pub const M_RST2: JPEG_MARKER = JPEG_MARKER(0xd2);
    pub const M_RST3: JPEG_MARKER = JPEG_MARKER(0xd3);
    pub const M_RST4: JPEG_MARKER = JPEG_MARKER(0xd4);
    pub const M_RST5: JPEG_MARKER = JPEG_MARKER(0xd5);
    pub const M_RST6: JPEG_MARKER = JPEG_MARKER(0xd6);
    pub const M_RST7: JPEG_MARKER = JPEG_MARKER(0xd7);
    pub const M_SOI: JPEG_MARKER = JPEG_MARKER(0xd8);
    pub const M_EOI: JPEG_MARKER = JPEG_MARKER(0xd9);
    pub const M_SOS: JPEG_MARKER = JPEG_MARKER(0xda);
    pub const M_DQT: JPEG_MARKER = JPEG_MARKER(0xdb);
    pub const M_DNL: JPEG_MARKER = JPEG_MARKER(0xdc);
    pub const M_DRI: JPEG_MARKER = JPEG_MARKER(0xdd);
    pub const M_DHP: JPEG_MARKER = JPEG_MARKER(0xde);
    pub const M_EXP: JPEG_MARKER = JPEG_MARKER(0xdf);
    pub const M_APP0: JPEG_MARKER = JPEG_MARKER(0xe0);
    pub const M_APP1: JPEG_MARKER = JPEG_MARKER(0xe1);
    pub const M_APP2: JPEG_MARKER = JPEG_MARKER(0xe2);
    pub const M_APP3: JPEG_MARKER = JPEG_MARKER(0xe3);
    pub const M_APP4: JPEG_MARKER = JPEG_MARKER(0xe4);
    pub const M_APP5: JPEG_MARKER = JPEG_MARKER(0xe5);
    pub const M_APP6: JPEG_MARKER = JPEG_MARKER(0xe6);
    pub const M_APP7: JPEG_MARKER = JPEG_MARKER(0xe7);
    pub const M_APP8: JPEG_MARKER = JPEG_MARKER(0xe8);
    pub const M_APP9: JPEG_MARKER = JPEG_MARKER(0xe9);
    pub const M_APP10: JPEG_MARKER = JPEG_MARKER(0xea);
    pub const M_APP11: JPEG_MARKER = JPEG_MARKER(0xeb);
    pub const M_APP12: JPEG_MARKER = JPEG_MARKER(0xec);
    pub const M_APP13: JPEG_MARKER = JPEG_MARKER(0xed);
    pub const M_APP14: JPEG_MARKER = JPEG_MARKER(0xee);
    pub const M_APP15: JPEG_MARKER = JPEG_MARKER(0xef);
    pub const M_JPG0: JPEG_MARKER = JPEG_MARKER(0xf0);
    pub const M_JPG13: JPEG_MARKER = JPEG_MARKER(0xfd);
    pub const M_COM: JPEG_MARKER = JPEG_MARKER(0xfe);
    pub const M_TEM: JPEG_MARKER = JPEG_MARKER(0x01);
    pub const M_ERROR: JPEG_MARKER = JPEG_MARKER(0x100);

    /// The marker code as the `int` that C compares it with.
    #[must_use]
    pub const fn code(self) -> i32 {
        self.0.cast_signed()
    }
}

// upstream: src/jdmarker.c my_marker_reader
/// The private state of the marker reader.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Debug)]
pub struct my_marker_reader<'src> {
    /// Public fields.
    pub pub_: jpeg_marker_reader,
    /// Application-overridable processing method for COM markers.
    pub process_COM: jpeg_marker_parser_method<'src>,
    /// Application-overridable processing methods for `APPn` markers.
    pub process_APPn: [jpeg_marker_parser_method<'src>; 16],
    /// Limit on COM marker data length to save.
    pub length_limit_COM: u32,
    /// Limit on `APPn` marker data length to save, per marker type.
    pub length_limit_APPn: [u32; 16],
    /// The marker being saved; `None` if not processing a marker. It is not
    /// linked into `marker_list` until it is all read.
    pub cur_marker: jpeg_saved_marker_ptr,
    /// Data bytes read so far in the marker.
    pub bytes_read: u32,
}

/// `sizeof(my_marker_reader)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_marker_reader: usize = CLayout::structure(&[
    // struct jpeg_marker_reader
    CLayout::structure(&[C_POINTER.array(3), C_INT, C_INT, C_INT, C_INT]),
    C_POINTER,
    C_POINTER.array(16),
    C_INT,
    C_INT.array(16),
    C_POINTER,
    C_INT,
])
.size;

/// The local copies of the source's `next_input_byte` and `bytes_in_buffer`
/// that the input macros work on. Not an upstream item.
#[derive(Clone, Copy, Debug)]
pub struct InputVars {
    /// The offset of the next byte in the source buffer.
    pub next_input_byte: usize,
    /// The number of bytes left in the source buffer.
    pub bytes_in_buffer: usize,
}

// upstream: src/jdmarker.c INPUT_VARS
/// Declares and initializes local copies of the input pointer and count.
#[expect(non_snake_case, reason = "C macro name")]
#[must_use]
pub fn INPUT_VARS(cinfo: &jpeg_decompress_struct<'_>) -> InputVars {
    let datasrc = cinfo.src();
    InputVars {
        next_input_byte: datasrc.next_input_byte,
        bytes_in_buffer: datasrc.bytes_in_buffer,
    }
}

// upstream: src/jdmarker.c INPUT_SYNC
/// Unloads the local copies; done only at a restart boundary.
#[expect(non_snake_case, reason = "C macro name")]
pub fn INPUT_SYNC(cinfo: &mut jpeg_decompress_struct<'_>, vars: &InputVars) {
    let datasrc = cinfo.src_mut();
    datasrc.next_input_byte = vars.next_input_byte;
    datasrc.bytes_in_buffer = vars.bytes_in_buffer;
}

// upstream: src/jdmarker.c INPUT_RELOAD
/// Reloads the local copies, after a successful fill.
#[expect(non_snake_case, reason = "C macro name")]
pub fn INPUT_RELOAD(cinfo: &jpeg_decompress_struct<'_>, vars: &mut InputVars) {
    *vars = INPUT_VARS(cinfo);
}

// upstream: src/jdmarker.c MAKE_BYTE_AVAIL
/// Makes a byte available; `false` where the macro takes its action
/// (suspension). The local copies are not synced before
/// `fill_input_buffer`.
///
/// # Errors
///
/// What `fill_input_buffer` reports.
#[expect(non_snake_case, reason = "C macro name")]
pub fn MAKE_BYTE_AVAIL(
    cinfo: j_decompress_ptr<'_, '_>,
    vars: &mut InputVars,
) -> Result<bool, ErrorExit> {
    if vars.bytes_in_buffer == 0 {
        if !call_fill_input_buffer(cinfo)? {
            return Ok(false);
        }
        INPUT_RELOAD(cinfo, vars);
    }
    Ok(true)
}

fn next_input_byte(cinfo: &jpeg_decompress_struct<'_>, vars: &mut InputVars) -> JOCTET {
    let byte = cinfo.src().buffer.bytes()[vars.next_input_byte];
    vars.next_input_byte += 1;
    byte
}

// upstream: src/jdmarker.c INPUT_BYTE
/// Reads a byte; `None` where the macro takes its action (suspension).
///
/// # Errors
///
/// What `fill_input_buffer` reports.
#[expect(non_snake_case, reason = "C macro name")]
pub fn INPUT_BYTE(
    cinfo: j_decompress_ptr<'_, '_>,
    vars: &mut InputVars,
) -> Result<Option<JOCTET>, ErrorExit> {
    if !MAKE_BYTE_AVAIL(cinfo, vars)? {
        return Ok(None);
    }
    vars.bytes_in_buffer -= 1;
    Ok(Some(next_input_byte(cinfo, vars)))
}

// upstream: src/jdmarker.c INPUT_2BYTES
/// Reads two bytes as an unsigned 16-bit integer into `value`; `false`
/// where the macro takes its action. As in C, a suspension after the first
/// byte leaves `value` holding that byte shifted left by 8.
///
/// # Errors
///
/// What `fill_input_buffer` reports.
#[expect(non_snake_case, reason = "C macro name")]
pub fn INPUT_2BYTES(
    cinfo: j_decompress_ptr<'_, '_>,
    vars: &mut InputVars,
    value: &mut u32,
) -> Result<bool, ErrorExit> {
    if !MAKE_BYTE_AVAIL(cinfo, vars)? {
        return Ok(false);
    }
    vars.bytes_in_buffer -= 1;
    *value = u32::from(next_input_byte(cinfo, vars)) << 8;
    if !MAKE_BYTE_AVAIL(cinfo, vars)? {
        return Ok(false);
    }
    vars.bytes_in_buffer -= 1;
    *value += u32::from(next_input_byte(cinfo, vars));
    Ok(true)
}

/// Reads a marker's 16-bit length word, as `INPUT_2BYTES` into a `JLONG`.
fn input_length(
    cinfo: j_decompress_ptr<'_, '_>,
    vars: &mut InputVars,
) -> Result<Option<JLONG>, ErrorExit> {
    let mut length = 0;
    if !INPUT_2BYTES(cinfo, vars, &mut length)? {
        return Ok(None);
    }
    Ok(Some(JLONG::from(length)))
}

// upstream: src/jdmarker.c get_soi
/// Processes an SOI marker.
fn get_soi(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    TRACEMS(cinfo, 1, J_MESSAGE_CODE::JTRC_SOI)?;

    if cinfo.marker().pub_.saw_SOI {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_SOI_DUPLICATE));
    }

    /* Reset all parameters that are defined to be reset by SOI */

    for i in 0..NUM_ARITH_TBLS {
        cinfo.arith_dc_L[i] = 0;
        cinfo.arith_dc_U[i] = 1;
        cinfo.arith_ac_K[i] = 5;
    }
    cinfo.restart_interval = 0;

    /* Set initial assumptions for colorspace etc */

    cinfo.jpeg_color_space = J_COLOR_SPACE::JCS_UNKNOWN;
    cinfo.CCIR601_sampling = false; /* Assume non-CCIR sampling??? */

    cinfo.saw_JFIF_marker = false;
    cinfo.JFIF_major_version = 1; /* set default JFIF APP0 values */
    cinfo.JFIF_minor_version = 1;
    cinfo.density_unit = 0;
    cinfo.X_density = 1;
    cinfo.Y_density = 1;
    cinfo.saw_Adobe_marker = false;
    cinfo.Adobe_transform = 0;

    cinfo.marker_mut().pub_.saw_SOI = true;

    Ok(true)
}

// upstream: src/jdmarker.c get_sof
/// Processes a `SOFn` marker. The fields it reads are stored as they arrive,
/// so a suspension leaves those already read, as in C.
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between JDIMENSION, int and size_t"
)]
fn get_sof(
    cinfo: j_decompress_ptr<'_, '_>,
    is_prog: bool,
    is_lossless: bool,
    is_arith: bool,
) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);

    if cinfo.marker().pub_.saw_SOF {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_SOF_DUPLICATE));
    }

    cinfo.progressive_mode = is_prog;
    cinfo.master_mut().pub_.lossless = is_lossless;
    cinfo.arith_code = is_arith;

    let Some(mut length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };

    let Some(precision) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    };
    cinfo.data_precision = i32::from(precision);
    cinfo.master_mut().pub_.jpeg_data_precision = cinfo.data_precision;
    let mut value = cinfo.image_height;
    let complete = INPUT_2BYTES(cinfo, &mut vars, &mut value)?;
    cinfo.image_height = value;
    if !complete {
        return Ok(false);
    }
    let mut value = cinfo.image_width;
    let complete = INPUT_2BYTES(cinfo, &mut vars, &mut value)?;
    cinfo.image_width = value;
    if !complete {
        return Ok(false);
    }
    let Some(num_components) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    };
    cinfo.num_components = i32::from(num_components);

    length -= 8;

    TRACEMS4(
        cinfo,
        1,
        J_MESSAGE_CODE::JTRC_SOF,
        cinfo.unread_marker,
        cinfo.image_width as i32,
        cinfo.image_height as i32,
        cinfo.num_components,
    )?;

    /* We don't support files in which the image height is initially specified */
    /* as 0 and is later redefined by DNL.  As long as we have to check that,  */
    /* might as well have a general sanity check. */
    if cinfo.image_height == 0 || cinfo.image_width == 0 || cinfo.num_components <= 0 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_EMPTY_IMAGE));
    }

    if length != JLONG::from(cinfo.num_components * 3) {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_LENGTH));
    }

    if cinfo.comp_info.is_none() {
        /* do only once, even if suspend */
        let count = cinfo.num_components as usize;
        cinfo.comp_info = Some(alloc_small_vec(
            cinfo,
            JPOOL_IMAGE,
            count.wrapping_mul(SIZEOF_jpeg_component_info),
            count,
            jpeg_component_info::default(),
        )?);
    }

    for ci in 0..cinfo.num_components {
        let index = ci as usize;
        cinfo.comp_info_mut()[index].component_index = ci;
        let Some(component_id) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        cinfo.comp_info_mut()[index].component_id = i32::from(component_id);
        let Some(c) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        let c = i32::from(c);
        let compptr = &mut cinfo.comp_info_mut()[index];
        compptr.h_samp_factor = (c >> 4) & 15;
        compptr.v_samp_factor = c & 15;
        let Some(quant_tbl_no) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        cinfo.comp_info_mut()[index].quant_tbl_no = i32::from(quant_tbl_no);

        let compptr = &cinfo.comp_info()[index];
        let (id, h, v, tq) = (
            compptr.component_id,
            compptr.h_samp_factor,
            compptr.v_samp_factor,
            compptr.quant_tbl_no,
        );
        TRACEMS4(cinfo, 1, J_MESSAGE_CODE::JTRC_SOF_COMPONENT, id, h, v, tq)?;
    }

    cinfo.marker_mut().pub_.saw_SOF = true;

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

// upstream: src/jdmarker.c get_sos
/// Processes a SOS marker.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "component counts and indexes are nonnegative"
)]
fn get_sos(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);

    if !cinfo.marker().pub_.saw_SOF {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_SOS_NO_SOF));
    }

    let Some(length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };

    let Some(n) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    }; /* Number of components */
    let n = i32::from(n);

    TRACEMS1(cinfo, 1, J_MESSAGE_CODE::JTRC_SOS, n)?;

    if length != JLONG::from(n * 2 + 6) || n < 1 || n > MAX_COMPS_IN_SCAN as i32 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_LENGTH));
    }

    cinfo.comps_in_scan = n;

    /* Collect the component-spec parameters */

    for i in 0..MAX_COMPS_IN_SCAN {
        cinfo.cur_comp_info[i] = None;
    }

    for i in 0..n as usize {
        let Some(cc) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        let cc = i32::from(cc);
        let Some(c) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        let c = i32::from(c);

        let mut found = None;
        let mut ci = 0;
        while ci < cinfo.num_components as usize && ci < MAX_COMPS_IN_SCAN {
            if cc == cinfo.comp_info()[ci].component_id && cinfo.cur_comp_info[ci].is_none() {
                found = Some(ci);
                break;
            }
            ci += 1;
        }

        let Some(ci) = found else {
            return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_BAD_COMPONENT_ID, cc));
        };

        cinfo.cur_comp_info[i] = Some(ci);
        let compptr = &mut cinfo.comp_info_mut()[ci];
        compptr.dc_tbl_no = (c >> 4) & 15;
        compptr.ac_tbl_no = c & 15;
        let (dc, ac) = (compptr.dc_tbl_no, compptr.ac_tbl_no);

        TRACEMS3(cinfo, 1, J_MESSAGE_CODE::JTRC_SOS_COMPONENT, cc, dc, ac)?;

        /* This CSi (cc) should differ from the previous CSi */
        for pi in 0..i {
            if cinfo.cur_comp_info[pi] == Some(ci) {
                return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_BAD_COMPONENT_ID, cc));
            }
        }
    }

    /* Collect the additional scan parameters Ss, Se, Ah/Al. */
    let Some(c) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    };
    cinfo.Ss = i32::from(c);
    let Some(c) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    };
    cinfo.Se = i32::from(c);
    let Some(c) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    };
    let c = i32::from(c);
    cinfo.Ah = (c >> 4) & 15;
    cinfo.Al = c & 15;

    TRACEMS4(
        cinfo,
        1,
        J_MESSAGE_CODE::JTRC_SOS_PARAMS,
        cinfo.Ss,
        cinfo.Se,
        cinfo.Ah,
        cinfo.Al,
    )?;

    /* Prepare to scan data & restart markers */
    cinfo.marker_mut().pub_.next_restart_num = 0;

    /* Count another SOS marker */
    cinfo.input_scan_number += 1;

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

// upstream: src/jdmarker.c get_dac
/// Processes a DAC marker (`D_ARITH_CODING_SUPPORTED` is defined).
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions of the table index and value"
)]
fn get_dac(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);

    let Some(mut length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };
    length -= 2;

    while length > 0 {
        let Some(index) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        let index = i32::from(index);
        let Some(val) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        let val = i32::from(val);

        length -= 2;

        TRACEMS2(cinfo, 1, J_MESSAGE_CODE::JTRC_DAC, index, val)?;

        if index < 0 || index >= (2 * NUM_ARITH_TBLS as i32) {
            return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_DAC_INDEX, index));
        }

        if index >= NUM_ARITH_TBLS as i32 {
            /* define AC table */
            cinfo.arith_ac_K[(index - NUM_ARITH_TBLS as i32) as usize] = val as UINT8;
        } else {
            /* define DC table */
            let index = index as usize;
            cinfo.arith_dc_L[index] = (val & 0x0F) as UINT8;
            cinfo.arith_dc_U[index] = (val >> 4) as UINT8;
            if cinfo.arith_dc_L[index] > cinfo.arith_dc_U[index] {
                return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_DAC_VALUE, val));
            }
        }
    }

    if length != 0 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_LENGTH));
    }

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

// upstream: src/jdmarker.c get_dht
/// Processes a DHT marker.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the table index is checked to be nonnegative"
)]
#[expect(clippy::needless_range_loop, reason = "the C loop over the table")]
fn get_dht(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut bits: [UINT8; 17] = [0; 17];
    let mut huffval: [UINT8; 256] = [0; 256];
    let mut vars = INPUT_VARS(cinfo);

    let Some(mut length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };
    length -= 2;

    while length > 16 {
        let Some(index) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        let mut index = i32::from(index);

        TRACEMS1(cinfo, 1, J_MESSAGE_CODE::JTRC_DHT, index)?;

        bits[0] = 0;
        let mut count: i32 = 0;
        for i in 1..=16 {
            let Some(bit) = INPUT_BYTE(cinfo, &mut vars)? else {
                return Ok(false);
            };
            bits[i] = bit;
            count += i32::from(bits[i]);
        }

        length -= 1 + 16;

        let b = |i: usize| i32::from(bits[i]);
        TRACEMS8(
            cinfo,
            2,
            J_MESSAGE_CODE::JTRC_HUFFBITS,
            b(1),
            b(2),
            b(3),
            b(4),
            b(5),
            b(6),
            b(7),
            b(8),
        )?;
        TRACEMS8(
            cinfo,
            2,
            J_MESSAGE_CODE::JTRC_HUFFBITS,
            b(9),
            b(10),
            b(11),
            b(12),
            b(13),
            b(14),
            b(15),
            b(16),
        )?;

        /* Here we just do minimal validation of the counts to avoid walking
         * off the end of our table space.  jdhuff.c will check more carefully.
         */
        if count > 256 || JLONG::from(count) > length {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_HUFF_TABLE));
        }

        for i in 0..count as usize {
            let Some(value) = INPUT_BYTE(cinfo, &mut vars)? else {
                return Ok(false);
            };
            huffval[i] = value;
        }

        huffval[count as usize..].fill(0);

        length -= JLONG::from(count);

        let is_ac = index & 0x10 != 0;
        if is_ac {
            /* AC table definition */
            index -= 0x10;
        }
        /* DC table definition */
        if index < 0 || index >= NUM_HUFF_TBLS as i32 {
            return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_DHT_INDEX, index));
        }
        let index = index as usize;

        let missing = if is_ac {
            cinfo.ac_huff_tbl_ptrs[index].is_none()
        } else {
            cinfo.dc_huff_tbl_ptrs[index].is_none()
        };
        if missing {
            let table = Some(jpeg_alloc_huff_table(cinfo)?);
            if is_ac {
                cinfo.ac_huff_tbl_ptrs[index] = table;
            } else {
                cinfo.dc_huff_tbl_ptrs[index] = table;
            }
        }

        let htbl = if is_ac {
            &mut cinfo.ac_huff_tbl_ptrs[index]
        } else {
            &mut cinfo.dc_huff_tbl_ptrs[index]
        };
        let htbl = htbl.as_mut().expect("allocated above");
        htbl.bits = bits;
        htbl.huffval = huffval;
    }

    if length != 0 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_LENGTH));
    }

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

// upstream: src/jdmarker.c get_dqt
/// Processes a DQT marker.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions of the table index and entries"
)]
#[allow(clippy::needless_range_loop, reason = "the C loop over the table")]
fn get_dqt(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);

    let Some(mut length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };
    length -= 2;

    while length > 0 {
        let Some(n) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        let mut n = i32::from(n);
        let prec = n >> 4;
        n &= 0x0F;

        TRACEMS2(cinfo, 1, J_MESSAGE_CODE::JTRC_DQT, n, prec)?;

        if n >= NUM_QUANT_TBLS as i32 {
            return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_DQT_INDEX, n));
        }
        let n = n as usize;

        if cinfo.quant_tbl_ptrs[n].is_none() {
            cinfo.quant_tbl_ptrs[n] = Some(jpeg_alloc_quant_table(cinfo)?);
        }

        for i in 0..DCTSIZE2 {
            let mut tmp: u32 = 0;
            if prec != 0 {
                if !INPUT_2BYTES(cinfo, &mut vars, &mut tmp)? {
                    return Ok(false);
                }
            } else {
                let Some(byte) = INPUT_BYTE(cinfo, &mut vars)? else {
                    return Ok(false);
                };
                tmp = u32::from(byte);
            }
            /* We convert the zigzag-order table to natural array order. */
            let quant_ptr = cinfo.quant_tbl_ptrs[n].as_mut().expect("allocated above");
            quant_ptr.quantval[jpeg_natural_order[i] as usize] = tmp as UINT16;
        }

        if cinfo.err.trace_level >= 2 {
            for i in (0..DCTSIZE2).step_by(8) {
                let quantval = cinfo.quant_tbl_ptrs[n]
                    .as_ref()
                    .expect("allocated above")
                    .quantval;
                let q = |k: usize| i32::from(quantval[i + k]);
                TRACEMS8(
                    cinfo,
                    2,
                    J_MESSAGE_CODE::JTRC_QUANTVALS,
                    q(0),
                    q(1),
                    q(2),
                    q(3),
                    q(4),
                    q(5),
                    q(6),
                    q(7),
                )?;
            }
        }

        length -= DCTSIZE2 as JLONG + 1;
        if prec != 0 {
            length -= DCTSIZE2 as JLONG;
        }
    }

    if length != 0 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_LENGTH));
    }

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

// upstream: src/jdmarker.c get_dri
/// Processes a DRI marker.
#[expect(clippy::cast_possible_wrap, reason = "the C conversion to int")]
fn get_dri(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);

    let Some(length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };

    if length != 4 {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_LENGTH));
    }

    let mut tmp: u32 = 0;
    if !INPUT_2BYTES(cinfo, &mut vars, &mut tmp)? {
        return Ok(false);
    }

    TRACEMS1(cinfo, 1, J_MESSAGE_CODE::JTRC_DRI, tmp as i32)?;

    cinfo.restart_interval = tmp;

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

// upstream: src/jdmarker.c APP0_DATA_LEN
/// Length of interesting data in APP0.
pub const APP0_DATA_LEN: u32 = 14;
// upstream: src/jdmarker.c APP14_DATA_LEN
/// Length of interesting data in APP14.
pub const APP14_DATA_LEN: u32 = 12;
// upstream: src/jdmarker.c APPN_DATA_LEN
/// Must be the largest of the above.
pub const APPN_DATA_LEN: u32 = 14;

// upstream: src/jdmarker.c examine_app0
/// Examines the first few bytes of an APP0 and takes appropriate action if
/// it is a JFIF marker. `datalen` is the number of bytes at `data` (of which
/// `data` holds at least the first `APPN_DATA_LEN`), `remaining` the length
/// of the rest of the marker data.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversions of the totals to int"
)]
fn examine_app0(
    cinfo: j_decompress_ptr<'_, '_>,
    data: &[JOCTET],
    datalen: u32,
    remaining: JLONG,
) -> Result<(), ErrorExit> {
    let mut totallen = JLONG::from(datalen) + remaining;

    if datalen >= APP0_DATA_LEN
        && data[0] == 0x4A
        && data[1] == 0x46
        && data[2] == 0x49
        && data[3] == 0x46
        && data[4] == 0
    {
        /* Found JFIF APP0 marker: save info */
        cinfo.saw_JFIF_marker = true;
        cinfo.JFIF_major_version = data[5];
        cinfo.JFIF_minor_version = data[6];
        cinfo.density_unit = data[7];
        cinfo.X_density = (UINT16::from(data[8]) << 8) + UINT16::from(data[9]);
        cinfo.Y_density = (UINT16::from(data[10]) << 8) + UINT16::from(data[11]);
        /* Check version.
         * Major version must be 1, anything else signals an incompatible change.
         * (We used to treat this as an error, but now it's a nonfatal warning,
         * because some bozo at Hijaak couldn't read the spec.)
         * Minor version should be 0..2, but process anyway if newer.
         */
        if cinfo.JFIF_major_version != 1 {
            WARNMS2(
                cinfo,
                J_MESSAGE_CODE::JWRN_JFIF_MAJOR,
                i32::from(cinfo.JFIF_major_version),
                i32::from(cinfo.JFIF_minor_version),
            )?;
        }
        /* Generate trace messages */
        TRACEMS5(
            cinfo,
            1,
            J_MESSAGE_CODE::JTRC_JFIF,
            i32::from(cinfo.JFIF_major_version),
            i32::from(cinfo.JFIF_minor_version),
            i32::from(cinfo.X_density),
            i32::from(cinfo.Y_density),
            i32::from(cinfo.density_unit),
        )?;
        /* Validate thumbnail dimensions and issue appropriate messages */
        if data[12] | data[13] != 0 {
            TRACEMS2(
                cinfo,
                1,
                J_MESSAGE_CODE::JTRC_JFIF_THUMBNAIL,
                i32::from(data[12]),
                i32::from(data[13]),
            )?;
        }
        totallen -= JLONG::from(APP0_DATA_LEN);
        if totallen != JLONG::from(data[12]) * JLONG::from(data[13]) * 3 {
            TRACEMS1(
                cinfo,
                1,
                J_MESSAGE_CODE::JTRC_JFIF_BADTHUMBNAILSIZE,
                totallen as i32,
            )?;
        }
    } else if datalen >= 6
        && data[0] == 0x4A
        && data[1] == 0x46
        && data[2] == 0x58
        && data[3] == 0x58
        && data[4] == 0
    {
        /* Found JFIF "JFXX" extension APP0 marker */
        /* The library doesn't actually do anything with these,
         * but we try to produce a helpful trace message.
         */
        match data[5] {
            0x10 => TRACEMS1(cinfo, 1, J_MESSAGE_CODE::JTRC_THUMB_JPEG, totallen as i32)?,
            0x11 => TRACEMS1(
                cinfo,
                1,
                J_MESSAGE_CODE::JTRC_THUMB_PALETTE,
                totallen as i32,
            )?,
            0x13 => TRACEMS1(cinfo, 1, J_MESSAGE_CODE::JTRC_THUMB_RGB, totallen as i32)?,
            _ => TRACEMS2(
                cinfo,
                1,
                J_MESSAGE_CODE::JTRC_JFIF_EXTENSION,
                i32::from(data[5]),
                totallen as i32,
            )?,
        }
    } else {
        /* Start of APP0 does not match "JFIF" or "JFXX", or too short */
        TRACEMS1(cinfo, 1, J_MESSAGE_CODE::JTRC_APP0, totallen as i32)?;
    }
    Ok(())
}

// upstream: src/jdmarker.c examine_app14
/// Examines the first few bytes of an APP14 and takes appropriate action if
/// it is an Adobe marker; see [`examine_app0`].
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "the C conversions to int"
)]
fn examine_app14(
    cinfo: j_decompress_ptr<'_, '_>,
    data: &[JOCTET],
    datalen: u32,
    remaining: JLONG,
) -> Result<(), ErrorExit> {
    if datalen >= APP14_DATA_LEN
        && data[0] == 0x41
        && data[1] == 0x64
        && data[2] == 0x6F
        && data[3] == 0x62
        && data[4] == 0x65
    {
        /* Found Adobe APP14 marker */
        let version = (u32::from(data[5]) << 8) + u32::from(data[6]);
        let flags0 = (u32::from(data[7]) << 8) + u32::from(data[8]);
        let flags1 = (u32::from(data[9]) << 8) + u32::from(data[10]);
        let transform = u32::from(data[11]);
        TRACEMS4(
            cinfo,
            1,
            J_MESSAGE_CODE::JTRC_ADOBE,
            version as i32,
            flags0 as i32,
            flags1 as i32,
            transform as i32,
        )?;
        cinfo.saw_Adobe_marker = true;
        cinfo.Adobe_transform = transform as UINT8;
    } else {
        /* Start of APP14 does not match "Adobe", or too short */
        TRACEMS1(
            cinfo,
            1,
            J_MESSAGE_CODE::JTRC_APP14,
            (JLONG::from(datalen) + remaining) as i32,
        )?;
    }
    Ok(())
}

// upstream: src/jdmarker.c get_interesting_appn
/// Processes an APP0 or APP14 marker without saving it.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversions of the length"
)]
#[expect(clippy::needless_range_loop, reason = "the C loop over the buffer")]
fn get_interesting_appn(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut b: [JOCTET; APPN_DATA_LEN as usize] = [0; APPN_DATA_LEN as usize];
    let mut vars = INPUT_VARS(cinfo);

    let Some(mut length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };
    length -= 2;

    /* get the interesting part of the marker data */
    let numtoread: u32 = if length >= JLONG::from(APPN_DATA_LEN) {
        APPN_DATA_LEN
    } else if length > 0 {
        length as u32
    } else {
        0
    };
    for i in 0..numtoread as usize {
        let Some(byte) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        b[i] = byte;
    }
    length -= JLONG::from(numtoread);

    /* process it */
    match JPEG_MARKER(cinfo.unread_marker.cast_unsigned()) {
        JPEG_MARKER::M_APP0 => examine_app0(cinfo, &b, numtoread, length)?,
        JPEG_MARKER::M_APP14 => examine_app14(cinfo, &b, numtoread, length)?,
        _ => {
            /* can't get here unless jpeg_save_markers chooses wrong processor */
            return Err(ERREXIT1(
                cinfo,
                J_MESSAGE_CODE::JERR_UNKNOWN_MARKER,
                cinfo.unread_marker,
            ));
        }
    }

    /* skip any remaining data -- could be lots */
    INPUT_SYNC(cinfo, &vars);
    if length > 0 {
        call_skip_input_data(cinfo, length as c_long)?;
    }

    Ok(true)
}

/// Appends a completely read marker to `cinfo.marker_list`, at the tail that
/// `master.marker_list_end` names, or starts the list when either is null.
fn append_marker(cinfo: j_decompress_ptr<'_, '_>, cur_marker: Box<jpeg_marker_struct>) {
    let end = cinfo.master().pub_.marker_list_end;
    if let (Some(mut tail), Some(end)) = (cinfo.marker_list.as_mut(), end) {
        let mut position = 0;
        while position < end && tail.next.is_some() {
            tail = tail.next.as_mut().expect("checked above");
            position += 1;
        }
        tail.next = Some(cur_marker);
        cinfo.master_mut().pub_.marker_list_end = Some(position + 1);
    } else {
        cinfo.marker_list = Some(cur_marker);
        cinfo.master_mut().pub_.marker_list_end = Some(0);
    }
}

// upstream: src/jdmarker.c save_marker
/// Saves an `APPn` or COM marker into the marker list. When it must suspend,
/// it keeps the partly read marker and its progress in the marker reader.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the C conversions of the length and marker code"
)]
fn save_marker(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut length: JLONG = 0;
    let mut vars = INPUT_VARS(cinfo);
    let mut bytes_read: u32;
    let data_length: u32;

    if cinfo.marker().cur_marker.is_none() {
        /* begin reading a marker */
        let Some(value) = input_length(cinfo, &mut vars)? else {
            return Ok(false);
        };
        length = value - 2;
        if length >= 0 {
            /* watch out for bogus length word */
            /* figure out how much we want to save */
            let marker = cinfo.marker();
            let mut limit = if cinfo.unread_marker == JPEG_MARKER::M_COM.code() {
                marker.length_limit_COM
            } else {
                marker.length_limit_APPn
                    [(cinfo.unread_marker - JPEG_MARKER::M_APP0.code()) as usize]
            };
            if (length as u32) < limit {
                limit = length as u32;
            }
            /* allocate and initialize the marker item */
            let data = alloc_large_vec(
                cinfo,
                JPOOL_IMAGE,
                SIZEOF_jpeg_marker_struct + limit as usize,
                limit as usize,
                0,
            )?;
            let cur_marker = Box::new(jpeg_marker_struct {
                next: None,
                marker: cinfo.unread_marker as UINT8,
                original_length: length as u32,
                data_length: limit,
                /* data area is just beyond the jpeg_marker_struct */
                data,
            });
            let marker = cinfo.marker_mut();
            marker.cur_marker = Some(cur_marker);
            marker.bytes_read = 0;
            bytes_read = 0;
            data_length = limit;
        } else {
            /* deal with bogus length word */
            bytes_read = 0;
            data_length = 0;
        }
    } else {
        /* resume reading a marker */
        let marker = cinfo.marker();
        bytes_read = marker.bytes_read;
        data_length = marker
            .cur_marker
            .as_ref()
            .expect("checked above")
            .data_length;
    }

    while bytes_read < data_length {
        INPUT_SYNC(cinfo, &vars); /* move the restart point to here */
        cinfo.marker_mut().bytes_read = bytes_read;
        /* If there's not at least one byte in buffer, suspend */
        if !MAKE_BYTE_AVAIL(cinfo, &mut vars)? {
            return Ok(false);
        }
        /* Copy bytes with reasonable rapidity */
        let source = cinfo
            .src
            .as_deref()
            .expect("cinfo->src is NULL")
            .buffer
            .bytes();
        let marker = cinfo.marker.as_deref_mut().expect("cinfo->marker is NULL");
        let data = &mut marker.cur_marker.as_mut().expect("being read").data;
        while bytes_read < data_length && vars.bytes_in_buffer > 0 {
            data[bytes_read as usize] = source[vars.next_input_byte];
            vars.next_input_byte += 1;
            vars.bytes_in_buffer -= 1;
            bytes_read += 1;
        }
    }

    /* Done reading what we want to read */
    let mut data: [JOCTET; APPN_DATA_LEN as usize] = [0; APPN_DATA_LEN as usize];
    if let Some(cur_marker) = cinfo.marker_mut().cur_marker.take() {
        /* will be NULL if bogus length word */
        let shown = cur_marker.data.len().min(APPN_DATA_LEN as usize);
        data[..shown].copy_from_slice(&cur_marker.data[..shown]);
        /* Reset pointer & calc remaining data length */
        length = JLONG::from(cur_marker.original_length - data_length);
        /* Add new marker to end of list */
        append_marker(cinfo, cur_marker);
    }
    /* Reset to initial state for next marker */
    cinfo.marker_mut().cur_marker = None;

    /* Process the marker if interesting; else just make a generic trace msg */
    match JPEG_MARKER(cinfo.unread_marker.cast_unsigned()) {
        JPEG_MARKER::M_APP0 => examine_app0(cinfo, &data, data_length, length)?,
        JPEG_MARKER::M_APP14 => examine_app14(cinfo, &data, data_length, length)?,
        _ => TRACEMS2(
            cinfo,
            1,
            J_MESSAGE_CODE::JTRC_MISC_MARKER,
            cinfo.unread_marker,
            (JLONG::from(data_length) + length) as i32,
        )?,
    }

    /* skip any remaining data -- could be lots */
    INPUT_SYNC(cinfo, &vars); /* do before skip_input_data */
    if length > 0 {
        call_skip_input_data(cinfo, length as c_long)?;
    }

    Ok(true)
}

// upstream: src/jdmarker.c skip_variable
/// Skips over an unknown or uninteresting variable-length marker.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion of the length to int"
)]
fn skip_variable(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);

    let Some(mut length) = input_length(cinfo, &mut vars)? else {
        return Ok(false);
    };
    length -= 2;

    TRACEMS2(
        cinfo,
        1,
        J_MESSAGE_CODE::JTRC_MISC_MARKER,
        cinfo.unread_marker,
        length as i32,
    )?;

    INPUT_SYNC(cinfo, &vars); /* do before skip_input_data */
    if length > 0 {
        call_skip_input_data(cinfo, length as c_long)?;
    }

    Ok(true)
}

// upstream: src/jdmarker.c next_marker
/// Finds the next JPEG marker and saves it in `cinfo.unread_marker`;
/// `false` if it had to suspend before reaching a marker, in which case
/// `unread_marker` is unchanged. The result might not be a valid marker code,
/// but it is never 0 or 0xFF.
fn next_marker(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);
    let mut c;

    loop {
        let Some(byte) = INPUT_BYTE(cinfo, &mut vars)? else {
            return Ok(false);
        };
        c = byte;
        /* Skip any non-FF bytes.
         * This may look a bit inefficient, but it will not occur in a valid file.
         * We sync after each discarded byte so that a suspending data source
         * can discard the byte from its buffer.
         */
        while c != 0xFF {
            let marker = cinfo.marker_mut();
            marker.pub_.discarded_bytes = marker.pub_.discarded_bytes.wrapping_add(1);
            INPUT_SYNC(cinfo, &vars);
            let Some(byte) = INPUT_BYTE(cinfo, &mut vars)? else {
                return Ok(false);
            };
            c = byte;
        }
        /* This loop swallows any duplicate FF bytes.  Extra FFs are legal as
         * pad bytes, so don't count them in discarded_bytes.  We assume there
         * will not be so many consecutive FF bytes as to overflow a suspending
         * data source's input buffer.
         */
        loop {
            let Some(byte) = INPUT_BYTE(cinfo, &mut vars)? else {
                return Ok(false);
            };
            c = byte;
            if c != 0xFF {
                break;
            }
        }
        if c != 0 {
            break; /* found a valid marker, exit loop */
        }
        /* Reach here if we found a stuffed-zero data sequence (FF/00).
         * Discard it and loop back to try again.
         */
        let marker = cinfo.marker_mut();
        marker.pub_.discarded_bytes = marker.pub_.discarded_bytes.wrapping_add(2);
        INPUT_SYNC(cinfo, &vars);
    }

    let discarded_bytes = cinfo.marker().pub_.discarded_bytes;
    if discarded_bytes != 0 {
        WARNMS2(
            cinfo,
            J_MESSAGE_CODE::JWRN_EXTRANEOUS_DATA,
            discarded_bytes.cast_signed(),
            i32::from(c),
        )?;
        cinfo.marker_mut().pub_.discarded_bytes = 0;
    }

    cinfo.unread_marker = i32::from(c);

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

// upstream: src/jdmarker.c first_marker
/// Like `next_marker`, but used to obtain the initial SOI marker, which may
/// not be preceded by garbage or fill.
fn first_marker(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    let mut vars = INPUT_VARS(cinfo);

    let Some(c) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    };
    let Some(c2) = INPUT_BYTE(cinfo, &mut vars)? else {
        return Ok(false);
    };
    let (c, c2) = (i32::from(c), i32::from(c2));
    if c != 0xFF || c2 != JPEG_MARKER::M_SOI.code() {
        return Err(ERREXIT2(cinfo, J_MESSAGE_CODE::JERR_NO_SOI, c, c2));
    }

    cinfo.unread_marker = c2;

    INPUT_SYNC(cinfo, &vars);
    Ok(true)
}

/// Which marker processor of the marker reader to call. Not an upstream
/// item.
#[derive(Clone, Copy, Debug)]
enum Processor {
    Com,
    App(usize),
}

/// `(*marker->process_COM) (cinfo)` or `(*marker->process_APPn[n])
/// (cinfo)`: copies a library routine out, or takes an application's
/// processor out of the marker reader for the call and puts it back after,
/// replacing whatever the processor installed for its own slot meanwhile.
fn call_processor(cinfo: j_decompress_ptr<'_, '_>, which: Processor) -> Result<bool, ErrorExit> {
    let marker = cinfo.marker_mut();
    let method = match which {
        Processor::Com => &mut marker.process_COM,
        Processor::App(n) => &mut marker.process_APPn[n],
    };
    if let jpeg_marker_parser_method::Library(routine) = method {
        let routine = *routine;
        return routine(cinfo);
    }
    let taken = core::mem::replace(method, jpeg_marker_parser_method::Library(skip_variable));
    let jpeg_marker_parser_method::Application(mut processor) = taken else {
        unreachable!("checked above")
    };
    let result = processor.process_marker(cinfo);
    let marker = cinfo.marker_mut();
    let method = match which {
        Processor::Com => &mut marker.process_COM,
        Processor::App(n) => &mut marker.process_APPn[n],
    };
    *method = jpeg_marker_parser_method::Application(processor);
    result
}

// upstream: src/jdmarker.c read_markers
/// Reads markers until SOS or EOI: `JPEG_SUSPENDED`, `JPEG_REACHED_SOS` or
/// `JPEG_REACHED_EOI`.
///
/// # Errors
///
/// A libjpeg error, recorded in `cinfo.err`.
#[expect(clippy::cast_sign_loss, reason = "APPn marker codes are above M_APP0")]
#[expect(clippy::too_many_lines, reason = "the C switch over the marker codes")]
pub fn read_markers(cinfo: j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit> {
    /* Outer loop repeats once for each marker. */
    loop {
        /* Collect the marker proper, unless we already did. */
        /* NB: first_marker() enforces the requirement that SOI appear first. */
        if cinfo.unread_marker == 0 {
            if !cinfo.marker().pub_.saw_SOI {
                if !first_marker(cinfo)? {
                    return Ok(JPEG_SUSPENDED);
                }
            } else if !next_marker(cinfo)? {
                return Ok(JPEG_SUSPENDED);
            }
        }
        /* At this point cinfo->unread_marker contains the marker code and the
         * input point is just past the marker proper, but before any parameters.
         * A suspension will cause us to return with this state still true.
         */
        let complete = match JPEG_MARKER(cinfo.unread_marker.cast_unsigned()) {
            JPEG_MARKER::M_SOI => get_soi(cinfo)?,

            JPEG_MARKER::M_SOF0 | /* Baseline */
            JPEG_MARKER::M_SOF1 => /* Extended sequential, Huffman */
                get_sof(cinfo, false, false, false)?,

            JPEG_MARKER::M_SOF2 => /* Progressive, Huffman */
                get_sof(cinfo, true, false, false)?,

            JPEG_MARKER::M_SOF3 => /* Lossless, Huffman */
                get_sof(cinfo, false, true, false)?,

            JPEG_MARKER::M_SOF9 => /* Extended sequential, arithmetic */
                get_sof(cinfo, false, false, true)?,

            JPEG_MARKER::M_SOF10 => /* Progressive, arithmetic */
                get_sof(cinfo, true, false, true)?,

            JPEG_MARKER::M_SOF11 => /* Lossless, arithmetic */
                get_sof(cinfo, false, true, true)?,

            /* Currently unsupported SOFn types */
            JPEG_MARKER::M_SOF5 |  /* Differential sequential, Huffman */
            JPEG_MARKER::M_SOF6 |  /* Differential progressive, Huffman */
            JPEG_MARKER::M_SOF7 |  /* Differential lossless, Huffman */
            JPEG_MARKER::M_JPG |   /* Reserved for JPEG extensions */
            JPEG_MARKER::M_SOF13 | /* Differential sequential, arithmetic */
            JPEG_MARKER::M_SOF14 | /* Differential progressive, arithmetic */
            JPEG_MARKER::M_SOF15 => /* Differential lossless, arithmetic */
            {
                return Err(ERREXIT1(
                    cinfo,
                    J_MESSAGE_CODE::JERR_SOF_UNSUPPORTED,
                    cinfo.unread_marker,
                ));
            }

            JPEG_MARKER::M_SOS => {
                if !get_sos(cinfo)? {
                    return Ok(JPEG_SUSPENDED);
                }
                cinfo.unread_marker = 0; /* processed the marker */
                return Ok(JPEG_REACHED_SOS);
            }

            JPEG_MARKER::M_EOI => {
                TRACEMS(cinfo, 1, J_MESSAGE_CODE::JTRC_EOI)?;
                cinfo.unread_marker = 0; /* processed the marker */
                return Ok(JPEG_REACHED_EOI);
            }

            JPEG_MARKER::M_DAC => get_dac(cinfo)?,

            JPEG_MARKER::M_DHT => get_dht(cinfo)?,

            JPEG_MARKER::M_DQT => get_dqt(cinfo)?,

            JPEG_MARKER::M_DRI => get_dri(cinfo)?,

            JPEG_MARKER::M_APP0
            | JPEG_MARKER::M_APP1
            | JPEG_MARKER::M_APP2
            | JPEG_MARKER::M_APP3
            | JPEG_MARKER::M_APP4
            | JPEG_MARKER::M_APP5
            | JPEG_MARKER::M_APP6
            | JPEG_MARKER::M_APP7
            | JPEG_MARKER::M_APP8
            | JPEG_MARKER::M_APP9
            | JPEG_MARKER::M_APP10
            | JPEG_MARKER::M_APP11
            | JPEG_MARKER::M_APP12
            | JPEG_MARKER::M_APP13
            | JPEG_MARKER::M_APP14
            | JPEG_MARKER::M_APP15 => {
                let n = (cinfo.unread_marker - JPEG_MARKER::M_APP0.code()) as usize;
                call_processor(cinfo, Processor::App(n))?
            }

            JPEG_MARKER::M_COM => call_processor(cinfo, Processor::Com)?,

            JPEG_MARKER::M_RST0 | /* these are all parameterless */
            JPEG_MARKER::M_RST1
            | JPEG_MARKER::M_RST2
            | JPEG_MARKER::M_RST3
            | JPEG_MARKER::M_RST4
            | JPEG_MARKER::M_RST5
            | JPEG_MARKER::M_RST6
            | JPEG_MARKER::M_RST7
            | JPEG_MARKER::M_TEM => {
                TRACEMS1(
                    cinfo,
                    1,
                    J_MESSAGE_CODE::JTRC_PARMLESS_MARKER,
                    cinfo.unread_marker,
                )?;
                true
            }

            JPEG_MARKER::M_DNL => /* Ignore DNL ... perhaps the wrong thing */
                skip_variable(cinfo)?,

            _ => {
                /* must be DHP, EXP, JPGn, or RESn */
                /* For now, we treat the reserved markers as fatal errors since they are
                 * likely to be used to signal incompatible JPEG Part 3 extensions.
                 * Once the JPEG 3 version-number marker is well defined, this code
                 * ought to change!
                 */
                return Err(ERREXIT1(
                    cinfo,
                    J_MESSAGE_CODE::JERR_UNKNOWN_MARKER,
                    cinfo.unread_marker,
                ));
            }
        };
        if !complete {
            return Ok(JPEG_SUSPENDED);
        }
        /* Successfully processed marker, so reset state variable */
        cinfo.unread_marker = 0;
    } /* end loop */
}

// upstream: src/jdmarker.c read_restart_marker
/// Reads a restart marker, which is expected to appear next in the
/// datastream; if the marker is not there, takes appropriate recovery
/// action. `false` if suspension is required. Called by the entropy decoder.
///
/// # Errors
///
/// A libjpeg error, recorded in `cinfo.err`.
pub fn read_restart_marker(cinfo: j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit> {
    /* Obtain a marker unless we already did. */
    /* Note that next_marker will complain if it skips any data. */
    if cinfo.unread_marker == 0 && !next_marker(cinfo)? {
        return Ok(false);
    }

    let next_restart_num = cinfo.marker().pub_.next_restart_num;
    if cinfo.unread_marker == JPEG_MARKER::M_RST0.code() + next_restart_num {
        /* Normal case --- swallow the marker and let entropy decoder continue */
        TRACEMS1(cinfo, 3, J_MESSAGE_CODE::JTRC_RST, next_restart_num)?;
        cinfo.unread_marker = 0;
    } else {
        /* Uh-oh, the restart markers have been messed up. */
        /* Let the data source manager determine how to resync. */
        if !call_resync_to_restart(cinfo, next_restart_num)? {
            return Ok(false);
        }
    }

    /* Update next-restart state */
    let marker = cinfo.marker_mut();
    marker.pub_.next_restart_num = (marker.pub_.next_restart_num + 1) & 7;

    Ok(true)
}

// upstream: src/jdmarker.c jpeg_resync_to_restart
/// The default `resync_to_restart` method for data source managers:
/// discards the marker, scans forward to the next one, or leaves it unread,
/// depending on how far the found marker is from the desired restart marker
/// (see upstream's comment). `false` if suspension is required.
///
/// # Errors
///
/// A libjpeg error, recorded in `cinfo.err`.
pub fn jpeg_resync_to_restart(
    cinfo: j_decompress_ptr<'_, '_>,
    desired: i32,
) -> Result<bool, ErrorExit> {
    let mut marker = cinfo.unread_marker;
    let mut action;

    /* Always put up a warning. */
    WARNMS2(cinfo, J_MESSAGE_CODE::JWRN_MUST_RESYNC, marker, desired)?;

    /* Outer loop handles repeated decision after scanning forward. */
    loop {
        let rst0 = JPEG_MARKER::M_RST0.code();
        if marker < JPEG_MARKER::M_SOF0.code() {
            action = 2; /* invalid marker */
        } else if marker < rst0 || marker > JPEG_MARKER::M_RST7.code() {
            action = 3; /* valid non-restart marker */
        } else if marker == rst0 + ((desired + 1) & 7) || marker == rst0 + ((desired + 2) & 7) {
            action = 3; /* one of the next two expected restarts */
        } else if marker == rst0 + ((desired - 1) & 7) || marker == rst0 + ((desired - 2) & 7) {
            action = 2; /* a prior restart, so advance */
        } else {
            action = 1; /* desired restart or too far away */
        }
        TRACEMS2(
            cinfo,
            4,
            J_MESSAGE_CODE::JTRC_RECOVERY_ACTION,
            marker,
            action,
        )?;
        match action {
            1 => {
                /* Discard marker and let entropy decoder resume processing. */
                cinfo.unread_marker = 0;
                return Ok(true);
            }
            2 => {
                /* Scan to the next marker, and repeat the decision loop. */
                if !next_marker(cinfo)? {
                    return Ok(false);
                }
                marker = cinfo.unread_marker;
            }
            _ => {
                /* Return without advancing past this marker. */
                /* Entropy decoder will be forced to process an empty segment. */
                return Ok(true);
            }
        }
    } /* end loop */
}

// upstream: src/jdmarker.c reset_marker_reader
/// Resets marker processing state to begin a fresh datastream.
pub fn reset_marker_reader(cinfo: j_decompress_ptr<'_, '_>) {
    cinfo.comp_info = None; /* until allocated by get_sof */
    cinfo.input_scan_number = 0; /* no SOS seen yet */
    cinfo.unread_marker = 0; /* no pending marker */
    let marker = cinfo.marker_mut();
    marker.pub_.saw_SOI = false; /* set internal state too */
    marker.pub_.saw_SOF = false;
    marker.pub_.discarded_bytes = 0;
    marker.cur_marker = None;
}

// upstream: src/jdmarker.c jinit_marker_reader
/// Initializes the marker reader module; called only once, when the
/// decompression object is created. By default APP0 and APP14 are examined
/// and discarded, COM and all other `APPn` simply discarded.
///
/// # Errors
///
/// What `alloc_small` reports.
#[expect(non_snake_case, reason = "named after the C field")]
pub fn jinit_marker_reader(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Create subobject in permanent pool */
    let mut process_APPn: [jpeg_marker_parser_method<'_>; 16] =
        core::array::from_fn(|_| jpeg_marker_parser_method::Library(skip_variable));
    process_APPn[0] = jpeg_marker_parser_method::Library(get_interesting_appn);
    process_APPn[14] = jpeg_marker_parser_method::Library(get_interesting_appn);
    let marker = my_marker_reader {
        /* Initialize public method pointers */
        pub_: jpeg_marker_reader {
            reset_marker_reader,
            read_markers,
            read_restart_marker,
            saw_SOI: false,
            saw_SOF: false,
            next_restart_num: 0,
            discarded_bytes: 0,
        },
        /* Initialize COM/APPn processing. */
        process_COM: jpeg_marker_parser_method::Library(skip_variable),
        process_APPn,
        length_limit_COM: 0,
        length_limit_APPn: [0; 16],
        cur_marker: None,
        bytes_read: 0,
    };
    cinfo.marker = Some(alloc_small_box(
        cinfo,
        JPOOL_PERMANENT,
        SIZEOF_my_marker_reader,
        marker,
    )?);
    /* Reset marker processing state */
    reset_marker_reader(cinfo);
    Ok(())
}

// upstream: src/jdmarker.c jpeg_save_markers
/// Controls saving of COM and `APPn` markers into `marker_list`.
///
/// # Errors
///
/// `JERR_UNKNOWN_MARKER` for any other marker code.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions between long, size_t and unsigned int"
)]
pub fn jpeg_save_markers(
    cinfo: j_decompress_ptr<'_, '_>,
    marker_code: i32,
    mut length_limit: u32,
) -> Result<(), ErrorExit> {
    /* Length limit mustn't be larger than what we can allocate
     * (should only be a concern in a 16-bit environment).
     */
    let maxlength = (cinfo.mem().pub_.max_alloc_chunk as usize)
        .wrapping_sub(SIZEOF_jpeg_marker_struct) as c_long;
    if c_long::from(length_limit) > maxlength {
        length_limit = maxlength as u32;
    }

    /* Choose processor routine to use.
     * APP0/APP14 have special requirements.
     */
    let processor: fn(j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit>;
    if length_limit != 0 {
        processor = save_marker;
        /* If saving APP0/APP14, save at least enough for our internal use. */
        if marker_code == JPEG_MARKER::M_APP0.code() && length_limit < APP0_DATA_LEN {
            length_limit = APP0_DATA_LEN;
        } else if marker_code == JPEG_MARKER::M_APP14.code() && length_limit < APP14_DATA_LEN {
            length_limit = APP14_DATA_LEN;
        }
    } else if marker_code == JPEG_MARKER::M_APP0.code()
        || marker_code == JPEG_MARKER::M_APP14.code()
    {
        /* If discarding APP0/APP14, use our regular on-the-fly processor. */
        processor = get_interesting_appn;
    } else {
        processor = skip_variable;
    }

    if marker_code == JPEG_MARKER::M_COM.code() {
        let marker = cinfo.marker_mut();
        marker.process_COM = jpeg_marker_parser_method::Library(processor);
        marker.length_limit_COM = length_limit;
    } else if marker_code >= JPEG_MARKER::M_APP0.code()
        && marker_code <= JPEG_MARKER::M_APP15.code()
    {
        let n = (marker_code - JPEG_MARKER::M_APP0.code()) as usize;
        let marker = cinfo.marker_mut();
        marker.process_APPn[n] = jpeg_marker_parser_method::Library(processor);
        marker.length_limit_APPn[n] = length_limit;
    } else {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_UNKNOWN_MARKER,
            marker_code,
        ));
    }
    Ok(())
}

// upstream: src/jdmarker.c jpeg_set_marker_processor
/// Installs a special processing method for COM or `APPn` markers.
///
/// # Errors
///
/// `JERR_UNKNOWN_MARKER` for any other marker code.
#[expect(clippy::cast_sign_loss, reason = "APPn marker codes are above M_APP0")]
pub fn jpeg_set_marker_processor<'src>(
    cinfo: &mut jpeg_decompress_struct<'src>,
    marker_code: i32,
    routine: Box<dyn jpeg_marker_processor + 'src>,
) -> Result<(), ErrorExit> {
    if marker_code == JPEG_MARKER::M_COM.code() {
        cinfo.marker_mut().process_COM = jpeg_marker_parser_method::Application(routine);
    } else if marker_code >= JPEG_MARKER::M_APP0.code()
        && marker_code <= JPEG_MARKER::M_APP15.code()
    {
        let n = (marker_code - JPEG_MARKER::M_APP0.code()) as usize;
        cinfo.marker_mut().process_APPn[n] = jpeg_marker_parser_method::Application(routine);
    } else {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_UNKNOWN_MARKER,
            marker_code,
        ));
    }
    Ok(())
}
