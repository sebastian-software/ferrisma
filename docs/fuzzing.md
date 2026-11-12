# Fuzzing and hostile-input policy

**Status:** Harness and six-lane ASan campaign workflow exist. The first hosted campaign ([run 43117625746](https://github.com/sebastian-software/ferrisma/actions/runs/43117625746), 2026-11-11) found one JPEG XL panic and three macOS link failures; both are fixed. Bounded 10-minute local ASan runs of all three lanes on macOS ARM64 (2026-11-12) found no further failure. The sustained, coverage-reviewed campaign required for release has **not** completed; see [results](#campaign-results).

Ferrisma's public processing facade accepts untrusted JPEG, WebP, JPEG XL, and AVIF bytes. The core forbids unsafe Rust; optional decoder, assembler, SIMD and encoder dependencies do **not** inherit that guarantee. The workspace AVIF reader checks dimensions and reserves the emitted RGB frame before pixel decoding, but libavif/AOM still allocate opaque parser and codec buffers outside the engine budget.

## Reproducible fuzz target

Install the pinned `cargo-fuzz` and run the target with the repository's pinned native nightly toolchain:

```sh
cargo install cargo-fuzz --version 0.13.2 --locked
cargo fuzz run transcode -- -max_len=524288 -timeout=10 -rss_limit_mb=1024
```

The target accepts one control byte followed by unchanged encoded image bytes.
Its two low bits choose JPEG (0), default JPEG XL (1), AVIF (2), or WebP (3);
bit 2 selects WebP instead of AVIF output. Bits 3–4 select the profile policy:
zero uses rendered ICC for JPEG XL and an explicit sRGB assumption only when
metadata is absent for the other formats; 1 is strict; 2 assumes sRGB only when
metadata is absent; 3 requests the rendered-ICC transform. Bit 5 selects the
`f64` reference rather than the `f32` executor. The high two bits select a small
output, 8192×1, 1×8192, or 64×64. Existing JPEG and JPEG XL CI seeds keep their
one-byte 0 and 1 prefixes and now reach the transform with the policies their
fixtures require. The chosen policy remains explicit; unsupported color still
fails rather than being silently reinterpreted.

The driver sets limits of 512 KiB compressed input, 1 million input pixels,
512 KiB retained output, and 2 MiB engine-managed memory, and requests one
transform and encoder thread. The extreme aspect-ratio lanes exercise planning
admission; a resource error is an expected result. These limits do not cap codec
scratch, thread/runtime overhead, or total RSS. Run fuzzing in a sandbox with
process-level limits and sanitizer support; preserve and minimize crashes in
`fuzz/artifacts/` or the upstream codec test corpus. The sustained workflow
also has a separate opt-in workspace JPEG XL decoder lane.

The shared driver has a deterministic smoke test using real JPEG, JPEG XL,
AVIF and WebP fixtures, each with both output formats. It checks that every
framed input reaches the decoder and produces encoded output, and checks strict
color rejection and input/geometry limits:

```sh
cargo test --manifest-path fuzz/Cargo.toml --lib
```

The hosted workflow seeds all eight tested input/output pairs. This workflow
configuration is not a completed campaign result; retain the exact run and
revision when reporting hosted coverage.

The default CI also runs **deterministic malformed-header smoke tests** for truncated JPEG/JXL/AVIF files and invalid API options, so regressions surface without an expensive nightly fuzz dependency.

## Sustained campaign workflow

[M1 Sustained Fuzz Campaign](../.github/workflows/m1-fuzz-campaign.yml) is a
manually dispatched, six-job experiment: the multi-format driver, the workspace
AVIF/AOM writer, and the opt-in workspace JPEG XL decoder each run on native Linux x86-64 and macOS
ARM64. Dispatch the registered `M1 Decoder Fuzz Smoke` workflow with the
`campaign` input set to `true` to run it on a branch; the default remains the
quick smoke. Each job uses the pinned nightly and cargo-fuzz 0.13.2 with explicit
AddressSanitizer, a libFuzzer seed of 1, and a 1,800-second mutation budget.
The input limit is 524,289 bytes including the selector; per-input timeout is
10 seconds and the libFuzzer RSS limit is 2 GiB. A 150-minute job timeout and
40-minute coverage-replay timeout bound setup and replay separately. These
limits are not a bound on all operating-system resources.

The checked-in seed generator frames the eight successful input/output pairs,
additional AVIF grid, alpha, ICC and upstream proof-of-concept files, JPEG XL
orientation fixtures, and valid and malformed JPEG ICC tag tables. The own-JXL
lane accepts only inputs whose selector chooses JPEG XL; it keeps the rest of
the selector and image bytes unchanged. Deterministic seed tests report whether
each starting input encodes or is rejected, and assert both output encoders
are reached for the applicable decoder. The generator hashes the exact starting
bytes. Mutations and duration vary with scheduling and hardware despite the
fixed PRNG seed; the seed does not make the resulting corpus deterministic.

The own-AVIF lane accepts only AVIF-output selectors and runs each admitted
input through the workspace AOM writer at both 8 and 10 bits. It uses the
default input decoders, including JXL-Oxide. Its deterministic fixtures cover
all four input formats and AVIF alpha; tests assert the reported encoder is
`libavif-rs/aom`. The separate own-JXL lane covers Rav1e and WebP output; the
own-JXL-to-own-AVIF combination is outside this campaign. Encoder choice and
bit depth are explicit lane settings and do not consume or rewrite image bytes.

Artifacts include the source commit and tree, toolchain, host, lockfile hashes,
seed hashes and classifications, full fuzz log and final libFuzzer statistics,
retained corpus and any crash files. A separate instrumented replay produces
source coverage data and a report for the retained corpus. Coverage describes
what that replay reached; it is not a correctness or security guarantee.
Failures and timeouts require inspection, and the default smoke remains a quick,
separate check. The standalone fuzz lockfile pins its dependencies; its AVIF
profile exceptions mirror the four generated packages in the root workspace.
Dependencies are fetched with `--locked`; mutation and replay builds run
offline, and lockfile hashes are checked after tests, the campaign and replay.
Seed classifications and source-coverage counts must be inspected before
claiming that a particular ICC or AVIF path was reached.
Record the actual job URLs, durations, crashes, coverage, and limitations
after each execution; the configuration alone is not a result. An ASan
campaign alone does not satisfy all release hardening gates.

The campaign and coverage steps set `RUSTFLAGS=--cfg libavif_rs_asan`, which
cargo-fuzz appends to its own flags. The generated dav1d and AOM ports register
C static initializers as `INIT_ARRAY` statics in `__mod_init_func`
(`.init_array` on Linux); the cfg disables ASan instrumentation of exactly
those statics, as the codec's own sanitizer tooling does. Without it the macOS
linker rejects the instrumented `cdylib` outputs of `libavif-dav1d` and
`libavif-aom` with `initializer pointer has no target`; this was a build
failure, not a decoder memory-safety finding. Linux linked without the cfg.

## Campaign results

### Hosted campaign, run 43117625746 (2026-11-11)

The first dispatch of the six-job workflow ran on the #194 branch. Only the two
Linux own-JXL and own-AVIF lanes completed their 1,800-second budget.

- **Linux `transcode`:** after 75,175 executions libFuzzer stopped on a panic in
  `jxl-color` 0.11.0 `icc/synthesize.rs:274` ("Unknown color space, ICC profile
  not embedded?"), reached from public `transcode` through the default
  JXL-Oxide adapter: the JPEG XL header used an enum colour encoding whose
  colour space was unknown, and `JxlImage::rendered_icc` synthesizes a profile
  from it. The 40-byte input `crash-510ca0e9c63aeae240305a2f57956ff3f3f7db72`
  is retained without its selector byte as
  `tests/fixtures/hostile/jxl-unknown-colour-space.jxl` and as a campaign
  seed. The adapter now rejects unknown or XYB colour spaces, an unknown
  transfer function and a zero gamma (a divisor in the same synthesis code) with
  `UnsupportedColor` before any profile synthesis or rendering
  ([ADR-0012](adr/0012-codec-source-contract.md)). Tests:
  `hardening_tests::jxl_oxide_unknown_colour_space_is_a_controlled_error`,
  `public_transcode_rejects_unknown_jxl_colour_space` (all three color
  policies) and `workspace_jxl_unknown_colour_space_is_a_controlled_error`
  (the workspace decoder rejects the same bytes as malformed). The zero-gamma
  branch has no minimized input yet.
- **All three macOS lanes:** the ASan build failed while linking
  `libavif-dav1d` and `libavif-aom`; no input was executed. See the
  `libavif_rs_asan` note above.

### Bounded local ASan runs (2026-11-12)

These runs confirm the two fixes and add a short mutation pass. They are **not**
the sustained campaign: each lane had a 600-second budget on a host shared with
other build jobs, so execution counts are not throughput evidence.

| | `transcode` | `transcode_own_avif` | `transcode_own_jxl` |
|---|---|---|---|
| Features | default (`cli`) | default (`cli`) | `own-jxl` |
| Seed files (from `seed_campaign.py`) | 16 | 5 | 11 |
| `-max_total_time` (s) | 600 | 600 | 600 |
| Executions | 237,741 | 123,843 | 489,701 |
| Final edge coverage (`cov`) | 34,720 | 34,228 | 20,152 |
| Retained corpus files | 3,812 | 2,396 | 2,232 |
| libFuzzer peak RSS (MB) | 563 | 606 | 762 |
| Crashes, leaks, timeouts, OOM | none | none | none |

Common settings: macOS 27.0 (Darwin 27.0.0) arm64, Apple M1 Ultra;
`rustc 1.100.0-nightly (5a2be9f5f 2026-09-06)` from `rust-toolchain.toml`;
cargo-fuzz 0.13.2; `--sanitizer address`, `RUSTFLAGS=--cfg libavif_rs_asan`,
`ASAN_OPTIONS=detect_odr_violation=0` (cargo-fuzz's default);
`-seed=1 -max_len=524289 -timeout=10 -rss_limit_mb=2048`. libFuzzer's length
control kept the effective maximum input near the largest seed (21,916 bytes
for `transcode`; 378 bytes for own-JXL, whose JPEG XL seeds are 40–174 bytes),
so the 512 KiB driver limit was not explored. Source: engine and fuzz driver at
commit `984910f8` (later commits on that branch change only documentation and
the workflow). Lockfile SHA-256: `Cargo.lock` `2f082fab…3086`,
`fuzz/Cargo.lock` `e28859d0…4631`. The seed manifest printed by
`seed_campaign.py` has SHA-256 `f9ae92b4…37b1`; replaying the hosted crash
input through the rebuilt ASan `transcode` binary exits cleanly. Retained
corpus manifests (SHA-256 over the sorted `shasum -a 256` listing):
`transcode` `5ee12e43…c862`, own-AVIF `01f7c47d…3416`, own-JXL
`5ed39970…55d5`. The corpora and logs stayed on the local host and were not
uploaded.

The deterministic seed test (`FERRISMA_FUZZ_SEEDS=… cargo test --manifest-path
fuzz/Cargo.toml --lib --features own-jxl`) passed: every own-AVIF seed encoded
at 8 and 10 bits; the default lane rejected the AVIF grid seed under the
rendered-ICC selector (`UnsupportedInput`: the ICC transform accepts opaque RGB
only), the upstream proof-of-concept file (`UnsupportedInput`), both malformed
ICC tag tables and the hostile JPEG XL seed (`UnsupportedColor`); the workspace
JPEG XL decoder rejected the hostile seed as `MalformedInput`.

### Deterministic hostile-input corpus

- `hardening_tests::upstream_libavif_corpus_returns_controlled_public_results`
  runs every upstream libavif `.avif` test file (grid, alpha, `clap`, gain-map,
  animation, HDR, ICC, truncated-extent and proof-of-concept inputs) through
  public `transcode` under all three color policies. None panics or reports
  `Internal`; `clap` is `UnsupportedInput`, PQ/HLG and container ICC without
  the ICC policy are `UnsupportedColor`, animation is `UnsupportedInput`, and
  grid/alpha stills encode.
- `cms::tests::hostile_icc_profiles_are_rejected_or_converted_without_panicking`
  derives 398 profiles from a real v4 profile: truncations, every header and
  tag-table word set to 0 and `u32::MAX`, wrong data/PCS color spaces and
  seeded byte flips. Each either converts a row or is rejected as unsupported
  color.
- Existing focused tests cover AVIF container/bitstream CICP and ICC conflicts,
  mirror-before-rotation order, oversized ICC properties, and truncated boxes
  (`avif_meta_tests`, `codecs::avif`), plus truncated JPEG/JXL/AVIF headers.

### Limitations

- No coverage-reviewed sustained run exists yet; rerun the hosted workflow with
  the fixes and record its six jobs here.
- Source coverage was not measured locally. An ASan coverage build of
  `transcode` (with the same cfg) linked on macOS ARM64 and replayed all 3,812
  retained inputs without a failure, but `llvm-tools-preview` is not installed
  on that host, so the profile was not merged or reported. The hosted workflow
  installs the tools; its coverage report is still needed.
- ASan only; no UBSan, MSan or independent sanitizer. The Rust engine forbids
  `unsafe`, but the generated libavif/AOM/dav1d ports, assembly and other codec
  dependencies do not inherit that guarantee.
- The own-JXL seeds are tiny; add larger JPEG XL inputs (ICC, alpha, animation,
  extra channels) before the sustained run.
- JPEG safe-API fuzzing is tracked separately in #73; color semantics in #17
  and #41.

## M1 exit evidence still needed

- [ ] Record a sustained, coverage-reviewed fuzz campaign (duration, seed corpus, architecture/toolchain, sanitizer configuration, coverage and unique crashes). A first hosted run and bounded local runs are recorded above.
- [ ] Audit decompression bombs and allocation ceilings at the **decoder preflight** stage before an untrusted frame is fully decoded. The [preflight audit](#preflight-and-resource-audit-2026-11-12) records the current state per decoder.
- [ ] Verify JPEG EXIF orientation, JPEG XL orientation/region coordinate mapping and AVIF orientation are not silently lost.
- [x] Verify ICC parsing with hostile profiles, not only short headers (deterministic corpus above). Output-profile contradictions remain with #17/#41.
- [ ] Recheck memory limits with actual process RSS, not just `MemoryBudget` snapshots.

The existence of a fuzz target alone does not satisfy the M1 hardening acceptance criteria.

## CI bounded smoke

The separate [M1 Decoder Fuzz Smoke](../.github/workflows/m1-fuzz-smoke.yml) uses the root pinned nightly and cargo-fuzz 0.13.2, verifies deterministic seed success, seeds eight input/output pairs from four real JPEG/JXL/AVIF/WebP fixtures, and runs **3,000 bounded attempts** with a 16 KiB maximum input and a 1 GiB process RSS cap. This is useful to catch shallow codec/protocol panics and integration bugs. It does **not** constitute a sustained, coverage-reviewed fuzz campaign and is not proof of process-wide memory safety; actual release acceptance still requires broader corpora, sanitizers and target-specific time budgets.

### Expanded hosted smoke, 2026-11-11

[Workflow run 43072724334](https://github.com/sebastian-software/ferrisma/actions/runs/43072724334)
completed successfully on revision `8d34b360940a3ad7cff19436baf47f5ae75bd1e4`
with Ubuntu 26.04 x86-64, `nightly-2026-09-07-x86_64-unknown-linux-gnu`,
and cargo-fuzz 0.13.2. The deterministic test exercised all eight combinations
of the four input fixtures and two output formats. The fuzz phase ran 3,000
iterations in 16 seconds without a reported crash; libFuzzer reported 406
retained corpus inputs and `rss: 405Mb`. The command specified a 16 KiB input
limit, a five-second per-input timeout, and a 1 GiB RSS limit. The job log does
not print an explicit sanitizer selection, so this record does not assert one.

The uploaded [corpus artifact](https://github.com/sebastian-software/ferrisma/actions/runs/43072724334/artifacts/13878059847)
has SHA-256 `a0a15b4dc3cfdcc637bcdbdb9c453c4c73a1995985f0c876063c51d49088d4dc`.
This single short run does not close the sustained-campaign, coverage review,
independent sanitizer, or security-sensitive release gates above.

## Preflight and resource audit (2026-11-12)

Source review of `src/pipeline.rs`, `src/memory.rs` and the codec adapters at
commit `984910f8`. "Before decode" means before pixel data is rendered.

| Check | State |
|---|---|
| Compressed byte cap | `max_input_bytes` (default 64 MiB) is checked before any decoder opens, in `transcode*` and `inspect`. |
| Pixel cap, AVIF/WebP/own-JXL | Passed into the adapter and checked from container/sequence headers before the frame is decoded or reserved (`new_bounded`). |
| Pixel cap, JPEG | Header-only `JpegSource::new`; the facade checks `max_input_pixels` before the first scanline request. |
| Pixel cap, JXL-Oxide | Construction parses the header and buffers frame data (bounded by the compressed size); the facade checks the pixel cap before rendering. Unrenderable colour encodings are rejected at construction (above). |
| Geometry overflow | Pixel counts use `u64`; budget reservations use `checked_add`/`checked_mul` and fallible `try_reserve_exact`; region and row sizes use checked conversions. |
| Engine allocation admission | `MemoryBudget` (`max_managed_bytes`, default 256 MiB) admits every engine row, frame, orientation and encoder-frame buffer; the 8192×1 driver lane is rejected by planning before encode. |
| Opaque codec memory | Not admitted: libjpeg-turbo-rs, JXL-Oxide render buffers, libavif/AOM parser state, libjxl-rs, libwebp-rs, rav1e and libavif encoder internals. Reports flag this with `opaque_decoder_memory`/`opaque_encoder_memory`. JXL-Oxide exposes an `AllocTracker` limit that is not wired up yet. |
| Output cap | `max_output_bytes` is checked after the encoder returns its complete output, not incrementally. |
| Cancellation | No caller-driven cancellation or deadline exists. `ProcessingErrorKind::Cancelled` arises only when an output sink or backend aborts. A hostile input is bounded by the byte/pixel caps and the caller's process limits, not by a time budget. |
| Process RSS | Not measured against `MemoryBudget` yet; libFuzzer peak RSS above is the only process-level observation and includes ASan overhead. |

No new engine gap required a code change in this audit beyond the JPEG XL
colour rejection; the opaque-memory, incremental output cap, cancellation and
RSS items remain open follow-ups.

## Early input limits

The experimental public facade rejects compressed input exceeding a configurable byte cap (default **64 MiB**) before opening any decoder and rejects image dimensions above a configured **100-megapixel** default. The AVIF adapter checks the selected primary color tiles' AV1 sequence headers **before full-frame decode**, with ICC copies capped at 16 MiB. This mitigates common decompression-bomb cases; it does **not** bound memory used by libavif/AOM, JPEG XL or encoders. Ferrisma's bounded color/orientation parser and libavif each inspect the AVIF container. Always retain process RSS and sandbox limits for hostile inputs. See [ADR-0031](adr/0031-workspace-libavif-input-experiment.md) for the current boundary; the measured fuzz run below predates this reader replacement.

## First measured fuzz result

[Workflow run 41235323267](https://github.com/sebastian-software/ferrisma/actions/runs/41235323267) completed 3,000 seeded/mutated invocations using cargo-fuzz 0.13.2 and a nightly compiler, without a reported crash. The final corpus was 141 inputs; the log's reported peak RSS was about 139 MiB. This is only a shallow smoke. Continuous fuzzing with longer duration, realistic large color profiles, AVIF grids/alpha and independent sanitizers is **still required** for a security-sensitive release.

## JPEG port target

The workspace JPEG port has its own cargo-fuzz target, `decode`, in
[codecs/libjpeg-turbo-rs/fuzz](../codecs/libjpeg-turbo-rs/fuzz/README.md),
decided in codec [ADR 0009](../codecs/libjpeg-turbo-rs/docs/adr/0009-coverage-guided-fuzzing.md).
It takes plain JPEG bytes and runs each input through seven cases of the
port's safe decoding API: output formats, scales from 1/8 to 16/8, the
islow, ifast and float IDCTs, fast upsampling, a column crop with skipped
rows, metadata reads, and failing on the first warning. Unlike the engine's
`transcode` target, it reaches decoder options that Ferrisma's adapter does
not use. Images above one megapixel are skipped, as in upstream's targets.

The seeds are upstream's test images, the in-scope `djpeg` inputs that
upstream's CTest suite writes with the release oracle (progressive,
arithmetic, unusual sampling, ICC), and the engine's JPEG fixtures; the
T.83 compliance data is not redistributable and stays out. A unit test of
the fuzz crate checks that each 8-bit seed finishes at least one case.
The nightly smoke runs 3,000 attempts of this target in a separate job, and
the dispatched sustained campaign adds an ASan job on Linux x86-64 and macOS
ARM64 with the same 30-minute budget, artifacts and coverage replay as the
other lanes. Neither hosted lane has run yet.

### Local bounded run, 2026-11-12

On revision `4d01f840` (macOS ARM64, `nightly-2026-09-07`, cargo-fuzz
0.13.2, AddressSanitizer, upstream `fuzz/jpeg.dict`, 2 GiB RSS limit, 10 s
per-input timeout), two local runs of 180 s and 600 s of configured budget
executed 9,959 and 30,775 inputs without a crash, a timeout or a sanitizer
report. The second started from the 25 seeds plus the first run's corpus,
reached 5,451 coverage points, kept 1,031 inputs, and reported a peak RSS of
907 MB. This is a short local smoke on a shared host, not a sustained
campaign, and it says nothing about throughput.
