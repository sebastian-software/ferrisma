// Copyright 2020 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/io.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifRWDataRealloc(raw: *mut avifRWData, newSize: size_t) -> avifResult;
    fn avifRWDataFree(raw: *mut avifRWData);
    fn avifCalloc(count: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn fclose(__stream: *mut FILE) -> core::ffi::c_int;
    fn fopen(
        __filename: *const core::ffi::c_char,
        __modes: *const core::ffi::c_char,
    ) -> *mut FILE;
    fn fread(
        __ptr: *mut core::ffi::c_void,
        __size: size_t,
        __n: size_t,
        __stream: *mut FILE,
    ) -> core::ffi::c_ulong;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: core::ffi::c_int,
    ) -> core::ffi::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn ferror(__stream: *mut FILE) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint32_t = u32;
pub type __uint64_t = u64;
pub type __off_t = core::ffi::c_long;
pub type __off64_t = core::ffi::c_long;
pub type uint8_t = __uint8_t;
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
pub struct avifIO {
    pub destroy: avifIODestroyFunc,
    pub read: avifIOReadFunc,
    pub write: avifIOWriteFunc,
    pub sizeHint: uint64_t,
    pub persistent: avifBool,
    pub data: *mut core::ffi::c_void,
}
pub type avifIOWriteFunc = Option<
    unsafe extern "C" fn(
        *mut avifIO,
        uint32_t,
        uint64_t,
        *const uint8_t,
        size_t,
    ) -> avifResult,
>;
pub type avifIOReadFunc = Option<
    unsafe extern "C" fn(
        *mut avifIO,
        uint32_t,
        uint64_t,
        size_t,
        *mut avifROData,
    ) -> avifResult,
