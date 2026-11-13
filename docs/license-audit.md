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
feature combination. Since 2026-11-12 `cli` includes `jpeg-xl-own-read` and
`jpeg-xl-write` (root `Cargo.toml`; `jpeg-xl-own-read` joined in `139a37d5`).
This tool selects the `ferrisma` package only; it does not inventory the separate
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

## v1.0 distribution inventory (2026-11-12)

Bundles were regenerated for each supported feature set with the tool above,
at the branch head `7ff04d68` on top of `90b638dd`, lockfile SHA-256
`2f082fabcb7b97e2172a821c549548b9b588f8824d53ccfa7d21d84707ab3086`, and
`cargo 1.100.0-nightly (3c0b53475 2026-09-04)`.

| Features (`--no-default-features`) | Target | Packages | Normal | Build-only | Remaining review items |
| --- | --- | ---: | ---: | ---: | --- |
| `cli` | `aarch64-apple-darwin` | 93 | 86 | 7 | `libjpeg-turbo-rs` (IJG), `simd_helpers` (notice) |
| `cli` | `x86_64-unknown-linux-gnu` | 93 | 86 | 7 | Same packages as macOS ARM64 |
| `cli,jpeg-xl-own-read` | `aarch64-apple-darwin` | 95 | 88 | 7 | `libjpeg-turbo-rs`, `simd_helpers` |
| `jpeg-read` | `aarch64-apple-darwin` | 3 | 3 | 0 | `libjpeg-turbo-rs` |
| `webp-read` | `aarch64-apple-darwin` | 4 | 4 | 0 | None in metadata |
| `webp-write` | `aarch64-apple-darwin` | 4 | 4 | 0 | None in metadata; ICC profile notice |
| `jpeg-xl-read` | `aarch64-apple-darwin` | 23 | 23 | 0 | None in metadata |
| `jpeg-xl-own-read` | `aarch64-apple-darwin` | 4 | 4 | 0 | None in metadata |
| `avif-read` | `aarch64-apple-darwin` | 15 | 15 | 0 | None in metadata |
| `avif-write` | `aarch64-apple-darwin` | 58 | 51 | 7 | `simd_helpers` |
| `avif-own-write` | `aarch64-apple-darwin` | 15 | 15 | 0 | None in metadata |
| `icc` | `aarch64-apple-darwin` | 6 | 5 | 1 | None in metadata |

Update after JPEG XL output joined `cli` (2026-11-12, ADR-0039, same tool,
uncommitted working tree on top of `32889ae5`, lockfile including the new
`ferrisma-jxl-encoder` package): `cli` on `aarch64-apple-darwin` collects 96
packages (89 normal, 7 build-only), with `ferrisma-jxl-encoder` (2 notice
files) and `libjxl-rs` (12 notice files) in the graph and the same two
remaining review items; `jpeg-xl-write` alone collects 4 packages
(`ferrisma`, `ferrisma-kernels`, `ferrisma-jxl-encoder`, `libjxl-rs`). The
x86-64 `cli` bundle and the other rows were not regenerated. The encoder is
now reachable from the engine, so the libjxl-rs encoder notice review in
ADR-0039's license gate applies to every `cli` distribution.

"None in metadata" means that every package passes the SPDX screen and
supplied at least one notice file. It is not a completed legal review.

