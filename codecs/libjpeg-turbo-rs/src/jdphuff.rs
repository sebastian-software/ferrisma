// libjpeg-turbo 3.2.1, src/jdphuff.c:
//
// /*
//  * jdphuff.c
//  *
//  * This file was part of the Independent JPEG Group's software:
//  * Copyright (C) 1995-1997, Thomas G. Lane.
//  * Lossless JPEG Modifications:
//  * Copyright (C) 1999, Ken Murchison.
//  * libjpeg-turbo Modifications:
//  * Copyright (C) 2015-2016, 2018-2022, D. R. Commander.
//  * For conditions of distribution and use, see the accompanying README.ijg
//  * file.
//  *
//  * This file contains Huffman entropy decoding routines for progressive JPEG.
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
//! Translation of `src/jdphuff.c` from libjpeg-turbo 3.2.1.
//!
//! Work package: 3 (Entropy decoding).
//!
//! The decoder follows the conventions of [`crate::jdhuff`], whose bit
//! reader it shares. `cinfo->coef_bits` is the vector that
//! `jinit_phuff_decoder` allocates; C leaves its second half, the previous
//! scan's progression status, uninitialized until `start_pass` writes it,
//! and the port fills it with zeros. The active AC table is a number into
//! `derived_tbls`. The `undoit` exit of `decode_mcu_AC_refine` is a
//! labeled block.

use crate::jdhuff::{
    BITREAD_LOAD_STATE, BITREAD_SAVE_STATE, CHECK_BIT_BUFFER, GET_BITS, HUFF_DECODE,
    bitread_perm_state, d_derived_tbl, jpeg_make_d_derived_tbl,
};
use crate::jerror::{ERREXIT, ERREXIT4, ErrorExit, J_MESSAGE_CODE, WARNMS, WARNMS2};
use crate::jmemmgr::{alloc_small_box, alloc_small_vec};
use crate::jmorecfg::{C_BOOLEAN, C_INT, C_POINTER, C_SIZE_T, CLayout, JCOEF};
use crate::jpegint::{
    JLONG, LEFT_SHIFT, MAX, MIN, jpeg_entropy_decoder, jpeg_entropy_decoder_impl,
};
use crate::jpeglib::{
    DCTSIZE2, JBLOCK, JPOOL_IMAGE, MAX_COMPS_IN_SCAN, NUM_HUFF_TBLS, j_decompress_ptr,
    jpeg_decompress_struct,
};
use crate::jutils::jpeg_natural_order;

// upstream: src/jdphuff.c savable_state
/// The state that changes within an MCU, but must not be updated
/// permanently until we complete the MCU.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Clone, Copy, Debug, Default)]
#[expect(non_snake_case, reason = "C names")]
pub struct savable_state {
    /// Remaining EOBs in EOBRUN.
    pub EOBRUN: u32,
    /// Last DC coefficient for each component.
    pub last_dc_val: [i32; MAX_COMPS_IN_SCAN],
}

// upstream: src/jdphuff.c phuff_entropy_decoder
/// Expanded entropy decoder object for progressive Huffman decoding.
#[expect(non_camel_case_types, reason = "C type name")]
#[derive(Debug)]
pub struct phuff_entropy_decoder {
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
    /// Derived tables.
    pub derived_tbls: [Option<Box<d_derived_tbl>>; NUM_HUFF_TBLS],

    /// The number of the active table in `derived_tbls` during an AC scan.
    pub ac_derived_tbl: usize,
}

/// `sizeof(phuff_entropy_decoder)` on the target.
#[expect(non_upper_case_globals, reason = "C type name")]
pub const SIZEOF_phuff_entropy_decoder: usize = CLayout::structure(&[
    // struct jpeg_entropy_decoder
    CLayout::structure(&[C_POINTER.array(4), C_BOOLEAN]),
    // bitread_perm_state
    CLayout::structure(&[C_SIZE_T, C_INT]),
    // savable_state
    CLayout::structure(&[C_INT, C_INT.array(MAX_COMPS_IN_SCAN)]),
    C_INT,
    C_POINTER.array(NUM_HUFF_TBLS),
    C_POINTER,
])
.size;

