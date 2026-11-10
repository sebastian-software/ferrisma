// WP9 probe: WebPPictureInitInternal, the WebPPictureInit wrapper, WebPValidatePicture,
// WebPPictureAlloc, WebPPictureFree and WebPPictureIsView (src/enc/picture_enc.c,
// src/enc/picture_rescale_enc.c). Commands, one per line on standard input:
//
//   init VERSION                     WebPPictureInitInternal on a picture filled with 0xa5
//   init_wrapper                     WebPPictureInit (the inline wrapper)
//   validate USE_ARGB COLORSPACE W H WebPValidatePicture on a fresh picture
//   alloc USE_ARGB COLORSPACE W H    WebPPictureAlloc, then WebPPictureFree
//
// The planes are printed as null flags and strides, not as contents, since their
// contents are not defined before the encoder writes them.

#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "src/enc/vp8i_enc.h"
#include "src/webp/encode.h"
#include "wp9_support.h"

static void PrintPicture(const char* prefix, const WebPPicture* pic) {
  printf("%suse_argb=%d\n", prefix, pic->use_argb);
  printf("%scolorspace=%u\n", prefix, (unsigned)pic->colorspace);
  printf("%swidth=%d\n", prefix, pic->width);
  printf("%sheight=%d\n", prefix, pic->height);
  printf("%sy_stride=%d\n", prefix, pic->y_stride);
  printf("%suv_stride=%d\n", prefix, pic->uv_stride);
  printf("%sa_stride=%d\n", prefix, pic->a_stride);
  printf("%sargb_stride=%d\n", prefix, pic->argb_stride);
  printf("%sy_null=%d\n", prefix, pic->y == NULL);
  printf("%su_null=%d\n", prefix, pic->u == NULL);
  printf("%sv_null=%d\n", prefix, pic->v == NULL);
  printf("%sa_null=%d\n", prefix, pic->a == NULL);
  printf("%sargb_null=%d\n", prefix, pic->argb == NULL);
  printf("%serror=%d\n", prefix, (int)pic->error_code);
  printf("%sis_view=%d\n", prefix, WebPPictureIsView(pic));
}

static void Fresh(WebPPicture* pic) { memset(pic, 0xa5, sizeof(*pic)); }

int main(int argc, char* argv[]) {
  char command[32];
  (void)argc;
  (void)argv;
  while (scanf("%31s", command) == 1) {
    WebPPicture pic;
    if (strcmp(command, "init") == 0) {
      char version[32];
      int ret;
      if (scanf("%31s", version) != 1) return 2;
      Fresh(&pic);
      ret = WebPPictureInitInternal(&pic, (int)WP9ParseInt(version));
      printf("ret=%d\n", ret);
      if (ret) PrintPicture("", &pic);
    } else if (strcmp(command, "init_wrapper") == 0) {
      int ret;
      Fresh(&pic);
      ret = WebPPictureInit(&pic);
      printf("ret=%d\n", ret);
      PrintPicture("", &pic);
    } else if (strcmp(command, "validate") == 0 || strcmp(command, "alloc") == 0) {
      char use_argb[32], colorspace[32], width[32], height[32];
      int ret;
      if (scanf("%31s %31s %31s %31s", use_argb, colorspace, width, height) != 4) {
        return 2;
      }
      WebPPictureInit(&pic);
      pic.use_argb = (int)WP9ParseInt(use_argb);
      pic.colorspace = (WebPEncCSP)WP9ParseInt(colorspace);
      pic.width = (int)WP9ParseInt(width);
      pic.height = (int)WP9ParseInt(height);
      if (strcmp(command, "validate") == 0) {
        ret = WebPValidatePicture(&pic);
        printf("ret=%d\n", ret);
        printf("error=%d\n", (int)pic.error_code);
      } else {
        ret = WebPPictureAlloc(&pic);
        printf("ret=%d\n", ret);
        PrintPicture("", &pic);
        WebPPictureFree(&pic);
        PrintPicture("freed_", &pic);
      }
    } else {
      fprintf(stderr, "picture_alloc: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
