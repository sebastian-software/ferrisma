# ADR-0025: Codec selection: reference or proven conformance, image scope, measured deviations

- **Status:** Accepted
- **Date:** 2026-11-06
- **Last reviewed:** 2026-11-10
- **Supersedes:** None; replaces the scaled-IDCT route in [0023](0023-decode-side-work-and-scaling-policy.md) decision 3
- **Related:** [0018](0018-optional-codec-and-cms-dependencies.md), [0023](0023-decode-side-work-and-scaling-policy.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0007](0007-correctness-and-performance-evidence.md), [JPEG and WebP evaluation](../research/2026-11-05-jpeg-and-webp-codec-evaluation.md), [libavif-rs](../../codecs/libavif-rs), [libjxl-rs](../../codecs/libjxl-rs)

## Context

For several formats Ferrixel now has more than one implementation within reach. Each one has to be built, tested and maintained:

- **AV1 encoders:**
  - `rav1e` is Ferrixel's built-in interim encoder.
  - libaom, through libavif-rs, is available as a backend (ADR-0024).
  - SVT-AV1 has not been evaluated.
- **AV1 decoders:**
  - `rav1d` serves `avif-read`.
  - libavif-rs contains both its dav1d port and the libaom decoder.
- **JPEG decoders:**
  - `zune-jpeg` is the interim decoder.
  - ADR-0023 needs a scaled IDCT for shrink-on-load. `zune-jpeg` does not have one.

Ferrixel's scope is still images and the operations Sharp offers. The maintainer stated on 2026-11-06 that the ported libraries do not need to keep functionality outside that scope, such as video coding tools, VMAF tuning modes or animation. Code that is ported but not used still has to be maintained whenever upstream changes are taken over.

The sister ports start mechanically. libavif-rs translates the pinned upstream C with C2Rust and records its fixes in tooling (its ADRs 0001, 0003 and 0013). While the port stays mechanical, a new upstream version means a new translation. The long-term goal, however, is to optimize the ports: better use of hardware, a better memory layout, idiomatic Rust. Once the code diverges from the translation, regenerating it is no longer the way to take over upstream changes.

The maintainer prefers reference implementations because they are the most likely to implement the format's original intent correctly. For decoders, a conformance suite proves correctness independently of where the implementation comes from.

Verified on 2026-11-06 in the `google/jpegli` source (BSD-3-Clause; the pinned libjxl no longer contains `lib/jpegli`, which corrects the 2026-11-05 JPEG evaluation):

- **API:** jpegli's decoder implements the libjpeg62 API and ABI.
- **Scaled decode:** `scale_num`/`scale_denom` yield a scaled IDCT of N/8 for N = 1…16. The scaled transforms use a generic scalar path; only the full 8×8 IDCT is dispatched through Highway SIMD. For chroma, the decoder prefers IDCT scaling over 2× upsampling.
- **Precision:** dequantization, IDCT, upsampling and color conversion run in `f32`. Output can be `u8`, `u16` or `f32`. Non-zero coefficients are dequantized to the expected value of their quantization interval under a Laplacian model, so the output deliberately differs from libjpeg's.
- **Not measured:** its speed and quality against `zune-jpeg` and libjpeg-turbo.

## Decision

1. **Origin rule.** Every codec that Ferrixel adopts for good is either the format's reference implementation (ported) or, for decoders only, an implementation with recorded conformance evidence from the format's suite: Argon for AV1, the JPEG XL conformance suite, or ISO/IEC 10918-2 for JPEG. An encoder's output is not defined by conformance, since any valid bitstream conforms. The reference is therefore the default encoder.
2. **Deviations need a clear measured advantage** under the criteria in point 3. Ferrixel also looks for highly optimized alternatives actively, for example in a survey before each codec decision, and treats them as candidates under the same rules.
3. **Selection criteria.** Measured on Arm and x86-64, only once every candidate has SIMD on both. On a corpus of Kodak images plus real photographs of 12–48 Mpx:
   - (a) For encoders: bytes at matched SSIMULACRA2 over several quality levels. For decoders: conformance and output quality.
   - (b) Time with all cores and with one thread.
   - (c) Peak memory.
   - (d) Binary size.
   - (e) Maintenance: the effort of taking over upstream changes, `unsafe`, and suitability as a base for optimization.
   - (f) Required features: 8/10 bits, alpha, 4:4:4, and scaled decode for JPEG.
