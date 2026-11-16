#!/usr/bin/env bash
# Diagnostic profile (not a benchmark): where the port's JPEG XL lossy encode
# and VarDCT decode spend their time on this host, beside the original's SIMD
# release build, both on one thread. Run by .github/workflows/jxl-profile.yml.
#
#   INPUTS=dir [WORK=dir] [OUT=dir] [JOBS=n] [CROP=WxH] bash scripts/bench/ci-perf/jxl_profile.sh
#
# The builds are the benchmark's (scripts/bench/ci-perf/codec_jxl.sh): the
# original's upstream CMake Release build with Highway's dynamic dispatch
# capped at the v1.0 scope (no AVX3*, SVE*, NEON_BF16; the flag lists below
# are copied from codec_jxl.sh and must stay equal to them), and the port's
# repository release profile without RUSTFLAGS. Neither binary gets debug
# information or other flags: perf and callgrind symbolize from the symbol
# tables, so the profiled code is the benchmarked code. On an AVX-512 host
# both sides run AVX2 (the original's AVX3 targets are not compiled; the port
# never selects above AVX2).
#
# Inputs: photo12.ppm (make_masters.sh), photo12-e7d1.jxl made by the
# original (`cjxl -e 7 -d 1`, as the benchmark makes it), and a centered
# CROP of photo12.ppm (default 1536x1024) for callgrind.
#
# Records in $OUT:
#   host.txt, build.txt          lscpu, /proc/cpuinfo flags, kernel, compilers,
#                                CMake cache flags, cjxl/djxl --version
#                                (the selected Highway target), cargo commands
#   timing.txt                   three wall-clock runs per side and operation
#                                with the tools' own throughput lines, output
#                                hashes (port == original's SIMD build?)
#   perf-<case>-<side>.*         perf stat, perf record (cpu-clock, 4 kHz),
#                                perf report --stdio by symbol and by DSO,
#                                perf annotate of the hottest symbols
#   callgrind-<case>-<side>.*    instructions executed per function (Ir) on
#                                the crop, callgrind_annotate exclusive and
#                                inclusive
#   bin/                         the four profiled binaries (for offline
#                                disassembly at the profiled addresses)
# Failures of the optional profilers are recorded and never fatal.
set -uo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${INPUTS:?set INPUTS}"
WORK=${WORK:-$ROOT/target/jxl-profile/work}
OUT=${OUT:-$ROOT/target/jxl-profile/out}
JOBS=${JOBS:-$(getconf _NPROCESSORS_ONLN)}
CROP=${CROP:-1536x1024}
mkdir -p "$WORK" "$OUT/bin"
WORK=$(cd "$WORK" && pwd)
OUT=$(cd "$OUT" && pwd)
INPUTS=$(cd "$INPUTS" && pwd)
UPSTREAM=$ROOT/codecs/libjxl-rs/upstream/libjxl

# 1. The original, as codec_jxl.sh's build-simd (column b).
CC_JXL=$(command -v clang)
CXX_JXL=$(command -v clang++)
CMAKE_FLAGS=(-DCMAKE_BUILD_TYPE=Release -DCMAKE_POLICY_VERSION_MINIMUM=3.5
  -DBUILD_TESTING=OFF -DBUILD_SHARED_LIBS=OFF
  -DJPEGXL_ENABLE_TOOLS=ON -DJPEGXL_ENABLE_DEVTOOLS=OFF -DJPEGXL_ENABLE_BENCHMARK=OFF
  -DJPEGXL_ENABLE_EXAMPLES=OFF -DJPEGXL_ENABLE_DOXYGEN=OFF -DJPEGXL_ENABLE_MANPAGES=OFF
  -DJPEGXL_ENABLE_JNI=OFF -DJPEGXL_ENABLE_SJPEG=OFF -DJPEGXL_ENABLE_OPENEXR=OFF
  -DJPEGXL_ENABLE_SKCMS=ON -DJPEGXL_ENABLE_VIEWERS=OFF -DJPEGXL_ENABLE_PLUGINS=OFF
  -DJPEGXL_ENABLE_FUZZERS=OFF -DJPEGXL_ENABLE_TCMALLOC=OFF -DJPEGXL_BUNDLE_LIBPNG=OFF
  -DHWY_ENABLE_CONTRIB=OFF -DHWY_ENABLE_EXAMPLES=OFF
  -DCMAKE_DISABLE_FIND_PACKAGE_GIF=ON -DCMAKE_DISABLE_FIND_PACKAGE_JPEG=ON
  -DCMAKE_DISABLE_FIND_PACKAGE_PNG=ON -DCMAKE_DISABLE_FIND_PACKAGE_ZLIB=ON)
