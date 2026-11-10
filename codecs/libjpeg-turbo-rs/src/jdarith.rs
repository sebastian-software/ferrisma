// libjpeg-turbo 3.2.1, src/jdarith.c:
//
// /*
//  * jdarith.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Developed 1997-2015 by Guido Vollbeding.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015-2020, 2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains portable arithmetic entropy encoding routines for JPEG
//  * (implementing Recommendation ITU-T T.81 | ISO/IEC 10918-1).
//  *
//  * Both sequential and progressive modes are supported in this single module.
//  *
//  * Suspension is not currently supported in this module.
//  *
//  * NOTE: All referenced figures are from
//  * Recommendation ITU-T T.81 (1992) | ISO/IEC 10918-1:1994.
//  */
//! Translation of `src/jdarith.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 3 (Entropy decoding).
//!
//! A pointer into a statistics area is an [`st`]: the area and an offset
//! into it. The decoder takes its module out of `cinfo.entropy` for the
//! duration of each method (ADR 0005, refinements from work package 3). C's
//! `fixed_bin[1..4]` stay uninitialized and unused; the port zeroes them.

use core::ffi::c_long;

use crate::jaricom::jpeg_aritab;
use crate::jerror::{ERREXIT, ERREXIT1, ERREXIT4, ErrorExit, J_MESSAGE_CODE, WARNMS, WARNMS2};
use crate::jmemmgr::{alloc_small_box, alloc_small_vec};
use crate::jmorecfg::{C_BOOLEAN, C_INT, C_LONG, C_POINTER, CLayout, JCOEF};
use crate::jpegint::{
    JLONG, LEFT_SHIFT, MAX, MIN, jpeg_entropy_decoder, jpeg_entropy_decoder_impl,
};
use crate::jpeglib::{
    DCTSIZE2, JBLOCK, JPOOL_IMAGE, MAX_COMPS_IN_SCAN, NUM_ARITH_TBLS, call_fill_input_buffer,
    j_decompress_ptr, jpeg_decompress_struct,
};
use crate::jutils::jpeg_natural_order;

// upstream: src/jdarith.c NEG_1
const NEG_1: u32 = u32::MAX;

// upstream: src/jdarith.c arith_entropy_decoder
/// Expanded entropy decoder object for arithmetic decoding.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Debug)]
pub struct arith_entropy_decoder {
    /// Public fields.
    pub pub_: jpeg_entropy_decoder,

    /// C register, base of coding interval + input bit buffer.
    pub c: JLONG,
    /// A register, normalized size of coding interval.
    pub a: JLONG,
    /// Bit shift counter, number of bits left in the bit buffer part of C:
    /// -16 at init, 0..7 while running, -1 after an error.
    pub ct: i32,
    /// Last DC coefficient for each component.
    pub last_dc_val: [i32; MAX_COMPS_IN_SCAN],
    /// Context index for DC conditioning.
    pub dc_context: [i32; MAX_COMPS_IN_SCAN],

    /// MCUs left in this restart interval.
    pub restarts_to_go: u32,

    /* Pointers to statistics areas (these workspaces have image lifespan) */
    /// DC statistics areas.
    pub dc_stats: [Option<Vec<u8>>; NUM_ARITH_TBLS],
    /// AC statistics areas.
    pub ac_stats: [Option<Vec<u8>>; NUM_ARITH_TBLS],

    /// Statistics bin for coding with fixed probability 0.5.
    pub fixed_bin: [u8; 4],
}

/// `sizeof(arith_entropy_decoder)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_arith_entropy_decoder: usize = CLayout::structure(&[
    // struct jpeg_entropy_decoder
    CLayout::structure(&[C_POINTER.array(4), C_BOOLEAN]),
    C_LONG,
    C_LONG,
    C_INT,
    C_INT.array(MAX_COMPS_IN_SCAN),
    C_INT.array(MAX_COMPS_IN_SCAN),
    C_INT,
    C_POINTER.array(NUM_ARITH_TBLS),
    C_POINTER.array(NUM_ARITH_TBLS),
    CLayout::of::<u8>().array(4),
])
.size;

/// The statistics area that an [`st`] points into. Not an upstream item.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatsArea {
    /// `entropy->dc_stats[tbl]`.
    Dc(usize),
    /// `entropy->ac_stats[tbl]`.
    Ac(usize),
    /// `entropy->fixed_bin`.
    Fixed,
}

/// `unsigned char *st`, a pointer to a statistics bin: an area and the
/// offset of the bin in it. Not an upstream item.
#[expect(non_camel_case_types, reason = "named after the C variable")]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct st {
    /// The area.
    pub area: StatsArea,
    /// The offset of the bin in the area.
    pub offset: usize,
}

impl st {
    /// `area + offset`.
    #[must_use]
    pub const fn at(area: StatsArea, offset: usize) -> st {
        st { area, offset }
    }

    /// `st + n`.
    #[must_use]
    pub const fn plus(self, n: usize) -> st {
        st {
            area: self.area,
            offset: self.offset + n,
        }
    }
}

