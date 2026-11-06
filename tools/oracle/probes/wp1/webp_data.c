// WP1 probe: runs WebPDataInit, WebPDataCopy and WebPDataClear from
// src/webp/mux_types.h on the commands read from standard input, one per line,
// and prints the observable state. tests/wp1_webp_data.rs compares the output
// with the Rust `WebPData` operations.
//
//   init                   WebPDataInit of a fresh struct; prints size and bytes
//   copy LEN HEX           a source of LEN bytes from HEX; WebPDataCopy into a
//                          fresh destination (copied, size, bytes), then
//                          WebPDataClear of the destination (cleared_size)
//   copy0                  a source with bytes == NULL and size 0
//   copyempty              a source with a non-NULL pointer and size 0
//
// Usage: webp_data < commands.txt

#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/webp/mux_types.h"

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "webp_data: bad hex digit %c\n", c);
  exit(2);
}

static void PrintData(const WebPData* data) {
  printf("size=%zu\n", data->size);
  ProbePrintHex("bytes", data->bytes, data->size);
}

static void Copy(const WebPData* src) {
  WebPData dst;
  int copied;
  WebPDataInit(&dst);
  copied = WebPDataCopy(src, &dst);
  printf("copied=%d\n", copied);
  PrintData(&dst);
  WebPDataClear(&dst);
  printf("cleared_size=%zu\n", dst.size);
}

int main(int argc, char* argv[]) {
  char command[16];
  static const uint8_t kEmpty[1] = {0};
  ProbeCheckArgs(argc, 0, 0, "webp_data < commands.txt");
  while (scanf("%15s", command) == 1) {
    if (strcmp(command, "init") == 0) {
      WebPData data;
      WebPDataInit(&data);
      PrintData(&data);
    } else if (strcmp(command, "copy") == 0) {
      size_t length, i;
      char* hex;
      uint8_t* bytes;
      WebPData src;
      if (scanf("%zu", &length) != 1) return 2;
      hex = (char*)malloc(2 * length + 2);
      bytes = (uint8_t*)malloc(length + 1);
      if (hex == NULL || bytes == NULL) return 2;
      if (scanf("%s", hex) != 1) return 2;
      for (i = 0; i < length; ++i) {
        bytes[i] = (uint8_t)(HexValue(hex[2 * i]) * 16 + HexValue(hex[2 * i + 1]));
      }
      src.bytes = bytes;
      src.size = length;
      Copy(&src);
      free(hex);
      free(bytes);
    } else if (strcmp(command, "copy0") == 0) {
      WebPData src;
      src.bytes = NULL;
      src.size = 0;
      Copy(&src);
    } else if (strcmp(command, "copyempty") == 0) {
      WebPData src;
      src.bytes = kEmpty;
      src.size = 0;
      Copy(&src);
    } else {
      fprintf(stderr, "webp_data: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
