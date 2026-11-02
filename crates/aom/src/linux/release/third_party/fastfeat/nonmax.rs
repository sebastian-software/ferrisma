// Copyright (c) 2006, 2008 Edward Rosten
// All rights reserved.
//
// Redistribution and use in source and binary forms, with or without
// modification, are permitted provided that the following conditions
// are met:
//
//  *Redistributions of source code must retain the above copyright
//   notice, this list of conditions and the following disclaimer.
//
//  *Redistributions in binary form must reproduce the above copyright
//   notice, this list of conditions and the following disclaimer in the
//   documentation and/or other materials provided with the distribution.
//
//  *Neither the name of the University of Cambridge nor the names of
//   its contributors may be used to endorse or promote products derived
//   from this software without specific prior written permission.
//
// THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS
// "AS IS" AND ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT
// LIMITED TO, THE IMPLIED WARRANTIES OF MERCHANTABILITY AND FITNESS FOR
// A PARTICULAR PURPOSE ARE DISCLAIMED.  IN NO EVENT SHALL THE COPYRIGHT OWNER
// OR CONTRIBUTORS BE LIABLE FOR ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL,
// EXEMPLARY, OR CONSEQUENTIAL DAMAGES (INCLUDING, BUT NOT LIMITED TO,
// PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES; LOSS OF USE, DATA, OR
// PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND ON ANY THEORY OF
// LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT (INCLUDING
// NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
// SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.

// clang-format off

