# Port the original's Highway SIMD paths per target

- Status: accepted
- Created: 2026-11-13
- Updated: 2026-11-14

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
selects on such a CPU. AVX2 is upstream's dispatch build since 2026-11-14
(every TU at the baseline x86-64 flags, `HWY_NAMESPACE` code per target,
every target above `HWY_AVX2` disabled; see "AVX2 reference: upstream's
dispatch build"); before, it compiled every TU with Highway's `HWY_AVX2`
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
`hwy::DECODER_SELECTS_SIMD` is on, and then only inside a decoder entry
point (see "Scope of the selection": the encoder stays on the scalar path);
tests switch it like Highway's `SetSupportedTargetsForTest`
(`hwy::set_decoder_simd_for_test`).

The selection is **on for AArch64 (NEON) and for x86-64 (AVX2)** since
2026-11-14 ("Default selection per target", "Default selection on
x86-64"). Working decision 2026-11-13
(delegated, revisable by the owner): it is turned on per target, and only
when on that target

- (a) every decode-path Highway export of the Plan that an in-scope file
  can reach is translated (a file that reaches a scalar kernel while the
  rest runs SIMD matches neither reference build). Upsampling, noise,
  blending, splines, `to_linear`, tone mapping, CMS, `dec_external_image`,
  `dec_modular`, `rct` and `squeeze` were still scalar when this was
  written; for their translation and the state on the combined branch see
  the package sections below and "Public decoder gate", "Combined branch";
- (b) a whole-decode comparison through the public decoder and its
  low-memory pipeline exists and reports zero differences over the
  conformance inputs;
- (c) the scalar gate (`tools/check.py --stage test`) then forces the
  scalar path explicitly, so that it still compares the scalar translation
  with the scalar oracle;
- (d) for AVX2, the same evidence exists from real x86-64 hardware.

The gate is per target (ADR-0041): AArch64 may be turned on before x86-64
when (a) to (c) hold there.
`whole_decodes_with_the_simd_stages_against_the_original_target` is a
parity gate of the simple pipeline of `decode_vardct`: it forces the SIMD
target and asserts zero differing float words and output bytes for every
admitted file, independently of the default selection (it passes on NEON
since the `compressed_dc` translation; see "VarDCT decode kernels").

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

<!-- render stages -->
## Render stages of Plan item 1 (2026-11-13)

The decoder's first lossy stages follow the same pattern as the stages
above: a `Kernel` body generic over `SimdF32` in its own file, dispatched by
the stage factory on `hwy::decoder_target()`, the scalar path unchanged.
The layer gains `LoadDup128`, `Broadcast<lane>` and the binary16 `DemoteTo`
(`fcvtn` through inline assembly on AArch64, because Rust's f16 intrinsic
types are not stable; `vcvtps2ph` on AVX2), each with its `hwy-op` test
(67 operations). Row parity uses the `rs-*` requests of
`tools/oracle/render_stages_simd.cc`, which only call the original's public
factories on 128-byte aligned rows, and `tests/simd_render.rs`.

- **XYB** (`stage_xyb_simd.rs`, `XybToRgb` of dec_xyb-inl.h): biases by
  `LoadDup128` + `Broadcast`, each matrix entry by `LoadDup128` of its four
  stored copies (lane `i` uses copy `i % 4`, observable with custom
  parameters), fused `MulAdd`. NEON: 12,348 rows, 2,628,948 words matched
  (Init at four intensity targets, three custom parameter sets, RGB and XYB
  output, widths 0-67, seven origins, special words).
- **FromLinear** (`stage_from_linear_simd.rs`, with `simd_transfer.rs`:
  EvalRationalPolynomial, FastLog2f/FastPow2f/FastPowf, TF_SRGB (the
  `JXL_HIGH_PRECISION` rational polynomial, not FastLinearToSRGB), TF_PQ,
  TF_HLG, TF_709 and HlgOOTF::Apply, translated operation by operation):
  all six operations. NEON: 21,168 rows, 4,506,768 words matched (linear,
  sRGB, BT.709, PQ at five intensity targets, HLG at five display targets
  and two primaries, DCI and five explicit gammas; each function's
  thresholds with their neighbor words).
- **Output conversion** (`stage_write_simd.rs`, called by
  `WriteToOutputStage` when `GetWriteToOutputStage` dispatched to a SIMD
  target): `MakeUnsigned` (Mul, dither Add, `Clamp(Zero, v, mul)` with its
  swapped operands, `NearestInt`, saturating `DemoteTo`), `StoreFloat16Row`
  (hardware `DemoteTo(df16)`) and `UnpremulAlpha` (`Div(1, Max(kSmallAlpha,
  alpha))`). The interleaving stores, the 16-bit byte swap and
  `StoreFloatRow` only move words and keep the scalar loops. The vector
  loops' lanes past `len` read padding and are never copied to an output,
  so the port computes them on zero padding and drops them. Parity reuses
  the existing `wo-run` observer with the default memory manager, so that
  only output buffers and callback pixels are compared, not scratch blocks.
  NEON: 5,472 runs, 514,728 words matched (u8 at 8 and 5 bits, u16 at 16
  and 10, f16, f32; one to four channels; identity, flips, rotation and
  transpose; widths 1-1030 across the 1024-sample chunk; buffers and
  callbacks; native, little and big endian; alpha with and without
  unpremultiplication; binary16 rounding, subnormal and overflow words).
  The scalar path fails the same requests, so they discriminate. The
  whole-decode comparison does not reach this stage (`decode_vardct` writes
  an ImageBundle and converts through `dec_external_image`), so its counts
  are those of the FromLinear column.

Whole-decode comparison after each stage (NEON, `whole_decodes_with_the_simd_stages_against_the_original_target`,
differing float words with the SIMD stages selected / differing output
bytes; the scalar path is unchanged). The counts barely move: the planes
already differ before these stages, because dequantization and the inverse
DCTs (Plan item 2) still run their `HWY_SCALAR` translation.

| File | Before (Gaborish, EPF, YCbCr, chroma) | + XYB | + FromLinear |
| --- | --- | --- | --- |
| 65x33 gab=0 epf=0 | 6,271 / 5 | 6,256 / 4 | 6,123 / 6 |
| 65x33 gab=0 epf=3 | 6,296 / 1 | 6,290 / 1 | 6,181 / 1 |
| 65x33 gab=1 epf=0 | 6,353 / 2 | 6,327 / 2 | 6,281 / 2 |
| 65x33 gab=1 epf=3 | 6,339 / 4 | 6,335 / 4 | 6,279 / 4 |
| 258x258 gab=0 epf=0 | 198,499 / 261 | 198,403 / 260 | 197,730 / 262 |
| 258x258 gab=0 epf=3 | 198,523 / 269 | 198,502 / 270 | 197,818 / 269 |
| 258x258 gab=1 epf=0 | 198,573 / 257 | 198,530 / 258 | 197,824 / 258 |
| 258x258 gab=1 epf=3 | 198,619 / 301 | 198,571 / 301 | 198,051 / 299 |
| 513x257 gab=0 epf=0 | 393,670 / 580 | 393,733 / 584 | 392,754 / 582 |
| 513x257 gab=0 epf=3 | 393,760 / 665 | 393,656 / 673 | 392,672 / 672 |
| 513x257 gab=1 epf=0 | 393,821 / 641 | 393,768 / 647 | 392,817 / 649 |
| 513x257 gab=1 epf=3 | 393,703 / 699 | 393,812 / 697 | 392,892 / 698 |

(Of 6,435 / 199,692 / 395,523 float words; all 24 files are logged by the
test, these rows are representative. No file reaches zero.)
<!-- end render stages -->

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
- Until 2026-11-14 the AVX2 oracle was a static AVX2 build, not upstream's
  dispatch build: code outside `HWY_NAMESPACE` was also compiled with FMA,
  so Clang contracted scalar `a * b + c` there, which upstream's
  SSE2-baseline dispatch build does not. It is now the dispatch build
  ("AVX2 reference: upstream's dispatch build"); AVX2 results obtained
  against the static build before that date are not evidence for the gate.
- The SIMD stages load and store the pipeline's rows in place, through
  `RowBuffers::spans` (ADR 0003, "SIMD row spans", 2026-11-14), and copy a
  span only where it reaches outside its plane's storage.
  The EPF kernel takes one window of `Lanes + 2 * border` samples per row
  and vector, so its loads index a slice of known length; the loads, their
  order and the arithmetic are the original's. The write stage's SIMD
  `StoreFloatRow` loads the input rows' whole vectors and stores them
  interleaved into the stage's scratch, the same words at the same offsets
  as the original's whole-vector stores; the scratch stays uninitialized
  until written, as the original's (the output-stage captures observe it).

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
- A target beyond NEON/AVX2 enters scope, or an adapter that the AVX2
  variant does not serve is needed by a SIMD test (see "AVX2 reference:
  upstream's dispatch build").

<!-- vardct kernels -->
## VarDCT decode kernels (2026-11-13, Plan item 2)

- **Translated.** `dct_simd.rs` (dct-inl.h, dct_block-inl.h,
  transpose-inl.h), `dec_transforms_simd.rs` (TransformToPixels,
  AFVIDCT4x4, LowestFrequenciesFromDC), `dec_group_simd.rs`
  (DequantLane/DequantBlock with quantizer-inl.h's AdjustQuantBias) and
  `quant_weights_simd.rs` (ComputeQuantTable's GetQuantWeights and
  Interpolate with FastLog2f/FastPow2f/FastPowf and
  EvalRationalPolynomial). `HWY_CAPPED(float, SZ)` with fewer lanes than
  the target (DCT columns 1, 2 and, on AVX2, 4; quant weights' `DF4` on
  AVX2 and the one-lane FastPowf) is computed lane by lane with the
  operation the partial vector performs on that lane (fused `MulAdd`), since
  every operation of these kernels is lane-wise; scratch keeps the
  original's `SZ`-strided layout, so scratch words match too. The scalar
  IDENTITY residual `block_dc - residual_sum * (1.0f / 16)` is fused on both
  targets (FMA in the target attributes). Quant tables became part of this
  package because the original's DequantMatrices are computed by the
  `ComputeQuantTable` export: DequantBlock cannot match with scalar tables
  (one ulp in the first `dq-block` request).
- **Dispatch.** `decode_group_impl` selects `hwy::decoder_target()` once
  per group, like `HWY_DYNAMIC_DISPATCH(DecodeGroupImpl)`; DequantBlock and
  TransformToPixels run the SIMD instantiation, the scalar path is
  unchanged. `DequantMatrices::ensure_computed_in` follows the same target
  (`ensure_computed_for_target`). Working decision 2026-11-13 evening
  (delegated, revisable by the owner; it replaces the earlier rule that kept
  AVX2's DequantBlock scalar): both targets select the SIMD DequantBlock. On
  AVX2, AdjustQuantBias executes the original's vendor-specific estimate
  (`_mm256_rcp_ps` through the layer's `approximate_reciprocal`), so AVX2
  output can differ between CPU vendors exactly as the original's AVX2 build
  does; identity is claimed per host against the original (ADR-0041
  section 2), never across hosts or vendors. NEON's `vrecpe` is
  architecturally defined. The owner's open question O4 (#250) still covers
  whether that is acceptable for shipped defaults; the default selection
  stayed off at the time (for its later state per target see "Default
  selection per target"). No other kernel of this package takes a scalar detour on
  AVX2.
- **Evidence (NEON, native, against `target/oracle-neon`).**
  `tests/simd_vardct.rs`, reusing the scalar suite's `dct-*`, `dq-*` and
  `qac-compute` requests, whose adapters already call `ORACLE_HWY_NS`:
  `dct-scaled` 3,638 requests (every shape 1-32 squared, both directions,
  strides, 24 value modes incl. subnormals, infinities and NaNs; the large
  shapes up to 256x256), 17,445,440 words; `dct-pixels` all 27 strategies
  and the IDENTITY rounding cases, 1,236 requests, 22,110,380 words;
  `dct-llf` all strategies, 1,296 requests, 9,175,392 words; `dq-bias` 168
  requests, 65,856 words; `dq-block` all strategies, both coefficient
  widths, 648 requests, 29,698,008 words; `qac-compute` library and custom
  tables of every mode, 649 requests, 16,588,372 words. No word differs
  (any NaN equals any NaN). `dq-lane` is not used: its adapter calls
  DequantLane at every `k`, so the original's full-vector loads read past
  the request's matrix. AVX2: compiles (`cargo check --target
  x86_64-apple-darwin`); not run locally (no Rosetta 2).
- **Whole decode, NEON, after these kernels** (on top of the render stages
  above; before both: 97.5-99.5 % differing float words). With the SIMD
  path selected, 20 of the 24 VarDCT files match the original's NEON build
  word for word: all 65x33 and 258x258 files and the four 513x257 files with
  Gaborish. The four 513x257 files without Gaborish still differ in 146
  (EPF 0, 1) and 182 (EPF 2, 3) of 395,523 float words, the first at word
  369,961/369,936 (the third color plane); no output byte differs on any
  file (scalar path: 1-696). With these kernels alone, on the earlier main
  without the render stages, 81-84 % of the words differed. The
  `DECODER_SELECTS_SIMD` selection stayed off then because not every file
  matches; the remaining kernels on that path are the next candidates
  (`compressed_dc`'s DC dequantization and adaptive smoothing, upsampling).
- **compressed_dc (2026-11-13).**
  `compressed_dc_simd.rs` translates AdaptiveDCSmoothing (one-lane edge
  pixels computed in lane 0 of a full vector) and DequantDC; the decoder
  dispatches both on `hwy::decoder_target()`. NEON parity
  (`tests/simd_vardct.rs`): `vdc-smooth` 507 requests / 94,796 words and
  `vdc-dequant` 384 requests / 200,664 words match (DequantDC
  columns past the rectangle width up to the next lane multiple are excluded:
  the original stores them from quantized samples it never initialized).
  With it, **all 24 admitted VarDCT files of the whole-decode comparison
  match the NEON oracle word for word** and no output byte differs. The
  selection was not turned on then: this ADR requires the comparison on every
  target in scope, AVX2 is not verified (hosted run pending), and the one `DECODER_SELECTS_SIMD` switch would
  also move the AArch64 decoder off the scalar oracle that the scalar gate
  compares with. The whole-decode test now asserts these zero differences
  with the SIMD target forced; the conditions for the default selection are
  in "Decoder path".
- **First native AVX2 run (2026-11-13, hosted Linux x86-64, run
  43502660563).** The SIMD oracle crashed (SIGSEGV) in `dct-pixels`,
  `dct-llf`, `dq-block` and `vdc-dequant`: the adapters handed the original
  buffers that are not shaped like its callers'. The aligned moves of the
  x86-64 AVX2 objects (`objdump`, inline frames by `llvm-dwarfdump
  --lookup`) are CoeffBundle/DCT1DImpl's `Load`/`Store` on the transform
  `tmp` (`dct-inl.h` 53-174; scratch, aligned `scratch_space` in libjxl),
  DequantLane's `Load(di, qblock)` and `Store(block)` (`dec_group.cc`
  136-138, 150-152; aligned coefficient storage and `dec_group_block`), and
  DequantDC's `Store` at `dc row + r.x0()` (`compressed_dc.cc` 292-294,
  311; DC-group rectangles start at multiples of 256). `dct.cc` and
  `dequant.cc` now allocate their owners with `hwy::AllocateAligned`
  (`aligned_vector.h`), and `vdc-dequant` requests start their rectangles at
  x0 0 or 256; observed words are unchanged. `dct-scaled`, `dct-llf` share
  `dct.cc`; `dq-bias` uses no memory; `qac-compute` and `vdc-smooth` use
  libjxl's own owners (both passed on AVX2).
- **Whole decode on AVX2 differed** in 1-5 % of the float words of the 16
  larger files (the eight 65x33 files matched). Found by reading the code:
  DecodeGroupImpl computes `YtoXRatio`/`YtoBRatio`
  (`base + x_factor * color_scale`, chroma_from_luma.h) inside
  `HWY_NAMESPACE`, so the AVX2 instantiation contracts it with FMA (as the
  NEON one does); the port used the x86 scalar build's unfused form on the
  SIMD path. The SIMD path now fuses it on both targets
  (`y_to_x_ratio_simd`). It matters only where a tile's CfL factor is not
  zero. AVX2 evidence is pending. Remaining risk on that gate: the AVX2
  oracle was a static AVX2 build, so it also contracted scalar code outside
  `HWY_NAMESPACE` (for example RecomputeDCFactors), which upstream's dispatch
  build and the port do not (see "Consequences"; replaced by the dispatch
  build on 2026-11-14, whose `N_AVX2::DecodeGroupImpl` still fuses the CfL
  ratios).
- **8-lane structure, native.** `tests/simd_vardct_lanes8.rs` runs the
  generic kernels with an 8-lane descriptor of two NEON halves (lane-wise
  NEON semantics) against NEON's 4 lanes: transforms (all shapes and
  strategies, LLF) 1,120 cases, DequantBlock 162, DC smoothing, DequantDC
  rows and quant weights 929; equal words (NaN-equal). That covers the
  8-lane loop bounds, `Part<4>` transforms, one-lane ranges and tails; it
  does not test AVX2's own instructions.
<!-- end vardct kernels -->

<!-- public gate -->
## Public decoder gate (2026-11-13, "Decoder path" conditions (b) and (c))

- **Comparison.** `tests/simd_public.rs`
  `public_decodes_with_the_simd_target_against_the_original_target` decodes
  each input through the port's public decoder (`JxlDecoder` API, the
  low-memory render pipeline the Ferrisma engine reaches through
  `crates/ferrisma-jxl-decoder`) with the SIMD target forced
  (`set_decoder_simd_for_test(Some(true))`) and through the original's
  public decoder in the single-target oracle (`dps-decode` of
  `tools/oracle/decoder_public_simd.cc`, which calls only the public API).
  Both run the engine's sequence (whole input, closed input, no runner, no
  CMS, default coalescing and orientation) plus frame headers and every
  extra channel buffer, for float, 8-bit and 16-bit output. Status, basic
  and extra channel info, both color profiles (ICC bytes and encoding) and
  frame headers must be equal; every output word is compared (any NaN equals
  any NaN). Inputs: the 24 VarDCT files and the 4:2:0 JPEG recompression of
  the `decode_vardct` gate, the scalar suite's public-decoder inputs (its
  encoder specs, decode_test.cc codestreams and frame sequences, all
  retained `.jxl` files in `tests/data`, the capture files) and, when
  `JXL_CONFORMANCE_CORPUS` names the `corpus` directory of
  `tools/conformance/corpus.py fetch` (docs/CONFORMANCE.md), the pinned
  conformance inputs; identical inputs are compared once.
- **Punch list in executable form.** `STILL_DIFFERS` names every file that
  still differs with its reason. The test asserts zero differences for every
  other file, that every listed file still differs, and fails with a
  request to remove an entry (here and in the test) when a listed file
  starts to match. `tools/check.py` runs the binary with the other SIMD
  tests; it does not set `JXL_CONFORMANCE_CORPUS`, so the corpus entries are
  checked only where the corpus has been fetched.
- **Port change found by the gate.** In the low-memory pipeline the group
  input rows start `kRenderPipelineXOffset - group_data_x_border` samples
  into their plane row, and the SIMD EPF reads `RoundUpTo(xextra_left,
  Lanes) + border` samples to the left: at `xextra_left = 1` that is before
  the row (258x258, EPF 2, the last group column), where the original reads
  the preceding row's padding. `simd_rows::read` panicked there; it now
  returns 0.0 for samples outside the row slice. Those samples only reach
  lanes left of `-xextra_left`, which no later stage or output uses; the
  scalar path is unchanged. Since 2026-11-14 the stages read such a span in
  place from the preceding row's padding, as the original, and copy it only
  on a plane's first row.
- **How the reasons were found.** `jxl::jxl::decode_trace` (test
  observation only, inactive unless a test starts it) records the frame
  header (encoding, flags, upsampling, extra channel upsampling, blending),
  the stage names of every pipeline the decoder finalizes and the modular
  inverse transforms; the test prints them for every differing file. Every
  differing file reaches `Upsample`, `ConvNoise`/`AddNoise` or `Splines`,
  whose exports (`stage_upsampling`, `stage_noise` with `dec_noise`,
  `stage_splines` with `splines`) are still the scalar translation; no file
  differs without one of them. The modular transforms these files also run
  (RCT, palette, squeeze; integer arithmetic) are not the explanation as
  far as this set shows: the extra-channel files of the same series without
  upsampling (`extra-channel-alpha-*-up-1-*`) match, as do the lossless
  conformance cases (`delta_palette`, `lossless_pfm`, `patches_lossless`);
  their transforms were not traced, as the trace is printed for differing
  files only. `Spot` and `Patches` have no Highway export in the original;
  `Blending` is one (`GetBlendingStage`), but its rows go through
  `PerformBlending` of `blending.cc`, which has none, and the blending
  inputs (`blendmodes`, the animations without splines, the layered
  `dpub` specs) match. No differing file reaches `ToLinear`, `ToneMapping` or `Cms`
  (no CMS is set, as in the engine); this set therefore does not test
  them.

Results, NEON (native, against `target/oracle-neon`, 2026-11-13; corpus
fetched at the pinned revision, inputs verified against
`tools/conformance/MANIFEST.sha256`): 352 distinct inputs, 276 match
word for word in all three formats, 76 still differ, all as listed.
Differing words are "differing / compared" per format. AVX2 was not run
(no Rosetta 2 here); the list is the NEON result and a hosted x86-64 run
may need its own entries.

| Input group | Files | Match (0 words in f32, u8 and u16) | Still differ |
| --- | ---: | ---: | ---: |
| `decode_vardct` gate files (24 VarDCT, JPEG 4:2:0) | 25 | 25 | 0 |
| public-decoder encoder specs (`dpub`) | 38 | 35 | 3 |
| decode_test.cc codestreams and frame sequences (`dti`) | 17 | 17 | 0 |
| retained files in tests/data | 244 | 174 | 70 |
| retained public-decoder captures | 1 | 1 | 0 |
| conformance corpus (pinned, 40 inputs, 27 distinct after `_5` links) | 27 | 24 | 3 |

| File | f32 | u8 | u16 | Reaches (still scalar) |
| --- | ---: | ---: | ---: | --- |
| `dpub w=64 h=48 c=3 lossless=0 opt=2:2` | 2489 / 9216 | 0 / 9216 | 28 / 9216 | **upsampling**: EPF1 EPF2 Upsample Upsample Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 2, ec [] |
| `dpub w=300 h=280 c=3 a=1 bits=2 opt=26:0 opt=3:4` | 27084 / 420000 | 0 / 420000 | 84 / 420000 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular palette |
| `dpub w=300 h=280 c=3 a=1 effort=3 opt=26:0 opt=3:4` | 52568 / 420000 | 1 / 420000 | 136 / 420000 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular RCT, palette |
| `data/decoder-geometry/sequence-extra-alpha-xyb-0-1-up-2-shift-1.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2] |
| `data/decoder-geometry/sequence-extra-xyb-1-up-2-4.jxl` | 109 / 441 | 0 / 441 | 0 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 2, ec [4, 4, 4]; modular squeeze |
| `data/decoder-geometry/sequence-resampling-alpha-0-2-2.jxl` | 186 / 765 | 0 / 765 | 1 / 765 | **upsampling**: Upsample Upsample Upsample Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 2, ec [2] |
| `data/decoder-geometry/sequence-resampling-alpha-1-4-4.jxl` | 236 / 765 | 0 / 765 | 2 / 765 | **upsampling**: Upsample Upsample Upsample Upsample WritePixelCB; flags 0x0, upsampling 4, ec [4]; modular palette |
| `data/decoder-geometry/sequence-resampling-main-0-2.jxl` | 127 / 459 | 0 / 459 | 4 / 459 | **upsampling**: Upsample Upsample Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 2, ec [] |
| `data/decoder-geometry/sequence-resampling-main-1-4.jxl` | 136 / 459 | 0 / 459 | 1 / 459 | **upsampling**: Upsample Upsample Upsample WritePixelCB; flags 0x0, upsampling 4, ec []; modular palette |
| `data/effects_gray_alpha16.jxl` | 18 / 72 | 0 / 72 | 0 / 72 | **noise and splines**: Gab EPF0 EPF1 EPF2 Splines ConvNoise AddNoise XYB FromLinear WritePixelCB; flags 0x11, upsampling 1, ec [1] |
| `data/effects_rgb_alpha8.jxl` | 56 / 120 | 0 / 120 | 1 / 120 | **noise and splines**: Gab EPF0 EPF1 EPF2 Splines ConvNoise AddNoise XYB FromLinear WritePixelCB; flags 0x11, upsampling 1, ec [1] |
| `data/extra-channel-alpha-0-up-2-shift-0.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular palette |
| `data/extra-channel-alpha-0-up-2-shift-1.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular palette |
| `data/extra-channel-alpha-0-up-4-shift-0.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular palette |
| `data/extra-channel-alpha-0-up-4-shift-1.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular palette |
| `data/extra-channel-alpha-0-up-4-shift-2.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular palette |
| `data/extra-channel-alpha-0-up-8-shift-0.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-0-up-8-shift-1.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-0-up-8-shift-2.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-0-up-8-shift-3.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-1-up-2-shift-0.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular palette |
| `data/extra-channel-alpha-1-up-2-shift-1.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular palette |
| `data/extra-channel-alpha-1-up-4-shift-0.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular palette |
| `data/extra-channel-alpha-1-up-4-shift-1.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular palette |
| `data/extra-channel-alpha-1-up-4-shift-2.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular palette |
| `data/extra-channel-alpha-1-up-8-shift-0.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-1-up-8-shift-1.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-1-up-8-shift-2.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-1-up-8-shift-3.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular palette |
| `data/extra-channel-alpha-xyb-0-0-up-2-shift-0.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2] |
| `data/extra-channel-alpha-xyb-0-0-up-2-shift-1.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2] |
| `data/extra-channel-alpha-xyb-0-0-up-4-shift-0.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4] |
| `data/extra-channel-alpha-xyb-0-0-up-4-shift-1.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4] |
| `data/extra-channel-alpha-xyb-0-0-up-4-shift-2.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4] |
| `data/extra-channel-alpha-xyb-0-0-up-8-shift-0.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-0-0-up-8-shift-1.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-0-0-up-8-shift-2.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-0-0-up-8-shift-3.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-0-1-up-2-shift-0.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2] |
| `data/extra-channel-alpha-xyb-0-1-up-4-shift-0.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4] |
| `data/extra-channel-alpha-xyb-0-1-up-4-shift-1.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4] |
| `data/extra-channel-alpha-xyb-0-1-up-4-shift-2.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4] |
| `data/extra-channel-alpha-xyb-0-1-up-8-shift-0.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-0-1-up-8-shift-1.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-0-1-up-8-shift-2.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-0-1-up-8-shift-3.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8] |
| `data/extra-channel-alpha-xyb-1-0-up-2-shift-0.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-2-shift-1.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-4-shift-0.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-4-shift-1.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-4-shift-2.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-8-shift-0.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-8-shift-1.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-8-shift-2.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-0-up-8-shift-3.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-2-shift-0.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-2-shift-1.jxl` | 34 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [2]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-4-shift-0.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-4-shift-1.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-4-shift-2.jxl` | 48 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [4]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-8-shift-0.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-8-shift-1.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-8-shift-2.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-alpha-xyb-1-1-up-8-shift-3.jxl` | 12 / 315 | 0 / 315 | 0 / 315 | **upsampling**: Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 1, ec [8]; modular squeeze |
| `data/extra-channel-xyb-0-up-2-2.jxl` | 124 / 441 | 0 / 441 | 1 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 2, ec [2, 2, 2]; modular palette |
| `data/extra-channel-xyb-0-up-2-4.jxl` | 105 / 441 | 0 / 441 | 0 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 2, ec [4, 4, 4]; modular palette |
| `data/extra-channel-xyb-0-up-4-4.jxl` | 112 / 441 | 0 / 441 | 3 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 4, ec [4, 4, 4]; modular palette |
| `data/extra-channel-xyb-0-up-4-8.jxl` | 87 / 441 | 0 / 441 | 0 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 4, ec [8, 8, 8]; modular palette |
| `data/extra-channel-xyb-0-up-8-8.jxl` | 58 / 441 | 0 / 441 | 1 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 8, ec [8, 8, 8]; modular palette |
| `data/extra-channel-xyb-1-up-2-2.jxl` | 129 / 441 | 0 / 441 | 0 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 2, ec [2, 2, 2]; modular squeeze |
| `data/extra-channel-xyb-1-up-4-4.jxl` | 104 / 441 | 0 / 441 | 0 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 4, ec [4, 4, 4]; modular squeeze |
| `data/extra-channel-xyb-1-up-4-8.jxl` | 80 / 441 | 0 / 441 | 1 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 4, ec [8, 8, 8]; modular squeeze |
| `data/extra-channel-xyb-1-up-8-8.jxl` | 56 / 441 | 0 / 441 | 0 / 441 | **upsampling**: Upsample Upsample Upsample Upsample Upsample Upsample XYB Spot FromLinear WritePixelCB; flags 0x0, upsampling 8, ec [8, 8, 8]; modular squeeze |
| `conformance/animation_spline` | 191374 / 18432000 | 2 / 18432000 | 1406 / 18432000 | **splines**: Gab EPF1 Splines XYB FromLinear WritePixelCB; flags 0x10, upsampling 1, ec [] |
| `conformance/noise` | 185761 / 909000 | 11 / 909000 | 2610 / 909000 | **noise**: Gab EPF1 ConvNoise AddNoise XYB FromLinear WritePixelCB; flags 0x1, upsampling 1, ec [] |
| `conformance/upsampling` | 380366 / 2400000 | 23 / 2400000 | 5992 / 2400000 | **upsampling**: Gab EPF1 Upsample Upsample Upsample Upsample XYB FromLinear WritePixelCB; flags 0x0, upsampling 4, ec [4]; modular squeeze |

The selection stayed off then: condition (b) had its comparison, which did
not report zero yet; the punch list is empty only when upsampling, noise
and splines are translated (and then on each target separately).

- **Combined branch (2026-11-14, `v1/jxl-merge6`: this gate, the VarDCT
  fix, the modular kernels, the upsampling/noise/splines stages and the
  color stages).** NEON, with the corpus: all 352 distinct inputs match the
  original word for word in f32, u8 and u16, so the 76 entries above were
  removed and `STILL_DIFFERS` is empty. The gate gained a second decode
  mode: the same sequence with the default CMS set and a Rec. 2020 PQ
  output profile requested at the color encoding event (float only; the
  oracle's `dps-decode <type> 1`). All 352 match there as well. The decode
  trace counts the color package's stages: the `Cms` stage is reached in
  340 of the 352 CMS-mode decodes (and in none of the engine-mode ones);
  `ToLinear` and `ToneMapping` are reached by no input. They are added only
  for tone mapping, which needs a PQ or HLG original with a desired
  intensity target below the original's; no input of this set is HDR, so
  conditions (a) and (b) are exercised for `Cms` but not yet for
  `ToLinear` and `ToneMapping` (their row-level parity is
  `tests/simd_color.rs`). AVX2 has not been run.
- **Condition (c).** `hwy::decoder_target()` is `decoder_target_for(
  DECODER_SELECTS_SIMD)`. In the oracle test builds (feature `cpp-oracle`)
  `JXL_DECODER_TARGET=scalar` forces the scalar path whatever the default
  is (`hwy::scalar_forced_by_environment`, read once); an explicit
  `set_decoder_simd_for_test` still wins, so the SIMD binaries keep forcing
  their target. `tools/check.py`'s test stage sets the variable for all
  test invocations. `tests/simd_public.rs`
  `the_scalar_gate_forces_the_scalar_decoder_path` asserts that the
  variable is set whenever the oracle environment of `check.py` is, that
  `decoder_target_for(true)` is the scalar path then (as if
  `DECODER_SELECTS_SIMD` were on), that plane padding follows it
  (`max_vector_size() == 4`), and that the explicit override still selects
  either path. Ordinary builds never read the environment.
<!-- end public gate -->

<!-- modular -->
## Modular and output-conversion kernels (2026-11-13, Plan item 2)

Branch `v1/jxl-modular`. Each export is a `Kernel` body generic over
`SimdF32` in its own file and dispatched on `hwy::decoder_target()` where
the original calls `HWY_DYNAMIC_DISPATCH`; the scalar path is unchanged.
Parity: `tests/simd_modular.rs` with the requests of
`tools/oracle/modular_simd.cc` (128-byte aligned buffers; the TU calls the
original's exported `ORACLE_HWY_NS` functions).

- **dec_external_image** (`dec_external_image_simd.rs`): `FloatToU32`
  (`Clamp(v, Zero, one)`, `NearestInt(Mul(v, scale))`, the bits stored as
  `uint32_t`) and `FloatToF16` (hardware `DemoteTo`), both over whole
  vectors to `RoundUpTo(num, Lanes)` as the original; where the caller's
  slice ends earlier the missing input lanes read zero and the missing
  output lanes are dropped (lane-wise operations). `ConvertChannelsToExternal`
  selects them, so `decode_vardct`'s integer output now runs the SIMD
  conversion. NEON: `mx-u32` 1,184 requests / 58,816 words (bits 1-16, the
  special words, every scale's rounding ties and neighbors, lengths 0-67,
  the whole-vector tail and the sentinel words after it), `mx-f16` 80
  requests / 3,920 words (binary16 rounding, subnormal and overflow words),
  and through the decoder's dispatch the scalar suite's `ue-channels`,
  `fe-channels` and `fe-row` requests at widths 1-33 (11,692 requests,
  35,076 words: status and every byte of the 1,024-byte buffer). The scalar
  path fails 208 `mx-u32` requests below `num` (a signaling NaN: scalar
  `Clamp` keeps 0, NEON's `fmaxnm` quiets it and `fminnm` returns 1; on AVX2
  `maxps`/`minps` give 1 for every NaN) and the `ue-channels` replay, so
  the requests discriminate. AVX2: compiles; not run locally (no Rosetta 2).
- **Whole decode after dec_external_image (NEON).** Rerun of
  `whole_decodes_with_the_simd_stages_against_the_original_target` with the
  SIMD target forced: all 24 admitted VarDCT files still have zero differing
  float words and zero differing output bytes (scalar path: 1-696 bytes).
  The integer output already matched before this change because the decoded
  samples of these files are finite: the SIMD and scalar `FloatToU32` differ
  only for NaN input (above) and in the lanes past `num`, which the
  conversion never copies out.
- **Signed 32-bit operations.** The layer gains, in its `modular` block,
  `Set`, `Load`, `And`, `Xor`, `Abs`, `Neg`, `Ne` (as `Not(Eq)`),
  `IfThenElse`, `IfThenZeroElse`, `MulEven`, `MulOdd` (the 64-bit products
  bit-cast to 32-bit lanes) and `InterleaveOdd` on `int32` lanes, each the
  instruction sequence Highway uses (`vmull_s32` after `vuzp1/2`, `vtrn2`,
  `vbsl`/`vbic`; `_mm256_mul_epi32` with `DupOdd` for `MulOdd`, the two
  `shuffle_ps` of `InterleaveOdd` below AVX3, `blendv_epi8`, `andnot`). The
  `hwy-op` test covers all 79 operations: 103,389 requests matched on NEON.
- **dec_modular** (`dec_modular_simd.rs`): `MultiplySum` (wrapping
  `Add` of the two `int32` rows), `RgbFromSingle` and `SingleFromSingle`
  (`ConvertTo`, `Mul`), over whole vectors as the original;
  `ModularImageToDecodedRect` selects them at its four
  `HWY_DYNAMIC_DISPATCH` sites. `SingleFromSingleAccurate` and
  `int_to_float` are not Highway code. NEON: 2,448 requests / 183,840
  words (lengths 0-67, twelve factors including the decoder's `1/255`,
  `1/65535`, `1/(2^22-1)`, negative, subnormal, maximal, infinite and NaN
  factors; samples at the 32-bit limits, so that `Add` wraps, and around
  2^24, where the conversion rounds). Below `xsize` the SIMD and scalar
  results agree by construction (one exact-or-rounded conversion and one
  product per sample); the scalar path fails 1,836 requests in the
  whole-vector tail the original writes, which is what discriminates.
- **rct** (`modular/transform/rct_simd.rs`): `InvRCTRow` for all seven
  transform kinds, the vector loop while `x + N - 1 < w` and the source's
  scalar `PixelAdd` tail; `InvRCT` takes it on `hwy::decoder_target()`
  (permute-only types keep the shared channel moves). NEON: `mx-rct`, all
  42 RCT types at widths 1-67 with small and extreme samples, 840 requests
  / 75,516 words (status, channel order, every sample). The original's
  vector and scalar forms compute the same wrapping 32-bit sums, so the
  output cannot tell the SIMD path from the scalar one; the requests cover
  widths that run both the vector loop and the tail on each target.
- **squeeze** (`modular/transform/squeeze_simd.rs`): the non-scalar
  `InvHSqueeze`/`InvVSqueeze` with `FastUnsqueeze` (32-bit wrapping
  lanes, the division by three as `MulEven`/`MulOdd` + `InterleaveOdd`),
  the horizontal path's 8x8 transposes and `StoreInterleaved` as plain
  word copies (data movement only, as `store_interleaved3`), the source's
  64-bit scalar tails; `inv_squeeze` dispatches through
  `inv_squeeze_for_target(hwy::decoder_target(), ...)`. NEON: `mx-squeeze`
  with the default parameter list, one horizontal, one vertical and a
  horizontal-then-vertical step, 34 sizes from 1x1 to 131x32 (horizontal
  blocks need a residual wider than 16 and eight rows), small and extreme
  samples: 272 requests / 180,096 words. The scalar path fails 78 of them
  (it computes the tendency in 64 bits where `FastUnsqueeze` wraps).
- **convolve_symmetric5 and simd_util: no decode path reaches them.**
  `Symmetric5` is called only by the encoder (`enc_gaborish.cc`,
  `enc_adaptive_quantization.cc`) and `MaxValue` only by `enc_ans.cc`.
  `MaxVectorSize`, which the decoder's image and memory-manager row padding
  use, is already the port's `hwy::max_vector_size()`. Both remain encoder
  work (Plan item 3).
- **Decoder path (a), status after this block.** Of the Plan item 2
  exports, `dec_external_image`, `dec_modular`, `rct` and `squeeze` are now
  translated and dispatched; the list of scalar kernels under "Decoder
  path" (a) shrinks accordingly (the render-pipeline stages there belong to
  other packages). AVX2 of all four compiles and has not been run.
<!-- end modular -->

<!-- stages2 -->
## Upsampling, noise, blending and splines stages (2026-11-13, Plan item 1)

Each stage is a `Kernel` body generic over `SimdF32` in its own file,
dispatched by the stage factory on `hwy::decoder_target()`; the scalar path
is unchanged. Parity uses the `s2-*` requests of
`tools/oracle/render_stages2_simd.cc` (the original's public factories and
pipelines only) and `tests/simd_stages2.rs`.

- **Upsampling** (`stage_upsampling_simd.rs`, 2x/4x/8x with the symmetric
  kernel expansion, the 5x5 min/max preprocessing over the per-thread
  scratch, the three-accumulator `Mul`/fused `MulAdd` sum,
  `Clamp(result, mins, maxs)` and simd_util-inl.h's `StoreInterleaved`,
  which only permutes lanes into `v0[0], v1[0], ...` order). The stage reads
  and writes whole vectors as the original does: column extrema to
  `RoundUpTo(len + 4, Lanes)`, output to `RoundUpTo(len, Lanes) * N`
  samples per row. Comparison rule: on AVX2, when `len % 8` of the last
  1024-sample chunk is 1 to 4, the extrema lanes from `RoundUpTo(len, 8) - 4`
  on read scratch the chunk never wrote (uninitialized after
  `PrepareForThreads`); the output samples they produce lie past
  `xsize * N` and are excluded. No NEON word is excluded. Row requests
  (`s2-ups`): every shift, the default, image-like, special-word and random
  weight sets, widths 0-67 and 1023-2050 across the chunk boundary, special,
  image-like and random samples in the two border rows and columns and past
  the row (the stage's `Settings::Symmetric(shift, 2)` border), every output
  sample including the over-written vector tail. Pipeline requests
  (`s2-ups-pipe`): the stage in the simple and the low-memory pipeline,
  which mirror the two-sample border at every image edge, eight image sizes
  from 1x1 to 1030x2, every shift and channel. NEON: `s2-ups` 1,104
  requests, 5,643,424 words; `s2-ups-pipe` 456 requests, 348,108 words; no
  word differs. The scalar path fails both (first differing word 28 / 8).
  AVX2: compiles; not run locally.
- **Noise** (`stage_noise_simd.rs`, `dec_noise_simd.rs`). AddNoise with the
  non-`HWY_SCALAR` StrengthEvalLut: the LUT as two 16-bit byte tables
  (`LoadDup128`), indices built with 32-bit `Mul`/`Add` and read with
  `TableLookupBytes`, then `And`/`Or` and the fused interpolation
  (`HWY_CAPPED(float, 8)` is the full vector on both targets);
  ConvolveNoise from `-xextra_left` in whole vectors; `Random3Planes` with
  the same Xorshift128Plus words as the scalar path (its vector `Fill` is
  integer and lane-wise) and `BitsToFloat` storing whole vectors, so the
  last span of each row is written to `RoundUpTo(remaining, Lanes)`. The
  layer gains `Mul` of 32-bit lanes (`vmulq_s32`, `vpmulld`) and
  `TableLookupBytes` (`vqtbl1q_u8`, `vpshufb`), each with its `hwy-op` test
  (69 operations, 2,145 requests each; the operands include indices of 16
  and more and with bit 7 set). NEON: `s2-noise-add` 10,080 requests,
  3,878,784 words (six LUTs including none, values past one, negative and
  special words; five base correlations decoded through `DecodeDC`; LUT
  knot neighbors; widths 0-67, five origins; every sample of every row,
  including the vector tail); `s2-noise-conv` 2,880 requests, 537,480 words
  (`xextra` 0-8 on each side, the two border rows and columns of
  `Settings::SymmetricBorderOnly(2)` filled with special words);
  `s2-noise-random` 153 requests, 126,261 words (widths 0-100 across the
  16-sample batch, three seeds, rectangles inside larger planes so the
  vector tail past the rectangle is observed). No word differs; the scalar
  path fails all three. AVX2: compiles; not run locally.
- **Blending** (no translation needed). `GetBlendingStage` is a Highway
  export, but the per-target `BlendingStage` contains no Highway operation:
  it clips the span to the canvas and calls `PerformBlending`
  (blending.cc, alpha.cc), which is compiled once outside `HWY_NAMESPACE`.
  The port therefore uses its one stage on every target. Evidence: the
  scalar suite's `blendstage-row`/`blendstage-simple` request families
  (tests/parity/blending_stage.rs, included unchanged) replayed against the
  NEON oracle with the SIMD target selected: 3,891 requests, 1,433,730
  words, no difference. Because the stage has no SIMD arithmetic, the
  scalar path passes the same replay; the replay shows that the NEON
  build's blending equals the port, not that a SIMD body was translated.
  The static AVX2 oracle compiled blending.cc with FMA (see
  "Consequences"), so an AVX2 comparison of `PerformBlending` against that
  oracle was not meaningful for an SSE2-baseline dispatch build; the AVX2
  oracle is the dispatch build since 2026-11-14.
- **Splines** (`splines_simd.rs`). `SegmentsFromPoints` with
  `ContinuousIDCT` (`HWY_CAPPED(float, 32)` is the full vector: 8 or 4
  iterations, fused `MulAdd`, `FastCosf`, then `SumOfLanes` in the
  target's order) and `DrawSegments` with both `DrawSegment` overloads
  (whole vectors while they fit in the span, `Iota` as the int32
  conversion of `x + x0` plus the lane index, fused `MulAdd`/`MulSub`,
  `Sqrt`, `FastErff` with `Div` and `NegMulAdd`; the `HWY_CAPPED(float, 1)`
  tail is computed in lane 0 of a full vector, since every operation is
  lane-wise, and only that sample is loaded and stored). `ComputeSegments`
  is scalar code in `HWY_NAMESPACE` without a contraction candidate and is
  shared with the scalar path. `kPi`-based constants keep the source's
  double expressions. The stage (`stage_splines`) has no Highway operation
  of its own and serves every target. `Splines::initialize_draw_cache`
  and `apply_to_row` dispatch on `hwy::decoder_target()`. Evidence: the
  scalar suite's spline request families (tests/parity/splines.rs, included
  unchanged: `sp-cache` draws curves through `InitializeDrawCache`, `AddTo`,
  `SubtractFrom`, a sub-rectangle and the stage with extended rows) replayed
  against the NEON oracle with the SIMD target selected: 6,253 requests,
  2,423,179 words, no difference; the scalar path fails `sp-cache` (first
  difference one ulp). AVX2: compiles; not run locally.

With these four, the remaining decode-path exports of Plan item 1 that
this package covers are translated; condition (a) of "Decoder path" still
waits for the other agents' exports (`to_linear`, tone mapping, CMS,
`dec_external_image`, `dec_modular`, `rct`, `squeeze`) and (b) to (d).
<!-- end stages2 -->

<!-- color -->
## Color stages of Plan item 1 (2026-11-13)

Row parity uses the `cl-*` requests of `tools/oracle/color_stages_simd.cc`
(only the original's public factories, on 128-byte aligned rows sized for
eight lanes) and `tests/simd_color.rs`. The test reaches the port through
the public factories with `hwy::set_decoder_simd_for_test`, and runs every
request through the scalar path as well: a family passes only when the SIMD
path matches every word and the scalar path differs in at least one request.

- **ToLinear** (`stage_to_linear_simd.rs`, with the decode direction
  `DisplayFromEncoded` of TF_SRGB, TF_PQ, TF_HLG and TF_709 appended to
  `simd_transfer.rs`): all seven operations, including OpHlg's
  `HlgOOTF::FromSceneLight(orig_intensity_target)` after the HLG decode and
  the factory's initialized OpInvalid identity. `GetToLinearStage`
  dispatches on `hwy::decoder_target()`. NEON: 22,050 rows, 7,340,550 words
  matched (linear, sRGB, BT.709, an unknown transfer function, PQ at five
  intensity targets, HLG at five original intensity targets and two
  primaries, DCI and five explicit gammas; widths 0-67, seven origins,
  thresholds with their neighbor words, special words); the scalar path
  differs in 15,304 of them.
- **Tone mapping** (`stage_tone_mapping_simd.rs`, with `Rec2408ToneMapper`
  and `GamutMap` of cms/tone_mapping-inl.h appended to `simd_transfer.rs`;
  `HlgOOTF::Apply` was already there): the stage keeps the original's
  constructor selection, the 10000-nit rescaling for a PQ output and the
  factory's null result. The base constructors (`Rec2408ToneMapperBase`,
  `HlgOOTF_Base`) are the existing scalar translations; on AArch64 they
  contract like the original. `GetToneMappingStage` dispatches on
  `hwy::decoder_target()`. NEON: 58,216 requests, 19,379,056 words matched
  (PQ to six lower targets and HLG at five display targets, each to PQ,
  sRGB and linear output with two primaries' luminances, plus four
  encodings that need no stage); the scalar path differs in 45,222.
  AVX2 caveat: the original's AVX2 objects may contract the scalar base
  constructors (`ks_ = 1.5f * max_lum_ - 0.5f`, the double PQ curve of
  `InvEOTF`) where the port's x86-64 scalar translation does not, because
  they are inlined into `HWY_ATTR` code with FMA; only a run against the
  AVX2 oracle can show it.
- **CMS: the decode-reachable Highway exports.** `stage_cms.cc` exports
  `GetCmsStage`, whose stage contains no Highway operation (its
  interleaving loops and `memcpy`s are scalar code compiled per target), so
  its SIMD instantiation is the existing translation and the factory needs
  no dispatch. `cms/jxl_cms.cc` has one export, `DoColorSpaceTransform`,
  which the decoder reaches as the default CMS's `run` callback
  (`ColorSpaceTransform::Run` from the CMS stage and the other callers of
  the default CMS). It includes only `transfer_functions-inl.h`. Of its
  body, only these branches execute vector operations: `BeforeTransform`
  kPQ (`TF_PQ::DisplayFromEncoded`) and kSRGB (`TF_SRGB::DisplayFromEncoded`),
  `AfterTransform` kPQ (`TF_PQ::EncodedFromDisplay`) and kSRGB
  (`TF_SRGB::EncodedFromDisplay`), each over whole vectors with `Load`/
  `Store` and one `LoadN`/`StoreN` tail. Scalar code in the export: the kHLG
  branches (`TF_HLG_Base` in double; `ApplyHlgOotf` lies outside
  `HWY_NAMESPACE`), the gray expansion and contraction around skcms, and
  the skcms call (the skcms engines are separate translations, not part of
  this export; the lcms2 branch is not built). `FastLinearToSRGB` is not
  reached (`JXL_HIGH_PRECISION`).
- **CMS: translated.** `cms/jxl_cms_simd.rs` holds the four vector branches
  with the transfer functions of `simd_transfer.rs` (now `pub(crate)`); the
  tail is computed in a zeroed full vector and only its `tail` lanes are
  written back, which is what `LoadN`/`StoreN` observe for lane-wise
  operations. The default CMS's `run` selects `hwy::decoder_target()` once
  per call, like `HWY_DYNAMIC_DISPATCH(DoColorSpaceTransform)`, and keeps
  the scalar HLG branches. NEON (`cl-cms`, the original's `GetCmsStage`
  with `JxlGetDefaultCms` against the port's stage with its default CMS):
  54,265 requests, 16,524,985 words matched (PQ, sRGB and HLG to and from
  linear in sRGB, BT.2100 and P3 primaries, both directions in one call,
  gray, four pairs across color spaces through skcms, three intensity
  targets, widths 1-67 for every interleaved tail); the scalar path
  differs in 30,481. The cross-space requests run the port's skcms engine
  for AArch64 against the oracle's; on x86-64 they would depend on which
  skcms engine runs (open question O9), and engine selection is unchanged.
- **Decoder path (a), color part.** With these, `to_linear`, tone mapping
  and CMS of the list in "Decoder path" (a) have SIMD translations on both
  targets; NEON row parity holds, AVX2 is compile-checked only.
- **AVX2 CMS contraction (2026-11-14).** Hosted
  run 43544974125 (dispatch-build AVX2 oracle) failed one `cl-cms` request
  with an HLG destination by one ulp (word 295, 1065407993 vs 1065407992).
  `objdump -d` of upstream's `jxl_cms.cc` compiled for x86-64 with
  upstream's own flags (the cross-compiled dispatch object of the AVX2
  oracle work): of all functions in that object only
  `N_AVX2::DoColorSpaceTransform` contains FMAs, 82 packed (the Highway
  `MulAdd`s of the PQ/sRGB branches, already fused in the port) and 4
  scalar. The 4 scalar ones are `AfterTransform`'s kHLG branch, which the
  compiler inlined from code outside the Highway types but inside
  `HWY_NAMESPACE`'s AVX2 attributes: the inverse `ApplyHlgOotf`
  (`forward = false`, recognizable by `1 / gamma`) with its luminance
  `fma(b, Yb, fma(r, Yr, g * Yg))`, and `TF_HLG_Base::OETF` as
  `fma(kA, log(fma(12, s, -kB)), kC)`. `BeforeTransform`'s kHLG branch
  calls the out-of-line `ApplyHlgOotf` (baseline code, no FMA) and its
  inverse OETF has no contractible expression; the gamma, `powf` ratio,
  1-channel OOTF and low OETF branch have none either; the PQ/sRGB branches
  have no scalar arithmetic. The port's SIMD CMS path now runs, on x86-64
  only, `after_transform_hlg_avx2` (`jxl_cms_simd.rs`) with exactly those
  two expressions fused (`f32::mul_add`/`f64::mul_add`, correctly rounded
  like `vfmadd`); the forward direction keeps the unfused scalar
  translation, AArch64 and the scalar path are unchanged (AArch64's scalar
  translation already contracts like the original's NEON build).
  Robustness: the fusion follows from one inlining decision of one compiler
  (Apple Clang cross-compiling with upstream's flags) and agrees with the
  hosted failure, whose request takes exactly this branch; another Clang
  or GCC could inline differently, and only a hosted run proves the fix.
  The other two color stages have no such pattern: in the same scan
  `N_AVX2::ToLinearStage<Op*>::ProcessRow` and
  `N_AVX2::ToneMappingStage::ProcessRow` have only packed FMAs (the
  Highway `MulAdd`s), and the tone mapper's base constructor, which the
  static oracle contracted (the caveat above), has none in the dispatch
  build; both passed in run 43544974125.
- **`s2-noise-random` adapter (2026-11-14, test scaffolding).** The same
  hosted run's AVX2 oracle died with SIGSEGV at the fifth request of
  `simd_noise_generation_matches_the_original_target` (`w = 1`, `rx = 2`),
  the first with a nonzero width and an `rx` off a vector boundary.
  Upstream's `N_AVX2::Random3Planes` (dispatch object of `dec_noise.cc`,
  which `noise_render.cc` includes unchanged) inlines `BitsToFloat` and
  stores to `row + x` with `vmovdqa`, which requires 32-byte alignment;
  libjxl's noise input rects start at `kRenderPipelineXOffset` of 128-byte
  aligned rows. The adapter (`render_stages2_simd.cc`) now places column
  `rx` on a 128-byte boundary by allocating `shift` unprinted samples before
  it and passes `Rect(shift + rx, ry, w, h)`; it prints exactly the same
  `rx + w + 24` by `ry + h + 2` samples per plane as before. Requests,
  observed words and the Rust side are unchanged.
<!-- end color -->

<!-- enable -->
## Scope of the selection (2026-11-14)

Before the default selection may be turned on for a target, the selection
must reach the decoder and nothing else. The original has one process-wide
`HWY_DYNAMIC_DISPATCH`: its SIMD build runs SIMD in the decoder and in the
encoder alike, and `MaxVectorSize()` sizes every plane. The port's encoder
Highway exports are not translated (Plan item 3, issues #231 and #243), and
the encoder is gated byte for byte against `cjxl` in the scalar
configuration. A process-wide switch would let the lossy encoder run the
decoder's SIMD kernels inside its own loop next to its scalar encoder
kernels, an output that matches neither reference build.

**Call sites (scope analysis).** `hwy::decoder_target()` is queried at 31
call sites in 21 source files, and `hwy::max_vector_size()` at one
(`memory_manager_internal::bytes_per_row`, which every plane allocation
calls). Which entry points reach each:

| Call site | Decoder entry points (public decoder, internal whole-decode helpers) | Encoder |
| --- | --- | --- |
| `bytes_per_row` (`max_vector_size`) | every plane | **every plane** (row padding only) |
| `quant_weights::ensure_computed_in` (ComputeQuantTable) | VarDCT frames | **yes**: `enc_ac_strategy` (DCT8 and the strategy mask) and the AR reconstruction |
| `dec_group_render::decode_group_impl` (dequantization, IDCTs, CfL) | VarDCT groups | **yes**: `decode_group_for_roundtrip` in `enc_heuristics_ar::reconstruct_image` |
| stage factories through `PassesDecoderState::prepare_pipeline`: Gaborish, EPF 0-2, upsampling, YCbCr, chroma upsampling, XYB, FromLinear, ToLinear, tone mapping, CMS (`jxl_cms::run`), noise (Add/Convolve), splines | every rendered frame | **yes**: `reconstruct_image` builds the same pipeline (effort 7 and slower, distance from 0.5, EPF on), writing to an `ImageBundle` |
| `stage_write::WriteToOutput::new` (output conversion) | public decoder output buffers and callbacks | no (the AR pipeline writes to an `ImageBundle`) |
| `dec_noise::prepare_noise_input` (Random3Planes) | frames with noise | **yes**: `reconstruct_image` |
| `splines::initialize_draw_cache`, `apply_to_row` | frames with splines | **yes**: `enc_heuristics_lossy` (`subtract_from`) |
| `rct::inv_rct`, `squeeze::inv_squeeze` | modular inverse transforms | **yes**: `enc_modular` (`Transform::inverse`) |
| `dec_modular` (`decode_var_dct_dc`, `modular_image_to_decoded_rect`) | modular and VarDCT DC | no |
| `dec_external_image` (`convert_*_channels_to_buffer`) | output conversion | no |
| `dec_frame::process_sections` | every frame | no |
| `dec_noise::random_three_planes` | (API; no caller in the port) | no |

JPEG reconstruction (`JxlToJpegDecoder` of `decode_to_jpeg`) runs inside
`ProcessInput` and calls no Highway export itself; the frame it
reconstructs from is decoded by the paths above.

**Decision.** The decoder carries its selection, and the kernels read it
from the scope the decoder's entry points establish:

- `JxlDecoder` stores the target it selects when it is created
  (`hwy::decoder_default_target()`: `Target::detect()` when
  `DECODER_SELECTS_SIMD` is on for this target). `jxl_decoder_process_input`,
  `jxl_decoder_flush_image`, `jxl_decoder_skip_current_frame` and
  `jxl_decoder_set_output_color_profile` (with `set_preferred_color_profile`)
  run their work inside `hwy::DispatchScope::Decoding(target)`; the other
  exported functions only store or read fields. The internal whole-decode
  helpers (`decode_vardct`, `decode_modular`, `decode_lossless_thunder`,
  `decode_jpeg_frame`, `SequenceDecoder::decode_frame`,
  `LosslessThunderConfig::decode_frame`) are entry points too and use the
  default selection (`hwy::decoding`).
- `ThreadPool::run` (RunOnPool) captures the calling thread's scope and runs
  every init and task body inside it, on whichever runner thread the runner
  chooses, so a decoder with a parallel runner and its image-out callbacks
  run under the decoder's selection. This holds for the concurrent task
  bodies of `run_on_pool_shared` (ADR 0003, "Concurrent decoder tasks"),
  which run through the same `ThreadPool::run`; `tests/simd_threads.rs`
  checks that decodes with 0 to 8 workers give the SIMD original's words
  (2026-11-14, branch `v1/jxl-threads`).
- `hwy::decoder_target()` is, in order: the test override
  (`set_decoder_simd_for_test`), the scalar gate's `JXL_DECODER_TARGET=scalar`,
  then the thread's scope; outside every decoder entry point it is the
  scalar path. `max_vector_size()` follows it, so planes allocated inside a
  decode get the decoder's row padding and the encoder's planes keep the
  scalar build's.
- Encoder entry points (`jxl_encoder_add_image_frame` and the
  `jxl_encoder_process_output` work loop) run inside
  `DispatchScope::Outside`, also when a decoder callback calls them.

Why this and not a target parameter threaded through every kernel and plane
allocation: the original passes neither. Its kernels call
`HWY_DYNAMIC_DISPATCH` and `BytesPerRow` calls `MaxVectorSize()`, both
global; the port's call sites keep that shape (`decoder_target()` where the
original dispatches), so the translation stays line for line. A parameter
would change the signatures of the functions above and of every plane
constructor the encoder and decoder share, a redesign of the original's
interfaces. The scope is the port's one deviation, at the API boundary,
and it exists only because the encoder is not yet translated: once the
encoder's exports are ported and gated against the original's SIMD build,
the scope can widen to the whole process, as in the original. A decoder
fixes its target at creation, so a decode never mixes targets across calls.

**Tests.** `tests/dispatch_scope.rs` (no oracle; runs in every build):
the engine's lossy encode (effort 7, patches off, distance 1.0 and 2.5) gives
exactly the bytes of an encode with the scalar path forced process-wide,
before and after a public decode, from inside the decoder's image-out
callback, and on another thread during a decode with a runner, and makes no
SIMD selection on its thread (`hwy::simd_selections_on_this_thread`, a
test observation counter in `decoder_target()`). Every public decode (no
runner, a three-thread runner, concurrent with an encode) sees the
decoder's selection and row padding in its callbacks, gives exactly the
float output of a decode with that target forced (a SIMD and a scalar
decode of the input differ, so the comparison tells them apart), and the
selection is the scalar path again when the entry point returns;
`decode_vardct` gives the forced target's output as well. A control in the
test shows that a process-wide SIMD selection reaches the encoder (142
SIMD selections in one encode). Three mutations were checked to fail the
test: a process-wide selection (no scope), no `Outside` scope around the
encoder's work loop (the encode inside a decoder callback then selects
SIMD), and a `RunOnPool` that does not carry the scope (runner threads then
run the scalar path).

**Finding.** With the SIMD target forced process-wide, the encoder's bytes
did not change for any probe input (synthetic RGB 136x72 to 1024x768,
distances 0.5 to 2.9): the AR heuristics' choice per block and the quant
tables did not flip. That is no guarantee for other inputs, which is why
the scope is enforced and tested by selection, not by output bytes.
<!-- end enable -->

<!-- enable: default -->
## Default selection per target (2026-11-14)

`hwy::DECODER_SELECTS_SIMD` was `cfg!(target_arch = "aarch64")`: a decoder
created on AArch64 selects NEON; x86-64 kept the scalar path until
"Default selection on x86-64" (2026-11-14) below.
Working decision 2026-11-14 (delegated, revisable by the owner), on the
evidence of "Decoder path":

- (a) every decode-path Highway export of the Plan has a NEON translation
  with row- or request-level parity against the original's NEON build;
- (b) the public decoder gate (`tests/simd_public.rs`, both modes, with the
  pinned conformance corpus) reports zero differing words on all 352
  distinct inputs. **No input of the gate reaches the `ToLinear` and
  `ToneMapping` stages** (they need a PQ or HLG original and a desired
  intensity target below the original's); for these two stages the
  evidence is the row-level parity of `tests/simd_color.rs` only, not a
  whole decode;
- (c) the scalar gate forces the scalar path (`JXL_DECODER_TARGET=scalar`);
- (d) did not hold for AVX2 then: there was no hosted x86-64 run of the
  combined branch. For its later state see "Default selection on x86-64".

The selection is scoped to decoding ("Scope of the selection"): the
encoder, including the decoder code it runs, stays on the scalar path on
every target. The scalar path stays selectable everywhere: the test
override `set_decoder_simd_for_test(Some(false))` and the scalar gate's
forcing. With the default on, decoded pixels equal the original's NEON
build instead of its scalar build; a few 8-bit samples may differ from
values pinned against a scalar decoder.
<!-- end enable: default -->

<!-- enable: tests -->
## Which path a test runs (2026-11-14)

With the AArch64 default on, a test that decodes through a decoder entry
point and compares with the original's scalar build would run NEON unless it
selects the scalar path itself. `JXL_DECODER_TARGET=scalar` is read only by
the `cpp-oracle` builds and set only by `tools/check.py`, so the PR stage
(`cargo xtask codecs --codec jxl --stage pr`: plain `cargo test -p libjxl-rs
--profile checked`) and a plain `cargo test` ran the retained-capture tests
on NEON: 7 failures without features (`chroma_image`, `decoder_effects`,
`decoder_geometry`, `vardct_image` capture tests and 3 `decoder_public`
tests), and 62 with `--all-features` outside `check.py` (26 of them through the
`jxl_abi_rust` child process, see below).

**Decision: one mechanism, by test kind, in the test itself.**

- *Scalar-reference tests*, the scalar half of the gate (live scalar oracle
  or retained captures of the scalar original): every test in the non-SIMD
  test files that can reach a decoder entry point starts with
  `hwy::select_scalar_decoder_for_test()` (=
  `set_decoder_simd_for_test(Some(false))`, process-wide like Highway's
  `SetSupportedTargetsForTest`). That is 245 tests in 47 files: all
  `#[test]`s of the files that reach the public decoder, the whole-decode
  helpers, `FrameDecoder`, the JPEG reconstruction or the public-decoder
  driver. No test of those binaries selects anything else, so the
  process-wide override cannot race. Tests that call a stage or kernel
  directly run outside every decoder entry point, which is the scalar path
  by construction.
- The scalar parity binary also replays public-decoder histories through
  `jxl_abi_rust`, a C driver linked against the port's static library (built
  with `cpp-oracle`) in its own process, where the test's override does not
  reach. `program_observations` starts every child of that binary with
  `JXL_DECODER_TARGET=scalar` set explicitly on the child, so the child
  selects the scalar path whatever the caller's environment is.
