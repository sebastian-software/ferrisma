# Decoder expansion

This work completes the decoder expansion after the first complete VarDCT
profile. The fixed Thunder and VarDCT profiles are the starting point, not the
acceptance boundary for this work. The remaining decoder items in the
[README checklist](../README.md#remaining-decoder-work) define the scope.
The [porting contract](PORTING.md) and accepted [ADRs](adr/README.md) continue to
apply: translate the pinned implementation mechanically, preserve defined
behavior, and keep C++ exclusively in the independent test oracle.

## Delivery and acceptance

Deliver independent source boundaries through isolated worktrees, unsigned
Conventional Commits, and pull requests. Integrate them serially after source
review and the applicable local and native CI gates. Component parity alone
does not complete a whole-file milestone. Update the README checkboxes only
when their stated scope is implemented, verified, and merged into `main`.

For each boundary, preserve statuses, partial state, reader consumption and
close obligations, reference lifetimes, callback ordering, metadata, and every
initialized output sample. Compare genuine original inputs with the independent
C++ implementation in the `checked` test profile, with ordinary and
ASan/UBSan references on the established native platform matrix. Retain all
existing cases, exact float comparisons, source pins, and sanitizer coverage.
Any compiler-dependent numeric boundary needs the same evidence and explicit
recording required by ADR 0002; passing one local build is insufficient.

## Work sequence

1. Complete EPF filtering: all three scalar stages, sigma ownership and borders,
   pipeline ordering, and complete files with one, two, and three iterations.
2. Complete noise parsing, seeded input generation, convolution, and application,
   including complete-file integration and frame-index-dependent seeds.
3. Complete patch parsing, reference checks, interval tree, blending, and rendering.
4. Complete spline parsing, quantization/dequantization, draw caches, and rendering.
5. Expand complete-file Modular decoding beyond the fixed Thunder configuration.
6. Connect reference/DC frames, blending, animation, and previews, preserving frame
   lifecycle and displayed/non-displayed frame accounting.
7. Connect chroma subsampling, cropping, resampling, and orientation in complete
   image decoding.
8. Support associated/subsampled alpha and the other extra-channel types.
9. Connect alternate color profiles, ICC/default CMS integration, and HDR/tone
   mapping beyond the admitted sRGB profiles.
10. Reconstruct original JPEG bitstreams and metadata.
11. Port the public incremental decoder API, events, callbacks, and progressive
    output, including required container input handling.

Each step needs representative successful files and malformed, truncated, or
otherwise failing inputs at its actual source boundary. Interacting features
need combined files as well as isolated component cases. The full objective is
complete only when every step has source-mapped implementation and matching
acceptance evidence; an intermediate EPF or effects PR does not complete it.

General encoding, the public encoder API, remaining optimized SIMD targets,
custom allocators/runners, the low-memory pipeline, and full-library release
acceptance remain separate checklist items. Dependencies reached by the decoder
work must still be translated before exposing the corresponding decoder path.

## Current integration

The first six steps are merged into `main` through
[PR #30](https://github.com/sebastian-software/libjxl-rs/pull/30). The internal
sequence decoder connects EPF/noise/patch/spline rendering, general NONE/XYB
Modular and VarDCT frames, shared reference/DC state, blending, animation and
previews. It also connects the admitted sRGB YCbCr/chroma configurations.

The final [native acceptance run](https://github.com/sebastian-software/libjxl-rs/actions/runs/41493531478)
passes Debug, Release and Rust 1.85 against ordinary and ASan/UBSan originals on
both Linux and macOS. Each configuration passes 307 differential groups and
926,392 exact defined requests, with zero failures or ignored tests. Linux runs
514 tests per configuration; macOS runs 513. The complete local gates, source
binding audits, all prior lifecycle requests and original-only probes also pass.

The retained lifecycle corpus contains 2,592 exact requests, including its
original 1,814-request inventory, resampling/multigroup cases, genuine DC3/DC4
writer fixtures and 156 positive-patch/truncated/mutated observations. A genuine
112-byte patch codestream reconstructs all 36,864 input words exactly. The
independent source audit repeats the full lifecycle corpus under both original
graphs. Detailed source mappings and component evidence remain in
[PORT_STATUS.md](PORT_STATUS.md).

Step eight is merged through [PR #31](https://github.com/sebastian-software/libjxl-rs/pull/31).
It connects associated/subsampled alpha, independent channel depths, every
source-defined extra-channel type and ordered spot rendering at the internal
sequence boundary. The [native acceptance run](https://github.com/sebastian-software/libjxl-rs/actions/runs/41494057418)
passes all six configurations on Linux and macOS: 318 differential groups and
1,032,779 exact defined requests each, with zero failures or ignored tests.
Linux runs 531 tests per configuration; macOS runs 530. The independent audit
also verifies 111 genuine complete files, 12 rejected-type fixtures and all
15,315 new defined observations. All 888 original half-stride probes remain
accounted for, including 528 positive original misalignment diagnostics.

Step seven is merged through [PR #32](https://github.com/sebastian-software/libjxl-rs/pull/32).
The sequence decoder connects every parsed YCbCr channel mode, source cropping
and resampling, and external orientation conversion. Its [native acceptance run](https://github.com/sebastian-software/libjxl-rs/actions/runs/41505031128)
passes all six configurations on Linux and macOS, each with 322 differential
groups and 1,091,467 exact defined requests, with zero failures or ignored tests.
Linux runs 536 tests per configuration; macOS runs 535. Both source audits
reproduce 128 genuine JPEG encodings, 20 sequence encodings, eight orientation
writes and 154 independent captures, and retain every one of the 58,688 geometry
requests. All 55,488 prior requests remain present with 3,200 cross-feature cases.
The merged runtime, fixtures, observers and CI are identical to the tested source;
the main merge updates only the checklist and acceptance documentation.

Steps nine through eleven remain open: ICC/default CMS and HDR, original JPEG
reconstruction, and the public incremental decoder/container API. Direct
output-stage callbacks remain part of step eleven. The fixed first-image and
VarDCT profile records retain their existing, narrower admission contracts.
