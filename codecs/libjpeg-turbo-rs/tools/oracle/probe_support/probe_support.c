/* Shared helpers for the oracle probes (WP0); see probe_support.h. */

#include "probe_support.h"

#include <stdlib.h>
#include <string.h>

static void ProbeErrorExit(j_common_ptr cinfo)
{
  ProbeErrorMgr *err = (ProbeErrorMgr *)cinfo->err;

  err->error_code = err->pub.msg_code;
  (*err->pub.format_message) (cinfo, err->error_message);
  longjmp(err->setjmp_buffer, 1);
}

static void ProbeEmitMessage(j_common_ptr cinfo, int msg_level)
{
  ProbeErrorMgr *err = (ProbeErrorMgr *)cinfo->err;

  if (msg_level < 0) {
    if (err->warning_count < PROBE_MAX_WARNINGS)
      err->warning_codes[err->warning_count] = err->pub.msg_code;
    err->warning_count++;
  }
  (*err->std_emit_message) (cinfo, msg_level);
}

struct jpeg_error_mgr *ProbeErrorInit(ProbeErrorMgr *err)
{
  memset(err, 0, sizeof(*err));
  jpeg_std_error(&err->pub);
  err->std_emit_message = err->pub.emit_message;
  err->pub.error_exit = ProbeErrorExit;
  err->pub.emit_message = ProbeEmitMessage;
  return &err->pub;
}

void ProbePrintErrors(const ProbeErrorMgr *err)
{
  int i, kept;

  printf("error=%d\n", err->error_code);
  if (err->error_code != 0)
    printf("error_message=%s\n", err->error_message);
  printf("num_warnings=%ld\n", err->pub.num_warnings);
  printf("warnings=");
  kept = err->warning_count < PROBE_MAX_WARNINGS ? err->warning_count :
                                                   PROBE_MAX_WARNINGS;
  for (i = 0; i < kept; i++)
    printf(i ? ",%d" : "%d", err->warning_codes[i]);
  printf("\n");
}

static void Fail(const char *what, const char *path)
{
  fprintf(stderr, "probe: cannot %s %s\n", what, path);
  exit(2);
}

unsigned char *ProbeReadInput(const char *path, size_t *size)
{
  const int use_stdin = (path == NULL || strcmp(path, "-") == 0);
  const char *name = use_stdin ? "standard input" : path;
  FILE *file = use_stdin ? stdin : fopen(path, "rb");
  size_t capacity = 4096, length = 0;
  unsigned char *data = (unsigned char *)malloc(capacity);

  if (file == NULL) Fail("open", name);
  if (data == NULL) Fail("allocate memory for", name);
  for (;;) {
    size_t count = fread(data + length, 1, capacity - length, file);

    length += count;
    if (length < capacity) {
      if (ferror(file)) Fail("read", name);
      break;
    }
    capacity *= 2;
    data = (unsigned char *)realloc(data, capacity);
    if (data == NULL) Fail("allocate memory for", name);
  }
  if (!use_stdin) fclose(file);
  *size = length;
  return data;
}

void ProbePrintHex(const char *key, const unsigned char *data, size_t size)
{
  size_t i;

  printf("%s=", key);
  for (i = 0; i < size; i++) printf("%02x", data[i]);
  printf("\n");
}

void ProbeCheckArgs(int argc, int min_args, int max_args, const char *usage)
{
  if (argc - 1 < min_args || argc - 1 > max_args) {
    fprintf(stderr, "usage: %s\n", usage);
    exit(2);
  }
}
