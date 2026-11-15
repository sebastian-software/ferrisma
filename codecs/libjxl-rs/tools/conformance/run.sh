#!/usr/bin/env bash
# Run the pinned external JPEG XL conformance corpus against the Rust port.
#
#   tools/conformance/run.sh [--target <triple>] <workdir>
#
# --target builds the decoders for that Rust target triple (for example
# x86_64-apple-darwin, run under Rosetta 2 on Apple silicon; set
# ROSETTA_ADVERTISE_AVX=1 there, or the decoder does not detect AVX2). The
# decoder runs its default path (NEON on AArch64, AVX2 on x86-64);
# JXL_DECODER_TARGET=scalar forces the scalar path of the codec run. The
# selected path is printed, and every decode records it in
# <workdir>/decoder-path-level<N>.log.
#
# Requires network access (first run), python3, uv (for numpy) or a python3
# with numpy, and the lcms2 shared library (LCMS2_LIB_PATH, defaulting to the
# Homebrew or Debian location). The official runner scripts come from the
# verified corpus revision and are copied outside the download directory.
set -euo pipefail

here="$(cd "$(dirname "$0")" && pwd)"
port="$(cd "$here/../.." && pwd)"
target=()
triple=""
if [ "${1:-}" = "--target" ]; then
  triple="$2"
  target=(--target "$triple")
  shift 2
fi
work="$(mkdir -p "$1" && cd "$1" && pwd)"

python3 -I "$here/corpus.py" fetch "$work"

# Build from the port directory so its scalar rustflags apply.
(cd "$port" && cargo build --release --locked ${target[@]+"${target[@]}"} --example conformance_decode)
decoder="$(cd "$port" && cargo metadata --format-version 1 --no-deps |
  python3 -I -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')/${triple:+$triple/}release/examples/conformance_decode"
echo "## Decoder: ${triple:-host} target, JXL_DECODER_TARGET=${JXL_DECODER_TARGET:-unset}, path $("$decoder" --decoder_path)"

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
  paths="$work/decoder-path-level$level.log"
  rm -f "$paths"
  (cd "$runner" && JXL_CONFORMANCE_PATH_LOG="$paths" "${python[@]}" conformance.py --decoder "$decoder" \
    --corpus "$work/corpus/testcases/main_level$level.txt" \
    --results "$work/level$level.json" >"$work/level$level.log" 2>&1) || status=1
  echo "## Level $level"
  python3 -I "$here/corpus.py" summarize "$work/level$level.json" || status=1
  echo "Decoder path per decode (count, path, whether a SIMD kernel ran):"
  cut -f1 "$paths" | sort | uniq -c
done

# Ferrisma's safe wrapper, as the engine builds it, in the runner's
# application (--lax) mode over every case. Its by-design rejections
# (animation, non-alpha extra channels, float samples) appear as decoder
# errors and do not change the exit status of the codec run above.
root="$(cd "$port/../.." && pwd)"
if [ -d "$root/crates/ferrisma-jxl-decoder" ]; then
  (cd "$root" && cargo build --release --locked ${target[@]+"${target[@]}"} -p ferrisma-jxl-decoder --example jxl_conformance_wrapper)
  wrapper="$(cd "$root" && cargo metadata --format-version 1 --no-deps |
    python3 -I -c 'import json,sys; print(json.load(sys.stdin)["target_directory"])')/${triple:+$triple/}release/examples/jxl_conformance_wrapper"
  sort -u "$work/corpus/testcases/main_level10.txt" "$work/corpus/testcases/main_level5.txt" \
    >"$work/corpus/testcases/all.txt"
  (cd "$runner" && "${python[@]}" conformance.py --lax \
    --decoder "$wrapper" \
    --corpus "$work/corpus/testcases/all.txt" \
    --results "$work/wrapper-lax.json" >"$work/wrapper-lax.log" 2>&1) || true
  # JXL_DECODER_TARGET does not reach the wrapper: it decodes on the default path.
  echo "## Ferrisma wrapper (--lax, default decoder path)"
  python3 -I "$here/corpus.py" summarize "$work/wrapper-lax.json" || true
fi
exit "$status"
