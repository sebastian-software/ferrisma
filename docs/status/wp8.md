# WP8: demux

- Status: not started
- Wave: 2
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on (catalog): WP1
- Probes: `tools/oracle/probes/wp8/`; tests: `tests/wp8_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs, and the
[findings from WP0](../WORKPLAN.md#findings-from-wp0) for dependencies
that the catalog does not declare.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/demux/demux.rs` | `src/demux/demux.c` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
