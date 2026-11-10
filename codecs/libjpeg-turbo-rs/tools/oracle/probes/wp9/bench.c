/* WP9 probe: the oracle's decode speed, for tools/bench.py (ADR 0004).
 *
 * Usage: bench [OPTION]... FILE
 * Reads FILE into memory and decodes it REPEATS times through the libjpeg
 * API as an application does: jpeg_create_decompress, jpeg_mem_src,
 * jpeg_read_header, the options, jpeg_start_decompress, jpeg_read_scanlines
 * into one buffer for the whole image, jpeg_finish_decompress and
 * jpeg_destroy_decompress. examples/bench.rs decodes the same way through
 * the port's safe API. Options:
 *
 *   -repeats N   decodes to time (default 10), after one untimed decode
 *   -out CS      out_color_space = CS (a J_COLOR_SPACE number)
 *   -scale N D   scale_num = N, scale_denom = D
 *   -dct M       dct_method = M (islow, ifast, float)
 *   -nofancy     do_fancy_upsampling = FALSE
 *
 * Output: `dims=<width>,<height>,<components>`, `decodes=<REPEATS>`,
 * `nanoseconds=<total of the timed decodes>` and `checksum=<FNV-1a of the
 * last decode's output>`.
 */

#include "probe_support.h"

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <time.h>

struct options {
  int repeats;
  int out_color_space;
  unsigned int scale_num, scale_denom;
  J_DCT_METHOD dct_method;
  boolean fancy;
};

static uint64_t now_ns(void)
{
  struct timespec ts;

  clock_gettime(CLOCK_MONOTONIC, &ts);
  return (uint64_t)ts.tv_sec * 1000000000u + (uint64_t)ts.tv_nsec;
}

/* Decodes data once into *buffer (grown as needed) and returns its size. */
static size_t decode(const unsigned char *data, size_t size,
                     const struct options *opt, unsigned char **buffer,
                     size_t *capacity, JDIMENSION *width, JDIMENSION *height,
                     int *components)
{
  struct jpeg_decompress_struct cinfo;
  struct jpeg_error_mgr jerr;
  JSAMPROW *rows;
  size_t row_bytes, total;
  JDIMENSION y;

  cinfo.err = jpeg_std_error(&jerr);
  jpeg_create_decompress(&cinfo);
  jpeg_mem_src(&cinfo, data, (unsigned long)size);
  jpeg_read_header(&cinfo, TRUE);
  if (opt->out_color_space >= 0)
    cinfo.out_color_space = (J_COLOR_SPACE)opt->out_color_space;
  cinfo.scale_num = opt->scale_num;
  cinfo.scale_denom = opt->scale_denom;
  cinfo.dct_method = opt->dct_method;
  cinfo.do_fancy_upsampling = opt->fancy;
  jpeg_start_decompress(&cinfo);

  row_bytes = (size_t)cinfo.output_width * cinfo.output_components;
  total = row_bytes * cinfo.output_height;
  if (total > *capacity) {
    free(*buffer);
    *buffer = (unsigned char *)malloc(total);
    if (!*buffer) {
      fprintf(stderr, "out of memory\n");
      exit(1);
    }
    *capacity = total;
  }
  rows = (JSAMPROW *)malloc(sizeof(JSAMPROW) * cinfo.output_height);
  if (!rows) {
    fprintf(stderr, "out of memory\n");
    exit(1);
  }
  for (y = 0; y < cinfo.output_height; y++)
    rows[y] = *buffer + row_bytes * y;
  while (cinfo.output_scanline < cinfo.output_height)
    jpeg_read_scanlines(&cinfo, rows + cinfo.output_scanline,
                        cinfo.output_height - cinfo.output_scanline);
  free(rows);

  *width = cinfo.output_width;
  *height = cinfo.output_height;
  *components = cinfo.output_components;
  jpeg_finish_decompress(&cinfo);
  jpeg_destroy_decompress(&cinfo);
  return total;
}

int main(int argc, char **argv)
{
  struct options opt = { 10, -1, 1, 1, JDCT_ISLOW, TRUE };
  const char *path = NULL;
  unsigned char *data, *buffer = NULL;
  size_t size, capacity = 0, total = 0, i;
  JDIMENSION width = 0, height = 0;
  int components = 0, argi, r;
  uint64_t start, elapsed, hash = 1469598103934665603ULL;

  for (argi = 1; argi < argc; argi++) {
    if (!strcmp(argv[argi], "-repeats") && argi + 1 < argc)
      opt.repeats = atoi(argv[++argi]);
    else if (!strcmp(argv[argi], "-out") && argi + 1 < argc)
      opt.out_color_space = atoi(argv[++argi]);
    else if (!strcmp(argv[argi], "-scale") && argi + 2 < argc) {
      opt.scale_num = (unsigned int)atoi(argv[++argi]);
      opt.scale_denom = (unsigned int)atoi(argv[++argi]);
    } else if (!strcmp(argv[argi], "-dct") && argi + 1 < argc) {
      argi++;
      if (!strcmp(argv[argi], "ifast"))
        opt.dct_method = JDCT_IFAST;
      else if (!strcmp(argv[argi], "float"))
        opt.dct_method = JDCT_FLOAT;
      else
        opt.dct_method = JDCT_ISLOW;
    } else if (!strcmp(argv[argi], "-nofancy"))
      opt.fancy = FALSE;
    else if (argv[argi][0] != '-' && !path)
      path = argv[argi];
    else {
      fprintf(stderr, "bench: unknown option %s\n", argv[argi]);
      return 2;
    }
  }
  if (!path || opt.repeats < 1) {
    fprintf(stderr, "usage: bench [-repeats N] [-out CS] [-scale N D] "
                    "[-dct M] [-nofancy] FILE\n");
    return 2;
  }
  data = ProbeReadInput(path, &size);

  decode(data, size, &opt, &buffer, &capacity, &width, &height, &components);
  start = now_ns();
  for (r = 0; r < opt.repeats; r++)
    total = decode(data, size, &opt, &buffer, &capacity, &width, &height,
                   &components);
  elapsed = now_ns() - start;

  for (i = 0; i < total; i++) {
    hash ^= buffer[i];
    hash *= 1099511628211ULL;
  }
  printf("dims=%u,%u,%d\n", width, height, components);
  printf("decodes=%d\n", opt.repeats);
  printf("nanoseconds=%llu\n", (unsigned long long)elapsed);
  printf("checksum=%016llx\n", (unsigned long long)hash);
  free(buffer);
  free(data);
  return 0;
}
