//! WP1 differential test: the constants, enumerators, predicates and macros of
//! the WP1 headers, compared with the C values from `tools/oracle/probes/wp1/
//! constants.c`, and the layout of the public `repr(C)` structs (ADR 0003),
//! compared with `tools/oracle/probes/wp1/struct_layout.c`.
//!
//! Each expectation is written with the C name, in the order the probe prints it,
//! so that a mismatch names the constant.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};

use webp::utils::random_utils::{VP8_RANDOM_DITHER_FIX, VP8_RANDOM_TABLE_SIZE};
use webp::utils::thread_utils::WebPWorkerStatus;
use webp::utils::utils::WEBP_ALIGN_CST;
use webp::utils::utils::WEBP_MAX_ALLOCABLE_MEMORY;
use webp::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WEBP_DECODER_ABI_VERSION, WebPBitstreamFeatures,
    WebPDecoderOptions, webp_is_alpha_mode, webp_is_premultiplied_mode, webp_is_rgb_mode,
};
use webp::webp::demux::{
    WEBP_DEMUX_ABI_VERSION, WebPAnimDecoderOptions, WebPAnimInfo, WebPDemuxState, WebPFormatFeature,
};
use webp::webp::encode::{
    WEBP_ENCODER_ABI_VERSION, WEBP_MAX_DIMENSION, WebPAuxStats, WebPConfig, WebPEncCSP,
    WebPEncodingError, WebPImageHint, WebPPreset,
};
use webp::webp::format_constants::{
    ALPHA_HEADER_LEN, ALPHA_LOSSLESS_COMPRESSION, ALPHA_NO_COMPRESSION, ALPHA_PREPROCESSED_LEVELS,
    ANIM_CHUNK_SIZE, ANMF_CHUNK_SIZE, ARGB_BLACK, CHUNK_HEADER_SIZE, CHUNK_SIZE_BYTES,
    CODE_LENGTH_CODES, DEFAULT_CODE_LENGTH, HUFFMAN_CODES_PER_META_CODE, MAX_ALLOWED_CODE_LENGTH,
    MAX_CACHE_BITS, MAX_CANVAS_SIZE, MAX_CHUNK_PAYLOAD, MAX_DURATION, MAX_IMAGE_AREA,
    MAX_LOOP_COUNT, MAX_PALETTE_SIZE, MAX_POSITION_OFFSET, MIN_HUFFMAN_BITS, MIN_TRANSFORM_BITS,
    NUM_DISTANCE_CODES, NUM_HUFFMAN_BITS, NUM_LENGTH_CODES, NUM_LITERAL_CODES, NUM_TRANSFORM_BITS,
    NUM_TRANSFORMS, RIFF_HEADER_SIZE, TAG_SIZE, TRANSFORM_PRESENT, VP8_FRAME_HEADER_SIZE,
    VP8_MAX_PARTITION_SIZE, VP8_MAX_PARTITION0_SIZE, VP8_SIGNATURE, VP8L_FRAME_HEADER_SIZE,
    VP8L_IMAGE_SIZE_BITS, VP8L_MAGIC_BYTE, VP8L_SIGNATURE_SIZE, VP8L_VERSION, VP8L_VERSION_BITS,
    VP8X_CHUNK_SIZE, mkfourcc,
};
use webp::webp::mux_types::{WebPFeatureFlags, WebPMuxAnimBlend, WebPMuxAnimDispose};

