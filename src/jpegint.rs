// libjpeg-turbo 3.2.1, src/jpegint.h:
//
// /*
//  * jpegint.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Modified 1997-2009 by Guido Vollbeding.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015-2017, 2019, 2021-2022, 2024-2026, D. R. Commander.
//  * Copyright (C) 2015, Google, Inc.
//  * Copyright (C) 2021, Alex Richardson.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file provides common declarations for the various JPEG modules.
//  * These declarations are considered internal to the JPEG library; most
//  * applications using the library shouldn't need to include this file.
//  */
//! Translation of `src/jpegint.h` from libjpeg-turbo 3.2.1.
//!
//! Work package: 1 (Common infrastructure).
//!
//! The module interfaces are added by the work package that translates the
//! module, together with its private struct and its slot in
//! `jpeg_decompress_struct` (ADR 0005, "Module interfaces arrive with their
//! modules"). Work package 2 adds `jpeg_input_controller` and
//! `jpeg_marker_reader`, the data fields of `jpeg_decomp_master` (its methods
//! arrive with `jdmaster.c` in work package 6), and the part of
//! `jpeg_d_coef_controller` that `jdinput.c` calls (ADR 0005, decisions of
//! 2026-11-07); work package 3 completes `jpeg_entropy_decoder`, and work
//! package 4 completes `jpeg_d_coef_controller` and adds
//! `jpeg_inverse_dct`. The compression interfaces, the `CSTATE_*` values, `JDIFF` and
//! the `jinit_*` declarations of untranslated modules are out of scope (ADR
//! 0001). `MALLOC` has no Rust form: storage comes from Rust's allocator
//! (ADR 0005, "Owned storage"). `SHIFT_TEMPS` declares nothing without
//! `RIGHT_SHIFT_IS_UNSIGNED`.

use core::ffi::{c_long, c_ulong};

use crate::jerror::ErrorExit;
use crate::jmorecfg::{JDIMENSION, MAX_COMPONENTS};
use crate::jpeglib::{
    J_COLOR_SPACE, JBLOCK, JSAMPROWS_MUT, j_decompress_ptr, jpeg_component_info,
    jpeg_decompress_struct, jvirt_barray_ptr,
};

// upstream: src/jpegint.h J_BUF_MODE
/// Operating modes for buffer controllers.
#[expect(non_camel_case_types, reason = "C type name")]
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct J_BUF_MODE(pub u32);

impl J_BUF_MODE {
    /// Plain stripwise operation.
    pub const JBUF_PASS_THRU: J_BUF_MODE = J_BUF_MODE(0);
    /// Run source subobject only, save output.
    pub const JBUF_SAVE_SOURCE: J_BUF_MODE = J_BUF_MODE(1);
    /// Run dest subobject only, using saved data.
    pub const JBUF_CRANK_DEST: J_BUF_MODE = J_BUF_MODE(2);
    /// Run both subobjects, save output.
    pub const JBUF_SAVE_AND_PASS: J_BUF_MODE = J_BUF_MODE(3);
}

// upstream: src/jpegint.h DSTATE_START
/// After `create_decompress`.
pub const DSTATE_START: i32 = 200;
// upstream: src/jpegint.h DSTATE_INHEADER
/// Reading header markers, no SOS yet.
pub const DSTATE_INHEADER: i32 = 201;
// upstream: src/jpegint.h DSTATE_READY
/// Found SOS, ready for `start_decompress`.
pub const DSTATE_READY: i32 = 202;
// upstream: src/jpegint.h DSTATE_PRELOAD
/// Reading multiscan file in `start_decompress`.
pub const DSTATE_PRELOAD: i32 = 203;
// upstream: src/jpegint.h DSTATE_PRESCAN
/// Performing dummy pass for 2-pass quant.
pub const DSTATE_PRESCAN: i32 = 204;
// upstream: src/jpegint.h DSTATE_SCANNING
/// `start_decompress` done, `read_scanlines` OK.
pub const DSTATE_SCANNING: i32 = 205;
// upstream: src/jpegint.h DSTATE_RAW_OK
/// `start_decompress` done, `read_raw_data` OK.
pub const DSTATE_RAW_OK: i32 = 206;
// upstream: src/jpegint.h DSTATE_BUFIMAGE
/// Expecting `jpeg_start_output`.
pub const DSTATE_BUFIMAGE: i32 = 207;
// upstream: src/jpegint.h DSTATE_BUFPOST
/// Looking for SOS/EOI in `jpeg_finish_output`.
pub const DSTATE_BUFPOST: i32 = 208;
// upstream: src/jpegint.h DSTATE_RDCOEFS
/// Reading file in `jpeg_read_coefficients`.
pub const DSTATE_RDCOEFS: i32 = 209;
// upstream: src/jpegint.h DSTATE_STOPPING
/// Looking for EOI in `jpeg_finish_decompress`.
pub const DSTATE_STOPPING: i32 = 210;