// Mechanically translated from upstream/aom/third_party/fastfeat/nonmax.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
}
pub type size_t = usize;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xy {
    pub x: core::ffi::c_int,
    pub y: core::ffi::c_int,
}
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_nonmax_suppression(
    mut corners: *const xy,
    mut scores: *const core::ffi::c_int,
    mut num_corners: core::ffi::c_int,
    mut ret_scores: *mut *mut core::ffi::c_int,
    mut ret_num_nonmax: *mut core::ffi::c_int,
) -> *mut xy {
    let mut current_block: u64;
    let mut num_nonmax: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut last_row: core::ffi::c_int = 0;
    let mut row_start: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut i: core::ffi::c_int = 0;
    let mut j: core::ffi::c_int = 0;
    let mut ret_nonmax: *mut xy = 0 as *mut xy;
    let mut nonmax_scores: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let sz: core::ffi::c_int = num_corners;
    let mut point_above: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut point_below: core::ffi::c_int = 0 as core::ffi::c_int;
    *ret_scores = 0 as *mut core::ffi::c_int;
    *ret_num_nonmax = -(1 as core::ffi::c_int);
    if !(!corners.is_null() && !scores.is_null()) || num_corners < 1 as core::ffi::c_int
    {
        *ret_num_nonmax = 0 as core::ffi::c_int;
        return 0 as *mut xy;
    }
    ret_nonmax = malloc(
        (num_corners as size_t).wrapping_mul(::core::mem::size_of::<xy>() as size_t),
    ) as *mut xy;
    if ret_nonmax.is_null() {
        return 0 as *mut xy;
    }
    nonmax_scores = malloc(
        (num_corners as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) as *mut core::ffi::c_int;
    if nonmax_scores.is_null() {
        free(ret_nonmax as *mut core::ffi::c_void);
        return 0 as *mut xy;
    }
    last_row = (*corners.offset((num_corners - 1 as core::ffi::c_int) as isize)).y;
    row_start = malloc(
        ((last_row + 1 as core::ffi::c_int) as size_t)
            .wrapping_mul(::core::mem::size_of::<core::ffi::c_int>() as size_t),
    ) as *mut core::ffi::c_int;
    if row_start.is_null() {
        free(ret_nonmax as *mut core::ffi::c_void);
        free(nonmax_scores as *mut core::ffi::c_void);
        return 0 as *mut xy;
    }
    i = 0 as core::ffi::c_int;
    while i < last_row + 1 as core::ffi::c_int {
        *row_start.offset(i as isize) = -(1 as core::ffi::c_int);
        i += 1;
    }
    let mut prev_row: core::ffi::c_int = -(1 as core::ffi::c_int);
    i = 0 as core::ffi::c_int;
    while i < num_corners {
        if (*corners.offset(i as isize)).y != prev_row {
            *row_start.offset((*corners.offset(i as isize)).y as isize) = i;
            prev_row = (*corners.offset(i as isize)).y;
        }
        i += 1;
    }
    i = 0 as core::ffi::c_int;
    while i < sz {
        let mut score: core::ffi::c_int = *scores.offset(i as isize);
        let mut pos: xy = *corners.offset(i as isize);
        if i > 0 as core::ffi::c_int {
            if (*corners.offset((i - 1 as core::ffi::c_int) as isize)).x
                == pos.x - 1 as core::ffi::c_int
                && (*corners.offset((i - 1 as core::ffi::c_int) as isize)).y == pos.y
                && *scores.offset((i - 1 as core::ffi::c_int) as isize) >= score
            {
                current_block = 0;
            } else {
                current_block = 1;
            }
        } else {
            current_block = 1;
        }
        match current_block {
            1 => {
                if i < sz - 1 as core::ffi::c_int {
                    if (*corners.offset((i + 1 as core::ffi::c_int) as isize)).x
                        == pos.x + 1 as core::ffi::c_int
                        && (*corners.offset((i + 1 as core::ffi::c_int) as isize)).y
                            == pos.y
                        && *scores.offset((i + 1 as core::ffi::c_int) as isize) >= score
                    {
                        current_block = 0;
                    } else {
                        current_block = 2;
                    }
                } else {
                    current_block = 2;
                }
                match current_block {
                    0 => {}
                    _ => {
                        if pos.y > 0 as core::ffi::c_int
                            && *row_start
                                .offset((pos.y - 1 as core::ffi::c_int) as isize)
                                != -(1 as core::ffi::c_int)
                        {
                            if (*corners.offset(point_above as isize)).y
                                < pos.y - 1 as core::ffi::c_int
                            {
                                point_above = *row_start
                                    .offset((pos.y - 1 as core::ffi::c_int) as isize);
                            }
                            while (*corners.offset(point_above as isize)).y < pos.y
                                && (*corners.offset(point_above as isize)).x
                                    < pos.x - 1 as core::ffi::c_int
                            {
                                point_above += 1;
                            }
                            j = point_above;
                            loop {
                                if !((*corners.offset(j as isize)).y < pos.y
                                    && (*corners.offset(j as isize)).x
                                        <= pos.x + 1 as core::ffi::c_int)
                                {
                                    current_block = 3;
                                    break;
                                }
                                let mut x: core::ffi::c_int = (*corners.offset(j as isize))
                                    .x;
                                if (x == pos.x - 1 as core::ffi::c_int || x == pos.x
                                    || x == pos.x + 1 as core::ffi::c_int)
                                    && *scores.offset(j as isize) >= score
                                {
                                    current_block = 0;
                                    break;
                                }
                                j += 1;
                            }
                        } else {
                            current_block = 3;
                        }
                        match current_block {
                            0 => {}
                            _ => {
                                if (pos.y + 1 as core::ffi::c_int)
                                    < last_row + 1 as core::ffi::c_int
                                    && *row_start
                                        .offset((pos.y + 1 as core::ffi::c_int) as isize)
                                        != -(1 as core::ffi::c_int) && point_below < sz
                                {
                                    if (*corners.offset(point_below as isize)).y
                                        < pos.y + 1 as core::ffi::c_int
                                    {
                                        point_below = *row_start
                                            .offset((pos.y + 1 as core::ffi::c_int) as isize);
                                    }
                                    while point_below < sz
                                        && (*corners.offset(point_below as isize)).y
                                            == pos.y + 1 as core::ffi::c_int
                                        && (*corners.offset(point_below as isize)).x
                                            < pos.x - 1 as core::ffi::c_int
                                    {
                                        point_below += 1;
                                    }
                                    j = point_below;
                                    loop {
                                        if !(j < sz
                                            && (*corners.offset(j as isize)).y
                                                == pos.y + 1 as core::ffi::c_int
                                            && (*corners.offset(j as isize)).x
                                                <= pos.x + 1 as core::ffi::c_int)
                                        {
                                            current_block = 4;
                                            break;
                                        }
                                        let mut x_0: core::ffi::c_int = (*corners
                                            .offset(j as isize))
                                            .x;
                                        if (x_0 == pos.x - 1 as core::ffi::c_int || x_0 == pos.x
                                            || x_0 == pos.x + 1 as core::ffi::c_int)
                                            && *scores.offset(j as isize) >= score
                                        {
                                            current_block = 0;
                                            break;
                                        }
                                        j += 1;
                                    }
                                } else {
                                    current_block = 4;
                                }
                                match current_block {
                                    0 => {}
                                    _ => {
                                        *ret_nonmax.offset(num_nonmax as isize) = *corners
                                            .offset(i as isize);
                                        *nonmax_scores.offset(num_nonmax as isize) = *scores
                                            .offset(i as isize);
                                        num_nonmax += 1;
                                    }
                                }
                            }
                        }
                    }
                }
            }
            _ => {}
        }
        i += 1;
    }
    free(row_start as *mut core::ffi::c_void);
    *ret_scores = nonmax_scores;
    *ret_num_nonmax = num_nonmax;
    return ret_nonmax;
}

extern "C" {
    fn malloc(__size: size_t) -> *mut core::ffi::c_void;
    fn free(__ptr: *mut core::ffi::c_void);
}
