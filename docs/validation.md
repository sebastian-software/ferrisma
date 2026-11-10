# Validation

Ferrixel uses scoped pull request checks and deeper reference suites. The
[codec guide](codecs.md) owns setup and commands; [ADR-0027](adr/0027-tiered-validation-and-rust-test-orchestration.md)
records the policy and its evidence. M1 release and license gates remain open.

## Local checks

From the repository root, the default members are the engine and kernels:

```sh
cargo fmt --all --check
cargo test --all-targets
cargo clippy --all-targets -- -D warnings
cargo xtask codecs --codec webp --stage pr
```

The codec example needs the pinned references and toolchains described in the
[setup guide](codecs.md#setup). Select `jpeg`, `webp`, `jxl`, or `avif` for the
component under change. The PR stage checks retained fixtures and provenance;
it does not build a live C/C++ oracle. Do not use workspace-wide `--all-features`:
the interim AVIF reader and the libavif adapter have conflicting symbols.

## Pull requests and deeper evidence

The [PR workflow](../.github/workflows/ci.yml) selects affected components and
fails closed when it cannot determine the scope. Common configuration changes
fan out; root documentation and website changes skip heavy tests. Codec-local
documentation can still select component checks. Its native Linux checks
cover x86-64 and ARM where supported. A green scoped run does not establish all
codec or release gates.

The [central nightly](../.github/workflows/nightly.yml) and manually selected
branch runs exercise deeper source, oracle, sanitizer and platform checks.
Exhaustive AOM original-consumer, recovery and translation workflows remain
manual. Changes to oracle pins, goldens, unsafe ABI, SIMD or floating-point
semantics require the relevant deep branch run or equivalent recorded native
evidence before acceptance. A missing required live oracle is an error, not a
skipped comparison. Nightly failures block release; do not silently refresh a
golden to make a test pass.
