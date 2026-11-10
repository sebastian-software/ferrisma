// WP9 probe: the WebPPictureImport* functions, WebPPictureARGBToYUVA (with and
// without dithering and sharp YUV) and WebPPictureYUVAToARGB (src/enc/picture_csp_enc.c).
// Commands, one per line on standard input:
//
//   import KIND USE_ARGB W H STRIDE HEX
//       KIND is rgb, rgba, bgr, bgra, rgbx or bgrx; HEX holds the STRIDE bytes of
//       each of the H rows, as the caller's buffer, with the padding.
//   argb2yuva MODE COLORSPACE DITHER_BITS W H STRIDE HEX
//       MODE is 0 (WebPPictureARGBToYUVA), 1 (WebPPictureARGBToYUVADithered) or
//       2 (WebPPictureSharpARGBToYUVA); HEX holds STRIDE words per row as LE bytes.
//   yuva2argb COLORSPACE W H Y_STRIDE UV_STRIDE A_STRIDE HEX_Y HEX_U HEX_V HEX_A
//       the caller's YUVA planes; HEX_A is "-" without alpha.
//
// Each command prints `ret=`, `error=`, the colorspace and the visible samples of the
// planes or the ARGB words the library produced.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/enc/vp8i_enc.h"
#include "src/webp/encode.h"
#include "wp9_support.h"

static void PrintYuva(const WebPPicture* pic) {
  int w = pic->width, h = pic->height;
  int uv_w = (w + 1) / 2, uv_h = (h + 1) / 2;
  printf("colorspace=%u\n", (unsigned)pic->colorspace);
  printf("use_argb=%d\n", pic->use_argb);
  if (pic->use_argb) {
    WP9PrintArgb("argb", pic->argb, w, h, pic->argb_stride);
    return;
  }
  WP9PrintPlane("y", pic->y, w, h, pic->y_stride);
  WP9PrintPlane("u", pic->u, uv_w, uv_h, pic->uv_stride);
  WP9PrintPlane("v", pic->v, uv_w, uv_h, pic->uv_stride);
  if (pic->a != NULL) WP9PrintPlane("a", pic->a, w, h, pic->a_stride);
}

static void DoImport(const char* kind, int use_argb, int w, int h, long stride,
                     const char* hex) {
  size_t size = 0;
  uint8_t* buffer = WP9ParseHex(hex, &size);
  WebPPicture pic;
  int ret;
  WebPPictureInit(&pic);
  pic.use_argb = use_argb;
  pic.width = w;
  pic.height = h;
  if (strcmp(kind, "rgb") == 0) {
    ret = WebPPictureImportRGB(&pic, buffer, (int)stride);
  } else if (strcmp(kind, "rgba") == 0) {
    ret = WebPPictureImportRGBA(&pic, buffer, (int)stride);
  } else if (strcmp(kind, "bgr") == 0) {
    ret = WebPPictureImportBGR(&pic, buffer, (int)stride);
  } else if (strcmp(kind, "bgra") == 0) {
    ret = WebPPictureImportBGRA(&pic, buffer, (int)stride);
  } else if (strcmp(kind, "rgbx") == 0) {
    ret = WebPPictureImportRGBX(&pic, buffer, (int)stride);
  } else if (strcmp(kind, "bgrx") == 0) {
    ret = WebPPictureImportBGRX(&pic, buffer, (int)stride);
  } else {
    fprintf(stderr, "picture_csp: unknown kind %s\n", kind);
    exit(2);
  }
  printf("ret=%d\n", ret);
  printf("error=%d\n", (int)pic.error_code);
  PrintYuva(&pic);
  WebPPictureFree(&pic);
  free(buffer);
}

