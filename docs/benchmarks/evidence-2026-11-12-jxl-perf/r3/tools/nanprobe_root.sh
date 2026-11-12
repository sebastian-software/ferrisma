#!/bin/sh
# usage: nanprobe_root.sh <dct-variant.rs> <label>
# Same probe built from the repository root in release (auto-vectorization on,
# as the engine builds); writes nanroot-<label>.txt and exroot-<label>.txt.
set -e
R="${S:?scratch directory}/r3"
W="${W:?repository root}"
cp "$1" "$W/codecs/libjxl-rs/src/jxl/dct.rs"
cd "$W"
export CARGO_BUILD_JOBS=4
cargo build --release -p libjxl-rs --example dct_nan_probe 2>&1 | grep -E "^error|Finished" || true
"$W/target/release/examples/dct_nan_probe" > "$R/nanroot-$2.txt" 2>/dev/null
EXHAUSTIVE=1 "$W/target/release/examples/dct_nan_probe" > "$R/exroot-$2.txt"
