/* Supplemental regression for byte-aligned external high-bit-depth validation. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "aom/aom_encoder.h"
#include "aom/aomcx.h"

#define CHECK(c) do { if (!(c)) { fprintf(stderr, "check failed at line %d: %s\n", __LINE__, #c); return 1; } } while (0)

int main(int argc, char **argv) {
  CHECK(argc == 4);
  const unsigned depth = (unsigned)atoi(argv[1]);
  const unsigned offset = (unsigned)atoi(argv[2]);
  const unsigned invalid_plane = (unsigned)atoi(argv[3]);
  CHECK(depth == 8 || depth == 10 || depth == 12);
  CHECK(offset <= 1 && invalid_plane <= 3);
  /* calloc supplies aligned storage; offset 1 forces the hosted failure's parity.
   * Keep valid samples zero, as in the original small-width/height fixture.
   * This checks input validation, not general unaligned internal pixel storage. */
  unsigned char *allocation = calloc(1, 2 * 16 * 16 * 3 + 2);
  CHECK(allocation != NULL && ((uintptr_t)allocation & 1) == 0);
  aom_image_t image;
  CHECK(aom_img_wrap(&image, AOM_IMG_FMT_I42016, 16, 16, 1,
                     allocation + offset) == &image);
  CHECK(((uintptr_t)image.planes[0] & 1) == offset);
  if (invalid_plane) {
    const unsigned plane = invalid_plane - 1;
    const unsigned width = (unsigned)aom_img_plane_width(&image, (int)plane);
    const unsigned height = (unsigned)aom_img_plane_height(&image, (int)plane);
    const uint16_t invalid = (uint16_t)(1u << depth);
    memcpy(image.planes[plane] + (height - 1) * image.stride[plane] +
               2 * (width - 1), &invalid, sizeof(invalid));
  }
  aom_codec_enc_cfg_t config;
  CHECK(aom_codec_enc_config_default(aom_codec_av1_cx(), &config,
                                     AOM_USAGE_ALL_INTRA) == AOM_CODEC_OK);
  config.g_w = 16;
  config.g_h = 16;
  config.g_limit = 1;
  config.g_lag_in_frames = 0;
  config.g_threads = 2;
  config.g_bit_depth = (aom_bit_depth_t)depth;
  config.g_input_bit_depth = depth;
  config.g_profile = depth == 12 ? 2 : 0;
  aom_codec_ctx_t encoder = { 0 };
  CHECK(aom_codec_enc_init(&encoder, aom_codec_av1_cx(), &config,
                           AOM_CODEC_USE_HIGHBITDEPTH) == AOM_CODEC_OK);
  CHECK(aom_codec_control(&encoder, AOME_SET_CPUUSED, 8) == AOM_CODEC_OK);
  CHECK(aom_codec_control(&encoder, AOME_SET_VALIDATE_HBD_INPUT, 1) == AOM_CODEC_OK);
  const aom_codec_err_t result = aom_codec_encode(&encoder, &image, 0, 1, 0);
  if (invalid_plane) {
    CHECK(result == AOM_CODEC_INVALID_PARAM);
    const char *detail = aom_codec_error_detail(&encoder);
    CHECK(detail != NULL);
    CHECK(strcmp(detail, "Input pixel value out of range for encoder bit depth") == 0);
    CHECK(printf("%d:%s\n", (int)result, detail) > 0);
  } else {
    CHECK(result == AOM_CODEC_OK);
    aom_codec_iter_t iterator = NULL;
    const aom_codec_cx_pkt_t *packet;
    unsigned frames = 0;
    while ((packet = aom_codec_get_cx_data(&encoder, &iterator)) != NULL) {
      if (packet->kind != AOM_CODEC_CX_FRAME_PKT) continue;
      CHECK(fwrite(packet->data.frame.buf, 1, packet->data.frame.sz, stdout) ==
            packet->data.frame.sz);
      ++frames;
    }
    CHECK(frames == 1);
  }
  CHECK(aom_codec_destroy(&encoder) == AOM_CODEC_OK);
  aom_img_free(&image);
  free(allocation);
  return 0;
}
