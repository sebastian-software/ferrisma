// WP6 probe: the decoder DSP of src/dsp/dec.c and the clip tables of
// src/dsp/dec_clip_tables.c (ADR 0002). Commands on standard input:
//
//   tables                       prints the four clip tables over their ranges
//   transform KIND COEFFS DST    KIND is two, one, ac3, uv, dc or dcuv; COEFFS
//                                are little-endian int16 values, DST the bytes
//                                of the destination (rows BPS bytes apart),
//                                which the transform updates at offset 0
//   wht COEFFS OUT               VP8TransformWHT from 16 int16 into 256 int16
//   pred FAMILY MODE POS BUF     FAMILY is luma4, luma16 or chroma8; the
//                                predictor writes the block at POS of BUF
//   simple NAME POS STRIDE THRESH BUF
//                                NAME is v16, h16, v16i or h16i
//   luma NAME POS STRIDE THRESH ITHRESH HEV BUF
//                                NAME is v16, h16, v16i or h16i
//   chroma NAME UPOS VPOS STRIDE THRESH ITHRESH HEV U V
//                                NAME is v8, h8, v8i or h8i
//   dither DITHER STRIDE DST     VP8DitherCombine8x8 of 64 values into DST
//
// After each command the probe prints the updated buffers in hex.
// tests/wp6_dsp.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dsp/dsp.h"
#include "probe_support.h"
#include "wp6_io.h"

static void ReadInt16(int16_t* const out, size_t count) {
  size_t size, i;
  uint8_t* const bytes = Wp6ReadBytes(&size);
  if (size != 2 * count) {
    fprintf(stderr, "wp6: expected %zu int16 values\n", count);
    exit(2);
  }
  for (i = 0; i < count; ++i) {
    out[i] = (int16_t)(uint16_t)(bytes[2 * i] | (bytes[2 * i + 1] << 8));
  }
  free(bytes);
}

static void PrintInt16(const char* key, const int16_t* values, size_t count) {
  size_t i;
  uint8_t* const bytes = (uint8_t*)malloc(2 * count + 1);
  if (bytes == NULL) exit(2);
  for (i = 0; i < count; ++i) {
    const uint16_t v = (uint16_t)values[i];
    bytes[2 * i] = (uint8_t)(v & 0xff);
    bytes[2 * i + 1] = (uint8_t)(v >> 8);
  }
  ProbePrintHex(key, bytes, 2 * count);
  free(bytes);
}

static void Tables(void) {
  uint8_t buffer[2041];
  int i;
  for (i = -1020; i <= 1020; ++i) buffer[i + 1020] = (uint8_t)VP8ksclip1[i];
  ProbePrintHex("sclip1", buffer, 2041);
  for (i = -112; i <= 112; ++i) buffer[i + 112] = (uint8_t)VP8ksclip2[i];
  ProbePrintHex("sclip2", buffer, 225);
  for (i = -255; i <= 511; ++i) buffer[i + 255] = VP8kclip1[i];
  ProbePrintHex("clip1", buffer, 767);
  for (i = -255; i <= 255; ++i) buffer[i + 255] = VP8kabs0[i];
  ProbePrintHex("abs0", buffer, 511);
}

static void Transform(void) {
  char* const kind = Wp6ReadToken();
  int16_t coeffs[384];
  size_t size;
  uint8_t* dst;
  if (kind == NULL) exit(2);
  ReadInt16(coeffs, 64);
  dst = Wp6ReadBytes(&size);
  if (size < 8 * BPS) exit(2);
  if (!strcmp(kind, "two")) {
    VP8Transform(coeffs, dst, 1);
  } else if (!strcmp(kind, "one")) {
    VP8Transform(coeffs, dst, 0);
  } else if (!strcmp(kind, "ac3")) {
    VP8TransformAC3(coeffs, dst);
  } else if (!strcmp(kind, "uv")) {
    VP8TransformUV(coeffs, dst);
  } else if (!strcmp(kind, "dc")) {
    VP8TransformDC(coeffs, dst);
  } else if (!strcmp(kind, "dcuv")) {
    VP8TransformDCUV(coeffs, dst);
  } else {
    exit(2);
  }
  ProbePrintHex("dst", dst, size);
  free(dst);
  free(kind);
}

static void Wht(void) {
  int16_t in[16];
  int16_t out[256];
  ReadInt16(in, 16);
  ReadInt16(out, 256);
  VP8TransformWHT(in, out);
  PrintInt16("out", out, 256);
}

static void Pred(void) {
  char* const family = Wp6ReadToken();
  const long mode = Wp6ReadLong();
  const long pos = Wp6ReadLong();
  size_t size;
  uint8_t* const buf = Wp6ReadBytes(&size);
  if (family == NULL || pos < 0 || (size_t)pos > size) exit(2);
  if (!strcmp(family, "luma4")) {
    VP8PredLuma4[mode](buf + pos);
  } else if (!strcmp(family, "luma16")) {
    VP8PredLuma16[mode](buf + pos);
  } else if (!strcmp(family, "chroma8")) {
    VP8PredChroma8[mode](buf + pos);
  } else {
    exit(2);
  }
  ProbePrintHex("buf", buf, size);
  free(buf);
  free(family);
}

