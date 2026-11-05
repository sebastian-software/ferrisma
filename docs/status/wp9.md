# WP9: encoder picture and config

- Status: not started
- Wave: 2
- Model: Claude Haiku 5.5 (`claude-haiku-5-5`)
- Depends on (catalog): WP1, WP4, WP5
- Probes: `tools/oracle/probes/wp9/`; tests: `tests/wp9_*.rs`

Written by the package session; the integrator aggregates it into
[PORT_STATUS.md](../PORT_STATUS.md). See the package's row in
[WORKPLAN.md](../WORKPLAN.md) for its verification inputs, and the
[findings from WP0](../WORKPLAN.md#findings-from-wp0) for dependencies
that the catalog does not declare.

## Inventory

| Rust module | Upstream units | Translated |
| --- | --- | --- |
| `src/enc/config_enc.rs` | `src/enc/config_enc.c` | no |
| `src/enc/picture_enc.rs` | `src/enc/picture_enc.c` | no |
| `src/enc/picture_csp_enc.rs` | `src/enc/picture_csp_enc.c` | no |
| `src/enc/picture_rescale_enc.rs` | `src/enc/picture_rescale_enc.c` | no |
| `src/enc/picture_tools_enc.rs` | `src/enc/picture_tools_enc.c` | no |
| `src/enc/picture_psnr_enc.rs` | `src/enc/picture_psnr_enc.c` | no |

## Evidence

None yet.

## Not-applicable checks

None recorded yet.

## Recorded deviations

None recorded yet.
