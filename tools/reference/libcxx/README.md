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

The original LLVM notices remain in each header. See
[Apache-2.0](../../../licenses/LLVM-Apache-2.0.txt) and the
[LLVM exception](../../../licenses/LLVM-exception.txt). The Rust translation
retains the same license and identifies its changes as a translation.
