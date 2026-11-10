// Shared helpers for the oracle probes (WP0); see probe_support.h.

#include "probe_support.h"

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

static void Fail(const char* what, const char* path) {
  fprintf(stderr, "probe: cannot %s %s\n", what, path);
  exit(2);
}

uint8_t* ProbeReadInput(const char* path, size_t* size) {
  const int use_stdin = (path == NULL || strcmp(path, "-") == 0);
  const char* name = use_stdin ? "standard input" : path;
  FILE* const file = use_stdin ? stdin : fopen(path, "rb");
  size_t capacity = 4096, length = 0;
  uint8_t* data = (uint8_t*)malloc(capacity);
  if (file == NULL) Fail("open", name);
  if (data == NULL) Fail("allocate memory for", name);
  for (;;) {
    const size_t count = fread(data + length, 1, capacity - length, file);
    length += count;
    if (length < capacity) {
      if (ferror(file)) Fail("read", name);
      break;
    }
    capacity *= 2;
    data = (uint8_t*)realloc(data, capacity);
    if (data == NULL) Fail("allocate memory for", name);
  }
  if (!use_stdin) fclose(file);
  *size = length;
  return data;
}

void ProbePrintHex(const char* key, const uint8_t* data, size_t size) {
  size_t i;
  printf("%s=", key);
  for (i = 0; i < size; ++i) printf("%02x", data[i]);
  printf("\n");
}

void ProbeCheckArgs(int argc, int min_args, int max_args, const char* usage) {
  if (argc - 1 < min_args || argc - 1 > max_args) {
    fprintf(stderr, "usage: %s\n", usage);
    exit(2);
  }
}