4. **No removal before selection.** Alternative implementations and the interim crates stay until a candidate has won under point 3 and matches what it replaces.
5. **Ports are reduced to the image scope**, in this order:
   - (a) Whole components: one decoder and one encoder per format.
   - (b) Upstream configuration switches applied at translation time, for example libaom's `CONFIG_TUNE_VMAF=0` and, once dav1d decodes, `CONFIG_AV1_DECODER=0`.
   - (c) Removing code that still-image workloads do not reach, but only with measured coverage evidence.

   While the mechanical translation is still how upstream changes come in, nobody deletes code from it by hand.
6. **Optimization starts after the scope is fixed.** The mechanical translation of the pinned upstream stays as a bit-exact oracle. Optimized modules are checked against it: bit-exact, or measured for quality where an algorithm deliberately changes. Upstream changes are then taken over selectively: translate the new upstream version mechanically, diff the two translations, and port security, conformance and quality changes that touch the kept paths.
7. **JPEG: decode before encode.** JPEG output is not planned (ADR-0018), so JPEG compression efficiency has low priority. mozjpeg is not pursued: it is an encoder, its gains cost encode time, and its decoder is libjpeg-turbo. The JPEG decoder candidates for a sister port are libjpeg-turbo, derived from the IJG reference libjpeg and used by libvips and Sharp, and jpegli. Each needs ISO/IEC 10918-2 evidence. Either one brings a scaled IDCT and would replace the planned scaled-IDCT fork of `zune-jpeg` for ADR-0023's shrink-on-load. jpegli's `f32` output would feed Ferrixel's `f32` executor without an 8-bit step. **Chosen 2026-11-06: libjpeg-turbo** (see the amendment log); jpegli's expectation-value dequantization stays a candidate for a later, measured opt-in quality mode in that port.

## Consequences

+ Each format ends with one decoder and one encoder. Each was chosen by recorded evidence, and its correctness rests on the reference or on a conformance suite.
+ Upstream changes have to be followed only for the code Ferrixel keeps, and the oracle keeps every optimization honest.
+ Shrink-on-load gets the same basis as Sharp's (a libjpeg-API scaled IDCT) instead of a fork of a third-party crate.
- `rav1e`, `rav1d`, `jxl-oxide` and `zune-jpeg` are interim by policy, not only by current state. Replacing each one needs a port that passes point 3.
- A JPEG decoder port is a fourth sister project. Its size is not yet estimated.
- Fair comparisons need SIMD in every candidate on both architectures, which delays decisions. libaom in libavif-rs has Neon kernels since 2026-11-07 and SSE2, SSE4.1 and AVX2 kernels since 2026-11-09 (see the amendment log).
- Once a port is optimized, upstream changes have to be ported by hand. rav1d, a C2Rust translation of dav1d later refactored into idiomatic Rust, shows that this path works and costs effort for every line kept.

## Alternatives considered

- **Keep every implementation.** Rejected: two or three decoders and encoders per format cost build time, binary size, tests and upstream maintenance.
- **Choose whatever is available or fastest today.** That is the interim state of ADR-0018. Rejected as a final rule: it ties correctness to third-party crates and does not compare fairly before SIMD exists everywhere.
- **Reduce ports by hand right away.** Rejected while the mechanical translation is still the update path: deletions in generated code turn every upstream update into a manual merge.
- **Reference implementations only, decoders included.** Rejected: dav1d is not the AV1 reference but is conformance-tested and much faster than libaom's decoder. Conformance proves decoder correctness just as well.
- **A scaled-IDCT fork of `zune-jpeg`.** Kept only as a fallback if neither JPEG port happens: the fork would be a third-party code base outside the origin rule.

