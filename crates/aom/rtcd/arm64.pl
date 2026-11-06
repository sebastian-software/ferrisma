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
  av1_highbd_quantize_fp
/;
foreach my $fn (keys %ALL_FUNCS) {
  undef ${"${fn}_neon"} unless $ported_neon{$fn};
}
1;
