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
`jpeg-xl-own-read`, `avif-read`, and `avif-own-write`.

## Packages

Compressed sizes come from the dry run below (`aarch64-apple-darwin`,
2026-11-12). crates.io's default upload limit is 10 MiB per crate.

| Order | Package | Version today | License expression | Needs nightly | `.crate` size | Role |
| --- | --- | --- | --- | --- | ---: | --- |
| 1 | `libsharpyuv-core` | 0.1.0 | BSD-3-Clause | no | 21.2 KiB | Shared SharpYUV core |
| 1 | `ferrisma-kernels` | 0.1.0 | MIT OR Apache-2.0 | no | 16.6 KiB | Engine SIMD kernels (unconditional dependency) |
| 1 | JPEG port, today `libjpeg-turbo-rs` | 0.1.0 | IJG AND Zlib | no | 317.1 KiB | `jpeg-read` |
| 1 | `libjxl-rs` | 0.1.0 | BSD-3-Clause AND MIT AND (Apache-2.0 WITH LLVM-exception) | no | 723.4 KiB | `jpeg-xl-own-read`, `jpeg-xl-write` |
| 1 | `libavif-aom` | 0.1.0 | BSD-2-Clause AND BSD-3-Clause AND MIT | yes | **16.4 MiB** | AVIF encode/decode |
| 1 | `libavif-dav1d` | 0.1.0 | BSD-2-Clause | yes | 2.6 MiB | Normal dependency of `libavif-rs` |
| 2 | `libwebp-rs` | 0.1.0 | BSD-3-Clause | no | 755.3 KiB | `webp-read`, `webp-write` |
| 2 | `libavif-sharpyuv` | 0.1.0 | BSD-3-Clause | yes | 22.0 KiB | SharpYUV C ABI for libavif |
| 3 | `libavif-rs` | 0.1.0 | BSD-2-Clause AND BSD-3-Clause | yes | 4.6 MiB | libavif port |
| 4 | `ferrisma-avif-read` | **0.0.0** | MIT OR Apache-2.0 | yes | 9.4 KiB | `avif-read` safe bridge |
| 4 | `ferrisma-avif-write` | **0.0.0** | MIT OR Apache-2.0 | yes | 9.9 KiB | `avif-own-write` safe bridge |
| 4 | `ferrisma-jxl-decoder` | 0.1.0 | MIT OR Apache-2.0 | no | 9.8 KiB | `jpeg-xl-own-read` safe bridge |
| 4 | `ferrisma-jxl-encoder` | 0.1.0 | MIT OR Apache-2.0 | no | not packaged yet | `jpeg-xl-write` safe bridge (lossy), added 2026-11-12 |
| 5 | `ferrisma` | 0.1.0 | MIT OR Apache-2.0 | only for `avif-read`, `avif-own-write`, `cli` | 204.1 KiB | Engine, facade, CLI binary |

Packages within one order level do not depend on each other. The JPEG XL
encoder bridge `ferrisma-jxl-encoder` (2026-11-12) is at level 4: it depends
only on `libjxl-rs` and must be published before `ferrisma`. Its manifest
follows `ferrisma-jxl-decoder` (license expression, `include` list, license
files as links to the root files). It was not part of the packaging run
below; its `.crate` size is not measured.

**Stay internal (never published):** `ferrisma-xtask` (workspace tooling),
the isolated fuzz and benchmark workspaces, and `ferrisma-libavif`. The last
is the ADR-0024 compatibility adapter; its encoder is reachable through
`ferrisma`'s own `avif-own-write` feature (ADR-0037), and its dependency on
`ferrisma` has no version requirement. Publish it only after an explicit
decision, and then after `ferrisma`.

## Blockers before the first functional publication

1. **The JPEG port's package name belongs to someone else.** crates.io's
   `libjpeg-turbo-rs` is an unrelated crate (versions 0.1.0 through 0.9.0,
   MIT OR Apache-2.0, another repository). A published `ferrisma` declaring
   `libjpeg-turbo-rs = "0.1.0"` would silently resolve to that crate. Rename
   the workspace package before publishing (the dry run used
   `ferrisma-libjpeg-turbo`; the library target name `jpeg` can stay, so the
   engine source does not change). The other port names were unregistered on
   2026-11-12; registering them is the only reservation.
