# WP5: `SharpYUV`

- Status: not started
- Sub-wave: 1b
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1, WP4; may start next to WP4 and merges after it, because only
  `sharpyuv_cpu.rs` (which includes `src/dsp/cpu.c`) and `SharpYuvInit` need WP4
- Probes: `tools/oracle/probes/wp5/`; tests: `tests/wp5_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/sharpyuv/sharpyuv.rs` | `sharpyuv/sharpyuv.c`, `sharpyuv/sharpyuv.h` | no |
| `src/sharpyuv/sharpyuv_cpu.rs` | `sharpyuv/sharpyuv_cpu.c`, `sharpyuv/sharpyuv_cpu.h` | no |
| `src/sharpyuv/sharpyuv_csp.rs` | `sharpyuv/sharpyuv_csp.c`, `sharpyuv/sharpyuv_csp.h` | no |
| `src/sharpyuv/sharpyuv_dsp.rs` | `sharpyuv/sharpyuv_dsp.c`, `sharpyuv/sharpyuv_dsp.h` | no |
| `src/sharpyuv/sharpyuv_gamma.rs` | `sharpyuv/sharpyuv_gamma.c`, `sharpyuv/sharpyuv_gamma.h` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
