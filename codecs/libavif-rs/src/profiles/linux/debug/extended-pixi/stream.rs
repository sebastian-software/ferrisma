// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/stream.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifRWDataRealloc(raw: *mut avifRWData, newSize: size_t) -> avifResult;
    fn avifRWDataFree(raw: *mut avifRWData);
    fn avifHTONS(s: uint16_t) -> uint16_t;
    fn avifNTOHS(s: uint16_t) -> uint16_t;
    fn avifCTOHS(s: uint16_t) -> uint16_t;
    fn avifHTONL(l: uint32_t) -> uint32_t;
    fn avifNTOHL(l: uint32_t) -> uint32_t;
    fn avifCTOHL(l: uint32_t) -> uint32_t;
    fn avifHTON64(l: uint64_t) -> uint64_t;
    fn avifNTOH64(l: uint64_t) -> uint64_t;
    fn avifDiagnosticsPrintf(
        diag: *mut avifDiagnostics,
        format: *const core::ffi::c_char,
        ...
    );
    fn __assert_fail(
        __assertion: *const core::ffi::c_char,
        __file: *const core::ffi::c_char,
        __line: core::ffi::c_uint,
        __function: *const core::ffi::c_char,
    ) -> !;
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
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
    fn strlen(__s: *const core::ffi::c_char) -> size_t;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type uint64_t = __uint64_t;
