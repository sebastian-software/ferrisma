// WP9 probe: WebPMemoryWriter (WebPMemoryWriterInit, WebPMemoryWrite and
// WebPMemoryWriterClear), WebPEncodingSetError and WebPReportProgress
// (src/enc/picture_enc.c, src/enc/webp_enc.c). Commands, one per line on standard
// input; the writer and the picture persist across commands:
//
//   writer_init                     WebPMemoryWriterInit
//   write SIZE HEX                  WebPMemoryWrite through the picture's custom_ptr
//   writer_clear                    WebPMemoryWriterClear
//   picture_init                    WebPPictureInit, which records no error
//   set CODE                        WebPEncodingSetError
//   hook THRESHOLD                  installs a progress hook that continues while the
//                                   percent is below THRESHOLD
//   progress STORE PERCENT          WebPReportProgress with a store that starts at STORE
//   progress_null PERCENT           WebPReportProgress with a NULL store
//
// Every command prints the results it produces and the state it leaves.

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/enc/vp8i_enc.h"
#include "src/webp/encode.h"
#include "wp9_support.h"

static int g_threshold = 0;
static int g_hook_calls = 0;

static int ProgressHook(int percent, const WebPPicture* picture) {
  (void)picture;
  ++g_hook_calls;
  return percent < g_threshold;
}

int main(int argc, char* argv[]) {
  char command[32];
  WebPMemoryWriter writer;
  WebPPicture pic;
  (void)argc;
  (void)argv;
  memset(&writer, 0xa5, sizeof(writer));
  memset(&pic, 0xa5, sizeof(pic));
  WebPPictureInit(&pic);
  pic.custom_ptr = &writer;
  while (scanf("%31s", command) == 1) {
    if (strcmp(command, "writer_init") == 0) {
      WebPMemoryWriterInit(&writer);
      printf("size=%zu\nmax_size=%zu\nmem_null=%d\n", writer.size, writer.max_size,
             writer.mem == NULL);
    } else if (strcmp(command, "write") == 0) {
      char size_text[32], hex[1 << 16];
      size_t size = 0;
      uint8_t* data;
      int ret;
      if (scanf("%31s %65535s", size_text, hex) != 2) return 2;
      data = WP9ParseHex(hex, &size);
      ret = WebPMemoryWrite(data, (size_t)WP9ParseInt(size_text), &pic);
      printf("ret=%d\n", ret);
      printf("size=%zu\nmax_size=%zu\n", writer.size, writer.max_size);
      if (writer.mem != NULL) WP9PrintHex("mem", writer.mem, writer.size);
      free(data);
    } else if (strcmp(command, "writer_clear") == 0) {
      WebPMemoryWriterClear(&writer);
      printf("size=%zu\nmax_size=%zu\nmem_null=%d\n", writer.size, writer.max_size,
             writer.mem == NULL);
    } else if (strcmp(command, "picture_init") == 0) {
      WebPPictureInit(&pic);
      pic.custom_ptr = &writer;
      printf("error=%d\n", (int)pic.error_code);
    } else if (strcmp(command, "set") == 0) {
      char code[32];
      int ret;
      if (scanf("%31s", code) != 1) return 2;
      ret = WebPEncodingSetError(&pic, (WebPEncodingError)WP9ParseInt(code));
      printf("ret=%d\n", ret);
      printf("error=%d\n", (int)pic.error_code);
    } else if (strcmp(command, "hook") == 0) {
      char threshold[32];
      if (scanf("%31s", threshold) != 1) return 2;
      g_threshold = (int)WP9ParseInt(threshold);
      g_hook_calls = 0;
      pic.progress_hook = ProgressHook;
      printf("hook=1\n");
    } else if (strcmp(command, "progress") == 0) {
      char store_text[32], percent_text[32];
      int store, ret;
      if (scanf("%31s %31s", store_text, percent_text) != 2) return 2;
      store = (int)WP9ParseInt(store_text);
      g_hook_calls = 0;
      ret = WebPReportProgress(&pic, (int)WP9ParseInt(percent_text), &store);
      printf("ret=%d\n", ret);
      printf("store=%d\n", store);
      printf("error=%d\n", (int)pic.error_code);
      printf("hook_calls=%d\n", g_hook_calls);
    } else if (strcmp(command, "progress_null") == 0) {
      char percent_text[32];
      int ret;
      g_hook_calls = 0;
      if (scanf("%31s", percent_text) != 1) return 2;
      ret = WebPReportProgress(&pic, (int)WP9ParseInt(percent_text), NULL);
      printf("ret=%d\n", ret);
      printf("error=%d\n", (int)pic.error_code);
      printf("hook_calls=%d\n", g_hook_calls);
    } else {
      fprintf(stderr, "writer_errors: unknown command %s\n", command);
      return 2;
    }
  }
  WebPMemoryWriterClear(&writer);
  return 0;
}
