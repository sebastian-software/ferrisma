# Port the original's Highway SIMD paths per target

- Status: accepted
- Created: 2026-11-13
- Updated: 2026-11-13

## Context

The pinned libjxl implements its hot kernels as Highway templates: 83
`HWY_EXPORT`s in `lib/` (`tools/hwy_inventory.py`), of which the decoder path
uses 37 and the encoder 32 (the rest are the CMS, extras, tests and
benchmarks). The port so far translates only the `HWY_SCALAR` instantiation
(`base/hwy_scalar.rs`, unfused `MulAdd`) and the scalar oracle is built with
`HWY_COMPILE_ONLY_SCALAR=1`. The owner's quality gate (decisions D4–D8 of
2026-11-13) requires every SIMD path in scope (NEON on AArch64, AVX2 on
x86-64) to be bit-identical with the original's SIMD build at the same
instruction-set level, per target. A SIMD kernel cannot reuse the scalar
port: Highway's `MulAdd` is fused on NEON and AVX2, `Min`/`Max` follow each
instruction's NaN rule, reductions depend on the lane count and the AVX2
block structure, and estimates such as `ApproximateReciprocal` are
instruction-defined.

## Decision

**Highway operations layer.** `src/jxl/hwy/` translates the Highway
operations the SIMD kernels use for exactly two targets. `SimdF32` is the
counterpart of `HWY_FULL(float)` (plus `RebindToSigned` and the mask type);
`Neon` (4 lanes, `hwy/ops/arm_neon-inl.h`, AArch64 branches) and `Avx2`
(8 lanes, `x86_256-inl.h` with the `x86_128-inl.h`/`generic_ops-inl.h`
definitions AVX2 uses) implement it. Each method is a thin mapping to the
`core::arch` instruction Highway uses for that target, for example
`vfmaq_f32`/`_mm256_fmadd_ps` for `MulAdd`, `vminnmq_f32`/`_mm256_min_ps`
for `Min`, `vrecpeq_f32`/`_mm256_rcp_ps` for `ApproximateReciprocal`,
`_mm256_cmp_ps(_CMP_NEQ_OQ)` against NEON's `Not(Eq)` for `Ne`, Highway's
x86 saturation fix-up around `cvttps2dq`, and the generic AVX2
`SumOfLanes` shuffle order. Loads and stores take bounds-checked slices
(alignment is not observable). The layer covers 63 operations: arithmetic
including all four fused forms, min/max/clamp, abs/neg/abs-diff, sqrt and
both estimates, the four roundings, `ZeroIfNegative`, `CopySign(ToAbs)`,
bitwise operations, the six comparisons and three selections, conversions
and `NearestInt`, signed 32-bit arithmetic, comparisons and shifts, lane
reductions, `GetLane`, `InterleaveLower/Upper` and `StoreInterleaved2/3`.
The other operations the remaining exports use (see the plan) are added with
the kernel that first needs them, each with its oracle test.

**Instantiation.** A kernel body is written once as a `Kernel` generic over
`D: SimdF32`, like an original `-inl.h` body in `HWY_NAMESPACE`, and run with
`hwy::dispatch(target, kernel)`, the counterpart of `HWY_DYNAMIC_DISPATCH`
over a `foreach_target` instantiation. A descriptor is a capability token:
`Target::detect()` returns NEON on AArch64 and AVX2 only when the CPU reports
Highway's whole AVX2 group (`kGroupAVX2`); AVX-512 and SVE are never
selected (D6). On AVX2 the body runs inside a function compiled with
Highway's AVX2 attribute string, as `HWY_ATTR` does. Scalar code inside
`HWY_NAMESPACE` follows that target's contraction: for example the SIMD
Gaborish constructor fuses `w0 + 4 * (w1 + w2)` on both targets.

**Oracle variants.** `tools/oracle/CMakeLists.txt` takes
`-DORACLE_HWY_TARGET=SCALAR|NEON|AVX2`; SCALAR stays the default and keeps
all its flags. The SIMD variants force-include `hwy_target.h`, which sets
Highway's own `HWY_DISABLED_TARGETS` (so upstream's CMake adds no default
list), and drop the scalar reference's per-TU pins (`-O0`,
`-fno-unroll-loops`, `-fno-inline-functions`, `-ffp-model=strict`), which
only stabilize competing-NaN operand order between the plain and sanitized
scalar graphs. NEON compiles `HWY_TARGETS == HWY_NEON` (the AArch64 baseline
with AES; `NEON_BF16`/SVE excluded), which is what upstream's dispatch build
selects on such a CPU. AVX2 compiles every TU with Highway's `HWY_AVX2`
attribute string and `HWY_COMPILE_ONLY_STATIC`. The adapters name the
target namespace through `ORACLE_HWY_NS` (`N_SCALAR`, `N_NEON`, `N_AVX2`)
instead of a hard-wired `N_SCALAR`; the SCALAR build expands to the same
code. `hwy_ops.cc` adds `hwy-target` and `hwy-op <op> ...` requests, compiled
per target with `foreach_target` and called through `HWY_DYNAMIC_DISPATCH`;
the SIMD variants assert their target set at compile time and abort at start
when the CPU does not report the target. Build on x86-64 macOS with
`-DCMAKE_OSX_ARCHITECTURES=x86_64` and run under Rosetta 2 with
`ROSETTA_ADVERTISE_AVX=1` (Highway then detects the complete AVX2 group,
including AES, CLMUL, BMI2, FMA, F16C and LZCNT; without it only SSE4).
`tools/check.py` builds the host's variant into `target/oracle-neon` or
`target/oracle-avx2` and passes `JXL_CPP_ORACLE_SIMD` to the tests.

