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

// Mechanically translated from upstream/dav1d/src/ipred_prepare_tmpl.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
use ::c2rust_bitfields;
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
pub type uint8_t = __uint8_t;
pub type size_t = usize;
pub type ptrdiff_t = isize;
pub type pixel = uint8_t;
pub type IntraPredMode = core::ffi::c_uint;
pub const FILTER_PRED: IntraPredMode = 13;
pub const Z3_PRED: IntraPredMode = 8;
pub const Z2_PRED: IntraPredMode = 7;
pub const Z1_PRED: IntraPredMode = 6;
pub const DC_128_PRED: IntraPredMode = 5;
pub const TOP_DC_PRED: IntraPredMode = 4;
pub const LEFT_DC_PRED: IntraPredMode = 3;
pub const N_IMPL_INTRA_PRED_MODES: IntraPredMode = 14;
pub const N_UV_INTRA_PRED_MODES: IntraPredMode = 14;
pub const CFL_PRED: IntraPredMode = 13;
pub const N_INTRA_PRED_MODES: IntraPredMode = 13;
pub const PAETH_PRED: IntraPredMode = 12;
pub const SMOOTH_H_PRED: IntraPredMode = 11;
pub const SMOOTH_V_PRED: IntraPredMode = 10;
pub const SMOOTH_PRED: IntraPredMode = 9;
pub const VERT_LEFT_PRED: IntraPredMode = 8;
pub const HOR_UP_PRED: IntraPredMode = 7;
pub const HOR_DOWN_PRED: IntraPredMode = 6;
pub const VERT_RIGHT_PRED: IntraPredMode = 5;
pub const DIAG_DOWN_RIGHT_PRED: IntraPredMode = 4;
pub const DIAG_DOWN_LEFT_PRED: IntraPredMode = 3;
pub const HOR_PRED: IntraPredMode = 2;
pub const VERT_PRED: IntraPredMode = 1;
pub const DC_PRED: IntraPredMode = 0;
pub type EdgeFlags = core::ffi::c_uint;
pub const EDGE_ALL_TR_AND_BL: EdgeFlags = 63;
pub const EDGE_ALL_LEFT_HAS_BOTTOM: EdgeFlags = 56;
pub const EDGE_ALL_TOP_HAS_RIGHT: EdgeFlags = 7;
pub const EDGE_I420_LEFT_HAS_BOTTOM: EdgeFlags = 32;
pub const EDGE_I422_LEFT_HAS_BOTTOM: EdgeFlags = 16;
pub const EDGE_I444_LEFT_HAS_BOTTOM: EdgeFlags = 8;
pub const EDGE_I420_TOP_HAS_RIGHT: EdgeFlags = 4;
pub const EDGE_I422_TOP_HAS_RIGHT: EdgeFlags = 2;
pub const EDGE_I444_TOP_HAS_RIGHT: EdgeFlags = 1;
#[derive(Copy, Clone, BitfieldStruct)]
#[repr(C)]
pub struct C2RustUnnamed {
    #[bitfield(name = "needs_left", ty = "uint8_t", bits = "0..=0")]
    #[bitfield(name = "needs_top", ty = "uint8_t", bits = "1..=1")]
    #[bitfield(name = "needs_topleft", ty = "uint8_t", bits = "2..=2")]
    #[bitfield(name = "needs_topright", ty = "uint8_t", bits = "3..=3")]
    #[bitfield(name = "needs_bottomleft", ty = "uint8_t", bits = "4..=4")]
    pub needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [u8; 1],
}
#[inline]
unsafe extern "C" fn imin(a: core::ffi::c_int, b: core::ffi::c_int) -> core::ffi::c_int {
    return if a < b { a } else { b };
}
static mut av1_mode_conv: [[[uint8_t; 2]; 2]; 13] = [
    [
        [
            DC_128_PRED as core::ffi::c_int as uint8_t,
            TOP_DC_PRED as core::ffi::c_int as uint8_t,
        ],
        [
            LEFT_DC_PRED as core::ffi::c_int as uint8_t,
            DC_PRED as core::ffi::c_int as uint8_t,
        ],
    ],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [[0; 2]; 2],
    [
        [
            DC_128_PRED as core::ffi::c_int as uint8_t,
            VERT_PRED as core::ffi::c_int as uint8_t,
        ],
        [
            HOR_PRED as core::ffi::c_int as uint8_t,
            PAETH_PRED as core::ffi::c_int as uint8_t,
        ],
    ],
];
static mut av1_mode_to_angle_map: [uint8_t; 8] = [
    90 as core::ffi::c_int as uint8_t,
    180 as core::ffi::c_int as uint8_t,
    45 as core::ffi::c_int as uint8_t,
    135 as core::ffi::c_int as uint8_t,
    113 as core::ffi::c_int as uint8_t,
    157 as core::ffi::c_int as uint8_t,
    203 as core::ffi::c_int as uint8_t,
    67 as core::ffi::c_int as uint8_t,
];
static mut av1_intra_prediction_edges: [C2RustUnnamed; 14] = [C2RustUnnamed {
    needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
}; 14];
#[no_mangle]
pub unsafe extern "C" fn dav1d_prepare_intra_edges_8bpc(
    x: core::ffi::c_int,
    have_left: core::ffi::c_int,
    y: core::ffi::c_int,
    have_top: core::ffi::c_int,
    w: core::ffi::c_int,
    h: core::ffi::c_int,
    edge_flags: EdgeFlags,
    dst: *const pixel,
    stride: ptrdiff_t,
    mut prefilter_toplevel_sb_edge: *const pixel,
    mut mode: IntraPredMode,
    angle: *mut core::ffi::c_int,
    tw: core::ffi::c_int,
    th: core::ffi::c_int,
    filter_edge: core::ffi::c_int,
    topleft_out: *mut pixel,
) -> IntraPredMode {
    let bitdepth: core::ffi::c_int = 8 as core::ffi::c_int;
    if !(y < h && x < w) {
        unreachable!();
    }
    match mode as core::ffi::c_uint {
        1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 => {
            *angle = av1_mode_to_angle_map[(mode as core::ffi::c_uint)
                .wrapping_sub(VERT_PRED as core::ffi::c_int as core::ffi::c_uint)
                as usize] as core::ffi::c_int + 3 as core::ffi::c_int * *angle;
            if *angle <= 90 as core::ffi::c_int {
                mode = (if *angle < 90 as core::ffi::c_int && have_top != 0 {
                    Z1_PRED as core::ffi::c_int
                } else {
                    VERT_PRED as core::ffi::c_int
                }) as IntraPredMode;
            } else if *angle < 180 as core::ffi::c_int {
                mode = Z2_PRED;
            } else {
                mode = (if *angle > 180 as core::ffi::c_int && have_left != 0 {
                    Z3_PRED as core::ffi::c_int
                } else {
                    HOR_PRED as core::ffi::c_int
                }) as IntraPredMode;
            }
        }
        0 | 12 => {
            mode = av1_mode_conv[mode as usize][have_left as usize][have_top as usize]
                as IntraPredMode;
        }
        _ => {}
    }
    let mut dst_top: *const pixel = 0 as *const pixel;
    if have_top != 0
        && ((av1_intra_prediction_edges[mode as usize]).needs_top() as core::ffi::c_int
            != 0
            || (av1_intra_prediction_edges[mode as usize]).needs_topleft()
                as core::ffi::c_int != 0
            || (av1_intra_prediction_edges[mode as usize]).needs_left()
                as core::ffi::c_int != 0 && have_left == 0)
    {
        if !prefilter_toplevel_sb_edge.is_null() {
            dst_top = &*prefilter_toplevel_sb_edge
                .offset((x * 4 as core::ffi::c_int) as isize) as *const pixel;
        } else {
            dst_top = &*dst.offset(-stride as isize) as *const pixel;
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_left() != 0 {
        let sz: core::ffi::c_int = th << 2 as core::ffi::c_int;
        let left: *mut pixel = &raw mut *topleft_out.offset(-sz as isize) as *mut pixel;
        if have_left != 0 {
            let px_have: core::ffi::c_int = imin(sz, h - y << 2 as core::ffi::c_int)
                as core::ffi::c_int;
            let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
            while i < px_have {
                *left.offset((sz - 1 as core::ffi::c_int - i) as isize) = *dst
                    .offset((stride * i as ptrdiff_t - 1 as ptrdiff_t) as isize);
                i += 1;
            }
            if px_have < sz {
                memset(
                    left as *mut core::ffi::c_void,
                    *left.offset((sz - px_have) as isize) as core::ffi::c_int,
                    (sz - px_have) as size_t,
                );
            }
        } else {
            memset(
                left as *mut core::ffi::c_void,
                if have_top != 0 {
                    *dst_top as core::ffi::c_int
                } else {
                    ((1 as core::ffi::c_int) << bitdepth >> 1 as core::ffi::c_int)
                        + 1 as core::ffi::c_int
                },
                sz as size_t,
            );
        }
        if (av1_intra_prediction_edges[mode as usize]).needs_bottomleft() != 0 {
            let have_bottomleft: core::ffi::c_int = (if have_left == 0 || y + th >= h {
                0 as core::ffi::c_uint
            } else {
                edge_flags as core::ffi::c_uint
                    & EDGE_I444_LEFT_HAS_BOTTOM as core::ffi::c_int as core::ffi::c_uint
            }) as core::ffi::c_int;
            if have_bottomleft != 0 {
                let px_have_0: core::ffi::c_int = imin(
                    sz,
                    h - y - th << 2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                let mut i_0: core::ffi::c_int = 0 as core::ffi::c_int;
                while i_0 < px_have_0 {
                    *left.offset(-(i_0 + 1 as core::ffi::c_int) as isize) = *dst
                        .offset(
                            ((sz + i_0) as ptrdiff_t * stride - 1 as ptrdiff_t) as isize,
                        );
                    i_0 += 1;
                }
                if px_have_0 < sz {
                    memset(
                        left.offset(-(sz as isize)) as *mut core::ffi::c_void,
                        *left.offset(-px_have_0 as isize) as core::ffi::c_int,
                        (sz - px_have_0) as size_t,
                    );
                }
            } else {
                memset(
                    left.offset(-(sz as isize)) as *mut core::ffi::c_void,
                    *left.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int,
                    sz as size_t,
                );
            }
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_top() != 0 {
        let sz_0: core::ffi::c_int = tw << 2 as core::ffi::c_int;
        let top: *mut pixel = &raw mut *topleft_out.offset(1 as core::ffi::c_int as isize)
            as *mut pixel;
        if have_top != 0 {
            let px_have_1: core::ffi::c_int = imin(sz_0, w - x << 2 as core::ffi::c_int)
                as core::ffi::c_int;
            memcpy(
                top as *mut core::ffi::c_void,
                dst_top as *const core::ffi::c_void,
                px_have_1 as size_t,
            );
            if px_have_1 < sz_0 {
                memset(
                    top.offset(px_have_1 as isize) as *mut core::ffi::c_void,
                    *top.offset((px_have_1 - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    (sz_0 - px_have_1) as size_t,
                );
            }
        } else {
            memset(
                top as *mut core::ffi::c_void,
                if have_left != 0 {
                    *dst.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
                } else {
                    ((1 as core::ffi::c_int) << bitdepth >> 1 as core::ffi::c_int)
                        - 1 as core::ffi::c_int
                },
                sz_0 as size_t,
            );
        }
        if (av1_intra_prediction_edges[mode as usize]).needs_topright() != 0 {
            let have_topright: core::ffi::c_int = (if have_top == 0 || x + tw >= w {
                0 as core::ffi::c_uint
            } else {
                edge_flags as core::ffi::c_uint
                    & EDGE_I444_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint
            }) as core::ffi::c_int;
            if have_topright != 0 {
                let px_have_2: core::ffi::c_int = imin(
                    sz_0,
                    w - x - tw << 2 as core::ffi::c_int,
                ) as core::ffi::c_int;
                memcpy(
                    top.offset(sz_0 as isize) as *mut core::ffi::c_void,
                    &*dst_top.offset(sz_0 as isize) as *const pixel
                        as *const core::ffi::c_void,
                    px_have_2 as size_t,
                );
                if px_have_2 < sz_0 {
                    memset(
                        top.offset(sz_0 as isize).offset(px_have_2 as isize)
                            as *mut core::ffi::c_void,
                        *top.offset((sz_0 + px_have_2 - 1 as core::ffi::c_int) as isize)
                            as core::ffi::c_int,
                        (sz_0 - px_have_2) as size_t,
                    );
                }
            } else {
                memset(
                    top.offset(sz_0 as isize) as *mut core::ffi::c_void,
                    *top.offset((sz_0 - 1 as core::ffi::c_int) as isize)
                        as core::ffi::c_int,
                    sz_0 as size_t,
                );
            }
        }
    }
    if (av1_intra_prediction_edges[mode as usize]).needs_topleft() != 0 {
        if have_left != 0 {
            *topleft_out = (if have_top != 0 {
                *dst_top.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
            } else {
                *dst.offset(-(1 as core::ffi::c_int) as isize) as core::ffi::c_int
            }) as pixel;
        } else {
            *topleft_out = (if have_top != 0 {
                *dst_top as core::ffi::c_int
            } else {
                (1 as core::ffi::c_int) << bitdepth >> 1 as core::ffi::c_int
            }) as pixel;
        }
        if mode as core::ffi::c_uint == Z2_PRED as core::ffi::c_int as core::ffi::c_uint
            && tw + th >= 6 as core::ffi::c_int && filter_edge != 0
        {
            *topleft_out = ((*topleft_out.offset(-(1 as core::ffi::c_int) as isize)
                as core::ffi::c_int
                + *topleft_out.offset(1 as core::ffi::c_int as isize)
                    as core::ffi::c_int) * 5 as core::ffi::c_int
                + *topleft_out.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int
                    * 6 as core::ffi::c_int + 8 as core::ffi::c_int
                >> 4 as core::ffi::c_int) as pixel;
        }
    }
    return mode;
}
unsafe extern "C" fn run_static_initializers() {
    av1_intra_prediction_edges = [
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0 as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(0);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(1 as uint8_t);
            init.set_needs_topright(1 as uint8_t);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(1 as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(0);
            init.set_needs_topleft(1 as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(1 as uint8_t);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(0);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(1 as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
        {
            let mut init = C2RustUnnamed {
                needs_left_needs_top_needs_topleft_needs_topright_needs_bottomleft: [0; 1],
            };
            init.set_needs_left(1 as uint8_t);
            init.set_needs_top(1 as uint8_t);
            init.set_needs_topleft(1 as uint8_t);
            init.set_needs_topright(0);
            init.set_needs_bottomleft(0);
            init
        },
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
