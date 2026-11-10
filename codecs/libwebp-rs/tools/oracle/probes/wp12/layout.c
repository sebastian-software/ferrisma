// WP12 probe: size, alignment and field offsets of the decode.h structs with
// pointers, which src/abi/decode.rs mirrors with repr(C) (ADR 0004). Prints
// one line per struct and member; tests/wp12_abi_layout.rs compares them with
// the Rust mirrors.

#include <stddef.h>
#include <stdio.h>

#include "src/webp/decode.h"
#include "probe_support.h"

#define STRUCT(T) printf(#T "=%zu,%zu\n", sizeof(T), _Alignof(T))
#define FIELD(T, F) printf(#T "." #F "=%zu\n", offsetof(T, F))

int main(int argc, char* argv[]) {
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "layout");
  STRUCT(WebPRGBABuffer);
  FIELD(WebPRGBABuffer, rgba);
  FIELD(WebPRGBABuffer, stride);
  FIELD(WebPRGBABuffer, size);
  STRUCT(WebPYUVABuffer);
  FIELD(WebPYUVABuffer, y);
  FIELD(WebPYUVABuffer, u);
  FIELD(WebPYUVABuffer, v);
  FIELD(WebPYUVABuffer, a);
  FIELD(WebPYUVABuffer, y_stride);
  FIELD(WebPYUVABuffer, u_stride);
  FIELD(WebPYUVABuffer, v_stride);
  FIELD(WebPYUVABuffer, a_stride);
  FIELD(WebPYUVABuffer, y_size);
  FIELD(WebPYUVABuffer, u_size);
  FIELD(WebPYUVABuffer, v_size);
  FIELD(WebPYUVABuffer, a_size);
  STRUCT(WebPDecBuffer);
  FIELD(WebPDecBuffer, colorspace);
  FIELD(WebPDecBuffer, width);
  FIELD(WebPDecBuffer, height);
  FIELD(WebPDecBuffer, is_external_memory);
  FIELD(WebPDecBuffer, u);
  FIELD(WebPDecBuffer, u.RGBA);
  FIELD(WebPDecBuffer, u.YUVA);
  FIELD(WebPDecBuffer, pad);
  FIELD(WebPDecBuffer, private_memory);
  printf("WebPDecBuffer.u=%zu,%zu\n", sizeof(((WebPDecBuffer*)0)->u),
         _Alignof(__typeof__(((WebPDecBuffer*)0)->u)));
  STRUCT(WebPDecoderConfig);
  FIELD(WebPDecoderConfig, input);
  FIELD(WebPDecoderConfig, output);
  FIELD(WebPDecoderConfig, options);
  STRUCT(WebPBitstreamFeatures);
  STRUCT(WebPDecoderOptions);
  printf("WEBP_CSP_MODE=%zu,%zu\n", sizeof(WEBP_CSP_MODE),
         _Alignof(WEBP_CSP_MODE));
  printf("VP8StatusCode=%zu,%zu\n", sizeof(VP8StatusCode),
         _Alignof(VP8StatusCode));
  return 0;
}
