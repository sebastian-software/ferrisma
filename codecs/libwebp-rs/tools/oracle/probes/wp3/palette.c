// WP3 probe: palette analysis of src/utils/palette.c and WebPGetColorPalette
// of src/utils/utils.c (ADR 0002). It reads commands from standard input and
// prints what the library computes:
//
//   palette W H STRIDE OFFSET PIXELS WITH
//       GetColorPalette and WebPGetColorPalette on the ARGB picture whose first
//       row starts at PIXELS[OFFSET]; WITH 0 passes NULL. Prints count=,
//       count2= and, for WITH 1, both palette buffers (palette=, palette2=)
//   prepare PALETTE      PrepareMapToPalette; prints sorted= and map=
//   search SORTED COLOR  SearchColorNoIdx; prints index=
//   sort METHOD W H STRIDE OFFSET PIXELS PALETTE
//       PaletteSort of the sorted PALETTE for the picture; prints sort=0|1 and
//       palette=
//
// Arrays are 32-bit hex values. Output buffers hold MAX_PALETTE_SIZE values,
// start filled with 0xdeadbeef and print whole, so that untouched entries are
// compared too. tests/wp3_palette.rs generates the commands and the expected
// output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/palette.h"
#include "src/utils/utils.h"
#include "src/webp/encode.h"
#include "src/webp/format_constants.h"
#include "wp3_io.h"

#define SENTINEL 0xdeadbeefu

static void Fill(uint32_t* buffer) {
  int i;
  for (i = 0; i < MAX_PALETTE_SIZE; ++i) buffer[i] = SENTINEL;
}

// Reads W H STRIDE OFFSET PIXELS into *pic; returns the pixel buffer.
static uint32_t* ReadPicture(WebPPicture* const pic) {
  size_t count;
  long offset;
  uint32_t* pixels;
  if (!WebPPictureInit(pic)) exit(2);
  pic->use_argb = 1;
  pic->width = (int)Wp3ReadLong();
  pic->height = (int)Wp3ReadLong();
  pic->argb_stride = (int)Wp3ReadLong();
  offset = Wp3ReadLong();
  pixels = Wp3ReadU32s(&count);
  if (offset < 0 || (size_t)offset >= count) exit(2);
  pic->argb = pixels + offset;
  return pixels;
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "palette < commands.txt");
  while ((command = Wp3ReadToken()) != NULL) {
    if (strcmp(command, "palette") == 0) {
      WebPPicture pic;
      uint32_t* pixels = ReadPicture(&pic);
      const int with = (int)Wp3ReadLong();
      uint32_t palette[MAX_PALETTE_SIZE], palette2[MAX_PALETTE_SIZE];
      Fill(palette);
      Fill(palette2);
      printf("count=%d\n", GetColorPalette(&pic, with ? palette : NULL));
      printf("count2=%d\n", WebPGetColorPalette(&pic, with ? palette2 : NULL));
      if (with) {
        Wp3PrintU32s("palette", palette, MAX_PALETTE_SIZE);
        Wp3PrintU32s("palette2", palette2, MAX_PALETTE_SIZE);
      }
      free(pixels);
    } else if (strcmp(command, "prepare") == 0) {
      size_t count;
      uint32_t* palette = Wp3ReadU32s(&count);
      uint32_t sorted[MAX_PALETTE_SIZE], map[MAX_PALETTE_SIZE];
      Fill(sorted);
      Fill(map);
      PrepareMapToPalette(palette, (uint32_t)count, sorted, map);
      Wp3PrintU32s("sorted", sorted, MAX_PALETTE_SIZE);
      Wp3PrintU32s("map", map, MAX_PALETTE_SIZE);
      free(palette);
    } else if (strcmp(command, "search") == 0) {
      size_t count;
      uint32_t* sorted = Wp3ReadU32s(&count);
      const uint32_t color = (uint32_t)Wp3ReadLong();
      printf("index=%d\n", SearchColorNoIdx(sorted, color, (int)count));
      free(sorted);
    } else if (strcmp(command, "sort") == 0) {
      const PaletteSorting method = (PaletteSorting)Wp3ReadLong();
      WebPPicture pic;
      uint32_t* pixels = ReadPicture(&pic);
      size_t count;
      uint32_t* sorted = Wp3ReadU32s(&count);
      uint32_t palette[MAX_PALETTE_SIZE];
      Fill(palette);
      printf("sort=%d\n",
             PaletteSort(method, &pic, sorted, (uint32_t)count, palette));
      Wp3PrintU32s("palette", palette, MAX_PALETTE_SIZE);
      free(sorted);
      free(pixels);
    } else {
      fprintf(stderr, "palette: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
