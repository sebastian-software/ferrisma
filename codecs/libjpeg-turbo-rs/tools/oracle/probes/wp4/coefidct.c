/* WP4 probe: the coefficient buffer controller (jdcoefct.c, jdcoefct.h)
 * driving the inverse DCT (jddctmgr.c and the jidct*.c routines).
 *
 * Usage: coefidct [OPTION]... [FILE]
 * Reads FILE (or standard input), attaches it with jpeg_mem_src, or with the
 * suspending source of wp2_support.c when -suspend is given, and reads the
 * header. It then does what jdmaster.c's master_selection does for these
 * modules: builds sample_range_limit (prepare_range_limit_table), calls
 * jinit_inverse_dct, initializes the entropy decoder, calls
 * jinit_d_coef_controller with use_c_buffer = has_multiple_scans ||
 * buffered_image, allocates a sample array per component (rows
 * v_samp_factor * DCT_scaled_size, width width_in_blocks * DCT_scaled_size,
 * zeroed) as the main controller's buffer, realizes the virtual arrays, calls
 * (*cinfo->inputctl->start_input_pass) and resets the crop columns and
 * last_good_iMCU_row. Output passes run (*cinfo->idct->start_pass),
 * (*cinfo->coef->start_output_pass) and (*cinfo->coef->decompress_data) until
 * output_iMCU_row reaches total_iMCU_rows, as prepare_for_output_pass and the
 * main controller do.
 *
 * Without -buffered, the input is absorbed first if the file has multiple
 * scans and output_scan_number = input_scan_number (jpeg_start_decompress);
 * one output pass follows, then consume_input runs to EOI
 * (jpeg_finish_decompress). With -buffered, buffered_image = TRUE and output
 * passes run as libjpeg.txt's progressive display loop does: while EOI has
 * not been reached, jpeg_start_output(input_scan_number), the pass and
 * jpeg_finish_output; then a final pass if the last one did not show the last
 * scan. When the source suspends, the next step of the data is released.
 * Options, applied after the header:
 *
 *   -trace N        err.trace_level = N (set before the header)
 *   -suspend N      the suspending source, releasing N bytes per step
 *   -dct M          dct_method = M (islow, ifast, float)
 *   -dct2 M         dct_method for the output passes after the first
 *   -scaled N       DCT_scaled_size = N for every component
 *   -scaledc CI N   DCT_scaled_size = N for component CI (repeatable)
 *   -unneeded CI    component_needed = FALSE for component CI (repeatable)
 *   -buffered       buffered-image mode
 *   -nosmooth       do_block_smoothing = FALSE
 *   -crop F L       first_iMCU_col = F, last_iMCU_col = L, and per component
 *                   first_MCU_col = F * hsf, last_MCU_col = (L + 1) * hsf - 1,
 *                   hsf as jpeg_crop_scanline computes it
 *
 * Output: the `msg=` lines of wp2_support.c, `header=<status>`, per output
 * pass `pass=<output_scan_number>,<input_scan_number>,<dct_method>` and
 * `rows=` (an FNV-1a hash of every component's buffer after each completed
 * iMCU row), `consume=` and `decompress=` (every result of consume_input and
 * decompress_data, with runs counted), `final=` with the state, the memory
 * pools (wp2_support.c's Wp2PrintMemory) and the error.
 * tests/wp4_coefidct.rs runs the same sequence on the Rust port.
 */

#include "wp2_support.h"

#include <stdlib.h>
#include <string.h>

#define MAX_CALLS  1000000L

static struct jpeg_decompress_struct cinfo;
static Wp2ErrorMgr err;
static SuspendingSource suspending;
static long suspend;
static long calls;
static int stuck;
static JSAMPARRAY output_buf[MAX_COMPONENTS];
static JDIMENSION buf_rows[MAX_COMPONENTS], buf_cols[MAX_COMPONENTS];

/* A log of return values, with runs of the same value counted. */
typedef struct {
  char text[1 << 18];
  size_t len;
  int last_value;
  long last_count;
} RunLog;

static RunLog consume_log = { "", 0, -1000, 0 };
static RunLog decompress_log = { "", 0, -1000, 0 };

