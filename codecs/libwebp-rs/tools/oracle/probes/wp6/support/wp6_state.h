// WP6 probe helpers: printers of the VP8 decoder state that the tests in
// tests/wp6_*.rs reproduce from the Rust translation (ADR 0002). Pointers into
// the input are printed as offsets from `base`, the start of the VP8 data.

#ifndef LIBWEBP_RS_WP6_STATE_H_
#define LIBWEBP_RS_WP6_STATE_H_

#include <stdint.h>

#include "src/dec/vp8_dec.h"
#include "src/dec/vp8i_dec.h"
#include "src/utils/bit_reader_utils.h"

#ifdef __cplusplus
extern "C" {
#endif

// Prints a boolean decoder under `prefix`.
void Wp6PrintBitReader(const char* prefix, const VP8BitReader* br,
                       const uint8_t* base);

// Prints the scalar fields of a VP8Io.
void Wp6PrintIo(const VP8Io* io);

// Prints the status, the headers, the quantizers, the probabilities, the bit
// readers and the scalar decoding fields of a decoder.
void Wp6PrintHeaders(const VP8Decoder* dec, const uint8_t* base);

#ifdef __cplusplus
}
#endif

#endif  // LIBWEBP_RS_WP6_STATE_H_
