// libjpeg-turbo 3.2.1, src/jdhuff.h:
//
// /*
//  * jdhuff.h
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2010-2011, 2015-2016, 2021, D. R. Commander.
//  * Copyright (C) 2018, Matthias Räncker.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains declarations for Huffman entropy decoding routines
//  * that are shared between the sequential decoder (jdhuff.c), the progressive
//  * decoder (jdphuff.c), and the lossless decoder (jdlhuff.c).  No other modules
//  * need to see these.
//  */
// libjpeg-turbo 3.2.1, src/jdhuff.c:
//
// /*
//  * jdhuff.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1991-1997, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2009-2011, 2016, 2018-2019, 2022, D. R. Commander.
//  * Copyright (C) 2018, Matthias Räncker.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains Huffman entropy decoding routines.
//  *
//  * Much of the complexity here has to do with supporting input suspension.
//  * If the data source module demands suspension, we want to be able to back
//  * up to the start of the current MCU.  To do this, we copy state variables
//  * into local working storage, and update them back to the permanent
//  * storage only upon successful completion of an MCU.
//  *
//  * NOTE: All referenced figures are from
//  * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
//  */
//! Translation of `src/jdhuff.h` and `src/jdhuff.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 3 (Entropy decoding).
//!
//! The bit-reading macros of `jdhuff.h` are functions over the locals that
//! `BITREAD_STATE_VARS` declares, passed by reference; a macro's `action`
//! is its return value. `bitread_working_state` has no back link to
//! `cinfo`, which the functions receive beside it, and its
//! `next_input_byte` is an offset into the source buffer. A derived table
//! keeps a copy of the public Huffman table it was derived from, where C
//! keeps a link: tables change only between scans, and every scan derives
//! its tables again in `start_pass`. The tables a block uses are numbers
//! into the derived-table arrays. The decoder takes its module out of
//! `cinfo.entropy` for the duration of `start_pass` and `decode_mcu`, and
//! keeps it across the source's `fill_input_buffer`, the error manager's
//! `emit_message` and the marker reader's `read_restart_marker`, none of
//! which reaches the entropy decoder; `jpeg_fill_bit_buffer`, which C lets
//! reach `cinfo->entropy`, receives the public part of the decoder (ADR
//! 0005, refinements from work package 3). Upstream exempts the DC prediction
//! sums from the undefined-behavior sanitizer because corrupt files
//! overflow them; they wrap here, as the compiled C does.

use core::ffi::c_char;

use crate::jerror::{ERREXIT, ERREXIT1, ErrorExit, J_MESSAGE_CODE, WARNMS};
use crate::jmemmgr::alloc_small_box;
use crate::jmorecfg::{C_BOOLEAN, C_INT, C_LONG, C_POINTER, C_SIZE_T, CLayout, JCOEF, JOCTET};
use crate::jpegint::{JLONG, jpeg_entropy_decoder, jpeg_entropy_decoder_impl};
use crate::jpeglib::{
    D_MAX_BLOCKS_IN_MCU, DCTSIZE2, JBLOCK, JHUFF_TBL, JPOOL_IMAGE, MAX_COMPS_IN_SCAN,
    NUM_HUFF_TBLS, call_fill_input_buffer, j_decompress_ptr, jpeg_decompress_struct,
};
use crate::jstdhuff::std_huff_tables;
use crate::jutils::jpeg_natural_order;

/* Derived data constructed for each Huffman table */

// upstream: src/jdhuff.h HUFF_LOOKAHEAD
/// Number of bits of lookahead.
pub const HUFF_LOOKAHEAD: i32 = 8;

// upstream: src/jdhuff.h d_derived_tbl
/// Derived data constructed for each Huffman table.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Debug)]
pub struct d_derived_tbl {
    /* Basic tables: (element [0] of each array is unused) */
    /// Largest code of length k (-1 if none); `maxcode[17]` is a sentinel
    /// to ensure `jpeg_huff_decode` terminates.
    pub maxcode: [JLONG; 18],
    /// `huffval[]` offset for codes of length k: `valoffset[k]` is the
    /// `huffval[]` index of the first symbol of code length k, less the
    /// smallest code of length k, so given a code of length k, the
    /// corresponding symbol is `huffval[code + valoffset[k]]`.
    pub valoffset: [JLONG; 18],
    /// The public Huffman table (needed only in `jpeg_huff_decode`): a copy
    /// where C keeps a link.
    pub pub_: JHUFF_TBL,
    /// Lookahead table, indexed by the next `HUFF_LOOKAHEAD` bits of the
    /// input data stream. The lower 8 bits of each entry contain the number
    /// of bits in the corresponding Huffman code, or `HUFF_LOOKAHEAD + 1` if
    /// too long; the next 8 bits contain the symbol.
    pub lookup: [i32; 1 << HUFF_LOOKAHEAD],
}

/// `sizeof(d_derived_tbl)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_d_derived_tbl: usize = CLayout::structure(&[
    C_LONG.array(18),
    C_LONG.array(18),
    C_POINTER,
    C_INT.array(1 << HUFF_LOOKAHEAD),
])
.size;

