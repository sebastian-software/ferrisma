/* WP2 probe: jpeg_read_header and the rest of the input side (jdapimin.c,
 * jdatasrc.c, jdinput.c, jdmarker.c, jdicc.c, jpeg_abort).
 *
 * Usage: header [OPTION]... [FILE]
 * Reads FILE (or standard input), creates a decompression object and
 * attaches the data with jpeg_mem_src, or with the suspending source of
 * wp2_support.c when -suspend is given. Options, applied in this order:
 *
 *   -create VERSION SIZE  call jpeg_CreateDecompress with these values
 *                         instead of jpeg_create_decompress
 *   -trace N              err.trace_level = N
 *   -save CODE LIMIT      jpeg_save_markers (repeatable)
 *   -processor CODE       jpeg_set_marker_processor with a counting processor
 *                         that skips the marker (repeatable)
 *   -suspend N            the suspending source, releasing N bytes per round;
 *                         jpeg_read_header is repeated while it suspends
 *   -noimage              require_image = FALSE
 *   -again                call jpeg_read_header a second time
 *   -consume N            call jpeg_consume_input N times after the header
 *   -query                call jpeg_input_complete and jpeg_has_multiple_scans
 *   -icc                  call jpeg_read_icc_profile
 *   -resync DESIRED       call jpeg_resync_to_restart
 *   -rst N                call (*cinfo->marker->read_restart_marker) N times
 *   -abort                call jpeg_abort_decompress at the end
 *
 * Every emit_message call prints `msg=<level>:<code>:<hex>`. Each call
 * prints its result (`status=`, `round<k>=` per suspension, and so on), and
 * at the end, also after an error, the probe prints the state of the object
 * (see Wp2PrintObject) and the error. tests/wp2_header.rs runs the same
 * sequences on the Rust port.
 */

#include "wp2_support.h"

#include <stdlib.h>
#include <string.h>

#define MAX_LIST  32

static struct jpeg_decompress_struct cinfo;
static Wp2ErrorMgr err;
static SuspendingSource suspending;

int main(int argc, char *argv[])
{
  int save_code[MAX_LIST], processor_code[MAX_LIST];
  unsigned int save_limit[MAX_LIST];
  int saves = 0, processors = 0, i, trace = 0, again = 0, consume = 0;
  int query = 0, icc = 0, abort_at_end = 0, resync = -1, rst = 0;
  int create = 0, create_version = 0;
  volatile int aborted = 0;
  long create_size = 0, suspend = 0;
  boolean require_image = TRUE;
  const char *path = NULL;
  unsigned char *data;
  size_t size;
  volatile int status = -1;

  for (i = 1; i < argc; i++) {
    if (!strcmp(argv[i], "-create") && i + 2 < argc) {
      create = 1;
      create_version = atoi(argv[++i]);
      create_size = atol(argv[++i]);
    } else if (!strcmp(argv[i], "-trace") && i + 1 < argc) {
      trace = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-save") && i + 2 < argc && saves < MAX_LIST) {
      save_code[saves] = atoi(argv[++i]);
      save_limit[saves++] = (unsigned int)strtoul(argv[++i], NULL, 10);
    } else if (!strcmp(argv[i], "-processor") && i + 1 < argc &&
               processors < MAX_LIST) {
      processor_code[processors++] = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-suspend") && i + 1 < argc) {
      suspend = atol(argv[++i]);
    } else if (!strcmp(argv[i], "-noimage")) {
      require_image = FALSE;
    } else if (!strcmp(argv[i], "-again")) {
      again = 1;
    } else if (!strcmp(argv[i], "-consume") && i + 1 < argc) {
      consume = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-query")) {
      query = 1;
    } else if (!strcmp(argv[i], "-icc")) {
      icc = 1;
    } else if (!strcmp(argv[i], "-resync") && i + 1 < argc) {
      resync = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-rst") && i + 1 < argc) {
      rst = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-abort")) {
      abort_at_end = 1;
    } else if (argv[i][0] != '-' || !strcmp(argv[i], "-")) {
      path = argv[i];
    } else {
      fprintf(stderr, "usage: header [OPTION]... [FILE]\n");
      return 2;
    }
  }
  data = ProbeReadInput(path, &size);
  printf("sizeof_jpeg_decompress_struct=%lu\n",
         (unsigned long)sizeof(struct jpeg_decompress_struct));

  cinfo.err = Wp2ErrorInit(&err);
  if (setjmp(err.setjmp_buffer) == 0) {
    int round;

    if (create)
      jpeg_CreateDecompress(&cinfo, create_version, (size_t)create_size);
    else
      jpeg_create_decompress(&cinfo);
    cinfo.err->trace_level = trace;
    for (i = 0; i < saves; i++)
      jpeg_save_markers(&cinfo, save_code[i], save_limit[i]);
    for (i = 0; i < processors; i++)
      jpeg_set_marker_processor(&cinfo, processor_code[i],
                                Wp2CountingProcessor);
    if (suspend > 0)
      Wp2InstallSuspendingSource(&cinfo, &suspending, data, size);
    else
      jpeg_mem_src(&cinfo, data, (unsigned long)size);

    for (round = 0;; round++) {
      if (suspend > 0)
        Wp2Release(&suspending, (size_t)suspend);
      status = jpeg_read_header(&cinfo, require_image);
      if (status != JPEG_SUSPENDED || suspend <= 0)
        break;
      Wp2PrintRound(&cinfo, round, status, &suspending);
    }
    printf("status=%d\n", status);
    if (again) {
      status = jpeg_read_header(&cinfo, require_image);
      printf("again=%d\n", status);
    }
    for (i = 0; i < consume; i++)
      printf("consume=%d\n", jpeg_consume_input(&cinfo));
    if (query) {
      printf("input_complete=%d\n", jpeg_input_complete(&cinfo));
      printf("multiple_scans=%d\n", jpeg_has_multiple_scans(&cinfo));
    }
    if (icc) {
      JOCTET *icc_data = NULL;
      unsigned int icc_len = 0;
      boolean found = jpeg_read_icc_profile(&cinfo, &icc_data, &icc_len);

      printf("icc=%d,%u\n", found, icc_len);
      if (found)
        ProbePrintHex("icc_data", icc_data, icc_len);
      free(icc_data);
    }
    if (resync >= 0)
      printf("resync=%d\n", jpeg_resync_to_restart(&cinfo, resync));
    for (i = 0; i < rst; i++) {
      boolean result = (*cinfo.marker->read_restart_marker) (&cinfo);

      printf("rst=%d,%d,%d\n", result, cinfo.unread_marker,
             cinfo.marker->next_restart_num);
    }
    if (abort_at_end) {
      jpeg_abort_decompress(&cinfo);
      aborted = 1;
      printf("aborted=1\n");
    }
  }
  printf("processor_calls=%d\n", wp2_processor_calls);
  Wp2PrintObject(&cinfo, data, size, suspend > 0 ? &suspending : NULL,
                 aborted);
  Wp2PrintError(&err);

  jpeg_destroy_decompress(&cinfo);
  free(data);
  return 0;
}
