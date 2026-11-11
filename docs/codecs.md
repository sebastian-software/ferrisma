# Codec development in the shared workspace

Ferrixel, its encoder adapter, and the four codec ports share one Cargo workspace,
lockfile, development toolchain, and repository. [ADR-0026](adr/0026-codecs-in-one-repository-and-workspace.md)
owns the consolidation decision. Importing a port does not select it as a pipeline
backend or approve publication; [ADR-0025](adr/0025-codec-selection-reference-conformance-and-image-scope.md)
and the M1 acceptance gates still apply. For the current pipeline, see [status](status.md); for navigation, see the [documentation guide](README.md).

| Component | Package | Local source |
| --- | --- | --- |
| Engine | `ferrixel` | repository root |
| SIMD kernels | `ferrixel-kernels` | `crates/ferrixel-kernels` |
| AVIF encoder adapter | `ferrixel-libavif` | `crates/ferrixel-libavif` |
| JPEG | `libjpeg-turbo-rs` | `codecs/libjpeg-turbo-rs` |
| WebP | `libwebp-rs` | `codecs/libwebp-rs` |
| JPEG XL | `libjxl-rs` | `codecs/libjxl-rs` |
| Tooling | `ferrixel-xtask` (invoked as `cargo xtask`) | `tools/xtask` |
| AVIF, AOM, dav1d, SharpYUV | `libavif-rs`, `libavif-aom`, `libavif-dav1d`, `libavif-sharpyuv` | `codecs/libavif-rs` |

## Setup

Run from the repository root. No sibling checkout is required. The AVIF upstream
snapshots are tracked files; the other references retain their pinned gitlinks.

```sh
git submodule update --init codecs/libjpeg-turbo-rs/upstream/libjpeg-turbo \
  codecs/libwebp-rs/upstream/libwebp codecs/libwebp-rs/upstream/libwebp-test-data \
  codecs/libjxl-rs/upstream/libjxl
git -C codecs/libjxl-rs/upstream/libjxl submodule update --init \
  third_party/highway third_party/brotli third_party/skcms
rustup show active-toolchain
rustup toolchain install stable --profile minimal --component clippy
rustup toolchain install 1.95 --profile minimal
cargo fetch --locked
```

Fuzz targets and the independent image-quality comparator retain their existing
isolated test workspaces; all engine, adapter and codec packages share the root.

The root pins the nightly required by generated AVIF code. JPEG, WebP and JXL
retain stable Clippy, whose lint expectations differ from that older nightly;
their check scripts select it explicitly. MSRV jobs also select their toolchain
explicitly. This is a verifier exception, not another workspace or dependency graph.
The engine still builds on stable Rust 1.98 and the handwritten ports on Rust 1.95.

Engine Clippy selects `ferrixel` and `ferrixel-kernels` with `--no-deps` when
codec features are enabled. Once the ports are local dependencies, this prevents
the pinned AVIF nightly from also linting their mechanically translated loops.
The component checks below still lint each port strictly with its supported
stable Clippy. See [contributing](../CONTRIBUTING.md) for the full command.

## Checks

Root default members are the engine and kernels. This keeps the ordinary
contributor loop bounded and preserves a stable-compatible engine. All 11 members (ten engine, adapter, and codec packages plus `xtask`) participate
in `cargo check --workspace --all-targets` on the AVIF port's supported native
hosts (macOS ARM64 and GNU Linux x86-64).

```sh
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo xtask codecs --codec webp --stage pr
# Deep local differential checks, when needed:
cargo xtask codecs --codec webp --stage oracle
cargo test -p ferrixel-libavif --release --all-features
```

The `pr` stage is the contributor fast loop for one codec. On JPEG, WebP,
and JXL it initializes no references itself: the setup above supplies pinned
upstream sources for the existing static provenance and MSRV checks, then Rust
runs checked tests against retained fixtures. These checks do not build a live
C/C++ oracle. AVIF also checks its tracked provenance, retained AOM goldens,
and SharpYUV cross-port cases. The shared runner preserves component oracle
protocols and working directories.
[ADR-0027](adr/0027-tiered-validation-and-rust-test-orchestration.md) defines
scoped PR checks, retained reference fixtures, and nightly or manual deep runs.
Live differential checks require their C/C++ references and fail if missing.
A bounded local run does not establish full codec or platform acceptance.
Rust `xtask` is the shared control plane; TypeScript may be used where warranted.
Existing Python oracle harnesses remain until their incremental replacements
preserve and compare protocol, fixtures, provenance, and error behavior. The
central nightly runs deep checks; manual component runs and the exhaustive AOM
original-consumer, recovery, and translation workflows supply additional evidence.
Changes to oracle pins, goldens, unsafe ABI, SIMD, or floating-point semantics
need the relevant deep branch run or equivalent recorded native evidence.

Do not use workspace-wide `--all-features`: the interim `avif-read` backend and
the libavif adapter export conflicting `dav1d_*` symbols. Engine all-feature
checks explicitly select `-p ferrixel -p ferrixel-kernels`; the adapter is tested
separately. JXL's local Cargo configuration disables auto-vectorization for its
reference-sensitive checks. Run those checks through its script or from its
directory. Rust artifacts live in the root `target/`; C/C++ reference outputs
stay local to each codec. Generated AVIF source is excluded from rustfmt because
its translation manifests verify source hashes.

## Ownership, provenance, and releases

Each codec's `IMPORT.json` identifies its main revision, integrated revision,
source tree, and included pull requests. Original commits and authors are kept
in Git history. Codec-local ADRs and porting contracts continue to govern their
implementation; ADR-0026 supersedes repository/workspace/tool placement; ADR-0027 supersedes the specified validation cadence.
The upstream revision and golden expectations do not change during relocation.

To review migration edits separately from the imported source, compare the
integrated tree recorded in a component's `IMPORT.json` with its current subtree:

```sh
git diff <integrated-revision>^{tree} HEAD:codecs/<component>
```

This comparison includes relocated workflow files as removals; their replacements
live under the root `.github/workflows/` with codec prefixes.

Package versions, licenses, and publication gates remain independent. Internal
production dependencies use `path` and a compatible `version` where needed for
eventual crates.io publication. Every imported package remains `publish = false`.
Shared development does not grant publication authority or relicense imported
source. The root license screen still covers the engine's reachable dependency
graph; codec notice and provenance checks remain separate.

The four original codec repositories have been retired and their issues migrated.
Development and issue tracking continue in Ferrixel; retained import metadata and
Git history document the source revisions.
