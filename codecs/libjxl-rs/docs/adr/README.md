# Living architecture decision records

Each numbered file contains the current decision for one topic. Accepted
records are edited in place; update their `Updated` date when their meaning
changes. Git history preserves previous decisions. This makes the current
direction easier to find, at the cost of relying on Git history for the audit
trail. Do not create a successor merely to amend the same decision.

Use `proposed`, `accepted`, `rejected`, or `deprecated` as the status. Keep
context, decision, consequences, and review triggers concise. Implementation
status belongs in [PORT_STATUS.md](../PORT_STATUS.md), not in an ADR.

- [0001: Preserve pinned upstream behavior before redesign](0001-mechanical-port.md)
- [0002: Compare against the original C++ implementation](0002-differential-oracle.md)
- [0003: Map ownership and storage explicitly](0003-rust-representation.md)