- *SIMD-oracle tests*: the `tests/simd_*.rs` binaries force the detected
  target with `set_decoder_simd_for_test(Some(true))`, as before.
- *Default-selection tests*: `tests/dispatch_scope.rs` alone runs the
  default.
- `tools/check.py` keeps `JXL_DECODER_TARGET=scalar` as a second guard;
  `simd_public`'s `the_scalar_gate_forces_the_scalar_decoder_path` checks it.

The selection is now independent of features, environment and runner (PR
stage, `check.py` stages, plain `cargo test`). No capture or expected value
changed. The retained captures in `tests/data` are documented as taken from
the scalar reference configuration (ADR 0002), the `.x86_64.f32le` files
from the native x86-64 scalar original; none is documented as a SIMD
capture.

Other consumers of the default: the `conformance_decode` example decodes
with the default (it compares with the conformance references under their
own tolerances, as `djxl` from a NEON build would); the encode examples and
the `fuzz/` targets pin no decoder values; the port has no benches and its
doctests do not decode. The engine's JPEG XL tests compare decoded pixels
with jxl-oxide, with the source pixels under a bound, or between two decodes
of the same input; none pins the port's decoded samples exactly.
<!-- end enable: tests -->
<!-- avx2 oracle -->
## AVX2 reference: upstream's dispatch build (2026-11-14)

