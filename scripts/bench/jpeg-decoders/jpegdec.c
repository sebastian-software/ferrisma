/*
 * Times one libjpeg-API decoder (libjpeg-turbo, or jpegli with -DJPEGLI) on
 * one JPEG held in memory, and optionally writes the last decode as a
 * quality-match `.ref` file (interleaved RGB f32, sRGB-encoded, [0, 1]).
 *
 * usage: jpegdec FILE DENOM REPS [OUT.ref [u8|u16|f32]]
 *
 * DENOM is the libjpeg scale denominator (1, 2, 4 or 8; scale_num = 1).
 * Prints: decoder, file, denom, output size, output type, best and median
 * wall time in milliseconds. Decoding is single-threaded in both libraries.
 * jpegli's headers are C++, so build.sh compiles this file as C++ for it.
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

#ifdef JPEGLI
#include "lib/jpegli/decode.h"
#define DEC(name) jpegli_##name
#define DECODER "jpegli"
#else
#include <jpeglib.h>
#define DEC(name) jpeg_##name
#define DECODER "libjpeg-turbo"
#endif

static double now_ms(void) {
  struct timespec t;
  clock_gettime(CLOCK_MONOTONIC, &t);
  return t.tv_sec * 1e3 + t.tv_nsec / 1e6;
}

static int cmp(const void *a, const void *b) {
  double x = *(const double *)a, y = *(const double *)b;
  return (x > y) - (x < y);
}

/* Decodes once; returns the output buffer (caller frees) and its geometry. */
static void *decode(const unsigned char *data, size_t size, int denom,
                    int type, unsigned *w, unsigned *h) {
  struct jpeg_decompress_struct cinfo;
  struct jpeg_error_mgr err;
  cinfo.err = DEC(std_error)(&err);
  DEC(create_decompress)(&cinfo);
  DEC(mem_src)(&cinfo, data, (unsigned long)size);
  DEC(read_header)(&cinfo, TRUE);
  cinfo.out_color_space = JCS_RGB;
  cinfo.scale_num = 1;
  cinfo.scale_denom = (unsigned)denom;
  size_t sample = 1;
#ifdef JPEGLI
  if (type == 1) {
    jpegli_set_output_format(&cinfo, JPEGLI_TYPE_UINT16, JPEGLI_NATIVE_ENDIAN);
    sample = 2;
  } else if (type == 2) {
    jpegli_set_output_format(&cinfo, JPEGLI_TYPE_FLOAT, JPEGLI_NATIVE_ENDIAN);
    sample = 4;
  }
#else
  if (type != 0) {
    fprintf(stderr, "libjpeg-turbo: only u8 output\n");
    exit(2);
  }
#endif
  DEC(start_decompress)(&cinfo);
  *w = cinfo.output_width;
  *h = cinfo.output_height;
  size_t stride = (size_t)*w * 3 * sample;
  unsigned char *out = (unsigned char *)malloc(stride * *h);
  if (!out) exit(3);
  while (cinfo.output_scanline < cinfo.output_height) {
    JSAMPROW row = out + stride * cinfo.output_scanline;
    DEC(read_scanlines)(&cinfo, &row, 1);
  }
  DEC(finish_decompress)(&cinfo);
  DEC(destroy_decompress)(&cinfo);
  return out;
}

static void write_ref(const char *path, const void *buf, unsigned w,
                      unsigned h, int type) {
  FILE *f = fopen(path, "wb");
  if (!f) exit(4);
  fwrite("QMREF1\0\0", 1, 8, f);
  uint32_t dims[2] = {w, h}; /* little-endian hosts only */
  fwrite(dims, 4, 2, f);
  size_t n = (size_t)w * h * 3;
  for (size_t i = 0; i < n; i++) {
    float v;
    if (type == 0) v = ((const uint8_t *)buf)[i] / 255.0f;
    else if (type == 1) v = ((const uint16_t *)buf)[i] / 65535.0f;
    else v = ((const float *)buf)[i];
    /* Display-referred scoring: clamp jpegli's unclamped float output. */
    v = v < 0.0f ? 0.0f : v > 1.0f ? 1.0f : v;
    fwrite(&v, 4, 1, f);
  }
  fclose(f);
}

int main(int argc, char **argv) {
  if (argc < 4) {
    fprintf(stderr, "usage: jpegdec FILE DENOM REPS [OUT.ref [u8|u16|f32]]\n");
    return 2;
  }
  int denom = atoi(argv[2]), reps = atoi(argv[3]), type = 0;
  if (argc > 5) type = !strcmp(argv[5], "u16") ? 1 : !strcmp(argv[5], "f32") ? 2 : 0;
  FILE *f = fopen(argv[1], "rb");
  if (!f) return 4;
  fseek(f, 0, SEEK_END);
  size_t size = (size_t)ftell(f);
  fseek(f, 0, SEEK_SET);
  unsigned char *data = (unsigned char *)malloc(size);
  if (fread(data, 1, size, f) != size) return 4;
  fclose(f);

  double *times = (double *)malloc(sizeof(double) * (size_t)reps);
  unsigned w = 0, h = 0;
  void *out = NULL;
  for (int i = 0; i < reps; i++) {
    free(out);
    double t0 = now_ms();
    out = decode(data, size, denom, type, &w, &h);
    times[i] = now_ms() - t0;
  }
  qsort(times, (size_t)reps, sizeof(double), cmp);
  const char *types[] = {"u8", "u16", "f32"};
  printf("%s\t%s\t1/%d\t%ux%u\t%s\tbest=%.2f\tmedian=%.2f\n", DECODER, argv[1],
         denom, w, h, types[type], times[0], times[reps / 2]);
  if (argc > 4) write_ref(argv[4], out, w, h, type);
  free(out);
  free(times);
  free(data);
  return 0;
}
