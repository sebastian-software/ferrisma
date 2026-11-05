# Contributing

Use US English in all project artifacts and Conventional Commits for every
commit, for example `feat(utils): port the VP8 bit reader` or
`fix(dec): keep the first decoding error`. Create commits without signatures,
using `git commit --no-gpg-sign`. Keep algorithm changes out of the mechanical
port. Read [PORTING.md](docs/PORTING.md), [WORKPLAN.md](docs/WORKPLAN.md) and
the [living ADRs](docs/adr/README.md) before changing a translated boundary.

## Prerequisites

- Rust 1.95 or later. `rust-toolchain.toml` selects stable with rustfmt and
  Clippy; the `--msrv` check also needs the 1.95 toolchain
  (`rustup toolchain install 1.95 --profile minimal`).
- Python 3.9 or later, Git, CMake 3.18 or later and a C compiler (Clang on
  Linux and macOS) for the oracle.
- An `md5sum` that accepts `-c -` for the baseline: GNU coreutils on Linux,
  `/sbin/md5sum` on macOS 14 or later, or `gmd5sum` from Homebrew coreutils.

Initialize both submodules. A full clone also fetches the `v1.6.0` tag, which
the provenance check compares with the pinned commit, and the newer test-data
revision that `tools/oracle_baseline.py --diagnose-post-v1.6.0` reads:

```sh
git submodule update --init
```

Keep the checkouts unmodified. `upstream.json` and the submodules must agree;
an upstream change is a separate migration with renewed evidence.

## Checks

```sh
python3 tools/check.py
```

This verifies the upstream pins and license copies, the module skeleton and the
local documentation links, then runs `cargo fmt --all --check`,
`cargo clippy --all-targets --all-features -- -D warnings` and `cargo test` in the
dev, release and `checked` profiles. The `checked` profile inherits release
optimizations and keeps overflow checks and debug assertions enabled, because
the port mirrors C unsigned arithmetic with explicit wrapping operations and an
omitted one only fails with overflow checks on. Without an oracle, differential
tests print why they are skipped.

```sh
python3 tools/check.py --oracle
```

builds the `release` oracle (`tools/build_oracle.py`), points
`LIBWEBP_ORACLE_DIR` at it and runs the tests with the `oracle` feature, so a
test that cannot reach the oracle fails instead of skipping. Further flags:

- `--oracle-variant asan-ubsan` (or `debug`) runs the differential tests against
  that variant instead; `--baseline` also runs `tools/oracle_baseline.py`.
- `--msrv` builds and tests with the toolchain named by `rust-version`.
- `--stage static` runs only the checks that need no C compiler.

Focused commands:

```sh
python3 tools/build_oracle.py --variant release --variant asan-ubsan
LIBWEBP_ORACLE_DIR=build/oracle/release cargo test --features oracle --test wp0_features
python3 tools/oracle_baseline.py --variant release
python3 tools/units.py
python3 tools/rust_name.py VP8GetHeaders
python3 tools/unit_dependencies.py
```

## Writing a differential test

1. Add a probe `tools/oracle/probes/<wp-id>/<name>.c`. It includes
   `probe_support.h` and the upstream headers it needs (internal ones as
   `"src/..."`), calls the pinned library and prints `key=value` lines: integers in
   decimal, byte strings in lowercase hex (`ProbePrintHex`). Shared C helpers for
   one package go into `tools/oracle/probes/<wp-id>/support/`.
2. Rebuild the oracle; every `.c` file becomes
   `build/oracle/<variant>/probes/<wp-id>/<name>`.
3. Add `tests/<wp-id>_<topic>.rs` with `#[path = "support/oracle.rs"] mod oracle;`.
   Call `Oracle::locate`, run the probe with `probe.run(&args, &stdin)`, parse the
   output with `fields()` and compare it with a `Fields` value built from the Rust
   translation's results. `tests/wp0_features.rs` is the worked example.

## CI

`.github/workflows/check.yml` runs on pull requests, on pushes to `main`, nightly
and on manual dispatch:

- static checks and tests on Linux x86-64 and macOS ARM64;
- the MSRV build and tests on Rust 1.95;
- the oracle on both systems: the `release`, `asan-ubsan` and `debug` variants,
  the differential tests against `release` and `asan-ubsan`, and the baseline
  for each variant.

The `Checks passed` job is the single required status check. A local result does
not imply that the hosted jobs have run.

Keep [PORT_STATUS.md](docs/PORT_STATUS.md) for the integrator; packages write
their own `docs/status/<wp-id>.md`.
