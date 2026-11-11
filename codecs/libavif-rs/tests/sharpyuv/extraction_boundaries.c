/* Pinned-C oracle for cross_port.rs signed-layout cases. Compile against the
 * SharpYUV 0.4.2 reference built by tools/build_sharpyuv.py. */
#include <inttypes.h>
#include <stddef.h>
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

static int in_allocation(ptrdiff_t offset, size_t access, size_t size) {
  return offset >= 0 && access <= size && (size_t)offset <= size - access;
}

int main(void) {
#ifndef SHARPYUV_REGENERATE_HASHES
  const uint64_t expected[16] = {
      UINT64_C(0xa42a4a0c61e4d1de), UINT64_C(0xa860f9defcb63669),
      UINT64_C(0xf22dccbba09d4982), UINT64_C(0x11abe18f588e52a9),
      UINT64_C(0xa42a4a0c61e4d1de), UINT64_C(0x5bbf3f9a1664616c),
      UINT64_C(0xf22dccbba09d4982), UINT64_C(0xfdf29ae38a42b629),
      UINT64_C(0xdaf66228b0d778f5), UINT64_C(0x91a6296df83d58b2),
      UINT64_C(0x9f02a04a3f987860), UINT64_C(0x338e5c7750ac5232),
      UINT64_C(0x1aab832c2c5a22c8), UINT64_C(0x755f6aa9adb9e4f1),
      UINT64_C(0x7f390f81cc2d2cf8), UINT64_C(0xc64efdc8329c3212),
  };
#endif
  size_t case_index = 0;
  const size_t width = 3, height = 5;
  /* SharpYUV forms final input pointers two rows past the last pair and
   * final output pointers one row past the last row. Both directions need
   * guards even though those pointers are not dereferenced. */
  const size_t input_guard = 64, output_guard = 32;
  uint16_t input[128 + 64];
  for (size_t i = 0; i < 128 + 64; ++i) input[i] = UINT16_C(0xa5a5);
  for (size_t i = 0; i < 128; ++i) {
    input[input_guard / 2 + i] = (uint16_t)(i * 1009 + 0x8001);
  }
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
            origins[channel] = input_guard + channels[channel] +
                (reverse_rows ? (height - 1) * row : 0) +
                (reverse_pixels ? (width - 1) * step : 0);
          }
          const int signed_step = reverse_pixels ? -(int)step : (int)step;
          const int signed_row = reverse_rows ? -(int)row : (int)row;
          for (int channel = 0; channel < 3; ++channel) {
            const ptrdiff_t formed = (ptrdiff_t)origins[channel] +
                                     6 * signed_row;
            if (!in_allocation(formed, 0, sizeof(input))) return 6;
            for (size_t j = 0; j < height; ++j) {
              for (size_t i = 0; i < width; ++i) {
                const ptrdiff_t sample = (ptrdiff_t)origins[channel] +
                    (ptrdiff_t)j * signed_row + (ptrdiff_t)i * signed_step;
                if (!in_allocation(sample, 2, sizeof(input))) return 7;
              }
            }
          }
          const size_t unit = yuv_depth == 8 ? 1 : 2;
          const size_t y_row = 8 * unit, uv_row = 6 * unit;
          const size_t y_origin = output_guard +
              (reverse_rows ? (height - 1) * y_row : 0);
          const size_t uv_origin = output_guard +
              (reverse_rows ? 2 * uv_row : 0);
          const int y_stride = reverse_rows ? -(int)y_row : (int)y_row;
          const int uv_stride = reverse_rows ? -(int)uv_row : (int)uv_row;
          uint16_t y[64 + 32], u[32 + 32], v[32 + 32];
          const ptrdiff_t y_formed = (ptrdiff_t)y_origin +
                                     (ptrdiff_t)height * y_stride;
          const ptrdiff_t uv_formed = (ptrdiff_t)uv_origin +
                                      (ptrdiff_t)((height + 1) / 2) * uv_stride;
          if (!in_allocation(y_formed, 0, sizeof(y)) ||
              !in_allocation(uv_formed, 0, sizeof(u))) return 8;
          for (size_t j = 0; j < height; ++j) {
            for (size_t i = 0; i < width; ++i) {
              const ptrdiff_t sample = (ptrdiff_t)y_origin +
                  (ptrdiff_t)j * y_stride + (ptrdiff_t)i * (ptrdiff_t)unit;
              if (!in_allocation(sample, unit, sizeof(y))) return 9;
            }
          }
          for (size_t j = 0; j < (height + 1) / 2; ++j) {
            for (size_t i = 0; i < (width + 1) / 2; ++i) {
              const ptrdiff_t sample = (ptrdiff_t)uv_origin +
                  (ptrdiff_t)j * uv_stride + (ptrdiff_t)i * (ptrdiff_t)unit;
              if (!in_allocation(sample, unit, sizeof(u))) return 10;
            }
          }
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
#ifndef SHARPYUV_REGENERATE_HASHES
          if (hash != expected[case_index++]) return 4;
#else
          ++case_index;
#endif
        }
      }
    }
  }
  return case_index == 16 ? 0 : 5;
}
