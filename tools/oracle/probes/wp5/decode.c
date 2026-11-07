/* WP5 probe: upsampling, color conversion and the main and postprocessing
 * controllers (jdsample.c, jdmerge.c, jdmrgext.c, jdcolor.c, jdcolext.c,
 * jdpostct.c, jdmainct.c), driven by the coefficient controller and the IDCT.
 *
 * Usage: decode [OPTION]... [FILE]
 * Reads FILE (or standard input), attaches it with jpeg_mem_src, or with the
 * suspending source of wp2_support.c when -suspend is given, and reads the
 * header. After the options it calls jpeg_calc_output_dimensions and does
 * what jdmaster.c's master_selection does without color quantization:
 * prepare_range_limit_table, use_merged_upsample (both copied below), the
 * merged upsampler or the color deconverter and the upsampler, the
 * postprocessing controller, the IDCT, the entropy decoder, the coefficient
 * controller and the main controller, realize_virt_arrays and
 * start_input_pass. Then it does what jpeg_start_decompress does (absorb a
 * multi-scan file, prepare_for_output_pass's start_pass calls), reads all
 * scanlines through (*cinfo->main->process_data) as jpeg_read_scanlines
 * does, K rows per call into application rows, and consumes input to EOI as
 * jpeg_finish_decompress does. When the source suspends, the next step of
 * the data is released. Options, applied after the header:
 *
 *   -trace N        err.trace_level = N (set before the header)
 *   -suspend N      the suspending source, releasing N bytes per step
 *   -out CS         out_color_space = CS (a J_COLOR_SPACE number)
 *   -nofancy        do_fancy_upsampling = FALSE
 *   -scale N D      scale_num = N, scale_denom = D
 *   -dct M          dct_method = M (islow, ifast, float)
 *   -nosmooth       do_block_smoothing = FALSE
 *   -ccir601        CCIR601_sampling = TRUE
 *   -rows K         rows per process_data call (default 1)
 *
 * Output: the `msg=` lines of wp2_support.c, `header=<status>`, `dims=` with
 * the output dimensions, components and recommended buffer height,
 * `merged=`, `comp<ci>=` with each component's scaled size, downsampled size
 * and need, `context=` (need_context_rows), `rows=` (an FNV-1a hash of every
 * output row), `process=` (every row count process_data returned, with runs
 * counted), `consume=`, `final=`, the memory pools and the error.
 * tests/wp5_decode.rs runs the same sequence on the Rust port.
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

/* A log of return values, with runs of the same value counted. */
typedef struct {
  char text[1 << 18];
  size_t len;
  int last_value;
  long last_count;
} RunLog;

static RunLog consume_log = { "", 0, -1000, 0 };

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

static RunLog process_log = { "", 0, -1000, 0 };

/* The row hashes. */
static char rows_text[1 << 21];
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

/* jdmaster.c's use_merged_upsample, with UPSAMPLE_MERGING_SUPPORTED and
 * without lossless mode. */
