/*
 * Copyright © 2018-2023, VideoLAN and dav1d authors
 * Copyright © 2018-2023, Two Orioles, LLC
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

// Mechanically translated from upstream/dav1d/src/intra_edge.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uintptr_t = usize;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeNode {
    pub o: uint8_t,
    pub h: [uint8_t; 2],
    pub v: [uint8_t; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeTip {
    pub node: EdgeNode,
    pub split: [uint8_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct EdgeBranch {
    pub node: EdgeNode,
    pub h4: uint8_t,
    pub v4: uint8_t,
    pub split_offset: [uint16_t; 4],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub branch_sb128: [EdgeBranch; 85],
    pub tip_sb128: [EdgeTip; 256],
    pub branch_sb64: [EdgeBranch; 21],
    pub tip_sb64: [EdgeTip; 64],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ModeSelMem {
    pub nwc: [*mut EdgeBranch; 3],
    pub nt: *mut EdgeTip,
}
pub const BL_32X32: BlockLevel = 2;
pub const BL_64X64: BlockLevel = 1;
pub type BlockLevel = core::ffi::c_uint;
pub const N_BL_LEVELS: BlockLevel = 5;
pub const BL_8X8: BlockLevel = 4;
pub const BL_16X16: BlockLevel = 3;
pub const BL_128X128: BlockLevel = 0;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
static mut nodes: crate::platform::Aligned16<C2RustUnnamed> = crate::platform::Aligned16(C2RustUnnamed {
    branch_sb128: [EdgeBranch {
        node: EdgeNode {
            o: 0,
            h: [0; 2],
            v: [0; 2],
        },
        h4: 0,
        v4: 0,
        split_offset: [0; 4],
    }; 85],
    tip_sb128: [EdgeTip {
        node: EdgeNode {
            o: 0,
            h: [0; 2],
            v: [0; 2],
        },
        split: [0; 3],
    }; 256],
    branch_sb64: [EdgeBranch {
        node: EdgeNode {
            o: 0,
            h: [0; 2],
            v: [0; 2],
        },
        h4: 0,
        v4: 0,
        split_offset: [0; 4],
    }; 21],
    tip_sb64: [EdgeTip {
        node: EdgeNode {
            o: 0,
            h: [0; 2],
            v: [0; 2],
        },
        split: [0; 3],
    }; 64],
});
#[no_mangle]
pub static mut dav1d_intra_edge_tree: [*const EdgeNode; 2] = [0 as *const EdgeNode; 2];
#[cold]
unsafe extern "C" fn init_edges(
    node: *mut EdgeNode,
    bl: BlockLevel,
    edge_flags: EdgeFlags,
) {
    (*node).o = edge_flags as uint8_t;
    (*node).h[0 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
        | EDGE_ALL_LEFT_HAS_BOTTOM as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    (*node).v[0 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
        | EDGE_ALL_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint) as uint8_t;
    if bl as core::ffi::c_uint == BL_8X8 as core::ffi::c_int as core::ffi::c_uint {
        let nt: *mut EdgeTip = node as *mut EdgeTip;
        (*node).h[1 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
            & (EDGE_ALL_LEFT_HAS_BOTTOM as core::ffi::c_int
                | EDGE_I420_TOP_HAS_RIGHT as core::ffi::c_int) as core::ffi::c_uint)
            as uint8_t;
        (*node).v[1 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
            & (EDGE_ALL_TOP_HAS_RIGHT as core::ffi::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as core::ffi::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as core::ffi::c_int) as core::ffi::c_uint)
            as uint8_t;
        (*nt).split[0 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
            & EDGE_ALL_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint
            | EDGE_I422_LEFT_HAS_BOTTOM as core::ffi::c_int as core::ffi::c_uint)
            as uint8_t;
        (*nt).split[1 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
            | EDGE_I444_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint)
            as uint8_t;
        (*nt).split[2 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
            & (EDGE_I420_TOP_HAS_RIGHT as core::ffi::c_int
                | EDGE_I420_LEFT_HAS_BOTTOM as core::ffi::c_int
                | EDGE_I422_LEFT_HAS_BOTTOM as core::ffi::c_int) as core::ffi::c_uint)
            as uint8_t;
    } else {
        let nwc: *mut EdgeBranch = node as *mut EdgeBranch;
        (*node).h[1 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
            & EDGE_ALL_LEFT_HAS_BOTTOM as core::ffi::c_int as core::ffi::c_uint)
            as uint8_t;
        (*node).v[1 as core::ffi::c_int as usize] = (edge_flags as core::ffi::c_uint
            & EDGE_ALL_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint)
            as uint8_t;
        (*nwc).h4 = EDGE_ALL_LEFT_HAS_BOTTOM as core::ffi::c_int as uint8_t;
        (*nwc).v4 = EDGE_ALL_TOP_HAS_RIGHT as core::ffi::c_int as uint8_t;
        if bl as core::ffi::c_uint == BL_16X16 as core::ffi::c_int as core::ffi::c_uint {
            (*nwc).h4 = ((*nwc).h4 as core::ffi::c_uint
                | edge_flags as core::ffi::c_uint
                    & EDGE_I420_TOP_HAS_RIGHT as core::ffi::c_int as core::ffi::c_uint)
                as uint8_t;
            (*nwc).v4 = ((*nwc).v4 as core::ffi::c_uint
                | edge_flags as core::ffi::c_uint
                    & (EDGE_I420_LEFT_HAS_BOTTOM as core::ffi::c_int
                        | EDGE_I422_LEFT_HAS_BOTTOM as core::ffi::c_int)
                        as core::ffi::c_uint) as uint8_t;
        }
    };
}
#[cold]
unsafe extern "C" fn init_mode_node(
    nwc: *mut EdgeBranch,
    bl: BlockLevel,
    mem: *mut ModeSelMem,
    top_has_right: core::ffi::c_int,
    left_has_bottom: core::ffi::c_int,
) {
    init_edges(
        &raw mut (*nwc).node,
        bl,
        ((if top_has_right != 0 {
            EDGE_ALL_TOP_HAS_RIGHT as core::ffi::c_int
        } else {
            0 as core::ffi::c_int
        })
            | (if left_has_bottom != 0 {
                EDGE_ALL_LEFT_HAS_BOTTOM as core::ffi::c_int
            } else {
                0 as core::ffi::c_int
            })) as EdgeFlags,
    );
    if bl as core::ffi::c_uint == BL_16X16 as core::ffi::c_int as core::ffi::c_uint {
        let mut n: core::ffi::c_int = 0 as core::ffi::c_int;
        while n < 4 as core::ffi::c_int {
            let fresh0 = (*mem).nt;
            (*mem).nt = ((*mem).nt).offset(1);
            let nt: *mut EdgeTip = fresh0;
            (*nwc).split_offset[n as usize] = (nt as uintptr_t)
                .wrapping_sub(nwc as uintptr_t) as uint16_t;
            init_edges(
                &raw mut (*nt).node,
                (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                    as BlockLevel,
                ((if n == 3 as core::ffi::c_int
                    || n == 1 as core::ffi::c_int && top_has_right == 0
                {
                    0 as core::ffi::c_int
                } else {
                    EDGE_ALL_TOP_HAS_RIGHT as core::ffi::c_int
                })
                    | (if !(n == 0 as core::ffi::c_int
                        || n == 2 as core::ffi::c_int && left_has_bottom != 0)
                    {
                        0 as core::ffi::c_int
                    } else {
                        EDGE_ALL_LEFT_HAS_BOTTOM as core::ffi::c_int
                    })) as EdgeFlags,
            );
            n += 1;
        }
    } else {
        let mut n_0: core::ffi::c_int = 0 as core::ffi::c_int;
        while n_0 < 4 as core::ffi::c_int {
            let fresh1 = (*mem).nwc[bl as usize];
            (*mem).nwc[bl as usize] = ((*mem).nwc[bl as usize]).offset(1);
            let nwc_child: *mut EdgeBranch = fresh1;
            (*nwc).split_offset[n_0 as usize] = (nwc_child as uintptr_t)
                .wrapping_sub(nwc as uintptr_t) as uint16_t;
            init_mode_node(
                nwc_child,
                (bl as core::ffi::c_uint).wrapping_add(1 as core::ffi::c_uint)
                    as BlockLevel,
                mem,
                !(n_0 == 3 as core::ffi::c_int
                    || n_0 == 1 as core::ffi::c_int && top_has_right == 0)
                    as core::ffi::c_int,
                (n_0 == 0 as core::ffi::c_int
                    || n_0 == 2 as core::ffi::c_int && left_has_bottom != 0)
                    as core::ffi::c_int,
            );
            n_0 += 1;
        }
    };
}
#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_intra_edge_tree() {
    let mut mem: ModeSelMem = ModeSelMem {
        nwc: [0 as *mut EdgeBranch; 3],
        nt: 0 as *mut EdgeTip,
    };
    mem.nwc[BL_128X128 as core::ffi::c_int as usize] = &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>()
        .offset(1 as core::ffi::c_int as isize) as *mut EdgeBranch;
    mem.nwc[BL_64X64 as core::ffi::c_int as usize] = &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>()
        .offset((1 as core::ffi::c_int + 4 as core::ffi::c_int) as isize)
        as *mut EdgeBranch;
    mem.nwc[BL_32X32 as core::ffi::c_int as usize] = &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>()
        .offset(
            (1 as core::ffi::c_int + 4 as core::ffi::c_int + 16 as core::ffi::c_int)
                as isize,
        ) as *mut EdgeBranch;
    mem.nt = core::ptr::addr_of_mut!(nodes.0.tip_sb128).cast::<EdgeTip>();
    init_mode_node(
        core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>(),
        BL_128X128,
        &raw mut mem,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
    if !(mem.nwc[BL_128X128 as core::ffi::c_int as usize]
        == &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>()
            .offset((1 as core::ffi::c_int + 4 as core::ffi::c_int) as isize)
            as *mut EdgeBranch)
    {
        unreachable!();
    }
    if !(mem.nwc[BL_64X64 as core::ffi::c_int as usize]
        == &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>()
            .offset(
                (1 as core::ffi::c_int + 4 as core::ffi::c_int + 16 as core::ffi::c_int)
                    as isize,
            ) as *mut EdgeBranch)
    {
        unreachable!();
    }
    if !(mem.nwc[BL_32X32 as core::ffi::c_int as usize]
        == &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>()
            .offset(
                (1 as core::ffi::c_int + 4 as core::ffi::c_int + 16 as core::ffi::c_int
                    + 64 as core::ffi::c_int) as isize,
            ) as *mut EdgeBranch)
    {
        unreachable!();
    }
    if !(mem.nt
        == &raw mut *core::ptr::addr_of_mut!(nodes.0.tip_sb128).cast::<EdgeTip>().offset(256 as core::ffi::c_int as isize)
            as *mut EdgeTip)
    {
        unreachable!();
    }
    mem.nwc[BL_128X128 as core::ffi::c_int as usize] = 0 as *mut EdgeBranch;
    mem.nwc[BL_64X64 as core::ffi::c_int as usize] = &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb64).cast::<EdgeBranch>()
        .offset(1 as core::ffi::c_int as isize) as *mut EdgeBranch;
    mem.nwc[BL_32X32 as core::ffi::c_int as usize] = &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb64).cast::<EdgeBranch>()
        .offset((1 as core::ffi::c_int + 4 as core::ffi::c_int) as isize)
        as *mut EdgeBranch;
    mem.nt = core::ptr::addr_of_mut!(nodes.0.tip_sb64).cast::<EdgeTip>();
    init_mode_node(
        core::ptr::addr_of_mut!(nodes.0.branch_sb64).cast::<EdgeBranch>(),
        BL_64X64,
        &raw mut mem,
        1 as core::ffi::c_int,
        0 as core::ffi::c_int,
    );
    if !(mem.nwc[BL_64X64 as core::ffi::c_int as usize]
        == &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb64).cast::<EdgeBranch>()
            .offset((1 as core::ffi::c_int + 4 as core::ffi::c_int) as isize)
            as *mut EdgeBranch)
    {
        unreachable!();
    }
    if !(mem.nwc[BL_32X32 as core::ffi::c_int as usize]
        == &raw mut *core::ptr::addr_of_mut!(nodes.0.branch_sb64).cast::<EdgeBranch>()
            .offset(
                (1 as core::ffi::c_int + 4 as core::ffi::c_int + 16 as core::ffi::c_int)
                    as isize,
            ) as *mut EdgeBranch)
    {
        unreachable!();
    }
    if !(mem.nt
        == &raw mut *core::ptr::addr_of_mut!(nodes.0.tip_sb64).cast::<EdgeTip>().offset(64 as core::ffi::c_int as isize)
            as *mut EdgeTip)
    {
        unreachable!();
    }
}
unsafe extern "C" fn run_static_initializers() {
    dav1d_intra_edge_tree = [
        core::ptr::addr_of_mut!(nodes.0.branch_sb128).cast::<EdgeBranch>() as *mut EdgeNode as *const EdgeNode,
        core::ptr::addr_of_mut!(nodes.0.branch_sb64).cast::<EdgeBranch>() as *mut EdgeNode as *const EdgeNode,
    ];
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