/// Runs `body` with the progressive decoder taken out of `cinfo.entropy`,
/// and puts it back afterwards, also when `body` fails. Not an upstream
/// item.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a progressive Huffman decoder.
fn with_entropy<R>(
    cinfo: j_decompress_ptr<'_, '_>,
    body: impl FnOnce(j_decompress_ptr<'_, '_>, &mut phuff_entropy_decoder) -> Result<R, ErrorExit>,
) -> Result<R, ErrorExit> {
    let mut slot = cinfo.entropy.take().expect("cinfo->entropy is NULL");
    let result = match &mut *slot {
        jpeg_entropy_decoder_impl::phuff_entropy_decoder(entropy) => body(cinfo, entropy),
        _ => unreachable!("cinfo->entropy is not a phuff_entropy_decoder"),
    };
    cinfo.entropy = Some(slot);
    result
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

// upstream: src/jdphuff.c start_pass_phuff_decoder
/// Initializes for a Huffman-compressed scan.
///
/// # Errors
///
/// `JERR_BAD_PROGRESSION`, or what the warnings or
/// `jpeg_make_d_derived_tbl` report.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a progressive Huffman decoder, or
/// a component of the scan or `cinfo.coef_bits` is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(clippy::too_many_lines, reason = "mirrors the C function")]
#[expect(non_snake_case, reason = "C names")]
pub fn start_pass_phuff_decoder(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        let is_DC_band = cinfo.Ss == 0;

        /* Validate scan parameters */
        let mut bad = false;
        if is_DC_band {
            if cinfo.Se != 0 {
                bad = true;
            }
        } else {
            /* need not check Ss/Se < 0 since they came from unsigned bytes */
            if cinfo.Ss > cinfo.Se || cinfo.Se >= DCTSIZE2 as i32 {
                bad = true;
            }
            /* AC scans may have only one component */
            if cinfo.comps_in_scan != 1 {
                bad = true;
            }
        }
        if cinfo.Ah != 0 {
            /* Successive approximation refinement scan: must have Al = Ah-1. */
            if cinfo.Al != cinfo.Ah - 1 {
                bad = true;
            }
        }
        if cinfo.Al > 13 {
            /* need not check for < 0 */
            bad = true;
        }
        /* Arguably the maximum Al value should be less than 13 for 8-bit precision,
         * but the spec doesn't say so, and we try to be liberal about what we
         * accept.  Note: large Al values could result in out-of-range DC
         * coefficients during early scans, leading to bizarre displays due to
         * overflows in the IDCT math.  But we won't crash.
         */
        if bad {
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
            if !is_DC_band && coef_bits(cinfo)[coef_bit_ptr][0] < 0 {
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
            if is_DC_band {
                entropy.pub_.decode_mcu = Some(decode_mcu_DC_first);
            } else {
                entropy.pub_.decode_mcu = Some(decode_mcu_AC_first);
            }
        } else if is_DC_band {
            entropy.pub_.decode_mcu = Some(decode_mcu_DC_refine);
        } else {
            entropy.pub_.decode_mcu = Some(decode_mcu_AC_refine);
        }

        for ci in 0..cinfo.comps_in_scan as usize {
            let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
            let compptr = &cinfo.comp_info()[index];
            /* Make sure requested tables are present, and compute derived tables.
             * We may build same derived table more than once, but it's not expensive.
             */
            // C forms the slot pointer before jpeg_make_d_derived_tbl rejects
            // an out-of-range number; the remainder only keeps that slot in
            // bounds until then.
            if is_DC_band {
                if cinfo.Ah == 0 {
                    /* DC refinement needs no table */
                    let tbl = compptr.dc_tbl_no;
                    jpeg_make_d_derived_tbl(
                        cinfo,
                        true,
                        tbl,
                        &mut entropy.derived_tbls[tbl as usize % NUM_HUFF_TBLS],
                    )?;
                }
            } else {
                let tbl = compptr.ac_tbl_no;
                jpeg_make_d_derived_tbl(
                    cinfo,
                    false,
                    tbl,
                    &mut entropy.derived_tbls[tbl as usize % NUM_HUFF_TBLS],
                )?;
                /* remember the single active table */
                entropy.ac_derived_tbl = tbl as usize;
            }
            /* Initialize DC predictions to 0 */
            entropy.saved.last_dc_val[ci] = 0;
        }

        /* Initialize bitread state variables */
        entropy.bitstate.bits_left = 0;
        entropy.bitstate.get_buffer = 0; /* unnecessary, but keeps Purify quiet */
        entropy.pub_.insufficient_data = false;

        /* Initialize private state variables */
        entropy.saved.EOBRUN = 0;

        /* Initialize restart counter */
        entropy.restarts_to_go = cinfo.restart_interval;
        Ok(())
    })
}

