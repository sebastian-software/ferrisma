/* Supplemental failure oracle; identical caller for the C and Rust archives.
 * Only codec object references are rewritten to these allocator observers.
 * The observer, caller, system runtime, and Rust unwind payload stay unhooked.
 */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "aom/aom_decoder.h"
#include "aom/aom_encoder.h"
#include "aom/aomcx.h"
#include "aom/aomdx.h"

static struct { void *pointer; size_t bytes; } allocations[32768];
static size_t calls, failure_point, failures, live, live_bytes, unknown_frees;

static void remember(void *p, size_t bytes) {
  if (!p) return;
  for (size_t i = 0; i < sizeof(allocations) / sizeof(*allocations); ++i) {
    if (allocations[i].pointer) continue;
    allocations[i].pointer = p;
    allocations[i].bytes = bytes;
    ++live;
    live_bytes += bytes;
    return;
  }
  abort();
}

static void forget(void *p) {
  if (!p) return;
  for (size_t i = 0; i < sizeof(allocations) / sizeof(*allocations); ++i) {
    if (allocations[i].pointer != p) continue;
    --live;
    live_bytes -= allocations[i].bytes;
    allocations[i].pointer = NULL;
    return;
  }
  ++unknown_frees;
}

static int fail(void) {
  ++calls;
  if (failure_point && calls == failure_point) {
    ++failures;
    return 1;
  }
  return 0;
}

void *port_aom_malloc(size_t bytes) {
  if (fail()) return NULL;
  void *p = malloc(bytes);
  remember(p, bytes);
  return p;
}

void *port_aom_calloc(size_t count, size_t bytes) {
  if (fail()) return NULL;
  void *p = calloc(count, bytes);
  remember(p, count * bytes);
  return p;
}

void *port_aom_realloc(void *old, size_t bytes) {
  if (fail()) return NULL;
  void *p = realloc(old, bytes);
  if (p || bytes == 0) {
    forget(old);
    remember(p, bytes);
  }
  return p;
}

void port_aom_free(void *p) {
  forget(p);
  free(p);
}

static void quote(const char *s) {
  putchar('"');
  for (; *s; ++s) {
    const unsigned char c = (unsigned char)*s;
    if (c == '"' || c == '\\') printf("\\%c", c);
    else if (c < 32) printf("\\u%04x", c);
    else putchar(c);
  }
  putchar('"');
}

int main(int argc, char **argv) {
  if (argc != 4) return 2;
  failure_point = (size_t)strtoull(argv[2], NULL, 10);
  aom_codec_ctx_t context;
  memset(&context, 0, sizeof(context));
  int init, control = -1, operation = -1, destroy = -1;
  char detail[2048] = { 0 };
  if (strcmp(argv[1], "encoder") == 0) {
    aom_codec_enc_cfg_t config;
    if (aom_codec_enc_config_default(aom_codec_av1_cx(), &config,
                                     AOM_USAGE_ALL_INTRA)) return 2;
    config.g_w = config.g_h = 32;
    config.g_threads = 1;
    config.g_lag_in_frames = 0;
    config.rc_end_usage = AOM_Q;
    init = aom_codec_enc_init(&context, aom_codec_av1_cx(), &config, 0);
    if (!init) {
      control = aom_codec_control(&context, AOME_SET_CPUUSED, 8);
      unsigned char samples[32 * 32 * 3 / 2];
      for (size_t i = 0; i < sizeof(samples); ++i) samples[i] = (unsigned char)(i * 97 + i / 31);
      aom_image_t image;
      if (!aom_img_wrap(&image, AOM_IMG_FMT_I420, 32, 32, 1, samples)) return 2;
      if (!control) operation = aom_codec_encode(&context, &image, 0, 1, 0);
      aom_img_free(&image);
    }
  } else if (strcmp(argv[1], "decoder") == 0) {
    FILE *file = fopen(argv[3], "rb");
    if (!file) return 2;
    unsigned char header[44];
    if (fread(header, 1, sizeof(header), file) != sizeof(header)) return 2;
    const size_t bytes = (size_t)header[32] | (size_t)header[33] << 8 |
                         (size_t)header[34] << 16 | (size_t)header[35] << 24;
    unsigned char *packet = malloc(bytes);
    if (!packet || fread(packet, 1, bytes, file) != bytes) return 2;
    fclose(file);
    const aom_codec_dec_cfg_t config = { 1, 0, 0, 1 };
    init = aom_codec_dec_init(&context, aom_codec_av1_dx(), &config, 0);
    if (!init) operation = aom_codec_decode(&context, packet, bytes, NULL);
    free(packet);
  } else return 2;
  const char *error = aom_codec_error_detail(&context);
  if (error) snprintf(detail, sizeof(detail), "%s", error);
  /* Failed init already destroys partial private state in the original API. */
  if (!init) destroy = aom_codec_destroy(&context);
  printf("{\"init\":%d,\"control\":%d,\"operation\":%d,\"destroy\":%d,\"calls\":%zu,"
         "\"failures\":%zu,\"live\":%zu,\"live_bytes\":%zu,\"unknown_frees\":%zu,\"detail\":",
         init, control, operation, destroy, calls, failures, live, live_bytes, unknown_frees);
  quote(detail);
  puts("}");
  return 0;
}
