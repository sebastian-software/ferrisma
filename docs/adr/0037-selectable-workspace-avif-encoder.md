# ADR-0037: Select the workspace AVIF encoder through the facade

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-15
- **Superseded in part by:** [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) (the comparison with rav1e is context, not a criterion); the owner decision of 2026-11-15 recorded [below](#the-only-avif-writer-2026-11-15-owner-decision) (the selection is gone: the workspace AOM writer is the only AVIF writer)
- **Related:** [0018](0018-optional-codec-and-cms-dependencies.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0031](0031-workspace-libavif-input-experiment.md)

## Context

The engine can receive libavif's AOM encoder only via `transcode_with` or the
adapter's separate CLI. The adapter depends on the engine to implement its row
contract, so the engine cannot depend on it without a Cargo cycle. A normal
facade and CLI caller needs to choose this existing backend while preserving
its budget, color, and error contracts. The reader has an independent safe
wrapper that establishes the needed dependency direction.

## Decision

Move the existing libavif write boundary into `ferrisma-avif-write`, a narrow
workspace crate that depends on the libavif port but not the engine. It owns
the C ABI calls and guards, validates requests, collects straight RGBA8 rows,
reports checked simultaneous RGBA and YUV(A) sample bytes, and returns owned
encoded bytes or typed errors. The existing `ferrisma-libavif` adapter delegates
to that bridge, preserving its backend API. No generated codec source changes.

*The default in this section was replaced on 2026-11-12; see
[Default switch](#default-switch-2026-11-12-pending-owner-confirmation).*

The engine's optional `avif-own-write` feature selects the same bridge through
its existing encoder backend and reservation seam. `AvifEncoder` explicitly
chooses Rav1e or libavif with 8- or 10-bit depth; the default stays Rav1e.
The normal native/nightly CLI exposes the same explicit selection. An explicit
external backend and a nondefault built-in selection are rejected together.
The CLI rejects any explicitly supplied selector or depth with an external
backend because its trait does not expose encoder identity or depth; the
programmatic default leaves the external backend authoritative.
The quality and speed scales belong to the chosen encoder and are not directly
comparable. Output remains full-range BT.709 primaries, sRGB transfer, BT.601
matrix, straight alpha and requested 4:4:4 or 4:2:0. Frame bytes are reserved
before allocation and held through encoding; encoded output and opaque codec
scratch are not bounded by that reservation. `max_output_bytes` is checked
after encoding. Codec-free and features without the native AVIF reader or
writer remain compatible with Rust 1.98; own AVIF requires the pinned nightly
and the supported native targets.

## Consequences

The common facade, batch API and CLI can select the own encoder without a
cycle. `TransformOptions` gains a public field: exhaustive struct literals
must supply it or use `..Default::default()`. This is a source compatibility
change to the experimental API. Existing default selection and quality scales
stay unchanged. The native writer retains its nightly and target restrictions.
The compatibility crate retains its backend type and CLI without selector flags.

## Alternatives and trade-offs

Keep only the separate adapter: avoids a new crate but prevents normal facade
selection. Depend on the adapter from the engine: creates a Cargo cycle. Put
unsafe in the engine: violates its no-unsafe contract. Duplicating the FFI
boundary would create diverging behavior. The small bridge adds a workspace
crate but lets both real callers use one audited implementation.

## Validation

ADR-0024 records the existing adapter's color, alpha, bit-depth, threading,
frame-budget and independent decode evidence. On macOS ARM64, the scoped
release adapter suite covers facade and compatibility routes, AVIF input and
output in one binary, 8/10-bit, 4:2:0/4:4:4, alpha, invalid settings, budgets,
output limits, and an independent ImageMagick/libheif oriented AVIF reference.
FFmpeg 9.0.2/libdav1d decoded one AVIF-to-AVIF CLI result and confirmed its
8-bit 4:2:0 and color signals. The JPEG-only stable Rust 1.98 test covers the
feature-disabled selection and pre-decode thread admission. This is scoped
integration evidence, not an encoder conformance corpus.

This selection is opt-in and does not complete ADR-0025's cross-architecture
corpus, quality, performance, memory, binary-size, or release/license gates.

## Default switch (2026-11-12, pending owner confirmation)

The owner's direction for v1.0 (2026-11-12) is to ship the engine on the
project's own four codec ports by default, keep external backends
selectable, and treat known performance gaps as follow-up issues. Under that
direction the recommendation of the
[non-timing evidence record](../benchmarks/M2-avif-encoder-default-evidence-2026-11-12.md)
is applied now, without waiting for its timing evidence:

- `AvifEncoder::default()` is `Libavif { depth: 10 }` whenever
  `avif-own-write` is compiled in, and `Rav1e` otherwise. `cli` already
  enables both, so the CLI encodes with libavif/AOM unless
  `--avif-encoder=rav1e` is given. `--avif-depth` alone applies to the
  default libavif encoder. Feature unification can change the default of a
  build that enabled only `avif-write`; callers that need a fixed encoder
  select it explicitly.
- **Quality scale (the evidence record's decision 1):** no mapping. Each
  encoder keeps its native scale and `quality`/`speed` are passed unchanged
  to the selected encoder only: Rav1e 1–100 through `ravif`'s curve and speed
  1–10; libavif 0–100 rounded to its 64 quantizer levels and `cpu-used`
  0–10. The numbers are not comparable: rav1e 70/80/90 matched roughly AOM
  50/55/78 at equal SSIMULACRA2 on the recorded photographs, so an unchanged
  `quality` now yields larger, higher-scoring files (rav1e q80 on the
  800×533 flower: 55,682 bytes, SSIMULACRA2 77.455; AOM 10-bit q80: 100,512
  bytes, 88.307). This is a documented scale change of the experimental API,
  stated in the rustdoc of `AvifEncoder` and `TransformOptions::quality` and
  in the CLI usage line. The default `quality` stays 75 on whichever scale is
  selected.
- **Alpha quality (decision 2):** AOM's behavior is kept: alpha uses the
  color quality (`qualityAlpha = quality`), whereas rav1e encodes alpha at a
  fixed quality 80. **Bit depth:** AOM defaults to 10 bits and accepts 8;
  rav1e is always 10-bit. Output color signaling stays full-range BT.709
  primaries, sRGB transfer, BT.601 matrix; AOM additionally writes a container
  `colr nclx` box that rav1e's serializer omits.
- **Determinism (decision 3):** not required for v1.0. AOM bytes are not
  demonstrated to be identical across ARM64 and x86-64 at every setting; the
  output contract is the documented semantics, not identical bytes
  (ADR-0040 decision 3).
- **Evidence relied on:** reference-encoder origin (ADR-0025 point 1); smaller
  estimated bytes at equal SSIMULACRA2 than rav1e speed 6 in 23 of 24 speed-6
  cells on four images (macOS ARM64), consistent with the earlier x86-64
  runs; independent FFmpeg/libdav1d, libheif and rav1d decodes of every
  semantics output; about 1.5 MB smaller binaries when the own AVIF reader is
  linked.
- **Gates that stay open:** wall time with one thread and all cores and peak
  RSS on quiet native ARM64 and x86-64 hosts, with at least one 12–48 MP
  photograph; the hosted golden evidence (#134); a direct YUV-to-AOM path
  (#60); a stable toolchain or targets beyond macOS ARM64 and GNU Linux x86-64
  for the own writer. rav1e is not removed (ADR-0025 point 4); it stays the
  AVIF encoder for stable-toolchain and other-target builds.
- **Pending owner confirmation:** applied on a best estimate of the stated
  direction; reversible by restoring the Rav1e `Default` of `AvifEncoder`.

Tests: `pipeline_tests::a_backend_receives_the_request_and_every_output_row`
asserts the default encoder name per feature set;
`hardening_tests::api_rejects_invalid_quality_and_mismatched_input_format`
checks each encoder's own quality range (quality 0 is valid only for
libavif); `tests/own_avif_cli.rs` checks the CLI default, `--avif-depth`
alone, explicit rav1e, and the per-encoder quality range; the
`ferrisma-libavif` facade test checks that only a nondefault selection
conflicts with WebP output or an external backend.

## The only AVIF writer (2026-11-15, owner decision)

**Decision.** rav1e leaves the engine; the workspace libavif/AOM writer is the
only AVIF writer, so there is nothing left to select. Removed: the rav1e sink
(`src/avif_sink.rs`, its quality/speed mapping `src/av1_tuning.rs` and its
tests `src/avif_sink_tests.rs`), the dependencies `rav1e` (with `asm` and
`threading`) and `avif-serialize`, `AvifEncoder` (`Rav1e`,
`Libavif { depth }`), `TransformOptions::avif_encoder`, the CLI flag
`--avif-encoder=`, the fallback "rav1e without the workspace feature", and
the `encoder_backend` value `"rav1e"`. The AV1 depth, formerly carried by
`AvifEncoder::Libavif`, is `TransformOptions::avif_depth` (8 or 10, default
10; _working decision 2026-11-15, delegated, revisable by the owner_, kept
deliberately small because the per-format options type follows in a
separate change). A nondefault depth with another output or an external
backend is refused, as a nondefault selection was. `quality` and `speed`
keep their meaning for the AOM encoder (0–100, 0–10). `transcode_with`, the
`EncoderBackend`/`RowEncoder` traits and `LibavifBackend` are unchanged
(experimental tier); the built-in writer is that backend.

**Why.** rav1e brought a second quality scale that is not comparable with
libavif's (#216: rav1e 70/80/90 ≈ libavif 50/55/78), bytes that depend on the
thread count and host, AVX-512 assembly outside the v1.0 SIMD scope
(ADR-0041, ADR-0020 D6), and no gate against an original, while the v1.0 bar
is bit identity with the ported original (ADR-0041). It was a transitional
technology from before the workspace port existed.

**Feature name.** _Working decision 2026-11-15 (delegated, revisable by the
owner):_ the workspace writer takes the shorter name `avif-write`, as
`webp-write` and `jpeg-xl-write` name the only writer of their format;
`avif-own-write` is removed without an alias (pre-1.0). The name changed in
the root `Cargo.toml` (`features`, `cli`), `crates/ferrisma-libavif/Cargo.toml`,
`src/`, `tests/` and `crates/ferrisma-libavif/tests/` cfg gates,
`src/cli.rs` (`--version` feature list), `.github/workflows/ci.yml` (`rust`
feature loop), `.github/workflows/bench-v1-perf.yml`,
`examples/cli_subset.rs`, the README, CONTRIBUTING, `docs/status.md`,
`docs/compatibility.md`, `docs/supported-matrix.md`, `docs/architecture.md`,
`docs/getting-started.md`, `docs/codecs.md`, `docs/publishing.md` and
`crates/ferrisma-avif-write/README.md`. Dated records keep the old names.

**Known cost.** `avif-write` built on stable Rust 1.98 with rav1e; the
workspace writer needs the pinned nightly on native macOS ARM64 or GNU
Linux x86-64, as AVIF input already does. A stable-toolchain build can
neither read nor write AVIF until the workspace AVIF port builds on stable
Rust (the task that follows). _Working decision 2026-11-15 (delegated,
revisable by the owner):_ `avif-write` moves from the stable to the
experimental tier until then (ADR-0040). CI's `minimum-rust` stable set
drops `avif-write`. NASM was needed only for rav1e's assembly: the
workspace AOM port is a Rust translation without an assembler step, so the
engine's CI jobs (`rust`, `minimum-rust`) and the fuzz workflows no longer
install it (the codec jobs that build the C originals keep it).

**Tests.** Removed with rav1e: `avif_sink_tests` (opaque output in both
chroma layouts, the alpha item, size-derived tiles and thread-count
independence, the budget charge), `supported_matrix::rav1e_writes_ten_bit_without_a_container_colr`,
`rav1e_writes_an_alpha_item_for_transparent_input`,
`rav1e_quality_and_speed_start_at_one`,
`tests/report_backends.rs::explicitly_selected_rav1e_is_reported`, and the
rav1e halves of `hardening_tests::api_rejects_invalid_quality_and_mismatched_input_format`,
`pipeline_tests::a_backend_receives_the_request_and_every_output_row`,
`tests/own_avif_cli.rs` and the `ferrisma-libavif` conflict tests. Re-pointed
to the workspace writer: the AVIF reader tests in `src/codecs/avif.rs` and
`hardening_tests::avif_decoder_rejects_truncated_container_without_panicking`,
which used the rav1e sink to write their fixtures
(`own_avif::encode_for_test`), and the output-limit, metadata and
selection-conflict cases of `supported_matrix` (now `avif_depth`). The CLI
tests assert that `--avif-encoder=` is an unknown option. The test-only
`ravif` dev-dependency, which wrote one identity-matrix fixture of the AVIF
reader tests, was removed in a follow-up the same day: the fixture is now the
committed file `tests/fixtures/avif-color/identity-rgb-10bit.avif`, so no
rav1e code is built anywhere in the root workspace (only the third-party
crates of that fixture's one-time writing are gone from `Cargo.lock`).

## Native AVIF options (2026-11-15, owner decision)

**Decision.** Encoder controls are native per format (owner decision of
2026-11-15, [ADR-0043](0043-formats-behave-alike.md) D4; #216). The AVIF
writer's settings move out of the format-independent looking fields of
`TransformOptions` into `AvifOptions`, parallel to `WebpOptions` and
`JpegXlOptions` ([ADR-0040](0040-stable-public-contract-and-compatibility-policy.md)
recommendation 2): `quality` and `alpha_quality` (libavif's `quality` and
`qualityAlpha`, integers 0–100, passed unchanged; the former `f32` was
rounded), `speed` (AOM `cpu-used` 0–10), `depth` (8 or 10; formerly
`TransformOptions::avif_depth`) and `chroma` (formerly
`TransformOptions::chroma_subsampling`). `TransformOptions::quality`,
`speed`, `chroma_subsampling` and `avif_depth` are removed, as are the CLI
flags `--quality=`, `--speed=` and `--chroma=`; the AVIF flags are
`--avif-quality=`, `--avif-alpha-quality=`, `--avif-speed=`,
`--avif-depth=` and `--avif-chroma=`. A nondefault `AvifOptions` with
another output is refused before decoding, as other foreign settings are.
`EncodeRequest` carries `avif` instead of `quality`, `speed` and `chroma`,
so an external backend receives the depth too; `LibavifBackend` no longer
carries a depth of its own (`LibavifBackend { depth }` was a second control
for the same setting, ADR-0040 problem 11) and is a unit struct.

**Defaults: the original tool's.** The defaults are those of `avifenc` in
the pinned libavif source, for the 8-bit RGB rows the writer receives:
quality 60, alpha quality 60, speed 6, depth 8, 4:4:4. Sources, read from
`codecs/libavif-rs/upstream/libavif` (libavif 1.4.2, revision `9b07a9d7`, `UPSTREAM.json`): quality 60 (`apps/avifenc.c:861` `#define DEFAULT_QUALITY 60`, applied at line 2353), alpha quality = the color quality when not given (`apps/avifenc.c:2369` `fileSettings->qualityAlpha = fileSettings->quality;`), speed 6 (`apps/avifenc.c:1514` `settings.speed = 6;`), depth = the input's, 8 for 8-bit input (`apps/shared/avifjpeg.c:1340` and `:1376` `avif->depth = requestedDepth ? requestedDepth : 8;`, `apps/shared/avifpng.c:376`-`379` for 8-bit PNG), chroma 4:4:4 for RGB input (`apps/shared/avifutil.h:141` `#define AVIF_APP_DEFAULT_PIXEL_FORMAT AVIF_PIXEL_FORMAT_YUV444`, used at `apps/shared/avifjpeg.c:1374` and `avifpng.c:373`). The alpha quality is explicit instead of following the
color quality: `avifenc` copies the color quality only when no alpha
quality is given, so its default for the default color quality is 60, and
`AvifOptions::alpha_quality` keeps that value when only `quality` is
changed. The default depth changes from 10 to 8 and the default quality
from 75 to 60; default AVIF output therefore changes.

**Depth: what `avifenc` does, and the engine's fixed default.** `avifenc`
has no fixed depth: without `--depth` it writes the depth of its input.
Its JPEG reader delivers 8 bits (`apps/shared/avifjpeg.c:1340` and `:1376`,
`avif->depth = requestedDepth ? requestedDepth : 8;`), its PNG reader the
PNG's (8 for 8-bit PNG, otherwise 12: `apps/shared/avifpng.c:376`-`382`),
and Y4M input keeps its own (`apps/avifenc.c:706`). The engine's writer
always receives 8-bit rows, whatever the input's depth, so "the input's
depth" has no single equivalent; the engine uses a fixed default of 8,
which is what `avifenc` writes for the inputs its readers deliver at 8
bits. _Working decision 2026-11-15 (delegated, revisable by the owner),
submitted to the owner for confirmation:_ the fixed default 8 (the
engine's default was 10 before). A deeper input is reported, whatever the
output depth, as `Discarded::PrecisionReduced`
(`discarded_report::avif::deeper_input_at_the_default_avif_depth_reports_precision_reduced`).

## Review when

Reassess the selection and default when ADR-0025 evidence is complete, the
libavif port gains a stable toolchain or safe native API, the supported target
set changes, or a streaming encoder can demonstrably replace the full frame.
Re-evaluate the default switch if the owner rejects the unmapped quality
scale, if quiet-host timing shows a regression the owner does not accept, or
if hosted x86-64 golden evidence fails.

## Amendment log

- 2026-11-11 — Added opt-in facade and CLI selection through an independent
  safe writer bridge. The original Rav1e default and selection gates remain.
- 2026-11-12 — Non-timing default-encoder evidence for issue #53
  ([record](../benchmarks/M2-avif-encoder-default-evidence-2026-11-12.md)).
  Both writers emit full-range 1/13/6 in the AV1 sequence header; only AOM
  also writes a container `colr nclx` box. Rav1e is always 10-bit and encodes
  alpha at a fixed quality 80; AOM uses 8 or 10 bits and the color quality
  for alpha. The quality scales differ (rav1e 70/80/90 matched AOM about
  50/55/78 here), so a default switch needs a quality mapping or a documented
  scale change. Independent FFmpeg/libdav1d, libheif and rav1d decodes
  accepted every output. AOM output differed by a few bytes between this
  ARM64 host and the earlier x86-64 smoke at two settings, while rav1e bytes
  matched. The record recommends switching the default to AOM after the
  pending timing evidence, keeping rav1e as an explicit non-default backend.
  This amendment does not change the default.
- 2026-11-12 — Under the owner's v1.0 direction, `Libavif { depth: 10 }`
  becomes the default whenever `avif-own-write` is compiled in; rav1e stays an
  explicit backend. Quality scales stay native and unmapped (documented scale
  change), AOM's alpha-follows-color quality is kept, cross-host byte identity
  is not required. Timing, peak RSS and hosted golden evidence stay open;
  pending owner confirmation. The earlier default rationale is kept above.
- 2026-11-12 — Indicative loaded-host timing in the
  [v1.0 baseline](../benchmarks/v1.0-baseline-2026-11-12.md#2-avif-encode-rav1e-speed-6-versus-workspace-aom-speed-67-53):
  interleaved, 7 samples, six photos up to 60.2 MP, rav1e 70/80/90 and matched
  AOM qualities. On one thread AOM speed 6 was faster than rav1e speed 6 in
  every accepted cell except the 42.2 MP elcito photo (encode +5 % to +84 %);
  AOM speed 7 was faster everywhere. AOM process RSS was higher on most images
  with all cores. Nine cells were rejected by the 0.5 SSIMULACRA2 gate and
  stay recorded. Idle-host ARM64/x86-64 timing remains open; the default is
  unchanged.
- 2026-11-12 — The writer bridge's RGB-to-YUV conversion runs in row bands
  when more than one encoder thread is requested: one band per thread, at
  most 8, each at least 64 rows; every band but the last has an even height,
  so bands start on even rows (a 4:2:0 view must, and each 2×2 chroma average
  reads the same rows as in the whole image). Each band converts into an
  `avifImageSetViewRect` view of the parent's preallocated planes on a scoped
  thread; one encoder thread keeps the serial whole-image call. The samples
  are identical for every band count
  (`ferrisma-avif-write` `tests::conversion_bands_do_not_change_samples`: 4:4:4
  and 4:2:0, 8 and 10 bits, with and without alpha, odd and even sizes, 1, 2,
  3 and 8 bands), and the CLI's AVIF bytes did not change at any tested
  setting. Interleaved A/B, 6000×4000 JPEG, all cores, 7 repetitions,
  **indicative; loaded host** (load 12.7–16.9): encode stage 229.9 → 216.3 ms
  (1600×1067 4:4:4), 192.7 → 180.3 ms (4:2:0), 1,335.7 → 1,159.3 ms (no
  resize); one encoder thread unchanged. The unsafe boundary grows by the
  plane allocation, view creation and view destruction, documented in
  [the unsafe review](../unsafe-review.md) and at each block
  ([analysis](../benchmarks/v1.0-engine-performance-analysis-2026-11-12.md#applied-fixes-2026-11-12-evening)).
- 2026-11-13 — Owner decision D1, recorded in [ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md): the workspace codec ports are the default paths even where they are slower than third-party libraries. The libavif/AOM writer therefore stays the default whenever `avif-own-write` is compiled in, and the comparison with rav1e (time, RSS, bytes at matched quality) is context, not a criterion; rav1e stays an explicit backend. Quiet-host timing and peak RSS are no longer gates for the default (D3); speed is reported against C libavif + libaom in ADR-0041's three columns. Still open and not decided by this amendment: the default AVIF quality per encoder (#216) and the other confirmations listed as ADR-0041 O5. The quality gate now applies to the writer's AOM port: its output must equal libaom's own builds on each target, including libaom's NEON build on AArch64, where the [CI record](../benchmarks/v1.0-ci-performance-2026-11-12.md#correctness-side-checks-all-hosts) found a difference against the release build. The hosted golden evidence (#134) stays a regression guard.
- 2026-11-15 — Owner decision: rav1e leaves the engine and the workspace AOM writer is the only AVIF writer; `avif-own-write` is renamed `avif-write` and the depth moves to `TransformOptions::avif_depth` (working decisions, delegated). Recorded in [The only AVIF writer](#the-only-avif-writer-2026-11-15-owner-decision); the sections above are kept for history.
- 2026-11-15 — Native AVIF options (owner decision; section above):
  `AvifOptions` with quality, alpha quality, speed, depth and chroma on
  libavif's scales and `avifenc`'s defaults (60, 60, 6, 8, 4:4:4, cited
  from the pinned source); `avif_depth`, `quality`, `speed`,
  `chroma_subsampling` and the CLI flags `--quality=`, `--speed=`,
  `--chroma=` removed; `LibavifBackend` without its own depth. Closes #216.
- 2026-11-15 — The workspace AVIF port builds on stable Rust 1.99 or later (libavif-rs [ADR 0060](../../codecs/libavif-rs/docs/adr/0060-build-the-translations-on-stable-rust.md)): the "pinned nightly" requirements above and the known cost of the rav1e removal (no AVIF on a stable toolchain) no longer apply; the native-target restriction stays. CI checks `avif-read`, `avif-write` and `cli` with `cargo +1.99.0`. Working decision 2026-11-15 (delegated, revisable by the owner).
- 2026-11-15 — The fixed default depth 8 (working decision above) is replaced by the owner's decision of the same day: the depth follows the input, as `avifenc` does (`apps/shared/avifpng.c:376`-`382`: 8 for 8-bit input, 12 for deeper PNG; Y4M keeps its own, `apps/avifenc.c:706`), 8, 10 or 12, and the writer receives `u16` rows at that depth (`avifRGBImage::depth` = image depth); `AvifOptions::depth` is `Option<u8>`. The bytes of a 10- and a 12-bit encode equal the C original's for the same samples (`ferrisma_avif_write::deep_output_equals_the_c_original`, `tests/fixtures/avif-deep/`). 8-bit input keeps its bytes. See the ADR-0021 amendment.
