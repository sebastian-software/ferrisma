# Ferrisma homepage

This Ardo site uses the Ferramenta family's pinned Monografie package. Ferrisma
is not yet a registered family identity, so its header, footer, mark and
instrument are site-local (`app/site/chrome.tsx`); shared typography, color and reading components come
from the package. The Ferrisma instrument and mark are rendered in Blender
using the family's pinned studio and mark helpers; their editable recipes and
provenance are in [artwork/](artwork/README.md). Blender is not needed to build or test the site.

From `homepage/`, run `pnpm install --frozen-lockfile`, then `pnpm dev`.
`pnpm test` checks the Markdown link and anchor translation. `pnpm typecheck` and
`pnpm build` synchronize the guide pages from the root
Markdown documentation before checking or building. The generated routes must
not be edited by hand. The site uses the root base path `/` and the canonical
URL `https://ferrisma.dev`. `public/CNAME` carries the custom domain into the
static build; hosting and DNS must serve that domain at the root. `pnpm preview`
serves the built site at `/`.

The Homepage workflow checks types, tests the link translation, builds every
route, and retains the static site as a CI artifact. It does not deploy the site.

The site describes an experimental project. Keep codec-port capabilities
separate from the engine's implemented adapters, and source performance claims
to reproducible evidence.

The homepage (`app/routes/home.tsx`) is built on the family's Monografie page
system. Its facts (the ports, their originals and pinned versions, the gate
state, the speed figures) live in `app/ferrisma.ts`, each with the repository
document it comes from; update the document first, then that file. The
drawings are inline SVG components in `app/drawings/`, in a wide and a narrow
layout. The per-port gate state is generated from the table "Where each port
stands" in `docs/quality-gate.md` (`scripts/gate-state.ts`, run with the docs
sync); `/how-a-port-is-made` explains the making of one port. `DESIGN-NOTES.md` records the page's arc, the copy decisions, what is
provisional and what changes once Ferrisma is in the family registry. Current
component status remains owned by `docs/status.md`. Guides are derived from
canonical Markdown, including the quality gate guide, the supported matrix,
the color contract, codec development and the M1 acceptance register.
