# Living architecture decision records

Each numbered file contains the current decision for one topic. Accepted
records are edited in place; update their `Updated` date when their meaning
changes. Git history preserves previous decisions.

Use `proposed`, `accepted`, `rejected` or `deprecated` as the status.
Implementation status belongs in [PORT_STATUS.md](../PORT_STATUS.md), not in an
ADR. A work package that meets a question these records do not answer stops and
reports it.

- [0001: Port the libjpeg-turbo decoder mechanically, 8-bit first](0001-mechanical-port.md)
- [0002: Compare against the original C decoder in separate processes](0002-differential-oracle.md)
- [0003: Build on stable Rust with unsafe code denied by default](0003-toolchain-and-unsafe-policy.md)
- [0004: Port SIMD after the portable decoder, as intrinsics on both architectures](0004-simd-after-the-portable-port.md)
- [0005: Represent the libjpeg object model in safe Rust](0005-rust-representation.md)
- [0006: Offer a safe row-streaming decoder over the translated API](0006-safe-decoding-api.md)
- [0007: Translate the NEON kernels with unsafe code only for memory access and entry](0007-neon-kernels-and-unsafe.md)
