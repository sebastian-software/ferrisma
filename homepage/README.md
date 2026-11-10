# Ferrixel homepage

This Ardo site uses the Ferramenta family's pinned Monografie package. Ferrixel
is not yet a registered family identity, so its header, footer and pipeline
illustration are site-local; shared typography, color and reading components
come from the package. No other product's identity is used.

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