## Validation

- Every adoption or removal of a codec cites a recorded measurement under point 3, with machine, versions and command lines (ADR-0007).
- Every adopted decoder cites its conformance run; every optimized port module cites its differential tests against the oracle.
- The decision is falsified if a reference-derived or conformance-proven implementation shows a correctness defect that a third-party crate does not have, or if reducing a port to the image scope breaks an output Ferrixel produces.

## Review when

- libavif-rs merges its Neon kernels or gains x86 SIMD.
- Measuring the image-scope coverage of libaom is complete.
- The JPEG decoder candidates have been measured.
- An optimized third-party implementation qualifies as a candidate under point 2.
- Ferrixel's scope grows beyond still images, for example animation.

## Amendment log

- 2026-11-06 — Initial decision by the maintainer: reference or proven conformance, image scope, deviations only with a clear measured advantage, no removal before selection, a mechanical oracle for later optimization, JPEG decode before encode, and the jpegli source facts.
- 2026-11-06 — First JPEG decoder measurement ([M2-jpeg-decoders](../benchmarks/M2-jpeg-decoders.md), Apple M1 Ultra, one thread). libjpeg-turbo 3.2.1 decodes 1.4–1.6× faster than `zune-jpeg` and 1.7–2.7× faster than jpegli; jpegli scores 0.2–1.6 SSIMULACRA2 higher on the same files; `zune-jpeg` matches libjpeg-turbo's quality. Scaled decode saves at most 31–41 % of decode time and, under libvips' 2× residual rule, scores 89.7–96.5 against Ferrixel's linear-light result for both decoders. x86-64, other encoders' JPEGs and conformance are not yet measured; the choice of decoder is open.
- 2026-11-06 — The maintainer chose libjpeg-turbo as the JPEG decoder for the sister port. Reasons: it is the fastest decoder measured, it descends from the reference, and its speed ports to Rust. On Arm its SIMD is C intrinsics, which libjpeg-turbo's own changelog reports as faster than its former assembly. An estimate from the 1/8 timings puts about 70 % of its photo decode time in serial, scalar entropy decoding, and it has SIMD for the reduced-size IDCT as well. jpegli's gap comes from its `f32` pipeline: NEON processes half as many `f32` as `i16` lanes, so a port would not close it without changing the algorithm. The x86-64 SIMD is NASM assembly and has to be rewritten as intrinsics; x86-64 is not measured yet.
- 2026-11-09 — Review trigger "libavif-rs merges its Neon kernels or gains x86 SIMD" fired: libavif-rs merged Neon ([libavif-rs#9](https://github.com/sebastian-software/libavif-rs/pull/9)) and x86 SIMD ([libavif-rs#11](https://github.com/sebastian-software/libavif-rs/pull/11)), each byte-identical to its scalar path. At matched SSIMULACRA2, libavif-rs speed 6 is smaller and faster than `rav1e` on macOS ARM64 and x86-64 Linux ([M2-libavif-backend](../benchmarks/M2-libavif-backend.md)), so libaom is the measured AVIF encoder candidate under point 3. The encoder is not switched here; ADR-0024 keeps `rav1e` as the default while ADR-0018 gates (a) and (b) are open. libavif-rs [ADR 0059](../../codecs/libavif-rs/docs/adr/0059-freeze-the-libaom-translation-and-gate-owned-code-on-golden-outputs.md) also changes point 6 for libaom: its translation is frozen at libaom 3.15.2 and becomes owned code, checked against recorded golden outputs instead of a fresh translation. Upstream changes reach it by porting selected features, not by re-translating and diffing. The mechanical translation stays the reproducible starting point and the source of the golden outputs.

- 2026-11-10 — ADR-0026 brings all four ports into this repository and workspace, including their open PRs. This changes coordination and dependency placement, not codec selection, still-image scope or the adoption gates above.
