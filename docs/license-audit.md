# Dependency licensing and release gates

Ferrisma itself is **MIT OR Apache-2.0** (see [ADR-0010](adr/0010-dual-licensing.md)). That license does **not** relicense embedded or transitive codec, compression, image, parser, assembler or CMS implementations.

## Automated metadata screening

```sh
python3 scripts/test_audit_licenses.py
python3 scripts/audit-licenses.py
```

The script invokes `cargo metadata --all-features`, traverses the resolved dependency graph and writes `target/m1-license-inventory.csv`. Ordinary CI **blocks strong-copyleft metadata** and reports missing/unknown/weak-copyleft terms as `REVIEW` without pretending they are approved. Historical Cargo slash alternatives such as `MIT/Apache-2.0` are interpreted as `MIT OR Apache-2.0` by the metadata screener; the original string remains in the inventory. A strict pre-release run (`python3 scripts/audit-licenses.py --strict`) fails if *any* review item remains. This is a **metadata preflight, not a lawyer-reviewed dependency license audit**. `SPDX OR` grants a choice; metadata does not demonstrate how that choice must be documented or distributed.

When a crate is flagged, review its exact source license, license-file and notices, selected feature/dependency tree, patent terms, and bundled native code before allowing it. Do not expand the allowlist just to make CI green.

## Target-specific distribution evidence

For each intended distribution, generate a new bundle using its exact Rust
target and feature set; this example describes the CLI on macOS ARM64:

```sh
python3 scripts/test_production_license_bundle.py
python3 scripts/production-license-bundle.py --target aarch64-apple-darwin --features cli --output target/license-bundle-macos-arm64
python3 scripts/production-license-bundle.py --target aarch64-apple-darwin --features cli --output target/license-bundle-macos-arm64 --verify
```

The output directory must not already exist. `--strict` makes generation fail
when SPDX metadata needs review or no notice file was found; it still writes the
bundle for inspection. `--verify` reconstructs the graph and checks identities
and bundled file hashes; combine it with `--strict` to fail on remaining review
items. Verification rejects symlinks in referenced bundle files. Passing
verification establishes reproducibility, not license compliance. Ship the reviewed notices with the matching artifact only
after the source and legal review below; include [specific embedded-material
notices](../THIRD-PARTY-NOTICES.md). Generate separately for every target and
feature combination: `cli` does not include the opt-in `jpeg-xl-own-read`. This
tool selects the `ferrisma` package only; it does not inventory the separate
`ferrisma-libavif` compatibility adapter or its CLI as a root package. Audit
those artifacts separately before distribution.

The ordinary engine CI runs the bundle's contract tests alongside the existing
license metadata tests. These tests check source identity, recovered-file
provenance, tampering, symlink rejection, and strict verification failures;
they do not approve a distribution or remove the independent review gate.

The JSON inventory records the Cargo version, target, requested features,
lockfile hash, checkout revision and status, exact package versions and source
identity. `cargo-tree-features.txt` records the resolved features on the selected
normal and build graph. Registry hashes are lockfile package-archive checksums, not hashes of
the extracted local source; workspace hashes cover tracked file contents and
Git submodule identities, with untracked paths disclosed in Git status. Check
the source checkout and registry cache independently before distribution.
Normal edges may include host-executed proc macros; build-only entries represent
additional build edges. Cargo dependency reachability does not prove that code
is present in the linked binary. Development dependencies, separate benchmark
projects, system libraries, and other codec workspace packages are outside this
specific graph. Bundled notice discovery is deliberately conservative and cannot
find every license header or embedded native component.

