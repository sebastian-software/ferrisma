// Shared helpers for the oracle probes (WP0). Probes print one `key=value`
// line per observation: integers in decimal, byte strings in lowercase hex.
// tests/support/oracle.rs parses that format.

#ifndef LIBWEBP_RS_PROBE_SUPPORT_H_
#define LIBWEBP_RS_PROBE_SUPPORT_H_

#include <stddef.h>
#include <stdint.h>

#ifdef __cplusplus
extern "C" {
#endif

// Reads the whole file, or standard input when path is NULL or "-". Returns a
// buffer of at least one byte, never NULL, so an empty input still reaches the
// library with a valid pointer, as a Rust slice would. Exits with status 2 on
// I/O failure. Release the buffer with free().
uint8_t* ProbeReadInput(const char* path, size_t* size);

// Prints `key=` followed by the bytes in lowercase hex and a newline.
void ProbePrintHex(const char* key, const uint8_t* data, size_t size);

// Exits with status 2 and a usage message on stderr unless argc is within
// [min_args, max_args] (argument count without the program name).
void ProbeCheckArgs(int argc, int min_args, int max_args, const char* usage);

#ifdef __cplusplus
}
#endif

#endif  // LIBWEBP_RS_PROBE_SUPPORT_H_
