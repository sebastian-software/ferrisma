/* WP1 probe: runs a script of memory manager operations (jmemmgr.c,
 * jmemnobs.c, jcomapi.c) and prints the manager's bookkeeping after each one.
 *
 * Usage: memory COMMAND...
 * Each command is a word followed by its integer arguments:
 *
 *   precision N              cinfo.data_precision = N
 *   init                     jinit_memory_mgr()
 *   maxmem N                 mem->max_memory_to_use = N
 *   small POOL SIZE          (*mem->alloc_small)()
 *   large POOL SIZE          (*mem->alloc_large)()
 *   sarray POOL WIDTH ROWS   (*mem->alloc_sarray)()
 *   barray POOL WIDTH ROWS   (*mem->alloc_barray)()
 *   vbarray POOL PREZERO WIDTH ROWS MAXACCESS
 *                            (*mem->request_virt_barray)()
 *   realize                  (*mem->realize_virt_arrays)()
 *   access ARRAY START ROWS WRITABLE
 *                            (*mem->access_virt_barray)(); prints the row
 *                            index of the result in the array's buffer
 *   poke ARRAY ROW BLOCK COEF VALUE
 *   peek ARRAY ROW BLOCK COEF
 *                            write or print one coefficient of a realized
 *                            virtual array's buffer
 *   free POOL                (*mem->free_pool)()
 *   quant / huff             jpeg_alloc_quant_table() / jpeg_alloc_huff_table()
 *   destroy                  jpeg_destroy()
 *
 * ARRAY counts virtual arrays in the order they were requested since the
 * last release of the image pool. After each command the probe prints
 * `step=<n>:<command>` and, while the memory manager exists, its state:
 * total_space_allocated, the small and large pool lists of both pools as
 * `used/left` records from the head of each list, last_rowsperchunk,
 * max_memory_to_use and max_alloc_chunk, and every virtual array. A failing
 * command prints the error and the state it leaves, and ends the script.
 *
 * my_memory_mgr and the virtual array control block are private to
 * jmemmgr.c; the structs below copy their definitions from libjpeg-turbo
 * 3.2.1. `init` checks the copy: total_space_allocated starts at the real
 * sizeof(my_memory_mgr), which the probe prints as `sizeof_mem` next to the
 * copy's size, and `vbarray` consumes the real control block size from a
 * small pool. tests/wp1_memory.rs runs the same scripts on the Rust port.
 */

#define JPEG_INTERNALS
#include "probe_support.h"
#include "jmemsys.h"

#include <stdlib.h>
#include <string.h>

typedef struct probe_pool_hdr {
  struct probe_pool_hdr *next;
  size_t bytes_used;
  size_t bytes_left;
} probe_pool_hdr;

struct probe_jvirt_barray {
  JBLOCKARRAY mem_buffer;
  JDIMENSION rows_in_array;
  JDIMENSION blocksperrow;
  JDIMENSION maxaccess;
  JDIMENSION rows_in_mem;
  JDIMENSION rowsperchunk;
  JDIMENSION cur_start_row;
  JDIMENSION first_undef_row;
  boolean pre_zero;
  boolean dirty;
  boolean b_s_open;
  struct probe_jvirt_barray *next;
  backing_store_info b_s_info;
};

typedef struct {
  struct jpeg_memory_mgr pub;
  probe_pool_hdr *small_list[JPOOL_NUMPOOLS];
  probe_pool_hdr *large_list[JPOOL_NUMPOOLS];
  void *virt_sarray_list;
  struct probe_jvirt_barray *virt_barray_list;
  size_t total_space_allocated;
  JDIMENSION last_rowsperchunk;
} probe_memory_mgr;

#define MAX_ARRAYS  64

static struct probe_jvirt_barray *arrays[MAX_ARRAYS];

/* The virtual arrays in request order: the list links each new one at its
 * head. */
static int CollectArrays(probe_memory_mgr *mem)
{
  struct probe_jvirt_barray *ptr;
  int count = 0, i;

  for (ptr = mem->virt_barray_list; ptr != NULL && count < MAX_ARRAYS;
       ptr = ptr->next)
    arrays[count++] = ptr;
  for (i = 0; i < count / 2; i++) {
    struct probe_jvirt_barray *swap = arrays[i];

    arrays[i] = arrays[count - 1 - i];
    arrays[count - 1 - i] = swap;
  }
  return count;
}

static void PrintPools(const char *key, probe_pool_hdr *hdr)
{
  printf("%s=", key);
  for (; hdr != NULL; hdr = hdr->next)
    printf("%lu/%lu%s", (unsigned long)hdr->bytes_used,
           (unsigned long)hdr->bytes_left, hdr->next != NULL ? "," : "");
  printf("\n");
}

