// WP2 probe: reports the target-dependent configuration of the bit I/O units,
// which the Rust translation selects from the target architecture (ADR 0002).
//
//   bits=N                  BITS of src/utils/bit_reader_utils.h
//   bit_t_bytes=N           sizeof(bit_t)
//   lbit_t_bytes=N          sizeof(lbit_t)
//   vp8l_atype_bytes=N      sizeof(vp8l_atype_t)
//   vp8l_wtype_bytes=N      sizeof(vp8l_wtype_t)
//   vp8l_writer_bytes=N     VP8L_WRITER_BYTES
//   vp8l_writer_bits=N      VP8L_WRITER_BITS
//   vp8l_writer_max_bits=N  VP8L_WRITER_MAX_BITS
//   fast_load=0|1           1 when VP8LDoFillBitWindow takes its four-byte fast
//                           path (VP8L_USE_FAST_LOAD, defined in
//                           bit_reader_utils.c, which is not visible here)
//
// fast_load is observed, not read from the source: with bit_pos 60 and 32 bytes
// of input, the fast path leaves bit_pos at 28, and the byte-by-byte path at 4.
//
// tests/wp2_config.rs compares these values with the Rust constants.

#include <stddef.h>
#include <stdint.h>
#include <stdio.h>
#include <string.h>

#include "probe_support.h"
#include "src/utils/bit_reader_inl_utils.h"
#include "src/utils/bit_reader_utils.h"
#include "src/utils/bit_writer_utils.h"

int main(int argc, char* argv[]) {
  uint8_t data[32];
  VP8LBitReader br;
  ProbeCheckArgs(argc, 0, 0, "config");
  printf("bits=%d\n", BITS);
  printf("bit_t_bytes=%zu\n", sizeof(bit_t));
  printf("lbit_t_bytes=%zu\n", sizeof(lbit_t));
  printf("vp8l_atype_bytes=%zu\n", sizeof(vp8l_atype_t));
  printf("vp8l_wtype_bytes=%zu\n", sizeof(vp8l_wtype_t));
  printf("vp8l_writer_bytes=%d\n", VP8L_WRITER_BYTES);
  printf("vp8l_writer_bits=%d\n", VP8L_WRITER_BITS);
  printf("vp8l_writer_max_bits=%d\n", VP8L_WRITER_MAX_BITS);
  memset(data, 0xa5, sizeof(data));
  VP8LInitBitReader(&br, data, sizeof(data));
  VP8LSetBitPos(&br, 60);
  VP8LFillBitWindow(&br);
  printf("fast_load=%d\n", br.bit_pos == 28 ? 1 : 0);
  return 0;
}
