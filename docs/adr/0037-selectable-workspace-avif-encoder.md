# ADR-0037: Select the workspace AVIF encoder through the facade

- **Status:** Experimental
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-11
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

## Review when

Reassess the selection and default when ADR-0025 evidence is complete, the
libavif port gains a stable toolchain or safe native API, the supported target
set changes, or a streaming encoder can demonstrably replace the full frame.

## Amendment log

- 2026-11-11 — Added opt-in facade and CLI selection through an independent
  safe writer bridge. The original Rav1e default and selection gates remain.