**Decided on 2026-11-14 and confirmed by the owner the same day** (it was
prepared as a delegated working decision on branch `v1/jxl-avx2-oracle` and
merged only after that confirmation). The
AVX2 oracle variant (`-DORACLE_HWY_TARGET=AVX2`) is now built as upstream's
dynamic-dispatch release build, capped at `HWY_AVX2` through Highway's
`HWY_DISABLED_TARGETS`. This changes how the AVX2 reference is *built*. It
changes neither the upstream source nor its commit (libjxl `a16d32d0`,
Highway `457c8917`), nor any expected value, golden, tolerance or test
domain, nor the SCALAR and NEON variants. Because the root rule forbids
refreshing an oracle, the change was put to the owner, who confirmed it.

**Why this is the reference ADR-0041 names.** ADR-0041 section 1 defines
the SIMD reference build at level L as "the original's release build with
its SIMD enabled and its dispatch capped at L through its own switches
(... Highway `HWY_DISABLED_TARGETS` for every target above `HWY_AVX2` or
`HWY_NEON`)". Upstream's own CMake build on x86-64 (configured at the pin
for `x86_64` with Apple Clang 21.0.0, `compile_commands.json`) passes no
`-m` flag and no `-ffp-contract` flag to any TU (Clang's default
`-ffp-contract=on` forms `fmuladd` inside one expression, which the backend
lowers to an FMA only in a function that has the FMA feature), keeps
`-fno-slp-vectorize -fno-vectorize`, and sets `HWY_DISABLED_TARGETS` to
`HWY_AVX3|HWY_AVX3_SPR|HWY_AVX3_ZEN4|HWY_RVV|HWY_SSSE3|HWY_SVE|HWY_SVE_256|HWY_SVE2|HWY_SVE2_128`
unless the macro is forced. Highway's default policy (`detect_targets.h`,
policy 4) then compiles the attainable targets at and above the static
target: `HWY_SSE2` (static), `HWY_SSE4`, `HWY_AVX2` (`HWY_AVX3_DL` only with
`HWY_WANT_AVX3_DL`). `foreach_target.h` compiles each TU's `HWY_NAMESPACE`
code once per target under `HWY_BEFORE_NAMESPACE` (`#pragma clang attribute
push(target(...))`), and `HWY_DYNAMIC_DISPATCH` selects the best supported
one at run time. Everything outside `HWY_NAMESPACE` is compiled once, for
the SSE2 baseline, without FMA; it is contracted only where the compiler
inlines it into an `N_AVX2` function. The static AVX2 oracle instead
compiled every TU, upstream's and the adapters', with
`-msse2 -mssse3 -msse4.1 -msse4.2 -mpclmul -maes -mavx -mavx2 -mbmi -mbmi2 -mfma -mf16c`
and `HWY_COMPILE_ONLY_STATIC`: the same `HWY_NAMESPACE` kernels, but the
baseline code contracted too. That is not upstream's release build, and
not a cap "through its own switches".

