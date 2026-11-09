# Working in libjpeg-turbo-rs

- Use US English for code, documentation, ADRs, comments and commit messages.
- Use Conventional Commits, for example `feat(dhuff): port the Huffman decoder`.
- Make one branch and one pull request per change, ideally for a
  [GitHub issue](https://github.com/sebastian-software/libjpeg-turbo-rs/issues);
  never push to `main`.
- The port is a mechanical, behavior-preserving translation of the pinned
  libjpeg-turbo 3.2.1 decoder. Preserve algorithms, tables, constants, numeric
  widths, operation order, error codes and messages, and partial state on
  error. Do not combine translation with redesign, tuning or new optimizations.
- Before writing code, read the [ADRs](docs/adr/README.md). Record every
  decision in the ADR that owns its topic, edited in place with its `Updated`
  date, or in a new numbered ADR when none fits; there is no separate plan or
  status document. A question the ADRs do not answer stops the change; report
  it instead of deciding alone.
- Mark each translated item with `// upstream: <path> <CName>`, for example
  `// upstream: src/jdhuff.c decode_mcu`. Keep the upstream copyright notices.
- No stubs, placeholders, partial functions, `todo!()`, C fallbacks, skipped
  parity cases, changed expected outputs or blanket warning suppressions.
  Unported functions stay absent.
- No `unsafe` code outside the modules an ADR names; there, one operation per
  block with a `// SAFETY:` argument.
- No new dependencies. Ordinary builds compile and link no C.
- Prove each unit against the C oracle ([ADR 0002](docs/adr/0002-differential-oracle.md)).
  A Rust round trip is not evidence. Show that the tests catch a deliberate
  fault in the code a change covers, and name the faults tried in the pull
  request (ADR 0002, "Mutations").
- Keep `upstream/` unmodified and never advance the submodule.
- Run `cargo fmt --check`, `cargo clippy --all-targets -- -D warnings` and
  `cargo test` before committing.
- Passing a change's own tests is not evidence of a faithful port. What is
  verified is shown by the tests, which assert the measured results, and
  stated in the ADRs.
- Do not publish crates, push to `main`, add remotes or create repositories
  unless explicitly asked.
