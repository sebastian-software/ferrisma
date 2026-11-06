// WP3 probe: the lossless DSP functions of src/dsp/lossless.c (ADR 0002). It
// reads commands from standard input and prints what the library computes.
// Pixel buffers are one array of 32-bit values; positions index into it, as the
// decoder's pointers point into its pixel buffer.
//
//   init                       VP8LDspInit and WebPInitAlphaProcessing
//   predict MODE QUADS         VP8LPredictors[MODE](&left, &top[1]) for each
//                              (left, top[-1], top[0], top[1]); prints pred=,
//                              and predc= from VP8LPredictorMODE_C for 2..13
//   add TABLE MODE N IN UPPER OUT BUF
//                              VP8LPredictorsAdd[MODE] (TABLE 0) or
//                              VP8LPredictorsAdd_C[MODE] (TABLE 1); prints buf=
//   green TABLE N SRC DST BUF  VP8LAddGreenToBlueAndRed (0) or its _C (1)
//   color TABLE G2R G2B R2B N SRC DST BUF
//                              VP8LTransformColorInverse (0) or its _C (1)
//   inverse TYPE BITS XSIZE YSIZE START END IN OUT DATA BUF
//                              VP8LInverseTransform; prints buf=
//   alpha BITS XSIZE YSIZE START END DATA SRC DSTLEN
//                              VP8LColorIndexInverseTransformAlpha into DSTLEN
//                              bytes filled with 0xaa; prints dst=
//   map32 START END WIDTH SRC DST MAP BUF
//                              VP8LMapColor32b; prints buf=
//   map8 START END WIDTH MAP SRC DSTLEN
//                              VP8LMapColor8b; prints dst=
//   convert MODE N IN OUTLEN   VP8LConvertFromBGRA into OUTLEN bytes filled
//                              with 0xaa; prints out=
//   conv KIND N IN OUTLEN      KIND 0..4: VP8LConvertBGRATo{RGB,RGBA,RGBA4444,
//                              RGB565,BGR}_C, 5..9: the same pointers; prints
//                              out=
//
// tests/wp3_lossless.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/dec/vp8li_dec.h"
#include "src/dsp/dsp.h"
#include "src/dsp/lossless.h"
#include "wp3_io.h"

static const VP8LPredictorFunc kExported[16] = {
  NULL, NULL, VP8LPredictor2_C, VP8LPredictor3_C, VP8LPredictor4_C,
  VP8LPredictor5_C, VP8LPredictor6_C, VP8LPredictor7_C, VP8LPredictor8_C,
  VP8LPredictor9_C, VP8LPredictor10_C, VP8LPredictor11_C, VP8LPredictor12_C,
  VP8LPredictor13_C, NULL, NULL
};

static const VP8LConvertFunc* const kConverters[5] = {
  &VP8LConvertBGRAToRGB, &VP8LConvertBGRAToRGBA, &VP8LConvertBGRAToRGBA4444,
  &VP8LConvertBGRAToRGB565, &VP8LConvertBGRAToBGR
};

static const VP8LConvertFunc kConvertersC[5] = {
  VP8LConvertBGRAToRGB_C, VP8LConvertBGRAToRGBA_C, VP8LConvertBGRAToRGBA4444_C,
  VP8LConvertBGRAToRGB565_C, VP8LConvertBGRAToBGR_C
};

static uint8_t* Filled(size_t size) {
  uint8_t* const buffer = (uint8_t*)malloc(size + 1);
  if (buffer == NULL) exit(2);
  memset(buffer, 0xaa, size + 1);
  return buffer;
}

