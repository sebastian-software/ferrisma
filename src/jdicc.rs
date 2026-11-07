// libjpeg-turbo 3.2.1, src/jdicc.c:
//
// /*
//  * jdicc.c
//  *
//  * Copyright (C) 1997-1998, Thomas G. Lane, Todd Newman.
//  * Copyright (C) 2017, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file provides code to read International Color Consortium (ICC) device
//  * profiles embedded in JFIF JPEG image files.  The ICC has defined a standard
//  * for including such data in JPEG "APP2" markers.  The code given here does
//  * not know anything about the internal structure of the ICC profile data; it
//  * just knows how to get the profile data from a JPEG file while reading it.
//  */
//! Translation of `src/jdicc.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 2 (Input and markers).
//!
//! The assembled profile is returned in a `Vec`, where C returns memory from
//! `malloc` that the caller frees; a refused reservation reports the
//! `JERR_OUT_OF_MEMORY` that a failing `malloc` does. The out-parameters are
//! `Option`s, so that a missing one still fails with `JERR_BUFFER_SIZE`.

use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE, WARNMS};
use crate::jmorecfg::JOCTET;
use crate::jpegint::DSTATE_READY;
use crate::jpeglib::{JPEG_APP0, j_decompress_ptr, jpeg_marker_struct};

// upstream: src/jdicc.c ICC_MARKER
/// JPEG marker code for ICC.
pub const ICC_MARKER: i32 = JPEG_APP0 + 2;
// upstream: src/jdicc.c ICC_OVERHEAD_LEN
/// Size of non-profile data in APP2.
pub const ICC_OVERHEAD_LEN: u32 = 14;
// upstream: src/jdicc.c MAX_SEQ_NO
/// Sufficient since marker numbers are bytes.
pub const MAX_SEQ_NO: usize = 255;

// upstream: src/jdicc.c marker_is_icc
/// Tests whether a saved marker is an ICC profile marker.
fn marker_is_icc(marker: &jpeg_marker_struct) -> bool {
    i32::from(marker.marker) == ICC_MARKER &&
    marker.data_length >= ICC_OVERHEAD_LEN &&
    /* verify the identifying string */
    marker.data[0] == 0x49 &&
    marker.data[1] == 0x43 &&
    marker.data[2] == 0x43 &&
    marker.data[3] == 0x5F &&
    marker.data[4] == 0x50 &&
    marker.data[5] == 0x52 &&
    marker.data[6] == 0x4F &&
    marker.data[7] == 0x46 &&
    marker.data[8] == 0x49 &&
    marker.data[9] == 0x4C &&
    marker.data[10] == 0x45 &&
    marker.data[11] == 0x0
}

// upstream: src/jdicc.c jpeg_read_icc_profile
/// Reassembles the ICC profile from the saved APP2 markers, if there is one:
/// `true` with the data in `icc_data_ptr` and its length in `icc_data_len`,
/// `false` if there is none or the markers are inconsistent (with the
/// warning `JWRN_BOGUS_ICC`).
///
/// # Errors
///
/// `JERR_BUFFER_SIZE` without an out-parameter, `JERR_BAD_STATE` before
/// `jpeg_read_header`, `JERR_OUT_OF_MEMORY` (case 11) when the profile
/// cannot be allocated, or what `emit_message` returns for a warning.
pub fn jpeg_read_icc_profile(
    cinfo: j_decompress_ptr<'_, '_>,
    icc_data_ptr: Option<&mut Option<Vec<JOCTET>>>,
    icc_data_len: Option<&mut u32>,
) -> Result<bool, ErrorExit> {
    let mut num_markers: i32 = 0;
    let mut marker_present: [u8; MAX_SEQ_NO + 1] = [0; MAX_SEQ_NO + 1]; /* 1 if marker found */
    let mut data_length: [u32; MAX_SEQ_NO + 1] = [0; MAX_SEQ_NO + 1]; /* size of profile data in marker */
    let mut data_offset: [u32; MAX_SEQ_NO + 1] = [0; MAX_SEQ_NO + 1]; /* offset for data in marker */

    let (Some(icc_data_ptr), Some(icc_data_len)) = (icc_data_ptr, icc_data_len) else {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BUFFER_SIZE));
    };
    if cinfo.global_state < DSTATE_READY {
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_STATE,
            cinfo.global_state,
        ));
    }

    *icc_data_ptr = None; /* avoid confusion if FALSE return */
    *icc_data_len = 0;

    /* This first pass over the saved markers discovers whether there are
     * any ICC markers and verifies the consistency of the marker numbering.
     */

    for present in &mut marker_present[1..=MAX_SEQ_NO] {
        *present = 0;
    }

    let mut bogus = false;
    let mut marker = cinfo.marker_list.as_deref();
    while let Some(current) = marker {
        if marker_is_icc(current) {
            if num_markers == 0 {
                num_markers = i32::from(current.data[13]);
            } else if num_markers != i32::from(current.data[13]) {
                bogus = true; /* inconsistent num_markers fields */
                break;
            }
            let seq_no = i32::from(current.data[12]);
            if seq_no <= 0 || seq_no > num_markers {
                bogus = true; /* bogus sequence number */
                break;
            }
            let seq_no = seq_no.unsigned_abs() as usize;
            if marker_present[seq_no] != 0 {
                bogus = true; /* duplicate sequence numbers */
                break;
            }
            marker_present[seq_no] = 1;
            data_length[seq_no] = current.data_length - ICC_OVERHEAD_LEN;
        }
        marker = current.next.as_deref();
    }
    if bogus {
        WARNMS(cinfo, J_MESSAGE_CODE::JWRN_BOGUS_ICC)?;
        return Ok(false);
    }

    if num_markers == 0 {
        return Ok(false);
    }

    /* Check for missing markers, count total space needed,
     * compute offset of each marker's part of the data.
     */

    let mut total_length: u32 = 0;
    for seq_no in 1..=num_markers.unsigned_abs() as usize {
        if marker_present[seq_no] == 0 {
            WARNMS(cinfo, J_MESSAGE_CODE::JWRN_BOGUS_ICC)?; /* missing sequence number */
            return Ok(false);
        }
        data_offset[seq_no] = total_length;
        total_length = total_length.wrapping_add(data_length[seq_no]);
    }

    if total_length == 0 {
        WARNMS(cinfo, J_MESSAGE_CODE::JWRN_BOGUS_ICC)?; /* found only empty markers? */
        return Ok(false);
    }

    /* Allocate space for assembled data */
    let mut icc_data: Vec<JOCTET> = Vec::new();
    if icc_data.try_reserve_exact(total_length as usize).is_err() {
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_OUT_OF_MEMORY, 11)); /* oops, out of memory */
    }
    icc_data.resize(total_length as usize, 0);

    /* and fill it in */
    let mut marker = cinfo.marker_list.as_deref();
    while let Some(current) = marker {
        if marker_is_icc(current) {
            let seq_no = usize::from(current.data[12]);
            let dst_ptr = data_offset[seq_no] as usize;
            let src_ptr = ICC_OVERHEAD_LEN as usize;
            let length = data_length[seq_no] as usize;
            icc_data[dst_ptr..dst_ptr + length]
                .copy_from_slice(&current.data[src_ptr..src_ptr + length]);
        }
        marker = current.next.as_deref();
    }

    *icc_data_ptr = Some(icc_data);
    *icc_data_len = total_length;

    Ok(true)
}
