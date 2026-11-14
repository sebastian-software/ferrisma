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
`hwy::DECODER_SELECTS_SIMD` is on, and then only inside a decoder entry
point (see "Scope of the selection": the encoder stays on the scalar path);
tests switch it like Highway's `SetSupportedTargetsForTest`
(`hwy::set_decoder_simd_for_test`).

The selection is **on for AArch64 (NEON) and off for x86-64** since
2026-11-14 ("Default selection per target"). Working decision 2026-11-13
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
- The AVX2 oracle is a static AVX2 build, not upstream's dispatch build:
  code outside `HWY_NAMESPACE` is also compiled with FMA, so Clang may
  contract scalar `a * b + c` there, which an SSE2-baseline dispatch build
  cannot. Kernels inside `HWY_NAMESPACE` are identical in both. A dispatch
  build needs adapters that compile per target with `foreach_target`
  instead of including pristine `.cc` files after `hwy/highway.h`; until
  then whole-decode AVX2 comparisons of non-kernel scalar code must use a
  dispatch `djxl`, not this oracle.
- The SIMD stages load and store the pipeline's rows in place, through
  `RowBuffers::spans` (ADR 0003, "SIMD row spans", 2026-11-14), and copy a
  span only where it reaches outside its plane's storage.

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
  oracle is a static AVX2 build, so it also contracts scalar code outside
  `HWY_NAMESPACE` (for example RecomputeDCFactors), which upstream's dispatch
  build and the port do not (see "Consequences").
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

The selection stays off: condition (b) now has its comparison, which does
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
  The static AVX2 oracle compiles blending.cc with FMA (see
  "Consequences"), so an AVX2 comparison of `PerformBlending` against that
  oracle is not meaningful for an SSE2-baseline dispatch build.
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
## Color stages of Plan item 1 (2026-11-13, branch `v1/jxl-color`)

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
<!-- end color -->

<!-- enable -->
## Scope of the selection (2026-11-14, branch `v1/jxl-enable`)

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
## Default selection per target (2026-11-14, branch `v1/jxl-enable`)

`hwy::DECODER_SELECTS_SIMD` is `cfg!(target_arch = "aarch64")`: a decoder
created on AArch64 selects NEON, a decoder on x86-64 keeps the scalar path.
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
- (d) does not hold for AVX2 yet: there is no hosted x86-64 run of the
  combined branch, so x86-64 stays on the scalar path.

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
