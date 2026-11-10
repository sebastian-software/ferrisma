// WP1 probe: prints the size, alignment and field offsets of the public structs
// of the WP1 headers that have no pointers or callbacks, as the C compiler lays
// them out. tests/wp1_struct_layout.rs compares them with the `repr(C)` Rust
// structs (ADR 0003). Each line is `Struct.member=value` in decimal.
//
// Usage: struct_layout

#include <stddef.h>
#include <stdio.h>

#include "probe_support.h"
#include "src/webp/decode.h"
#include "src/webp/demux.h"
#include "src/webp/encode.h"

#define PRINT_LAYOUT(T)                                       \
  printf(#T ".size=%zu\n", sizeof(T));                        \
  printf(#T ".align=%zu\n", _Alignof(T))
#define PRINT_OFFSET(T, member) \
  printf(#T "." #member ".offset=%zu\n", offsetof(T, member))

int main(int argc, char* argv[]) {
  ProbeCheckArgs(argc, 0, 0, "struct_layout");

  PRINT_LAYOUT(WebPBitstreamFeatures);
  PRINT_OFFSET(WebPBitstreamFeatures, width);
  PRINT_OFFSET(WebPBitstreamFeatures, height);
  PRINT_OFFSET(WebPBitstreamFeatures, has_alpha);
  PRINT_OFFSET(WebPBitstreamFeatures, has_animation);
  PRINT_OFFSET(WebPBitstreamFeatures, format);
  PRINT_OFFSET(WebPBitstreamFeatures, pad);

  PRINT_LAYOUT(WebPDecoderOptions);
  PRINT_OFFSET(WebPDecoderOptions, bypass_filtering);
  PRINT_OFFSET(WebPDecoderOptions, no_fancy_upsampling);
  PRINT_OFFSET(WebPDecoderOptions, use_cropping);
  PRINT_OFFSET(WebPDecoderOptions, crop_left);
  PRINT_OFFSET(WebPDecoderOptions, crop_top);
  PRINT_OFFSET(WebPDecoderOptions, crop_width);
  PRINT_OFFSET(WebPDecoderOptions, crop_height);
  PRINT_OFFSET(WebPDecoderOptions, use_scaling);
  PRINT_OFFSET(WebPDecoderOptions, scaled_width);
  PRINT_OFFSET(WebPDecoderOptions, scaled_height);
  PRINT_OFFSET(WebPDecoderOptions, use_threads);
  PRINT_OFFSET(WebPDecoderOptions, dithering_strength);
  PRINT_OFFSET(WebPDecoderOptions, flip);
  PRINT_OFFSET(WebPDecoderOptions, alpha_dithering_strength);
  PRINT_OFFSET(WebPDecoderOptions, pad);

  PRINT_LAYOUT(WebPConfig);
  PRINT_OFFSET(WebPConfig, lossless);
  PRINT_OFFSET(WebPConfig, quality);
  PRINT_OFFSET(WebPConfig, method);
  PRINT_OFFSET(WebPConfig, image_hint);
  PRINT_OFFSET(WebPConfig, target_size);
  PRINT_OFFSET(WebPConfig, target_PSNR);
  PRINT_OFFSET(WebPConfig, segments);
  PRINT_OFFSET(WebPConfig, sns_strength);
  PRINT_OFFSET(WebPConfig, filter_strength);
  PRINT_OFFSET(WebPConfig, filter_sharpness);
  PRINT_OFFSET(WebPConfig, filter_type);
  PRINT_OFFSET(WebPConfig, autofilter);
  PRINT_OFFSET(WebPConfig, alpha_compression);
  PRINT_OFFSET(WebPConfig, alpha_filtering);
  PRINT_OFFSET(WebPConfig, alpha_quality);
  PRINT_OFFSET(WebPConfig, pass);
  PRINT_OFFSET(WebPConfig, show_compressed);
  PRINT_OFFSET(WebPConfig, preprocessing);
  PRINT_OFFSET(WebPConfig, partitions);
  PRINT_OFFSET(WebPConfig, partition_limit);
  PRINT_OFFSET(WebPConfig, emulate_jpeg_size);
  PRINT_OFFSET(WebPConfig, thread_level);
  PRINT_OFFSET(WebPConfig, low_memory);
  PRINT_OFFSET(WebPConfig, near_lossless);
  PRINT_OFFSET(WebPConfig, exact);
  PRINT_OFFSET(WebPConfig, use_delta_palette);
  PRINT_OFFSET(WebPConfig, use_sharp_yuv);
  PRINT_OFFSET(WebPConfig, qmin);
  PRINT_OFFSET(WebPConfig, qmax);

  PRINT_LAYOUT(WebPAuxStats);
  PRINT_OFFSET(WebPAuxStats, coded_size);
  PRINT_OFFSET(WebPAuxStats, PSNR);
  PRINT_OFFSET(WebPAuxStats, block_count);
  PRINT_OFFSET(WebPAuxStats, header_bytes);
  PRINT_OFFSET(WebPAuxStats, residual_bytes);
  PRINT_OFFSET(WebPAuxStats, segment_size);
  PRINT_OFFSET(WebPAuxStats, segment_quant);
  PRINT_OFFSET(WebPAuxStats, segment_level);
  PRINT_OFFSET(WebPAuxStats, alpha_data_size);
  PRINT_OFFSET(WebPAuxStats, layer_data_size);
  PRINT_OFFSET(WebPAuxStats, lossless_features);
  PRINT_OFFSET(WebPAuxStats, histogram_bits);
  PRINT_OFFSET(WebPAuxStats, transform_bits);
  PRINT_OFFSET(WebPAuxStats, cache_bits);
  PRINT_OFFSET(WebPAuxStats, palette_size);
  PRINT_OFFSET(WebPAuxStats, lossless_size);
  PRINT_OFFSET(WebPAuxStats, lossless_hdr_size);
  PRINT_OFFSET(WebPAuxStats, lossless_data_size);
  PRINT_OFFSET(WebPAuxStats, cross_color_transform_bits);
  PRINT_OFFSET(WebPAuxStats, pad);

  PRINT_LAYOUT(WebPAnimDecoderOptions);
  PRINT_OFFSET(WebPAnimDecoderOptions, color_mode);
  PRINT_OFFSET(WebPAnimDecoderOptions, use_threads);
  PRINT_OFFSET(WebPAnimDecoderOptions, padding);

  PRINT_LAYOUT(WebPAnimInfo);
  PRINT_OFFSET(WebPAnimInfo, canvas_width);
  PRINT_OFFSET(WebPAnimInfo, canvas_height);
  PRINT_OFFSET(WebPAnimInfo, loop_count);
  PRINT_OFFSET(WebPAnimInfo, bgcolor);
  PRINT_OFFSET(WebPAnimInfo, frame_count);
  PRINT_OFFSET(WebPAnimInfo, pad);
  return 0;
}