**Comparison rule.** A SIMD path is compared only with the original's SIMD
build at the same target and on the same architecture, never with the
scalar oracle and never NEON with AVX2. Words are bit-identical except that
any NaN equals any NaN (D8). Words the original computes from memory it
never initialized are not comparable and are excluded explicitly: full-vector
over-reads past a request's row length, horizontal-upsampling writes before
the output row, EPF's `size_t` sigma index that wraps for negative
`x + xpos + 16`, and EPF's full-vector load from its 8-entry `sad_mul`
array when `xpos` is not a lane multiple. `tests/simd_parity.rs`
(`cpp-oracle-simd`) states each rule next to its use.

**Decoder path.** The stage factories dispatch like upstream's
`HWY_DYNAMIC_DISPATCH(GetGaborishStage)` and its siblings:
`get_gaborish_stage`, `get_ycbcr_stage`, `get_chroma_upsampling_stage` and
`get_epf_stage`/`get_epf_stage_shared` return the SIMD stage for
`hwy::decoder_target()` and the scalar stage when it is `None`. Plane row
padding follows the selected target as `BytesPerRow` follows
`MaxVectorSize()` (`hwy::max_vector_size()`: 4 bytes on the scalar path,
16 on NEON, 32 on AVX2). `decoder_target()` is `None` unless
`hwy::DECODER_SELECTS_SIMD` is on; tests switch it like Highway's
`SetSupportedTargetsForTest` (`hwy::set_decoder_simd_for_test`). The
selection is **off** (2026-11-13): the whole-decode comparison below
differs, so the default decoder, its scalar gate and its outputs are
unchanged. Turn it on only when
`whole_decodes_with_the_simd_stages_against_the_original_target` reports no
differing word on every target in scope; the test fails once nothing
differs while the selection is still off, and fails on any difference once
it is on.

## Evidence (2026-11-13, macOS 27.0.1, M1 Ultra)

- `tests/simd_parity.rs` against the NEON oracle (native) and the AVX2
  oracle (x86-64 test binary under Rosetta 2): all 63 operations on 85,085
  `hwy-op` requests each target; Gaborish 2,000 rows (1,647,000 words NEON,
  1,967,000 AVX2); YCbCr and chroma upsampling 1,325 rows (4,393,325 /
  5,029,325 words); EPF passes 0–2, 37,200 rows (23,383,200 / 27,847,200
  words). A replay of the scalar parity suite's captured `gab-row`,
  `cs-row` and `epf-row` requests (1,860 + 4,884 + 2,208 in contract)
  also matched on both targets. No finite word differs.
- All 2,722 captured request files of the scalar suite run through both
  SIMD variants. The only crash was `etx-row`: the adapter sized the
  original's `premul_absorb` for one lane and passed unaligned rows to
  aligned `Load`; it now allocates full aligned vectors and prints the same
  words. The earlier fact-finding build's other two crashes did not recur.
- Whole decode, NEON (2026-11-13). `tests/simd_parity.rs`
  `whole_decodes_with_the_simd_stages_against_the_original_target` decodes
  24 VarDCT/XYB files of the original's encoder (65x33, 258x258, 513x257;
  Gaborish off/on; EPF 0-3) with the SIMD stages selected and with the
  scalar path, and compares the float color planes and the integer output
  with the NEON oracle's `vd-decode`. Status, frame size and output layout
  match on all 24, and the SIMD stages run without fault in the decoder's
  pipeline; the words do not match: 97.5-99.5 % of the float words differ
  on every file, with or without the SIMD stages, including the files
  without Gaborish and EPF (65x33: 6,271 of 6,435; 258x258: about 198,500
  of 199,692; 513x257: about 393,700 of 395,523), and 1-699 output bytes
  per file. The cause is outside these stages: the original's NEON build
  also runs dequantization, the inverse DCTs, XYB and the output stages as
  NEON code (fused `MulAdd`), which the port still runs as its
  `HWY_SCALAR` translation (Plan items 1 and 2). A 4:2:0 JPEG recompression
  (YCbCr, chroma upsampling) is not admitted by the port's `decode_vardct`
  yet and is reported, not compared. The comparison uses the simple
  pipeline of `decode_vardct`; the low-memory pipeline of the public decoder
  with SIMD stages has no whole-decode comparison yet. x86-64 (AVX2) was not
  run: the local machine has no Rosetta 2, and the hosted run of this test
  is pending.
