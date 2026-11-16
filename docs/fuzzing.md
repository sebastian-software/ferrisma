# Fuzzing and hostile-input policy

**Status:** Harness and four-job ASan campaign workflow exist (eight jobs until 2026-11-15). The first hosted campaign ([run 43117625746](https://github.com/sebastian-software/ferrisma/actions/runs/43117625746), 2026-11-11) found one JPEG XL panic and three macOS link failures; both are fixed. Bounded 10-minute local ASan runs of all three lanes on macOS ARM64 (2026-11-12) found no further failure; a 20-minute local ASan run of the JPEG XL output lane `transcode_jxl_out` (2026-11-15) found none either. The hosted smoke [run 43690971661](https://github.com/sebastian-software/ferrisma/actions/runs/43690971661) (2026-11-15) found a 1 GiB allocation request in jxl-oxide from an 11-byte JPEG XL codestream; it was bounded by the input cap the same day ([details](#hosted-smoke-run-43690971661-2026-11-15)), and on 2026-11-15 jxl-oxide left the engine's runtime altogether (owner decision, [ADR-0032](adr/0032-jpeg-xl-workspace-decoder-input.md)): every lane now reads JPEG XL through the workspace libjxl-rs reader, and the `transcode_own_jxl` lane, which differed only by that reader, is merged into `transcode` ([lanes since 2026-11-15](#lanes-since-2026-11-15)). The sustained, coverage-reviewed campaign required for release has **not** completed; see [results](#campaign-results).

Ferrisma's public processing facade accepts untrusted JPEG, WebP, JPEG XL, and AVIF bytes. The core forbids unsafe Rust; optional decoder, assembler, SIMD and encoder dependencies do **not** inherit that guarantee. The workspace AVIF reader checks dimensions and reserves the emitted RGB frame before pixel decoding, but libavif/AOM still allocate opaque parser and codec buffers outside the engine budget.

## Reproducible fuzz target

Install the pinned `cargo-fuzz` and run the target with the repository's pinned native nightly toolchain:

```sh
cargo install cargo-fuzz --version 0.13.2 --locked
cargo fuzz run transcode -- -max_len=524288 -timeout=10 -rss_limit_mb=1024
```

The target accepts one control byte followed by unchanged encoded image bytes.
Its two low bits choose JPEG (0), JPEG XL (1, the workspace libjxl-rs reader), AVIF (2), or WebP (3);
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
`fuzz/artifacts/` or the upstream codec test corpus.

The shared driver has a deterministic smoke test using real JPEG, JPEG XL,
AVIF and WebP fixtures, each with both output formats. It checks that every
framed input reaches the decoder and produces encoded output, and checks strict
color rejection and input/geometry limits:

```sh
cargo test --manifest-path fuzz/Cargo.toml --lib
```

The hosted smoke seeds all eight tested input/output pairs. This workflow
configuration is not a completed campaign result; retain the exact run and
revision when reporting hosted coverage.

The default CI also runs **deterministic malformed-header smoke tests** for truncated JPEG/JXL/AVIF files and invalid API options, so regressions surface without an expensive nightly fuzz dependency.

## JPEG XL output lane

`transcode_jxl_out` (issues #27, #65) writes JPEG XL through the workspace
libjxl-rs encoder ([ADR-0039](adr/0039-jpeg-xl-output-through-the-workspace-encoder.md)):
lossy `VarDCT` through `ferrisma-jxl-encoder` and lossless effort 2 through
the port's fixed-profile entry point, both behind `jpeg-xl-write` in `cli`.
It uses the default features, so JPEG XL input goes through the workspace
libjxl-rs reader.

```sh
cargo fuzz run --sanitizer address transcode_jxl_out -- \
  -seed=1 -max_len=524289 -timeout=10 -rss_limit_mb=2048
```

One control byte, then unchanged image bytes (`run_jxl_out_case` in
`fuzz/src/lib.rs`):

| Bits | Selects |
|---|---|
| 0–1 | input JPEG (0), JPEG XL (1), AVIF (2), WebP (3) |
| 2 | lossy d1.0 (0) or lossless effort 2 (1) |
| 3–4 | profile policy, as in `transcode` |
| 5–7 | geometry: 0 small (1–16 per axis, from the input length), 1 8192×1, 2 1×8192, 3 64×64, 4 inside 64×64 without enlargement (a small input keeps its size and its alpha edges), 5 inside 320×320 without enlargement (crosses the 256-pixel group), 6 and 7 64×64 at the lossy distance bounds 0.05 and 2.999 (lossless ignores the distance) |

The encoder is serial and rejects `encoder_threads != 1` before decoding, so
no bit selects a thread count. Limits: 512 KiB compressed input, 1 million
input pixels, 512 KiB retained output, and **32 MiB** engine-managed memory.
The lossless sink reserves its measured peak, about 76 bytes per opaque pixel
and 101 with alpha (#223); 32 MiB admits 320×320 with alpha (about 10 MiB)
and the resample plan of the 8192×1 and 1×8192 classes (about 18 MB for a
small input; 16 MiB rejected it), so every class reaches the encoder. A
request that does not fit is `ResourceExhausted`. Every `Err` from the public
API is accepted. On success the target checks the JPEG XL signature (`FF 0A`
or the container box), the output limit, the requested dimensions (exact, or
within the bound for classes 4 and 5), and decodes the output with
`ferrisma-jxl-decoder`: same width and height, three or four channels, a
complete decode. Pixels are not compared.

`seed_campaign.py` writes 48 seeds for the lane: the four base fixtures at
64×64 in both profiles, the small class and the distance bounds, alpha
(`jxl-lossless/rgba-37x19.jxl`, `effects_rgb_alpha8.jxl`, the libavif
`abc_color_irot_alpha_irot.avif`), 16-, 31- and 12-bit RGB
(`modular-palette16.jxl`, `modular-rgb31.jxl`, `weld_sato_12B_8B_q0.avif`),
gray JPEG and gray JPEG XL (8-, 12- and 16-bit with alpha), 257×3 and 3×257
inputs to 8192×1, 1×8192 and the bounded classes, the two-group
`rgb-300x260.jxl`, and the two hostile JPEG XL regressions. The
deterministic replay (`campaign_seeds_report_reachability`) classifies them:
36 encode, the eight gray JPEG XL seeds are `UnsupportedInput` (the pipeline
refuses gray JPEG XL under every policy) and the hostile ones are
`MalformedInput` in both profiles. Two unit tests (`jxl_output_lane_*`) run in every smoke without the
seed directory: all four inputs in both profiles, every exact geometry class
from JPEG, and alpha surviving the round trip at 37×19 in both profiles.

### Local ASan run, 2026-11-15

| | `transcode_jxl_out` |
|---|---|
| Features | default (`cli`) |
| Seed files | 46 (41 retained after the initial pass, 523 KB) |
| `-max_total_time` (s) | 1,200 (wall clock 20 min 25 s including the no-op rebuild) |
| Executions | 34,517 (28/s on a host shared with other agents' builds) |
| Coverage at `INITED` | `cov: 56227 ft: 131600` |
| Final coverage (`DONE`) | `cov: 67155 ft: 187437` |
| Retained corpus | 1,893 files, 9,910 KB (993 lossy, 917 lossless selectors) |
| libFuzzer peak RSS (MB) | 671 |
| Slowest unit (s) | below 1 |
| Crashes, leaks, timeouts, OOM, sanitizer reports | none |

Settings: macOS 27.2.1 arm64 (Apple M1 Pro); `rustc 1.100.0-nightly
(5a2be9f5f 2026-09-06)` from `rust-toolchain.toml`; cargo-fuzz 0.13.2;
`--sanitizer address`, `RUSTFLAGS=--cfg libavif_rs_asan`; `-seed=1
-max_total_time=1200 -max_len=524289 -timeout=10 -rss_limit_mb=2048`. The
ASan build of the full `cli` feature set linked on macOS ARM64 (9 min 57 s,
cold); no smaller feature set was needed. libFuzzer's length control
reached 196,895 bytes, the largest seed. One corpus for both profiles; no
dictionary. Source: fuzz driver at commit `325dc378` on `v1/fuzz-jxl-out`
(engine at `bc566916`). SHA-256: `Cargo.lock` `a5196d3a…4b94`,
`fuzz/Cargo.lock` `5b1dbea1…beba`, seed manifest `080d5e39…a6d8`, retained
corpus manifest `132f1c5c…66cd`. The corpus and log stayed on the local
host. This is a bounded local run, not the sustained, coverage-reviewed
campaign; source coverage was not measured, and the hosted Linux and macOS
lanes have not run.

## Sustained campaign workflow

[M1 Sustained Fuzz Campaign](../.github/workflows/m1-fuzz-campaign.yml) is a
manually dispatched, four-job experiment: the multi-format driver and the
[JPEG XL output lane](#jpeg-xl-output-lane) each run on native Linux x86-64 and macOS
ARM64 (until 2026-11-15 also a workspace AVIF/AOM writer lane and an opt-in
workspace JPEG XL decoder lane, merged into the multi-format driver when
rav1e and jxl-oxide left the runtime). Dispatch the registered `M1 Decoder Fuzz Smoke` workflow with the
`campaign` input set to `true` to run it on a branch; the default remains the
quick smoke. Each job uses the pinned nightly and cargo-fuzz 0.13.2 with explicit
AddressSanitizer, a libFuzzer seed of 1, and a 1,800-second mutation budget.
The input limit is 524,289 bytes including the selector; per-input timeout is
10 seconds and the libFuzzer RSS limit is 2 GiB. A 150-minute job timeout and
40-minute coverage-replay timeout bound setup and replay separately. These
limits are not a bound on all operating-system resources.

The checked-in seed generator frames the eight successful input/output pairs,
additional AVIF grid, alpha, ICC and upstream proof-of-concept files, JPEG XL
orientation fixtures, and valid and malformed JPEG ICC tag tables.
Deterministic seed tests report whether
each starting input encodes or is rejected, and assert both output encoders
are reached for the applicable decoder. The generator hashes the exact starting
bytes. Mutations and duration vary with scheduling and hardware despite the
fixed PRNG seed; the seed does not make the resulting corpus deterministic.

The `transcode` target runs each AVIF-output input through the workspace
AOM writer at both 10 and 8 bits (`avif_depths`, `run_case_at_depth`), as
the former `transcode` (rav1e) and `transcode_own_avif` lanes did together;
WebP-output inputs run once. Tests assert the reported encoder is
`libavif-rs/aom` for every AVIF output. Encoder choice and
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

### Lanes since 2026-11-15

jxl-oxide is no longer a reader of the engine (ADR-0032 amendment of
2026-11-15), so no lane can select it. `transcode` reads JPEG XL through the
workspace libjxl-rs reader, and the former `transcode_own_jxl` lane (and the
fuzz crate's `own-jxl` feature), which differed from `transcode` only by
that reader, is retired: its JPEG XL orientation seeds moved to
`transcode` as `jxl-orientation-1` … `-8`, and its other seeds were
byte-identical to seeds `transcode` already has. No seed was dropped. The `transcode_own_avif` lane, which differed from
`transcode` only by its AVIF encoder once rav1e left (ADR-0037), is merged
the same way: `transcode` runs AVIF-output inputs at 10 and 8 bits, and the
own-AVIF seeds were byte-identical to `transcode`'s AVIF-output seeds. The
deterministic replay re-classifies the two hostile JPEG XL seeds on
`transcode` (they were read by jxl-oxide there before): the oversized TOC
section is `MalformedInput` (before: `ResourceExhausted` from the adapter's
allocation bound) and the unknown colour space is `MalformedInput` (before:
`UnsupportedColor`), in every lane, as libjxl rejects them. The sustained
campaign runs the remaining lanes on Linux x86-64 and macOS ARM64. The
records below are kept as they were measured; they name the readers of
their day.

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

### Hosted smoke, run 43690971661 (2026-11-15)

[Run 43690971661](https://github.com/sebastian-software/ferrisma/actions/runs/43690971661)
(`M1 Decoder Fuzz Smoke`, job `fuzz`, Linux x86-64) stopped after about 2,800
executions of `transcode` with `libFuzzer: out-of-memory (malloc(1077953535))`
on the 285-byte input `oom-444b7241bd4fef31ceb81b35807441cf9714188c`
(SHA-1): selector 1, then a bare JPEG XL codestream.

- **Reader.** `transcode` pins `JpegXlDecoder::JxlOxide` (`run_case`), so
  the input went through jxl-oxide 0.12.6, not the workspace reader that is
  the engine default since 2026-11-12.
- **Site.** Reproduced with the ASan build on macOS ARM64 (same request):
  `jxl_frame::GroupData::ensure_allocated` (jxl-frame 0.13.3, `lib.rs:95`, `try_reserve`),
  reached from `Frame::feed_bytes` during `JxlImage::builder().read` in the
  adapter's constructor. jxl-frame reserves the full declared size of a TOC
  section before it copies the section. The TOC entry is `U32(Bits(10),
  1024 + Bits(14), 17408 + Bits(22), 4211712 + Bits(30))`; the input sets the
  last selector with all 30 bits, 4,211,712 + 2^30 - 1 = 1,077,953,535 bytes.
  `cargo fuzz tmin` reduced it to 12 bytes (11 without the selector,
  `ff 0a 30 40 10 09 01 ff ff ff ff`), kept as
  `tests/fixtures/hostile/jxl-oversized-toc-section.jxl`.
- **Original.** libjxl at the pinned `a16d32d0` (`djxl`, scalar build) reads
  sections in place and checks `pos + toc_entry.size <= avail_in`
  (`dec_frame.cc`); it rejects both the 284-byte and the 11-byte codestream
  as truncated (also with `--allow_partial_files`), peak footprint 1.9 MB.
  The workspace libjxl-rs reader accepts the header and rejects the decode
  as `MalformedInput` (under the strict policy the header's color is refused
  first, `UnsupportedColor`).
- **Fix (adapter, not jxl-oxide).** The jxl-oxide adapter now passes an
  `AllocTracker` whose limit is `max_input_bytes`, the engine's
  compressed-input cap, while jxl-oxide reads the file; jxl-frame charges
  each section buffer to it before reserving. Every section of an admitted
  file fits in that cap, truncated files included, so only a declared size
  that no admitted input can hold is refused. The refusal is
  `AllocationFailed`, which the facade reports as `ResourceExhausted`, as
  the workspace reader reports its own allocation failures. After the read
  the limit is lifted: render buffers are sized from the header after the
  pixel check and stay outside any limit (#200). Tests:
  `hardening_tests::jxl_oxide_oversized_toc_section_is_resource_exhaustion`,
  `public_transcode_bounds_jxl_oxide_frame_data_by_the_input_cap` (512 KiB
  and the 64 MiB default), `workspace_jxl_oversized_toc_section_is_a_controlled_error`,
  and the fuzz crate's `oversized_jxl_toc_section_is_a_controlled_error`;
  `seed_campaign.py` adds the seed to the `transcode`, `transcode_own_jxl`
  and `transcode_jxl_out` lanes, and `campaign_seeds_report_reachability`
  asserts its classification.
- **Neighbors (source review of the jxl-oxide 0.12.6 crates).** The TOC has
  at most 65,536 entries (and its permutation as many); splines are capped
  at min(2^24, pixels / 4) and patches by libjxl's limits; MA-tree lookup
  tables at 1,024 entries; extra-channel and frame names at 1,071 bytes.
  One more allocation is sized from a field without a bound by the
  remaining input: `jxl_color::icc::read_icc` resizes the encoded ICC
  buffer to `enc_size` (at most 2^28 bytes, zero-filled) before decoding
  it, where libjxl grows it in 1 KiB steps as symbols arrive; and because
  `JxlImageBuilder::read` retries header parsing after each 4 KiB chunk,
  a truncated stream repeats that allocation per chunk. The tracker does
  not cover it and the adapter cannot reach it without parsing the header
  itself, so it is recorded for #200 and #213, not fixed.

Local ASan reruns after the fix (2026-11-15), 600 s each from the
`seed_campaign.py` seeds of each lane, with the smoke's limits:

| | `transcode` | `transcode_jxl_out` | `transcode_own_jxl` |
|---|---|---|---|
| Features | default (`cli`) | default (`cli`) | `own-jxl` |
| Seed files | 17 | 48 | 12 |
| `-max_len` / `-timeout` (s) | 16,384 / 5 | 65,536 / 10 | 16,384 / 5 |
| Executions | 193,915 | 37,114 | 395,573 |
| Coverage at `INITED` | `cov: 22820 ft: 31935` | `cov: 56105 ft: 126495` | `cov: 14504 ft: 17342` |
| Final coverage (`DONE`) | `cov: 37097 ft: 90214` | `cov: 65147 ft: 178331` | `cov: 20717 ft: 42687` |
| Retained corpus files | 3,245 | 1,972 | 2,102 |
| libFuzzer peak RSS (MB) | 483 | 663 | 538 |
| Crashes, leaks, timeouts, OOM, sanitizer reports | none | none | none |

Settings: macOS 27.2.1 arm64 (Apple M1 Pro), shared with other agents'
builds (execution counts are not throughput evidence); `rustc
1.100.0-nightly (5a2be9f5f 2026-09-06)`; cargo-fuzz 0.13.2; `--sanitizer
address`, `RUSTFLAGS=--cfg libavif_rs_asan`; `-seed=1 -max_total_time=600
-rss_limit_mb=1024` (so `-malloc_limit_mb` is 1,024 as in the smoke). The
own-JXL lane's length control stayed at 274 bytes, so it explored only
small codestreams. Before the fix the same `transcode` binary reproduced the
hosted `malloc(1077953535)` on the original input; afterwards it is
`ResourceExhausted`. Source: branch `v1/fuzz-oom-jxl`. The corpora stayed on
the local host. This is a bounded local run, not the sustained campaign.

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
  the fixes and record its eight jobs here.
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

The separate [M1 Decoder Fuzz Smoke](../.github/workflows/m1-fuzz-smoke.yml) uses the root pinned nightly and cargo-fuzz 0.13.2, verifies deterministic seed success, seeds eight input/output pairs from four real JPEG/JXL/AVIF/WebP fixtures, and runs **3,000 bounded attempts** with a 16 KiB maximum input and a 1 GiB process RSS cap. It then generates the campaign seeds, classifies them with `campaign_seeds_report_reachability` (the JPEG XL output lane's invariants included), and runs 3,000 bounded attempts of `transcode_jxl_out` from its 48 seeds with a 64 KiB maximum input, a 10-second timeout and the same RSS cap. This is useful to catch shallow codec/protocol panics and integration bugs. It does **not** constitute a sustained, coverage-reviewed fuzz campaign and is not proof of process-wide memory safety; actual release acceptance still requires broader corpora, sanitizers and target-specific time budgets.

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
| Pixel cap, JXL-Oxide | Construction parses the header and buffers frame data (bounded by the compressed size); the facade checks the pixel cap before rendering. Unrenderable colour encodings are rejected at construction (above). Obsolete since 2026-11-15: jxl-oxide is not a reader any more. |
| Geometry overflow | Pixel counts use `u64`; budget reservations use `checked_add`/`checked_mul` and fallible `try_reserve_exact`; region and row sizes use checked conversions. |
| Engine allocation admission | `MemoryBudget` (`max_managed_bytes`, default 1 GiB since 2026-11-15, 256 MiB before; the fuzz driver sets its own 2 MiB) admits every engine row, frame, orientation and encoder-frame buffer; the 8192×1 driver lane is rejected by planning before encode. |
| Opaque codec memory | Not admitted: libjpeg-turbo-rs, JXL-Oxide render buffers, libavif/AOM parser state, libjxl-rs, libwebp-rs, rav1e and libavif encoder internals. Reports flag this with `opaque_decoder_memory`/`opaque_encoder_memory`. Since 2026-11-15 JXL-Oxide's `AllocTracker` bounds its buffered frame data by `max_input_bytes` while the file is read ([run 43690971661](#hosted-smoke-run-43690971661-2026-11-15)); its render buffers and its encoded-ICC buffer were still unbounded when jxl-oxide left the runtime on 2026-11-15, which made both moot. |
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
