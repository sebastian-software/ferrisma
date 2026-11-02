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
#[inline]
unsafe extern "C-unwind" fn get_msb(mut n: core::ffi::c_uint) -> core::ffi::c_int {
    return 31 as core::ffi::c_int ^ n.leading_zeros() as i32;
}
