// WP3 probe: Huffman tree building for the encoder,
// src/utils/huffman_encode_utils.c (ADR 0002). It reads commands from standard
// input and prints what the library computes:
//
//   tree LIMIT HISTOGRAM LENGTHS   VP8LCreateHuffmanTree on HISTOGRAM (32-bit
//                                  values, one per symbol) with the code lengths
//                                  starting as LENGTHS; prints histogram=,
//                                  rle=, lengths= and codes= afterwards
//   compress LENGTHS               VP8LCreateCompressedHuffmanTree with
//                                  max_tokens = the symbol count; prints
//                                  tokens=N and token= (code, extra_bits pairs)
//
// Codes print as four hex digits each. The buffers are allocated as the
// encoder allocates them: 3 * num_symbols trees and num_symbols RLE bytes.
//
// tests/wp3_huffman_encode.rs generates the commands and the expected output.

#include <stdio.h>
#include <stdlib.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/huffman_encode_utils.h"
#include "wp3_io.h"

int main(int argc, char* argv[]) {
  char* command;
  ProbeCheckArgs(argc, 0, 0, "huffman_encode < commands.txt");
  while ((command = Wp3ReadToken()) != NULL) {
    if (strcmp(command, "tree") == 0) {
      const int limit = (int)Wp3ReadLong();
      size_t num_symbols, num_lengths, i;
      uint32_t* histogram = Wp3ReadU32s(&num_symbols);
      uint8_t* lengths = Wp3ReadBytes(&num_lengths);
      uint16_t* codes = (uint16_t*)calloc(num_symbols + 1, sizeof(*codes));
      uint8_t* buf_rle = (uint8_t*)malloc(num_symbols + 1);
      HuffmanTree* huff_tree =
          (HuffmanTree*)malloc((3 * num_symbols + 1) * sizeof(*huff_tree));
      HuffmanTreeCode code;
      if (num_lengths != num_symbols || codes == NULL || buf_rle == NULL ||
          huff_tree == NULL) {
        return 2;
      }
      code.num_symbols = (int)num_symbols;
      code.code_lengths = lengths;
      code.codes = codes;
      VP8LCreateHuffmanTree(histogram, limit, buf_rle, huff_tree, &code);
      Wp3PrintU32s("histogram", histogram, num_symbols);
      ProbePrintHex("rle", buf_rle, num_symbols);
      ProbePrintHex("lengths", lengths, num_symbols);
      printf("codes=");
      for (i = 0; i < num_symbols; ++i) printf("%04x", codes[i]);
      printf("\n");
      free(histogram);
      free(lengths);
      free(codes);
      free(buf_rle);
      free(huff_tree);
    } else if (strcmp(command, "compress") == 0) {
      size_t num_symbols;
      uint8_t* lengths = Wp3ReadBytes(&num_symbols);
      HuffmanTreeToken* tokens =
          (HuffmanTreeToken*)malloc((num_symbols + 1) * sizeof(*tokens));
      HuffmanTreeCode code;
      int count, i;
      if (tokens == NULL) return 2;
      code.num_symbols = (int)num_symbols;
      code.code_lengths = lengths;
      code.codes = NULL;
      count = VP8LCreateCompressedHuffmanTree(&code, tokens, (int)num_symbols);
      printf("tokens=%d\ntoken=", count);
      for (i = 0; i < count; ++i) {
        printf("%02x%02x", tokens[i].code, tokens[i].extra_bits);
      }
      printf("\n");
      free(lengths);
      free(tokens);
    } else {
      fprintf(stderr, "huffman_encode: unknown command %s\n", command);
      return 2;
    }
    free(command);
  }
  return 0;
}