**Evidence (2026-11-14, `-arch x86_64` objects cross-compiled with Apple
Clang 21.0.0, `objdump -d`; x86-64 code cannot run on this machine, and the
hosted runners' Clang may make other inlining decisions).** Every decoder and CMS TU
of upstream's build (`jxl_dec-obj`, `jxl_cms`) was compiled twice, with
upstream's flags and with the static oracle's flags. With upstream's flags
no function outside `HWY_NAMESPACE` contains an FMA instruction; with the
static flags 114 such functions in 16 TUs do (among them `alpha.cc`,
`splines.cc`, `jxl_cms.cc`, `dec_xyb.cc`, `decode.cc`, `dec_cache.cc`,
`quantizer.cc`, `color_encoding_internal.cc`, `stage_spot.cc` and the skcms
baseline engine). The `N_AVX2` functions are the same in both, except where
the static build inlines more. The four failures of the first complete
hosted AVX2 run (run 43533597186, Linux x86-64) each come from such a
function:

| Request (first failure) | Differing word | Original function | FMA, static flags | FMA, upstream flags |
| --- | --- | --- | --- | --- |
| `cl-tm` PQ 10000 to PQ 1000 nits | 36: 1025594320 vs 1025594317 | `Rec2408ToneMapperBase` constructor (`cms/tone_mapping.h`) in `stage_tone_mapping.cc` | 9 | none; only `N_AVX2::ToneMappingStage::ProcessRow` (52 in both) |
| `cl-cms` HLG to linear, sRGB, 1000 nits | 33: 1064533059 vs 1064533058 | forward `ApplyHlgOotf` luminance (`jxl_cms.cc`, outside `HWY_NAMESPACE`) | contracted | `N_AVX2::DoColorSpaceTransform` calls the out-of-line `ApplyHlgOotf`, which has no FMA |
| `sp-cache` (spline replay) | 49: 1069378134 vs 1069378135 | `Splines::InitializeDrawCache` (`splines.cc`) | 36 | none; only `N_AVX2` `SegmentsFromPoints`/`DrawSegments` |
| `blendstage-row` (blending replay) | 65: 0 vs 2147483648 | `PerformAlphaBlending` (`alpha.cc`; blending has no Highway code) | 16 + 15 | none |

The port's scalar translations of these functions fuse only on AArch64
(`mul_add_f32`), as upstream's baseline objects do. Numeric check, tone
mapping: a scratch model on AArch64 that compiles the pristine
`stage_tone_mapping.cc` with `HWY_NAMESPACE` code contracted and the base
headers either contracted or not (`#pragma clang fp contract`), fed the
failing request with x86-64's row offset, prints word 36 as 1025594317
(contracted: the static oracle's word) and 1025594320 (not contracted: the
port's word). The other three were not modeled numerically; the
instruction evidence above is their evidence. The same model showed the
link-order hazard of the static build: the linker keeps one copy of an
inline function per program, so which TU's flags win was not under the
oracle's control. In the dispatch build every TU has the same baseline
flags.

**What changed (compile flags and sources).**

- `CMakeLists.txt`: the AVX2 variant no longer appends
  `-msse2 -mssse3 -msse4.1 -msse4.2 -mpclmul -maes -mavx -mavx2 -mbmi -mbmi2 -mfma -mf16c`
  to `CMAKE_CXX_FLAGS`; it adds the compile definition
  `ORACLE_HWY_DISPATCH=1` to the adapters. Nothing else in the flags of
  any TU changes (the SIMD variants' removal of the scalar per-TU pins is
  unchanged).
- `hwy_target.h`: `HWY_COMPILE_ONLY_STATIC` is gone; `HWY_DISABLED_TARGETS`
  changes from `HWY_AVX3|HWY_AVX3_DL|HWY_AVX3_ZEN4|HWY_AVX3_SPR` to
  upstream's default list plus `HWY_AVX3_DL`:
  `HWY_SSSE3|HWY_AVX3|HWY_AVX3_DL|HWY_AVX3_ZEN4|HWY_AVX3_SPR|HWY_RVV|HWY_SVE|HWY_SVE_256|HWY_SVE2|HWY_SVE2_128`.
  Compiled targets: `HWY_SSE2` (static), `HWY_SSE4`, `HWY_AVX2`. It also
  defines `FJXL_ENABLE_AVX512 0`, which upstream's CMake derives from the
  same `JPEGXL_ENABLE_HWY_AVX3=OFF` but skips when the list is forced: the
  fast-lossless encoder's own AVX-512 path was compiled, and selectable on
  an AVX-512 runner, in the static build too.
- `hwy_ops.cc` keeps its checks (no target above AVX2 compiled; abort at
  start unless dispatch selects `HWY_AVX2` and the CPU reports it, which a
  non-AVX2 x86-64 CPU now fails instead of running SSE4) and adds a
  compile-time check that the static target stays below `HWY_AVX2`.
- Adapters that call `N_AVX2` code directly compile it as upstream does:
  `oracle_target.h` gives their functions Highway's `HWY_AVX2` target
  attributes (`ORACLE_TARGET_BEGIN`/`END`, empty in the other variants);
  `dct.cc` and `encoder_transforms_unit.cc` compile their `-inl.h` header
  per target through `foreach_target`, and `dequant.cc`'s per-target block
  moves into `dequant_targets-inl.h` for that variant; `to_linear.cc`,
  `from_linear.cc` and `xyb_stage.inc` only gain the attributes.
- Not served by the AVX2 variant (an unknown command there, which fails a
  request loudly): the scalar suite's `dq-group`/`dq-jpeg` requests
  (`group_render.cc`, which instantiates the extracted `DecodeGroupImpl`
  for one target) and its `tm-*` requests (`tone_mapping.inc` in
  `sample_conversion.cc`). Their pristine `stage_tone_mapping.cc` would
  follow `dec_modular.cc`'s `foreach_target` in that TU, compile only the
  static target (`foreach_target.h` runs once per TU) and replace the
  archive's dispatch factory; for the same reason `uint_output.inc` takes
  `dec_external_image.cc`'s functions from the archive in this variant
  instead of including the pristine file. A request log of every SIMD test
  binary against the NEON oracle (2026-11-14: 45 request kinds, 373,033
  requests, among them `ue-channels`, `fe-channels`, `fe-row`, `gab-row`,
  `vd-decode`, `dct-*`, `dq-bias`, `dq-block`) contains none of the
  unserved kinds.
- SCALAR and NEON unchanged: the preprocessed token streams of all changed
  adapter TUs, with the SCALAR and the NEON build's own compile commands,
  are identical before and after (line markers aside), and their compile
  commands are identical.

**Port choices that relied on the static build.** Any port result that
matched the static oracle only because it fused baseline code would now
fail. The port fuses scalar code on x86-64 only through explicit
`mul_add`; on the SIMD path these are `y_to_x_ratio_simd`/
`y_to_b_ratio_simd` (`dec_group_render.rs`), the Gaborish weight
normalization (`stage_gaborish_simd.rs`) and the IDENTITY residual
(`dec_transforms_simd.rs`). In upstream's dispatch objects
`N_AVX2::DecodeGroupImpl` contains the two scalar FMAs of the CfL ratios
(`vfmadd...ss`; commit `7531646f` stays right), `N_AVX2::GetGaborishStage`
the three of the normalization, and `N_AVX2::TransformToPixels` its scalar
ones. The noise stage's `YtoXRatio(0)`/`YtoBRatio(0)` are fused in
`N_AVX2::AddNoiseStage::ProcessRow` and unfused in the port; with factor 0
both give `base` exactly, so they cannot differ. One place goes the other
way and is a finding about the port (not changed here, `src/**` belongs to
another package): in upstream's dispatch objects
`N_AVX2::DoColorSpaceTransform` inlines the *inverse* `ApplyHlgOotf` and
`TF_HLG_Base::EncodedFromDisplay` of `AfterTransform` and contracts them
(two `vfmadd231ss` before `powf`, two `vfmadd132sd` around `log`), while
the port's SIMD CMS path keeps their x86-64 unfused scalar translation.
`cl-cms` requests with an HLG destination are therefore expected to differ
against the dispatch oracle on AVX2. That contraction depends on the
compiler's inlining decision (Apple Clang 21 here; the hosted runners use
their own Clang), so only the hosted run can settle it.

**Still to show on real AVX2 hardware (hosted run):** that the dispatch
oracle selects `HWY_AVX2`, that the four failures above pass, and that the
results that passed against the static build still pass.
<!-- end avx2 oracle -->


<!-- encoder simd -->
## Encoder exports the engine's profiles reach (2026-11-14)

The engine encodes JPEG XL in two profiles (root ADR-0039): lossy VarDCT
through the public encoder (effort 7, distance 0.05 to below 3.0, patches
off, gray or RGB, enum sRGB or linear sRGB, optionally one straight 8-bit
alpha channel coded losslessly, one-shot and streaming input) and lossless
`LosslessThunderConfig` (effort 2, gray or RGB, optionally with alpha).
Which Highway exports each reaches was read from the code paths and
checked by running them: a temporary probe (not committed) at the port's
counterpart of every export and at every `hwy::decoder_target()` call site
recorded what `examples/encode_lossy` (distances 0.1, 1.0, 2.9),
`examples/encode_lossless`, a lossless encode with alpha and gray, and the
`ferrisma-jxl-encoder` tests for gray, linear sRGB, alpha and the
streaming path reached on a 403x302 and a 2268x1512 photograph.

**Lossy profile.** Encoder exports:

| Export (file) | Functions reached | Lines |
| --- | --- | --- |
| `enc_xyb` | `ToXYB`, `ComputePremulAbsorb` (not `LinearRGBRowToXYB`, not `RgbToYcbcr`) | 1,009 |
| `enc_transforms` | `TransformFromPixels`, `DCFromLowestFrequencies`, `AFVDCT4x4` | 1,526 |
| `enc_group` | `ComputeCoefficients` | 2,913 |
| `enc_chroma_from_luma` | `InitDCStorage`, `ComputeTile` | 2,669 |
| `enc_ac_strategy` | `ProcessRectACS` | 3,468 |
| `enc_adaptive_quantization` | `AdaptiveQuantizationMap` | 2,514 |
| `enc_cluster` | `FastClusterHistograms`, `HistogramEntropy`, `HistogramCondition` | 560 |
| `enc_entropy_coder` | `TokenizeCoefficients` | 217 |
| `enc_ans_simd` | `EstimateTokenCost` | 211 |
| `enc_ma` | `FindBestSplit` (only with alpha: the extra channel's tree learning) | 809 |
| `enc_modular_simd` | `EstimateCost` (only with alpha) | 223 |

Decode-path exports the lossy encoder runs as well: `convolve_symmetric5`
(`Symmetric5`, from `enc_gaborish`'s inverse Gaborish and the
`enc_adaptive_quantization` mask; the decoder never calls it; translated
since, see "Encoder path: chroma from luma"),
`quant_weights` (`ComputeQuantTable`), `dec_group` (`DecodeGroupImpl`, the
AR round trip, distance 0.5 and above), the AR reconstruction pipeline's
factories `GetGaborishStage`, `GetEPFStage0..2`, `GetUpsamplingStage` (the
8x upsampler is built) and `GetToneMappingStage` (built, not needed), and
`simd_util`'s `MaxVectorSize` (row padding of every plane).

Not reached: `enc_rct`, `enc_convolve_separable5` and `enc_detect_dots`
(dots need a distance of 3.0 or more), the encoder's butteraugli
(`FindBestQuantizer` compares with butteraugli only at effort 8 and
slower; effort 7 is Squirrel) and `dec_xyb`'s `OpsinToLinear` (only on the
downsampling path, outside the profile).

**Lossless profile.** `enc_rct` (`FwdRctImpl`), `enc_cluster`
(`FastClusterHistograms`, `HistogramEntropy`) and `MaxVectorSize`. Not
reached: `enc_ma` (Thunder uses a fixed tree), `enc_modular_simd`,
`enc_ans_simd`, `HistogramCondition`.

So the lossy profile needs 11 encoder exports (9 without alpha) plus
`Symmetric5` and the decode-path exports above, the lossless profile 2
plus `MaxVectorSize`. This branch translates them in the order of the
encoder SIMD package: `enc_xyb`, `enc_transforms`, `enc_group`, then
`enc_chroma_from_luma`.

## Encoder path (2026-11-14)

**Dispatch point.** The encoder's translated kernels dispatch on
`hwy::encoder_target()`, the encoder's counterpart of `HWY_DYNAMIC_DISPATCH`,
at the call site where the original dispatches (for example
`enc_xyb::to_xyb` for `HWY_DYNAMIC_DISPATCH(ToXYB)`). It is independent of
the decoder's `decoder_target()` and of its scope: `ENCODER_SELECTS_SIMD`
is `false` on every target, so `encoder_target()` is the scalar path unless
a test forces the detected target with `hwy::set_encoder_simd_for_test`
(process-wide, like `SetSupportedTargetsForTest`). Only `tests/simd_encoder.rs`
calls it, so every encoder parity test against the scalar original (the
`cjxl`-equivalent public encoder, the checkpoint suites) and
`tests/dispatch_scope.rs` keep running the scalar path unchanged. The
decoder code the encoder runs (the AR round trip, quant tables, the AR
pipeline's stages) and the row padding of the encoder's planes
(`max_vector_size()`) keep following `decoder_target()`, which is the
scalar path outside decoding.

**When the encoder may select SIMD on a target.** Working decision
2026-11-14 (delegated, revisable by the owner): `ENCODER_SELECTS_SIMD` may
be turned on for a target only when, on that target,

- (a) every Highway export that an encoder setting in scope reaches is
  translated with request-level parity against the original built for the
  same single Highway target: the exports listed above for the profile in
  scope, including the decode-path ones the encoder runs (`Symmetric5`,
  `ComputeQuantTable`, `DecodeGroupImpl`, the AR pipeline's stages) and
  `MaxVectorSize`; an encode that mixes scalar and SIMD kernels matches
  neither reference build;
- (b) the selection reaches the decoder code inside the encoder and the
  encoder's plane padding as well, as the original's one process-wide
  dispatch does (the "Scope of the selection" deviation then narrows or
  goes);
- (c) a whole-encode comparison through the public encoder (and
  `LosslessThunderConfig` for the lossless profile) against `cjxl` built for
  that single target is byte-identical over the engine's profiles and a
  corpus at least as wide as the scalar `cjxl` gate;
- (d) the scalar encoder gates then force the scalar path explicitly, so
  they still compare the scalar translation with the scalar original;
- (e) for AVX2, the same evidence exists from real x86-64 hardware, as for
  the decoder.

**Translated so far.**

- `enc_xyb` (`enc_xyb_simd.rs`): ToXYB with its linear-sRGB and sRGB
  branches (LinearSRGBToXYB, SRGBToXYB, SRGBToXYBAndLinear),
  LinearRGBRowToXYB, ComputePremulAbsorb (`Lanes` copies of each constant),
  OpsinAbsorbance, StoreXYB, LinearRGBToXYB and fast_math-inl.h's
  CubeRootAndAdd; LinearFromSRGB is `simd_transfer`'s
  `srgb_display_from_encoded`. Rows are processed in whole vectors up to
  `RoundUpTo(xsize, Lanes)`, inside the planes' 128-byte row padding.
  RgbToYcbcr is not translated (no scalar translation either; the profiles
  do not reach it). `tests/simd_encoder.rs` compares `esx-premul`,
  `esx-row` and `esx-xyb` (tools/oracle/encoder_simd.cc: buffers 128-byte
  aligned and sized for 8 lanes, every sample up to `RoundUpTo(xsize, 8)`
  printed, finite sentinels) over 1-129 columns, twelve value kinds
  (pixel-like, ramps, wide gamut, cube-root and sRGB-knee boundaries,
  the special words), the four admitted encodings in both rendering
  intents, the SameSize failures and the exceptional intensities; `esx-xyb`
  goes through `to_xyb` with the encoder's selection forced, and the scalar
  path differs from it in most requests. `tests/simd_vardct_lanes8.rs`
  checks the row loop and constant layout with eight lanes.
- `enc_transforms` (`enc_transforms_simd.rs`): TransformFromPixels for all
  27 strategies (DCT2TopBlock, AFVTransformFromPixels), DCFromLowestFrequencies
  with ReinterpretingIDCT, and AFVDCT4x4 (`HWY_CAPPED(float, 16)`: four NEON,
  eight AVX2 lanes). The scaled DCT and IDCT are `dct_simd`'s, shared with
  the decoder. The scalar statements lie inside `HWY_NAMESPACE`: Clang
  contracts AFVTransformFromPixels' DC mix `block00 + block01 +/- 2 *
  block10` on both targets (the x86-64 scalar build does not), no other
  statement has a contractible product and sum. `esx-pixels`, `esx-llf`
  and `esx-afv` pass the scratch the encoder's callers pass (`2 *
  kMaxCoeffArea + 3 * 8 * kMaxBlockDim`, enc_group.cc for 8 lanes), 128-byte
  aligned, and print it up to its last written word; coefficients and DC
  outputs are printed with 8 sentinel words after them. The domain: every
  strategy, pixel strides `cols + {0, 1, 5, 8}` (DC strides `+ {0, 3, 8}`),
  the twelve value kinds (fewer for transforms above 64x64), and impulses
  for AFVDCT4x4; all go through the dispatching `transform_from_pixels`,
  `dc_from_lowest_frequencies` and `afvdct4x4`.
- `enc_group` (`enc_group_simd.rs`): ComputeCoefficients with
  QuantizeBlockAC (`HWY_CAPPED(float, kBlockDim)`: four NEON, eight AVX2
  lanes, the `xsize == 1` threshold mask as a lane select),
  QuantizeRoundtripYBlockAC (quantizer-inl.h's vector AdjustQuantBias with
  its `ApproximateReciprocal`, `dec_group_simd`'s) and the fused
  color-correlation unapply; inside `HWY_NAMESPACE` it calls the SIMD
  transforms above. AdjustQuantBlockAC is all scalar: the SIMD path runs
  the scalar translation's body with its two double contraction sites
  fused (`adjust_quant_block_ac_with::<true>`), as is QuantizeBlockAC's
  `thresholds[i] -= 0.00744f * xsize * ysize`; on x86-64 the scalar
  build keeps them separate. `esg-block` calls the target namespace's
  QuantizeBlockAC (external linkage, not an export) over every strategy's
  layout, three channels, six threshold sets and the value kinds;
  `esg-coeffs` runs the dispatching ComputeCoefficients over every group of
  seven image sizes up to 300x260 (multi-group), random aligned strategy
  layouts, color-correlation tiles, quant fields, speed tiers Squirrel,
  Wombat, Hare and Falcon (both AdjustQuantBlockAC branches) and special
  words, with the matrices of the target's ComputeQuantTable on both sides,
  and compares the quant field, the DC image and every group's coefficients.
  A mutation that unfuses the color-correlation NegMulAdd fails it; one that
  unfuses AdjustQuantBlockAC's double sites does not change a word over this
  domain on NEON (the rounded results agree), so those sites rest on the
  contraction analysis and, on x86-64, on the AVX2 run.

**Evidence (2026-11-14, M1 Pro).** `tests/simd_encoder.rs` against the
NEON build and, under Rosetta 2 (`ROSETTA_ADVERTISE_AVX=1`), against the
AVX2 dispatch-build oracle (`5aaf0089`): zero differing words on both.

| Family | Requests | Words (NEON) | Words (AVX2) |
| --- | --- | --- | --- |
| `esx-premul` | 93 | 9,486 | 9,486 |
| `esx-row` | 1,653 | 181,824 | 181,824 |
| `esx-xyb` | 3,403 | 758,804 | 758,804 |
| `esx-pixels` | 1,056 | 6,079,104 | 6,414,720 |
| `esx-llf` | 972 | 7,241,232 | 7,280,976 |
| `esx-afv` | 304 | 15,504 | 15,504 |
| `esg-block` | 4,698 | 7,070,112 | 7,070,112 |
| `esg-coeffs` | 42 | 2,286,858 | 2,286,858 |

(The scratch words differ in count because the scratch layout follows the
lane count.) The AVX2 run found one defect, fixed: ComputeCoefficients
inlines `YtoXRatio`/`YtoBRatio` inside `HWY_NAMESPACE`, so both SIMD
builds fuse them; the scalar x86-64 form differed by one ULP in a B
coefficient. Rosetta does not fault on misaligned aligned moves: the
adapters' alignment and 8-lane sizing rest on the reasoning above, and
the hosted Linux x86-64 run is still owed.
<!-- end encoder simd -->
<!-- audit-fix: the audit tools -->
## Which path the audit tools run (2026-11-14, evening)

Addendum to "Which path a test runs".

The scheduled nightly of 2026-11-14 (run 43578105956, `main` at `7d663784`)
failed in `JXL: Audit (macos-latest, decoder)`: `tools/chroma_image_reference.py`
runs `cargo test --all-features --test parity chroma_image::` outside
`check.py`, and the three `chroma_image` tests of the parity binary did not
select the scalar path. They send `fs-decode` requests, which
`frame_sequences::observe` answers through a decoder entry point, so on
AArch64 they decoded on NEON and compared with the scalar original. The
selection above missed them because the file itself calls no entry point;
it reaches one only through another module's request handler. The same
audit's `decoder_public_reference.py` also starts two children that link the
port, the `decoder_public_probe` replay and `jxl_abi_rust`, without
`JXL_DECODER_TARGET=scalar`.

- The three `chroma_image` tests now start with
  `select_scalar_decoder_for_test()`, as every scalar-reference test does.
- `decoder_public_reference.py` sets `JXL_DECODER_TARGET=scalar` on both
  children (their libraries are built with `--all-features`, so with
  `cpp-oracle`), as `program_observations` does for `jxl_abi_rust`.
- Every audit tool that runs the parity binary against the scalar original
  (`chroma_image`, `decoder_api`, `decoder_geometry`, `extra_channels`,
  `icc_reader`, `jpeg_payload`, `jpeg_reconstruction`, `output_stage`,
  `skcms`, `skcms_parser`, `skcms_transform`) sets
  `JXL_DECODER_TARGET=scalar` on that `cargo test`, the second guard
  `check.py`'s test stage has.

Both mechanisms are independent of the target: they select the scalar path
whatever `DECODER_SELECTS_SIMD` says, so they keep holding when x86-64 is
switched on. No capture, reference or expected value changed. The audit
tools' other children run only the original, a relinked original or a Rust
probe that calls no decoder entry point (Brotli, box contents, decoder
input, the CMS and skcms probes).
<!-- end audit-fix -->
<!-- enable-x86 -->
## Default selection on x86-64 (2026-11-14)

`hwy::DECODER_SELECTS_SIMD` is `cfg!(any(target_arch = "aarch64",
target_arch = "x86_64"))`: a decoder created on x86-64 selects AVX2 when
`Target::detect()` finds Highway's complete AVX2 group, and the scalar path
otherwise (AVX-512 is never selected, D6). Working decision 2026-11-14
(delegated, revisable by the owner), on this evidence:

- (a) every decode-path Highway export of the Plan has an AVX2 translation;
  the SIMD binaries match the AVX2 oracle (upstream's dispatch build capped
  at AVX2, "AVX2 reference: upstream's dispatch build") under Rosetta 2 on
  this machine and on hosted Linux x86-64 (run 43599382021);
- (b) the public decoder gate on AVX2 under Rosetta 2: 650 decodes of 325
  files without the corpus and, for the first time on x86-64 (issue #215),
  704 decodes of 352 files with the pinned conformance corpus, all
  matching; hosted run 43599382021 matched the gate without the corpus on
  native hardware. As on AArch64, no input reaches `ToLinear` or
  `ToneMapping`; their evidence is the row-level parity of
  `tests/simd_color.rs` (also matching on AVX2);
- (c) the scalar gate forces the scalar path on x86-64 as well
  (`tools/check.py --target x86_64-apple-darwin`, and the explicit
  scalar-reference selection of "Which path a test runs");
- (d) native x86-64 evidence is hosted run 43599382021 plus the hosted run
  of this branch; Rosetta 2 is not native hardware (it does not fault on a
  misaligned aligned move, for example), so the local runs do not replace
  them.

Limits. AVX2 output can differ between CPU vendors exactly as the
original's AVX2 build does (`rcpps` in AdjustQuantBias, open question O4);
identity is per host. The originals cross-built on this machine contain only
skcms's baseline engine (CMake sees the host's `arm64` processor, upstream's
`skcms.cmake` then defines `SKCMS_DISABLE_HSW`), so the port's skcms HSW
engine was not compared with the original's HSW engine locally; hosted Linux
builds both with HSW (open question O9). Engine selection in skcms does not
depend on this default.

Defects found by the x86-64 runs with the default on (fixed before it):
`Avx2::load_dup128` formed a `&__m128` from a slice pointer, which requires
16-byte alignment (undefined behavior for an unaligned row; a debug build of
`ferrisma-jxl-decoder`'s tests aborted on it); it now loads unaligned into a
local and broadcasts that, the same lanes. `DequantMatrices::set_dc_quant`
lost a signaling NaN's payload after LLVM merged its copy into the division
(`dc / 1.0`) in one x86-64 build; the copy is now a separate array copy.
<!-- end enable-x86 -->

<!-- encoder acs -->
## Encoder path: `enc_ac_strategy` (2026-11-14)

Addendum to "Encoder path", "Translated so far".

`enc_ac_strategy_simd.rs` translates the whole `HWY_NAMESPACE` part of
enc_ac_strategy.cc, the `ProcessRectACS` export, for NEON and AVX2, written
once against `SimdF32`, in four units:

- EstimateEntropy (the entropy estimate): the in-namespace SIMD
  TransformFromPixels and TransformToPixels (`enc_transforms_simd`,
  `dec_transforms_simd`; nothing duplicated), fast_math-inl.h's one-lane
  vector FastPowf, the target's `SumOfLanes`, and the masking loop with
  `Lanes(df8)` in its bound. `HWY_FULL(float)` and `HWY_CAPPED(float, 8)`
  are the same descriptor on both targets, so the result depends on the
  lane count by design (NEON and AVX2 give different words).
- FindBest8x8Transform (the per-strategy cost evaluation of the ten 8x8
  candidates).
- TryMergeAcs and FindBestFirstLevelDivisionForSquare (the block-level
  merges); the boundary checks, SetEntropyForTransform and the strategy
  helpers have no floating-point or vector code and are the scalar
  translation's.
- ProcessRectACS itself, dispatched from `AcStrategyHeuristics::process_rect`
  on `hwy::encoder_target()` (`HWY_DYNAMIC_DISPATCH(ProcessRectACS)`).

The scalar statements inside `HWY_NAMESPACE` carry FMA on both targets, so
the sites the scalar translation fuses only on AArch64 are fused on x86-64
too: `quant_norm16 += qval * qval`, EstimateEntropy's three `entropy += a *
b`, FindBest8x8Transform's two `entropy_mul` adjustments and the inlined
`YtoXRatio`/`YtoBRatio` (`y_to_x_ratio_simd`). Init, PrepareForThreads and
Finalize (the finalization: block counting into `AuxOut`) lie outside
`HWY_NAMESPACE`; they stay the scalar translation's, and the SIMD path runs
them unchanged. Remaining differences from the original's SIMD build: Init
computes the dequant matrices through `ensure_computed_in`, whose
ComputeQuantTable follows `decoder_target()` (scalar outside decoding),
while the original's Init runs the target's ComputeQuantTable; and
PrepareForThreads sizes the DCT scratch with the scalar `MAX_VECTOR_SIZE`
(no word depends on that size up to 64x64 transforms). Both belong to
condition (b) above; the parity tests compute the target's matrices first.

`tests/simd_encoder_acs.rs` against tools/oracle/encoder_acs_simd.cc
(`esa-*`; the target namespace's functions, external linkage, and the
original AcStrategyHeuristics for `esa-rect`; buffers 128-byte aligned,
sized as PrepareForThreads for 8 lanes, sentinel-filled, every written
word printed) compares costs and decisions word for word: `esa-entropy`
(every strategy below DCT128X128, three image sizes, three positions, five
contents with the special words, four `mask1x1_xsize` cuts, four distances),
`esa-best8` (eight distances across the 4, 5 and 12 thresholds, six speed
tiers), `esa-merge` and `esa-square` (every tile of three images, nine
merge strategies, priorities, aligned and floating layouts and positions,
blocks 2/4/8 with and without the square) and `esa-rect` (through the
dispatching `process_rect` with the encoder's selection forced: six image
sizes up to 200x136, speed tiers Glacier to Hare and Falcon, decoding tiers
0/1/2/4; the strategy image, Finalize's counts and the thread's block and
first `kMaxCoeffArea` scratch words). Each family shows that the scalar
translation differs; in `esa-rect` it differs only in the costs: the scalar
and SIMD searches chose the same strategies in all 178 requests, so a
comparison of the decisions alone would not tell the paths apart. An
AArch64-only module checks the 8-lane structure (an 8-lane descriptor of
two NEON halves with AVX2's reduction order against a lane-explicit model
of the original's loop, every masking cut around the strategy's columns).

| Family | Requests | Words (NEON) | Words (AVX2) |
| --- | --- | --- | --- |
| `esa-entropy` | 945 | 46,610,865 | 46,822,545 |
| `esa-best8` | 720 | 47,440,800 | 47,498,400 |
| `esa-merge` | 705 | 28,640,093 | 28,757,517 |
| `esa-square` | 440 | 20,889,552 | 20,998,640 |
| `esa-rect` | 178 | 29,825,905 | 29,825,905 |

Zero differing words on both targets (M1 Pro; AVX2 under Rosetta 2 with
`ROSETTA_ADVERTISE_AVX=1` for the SIMD test binary only, against the AVX2
dispatch-build oracle built with `build_simd_oracle`'s CMake flags). The
scratch word counts differ between the targets because the scratch layout
follows the lane count. The AVX2 run found no defect; the hosted Linux
x86-64 run (alignment, real hardware) is still owed.

With this export the lossy profile's encoder exports still untranslated on
this branch are `enc_chroma_from_luma`, `enc_adaptive_quantization`, `enc_cluster`,
`enc_entropy_coder`, `enc_ans_simd` (and `enc_ma`, `enc_modular_simd` with
alpha), plus `Symmetric5`; nothing of `enc_ac_strategy` remains.
<!-- end encoder acs -->
<!-- encoder cfl -->
### Encoder path: chroma from luma (2026-11-14, branch `v1/jxl-enc-cfl`)

Translated further, on the encoder's dispatch point `hwy::encoder_target()`
(off by default; only `tests/simd_encoder_cfl.rs` forces it):

- `enc_chroma_from_luma` InitDCStorage (`enc_chroma_from_luma_simd.rs`),
  dispatched from `CfLHeuristics::init`: four rows of
  `RoundUpTo(num_blocks, Lanes(df))` samples with `HWY_FULL(float)` (four
  NEON, eight AVX2 lanes), the last `Lanes` of each zeroed. `ecf-init`
  (tools/oracle/encoder_cfl_simd.cc, through the original's
  `CfLHeuristics::Init`) compares the status, the lanes of the original's
  `MaxVectorSize()`, the plane size and the zero-filled lanes of each row
  (the others are uninitialized in the original) for every block count
  from 0 to 60 in several shapes, pixel sizes that are not block multiples
  and photograph-sized frames; the scalar path differs in the plane width.
  The `lanes8` module checks the rounding and the zeroed lanes with eight.
- `enc_chroma_from_luma` ComputeTile with CFLFunction and
  FindBestMultiplier (same file), dispatched from `CfLHeuristics`'s
  `compute_tile`; inside `HWY_NAMESPACE` it calls the SIMD transforms of
  `enc_transforms_simd`. The searches accumulate per lane with fused MulAdd
  and reduce with the target's SumOfLanes (`vaddv` on NEON, Highway's
  block-swap tree on AVX2), so their results follow the lane count. The
  fast search's denominator `GetLane(SumOfLanes(ca)) + num * distance_mul
  * 0.5f` is contracted inside `HWY_NAMESPACE` on both targets
  (`vfmadd231ss` in the AVX2 dispatch object's `N_AVX2::FindBestMultiplier`,
  read from its disassembly) and fused here; the scalar translation fuses
  it only on AArch64. A mutation that unfuses it changes no word of
  `ecf-best` or `ecf-tile` on NEON (the rounded multipliers agree), so that
  site rests on the disassembly. No other scalar statement has a product and a sum in
  one expression. `CfLHeuristics::items_per_thread` sizes the per-thread
  scratch with the encoder target's `MaxVectorSize()` when the encoder's
  SIMD path is selected (ComputeTile ensures the size). `ecf-best` calls the
  target namespace's FindBestMultiplier (external linkage, not an export)
  on 128-byte aligned inputs: 8 to 4,096 values (multiples of 8), six value
  kinds (noise, chroma proportional to luma with swept multipliers,
  magnitudes around CFLFunction's threshold of 100, six decades, sparse
  impulses, the special words), six distance multipliers, both bases and
  both searches. `ecf-tile` runs the CfL loop of enc_heuristics.cc through
  the original's `CfLHeuristics` (Init, PrepareForThreads, ComputeTile per
  8x8-block tile) over nine frame sizes from one block to 40x34 blocks
  (partial tiles, transforms wider than a tile), both passes (DCT8, and
  random aligned strategy layouts with a raw quant field and a quantizer,
  Newton and fast search), `opsin_rect` at the origin and offset, and
  special words; it compares the statuses, both color maps and every word
  of `dc_values` (sentinels after Init). The scalar path differs in most
  requests. The `lanes8` module (8 lanes from two NEON halves, AVX2's
  SumOfLanes order) checks that ComputeTile's blocks, collected
  coefficients and `dc_values` equal the 4-lane ones word for word, and
  that both searches equal a model of the per-lane accumulation for 4 and
  8 lanes; over its domain the rounded multipliers of 4 and 8 lanes never
  differ, so the 8-lane reduction itself rests on the AVX2 oracle run.
- `convolve_symmetric5` Symmetric5 (`convolve_symmetric5_simd.rs`), the
  decode-path export the lossy encoder runs (inverse Gaborish, the
  adaptive-quantization mask) and the decoder never calls: it now has a
  dispatch point, on `hwy::encoder_target()` in `symmetric5`. Each row runs
  the target's Symmetric5Row: Symmetric5Border for the first
  `RoundUpTo(kRadius, Lanes)` pixels and the tail, Symmetric5Interior in
  whole vectors (LoadU around the center, LoadDup128 of each four-word
  weight array, separate Mul and Add). Neither SIMD object of the file
  contains a fused operation, so the border is the scalar translation's
  `symmetric5_border` and the interior lanes round as the scalar
  interior; SIMD and scalar differ only where a weight array's four copies
  differ (the interior reads all four, the border the first), which the
  encoder's own weights never do. `ecf-sym5` calls the original's
  dispatching `Symmetric5` (no pool) on whole planes of every width from 1
  to 20 and around the vector multiples up to 129, heights 1 to 9 (all
  rows mirrored up to 4), rects inside larger planes into offset rects of
  other planes, five weight sets (Gaborish-like, mask-like, random, and
  copies that differ per lane) and value kinds with special words, and
  compares every word of the sentinel-filled output plane; the scalar path
  differs in the requests with differing copies. The `lanes8` module
  compares 8-lane rows (AVX2's LoadDup128) with 4-lane rows word for word.

With these and `enc_ac_strategy` ("encoder acs" above), the lossy
profile's encoder exports still untranslated are `enc_adaptive_quantization`,
`enc_cluster`, `enc_entropy_coder`, `enc_ans_simd` (and `enc_ma`,
`enc_modular_simd` with alpha). `Symmetric5`, which "convolve_symmetric5
and simd_util" above leaves as encoder work, is translated.
<!-- end encoder cfl -->
<!-- encoder entropy -->
## Encoder path: the smaller exports (2026-11-15, branch `v1/jxl-enc-entropy`)

Addendum to "Encoder path", "Translated so far".

Translated behind `hwy::encoder_target()` like the wave-1 exports, each
next to its scalar translation, which stays the default path:

- `enc_rct` (`modular/transform/enc_rct_simd.rs`): FwdRctImpl's
  `FwdRctRow` for all seven transforms and six permutations, through both
  `FwdRct` entry points. The vector loop has no scalar tail: the last step
  loads and stores a whole vector, so `RoundUpTo(w, Lanes)` samples per row
  are read and written inside the row padding. `Add`/`Sub` wrap.
- `enc_cluster` (`enc_cluster_simd.rs`): HistogramCondition,
  HistogramEntropy and FastClusterHistograms with the in-namespace Entropy,
  HistogramDistance, ClusterDistance and HistogramKLDivergence
  (`HWY_CAPPED(float, 8)`: the full vectors). HistogramCondition cuts a
  histogram to a multiple of the lane count, not to its last nonzero count
  as the scalar build does; the clustering decisions (symbols, the output
  histograms, the entropy caches) are compared. The whole-vector loads need
  histogram lengths that are multiples of the lane count, as the original's
  reads do (Add and EnsureCapacity keep them multiples of 8); the port's
  bounds-checked loads panic where the original would read past the end.
- `enc_entropy_coder` (`enc_entropy_coder_simd.rs`): TokenizeCoefficients
  with NumNonZeroExceptLLF and NumNonZero8x8ExceptDC. Integer counting
  only, so the SIMD and scalar paths give the same words; the test shows
  that both agree with the original's target. The source's LLF mask array
  sets only its first four words and is read from `kMaxCoeffBlocks - cx`, so
  it masks coefficients only for 29 or more covered columns (as the scalar
  translation already records); translated as written.
- `enc_ans_simd` (`enc_ans_simd.rs`): EstimateTokenCost with every
  `EstimateTokenCostImpl<E, M, L>` of the source's dispatch, its
  fallthrough branches and the `~0` result outside them. The unsigned
  comparisons and `Min` are formed exactly from signed lanes; `ConvertTo` of
  values below 2^22 is exact on every conversion. The caller's padding
  (`MaxVectorSize()` added as a count of words in `enc_ans.cc`) follows the
  encoder's target (16 or 32 words), since the last vector is loaded and
  stored whole.
- `enc_ma` (`modular/encoding/enc_ma_simd.rs`): FindBestSplit with the
  vector EstimateBits and `Padded` (the histogram length rounded up to the
  lane count). The scalar statements inside `HWY_NAMESPACE` have no product
  that is added to or subtracted from another term, so no contraction site
  differs from the scalar translation.
- `enc_modular_simd` (`enc_modular_simd_simd.rs`, next to the scalar
  `enc_modular_simd.rs`): the vector branch of EstimateCost, a different
  algorithm from the `HWY_SCALAR` branch (clamped-gradient residuals per
  lane, `estimate_cost_detail::ContextMap()` as the source's literal table,
  Condition and ShannonEntropy per histogram).

With `enc_rct` and `enc_cluster` the lossless effort-2 profile has every
Highway export it reaches translated (list above: `enc_rct`, `enc_cluster`,
`MaxVectorSize`). The lossy profile now also has `enc_cluster`,
`enc_entropy_coder`, `enc_ans_simd`, `enc_ma` and `enc_modular_simd`;
with `enc_chroma_from_luma` and `Symmetric5` on `main` (package
`encoder cfl`), the only encoder export it still lacks is
`enc_adaptive_quantization` (another package).

**Whole lossless encodes.** `enc_simd_lossless_thunder_frames_...`
encodes 136 images (1x1 to 300x260, two groups across, 513x3; gray and
RGB; 8 and 16 bits; flat, two-level, ramp, photo-like, noise and
region patterns) through `LosslessThunderConfig`'s frame encode with
`hwy::set_encoder_simd_for_test(Some(true))`, and compares the frame
bytes with the original's `EFThunderOneShot` (`df-source` of
tools/oracle/decode_frame.inc) built for the same single target:
byte-identical on NEON and AVX2. The scalar encoder path gives the same
bytes for every one of these images (the profile's SIMD exports change
clustering costs, not the chosen codes, on this corpus), so this test
shows agreement, not which path ran; the kernel families above do. The
default stays scalar (`ENCODER_SELECTS_SIMD = false`).

What a whole-encode comparison of the lossless profile against `cjxl`
built for one target still needs, for condition (c) of "Encoder path":
(1) the same through the public encoder (container and codestream
headers, `JxlEncoderFrameSettings` at effort 2, one-shot and streaming
input), i.e. the `encoder_public.cc` driver run against the SIMD oracle
with the encoder's selection forced, over a corpus at least as wide as
the scalar `cjxl` gate, alpha included; (2) for condition (b), planes
allocated with the target's `MaxVectorSize()` row padding inside the
encoder (they still follow `decoder_target()`, the scalar 4 bytes
outside decoding); the translated rows only read and write that padding,
so no output byte depends on it, but the allocation should match the
original before the default changes; (3) for AVX2, the same on real
x86-64 hardware (condition (e)).

**Evidence (2026-11-15, M1 Pro).** `tests/simd_encoder_entropy.rs`
against tools/oracle/encoder_entropy_simd.cc (and learn.inc,
decode_frame.inc) on NEON and, under Rosetta 2 with
`ROSETTA_ADVERTISE_AVX=1` for the SIMD test binary only, against the AVX2
dispatch-build oracle (`tools/check.py --stage oracle --target
x86_64-apple-darwin` with `SDKROOT` set): zero differing words.

| Family | Export | Requests | Words | Scalar differs (NEON / AVX2) |
| --- | --- | --- | --- | --- |
| `ene-rct` | `enc_rct` | 1,514 | 148,690 | 581 / 664 of 842 |
| `ene-hcond`, `ene-hent` | `enc_cluster` | 304 | 18,512 / 18,672 | 106 / 115 of 304 |
| `ene-hdist` | `enc_cluster` | 216 | 648 | (direct call) |
| `ene-fast` | `enc_cluster` | 180 | 103,132 | (direct call) |
| `ene-cluster` | `enc_cluster` | 324 | 172,772 | 288 / 252 of 324 |
| lossless frames | both | 136 | 272 | 0 of 136 |
| `ene-cost` | `enc_ans_simd` | 1,364 | 41,788 | 800 / 1,024 of 1,364 |
| `ene-tok` | `enc_entropy_coder` | 192 | 7,605,854 | 0 (integer, asserted equal) |
| `learn-estimate` | `enc_ma` | 360 | 360 | (direct call) |
| `learn-split` | `enc_ma` | 120 | 389,009 | 3 / 2 of 120 |
| `ene-mcost` | `enc_modular_simd` | 40 | 120 | 0 of 32 |

(The `ene-hcond` word count follows the lane count: Condition cuts to a
multiple of it.) The scalar paths of the RCT and EstimateCost families run
only the requests whose arithmetic the scalar source requires not to
overflow (the vector forms wrap). `ene-mcost`'s vector branch partitions
the same contexts and tokens as the scalar branch; only the order of the
entropy sums differs, which its truncated integer cost hides in this
domain. Rosetta does not fault on misaligned aligned moves: the hosted
Linux x86-64 run is still owed.
<!-- end encoder entropy -->

<!-- codegen -->
## One attribute context per kernel on x86-64 (2026-11-15)

**Finding.** The hosted benchmark of `79c6884b` (run 43688166701) measured
the port's lossy VarDCT decode at 4.0–4.6× libjxl's AVX2 build on both
x86-64 hosts, and 1.6–1.7× slower than libjxl's *scalar* build, while the
same code was 1.16–1.27× libjxl's NEON build on every AArch64 host. The
AVX2 path was selected (the conformance run 43690766156 logs `path=AVX2`
on native Linux; under Rosetta 2, `jxl_decode_compare` gives AVX2's output
words with `ROSETTA_ADVERTISE_AVX=1` and the scalar words without), but it
was compiled badly. `hwy::dispatch` runs a kernel inside
`avx2::run_with_features`, the only `#[target_feature]` function of the
layer. A generic helper of the kernel body that the compiler does not
inline is a function of its own and does **not** inherit those features;
inside it the `#[inline(always)]` operations of `Avx2` still inline, but
the `core::arch` intrinsics they call (`#[target_feature(enable = "avx")]`)
cannot, so every vector operation became an out-of-line call with its
`__m256` arguments and result passed in memory, followed by `vzeroupper`;
`f32::mul_add` of the partial vectors became a call of libm's `fmaf`. On
AArch64 NEON is part of the baseline and the same helpers inline normally,
which is why only x86-64 was slow.

The out-of-line helpers were exactly the ones the DCT leaves to the
compiler: the 81 per-shape `scaled_dct`/`scaled_idct` instantiations and
the 64-, 128- and 256-point levels (`#[inline]`), plus one closure of the
DC smoothing kernel. In the x86-64 release build of
`ferrisma-jxl-decoder`'s `jxl_decode_compare` (the benchmark's binary, the
repository's release profile, no RUSTFLAGS) at `b3f29a71`:
51,769 call instructions to `core::core_arch::x86` functions in 155
functions, 51,722 of them in `dct_simd` and 45 in `compressed_dc_simd`,
plus 4,590 calls of `fmaf`, against none in any other kernel. The original
does not have this hazard: `HWY_BEFORE_NAMESPACE()` gives every function of
`HWY_NAMESPACE`, inlined or not, the target's attribute.

**Decision.** Every function a kernel body reaches is compiled in the
target's attribute context, as under `HWY_ATTR`: either it is
`#[inline(always)]` (the operations and most helpers already are), or it
enters the target again through `SimdF32::run_in_target(kernel)`, which is
the kernel call itself on NEON and `avx2::run` (the `#[target_feature]`
function) on AVX2. Where the original leaves inlining to the compiler, the
port keeps `#[inline]` on the wrapper and moves the body into a small
`Kernel` that `run_in_target` runs, so the compiler's inlining choice stays
free and an out-of-line copy is still AVX2 code. On AArch64 the wrapper is
not generated (`cfg(target_arch = "x86_64")`): the method is the body, as
before. Applied to `dct_simd` (the per-shape transforms and the 64- to
256-point levels; `Part` now carries its target descriptor so that a level
of a partial vector can enter the context) and to the edge-pixel helper of
`compressed_dc_simd` (a nested `#[inline(always)]` function instead of a
closure). The arithmetic is unchanged: the same operations in the same
order, now as inline AVX2/FMA instructions (`vfmadd231ss` instead of
`fmaf`, both a single rounding).

**Check.** Count the call instructions to `core::core_arch::x86` in an
x86-64 release build (`objdump -d --no-show-raw-insn -C`); after the
change `jxl_decode_compare` and `conformance_decode` have none in any
kernel (the remaining ones are `std_detect`'s `xgetbv` and jxl-oxide's,
the comparison dependency). A new kernel helper that is neither
`#[inline(always)]` nor entered through `run_in_target` shows up there.

**Evidence (2026-11-15).** Outputs unchanged: the SIMD binaries of
`tools/check.py` with the conformance corpus on NEON (12 binaries, 79
tests) and on AVX2 under Rosetta 2 (12 binaries, 73 tests), all matching,
and the full test stage. Native x86-64, hosted run 43697684326: lossy
VarDCT decode 1.38–1.46× libjxl's AVX2 build (4.0–4.6× before, run
43688166701), the AArch64 rows unchanged; numbers and the Rosetta and NEON
before/after in the
[2026-11-15 addendum](../../../../docs/benchmarks/v1.0-jxl-decode-three-column-2026-11-14.md).

**Encoder kernels (not selected by default).** The same pattern remains in
the encoder's translated kernels: in `encode_lossy`'s x86-64 release build
4,069 such calls in 40 functions after this change (83,757 before; the DCT
was 79,475 of them): `enc_ans_simd` (`estimate_token_cost_d` and its ten
`estimate_token_cost_impl` instantiations, 2,570), `enc_modular_simd`
(`estimate_cost_d`, 145), `enc_cluster_simd` (142),
`convolve_symmetric5_simd` (142), `enc_chroma_from_luma_simd` (116),
`enc_group_simd` (58), `enc_ma_simd` (48), `enc_entropy_coder_simd` (44)
and `enc_transforms_simd` (`afv_transform_from_pixels`, 9). Before
`ENCODER_SELECTS_SIMD` is turned on for x86-64 they need the same
treatment. The port's skcms HSW engine (`skcms/transform_x86.rs`, reached
only by the encoder's color conversion in these binaries) has the same
shape for the same reason (794 calls: `exec_stages::<8>`, `clut::<8>`,
the transfer-function helpers): its `#[target_feature]` entry
`run_program_hsw` calls helpers that are not `#[inline(always)]`.
<!-- end codegen -->
<!-- encoder aq -->
### Encoder path: `enc_adaptive_quantization` (2026-11-14, branch `v1/jxl-enc-aq`)

**What the export is.** `enc_adaptive_quantization.cc` exports one Highway
function, `AdaptiveQuantizationMap`, dispatched from `InitialQuantField`;
everything else in the file is outside `HWY_NAMESPACE` or unreached.
Reading the code paths: the lossy profile (effort 7, Squirrel) calls
`InitialQuantField` (enc_heuristics.cc, speed tier Hare or slower), and
`FindBestQuantizer` returns without work, because its butteraugli loops
(`FindBestQuantization`, `FindBestQuantizationMaxError`, `RoundtripImage`,
`TileDistMap`) run only at Kitten and slower or in max-error mode. Those
are scalar code outside `HWY_NAMESPACE` that would call butteraugli's own
exports; they stay out of scope (and unported), as the profile list above
says. `InitialQuantDC` and `AdjustQuantField` are scalar code outside
`HWY_NAMESPACE` and keep the scalar translation on every target.

**Translated** (`enc_adaptive_quantization_simd.rs`): AdaptiveQuantizationMap
with AdaptiveQuantizationImpl::PrepareBuffers/ComputeTile (the
`HWY_FULL` vector loop of the pre-erosion pass with its scalar first and
last pixels), ComputeMask, RatioOfDerivativesOfCubicRootToSimpleGamma
(vector and one-lane forms), GammaModulation, BlueModulation,
HfModulation (kMaskRight), PerBlockModulations, MaskingSqrt (both forms)
and FuzzyErosion; FastLog2f and FastPow2f are `simd_transfer`'s.
`HWY_CAPPED(float, kBlockDim)` is the full vector on both targets (four
NEON, eight AVX2 lanes); the `HWY_CAPPED(float, 1)` forms are lane 0 of
the full vector operation on a broadcast (every operation they use is
lane-wise; their MulAdd is fused). The whole export lies inside
`HWY_NAMESPACE`, so the five contraction sites of
`tools/encoder_heuristics_contraction.json` (PerBlockModulations' final
scale, FuzzyErosion's weights and its four-product sum) are fused on both
targets, including x86-64, where the scalar build keeps them separate.
Products of two constants are folded before contraction and stay unfused
(`kSGVOffset * kInvLog2e + kEpsilon`, `64 * kLimit - scalar_sum`; the
latter is exact anyway). Blur1x1Masking is the scalar translation's (its
one contractible product, `2 * kFilterMask1x1[3]`, is exact). The SIMD
path dispatches at `initial_quant_field` through `hwy::encoder_target()`.

**What remains of this export: nothing.** Blur1x1Masking calls
`Symmetric5`, the separate `convolve_symmetric5` export, through its own
dispatch (`convolve_symmetric5_simd`, translated on another branch), so
with the encoder's selection forced the blur runs the SIMD Symmetric5 as
in the original. The parity test compares `mask1x1` after the blur.

**Tests.** `tests/simd_encoder_aq.rs`, request `eaq-map`
(tools/oracle/encoder_aq_simd.cc): the original's public
`InitialQuantField` on an `Image3F` whose row padding (up to
`PixelsPerRow`; the strides themselves differ, since the encoder's planes
keep the scalar padding, see above) holds one of four words (+0.0, a quiet NaN,
+inf, -1e30), because HfModulation's last vector reads one sample past
each block, masked out; status, `aq_map`, `mask` and `mask1x1` printed in
full. The domain: 14 whole images from 8x8 to 200x136 (one to four tiles
per direction, every SIMD loop remainder of both lane counts), nine value
kinds (photo-like, high-frequency noise, flat, ramps, blue-dominated
blocks for BlueModulation's branches, negative samples, large magnitudes,
special words sprinkled into photo content, special words only), ten
distances across the FuzzyErosion branch and the dampening ramp (0.05 to
20), three rescales, images larger than the rect by a partial block, and
six rects inside a 152x144 image (the +-2 and +-4 pixel extensions). It
runs through `initial_quant_field` with the encoder's selection forced and
shows the scalar path differs. An 8-lane structure test in the same file
(`Neon8`, two NEON halves, AArch64 only, no oracle) runs 108 requests:
every 8-lane load and store stays inside the planes, and `mask1x1`, which
only scalar pixels compute, equals the 4-lane result.

**Evidence (2026-11-15, M1 Ultra).** NEON: `eaq-map`, 286 requests,
1,203,950 words, zero differences (the scalar path differs in 277); the
8-lane test passes. AVX2 under Rosetta 2 (`ROSETTA_ADVERTISE_AVX=1`,
dispatch-build oracle): 240 of 286 requests match; **46 differ, all from
the two special-word value kinds, only in `aq_map`** (`mask` and `mask1x1`
match). Diagnosis of one of them (16x8, a signaling NaN in X and a quiet
NaN in Y within one block): removing either NaN makes it match. The two
NaNs reach different lanes of GammaModulation's `overall_ratio` and meet
in SumOfLanes, where `Add` of two NaNs returns one of them (x86: the first
source operand). FastLog2f then reads the NaN's bits as an integer, so the
payload becomes a finite exponent and the block's quant field differs by
orders of magnitude. The original's choice is not fixed by its source: in
the AVX2 object, Clang commuted the reduction step
`f(v03_12_12_03, Reverse2(...))` (`vshufps $0x1b` then `vaddps` with the
shuffled vector first), as LLVM may for any `fadd`, and the port's Rust
build makes its own choice. No source-faithful translation can promise the
original's words for inputs that put two NaNs into one block's reduction.

**Owner decision 2026-11-15** (#246, #250; the principle of the NaN-input
rule of #246): on AVX2 the `aq_map` words of a request are not compared
when the request's input puts two or more NaNs into the reduction of one
block, because the original's own result there is not defined by its
source. `mask` and `mask1x1` of those requests, every word of every other
request and every word on NEON stay compared exactly; no operand order is
pinned in the port. `tests/simd_encoder_aq.rs` computes the class from the
input (`map_nan_blocks`): for each block of the rect, GammaModulation's two
terms `r = (Y + 0.16) - X` and `g = (Y + 0.16) + X` over the block's 8x8
window (row-padding words past the image width), counting a term that is a
NaN or +infinity (whose ratio is inf / inf). It counts NaN terms with the
sign bit set as well, which ZeroIfNegative zeroes: the sign of a NaN the
arithmetic produces is itself the target's operand or default-NaN choice,
and inside one lane the accumulating `Add` of two NaNs is the same
compiler choice as across lanes in SumOfLanes, so the boundary is the
source's "two or more NaN-producing terms in one block's reduction". The
test asserts that the class is not empty. Result: the class selects 61 of
286 requests (2,455 blocks); on AVX2 all 46 differing requests are inside
it, the other 15 happen to give the original's words (equal payloads, or a
NaN zeroed by its sign); 4,295 `aq_map` words are not compared on AVX2,
1,199,655 words match; NEON compares all 1,203,950 words and matches. No
differing request lies outside the class.
<!-- end encoder aq -->
<!-- encoder selection -->
## Encoder selection (2026-11-15, branch `v1/jxl-enc-select`)

Addendum to "Encoder path", "Scope of the selection" and "Which path a test
runs". The encoder is prepared to select its SIMD path by default; the
switch stays off (`hwy::ENCODER_SELECTS_SIMD = false`).

**Selection and scope (condition (b)).** `DispatchScope` has a third state,
`Encoding(target)`. `JxlEncoder` stores the target it selects when it is
created (`hwy::encoder_default_target()`: `Target::detect()` when
`ENCODER_SELECTS_SIMD` is on), and the work calls (`add_image_frame_internal`
for AddImageFrame and AddChunkedFrame, the `ProcessOutput` and `FlushInput`
work loops) run inside `Encoding` with it, also from a decoder callback;
`LosslessThunderConfig::encode_pixels` (the engine's lossless writer) is an
entry point with the default selection (`hwy::encoding`). Inside it, as in
the original's one process-wide dispatch, `encoder_target()` (the encoder's
kernels), `decoder_target()` (the decoder code the encoder runs: quant
tables, the AR round trip and its pipeline, AdaptiveDCSmoothing) and
`max_vector_size()` (plane padding) all return the encoder's selection; the
decoder's override does not reach the encoder. `encoder_target()` is, in
order: the test override (`set_encoder_simd_for_test`), the scalar gate's
`JXL_ENCODER_TARGET=scalar` (read only by `cpp-oracle` builds, set by
`tools/check.py`), then the scope; outside every encoder entry point it is
the scalar path. AcStrategyHeuristics' DCT scratch follows the encoder's
lane count, like CfL's. `ThreadPool::run` carries the scope: the encoder
has no runner (SetParallelRunner is not translated; every encoder
`RunOnPool` gets no pool and runs on the calling thread), so no whole
encode runs on worker threads, and `dispatch_scope.rs`
`the_encoder_scope_reaches_every_runner_thread` shows with 0, 1, 2 and 8
workers that every init and task body sees the scope's selection in all
three queries. Flipping `ENCODER_SELECTS_SIMD` per target is the whole
change.

**Export missing from the profile list.** The whole-encode gate found one
decode-path export the lossy encoder runs that "Encoder exports the
engine's profiles reach" did not list: `compressed_dc`'s
`AdaptiveDCSmoothing`, which InitializePassesEncoder calls on the encoder's
DC (`enc_cache.rs`), unless the frame skips adaptive DC smoothing (it does
not at d2.9: a 128x128 gradient at d2.9 differed from the NEON original by
1 ULP in 93 DC words, the step-1 checkpoints located it). `enc_cache.rs`
called the scalar translation directly; it now dispatches as the decoder
does (`compressed_dc_simd`, on `decoder_target()`, the encoder's selection
in its scope). `simd_util`'s `MaxValue` (enc_ans.cc) is reached too; it is
an exact masked maximum of integers, so the port's scalar maximum gives the
same value on every target.

**Whole-encode gate (condition (c)).** `tests/simd_encoder_public.rs` runs
the scalar `cjxl` gate's public-encoder histories (every admitted family of
`tests/parity/encoder_public.rs`: small, large, settings, admissions with
the AR heuristics, streaming, alpha, patches off and text-like, extra
channel types, output processor, chunked input, the streaming suite, the
step-2 gate sizes up to 4000x3000), the engine's sequence (effort 7, patches
off, d1.0 and d2.0, gray and RGB, with and without alpha), the step-1
checkpoint histories (`es1-ckpt`: the state after LossyFrameHeuristics,
InitializePassesEncoder and ComputeARHeuristics, word for word), the
lossless effort-2 profile (`encode_pixels` against `ss-image`/`ss-image-alpha`
over the scalar whole-image corpus and larger images), and, when
`JXL_ENCODER_GATE_PHOTOS` names PPM files, photographs (the whole image and
three crops, d1.0 and d2.0, alpha on the crops, through the new `esel-ppm`
request of tools/oracle/encoder_selection.cc, and lossless). Each encode
runs with the SIMD target forced against the SIMD oracle and with the
scalar path forced, and records the call sites that selected SIMD
(`hwy::encoder_trace_start`): a family fails when a dispatch point its
profile reaches never selected the SIMD target. Results (M1 Ultra; AVX2
under Rosetta 2 with `ROSETTA_ADVERTISE_AVX=1`, against the AVX2 dispatch
build):

