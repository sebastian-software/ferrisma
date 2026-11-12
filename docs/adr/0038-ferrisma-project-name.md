# ADR-0038: Adopt Ferrisma as the project name

- **Status:** Accepted
- **Date:** 2026-11-11
- **Last reviewed:** 2026-11-12
- **Supersedes:** None
- **Related:** [0028](0028-ardo-documentation-site.md)

## Context

Ferrixel was the project's working name. The maintainer found it less appealing
with repeated use and wanted a shorter name that fits the Ferramenta family,
including Ferroni and Ferrocat. The desired associations are shimmer, light,
and premium optical quality. Retaining the recognizable `Ferr-` prefix matters
more than spelling out pixels or an image operation.

## Decision

Adopt **Ferrisma** as the project name, with **ferrisma** as its lowercase
spelling. The intended associations are a prism, light refraction, and color.

On November 11, 2026, the maintainer selected Ferrisma and reported reserving
**ferrisma.dev** as the project domain.

On November 12, 2026, the maintainer confirmed publication of both `ferrisma`
placeholder packages at version `0.0.0` and requested the complete technical
rename. The engine crate and CLI use `ferrisma`; first-party adapters, kernels,
tooling, examples, CI, fuzzing, and website assets use the same name and prefix.
The website uses `https://ferrisma.dev` with the root base path `/`; its static
build includes the matching `CNAME`.

The GitHub repository is `sebastian-software/ferrisma`, its website field is
`https://ferrisma.dev`, and the local `origin` points to the renamed repository.

The [published npm and crates.io stubs](../../publishing/ferrisma/README.md)
retain their original source and metadata as release artifacts. They expose
no image processing API. Their archived source URLs, dated benchmark evidence,
and render provenance may retain the former name. The Rust stub's empty
workspace declaration continues to isolate that artifact from the engine and
codec dependency graph. The functional engine still has its existing release
gates and `publish = false`.

## Consequences

- Future identity work has an agreed name and domain, with a light and color
  association that fits the maintainer's desired character.
- Package names, Rust imports, commands, and source paths change together.
  Existing source consumers must adopt `ferrisma` and its adapter crate names.
- Historical measurements and the published stubs remain identifiable by
  their original names and hashes. A naming change does not revalidate their
  performance or release readiness.

## Alternatives considered

- **Ferrixel:** recognizable image reference, but the maintainer disliked the
  name increasingly with use.
- **Ferriscope:** appealing optical instrument association, but the maintainer
  reported that `ferriscope.dev` appeared unavailable and preferred a shorter
  name.
- **Ferricut:** rejected because its sound and spelling are too close to
  Ferrocat.
- **Ferrut:** compact and tool-like; Ferrisma better matches the subsequent
  preference for shimmer and premium optical character.

## Validation

The evidence is the maintainer's explicit selection and reported domain
reservation on November 11, 2026, followed by confirmation of both placeholder
publications on November 12. The associations above are naming judgments, not
measured audience responses. Domain configuration in the source does not
establish a deployed website or completed DNS setup.

Verify the technical rename with Cargo metadata, the existing engine and
adapter checks, CLI smoke tests, website tests/type checking/build, and a scan
for obsolete active identifiers. Preserve published artifact contents, codec
reference inputs, generated AVIF hashes, and recorded benchmark hashes.

Local validation on November 12, 2026 (macOS ARM64):

- Default formatting, tests, and strict Clippy passed. The scoped all-feature
  engine, kernel, adapter, and tooling run passed 211 tests and strict Clippy
  with `--no-deps`; the imported AVIF dependencies still emit their existing
  warnings.
- Website tests, type checking, and production build passed: 13 prerendered
  pages and 571 checked internal links. Static checks confirmed the canonical
  URLs, root asset paths, and CNAME. Browser navigation from the homepage to the
  getting-started guide showed Ferrisma, loaded artwork, the correct canonical
  URLs, and no console errors.
- Cargo metadata resolved all 15 workspace members and the independent
  benchmark/fuzzing tools. The benchmark harness tests passed. All 15 frozen
  published-package and benchmark evidence files, protected generated AVIF
  sources, and original image export hashes were preserved.
- GitHub returned the original repository ID `1440750050` with the new full
  name and homepage; the local remote URL was updated after that verification.

After rebasing onto five newer main commits on the same day, the default and
scoped Rust checks, website checks, and benchmark/license harness tests passed
again. The updated site contains 13 prerendered pages and 574 checked internal
links. All three isolated fuzz targets compile with their renamed dependencies,
including the explicit own-JPEG-XL feature. This is a compile check, not a new
fuzz campaign or codec conformance result.

## Review when

- A material naming conflict or recurring pronunciation or spelling problem
  becomes apparent.
- The functional engine is published or hosting is configured, so the release
  and domain status can be recorded with their own evidence.

## Amendment log

- 2026-11-11 — Accepted Ferrisma and recorded the maintainer's reservation of
  ferrisma.dev; technical rename pending.
- 2026-11-11 — Added temporary npm and crates.io placeholder packages at version
  0.0.0 and maintainer publishing instructions; registry publication is pending.
- 2026-11-12 — The maintainer confirmed both placeholder publications. Renamed
  active project identifiers and assets to Ferrisma and configured ferrisma.dev
  at the website root. Retained the published stubs, original benchmark data,
  and prior render hashes as historical evidence.
- 2026-11-12 — Recorded successful local validation, renamed the existing
  GitHub repository to sebastian-software/ferrisma, set its website field, and
  updated the local origin. No engine or codec release gate changed.
- 2026-11-12 — Integrated five newer main commits, retaining their homepage
  vision, native-corpus comparisons, fuzz lanes, and distribution-license
  evidence. Applied the new name to those additions and repeated the relevant
  checks before committing the complete rename on main.
