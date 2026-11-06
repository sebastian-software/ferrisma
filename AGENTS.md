# Working in libjpeg-turbo-rs

- Use US English for code, documentation, ADRs, comments and commit messages.
- Use Conventional Commits, for example `feat(dhuff): port the Huffman decoder`.
- Work on a branch per work package and open a pull request; never push to
  `main`.
- The port is a mechanical, behavior-preserving translation of the pinned
  libjpeg-turbo 3.2.1 decoder. Preserve algorithms, tables, constants, numeric
  widths, operation order, error codes and messages, and partial state on
  error. Do not combine translation with redesign, tuning or new optimizations.
- Before writing code, read the [port plan](docs/PORT_PLAN.md) row of your
  package and the [ADRs](docs/adr/README.md). Update an ADR in place, with its
  `Updated` date, when its decision changes. A question the ADRs do not answer
  stops the package; report it instead of deciding alone.
- Mark each translated item with `// upstream: <path> <CName>`, for example
  `// upstream: src/jdhuff.c decode_mcu`. Keep the upstream copyright notices.
- No stubs, placeholders, partial functions, `todo!()`, C fallbacks, skipped
  parity cases, changed expected outputs or blanket warning suppressions.
  Unported functions stay absent.
- No `unsafe` code outside the modules an ADR names; there, one operation per
  block with a `// SAFETY:` argument.
- No new dependencies. Ordinary builds compile and link no C.
- Prove each unit against the C oracle ([ADR 0002](docs/adr/0002-differential-oracle.md)).
  A Rust round trip is not evidence.
- Keep `upstream/` unmodified and never advance the submodule.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and
  `cargo test` before committing.
- Passing a package's tests is not evidence that the decoder is ported; record
  exactly what is translated and verified in [PORT_STATUS.md](docs/PORT_STATUS.md).
- Do not publish crates, push to `main`, add remotes or create repositories
  unless explicitly asked.