impl arith_entropy_decoder {
    /// `*st`, the bin that `st` points to. Not an upstream item.
    ///
    /// # Panics
    ///
    /// When the area is unallocated.
    pub fn bin(&mut self, st: st) -> &mut u8 {
        match st.area {
            StatsArea::Dc(tbl) => {
                &mut self.dc_stats[tbl].as_mut().expect("dc_stats[tbl] is NULL")[st.offset]
            }
            StatsArea::Ac(tbl) => {
                &mut self.ac_stats[tbl].as_mut().expect("ac_stats[tbl] is NULL")[st.offset]
            }
            StatsArea::Fixed => &mut self.fixed_bin[st.offset],
        }
    }
}

/* The following two definitions specify the allocation chunk size
 * for the statistics area.
 * According to sections F.1.4.4.1.3 and F.1.4.4.2, we need at least
 * 49 statistics bins for DC, and 245 statistics bins for AC coding.
 *
 * We use a compact representation with 1 byte per statistics bin,
 * thus the numbers directly represent byte sizes.
 * This 1 byte per statistics bin contains the meaning of the MPS
 * (more probable symbol) in the highest bit (mask 0x80), and the
 * index into the probability estimation state machine table
 * in the lower bits (mask 0x7F).
 */

// upstream: src/jdarith.c DC_STAT_BINS
const DC_STAT_BINS: usize = 64;
// upstream: src/jdarith.c AC_STAT_BINS
const AC_STAT_BINS: usize = 256;

/// Runs `body` with the arithmetic decoder taken out of `cinfo.entropy`,
/// and puts it back afterwards, also when `body` fails. Not an upstream
/// item.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold an arithmetic decoder.
fn with_entropy<R>(
    cinfo: j_decompress_ptr<'_, '_>,
    body: impl FnOnce(j_decompress_ptr<'_, '_>, &mut arith_entropy_decoder) -> Result<R, ErrorExit>,
) -> Result<R, ErrorExit> {
    let mut slot = cinfo.entropy.take().expect("cinfo->entropy is NULL");
    let result = match &mut *slot {
        jpeg_entropy_decoder_impl::arith_entropy_decoder(entropy) => body(cinfo, entropy),
        _ => unreachable!("cinfo->entropy is not an arith_entropy_decoder"),
    };
    cinfo.entropy = Some(slot);
    result
}

// upstream: src/jdarith.c get_byte
/// Reads the next input byte; suspension is not supported in this module.
///
/// # Errors
///
/// `JERR_CANT_SUSPEND`, or what `fill_input_buffer` reports.
fn get_byte(cinfo: j_decompress_ptr<'_, '_>) -> Result<i32, ErrorExit> {
    if cinfo.src().bytes_in_buffer == 0 && !call_fill_input_buffer(cinfo)? {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CANT_SUSPEND));
    }
    let src = cinfo.src_mut();
    src.bytes_in_buffer -= 1;
    let byte = src.buffer.bytes()[src.next_input_byte];
    src.next_input_byte += 1;
    Ok(i32::from(byte))
}

/*
 * The core arithmetic decoding routine (common in JPEG and JBIG).
 * This needs to go as fast as possible.
 * Machine-dependent optimization facilities
 * are not utilized in this portable implementation.
 * However, this code should be fairly efficient and
 * may be a good base for further optimizations anyway.
 *
 * Return value is 0 or 1 (binary decision).
 *
 * Note: I've changed the handling of the code base & bit
 * buffer register C compared to other implementations
 * based on the standards layout & procedures.
 * While it also contains both the actual base of the
 * coding interval (16 bits) and the next-bits buffer,
 * the cut-point between these two parts is floating
 * (instead of fixed) with the bit shift counter CT.
 * Thus, we also need only one (variable instead of
 * fixed size) shift for the LPS/MPS decision, and
 * we can do away with any renormalization update
 * of C (except for new data insertion, of course).
 *
 * I've also introduced a new scheme for accessing
 * the probability estimation state machine table,
 * derived from Markus Kuhn's JBIG implementation.
 */

