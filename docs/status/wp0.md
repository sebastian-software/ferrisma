# WP0: bootstrap

- Status: done on 5 November 2026 (local host); CI not run yet, because the repository has no remote
- Wave: 0
- Model: Claude Opus 5.5 (`claude-opus-5-5`)
- Depends on: nothing

## Inventory

WP0 translates no codec code. It created:

- The repository layout of [WORKPLAN.md](../WORKPLAN.md): Cargo package `libwebp-rs`
  (library `webp`; `rlib`, `staticlib`, `cdylib`; `publish = false`; edition 2024;
  `rust-version = "1.95"`; `checked` profile; `oracle` feature), `rust-toolchain.toml`,
  `LICENSE`, `PATENTS` and `AUTHORS` (unchanged copies of upstream `COPYING`,
  `PATENTS` and `AUTHORS`), `AGENTS.md`, `CONTRIBUTING.md`, `README.md`.
- Submodules `upstream/libwebp` (v1.6.0, `4fa21912338357f89e4fd51cf2368325b59e9bd9`) and
  `upstream/libwebp-test-data` (`06ddd96e276c2c638a72d39d3c0f340afd61978c`), pinned in
  `upstream.json` and checked by `tools/verify_upstream.py`.
- The module skeleton: 126 translation modules covering all 151 portable and SIMD
  upstream `.c` and `.h` files under `src/` and `sharpyuv/`, five C ABI modules and
  nine `mod.rs` files, each module with its provenance header only. The 25 MIPS and
  MSA files are excluded. `tools/units.json` maps every file; `tools/units.py` checks
  the headers and declarations.
- Documentation: [PORTING.md](../PORTING.md), [PROVENANCE.md](../PROVENANCE.md),
  [PORT_STATUS.md](../PORT_STATUS.md), [WORKPLAN.md](../WORKPLAN.md), a copy of the port
  plan as [PORT_PLAN.md](../PORT_PLAN.md), the [ADR index](../adr/README.md) with five
  accepted ADRs, and one status page per package.
- The oracle harness: `tools/build_oracle.py`, `tools/oracle/CMakeLists.txt`,
  `tools/oracle/probe_support/`, the probe `tools/oracle/probes/wp0/webp_get_features.c`,
  `tests/support/oracle.rs` and the differential test `tests/wp0_features.rs`.
- Tools: `tools/check.py`, `tools/oracle_baseline.py`, `tools/rust_name.py`,
  `tools/unit_dependencies.py`.
- CI: `.github/workflows/check.yml` with Linux x86-64 and macOS ARM64 checks, an
  MSRV job on Rust 1.95 and an oracle job that builds all three variants.

## Evidence

On the macOS ARM64 host described in [PORT_STATUS.md](../PORT_STATUS.md):

- `cargo build` and `python3 tools/check.py --oracle --msrv` pass: formatting,
  Clippy with `-D warnings` on all targets and features, the upstream pins, the
  module skeleton, documentation links, the tests in the dev, release and
  `checked` profiles with the release oracle required, and build and tests on
  Rust 1.95.
- `tests/wp0_features.rs` passes against the `release`, `asan-ubsan` and `debug`
  oracle variants: the probe reports status 0 and positive dimensions for all 131
  vectors (88 lossy, 43 lossless), matches the hand-checked expectation for
  `alpha_color_cache.webp` (588 by 97, alpha, not animated, lossy) from a path and
  from standard input, and keeps the reset fields with status 7 for a 20-byte prefix.
- `tools/oracle_baseline.py` passes with identical counts for all three variants;
  [PORT_STATUS.md](../PORT_STATUS.md) records them.
- The skeleton also passes Clippy for `x86_64-unknown-linux-gnu`, so the SSE module
  declarations compile.
- A cross-compile of the oracle for x86-64 on the same host reproduced the
  `-mno-sse2` build failure and confirmed that `-msse2` fixes it (ADR 0002).

The workflows have not run on GitHub; `act` is not installed, so they were checked
by review only.

## Not-applicable checks

- No translation, so no source-correspondence review and no Rust-versus-C comparison.
- `test_cwebp.sh` is not part of the WP0 baseline; WP13 runs it.

## Recorded deviations

See [Findings from WP0](../WORKPLAN.md#findings-from-wp0): the pinned test data
postdates v1.6.0 (baseline in two views); `test_lossless.sh` covers 33 vectors, not
46; the oracle adds `-msse2` on x86-64; the crate denies rather than forbids
`unsafe_code`; assignments the catalog left open; the `fuzz/` crate is deferred;
undeclared cross-package calls, with a proposal that the maintainer decides. WP0
also copied `AUTHORS`, which the copied notices reference, and the port plan.
