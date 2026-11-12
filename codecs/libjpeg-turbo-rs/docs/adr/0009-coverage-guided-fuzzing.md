# Fuzz the safe decoding API with cargo-fuzz in a separate crate

- Status: accepted
- Created: 2026-11-12
- Updated: 2026-11-12

## Context

The damaged-stream tests of work packages 2 to 6 compare corrupt input with
the oracle ([ADR 0002](0002-differential-oracle.md)), but they do not explore
inputs the way a coverage-guided fuzzer does
([ferrisma#73](https://github.com/sebastian-software/ferrisma/issues/73)).
Upstream's fuzzers in `upstream/libjpeg-turbo/fuzz/` are C programs over the
TurboJPEG and libjpeg C APIs, which the port does not export
([ADR 0003](0003-toolchain-and-unsafe-policy.md)). Fuzzing in Rust needs a
development dependency, which ADR 0003 leaves to an amendment.

The Ferrisma workspace already fuzzes with `cargo-fuzz` 0.13.2 and
`libfuzzer-sys` 0.4: the root `fuzz/` crate drives the engine, including JPEG
input through this port's safe API, and `codecs/libwebp-rs/fuzz/` is a
separate crate for the WebP port. The engine target reaches the decoder only
with the options Ferrisma's adapter uses.

## Decision

- **Tool.** `cargo-fuzz` 0.13.2 with `libfuzzer-sys` 0.4, as the rest of the
  workspace pins them. It needs the workspace's pinned nightly toolchain.
- **Crate.** The target lives in `fuzz/`, a separate crate with its own
  `[workspace]` and lockfile, as in libwebp-rs. It is not a member of the
  library's or the monorepo's workspace and never a dependency of the
  library. Its only dependency besides the library is `libfuzzer-sys`. The
  library itself gains no dependency, development or otherwise; its stable,
  MSRV and oracle checks never compile the fuzz crate.
- **Target.** `decode` takes the JPEG bytes themselves and runs each input
  through a fixed list of cases over the safe API of
  [ADR 0006](0006-safe-decoding-api.md) (`fuzz/src/lib.rs`): header, EXIF
  data, ICC profile, `output_info`, `start`, column crop with skipped rows,
  `read_rows` in strips, and `finish`; RGB with the fast IDCT and fast
  upsampling, BGRX at 1/2 with a crop, ABGR at 1/8, grayscale with the float
  IDCT and a crop, CMYK stopping at the first warning, YCbCr at 6/8, and the
  default output at 16/8. The cases follow upstream's `decompress.cc` and
  `decompress_libjpeg.cc` where the safe API has the same calls. Like
  upstream's targets, it skips images above one megapixel. Since the input
  is plain JPEG, upstream's `fuzz/jpeg.dict` applies. A `jpeg::Error` ends a
  case; a panic (overflow checks are on), a crash, a timeout or a sanitizer
  report is a finding.
- **Seeds.** `fuzz/seed_corpus.py` copies the JPEG inputs of the existing
  tests: upstream's `testimages`, the in-scope `djpeg` inputs of
  [tools/bittests.json](../../tools/bittests.json) that upstream's CTest suite
  writes with the oracle (progressive, arithmetic, 4:4:0, 3x2, grayscale,
  optimized and ICC files), and the engine's JPEG fixtures. It prints their
  SHA-256. The T.83 data stays out, since its license grants no
  redistribution and campaign artifacts upload the corpus. A unit test of the
  fuzz crate checks that every 8-bit seed finishes at least one case.
- **Schedule.** The nightly `M1 Decoder Fuzz Smoke` workflow runs a bounded
  smoke of the target, and the dispatched `M1 Sustained Fuzz Campaign` runs it
  for 30 minutes with AddressSanitizer on Linux x86-64 and macOS ARM64
  ([Ferrisma fuzzing policy](../../../../docs/fuzzing.md)). Both build the
  release oracle first to write the bit-test seeds.
- **Findings.** A finding is fixed in the port only where the port departs
  from upstream; the fix comes with a regression test against the oracle as
  ADR 0002 requires. A finding that upstream shares is reported, not
  "fixed", since the port preserves upstream's behavior
  ([ADR 0001](0001-mechanical-port.md)).

## Alternatives

- **Translating upstream's fuzzers.** They call TurboJPEG and the C API,
  which the port does not have; the safe API is what Rust callers use.
- **`afl.rs` or `honggfuzz-rs`.** Either adds a second toolchain to a
  workspace that already standardized on `cargo-fuzz`.
- **A fuzzer over the translated libjpeg API.** It would reach states the
  safe API cannot produce, for example calls in the wrong order, which
  upstream answers with `JERR_BAD_STATE`; the safe API is the boundary
  Ferrisma exposes to untrusted bytes. Revisit if a caller drives the
  translated API directly.
- **Only the engine target.** It reaches the decoder only through Ferrisma's
  adapter options, not scaling, the IDCT methods or CMYK output.

## Consequences

Fuzzing needs nightly, `libfuzzer-sys`, and for the full seed set CMake and a
C compiler for the oracle, only in the fuzz crate's workflow jobs. The target
decodes each input up to seven times, so it runs fewer inputs per second than
a single-decode target would; the cases trade speed for API coverage.

## Validation

`cargo test --manifest-path fuzz/Cargo.toml --lib` runs the cases on known
images and, with `LIBJPEG_FUZZ_SEEDS`, classifies every seed. A local
bounded campaign is recorded below; it is not a sustained campaign.

## Review when

- A finding shows a case or a limit hides a class of bugs.
- The safe API gains a call or an option (ADR 0006).
- The workspace changes its fuzzing tool or toolchain.
