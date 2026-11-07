/* WP2 probe support; see wp2_support.h.
 *
 * This file also defines the eight functions of jmemnobs.c, with calloc()
 * in place of malloc(), so that the linker takes them instead of the
 * library's jmemnobs.o. Memory that a failing or suspended call leaves
 * unwritten is then zero, as in the port, and the probe can print it. The
 * library itself is unmodified.
 */

#include "wp2_support.h"
#include "jmemsys.h"

#include <stdlib.h>
#include <string.h>

/* --- jmemnobs.c with zeroed memory --- */

GLOBAL(void *)
jpeg_get_small(j_common_ptr cinfo, size_t sizeofobject)
{
  return calloc(1, sizeofobject);
}

GLOBAL(void)
jpeg_free_small(j_common_ptr cinfo, void *object, size_t sizeofobject)
{
  free(object);
}

GLOBAL(void *)
jpeg_get_large(j_common_ptr cinfo, size_t sizeofobject)
{
  return calloc(1, sizeofobject);
}

GLOBAL(void)
jpeg_free_large(j_common_ptr cinfo, void *object, size_t sizeofobject)
{
  free(object);
}

GLOBAL(size_t)
jpeg_mem_available(j_common_ptr cinfo, size_t min_bytes_needed,
                   size_t max_bytes_needed, size_t already_allocated)
{
  if (cinfo->mem->max_memory_to_use) {
    if ((size_t)cinfo->mem->max_memory_to_use > already_allocated)
      return cinfo->mem->max_memory_to_use - already_allocated;
    else
      return 0;
  } else {
    return max_bytes_needed;
  }
}

GLOBAL(void)
jpeg_open_backing_store(j_common_ptr cinfo, backing_store_ptr info,
                        long total_bytes_needed)
{
  ERREXIT(cinfo, JERR_NO_BACKING_STORE);
}

GLOBAL(long)
jpeg_mem_init(j_common_ptr cinfo)
{
  return 0;
}

GLOBAL(void)
jpeg_mem_term(j_common_ptr cinfo)
{
}

/* --- the memory manager's private state, copied from jmemmgr.c --- */

typedef struct probe_pool_hdr {
  struct probe_pool_hdr *next;
  size_t bytes_used;
  size_t bytes_left;
} probe_pool_hdr;

typedef struct {
  struct jpeg_memory_mgr pub;
  probe_pool_hdr *small_list[JPOOL_NUMPOOLS];
  probe_pool_hdr *large_list[JPOOL_NUMPOOLS];
  void *virt_sarray_list;
  void *virt_barray_list;
  size_t total_space_allocated;
  JDIMENSION last_rowsperchunk;
} probe_memory_mgr;

/* --- the suspending source --- */

static const JOCTET fake_eoi[4] = { 0xFF, JPEG_EOI, 0, 0 };

static void SuspendInit(j_decompress_ptr cinfo)
{
}

static boolean SuspendFill(j_decompress_ptr cinfo)
{
  SuspendingSource *src = (SuspendingSource *)cinfo->src;

  if (src->released < src->size)
    return FALSE;
  WARNMS(cinfo, JWRN_JPEG_EOF);
  src->pub.next_input_byte = fake_eoi;
  src->pub.bytes_in_buffer = 2;
  src->fake_eoi = TRUE;
  return TRUE;
}

static void SuspendSkip(j_decompress_ptr cinfo, long num_bytes)
{
  SuspendingSource *src = (SuspendingSource *)cinfo->src;

  if (num_bytes <= 0)
    return;
  if ((size_t)num_bytes > src->pub.bytes_in_buffer) {
    src->skip_pending += num_bytes - (long)src->pub.bytes_in_buffer;
    src->pub.next_input_byte += src->pub.bytes_in_buffer;
    src->pub.bytes_in_buffer = 0;
  } else {
    src->pub.next_input_byte += num_bytes;
    src->pub.bytes_in_buffer -= num_bytes;
  }
}

static void SuspendTerm(j_decompress_ptr cinfo)
{
}

