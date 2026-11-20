/*
 * Copyright (c) 2019, Alliance for Open Media. All rights reserved.
 *
 * This source code is subject to the terms of the BSD 2 Clause License and
 * the Alliance for Open Media Patent License 1.0. If the BSD 2 Clause License
 * was not distributed with this source code in the LICENSE file, you can
 * obtain it at www.aomedia.org/license/software. If the Alliance for Open
 * Media Patent License 1.0 was not distributed with this source code in the
 * PATENTS file, you can obtain it at www.aomedia.org/license/patent.
 */

// Mechanically translated from upstream/aom/aom_dsp/blk_sse_sum.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type int16_t = i16;
pub type int64_t = i64;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_get_blk_sse_sum_c(
    mut data: *const int16_t,
    mut stride: core::ffi::c_int,
    mut bw: core::ffi::c_int,
    mut bh: core::ffi::c_int,
    mut x_sum: *mut core::ffi::c_int,
    mut x2_sum: *mut int64_t,
) {
    *x_sum = 0 as core::ffi::c_int;
    *x2_sum = 0 as int64_t;
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < bh {
        let mut j: core::ffi::c_int = 0 as core::ffi::c_int;
        while j < bw {
            let val: core::ffi::c_int = *data.offset(j as isize) as core::ffi::c_int;
            *x_sum += val;
            *x2_sum += (val * val) as int64_t;
            j += 1;
        }
        data = data.offset(stride as isize);
        i += 1;
    }
}