// upstream: src/jdarith.c arith_decode
/// The core arithmetic decoding routine: decodes one binary decision with
/// the statistics bin `st`.
///
/// # Errors
///
/// What `get_byte` reports.
#[expect(clippy::cast_sign_loss, reason = "C's unsigned char conversions")]
fn arith_decode(
    cinfo: j_decompress_ptr<'_, '_>,
    e: &mut arith_entropy_decoder,
    st: st,
) -> Result<i32, ErrorExit> {
    /* Renormalization & data input per section D.2.6 */
    while e.a < 0x8000 {
        e.ct -= 1;
        if e.ct < 0 {
            /* Need to fetch next data byte */
            let mut data;
            if cinfo.unread_marker != 0 {
                data = 0; /* stuff zero data */
            } else {
                data = get_byte(cinfo)?; /* read next input byte */
                if data == 0xFF {
                    /* zero stuff or marker code */
                    loop {
                        data = get_byte(cinfo)?;
                        if data != 0xFF {
                            break; /* swallow extra 0xFF bytes */
                        }
                    }
                    if data == 0 {
                        data = 0xFF; /* discard stuffed zero byte */
                    } else {
                        /* Note: Different from the Huffman decoder, hitting
                         * a marker while processing the compressed data
                         * segment is legal in arithmetic coding.
                         * The convention is to supply zero data
                         * then until decoding is complete.
                         */
                        cinfo.unread_marker = data;
                        data = 0;
                    }
                }
            }
            e.c = (e.c << 8) | JLONG::from(data); /* insert data into C register */
            e.ct += 8;
            if e.ct < 0 {
                /* update bit shift counter */
                /* Need more initial bytes */
                e.ct += 1;
                if e.ct == 0 {
                    /* Got 2 initial bytes -> re-init A and exit loop */
                    e.a = 0x8000; /* => e->a = 0x10000L after loop exit */
                }
            }
        }
        e.a <<= 1;
    }

    /* Fetch values from our compact representation of Table D.2:
     * Qe values and probability estimation state machine
     */
    let mut sv = i32::from(*e.bin(st));
    let mut qe = jpeg_aritab[(sv & 0x7F) as usize]; /* => Qe_Value */
    let nl = (qe & 0xFF) as u8;
    qe >>= 8; /* Next_Index_LPS + Switch_MPS */
    let nm = (qe & 0xFF) as u8;
    qe >>= 8; /* Next_Index_MPS */

    /* Decode & estimation procedures per sections D.2.4 & D.2.5 */
    let mut temp = e.a - qe;
    e.a = temp;
    temp <<= e.ct;
    if e.c >= temp {
        e.c -= temp;
        /* Conditional LPS (less probable symbol) exchange */
        if e.a < qe {
            e.a = qe;
            *e.bin(st) = (sv & 0x80) as u8 ^ nm; /* Estimate_after_MPS */
        } else {
            e.a = qe;
            *e.bin(st) = (sv & 0x80) as u8 ^ nl; /* Estimate_after_LPS */
            sv ^= 0x80; /* Exchange LPS/MPS */
        }
    } else if e.a < 0x8000 {
        /* Conditional MPS (more probable symbol) exchange */
        if e.a < qe {
            *e.bin(st) = (sv & 0x80) as u8 ^ nl; /* Estimate_after_LPS */
            sv ^= 0x80; /* Exchange LPS/MPS */
        } else {
            *e.bin(st) = (sv & 0x80) as u8 ^ nm; /* Estimate_after_MPS */
        }
    }

    Ok(sv >> 7)
}

// upstream: src/jdarith.c process_restart
/// Checks for a restart marker and resynchronizes the decoder.
///
/// # Errors
///
/// `JERR_CANT_SUSPEND`, or what `read_restart_marker` reports.
#[expect(clippy::cast_sign_loss, reason = "C's integer conversions")]
fn process_restart(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut arith_entropy_decoder,
) -> Result<(), ErrorExit> {
    /* Advance past the RSTn marker */
    let read_restart_marker = cinfo.marker().pub_.read_restart_marker;
    if !read_restart_marker(cinfo)? {
        return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_CANT_SUSPEND));
    }

    /* Re-initialize statistics areas */
    for ci in 0..cinfo.comps_in_scan as usize {
        let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
        let compptr = &cinfo.comp_info()[index];
        if !cinfo.progressive_mode || (cinfo.Ss == 0 && cinfo.Ah == 0) {
            entropy.dc_stats[compptr.dc_tbl_no as usize]
                .as_mut()
                .expect("dc_stats[tbl] is NULL")
                .fill(0);
            /* Reset DC predictions to 0 */
            entropy.last_dc_val[ci] = 0;
            entropy.dc_context[ci] = 0;
        }
        if !cinfo.progressive_mode || cinfo.Ss != 0 {
            entropy.ac_stats[compptr.ac_tbl_no as usize]
                .as_mut()
                .expect("ac_stats[tbl] is NULL")
                .fill(0);
        }
    }

    /* Reset arithmetic decoding variables */
    entropy.c = 0;
    entropy.a = 0;
    entropy.ct = -16; /* force reading 2 initial bytes to fill C */

    /* Reset restart counter */
    entropy.restarts_to_go = cinfo.restart_interval;
    Ok(())
}

/// Processes a restart marker if needed, and counts the MCU against the
/// restart interval: the prologue that every MCU routine repeats. Not an
/// upstream item.
///
/// # Errors
///
/// What `process_restart` reports.
fn restart_prologue(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut arith_entropy_decoder,
) -> Result<(), ErrorExit> {
    if cinfo.restart_interval != 0 {
        if entropy.restarts_to_go == 0 {
            process_restart(cinfo, entropy)?;
        }
        entropy.restarts_to_go = entropy.restarts_to_go.wrapping_sub(1);
    }
    Ok(())
}

