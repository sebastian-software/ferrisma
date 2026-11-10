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

// Mechanically translated from upstream/aom/third_party/fastfeat/fast.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    fn aom_fast9_detect(
        im: *const byte,
        xsize: core::ffi::c_int,
        ysize: core::ffi::c_int,
        stride: core::ffi::c_int,
        b: core::ffi::c_int,
        ret_num_corners: *mut core::ffi::c_int,
    ) -> *mut xy;
    fn aom_fast9_score(
        i: *const byte,
        stride: core::ffi::c_int,
        corners: *const xy,
        num_corners: core::ffi::c_int,
        b: core::ffi::c_int,
    ) -> *mut core::ffi::c_int;
    fn aom_nonmax_suppression(
        corners: *const xy,
        scores: *const core::ffi::c_int,
        num_corners: core::ffi::c_int,
        ret_scores: *mut *mut core::ffi::c_int,
        ret_num_nonmax: *mut core::ffi::c_int,
    ) -> *mut xy;
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct xy {
    pub x: core::ffi::c_int,
    pub y: core::ffi::c_int,
}
pub type byte = core::ffi::c_uchar;
pub const __DARWIN_NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const NULL: *mut core::ffi::c_void = __DARWIN_NULL;
#[no_mangle]
pub unsafe extern "C-unwind" fn aom_fast9_detect_nonmax(
    mut im: *const byte,
    mut xsize: core::ffi::c_int,
    mut ysize: core::ffi::c_int,
    mut stride: core::ffi::c_int,
    mut b: core::ffi::c_int,
    mut ret_scores: *mut *mut core::ffi::c_int,
    mut ret_num_corners: *mut core::ffi::c_int,
) -> *mut xy {
    let mut corners: *mut xy = 0 as *mut xy;
    let mut num_corners: core::ffi::c_int = 0;
    let mut scores: *mut core::ffi::c_int = 0 as *mut core::ffi::c_int;
    let mut nonmax: *mut xy = 0 as *mut xy;
    corners = aom_fast9_detect(im, xsize, ysize, stride, b, &raw mut num_corners);
    if corners.is_null() {
        *ret_num_corners = -(1 as core::ffi::c_int);
        return 0 as *mut xy;
    }
    scores = aom_fast9_score(im, stride, corners, num_corners, b);
    if scores.is_null() && num_corners > 0 as core::ffi::c_int {
        free(corners as *mut core::ffi::c_void);
        *ret_num_corners = -(1 as core::ffi::c_int);
        return 0 as *mut xy;
    }
    nonmax = aom_nonmax_suppression(
        corners,
        scores,
        num_corners,
        ret_scores,
        ret_num_corners,
    );
    free(corners as *mut core::ffi::c_void);
    free(scores as *mut core::ffi::c_void);
    return nonmax;
}

extern "C" {
    fn free(_: *mut core::ffi::c_void);
}
