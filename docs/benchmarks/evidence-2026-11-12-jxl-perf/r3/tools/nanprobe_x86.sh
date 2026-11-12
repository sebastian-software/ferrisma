#!/bin/sh
# usage: nanprobe_x86.sh <dct-variant.rs> <label>
# Cross-builds the checked-profile NaN probe for x86_64-apple-darwin from the
# port directory (scalar rustflags) and runs it under Rosetta 2.
set -e
R="${S:?scratch directory}/r3"
W="${W:?repository root}"
cp "$1" "$W/codecs/libjxl-rs/src/jxl/dct.rs"
cd "$W/codecs/libjxl-rs"
export CARGO_BUILD_JOBS=4
cargo build --profile checked --target x86_64-apple-darwin --example dct_nan_probe > "$R/build-x86-$2.log" 2>&1 || { tail -20 "$R/build-x86-$2.log"; exit 1; }
arch -x86_64 "$W/target/x86_64-apple-darwin/checked/examples/dct_nan_probe" > "$R/nanx86-$2.txt" 2>/dev/null
EXHAUSTIVE=1 arch -x86_64 "$W/target/x86_64-apple-darwin/checked/examples/dct_nan_probe" > "$R/exx86-$2.txt"
