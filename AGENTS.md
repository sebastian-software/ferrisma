# Working in libwebp-rs

- Use US English for code, documentation, ADRs, comments and commit messages.
- Use Conventional Commits, for example `feat(dec): port the VP8 frame decoder`.
- Create unsigned commits with `git commit --no-gpg-sign`. Work on a branch
  per work package and open a pull request; never push to `main`.
- The port is a mechanical, behavior-preserving translation of the pinned
  libwebp v1.6.0. Preserve algorithms, tables, constants, numeric widths,
  operation order, status codes and partial state on error. Do not combine
  translation with redesign, tuning or new optimizations.
- Before writing code, read [the porting contract](docs/PORTING.md), the
  [work plan](docs/WORKPLAN.md) row of your package and the
  [ADRs](docs/adr/README.md). Update an ADR in place, with its `Updated` date,
  when its decision changes.
- Translate into the module files that already exist; `tools/units.json` says
  which package owns which file, and which functions or types a package adds to
  another package's module as an exception (the module header names them too).
  Keep other packages' exception items unchanged. Edit only your package's files and
  `docs/status/<wp-id>.md`. Never edit a provenance header, a `mod.rs`,
  `src/lib.rs` or `docs/PORT_STATUS.md`. If you need something another package
  owns, stop and report it.
- Name items with `python3 tools/rust_name.py` and mark each translated item
  with `// upstream: <path> <CName>`. Keep the upstream copyright notices.
- No stubs, placeholders, partial functions, `todo!()`, C fallbacks, skipped
  parity cases, changed expected outputs or blanket warning suppressions.
  Unported functions stay absent.
- No `unsafe` code outside `src/abi`; there, one operation per block with a
  `// SAFETY:` argument. A unit that needs more stops and asks for an ADR.
- No new dependencies. Ordinary builds compile and link no C.
- Prove each unit differentially: a C probe under `tools/oracle/probes/<wp-id>/`
  and a test in `tests/<wp-id>_*.rs`. A Rust round trip is not evidence.
- Keep `upstream/` unmodified and never advance a submodule.
- Run `python3 tools/check.py --oracle` before committing translated code. See
  [CONTRIBUTING.md](CONTRIBUTING.md) for focused checks and prerequisites.
- Passing a package's tests is not evidence that a codec or the library is
  ported; record exactly what is translated and verified in the status page.
- Do not publish crates, push, add remotes or create repositories unless
  explicitly asked.
