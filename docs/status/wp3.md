# WP3: lossless shared

- Status: not started
- Sub-wave: 1c
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on: WP1, WP4
- Probes: `tools/oracle/probes/wp3/`; tests: `tests/wp3_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/utils/huffman_utils.rs` | `src/utils/huffman_utils.c`, `src/utils/huffman_utils.h` | no |
| `src/utils/huffman_encode_utils.rs` | `src/utils/huffman_encode_utils.c`, `src/utils/huffman_encode_utils.h` | no |
| `src/utils/color_cache_utils.rs` | `src/utils/color_cache_utils.c`, `src/utils/color_cache_utils.h` | no |
| `src/utils/palette.rs` | `src/utils/palette.c`, `src/utils/palette.h` | no |
| `src/dsp/lossless.rs` | `src/dsp/lossless.c`, `src/dsp/lossless.h` | no |
| `src/dsp/lossless_common.rs` | `src/dsp/lossless_common.h` | no |
| `src/utils/utils.rs` (exception) | functions `WebPGetColorPalette` from `src/utils/utils.c` | no |
| `src/dec/vp8li_dec.rs` (exception) | types `VP8LTransform` from `src/dec/vp8li_dec.h` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
