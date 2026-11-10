# Codec development in the shared workspace

Ferrixel, its encoder adapter, and the four codec ports share one Cargo workspace,
lockfile, development toolchain, and repository. [ADR-0026](adr/0026-codecs-in-one-repository-and-workspace.md)
owns the consolidation decision. Importing a port does not select it as a pipeline
backend or approve publication; [ADR-0025](adr/0025-codec-selection-reference-conformance-and-image-scope.md)
and the M1 acceptance gates still apply.

| Component | Package | Local source |
| --- | --- | --- |
| Engine | `ferrixel` | repository root |
| SIMD kernels | `ferrixel-kernels` | `crates/ferrixel-kernels` |
| AVIF encoder adapter | `ferrixel-libavif` | `crates/ferrixel-libavif` |
| JPEG | `libjpeg-turbo-rs` | `codecs/libjpeg-turbo-rs` |
| WebP | `libwebp-rs` | `codecs/libwebp-rs` |
| JPEG XL | `libjxl-rs` | `codecs/libjxl-rs` |
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
cargo fetch --locked
```

Fuzz targets and the independent image-quality comparator retain their existing
isolated test workspaces; all engine, adapter and codec packages share the root.

The root pins the nightly required by generated AVIF code. JPEG, WebP and JXL
retain stable Clippy, whose lint expectations differ from that older nightly;
their check scripts select it explicitly. MSRV jobs also select their toolchain
explicitly. This is a verifier exception, not another workspace or dependency graph.
The engine still builds on stable Rust 1.98 and the handwritten ports on Rust 1.95.

## Checks

Root default members are the engine and kernels. This keeps the ordinary
contributor loop bounded and preserves a stable-compatible engine. All ten
packages participate in `cargo check --workspace --all-targets` on the AVIF
port's supported native hosts (macOS ARM64 and GNU Linux x86-64).

```sh
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
python3 scripts/check-codecs.py --stage static
python3 scripts/check-codecs.py --codec webp --stage oracle
cargo test -p ferrixel-libavif --release --all-features
```

The shared runner preserves each component's oracle protocol and working
directory. `--stage test` runs the component's established Rust profiles; JXL's
tests always require its C++ oracle. `--stage oracle` requires the JPEG/WebP
references and executes the JXL differential suite. AVIF's local oracle tier
runs SharpYUV parity, the cross-port SharpYUV comparison, and AOM golden outputs;
the complete original AVIF/dav1d/AOM suites, sanitizer runs, and native platform
checks remain in the `avif-*` workflows. A bounded local run does not establish
their acceptance.

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
implementation; ADR-0026 supersedes only repository/workspace/tool placement.
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

The young source repositories need no permanent archive arrangement. Their
retirement follows successful migration validation and transfer of still-active
issue context. Deletion is a separate remote action; the import itself does not
delete repositories or close their pull requests.