static void PrintState(struct jpeg_decompress_struct *cinfo)
{
  probe_memory_mgr *mem = (probe_memory_mgr *)cinfo->mem;
  int count, i;

  printf("mem=%d\n", mem != NULL);
  printf("global_state=%d\n", cinfo->global_state);
  if (mem == NULL)
    return;
  printf("total=%lu\n", (unsigned long)mem->total_space_allocated);
  PrintPools("small0", mem->small_list[JPOOL_PERMANENT]);
  PrintPools("small1", mem->small_list[JPOOL_IMAGE]);
  PrintPools("large0", mem->large_list[JPOOL_PERMANENT]);
  PrintPools("large1", mem->large_list[JPOOL_IMAGE]);
  printf("last_rowsperchunk=%u\n", mem->last_rowsperchunk);
  printf("max_memory_to_use=%ld\n", mem->pub.max_memory_to_use);
  printf("max_alloc_chunk=%ld\n", mem->pub.max_alloc_chunk);
  count = CollectArrays(mem);
  printf("virt=");
  for (i = 0; i < count; i++) {
    struct probe_jvirt_barray *ptr = arrays[i];

    printf("%s%u/%u/%u/%d", i ? ";" : "", ptr->rows_in_array,
           ptr->blocksperrow, ptr->maxaccess, ptr->pre_zero);
    if (ptr->mem_buffer != NULL)
      printf("/%u/%u/%u/%u/%d", ptr->rows_in_mem, ptr->rowsperchunk,
             ptr->cur_start_row, ptr->first_undef_row, ptr->dirty);
  }
  printf("\n");
}

static struct probe_jvirt_barray *Array(struct jpeg_decompress_struct *cinfo,
                                        long index)
{
  int count = CollectArrays((probe_memory_mgr *)cinfo->mem);

  if (index < 0 || index >= count) {
    fprintf(stderr, "probe: no virtual array %ld\n", index);
    exit(2);
  }
  return arrays[index];
}

static long Argument(int argc, char *argv[], int *next)
{
  char *end;
  long value;

  if (*next >= argc) {
    fprintf(stderr, "probe: missing argument for %s\n", argv[*next - 1]);
    exit(2);
  }
  value = strtol(argv[*next], &end, 10);
  if (*end != '\0') {
    fprintf(stderr, "probe: bad integer %s\n", argv[*next]);
    exit(2);
  }
  (*next)++;
  return value;
}

/* Static, so that their values are determinate after longjmp(). */
static struct jpeg_decompress_struct cinfo;
static ProbeErrorMgr err;

