# WP2: bit I/O

- Status: not started
- Wave: 1
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on (catalog): WP0 only
- Probes: `tools/oracle/probes/wp2/`; tests: `tests/wp2_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs, and the
[findings from WP0](../WORKPLAN.md#findings-from-wp0) for dependencies
that the catalog does not declare.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/utils/bit_reader_utils.rs` | `src/utils/bit_reader_utils.c`, `src/utils/bit_reader_utils.h` | no |
| `src/utils/bit_reader_inl_utils.rs` | `src/utils/bit_reader_inl_utils.h` | no |
| `src/utils/bit_writer_utils.rs` | `src/utils/bit_writer_utils.c`, `src/utils/bit_writer_utils.h` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
