# libjxl-rs

A mechanical Rust port of the JPEG XL reference implementation, starting from
the pinned [libjxl](https://github.com/libjxl/libjxl) source. Phase one preserves
source algorithms and behavior before any redesign or additional optimization.

**The first complete lossless image path is implemented in Rust.** The fixed
Thunder configuration encodes 8-/16-bit grayscale or RGB buffers into complete
raw JPEG XL codestreams. Rust parses their global metadata and frames and
recovers the original integer pixels. Encoded bytes, metadata, samples, and
error paths are compared with pinned libjxl, including cross-decoding in both
directions. See the [first-image evidence](docs/FIRST_IMAGE.md). General codec
configurations and the public C API remain unported.

**The first complete VarDCT decode path is also implemented.** `decode_vardct`
parses raw original codestreams through DC/AC groups, dequantization and scalar
inverse transforms to exact sRGB float and integer pixels. Its admitted profile
includes gray/RGB, 8/16-bit samples, optional full-resolution alpha, progressive
AC passes and optional Gaborish. See [the VarDCT milestone](docs/VARDCT_IMAGE.md)
for the precise configuration, source mapping, evidence and remaining scope.

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
pipeline and float image/bundle write stages. Modular group decoding now feeds
that pipeline through the original conversion and full-image finalization
operations. The complete fixed path connects those components to global
codestream parsing, original frame scheduling, and integer buffer export.
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

## First image path

```rust
use jxl::jxl::dec_frame::decode_lossless_thunder;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};

fn round_trip() -> Result<(), jxl::jxl::base::status::StatusCode> {
    let pixels = [255, 0, 0, 0, 255, 0]; // Two RGB pixels.
    let mut config = LosslessThunderConfig::new(
        2, 1, false, LosslessSampleDepth::Uint8,
    )?;
    let encoded = config.encode_pixels(&pixels, None)?;
    let decoded = decode_lossless_thunder(encoded.as_slice());
    decoded.status?;
    assert_eq!(decoded.pixels.as_slice(), &pixels);
    Ok(())
}
```

The decoder obtains dimensions, channel layout, and bit depth from the file.
Output is interleaved bytes, with native-endian words for 16-bit samples.
The fixed profile uses sRGB, one final frame, and the original simple rendering
option. `LosslessThunderConfig::new_with_alpha` also accepts interleaved RGBA
or gray/alpha with one full-resolution, unassociated alpha channel at the color
sample depth. Lossless defaults preserve color values under zero alpha.
Animation and other extra channels remain outside this profile. Input outside
that profile has an explicit admission boundary; this is not the general
decoder API.

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
