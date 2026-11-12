# libjpeg-turbo-rs fuzz target

This is the JPEG port in the Ferrisma monorepo. See the [project overview](../../../README.md),
[status](../../../docs/status.md), and [fuzzing policy](../../../docs/fuzzing.md).

A cargo-fuzz target over the port's safe decoding API
([ADR 0006](../docs/adr/0006-safe-decoding-api.md)), decided in
[ADR 0009](../docs/adr/0009-coverage-guided-fuzzing.md). This crate is
separate from the library: it is not a member of the library's or the
monorepo's workspace, it is never a dependency of the library, and its only
dependency besides the library is `libfuzzer-sys`.

| Target | What it calls |
| --- | --- |
| `decode` | `jpeg::Decoder` and `jpeg::Scanlines` on the input bytes, in seven cases of output format, scale, IDCT, upsampling, crop with skipped rows and fail-on-warning (`src/lib.rs`) |

Run from `codecs/libjpeg-turbo-rs` with the workspace's pinned nightly and
cargo-fuzz 0.13.2, after initializing the upstream submodule:

```sh
python3 tools/build_oracle.py --variant release   # optional: the bit-test seeds
python3 tools/bittests.py --run --variant release
python3 fuzz/seed_corpus.py fuzz/corpus/decode --oracle build/oracle/release
LIBJPEG_FUZZ_SEEDS="$PWD/fuzz/corpus/decode" cargo test --manifest-path fuzz/Cargo.toml --lib -- --nocapture
cargo fuzz run --sanitizer address decode -- \
  -dict=upstream/libjpeg-turbo/fuzz/jpeg.dict -max_len=524288 -timeout=10 -rss_limit_mb=2048
```

Without `--oracle`, the seed script copies only upstream's test images and the
engine's JPEG fixtures. A crash lands in `fuzz/artifacts/decode/`; reproduce
it with `cargo fuzz run decode fuzz/artifacts/decode/<file>`. Before fixing
it, check the input against the C oracle: the port keeps upstream's behavior
(ADR 0009, "Findings").