static void FlushLog(RunLog *log)
{
  if (log->last_count == 0)
    return;
  if (log->len + 32 < sizeof(log->text))
    log->len += (size_t)snprintf(log->text + log->len,
                                 sizeof(log->text) - log->len, "%s%d*%ld",
                                 log->len ? "," : "", log->last_value,
                                 log->last_count);
  log->last_count = 0;
}

static void LogValue(RunLog *log, int value)
{
  if (value != log->last_value)
    FlushLog(log);
  log->last_value = value;
  log->last_count++;
}

/* The hashes of the current pass. */
static char rows_text[1 << 20];
static size_t rows_len;

/* jdmaster.c's prepare_range_limit_table, 8-bit branch. */
static void PrepareRangeLimitTable(j_decompress_ptr cinfo)
{
  JSAMPLE *table;
  int i;

  table = (JSAMPLE *)
    (*cinfo->mem->alloc_small) ((j_common_ptr)cinfo, JPOOL_IMAGE,
                (5 * (MAXJSAMPLE + 1) + CENTERJSAMPLE) * sizeof(JSAMPLE));
  table += (MAXJSAMPLE + 1);
  cinfo->sample_range_limit = table;
  memset(table - (MAXJSAMPLE + 1), 0, (MAXJSAMPLE + 1) * sizeof(JSAMPLE));
  for (i = 0; i <= MAXJSAMPLE; i++)
    table[i] = (JSAMPLE)i;
  table += CENTERJSAMPLE;
  for (i = CENTERJSAMPLE; i < 2 * (MAXJSAMPLE + 1); i++)
    table[i] = MAXJSAMPLE;
  memset(table + (2 * (MAXJSAMPLE + 1)), 0,
         (2 * (MAXJSAMPLE + 1) - CENTERJSAMPLE) * sizeof(JSAMPLE));
  memcpy(table + (4 * (MAXJSAMPLE + 1) - CENTERJSAMPLE),
         cinfo->sample_range_limit, CENTERJSAMPLE * sizeof(JSAMPLE));
}

static J_DCT_METHOD ParseMethod(const char *name)
{
  if (!strcmp(name, "ifast"))
    return JDCT_IFAST;
  if (!strcmp(name, "float"))
    return JDCT_FLOAT;
  if (!strcmp(name, "islow"))
    return JDCT_ISLOW;
  return (J_DCT_METHOD)atoi(name);
}

/* Handles a suspension: releases the next step, or reports that the input
 * is stuck.  Returns FALSE when the caller must stop. */
static boolean Resume(void)
{
  if (suspend == 0 || suspending.fake_eoi || ++calls > MAX_CALLS) {
    stuck = 1;
    return FALSE;
  }
  Wp2Release(&suspending, (size_t)suspend);
  return TRUE;
}

static int ConsumeInput(void)
{
  int value = (*cinfo.inputctl->consume_input) (&cinfo);

  LogValue(&consume_log, value);
  return value;
}

static void HashRows(void)
{
  unsigned long long hash = 1469598103934665603ULL;
  int ci;
  JDIMENSION row, col;

  for (ci = 0; ci < cinfo.num_components; ci++)
    for (row = 0; row < buf_rows[ci]; row++)
      for (col = 0; col < buf_cols[ci]; col++)
        hash = (hash ^ output_buf[ci][row][col]) * 1099511628211ULL;
  if (rows_len + 20 < sizeof(rows_text))
    rows_len += (size_t)snprintf(rows_text + rows_len,
                                 sizeof(rows_text) - rows_len, "%s%016llx",
                                 rows_len ? "," : "", hash);
}

/* prepare_for_output_pass and the main controller's calls. */
static void OutputPass(void)
{
  printf("pass=%d,%d,%d\n", cinfo.output_scan_number, cinfo.input_scan_number,
         (int)cinfo.dct_method);
  rows_len = 0;
  rows_text[0] = '\0';
  (*cinfo.idct->start_pass) (&cinfo);
  (*cinfo.coef->start_output_pass) (&cinfo);
  while (cinfo.output_iMCU_row < cinfo.total_iMCU_rows) {
    int status = (*cinfo.coef->decompress_data) (&cinfo, output_buf);

    LogValue(&decompress_log, status);
    if (status == JPEG_SUSPENDED) {
      if (!Resume())
        break;
      continue;
    }
    HashRows();
  }
  printf("rows=%s\n", rows_text);
}

