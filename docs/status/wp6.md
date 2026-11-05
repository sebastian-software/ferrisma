# WP6: VP8 decoder core

- Status: not started
- Sub-wave: 2b
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on: WP1, WP2, WP4, WP7
- Probes: `tools/oracle/probes/wp6/`; tests: `tests/wp6_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/dsp/dec.rs` | `src/dsp/dec.c` | no |
| `src/dsp/dec_clip_tables.rs` | `src/dsp/dec_clip_tables.c` | no |
| `src/dec/common_dec.rs` | `src/dec/common_dec.h` | no |
| `src/dec/vp8_dec.rs` | `src/dec/vp8_dec.c`, `src/dec/vp8_dec.h`; except `VP8Io`, `VP8IoPutHook`, `VP8IoSetupHook`, `VP8IoTeardownHook` (WP7) | no |
| `src/dec/vp8i_dec.rs` | `src/dec/vp8i_dec.h` | no |
| `src/dec/tree_dec.rs` | `src/dec/tree_dec.c` | no |
| `src/dec/quant_dec.rs` | `src/dec/quant_dec.c` | no |
| `src/dec/frame_dec.rs` | `src/dec/frame_dec.c` | no |
| `src/dec/io_dec.rs` | `src/dec/io_dec.c` | no |
| `src/dec/alpha_dec.rs` | `src/dec/alpha_dec.c` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
