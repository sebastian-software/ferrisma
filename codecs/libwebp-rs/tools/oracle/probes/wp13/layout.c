// WP13 probe: size, alignment and field offsets of the encode.h structs:
// WebPPicture and WebPMemoryWriter, which src/abi/encode.rs mirrors with
// repr(C), and WebPConfig and WebPAuxStats, which the C ABI shares with
// src/webp/encode.rs (ADR 0004); and the sizes of the enums and callback
// pointers. Prints one line per struct and member; tests/wp13_abi_layout.rs
// compares them with the Rust types.

#include <stddef.h>
#include <stdio.h>

#include "src/webp/encode.h"
#include "probe_support.h"

#define STRUCT(T) printf(#T "=%zu,%zu\n", sizeof(T), _Alignof(T))
#define FIELD(T, F) printf(#T "." #F "=%zu\n", offsetof(T, F))
#define TYPE(T) printf(#T "=%zu,%zu\n", sizeof(T), _Alignof(T))

int main(int argc, char* argv[]) {
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "layout");
  STRUCT(WebPPicture);
  FIELD(WebPPicture, use_argb);
  FIELD(WebPPicture, colorspace);
  FIELD(WebPPicture, width);
  FIELD(WebPPicture, height);
  FIELD(WebPPicture, y);
  FIELD(WebPPicture, u);
  FIELD(WebPPicture, v);
  FIELD(WebPPicture, y_stride);
  FIELD(WebPPicture, uv_stride);
  FIELD(WebPPicture, a);
  FIELD(WebPPicture, a_stride);
  FIELD(WebPPicture, pad1);
  FIELD(WebPPicture, argb);
  FIELD(WebPPicture, argb_stride);
  FIELD(WebPPicture, pad2);
  FIELD(WebPPicture, writer);
  FIELD(WebPPicture, custom_ptr);
  FIELD(WebPPicture, extra_info_type);
  FIELD(WebPPicture, extra_info);
  FIELD(WebPPicture, stats);
  FIELD(WebPPicture, error_code);
  FIELD(WebPPicture, progress_hook);
  FIELD(WebPPicture, user_data);
  FIELD(WebPPicture, pad3);
  FIELD(WebPPicture, pad4);
  FIELD(WebPPicture, pad5);
  FIELD(WebPPicture, pad6);
  FIELD(WebPPicture, memory_);
  FIELD(WebPPicture, memory_argb_);
  FIELD(WebPPicture, pad7);
  STRUCT(WebPMemoryWriter);
  FIELD(WebPMemoryWriter, mem);
  FIELD(WebPMemoryWriter, size);
  FIELD(WebPMemoryWriter, max_size);
  FIELD(WebPMemoryWriter, pad);
  STRUCT(WebPConfig);
  FIELD(WebPConfig, lossless);
  FIELD(WebPConfig, quality);
  FIELD(WebPConfig, method);
  FIELD(WebPConfig, image_hint);
  FIELD(WebPConfig, target_size);
  FIELD(WebPConfig, target_PSNR);
  FIELD(WebPConfig, segments);
  FIELD(WebPConfig, sns_strength);
  FIELD(WebPConfig, filter_strength);
  FIELD(WebPConfig, filter_sharpness);
  FIELD(WebPConfig, filter_type);
  FIELD(WebPConfig, autofilter);
  FIELD(WebPConfig, alpha_compression);
  FIELD(WebPConfig, alpha_filtering);
  FIELD(WebPConfig, alpha_quality);
  FIELD(WebPConfig, pass);
  FIELD(WebPConfig, show_compressed);
  FIELD(WebPConfig, preprocessing);
  FIELD(WebPConfig, partitions);
  FIELD(WebPConfig, partition_limit);
  FIELD(WebPConfig, emulate_jpeg_size);
  FIELD(WebPConfig, thread_level);
  FIELD(WebPConfig, low_memory);
  FIELD(WebPConfig, near_lossless);
  FIELD(WebPConfig, exact);
  FIELD(WebPConfig, use_delta_palette);
  FIELD(WebPConfig, use_sharp_yuv);
  FIELD(WebPConfig, qmin);
  FIELD(WebPConfig, qmax);
  STRUCT(WebPAuxStats);
  FIELD(WebPAuxStats, coded_size);
  FIELD(WebPAuxStats, PSNR);
  FIELD(WebPAuxStats, block_count);
  FIELD(WebPAuxStats, header_bytes);
  FIELD(WebPAuxStats, residual_bytes);
  FIELD(WebPAuxStats, segment_size);
  FIELD(WebPAuxStats, segment_quant);
  FIELD(WebPAuxStats, segment_level);
  FIELD(WebPAuxStats, alpha_data_size);
  FIELD(WebPAuxStats, layer_data_size);
  FIELD(WebPAuxStats, lossless_features);
  FIELD(WebPAuxStats, histogram_bits);
  FIELD(WebPAuxStats, transform_bits);
  FIELD(WebPAuxStats, cache_bits);
  FIELD(WebPAuxStats, palette_size);
  FIELD(WebPAuxStats, lossless_size);
  FIELD(WebPAuxStats, lossless_hdr_size);
  FIELD(WebPAuxStats, lossless_data_size);
  FIELD(WebPAuxStats, cross_color_transform_bits);
  FIELD(WebPAuxStats, pad);
  TYPE(WebPEncCSP);
  TYPE(WebPEncodingError);
  TYPE(WebPPreset);
  TYPE(WebPImageHint);
  TYPE(WebPWriterFunction);
  TYPE(WebPProgressHook);
  return 0;
}
