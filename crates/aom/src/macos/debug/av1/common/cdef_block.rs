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

// Mechanically translated from upstream/aom/av1/common/cdef_block.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static mut cdef_filter_16_0: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_filter_16_1: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_filter_16_2: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_filter_16_3: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_filter_8_0: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_filter_8_1: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_filter_8_2: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_filter_8_3: Option<
        unsafe extern "C-unwind" fn(
            *mut core::ffi::c_void,
            core::ffi::c_int,
            *const uint16_t,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
            core::ffi::c_int,
        ) -> (),
    >;
    static mut cdef_find_dir: Option<
        unsafe extern "C-unwind" fn(
            *const uint16_t,
            core::ffi::c_int,
            *mut int32_t,
            core::ffi::c_int,
        ) -> core::ffi::c_int,
    >;
    static mut cdef_find_dir_dual: Option<
        unsafe extern "C-unwind" fn(
            *const uint16_t,
            *const uint16_t,
            core::ffi::c_int,
            *mut int32_t,
            *mut int32_t,
            core::ffi::c_int,
            *mut core::ffi::c_int,
            *mut core::ffi::c_int,
        ) -> (),
    >;
}
pub type __darwin_size_t = usize;
pub type int16_t = i16;
pub type int32_t = i32;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type uint16_t = u16;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct cdef_list {
    pub by: uint8_t,
    pub bx: uint8_t,
}
pub type cdef_filter_block_func = Option<
    unsafe extern "C-unwind" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        *const uint16_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[inline]