>;
pub type avifIODestroyFunc = Option<unsafe extern "C" fn(*mut avifIO) -> ()>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifIOMemoryReader {
    pub io: avifIO,
    pub rodata: avifROData,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifIOFileReader {
    pub io: avifIO,
    pub buffer: avifRWData,
    pub f: *mut FILE,
}
pub type FILE = _IO_FILE;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: core::ffi::c_int,
    pub _IO_read_ptr: *mut core::ffi::c_char,
    pub _IO_read_end: *mut core::ffi::c_char,
    pub _IO_read_base: *mut core::ffi::c_char,
    pub _IO_write_base: *mut core::ffi::c_char,
    pub _IO_write_ptr: *mut core::ffi::c_char,
    pub _IO_write_end: *mut core::ffi::c_char,
    pub _IO_buf_base: *mut core::ffi::c_char,
    pub _IO_buf_end: *mut core::ffi::c_char,
    pub _IO_save_base: *mut core::ffi::c_char,
    pub _IO_backup_base: *mut core::ffi::c_char,
    pub _IO_save_end: *mut core::ffi::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: core::ffi::c_int,
    pub _flags2: core::ffi::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: core::ffi::c_ushort,
    pub _vtable_offset: core::ffi::c_schar,
    pub _shortbuf: [core::ffi::c_char; 1],
    pub _lock: *mut core::ffi::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut core::ffi::c_void,
    pub __pad5: size_t,
    pub _mode: core::ffi::c_int,
    pub _unused2: [core::ffi::c_char; 20],
}
pub type _IO_lock_t = ();
pub type avif_off_t = off_t;
pub type off_t = __off64_t;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT64_MAX: core::ffi::c_long = 9223372036854775807 as core::ffi::c_long;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SEEK_SET: core::ffi::c_int = 0 as core::ffi::c_int;
pub const SEEK_END: core::ffi::c_int = 2 as core::ffi::c_int;
pub const AVIF_OFF_MAX: core::ffi::c_long = INT64_MAX;
unsafe extern "C" fn avif_fseeko(
    mut stream: *mut FILE,
    mut offset: avif_off_t,
    mut whence: core::ffi::c_int,
) -> core::ffi::c_int {
    return fseeko(stream, offset as __off64_t, whence);
}
unsafe extern "C" fn avif_ftello(mut stream: *mut FILE) -> avif_off_t {
    return ftello(stream) as avif_off_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifIODestroy(mut io: *mut avifIO) {
    if !io.is_null() && ((*io).destroy).is_some() {
        ((*io).destroy).expect("non-null function pointer")(io as *mut avifIO);
    }
}
unsafe extern "C" fn avifIOMemoryReaderRead(
    mut io: *mut avifIO,
    mut readFlags: uint32_t,
    mut offset: uint64_t,
    mut size: size_t,
    mut out: *mut avifROData,
) -> avifResult {
    if readFlags != 0 as uint32_t {
        return AVIF_RESULT_IO_ERROR;
    }
    let mut reader: *mut avifIOMemoryReader = io as *mut avifIOMemoryReader;
    if offset > (*reader).rodata.size as uint64_t {
        return AVIF_RESULT_IO_ERROR;
    }
    let mut availableSize: uint64_t = ((*reader).rodata.size as uint64_t)
        .wrapping_sub(offset);
    if size > availableSize as size_t {
        size = availableSize as size_t;
    }
    (*out).data = if offset != 0 {
        ((*reader).rodata.data).offset(offset as isize)
    } else {
        (*reader).rodata.data
    };
    (*out).size = size;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifIOMemoryReaderDestroy(mut io: *mut avifIO) {
    avifFree(io as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifIOCreateMemoryReader(
    mut data: *const uint8_t,
    mut size: size_t,
) -> *mut avifIO {
    let mut reader: *mut avifIOMemoryReader = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifIOMemoryReader>() as size_t,
    ) as *mut avifIOMemoryReader;
    if reader.is_null() {
        return 0 as *mut avifIO;
    }
    (*reader).io.destroy = Some(
        avifIOMemoryReaderDestroy as unsafe extern "C" fn(*mut avifIO) -> (),
    ) as avifIODestroyFunc;
    (*reader).io.read = Some(
        avifIOMemoryReaderRead
            as unsafe extern "C" fn(
                *mut avifIO,
                uint32_t,
                uint64_t,
                size_t,
                *mut avifROData,
            ) -> avifResult,
    ) as avifIOReadFunc;
    (*reader).io.sizeHint = size as uint64_t;
    (*reader).io.persistent = AVIF_TRUE as avifBool;
    (*reader).rodata.data = data;
    (*reader).rodata.size = size;
    return reader as *mut avifIO;
}
unsafe extern "C" fn avifIOFileReaderRead(
    mut io: *mut avifIO,
    mut readFlags: uint32_t,
    mut offset: uint64_t,
    mut size: size_t,
    mut out: *mut avifROData,
) -> avifResult {
    if readFlags != 0 as uint32_t {
        return AVIF_RESULT_IO_ERROR;
    }
    let mut reader: *mut avifIOFileReader = io as *mut avifIOFileReader;
    if offset > (*reader).io.sizeHint {
        return AVIF_RESULT_IO_ERROR;
    }
    let mut availableSize: uint64_t = ((*reader).io.sizeHint).wrapping_sub(offset);
    if size > availableSize as size_t {
        size = availableSize as size_t;
    }
    if size > 0 as size_t {
        if offset > AVIF_OFF_MAX as uint64_t {
            return AVIF_RESULT_IO_ERROR;
        }
        if (*reader).buffer.size < size {
            let result__: avifResult = avifRWDataRealloc(&mut (*reader).buffer, size)
                as avifResult;
            if result__ as core::ffi::c_uint
                != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
            {
                return result__;
            }
        }
        if avif_fseeko((*reader).f, offset as avif_off_t, SEEK_SET)
            != 0 as core::ffi::c_int
        {
            return AVIF_RESULT_IO_ERROR;
        }
        let mut bytesRead: size_t = fread(
            (*reader).buffer.data as *mut core::ffi::c_void,
            1 as size_t,
            size,
            (*reader).f,
        ) as size_t;
        if size != bytesRead {
            if ferror((*reader).f) != 0 {
                return AVIF_RESULT_IO_ERROR;
            }
            size = bytesRead;
        }
    }
    (*out).data = (*reader).buffer.data;
    (*out).size = size;
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifIOFileReaderDestroy(mut io: *mut avifIO) {
    let mut reader: *mut avifIOFileReader = io as *mut avifIOFileReader;
    fclose((*reader).f);
    avifRWDataFree(&mut (*reader).buffer);
    avifFree(io as *mut core::ffi::c_void);
}
#[no_mangle]
pub unsafe extern "C" fn avifIOCreateFileReader(
    mut filename: *const core::ffi::c_char,
) -> *mut avifIO {
    let mut f: *mut FILE = fopen(
        filename,
        b"rb\0" as *const u8 as *const core::ffi::c_char,
    ) as *mut FILE;
    if f.is_null() {
        return 0 as *mut avifIO;
    }
    if avif_fseeko(f, 0 as avif_off_t, SEEK_END) != 0 as core::ffi::c_int {
        fclose(f);
        return 0 as *mut avifIO;
    }
    let mut fileSize: avif_off_t = avif_ftello(f);
    if fileSize < 0 as avif_off_t {
        fclose(f);
        return 0 as *mut avifIO;
    }
    if avif_fseeko(f, 0 as avif_off_t, SEEK_SET) != 0 as core::ffi::c_int {
        fclose(f);
        return 0 as *mut avifIO;
    }
    let mut reader: *mut avifIOFileReader = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifIOFileReader>() as size_t,
    ) as *mut avifIOFileReader;
    if reader.is_null() {
        fclose(f);
        return 0 as *mut avifIO;
    }
    (*reader).f = f;
    (*reader).io.destroy = Some(
        avifIOFileReaderDestroy as unsafe extern "C" fn(*mut avifIO) -> (),
    ) as avifIODestroyFunc;
    (*reader).io.read = Some(
        avifIOFileReaderRead
            as unsafe extern "C" fn(
                *mut avifIO,
                uint32_t,
                uint64_t,
                size_t,
                *mut avifROData,
            ) -> avifResult,
    ) as avifIOReadFunc;
    (*reader).io.sizeHint = fileSize as uint64_t;
    (*reader).io.persistent = AVIF_FALSE as avifBool;
    if avifRWDataRealloc(&mut (*reader).buffer, 1024 as size_t) as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        avifFree(reader as *mut core::ffi::c_void);
        fclose(f);
        return 0 as *mut avifIO;
    }
    return reader as *mut avifIO;
}
