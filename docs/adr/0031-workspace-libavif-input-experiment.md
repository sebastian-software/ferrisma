# ADR-0031: Experimental AVIF input through the workspace libavif port

- **Status:** Accepted
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-26
- **Partially supersedes:** [0018](0018-optional-codec-and-cms-dependencies.md), the interim AVIF reader and its toolchain contract; [0024](0024-pluggable-encoders-and-libavif-rs-backend.md) and [0026](0026-codecs-in-one-repository-and-workspace.md), the AVIF reader/encoder link exclusion. [0025](0025-codec-selection-reference-conformance-and-image-scope.md)'s final selection gates remain in force.
- **Related:** [0015](0015-typed-output-color-signals.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0019](0019-source-side-orientation.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [issue 54](https://github.com/sebastian-software/ferrisma/issues/54)

## Context

The former interim `avif-decode` reader used `rav1d`, whose `dav1d_*` C symbols collided
with the workspace libavif port's dav1d. It also materializes an opaque RGB
frame and requires three passes over the container. The workspace port exposes
libavif's raw C-style API and includes its Rust libaom reference decoder.
Moving the raw API into Ferrisma's engine would violate its `unsafe` rule.

## Decision

Introduce `ferrisma-avif-read`, a small crate that depends on the local
`libavif-rs` package but not on Ferrisma. It owns the decoder and memory reader,
sets the AV1 decoder explicitly (`AVIF_CODEC_CHOICE_AOM` until 2026-11-22;
since then `AVIF_CODEC_CHOICE_DAV1D` by default and libaom on request, see the
amendment of 2026-11-22), guards cleanup, and writes interleaved
RGB or RGBA into a caller-owned buffer. It selects the primary image item
explicitly and rejects files carrying tracks, so container color, orientation,
and extent metadata refer to the decoded item. Ferrisma's private AVIF source owns the
pixel buffer under `MemoryBudget`, reports a full-frame fallback, and borrows
rows from it. AVIF input remains an optional, native-target
feature (it builds on stable Rust 1.99 since 2026-11-15, libavif-rs ADR 0060). The codec-free engine and
features without `avif-read`/`cli` retain stable Rust 1.99; the engine contains no `unsafe`.

The independent `avif_meta` parser still reads container color and transform
properties. A handwritten same-module libavif helper prepares the selected
primary color tile samples and calls the port's AV1 sequence-header parser
before pixel decode. The adapter rejects contradictory container and bitstream color,
unsupported `clap`, unsupported transform order, monochrome and animation.
The independent parser caps ICC copies at 16 MiB and uses fallible allocation;
the wrapper applies the same cap to libavif's retained ICC.
libavif's YUV-to-RGB conversion uses nearest-neighbor chroma upsampling and
emits full-range RGB, so the rendered CICP
descriptor has RGB matrix code 0 and full-range flag 1; the original ICC is
retained only if the decoded output's color meaning is unchanged. For 10/12-bit
input, a managed U16 frame is normalized to F32 rows. RGB alpha output is
requested straight. Orientation remains in Ferrisma's source wrapper, with
crops in upright display coordinates. Only AVIF `irot` and `imir` determine
display orientation; EXIF payload orientation is not applied or forwarded by
this pipeline. The decoded image's chroma siting is interpreted by libavif's
nearest-neighbor YUV-to-RGB converter.

This is an integration experiment, not completion of issue 54. Its requested
Argon evidence for dav1d was recorded on 2026-11-23 (amendment of that date).
Explicit libaom selection follows
ADR-0025's reference-origin route, but it does not establish complete AV1
conformance or justify a universal performance claim.

## Consequences

- The encoder adapter and AVIF input can link without selecting `rav1d`.
- The engine budget covers its full RGB output and F32 row scratch, while
  libavif and libaom scratch and retained compressed data remain opaque.
- An AVIF input build requires supported macOS ARM64 or GNU Linux x86-64 targets (stable Rust 1.99; the nightly requirement ended on 2026-11-15).
- `avif-parse` is removed. The handwritten sample helper is included in the
  port's generated module selectors; translated `read.rs` bytes and hashes
  remain unchanged. Selector hashes are updated in the translation manifests.

## Alternatives considered

- Use the port's dav1d by default. Deferred on 2026-11-11 because its
  optional external Argon conformance corpus had not been run; adopted on
  2026-11-22 by owner decision on bit-identity evidence; its Argon evidence was
  recorded on 2026-11-23 (amendment of that date).
- Put raw calls in the engine. Rejected by its no-`unsafe` policy.
- Put the reader in `ferrisma-libavif`. Rejected because that encoder adapter
  already depends on Ferrisma, creating a Cargo dependency cycle.

## Validation

Focused source tests cover color conflict, primary-item versus track selection,
ICC copy limits, 8/10/12-bit normalization,
alpha, malformed input, pixel limits, budget release, selected color grid
tiles, and AVIF orientation fixtures from ImageMagick/libheif. The selected raw
output matches libavif's pinned C oracle byte for byte on eight fixtures. A
frozen pre-format-neutral harness at reviewed base `f0ffebf4` retains the
interim-reader comparison path after removal from the live dependency graph.
Scoped PR CI checks the combined reader/encoder feature graph on native GNU
Linux x86-64 and macOS ARM64. Wider quality and memory comparison remains a gate
for a final decoder selection under ADR-0025 and issue #54; the Argon/dav1d
corpus was run on 2026-11-23 (amendment of that date). The current own-reader
default is experimental; it
does not mean those gates passed. Measure representative full-frame and crop
workloads with versions, commands, time, and peak RSS; report the differences
only for those workloads. The
eight-fixture pinned C byte parity and the measured flower-photo comparison
are recorded in [M2 AVIF input evidence](../benchmarks/M2-avif-input.md).

## Review when

The libavif port gains a stable toolchain or safe API; the dav1d Argon corpus
is validated; module generation or sample ownership changes; native parity or
measured behavior changes the decoder choice; or
an AVIF feature needs another platform.

## Open questions

- Alpha without `irot` is not decided to fail closed (no issue). AVIF decode memory outside the managed budget: #199.

## Amendment log

- 2026-11-11 — Introduced the bounded-output experimental adapter with
  explicit libaom decoding. It is the live AVIF input path on supported
  native/nightly builds. The frozen interim harness preserves comparison
  evidence while ADR-0025's final selection and issue #54 remain open.
- 2026-11-12 — **Independent cross-check and decision state for issue #54.**
  On macOS ARM64 at `54da920e`, 43 fixtures were compared with libheif
  1.23.7 (own HEIF parser, own `irot`/`imir`/`clap` handling, own
  nearest-neighbor YUV-to-RGB, C libaom 3.15.3 plugin), ImageMagick (ICC
  bytes) and FFmpeg 9.0.2/libdav1d ([evidence](../benchmarks/M2-avif-input.md#independent-reader-cross-check-2026-11-12)).
  Depth, alpha presence, container CICP, ICC bytes, upright size for all eight
  orientations and grids agree; engine sRGB8 rows differ by at most one code
  on visible pixels across 8/10/12-bit and 4:2:0/4:2:2/4:4:4. `clap`,
  monochrome, animation, PQ/BT.470M color and malformed files are rejected.
  A new test asserts equal layouts of the generated `avifImage` and
  `avifRGBImage` copies the wrapper casts between, and a `compile_fail`
  doctest shows that the decoder cannot outlive its input.

  **Argon/dav1d requirement.** The Argon AV1 conformance suite is not in the
  repository and was not obtained: it is an external multi-gigabyte stream
  set whose download and terms need the owner's approval. What exists instead:
  (1) the live reader uses libaom, the AV1 *reference* decoder, which
  satisfies ADR-0025 point 1 by origin, not by conformance; (2) the Rust port
  equals pinned C libaom on the eight raw-output fixtures above and passes the
  complete original libaom Release suites (libavif-rs ADR 0048), whose test
  sources include libaom's decoder `test_vector_test.cc` (its retained
  per-case reports were not re-inspected here); (3) the workspace dav1d port,
  which the reader does **not** select, passed all 1,711 dav1d meson tests on
  macOS ARM64 and GNU Linux x86-64 in nightly run 43194200648, including the
  seven Argon streams pinned in `dav1d-test-data` (10-bit, 12-bit and
  multi-bit) — a sample, not the suite; (4) FFmpeg/libdav1d decodes agree
  with the reader within scaler rounding. **Decision state:** the Argon gate
  is not waived. It remains a precondition for selecting the dav1d port, or
  any other non-reference decoder, as the AVIF reader. The libaom selection
  relies on the reference-origin route instead; under issue #54 that
  different route must be confirmed explicitly by the owner.

  **Remaining limitations.** The reader stays experimental. It needs the
  pinned nightly and macOS ARM64 or GNU Linux x86-64. Decoding is full-frame
  for crops, and libavif/libaom scratch is outside the engine budget. Files
  whose alpha item lacks the color item's `irot`/`imir` are decoded like
  libavif does (alpha rotated with color); libheif disagrees, and Ferrisma does
  not fail closed on this nonconforming case. `clap`, monochrome, animation and
  CICP outside the engine's color policy are rejected. The cross-check ran on
  ARM64 only. The 2026-11-11 full/crop latency and RSS comparison is the only
  retained resource measurement (one photo, ARM64); new timing was not
  measured on the shared host. The 2026-11-12 nightly `root` generate jobs
  failed with "The production package compiled a different translation than
  the native profile"; that port-level check needs its own follow-up.

- 2026-11-12 — **Threads and decode memory** ([engine performance analysis](../benchmarks/v1.0-engine-performance-analysis-2026-11-12.md)).
  The bridge now passes the resolved transform thread count to libavif's
  nearest-neighbor YUV-to-RGB conversion (`avifRGBImage.maxThreads`, at most
  eight row jobs; one thread keeps the serial path). The jobs own disjoint
  rows; `decode_threads_do_not_change_samples` checks 8-bit 4:4:4/4:2:0 and
  12-bit 4:4:4 files on 1, 2, 3 and 8 threads. The AV1 decode stays on one
  thread: libavif's `decoder.maxThreads` above one enables libaom's row-based
  multithreading, which halved a 24 MP decode stage but raised peak RSS from
  about 0.42 to 1.8 GB at any thread count (indicative, loaded host). A live
  allocation snapshot of a 24 MP 8-bit 4:2:0 decode attributes the opaque
  memory to libaom: 271 MB mode-info grid (11.3 bytes per pixel) and a 38 MB
  frame buffer; C libavif with libaom 3.15.3 shows the same RSS (445 MB).
  Re-evaluate when libaom's row-MT can be disabled separately or when dav1d
  is selected.
- 2026-11-15 — **Owner decision on issue #54, both parts.** (1) **Reader
  evidence:** bit identity with libaom, the AV1 reference implementation,
  is the AVIF reader's evidence for 1.0 (the reference-origin route of
  ADR-0025 point 1, which the 2026-11-12 entry above asked the owner to
  confirm explicitly). The Argon conformance suite is not a 1.0 gate; it is
  later extra evidence, earmarked for animated AVIF, because it exercises
  the video code that no test reaches today. The statement above that the
  Argon gate "remains a precondition for selecting the dav1d port, or any
  other non-reference decoder" stands. (2) **dav1d's place:** dav1d leaves
  the runtime and the published chain, as rav1e and the jxl-oxide reader
  did; it may stay in tests as an independent cross-check. **Not done
  yet:** `libavif-rs` still depends on `libavif-dav1d` as a normal
  dependency (it is in the published chain in [publishing](../publishing.md)),
  and the effort of the removal is being checked first (it touches the
  libavif port's codec table and the root references of libavif-rs ADR 0050
  and ADR 0053). Also owed: a comparison of the engine's AVIF decode with
  libavif using dav1d in its stock configuration, so that the performance
  statement is honest; the recorded ratios are against libavif with libaom.
  The hosted AVIF benchmark (`scripts/bench/ci-perf/codec_avif.sh`) already
  has C libavif with dav1d as a context column (`c-dav1d`); no statement
  from it is written yet.
- 2026-11-16 — **The dav1d removal is paused (owner decision of
  2026-11-16).** Part (2) of the 2026-11-15 entry, dav1d leaving the
  runtime and the published chain, is not carried out for now. The reason
  is speed: the port's AVIF decode through the translated libaom took 1.75
  to 2.1 times as long as stock libavif with dav1d, one thread, on the
  hosted runners of run 43789376239 (context column `c-dav1d`; pixels
  identical), and dav1d's speed lies in its hand-written assembly, which
  the workspace dav1d port does not contain (it is translated with
  `-Denable_asm=false` and runs scalar). Instead, dav1d's kernels are
  rebuilt with Rust intrinsics under [ADR-0044](0044-no-assembly-acceleration-through-rust-intrinsics.md)
  D2, with no time pressure for 1.0, in parallel with the other work;
  their gate is bit identity with dav1d's own build with its assembly at
  the same level. **Decided:** that work starts; the branch that removes
  dav1d (`v1/avif-drop-dav1d`) is not merged; `libavif-rs` keeps
  `libavif-dav1d` as a normal dependency, and the reader keeps selecting
  `AVIF_CODEC_CHOICE_AOM` explicitly. **Not decided:** which decoder the
  engine uses in the end, and whether both decoders remain in the
  published chain. That is decided once the rebuilt kernels can be
  measured. Part (1) of the 2026-11-15 entry (bit identity with libaom as
  the reader's evidence for 1.0) is unchanged, and so is the statement
  that the Argon gate remains a precondition for selecting the dav1d port
  as the reader.
- 2026-11-22 — **Owner decision: the dav1d port is the default AVIF
  decoder for 1.0** ("Passt: dav1d wird der Standard-Decoder für 1.0").
  Evidence: the [decoder measurement of 2026-11-22](../benchmarks/v1.0-decoders-2026-11-22.md)
  at current `main` with the rebuilt kernels (merged 2026-11-20): macOS
  ARM64 local, two passes, and x86-64 Linux hosted, runs 45015543707 and
  45016849091. The dav1d port takes 0.50–0.69 of the libaom port's time at
  one thread (0.27–0.66 with all threads), less than half its peak memory
  (12 MP 8-bit 4:2:0, one thread: 104–105 against 251 MB), and decodes
  every input to the same RGB as the libaom port on every host; against
  stock dav1d it is identical on 994 of 994 cases (libavif-rs ADR 0062).
  This supersedes part (2) of the 2026-11-15 entry (dav1d leaves the
  runtime and the published chain) and the 2026-11-16 pause: dav1d and
  its kernels stay in the runtime and the published chain, and the branch
  that removed them (`v1/avif-drop-dav1d-3`) is obsolete. **Recorded
  exception to ADR-0025 point 1:** the Argon precondition for selecting
  the dav1d port, which the 2026-11-12, 2026-11-15 and 2026-11-16 entries
  kept, is set aside for this decoder by the owner; its 1.0 evidence is
  bit identity with the libaom reference and with stock dav1d, and Argon
  conformance is a follow-up ([#267](https://github.com/sebastian-software/ferrisma/issues/267), "AVIF: Argon conformance
  evidence for the dav1d default decoder"). **Implemented** on
  `v1/avif-dav1d-default`: `ferrisma-avif-read` takes a `Codec`
  (`Dav1d`, the default, or `Libaom`) and sets it explicitly, never
  libavif's `AUTO`; the engine has `pipeline::AvifDecoder` and
  `TransformOptions::avif_decoder` (default `Dav1d`), the CLI
  `--avif-decoder=dav1d|libaom`; a nondefault choice with an input that
  is not AVIF is `InvalidOptions` (ADR-0042, options without effect). The
  report's `decoder=` is `libavif/dav1d` by default and `libavif/aom` with
  the libaom reference. The libaom port stays the only AVIF encoder. The
  AV1 decode stays on one thread for both decoders; enabling dav1d's
  threads is a separate decision. **Evidence in CI:**
  `dav1d_and_libaom_decode_the_avif_corpus_identically` decodes every
  committed AVIF file (95 on 2026-11-22: 81 decode, 14 are refused alike)
  with both decoders and compares the YUV and alpha planes and the RGB
  output; `both_decoders_reproduce_the_pinned_c_oracle` checks both
  against the unchanged C libavif/libaom SHA-256 of the seven committed
  fixtures of the 2026-11-11 record above; both run in `ci.yml` (`rust`,
  Linux x86-64; `native-adapters`, macOS ARM64), next to the engine test
  `dav1d_default_rows_equal_the_libaom_reference` and the CLI test
  `cli_reads_avif_with_dav1d_by_default_and_libaom_on_request`. The
  existing reader tests now run through the dav1d default with their
  expected values unchanged; only the report name of the default reader
  changed with the decoder. The dav1d port keeps its own parity gate
  against stock dav1d (`avif-dav1d.yml`). Part (1) of the 2026-11-15
  entry (bit identity with libaom as the reader's evidence) now applies to
  the dav1d default through its identity with libaom.
- 2026-11-23 — **dav1d decodes on the engine's threads** (owner approval of
  2026-11-23: measure dav1d's decode threads and enable them for dav1d if
  they pay off; [record](../benchmarks/v1.0-avif-dav1d-threads-2026-11-23.md)).
  `Decoder::set_threads` now sets libavif's `maxThreads` for
  `Codec::Dav1d`; libavif's dav1d glue passes it as `n_threads` with
  `max_frame_delay` 1, so dav1d runs one frame context and spreads its tile
  and superblock-row tasks over the pool. The engine's resolved
  `transform_threads` reach the decode as before (clamped to 64 by the
  reader); a batch item keeps one thread. The libaom reference keeps its
  decode on one thread for the reason of the 2026-11-12 entry; `threads`
  still drives its YUV-to-RGB row jobs and the engine's row pipeline, so
  `--avif-decoder=libaom` with threads is not an option without effect
  (ADR-0042). Measured on the 16 inputs of the 2026-11-22 record through
  the engine's reader: macOS ARM64 (10 cores) 2.15–2.47× at 12 MP and
  3.03–3.45× at 24 MP with ten threads against one, 1.06–1.30× at web
  sizes; peak RSS grows by 1–7 % (24 MP 10-bit 4:4:4: 306 → 326 MB, against
  597 MB for libaom on one thread). On x86-64 Colima (4 vCPUs, Rosetta, indicative) four threads gave
  1.36–2.02× at 12 MP and 1.66–2.73× at 24 MP, RSS +0–8 %; the hosted runs
  of 2026-11-22 (dav1d decode on four threads, serial conversion; Intel
  and AMD) gave 1.39–1.80× at 12 and 24 MP and 0.94–1.06× at web sizes. Every output is
  identical to the one-thread output: RGB8 of every sample, and the YUV
  planes and full-depth RGB of every input at every thread count, natively
  and under `qemu-x86_64 -cpu Haswell-v4`. **Evidence in CI:**
  `dav1d_threads_decode_the_avif_corpus_identically` decodes every
  committed AVIF file (81 of 95 decode) with dav1d on 2, 3, 4 and 8
  threads and compares YUV and alpha planes and RGB with one thread.
  The 2026-11-22 entry's "the AV1 decode stays on one thread for both
  decoders" is superseded for dav1d.
- 2026-11-23 — **Argon evidence for the dav1d default is recorded**
  ([record](../benchmarks/v1.0-argon-dav1d-2026-11-23.md)). The 2,763 streams of
  the default set of dav1d's Argon test script decode to the expected MD5 with
  stock C dav1d and with the port on every configuration: macOS ARM64 scalar and
  NEON, x86-64 scalar and AVX2 under Rosetta, and AVX2 under `qemu-x86_64`. The
  Argon precondition kept by the 2026-11-12, 2026-11-15 and 2026-11-16 entries
  for selecting the dav1d port is satisfied, and the exception of 2026-11-22 to
  ADR-0025 point 1 is resolved. The large-scale-tile, stress and error streams
  were not run.
- 2026-11-26 — Finalized as a living record, owner decision: status Accepted; stale passages corrected in place, dropped decisions marked, open questions limited to open issues.