static void DoArgbToYuva(int mode, int colorspace, float dithering, int w, int h,
                         long stride, const char* hex) {
  size_t size = 0;
  uint8_t* bytes = WP9ParseHex(hex, &size);
  uint32_t* argb = (uint32_t*)malloc(4 * (size_t)stride * (size_t)h + 4);
  WebPPicture pic;
  int ret = 0;
  size_t i;
  if (argb == NULL) exit(2);
  for (i = 0; i < (size_t)stride * (size_t)h; ++i) {
    memcpy(&argb[i], bytes + 4 * i, 4);
  }
  WebPPictureInit(&pic);
  pic.use_argb = 1;
  pic.width = w;
  pic.height = h;
  pic.argb_stride = (int)stride;
  pic.argb = argb;
  if (mode == 0) {
    ret = WebPPictureARGBToYUVA(&pic, (WebPEncCSP)colorspace);
  } else if (mode == 1) {
    ret = WebPPictureARGBToYUVADithered(&pic, (WebPEncCSP)colorspace, dithering);
  } else {
    ret = WebPPictureSharpARGBToYUVA(&pic);
  }
  printf("ret=%d\n", ret);
  printf("error=%d\n", (int)pic.error_code);
  PrintYuva(&pic);
  WebPPictureFree(&pic);
  free(argb);
  free(bytes);
}

static void DoYuvaToArgb(int colorspace, int w, int h, long y_stride, long uv_stride,
                         long a_stride, const char* hex_y, const char* hex_u,
                         const char* hex_v, const char* hex_a) {
  size_t y_size = 0, u_size = 0, v_size = 0, a_size = 0;
  uint8_t* y = WP9ParseHex(hex_y, &y_size);
  uint8_t* u = WP9ParseHex(hex_u, &u_size);
  uint8_t* v = WP9ParseHex(hex_v, &v_size);
  uint8_t* a = WP9ParseHex(hex_a, &a_size);
  WebPPicture pic;
  int ret;
  WebPPictureInit(&pic);
  pic.colorspace = (WebPEncCSP)colorspace;
  pic.width = w;
  pic.height = h;
  pic.y = y;
  pic.u = u;
  pic.v = v;
  pic.y_stride = (int)y_stride;
  pic.uv_stride = (int)uv_stride;
  pic.a = (colorspace & WEBP_CSP_ALPHA_BIT) ? a : NULL;
  pic.a_stride = (int)a_stride;
  ret = WebPPictureYUVAToARGB(&pic);
  printf("ret=%d\n", ret);
  printf("error=%d\n", (int)pic.error_code);
  PrintYuva(&pic);
  WebPPictureFree(&pic);
  free(y);
  free(u);
  free(v);
  free(a);
}

int main(int argc, char* argv[]) {
  char command[32];
  (void)argc;
  (void)argv;
  while (scanf("%31s", command) == 1) {
    if (strcmp(command, "import") == 0) {
      char kind[16], use_argb[16], w[16], h[16], stride[16], hex[1 << 16];
      if (scanf("%15s %15s %15s %15s %15s %65535s", kind, use_argb, w, h, stride,
                hex) != 6) {
        return 2;
      }
      DoImport(kind, (int)WP9ParseInt(use_argb), (int)WP9ParseInt(w),
               (int)WP9ParseInt(h), WP9ParseInt(stride), hex);
    } else if (strcmp(command, "argb2yuva") == 0) {
      char mode[16], colorspace[16], dither[16], w[16], h[16], stride[16], hex[1 << 16];
      if (scanf("%15s %15s %15s %15s %15s %15s %65535s", mode, colorspace, dither,
                w, h, stride, hex) != 7) {
        return 2;
      }
      DoArgbToYuva((int)WP9ParseInt(mode), (int)WP9ParseInt(colorspace),
                   WP9ParseFloatBits(dither), (int)WP9ParseInt(w),
                   (int)WP9ParseInt(h), WP9ParseInt(stride), hex);
    } else if (strcmp(command, "yuva2argb") == 0) {
      char colorspace[16], w[16], h[16], ys[16], uvs[16], as[16];
      char hex_y[1 << 16], hex_u[1 << 16], hex_v[1 << 16], hex_a[1 << 16];
      if (scanf("%15s %15s %15s %15s %15s %15s %65535s %65535s %65535s %65535s",
                colorspace, w, h, ys, uvs, as, hex_y, hex_u, hex_v, hex_a) != 10) {
        return 2;
      }
      DoYuvaToArgb((int)WP9ParseInt(colorspace), (int)WP9ParseInt(w),
                   (int)WP9ParseInt(h), WP9ParseInt(ys), WP9ParseInt(uvs),
                   WP9ParseInt(as), hex_y, hex_u, hex_v, hex_a);
    } else {
      fprintf(stderr, "picture_csp: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
