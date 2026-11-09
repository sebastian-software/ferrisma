# Living architecture decision records

Each numbered file contains the current decision for one topic. Accepted
records are edited in place; update their `Updated` date when their meaning
changes. Git history preserves previous decisions. Do not create a successor
merely to amend the same decision.

Use `proposed`, `accepted`, `rejected` or `deprecated` as the status. Keep
context, decision, consequences and review triggers concise. Implementation
status belongs in [PORT_STATUS.md](../PORT_STATUS.md), not in an ADR.

A work package that meets a question these records do not answer stops and
reports it instead of deciding alone ([WORKPLAN.md](../WORKPLAN.md)).

- [0001: Preserve pinned upstream behavior before redesign](0001-mechanical-port.md)
- [0002: Compare against the original C implementation in separate processes](0002-differential-oracle.md)
- [0003: Map pointers, errors, allocation, dispatch and threads explicitly](0003-rust-representation.md)
- [0004: Export the C ABI from one audited module](0004-c-abi-boundary.md)
- [0005: Build on stable Rust 1.95 with unsafe code denied outside the C ABI](0005-toolchain-and-unsafe-policy.md)
- [0006: Wrap the encoder translation in a safe, validated Rust API](0006-safe-encoder-api.md)
- [0007: Wrap the decoder translation in a safe, validated Rust API](0007-safe-decoder-api.md)
