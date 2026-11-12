# Promise Ferrisma a safe API, a stable toolchain, a pinned revision and stated semantics

- Status: accepted
- Created: 2026-11-09
- Updated: 2026-11-09

## Context

libwebp-rs exists to give [Ferrisma](https://github.com/sebastian-software/ferrisma)
WebP input and output behind its `webp-read` and `webp-write` features.
Ferrisma admits a codec dependency under the gates of its
[ADR-0018](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0018-optional-codec-and-cms-dependencies.md):
(a) a safe Rust API, (b) a stable toolchain, (c) a tagged release or a pinned
Git revision, (d) CI evidence on Ferrisma's targets, and (e) stated semantics
for color, alpha and orientation. Its sources follow the
[ADR-0012](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0012-codec-source-contract.md)
contract (describe the source before decoding; stream borrowed rows of a region
into engine-budgeted memory; declare the access class honestly), and its
orientation policy is
[ADR-0019](https://github.com/sebastian-software/ferrisma/blob/main/docs/adr/0019-source-side-orientation.md)
(the adapter reports the EXIF orientation 1 to 8, the engine applies it). The
research plan that founded this repository stated these gates as goals; this
record states what the library promises now that the translation is complete.

## Decision

- **(a) The safe API is the contract.** Ferrisma uses `webp::api::decode`
  ([ADR 0007](0007-safe-decoder-api.md)) and `webp::api::encode`
  ([ADR 0006](0006-safe-encoder-api.md)) and nothing else of this crate. The
  translation modules are public for the tools and tests of this repository,
  not an interface Ferrisma depends on; the C ABI (`src/abi`,
  [ADR 0004](0004-c-abi-boundary.md)) exists for the unmodified upstream tools
  of the differential tests and will move into a separate crate that Ferrisma
  never links. The library contains no `unsafe` outside that module and the
  SIMD modules of [ADR 0008](0008-simd-modules.md), whose two operations are
  argued locally.
- **(b) Stable Rust**, `rust-version = "1.95"`, edition 2024, no dependencies
  ([ADR 0005](0005-toolchain-and-unsafe-policy.md)); the `simd` feature is on
  by default and `--no-default-features` is the scalar translation.
- **(c) Pinned revisions.** Ferrisma pins a revision of `main` whose
  `Checks` workflow is green, or a pre-release tag placed on such a revision
  (`v0.1.0-alpha.N` until the API settles; the crate is not published). A
  change to `webp::api` updates its ADR in place and gets a new tag.
- **(d) CI evidence.** The `Checks` workflow runs on Linux x86-64 and macOS
  ARM64 (Ferrisma's targets): the static checks, the tests in three profiles
  with and without SIMD, the MSRV job and the oracle jobs that compare every
  unit with libwebp v1.6.0 built in the reference configuration, in the
  `release` and `asan-ubsan` variants, with upstream's own test scripts through
  the unmodified tools linked against this library. The workflow's runs on
  `main` are the record.
- **(e) Semantics.** The library adds no behavior: a decoding or encoding
  through the safe API is byte-identical to libwebp v1.6.0 with the same
  options. The statements for color, alpha and orientation are recorded as the
  gate (e) sections of ADR 0006 and ADR 0007: lossy pictures are ITU-R BT.601
  Y'CbCr 4:2:0 in limited range (Y 16 to 235, Cb and Cr 16 to 240), converted
  with the fixed-point matrix of `src/dsp/yuv.h` in both directions; lossless
  pictures are exact ARGB; alpha is straight unless a premultiplied mode is
  chosen; WebP carries no color signal besides an `ICCP` chunk and no
  orientation besides an `EXIF` chunk, both returned as raw payloads for
  Ferrisma to parse and apply; 8 bits per channel at most. Cropping follows the
  measurement of WP12 (`tests/wp12_crop_rows.rs`: a crop saves the rows below
  it for VP8 and VP8L, nothing horizontally, and the incremental decoder saves
  nothing for VP8 parsing), so a `webp-read` source may claim only that;
  libwebp's internal memory is not bounded by the crop and is reported as
  unbounded. The encoder needs the whole picture before it analyzes, so a
  `webp-write` sink declares and budgets one output frame. An animation decoder
  allocates two canvases of `4 * width * height` bytes, to be budgeted before
  construction.

## Consequences

Ferrisma's adapters (sebastian-software/ferrisma#15 for `webp-read`,
sebastian-software/ferrisma#16 for `webp-write`) can be written against the
safe API and its status-page notes without reading the translation. The
promises bind this repository: a change of an exposed semantic, of the
toolchain or of the API shape is an ADR amendment and a new tag, never a
silent change. The C ABI's move out of the crate (issue #31) completes gate
(a) as ADR-0018 words it.

## Review triggers

Revisit when Ferrisma's ADR-0018 changes its gates, when the first tag is
placed (issue #30), when the C ABI moves into its own crate, or when the
upstream pin advances.