/// `MCU_data[blkn]`, which the progressive routines read without a `NULL`
/// check. Not an upstream item.
///
/// # Panics
///
/// When `MCU_data` is `None`.
#[expect(non_snake_case, reason = "C names")]
fn mcu_block<'a>(MCU_data: &'a mut Option<&mut [&mut JBLOCK]>, blkn: usize) -> &'a mut JBLOCK {
    &mut *MCU_data.as_deref_mut().expect("MCU_data is NULL")[blkn]
}

/// Sections F.2.4.1 and F.1.4.4.1: decodes the DC difference of component
/// `ci` with table `tbl` into `entropy.last_dc_val[ci]`; `false` after a
/// magnitude overflow, where the C routines return `TRUE` at once. Shared
/// by `decode_mcu_DC_first` and `decode_mcu`, which repeat it. Not an
/// upstream item.
///
/// # Errors
///
/// What `arith_decode` or the warning reports.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
fn decode_dc_diff(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut arith_entropy_decoder,
    ci: usize,
    tbl: usize,
) -> Result<bool, ErrorExit> {
    /* Table F.4: Point to statistics bin S0 for DC coefficient coding */
    let mut st = st::at(StatsArea::Dc(tbl), entropy.dc_context[ci] as usize);

    /* Figure F.19: Decode_DC_DIFF */
    if arith_decode(cinfo, entropy, st)? == 0 {
        entropy.dc_context[ci] = 0;
    } else {
        /* Figure F.21: Decoding nonzero value v */
        /* Figure F.22: Decoding the sign of v */
        let sign = arith_decode(cinfo, entropy, st.plus(1))?;
        st = st.plus(2);
        st = st.plus(sign as usize);
        /* Figure F.23: Decoding the magnitude category of v */
        let mut m = arith_decode(cinfo, entropy, st)?;
        if m != 0 {
            st = st::at(StatsArea::Dc(tbl), 20); /* Table F.4: X1 = 20 */
            while arith_decode(cinfo, entropy, st)? != 0 {
                m <<= 1;
                if m == 0x8000 {
                    WARNMS(cinfo, J_MESSAGE_CODE::JWRN_ARITH_BAD_CODE)?;
                    entropy.ct = -1; /* magnitude overflow */
                    return Ok(false);
                }
                st = st.plus(1);
            }
        }
        /* Section F.1.4.4.1.2: Establish dc_context conditioning category */
        if m < ((c_long::from(1_u8) << cinfo.arith_dc_L[tbl]) >> 1) as i32 {
            entropy.dc_context[ci] = 0; /* zero diff category */
        } else if m > ((c_long::from(1_u8) << cinfo.arith_dc_U[tbl]) >> 1) as i32 {
            entropy.dc_context[ci] = 12 + (sign * 4); /* large diff category */
        } else {
            entropy.dc_context[ci] = 4 + (sign * 4); /* small diff category */
        }
        let mut v = m;
        /* Figure F.24: Decoding the magnitude bit pattern of v */
        st = st.plus(14);
        loop {
            m >>= 1;
            if m == 0 {
                break;
            }
            if arith_decode(cinfo, entropy, st)? != 0 {
                v |= m;
            }
        }
        v += 1;
        if sign != 0 {
            v = -v;
        }
        entropy.last_dc_val[ci] = (entropy.last_dc_val[ci] + v) & 0xffff;
    }
    Ok(true)
}

/// Figures F.21 to F.24: decodes the nonzero AC value at `st` (its sign
/// with the fixed bin, then its magnitude), with `k` and table `tbl`
/// selecting the magnitude bins; `None` after a magnitude overflow, where
/// the C routines return `TRUE` at once. Shared by `decode_mcu_AC_first`
/// and `decode_mcu`, which repeat it. Not an upstream item.
///
/// # Errors
///
/// What `arith_decode` or the warning reports.
fn decode_ac_value(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut arith_entropy_decoder,
    mut st: st,
    k: i32,
    tbl: usize,
) -> Result<Option<i32>, ErrorExit> {
    /* Figure F.21: Decoding nonzero value v */
    /* Figure F.22: Decoding the sign of v */
    let sign = arith_decode(cinfo, entropy, st::at(StatsArea::Fixed, 0))?;
    st = st.plus(2);
    /* Figure F.23: Decoding the magnitude category of v */
    let mut m = arith_decode(cinfo, entropy, st)?;
    if m != 0 && arith_decode(cinfo, entropy, st)? != 0 {
        m <<= 1;
        st = st::at(
            StatsArea::Ac(tbl),
            if k <= i32::from(cinfo.arith_ac_K[tbl]) {
                189
            } else {
                217
            },
        );
        while arith_decode(cinfo, entropy, st)? != 0 {
            m <<= 1;
            if m == 0x8000 {
                WARNMS(cinfo, J_MESSAGE_CODE::JWRN_ARITH_BAD_CODE)?;
                entropy.ct = -1; /* magnitude overflow */
                return Ok(None);
            }
            st = st.plus(1);
        }
    }
    let mut v = m;
    /* Figure F.24: Decoding the magnitude bit pattern of v */
    st = st.plus(14);
    loop {
        m >>= 1;
        if m == 0 {
            break;
        }
        if arith_decode(cinfo, entropy, st)? != 0 {
            v |= m;
        }
    }
    v += 1;
    if sign != 0 {
        v = -v;
    }
    Ok(Some(v))
}

