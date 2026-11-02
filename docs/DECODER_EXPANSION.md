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
C++ implementation in Debug, Release, and Rust 1.85, with ordinary and
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

## Current batch

The first batch translates the EPF, noise, patch, and spline scalar boundaries
and connects EPF/noise/splines to the first complete VarDCT profile. Local
complete-file evidence includes 305 successful original files and 4,582 error
observations, with combined filters, progressive passes, grayscale/RGB and alpha.
Both full local checks, including Rust 1.85 and ordinary/ASan/UBSan originals,
pass 466 tests, 275 differential groups and 893,681 exact defined requests per
configuration, plus all 72 explicit original-only boundary probes. The corrected alpha/patch component has passed native Linux and macOS CI, including the independently confirmed 48 undefined original tree boundaries. Native combined CI and integration into `main` remain required.

The next integration connects one retained state across reference/DC frames,
blending, animation, previews, and NONE/XYB Modular and VarDCT frames. Its seven
lifecycle groups retain 2,592 exact original requests, including the original
1,814-request inventory, resampling and multigroup cases, genuine DC3/DC4 writer
fixtures, and 156 positive-patch/truncated/mutated observations. A genuine
112-byte patch codestream reconstructs all 36,864 input words exactly.

With the verified alpha/patch prerequisite, all six local complete checks pass
494 tests, 294 differential groups and 907,035 exact defined requests each,
alongside the 72 source-only probes. The independent source audit repeats all
2,592 lifecycle requests under that combined configuration. Native CI and
integration into main remain required. Chroma, broader extra channels, CMS/HDR,
JPEG reconstruction, and the public incremental API remain open. The first-image
and VarDCT milestone records describe the merged baseline; unmerged work does
not change those claims.

The subsequent internal extra-channel packet connects associated/subsampled
alpha, independent channel depths, all source-defined extra-channel metadata,
and ordered spot rendering, with genuine successes and original failure paths.
Its complete-file scope uses bundle output and actual post-decode buffer
conversions; direct output-stage callbacks remain part of step 11. The packet's
focused/source evidence is recorded in PORT_STATUS.md. All six local full checks
pass: 530 tests, 318 differential groups and 1,032,779 defined requests per
configuration, with no failures or ignored tests. Native CI and integration
remain required; this does not complete the overall decoder expansion or close
the color/JPEG/public API steps.