// upstream: src/jdphuff.c NEG_1
const NEG_1: u32 = u32::MAX;

// upstream: src/jdphuff.c HUFF_EXTEND
/// Figure F.12: extends the sign bit (the `AVOID_TABLES` form, which
/// upstream defines).
#[inline]
#[expect(non_snake_case, reason = "C macro name")]
#[expect(
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's unsigned arithmetic"
)]
fn HUFF_EXTEND(x: i32, s: i32) -> i32 {
    if x < (1 << (s - 1)) {
        (x as u32).wrapping_add((NEG_1 << s).wrapping_add(1)) as i32
    } else {
        x
    }
}

// upstream: src/jdphuff.c process_restart
/// Checks for a restart marker and resynchronizes the decoder; `false` if
/// it must suspend.
///
/// # Errors
///
/// What `read_restart_marker` reports.
#[expect(clippy::cast_sign_loss, reason = "C's unsigned arithmetic")]
#[allow(clippy::needless_range_loop, reason = "the C index loops")]
fn process_restart(
    cinfo: j_decompress_ptr<'_, '_>,
    entropy: &mut phuff_entropy_decoder,
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
    for ci in 0..cinfo.comps_in_scan as usize {
        entropy.saved.last_dc_val[ci] = 0;
    }
    /* Re-init EOB run count, too */
    entropy.saved.EOBRUN = 0;

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

/// `MCU_data[blkn]`, which C reads without a `NULL` check. Not an
/// upstream item.
///
/// # Panics
///
/// When `MCU_data` is `None`.
#[expect(non_snake_case, reason = "C names")]
fn mcu_block<'a>(MCU_data: &'a mut Option<&mut [&mut JBLOCK]>, blkn: usize) -> &'a mut JBLOCK {
    &mut *MCU_data.as_deref_mut().expect("MCU_data is NULL")[blkn]
}

/*
 * Huffman MCU decoding.
 * Each of these routines decodes and returns one MCU's worth of
 * Huffman-compressed coefficients.
 * The coefficients are reordered from zigzag order into natural array order,
 * but are not dequantized.
 *
 * The i'th block of the MCU is stored into the block pointed to by
 * MCU_data[i].  WE ASSUME THIS AREA IS INITIALLY ZEROED BY THE CALLER.
 *
 * We return FALSE if data source requested suspension.  In that case no
 * changes have been made to permanent state.  (Exception: some output
 * coefficients may already have been assigned.  This is harmless for
 * spectral selection, since we'll just re-assign them on the next call.
 * Successive approximation AC refinement has to be more careful, however.)
 */

