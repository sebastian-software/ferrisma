# Contributing

This guide uses `codecs/libjxl-rs` as its working directory. See the
[Ferrisma README](../../README.md), [project status](../../docs/status.md),
and [codec guide](../../docs/codecs.md) for workspace setup and current gates.

Use US English in all project artifacts and Conventional Commits for every
commit, for example `feat(bitstream): port header serialization` or
`fix(fields): preserve truncated-read status`. Keep algorithm improvements
separate from the mechanical port. Read [PORTING.md](docs/PORTING.md) and the
[living ADRs](docs/adr/README.md) before changing a translated boundary.
Create commits without signatures, using `git commit --no-gpg-sign`.

Repository Cargo configuration disables LLVM loop and SLP vectorization to
match the pinned scalar reference's Clang settings. Retain those flags when
overriding `RUSTFLAGS`; changing them requires renewed numeric parity evidence.

The oracle also fixes Quantizer constructor/recomputation calls with a scoped
`-fno-inline-functions` option on the original quantizer.cc. Rust keeps
RecomputeFromGlobalScale out of line. This prevents instrumentation-dependent
signaling-NaN differences from folding multiplication by one into a copy.
CMake 3.18 supplies the source-property directory scope used for this option.
See [ADR 0002](docs/adr/0002-differential-oracle.md).

The separate noise observation unit uses `-fno-unroll-loops` to retain the
original HasAny early-return loop. Clang's unrolled AArch64 max-number reduction
otherwise changes its result for a signaling NaN after an above-threshold
value. This setting does not affect other fixtures or the original codec
library. Keep the exceptional-value cases and exact comparisons.

The frame/pipeline adapter uses `-fno-rtti` to match upstream CMake's class
hierarchy when deriving fixture stages. The original archive does not provide
base RTTI. The setting is limited to that adapter; reference algorithms remain
unmodified.

The dedicated `gaborish.cc` oracle unit includes pristine stage_gaborish.cc and
uses scoped `-O0` with all sanitizer flags active. Ordinary and instrumented
Clang 21 x86 `-O3` objects otherwise choose different NaN operands in the same
source expression. This separate unit retains Highway's original scalar
operator boundaries, including its explicit double product and binary32
narrowing. The original library and existing adapter keep their options; no
algorithm is replaced. Review the original-stage/factory link binding and exact
float-word evidence with compiler, source, target, linker, or instrumentation
changes. See ADRs 0002 and 0003.

The separate XYB fixture unit compiles its included pristine stage/helper at
`-O0` to retain original scalar operand order with and without ASan matrix-load
checks. All sanitizers remain active. This flag affects no other fixture or
original codec-library object. Native local Add/Mul boundaries keep the same
operand order and separate MulAdd rounding in release builds. See
[ADR 0002](docs/adr/0002-differential-oracle.md) and
[ADR 0003](docs/adr/0003-rust-representation.md). This reference configuration
makes no performance parity claim.

The scalar DCT observation unit uses source-scoped `-fno-unroll-loops` with O3,
default contraction, and all sanitizers retained. The original identity loop
otherwise becomes FMSUB in an ordinary native AArch64 object and FMADD with a
negative scale in its instrumented object, changing a NaN sign. Keeping the loop
shape stabilizes those observations and retains the default original finite
rounding, including subnormal and signed-zero cases. See ADRs 0002 and 0003.
The original archive and existing fixture flags remain unchanged.

The encoder transform unit `encoder_transforms.cc` calls the archive's own
transform, Gaborish, convolution and XYB objects. Competing NaN payloads in
TransformFromPixels' 8-point and larger DCTs (AArch64) and in the AFVDCT4x4
basis sum (x86-64) are selected differently by the ordinary and instrumented
archive objects; only those requests use the
separate `encoder_transforms_unit.cc`, which instantiates the pristine
template with source-scoped `-fno-unroll-loops` as for dct.cc, with O3,
default contraction and all sanitizers retained. The original archive keeps
its flags. `tools/encoder_transforms_contraction.py --output DIR` audits the
contraction map of these units in every built oracle graph; see ADR 0002.

