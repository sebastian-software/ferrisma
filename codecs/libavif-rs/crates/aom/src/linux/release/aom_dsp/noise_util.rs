/*
 * Copyright (c) 2017, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/noise_util.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn aom_memalign(align: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_malloc(size: size_t) -> *mut core::ffi::c_void;
    fn aom_calloc(num: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn aom_free(memblk: *mut core::ffi::c_void);
    fn aom_fft16x16_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_fft2x2_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_fft32x32_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_fft4x4_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_fft8x8_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_ifft16x16_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_ifft2x2_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_ifft32x32_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_ifft4x4_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
    fn aom_ifft8x8_float_c(
        input: *const core::ffi::c_float,
        temp: *mut core::ffi::c_float,
        output: *mut core::ffi::c_float,
    );
}
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = libc::FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct aom_noise_tx_t {
    pub tx_block: *mut core::ffi::c_float,
    pub temp: *mut core::ffi::c_float,
    pub block_size: core::ffi::c_int,
    pub fft: Option<
        unsafe extern "C-unwind" fn(
            *const core::ffi::c_float,
            *mut core::ffi::c_float,
            *mut core::ffi::c_float,
        ) -> (),
    >,
    pub ifft: Option<
        unsafe extern "C-unwind" fn(
            *const core::ffi::c_float,
            *mut core::ffi::c_float,
            *mut core::ffi::c_float,
        ) -> (),
    >,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_psd_get_default_value(
    mut block_size: core::ffi::c_int,
    mut factor: core::ffi::c_float,
) -> core::ffi::c_float {
    return factor * factor / 10000 as core::ffi::c_int as core::ffi::c_float
        * block_size as core::ffi::c_float * block_size as core::ffi::c_float
        / 8 as core::ffi::c_int as core::ffi::c_float;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_tx_malloc(
    mut block_size: core::ffi::c_int,
) -> *mut aom_noise_tx_t {
    let mut noise_tx: *mut aom_noise_tx_t = aom_malloc(
        ::core::mem::size_of::<aom_noise_tx_t>() as size_t,
    ) as *mut aom_noise_tx_t;
    if noise_tx.is_null() {
        return 0 as *mut aom_noise_tx_t;
    }
    memset(
        noise_tx as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<aom_noise_tx_t>() as size_t,
    );
    match block_size {
        2 => {
            (*noise_tx).fft = Some(
                aom_fft2x2_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
            (*noise_tx).ifft = Some(
                aom_ifft2x2_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
        }
        4 => {
            (*noise_tx).fft = Some(
                aom_fft4x4_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
            (*noise_tx).ifft = Some(
                aom_ifft4x4_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
        }
        8 => {
            (*noise_tx).fft = Some(
                aom_fft8x8_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
            (*noise_tx).ifft = Some(
                aom_ifft8x8_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
        }
        16 => {
            (*noise_tx).fft = Some(
                aom_fft16x16_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
            (*noise_tx).ifft = Some(
                aom_ifft16x16_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
        }
        32 => {
            (*noise_tx).fft = Some(
                aom_fft32x32_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
            (*noise_tx).ifft = Some(
                aom_ifft32x32_float_c
                    as unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
            )
                as Option<
                    unsafe extern "C-unwind" fn(
                        *const core::ffi::c_float,
                        *mut core::ffi::c_float,
                        *mut core::ffi::c_float,
                    ) -> (),
                >;
        }
        _ => {
            aom_free(noise_tx as *mut core::ffi::c_void);
            fprintf(
                stderr,
                b"Unsupported block size %d\n\0" as *const u8
                    as *const core::ffi::c_char,
                block_size,
            );
            return 0 as *mut aom_noise_tx_t;
        }
    }
    (*noise_tx).block_size = block_size;
    (*noise_tx).tx_block = aom_memalign(
        32 as size_t,
        (2 as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul(block_size as size_t)
            .wrapping_mul(block_size as size_t),
    ) as *mut core::ffi::c_float;
    (*noise_tx).temp = aom_memalign(
        32 as size_t,
        (2 as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul(block_size as size_t)
            .wrapping_mul(block_size as size_t),
    ) as *mut core::ffi::c_float;
    if ((*noise_tx).tx_block).is_null() || ((*noise_tx).temp).is_null() {
        aom_noise_tx_free(noise_tx);
        return 0 as *mut aom_noise_tx_t;
    }
    memset(
        (*noise_tx).tx_block as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (2 as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul(block_size as size_t)
            .wrapping_mul(block_size as size_t),
    );
    memset(
        (*noise_tx).temp as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (2 as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_float>() as size_t)
            .wrapping_mul(block_size as size_t)
            .wrapping_mul(block_size as size_t),
    );
    return noise_tx;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_tx_forward(
    mut noise_tx: *mut aom_noise_tx_t,
    mut data: *const core::ffi::c_float,
) {
    ((*noise_tx).fft)
        .expect(
            "non-null function pointer",
        )(data, (*noise_tx).temp, (*noise_tx).tx_block);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_tx_filter(
    mut noise_tx: *mut aom_noise_tx_t,
    mut psd: *const core::ffi::c_float,
) {
    let block_size: core::ffi::c_int = (*noise_tx).block_size;
    let kBeta: core::ffi::c_float = 1.1f32;
    let kEps: core::ffi::c_float = 1e-6f32;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < block_size {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < block_size {
            let mut i: core::ffi::c_int = y * block_size + x;
            let mut c: *mut core::ffi::c_float = ((*noise_tx).tx_block)
                .offset((2 as core::ffi::c_int * i) as isize);
            let c0: core::ffi::c_float = if fabs(
                *c.offset(0 as core::ffi::c_int as isize) as core::ffi::c_double,
            ) as core::ffi::c_float > 1e-8f32
            {
                fabs(*c.offset(0 as core::ffi::c_int as isize) as core::ffi::c_double)
                    as core::ffi::c_float
            } else {
                1e-8f32
            };
            let c1: core::ffi::c_float = if fabs(
                *c.offset(1 as core::ffi::c_int as isize) as core::ffi::c_double,
            ) as core::ffi::c_float > 1e-8f32
            {
                fabs(*c.offset(1 as core::ffi::c_int as isize) as core::ffi::c_double)
                    as core::ffi::c_float
            } else {
                1e-8f32
            };
            let p: core::ffi::c_float = c0 * c0 + c1 * c1;
            if p > kBeta * *psd.offset(i as isize) && p as core::ffi::c_double > 1e-6f64
            {
                *((*noise_tx).tx_block)
                    .offset((2 as core::ffi::c_int * i + 0 as core::ffi::c_int) as isize)
                    *= (p - *psd.offset(i as isize)) / (if p > kEps { p } else { kEps });
                *((*noise_tx).tx_block)
                    .offset((2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as isize)
                    *= (p - *psd.offset(i as isize)) / (if p > kEps { p } else { kEps });
            } else {
                *((*noise_tx).tx_block)
                    .offset((2 as core::ffi::c_int * i + 0 as core::ffi::c_int) as isize)
                    *= (kBeta - 1.0f32) / kBeta;
                *((*noise_tx).tx_block)
                    .offset((2 as core::ffi::c_int * i + 1 as core::ffi::c_int) as isize)
                    *= (kBeta - 1.0f32) / kBeta;
            }
            x += 1;
        }
        y += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_tx_inverse(
    mut noise_tx: *mut aom_noise_tx_t,
    mut data: *mut core::ffi::c_float,
) {
    let n: core::ffi::c_int = (*noise_tx).block_size * (*noise_tx).block_size;
    ((*noise_tx).ifft)
        .expect(
            "non-null function pointer",
        )((*noise_tx).tx_block, (*noise_tx).temp, data);
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n {
        *data.offset(i as isize) /= n as core::ffi::c_float;
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_tx_add_energy(
    mut noise_tx: *const aom_noise_tx_t,
    mut psd: *mut core::ffi::c_float,
) {
    let block_size: core::ffi::c_int = (*noise_tx).block_size;
    let mut yb: core::ffi::c_int = 0 as core::ffi::c_int;
    while yb < block_size {
        let mut xb: core::ffi::c_int = 0 as core::ffi::c_int;
        while xb <= block_size / 2 as core::ffi::c_int {
            let mut c: *mut core::ffi::c_float = ((*noise_tx).tx_block)
                .offset((2 as core::ffi::c_int * (yb * block_size + xb)) as isize);
            *psd.offset((yb * block_size + xb) as isize)
                += *c.offset(0 as core::ffi::c_int as isize)
                    * *c.offset(0 as core::ffi::c_int as isize)
                    + *c.offset(1 as core::ffi::c_int as isize)
                        * *c.offset(1 as core::ffi::c_int as isize);
            xb += 1;
        }
        yb += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_tx_free(mut noise_tx: *mut aom_noise_tx_t) {
    if noise_tx.is_null() {
        return;
    }
    aom_free((*noise_tx).tx_block as *mut core::ffi::c_void);
    aom_free((*noise_tx).temp as *mut core::ffi::c_void);
    aom_free(noise_tx as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_normalized_cross_correlation(
    mut a: *const core::ffi::c_double,
    mut b: *const core::ffi::c_double,
    mut n: core::ffi::c_int,
) -> core::ffi::c_double {
    let mut c: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let mut a_len: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let mut b_len: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < n {
        a_len += *a.offset(i as isize) * *a.offset(i as isize);
        b_len += *b.offset(i as isize) * *b.offset(i as isize);
        c += *a.offset(i as isize) * *b.offset(i as isize);
        i += 1;
    }
    return c / (sqrt(a_len) * sqrt(b_len));
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_noise_data_validate(
    mut data: *const core::ffi::c_double,
    mut w: core::ffi::c_int,
    mut h: core::ffi::c_int,
) -> core::ffi::c_int {
    let kVarianceThreshold: core::ffi::c_double = 2 as core::ffi::c_int
        as core::ffi::c_double;
    let kMeanThreshold: core::ffi::c_double = 2 as core::ffi::c_int
        as core::ffi::c_double;
    let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ret_value: core::ffi::c_int = 1 as core::ffi::c_int;
    let mut var: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let mut mean: core::ffi::c_double = 0 as core::ffi::c_int as core::ffi::c_double;
    let mut mean_x: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut mean_y: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut var_x: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    let mut var_y: *mut core::ffi::c_double = 0 as *mut core::ffi::c_double;
    mean_x = aom_calloc(
        w as size_t,
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    var_x = aom_calloc(
        w as size_t,
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    mean_y = aom_calloc(
        h as size_t,
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    var_y = aom_calloc(
        h as size_t,
        ::core::mem::size_of::<core::ffi::c_double>() as size_t,
    ) as *mut core::ffi::c_double;
    if !(!mean_x.is_null() && !var_x.is_null() && !mean_y.is_null() && !var_y.is_null())
    {
        aom_free(mean_x as *mut core::ffi::c_void);
        aom_free(mean_y as *mut core::ffi::c_void);
        aom_free(var_x as *mut core::ffi::c_void);
        aom_free(var_y as *mut core::ffi::c_void);
        return 0 as core::ffi::c_int;
    }
    y = 0 as core::ffi::c_int;
    while y < h {
        x = 0 as core::ffi::c_int;
        while x < w {
            let d: core::ffi::c_double = *data.offset((y * w + x) as isize);
            *var_x.offset(x as isize) += d * d;
            *var_y.offset(y as isize) += d * d;
            *mean_x.offset(x as isize) += d;
            *mean_y.offset(y as isize) += d;
            var += d * d;
            mean += d;
            x += 1;
        }
        y += 1;
    }
    mean /= (w * h) as core::ffi::c_double;
    var = var / (w * h) as core::ffi::c_double - mean * mean;
    y = 0 as core::ffi::c_int;
    while y < h {
        *mean_y.offset(y as isize) /= h as core::ffi::c_double;
        *var_y.offset(y as isize) = *var_y.offset(y as isize) / h as core::ffi::c_double
            - *mean_y.offset(y as isize) * *mean_y.offset(y as isize);
        if fabs(*var_y.offset(y as isize) - var) >= kVarianceThreshold {
            fprintf(
                stderr,
                b"Variance distance too large %f %f\n\0" as *const u8
                    as *const core::ffi::c_char,
                *var_y.offset(y as isize),
                var,
            );
            ret_value = 0 as core::ffi::c_int;
            break;
        } else if fabs(*mean_y.offset(y as isize) - mean) >= kMeanThreshold {
            fprintf(
                stderr,
                b"Mean distance too large %f %f\n\0" as *const u8
                    as *const core::ffi::c_char,
                *mean_y.offset(y as isize),
                mean,
            );
            ret_value = 0 as core::ffi::c_int;
            break;
        } else {
            y += 1;
        }
    }
    x = 0 as core::ffi::c_int;
    while x < w {
        *mean_x.offset(x as isize) /= w as core::ffi::c_double;
        *var_x.offset(x as isize) = *var_x.offset(x as isize) / w as core::ffi::c_double
            - *mean_x.offset(x as isize) * *mean_x.offset(x as isize);
        if fabs(*var_x.offset(x as isize) - var) >= kVarianceThreshold {
            fprintf(
                stderr,
                b"Variance distance too large %f %f\n\0" as *const u8
                    as *const core::ffi::c_char,
                *var_x.offset(x as isize),
                var,
            );
            ret_value = 0 as core::ffi::c_int;
            break;
        } else if fabs(*mean_x.offset(x as isize) - mean) >= kMeanThreshold {
            fprintf(
                stderr,
                b"Mean distance too large %f %f\n\0" as *const u8
                    as *const core::ffi::c_char,
                *mean_x.offset(x as isize),
                mean,
            );
            ret_value = 0 as core::ffi::c_int;
            break;
        } else {
            x += 1;
        }
    }
    aom_free(mean_x as *mut core::ffi::c_void);
    aom_free(mean_y as *mut core::ffi::c_void);
    aom_free(var_x as *mut core::ffi::c_void);
    aom_free(var_y as *mut core::ffi::c_void);
    return ret_value;
}

extern "C" {
    fn sqrt(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn fprintf(
        __stream: *mut FILE,
        __format: *const core::ffi::c_char,
        ...
    ) -> core::ffi::c_int;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