HWY_V1_SCOPE=(-DJPEGXL_ENABLE_HWY_AVX3=OFF -DJPEGXL_ENABLE_HWY_AVX3_DL=OFF
  -DJPEGXL_ENABLE_HWY_AVX3_ZEN4=OFF -DJPEGXL_ENABLE_HWY_AVX3_SPR=OFF
  -DJPEGXL_ENABLE_HWY_SVE=OFF -DJPEGXL_ENABLE_HWY_SVE2=OFF -DJPEGXL_ENABLE_HWY_SVE_256=OFF
  -DJPEGXL_ENABLE_HWY_SVE2_128=OFF -DJPEGXL_ENABLE_HWY_NEON_BF16=OFF)
cmake -S "$UPSTREAM" -B "$WORK/build-simd" -G Ninja \
  -DCMAKE_C_COMPILER="$CC_JXL" -DCMAKE_CXX_COMPILER="$CXX_JXL" -DCMAKE_EXPORT_COMPILE_COMMANDS=ON \
  "${CMAKE_FLAGS[@]}" "${HWY_V1_SCOPE[@]}" >"$WORK/simd-configure.log" || exit 1
cmake --build "$WORK/build-simd" --parallel "$JOBS" --target cjxl djxl >"$WORK/simd-build.log" || exit 1
CJXL=$WORK/build-simd/tools/cjxl
DJXL=$WORK/build-simd/tools/djxl