// upstream: src/jdhuff.c jpeg_make_d_derived_tbl
/// Computes the derived values for a Huffman table, allocating the
/// workspace in `pdtbl` if it is empty. This routine also performs some
/// validation checks on the table. Also used by `jdphuff.c`.
///
/// # Errors
///
/// `JERR_NO_HUFF_TABLE` or `JERR_BAD_HUFF_TABLE`, or what the memory
/// manager reports.
///
/// # Panics
///
/// When `cinfo.master` is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(clippy::too_many_lines, reason = "mirrors the C function")]
#[expect(non_snake_case, reason = "C names")]
pub fn jpeg_make_d_derived_tbl(
    cinfo: j_decompress_ptr<'_, '_>,
    isDC: bool,
    tblno: i32,
    pdtbl: &mut Option<Box<d_derived_tbl>>,
) -> Result<(), ErrorExit> {
    let mut huffsize: [c_char; 257] = [0; 257];
    let mut huffcode: [u32; 257] = [0; 257];

    /* Note that huffsize[] and huffcode[] are filled in code-length order,
     * paralleling the order of the symbols themselves in htbl->huffval[].
     */

    /* Find the input Huffman table */
    if tblno < 0 || tblno >= NUM_HUFF_TBLS as i32 {
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_NO_HUFF_TABLE, tblno));
    }
    let present = if isDC {
        cinfo.dc_huff_tbl_ptrs[tblno as usize].is_some()
    } else {
        cinfo.ac_huff_tbl_ptrs[tblno as usize].is_some()
    };
    if !present {
        return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_NO_HUFF_TABLE, tblno));
    }

    /* Allocate a workspace if we haven't already done so. */
    if pdtbl.is_none() {
        *pdtbl = Some(alloc_small_box(
            cinfo,
            JPOOL_IMAGE,
            SIZEOF_d_derived_tbl,
            d_derived_tbl {
                maxcode: [0; 18],
                valoffset: [0; 18],
                pub_: JHUFF_TBL {
                    bits: [0; 17],
                    huffval: [0; 256],
                    sent_table: false,
                },
                lookup: [0; 1 << HUFF_LOOKAHEAD],
            },
        )?);
    }
    let dtbl = pdtbl.as_deref_mut().expect("allocated above");
    let htbl = if isDC {
        cinfo.dc_huff_tbl_ptrs[tblno as usize].as_deref()
    } else {
        cinfo.ac_huff_tbl_ptrs[tblno as usize].as_deref()
    }
    .expect("checked above");
    dtbl.pub_.clone_from(htbl); /* fill in back link */
    let htbl = &dtbl.pub_;

    /* Figure C.1: make table of Huffman code length for each symbol */

    let mut p: i32 = 0;
    for l in 1..=16 {
        let mut i = i32::from(htbl.bits[l]);
        if i < 0 || p + i > 256 {
            /* protect against table overrun */
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_HUFF_TABLE));
        }
        while i != 0 {
            i -= 1;
            huffsize[p as usize] = l as c_char;
            p += 1;
        }
    }
    huffsize[p as usize] = 0;
    let numsymbols = p;

    /* Figure C.2: generate the codes themselves */
    /* We also validate that the counts represent a legal Huffman code tree. */

    let mut code: u32 = 0;
    let mut si = i32::from(huffsize[0]);
    p = 0;
    while huffsize[p as usize] != 0 {
        while i32::from(huffsize[p as usize]) == si {
            huffcode[p as usize] = code;
            p += 1;
            code = code.wrapping_add(1);
        }
        /* code is now 1 more than the last code used for codelength si; but
         * it must still fit in si bits, since no code is allowed to be all ones.
         */
        if JLONG::from(code) >= JLONG::from(1_i32) << si {
            return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_HUFF_TABLE));
        }
        code <<= 1;
        si += 1;
    }

    /* Figure F.15: generate decoding tables for bit-sequential decoding */

    p = 0;
    for l in 1..=16 {
        if htbl.bits[l] != 0 {
            /* valoffset[l] = huffval[] index of 1st symbol of code length l,
             * minus the minimum code of length l
             */
            dtbl.valoffset[l] = JLONG::from(p) - JLONG::from(huffcode[p as usize]);
            p += i32::from(htbl.bits[l]);
            dtbl.maxcode[l] = JLONG::from(huffcode[(p - 1) as usize]); /* maximum code of length l */
        } else {
            dtbl.maxcode[l] = -1; /* -1 if no codes of this length */
        }
    }
    dtbl.valoffset[17] = 0;
    dtbl.maxcode[17] = 0xFFFFF; /* ensures jpeg_huff_decode terminates */

    /* Compute lookahead tables to speed up decoding.
     * First we set all the table entries to 0, indicating "too long";
     * then we iterate through the Huffman codes that are short enough and
     * fill in all the entries that correspond to bit sequences starting
     * with that code.
     */

    dtbl.lookup.fill((HUFF_LOOKAHEAD + 1) << HUFF_LOOKAHEAD);

    p = 0;
    for l in 1..=HUFF_LOOKAHEAD {
        let mut i = 1;
        while i <= i32::from(htbl.bits[l as usize]) {
            /* l = current code's length, p = its index in huffcode[] & huffval[]. */
            /* Generate left-justified code followed by all possible bit sequences */
            let mut lookbits = (huffcode[p as usize] << (HUFF_LOOKAHEAD - l)) as i32;
            let mut ctr = 1 << (HUFF_LOOKAHEAD - l);
            while ctr > 0 {
                dtbl.lookup[lookbits as usize] =
                    (l << HUFF_LOOKAHEAD) | i32::from(htbl.huffval[p as usize]);
                lookbits += 1;
                ctr -= 1;
            }
            i += 1;
            p += 1;
        }
    }

    /* Validate symbols as being reasonable.
     * For AC tables, we make no check, but accept all byte values 0..255.
     * For DC tables, we require the symbols to be in range 0..15 in lossy mode
     * and 0..16 in lossless mode.  (Tighter bounds could be applied depending on
     * the data depth and mode, but this is sufficient to ensure safe decoding.)
     */
    if isDC {
        let limit = if cinfo.master().pub_.lossless { 16 } else { 15 };
        for i in 0..numsymbols {
            let sym = i32::from(htbl.huffval[i as usize]);
            if sym < 0 || sym > limit {
                return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_HUFF_TABLE));
            }
        }
    }
    Ok(())
}

/*
 * Fetching the next N bits from the input stream is a time-critical operation
 * for the Huffman decoders.  We implement it with a combination of inline
 * macros and out-of-line subroutines.  Note that N (the number of bits
 * demanded at one time) never exceeds 15 for JPEG use.
 *
 * We read source bytes into get_buffer and dole out bits as needed.
 * If get_buffer already contains enough bits, they are fetched in-line
 * by the macros CHECK_BIT_BUFFER and GET_BITS.  When there aren't enough
 * bits, jpeg_fill_bit_buffer is called; it will attempt to fill get_buffer
 * as full as possible (not just to the number of bits needed; this
 * prefetching reduces the overhead cost of calling jpeg_fill_bit_buffer).
 * Note that jpeg_fill_bit_buffer may return FALSE to indicate suspension.
 * On TRUE return, jpeg_fill_bit_buffer guarantees that get_buffer contains
 * at least the requested number of bits --- dummy zeroes are inserted if
 * necessary.
 */

// upstream: src/jdhuff.h bit_buf_type
/// Type of the bit-extraction buffer: `size_t` where `SIZEOF_SIZE_T == 8`,
/// otherwise `unsigned long`, which has the width of `usize` on the
/// remaining targets.
#[expect(non_camel_case_types, reason = "C type name")]
pub type bit_buf_type = usize;

// upstream: src/jdhuff.h BIT_BUF_SIZE
/// Size of the bit buffer in bits.
#[cfg(target_pointer_width = "64")]
pub const BIT_BUF_SIZE: i32 = 64;
// upstream: src/jdhuff.h BIT_BUF_SIZE
/// Size of the bit buffer in bits.
#[cfg(not(target_pointer_width = "64"))]
pub const BIT_BUF_SIZE: i32 = 32;

// upstream: src/jdhuff.h bitread_perm_state
/// Bitreading state saved across MCUs.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug, Default)]
pub struct bitread_perm_state {
    /// Current bit-extraction buffer.
    pub get_buffer: bit_buf_type,
    /// Number of unused bits in it.
    pub bits_left: i32,
}

