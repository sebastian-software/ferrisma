# Dependency licensing and release gates

Ferrixel itself is **MIT OR Apache-2.0** (see [ADR-0010](adr/0010-dual-licensing.md)). That license does **not** relicense embedded or transitive codec, compression, image, parser, assembler or CMS implementations.

## Automated metadata screening

```sh
python3 scripts/test_audit_licenses.py
python3 scripts/audit-licenses.py
```

The script invokes `cargo metadata --all-features`, traverses the resolved dependency graph and writes `target/m1-license-inventory.csv`. Ordinary CI **blocks strong-copyleft metadata** and reports missing/unknown/weak-copyleft terms as `REVIEW` without pretending they are approved. Historical Cargo slash alternatives such as `MIT/Apache-2.0` are interpreted as `MIT OR Apache-2.0` by the metadata screener; the original string remains in the inventory. A strict pre-release run (`python3 scripts/audit-licenses.py --strict`) fails if *any* review item remains. This is a **metadata preflight, not a lawyer-reviewed dependency license audit**. `SPDX OR` grants a choice; metadata does not demonstrate how that choice must be documented or distributed.

When a crate is flagged, review its exact source license, license-file and notices, selected feature/dependency tree, patent terms, and bundled native code before allowing it. Do not expand the allowlist just to make CI green.

## Current direct codec/CMS choices

| Dependency | Cargo feature | Upstream metadata/license | Review focus |
| --- | --- | --- | --- |
| workspace `libjpeg-turbo-rs` | `jpeg-read` | IJG and Zlib; pinned port of libjpeg-turbo 3.2.1 | Retain [IJG license](../codecs/libjpeg-turbo-rs/README.ijg), [Zlib notice](../codecs/libjpeg-turbo-rs/LICENSE.md), upstream notices, safe API boundary, ICC/EXIF handling, and codec scratch |
| workspace `libwebp-rs` | `webp-read` | BSD-3-Clause; [license](../codecs/libwebp-rs/LICENSE), [patent grant](../codecs/libwebp-rs/PATENTS), [authors](../codecs/libwebp-rs/AUTHORS), [provenance](../codecs/libwebp-rs/docs/PROVENANCE.md) | Still decoder, ICC/EXIF, opaque scratch and notices; publication gate remains open |
| `jxl-oxide` | `jpeg-xl-read` | Permissive; verify exact crate release | CMS output profile, color/alpha, crop/internal cache |
| `avif-decode` / `rav1d` | `avif-read` | BSD-3-Clause decoder; inspect native/build artifacts | Target-specific NASM assembly, malformed bitstreams, MSRV 1.98 |
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
- [ ] Review native code, assembler and unsafe boundaries in *all* dependencies, not only Ferrixel's `#![forbid(unsafe_code)]` core.
- [x] Commit `Cargo.lock` for the CLI and reproducible measurements ([ADR-0018](adr/0018-optional-codec-and-cms-dependencies.md)).
- [ ] Check that the package advertises its required Rust version accurately and that the target platforms actually compile.
- [ ] Record real AVIF output ICC/CICP and alpha behavior, HDR rejection, and total RSS on representative images.
- [ ] Perform an independent source and legal review of third-party license obligations.

Do **not** declare M1 release-ready merely because the metadata gate passes.

## Transitive review: `avif-parse` (MPL-2.0)

The `avif-decode` dependency currently incorporates `avif-parse` version 2.1.0, which declares **MPL-2.0** (confirmed in its upstream `Cargo.toml` and source headers). MPL-2.0 is **file-level copyleft**, not equivalent to a permissive MIT/Apache grant. A separately linked, unmodified MPL crate may be usable alongside an MIT/Apache project, but distributions must still honor MPL notices and source-availability obligations for covered files and assess any modifications. This review item must not be silently added to the permissive allowlist. Until the source/NOTICE redistribution requirements are explicitly reviewed, the full-dependency license gate remains **open**, even if code CI is green.

Reference: https://github.com/kornelski/avif-parse/blob/main/Cargo.toml
