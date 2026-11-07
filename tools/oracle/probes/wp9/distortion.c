// WP9 probe: WebPPlaneDistortion (src/enc/picture_psnr_enc.c). Each command is
//
//   plane TYPE W H X_STEP SRC_STRIDE REF_STRIDE HEX_SRC HEX_REF
//
// with TYPE 0 (PSNR), 1 (SSIM) or 2 (LSIM); the buffers hold SRC_STRIDE by H and
// REF_STRIDE by H bytes. The probe prints the result code and the bits of the two
// floats, which start as -1 and are printed unchanged when the call fails.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/webp/encode.h"
#include "wp9_support.h"

static uint32_t FloatBits(float value) {
  uint32_t bits;
  memcpy(&bits, &value, sizeof(bits));
  return bits;
}

int main(int argc, char* argv[]) {
  char command[32];
  (void)argc;
  (void)argv;
  while (scanf("%31s", command) == 1) {
    char type[16], w[16], h[16], step[16], src_stride[16], ref_stride[16];
    char hex_src[1 << 16], hex_ref[1 << 16];
    size_t src_size = 0, ref_size = 0;
    uint8_t* src;
    uint8_t* ref;
    float distortion = -1.0f, result = -1.0f;
    int ret;
    if (strcmp(command, "plane") != 0) {
      fprintf(stderr, "distortion: unknown command %s\n", command);
      return 2;
    }
    if (scanf("%15s %15s %15s %15s %15s %15s %65535s %65535s", type, w, h, step,
              src_stride, ref_stride, hex_src, hex_ref) != 8) {
      return 2;
    }
    src = WP9ParseHex(hex_src, &src_size);
    ref = WP9ParseHex(hex_ref, &ref_size);
    ret = WebPPlaneDistortion(src, (size_t)WP9ParseInt(src_stride), ref,
                              (size_t)WP9ParseInt(ref_stride), (int)WP9ParseInt(w),
                              (int)WP9ParseInt(h), (size_t)WP9ParseInt(step),
                              (int)WP9ParseInt(type), &distortion, &result);
    printf("ret=%d\n", ret);
    printf("distortion=%08x\n", FloatBits(distortion));
    printf("result=%08x\n", FloatBits(result));
    free(src);
    free(ref);
  }
  return 0;
}
