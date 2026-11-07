// libjpeg-turbo 3.2.1, src/jddctmgr.c:
//
// /*
//  * jddctmgr.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1994-1996, Thomas G. Lane.
//  * Modified 2002-2010 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright 2009 Pierre Ossman <ossman@cendio.se> for Cendio AB
//  * Copyright (C) 2010, 2015, 2022, 2025-2026, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains the inverse-DCT management logic.
//  * This code selects a particular IDCT implementation to be used,
//  * and it performs related housekeeping chores.  No code in this file
//  * is executed per IDCT step, only during output pass setup.
//  *
//  * Note that the IDCT routines are responsible for performing coefficient
//  * dequantization as well as the IDCT proper.  This module sets up the
//  * dequantization multiplier table needed by the IDCT routine.
//  */
//! Translation of `src/jddctmgr.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 4 (Coefficients and IDCT).
//!
//! `WITH_SIMD` is undefined, so `start_pass` selects the C routines. The
//! multiplier table, a union of three arrays in C, keeps the bits of all
//! three in one array of `int`s: the integer methods' arrays share its
//! elements, and the float array reads and writes their bits (ADR 0005,
//! "Messages" keeps `msg_parm`'s union the same way). A table set up for one
//! method and read by another is therefore read as C reads it.

use crate::jconfig::BITS_IN_JSAMPLE;
use crate::jdct::{
    DESCALE, FLOAT_MULT_TYPE, IFAST_MULT_TYPE, IFAST_SCALE_BITS, ISLOW_MULT_TYPE, MULTIPLY16V16,
};
use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE};
use crate::jidctflt::jpeg_idct_float;
use crate::jidctfst::jpeg_idct_ifast;
use crate::jidctint::{
    jpeg_idct_3x3, jpeg_idct_5x5, jpeg_idct_6x6, jpeg_idct_7x7, jpeg_idct_9x9, jpeg_idct_10x10,
    jpeg_idct_11x11, jpeg_idct_12x12, jpeg_idct_13x13, jpeg_idct_14x14, jpeg_idct_15x15,
    jpeg_idct_16x16, jpeg_idct_islow,
};
use crate::jidctred::{jpeg_idct_1x1, jpeg_idct_2x2, jpeg_idct_4x4};
use crate::jmemmgr::alloc_small_box;
use crate::jmorecfg::{C_INT, C_POINTER, CLayout, INT16, MAX_COMPONENTS};
use crate::jpegint::{JLONG, inverse_DCT_method_ptr, jpeg_inverse_dct};
use crate::jpeglib::{DCTSIZE, DCTSIZE2, J_DCT_METHOD, JPOOL_IMAGE, j_decompress_ptr};

// upstream: src/jddctmgr.c my_idct_controller
/// Private subobject for this module.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug)]
pub struct my_idct_controller {
    /// Public fields.
    pub pub_: jpeg_inverse_dct,
    /// The IDCT method code that each multiplier table is currently set up
    /// for, or -1 if it's not yet set up. The actual multiplier tables are
    /// in `dct_table` of the per-component `comp_info` structures.
    pub cur_method: [i32; MAX_COMPONENTS],
}

/// `sizeof(my_idct_controller)` on the target, with the `inverse_DCT_12`
/// array and the three SIMD method pointers of `struct jpeg_inverse_dct`.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_my_idct_controller: usize = CLayout::structure(&[
    // struct jpeg_inverse_dct
    CLayout::structure(&[
        C_POINTER,
        C_POINTER.array(MAX_COMPONENTS),
        C_POINTER.array(MAX_COMPONENTS),
        C_POINTER.array(3),
    ]),
    C_INT.array(MAX_COMPONENTS),
])
.size;

// upstream: src/jddctmgr.c multiplier_table
/// Allocated multiplier tables: big enough for any supported variant. The
/// union of `islow_array`, `ifast_array` and `float_array`; see the module
/// documentation.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct multiplier_table {
    /// The elements of `islow_array` and `ifast_array`, and the bits of
    /// `float_array`.
    array: [i32; DCTSIZE2],
}

/// `sizeof(multiplier_table)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_multiplier_table: usize = CLayout::of::<i32>().array(DCTSIZE2).size;

