#!/usr/bin/env bash
# Run the pinned external JPEG XL conformance corpus against the Rust port.
#
#   tools/conformance/run.sh <workdir>
#
# Requires network access (first run), python3, uv (for numpy) or a python3
# with numpy, and the lcms2 shared library (LCMS2_LIB_PATH, defaulting to the
# Homebrew or Debian location). The official runner scripts come from the
# verified corpus revision and are copied outside the download directory.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
port="$(cd "$here/../.." && pwd)"
work="$(mkdir -p "$1" && cd "$1" && pwd)"

python3 -I "$here/corpus.py" fetch "$work"

# Build from the port directory so its scalar rustflags apply.
(cd "$port" && cargo build --release --locked --example conformance_decode)
decoder="$(cd "$port" && cargo metadata --format-version 1 --no-deps |
  python3 -I -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')/release/examples/conformance_decode"

runner="$work/runner"
mkdir -p "$runner"
cp "$work/corpus/scripts/conformance.py" "$work/corpus/scripts/lcms2.py" "$runner/"

if [ -z "${LCMS2_LIB_PATH:-}" ]; then
  for candidate in /opt/homebrew/lib/liblcms2.2.dylib /usr/local/lib/liblcms2.2.dylib \
    /usr/lib/x86_64-linux-gnu/liblcms2.so.2 /usr/lib/aarch64-linux-gnu/liblcms2.so.2; do
    if [ -e "$candidate" ]; then export LCMS2_LIB_PATH="$candidate"; break; fi
  done
fi

if command -v uv >/dev/null; then
  python=(uv run -q --no-project --with numpy python)
else
  python=(python3)
fi

status=0
for level in 10 5; do
  (cd "$runner" && "${python[@]}" conformance.py --decoder "$decoder" \
    --corpus "$work/corpus/testcases/main_level$level.txt" \
    --results "$work/level$level.json" >"$work/level$level.log" 2>&1) || status=1
  echo "## Level $level"
  python3 -I "$here/corpus.py" summarize "$work/level$level.json" || status=1
done

# Ferrisma's safe wrapper, as the engine builds it, in the runner's
# application (--lax) mode over every case. Its by-design rejections
# (animation, non-alpha extra channels, float samples) appear as decoder
# errors and do not change the exit status of the codec run above.
root="$(cd "$port/../.." && pwd)"
if [ -d "$root/crates/ferrisma-jxl-decoder" ]; then
  (cd "$root" && cargo build --release --locked -p ferrisma-jxl-decoder --example jxl_conformance_wrapper)
  sort -u "$work/corpus/testcases/main_level10.txt" "$work/corpus/testcases/main_level5.txt" \
    >"$work/corpus/testcases/all.txt"
  (cd "$runner" && "${python[@]}" conformance.py --lax \
    --decoder "$(dirname "$decoder")/jxl_conformance_wrapper" \
    --corpus "$work/corpus/testcases/all.txt" \
    --results "$work/wrapper-lax.json" >"$work/wrapper-lax.log" 2>&1) || true
  echo "## Ferrisma wrapper (--lax)"
  python3 -I "$here/corpus.py" summarize "$work/wrapper-lax.json" || true
fi
exit "$status"
