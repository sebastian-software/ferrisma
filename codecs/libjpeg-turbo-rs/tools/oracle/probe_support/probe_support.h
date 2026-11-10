/* Shared helpers for the oracle probes (WP0).
 *
 * Probes print one `key=value` line per observation: integers in decimal, byte
 * strings in lowercase hex, messages as text up to the end of the line.
 * tests/support/oracle.rs parses that format.
 *
 * Probes are C programs, so they may leave a failing libjpeg call with
 * setjmp()/longjmp() as libjpeg's own example.c does. ProbeErrorMgr records
 * the error code, the formatted message and every warning code before it
 * jumps, so a probe can print the state a failing call leaves behind.
 */

#ifndef LIBJPEG_TURBO_RS_PROBE_SUPPORT_H
#define LIBJPEG_TURBO_RS_PROBE_SUPPORT_H

#include <setjmp.h>
#include <stddef.h>
#include <stdio.h>

#include "jpeglib.h"

#define PROBE_MAX_WARNINGS  64

typedef struct {
  struct jpeg_error_mgr pub;   /* must be first: libjpeg sees only this part */
  jmp_buf setjmp_buffer;       /* where error_exit returns to */
  /* Filled in by error_exit. */
  int error_code;              /* msg_code of the error, 0 when none */
  char error_message[JMSG_LENGTH_MAX];
  /* Filled in by emit_message for every warning (msg_level < 0). */
  int warning_count;           /* warnings seen, may exceed the codes kept */
  int warning_codes[PROBE_MAX_WARNINGS];
  /* The library's emit_message, which still counts num_warnings. */
  void (*std_emit_message) (j_common_ptr cinfo, int msg_level);
} ProbeErrorMgr;

/* Installs jpeg_std_error() defaults with the recording error_exit and
 * emit_message, and returns the manager's public part. Call setjmp() on
 * err->setjmp_buffer before the first libjpeg call that can fail. Messages
 * that the library would print go to stderr, which tests do not compare. */
struct jpeg_error_mgr *ProbeErrorInit(ProbeErrorMgr *err);

/* Prints `error=<code>`, `error_message=<text>` (only when an error
 * occurred), `num_warnings=<n>` and `warnings=<code,code,...>`. */
void ProbePrintErrors(const ProbeErrorMgr *err);

/* Reads the whole file, or standard input when path is NULL or "-". Returns a
 * buffer of at least one byte, never NULL. Exits with status 2 on I/O failure.
 * Release the buffer with free(). */
unsigned char *ProbeReadInput(const char *path, size_t *size);

/* Prints `key=` followed by the bytes in lowercase hex and a newline. */
void ProbePrintHex(const char *key, const unsigned char *data, size_t size);

/* Exits with status 2 and a usage message on stderr unless argc is within
 * [min_args, max_args] (argument count without the program name). */
void ProbeCheckArgs(int argc, int min_args, int max_args, const char *usage);

#endif /* LIBJPEG_TURBO_RS_PROBE_SUPPORT_H */
