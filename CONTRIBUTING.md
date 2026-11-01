# Contributing

Use US English in all project artifacts and Conventional Commits for every
commit, for example `feat(bitstream): port header serialization` or
`fix(fields): preserve truncated-read status`. Keep algorithm improvements
separate from the mechanical port. Read [PORTING.md](docs/PORTING.md) and the
[living ADRs](docs/adr/README.md) before changing a translated boundary.
Create commits without signatures, using `git commit --no-gpg-sign`.

Repository Cargo configuration disables LLVM loop and SLP vectorization to
match the pinned scalar reference's Clang settings. Retain those flags when
overriding `RUSTFLAGS`; changing them requires renewed numeric parity evidence.

The oracle also fixes Quantizer constructor/recomputation calls with a scoped
`-fno-inline-functions` option on the original quantizer.cc. Rust keeps
RecomputeFromGlobalScale out of line. This prevents instrumentation-dependent
signaling-NaN differences from folding multiplication by one into a copy.
CMake 3.18 supplies the source-property directory scope used for this option.
See [ADR 0002](docs/adr/0002-differential-oracle.md).

## Prerequisites

- Rust 1.85 or later; `rust-toolchain.toml` selects stable with rustfmt and Clippy.
- Python 3, Git, CMake 3.18 or later, and a C++17 compiler for the oracle.
- Linux or macOS for the current oracle build configuration.

Initialize the reference source and its pinned Highway, Brotli, and skcms
dependencies. The oracle builds the complete original static codec library
through upstream CMake, including its default color management implementation:

```sh
git submodule update --init --depth 1 upstream/libjxl
git -C upstream/libjxl submodule update --init --depth 1 third_party/highway third_party/brotli third_party/skcms
```

After those downloads, the checks do not require network access. Keep the
reference checkouts unmodified. `upstream.json` and the Git submodule must agree;
upstream changes require a separate migration and renewed evidence.

## Checks

```sh
python3 tools/check.py
```

This checks the source pins, notices, and local documentation links, then runs
formatting, oracle compilation, Clippy, and all Rust and differential tests in
debug and release. It does not install or update toolchains or dependencies.

For an instrumented C++ reference and the minimum Rust version:

```sh
python3 tools/check.py --sanitizers --msrv
```

`--msrv` requires the Rust `1.85` toolchain to be installed already. Sanitizers
cover the oracle adapters, original codec library, and its built dependencies;
they do not instrument Rust or prove Rust memory safety. The current platform
evidence is listed in
[PORT_STATUS.md](docs/PORT_STATUS.md).

Focused checks:

```sh
cargo test --locked --offline --test bitstream
cargo test --locked --offline --test headers
cmake -S tools/oracle -B target/oracle -DCMAKE_BUILD_TYPE=Release
cmake --build target/oracle --target jxl_oracle --parallel 4
cargo test --locked --offline --features cpp-oracle --test parity -- --nocapture
```

Set `JXL_CPP_ORACLE` to an absolute path when using a separately built oracle.
The default is `target/oracle/jxl_oracle`. Oracle comparisons use the source's
release status behavior; debug-only abort and diagnostic instrumentation is
outside the current parity contract.

The GitHub Actions workflow schedules the same complete check on Linux and
macOS. A local result does not imply that those hosted jobs have run.

Keep [PORT_STATUS.md](docs/PORT_STATUS.md) current as units and evidence change.
Update an existing ADR in place when changing its decision and its `Updated`
date, instead of appending a chain of replacement records for the same topic.
