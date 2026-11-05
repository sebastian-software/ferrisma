# WP1: types, memory, workers

- Status: not started
- Wave: 1
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on (catalog): WP0 only
- Probes: `tools/oracle/probes/wp1/`; tests: `tests/wp1_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs, and the
[findings from WP0](../WORKPLAN.md#findings-from-wp0) for dependencies
that the catalog does not declare.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/webp/types.rs` | `src/webp/types.h` | no |
| `src/webp/format_constants.rs` | `src/webp/format_constants.h` | no |
| `src/webp/decode.rs` | `src/webp/decode.h` | no |
| `src/webp/encode.rs` | `src/webp/encode.h` | no |
| `src/webp/demux.rs` | `src/webp/demux.h` | no |
| `src/webp/mux_types.rs` | `src/webp/mux_types.h` | no |
| `src/utils/utils.rs` | `src/utils/utils.c`, `src/utils/utils.h` | no |
| `src/utils/endian_inl_utils.rs` | `src/utils/endian_inl_utils.h` | no |
| `src/utils/random_utils.rs` | `src/utils/random_utils.c`, `src/utils/random_utils.h` | no |
| `src/utils/thread_utils.rs` | `src/utils/thread_utils.c`, `src/utils/thread_utils.h` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
