// WP5 probe: SharpYuvConvert and SharpYuvConvertWithOptions, run on the
// inventory of libavif-rs docs/SHARPYUV-VALIDATION.md, which
// tools/check_sharpyuv.py executes, with the generator of wp5_rng.h in place of
// Python's random module.
//
// Groups, in this order (1,296 in all):
//   1. concurrent-cold-initialization-24: 24 conversions on 8 threads, first in
//      the process so that the library initializes concurrently.
//   2. convert-rgbR-yuvY-tfT-matrixM-layoutL-WxH: every RGB depth, YUV depth,
//      transfer function (1 and 4 to 18), predefined matrix and layout, nine
//      shapes, 960 groups. The seed is the group index.
//   3. seeded-I: 256 conversions whose parameters come from one generator stream
//      (rgb, yuv, transfer, matrix, layout, width, height, seed).
//   4. reject-I: 15 calls with one argument replaced by a value the C function
//      rejects. The six NULL-pointer rejections are not printed: the Rust API takes
//      references, so NULL exists only at the C ABI (docs/status/wp5.md).
//   5. flipped-I: 64 seeded conversions whose input rows, input pixels and output
//      rows may run backwards (negative strides and steps, as ADR 0003 allows). The
//      origin is then the last row or pixel, and the logical image is the same.
//
// Each group prints code=RESULT, then y=, u= and v= with the whole output buffer
// of each plane in hex, guard bytes included. The input buffer must come back
// unchanged, and every output guard must keep its value; a violation exits with a
// status other than 0. For transfer 13 without an override, SharpYuvConvert must
// produce the same code and bytes as SharpYuvConvertWithOptions (exit 5 otherwise).
//
// Output: one group per line set, as above. Exit status 2 is a usage error.

#include <pthread.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "sharpyuv/sharpyuv.h"
#include "sharpyuv/sharpyuv_csp.h"
#include "wp5_rng.h"

#define kGuardBytes 32
#define kGuard 0xA5

typedef struct {
  int rgb, yuv, transfer, matrix_type, layout, width, height;
  uint64_t seed;
  int has_override, override_index;
  long long override_value;
  // Negative steps and strides: the input rows, the input pixels and the output
  // rows run backwards, with the origin at the last row or pixel.
  int flip_in_rows, flip_in_cols, flip_out_rows;
} Case;

typedef struct {
  int code;
  uint8_t* buffers[3];  // whole plane buffers, guard bytes included
  size_t sizes[3];
} Result;

static void ResultFree(Result* result) {
  int i;
  for (i = 0; i < 3; ++i) free(result->buffers[i]);
}

static void WriteSample(uint8_t* at, uint64_t value, int unit) {
  if (unit == 1) {
    *at = (uint8_t)value;
  } else {
    const uint16_t sample = (uint16_t)value;
    memcpy(at, &sample, 2);
  }
}