// upstream: src/jpegint.h JLONG
/// `long` in upstream (ADR 0005).
pub type JLONG = c_long;

// upstream: src/jpegint.h JUINTPTR
/// `__UINTPTR_TYPE__`, an unsigned integer as wide as a pointer.
pub type JUINTPTR = usize;

// upstream: src/jpegint.h IsExtRGB
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn IsExtRGB(cs: J_COLOR_SPACE) -> bool {
    cs == J_COLOR_SPACE::JCS_RGB
        || (cs >= J_COLOR_SPACE::JCS_EXT_RGB && cs <= J_COLOR_SPACE::JCS_EXT_ARGB)
}

// upstream: src/jpegint.h LEFT_SHIFT
/// Left shift through `unsigned long`, so that a negative operand shifts
/// without signed overflow.
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_sign_loss,
    clippy::cast_possible_wrap,
    reason = "the C casts to unsigned long and back to JLONG"
)]
#[inline]
#[must_use]
pub fn LEFT_SHIFT(a: JLONG, b: u32) -> JLONG {
    ((a as c_ulong) << b) as JLONG
}

// upstream: src/jpegint.h MAX
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn MAX<T: PartialOrd>(a: T, b: T) -> T {
    if a > b { a } else { b }
}

// upstream: src/jpegint.h MIN
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn MIN<T: PartialOrd>(a: T, b: T) -> T {
    if a < b { a } else { b }
}

// upstream: src/jpegint.h RIGHT_SHIFT
/// A signed right shift of a `JLONG`: arithmetic, since
/// `RIGHT_SHIFT_IS_UNSIGNED` is undefined on both hosts (ADR 0005).
#[expect(non_snake_case, reason = "C macro name")]
#[inline]
#[must_use]
pub fn RIGHT_SHIFT(x: JLONG, shft: u32) -> JLONG {
    x >> shft
}

// upstream: src/jpegint.h jpeg_decomp_master
/// Master control module: its state variables. The methods
/// `prepare_for_output_pass` and `finish_output_pass` arrive with
/// `jdmaster.c` (work package 6), and `WITH_PROFILE` is off.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct jpeg_decomp_master {
    /// True during 1st pass for 2-pass quant.
    pub is_dummy_pass: bool,
    /// True if decompressing a lossless image.
    pub lossless: bool,
    /// Partial decompression: first iMCU column.
    pub first_iMCU_col: JDIMENSION,
    /// Partial decompression: last iMCU column.
    pub last_iMCU_col: JDIMENSION,
    /// Partial decompression: first MCU column of each component.
    pub first_MCU_col: [JDIMENSION; MAX_COMPONENTS],
    /// Partial decompression: last MCU column of each component.
    pub last_MCU_col: [JDIMENSION; MAX_COMPONENTS],
    /// Reuse the upsampler's buffers.
    pub jinit_upsampler_no_alloc: bool,
    /// Last iMCU row that was successfully decoded.
    pub last_good_iMCU_row: JDIMENSION,
    /// Tail of the list of saved markers: the position of the last entry of
    /// `cinfo.marker_list`, or `None` where C's pointer is null.
    pub marker_list_end: Option<usize>,
    /// SIMD-specific variable.
    pub simd_support: u32,
    /// SIMD-specific variable.
    pub simd_huffman: u32,
    /// The data precision of the JPEG image.
    pub jpeg_data_precision: i32,
}

