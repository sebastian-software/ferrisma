/* WP6 probe: master control and the decompression API (jdmaster.c,
 * jdapistd.c and jpeg_finish_decompress of jdapimin.c), driven through the
 * public functions as an application drives them.
 *
 * Usage: api [OPTION]... [FILE]
 * Reads FILE (or standard input), attaches it with jpeg_mem_src, or with the
 * suspending source of wp2_support.c when -suspend is given, reads the
 * header, applies the options and decompresses: jpeg_start_decompress, then
 * an output pass of jpeg_read_scanlines and jpeg_skip_scanlines calls as the
 * plan says, then jpeg_finish_decompress. With -raw the output pass reads
 * jpeg_read_raw_data, and with -buffered every output pass of buffered-image
 * mode runs the plan between jpeg_start_output and jpeg_finish_output. When
 * a call suspends, the next step of the data is released and the call is
 * repeated. Options:
 *
 *   -trace N        err.trace_level = N (set before the header)
 *   -suspend N      the suspending source, releasing N bytes per step
 *   -out CS         out_color_space = CS (a J_COLOR_SPACE number)
 *   -nofancy        do_fancy_upsampling = FALSE
 *   -scale N D      scale_num = N, scale_denom = D
 *   -dct M          dct_method = M (islow, ifast, float)
 *   -nosmooth       do_block_smoothing = FALSE
 *   -ccir601        CCIR601_sampling = TRUE
 *   -quantize       quantize_colors = TRUE
 *   -twopass        two_pass_quantize = TRUE
 *   -precision N    data_precision = N
 *   -calc           call jpeg_calc_output_dimensions before starting
 *   -progress       install a progress monitor that hashes its fields
 *   -raw            raw_data_out = TRUE; read with jpeg_read_raw_data after
 *                   the plan
 *   -rawlines N     max_lines of jpeg_read_raw_data (default: one iMCU row)
 *   -buffered       buffered_image = TRUE
 *   -absorb         in buffered-image mode, jpeg_consume_input to EOI first
 *   -scan N         the scan number of the first jpeg_start_output
 *   -passes N       stop buffered-image mode after N output passes
 *   -newcolormap    call jpeg_new_colormap before the first output pass
 *   -extra          one more read and skip (with -raw: raw read) after the
 *                   last scanline
 *   -twice          decompress the file again with the same object
 *   -rows K         max_lines of the reads after the plan (default 1)
 *   -plan LIST      the start of each output pass, as comma-separated steps:
 *                     rK   read K lines: jpeg_read_scanlines calls with
 *                          max_lines of the lines still wanted
 *                     sK   one jpeg_skip_scanlines call for K lines
 *                     cX:W jpeg_crop_scanline with *xoffset X, *width W
 *                     C    jpeg_crop_scanline with null pointers
 *                     d    jpeg_calc_output_dimensions
 *                     n    jpeg_new_colormap
 *                     S    jpeg_start_decompress
 *                     O    jpeg_start_output of input_scan_number
 *                     F    jpeg_finish_output
 *                     P    data_precision = 12
 *                     e    end the pass: jpeg_finish_decompress now
 *                   After the plan, the rest of the pass is read with
 *                   max_lines K of -rows.
 *
 * Output: the `msg=` lines of wp2_support.c, `header=<status>`, `calc=` and
 * `crop=` when requested, `start=` (the number of suspensions of
 * jpeg_start_decompress), `dims=` with the output dimensions and state,
 * `pass=` for every buffered-image pass, `memory` before
 * jpeg_finish_decompress, `finish=`, and at the end `rows=` (an FNV-1a hash
 * of every output row, `p<N>` at the start of a buffered-image pass),
 * `read=`, `skip=` and `raw=` (every count those calls returned, with runs
 * counted), `progress=` (calls and hash), `final=` and the error.
 * tests/wp6_api.rs runs the same sequence on the Rust port.
 */

#include "wp2_support.h"

#include <stdlib.h>
#include <string.h>

#define MAX_CALLS  1000000L
#define MAX_PLAN  4096
#define FNV_OFFSET  1469598103934665603ULL
#define FNV_PRIME  1099511628211ULL

static struct jpeg_decompress_struct cinfo;
static Wp2ErrorMgr err;
static SuspendingSource suspending;
static unsigned char *data;
static size_t size;
static long suspend;
static long calls;
static int stuck;

