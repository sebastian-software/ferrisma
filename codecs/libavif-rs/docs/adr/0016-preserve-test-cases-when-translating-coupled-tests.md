# Preserve test cases when translating coupled tests

Keep upstream tests unchanged wherever they can exercise the Rust implementation
through a faithful boundary. Mechanically translate internally coupled C++
tests to Rust when retaining their implementation headers would otherwise test
the original algorithms instead of the port. Preserve the original tests as
the native reference.

Record a correspondence for test cases, parameter combinations, inputs,
assertions, fixtures, and relevant skip or conditional behavior. Translation is
not permission to simplify assertions, drop cases, change expected results, or
replace the original suite with new smoke tests. Unmapped cases remain explicit
coverage gaps. Supplemental differential tests are identified separately.

Verify that candidate test executables execute Rust algorithms rather than
original implementation objects or header-defined C++ algorithms. This policy
supports the Rust consumer goal in
[ADR 0015](0015-target-rust-consumers-without-cpp-binary-compatibility.md) without
requiring a binary-compatible C++ library.
