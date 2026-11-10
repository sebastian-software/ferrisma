// WP7 probe helpers; see wp7_state.h.

#include "wp7_state.h"

#include <stdio.h>

#include "src/dsp/lossless_common.h"
#include "wp7_io.h"

void Wp7PrintBitReader(const char* prefix, const VP8LBitReader* br) {
  printf("%s_val=%llu\n", prefix, (unsigned long long)br->val);
  printf("%s_len=%zu\n", prefix, br->len);
  printf("%s_pos=%zu\n", prefix, br->pos);
  printf("%s_bit_pos=%d\n", prefix, br->bit_pos);
  printf("%s_eos=%d\n", prefix, br->eos);
}

// Digest of codes as bits, value low byte, value high byte.
static void PrintCodes(const char* key, const HuffmanCode* codes, size_t n) {
  uint64_t hash = 0xcbf29ce484222325ULL;
  size_t i;
  for (i = 0; i < n; ++i) {
    const uint8_t bytes[3] = {codes[i].bits, (uint8_t)(codes[i].value & 0xff),
                              (uint8_t)(codes[i].value >> 8)};
    int k;
    for (k = 0; k < 3; ++k) {
      hash ^= bytes[k];
      hash *= 0x100000001b3ULL;
    }
  }
  printf("%s=%016llx\n", key, (unsigned long long)hash);
}

void Wp7PrintTables(const HuffmanTables* tables) {
  const HuffmanTablesSegment* segment;
  int count = 0, current = -1;
  for (segment = &tables->root; segment != NULL; segment = segment->next) {
    if (segment == tables->curr_segment) current = count;
    ++count;
  }
  printf("segments=%d\n", count);
  // A zeroed HuffmanTables has a NULL curr_segment, which the Rust index
  // cannot express; the current segment is printed while the tables are
  // allocated.
  if (tables->root.start != NULL) printf("current=%d\n", current);
  for (segment = &tables->root; segment != NULL; segment = segment->next) {
    const long used = (segment->start == NULL)
                          ? 0L
                          : (long)(segment->curr_table - segment->start);
    printf("segment=%d,%ld,%d\n", segment->size, used, segment->start != NULL);
    if (segment->start != NULL) {
      PrintCodes("codes", segment->start, (size_t)used);
    }
  }
}

// The segment index and offset of a code pointer, or -1,-1.
static void PrintTree(const HuffmanCode* tree, const HuffmanTables* tables) {
  const HuffmanTablesSegment* segment;
  int index = 0;
  for (segment = &tables->root; segment != NULL; segment = segment->next) {
    if (segment->start != NULL && tree >= segment->start &&
        tree < segment->start + segment->size) {
      printf("tree=%d,%ld\n", index, (long)(tree - segment->start));
      return;
    }
    ++index;
  }
  printf("tree=-1,-1\n");
}

void Wp7PrintGroups(const HTreeGroup* groups, int count,
                    const HuffmanTables* tables) {
  int i, j;
  printf("groups=%d\n", groups == NULL ? -1 : count);
  if (groups == NULL) return;
  for (i = 0; i < count; ++i) {
    const HTreeGroup* const group = &groups[i];
    for (j = 0; j < HUFFMAN_CODES_PER_META_CODE; ++j) {
      PrintTree(group->htrees[j], tables);
    }
    printf("trivial_literal=%d\n", group->is_trivial_literal);
    // C sets literal_arb only for a trivial literal; the allocation of the
    // groups leaves it uninitialized otherwise.
    if (group->is_trivial_literal) {
      printf("literal_arb=%u\n", group->literal_arb);
    }
    printf("trivial_code=%d\n", group->is_trivial_code);
    printf("packed=%d\n", group->use_packed_table);
    if (group->use_packed_table) {
      uint64_t hash = 0xcbf29ce484222325ULL;
      int k, b;
      for (k = 0; k < HUFFMAN_PACKED_TABLE_SIZE; ++k) {
        const uint32_t words[2] = {(uint32_t)group->packed_table[k].bits,
                                   group->packed_table[k].value};
        for (b = 0; b < 8; ++b) {
          hash ^= (uint8_t)(words[b >> 2] >> (8 * (b & 3)));
          hash *= 0x100000001b3ULL;
        }
      }
      printf("packed_table=%016llx\n", (unsigned long long)hash);
    }
  }
}

