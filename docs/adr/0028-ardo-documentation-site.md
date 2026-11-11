# ADR-0028: Present the pipeline and codec workspace through an Ardo site

- **Status:** Accepted
- **Date:** 2026-11-10
- **Last reviewed:** 2026-11-11
- **Supersedes:** None
- **Related:** [0026](0026-codecs-in-one-repository-and-workspace.md), [0027](0027-tiered-validation-and-rust-test-orchestration.md)

## Context

Consolidation made the README responsible for explaining an engine, four codec
ports, their integration, and multiple validation tiers. Historical experiments
and port progress were easy to mistake for current pipeline capabilities. The
maintainer requested a short README, a coherent status overview, and a website
using Ardo and the Ferramenta family's theming, with Ferromark and Ferroni as
references.

Both reference sites use Ardo and the Monografie components/styles from
`ferramenta-family`. The inspected family revision does not register Ferrixel's
identity or provide a Ferrixel instrument image. Reusing another engine's name
or asset would misrepresent the product.

## Decision

Keep the README as a compact repository entry. The
[status overview](../status.md) owns current capabilities and integration gates;
the [documentation guide](../README.md) maps the deeper sources. Port-specific
contracts, provenance, licenses, and historical evidence stay with their owners.

Build a static Ardo site in `homepage/`, using the shared Monografie theme and
components. The homepage explains the pipeline and links to structured guides
and codec pages. A TypeScript build step derives those pages from the canonical
repository Markdown and translates links for the website. Do not maintain a
second editable status register or copy benchmark figures into marketing claims
without their measurement context.

Use narrow site-local identity markup and an original Ferrixel instrument until
the family package supports Ferrixel. The Blender scene uses the pinned shared
studio and records its authoring sources; only the optimized render is needed
at website build time. Typography, spacing, color tokens, reading styles,
and general components come from the pinned shared package. The local layer
must not impersonate another registered product or fork the entire theme.

Website checks run independently of codec oracle suites. Website dependencies
stay in a private pnpm package and do not alter the Cargo dependency graph.
Build artifacts and local previews make the result reviewable; production
hosting and a custom domain remain separate configuration choices.

## Consequences

- Readers can distinguish working engine paths, standalone port coverage, and
  unfulfilled acceptance gates without reconstructing history.
- Website and repository guides share content and change together. The build
  must verify link translation and fail when a canonical source is missing.
- The repository gains a TypeScript website toolchain and its maintenance cost.
  Rust contributors do not need it for engine or codec checks.
- Local identity markup is a small temporary duplication; family registration
  should remove it when shared components can represent Ferrixel correctly.

## Alternatives considered

- A longer README: keeps one file but makes the existing navigation problem
  worse as codec evidence grows.
- Independent website prose for every technical page: gives editorial freedom
  but creates competing status and command references.
- A different framework or a copied family stylesheet: diverges from the
  maintainer's chosen stack and duplicates shared theme maintenance.
- Registering a fake product identity to reuse another instrument: rejected
  because the rendered identity would describe another engine.

## Validation

Compare the build with the current Cargo features, port guides, and acceptance
records. Check generated links, static routes, search, type checking, and the
production build. Inspect the homepage and a documentation page on desktop and
mobile, including keyboard navigation. Claims must retain the project's
experimental status and the separation between ports and active backends.

The reference source inspection establishes stack/theme compatibility, not
Ferrixel's rendered result; build and browser evidence are recorded separately.

## Review when

- The family registry and shared shell support Ferrixel's identity.
- Canonical Markdown cannot express a required website interaction or source
  links stop surviving generation.
- A release changes the experimental status, codec selection, or claims.
- Hosting requires a different base path, domain, or privacy boundary.

## Amendment log

- 2026-11-10 — Accepted the maintainer's Ardo and shared-theme direction; kept
  technical Markdown canonical and deployment separate from the site build.
- 2026-11-10 — The first homepage's short passages and abstract diagram were
  too sparse beside the reference sites. Added an original Blender instrument,
  source-backed pipeline and color illustrations, usage examples, richer codec
  and validation explanations, and canonical color, development and acceptance
  pages. The original diagram-only identity is replaced; family registration
  remains open and no pipeline capability is added by this presentation work.
- 2026-11-11 — Replaced the registration-bench instrument with an image
  railway: wagons on a gently curved track pass a crop guillotine, a spindle
  press and a pixel loupe, none of which blocks the rails. The maintainer chose
  the station sequence from three studies, rejected a straight bench whose
  press could not be passed, and asked for softer curves and less detail than
  a right-angled first track. The pinned family studio, file path and provenance format are
  unchanged; only the illustration and its alt text change. A site-local
  Blender mark (stepped pixel mountain, copper sun) built with the pinned family
  mark helpers replaces the placeholder favicon and now leads the header.
