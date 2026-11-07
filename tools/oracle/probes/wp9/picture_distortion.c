// WP9 probe: WebPPictureDistortion (src/enc/picture_psnr_enc.c). Each command is
//
//   distortion KIND A B
//
// with KIND 0 (PSNR), 1 (SSIM) or 2 (LSIM), and each picture A and B in the form
//
//   USE_ARGB COLORSPACE W H Y_STRIDE UV_STRIDE A_STRIDE ARGB_STRIDE HEX_ARGB HEX_Y HEX_U HEX_V HEX_A
//
// A HEX argument is "-" for a plane the picture does not use. The results start at -1
// and the probe prints ret and the five result floats as bits, as they stand after the
// call.

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

typedef struct {
  WebPPicture pic;
  uint8_t* argb_bytes;
  size_t argb_size;
  uint8_t* y;
  size_t y_size;
  uint8_t* u;
  size_t u_size;
  uint8_t* v;
  size_t v_size;
  uint8_t* a;
  size_t a_size;
} Picture;

static void ReadPicture(Picture* p) {
  char f[8][32];
  char hex_argb[1 << 16], hex_y[1 << 16], hex_u[1 << 16], hex_v[1 << 16],
      hex_a[1 << 16];
  int i;
  memset(p, 0, sizeof(*p));
  for (i = 0; i < 8; ++i) {
    if (scanf("%31s", f[i]) != 1) exit(2);
  }
  if (scanf("%65535s %65535s %65535s %65535s %65535s", hex_argb, hex_y, hex_u, hex_v,
            hex_a) != 5) {
    exit(2);
  }
  WebPPictureInit(&p->pic);
  p->pic.use_argb = (int)WP9ParseInt(f[0]);
  p->pic.colorspace = (WebPEncCSP)WP9ParseInt(f[1]);
  p->pic.width = (int)WP9ParseInt(f[2]);
  p->pic.height = (int)WP9ParseInt(f[3]);
  if (p->pic.use_argb) {
    p->argb_bytes = WP9ParseHex(hex_argb, &p->argb_size);
    p->pic.argb = (uint32_t*)p->argb_bytes;
    p->pic.argb_stride = (int)WP9ParseInt(f[7]);
  } else {
    p->y = WP9ParseHex(hex_y, &p->y_size);
    p->u = WP9ParseHex(hex_u, &p->u_size);
    p->v = WP9ParseHex(hex_v, &p->v_size);
    p->pic.y = p->y;
    p->pic.u = p->u;
    p->pic.v = p->v;
    p->pic.y_stride = (int)WP9ParseInt(f[4]);
    p->pic.uv_stride = (int)WP9ParseInt(f[5]);
    if (p->pic.colorspace & WEBP_CSP_ALPHA_BIT) {
      p->a = WP9ParseHex(hex_a, &p->a_size);
      p->pic.a = p->a;
      p->pic.a_stride = (int)WP9ParseInt(f[6]);
    }
  }
}

static void ReleasePicture(Picture* p) {
  WebPPictureFree(&p->pic);
  free(p->argb_bytes);
  free(p->y);
  free(p->u);
  free(p->v);
  free(p->a);
}

int main(int argc, char* argv[]) {
  char command[32];
  (void)argc;
  (void)argv;
  while (scanf("%31s", command) == 1) {
    char kind[16];
    Picture a, b;
    float results[5] = {-1.0f, -1.0f, -1.0f, -1.0f, -1.0f};
    int ret, i;
    if (strcmp(command, "distortion") != 0) {
      fprintf(stderr, "picture_distortion: unknown command %s\n", command);
      return 2;
    }
    if (scanf("%15s", kind) != 1) return 2;
    ReadPicture(&a);
    ReadPicture(&b);
    ret = WebPPictureDistortion(&a.pic, &b.pic, (int)WP9ParseInt(kind), results);
    printf("ret=%d\n", ret);
    for (i = 0; i < 5; ++i) {
      printf("result%d=%08x\n", i, FloatBits(results[i]));
    }
    ReleasePicture(&a);
    ReleasePicture(&b);
  }
  return 0;
}
