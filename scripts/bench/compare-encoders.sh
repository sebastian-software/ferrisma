#!/usr/bin/env bash
# Compare rav1e and the workspace libavif encoder using the existing prepared
# corpus and independent scorer. See docs/benchmarks/encoder-adoption.md.
# Usage: compare-encoders.sh OUT_DIR [match|time|all]
set -euo pipefail
out=${1:?usage: compare-encoders.sh OUT_DIR [match|time|all]}
phase=${2:-all}
repo=$(cd "$(dirname "$0")/../.." && pwd)
mkdir -p "$out"
out=$(cd "$out" && pwd)
cd "$repo"
case "$phase" in
  match|all)
    cargo build -p ferrixel-libavif --release --features cli --bin ferrixel-libavif
    python3 -I "$repo/scripts/bench/compare-sharp.py" encoder-match "$out"
    ;;
  time) ;;
  *) echo "unknown phase: $phase" >&2; exit 2 ;;
esac
if [[ "$phase" == time || "$phase" == all ]]; then
  python3 -I "$repo/scripts/bench/compare-sharp.py" encoder-time "$out"
fi