int main(int argc, char *argv[])
{
  volatile int next = 1, step = 0;

  memset(&cinfo, 0, sizeof(cinfo));
  cinfo.err = ProbeErrorInit(&err);
  cinfo.is_decompressor = TRUE;
  cinfo.data_precision = 8;

  printf("sizeof_mem_copy=%lu\n", (unsigned long)sizeof(probe_memory_mgr));
  printf("sizeof_virt_copy=%lu\n",
         (unsigned long)sizeof(struct probe_jvirt_barray));

  if (setjmp(err.setjmp_buffer) == 0) {
    while (next < argc) {
      const char *command = argv[next];
      int position = next + 1;
      j_common_ptr common = (j_common_ptr)&cinfo;

      printf("step=%d:%s\n", step, command);
      if (strcmp(command, "precision") == 0) {
        cinfo.data_precision = (int)Argument(argc, argv, &position);
      } else if (strcmp(command, "init") == 0) {
        jinit_memory_mgr(common);
        printf("sizeof_mem=%lu\n", (unsigned long)
               ((probe_memory_mgr *)cinfo.mem)->total_space_allocated);
        /* jinit_memory_mgr leaves last_rowsperchunk uninitialized; the
         * library reads it only after alloc_sarray or alloc_barray has set
         * it. Define it for printing, as the port's 0. */
        ((probe_memory_mgr *)cinfo.mem)->last_rowsperchunk = 0;
      } else if (strcmp(command, "maxmem") == 0) {
        cinfo.mem->max_memory_to_use = Argument(argc, argv, &position);
      } else if (strcmp(command, "small") == 0) {
        int pool = (int)Argument(argc, argv, &position);
        size_t size = (size_t)Argument(argc, argv, &position);

        (*cinfo.mem->alloc_small) (common, pool, size);
      } else if (strcmp(command, "large") == 0) {
        int pool = (int)Argument(argc, argv, &position);
        size_t size = (size_t)Argument(argc, argv, &position);

        (*cinfo.mem->alloc_large) (common, pool, size);
      } else if (strcmp(command, "sarray") == 0) {
        int pool = (int)Argument(argc, argv, &position);
        JDIMENSION width = (JDIMENSION)Argument(argc, argv, &position);
        JDIMENSION rows = (JDIMENSION)Argument(argc, argv, &position);
        JSAMPARRAY array = (*cinfo.mem->alloc_sarray) (common, pool, width,
                                                       rows);

        /* the distance of rows 0 and 1 when one chunk holds both */
        printf("stride=%ld\n",
               rows > 1 && ((probe_memory_mgr *)cinfo.mem)->last_rowsperchunk > 1 ?
               (long)(array[1] - array[0]) : -1L);
      } else if (strcmp(command, "barray") == 0) {
        int pool = (int)Argument(argc, argv, &position);
        JDIMENSION width = (JDIMENSION)Argument(argc, argv, &position);
        JDIMENSION rows = (JDIMENSION)Argument(argc, argv, &position);

        (*cinfo.mem->alloc_barray) (common, pool, width, rows);
      } else if (strcmp(command, "vbarray") == 0) {
        int pool = (int)Argument(argc, argv, &position);
        boolean pre_zero = (boolean)Argument(argc, argv, &position);
        JDIMENSION width = (JDIMENSION)Argument(argc, argv, &position);
        JDIMENSION rows = (JDIMENSION)Argument(argc, argv, &position);
        JDIMENSION maxaccess = (JDIMENSION)Argument(argc, argv, &position);

        (*cinfo.mem->request_virt_barray) (common, pool, pre_zero, width,
                                           rows, maxaccess);
      } else if (strcmp(command, "realize") == 0) {
        (*cinfo.mem->realize_virt_arrays) (common);
      } else if (strcmp(command, "access") == 0) {
        struct probe_jvirt_barray *ptr =
          Array(&cinfo, Argument(argc, argv, &position));
        JDIMENSION start = (JDIMENSION)Argument(argc, argv, &position);
        JDIMENSION rows = (JDIMENSION)Argument(argc, argv, &position);
        boolean writable = (boolean)Argument(argc, argv, &position);
        JBLOCKARRAY result =
          (*cinfo.mem->access_virt_barray) (common, (jvirt_barray_ptr)ptr,
                                            start, rows, writable);

        printf("access_row=%ld\n", (long)(result - ptr->mem_buffer));
      } else if (strcmp(command, "poke") == 0) {
        struct probe_jvirt_barray *ptr =
          Array(&cinfo, Argument(argc, argv, &position));
        long row = Argument(argc, argv, &position);
        long block = Argument(argc, argv, &position);
        long coef = Argument(argc, argv, &position);

        ptr->mem_buffer[row][block][coef] =
          (JCOEF)Argument(argc, argv, &position);
      } else if (strcmp(command, "peek") == 0) {
        struct probe_jvirt_barray *ptr =
          Array(&cinfo, Argument(argc, argv, &position));
        long row = Argument(argc, argv, &position);
        long block = Argument(argc, argv, &position);
        long coef = Argument(argc, argv, &position);

        printf("peek=%d\n", ptr->mem_buffer[row][block][coef]);
      } else if (strcmp(command, "free") == 0) {
        (*cinfo.mem->free_pool) (common, (int)Argument(argc, argv, &position));
      } else if (strcmp(command, "quant") == 0) {
        JQUANT_TBL *tbl = jpeg_alloc_quant_table(common);

        printf("sent_table=%d\n", tbl->sent_table);
      } else if (strcmp(command, "huff") == 0) {
        JHUFF_TBL *tbl = jpeg_alloc_huff_table(common);

        printf("sent_table=%d\n", tbl->sent_table);
      } else if (strcmp(command, "destroy") == 0) {
        jpeg_destroy(common);
      } else {
        fprintf(stderr, "probe: unknown command %s\n", command);
        return 2;
      }
      next = position;
      PrintState(&cinfo);
      step++;
    }
  } else {
    /* realize_virt_arrays sets rows_in_mem of the array it fails on, except
     * for case 11, which it reports before it sets any. */
    probe_memory_mgr *mem = (probe_memory_mgr *)cinfo.mem;
    int case11 = err.error_code == JERR_OUT_OF_MEMORY &&
                 err.pub.msg_parm.i[0] == 11;

    if (mem != NULL && !case11) {
      int count = CollectArrays(mem), i;

      for (i = count - 1; i >= 0; i--) {
        if (arrays[i]->mem_buffer == NULL) {
          if (strcmp(argv[next], "realize") == 0)
            printf("pending_rows_in_mem=%u\n", arrays[i]->rows_in_mem);
          break;
        }
      }
    }
    PrintState(&cinfo);
  }
  ProbePrintErrors(&err);

  if (cinfo.mem != NULL)
    jpeg_destroy((j_common_ptr)&cinfo);
  return 0;
}