/* jpeg_finish_output's input loop. */
static void FinishOutput(void)
{
  while (cinfo.input_scan_number <= cinfo.output_scan_number &&
         !cinfo.inputctl->eoi_reached) {
    if (ConsumeInput() == JPEG_SUSPENDED && !Resume())
      return;
  }
}

/* Consumes input until EOI, as jpeg_start_decompress and
 * jpeg_finish_decompress do. */
static void ConsumeToEOI(void)
{
  while (!cinfo.inputctl->eoi_reached) {
    if (ConsumeInput() == JPEG_SUSPENDED && !Resume())
      return;
  }
}

static void PrintFinal(void)
{
  FlushLog(&consume_log);
  FlushLog(&decompress_log);
  printf("consume=%s\n", consume_log.text);
  printf("decompress=%s\n", decompress_log.text);
  if (stuck)
    printf("stuck=1\n");
  if (cinfo.master && cinfo.inputctl)
    printf("final=%d,%d,%u,%u,%u,%d\n", cinfo.input_scan_number,
           cinfo.output_scan_number, cinfo.input_iMCU_row,
           cinfo.output_iMCU_row, cinfo.master->last_good_iMCU_row,
           cinfo.inputctl->eoi_reached);
  Wp2PrintMemory(&cinfo);
  Wp2PrintError(&err);
}

