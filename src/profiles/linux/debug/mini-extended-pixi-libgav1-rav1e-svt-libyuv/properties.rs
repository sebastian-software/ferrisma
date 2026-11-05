// Copyright 2024 Brad Hards. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/properties.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type uint8_t = __uint8_t;
pub type avifBool = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifKnownProperty {
    pub fourcc: [uint8_t; 4],
}
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
static mut knownProperties: [avifKnownProperty; 45] = [
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'f' as i32 as uint8_t,
                't' as i32 as uint8_t,
                'y' as i32 as uint8_t,
                'p' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'u' as i32 as uint8_t,
                'u' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                'd' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'm' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                't' as i32 as uint8_t,
                'a' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'h' as i32 as uint8_t,
                'd' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'r' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'p' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                't' as i32 as uint8_t,
                'm' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'd' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                'n' as i32 as uint8_t,
                'f' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'd' as i32 as uint8_t,
                'r' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                'f' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'd' as i32 as uint8_t,
                'a' as i32 as uint8_t,
                't' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'o' as i32 as uint8_t,
                'c' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                'n' as i32 as uint8_t,
                'f' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'n' as i32 as uint8_t,
                'f' as i32 as uint8_t,
                'e' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'p' as i32 as uint8_t,
                'r' as i32 as uint8_t,
                'p' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'p' as i32 as uint8_t,
                'c' as i32 as uint8_t,
                'o' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                'v' as i32 as uint8_t,
                '1' as i32 as uint8_t,
                'C' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                'v' as i32 as uint8_t,
                '2' as i32 as uint8_t,
                'C' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                's' as i32 as uint8_t,
                'p' as i32 as uint8_t,
                'e' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'p' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                'x' as i32 as uint8_t,
                'i' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'p' as i32 as uint8_t,
                'a' as i32 as uint8_t,
                's' as i32 as uint8_t,
                'p' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'c' as i32 as uint8_t,
                'o' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'r' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                'u' as i32 as uint8_t,
                'x' as i32 as uint8_t,
                'C' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'c' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'a' as i32 as uint8_t,
                'p' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'r' as i32 as uint8_t,
                'o' as i32 as uint8_t,
                't' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'm' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                'r' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'c' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'i' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'c' as i32 as uint8_t,
                'c' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'v' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'm' as i32 as uint8_t,
                'd' as i32 as uint8_t,
                'c' as i32 as uint8_t,
                'v' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                'm' as i32 as uint8_t,
                'v' as i32 as uint8_t,
                'e' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'r' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                'v' as i32 as uint8_t,
                'e' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'n' as i32 as uint8_t,
                'd' as i32 as uint8_t,
                'w' as i32 as uint8_t,
                't' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                '1' as i32 as uint8_t,
                'o' as i32 as uint8_t,
                'p' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'l' as i32 as uint8_t,
                's' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                'l' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                '1' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                'x' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'c' as i32 as uint8_t,
                'm' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                'n' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'c' as i32 as uint8_t,
                'm' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                'x' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'p' as i32 as uint8_t,
                'm' as i32 as uint8_t,
                'a' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'i' as i32 as uint8_t,
                'r' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                'f' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                'u' as i32 as uint8_t,
                'x' as i32 as uint8_t,
                'l' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                't' as i32 as uint8_t,
                'h' as i32 as uint8_t,
                'm' as i32 as uint8_t,
                'b' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'd' as i32 as uint8_t,
                'i' as i32 as uint8_t,
                'm' as i32 as uint8_t,
                'g' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'p' as i32 as uint8_t,
                'r' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                'm' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'c' as i32 as uint8_t,
                'd' as i32 as uint8_t,
                's' as i32 as uint8_t,
                'c' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'g' as i32 as uint8_t,
                'r' as i32 as uint8_t,
                'p' as i32 as uint8_t,
                'l' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'a' as i32 as uint8_t,
                'l' as i32 as uint8_t,
                't' as i32 as uint8_t,
                'r' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                's' as i32 as uint8_t,
                't' as i32 as uint8_t,
                'e' as i32 as uint8_t,
                'r' as i32 as uint8_t,
            ],
        };
        init
    },
    {
        let mut init = avifKnownProperty {
            fourcc: [
                'm' as i32 as uint8_t,
                'd' as i32 as uint8_t,
                'a' as i32 as uint8_t,
                't' as i32 as uint8_t,
            ],
        };
        init
    },
];
static mut numKnownProperties: size_t = 0;
static mut FOURCC_BYTES: size_t = 4 as size_t;
static mut UUID_BYTES: size_t = 16 as size_t;
static mut ISO_UUID_SUFFIX: [uint8_t; 12] = [
    0 as core::ffi::c_int as uint8_t,
    0x1 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0x10 as core::ffi::c_int as uint8_t,
    0x80 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0xaa as core::ffi::c_int as uint8_t,
    0 as core::ffi::c_int as uint8_t,
    0x38 as core::ffi::c_int as uint8_t,
    0x9b as core::ffi::c_int as uint8_t,
    0x71 as core::ffi::c_int as uint8_t,
];
#[no_mangle]
pub unsafe extern "C" fn avifIsKnownPropertyType(
    mut boxtype: *const uint8_t,
) -> avifBool {
    let mut i: size_t = 0 as size_t;
    while i < numKnownProperties {
        if memcmp(
            (knownProperties[i as usize].fourcc).as_ptr() as *const core::ffi::c_void,
            boxtype as *const core::ffi::c_void,
            FOURCC_BYTES,
        ) == 0 as core::ffi::c_int
        {
            return AVIF_TRUE;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn avifIsValidUUID(mut uuid: *const uint8_t) -> avifBool {
    let mut i: size_t = 0 as size_t;
    while i < numKnownProperties {
        if memcmp(
            (knownProperties[i as usize].fourcc).as_ptr() as *const core::ffi::c_void,
            uuid as *const core::ffi::c_void,
            FOURCC_BYTES,
        ) == 0 as core::ffi::c_int
            && memcmp(
                core::ptr::addr_of!(ISO_UUID_SUFFIX).cast::<uint8_t>() as *const core::ffi::c_void,
                uuid.offset(FOURCC_BYTES as isize) as *const core::ffi::c_void,
                UUID_BYTES.wrapping_sub(FOURCC_BYTES),
            ) == 0 as core::ffi::c_int
        {
            return AVIF_FALSE;
        }
        i = i.wrapping_add(1);
    }
    let mut variant: uint8_t = (*uuid.offset(8 as core::ffi::c_int as isize)
        as core::ffi::c_int >> 4 as core::ffi::c_int) as uint8_t;
    if (variant as core::ffi::c_int) < 0x8 as core::ffi::c_int
        || variant as core::ffi::c_int > 0xb as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    let mut version: uint8_t = (*uuid.offset(6 as core::ffi::c_int as isize)
        as core::ffi::c_int >> 4 as core::ffi::c_int) as uint8_t;
    if (version as core::ffi::c_int) < 1 as core::ffi::c_int
        || version as core::ffi::c_int > 8 as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn run_static_initializers() {
    numKnownProperties = (::core::mem::size_of::<[avifKnownProperty; 45]>() as size_t)
        .wrapping_div(::core::mem::size_of::<avifKnownProperty>() as size_t);
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
