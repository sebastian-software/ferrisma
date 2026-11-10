// WP7 probe: VP8LCheckSignature and VP8LGetInfo of src/dec/vp8l_dec.c
// (ADR 0002). Commands on standard input:
//
//   info HEX SIZE     calls both functions on the first SIZE bytes; prints
//                     signature=0|1, info=0|1 and, on success, the width, the
//                     height and the alpha flag
//
// tests/wp7_info.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8_dec.h"
#include "probe_support.h"
#include "wp7_io.h"

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "info < commands");
  (void)argv;
  while ((command = Wp7ReadToken()) != NULL) {
    if (strcmp(command, "info") == 0) {
      size_t length;
      uint8_t* const data = Wp7ReadBytes(&length);
      const long size = Wp7ReadLong();
      int width = -1, height = -1, has_alpha = -1, ok;
      if (size < 0 || (size_t)size > length) exit(2);
      printf("signature=%d\n", VP8LCheckSignature(data, (size_t)size));
      ok = VP8LGetInfo(data, (size_t)size, &width, &height, &has_alpha);
      printf("info=%d\n", ok);
      printf("width=%d\nheight=%d\nhas_alpha=%d\n", width, height, has_alpha);
      free(data);
    } else {
      fprintf(stderr, "info: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
