/* WP0 example probe: prints what jpeg_read_header() leaves in the
 * decompression object for one input.
 *
 * Usage: read_header [FILE]
 * Reads FILE, or standard input when FILE is omitted or "-", into memory,
 * attaches it with jpeg_mem_src() and calls jpeg_read_header(cinfo, TRUE).
 * Prints the return value, the header fields and the component layout, then
 * the error and warning codes. The fields are printed after a failing call
 * too, because the state a failing call leaves is part of the compared
 * behavior. tests/wp0_read_header.rs runs it.
 */

#include <stdio.h>
#include <stdlib.h>

#include "probe_support.h"

static void PrintHeader(const struct jpeg_decompress_struct *cinfo)
{
  int ci;

  printf("global_state=%d\n", cinfo->global_state);
  printf("image_width=%u\n", (unsigned)cinfo->image_width);
  printf("image_height=%u\n", (unsigned)cinfo->image_height);
  printf("num_components=%d\n", cinfo->num_components);
  printf("jpeg_color_space=%d\n", (int)cinfo->jpeg_color_space);
  printf("out_color_space=%d\n", (int)cinfo->out_color_space);
  printf("data_precision=%d\n", cinfo->data_precision);
  printf("progressive_mode=%d\n", (int)cinfo->progressive_mode);
  printf("arith_code=%d\n", (int)cinfo->arith_code);
  printf("restart_interval=%u\n", cinfo->restart_interval);
  printf("saw_JFIF_marker=%d\n", (int)cinfo->saw_JFIF_marker);
  printf("JFIF_version=%d.%02d\n", cinfo->JFIF_major_version,
         cinfo->JFIF_minor_version);
  printf("density_unit=%d\n", cinfo->density_unit);
  printf("X_density=%u\n", cinfo->X_density);
  printf("Y_density=%u\n", cinfo->Y_density);
  printf("saw_Adobe_marker=%d\n", (int)cinfo->saw_Adobe_marker);
  printf("max_h_samp_factor=%d\n", cinfo->max_h_samp_factor);
  printf("max_v_samp_factor=%d\n", cinfo->max_v_samp_factor);
  for (ci = 0; ci < cinfo->num_components && cinfo->comp_info != NULL; ci++) {
    const jpeg_component_info *comp = &cinfo->comp_info[ci];

    printf("component%d=id:%d,h:%d,v:%d,tq:%d\n", ci, comp->component_id,
           comp->h_samp_factor, comp->v_samp_factor, comp->quant_tbl_no);
  }
}

int main(int argc, char *argv[])
{
  struct jpeg_decompress_struct cinfo;
  ProbeErrorMgr err;
  size_t size;
  unsigned char *data;
  int status = -1;

  ProbeCheckArgs(argc, 0, 1, "read_header [FILE]");
  data = ProbeReadInput(argc > 1 ? argv[1] : NULL, &size);

  cinfo.err = ProbeErrorInit(&err);
  if (setjmp(err.setjmp_buffer) == 0) {
    jpeg_create_decompress(&cinfo);
    jpeg_mem_src(&cinfo, data, (unsigned long)size);
    status = jpeg_read_header(&cinfo, TRUE);
  }
  printf("status=%d\n", status);
  PrintHeader(&cinfo);
  ProbePrintErrors(&err);

  jpeg_destroy_decompress(&cinfo);
  free(data);
  return 0;
}
