// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// The exported decoder C ABI. One C/C++ driver, compiled against the pinned
// public headers only, links either the original library (jxl_abi_original)
// or the Rust static library (jxl_abi_rust). Every public decoder history that
// the Rust port replays in-process also runs through both executables, and
// both must reproduce the original oracle's observation exactly: statuses,
// events, getters, buffers, output digests and allocator events. A C unit of
// the driver measures the layout of every public struct and the value of
// every public enumerator, and takes the address of every exported function.
use jxl::jxl::cms_interface::{JxlCmsInterface, JxlColorProfile};
use jxl::jxl::codestream_header::{
    JxlAnimationHeader, JxlBasicInfo, JxlBlendInfo, JxlBlendMode, JxlExtraChannelInfo,
    JxlExtraChannelType, JxlFrameHeader, JxlHeaderExtensions, JxlLayerInfo, JxlOrientation,
    JxlPreviewHeader,
};
use jxl::jxl::color_encoding::{
    ColorSpace, JxlColorEncoding, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};
use jxl::jxl::decode::{
    JPEGXL_MAJOR_VERSION, JPEGXL_MINOR_VERSION, JPEGXL_PATCH_VERSION, JxlBoxType,
    JxlColorProfileTarget, JxlDecoderStatus, JxlProgressiveDetail, JxlSignature,
};
use jxl::jxl::memory_manager::JxlMemoryManager;
use jxl::jxl::types::{JxlBitDepth, JxlBitDepthType, JxlDataType, JxlEndianness, JxlPixelFormat};
use std::fmt::Write as _;
use std::mem::{align_of, offset_of, size_of};
use std::path::PathBuf;

/// The functions that decode.h declares, without JxlDecoderSetParallelRunner,
/// whose export waits for the runner C ABI.
const EXPORTED_FUNCTIONS: usize = 51;

fn driver(variant: &str) -> PathBuf {
    super::oracle_path().with_file_name(format!("jxl_abi_{variant}"))
}

/// Runs the histories through the exported C ABI of both libraries and
/// requires the original oracle's observations.
pub(super) fn compare(family: &str, requests: &[String], originals: &[String]) {
    assert_eq!(requests.len(), originals.len());
    if requests.is_empty() {
        return;
    }
    for request in requests {
        assert!(
            request.starts_with("dpub-run ")
                && !request
                    .split(' ')
                    .any(|op| op.starts_with('@') || op.starts_with('#')),
            "the C ABI drivers replay runner-free public decoder histories: {request}"
        );
    }
    for variant in ["original", "rust"] {
        let observations = super::program_observations(&driver(variant), requests);
        for ((request, original), observed) in requests.iter().zip(originals).zip(&observations) {
            assert_eq!(
                observed, original,
                "{variant} C ABI differs from the original oracle: {request}"
            );
        }
    }
    println!(
        "{family}: {} histories matched through both C ABI libraries",
        requests.len()
    );
}

/// `compare` for the runner-free histories of a family: the C ABI does not
/// export JxlDecoderSetParallelRunner yet, so runner histories compare only
/// in-process.
pub(super) fn compare_runner_free(family: &str, requests: &[String], originals: &[String]) {
    assert_eq!(requests.len(), originals.len());
    let (requests, originals): (Vec<String>, Vec<String>) = requests
        .iter()
        .zip(originals)
        .filter(|(request, _)| {
            !request
                .split(' ')
                .any(|op| op.starts_with('@') || op.starts_with('#'))
        })
        .map(|(request, original)| (request.clone(), original.clone()))
        .unzip();
    compare(family, &requests, &originals);
}

