// WP6 probe: the VP8 frame header parsing of src/dec/vp8_dec.c (VP8GetHeaders
// with ParseSegmentHeader, ParseFilterHeader, ParsePartitions, VP8ParseQuant of
// quant_dec.c and VP8ParseProba of tree_dec.c), VP8GetInfo and
// VP8CheckSignature (ADR 0002). Commands on standard input:
//
//   headers DATA INCREMENTAL     VP8GetHeaders on DATA (hex) as io->data
//   file PATH OFFSET SIZE INCREMENTAL
//                                the same on SIZE bytes of a file
//   info DATA CHUNK_SIZE         VP8GetInfo
//   signature DATA               VP8CheckSignature
//
// After `headers` and `file` the probe prints the result, the decoder state
// (wp6_state.c) and the io. tests/wp6_headers.rs generates the commands and the
// expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8_dec.h"
#include "src/dec/vp8i_dec.h"
#include "probe_support.h"
#include "wp6_io.h"
#include "wp6_state.h"

static void Headers(const uint8_t* data, size_t size, int incremental) {
  VP8Decoder* const dec = VP8New();
  VP8Io io;
  int ok;
  if (dec == NULL || !VP8InitIo(&io)) exit(2);
  dec->incremental = incremental;
  io.data = data;
  io.data_size = size;
  ok = VP8GetHeaders(dec, &io);
  printf("ok=%d\n", ok);
  Wp6PrintHeaders(dec, data);
  Wp6PrintIo(&io);
  VP8Delete(dec);
}

int main(int argc, char* argv[]) {
  char* command;
  (void)argv;
  ProbeCheckArgs(argc, 0, 0, "headers < commands");
  while ((command = Wp6ReadToken()) != NULL) {
    if (!strcmp(command, "headers")) {
      size_t size;
      uint8_t* const data = Wp6ReadBytes(&size);
      const int incremental = (int)Wp6ReadLong();
      Headers(data, size, incremental);
      free(data);
    } else if (!strcmp(command, "file")) {
      char* const path = Wp6ReadToken();
      const long offset = Wp6ReadLong();
      const long size = Wp6ReadLong();
      const int incremental = (int)Wp6ReadLong();
      uint8_t* data;
      if (path == NULL) exit(2);
      data = Wp6ReadFile(path, offset, size);
      Headers(data, (size_t)size, incremental);
      free(data);
      free(path);
    } else if (!strcmp(command, "info")) {
      size_t size;
      uint8_t* const data = Wp6ReadBytes(&size);
      const long chunk_size = Wp6ReadLong();
      int width = -1, height = -1;
      const int ok = VP8GetInfo(data, size, (size_t)chunk_size, &width, &height);
      printf("info=%d,%d,%d\n", ok, width, height);
      free(data);
    } else if (!strcmp(command, "signature")) {
      size_t size;
      uint8_t* const data = Wp6ReadBytes(&size);
      printf("signature=%d\n", VP8CheckSignature(data, size));
      free(data);
    } else {
      fprintf(stderr, "wp6: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
