# Arm specializations implemented by the hand-translated kernels in
# crates/aom/src/simd (ADR 0056). tools/transpile_aom.py passes this file to
# the original cmake/rtcd.pl after aom_dsp_rtcd_defs.pl, aom_scale_rtcd.pl,
# or av1_rtcd_defs.pl. It withdraws every Neon specialization that the port
# does not implement, so the generated dispatch header names only existing
# functions. Every listed function must have a `<name>_neon` definition.
my %ported_neon = map { $_ => 1 } qw/
  aom_highbd_quantize_b
  aom_highbd_quantize_b_32x32
  aom_highbd_quantize_b_32x32_adaptive
  aom_highbd_quantize_b_64x64
  aom_highbd_quantize_b_64x64_adaptive
  aom_highbd_quantize_b_adaptive
  aom_highbd_subtract_block
  aom_subtract_block
  av1_block_error
  av1_block_error_lp
  av1_highbd_block_error
  av1_highbd_quantize_fp
  cdef_copy_rect8_16bit_to_16bit
  cdef_copy_rect8_8bit_to_16bit
  cdef_filter_16_0
  cdef_filter_16_1
  cdef_filter_16_2
  cdef_filter_16_3
  cdef_filter_8_0
  cdef_filter_8_1
  cdef_filter_8_2
  cdef_filter_8_3
  cdef_find_dir
  cdef_find_dir_dual
/;
foreach my $fn (keys %ALL_FUNCS) {
  undef ${"${fn}_neon"} unless $ported_neon{$fn};
}
1;
