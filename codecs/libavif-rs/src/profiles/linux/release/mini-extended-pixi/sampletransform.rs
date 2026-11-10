// Copyright 2024 Google LLC
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/sampletransform.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    fn avifAlloc(size: size_t) -> *mut core::ffi::c_void;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifImageUsesU16(image: *const avifImage) -> avifBool;
    fn avifImagePlane(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> *mut uint8_t;
    fn avifImagePlaneRowBytes(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImagePlaneWidth(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifImagePlaneHeight(
        image: *const avifImage,
        channel: core::ffi::c_int,
    ) -> uint32_t;
    fn avifArrayCreate(
        arrayStruct: *mut core::ffi::c_void,
        elementSize: uint32_t,
        initialCapacity: uint32_t,
    ) -> avifBool;
    fn avifArrayPush(arrayStruct: *mut core::ffi::c_void) -> *mut core::ffi::c_void;
    fn avifArrayDestroy(arrayStruct: *mut core::ffi::c_void);
}
pub type size_t = usize;
pub type __uint8_t = u8;
pub type __uint16_t = u16;
pub type __int32_t = i32;
pub type __uint32_t = u32;
pub type __int64_t = i64;
pub type int32_t = __int32_t;
pub type int64_t = __int64_t;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type avifBool = core::ffi::c_int;
pub type avifPlanesFlag = core::ffi::c_uint;
pub const AVIF_PLANES_ALL: avifPlanesFlag = 255;
pub const AVIF_PLANES_A: avifPlanesFlag = 2;
pub const AVIF_PLANES_YUV: avifPlanesFlag = 1;
pub type avifPlanesFlags = uint32_t;
pub type avifChannelIndex = core::ffi::c_uint;
pub const AVIF_CHAN_A: avifChannelIndex = 3;
pub const AVIF_CHAN_V: avifChannelIndex = 2;
pub const AVIF_CHAN_U: avifChannelIndex = 1;
pub const AVIF_CHAN_Y: avifChannelIndex = 0;
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
pub type avifSampleTransformRecipe = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B: avifSampleTransformRecipe = 3;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B: avifSampleTransformRecipe = 2;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B: avifSampleTransformRecipe = 1;
pub const AVIF_SAMPLE_TRANSFORM_NONE: avifSampleTransformRecipe = 0;
pub type avifSampleTransformBitDepth = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_64: avifSampleTransformBitDepth = 3;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32: avifSampleTransformBitDepth = 2;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_16: avifSampleTransformBitDepth = 1;
pub const AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_8: avifSampleTransformBitDepth = 0;
pub type avifSampleTransformTokenType = core::ffi::c_uint;
pub const AVIF_SAMPLE_TRANSFORM_RESERVED: avifSampleTransformTokenType = 138;
pub const AVIF_SAMPLE_TRANSFORM_LAST_BINARY_OPERATOR: avifSampleTransformTokenType = 137;
pub const AVIF_SAMPLE_TRANSFORM_MAX: avifSampleTransformTokenType = 137;
pub const AVIF_SAMPLE_TRANSFORM_MIN: avifSampleTransformTokenType = 136;
pub const AVIF_SAMPLE_TRANSFORM_POW: avifSampleTransformTokenType = 135;
pub const AVIF_SAMPLE_TRANSFORM_XOR: avifSampleTransformTokenType = 134;
pub const AVIF_SAMPLE_TRANSFORM_OR: avifSampleTransformTokenType = 133;
pub const AVIF_SAMPLE_TRANSFORM_AND: avifSampleTransformTokenType = 132;
pub const AVIF_SAMPLE_TRANSFORM_QUOTIENT: avifSampleTransformTokenType = 131;
pub const AVIF_SAMPLE_TRANSFORM_PRODUCT: avifSampleTransformTokenType = 130;
pub const AVIF_SAMPLE_TRANSFORM_DIFFERENCE: avifSampleTransformTokenType = 129;
pub const AVIF_SAMPLE_TRANSFORM_SUM: avifSampleTransformTokenType = 128;
pub const AVIF_SAMPLE_TRANSFORM_FIRST_BINARY_OPERATOR: avifSampleTransformTokenType = 128;
pub const AVIF_SAMPLE_TRANSFORM_LAST_UNARY_OPERATOR: avifSampleTransformTokenType = 67;
pub const AVIF_SAMPLE_TRANSFORM_BSR: avifSampleTransformTokenType = 67;
pub const AVIF_SAMPLE_TRANSFORM_NOT: avifSampleTransformTokenType = 66;
pub const AVIF_SAMPLE_TRANSFORM_ABSOLUTE: avifSampleTransformTokenType = 65;
pub const AVIF_SAMPLE_TRANSFORM_NEGATION: avifSampleTransformTokenType = 64;
pub const AVIF_SAMPLE_TRANSFORM_FIRST_UNARY_OPERATOR: avifSampleTransformTokenType = 64;
pub const AVIF_SAMPLE_TRANSFORM_LAST_INPUT_IMAGE_ITEM_INDEX: avifSampleTransformTokenType = 32;
pub const AVIF_SAMPLE_TRANSFORM_FIRST_INPUT_IMAGE_ITEM_INDEX: avifSampleTransformTokenType = 1;
pub const AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX: avifSampleTransformTokenType = 1;
pub const AVIF_SAMPLE_TRANSFORM_CONSTANT: avifSampleTransformTokenType = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTransformToken {
    pub type_0: avifSampleTransformTokenType,
    pub constant: int32_t,
    pub inputImageItemIndex: uint8_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifSampleTransformExpression {
    pub tokens: *mut avifSampleTransformToken,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const INT32_MIN: core::ffi::c_int = -(2147483647 as core::ffi::c_int)
    - 1 as core::ffi::c_int;
pub const INT32_MAX: core::ffi::c_int = 2147483647 as core::ffi::c_int;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
#[no_mangle]
pub unsafe extern "C" fn avifSampleTransformExpressionIsValid(
    mut tokens: *const avifSampleTransformExpression,
    mut numInputImageItems: uint32_t,
) -> avifBool {
    let mut stackSize: uint32_t = 0 as uint32_t;
    let mut t: uint32_t = 0 as uint32_t;
    while t < (*tokens).count {
        let mut token: *const avifSampleTransformToken = &mut *((*tokens).tokens)
            .offset(t as isize) as *mut avifSampleTransformToken;
        if !(((*token).type_0 as core::ffi::c_uint)
            < AVIF_SAMPLE_TRANSFORM_RESERVED as core::ffi::c_int as core::ffi::c_uint)
        {
            return AVIF_FALSE;
        }
        if (*token).type_0 as core::ffi::c_uint
            == AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX as core::ffi::c_int
                as core::ffi::c_uint
        {
            if !((*token).inputImageItemIndex as core::ffi::c_int
                != 0 as core::ffi::c_int)
            {
                return AVIF_FALSE;
            }
            if !((*token).inputImageItemIndex as uint32_t <= numInputImageItems) {
                return AVIF_FALSE;
            }
        }
        if ((*token).type_0 as core::ffi::c_uint)
            < AVIF_SAMPLE_TRANSFORM_FIRST_UNARY_OPERATOR as core::ffi::c_int
                as core::ffi::c_uint
        {
            stackSize = stackSize.wrapping_add(1);
        } else if ((*token).type_0 as core::ffi::c_uint)
            < AVIF_SAMPLE_TRANSFORM_FIRST_BINARY_OPERATOR as core::ffi::c_int
                as core::ffi::c_uint
        {
            if !(stackSize >= 1 as uint32_t) {
                return AVIF_FALSE;
            }
        } else {
            if !(stackSize >= 2 as uint32_t) {
                return AVIF_FALSE;
            }
            stackSize = stackSize.wrapping_sub(1);
        }
        t = t.wrapping_add(1);
    }
    if !(stackSize == 1 as uint32_t) {
        return AVIF_FALSE;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifSampleTransformExpressionIsEquivalentTo(
    mut a: *const avifSampleTransformExpression,
    mut b: *const avifSampleTransformExpression,
) -> avifBool {
    if (*a).count != (*b).count {
        return AVIF_FALSE;
    }
    let mut t: uint32_t = 0 as uint32_t;
    while t < (*a).count {
        let mut aToken: *const avifSampleTransformToken = &mut *((*a).tokens)
            .offset(t as isize) as *mut avifSampleTransformToken;
        let mut bToken: *const avifSampleTransformToken = &mut *((*b).tokens)
            .offset(t as isize) as *mut avifSampleTransformToken;
        if (*aToken).type_0 as core::ffi::c_uint != (*bToken).type_0 as core::ffi::c_uint
            || (*aToken).type_0 as core::ffi::c_uint
                == AVIF_SAMPLE_TRANSFORM_CONSTANT as core::ffi::c_int
                    as core::ffi::c_uint && (*aToken).constant != (*bToken).constant
        {
            return AVIF_FALSE;
        }
        t = t.wrapping_add(1);
    }
    return AVIF_TRUE;
}
unsafe extern "C" fn avifPushConstant(
    mut expression: *mut avifSampleTransformExpression,
    mut constant: int32_t,
) -> avifBool {
    let mut token: *mut avifSampleTransformToken = avifArrayPush(
        expression as *mut core::ffi::c_void,
    ) as *mut avifSampleTransformToken;
    if token.is_null() {
        return AVIF_FALSE;
    }
    (*token).type_0 = AVIF_SAMPLE_TRANSFORM_CONSTANT;
    (*token).constant = constant;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifPushInputImageItem(
    mut expression: *mut avifSampleTransformExpression,
    mut inputImageItemIndex: uint8_t,
) -> avifBool {
    let mut token: *mut avifSampleTransformToken = avifArrayPush(
        expression as *mut core::ffi::c_void,
    ) as *mut avifSampleTransformToken;
    if token.is_null() {
        return AVIF_FALSE;
    }
    (*token).type_0 = AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX;
    (*token).inputImageItemIndex = inputImageItemIndex;
    return AVIF_TRUE;
}
unsafe extern "C" fn avifPushOperator(
    mut expression: *mut avifSampleTransformExpression,
    mut operator: avifSampleTransformTokenType,
) -> avifBool {
    let mut token: *mut avifSampleTransformToken = avifArrayPush(
        expression as *mut core::ffi::c_void,
    ) as *mut avifSampleTransformToken;
    if token.is_null() {
        return AVIF_FALSE;
    }
    (*token).type_0 = operator as uint8_t as avifSampleTransformTokenType;
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifSampleTransformRecipeToExpression(
    mut recipe: avifSampleTransformRecipe,
    mut expression: *mut avifSampleTransformExpression,
) -> avifResult {
    if recipe as core::ffi::c_uint
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B as core::ffi::c_int
            as core::ffi::c_uint
    {
        if avifArrayCreate(
            expression as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSampleTransformToken>() as uint32_t,
            5 as uint32_t,
        ) == 0
        {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if avifPushConstant(expression, 256 as int32_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushInputImageItem(expression, 1 as uint8_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_PRODUCT) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushInputImageItem(expression, 2 as uint8_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_OR) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        return AVIF_RESULT_OK;
    }
    if recipe as core::ffi::c_uint
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B as core::ffi::c_int
            as core::ffi::c_uint
    {
        if avifArrayCreate(
            expression as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSampleTransformToken>() as uint32_t,
            7 as uint32_t,
        ) == 0
        {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if avifPushConstant(expression, 16 as int32_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushInputImageItem(expression, 1 as uint8_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_PRODUCT) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushInputImageItem(expression, 2 as uint8_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushConstant(expression, 16 as int32_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_QUOTIENT) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_SUM) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        return AVIF_RESULT_OK;
    }
    if recipe as core::ffi::c_uint
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B
            as core::ffi::c_int as core::ffi::c_uint
    {
        if avifArrayCreate(
            expression as *mut core::ffi::c_void,
            ::core::mem::size_of::<avifSampleTransformToken>() as uint32_t,
            7 as uint32_t,
        ) == 0
        {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        if avifPushConstant(expression, 16 as int32_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushInputImageItem(expression, 1 as uint8_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_PRODUCT) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushInputImageItem(expression, 2 as uint8_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_SUM) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushConstant(expression, 128 as int32_t) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        if avifPushOperator(expression, AVIF_SAMPLE_TRANSFORM_DIFFERENCE) == 0 {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        return AVIF_RESULT_OK;
    }
    return AVIF_RESULT_INVALID_ARGUMENT;
}
#[no_mangle]
pub unsafe extern "C" fn avifSampleTransformExpressionToRecipe(
    mut expression: *const avifSampleTransformExpression,
    mut recipe: *mut avifSampleTransformRecipe,
) -> avifResult {
    *recipe = AVIF_SAMPLE_TRANSFORM_NONE;
    let kAllRecipes: [avifSampleTransformRecipe; 3] = [
        AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_8B_8B,
        AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_4B,
        AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_EXTENSION_12B_8B_OVERLAP_4B,
    ];
    let mut i: size_t = 0 as size_t;
    while i
        < (::core::mem::size_of::<[avifSampleTransformRecipe; 3]>() as usize)
            .wrapping_div(::core::mem::size_of::<avifSampleTransformRecipe>() as usize)
    {
        let mut candidateRecipe: avifSampleTransformRecipe = kAllRecipes[i as usize];
        let mut candidateExpression: avifSampleTransformExpression = {
            let mut init = avifSampleTransformExpression {
                tokens: 0 as *mut avifSampleTransformToken,
                elementSize: 0,
                count: 0,
                capacity: 0,
            };
            init
        };
        let result__: avifResult = avifSampleTransformRecipeToExpression(
            candidateRecipe,
            &mut candidateExpression,
        ) as avifResult;
        if result__ as core::ffi::c_uint
            != AVIF_RESULT_OK as core::ffi::c_int as core::ffi::c_uint
        {
            return result__;
        }
        let equivalence: avifBool = avifSampleTransformExpressionIsEquivalentTo(
            expression,
            &mut candidateExpression,
        ) as avifBool;
        avifArrayDestroy(
            &mut candidateExpression as *mut avifSampleTransformExpression
                as *mut core::ffi::c_void,
        );
        if equivalence != 0 {
            *recipe = candidateRecipe;
            return AVIF_RESULT_OK;
        }
        i = i.wrapping_add(1);
    }
    return AVIF_RESULT_OK;
}
unsafe extern "C" fn avifSampleTransformClamp32b(mut value: int64_t) -> int32_t {
    return if value <= INT32_MIN as int64_t {
        INT32_MIN as int32_t
    } else if value >= INT32_MAX as int64_t {
        INT32_MAX as int32_t
    } else {
        value as int32_t
    };
}
unsafe extern "C" fn avifSampleTransformOperation32bOneOperand(
    mut operand: int32_t,
    mut operator: uint8_t,
) -> int32_t {
    match operator as core::ffi::c_int {
        64 => return avifSampleTransformClamp32b(-(operand as int64_t)),
        65 => {
            return if operand >= 0 as int32_t {
                operand
            } else {
                avifSampleTransformClamp32b(-(operand as int64_t))
            };
        }
        66 => return !operand,
        67 => {
            if operand <= 0 as int32_t {
                return 0 as int32_t;
            }
            let mut log2: int32_t = 0 as int32_t;
            operand >>= 1 as core::ffi::c_int;
            while operand != 0 as int32_t {
                operand >>= 1 as core::ffi::c_int;
                log2 += 1;
            }
            return log2;
        }
        _ => {}
    }
    return 0 as int32_t;
}
unsafe extern "C" fn avifSampleTransformOperation32bTwoOperands(
    mut leftOperand: int32_t,
    mut rightOperand: int32_t,
    mut operator: uint8_t,
) -> int32_t {
    match operator as core::ffi::c_int {
        128 => {
            return avifSampleTransformClamp32b(
                leftOperand as int64_t + rightOperand as int64_t,
            );
        }
        129 => {
            return avifSampleTransformClamp32b(
                leftOperand as int64_t - rightOperand as int64_t,
            );
        }
        130 => {
            return avifSampleTransformClamp32b(
                leftOperand as int64_t * rightOperand as int64_t,
            );
        }
        131 => {
            return if rightOperand == 0 as int32_t {
                leftOperand
            } else {
                avifSampleTransformClamp32b(
                    leftOperand as int64_t / rightOperand as int64_t,
                )
            };
        }
        132 => return leftOperand & rightOperand,
        133 => return leftOperand | rightOperand,
        134 => return leftOperand ^ rightOperand,
        135 => {
            if leftOperand == 0 as int32_t || leftOperand == 1 as int32_t {
                return leftOperand;
            }
            if leftOperand == -(1 as int32_t) {
                return if rightOperand % 2 as int32_t == 0 as int32_t {
                    1 as int32_t
                } else {
                    -(1 as int32_t)
                };
            }
            if rightOperand == 0 as int32_t {
                return 1 as int32_t;
            }
            if rightOperand == 1 as int32_t {
                return leftOperand;
            }
            if rightOperand < 0 as int32_t {
                return 0 as int32_t;
            }
            let mut result: int64_t = leftOperand as int64_t;
            let mut i: int32_t = 1 as int32_t;
            while i < rightOperand {
                result *= leftOperand as int64_t;
                if result < INT32_MIN as int64_t || result > INT32_MAX as int64_t {
                    return if leftOperand > 0 as int32_t
                        || rightOperand % 2 as int32_t == 0 as int32_t
                    {
                        INT32_MAX as int32_t
                    } else {
                        INT32_MIN as int32_t
                    };
                }
                i += 1;
            }
            return result as int32_t;
        }
        136 => {
            return if leftOperand <= rightOperand { leftOperand } else { rightOperand };
        }
        137 => {
            return if leftOperand <= rightOperand { rightOperand } else { leftOperand };
        }
        _ => {}
    }
    return 0 as int32_t;
}
unsafe extern "C" fn avifImageApplyExpression32b(
    mut dstImage: *mut avifImage,
    mut expression: *const avifSampleTransformExpression,
    mut inputImageItems: *mut *const avifImage,
    mut planes: avifPlanesFlags,
    mut stack: *mut int32_t,
    mut stackCapacity: uint32_t,
) -> avifResult {
    let minValue: int32_t = 0 as int32_t;
    let maxValue: int32_t = ((1 as int32_t) << (*dstImage).depth) - 1 as int32_t;
    let skipColor: avifBool = (planes
        & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags == 0)
        as core::ffi::c_int;
    let skipAlpha: avifBool = (planes
        & AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags == 0) as core::ffi::c_int;
    let mut c: core::ffi::c_int = AVIF_CHAN_Y as core::ffi::c_int;
    while c <= AVIF_CHAN_A as core::ffi::c_int {
        let alpha: avifBool = (c == AVIF_CHAN_A as core::ffi::c_int) as core::ffi::c_int;
        if !(skipColor != 0 && alpha == 0 || skipAlpha != 0 && alpha != 0) {
            let planeWidth: uint32_t = avifImagePlaneWidth(dstImage, c) as uint32_t;
            let planeHeight: uint32_t = avifImagePlaneHeight(dstImage, c) as uint32_t;
            let mut y: uint32_t = 0 as uint32_t;
            while y < planeHeight {
                let mut x: uint32_t = 0 as uint32_t;
                while x < planeWidth {
                    let mut stackSize: uint32_t = 0 as uint32_t;
                    let mut t: uint32_t = 0 as uint32_t;
                    while t < (*expression).count {
                        let mut token: *const avifSampleTransformToken = &mut *((*expression)
                            .tokens)
                            .offset(t as isize) as *mut avifSampleTransformToken;
                        if (*token).type_0 as core::ffi::c_uint
                            == AVIF_SAMPLE_TRANSFORM_CONSTANT as core::ffi::c_int
                                as core::ffi::c_uint
                        {
                            if !(stackSize < stackCapacity) {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                            let fresh0 = stackSize;
                            stackSize = stackSize.wrapping_add(1);
                            *stack.offset(fresh0 as isize) = (*token).constant;
                        } else if (*token).type_0 as core::ffi::c_uint
                            == AVIF_SAMPLE_TRANSFORM_INPUT_IMAGE_ITEM_INDEX
                                as core::ffi::c_int as core::ffi::c_uint
                        {
                            let mut image: *const avifImage = *inputImageItems
                                .offset(
                                    ((*token).inputImageItemIndex as core::ffi::c_int
                                        - 1 as core::ffi::c_int) as isize,
                                );
                            let mut row: *const uint8_t = avifImagePlane(image, c);
                            if row.is_null() {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                            row = row
                                .offset(
                                    (avifImagePlaneRowBytes(image, c) as size_t)
                                        .wrapping_mul(y as size_t) as isize,
                                );
                            if !(stackSize < stackCapacity) {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                            let fresh1 = stackSize;
                            stackSize = stackSize.wrapping_add(1);
                            *stack.offset(fresh1 as isize) = (if avifImageUsesU16(image)
                                != 0
                            {
                                *(row as *const uint16_t).offset(x as isize)
                                    as core::ffi::c_int
                            } else {
                                *row.offset(x as isize) as core::ffi::c_int
                            }) as int32_t;
                        } else if (*token).type_0 as core::ffi::c_uint
                            == AVIF_SAMPLE_TRANSFORM_NEGATION as core::ffi::c_int
                                as core::ffi::c_uint
                            || (*token).type_0 as core::ffi::c_uint
                                == AVIF_SAMPLE_TRANSFORM_ABSOLUTE as core::ffi::c_int
                                    as core::ffi::c_uint
                            || (*token).type_0 as core::ffi::c_uint
                                == AVIF_SAMPLE_TRANSFORM_NOT as core::ffi::c_int
                                    as core::ffi::c_uint
                            || (*token).type_0 as core::ffi::c_uint
                                == AVIF_SAMPLE_TRANSFORM_BSR as core::ffi::c_int
                                    as core::ffi::c_uint
                        {
                            if !(stackSize >= 1 as uint32_t) {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                            *stack
                                .offset(stackSize.wrapping_sub(1 as uint32_t) as isize) = avifSampleTransformOperation32bOneOperand(
                                *stack
                                    .offset(stackSize.wrapping_sub(1 as uint32_t) as isize),
                                (*token).type_0 as uint8_t,
                            );
                        } else {
                            if !(stackSize >= 2 as uint32_t) {
                                return AVIF_RESULT_INTERNAL_ERROR;
                            }
                            *stack
                                .offset(stackSize.wrapping_sub(2 as uint32_t) as isize) = avifSampleTransformOperation32bTwoOperands(
                                *stack
                                    .offset(stackSize.wrapping_sub(2 as uint32_t) as isize),
                                *stack
                                    .offset(stackSize.wrapping_sub(1 as uint32_t) as isize),
                                (*token).type_0 as uint8_t,
                            );
                            stackSize = stackSize.wrapping_sub(1);
                        }
                        t = t.wrapping_add(1);
                    }
                    if !(stackSize == 1 as uint32_t) {
                        return AVIF_RESULT_INTERNAL_ERROR;
                    }
                    *stack.offset(0 as core::ffi::c_int as isize) = if *stack
                        .offset(0 as core::ffi::c_int as isize) < minValue
                    {
                        minValue
                    } else if maxValue < *stack.offset(0 as core::ffi::c_int as isize) {
                        maxValue
                    } else {
                        *stack.offset(0 as core::ffi::c_int as isize)
                    };
                    let mut row_0: *mut uint8_t = avifImagePlane(dstImage, c);
                    if row_0.is_null() {
                        return AVIF_RESULT_INTERNAL_ERROR;
                    }
                    row_0 = row_0
                        .offset(
                            (avifImagePlaneRowBytes(dstImage, c) as size_t)
                                .wrapping_mul(y as size_t) as isize,
                        );
                    if avifImageUsesU16(dstImage) != 0 {
                        *(row_0 as *mut uint16_t).offset(x as isize) = *stack
                            .offset(0 as core::ffi::c_int as isize) as uint16_t;
                    } else {
                        *row_0.offset(x as isize) = *stack
                            .offset(0 as core::ffi::c_int as isize) as uint8_t;
                    }
                    x = x.wrapping_add(1);
                }
                y = y.wrapping_add(1);
            }
        }
        c += 1;
    }
    return AVIF_RESULT_OK;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageApplyExpression(
    mut dstImage: *mut avifImage,
    mut bitDepth: avifSampleTransformBitDepth,
    mut expression: *const avifSampleTransformExpression,
    mut numInputImageItems: uint8_t,
    mut inputImageItems: *mut *const avifImage,
    mut planes: avifPlanesFlags,
) -> avifResult {
    if avifSampleTransformExpressionIsValid(expression, numInputImageItems as uint32_t)
        == 0
    {
        return AVIF_RESULT_INTERNAL_ERROR;
    }
    let skipColor: avifBool = (planes
        & AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags == 0)
        as core::ffi::c_int;
    let skipAlpha: avifBool = (planes
        & AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags == 0) as core::ffi::c_int;
    let mut c: core::ffi::c_int = AVIF_CHAN_Y as core::ffi::c_int;
    while c <= AVIF_CHAN_A as core::ffi::c_int {
        let alpha: avifBool = (c == AVIF_CHAN_A as core::ffi::c_int) as core::ffi::c_int;
        if !(skipColor != 0 && alpha == 0 || skipAlpha != 0 && alpha != 0) {
            let planeWidth: uint32_t = avifImagePlaneWidth(dstImage, c) as uint32_t;
            let planeHeight: uint32_t = avifImagePlaneHeight(dstImage, c) as uint32_t;
            let mut i: uint32_t = 0 as uint32_t;
            while i < numInputImageItems as uint32_t {
                if !(avifImagePlaneWidth(*inputImageItems.offset(i as isize), c)
                    == planeWidth)
                {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                if !(avifImagePlaneHeight(*inputImageItems.offset(i as isize), c)
                    == planeHeight)
                {
                    return AVIF_RESULT_BMFF_PARSE_FAILED;
                }
                i = i.wrapping_add(1);
            }
        }
        c += 1;
    }
    if bitDepth as core::ffi::c_uint
        == AVIF_SAMPLE_TRANSFORM_BIT_DEPTH_32 as core::ffi::c_int as core::ffi::c_uint
    {
        let mut stackCapacity: uint32_t = ((*expression).count)
            .wrapping_div(2 as uint32_t)
            .wrapping_add(1 as uint32_t);
        let mut stack: *mut int32_t = avifAlloc(
            (stackCapacity as size_t)
                .wrapping_mul(::core::mem::size_of::<int32_t>() as size_t),
        ) as *mut int32_t;
        if stack.is_null() {
            return AVIF_RESULT_OUT_OF_MEMORY;
        }
        let result: avifResult = avifImageApplyExpression32b(
            dstImage,
            expression,
            inputImageItems,
            planes,
            stack,
            stackCapacity,
        ) as avifResult;
        avifFree(stack as *mut core::ffi::c_void);
        return result;
    }
    return AVIF_RESULT_NOT_IMPLEMENTED;
}
#[no_mangle]
pub unsafe extern "C" fn avifImageApplyOperations(
    mut dstImage: *mut avifImage,
    mut bitDepth: avifSampleTransformBitDepth,
    mut numTokens: uint32_t,
    mut tokens: *const avifSampleTransformToken,
    mut numInputImageItems: uint8_t,
    mut inputImageItems: *mut *const avifImage,
    mut planes: avifPlanesFlags,
) -> avifResult {
    let mut expression: avifSampleTransformExpression = {
        let mut init = avifSampleTransformExpression {
            tokens: 0 as *mut avifSampleTransformToken,
            elementSize: 0,
            count: 0,
            capacity: 0,
        };
        init
    };
    if avifArrayCreate(
        &mut expression as *mut avifSampleTransformExpression as *mut core::ffi::c_void,
        ::core::mem::size_of::<avifSampleTransformToken>() as uint32_t,
        numTokens,
    ) == 0
    {
        return AVIF_RESULT_OUT_OF_MEMORY;
    }
    let mut t: uint32_t = 0 as uint32_t;
    while t < numTokens {
        let mut token: *mut avifSampleTransformToken = avifArrayPush(
            &mut expression as *mut avifSampleTransformExpression
                as *mut core::ffi::c_void,
        ) as *mut avifSampleTransformToken;
        if token.is_null() {
            return AVIF_RESULT_INTERNAL_ERROR;
        }
        *token = *tokens.offset(t as isize);
        t = t.wrapping_add(1);
    }
    let result: avifResult = avifImageApplyExpression(
        dstImage,
        bitDepth,
        &mut expression,
        numInputImageItems,
        inputImageItems,
        planes,
    ) as avifResult;
    avifArrayDestroy(
        &mut expression as *mut avifSampleTransformExpression as *mut core::ffi::c_void,
    );
    return result;
}
