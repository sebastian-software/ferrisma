// WP3 probe: Huffman table construction of src/utils/huffman_utils.c
// (ADR 0002). It reads commands from standard input and prints what the
// library computes:
//
//   alloc SIZE                 VP8LHuffmanTablesAllocate; prints alloc=0|1 and
//                              the state
//   build ROOT_BITS LENGTHS    VP8LBuildHuffmanTable into the tables; prints
//                              size=N, the N codes at the current table when N
//                              is positive, and the state
//   validate ROOT_BITS LENGTHS VP8LBuildHuffmanTable with a NULL root table;
//                              prints size=N
//   advance N                  curr_segment->curr_table += N, as the decoder
//                              does after a table; prints the state
//   dealloc                    VP8LHuffmanTablesDeallocate; prints the root
//   groups N                   VP8LHtreeGroupsNew; prints groups=0|1
//
// LENGTHS is a byte array, one code length per byte. A code is printed as two
// hex digits of bits and four of value. The state is segments=K, current=I
// (the index of curr_segment in the chain) and one segment=SIZE,CURR,HAS_START
// line per segment, CURR being curr_table - start.
//
// tests/wp3_huffman.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/huffman_utils.h"
#include "wp3_io.h"

static void PrintSegment(const HuffmanTablesSegment* segment) {
  printf("segment=%d,%ld,%d\n", segment->size,
         segment->start == NULL ? 0L : (long)(segment->curr_table - segment->start),
         segment->start != NULL);
}

static void PrintState(const HuffmanTables* tables) {
  const HuffmanTablesSegment* segment;
  int count = 0, current = -1;
  for (segment = &tables->root; segment != NULL; segment = segment->next) {
    if (segment == tables->curr_segment) current = count;
    ++count;
  }
  printf("segments=%d\ncurrent=%d\n", count, current);
  for (segment = &tables->root; segment != NULL; segment = segment->next) {
    PrintSegment(segment);
  }
}

static int* ReadLengths(int* size) {
  size_t count, i;
  uint8_t* bytes = Wp3ReadBytes(&count);
  int* lengths = (int*)calloc(count + 1, sizeof(*lengths));
  if (lengths == NULL) exit(2);
  for (i = 0; i < count; ++i) lengths[i] = bytes[i];
  free(bytes);
  *size = (int)count;
  return lengths;
}

static void PrintCodes(const HuffmanCode* codes, int size) {
  int i;
  printf("codes=");
  for (i = 0; i < size; ++i) printf("%02x%04x", codes[i].bits, codes[i].value);
  printf("\n");
}

int main(int argc, char* argv[]) {
  HuffmanTables tables;
  char* command;
  int allocated = 0;
  ProbeCheckArgs(argc, 0, 0, "huffman < commands.txt");
  memset(&tables, 0, sizeof(tables));
  while ((command = Wp3ReadToken()) != NULL) {
    if (strcmp(command, "alloc") == 0) {
      const int size = (int)Wp3ReadLong();
      if (allocated) VP8LHuffmanTablesDeallocate(&tables);
      printf("alloc=%d\n", VP8LHuffmanTablesAllocate(size, &tables));
      allocated = 1;
      PrintState(&tables);
    } else if (strcmp(command, "build") == 0) {
      const int root_bits = (int)Wp3ReadLong();
      int size;
      int* lengths = ReadLengths(&size);
      const int ret = VP8LBuildHuffmanTable(&tables, root_bits, lengths, size);
      printf("size=%d\n", ret);
      if (ret > 0) PrintCodes(tables.curr_segment->curr_table, ret);
      PrintState(&tables);
      free(lengths);
    } else if (strcmp(command, "validate") == 0) {
      const int root_bits = (int)Wp3ReadLong();
      int size;
      int* lengths = ReadLengths(&size);
      printf("size=%d\n", VP8LBuildHuffmanTable(NULL, root_bits, lengths, size));
      free(lengths);
    } else if (strcmp(command, "advance") == 0) {
      tables.curr_segment->curr_table += Wp3ReadLong();
      PrintState(&tables);
    } else if (strcmp(command, "dealloc") == 0) {
      VP8LHuffmanTablesDeallocate(&tables);
      PrintSegment(&tables.root);
      allocated = 0;
    } else if (strcmp(command, "groups") == 0) {
      HTreeGroup* const groups = VP8LHtreeGroupsNew((int)Wp3ReadLong());
      printf("groups=%d\n", groups != NULL);
      VP8LHtreeGroupsFree(groups);
    } else {
      fprintf(stderr, "huffman: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  if (allocated) VP8LHuffmanTablesDeallocate(&tables);
  return 0;
}