/* Options */
static int trace, out = -1, nofancy, nosmooth, ccir601, quantize, twopass;
static int precision, calc, progress, raw, buffered, newcolormap, extra, twice;
static int absorb;
static unsigned int scale_num, scale_denom;
static const char *dct;
static long rawlines, first_scan, passes, rows = 1;
static int scan_set;

typedef struct {
  char op;
  long a, b;
} Step;

static Step plan[MAX_PLAN];
static int plan_len;

/* A log of return values, with runs of the same value counted. */
typedef struct {
  char text[1 << 18];
  size_t len;
  long last_value;
  long last_count;
} RunLog;

static RunLog read_log = { "", 0, -1000, 0 };
static RunLog skip_log = { "", 0, -1000, 0 };
static RunLog raw_log = { "", 0, -1000, 0 };

static void FlushLog(RunLog *log)
{
  if (log->last_count == 0)
    return;
  if (log->len + 48 < sizeof(log->text))
    log->len += (size_t)snprintf(log->text + log->len,
                                 sizeof(log->text) - log->len, "%s%ld*%ld",
                                 log->len ? "," : "", log->last_value,
                                 log->last_count);
  log->last_count = 0;
}

static void LogValue(RunLog *log, long value)
{
  if (value != log->last_value)
    FlushLog(log);
  log->last_value = value;
  log->last_count++;
}

/* The row hashes. */
static char rows_text[1 << 22];
static size_t rows_len;

static void AddEntry(const char *entry)
{
  if (rows_len + 24 < sizeof(rows_text))
    rows_len += (size_t)snprintf(rows_text + rows_len,
                                 sizeof(rows_text) - rows_len, "%s%s",
                                 rows_len ? "," : "", entry);
}

static void AddRow(const JSAMPLE *row, size_t width)
{
  unsigned long long hash = FNV_OFFSET;
  char entry[24];
  size_t col;

  for (col = 0; col < width; col++)
    hash = (hash ^ row[col]) * FNV_PRIME;
  snprintf(entry, sizeof(entry), "%016llx", hash);
  AddEntry(entry);
}

/* The progress monitor */
static long progress_calls;
static unsigned long long progress_hash = FNV_OFFSET;

static void Mix(long value)
{
  progress_hash = (progress_hash ^ (unsigned long long)value) * FNV_PRIME;
}

static void ProgressMonitor(j_common_ptr common)
{
  struct jpeg_progress_mgr *mgr = common->progress;

  progress_calls++;
  Mix(mgr->pass_counter);
  Mix(mgr->pass_limit);
  Mix(mgr->completed_passes);
  Mix(mgr->total_passes);
}

static struct jpeg_progress_mgr progress_mgr = { ProgressMonitor, 0, 0, 0, 0 };

/* The application's rows, freed by FreeBuffers on every exit. */
static JSAMPARRAY scanlines;
static size_t scanline_count, scanline_width;
static JSAMPARRAY planes[MAX_COMPONENTS];
static size_t plane_rows[MAX_COMPONENTS];

static void FreeRows(JSAMPARRAY *rows, size_t count)
{
  size_t i;

  if (!*rows)
    return;
  for (i = 0; i < count; i++)
    free((*rows)[i]);
  free(*rows);
  *rows = NULL;
}

static void FreeBuffers(void)
{
  int ci;

  FreeRows(&scanlines, scanline_count);
  scanline_count = scanline_width = 0;
  for (ci = 0; ci < MAX_COMPONENTS; ci++) {
    FreeRows(&planes[ci], plane_rows[ci]);
    plane_rows[ci] = 0;
  }
}

static JSAMPARRAY AllocRows(size_t count, size_t width)
{
  JSAMPARRAY rows = (JSAMPARRAY)calloc(count ? count : 1, sizeof(JSAMPROW));
  size_t i;

  for (i = 0; i < count; i++)
    rows[i] = (JSAMPROW)calloc(width ? width : 1, 1);
  return rows;
}

