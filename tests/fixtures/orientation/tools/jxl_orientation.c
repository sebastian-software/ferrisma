// Write a lossless 24x16 RGB8 JPEG XL file with a given EXIF orientation.
// Usage: orient <pattern.ppm> <orientation 1-8> <out.jxl>
#include <jxl/encode.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

int main(int argc, char **argv) {
  if (argc != 4) return 2;
  FILE *in = fopen(argv[1], "rb");
  char magic[3];
  int w, h, max;
  if (!in || fscanf(in, "%2s %d %d %d", magic, &w, &h, &max) != 4) return 3;
  fgetc(in);
  size_t size = (size_t)w * h * 3;
  unsigned char *pixels = malloc(size);
  if (fread(pixels, 1, size, in) != size) return 4;
  fclose(in);

  JxlEncoder *enc = JxlEncoderCreate(NULL);
  JxlBasicInfo info;
  JxlEncoderInitBasicInfo(&info);
  info.xsize = w;
  info.ysize = h;
  info.bits_per_sample = 8;
  info.num_color_channels = 3;
  info.uses_original_profile = JXL_TRUE;
  info.orientation = (JxlOrientation)atoi(argv[2]);
  if (JxlEncoderSetBasicInfo(enc, &info) != JXL_ENC_SUCCESS) return 5;
  JxlColorEncoding color;
  JxlColorEncodingSetToSRGB(&color, JXL_FALSE);
  if (JxlEncoderSetColorEncoding(enc, &color) != JXL_ENC_SUCCESS) return 6;
  JxlEncoderFrameSettings *settings = JxlEncoderFrameSettingsCreate(enc, NULL);
  JxlEncoderSetFrameLossless(settings, JXL_TRUE);
  JxlPixelFormat format = {3, JXL_TYPE_UINT8, JXL_NATIVE_ENDIAN, 0};
  if (JxlEncoderAddImageFrame(settings, &format, pixels, size) != JXL_ENC_SUCCESS) return 7;
  JxlEncoderCloseInput(enc);

  size_t capacity = 1 << 16, used = 0;
  unsigned char *out = malloc(capacity);
  unsigned char *next = out;
  size_t avail = capacity;
  JxlEncoderStatus status;
  while ((status = JxlEncoderProcessOutput(enc, &next, &avail)) == JXL_ENC_NEED_MORE_OUTPUT) {
    used = next - out;
    capacity *= 2;
    out = realloc(out, capacity);
    next = out + used;
    avail = capacity - used;
  }
  if (status != JXL_ENC_SUCCESS) return 8;
  used = next - out;
  FILE *f = fopen(argv[3], "wb");
  fwrite(out, 1, used, f);
  fclose(f);
  JxlEncoderDestroy(enc);
  return 0;
}
