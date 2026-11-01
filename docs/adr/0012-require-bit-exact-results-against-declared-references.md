# Require bit-exact results against declared references

For each port milestone, compare against a pinned original implementation and
an explicitly declared reference configuration. Require identical encoded
bitstreams and decoded pixels, together with matching return values and errors,
rather than accepting visual similarity or an existing test's numerical
tolerance as proof of an exact match.

Record source provenance, build and compiler options, CPU-path selection,
thread settings, inputs, and applicable parameters so the comparison has a
defined boundary. Preserve the original tests and supplement their evidence
with direct differential comparisons. Unexplained differences or reference
repeatability gaps remain open failures of the exact-match claim; do not
silently weaken assertions or introduce tolerances.

The existing libavif RGB rounding difference in
[ADR 0005](0005-floating-point-evaluation.md) remains an unresolved full-pipeline
verification item. It does not grant an exception to this new-port contract.
Public C++ binary compatibility is a separate, still-open decision.
