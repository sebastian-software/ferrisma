# Living architecture decision records

Each numbered file contains the current decision for one topic. Accepted
records are edited in place; update their `Updated` date when their meaning
changes. Git history preserves previous decisions.

Use `proposed`, `accepted`, `rejected` or `deprecated` as the status. These
records are the only place for decisions: there is no separate plan, status or
verification document. What is verified is shown by the tests, which assert
the measured results. Open work is tracked as
[GitHub issues](https://github.com/sebastian-software/libjpeg-turbo-rs/issues),
not in an ADR. A change that meets a question these records do not answer
stops and reports it.

- [0001: Port the libjpeg-turbo decoder mechanically, 8-bit first](0001-mechanical-port.md)
- [0002: Compare against the original C decoder in separate processes](0002-differential-oracle.md)
- [0003: Build on stable Rust with unsafe code denied by default](0003-toolchain-and-unsafe-policy.md)
- [0004: Port SIMD after the portable decoder, as intrinsics on both architectures](0004-simd-after-the-portable-port.md)
- [0005: Represent the libjpeg object model in safe Rust](0005-rust-representation.md)
- [0006: Offer a safe row-streaming decoder over the translated API](0006-safe-decoding-api.md)
- [0007: Translate the NEON kernels with unsafe code only for memory access and entry](0007-neon-kernels-and-unsafe.md)
- [0008: Rewrite the x86-64 kernels from NASM as SSE2 and AVX2 intrinsics](0008-x86-64-kernels-from-nasm.md)
