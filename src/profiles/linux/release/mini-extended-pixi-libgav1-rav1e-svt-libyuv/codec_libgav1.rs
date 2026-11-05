// Copyright 2020 Google LLC
// SPDX-License-Identifier: BSD-2-Clause

// Mechanically translated from upstream/libavif/src/codec_libgav1.c.
// C ABI, ownership, pointer validity, and bounds follow the original headers.
extern "C" {
    pub type avifEncoderData;
    pub type Libgav1Decoder;
    fn avifFree(p: *mut core::ffi::c_void);
    fn avifImageFreePlanes(image: *mut avifImage, planes: avifPlanesFlags);
    fn avifCalloc(count: size_t, size: size_t) -> *mut core::ffi::c_void;
    fn Libgav1DecoderSettingsInitDefault(settings: *mut Libgav1DecoderSettings);
    fn Libgav1GetVersionString() -> *const core::ffi::c_char;
    fn Libgav1DecoderCreate(
        settings: *const Libgav1DecoderSettings,
        decoder_out: *mut *mut Libgav1Decoder,
    ) -> Libgav1StatusCode;
    fn Libgav1DecoderDestroy(decoder: *mut Libgav1Decoder);
    fn Libgav1DecoderEnqueueFrame(
        decoder: *mut Libgav1Decoder,
        data: *const uint8_t,
        size: size_t,
        user_private_data: int64_t,
        buffer_private_data: *mut core::ffi::c_void,
    ) -> Libgav1StatusCode;
    fn Libgav1DecoderDequeueFrame(
        decoder: *mut Libgav1Decoder,
        out_ptr: *mut *const Libgav1DecoderBuffer,
    ) -> Libgav1StatusCode;
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
pub type avifPlanesFlag = core::ffi::c_uint;
pub const AVIF_PLANES_ALL: avifPlanesFlag = 255;
pub const AVIF_PLANES_A: avifPlanesFlag = 2;
pub const AVIF_PLANES_YUV: avifPlanesFlag = 1;
pub type avifPlanesFlags = uint32_t;
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
pub type avifHeaderFormatFlags = core::ffi::c_int;
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
pub struct avifDiagnostics {
    pub error: [core::ffi::c_char; 256],
}
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
pub type avifCodecChoice = core::ffi::c_uint;
pub const AVIF_CODEC_CHOICE_AVM: avifCodecChoice = 6;
pub const AVIF_CODEC_CHOICE_SVT: avifCodecChoice = 5;
pub const AVIF_CODEC_CHOICE_RAV1E: avifCodecChoice = 4;
pub const AVIF_CODEC_CHOICE_LIBGAV1: avifCodecChoice = 3;
pub const AVIF_CODEC_CHOICE_DAV1D: avifCodecChoice = 2;
pub const AVIF_CODEC_CHOICE_AOM: avifCodecChoice = 1;
pub const AVIF_CODEC_CHOICE_AUTO: avifCodecChoice = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifIOStats {
    pub colorOBUSize: size_t,
    pub alphaOBUSize: size_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecSpecificOptions {
    pub entries: *mut avifCodecSpecificOption,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecSpecificOption {
    pub key: *mut core::ffi::c_char,
    pub value: *mut core::ffi::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifScalingMode {
    pub horizontal: avifFraction,
    pub vertical: avifFraction,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncoder {
    pub codecChoice: avifCodecChoice,
    pub maxThreads: core::ffi::c_int,
    pub speed: core::ffi::c_int,
    pub keyframeInterval: core::ffi::c_int,
    pub timescale: uint64_t,
    pub repetitionCount: core::ffi::c_int,
    pub extraLayerCount: uint32_t,
    pub quality: core::ffi::c_int,
    pub qualityAlpha: core::ffi::c_int,
    pub minQuantizer: core::ffi::c_int,
    pub maxQuantizer: core::ffi::c_int,
    pub minQuantizerAlpha: core::ffi::c_int,
    pub maxQuantizerAlpha: core::ffi::c_int,
    pub tileRowsLog2: core::ffi::c_int,
    pub tileColsLog2: core::ffi::c_int,
    pub autoTiling: avifBool,
    pub scalingMode: avifScalingMode,
    pub ioStats: avifIOStats,
    pub diag: avifDiagnostics,
    pub data: *mut avifEncoderData,
    pub csOptions: *mut avifCodecSpecificOptions,
    pub headerFormat: avifHeaderFormatFlags,
    pub qualityGainMap: core::ffi::c_int,
    pub creationTime: uint64_t,
    pub modificationTime: uint64_t,
    pub sampleTransformRecipe: avifSampleTransformRecipe,
    pub width: uint32_t,
    pub height: uint32_t,
}
pub type avifAddImageFlags = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifDecodeSample {
    pub data: avifROData,
    pub ownsData: avifBool,
    pub partialData: avifBool,
    pub itemID: uint32_t,
    pub offset: uint64_t,
    pub size: size_t,
    pub spatialID: uint8_t,
    pub sync: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncodeSample {
    pub data: avifRWData,
    pub sync: avifBool,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifEncodeSampleArray {
    pub sample: *mut avifEncodeSample,
    pub elementSize: uint32_t,
    pub count: uint32_t,
    pub capacity: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecEncodeOutput {
    pub samples: avifEncodeSampleArray,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodec {
    pub csOptions: *const avifCodecSpecificOptions,
    pub internal: *mut avifCodecInternal,
    pub diag: *mut avifDiagnostics,
    pub maxThreads: core::ffi::c_int,
    pub imageSizeLimit: uint32_t,
    pub imageDimensionLimit: uint32_t,
    pub operatingPoint: uint8_t,
    pub allLayers: avifBool,
    pub getNextImage: avifCodecGetNextImageFunc,
    pub encodeImage: avifCodecEncodeImageFunc,
    pub encodeFinish: avifCodecEncodeFinishFunc,
    pub destroyInternal: avifCodecDestroyInternalFunc,
}
pub type avifCodecDestroyInternalFunc = Option<
    unsafe extern "C" fn(*mut avifCodec) -> (),
>;
pub type avifCodecEncodeFinishFunc = Option<
    unsafe extern "C" fn(*mut avifCodec, *mut avifCodecEncodeOutput) -> avifBool,
>;
pub type avifCodecEncodeImageFunc = Option<
    unsafe extern "C" fn(
        *mut avifCodec,
        *mut avifEncoder,
        *const avifImage,
        avifBool,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        avifEncoderChanges,
        avifBool,
        avifAddImageFlags,
        *mut avifCodecEncodeOutput,
    ) -> avifResult,
>;
pub type avifEncoderChanges = core::ffi::c_int;
pub type avifCodecGetNextImageFunc = Option<
    unsafe extern "C" fn(
        *mut avifCodec,
        *const avifDecodeSample,
        avifBool,
        *mut avifBool,
        *mut avifImage,
    ) -> avifBool,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct avifCodecInternal {
    pub gav1Settings: Libgav1DecoderSettings,
    pub gav1Decoder: *mut Libgav1Decoder,
    pub gav1Image: *const Libgav1DecoderBuffer,
    pub colorRange: avifRange,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Libgav1DecoderBuffer {
    pub chroma_sample_position: Libgav1ChromaSamplePosition,
    pub image_format: Libgav1ImageFormat,
    pub color_range: Libgav1ColorRange,
    pub color_primary: Libgav1ColorPrimary,
    pub transfer_characteristics: Libgav1TransferCharacteristics,
    pub matrix_coefficients: Libgav1MatrixCoefficients,
    pub bitdepth: core::ffi::c_int,
    pub displayed_width: [core::ffi::c_int; 3],
    pub displayed_height: [core::ffi::c_int; 3],
    pub stride: [core::ffi::c_int; 3],
    pub plane: [*mut uint8_t; 3],
    pub spatial_id: core::ffi::c_int,
    pub temporal_id: core::ffi::c_int,
    pub hdr_cll: Libgav1ObuMetadataHdrCll,
    pub has_hdr_cll: core::ffi::c_int,
    pub hdr_mdcv: Libgav1ObuMetadataHdrMdcv,
    pub has_hdr_mdcv: core::ffi::c_int,
    pub itut_t35: Libgav1ObuMetadataItutT35,
    pub has_itut_t35: core::ffi::c_int,
    pub user_private_data: int64_t,
    pub buffer_private_data: *mut core::ffi::c_void,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Libgav1ObuMetadataItutT35 {
    pub country_code: uint8_t,
    pub country_code_extension_byte: uint8_t,
    pub payload_bytes: *mut uint8_t,
    pub payload_size: core::ffi::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Libgav1ObuMetadataHdrMdcv {
    pub primary_chromaticity_x: [uint16_t; 3],
    pub primary_chromaticity_y: [uint16_t; 3],
    pub white_point_chromaticity_x: uint16_t,
    pub white_point_chromaticity_y: uint16_t,
    pub luminance_max: uint32_t,
    pub luminance_min: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Libgav1ObuMetadataHdrCll {
    pub max_cll: uint16_t,
    pub max_fall: uint16_t,
}
pub type Libgav1MatrixCoefficients = core::ffi::c_uint;
pub const kLibgav1MaxMatrixCoefficients: Libgav1MatrixCoefficients = 255;
pub const kLibgav1MatrixCoefficientsIctcp: Libgav1MatrixCoefficients = 14;
pub const kLibgav1MatrixCoefficientsChromatCl: Libgav1MatrixCoefficients = 13;
pub const kLibgav1MatrixCoefficientsChromatNcl: Libgav1MatrixCoefficients = 12;
pub const kLibgav1MatrixCoefficientsSmpte2085: Libgav1MatrixCoefficients = 11;
pub const kLibgav1MatrixCoefficientsBt2020Cl: Libgav1MatrixCoefficients = 10;
pub const kLibgav1MatrixCoefficientsBt2020Ncl: Libgav1MatrixCoefficients = 9;
pub const kLibgav1MatrixCoefficientsSmpteYcgco: Libgav1MatrixCoefficients = 8;
pub const kLibgav1MatrixCoefficientsSmpte240: Libgav1MatrixCoefficients = 7;
pub const kLibgav1MatrixCoefficientsBt601: Libgav1MatrixCoefficients = 6;
pub const kLibgav1MatrixCoefficientsBt470BG: Libgav1MatrixCoefficients = 5;
pub const kLibgav1MatrixCoefficientsFcc: Libgav1MatrixCoefficients = 4;
pub const kLibgav1MatrixCoefficientsUnspecified: Libgav1MatrixCoefficients = 2;
pub const kLibgav1MatrixCoefficientsBt709: Libgav1MatrixCoefficients = 1;
pub const kLibgav1MatrixCoefficientsIdentity: Libgav1MatrixCoefficients = 0;
pub type Libgav1TransferCharacteristics = core::ffi::c_uint;
pub const kLibgav1MaxTransferCharacteristics: Libgav1TransferCharacteristics = 255;
pub const kLibgav1TransferCharacteristicsHlg: Libgav1TransferCharacteristics = 18;
pub const kLibgav1TransferCharacteristicsSmpte428: Libgav1TransferCharacteristics = 17;
pub const kLibgav1TransferCharacteristicsSmpte2084: Libgav1TransferCharacteristics = 16;
pub const kLibgav1TransferCharacteristicsBt2020TwelveBit: Libgav1TransferCharacteristics = 15;
pub const kLibgav1TransferCharacteristicsBt2020TenBit: Libgav1TransferCharacteristics = 14;
pub const kLibgav1TransferCharacteristicsSrgb: Libgav1TransferCharacteristics = 13;
pub const kLibgav1TransferCharacteristicsBt1361: Libgav1TransferCharacteristics = 12;
pub const kLibgav1TransferCharacteristicsIec61966: Libgav1TransferCharacteristics = 11;
pub const kLibgav1TransferCharacteristicsLog100Sqrt10: Libgav1TransferCharacteristics = 10;
pub const kLibgav1TransferCharacteristicsLog100: Libgav1TransferCharacteristics = 9;
pub const kLibgav1TransferCharacteristicsLinear: Libgav1TransferCharacteristics = 8;
pub const kLibgav1TransferCharacteristicsSmpte240: Libgav1TransferCharacteristics = 7;
pub const kLibgav1TransferCharacteristicsBt601: Libgav1TransferCharacteristics = 6;
pub const kLibgav1TransferCharacteristicsBt470Bg: Libgav1TransferCharacteristics = 5;
pub const kLibgav1TransferCharacteristicsBt470M: Libgav1TransferCharacteristics = 4;
pub const kLibgav1TransferCharacteristicsUnspecified: Libgav1TransferCharacteristics = 2;
pub const kLibgav1TransferCharacteristicsBt709: Libgav1TransferCharacteristics = 1;
pub type Libgav1ColorPrimary = core::ffi::c_uint;
pub const kLibgav1MaxColorPrimaries: Libgav1ColorPrimary = 255;
pub const kLibgav1ColorPrimaryEbu3213: Libgav1ColorPrimary = 22;
pub const kLibgav1ColorPrimarySmpte432: Libgav1ColorPrimary = 12;
pub const kLibgav1ColorPrimarySmpte431: Libgav1ColorPrimary = 11;
pub const kLibgav1ColorPrimaryXyz: Libgav1ColorPrimary = 10;
pub const kLibgav1ColorPrimaryBt2020: Libgav1ColorPrimary = 9;
pub const kLibgav1ColorPrimaryGenericFilm: Libgav1ColorPrimary = 8;
pub const kLibgav1ColorPrimarySmpte240: Libgav1ColorPrimary = 7;
pub const kLibgav1ColorPrimaryBt601: Libgav1ColorPrimary = 6;
pub const kLibgav1ColorPrimaryBt470Bg: Libgav1ColorPrimary = 5;
pub const kLibgav1ColorPrimaryBt470M: Libgav1ColorPrimary = 4;
pub const kLibgav1ColorPrimaryUnspecified: Libgav1ColorPrimary = 2;
pub const kLibgav1ColorPrimaryBt709: Libgav1ColorPrimary = 1;
pub type Libgav1ColorRange = core::ffi::c_uint;
pub const kLibgav1ColorRangeFull: Libgav1ColorRange = 1;
pub const kLibgav1ColorRangeStudio: Libgav1ColorRange = 0;
pub type Libgav1ImageFormat = core::ffi::c_uint;
pub const kLibgav1ImageFormatMonochrome400: Libgav1ImageFormat = 3;
pub const kLibgav1ImageFormatYuv444: Libgav1ImageFormat = 2;
pub const kLibgav1ImageFormatYuv422: Libgav1ImageFormat = 1;
pub const kLibgav1ImageFormatYuv420: Libgav1ImageFormat = 0;
pub type Libgav1ChromaSamplePosition = core::ffi::c_uint;
pub const kLibgav1ChromaSamplePositionReserved: Libgav1ChromaSamplePosition = 3;
pub const kLibgav1ChromaSamplePositionColocated: Libgav1ChromaSamplePosition = 2;
pub const kLibgav1ChromaSamplePositionVertical: Libgav1ChromaSamplePosition = 1;
pub const kLibgav1ChromaSamplePositionUnknown: Libgav1ChromaSamplePosition = 0;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Libgav1DecoderSettings {
    pub threads: core::ffi::c_int,
    pub frame_parallel: core::ffi::c_int,
    pub blocking_dequeue: core::ffi::c_int,
    pub on_frame_buffer_size_changed: Libgav1FrameBufferSizeChangedCallback,
    pub get_frame_buffer: Libgav1GetFrameBufferCallback,
    pub release_frame_buffer: Libgav1ReleaseFrameBufferCallback,
    pub release_input_buffer: Libgav1ReleaseInputBufferCallback,
    pub callback_private_data: *mut core::ffi::c_void,
    pub output_all_layers: core::ffi::c_int,
    pub operating_point: core::ffi::c_int,
    pub post_filter_mask: uint8_t,
    pub parse_only: core::ffi::c_int,
}
pub type Libgav1ReleaseInputBufferCallback = Option<
    unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> (),
>;
pub type Libgav1ReleaseFrameBufferCallback = Option<
    unsafe extern "C" fn(*mut core::ffi::c_void, *mut core::ffi::c_void) -> (),
>;
pub type Libgav1GetFrameBufferCallback = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        Libgav1ImageFormat,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        *mut Libgav1FrameBuffer,
    ) -> Libgav1StatusCode,
>;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Libgav1FrameBuffer {
    pub plane: [*mut uint8_t; 3],
    pub stride: [core::ffi::c_int; 3],
    pub private_data: *mut core::ffi::c_void,
}
pub type Libgav1StatusCode = core::ffi::c_int;
pub const kLibgav1StatusReservedForFutureExpansionUseDefaultInSwitchInstead_: Libgav1StatusCode = -1000;
pub const kLibgav1StatusNothingToDequeue: Libgav1StatusCode = -11;
pub const kLibgav1StatusTryAgain: Libgav1StatusCode = -10;
pub const kLibgav1StatusBitstreamError: Libgav1StatusCode = -9;
pub const kLibgav1StatusInternalError: Libgav1StatusCode = -8;
pub const kLibgav1StatusUnimplemented: Libgav1StatusCode = -7;
pub const kLibgav1StatusAlready: Libgav1StatusCode = -6;
pub const kLibgav1StatusNotInitialized: Libgav1StatusCode = -5;
pub const kLibgav1StatusResourceExhausted: Libgav1StatusCode = -4;
pub const kLibgav1StatusOutOfMemory: Libgav1StatusCode = -3;
pub const kLibgav1StatusInvalidArgument: Libgav1StatusCode = -2;
pub const kLibgav1StatusUnknownError: Libgav1StatusCode = -1;
pub const kLibgav1StatusOk: Libgav1StatusCode = 0;
pub type Libgav1FrameBufferSizeChangedCallback = Option<
    unsafe extern "C" fn(
        *mut core::ffi::c_void,
        core::ffi::c_int,
        Libgav1ImageFormat,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
        core::ffi::c_int,
    ) -> Libgav1StatusCode,
>;
pub const NULL: *mut core::ffi::c_void = 0 as *mut core::ffi::c_void;
pub const AVIF_TRUE: core::ffi::c_int = 1 as core::ffi::c_int;
pub const AVIF_FALSE: core::ffi::c_int = 0 as core::ffi::c_int;
pub const AVIF_SPATIAL_ID_UNSET: core::ffi::c_int = 0xff as core::ffi::c_int;
unsafe extern "C" fn gav1CodecDestroyInternal(mut codec: *mut avifCodec) {
    if !((*(*codec).internal).gav1Decoder).is_null() {
        Libgav1DecoderDestroy((*(*codec).internal).gav1Decoder);
    }
    avifFree((*codec).internal as *mut core::ffi::c_void);
}
unsafe extern "C" fn gav1CodecGetNextImage(
    mut codec: *mut avifCodec,
    mut sample: *const avifDecodeSample,
    mut alpha: avifBool,
    mut isLimitedRangeAlpha: *mut avifBool,
    mut image: *mut avifImage,
) -> avifBool {
    if ((*(*codec).internal).gav1Decoder).is_null() {
        (*(*codec).internal).gav1Settings.threads = (*codec).maxThreads;
        (*(*codec).internal).gav1Settings.operating_point = (*codec).operatingPoint
            as core::ffi::c_int;
        (*(*codec).internal).gav1Settings.output_all_layers = (*codec).allLayers
            as core::ffi::c_int;
        if Libgav1DecoderCreate(
            &mut (*(*codec).internal).gav1Settings,
            &mut (*(*codec).internal).gav1Decoder,
        ) as core::ffi::c_int != kLibgav1StatusOk as core::ffi::c_int
        {
            return AVIF_FALSE;
        }
    }
    if Libgav1DecoderEnqueueFrame(
        (*(*codec).internal).gav1Decoder,
        (*sample).data.data,
        (*sample).data.size,
        0 as int64_t,
        NULL,
    ) as core::ffi::c_int != kLibgav1StatusOk as core::ffi::c_int
    {
        return AVIF_FALSE;
    }
    (*(*codec).internal).gav1Image = 0 as *const Libgav1DecoderBuffer;
    let mut nextFrame: *const Libgav1DecoderBuffer = 0 as *const Libgav1DecoderBuffer;
    loop {
        if Libgav1DecoderDequeueFrame((*(*codec).internal).gav1Decoder, &mut nextFrame)
            as core::ffi::c_int != kLibgav1StatusOk as core::ffi::c_int
        {
            return AVIF_FALSE;
        }
        if !(!nextFrame.is_null()
            && (*sample).spatialID as core::ffi::c_int != AVIF_SPATIAL_ID_UNSET
            && (*nextFrame).spatial_id != (*sample).spatialID as core::ffi::c_int)
        {
            break;
        }
        nextFrame = 0 as *const Libgav1DecoderBuffer;
    }
    if !nextFrame.is_null() {
        (*(*codec).internal).gav1Image = nextFrame;
        (*(*codec).internal).colorRange = (if (*nextFrame).color_range
            as core::ffi::c_uint
            == kLibgav1ColorRangeStudio as core::ffi::c_int as core::ffi::c_uint
        {
            AVIF_RANGE_LIMITED as core::ffi::c_int
        } else {
            AVIF_RANGE_FULL as core::ffi::c_int
        }) as avifRange;
    } else if alpha != 0 && !((*(*codec).internal).gav1Image).is_null() {} else {
        return AVIF_FALSE
    }
    let mut gav1Image: *const Libgav1DecoderBuffer = (*(*codec).internal).gav1Image;
    let mut isColor: avifBool = (alpha == 0) as core::ffi::c_int;
    if isColor != 0 {
        let mut yuvFormat: avifPixelFormat = AVIF_PIXEL_FORMAT_NONE;
        match (*gav1Image).image_format as core::ffi::c_uint {
            3 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV400;
            }
            0 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV420;
            }
            1 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV422;
            }
            2 => {
                yuvFormat = AVIF_PIXEL_FORMAT_YUV444;
            }
            _ => {}
        }
        (*image).width = (*gav1Image).displayed_width[0 as core::ffi::c_int as usize]
            as uint32_t;
        (*image).height = (*gav1Image).displayed_height[0 as core::ffi::c_int as usize]
            as uint32_t;
        (*image).depth = (*gav1Image).bitdepth as uint32_t;
        (*image).yuvFormat = yuvFormat;
        (*image).yuvRange = (*(*codec).internal).colorRange;
        (*image).yuvChromaSamplePosition = (*gav1Image).chroma_sample_position
            as avifChromaSamplePosition;
        (*image).colorPrimaries = (*gav1Image).color_primary as avifColorPrimaries;
        (*image).transferCharacteristics = (*gav1Image).transfer_characteristics
            as avifTransferCharacteristics;
        (*image).matrixCoefficients = (*gav1Image).matrix_coefficients
            as avifMatrixCoefficients;
        avifImageFreePlanes(
            image,
            AVIF_PLANES_YUV as core::ffi::c_int as avifPlanesFlags,
        );
        let mut yuvPlaneCount: core::ffi::c_int = if yuvFormat as core::ffi::c_uint
            == AVIF_PIXEL_FORMAT_YUV400 as core::ffi::c_int as core::ffi::c_uint
        {
            1 as core::ffi::c_int
        } else {
            3 as core::ffi::c_int
        };
        let mut yuvPlane: core::ffi::c_int = 0 as core::ffi::c_int;
        while yuvPlane < yuvPlaneCount {
            (*image).yuvPlanes[yuvPlane as usize] = (*gav1Image)
                .plane[yuvPlane as usize];
            (*image).yuvRowBytes[yuvPlane as usize] = (*gav1Image)
                .stride[yuvPlane as usize] as uint32_t;
            yuvPlane += 1;
        }
        (*image).imageOwnsYUVPlanes = AVIF_FALSE as avifBool;
    } else {
        (*image).width = (*gav1Image).displayed_width[0 as core::ffi::c_int as usize]
            as uint32_t;
        (*image).height = (*gav1Image).displayed_height[0 as core::ffi::c_int as usize]
            as uint32_t;
        (*image).depth = (*gav1Image).bitdepth as uint32_t;
        avifImageFreePlanes(image, AVIF_PLANES_A as core::ffi::c_int as avifPlanesFlags);
        (*image).alphaPlane = (*gav1Image).plane[0 as core::ffi::c_int as usize];
        (*image).alphaRowBytes = (*gav1Image).stride[0 as core::ffi::c_int as usize]
            as uint32_t;
        *isLimitedRangeAlpha = ((*(*codec).internal).colorRange as core::ffi::c_uint
            == AVIF_RANGE_LIMITED as core::ffi::c_int as core::ffi::c_uint)
            as core::ffi::c_int as avifBool;
        (*image).imageOwnsAlphaPlane = AVIF_FALSE as avifBool;
    }
    return AVIF_TRUE;
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecVersionGav1() -> *const core::ffi::c_char {
    return Libgav1GetVersionString();
}
#[no_mangle]
pub unsafe extern "C" fn avifCodecCreateGav1() -> *mut avifCodec {
    let mut codec: *mut avifCodec = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifCodec>() as size_t,
    ) as *mut avifCodec;
    if codec.is_null() {
        return 0 as *mut avifCodec;
    }
    (*codec).getNextImage = Some(
        gav1CodecGetNextImage
            as unsafe extern "C" fn(
                *mut avifCodec,
                *const avifDecodeSample,
                avifBool,
                *mut avifBool,
                *mut avifImage,
            ) -> avifBool,
    ) as avifCodecGetNextImageFunc;
    (*codec).destroyInternal = Some(
        gav1CodecDestroyInternal as unsafe extern "C" fn(*mut avifCodec) -> (),
    ) as avifCodecDestroyInternalFunc;
    (*codec).internal = avifCalloc(
        1 as size_t,
        ::core::mem::size_of::<avifCodecInternal>() as size_t,
    ) as *mut avifCodecInternal;
    if ((*codec).internal).is_null() {
        avifFree(codec as *mut core::ffi::c_void);
        return 0 as *mut avifCodec;
    }
    Libgav1DecoderSettingsInitDefault(&mut (*(*codec).internal).gav1Settings);
    return codec;
}
