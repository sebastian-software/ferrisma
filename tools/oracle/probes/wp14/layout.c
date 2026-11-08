// WP14 probe: size, alignment and field offsets of the demux.h structs:
// WebPIterator, WebPChunkIterator and WebPData, which src/abi/demux.rs mirrors
// with repr(C), and WebPAnimDecoderOptions and WebPAnimInfo, which the C ABI
// shares with src/webp/demux.rs (ADR 0004); and the sizes of the enums that the
// ABI passes. Prints one line per struct and member; tests/wp14_abi_layout.rs
// compares them with the Rust types.

#include <stddef.h>
#include <stdio.h>

#include "src/webp/demux.h"
#include "probe_support.h"

#define STRUCT(T) printf(#T "=%zu,%zu\n", sizeof(T), _Alignof(T))
#define FIELD(T, F) printf(#T "." #F "=%zu\n", offsetof(T, F))
#define TYPE(T) printf(#T "=%zu,%zu\n", sizeof(T), _Alignof(T))

int main(int argc, char* argv[]) {
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "layout");
  STRUCT(WebPData);
  FIELD(WebPData, bytes);
  FIELD(WebPData, size);
  STRUCT(WebPIterator);
  FIELD(WebPIterator, frame_num);
  FIELD(WebPIterator, num_frames);
  FIELD(WebPIterator, x_offset);
  FIELD(WebPIterator, y_offset);
  FIELD(WebPIterator, width);
  FIELD(WebPIterator, height);
  FIELD(WebPIterator, duration);
  FIELD(WebPIterator, dispose_method);
  FIELD(WebPIterator, complete);
  FIELD(WebPIterator, fragment);
  FIELD(WebPIterator, has_alpha);
  FIELD(WebPIterator, blend_method);
  FIELD(WebPIterator, pad);
  FIELD(WebPIterator, private_);
  STRUCT(WebPChunkIterator);
  FIELD(WebPChunkIterator, chunk_num);
  FIELD(WebPChunkIterator, num_chunks);
  FIELD(WebPChunkIterator, chunk);
  FIELD(WebPChunkIterator, pad);
  FIELD(WebPChunkIterator, private_);
  STRUCT(WebPAnimDecoderOptions);
  FIELD(WebPAnimDecoderOptions, color_mode);
  FIELD(WebPAnimDecoderOptions, use_threads);
  FIELD(WebPAnimDecoderOptions, padding);
  STRUCT(WebPAnimInfo);
  FIELD(WebPAnimInfo, canvas_width);
  FIELD(WebPAnimInfo, canvas_height);
  FIELD(WebPAnimInfo, loop_count);
  FIELD(WebPAnimInfo, bgcolor);
  FIELD(WebPAnimInfo, frame_count);
  FIELD(WebPAnimInfo, pad);
  TYPE(WebPDemuxState);
  TYPE(WebPFormatFeature);
  TYPE(WebPMuxAnimDispose);
  TYPE(WebPMuxAnimBlend);
  TYPE(WEBP_CSP_MODE);
  return 0;
}
