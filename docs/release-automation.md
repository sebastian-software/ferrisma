# Release automation with Release Please (decision D10, inert draft)

Status: **draft, inactive.** Prepared 2026-11-13 for owner decision D10
("0.x pre-releases from `main`; 1.0 is reserved for the state that meets the
quality gate completely; releases use Release Please the way the
organization's other packages do"). Nothing here runs: the configurations and
the workflow live in [`publishing/release-please/`](../publishing/release-please/),
which GitHub and Release Please do not read, the workflow file has a
non-`.yml` extension, every package keeps `publish = false`, and no version
changed. Whether all crates share one version is open (O3); this document
lays out both options so the owner can decide.

Related: [publishing](publishing.md) (the crates.io chain, order, and manual
procedure, #137), [ADR-0040](adr/0040-stable-public-contract-and-compatibility-policy.md)
(SemVer and compatibility policy), [ADR-0026](adr/0026-codecs-in-one-repository-and-workspace.md)
(one workspace), [`publishing/ferrisma`](../publishing/ferrisma/README.md)
(the 0.0.0 placeholders).

## The organization blueprint

Read on 2026-11-13 with read-only GitHub API calls.

| Convention | What the organization does | Source |
| --- | --- | --- |
| Tool and trigger | Release Please runs on every push to `main` as the first job of one `publish.yml`; it keeps one release pull request open. **Merging that pull request publishes**: it writes the version commit, creates the tag and the GitHub release, and the publish jobs run from that commit. `workflow_dispatch` only retries an existing tag. | ferromark `docs/arch/ADR-0020-standards-release-blueprint.md`, `.github/workflows/publish.yml`; ferrugo and ferriki `.github/workflows/publish.yml`; ferromark `docs/releasing.md` |
| Release type | `release-type: rust`. Versions are concrete in every `Cargo.toml`; `version.workspace = true` is not used because the Rust updater cannot replace it. The `simple` strategy with a version file is only for workspaces that inherit one version. | ferromark ADR-0020 ("One root package"); project-infra v0.3.0 `skills/project-infra/references/ci-and-releases.md` |
| Workspace shapes | One root package with one component (ferromark, ferriki), or `cargo-workspace` plus `linked-versions` with one component per crate (ferrugo: ten crates on one version, `separate-pull-requests: false`, `updateAllPackages: true`). | ferromark and ferriki `release-please-config.json` / `.release-please-config.json`; ferrugo `.release-please-config.json`, `.release-please-manifest.json`, `docs/packaging.md` |
| Tag format | `include-component-in-tag` decides `<component>-v<version>` or `v<version>`. Guidance: use a component tag whenever the repository can ever hold a second versioned artifact. ferromark and ferriki use `v<version>` for historical continuity; ferrugo produces `<crate>-v<version>` per crate. | project-infra `ci-and-releases.md` ("Configure Release Please"); ferrugo tags |
| Changelog | Release Please writes it. One `CHANGELOG.md` per component (root for a single component, one per crate in ferrugo). Curated narrative goes above the generated entry in a separate `docs:` commit. | ferromark `docs/releasing.md`; ferrugo `crates/*/CHANGELOG.md`; project-infra `ci-and-releases.md` |
| Pre-1.0 versions | `bump-minor-pre-major: true` "keeps a breaking change from declaring 1.0"; `bump-patch-for-minor-pre-major` optionally keeps a feature a patch. ferriki used `bump-minor-pre-major` through 0.x. ferrugo does not set it. | project-infra `ci-and-releases.md`; ferriki `.release-please-config.json` |
| Deliberate versions | A `Release-As: X.Y.Z` commit footer forces a version for one release (first release, leaving a candidate series, a stable 1.0). Never a permanent forced version in configuration. Candidate series use `versioning: prerelease`, `prerelease-type: rc`, `prerelease: true`, removed again with the stable `Release-As`. | ferromark `docs/arch/ADR-0016-coordinated-workspace-releases.md`, ADR-0020 amendment 2026-09-18, `docs/releasing.md` |
| GitHub prerelease flag | Release Please marks a GitHub release as prerelease only when `prerelease: true` **and** the version has a prerelease part or major 0 (`release-please` 17.6.0 `src/manifest.ts`). ferrugo's 0.x releases are therefore ordinary releases. | release-please v17.6.0 `src/manifest.ts`; ferrugo releases |
| Action pin | `googleapis/release-please-action@0dfd8538845b8e92600d271a895a5372865d4062 # v5` in ferromark and ferriki. That SHA is the annotated `v5` **tag object**; it peels to commit `45996ed1f6d02564a971a2fa1b5860e934307cf7` (v5.0.0), which locks `release-please` 17.6.0. project-infra asks for the peeled commit (`git ls-remote … 'refs/tags/<tag>^{}'`). ferrugo still uses the floating `@v5`. | ferromark, ferriki `publish.yml`; release-please-action `package-lock.json` at v5.0.0; project-infra `ci-and-releases.md` |
| Release pull request token | `token: ${{ secrets.RELEASE_PLEASE_TOKEN \|\| secrets.GITHUB_TOKEN }}`: a pull request opened with `GITHUB_TOKEN` starts no workflow runs, so it would show no CI checks. | ferromark ADR-0016 amendment, `publish.yml`; project-infra `assets/ci/release-please/release.yml` |
| crates.io authentication | **Trusted Publishing** (OIDC, `permissions: id-token: write`) through `sebastian-software/project-infra/.github/actions/publish-crates@c97d8db475ce5abb7e7ab7b0ac22e249346b9fab # v0.3.0`, which wraps `rust-lang/crates-io-auth-action`. A first-ever publish of a crate cannot use it; pass a `token` input for that release only, enable Trusted Publishing per crate, then remove the token. ferrugo uses `rust-lang/crates-io-auth-action@v1` directly with a local script. | project-infra `.github/actions/publish-crates/action.yml`, `ci-and-releases.md`; ferromark, ferriki, ferrugo `publish.yml` |
| Publish order and retries | The shared action takes an ordered list (leaves first), skips versions already in the sparse index, retries once, and waits for index propagation, so a re-run is idempotent. | project-infra `publish-crates/action.yml`; ferrugo `scripts/publish_crates.sh` |
| Rehearsal | ferromark runs the pinned `release-please` 17.6.0 `Manifest`, strategies and updaters against its own files and a synthetic commit history, with no GitHub client and no publishing method, including negative cases (inherited member version, path dependency without version). A second step writes the candidate files and runs `cargo metadata --locked` on them, and a third builds the real `.crate` and a consumer against it. Changing the action pin requires repeating the rehearsal. | ferromark ADR-0016 (contract), ADR-0020 "Validation", `scripts/test-release-rehearsal.mjs`, `scripts/lib/release-rehearsal.mjs` |
| Org-wide rules | Pin every action to a full commit SHA with a version comment; job timeouts; read-only default permissions; serialize publication (`cancel-in-progress: false`); validate Conventional Commit subjects (every subject in a repository that does not squash); keep every file that carries a version in the strategy or `extra-files`, including secondary lockfiles; check out the release tag, never `main`, in publish jobs; an optional committed "publication hold" (`minimumMajor`, `publicationEnabled`, `reason`) blocks release pull requests and publication while set. | project-infra `ci-and-releases.md` |

## How the blueprint maps onto Ferrisma

Ferrisma differs from the blueprint repositories in four ways that matter:

1. **The workspace root is also the engine package** (`ferrisma`), and it
   lists `"."` in `members`. ferromark has the same root-package shape but
   does not list `"."`; ferrugo has a virtual root.
2. **Fourteen packages would be published** (the table in
   [publishing](publishing.md): engine, kernels, four bridges, four codec ports,
   the three AVIF sub-crates, and `libsharpyuv-core`), plus two members that
   stay internal (`ferrisma-xtask`, `ferrisma-libavif`).
3. **Several first-party dependencies were renamed** when these drafts were
   written: `webp = { package = "libwebp-rs" }` and
   `jxl = { package = "libjxl-rs" }` in the engine, `jxl = { package =
   "libjxl-rs" }` in both JPEG XL bridges, and `sharpyuv`/`dav1d`/`aom` in
   `libavif-rs`. Release Please matches dependencies by key, not by
   `package` (see [known limits](#known-limits-measured)). Since `86f9e0df`
   (2026-11-13, #248) every manifest keys them by package name; no
   `package =` entry is left.
4. **The AVIF packages need the pinned nightly** (ADR-0026), so the publish
   job must use `rust-toolchain.toml`, not `stable`.

Existing version state (2026-11-13): every member is `0.1.0` except
`ferrisma-avif-read`, `ferrisma-avif-write`, and `ferrisma-libavif` (`0.0.0`).
Every package is `publish = false`. First-party requirements are **caret**
requirements (`version = "0.1.0"`, and `"0.0.0"` for the two AVIF bridges,
which under Cargo's 0.0.x rule is effectively exact). There is **no** `=` pin
between first-party crates; the only exact pins are the external
`libc = "=0.2.191"` (four AVIF packages) and `c2rust-bitfields = "=0.3.0"`
(`libavif-aom`, `libavif-dav1d`), tracked in #209. Three secondary lockfiles
carry first-party versions: `fuzz/Cargo.lock` (all fourteen),
`codecs/libjpeg-turbo-rs/fuzz/Cargo.lock`, and
`codecs/libwebp-rs/fuzz/Cargo.lock`; both drafts update them as `toml`
extra files with the jsonpath form ferriki uses.

Common settings in both drafts: `release-type: rust`, component tags
(`include-component-in-tag: true`, following project-infra because the
repository already holds several versioned artifacts), `bump-minor-pre-major:
true`, `bump-patch-for-minor-pre-major: false`, `prerelease: true` (so 0.x
GitHub releases are marked as pre-releases, matching D10's wording), and a
`bootstrap-sha` so the first changelog does not list the whole history. The
`bootstrap-sha` in the drafts is `e4038a4a`, the head of `main` when the
drafts were written (2026-11-12, 23:15 +0100); replace it with the activation
commit.

## Configuration A: one shared version

Every first-party package (published or not) carries the same version, one
`CHANGELOG.md` at the root, one tag `ferrisma-v<version>`, one GitHub release.
Mechanically this is ferromark's shape: one root package with the `rust`
strategy, which already rewrites **every** workspace member to the root's
version, the members' path requirements, and `Cargo.lock`. It reaches what
ferrugo reaches with `cargo-workspace` plus `linked-versions`, without
fourteen tags and fourteen GitHub releases per release.

File: [`publishing/release-please/config-shared-version.json`](../publishing/release-please/config-shared-version.json)

```json
{
  "$schema": "https://raw.githubusercontent.com/googleapis/release-please/main/schemas/config.json",
  "release-type": "rust",
  "include-component-in-tag": true,
  "bump-minor-pre-major": true,
  "bump-patch-for-minor-pre-major": false,
  "prerelease": true,
  "bootstrap-sha": "e4038a4a9adb4a4b92b9a9ba4c6441f58bbcbfbf",
  "packages": {
    ".": {
      "component": "ferrisma",
      "changelog-path": "CHANGELOG.md",
      "extra-files": [
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-kernels' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-avif-read' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-avif-write' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-jxl-decoder' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-jxl-encoder' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libsharpyuv-core' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libjpeg-turbo-rs' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libwebp-rs' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libjxl-rs' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-rs' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-aom' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-dav1d' && !@.source)].version"},
        {"type": "toml", "path": "fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-sharpyuv' && !@.source)].version"},
        {"type": "toml", "path": "codecs/libjpeg-turbo-rs/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libjpeg-turbo-rs' && !@.source)].version"},
        {"type": "toml", "path": "codecs/libwebp-rs/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libwebp-rs' && !@.source)].version"}
      ]
    }
  }
}
```

Manifest, [`publishing/release-please/manifest-shared-version.json`](../publishing/release-please/manifest-shared-version.json)
(becomes `.release-please-manifest.json`; `0.0.0` records that no functional
version was released; the crates.io placeholder `ferrisma 0.0.0` was not
tagged):

```json
{
  ".": "0.0.0"
}
```

What A implies:

- **Pins.** All requirements move in lockstep, so caret requirements are
  enough; under 0.x a caret requirement `0.2.0` already excludes `0.3.0`.
  If the owner wants `=` pins between first-party crates (for example a bridge
  on the port's raw-pointer API), note that the Rust updater writes a plain
  version and drops `=`; ferromark restores its one exact pin with a post-step
  (`scripts/preserve-exact-transform-pin.mjs`) pushed to the release branch.
  Nothing in Ferrisma needs that today.
- **Publish order.** Every release publishes all fourteen crates in the order
  of [publishing](publishing.md); the shared action skips nothing new, so a
  one-line fix in one port still uploads fourteen versions. Unchanged codec
  ports get new versions with no change, which costs crates.io storage (the AVIF
  ports are several MiB each) and download noise.
- **SemVer.** A breaking change anywhere bumps everyone's minor (in 0.x),
  including ports whose API did not change. ADR-0040 problem 7
  (`ferrisma-kernels` as a public dependency) is solved by construction.
- **Changelog.** One root `CHANGELOG.md`; commit scopes (`fix(webp):`,
  `feat(jxl):`) show which component changed.

## Configuration B: engine and bridges on one version, each port its own

The engine group (`ferrisma`, `ferrisma-kernels`, the four bridges, and the
internal `ferrisma-libavif`) is one `linked-versions` group; the AVIF family
(`libavif-rs` and its `aom`, `dav1d`, `sharpyuv` crates) is a second group;
`libjpeg-turbo-rs`, `libwebp-rs`, `libjxl-rs`, and `libsharpyuv-core` are
independent components. `cargo-workspace` propagates dependency bumps. Each
component has its own `CHANGELOG.md` in its directory and its own
`<component>-v<version>` tag and GitHub release; one merged release pull request
carries all of them (`separate-pull-requests: false`).

**B requires a virtual workspace root.** With the engine package at the
repository root, the `rust` strategy for path `.` sees `[workspace].members`
and rewrites every member's version to the engine's (release-please 17.6.0
`src/strategies/rust.ts`), which collides with per-port versions. The rehearsal
reproduced it with an earlier draft that kept the engine at `.`: a fix in
`libwebp-rs` set every package to 0.1.1, left six root `Cargo.lock` entries
behind, and produced 23 findings in total. B therefore assumes the
engine moves to `crates/ferrisma` (ferrugo's layout), reversing the choice
ferromark made in its ADR-0020 for the `rust` strategy. That move is a
workspace change (shared root `Cargo.toml`, CI paths, `cargo xtask`, docs) and
needs its own decision.

File: [`publishing/release-please/config-per-port.json`](../publishing/release-please/config-per-port.json)

```json
{
  "$schema": "https://raw.githubusercontent.com/googleapis/release-please/main/schemas/config.json",
  "release-type": "rust",
  "include-component-in-tag": true,
  "bump-minor-pre-major": true,
  "bump-patch-for-minor-pre-major": false,
  "prerelease": true,
  "bootstrap-sha": "e4038a4a9adb4a4b92b9a9ba4c6441f58bbcbfbf",
  "separate-pull-requests": false,
  "plugins": [
    {"type": "cargo-workspace", "merge": false},
    {
      "type": "linked-versions",
      "groupName": "ferrisma",
      "components": ["ferrisma", "ferrisma-kernels", "ferrisma-avif-read", "ferrisma-avif-write", "ferrisma-jxl-decoder", "ferrisma-jxl-encoder", "ferrisma-libavif"],
      "merge": false
    },
    {
      "type": "linked-versions",
      "groupName": "libavif-rs",
      "components": ["libavif-rs", "libavif-aom", "libavif-dav1d", "libavif-sharpyuv"],
      "merge": false
    }
  ],
  "packages": {
    "crates/ferrisma": {
      "component": "ferrisma",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma' && !@.source)].version"}
      ]
    },
    "crates/ferrisma-kernels": {
      "component": "ferrisma-kernels",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-kernels' && !@.source)].version"}
      ]
    },
    "crates/ferrisma-avif-read": {
      "component": "ferrisma-avif-read",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-avif-read' && !@.source)].version"}
      ]
    },
    "crates/ferrisma-avif-write": {
      "component": "ferrisma-avif-write",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-avif-write' && !@.source)].version"}
      ]
    },
    "crates/ferrisma-jxl-decoder": {
      "component": "ferrisma-jxl-decoder",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-jxl-decoder' && !@.source)].version"}
      ]
    },
    "crates/ferrisma-jxl-encoder": {
      "component": "ferrisma-jxl-encoder",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='ferrisma-jxl-encoder' && !@.source)].version"}
      ]
    },
    "crates/libsharpyuv-core": {
      "component": "libsharpyuv-core",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libsharpyuv-core' && !@.source)].version"}
      ]
    },
    "codecs/libjpeg-turbo-rs": {
      "component": "libjpeg-turbo-rs",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libjpeg-turbo-rs' && !@.source)].version"},
        {"type": "toml", "path": "/codecs/libjpeg-turbo-rs/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libjpeg-turbo-rs' && !@.source)].version"}
      ]
    },
    "codecs/libwebp-rs": {
      "component": "libwebp-rs",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libwebp-rs' && !@.source)].version"},
        {"type": "toml", "path": "/codecs/libwebp-rs/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libwebp-rs' && !@.source)].version"}
      ]
    },
    "codecs/libjxl-rs": {
      "component": "libjxl-rs",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libjxl-rs' && !@.source)].version"}
      ]
    },
    "codecs/libavif-rs": {
      "component": "libavif-rs",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-rs' && !@.source)].version"}
      ]
    },
    "codecs/libavif-rs/crates/aom": {
      "component": "libavif-aom",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-aom' && !@.source)].version"}
      ]
    },
    "codecs/libavif-rs/crates/dav1d": {
      "component": "libavif-dav1d",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-dav1d' && !@.source)].version"}
      ]
    },
    "codecs/libavif-rs/crates/sharpyuv": {
      "component": "libavif-sharpyuv",
      "extra-files": [
        {"type": "toml", "path": "/fuzz/Cargo.lock", "jsonpath": "$.package[?(@.name.value=='libavif-sharpyuv' && !@.source)].version"}
      ]
    },
    "crates/ferrisma-libavif": {"component": "ferrisma-libavif", "skip-github-release": true, "skip-changelog": true}
  }
}
```

Manifest, [`publishing/release-please/manifest-per-port.json`](../publishing/release-please/manifest-per-port.json):

```json
{
  "crates/ferrisma": "0.0.0",
  "crates/ferrisma-kernels": "0.0.0",
  "crates/ferrisma-avif-read": "0.0.0",
  "crates/ferrisma-avif-write": "0.0.0",
  "crates/ferrisma-jxl-decoder": "0.0.0",
  "crates/ferrisma-jxl-encoder": "0.0.0",
  "crates/libsharpyuv-core": "0.0.0",
  "codecs/libjpeg-turbo-rs": "0.0.0",
  "codecs/libwebp-rs": "0.0.0",
  "codecs/libjxl-rs": "0.0.0",
  "codecs/libavif-rs": "0.0.0",
  "codecs/libavif-rs/crates/aom": "0.0.0",
  "codecs/libavif-rs/crates/dav1d": "0.0.0",
  "codecs/libavif-rs/crates/sharpyuv": "0.0.0",
  "crates/ferrisma-libavif": "0.0.0"
}
```

`ferrisma-libavif` is internal but depends on the engine, so `cargo-workspace`
would bump it on its own (the rehearsal showed `0.0.1`, `0.0.2`); listing it
in the engine group with `skip-github-release` and `skip-changelog` keeps it on
the engine version without a tag or release. `ferrisma-xtask` has no
first-party dependency and is never touched in B.

What B implies:

- **Pins.** Requirements differ per component, and `cargo-workspace` rewrites
  each dependent's requirement and gives it a dependency-only patch release.
  Under 0.x, a minor bump of a port (0.1 → 0.2) is breaking for its dependents,
  and the plugin answers it with a patch release of the dependent (rehearsal:
  `libjxl-rs` 0.2.0, `ferrisma` 0.1.1). That is correct only because codec
  types never appear in `ferrisma`'s public signatures (ADR-0040 decision 5);
  `ferrisma-kernels` is public and stays in the engine group. Exact `=` pins
  would again need a post-step.
- **Publish order.** Unchanged (dependency order from
  [publishing](publishing.md)); the shared action skips every version already
  on crates.io, so the same fourteen-name list publishes only what the release
  changed.
- **Linked groups are commit-linked, not dependency-linked.** In the rehearsal,
  a `libwebp-rs` fix produced `ferrisma` 0.1.1 and `ferrisma-avif-read` 0.1.1
  but left `ferrisma-kernels` and the JPEG XL bridges at 0.1.0, and moved
  `libavif-rs` and `libavif-sharpyuv` to 0.1.1 while `libavif-aom` and
  `libavif-dav1d` stayed at 0.1.0. `linked-versions` aligns versions from the
  commit history; bumps that `cargo-workspace` adds for dependents are not
  propagated to the rest of the group (plugin order does not change this). The
  requirements stay consistent, but "one engine version" holds only for
  releases with an engine commit. ferrugo avoids this with
  `updateAllPackages: true`, which would also bump every port and defeat B.
- **Dev-dependencies count.** The plugin's dependency graph includes
  dev-dependencies: `libavif-sharpyuv` has a dev-dependency on `libwebp-rs`,
  so a WebP fix releases half of the AVIF family.
- **Changelogs.** One per component, in each package directory (ferrugo
  style); the engine group's dependency-only releases get "workspace
  dependencies were updated" entries.

## Known limits (measured)

Measured with the offline rehearsal below, release-please 17.6.0 (the version
release-please-action v5.0.0 locks), on 2026-11-13. "Findings" are first-party
requirements, package versions, or lockfile entries that disagree after a
proposed release.

| Limit | Evidence | Consequence |
| --- | --- | --- |
| The Rust updater (`CargoToml`) and the `cargo-workspace` graph match path dependencies by their **key**, not by `package = …`. | The manifests before `86f9e0df`: A 24 findings, B 6 findings, all on the eight renamed entries (`webp`, `jxl`×4, `sharpyuv`, `dav1d`, `aom`). The first release looked clean only because everything is already 0.1.0. | Key those dependencies by package name: done on `main` in `86f9e0df` (#248); the rehearsal has not been repeated against the changed manifests. The library target names (`webp`, `jxl`, `sharpyuv`, `dav1d`, `aom`) stayed, so no Rust source changed; `dep:webp`/`dep:jxl` in the engine's features became `dep:libwebp-rs`/`dep:libjxl-rs`. Before that, applied temporarily on 2026-11-13: `cargo metadata --offline --locked` passed and `cargo check --offline --locked -p ferrisma --no-default-features --features webp-read,webp-write,jpeg-xl-write,jpeg-xl-own-read` passed (AArch64; the AVIF crates were not built). With it, both configurations rehearse with 0 findings. |
| A path dependency without `version` is skipped. | release-please `src/updaters/rust/cargo-toml.ts`; ferromark rehearsal negative case. | Affects only `ferrisma-libavif → ferrisma` and dev-only `libavif-sharpyuv → libwebp-rs`, neither published. Every published requirement already has a `version`. |
| `version.workspace = true` cannot be updated. | ferromark ADR-0020; its rehearsal negative case. | Not used in Ferrisma; keep it that way. |
| A root package that is also a workspace root makes the `rust` strategy rewrite every member. | release-please `src/strategies/rust.ts`; B on today's layout. | Fine for A (that is A's mechanism). B needs a virtual root. |
| `"."` in `[workspace].members` | Rehearsal warning "member . declared but did not find Cargo.toml"; no effect on the result. | Harmless in the rehearsal; not verified against the GitHub API. Removing `"."` from `members` (Cargo includes the root package anyway) would make it moot. |
| `=` requirements are rewritten without `=`. | ferromark `docs/releasing.md`, `scripts/preserve-exact-transform-pin.mjs`. | No first-party `=` pins exist; adding one needs the same post-step. |
| Secondary lockfiles are not part of the strategy. | Three lockfiles carry first-party versions (see above). | Covered by `toml` extra files in both drafts; the rehearsal checks them. |
| Linked groups and dependency-only bumps (B). | See configuration B. | Accept, or move to A. |

## Pre-1.0 versions and the reserved 1.0

- **First release.** Land the activation with a `Release-As: 0.1.0` footer (A:
  on any commit; B: on a commit that touches every component's directory, or a
  temporary `release-as` per package removed afterwards). Rehearsed: all
  packages 0.1.0, including the two AVIF bridges now at 0.0.0.
- **0.x cycle.** `fix:` → 0.1.1, `feat:` → 0.2.0, breaking (`!` or
  `BREAKING CHANGE:`) → 0.2.0, never 1.0.0. Rehearsed; the negative control
  confirms that without `bump-minor-pre-major` a breaking change proposes
  1.0.0. If the owner prefers features as patches while the API moves, set
  `bump-patch-for-minor-pre-major: true` (ferriki did not).
- **1.0.0** comes only from an explicit `Release-As: 1.0.0` footer on the
  commit that completes the quality gate (D4–D6, D8). Nothing in the
  configuration can propose it on its own. A publication hold (project-infra:
  a committed JSON with `minimumMajor`, `publicationEnabled`, `reason`, read by
  the workflow) could additionally block any release until the owner lifts it;
  it is not in the draft.
- **Release candidates** are not needed for 0.x. If wanted before 1.0, use the
  ferromark recipe: `versioning: prerelease`, `prerelease-type: rc`,
  `prerelease: true` for the series, started with `Release-As: 1.0.0-rc.1` and
  removed by the commit carrying `Release-As: 1.0.0`.
- **Changelog sections.** ADR-0040 decision 6 asks for *Breaking*, *Added*,
  *Changed*, *Output changes*, and *Toolchain*. Release Please's defaults are
  "⚠ BREAKING CHANGES", "Features", "Bug Fixes", "Performance Improvements",
  and so on. `changelog-sections` can rename and hide types, but "Output
  changes" and "Toolchain" have no Conventional Commit type; they need either
  new types (and the commit-subject check to accept them) or curated notes
  above the generated entry. Owner decision; the drafts keep the defaults.

## Publishing (draft workflow)

[`publishing/release-please/publish.workflow.yml.draft`](../publishing/release-please/publish.workflow.yml.draft)
is the blueprint's skeleton for crates only (no npm, no native matrix):

1. `release-please` on push to `main`, action pinned to the peeled v5.0.0
   commit, `RELEASE_PLEASE_TOKEN` with `GITHUB_TOKEN` fallback.
2. `validate-release` resolves the release commit (the push SHA, or the tag on
   a manual retry) and checks that every tag at that commit names a
   publishable package at that version (`cargo metadata`, `jq`).
3. `publish-crates` checks out that commit, installs the pinned nightly from
   `rust-toolchain.toml`, packages and verifies the whole chain once
   (`cargo package --locked -p …`, the command from
   [publishing](publishing.md)), then calls project-infra `publish-crates` with
   the fourteen names in dependency order and Trusted Publishing. A commented
   `token:` line documents the one-time bootstrap for the very first publish.

It works unchanged for A and B, because the shared action skips versions that
already exist. It does not run the engine and codec test gates; the release
pull request's CI (which needs `RELEASE_PLEASE_TOKEN`) carries them, as in
ferromark. Not covered: license bundles and source identities as release
assets (publishing step 4 and 9), consumer verification outside the workspace
(publishing step 8), CLI binaries.

## The one exception to "push to main, no pull requests"

Ferrisma's working rule is to rebase onto `origin/main` and push directly,
without pull requests. Release Please works only through its release pull
request. Activating D10 therefore adds **one** exception: the bot's release
pull request (branch `release-please--branches--main…`, label
`autorelease: pending`). The owner reviews and merges it; merging is the
release decision. All other work keeps pushing to `main`. Two consequences:

- Every commit subject on `main` must be a valid Conventional Commit, because
  without squash merges each subject is release input; project-infra asks
  repositories that do not squash to check every non-merge subject against the
  configured types.
- Edits to the release branch by hand are overwritten on the next push to
  `main`; change the version with a `Release-As` footer instead.

## Rehearsal procedure (local, offline)

Nothing below needs GitHub or crates.io credentials.

1. Install the library pinned by the action, outside the repository:

   ```sh
   npm install --prefix "$TMPDIR/rp-17.6.0" release-please@17.6.0
   ```

2. Run the release proposals with the real Manifest, strategies, plugins, and
   updaters against this checkout and a synthetic history (first release with
   `Release-As: 0.1.0`, a port fix, a port feature, a breaking engine change,
   and for A the `bump-minor-pre-major` negative control):

   ```sh
   node publishing/release-please/rehearse.mjs "$TMPDIR/rp-17.6.0/node_modules" shared
   node publishing/release-please/rehearse.mjs "$TMPDIR/rp-17.6.0/node_modules" per-port
   # the same with the proposed dependency-key fix applied in memory
   node publishing/release-please/rehearse.mjs "$TMPDIR/rp-17.6.0/node_modules" shared --key-by-package
   node publishing/release-please/rehearse.mjs "$TMPDIR/rp-17.6.0/node_modules" per-port --key-by-package
   ```

   `per-port` always applies the virtual-root move in memory. Exit status 0
   means no finding. Results on 2026-11-13: without the key fix, A 24 and B 6
   findings (exit 1); with it, both clean (exit 0).

3. Validate the drafts against the Release Please schema (`schemas/config.json`
   and `schemas/manifest.json` at v17.6.0 and at `main`), for example with Ajv
   8. Result on 2026-11-13: all four files valid against both; a misspelled
   key is rejected (negative control).

4. After applying a real manifest change (dependency keys, versions,
   `publish`), check the result with Cargo: `cargo metadata --offline --locked`
   and the multi-package `cargo package` dry run from
   [publishing](publishing.md).

5. Repeat steps 1–3 whenever the action pin, the release-please version, the
   workspace members, or a dependency edge changes (ferromark ADR-0016
   contract).

Not rehearsable locally: Trusted Publishing, the `autorelease:` label
lifecycle, the GitHub tag and release creation, CI on the release pull request,
and the real `cargo publish` uploads.

## Blockers before a first 0.1.0

| # | Blocker | Issue |
| --- | --- | --- |
| 1 | The JPEG port's package name `libjpeg-turbo-rs` belongs to an unrelated crates.io crate; rename the package (both drafts, the workflow list, and the lockfile jsonpaths use today's name and must follow). | #207 (name pending, O5) |
| 2 | `libavif-aom` is 16.4 MiB compressed, above crates.io's 10 MiB limit: request a limit increase or split the platform translations (codec ADR). | #208 |
| 3 | AVIF packages need `overflow-checks = false` and `panic = "unwind"` that dependents do not inherit; exact `libc = "=0.2.191"` and `c2rust-bitfields = "=0.3.0"` pins. | #209 |
| 4 | Renamed first-party dependency keys break Release Please's version updates (measured above). Applied on `main` on 2026-11-13: the manifests key those dependencies by package name. The rehearsal has not been repeated against the changed manifests. | #248 |
| 5 | Every package is `publish = false`; first-publication versions (the AVIF bridges are 0.0.0) and the published set. | #137, [publishing](publishing.md) blockers 5 |
| 6 | License review gates for redistributing source crates (IJG terms, `simd_helpers` notice, independent review; per-crate license metadata). | #66, #136 |
| 7 | Configuration B only: move the engine package out of the workspace root. | none; decision with O3 |
| 8 | Repository setup outside Git: `RELEASE_PLEASE_TOKEN`, Trusted Publishing for each of the fourteen crates (after a one-time token bootstrap), crates.io team as co-owner, branch protection that lets the release pull request run CI. | #137 |
| 9 | Release artifacts beyond crates. The CLI has `--version` since 2026-11-13; it prints the build identity only when `FERRISMA_BUILD_ID` is set at compile time, and no release workflow sets it yet (not needed for a crates-only 0.1.0). | #237, #196 |

## Decisions the owner still has to make

1. **O3: one shared version (A) or engine group plus per-port versions (B).**
   Recommendation (best estimate): A for the 0.x series. It matches the
   existing root-package layout, rehearses clean once the dependency keys are
   fixed, gives one tag and one changelog, and makes ADR-0040 problem 7 moot;
   its cost is re-publishing unchanged ports. Revisit B when a port gets
   outside consumers who need independent versions; switching later keeps
   `ferrisma-v<version>` tags valid because both drafts use component tags.
2. The JPEG port's new name (#207) and the `libavif-aom` size path (#208).
3. Whether to key the renamed dependencies by package name (blocker 4), and
   whether to drop `"."` from `[workspace].members`.
4. `bump-patch-for-minor-pre-major` (features as 0.x minors or patches), and
   whether to flag 0.x GitHub releases as prereleases (`prerelease: true`).
5. Changelog sections versus ADR-0040 decision 6.
6. Whether to add a publication hold file in addition to the inert location.
7. The `bootstrap-sha` (activation commit) and whether to keep the action pin
   at the peeled commit (project-infra) or the tag object (ferromark, ferriki).
8. Approving the release-pull-request exception to the push-to-main rule.

## Activation (for later; not done)

1. Resolve the blockers above and record them in the owning ADRs.
2. Copy the chosen `config-*.json` to `release-please-config.json` and the
   matching manifest to `.release-please-manifest.json` at the repository root;
   set `bootstrap-sha`.
3. Rename `publish.workflow.yml.draft` to `.github/workflows/publish.yml`.
4. Set `publish = ["crates-io"]` on exactly the fourteen packages.
5. Rehearse (all steps above), then push with a `Release-As: 0.1.0` footer.

## Review when

O3 is decided; a package is added to or removed from the published set; a
package is renamed (#207) or split (#208); the action pin or the release-please
version changes; the organization's blueprint (project-infra) changes its
release guidance.