// upstream: src/jdphuff.c decode_mcu_DC_first
/// MCU decoding for DC initial scan (either spectral selection, or first
/// pass of successive approximation).
///
/// # Errors
///
/// `JERR_BAD_DCT_COEF`, or what the bit reader or `read_restart_marker`
/// reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a progressive Huffman decoder,
/// `MCU_data` is `None`, or the scan's derived table is absent.
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
        let Al = cinfo.Al;

        /* Process restart marker if needed; may have to suspend */
        if cinfo.restart_interval != 0
            && entropy.restarts_to_go == 0
            && !process_restart(cinfo, entropy)?
        {
            return Ok(false);
        }

        /* If we've run out of data, just leave the MCU set to zeroes.
         * This way, we return uniform gray for the remainder of the segment.
         */
        if !entropy.pub_.insufficient_data {
            let phuff_entropy_decoder {
                pub_,
                bitstate,
                saved,
                derived_tbls,
                ..
            } = &mut *entropy;

            /* Load up working state */
            let (mut get_buffer, mut bits_left, mut br_state) = BITREAD_LOAD_STATE(cinfo, bitstate);
            let mut state = *saved;

            /* Outer loop handles each block in the MCU */

            for blkn in 0..cinfo.blocks_in_MCU as usize {
                let ci = cinfo.MCU_membership[blkn] as usize;
                let index = cinfo.cur_comp_info[ci].expect("cur_comp_info[ci] is NULL");
                let dc_tbl_no = cinfo.comp_info()[index].dc_tbl_no;
                let tbl = derived_tbls[dc_tbl_no as usize]
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
                    tbl,
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

                /* Convert DC difference to actual value, update last_dc_val */
                if (state.last_dc_val[ci] >= 0 && s > i32::MAX - state.last_dc_val[ci])
                    || (state.last_dc_val[ci] < 0 && s < i32::MIN - state.last_dc_val[ci])
                {
                    return Err(ERREXIT(cinfo, J_MESSAGE_CODE::JERR_BAD_DCT_COEF));
                }
                s += state.last_dc_val[ci];
                state.last_dc_val[ci] = s;
                /* Scale and output the coefficient (assumes jpeg_natural_order[0]=0) */
                mcu_block(&mut MCU_data, blkn)[0] = LEFT_SHIFT(JLONG::from(s), Al as u32) as JCOEF;
            }

            /* Completed MCU, so update state */
            BITREAD_SAVE_STATE(cinfo, bitstate, get_buffer, bits_left, &br_state);
            *saved = state;
        }

        /* Account for restart interval (no-op if not using restarts) */
        if cinfo.restart_interval != 0 {
            entropy.restarts_to_go = entropy.restarts_to_go.wrapping_sub(1);
        }

        Ok(true)
    })
}

// upstream: src/jdphuff.c decode_mcu_AC_first
/// MCU decoding for AC initial scan (either spectral selection, or first
/// pass of successive approximation).
///
/// # Errors
///
/// What the bit reader or `read_restart_marker` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a progressive Huffman decoder,
/// `MCU_data` is `None`, or the scan's derived table is absent.
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
        let Se = cinfo.Se;
        let Al = cinfo.Al;

        /* Process restart marker if needed; may have to suspend */
        if cinfo.restart_interval != 0
            && entropy.restarts_to_go == 0
            && !process_restart(cinfo, entropy)?
        {
            return Ok(false);
        }

        /* If we've run out of data, just leave the MCU set to zeroes.
         * This way, we return uniform gray for the remainder of the segment.
         */
        if !entropy.pub_.insufficient_data {
            /* Load up working state.
             * We can avoid loading/saving bitread state if in an EOB run.
             */
            let mut EOBRUN = entropy.saved.EOBRUN; /* only part of saved state we need */

            /* There is always only one block per MCU */

            if EOBRUN > 0 {
                /* if it's a band of zeroes... */
                EOBRUN -= 1; /* ...process it now (we do nothing) */
            } else {
                let phuff_entropy_decoder {
                    pub_,
                    bitstate,
                    derived_tbls,
                    ac_derived_tbl,
                    ..
                } = &mut *entropy;
                let (mut get_buffer, mut bits_left, mut br_state) =
                    BITREAD_LOAD_STATE(cinfo, bitstate);
                let block = mcu_block(&mut MCU_data, 0);
                let tbl = derived_tbls[*ac_derived_tbl]
                    .as_deref()
                    .expect("derived in start_pass");

                let mut k = cinfo.Ss;
                while k <= Se {
                    let Some(mut s) = HUFF_DECODE(
                        cinfo,
                        pub_,
                        &mut br_state,
                        &mut get_buffer,
                        &mut bits_left,
                        tbl,
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
                        /* Scale and output coefficient in natural (dezigzagged) order */
                        block[jpeg_natural_order[k as usize] as usize] =
                            LEFT_SHIFT(JLONG::from(s), Al as u32) as JCOEF;
                    } else if r == 15 {
                        /* ZRL */
                        k += 15; /* skip 15 zeroes in band */
                    } else {
                        /* EOBr, run length is 2^r + appended bits */
                        EOBRUN = 1 << r;
                        if r != 0 {
                            /* EOBr, r > 0 */
                            if !CHECK_BIT_BUFFER(
                                cinfo,
                                pub_,
                                &mut br_state,
                                &mut get_buffer,
                                &mut bits_left,
                                r,
                            )? {
                                return Ok(false);
                            }
                            r = GET_BITS(get_buffer, &mut bits_left, r);
                            EOBRUN += r as u32;
                        }
                        EOBRUN -= 1; /* this band is processed at this moment */
                        break; /* force end-of-band */
                    }
                    k += 1;
                }

                BITREAD_SAVE_STATE(cinfo, bitstate, get_buffer, bits_left, &br_state);
            }

            /* Completed MCU, so update state */
            entropy.saved.EOBRUN = EOBRUN; /* only part of saved state we need */
        }

        /* Account for restart interval (no-op if not using restarts) */
        if cinfo.restart_interval != 0 {
            entropy.restarts_to_go = entropy.restarts_to_go.wrapping_sub(1);
        }

        Ok(true)
    })
}

