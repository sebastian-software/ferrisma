// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Measures the public decoder ABI of the pinned headers with a C compiler:
// the size, alignment and field offsets of every public struct that decode.h
// uses, every public enumerator, the version, and the address of every
// function that decode.h, thread_parallel_runner.h and
// resizable_parallel_runner.h declare. Linking this unit against a library
// therefore requires all of those functions.
#include <jxl/decode.h>
#include <jxl/resizable_parallel_runner.h>
#include <jxl/thread_parallel_runner.h>
#include <stdalign.h>
#include <stdarg.h>
#include <stddef.h>
#include <stdio.h>
#include <stdlib.h>

static char layout[32768];
static size_t used;

static void Put(const char* format, ...) {
  va_list args;
  va_start(args, format);
  int n = vsnprintf(layout + used, sizeof(layout) - used, format, args);
  va_end(args);
  if (n < 0 || (size_t)n >= sizeof(layout) - used) abort();
  used += (size_t)n;
}

#define STRUCT(T) Put(" S:%s:%zu:%zu", #T, sizeof(T), alignof(T))
#define FIELD(T, f) Put(" O:%s.%s:%zu", #T, #f, offsetof(T, f))
#define ENUM(v) Put(" E:%s:%lld", #v, (long long)(v))

typedef void (*Function)(void);
#define EXPORT(f) (Function)(f)

// Every JXL_EXPORT function of decode.h.
static const Function kDecoderExports[] = {
    EXPORT(JxlDecoderVersion),
    EXPORT(JxlSignatureCheck),
    EXPORT(JxlDecoderCreate),
    EXPORT(JxlDecoderReset),
    EXPORT(JxlDecoderDestroy),
    EXPORT(JxlDecoderRewind),
    EXPORT(JxlDecoderSkipFrames),
    EXPORT(JxlDecoderSkipCurrentFrame),
    EXPORT(JxlDecoderSetParallelRunner),
    EXPORT(JxlDecoderSizeHintBasicInfo),
    EXPORT(JxlDecoderSubscribeEvents),
    EXPORT(JxlDecoderSetKeepOrientation),
    EXPORT(JxlDecoderSetUnpremultiplyAlpha),
    EXPORT(JxlDecoderSetRenderSpotcolors),
    EXPORT(JxlDecoderSetCoalescing),
    EXPORT(JxlDecoderProcessInput),
    EXPORT(JxlDecoderSetInput),
    EXPORT(JxlDecoderReleaseInput),
    EXPORT(JxlDecoderCloseInput),
    EXPORT(JxlDecoderGetBasicInfo),
    EXPORT(JxlDecoderGetExtraChannelInfo),
    EXPORT(JxlDecoderGetExtraChannelName),
    EXPORT(JxlDecoderGetColorAsEncodedProfile),
    EXPORT(JxlDecoderGetICCProfileSize),
    EXPORT(JxlDecoderGetColorAsICCProfile),
    EXPORT(JxlDecoderSetPreferredColorProfile),
    EXPORT(JxlDecoderSetDesiredIntensityTarget),
    EXPORT(JxlDecoderSetOutputColorProfile),
    EXPORT(JxlDecoderSetCms),
    EXPORT(JxlDecoderPreviewOutBufferSize),
    EXPORT(JxlDecoderSetPreviewOutBuffer),
    EXPORT(JxlDecoderGetFrameHeader),
    EXPORT(JxlDecoderGetFrameName),
    EXPORT(JxlDecoderGetExtraChannelBlendInfo),
    EXPORT(JxlDecoderImageOutBufferSize),
    EXPORT(JxlDecoderSetImageOutBuffer),
    EXPORT(JxlDecoderSetImageOutCallback),
    EXPORT(JxlDecoderSetMultithreadedImageOutCallback),
    EXPORT(JxlDecoderExtraChannelBufferSize),
    EXPORT(JxlDecoderSetExtraChannelBuffer),
    EXPORT(JxlDecoderSetJPEGBuffer),
    EXPORT(JxlDecoderReleaseJPEGBuffer),
    EXPORT(JxlDecoderSetBoxBuffer),
    EXPORT(JxlDecoderReleaseBoxBuffer),
    EXPORT(JxlDecoderSetDecompressBoxes),
    EXPORT(JxlDecoderGetBoxType),
    EXPORT(JxlDecoderGetBoxSizeRaw),
    EXPORT(JxlDecoderGetBoxSizeContents),
    EXPORT(JxlDecoderSetProgressiveDetail),
    EXPORT(JxlDecoderGetIntendedDownsamplingRatio),
    EXPORT(JxlDecoderFlushImage),
    EXPORT(JxlDecoderSetImageOutBitDepth),
};

