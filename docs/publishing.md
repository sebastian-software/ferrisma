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

Compressed sizes come from the packaging run of 2026-11-14 below
(`aarch64-apple-darwin`, `main` at `776747fd` plus this branch). crates.io's
upload limit is 10 MiB per `.crate` (Cargo reference, "Publishing on
crates.io").

| Order | Package | Version today | License expression | Needs nightly | `rust-version` | README | `.crate` size | Role |
| --- | --- | --- | --- | --- | --- | --- | ---: | --- |
| 1 | `libsharpyuv-core` | 0.1.0 | BSD-3-Clause | no | 1.95 | yes | 21.1 KiB | Shared SharpYUV core |
| 1 | `ferrisma-kernels` | 0.1.0 | MIT OR Apache-2.0 | no | 1.98 | yes | 18.3 KiB | Engine SIMD kernels (unconditional dependency) |
| 1 | JPEG port, today `libjpeg-turbo-rs` | 0.1.0 | IJG AND Zlib | no | 1.95 | yes | 318.9 KiB | `jpeg-read` |
| 1 | `libjxl-rs` | 0.1.0 | BSD-3-Clause AND MIT AND (Apache-2.0 WITH LLVM-exception) | no | 1.95 | yes | 868.2 KiB | `jpeg-xl-own-read`, `jpeg-xl-write` |
| 1 | `libavif-aom` | 0.1.0 | BSD-2-Clause AND BSD-3-Clause AND MIT | yes | none (nightly) | yes | **16.6 MiB** | AVIF encode/decode |
| 1 | `libavif-dav1d` | 0.1.0 | BSD-2-Clause | yes | none (nightly) | yes | 2.6 MiB | Normal dependency of `libavif-rs` |
| 2 | `libwebp-rs` | 0.1.0 | BSD-3-Clause | no | 1.95 | yes | 796.9 KiB | `webp-read`, `webp-write` |
| 2 | `libavif-sharpyuv` | 0.1.0 | BSD-3-Clause | yes | none (nightly) | yes | 21.8 KiB | SharpYUV C ABI for libavif |
| 3 | `libavif-rs` | 0.1.0 | BSD-2-Clause AND BSD-3-Clause | yes | none (nightly) | yes | 4.6 MiB | libavif port |
| 4 | `ferrisma-avif-read` | **0.0.0** | MIT OR Apache-2.0 | yes | none (nightly) | yes (2026-11-14) | 10.0 KiB | `avif-read` safe bridge |
| 4 | `ferrisma-avif-write` | **0.0.0** | MIT OR Apache-2.0 | yes | none (nightly) | yes (2026-11-14) | 12.5 KiB | `avif-own-write` safe bridge |
| 4 | `ferrisma-jxl-decoder` | 0.1.0 | MIT OR Apache-2.0 | no | 1.95 | yes (2026-11-14) | 13.5 KiB | `jpeg-xl-own-read` safe bridge |
| 4 | `ferrisma-jxl-encoder` | 0.1.0 | MIT OR Apache-2.0 | no | 1.95 | yes (2026-11-14) | 17.6 KiB, first packaged 2026-11-14 | `jpeg-xl-write` safe bridge (lossy) |
| 5 | `ferrisma` | 0.1.0 | MIT OR Apache-2.0 | only for `avif-read`, `avif-own-write`, `cli` | 1.98 | yes | 606.9 KiB | Engine, facade, CLI binary |

Every package now has `description`, `license`, `repository`, `homepage`
(`https://ferrisma.dev`, added to the ports, the AVIF sub-crates and
`libsharpyuv-core` on 2026-11-14), an `include` list, and a README in the
package. The six nightly-only AVIF packages declare no `rust-version` on
purpose: a value cannot say "the pinned nightly". Measured: `rust-version =
"1.100"` accepts the pinned `1.100.0-nightly` and refuses 1.95 with a clear
message, but would also accept a future stable 1.100 that rejects
`#![feature]`. ADR-0040 decision 4 names the nightly in the release notes
instead. The sizes moved since 2026-11-12: `libjxl-rs` 723 to 868 KiB and
`libavif-aom` 16.4 to 16.6 MiB with their sources. `ferrisma` measured
204.1 KiB on 2026-11-12 and 606.9 KiB now, although its `include` list and
the size of its tracked `src`, `tests` and `examples` (1.69 versus 1.73 MB)
barely changed; the earlier figure could not be reproduced, today's is the
measured one (195 files, 2.2 MiB unpacked, 1.1 MiB of it `tests/fixtures`).

Packages within one order level do not depend on each other. The JPEG XL
encoder bridge `ferrisma-jxl-encoder` is at level 4: it depends only on
`libjxl-rs` and must be published before `ferrisma`.

**Stay internal (never published):** `ferrisma-xtask` (workspace tooling),
the isolated fuzz and benchmark workspaces, and `ferrisma-libavif`. The last
is the ADR-0024 compatibility adapter; its encoder is reachable through
`ferrisma`'s own `avif-own-write` feature (ADR-0037), and its dependency on
`ferrisma` has no version requirement. Publish it only after an explicit
decision, and then after `ferrisma`.

## Blockers before the first functional publication

State on 2026-11-14 (on `main` since `aba85c54`). "Proven" means measured
today with the commands in [the evidence below](#evidence-2026-11-14);
"decision" names who has to decide.

| # | Blocker | State | Waits for |
| --- | --- | --- | --- |
| 1 | JPEG port's package name is taken on crates.io (#207) | Proven: still taken; candidate names checked; rename scope listed | Owner: the name (recommendation `ferrisma-libjpeg-turbo`) |
| 2 | `libavif-aom` exceeds the 10 MiB upload limit (#208) | Proven: 16.6 MiB; the four profile translations are nearly all of it; no safe `include` trimming exists | Owner/codec: limit increase or split (recommendation below) |
| 3 | AVIF packages need profile settings dependents do not inherit (#209) | Reproduced with outside consumers: no overflow failure found in the default `dev` profile; `panic = "abort"` is a compile-time error; the documented block works for dependents | Owner: accept option (a), requirement for `unwind`, recommendation for the overflow overrides; codec owners: one AOM test run with overflow checks on |
| 4 | Exact `libc` and `c2rust-bitfields` pins | Fixed on `main` since `aba85c54`: caret requirements, same minimum, `Cargo.lock` unchanged | Codec owners: ABI checks against a newer `libc` (not run) |
| 5 | Versions (the AVIF bridges are 0.0.0) | Open | Owner: O3 (one shared version or per port), first versions |
| 6 | License review gates | Open | Owner/legal (#66, #136) |
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

**Recommendation: `ferrisma-libjpeg-turbo`.** It says whose port it is,
cannot be mistaken for the existing `libjpeg-turbo-rs` (names one suffix
apart, such as `libjpeg-turbo-port`, invite exactly that confusion), does
not claim the upstream project's own name (`libjpeg-turbo`), and is the name
the 2026-11-12 dry run already used. The other ports keep `lib*-rs`; if the
owner wants one scheme, the alternative is `ferrisma-` for every port, which
is a larger rename and not needed for publication.

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

Recommendation: ask crates.io for a per-crate limit (for example 24 MiB) for
`libavif-aom` for the first 0.x releases, since it costs nothing in the
frozen translation and keeps all evidence valid, and record the per-platform
split as the fallback if the request is refused. `libavif-rs` (4.6 MiB) and
`libavif-dav1d` (2.6 MiB) are below the limit.

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
  `panic = "abort"`; that is a source change in the codec crates.

**Recommendation:** (a). Keep `panic = "unwind"` as a stated requirement
(it is enforced), state the four overflow overrides as recommended rather
than required, since no failure was reproduced, and have the codec owners
run the AOM kernel tier and the debug translation once with overflow checks
on (the golden tier already passed that way, release translation only). Do not start (b). Use (c) only if such a run
finds an overflow. Put the block into the `ferrisma` README before
publication:

```toml
# Required by the AVIF features (avif-read, avif-own-write, cli): the
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

`ferrisma-avif-read` and `ferrisma-avif-write` are 0.0.0. Choose
first-publication versions for every package (ADR-0040 proposes the SemVer
policy, [release automation](release-automation.md) the Release Please
shapes; O3 decides one shared version or per-port versions) and update each
dependent's `version` requirement.

### 6. License review gates

`docs/license-audit.md` still lists open review items (IJG terms, the
`simd_helpers` notice, independent legal review). Publishing a source crate
redistributes its files; resolve or explicitly accept these items first.

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
   repository fixtures. Repeat with an AVIF feature on the pinned nightly,
   once with the profile block above and once without it (the 2026-11-14
   rehearsal of this step used `[patch.crates-io]` to the unpacked
   `.crate` files; see the evidence section).
9. Tag the release commit and attach the license bundles and source
   identities.

## Review when

A package is added to or removed from `ferrisma`'s dependency graph, a
package name or version changes, the AVIF packages stop needing nightly or
the profile exceptions, crates.io limits change, or a binary distribution
(CLI archives, npm) is added to the release.