| Family | Encodes | NEON | AVX2 | Scalar path differs (NEON / AVX2) |
| --- | --- | --- | --- | --- |
| lossy (small, large, settings, admissions, smoke) | 524 | 524 | 524 | 41 / 39 |
| lossy streaming | 16 | 16 | 16 | 7 / 7 |
| lossy with alpha | 279 | 279 | 279 | 48 / 47 |
| patches off, text-like, extra channels | 177 | 177 | 177 | 18 / 18 |
| output processor, chunked, streaming suite | 328 (128 admission prefixes) | 200 + 128 | 200 + 128 | 102 / 102 |
| engine e7 d1.0/d2.0 | 64 | 64 | 64 | 29 / 27 |
| step-2 gate 768x768 to 2049x2049 | 32 | 32 | 32 | 29 / 29 |
| step-2 gate 4000x3000 | 8 | 8 | 8 | 8 / 8 |
| step-1 checkpoints (9,593,524 words) | 32 | 32 | 32 | 32 / 32 |
| lossless effort 2 | 532 | 532 | 532 | 1 / 4 |
| photograph 2268x1512, lossy / lossless | 14 / 8 | 14 / 8 | 14 / 8 | 10, 1 / 10, 0 |
| photograph 6000x4000, lossy / lossless | 14 / 6 | 14 / 6 | not run | 11, 0 / - |

