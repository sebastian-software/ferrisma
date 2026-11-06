// WP1 probe: runs the byte, bit, endian and plane helpers of src/utils/utils.h,
// src/utils/endian_inl_utils.h and WebPCopyPlane on the values read from
// standard input, one command per line, and prints the results.
//
//   le HEX4                 GetLE16, GetLE24 and GetLE32 of four bytes
//   put16|put24|put32 V     PutLEnn of V into four zero bytes; prints `bytes=`
//   log2 N                  BitsLog2Floor(N), N > 0
//   ctz N                   BitsCtz(N), N != 0
//   memu32 HEX4             WebPMemToUint32 of four bytes
//   memi32 HEX4             WebPMemToInt32 of four bytes
//   wu32 V | wi32 V         WebPUint32ToMem / WebPInt32ToMem into four zero bytes
//   hle16 V | hle32 V       HToLE16 / HToLE32
//   bs16 V | bs32 V | bs64 V  BSwap16 / BSwap32 / BSwap64
//   csov N                  CheckSizeOverflow(N)
//   align A                 WEBP_ALIGN of the address A
//   copy_plane SS DS W H SO DO SL DL HEX
//                           WebPCopyPlane from src+SO (SL bytes, HEX, or "-" when
//                           SL is 0) to dst+DO (DL bytes, initially 0xa5); prints
//                           the destination as `dst=`
//
// tests/wp1_utils.rs generates the commands and the expected output.

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/endian_inl_utils.h"
#include "src/utils/utils.h"

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "utils_inline: bad hex digit %c\n", c);
  exit(2);
}

static void ParseHex(const char* text, uint8_t* out, size_t size) {
  size_t i;
  for (i = 0; i < size; ++i) {
    out[i] = (uint8_t)(HexValue(text[2 * i]) * 16 + HexValue(text[2 * i + 1]));
  }
}

static void PrintHexOf(const char* key, const uint8_t* data, size_t size) {
  ProbePrintHex(key, data, size);
}

int main(int argc, char* argv[]) {
  char command[24];
  ProbeCheckArgs(argc, 0, 0, "utils_inline < commands.txt");
  while (scanf("%23s", command) == 1) {
    if (strcmp(command, "le") == 0) {
      char hex[9];
      uint8_t data[4];
      if (scanf("%8s", hex) != 1) return 2;
      ParseHex(hex, data, sizeof(data));
      printf("get_le16=%d\n", GetLE16(data));
      printf("get_le24=%d\n", GetLE24(data));
      printf("get_le32=%u\n", (unsigned)GetLE32(data));
    } else if (strcmp(command, "put16") == 0 || strcmp(command, "put24") == 0 ||
               strcmp(command, "put32") == 0) {
      uint8_t data[4] = {0, 0, 0, 0};
      if (strcmp(command, "put16") == 0) {
        int value;
        if (scanf("%d", &value) != 1) return 2;
        PutLE16(data, value);
      } else if (strcmp(command, "put24") == 0) {
        int value;
        if (scanf("%d", &value) != 1) return 2;
        PutLE24(data, value);
      } else {
        unsigned int value;
        if (scanf("%u", &value) != 1) return 2;
        PutLE32(data, value);
      }
      PrintHexOf("bytes", data, sizeof(data));
    } else if (strcmp(command, "log2") == 0) {
      unsigned int value;
      if (scanf("%u", &value) != 1) return 2;
      printf("value=%d\n", BitsLog2Floor(value));
    } else if (strcmp(command, "ctz") == 0) {
      unsigned int value;
      if (scanf("%u", &value) != 1) return 2;
      printf("value=%d\n", BitsCtz(value));
    } else if (strcmp(command, "memu32") == 0 || strcmp(command, "memi32") == 0) {
      char hex[9];
      uint8_t data[4];
      if (scanf("%8s", hex) != 1) return 2;
      ParseHex(hex, data, sizeof(data));
      if (strcmp(command, "memu32") == 0) {
        printf("value=%u\n", (unsigned)WebPMemToUint32(data));
      } else {
        printf("value=%d\n", (int)WebPMemToInt32(data));
      }
    } else if (strcmp(command, "wu32") == 0 || strcmp(command, "wi32") == 0) {
      uint8_t data[4] = {0, 0, 0, 0};
      if (strcmp(command, "wu32") == 0) {
        unsigned int value;
        if (scanf("%u", &value) != 1) return 2;
        WebPUint32ToMem(data, value);
      } else {
        int value;
        if (scanf("%d", &value) != 1) return 2;
        WebPInt32ToMem(data, value);
      }
      PrintHexOf("bytes", data, sizeof(data));
    } else if (strcmp(command, "hle16") == 0) {
      unsigned int value;
      if (scanf("%u", &value) != 1) return 2;
      printf("value=%u\n", (unsigned)HToLE16((uint16_t)value));
    } else if (strcmp(command, "hle32") == 0) {
      unsigned int value;
      if (scanf("%u", &value) != 1) return 2;
      printf("value=%u\n", (unsigned)HToLE32((uint32_t)value));
    } else if (strcmp(command, "bs16") == 0) {
      unsigned int value;
      if (scanf("%u", &value) != 1) return 2;
      printf("value=%u\n", (unsigned)BSwap16((uint16_t)value));
    } else if (strcmp(command, "bs32") == 0) {
      unsigned int value;
      if (scanf("%u", &value) != 1) return 2;
      printf("value=%u\n", (unsigned)BSwap32((uint32_t)value));
    } else if (strcmp(command, "bs64") == 0) {
      unsigned long long value;
      if (scanf("%llu", &value) != 1) return 2;
      printf("value=%llu\n", (unsigned long long)BSwap64((uint64_t)value));
    } else if (strcmp(command, "csov") == 0) {
      unsigned long long value;
      if (scanf("%llu", &value) != 1) return 2;
      printf("value=%d\n", CheckSizeOverflow((uint64_t)value));
    } else if (strcmp(command, "align") == 0) {
      unsigned long long address;
      if (scanf("%llu", &address) != 1) return 2;
      printf("value=%llu\n",
             (unsigned long long)(uintptr_t)WEBP_ALIGN((void*)(uintptr_t)address));
    } else if (strcmp(command, "copy_plane") == 0) {
      int src_stride, dst_stride, width, height;
      size_t src_offset, dst_offset, src_len, dst_len;
      uint8_t *src, *dst;
      size_t i;
      char* hex = NULL;
      if (scanf("%d %d %d %d %zu %zu %zu %zu", &src_stride, &dst_stride, &width,
                &height, &src_offset, &dst_offset, &src_len, &dst_len) != 8) {
        return 2;
      }
      hex = (char*)malloc(2 * src_len + 2);
      if (hex == NULL) return 2;
      if (scanf("%s", hex) != 1) return 2;
      src = (uint8_t*)malloc(src_len > 0 ? src_len : 1);
      dst = (uint8_t*)malloc(dst_len > 0 ? dst_len : 1);
      if (src == NULL || dst == NULL) return 2;
      if (src_len > 0) ParseHex(hex, src, src_len);
      for (i = 0; i < dst_len; ++i) dst[i] = 0xa5;
      WebPCopyPlane(src + src_offset, src_stride, dst + dst_offset, dst_stride,
                    width, height);
      PrintHexOf("dst", dst, dst_len);
      free(src);
      free(dst);
      free(hex);
    } else {
      fprintf(stderr, "utils_inline: unknown command %s\n", command);
      return 2;
    }
  }
  return 0;
}
