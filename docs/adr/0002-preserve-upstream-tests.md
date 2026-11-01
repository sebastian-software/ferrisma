# Retain an immutable upstream snapshot and use its original tests

Vendor the complete tracked upstream tree at an exact commit, with a per-file
SHA-256 manifest, and keep its tests, fixtures, headers, and CMake files unchanged.
Build the C implementation as the reference and link the same C/C++ tests and
applications against the Rust implementation through the original C ABI; this
keeps the equivalence oracle independent of the translation.
