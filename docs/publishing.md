# Publishing the crates.io dependency chain

This document defines which packages must reach crates.io for `ferrisma` to be
installable, in which order, what still blocks the first functional
publication, and the exact procedure for the maintainer. It answers #137. It
does not authorize publication: every package remains `publish = false`, and
no version has changed. The `ferrisma` 0.0.0 crate and npm placeholders are
documented separately in [`publishing/ferrisma`](../publishing/ferrisma/README.md).

## Why the codec ports must be published

Cargo strips `path` from dependencies when it packages a crate, so every
dependency of a published crate must itself exist on the registry under the
declared name and a matching version. This includes **optional**
dependencies: crates.io resolves the whole manifest, not only the default
features. A `ferrisma` release with its current features therefore needs every
package reachable through `jpeg-read`, `webp-read`, `webp-write`,
`jpeg-xl-read`, `jpeg-xl-write`, `avif-read`, and `avif-write`. (Until
2026-11-15 the workspace JPEG XL reader was `jpeg-xl-own-read` and the
workspace AVIF writer `avif-own-write`; the dated packaging run below used
those names.)

**Fewer registry packages since 2026-11-15.** With jxl-oxide and rav1e out
of the runtime (ADR-0032, ADR-0037), a published `ferrisma` no longer pulls
in `jxl-oxide`, `jxl-grid` (direct), `rav1e` or `avif-serialize` through any
feature. Measured on the fuzz workspace's lockfile, which resolves `cli` as a
consumer does: 83 package entries left it, the jxl-oxide family
(`jxl-oxide`, `jxl-bitstream`, `jxl-coding`, `jxl-color`, `jxl-frame`,
`jxl-grid`, `jxl-image`, `jxl-jbr`, `jxl-modular`, `jxl-oxide-common`,
`jxl-render`, `jxl-threadpool`, `jxl-vardct`), rav1e and its graph
(`rav1e`, `avif-serialize`, `av-scenechange`, `av1-grain`, `nasm-rs`,
`v_frame`, `rayon`, `rayon-core`, `crossbeam-*`, `bitstream-io`, `y4m`,
`rand`, `thiserror`, `nom`, `itertools`, the `wasm-bindgen` and `syn`
build-time chain, among others). The workspace's own chain of
packages to publish (the table below) is unchanged except that no row
needs NASM. The test-only dev-dependency `jxl-oxide` stays in the root lockfile
(`ravif`, and with it rav1e, left it as well: the one fixture it wrote is
committed); dev-dependencies are not resolved for consumers.

## Packages

