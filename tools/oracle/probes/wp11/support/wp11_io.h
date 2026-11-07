// WP11 probe helpers: a token reader for the command scripts that the tests in
// tests/wp11_*.rs generate, and printers for arrays (ADR 0002).
//
// Tokens are separated by white space. Arrays of 32-bit values are hex
// strings, eight digits per value, most significant digit first; "-" stands
// for an empty array. Malformed input exits with status 2.

#ifndef LIBWEBP_RS_WP11_IO_H_
#define LIBWEBP_RS_WP11_IO_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// The next token, or NULL at the end of the input. Release it with free().
char* Wp11ReadToken(void);

// The next token as a decimal integer.
long Wp11ReadLong(void);

// The next token as an array of 32-bit values. Stores the value count in
// *count and returns a zero-filled buffer of at least count + 1 values.
// Release it with free().
uint32_t* Wp11ReadU32s(size_t* count);

// Prints `key=` and the values, eight lowercase hex digits each, or "-".
void Wp11PrintU32s(const char* key, const uint32_t* values, size_t count);

// Prints `key=` and the values, sixteen lowercase hex digits each, or "-".
void Wp11PrintU64s(const char* key, const uint64_t* values, size_t count);

#ifdef __cplusplus
}
#endif

#endif  // LIBWEBP_RS_WP11_IO_H_