> **Note (2026-11-13): feature set changed after this inventory.** The table
> above was generated on top of `90b638dd`, where `cli` did not yet include
> `jpeg-xl-own-read`; that is why it lists `cli` (93 packages) and
> `cli,jpeg-xl-own-read` (95) separately. Since `139a37d5` (2026-11-12) the
> root `Cargo.toml` `cli` feature includes `jpeg-xl-own-read`, and since
> ADR-0039 also `jpeg-xl-write`. Only the macOS ARM64 `cli` bundle was
> regenerated after that (the update paragraph above, 96 packages). The
> `cli` rows of the table and the x86-64 `cli` row do not describe today's
> `cli` and need a re-audit before any distribution
> ([#66](https://github.com/sebastian-software/ferrisma/issues/66)). No
> license conclusion is changed by this note.

Workspace packages in the `cli` graph and their declared terms:

| Package | Expression | Notice files bundled | Source of the terms |
| --- | --- | ---: | --- |
| `ferrisma` | MIT OR Apache-2.0 | 6 | ADR-0010; plus `src/av1_tuning.rs` (BSD-3-Clause, ravif) and the ICC profile |
| `ferrisma-kernels`, `ferrisma-avif-read`, `ferrisma-avif-write` | MIT OR Apache-2.0 | 2 each | Project-owned |
| `libjpeg-turbo-rs` | IJG AND Zlib | 4 | `README.ijg`, `LICENSE.md`, retained file headers |
| `libwebp-rs` | BSD-3-Clause | 3 | `LICENSE`, `PATENTS`, `AUTHORS` |
| `libsharpyuv-core` | BSD-3-Clause | 6 | `LICENSES/` (libwebp `COPYING`, `PATENTS`, `AUTHORS`) |
| `libavif-rs` | BSD-2-Clause AND BSD-3-Clause | 2 | `LICENSE` (libavif, with its third-party sections) |
| `libavif-aom` | BSD-2-Clause AND BSD-3-Clause AND MIT | 11 | `LICENSES/` (libaom `LICENSE`, `PATENTS`, `AUTHORS`, fastfeat, vector) |
| `libavif-dav1d` | BSD-2-Clause | 5 | `LICENSES/` (`COPYING`, `doc/PATENTS`) |
| `libavif-sharpyuv` | BSD-3-Clause | 7 | `LICENSES/` (libwebp `COPYING`, `PATENTS`, `AUTHORS`) |
| `ferrisma-jxl-decoder` (`jpeg-xl-own-read`, in `cli` since 2026-11-12) | MIT OR Apache-2.0 | 2 | Project-owned |
| `ferrisma-jxl-encoder` (`jpeg-xl-write`, in `cli` since 2026-11-12) | MIT OR Apache-2.0 | 2 | Project-owned |
| `libjxl-rs` (`jpeg-xl-own-read` and `jpeg-xl-write`, both in `cli`) | BSD-3-Clause AND MIT AND (Apache-2.0 WITH LLVM-exception) | 7 | `LICENSE`, `PATENTS`, `licenses/` (Brotli MIT, Highway BSD, skcms BSD, libc++ Apache-2.0 with LLVM exception) |

### Findings from the 2026-11-12 review

1. **`libavif-rs` declared too little.** `src/libyuv` contains translated
   libyuv scaling and planar functions (Copyright The LibYuv Project
   Authors, BSD-3-Clause). `src/modules.rs` compiles them whenever the native
   `libyuv` feature is off, which is the engine's configuration. The package
   expression is now `BSD-2-Clause AND BSD-3-Clause`; its `LICENSE` already
   contains the libyuv text. The translated headers mention a libyuv
   `PATENTS` file that the pinned libavif snapshot does not contain; it is
   not invented here. Its `src/obu.rs` derives from dav1d's BSD-2-Clause
   `obu.c`, which the same `LICENSE` covers.
2. **The JPEG source-bound evidence no longer applies.** The reviewed
   `libjpeg-turbo-rs` tree hash `f5ba795a…` differs from the hash at
   `90b638dd` (`3531c839ba87e6acb563b5fe04644d88aa55332343196a9b88d455f78a01ea59`),
   which predates this branch; this branch does not modify the port. The
   generator therefore no longer links the JPEG evidence. The pin was not
   refreshed: the changed files need review before the evidence is re-bound.
3. **`simd_helpers` 0.1.0** still has no copyright or permission notice in its
   published package or upstream revision. It is a proc-macro of `rav1e` and
   reaches `avif-write` and `cli`. Either obtain a notice from its author,
   confirm it is absent from the linked binary and document that, or keep the
   item open. Do not invent a notice.
4. **The ICC profile** is now a package-local copy (`assets/icc`), so the
   published `ferrisma` crate contains the file it embeds. The bundle collects
   the copy and its provenance file instead of the libavif test-data paths.
5. **Native, assembly, and unsafe boundaries** are reviewed in the separate
   [unsafe review](unsafe-review.md). It also records the consumer profile
   requirements that the AVIF packages need.
6. **Patents.** `libwebp-rs`, `libsharpyuv-core`, `libavif-sharpyuv`
   (Google), `libjxl-rs` (Google, JPEG XL), and `libavif-aom` and
   `libavif-dav1d` (AOM) ship separate patent grants. The bundles retain
   them. Whether those grants cover a given distribution was not assessed.

### Policy for project-owned code and per-crate metadata (#136)

Proposed on 2026-11-12 for maintainer confirmation; it applies the existing
ADR-0010 and the codec ADRs 0014 and 0017 of libavif-rs to every workspace
package:

- **Standalone project-owned packages** (`ferrisma`, `ferrisma-kernels`,
  `ferrisma-avif-read`, `ferrisma-avif-write`, `ferrisma-jxl-decoder`,
  `ferrisma-libavif`, `ferrisma-xtask`) are `MIT OR Apache-2.0` (ADR-0010) and
  carry both license texts in the package. The bridge crates use them as
  links to the root files.
- **Original additions inside a port package** (safe API modules, dispatch,
  adapters, tests) are contributed under that port's upstream terms, so a
  port package has one expression that describes all of its files. The
  JPEG, WebP, JPEG XL, and AVIF ports already follow this; files without an
  upstream header in those packages are such additions.
- **Every package's expression lists all terms of the files it packages**,
  joined with `AND`, and uses `OR` only for an actual choice. Packages retain
  their upstream license, notice, and patent files in their published
  contents (enforced with `include` lists).
- **Code copied into a project-owned package** keeps its own header and is
  listed in `THIRD-PARTY-NOTICES.md` (currently `src/av1_tuning.rs` and the
  ICC profile).

Applied on this branch: `libavif-rs` expression corrected; license texts
added to the project-owned bridge packages and `ferrisma-kernels`; `license`,
`description`, and `repository` completed where they were missing; `include`
lists added to every package in the publication chain except the JPEG port.
The JPEG port's manifest is unchanged here because it must be renamed before
publication (see [publishing](publishing.md)); its `include` list belongs
in that change.

## Current direct codec/CMS choices

| Dependency | Cargo feature | Upstream metadata/license | Review focus |
| --- | --- | --- | --- |
| workspace `libjpeg-turbo-rs` | `jpeg-read` | IJG and Zlib; pinned port of libjpeg-turbo 3.2.1 | Retain [IJG license](../codecs/libjpeg-turbo-rs/README.ijg), [Zlib notice](../codecs/libjpeg-turbo-rs/LICENSE.md), upstream notices, safe API boundary, ICC/EXIF handling, and codec scratch |
| workspace `libwebp-rs` | `webp-read`, `webp-write` | BSD-3-Clause; [license](../codecs/libwebp-rs/LICENSE), [patent grant](../codecs/libwebp-rs/PATENTS), [authors](../codecs/libwebp-rs/AUTHORS), [provenance](../codecs/libwebp-rs/docs/PROVENANCE.md) | Still decoder/encoder, ICC/EXIF input, opaque scratch and notices; publication gate remains open |
| embedded [sRGB2014.icc](../assets/icc/sRGB2014.icc) | `webp-write` | International Color Consortium, 2015; [package-local copy and provenance](../assets/icc/README.md), [upstream provenance](../codecs/libavif-rs/upstream/libavif/tests/data/README.md), [profile license](https://registry.color.org/profile-library/#licensing) | Unchanged profile embedded in WebP output to describe emitted sRGB samples; SHA-256 `384b832de3412066743b52a75ee906b6fb9fb8d9e09e936fc2c43223815c6e0a`; separate from libwebp's BSD license |
| workspace `libjxl-rs` through `ferrisma-jxl-decoder` | `jpeg-xl-own-read` | BSD-3-Clause with retained MIT and Apache-2.0 with LLVM exception notices in the port | Raw decoder safety boundary, DATA ICC, alpha, full-frame output and opaque scratch; external conformance remains open |
| workspace `libjxl-rs` encoder sources (`src/jxl/enc_*.rs`, `encode*.rs`, `modular/encoding/enc_*.rs`, `jpeg/enc_jpeg_data.rs`) | `jpeg-xl-write` (in `cli` since 2026-11-12): lossless entry point and, through `ferrisma-jxl-encoder`, the public lossy encoder (#27, #92) | Same package and expression; every encoder file reviewed on 2026-11-12 carries the JPEG XL Project Authors BSD-3-Clause header, and the package retains [LICENSE](../codecs/libjxl-rs/LICENSE) and [PATENTS](../codecs/libjxl-rs/PATENTS) | Reachable from the engine; the `cli` macOS ARM64 bundle was regenerated (above); the PATENTS scope question and the encoder's internal unsafe review ([unsafe review](unsafe-review.md)) remain open |
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

- [ ] Store an externally reviewable license inventory for an exact, reproducible version set (including NOTICE/license files). The 2026-11-12 bundles above are reproducible; storing the bundle of the exact release commit remains.
- [ ] Review native code, assembler and unsafe boundaries in *all* dependencies, not only Ferrisma's `#![forbid(unsafe_code)]` core. The [unsafe review](unsafe-review.md) screens the full graph and describes every project-owned boundary; third-party code was not audited.
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
