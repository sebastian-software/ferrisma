# Generate dav1d platform profiles from native headers

Generate macOS ARM64 and Linux x86-64 translations from their own original
reference builds and system headers. Keep Debug and Release preprocessing
separate on each target. This preserves platform errno values, pthread details,
CPU-affinity queries, thread naming, and glibc's original stack-size workaround
without hand-recreating operating-system branches in the decoder.

Use the same pinned C2Rust 0.21.0 and LLVM 16 translator and the same recorded
mechanical corrections. Keep source correspondence and native ABI/original-test
checks for every profile. Repeated generated declarations may be consolidated
only in separate work after parity; a macOS-generated configuration is not
accepted as Linux evidence.

Build the unchanged legacy translator with Rust 1.80.1. Newer standard-library
sorting rejects one of its comparison functions on the Linux declarations;
the decoder itself still uses the pinned nightly-2026-09-07 compiler. Require
fresh repeat generation rather than accepting a partial translator run.