// Runs one conversion. Returns 0 on success, or a nonzero status when the probe's
// own checks fail; the result then holds the buffers of the failing call.
static int RunCase(const Case* c, int plain, Result* result) {
  const int unit = (c->rgb == 8) ? 1 : 2;
  const int step = unit * ((c->layout == 2) ? 1 : (c->layout == 1) ? 4 : 3);
  const int stride = c->width * step + 6 * unit;
  const size_t span = (size_t)stride * (size_t)c->height;
  const size_t raw_samples = (3 * span + 64 + 1) / 2;
  const size_t raw_bytes = 2 * raw_samples;
  const uint64_t max_value = ((uint64_t)1 << c->rgb) - 1;
  const int out_unit = (c->yuv == 8) ? 1 : 2;
  const size_t offsets[3] = {0, (c->layout == 2) ? span : (size_t)unit,
                             (c->layout == 2) ? 2 * span : (size_t)(2 * unit)};
  uint8_t* raw = (uint8_t*)malloc(raw_bytes);
  uint8_t* before = (uint8_t*)malloc(raw_bytes);
  uint8_t* base;
  Wp5Rng rng;
  int x, y, ch, p, status = 0;
  int plane_w[3], plane_h[3], plane_stride[3];
  void* plane_ptr[3];
  const SharpYuvConversionMatrix* matrix;
  SharpYuvOptions options;
  const uint8_t* g_ptr;
  const uint8_t* b_ptr;
  int rgb_step = step, rgb_stride = stride, rgb = c->rgb, yuv = c->yuv;
  int width = c->width, height = c->height, code;
  int y_stride, u_stride, v_stride;
  void* y_ptr;
  void* u_ptr;
  void* v_ptr;
  const uint8_t* rgb_r;
  if (raw == NULL || before == NULL) exit(2);

  memset(raw, kGuard, raw_bytes);
  base = raw + kGuardBytes;
  rng.state = c->seed;
  for (y = 0; y < c->height; ++y) {
    for (x = 0; x < c->width; ++x) {
      for (ch = 0; ch < 3; ++ch) {
        uint64_t value = Wp5RngBelow(&rng, (uint64_t)1 << c->rgb);
        if (c->seed % 4 == 0) {
          value = ((x + y + ch) % 2) ? max_value : 0;
        }
        const int phys_y = c->flip_in_rows ? (c->height - 1 - y) : y;
        const int phys_x = c->flip_in_cols ? (c->width - 1 - x) : x;
        WriteSample(base + offsets[ch] + (size_t)phys_y * stride + (size_t)phys_x * step,
                    value, unit);
      }
    }
  }
  memcpy(before, raw, raw_bytes);

  plane_w[0] = c->width;
  plane_h[0] = c->height;
  plane_w[1] = plane_w[2] = (c->width + 1) / 2;
  plane_h[1] = plane_h[2] = (c->height + 1) / 2;
  for (p = 0; p < 3; ++p) {
    const size_t samples = (size_t)((plane_w[p] + 5) * out_unit) * (size_t)plane_h[p];
    const size_t bytes = 2 * ((samples + 65) / 2);
    plane_stride[p] = (plane_w[p] + 5) * out_unit;
    result->buffers[p] = (uint8_t*)malloc(bytes);
    if (result->buffers[p] == NULL) exit(2);
    memset(result->buffers[p], kGuard, bytes);
    result->sizes[p] = bytes;
    plane_ptr[p] = result->buffers[p] + kGuardBytes;
    if (c->flip_out_rows) {
      plane_ptr[p] = (uint8_t*)plane_ptr[p] + (size_t)(plane_h[p] - 1) * plane_stride[p];
    }
  }

  matrix = SharpYuvGetConversionMatrix((SharpYuvMatrixType)c->matrix_type);
  if (matrix == NULL || !SharpYuvOptionsInit(matrix, &options)) exit(3);
  options.transfer_type = (SharpYuvTransferFunctionType)c->transfer;

  {
    const size_t row_adjust = c->flip_in_rows ? (size_t)(c->height - 1) * stride : 0;
    const size_t col_adjust = c->flip_in_cols ? (size_t)(c->width - 1) * step : 0;
    rgb_step = c->flip_in_cols ? -step : step;
    rgb_stride = c->flip_in_rows ? -stride : stride;
    rgb_r = base + offsets[0] + row_adjust + col_adjust;
    g_ptr = base + offsets[1] + row_adjust + col_adjust;
    b_ptr = base + offsets[2] + row_adjust + col_adjust;
  }
  y_ptr = plane_ptr[0];
  u_ptr = plane_ptr[1];
  v_ptr = plane_ptr[2];
  y_stride = c->flip_out_rows ? -plane_stride[0] : plane_stride[0];
  u_stride = c->flip_out_rows ? -plane_stride[1] : plane_stride[1];
  v_stride = c->flip_out_rows ? -plane_stride[2] : plane_stride[2];

  if (c->has_override) {
    switch (c->override_index) {
      case 0: rgb_r = NULL; break;
      case 1: g_ptr = NULL; break;
      case 2: b_ptr = NULL; break;
      case 3: rgb_step = (int)c->override_value; break;
      case 4: rgb_stride = (int)c->override_value; break;
      case 5: rgb = (int)c->override_value; break;
      case 6: y_ptr = NULL; break;
      case 7: y_stride = (int)c->override_value; break;
      case 8: u_ptr = NULL; break;
      case 9: u_stride = (int)c->override_value; break;
      case 10: v_ptr = NULL; break;
      case 11: v_stride = (int)c->override_value; break;
      case 12: yuv = (int)c->override_value; break;
      case 13: width = (int)c->override_value; break;
      case 14: height = (int)c->override_value; break;
      default: exit(2);
    }
  }

  if (plain) {
    code = SharpYuvConvert(rgb_r, g_ptr, b_ptr, rgb_step, rgb_stride, rgb, y_ptr,
                           y_stride, u_ptr, u_stride, v_ptr, v_stride, yuv, width,
                           height, matrix);
  } else {
    code = SharpYuvConvertWithOptions(rgb_r, g_ptr, b_ptr, rgb_step, rgb_stride, rgb,
                                      y_ptr, y_stride, u_ptr, u_stride, v_ptr,
                                      v_stride, yuv, width, height, &options);
  }
  result->code = code;

  if (memcmp(raw, before, raw_bytes) != 0) status = 3;
  for (p = 0; p < 3; ++p) {
    const uint8_t* buffer = result->buffers[p];
    const size_t bytes = result->sizes[p];
    int i;
    for (i = 0; i < kGuardBytes; ++i) {
      if (buffer[i] != kGuard || buffer[bytes - 1 - i] != kGuard) status = 4;
    }
  }
  free(raw);
  free(before);
  return status;
}

