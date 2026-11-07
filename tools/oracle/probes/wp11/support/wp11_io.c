// WP11 probe helpers; see wp11_io.h.

#include "wp11_io.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* Wp11ReadToken(void) {
  size_t capacity = 64, size = 0;
  char* token;
  int c;
  do {
    c = getchar();
  } while (c == ' ' || c == '\n' || c == '\t' || c == '\r');
  if (c == EOF) return NULL;
  token = (char*)malloc(capacity);
  if (token == NULL) exit(2);
  while (c != EOF && c != ' ' && c != '\n' && c != '\t' && c != '\r') {
    if (size + 1 >= capacity) {
      char* const grown = (char*)realloc(token, capacity *= 2);
      if (grown == NULL) exit(2);
      token = grown;
    }
    token[size++] = (char)c;
    c = getchar();
  }
  token[size] = '\0';
  return token;
}

long Wp11ReadLong(void) {
  char* const token = Wp11ReadToken();
  char* end;
  long value;
  if (token == NULL) exit(2);
  value = strtol(token, &end, 10);
  if (*end != '\0') exit(2);
  free(token);
  return value;
}

static int HexDigit(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  exit(2);
}

uint32_t* Wp11ReadU32s(size_t* count) {
  char* const token = Wp11ReadToken();
  size_t length, i;
  uint32_t* values;
  if (token == NULL) exit(2);
  length = (strcmp(token, "-") == 0) ? 0 : strlen(token);
  if (length % 8 != 0) exit(2);
  *count = length / 8;
  values = (uint32_t*)calloc(*count + 1, sizeof(*values));
  if (values == NULL) exit(2);
  for (i = 0; i < length; ++i) {
    values[i / 8] = (values[i / 8] << 4) | (uint32_t)HexDigit(token[i]);
  }
  free(token);
  return values;
}

void Wp11PrintU32s(const char* key, const uint32_t* values, size_t count) {
  size_t i;
  printf("%s=", key);
  if (count == 0) printf("-");
  for (i = 0; i < count; ++i) printf("%08x", values[i]);
  printf("\n");
}

void Wp11PrintU64s(const char* key, const uint64_t* values, size_t count) {
  size_t i;
  printf("%s=", key);
  if (count == 0) printf("-");
  for (i = 0; i < count; ++i) printf("%016llx", (unsigned long long)values[i]);
  printf("\n");
}