/// Expected values in the order the probe prints them.
fn expected_constants() -> Fields {
    let mut f = Fields::new();
    macro_rules! put {
        ($($name:literal = $value:expr),* $(,)?) => {
            $( f = f.with($name, $value); )*
        };
    }
    // Decoder.
    put!(
        "WEBP_DECODER_ABI_VERSION" = WEBP_DECODER_ABI_VERSION,
        "VP8_STATUS_OK" = VP8StatusCode::VP8_STATUS_OK.0,
        "VP8_STATUS_OUT_OF_MEMORY" = VP8StatusCode::VP8_STATUS_OUT_OF_MEMORY.0,
        "VP8_STATUS_INVALID_PARAM" = VP8StatusCode::VP8_STATUS_INVALID_PARAM.0,
        "VP8_STATUS_BITSTREAM_ERROR" = VP8StatusCode::VP8_STATUS_BITSTREAM_ERROR.0,
        "VP8_STATUS_UNSUPPORTED_FEATURE" = VP8StatusCode::VP8_STATUS_UNSUPPORTED_FEATURE.0,
        "VP8_STATUS_SUSPENDED" = VP8StatusCode::VP8_STATUS_SUSPENDED.0,
        "VP8_STATUS_USER_ABORT" = VP8StatusCode::VP8_STATUS_USER_ABORT.0,
        "VP8_STATUS_NOT_ENOUGH_DATA" = VP8StatusCode::VP8_STATUS_NOT_ENOUGH_DATA.0,
        "MODE_RGB" = WEBP_CSP_MODE::MODE_RGB.0,
        "MODE_RGBA" = WEBP_CSP_MODE::MODE_RGBA.0,
        "MODE_BGR" = WEBP_CSP_MODE::MODE_BGR.0,
        "MODE_BGRA" = WEBP_CSP_MODE::MODE_BGRA.0,
        "MODE_ARGB" = WEBP_CSP_MODE::MODE_ARGB.0,
        "MODE_RGBA_4444" = WEBP_CSP_MODE::MODE_RGBA_4444.0,
        "MODE_RGB_565" = WEBP_CSP_MODE::MODE_RGB_565.0,
        "MODE_rgbA" = WEBP_CSP_MODE::MODE_rgbA.0,
        "MODE_bgrA" = WEBP_CSP_MODE::MODE_bgrA.0,
        "MODE_Argb" = WEBP_CSP_MODE::MODE_Argb.0,
        "MODE_rgbA_4444" = WEBP_CSP_MODE::MODE_rgbA_4444.0,
        "MODE_YUV" = WEBP_CSP_MODE::MODE_YUV.0,
        "MODE_YUVA" = WEBP_CSP_MODE::MODE_YUVA.0,
        "MODE_LAST" = WEBP_CSP_MODE::MODE_LAST.0,
    );
    for mode in 0..WEBP_CSP_MODE::MODE_LAST.0 {
        let csp = WEBP_CSP_MODE(mode);
        f = f
            .with(
                &format!("WebPIsPremultipliedMode.{mode}"),
                i32::from(webp_is_premultiplied_mode(csp)),
            )
            .with(
                &format!("WebPIsAlphaMode.{mode}"),
                i32::from(webp_is_alpha_mode(csp)),
            )
            .with(
                &format!("WebPIsRGBMode.{mode}"),
                i32::from(webp_is_rgb_mode(csp)),
            );
    }

    // Encoder.
    put!(
        "WEBP_ENCODER_ABI_VERSION" = WEBP_ENCODER_ABI_VERSION,
        "WEBP_MAX_DIMENSION" = WEBP_MAX_DIMENSION,
        "WEBP_HINT_DEFAULT" = WebPImageHint::WEBP_HINT_DEFAULT.0,
        "WEBP_HINT_PICTURE" = WebPImageHint::WEBP_HINT_PICTURE.0,
        "WEBP_HINT_PHOTO" = WebPImageHint::WEBP_HINT_PHOTO.0,
        "WEBP_HINT_GRAPH" = WebPImageHint::WEBP_HINT_GRAPH.0,
        "WEBP_HINT_LAST" = WebPImageHint::WEBP_HINT_LAST.0,
        "WEBP_PRESET_DEFAULT" = WebPPreset::WEBP_PRESET_DEFAULT.0,
        "WEBP_PRESET_PICTURE" = WebPPreset::WEBP_PRESET_PICTURE.0,
        "WEBP_PRESET_PHOTO" = WebPPreset::WEBP_PRESET_PHOTO.0,
        "WEBP_PRESET_DRAWING" = WebPPreset::WEBP_PRESET_DRAWING.0,
        "WEBP_PRESET_ICON" = WebPPreset::WEBP_PRESET_ICON.0,
        "WEBP_PRESET_TEXT" = WebPPreset::WEBP_PRESET_TEXT.0,
        "WEBP_YUV420" = WebPEncCSP::WEBP_YUV420.0,
        "WEBP_YUV420A" = WebPEncCSP::WEBP_YUV420A.0,
        "WEBP_CSP_UV_MASK" = WebPEncCSP::WEBP_CSP_UV_MASK.0,
        "WEBP_CSP_ALPHA_BIT" = WebPEncCSP::WEBP_CSP_ALPHA_BIT.0,
        "VP8_ENC_OK" = WebPEncodingError::VP8_ENC_OK.0,
        "VP8_ENC_ERROR_OUT_OF_MEMORY" = WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY.0,
        "VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY" =
            WebPEncodingError::VP8_ENC_ERROR_BITSTREAM_OUT_OF_MEMORY.0,
        "VP8_ENC_ERROR_NULL_PARAMETER" = WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER.0,
        "VP8_ENC_ERROR_INVALID_CONFIGURATION" =
            WebPEncodingError::VP8_ENC_ERROR_INVALID_CONFIGURATION.0,
        "VP8_ENC_ERROR_BAD_DIMENSION" = WebPEncodingError::VP8_ENC_ERROR_BAD_DIMENSION.0,
        "VP8_ENC_ERROR_PARTITION0_OVERFLOW" =
            WebPEncodingError::VP8_ENC_ERROR_PARTITION0_OVERFLOW.0,
        "VP8_ENC_ERROR_PARTITION_OVERFLOW" = WebPEncodingError::VP8_ENC_ERROR_PARTITION_OVERFLOW.0,
        "VP8_ENC_ERROR_BAD_WRITE" = WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE.0,
        "VP8_ENC_ERROR_FILE_TOO_BIG" = WebPEncodingError::VP8_ENC_ERROR_FILE_TOO_BIG.0,
        "VP8_ENC_ERROR_USER_ABORT" = WebPEncodingError::VP8_ENC_ERROR_USER_ABORT.0,
        "VP8_ENC_ERROR_LAST" = WebPEncodingError::VP8_ENC_ERROR_LAST.0,
    );

    // Demuxer and muxer types.
    put!(
        "WEBP_DEMUX_ABI_VERSION" = WEBP_DEMUX_ABI_VERSION,
        "WEBP_DEMUX_PARSE_ERROR" = WebPDemuxState::WEBP_DEMUX_PARSE_ERROR.0,
        "WEBP_DEMUX_PARSING_HEADER" = WebPDemuxState::WEBP_DEMUX_PARSING_HEADER.0,
        "WEBP_DEMUX_PARSED_HEADER" = WebPDemuxState::WEBP_DEMUX_PARSED_HEADER.0,
        "WEBP_DEMUX_DONE" = WebPDemuxState::WEBP_DEMUX_DONE.0,
        "WEBP_FF_FORMAT_FLAGS" = WebPFormatFeature::WEBP_FF_FORMAT_FLAGS.0,
        "WEBP_FF_CANVAS_WIDTH" = WebPFormatFeature::WEBP_FF_CANVAS_WIDTH.0,
        "WEBP_FF_CANVAS_HEIGHT" = WebPFormatFeature::WEBP_FF_CANVAS_HEIGHT.0,
        "WEBP_FF_LOOP_COUNT" = WebPFormatFeature::WEBP_FF_LOOP_COUNT.0,
        "WEBP_FF_BACKGROUND_COLOR" = WebPFormatFeature::WEBP_FF_BACKGROUND_COLOR.0,
        "WEBP_FF_FRAME_COUNT" = WebPFormatFeature::WEBP_FF_FRAME_COUNT.0,
        "ANIMATION_FLAG" = WebPFeatureFlags::ANIMATION_FLAG.0,
        "XMP_FLAG" = WebPFeatureFlags::XMP_FLAG.0,
        "EXIF_FLAG" = WebPFeatureFlags::EXIF_FLAG.0,
        "ALPHA_FLAG" = WebPFeatureFlags::ALPHA_FLAG.0,
        "ICCP_FLAG" = WebPFeatureFlags::ICCP_FLAG.0,
        "ALL_VALID_FLAGS" = WebPFeatureFlags::ALL_VALID_FLAGS.0,
        "WEBP_MUX_DISPOSE_NONE" = WebPMuxAnimDispose::WEBP_MUX_DISPOSE_NONE.0,
        "WEBP_MUX_DISPOSE_BACKGROUND" = WebPMuxAnimDispose::WEBP_MUX_DISPOSE_BACKGROUND.0,
        "WEBP_MUX_BLEND" = WebPMuxAnimBlend::WEBP_MUX_BLEND.0,
        "WEBP_MUX_NO_BLEND" = WebPMuxAnimBlend::WEBP_MUX_NO_BLEND.0,
    );

    // Format constants.
    put!(
        "VP8_SIGNATURE" = VP8_SIGNATURE,
        "VP8_MAX_PARTITION0_SIZE" = VP8_MAX_PARTITION0_SIZE,
        "VP8_MAX_PARTITION_SIZE" = VP8_MAX_PARTITION_SIZE,
        "VP8_FRAME_HEADER_SIZE" = VP8_FRAME_HEADER_SIZE,
        "VP8L_SIGNATURE_SIZE" = VP8L_SIGNATURE_SIZE,
        "VP8L_MAGIC_BYTE" = VP8L_MAGIC_BYTE,
        "VP8L_IMAGE_SIZE_BITS" = VP8L_IMAGE_SIZE_BITS,
        "VP8L_VERSION_BITS" = VP8L_VERSION_BITS,
        "VP8L_VERSION" = VP8L_VERSION,
        "VP8L_FRAME_HEADER_SIZE" = VP8L_FRAME_HEADER_SIZE,
        "MAX_PALETTE_SIZE" = MAX_PALETTE_SIZE,
        "MAX_CACHE_BITS" = MAX_CACHE_BITS,
        "HUFFMAN_CODES_PER_META_CODE" = HUFFMAN_CODES_PER_META_CODE,
        "ARGB_BLACK" = ARGB_BLACK,
        "DEFAULT_CODE_LENGTH" = DEFAULT_CODE_LENGTH,
        "MAX_ALLOWED_CODE_LENGTH" = MAX_ALLOWED_CODE_LENGTH,
        "NUM_LITERAL_CODES" = NUM_LITERAL_CODES,
        "NUM_LENGTH_CODES" = NUM_LENGTH_CODES,
        "NUM_DISTANCE_CODES" = NUM_DISTANCE_CODES,
        "CODE_LENGTH_CODES" = CODE_LENGTH_CODES,
        "MIN_HUFFMAN_BITS" = MIN_HUFFMAN_BITS,
        "NUM_HUFFMAN_BITS" = NUM_HUFFMAN_BITS,
        "MIN_TRANSFORM_BITS" = MIN_TRANSFORM_BITS,
        "NUM_TRANSFORM_BITS" = NUM_TRANSFORM_BITS,
        "TRANSFORM_PRESENT" = TRANSFORM_PRESENT,
        "NUM_TRANSFORMS" = NUM_TRANSFORMS,
        "PREDICTOR_TRANSFORM" =
            webp::webp::format_constants::VP8LImageTransformType::PREDICTOR_TRANSFORM.0,
        "CROSS_COLOR_TRANSFORM" =
            webp::webp::format_constants::VP8LImageTransformType::CROSS_COLOR_TRANSFORM.0,
        "SUBTRACT_GREEN_TRANSFORM" =
            webp::webp::format_constants::VP8LImageTransformType::SUBTRACT_GREEN_TRANSFORM.0,
        "COLOR_INDEXING_TRANSFORM" =
            webp::webp::format_constants::VP8LImageTransformType::COLOR_INDEXING_TRANSFORM.0,
        "ALPHA_HEADER_LEN" = ALPHA_HEADER_LEN,
        "ALPHA_NO_COMPRESSION" = ALPHA_NO_COMPRESSION,
        "ALPHA_LOSSLESS_COMPRESSION" = ALPHA_LOSSLESS_COMPRESSION,
        "ALPHA_PREPROCESSED_LEVELS" = ALPHA_PREPROCESSED_LEVELS,
        "TAG_SIZE" = TAG_SIZE,
        "CHUNK_SIZE_BYTES" = CHUNK_SIZE_BYTES,
        "CHUNK_HEADER_SIZE" = CHUNK_HEADER_SIZE,
        "RIFF_HEADER_SIZE" = RIFF_HEADER_SIZE,
        "ANMF_CHUNK_SIZE" = ANMF_CHUNK_SIZE,
        "ANIM_CHUNK_SIZE" = ANIM_CHUNK_SIZE,
        "VP8X_CHUNK_SIZE" = VP8X_CHUNK_SIZE,
        "MAX_CANVAS_SIZE" = MAX_CANVAS_SIZE,
        "MAX_IMAGE_AREA" = MAX_IMAGE_AREA,
        "MAX_LOOP_COUNT" = MAX_LOOP_COUNT,
        "MAX_DURATION" = MAX_DURATION,
        "MAX_POSITION_OFFSET" = MAX_POSITION_OFFSET,
        "MAX_CHUNK_PAYLOAD" = MAX_CHUNK_PAYLOAD,
        "MKFOURCC.RIFF" = mkfourcc(b'R', b'I', b'F', b'F'),
        "MKFOURCC.VP8X" = mkfourcc(b'V', b'P', b'8', b'X'),
    );

    // Utilities, random generator and worker states.
    put!(
        "VP8_RANDOM_DITHER_FIX" = VP8_RANDOM_DITHER_FIX,
        "VP8_RANDOM_TABLE_SIZE" = VP8_RANDOM_TABLE_SIZE,
        "WEBP_MAX_ALLOCABLE_MEMORY" = WEBP_MAX_ALLOCABLE_MEMORY,
        "WEBP_ALIGN_CST" = WEBP_ALIGN_CST,
        "NOT_OK" = WebPWorkerStatus::NOT_OK.0,
        "OK" = WebPWorkerStatus::OK.0,
        "WORK" = WebPWorkerStatus::WORK.0,
    );
    // `WEBP_ABI_IS_INCOMPATIBLE` pairs, in the probe's order.
    use webp::webp::types::webp_abi_is_incompatible;
    put!(
        "WEBP_ABI_IS_INCOMPATIBLE.0x0210.0x0210" =
            i32::from(webp_abi_is_incompatible(0x0210, 0x0210)),
        "WEBP_ABI_IS_INCOMPATIBLE.0x0210.0x02ff" =
            i32::from(webp_abi_is_incompatible(0x0210, 0x02ff)),
        "WEBP_ABI_IS_INCOMPATIBLE.0x0210.0x0300" =
            i32::from(webp_abi_is_incompatible(0x0210, 0x0300)),
        "WEBP_ABI_IS_INCOMPATIBLE.0x0107.0x0109" =
            i32::from(webp_abi_is_incompatible(0x0107, 0x0109)),
        "WEBP_ABI_IS_INCOMPATIBLE.0x0100.0x0200" =
            i32::from(webp_abi_is_incompatible(0x0100, 0x0200)),
    );
    f
}