// upstream: src/jdhuff.h bitread_working_state
/// Bitreading working state within an MCU. The data source location is a
/// copy, rather than the original, in case of suspension. C's back link to
/// `cinfo` is the functions' `cinfo` parameter.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug)]
pub struct bitread_working_state {
    /// Offset of the next byte to read in the source buffer.
    pub next_input_byte: usize,
    /// Number of bytes remaining in the source buffer.
    pub bytes_in_buffer: usize,
    /// Current bit-extraction buffer.
    pub get_buffer: bit_buf_type,
    /// Number of unused bits in it.
    pub bits_left: i32,
}

// upstream: src/jdhuff.h BITREAD_LOAD_STATE
/// Loads the bitread locals: returns `get_buffer`, `bits_left` and
/// `br_state`.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
#[must_use]
pub fn BITREAD_LOAD_STATE(
    cinfop: &jpeg_decompress_struct<'_>,
    permstate: &bitread_perm_state,
) -> (bit_buf_type, i32, bitread_working_state) {
    let br_state = bitread_working_state {
        next_input_byte: cinfop.src().next_input_byte,
        bytes_in_buffer: cinfop.src().bytes_in_buffer,
        get_buffer: 0,
        bits_left: 0,
    };
    (permstate.get_buffer, permstate.bits_left, br_state)
}

// upstream: src/jdhuff.h BITREAD_SAVE_STATE
/// Saves the bitread locals.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
pub fn BITREAD_SAVE_STATE(
    cinfop: &mut jpeg_decompress_struct<'_>,
    permstate: &mut bitread_perm_state,
    get_buffer: bit_buf_type,
    bits_left: i32,
    br_state: &bitread_working_state,
) {
    cinfop.src_mut().next_input_byte = br_state.next_input_byte;
    cinfop.src_mut().bytes_in_buffer = br_state.bytes_in_buffer;
    permstate.get_buffer = get_buffer;
    permstate.bits_left = bits_left;
}

// upstream: src/jdhuff.h CHECK_BIT_BUFFER
/// Ensures there are `nbits` bits in `get_buffer`; `false` where the macro
/// takes its action (suspension).
///
/// # Errors
///
/// What `jpeg_fill_bit_buffer` reports.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
pub fn CHECK_BIT_BUFFER(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut jpeg_entropy_decoder,
    state: &mut bitread_working_state,
    get_buffer: &mut bit_buf_type,
    bits_left: &mut i32,
    nbits: i32,
) -> Result<bool, ErrorExit> {
    if *bits_left < nbits {
        if !jpeg_fill_bit_buffer(cinfo, entropy, state, *get_buffer, *bits_left, nbits)? {
            return Ok(false);
        }
        *get_buffer = state.get_buffer;
        *bits_left = state.bits_left;
    }
    Ok(true)
}

// upstream: src/jdhuff.h GET_BITS
/// Fetches the next `nbits` bits.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "C's (int) cast"
)]
pub fn GET_BITS(get_buffer: bit_buf_type, bits_left: &mut i32, nbits: i32) -> i32 {
    *bits_left -= nbits;
    ((get_buffer >> *bits_left) as i32) & ((1 << nbits) - 1)
}

// upstream: src/jdhuff.h PEEK_BITS
/// Fetches the next `nbits` bits without removing them from the buffer.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "C's (int) cast"
)]
#[must_use]
pub fn PEEK_BITS(get_buffer: bit_buf_type, bits_left: i32, nbits: i32) -> i32 {
    ((get_buffer >> (bits_left - nbits)) as i32) & ((1 << nbits) - 1)
}

// upstream: src/jdhuff.h DROP_BITS
/// Discards the next `nbits` bits.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
pub fn DROP_BITS(bits_left: &mut i32, nbits: i32) {
    *bits_left -= nbits;
}

// upstream: src/jdhuff.c MIN_GET_BITS
/// The number of bits `jpeg_fill_bit_buffer` attempts to load
/// (`SLOW_SHIFT_32` is not defined).
const MIN_GET_BITS: i32 = BIT_BUF_SIZE - 7;

/// Reads the byte at `next_input_byte` and advances past it.
fn read_byte(cinfo: &jpeg_decompress_struct<'_>, next_input_byte: &mut usize) -> i32 {
    let c = i32::from(cinfo.src().buffer.bytes()[*next_input_byte]);
    *next_input_byte += 1;
    c
}

// upstream: src/jdhuff.c jpeg_fill_bit_buffer
/// Loads up the bit buffer to a depth of at least `nbits`; `false`
/// suspends. Current values of `get_buffer` and `bits_left` are passed as
/// parameters, but are returned in the corresponding fields of the state
/// struct. `entropy` is the public part of the decoder, which C reaches as
/// `cinfo->entropy`. Shared with `jdphuff.c`.
///
/// # Errors
///
/// What `fill_input_buffer` or the warning reports.
#[expect(clippy::cast_sign_loss, reason = "C's integer conversions")]
pub fn jpeg_fill_bit_buffer(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut jpeg_entropy_decoder,
    state: &mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: i32,
    nbits: i32,
) -> Result<bool, ErrorExit> {
    /* Copy heavily used state fields into locals (hopefully registers) */
    let mut next_input_byte = state.next_input_byte;
    let mut bytes_in_buffer = state.bytes_in_buffer;

    /* Attempt to load at least MIN_GET_BITS bits into get_buffer. */
    /* (It is assumed that no request will be for more than that many bits.) */
    /* We fail to do so only if we hit a marker or are forced to suspend. */

    let mut no_more_bytes = cinfo.unread_marker != 0; /* cannot advance past a marker */
    if !no_more_bytes {
        while bits_left < MIN_GET_BITS {
            /* Attempt to read a byte */
            if bytes_in_buffer == 0 {
                if !call_fill_input_buffer(cinfo)? {
                    return Ok(false);
                }
                next_input_byte = cinfo.src().next_input_byte;
                bytes_in_buffer = cinfo.src().bytes_in_buffer;
            }
            bytes_in_buffer -= 1;
            let mut c = read_byte(cinfo, &mut next_input_byte);

            /* If it's 0xFF, check and discard stuffed zero byte */
            if c == 0xFF {
                /* Loop here to discard any padding FF's on terminating marker,
                 * so that we can save a valid unread_marker value.  NOTE: we will
                 * accept multiple FF's followed by a 0 as meaning a single FF data
                 * byte.  This data pattern is not valid according to the standard.
                 */
                loop {
                    if bytes_in_buffer == 0 {
                        if !call_fill_input_buffer(cinfo)? {
                            return Ok(false);
                        }
                        next_input_byte = cinfo.src().next_input_byte;
                        bytes_in_buffer = cinfo.src().bytes_in_buffer;
                    }
                    bytes_in_buffer -= 1;
                    c = read_byte(cinfo, &mut next_input_byte);
                    if c != 0xFF {
                        break;
                    }
                }

                if c == 0 {
                    /* Found FF/00, which represents an FF data byte */
                    c = 0xFF;
                } else {
                    /* Oops, it's actually a marker indicating end of compressed data.
                     * Save the marker code for later use.
                     * Fine point: it might appear that we should save the marker into
                     * bitread working state, not straight into permanent state.  But
                     * once we have hit a marker, we cannot need to suspend within the
                     * current MCU, because we will read no more bytes from the data
                     * source.  So it is OK to update permanent state right away.
                     */
                    cinfo.unread_marker = c;
                    /* See if we need to insert some fake zero bits. */
                    no_more_bytes = true;
                    break;
                }
            }

            /* OK, load c into get_buffer */
            get_buffer = (get_buffer << 8) | c as bit_buf_type;
            bits_left += 8;
        } /* end while */
    }
    if no_more_bytes {
        /* We get here if we've read the marker that terminates the compressed
         * data segment.  There should be enough bits in the buffer register
         * to satisfy the request; if so, no problem.
         */
        if nbits > bits_left {
            /* Uh-oh.  Report corrupted data to user and stuff zeroes into
             * the data stream, so that we can produce some kind of image.
             * We use a nonvolatile flag to ensure that only one warning message
             * appears per data segment.
             */
            if !entropy.insufficient_data {
                WARNMS(cinfo, J_MESSAGE_CODE::JWRN_HIT_MARKER)?;
                entropy.insufficient_data = true;
            }
            /* Fill the buffer with zero bits */
            get_buffer <<= MIN_GET_BITS - bits_left;
            bits_left = MIN_GET_BITS;
        }
    }

    /* Unload the local registers */
    state.next_input_byte = next_input_byte;
    state.bytes_in_buffer = bytes_in_buffer;
    state.get_buffer = get_buffer;
    state.bits_left = bits_left;

    Ok(true)
}

