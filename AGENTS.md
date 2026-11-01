# Project instructions

- Use US English for code, documentation, ADRs, and Conventional Commits.
- Preserve the pinned libavif implementation mechanically. Keep names, control
  flow, C layouts, numeric behavior, allocation ownership, and diagnostics.
- Do not introduce Rust-specific redesigns or optimizations during the port.
- Record lasting implementation decisions in `docs/adr/` and possible later
  improvements in `docs/FUTURE.md`.
- Keep `upstream/libavif` byte-for-byte identical to the pinned source. Integrate
  the original tests externally; do not weaken assertions or modify fixtures.
- Never satisfy missing functionality with stubs or a hidden C fallback.
- Validate and commit coherent intermediate states separately using Conventional
  Commits. Local commit signing is intentionally disabled for this repository.
- Keep platform, feature, and test limitations explicit in `docs/PORTING.md`.
