#!/bin/sh
# usage: nanprobe.sh <dct-variant.rs> <label>
# Installs the variant as dct.rs, builds the checked-profile NaN probe from the
# port directory (scalar rustflags, as the parity tests), records per-case hashes.
set -e
R="${S:?scratch directory}/r3"
P="${W:?repository root}/codecs/libjxl-rs"
cp "$1" "$P/src/jxl/dct.rs"
cd "$P"
export CARGO_BUILD_JOBS=4
cargo build --profile checked --example dct_nan_probe > "$R/build-$2.log" 2>&1 || { tail -20 "$R/build-$2.log"; exit 1; }
"$P/../../target/checked/examples/dct_nan_probe" > "$R/nan-$2.txt"
if [ "$2" != base ]; then
  if cmp -s "$R/nan-base.txt" "$R/nan-$2.txt"; then echo "$2: IDENTICAL to base"; else
    echo "$2: DIFFERS: $(diff "$R/nan-base.txt" "$R/nan-$2.txt" | grep -c '^>') cases"
    diff "$R/nan-base.txt" "$R/nan-$2.txt" | grep '^>' | awk '{print $2,$3,$4,$5}' | sort | uniq -c | head -30
  fi
fi
