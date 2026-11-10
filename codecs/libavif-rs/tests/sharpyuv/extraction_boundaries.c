/* Pinned-C oracle for cross_port.rs signed-layout cases. Compile against the
 * SharpYUV 0.4.2 reference built by tools/build_sharpyuv.py. */
#include <inttypes.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>
#include "sharpyuv/sharpyuv.h"
#include "sharpyuv/sharpyuv_csp.h"

static uint64_t hash_bytes(uint64_t hash, const void *data, size_t size) {
  const uint8_t *bytes = (const uint8_t *)data;
  for (size_t i = 0; i < size; ++i) {
    hash = (hash ^ bytes[i]) * UINT64_C(1099511628211);
  }
  return hash;
}

int main(void) {
  const uint64_t expected[16] = {
      UINT64_C(0x6115d527c3f8a95e), UINT64_C(0xe7c659af2ecb6ee9),
      UINT64_C(0x4b424b22f2a83d02), UINT64_C(0x01c517a52b574ba9),
      UINT64_C(0x6115d527c3f8a95e), UINT64_C(0x5535dadb0411e06c),
      UINT64_C(0x4b424b22f2a83d02), UINT64_C(0x7bc4036b53039e29),
      UINT64_C(0x97873e7a20d8c3b5), UINT64_C(0xf7ea3fa1cbfd65f2),
      UINT64_C(0x6b86e2f60f18cf60), UINT64_C(0xc9058676cd2bd032),
      UINT64_C(0x65411d9724e1b448), UINT64_C(0x39594bb76cd0e671),
      UINT64_C(0x0431410577ac9038), UINT64_C(0x38e11d20da46aa92),
  };
  size_t case_index = 0;
  const size_t width = 3, height = 5;
  uint16_t input[128];
  for (size_t i = 0; i < 128; ++i) input[i] = (uint16_t)(i * 1009 + 0x8001);
  const uint8_t *base = (const uint8_t *)input;
  const SharpYuvConversionMatrix *matrix =
      SharpYuvGetConversionMatrix(kSharpYuvMatrixWebp);
  SharpYuvOptions options;
  if (!SharpYuvOptionsInit(matrix, &options)) return 2;
  options.transfer_type = kSharpYuvTransferFunctionSrgb;
  for (int planar = 0; planar < 2; ++planar) {
    for (int reverse_rows = 0; reverse_rows < 2; ++reverse_rows) {
      for (int reverse_pixels = 0; reverse_pixels < 2; ++reverse_pixels) {
        for (int yuv_depth = 8; yuv_depth <= 10; yuv_depth += 2) {
          const size_t step = planar ? 2 : 6;
          const size_t row = planar ? 16 : 24;
          const size_t channels[3] = {0, planar ? 64 : 2,
                                      planar ? 128 : 4};
          size_t origins[3];
          for (int channel = 0; channel < 3; ++channel) {
            origins[channel] = channels[channel] +
                (reverse_rows ? (height - 1) * row : 0) +
                (reverse_pixels ? (width - 1) * step : 0);
          }
          const int signed_step = reverse_pixels ? -(int)step : (int)step;
          const int signed_row = reverse_rows ? -(int)row : (int)row;
          const size_t unit = yuv_depth == 8 ? 1 : 2;
          const size_t y_row = 8 * unit, uv_row = 6 * unit;
          const size_t y_origin = reverse_rows ? (height - 1) * y_row : 0;
          const size_t uv_origin = reverse_rows ? 2 * uv_row : 0;
          const int y_stride = reverse_rows ? -(int)y_row : (int)y_row;
          const int uv_stride = reverse_rows ? -(int)uv_row : (int)uv_row;
          uint16_t y[64], u[32], v[32];
          memset(y, 0xa5, sizeof(y));
          memset(u, 0xa5, sizeof(u));
          memset(v, 0xa5, sizeof(v));
          if (SharpYuvConvertWithOptions(
                  base + origins[0], base + origins[1], base + origins[2],
                  signed_step, signed_row, 16,
                  (uint8_t *)y + y_origin, y_stride,
                  (uint8_t *)u + uv_origin, uv_stride,
                  (uint8_t *)v + uv_origin, uv_stride,
                  yuv_depth, (int)width, (int)height, &options) != 1) return 3;
          uint64_t hash = UINT64_C(14695981039346656037);
          hash = hash_bytes(hash, y, sizeof(y));
          hash = hash_bytes(hash, u, sizeof(u));
          hash = hash_bytes(hash, v, sizeof(v));
          printf("%016" PRIx64 "\n", hash);
          if (hash != expected[case_index++]) return 4;
        }
      }
    }
  }
  return case_index == 16 ? 0 : 5;
}