void Wp2InstallSuspendingSource(j_decompress_ptr cinfo, SuspendingSource *src,
                                const JOCTET *data, size_t size)
{
  memset(src, 0, sizeof(*src));
  src->pub.init_source = SuspendInit;
  src->pub.fill_input_buffer = SuspendFill;
  src->pub.skip_input_data = SuspendSkip;
  src->pub.resync_to_restart = jpeg_resync_to_restart;
  src->pub.term_source = SuspendTerm;
  src->pub.next_input_byte = data;
  src->pub.bytes_in_buffer = 0;
  src->data = data;
  src->size = size;
  cinfo->src = &src->pub;
}

void Wp2Release(SuspendingSource *src, size_t step)
{
  size_t pos;

  if (src->fake_eoi)
    return;
  pos = (size_t)(src->pub.next_input_byte - src->data) +
        (size_t)src->skip_pending;
  src->released = src->size - src->released > step ? src->released + step :
                                                     src->size;
  if (pos > src->released) {
    src->skip_pending = (long)(pos - src->released);
    pos = src->released;
  } else {
    src->skip_pending = 0;
  }
  src->pub.next_input_byte = src->data + pos;
  src->pub.bytes_in_buffer = src->released - pos;
}

/* --- the recording error manager --- */

static void Wp2ErrorExit(j_common_ptr cinfo)
{
  Wp2ErrorMgr *err = (Wp2ErrorMgr *)cinfo->err;

  err->error_code = err->pub.msg_code;
  (*err->pub.format_message) (cinfo, err->error_message);
  longjmp(err->setjmp_buffer, 1);
}

static void Wp2EmitMessage(j_common_ptr cinfo, int msg_level)
{
  Wp2ErrorMgr *err = (Wp2ErrorMgr *)cinfo->err;
  char buffer[JMSG_LENGTH_MAX];

  (*err->pub.format_message) (cinfo, buffer);
  printf("msg=%d:%d:", msg_level, err->pub.msg_code);
  ProbePrintHex("", (const unsigned char *)buffer, strlen(buffer));
  (*err->std_emit_message) (cinfo, msg_level);
}

static void Wp2OutputMessage(j_common_ptr cinfo)
{
  /* The standard emit_message decides what to show; nothing is printed. */
}

struct jpeg_error_mgr *Wp2ErrorInit(Wp2ErrorMgr *err)
{
  memset(err, 0, sizeof(*err));
  jpeg_std_error(&err->pub);
  err->std_emit_message = err->pub.emit_message;
  err->pub.error_exit = Wp2ErrorExit;
  err->pub.emit_message = Wp2EmitMessage;
  err->pub.output_message = Wp2OutputMessage;
  return &err->pub;
}

void Wp2PrintError(const Wp2ErrorMgr *err)
{
  printf("error=%d\n", err->error_code);
  if (err->error_code != 0)
    printf("error_message=%s\n", err->error_message);
  printf("num_warnings=%ld\n", err->pub.num_warnings);
}

/* --- the counting marker processor --- */

int wp2_processor_calls = 0;

boolean Wp2CountingProcessor(j_decompress_ptr cinfo)
{
  struct jpeg_source_mgr *src = cinfo->src;
  unsigned int length = 0;
  int i;

  wp2_processor_calls++;
  for (i = 0; i < 2; i++) {
    if (src->bytes_in_buffer == 0 && !(*src->fill_input_buffer) (cinfo))
      return FALSE;
    length = (length << 8) + *src->next_input_byte++;
    src->bytes_in_buffer--;
  }
  printf("processor=%d:%u\n", cinfo->unread_marker, length);
  if (length > 2)
    (*src->skip_input_data) (cinfo, (long)length - 2);
  return TRUE;
}

/* --- printing --- */

static void PrintPools(const char *key, probe_pool_hdr *hdr)
{
  printf("%s=", key);
  for (; hdr != NULL; hdr = hdr->next)
    printf("%lu/%lu%s", (unsigned long)hdr->bytes_used,
           (unsigned long)hdr->bytes_left, hdr->next != NULL ? "," : "");
  printf("\n");
}

/* The offset of the source position in the input data; 1000000 plus the
 * offset in the suspending source's fake EOI, or 2000000 when the memory
 * source points at its own fake EOI. */