// upstream: src/jpegint.h jpeg_input_controller
/// Input control module.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug)]
pub struct jpeg_input_controller {
    /// Reads markers or compressed data; `JPEG_SUSPENDED`, `JPEG_REACHED_SOS`,
    /// `JPEG_REACHED_EOI`, `JPEG_ROW_COMPLETED` or `JPEG_SCAN_COMPLETED`.
    pub consume_input: fn(j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit>,
    /// Resets state to begin a fresh datastream.
    pub reset_input_controller: fn(j_decompress_ptr<'_, '_>),
    /// Initializes the input modules to read a scan of compressed data.
    pub start_input_pass: fn(j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit>,
    /// Finishes up after inputting a compressed-data scan.
    pub finish_input_pass: fn(j_decompress_ptr<'_, '_>),
    /// True if file has multiple scans.
    pub has_multiple_scans: bool,
    /// True when EOI has been consumed.
    pub eoi_reached: bool,
}

// upstream: src/jpegint.h jpeg_marker_reader
/// Marker reading and parsing.
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Copy, Debug)]
pub struct jpeg_marker_reader {
    /// Resets marker processing state to begin a fresh datastream.
    pub reset_marker_reader: fn(j_decompress_ptr<'_, '_>),
    /// Reads markers until SOS or EOI: `JPEG_SUSPENDED`, `JPEG_REACHED_SOS` or
    /// `JPEG_REACHED_EOI`.
    pub read_markers: fn(j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit>,
    /// Reads a restart marker, for the entropy decoder; `false` suspends.
    pub read_restart_marker: fn(j_decompress_ptr<'_, '_>) -> Result<bool, ErrorExit>,
    /// Found SOI?
    pub saw_SOI: bool,
    /// Found SOF?
    pub saw_SOF: bool,
    /// Next restart number expected (0-7).
    pub next_restart_num: i32,
    /// Number of bytes skipped looking for a marker.
    pub discarded_bytes: u32,
}

// upstream: src/jpegint.h jpeg_entropy_decoder
/// Entropy decoding. `decode_mcus` and `process_restart` serve lossless mode
/// and are not translated (ADR 0001).
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug)]
pub struct jpeg_entropy_decoder {
    /// Prepares for a scan.
    pub start_pass: fn(j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit>,
    /// Decodes one MCU into the blocks of `MCU_data` (or skips it when that is
    /// `None`, C's `NULL`); `false` suspends. `None` until `start_pass`
    /// selects the routine, where the progressive and arithmetic decoders
    /// leave C's pointer unset.
    #[expect(clippy::type_complexity, reason = "C's method pointer type")]
    pub decode_mcu:
        Option<fn(j_decompress_ptr<'_, '_>, Option<&mut [&mut JBLOCK]>) -> Result<bool, ErrorExit>>,
    /// Set `true` after emitting the warning that data ran out; shared
    /// between the baseline and progressive decoders.
    pub insufficient_data: bool,
}

// upstream: src/jpegint.h jpeg_d_coef_controller
/// Coefficient buffer control. `decompress_data_12` and `decompress_data_16`
/// serve other precisions and lossless mode and are absent (ADR 0005,
/// "Methods are function pointers").
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Copy, Debug)]
pub struct jpeg_d_coef_controller {
    /// Initializes for an input pass.
    pub start_input_pass: fn(j_decompress_ptr<'_, '_>),
    /// Consumes compressed data; returns like `consume_input`.
    pub consume_data: fn(j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit>,
    /// Initializes for an output pass. Returns a `Result` where C returns
    /// nothing, because block smoothing allocates its latch on the first
    /// pass.
    pub start_output_pass: fn(j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit>,
    /// Decompresses one iMCU row into `output_buf`, which holds a row list
    /// per component (C's `JSAMPIMAGE`): `JPEG_ROW_COMPLETED`,
    /// `JPEG_SCAN_COMPLETED` or `JPEG_SUSPENDED`.
    pub decompress_data:
        fn(j_decompress_ptr<'_, '_>, &mut [JSAMPROWS_MUT<'_, '_>]) -> Result<i32, ErrorExit>,

    /// Counts MCUs processed in the current row; used by lossless mode's
    /// difference buffer controller, which is not translated.
    pub MCU_ctr: JDIMENSION,
    /// Counts MCU rows within the iMCU row; lossless mode only.
    pub MCU_vert_offset: i32,
    /// Number of such rows needed; lossless mode only.
    pub MCU_rows_per_iMCU_row: i32,

    /// The coefficient virtual arrays, one per component, or `None`: a copy
    /// of `my_coef_controller::whole_image`, which C links (ADR 0005,
    /// "Pointers into a module's data").
    pub coef_arrays: Option<[Option<jvirt_barray_ptr>; MAX_COMPONENTS]>,
}

// upstream: src/jpegint.h inverse_DCT_method_ptr
/// An inverse DCT routine: dequantizes and transforms `coef_block` into the
/// rows of `output_buf` from column `output_col` on. It only reads the
/// object, so it takes `cinfo` shared, beside `compptr`, which points into
/// `cinfo.comp_info` (ADR 0005, "Disjoint borrows"); `JCOEFPTR` is a block
/// the routine only reads.
#[expect(non_camel_case_types, reason = "C type name")]
pub type inverse_DCT_method_ptr = fn(
    &jpeg_decompress_struct<'_>,
    &jpeg_component_info,
    &JBLOCK,
    JSAMPROWS_MUT<'_, '_>,
    JDIMENSION,
);

// upstream: src/jpegint.h jpeg_inverse_dct
/// Inverse DCT (also performs dequantization). `inverse_DCT_12` and the
/// SIMD slots are absent (ADR 0005, "Methods are function pointers").
#[expect(non_camel_case_types, reason = "C type name")]
#[expect(non_snake_case, reason = "C field names")]
#[derive(Clone, Copy, Debug)]
pub struct jpeg_inverse_dct {
    /// Prepares for an output pass.
    pub start_pass: fn(j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit>,
    /// The IDCT routine of each component; `None` until `start_pass` selects
    /// it, where C leaves the pointer unset.
    pub inverse_DCT: [Option<inverse_DCT_method_ptr>; MAX_COMPONENTS],
}

/// The implementations of the `entropy` slot (ADR 0005, "Module objects").
/// Not an upstream item.
#[expect(non_camel_case_types, reason = "named after the C struct")]
#[derive(Debug)]
#[expect(
    clippy::large_enum_variant,
    reason = "the slot boxes the decoder, as C allocates it"
)]
pub enum jpeg_entropy_decoder_impl {
    /// `jdhuff.c`: sequential Huffman decoding.
    huff_entropy_decoder(crate::jdhuff::huff_entropy_decoder),
    /// `jdphuff.c`: progressive Huffman decoding.
    phuff_entropy_decoder(crate::jdphuff::phuff_entropy_decoder),
    /// `jdarith.c`: arithmetic decoding.
    arith_entropy_decoder(crate::jdarith::arith_entropy_decoder),
}

impl jpeg_entropy_decoder_impl {
    /// The public part of whichever implementation the slot holds.
    #[must_use]
    pub fn pub_(&self) -> &jpeg_entropy_decoder {
        match self {
            jpeg_entropy_decoder_impl::huff_entropy_decoder(entropy) => &entropy.pub_,
            jpeg_entropy_decoder_impl::phuff_entropy_decoder(entropy) => &entropy.pub_,
            jpeg_entropy_decoder_impl::arith_entropy_decoder(entropy) => &entropy.pub_,
        }
    }

    /// The public part, mutably.
    pub fn pub_mut(&mut self) -> &mut jpeg_entropy_decoder {
        match self {
            jpeg_entropy_decoder_impl::huff_entropy_decoder(entropy) => &mut entropy.pub_,
            jpeg_entropy_decoder_impl::phuff_entropy_decoder(entropy) => &mut entropy.pub_,
            jpeg_entropy_decoder_impl::arith_entropy_decoder(entropy) => &mut entropy.pub_,
        }
    }
}