/*
 * Arithmetic MCU decoding.
 * Each of these routines decodes and returns one MCU's worth of
 * arithmetic-compressed coefficients.
 * The coefficients are reordered from zigzag order into natural array order,
 * but are not dequantized.
 *
 * The i'th block of the MCU is stored into the block pointed to by
 * MCU_data[i].  WE ASSUME THIS AREA IS INITIALLY ZEROED BY THE CALLER.
 */

// upstream: src/jdarith.c decode_mcu_DC_first
/// MCU decoding for DC initial scan (either spectral selection, or first
/// pass of successive approximation).
///
/// # Errors
///
/// What the decoding or `process_restart` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold an arithmetic decoder, `MCU_data` is
/// `None` where the routine needs it, or a statistics area of the scan is
/// absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(non_snake_case, reason = "C names")]
pub fn decode_mcu_DC_first(
    cinfo: j_decompress_ptr<'_, '_>,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        /* Process restart marker if needed */
        restart_prologue(cinfo, entropy)?;

        if entropy.ct == -1 {
            return Ok(true); /* if error do nothing */
        }

        /* Outer loop handles each block in the MCU */

        for blkn in 0..cinfo.blocks_in_MCU as usize {
            let ci = cinfo.MCU_membership[blkn] as usize;
            let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
            let tbl = cinfo.comp_info()[index].dc_tbl_no as usize;

            /* Sections F.2.4.1 & F.1.4.4.1: Decoding of DC coefficients */
            if !decode_dc_diff(cinfo, entropy, ci, tbl)? {
                return Ok(true);
            }

            /* Scale and output the DC coefficient (assumes jpeg_natural_order[0]=0) */
            mcu_block(&mut MCU_data, blkn)[0] =
                LEFT_SHIFT(JLONG::from(entropy.last_dc_val[ci]), cinfo.Al as u32) as JCOEF;
        }

        Ok(true)
    })
}

// upstream: src/jdarith.c decode_mcu_AC_first
/// MCU decoding for AC initial scan (either spectral selection, or first
/// pass of successive approximation).
///
/// # Errors
///
/// What the decoding or `process_restart` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold an arithmetic decoder, `MCU_data` is
/// `None` where the routine needs it, or a statistics area of the scan is
/// absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(non_snake_case, reason = "C names")]
pub fn decode_mcu_AC_first(
    cinfo: j_decompress_ptr<'_, '_>,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        /* Process restart marker if needed */
        restart_prologue(cinfo, entropy)?;

        if entropy.ct == -1 {
            return Ok(true); /* if error do nothing */
        }

        /* There is always only one block per MCU */
        let block = mcu_block(&mut MCU_data, 0);
        let index = cinfo.cur_comp_info[0].expect("cur_comp_info[0] is NULL");
        let tbl = cinfo.comp_info()[index].ac_tbl_no as usize;

        /* Sections F.2.4.2 & F.1.4.4.2: Decoding of AC coefficients */

        /* Figure F.20: Decode_AC_coefficients */
        let mut k = cinfo.Ss;
        while k <= cinfo.Se {
            let mut st = st::at(StatsArea::Ac(tbl), (3 * (k - 1)) as usize);
            if arith_decode(cinfo, entropy, st)? != 0 {
                break; /* EOB flag */
            }
            while arith_decode(cinfo, entropy, st.plus(1))? == 0 {
                st = st.plus(3);
                k += 1;
                if k > cinfo.Se {
                    WARNMS(cinfo, J_MESSAGE_CODE::JWRN_ARITH_BAD_CODE)?;
                    entropy.ct = -1; /* spectral overflow */
                    return Ok(true);
                }
            }
            let Some(v) = decode_ac_value(cinfo, entropy, st, k, tbl)? else {
                return Ok(true);
            };
            /* Scale and output coefficient in natural (dezigzagged) order */
            block[jpeg_natural_order[k as usize] as usize] = ((v as u32) << cinfo.Al) as JCOEF;
            k += 1;
        }

        Ok(true)
    })
}

// upstream: src/jdarith.c decode_mcu_DC_refine
/// MCU decoding for DC successive approximation refinement scan.
///
/// # Errors
///
/// What the decoding or `process_restart` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold an arithmetic decoder, `MCU_data` is
/// `None` where the routine needs it, or a statistics area of the scan is
/// absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(non_snake_case, reason = "C names")]
pub fn decode_mcu_DC_refine(
    cinfo: j_decompress_ptr<'_, '_>,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        /* Process restart marker if needed */
        restart_prologue(cinfo, entropy)?;

        let st = st::at(StatsArea::Fixed, 0); /* use fixed probability estimation */
        let p1 = 1 << cinfo.Al; /* 1 in the bit position being coded */

        /* Outer loop handles each block in the MCU */

        for blkn in 0..cinfo.blocks_in_MCU as usize {
            /* Encoded data is simply the next bit of the two's-complement DC value */
            if arith_decode(cinfo, entropy, st)? != 0 {
                let block = mcu_block(&mut MCU_data, blkn);
                block[0] = (i32::from(block[0]) | p1) as JCOEF;
            }
        }

        Ok(true)
    })
}

