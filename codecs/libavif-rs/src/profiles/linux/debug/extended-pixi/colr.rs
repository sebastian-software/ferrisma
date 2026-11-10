// Copyright 2019 Joe Drago. All rights reserved.
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/colr.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn expf(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn logf(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn log10f(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn powf(__x: core::ffi::c_float, __y: core::ffi::c_float) -> core::ffi::c_float;
    fn sqrtf(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn fabsf(__x: core::ffi::c_float) -> core::ffi::c_float;
    fn memcpy(
        __dest: *mut core::ffi::c_void,
        __src: *const core::ffi::c_void,
        __n: size_t,
    ) -> *mut core::ffi::c_void;
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
pub struct avifRWData {
    pub data: *mut uint8_t,
    pub size: size_t,
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
pub type C2RustUnnamed = core::ffi::c_uint;
pub const AVIF_COLOR_PRIMARIES_EBU3213: C2RustUnnamed = 22;
pub const AVIF_COLOR_PRIMARIES_DCI_P3: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE432: C2RustUnnamed = 12;
pub const AVIF_COLOR_PRIMARIES_SMPTE431: C2RustUnnamed = 11;
pub const AVIF_COLOR_PRIMARIES_XYZ: C2RustUnnamed = 10;
pub const AVIF_COLOR_PRIMARIES_BT2100: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_BT2020: C2RustUnnamed = 9;
pub const AVIF_COLOR_PRIMARIES_GENERIC_FILM: C2RustUnnamed = 8;
pub const AVIF_COLOR_PRIMARIES_SMPTE240: C2RustUnnamed = 7;
pub const AVIF_COLOR_PRIMARIES_BT601: C2RustUnnamed = 6;
pub const AVIF_COLOR_PRIMARIES_BT470BG: C2RustUnnamed = 5;
pub const AVIF_COLOR_PRIMARIES_BT470M: C2RustUnnamed = 4;
pub const AVIF_COLOR_PRIMARIES_UNSPECIFIED: C2RustUnnamed = 2;
pub const AVIF_COLOR_PRIMARIES_IEC61966_2_4: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_SRGB: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_BT709: C2RustUnnamed = 1;
pub const AVIF_COLOR_PRIMARIES_UNKNOWN: C2RustUnnamed = 0;
pub type avifColorPrimaries = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifColorPrimariesTable {
    pub colorPrimariesEnum: avifColorPrimaries,
    pub name: *const core::ffi::c_char,
    pub primaries: [core::ffi::c_float; 8],
}
pub type C2RustUnnamed_0 = core::ffi::c_uint;
pub const AVIF_TRANSFER_CHARACTERISTICS_HLG: C2RustUnnamed_0 = 18;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE428: C2RustUnnamed_0 = 17;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084: C2RustUnnamed_0 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_PQ: C2RustUnnamed_0 = 16;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT: C2RustUnnamed_0 = 15;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT: C2RustUnnamed_0 = 14;
pub const AVIF_TRANSFER_CHARACTERISTICS_SRGB: C2RustUnnamed_0 = 13;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT1361: C2RustUnnamed_0 = 12;
pub const AVIF_TRANSFER_CHARACTERISTICS_IEC61966: C2RustUnnamed_0 = 11;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10: C2RustUnnamed_0 = 10;
pub const AVIF_TRANSFER_CHARACTERISTICS_LOG100: C2RustUnnamed_0 = 9;
pub const AVIF_TRANSFER_CHARACTERISTICS_LINEAR: C2RustUnnamed_0 = 8;
pub const AVIF_TRANSFER_CHARACTERISTICS_SMPTE240: C2RustUnnamed_0 = 7;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT601: C2RustUnnamed_0 = 6;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470BG: C2RustUnnamed_0 = 5;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT470M: C2RustUnnamed_0 = 4;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNSPECIFIED: C2RustUnnamed_0 = 2;
pub const AVIF_TRANSFER_CHARACTERISTICS_BT709: C2RustUnnamed_0 = 1;
pub const AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN: C2RustUnnamed_0 = 0;
pub type avifTransferCharacteristics = uint16_t;
pub type C2RustUnnamed_1 = core::ffi::c_uint;
pub const AVIF_MATRIX_COEFFICIENTS_LAST: C2RustUnnamed_1 = 18;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RO: C2RustUnnamed_1 = 17;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO_RE: C2RustUnnamed_1 = 16;
pub const AVIF_MATRIX_COEFFICIENTS_ICTCP: C2RustUnnamed_1 = 14;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_CL: C2RustUnnamed_1 = 13;
pub const AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL: C2RustUnnamed_1 = 12;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE2085: C2RustUnnamed_1 = 11;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_CL: C2RustUnnamed_1 = 10;
pub const AVIF_MATRIX_COEFFICIENTS_BT2020_NCL: C2RustUnnamed_1 = 9;
pub const AVIF_MATRIX_COEFFICIENTS_YCGCO: C2RustUnnamed_1 = 8;
pub const AVIF_MATRIX_COEFFICIENTS_SMPTE240: C2RustUnnamed_1 = 7;
pub const AVIF_MATRIX_COEFFICIENTS_BT601: C2RustUnnamed_1 = 6;
pub const AVIF_MATRIX_COEFFICIENTS_BT470BG: C2RustUnnamed_1 = 5;
pub const AVIF_MATRIX_COEFFICIENTS_FCC: C2RustUnnamed_1 = 4;
pub const AVIF_MATRIX_COEFFICIENTS_UNSPECIFIED: C2RustUnnamed_1 = 2;
pub const AVIF_MATRIX_COEFFICIENTS_BT709: C2RustUnnamed_1 = 1;
pub const AVIF_MATRIX_COEFFICIENTS_IDENTITY: C2RustUnnamed_1 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifMatrixCoefficientsTable {
    pub matrixCoefficientsEnum: avifMatrixCoefficients,
    pub name: *const core::ffi::c_char,
    pub kr: core::ffi::c_float,
    pub kb: core::ffi::c_float,
}
pub type avifTransferFunction = Option<
    unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifTransferCharacteristicsTable {
    pub transferCharacteristicsEnum: avifTransferCharacteristics,
    pub name: *const core::ffi::c_char,
    pub toLinear: avifTransferFunction,
    pub toGamma: avifTransferFunction,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
static mut avifColorPrimariesTables: [avifColorPrimariesTable; 11] = [
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_BT709 as core::ffi::c_int
                as avifColorPrimaries,
            name: b"BT.709\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.64f32,
                0.33f32,
                0.3f32,
                0.6f32,
                0.15f32,
                0.06f32,
                0.3127f32,
                0.329f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_BT470M as core::ffi::c_int
                as avifColorPrimaries,
            name: b"BT.470-6 System M\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.67f32,
                0.33f32,
                0.21f32,
                0.71f32,
                0.14f32,
                0.08f32,
                0.310f32,
                0.316f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_BT470BG as core::ffi::c_int
                as avifColorPrimaries,
            name: b"BT.470-6 System BG\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.64f32,
                0.33f32,
                0.29f32,
                0.60f32,
                0.15f32,
                0.06f32,
                0.3127f32,
                0.3290f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_BT601 as core::ffi::c_int
                as avifColorPrimaries,
            name: b"BT.601\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.630f32,
                0.340f32,
                0.310f32,
                0.595f32,
                0.155f32,
                0.070f32,
                0.3127f32,
                0.3290f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_SMPTE240 as core::ffi::c_int
                as avifColorPrimaries,
            name: b"SMPTE 240M\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.630f32,
                0.340f32,
                0.310f32,
                0.595f32,
                0.155f32,
                0.070f32,
                0.3127f32,
                0.3290f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_GENERIC_FILM as core::ffi::c_int
                as avifColorPrimaries,
            name: b"Generic film\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.681f32,
                0.319f32,
                0.243f32,
                0.692f32,
                0.145f32,
                0.049f32,
                0.310f32,
                0.316f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_BT2020 as core::ffi::c_int
                as avifColorPrimaries,
            name: b"BT.2020\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.708f32,
                0.292f32,
                0.170f32,
                0.797f32,
                0.131f32,
                0.046f32,
                0.3127f32,
                0.3290f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_XYZ as core::ffi::c_int
                as avifColorPrimaries,
            name: b"XYZ\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                1.0f32,
                0.0f32,
                0.0f32,
                1.0f32,
                0.0f32,
                0.0f32,
                0.3333f32,
                0.3333f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_SMPTE431 as core::ffi::c_int
                as avifColorPrimaries,
            name: b"SMPTE RP 431-2\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.680f32,
                0.320f32,
                0.265f32,
                0.690f32,
                0.150f32,
                0.060f32,
                0.314f32,
                0.351f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_SMPTE432 as core::ffi::c_int
                as avifColorPrimaries,
            name: b"SMPTE EG 432-1 (DCI P3)\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.680f32,
                0.320f32,
                0.265f32,
                0.690f32,
                0.150f32,
                0.060f32,
                0.3127f32,
                0.3290f32,
            ],
        };
        init
    },
    {
        let mut init = avifColorPrimariesTable {
            colorPrimariesEnum: AVIF_COLOR_PRIMARIES_EBU3213 as core::ffi::c_int
                as avifColorPrimaries,
            name: b"EBU Tech. 3213-E\0" as *const u8 as *const core::ffi::c_char,
            primaries: [
                0.630f32,
                0.340f32,
                0.295f32,
                0.605f32,
                0.155f32,
                0.077f32,
                0.3127f32,
                0.3290f32,
            ],
        };
        init
    },
];
static mut avifColorPrimariesTableSize: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn avifColorPrimariesGetValues(
    mut acp: avifColorPrimaries,
    mut outPrimaries: *mut core::ffi::c_float,
) {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < avifColorPrimariesTableSize {
        if avifColorPrimariesTables[i as usize].colorPrimariesEnum as core::ffi::c_int
            == acp as core::ffi::c_int
        {
            memcpy(
                outPrimaries as *mut core::ffi::c_void,
                (avifColorPrimariesTables[i as usize].primaries).as_ptr()
                    as *const core::ffi::c_void,
                ::core::mem::size_of::<[core::ffi::c_float; 8]>() as size_t,
            );
            return;
        }
        i += 1;
    }
    memcpy(
        outPrimaries as *mut core::ffi::c_void,
        (avifColorPrimariesTables[0 as core::ffi::c_int as usize].primaries).as_ptr()
            as *const core::ffi::c_void,
        ::core::mem::size_of::<[core::ffi::c_float; 8]>() as size_t,
    );
}
unsafe extern "C" fn matchesTo3RoundedPlaces(
    mut a: core::ffi::c_float,
    mut b: core::ffi::c_float,
) -> avifBool {
    return (fabsf(a - b) < 0.001f32) as core::ffi::c_int;
}
unsafe extern "C" fn primariesMatch(
    mut p1: *const core::ffi::c_float,
    mut p2: *const core::ffi::c_float,
) -> avifBool {
    return (matchesTo3RoundedPlaces(
        *p1.offset(0 as core::ffi::c_int as isize),
        *p2.offset(0 as core::ffi::c_int as isize),
    ) != 0
        && matchesTo3RoundedPlaces(
            *p1.offset(1 as core::ffi::c_int as isize),
            *p2.offset(1 as core::ffi::c_int as isize),
        ) != 0
        && matchesTo3RoundedPlaces(
            *p1.offset(2 as core::ffi::c_int as isize),
            *p2.offset(2 as core::ffi::c_int as isize),
        ) != 0
        && matchesTo3RoundedPlaces(
            *p1.offset(3 as core::ffi::c_int as isize),
            *p2.offset(3 as core::ffi::c_int as isize),
        ) != 0
        && matchesTo3RoundedPlaces(
            *p1.offset(4 as core::ffi::c_int as isize),
            *p2.offset(4 as core::ffi::c_int as isize),
        ) != 0
        && matchesTo3RoundedPlaces(
            *p1.offset(5 as core::ffi::c_int as isize),
            *p2.offset(5 as core::ffi::c_int as isize),
        ) != 0
        && matchesTo3RoundedPlaces(
            *p1.offset(6 as core::ffi::c_int as isize),
            *p2.offset(6 as core::ffi::c_int as isize),
        ) != 0
        && matchesTo3RoundedPlaces(
            *p1.offset(7 as core::ffi::c_int as isize),
            *p2.offset(7 as core::ffi::c_int as isize),
        ) != 0) as core::ffi::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn avifColorPrimariesFind(
    mut inPrimaries: *const core::ffi::c_float,
    mut outName: *mut *const core::ffi::c_char,
) -> avifColorPrimaries {
    if !outName.is_null() {
        *outName = 0 as *const core::ffi::c_char;
    }
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < avifColorPrimariesTableSize {
        if primariesMatch(
            inPrimaries,
            (avifColorPrimariesTables[i as usize].primaries).as_ptr(),
        ) != 0
        {
            if !outName.is_null() {
                *outName = avifColorPrimariesTables[i as usize].name;
            }
            return avifColorPrimariesTables[i as usize].colorPrimariesEnum;
        }
        i += 1;
    }
    return AVIF_COLOR_PRIMARIES_UNKNOWN as core::ffi::c_int as avifColorPrimaries;
}
#[no_mangle]
pub unsafe extern "C" fn avifTransferCharacteristicsGetGamma(
    mut atc: avifTransferCharacteristics,
    mut gamma: *mut core::ffi::c_float,
) -> avifResult {
    if gamma.is_null() {
        return AVIF_RESULT_INVALID_ARGUMENT;
    }
    match atc as core::ffi::c_int {
        4 => {
            *gamma = 2.2f32;
            return AVIF_RESULT_OK;
        }
        5 => {
            *gamma = 2.8f32;
            return AVIF_RESULT_OK;
        }
        8 => {
            *gamma = 1.0f32;
            return AVIF_RESULT_OK;
        }
        _ => return AVIF_RESULT_NOT_IMPLEMENTED,
    };
}
#[no_mangle]
pub unsafe extern "C" fn avifTransferCharacteristicsFindByGamma(
    mut gamma: core::ffi::c_float,
) -> avifTransferCharacteristics {
    if matchesTo3RoundedPlaces(gamma, 2.2f32) != 0 {
        return AVIF_TRANSFER_CHARACTERISTICS_BT470M as core::ffi::c_int
            as avifTransferCharacteristics
    } else if matchesTo3RoundedPlaces(gamma, 1.0f32) != 0 {
        return AVIF_TRANSFER_CHARACTERISTICS_LINEAR as core::ffi::c_int
            as avifTransferCharacteristics
    } else if matchesTo3RoundedPlaces(gamma, 2.8f32) != 0 {
        return AVIF_TRANSFER_CHARACTERISTICS_BT470BG as core::ffi::c_int
            as avifTransferCharacteristics
    }
    return AVIF_TRANSFER_CHARACTERISTICS_UNKNOWN as core::ffi::c_int
        as avifTransferCharacteristics;
}
static mut matrixCoefficientsTables: [avifMatrixCoefficientsTable; 6] = [
    {
        let mut init = avifMatrixCoefficientsTable {
            matrixCoefficientsEnum: AVIF_MATRIX_COEFFICIENTS_BT709 as core::ffi::c_int
                as avifMatrixCoefficients,
            name: b"BT.709\0" as *const u8 as *const core::ffi::c_char,
            kr: 0.2126f32,
            kb: 0.0722f32,
        };
        init
    },
    {
        let mut init = avifMatrixCoefficientsTable {
            matrixCoefficientsEnum: AVIF_MATRIX_COEFFICIENTS_FCC as core::ffi::c_int
                as avifMatrixCoefficients,
            name: b"FCC USFC 73.682\0" as *const u8 as *const core::ffi::c_char,
            kr: 0.30f32,
            kb: 0.11f32,
        };
        init
    },
    {
        let mut init = avifMatrixCoefficientsTable {
            matrixCoefficientsEnum: AVIF_MATRIX_COEFFICIENTS_BT470BG as core::ffi::c_int
                as avifMatrixCoefficients,
            name: b"BT.470-6 System BG\0" as *const u8 as *const core::ffi::c_char,
            kr: 0.299f32,
            kb: 0.114f32,
        };
        init
    },
    {
        let mut init = avifMatrixCoefficientsTable {
            matrixCoefficientsEnum: AVIF_MATRIX_COEFFICIENTS_BT601 as core::ffi::c_int
                as avifMatrixCoefficients,
            name: b"BT.601\0" as *const u8 as *const core::ffi::c_char,
            kr: 0.299f32,
            kb: 0.114f32,
        };
        init
    },
    {
        let mut init = avifMatrixCoefficientsTable {
            matrixCoefficientsEnum: AVIF_MATRIX_COEFFICIENTS_SMPTE240 as core::ffi::c_int
                as avifMatrixCoefficients,
            name: b"SMPTE ST 240\0" as *const u8 as *const core::ffi::c_char,
            kr: 0.212f32,
            kb: 0.087f32,
        };
        init
    },
    {
        let mut init = avifMatrixCoefficientsTable {
            matrixCoefficientsEnum: AVIF_MATRIX_COEFFICIENTS_BT2020_NCL
                as core::ffi::c_int as avifMatrixCoefficients,
            name: b"BT.2020 (non-constant luminance)\0" as *const u8
                as *const core::ffi::c_char,
            kr: 0.2627f32,
            kb: 0.0593f32,
        };
        init
    },
];
static mut avifMatrixCoefficientsTableSize: core::ffi::c_int = 0;
unsafe extern "C" fn calcYUVInfoFromCICP(
    mut image: *const avifImage,
    mut coeffs: *mut core::ffi::c_float,
) -> avifBool {
    if (*image).matrixCoefficients as core::ffi::c_int
        == AVIF_MATRIX_COEFFICIENTS_CHROMA_DERIVED_NCL as core::ffi::c_int
    {
        avifColorPrimariesComputeYCoeffs((*image).colorPrimaries, coeffs);
        return AVIF_TRUE;
    } else {
        let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
        while i < avifMatrixCoefficientsTableSize {
            let table: *const avifMatrixCoefficientsTable = &*core::ptr::addr_of!(matrixCoefficientsTables).cast::<avifMatrixCoefficientsTable>()
                .offset(i as isize) as *const avifMatrixCoefficientsTable;
            if (*table).matrixCoefficientsEnum as core::ffi::c_int
                == (*image).matrixCoefficients as core::ffi::c_int
            {
                *coeffs.offset(0 as core::ffi::c_int as isize) = (*table).kr;
                *coeffs.offset(2 as core::ffi::c_int as isize) = (*table).kb;
                *coeffs.offset(1 as core::ffi::c_int as isize) = 1.0f32
                    - *coeffs.offset(0 as core::ffi::c_int as isize)
                    - *coeffs.offset(2 as core::ffi::c_int as isize);
                return AVIF_TRUE;
            }
            i += 1;
        }
    }
    return AVIF_FALSE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCalcYUVCoefficients(
    mut image: *const avifImage,
    mut outR: *mut core::ffi::c_float,
    mut outG: *mut core::ffi::c_float,
    mut outB: *mut core::ffi::c_float,
) {
    let mut kr: core::ffi::c_float = 0.299f32;
    let mut kb: core::ffi::c_float = 0.114f32;
    let mut kg: core::ffi::c_float = 1.0f32 - kr - kb;
    let mut coeffs: [core::ffi::c_float; 3] = [0.; 3];
    if calcYUVInfoFromCICP(image, coeffs.as_mut_ptr()) != 0 {
        kr = coeffs[0 as core::ffi::c_int as usize];
        kg = coeffs[1 as core::ffi::c_int as usize];
        kb = coeffs[2 as core::ffi::c_int as usize];
    }
    *outR = kr;
    *outG = kg;
    *outB = kb;
}
unsafe extern "C" fn avifToLinear709(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    if gamma < 0.0f32 {
        return 0.0f32
    } else if gamma < 4.5f32 * 0.018053968510807f32 {
        return gamma / 4.5f32
    } else if gamma < 1.0f32 {
        return powf(
            (gamma + 0.09929682680944f32) / 1.09929682680944f32,
            1.0f32 / 0.45f32,
        )
    } else {
        return 1.0f32
    };
}
unsafe extern "C" fn avifToGamma709(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    if linear < 0.0f32 {
        return 0.0f32
    } else if linear < 0.018053968510807f32 {
        return linear * 4.5f32
    } else if linear < 1.0f32 {
        return 1.09929682680944f32 * powf(linear, 0.45f32) - 0.09929682680944f32
    } else {
        return 1.0f32
    };
}
unsafe extern "C" fn avifToLinear470M(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    return powf(
        if gamma < 0.0f32 { 0.0f32 } else if 1.0f32 < gamma { 1.0f32 } else { gamma },
        2.2f32,
    );
}
unsafe extern "C" fn avifToGamma470M(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    return powf(
        if linear < 0.0f32 { 0.0f32 } else if 1.0f32 < linear { 1.0f32 } else { linear },
        1.0f32 / 2.2f32,
    );
}
unsafe extern "C" fn avifToLinear470BG(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    return powf(
        if gamma < 0.0f32 { 0.0f32 } else if 1.0f32 < gamma { 1.0f32 } else { gamma },
        2.8f32,
    );
}
unsafe extern "C" fn avifToGamma470BG(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    return powf(
        if linear < 0.0f32 { 0.0f32 } else if 1.0f32 < linear { 1.0f32 } else { linear },
        1.0f32 / 2.8f32,
    );
}
unsafe extern "C" fn avifToLinearSMPTE240(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    if gamma < 0.0f32 {
        return 0.0f32
    } else if gamma < 4.0f32 * 0.022821585529445f32 {
        return gamma / 4.0f32
    } else if gamma < 1.0f32 {
        return powf(
            (gamma + 0.111572195921731f32) / 1.111572195921731f32,
            1.0f32 / 0.45f32,
        )
    } else {
        return 1.0f32
    };
}
unsafe extern "C" fn avifToGammaSMPTE240(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    if linear < 0.0f32 {
        return 0.0f32
    } else if linear < 0.022821585529445f32 {
        return linear * 4.0f32
    } else if linear < 1.0f32 {
        return 1.111572195921731f32 * powf(linear, 0.45f32) - 0.111572195921731f32
    } else {
        return 1.0f32
    };
}
unsafe extern "C" fn avifToGammaLinear(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    return if gamma < 0.0f32 { 0.0f32 } else if 1.0f32 < gamma { 1.0f32 } else { gamma };
}
unsafe extern "C" fn avifToLinearLog100(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    let mid_interval: core::ffi::c_float = 0.01f32 / 2.0f32;
    return if gamma <= 0.0f32 {
        mid_interval
    } else {
        powf(10.0f32, 2.0f32 * ((if gamma < 1.0f32 { gamma } else { 1.0f32 }) - 1.0f32))
    };
}
unsafe extern "C" fn avifToGammaLog100(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    return if linear <= 0.01f32 {
        0.0f32
    } else {
        1.0f32 + log10f((if linear < 1.0f32 { linear } else { 1.0f32 })) / 2.0f32
    };
}
unsafe extern "C" fn avifToLinearLog100Sqrt10(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    let mid_interval: core::ffi::c_float = 0.00316227766f32 / 2.0f32;
    return if gamma <= 0.0f32 {
        mid_interval
    } else {
        powf(10.0f32, 2.5f32 * ((if gamma < 1.0f32 { gamma } else { 1.0f32 }) - 1.0f32))
    };
}
unsafe extern "C" fn avifToGammaLog100Sqrt10(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    return if linear <= 0.00316227766f32 {
        0.0f32
    } else {
        1.0f32 + log10f((if linear < 1.0f32 { linear } else { 1.0f32 })) / 2.5f32
    };
}
unsafe extern "C" fn avifToLinearIEC61966(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    if gamma < -4.5f32 * 0.018053968510807f32 {
        return -powf(
            (gamma - 0.09929682680944f32) / -1.09929682680944f32,
            1.0f32 / 0.45f32,
        )
    } else if gamma < 4.5f32 * 0.018053968510807f32 {
        return gamma / 4.5f32
    } else {
        return powf(
            (gamma + 0.09929682680944f32) / 1.09929682680944f32,
            1.0f32 / 0.45f32,
        )
    };
}
unsafe extern "C" fn avifToGammaIEC61966(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    if linear < -0.018053968510807f32 {
        return -1.09929682680944f32 * powf(-linear, 0.45f32) + 0.09929682680944f32
    } else if linear < 0.018053968510807f32 {
        return linear * 4.5f32
    } else {
        return 1.09929682680944f32 * powf(linear, 0.45f32) - 0.09929682680944f32
    };
}
unsafe extern "C" fn avifToLinearBT1361(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    if gamma < -0.25f32 {
        return -0.25f32
    } else if gamma < 0.0f32 {
        return powf(
            (gamma - 0.02482420670236f32) / -0.27482420670236f32,
            1.0f32 / 0.45f32,
        ) / -4.0f32
    } else if gamma < 4.5f32 * 0.018053968510807f32 {
        return gamma / 4.5f32
    } else if gamma < 1.0f32 {
        return powf(
            (gamma + 0.09929682680944f32) / 1.09929682680944f32,
            1.0f32 / 0.45f32,
        )
    } else {
        return 1.0f32
    };
}
unsafe extern "C" fn avifToGammaBT1361(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    if linear < -0.25f32 {
        return -0.25f32
    } else if linear < 0.0f32 {
        return -0.27482420670236f32 * powf(-4.0f32 * linear, 0.45f32)
            + 0.02482420670236f32
    } else if linear < 0.018053968510807f32 {
        return linear * 4.5f32
    } else if linear < 1.0f32 {
        return 1.09929682680944f32 * powf(linear, 0.45f32) - 0.09929682680944f32
    } else {
        return 1.0f32
    };
}
unsafe extern "C" fn avifToLinearSRGB(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    if gamma < 0.0f32 {
        return 0.0f32
    } else if gamma < 12.92f32 * 0.0030412825601275209f32 {
        return gamma / 12.92f32
    } else if gamma < 1.0f32 {
        return powf((gamma + 0.0550107189475866f32) / 1.0550107189475866f32, 2.4f32)
    } else {
        return 1.0f32
    };
}
unsafe extern "C" fn avifToGammaSRGB(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    if linear < 0.0f32 {
        return 0.0f32
    } else if linear < 0.0030412825601275209f32 {
        return linear * 12.92f32
    } else if linear < 1.0f32 {
        return 1.0550107189475866f32 * powf(linear, 1.0f32 / 2.4f32)
            - 0.0550107189475866f32
    } else {
        return 1.0f32
    };
}
pub const PQ_MAX_NITS: core::ffi::c_float = 10000.0f32;
pub const HLG_PEAK_LUMINANCE_NITS: core::ffi::c_float = 1000.0f32;
pub const SDR_WHITE_NITS: core::ffi::c_float = 203.0f32;
unsafe extern "C" fn avifToLinearPQ(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    if gamma > 0.0f32 {
        let powGamma: core::ffi::c_float = powf(gamma, 1.0f32 / 78.84375f32)
            as core::ffi::c_float;
        let num: core::ffi::c_float = if powGamma - 0.8359375f32 > 0.0f32 {
            powGamma - 0.8359375f32
        } else {
            0.0f32
        };
        let den: core::ffi::c_float = if 18.8515625f32 - 18.6875f32 * powGamma
            > 1.17549435e-38f32
        {
            18.8515625f32 - 18.6875f32 * powGamma
        } else {
            1.17549435e-38f32
        };
        let linear: core::ffi::c_float = powf(num / den, 1.0f32 / 0.1593017578125f32)
            as core::ffi::c_float;
        return linear * PQ_MAX_NITS / SDR_WHITE_NITS;
    } else {
        return 0.0f32
    };
}
unsafe extern "C" fn avifToGammaPQ(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    if linear > 0.0f32 {
        linear = if linear * 203.0f32 / 10000.0f32 < 0.0f32 {
            0.0f32
        } else if 1.0f32 < linear * 203.0f32 / 10000.0f32 {
            1.0f32
        } else {
            linear * 203.0f32 / 10000.0f32
        };
        let powLinear: core::ffi::c_float = powf(linear, 0.1593017578125f32)
            as core::ffi::c_float;
        let num: core::ffi::c_float = 0.1640625f32 * powLinear - 0.1640625f32;
        let den: core::ffi::c_float = 1.0f32 + 18.6875f32 * powLinear;
        return powf(1.0f32 + num / den, 78.84375f32);
    } else {
        return 0.0f32
    };
}
unsafe extern "C" fn avifToLinearSMPTE428(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    return powf((if gamma > 0.0f32 { gamma } else { 0.0f32 }), 2.6f32)
        / 0.91655527974030934f32;
}
unsafe extern "C" fn avifToGammaSMPTE428(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    return powf(
        0.91655527974030934f32 * (if linear > 0.0f32 { linear } else { 0.0f32 }),
        1.0f32 / 2.6f32,
    );
}
unsafe extern "C" fn avifToLinearHLG(
    mut gamma: core::ffi::c_float,
) -> core::ffi::c_float {
    if gamma < 0.0f32 {
        return 0.0f32;
    }
    let mut linear: core::ffi::c_float = 0.0f32;
    if gamma <= 0.5f32 {
        linear = powf(gamma * gamma * (1.0f32 / 3.0f32), 1.2f32);
    } else {
        linear = powf(
            (expf((gamma - 0.55991073f32) / 0.17883277f32) + 0.28466892f32) / 12.0f32,
            1.2f32,
        );
    }
    return linear * HLG_PEAK_LUMINANCE_NITS / SDR_WHITE_NITS;
}
unsafe extern "C" fn avifToGammaHLG(
    mut linear: core::ffi::c_float,
) -> core::ffi::c_float {
    linear = if linear * 203.0f32 / 1000.0f32 < 0.0f32 {
        0.0f32
    } else if 1.0f32 < linear * 203.0f32 / 1000.0f32 {
        1.0f32
    } else {
        linear * 203.0f32 / 1000.0f32
    };
    linear = powf(linear, 1.0f32 / 1.2f32);
    if linear < 0.0f32 {
        return 0.0f32
    } else if linear <= 1.0f32 / 12.0f32 {
        return sqrtf(3.0f32 * linear)
    } else {
        return 0.17883277f32 * logf(12.0f32 * linear - 0.28466892f32) + 0.55991073f32
    };
}
static mut transferCharacteristicsTables: [avifTransferCharacteristicsTable; 16] = unsafe {
    [
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_BT709
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"BT.709\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinear709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGamma709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_BT470M
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"BT.470-6 System M\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinear470M
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGamma470M
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_BT470BG
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"BT.470-6 System BG\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinear470BG
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGamma470BG
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_BT601
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"BT.601\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinear709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGamma709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_SMPTE240
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"SMPTE 240M\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearSMPTE240
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaSMPTE240
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_LINEAR
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"Linear\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToGammaLinear
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaLinear
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_LOG100
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"100:1 Log\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearLog100
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaLog100
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_LOG100_SQRT10
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"100sqrt(10):1 Log\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearLog100Sqrt10
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaLog100Sqrt10
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_IEC61966
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"IEC 61966-2-4\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearIEC61966
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaIEC61966
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_BT1361
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"BT.1361\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearBT1361
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaBT1361
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_SRGB
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"sRGB\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearSRGB
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaSRGB
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_BT2020_10BIT
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"10bit BT.2020\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinear709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGamma709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_BT2020_12BIT
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"12bit BT.2020\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinear709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGamma709
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_SMPTE2084
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"SMPTE ST 2084 (PQ)\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearPQ
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaPQ
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_SMPTE428
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"SMPTE ST 428-1\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearSMPTE428
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaSMPTE428
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
        {
            let mut init = avifTransferCharacteristicsTable {
                transferCharacteristicsEnum: AVIF_TRANSFER_CHARACTERISTICS_HLG
                    as core::ffi::c_int as avifTransferCharacteristics,
                name: b"ARIB STD-B67 (HLG)\0" as *const u8 as *const core::ffi::c_char,
                toLinear: Some(
                    avifToLinearHLG
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
                toGamma: Some(
                    avifToGammaHLG
                        as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
                ),
            };
            init
        },
    ]
};
static mut avifTransferCharacteristicsTableSize: core::ffi::c_int = 0;
#[no_mangle]
pub unsafe extern "C" fn avifTransferCharacteristicsGetGammaToLinearFunction(
    mut atc: avifTransferCharacteristics,
) -> avifTransferFunction {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < avifTransferCharacteristicsTableSize {
        let table: *const avifTransferCharacteristicsTable = &*core::ptr::addr_of!(transferCharacteristicsTables).cast::<avifTransferCharacteristicsTable>()
            .offset(i as isize) as *const avifTransferCharacteristicsTable;
        if (*table).transferCharacteristicsEnum as core::ffi::c_int
            == atc as core::ffi::c_int
        {
            return (*table).toLinear;
        }
        i += 1;
    }
    return Some(
        avifToLinear709 as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifTransferCharacteristicsGetLinearToGammaFunction(
    mut atc: avifTransferCharacteristics,
) -> avifTransferFunction {
    let mut i: core::ffi::c_int = 0 as core::ffi::c_int;
    while i < avifTransferCharacteristicsTableSize {
        let table: *const avifTransferCharacteristicsTable = &*core::ptr::addr_of!(transferCharacteristicsTables).cast::<avifTransferCharacteristicsTable>()
            .offset(i as isize) as *const avifTransferCharacteristicsTable;
        if (*table).transferCharacteristicsEnum as core::ffi::c_int
            == atc as core::ffi::c_int
        {
            return (*table).toGamma;
        }
        i += 1;
    }
    return Some(
        avifToGamma709 as unsafe extern "C" fn(core::ffi::c_float) -> core::ffi::c_float,
    );
}
#[no_mangle]
pub unsafe extern "C" fn avifColorPrimariesComputeYCoeffs(
    mut colorPrimaries: avifColorPrimaries,
    mut coeffs: *mut core::ffi::c_float,
) {
    let mut primaries: [core::ffi::c_float; 8] = [0.; 8];
    avifColorPrimariesGetValues(colorPrimaries, primaries.as_mut_ptr());
    let rX: core::ffi::c_float = primaries[0 as core::ffi::c_int as usize];
    let rY: core::ffi::c_float = primaries[1 as core::ffi::c_int as usize];
    let gX: core::ffi::c_float = primaries[2 as core::ffi::c_int as usize];
    let gY: core::ffi::c_float = primaries[3 as core::ffi::c_int as usize];
    let bX: core::ffi::c_float = primaries[4 as core::ffi::c_int as usize];
    let bY: core::ffi::c_float = primaries[5 as core::ffi::c_int as usize];
    let wX: core::ffi::c_float = primaries[6 as core::ffi::c_int as usize];
    let wY: core::ffi::c_float = primaries[7 as core::ffi::c_int as usize];
    let rZ: core::ffi::c_float = 1.0f32 - (rX + rY);
    let gZ: core::ffi::c_float = 1.0f32 - (gX + gY);
    let bZ: core::ffi::c_float = 1.0f32 - (bX + bY);
    let wZ: core::ffi::c_float = 1.0f32 - (wX + wY);
    let kr: core::ffi::c_float = rY
        * (wX * (gY * bZ - bY * gZ) + wY * (bX * gZ - gX * bZ)
            + wZ * (gX * bY - bX * gY))
        / (wY
            * (rX * (gY * bZ - bY * gZ) + gX * (bY * rZ - rY * bZ)
                + bX * (rY * gZ - gY * rZ)));
    let kb: core::ffi::c_float = bY
        * (wX * (rY * gZ - gY * rZ) + wY * (gX * rZ - rX * gZ)
            + wZ * (rX * gY - gX * rY))
        / (wY
            * (rX * (gY * bZ - bY * gZ) + gX * (bY * rZ - rY * bZ)
                + bX * (rY * gZ - gY * rZ)));
    *coeffs.offset(0 as core::ffi::c_int as isize) = kr;
    *coeffs.offset(2 as core::ffi::c_int as isize) = kb;
    *coeffs.offset(1 as core::ffi::c_int as isize) = 1.0f32
        - *coeffs.offset(0 as core::ffi::c_int as isize)
        - *coeffs.offset(2 as core::ffi::c_int as isize);
}
unsafe extern "C" fn run_static_initializers() {
    avifColorPrimariesTableSize = (::core::mem::size_of::<
        [avifColorPrimariesTable; 11],
    >() as usize)
        .wrapping_div(::core::mem::size_of::<avifColorPrimariesTable>() as usize)
        as core::ffi::c_int;
    avifMatrixCoefficientsTableSize = (::core::mem::size_of::<
        [avifMatrixCoefficientsTable; 6],
    >() as usize)
        .wrapping_div(::core::mem::size_of::<avifMatrixCoefficientsTable>() as usize)
        as core::ffi::c_int;
    avifTransferCharacteristicsTableSize = (::core::mem::size_of::<
        [avifTransferCharacteristicsTable; 16],
    >() as usize)
        .wrapping_div(
            ::core::mem::size_of::<avifTransferCharacteristicsTable>() as usize,
        ) as core::ffi::c_int;
}
#[used]
#[cfg_attr(target_os = "linux", link_section = ".init_array")]
#[cfg_attr(target_os = "windows", link_section = ".CRT$XIB")]
#[cfg_attr(target_os = "macos", link_section = "__DATA,__mod_init_func")]
#[cfg_attr(libavif_rs_asan, sanitize(address = "off"))]
static INIT_ARRAY: [unsafe extern "C" fn(); 1] = [run_static_initializers];
