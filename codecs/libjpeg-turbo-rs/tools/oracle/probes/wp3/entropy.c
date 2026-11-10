/* WP3 probe: entropy decoding (jdhuff.c, jdphuff.c, jdarith.c, jaricom.c,
 * jstdhuff.c).
 *
 * Usage: entropy [OPTION]... [FILE]
 * Reads FILE (or standard input), attaches it with jpeg_mem_src, or with the
 * suspending source of wp2_support.c when -suspend is given, and reads the
 * header. It then initializes the entropy decoder that jdmaster.c's
 * master_selection selects, installs a coefficient controller whose
 * start_input_pass and consume_data mirror the multi-scan ones of jdcoefct.c
 * and jdcoefct.h (every block of the image is kept), calls
 * (*cinfo->inputctl->start_input_pass), and calls
 * (*cinfo->inputctl->consume_input) until it returns JPEG_REACHED_EOI, as
 * jpeg_start_decompress and jpeg_consume_input do in buffered-image mode.
 * When the source suspends, the next step of the data is released.
 * Options, applied after the header in this order:
 *
 *   -trace N        err.trace_level = N (set before the header)
 *   -suspend N      the suspending source, releasing N bytes per step
 *   -scaled N       DCT_scaled_size = N for every component, as jdmaster.c's
 *                   scaling sets it (1 means the AC coefficients are skipped)
 *   -unneeded CI    component_needed = FALSE for component CI (repeatable)
 *   -nullrows N     pass MCU_data = NULL for the first N iMCU rows of every
 *                   scan, as jdcoefct.c does when it skips rows (sequential
 *                   decoders only)
 *
 * Output: the `msg=` lines of wp2_support.c, `header=<status>`,
 * `decoder=<huff|phuff|arith>`, and after every scan `scan=` with the
 * state (see PrintState) and, for progressive files, `coef_bits=`. At the
 * end `consume=` (every return value of consume_input, with runs of
 * JPEG_SUSPENDED counted), `final=`, the memory pools (wp2_support.c's
 * Wp2PrintMemory, which shows the C size of every allocation), `rows<ci>=`
 * (an FNV-1a hash of every block row of component ci), and the error. tests/wp3_entropy.rs runs the
 * same sequence on the Rust port.
 */

#include "wp2_support.h"

#include <stdlib.h>
#include <string.h>

#define MAX_UNNEEDED  MAX_COMPONENTS
#define MAX_CALLS     1000000L

typedef struct {
  struct jpeg_d_coef_controller pub;
  JDIMENSION MCU_ctr;
  int MCU_vert_offset;
  int MCU_rows_per_iMCU_row;
  JBLOCKROW MCU_buffer[D_MAX_BLOCKS_IN_MCU];
  /* The whole image: rows of blocks per component, sized as jdcoefct.c's
   * virtual arrays (rounded up to the sampling factors). */
  JBLOCKARRAY whole_image[MAX_COMPONENTS];
  JDIMENSION rows[MAX_COMPONENTS], cols[MAX_COMPONENTS];
  int null_rows;
} FakeCoef;

static struct jpeg_decompress_struct cinfo;
static Wp2ErrorMgr err;
static SuspendingSource suspending;
static FakeCoef coef;
static const JOCTET *input_data;
static size_t input_size;

/* The consume_input results: every value, with runs of the same value
 * counted. */
static char consume_log[1 << 20];
static size_t consume_len;
static int last_value = -1000;
static long last_count;

static void FlushValue(void)
{
  if (last_count == 0)
    return;
  if (consume_len + 32 < sizeof(consume_log))
    consume_len += (size_t)snprintf(consume_log + consume_len,
                                    sizeof(consume_log) - consume_len,
                                    "%s%d*%ld", consume_len ? "," : "",
                                    last_value, last_count);
  last_count = 0;
}

static void LogValue(int value)
{
  if (value != last_value)
    FlushValue();
  last_value = value;
  last_count++;
}

/* jdcoefct.h's start_iMCU_row. */
static void StartIMCURow(j_decompress_ptr cinfo)
{
  if (cinfo->comps_in_scan > 1) {
    coef.MCU_rows_per_iMCU_row = 1;
  } else {
    if (cinfo->input_iMCU_row < (cinfo->total_iMCU_rows - 1))
      coef.MCU_rows_per_iMCU_row = cinfo->cur_comp_info[0]->v_samp_factor;
    else
      coef.MCU_rows_per_iMCU_row = cinfo->cur_comp_info[0]->last_row_height;
  }
  coef.MCU_ctr = 0;
  coef.MCU_vert_offset = 0;
}

