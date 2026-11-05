# WP7: VP8L decoder

- Status: not started
- Sub-wave: 2a
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on: WP1, WP2, WP3, WP4
- Probes: `tools/oracle/probes/wp7/`; tests: `tests/wp7_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/dec/vp8l_dec.rs` | `src/dec/vp8l_dec.c` | no |
| `src/dec/vp8li_dec.rs` | `src/dec/vp8li_dec.h`; except `VP8LTransform` (WP3) | no |
| `src/dec/alphai_dec.rs` | `src/dec/alphai_dec.h` | no |
| `src/dec/vp8_dec.rs` (exception) | types `VP8Io`, `VP8IoPutHook`, `VP8IoSetupHook`, `VP8IoTeardownHook` from `src/dec/vp8_dec.h` | no |
| `src/dec/webpi_dec.rs` (exception) | types `WebPDecParams`, `OutputFunc`, `OutputAlphaFunc`, `OutputRowFunc` from `src/dec/webpi_dec.h` | no |
| `src/dec/webp_dec.rs` (exception) | functions `WebPIoInitFromOptions`, `WebPCheckCropDimensions`, `WebPCheckCropDimensionsBasic` from `src/dec/webp_dec.c` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