2. **`libavif-aom` exceeds the crates.io upload limit** (16.4 MiB compressed,
   206 MiB unpacked). Most of it is the translated source for both supported
   targets (`src/macos`, `src/linux`, about 103 MiB each). Either request a
   per-crate limit increase from the crates.io team or split the platform
   translations into separate crates. The second option is a codec packaging
   change that needs its own codec ADR.
3. **Consumer profile requirements for the AVIF packages.** The generated
   packages rely on the workspace's `overflow-checks = false` overrides and
   `panic = "unwind"` (root `Cargo.toml`, ADR-0026); dependents do not inherit
   these profiles. See the [unsafe review](unsafe-review.md), finding 1.
   Until the generated code no longer needs them, every consumer that enables
   `avif-read`, `avif-own-write`, or `cli` must add:

   ```toml
   [profile.dev]
   panic = "unwind"
   [profile.release]
   panic = "unwind"
   [profile.dev.package.libavif-rs]
   overflow-checks = false
   [profile.dev.package.libavif-aom]
   overflow-checks = false
   [profile.dev.package.libavif-dav1d]
   overflow-checks = false
   [profile.dev.package.libavif-sharpyuv]
   overflow-checks = false
   ```

   Repeat the four package overrides for any other profile that enables
   overflow checks. Put this in the `ferrisma` README before publication.
4. **Exact `libc` and `c2rust-bitfields` pins.** `libavif-rs`,
   `libavif-aom`, `libavif-dav1d`, and `libavif-sharpyuv` require
   `libc = "=0.2.191"`; `libavif-aom` and `libavif-dav1d` also pin
   `c2rust-bitfields = "=0.3.0"`.
   Cargo allows only one semver-compatible `libc` 0.2 in a graph, so a
   dependent whose other dependencies need a newer `libc` cannot resolve.
   The codec owners should relax these to caret requirements if their ABI
   checks permit it, or record why the exact pin must ship.
5. **Versions.** `ferrisma-avif-read` and `ferrisma-avif-write` are 0.0.0.
   Choose first-publication versions for every package (ADR-0040 proposes
   the SemVer policy) and update each dependent's `version` requirement.
6. **License review gates.** `docs/license-audit.md` still lists open review
   items (IJG terms, the `simd_helpers` notice, independent legal review).
   Publishing a source crate redistributes its files; resolve or explicitly
   accept these items first.

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

The same run exposed one packaging defect, fixed on this branch: the WebP
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
   versions, size limit, pins) in the owning ADRs.
2. Change `publish = false` to `publish = ["crates-io"]` on exactly the
   packages in the table, and set their versions and dependency version
   requirements. Leave `ferrisma-xtask` and `ferrisma-libavif` unpublished.
3. Run the local gates: `cargo fmt --all --check`, the engine checks from
   [codec development](codecs.md), `cargo xtask codecs --stage pr` for each
   codec, and the consumer self-check:
   `cargo run --example consumer --features jpeg-read,webp-read,webp-write -- --self-check`.
4. Generate and review the license bundles for every published feature set
   and target ([license audit](license-audit.md)); attach them to the release.
5. Dry-run the chain exactly as above, without `--allow-dirty`, then
   `cargo publish --dry-run` with the same package list.
6. Log in (`cargo login`) and publish in order, one level at a time:

   ```sh
   cargo publish -p libsharpyuv-core
   cargo publish -p ferrisma-kernels
   cargo publish -p <renamed JPEG port>
   cargo publish -p libjxl-rs
   cargo publish -p libavif-aom
   cargo publish -p libavif-dav1d
   cargo publish -p libwebp-rs
   cargo publish -p libavif-sharpyuv
   cargo publish -p libavif-rs
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
   repository fixtures. Repeat with an AVIF feature and the profile block
   above on the pinned nightly.
9. Tag the release commit and attach the license bundles and source
   identities.

## Review when

A package is added to or removed from `ferrisma`'s dependency graph, a
package name or version changes, the AVIF packages stop needing nightly or
the profile exceptions, crates.io limits change, or a binary distribution
(CLI archives, npm) is added to the release.
