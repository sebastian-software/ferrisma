# Preserve pinned upstream behavior before redesign

- Status: accepted
- Created: 2026-10-31
- Updated: 2026-11-12

The first phase translates libjxl into Rust while keeping the source and target
easy to compare. Combining the port with algorithm, API, or performance changes
would make discrepancies harder to locate.

Preserve source module boundaries, type names, control flow, tables, field
ordering, numeric widths, and existing optimized algorithms. Apply Rust's naming
conventions to functions and constants. Translate language-dependent ownership,
error propagation, and pointer representations explicitly; record consequential
differences in [0003](0003-rust-representation.md). Do not substitute a different
codec implementation or delegate ported operations to C++.

The Git submodule at `upstream/libjxl` pins the complete source. Advance that
revision only as a separate, reviewed migration with renewed parity evidence.
Port dependency boundaries incrementally, with a tested bitstream/header slice
before image decoding and encoding. Existing test cases remain part of the
migration backlog until translated or explicitly accounted for.

The [first-image milestone](../FIRST_IMAGE.md) requires a complete Rust image
encode/decode path with cross-decoding against the original implementation.
Keep that integration gate distinct from completion of individual dependency
slices and from completion of the entire library port.

Connect that first path under one fixed, original encoder configuration before
exposing the general image API. A configuration-specific internal entry point
may translate only the branches reached by that configuration if its types
prevent callers from selecting unported branches. Preserve every reached
algorithm, parameter default, operation, and error checkpoint. Do not add a
general entry point containing unsupported branches or no-op substitutes for
missing dependencies. Record the original configuration in
[FIRST_IMAGE.md](../FIRST_IMAGE.md) and expand these entry points as additional
source paths are translated. This bounds the first integration dependency graph
without changing the full encoder/decoder port scope or its acceptance gates.

Explicitly select the original simple decoder pipeline through
use_slow_rendering_pipeline=true for this first configuration. A private
admitted-header type excludes unported frame branches before decoder output
initialization; the configuration continues to own immutable metadata. A native
Unsupported result at that admission boundary identifies input outside this
fixed profile and is not a claim about the general C++ decoder's status.
General decoding remains absent. The complete fixed entry point parses global
metadata into a private admitted owner, then follows the original frame and
integer-export path. Out-of-profile metadata and frame headers are admission
results, rather than substitutes for unported generic decoder branches. The
first-image gate includes whole-codestream parsing and integer samples beyond
raw-frame and Float32 component integration; its verified evidence belongs in
the milestone and port-status records.

Expand the fixed image profile to one optional, full-resolution, unassociated
alpha channel at the same 8/16-bit depth as color. Keep the no-alpha constructor
unchanged and use a distinct constructor for interleaved gray/alpha or RGBA.
Translate the reached color/alpha input, extra-plane preparation, modular
channel count, simple-pipeline channel count, and integer export branches as one
complete-file boundary. Preserve the original lossless keep-invisible default;
color values under alpha zero remain encoded. Other extra-channel types,
subsampling, associated alpha, and independently selected alpha depths remain
outside admission until their complete source paths are translated.

The public encoder API from `encode.cc` is admitted before the general frame
encoder underneath it is complete. Translate its object lifecycle, setters and
owners in source order, with every original argument, ordering and allocation
check. Until the last admission is removed, the encoder enforces a recorded
profile at run time instead of through types: a call that would enter a branch
of the original that is not yet translated returns `JXL_ENC_ERROR` and sets
`JXL_ENC_ERR_NOT_SUPPORTED` at the first such source checkpoint. Every check
that the original performs before that checkpoint keeps its original status,
error and stored state. An admission result is never a substitute for a missing
algorithm and never counts as parity: the differential corpus compares each
admitted history exactly up to that call and shows, from the original's own
state, that the original enters the excluded branch there. This is the only
general entry point with unported branches; the internal configuration entry
points above keep their type-level admission. Do not export the encoder C ABI
while any admission remains, because a C caller cannot tell an admission
result from a libjxl status. [ENCODER_EXPANSION.md](../ENCODER_EXPANSION.md)
records the admitted profile and its checkpoints; each step that translates an
excluded branch deletes its check.

Amendment (2026-11-12, encoder step 1 integration). The first admitted encode
profile is gated at EncodeFrame's choice of the one-shot path: settings,
metadata and input outside the compared profile return the `FrameProfile`
admission result there, even where the branches that follow are translated
but not yet compared with the original (other efforts, option overrides,
unclosed frames, crops, a custom memory manager). This is earlier than the
first untranslated branch and therefore conservative; it widens only with
oracle evidence. Branches that do enter untranslated units (streaming,
patches, the AR heuristics, alpha, ICC, containers) keep their own checkpoints,
and the original's branch evidence for each is recorded by `epub-probe`.

This preserves traceability at the cost of a less idiomatic initial Rust API.
Revisit this decision after the complete migration contract passes, or if a
specific source construct has no sound Rust representation. Compilation alone
does not establish equivalence.

The first complete VarDCT decode profile follows the original rendering test's
NoGabNoEpfNoPatches configuration plus its Gaborish variant: XYB 4:4:4, sRGB,
one final regular frame, no EPF, patches, splines, noise, previews, animation,
references, cropping or resampling. Support actual DC/AC groups, default adaptive
DC smoothing, all scalar inverse-transform strategies, coefficient widths,
multiple AC passes and the existing full-resolution unassociated-alpha profile.
Admit metadata and frame headers before unported branches become selectable.
Use the existing simple pipeline and original HIGH_PRECISION=1 sRGB stage.
Record full-file pixel/error evidence in VARDCT_IMAGE.md separately from the
remaining general decoder and encoder work. Container wrapping is outside the
raw entry point. Original level-10 encoder fixtures may call the actual internal
header/one-shot boundary to obtain raw codestreams instead of the public API's
mandatory container; no codestream framing algorithm is supplied by the fixture.

## Amendment log

- 2026-10-31: Accepted with the bitstream and header slice.
- 2026-11-01: Admitted configuration-specific internal entry points for the
  fixed lossless image path and its alpha profile.
- 2026-11-02: Added the first complete VarDCT decode profile.
- 2026-11-05: Admitted the public encoder API with explicit run-time admission
  results at the first unported source checkpoint.