static void PrintBuffer(const uint32_t* buffer, size_t count) {
  Wp3PrintU32s("buf", buffer, count);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "lossless < commands.txt");
  while ((command = Wp3ReadToken()) != NULL) {
    if (strcmp(command, "init") == 0) {
      VP8LDspInit();
      WebPInitAlphaProcessing();
    } else if (strcmp(command, "predict") == 0) {
      const int mode = (int)Wp3ReadLong();
      size_t count, i;
      uint32_t* quads = Wp3ReadU32s(&count);
      uint32_t* out = (uint32_t*)calloc(count / 4 + 1, sizeof(*out));
      if (out == NULL || count % 4 != 0) return 2;
      for (i = 0; i < count / 4; ++i) {
        out[i] = VP8LPredictors[mode](&quads[4 * i], &quads[4 * i + 2]);
      }
      Wp3PrintU32s("pred", out, count / 4);
      if (kExported[mode] != NULL) {
        for (i = 0; i < count / 4; ++i) {
          out[i] = kExported[mode](&quads[4 * i], &quads[4 * i + 2]);
        }
        Wp3PrintU32s("predc", out, count / 4);
      }
      free(quads);
      free(out);
    } else if (strcmp(command, "add") == 0) {
      const int table = (int)Wp3ReadLong();
      const int mode = (int)Wp3ReadLong();
      const int n = (int)Wp3ReadLong();
      const long in = Wp3ReadLong(), upper = Wp3ReadLong(), out = Wp3ReadLong();
      size_t count;
      uint32_t* buf = Wp3ReadU32s(&count);
      (table ? VP8LPredictorsAdd_C : VP8LPredictorsAdd)[mode](
          buf + in, buf + upper, n, buf + out);
      PrintBuffer(buf, count);
      free(buf);
    } else if (strcmp(command, "green") == 0) {
      const int table = (int)Wp3ReadLong();
      const int n = (int)Wp3ReadLong();
      const long src = Wp3ReadLong(), dst = Wp3ReadLong();
      size_t count;
      uint32_t* buf = Wp3ReadU32s(&count);
      (table ? VP8LAddGreenToBlueAndRed_C : VP8LAddGreenToBlueAndRed)(
          buf + src, n, buf + dst);
      PrintBuffer(buf, count);
      free(buf);
    } else if (strcmp(command, "color") == 0) {
      const int table = (int)Wp3ReadLong();
      VP8LMultipliers m;
      int n;
      long src, dst;
      size_t count;
      uint32_t* buf;
      m.green_to_red = (uint8_t)Wp3ReadLong();
      m.green_to_blue = (uint8_t)Wp3ReadLong();
      m.red_to_blue = (uint8_t)Wp3ReadLong();
      n = (int)Wp3ReadLong();
      src = Wp3ReadLong();
      dst = Wp3ReadLong();
      buf = Wp3ReadU32s(&count);
      (table ? VP8LTransformColorInverse_C : VP8LTransformColorInverse)(
          &m, buf + src, n, buf + dst);
      PrintBuffer(buf, count);
      free(buf);
    } else if (strcmp(command, "inverse") == 0) {
      VP8LTransform transform;
      int start, end;
      long in, out;
      size_t data_count, count;
      uint32_t* buf;
      transform.type = (VP8LImageTransformType)Wp3ReadLong();
      transform.bits = (int)Wp3ReadLong();
      transform.xsize = (int)Wp3ReadLong();
      transform.ysize = (int)Wp3ReadLong();
      start = (int)Wp3ReadLong();
      end = (int)Wp3ReadLong();
      in = Wp3ReadLong();
      out = Wp3ReadLong();
      transform.data = Wp3ReadU32s(&data_count);
      buf = Wp3ReadU32s(&count);
      VP8LInverseTransform(&transform, start, end, buf + in, buf + out);
      PrintBuffer(buf, count);
      free(transform.data);
      free(buf);
    } else if (strcmp(command, "alpha") == 0) {
      VP8LTransform transform;
      int start, end;
      size_t data_count, src_count, dst_count;
      uint8_t *src, *dst;
      transform.type = COLOR_INDEXING_TRANSFORM;
      transform.bits = (int)Wp3ReadLong();
      transform.xsize = (int)Wp3ReadLong();
      transform.ysize = (int)Wp3ReadLong();
      start = (int)Wp3ReadLong();
      end = (int)Wp3ReadLong();
      transform.data = Wp3ReadU32s(&data_count);
      src = Wp3ReadBytes(&src_count);
      dst_count = (size_t)Wp3ReadLong();
      dst = Filled(dst_count);
      VP8LColorIndexInverseTransformAlpha(&transform, start, end, src, dst);
      ProbePrintHex("dst", dst, dst_count);
      free(transform.data);
      free(src);
      free(dst);
    } else if (strcmp(command, "map32") == 0) {
      const int start = (int)Wp3ReadLong();
      const int end = (int)Wp3ReadLong();
      const int width = (int)Wp3ReadLong();
      const long src = Wp3ReadLong(), dst = Wp3ReadLong();
      size_t map_count, count;
      uint32_t* map = Wp3ReadU32s(&map_count);
      uint32_t* buf = Wp3ReadU32s(&count);
      VP8LMapColor32b(buf + src, map, buf + dst, start, end, width);
      PrintBuffer(buf, count);
      free(map);
      free(buf);
    } else if (strcmp(command, "map8") == 0) {
      const int start = (int)Wp3ReadLong();
      const int end = (int)Wp3ReadLong();
      const int width = (int)Wp3ReadLong();
      size_t map_count, src_count, dst_count;
      uint32_t* map = Wp3ReadU32s(&map_count);
      uint8_t* src = Wp3ReadBytes(&src_count);
      uint8_t* dst;
      dst_count = (size_t)Wp3ReadLong();
      dst = Filled(dst_count);
      VP8LMapColor8b(src, map, dst, start, end, width);
      ProbePrintHex("dst", dst, dst_count);
      free(map);
      free(src);
      free(dst);
    } else if (strcmp(command, "convert") == 0 || strcmp(command, "conv") == 0) {
      const int is_convert = (strcmp(command, "convert") == 0);
      const int kind = (int)Wp3ReadLong();
      const int n = (int)Wp3ReadLong();
      size_t count, out_count;
      uint32_t* in = Wp3ReadU32s(&count);
      uint8_t* out;
      out_count = (size_t)Wp3ReadLong();
      out = Filled(out_count);
      if (is_convert) {
        VP8LConvertFromBGRA(in, n, (WEBP_CSP_MODE)kind, out);
      } else if (kind < 5) {
        kConvertersC[kind](in, n, out);
      } else {
        (*kConverters[kind - 5])(in, n, out);
      }
      ProbePrintHex("out", out, out_count);
      free(in);
      free(out);
    } else {
      fprintf(stderr, "lossless: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
