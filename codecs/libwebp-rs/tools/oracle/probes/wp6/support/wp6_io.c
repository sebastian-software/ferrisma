// WP6 probe helpers; see wp6_io.h.

#include "wp6_io.h"

#include <ctype.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

char* Wp6ReadToken(void) {
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

long Wp6ReadLong(void) {
  char* token = Wp6ReadToken();
  char* end;
  long value;
  if (token == NULL) exit(2);
  value = strtol(token, &end, 10);
  if (*end != '\0') {
    fprintf(stderr, "wp6: bad integer %s\n", token);
    exit(2);
  }
  free(token);
  return value;
}

static int HexValue(char c) {
  if (c >= '0' && c <= '9') return c - '0';
  if (c >= 'a' && c <= 'f') return c - 'a' + 10;
  fprintf(stderr, "wp6: bad hex digit %c\n", c);
  exit(2);
}

uint8_t* Wp6ReadBytes(size_t* size) {
  char* token = Wp6ReadToken();
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

long* Wp6ReadList(size_t* count) {
  char* token = Wp6ReadToken();
  char* cursor;
  size_t n = 0, cap = 1;
  long* values;
  if (token == NULL) exit(2);
  for (cursor = token; *cursor != '\0'; ++cursor) {
    if (*cursor == ',') ++cap;
  }
  values = (long*)calloc(cap + 1, sizeof(*values));
  if (values == NULL) exit(2);
  if (strcmp(token, "-") != 0) {
    cursor = token;
    for (;;) {
      char* end;
      values[n++] = strtol(cursor, &end, 10);
      if (end == cursor || (*end != ',' && *end != '\0')) {
        fprintf(stderr, "wp6: bad list %s\n", token);
        exit(2);
      }
      if (*end == '\0') break;
      cursor = end + 1;
    }
  }
  free(token);
  *count = n;
  return values;
}

uint8_t* Wp6ReadFile(const char* path, long offset, long size) {
  FILE* file = fopen(path, "rb");
  uint8_t* buffer;
  if (file == NULL || size < 0 || offset < 0) {
    fprintf(stderr, "wp6: cannot open %s\n", path);
    exit(2);
  }
  buffer = (uint8_t*)calloc((size_t)size + 1, 1);
  if (buffer == NULL) exit(2);
  if (fseek(file, offset, SEEK_SET) != 0 ||
      fread(buffer, 1, (size_t)size, file) != (size_t)size) {
    fprintf(stderr, "wp6: cannot read %ld bytes at %ld of %s\n", size, offset,
            path);
    exit(2);
  }
  fclose(file);
  return buffer;
}

uint64_t Wp6Digest(const uint8_t* data, size_t size) {
  uint64_t hash = 0xcbf29ce484222325ULL;
  size_t i;
  for (i = 0; i < size; ++i) {
    hash ^= data[i];
    hash *= 0x100000001b3ULL;
  }
  return hash;
}

void Wp6PrintDigest(const char* key, const uint8_t* data, size_t size) {
  printf("%s=%016llx\n", key, (unsigned long long)Wp6Digest(data, size));
}
