# Use C2Rust and preserve preprocessor configuration explicitly

Use C2Rust 0.21.0 with LLVM 16 to translate the library's actual CMake compilation
database. Keep the generated C-shaped Rust and narrowly documented mechanical
compiler fixes; record all translation inputs and fixes in executable tooling.
Because preprocessing selects a concrete feature and host configuration, each
additional configuration requires its own translation and parity evidence.
