// WP15 probe: size, alignment and field offsets of the mux.h structs:
// WebPMuxFrameInfo, which src/abi/mux.rs mirrors with repr(C), and
// WebPMuxAnimParams and WebPAnimEncoderOptions, which the C ABI shares with
// src/webp/mux.rs (ADR 0004); the sizes of the enums that the ABI passes
// (WebPMuxError, WebPChunkId); and the values of WEBP_MUX_ABI_VERSION and the
// enumerators. Prints one line per struct, member and value;
// tests/wp15_abi_layout.rs compares them with the Rust types.

#include <stddef.h>
#include <stdio.h>

#include "src/webp/mux.h"
#include "probe_support.h"

#define STRUCT(T) printf(#T "=%zu,%zu\n", sizeof(T), _Alignof(T))
#define FIELD(T, F) printf(#T "." #F "=%zu\n", offsetof(T, F))
#define VALUE(V) printf(#V "=%d\n", (int)(V))

int main(int argc, char* argv[]) {
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "layout");
  STRUCT(WebPMuxFrameInfo);
  FIELD(WebPMuxFrameInfo, bitstream);
  FIELD(WebPMuxFrameInfo, x_offset);
  FIELD(WebPMuxFrameInfo, y_offset);
  FIELD(WebPMuxFrameInfo, duration);
  FIELD(WebPMuxFrameInfo, id);
  FIELD(WebPMuxFrameInfo, dispose_method);
  FIELD(WebPMuxFrameInfo, blend_method);
  FIELD(WebPMuxFrameInfo, pad);
  STRUCT(WebPMuxAnimParams);
  FIELD(WebPMuxAnimParams, bgcolor);
  FIELD(WebPMuxAnimParams, loop_count);
  STRUCT(WebPAnimEncoderOptions);
  FIELD(WebPAnimEncoderOptions, anim_params);
  FIELD(WebPAnimEncoderOptions, minimize_size);
  FIELD(WebPAnimEncoderOptions, kmin);
  FIELD(WebPAnimEncoderOptions, kmax);
  FIELD(WebPAnimEncoderOptions, allow_mixed);
  FIELD(WebPAnimEncoderOptions, verbose);
  FIELD(WebPAnimEncoderOptions, padding);
  STRUCT(WebPMuxError);
  STRUCT(WebPChunkId);
  VALUE(WEBP_MUX_ABI_VERSION);
  VALUE(WEBP_MUX_OK);
  VALUE(WEBP_MUX_NOT_FOUND);
  VALUE(WEBP_MUX_INVALID_ARGUMENT);
  VALUE(WEBP_MUX_BAD_DATA);
  VALUE(WEBP_MUX_MEMORY_ERROR);
  VALUE(WEBP_MUX_NOT_ENOUGH_DATA);
  VALUE(WEBP_CHUNK_VP8X);
  VALUE(WEBP_CHUNK_ICCP);
  VALUE(WEBP_CHUNK_ANIM);
  VALUE(WEBP_CHUNK_ANMF);
  VALUE(WEBP_CHUNK_DEPRECATED);
  VALUE(WEBP_CHUNK_ALPHA);
  VALUE(WEBP_CHUNK_IMAGE);
  VALUE(WEBP_CHUNK_EXIF);
  VALUE(WEBP_CHUNK_XMP);
  VALUE(WEBP_CHUNK_UNKNOWN);
  VALUE(WEBP_CHUNK_NIL);
  VALUE((WebPChunkId)-1 > 0);
  VALUE((WebPMuxError)-1 > 0);
  return 0;
}
