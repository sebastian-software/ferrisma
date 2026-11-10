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

/*!\file
 * \brief This file has the implementation details of the grain table.
 *
 * The file format is an ascii representation for readability and
 * editability. Array parameters are separated from the non-array
 * parameters and prefixed with a few characters to make for easy
 * localization with a parameter set. Each entry is prefixed with "E"
 * and the other parameters are only specified if "update-parms" is
 * non-zero.
 *
 * filmgrn1
 * E <start-time> <end-time> <apply-grain> <random-seed> <update-parms>
 *  p <ar_coeff_lag> <ar_coeff_shift> <grain_scale_shift> ...
 *  sY <num_y_points> <point_0_x> <point_0_y> ...
 *  sCb <num_cb_points> <point_0_x> <point_0_y> ...
 *  sCr <num_cr_points> <point_0_x> <point_0_y> ...
 *  cY <ar_coeff_y_0> ....
 *  cCb <ar_coeff_cb_0> ....
 *  cCr <ar_coeff_cr_0> ....
 * E <start-time> ...
 */

// Mechanically translated from upstream/aom/aom_dsp/grain_table.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type __sFILEX;
    fn aom_internal_error(
        info: *mut aom_internal_error_info,
        error: aom_codec_err_t,
        fmt: *const core::ffi::c_char,
        ...
    );
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
}
pub type __int64_t = i64;
pub type __darwin_size_t = usize;
pub type __darwin_off_t = __int64_t;
pub type int64_t = i64;
pub type uint16_t = u16;
pub type size_t = __darwin_size_t;
pub type fpos_t = __darwin_off_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sbuf {
    pub _base: *mut core::ffi::c_uchar,
    pub _size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __sFILE {
    pub _p: *mut core::ffi::c_uchar,
    pub _r: core::ffi::c_int,
    pub _w: core::ffi::c_int,
    pub _flags: core::ffi::c_short,
    pub _file: core::ffi::c_short,
    pub _bf: __sbuf,
    pub _lbfsize: core::ffi::c_int,
    pub _cookie: *mut core::ffi::c_void,
    pub _close: Option<unsafe extern "C-unwind" fn(*mut core::ffi::c_void) -> core::ffi::c_int>,
    pub _read: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            *mut core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _seek: Option<
        unsafe extern "C-unwind" fn(*mut core::ffi::c_void, fpos_t, core::ffi::c_int) -> fpos_t,
    >,
    pub _write: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            *const core::ffi::c_char,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >,
    pub _ub: __sbuf,
    pub _extra: *mut __sFILEX,
    pub _ur: core::ffi::c_int,
    pub _ubuf: [core::ffi::c_uchar; 3],
    pub _nbuf: [core::ffi::c_uchar; 1],
    pub _lb: __sbuf,
    pub _blksize: core::ffi::c_int,
    pub _offset: fpos_t,
}
pub type FILE = libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_film_grain_t {
    pub apply_grain: core::ffi::c_int,
    pub update_parameters: core::ffi::c_int,
    pub scaling_points_y: [[core::ffi::c_int; 2]; 14],
    pub num_y_points: core::ffi::c_int,
    pub scaling_points_cb: [[core::ffi::c_int; 2]; 10],
    pub num_cb_points: core::ffi::c_int,
    pub scaling_points_cr: [[core::ffi::c_int; 2]; 10],
    pub num_cr_points: core::ffi::c_int,
    pub scaling_shift: core::ffi::c_int,
    pub ar_coeff_lag: core::ffi::c_int,
    pub ar_coeffs_y: [core::ffi::c_int; 24],
    pub ar_coeffs_cb: [core::ffi::c_int; 25],
    pub ar_coeffs_cr: [core::ffi::c_int; 25],
    pub ar_coeff_shift: core::ffi::c_int,
    pub cb_mult: core::ffi::c_int,
    pub cb_luma_mult: core::ffi::c_int,
    pub cb_offset: core::ffi::c_int,
    pub cr_mult: core::ffi::c_int,
    pub cr_luma_mult: core::ffi::c_int,
    pub cr_offset: core::ffi::c_int,
    pub overlap_flag: core::ffi::c_int,
    pub clip_to_restricted_range: core::ffi::c_int,
    pub bit_depth: core::ffi::c_uint,
    pub chroma_scaling_from_luma: core::ffi::c_int,
    pub grain_scale_shift: core::ffi::c_int,
    pub random_seed: uint16_t,
}
pub type aom_codec_err_t = core::ffi::c_uint;
pub const AOM_CODEC_LIST_END: aom_codec_err_t = 9;
pub const AOM_CODEC_INVALID_PARAM: aom_codec_err_t = 8;
pub const AOM_CODEC_CORRUPT_FRAME: aom_codec_err_t = 7;
pub const AOM_CODEC_UNSUP_FEATURE: aom_codec_err_t = 6;
pub const AOM_CODEC_UNSUP_BITSTREAM: aom_codec_err_t = 5;
pub const AOM_CODEC_INCAPABLE: aom_codec_err_t = 4;
pub const AOM_CODEC_ABI_MISMATCH: aom_codec_err_t = 3;
pub const AOM_CODEC_MEM_ERROR: aom_codec_err_t = 2;
pub const AOM_CODEC_ERROR: aom_codec_err_t = 1;
pub const AOM_CODEC_OK: aom_codec_err_t = 0;
pub type jmp_buf = [core::ffi::c_int; 48];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_internal_error_info {
    pub error_code: aom_codec_err_t,
    pub has_detail: core::ffi::c_int,
    pub detail: [core::ffi::c_char; 200],
    pub setjmp: core::ffi::c_int,
    pub jmp: jmp_buf,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_film_grain_table_entry_t {
    pub params: aom_film_grain_t,
    pub start_time: int64_t,
    pub end_time: int64_t,
    pub next: *mut aom_film_grain_table_entry_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_film_grain_table_t {
    pub head: *mut aom_film_grain_table_entry_t,
    pub tail: *mut aom_film_grain_table_entry_t,
}
pub const EOF: core::ffi::c_int = -(1 as core::ffi::c_int);
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut kFileMagic: [core::ffi::c_char; 8] = [
    'f' as i32 as core::ffi::c_char,
    'i' as i32 as core::ffi::c_char,
    'l' as i32 as core::ffi::c_char,
    'm' as i32 as core::ffi::c_char,
    'g' as i32 as core::ffi::c_char,
    'r' as i32 as core::ffi::c_char,
    'n' as i32 as core::ffi::c_char,
    '1' as i32 as core::ffi::c_char,
];
unsafe extern "C-unwind" fn grain_table_entry_read(
    mut file: *mut FILE,
    mut error_info: *mut aom_internal_error_info,
    mut entry: *mut aom_film_grain_table_entry_t,
) {
    let mut pars: *mut aom_film_grain_t = &raw mut (*entry).params;
    let mut num_read: core::ffi::c_int = fscanf(
        file,
        b"E %lld %lld %d %hd %d\n\0" as *const u8 as *const core::ffi::c_char,
        &raw mut (*entry).start_time as *mut int64_t,
        &raw mut (*entry).end_time as *mut int64_t,
        &raw mut (*pars).apply_grain as *mut core::ffi::c_int,
        &raw mut (*pars).random_seed as *mut uint16_t,
        &raw mut (*pars).update_parameters as *mut core::ffi::c_int,
    );
    if num_read == 0 as core::ffi::c_int && feof(file) != 0 {
        return;
    }
    if num_read != 5 as core::ffi::c_int {
        aom_internal_error(
            error_info,
            AOM_CODEC_ERROR,
            b"Unable to read entry header. Read %d != 5\0" as *const u8
                as *const core::ffi::c_char,
            num_read,
        );
    }
    if (*pars).update_parameters != 0 {
        num_read = fscanf(
            file,
            b"p %d %d %d %d %d %d %d %d %d %d %d %d\n\0" as *const u8
                as *const core::ffi::c_char,
            &raw mut (*pars).ar_coeff_lag as *mut core::ffi::c_int,
            &raw mut (*pars).ar_coeff_shift as *mut core::ffi::c_int,
            &raw mut (*pars).grain_scale_shift as *mut core::ffi::c_int,
            &raw mut (*pars).scaling_shift as *mut core::ffi::c_int,
            &raw mut (*pars).chroma_scaling_from_luma as *mut core::ffi::c_int,
            &raw mut (*pars).overlap_flag as *mut core::ffi::c_int,
            &raw mut (*pars).cb_mult as *mut core::ffi::c_int,
            &raw mut (*pars).cb_luma_mult as *mut core::ffi::c_int,
            &raw mut (*pars).cb_offset as *mut core::ffi::c_int,
            &raw mut (*pars).cr_mult as *mut core::ffi::c_int,
            &raw mut (*pars).cr_luma_mult as *mut core::ffi::c_int,
            &raw mut (*pars).cr_offset as *mut core::ffi::c_int,
        );
        if num_read != 12 as core::ffi::c_int {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to read entry params. Read %d != 12\0" as *const u8
                    as *const core::ffi::c_char,
                num_read,
            );
        }
        if 1 as core::ffi::c_int
            != fscanf(
                file,
                b"\tsY %d \0" as *const u8 as *const core::ffi::c_char,
                &raw mut (*pars).num_y_points as *mut core::ffi::c_int,
            )
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to read num y points\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*pars).num_y_points {
            if 2 as core::ffi::c_int
                != fscanf(
                    file,
                    b"%d %d\0" as *const u8 as *const core::ffi::c_char,
                    &raw mut *(*((*pars).scaling_points_y).as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut core::ffi::c_int,
                    &raw mut *(*((*pars).scaling_points_y).as_mut_ptr().offset(i as isize))
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize) as *mut core::ffi::c_int,
                )
            {
                aom_internal_error(
                    error_info,
                    AOM_CODEC_ERROR,
                    b"Unable to read y scaling points\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            i += 1;
        }
        if 1 as core::ffi::c_int
            != fscanf(
                file,
                b"\n\tsCb %d\0" as *const u8 as *const core::ffi::c_char,
                &raw mut (*pars).num_cb_points as *mut core::ffi::c_int,
            )
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to read num cb points\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < (*pars).num_cb_points {
            if 2 as core::ffi::c_int
                != fscanf(
                    file,
                    b"%d %d\0" as *const u8 as *const core::ffi::c_char,
                    &raw mut *(*((*pars).scaling_points_cb)
                        .as_mut_ptr()
                        .offset(i_0 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut core::ffi::c_int,
                    &raw mut *(*((*pars).scaling_points_cb)
                        .as_mut_ptr()
                        .offset(i_0 as isize))
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize) as *mut core::ffi::c_int,
                )
            {
                aom_internal_error(
                    error_info,
                    AOM_CODEC_ERROR,
                    b"Unable to read cb scaling points\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            i_0 += 1;
        }
        if 1 as core::ffi::c_int
            != fscanf(
                file,
                b"\n\tsCr %d\0" as *const u8 as *const core::ffi::c_char,
                &raw mut (*pars).num_cr_points as *mut core::ffi::c_int,
            )
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to read num cr points\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < (*pars).num_cr_points {
            if 2 as core::ffi::c_int
                != fscanf(
                    file,
                    b"%d %d\0" as *const u8 as *const core::ffi::c_char,
                    &raw mut *(*((*pars).scaling_points_cr)
                        .as_mut_ptr()
                        .offset(i_1 as isize))
                        .as_mut_ptr()
                        .offset(0 as core::ffi::c_int as isize) as *mut core::ffi::c_int,
                    &raw mut *(*((*pars).scaling_points_cr)
                        .as_mut_ptr()
                        .offset(i_1 as isize))
                        .as_mut_ptr()
                        .offset(1 as core::ffi::c_int as isize) as *mut core::ffi::c_int,
                )
            {
                aom_internal_error(
                    error_info,
                    AOM_CODEC_ERROR,
                    b"Unable to read cr scaling points\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            i_1 += 1;
        }
        if fscanf(file, b"\n\tcY\0" as *const u8 as *const core::ffi::c_char) != 0 {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to read Y coeffs header (cY)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let n: core::ffi::c_int = 2 as core::ffi::c_int * (*pars).ar_coeff_lag
            * ((*pars).ar_coeff_lag + 1 as core::ffi::c_int);
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < n {
            if 1 as core::ffi::c_int
                != fscanf(
                    file,
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &raw mut *((*pars).ar_coeffs_y).as_mut_ptr().offset(i_2 as isize)
                        as *mut core::ffi::c_int,
                )
            {
                aom_internal_error(
                    error_info,
                    AOM_CODEC_ERROR,
                    b"Unable to read Y coeffs\0" as *const u8 as *const core::ffi::c_char,
                );
            }
            i_2 += 1;
        }
        if fscanf(file, b"\n\tcCb\0" as *const u8 as *const core::ffi::c_char) != 0 {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to read Cb coeffs header (cCb)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_3 <= n {
            if 1 as core::ffi::c_int
                != fscanf(
                    file,
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &raw mut *((*pars).ar_coeffs_cb).as_mut_ptr().offset(i_3 as isize)
                        as *mut core::ffi::c_int,
                )
            {
                aom_internal_error(
                    error_info,
                    AOM_CODEC_ERROR,
                    b"Unable to read Cb coeffs\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            i_3 += 1;
        }
        if fscanf(file, b"\n\tcCr\0" as *const u8 as *const core::ffi::c_char) != 0 {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable read to Cr coeffs header (cCr)\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut i_4: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_4 <= n {
            if 1 as core::ffi::c_int
                != fscanf(
                    file,
                    b"%d\0" as *const u8 as *const core::ffi::c_char,
                    &raw mut *((*pars).ar_coeffs_cr).as_mut_ptr().offset(i_4 as isize)
                        as *mut core::ffi::c_int,
                )
            {
                aom_internal_error(
                    error_info,
                    AOM_CODEC_ERROR,
                    b"Unable to read Cr coeffs\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            i_4 += 1;
        }
        fscanf(file, b"\n\0" as *const u8 as *const core::ffi::c_char);
    }
}
unsafe extern "C-unwind" fn grain_table_entry_write(
    mut file: *mut FILE,
    mut entry: *mut aom_film_grain_table_entry_t,
) {
    let mut pars: *const aom_film_grain_t = &raw mut (*entry).params;
    fprintf(
        file,
        b"E %lld %lld %d %d %d\n\0" as *const u8 as *const core::ffi::c_char,
        (*entry).start_time,
        (*entry).end_time,
        (*pars).apply_grain,
        (*pars).random_seed as core::ffi::c_int,
        (*pars).update_parameters,
    );
    if (*pars).update_parameters != 0 {
        fprintf(
            file,
            b"\tp %d %d %d %d %d %d %d %d %d %d %d %d\n\0" as *const u8
                as *const core::ffi::c_char,
            (*pars).ar_coeff_lag,
            (*pars).ar_coeff_shift,
            (*pars).grain_scale_shift,
            (*pars).scaling_shift,
            (*pars).chroma_scaling_from_luma,
            (*pars).overlap_flag,
            (*pars).cb_mult,
            (*pars).cb_luma_mult,
            (*pars).cb_offset,
            (*pars).cr_mult,
            (*pars).cr_luma_mult,
            (*pars).cr_offset,
        );
        fprintf(
            file,
            b"\tsY %d \0" as *const u8 as *const core::ffi::c_char,
            (*pars).num_y_points,
        );
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < (*pars).num_y_points {
            fprintf(
                file,
                b" %d %d\0" as *const u8 as *const core::ffi::c_char,
                (*pars).scaling_points_y[i as usize][0 as core::ffi::c_int as usize],
                (*pars).scaling_points_y[i as usize][1 as core::ffi::c_int as usize],
            );
            i += 1;
        }
        fprintf(
            file,
            b"\n\tsCb %d\0" as *const u8 as *const core::ffi::c_char,
            (*pars).num_cb_points,
        );
        let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_0 < (*pars).num_cb_points {
            fprintf(
                file,
                b" %d %d\0" as *const u8 as *const core::ffi::c_char,
                (*pars).scaling_points_cb[i_0 as usize][0 as core::ffi::c_int as usize],
                (*pars).scaling_points_cb[i_0 as usize][1 as core::ffi::c_int as usize],
            );
            i_0 += 1;
        }
        fprintf(
            file,
            b"\n\tsCr %d\0" as *const u8 as *const core::ffi::c_char,
            (*pars).num_cr_points,
        );
        let mut i_1: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_1 < (*pars).num_cr_points {
            fprintf(
                file,
                b" %d %d\0" as *const u8 as *const core::ffi::c_char,
                (*pars).scaling_points_cr[i_1 as usize][0 as core::ffi::c_int as usize],
                (*pars).scaling_points_cr[i_1 as usize][1 as core::ffi::c_int as usize],
            );
            i_1 += 1;
        }
        fprintf(file, b"\n\tcY\0" as *const u8 as *const core::ffi::c_char);
        let n: core::ffi::c_int = 2 as core::ffi::c_int * (*pars).ar_coeff_lag
            * ((*pars).ar_coeff_lag + 1 as core::ffi::c_int);
        let mut i_2: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_2 < n {
            fprintf(
                file,
                b" %d\0" as *const u8 as *const core::ffi::c_char,
                (*pars).ar_coeffs_y[i_2 as usize],
            );
            i_2 += 1;
        }
        fprintf(file, b"\n\tcCb\0" as *const u8 as *const core::ffi::c_char);
        let mut i_3: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_3 <= n {
            fprintf(
                file,
                b" %d\0" as *const u8 as *const core::ffi::c_char,
                (*pars).ar_coeffs_cb[i_3 as usize],
            );
            i_3 += 1;
        }
        fprintf(file, b"\n\tcCr\0" as *const u8 as *const core::ffi::c_char);
        let mut i_4: core::ffi::c_int = 0 as core::ffi::c_int;
        while i_4 <= n {
            fprintf(
                file,
                b" %d\0" as *const u8 as *const core::ffi::c_char,
                (*pars).ar_coeffs_cr[i_4 as usize],
            );
            i_4 += 1;
        }
        fprintf(file, b"\n\0" as *const u8 as *const core::ffi::c_char);
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_film_grain_table_append(
    mut t: *mut aom_film_grain_table_t,
    mut time_stamp: int64_t,
    mut end_time: int64_t,
    mut grain: *const aom_film_grain_t,
) {
    if ((*t).tail).is_null()
        || memcmp(
            grain as *const core::ffi::c_void,
            &raw mut (*(*t).tail).params as *mut aom_film_grain_t
                as *const core::ffi::c_void,
            ::core::mem::size_of::<aom_film_grain_t>() as size_t,
        ) != 0
    {
        let mut new_tail: *mut aom_film_grain_table_entry_t = aom_malloc(
            ::core::mem::size_of::<aom_film_grain_table_entry_t>() as size_t,
        ) as *mut aom_film_grain_table_entry_t;
        if new_tail.is_null() {
            return;
        }
        memset(
            new_tail as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_film_grain_table_entry_t>() as size_t,
        );
        if !((*t).tail).is_null() {
            (*(*t).tail).next = new_tail as *mut aom_film_grain_table_entry_t;
        }
        if ((*t).head).is_null() {
            (*t).head = new_tail;
        }
        (*t).tail = new_tail;
        (*new_tail).start_time = time_stamp;
        (*new_tail).end_time = end_time;
        (*new_tail).params = *grain;
    } else {
        (*(*t).tail).end_time = if (*(*t).tail).end_time > end_time {
            (*(*t).tail).end_time
        } else {
            end_time
        };
        (*(*t).tail).start_time = if (*(*t).tail).start_time < time_stamp {
            (*(*t).tail).start_time
        } else {
            time_stamp
        };
    };
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_film_grain_table_lookup(
    mut t: *mut aom_film_grain_table_t,
    mut time_stamp: int64_t,
    mut end_time: int64_t,
    mut erase: core::ffi::c_int,
    mut grain: *mut aom_film_grain_t,
) -> core::ffi::c_int {
    let mut entry: *mut aom_film_grain_table_entry_t = (*t).head;
    let mut prev_entry: *mut aom_film_grain_table_entry_t = 0
        as *mut aom_film_grain_table_entry_t;
    let mut random_seed: uint16_t = (if !grain.is_null() {
        (*grain).random_seed as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    }) as uint16_t;
    if !grain.is_null() {
        memset(
            grain as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<aom_film_grain_t>() as size_t,
        );
    }
    while !entry.is_null() {
        let mut next: *mut aom_film_grain_table_entry_t = (*entry).next
            as *mut aom_film_grain_table_entry_t;
        if time_stamp >= (*entry).start_time && time_stamp < (*entry).end_time {
            if !grain.is_null() {
                *grain = (*entry).params;
                if time_stamp != 0 as int64_t {
                    (*grain).random_seed = random_seed;
                }
            }
            if erase == 0 {
                return 1 as core::ffi::c_int;
            }
            let entry_end_time: int64_t = (*entry).end_time;
            if time_stamp <= (*entry).start_time && end_time >= (*entry).end_time {
                if (*t).tail == entry {
                    (*t).tail = prev_entry;
                }
                if !prev_entry.is_null() {
                    (*prev_entry).next = (*entry).next;
                } else {
                    (*t).head = (*entry).next as *mut aom_film_grain_table_entry_t;
                }
                aom_free(entry as *mut core::ffi::c_void);
            } else if time_stamp <= (*entry).start_time && end_time < (*entry).end_time {
                (*entry).start_time = end_time;
            } else if time_stamp > (*entry).start_time && end_time >= (*entry).end_time {
                (*entry).end_time = time_stamp;
            } else {
                let mut new_entry: *mut aom_film_grain_table_entry_t = aom_malloc(
                    ::core::mem::size_of::<aom_film_grain_table_entry_t>() as size_t,
                ) as *mut aom_film_grain_table_entry_t;
                if new_entry.is_null() {
                    return 0 as core::ffi::c_int;
                }
                (*new_entry).next = (*entry).next;
                (*new_entry).start_time = end_time;
                (*new_entry).end_time = (*entry).end_time;
                (*new_entry).params = (*entry).params;
                (*entry).next = new_entry as *mut aom_film_grain_table_entry_t;
                (*entry).end_time = time_stamp;
                if (*t).tail == entry {
                    (*t).tail = new_entry;
                }
            }
            if end_time > entry_end_time {
                aom_film_grain_table_lookup(
                    t,
                    entry_end_time,
                    end_time,
                    1 as core::ffi::c_int,
                    0 as *mut aom_film_grain_t,
                );
            }
            return 1 as core::ffi::c_int;
        }
        prev_entry = entry;
        entry = next;
    }
    return 0 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_film_grain_table_read(
    mut t: *mut aom_film_grain_table_t,
    mut filename: *const core::ffi::c_char,
    mut error_info: *mut aom_internal_error_info,
) -> aom_codec_err_t {
    let file: *mut FILE = fopen(
        filename,
        b"rb\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    (*error_info).error_code = AOM_CODEC_OK;
    if file.is_null() {
        (*error_info).error_code = AOM_CODEC_ERROR;
        return AOM_CODEC_ERROR;
    }
    (*error_info).setjmp = 1 as core::ffi::c_int;
    match crate::jump::scope((((*error_info).jmp).as_mut_ptr()).cast(), || {

        let mut magic: [core::ffi::c_char; 9] = [0; 9];
        if fread(
            magic.as_mut_ptr() as *mut core::ffi::c_void,
            9 as size_t,
            1 as size_t,
            file,
        ) == 0
            || memcmp(
                magic.as_mut_ptr() as *const core::ffi::c_void,
                core::ptr::addr_of!(kFileMagic).cast::<core::ffi::c_char>() as *const core::ffi::c_void,
                8 as size_t,
            ) != 0
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to read (or invalid) file magic\0" as *const u8
                    as *const core::ffi::c_char,
            );
        }
        let mut prev_entry: *mut aom_film_grain_table_entry_t = 0
            as *mut aom_film_grain_table_entry_t;
        loop {
            let mut c: core::ffi::c_int = fgetc(file);
            if c == EOF {
                break;
            }
            ungetc(c, file);
            let mut entry: *mut aom_film_grain_table_entry_t = aom_malloc(
                ::core::mem::size_of::<aom_film_grain_table_entry_t>() as size_t,
            ) as *mut aom_film_grain_table_entry_t;
            if entry.is_null() {
                aom_internal_error(
                    error_info,
                    AOM_CODEC_MEM_ERROR,
                    b"Unable to allocate grain table entry\0" as *const u8
                        as *const core::ffi::c_char,
                );
            }
            memset(
                entry as *mut core::ffi::c_void,
                0 as core::ffi::c_int,
                ::core::mem::size_of::<aom_film_grain_table_entry_t>() as size_t,
            );
            (*entry).next = 0 as *mut aom_film_grain_table_entry_t;
            if !prev_entry.is_null() {
                (*prev_entry).next = entry as *mut aom_film_grain_table_entry_t;
            }
            if ((*t).head).is_null() {
                (*t).head = entry;
            }
            (*t).tail = entry;
            prev_entry = entry;
            grain_table_entry_read(file, error_info, entry);
        }
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) | None => (),
}
    fclose(file);
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return (*error_info).error_code;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_film_grain_table_write(
    mut t: *const aom_film_grain_table_t,
    mut filename: *const core::ffi::c_char,
    mut error_info: *mut aom_internal_error_info,
) -> aom_codec_err_t {
    let file: *mut FILE = fopen(
        filename,
        b"wb\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    (*error_info).error_code = AOM_CODEC_OK;
    if file.is_null() {
        (*error_info).error_code = AOM_CODEC_ERROR;
        return AOM_CODEC_ERROR;
    }
    (*error_info).setjmp = 1 as core::ffi::c_int;
    match crate::jump::scope((((*error_info).jmp).as_mut_ptr()).cast(), || {

        if fwrite(
            core::ptr::addr_of!(kFileMagic).cast::<core::ffi::c_char>() as *const core::ffi::c_void,
            8 as size_t,
            1 as size_t,
            file,
        ) == 0
        {
            aom_internal_error(
                error_info,
                AOM_CODEC_ERROR,
                b"Unable to write file magic\0" as *const u8 as *const core::ffi::c_char,
            );
        }
        fprintf(file, b"\n\0" as *const u8 as *const core::ffi::c_char);
        let mut entry: *mut aom_film_grain_table_entry_t = (*t).head;
        while !entry.is_null() {
            grain_table_entry_write(file, entry);
            entry = (*entry).next as *mut aom_film_grain_table_entry_t;
        }
crate::jump::Scope::Done
}) {
Some(crate::jump::Scope::Returned(value)) => return value,
Some(crate::jump::Scope::Done) | None => (),
}
    fclose(file);
    (*error_info).setjmp = 0 as core::ffi::c_int;
    return (*error_info).error_code;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_film_grain_table_free(mut t: *mut aom_film_grain_table_t) {
    let mut entry: *mut aom_film_grain_table_entry_t = (*t).head;
    while !entry.is_null() {
        let mut next: *mut aom_film_grain_table_entry_t = (*entry).next
            as *mut aom_film_grain_table_entry_t;
        aom_free(entry as *mut core::ffi::c_void);
        entry = next;
    }
    memset(
        t as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_film_grain_table_t>() as size_t,
    );
}

extern "C" {
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
    fn fclose(_: *mut FILE) -> core::ffi::c_int;
    fn feof(_: *mut FILE) -> core::ffi::c_int;
    fn fgetc(_: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __mode: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fscanf(_: *mut FILE, _: *const core::ffi::c_char, ...) -> core::ffi::c_int;
    fn fwrite(
        __ptr: *const core::ffi::c_void,
        __size: size_t,
        __nitems: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn ungetc(_: core::ffi::c_int, _: *mut FILE) -> core::ffi::c_int;
}