const _: () = assert!(
    size_of::<ISLOW_MULT_TYPE>() == size_of::<i32>()
        && size_of::<IFAST_MULT_TYPE>() == size_of::<i32>()
        && size_of::<FLOAT_MULT_TYPE>() == size_of::<i32>()
);

impl multiplier_table {
    /// A table of zero bits, as after `jinit_inverse_dct`'s `memset`.
    #[must_use]
    pub fn zeroed() -> Self {
        multiplier_table {
            array: [0; DCTSIZE2],
        }
    }

    /// `islow_array`.
    #[must_use]
    pub fn islow_array(&self) -> &[ISLOW_MULT_TYPE; DCTSIZE2] {
        &self.array
    }

    /// `islow_array`, mutably.
    pub fn islow_array_mut(&mut self) -> &mut [ISLOW_MULT_TYPE; DCTSIZE2] {
        &mut self.array
    }

    /// `ifast_array`.
    #[must_use]
    pub fn ifast_array(&self) -> &[IFAST_MULT_TYPE; DCTSIZE2] {
        &self.array
    }

    /// `ifast_array`, mutably.
    pub fn ifast_array_mut(&mut self) -> &mut [IFAST_MULT_TYPE; DCTSIZE2] {
        &mut self.array
    }

    /// `float_array[i]`.
    #[expect(clippy::cast_sign_loss, reason = "the union's reinterpretation")]
    #[must_use]
    #[inline]
    pub fn float_array(&self, i: usize) -> FLOAT_MULT_TYPE {
        FLOAT_MULT_TYPE::from_bits(self.array[i] as u32)
    }

    /// Stores `float_array[i]`.
    #[expect(clippy::cast_possible_wrap, reason = "the union's reinterpretation")]
    #[inline]
    pub fn set_float_array(&mut self, i: usize, value: FLOAT_MULT_TYPE) {
        self.array[i] = value.to_bits() as i32;
    }
}

/// `DCTSIZE` as the `int` that `DCT_scaled_size` is compared with.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "DCTSIZE is 8"
)]
const DCTSIZE_INT: i32 = DCTSIZE as i32;

/// A `J_DCT_METHOD` as the `int` that `my_idct_controller` stores.
#[expect(clippy::cast_possible_wrap, reason = "the enumerators are small")]
const fn method_code(method: J_DCT_METHOD) -> i32 {
    method.0 as i32
}

/// The method codes, for matching.
const JDCT_ISLOW: i32 = method_code(J_DCT_METHOD::JDCT_ISLOW);
const JDCT_IFAST: i32 = method_code(J_DCT_METHOD::JDCT_IFAST);
const JDCT_FLOAT: i32 = method_code(J_DCT_METHOD::JDCT_FLOAT);

/// `CONST_BITS` of `start_pass`'s AA&N multiplier tables.
const IFAST_CONST_BITS: u32 = 14;

/// `start_pass`'s AA&N scale factors for the fast integer method:
/// precomputed values scaled up by 14 bits.
#[expect(non_upper_case_globals, reason = "C variable name")]
#[rustfmt::skip]
const aanscales: [INT16; DCTSIZE2] = [
    /* precomputed values scaled up by 14 bits */
    16384, 22725, 21407, 19266, 16384, 12873,  8867,  4520,
    22725, 31521, 29692, 26722, 22725, 17855, 12299,  6270,
    21407, 29692, 27969, 25172, 21407, 16819, 11585,  5906,
    19266, 26722, 25172, 22654, 19266, 15137, 10426,  5315,
    16384, 22725, 21407, 19266, 16384, 12873,  8867,  4520,
    12873, 17855, 16819, 15137, 12873, 10114,  6967,  3552,
     8867, 12299, 11585, 10426,  8867,  6967,  4799,  2446,
     4520,  6270,  5906,  5315,  4520,  3552,  2446,  1247,
];

/// `start_pass`'s AA&N scale factors for the float method.
#[expect(non_upper_case_globals, reason = "C variable name")]
const aanscalefactor: [f64; DCTSIZE] = [
    1.0,
    1.387_039_845,
    1.306_562_965,
    1.175_875_602,
    1.0,
    0.785_694_958,
    0.541_196_100,
    0.275_899_379,
];