static void Simple(void) {
  char* const name = Wp6ReadToken();
  const long pos = Wp6ReadLong();
  const int stride = (int)Wp6ReadLong();
  const int thresh = (int)Wp6ReadLong();
  size_t size;
  uint8_t* const buf = Wp6ReadBytes(&size);
  if (name == NULL || pos < 0 || (size_t)pos > size) exit(2);
  if (!strcmp(name, "v16")) {
    VP8SimpleVFilter16(buf + pos, stride, thresh);
  } else if (!strcmp(name, "h16")) {
    VP8SimpleHFilter16(buf + pos, stride, thresh);
  } else if (!strcmp(name, "v16i")) {
    VP8SimpleVFilter16i(buf + pos, stride, thresh);
  } else if (!strcmp(name, "h16i")) {
    VP8SimpleHFilter16i(buf + pos, stride, thresh);
  } else {
    exit(2);
  }
  ProbePrintHex("buf", buf, size);
  free(buf);
  free(name);
}

static void Luma(void) {
  char* const name = Wp6ReadToken();
  const long pos = Wp6ReadLong();
  const int stride = (int)Wp6ReadLong();
  const int thresh = (int)Wp6ReadLong();
  const int ithresh = (int)Wp6ReadLong();
  const int hev = (int)Wp6ReadLong();
  size_t size;
  uint8_t* const buf = Wp6ReadBytes(&size);
  if (name == NULL || pos < 0 || (size_t)pos > size) exit(2);
  if (!strcmp(name, "v16")) {
    VP8VFilter16(buf + pos, stride, thresh, ithresh, hev);
  } else if (!strcmp(name, "h16")) {
    VP8HFilter16(buf + pos, stride, thresh, ithresh, hev);
  } else if (!strcmp(name, "v16i")) {
    VP8VFilter16i(buf + pos, stride, thresh, ithresh, hev);
  } else if (!strcmp(name, "h16i")) {
    VP8HFilter16i(buf + pos, stride, thresh, ithresh, hev);
  } else {
    exit(2);
  }
  ProbePrintHex("buf", buf, size);
  free(buf);
  free(name);
}

static void Chroma(void) {
  char* const name = Wp6ReadToken();
  const long upos = Wp6ReadLong();
  const long vpos = Wp6ReadLong();
  const int stride = (int)Wp6ReadLong();
  const int thresh = (int)Wp6ReadLong();
  const int ithresh = (int)Wp6ReadLong();
  const int hev = (int)Wp6ReadLong();
  size_t usize, vsize;
  uint8_t* const u = Wp6ReadBytes(&usize);
  uint8_t* const v = Wp6ReadBytes(&vsize);
  if (name == NULL || upos < 0 || vpos < 0) exit(2);
  if (!strcmp(name, "v8")) {
    VP8VFilter8(u + upos, v + vpos, stride, thresh, ithresh, hev);
  } else if (!strcmp(name, "h8")) {
    VP8HFilter8(u + upos, v + vpos, stride, thresh, ithresh, hev);
  } else if (!strcmp(name, "v8i")) {
    VP8VFilter8i(u + upos, v + vpos, stride, thresh, ithresh, hev);
  } else if (!strcmp(name, "h8i")) {
    VP8HFilter8i(u + upos, v + vpos, stride, thresh, ithresh, hev);
  } else {
    exit(2);
  }
  ProbePrintHex("u", u, usize);
  ProbePrintHex("v", v, vsize);
  free(u);
  free(v);
  free(name);
}

static void Dither(void) {
  size_t dither_size, size;
  uint8_t* const dither = Wp6ReadBytes(&dither_size);
  const int stride = (int)Wp6ReadLong();
  uint8_t* const dst = Wp6ReadBytes(&size);
  if (dither_size != 64) exit(2);
  VP8DitherCombine8x8(dither, dst, stride);
  ProbePrintHex("dst", dst, size);
  free(dither);
  free(dst);
}

int main(int argc, char* argv[]) {
  char* command;
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "dsp < commands");
  VP8DspInit();
  while ((command = Wp6ReadToken()) != NULL) {
    if (!strcmp(command, "tables")) {
      Tables();
    } else if (!strcmp(command, "transform")) {
      Transform();
    } else if (!strcmp(command, "wht")) {
      Wht();
    } else if (!strcmp(command, "pred")) {
      Pred();
    } else if (!strcmp(command, "simple")) {
      Simple();
    } else if (!strcmp(command, "luma")) {
      Luma();
    } else if (!strcmp(command, "chroma")) {
      Chroma();
    } else if (!strcmp(command, "dither")) {
      Dither();
    } else {
      fprintf(stderr, "wp6: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