/* jdcoefct.c's start_input_pass. */
static void StartInputPass(j_decompress_ptr cinfo)
{
  cinfo->input_iMCU_row = 0;
  StartIMCURow(cinfo);
}

/* jdcoefct.c's multi-scan consume_data, with the whole image in place of the
 * virtual arrays, and MCU_data = NULL for the first -nullrows iMCU rows. */
static int ConsumeData(j_decompress_ptr cinfo)
{
  JDIMENSION MCU_col_num;
  int blkn, ci, xindex, yindex, yoffset;
  JDIMENSION start_col;
  JBLOCKARRAY buffer[MAX_COMPS_IN_SCAN];
  JBLOCKROW buffer_ptr;
  jpeg_component_info *compptr;

  for (ci = 0; ci < cinfo->comps_in_scan; ci++) {
    compptr = cinfo->cur_comp_info[ci];
    buffer[ci] = coef.whole_image[compptr->component_index] +
                 cinfo->input_iMCU_row * compptr->v_samp_factor;
  }

  for (yoffset = coef.MCU_vert_offset; yoffset < coef.MCU_rows_per_iMCU_row;
       yoffset++) {
    for (MCU_col_num = coef.MCU_ctr; MCU_col_num < cinfo->MCUs_per_row;
         MCU_col_num++) {
      blkn = 0;
      for (ci = 0; ci < cinfo->comps_in_scan; ci++) {
        compptr = cinfo->cur_comp_info[ci];
        start_col = MCU_col_num * compptr->MCU_width;
        for (yindex = 0; yindex < compptr->MCU_height; yindex++) {
          buffer_ptr = buffer[ci][yindex + yoffset] + start_col;
          for (xindex = 0; xindex < compptr->MCU_width; xindex++) {
            coef.MCU_buffer[blkn++] = buffer_ptr++;
          }
        }
      }
      if (!cinfo->entropy->insufficient_data)
        cinfo->master->last_good_iMCU_row = cinfo->input_iMCU_row;
      if (!(*cinfo->entropy->decode_mcu)
            (cinfo, (int)cinfo->input_iMCU_row < coef.null_rows ? NULL :
                    coef.MCU_buffer)) {
        coef.MCU_vert_offset = yoffset;
        coef.MCU_ctr = MCU_col_num;
        return JPEG_SUSPENDED;
      }
    }
    coef.MCU_ctr = 0;
  }
  if (++(cinfo->input_iMCU_row) < cinfo->total_iMCU_rows) {
    StartIMCURow(cinfo);
    return JPEG_ROW_COMPLETED;
  }
  (*cinfo->inputctl->finish_input_pass) (cinfo);
  return JPEG_SCAN_COMPLETED;
}

/* The source position: an offset into the input, or -1 elsewhere (a fake
 * EOI). */
static long SourceOffset(j_decompress_ptr cinfo)
{
  const JOCTET *next = cinfo->src->next_input_byte;

  if (suspending.fake_eoi)
    return -1;
  if (next >= input_data && next <= input_data + input_size)
    return (long)(next - input_data);
  return -1;
}

static void PrintState(const char *key, j_decompress_ptr cinfo)
{
  printf("%s=%d,%d,%d,%d,%d,%d,%d,%d,%d,%u,%ld,%lu,%u,%u,%ld\n", key,
         cinfo->input_scan_number, cinfo->comps_in_scan, cinfo->Ss,
         cinfo->Se, cinfo->Ah, cinfo->Al,
         cinfo->entropy ? cinfo->entropy->insufficient_data : -1,
         cinfo->unread_marker, cinfo->marker->next_restart_num,
         cinfo->marker->discarded_bytes, SourceOffset(cinfo),
         (unsigned long)cinfo->src->bytes_in_buffer, cinfo->input_iMCU_row,
         cinfo->master->last_good_iMCU_row, cinfo->err->num_warnings);
  if (cinfo->coef_bits) {
    int i, n = cinfo->num_components * 2 * DCTSIZE2;
    const int *bits = &cinfo->coef_bits[0][0];

    printf("coef_bits=");
    for (i = 0; i < n; i++)
      printf("%s%d", i ? "," : "", bits[i]);
    printf("\n");
  }
}

static void PrintRows(void)
{
  int ci;
  JDIMENSION row, col;
  int k;

  for (ci = 0; ci < cinfo.num_components; ci++) {
    if (!coef.whole_image[ci])
      continue;
    printf("rows%d=", ci);
    for (row = 0; row < coef.rows[ci]; row++) {
      unsigned long long hash = 1469598103934665603ULL;

      for (col = 0; col < coef.cols[ci]; col++) {
        for (k = 0; k < DCTSIZE2; k++) {
          unsigned short value = (unsigned short)coef.whole_image[ci][row][col][k];

          hash = (hash ^ (value & 0xFF)) * 1099511628211ULL;
          hash = (hash ^ (value >> 8)) * 1099511628211ULL;
        }
      }
      printf("%s%016llx", row ? "," : "", hash);
    }
    printf("\n");
  }
}