// upstream: src/jddctmgr.c start_pass
/// Prepares for an output pass: selects the proper IDCT routine for each
/// component and builds a matching multiplier table.
///
/// # Errors
///
/// `JERR_NOT_COMPILED` for an unknown `dct_method` and `JERR_BAD_DCTSIZE`
/// for an unsupported `DCT_scaled_size`.
///
/// # Panics
///
/// When `cinfo.idct` or `cinfo.comp_info` is absent, or a needed
/// component has no multiplier table.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
#[expect(non_snake_case, reason = "C names")]
pub fn start_pass(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    for ci in 0..cinfo.num_components as usize {
        let compptr = &cinfo.comp_info()[ci];
        /* Select the proper IDCT routine for this component's scaling */
        let (method_ptr, method): (inverse_DCT_method_ptr, i32) = match compptr.DCT_scaled_size {
            1 => (jpeg_idct_1x1, JDCT_ISLOW), /* jidctred uses islow-style table */
            2 => (jpeg_idct_2x2, JDCT_ISLOW), /* jidctred uses islow-style table */
            3 => (jpeg_idct_3x3, JDCT_ISLOW), /* jidctint uses islow-style table */
            4 => (jpeg_idct_4x4, JDCT_ISLOW), /* jidctred uses islow-style table */
            5 => (jpeg_idct_5x5, JDCT_ISLOW), /* jidctint uses islow-style table */
            6 => (jpeg_idct_6x6, JDCT_ISLOW), /* jidctint uses islow-style table */
            7 => (jpeg_idct_7x7, JDCT_ISLOW), /* jidctint uses islow-style table */
            DCTSIZE_INT => match cinfo.dct_method {
                J_DCT_METHOD::JDCT_ISLOW => (jpeg_idct_islow, JDCT_ISLOW),
                J_DCT_METHOD::JDCT_IFAST => (jpeg_idct_ifast, JDCT_IFAST),
                J_DCT_METHOD::JDCT_FLOAT => (jpeg_idct_float, JDCT_FLOAT),
                _ => return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED)),
            },
            9 => (jpeg_idct_9x9, JDCT_ISLOW), /* jidctint uses islow-style table */
            10 => (jpeg_idct_10x10, JDCT_ISLOW), /* jidctint uses islow-style table */
            11 => (jpeg_idct_11x11, JDCT_ISLOW), /* jidctint uses islow-style table */
            12 => (jpeg_idct_12x12, JDCT_ISLOW), /* jidctint uses islow-style table */
            13 => (jpeg_idct_13x13, JDCT_ISLOW), /* jidctint uses islow-style table */
            14 => (jpeg_idct_14x14, JDCT_ISLOW), /* jidctint uses islow-style table */
            15 => (jpeg_idct_15x15, JDCT_ISLOW), /* jidctint uses islow-style table */
            16 => (jpeg_idct_16x16, JDCT_ISLOW), /* jidctint uses islow-style table */
            DCT_scaled_size => {
                return Err(ERREXIT1(
                    cinfo,
                    J_MESSAGE_CODE::JERR_BAD_DCTSIZE,
                    DCT_scaled_size,
                ));
            }
        };
        let idct = cinfo.idct.as_deref_mut().expect("cinfo->idct is NULL");
        idct.pub_.inverse_DCT[ci] = Some(method_ptr);
        /* Create multiplier table from quant table.
         * However, we can skip this if the component is uninteresting
         * or if we already built the table.  Also, if no quant table
         * has yet been saved for the component, we leave the
         * multiplier table all-zero; we'll be reading zeroes from the
         * coefficient controller's buffer anyway.
         */
        let compptr = &mut cinfo
            .comp_info
            .as_deref_mut()
            .expect("cinfo->comp_info is NULL")[ci];
        if !compptr.component_needed || idct.cur_method[ci] == method {
            continue;
        }
        let Some(qtbl) = compptr.quant_table.as_deref() else {
            /* happens if no data yet for component */
            continue;
        };
        idct.cur_method[ci] = method;
        let dct_table = compptr
            .dct_table
            .as_deref_mut()
            .expect("compptr->dct_table is NULL");
        match method {
            JDCT_ISLOW => {
                /* For LL&M IDCT method, multipliers are equal to raw quantization
                 * coefficients, but are stored as ints to ensure access efficiency.
                 */
                let ismtbl = dct_table.islow_array_mut();
                for (i, entry) in ismtbl.iter_mut().enumerate() {
                    *entry = ISLOW_MULT_TYPE::from(qtbl.quantval[i]);
                }
            }
            JDCT_IFAST => {
                /* For AA&N IDCT method, multipliers are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 * For integer operation, the multiplier table is to be scaled by
                 * IFAST_SCALE_BITS.
                 */
                let ifmtbl = dct_table.ifast_array_mut();

                for i in 0..DCTSIZE2 {
                    ifmtbl[i] = ifast_multiplier(qtbl.quantval[i], aanscales[i], IFAST_CONST_BITS);
                }
            }
            JDCT_FLOAT => {
                /* For float AA&N IDCT method, multipliers are equal to quantization
                 * coefficients scaled by scalefactor[row]*scalefactor[col], where
                 *   scalefactor[0] = 1
                 *   scalefactor[k] = cos(k*PI/16) * sqrt(2)    for k=1..7
                 */

                let mut i = 0;
                for rowscale in aanscalefactor {
                    for colscale in aanscalefactor {
                        dct_table.set_float_array(
                            i,
                            float_multiplier(qtbl.quantval[i], rowscale, colscale),
                        );
                        i += 1;
                    }
                }
            }
            _ => return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_NOT_COMPILED)),
        }
    }
    Ok(())
}