int main(int argc, char *argv[])
{
  int scaledc[MAX_COMPONENTS], unneeded[MAX_COMPONENTS];
  int i, ci, trace = 0, scaled = 0, buffered = 0, nosmooth = 0;
  int crop = 0, crop_first = 0, crop_last = 0;
  const char *dct = NULL, *dct2 = NULL;
  const char *path = NULL;
  unsigned char *data;
  size_t size;
  int status;

  for (ci = 0; ci < MAX_COMPONENTS; ci++)
    scaledc[ci] = unneeded[ci] = 0;
  for (i = 1; i < argc; i++) {
    if (!strcmp(argv[i], "-trace") && i + 1 < argc) {
      trace = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-suspend") && i + 1 < argc) {
      suspend = atol(argv[++i]);
    } else if (!strcmp(argv[i], "-dct") && i + 1 < argc) {
      dct = argv[++i];
    } else if (!strcmp(argv[i], "-dct2") && i + 1 < argc) {
      dct2 = argv[++i];
    } else if (!strcmp(argv[i], "-scaled") && i + 1 < argc) {
      scaled = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-scaledc") && i + 2 < argc) {
      ci = atoi(argv[++i]);
      if (ci >= 0 && ci < MAX_COMPONENTS)
        scaledc[ci] = atoi(argv[i + 1]);
      i++;
    } else if (!strcmp(argv[i], "-unneeded") && i + 1 < argc) {
      ci = atoi(argv[++i]);
      if (ci >= 0 && ci < MAX_COMPONENTS)
        unneeded[ci] = 1;
    } else if (!strcmp(argv[i], "-buffered")) {
      buffered = 1;
    } else if (!strcmp(argv[i], "-nosmooth")) {
      nosmooth = 1;
    } else if (!strcmp(argv[i], "-crop") && i + 2 < argc) {
      crop = 1;
      crop_first = atoi(argv[++i]);
      crop_last = atoi(argv[++i]);
    } else if (argv[i][0] == '-' && argv[i][1] != '\0') {
      fprintf(stderr, "unknown option %s\n", argv[i]);
      return 2;
    } else {
      path = argv[i];
    }
  }

  data = ProbeReadInput(path, &size);
  cinfo.err = Wp2ErrorInit(&err);
  if (setjmp(err.setjmp_buffer)) {
    if (rows_len)
      printf("rows=%s\n", rows_text);
    PrintFinal();
    return 0;
  }
  jpeg_create_decompress(&cinfo);
  cinfo.err->trace_level = trace;
  if (suspend > 0)
    Wp2InstallSuspendingSource(&cinfo, &suspending, data, size);
  else
    jpeg_mem_src(&cinfo, data, (unsigned long)size);

  for (;;) {
    if (suspend > 0)
      Wp2Release(&suspending, (size_t)suspend);
    status = jpeg_read_header(&cinfo, TRUE);
    if (status != JPEG_SUSPENDED || suspend == 0)
      break;
  }
  printf("header=%d\n", status);
  if (status != JPEG_HEADER_OK) {
    PrintFinal();
    return 0;
  }

  if (dct)
    cinfo.dct_method = ParseMethod(dct);
  for (ci = 0; ci < cinfo.num_components; ci++) {
    if (scaled > 0)
      cinfo.comp_info[ci].DCT_scaled_size = scaled;
    if (ci < MAX_COMPONENTS && scaledc[ci] != 0)
      cinfo.comp_info[ci].DCT_scaled_size = scaledc[ci];
    if (ci < MAX_COMPONENTS && unneeded[ci])
      cinfo.comp_info[ci].component_needed = FALSE;
  }
  cinfo.buffered_image = buffered ? TRUE : FALSE;
  if (nosmooth)
    cinfo.do_block_smoothing = FALSE;

  /* jdmaster.c's master_selection, for these modules */
  PrepareRangeLimitTable(&cinfo);
  jinit_inverse_dct(&cinfo);
  if (cinfo.arith_code)
    jinit_arith_decoder(&cinfo);
  else if (cinfo.progressive_mode)
    jinit_phuff_decoder(&cinfo);
  else
    jinit_huff_decoder(&cinfo);
  jinit_d_coef_controller(&cinfo, cinfo.inputctl->has_multiple_scans ||
                                  cinfo.buffered_image);
  /* The main controller's buffer */
  for (ci = 0; ci < cinfo.num_components; ci++) {
    jpeg_component_info *compptr = &cinfo.comp_info[ci];
    JDIMENSION row;

    buf_cols[ci] = compptr->width_in_blocks *
                   (JDIMENSION)compptr->DCT_scaled_size;
    buf_rows[ci] = (JDIMENSION)(compptr->v_samp_factor *
                                compptr->DCT_scaled_size);
    output_buf[ci] = (*cinfo.mem->alloc_sarray) ((j_common_ptr)&cinfo,
                                                 JPOOL_IMAGE, buf_cols[ci],
                                                 buf_rows[ci]);
    for (row = 0; row < buf_rows[ci]; row++)
      memset(output_buf[ci][row], 0, buf_cols[ci]);
  }
  (*cinfo.mem->realize_virt_arrays) ((j_common_ptr)&cinfo);
  (*cinfo.inputctl->start_input_pass) (&cinfo);
  cinfo.master->first_iMCU_col = 0;
  cinfo.master->last_iMCU_col = cinfo.MCUs_per_row - 1;
  cinfo.master->last_good_iMCU_row = 0;
  if (crop) {
    cinfo.master->first_iMCU_col = (JDIMENSION)crop_first;
    cinfo.master->last_iMCU_col = (JDIMENSION)crop_last;
    for (ci = 0; ci < cinfo.num_components; ci++) {
      int hsf = (cinfo.comps_in_scan == 1 && cinfo.num_components == 1) ?
                1 : cinfo.comp_info[ci].h_samp_factor;

      cinfo.master->first_MCU_col[ci] = (JDIMENSION)(crop_first * hsf);
      cinfo.master->last_MCU_col[ci] = (JDIMENSION)((crop_last + 1) * hsf - 1);
    }
  }

  if (!buffered) {
    /* jpeg_start_decompress */
    if (cinfo.inputctl->has_multiple_scans)
      ConsumeToEOI();
    cinfo.output_scan_number = cinfo.input_scan_number;
    if (!stuck)
      OutputPass();
    /* jpeg_finish_decompress */
    if (!stuck)
      ConsumeToEOI();
  } else {
    int passes = 0;

    while (!cinfo.inputctl->eoi_reached && !stuck) {
      /* jpeg_start_output(&cinfo, cinfo.input_scan_number) */
      int scan_number = cinfo.input_scan_number;

      if (scan_number <= 0)
        scan_number = 1;
      if (cinfo.inputctl->eoi_reached && scan_number > cinfo.input_scan_number)
        scan_number = cinfo.input_scan_number;
      cinfo.output_scan_number = scan_number;
      if (passes++ > 0 && dct2)
        cinfo.dct_method = ParseMethod(dct2);
      OutputPass();
      if (stuck)
        break;
      FinishOutput();
    }
    if (!stuck && cinfo.output_scan_number != cinfo.input_scan_number) {
      cinfo.output_scan_number = cinfo.input_scan_number;
      if (passes++ > 0 && dct2)
        cinfo.dct_method = ParseMethod(dct2);
      OutputPass();
    }
  }
  PrintFinal();
  return 0;
}
