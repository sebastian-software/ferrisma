# libavif-aom

Mechanical portable translation of libaom 3.15.2, commit
`af3dc9aadc793c1b00edc003e3fc19a44fdeb574`, built without VMAF tuning
([ADR 0057](../../docs/adr/0057-translate-libaom-without-vmaf-tuning.md)). It
covers the 166 C translation units of the generic configuration; the reference
boundary and the original test inventories are in the docstring of
`tools/build_aom.py`. The Rust API keeps the original unsafe contracts. The
crate is the mandatory root codec ([ADR 0049](../../docs/adr/0049-use-rust-aom-as-the-mandatory-root-codec.md)),
acceptance follows [ADR 0048](../../docs/adr/0048-require-complete-release-and-focused-debug-aom-acceptance.md),
and encoder output is gated on golden outputs
([ADR 0059](../../docs/adr/0059-freeze-the-libaom-translation-and-gate-owned-code-on-golden-outputs.md),
`tools/check_aom_golden.py`). Registry publication is disabled. The original
licenses, patent grant, and author notices are preserved in `LICENSES/` and
indexed by `NOTICE.md`.

## SIMD dispatch

Hand-translated kernels live in `src/simd` at their original paths and are
reached through the translated RTCD tables
([ADR 0056](../../docs/adr/0056-dispatch-hand-translated-aom-simd-through-translated-rtcd.md),
[ADR 0058](../../docs/adr/0058-dispatch-hand-translated-x86-simd-on-gnu-linux.md)).
`tools/transpile_aom.py` generates the dispatch headers with the original
`cmake/rtcd.pl` and one overlay per target: `rtcd/arm64.pl` for macOS ARM64 and
`rtcd/x86_64.pl` for GNU Linux x86-64 (run with `--arch=x86_64 --require-c` and
every family other than SSE2, SSE4.1, and AVX2 disabled). The overlay withdraws
every specialization the port does not implement, so the headers name only
existing functions. The translation manifests record each overlay's hash. Since
the freeze (ADR 0059), `tools/enable_aom_dispatch.py` changes the arm64 overlay
together with the dispatch headers, the owned RTCD and caller units, and their
recorded hashes (ADR 0056, amendment of 2026-11-13).

- Arm: 513 RTCD functions dispatch to Neon kernels: the high-bitdepth set (190
  intra predictors, transforms, CDEF, quantization) and, since 2026-11-13, the
  still-image 8-bit inverse transform `av1_inv_txfm_add`, the 40 loop filters,
  the 190 8-bit intra predictors with `av1_dr_prediction_z1/z2/z3`, the 9 CfL
  getters, the 4 loop-restoration kernels, 20 quantization, 8-bit forward
  transform, SSE/distortion and txb kernels, and `aom_sum_sse_2d_i16`,
  `av1_get_horver_correlation_full`, `av1_get_nz_map_contexts` and
  `av1_round_shift_array`. The last two are dispatched although they are not
  exact with `_c` for every argument (extra context positions; wrapping where
  `_c` saturates), because the original's Neon build runs them too and the
  SIMD-against-original gate is identical. The per-size `av1_inv_txfm2d_add_*`
  entry points are not dispatched (see `rtcd/arm64.pl`), nor are the
  high-bitdepth Hadamard kernels (coefficient order).
  ADR 0056 lists what remains on `_c`.
- x86-64: 104 RTCD functions dispatch through 111 kernel assignments, because
  some functions have both an SSE4.1 and an AVX2 variant. The high-bitdepth DC
  and V predictors for 4x4 to 32x32 exist upstream only as NASM and stay on `_c`,
  as do the high-bitdepth smooth and Paeth predictors, which have no x86
  specialization. AVX-512 is not dispatched.
- Scope (Ferrisma decision D6, 2026-11-13): nothing above AVX2 on x86-64 or
  above baseline Neon on Arm runs in v1.0, and none is translated, so there is
  no opt-in. `x86_simd_caps()` still reports `HAS_AVX512` and
  `aom_arm_cpu_caps()` still reports DotProd and I8MM, as the originals do, but
  no table entry tests those flags; `tests/dispatch_scope.rs` scans all twelve
  RTCD sources to keep it so. An original comparable with this port is built
  with `-DENABLE_AVX512=0` (x86-64) or `-DENABLE_NEON_DOTPROD=0
  -DENABLE_NEON_I8MM=0 -DENABLE_SVE=0 -DENABLE_SVE2=0` (arm64), the CMake
  switches behind the generator's `--disable-*` options, or run with
  `AOM_SIMD_CAPS_MASK=0x1ff` (x86-64) or `0x3` (arm64).
- `x86_simd_caps()` is translated with the RTCD units. Only its CPUID lowering is
  replaced by `core::arch::x86_64::__cpuid_count` (correction
  `cpuid-register-mapping`, three sites per profile, `map_cpuid` in
  `tools/transpile_aom.py`); XGETBV keeps its translated inline assembly.

Every dispatched `<name>_<extension>` must be bit-exact with `<name>_c` for the
same arguments. `cargo test --release -p libavif-aom --test simd` runs
supplementary transcriptions of the original exact-match SIMD tests (147 on Arm
on 2026-11-13, 79 on x86-64 at the last recorded run), and `tools/check_aom_simd_identity.py`
requires byte-identical AVIF output for every selection (`AOM_SIMD_CAPS`,
`AOM_SIMD_CAPS_MASK=0`). The original libaom SIMD tests do not run against these
kernels yet, because the original test build uses the generic configuration and
compiles no SIMD instantiations.
