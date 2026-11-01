/*
 * Copyright © 2018, VideoLAN and dav1d authors
 * Copyright © 2018, Two Orioles, LLC
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

// Mechanically translated from upstream/dav1d/src/wedge.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
    fn memset(
        __s: *mut core::ffi::c_void,
        __c: core::ffi::c_int,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
}
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uintptr_t = usize;
pub type size_t = usize;
pub type InterIntraPredMode = core::ffi::c_uint;
pub const N_INTER_INTRA_PRED_MODES: InterIntraPredMode = 4;
pub const II_SMOOTH_PRED: InterIntraPredMode = 3;
pub const II_HOR_PRED: InterIntraPredMode = 2;
pub const II_VERT_PRED: InterIntraPredMode = 1;
pub const II_DC_PRED: InterIntraPredMode = 0;
pub type BlockSize = core::ffi::c_uint;
pub const N_BS_SIZES: BlockSize = 22;
pub const BS_4x4: BlockSize = 21;
pub const BS_4x8: BlockSize = 20;
pub const BS_4x16: BlockSize = 19;
pub const BS_8x4: BlockSize = 18;
pub const BS_8x8: BlockSize = 17;
pub const BS_8x16: BlockSize = 16;
pub const BS_8x32: BlockSize = 15;
pub const BS_16x4: BlockSize = 14;
pub const BS_16x8: BlockSize = 13;
pub const BS_16x16: BlockSize = 12;
pub const BS_16x32: BlockSize = 11;
pub const BS_16x64: BlockSize = 10;
pub const BS_32x8: BlockSize = 9;
pub const BS_32x16: BlockSize = 8;
pub const BS_32x32: BlockSize = 7;
pub const BS_32x64: BlockSize = 6;
pub const BS_64x16: BlockSize = 5;
pub const BS_64x32: BlockSize = 4;
pub const BS_64x64: BlockSize = 3;
pub const BS_64x128: BlockSize = 2;
pub const BS_128x64: BlockSize = 1;
pub const BS_128x128: BlockSize = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Dav1dMasks {
    pub offsets: [[C2RustUnnamed; 11]; 3],
    pub __dav1d_align_wedge_444_32x32: [crate::platform::Align64; 0],
    pub wedge_444_32x32: [uint8_t; 16384],
    pub __dav1d_align_wedge_444_32x16: [crate::platform::Align64; 0],
    pub wedge_444_32x16: [uint8_t; 8192],
    pub __dav1d_align_wedge_444_32x8: [crate::platform::Align64; 0],
    pub wedge_444_32x8: [uint8_t; 4096],
    pub __dav1d_align_wedge_444_16x32: [crate::platform::Align64; 0],
    pub wedge_444_16x32: [uint8_t; 8192],
    pub __dav1d_align_wedge_444_16x16: [crate::platform::Align64; 0],
    pub wedge_444_16x16: [uint8_t; 4096],
    pub __dav1d_align_wedge_444_16x8: [crate::platform::Align64; 0],
    pub wedge_444_16x8: [uint8_t; 2048],
    pub __dav1d_align_wedge_444_8x32: [crate::platform::Align64; 0],
    pub wedge_444_8x32: [uint8_t; 4096],
    pub __dav1d_align_wedge_444_8x16: [crate::platform::Align64; 0],
    pub wedge_444_8x16: [uint8_t; 2048],
    pub __dav1d_align_wedge_444_8x8: [crate::platform::Align64; 0],
    pub wedge_444_8x8: [uint8_t; 1024],
    pub __dav1d_align_wedge_422_16x32: [crate::platform::Align64; 0],
    pub wedge_422_16x32: [uint8_t; 16384],
    pub __dav1d_align_wedge_422_16x16: [crate::platform::Align64; 0],
    pub wedge_422_16x16: [uint8_t; 8192],
    pub __dav1d_align_wedge_422_16x8: [crate::platform::Align64; 0],
    pub wedge_422_16x8: [uint8_t; 4096],
    pub __dav1d_align_wedge_422_8x32: [crate::platform::Align64; 0],
    pub wedge_422_8x32: [uint8_t; 8192],
    pub __dav1d_align_wedge_422_8x16: [crate::platform::Align64; 0],
    pub wedge_422_8x16: [uint8_t; 4096],
    pub __dav1d_align_wedge_422_8x8: [crate::platform::Align64; 0],
    pub wedge_422_8x8: [uint8_t; 2048],
    pub __dav1d_align_wedge_422_4x32: [crate::platform::Align64; 0],
    pub wedge_422_4x32: [uint8_t; 4096],
    pub __dav1d_align_wedge_422_4x16: [crate::platform::Align64; 0],
    pub wedge_422_4x16: [uint8_t; 2048],
    pub __dav1d_align_wedge_422_4x8: [crate::platform::Align64; 0],
    pub wedge_422_4x8: [uint8_t; 1024],
    pub __dav1d_align_wedge_420_16x16: [crate::platform::Align64; 0],
    pub wedge_420_16x16: [uint8_t; 8192],
    pub __dav1d_align_wedge_420_16x8: [crate::platform::Align64; 0],
    pub wedge_420_16x8: [uint8_t; 4096],
    pub __dav1d_align_wedge_420_16x4: [crate::platform::Align64; 0],
    pub wedge_420_16x4: [uint8_t; 2048],
    pub __dav1d_align_wedge_420_8x16: [crate::platform::Align64; 0],
    pub wedge_420_8x16: [uint8_t; 4096],
    pub __dav1d_align_wedge_420_8x8: [crate::platform::Align64; 0],
    pub wedge_420_8x8: [uint8_t; 2048],
    pub __dav1d_align_wedge_420_8x4: [crate::platform::Align64; 0],
    pub wedge_420_8x4: [uint8_t; 1024],
    pub __dav1d_align_wedge_420_4x16: [crate::platform::Align64; 0],
    pub wedge_420_4x16: [uint8_t; 2048],
    pub __dav1d_align_wedge_420_4x8: [crate::platform::Align64; 0],
    pub wedge_420_4x8: [uint8_t; 1024],
    pub __dav1d_align_wedge_420_4x4: [crate::platform::Align64; 0],
    pub wedge_420_4x4: [uint8_t; 512],
    pub __dav1d_align_ii_dc: [crate::platform::Align64; 0],
    pub ii_dc: [uint8_t; 1024],
    pub __dav1d_align_ii_nondc_32x32: [crate::platform::Align64; 0],
    pub ii_nondc_32x32: [uint8_t; 3072],
    pub __dav1d_align_ii_nondc_16x32: [crate::platform::Align64; 0],
    pub ii_nondc_16x32: [uint8_t; 1536],
    pub __dav1d_align_ii_nondc_16x16: [crate::platform::Align64; 0],
    pub ii_nondc_16x16: [uint8_t; 768],
    pub __dav1d_align_ii_nondc_8x32: [crate::platform::Align64; 0],
    pub ii_nondc_8x32: [uint8_t; 768],
    pub __dav1d_align_ii_nondc_8x16: [crate::platform::Align64; 0],
    pub ii_nondc_8x16: [uint8_t; 384],
    pub __dav1d_align_ii_nondc_8x8: [crate::platform::Align64; 0],
    pub ii_nondc_8x8: [uint8_t; 192],
    pub __dav1d_align_ii_nondc_4x16: [crate::platform::Align64; 0],
    pub ii_nondc_4x16: [uint8_t; 192],
    pub __dav1d_align_ii_nondc_4x8: [crate::platform::Align32; 0],
    pub ii_nondc_4x8: [uint8_t; 96],
    pub __dav1d_align_ii_nondc_4x4: [crate::platform::Align16; 0],
    pub ii_nondc_4x4: [uint8_t; 48],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub wedge: [[uint16_t; 16]; 2],
    pub ii: [uint16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct wedge_code_type {
    pub direction: uint8_t,
    pub x_offset: uint8_t,
    pub y_offset: uint8_t,
}
pub const WEDGE_OBLIQUE117: WedgeDirectionType = 4;
pub const WEDGE_OBLIQUE63: WedgeDirectionType = 3;
pub const WEDGE_OBLIQUE153: WedgeDirectionType = 5;
pub const WEDGE_OBLIQUE27: WedgeDirectionType = 2;
pub const WEDGE_VERTICAL: WedgeDirectionType = 1;
pub const WEDGE_HORIZONTAL: WedgeDirectionType = 0;
pub const WEDGE_MASTER_LINE_ODD: WedgeMasterLineType = 0;
pub const WEDGE_MASTER_LINE_EVEN: WedgeMasterLineType = 1;
pub const WEDGE_MASTER_LINE_VERT: WedgeMasterLineType = 2;
pub type WedgeMasterLineType = core::ffi::c_uint;
pub const N_WEDGE_MASTER_LINES: WedgeMasterLineType = 3;
pub type WedgeDirectionType = core::ffi::c_uint;
pub const N_WEDGE_DIRECTIONS: WedgeDirectionType = 6;
#[inline]
unsafe extern "C" fn imax(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a > b { a } else { b };
}
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
}
static mut wedge_codebook_16_hgtw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
];
static mut wedge_codebook_16_hltw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
];
static mut wedge_codebook_16_heqw: [wedge_code_type; 16] = [
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_HORIZONTAL as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_VERTICAL as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE27 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 2 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE153 as core::ffi::c_int as uint8_t,
            x_offset: 4 as uint8_t,
            y_offset: 6 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE63 as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 2 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
    {
        let mut init = wedge_code_type {
            direction: WEDGE_OBLIQUE117 as core::ffi::c_int as uint8_t,
            x_offset: 6 as uint8_t,
            y_offset: 4 as uint8_t,
        };
        init
    },
];
#[no_mangle]
pub static mut dav1d_masks: Dav1dMasks = Dav1dMasks { __dav1d_align_wedge_444_32x32: [], __dav1d_align_wedge_444_32x16: [], __dav1d_align_wedge_444_32x8: [], __dav1d_align_wedge_444_16x32: [], __dav1d_align_wedge_444_16x16: [], __dav1d_align_wedge_444_16x8: [], __dav1d_align_wedge_444_8x32: [], __dav1d_align_wedge_444_8x16: [], __dav1d_align_wedge_444_8x8: [], __dav1d_align_wedge_422_16x32: [], __dav1d_align_wedge_422_16x16: [], __dav1d_align_wedge_422_16x8: [], __dav1d_align_wedge_422_8x32: [], __dav1d_align_wedge_422_8x16: [], __dav1d_align_wedge_422_8x8: [], __dav1d_align_wedge_422_4x32: [], __dav1d_align_wedge_422_4x16: [], __dav1d_align_wedge_422_4x8: [], __dav1d_align_wedge_420_16x16: [], __dav1d_align_wedge_420_16x8: [], __dav1d_align_wedge_420_16x4: [], __dav1d_align_wedge_420_8x16: [], __dav1d_align_wedge_420_8x8: [], __dav1d_align_wedge_420_8x4: [], __dav1d_align_wedge_420_4x16: [], __dav1d_align_wedge_420_4x8: [], __dav1d_align_wedge_420_4x4: [], __dav1d_align_ii_dc: [], __dav1d_align_ii_nondc_32x32: [], __dav1d_align_ii_nondc_16x32: [], __dav1d_align_ii_nondc_16x16: [], __dav1d_align_ii_nondc_8x32: [], __dav1d_align_ii_nondc_8x16: [], __dav1d_align_ii_nondc_8x8: [], __dav1d_align_ii_nondc_4x16: [], __dav1d_align_ii_nondc_4x8: [], __dav1d_align_ii_nondc_4x4: [],
    offsets: [[C2RustUnnamed {
        wedge: [[0; 16]; 2],
        ii: [0; 4],
    }; 11]; 3],
    wedge_444_32x32: [0; 16384],
    wedge_444_32x16: [0; 8192],
    wedge_444_32x8: [0; 4096],
    wedge_444_16x32: [0; 8192],
    wedge_444_16x16: [0; 4096],
    wedge_444_16x8: [0; 2048],
    wedge_444_8x32: [0; 4096],
    wedge_444_8x16: [0; 2048],
    wedge_444_8x8: [0; 1024],
    wedge_422_16x32: [0; 16384],
    wedge_422_16x16: [0; 8192],
    wedge_422_16x8: [0; 4096],
    wedge_422_8x32: [0; 8192],
    wedge_422_8x16: [0; 4096],
    wedge_422_8x8: [0; 2048],
    wedge_422_4x32: [0; 4096],
    wedge_422_4x16: [0; 2048],
    wedge_422_4x8: [0; 1024],
    wedge_420_16x16: [0; 8192],
    wedge_420_16x8: [0; 4096],
    wedge_420_16x4: [0; 2048],
    wedge_420_8x16: [0; 4096],
    wedge_420_8x8: [0; 2048],
    wedge_420_8x4: [0; 1024],
    wedge_420_4x16: [0; 2048],
    wedge_420_4x8: [0; 1024],
    wedge_420_4x4: [0; 512],
    ii_dc: [0; 1024],
    ii_nondc_32x32: [0; 3072],
    ii_nondc_16x32: [0; 1536],
    ii_nondc_16x16: [0; 768],
    ii_nondc_8x32: [0; 768],
    ii_nondc_8x16: [0; 384],
    ii_nondc_8x8: [0; 192],
    ii_nondc_4x16: [0; 192],
    ii_nondc_4x8: [0; 96],
    ii_nondc_4x4: [0; 48],
};
unsafe extern "C" fn insert_border(
    dst: *mut uint8_t,
    src: *const uint8_t,
    ctr: core::ffi::c_int,
) {
    if ctr > 4 as core::ffi::c_int {
        memset(
            dst as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            (ctr - 4 as core::ffi::c_int) as size_t,
        );
    }
    memcpy(
        dst
            .offset(imax(ctr, 4 as core::ffi::c_int) as isize)
            .offset(-(4 as core::ffi::c_int as isize)) as *mut core::ffi::c_void,
        src.offset(imax(4 as core::ffi::c_int - ctr, 0 as core::ffi::c_int) as isize)
            as *const core::ffi::c_void,
        imin(64 as core::ffi::c_int - ctr, 8 as core::ffi::c_int) as size_t,
    );
    if ctr < 64 as core::ffi::c_int - 4 as core::ffi::c_int {
        memset(
            dst.offset(ctr as isize).offset(4 as core::ffi::c_int as isize)
                as *mut core::ffi::c_void,
            64 as core::ffi::c_int,
            (64 as core::ffi::c_int - 4 as core::ffi::c_int - ctr) as size_t,
        );
    }
}
unsafe extern "C" fn transpose(dst: *mut uint8_t, src: *const uint8_t) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y_off: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < 64 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        let mut x_off: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < 64 as core::ffi::c_int {
            *dst.offset((x_off + y) as isize) = *src.offset((y_off + x) as isize);
            x += 1;
            x_off += 64 as core::ffi::c_int;
        }
        y += 1;
        y_off += 64 as core::ffi::c_int;
    }
}
unsafe extern "C" fn hflip(dst: *mut uint8_t, src: *const uint8_t) {
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut y_off: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < 64 as core::ffi::c_int {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < 64 as core::ffi::c_int {
            *dst
                .offset(
                    (y_off + 64 as core::ffi::c_int - 1 as core::ffi::c_int - x) as isize,
                ) = *src.offset((y_off + x) as isize);
            x += 1;
        }
        y += 1;
        y_off += 64 as core::ffi::c_int;
    }
}
unsafe extern "C" fn copy2d(
    mut dst: *mut uint8_t,
    mut src: *const uint8_t,
    mut sign: core::ffi::c_int,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    x_off: core::ffi::c_int,
    y_off: core::ffi::c_int,
) {
    src = src.offset((y_off * 64 as core::ffi::c_int + x_off) as isize);
    if sign != 0 {
        let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
        while y < h {
            let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
            while x < w {
                *dst.offset(x as isize) = (64 as core::ffi::c_int
                    - *src.offset(x as isize) as core::ffi::c_int) as uint8_t;
                x += 1;
            }
            src = src.offset(64 as core::ffi::c_int as isize);
            dst = dst.offset(w as isize);
            y += 1;
        }
    } else {
        let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while y_0 < h {
            memcpy(
                dst as *mut core::ffi::c_void,
                src as *const core::ffi::c_void,
                w as size_t,
            );
            src = src.offset(64 as core::ffi::c_int as isize);
            dst = dst.offset(w as isize);
            y_0 += 1;
        }
    };
}
#[cold]
unsafe extern "C" fn init_chroma(
    mut chroma: *mut uint8_t,
    mut luma: *const uint8_t,
    sign: core::ffi::c_int,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    ss_ver: core::ffi::c_int,
) -> uint16_t {
    let offset: uint16_t = ((chroma as uintptr_t)
        .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
        >> 3 as core::ffi::c_int) as uint16_t;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            let mut sum: core::ffi::c_int = *luma.offset(x as isize) as core::ffi::c_int
                + *luma.offset((x + 1 as core::ffi::c_int) as isize) as core::ffi::c_int
                + 1 as core::ffi::c_int;
            if ss_ver != 0 {
                sum
                    += *luma.offset((w + x) as isize) as core::ffi::c_int
                        + *luma.offset((w + x + 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int + 1 as core::ffi::c_int;
            }
            *chroma.offset((x >> 1 as core::ffi::c_int) as isize) = (sum - sign
                >> 1 as core::ffi::c_int + ss_ver) as uint8_t;
            x += 2 as core::ffi::c_int;
        }
        luma = luma.offset((w << ss_ver) as isize);
        chroma = chroma.offset((w >> 1 as core::ffi::c_int) as isize);
        y += 1 as core::ffi::c_int + ss_ver;
    }
    return offset;
}
#[cold]
unsafe extern "C" fn fill2d_16x2(
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    bs: BlockSize,
    master: *const [uint8_t; 4096],
    cb: *const wedge_code_type,
    mut masks_444: *mut uint8_t,
    mut masks_422: *mut uint8_t,
    mut masks_420: *mut uint8_t,
    mut signs: core::ffi::c_uint,
) {
    let n_stride_444: core::ffi::c_int = w * h;
    let n_stride_422: core::ffi::c_int = n_stride_444 >> 1 as core::ffi::c_int;
    let n_stride_420: core::ffi::c_int = n_stride_444 >> 2 as core::ffi::c_int;
    let sign_stride_422: core::ffi::c_int = 16 as core::ffi::c_int * n_stride_422;
    let sign_stride_420: core::ffi::c_int = 16 as core::ffi::c_int * n_stride_420;
    let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
    while n < 16 as core::ffi::c_int {
        let sign: core::ffi::c_int = (signs & 1 as core::ffi::c_uint)
            as core::ffi::c_int;
        copy2d(
            masks_444,
            (*master.offset((*cb.offset(n as isize)).direction as isize)).as_ptr(),
            sign,
            w,
            h,
            32 as core::ffi::c_int
                - (w * (*cb.offset(n as isize)).x_offset as core::ffi::c_int
                    >> 3 as core::ffi::c_int),
            32 as core::ffi::c_int
                - (h * (*cb.offset(n as isize)).y_offset as core::ffi::c_int
                    >> 3 as core::ffi::c_int),
        );
        dav1d_masks
            .offsets[0 as core::ffi::c_int as usize][bs as usize]
            .wedge[1 as core::ffi::c_int as usize][n as usize] = ((masks_444
            as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[0 as core::ffi::c_int as usize][bs as usize]
            .wedge[0 as core::ffi::c_int as usize][n as usize] = dav1d_masks
            .offsets[0 as core::ffi::c_int as usize][bs as usize]
            .wedge[1 as core::ffi::c_int as usize][n as usize];
        dav1d_masks
            .offsets[1 as core::ffi::c_int as usize][bs as usize]
            .wedge[0 as core::ffi::c_int as usize][n as usize] = init_chroma(
            &raw mut *masks_422.offset((sign * sign_stride_422) as isize),
            masks_444,
            0 as core::ffi::c_int,
            w,
            h,
            0 as core::ffi::c_int,
        );
        dav1d_masks
            .offsets[1 as core::ffi::c_int as usize][bs as usize]
            .wedge[1 as core::ffi::c_int as usize][n as usize] = init_chroma(
            &raw mut *masks_422
                .offset(((sign == 0) as core::ffi::c_int * sign_stride_422) as isize),
            masks_444,
            1 as core::ffi::c_int,
            w,
            h,
            0 as core::ffi::c_int,
        );
        dav1d_masks
            .offsets[2 as core::ffi::c_int as usize][bs as usize]
            .wedge[0 as core::ffi::c_int as usize][n as usize] = init_chroma(
            &raw mut *masks_420.offset((sign * sign_stride_420) as isize),
            masks_444,
            0 as core::ffi::c_int,
            w,
            h,
            1 as core::ffi::c_int,
        );
        dav1d_masks
            .offsets[2 as core::ffi::c_int as usize][bs as usize]
            .wedge[1 as core::ffi::c_int as usize][n as usize] = init_chroma(
            &raw mut *masks_420
                .offset(((sign == 0) as core::ffi::c_int * sign_stride_420) as isize),
            masks_444,
            1 as core::ffi::c_int,
            w,
            h,
            1 as core::ffi::c_int,
        );
        signs >>= 1 as core::ffi::c_int;
        masks_444 = masks_444.offset(n_stride_444 as isize);
        masks_422 = masks_422.offset(n_stride_422 as isize);
        masks_420 = masks_420.offset(n_stride_420 as isize);
        n += 1;
    }
}
#[cold]
unsafe extern "C" fn build_nondc_ii_masks(
    mask_v: *mut uint8_t,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    step: core::ffi::c_int,
) {
    static mut ii_weights_1d: [uint8_t; 32] = [
        60 as core::ffi::c_int as uint8_t,
        52 as core::ffi::c_int as uint8_t,
        45 as core::ffi::c_int as uint8_t,
        39 as core::ffi::c_int as uint8_t,
        34 as core::ffi::c_int as uint8_t,
        30 as core::ffi::c_int as uint8_t,
        26 as core::ffi::c_int as uint8_t,
        22 as core::ffi::c_int as uint8_t,
        19 as core::ffi::c_int as uint8_t,
        17 as core::ffi::c_int as uint8_t,
        15 as core::ffi::c_int as uint8_t,
        13 as core::ffi::c_int as uint8_t,
        11 as core::ffi::c_int as uint8_t,
        10 as core::ffi::c_int as uint8_t,
        8 as core::ffi::c_int as uint8_t,
        7 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        6 as core::ffi::c_int as uint8_t,
        5 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        4 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        3 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        2 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
        1 as core::ffi::c_int as uint8_t,
    ];
    let mask_h: *mut uint8_t = &raw mut *mask_v.offset((w * h) as isize) as *mut uint8_t;
    let mask_sm: *mut uint8_t = &raw mut *mask_h.offset((w * h) as isize) as *mut uint8_t;
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut off: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < h {
        memset(
            &raw mut *mask_v.offset(off as isize) as *mut uint8_t as *mut core::ffi::c_void,
            ii_weights_1d[(y * step) as usize] as core::ffi::c_int,
            w as size_t,
        );
        let mut x: core::ffi::c_int = 0 as core::ffi::c_int;
        while x < w {
            *mask_sm.offset((off + x) as isize) = ii_weights_1d[(imin(x, y) * step)
                as usize];
            *mask_h.offset((off + x) as isize) = ii_weights_1d[(x * step) as usize];
            x += 1;
        }
        y += 1;
        off += w;
    }
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_ii_wedge_masks() {
    static mut wedge_master_border: [[uint8_t; 8]; 3] = [
        [
            1 as core::ffi::c_int as uint8_t,
            2 as core::ffi::c_int as uint8_t,
            6 as core::ffi::c_int as uint8_t,
            18 as core::ffi::c_int as uint8_t,
            37 as core::ffi::c_int as uint8_t,
            53 as core::ffi::c_int as uint8_t,
            60 as core::ffi::c_int as uint8_t,
            63 as core::ffi::c_int as uint8_t,
        ],
        [
            1 as core::ffi::c_int as uint8_t,
            4 as core::ffi::c_int as uint8_t,
            11 as core::ffi::c_int as uint8_t,
            27 as core::ffi::c_int as uint8_t,
            46 as core::ffi::c_int as uint8_t,
            58 as core::ffi::c_int as uint8_t,
            62 as core::ffi::c_int as uint8_t,
            63 as core::ffi::c_int as uint8_t,
        ],
        [
            0 as core::ffi::c_int as uint8_t,
            2 as core::ffi::c_int as uint8_t,
            7 as core::ffi::c_int as uint8_t,
            21 as core::ffi::c_int as uint8_t,
            43 as core::ffi::c_int as uint8_t,
            57 as core::ffi::c_int as uint8_t,
            62 as core::ffi::c_int as uint8_t,
            64 as core::ffi::c_int as uint8_t,
        ],
    ];
    let mut master: [[uint8_t; 4096]; 6] = [[0; 4096]; 6];
    let mut y: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut off: core::ffi::c_int = 0 as core::ffi::c_int;
    while y < 64 as core::ffi::c_int {
        insert_border(
            &raw mut *(*master
                .as_mut_ptr()
                .offset(WEDGE_VERTICAL as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(off as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_VERT as core::ffi::c_int as usize])
                .as_ptr(),
            32 as core::ffi::c_int,
        );
        y += 1;
        off += 64 as core::ffi::c_int;
    }
    let mut y_0: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut off_0: core::ffi::c_int = 0 as core::ffi::c_int;
    let mut ctr: core::ffi::c_int = 48 as core::ffi::c_int;
    while y_0 < 64 as core::ffi::c_int {
        insert_border(
            &raw mut *(*master
                .as_mut_ptr()
                .offset(WEDGE_OBLIQUE63 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset(off_0 as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_EVEN as core::ffi::c_int as usize])
                .as_ptr(),
            ctr,
        );
        insert_border(
            &raw mut *(*master
                .as_mut_ptr()
                .offset(WEDGE_OBLIQUE63 as core::ffi::c_int as isize))
                .as_mut_ptr()
                .offset((off_0 + 64 as core::ffi::c_int) as isize),
            (wedge_master_border[WEDGE_MASTER_LINE_ODD as core::ffi::c_int as usize])
                .as_ptr(),
            ctr - 1 as core::ffi::c_int,
        );
        y_0 += 2 as core::ffi::c_int;
        off_0 += 128 as core::ffi::c_int;
        ctr -= 1;
    }
    transpose(
        (master[WEDGE_OBLIQUE27 as core::ffi::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE63 as core::ffi::c_int as usize]).as_mut_ptr(),
    );
    transpose(
        (master[WEDGE_HORIZONTAL as core::ffi::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_VERTICAL as core::ffi::c_int as usize]).as_mut_ptr(),
    );
    hflip(
        (master[WEDGE_OBLIQUE117 as core::ffi::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE63 as core::ffi::c_int as usize]).as_mut_ptr(),
    );
    hflip(
        (master[WEDGE_OBLIQUE153 as core::ffi::c_int as usize]).as_mut_ptr(),
        (master[WEDGE_OBLIQUE27 as core::ffi::c_int as usize]).as_mut_ptr(),
    );
    fill2d_16x2(
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (BS_32x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_heqw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_32x32).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_16x32).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_16x16).cast::<uint8_t>(),
        0x7bfb as core::ffi::c_uint,
    );
    fill2d_16x2(
        32 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (BS_32x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_hltw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_32x16).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_16x16).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_16x8).cast::<uint8_t>(),
        0x7beb as core::ffi::c_uint,
    );
    fill2d_16x2(
        32 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (BS_32x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_hltw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_32x8).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_16x8).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_16x4).cast::<uint8_t>(),
        0x6beb as core::ffi::c_uint,
    );
    fill2d_16x2(
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (BS_16x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_hgtw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_16x32).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_8x32).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_8x16).cast::<uint8_t>(),
        0x7beb as core::ffi::c_uint,
    );
    fill2d_16x2(
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (BS_16x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_heqw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_16x16).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_8x16).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_8x8).cast::<uint8_t>(),
        0x7bfb as core::ffi::c_uint,
    );
    fill2d_16x2(
        16 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (BS_16x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_hltw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_16x8).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_8x8).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_8x4).cast::<uint8_t>(),
        0x7beb as core::ffi::c_uint,
    );
    fill2d_16x2(
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        (BS_8x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_hgtw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_8x32).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_4x32).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_4x16).cast::<uint8_t>(),
        0x7aeb as core::ffi::c_uint,
    );
    fill2d_16x2(
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        (BS_8x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_hgtw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_8x16).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_4x16).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_4x8).cast::<uint8_t>(),
        0x7beb as core::ffi::c_uint,
    );
    fill2d_16x2(
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        (BS_8x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int) as BlockSize,
        master.as_mut_ptr() as *const [uint8_t; 4096],
        core::ptr::addr_of!(wedge_codebook_16_heqw).cast::<wedge_code_type>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_444_8x8).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_422_4x8).cast::<uint8_t>(),
        core::ptr::addr_of_mut!(dav1d_masks.wedge_420_4x4).cast::<uint8_t>(),
        0x7bfb as core::ffi::c_uint,
    );
    memset(
        core::ptr::addr_of_mut!(dav1d_masks.ii_dc).cast::<uint8_t>() as *mut core::ffi::c_void,
        32 as core::ffi::c_int,
        (32 as core::ffi::c_int * 32 as core::ffi::c_int) as size_t,
    );
    let mut c: core::ffi::c_int = 0 as core::ffi::c_int;
    while c < 3 as core::ffi::c_int {
        dav1d_masks
            .offsets[c
                as usize][(BS_8x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize] = ((core::ptr::addr_of_mut!(dav1d_masks.ii_dc).cast::<uint8_t>() as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[c
                as usize][(BS_8x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize] = dav1d_masks
            .offsets[c
                as usize][(BS_8x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize];
        dav1d_masks
            .offsets[c
                as usize][(BS_16x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize] = dav1d_masks
            .offsets[c
                as usize][(BS_8x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize];
        dav1d_masks
            .offsets[c
                as usize][(BS_16x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize] = dav1d_masks
            .offsets[c
                as usize][(BS_16x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize];
        dav1d_masks
            .offsets[c
                as usize][(BS_16x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize] = dav1d_masks
            .offsets[c
                as usize][(BS_16x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize];
        dav1d_masks
            .offsets[c
                as usize][(BS_32x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize] = dav1d_masks
            .offsets[c
                as usize][(BS_16x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize];
        dav1d_masks
            .offsets[c
                as usize][(BS_32x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize] = dav1d_masks
            .offsets[c
                as usize][(BS_32x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[II_DC_PRED as core::ffi::c_int as usize];
        c += 1;
    }
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_32x32).cast::<uint8_t>(),
        32 as core::ffi::c_int,
        32 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_16x32).cast::<uint8_t>(),
        16 as core::ffi::c_int,
        32 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_16x16).cast::<uint8_t>(),
        16 as core::ffi::c_int,
        16 as core::ffi::c_int,
        2 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_8x32).cast::<uint8_t>(),
        8 as core::ffi::c_int,
        32 as core::ffi::c_int,
        1 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_8x16).cast::<uint8_t>(),
        8 as core::ffi::c_int,
        16 as core::ffi::c_int,
        2 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_8x8).cast::<uint8_t>(),
        8 as core::ffi::c_int,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_4x16).cast::<uint8_t>(),
        4 as core::ffi::c_int,
        16 as core::ffi::c_int,
        2 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_4x8).cast::<uint8_t>(),
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
        4 as core::ffi::c_int,
    );
    build_nondc_ii_masks(
        core::ptr::addr_of_mut!(dav1d_masks.ii_nondc_4x4).cast::<uint8_t>(),
        4 as core::ffi::c_int,
        4 as core::ffi::c_int,
        8 as core::ffi::c_int,
    );
    let mut p: core::ffi::c_int = 0 as core::ffi::c_int;
    while p < 3 as core::ffi::c_int {
        dav1d_masks
            .offsets[0 as core::ffi::c_int
                as usize][(BS_32x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_32x32)
            .as_mut_ptr()
            .offset((p * 32 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[1 as core::ffi::c_int
                as usize][(BS_32x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_16x32)
            .as_mut_ptr()
            .offset((p * 16 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[2 as core::ffi::c_int
                as usize][(BS_32x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_16x16)
            .as_mut_ptr()
            .offset((p * 16 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[0 as core::ffi::c_int
                as usize][(BS_32x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_32x32)
            .as_mut_ptr()
            .offset((p * 32 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[1 as core::ffi::c_int
                as usize][(BS_32x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_16x16)
            .as_mut_ptr()
            .offset((p * 16 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[2 as core::ffi::c_int
                as usize][(BS_32x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_16x16)
            .as_mut_ptr()
            .offset((p * 16 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[0 as core::ffi::c_int
                as usize][(BS_16x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_16x32)
            .as_mut_ptr()
            .offset((p * 16 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[1 as core::ffi::c_int
                as usize][(BS_16x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x32)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 32 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[2 as core::ffi::c_int
                as usize][(BS_16x32 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x16)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[0 as core::ffi::c_int
                as usize][(BS_16x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_16x16)
            .as_mut_ptr()
            .offset((p * 16 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[1 as core::ffi::c_int
                as usize][(BS_16x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x16)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[2 as core::ffi::c_int
                as usize][(BS_16x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x8)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[0 as core::ffi::c_int
                as usize][(BS_16x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_16x16)
            .as_mut_ptr()
            .offset((p * 16 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[1 as core::ffi::c_int
                as usize][(BS_16x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x8)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[2 as core::ffi::c_int
                as usize][(BS_16x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x8)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[0 as core::ffi::c_int
                as usize][(BS_8x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x16)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[1 as core::ffi::c_int
                as usize][(BS_8x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_4x16)
            .as_mut_ptr()
            .offset((p * 4 as core::ffi::c_int * 16 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[2 as core::ffi::c_int
                as usize][(BS_8x16 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_4x8)
            .as_mut_ptr()
            .offset((p * 4 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[0 as core::ffi::c_int
                as usize][(BS_8x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_8x8)
            .as_mut_ptr()
            .offset((p * 8 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[1 as core::ffi::c_int
                as usize][(BS_8x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_4x8)
            .as_mut_ptr()
            .offset((p * 4 as core::ffi::c_int * 8 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        dav1d_masks
            .offsets[2 as core::ffi::c_int
                as usize][(BS_8x8 as core::ffi::c_int - BS_32x32 as core::ffi::c_int)
                as usize]
            .ii[(p + 1 as core::ffi::c_int) as usize] = ((&raw mut *(dav1d_masks
            .ii_nondc_4x4)
            .as_mut_ptr()
            .offset((p * 4 as core::ffi::c_int * 4 as core::ffi::c_int) as isize)
            as *mut uint8_t as uintptr_t)
            .wrapping_sub(&raw mut dav1d_masks as *mut Dav1dMasks as uintptr_t)
            >> 3 as core::ffi::c_int) as uint16_t;
        p += 1;
    }
}