- Row-request alignment (issue #252, 2026-11-13). The first hosted Linux
  x86-64 run failed the Gaborish, chroma/YCbCr and EPF row tests with a
  broken request pipe: the AVX2 oracle died at the first request whose
  `origin` is not a multiple of eight. The adapters (`gaborish_stage.inc`,
  `chroma_stages.cc`, `epf.cc`) handed the stages `Row(y) + origin`, while
  libjxl's pipelines hand them 128-byte aligned rows (the stage comment in
  `stage_gaborish.cc` relies on it) and the original's AVX2 objects store
  with `vmovaps` (`Store`, x86_256-inl.h `_mm256_store_ps`) and load with
  `vmovaps` in EPF; a misaligned `vmovaps` faults on x86-64 hardware. NEON's
  `Store`/`Load` are the unaligned `StoreU`/`LoadU` (arm_neon-inl.h), and
  Rosetta 2 did not fault. The adapters now shift each request's sample 0
  so that sample `origin` lies on a 128-byte boundary; requests, observed
  words and the compared domain are unchanged. The test helper reports the
  oracle's exit status and stderr before a write error.

## Consequences

- `rcpps` (open question O4). `_mm256_rcp_ps` and `rsqrtps` are
  vendor-specific approximations; Intel, AMD and Rosetta 2 need not agree.
  On one host the port and the oracle run the same instruction, so their
  agreement there is provable, and the operation tests prove it under
  Rosetta 2. That agreement says nothing about another CPU vendor, and a
  Rosetta-only run cannot show which hardware table the results correspond
  to. The pinned build sets `JXL_HIGH_PRECISION 1`, so EPF divides; the
  estimate remains on the decode path in `AdjustQuantBias`
  (`quantizer-inl.h`, dequantization), the rational polynomials and
  `!JXL_HIGH_PRECISION` code. How the
  gate treats these is the owner's open question O4; this ADR does not
  resolve it. The hosted Linux x86-64 run per release candidate (D7) is the
  first evidence on real AVX2 hardware.
- The AVX2 oracle is a static AVX2 build, not upstream's dispatch build:
  code outside `HWY_NAMESPACE` is also compiled with FMA, so Clang may
  contract scalar `a * b + c` there, which an SSE2-baseline dispatch build
  cannot. Kernels inside `HWY_NAMESPACE` are identical in both. A dispatch
  build needs adapters that compile per target with `foreach_target`
  instead of including pristine `.cc` files after `hwy/highway.h`; until
  then whole-decode AVX2 comparisons of non-kernel scalar code must use a
  dispatch `djxl`, not this oracle.
- The SIMD stages copy their row spans to apply the over-read/over-write
  semantics safely; they are correctness references, not performance
  measurements.

## Plan for the remaining exports

Ordered by decode-path reach, then size (lines of `HWY_NAMESPACE` code with
the included `-inl.h`, from `tools/hwy_inventory.py`):

1. Render pipeline, decoder: `stage_xyb` (441), `stage_upsampling` (616),
   `stage_noise` (293) with `dec_noise` (183), `stage_write` (686),
   `stage_blending` (219), `stage_splines` (39) with `splines` (513),
   `stage_from_linear` (1,046), `stage_to_linear` (1,051),
   `stage_tone_mapping` (1,003), `stage_cms` (133) and `cms/jxl_cms` (911).
2. VarDCT and modular decoding: `dec_group` with `dec_transforms-inl.h`
   (1,999; DCT, dequantization, `quantizer-inl.h` with the vendor-specific
   estimate), `compressed_dc` (336), `quant_weights` (671), `dec_xyb` (405),
   `dec_external_image` (57), `dec_modular` (56), `rct` (132), `squeeze`
   (696), `convolve_symmetric5` (156), `simd_util` (37).
3. Encoder: `enc_ac_strategy` (3,468), `enc_group` (2,913),
   `enc_chroma_from_luma` (2,669), `enc_adaptive_quantization` (2,514),
   `enc_transforms` (1,526), `enc_xyb` (1,009), `enc_ma` (809),
   `enc_cluster` (560), `enc_convolve_separable5` (479), `enc_modular_simd`
   (223), `enc_entropy_coder` (217), `enc_ans_simd` (211), `enc_rct` (165),
   `enc_detect_dots` (51); the encoder's butteraugli (1,735) when an
   encoder setting in scope reaches it.
4. Per stage, wire selection behind `Target::detect()` with the scalar path
   selectable, after the whole-decode comparison with a NEON/AVX2-restricted
   `djxl` passes on the conformance inputs.

## Review triggers

- The owner decides O4 or O1, or the Highway pin changes.
- An adapter is made per-target (then switch the AVX2 oracle to a dispatch
  build) or a target beyond NEON/AVX2 enters scope.