// upstream: src/jdarith.c decode_mcu_AC_refine
/// MCU decoding for AC successive approximation refinement scan.
///
/// # Errors
///
/// What the decoding or `process_restart` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold an arithmetic decoder, `MCU_data` is
/// `None` where the routine needs it, or a statistics area of the scan is
/// absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(non_snake_case, reason = "C names")]
pub fn decode_mcu_AC_refine(
    cinfo: j_decompress_ptr<'_, '_>,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        /* Process restart marker if needed */
        restart_prologue(cinfo, entropy)?;

        if entropy.ct == -1 {
            return Ok(true); /* if error do nothing */
        }

        /* There is always only one block per MCU */
        let block = mcu_block(&mut MCU_data, 0);
        let index = cinfo.cur_comp_info[0].expect("cur_comp_info[0] is NULL");
        let tbl = cinfo.comp_info()[index].ac_tbl_no as usize;

        let p1 = 1 << cinfo.Al; /* 1 in the bit position being coded */
        let m1 = (NEG_1 << cinfo.Al) as i32; /* -1 in the bit position being coded */

        /* Establish EOBx (previous stage end-of-block) index */
        let mut kex = cinfo.Se;
        while kex > 0 {
            if block[jpeg_natural_order[kex as usize] as usize] != 0 {
                break;
            }
            kex -= 1;
        }

        let mut k = cinfo.Ss;
        while k <= cinfo.Se {
            let mut st = st::at(StatsArea::Ac(tbl), (3 * (k - 1)) as usize);
            if k > kex && arith_decode(cinfo, entropy, st)? != 0 {
                break; /* EOB flag */
            }
            loop {
                let thiscoef = &mut block[jpeg_natural_order[k as usize] as usize];
                if *thiscoef != 0 {
                    /* previously nonzero coef */
                    if arith_decode(cinfo, entropy, st.plus(2))? != 0 {
                        if *thiscoef < 0 {
                            *thiscoef = thiscoef.wrapping_add(m1 as JCOEF);
                        } else {
                            *thiscoef = thiscoef.wrapping_add(p1 as JCOEF);
                        }
                    }
                    break;
                }
                if arith_decode(cinfo, entropy, st.plus(1))? != 0 {
                    /* newly nonzero coef */
                    if arith_decode(cinfo, entropy, st::at(StatsArea::Fixed, 0))? != 0 {
                        *thiscoef = m1 as JCOEF;
                    } else {
                        *thiscoef = p1 as JCOEF;
                    }
                    break;
                }
                st = st.plus(3);
                k += 1;
                if k > cinfo.Se {
                    WARNMS(cinfo, J_MESSAGE_CODE::JWRN_ARITH_BAD_CODE)?;
                    entropy.ct = -1; /* spectral overflow */
                    return Ok(true);
                }
            }
            k += 1;
        }

        Ok(true)
    })
}

// upstream: src/jdarith.c decode_mcu
/// Decodes one MCU's worth of arithmetic-compressed coefficients.
///
/// # Errors
///
/// What the decoding or `process_restart` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold an arithmetic decoder, `MCU_data` is
/// `None` where the routine needs it, or a statistics area of the scan is
/// absent.
#[expect(clippy::int_plus_one, reason = "C's loop bound")]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(non_snake_case, reason = "C names")]
pub fn decode_mcu(
    cinfo: j_decompress_ptr<'_, '_>,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        /* Process restart marker if needed */
        restart_prologue(cinfo, entropy)?;

        if entropy.ct == -1 {
            return Ok(true); /* if error do nothing */
        }

        /* Outer loop handles each block in the MCU */

        for blkn in 0..cinfo.blocks_in_MCU as usize {
            let mut block = MCU_data.as_deref_mut().map(|blocks| &mut *blocks[blkn]);
            let ci = cinfo.MCU_membership[blkn] as usize;
            let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
            let (dc_tbl_no, ac_tbl_no) = {
                let compptr = &cinfo.comp_info()[index];
                (compptr.dc_tbl_no, compptr.ac_tbl_no)
            };

            /* Sections F.2.4.1 & F.1.4.4.1: Decoding of DC coefficients */

            let tbl = dc_tbl_no as usize;
            if !decode_dc_diff(cinfo, entropy, ci, tbl)? {
                return Ok(true);
            }

            if let Some(block) = block.as_deref_mut() {
                block[0] = entropy.last_dc_val[ci] as JCOEF;
            }

            /* Sections F.2.4.2 & F.1.4.4.2: Decoding of AC coefficients */

            let tbl = ac_tbl_no as usize;

            /* Figure F.20: Decode_AC_coefficients */
            let mut k: i32 = 1;
            while k <= DCTSIZE2 as i32 - 1 {
                let mut st = st::at(StatsArea::Ac(tbl), (3 * (k - 1)) as usize);
                if arith_decode(cinfo, entropy, st)? != 0 {
                    break; /* EOB flag */
                }
                while arith_decode(cinfo, entropy, st.plus(1))? == 0 {
                    st = st.plus(3);
                    k += 1;
                    if k > DCTSIZE2 as i32 - 1 {
                        WARNMS(cinfo, J_MESSAGE_CODE::JWRN_ARITH_BAD_CODE)?;
                        entropy.ct = -1; /* spectral overflow */
                        return Ok(true);
                    }
                }
                let Some(v) = decode_ac_value(cinfo, entropy, st, k, tbl)? else {
                    return Ok(true);
                };
                if let Some(block) = block.as_deref_mut() {
                    block[jpeg_natural_order[k as usize] as usize] = v as JCOEF;
                }
                k += 1;
            }
        }

        Ok(true)
    })
}

