# WP8: demux

- Status: not started
- Sub-wave: 3b
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1, WP12
- Probes: `tools/oracle/probes/wp8/`; tests: `tests/wp8_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

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
