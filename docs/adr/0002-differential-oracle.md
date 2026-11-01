# Compare against the original C++ implementation

- Status: accepted
- Created: 2026-10-31
- Updated: 2026-11-01

Round trips within the Rust port can agree with each other while both differ
from libjxl. Tests therefore compile the pinned, unmodified C++ source into a
separate oracle executable and compare its observations with Rust.

The oracle's line protocol is test infrastructure. It calls original upstream
functions; it does not reimplement their algorithms. The Rust crate has no
C/C++ build or link dependency. `cpp-oracle` enables differential integration
tests and fails when the executable is unavailable. The repository check
command always runs this evidence, including debug and release Rust builds.

Compare encoded bytes and bit counts, decoded values, exact status codes,
consumed-bit counters, padding, truncation, and finalization behavior. Include
boundary values and deterministic generated inputs. Image-level conformance,
cross-platform behavior, and performance remain separate, future gates.

Build the complete original static codec library through its own CMake project,
with scalar Highway, Brotli, and skcms at upstream's submodule pins. This keeps
the reference's image entry points available without copying build definitions
or supplying replacement implementations. Disable upstream tools, tests, and
optional integrations; do not change codec routines. Generate export and version
headers through the original build. These dependencies belong exclusively to
the C++ oracle; normal Rust builds remain independent of them. Sanitizer flags
apply before adding the upstream project, covering the adapters, codec, and
built dependencies.

This requires a C++ toolchain and the reference's pinned dependencies for
the checks. Review the oracle whenever a source revision, compile configuration,
protocol, or ported boundary changes. Never turn missing oracle evidence into a
passing test.

The scalar Clang build retains upstream `lib/CMakeLists.txt`'s
`-fno-slp-vectorize -fno-vectorize` settings. Repository Rust builds mirror them
with `-C no-vectorize-slp -C no-vectorize-loops` in
[Cargo configuration](../../.cargo/config.toml). They affect floating-point
reductions: allowing auto-vectorization changed LZ77 cost rounding, and ASan
then selected a different reduction loop. Both ordinary and instrumented
oracles must use the upstream settings. Rust SLP also combined three DC
reciprocals and an unchanged copy into a four-lane division, computing the copy
as `value / 1`. That quieted a signaling NaN in release while C++ retained its
original stored bits. Preserve the original compile configuration in both
languages so raw copies remain bit-exact.
Keep exact float-bit comparisons;
do not compensate with relaxed tolerances or changed reference algorithms.

The frame fixture translation unit uses Clang/GCC `-fno-access-control` to
observe and seed the original private state at internal boundaries. This is
limited to the test adapter and does not change declarations, layouts, source
files, or runtime algorithms. Do not replace private methods with test versions
or rewrite source access specifiers. Omit source-indeterminate state until the
original algorithm initializes it. Observe writer bits through a padded scratch
copy so inspecting an intermediate result does not alter the real writer.

Frame fixtures follow the original caller's lifecycle: shared trees/tokens and
histograms belong to the non-streaming path; streaming uses local generic
streams. Internal callers must satisfy source preconditions before observation.
Undefined source inputs or invalid lifecycle combinations do not become passing
Rust behavior by suppressing sanitizer failures or changing reference routines.

The shared-state frame fixture calls the complete original ComputeEncodingData
for state/group observations, then independently calls EncodeFrameOneShot and
its real output processor for complete frame bytes. The native side composes
ported components without claiming that the enclosing constructor/entry point
is translated. Small DC-reader fixture spans use real backing storage with
eight leading and trailing bytes while preserving their original span length
and contents. This keeps the source end-minus-eight pointer inside its backing
allocation and represents a non-null empty stream rather than the null-reader
sentinel. The source reader and its status/close algorithms stay unchanged.

Keep Quantizer's constructor/recomputation boundary stable in the reference.
Clang's ordinary release build inlined the default constructor's arithmetic
and reduced multiplier-by-one operations to stored copies. ASan's higher
inlining cost retained the RecomputeFromGlobalScale call and quieted signaling
NaNs. Assembly inspection verified both paths. Compile only the original
quantizer.cc with -fno-inline-functions so both reference variants execute the
source recomputation. CMake 3.18 is required for that source's directory-scoped
option. Rust marks RecomputeFromGlobalScale inline(never) for the same reason.
This pins one reproducible numeric configuration while keeping all original
source routines, scalar algorithms, and exact comparisons. It does not establish
performance parity. Review this boundary after compiler/source changes; do not
replace it with NaN-bit rewriting or relaxed comparisons.
