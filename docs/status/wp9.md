# WP9: encoder picture and config

- Status: not started
- Sub-wave: 2a
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on: WP1, WP3, WP4, WP5
- Probes: `tools/oracle/probes/wp9/`; tests: `tests/wp9_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs. The inventory
comes from `tools/units.json`, including the function- and type-level
exceptions inside a module.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/enc/config_enc.rs` | `src/enc/config_enc.c` | no |
| `src/enc/picture_enc.rs` | `src/enc/picture_enc.c`; except `Encode`, `WebPEncodeRGB`, `WebPEncodeBGR`, `WebPEncodeRGBA`, `WebPEncodeBGRA`, `WebPEncodeLosslessRGB`, `WebPEncodeLosslessBGR`, `WebPEncodeLosslessRGBA`, `WebPEncodeLosslessBGRA` (WP13) | no |
| `src/enc/picture_csp_enc.rs` | `src/enc/picture_csp_enc.c` | no |
| `src/enc/picture_rescale_enc.rs` | `src/enc/picture_rescale_enc.c` | no |
| `src/enc/picture_tools_enc.rs` | `src/enc/picture_tools_enc.c` | no |
| `src/enc/picture_psnr_enc.rs` | `src/enc/picture_psnr_enc.c` | no |
| `src/enc/webp_enc.rs` (exception) | functions `WebPEncodingSetError`, `WebPReportProgress` from `src/enc/webp_enc.c` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
