# Preserve pinned upstream behavior before redesign

- Status: accepted
- Created: 2026-10-31
- Updated: 2026-10-31

The first phase translates libjxl into Rust while keeping the source and target
easy to compare. Combining the port with algorithm, API, or performance changes
would make discrepancies harder to locate.

Preserve source module boundaries, type names, control flow, tables, field
ordering, numeric widths, and existing optimized algorithms. Apply Rust's naming
conventions to functions and constants. Translate language-dependent ownership,
error propagation, and pointer representations explicitly; record consequential
differences in [0003](0003-rust-representation.md). Do not substitute a different
codec implementation or delegate ported operations to C++.

The Git submodule at `upstream/libjxl` pins the complete source. Advance that
revision only as a separate, reviewed migration with renewed parity evidence.
Port dependency boundaries incrementally, with a tested bitstream/header slice
before image decoding and encoding. Existing test cases remain part of the
migration backlog until translated or explicitly accounted for.

This preserves traceability at the cost of a less idiomatic initial Rust API.
Revisit this decision after the complete migration contract passes, or if a
specific source construct has no sound Rust representation. Compilation alone
does not establish equivalence.
