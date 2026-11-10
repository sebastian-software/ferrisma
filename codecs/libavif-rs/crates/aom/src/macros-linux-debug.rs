/* Optional test subjects expanded from the pinned original production headers. */

// Mechanically translated from tests/ports/aom_macro_subjects.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static mut OD_DIVU_SMALL_CONSTS: [[uint32_t; 2]; 1024];
}
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
#[no_mangle]
pub unsafe extern "C-unwind" fn port_aom_macro_OD_DIVU_SMALL(
    mut x: uint32_t,
    mut d: uint32_t,
) -> uint32_t {
    return ((OD_DIVU_SMALL_CONSTS[d.wrapping_sub(1 as uint32_t)
        as usize][0 as core::ffi::c_int as usize] as uint64_t)
        .wrapping_mul(x as uint64_t)
        .wrapping_add(
            OD_DIVU_SMALL_CONSTS[d.wrapping_sub(1 as uint32_t)
                as usize][1 as core::ffi::c_int as usize] as uint64_t,
        ) >> 32 as core::ffi::c_int) as uint32_t
        >> 1 as core::ffi::c_int + get_msb(d as core::ffi::c_uint)
            - 1 as core::ffi::c_int;
}
pub const __ASSERT_FUNCTION: [core::ffi::c_char; 26] = unsafe {
    ::core::mem::transmute::<
        [u8; 26],
        [core::ffi::c_char; 26],
    >(*b"int get_msb(unsigned int)\0")
};
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    if n != 0 as core::ffi::c_uint {} else {
        __assert_fail(
            b"n != 0\0" as *const u8 as *const core::ffi::c_char,
            b"upstream/aom/aom_ports/bitops.h\0" as *const u8
                as *const core::ffi::c_char,
            43 as core::ffi::c_uint,
            __ASSERT_FUNCTION.as_ptr(),
        );
    };
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}

extern "C" {
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
}