/* Macro version of the above, which performs much better but does not
handle markers.  We have to hand off any blocks with markers to the
slower routines. */

// upstream: src/jdhuff.c GET_BYTE
/// Loads the byte at `buffer` (an offset into `bytes`, the source buffer).
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
fn GET_BYTE(
    bytes: &[JOCTET],
    buffer: &mut usize,
    get_buffer: &mut bit_buf_type,
    bits_left: &mut i32,
    unread_marker: &mut i32,
) {
    let c0 = bytes[*buffer];
    *buffer += 1;
    let c1 = bytes[*buffer];
    /* Pre-execute most common case */
    *get_buffer = (*get_buffer << 8) | bit_buf_type::from(c0);
    *bits_left += 8;
    if c0 == 0xFF {
        /* Pre-execute case of FF/00, which represents an FF data byte */
        *buffer += 1;
        if c1 != 0 {
            /* Oops, it's actually a marker indicating end of compressed data. */
            *unread_marker = i32::from(c1);
            /* Back out pre-execution and fill the buffer with zero bits */
            *buffer -= 2;
            *get_buffer &= !0xFF;
        }
    }
}

// upstream: src/jdhuff.c FILL_BIT_BUFFER_FAST
/// Pre-fetches 48 bytes, because the holding register is 64-bit.
#[cfg(target_pointer_width = "64")]
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
fn FILL_BIT_BUFFER_FAST(
    bytes: &[JOCTET],
    buffer: &mut usize,
    get_buffer: &mut bit_buf_type,
    bits_left: &mut i32,
    unread_marker: &mut i32,
) {
    if *bits_left <= 16 {
        for _ in 0..6 {
            GET_BYTE(bytes, buffer, get_buffer, bits_left, unread_marker);
        }
    }
}

// upstream: src/jdhuff.c FILL_BIT_BUFFER_FAST
/// Pre-fetches 16 bytes, because the holding register is 32-bit.
#[cfg(not(target_pointer_width = "64"))]
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
fn FILL_BIT_BUFFER_FAST(
    bytes: &[JOCTET],
    buffer: &mut usize,
    get_buffer: &mut bit_buf_type,
    bits_left: &mut i32,
    unread_marker: &mut i32,
) {
    if *bits_left <= 16 {
        for _ in 0..2 {
            GET_BYTE(bytes, buffer, get_buffer, bits_left, unread_marker);
        }
    }
}

/*
 * Code for extracting next Huffman-coded symbol from input bit stream.
 * Again, this is time-critical and we make the main paths be macros.
 *
 * We use a lookahead table to process codes of up to HUFF_LOOKAHEAD bits
 * without looping.  Usually, more than 95% of the Huffman codes will be 8
 * or fewer bits long.  The few overlength codes are handled with a loop,
 * which need not be inline code.
 *
 * Notes about the HUFF_DECODE macro:
 * 1. Near the end of the data segment, we may fail to get enough bits
 *    for a lookahead.  In that case, we do it the hard way.
 * 2. If the lookahead table contains no entry, the next code must be
 *    more than HUFF_LOOKAHEAD bits long.
 * 3. jpeg_huff_decode returns -1 if forced to suspend.
 */

// upstream: src/jdhuff.h HUFF_DECODE
/// Decodes the next Huffman-coded symbol; `None` where the macro takes its
/// `failaction` (suspension). The `slowlabel` jump is the shared
/// `jpeg_huff_decode` branch.
///
/// # Errors
///
/// What `jpeg_fill_bit_buffer` or `jpeg_huff_decode` reports.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
#[expect(clippy::cast_sign_loss, reason = "C's array index")]
pub fn HUFF_DECODE(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut jpeg_entropy_decoder,
    state: &mut bitread_working_state,
    get_buffer: &mut bit_buf_type,
    bits_left: &mut i32,
    htbl: &d_derived_tbl,
) -> Result<Option<i32>, ErrorExit> {
    let mut slow = false;
    if *bits_left < HUFF_LOOKAHEAD {
        if !jpeg_fill_bit_buffer(cinfo, entropy, state, *get_buffer, *bits_left, 0)? {
            return Ok(None);
        }
        *get_buffer = state.get_buffer;
        *bits_left = state.bits_left;
        if *bits_left < HUFF_LOOKAHEAD {
            slow = true;
        }
    }
    let nb = if slow {
        1
    } else {
        let look = PEEK_BITS(*get_buffer, *bits_left, HUFF_LOOKAHEAD);
        let nb = htbl.lookup[look as usize] >> HUFF_LOOKAHEAD;
        if nb <= HUFF_LOOKAHEAD {
            DROP_BITS(bits_left, nb);
            return Ok(Some(
                htbl.lookup[look as usize] & ((1 << HUFF_LOOKAHEAD) - 1),
            ));
        }
        nb
    };
    /* slowlabel: */
    let result = jpeg_huff_decode(cinfo, entropy, state, *get_buffer, *bits_left, htbl, nb)?;
    if result < 0 {
        return Ok(None);
    }
    *get_buffer = state.get_buffer;
    *bits_left = state.bits_left;
    Ok(Some(result))
}

