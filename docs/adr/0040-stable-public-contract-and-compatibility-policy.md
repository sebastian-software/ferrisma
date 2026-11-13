# ADR-0040: Stable 1.0 public contract, compatibility, versioning, and upgrade policy

- **Status:** Proposed
- **Date:** 2026-11-12
- **Last reviewed:** 2026-11-13
- **Supersedes:** None. Once accepted, it replaces decision 6 ("no stability promise") of [ADR-0017](0017-minimal-public-facade-and-cli.md) for the items it marks stable.
- **Superseded in part by:** [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) (when the version 1.0 may be used, and the release tooling)
- **Related:** [0010](0010-dual-licensing.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [0034](0034-thumbnail-geometry.md), [0037](0037-selectable-workspace-avif-encoder.md), [publishing](../publishing.md), [compatibility guide](../compatibility.md), tracker #196
- **Related:** [0010](0010-dual-licensing.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [0034](0034-thumbnail-geometry.md), [0037](0037-selectable-workspace-avif-encoder.md), [publishing](../publishing.md), [release automation](../release-automation.md), [compatibility guide](../compatibility.md), tracker #196

## Context

The v1.0 tracker (#196) asks for a reviewed public API and CLI contract,
feature stability boundaries, error semantics, a compatibility and
versioning policy, and upgrade guidance before stability is promised.
ADR-0017 records a deliberately small facade, but its decision 6 says any
part may change without deprecation. Nothing functional is published yet:
crates.io and npm hold only `ferrisma` 0.0.0 placeholders (ADR-0038), and the
workspace version is 0.1.0 with `publish = false` everywhere.

Verified facts on 2026-11-12 (branch on top of `90b638dd`):

- The library exposes two modules, `pipeline` and `cli`. Graph, planner,
  executor, codec adapters, color contract, and memory types are private.
  The engine crate forbids `unsafe`; codec types never appear in public
  signatures.
- `ferrisma::pipeline::{KernelLevel, KernelLevelReason}` are re-exports of
  `ferrisma_kernels::{IsaLevel, LevelReason}`, so `ferrisma-kernels` is a
  public dependency.
- `examples/consumer.rs` exercises the facade outside the test suite,
  including every documented failure category and resource limit; its
  `--self-check` passes (10 scenarios) and runs in CI.
- The `avif-read`, `avif-own-write`, and `cli` features require the pinned
  nightly and native macOS ARM64 or GNU Linux x86-64; everything else
  builds on stable Rust 1.98 (CI MSRV job).
- On this branch, every public facade item is documented
  (`#![warn(missing_docs)]`), and the growable enums and output-only structs
  are `#[non_exhaustive]` (see the inventory below).

## Public surface inventory

### Rust API (`ferrisma::pipeline`)

| Item | Kind | Proposed 1.0 tier | Notes |
| --- | --- | --- | --- |
| `transcode`, `transcode_resized` | fn | Stable | Exact and aspect-aware geometry (ADR-0034) |
| `inspect`, `InspectionLimits`, `ImageMetadata` | fn, struct, struct | Stable | JPEG and WebP only; `ImageMetadata` is non-exhaustive |
| `TransformOptions` | struct, public fields, `Copy`, `Default` | Stable, see problems 1–3 | Field docs state each field's scale and limit |
| `Crop` | struct, public fields | Stable | Deliberately exhaustive plain rectangle |
| `ResizeOptions`, `ResizeFit` | struct, enum | Stable | `ResizeFit` non-exhaustive |
| `ResizeFilter` (`Area`, `Lanczos3`) | enum | Stable | Non-exhaustive |
| `InputProfilePolicy` | enum | Stable | Non-exhaustive; `Strict` default |
| `ImageFormat`, `OutputFormat` | enum | Stable | Non-exhaustive |
| `WebpOptions` | enum | Stable | Non-exhaustive |
| `JpegXlOptions` (`Lossy { distance }`, `Lossless { effort }`) | enum | Experimental (ADR-0039); default lossy d1.0 pending owner confirmation | Non-exhaustive; `PartialEq` only |
| `ChromaSubsampling` | enum | Stable | Non-exhaustive |
| `AvifEncoder::Rav1e` | enum variant | Stable | Non-exhaustive enum |
| `AvifEncoder::Libavif`, `LibavifBackend` | variant, struct | Experimental (`avif-own-write`, nightly) | ADR-0037 |
| `JpegXlDecoder` | enum | `JxlOxide` stable as "default JPEG XL reader"; `LibjxlRs` experimental (`jpeg-xl-own-read`) | Non-exhaustive |
| `WorkingPrecision` | enum | `F32` stable; `F64Reference` experimental (verification only) | Non-exhaustive |
| `Output` | struct | Stable | Non-exhaustive; `format`, `bytes`, `width`, `height`, `report` |
| `ProcessingReport`, `SourceAccess` | struct, enum | Stable field names; values are observations | Non-exhaustive; new fields are additive |
| `StageTimings` | struct | Experimental values | Wall-clock observations, never a contract |
| `KernelLevel`, `KernelLevelReason`, `TransformOptions::kernel_level_ceiling` | re-exported enums, field | Experimental (diagnostics) | Couples to `ferrisma-kernels`; see problem 7 |
| `ProcessingError`, `ProcessingErrorKind` | struct, enum | Stable | Kind is non-exhaustive; message text is not stable |
| `Engine`, `BatchItem` | struct, struct | Stable | ADR-0022 |
| `transcode_with`, `transcode_resized_with`, `Engine::transcode_batch_with` | fn | Experimental | ADR-0024 backend extension point |
| `EncoderBackend`, `RowEncoder`, `EncodeRequest`, `EncodedImage`, `EncodeError`, `EncodeErrorKind` | traits, structs, enum | Experimental | Implementors break if a required method is added; the structs and kind are already non-exhaustive |

### Rust API (`ferrisma::cli`)

`cli::run(args, backend)` is public so adapter crates can ship the same
command. Proposed tier: experimental and `#[doc(hidden)]` candidate; the
binary's command line, not this function, is the stable CLI contract.

### Cargo features

| Feature | Proposed 1.0 tier | Toolchain |
| --- | --- | --- |
| `jpeg-read`, `webp-read`, `webp-write`, `icc` | Stable | Stable 1.98 |
| `avif-write` (rav1e) | Stable as "AVIF output"; the backend behind it may change per ADR-0025 | Stable 1.98; NASM on x86-64 |
| `jpeg-xl-read` | Stable as "JPEG XL input"; backend may change per ADR-0025 (#55) | Stable 1.98 |
| `jpeg-xl-own-read` | Experimental | Stable 1.98 |
| `jpeg-xl-write` (in `cli` since 2026-11-12, ADR-0039) | Experimental | Stable 1.98 |
| `avif-read`, `avif-own-write` | Experimental until they build on stable | Pinned nightly, native targets only |
| `cli` | Stable command line; inherits the nightly requirement through `avif-read`/`avif-own-write` | Pinned nightly |

### Command line (`ferrisma` binary, `cli` feature)

| Element | Proposed tier |
| --- | --- |
| Positional `INPUT OUTPUT WIDTH HEIGHT`; input format from the extension (`jpg`, `jpeg`, `webp`, `jxl`, `avif`); output from `.avif` or `.webp` | Stable |
| `--assume-srgb`, `--use-rendered-icc` | Stable |
| `--quality=`, `--speed=`, `--chroma=444\|420`, `--webp-lossless`, `--webp-level=` | Stable |
| `--crop=x,y,w,h` | Stable |
| `--max-input-mb=`, `--max-input-mpx=`, `--budget-mb=`, `--max-output-mb=` | Stable |
| `--transform-threads=`, `--encoder-threads=` | Stable |
| `--area` | Stable, see problem 15 |
| `--avif-encoder=rav1e\|libavif`, `--avif-depth=8\|10`, `--jxl-decoder=jxl-oxide\|libjxl-rs` | Experimental (selects experimental backends) |
| `--reference`, `--kernel-level=scalar\|neon\|avx2\|avx512` | Experimental (verification and diagnostics) |
| `.jxl` output, `--jxl-distance=`, `--jxl-lossless` (ADR-0039) | Experimental |
| Exit status 0 success, 2 usage (fewer than four positional arguments), 1 any other error (Rust's default `Error: …` on stderr) | Stable |
| `report:` key=value line on stdout | Stable keys; new keys are additive; integer values |
| Human-readable `Encoded …`, `Timings: …` lines and the stderr memory note | Not stable |

## Decision

Proposed, pending maintainer review:

1. **Tiers.** Items marked stable above are covered by SemVer from 1.0.0 on.
   Experimental items are documented as such in rustdoc and in the
   [compatibility guide](../compatibility.md), may change or disappear in any
   minor release, and must not be required to use a stable feature. Private
   items have no compatibility promise.
2. **Semantic versioning** follows Cargo's SemVer rules for every published
   package. Breaking changes for `ferrisma` include: removing or renaming a
   stable item, feature, or CLI flag; changing a signature; adding a public
   field to a struct that is not `#[non_exhaustive]`; adding a required method
   to a stable trait; changing a default that changes output semantics
   (dimensions, orientation, color meaning, alpha, the strict color policy);
   or making a stable feature require a different toolchain. Not breaking:
   new variants of non-exhaustive enums (including new
   `ProcessingErrorKind` categories), new report fields, new optional
   features, error message text, timings, memory observations, and changed
   encoded bytes at the same documented semantics (encoder improvements).
3. **Output compatibility.** Ferrisma promises the documented meaning of the
   output (format, dimensions, color, alpha, orientation), not byte-identical
   files across versions. Byte-level changes are listed in the changelog.
4. **Rust version policy.** The stable feature set declares
   `rust-version` and keeps a CI job at that version. Raising it is allowed
   in a minor release, never in a patch release, and is announced in the
   changelog. Nightly-only experimental features name the exact pinned
   nightly in the release notes; changing that pin is a minor-release change.
5. **Dependencies.** Codec and CMS crates stay out of public signatures
   (ADR-0017), so their major versions are internal. `ferrisma-kernels` is a
   public dependency while `KernelLevel` is re-exported; a breaking change to
   those two types is a breaking change of `ferrisma` (problem 7).
6. **Deprecation and upgrade guidance.** Stable items are deprecated with
   `#[deprecated(since, note)]` for at least one minor release before removal
   in the next major release. Each release has a changelog entry with
   *Breaking*, *Added*, *Changed*, *Output changes*, and *Toolchain* sections;
   each major release adds an upgrade guide to the compatibility document.
7. **Applied now (uncontroversial):** documentation for every public item and
   `#[non_exhaustive]` on the growable enums (`ImageFormat`, `OutputFormat`,
   `WebpOptions`, `AvifEncoder`, `JpegXlDecoder`, `InputProfilePolicy`,
   `WorkingPrecision`, `ChromaSubsampling`, `SourceAccess`, `ResizeFit`,
   `ResizeFilter`, `KernelLevel`, `KernelLevelReason`) and the output-only
   structs (`Output`, `ProcessingReport`, `StageTimings`, `ImageMetadata`).
   `ProcessingErrorKind`, `EncodeErrorKind`, `EncodeRequest`, and
   `EncodedImage` already were. Consumers that matched these enums
   exhaustively need a wildcard arm; nothing in the workspace did.

## Problems to resolve before 1.0 (recommendations, not applied)

These change signatures or behavior and therefore need a maintainer
decision. Items 1, 2, 7, and 10 should be settled before 1.0.0 because they
cannot be fixed later without a major release.

1. **Input option structs cannot grow.** `TransformOptions`,
   `ResizeOptions`, `InspectionLimits`, `BatchItem`, and `LibavifBackend` have
   public fields and no `#[non_exhaustive]`, and the documented usage is a
   struct literal with `..TransformOptions::default()`. Adding any option is
   then a breaking change. Making them non-exhaustive forbids that literal
   (including the functional-update form) outside the crate; callers would
   use `let mut o = TransformOptions::default(); o.quality = 80.0;` or a
   builder. Recommendation: mark them `#[non_exhaustive]` before 1.0 and
   update the examples and documentation in the same change.
2. **`TransformOptions` mixes formats.** `quality`, `speed`,
   `chroma_subsampling`, and `avif_encoder` apply only to AVIF, while WebP uses
   `webp`; `quality`/`speed` change scale with the selected AVIF encoder.
   Recommendation: an `AvifOptions` value parallel to `WebpOptions`, or an
   output enum carrying per-format settings.
3. **Two geometry entry points.** `output_width`/`output_height` are ignored
   by `transcode_resized`. Recommendation: one geometry enum (exact or
   `ResizeOptions`) inside the options, or keep both functions and document
   the ignored fields (done in rustdoc).
4. **Backend variants double the API.** Each entry point has a `_with`
   twin. Recommendation: keep the twins experimental for 1.0, or fold the
   backend into an options value later.
5. **Output-limit errors are misreported.** Exceeding `max_output_bytes`
   returns `ResourceExhausted`, and the WebP writer's message says "managed
   buffer needs 12 bytes with 64 bytes available", which describes the
   pixel budget. Recommendation: an additive `OutputLimitExceeded` kind and
   an accurate message.
6. **No structured error details.** `ProcessingError` exposes only `kind()`
   and the message. Additive accessors (which limit, requested and allowed
   values) and `source()` can follow after 1.0.
7. **`KernelLevel` couples SemVer to `ferrisma-kernels`.** Recommendation:
   define the two diagnostic enums in `ferrisma` and map them, or keep them
   experimental and version both crates together.
8. **Raw CICP.** `ImageMetadata::rendered_cicp` is `Option<[u8; 4]>`.
   Recommendation: a named `Cicp` struct with primaries, transfer, matrix,
   and full-range fields.
9. **Thread counts use `0` as "automatic".** `transform_threads`,
   `encoder_threads`, and `Engine::new` overload `usize`. Recommendation:
   `Option<NonZeroUsize>` or a small `Threads` enum.
10. **`Copy` on `TransformOptions`.** Any future non-`Copy` option (for
    example a caller-supplied ICC profile) would break it. Recommendation:
    drop `Copy` before 1.0 if such options are plausible.
11. **`AvifEncoder::Libavif { depth }` and `LibavifBackend { depth }`
    duplicate the same configuration.** Recommendation: one type.
12. **`cli::run` is a public library function** returning
    `Box<dyn Error>`. Recommendation: `#[doc(hidden)]` or move it to the
    binary's crate if adapters no longer need it.
13. **CLI help and version.** There is no `--help` or `--version`; usage is
    printed only for too few arguments, and an unknown flag exits 1.
    Recommendation: add both before 1.0.
14. **CLI units differ.** `--max-input-mb`, `--budget-mb`, and
    `--max-output-mb` use MiB, while `--max-input-mpx` uses 10^6 pixels.
    Recommendation: document the units (now in the compatibility guide) or
    rename with explicit units.
15. **CLI filter and fit flags.** `--area` is a lone switch, the CLI cannot
    express `ResizeFit`, and `--kernel-level` lacks `avx512` (added 2026-11-13
    as the AVX-512 opt-in, ADR-0020 decision D6).
    Recommendation: `--filter=area|lanczos3` and `--fit=inside|cover|fill`
    before 1.0, keeping `--area` as a deprecated alias.

## Consequences

+ Callers can rely on a documented subset, and maintainers know which
  changes need a major release.
+ Non-exhaustive enums and output structs let Ferrisma add formats, policies,
  error kinds, and report fields in minor releases.
- Exhaustive matches on the affected enums now need a wildcard arm, and
  output structs can no longer be constructed outside the crate (no
  workspace code did either).
- Until problems 1, 2, 7, and 10 are decided, a 1.0.0 release would freeze
  shapes that the review identifies as hard to evolve.
- Nightly-only features stay experimental, so the stable promise covers
  fewer formats than the full CLI.

## Alternatives considered

- **Declare the whole current surface stable.** Freezes experimental
  backends, diagnostic types, and option shapes with known problems.
- **Stay on 0.x indefinitely.** Honest about churn, but gives users no
  upgrade rules; Cargo already treats 0.x minor versions as breaking, which
  this policy keeps until the maintainer accepts 1.0.
- **Separate `ferrisma-cli` crate now.** Cleaner library surface, but ADR-0008
  asks for a demonstrated need; deferring keeps the published chain shorter.
- **Builder-only API.** Avoids public fields entirely, at the cost of more
  surface; problem 1 lists it as one option.

## Validation

- `cargo doc` without missing-documentation warnings for the engine crate
  (`#![warn(missing_docs)]`, denied in CI through `-D warnings`).
- `examples/consumer.rs --self-check` in CI asserts the stable failure
  categories and limits through the public API.
- Before acceptance: build a consumer against the packaged crates outside
  the workspace (see [publishing](../publishing.md)), and run a SemVer checker
  such as `cargo-semver-checks` against the previous release for every
  release after 1.0.0.

## Review when

The maintainer accepts or rejects the recommendations above; the AVIF
features build on stable Rust; a new output format, color policy, or backend
is added; the first functional crates.io release is prepared; or a consumer
reports that a stable item cannot express its use case.

## Amendment log

- 2026-11-12 — Proposed with the surface inventory, tiers, SemVer, Rust
  version, deprecation and upgrade policy, the applied `#[non_exhaustive]` and
  documentation changes, and fifteen recommendations.
- 2026-11-12 — Default switch interaction (ADR-0032, ADR-0037, pending owner
  confirmation): `JpegXlDecoder::default()` is now `LibjxlRs` whenever
  `jpeg-xl-own-read` is compiled in (including `cli`), and
  `AvifEncoder::default()` is `Libavif { depth: 10 }` whenever
  `avif-own-write` is compiled in (including `cli`). The inventory above still
  lists those variants and features as experimental and `JxlOxide`/`Rav1e` as
  stable, so the stable `cli` feature now runs experimental backends by
  default, and an unchanged AVIF `quality` value means a different encoder
  scale. Before 1.0 the maintainer needs to either promote the two workspace
  backends (with their nightly/native-target boundary) or state that the
  stable promise for `cli` covers the capability, not the backend, as it
  already does for `jpeg-xl-read` and `avif-write`. Under decision 2, a
  default change that changes output semantics is breaking after 1.0; this
  change happens before any functional release.
- 2026-11-13 — Owner decision D10, recorded in [ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) section 8: pre-releases are published from `main` as 0.x; the version **1.0 is reserved** for the state that meets ADR-0041's quality gate completely (all four ports, AArch64 and x86-64, scalar and the v1.0 SIMD levels). Releases use Release Please as in the organization's other packages (`sebastian-software/ferromark` ADR-0020, `sebastian-software/ferrugo`). Whether all crates share one version is open (ADR-0041 O3). This adds a release condition; it does not accept this ADR's proposed tiers or decide its fifteen recommendations, which remain pending maintainer review. Under decision 2, 0.x minor releases may break, as Cargo treats them. The alternative "stay on 0.x indefinitely" is replaced by "stay on 0.x until the gate is met".
- 2026-11-13 — Release automation (owner decision D10: 0.x pre-releases from
  `main`, 1.0 reserved for the state that meets the quality gate, Release
  Please as in the organization's other packages). The inert draft and its
  evidence are in [release automation](../release-automation.md): the
  organization conventions with sources, two configurations (one shared
  version, or engine group plus per-port versions; open question O3), a local
  rehearsal with release-please 17.6.0, and the blockers before a first 0.1.0.
  Relevant here: `bump-minor-pre-major` keeps breaking changes in 0.x minors,
  so 1.0.0 only comes from an explicit `Release-As: 1.0.0`; Release Please's
  default changelog sections do not match decision 6's *Output changes* and
  *Toolchain* sections without extra commit types or curated notes; and with a
  shared version problem 7 disappears. Nothing was activated or published.
- 2026-11-13 — ADR-0020 decision D6: `kernel_level_ceiling: None` now stops
  at AVX2 on x86-64 and NEON on AArch64; `KernelLevelReason` gains
  `DefaultCeiling` (the enum is `#[non_exhaustive]`), and
  `--kernel-level=avx512` is the opt-in. Both stay experimental.
