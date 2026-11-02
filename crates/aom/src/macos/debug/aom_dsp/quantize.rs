/*
 * Copyright (c) 2016, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/quantize.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type size_t = usize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type int64_t = i64;
pub type uint8_t = u8;
pub type uint16_t = u16;
pub type intptr_t = isize;
pub type qm_val_t = uint8_t;
pub type tran_low_t = int32_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT16_MAX: core::ffi::c_int = 32767 as core::ffi::c_int;
pub const INT16_MIN: core::ffi::c_int = -(32768 as core::ffi::c_int);
pub const AOM_QM_BITS: core::ffi::c_int = 5 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const EOB_FACTOR: core::ffi::c_int = 325 as core::ffi::c_int;
pub const SKIP_EOB_FACTOR_ADJUST: core::ffi::c_int = 200 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_adaptive_helper_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
    mut qm_ptr: *const qm_val_t,
    mut iqm_ptr: *const qm_val_t,
    log_scale: core::ffi::c_int,
) {
    let zbins: [core::ffi::c_int; 2] = [
        *zbin_ptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
        *zbin_ptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
    ];
    let nzbins: [core::ffi::c_int; 2] = [
        zbins[0 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
        zbins[1 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
    ];
    let mut i: core::ffi::c_int = 0;
    let mut non_zero_count: core::ffi::c_int = n_coeffs as core::ffi::c_int;
    let mut eob: core::ffi::c_int = -(1 as core::ffi::c_int);
    memset(
        qcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    memset(
        dqcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    let mut prescan_add: [core::ffi::c_int; 2] = [0; 2];
    i = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        prescan_add[i as usize] = *dequant_ptr.offset(i as isize) as core::ffi::c_int
            * 325 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 7 as core::ffi::c_int;
        i += 1;
    }
    i = n_coeffs as core::ffi::c_int - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let rc: core::ffi::c_int = *scan.offset(i as isize) as core::ffi::c_int;
        let wt: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        let coeff: core::ffi::c_int = *coeff_ptr.offset(rc as isize)
            * wt as core::ffi::c_int;
        let prescan_add_val: core::ffi::c_int = prescan_add[(rc != 0 as core::ffi::c_int)
            as core::ffi::c_int as usize];
        if !(coeff
            < zbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                * ((1 as core::ffi::c_int) << AOM_QM_BITS) + prescan_add_val
            && coeff
                > nzbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                    * ((1 as core::ffi::c_int) << AOM_QM_BITS) - prescan_add_val)
        {
            break;
        }
        non_zero_count -= 1;
        i -= 1;
    }
    let mut first: core::ffi::c_int = -(1 as core::ffi::c_int);
    i = 0 as core::ffi::c_int;
    while i < non_zero_count {
        let rc_0: core::ffi::c_int = *scan.offset(i as isize) as core::ffi::c_int;
        let coeff_0: core::ffi::c_int = *coeff_ptr.offset(rc_0 as isize);
        let coeff_sign: core::ffi::c_int = if coeff_0 < 0 as core::ffi::c_int {
            -(1 as core::ffi::c_int)
        } else {
            0 as core::ffi::c_int
        };
        let abs_coeff: core::ffi::c_int = (coeff_0 ^ coeff_sign) - coeff_sign;
        let mut tmp32: core::ffi::c_int = 0;
        let wt_0: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc_0 as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        if abs_coeff * wt_0 as core::ffi::c_int
            >= zbins[(rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                << AOM_QM_BITS
        {
            let mut tmp: int64_t = clamp(
                abs_coeff
                    + (*round_ptr
                        .offset(
                            (rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize,
                        ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
                        >> log_scale),
                INT16_MIN,
                INT16_MAX,
            ) as int64_t;
            tmp *= wt_0 as int64_t;
            tmp32 = (((tmp
                * *quant_ptr
                    .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                    as int64_t >> 16 as core::ffi::c_int) + tmp)
                * *quant_shift_ptr
                    .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                    as int64_t >> 16 as core::ffi::c_int - log_scale + AOM_QM_BITS)
                as core::ffi::c_int;
            *qcoeff_ptr.offset(rc_0 as isize) = ((tmp32 ^ coeff_sign) - coeff_sign)
                as tran_low_t;
            let iwt: core::ffi::c_int = if !iqm_ptr.is_null() {
                *iqm_ptr.offset(rc_0 as isize) as core::ffi::c_int
            } else {
                (1 as core::ffi::c_int) << AOM_QM_BITS
            };
            let dequant: core::ffi::c_int = *dequant_ptr
                .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as core::ffi::c_int * iwt
                + ((1 as core::ffi::c_int) << AOM_QM_BITS - 1 as core::ffi::c_int)
                >> AOM_QM_BITS;
            let abs_dqcoeff: tran_low_t = tmp32 as tran_low_t * dequant as tran_low_t
                >> log_scale;
            *dqcoeff_ptr.offset(rc_0 as isize) = (abs_dqcoeff ^ coeff_sign as tran_low_t)
                - coeff_sign as tran_low_t;
            if tmp32 != 0 {
                eob = i;
                if first == -(1 as core::ffi::c_int) {
                    first = i;
                }
            }
        }
        i += 1;
    }
    if eob >= 0 as core::ffi::c_int && first == eob {
        let rc_1: core::ffi::c_int = *scan.offset(eob as isize) as core::ffi::c_int;
        if *qcoeff_ptr.offset(rc_1 as isize) == 1 as tran_low_t
            || *qcoeff_ptr.offset(rc_1 as isize) == -(1 as tran_low_t)
        {
            let wt_1: qm_val_t = (if !qm_ptr.is_null() {
                *qm_ptr.offset(rc_1 as isize) as core::ffi::c_int
            } else {
                (1 as core::ffi::c_int) << AOM_QM_BITS
            }) as qm_val_t;
            let coeff_1: core::ffi::c_int = *coeff_ptr.offset(rc_1 as isize)
                * wt_1 as core::ffi::c_int;
            let factor: core::ffi::c_int = EOB_FACTOR + SKIP_EOB_FACTOR_ADJUST;
            let prescan_add_val_0: core::ffi::c_int = *dequant_ptr
                .offset((rc_1 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as core::ffi::c_int * factor
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int;
            if coeff_1
                < zbins[(rc_1 != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                    * ((1 as core::ffi::c_int) << AOM_QM_BITS) + prescan_add_val_0
                && coeff_1
                    > nzbins[(rc_1 != 0 as core::ffi::c_int) as core::ffi::c_int
                        as usize] * ((1 as core::ffi::c_int) << AOM_QM_BITS)
                        - prescan_add_val_0
            {
                *qcoeff_ptr.offset(rc_1 as isize) = 0 as core::ffi::c_int as tran_low_t;
                *dqcoeff_ptr.offset(rc_1 as isize) = 0 as core::ffi::c_int as tran_low_t;
                eob = -(1 as core::ffi::c_int);
            }
        }
    }
    *eob_ptr = (eob + 1 as core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_helper_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
    mut qm_ptr: *const qm_val_t,
    mut iqm_ptr: *const qm_val_t,
    log_scale: core::ffi::c_int,
) {
    let zbins: [core::ffi::c_int; 2] = [
        *zbin_ptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
        *zbin_ptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
    ];
    let nzbins: [core::ffi::c_int; 2] = [
        zbins[0 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
        zbins[1 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
    ];
    let mut i: core::ffi::c_int = 0;
    let mut non_zero_count: core::ffi::c_int = n_coeffs as core::ffi::c_int;
    let mut eob: core::ffi::c_int = -(1 as core::ffi::c_int);
    memset(
        qcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    memset(
        dqcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    i = n_coeffs as core::ffi::c_int - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let rc: core::ffi::c_int = *scan.offset(i as isize) as core::ffi::c_int;
        let wt: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        let coeff: core::ffi::c_int = *coeff_ptr.offset(rc as isize)
            * wt as core::ffi::c_int;
        if !(coeff
            < zbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                * ((1 as core::ffi::c_int) << AOM_QM_BITS)
            && coeff
                > nzbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                    * ((1 as core::ffi::c_int) << AOM_QM_BITS))
        {
            break;
        }
        non_zero_count -= 1;
        i -= 1;
    }
    i = 0 as core::ffi::c_int;
    while i < non_zero_count {
        let rc_0: core::ffi::c_int = *scan.offset(i as isize) as core::ffi::c_int;
        let coeff_0: core::ffi::c_int = *coeff_ptr.offset(rc_0 as isize);
        let coeff_sign: core::ffi::c_int = if coeff_0 < 0 as core::ffi::c_int {
            -(1 as core::ffi::c_int)
        } else {
            0 as core::ffi::c_int
        };
        let abs_coeff: core::ffi::c_int = (coeff_0 ^ coeff_sign) - coeff_sign;
        let mut tmp32: core::ffi::c_int = 0;
        let wt_0: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc_0 as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        if abs_coeff * wt_0 as core::ffi::c_int
            >= zbins[(rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                << AOM_QM_BITS
        {
            let mut tmp: int64_t = clamp(
                abs_coeff
                    + (*round_ptr
                        .offset(
                            (rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize,
                        ) as core::ffi::c_int
                        + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
                        >> log_scale),
                INT16_MIN,
                INT16_MAX,
            ) as int64_t;
            tmp *= wt_0 as int64_t;
            tmp32 = (((tmp
                * *quant_ptr
                    .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                    as int64_t >> 16 as core::ffi::c_int) + tmp)
                * *quant_shift_ptr
                    .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                    as int64_t >> 16 as core::ffi::c_int - log_scale + AOM_QM_BITS)
                as core::ffi::c_int;
            *qcoeff_ptr.offset(rc_0 as isize) = ((tmp32 ^ coeff_sign) - coeff_sign)
                as tran_low_t;
            let iwt: core::ffi::c_int = if !iqm_ptr.is_null() {
                *iqm_ptr.offset(rc_0 as isize) as core::ffi::c_int
            } else {
                (1 as core::ffi::c_int) << AOM_QM_BITS
            };
            let dequant: core::ffi::c_int = *dequant_ptr
                .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as core::ffi::c_int * iwt
                + ((1 as core::ffi::c_int) << AOM_QM_BITS - 1 as core::ffi::c_int)
                >> AOM_QM_BITS;
            let abs_dqcoeff: tran_low_t = tmp32 as tran_low_t * dequant as tran_low_t
                >> log_scale;
            *dqcoeff_ptr.offset(rc_0 as isize) = (abs_dqcoeff ^ coeff_sign as tran_low_t)
                - coeff_sign as tran_low_t;
            if tmp32 != 0 {
                eob = i;
            }
        }
        i += 1;
    }
    *eob_ptr = (eob + 1 as core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_adaptive_helper_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
    mut qm_ptr: *const qm_val_t,
    mut iqm_ptr: *const qm_val_t,
    log_scale: core::ffi::c_int,
) {
    let zbins: [core::ffi::c_int; 2] = [
        *zbin_ptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
        *zbin_ptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
    ];
    let nzbins: [core::ffi::c_int; 2] = [
        zbins[0 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
        zbins[1 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
    ];
    let mut i: core::ffi::c_int = 0;
    let mut non_zero_count: core::ffi::c_int = n_coeffs as core::ffi::c_int;
    let mut eob: core::ffi::c_int = -(1 as core::ffi::c_int);
    memset(
        qcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    memset(
        dqcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    let mut prescan_add: [core::ffi::c_int; 2] = [0; 2];
    i = 0 as core::ffi::c_int;
    while i < 2 as core::ffi::c_int {
        prescan_add[i as usize] = *dequant_ptr.offset(i as isize) as core::ffi::c_int
            * 325 as core::ffi::c_int
            + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int >> 1 as core::ffi::c_int)
            >> 7 as core::ffi::c_int;
        i += 1;
    }
    i = n_coeffs as core::ffi::c_int - 1 as core::ffi::c_int;
    while i >= 0 as core::ffi::c_int {
        let rc: core::ffi::c_int = *scan.offset(i as isize) as core::ffi::c_int;
        let wt: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        let coeff: core::ffi::c_int = *coeff_ptr.offset(rc as isize)
            * wt as core::ffi::c_int;
        let prescan_add_val: core::ffi::c_int = prescan_add[(rc != 0 as core::ffi::c_int)
            as core::ffi::c_int as usize];
        if !(coeff
            < zbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                * ((1 as core::ffi::c_int) << AOM_QM_BITS) + prescan_add_val
            && coeff
                > nzbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                    * ((1 as core::ffi::c_int) << AOM_QM_BITS) - prescan_add_val)
        {
            break;
        }
        non_zero_count -= 1;
        i -= 1;
    }
    let mut first: core::ffi::c_int = -(1 as core::ffi::c_int);
    i = 0 as core::ffi::c_int;
    while i < non_zero_count {
        let rc_0: core::ffi::c_int = *scan.offset(i as isize) as core::ffi::c_int;
        let coeff_0: core::ffi::c_int = *coeff_ptr.offset(rc_0 as isize);
        let coeff_sign: core::ffi::c_int = if coeff_0 < 0 as core::ffi::c_int {
            -(1 as core::ffi::c_int)
        } else {
            0 as core::ffi::c_int
        };
        let wt_0: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc_0 as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        let abs_coeff: core::ffi::c_int = (coeff_0 ^ coeff_sign) - coeff_sign;
        if abs_coeff * wt_0 as core::ffi::c_int
            >= zbins[(rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                << AOM_QM_BITS
        {
            let tmp1: int64_t = (abs_coeff
                + (*round_ptr
                    .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                    as core::ffi::c_int
                    + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
                    >> log_scale)) as int64_t;
            let tmpw: int64_t = tmp1 * wt_0 as int64_t;
            let tmp2: int64_t = (tmpw
                * *quant_ptr
                    .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                    as int64_t >> 16 as core::ffi::c_int) + tmpw;
            let abs_qcoeff: core::ffi::c_int = (tmp2
                * *quant_shift_ptr
                    .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                    as int64_t >> 16 as core::ffi::c_int - log_scale + AOM_QM_BITS)
                as core::ffi::c_int;
            *qcoeff_ptr.offset(rc_0 as isize) = ((abs_qcoeff ^ coeff_sign) - coeff_sign)
                as tran_low_t;
            let iwt: qm_val_t = (if !iqm_ptr.is_null() {
                *iqm_ptr.offset(rc_0 as isize) as core::ffi::c_int
            } else {
                (1 as core::ffi::c_int) << AOM_QM_BITS
            }) as qm_val_t;
            let dequant: core::ffi::c_int = *dequant_ptr
                .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as core::ffi::c_int * iwt as core::ffi::c_int
                + ((1 as core::ffi::c_int) << AOM_QM_BITS - 1 as core::ffi::c_int)
                >> AOM_QM_BITS;
            let abs_dqcoeff: tran_low_t = abs_qcoeff as tran_low_t
                * dequant as tran_low_t >> log_scale;
            *dqcoeff_ptr.offset(rc_0 as isize) = (abs_dqcoeff ^ coeff_sign as tran_low_t)
                - coeff_sign as tran_low_t;
            if abs_qcoeff != 0 {
                eob = i;
                if first == -(1 as core::ffi::c_int) {
                    first = eob;
                }
            }
        }
        i += 1;
    }
    if eob >= 0 as core::ffi::c_int && first == eob {
        let rc_1: core::ffi::c_int = *scan.offset(eob as isize) as core::ffi::c_int;
        if *qcoeff_ptr.offset(rc_1 as isize) == 1 as tran_low_t
            || *qcoeff_ptr.offset(rc_1 as isize) == -(1 as tran_low_t)
        {
            let wt_1: qm_val_t = (if !qm_ptr.is_null() {
                *qm_ptr.offset(rc_1 as isize) as core::ffi::c_int
            } else {
                (1 as core::ffi::c_int) << AOM_QM_BITS
            }) as qm_val_t;
            let coeff_1: core::ffi::c_int = *coeff_ptr.offset(rc_1 as isize)
                * wt_1 as core::ffi::c_int;
            let factor: core::ffi::c_int = EOB_FACTOR + SKIP_EOB_FACTOR_ADJUST;
            let prescan_add_val_0: core::ffi::c_int = *dequant_ptr
                .offset((rc_1 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as core::ffi::c_int * factor
                + ((1 as core::ffi::c_int) << 7 as core::ffi::c_int
                    >> 1 as core::ffi::c_int) >> 7 as core::ffi::c_int;
            if coeff_1
                < zbins[(rc_1 != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                    * ((1 as core::ffi::c_int) << AOM_QM_BITS) + prescan_add_val_0
                && coeff_1
                    > nzbins[(rc_1 != 0 as core::ffi::c_int) as core::ffi::c_int
                        as usize] * ((1 as core::ffi::c_int) << AOM_QM_BITS)
                        - prescan_add_val_0
            {
                *qcoeff_ptr.offset(rc_1 as isize) = 0 as core::ffi::c_int as tran_low_t;
                *dqcoeff_ptr.offset(rc_1 as isize) = 0 as core::ffi::c_int as tran_low_t;
                eob = -(1 as core::ffi::c_int);
            }
        }
    }
    *eob_ptr = (eob + 1 as core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_helper_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
    mut qm_ptr: *const qm_val_t,
    mut iqm_ptr: *const qm_val_t,
    log_scale: core::ffi::c_int,
) {
    let mut i: core::ffi::c_int = 0;
    let mut eob: core::ffi::c_int = -(1 as core::ffi::c_int);
    let zbins: [core::ffi::c_int; 2] = [
        *zbin_ptr.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
        *zbin_ptr.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int
            + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
            >> log_scale,
    ];
    let nzbins: [core::ffi::c_int; 2] = [
        zbins[0 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
        zbins[1 as core::ffi::c_int as usize] * -(1 as core::ffi::c_int),
    ];
    let mut dequant: core::ffi::c_int = 0;
    let mut idx_arr: [core::ffi::c_int; 4096] = [0; 4096];
    let mut idx: core::ffi::c_int = 0 as core::ffi::c_int;
    memset(
        qcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    memset(
        dqcoeff_ptr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (n_coeffs as size_t).wrapping_mul(::core::mem::size_of::<tran_low_t>() as size_t),
    );
    i = 0 as core::ffi::c_int;
    while (i as intptr_t) < n_coeffs {
        let rc: core::ffi::c_int = *scan.offset(i as isize) as core::ffi::c_int;
        let wt: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        let coeff: core::ffi::c_int = *coeff_ptr.offset(rc as isize)
            * wt as core::ffi::c_int;
        if coeff
            >= zbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                * ((1 as core::ffi::c_int) << AOM_QM_BITS)
            || coeff
                <= nzbins[(rc != 0 as core::ffi::c_int) as core::ffi::c_int as usize]
                    * ((1 as core::ffi::c_int) << AOM_QM_BITS)
        {
            let fresh0 = idx;
            idx = idx + 1;
            idx_arr[fresh0 as usize] = i;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < idx {
        let rc_0: core::ffi::c_int = *scan.offset(idx_arr[i as usize] as isize)
            as core::ffi::c_int;
        let coeff_0: core::ffi::c_int = *coeff_ptr.offset(rc_0 as isize);
        let coeff_sign: core::ffi::c_int = if coeff_0 < 0 as core::ffi::c_int {
            -(1 as core::ffi::c_int)
        } else {
            0 as core::ffi::c_int
        };
        let wt_0: qm_val_t = (if !qm_ptr.is_null() {
            *qm_ptr.offset(rc_0 as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        let iwt: qm_val_t = (if !iqm_ptr.is_null() {
            *iqm_ptr.offset(rc_0 as isize) as core::ffi::c_int
        } else {
            (1 as core::ffi::c_int) << AOM_QM_BITS
        }) as qm_val_t;
        let abs_coeff: core::ffi::c_int = (coeff_0 ^ coeff_sign) - coeff_sign;
        let tmp1: int64_t = (abs_coeff
            + (*round_ptr
                .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as core::ffi::c_int
                + ((1 as core::ffi::c_int) << log_scale >> 1 as core::ffi::c_int)
                >> log_scale)) as int64_t;
        let tmpw: int64_t = tmp1 * wt_0 as int64_t;
        let tmp2: int64_t = (tmpw
            * *quant_ptr
                .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as int64_t >> 16 as core::ffi::c_int) + tmpw;
        let abs_qcoeff: core::ffi::c_int = (tmp2
            * *quant_shift_ptr
                .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
                as int64_t >> 16 as core::ffi::c_int - log_scale + AOM_QM_BITS)
            as core::ffi::c_int;
        *qcoeff_ptr.offset(rc_0 as isize) = ((abs_qcoeff ^ coeff_sign) - coeff_sign)
            as tran_low_t;
        dequant = *dequant_ptr
            .offset((rc_0 != 0 as core::ffi::c_int) as core::ffi::c_int as isize)
            as core::ffi::c_int * iwt as core::ffi::c_int
            + ((1 as core::ffi::c_int) << AOM_QM_BITS - 1 as core::ffi::c_int)
            >> AOM_QM_BITS;
        let abs_dqcoeff: tran_low_t = abs_qcoeff as tran_low_t * dequant as tran_low_t
            >> log_scale;
        *dqcoeff_ptr.offset(rc_0 as isize) = (abs_dqcoeff ^ coeff_sign as tran_low_t)
            - coeff_sign as tran_low_t;
        if abs_qcoeff != 0 {
            eob = idx_arr[i as usize];
        }
        i += 1;
    }
    *eob_ptr = (eob + 1 as core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_adaptive_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_quantize_b_adaptive_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_32x32_adaptive_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_quantize_b_adaptive_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_64x64_adaptive_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_quantize_b_adaptive_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        2 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_adaptive_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_highbd_quantize_b_adaptive_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_32x32_adaptive_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_highbd_quantize_b_adaptive_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_64x64_adaptive_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_highbd_quantize_b_adaptive_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        2 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_quantize_b_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_32x32_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_quantize_b_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_quantize_b_64x64_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_quantize_b_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        2 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_highbd_quantize_b_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_32x32_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_highbd_quantize_b_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_highbd_quantize_b_64x64_c(
    mut coeff_ptr: *const tran_low_t,
    mut n_coeffs: intptr_t,
    mut zbin_ptr: *const int16_t,
    mut round_ptr: *const int16_t,
    mut quant_ptr: *const int16_t,
    mut quant_shift_ptr: *const int16_t,
    mut qcoeff_ptr: *mut tran_low_t,
    mut dqcoeff_ptr: *mut tran_low_t,
    mut dequant_ptr: *const int16_t,
    mut eob_ptr: *mut uint16_t,
    mut scan: *const int16_t,
    mut iscan: *const int16_t,
) {
    aom_highbd_quantize_b_helper_c(
        coeff_ptr,
        n_coeffs,
        zbin_ptr,
        round_ptr,
        quant_ptr,
        quant_shift_ptr,
        qcoeff_ptr,
        dqcoeff_ptr,
        dequant_ptr,
        eob_ptr,
        scan,
        iscan,
        0 as *const qm_val_t,
        0 as *const qm_val_t,
        2 as core::ffi::c_int,
    );
}

extern "C" {
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
