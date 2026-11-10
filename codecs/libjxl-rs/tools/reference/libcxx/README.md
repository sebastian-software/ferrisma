# Reference libc++ sorting source

These unchanged headers were captured from the installed reference macOS SDK
on 2026-11-01. `__config` identifies libc++ version 220106; `source.json` pins
each file by SHA-256. The repository check verifies these snapshots. They are
reference material, not a complete or compilable vendored C++ standard library.

`src/jxl/base/libcpp_sort.rs` translates the non-arithmetic comparator path in
`__algorithm/sort.h`, including the heap fallback through `partial_sort`,
`make_heap`, `sort_heap`, `pop_heap`, `push_heap`, and `sift_down`. This preserves
equivalent-element order in the pinned libjxl palette comparators. No arithmetic
branchless specialization, debug range randomization, or other library boundary
is claimed. The oracle calls the actual standard-library algorithms used by
its reference compiler, including a direct heap-fallback comparison.

`src/jxl/base/libcpp_nth_element.rs` translates `__algorithm/nth_element.h`
for a contiguous `float` range with `__less<>`: its `__sort3` is the
branchless arithmetic specialization of `sort.h`, and `__selection_sort` uses
`__min_element` from `__algorithm/min_element.h`. Quantizer::SetQuantField
selects its median and median absolute deviation with it. Only the selected
order statistics are observable there, and for comparable (non-NaN) inputs
their values, including the sign of a zero median after
ComputeGlobalScaleAndQuant's subtraction and clamp, do not depend on the
element permutation; NaN violates the comparator's strict weak ordering.

The original LLVM notices remain in each header. See
[Apache-2.0](../../../licenses/LLVM-Apache-2.0.txt) and the
[LLVM exception](../../../licenses/LLVM-exception.txt). The Rust translation
retains the same license and identifies its changes as a translation.
