# ADR-0031: Experimental AVIF input through the workspace libavif port

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-11
- **Partially supersedes:** [0018](0018-optional-codec-and-cms-dependencies.md), the interim AVIF reader and its toolchain contract; [0024](0024-pluggable-encoders-and-libavif-rs-backend.md) and [0026](0026-codecs-in-one-repository-and-workspace.md), the AVIF reader/encoder link exclusion. [0025](0025-codec-selection-reference-conformance-and-image-scope.md)'s final selection gates remain in force.
- **Related:** [0015](0015-typed-output-color-signals.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0019](0019-source-side-orientation.md), [0025](0025-codec-selection-reference-conformance-and-image-scope.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [issue 54](https://github.com/sebastian-software/ferrixel/issues/54)

## Context

The former interim `avif-decode` reader used `rav1d`, whose `dav1d_*` C symbols collided
with the workspace libavif port's dav1d. It also materializes an opaque RGB
frame and requires three passes over the container. The workspace port exposes
libavif's raw C-style API and includes its Rust libaom reference decoder.
Moving the raw API into Ferrixel's engine would violate its `unsafe` rule.

## Decision

Introduce `ferrixel-avif-read`, a small crate that depends on the local
`libavif-rs` package but not on Ferrixel. It owns the decoder and memory reader,
sets `AVIF_CODEC_CHOICE_AOM` explicitly, guards cleanup, and writes interleaved
RGB or RGBA into a caller-owned buffer. It selects the primary image item
explicitly and rejects files carrying tracks, so container color, orientation,
and extent metadata refer to the decoded item. Ferrixel's private AVIF source owns the
pixel buffer under `MemoryBudget`, reports a full-frame fallback, and borrows
rows from it. AVIF input remains an optional, native-target and nightly-only
feature while the codec port has those requirements. The codec-free engine and
features without `avif-read`/`cli` retain stable Rust 1.98; the engine contains no `unsafe`.

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
requested straight. Orientation remains in Ferrixel's source wrapper, with
crops in upright display coordinates. Only AVIF `irot` and `imir` determine
display orientation; EXIF payload orientation is not applied or forwarded by
this pipeline. The decoded image's chroma siting is interpreted by libavif's
nearest-neighbor YUV-to-RGB converter.

This is an integration experiment, not completion of issue 54. Its requested
Argon evidence for dav1d remains open. Explicit libaom selection follows
ADR-0025's reference-origin route, but it does not establish complete AV1
conformance or justify a universal performance claim.

## Consequences

- The encoder adapter and AVIF input can link without selecting `rav1d`.
- The engine budget covers its full RGB output and F32 row scratch, while
  libavif and libaom scratch and retained compressed data remain opaque.
- An AVIF input build requires the pinned nightly on supported macOS ARM64 or
  GNU Linux x86-64. Codec-free and builds without `avif-read`/`cli` remain stable-compatible.
- `avif-parse` is removed. The handwritten sample helper is included in the
  port's generated module selectors; translated `read.rs` bytes and hashes
  remain unchanged. Selector hashes are updated in the translation manifests.

## Alternatives considered

- Use the port's dav1d by default. Deferred because its optional external
  Argon conformance corpus has not been run; it can be selected deliberately
  after the evidence and quality gates.
- Put raw calls in the engine. Rejected by its no-`unsafe` policy.
- Put the reader in `ferrixel-libavif`. Rejected because that encoder adapter
  already depends on Ferrixel, creating a Cargo dependency cycle.

## Validation

Focused source tests cover color conflict, primary-item versus track selection,
ICC copy limits, 8/10/12-bit normalization,
alpha, malformed input, pixel limits, budget release, selected color grid
tiles, and AVIF orientation fixtures from ImageMagick/libheif. The selected raw
output matches libavif's pinned C oracle byte for byte on eight fixtures. A
frozen pre-format-neutral harness at reviewed base `f0ffebf4` retains the
interim-reader comparison path after removal from the live dependency graph.
Scoped PR CI checks the combined reader/encoder feature graph on native GNU
Linux x86-64 and macOS ARM64. Wider quality and memory comparison and the
Argon/dav1d corpus remain gates for a final decoder selection under ADR-0025
and issue #54. The current own-reader default is experimental; it
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

## Amendment log

- 2026-11-11 — Introduced the bounded-output experimental adapter with
  explicit libaom decoding. It is the live AVIF input path on supported
  native/nightly builds. The frozen interim harness preserves comparison
  evidence while ADR-0025's final selection and issue #54 remain open.
