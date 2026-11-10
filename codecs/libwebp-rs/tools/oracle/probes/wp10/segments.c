// WP10 probe: VP8SetSegmentParams (src/enc/quant_enc.c), which sets the
// quantizers of the segments, their matrices and lambdas, the filter
// strengths, and merges equivalent segments (ADR 0002).
//
// Standard input is a sequence of records of little-endian int32 values:
//
//   mb_w mb_h method num_segments quality_bits sns_strength emulate_jpeg_size
//   filter_strength filter_type filter_sharpness hdr_sharpness alpha uv_alpha
//   dqm_alpha[4] dqm_beta[4], then mb_w * mb_h segment numbers
//
// The probe fills a zeroed VP8Encoder with these fields, calls
// VP8SetSegmentParams, and prints `encoder=` (the encoder fields it writes),
// `dqm=` (each segment's fields and matrices) and `segments=` (the segment
// of each macroblock), all comma-separated decimals.
// tests/wp10_segments.rs builds the records and the expected lines.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/enc/vp8i_enc.h"
#include "src/webp/encode.h"

static int ReadI32(int32_t* value) {
  uint8_t b[4];
  if (fread(b, 1, 4, stdin) != 4) return 0;
  *value = (int32_t)((uint32_t)b[0] | ((uint32_t)b[1] << 8) |
                     ((uint32_t)b[2] << 16) | ((uint32_t)b[3] << 24));
  return 1;
}

static int32_t MustReadI32(void) {
  int32_t value;
  if (!ReadI32(&value)) exit(2);
  return value;
}

static void PrintMatrix(const VP8Matrix* const m) {
  int i;
  for (i = 0; i < 16; ++i) {
    printf(",%u,%u,%u,%u,%u", m->q[i], m->iq[i], m->bias[i], m->zthresh[i],
           m->sharpen[i]);
  }
}

int main(int argc, char* argv[]) {
  int32_t mb_w;
  ProbeCheckArgs(argc, 0, 0, "segments < records.bin");
  (void)argv;
  while (ReadI32(&mb_w)) {
    const int32_t mb_h = MustReadI32();
    VP8Encoder* const enc = (VP8Encoder*)calloc(1, sizeof(*enc));
    WebPConfig config;
    float quality;
    int32_t quality_bits;
    int i;
    if (enc == NULL || !WebPConfigInit(&config)) return 2;
    enc->mb_w = mb_w;
    enc->mb_h = mb_h;
    enc->method = MustReadI32();
    enc->segment_hdr.num_segments = MustReadI32();
    quality_bits = MustReadI32();
    memcpy(&quality, &quality_bits, 4);
    config.sns_strength = MustReadI32();
    config.emulate_jpeg_size = MustReadI32();
    config.filter_strength = MustReadI32();
    config.filter_type = MustReadI32();
    config.filter_sharpness = MustReadI32();
    enc->filter_hdr.sharpness = MustReadI32();
    enc->alpha = MustReadI32();
    enc->uv_alpha = MustReadI32();
    for (i = 0; i < NUM_MB_SEGMENTS; ++i) enc->dqm[i].alpha = MustReadI32();
    for (i = 0; i < NUM_MB_SEGMENTS; ++i) enc->dqm[i].beta = MustReadI32();
    enc->mb_info = (VP8MBInfo*)calloc((size_t)(mb_w * mb_h), sizeof(VP8MBInfo));
    if (enc->mb_info == NULL) return 2;
    for (i = 0; i < mb_w * mb_h; ++i) enc->mb_info[i].segment = MustReadI32();
    enc->config = &config;

    VP8SetSegmentParams(enc, quality);

    printf("encoder=%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d\n", enc->base_quant,
           enc->dq_y1_dc, enc->dq_y2_dc, enc->dq_y2_ac, enc->dq_uv_dc,
           enc->dq_uv_ac, enc->segment_hdr.num_segments, enc->filter_hdr.level,
           enc->filter_hdr.simple, enc->filter_hdr.sharpness,
           enc->filter_hdr.i4x4_lf_delta);
    printf("dqm=");
    for (i = 0; i < NUM_MB_SEGMENTS; ++i) {
      const VP8SegmentInfo* const m = &enc->dqm[i];
      printf("%s%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%lld", i ? ";" : "",
             m->alpha, m->beta, m->quant, m->fstrength, m->max_edge,
             m->min_disto, m->lambda_i16, m->lambda_i4, m->lambda_uv,
             m->lambda_mode, m->lambda_trellis, m->tlambda,
             m->lambda_trellis_i16, m->lambda_trellis_i4, m->lambda_trellis_uv,
             0, (long long)m->i4_penalty);
      PrintMatrix(&m->y1);
      PrintMatrix(&m->y2);
      PrintMatrix(&m->uv);
    }
    printf("\nsegments=");
    for (i = 0; i < mb_w * mb_h; ++i) {
      printf("%s%u", i ? "," : "", enc->mb_info[i].segment);
    }
    printf("\n");
    fflush(stdout);
    free(enc->mb_info);
    free(enc);
  }
  return 0;
}