/// One element of the AA&N multiplier table of `start_pass`:
/// `(IFAST_MULT_TYPE)DESCALE(MULTIPLY16V16((JLONG)quantval, (JLONG)aanscale),
/// CONST_BITS - IFAST_SCALE_BITS)`. Not an upstream item.
#[expect(non_snake_case, reason = "C macro parameter")]
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversion to IFAST_MULT_TYPE"
)]
#[inline]
fn ifast_multiplier(quantval: u16, aanscale: INT16, CONST_BITS: u32) -> IFAST_MULT_TYPE {
    DESCALE(
        MULTIPLY16V16(JLONG::from(quantval), JLONG::from(aanscale)),
        CONST_BITS - IFAST_SCALE_BITS,
    ) as IFAST_MULT_TYPE
}

/// One element of the float AA&N multiplier table of `start_pass`, in C's
/// `double` arithmetic. Not an upstream item.
#[expect(
    clippy::cast_possible_truncation,
    reason = "C's conversion to FLOAT_MULT_TYPE"
)]
#[inline]
fn float_multiplier(quantval: u16, rowscale: f64, colscale: f64) -> FLOAT_MULT_TYPE {
    (f64::from(quantval) * rowscale * colscale) as FLOAT_MULT_TYPE
}

// upstream: src/jddctmgr.c _jinit_inverse_dct
/// Initializes the IDCT manager: allocates the module and a pre-zeroed
/// multiplier table for each component.
///
/// # Errors
///
/// `JERR_BAD_PRECISION` for a data precision other than 8, and what the
/// allocations report.
///
/// # Panics
///
/// When `cinfo.comp_info` is absent.
#[expect(clippy::cast_sign_loss, reason = "C's nonnegative counts")]
pub fn jinit_inverse_dct(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    if cinfo.data_precision != BITS_IN_JSAMPLE {
        let data_precision = cinfo.data_precision;
        return Err(ERREXIT1(
            cinfo,
            J_MESSAGE_CODE::JERR_BAD_PRECISION,
            data_precision,
        ));
    }

    let idct = alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_my_idct_controller,
        my_idct_controller {
            pub_: jpeg_inverse_dct {
                start_pass,
                inverse_DCT: [None; MAX_COMPONENTS],
            },
            cur_method: [0; MAX_COMPONENTS],
        },
    )?;
    cinfo.idct = Some(idct);

    for ci in 0..cinfo.num_components as usize {
        /* Allocate and pre-zero a multiplier table for each component */
        let dct_table = alloc_small_box(
            cinfo,
            JPOOL_IMAGE,
            SIZEOF_multiplier_table,
            multiplier_table::zeroed(),
        )?;
        cinfo.comp_info_mut()[ci].dct_table = Some(dct_table);
        /* Mark multiplier table not yet set up for any method */
        cinfo.idct.as_deref_mut().expect("set above").cur_method[ci] = -1;
    }
    Ok(())
}