# 2. The port, as codec_jxl.sh builds it.
(cd "$ROOT" && cargo build --release --locked -p libjxl-rs --example encode_lossy) || exit 1
(cd "$ROOT" && cargo build --release --locked -p ferrisma-jxl-decoder --example jxl_decode_compare) || exit 1
TARGET=${CARGO_TARGET_DIR:-$ROOT/target}
[[ "$TARGET" = /* ]] || TARGET=$ROOT/$TARGET
PORT_ENC=$TARGET/release/examples/encode_lossy
PORT_DEC=$TARGET/release/examples/jxl_decode_compare
cp "$CJXL" "$DJXL" "$PORT_ENC" "$PORT_DEC" "$OUT/bin/"

# 3. Host and build record.
{
  echo "## uname"; uname -a
  echo "## lscpu"; lscpu
  echo "## /proc/cpuinfo flags (first CPU)"; grep -m1 '^flags' /proc/cpuinfo
  echo "## Highway AVX2 group (kGroupAVX2) present?"
  for f in sse2 ssse3 sse4_1 sse4_2 pclmulqdq aes avx avx2 bmi1 bmi2 fma f16c abm; do
    grep -m1 '^flags' /proc/cpuinfo | grep -qw "$f" && echo "  $f yes" || echo "  $f NO"
  done
  echo "## perf_event_paranoid"; cat /proc/sys/kernel/perf_event_paranoid
} >"$OUT/host.txt" 2>&1
{
  echo "## original"; git -C "$UPSTREAM" describe --tags --always; git -C "$UPSTREAM" rev-parse HEAD
  git -C "$UPSTREAM" submodule status third_party/highway third_party/brotli third_party/skcms
  echo "## cjxl --version (the selected Highway target is in underscores)"; "$CJXL" --version
  echo "## djxl --version"; "$DJXL" --version
  echo "## compiler"; "$CXX_JXL" --version
  echo "## CMake flags"; printf '%s\n' "${CMAKE_FLAGS[@]}" "${HWY_V1_SCOPE[@]}"
  echo "## CMakeCache"; grep -E '^(CMAKE_CXX_FLAGS|CMAKE_CXX_FLAGS_RELEASE|CMAKE_C_FLAGS|CMAKE_C_FLAGS_RELEASE|JPEGXL_ENABLE_HWY_[A-Z0-9_]+):' "$WORK/build-simd/CMakeCache.txt"
  echo "## compile command of lib/jxl/enc_ac_strategy.cc"
  python3 -I -c 'import json,sys; [print(e["command"]) for e in json.load(open(sys.argv[1])) if e["file"].endswith("lib/jxl/enc_ac_strategy.cc")]' \
    "$WORK/build-simd/compile_commands.json"
  echo "## port"; git -C "$ROOT" rev-parse HEAD; rustc -vV; cargo -V
  echo "cargo build --release --locked -p libjxl-rs --example encode_lossy"
  echo "cargo build --release --locked -p ferrisma-jxl-decoder --example jxl_decode_compare"
  echo "RUSTFLAGS=${RUSTFLAGS:-<unset>} CARGO_ENCODED_RUSTFLAGS=${CARGO_ENCODED_RUSTFLAGS:-<unset>}"
  sed -n '/^\[profile.release\]/,/^\[/p' "$ROOT/Cargo.toml"
} >"$OUT/build.txt" 2>&1

# 4. Inputs.
MASTER=$INPUTS/photo12.ppm
VARDCT=$WORK/photo12-e7d1.jxl
"$CJXL" "$MASTER" "$VARDCT" -e 7 -d 1 --quiet || exit 1
CROPPED=$WORK/crop.ppm
python3 -I - "$MASTER" "$CROPPED" "$CROP" <<'EOF' || exit 1
import sys
src, dst, crop = sys.argv[1:]
w, h = (int(v) for v in crop.split("x"))
data = open(src, "rb").read()
fields, pos = [], 0
while len(fields) < 4:
    while data[pos:pos + 1].isspace():
        pos += 1
    start = pos
    while not data[pos:pos + 1].isspace():
        pos += 1
    fields.append(data[start:pos])
pos += 1
W, H = int(fields[1]), int(fields[2])
assert fields[0] == b"P6" and int(fields[3]) == 255 and w <= W and h <= H
x0, y0 = (W - w) // 2, (H - h) // 2
rows = [data[pos + ((y0 + y) * W + x0) * 3: pos + ((y0 + y) * W + x0 + w) * 3] for y in range(h)]
open(dst, "wb").write(b"P6\n%d %d\n255\n" % (w, h) + b"".join(rows))
EOF

# The four profiled commands: <case> <side> <command...>, with %IN% and
# %OUTF% substituted. One thread on both sides (codec_jxl.sh's column b).
cmd_for() { # case side in out
  case "$1/$2" in
    enc/orig) echo "$CJXL $3 $4 -e 7 -d 1 --num_threads=0" ;;
    enc/port) echo "$PORT_ENC $3 $4 7 1" ;;
    dec/orig) echo "$DJXL $3 --disable_output --num_threads=0" ;;
    dec/port) echo "$PORT_DEC $3 own full 1" ;;
  esac
}

# 5. Timing and output identity (three runs each; diagnostic only).
{
  for case in enc dec; do
    for side in orig port; do
      in=$MASTER; [[ $case == dec ]] && in=$VARDCT
      for rep in 1 2 3; do
        echo "## $case $side run $rep"
        # shellcheck disable=SC2046
        /usr/bin/time -f "wall=%es user=%Us maxrss=%MkB" $(cmd_for $case $side "$in" "$WORK/$case-$side.jxl") 2>&1 | tail -5
      done
    done
  done
  echo "## encode output sha256 (port and original's SIMD build)"
  sha256sum "$WORK/enc-orig.jxl" "$WORK/enc-port.jxl"
  cmp -s "$WORK/enc-orig.jxl" "$WORK/enc-port.jxl" && echo "IDENTICAL" || echo "DIFFERENT"
} >"$OUT/timing.txt" 2>&1

# 6. perf (cpu-clock sampling works without hardware counters in a VM).
PERF=""
sudo sysctl -w kernel.perf_event_paranoid=-1 kernel.kptr_restrict=0 >>"$OUT/host.txt" 2>&1 || true
for candidate in "$(command -v perf 2>/dev/null)" /usr/lib/linux-tools/*/perf /usr/lib/linux-tools-*/perf; do
  if [[ -n "$candidate" && -x "$candidate" ]] && "$candidate" stat -e task-clock true >/dev/null 2>&1; then
    PERF=$candidate; break
  fi
done
echo "perf: ${PERF:-unavailable}" >>"$OUT/host.txt"
if [[ -n "$PERF" ]]; then
  "$PERF" version >>"$OUT/host.txt" 2>&1
  for case in enc dec; do
    for side in orig port; do
      in=$MASTER; [[ $case == dec ]] && in=$VARDCT
      base=$OUT/perf-$case-$side
      # shellcheck disable=SC2046
      "$PERF" stat -e task-clock,cycles,instructions,branches,branch-misses,L1-dcache-loads,L1-dcache-load-misses \
        -o "$base.stat.txt" -- $(cmd_for $case $side "$in" "$WORK/$case-$side-perf.jxl") >/dev/null 2>&1 || true
      # shellcheck disable=SC2046
      "$PERF" record -e cpu-clock -F 4000 -o "$base.data" -- $(cmd_for $case $side "$in" "$WORK/$case-$side-perf.jxl") \
        >"$base.record.log" 2>&1 || true
      "$PERF" report -i "$base.data" --stdio --no-children --sort symbol --percent-limit 0.1 \
        >"$base.symbols.txt" 2>&1 || true
      "$PERF" report -i "$base.data" --stdio --no-children --sort dso >"$base.dso.txt" 2>&1 || true
      # The hottest symbols, annotated at the instruction level.
      python3 -I - "$base.symbols.txt" >"$base.top.txt" <<'EOF' || true
import re, sys
n = 0
for line in open(sys.argv[1], errors="replace"):
    m = re.match(r"\s*([0-9.]+)%\s+\[\.\]\s+(.*\S)\s*$", line)
    if m and float(m.group(1)) >= 1.0 and n < 15:
        print(m.group(2)); n += 1
EOF
      i=0
      while IFS= read -r sym; do
        i=$((i + 1))
        { echo "### $sym"; "$PERF" annotate -i "$base.data" --stdio --symbol="$sym" 2>&1 | head -4000; } \
          >"$base.annotate-$i.txt" || true
      done <"$base.top.txt"
    done
  done
fi

# 7. callgrind on the crop: instructions executed per function.
if command -v valgrind >/dev/null; then
  for case in enc dec; do
    for side in orig port; do
      in=$CROPPED
      if [[ $case == dec ]]; then
        in=$WORK/crop-e7d1.jxl
        [[ -f "$in" ]] || "$CJXL" "$CROPPED" "$in" -e 7 -d 1 --quiet || continue
      fi
      base=$OUT/callgrind-$case-$side
      # shellcheck disable=SC2046
      valgrind --tool=callgrind --callgrind-out-file="$base.out" \
        $(cmd_for $case $side "$in" "$WORK/$case-$side-cg.jxl") >"$base.log" 2>&1 || true
      callgrind_annotate --threshold=99.5 "$base.out" >"$base.exclusive.txt" 2>&1 || true
      callgrind_annotate --threshold=99.5 --inclusive=yes "$base.out" >"$base.inclusive.txt" 2>&1 || true
    done
  done
  echo "## valgrind: cjxl --version under valgrind (the target Highway selects there)" >>"$OUT/host.txt"
  valgrind -q "$CJXL" --version >>"$OUT/host.txt" 2>&1 || true
fi
ls -l "$OUT"
