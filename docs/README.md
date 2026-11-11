# Documentation guide

Start with the [project overview](../README.md) and
[current status](status.md). Ferrixel contains an experimental image engine,
its CPU kernels and encoder adapter, and four codec ports in one workspace.
This guide separates today's usable paths from design decisions and recorded
experiments.

## Find the right document

| I want to… | Start here |
| --- | --- |
| Understand what is included and what works today | [Current status](status.md): components, actual engine backends, and remaining gates |
| Try the CLI or contribute a change | [Getting started](getting-started.md), [contributor guide](../CONTRIBUTING.md), then [codec development](codecs.md) for workspace setup and scoped checks |
| Understand the pipeline and its constraints | [Architecture](architecture.md) and [color contract](design/color-management-contract.md) |
| See what must happen before a release | [M1 acceptance plan](milestones/M1-complete-image-pipeline.md) and [evidence/gate register](milestones/M1-evidence-and-gates.md) |
| Follow priorities and performance work | [Roadmap](roadmap.md) and [M2 plan with dated progress](milestones/M2-performance-architecture.md) |
| Understand why a decision was made | [Engine ADR index](adr/README.md); Proposed and Experimental decisions do not imply completed features |
| Work on a codec or inspect its coverage | The component guides below, plus [shared verification and provenance](codecs.md) |
| Assess measured quality or performance | [M1 benchmark protocol](benchmarks/M1-protocol.md), [M2 baseline](benchmarks/M2-baseline.md), [Sharp comparison](benchmarks/M2-sharp-comparison.md), and [AVIF backend comparison](benchmarks/M2-libavif-backend.md) |
| Reproduce corpus and comparison checks | [Corpus inventory and harness](benchmarks/corpus-and-comparison.md), [2026-11-11 quality and large-region smoke](benchmarks/M2-corpus-smoke-2026-11-11.md) |
| Review safety and distribution boundaries | [Fuzzing](fuzzing.md), [license audit](license-audit.md), and each codec's own notices and safety policy |

## Component guides

| Component | Entry point | Detailed contracts |
| --- | --- | --- |
| JPEG | [Port overview](../codecs/libjpeg-turbo-rs/README.md) | [JPEG ADRs](../codecs/libjpeg-turbo-rs/docs/adr/README.md) |
| WebP | [Port overview](../codecs/libwebp-rs/README.md) | [Porting rules](../codecs/libwebp-rs/docs/PORTING.md), [provenance](../codecs/libwebp-rs/docs/PROVENANCE.md), [ADRs](../codecs/libwebp-rs/docs/adr/README.md) |
| JPEG XL | [Port overview](../codecs/libjxl-rs/README.md) | [Port status](../codecs/libjxl-rs/docs/PORT_STATUS.md), [porting rules](../codecs/libjxl-rs/docs/PORTING.md), [ADRs](../codecs/libjxl-rs/docs/adr/README.md) |
| AVIF and its codec dependencies | [Port overview](../codecs/libavif-rs/README.md) | [Porting rules](../codecs/libavif-rs/docs/PORTING.md), [native licenses](../codecs/libavif-rs/docs/NATIVE-LICENSES.md), [ADRs](../codecs/libavif-rs/docs/adr/) |
| Ferrixel's optional libavif encoder adapter | [Adapter guide](../crates/ferrixel-libavif/README.md) | [ADR-0024](adr/0024-pluggable-encoders-and-libavif-rs-backend.md) |
| CPU kernels | [Source and API contracts](../crates/ferrixel-kernels/src/lib.rs) | [ADR-0020](adr/0020-kernel-crate-and-isa-dispatch.md) and [precision policy](adr/0021-working-precision-policy.md) |

## Decisions and historical evidence

The engine's [ADRs](adr/README.md) own architectural policy. Codec-local ADRs
retain each port's scope, oracle, provenance, and unsafe-boundary contracts.
[ADR-0026](adr/0026-codecs-in-one-repository-and-workspace.md) changes repository
and workspace placement; [ADR-0027](adr/0027-tiered-validation-and-rust-test-orchestration.md)
changes validation cadence. Neither erases those codec contracts.

The early [Gray8](design/region-experiment.md),
[borrowed planar](design/planar-surface-experiment.md),
[decoder](design/real-decoder-experiment.md),
[encoded-sample resize](design/decoded-resize-experiment.md), and
[linear-sRGB](design/color-correctness-experiment.md) experiments explain how
contracts were tested. Their narrow scopes do not describe the complete current
pipeline. Likewise, dated research and benchmark records describe the code,
dependencies, and hardware they measured. Start from [current status](status.md)
when deciding what is available now.

Keep new claims in the document that owns them: status in the status overview,
commands in contributor/component guides, acceptance evidence in milestone or
benchmark records, and decisions in ADRs. Link those sources from entry points
instead of maintaining competing progress summaries.