// upstream: src/jdhuff.h HUFF_DECODE_FAST
/// Decodes the next Huffman-coded symbol from a buffer known to hold
/// enough bytes, setting `nb` to its code length.
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
fn HUFF_DECODE_FAST(
    bytes: &[JOCTET],
    buffer: &mut usize,
    get_buffer: &mut bit_buf_type,
    bits_left: &mut i32,
    unread_marker: &mut i32,
    nb: &mut i32,
    htbl: &d_derived_tbl,
) -> i32 {
    FILL_BIT_BUFFER_FAST(bytes, buffer, get_buffer, bits_left, unread_marker);
    let mut s = PEEK_BITS(*get_buffer, *bits_left, HUFF_LOOKAHEAD);
    s = htbl.lookup[s as usize];
    *nb = s >> HUFF_LOOKAHEAD;
    /* Pre-execute the common case of nb <= HUFF_LOOKAHEAD */
    DROP_BITS(bits_left, *nb);
    s &= (1 << HUFF_LOOKAHEAD) - 1;
    if *nb > HUFF_LOOKAHEAD {
        /* Equivalent of jpeg_huff_decode() */
        /* Don't use GET_BITS() here because we don't want to modify bits_left */
        s = ((*get_buffer >> *bits_left) & ((1 << *nb) - 1)) as i32;
        while JLONG::from(s) > htbl.maxcode[*nb as usize] {
            s <<= 1;
            s |= GET_BITS(*get_buffer, bits_left, 1);
            *nb += 1;
        }
        if *nb > 16 {
            s = 0;
        } else {
            s = i32::from(
                htbl.pub_.huffval
                    [((JLONG::from(s) + htbl.valoffset[*nb as usize]) as i32 & 0xFF) as usize],
            );
        }
    }
    s
}

// upstream: src/jdhuff.c jpeg_huff_decode
/// Out-of-line code for Huffman code decoding: returns the symbol, or -1
/// if forced to suspend. See [`jpeg_fill_bit_buffer`] for the parameters.
///
/// # Errors
///
/// What `jpeg_fill_bit_buffer` or the warning reports.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
pub fn jpeg_huff_decode(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut jpeg_entropy_decoder,
    state: &mut bitread_working_state,
    mut get_buffer: bit_buf_type,
    mut bits_left: i32,
    htbl: &d_derived_tbl,
    min_bits: i32,
) -> Result<i32, ErrorExit> {
    let mut l = min_bits;

    /* HUFF_DECODE has determined that the code is at least min_bits */
    /* bits long, so fetch that many bits in one swoop. */

    if !CHECK_BIT_BUFFER(cinfo, entropy, state, &mut get_buffer, &mut bits_left, l)? {
        return Ok(-1);
    }
    let mut code = JLONG::from(GET_BITS(get_buffer, &mut bits_left, l));

    /* Collect the rest of the Huffman code one bit at a time. */
    /* This is per Figure F.16. */

    while code > htbl.maxcode[l as usize] {
        code <<= 1;
        if !CHECK_BIT_BUFFER(cinfo, entropy, state, &mut get_buffer, &mut bits_left, 1)? {
            return Ok(-1);
        }
        code |= JLONG::from(GET_BITS(get_buffer, &mut bits_left, 1));
        l += 1;
    }

    /* Unload the local registers */
    state.get_buffer = get_buffer;
    state.bits_left = bits_left;

    /* With garbage input we may reach the sentinel value l = 17. */

    if l > 16 {
        WARNMS(cinfo, J_MESSAGE_CODE::JWRN_HUFF_BAD_CODE)?;
        return Ok(0); /* fake a zero as the safest result */
    }

    Ok(i32::from(
        htbl.pub_.huffval[(code + htbl.valoffset[l as usize]) as i32 as usize],
    ))
}

// upstream: src/jdhuff.c NEG_1
const NEG_1: u32 = u32::MAX;

// upstream: src/jdhuff.c HUFF_EXTEND
/// Figure F.12: extends the sign bit (the `AVOID_TABLES` form, which
/// upstream defines).
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's unsigned arithmetic"
)]
#[must_use]
pub fn HUFF_EXTEND(x: i32, s: i32) -> i32 {
    (x as u32).wrapping_add((((x - (1 << (s - 1))) >> 31) as u32) & (NEG_1 << s).wrapping_add(1))
        as i32
}

// upstream: src/jdhuff.c savable_state
/// The state that changes within an MCU, but must not be updated
/// permanently until we complete the MCU.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug, Default)]
pub struct savable_state {
    /// Last DC coefficient for each component.
    pub last_dc_val: [i32; MAX_COMPS_IN_SCAN],
}

// upstream: src/jdhuff.c huff_entropy_decoder
/// Expanded entropy decoder object for Huffman decoding.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Debug)]
pub struct huff_entropy_decoder {
    /// Public fields.
    pub pub_: jpeg_entropy_decoder,

    /* These fields are loaded into local variables at start of each MCU.
     * In case of suspension, we exit WITHOUT updating them.
     */
    /// Bit buffer at start of MCU.
    pub bitstate: bitread_perm_state,
    /// Other state at start of MCU.
    pub saved: savable_state,

    /* These fields are NOT loaded into local working state. */
    /// MCUs left in this restart interval.
    pub restarts_to_go: u32,

    /* Pointers to derived tables (these workspaces have image lifespan) */
    /// Derived DC tables.
    pub dc_derived_tbls: [Option<Box<d_derived_tbl>>; NUM_HUFF_TBLS],
    /// Derived AC tables.
    pub ac_derived_tbls: [Option<Box<d_derived_tbl>>; NUM_HUFF_TBLS],

    /* Precalculated info set up by start_pass for use in decode_mcu: */
    /// The number of the derived DC table to use for each block within an
    /// MCU.
    pub dc_cur_tbls: [usize; D_MAX_BLOCKS_IN_MCU],
    /// The number of the derived AC table to use for each block.
    pub ac_cur_tbls: [usize; D_MAX_BLOCKS_IN_MCU],
    /// Whether we care about the DC coefficient values for each block.
    pub dc_needed: [bool; D_MAX_BLOCKS_IN_MCU],
    /// Whether we care about the AC coefficient values for each block.
    pub ac_needed: [bool; D_MAX_BLOCKS_IN_MCU],
}

