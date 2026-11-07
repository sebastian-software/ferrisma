// WP7 probe: ReadHuffmanCodesHelper of src/dec/vp8l_dec.c, which reads the
// Huffman codes of the tree groups, as tests/fuzzer/huffman_fuzzer.cc calls it
// (ADR 0002). Commands on standard input:
//
//   helper CACHE_BITS NUM NUM_MAX MAPPING HEX
//                     reads the codes from the bytes with a new decoder;
//                     MAPPING is a comma-separated list of NUM_MAX group
//                     indexes, or "-" for NULL. Prints ok=0|1, the decoder,
//                     the tree groups and the tables.
//
// tests/wp7_huffman_codes.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "src/dec/vp8li_dec.h"
#include "src/utils/bit_reader_utils.h"
#include "src/utils/huffman_utils.h"
#include "probe_support.h"
#include "wp7_io.h"
#include "wp7_state.h"

static void Helper(void) {
  const int color_cache_bits = (int)Wp7ReadLong();
  const int num_htree_groups = (int)Wp7ReadLong();
  const int num_htree_groups_max = (int)Wp7ReadLong();
  size_t num_mapping, size, i;
  long* const list = Wp7ReadList(&num_mapping);
  uint8_t* const data = Wp7ReadBytes(&size);
  int* mapping = NULL;
  HuffmanTables tables;
  HTreeGroup* groups = NULL;
  VP8LDecoder* const dec = VP8LNew();
  int ok;

  if (dec == NULL) exit(2);
  if (num_mapping > 0) {
    if (num_mapping != (size_t)num_htree_groups_max) exit(2);
    mapping = (int*)malloc(num_mapping * sizeof(*mapping));
    if (mapping == NULL) exit(2);
    for (i = 0; i < num_mapping; ++i) mapping[i] = (int)list[i];
  }
  memset(&tables, 0, sizeof(tables));
  VP8LInitBitReader(&dec->br, data, size);
  ok = ReadHuffmanCodesHelper(color_cache_bits, num_htree_groups,
                              num_htree_groups_max, mapping, dec, &tables,
                              &groups);
  printf("ok=%d\n", ok);
  Wp7PrintDecoder(dec);
  Wp7PrintGroups(groups, num_htree_groups, &tables);
  Wp7PrintTables(&tables);

  VP8LHtreeGroupsFree(groups);
  VP8LHuffmanTablesDeallocate(&tables);
  VP8LDelete(dec);
  free(mapping);
  free(list);
  free(data);
}

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "huffman_codes < commands");
  (void)argv;
  while ((command = Wp7ReadToken()) != NULL) {
    if (strcmp(command, "helper") == 0) {
      Helper();
    } else {
      fprintf(stderr, "huffman_codes: unknown command %s\n", command);
      return 2;
    }
    free(command);
    fflush(stdout);
  }
  return 0;
}
