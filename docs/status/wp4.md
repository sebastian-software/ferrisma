# WP4: pixel ops

- Status: not started
- Sub-wave: 1b
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1
- Probes: `tools/oracle/probes/wp4/`; tests: `tests/wp4_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/utils/filters_utils.rs` | `src/utils/filters_utils.c`, `src/utils/filters_utils.h` | no |
| `src/utils/rescaler_utils.rs` | `src/utils/rescaler_utils.c`, `src/utils/rescaler_utils.h` | no |
| `src/utils/quant_levels_utils.rs` | `src/utils/quant_levels_utils.c`, `src/utils/quant_levels_utils.h` | no |
| `src/utils/quant_levels_dec_utils.rs` | `src/utils/quant_levels_dec_utils.c`, `src/utils/quant_levels_dec_utils.h` | no |
| `src/dsp/dsp.rs` | `src/dsp/dsp.h` | no |
| `src/dsp/cpu.rs` | `src/dsp/cpu.c`, `src/dsp/cpu.h` | no |
| `src/dsp/yuv.rs` | `src/dsp/yuv.c`, `src/dsp/yuv.h` | no |
| `src/dsp/upsampling.rs` | `src/dsp/upsampling.c` | no |
| `src/dsp/alpha_processing.rs` | `src/dsp/alpha_processing.c` | no |
| `src/dsp/filters.rs` | `src/dsp/filters.c` | no |
| `src/dsp/rescaler.rs` | `src/dsp/rescaler.c` | no |
| `src/dsp/ssim.rs` | `src/dsp/ssim.c` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
