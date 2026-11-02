/* Supplemental public-API oracle, compiled unchanged for original C and Rust. */
#include <stdint.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "aom/aom_decoder.h"
#include "aom/aom_encoder.h"
#include "aom/aomcx.h"
#include "aom/aomdx.h"

#define CHECK(c) do { if (!(c)) { fprintf(stderr, "check failed at line %d: %s\n", __LINE__, #c); return 1; } } while (0)

static void number(uint64_t value) {
  unsigned char bytes[8];
  for (unsigned i = 0; i < 8; ++i) bytes[i] = (unsigned char)(value >> (i * 8));
  if (fwrite(bytes, 1, 8, stdout) != 8) abort();
}

static void frame(const aom_image_t *image) {
  number(image->fmt);
  number(image->d_w);
  number(image->d_h);
  number(image->bit_depth);
  number(image->monochrome);
  number(image->x_chroma_shift);
  number(image->y_chroma_shift);
  number(image->cp);
  number(image->tc);
  number(image->mc);
  number(image->range);
  number(image->csp);
  const unsigned bytes = (image->fmt & AOM_IMG_FMT_HIGHBITDEPTH) ? 2 : 1;
  for (unsigned plane = 0; plane < (image->monochrome ? 1u : 3u); ++plane) {
    const unsigned width = plane ? (image->d_w + image->x_chroma_shift) >> image->x_chroma_shift : image->d_w;
    const unsigned height = plane ? (image->d_h + image->y_chroma_shift) >> image->y_chroma_shift : image->d_h;
    for (unsigned y = 0; y < height; ++y) {
      const unsigned char *row = image->planes[plane] + y * image->stride[plane];
      if (fwrite(row, bytes, width, stdout) != width) abort();
    }
  }
}

static int packets(aom_codec_ctx_t *encoder, aom_codec_ctx_t *decoder, unsigned *decoded) {
  aom_codec_iter_t iter = NULL;
  const aom_codec_cx_pkt_t *packet;
  while ((packet = aom_codec_get_cx_data(encoder, &iter)) != NULL) {
    if (packet->kind != AOM_CODEC_CX_FRAME_PKT) continue;
    number(packet->data.frame.sz);
    number((uint64_t)packet->data.frame.pts);
    number(packet->data.frame.duration);
    number(packet->data.frame.flags);
    CHECK(fwrite(packet->data.frame.buf, 1, packet->data.frame.sz, stdout) == packet->data.frame.sz);
    CHECK(aom_codec_decode(decoder, packet->data.frame.buf, packet->data.frame.sz, NULL) == AOM_CODEC_OK);
    aom_codec_iter_t frames = NULL;
    aom_image_t *image;
    while ((image = aom_codec_get_frame(decoder, &frames)) != NULL) {
      frame(image);
      ++*decoded;
    }
  }
  return 0;
}

int main(int argc, char **argv) {
  CHECK(argc == 4);
  const unsigned depth = (unsigned)atoi(argv[1]);
  const unsigned chroma = (unsigned)atoi(argv[2]);
  const unsigned mono = (unsigned)atoi(argv[3]);
  CHECK(depth == 8 || depth == 10 || depth == 12);
  CHECK(chroma == 420 || chroma == 422 || chroma == 444);
  aom_codec_enc_cfg_t config;
  CHECK(aom_codec_enc_config_default(aom_codec_av1_cx(), &config, AOM_USAGE_ALL_INTRA) == AOM_CODEC_OK);
  config.g_w = 35;
  config.g_h = 33;
  config.g_limit = 3;
  config.g_lag_in_frames = 0;
  config.g_threads = 2;
  config.g_profile = depth == 12 || chroma == 422 ? 2 : chroma == 444 ? 1 : 0;
  config.g_bit_depth = (aom_bit_depth_t)depth;
  config.g_input_bit_depth = depth;
  config.rc_end_usage = AOM_Q;
  config.monochrome = mono;
  aom_codec_ctx_t encoder, decoder;
  memset(&encoder, 0, sizeof(encoder));
  memset(&decoder, 0, sizeof(decoder));
  const aom_codec_flags_t flags = depth > 8 ? AOM_CODEC_USE_HIGHBITDEPTH : 0;
  CHECK(aom_codec_enc_init(&encoder, aom_codec_av1_cx(), &config, flags) == AOM_CODEC_OK);
  CHECK(aom_codec_control(&encoder, AOME_SET_CPUUSED, 8) == AOM_CODEC_OK);
  CHECK(aom_codec_control(&encoder, AOME_SET_CQ_LEVEL, 28u) == AOM_CODEC_OK);
  const aom_codec_dec_cfg_t decoder_config = { 2, 0, 0, 1 };
  CHECK(aom_codec_dec_init(&decoder, aom_codec_av1_dx(), &decoder_config, 0) == AOM_CODEC_OK);
  aom_img_fmt_t format = chroma == 420 ? AOM_IMG_FMT_I420 : chroma == 422 ? AOM_IMG_FMT_I422 : AOM_IMG_FMT_I444;
  if (depth > 8) format = (aom_img_fmt_t)(format | AOM_IMG_FMT_HIGHBITDEPTH);
  aom_image_t *image = aom_img_alloc(NULL, format, config.g_w, config.g_h, 32);
  CHECK(image != NULL);
  image->monochrome = (int)mono;
  unsigned decoded = 0;
  for (unsigned index = 0; index < 3; ++index) {
    for (unsigned plane = 0; plane < 3; ++plane) {
      const unsigned width = plane ? (image->d_w + image->x_chroma_shift) >> image->x_chroma_shift : image->d_w;
      const unsigned height = plane ? (image->d_h + image->y_chroma_shift) >> image->y_chroma_shift : image->d_h;
      for (unsigned y = 0; y < height; ++y) {
        unsigned char *row = image->planes[plane] + y * image->stride[plane];
        for (unsigned x = 0; x < width; ++x) {
          const unsigned sample = (x * 97 + y * 61 + plane * 173 + index * 211 + x * y * (index + 1)) & ((1u << depth) - 1);
          if (depth > 8) ((uint16_t *)row)[x] = (uint16_t)sample;
          else row[x] = (unsigned char)sample;
        }
      }
    }
    CHECK(aom_codec_encode(&encoder, image, index, 1, 0) == AOM_CODEC_OK);
    CHECK(packets(&encoder, &decoder, &decoded) == 0);
  }
  CHECK(aom_codec_encode(&encoder, NULL, 3, 1, 0) == AOM_CODEC_OK);
  CHECK(packets(&encoder, &decoder, &decoded) == 0);
  CHECK(aom_codec_decode(&decoder, NULL, 0, NULL) == AOM_CODEC_OK);
  CHECK(decoded == 3);
  aom_img_free(image);
  CHECK(aom_codec_destroy(&encoder) == AOM_CODEC_OK);
  CHECK(aom_codec_destroy(&decoder) == AOM_CODEC_OK);
  return 0;
}
