// WP10 probe: the encoder DSP functions of src/dsp/enc.c and src/dsp/quant.h
// after VP8EncDspInit (ADR 0002).
//
// Standard input is a sequence of fixed-size records:
//
//   op (int32), p[4] (int32),
//   a[16 * BPS], b[16 * BPS], d[16 * BPS] (bytes),
//   in[256] (int16), w[16] (uint16),
//   q[16] iq[16] (uint16), bias[16] zthresh[16] (uint32), sharpen[16] (uint16)
//
// all little-endian. The probe applies operation `op` and prints one line
// `r=` with its results in hex: int16/uint16 values as four digits, int32
// values as eight, bytes as two. tests/wp10_dsp.rs builds the records and the
// expected lines.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dsp/dsp.h"
#include "src/dsp/quant.h"
#include "src/enc/vp8i_enc.h"
#include "src/utils/utils.h"

enum {
  OP_FTRANSFORM = 0,
  OP_FTRANSFORM2,
  OP_FTRANSFORM_WHT,
  OP_ITRANSFORM,
  OP_SSE,
  OP_TDISTO,
  OP_MEAN16X4,
  OP_HISTOGRAM,
  OP_QUANTIZE,
  OP_PRED16,
  OP_PREDC8,
  OP_PRED4,
  OP_COPY,
  OP_IS_FLAT,
  OP_SET_HISTOGRAM_DATA
};

#define AREA (16 * BPS)

typedef struct {
  int32_t op, p[4];
  uint8_t a[AREA], b[AREA], d[AREA];
  int16_t in[256];
  uint16_t w[16];
  VP8Matrix mtx;
} Record;

static int ReadBytes(void* dst, size_t size) {
  return fread(dst, 1, size, stdin) == size;
}

static int ReadRecord(Record* r) {
  uint8_t raw[4];
  int i;
  if (!ReadBytes(raw, 4)) return 0;
  r->op = (int32_t)((uint32_t)raw[0] | ((uint32_t)raw[1] << 8) |
                    ((uint32_t)raw[2] << 16) | ((uint32_t)raw[3] << 24));
  for (i = 0; i < 4; ++i) {
    if (!ReadBytes(raw, 4)) exit(2);
    r->p[i] = (int32_t)((uint32_t)raw[0] | ((uint32_t)raw[1] << 8) |
                        ((uint32_t)raw[2] << 16) | ((uint32_t)raw[3] << 24));
  }
  if (!ReadBytes(r->a, AREA) || !ReadBytes(r->b, AREA) ||
      !ReadBytes(r->d, AREA)) {
    exit(2);
  }
  // The host is little-endian (the reference configuration).
  if (!ReadBytes(r->in, sizeof(r->in)) || !ReadBytes(r->w, sizeof(r->w)) ||
      !ReadBytes(r->mtx.q, sizeof(r->mtx.q)) ||
      !ReadBytes(r->mtx.iq, sizeof(r->mtx.iq)) ||
      !ReadBytes(r->mtx.bias, sizeof(r->mtx.bias)) ||
      !ReadBytes(r->mtx.zthresh, sizeof(r->mtx.zthresh)) ||
      !ReadBytes(r->mtx.sharpen, sizeof(r->mtx.sharpen))) {
    exit(2);
  }
  return 1;
}

static void PrintI16s(const int16_t* v, int n) {
  int i;
  for (i = 0; i < n; ++i) printf("%04x", (uint16_t)v[i]);
}

static void PrintI32(int32_t v) { printf("%08x", (uint32_t)v); }

static void PrintBytes(const uint8_t* v, int n) {
  int i;
  for (i = 0; i < n; ++i) printf("%02x", v[i]);
}