Zero differing encodes on both targets (the 128 admission prefixes are the
streaming suite's histories outside the admitted profile, compared up to the
admission call as in the scalar gate). The scalar path's bytes differ from
the SIMD path's in many encodes, so the gate tells the paths apart; for
the lossless profile the clustering costs rarely change a chosen code.
Rosetta does not fault on misaligned aligned moves and is not x86-64
hardware: condition (e) is still open.

**Which path a test runs (condition (d)).** Every test of the files whose
tests reach an encoder entry point and compare with the scalar original
calls `hwy::select_scalar_encoder_for_test()` after
`select_scalar_decoder_for_test()` (`tests/parity/encoder_public.rs`,
`encoder_step1.rs`, `encoder_step1_checkpoints.rs`, `shared_primitives.rs`,
`whole_image.rs`, `tests/first_image.rs`, `frame_owners.rs`,
`vardct_image.rs`); `tools/check.py` sets `JXL_ENCODER_TARGET=scalar` as a
second guard. Fallout measured with the switch on for AArch64 and x86-64
(before these calls): the PR stage (`cargo test -p libjxl-rs --profile
checked`) 446 passed, none failed; `tools/check.py --stage test` without
`JXL_ENCODER_TARGET` 25 scalar-reference tests failed (21 of
`encoder_public`, `encoder_step1::upstream_speed_tier_test_squirrel`,
`encoder_step1_checkpoints`, `shared_primitives::complete_raw_codestreams…`,
`whole_image::alpha_complete_codestreams…`), every other test passed; with
the environment guard alone none fails. `dispatch_scope.rs` and
`simd_encoder.rs` were made independent of the switch.