The encoder heuristics unit `encoder_heuristics.cc` calls the archive's
quantizer field selection, InitialQuantDC, InitialQuantField,
AdjustQuantField, CfLHeuristics and AcStrategyHeuristics objects and the
external-linkage HWY_SCALAR EstimateEntropy, FindBest8x8Transform and
FindBestMultiplier, all with the archive's flags; its `eqh-chain` request
calls them in LossyFrameHeuristics' kSquirrel order. Its contraction map is
`tools/encoder_heuristics_contraction.json`, audited with
`tools/encoder_transforms_contraction.py --map
tools/encoder_heuristics_contraction.json --output DIR`. The audit reads the
front end's fmuladd sites (`-Xclang -disable-llvm-passes` on the unchanged
compile command), because optimized IR folds constant sites away and places
speculated sites at different debug locations in the ordinary and ASan
objects.

The complete-file oracle compiles the original DecodeFile and ReadICC helper
bodies without their unrelated test framework. At CMake configure time,
`tools/oracle/extract_source.py` copies their exact pinned source ranges and
checks SHA-256 values in `source_fragments.json`. Generated fragments preserve
copyright notices and source line locations. A mismatch fails configuration;
review and renew this evidence with a source migration. The original test memory
manager and codec routines are compiled directly. No framing or ICC algorithm
is reimplemented in the adapter.

The decoder C ABI drivers `jxl_abi_original` and `jxl_abi_rust` link one
driver, written against the pinned public headers only, to the original archive
with its `jxl_threads` runner library and to the Rust static library, which
exports the runners itself. Building the oracle therefore runs
`cargo build --lib --profile checked --features cpp-oracle`, whose library the
checked-profile tests reuse; the `ORACLE_RUST_PROFILE` CMake cache variable
selects another profile. See [ADR 0002](docs/adr/0002-differential-oracle.md).

## Prerequisites

- The shared root toolchain and stable Clippy; the library supports Rust 1.95 or later.
- Python 3, Git, CMake 3.18 or later, and a C++17 compiler for the oracle.
- Linux or macOS for the current oracle build configuration.

The Linux oracle requires Clang and the libc++/libc++abi development packages
(`libc++-dev` and `libc++abi-dev` on Ubuntu). CMake explicitly selects libc++:
the Rust palette sorting boundary translates that library's recorded algorithm.
Using libstdc++ changes the order of equivalent palette entries. This choice
affects only the C++ test reference; ordinary Rust builds remain independent
of a C++ standard library.

Initialize the reference source and its pinned Highway, Brotli, and skcms
dependencies. The oracle builds the complete original static codec library
through upstream CMake, including its default color management implementation:

```sh
git -C ../.. submodule update --init --depth 1 codecs/libjxl-rs/upstream/libjxl
git -C upstream/libjxl submodule update --init --depth 1 third_party/highway third_party/brotli third_party/skcms
```

After those downloads, the checks do not require network access. Keep the
reference checkouts unmodified. `upstream.json` and the Git submodule must agree;
upstream changes require a separate migration and renewed evidence.

## Checks

```sh
python3 tools/check.py
```

This checks the source pins, notices, and local documentation links. It then
runs formatting, Clippy, a release build of all targets, oracle compilation, and
all Rust and differential tests in the `checked` Cargo profile. That profile
inherits release optimizations and keeps overflow checks and debug assertions
enabled. The port mirrors C++ unsigned arithmetic with explicit wrapping
operations, so an omitted one only fails with overflow checks on. Test files
with a `cfg(not(debug_assertions))` variant also run in an ordinary release
build. The check does not install or update toolchains or dependencies.

For an instrumented C++ reference and the minimum Rust version:

```sh
python3 tools/check.py --sanitizers --msrv
```