// Runs the conversion through SharpYuvConvertWithOptions, and for the sRGB transfer
// without an override also through SharpYuvConvert, which must agree.
static int RunChecked(const Case* c, Result* result) {
  int status = RunCase(c, 0, result);
  Result plain;
  int p;
  if (status != 0) return status;
  if (c->transfer != 13 || c->has_override) return 0;
  // The plain API converts into fresh buffers, which RunCase allocates and fills.
  status = RunCase(c, 1, &plain);
  if (status != 0) return status;
  if (plain.code != result->code) return 5;
  for (p = 0; p < 3; ++p) {
    if (plain.sizes[p] != result->sizes[p]) return 5;
    if (memcmp(plain.buffers[p], result->buffers[p], plain.sizes[p]) != 0) return 5;
  }
  ResultFree(&plain);
  return 0;
}

static void PrintResult(const char* name, const Result* result) {
  printf("group=%s\n", name);
  printf("code=%d\n", result->code);
  ProbePrintHex("y", result->buffers[0], result->sizes[0]);
  ProbePrintHex("u", result->buffers[1], result->sizes[1]);
  ProbePrintHex("v", result->buffers[2], result->sizes[2]);
}

static int PrintCase(const char* name, const Case* c) {
  Result result;
  const int status = RunChecked(c, &result);
  if (status != 0) {
    fprintf(stderr, "convert: %s failed with status %d\n", name, status);
    exit(status);
  }
  PrintResult(name, &result);
  ResultFree(&result);
  return 0;
}

typedef struct {
  Case c;
  Result results[3];
  int status;
} Job;

static void* ThreadMain(void* argument) {
  Job* job = (Job*)argument;
  int i;
  job->status = 0;
  for (i = 0; i < 3 && job->status == 0; ++i) {
    job->status = RunChecked(&job->c, &job->results[i]);
  }
  return NULL;
}

static void ConcurrentColdInitialization(void) {
  Job jobs[8];
  pthread_t threads[8];
  int t, i;
  Case c = {8, 8, 13, 0, 0, 17, 9, 42, 0, 0, 0};
  for (t = 0; t < 8; ++t) {
    jobs[t].c = c;
    jobs[t].status = 0;
    if (pthread_create(&threads[t], NULL, ThreadMain, &jobs[t]) != 0) exit(2);
  }
  for (t = 0; t < 8; ++t) pthread_join(threads[t], NULL);
  // Every conversion must produce the same code and bytes.
  for (t = 0; t < 8; ++t) {
    if (jobs[t].status != 0) {
      fprintf(stderr, "convert: concurrent conversion failed with %d\n", jobs[t].status);
      exit(jobs[t].status);
    }
  }
  for (t = 0; t < 8; ++t) {
    for (i = 0; i < 3; ++i) {
      int p;
      Result* r = &jobs[t].results[i];
      Result* first = &jobs[0].results[0];
      if (r->code != first->code) exit(6);
      for (p = 0; p < 3; ++p) {
        if (r->sizes[p] != first->sizes[p] ||
            memcmp(r->buffers[p], first->buffers[p], r->sizes[p]) != 0) {
          exit(6);
        }
      }
    }
  }
  PrintResult("concurrent-cold-initialization-24", &jobs[0].results[0]);
  for (t = 0; t < 8; ++t) {
    for (i = 0; i < 3; ++i) ResultFree(&jobs[t].results[i]);
  }
}