/// Expected layout, in the order the probe prints it.
fn expected_layout() -> Fields {
    let mut f = Fields::new();
    macro_rules! layout {
        ($name:literal, $ty:ty; $($cname:literal = $field:ident),* $(,)?) => {
            f = f
                .with(concat!($name, ".size"), std::mem::size_of::<$ty>())
                .with(concat!($name, ".align"), std::mem::align_of::<$ty>());
            $(
                f = f.with(
                    concat!($name, ".", $cname, ".offset"),
                    std::mem::offset_of!($ty, $field),
                );
            )*
        };
    }
    layout!("WebPBitstreamFeatures", WebPBitstreamFeatures;
        "width" = width, "height" = height, "has_alpha" = has_alpha,
        "has_animation" = has_animation, "format" = format, "pad" = pad);
    layout!("WebPDecoderOptions", WebPDecoderOptions;
        "bypass_filtering" = bypass_filtering, "no_fancy_upsampling" = no_fancy_upsampling,
        "use_cropping" = use_cropping, "crop_left" = crop_left, "crop_top" = crop_top,
        "crop_width" = crop_width, "crop_height" = crop_height, "use_scaling" = use_scaling,
        "scaled_width" = scaled_width, "scaled_height" = scaled_height,
        "use_threads" = use_threads, "dithering_strength" = dithering_strength,
        "flip" = flip, "alpha_dithering_strength" = alpha_dithering_strength, "pad" = pad);
    layout!("WebPConfig", WebPConfig;
        "lossless" = lossless, "quality" = quality, "method" = method,
        "image_hint" = image_hint, "target_size" = target_size, "target_PSNR" = target_psnr,
        "segments" = segments, "sns_strength" = sns_strength,
        "filter_strength" = filter_strength, "filter_sharpness" = filter_sharpness,
        "filter_type" = filter_type, "autofilter" = autofilter,
        "alpha_compression" = alpha_compression, "alpha_filtering" = alpha_filtering,
        "alpha_quality" = alpha_quality, "pass" = pass, "show_compressed" = show_compressed,
        "preprocessing" = preprocessing, "partitions" = partitions,
        "partition_limit" = partition_limit, "emulate_jpeg_size" = emulate_jpeg_size,
        "thread_level" = thread_level, "low_memory" = low_memory,
        "near_lossless" = near_lossless, "exact" = exact,
        "use_delta_palette" = use_delta_palette, "use_sharp_yuv" = use_sharp_yuv,
        "qmin" = qmin, "qmax" = qmax);
    layout!("WebPAuxStats", WebPAuxStats;
        "coded_size" = coded_size, "PSNR" = psnr, "block_count" = block_count,
        "header_bytes" = header_bytes, "residual_bytes" = residual_bytes,
        "segment_size" = segment_size, "segment_quant" = segment_quant,
        "segment_level" = segment_level, "alpha_data_size" = alpha_data_size,
        "layer_data_size" = layer_data_size, "lossless_features" = lossless_features,
        "histogram_bits" = histogram_bits, "transform_bits" = transform_bits,
        "cache_bits" = cache_bits, "palette_size" = palette_size,
        "lossless_size" = lossless_size, "lossless_hdr_size" = lossless_hdr_size,
        "lossless_data_size" = lossless_data_size,
        "cross_color_transform_bits" = cross_color_transform_bits, "pad" = pad);
    layout!("WebPAnimDecoderOptions", WebPAnimDecoderOptions;
        "color_mode" = color_mode, "use_threads" = use_threads, "padding" = padding);
    layout!("WebPAnimInfo", WebPAnimInfo;
        "canvas_width" = canvas_width, "canvas_height" = canvas_height,
        "loop_count" = loop_count, "bgcolor" = bgcolor, "frame_count" = frame_count,
        "pad" = pad);
    f
}

#[test]
fn constants_and_enumerators_match_the_c_headers() {
    let Some(oracle) = Oracle::locate("constants_and_enumerators_match_the_c_headers") else {
        return;
    };
    let probe = oracle.probe("wp1", "constants");
    let c_values = probe.run::<&str>(&[], &[]).success().fields();
    assert_eq!(c_values, expected_constants());
}

#[test]
fn public_struct_layout_matches_the_c_compiler() {
    let Some(oracle) = Oracle::locate("public_struct_layout_matches_the_c_compiler") else {
        return;
    };
    let probe = oracle.probe("wp1", "struct_layout");
    let c_layout = probe.run::<&str>(&[], &[]).success().fields();
    assert_eq!(c_layout, expected_layout());
}