/// `cinfo->coef_bits`. Not an upstream item.
///
/// # Panics
///
/// When the table is absent.
fn coef_bits<'a>(cinfo: &'a mut jpeg_decompress_struct<'_>) -> &'a mut [[i32; DCTSIZE2]] {
    cinfo
        .coef_bits
        .as_deref_mut()
        .expect("cinfo->coef_bits is NULL")
}

// upstream: src/jdarith.c start_pass
/// Initializes for an arithmetic-compressed scan.
///
/// # Errors
///
/// `JERR_BAD_PROGRESSION` or `JERR_NO_ARITH_TABLE`, or what the warnings
/// or the memory manager report.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold an arithmetic decoder, or a
/// component of the scan or `cinfo.coef_bits` is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(clippy::too_many_lines, reason = "mirrors the C function")]
#[expect(non_snake_case, reason = "C names")]
pub fn start_pass(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        if cinfo.progressive_mode {
            /* Validate progressive scan parameters */
            let mut bad = false;
            if cinfo.Ss == 0 {
                if cinfo.Se != 0 {
                    bad = true;
                }
            } else {
                /* need not check Ss/Se < 0 since they came from unsigned bytes */
                if cinfo.Se < cinfo.Ss || cinfo.Se > DCTSIZE2 as i32 - 1 {
                    bad = true;
                }
                /* AC scans may have only one component */
                if cinfo.comps_in_scan != 1 {
                    bad = true;
                }
            }
            if cinfo.Ah != 0 {
                /* Successive approximation refinement scan: must have Al = Ah-1. */
                if cinfo.Ah - 1 != cinfo.Al {
                    bad = true;
                }
            }
            if bad || cinfo.Al > 13 {
                /* need not check for < 0 */
                /* bad: */
                let (Ss, Se, Ah, Al) = (cinfo.Ss, cinfo.Se, cinfo.Ah, cinfo.Al);
                return Err(ERREXIT4(
                    cinfo,
                    J_MESSAGE_CODE::JERR_BAD_PROGRESSION,
                    Ss,
                    Se,
                    Ah,
                    Al,
                ));
            }
            /* Update progression status, and verify that scan order is legal.
             * Note that inter-scan inconsistencies are treated as warnings
             * not fatal errors ... not clear if this is right way to behave.
             */
            for ci in 0..cinfo.comps_in_scan as usize {
                let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
                let cindex = cinfo.comp_info()[index].component_index;
                let coef_bit_ptr = cindex as usize;
                let prev_coef_bit_ptr = (cindex + cinfo.num_components) as usize;
                if cinfo.Ss != 0 && coef_bits(cinfo)[coef_bit_ptr][0] < 0 {
                    /* AC without prior DC scan */
                    WARNMS2(cinfo, J_MESSAGE_CODE::JWRN_BOGUS_PROGRESSION, cindex, 0)?;
                }
                for coefi in MIN(cinfo.Ss, 1)..=MAX(cinfo.Se, 9) {
                    let coefi = coefi as usize;
                    if cinfo.input_scan_number > 1 {
                        let bits = coef_bits(cinfo);
                        bits[prev_coef_bit_ptr][coefi] = bits[coef_bit_ptr][coefi];
                    } else {
                        coef_bits(cinfo)[prev_coef_bit_ptr][coefi] = 0;
                    }
                }
                for coefi in cinfo.Ss..=cinfo.Se {
                    let bit = coef_bits(cinfo)[coef_bit_ptr][coefi as usize];
                    let expected = if bit < 0 { 0 } else { bit };
                    if cinfo.Ah != expected {
                        WARNMS2(cinfo, J_MESSAGE_CODE::JWRN_BOGUS_PROGRESSION, cindex, coefi)?;
                    }
                    let Al = cinfo.Al;
                    coef_bits(cinfo)[coef_bit_ptr][coefi as usize] = Al;
                }
            }
            /* Select MCU decoding routine */
            if cinfo.Ah == 0 {
                if cinfo.Ss == 0 {
                    entropy.pub_.decode_mcu = Some(decode_mcu_DC_first);
                } else {
                    entropy.pub_.decode_mcu = Some(decode_mcu_AC_first);
                }
            } else if cinfo.Ss == 0 {
                entropy.pub_.decode_mcu = Some(decode_mcu_DC_refine);
            } else {
                entropy.pub_.decode_mcu = Some(decode_mcu_AC_refine);
            }
        } else {
            /* Check that the scan parameters Ss, Se, Ah/Al are OK for sequential JPEG.
             * This ought to be an error condition, but we make it a warning.
             */
            if cinfo.Ss != 0 || cinfo.Se != DCTSIZE2 as i32 - 1 || cinfo.Ah != 0 || cinfo.Al != 0 {
                WARNMS(cinfo, J_MESSAGE_CODE::JWRN_NOT_SEQUENTIAL)?;
            }
            /* Select MCU decoding routine */
            entropy.pub_.decode_mcu = Some(decode_mcu);
        }

        /* Allocate & initialize requested statistics areas */
        for ci in 0..cinfo.comps_in_scan as usize {
            let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
            let (dc_tbl_no, ac_tbl_no) = {
                let compptr = &cinfo.comp_info()[index];
                (compptr.dc_tbl_no, compptr.ac_tbl_no)
            };
            if !cinfo.progressive_mode || (cinfo.Ss == 0 && cinfo.Ah == 0) {
                let tbl = dc_tbl_no;
                if tbl < 0 || tbl >= NUM_ARITH_TBLS as i32 {
                    return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_NO_ARITH_TABLE, tbl));
                }
                let tbl = tbl as usize;
                if entropy.dc_stats[tbl].is_none() {
                    entropy.dc_stats[tbl] = Some(alloc_small_vec(
                        cinfo,
                        JPOOL_IMAGE,
                        DC_STAT_BINS,
                        DC_STAT_BINS,
                        0,
                    )?);
                }
                entropy.dc_stats[tbl]
                    .as_mut()
                    .expect("allocated above")
                    .fill(0);
                /* Initialize DC predictions to 0 */
                entropy.last_dc_val[ci] = 0;
                entropy.dc_context[ci] = 0;
            }
            if !cinfo.progressive_mode || cinfo.Ss != 0 {
                let tbl = ac_tbl_no;
                if tbl < 0 || tbl >= NUM_ARITH_TBLS as i32 {
                    return Err(ERREXIT1(cinfo, J_MESSAGE_CODE::JERR_NO_ARITH_TABLE, tbl));
                }
                let tbl = tbl as usize;
                if entropy.ac_stats[tbl].is_none() {
                    entropy.ac_stats[tbl] = Some(alloc_small_vec(
                        cinfo,
                        JPOOL_IMAGE,
                        AC_STAT_BINS,
                        AC_STAT_BINS,
                        0,
                    )?);
                }
                entropy.ac_stats[tbl]
                    .as_mut()
                    .expect("allocated above")
                    .fill(0);
            }
        }

        /* Initialize arithmetic decoding variables */
        entropy.c = 0;
        entropy.a = 0;
        entropy.ct = -16; /* force reading 2 initial bytes to fill C */
        entropy.pub_.insufficient_data = false;

        /* Initialize restart counter */
        entropy.restarts_to_go = cinfo.restart_interval;
        Ok(())
    })
}

