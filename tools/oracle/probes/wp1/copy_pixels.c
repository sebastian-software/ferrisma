// WP1 probe: runs WebPCopyPixels (src/utils/utils.c) on two ARGB pictures that
// the library initializes with WebPPictureInit. Each input line is
//
//   copy SW DW W H SO DO SL DL HEX
//
// with the source and destination strides SW and DW in pixels, the dimensions W
// and H, the first pixel of each picture SO and DO in pixels, the sizes SL and DL
// in pixels, and HEX the source pixels as little-endian bytes, or "-" when SL is 0.
// The destination starts as 0xa5 bytes, and the probe prints it as `dst=` hex.
//
// Usage: copy_pixels < commands.txt

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/utils.h"
#include "src/webp/encode.h"

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "copy_pixels: bad hex digit %c\n", c);
  exit(2);
}

int main(int argc, char* argv[]) {
  char command[16];
  ProbeCheckArgs(argc, 0, 0, "copy_pixels < commands.txt");
  while (scanf("%15s", command) == 1) {
    if (strcmp(command, "copy") == 0) {
      int src_stride, dst_stride, width, height;
      size_t src_offset, dst_offset, src_len, dst_len, i;
      char* hex;
      uint8_t* src_bytes;
      uint32_t *src, *dst;
      WebPPicture src_picture, dst_picture;
      if (scanf("%d %d %d %d %zu %zu %zu %zu", &src_stride, &dst_stride, &width,
                &height, &src_offset, &dst_offset, &src_len, &dst_len) != 8) {
        return 2;
      }
      hex = (char*)malloc(8 * src_len + 2);
      if (hex == NULL) return 2;
      if (scanf("%s", hex) != 1) return 2;
      src_bytes = (uint8_t*)malloc(4 * src_len + 4);
      src = (uint32_t*)malloc(4 * src_len + 4);
      dst = (uint32_t*)malloc(4 * dst_len + 4);
      if (src_bytes == NULL || src == NULL || dst == NULL) return 2;
      for (i = 0; i < 4 * src_len; ++i) {
        src_bytes[i] = (uint8_t)(HexValue(hex[2 * i]) * 16 + HexValue(hex[2 * i + 1]));
      }
      memcpy(src, src_bytes, 4 * src_len);
      memset(dst, 0xa5, 4 * dst_len);

      (void)WebPPictureInit(&src_picture);
      (void)WebPPictureInit(&dst_picture);
      src_picture.use_argb = 1;
      src_picture.width = width;
      src_picture.height = height;
      src_picture.argb_stride = src_stride;
      src_picture.argb = src + src_offset;
      dst_picture.use_argb = 1;
      dst_picture.width = width;
      dst_picture.height = height;
      dst_picture.argb_stride = dst_stride;
      dst_picture.argb = dst + dst_offset;
      WebPCopyPixels(&src_picture, &dst_picture);

      ProbePrintHex("dst", (const uint8_t*)dst, 4 * dst_len);
      free(hex);
      free(src_bytes);
      free(src);
      free(dst);
    } else {
      fprintf(stderr, "copy_pixels: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
