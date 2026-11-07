/* WP2 probe support: zeroed system memory, the suspending source, the
 * recording error manager and the dump of a decompression object.
 * See header.c for the output format.
 */

#ifndef LIBJPEG_TURBO_RS_WP2_SUPPORT_H
#define LIBJPEG_TURBO_RS_WP2_SUPPORT_H

#define JPEG_INTERNALS
#include "probe_support.h"

/* The suspending source: releases data in steps; fill_input_buffer suspends
 * while data is unreleased and inserts a fake EOI after JWRN_JPEG_EOF once
 * all of it has been consumed. */
typedef struct {
  struct jpeg_source_mgr pub;
  const JOCTET *data;
  size_t size;
  size_t released;
  long skip_pending;
  boolean fake_eoi;             /* next_input_byte points at the fake EOI */
} SuspendingSource;

void Wp2InstallSuspendingSource(j_decompress_ptr cinfo, SuspendingSource *src,
                                const JOCTET *data, size_t size);
/* Releases `step` more bytes and moves the buffer to the restart point. */
void Wp2Release(SuspendingSource *src, size_t step);

/* The recording error manager: every emit_message call is printed as
 * `msg=<level>:<code>:<message hex>` before the standard emit_message runs;
 * error_exit records the code and message and leaves with longjmp. */
typedef struct {
  struct jpeg_error_mgr pub;
  jmp_buf setjmp_buffer;
  int error_code;
  char error_message[JMSG_LENGTH_MAX];
  void (*std_emit_message) (j_common_ptr cinfo, int msg_level);
} Wp2ErrorMgr;

struct jpeg_error_mgr *Wp2ErrorInit(Wp2ErrorMgr *err);
void Wp2PrintError(const Wp2ErrorMgr *err);

/* The marker processor installed by -processor: counts calls and skips the
 * marker using the public source fields. */
extern int wp2_processor_calls;
boolean Wp2CountingProcessor(j_decompress_ptr cinfo);

/* Prints the round line of a suspension. */
void Wp2PrintRound(j_decompress_ptr cinfo, int round, int status,
                   const SuspendingSource *src);
/* Prints everything jpeg_read_header leaves in the object. */
/* After jpeg_abort (aborted), the component layout is left out. */
void Wp2PrintObject(j_decompress_ptr cinfo, const JOCTET *data, size_t size,
                    const SuspendingSource *src, int aborted);

#endif