static long SourceOffset(j_decompress_ptr cinfo, const JOCTET *data,
                         size_t size, const SuspendingSource *src)
{
  if (cinfo->src == NULL)
    return -1;
  if (src != NULL && src->fake_eoi)
    return 1000000 + (long)(cinfo->src->next_input_byte - fake_eoi);
  if (cinfo->src->next_input_byte < data ||
      cinfo->src->next_input_byte > data + size)
    return 2000000;
  return (long)(cinfo->src->next_input_byte - data);
}

void Wp2PrintRound(j_decompress_ptr cinfo, int round, int status,
                   const SuspendingSource *src)
{
  printf("round%d=%d,%d,%d,%ld,%lu,%u,%u,%d,%d,%d,%d,%d,%u,%ld,%u\n", round,
         status, cinfo->global_state, cinfo->unread_marker,
         SourceOffset(cinfo, src->data, src->size, src),
         (unsigned long)cinfo->src->bytes_in_buffer, cinfo->image_width,
         cinfo->image_height, cinfo->num_components, cinfo->data_precision,
         cinfo->marker->saw_SOI, cinfo->marker->saw_SOF,
         cinfo->input_scan_number, cinfo->marker->discarded_bytes,
         cinfo->err->num_warnings, cinfo->restart_interval);
}