/// `sizeof(huff_entropy_decoder)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_huff_entropy_decoder: usize = CLayout::structure(&[
    // struct jpeg_entropy_decoder
    CLayout::structure(&[C_POINTER.array(4), C_BOOLEAN]),
    // bitread_perm_state
    CLayout::structure(&[C_SIZE_T, C_INT]),
    // savable_state
    CLayout::structure(&[C_INT.array(MAX_COMPS_IN_SCAN)]),
    C_INT,
    C_POINTER.array(NUM_HUFF_TBLS),
    C_POINTER.array(NUM_HUFF_TBLS),
    C_POINTER.array(D_MAX_BLOCKS_IN_MCU),
    C_POINTER.array(D_MAX_BLOCKS_IN_MCU),
    C_BOOLEAN.array(D_MAX_BLOCKS_IN_MCU),
    C_BOOLEAN.array(D_MAX_BLOCKS_IN_MCU),
])
.size;

/// Runs `body` with the Huffman decoder taken out of `cinfo.entropy`, and
/// puts it back afterwards, also when `body` fails. Not an upstream item.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a Huffman decoder.
fn with_entropy<R>(
    cinfo: j_decompress_ptr<'_, '_>,
    body: impl FnOnce(j_decompress_ptr<'_, '_>, &mut huff_entropy_decoder) -> Result<R, ErrorExit>,
) -> Result<R, ErrorExit> {
    let mut slot = cinfo.entropy.take().expect("cinfo->entropy is NULL");
    let result = match &mut *slot {
        jpeg_entropy_decoder_impl::huff_entropy_decoder(entropy) => body(cinfo, entropy),
        _ => unreachable!("cinfo->entropy is not a huff_entropy_decoder"),
    };
    cinfo.entropy = Some(slot);
    result
}

// upstream: src/jdhuff.c start_pass_huff_decoder
/// Initializes for a Huffman-compressed scan.
///
/// # Errors
///
/// What the warning or `jpeg_make_d_derived_tbl` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a Huffman decoder, or a component
/// of the scan is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "C's integer conversions"
)]
pub fn start_pass_huff_decoder(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        /* Check that the scan parameters Ss, Se, Ah/Al are OK for sequential JPEG.
         * This ought to be an error condition, but we make it a warning because
         * there are some baseline files out there with all zeroes in these bytes.
         */
        if cinfo.Ss != 0 || cinfo.Se != DCTSIZE2 as i32 - 1 || cinfo.Ah != 0 || cinfo.Al != 0 {
            WARNMS(cinfo, J_MESSAGE_CODE::JWRN_NOT_SEQUENTIAL)?;
        }

        for ci in 0..cinfo.comps_in_scan.unsigned_abs() as usize {
            let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
            let compptr = &cinfo.comp_info()[index];
            let dctbl = compptr.dc_tbl_no;
            let actbl = compptr.ac_tbl_no;
            /* Compute derived values for Huffman tables */
            /* We may do this more than once for a table, but it's not expensive */
            // C forms the slot pointer before jpeg_make_d_derived_tbl rejects
            // an out-of-range number; the remainder only keeps that slot in
            // bounds until then.
            jpeg_make_d_derived_tbl(
                cinfo,
                true,
                dctbl,
                &mut entropy.dc_derived_tbls[dctbl.unsigned_abs() as usize % NUM_HUFF_TBLS],
            )?;
            jpeg_make_d_derived_tbl(
                cinfo,
                false,
                actbl,
                &mut entropy.ac_derived_tbls[actbl.unsigned_abs() as usize % NUM_HUFF_TBLS],
            )?;
            /* Initialize DC predictions to 0 */
            entropy.saved.last_dc_val[ci] = 0;
        }

        /* Precalculate decoding info for each block in an MCU of this scan */
        for blkn in 0..cinfo.blocks_in_MCU.unsigned_abs() as usize {
            let ci = cinfo.MCU_membership[blkn].unsigned_abs() as usize;
            let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
            let compptr = &cinfo.comp_info()[index];
            /* Precalculate which table to use for each block */
            entropy.dc_cur_tbls[blkn] = compptr.dc_tbl_no.unsigned_abs() as usize;
            entropy.ac_cur_tbls[blkn] = compptr.ac_tbl_no.unsigned_abs() as usize;
            /* Decide whether we really care about the coefficient values */
            if compptr.component_needed {
                entropy.dc_needed[blkn] = true;
                /* we don't need the ACs if producing a 1/8th-size image */
                entropy.ac_needed[blkn] = compptr.DCT_scaled_size > 1;
            } else {
                entropy.dc_needed[blkn] = false;
                entropy.ac_needed[blkn] = false;
            }
        }

        /* Initialize bitread state variables */
        entropy.bitstate.bits_left = 0;
        entropy.bitstate.get_buffer = 0; /* unnecessary, but keeps Purify quiet */
        entropy.pub_.insufficient_data = false;

        /* Initialize restart counter */
        entropy.restarts_to_go = cinfo.restart_interval;
        Ok(())
    })
}

// upstream: src/jdhuff.c process_restart
/// Checks for a restart marker and resynchronizes the decoder; `false` if
/// it must suspend.
///
/// # Errors
///
/// What `read_restart_marker` reports.
#[expect(clippy::cast_sign_loss, reason = "C's unsigned arithmetic")]
fn process_restart(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut huff_entropy_decoder,
) -> Result<bool, ErrorExit> {
    /* Throw away any unused bits remaining in bit buffer; */
    /* include any full bytes in next_marker's count of discarded bytes */
    let marker = &mut cinfo.marker_mut().pub_;
    marker.discarded_bytes = marker
        .discarded_bytes
        .wrapping_add((entropy.bitstate.bits_left / 8) as u32);
    entropy.bitstate.bits_left = 0;

    /* Advance past the RSTn marker */
    let read_restart_marker = cinfo.marker().pub_.read_restart_marker;
    if !read_restart_marker(cinfo)? {
        return Ok(false);
    }

    /* Re-initialize DC predictions to 0 */
    for ci in 0..cinfo.comps_in_scan.unsigned_abs() as usize {
        entropy.saved.last_dc_val[ci] = 0;
    }

    /* Reset restart counter */
    entropy.restarts_to_go = cinfo.restart_interval;

    /* Reset out-of-data flag, unless read_restart_marker left us smack up
     * against a marker.  In that case we will end up treating the next data
     * segment as empty, and we can avoid producing bogus output pixels by
     * leaving the flag set.
     */
    if cinfo.unread_marker == 0 {
        entropy.pub_.insufficient_data = false;
    }

    Ok(true)
}

/// `MCU_data ? MCU_data[blkn] : NULL`. Not an upstream item.
#[expect(non_snake_case, reason = "C names")]
fn mcu_block<'a>(
    MCU_data: &'a mut Option<&mut [&mut JBLOCK]>,
    blkn: usize,
) -> Option<&'a mut JBLOCK> {
    match MCU_data {
        Some(blocks) => Some(&mut *blocks[blkn]),
        None => None,
    }
}