int main(int argc, char* argv[]) {
  static const int kRgbDepths[4] = {8, 10, 12, 16};
  static const int kYuvDepths[3] = {8, 10, 12};
  static const int kTransfers[16] = {1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18};
  static const int kShapes[9][2] = {{1, 1}, {1, 9}, {9, 1}, {2, 2}, {3, 5},
                                    {15, 7}, {16, 8}, {17, 9}, {33, 11}};
  // Overrides of the 15 rejected calls, by index into the argument list of
  // SharpYuvConvertWithOptions (indexes 0, 1, 2, 6, 8 and 10 are pointers).
  static const int kRejectIndex[15] = {13, 13, 13, 14, 14, 14, 5, 5, 12, 12, 3, 4, 7, 9, 11};
  static const long long kRejectValue[15] = {0, -1, 0x7fffffffLL, 0, -1, 0x7fffffffLL,
                                             9, 15, 9, 11, 3, 3, 3, 3, 3};
  char name[96];
  int index, i, rgb_i, yuv_i, t_i, m;
  Wp5Rng rng;
  ProbeCheckArgs(argc, 0, 0, "convert");

  ConcurrentColdInitialization();

  index = 0;
  for (rgb_i = 0; rgb_i < 4; ++rgb_i) {
    for (yuv_i = 0; yuv_i < 3; ++yuv_i) {
      for (t_i = 0; t_i < 16; ++t_i) {
        for (m = 0; m < 5; ++m) {
          Case c = {0};
          c.rgb = kRgbDepths[rgb_i];
          c.yuv = kYuvDepths[yuv_i];
          c.transfer = kTransfers[t_i];
          c.matrix_type = m;
          c.layout = index % 3;
          c.width = kShapes[index % 9][0];
          c.height = kShapes[index % 9][1];
          c.seed = (uint64_t)index;
          c.has_override = 0;
          c.override_index = 0;
          c.override_value = 0;
          snprintf(name, sizeof(name), "convert-rgb%d-yuv%d-tf%d-matrix%d-layout%d-%dx%d",
                   c.rgb, c.yuv, c.transfer, c.matrix_type, c.layout, c.width, c.height);
          PrintCase(name, &c);
          ++index;
        }
      }
    }
  }

  rng.state = 0x41564946ULL;
  for (i = 0; i < 256; ++i) {
    Case c = {0};
    c.rgb = kRgbDepths[Wp5RngBelow(&rng, 4)];
    c.yuv = kYuvDepths[Wp5RngBelow(&rng, 3)];
    c.transfer = kTransfers[Wp5RngBelow(&rng, 16)];
    c.matrix_type = (int)Wp5RngBelow(&rng, 5);
    c.layout = (int)Wp5RngBelow(&rng, 3);
    c.width = 1 + (int)Wp5RngBelow(&rng, 64);
    c.height = 1 + (int)Wp5RngBelow(&rng, 32);
    c.seed = Wp5RngBelow(&rng, (uint64_t)1 << 30);
    c.has_override = 0;
    c.override_index = 0;
    c.override_value = 0;
    snprintf(name, sizeof(name), "seeded-%d", i);
    PrintCase(name, &c);
  }

  for (i = 0; i < 15; ++i) {
    Case c = {10, 10, 13, 0, 0, 3, 3, 111, 1, kRejectIndex[i], kRejectValue[i]};
    snprintf(name, sizeof(name), "reject-%d", i);
    PrintCase(name, &c);
  }

  // The six NULL-pointer rejections of the inventory, not printed: each must
  // return 0 (the Rust API has no NULL; see docs/status/wp5.md).
  for (i = 0; i < 6; ++i) {
    static const int kNullIndex[6] = {0, 1, 2, 6, 8, 10};
    Case c = {10, 10, 13, 0, 0, 3, 3, 111, 1, kNullIndex[i], 0};
    Result result;
    if (RunCase(&c, 0, &result) != 0 || result.code != 0) {
      fprintf(stderr, "convert: a NULL argument %d was accepted\n", kNullIndex[i]);
      return 4;
    }
    ResultFree(&result);
  }

  // Negative steps and strides, from their own generator stream: 64 seeded
  // conversions with each flip drawn at random.
  rng.state = 0x666c6970ULL;
  for (i = 0; i < 64; ++i) {
    Case c = {0};
    c.rgb = kRgbDepths[Wp5RngBelow(&rng, 4)];
    c.yuv = kYuvDepths[Wp5RngBelow(&rng, 3)];
    c.transfer = kTransfers[Wp5RngBelow(&rng, 16)];
    c.matrix_type = (int)Wp5RngBelow(&rng, 5);
    c.layout = (int)Wp5RngBelow(&rng, 3);
    c.width = 1 + (int)Wp5RngBelow(&rng, 24);
    c.height = 1 + (int)Wp5RngBelow(&rng, 16);
    c.seed = Wp5RngBelow(&rng, (uint64_t)1 << 30);
    c.flip_in_rows = (int)Wp5RngBelow(&rng, 2);
    c.flip_in_cols = (int)Wp5RngBelow(&rng, 2);
    c.flip_out_rows = (int)Wp5RngBelow(&rng, 2);
    snprintf(name, sizeof(name), "flipped-%d", i);
    PrintCase(name, &c);
  }
  return 0;
}
