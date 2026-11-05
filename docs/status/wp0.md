# WP0: bootstrap

- Status: done on 5 November 2026, revised on 6 November 2026 after the
  maintainer's decisions on findings 1, 6 and 7 (local host); CI not run yet,
  because the repository has no remote
- Sub-wave: 0
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
  `upstream/libwebp-test-data` (`260804d686f0491e0326a50eef491829dae09e9e`), pinned in
  `upstream.json` and checked by `tools/verify_upstream.py`.
- The module skeleton: 126 translation modules covering all 151 portable and SIMD
  upstream `.c` and `.h` files under `src/` and `sharpyuv/`, five C ABI modules and
  nine `mod.rs` files, each module with its provenance header only. The 25 MIPS and
  MSA files are excluded. `tools/units.json` maps every file to its package and
  records sub-waves, dependencies, eight function- and type-level exceptions and 16
  reviewed header includes; `tools/units.py` checks the headers, which name the
  exceptions, and the declarations.
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

On the macOS ARM64 host described in [PORT_STATUS.md](../PORT_STATUS.md), on
6 November 2026 after the revision:

- `cargo build` and `python3 tools/check.py --oracle --msrv` pass: formatting,
  Clippy with `-D warnings` on all targets and features, the upstream pins, the
  module skeleton, documentation links, the dependency check against the oracle
  (`tools/unit_dependencies.py --check`: no undeclared reference), the tests in
  the dev, release and `checked` profiles with the release oracle required, and
  build and tests on Rust 1.95.
- `tests/wp0_features.rs` passes against the `release`, `asan-ubsan` and `debug`
  oracle variants: the probe reports status 0 and positive dimensions for all 131
  vectors (88 lossy, 43 lossless), matches the hand-checked expectation for
  `alpha_color_cache.webp` (588 by 97, alpha, not animated, lossy) from a path and
  from standard input, and keeps the reset fields with status 7 for a 20-byte prefix.
- `tools/oracle_baseline.py` passes with identical counts for all three variants on
  the pinned test data: 480 of 480 checksums (960 verifications) and 33 lossless
  vectors (330 comparisons). `--diagnose-post-v1.6.0` reproduces the seven
  differing PGM checksums against test-data `06ddd96`.
- The skeleton also passes Clippy for `x86_64-unknown-linux-gnu`, so the SSE module
  declarations compile.
- A cross-compile of the oracle for x86-64 on the same host reproduced the
  `-mno-sse2` build failure and confirmed that `-msse2` fixes it (ADR 0002).

The workflows have not run on GitHub; `act` is not installed, so they were checked
by review only. The Linux x86-64 oracle job will be the first native evidence for
the `-msse2` fix.

## Not-applicable checks

- No translation, so no source-correspondence review and no Rust-versus-C comparison.
- `test_cwebp.sh` is not part of the WP0 baseline; WP13 runs it.

## Recorded deviations

See [Findings from WP0](../WORKPLAN.md#findings-from-wp0). WP0 itself: `test_lossless.sh`
covers 33 vectors, not 46; the oracle adds `-msse2` on x86-64; the crate denies rather
than forbids `unsafe_code`; assignments the catalog left open; `AUTHORS` and the port
plan are copied as well. Decided by the maintainer after the first report and applied
on 6 November 2026: the test data is pinned at `260804d` instead of `06ddd96`; WP12
creates the `fuzz/` crate; the packages run in call-graph sub-waves with
function- and type-level exceptions. Applying that order required four corrections
found in the types: `alpha_dec.c` and `io_dec.c` go to WP6 rather than `alpha_dec.c`
to WP7, `alpha_enc.c` goes to WP10, three types become exceptions, and two crop
checks and three statistics helpers join the function exceptions they serve.