static void AllocateImage(j_decompress_ptr cinfo)
{
  int ci;
  JDIMENSION row;

  for (ci = 0; ci < cinfo->num_components; ci++) {
    jpeg_component_info *compptr = &cinfo->comp_info[ci];
    JDIMENSION rows = (JDIMENSION)jround_up((long)compptr->height_in_blocks,
                                            (long)compptr->v_samp_factor);
    JDIMENSION cols = (JDIMENSION)jround_up((long)compptr->width_in_blocks,
                                            (long)compptr->h_samp_factor);

    coef.rows[ci] = rows;
    coef.cols[ci] = cols;
    coef.whole_image[ci] = (JBLOCKARRAY)calloc(rows, sizeof(JBLOCKROW));
    for (row = 0; row < rows; row++)
      coef.whole_image[ci][row] = (JBLOCKROW)calloc(cols, sizeof(JBLOCK));
  }
}

int main(int argc, char *argv[])
{
  int unneeded[MAX_UNNEEDED];
  int unneeded_count = 0, i, trace = 0, scaled = 0, null_rows = 0;
  long suspend = 0;
  volatile long calls = 0;
  const char *path = NULL;
  unsigned char *data;
  size_t size;
  int status;

  for (i = 1; i < argc; i++) {
    if (!strcmp(argv[i], "-trace") && i + 1 < argc) {
      trace = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-suspend") && i + 1 < argc) {
      suspend = atol(argv[++i]);
    } else if (!strcmp(argv[i], "-scaled") && i + 1 < argc) {
      scaled = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-unneeded") && i + 1 < argc &&
               unneeded_count < MAX_UNNEEDED) {
      unneeded[unneeded_count++] = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-nullrows") && i + 1 < argc) {
      null_rows = atoi(argv[++i]);
    } else if (argv[i][0] == '-' && argv[i][1] != '\0') {
      fprintf(stderr, "unknown option %s\n", argv[i]);
      return 2;
    } else {
      path = argv[i];
    }
  }

  data = ProbeReadInput(path, &size);
  input_data = data;
  input_size = size;
  cinfo.err = Wp2ErrorInit(&err);
  if (setjmp(err.setjmp_buffer)) {
    FlushValue();
    printf("consume=%s\n", consume_log);
    if (cinfo.master && cinfo.marker && cinfo.src)
      PrintState("final", &cinfo);
    Wp2PrintMemory(&cinfo);
    PrintRows();
    Wp2PrintError(&err);
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

  if (scaled > 0)
    for (i = 0; i < cinfo.num_components; i++)
      cinfo.comp_info[i].DCT_scaled_size = scaled;
  for (i = 0; i < unneeded_count; i++)
    if (unneeded[i] >= 0 && unneeded[i] < cinfo.num_components)
      cinfo.comp_info[unneeded[i]].component_needed = FALSE;

  /* jdmaster.c's master_selection */
  if (cinfo.arith_code) {
    printf("decoder=arith\n");
    jinit_arith_decoder(&cinfo);
  } else if (cinfo.progressive_mode) {
    printf("decoder=phuff\n");
    jinit_phuff_decoder(&cinfo);
  } else {
    printf("decoder=huff\n");
    jinit_huff_decoder(&cinfo);
  }

  AllocateImage(&cinfo);
  coef.null_rows = null_rows;
  coef.pub.start_input_pass = StartInputPass;
  coef.pub.consume_data = ConsumeData;
  cinfo.coef = &coef.pub;

  (*cinfo.inputctl->start_input_pass) (&cinfo);
  for (;;) {
    int value;

    if (++calls > MAX_CALLS) {
      printf("call_limit=1\n");
      break;
    }
    value = (*cinfo.inputctl->consume_input) (&cinfo);
    LogValue(value);
    if (value == JPEG_SCAN_COMPLETED)
      PrintState("scan", &cinfo);
    if (value == JPEG_REACHED_EOI)
      break;
    if (value == JPEG_SUSPENDED) {
      if (suspend == 0 || suspending.fake_eoi) {
        printf("stuck=1\n");
        break;
      }
      Wp2Release(&suspending, (size_t)suspend);
    }
  }
  FlushValue();
  printf("consume=%s\n", consume_log);
  PrintState("final", &cinfo);
  Wp2PrintMemory(&cinfo);
  PrintRows();
  Wp2PrintError(&err);
  return 0;
}
