// Copyright 2022 Google LLC
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/exif.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifRWDataSet(
        raw: *mut avifRWData,
        data: *const uint8_t,
        len: size_t,
    ) -> avifResult;
    fn avifROStreamStart(
        stream: *mut avifROStream,
        raw: *mut avifROData,
        diag: *mut avifDiagnostics,
        diagContext: *const core::ffi::c_char,
    );
    fn avifROStreamOffset(stream: *const avifROStream) -> size_t;
    fn avifROStreamSetOffset(stream: *mut avifROStream, offset: size_t);
    fn avifROStreamSkip(stream: *mut avifROStream, byteCount: size_t) -> avifBool;
    fn avifROStreamReadU16Endianness(
        stream: *mut avifROStream,
        v: *mut uint16_t,
        littleEndian: avifBool,
    ) -> avifBool;
    fn avifROStreamReadU32Endianness(
        stream: *mut avifROStream,
        v: *mut uint32_t,
        littleEndian: avifBool,
    ) -> avifBool;
    fn memcmp(
        __s1: *const core::ffi::c_void,
        __s2: *const core::ffi::c_void,
        __n: size_t,
    ) -> core::ffi::c_int;
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type int32_t = __int32_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
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
pub struct avifROStream {
    pub raw: *mut avifROData,
    pub offset: size_t,
    pub numUsedBitsInPartialByte: size_t,
    pub diag: *mut avifDiagnostics,
    pub diagContext: *const core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDiagnostics {
    pub error: [core::ffi::c_char; 256],
}
pub type avifPixelFormat = core::ffi::c_uint;
pub const AVIF_PIXEL_FORMAT_COUNT: avifPixelFormat = 5;
pub const AVIF_PIXEL_FORMAT_YUV400: avifPixelFormat = 4;
pub const AVIF_PIXEL_FORMAT_YUV420: avifPixelFormat = 3;
pub const AVIF_PIXEL_FORMAT_YUV422: avifPixelFormat = 2;
pub const AVIF_PIXEL_FORMAT_YUV444: avifPixelFormat = 1;
pub const AVIF_PIXEL_FORMAT_NONE: avifPixelFormat = 0;
pub type avifChromaSamplePosition = core::ffi::c_uint;
pub const AVIF_CHROMA_SAMPLE_POSITION_RESERVED: avifChromaSamplePosition = 3;
pub const AVIF_CHROMA_SAMPLE_POSITION_COLOCATED: avifChromaSamplePosition = 2;
pub const AVIF_CHROMA_SAMPLE_POSITION_VERTICAL: avifChromaSamplePosition = 1;
pub const AVIF_CHROMA_SAMPLE_POSITION_UNKNOWN: avifChromaSamplePosition = 0;
pub type avifRange = core::ffi::c_uint;
pub const AVIF_RANGE_FULL: avifRange = 1;
pub const AVIF_RANGE_LIMITED: avifRange = 0;
pub type avifColorPrimaries = uint16_t;
pub type avifTransferCharacteristics = uint16_t;
pub type avifMatrixCoefficients = uint16_t;
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
pub type avifTransformFlag = core::ffi::c_uint;
pub const AVIF_TRANSFORM_IMIR: avifTransformFlag = 8;
pub const AVIF_TRANSFORM_IROT: avifTransformFlag = 4;
pub const AVIF_TRANSFORM_CLAP: avifTransformFlag = 2;
pub const AVIF_TRANSFORM_PASP: avifTransformFlag = 1;
pub const AVIF_TRANSFORM_NONE: avifTransformFlag = 0;
pub type avifTransformFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifPixelAspectRatioBox {
    pub hSpacing: uint32_t,
    pub vSpacing: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCleanApertureBox {
    pub widthN: uint32_t,
    pub widthD: uint32_t,
    pub heightN: uint32_t,
    pub heightD: uint32_t,
    pub horizOffN: uint32_t,
    pub horizOffD: uint32_t,
    pub vertOffN: uint32_t,
    pub vertOffD: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageRotation {
    pub angle: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageMirror {
    pub axis: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifContentLightLevelInformationBox {
    pub maxCLL: uint16_t,
    pub maxPALL: uint16_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImage {
    pub width: uint32_t,
    pub height: uint32_t,
    pub depth: uint32_t,
    pub yuvFormat: avifPixelFormat,
    pub yuvRange: avifRange,
    pub yuvChromaSamplePosition: avifChromaSamplePosition,
    pub yuvPlanes: [*mut uint8_t; 3],
    pub yuvRowBytes: [uint32_t; 3],
    pub imageOwnsYUVPlanes: avifBool,
    pub alphaPlane: *mut uint8_t,
    pub alphaRowBytes: uint32_t,
    pub imageOwnsAlphaPlane: avifBool,
    pub alphaPremultiplied: avifBool,
    pub icc: avifRWData,
    pub colorPrimaries: avifColorPrimaries,
    pub transferCharacteristics: avifTransferCharacteristics,
    pub matrixCoefficients: avifMatrixCoefficients,
    pub clli: avifContentLightLevelInformationBox,
    pub transformFlags: avifTransformFlags,
    pub pasp: avifPixelAspectRatioBox,
    pub clap: avifCleanApertureBox,
    pub irot: avifImageRotation,
    pub imir: avifImageMirror,
    pub exif: avifRWData,
    pub xmp: avifRWData,
    pub properties: *mut avifImageItemProperty,
    pub numProperties: size_t,
    pub gainMap: *mut avifGainMap,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifGainMap {
    pub image: *mut avifImage,
    pub gainMapMin: [avifSignedFraction; 3],
    pub gainMapMax: [avifSignedFraction; 3],
    pub gainMapGamma: [avifUnsignedFraction; 3],
    pub baseOffset: [avifSignedFraction; 3],
    pub alternateOffset: [avifSignedFraction; 3],
    pub baseHdrHeadroom: avifUnsignedFraction,
    pub alternateHdrHeadroom: avifUnsignedFraction,
    pub useBaseColorSpace: avifBool,
    pub altICC: avifRWData,
    pub altColorPrimaries: avifColorPrimaries,
    pub altTransferCharacteristics: avifTransferCharacteristics,
    pub altMatrixCoefficients: avifMatrixCoefficients,
    pub altYUVRange: avifRange,
    pub altDepth: uint32_t,
    pub altPlaneCount: uint32_t,
    pub altCLLI: avifContentLightLevelInformationBox,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifImageItemProperty {
    pub boxtype: [uint8_t; 4],
    pub usertype: [uint8_t; 16],
    pub boxPayload: avifRWData,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
#[no_mangle]
pub unsafe extern "C" fn avifGetExifTiffHeaderOffset(
    mut exif: *const uint8_t,
    mut exifSize: size_t,
    mut offset: *mut size_t,
) -> avifResult {
    let tiffHeaderBE: [uint8_t; 4] = [
        'M' as i32 as uint8_t,
        'M' as i32 as uint8_t,
        0 as core::ffi::c_int as uint8_t,
        42 as core::ffi::c_int as uint8_t,
    ];
    let tiffHeaderLE: [uint8_t; 4] = [
        'I' as i32 as uint8_t,
        'I' as i32 as uint8_t,
        42 as core::ffi::c_int as uint8_t,
        0 as core::ffi::c_int as uint8_t,
    ];
    exifSize = if exifSize < 4294967295 as size_t {
        exifSize
    } else {
        4294967295 as size_t
    };
    *offset = 0 as size_t;
    while (*offset).wrapping_add(4 as size_t) <= exifSize {
        if memcmp(
            &*exif.offset(*offset as isize) as *const uint8_t
                as *const core::ffi::c_void,
            tiffHeaderBE.as_ptr() as *const core::ffi::c_void,
            4 as size_t,
        ) == 0
            || memcmp(
                &*exif.offset(*offset as isize) as *const uint8_t
                    as *const core::ffi::c_void,
                tiffHeaderLE.as_ptr() as *const core::ffi::c_void,
                4 as size_t,
            ) == 0
        {
            return AVIF_RESULT_OK;
        }
        *offset = (*offset).wrapping_add(1);
    }
    return AVIF_RESULT_INVALID_EXIF_PAYLOAD;
}
#[no_mangle]
pub unsafe extern "C" fn avifGetExifOrientationOffset(
    mut exif: *const uint8_t,
    mut exifSize: size_t,
    mut offset: *mut size_t,
) -> avifResult {
    let result: avifResult = avifGetExifTiffHeaderOffset(exif, exifSize, offset)
        as avifResult;
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result;
    }
    let mut raw: avifROData = {
        let mut init = avifROData {
            data: exif.offset(*offset as isize),
            size: exifSize.wrapping_sub(*offset),
        };
        init
    };
    let littleEndian: avifBool = (*(raw.data).offset(0 as core::ffi::c_int as isize)
        as core::ffi::c_int == 'I' as i32) as core::ffi::c_int;
    let mut stream: avifROStream = avifROStream {
        raw: 0 as *mut avifROData,
        offset: 0,
        numUsedBitsInPartialByte: 0,
        diag: 0 as *mut avifDiagnostics,
        diagContext: 0 as *const core::ffi::c_char,
    };
    avifROStreamStart(
        &mut stream,
        &mut raw,
        0 as *mut avifDiagnostics,
        0 as *const core::ffi::c_char,
    );
    let mut offsetTo0thIfd: uint32_t = 0;
    if avifROStreamSkip(&mut stream, 4 as size_t) == 0
        || avifROStreamReadU32Endianness(&mut stream, &mut offsetTo0thIfd, littleEndian)
            == 0
    {
        return AVIF_RESULT_INVALID_EXIF_PAYLOAD;
    }
    avifROStreamSetOffset(&mut stream, offsetTo0thIfd as size_t);
    let mut fieldCount: uint16_t = 0;
    if avifROStreamReadU16Endianness(&mut stream, &mut fieldCount, littleEndian) == 0 {
        return AVIF_RESULT_INVALID_EXIF_PAYLOAD;
    }
    let mut field: uint16_t = 0 as uint16_t;
    while (field as core::ffi::c_int) < fieldCount as core::ffi::c_int {
        let mut tag: uint16_t = 0;
        let mut type_0: uint16_t = 0;
        let mut count: uint32_t = 0;
        let mut firstHalfOfValueOffset: uint16_t = 0;
        if avifROStreamReadU16Endianness(&mut stream, &mut tag, littleEndian) == 0
            || avifROStreamReadU16Endianness(&mut stream, &mut type_0, littleEndian) == 0
            || avifROStreamReadU32Endianness(&mut stream, &mut count, littleEndian) == 0
            || avifROStreamReadU16Endianness(
                &mut stream,
                &mut firstHalfOfValueOffset,
                littleEndian,
            ) == 0 || avifROStreamSkip(&mut stream, 2 as size_t) == 0
        {
            return AVIF_RESULT_INVALID_EXIF_PAYLOAD;
        }
        let shortType: uint16_t = 0x3 as uint16_t;
        if tag as core::ffi::c_int == 0x112 as core::ffi::c_int
            && type_0 as core::ffi::c_int == shortType as core::ffi::c_int
            && count == 0x1 as uint32_t
        {
            if firstHalfOfValueOffset as core::ffi::c_int >= 1 as core::ffi::c_int
                && firstHalfOfValueOffset as core::ffi::c_int <= 8 as core::ffi::c_int
            {
                *offset = (*offset)
                    .wrapping_add(
                        (avifROStreamOffset(&mut stream))
                            .wrapping_sub(
                                (if littleEndian != 0 {
                                    4 as core::ffi::c_int
                                } else {
                                    3 as core::ffi::c_int
                                }) as size_t,
                            ),
                    );
                return AVIF_RESULT_OK;
            }
        }
        field = field.wrapping_add(1);
    }
    *offset = exifSize;
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageExtractExifOrientationToIrotImir(
    mut image: *mut avifImage,
) -> avifResult {
    let otherFlags: avifTransformFlags = (*image).transformFlags
        & !(AVIF_TRANSFORM_IROT as core::ffi::c_int
            | AVIF_TRANSFORM_IMIR as core::ffi::c_int) as avifTransformFlags;
    let mut offset: size_t = 0;
    let result: avifResult = avifGetExifOrientationOffset(
        (*image).exif.data,
        (*image).exif.size,
        &mut offset,
    ) as avifResult;
    if result as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result;
    }
    if offset < (*image).exif.size {
        let orientation: uint8_t = *((*image).exif.data).offset(offset as isize);
        match orientation as core::ffi::c_int {
            1 => {
                (*image).transformFlags = otherFlags;
                (*image).irot.angle = 0 as uint8_t;
                (*image).imir.axis = 0 as uint8_t;
                return AVIF_RESULT_OK;
            }
            2 => {
                (*image).transformFlags = otherFlags
                    | AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags;
                (*image).irot.angle = 0 as uint8_t;
                (*image).imir.axis = 1 as uint8_t;
                return AVIF_RESULT_OK;
            }
            3 => {
                (*image).transformFlags = otherFlags
                    | AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags;
                (*image).irot.angle = 2 as uint8_t;
                (*image).imir.axis = 0 as uint8_t;
                return AVIF_RESULT_OK;
            }
            4 => {
                (*image).transformFlags = otherFlags
                    | AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags;
                (*image).irot.angle = 0 as uint8_t;
                (*image).imir.axis = 0 as uint8_t;
                return AVIF_RESULT_OK;
            }
            5 => {
                (*image).transformFlags = otherFlags
                    | AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags
                    | AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags;
                (*image).irot.angle = 1 as uint8_t;
                (*image).imir.axis = 0 as uint8_t;
                return AVIF_RESULT_OK;
            }
            6 => {
                (*image).transformFlags = otherFlags
                    | AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags;
                (*image).irot.angle = 3 as uint8_t;
                (*image).imir.axis = 0 as uint8_t;
                return AVIF_RESULT_OK;
            }
            7 => {
                (*image).transformFlags = otherFlags
                    | AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags
                    | AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags;
                (*image).irot.angle = 3 as uint8_t;
                (*image).imir.axis = 0 as uint8_t;
                return AVIF_RESULT_OK;
            }
            8 => {
                (*image).transformFlags = otherFlags
                    | AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags;
                (*image).irot.angle = 1 as uint8_t;
                (*image).imir.axis = 0 as uint8_t;
                return AVIF_RESULT_OK;
            }
            _ => {}
        }
    }
    (*image).transformFlags = otherFlags;
    (*image).irot.angle = 0 as uint8_t;
    (*image).imir.axis = 0 as uint8_t;
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageIrotImirToExifOrientation(
    mut image: *const avifImage,
) -> uint8_t {
    if (*image).transformFlags
        & AVIF_TRANSFORM_IROT as core::ffi::c_int as avifTransformFlags == 0
        || (*image).irot.angle as core::ffi::c_int == 0 as core::ffi::c_int
    {
        if (*image).transformFlags
            & AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags == 0
        {
            return 1 as uint8_t;
        }
        if (*image).imir.axis as core::ffi::c_int == 0 as core::ffi::c_int {
            return 4 as uint8_t;
        }
        return 2 as uint8_t;
    }
    if (*image).irot.angle as core::ffi::c_int == 1 as core::ffi::c_int {
        if (*image).transformFlags
            & AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags == 0
        {
            return 8 as uint8_t;
        }
        if (*image).imir.axis as core::ffi::c_int == 0 as core::ffi::c_int {
            return 5 as uint8_t;
        }
        return 7 as uint8_t;
    }
    if (*image).irot.angle as core::ffi::c_int == 2 as core::ffi::c_int {
        if (*image).transformFlags
            & AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags == 0
        {
            return 3 as uint8_t;
        }
        if (*image).imir.axis as core::ffi::c_int == 0 as core::ffi::c_int {
            return 2 as uint8_t;
        }
        return 4 as uint8_t;
    }
    if (*image).transformFlags
        & AVIF_TRANSFORM_IMIR as core::ffi::c_int as avifTransformFlags == 0
    {
        return 6 as uint8_t;
    }
    if (*image).imir.axis as core::ffi::c_int == 0 as core::ffi::c_int {
        return 7 as uint8_t;
    }
    return 5 as uint8_t;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageSetMetadataExif(
    mut image: *mut avifImage,
    mut exif: *const uint8_t,
    mut exifSize: size_t,
) -> avifResult {
    let result__: avifResult = avifRWDataSet(&mut (*image).exif, exif, exifSize)
        as avifResult;
    if result__ as core::ffi::c_uint
        != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
    {
        return result__;
    }
    avifImageExtractExifOrientationToIrotImir(image);
    return AVIF_RESULT_OK;
}
