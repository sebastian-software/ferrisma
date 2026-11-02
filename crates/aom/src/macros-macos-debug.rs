/* Optional test subjects expanded from the pinned original production headers. */

// Mechanically translated from tests/ports/aom_macro_subjects.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C-unwind" {
    static mut OD_DIVU_SMALL_CONSTS: [[uint32_t; 2]; 1024];
}
pub type uint32_t = u32;
pub type uint64_t = u64;
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
