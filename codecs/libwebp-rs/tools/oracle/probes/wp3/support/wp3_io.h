// WP3 probe helpers: a token reader for the command scripts that the tests in
// tests/wp3_*.rs generate, and printers for 32-bit arrays (ADR 0002).
//
// Tokens are separated by white space. Byte arrays are hex strings, two digits
// per byte; arrays of 32-bit values are hex strings, eight digits per value,
// most significant digit first. "-" stands for an empty array. Malformed input
// exits with status 2.

#ifndef LIBWEBP_RS_WP3_IO_H_
#define LIBWEBP_RS_WP3_IO_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// The next token, or NULL at the end of the input. Release it with free().
char* Wp3ReadToken(void);

// The next token as a decimal integer.
long Wp3ReadLong(void);

// The next token as a byte array. Stores the byte count in *size and returns a
// zero-filled buffer of at least size + 1 bytes. Release it with free().
uint8_t* Wp3ReadBytes(size_t* size);

// The next token as an array of 32-bit values. Stores the value count in
// *count and returns a zero-filled buffer of at least count + 1 values.
// Release it with free().
uint32_t* Wp3ReadU32s(size_t* count);

// Prints `key=` and the values, eight lowercase hex digits each.
void Wp3PrintU32s(const char* key, const uint32_t* values, size_t count);

#ifdef __cplusplus
}
#endif

#endif  // LIBWEBP_RS_WP3_IO_H_
