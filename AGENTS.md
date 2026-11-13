# Working on Ferrisma

Ferrisma is in an intentionally **design-first** phase. Do not turn a proposed architectural idea into a public API or dependency merely because it is described in an ADR.

## Non-negotiable conventions

- **US English** for source code, comments, documentation, ADRs, tests, commit messages, and issues.
- **Conventional Commits** for commit messages.
- Read [README.md](README.md), [docs/architecture.md](docs/architecture.md), and [docs/adr/README.md](docs/adr/README.md) before implementation.
- Keep architecture decisions **living**: update the ADR and its evidence and review date when design changes. Record replacements and supersessions explicitly; do not silently rewrite historical rationale.
- Prefer narrow abstractions supported by a realistic end-to-end use case to speculative extensibility.
- No `unsafe` in the engine crate. If a future codec or platform adapter needs `unsafe` or FFI, isolate and audit that boundary; do not imply that the entire dependency graph is memory-safe.
- Read [M1 milestone plan](docs/milestones/M1-complete-image-pipeline.md) before expanding pipeline scope; preserve its acceptance criteria and explicit codec-license gates.
- No unmeasured performance claims. Establish correctness and a scalar/reference path before hand-written SIMD or GPU specialization.
- Never optimize only for historical JPEG compatibility at the expense of modern formats; do not claim partial decoding is supported unless the *specific decoder implementation* exposes it.
- Keep output semantics explicit: crop coordinates, orientation, bit depth, color space, transfer function, chroma siting, alpha handling, precision and filtering all matter.
- Codec color metadata must describe the **emitted samples**. ICC header inspection alone is not a color transform; never auto-prioritize ICC versus CICP or reinterpret P3/PQ/HLG as sRGB. See ADR-0015. In particular, [ADR-0011](docs/adr/0011-borrowed-planar-surface-contract.md) permits only even-origin YUV420P10 crops and no planar resize until a chroma-aware quality policy exists.
- The plan must make materializations and CPU/GPU transfers observable.
- **v1.0 quality gate** ([ADR-0041](docs/adr/0041-v1-quality-gate-equivalence-with-the-ported-original.md)): a port's scalar path must be bit-identical to the original's scalar build, and its SIMD path to the original's SIMD build at the same level (NEON; AVX2 and the lower x86 levels), per target. Only NaN payloads are exempt. AVX-512, SVE and DotProd/I8MM stay off by default. Never refresh an expected output, golden, tolerance or oracle pin to pass it.
- **Validation is local first:** AArch64 natively and x86-64 under Rosetta 2 (`--target x86_64-apple-darwin`, `ROSETTA_ADVERTISE_AVX=1`), plus one hosted Linux x86-64 run per release candidate. Performance is reported against the original in three columns and is not a release gate. See [the quality gate guide](docs/quality-gate.md).

## Local checks

```sh
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
```

## Before expanding scope

Add or amend an ADR with its problem, alternatives, evidence, trade-offs and re-evaluation trigger. For any optimization, include a reproducible benchmark, peak-memory observation, and image-quality/correctness checks. Update the ADR index and architecture sketch accordingly.

## Shared codec workspace

- Read [codec development](docs/codecs.md) and [ADR-0026](docs/adr/0026-codecs-in-one-repository-and-workspace.md) and [ADR-0027](docs/adr/0027-tiered-validation-and-rust-test-orchestration.md) before changing workspace configuration or imported ports.
- Root default members are the engine and kernels. Run component verification through `cargo xtask codecs`; preserve each port's scoped oracle and provenance gates.
- Do not run `--all-features` across the entire workspace. Use scoped package checks for the engine and adapters. AVIF input and the encoder now share the workspace port and can link together (ADR-0031); the native AVIF platform and nightly requirements still apply.
- Keep generated AVIF source hashes intact. Root profiles preserve its existing arithmetic semantics; do not broaden those exceptions.
