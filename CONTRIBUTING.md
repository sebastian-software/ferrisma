# Contributing to Ferrixel

Ferrixel develops an experimental image pipeline and its codec ports in one
workspace. Start with [current status](docs/status.md) and the
[documentation guide](docs/README.md) to identify the component and its open
gates. The project remains design-first; implementation does not by itself
establish release readiness or a stable contract.

## Working agreements

- Write **US English** throughout the project.
- Use **Conventional Commits**, such as `docs(adr): clarify region dependencies` or `feat(core): introduce surface metadata`.
- Prefer small, independently reviewable changes with explicit correctness criteria.
- Keep core logic in safe Rust. Treat decoder and GPU adapter safety boundaries independently and document them.
- Never add a dependency solely because it is popular or appears in a benchmark. Explain why the interface is needed and what can be substituted later.

## Living ADRs

Before material changes to graph semantics, memory ownership, decoder contracts, scheduling, pixel layouts, or backend selection:

1. Open the related ADR or create a new one from [the template](docs/adr/TEMPLATE.md).
2. Describe the actual problem and evidence; separate observations from assumptions.
3. Mark the decision `Proposed`, `Accepted`, `Experimental`, `Superseded`, or `Rejected` as appropriate.
4. Update `Last reviewed` and any `Review when` triggers; document alternatives and consequences.
5. Preserve a short amendment log for substantive changes and cross-link superseding ADRs.

These documents are **living** but not context-free: do not replace a decision's reasoning without explaining why it changed. A proposed implementation is not an accepted commitment.

## Validation

Run these from the repository root. Default Cargo members are the engine and
kernels; the pinned development nightly also supports the generated AVIF
packages. See [codec setup](docs/codecs.md#setup) for upstream references,
stable/MSRV verification, and native build prerequisites.

```sh
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings

# Current engine adapters, ICC, and CLI on the pinned nightly.
cargo test -p ferrixel -p ferrixel-kernels --all-targets --all-features
cargo clippy -p ferrixel -p ferrixel-kernels --all-targets --all-features --no-deps -- -D warnings
```

Do not run `--all-features` over the entire workspace. Test adapters and
ports through their [scoped component checks](docs/codecs.md#checks).
The current AVIF reader and encoder share the same port and can link together;
AVIF input still requires the pinned nightly and a supported native target.
The stable Rust 1.98 check covers engine features without `avif-read`, `avif-own-write`, or `cli`.
Scoped PR checks provide the ordinary feedback loop; the
[validation guide](docs/validation.md) explains when live oracle and manual
acceptance evidence is required.

Benchmarks should report the operation, input dimensions and format, hardware, decoder/encoder versions, quality settings, warmup, repetitions, latency distribution, throughput, total allocated bytes, and peak resident memory where possible. Compare equivalent image quality; log planner decisions and intermediate materializations. Performance comparisons must not exclude decoding or encoding unless explicitly labeled as *transform-only*. Use the [quality protocol](docs/benchmarks/M1-protocol.md) and existing benchmark records.

## Project boundaries

Changes to release automation, external APIs, bindings, C FFI, GPU execution,
or public compatibility guarantees require an architectural decision. Existing
experimental APIs and codec boundaries do not grant broader guarantees. First
publication still requires an explicit decision; project dual-licensing is
accepted in [ADR-0010](docs/adr/0010-dual-licensing.md).

## License of contributions

Unless explicitly stated otherwise, contributions intentionally submitted to Ferrixel are licensed under **MIT OR Apache-2.0**, at the recipient's option. Only submit code and assets you have the right to license on those terms. Third-party libraries and copied code must undergo a separate license, attribution, and patent-obligation review before inclusion; the project's license does not relicense its dependencies.

## Codec changes

Follow [the shared codec development guide](docs/codecs.md). Use one branch for a codec change and its Ferrixel integration. Keep the codec-local porting and oracle contracts, and run its scoped fast loop, for example `cargo xtask codecs --codec webp --stage pr`.

## Website and documentation changes

The [Ardo website](homepage/README.md) uses the shared Ferramenta theme. Edit
technical guides in their repository Markdown sources; the site generates its
guide and codec routes from those files. Do not edit generated route copies.
The website's pnpm checks are independent of Cargo and live codec oracles.
See [ADR-0028](docs/adr/0028-ardo-documentation-site.md) for content ownership.
