# Ferrixel homepage

This Ardo site uses the Ferramenta family's pinned Monografie package. Ferrixel
is not yet a registered family identity, so its header, footer and artwork
are site-local; shared typography, color and reading components come from the
package. Ferrixel's raster loupe and matching product mark are rendered in
Blender using the family's pinned studio. Their editable recipes, favicon and
touch-icon export commands, and provenance are in
[artwork/](artwork/README.md). Blender is not needed to build or test the site.

From `homepage/`, run `pnpm install --frozen-lockfile`, then `pnpm dev`.
`pnpm test` checks the Markdown link translation. `pnpm typecheck` and
`pnpm build` synchronize the guide pages from the root
Markdown documentation before checking or building. The generated routes must
not be edited by hand. The default base path is `/ferrixel/` for GitHub Pages;
`pnpm preview` serves the built site under that path. A custom domain requires
updating the Ardo base and canonical site URL together.

The Homepage workflow checks types, tests the link translation, builds every
route, and retains the static site as a CI artifact. It does not deploy the site.

The site describes an experimental project. Keep codec-port capabilities
separate from the engine's implemented adapters, and source performance claims
to reproducible evidence.

The homepage's pipeline explorer explains the current stages. Its interactive
black/white example illustrates the sRGB transfer formula in
`src/working_color.rs` and the midpoint covered in `src/linear_srgb_tests.rs`;
it does not execute the Rust pipeline in the browser. Current component status
remains owned by `docs/status.md`. Guides are derived from canonical Markdown,
including the color contract, codec development and M1 acceptance register.
