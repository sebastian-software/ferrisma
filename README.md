# libjxl-rs

A mechanical Rust port of the JPEG XL reference implementation, starting from
the pinned [libjxl](https://github.com/libjxl/libjxl) source. Phase one preserves
source algorithms and behavior before any redesign or additional optimization.

**A fixed lossless encoder now produces complete raw JPEG XL codestreams;
complete image decoding remains unported.** The private Thunder configuration
accepts 8-/16-bit grayscale or RGB pixel buffers. Its output is compared byte
for byte with libjxl and decoded by the original C++ decoder. This does not yet
complete the [first-image milestone](docs/FIRST_IMAGE.md).

The port implements bit I/O, byte-order/bit helpers, enum and primitive field coding,
field visitors, size/preview/animation headers, ANS alias tables, and Huffman
tree encoding and prefix decoding, entropy-stream parsing, hybrid integers,
LZ77 decoding/token generation, histogram normalization/clustering, hybrid
strategy selection, context-map output, complete entropy-stream encoding, and
ICC profile generation with its scalar color-math dependencies, color bundles,
image metadata, custom transform data, complete codestream-header output,
frame headers, frame geometry, filter parameter bundles, permutation streams,
section tables with group offsets, aligned image/channel storage, and modular
transform parameter bundles, the existing scalar reversible color transforms,
and modular predictors with weighted state and reference properties. Modular
decision trees now include serialization, parsing, static-branch filtering,
flat-tree lookup, lookup tables, and group-header bundles. Given-tree residual
encoding and channel decoding now connect samples through real entropy streams.
Palette lookup/inversion and scalar Squeeze encoding/decoding now run through
the original transform dispatch and reverse stack traversal.
Forward palettes include the original lossless selection and RGB lossy
iterations/dithering, with the reference sort order retained for tied colors.
The original modular image decoder now reads standalone streams with local or
global entropy tables and applies their transformations. The generic modular
encoder includes original sample collection, quantization, tree learning, and
entropy output. Standalone streams require caller-supplied dimensions and bit
depth. Encoder parameter state, original header preparation, progressive setup,
stream indexing, and modular sample conversion are translated. Modular frame
stream owners now prepare grouped channels, select local transforms, construct
shared trees/tokens, and write global or local entropy streams. The fixed
configuration connects input conversion, padding, shared-state construction,
frame/group output, image metadata, and byte ownership. General encoder and
decoder orchestration remain unported. Rendering includes the original simple
pipeline and float image/bundle write stages as independently tested components.
Frame transform selection includes original costs, palette/compaction decisions,
rollback, and tree merging. The [first-image milestone](docs/FIRST_IMAGE.md)
defines their integration gate. The Rust library has no external crate
dependencies and does not compile or link C/C++. A separate executable built
from the original C++ source verifies the translated boundaries.

## Build and verify

With the source initialized as described in [CONTRIBUTING.md](CONTRIBUTING.md):

```sh
cargo test --locked --offline
python3 tools/check.py
```

The complete check builds the C++ oracle, runs Clippy, and compares encoded
bytes, decoded values, statuses, bit counts, truncated headers, and lifecycle
checkpoints in debug and release Rust builds. Missing oracle prerequisites are
an error. All binary16 input patterns are compared against the original code.

The provisional Rust API follows upstream's low-level module structure under
`jxl::jxl`. It is not a stable public API, and the crate is not publishable yet.

## Project records

- [Porting contract](docs/PORTING.md): scope, invariants, and gates.
- [Port status](docs/PORT_STATUS.md): implemented units and remaining work.
- [Living ADRs](docs/adr/README.md): current decisions and rationale.
- [Provenance](docs/PROVENANCE.md): source and test origins.
- [Contributing](CONTRIBUTING.md): prerequisites, checks, and commit conventions.

Derived source retains the upstream [BSD-3-Clause license](LICENSE) and
[additional IP rights grant](PATENTS). The C++ reference and Highway dependency
remain separate Git submodules with their own notices.
The translated Highway prefetch helper retains its
[BSD license notice](licenses/HIGHWAY-BSD-3-Clause.txt).
