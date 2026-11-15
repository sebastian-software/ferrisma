# ADR-0040: Stable 1.0 public contract, compatibility, versioning, and upgrade policy

- **Status:** Proposed
- **Date:** 2026-11-12
- **Last reviewed:** 2026-11-15
- **Supersedes:** None. Once accepted, it replaces decision 6 ("no stability promise") of [ADR-0017](0017-minimal-public-facade-and-cli.md) for the items it marks stable.
- **Superseded in part by:** [0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md) (when the version 1.0 may be used, and the release tooling)
- **Related:** [0010](0010-dual-licensing.md), [0017](0017-minimal-public-facade-and-cli.md), [0018](0018-optional-codec-and-cms-dependencies.md), [0020](0020-kernel-crate-and-isa-dispatch.md), [0024](0024-pluggable-encoders-and-libavif-rs-backend.md), [0026](0026-codecs-in-one-repository-and-workspace.md), [0034](0034-thumbnail-geometry.md), [0037](0037-selectable-workspace-avif-encoder.md), [publishing](../publishing.md), [release automation](../release-automation.md), [compatibility guide](../compatibility.md), tracker #196

## Context

The v1.0 tracker (#196) asks for a reviewed public API and CLI contract,
feature stability boundaries, error semantics, a compatibility and
versioning policy, and upgrade guidance before stability is promised.
ADR-0017 records a deliberately small facade, but its decision 6 says any
part may change without deprecation. Nothing functional is published yet:
crates.io and npm hold only `ferrisma` 0.0.0 placeholders (ADR-0038), and the
workspace members are at 0.1.0 except `ferrisma-avif-read`,
`ferrisma-avif-write` and `ferrisma-libavif` (0.0.0), with `publish = false`
everywhere.

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
  builds on stable Rust 1.98 (CI MSRV job). Since 2026-11-15 the AVIF features (`avif-read`,
  `avif-write`, `cli`) build on stable Rust 1.99 instead (libavif-rs ADR 0060).
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
| `TransformOptions::avif_depth` | field | Experimental (working decision 2026-11-15; replaces `AvifEncoder::Libavif { depth }`) | 8 or 10, default 10 |
| `LibavifBackend` | struct | Experimental (`avif-write`; stable Rust 1.99 since 2026-11-15, pinned nightly before) | ADR-0037; the built-in AVIF writer |
| `WorkingPrecision` | enum | `F32` stable; `F64Reference` experimental (verification only) | Non-exhaustive |
| `Output` | struct | Stable | Non-exhaustive; `format`, `bytes`, `width`, `height`, `report` |
| `ProcessingReport`, `SourceAccess` | struct, enum | Stable field names; values are observations | Non-exhaustive; new fields are additive. `decoder_backend` and `encoder_backend` strings are experimental values (2026-11-14, #217) |
| `StageTimings` | struct | Experimental values | Wall-clock observations, never a contract |
| `ColorSignal`, `ProcessingReport::color_signal` | enum, field | Experimental (working decision 2026-11-14, #212) | Non-exhaustive; reports the signal used, never a precedence |
| `KernelLevel`, `KernelLevelReason`, `TransformOptions::kernel_level_ceiling` | re-exported enums, field | Experimental (diagnostics) | Couples to `ferrisma-kernels`; see problem 7 |
| `ProcessingError`, `ProcessingErrorKind` | struct, enum | Stable | Kind is non-exhaustive; message text is not stable |
| `RefusalReason`, `ProcessingError::refusal_reason` | enum, method | Experimental (working decisions 2026-11-14 and 2026-11-15, #212) | Non-exhaustive closed vocabulary by refusal class (thirteen variants since 2026-11-15, `IccColorSpaceMismatch` added with gray input); `None` outside it |
| `ProcessingReport::discarded`, `Discarded`, `DiscardedSet` | field, enum, set | Experimental (working decision 2026-11-15, ADR-0042 D2) | Non-exhaustive classes of information left out of a correct picture; CLI `discarded=` |
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
| `jpeg-xl-read` | Stable as "JPEG XL input"; since 2026-11-15 the workspace libjxl-rs reader, the only one (ADR-0032) | Stable 1.98 |
| `jpeg-xl-write` (in `cli` since 2026-11-12, ADR-0039) | Experimental | Stable 1.98 |
| `avif-read`, `avif-write` | Experimental until they build on stable (`avif-write` was the stable rav1e capability until 2026-11-15); they build on stable 1.99 since 2026-11-15, the tier change is the owner's | Stable 1.99 (pinned nightly before 2026-11-15), native targets only |
| `cli` | Stable command line; inherits the 1.99 floor through `avif-read`/`avif-write` (the nightly requirement until 2026-11-15) | Stable 1.99 |

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
| `--avif-depth=8\|10` | Experimental |
| `--reference`, `--kernel-level=scalar\|neon\|avx2\|avx512` | Experimental (verification and diagnostics) |
| `.jxl` output, `--jxl-distance=`, `--jxl-lossless` (ADR-0039) | Experimental |
| Exit status 0 success, 2 usage (fewer than four positional arguments), 1 any other error (Rust's default `Error: …` on stderr) | Stable |
| Each flag at most once; mutually exclusive pairs refused with both names (owner decision 2026-11-15) | Stable |
| `report:` key=value line on stdout | Stable keys; new keys are additive; integer values except `precision`, `kernel_level`, `encoder`, and `decoder` (appended 2026-11-14, #217) |
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
6. **No structured error details.** `ProcessingError` exposes `kind()`,
   the message, and `refusal_reason()` (#212): since 2026-11-14 for four
   refusals, since 2026-11-15 for every documented refusal class of the
   [supported matrix](../supported-matrix.md) (twelve reasons). Additive accessors (which limit, requested and allowed
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
13. **CLI help.** There is no `--help`; usage is printed only for too few
    arguments, and an unknown flag exits 1. `--version` (and `-V`) exists
    since 2026-11-13 (#237, [compatibility guide](../compatibility.md)) but
    is not yet assigned to a tier. Recommendation: add `--help` and assign
    both flags a tier before 1.0.
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
- 2026-11-13 — Corrections, no decision change: the version statement in the
  context now names the three members at 0.0.0 (`ferrisma-avif-read`,
  `ferrisma-avif-write`, `ferrisma-libavif`; their manifests), and a
  duplicated "Related" line is removed. The default switch of 2026-11-12 is
  confirmed by owner decision D1 ([ADR-0041](0041-v1-quality-gate-equivalence-with-the-ported-original.md)
  section 7); the tier question it raised for `cli` stays open with this
  record's other recommendations.
- 2026-11-14 — Decoder backend in the report (issue #217; design in
  [ADR-0017](0017-minimal-public-facade-and-cli.md), amendment of the same
  date): `ProcessingReport::decoder_backend: &'static str` is an additive
  field of the non-exhaustive report (decision 2), and the `report:` line
  appends `decoder=`. Working decision 2026-11-14 (delegated, revisable by
  the owner): its string values share the experimental tier of the
  `encoder_backend` strings; the field name follows the report's stable
  field-name rule. Correction, no decision change: problem 13 said there is
  no `--version`; the flag exists since 2026-11-13 (`src/cli.rs`, #237), so
  the problem now covers `--help` and the tier of both flags.
- 2026-11-14 — Color signal and refusal reasons (issue #212; design in
  [ADR-0017](0017-minimal-public-facade-and-cli.md), amendment of the same
  date): `ColorSignal` with `ProcessingReport::color_signal`, and
  `RefusalReason` with `ProcessingError::refusal_reason()`, both additive and
  `#[non_exhaustive]` (decision 2: new variants are not breaking). The CLI
  `report:` line appends `color_signal=`. Working decision 2026-11-14
  (delegated, revisable by the owner): both enter the inventory as
  experimental, and a reason, once documented, keeps its meaning; making
  them stable is the owner's choice. Problem 6 is narrowed accordingly; which
  limit was exceeded and `source()` remain open.
- 2026-11-15 — Refusal reasons for every documented refusal class (issue
  #212; design in [ADR-0017](0017-minimal-public-facade-and-cli.md),
  amendment of the same date): seven additive `RefusalReason` variants
  (`HdrTransfer`, `UnsupportedColorEncoding`, `RenderedIccMissing`,
  `ContradictoryColorSignals`, `UnsupportedLayout`, `AlphaAssociation`,
  `GeometricTransform`), allowed by decision 2. No `ProcessingErrorKind` and
  no message text changed. Inventory row and problem 6 updated. Working
  decision 2026-11-15 (delegated, revisable by the owner): the enum stays
  experimental; the names and class boundaries are the owner's to confirm.
  Not decided here, and described as today's behavior: whether grayscale or
  premultiplied JPEG XL should be accepted, the strict color policy (#222),
  and the kind of an unknown essential AVIF property (`MalformedInput`,
  #67), which keeps no reason.
- 2026-11-15 — Correction of the entry above (issue #67, approved by the
  owner through the v1.0 coordinator's brief): an unknown essential AVIF
  property is `UnsupportedInput` with the additive
  `RefusalReason::UnsupportedEssentialProperty` (twelve variants), no
  longer `MalformedInput`; a malformed property box stays `MalformedInput`.
  A changed kind for one input class, allowed in 0.x and recorded in
  [ADR-0017](0017-minimal-public-facade-and-cli.md). Working decision
  2026-11-15 (delegated, revisable by the owner): the variant name.
- 2026-11-15 — `ProcessingReport::discarded` (`Discarded`, `DiscardedSet`)
  and the CLI key `discarded=` enter the inventory as experimental
  (additive, decision 2); the rule is [ADR-0042](0042-no-silent-loss-refuse-or-report.md)
  D2, the design the [ADR-0017](0017-minimal-public-facade-and-cli.md)
  amendment of the same date.
- 2026-11-15 — Removed items (owner decision, ADR-0032; pre-1.0 breaking
  change of the inventory): `JpegXlDecoder` with `JxlOxide` and
  `LibjxlRs`, `TransformOptions::jxl_decoder`, the CLI flag
  `--jxl-decoder=`, the `decoder_backend` value `"jxl-oxide"`, and the
  feature `jpeg-xl-own-read`. `jpeg-xl-read` keeps its tier ("JPEG XL
  input") and now compiles the workspace reader, which was
  `jpeg-xl-own-read` (working decision 2026-11-15, delegated, revisable by
  the owner). A build that enabled `jpeg-xl-read` keeps JPEG XL input; one
  that enabled `jpeg-xl-own-read` must rename it; code that set
  `jxl_decoder` must drop the field.
- 2026-11-15 — Removed items (owner decision, ADR-0037; pre-1.0 breaking
  change of the inventory): `AvifEncoder` with `Rav1e` (stable) and
  `Libavif { depth }`, `TransformOptions::avif_encoder`, the CLI flag
  `--avif-encoder=`, the `encoder_backend` value `"rav1e"`, the feature
  `avif-own-write`, and the stable feature `avif-write` as the rav1e
  capability. Added, experimental: `TransformOptions::avif_depth`. The name
  `avif-write` now compiles the workspace AOM writer, which needs the pinned
  nightly, so it moves to the experimental tier until it builds on stable
  (working decisions 2026-11-15, delegated, revisable by the owner). A
  stable-toolchain consumer of the old `avif-write` loses AVIF output.
- 2026-11-15 — Owner decision on repeated and contradictory command-line
  flags (ADR-0042): every flag may be given once, and a pair of mutually
  exclusive flags (`--assume-srgb`/`--use-rendered-icc`,
  `--jxl-lossless`/`--jxl-distance=`, `--webp-lossless` or `--webp-level=`
  with lossy-only settings) is refused with both names, exit status 1,
  before any input is read. A script that relied on "the last one wins"
  breaks; the old result was a silent choice between two instructions.
  Added to the command-line inventory as stable.