void Wp7PrintDecoder(const VP8LDecoder* dec) {
  printf("status=%d\n", dec->status);
  printf("state=%d\n", dec->state);
  printf("dec_width=%d\n", dec->width);
  printf("dec_height=%d\n", dec->height);
  printf("last_row=%d\n", dec->last_row);
  printf("last_pixel=%d\n", dec->last_pixel);
  printf("last_out_row=%d\n", dec->last_out_row);
  printf("incremental=%d\n", dec->incremental);
  printf("next_transform=%d\n", dec->next_transform);
  printf("transforms_seen=%u\n", dec->transforms_seen);
  printf("has_pixels=%d\n", dec->pixels != NULL);
  printf("has_rescaler=%d\n", dec->rescaler_memory != NULL);
  Wp7PrintBitReader("br", &dec->br);
}

// Digest of 32-bit values in little-endian byte order.
static void PrintWords(const char* key, const uint32_t* words, size_t n) {
  uint64_t hash = 0xcbf29ce484222325ULL;
  size_t i;
  int b;
  for (i = 0; i < n; ++i) {
    for (b = 0; b < 4; ++b) {
      hash ^= (uint8_t)(words[i] >> (8 * b));
      hash *= 0x100000001b3ULL;
    }
  }
  printf("%s=%016llx\n", key, (unsigned long long)hash);
}

void Wp7PrintDecoderFull(const VP8LDecoder* dec) {
  const VP8LMetadata* const hdr = &dec->hdr;
  int i;
  Wp7PrintDecoder(dec);
  for (i = 0; i < dec->next_transform; ++i) {
    const VP8LTransform* const transform = &dec->transforms[i];
    size_t n = 0;
    printf("transform=%d,%d,%d,%d\n", transform->type, transform->bits,
           transform->xsize, transform->ysize);
    if (transform->type == PREDICTOR_TRANSFORM ||
        transform->type == CROSS_COLOR_TRANSFORM) {
      n = (size_t)VP8LSubSampleSize(transform->xsize, transform->bits) *
          VP8LSubSampleSize(transform->ysize, transform->bits);
    } else if (transform->type == COLOR_INDEXING_TRANSFORM) {
      n = (size_t)1 << (8 >> transform->bits);
    }
    if (transform->data == NULL) n = 0;
    printf("transform_size=%zu\n", n);
    PrintWords("transform_data", transform->data, n);
  }
  printf("color_cache_size=%d\n", hdr->color_cache_size);
  printf("cache_bits=%d\n", hdr->color_cache.hash_bits);
  printf("cache_shift=%d\n", hdr->color_cache.hash_shift);
  if (hdr->color_cache.colors != NULL) {
    PrintWords("cache", hdr->color_cache.colors,
               (size_t)1 << hdr->color_cache.hash_bits);
  }
  printf("saved_cache=%d\n", hdr->saved_color_cache.colors != NULL);
  printf("huffman_mask=%d\n", hdr->huffman_mask);
  printf("huffman_bits=%d\n", hdr->huffman_subsample_bits);
  printf("huffman_xsize=%d\n", hdr->huffman_xsize);
  if (hdr->huffman_image != NULL && hdr->huffman_subsample_bits > 0) {
    const size_t n =
        (size_t)hdr->huffman_xsize *
        VP8LSubSampleSize(dec->height, hdr->huffman_subsample_bits);
    PrintWords("huffman_image", hdr->huffman_image, n);
  } else {
    printf("huffman_image=%d\n", hdr->huffman_image != NULL);
  }
  printf("num_htree_groups=%d\n", hdr->num_htree_groups);
  Wp7PrintGroups(hdr->htree_groups, hdr->num_htree_groups,
                 &hdr->huffman_tables);
  Wp7PrintTables(&hdr->huffman_tables);
}

void Wp7PrintIo(const VP8Io* io) {
  printf("io=%d,%d,%d,%d,%d,%d,%d,%d,%d\n", io->width, io->height, io->mb_y,
         io->mb_w, io->mb_h, io->y_stride, io->uv_stride, io->fancy_upsampling,
         io->bypass_filtering);
  printf("io_crop=%d,%d,%d,%d,%d\n", io->use_cropping, io->crop_left,
         io->crop_right, io->crop_top, io->crop_bottom);
  printf("io_scale=%d,%d,%d\n", io->use_scaling, io->scaled_width,
         io->scaled_height);
}
