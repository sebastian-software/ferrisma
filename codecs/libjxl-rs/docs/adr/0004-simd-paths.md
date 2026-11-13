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
`SetSupportedTargetsForTest` (`hwy::set_decoder_simd_for_test`).

The selection is **off**. Working decision 2026-11-13 (delegated,
revisable by the owner): it is turned on per target, and only when on that
target

- (a) every decode-path Highway export of the Plan that an in-scope file
  can reach is translated. Upsampling, noise, blending, splines,
  `to_linear`, tone mapping, CMS, `dec_external_image`, `dec_modular`,
  `rct` and `squeeze` are still scalar; a file that reaches a scalar kernel
  while the rest runs SIMD matches neither reference build;
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
  stays off. No other kernel of this package takes a scalar detour on
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
  `DECODER_SELECTS_SIMD` selection stays off because not every file
  matches; the remaining kernels on that path are the next candidates
  (`compressed_dc`'s DC dequantization and adaptive smoothing, upsampling).
- **compressed_dc (2026-11-13, branch `v1/jxl-vardct-dc`).**
  `compressed_dc_simd.rs` translates AdaptiveDCSmoothing (one-lane edge
  pixels computed in lane 0 of a full vector) and DequantDC; the decoder
  dispatches both on `hwy::decoder_target()`. NEON parity
  (`tests/simd_vardct.rs`): `vdc-smooth` 507 requests / 94,796 words and
  `vdc-dequant` 384 requests / 200,664 words match (DequantDC
  columns past the rectangle width up to the next lane multiple are excluded:
  the original stores them from quantized samples it never initialized).
  With it, **all 24 admitted VarDCT files of the whole-decode comparison
  match the NEON oracle word for word** and no output byte differs. The
  selection is not turned on: this ADR requires the comparison on every
  target in scope, AVX2 is not verified (hosted run pending), and the one `DECODER_SELECTS_SIMD` switch would
  also move the AArch64 decoder off the scalar oracle that the scalar gate
  compares with. The whole-decode test now asserts these zero differences
  with the SIMD target forced; the conditions for the default selection are
  in "Decoder path".
<!-- end vardct kernels -->

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
<!-- end stages2 -->