/// The layout line of decoder_abi_layout.c, from the Rust definitions.
fn rust_layout() -> String {
    let mut out = String::new();
    macro_rules! s {
        ($name:literal, $ty:ty) => {
            write!(
                out,
                " S:{}:{}:{}",
                $name,
                size_of::<$ty>(),
                align_of::<$ty>()
            )
            .unwrap()
        };
    }
    macro_rules! o {
        ($name:literal, $ty:ty, $($field:ident).+) => {
            write!(out, " O:{}:{}", $name, offset_of!($ty, $($field).+)).unwrap()
        };
    }
    macro_rules! e {
        ($name:literal, $value:expr) => {
            write!(out, " E:{}:{}", $name, $value).unwrap()
        };
    }
    write!(
        out,
        "F:{EXPORTED_FUNCTIONS} V:{}:{JPEGXL_MAJOR_VERSION}.{JPEGXL_MINOR_VERSION}.{JPEGXL_PATCH_VERSION}",
        jxl::jxl::decode::jxl_decoder_version()
    )
    .unwrap();

    s!("JxlPixelFormat", JxlPixelFormat);
    o!("JxlPixelFormat.num_channels", JxlPixelFormat, num_channels);
    o!("JxlPixelFormat.data_type", JxlPixelFormat, data_type);
    o!("JxlPixelFormat.endianness", JxlPixelFormat, endianness);
    o!("JxlPixelFormat.align", JxlPixelFormat, align);
    s!("JxlBitDepth", JxlBitDepth);
    o!("JxlBitDepth.type", JxlBitDepth, depth_type);
    o!("JxlBitDepth.bits_per_sample", JxlBitDepth, bits_per_sample);
    o!(
        "JxlBitDepth.exponent_bits_per_sample",
        JxlBitDepth,
        exponent_bits_per_sample
    );
    s!("JxlBoxType", JxlBoxType);
    // JXL_BOOL is int.
    s!("JXL_BOOL", i32);
    s!("JxlDataType", JxlDataType);
    s!("JxlEndianness", JxlEndianness);
    s!("JxlBitDepthType", JxlBitDepthType);
    e!("JXL_TRUE", i32::from(true));
    e!("JXL_FALSE", i32::from(false));
    e!("JXL_TYPE_FLOAT", JxlDataType::FLOAT.0);
    e!("JXL_TYPE_UINT8", JxlDataType::UINT8.0);
    e!("JXL_TYPE_UINT16", JxlDataType::UINT16.0);
    e!("JXL_TYPE_FLOAT16", JxlDataType::FLOAT16.0);
    e!("JXL_NATIVE_ENDIAN", JxlEndianness::NATIVE.0);
    e!("JXL_LITTLE_ENDIAN", JxlEndianness::LITTLE.0);
    e!("JXL_BIG_ENDIAN", JxlEndianness::BIG.0);
    e!(
        "JXL_BIT_DEPTH_FROM_PIXEL_FORMAT",
        JxlBitDepthType::FROM_PIXEL_FORMAT.0
    );
    e!(
        "JXL_BIT_DEPTH_FROM_CODESTREAM",
        JxlBitDepthType::FROM_CODESTREAM.0
    );
    e!("JXL_BIT_DEPTH_CUSTOM", JxlBitDepthType::CUSTOM.0);

    s!("JxlMemoryManager", JxlMemoryManager);
    o!("JxlMemoryManager.opaque", JxlMemoryManager, opaque);
    o!("JxlMemoryManager.alloc", JxlMemoryManager, alloc);
    o!("JxlMemoryManager.free", JxlMemoryManager, free);

    s!("JxlOrientation", JxlOrientation);
    e!("JXL_ORIENT_IDENTITY", JxlOrientation::IDENTITY.0);
    e!(
        "JXL_ORIENT_FLIP_HORIZONTAL",
        JxlOrientation::FLIP_HORIZONTAL.0
    );
    e!("JXL_ORIENT_ROTATE_180", JxlOrientation::ROTATE_180.0);
    e!("JXL_ORIENT_FLIP_VERTICAL", JxlOrientation::FLIP_VERTICAL.0);
    e!("JXL_ORIENT_TRANSPOSE", JxlOrientation::TRANSPOSE.0);
    e!("JXL_ORIENT_ROTATE_90_CW", JxlOrientation::ROTATE_90_CW.0);
    e!(
        "JXL_ORIENT_ANTI_TRANSPOSE",
        JxlOrientation::ANTI_TRANSPOSE.0
    );
    e!("JXL_ORIENT_ROTATE_90_CCW", JxlOrientation::ROTATE_90_CCW.0);
    s!("JxlExtraChannelType", JxlExtraChannelType);
    e!("JXL_CHANNEL_ALPHA", JxlExtraChannelType::ALPHA.0);
    e!("JXL_CHANNEL_DEPTH", JxlExtraChannelType::DEPTH.0);
    e!("JXL_CHANNEL_SPOT_COLOR", JxlExtraChannelType::SPOT_COLOR.0);
    e!(
        "JXL_CHANNEL_SELECTION_MASK",
        JxlExtraChannelType::SELECTION_MASK.0
    );
    e!("JXL_CHANNEL_BLACK", JxlExtraChannelType::BLACK.0);
    e!("JXL_CHANNEL_CFA", JxlExtraChannelType::CFA.0);
    e!("JXL_CHANNEL_THERMAL", JxlExtraChannelType::THERMAL.0);
    e!("JXL_CHANNEL_RESERVED0", JxlExtraChannelType::RESERVED0.0);
    e!("JXL_CHANNEL_RESERVED1", JxlExtraChannelType::RESERVED1.0);
    e!("JXL_CHANNEL_RESERVED2", JxlExtraChannelType::RESERVED2.0);
    e!("JXL_CHANNEL_RESERVED3", JxlExtraChannelType::RESERVED3.0);
    e!("JXL_CHANNEL_RESERVED4", JxlExtraChannelType::RESERVED4.0);
    e!("JXL_CHANNEL_RESERVED5", JxlExtraChannelType::RESERVED5.0);
    e!("JXL_CHANNEL_RESERVED6", JxlExtraChannelType::RESERVED6.0);
    e!("JXL_CHANNEL_RESERVED7", JxlExtraChannelType::RESERVED7.0);
    e!("JXL_CHANNEL_UNKNOWN", JxlExtraChannelType::UNKNOWN.0);
    e!("JXL_CHANNEL_OPTIONAL", JxlExtraChannelType::OPTIONAL.0);
    s!("JxlPreviewHeader", JxlPreviewHeader);
    o!("JxlPreviewHeader.xsize", JxlPreviewHeader, xsize);
    o!("JxlPreviewHeader.ysize", JxlPreviewHeader, ysize);
    s!("JxlAnimationHeader", JxlAnimationHeader);
    o!(
        "JxlAnimationHeader.tps_numerator",
        JxlAnimationHeader,
        tps_numerator
    );
    o!(
        "JxlAnimationHeader.tps_denominator",
        JxlAnimationHeader,
        tps_denominator
    );
    o!(
        "JxlAnimationHeader.num_loops",
        JxlAnimationHeader,
        num_loops
    );
    o!(
        "JxlAnimationHeader.have_timecodes",
        JxlAnimationHeader,
        have_timecodes
    );
    s!("JxlBasicInfo", JxlBasicInfo);
    o!("JxlBasicInfo.have_container", JxlBasicInfo, have_container);
    o!("JxlBasicInfo.xsize", JxlBasicInfo, xsize);
    o!("JxlBasicInfo.ysize", JxlBasicInfo, ysize);
    o!(
        "JxlBasicInfo.bits_per_sample",
        JxlBasicInfo,
        bits_per_sample
    );
    o!(
        "JxlBasicInfo.exponent_bits_per_sample",
        JxlBasicInfo,
        exponent_bits_per_sample
    );
    o!(
        "JxlBasicInfo.intensity_target",
        JxlBasicInfo,
        intensity_target
    );
    o!("JxlBasicInfo.min_nits", JxlBasicInfo, min_nits);
    o!(
        "JxlBasicInfo.relative_to_max_display",
        JxlBasicInfo,
        relative_to_max_display
    );
    o!("JxlBasicInfo.linear_below", JxlBasicInfo, linear_below);
    o!(
        "JxlBasicInfo.uses_original_profile",
        JxlBasicInfo,
        uses_original_profile
    );
    o!("JxlBasicInfo.have_preview", JxlBasicInfo, have_preview);
    o!("JxlBasicInfo.have_animation", JxlBasicInfo, have_animation);
    o!("JxlBasicInfo.orientation", JxlBasicInfo, orientation);
    o!(
        "JxlBasicInfo.num_color_channels",
        JxlBasicInfo,
        num_color_channels
    );
    o!(
        "JxlBasicInfo.num_extra_channels",
        JxlBasicInfo,
        num_extra_channels
    );
    o!("JxlBasicInfo.alpha_bits", JxlBasicInfo, alpha_bits);
    o!(
        "JxlBasicInfo.alpha_exponent_bits",
        JxlBasicInfo,
        alpha_exponent_bits
    );
    o!(
        "JxlBasicInfo.alpha_premultiplied",
        JxlBasicInfo,
        alpha_premultiplied
    );
    o!("JxlBasicInfo.preview", JxlBasicInfo, preview);
    o!("JxlBasicInfo.animation", JxlBasicInfo, animation);
    o!(
        "JxlBasicInfo.intrinsic_xsize",
        JxlBasicInfo,
        intrinsic_xsize
    );
    o!(
        "JxlBasicInfo.intrinsic_ysize",
        JxlBasicInfo,
        intrinsic_ysize
    );
    o!("JxlBasicInfo.padding", JxlBasicInfo, padding);
    s!("JxlExtraChannelInfo", JxlExtraChannelInfo);
    o!(
        "JxlExtraChannelInfo.type",
        JxlExtraChannelInfo,
        channel_type
    );
    o!(
        "JxlExtraChannelInfo.bits_per_sample",
        JxlExtraChannelInfo,
        bits_per_sample
    );
    o!(
        "JxlExtraChannelInfo.exponent_bits_per_sample",
        JxlExtraChannelInfo,
        exponent_bits_per_sample
    );
    o!(
        "JxlExtraChannelInfo.dim_shift",
        JxlExtraChannelInfo,
        dim_shift
    );
    o!(
        "JxlExtraChannelInfo.name_length",
        JxlExtraChannelInfo,
        name_length
    );
    o!(
        "JxlExtraChannelInfo.alpha_premultiplied",
        JxlExtraChannelInfo,
        alpha_premultiplied
    );
    o!(
        "JxlExtraChannelInfo.spot_color",
        JxlExtraChannelInfo,
        spot_color
    );
    o!(
        "JxlExtraChannelInfo.cfa_channel",
        JxlExtraChannelInfo,
        cfa_channel
    );
    s!("JxlHeaderExtensions", JxlHeaderExtensions);
    o!(
        "JxlHeaderExtensions.extensions",
        JxlHeaderExtensions,
        extensions
    );
    s!("JxlBlendMode", JxlBlendMode);
    e!("JXL_BLEND_REPLACE", JxlBlendMode::REPLACE.0);
    e!("JXL_BLEND_ADD", JxlBlendMode::ADD.0);
    e!("JXL_BLEND_BLEND", JxlBlendMode::BLEND.0);
    e!("JXL_BLEND_MULADD", JxlBlendMode::MULADD.0);
    e!("JXL_BLEND_MUL", JxlBlendMode::MUL.0);
    s!("JxlBlendInfo", JxlBlendInfo);
    o!("JxlBlendInfo.blendmode", JxlBlendInfo, blendmode);
    o!("JxlBlendInfo.source", JxlBlendInfo, source);
    o!("JxlBlendInfo.alpha", JxlBlendInfo, alpha);
    o!("JxlBlendInfo.clamp", JxlBlendInfo, clamp);
    s!("JxlLayerInfo", JxlLayerInfo);
    o!("JxlLayerInfo.have_crop", JxlLayerInfo, have_crop);
    o!("JxlLayerInfo.crop_x0", JxlLayerInfo, crop_x0);
    o!("JxlLayerInfo.crop_y0", JxlLayerInfo, crop_y0);
    o!("JxlLayerInfo.xsize", JxlLayerInfo, xsize);
    o!("JxlLayerInfo.ysize", JxlLayerInfo, ysize);
    o!("JxlLayerInfo.blend_info", JxlLayerInfo, blend_info);
    o!(
        "JxlLayerInfo.save_as_reference",
        JxlLayerInfo,
        save_as_reference
    );
    s!("JxlFrameHeader", JxlFrameHeader);
    o!("JxlFrameHeader.duration", JxlFrameHeader, duration);
    o!("JxlFrameHeader.timecode", JxlFrameHeader, timecode);
    o!("JxlFrameHeader.name_length", JxlFrameHeader, name_length);
    o!("JxlFrameHeader.is_last", JxlFrameHeader, is_last);
    o!("JxlFrameHeader.layer_info", JxlFrameHeader, layer_info);

    s!("JxlColorSpace", ColorSpace);
    e!("JXL_COLOR_SPACE_RGB", ColorSpace::RGB.0);
    e!("JXL_COLOR_SPACE_GRAY", ColorSpace::GRAY.0);
    e!("JXL_COLOR_SPACE_XYB", ColorSpace::XYB.0);
    e!("JXL_COLOR_SPACE_UNKNOWN", ColorSpace::UNKNOWN.0);
    s!("JxlWhitePoint", WhitePoint);
    e!("JXL_WHITE_POINT_D65", WhitePoint::D65.0);
    e!("JXL_WHITE_POINT_CUSTOM", WhitePoint::CUSTOM.0);
    e!("JXL_WHITE_POINT_E", WhitePoint::E.0);
    e!("JXL_WHITE_POINT_DCI", WhitePoint::DCI.0);
    s!("JxlPrimaries", Primaries);
    e!("JXL_PRIMARIES_SRGB", Primaries::SRGB.0);
    e!("JXL_PRIMARIES_CUSTOM", Primaries::CUSTOM.0);
    e!("JXL_PRIMARIES_2100", Primaries::BT2100.0);
    e!("JXL_PRIMARIES_P3", Primaries::P3.0);
    s!("JxlTransferFunction", TransferFunction);
    e!("JXL_TRANSFER_FUNCTION_709", TransferFunction::BT709.0);
    e!("JXL_TRANSFER_FUNCTION_UNKNOWN", TransferFunction::UNKNOWN.0);
    e!("JXL_TRANSFER_FUNCTION_LINEAR", TransferFunction::LINEAR.0);
    e!("JXL_TRANSFER_FUNCTION_SRGB", TransferFunction::SRGB.0);
    e!("JXL_TRANSFER_FUNCTION_PQ", TransferFunction::PQ.0);
    e!("JXL_TRANSFER_FUNCTION_DCI", TransferFunction::DCI.0);
    e!("JXL_TRANSFER_FUNCTION_HLG", TransferFunction::HLG.0);
    e!("JXL_TRANSFER_FUNCTION_GAMMA", TransferFunction::GAMMA.0);
    s!("JxlRenderingIntent", RenderingIntent);
    e!(
        "JXL_RENDERING_INTENT_PERCEPTUAL",
        RenderingIntent::PERCEPTUAL.0
    );
    e!("JXL_RENDERING_INTENT_RELATIVE", RenderingIntent::RELATIVE.0);
    e!(
        "JXL_RENDERING_INTENT_SATURATION",
        RenderingIntent::SATURATION.0
    );
    e!("JXL_RENDERING_INTENT_ABSOLUTE", RenderingIntent::ABSOLUTE.0);
    s!("JxlColorEncoding", JxlColorEncoding);
    o!(
        "JxlColorEncoding.color_space",
        JxlColorEncoding,
        color_space
    );
    o!(
        "JxlColorEncoding.white_point",
        JxlColorEncoding,
        white_point
    );
    o!(
        "JxlColorEncoding.white_point_xy",
        JxlColorEncoding,
        white_point_xy
    );
    o!("JxlColorEncoding.primaries", JxlColorEncoding, primaries);
    o!(
        "JxlColorEncoding.primaries_red_xy",
        JxlColorEncoding,
        primaries_red_xy
    );
    o!(
        "JxlColorEncoding.primaries_green_xy",
        JxlColorEncoding,
        primaries_green_xy
    );
    o!(
        "JxlColorEncoding.primaries_blue_xy",
        JxlColorEncoding,
        primaries_blue_xy
    );
    o!(
        "JxlColorEncoding.transfer_function",
        JxlColorEncoding,
        transfer_function
    );
    o!("JxlColorEncoding.gamma", JxlColorEncoding, gamma);
    o!(
        "JxlColorEncoding.rendering_intent",
        JxlColorEncoding,
        rendering_intent
    );

    s!("JxlColorProfile", JxlColorProfile);
    o!("JxlColorProfile.icc", JxlColorProfile, icc);
    o!("JxlColorProfile.icc.data", JxlColorProfile, icc.data);
    o!("JxlColorProfile.icc.size", JxlColorProfile, icc.size);
    o!(
        "JxlColorProfile.color_encoding",
        JxlColorProfile,
        color_encoding
    );
    o!(
        "JxlColorProfile.num_channels",
        JxlColorProfile,
        num_channels
    );
    s!("JxlCmsInterface", JxlCmsInterface);
    o!(
        "JxlCmsInterface.set_fields_data",
        JxlCmsInterface,
        set_fields_data
    );
    o!(
        "JxlCmsInterface.set_fields_from_icc",
        JxlCmsInterface,
        set_fields_from_icc
    );
    o!("JxlCmsInterface.init_data", JxlCmsInterface, init_data);
    o!("JxlCmsInterface.init", JxlCmsInterface, init);
    o!("JxlCmsInterface.get_src_buf", JxlCmsInterface, get_src_buf);
    o!("JxlCmsInterface.get_dst_buf", JxlCmsInterface, get_dst_buf);
    o!("JxlCmsInterface.run", JxlCmsInterface, run);
    o!("JxlCmsInterface.destroy", JxlCmsInterface, destroy);

    s!("JxlSignature", JxlSignature);
    e!("JXL_SIG_NOT_ENOUGH_BYTES", JxlSignature::NOT_ENOUGH_BYTES.0);
    e!("JXL_SIG_INVALID", JxlSignature::INVALID.0);
    e!("JXL_SIG_CODESTREAM", JxlSignature::CODESTREAM.0);
    e!("JXL_SIG_CONTAINER", JxlSignature::CONTAINER.0);
    s!("JxlDecoderStatus", JxlDecoderStatus);
    e!("JXL_DEC_SUCCESS", JxlDecoderStatus::SUCCESS.0);
    e!("JXL_DEC_ERROR", JxlDecoderStatus::ERROR.0);
    e!(
        "JXL_DEC_NEED_MORE_INPUT",
        JxlDecoderStatus::NEED_MORE_INPUT.0
    );
    e!(
        "JXL_DEC_NEED_PREVIEW_OUT_BUFFER",
        JxlDecoderStatus::NEED_PREVIEW_OUT_BUFFER.0
    );
    e!(
        "JXL_DEC_NEED_IMAGE_OUT_BUFFER",
        JxlDecoderStatus::NEED_IMAGE_OUT_BUFFER.0
    );
    e!(
        "JXL_DEC_JPEG_NEED_MORE_OUTPUT",
        JxlDecoderStatus::JPEG_NEED_MORE_OUTPUT.0
    );
    e!(
        "JXL_DEC_BOX_NEED_MORE_OUTPUT",
        JxlDecoderStatus::BOX_NEED_MORE_OUTPUT.0
    );
    e!("JXL_DEC_BASIC_INFO", JxlDecoderStatus::BASIC_INFO.0);
    e!("JXL_DEC_COLOR_ENCODING", JxlDecoderStatus::COLOR_ENCODING.0);
    e!("JXL_DEC_PREVIEW_IMAGE", JxlDecoderStatus::PREVIEW_IMAGE.0);
    e!("JXL_DEC_FRAME", JxlDecoderStatus::FRAME.0);
    e!("JXL_DEC_FULL_IMAGE", JxlDecoderStatus::FULL_IMAGE.0);
    e!(
        "JXL_DEC_JPEG_RECONSTRUCTION",
        JxlDecoderStatus::JPEG_RECONSTRUCTION.0
    );
    e!("JXL_DEC_BOX", JxlDecoderStatus::BOX.0);
    e!(
        "JXL_DEC_FRAME_PROGRESSION",
        JxlDecoderStatus::FRAME_PROGRESSION.0
    );
    e!("JXL_DEC_BOX_COMPLETE", JxlDecoderStatus::BOX_COMPLETE.0);
    s!("JxlProgressiveDetail", JxlProgressiveDetail);
    e!("kFrames", JxlProgressiveDetail::FRAMES.0);
    e!("kDC", JxlProgressiveDetail::DC.0);
    e!("kLastPasses", JxlProgressiveDetail::LAST_PASSES.0);
    e!("kPasses", JxlProgressiveDetail::PASSES.0);
    e!("kDCProgressive", JxlProgressiveDetail::DC_PROGRESSIVE.0);
    e!("kDCGroups", JxlProgressiveDetail::DC_GROUPS.0);
    e!("kGroups", JxlProgressiveDetail::GROUPS.0);
    s!("JxlColorProfileTarget", JxlColorProfileTarget);
    e!(
        "JXL_COLOR_PROFILE_TARGET_ORIGINAL",
        JxlColorProfileTarget::ORIGINAL.0
    );
    e!(
        "JXL_COLOR_PROFILE_TARGET_DATA",
        JxlColorProfileTarget::DATA.0
    );
    out
}

#[test]
fn c_abi_layouts_enumerators_and_exports_match_the_pinned_headers() {
    let expected = rust_layout();
    for variant in ["original", "rust"] {
        let measured = super::program_observations(&driver(variant), &["abi-layout".into()]);
        let measured: Vec<&str> = measured[0].split(' ').collect();
        let expected: Vec<&str> = expected.split(' ').collect();
        for (c, rust) in measured.iter().zip(&expected) {
            assert_eq!(c, rust, "{variant}: C header measurement differs from Rust");
        }
        assert_eq!(
            measured.len(),
            expected.len(),
            "{variant}: measurement count"
        );
        println!(
            "{variant}: {} C ABI layout, enumerator and export measurements match",
            measured.len()
        );
    }
}
