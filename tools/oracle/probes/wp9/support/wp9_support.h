// Helpers shared by the WP9 probes: hex parsing and printing of planes, with the
// same `key=value` output as the other probes (tools/oracle/probe_support).

#ifndef LIBWEBP_RS_WP9_SUPPORT_H_
#define LIBWEBP_RS_WP9_SUPPORT_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Parses a string of hex byte pairs into a new buffer of at least one byte. The
// string "-" is empty. Exits with status 2 on a malformed string. Release the
// buffer with free().
uint8_t* WP9ParseHex(const char* hex, size_t* size);

// Prints `key=` and the bytes of `buffer` as lowercase hex, then a newline.
void WP9PrintHex(const char* key, const uint8_t* buffer, size_t size);

// Prints `key=` and the bytes of the `width` by `height` samples of a plane whose
// rows are `stride` bytes apart, row by row, without the padding.
void WP9PrintPlane(const char* key, const uint8_t* plane, int width, int height,
                   long stride);

// Prints `key=` and the little-endian bytes of the `width` by `height` words of an
// ARGB plane whose rows are `stride` words apart, without the padding.
void WP9PrintArgb(const char* key, const uint32_t* argb, int width, int height,
                  long stride);

// Parses a decimal integer, exiting with status 2 on a malformed value.
long WP9ParseInt(const char* text);

// Parses a float given as the hex of its IEEE 754 bits (8 digits).
float WP9ParseFloatBits(const char* text);

#ifdef __cplusplus
}
#endif

#endif  // LIBWEBP_RS_WP9_SUPPORT_H_