// upstream: src/jdhuff.c decode_mcu_slow
/// Decodes one MCU with the marker-aware bit reader; `false` suspends.
///
/// # Errors
///
/// What the bit reader reports.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(clippy::too_many_lines, reason = "mirrors the C function")]
#[expect(non_snake_case, reason = "C names")]
fn decode_mcu_slow(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut huff_entropy_decoder,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    let huff_entropy_decoder {
        pub_,
        bitstate,
        saved,
        dc_derived_tbls,
        ac_derived_tbls,
        dc_cur_tbls,
        ac_cur_tbls,
        dc_needed,
        ac_needed,
        ..
    } = entropy;
    /* Outer loop handles each block in the MCU */

    /* Load up working state */
    let (mut get_buffer, mut bits_left, mut br_state) = BITREAD_LOAD_STATE(cinfo, bitstate);
    let mut state = *saved;

    for blkn in 0..cinfo.blocks_in_MCU.unsigned_abs() as usize {
        let mut block = mcu_block(&mut MCU_data, blkn);
        let dctbl = dc_derived_tbls[dc_cur_tbls[blkn]]
            .as_deref()
            .expect("derived in start_pass");
        let actbl = ac_derived_tbls[ac_cur_tbls[blkn]]
            .as_deref()
            .expect("derived in start_pass");

        /* Decode a single block's worth of coefficients */

        /* Section F.2.2.1: decode the DC coefficient difference */
        let Some(mut s) = HUFF_DECODE(
            cinfo,
            pub_,
            &mut br_state,
            &mut get_buffer,
            &mut bits_left,
            dctbl,
        )?
        else {
            return Ok(false);
        };
        if s != 0 {
            if !CHECK_BIT_BUFFER(
                cinfo,
                pub_,
                &mut br_state,
                &mut get_buffer,
                &mut bits_left,
                s,
            )? {
                return Ok(false);
            }
            let r = GET_BITS(get_buffer, &mut bits_left, s);
            s = HUFF_EXTEND(r, s);
        }

        if dc_needed[blkn] {
            /* Convert DC difference to actual value, update last_dc_val */
            let ci = cinfo.MCU_membership[blkn].unsigned_abs() as usize;
            /* Certain malformed JPEG images produce repeated DC coefficient
             * differences of 2047 or -2047, which causes state.last_dc_val[ci] to
             * grow until it overflows or underflows a 32-bit signed integer.  This
             * behavior is, to the best of our understanding, innocuous, and it is
             * unclear how to work around it without potentially affecting
             * performance.  Thus, we (hopefully temporarily) suppress UBSan integer
             * overflow errors for this function and decode_mcu_fast().
             */
            s = s.wrapping_add(state.last_dc_val[ci]);
            state.last_dc_val[ci] = s;
            if let Some(block) = block.as_deref_mut() {
                /* Output the DC coefficient (assumes jpeg_natural_order[0] = 0) */
                block[0] = s as JCOEF;
            }
        }

        if ac_needed[blkn]
            && let Some(block) = block
        {
            /* Section F.2.2.2: decode the AC coefficients */
            /* Since zeroes are skipped, output area must be cleared beforehand */
            let mut k: i32 = 1;
            while k < DCTSIZE2 as i32 {
                let Some(mut s) = HUFF_DECODE(
                    cinfo,
                    pub_,
                    &mut br_state,
                    &mut get_buffer,
                    &mut bits_left,
                    actbl,
                )?
                else {
                    return Ok(false);
                };

                let mut r = s >> 4;
                s &= 15;

                if s != 0 {
                    k += r;
                    if !CHECK_BIT_BUFFER(
                        cinfo,
                        pub_,
                        &mut br_state,
                        &mut get_buffer,
                        &mut bits_left,
                        s,
                    )? {
                        return Ok(false);
                    }
                    r = GET_BITS(get_buffer, &mut bits_left, s);
                    s = HUFF_EXTEND(r, s);
                    /* Output coefficient in natural (dezigzagged) order.
                     * Note: the extra entries in jpeg_natural_order[] will save us
                     * if k >= DCTSIZE2, which could happen if the data is corrupted.
                     */
                    block[jpeg_natural_order[k as usize] as usize] = s as JCOEF;
                } else {
                    if r != 15 {
                        break;
                    }
                    k += 15;
                }
                k += 1;
            }
        } else {
            /* Section F.2.2.2: decode the AC coefficients */
            /* In this path we just discard the values */
            let mut k: i32 = 1;
            while k < DCTSIZE2 as i32 {
                let Some(mut s) = HUFF_DECODE(
                    cinfo,
                    pub_,
                    &mut br_state,
                    &mut get_buffer,
                    &mut bits_left,
                    actbl,
                )?
                else {
                    return Ok(false);
                };

                let r = s >> 4;
                s &= 15;

                if s != 0 {
                    k += r;
                    if !CHECK_BIT_BUFFER(
                        cinfo,
                        pub_,
                        &mut br_state,
                        &mut get_buffer,
                        &mut bits_left,
                        s,
                    )? {
                        return Ok(false);
                    }
                    DROP_BITS(&mut bits_left, s);
                } else {
                    if r != 15 {
                        break;
                    }
                    k += 15;
                }
                k += 1;
            }
        }
    }

    /* Completed MCU, so update state */
    BITREAD_SAVE_STATE(cinfo, bitstate, get_buffer, bits_left, &br_state);
    *saved = state;
    Ok(true)
}