void Wp2PrintObject(j_decompress_ptr cinfo, const JOCTET *data, size_t size,
                    const SuspendingSource *src, int aborted)
{
  probe_memory_mgr *mem = (probe_memory_mgr *)cinfo->mem;
  int ci, i;
  jpeg_saved_marker_ptr marker;

  printf("global_state=%d\n", cinfo->global_state);
  if (mem == NULL) {
    printf("mem=0\n");
    return;
  }
  printf("total=%lu\n", (unsigned long)mem->total_space_allocated);
  PrintPools("small0", mem->small_list[JPOOL_PERMANENT]);
  PrintPools("small1", mem->small_list[JPOOL_IMAGE]);
  PrintPools("large0", mem->large_list[JPOOL_PERMANENT]);
  PrintPools("large1", mem->large_list[JPOOL_IMAGE]);
  printf("source=%ld,%lu\n", SourceOffset(cinfo, data, size, src),
         cinfo->src ? (unsigned long)cinfo->src->bytes_in_buffer : 0UL);
  printf("image=%u,%u,%d,%d,%d\n", cinfo->image_width, cinfo->image_height,
         cinfo->num_components, cinfo->data_precision,
         (int)cinfo->jpeg_color_space);
  printf("params=%d,%u,%u,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d,%d\n",
         (int)cinfo->out_color_space, cinfo->scale_num, cinfo->scale_denom,
         (int)(cinfo->output_gamma * 1000), cinfo->buffered_image,
         cinfo->raw_data_out, (int)cinfo->dct_method,
         cinfo->do_fancy_upsampling, cinfo->do_block_smoothing,
         cinfo->quantize_colors, (int)cinfo->dither_mode,
         cinfo->two_pass_quantize, cinfo->desired_number_of_colors,
         cinfo->enable_1pass_quant, cinfo->enable_external_quant,
         cinfo->enable_2pass_quant);
  printf("progress=%d,%d,%d,%d,%u,%d\n", cinfo->progressive_mode,
         cinfo->arith_code, cinfo->CCIR601_sampling, cinfo->input_scan_number,
         cinfo->restart_interval, cinfo->unread_marker);
  printf("jfif=%d,%d,%d,%d,%u,%u\n", cinfo->saw_JFIF_marker,
         cinfo->JFIF_major_version, cinfo->JFIF_minor_version,
         cinfo->density_unit, cinfo->X_density, cinfo->Y_density);
  printf("adobe=%d,%d\n", cinfo->saw_Adobe_marker, cinfo->Adobe_transform);
  printf("arith=");
  for (i = 0; i < NUM_ARITH_TBLS; i++)
    printf("%s%d/%d/%d", i ? "," : "", cinfo->arith_dc_L[i],
           cinfo->arith_dc_U[i], cinfo->arith_ac_K[i]);
  printf("\n");
  printf("sampling=%d,%d,%d,%u\n", cinfo->max_h_samp_factor,
         cinfo->max_v_samp_factor, cinfo->min_DCT_scaled_size,
         cinfo->total_iMCU_rows);
  printf("scan=%d,%d,%d,%d,%d\n", cinfo->comps_in_scan, cinfo->Ss, cinfo->Se,
         cinfo->Ah, cinfo->Al);
  if (aborted) {
    /* jpeg_abort released comp_info; cinfo->comp_info dangles. */
    printf("comp_info_present=-\n");
  } else {
    printf("cur_comp_info=");
    for (i = 0; i < MAX_COMPS_IN_SCAN; i++)
      printf("%s%ld", i ? "," : "",
             cinfo->cur_comp_info[i] ? (long)(cinfo->cur_comp_info[i] -
                                              cinfo->comp_info) : -1L);
    printf("\n");
    printf("comp_info_present=%d\n", cinfo->comp_info != NULL);
    for (ci = 0; cinfo->comp_info != NULL && ci < cinfo->num_components &&
                 ci < MAX_COMPONENTS; ci++) {
      jpeg_component_info *c = &cinfo->comp_info[ci];

      printf("component%d=%d,%d,%d,%d,%d,%d,%d,%u,%u,%d,%u,%u,%d,%d\n", ci,
             c->component_id, c->component_index, c->h_samp_factor,
             c->v_samp_factor, c->quant_tbl_no, c->dc_tbl_no, c->ac_tbl_no,
             c->width_in_blocks, c->height_in_blocks, c->DCT_scaled_size,
             c->downsampled_width, c->downsampled_height, c->component_needed,
             c->quant_table != NULL);
    }
  }
  for (i = 0; i < NUM_QUANT_TBLS; i++) {
    int k;

    if (cinfo->quant_tbl_ptrs[i] == NULL) {
      printf("quant%d=-\n", i);
      continue;
    }
    printf("quant%d=%d:", i, cinfo->quant_tbl_ptrs[i]->sent_table);
    for (k = 0; k < DCTSIZE2; k++)
      printf("%s%u", k ? "," : "", cinfo->quant_tbl_ptrs[i]->quantval[k]);
    printf("\n");
  }
  for (i = 0; i < 2 * NUM_HUFF_TBLS; i++) {
    JHUFF_TBL *tbl = i < NUM_HUFF_TBLS ? cinfo->dc_huff_tbl_ptrs[i] :
                     cinfo->ac_huff_tbl_ptrs[i - NUM_HUFF_TBLS];
    char key[16];

    snprintf(key, sizeof(key), "huff%d", i);
    if (tbl == NULL) {
      printf("%s=-\n", key);
      continue;
    }
    printf("%s=%d:", key, tbl->sent_table);
    ProbePrintHex("", tbl->bits, 17);
    ProbePrintHex("huffval", tbl->huffval, 256);
  }
  for (i = 0, marker = cinfo->marker_list; marker != NULL;
       marker = marker->next, i++) {
    printf("saved%d=%d,%u,%u:", i, marker->marker, marker->original_length,
           marker->data_length);
    ProbePrintHex("", marker->data, marker->data_length);
  }
  printf("master=%d,%d,%d,", cinfo->master->lossless,
         cinfo->master->jpeg_data_precision,
         cinfo->master->marker_list_end != NULL);
  for (ci = 0; ci < MAX_COMPONENTS; ci++)
    printf("%s%u/%u", ci ? "," : "", cinfo->master->first_MCU_col[ci],
           cinfo->master->last_MCU_col[ci]);
  printf("\n");
  printf("inputctl=%d,%d\n", cinfo->inputctl->has_multiple_scans,
         cinfo->inputctl->eoi_reached);
  printf("marker=%d,%d,%d,%u\n", cinfo->marker->saw_SOI,
         cinfo->marker->saw_SOF, cinfo->marker->next_restart_num,
         cinfo->marker->discarded_bytes);
  printf("coef_bits=%d\n", cinfo->coef_bits != NULL);
}