// Every JXL_THREADS_EXPORT function of thread_parallel_runner.h and
// resizable_parallel_runner.h.
static const Function kRunnerExports[] = {
    EXPORT(JxlThreadParallelRunner),
    EXPORT(JxlThreadParallelRunnerCreate),
    EXPORT(JxlThreadParallelRunnerDestroy),
    EXPORT(JxlThreadParallelRunnerDefaultNumWorkerThreads),
    EXPORT(JxlResizableParallelRunner),
    EXPORT(JxlResizableParallelRunnerCreate),
    EXPORT(JxlResizableParallelRunnerSetThreads),
    EXPORT(JxlResizableParallelRunnerSuggestThreads),
    EXPORT(JxlResizableParallelRunnerDestroy),
};

const char* DecoderAbiLayout(void) {
  used = 0;
  layout[0] = 0;
  // Read each address through a volatile object so that every reference,
  // and with it every symbol, survives optimization and section collection.
  size_t exports = 0;
  for (size_t i = 0; i < sizeof(kDecoderExports) / sizeof(kDecoderExports[0]); ++i) {
    Function volatile f = kDecoderExports[i];
    if (f) ++exports;
  }
  size_t runner_exports = 0;
  for (size_t i = 0; i < sizeof(kRunnerExports) / sizeof(kRunnerExports[0]); ++i) {
    Function volatile f = kRunnerExports[i];
    if (f) ++runner_exports;
  }
  Put("F:%zu R:%zu V:%u:%d.%d.%d", exports, runner_exports, JxlDecoderVersion(),
      JPEGXL_MAJOR_VERSION, JPEGXL_MINOR_VERSION, JPEGXL_PATCH_VERSION);

  // types.h
  STRUCT(JxlPixelFormat);
  FIELD(JxlPixelFormat, num_channels);
  FIELD(JxlPixelFormat, data_type);
  FIELD(JxlPixelFormat, endianness);
  FIELD(JxlPixelFormat, align);
  STRUCT(JxlBitDepth);
  FIELD(JxlBitDepth, type);
  FIELD(JxlBitDepth, bits_per_sample);
  FIELD(JxlBitDepth, exponent_bits_per_sample);
  STRUCT(JxlBoxType);
  STRUCT(JXL_BOOL);
  STRUCT(JxlDataType);
  STRUCT(JxlEndianness);
  STRUCT(JxlBitDepthType);
  ENUM(JXL_TRUE);
  ENUM(JXL_FALSE);
  ENUM(JXL_TYPE_FLOAT);
  ENUM(JXL_TYPE_UINT8);
  ENUM(JXL_TYPE_UINT16);
  ENUM(JXL_TYPE_FLOAT16);
  ENUM(JXL_NATIVE_ENDIAN);
  ENUM(JXL_LITTLE_ENDIAN);
  ENUM(JXL_BIG_ENDIAN);
  ENUM(JXL_BIT_DEPTH_FROM_PIXEL_FORMAT);
  ENUM(JXL_BIT_DEPTH_FROM_CODESTREAM);
  ENUM(JXL_BIT_DEPTH_CUSTOM);

  // memory_manager.h
  STRUCT(JxlMemoryManager);
  FIELD(JxlMemoryManager, opaque);
  FIELD(JxlMemoryManager, alloc);
  FIELD(JxlMemoryManager, free);

  // codestream_header.h
  STRUCT(JxlOrientation);
  ENUM(JXL_ORIENT_IDENTITY);
  ENUM(JXL_ORIENT_FLIP_HORIZONTAL);
  ENUM(JXL_ORIENT_ROTATE_180);
  ENUM(JXL_ORIENT_FLIP_VERTICAL);
  ENUM(JXL_ORIENT_TRANSPOSE);
  ENUM(JXL_ORIENT_ROTATE_90_CW);
  ENUM(JXL_ORIENT_ANTI_TRANSPOSE);
  ENUM(JXL_ORIENT_ROTATE_90_CCW);
  STRUCT(JxlExtraChannelType);
  ENUM(JXL_CHANNEL_ALPHA);
  ENUM(JXL_CHANNEL_DEPTH);
  ENUM(JXL_CHANNEL_SPOT_COLOR);
  ENUM(JXL_CHANNEL_SELECTION_MASK);
  ENUM(JXL_CHANNEL_BLACK);
  ENUM(JXL_CHANNEL_CFA);
  ENUM(JXL_CHANNEL_THERMAL);
  ENUM(JXL_CHANNEL_RESERVED0);
  ENUM(JXL_CHANNEL_RESERVED1);
  ENUM(JXL_CHANNEL_RESERVED2);
  ENUM(JXL_CHANNEL_RESERVED3);
  ENUM(JXL_CHANNEL_RESERVED4);
  ENUM(JXL_CHANNEL_RESERVED5);
  ENUM(JXL_CHANNEL_RESERVED6);
  ENUM(JXL_CHANNEL_RESERVED7);
  ENUM(JXL_CHANNEL_UNKNOWN);
  ENUM(JXL_CHANNEL_OPTIONAL);
  STRUCT(JxlPreviewHeader);
  FIELD(JxlPreviewHeader, xsize);
  FIELD(JxlPreviewHeader, ysize);
  STRUCT(JxlAnimationHeader);
  FIELD(JxlAnimationHeader, tps_numerator);
  FIELD(JxlAnimationHeader, tps_denominator);
  FIELD(JxlAnimationHeader, num_loops);
  FIELD(JxlAnimationHeader, have_timecodes);
  STRUCT(JxlBasicInfo);
  FIELD(JxlBasicInfo, have_container);
  FIELD(JxlBasicInfo, xsize);
  FIELD(JxlBasicInfo, ysize);
  FIELD(JxlBasicInfo, bits_per_sample);
  FIELD(JxlBasicInfo, exponent_bits_per_sample);
  FIELD(JxlBasicInfo, intensity_target);
  FIELD(JxlBasicInfo, min_nits);
  FIELD(JxlBasicInfo, relative_to_max_display);
  FIELD(JxlBasicInfo, linear_below);
  FIELD(JxlBasicInfo, uses_original_profile);
  FIELD(JxlBasicInfo, have_preview);
  FIELD(JxlBasicInfo, have_animation);
  FIELD(JxlBasicInfo, orientation);
  FIELD(JxlBasicInfo, num_color_channels);
  FIELD(JxlBasicInfo, num_extra_channels);
  FIELD(JxlBasicInfo, alpha_bits);
  FIELD(JxlBasicInfo, alpha_exponent_bits);
  FIELD(JxlBasicInfo, alpha_premultiplied);
  FIELD(JxlBasicInfo, preview);
  FIELD(JxlBasicInfo, animation);
  FIELD(JxlBasicInfo, intrinsic_xsize);
  FIELD(JxlBasicInfo, intrinsic_ysize);
  FIELD(JxlBasicInfo, padding);
  STRUCT(JxlExtraChannelInfo);
  FIELD(JxlExtraChannelInfo, type);
  FIELD(JxlExtraChannelInfo, bits_per_sample);
  FIELD(JxlExtraChannelInfo, exponent_bits_per_sample);
  FIELD(JxlExtraChannelInfo, dim_shift);
  FIELD(JxlExtraChannelInfo, name_length);
  FIELD(JxlExtraChannelInfo, alpha_premultiplied);
  FIELD(JxlExtraChannelInfo, spot_color);
  FIELD(JxlExtraChannelInfo, cfa_channel);
  STRUCT(JxlHeaderExtensions);
  FIELD(JxlHeaderExtensions, extensions);
  STRUCT(JxlBlendMode);
  ENUM(JXL_BLEND_REPLACE);
  ENUM(JXL_BLEND_ADD);
  ENUM(JXL_BLEND_BLEND);
  ENUM(JXL_BLEND_MULADD);
  ENUM(JXL_BLEND_MUL);
  STRUCT(JxlBlendInfo);
  FIELD(JxlBlendInfo, blendmode);
  FIELD(JxlBlendInfo, source);
  FIELD(JxlBlendInfo, alpha);
  FIELD(JxlBlendInfo, clamp);
  STRUCT(JxlLayerInfo);
  FIELD(JxlLayerInfo, have_crop);
  FIELD(JxlLayerInfo, crop_x0);
  FIELD(JxlLayerInfo, crop_y0);
  FIELD(JxlLayerInfo, xsize);
  FIELD(JxlLayerInfo, ysize);
  FIELD(JxlLayerInfo, blend_info);
  FIELD(JxlLayerInfo, save_as_reference);
  STRUCT(JxlFrameHeader);
  FIELD(JxlFrameHeader, duration);
  FIELD(JxlFrameHeader, timecode);
  FIELD(JxlFrameHeader, name_length);
  FIELD(JxlFrameHeader, is_last);
  FIELD(JxlFrameHeader, layer_info);

  // color_encoding.h
  STRUCT(JxlColorSpace);
  ENUM(JXL_COLOR_SPACE_RGB);
  ENUM(JXL_COLOR_SPACE_GRAY);
  ENUM(JXL_COLOR_SPACE_XYB);
  ENUM(JXL_COLOR_SPACE_UNKNOWN);
  STRUCT(JxlWhitePoint);
  ENUM(JXL_WHITE_POINT_D65);
  ENUM(JXL_WHITE_POINT_CUSTOM);
  ENUM(JXL_WHITE_POINT_E);
  ENUM(JXL_WHITE_POINT_DCI);
  STRUCT(JxlPrimaries);
  ENUM(JXL_PRIMARIES_SRGB);
  ENUM(JXL_PRIMARIES_CUSTOM);
  ENUM(JXL_PRIMARIES_2100);
  ENUM(JXL_PRIMARIES_P3);
  STRUCT(JxlTransferFunction);
  ENUM(JXL_TRANSFER_FUNCTION_709);
  ENUM(JXL_TRANSFER_FUNCTION_UNKNOWN);
  ENUM(JXL_TRANSFER_FUNCTION_LINEAR);
  ENUM(JXL_TRANSFER_FUNCTION_SRGB);
  ENUM(JXL_TRANSFER_FUNCTION_PQ);
  ENUM(JXL_TRANSFER_FUNCTION_DCI);
  ENUM(JXL_TRANSFER_FUNCTION_HLG);
  ENUM(JXL_TRANSFER_FUNCTION_GAMMA);
  STRUCT(JxlRenderingIntent);
  ENUM(JXL_RENDERING_INTENT_PERCEPTUAL);
  ENUM(JXL_RENDERING_INTENT_RELATIVE);
  ENUM(JXL_RENDERING_INTENT_SATURATION);
  ENUM(JXL_RENDERING_INTENT_ABSOLUTE);
  STRUCT(JxlColorEncoding);
  FIELD(JxlColorEncoding, color_space);
  FIELD(JxlColorEncoding, white_point);
  FIELD(JxlColorEncoding, white_point_xy);
  FIELD(JxlColorEncoding, primaries);
  FIELD(JxlColorEncoding, primaries_red_xy);
  FIELD(JxlColorEncoding, primaries_green_xy);
  FIELD(JxlColorEncoding, primaries_blue_xy);
  FIELD(JxlColorEncoding, transfer_function);
  FIELD(JxlColorEncoding, gamma);
  FIELD(JxlColorEncoding, rendering_intent);

  // cms_interface.h
  STRUCT(JxlColorProfile);
  FIELD(JxlColorProfile, icc);
  FIELD(JxlColorProfile, icc.data);
  FIELD(JxlColorProfile, icc.size);
  FIELD(JxlColorProfile, color_encoding);
  FIELD(JxlColorProfile, num_channels);
  STRUCT(JxlCmsInterface);
  FIELD(JxlCmsInterface, set_fields_data);
  FIELD(JxlCmsInterface, set_fields_from_icc);
  FIELD(JxlCmsInterface, init_data);
  FIELD(JxlCmsInterface, init);
  FIELD(JxlCmsInterface, get_src_buf);
  FIELD(JxlCmsInterface, get_dst_buf);
  FIELD(JxlCmsInterface, run);
  FIELD(JxlCmsInterface, destroy);

  // decode.h
  STRUCT(JxlSignature);
  ENUM(JXL_SIG_NOT_ENOUGH_BYTES);
  ENUM(JXL_SIG_INVALID);
  ENUM(JXL_SIG_CODESTREAM);
  ENUM(JXL_SIG_CONTAINER);
  STRUCT(JxlDecoderStatus);
  ENUM(JXL_DEC_SUCCESS);
  ENUM(JXL_DEC_ERROR);
  ENUM(JXL_DEC_NEED_MORE_INPUT);
  ENUM(JXL_DEC_NEED_PREVIEW_OUT_BUFFER);
  ENUM(JXL_DEC_NEED_IMAGE_OUT_BUFFER);
  ENUM(JXL_DEC_JPEG_NEED_MORE_OUTPUT);
  ENUM(JXL_DEC_BOX_NEED_MORE_OUTPUT);
  ENUM(JXL_DEC_BASIC_INFO);
  ENUM(JXL_DEC_COLOR_ENCODING);
  ENUM(JXL_DEC_PREVIEW_IMAGE);
  ENUM(JXL_DEC_FRAME);
  ENUM(JXL_DEC_FULL_IMAGE);
  ENUM(JXL_DEC_JPEG_RECONSTRUCTION);
  ENUM(JXL_DEC_BOX);
  ENUM(JXL_DEC_FRAME_PROGRESSION);
  ENUM(JXL_DEC_BOX_COMPLETE);
  STRUCT(JxlProgressiveDetail);
  ENUM(kFrames);
  ENUM(kDC);
  ENUM(kLastPasses);
  ENUM(kPasses);
  ENUM(kDCProgressive);
  ENUM(kDCGroups);
  ENUM(kGroups);
  STRUCT(JxlColorProfileTarget);
  ENUM(JXL_COLOR_PROFILE_TARGET_ORIGINAL);
  ENUM(JXL_COLOR_PROFILE_TARGET_DATA);
  return layout;
}
