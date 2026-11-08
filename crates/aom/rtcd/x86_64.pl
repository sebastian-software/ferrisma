# x86-64 specializations implemented by the hand-translated kernels in
# crates/aom/src/simd (ADR 0056, ADR 0058). tools/transpile_aom.py passes this
# file to the original cmake/rtcd.pl after aom_dsp_rtcd_defs.pl,
# aom_scale_rtcd.pl, or av1_rtcd_defs.pl. It withdraws every SSE2, SSE4.1 and
# AVX2 specialization that the port does not implement, so the generated
# dispatch header names only existing functions. Every listed
# `<name>_<extension>` must be bit-exact with `<name>_c` for the same
# arguments. Variants implemented only in NASM files are never listed.
# aom_subtract_block_avx2 calls aom_subtract_block_sse2 for widths 4 and 8,
# which upstream implements only in NASM; the translation calls the
# equal-valued aom_subtract_block_c there instead. As on Arm, only
# av1_highbd_inv_txfm_add is dispatched among the inverse transforms: no
# libaom caller uses the per-size av1_inv_txfm2d_add_* names, the 8x8 SSE4.1
# entry handles only the non-identity types, and the SSE4.1 inverse WHT
# saturates outside the range that _c's range_check accepts.
my %ported_x86 = map { $_ => 1 } qw/
  aom_highbd_dc_128_predictor_16x16_sse2
  aom_highbd_dc_128_predictor_16x32_sse2
  aom_highbd_dc_128_predictor_16x8_sse2
  aom_highbd_dc_128_predictor_32x16_sse2
  aom_highbd_dc_128_predictor_32x32_sse2
  aom_highbd_dc_128_predictor_4x4_sse2
  aom_highbd_dc_128_predictor_4x8_sse2
  aom_highbd_dc_128_predictor_8x16_sse2
  aom_highbd_dc_128_predictor_8x4_sse2
  aom_highbd_dc_128_predictor_8x8_sse2
  aom_highbd_dc_left_predictor_16x16_sse2
  aom_highbd_dc_left_predictor_16x32_sse2
  aom_highbd_dc_left_predictor_16x8_sse2
  aom_highbd_dc_left_predictor_32x16_sse2
  aom_highbd_dc_left_predictor_32x32_sse2
  aom_highbd_dc_left_predictor_4x4_sse2
  aom_highbd_dc_left_predictor_4x8_sse2
  aom_highbd_dc_left_predictor_8x16_sse2
  aom_highbd_dc_left_predictor_8x4_sse2
  aom_highbd_dc_left_predictor_8x8_sse2
  aom_highbd_dc_predictor_16x32_sse2
  aom_highbd_dc_predictor_16x8_sse2
  aom_highbd_dc_predictor_32x16_sse2
  aom_highbd_dc_predictor_4x8_sse2
  aom_highbd_dc_predictor_8x16_sse2
  aom_highbd_dc_predictor_8x4_sse2
  aom_highbd_dc_top_predictor_16x16_sse2
  aom_highbd_dc_top_predictor_16x32_sse2
  aom_highbd_dc_top_predictor_16x8_sse2
  aom_highbd_dc_top_predictor_32x16_sse2
  aom_highbd_dc_top_predictor_32x32_sse2
  aom_highbd_dc_top_predictor_4x4_sse2
  aom_highbd_dc_top_predictor_4x8_sse2
  aom_highbd_dc_top_predictor_8x16_sse2
  aom_highbd_dc_top_predictor_8x4_sse2
  aom_highbd_dc_top_predictor_8x8_sse2
  aom_highbd_h_predictor_16x16_sse2
  aom_highbd_h_predictor_16x32_sse2
  aom_highbd_h_predictor_16x8_sse2
  aom_highbd_h_predictor_32x16_sse2
  aom_highbd_h_predictor_32x32_sse2
  aom_highbd_h_predictor_4x4_sse2
  aom_highbd_h_predictor_4x8_sse2
  aom_highbd_h_predictor_8x16_sse2
  aom_highbd_h_predictor_8x4_sse2
  aom_highbd_h_predictor_8x8_sse2
  aom_highbd_quantize_b_32x32_adaptive_avx2
  aom_highbd_quantize_b_32x32_avx2
  aom_highbd_quantize_b_64x64_adaptive_sse2
  aom_highbd_quantize_b_64x64_avx2
  aom_highbd_quantize_b_adaptive_avx2
  aom_highbd_quantize_b_avx2
  aom_highbd_subtract_block_sse2
  aom_highbd_v_predictor_16x32_sse2
  aom_highbd_v_predictor_16x8_sse2
  aom_highbd_v_predictor_32x16_sse2
  aom_highbd_v_predictor_4x8_sse2
  aom_highbd_v_predictor_8x16_sse2
  aom_highbd_v_predictor_8x4_sse2
  aom_subtract_block_avx2
  av1_block_error_avx2
  av1_block_error_lp_avx2
  av1_filter_intra_edge_sse4_1
  av1_filter_intra_predictor_sse4_1
  av1_highbd_block_error_avx2
  av1_highbd_dr_prediction_z1_avx2
  av1_highbd_dr_prediction_z2_avx2
  av1_highbd_dr_prediction_z3_avx2
  av1_highbd_filter_intra_edge_sse4_1
  av1_highbd_inv_txfm_add_avx2
  av1_highbd_inv_txfm_add_sse4_1
  av1_highbd_quantize_fp_avx2
  av1_highbd_upsample_intra_edge_sse4_1
  av1_upsample_intra_edge_sse4_1
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