int main(int argc, char* argv[]) {
  static Record r;
  ProbeCheckArgs(argc, 0, 0, "dsp < records.bin");
  (void)argv;
  VP8EncDspInit();
  while (ReadRecord(&r)) {
    printf("r=");
    switch (r.op) {
      case OP_FTRANSFORM: {
        int16_t out[16];
        VP8FTransform(r.a, r.b, out);
        PrintI16s(out, 16);
        break;
      }
      case OP_FTRANSFORM2: {
        int16_t out[32];
        VP8FTransform2(r.a, r.b, out);
        PrintI16s(out, 32);
        break;
      }
      case OP_FTRANSFORM_WHT: {
        int16_t out[16];
        VP8FTransformWHT(r.in, out);
        PrintI16s(out, 16);
        break;
      }
      case OP_ITRANSFORM:
        VP8ITransform(r.a, r.in, r.d, r.p[0]);
        PrintBytes(r.d, 4 * BPS);
        break;
      case OP_SSE:
        PrintI32(VP8SSE16x16(r.a, r.b));
        PrintI32(VP8SSE16x8(r.a, r.b));
        PrintI32(VP8SSE8x8(r.a, r.b));
        PrintI32(VP8SSE4x4(r.a, r.b));
        break;
      case OP_TDISTO:
        PrintI32(VP8TDisto4x4(r.a, r.b, r.w));
        PrintI32(VP8TDisto16x16(r.a, r.b, r.w));
        break;
      case OP_MEAN16X4: {
        uint32_t dc[4];
        int i;
        VP8Mean16x4(r.a, dc);
        for (i = 0; i < 4; ++i) PrintI32((int32_t)dc[i]);
        break;
      }
      case OP_HISTOGRAM: {
        VP8Histogram histo;
        histo.max_value = 0;
        histo.last_non_zero = 1;
        VP8CollectHistogram(r.a, r.b, r.p[0], r.p[1], &histo);
        PrintI32(histo.max_value);
        PrintI32(histo.last_non_zero);
        break;
      }
      case OP_QUANTIZE: {
        int16_t in[32], out[32];
        int ret;
        memcpy(in, r.in, sizeof(in));
        ret = VP8EncQuantizeBlock(in, out, &r.mtx);
        PrintI32(ret);
        PrintI16s(in, 16);
        PrintI16s(out, 16);
        memcpy(in, r.in, sizeof(in));
        ret = VP8EncQuantize2Blocks(in, out, &r.mtx);
        PrintI32(ret);
        PrintI16s(in, 32);
        PrintI16s(out, 32);
        memcpy(in, r.in, sizeof(in));
        ret = VP8EncQuantizeBlockWHT(in, out, &r.mtx);
        PrintI32(ret);
        PrintI16s(in, 16);
        PrintI16s(out, 16);
        break;
      }
      case OP_PRED16:
      case OP_PREDC8: {
        // left[-1..] is b[p[2] - 1 ..], top is b[p[3] ..]; p[0] and p[1] say
        // whether each one is present. libwebp's encoder passes 32-byte
        // aligned rows (iterator_enc.c: y_left = WEBP_ALIGN(yuv_left_mem + 1);
        // webp_enc.c: y_top = WEBP_ALIGN(mem)), and the SSE2 luma predictors
        // read them with _mm_load_si128 (enc_sse2.c DC16_SSE2, VE16_SSE2,
        // TM_SSE2), so the probe copies the same bytes to aligned rows.
        // left[-1..31] and top[0..31] cover luma and both chroma blocks.
        static uint8_t dst[PRED_SIZE_ENC];
        static uint8_t left_mem[1 + 32 + WEBP_ALIGN_CST];
        static uint8_t top_mem[32 + WEBP_ALIGN_CST];
        uint8_t* const left_row = (uint8_t*)WEBP_ALIGN(left_mem + 1);
        uint8_t* const top_row = (uint8_t*)WEBP_ALIGN(top_mem);
        const uint8_t* left = NULL;
        const uint8_t* top = NULL;
        if (r.p[0]) {
          memcpy(left_row - 1, r.b + r.p[2] - 1, 1 + 32);
          left = left_row;
        }
        if (r.p[1]) {
          memcpy(top_row, r.b + r.p[3], 32);
          top = top_row;
        }
        memset(dst, 0x5a, sizeof(dst));
        if (r.op == OP_PRED16) {
          VP8EncPredLuma16(dst, left, top);
        } else {
          VP8EncPredChroma8(dst, left, top);
        }
        PrintBytes(dst, PRED_SIZE_ENC);
        break;
      }
      case OP_PRED4: {
        static uint8_t dst[PRED_SIZE_ENC];
        memset(dst, 0x5a, sizeof(dst));
        VP8EncPredLuma4(dst, r.b + r.p[0]);
        PrintBytes(dst, PRED_SIZE_ENC);
        break;
      }
      case OP_COPY: {
        uint8_t dst[AREA];
        memcpy(dst, r.d, AREA);
        VP8Copy4x4(r.a, dst);
        PrintBytes(dst, 4 * BPS);
        memcpy(dst, r.d, AREA);
        VP8Copy16x8(r.a, dst);
        PrintBytes(dst, 8 * BPS);
        break;
      }
      case OP_IS_FLAT:
        PrintI32(IsFlat(r.in, r.p[0], r.p[1]));
        PrintI32(IsFlatSource16(r.a));
        break;
      case OP_SET_HISTOGRAM_DATA: {
        int distribution[MAX_COEFF_THRESH + 1];
        VP8Histogram histo;
        int i;
        for (i = 0; i <= MAX_COEFF_THRESH; ++i) distribution[i] = r.in[i];
        VP8SetHistogramData(distribution, &histo);
        PrintI32(histo.max_value);
        PrintI32(histo.last_non_zero);
        break;
      }
      default:
        return 2;
    }
    printf("\n");
  }
  return 0;
}
