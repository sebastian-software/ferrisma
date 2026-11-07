// WP6 probe helpers; see wp6_state.h.

#include "wp6_state.h"

#include <stdio.h>

#include "probe_support.h"
#include "wp6_io.h"

void Wp6PrintBitReader(const char* prefix, const VP8BitReader* br,
                       const uint8_t* base) {
  printf("%s=%llu,%u,%d,%d,%d", prefix, (unsigned long long)br->value,
         (unsigned)br->range, br->bits, br->eof, br->buf != NULL);
  if (br->buf != NULL) {
    printf(",%ld,%ld,%ld\n", (long)(br->buf - base), (long)(br->buf_end - base),
           (long)(br->buf_max - base));
  } else {
    printf(",0,0,0\n");
  }
}

void Wp6PrintIo(const VP8Io* io) {
  printf("io=%d,%d,%d,%d,%d,%d,%d,%d,%d\n", io->width, io->height, io->mb_y,
         io->mb_w, io->mb_h, io->y_stride, io->uv_stride,
         io->fancy_upsampling, io->bypass_filtering);
  printf("io_crop=%d,%d,%d,%d,%d\n", io->use_cropping, io->crop_left,
         io->crop_right, io->crop_top, io->crop_bottom);
  printf("io_scale=%d,%d,%d\n", io->use_scaling, io->scaled_width,
         io->scaled_height);
}

void Wp6PrintHeaders(const VP8Decoder* dec, const uint8_t* base) {
  int s, t, b;
  printf("status=%d\n", dec->status);
  printf("error=%s\n", dec->error_msg == NULL ? "(null)" : dec->error_msg);
  printf("ready=%d incremental=%d\n", dec->ready, dec->incremental);
  printf("frm=%d,%d,%d,%u\n", dec->frm_hdr.key_frame, dec->frm_hdr.profile,
         dec->frm_hdr.show, dec->frm_hdr.partition_length);
  printf("pic=%d,%d,%d,%d,%d,%d\n", dec->pic_hdr.width, dec->pic_hdr.height,
         dec->pic_hdr.xscale, dec->pic_hdr.yscale, dec->pic_hdr.colorspace,
         dec->pic_hdr.clamp_type);
  printf("seg=%d,%d,%d", dec->segment_hdr.use_segment,
         dec->segment_hdr.update_map, dec->segment_hdr.absolute_delta);
  for (s = 0; s < NUM_MB_SEGMENTS; ++s) {
    printf(",%d,%d", dec->segment_hdr.quantizer[s],
           dec->segment_hdr.filter_strength[s]);
  }
  printf("\n");
  printf("filter=%d,%d,%d,%d", dec->filter_hdr.simple, dec->filter_hdr.level,
         dec->filter_hdr.sharpness, dec->filter_hdr.use_lf_delta);
  for (s = 0; s < NUM_REF_LF_DELTAS; ++s) {
    printf(",%d", dec->filter_hdr.ref_lf_delta[s]);
  }
  for (s = 0; s < NUM_MODE_LF_DELTAS; ++s) {
    printf(",%d", dec->filter_hdr.mode_lf_delta[s]);
  }
  printf(" type=%d\n", dec->filter_type);
  printf("mb=%d,%d parts=%u\n", dec->mb_w, dec->mb_h,
         dec->num_parts_minus_one);
  for (s = 0; s < NUM_MB_SEGMENTS; ++s) {
    const VP8QuantMatrix* const m = &dec->dqm[s];
    printf("dqm%d=%d,%d,%d,%d,%d,%d,%d,%d\n", s, m->y1_mat[0], m->y1_mat[1],
           m->y2_mat[0], m->y2_mat[1], m->uv_mat[0], m->uv_mat[1],
           m->uv_quant, m->dither);
  }
  printf("segments=%d,%d,%d skip=%d,%d\n", dec->proba.segments[0],
         dec->proba.segments[1], dec->proba.segments[2], dec->use_skip_proba,
         dec->skip_p);
  Wp6PrintDigest("bands", (const uint8_t*)dec->proba.bands,
                 sizeof(dec->proba.bands));
  // The band pointers are set by VP8ParseProba, before `ready`; they are
  // printed only then (NULL before it, index 0 in the Rust translation).
  if (dec->ready) {
    printf("bands_ptr=");
    for (t = 0; t < NUM_TYPES; ++t) {
      for (b = 0; b < 16 + 1; ++b) {
        printf("%ld,", (long)(dec->proba.bands_ptr[t][b] - dec->proba.bands[t]));
      }
    }
    printf("\n");
  }
  Wp6PrintBitReader("br", &dec->br, base);
  for (s = 0; s < MAX_NUM_PARTITIONS; ++s) {
    char name[8];
    snprintf(name, sizeof(name), "part%d", s);
    Wp6PrintBitReader(name, &dec->parts[s], base);
  }
}
