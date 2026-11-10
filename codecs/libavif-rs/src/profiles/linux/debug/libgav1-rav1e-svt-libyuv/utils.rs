// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/utils.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
    fn fabs(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn floor(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn round(__x: core::ffi::c_double) -> core::ffi::c_double;
    fn floorf(__x: core::ffi::c_float) -> core::ffi::c_float;
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
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type __uint64_t = u64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type avifBool = core::ffi::c_int;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifFraction {
    pub n: int32_t,
    pub d: int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSignedFraction {
    pub n: int32_t,
    pub d: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifUnsignedFraction {
    pub n: uint32_t,
    pub d: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifArrayInternal {
    pub ptr: *mut uint8_t,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT32_MIN: core::ffi::c_int = -(2147483647 as core::ffi::c_int)
    - 1 as core::ffi::c_int;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const UINT32_MAX: core::ffi::c_uint = 4294967295 as core::ffi::c_uint;
pub const SIZE_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn avifRoundf(mut v: core::ffi::c_float) -> core::ffi::c_float {
    return floorf(v + 0.5f32);
}
#[no_mangle]
pub unsafe extern "C" fn avifHTONS(mut s: uint16_t) -> uint16_t {
    let mut result: uint16_t = 0 as uint16_t;
    let mut data: *mut uint8_t = &mut result as *mut uint16_t as *mut uint8_t;
    *data.offset(0 as core::ffi::c_int as isize) = (s as core::ffi::c_int
        >> 8 as core::ffi::c_int & 0xff as core::ffi::c_int) as uint8_t;
    *data.offset(1 as core::ffi::c_int as isize) = (s as core::ffi::c_int
        >> 0 as core::ffi::c_int & 0xff as core::ffi::c_int) as uint8_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn avifNTOHS(mut s: uint16_t) -> uint16_t {
    let mut data: *const uint8_t = &mut s as *mut uint16_t as *const uint8_t;
    return ((*data.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 0 as core::ffi::c_int
        | (*data.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
            << 8 as core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifCTOHS(mut s: uint16_t) -> uint16_t {
    let mut data: *const uint8_t = &mut s as *mut uint16_t as *const uint8_t;
    return ((*data.offset(0 as core::ffi::c_int as isize) as core::ffi::c_int)
        << 0 as core::ffi::c_int
        | (*data.offset(1 as core::ffi::c_int as isize) as core::ffi::c_int)
            << 8 as core::ffi::c_int) as uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifHTONL(mut l: uint32_t) -> uint32_t {
    let mut result: uint32_t = 0 as uint32_t;
    let mut data: *mut uint8_t = &mut result as *mut uint32_t as *mut uint8_t;
    *data.offset(0 as core::ffi::c_int as isize) = (l >> 24 as core::ffi::c_int
        & 0xff as uint32_t) as uint8_t;
    *data.offset(1 as core::ffi::c_int as isize) = (l >> 16 as core::ffi::c_int
        & 0xff as uint32_t) as uint8_t;
    *data.offset(2 as core::ffi::c_int as isize) = (l >> 8 as core::ffi::c_int
        & 0xff as uint32_t) as uint8_t;
    *data.offset(3 as core::ffi::c_int as isize) = (l >> 0 as core::ffi::c_int
        & 0xff as uint32_t) as uint8_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn avifNTOHL(mut l: uint32_t) -> uint32_t {
    let mut data: *const uint8_t = &mut l as *mut uint32_t as *const uint8_t;
    return (*data.offset(3 as core::ffi::c_int as isize) as uint32_t)
        << 0 as core::ffi::c_int
        | (*data.offset(2 as core::ffi::c_int as isize) as uint32_t)
            << 8 as core::ffi::c_int
        | (*data.offset(1 as core::ffi::c_int as isize) as uint32_t)
            << 16 as core::ffi::c_int
        | (*data.offset(0 as core::ffi::c_int as isize) as uint32_t)
            << 24 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifCTOHL(mut l: uint32_t) -> uint32_t {
    let mut data: *const uint8_t = &mut l as *mut uint32_t as *const uint8_t;
    return (*data.offset(0 as core::ffi::c_int as isize) as uint32_t)
        << 0 as core::ffi::c_int
        | (*data.offset(1 as core::ffi::c_int as isize) as uint32_t)
            << 8 as core::ffi::c_int
        | (*data.offset(2 as core::ffi::c_int as isize) as uint32_t)
            << 16 as core::ffi::c_int
        | (*data.offset(3 as core::ffi::c_int as isize) as uint32_t)
            << 24 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifHTON64(mut l: uint64_t) -> uint64_t {
    let mut result: uint64_t = 0 as uint64_t;
    let mut data: *mut uint8_t = &mut result as *mut uint64_t as *mut uint8_t;
    *data.offset(0 as core::ffi::c_int as isize) = (l >> 56 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    *data.offset(1 as core::ffi::c_int as isize) = (l >> 48 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    *data.offset(2 as core::ffi::c_int as isize) = (l >> 40 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    *data.offset(3 as core::ffi::c_int as isize) = (l >> 32 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    *data.offset(4 as core::ffi::c_int as isize) = (l >> 24 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    *data.offset(5 as core::ffi::c_int as isize) = (l >> 16 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    *data.offset(6 as core::ffi::c_int as isize) = (l >> 8 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    *data.offset(7 as core::ffi::c_int as isize) = (l >> 0 as core::ffi::c_int
        & 0xff as uint64_t) as uint8_t;
    return result;
}
#[no_mangle]
pub unsafe extern "C" fn avifNTOH64(mut l: uint64_t) -> uint64_t {
    let mut data: *const uint8_t = &mut l as *mut uint64_t as *const uint8_t;
    return (*data.offset(7 as core::ffi::c_int as isize) as uint64_t)
        << 0 as core::ffi::c_int
        | (*data.offset(6 as core::ffi::c_int as isize) as uint64_t)
            << 8 as core::ffi::c_int
        | (*data.offset(5 as core::ffi::c_int as isize) as uint64_t)
            << 16 as core::ffi::c_int
        | (*data.offset(4 as core::ffi::c_int as isize) as uint64_t)
            << 24 as core::ffi::c_int
        | (*data.offset(3 as core::ffi::c_int as isize) as uint64_t)
            << 32 as core::ffi::c_int
        | (*data.offset(2 as core::ffi::c_int as isize) as uint64_t)
            << 40 as core::ffi::c_int
        | (*data.offset(1 as core::ffi::c_int as isize) as uint64_t)
            << 48 as core::ffi::c_int
        | (*data.offset(0 as core::ffi::c_int as isize) as uint64_t)
            << 56 as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifArrayCreate(
    mut arrayStruct: *mut core::ffi::c_void,
    mut elementSize: uint32_t,
    mut initialCapacity: uint32_t,
) -> avifBool {
    let mut arr: *mut avifArrayInternal = arrayStruct as *mut avifArrayInternal;
    (*arr).elementSize = if elementSize != 0 { elementSize } else { 1 as uint32_t };
    (*arr).count = 0 as uint32_t;
    (*arr).capacity = initialCapacity;
    if (*arr).capacity as core::ffi::c_ulong
        > SIZE_MAX.wrapping_div((*arr).elementSize as core::ffi::c_ulong)
    {
        (*arr).ptr = 0 as *mut uint8_t;
        (*arr).capacity = 0 as uint32_t;
        return AVIF_FALSE;
    }
    let mut byteCount: size_t = ((*arr).elementSize as size_t)
        .wrapping_mul((*arr).capacity as size_t);
    (*arr).ptr = avifAlloc(byteCount) as *mut uint8_t;
    if ((*arr).ptr).is_null() {
        (*arr).capacity = 0 as uint32_t;
        return AVIF_FALSE;
    }
    memset((*arr).ptr as *mut core::ffi::c_void, 0 as core::ffi::c_int, byteCount);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifArrayPush(
    mut arrayStruct: *mut core::ffi::c_void,
) -> *mut core::ffi::c_void {
    let mut arr: *mut avifArrayInternal = arrayStruct as *mut avifArrayInternal;
    if (*arr).count == (*arr).capacity {
        let mut oldPtr: *mut uint8_t = (*arr).ptr;
        let mut oldByteCount: size_t = ((*arr).elementSize as size_t)
            .wrapping_mul((*arr).capacity as size_t);
        if oldByteCount > (SIZE_MAX as size_t).wrapping_div(2 as size_t)
            || (*arr).capacity > (UINT32_MAX as uint32_t).wrapping_div(2 as uint32_t)
        {
            return NULL;
        }
        let mut newByteCount: size_t = oldByteCount.wrapping_mul(2 as size_t);
        let mut newPtr: *mut uint8_t = avifAlloc(newByteCount) as *mut uint8_t;
        if newPtr.is_null() {
            return NULL;
        }
        (*arr).ptr = newPtr;
        memset(
            ((*arr).ptr).offset(oldByteCount as isize) as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            oldByteCount,
        );
        memcpy(
            (*arr).ptr as *mut core::ffi::c_void,
            oldPtr as *const core::ffi::c_void,
            oldByteCount,
        );
        (*arr).capacity = ((*arr).capacity).wrapping_mul(2 as uint32_t);
        avifFree(oldPtr as *mut core::ffi::c_void);
    }
    (*arr).count = ((*arr).count).wrapping_add(1);
    return &mut *((*arr).ptr)
        .offset(
            (((*arr).count).wrapping_sub(1 as uint32_t) as size_t)
                .wrapping_mul((*arr).elementSize as size_t) as isize,
        ) as *mut uint8_t as *mut core::ffi::c_void;
}
#[no_mangle]
pub unsafe extern "C" fn avifArrayPop(mut arrayStruct: *mut core::ffi::c_void) {
    let mut arr: *mut avifArrayInternal = arrayStruct as *mut avifArrayInternal;
    'c_2747: {
        if (*arr).count > 0 as uint32_t {} else {
            __assert_fail(
                b"arr->count > 0\0" as *const u8 as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/utils.c\0"
                    as *const u8 as *const core::ffi::c_char,
                135 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 26],
                    [core::ffi::c_char; 26],
                >(*b"void avifArrayPop(void *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*arr).count = ((*arr).count).wrapping_sub(1);
    memset(
        &mut *((*arr).ptr)
            .offset(
                ((*arr).count as size_t).wrapping_mul((*arr).elementSize as size_t)
                    as isize,
            ) as *mut uint8_t as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        (*arr).elementSize as size_t,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifArrayDestroy(mut arrayStruct: *mut core::ffi::c_void) {
    let mut arr: *mut avifArrayInternal = arrayStruct as *mut avifArrayInternal;
    if !((*arr).ptr).is_null() {
        avifFree((*arr).ptr as *mut core::ffi::c_void);
        (*arr).ptr = 0 as *mut uint8_t;
    }
    memset(
        arr as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        ::core::mem::size_of::<avifArrayInternal>() as size_t,
    );
}
unsafe extern "C" fn calcGCD(mut a: int64_t, mut b: int64_t) -> int64_t {
    if a < 0 as int64_t {
        a *= -(1 as core::ffi::c_int) as int64_t;
    }
    if b < 0 as int64_t {
        b *= -(1 as core::ffi::c_int) as int64_t;
    }
    while b != 0 as int64_t {
        let mut r: int64_t = a % b;
        a = b;
        b = r;
    }
    return a;
}
#[no_mangle]
pub unsafe extern "C" fn avifFractionSimplify(mut f: *mut avifFraction) {
    let mut gcd: int64_t = calcGCD((*f).n as int64_t, (*f).d as int64_t);
    if gcd > 1 as int64_t {
        (*f).n = ((*f).n as int64_t / gcd) as int32_t;
        (*f).d = ((*f).d as int64_t / gcd) as int32_t;
    }
}
unsafe extern "C" fn overflowsInt32(mut x: int64_t) -> avifBool {
    return (x < INT32_MIN as int64_t || x > INT32_MAX as int64_t) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifFractionCD(
    mut a: *mut avifFraction,
    mut b: *mut avifFraction,
) -> avifBool {
    avifFractionSimplify(a);
    avifFractionSimplify(b);
    if (*a).d != (*b).d {
        let ad: int64_t = (*a).d as int64_t;
        let bd: int64_t = (*b).d as int64_t;
        let anNew: int64_t = (*a).n as int64_t * bd;
        let adNew: int64_t = (*a).d as int64_t * bd;
        let bnNew: int64_t = (*b).n as int64_t * ad;
        let bdNew: int64_t = (*b).d as int64_t * ad;
        if overflowsInt32(anNew) != 0 || overflowsInt32(adNew) != 0
            || overflowsInt32(bnNew) != 0 || overflowsInt32(bdNew) != 0
        {
            return AVIF_FALSE;
        }
        (*a).n = anNew as int32_t;
        (*a).d = adNew as int32_t;
        (*b).n = bnNew as int32_t;
        (*b).d = bdNew as int32_t;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifFractionAdd(
    mut a: avifFraction,
    mut b: avifFraction,
    mut result: *mut avifFraction,
) -> avifBool {
    if avifFractionCD(&mut a, &mut b) == 0 {
        return AVIF_FALSE;
    }
    let resultN: int64_t = a.n as int64_t + b.n as int64_t;
    if overflowsInt32(resultN) != 0 {
        return AVIF_FALSE;
    }
    (*result).n = resultN as int32_t;
    (*result).d = a.d;
    avifFractionSimplify(result);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifFractionSub(
    mut a: avifFraction,
    mut b: avifFraction,
    mut result: *mut avifFraction,
) -> avifBool {
    if avifFractionCD(&mut a, &mut b) == 0 {
        return AVIF_FALSE;
    }
    let resultN: int64_t = a.n as int64_t - b.n as int64_t;
    if overflowsInt32(resultN) != 0 {
        return AVIF_FALSE;
    }
    (*result).n = resultN as int32_t;
    (*result).d = a.d;
    avifFractionSimplify(result);
    return AVIF_TRUE;
}
unsafe extern "C" fn avifDoubleToUnsignedFractionImpl(
    mut v: core::ffi::c_double,
    mut maxNumerator: uint32_t,
    mut numerator: *mut uint32_t,
    mut denominator: *mut uint32_t,
) -> avifBool {
    if v.is_nan() as i32 != 0 || v < 0 as core::ffi::c_int as core::ffi::c_double
        || v > maxNumerator as core::ffi::c_double
    {
        return AVIF_FALSE;
    }
    let maxD: uint32_t = if v <= 1 as core::ffi::c_int as core::ffi::c_double {
        UINT32_MAX as uint32_t
    } else {
        floor(maxNumerator as core::ffi::c_double / v) as uint32_t
    };
    *denominator = 1 as uint32_t;
    let mut previousD: uint32_t = 0 as uint32_t;
    let mut currentV: core::ffi::c_double = v - floor(v);
    let mut iter: core::ffi::c_int = 0 as core::ffi::c_int;
    let maxIter: core::ffi::c_int = 39 as core::ffi::c_int;
    while iter < maxIter {
        let numeratorDouble: core::ffi::c_double = *denominator as core::ffi::c_double
            * v;
        'c_808: {
            if numeratorDouble <= maxNumerator as core::ffi::c_double {} else {
                __assert_fail(
                    b"numeratorDouble <= maxNumerator\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/utils.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    259 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 84],
                        [core::ffi::c_char; 84],
                    >(
                        *b"avifBool avifDoubleToUnsignedFractionImpl(double, uint32_t, uint32_t *, uint32_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *numerator = round(numeratorDouble) as uint32_t;
        if fabs(numeratorDouble - *numerator as core::ffi::c_double) == 0.0f64 {
            return AVIF_TRUE;
        }
        currentV = 1.0f64 / currentV;
        let newD: core::ffi::c_double = previousD as core::ffi::c_double
            + floor(currentV) as core::ffi::c_double
                * *denominator as core::ffi::c_double;
        if newD > maxD as core::ffi::c_double {
            return AVIF_TRUE;
        }
        previousD = *denominator;
        'c_677: {
            if newD <= 4294967295 as core::ffi::c_uint as core::ffi::c_double {} else {
                __assert_fail(
                    b"newD <= UINT32_MAX\0" as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/utils.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    271 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 84],
                        [core::ffi::c_char; 84],
                    >(
                        *b"avifBool avifDoubleToUnsignedFractionImpl(double, uint32_t, uint32_t *, uint32_t *)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        *denominator = newD as uint32_t;
        currentV -= floor(currentV);
        iter += 1;
    }
    *numerator = round(*denominator as core::ffi::c_double * v) as uint32_t;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifDoubleToSignedFraction(
    mut v: core::ffi::c_double,
    mut fraction: *mut avifSignedFraction,
) -> avifBool {
    let mut positive_numerator: uint32_t = 0;
    if avifDoubleToUnsignedFractionImpl(
        fabs(v),
        INT32_MAX as uint32_t,
        &mut positive_numerator,
        &mut (*fraction).d,
    ) == 0
    {
        return AVIF_FALSE;
    }
    (*fraction).n = positive_numerator as int32_t;
    if v < 0 as core::ffi::c_int as core::ffi::c_double {
        (*fraction).n = ((*fraction).n as core::ffi::c_int * -(1 as core::ffi::c_int))
            as int32_t;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifDoubleToUnsignedFraction(
    mut v: core::ffi::c_double,
    mut fraction: *mut avifUnsignedFraction,
) -> avifBool {
    return avifDoubleToUnsignedFractionImpl(
        v,
        UINT32_MAX as uint32_t,
        &mut (*fraction).n,
        &mut (*fraction).d,
    );
}
