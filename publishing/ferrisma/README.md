# Ferrisma 0.0.0 publication archive

On November 12, 2026, the maintainer confirmed that both packages below had
been published. Keep `npm/` and `crate/` as the source of those placeholder
releases, including their original repository URLs. The commands below record
the original publication procedure; version `0.0.0` must not be published again.
Future functional releases come from the renamed engine or actual bindings,
with a new version and their existing release gates.

These temporary publication artifacts use the name `ferrisma` and version
`0.0.0` on npm and crates.io. They identify the experimental image pipeline
chosen in [ADR-0038](../../docs/adr/0038-ferrisma-project-name.md), but expose no
image processing API. Neither package has dependencies or build scripts.

The Rust stub has its own empty workspace declaration so packaging it does not
resolve or publish the engine and codec workspace. It is a temporary publication
artifact, not a second engine development workspace.

## npm

From the repository root:

```sh
cd publishing/ferrisma/npm
npm login --registry https://registry.npmjs.org/
npm publish --dry-run --access public --registry https://registry.npmjs.org/
npm publish --access public --registry https://registry.npmjs.org/
```

Complete the browser login and any requested two-factor verification. The
published package is the unscoped `ferrisma@0.0.0`.

## crates.io

From the repository root:

```sh
cd publishing/ferrisma/crate
cargo +stable login --registry crates-io
cargo +stable publish --dry-run --allow-dirty --registry crates-io
cargo +stable publish --allow-dirty --registry crates-io
```

For `cargo login`, enter your crates.io API token at the prompt. `--allow-dirty`
allows the newly created, uncommitted placeholder files to be packaged; Cargo
still builds and verifies the package. The published crate is `ferrisma 0.0.0`.

## Verify publication

```sh
npm view ferrisma@0.0.0 name version --registry https://registry.npmjs.org/
cargo +stable info ferrisma@0.0.0 --registry crates-io
```

Both should report `ferrisma` and `0.0.0`. Preparing the local artifacts or running
the dry runs does not reserve a registry name; publication must succeed.

Command references: [npm publish](https://docs.npmjs.com/cli/v11/commands/npm-publish/),
[Cargo publish](https://doc.rust-lang.org/cargo/commands/cargo-publish.html).
