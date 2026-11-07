// Helpers shared by the WP9 probes (see wp9_support.h).

#include "wp9_support.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static int HexDigit(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  if (c >= 'A' && c <= 'F') return c - 'A' + 10;
  fprintf(stderr, "wp9: bad hex digit %c\n", c);
  exit(2);
}

uint8_t* WP9ParseHex(const char* hex, size_t* size) {
  size_t length = strlen(hex);
  size_t i;
  uint8_t* bytes;
  if (strcmp(hex, "-") == 0) {
    length = 0;
  }
  if (length % 2 != 0) {
    fprintf(stderr, "wp9: odd hex length\n");
    exit(2);
  }
  bytes = (uint8_t*)malloc(length / 2 + 1);
  if (bytes == NULL) exit(2);
  for (i = 0; i < length / 2; ++i) {
    bytes[i] = (uint8_t)(HexDigit(hex[2 * i]) * 16 + HexDigit(hex[2 * i + 1]));
  }
  *size = length / 2;
  return bytes;
}

void WP9PrintHex(const char* key, const uint8_t* buffer, size_t size) {
  size_t i;
  printf("%s=", key);
  for (i = 0; i < size; ++i) printf("%02x", buffer[i]);
  printf("\n");
}

void WP9PrintPlane(const char* key, const uint8_t* plane, int width, int height,
                   long stride) {
  int y;
  printf("%s=", key);
  for (y = 0; y < height; ++y) {
    int x;
    const uint8_t* row = plane + (long)y * stride;
    for (x = 0; x < width; ++x) printf("%02x", row[x]);
  }
  printf("\n");
}

void WP9PrintArgb(const char* key, const uint32_t* argb, int width, int height,
                  long stride) {
  int y;
  printf("%s=", key);
  for (y = 0; y < height; ++y) {
    int x;
    const uint32_t* row = argb + (long)y * stride;
    for (x = 0; x < width; ++x) {
      uint32_t value = row[x];
      uint8_t bytes[4];
      int b;
      memcpy(bytes, &value, sizeof(bytes));
      for (b = 0; b < 4; ++b) printf("%02x", bytes[b]);
    }
  }
  printf("\n");
}

long WP9ParseInt(const char* text) {
  char* end = NULL;
  long value = strtol(text, &end, 10);
  if (end == text || *end != '\0') {
    fprintf(stderr, "wp9: bad integer %s\n", text);
    exit(2);
  }
  return value;
}

float WP9ParseFloatBits(const char* text) {
  char* end = NULL;
  unsigned long bits = strtoul(text, &end, 16);
  uint32_t word;
  float value;
  if (end == text || *end != '\0' || bits > 0xffffffffUL) {
    fprintf(stderr, "wp9: bad float bits %s\n", text);
    exit(2);
  }
  word = (uint32_t)bits;
  memcpy(&value, &word, sizeof(value));
  return value;
}