pub type avifBool = core::ffi::c_int;
pub type avifResult = core::ffi::c_uint;
pub const AVIF_RESULT_NO_AV1_ITEMS_FOUND: avifResult = 10;
pub const AVIF_RESULT_DECODE_SAMPLE_TRANSFORM_FAILED: avifResult = 34;
pub const AVIF_RESULT_ENCODE_SAMPLE_TRANSFORM_FAILED: avifResult = 33;
pub const AVIF_RESULT_INVALID_TONE_MAPPED_IMAGE: avifResult = 32;
pub const AVIF_RESULT_DECODE_GAIN_MAP_FAILED: avifResult = 31;
pub const AVIF_RESULT_ENCODE_GAIN_MAP_FAILED: avifResult = 30;
pub const AVIF_RESULT_INTERNAL_ERROR: avifResult = 29;
pub const AVIF_RESULT_INCOMPATIBLE_IMAGE: avifResult = 28;
pub const AVIF_RESULT_CANNOT_CHANGE_SETTING: avifResult = 27;
pub const AVIF_RESULT_OUT_OF_MEMORY: avifResult = 26;
pub const AVIF_RESULT_NOT_IMPLEMENTED: avifResult = 25;
pub const AVIF_RESULT_INVALID_ARGUMENT: avifResult = 24;
pub const AVIF_RESULT_WAITING_ON_IO: avifResult = 23;
pub const AVIF_RESULT_IO_ERROR: avifResult = 22;
pub const AVIF_RESULT_IO_NOT_SET: avifResult = 21;
pub const AVIF_RESULT_TRUNCATED_DATA: avifResult = 20;
pub const AVIF_RESULT_INVALID_CODEC_SPECIFIC_OPTION: avifResult = 19;
pub const AVIF_RESULT_INVALID_IMAGE_GRID: avifResult = 18;
pub const AVIF_RESULT_INVALID_EXIF_PAYLOAD: avifResult = 17;
pub const AVIF_RESULT_NO_IMAGES_REMAINING: avifResult = 16;
pub const AVIF_RESULT_NO_CODEC_AVAILABLE: avifResult = 15;
pub const AVIF_RESULT_ISPE_SIZE_MISMATCH: avifResult = 14;
pub const AVIF_RESULT_COLOR_ALPHA_SIZE_MISMATCH: avifResult = 13;
pub const AVIF_RESULT_DECODE_ALPHA_FAILED: avifResult = 12;
pub const AVIF_RESULT_DECODE_COLOR_FAILED: avifResult = 11;
pub const AVIF_RESULT_MISSING_IMAGE_ITEM: avifResult = 10;
pub const AVIF_RESULT_BMFF_PARSE_FAILED: avifResult = 9;
pub const AVIF_RESULT_ENCODE_ALPHA_FAILED: avifResult = 8;
pub const AVIF_RESULT_ENCODE_COLOR_FAILED: avifResult = 7;
pub const AVIF_RESULT_UNSUPPORTED_DEPTH: avifResult = 6;
pub const AVIF_RESULT_REFORMAT_FAILED: avifResult = 5;
pub const AVIF_RESULT_NO_YUV_FORMAT_SELECTED: avifResult = 4;
pub const AVIF_RESULT_NO_CONTENT: avifResult = 3;
pub const AVIF_RESULT_INVALID_FTYP: avifResult = 2;
pub const AVIF_RESULT_UNKNOWN_ERROR: avifResult = 1;
pub const AVIF_RESULT_OK: avifResult = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifROData {
    pub data: *const uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRWData {
    pub data: *mut uint8_t,
    pub size: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDiagnostics {
    pub error: [core::ffi::c_char; 256],
}
pub type avifBoxMarker = size_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifBoxHeader {
    pub isSizeZeroBox: avifBool,
    pub size: size_t,
    pub type_0: [uint8_t; 4],
    pub usertype: [uint8_t; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifROStream {
    pub raw: *mut avifROData,
    pub offset: size_t,
    pub numUsedBitsInPartialByte: size_t,
    pub diag: *mut avifDiagnostics,
    pub diagContext: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifRWStream {
    pub raw: *mut avifRWData,
    pub offset: size_t,
    pub numUsedBitsInPartialByte: size_t,
}
pub const SIZE_MAX: core::ffi::c_ulong = 18446744073709551615 as core::ffi::c_ulong;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn avifROStreamCurrent(
    mut stream: *mut avifROStream,
) -> *const uint8_t {
    return ((*(*stream).raw).data).offset((*stream).offset as isize);
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamStart(
    mut stream: *mut avifROStream,
    mut raw: *mut avifROData,
    mut diag: *mut avifDiagnostics,
    mut diagContext: *const core::ffi::c_char,
) {
    (*stream).raw = raw;
    (*stream).offset = 0 as size_t;
    (*stream).numUsedBitsInPartialByte = 0 as size_t;
    (*stream).diag = diag;
    (*stream).diagContext = diagContext;
    'c_2129: {
        if ((*stream).diag).is_null() || !((*stream).diagContext).is_null() {} else {
            __assert_fail(
                b"!stream->diag || stream->diagContext\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                28 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 86],
                    [core::ffi::c_char; 86],
                >(
                    *b"void avifROStreamStart(avifROStream *, avifROData *, avifDiagnostics *, const char *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamHasBytesLeft(
    mut stream: *const avifROStream,
    mut byteCount: size_t,
) -> avifBool {
    return (byteCount <= ((*(*stream).raw).size).wrapping_sub((*stream).offset))
        as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamRemainingBytes(
    mut stream: *const avifROStream,
) -> size_t {
    return ((*(*stream).raw).size).wrapping_sub((*stream).offset);
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamOffset(mut stream: *const avifROStream) -> size_t {
    return (*stream).offset;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamSetOffset(
    mut stream: *mut avifROStream,
    mut offset: size_t,
) {
    'c_2268: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                48 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 51],
                    [core::ffi::c_char; 51],
                >(*b"void avifROStreamSetOffset(avifROStream *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    (*stream).offset = offset;
    if (*stream).offset > (*(*stream).raw).size {
        (*stream).offset = (*(*stream).raw).size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamSkip(
    mut stream: *mut avifROStream,
    mut byteCount: size_t,
) -> avifBool {
    'c_2399: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                57 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 50],
                    [core::ffi::c_char; 50],
                >(*b"avifBool avifROStreamSkip(avifROStream *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if avifROStreamHasBytesLeft(stream, byteCount) == 0 {
        avifDiagnosticsPrintf(
            (*stream).diag,
            b"%s: Failed to skip %zu bytes, truncated data?\0" as *const u8
                as *const core::ffi::c_char,
            (*stream).diagContext,
            byteCount,
        );
        return AVIF_FALSE;
    }
    (*stream).offset = ((*stream).offset).wrapping_add(byteCount);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamRead(
    mut stream: *mut avifROStream,
    mut data: *mut uint8_t,
    mut size: size_t,
) -> avifBool {
    'c_2517: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                68 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 61],
                    [core::ffi::c_char; 61],
                >(*b"avifBool avifROStreamRead(avifROStream *, uint8_t *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    if avifROStreamHasBytesLeft(stream, size) == 0 {
        avifDiagnosticsPrintf(
            (*stream).diag,
            b"%s: Failed to read %zu bytes, truncated data?\0" as *const u8
                as *const core::ffi::c_char,
            (*stream).diagContext,
            size,
        );
        return AVIF_FALSE;
    }
    memcpy(
        data as *mut core::ffi::c_void,
        ((*(*stream).raw).data).offset((*stream).offset as isize)
            as *const core::ffi::c_void,
        size,
    );
    (*stream).offset = ((*stream).offset).wrapping_add(size);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadUX8(
    mut stream: *mut avifROStream,
    mut v: *mut uint64_t,
    mut factor: uint64_t,
) -> avifBool {
    'c_3188: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                81 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 67],
                    [core::ffi::c_char; 67],
                >(
                    *b"avifBool avifROStreamReadUX8(avifROStream *, uint64_t *, uint64_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if factor == 0 as uint64_t {
        *v = 0 as uint64_t;
    } else if factor == 1 as uint64_t {
        let mut tmp: uint8_t = 0;
        if avifROStreamRead(stream, &mut tmp, 1 as size_t) == 0 {
            return AVIF_FALSE;
        }
        *v = tmp as uint64_t;
    } else if factor == 2 as uint64_t {
        let mut tmp_0: uint16_t = 0;
        if avifROStreamReadU16(stream, &mut tmp_0) == 0 {
            return AVIF_FALSE;
        }
        *v = tmp_0 as uint64_t;
    } else if factor == 4 as uint64_t {
        let mut tmp_1: uint32_t = 0;
        if avifROStreamReadU32(stream, &mut tmp_1) == 0 {
            return AVIF_FALSE;
        }
        *v = tmp_1 as uint64_t;
    } else if factor == 8 as uint64_t {
        let mut tmp_2: uint64_t = 0;
        if avifROStreamReadU64(stream, &mut tmp_2) == 0 {
            return AVIF_FALSE;
        }
        *v = tmp_2;
    } else {
        avifDiagnosticsPrintf(
            (*stream).diag,
            b"%s: Failed to read UX8 value; Unsupported UX8 factor [%lu]\0" as *const u8
                as *const core::ffi::c_char,
            (*stream).diagContext,
            factor,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadU16(
    mut stream: *mut avifROStream,
    mut v: *mut uint16_t,
) -> avifBool {
    'c_2603: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                111 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 57],
                    [core::ffi::c_char; 57],
                >(*b"avifBool avifROStreamReadU16(avifROStream *, uint16_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if avifROStreamRead(
        stream,
        v as *mut uint8_t,
        ::core::mem::size_of::<uint16_t>() as size_t,
    ) == 0
    {
        return AVIF_FALSE;
    }
    *v = avifNTOHS(*v);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadU16Endianness(
    mut stream: *mut avifROStream,
    mut v: *mut uint16_t,
    mut littleEndian: avifBool,
) -> avifBool {
    'c_2700: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                119 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 77],
                    [core::ffi::c_char; 77],
                >(
                    *b"avifBool avifROStreamReadU16Endianness(avifROStream *, uint16_t *, avifBool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if avifROStreamRead(
        stream,
        v as *mut uint8_t,
        ::core::mem::size_of::<uint16_t>() as size_t,
    ) == 0
    {
        return AVIF_FALSE;
    }
    *v = (if littleEndian != 0 {
        avifCTOHS(*v) as core::ffi::c_int
    } else {
        avifNTOHS(*v) as core::ffi::c_int
    }) as uint16_t;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadU32(
    mut stream: *mut avifROStream,
    mut v: *mut uint32_t,
) -> avifBool {
    'c_2785: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                127 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 57],
                    [core::ffi::c_char; 57],
                >(*b"avifBool avifROStreamReadU32(avifROStream *, uint32_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if avifROStreamRead(
        stream,
        v as *mut uint8_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) == 0
    {
        return AVIF_FALSE;
    }
    *v = avifNTOHL(*v);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadU32Endianness(
    mut stream: *mut avifROStream,
    mut v: *mut uint32_t,
    mut littleEndian: avifBool,
) -> avifBool {
    'c_2878: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                135 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 77],
                    [core::ffi::c_char; 77],
                >(
                    *b"avifBool avifROStreamReadU32Endianness(avifROStream *, uint32_t *, avifBool)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if avifROStreamRead(
        stream,
        v as *mut uint8_t,
        ::core::mem::size_of::<uint32_t>() as size_t,
    ) == 0
    {
        return AVIF_FALSE;
    }
    *v = if littleEndian != 0 { avifCTOHL(*v) } else { avifNTOHL(*v) };
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadU64(
    mut stream: *mut avifROStream,
    mut v: *mut uint64_t,
) -> avifBool {
    'c_3034: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                143 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 57],
                    [core::ffi::c_char; 57],
                >(*b"avifBool avifROStreamReadU64(avifROStream *, uint64_t *)\0"))
                    .as_ptr(),
            );
        }
    };
    if avifROStreamRead(
        stream,
        v as *mut uint8_t,
        ::core::mem::size_of::<uint64_t>() as size_t,
    ) == 0
    {
        return AVIF_FALSE;
    }
    *v = avifNTOH64(*v);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamSkipBits(
    mut stream: *mut avifROStream,
    mut bitCount: size_t,
) -> avifBool {
    if (*stream).numUsedBitsInPartialByte != 0 as size_t {
        'c_3999: {
            if (*stream).numUsedBitsInPartialByte < 8 as size_t {} else {
                __assert_fail(
                    b"stream->numUsedBitsInPartialByte < 8\0" as *const u8
                        as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    152 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 54],
                        [core::ffi::c_char; 54],
                    >(*b"avifBool avifROStreamSkipBits(avifROStream *, size_t)\0"))
                        .as_ptr(),
                );
            }
        };
        let padding: size_t = if (8 as size_t)
            .wrapping_sub((*stream).numUsedBitsInPartialByte) < bitCount
        {
            (8 as size_t).wrapping_sub((*stream).numUsedBitsInPartialByte)
        } else {
            bitCount
        };
        (*stream).numUsedBitsInPartialByte = ((*stream).numUsedBitsInPartialByte)
            .wrapping_add(padding)
            .wrapping_rem(8 as size_t);
        bitCount = bitCount.wrapping_sub(padding);
        if bitCount == 0 as size_t {
            return AVIF_TRUE;
        }
    }
    let num_bytes: size_t = bitCount.wrapping_add(7 as size_t).wrapping_div(8 as size_t);
    if avifROStreamSkip(stream, num_bytes) == 0 {
        return AVIF_FALSE;
    }
    (*stream).numUsedBitsInPartialByte = bitCount.wrapping_rem(8 as size_t);
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadBitsU8(
    mut stream: *mut avifROStream,
    mut v: *mut uint8_t,
    mut bitCount: size_t,
) -> avifBool {
    if !(bitCount
        <= (::core::mem::size_of::<uint8_t>() as usize).wrapping_mul(8 as usize))
    {
        return AVIF_FALSE;
    }
    let mut vU32: uint32_t = 0;
    if avifROStreamReadBitsU32(stream, &mut vU32, bitCount) == 0 {
        return AVIF_FALSE;
    }
    *v = vU32 as uint8_t;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadBitsU16(
    mut stream: *mut avifROStream,
    mut v: *mut uint16_t,
    mut bitCount: size_t,
) -> avifBool {
    if !(bitCount
        <= (::core::mem::size_of::<uint16_t>() as usize).wrapping_mul(8 as usize))
    {
        return AVIF_FALSE;
    }
    let mut vU32: uint32_t = 0;
    if avifROStreamReadBitsU32(stream, &mut vU32, bitCount) == 0 {
        return AVIF_FALSE;
    }
    *v = vU32 as uint16_t;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadBitsU32(
    mut stream: *mut avifROStream,
    mut v: *mut uint32_t,
    mut bitCount: size_t,
) -> avifBool {
    if !(bitCount
        <= (::core::mem::size_of::<uint32_t>() as usize).wrapping_mul(8 as usize))
    {
        return AVIF_FALSE;
    }
    *v = 0 as uint32_t;
    while bitCount != 0 {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {
            if avifROStreamSkip(stream, ::core::mem::size_of::<uint8_t>() as size_t) == 0
            {
                return AVIF_FALSE;
            }
        }
        'c_4210: {
            if (*stream).offset > 0 as size_t {} else {
                __assert_fail(
                    b"stream->offset > 0\0" as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    192 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 69],
                        [core::ffi::c_char; 69],
                    >(
                        *b"avifBool avifROStreamReadBitsU32(avifROStream *, uint32_t *, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut packedBits: *const uint8_t = ((*(*stream).raw).data)
            .offset((*stream).offset as isize)
            .offset(-(1 as core::ffi::c_int as isize));
        let numBits: size_t = if bitCount
            < (8 as size_t).wrapping_sub((*stream).numUsedBitsInPartialByte)
        {
            bitCount
        } else {
            (8 as size_t).wrapping_sub((*stream).numUsedBitsInPartialByte)
        };
        (*stream).numUsedBitsInPartialByte = ((*stream).numUsedBitsInPartialByte)
            .wrapping_add(numBits);
        bitCount = bitCount.wrapping_sub(numBits);
        let bits: uint32_t = (*packedBits as core::ffi::c_int
            >> (8 as size_t).wrapping_sub((*stream).numUsedBitsInPartialByte)
            & ((1 as core::ffi::c_int) << numBits) - 1 as core::ffi::c_int) as uint32_t;
        *v |= bits << bitCount;
        if (*stream).numUsedBitsInPartialByte == 8 as size_t {
            (*stream).numUsedBitsInPartialByte = 0 as size_t;
        }
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadString(
    mut stream: *mut avifROStream,
    mut output: *mut core::ffi::c_char,
    mut outputSize: size_t,
) -> avifBool {
    'c_3381: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                216 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 64],
                    [core::ffi::c_char; 64],
                >(*b"avifBool avifROStreamReadString(avifROStream *, char *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut remainingBytes: size_t = avifROStreamRemainingBytes(stream);
    let mut p: *const uint8_t = avifROStreamCurrent(stream);
    let mut foundNullTerminator: avifBool = AVIF_FALSE;
    let mut i: size_t = 0 as size_t;
    while i < remainingBytes {
        if *p.offset(i as isize) as core::ffi::c_int == 0 as core::ffi::c_int {
            foundNullTerminator = AVIF_TRUE as avifBool;
            break;
        } else {
            i = i.wrapping_add(1);
        }
    }
    if foundNullTerminator == 0 {
        avifDiagnosticsPrintf(
            (*stream).diag,
            b"%s: Failed to find a NULL terminator when reading a string\0" as *const u8
                as *const core::ffi::c_char,
            (*stream).diagContext,
        );
        return AVIF_FALSE;
    }
    let mut streamString: *const core::ffi::c_char = p as *const core::ffi::c_char;
    let mut stringLen: size_t = strlen(streamString);
    (*stream).offset = ((*stream).offset)
        .wrapping_add(stringLen.wrapping_add(1 as size_t));
    if !output.is_null() && outputSize != 0 {
        if stringLen >= outputSize {
            stringLen = outputSize.wrapping_sub(1 as size_t);
        }
        memcpy(
            output as *mut core::ffi::c_void,
            streamString as *const core::ffi::c_void,
            stringLen,
        );
        *output.offset(stringLen as isize) = 0 as core::ffi::c_char;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadBoxHeaderPartial(
    mut stream: *mut avifROStream,
    mut header: *mut avifBoxHeader,
    mut topLevel: avifBool,
) -> avifBool {
    let mut startOffset: size_t = (*stream).offset;
    let mut smallSize: uint32_t = 0;
    if avifROStreamReadU32(stream, &mut smallSize) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamRead(stream, ((*header).type_0).as_mut_ptr(), 4 as size_t) == 0 {
        return AVIF_FALSE;
    }
    let mut size: uint64_t = smallSize as uint64_t;
    if size == 1 as uint64_t {
        if avifROStreamReadU64(stream, &mut size) == 0 {
            return AVIF_FALSE;
        }
    }
    if memcmp(
        ((*header).type_0).as_mut_ptr() as *const core::ffi::c_void,
        b"uuid\0" as *const u8 as *const core::ffi::c_char as *const core::ffi::c_void,
        4 as size_t,
    ) == 0
    {
        if avifROStreamRead(stream, ((*header).usertype).as_mut_ptr(), 16 as size_t) == 0
        {
            return AVIF_FALSE;
        }
    } else {
        memset(
            ((*header).usertype).as_mut_ptr() as *mut core::ffi::c_void,
            0 as core::ffi::c_int,
            ::core::mem::size_of::<[uint8_t; 16]>() as size_t,
        );
    }
    let mut bytesRead: size_t = ((*stream).offset).wrapping_sub(startOffset);
    if size == 0 as uint64_t {
        if topLevel == 0 {
            avifDiagnosticsPrintf(
                (*stream).diag,
                b"%s: Non-top-level box with size 0\0" as *const u8
                    as *const core::ffi::c_char,
                (*stream).diagContext,
            );
            return AVIF_FALSE;
        }
        (*header).isSizeZeroBox = AVIF_TRUE as avifBool;
        (*header).size = 0 as size_t;
        return AVIF_TRUE;
    }
    if size < bytesRead as uint64_t
        || size.wrapping_sub(bytesRead as uint64_t) > SIZE_MAX as uint64_t
    {
        avifDiagnosticsPrintf(
            (*stream).diag,
            b"%s: Header size overflow check failure\0" as *const u8
                as *const core::ffi::c_char,
            (*stream).diagContext,
        );
        return AVIF_FALSE;
    }
    (*header).isSizeZeroBox = AVIF_FALSE as avifBool;
    (*header).size = size.wrapping_sub(bytesRead as uint64_t) as size_t;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadBoxHeader(
    mut stream: *mut avifROStream,
    mut header: *mut avifBoxHeader,
) -> avifBool {
    if avifROStreamReadBoxHeaderPartial(stream, header, 0 as avifBool) == 0 {
        return AVIF_FALSE;
    }
    if avifROStreamHasBytesLeft(stream, (*header).size) == 0 {
        avifDiagnosticsPrintf(
            (*stream).diag,
            b"%s: Child box too large, possibly truncated data\0" as *const u8
                as *const core::ffi::c_char,
            (*stream).diagContext,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadVersionAndFlags(
    mut stream: *mut avifROStream,
    mut version: *mut uint8_t,
    mut flags: *mut uint32_t,
) -> avifBool {
    let mut versionAndFlags: [uint8_t; 4] = [0; 4];
    if avifROStreamRead(stream, versionAndFlags.as_mut_ptr(), 4 as size_t) == 0 {
        return AVIF_FALSE;
    }
    if !version.is_null() {
        *version = versionAndFlags[0 as core::ffi::c_int as usize];
    }
    if !flags.is_null() {
        *flags = (((versionAndFlags[1 as core::ffi::c_int as usize] as core::ffi::c_int)
            << 16 as core::ffi::c_int)
            + ((versionAndFlags[2 as core::ffi::c_int as usize] as core::ffi::c_int)
                << 8 as core::ffi::c_int)
            + ((versionAndFlags[3 as core::ffi::c_int as usize] as core::ffi::c_int)
                << 0 as core::ffi::c_int)) as uint32_t;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifROStreamReadAndEnforceVersion(
    mut stream: *mut avifROStream,
    mut enforcedVersion: uint8_t,
    mut flags: *mut uint32_t,
) -> avifBool {
    let mut version: uint8_t = 0;
    if avifROStreamReadVersionAndFlags(stream, &mut version, flags) == 0 {
        return AVIF_FALSE;
    }
    if version as core::ffi::c_int != enforcedVersion as core::ffi::c_int {
        avifDiagnosticsPrintf(
            (*stream).diag,
            b"%s: Expecting box version %u, got version %u\0" as *const u8
                as *const core::ffi::c_char,
            (*stream).diagContext,
            enforcedVersion as core::ffi::c_int,
            version as core::ffi::c_int,
        );
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
pub const AVIF_STREAM_BUFFER_INCREMENT: core::ffi::c_int = 1024 as core::ffi::c_int
    * 1024 as core::ffi::c_int;
unsafe extern "C" fn makeRoom(
    mut stream: *mut avifRWStream,
    mut size: size_t,
) -> avifResult {
    if !(size <= (18446744073709551615 as size_t).wrapping_sub((*stream).offset)) {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let mut newSize: size_t = ((*stream).offset).wrapping_add(size);
    if newSize <= (*(*stream).raw).size {
        return AVIF_RESULT_OK;
    }
    let mut rem: size_t = newSize.wrapping_rem(AVIF_STREAM_BUFFER_INCREMENT as size_t);
    let mut padding: size_t = if rem == 0 as size_t {
        0 as size_t
    } else {
        (AVIF_STREAM_BUFFER_INCREMENT as size_t).wrapping_sub(rem)
    };
    if !(newSize <= (18446744073709551615 as size_t).wrapping_sub(padding)) {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    newSize = newSize.wrapping_add(padding);
    return avifRWDataRealloc((*stream).raw, newSize);
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamStart(
    mut stream: *mut avifRWStream,
    mut raw: *mut avifRWData,
) {
    (*stream).raw = raw;
    (*stream).offset = 0 as size_t;
    (*stream).numUsedBitsInPartialByte = 0 as size_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamOffset(mut stream: *const avifRWStream) -> size_t {
    return (*stream).offset;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamSetOffset(
    mut stream: *mut avifRWStream,
    mut offset: size_t,
) {
    (*stream).offset = offset;
    if (*stream).offset > (*(*stream).raw).size {
        (*stream).offset = (*(*stream).raw).size;
    }
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamFinishWrite(mut stream: *mut avifRWStream) {
    if (*(*stream).raw).size != (*stream).offset {
        if (*stream).offset != 0 {
            (*(*stream).raw).size = (*stream).offset;
        } else {
            avifRWDataFree((*stream).raw);
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWrite(
    mut stream: *mut avifRWStream,
    mut data: *const core::ffi::c_void,
    mut size: size_t,
) -> avifResult {
    'c_4696: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                383 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 67],
                    [core::ffi::c_char; 67],
                >(
                    *b"avifResult avifRWStreamWrite(avifRWStream *, const void *, size_t)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if size != 0 {
        let result__: avifResult = makeRoom(stream, size) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        memcpy(
            ((*(*stream).raw).data).offset((*stream).offset as isize)
                as *mut core::ffi::c_void,
            data,
            size,
        );
        (*stream).offset = ((*stream).offset).wrapping_add(size);
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteChars(
    mut stream: *mut avifRWStream,
    mut chars: *const core::ffi::c_char,
    mut size: size_t,
) -> avifResult {
    return avifRWStreamWrite(stream, chars as *const core::ffi::c_void, size);
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteFullBox(
    mut stream: *mut avifRWStream,
    mut type_0: *const core::ffi::c_char,
    mut contentSize: size_t,
    mut version: core::ffi::c_int,
    mut flags: uint32_t,
    mut marker: *mut avifBoxMarker,
) -> avifResult {
    'c_5040: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                399 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 106],
                    [core::ffi::c_char; 106],
                >(
                    *b"avifResult avifRWStreamWriteFullBox(avifRWStream *, const char *, size_t, int, uint32_t, avifBoxMarker *)\0",
                ))
                    .as_ptr(),
            );
        }
    };
    if !marker.is_null() {
        *marker = (*stream).offset as avifBoxMarker;
    }
    let mut headerSize: size_t = (::core::mem::size_of::<uint32_t>() as size_t)
        .wrapping_add(4 as size_t);
    if version != -(1 as core::ffi::c_int) {
        headerSize = headerSize.wrapping_add(4 as size_t);
    }
    let result__: avifResult = makeRoom(stream, headerSize) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    memset(
        ((*(*stream).raw).data).offset((*stream).offset as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        headerSize,
    );
    let mut noSize: uint32_t = avifHTONL(
        headerSize.wrapping_add(contentSize) as uint32_t,
    );
    memcpy(
        ((*(*stream).raw).data).offset((*stream).offset as isize)
            as *mut core::ffi::c_void,
        &mut noSize as *mut uint32_t as *const core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    memcpy(
        ((*(*stream).raw).data)
            .offset((*stream).offset as isize)
            .offset(4 as core::ffi::c_int as isize) as *mut core::ffi::c_void,
        type_0 as *const core::ffi::c_void,
        4 as size_t,
    );
    if version != -(1 as core::ffi::c_int) {
        *((*(*stream).raw).data)
            .offset(((*stream).offset).wrapping_add(8 as size_t) as isize) = version
            as uint8_t;
        *((*(*stream).raw).data)
            .offset(((*stream).offset).wrapping_add(9 as size_t) as isize) = (flags
            >> 16 as core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        *((*(*stream).raw).data)
            .offset(((*stream).offset).wrapping_add(10 as size_t) as isize) = (flags
            >> 8 as core::ffi::c_int & 0xff as uint32_t) as uint8_t;
        *((*(*stream).raw).data)
            .offset(((*stream).offset).wrapping_add(11 as size_t) as isize) = (flags
            >> 0 as core::ffi::c_int & 0xff as uint32_t) as uint8_t;
    }
    (*stream).offset = ((*stream).offset).wrapping_add(headerSize);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteBox(
    mut stream: *mut avifRWStream,
    mut type_0: *const core::ffi::c_char,
    mut contentSize: size_t,
    mut marker: *mut avifBoxMarker,
) -> avifResult {
    return avifRWStreamWriteFullBox(
        stream,
        type_0,
        contentSize,
        -(1 as core::ffi::c_int),
        0 as uint32_t,
        marker,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamFinishBox(
    mut stream: *mut avifRWStream,
    mut marker: avifBoxMarker,
) -> avifResult {
    'c_5189: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                431 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 64],
                    [core::ffi::c_char; 64],
                >(*b"avifResult avifRWStreamFinishBox(avifRWStream *, avifBoxMarker)\0"))
                    .as_ptr(),
            );
        }
    };
    let mut boxSize: size_t = ((*stream).offset).wrapping_sub(marker as size_t);
    'c_5146: {
        if boxSize
            >= (::core::mem::size_of::<uint32_t>() as usize).wrapping_add(4 as usize)
        {} else {
            __assert_fail(
                b"boxSize >= sizeof(uint32_t) + 4\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                437 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 64],
                    [core::ffi::c_char; 64],
                >(*b"avifResult avifRWStreamFinishBox(avifRWStream *, avifBoxMarker)\0"))
                    .as_ptr(),
            );
        }
    };
    if !(boxSize <= 4294967295 as size_t) {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    let mut noSize: uint32_t = avifHTONL(boxSize as uint32_t);
    memcpy(
        ((*(*stream).raw).data).offset(marker as isize) as *mut core::ffi::c_void,
        &mut noSize as *mut uint32_t as *const core::ffi::c_void,
        ::core::mem::size_of::<uint32_t>() as size_t,
    );
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteU8(
    mut stream: *mut avifRWStream,
    mut v: uint8_t,
) -> avifResult {
    'c_5288: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                446 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 56],
                    [core::ffi::c_char; 56],
                >(*b"avifResult avifRWStreamWriteU8(avifRWStream *, uint8_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let result__: avifResult = makeRoom(stream, 1 as size_t) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    *((*(*stream).raw).data).offset((*stream).offset as isize) = v;
    (*stream).offset = ((*stream).offset).wrapping_add(1 as size_t);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteU16(
    mut stream: *mut avifRWStream,
    mut v: uint16_t,
) -> avifResult {
    'c_5405: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                455 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 58],
                    [core::ffi::c_char; 58],
                >(*b"avifResult avifRWStreamWriteU16(avifRWStream *, uint16_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let size: size_t = ::core::mem::size_of::<uint16_t>() as size_t;
    let result__: avifResult = makeRoom(stream, size) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    v = avifHTONS(v);
    memcpy(
        ((*(*stream).raw).data).offset((*stream).offset as isize)
            as *mut core::ffi::c_void,
        &mut v as *mut uint16_t as *const core::ffi::c_void,
        size,
    );
    (*stream).offset = ((*stream).offset).wrapping_add(size);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteU32(
    mut stream: *mut avifRWStream,
    mut v: uint32_t,
) -> avifResult {
    'c_5522: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                466 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 58],
                    [core::ffi::c_char; 58],
                >(*b"avifResult avifRWStreamWriteU32(avifRWStream *, uint32_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let size: size_t = ::core::mem::size_of::<uint32_t>() as size_t;
    let result__: avifResult = makeRoom(stream, size) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    v = avifHTONL(v);
    memcpy(
        ((*(*stream).raw).data).offset((*stream).offset as isize)
            as *mut core::ffi::c_void,
        &mut v as *mut uint32_t as *const core::ffi::c_void,
        size,
    );
    (*stream).offset = ((*stream).offset).wrapping_add(size);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteU64(
    mut stream: *mut avifRWStream,
    mut v: uint64_t,
) -> avifResult {
    'c_5638: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                477 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 58],
                    [core::ffi::c_char; 58],
                >(*b"avifResult avifRWStreamWriteU64(avifRWStream *, uint64_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let size: size_t = ::core::mem::size_of::<uint64_t>() as size_t;
    let result__: avifResult = makeRoom(stream, size) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    v = avifHTON64(v);
    memcpy(
        ((*(*stream).raw).data).offset((*stream).offset as isize)
            as *mut core::ffi::c_void,
        &mut v as *mut uint64_t as *const core::ffi::c_void,
        size,
    );
    (*stream).offset = ((*stream).offset).wrapping_add(size);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteZeros(
    mut stream: *mut avifRWStream,
    mut byteCount: size_t,
) -> avifResult {
    'c_5740: {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {} else {
            __assert_fail(
                b"stream->numUsedBitsInPartialByte == 0\0" as *const u8
                    as *const core::ffi::c_char,
                b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                    as *const u8 as *const core::ffi::c_char,
                488 as core::ffi::c_uint,
                (::core::mem::transmute::<
                    [u8; 58],
                    [core::ffi::c_char; 58],
                >(*b"avifResult avifRWStreamWriteZeros(avifRWStream *, size_t)\0"))
                    .as_ptr(),
            );
        }
    };
    let result__: avifResult = makeRoom(stream, byteCount) as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    memset(
        ((*(*stream).raw).data).offset((*stream).offset as isize)
            as *mut core::ffi::c_void,
        0 as core::ffi::c_int,
        byteCount,
    );
    (*stream).offset = ((*stream).offset).wrapping_add(byteCount);
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifRWStreamWriteBits(
    mut stream: *mut avifRWStream,
    mut v: uint32_t,
    mut bitCount: size_t,
) -> avifResult {
    if !(bitCount >= 32 as size_t || v >> bitCount == 0 as uint32_t) {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    while bitCount != 0 {
        if (*stream).numUsedBitsInPartialByte == 0 as size_t {
            let result__: avifResult = makeRoom(stream, 1 as size_t) as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
            *((*(*stream).raw).data).offset((*stream).offset as isize) = 0 as uint8_t;
            (*stream).offset = ((*stream).offset).wrapping_add(1 as size_t);
        }
        'c_5902: {
            if (*stream).offset > 0 as size_t {} else {
                __assert_fail(
                    b"stream->offset > 0\0" as *const u8 as *const core::ffi::c_char,
                    b"/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/stream.c\0"
                        as *const u8 as *const core::ffi::c_char,
                    504 as core::ffi::c_uint,
                    (::core::mem::transmute::<
                        [u8; 67],
                        [core::ffi::c_char; 67],
                    >(
                        *b"avifResult avifRWStreamWriteBits(avifRWStream *, uint32_t, size_t)\0",
                    ))
                        .as_ptr(),
                );
            }
        };
        let mut packedBits: *mut uint8_t = ((*(*stream).raw).data)
            .offset((*stream).offset as isize)
            .offset(-(1 as core::ffi::c_int as isize));
        let numBits: size_t = if bitCount
            < (8 as size_t).wrapping_sub((*stream).numUsedBitsInPartialByte)
        {
            bitCount
        } else {
            (8 as size_t).wrapping_sub((*stream).numUsedBitsInPartialByte)
        };
        (*stream).numUsedBitsInPartialByte = ((*stream).numUsedBitsInPartialByte)
            .wrapping_add(numBits);
        bitCount = bitCount.wrapping_sub(numBits);
        let bits: uint32_t = v >> bitCount
            & (((1 as core::ffi::c_int) << numBits) - 1 as core::ffi::c_int) as uint32_t;
        *packedBits = (*packedBits as uint32_t
            | bits << (8 as size_t).wrapping_sub((*stream).numUsedBitsInPartialByte))
            as uint8_t;
        if (*stream).numUsedBitsInPartialByte == 8 as size_t {
            (*stream).numUsedBitsInPartialByte = 0 as size_t;
        }
    }
    return AVIF_RESULT_OK;
}
