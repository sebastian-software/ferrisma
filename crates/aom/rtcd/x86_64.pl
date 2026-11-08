# x86-64 specializations implemented by the hand-translated kernels in
# crates/aom/src/simd (ADR 0056, ADR 0058). tools/transpile_aom.py passes this
# file to the original cmake/rtcd.pl after aom_dsp_rtcd_defs.pl,
# aom_scale_rtcd.pl, or av1_rtcd_defs.pl. It withdraws every SSE2, SSE4.1 and
# AVX2 specialization that the port does not implement, so the generated
# dispatch header names only existing functions. Every listed
# `<name>_<extension>` must be bit-exact with `<name>_c` for the same
# arguments. Variants implemented only in NASM files are never listed.
my %ported_x86 = map { $_ => 1 } qw/
  av1_block_error_avx2
  av1_block_error_lp_avx2
  av1_highbd_block_error_avx2
  cdef_copy_rect8_16bit_to_16bit_avx2
  cdef_copy_rect8_8bit_to_16bit_avx2
  cdef_filter_16_0_avx2
  cdef_filter_16_1_avx2
  cdef_filter_16_2_avx2
  cdef_filter_16_3_avx2
  cdef_filter_8_0_avx2
  cdef_filter_8_1_avx2
  cdef_filter_8_2_avx2
  cdef_filter_8_3_avx2
  cdef_find_dir_avx2
  cdef_find_dir_dual_avx2
/;
foreach my $fn (keys %ALL_FUNCS) {
  foreach my $ext (qw/sse2 sse4_1 avx2/) {
    undef ${"${fn}_${ext}"} unless $ported_x86{"${fn}_${ext}"};
  }
}
1;