On 2026-11-11, the `cli` graph for `aarch64-apple-darwin` contained 93
packages: 86 reachable by normal edges and seven additional build-only
packages. The `libavif-dav1d` package remains a normal dependency of the
workspace libavif port even when the AOM decoder is selected; package presence
alone does not establish its linked code. The captured JPEG manifest states
`IJG AND Zlib`, and the port's source retains both IJG and zlib material. IJG
remains `REVIEW` in the metadata gate. The registry packages `alloc-stdlib`
0.2.4, `c2rust-bitfields` 0.3.0, `c2rust-bitfields-derive` 0.2.1,
`profiling` 1.0.18, `profiling-procmacros` 1.0.18, and `simd_helpers` 0.1.0
had no separately named license/notice file in their cached crate sources.
For the first five, [recovered upstream files and provenance](../licenses/recovered/manifest.json)
record exact source commits, Git blob IDs, file hashes, and package-archive
checksums. The generator checks all three identities before using these files.
The c2rust derive revision's root license also names an unrelated `c2rust-ast-printer`
component; do not infer that component is in this graph. No separate license
file appears in the [exact simd_helpers upstream revision](https://github.com/lu-zero/simd_helpers/tree/ca1a2f84aa386d758e98f8a609d990263932fb85).
Its manifest names an author and declares MIT, but the copyright and permission
notice remain unresolved; do not invent them. It is a proc-macro package, and
its graph presence does not establish inclusion in the final binary. Independent
source and legal review remains open for all entries.

### JPEG source-bound evidence

The `libjpeg-turbo-rs` 0.1.0 workspace package declares `IJG AND Zlib`.
The tracked source hash calculated by the bundle generator is
`f5ba795a3d447b0adcaeef7e239cf45fb5495665529c5f11c9d7de599c0afca2`
for the version reviewed on 2026-11-11; its [import record](../codecs/libjpeg-turbo-rs/IMPORT.json)
identifies the port revision. [README.ijg](../codecs/libjpeg-turbo-rs/README.ijg)
contains the IJG source redistribution, executable documentation, warranty,
liability, and publicity conditions. Upstream [LICENSE.md](../codecs/libjpeg-turbo-rs/LICENSE.md)
identifies IJG terms for the libjpeg API and zlib terms for SIMD. For example,
[jdcolext_neon.rs](../codecs/libjpeg-turbo-rs/src/jdcolext_neon.rs) preserves
the original SIMD copyright and zlib text and identifies the translation.
The binary documentation wording is included in [THIRD-PARTY-NOTICES.md](../THIRD-PARTY-NOTICES.md).
Ship the retained README and source notices with source distributions, and
record modifications as the original terms require.

This evidence supports retaining the exact port and both license components
in a proposed notice bundle; it does not establish a general `IJG` allowlist,
replace a per-distribution audit, or resolve patent questions. The tool links
this evidence only when package, version, expression, and tracked source hash
all match. IJG continues to be `REVIEW`, including in strict mode, until the
remaining independent review is completed.

## Current direct codec/CMS choices

| Dependency | Cargo feature | Upstream metadata/license | Review focus |
| --- | --- | --- | --- |
| workspace `libjpeg-turbo-rs` | `jpeg-read` | IJG and Zlib; pinned port of libjpeg-turbo 3.2.1 | Retain [IJG license](../codecs/libjpeg-turbo-rs/README.ijg), [Zlib notice](../codecs/libjpeg-turbo-rs/LICENSE.md), upstream notices, safe API boundary, ICC/EXIF handling, and codec scratch |
| workspace `libwebp-rs` | `webp-read`, `webp-write` | BSD-3-Clause; [license](../codecs/libwebp-rs/LICENSE), [patent grant](../codecs/libwebp-rs/PATENTS), [authors](../codecs/libwebp-rs/AUTHORS), [provenance](../codecs/libwebp-rs/docs/PROVENANCE.md) | Still decoder/encoder, ICC/EXIF input, opaque scratch and notices; publication gate remains open |
| embedded [sRGB2014.icc](../codecs/libavif-rs/upstream/libavif/tests/data/sRGB2014.icc) | `webp-write` | International Color Consortium, 2015; [upstream provenance](../codecs/libavif-rs/upstream/libavif/tests/data/README.md), [profile license](https://registry.color.org/profile-library/#licensing) | Unchanged profile embedded in WebP output to describe emitted sRGB samples; SHA-256 `384b832de3412066743b52a75ee906b6fb9fb8d9e09e936fc2c43223815c6e0a`; separate from libwebp's BSD license |
| workspace `libjxl-rs` through `ferrisma-jxl-decoder` | `jpeg-xl-own-read` | BSD-3-Clause with retained MIT and Apache-2.0 with LLVM exception notices in the port | Raw decoder safety boundary, DATA ICC, alpha, full-frame output and opaque scratch; external conformance remains open |
| `jxl-oxide` | `jpeg-xl-read`, default JPEG XL backend | MIT OR Apache-2.0 | Runtime ROI request; retained for measured full/crop comparison |
| workspace `libavif-rs` / `libavif-aom` through `ferrisma-avif-read` | `avif-read` | libavif BSD-2-Clause; AOM port retains BSD-2-Clause, BSD-3-Clause, MIT notices and libaom's patent grant | Isolated unsafe boundary, generated-source provenance, AOM reference decode, normal dependency on the dav1d port and its retained notices, malformed bitstreams, native/nightly target matrix, opaque codec scratch and full Argon/dav1d gate |
| `rav1e` / `avif-serialize` | `avif-write` | BSD-2-Clause encoder, BSD-3-Clause container writer; `rav1e` links hand-written assembly | Encoded CICP, alpha, frame materialization and opaque scratch |
| `ravif` | dev-dependency only | BSD-3-Clause | Writes one test fixture; not linked into the library or CLI |
| `moxcms` | `icc` | BSD-3-Clause OR Apache-2.0 | ICC parsing, color fidelity, malformed profiles, SIMD/native code |

The standalone `zune-jpeg` benchmark harness remains under `scripts/bench/`
for comparison; it is not a `jpeg-read` runtime dependency. The workspace
JPEG port retains its pinned upstream notices and its own scoped license
terms; the engine's MIT OR Apache-2.0 license does not relicense that port.

**Embedded third-party code.** `src/av1_tuning.rs` adapts `ravif` 0.13.0's quality-to-quantizer curve and speed overrides (BSD-3-Clause, Copyright (c) 2020, Kornel). The file carries the full license notice; binary distributions must reproduce it with the other third-party notices.

The choices and their remaining gates are recorded in [ADR-0018](adr/0018-optional-codec-and-cms-dependencies.md). A future JPEG XL encoder, including the AGPL-3.0/commercial `jxl-encoder` candidate, is **not** an approved default dependency and requires a separate ADR.

## M1 release prerequisites

- [ ] Store an externally reviewable license inventory for an exact, reproducible version set (including NOTICE/license files).
- [ ] Review native code, assembler and unsafe boundaries in *all* dependencies, not only Ferrisma's `#![forbid(unsafe_code)]` core.
- [x] Commit `Cargo.lock` for the CLI and reproducible measurements ([ADR-0018](adr/0018-optional-codec-and-cms-dependencies.md)).
- [ ] Check that the package advertises its required Rust version accurately and that the target platforms actually compile.
- [ ] Record real AVIF output ICC/CICP and alpha behavior, HDR rejection, and total RSS on representative images.
- [ ] Perform an independent source and legal review of third-party license obligations.

Do **not** declare M1 release-ready merely because the metadata gate passes.

## Historical transitive review: `avif-parse` (MPL-2.0)

The former `avif-decode` input path incorporated `avif-parse` version 2.1.0,
which declares **MPL-2.0** (confirmed in its upstream `Cargo.toml` and source
headers). MPL-2.0 is file-level copyleft and was a distribution review item for
that historical graph. `avif-read` now uses the workspace libavif/AOM port and
an independent bounded container-property parser; neither `avif-decode` nor
`avif-parse` is a runtime dependency of the current engine. The old MPL item
does not clear the separate, still-open full-dependency license and notice
review for the current graph. Preserve its history when comparing earlier
releases or benchmark binaries.

Reference: https://github.com/kornelski/avif-parse/blob/main/Cargo.toml