// upstream: src/jdphuff.c decode_mcu_DC_refine
/// MCU decoding for DC successive approximation refinement scan. Such
/// scans are assumed to be possibly multi-component, although the spec is
/// not very clear on the point.
///
/// # Errors
///
/// What the bit reader or `read_restart_marker` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a progressive Huffman decoder,
/// `MCU_data` is `None`, or the scan's derived table is absent.
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
        let p1 = 1 << cinfo.Al; /* 1 in the bit position being coded */

        /* Process restart marker if needed; may have to suspend */
        if cinfo.restart_interval != 0
            && entropy.restarts_to_go == 0
            && !process_restart(cinfo, entropy)?
        {
            return Ok(false);
        }

        /* Not worth the cycles to check insufficient_data here,
         * since we will not change the data anyway if we read zeroes.
         */

        let phuff_entropy_decoder { pub_, bitstate, .. } = &mut *entropy;

        /* Load up working state */
        let (mut get_buffer, mut bits_left, mut br_state) = BITREAD_LOAD_STATE(cinfo, bitstate);

        /* Outer loop handles each block in the MCU */

        for blkn in 0..cinfo.blocks_in_MCU as usize {
            /* Encoded data is simply the next bit of the two's-complement DC value */
            if !CHECK_BIT_BUFFER(
                cinfo,
                pub_,
                &mut br_state,
                &mut get_buffer,
                &mut bits_left,
                1,
            )? {
                return Ok(false);
            }
            if GET_BITS(get_buffer, &mut bits_left, 1) != 0 {
                let block = mcu_block(&mut MCU_data, blkn);
                block[0] = (i32::from(block[0]) | p1) as JCOEF;
            }
            /* Note: since we use |=, repeating the assignment later is safe */
        }

        /* Completed MCU, so update state */
        BITREAD_SAVE_STATE(cinfo, bitstate, get_buffer, bits_left, &br_state);

        /* Account for restart interval (no-op if not using restarts) */
        if cinfo.restart_interval != 0 {
            entropy.restarts_to_go = entropy.restarts_to_go.wrapping_sub(1);
        }

        Ok(true)
    })
}

/// Appends a correction bit to the already-nonzero coefficient `thiscoef`:
/// the inner step that `decode_mcu_AC_refine` repeats. Not an upstream
/// item.
#[expect(clippy::cast_possible_truncation, reason = "C's (JCOEF) casts")]
fn correct(thiscoef: &mut JCOEF, p1: i32, m1: i32) {
    if (i32::from(*thiscoef) & p1) == 0 {
        /* do nothing if already set it */
        if *thiscoef >= 0 {
            *thiscoef = thiscoef.wrapping_add(p1 as JCOEF);
        } else {
            *thiscoef = thiscoef.wrapping_add(m1 as JCOEF);
        }
    }
}

