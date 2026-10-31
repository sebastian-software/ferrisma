# Working in libjxl-rs

- Use US English for code, documentation, ADRs, comments, and commit messages.
- Use Conventional Commits, for example `feat(bitstream): port the bit reader`.
- Create unsigned commits with `git commit --no-gpg-sign`.
- Phase one is a mechanical, behavior-preserving port of the pinned libjxl
  source. Preserve algorithms, field order, constants, numeric widths, and
  existing optimizations. Do not combine translation with redesign or tuning.
- Read [the porting contract](docs/PORTING.md) and the accepted
  [living ADRs](docs/adr/README.md) before changing a ported boundary. Update the
  relevant ADR in place when its decision changes, including its update date.
- Keep `upstream/libjxl` and its dependencies unmodified. Its Git submodule
  commit is the reference revision. Do not advance it during a porting batch.
- Identify the source file and any language-required deviations in each ported
  module. Preserve upstream copyright notices, LICENSE, and PATENTS.
- Do not add stubs, fallbacks to C++, skipped parity cases, changed expected
  outputs, or blanket warning suppressions to make checks pass.
- Keep C++ in the explicit test oracle. Ordinary Rust builds must not compile
  or link C/C++ code. Every unsafe operation needs a local safety argument.
- Run `python3 tools/check.py` before committing implementation changes. See
  [CONTRIBUTING.md](CONTRIBUTING.md) for focused checks and prerequisites.
- Keep [port status](docs/PORT_STATUS.md) accurate. Passing a slice's tests is
  not evidence that an image codec or the entire library has been ported.
- Do not publish crates, push commits, or create remote repositories unless
  explicitly requested.