/* Grows the application's rows to `count` rows of `width` samples. */
static void EnsureScanlines(size_t count, size_t width)
{
  if (count <= scanline_count && width <= scanline_width)
    return;
  FreeRows(&scanlines, scanline_count);
  if (count < scanline_count)
    count = scanline_count;
  if (width < scanline_width)
    width = scanline_width;
  scanlines = AllocRows(count, width);
  scanline_count = count;
  scanline_width = width;
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

static int ParsePlan(const char *text)
{
  plan_len = 0;
  while (*text) {
    Step *step;
    char *end;

    if (plan_len == MAX_PLAN)
      return FALSE;
    step = &plan[plan_len++];
    step->op = *text++;
    step->a = step->b = 0;
    if (step->op == 'r' || step->op == 's' || step->op == 'c') {
      step->a = strtol(text, &end, 10);
      text = end;
      if (step->op == 'c') {
        if (*text != ':')
          return FALSE;
        step->b = strtol(text + 1, &end, 10);
        text = end;
      }
    } else if (!strchr("CdneSOFP", step->op)) {
      return FALSE;
    }
    if (*text == ',')
      text++;
    else if (*text)
      return FALSE;
  }
  return TRUE;
}

/* One jpeg_read_scanlines call with max_lines K. */
static JDIMENSION ReadRows(JDIMENSION max_lines)
{
  size_t width = (size_t)cinfo.output_width *
                 (size_t)cinfo.output_components;
  JDIMENSION n, row;

  EnsureScanlines(max_lines, width);
  n = jpeg_read_scanlines(&cinfo, scanlines, max_lines);
  LogValue(&read_log, (long)n);
  for (row = 0; row < n; row++)
    AddRow(scanlines[row], width);
  return n;
}

/* Reads `lines` lines: 1 when done, 0 when stuck. */
static int ReadLines(JDIMENSION lines, JDIMENSION max_lines)
{
  JDIMENSION got = 0;

  while (got < lines && cinfo.output_scanline < cinfo.output_height) {
    JDIMENSION n = ReadRows(max_lines ? max_lines : lines - got);

    got += n;
    if (n == 0 && !Resume())
      return 0;
  }
  return 1;
}

/* The plan's steps: 1 to go on, 0 when stuck, 2 for e. */
static int RunSteps(void)
{
  int i;

  for (i = 0; i < plan_len; i++) {
    const Step *step = &plan[i];

    switch (step->op) {
    case 'r':
      if (!ReadLines((JDIMENSION)step->a, 0))
        return 0;
      break;
    case 's':
      LogValue(&skip_log,
               (long)jpeg_skip_scanlines(&cinfo, (JDIMENSION)step->a));
      break;
    case 'c': {
      JDIMENSION xoffset = (JDIMENSION)step->a, width = (JDIMENSION)step->b;

      jpeg_crop_scanline(&cinfo, &xoffset, &width);
      printf("crop=%u,%u,%u\n", xoffset, width, cinfo.output_width);
      break;
    }
    case 'C':
      jpeg_crop_scanline(&cinfo, NULL, NULL);
      break;
    case 'd':
      jpeg_calc_output_dimensions(&cinfo);
      break;
    case 'n':
      jpeg_new_colormap(&cinfo);
      break;
    case 'S':
      jpeg_start_decompress(&cinfo);
      break;
    case 'O':
      jpeg_start_output(&cinfo, cinfo.input_scan_number);
      break;
    case 'F':
      jpeg_finish_output(&cinfo);
      break;
    case 'P':
      cinfo.data_precision = 12;
      break;
    case 'e':
      return 2;
    }
  }
  return 1;
}

/* An output pass of scanlines: 1 when complete, 0 when stuck, 2 for e. */
static int ReadPass(void)
{
  int result = RunSteps();

  if (result != 1)
    return result;
  return ReadLines(cinfo.output_height, (JDIMENSION)rows);
}

/* The raw output pass. */
static int RawPass(void)
{
  JDIMENSION lines = (JDIMENSION)(cinfo.max_v_samp_factor *
                                  cinfo.min_DCT_scaled_size);
  int ci, result = RunSteps();

  if (result != 1)
    return result;
  if (rawlines)
    lines = (JDIMENSION)rawlines;
  for (ci = 0; ci < cinfo.num_components; ci++) {
    jpeg_component_info *compptr = &cinfo.comp_info[ci];

    plane_rows[ci] = (size_t)compptr->v_samp_factor *
                     (size_t)compptr->DCT_scaled_size;
    planes[ci] = AllocRows(plane_rows[ci],
                           (size_t)compptr->width_in_blocks *
                           (size_t)compptr->DCT_scaled_size);
  }
  while (cinfo.output_scanline < cinfo.output_height) {
    JDIMENSION n = jpeg_read_raw_data(&cinfo, planes, lines);

    LogValue(&raw_log, (long)n);
    if (n == 0) {
      if (!Resume())
        return 0;
      continue;
    }
    for (ci = 0; ci < cinfo.num_components; ci++) {
      jpeg_component_info *compptr = &cinfo.comp_info[ci];
      size_t row;

      for (row = 0; row < plane_rows[ci]; row++)
        AddRow(planes[ci][row], (size_t)compptr->width_in_blocks *
                                (size_t)compptr->DCT_scaled_size);
    }
  }
  return 1;
}

/* The output passes of buffered-image mode. */
static int BufferedPasses(void)
{
  long pass;

  if (newcolormap)
    jpeg_new_colormap(&cinfo);
  if (absorb) {
    int value;

    while ((value = jpeg_consume_input(&cinfo)) != JPEG_REACHED_EOI)
      if (value == JPEG_SUSPENDED && !Resume())
        return 0;
    printf("absorb=%d\n", cinfo.input_scan_number);
  }
  for (pass = 0;; pass++) {
    int scan = cinfo.input_scan_number, result;
    char entry[24];

    if (pass == 0 && scan_set)
      scan = (int)first_scan;
    while (!jpeg_start_output(&cinfo, scan))
      if (!Resume())
        return 0;
    snprintf(entry, sizeof(entry), "p%d", cinfo.output_scan_number);
    AddEntry(entry);
    result = ReadPass();
    if (result != 1)
      return result;
    while (!jpeg_finish_output(&cinfo))
      if (!Resume())
        return 0;
    printf("pass=%d,%d,%d\n", cinfo.output_scan_number,
           cinfo.input_scan_number, cinfo.inputctl->eoi_reached);
    if (jpeg_input_complete(&cinfo) &&
        cinfo.input_scan_number == cinfo.output_scan_number)
      return 1;
    if (passes && pass + 1 >= passes)
      return 1;
  }
}

static void PrintFinal(void)
{
  FlushLog(&read_log);
  FlushLog(&skip_log);
  FlushLog(&raw_log);
  if (rows_len)
    printf("rows=%s\n", rows_text);
  printf("read=%s\n", read_log.text);
  printf("skip=%s\n", skip_log.text);
  printf("raw=%s\n", raw_log.text);
  if (progress)
    printf("progress=%ld,%016llx\n", progress_calls, progress_hash);
  if (stuck)
    printf("stuck=1\n");
  printf("final=%d,%u,%d,%d,%u,%u,%d\n", cinfo.global_state,
         cinfo.output_scanline, cinfo.input_scan_number,
         cinfo.output_scan_number, cinfo.input_iMCU_row,
         cinfo.output_iMCU_row,
         cinfo.inputctl ? cinfo.inputctl->eoi_reached : -1);
  Wp2PrintError(&err);
}

/* One decompression with the object: FALSE when stuck. */
static int DecodeOnce(void)
{
  int status, starts = 0, result, finishes = 0;

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
  if (status != JPEG_HEADER_OK)
    return FALSE;

  if (out >= 0)
    cinfo.out_color_space = (J_COLOR_SPACE)out;
  if (dct)
    cinfo.dct_method = ParseMethod(dct);
  if (scale_denom) {
    cinfo.scale_num = scale_num;
    cinfo.scale_denom = scale_denom;
  }
  if (nofancy)
    cinfo.do_fancy_upsampling = FALSE;
  if (nosmooth)
    cinfo.do_block_smoothing = FALSE;
  if (ccir601)
    cinfo.CCIR601_sampling = TRUE;
  if (quantize)
    cinfo.quantize_colors = TRUE;
  if (twopass)
    cinfo.two_pass_quantize = TRUE;
  if (precision)
    cinfo.data_precision = precision;
  if (raw)
    cinfo.raw_data_out = TRUE;
  if (buffered)
    cinfo.buffered_image = TRUE;
  if (calc) {
    jpeg_calc_output_dimensions(&cinfo);
    printf("calc=%u,%u,%d,%d,%d,%d\n", cinfo.output_width,
           cinfo.output_height, cinfo.out_color_components,
           cinfo.output_components, cinfo.rec_outbuf_height,
           cinfo.min_DCT_scaled_size);
  }

  while (!jpeg_start_decompress(&cinfo)) {
    starts++;
    if (!Resume()) {
      printf("start=%d\n", starts);
      return FALSE;
    }
  }
  printf("start=%d\n", starts);
  printf("dims=%u,%u,%d,%d,%d,%d,%d\n", cinfo.output_width,
         cinfo.output_height, cinfo.out_color_components,
         cinfo.output_components, cinfo.rec_outbuf_height,
         cinfo.min_DCT_scaled_size, cinfo.global_state);

  if (raw)
    result = RawPass();
  else if (buffered)
    result = BufferedPasses();
  else
    result = ReadPass();
  if (result == 0)
    return FALSE;
  if (result == 1 && extra) {
    if (raw) {
      LogValue(&raw_log, (long)jpeg_read_raw_data(&cinfo, planes, 1000));
    } else {
      ReadRows(1);
      LogValue(&skip_log, (long)jpeg_skip_scanlines(&cinfo, 5));
    }
  }

  Wp2PrintMemory(&cinfo);
  while (!jpeg_finish_decompress(&cinfo)) {
    finishes++;
    if (!Resume()) {
      printf("finish=%d\n", finishes);
      return FALSE;
    }
  }
  printf("finish=%d\n", finishes);
  return TRUE;
}

int main(int argc, char *argv[])
{
  const char *path = NULL;
  int i;

  for (i = 1; i < argc; i++) {
    const char *arg = argv[i];
    int more = i + 1 < argc;

    if (!strcmp(arg, "-trace") && more)
      trace = atoi(argv[++i]);
    else if (!strcmp(arg, "-suspend") && more)
      suspend = atol(argv[++i]);
    else if (!strcmp(arg, "-out") && more)
      out = atoi(argv[++i]);
    else if (!strcmp(arg, "-dct") && more)
      dct = argv[++i];
    else if (!strcmp(arg, "-scale") && i + 2 < argc) {
      scale_num = (unsigned int)atol(argv[++i]);
      scale_denom = (unsigned int)atol(argv[++i]);
    } else if (!strcmp(arg, "-precision") && more)
      precision = atoi(argv[++i]);
    else if (!strcmp(arg, "-rows") && more)
      rows = atol(argv[++i]);
    else if (!strcmp(arg, "-rawlines") && more)
      rawlines = atol(argv[++i]);
    else if (!strcmp(arg, "-scan") && more) {
      first_scan = atol(argv[++i]);
      scan_set = 1;
    }
    else if (!strcmp(arg, "-passes") && more)
      passes = atol(argv[++i]);
    else if (!strcmp(arg, "-plan") && more) {
      if (!ParsePlan(argv[++i])) {
        fprintf(stderr, "bad plan %s\n", argv[i]);
        return 2;
      }
    } else if (!strcmp(arg, "-nofancy"))
      nofancy = 1;
    else if (!strcmp(arg, "-nosmooth"))
      nosmooth = 1;
    else if (!strcmp(arg, "-ccir601"))
      ccir601 = 1;
    else if (!strcmp(arg, "-quantize"))
      quantize = 1;
    else if (!strcmp(arg, "-twopass"))
      twopass = 1;
    else if (!strcmp(arg, "-calc"))
      calc = 1;
    else if (!strcmp(arg, "-progress"))
      progress = 1;
    else if (!strcmp(arg, "-raw"))
      raw = 1;
    else if (!strcmp(arg, "-buffered"))
      buffered = 1;
    else if (!strcmp(arg, "-absorb"))
      absorb = 1;
    else if (!strcmp(arg, "-newcolormap"))
      newcolormap = 1;
    else if (!strcmp(arg, "-extra"))
      extra = 1;
    else if (!strcmp(arg, "-twice"))
      twice = 1;
    else if (arg[0] == '-') {
      fprintf(stderr, "unknown option %s\n", arg);
      return 2;
    } else
      path = arg;
  }

  data = ProbeReadInput(path, &size);
  cinfo.err = Wp2ErrorInit(&err);
  if (setjmp(err.setjmp_buffer)) {
    PrintFinal();
    jpeg_destroy_decompress(&cinfo);
    FreeBuffers();
    free(data);
    return 0;
  }
  jpeg_create_decompress(&cinfo);
  cinfo.err->trace_level = trace;
  if (progress)
    cinfo.progress = &progress_mgr;

  if (DecodeOnce() && twice) {
    FreeBuffers();
    AddEntry("again");
    DecodeOnce();
  }
  PrintFinal();
  jpeg_destroy_decompress(&cinfo);
  FreeBuffers();
  free(data);
  return 0;
}