**Conditions (a) to (e), 2026-11-15.**

- (a) Met on NEON and AVX2 (Rosetta 2): every export of the lossy and
  lossless profiles is translated with request-level parity, including
  `enc_adaptive_quantization` (on AVX2 under the owner decision of
  2026-11-15 for two NaNs in one block's reduction) and, found here,
  `AdaptiveDCSmoothing`.
- (b) Met: the `Encoding` scope above; the gate's trace shows the decoder
  code (`quant_weights`, `dec_group_render`, `enc_cache`, the AR pipeline's
  stages) and the plane padding (`memory_manager_internal`) selecting the
  target inside encodes.
- (c) Met on NEON and on AVX2 under Rosetta 2: the table above, a corpus at
  least as wide as the scalar `cjxl` gate plus photographs.
- (d) Met: the explicit selections and the environment guard.
- (e) Open: no run on x86-64 hardware. The hosted Linux x86-64 run of this
  branch is owed.

**Attribute context of the encoder's kernels.** The encoder's kernels enter
the target as the decoder's do: each dispatch point calls `hwy::dispatch`
(per call, per row or per block, where the original's export is called),
which on AVX2 runs the kernel inside `avx2::run_with_features`. Their
generic helpers had the defect of "One attribute context per kernel on
x86-64": in the x86-64 release `encode_lossy` 3,296 out-of-line intrinsic
calls in the port's encoder kernels (`enc_ans_simd` 2,570,
`enc_modular_simd_simd` 145, `enc_cluster_simd` 142,
`convolve_symmetric5_simd` 142, `enc_chroma_from_luma_simd` 116,
`enc_group_simd` 58, `enc_ma_simd` 48, `enc_entropy_coder_simd` 44,
`enc_adaptive_quantization_simd` 22, `enc_transforms_simd` 9) and 2,905 in
`encode_lossless`. Every generic helper an encoder kernel reaches is now
`#[inline(always)]`, and AdaptiveQuantizationMap's scalar-pixel closure is a
nested inline function: zero such calls and zero `fmaf` calls in the port's
functions of `encode_lossy`, `encode_lossless` and `conformance_decode`
(std_detect's `xgetbv` remains). `tools/check.py --stage static` now builds
those three x86-64 release examples and fails on any out-of-line `core::arch`
x86 or `fmaf` call in a `jxl::jxl::` function (about two minutes
incremental). The skcms HSW engine (`skcms/transform_x86.rs`, 799 calls in
`encode_lossy`, reached by the encoder's color conversion) is reported, not
held to zero: marking its helpers `#[inline(always)]` is not enough, because
`exec_stages` selects the transfer functions through a table of function
pointers (`apply_gamma`, `apply_tf`, `apply_pq`, `apply_hlg`,
`apply_hlginv`), builds values with `std::array::from_fn` closures (std's
`try_from_fn` stays out of line) and uses local closures in `exec_stages`
and `store_pixels`; it needs those turned into a `match`, loops and nested
inline functions, and its HSW engine cannot be compared with the original's
on this machine (O9), so it is left for a change with a hosted Linux run.
No x86-64 speed is claimed from Rosetta.

**Indication only (not a measurement).** One thread, the 6000x4000
photograph, release builds of `examples/encode_lossy` (e7 d1.0) and
`examples/encode_lossless` (e2) with the switch off (scalar) and on (NEON),
next to the original's public encoder from the NEON oracle build's archives
(no runner, e7 d1.0, and e2 lossless with buffering 0), five interleaved
repetitions under the gate lock on the shared machine (load average 85 to
24 during the run). Medians: lossy 47.2 s scalar, 13.2 s NEON, 9.9 s
original (port NEON about 1.3x the original); lossless 2.04 s scalar,
1.92 s NEON, 1.77 s original. No conclusion about x86-64 speed may be drawn
from Rosetta.

**The skcms HSW engine: an attempt that changed results (2026-11-15, not
committed).** With every helper of `transform_x86.rs` marked
`#[inline(always)]` and the function-pointer table of `exec_stages`
replaced by a selector `match`, 189 of the 799 out-of-line calls remained
in `encode_lossy` (the matrix rows built with `std::array::from_fn`, whose
`try_from_fn` stays out of line, and the `write` closure of
`store_pixels`); with those turned into loops and nested inline functions
too, the x86-64 release `encode_lossy` has no out-of-line intrinsic call
left in `jxl::skcms` (799 to 0). The results
changed, however: a scratch example ran the 135,236 requests of the
`skcms_transform` case lists (`tests/common/skcms_transform.rs`: formats,
color, alias, guard, program, half domain, rounding, partial context and
selection requests) on the HSW engine under Rosetta 2
(`ROSETTA_ADVERTISE_AVX=1`) before and after, and 657 observations differ
(the baseline engine's 135,236 are identical). For example Alpha_8 to
RGBA_8888_sRGB with one pixel writes R=G=B=-13 instead of 0: the inlined
transfer function took the branch for `x >= d` at `x = 0` and returned the
exponent offset `e`. Bisecting by `#[inline(never)]`: every change but the
selector keeps all 135,236 observations identical (inline-only, `write`);
the selector with `apply_tf`, `approx_pow` or the whole `apply` kept out of
line is identical again; `strip_sign`/`apply_sign` or `approx_exp2`/
`approx_log2` alone out of line is not. So `approx_pow`'s own body
(`equal`, the `[bool; N]` `from_fn`, `select`) behaves differently when
compiled inside the `avx,avx2,f16c` context than in the baseline context;
whether the cause is the compiler or Rosetta 2's AVX2 emulation is not
established. The change was therefore not committed. The HSW engine cannot
be compared with the original on this machine (O9: the cross-built
originals contain only the baseline engine). What it needs: an x86-64
original built with its HSW engine (a native hosted run, or an original
built with the HSW file), a native run of the scratch example before and
after, and then the change above, which is otherwise mechanical (about 60
lines of `transform_x86.rs`).
<!-- end encoder selection -->