`--msrv` compiles all targets with the toolchain named by `rust-version` in
`Cargo.toml`, currently `1.95`, which must be installed already. It does not
run the tests on that toolchain. Sanitizers cover the oracle adapters, original
codec library, and its built dependencies; they do not instrument Rust or prove
Rust memory safety. The current platform evidence is listed in
[PORT_STATUS.md](docs/PORT_STATUS.md).

`--stage` runs one part of the check: `static` needs no C++ toolchain, `oracle`
only builds the selected reference graph, and `test` builds it and runs the
tests. `--partition K/N` runs the K-th of N disjoint subsets of the test names,
so N jobs together run every test exactly once. `--filter NAME` runs only the
tests whose names contain `NAME`. The default CMS audit replays the requests
that `--stage test --filter default_cms::` captures for each oracle graph.

Focused checks:

```sh
cargo test -p libjxl-rs --locked --offline --test bitstream
cargo test -p libjxl-rs --locked --offline --test headers
cmake -S tools/oracle -B target/oracle -DCMAKE_BUILD_TYPE=Release
cmake --build target/oracle --target jxl_oracle --parallel 4
cargo test -p libjxl-rs --locked --offline --profile checked --features cpp-oracle --test parity -- --nocapture
```

Set `JXL_CPP_ORACLE` to an absolute path when using a separately built oracle.
The default is `target/oracle/jxl_oracle`. Oracle comparisons use the source's
release status behavior; debug-only abort and diagnostic instrumentation is
outside the current parity contract.

The shared [PR workflow](../../.github/workflows/ci.yml) selects affected
components. For JXL it runs checked-profile Rust tests and retained captures on
Linux x86-64 and ARM, with static checks and MSRV on x86-64. It does not build
the C++ oracle in the PR tier. Nightly and manually dispatched deep workflows
retain the live oracle and broader audits. See
[Ferrisma ADR-0027](../../docs/adr/0027-tiered-validation-and-rust-test-orchestration.md)
for cadence and high-risk change requirements. Historical results in
[PORT_STATUS.md](docs/PORT_STATUS.md) apply to their stated revisions; a local
result does not establish hosted or release acceptance.

The minimum supported Rust version trails stable by about four releases, or
about six months. Raise it deliberately at regular intervals in its own change:
update `rust-version`, the `msrv` toolchains in the Brotli audit tools, and the
documentation. No CI rule fails when it falls behind. Clippy reads
`rust-version`, so a raise can enable lints that suggest newer idioms. In
translated code, keep the source's form and add a function-scoped
`#[expect(clippy::..., reason = "...")]`; for example, `is_multiple_of` differs
from the source's `%` test for a zero divisor. When a lint fires under one
supported stable Clippy and not under another, use `#[allow(clippy::...,
reason = "...")]` instead, because an unfulfilled expectation is itself an
error (Ferrisma ADR-0027, 2026-11-12).

Keep [PORT_STATUS.md](docs/PORT_STATUS.md) current as units and evidence change.
Update the relevant ADR and its evidence when a decision changes, and record
replacements or supersessions explicitly under the workspace policy.


The same check command builds sixteen pinned-original Brotli support observers
in a separate graph and supplies their required `BROTLI_*` environments. It
retains all 38,187 ContextMap requests, including source-defined synthetic width
boundaries. No extra environment setup or runtime C dependency is added to
ordinary Rust builds. The source graphs keep O3/NDEBUG and full sanitizers;
visibility-only test bridges are limited to their existing adapter TUs.
Focused complete-stream and API evidence can be reproduced independently:

```sh
python3 tools/brotli_stream_reference.py --output target/brotli-stream-reference
python3 tools/brotli_stream_api_reference.py --output target/brotli-api-reference
```

These output directories must be new so earlier observations are preserved.
Native AArch64 and x86-64 are the admitted state/partial-bit targets. Actual
per-symbol link bindings, compiler flags, source-generated requests, complete
outputs and test logs are retained for review; slice-level source proofs do not
claim JPEG reconstruction, container integration or whole-codec completion.