// upstream: src/jdhuff.c decode_mcu_fast
/// Decodes one MCU straight from a source buffer that holds enough bytes;
/// `false` when it meets a marker, so that the slow path redoes the MCU.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(clippy::too_many_lines, reason = "mirrors the C function")]
#[expect(clippy::needless_range_loop, reason = "C's loop over the blocks")]
#[expect(non_snake_case, reason = "C names")]
fn decode_mcu_fast(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut huff_entropy_decoder,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> bool {
    /* Outer loop handles each block in the MCU */

    /* Load up working state */
    let (mut get_buffer, mut bits_left, mut br_state) =
        BITREAD_LOAD_STATE(cinfo, &entropy.bitstate);
    let mut buffer = br_state.next_input_byte;
    let mut state = entropy.saved;

    {
        let jpeg_decompress_struct {
            src,
            unread_marker,
            blocks_in_MCU,
            MCU_membership,
            ..
        } = &mut *cinfo;
        let bytes = src.as_deref().expect("cinfo->src is NULL").buffer.bytes();

        for blkn in 0..blocks_in_MCU.unsigned_abs() as usize {
            let mut block = mcu_block(&mut MCU_data, blkn);
            let dctbl = entropy.dc_derived_tbls[entropy.dc_cur_tbls[blkn]]
                .as_deref()
                .expect("derived in start_pass");
            let actbl = entropy.ac_derived_tbls[entropy.ac_cur_tbls[blkn]]
                .as_deref()
                .expect("derived in start_pass");
            let mut l: i32 = 0;

            let mut s = HUFF_DECODE_FAST(
                bytes,
                &mut buffer,
                &mut get_buffer,
                &mut bits_left,
                unread_marker,
                &mut l,
                dctbl,
            );
            if s != 0 {
                FILL_BIT_BUFFER_FAST(
                    bytes,
                    &mut buffer,
                    &mut get_buffer,
                    &mut bits_left,
                    unread_marker,
                );
                let r = GET_BITS(get_buffer, &mut bits_left, s);
                s = HUFF_EXTEND(r, s);
            }

            if entropy.dc_needed[blkn] {
                let ci = MCU_membership[blkn].unsigned_abs() as usize;
                /* Refer to the comment in decode_mcu_slow() regarding the supression of
                 * a UBSan integer overflow error in this line of code.
                 */
                s = s.wrapping_add(state.last_dc_val[ci]);
                state.last_dc_val[ci] = s;
                if let Some(block) = block.as_deref_mut() {
                    block[0] = s as JCOEF;
                }
            }

            if entropy.ac_needed[blkn]
                && let Some(block) = block
            {
                let mut k: i32 = 1;
                while k < DCTSIZE2 as i32 {
                    let mut s = HUFF_DECODE_FAST(
                        bytes,
                        &mut buffer,
                        &mut get_buffer,
                        &mut bits_left,
                        unread_marker,
                        &mut l,
                        actbl,
                    );
                    let mut r = s >> 4;
                    s &= 15;

                    if s != 0 {
                        k += r;
                        FILL_BIT_BUFFER_FAST(
                            bytes,
                            &mut buffer,
                            &mut get_buffer,
                            &mut bits_left,
                            unread_marker,
                        );
                        r = GET_BITS(get_buffer, &mut bits_left, s);
                        s = HUFF_EXTEND(r, s);
                        block[jpeg_natural_order[k as usize] as usize] = s as JCOEF;
                    } else {
                        if r != 15 {
                            break;
                        }
                        k += 15;
                    }
                    k += 1;
                }
            } else {
                let mut k: i32 = 1;
                while k < DCTSIZE2 as i32 {
                    let mut s = HUFF_DECODE_FAST(
                        bytes,
                        &mut buffer,
                        &mut get_buffer,
                        &mut bits_left,
                        unread_marker,
                        &mut l,
                        actbl,
                    );
                    let r = s >> 4;
                    s &= 15;

                    if s != 0 {
                        k += r;
                        FILL_BIT_BUFFER_FAST(
                            bytes,
                            &mut buffer,
                            &mut get_buffer,
                            &mut bits_left,
                            unread_marker,
                        );
                        DROP_BITS(&mut bits_left, s);
                    } else {
                        if r != 15 {
                            break;
                        }
                        k += 15;
                    }
                    k += 1;
                }
            }
        }
    }

    if cinfo.unread_marker != 0 {
        cinfo.unread_marker = 0;
        return false;
    }

    br_state.bytes_in_buffer -= buffer - br_state.next_input_byte;
    br_state.next_input_byte = buffer;
    BITREAD_SAVE_STATE(
        cinfo,
        &mut entropy.bitstate,
        get_buffer,
        bits_left,
        &br_state,
    );
    entropy.saved = state;
    true
}

// upstream: src/jdhuff.c BUFSIZE
const BUFSIZE: usize = DCTSIZE2 * 8;

// upstream: src/jdhuff.c decode_mcu
/// Decodes and returns one MCU's worth of Huffman-compressed coefficients.
/// The coefficients are reordered from zigzag order into natural array
/// order, but are not dequantized.
///
/// The i'th block of the MCU is stored into the block `MCU_data[i]`, which
/// the caller has zeroed. Returns `false` if the data source requested
/// suspension. In that case no changes have been made to permanent state
/// (exception: some output coefficients may already have been assigned,
/// which is harmless, since they are assigned again on the next call).
///
/// # Errors
///
/// What the bit reader or `read_restart_marker` reports.
#[expect(clippy::cast_sign_loss, reason = "C's (size_t) cast")]
#[expect(non_snake_case, reason = "C names")]
pub fn decode_mcu(
    cinfo: j_decompress_ptr<'_, '_>,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        let mut usefast = true;

        /* Process restart marker if needed; may have to suspend */
        if cinfo.restart_interval != 0 {
            if entropy.restarts_to_go == 0 && !process_restart(cinfo, entropy)? {
                return Ok(false);
            }
            usefast = false;
        }

        if cinfo.src().bytes_in_buffer < BUFSIZE * cinfo.blocks_in_MCU as usize
            || cinfo.unread_marker != 0
        {
            usefast = false;
        }

        /* If we've run out of data, just leave the MCU set to zeroes.
         * This way, we return uniform gray for the remainder of the segment.
         */
        if !entropy.pub_.insufficient_data
            && (!usefast || !decode_mcu_fast(cinfo, entropy, MCU_data.as_deref_mut()))
        {
            /* use_slow: */
            if !decode_mcu_slow(cinfo, entropy, MCU_data)? {
                return Ok(false);
            }
        }

        /* Account for restart interval (no-op if not using restarts) */
        if cinfo.restart_interval != 0 {
            entropy.restarts_to_go = entropy.restarts_to_go.wrapping_sub(1);
        }

        Ok(true)
    })
}

// upstream: src/jdhuff.c jinit_huff_decoder
/// Module initialization routine for Huffman entropy decoding.
///
/// # Errors
///
/// What `std_huff_tables` or the memory manager reports.
pub fn jinit_huff_decoder(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    /* Motion JPEG frames typically do not include the Huffman tables if they
    are the default tables.  Thus, if the tables are not set by the time
    the Huffman decoder is initialized (usually within the body of
    jpeg_start_decompress()), we set them to default values. */
    std_huff_tables(cinfo)?;

    let entropy = huff_entropy_decoder {
        pub_: jpeg_entropy_decoder {
            start_pass: start_pass_huff_decoder,
            decode_mcu: Some(decode_mcu),
            insufficient_data: false,
        },
        bitstate: bitread_perm_state::default(),
        saved: savable_state::default(),
        restarts_to_go: 0,
        /* Mark tables unallocated */
        dc_derived_tbls: [const { None }; NUM_HUFF_TBLS],
        ac_derived_tbls: [const { None }; NUM_HUFF_TBLS],
        dc_cur_tbls: [0; D_MAX_BLOCKS_IN_MCU],
        ac_cur_tbls: [0; D_MAX_BLOCKS_IN_MCU],
        dc_needed: [false; D_MAX_BLOCKS_IN_MCU],
        ac_needed: [false; D_MAX_BLOCKS_IN_MCU],
    };
    cinfo.entropy = Some(alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_huff_entropy_decoder,
        jpeg_entropy_decoder_impl::huff_entropy_decoder(entropy),
    )?);
    Ok(())
}
