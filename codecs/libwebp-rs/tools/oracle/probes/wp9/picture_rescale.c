// WP9 probe: WebPPictureRescale and WebPPictureCrop (src/enc/picture_rescale_enc.c).
// The input planes are the caller's buffers; the result replaces the picture with
// library-owned planes, so the probe prints the visible samples only. Commands, one
// per line on standard input:
//
//   rescale USE_ARGB COLORSPACE W H Y_STRIDE UV_STRIDE A_STRIDE ARGB_STRIDE NEW_W NEW_H HEX_ARGB HEX_Y HEX_U HEX_V HEX_A
//   crop    USE_ARGB COLORSPACE W H Y_STRIDE UV_STRIDE A_STRIDE ARGB_STRIDE LEFT TOP CW CH HEX_ARGB HEX_Y HEX_U HEX_V HEX_A
//
// A HEX argument is "-" for a plane that the picture does not use.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/enc/vp8i_enc.h"
#include "src/webp/encode.h"
#include "wp9_support.h"

static void PrintVisible(const WebPPicture* pic) {
  int w = pic->width, h = pic->height;
  int uv_w = (w + 1) / 2, uv_h = (h + 1) / 2;
  printf("width=%d\n", pic->width);
  printf("height=%d\n", pic->height);
  printf("colorspace=%u\n", (unsigned)pic->colorspace);
  printf("use_argb=%d\n", pic->use_argb);
  printf("error=%d\n", (int)pic->error_code);
  if (pic->use_argb) {
    WP9PrintArgb("argb", pic->argb, w, h, pic->argb_stride);
    return;
  }
  WP9PrintPlane("y", pic->y, w, h, pic->y_stride);
  WP9PrintPlane("u", pic->u, uv_w, uv_h, pic->uv_stride);
  WP9PrintPlane("v", pic->v, uv_w, uv_h, pic->uv_stride);
  if (pic->a != NULL) WP9PrintPlane("a", pic->a, w, h, pic->a_stride);
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
} Setup;

static void Load(Setup* s, int use_argb, int colorspace, int w, int h, long y_stride,
                 long uv_stride, long a_stride, long argb_stride, const char* hex_argb,
                 const char* hex_y, const char* hex_u, const char* hex_v,
                 const char* hex_a) {
  memset(s, 0, sizeof(*s));
  WebPPictureInit(&s->pic);
  s->pic.use_argb = use_argb;
  s->pic.colorspace = (WebPEncCSP)colorspace;
  s->pic.width = w;
  s->pic.height = h;
  if (use_argb) {
    s->argb_bytes = WP9ParseHex(hex_argb, &s->argb_size);
    s->pic.argb = (uint32_t*)s->argb_bytes;
    s->pic.argb_stride = (int)argb_stride;
  } else {
    s->y = WP9ParseHex(hex_y, &s->y_size);
    s->u = WP9ParseHex(hex_u, &s->u_size);
    s->v = WP9ParseHex(hex_v, &s->v_size);
    s->pic.y = s->y;
    s->pic.u = s->u;
    s->pic.v = s->v;
    s->pic.y_stride = (int)y_stride;
    s->pic.uv_stride = (int)uv_stride;
    if (colorspace & WEBP_CSP_ALPHA_BIT) {
      s->a = WP9ParseHex(hex_a, &s->a_size);
      s->pic.a = s->a;
      s->pic.a_stride = (int)a_stride;
    }
  }
}

static void Release(Setup* s) {
  free(s->argb_bytes);
  free(s->y);
  free(s->u);
  free(s->v);
  free(s->a);
}

int main(int argc, char* argv[]) {
  char command[32];
  (void)argc;
  (void)argv;
  while (scanf("%31s", command) == 1) {
    char f[12][32];
    char hex_argb[1 << 16], hex_y[1 << 16], hex_u[1 << 16], hex_v[1 << 16],
        hex_a[1 << 16];
    Setup s;
    int i;
    if (strcmp(command, "rescale") == 0) {
      for (i = 0; i < 10; ++i) {
        if (scanf("%31s", f[i]) != 1) return 2;
      }
    } else if (strcmp(command, "crop") == 0) {
      for (i = 0; i < 12; ++i) {
        if (scanf("%31s", f[i]) != 1) return 2;
      }
    } else {
      fprintf(stderr, "picture_rescale: unknown command %s\n", command);
      return 2;
    }
    if (scanf("%65535s %65535s %65535s %65535s %65535s", hex_argb, hex_y, hex_u,
              hex_v, hex_a) != 5) {
      return 2;
    }
    Load(&s, (int)WP9ParseInt(f[0]), (int)WP9ParseInt(f[1]), (int)WP9ParseInt(f[2]),
         (int)WP9ParseInt(f[3]), WP9ParseInt(f[4]), WP9ParseInt(f[5]),
         WP9ParseInt(f[6]), WP9ParseInt(f[7]), hex_argb, hex_y, hex_u, hex_v, hex_a);
    if (strcmp(command, "rescale") == 0) {
      int ret = WebPPictureRescale(&s.pic, (int)WP9ParseInt(f[8]),
                                   (int)WP9ParseInt(f[9]));
      printf("ret=%d\n", ret);
    } else {
      int ret = WebPPictureCrop(&s.pic, (int)WP9ParseInt(f[8]), (int)WP9ParseInt(f[9]),
                                (int)WP9ParseInt(f[10]), (int)WP9ParseInt(f[11]));
      printf("ret=%d\n", ret);
    }
    PrintVisible(&s.pic);
    WebPPictureFree(&s.pic);
    Release(&s);
  }
  return 0;
}
