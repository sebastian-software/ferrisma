# WP7: VP8L decoder

- Status: not started
- Wave: 2
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on (catalog): WP1, WP2, WP3, WP4
- Probes: `tools/oracle/probes/wp7/`; tests: `tests/wp7_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs, and the
[findings from WP0](../WORKPLAN.md#findings-from-wp0) for dependencies
that the catalog does not declare.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/dec/vp8l_dec.rs` | `src/dec/vp8l_dec.c` | no |
| `src/dec/vp8li_dec.rs` | `src/dec/vp8li_dec.h` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
