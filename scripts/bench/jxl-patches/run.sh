#!/usr/bin/env bash
# End to end: corpus, original libjxl tools, measurement. Everything is written
# under WORK (default target/jxl-patches); only the raw JSON is committed
# (scripts/bench/jxl-patches/results-2026-11-15.json). Tables for the record:
#   python3 scripts/bench/jxl-patches/tables.py WORK/results.json
#   WORK=dir PYTHON=python3-with-pillow-and-numpy bash scripts/bench/jxl-patches/run.sh
set -euo pipefail
HERE=$(cd "$(dirname "$0")" && pwd)
ROOT=$(cd "$HERE/../../.." && pwd)
WORK=${WORK:-$ROOT/target/jxl-patches}
PYTHON=${PYTHON:-python3}
mkdir -p "$WORK"
[[ -x "$WORK/build/tools/cjxl" && -x "$WORK/build/tools/ssimulacra2" ]] || bash "$HERE/build_libjxl.sh" "$WORK/build"
"$PYTHON" "$HERE/make_corpus.py" "$WORK/corpus"
"$PYTHON" "$HERE/measure.py" "$WORK/corpus" "$WORK/build/tools" "$WORK/results.json" "$WORK/encoded"