// upstream: src/jdarith.c jinit_arith_decoder
/// Module initialization routine for arithmetic entropy decoding.
///
/// # Errors
///
/// What the memory manager reports.
#[expect(clippy::cast_sign_loss, reason = "C's integer conversions")]
pub fn jinit_arith_decoder(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    let entropy = arith_entropy_decoder {
        pub_: jpeg_entropy_decoder {
            start_pass,
            decode_mcu: None,
            insufficient_data: false,
        },
        c: 0,
        a: 0,
        ct: 0,
        last_dc_val: [0; MAX_COMPS_IN_SCAN],
        dc_context: [0; MAX_COMPS_IN_SCAN],
        restarts_to_go: 0,
        /* Mark tables unallocated */
        dc_stats: [const { None }; NUM_ARITH_TBLS],
        ac_stats: [const { None }; NUM_ARITH_TBLS],
        /* Initialize index for fixed probability estimation */
        fixed_bin: [113, 0, 0, 0],
    };
    cinfo.entropy = Some(alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_arith_entropy_decoder,
        jpeg_entropy_decoder_impl::arith_entropy_decoder(entropy),
    )?);

    if cinfo.progressive_mode {
        /* Create progression status table */
        let num_components = cinfo.num_components as usize;
        let mut coef_bits = alloc_small_vec(
            cinfo,
            JPOOL_IMAGE,
            num_components * 2 * DCTSIZE2 * core::mem::size_of::<i32>(),
            num_components * 2,
            [0; DCTSIZE2],
        )?;
        for row in &mut coef_bits[..num_components] {
            *row = [-1; DCTSIZE2];
        }
        cinfo.coef_bits = Some(coef_bits);
    }
    Ok(())
}
