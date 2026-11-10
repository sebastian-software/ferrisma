/* Supplemental pthread-create failure oracle for the original public API. */
#include <errno.h>
#include <pthread.h>
#include <stdatomic.h>
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "aom/aom_decoder.h"
#include "aom/aom_encoder.h"
#include "aom/aomcx.h"
#include "aom/aomdx.h"

static atomic_uint calls, failures;
static unsigned failure_point;

int port_aom_pthread_create(pthread_t *thread, const pthread_attr_t *attributes,
                           void *(*start)(void *), void *argument) {
  unsigned index = atomic_fetch_add(&calls, 1) + 1;
  if (failure_point && index == failure_point) {
    atomic_fetch_add(&failures, 1);
    return EAGAIN;
  }
  return pthread_create(thread, attributes, start, argument);
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
  failure_point = (unsigned)strtoul(argv[2], NULL, 10);
  aom_codec_ctx_t context;
  memset(&context, 0, sizeof(context));
  int init, control = -1, operation = -1, destroy = -1;
  char detail[2048] = { 0 };
  if (strcmp(argv[1], "encoder") == 0) {
    aom_codec_enc_cfg_t config;
    if (aom_codec_enc_config_default(aom_codec_av1_cx(), &config,
                                     AOM_USAGE_ALL_INTRA)) return 2;
    config.g_w = 128;
    config.g_h = 96;
    config.g_threads = 4;
    config.g_lag_in_frames = 0;
    config.rc_end_usage = AOM_Q;
    init = aom_codec_enc_init(&context, aom_codec_av1_cx(), &config, 0);
    if (!init) {
      control = aom_codec_control(&context, AOME_SET_CPUUSED, 8);
      if (!control) control = aom_codec_control(&context, AV1E_SET_ROW_MT, 1u);
      unsigned char samples[128 * 96 * 3 / 2];
      for (size_t i = 0; i < sizeof(samples); ++i) samples[i] = (unsigned char)(i * 97 + i / 31);
      aom_image_t image;
      if (!aom_img_wrap(&image, AOM_IMG_FMT_I420, 128, 96, 1, samples)) return 2;
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
    const aom_codec_dec_cfg_t config = { 4, 0, 0, 1 };
    init = aom_codec_dec_init(&context, aom_codec_av1_dx(), &config, 0);
    if (!init) {
      control = aom_codec_control(&context, AV1D_SET_ROW_MT, 1u);
      if (!control) operation = aom_codec_decode(&context, packet, bytes, NULL);
    }
    free(packet);
  } else return 2;
  const char *error = aom_codec_error_detail(&context);
  if (error) snprintf(detail, sizeof(detail), "%s", error);
  if (!init) destroy = aom_codec_destroy(&context);
  printf("{\"init\":%d,\"control\":%d,\"operation\":%d,\"destroy\":%d,"
         "\"calls\":%u,\"failures\":%u,\"detail\":",
         init, control, operation, destroy, atomic_load(&calls), atomic_load(&failures));
  quote(detail);
  puts("}");
  return 0;
}
