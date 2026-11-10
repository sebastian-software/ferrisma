/*
 * Copyright © 2023, VideoLAN and dav1d authors
 * Copyright © 2023, Two Orioles, LLC
 * All rights reserved.
 *
 * Redistribution and use in source and binary forms, with or without
 * modification, are permitted provided that the following conditions are met:
 *
 * 1. Redistributions of source code must retain the above copyright notice, this
 *    list of conditions and the following disclaimer.
 *
 * 2. Redistributions in binary form must reproduce the above copyright notice,
 *    this list of conditions and the following disclaimer in the documentation
 *    and/or other materials provided with the distribution.
 *
 * THIS SOFTWARE IS PROVIDED BY THE COPYRIGHT HOLDERS AND CONTRIBUTORS "AS IS" AND
 * ANY EXPRESS OR IMPLIED WARRANTIES, INCLUDING, BUT NOT LIMITED TO, THE IMPLIED
 * WARRANTIES OF MERCHANTABILITY AND FITNESS FOR A PARTICULAR PURPOSE ARE
 * DISCLAIMED. IN NO EVENT SHALL THE COPYRIGHT OWNER OR CONTRIBUTORS BE LIABLE FOR
 * ANY DIRECT, INDIRECT, INCIDENTAL, SPECIAL, EXEMPLARY, OR CONSEQUENTIAL DAMAGES
 * (INCLUDING, BUT NOT LIMITED TO, PROCUREMENT OF SUBSTITUTE GOODS OR SERVICES;
 * LOSS OF USE, DATA, OR PROFITS; OR BUSINESS INTERRUPTION) HOWEVER CAUSED AND
 * ON ANY THEORY OF LIABILITY, WHETHER IN CONTRACT, STRICT LIABILITY, OR TORT
 * (INCLUDING NEGLIGENCE OR OTHERWISE) ARISING IN ANY WAY OUT OF THE USE OF THIS
 * SOFTWARE, EVEN IF ADVISED OF THE POSSIBILITY OF SUCH DAMAGE.
 */

// Mechanically translated from upstream/dav1d/src/pal.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn memcpy(
        __dst: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __b: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __len: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type __darwin_size_t = usize;
pub type size_t = __darwin_size_t;
pub type uint8_t = u8;
pub type pal_idx_finish_fn = Option<
    unsafe extern "C" fn(
        *mut uint8_t,
        *const uint8_t,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> (),
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dPalDSPContext {
    pub pal_idx_finish: pal_idx_finish_fn,
}
unsafe extern "C" fn pal_idx_finish_c(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    bw: core::ffi::c_int,
    bh: core::ffi::c_int,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
) {
    if !(bw >= 4 as core::ffi::c_int && bw <= 64 as core::ffi::c_int
        && bw & bw - 1 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"pal_idx_finish_c\0"))
                .as_ptr(),
            b"pal.c\0" as *const u8 as *const core::ffi::c_char,
            41 as core::ffi::c_int,
            b"bw >= 4 && bw <= 64 && !(bw & (bw - 1))\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(bh >= 4 as core::ffi::c_int && bh <= 64 as core::ffi::c_int
        && bh & bh - 1 as core::ffi::c_int == 0) as core::ffi::c_int as core::ffi::c_long
        != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"pal_idx_finish_c\0"))
                .as_ptr(),
            b"pal.c\0" as *const u8 as *const core::ffi::c_char,
            42 as core::ffi::c_int,
            b"bh >= 4 && bh <= 64 && !(bh & (bh - 1))\0" as *const u8
                as *const core::ffi::c_char,
        );
    } else {};
    if !(w >= 4 as core::ffi::c_int && w <= bw && w & 3 as core::ffi::c_int == 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"pal_idx_finish_c\0"))
                .as_ptr(),
            b"pal.c\0" as *const u8 as *const core::ffi::c_char,
            43 as core::ffi::c_int,
            b"w >= 4 && w <= bw && !(w & 3)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    if !(h >= 4 as core::ffi::c_int && h <= bh && h & 3 as core::ffi::c_int == 0)
        as core::ffi::c_int as core::ffi::c_long != 0
    {
        crate::platform::assertion(
            (::core::mem::transmute::<
                [u8; 17],
                [core::ffi::c_char; 17],
            >(*b"pal_idx_finish_c\0"))
                .as_ptr(),
            b"pal.c\0" as *const u8 as *const core::ffi::c_char,
            44 as core::ffi::c_int,
            b"h >= 4 && h <= bh && !(h & 3)\0" as *const u8 as *const core::ffi::c_char,
        );
    } else {};
    let dst_w: core::ffi::c_int = w / 2 as core::ffi::c_int;
    let dst_bw: core::ffi::c_int = bw / 2 as core::ffi::c_int;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < dst_w {
            *dst.offset(x as isize) = (*src
                .offset((x * 2 as core::ffi::c_int + 0 as core::ffi::c_int) as isize)
                as core::ffi::c_int
                | (*src
                    .offset((x * 2 as core::ffi::c_int + 1 as core::ffi::c_int) as isize)
                    as core::ffi::c_int) << 4 as core::ffi::c_int) as uint8_t;
            x += 1;
        }
        if dst_w < dst_bw {
            memset(
                dst.offset(dst_w as isize) as *mut core::ffi::c_void,
                *src.offset((w - 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                    * 0x11 as core::ffi::c_int,
                (dst_bw - dst_w) as size_t,
            );
        }
        y += 1;
        src = src.offset(bw as isize);
        dst = dst.offset(dst_bw as isize);
    }
    if h < bh {
        let last_row: *const uint8_t = &raw mut *dst.offset(-dst_bw as isize)
            as *mut uint8_t;
        let mut y_0: core::ffi::c_int = h;
        while y_0 < bh {
            memcpy(
                dst as *mut core::ffi::c_void,
                last_row as *const core::ffi::c_void,
                dst_bw as size_t,
            );
            y_0 += 1;
            dst = dst.offset(dst_bw as isize);
        }
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_pal_dsp_init(c: *mut Dav1dPalDSPContext) {
    (*c).pal_idx_finish = Some(
        pal_idx_finish_c
            as unsafe extern "C" fn(
                *mut uint8_t,
                *const uint8_t,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
                core::ffi::c_int,
            ) -> (),
    ) as pal_idx_finish_fn;
}