Compressed sizes come from the packaging run of 2026-11-14 below
(`aarch64-apple-darwin`, `main` at `776747fd` plus this branch). crates.io's
upload limit is 10 MiB per `.crate` (Cargo reference, "Publishing on
crates.io").

| Order | Package | Version today | License expression | Needs nightly | `rust-version` | README | `.crate` size | Role |
| --- | --- | --- | --- | --- | --- | --- | ---: | --- |
| 1 | `libsharpyuv-core` | 0.1.0 | BSD-3-Clause | no | 1.88 | yes | 21.1 KiB | Shared SharpYUV core |
| 1 | `ferrisma-kernels` | 0.1.0 | MIT OR Apache-2.0 | no | 1.99 | yes | 18.3 KiB | Engine SIMD kernels (unconditional dependency) |
| 1 | JPEG port, today `libjpeg-turbo-rs` | 0.1.0 | IJG AND Zlib | no | 1.93 | yes | 318.9 KiB | `jpeg-read` |
| 1 | `libjxl-rs` | 0.1.0 | BSD-3-Clause AND MIT AND (Apache-2.0 WITH LLVM-exception) | no | 1.94 | yes | 868.2 KiB | `jpeg-xl-read`, `jpeg-xl-write` |
| 1 | `libavif-aom` | 0.1.0 | BSD-2-Clause AND BSD-3-Clause AND MIT | no (since 2026-11-15) | 1.99 | yes | **16.6 MiB** | AVIF encode/decode |
| 1 | `libavif-dav1d` | 0.1.0 | BSD-2-Clause | no (since 2026-11-15) | 1.99 | yes | 2.6 MiB | Normal dependency of `libavif-rs`; stays in the chain (its removal, decided 2026-11-15, is paused since 2026-11-16: [ADR-0031](adr/0031-workspace-libavif-input-experiment.md)) |
| 2 | `libwebp-rs` | 0.1.0 | BSD-3-Clause | no | 1.95 | yes | 796.9 KiB | `webp-read`, `webp-write` |
| 2 | `libavif-sharpyuv` | 0.1.0 | BSD-3-Clause | no | 1.95 | yes | 21.8 KiB | SharpYUV C ABI for libavif |
| 3 | `libavif-rs` | 0.1.0 | BSD-2-Clause AND BSD-3-Clause | no (since 2026-11-15) | 1.99 | yes | 4.6 MiB | libavif port |
| 4 | `ferrisma-avif-read` | **0.0.0** | MIT OR Apache-2.0 | no (since 2026-11-15) | 1.99 | yes (2026-11-14) | 10.0 KiB | `avif-read` safe bridge |
| 4 | `ferrisma-avif-write` | **0.0.0** | MIT OR Apache-2.0 | no (since 2026-11-15) | 1.99 | yes (2026-11-14) | 12.5 KiB | `avif-write` safe bridge |
| 4 | `ferrisma-jxl-decoder` | 0.1.0 | MIT OR Apache-2.0 | no | 1.99 | yes (2026-11-14) | 13.5 KiB | `jpeg-xl-read` safe bridge |
| 4 | `ferrisma-jxl-encoder` | 0.1.0 | MIT OR Apache-2.0 | no | 1.99 | yes (2026-11-14) | 17.6 KiB, first packaged 2026-11-14 | `jpeg-xl-write` safe bridge (lossy) |
| 5 | `ferrisma` | 0.1.0 | MIT OR Apache-2.0 | no | 1.99 | yes | 606.9 KiB | Engine, facade, CLI binary |

Every package now has `description`, `license`, `repository`, `homepage`
(`https://ferrisma.dev`, added to the ports, the AVIF sub-crates and
`libsharpyuv-core` on 2026-11-14), an `include` list, and a README in the
package. Until 2026-11-15 the six AVIF packages needed the pinned nightly and declared
no `rust-version` on purpose (a value cannot say "the pinned nightly").
Since the port's translations were rewritten for stable Rust (libavif-rs
[ADR 0060](../codecs/libavif-rs/docs/adr/0060-build-the-translations-on-stable-rust.md)),
`libavif-aom`, `libavif-dav1d`, `libavif-rs` and the two AVIF bridges
declare `rust-version = "1.99"` (C-variadic functions are stable since
1.99), and `libavif-sharpyuv`, which never used a nightly feature, declares
1.95. ADR-0040 decision 4 (naming the nightly in the release notes) no
longer applies to them. Since the owner's MSRV decision of 2026-11-15
(ADR-0040 amendment) the engine, `ferrisma-kernels` and the four bridges
declare 1.99, the product's one minimum, and the ports published on their
own declare their floors: `libjxl-rs` 1.94 (since 2026-11-16, for the NEON
half-precision intrinsics; 1.88 before), `libsharpyuv-core` 1.88,
`libjpeg-turbo-rs` 1.93, `libwebp-rs` 1.95
([minimum Rust versions](supported-matrix.md#minimum-rust-versions)). The sizes moved since 2026-11-12: `libjxl-rs` 723 to 868 KiB and
`libavif-aom` 16.4 to 16.6 MiB with their sources. `ferrisma` measured
204.1 KiB on 2026-11-12 and 606.9 KiB now, although its `include` list and
the size of its tracked `src`, `tests` and `examples` (1.69 versus 1.73 MB)
barely changed; the earlier figure could not be reproduced, today's is the
measured one (195 files, 2.2 MiB unpacked, 1.1 MiB of it `tests/fixtures`).

**Packaged again on 2026-11-20** (`main` at `61e20ecf`, the same
`cargo package --no-verify --exclude-lockfile --offline` per package; leaves
also verified, that is built from the extracted package:
`libsharpyuv-core`, `ferrisma-kernels`, `libjpeg-turbo-rs`, `libjxl-rs`).
All 14 package; sizes that moved: `libjxl-rs` 938.8 KiB, `libavif-aom`
16.8 MiB, `libavif-dav1d` 3.3 MiB, `libjpeg-turbo-rs` 319.2 KiB, `libwebp-rs`
798.7 KiB, the AVIF bridges 11.2 and 14.6 KiB, `ferrisma-jxl-decoder`
15.4 KiB, `ferrisma-jxl-encoder` 19.8 KiB, and `ferrisma` **1.6 MiB**
(244 files; 606.9 KiB and 195 files on 2026-11-14). `cargo publish --dry-run`
stops every package at `publish = false`, as intended.

Packages within one order level do not depend on each other. The JPEG XL
encoder bridge `ferrisma-jxl-encoder` is at level 4: it depends only on
`libjxl-rs` and must be published before `ferrisma`.

**Stay internal (never published):** `ferrisma-xtask` (workspace tooling),
the isolated fuzz and benchmark workspaces, and `ferrisma-libavif`. The last
is the ADR-0024 compatibility adapter; its encoder is reachable through
`ferrisma`'s own `avif-write` feature (ADR-0037), and its dependency on
`ferrisma` has no version requirement. Publish it only after an explicit
decision, and then after `ferrisma`.

## Blockers before the first functional publication

State on 2026-11-14 (on `main` since `aba85c54`), with the owner's decisions
of 2026-11-15 entered. "Proven" means measured on 2026-11-14 with the
commands in [the evidence below](#evidence-2026-11-14); "decision" names who
has to decide.

| # | Blocker | State | Waits for |
| --- | --- | --- | --- |
| 1 | JPEG port's package name is taken on crates.io (#207) | **Renamed** (2026-11-20, `tools/rename_ports.py --sub-crates`): the four ports and the sub-crates take the `ferrisma-` prefix, the internal adapter is `ferrisma-avif-adapter` (section 1) | Prose of other documents; license inventory and bundles (blocker 6) |
| 2 | `libavif-aom` exceeds the 10 MiB upload limit (#208) | Proven: 16.6 MiB. **Decided** (owner, 2026-11-15): consolidation step (b), then measure again; no split crates, no release-only shipping (section 2) | Codec: step (b) and the new measurement; owner: above about 9 MiB, step (c) before publication or a limit request |
| 3 | AVIF packages need profile settings dependents do not inherit (#209) | Reproduced with outside consumers: no overflow failure found in the default `dev` profile; `panic = "abort"` is a compile-time error; the documented block works for dependents. **Decided** (owner, 2026-11-15): option (a) (section 3). The dav1d kernels' opt-level-0 crash found on 2026-11-20 is fixed in the code (section 3). The block is in the README since 2026-11-20 | Codec owners: the AOM kernel tier and the debug translation with overflow checks on |
| 4 | Exact `libc` and `c2rust-bitfields` pins | Fixed on `main` since `aba85c54`: caret requirements, same minimum, `Cargo.lock` unchanged | Codec owners: ABI checks against a newer `libc` (not run) |
| 5 | Versions (the AVIF bridges are 0.0.0) | **Decided** (owner, 2026-11-15, O3): one shared version for all packages (section 5) | Owner: the first version number |
| 6 | License review gates | **Decided** (owner, 2026-11-15): the #136 policy is confirmed; the documented self-review is accepted for 1.0, without an independent legal review before 1.0; the scope of the patent grants for the translated ports stays named as not assessed (section 6) | After the rename: regenerate the inventory and bundles, re-bind the JPEG evidence |
| 7 | crates.io account setup: team owner, first-publish token, Trusted Publishing per crate | Open, outside Git | Owner |
| 8 | Release Please key mismatch (#248) | Fixed on `main` (`86f9e0df`); rehearsal clean on 2026-11-14 ([release automation](release-automation.md)) | Nobody |

### 1. The JPEG port's package name (#207)

crates.io's `libjpeg-turbo-rs` is an unrelated crate (versions 0.1.0 through
0.9.0, created 2026-03-29, another repository; read again on 2026-11-14). A
published `ferrisma` declaring `libjpeg-turbo-rs = "0.1.0"` would resolve to
that crate. Do not publish under today's name.

Free on 2026-11-14 (crates.io API `GET /api/v1/crates/<name>`, 404):
`ferrisma-libjpeg-turbo`, `ferrisma-jpeg`, `ferrisma-jpeg-turbo`,
`ferrisma-libjpeg`, `libjpeg-turbo-port`, `libjpegturbo-rs`,
`jpeg-turbo-rs`, `libjpeg-turbo-rust`, `libjpeg-turbo`, `libjpeg-rs`,
`turbojpeg-rs`. Every other name of the chain is still free (`libwebp-rs`,
`libjxl-rs`, `libavif-rs`, `libavif-aom`, `libavif-dav1d`,
`libavif-sharpyuv`, `libsharpyuv-core`, `ferrisma-kernels`, the four
bridges, `ferrisma-libavif`); `ferrisma` is the project's 0.0.0 placeholder.

**Decision (owner, 2026-11-15; B1): all four ports take the `ferrisma-`
prefix with the originals' exact names.** The owner's reason: Ferrisma as a
brand. The names are:

| Port today | Package name decided |
| --- | --- |
| `libjpeg-turbo-rs` | `ferrisma-libjpeg-turbo` |
| `libwebp-rs` | `ferrisma-libwebp` |
| `libjxl-rs` | `ferrisma-libjxl` |
| `libavif-rs` | `ferrisma-libavif` |

All four were free on crates.io on 2026-11-15. **Renamed on 2026-11-20**
(branch `v1/rename-ports`) by `python3 tools/rename_ports.py --sub-crates`;
the owner approved the sub-crate names and the adapter's new name that day:

| Package before | Package now |
| --- | --- |
| `libavif-aom` | `ferrisma-libaom` |
| `libavif-dav1d` | `ferrisma-dav1d` |
| `libavif-dav1d-kernels` | `ferrisma-dav1d-kernels` |
| `libavif-sharpyuv` | `ferrisma-libavif-sharpyuv` |
| `libsharpyuv-core` | `ferrisma-sharpyuv` |
| `ferrisma-libavif` (internal ADR-0024 adapter, `crates/ferrisma-libavif`) | `ferrisma-avif-adapter` |

Directories and library target names stay (`jpeg`, `webp`, `jxl`, `avif`,
`aom`, `dav1d`, `dav1d_kernels`, `sharpyuv`; `libsharpyuv_core` and
`ferrisma_libavif` are now pinned in their manifests), so no Rust `use`
changes. The engine's decoder identifiers (`--jxl-decoder=libjxl-rs`,
`pipeline` names) are API values, not package names, and stay. The tables,
sizes and dated evidence on this page keep the names they were measured
with; read them through the two tables above.

The recommendation of 2026-11-14 (rename only the JPEG port to
`ferrisma-libjpeg-turbo`, the other ports keep `lib*-rs`) is replaced by the
decision; its JPEG name is the one decided.

What a rename touches (the library target name `jpeg` stays, so no Rust
`use` changes; the directory `codecs/libjpeg-turbo-rs` can stay):

- Manifests: `codecs/libjpeg-turbo-rs/Cargo.toml` (`name`), the root
  `Cargo.toml` (dependency key and `jpeg-read = ["dep:…"]`), the fuzz
  crate `codecs/libjpeg-turbo-rs/fuzz/Cargo.toml`, and the three lockfiles
  (`Cargo.lock`, `fuzz/Cargo.lock`, `codecs/libjpeg-turbo-rs/fuzz/Cargo.lock`).
- Tooling: `tools/xtask/src/main.rs` (two package maps),
  `codecs/libjpeg-turbo-rs/tools/check.py` (`cargo fmt -p`),
  `scripts/bench/ci-perf/codec_jpeg.sh` (`-p`),
  `scripts/production-license-bundle.py` and its test (package name keys),
  `examples/jpeg_decode_bench.rs` (a printed label).
- Release drafts: both `publishing/release-please/config-*.json` (component
  and lockfile jsonpaths), `publish.workflow.yml.draft` (two lists),
  `rehearse.mjs` (expected versions).
- Prose: 75 mentions in 29 Markdown files; most name the port, not the
  package, and can stay.
- Not affected: no workflow under `.github/workflows` passes the package
  name; the crate's `jpeg` target and the engine sources.

### 2. `libavif-aom` and the upload limit (#208)

`cargo package --no-verify -p libavif-aom` on 2026-11-14: 765 files,
207.7 MiB unpacked, **16.6 MiB compressed**. The `include` list already
packages only `Cargo.toml`, `build.rs`, `README.md`, `NOTICE.md`,
`LICENSES/` and `src/`; tests, examples, golden outputs, references and the
`TRANSLATION-*.json` records stay out. Compressed size by part (gzip of each
subtree of the extracted package):

| Part | Unpacked | Compressed |
| --- | ---: | ---: |
| `src/macos/debug` | 52.4 MiB | 4.3 MiB |
| `src/macos/release` | 50.7 MiB | 4.1 MiB |
| `src/linux/debug` | 52.3 MiB | 4.3 MiB |
| `src/linux/release` | 50.3 MiB | 4.2 MiB |
| `src/simd` (owned kernels) | 3.2 MiB | 0.4 MiB |
| everything else | < 0.1 MiB | < 0.1 MiB |

`src/lib.rs` selects one of the four translations by target OS and
`debug_assertions`, so a dependent's dev build on macOS needs
`macos/debug` and its release build `macos/release`. Leaving any of them
out breaks a build profile on a supported target: **no `include` trimming is
safe**. The trees are not duplicates (37 of 166 files identical between
debug and release on macOS, 3 between macOS and Linux), and gzip `-9` saves
only 0.3 MiB.

Options:

- **Limit increase.** The Cargo reference states the 10 MiB limit and no
  procedure. crates.io administrators raise the limit per crate on request
  (help@crates.io; the usual practice, not confirmed from an official page
  on 2026-11-14). Not asked. Cost: none in the repository; the
  crate stays a 16.6 MiB download for every dependent, and growth (more
  owned SIMD kernels) needs a new request.
- **One crate per platform** (`…-macos`, `…-linux`, each with both
  profiles, behind target-specific dependencies of a small `libavif-aom`
  facade): about 8.8 MiB each with today's sources, about 1.2 MiB under the
  limit. The translations address each other through 1,935 `crate::` paths
  per profile and through the shared `simd`, `jump` and `platform` modules,
  so each platform crate must hold the whole module tree plus those shared
  modules; the facade re-exports `raw`. Touches the aom manifest and
  `lib.rs`, four `modules-*.rs`/`api-*.rs` pairs, `tools/transpile_aom.py`
  (frozen, ADR 0059), `tools/enable_aom_dispatch.py`,
  `tools/import_aom_linux.py`, `tools/root_production.py`,
  `tools/update_production_selection.py`, two translation workflows, and the
  release lists. Needs a codec ADR.
- **One crate per platform and profile:** about 4.7 MiB each (with `simd`), comfortable
  headroom, four crates plus the facade; same tooling cost, more manifests.
- **Ship one profile:** use the release translation for every build profile
  (about 8.7 MiB). Changes which translation a debug build runs, against
  ADR 0048's focused debug acceptance; a codec decision, not packaging.

**Decision (owner, 2026-11-15):** solve the size by consolidation.
Consolidation step (b), merging the debug and release translations of each
platform (the consolidation plan named in libavif-rs
[ADR 0060](../codecs/libavif-rs/docs/adr/0060-build-the-translations-on-stable-rust.md);
step (a), stable Rust, is done), comes first; its expected size is about
8.8 MiB, an estimate, not a measurement. Then the package is measured again.
If it is above about 9 MiB, the owner chooses between step (c) before the
first publication and a limit request to crates.io. No per-platform split
crates and no shipping of one profile only (the options above). crates.io
has not been asked. **Not done yet:** step (b) and the new measurement.

The recommendation of 2026-11-14 (ask crates.io for a per-crate limit first,
keep the per-platform split as the fallback) is replaced by this decision.

**Measured again on 2026-11-20** (`main` at `61e20ecf`, `cargo package
--no-verify --exclude-lockfile --offline -p libavif-aom`): 808 files,
210.4 MiB unpacked, **16.8 MiB compressed** (17,597,959 bytes). Nothing in
the package can be left out without changing what a build runs: every one of
the 4 × 166 translated files is named by its profile's `modules-*.rs`, and
the rest is `simd` (0.6 MB compressed), `lib.rs`, the four module and API
indexes, the README, the notices and the licenses. A trimmed `include`
cannot reach 10 MiB.

What the trees share, from the extracted package (gzip `-6` of each
`diff -r -U0`): macOS debug against release 196 KB, Linux debug against
release 212 KB, macOS release against Linux release 640 KB. A merged
translation per platform (step (b)) would therefore weigh about the release
tree plus its debug differences, about 4.5 MB each, and the package about
9.7 MB, **about 9.3 MiB** with `simd`, before any merge markup: under the
limit, above the owner's 9 MiB threshold. Merging the platforms as well
(step (c)) would add about 0.6 MB to one tree, about 5.6 MB in all. The
whole `src` compresses to 2.6 MiB with `xz -9e` and to 2.0 MiB with
`zstd -19 --long`: the redundancy is across files, which gzip's 32 KiB
window cannot see.

Options and their consequences (the owner's rules: translated code changes
only through committed tools, O8 and port ADR 0059):

| Option | Package | Code behavior | Cost and risk |
| --- | --- | --- | --- |
| Leave files out (`include`) | 16.8 MiB | unchanged | Not possible: every file is used by a supported target and profile |
| (b) merge debug and release per platform (decided) | about 9.3 MiB, estimate | unchanged if the merge only adds `#[cfg(debug_assertions)]` around the differing items | A committed merge tool, re-run of the AOM golden, kernel and ABI tiers on both profiles; about 0.7 MiB headroom, shrinking as owned kernels grow |
| (c) merge the platforms too | about 5.6 MB, estimate | unchanged under the same condition (`#[cfg(target_os)]`) | Same tool, larger; touches `tools/import_aom_linux.py` and the Linux translation workflow |
| Limit request at crates.io | 16.8 MiB | unchanged | Outside Git; every dependent downloads 16.8 MiB; a new request when it grows |
| Ship the translations compressed and expand them in `build.rs` | about 2–3 MiB | unchanged | An opaque archive in the package instead of reviewable source, a decompressor build dependency, about 100 MB written to `OUT_DIR` per build; docs.rs and auditors see no source. Not recommended |
| Split crates per platform or profile | 4.7–8.8 MiB each | unchanged | Rejected by the owner on 2026-11-15 |
`libavif-rs` (4.6 MiB) and `libavif-dav1d` (2.6 MiB) are below the limit.

### 3. Consumer profile requirements of the AVIF packages (#209)

The generated packages were assumed to need the workspace's
`overflow-checks = false` overrides and `panic = "unwind"` (root
`Cargo.toml`, ADR-0026; [unsafe review](unsafe-review.md), finding 1).
Cargo applies profiles only from the root manifest of a build: a
dependent's build ignores those of `ferrisma` (the packaged `ferrisma`
manifest does carry them, so they apply to `cargo install ferrisma`, where
`ferrisma` is the root). Reproduced on 2026-11-14 with consumers outside
the workspace (macOS ARM64, pinned nightly):

- **Overflow checks: no failure found.** A consumer depending on `ferrisma`
  by path (`jpeg-read,avif-read,avif-own-write,webp-write`), with no profile
  settings of its own, builds every crate in Cargo's default `dev` profile:
  overflow checks and debug assertions on, so `libavif-aom` uses its
  `macos/debug` translation, and its objects call the overflow panics
  (`panic_const_add_overflow` is referenced by 169 of its 256 object files,
  `panic_const_sub_overflow` by 162, `panic_const_mul_overflow` by 156,
  `panic_const_shl_overflow` by 110). It ran 23 libavif encode and decode scenarios through the
  public facade (8 and 10 bit, 4:4:4 and 4:2:0, 33x17 to 640x480, speed 0
  to 10, quality 0 to 100, one, two and four encoder threads, AVIF to AVIF
  and AVIF to WebP, truncated inputs) and decoded all 59 AVIF files of the
  libavif test data (21 decoded, 38 rejected with a typed error, 0 panics).
  No overflow panic and no abort. The port's golden tier with overflow
  checks on for every package (`CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true
  python3 tools/check_aom_golden.py` in `codecs/libavif-rs`, release
  optimization, Neon RTCD): 62 of 62 golden AVIF outputs identical (8, 10
  and 12 bit, lossless, alpha, one and four threads), 62 of 62 C-confirmed.
- **Kernel tier with overflow checks: no failure found.** Measured on
  2026-11-16 (Apple M1 Pro, macOS 27.2.1, the pinned nightly; branch
  `v1/aom-x86-7`, `a45cbcf0`) with `CARGO_PROFILE_RELEASE_OVERFLOW_CHECKS=true`
  for every package, in `codecs/libavif-rs`. The `libavif-aom` archive then
  references `panic_const_add_overflow` and `panic_const_mul_overflow` (in
  16 objects each; none without the variable):
  - release translation, release optimization: `cargo test --release
    --locked -p libavif-aom --test simd` 152 of 152 pass; `python3
    tools/check_aom_simd_original.py` (the SIMD-against-original gate) 82 of
    82 cases identical to the original at `neon` and at `scalar`;
  - debug translation (`macos/debug`, selected by adding
    `CARGO_PROFILE_RELEASE_DEBUG_ASSERTIONS=true`): the same two commands,
    152 of 152 and 82 of 82 at `neon` and `scalar`;
  - x86-64, release translation, under QEMU (`qemu-x86_64 -cpu max` in the
    `tools/qemu_x86_64` container, the port cross-built with the same
    variable): `check_aom_simd_original.py --qemu --level avx2` 82 of 82
    cases identical to the original. The x86-64 kernel tests and the debug
    translation on x86-64 were not run with overflow checks.

  No overflow panic in any run. Note that the `checked` profile keeps
  `overflow-checks = false` for the AVIF packages (root `Cargo.toml`), so its
  kernel runs are not overflow evidence.
- **`panic = "abort"` is a build error, not a run-time abort.**
  `libavif-aom` has `#[cfg(panic = "abort")] compile_error!`; a dependent
  whose profile aborts gets "libaom's mapped nonlocal error handling
  requires panic=unwind" at compile time, with or without the overflow
  overrides. Cargo rejects `panic` in a per-package profile ("`panic` may
  not be specified in a `package` profile"), so such a dependent can only
  switch its whole profile to `unwind` (Cargo's default) or leave the AVIF
  features off.
- **The documented block works.** A second consumer built from the packaged
  crates (see [the evidence](#evidence-2026-11-14), item 2) with the block
  below copied into its own manifest: `libavif-aom`'s objects then reference
  only the division and remainder overflow panics, which Rust keeps in every
  profile, and none of the add, subtract, multiply, shift or negate ones;
  the consumer's own crate keeps its overflow checks. The same 23 scenarios
  and 59 files gave the same result.

Options and their cost:

- **(a) Document the profile.** Works as measured above. `panic = "unwind"`
  is required and already enforced at compile time; the overflow overrides
  are a precaution that keeps a dependent's debug build on the configuration
  the codec evidence was recorded with. Cost: one README paragraph.
- **(b) Make the generated code independent of the profile.** The
  translations already use explicit `wrapping_*` calls where C2Rust knows C
  wraps (unsigned arithmetic: about 2,400 calls per `libavif-aom`
  translation); the plain `+ - * <<` operators left are signed or
  floating-point C arithmetic, about 57,000 sites per translation (4
  translations), about 97,000 in `libavif-dav1d` and about 20,000 in
  `libavif-rs` (upper bounds from a token count, including floating
  point). Signed overflow is undefined in C, so a conforming original never
  relies on it. The translation tooling (C2Rust 0.21.0, called without
  options in `tools/transpile_aom.py`) has no switch for this that the
  repository uses, and port ADR 0059 froze the libaom translation: the
  committed Rust is now the source, so a rewrite would be an owned-code
  change of some 230,000 sites gated on the golden outputs, not a
  regeneration.
- **(c) What Cargo offers a library.** Nothing that forces a profile:
  `[profile]` in a dependency is ignored, `panic` cannot be set per package,
  and a build script cannot pass `-C overflow-checks`. On the pinned
  nightly a library can detect the setting at compile time
  (`#![feature(cfg_overflow_checks)]`, `#[cfg(overflow_checks)]`; measured),
  so the AVIF crates could reject or warn about it the way they reject
  `panic = "abort"`; that is a source change in the codec crates. Since the
  AVIF packages build on stable Rust (2026-11-15), this option would bring a
  nightly requirement back and is no longer open without that cost.

**Decision (owner, 2026-11-15): option (a).** `panic = "unwind"` is
required (it is enforced at compile time), and the four overflow overrides
are recommended, not required. Removing the unwind requirement is a
candidate for after 1.0, together with consolidation step (c). (b) is not
started. Since the rename (2026-11-20) the block, with the new names, is in
the [`ferrisma` README](../README.md#build-profile-for-the-avif-features);
the supported matrix does not repeat it yet. The block as it stood before
the rename:

```toml
# Required by the AVIF features (avif-read, avif-write, cli): the
# libaom port maps C's longjmp error recovery to unwinding.
[profile.dev]
panic = "unwind"
[profile.release]
panic = "unwind"
# Recommended: build the generated AVIF code as the codec evidence was
# recorded (no overflow checks); repeat for every profile with overflow checks.
[profile.dev.package.libavif-rs]
overflow-checks = false
[profile.dev.package.libavif-aom]
overflow-checks = false
[profile.dev.package.libavif-dav1d]
overflow-checks = false
[profile.dev.package.libavif-sharpyuv]
overflow-checks = false
```

**The kernels' optimization level (2026-11-20, resolved the same day).**
The dav1d kernel rollout moved dav1d's CDEF and inverse transforms into a
package of their own (`libavif-dav1d-kernels`, now `ferrisma-dav1d-kernels`)
on fearless_simd, with `opt-level = 1` for it and for `fearless_simd` in
the root `dev` profile. A dependent does not inherit that. Reproduced with a
consumer outside the workspace that depended on `libavif-rs` from
`v1/dav1d-kernel-rollout` (`d522dd15`), with no profile of its own, over the
59 AVIF files of the libavif test data through the port's dav1d: on x86-64
(AVX2, Linux under Rosetta) with 1 and 8 threads, **7 runs crashed with
SIGSEGV, all with 8 threads**; with the two `opt-level` lines, none. The
cause was stack frames of up to 1.75 MB at opt-level 0 on dav1d's 1 MB
worker stacks. macOS ARM64 did not crash.

**Fixed in the code, not in the profile** (`e80d0713`, `adcde4dc`, merged
to `main` in `8e86ae17`; libavif-rs ADR 0062): the transforms are selected
with `if const` and each computing statement runs in a frame of its own
when unoptimized, so the crate's largest frame at opt-level 0 is about
104 KiB on x86-64. The same consumer then decoded without a crash at
opt-level 0 (ADR 0062: x86-64 106 decoded, 12 typed failures, 0 crashes;
ARM64 159, 18, 0). The `build.rs` guard proposed earlier that day (a library's
build script does see the dependent's `OPT_LEVEL`, though not its overflow
setting) is therefore not needed and was not added. The root profile's
`opt-level = 1` for the kernels is a compile-time setting only; the README
block lists it as optional.

### 4. Exact pins

`libavif-rs`, `libavif-aom`, `libavif-dav1d` and `libavif-sharpyuv` required
`libc = "=0.2.191"`; `libavif-aom` and `libavif-dav1d` also
`c2rust-bitfields = "=0.3.0"`. Cargo allows one semver-compatible `libc`
0.2 per graph, so any dependent needing a newer `libc` could not resolve.
No document recorded a reason for the pins. The translations use `libc`
only for opaque storage types passed back to `libc` functions
(`pthread_mutex_t`, `pthread_cond_t`, `pthread_t`, `FILE`, …). Since
2026-11-14 they are caret requirements with the same minimum (`0.2.191`,
`0.3.0`); `Cargo.lock` is unchanged, so every recorded result still used
0.2.191. The codec ABI checks have not been run against a newer `libc`.

### 5. Versions

`ferrisma-avif-read` and `ferrisma-avif-write` are 0.0.0. **Decided (owner,
2026-11-15, O3):** every package shares one version, configuration A of
[release automation](release-automation.md); configuration B stays possible
later. Still to do: choose the first version (ADR-0040 proposes the SemVer
policy) and update each dependent's `version` requirement.

### 6. License review gates

**Decided (owner, 2026-11-15):** the licensing policy for project-owned
code and additions inside a port (#136) is confirmed; the documented
self-review is accepted for 1.0, and there is no independent legal review
before 1.0; the scope of the patent grants for the translated ports stays
named as not assessed ([license audit](license-audit.md)). The
`simd_helpers` item is moot: the package left the root `Cargo.lock` with
rav1e. **Not done yet**, after the rename: regenerate the inventory and the
license bundles, and re-bind the JPEG source-bound evidence to the reviewed
files.

## Evidence (2026-11-14)

Run on `aarch64-apple-darwin` with the pinned nightly, from a clean checkout
of the release-plumbing work, on `main` since `aba85c54`. Nothing was uploaded; no `publish = false`
and no version was changed, not even temporarily.

1. **Packaging of every crate, one at a time:**

   ```sh
   for p in libsharpyuv-core ferrisma-kernels libjpeg-turbo-rs libjxl-rs \
            libavif-aom libavif-dav1d libwebp-rs libavif-sharpyuv libavif-rs \
            ferrisma-avif-read ferrisma-avif-write ferrisma-jxl-decoder \
            ferrisma-jxl-encoder ferrisma; do
     cargo package --no-verify --exclude-lockfile --offline -p "$p" --target-dir target/pkg-all
   done
   ```

   `--exclude-lockfile` skips the lockfile generation that would look up the
   unpublished first-party dependencies on crates.io; the multi-package run
   of 2026-11-12 needed `publish = false` removed for Cargo's local overlay
   registry, which this run avoids. The sizes are in the table above.
2. **Build from the packaged crates:** the 14 `.crate` files were unpacked
   outside the repository and a separate consumer crate (its own
   `[workspace]`, the workspace's `Cargo.lock` copied in) depended on
   `ferrisma = "0.1.0"` with `cli`, every chain package replaced through
   `[patch.crates-io]` by its unpacked directory. `cargo tree` confirms that
   `libjpeg-turbo-rs` resolves to the unpacked port, not to the crates.io
   crate of the same name and version. `cargo build` of that consumer (`dev`
   profile, the #209 block in its manifest) compiled every package of the
   chain from its packaged sources, including `ferrisma` with `cli`, and its
   AVIF run passed (23 scenarios, 59 test files, 0 panics).
3. **JPEG port alone:** `cargo package -p libjpeg-turbo-rs` (Cargo's own
   build of the extracted crate) passes; from the unpacked `.crate`,
   `cargo +1.95 build --locked` passes with default features, with
   `--no-default-features`, and for `--target x86_64-apple-darwin`. With the
   upstream submodule initialized, no file under `upstream/` is packaged.
4. **Release Please rehearsal** for both drafts: clean, see
   [release automation](release-automation.md#rehearsal-procedure-local-offline).

## Dry-run evidence (2026-11-12)

With temporary, uncommitted edits that removed `publish = false` from the
13 packages above other than `ferrisma-jxl-encoder` (which was not part of
this run, see above) and renamed the JPEG package to `ferrisma-libjpeg-turbo`,
one multi-package invocation packaged and verified the whole chain through
Cargo's local overlay registry:

```sh
cargo package --allow-dirty \
  -p ferrisma -p ferrisma-kernels -p libsharpyuv-core -p ferrisma-libjpeg-turbo \
  -p libwebp-rs -p libjxl-rs -p libavif-rs -p libavif-aom -p libavif-dav1d \
  -p libavif-sharpyuv -p ferrisma-avif-read -p ferrisma-avif-write \
  -p ferrisma-jxl-decoder --features ferrisma/cli,ferrisma/jpeg-xl-own-read
```

Result: exit status 0. Cargo chose the order shown in the table; every
package built from its extracted `.crate` (debug profile), and `ferrisma` built
with `cli,jpeg-xl-own-read` against the packaged dependencies. The edits were
then reverted. This proves that the packages contain what their library
builds need; it does not run their tests, does not check the crates.io size
limit, and does not exercise a consumer outside the workspace profiles.

The same run exposed one packaging defect, fixed on `main` since `aba85c54`: the WebP
writer embedded the sRGB profile from the nested `libavif-rs` upstream
snapshot, which the `ferrisma` package cannot contain. The profile now has a
package-local copy in `assets/icc` (same SHA-256).

Package contents are restricted with `include` lists, so the upstream
submodules (`upstream/`), oracle tooling, test data, and the libavif upstream
snapshot stay out of the packages even when a maintainer checkout has them
initialized. Each package keeps its README, its license, notice, and patent
files, and its sources. `libavif-rs` also ships the bundled libyuv `AUTHORS`
file.

## Release procedure for the maintainer

Run from a clean checkout of the release commit, with the pinned toolchain.

1. Resolve the blockers above and record the decisions (package rename,
   versions, size limit, consumer profile) in the owning ADRs.
2. Change `publish = false` to `publish = ["crates-io"]` on exactly the
   packages in the table, and set their versions and dependency version
   requirements. Leave `ferrisma-xtask` and `ferrisma-libavif` unpublished.
3. Run the local gates: `cargo fmt --all --check`, the engine checks from
   [codec development](codecs.md), `cargo xtask codecs --stage pr` for each
   codec, and the consumer self-check:
   `cargo run --example consumer --features jpeg-read,webp-read,webp-write -- --self-check`.
   Then require the hosted tiers on the same commit (engine ADR-0027,
   amendment 2026-11-17): the last [nightly](../.github/workflows/nightly.yml)
   and the last [deep](../.github/workflows/deep.yml) run of the release
   commit passed. If the commit has none, dispatch both on it
   (`gh workflow run nightly.yml --ref <commit's branch>` and
   `gh workflow run deep.yml --ref <commit's branch>`) and wait for them. A
   red job blocks the release until a change fixes it and both tiers pass on
   the new commit.
4. Generate and review the license bundles for every published feature set
   and target ([license audit](license-audit.md)); attach them to the release.
5. Dry-run the chain exactly as above, without `--allow-dirty`, then
   `cargo publish --dry-run` with the same package list.
6. Log in (`cargo login`) and publish in order, one level at a time:

   ```sh
   cargo publish -p ferrisma-sharpyuv
   cargo publish -p ferrisma-kernels
   cargo publish -p ferrisma-libjpeg-turbo
   cargo publish -p ferrisma-libjxl
   cargo publish -p ferrisma-libaom
   cargo publish -p ferrisma-dav1d-kernels
   cargo publish -p ferrisma-dav1d
   cargo publish -p ferrisma-libwebp
   cargo publish -p ferrisma-libavif-sharpyuv
   cargo publish -p ferrisma-libavif
   cargo publish -p ferrisma-avif-read
   cargo publish -p ferrisma-avif-write
   cargo publish -p ferrisma-jxl-decoder
   cargo publish -p ferrisma-jxl-encoder
   cargo publish -p ferrisma
   ```

   Cargo can also publish the list in one `cargo publish -p … -p …`
   invocation; publishing level by level makes a partial failure easier to
   resume. A published version cannot be replaced, only yanked.
7. Add the project's crates.io team as an owner of every new crate
   (`cargo owner --add`), so publication does not depend on one account.
8. Verify from outside the workspace: create a new crate, depend on the
   published `ferrisma` with `jpeg-read,webp-read,webp-write`, copy
   `examples/consumer.rs` into it, and run `--self-check` with the
   repository fixtures. Repeat with an AVIF feature on the pinned nightly,
   once with the profile block above and once without it (the 2026-11-14
   rehearsal of this step used `[patch.crates-io]` to the unpacked
   `.crate` files; see the evidence section).
9. Tag the release commit and attach the license bundles and source
   identities.

## Review when

The rename of #207 is done (this page then needs the new names throughout);
consolidation step (b) changes the `libavif-aom` size; a package is added to or removed from `ferrisma`'s dependency graph, a
package name or version changes, the AVIF packages stop needing nightly or
the profile exceptions, crates.io limits change, or a binary distribution
(CLI archives, npm) is added to the release.