static boolean UseMergedUpsample(j_decompress_ptr cinfo)
{
  if (cinfo->do_fancy_upsampling || cinfo->CCIR601_sampling)
    return FALSE;
  if (cinfo->jpeg_color_space != JCS_YCbCr || cinfo->num_components != 3 ||
      (cinfo->out_color_space != JCS_RGB &&
       cinfo->out_color_space != JCS_RGB565 &&
       cinfo->out_color_space != JCS_EXT_RGB &&
       cinfo->out_color_space != JCS_EXT_RGBX &&
       cinfo->out_color_space != JCS_EXT_BGR &&
       cinfo->out_color_space != JCS_EXT_BGRX &&
       cinfo->out_color_space != JCS_EXT_XBGR &&
       cinfo->out_color_space != JCS_EXT_XRGB &&
       cinfo->out_color_space != JCS_EXT_RGBA &&
       cinfo->out_color_space != JCS_EXT_BGRA &&
       cinfo->out_color_space != JCS_EXT_ABGR &&
       cinfo->out_color_space != JCS_EXT_ARGB))
    return FALSE;
  if ((cinfo->out_color_space == JCS_RGB565 &&
       cinfo->out_color_components != 3) ||
      (cinfo->out_color_space != JCS_RGB565 &&
       cinfo->out_color_components != rgb_pixelsize[cinfo->out_color_space]))
    return FALSE;
  if (cinfo->comp_info[0].h_samp_factor != 2 ||
      cinfo->comp_info[1].h_samp_factor != 1 ||
      cinfo->comp_info[2].h_samp_factor != 1 ||
      cinfo->comp_info[0].v_samp_factor >  2 ||
      cinfo->comp_info[1].v_samp_factor != 1 ||
      cinfo->comp_info[2].v_samp_factor != 1)
    return FALSE;
  if (cinfo->comp_info[0].DCT_scaled_size != cinfo->min_DCT_scaled_size ||
      cinfo->comp_info[1].DCT_scaled_size != cinfo->min_DCT_scaled_size ||
      cinfo->comp_info[2].DCT_scaled_size != cinfo->min_DCT_scaled_size)
    return FALSE;
  return TRUE;
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

static boolean Resume(void)
{
  if (suspend == 0 || suspending.fake_eoi || ++calls > MAX_CALLS) {
    stuck = 1;
    return FALSE;
  }
  Wp2Release(&suspending, (size_t)suspend);
  return TRUE;
}

static void ConsumeToEOI(void)
{
  while (!cinfo.inputctl->eoi_reached) {
    int value = (*cinfo.inputctl->consume_input) (&cinfo);

    LogValue(&consume_log, value);
    if (value == JPEG_SUSPENDED && !Resume())
      return;
  }
}

static void PrintFinal(void)
{
  FlushLog(&process_log);
  FlushLog(&consume_log);
  if (rows_len)
    printf("rows=%s\n", rows_text);
  printf("process=%s\n", process_log.text);
  printf("consume=%s\n", consume_log.text);
  if (stuck)
    printf("stuck=1\n");
  if (cinfo.master && cinfo.inputctl)
    printf("final=%d,%d,%u,%u,%u,%d\n", cinfo.input_scan_number,
           cinfo.output_scan_number, cinfo.input_iMCU_row,
           cinfo.output_iMCU_row, cinfo.output_scanline,
           cinfo.inputctl->eoi_reached);
  Wp2PrintMemory(&cinfo);
  Wp2PrintError(&err);
}

int main(int argc, char *argv[])
{
  int i, ci, trace = 0, rows = 1, merged;
  const char *path = NULL;
  unsigned char *data;
  size_t size;
  int status;
  JSAMPARRAY scanlines = NULL;
  JDIMENSION row_stride = 0, row;

  for (i = 1; i < argc; i++) {
    if (!strcmp(argv[i], "-trace") && i + 1 < argc) {
      trace = atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-suspend") && i + 1 < argc) {
      suspend = atol(argv[++i]);
    } else if (!strcmp(argv[i], "-rows") && i + 1 < argc) {
      rows = atoi(argv[++i]);
    } else if (argv[i][0] == '-' && argv[i][1] != '\0') {
      /* applied after the header */
      if (!strcmp(argv[i], "-out") || !strcmp(argv[i], "-dct"))
        i++;
      else if (!strcmp(argv[i], "-scale"))
        i += 2;
      else if (strcmp(argv[i], "-nofancy") && strcmp(argv[i], "-nosmooth") &&
               strcmp(argv[i], "-ccir601")) {
        fprintf(stderr, "unknown option %s\n", argv[i]);
        return 2;
      }
    } else {
      path = argv[i];
    }
  }

  data = ProbeReadInput(path, &size);
  cinfo.err = Wp2ErrorInit(&err);
  if (setjmp(err.setjmp_buffer)) {
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

  for (i = 1; i < argc; i++) {
    if (!strcmp(argv[i], "-out") && i + 1 < argc)
      cinfo.out_color_space = (J_COLOR_SPACE)atoi(argv[++i]);
    else if (!strcmp(argv[i], "-dct") && i + 1 < argc)
      cinfo.dct_method = ParseMethod(argv[++i]);
    else if (!strcmp(argv[i], "-scale") && i + 2 < argc) {
      cinfo.scale_num = (unsigned int)atoi(argv[++i]);
      cinfo.scale_denom = (unsigned int)atoi(argv[++i]);
    } else if (!strcmp(argv[i], "-nofancy"))
      cinfo.do_fancy_upsampling = FALSE;
    else if (!strcmp(argv[i], "-nosmooth"))
      cinfo.do_block_smoothing = FALSE;
    else if (!strcmp(argv[i], "-ccir601"))
      cinfo.CCIR601_sampling = TRUE;
    else if ((!strcmp(argv[i], "-trace") || !strcmp(argv[i], "-suspend") ||
              !strcmp(argv[i], "-rows")) && i + 1 < argc)
      i++;
  }

  /* jdmaster.c's master_selection, without color quantization */
  jpeg_calc_output_dimensions(&cinfo);
  PrepareRangeLimitTable(&cinfo);
  merged = UseMergedUpsample(&cinfo);
  printf("dims=%u,%u,%d,%d,%d,%d\n", cinfo.output_width, cinfo.output_height,
         cinfo.out_color_components, cinfo.output_components,
         cinfo.rec_outbuf_height, cinfo.min_DCT_scaled_size);
  printf("merged=%d\n", merged);
  if (merged) {
    jinit_merged_upsampler(&cinfo);
  } else {
    jinit_color_deconverter(&cinfo);
    jinit_upsampler(&cinfo);
  }
  jinit_d_post_controller(&cinfo, cinfo.enable_2pass_quant);
  jinit_inverse_dct(&cinfo);
  if (cinfo.arith_code)
    jinit_arith_decoder(&cinfo);
  else if (cinfo.progressive_mode)
    jinit_phuff_decoder(&cinfo);
  else
    jinit_huff_decoder(&cinfo);
  jinit_d_coef_controller(&cinfo, cinfo.inputctl->has_multiple_scans ||
                                  cinfo.buffered_image);
  jinit_d_main_controller(&cinfo, FALSE);
  (*cinfo.mem->realize_virt_arrays) ((j_common_ptr)&cinfo);
  (*cinfo.inputctl->start_input_pass) (&cinfo);
  cinfo.master->first_iMCU_col = 0;
  cinfo.master->last_iMCU_col = cinfo.MCUs_per_row - 1;
  cinfo.master->last_good_iMCU_row = 0;
  for (ci = 0; ci < cinfo.num_components; ci++) {
    jpeg_component_info *compptr = &cinfo.comp_info[ci];

    printf("comp%d=%d,%u,%u,%d\n", ci, compptr->DCT_scaled_size,
           compptr->downsampled_width, compptr->downsampled_height,
           compptr->component_needed);
  }
  printf("context=%d\n", cinfo.upsample->need_context_rows);

  /* jpeg_start_decompress */
  if (cinfo.inputctl->has_multiple_scans)
    ConsumeToEOI();
  cinfo.output_scan_number = cinfo.input_scan_number;
  if (!stuck) {
    /* prepare_for_output_pass */
    (*cinfo.idct->start_pass) (&cinfo);
    (*cinfo.coef->start_output_pass) (&cinfo);
    if (!merged)
      (*cinfo.cconvert->start_pass) (&cinfo);
    (*cinfo.upsample->start_pass) (&cinfo);
    (*cinfo.post->start_pass) (&cinfo, JBUF_PASS_THRU);
    (*cinfo.main->start_pass) (&cinfo, JBUF_PASS_THRU);

    /* The application's rows */
    row_stride = cinfo.output_width * (JDIMENSION)cinfo.output_components;
    scanlines = (JSAMPARRAY)calloc((size_t)rows, sizeof(JSAMPROW));
    for (i = 0; i < rows; i++)
      scanlines[i] = (JSAMPROW)calloc(row_stride, 1);

    /* jpeg_read_scanlines */
    while (cinfo.output_scanline < cinfo.output_height) {
      JDIMENSION row_ctr = 0;

      (*cinfo.main->process_data) (&cinfo, scanlines, &row_ctr,
                                   (JDIMENSION)rows);
      cinfo.output_scanline += row_ctr;
      LogValue(&process_log, (int)row_ctr);
      for (row = 0; row < row_ctr; row++) {
        unsigned long long hash = 1469598103934665603ULL;
        JDIMENSION col;

        for (col = 0; col < row_stride; col++)
          hash = (hash ^ scanlines[row][col]) * 1099511628211ULL;
        if (rows_len + 20 < sizeof(rows_text))
          rows_len += (size_t)snprintf(rows_text + rows_len,
                                       sizeof(rows_text) - rows_len,
                                       "%s%016llx", rows_len ? "," : "", hash);
      }
      if (row_ctr == 0 && !Resume())
        break;
    }
    /* jpeg_finish_decompress */
    if (!stuck)
      ConsumeToEOI();
  }
  PrintFinal();
  return 0;
}
