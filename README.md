# libjxl-rs

A mechanical Rust port of the JPEG XL reference implementation, starting from
the pinned [libjxl](https://github.com/libjxl/libjxl) source. Phase one preserves
source algorithms and behavior before any redesign or additional optimization.

**This is an initial porting slice, not an image encoder or decoder.** It
implements bit I/O, byte-order/bit helpers, enum and primitive field coding,
field visitors, size/preview/animation headers, ANS alias tables, and Huffman
tree encoding and prefix decoding, entropy-stream parsing, hybrid integers,
LZ77 decoding, histogram normalization/clustering, and token output.
Encoding-strategy selection and the image path are still being translated; the
[first-image milestone](docs/FIRST_IMAGE.md) defines their integration gate.
The Rust library has no external crate
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
