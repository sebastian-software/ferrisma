// WP6 probe helpers: a token reader for the command scripts that the tests in
// tests/wp6_*.rs generate, the input that the commands decode, and the digest
// that the probes print for large buffers (ADR 0002).
//
// Tokens are separated by white space. Byte arrays are hex strings, two digits
// per byte; "-" stands for an empty array. Lists of integers are separated by
// commas; "-" stands for an empty list. Malformed input exits with status 2.

#ifndef LIBWEBP_RS_WP6_IO_H_
#define LIBWEBP_RS_WP6_IO_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// The next token, or NULL at the end of the input. Release it with free().
char* Wp6ReadToken(void);

// The next token as a decimal integer.
long Wp6ReadLong(void);

// The next token as a byte array. Stores the byte count in *size and returns a
// zero-filled buffer of at least size + 1 bytes. Release it with free().
uint8_t* Wp6ReadBytes(size_t* size);

// The next token as a comma-separated list of decimal integers. Stores the
// count in *count and returns a buffer of at least count + 1 values. Release it
// with free().
long* Wp6ReadList(size_t* count);

// Reads `size` bytes at `offset` of the file `path`. Returns a buffer of at
// least size + 1 bytes. Exits with status 2 on I/O failure. Release it with
// free().
uint8_t* Wp6ReadFile(const char* path, long offset, long size);

// The 64-bit FNV-1a digest of the bytes, which tests/wp6_*.rs computes the same
// way for buffers too large to print.
uint64_t Wp6Digest(const uint8_t* data, size_t size);

// Prints `key=` and the digest in 16 lowercase hex digits.
void Wp6PrintDigest(const char* key, const uint8_t* data, size_t size);

#ifdef __cplusplus
}
#endif

#endif  // LIBWEBP_RS_WP6_IO_H_