// upstream: src/jdphuff.c decode_mcu_AC_refine
/// MCU decoding for AC successive approximation refinement scan.
///
/// # Errors
///
/// What the bit reader, the warning or `read_restart_marker` reports.
///
/// # Panics
///
/// When `cinfo.entropy` does not hold a progressive Huffman decoder,
/// `MCU_data` is `None`, or the scan's derived table is absent.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "C's integer conversions"
)]
#[expect(clippy::too_many_lines, reason = "mirrors the C function")]
#[expect(non_snake_case, reason = "C names")]
pub fn decode_mcu_AC_refine(
    cinfo: j_decompress_ptr<'_, '_>,
    mut MCU_data: Option<&mut [&mut JBLOCK]>,
) -> Result<bool, ErrorExit> {
    with_entropy(cinfo, |cinfo, entropy| {
        let Se = cinfo.Se;
        let p1 = 1 << cinfo.Al; /* 1 in the bit position being coded */
        let m1 = (NEG_1 << cinfo.Al) as i32; /* -1 in the bit position being coded */
        let mut newnz_pos = [0_i32; DCTSIZE2];

        /* Process restart marker if needed; may have to suspend */
        if cinfo.restart_interval != 0
            && entropy.restarts_to_go == 0
            && !process_restart(cinfo, entropy)?
        {
            return Ok(false);
        }

        /* If we've run out of data, don't modify the MCU.
         */
        if !entropy.pub_.insufficient_data {
            let phuff_entropy_decoder {
                pub_,
                bitstate,
                saved,
                derived_tbls,
                ac_derived_tbl,
                ..
            } = &mut *entropy;

            /* Load up working state */
            let (mut get_buffer, mut bits_left, mut br_state) = BITREAD_LOAD_STATE(cinfo, bitstate);
            let mut EOBRUN = saved.EOBRUN; /* only part of saved state we need */

            /* There is always only one block per MCU */
            let block = mcu_block(&mut MCU_data, 0);
            let tbl = derived_tbls[*ac_derived_tbl]
                .as_deref()
                .expect("derived in start_pass");

            /* If we are forced to suspend, we must undo the assignments to any newly
             * nonzero coefficients in the block, because otherwise we'd get confused
             * next time about which coefficients were already nonzero.
             * But we need not undo addition of bits to already-nonzero coefficients;
             * instead, we can test the current bit to see if we already did it.
             */
            let mut num_newnz: usize = 0;

            /* initialize coefficient loop counter to start of band */
            let mut k = cinfo.Ss;

            let completed = 'undoit: {
                if EOBRUN == 0 {
                    while k <= Se {
                        let Some(mut s) = HUFF_DECODE(
                            cinfo,
                            pub_,
                            &mut br_state,
                            &mut get_buffer,
                            &mut bits_left,
                            tbl,
                        )?
                        else {
                            break 'undoit false;
                        };
                        let mut r = s >> 4;
                        s &= 15;
                        if s != 0 {
                            if s != 1 {
                                /* size of new coef should always be 1 */
                                WARNMS(cinfo, J_MESSAGE_CODE::JWRN_HUFF_BAD_CODE)?;
                            }
                            if !CHECK_BIT_BUFFER(
                                cinfo,
                                pub_,
                                &mut br_state,
                                &mut get_buffer,
                                &mut bits_left,
                                1,
                            )? {
                                break 'undoit false;
                            }
                            if GET_BITS(get_buffer, &mut bits_left, 1) != 0 {
                                s = p1; /* newly nonzero coef is positive */
                            } else {
                                s = m1; /* newly nonzero coef is negative */
                            }
                        } else if r != 15 {
                            EOBRUN = 1 << r; /* EOBr, run length is 2^r + appended bits */
                            if r != 0 {
                                if !CHECK_BIT_BUFFER(
                                    cinfo,
                                    pub_,
                                    &mut br_state,
                                    &mut get_buffer,
                                    &mut bits_left,
                                    r,
                                )? {
                                    break 'undoit false;
                                }
                                r = GET_BITS(get_buffer, &mut bits_left, r);
                                EOBRUN += r as u32;
                            }
                            break; /* rest of block is handled by EOB logic */
                        }
                        /* note s = 0 for processing ZRL */
                        /* Advance over already-nonzero coefs and r still-zero coefs,
                         * appending correction bits to the nonzeroes.  A correction bit is 1
                         * if the absolute value of the coefficient must be increased.
                         */
                        loop {
                            let thiscoef = &mut block[jpeg_natural_order[k as usize] as usize];
                            if *thiscoef != 0 {
                                if !CHECK_BIT_BUFFER(
                                    cinfo,
                                    pub_,
                                    &mut br_state,
                                    &mut get_buffer,
                                    &mut bits_left,
                                    1,
                                )? {
                                    break 'undoit false;
                                }
                                if GET_BITS(get_buffer, &mut bits_left, 1) != 0 {
                                    correct(thiscoef, p1, m1);
                                }
                            } else {
                                r -= 1;
                                if r < 0 {
                                    break; /* reached target zero coefficient */
                                }
                            }
                            k += 1;
                            if k > Se {
                                break;
                            }
                        }
                        if s != 0 {
                            let pos = jpeg_natural_order[k as usize];
                            /* Output newly nonzero coefficient */
                            block[pos as usize] = s as JCOEF;
                            /* Remember its position in case we have to suspend */
                            newnz_pos[num_newnz] = pos;
                            num_newnz += 1;
                        }
                        k += 1;
                    }
                }

                if EOBRUN > 0 {
                    /* Scan any remaining coefficient positions after the end-of-band
                     * (the last newly nonzero coefficient, if any).  Append a correction
                     * bit to each already-nonzero coefficient.  A correction bit is 1
                     * if the absolute value of the coefficient must be increased.
                     */
                    while k <= Se {
                        let thiscoef = &mut block[jpeg_natural_order[k as usize] as usize];
                        if *thiscoef != 0 {
                            if !CHECK_BIT_BUFFER(
                                cinfo,
                                pub_,
                                &mut br_state,
                                &mut get_buffer,
                                &mut bits_left,
                                1,
                            )? {
                                break 'undoit false;
                            }
                            if GET_BITS(get_buffer, &mut bits_left, 1) != 0 {
                                correct(thiscoef, p1, m1);
                            }
                        }
                        k += 1;
                    }
                    /* Count one block completed in EOB run */
                    EOBRUN -= 1;
                }
                true
            };

            if !completed {
                /* undoit: */
                /* Re-zero any output coefficients that we made newly nonzero */
                while num_newnz > 0 {
                    num_newnz -= 1;
                    block[newnz_pos[num_newnz] as usize] = 0;
                }

                return Ok(false);
            }

            /* Completed MCU, so update state */
            BITREAD_SAVE_STATE(cinfo, bitstate, get_buffer, bits_left, &br_state);
            saved.EOBRUN = EOBRUN; /* only part of saved state we need */
        }

        /* Account for restart interval (no-op if not using restarts) */
        if cinfo.restart_interval != 0 {
            entropy.restarts_to_go = entropy.restarts_to_go.wrapping_sub(1);
        }

        Ok(true)
    })
}

// upstream: src/jdphuff.c jinit_phuff_decoder
/// Module initialization routine for progressive Huffman entropy decoding.
///
/// # Errors
///
/// What the memory manager reports.
#[expect(clippy::cast_sign_loss, reason = "C's integer conversions")]
pub fn jinit_phuff_decoder(cinfo: j_decompress_ptr<'_, '_>) -> Result<(), ErrorExit> {
    let entropy = phuff_entropy_decoder {
        pub_: jpeg_entropy_decoder {
            start_pass: start_pass_phuff_decoder,
            decode_mcu: None,
            insufficient_data: false,
        },
        bitstate: bitread_perm_state::default(),
        saved: savable_state::default(),
        restarts_to_go: 0,
        /* Mark derived tables unallocated */
        derived_tbls: [const { None }; NUM_HUFF_TBLS],
        ac_derived_tbl: 0,
    };
    cinfo.entropy = Some(alloc_small_box(
        cinfo,
        JPOOL_IMAGE,
        SIZEOF_phuff_entropy_decoder,
        jpeg_entropy_decoder_impl::phuff_entropy_decoder(entropy),
    )?);

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
    Ok(())
}
