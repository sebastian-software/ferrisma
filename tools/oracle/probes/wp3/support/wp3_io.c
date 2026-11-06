// WP3 probe helpers; see wp3_io.h.

#include "wp3_io.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* Wp3ReadToken(void) {
  size_t cap = 64, len = 0;
  int c;
  char* s = (char*)malloc(cap);
  if (s == NULL) exit(2);
  do {
    c = getchar();
  } while (c != EOF && isspace(c));
  if (c == EOF) {
    free(s);
    return NULL;
  }
  do {
    if (len + 1 >= cap) {
      char* grown;
      cap *= 2;
      grown = (char*)realloc(s, cap);
      if (grown == NULL) exit(2);
      s = grown;
    }
    s[len++] = (char)c;
    c = getchar();
  } while (c != EOF && !isspace(c));
  s[len] = '\0';
  return s;
}

long Wp3ReadLong(void) {
  char* token = Wp3ReadToken();
  char* end;
  long value;
  if (token == NULL) exit(2);
  value = strtol(token, &end, 10);
  if (*end != '\0') {
    fprintf(stderr, "wp3: bad integer %s\n", token);
    exit(2);
  }
  free(token);
  return value;
}

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  fprintf(stderr, "wp3: bad hex digit %c\n", c);
  exit(2);
}

uint8_t* Wp3ReadBytes(size_t* size) {
  char* token = Wp3ReadToken();
  size_t len, i;
  uint8_t* buffer;
  if (token == NULL) exit(2);
  len = (strcmp(token, "-") == 0) ? 0 : strlen(token);
  if (len % 2 != 0) exit(2);
  *size = len / 2;
  buffer = (uint8_t*)calloc(*size + 1, 1);
  if (buffer == NULL) exit(2);
  for (i = 0; i < *size; ++i) {
    buffer[i] =
        (uint8_t)(HexValue(token[2 * i]) * 16 + HexValue(token[2 * i + 1]));
  }
  free(token);
  return buffer;
}

uint32_t* Wp3ReadU32s(size_t* count) {
  char* token = Wp3ReadToken();
  size_t len, i;
  uint32_t* values;
  if (token == NULL) exit(2);
  len = (strcmp(token, "-") == 0) ? 0 : strlen(token);
  if (len % 8 != 0) exit(2);
  *count = len / 8;
  values = (uint32_t*)calloc(*count + 1, sizeof(*values));
  if (values == NULL) exit(2);
  for (i = 0; i < *count; ++i) {
    uint32_t v = 0;
    int k;
    for (k = 0; k < 8; ++k) v = (v << 4) | (uint32_t)HexValue(token[8 * i + k]);
    values[i] = v;
  }
  free(token);
  return values;
}

void Wp3PrintU32s(const char* key, const uint32_t* values, size_t count) {
  size_t i;
  printf("%s=", key);
  for (i = 0; i < count; ++i) printf("%08x", values[i]);
  printf("\n");
}
