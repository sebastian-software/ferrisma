// WP9 probe: WebPCleanupTransparentArea, WebPBlendAlpha, WebPReplaceTransparentPixels
// and WebPPictureHasTransparency (src/enc/picture_tools_enc.c, src/enc/picture_csp_enc.c).
// The planes are the caller's buffers, so the probe prints them whole, padding
// included. Commands, one per line on standard input:
//
//   cleanup USE_ARGB COLORSPACE W H Y_STRIDE UV_STRIDE A_STRIDE ARGB_STRIDE HEX_ARGB HEX_Y HEX_U HEX_V HEX_A
//   blend   USE_ARGB COLORSPACE W H Y_STRIDE UV_STRIDE A_STRIDE ARGB_STRIDE BACKGROUND HEX_ARGB HEX_Y HEX_U HEX_V HEX_A
//   hastransp USE_ARGB COLORSPACE W H Y_STRIDE UV_STRIDE A_STRIDE ARGB_STRIDE HEX_ARGB HEX_Y HEX_U HEX_V HEX_A
//   replace W H ARGB_STRIDE COLOR HEX_ARGB
//
// A HEX argument is "-" for a plane that the picture does not use. The alpha plane
// is NULL when the colorspace has no alpha bit, whatever HEX_A holds.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/enc/vp8i_enc.h"
#include "src/webp/encode.h"
#include "wp9_support.h"

typedef struct {
  WebPPicture pic;
  uint8_t* argb_bytes;   // the ARGB words as bytes, native order
  size_t argb_size;
  uint8_t* y;
  size_t y_size;
  uint8_t* u;
  size_t u_size;
  uint8_t* v;
  size_t v_size;
  uint8_t* a;
  size_t a_size;
  int has_alpha;
} Setup;

static void Load(Setup* s, int use_argb, int colorspace, int w, int h, long y_stride,
                 long uv_stride, long a_stride, long argb_stride, const char* hex_argb,
                 const char* hex_y, const char* hex_u, const char* hex_v,
                 const char* hex_a) {
  int uv_h = (h + 1) / 2;
  WebPPictureInit(&s->pic);
  s->pic.use_argb = use_argb;
  s->pic.colorspace = (WebPEncCSP)colorspace;
  s->pic.width = w;
  s->pic.height = h;
  s->has_alpha = (colorspace & WEBP_CSP_ALPHA_BIT) != 0;
  if (use_argb) {
    s->argb_bytes = WP9ParseHex(hex_argb, &s->argb_size);
    s->pic.argb = (uint32_t*)s->argb_bytes;
    s->pic.argb_stride = (int)argb_stride;
  } else {
    s->argb_bytes = NULL;
    s->pic.argb = NULL;
    s->y = WP9ParseHex(hex_y, &s->y_size);
    s->u = WP9ParseHex(hex_u, &s->u_size);
    s->v = WP9ParseHex(hex_v, &s->v_size);
    s->pic.y = s->y;
    s->pic.u = s->u;
    s->pic.v = s->v;
    s->pic.y_stride = (int)y_stride;
    s->pic.uv_stride = (int)uv_stride;
    if (s->has_alpha) {
      s->a = WP9ParseHex(hex_a, &s->a_size);
      s->pic.a = s->a;
      s->pic.a_stride = (int)a_stride;
    }
  }
  (void)uv_h;
}

static void PrintBuffers(const Setup* s) {
  if (s->pic.use_argb) {
    WP9PrintHex("argb", s->argb_bytes, s->argb_size);
  } else {
    WP9PrintHex("y", s->y, s->y_size);
    WP9PrintHex("u", s->u, s->u_size);
    WP9PrintHex("v", s->v, s->v_size);
    if (s->has_alpha) WP9PrintHex("a", s->a, s->a_size);
  }
}

static void Free(Setup* s) {
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
    if (strcmp(command, "replace") == 0) {
      char w[16], h[16], stride[16], color[16], hex[1 << 16];
      Setup s;
      size_t n = 0;
      if (scanf("%15s %15s %15s %15s %65535s", w, h, stride, color, hex) != 5) {
        return 2;
      }
      memset(&s, 0, sizeof(s));
      Load(&s, 1, 0, (int)WP9ParseInt(w), (int)WP9ParseInt(h), 0, 0, 0,
           WP9ParseInt(stride), hex, "-", "-", "-", "-");
      (void)n;
      WebPReplaceTransparentPixels(&s.pic, (uint32_t)WP9ParseInt(color));
      PrintBuffers(&s);
      Free(&s);
    } else if (strcmp(command, "cleanup") == 0 || strcmp(command, "blend") == 0 ||
               strcmp(command, "hastransp") == 0) {
      char fields[8][32];
      char background[32], hex_argb[1 << 16], hex_y[1 << 16], hex_u[1 << 16],
          hex_v[1 << 16], hex_a[1 << 16];
      int is_blend = strcmp(command, "blend") == 0;
      int is_hastransp = strcmp(command, "hastransp") == 0;
      int i, count = 8;
      Setup s;
      int ret = 0;
      for (i = 0; i < count; ++i) {
        if (scanf("%31s", fields[i]) != 1) return 2;
      }
      if (is_blend && scanf("%31s", background) != 1) return 2;
      if (scanf("%65535s %65535s %65535s %65535s %65535s", hex_argb, hex_y, hex_u,
                hex_v, hex_a) != 5) {
        return 2;
      }
      memset(&s, 0, sizeof(s));
      Load(&s, (int)WP9ParseInt(fields[0]), (int)WP9ParseInt(fields[1]),
           (int)WP9ParseInt(fields[2]), (int)WP9ParseInt(fields[3]),
           WP9ParseInt(fields[4]), WP9ParseInt(fields[5]), WP9ParseInt(fields[6]),
           WP9ParseInt(fields[7]), hex_argb, hex_y, hex_u, hex_v, hex_a);
      if (strcmp(command, "cleanup") == 0) {
        WebPCleanupTransparentArea(&s.pic);
      } else if (is_blend) {
        WebPBlendAlpha(&s.pic, (uint32_t)WP9ParseInt(background));
      } else {
        ret = WebPPictureHasTransparency(&s.pic);
      }
      if (is_hastransp) {
        printf("ret=%d\n", ret);
      } else {
        PrintBuffers(&s);
      }
      Free(&s);
    } else {
      fprintf(stderr, "picture_tools: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
