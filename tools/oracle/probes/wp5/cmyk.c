/* WP5 test-input generator: encodes a PPM image as a four-component JPEG,
 * for the CMYK and YCCK conversions of jdcolor.c, which no upstream test
 * image exercises.
 *
 * Usage: cmyk cmyk|ycck [HxV,HxV,HxV,HxV] < image.ppm > image.jpg
 * Each pixel (r, g, b) becomes C = 255 - r, M = 255 - g, Y = 255 - b and
 * K = (r + g) / 2; the oracle's compressor stores it as CMYK or YCCK (with
 * an Adobe marker) with the given sampling factors.
 */

#include "probe_support.h"

#include <stdlib.h>
#include <string.h>

int main(int argc, char *argv[])
{
  struct jpeg_compress_struct cinfo;
  struct jpeg_error_mgr jerr;
  unsigned char *data, *pixels, *out = NULL;
  unsigned long out_size = 0;
  size_t size;
  int width, height, maxval, offset = 0, ci;
  JSAMPROW row;
  JDIMENSION y, x;

  if (argc < 2) {
    fprintf(stderr, "usage: cmyk cmyk|ycck [HxV,...]\n");
    return 2;
  }
  data = ProbeReadInput(NULL, &size);
  if (sscanf((char *)data, "P6 %d %d %d%n", &width, &height, &maxval, &offset) != 3 ||
      maxval != 255) {
    fprintf(stderr, "cmyk: not a binary 8-bit PPM\n");
    free(data);
    return 2;
  }
  pixels = data + offset + 1;

  cinfo.err = jpeg_std_error(&jerr);
  jpeg_create_compress(&cinfo);
  jpeg_mem_dest(&cinfo, &out, &out_size);
  cinfo.image_width = (JDIMENSION)width;
  cinfo.image_height = (JDIMENSION)height;
  cinfo.input_components = 4;
  cinfo.in_color_space = JCS_CMYK;
  jpeg_set_defaults(&cinfo);
  jpeg_set_colorspace(&cinfo, strcmp(argv[1], "ycck") ? JCS_CMYK : JCS_YCCK);
  if (argc > 2) {
    const char *spec = argv[2];

    for (ci = 0; ci < 4 && *spec; ci++) {
      int h, v, n;

      if (sscanf(spec, "%dx%d%n", &h, &v, &n) != 2)
        break;
      cinfo.comp_info[ci].h_samp_factor = h;
      cinfo.comp_info[ci].v_samp_factor = v;
      spec += n;
      if (*spec == ',')
        spec++;
    }
  }
  jpeg_start_compress(&cinfo, TRUE);
  row = (JSAMPROW)malloc((size_t)width * 4);
  for (y = 0; y < (JDIMENSION)height; y++) {
    const unsigned char *p = pixels + (size_t)y * (size_t)width * 3;

    for (x = 0; x < (JDIMENSION)width; x++) {
      row[x * 4] = (JSAMPLE)(255 - p[x * 3]);
      row[x * 4 + 1] = (JSAMPLE)(255 - p[x * 3 + 1]);
      row[x * 4 + 2] = (JSAMPLE)(255 - p[x * 3 + 2]);
      row[x * 4 + 3] = (JSAMPLE)((p[x * 3] + p[x * 3 + 1]) / 2);
    }
    jpeg_write_scanlines(&cinfo, &row, 1);
  }
  jpeg_finish_compress(&cinfo);
  fwrite(out, 1, out_size, stdout);
  jpeg_destroy_compress(&cinfo);
  free(row);
  free(out);
  free(data);
  return 0;
}
