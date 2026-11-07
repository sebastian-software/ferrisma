// WP7 probe helpers that print the state of the lossless decoder and of a
// VP8Io in the key=value format that tests/wp7_*.rs reproduces from the Rust
// translation (ADR 0002).

#ifndef LIBWEBP_RS_WP7_STATE_H_
#define LIBWEBP_RS_WP7_STATE_H_

#include "src/dec/vp8_dec.h"
#include "src/dec/vp8li_dec.h"
#include "src/utils/huffman_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

// Prints the bit reader: val, len, pos, bit_pos and eos.
void Wp7PrintBitReader(const char* prefix, const VP8LBitReader* br);

// Prints the segment chain of the tables and a digest of the codes of each
// segment up to its current table.
void Wp7PrintTables(const HuffmanTables* tables);

// Prints `count` tree groups, their trees as segment and offset in `tables`,
// and a digest of each packed table in use.
void Wp7PrintGroups(const HTreeGroup* groups, int count,
                    const HuffmanTables* tables);

// Prints the scalar fields of the decoder and its bit reader.
void Wp7PrintDecoder(const VP8LDecoder* dec);

// Prints Wp7PrintDecoder, the transforms, the metadata, the tree groups and
// the tables.
void Wp7PrintDecoderFull(const VP8LDecoder* dec);

// Prints the fields of a VP8Io that the Rust VP8Io keeps.
void Wp7PrintIo(const VP8Io* io);

#ifdef __cplusplus
}
#endif

#endif  // LIBWEBP_RS_WP7_STATE_H_