unsafe extern "C-unwind" fn clamp(
    mut value: core::ffi::c_int,
    mut low: core::ffi::c_int,
    mut high: core::ffi::c_int,
) -> core::ffi::c_int {
    return if value < low { low } else if value > high { high } else { value };
}
pub const AOM_PLANE_Y: core::ffi::c_int = 0 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    if !(n != 0 as core::ffi::c_uint) as core::ffi::c_int as core::ffi::c_long != 0 {
        crate::platform::assertion(
            (::core::mem::transmute::<[u8; 8], [core::ffi::c_char; 8]>(*b"get_msb\0"))
                .as_ptr(),
            b"bitops.h\0" as *const u8 as *const core::ffi::c_char,
            43 as core::ffi::c_int,
            b"n != 0\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
pub const CDEF_BSTRIDE: core::ffi::c_int = ((1 as core::ffi::c_int)
    << 7 as core::ffi::c_int) + 2 as core::ffi::c_int * 8 as core::ffi::c_int
    + (((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int)
    & !(((1 as core::ffi::c_int) << 3 as core::ffi::c_int) - 1 as core::ffi::c_int);
pub const CDEF_VERY_LARGE: core::ffi::c_int = 0x4000 as core::ffi::c_int;
#[inline]
unsafe extern "C-unwind" fn sign(mut i: core::ffi::c_int) -> core::ffi::c_int {
    return if i < 0 as core::ffi::c_int {
        -(1 as core::ffi::c_int)
    } else {
        1 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn constrain(
    mut diff: core::ffi::c_int,
    mut threshold: core::ffi::c_int,
    mut damping: core::ffi::c_int,
) -> core::ffi::c_int {
    if threshold == 0 {
        return 0 as core::ffi::c_int;
    }
    let mut shift: core::ffi::c_int = damping - get_msb(threshold as core::ffi::c_uint);
    shift = if 0 as core::ffi::c_int > shift { 0 as core::ffi::c_int } else { shift };
    return sign(diff)
        * clamp(threshold - (abs(diff) >> shift), 0 as core::ffi::c_int, abs(diff));
}
static mut cdef_directions_padded: crate::platform::Aligned16<[[core::ffi::c_int; 2]; 12]> = crate::platform::Aligned16([
    [
        1 as core::ffi::c_int * CDEF_BSTRIDE + 0 as core::ffi::c_int,
        2 as core::ffi::c_int * CDEF_BSTRIDE + 0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int * CDEF_BSTRIDE + 0 as core::ffi::c_int,
        2 as core::ffi::c_int * CDEF_BSTRIDE - 1 as core::ffi::c_int,
    ],
    [
        -(1 as core::ffi::c_int) * CDEF_BSTRIDE + 1 as core::ffi::c_int,
        -(2 as core::ffi::c_int) * CDEF_BSTRIDE + 2 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int * CDEF_BSTRIDE + 1 as core::ffi::c_int,
        -(1 as core::ffi::c_int) * CDEF_BSTRIDE + 2 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int * CDEF_BSTRIDE + 1 as core::ffi::c_int,
        0 as core::ffi::c_int * CDEF_BSTRIDE + 2 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int * CDEF_BSTRIDE + 1 as core::ffi::c_int,
        1 as core::ffi::c_int * CDEF_BSTRIDE + 2 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int * CDEF_BSTRIDE + 1 as core::ffi::c_int,
        2 as core::ffi::c_int * CDEF_BSTRIDE + 2 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int * CDEF_BSTRIDE + 0 as core::ffi::c_int,
        2 as core::ffi::c_int * CDEF_BSTRIDE + 1 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int * CDEF_BSTRIDE + 0 as core::ffi::c_int,
        2 as core::ffi::c_int * CDEF_BSTRIDE + 0 as core::ffi::c_int,
    ],
    [
        1 as core::ffi::c_int * CDEF_BSTRIDE + 0 as core::ffi::c_int,
        2 as core::ffi::c_int * CDEF_BSTRIDE - 1 as core::ffi::c_int,
    ],
    [
        -(1 as core::ffi::c_int) * CDEF_BSTRIDE + 1 as core::ffi::c_int,
        -(2 as core::ffi::c_int) * CDEF_BSTRIDE + 2 as core::ffi::c_int,
    ],
    [
        0 as core::ffi::c_int * CDEF_BSTRIDE + 1 as core::ffi::c_int,
        -(1 as core::ffi::c_int) * CDEF_BSTRIDE + 2 as core::ffi::c_int,
    ],
]);
#[no_mangle]
pub static mut cdef_directions: *const [core::ffi::c_int; 2] = 0
    as *const [core::ffi::c_int; 2];
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_find_dir_c(
    mut img: *const uint16_t,
    mut stride: core::ffi::c_int,
    mut var: *mut int32_t,
    mut coeff_shift: core::ffi::c_int,
) -> core::ffi::c_int {
    let mut i: core::ffi::c_int = 0;
    let mut cost: [int32_t; 8] = [0 as core::ffi::c_int; 8];
    let mut partial: [[core::ffi::c_int; 15]; 8] = [
        [0 as core::ffi::c_int; 15],
        [0; 15],
        [0; 15],
        [0; 15],
        [0; 15],
        [0; 15],
        [0; 15],
        [0; 15],
    ];
    let mut best_cost: int32_t = 0 as int32_t;
    let mut best_dir: core::ffi::c_int = 0 as core::ffi::c_int;
    static mut div_table: [core::ffi::c_int; 9] = [
        0 as core::ffi::c_int,
        840 as core::ffi::c_int,
        420 as core::ffi::c_int,
        280 as core::ffi::c_int,
        210 as core::ffi::c_int,
        168 as core::ffi::c_int,
        140 as core::ffi::c_int,
        120 as core::ffi::c_int,
        105 as core::ffi::c_int,
    ];
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        let mut j: core::ffi::c_int = 0;
        j = 0 as core::ffi::c_int;
        while j < 8 as core::ffi::c_int {
            let mut x: core::ffi::c_int = 0;
            x = (*img.offset((i * stride + j) as isize) as core::ffi::c_int
                >> coeff_shift) - 128 as core::ffi::c_int;
            partial[0 as core::ffi::c_int as usize][(i + j) as usize] += x;
            partial[1 as core::ffi::c_int
                as usize][(i + j / 2 as core::ffi::c_int) as usize] += x;
            partial[2 as core::ffi::c_int as usize][i as usize] += x;
            partial[3 as core::ffi::c_int
                as usize][(3 as core::ffi::c_int + i - j / 2 as core::ffi::c_int)
                as usize] += x;
            partial[4 as core::ffi::c_int
                as usize][(7 as core::ffi::c_int + i - j) as usize] += x;
            partial[5 as core::ffi::c_int
                as usize][(3 as core::ffi::c_int - i / 2 as core::ffi::c_int + j)
                as usize] += x;
            partial[6 as core::ffi::c_int as usize][j as usize] += x;
            partial[7 as core::ffi::c_int
                as usize][(i / 2 as core::ffi::c_int + j) as usize] += x;
            j += 1;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        cost[2 as core::ffi::c_int as usize] = (cost[2 as core::ffi::c_int as usize]
            as core::ffi::c_int
            + partial[2 as core::ffi::c_int as usize][i as usize]
                * partial[2 as core::ffi::c_int as usize][i as usize]) as int32_t;
        cost[6 as core::ffi::c_int as usize] = (cost[6 as core::ffi::c_int as usize]
            as core::ffi::c_int
            + partial[6 as core::ffi::c_int as usize][i as usize]
                * partial[6 as core::ffi::c_int as usize][i as usize]) as int32_t;
        i += 1;
    }
    cost[2 as core::ffi::c_int as usize] = (cost[2 as core::ffi::c_int as usize]
        as core::ffi::c_int * div_table[8 as core::ffi::c_int as usize]) as int32_t;
    cost[6 as core::ffi::c_int as usize] = (cost[6 as core::ffi::c_int as usize]
        as core::ffi::c_int * div_table[8 as core::ffi::c_int as usize]) as int32_t;
    i = 0 as core::ffi::c_int;
    while i < 7 as core::ffi::c_int {
        cost[0 as core::ffi::c_int as usize] = (cost[0 as core::ffi::c_int as usize]
            as core::ffi::c_int
            + (partial[0 as core::ffi::c_int as usize][i as usize]
                * partial[0 as core::ffi::c_int as usize][i as usize]
                + partial[0 as core::ffi::c_int
                    as usize][(14 as core::ffi::c_int - i) as usize]
                    * partial[0 as core::ffi::c_int
                        as usize][(14 as core::ffi::c_int - i) as usize])
                * div_table[(i + 1 as core::ffi::c_int) as usize]) as int32_t;
        cost[4 as core::ffi::c_int as usize] = (cost[4 as core::ffi::c_int as usize]
            as core::ffi::c_int
            + (partial[4 as core::ffi::c_int as usize][i as usize]
                * partial[4 as core::ffi::c_int as usize][i as usize]
                + partial[4 as core::ffi::c_int
                    as usize][(14 as core::ffi::c_int - i) as usize]
                    * partial[4 as core::ffi::c_int
                        as usize][(14 as core::ffi::c_int - i) as usize])
                * div_table[(i + 1 as core::ffi::c_int) as usize]) as int32_t;
        i += 1;
    }
    cost[0 as core::ffi::c_int as usize] = (cost[0 as core::ffi::c_int as usize]
        as core::ffi::c_int
        + partial[0 as core::ffi::c_int as usize][7 as core::ffi::c_int as usize]
            * partial[0 as core::ffi::c_int as usize][7 as core::ffi::c_int as usize]
            * div_table[8 as core::ffi::c_int as usize]) as int32_t;
    cost[4 as core::ffi::c_int as usize] = (cost[4 as core::ffi::c_int as usize]
        as core::ffi::c_int
        + partial[4 as core::ffi::c_int as usize][7 as core::ffi::c_int as usize]
            * partial[4 as core::ffi::c_int as usize][7 as core::ffi::c_int as usize]
            * div_table[8 as core::ffi::c_int as usize]) as int32_t;
    i = 1 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        let mut j_0: core::ffi::c_int = 0;
        j_0 = 0 as core::ffi::c_int;
        while j_0 < 4 as core::ffi::c_int + 1 as core::ffi::c_int {
            cost[i as usize] = (cost[i as usize] as core::ffi::c_int
                + partial[i as usize][(3 as core::ffi::c_int + j_0) as usize]
                    * partial[i as usize][(3 as core::ffi::c_int + j_0) as usize])
                as int32_t;
            j_0 += 1;
        }
        cost[i as usize] = (cost[i as usize] as core::ffi::c_int
            * div_table[8 as core::ffi::c_int as usize]) as int32_t;
        j_0 = 0 as core::ffi::c_int;
        while j_0 < 4 as core::ffi::c_int - 1 as core::ffi::c_int {
            cost[i as usize] = (cost[i as usize] as core::ffi::c_int
                + (partial[i as usize][j_0 as usize] * partial[i as usize][j_0 as usize]
                    + partial[i as usize][(10 as core::ffi::c_int - j_0) as usize]
                        * partial[i as usize][(10 as core::ffi::c_int - j_0) as usize])
                    * div_table[(2 as core::ffi::c_int * j_0 + 2 as core::ffi::c_int)
                        as usize]) as int32_t;
            j_0 += 1;
        }
        i += 2 as core::ffi::c_int;
    }
    i = 0 as core::ffi::c_int;
    while i < 8 as core::ffi::c_int {
        if cost[i as usize] > best_cost {
            best_cost = cost[i as usize];
            best_dir = i;
        }
        i += 1;
    }
    *var = best_cost
        - cost[(best_dir + 4 as core::ffi::c_int & 7 as core::ffi::c_int) as usize];
    *var >>= 10 as core::ffi::c_int;
    return best_dir;
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_find_dir_dual_c(
    mut img1: *const uint16_t,
    mut img2: *const uint16_t,
    mut stride: core::ffi::c_int,
    mut var1: *mut int32_t,
    mut var2: *mut int32_t,
    mut coeff_shift: core::ffi::c_int,
    mut out1: *mut core::ffi::c_int,
    mut out2: *mut core::ffi::c_int,
) {
    *out1 = cdef_find_dir_c(img1, stride, var1, coeff_shift);
    *out2 = cdef_find_dir_c(img2, stride, var2, coeff_shift);
}
#[no_mangle]
pub static mut cdef_pri_taps: [[core::ffi::c_int; 2]; 2] = [
    [4 as core::ffi::c_int, 2 as core::ffi::c_int],
    [3 as core::ffi::c_int, 3 as core::ffi::c_int],
];
#[no_mangle]
pub static mut cdef_sec_taps: [core::ffi::c_int; 2] = [
    2 as core::ffi::c_int,
    1 as core::ffi::c_int,
];
unsafe extern "C-unwind" fn cdef_filter_block_internal(
    mut dst8: *mut uint8_t,
    mut dst16: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
    mut enable_primary: core::ffi::c_int,
    mut enable_secondary: core::ffi::c_int,
) {
    let clipping_required: core::ffi::c_int = (enable_primary != 0
        && enable_secondary != 0) as core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut k: core::ffi::c_int = 0;
    let s: core::ffi::c_int = CDEF_BSTRIDE;
    let mut pri_taps: *const core::ffi::c_int = (cdef_pri_taps[(pri_strength
        >> coeff_shift & 1 as core::ffi::c_int) as usize])
        .as_ptr();
    let mut sec_taps: *const core::ffi::c_int = cdef_sec_taps.as_ptr();
    i = 0 as core::ffi::c_int;
    while i < block_height {
        j = 0 as core::ffi::c_int;
        while j < block_width {
            let mut sum: int16_t = 0 as int16_t;
            let mut y: int16_t = 0;
            let mut x: int16_t = *in_0.offset((i * s + j) as isize) as int16_t;
            let mut max: core::ffi::c_int = x as core::ffi::c_int;
            let mut min: core::ffi::c_int = x as core::ffi::c_int;
            k = 0 as core::ffi::c_int;
            while k < 2 as core::ffi::c_int {
                if enable_primary != 0 {
                    let mut p0: int16_t = *in_0
                        .offset(
                            (i * s + j
                                + (*cdef_directions.offset(dir as isize))[k as usize])
                                as isize,
                        ) as int16_t;
                    let mut p1: int16_t = *in_0
                        .offset(
                            (i * s + j
                                - (*cdef_directions.offset(dir as isize))[k as usize])
                                as isize,
                        ) as int16_t;
                    sum = (sum as core::ffi::c_int
                        + *pri_taps.offset(k as isize)
                            * constrain(
                                p0 as core::ffi::c_int - x as core::ffi::c_int,
                                pri_strength,
                                pri_damping,
                            )) as int16_t;
                    sum = (sum as core::ffi::c_int
                        + *pri_taps.offset(k as isize)
                            * constrain(
                                p1 as core::ffi::c_int - x as core::ffi::c_int,
                                pri_strength,
                                pri_damping,
                            )) as int16_t;
                    if clipping_required != 0 {
                        if p0 as core::ffi::c_int != CDEF_VERY_LARGE {
                            max = if p0 as core::ffi::c_int > max {
                                p0 as core::ffi::c_int
                            } else {
                                max
                            };
                        }
                        if p1 as core::ffi::c_int != CDEF_VERY_LARGE {
                            max = if p1 as core::ffi::c_int > max {
                                p1 as core::ffi::c_int
                            } else {
                                max
                            };
                        }
                        min = if (p0 as core::ffi::c_int) < min {
                            p0 as core::ffi::c_int
                        } else {
                            min
                        };
                        min = if (p1 as core::ffi::c_int) < min {
                            p1 as core::ffi::c_int
                        } else {
                            min
                        };
                    }
                }
                if enable_secondary != 0 {
                    let mut s0: int16_t = *in_0
                        .offset(
                            (i * s + j
                                + (*cdef_directions
                                    .offset(
                                        (dir + 2 as core::ffi::c_int) as isize,
                                    ))[k as usize]) as isize,
                        ) as int16_t;
                    let mut s1: int16_t = *in_0
                        .offset(
                            (i * s + j
                                - (*cdef_directions
                                    .offset(
                                        (dir + 2 as core::ffi::c_int) as isize,
                                    ))[k as usize]) as isize,
                        ) as int16_t;
                    let mut s2: int16_t = *in_0
                        .offset(
                            (i * s + j
                                + (*cdef_directions
                                    .offset(
                                        (dir - 2 as core::ffi::c_int) as isize,
                                    ))[k as usize]) as isize,
                        ) as int16_t;
                    let mut s3: int16_t = *in_0
                        .offset(
                            (i * s + j
                                - (*cdef_directions
                                    .offset(
                                        (dir - 2 as core::ffi::c_int) as isize,
                                    ))[k as usize]) as isize,
                        ) as int16_t;
                    if clipping_required != 0 {
                        if s0 as core::ffi::c_int != CDEF_VERY_LARGE {
                            max = if s0 as core::ffi::c_int > max {
                                s0 as core::ffi::c_int
                            } else {
                                max
                            };
                        }
                        if s1 as core::ffi::c_int != CDEF_VERY_LARGE {
                            max = if s1 as core::ffi::c_int > max {
                                s1 as core::ffi::c_int
                            } else {
                                max
                            };
                        }
                        if s2 as core::ffi::c_int != CDEF_VERY_LARGE {
                            max = if s2 as core::ffi::c_int > max {
                                s2 as core::ffi::c_int
                            } else {
                                max
                            };
                        }
                        if s3 as core::ffi::c_int != CDEF_VERY_LARGE {
                            max = if s3 as core::ffi::c_int > max {
                                s3 as core::ffi::c_int
                            } else {
                                max
                            };
                        }
                        min = if (s0 as core::ffi::c_int) < min {
                            s0 as core::ffi::c_int
                        } else {
                            min
                        };
                        min = if (s1 as core::ffi::c_int) < min {
                            s1 as core::ffi::c_int
                        } else {
                            min
                        };
                        min = if (s2 as core::ffi::c_int) < min {
                            s2 as core::ffi::c_int
                        } else {
                            min
                        };
                        min = if (s3 as core::ffi::c_int) < min {
                            s3 as core::ffi::c_int
                        } else {
                            min
                        };
                    }
                    sum = (sum as core::ffi::c_int
                        + *sec_taps.offset(k as isize)
                            * constrain(
                                s0 as core::ffi::c_int - x as core::ffi::c_int,
                                sec_strength,
                                sec_damping,
                            )) as int16_t;
                    sum = (sum as core::ffi::c_int
                        + *sec_taps.offset(k as isize)
                            * constrain(
                                s1 as core::ffi::c_int - x as core::ffi::c_int,
                                sec_strength,
                                sec_damping,
                            )) as int16_t;
                    sum = (sum as core::ffi::c_int
                        + *sec_taps.offset(k as isize)
                            * constrain(
                                s2 as core::ffi::c_int - x as core::ffi::c_int,
                                sec_strength,
                                sec_damping,
                            )) as int16_t;
                    sum = (sum as core::ffi::c_int
                        + *sec_taps.offset(k as isize)
                            * constrain(
                                s3 as core::ffi::c_int - x as core::ffi::c_int,
                                sec_strength,
                                sec_damping,
                            )) as int16_t;
                }
                k += 1;
            }
            y = (x as core::ffi::c_int
                + (8 as core::ffi::c_int + sum as core::ffi::c_int
                    - ((sum as core::ffi::c_int) < 0 as core::ffi::c_int)
                        as core::ffi::c_int >> 4 as core::ffi::c_int)) as int16_t;
            if clipping_required != 0 {
                y = clamp(y as core::ffi::c_int, min, max) as int16_t;
            }
            if !dst8.is_null() {
                *dst8.offset((i * dstride + j) as isize) = y as uint8_t;
            } else {
                *dst16.offset((i * dstride + j) as isize) = y as uint16_t;
            }
            j += 1;
        }
        i += 1;
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_0_c(
    mut dst8: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        dst8 as *mut uint8_t,
        0 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_1_c(
    mut dst8: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        dst8 as *mut uint8_t,
        0 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_2_c(
    mut dst8: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        dst8 as *mut uint8_t,
        0 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_8_3_c(
    mut dst8: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        dst8 as *mut uint8_t,
        0 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_0_c(
    mut dst16: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        0 as *mut uint8_t,
        dst16 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        1 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_1_c(
    mut dst16: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        0 as *mut uint8_t,
        dst16 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_2_c(
    mut dst16: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        0 as *mut uint8_t,
        dst16 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        0 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
}
#[no_mangle]
pub unsafe extern "C-unwind" fn cdef_filter_16_3_c(
    mut dst16: *mut core::ffi::c_void,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut pri_strength: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut dir: core::ffi::c_int,
    mut pri_damping: core::ffi::c_int,
    mut sec_damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut block_width: core::ffi::c_int,
    mut block_height: core::ffi::c_int,
) {
    cdef_filter_block_internal(
        0 as *mut uint8_t,
        dst16 as *mut uint16_t,
        dstride,
        in_0,
        pri_strength,
        sec_strength,
        dir,
        pri_damping,
        sec_damping,
        coeff_shift,
        block_width,
        block_height,
        0 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
}
#[inline]
unsafe extern "C-unwind" fn adjust_strength(
    mut strength: core::ffi::c_int,
    mut var: int32_t,
) -> core::ffi::c_int {
    let i: core::ffi::c_int = if var >> 6 as core::ffi::c_int != 0 {
        if get_msb((var >> 6 as core::ffi::c_int) as core::ffi::c_uint)
            < 12 as core::ffi::c_int
        {
            get_msb((var >> 6 as core::ffi::c_int) as core::ffi::c_uint)
                as core::ffi::c_int
        } else {
            12 as core::ffi::c_int
        }
    } else {
        0 as core::ffi::c_int
    };
    return if var != 0 {
        strength * (4 as core::ffi::c_int + i) + 8 as core::ffi::c_int
            >> 4 as core::ffi::c_int
    } else {
        0 as core::ffi::c_int
    };
}
#[inline]
unsafe extern "C-unwind" fn aom_cdef_find_dir(
    mut in_0: *const uint16_t,
    mut dlist: *mut cdef_list,
    mut var: *mut [core::ffi::c_int; 16],
    mut cdef_count: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
    mut dir: *mut [core::ffi::c_int; 16],
) {
    let mut bi: core::ffi::c_int = 0;
    bi = 0 as core::ffi::c_int;
    while bi < cdef_count - 1 as core::ffi::c_int {
        let by: core::ffi::c_int = (*dlist.offset(bi as isize)).by as core::ffi::c_int;
        let bx: core::ffi::c_int = (*dlist.offset(bi as isize)).bx as core::ffi::c_int;
        let by2: core::ffi::c_int = (*dlist
            .offset((bi + 1 as core::ffi::c_int) as isize))
            .by as core::ffi::c_int;
        let bx2: core::ffi::c_int = (*dlist
            .offset((bi + 1 as core::ffi::c_int) as isize))
            .bx as core::ffi::c_int;
        let pos1: core::ffi::c_int = 8 as core::ffi::c_int * by * CDEF_BSTRIDE
            + 8 as core::ffi::c_int * bx;
        let pos2: core::ffi::c_int = 8 as core::ffi::c_int * by2 * CDEF_BSTRIDE
            + 8 as core::ffi::c_int * bx2;
        cdef_find_dir_dual
            .expect(
                "non-null function pointer",
            )(
            &*in_0.offset(pos1 as isize),
            &*in_0.offset(pos2 as isize),
            CDEF_BSTRIDE,
            &raw mut *(*var.offset(by as isize)).as_mut_ptr().offset(bx as isize),
            &raw mut *(*var.offset(by2 as isize)).as_mut_ptr().offset(bx2 as isize),
            coeff_shift,
            &raw mut *(*dir.offset(by as isize)).as_mut_ptr().offset(bx as isize),
            &raw mut *(*dir.offset(by2 as isize)).as_mut_ptr().offset(bx2 as isize),
        );
        bi += 2 as core::ffi::c_int;
    }
    if cdef_count % 2 as core::ffi::c_int != 0 {
        let by_0: core::ffi::c_int = (*dlist.offset(bi as isize)).by as core::ffi::c_int;
        let bx_0: core::ffi::c_int = (*dlist.offset(bi as isize)).bx as core::ffi::c_int;
        (*dir.offset(by_0 as isize))[bx_0 as usize] = cdef_find_dir
            .expect(
                "non-null function pointer",
            )(
            &*in_0
                .offset(
                    (8 as core::ffi::c_int * by_0 * CDEF_BSTRIDE
                        + 8 as core::ffi::c_int * bx_0) as isize,
                ),
            CDEF_BSTRIDE,
            &raw mut *(*var.offset(by_0 as isize)).as_mut_ptr().offset(bx_0 as isize),
            coeff_shift,
        );
    }
}
#[no_mangle]
pub unsafe extern "C-unwind" fn av1_cdef_filter_fb(
    mut dst8: *mut uint8_t,
    mut dst16: *mut uint16_t,
    mut dstride: core::ffi::c_int,
    mut in_0: *const uint16_t,
    mut xdec: core::ffi::c_int,
    mut ydec: core::ffi::c_int,
    mut dir: *mut [core::ffi::c_int; 16],
    mut dirinit: *mut core::ffi::c_int,
    mut var: *mut [core::ffi::c_int; 16],
    mut pli: core::ffi::c_int,
    mut dlist: *mut cdef_list,
    mut cdef_count: core::ffi::c_int,
    mut level: core::ffi::c_int,
    mut sec_strength: core::ffi::c_int,
    mut damping: core::ffi::c_int,
    mut coeff_shift: core::ffi::c_int,
) {
    let mut bi: core::ffi::c_int = 0;
    let mut bx: core::ffi::c_int = 0;
    let mut by: core::ffi::c_int = 0;
    let pri_strength: core::ffi::c_int = level << coeff_shift;
    sec_strength <<= coeff_shift;
    damping += coeff_shift - (pli != AOM_PLANE_Y) as core::ffi::c_int;
    let bw_log2: core::ffi::c_int = 3 as core::ffi::c_int - xdec;
    let bh_log2: core::ffi::c_int = 3 as core::ffi::c_int - ydec;
    if !dirinit.is_null() && pri_strength == 0 as core::ffi::c_int
        && sec_strength == 0 as core::ffi::c_int
    {
        bi = 0 as core::ffi::c_int;
        while bi < cdef_count {
            by = (*dlist.offset(bi as isize)).by as core::ffi::c_int;
            bx = (*dlist.offset(bi as isize)).bx as core::ffi::c_int;
            let mut iy: core::ffi::c_int = 0 as core::ffi::c_int;
            while iy < (1 as core::ffi::c_int) << bh_log2 {
                memcpy(
                    &raw mut *dst16
                        .offset(((bi << bw_log2 + bh_log2) + (iy << bw_log2)) as isize)
                        as *mut uint16_t as *mut core::ffi::c_void,
                    &*in_0
                        .offset(
                            (((by << bh_log2) + iy) * CDEF_BSTRIDE + (bx << bw_log2))
                                as isize,
                        ) as *const uint16_t as *const core::ffi::c_void,
                    ((1 as core::ffi::c_int as size_t) << bw_log2)
                        .wrapping_mul(::core::mem::size_of::<uint16_t>() as size_t),
                );
                iy += 1;
            }
            bi += 1;
        }
        return;
    }
    if pli == 0 as core::ffi::c_int {
        if dirinit.is_null() || *dirinit == 0 {
            aom_cdef_find_dir(in_0, dlist, var, cdef_count, coeff_shift, dir);
            if !dirinit.is_null() {
                *dirinit = 1 as core::ffi::c_int;
            }
        }
    }
    if pli == 1 as core::ffi::c_int && xdec != ydec {
        bi = 0 as core::ffi::c_int;
        while bi < cdef_count {
            static mut conv422: [core::ffi::c_int; 8] = [
                7 as core::ffi::c_int,
                0 as core::ffi::c_int,
                2 as core::ffi::c_int,
                4 as core::ffi::c_int,
                5 as core::ffi::c_int,
                6 as core::ffi::c_int,
                6 as core::ffi::c_int,
                6 as core::ffi::c_int,
            ];
            static mut conv440: [core::ffi::c_int; 8] = [
                1 as core::ffi::c_int,
                2 as core::ffi::c_int,
                2 as core::ffi::c_int,
                2 as core::ffi::c_int,
                3 as core::ffi::c_int,
                4 as core::ffi::c_int,
                6 as core::ffi::c_int,
                0 as core::ffi::c_int,
            ];
            by = (*dlist.offset(bi as isize)).by as core::ffi::c_int;
            bx = (*dlist.offset(bi as isize)).bx as core::ffi::c_int;
            (*dir.offset(by as isize))[bx as usize] = *if xdec != 0 {
                core::ptr::addr_of!(conv422).cast::<core::ffi::c_int>()
            } else {
                core::ptr::addr_of!(conv440).cast::<core::ffi::c_int>()
            }
                .offset((*dir.offset(by as isize))[bx as usize] as isize);
            bi += 1;
        }
    }
    if !dst8.is_null() {
        let block_width: core::ffi::c_int = 8 as core::ffi::c_int >> xdec;
        let block_height: core::ffi::c_int = 8 as core::ffi::c_int >> ydec;
        let cdef_filter_fn: [cdef_filter_block_func; 4] = [
            cdef_filter_8_0,
            cdef_filter_8_1,
            cdef_filter_8_2,
            cdef_filter_8_3,
        ];
        bi = 0 as core::ffi::c_int;
        while bi < cdef_count {
            by = (*dlist.offset(bi as isize)).by as core::ffi::c_int;
            bx = (*dlist.offset(bi as isize)).bx as core::ffi::c_int;
            let t: core::ffi::c_int = if pli != 0 {
                pri_strength
            } else {
                adjust_strength(pri_strength, (*var.offset(by as isize))[bx as usize])
                    as core::ffi::c_int
            };
            let strength_index: core::ffi::c_int = (sec_strength
                == 0 as core::ffi::c_int) as core::ffi::c_int
                | ((t == 0 as core::ffi::c_int) as core::ffi::c_int)
                    << 1 as core::ffi::c_int;
            (cdef_filter_fn[strength_index as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *dst8.offset(((by << bh_log2) * dstride + (bx << bw_log2)) as isize)
                    as *mut uint8_t as *mut core::ffi::c_void,
                dstride,
                &*in_0
                    .offset(((by * CDEF_BSTRIDE << bh_log2) + (bx << bw_log2)) as isize),
                t,
                sec_strength,
                if pri_strength != 0 {
                    (*dir.offset(by as isize))[bx as usize]
                } else {
                    0 as core::ffi::c_int
                },
                damping,
                damping,
                coeff_shift,
                block_width,
                block_height,
            );
            bi += 1;
        }
    } else {
        let block_width_0: core::ffi::c_int = 8 as core::ffi::c_int >> xdec;
        let block_height_0: core::ffi::c_int = 8 as core::ffi::c_int >> ydec;
        let cdef_filter_fn_0: [cdef_filter_block_func; 4] = [
            cdef_filter_16_0,
            cdef_filter_16_1,
            cdef_filter_16_2,
            cdef_filter_16_3,
        ];
        bi = 0 as core::ffi::c_int;
        while bi < cdef_count {
            by = (*dlist.offset(bi as isize)).by as core::ffi::c_int;
            bx = (*dlist.offset(bi as isize)).bx as core::ffi::c_int;
            let t_0: core::ffi::c_int = if pli != 0 {
                pri_strength
            } else {
                adjust_strength(pri_strength, (*var.offset(by as isize))[bx as usize])
                    as core::ffi::c_int
            };
            let strength_index_0: core::ffi::c_int = (sec_strength
                == 0 as core::ffi::c_int) as core::ffi::c_int
                | ((t_0 == 0 as core::ffi::c_int) as core::ffi::c_int)
                    << 1 as core::ffi::c_int;
            (cdef_filter_fn_0[strength_index_0 as usize])
                .expect(
                    "non-null function pointer",
                )(
                &raw mut *dst16
                    .offset(
                        (if !dirinit.is_null() {
                            bi << bw_log2 + bh_log2
                        } else {
                            (by << bh_log2) * dstride + (bx << bw_log2)
                        }) as isize,
                    ) as *mut uint16_t as *mut core::ffi::c_void,
                if !dirinit.is_null() {
                    (1 as core::ffi::c_int) << bw_log2
                } else {
                    dstride
                },
                &*in_0
                    .offset(((by * CDEF_BSTRIDE << bh_log2) + (bx << bw_log2)) as isize),
                t_0,
                sec_strength,
                if pri_strength != 0 {
                    (*dir.offset(by as isize))[bx as usize]
                } else {
                    0 as core::ffi::c_int
                },
                damping,
                damping,
                coeff_shift,
                block_width_0,
                block_height_0,
            );
            bi += 1;
        }
    };
}
unsafe extern "C-unwind" fn run_static_initializers() {
    cdef_directions = core::ptr::addr_of!(cdef_directions_padded.0).cast::<[core::ffi::c_int; 2]>()
        .offset(2 as core::ffi::c_int as isize);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C-unwind" fn(); 1] = [run_static_initializers];

extern "C" {
    fn abs(_: core::ffi::c_int) -> core::ffi::c_int;
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
