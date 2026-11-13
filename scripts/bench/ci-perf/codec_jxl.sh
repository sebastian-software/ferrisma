#!/usr/bin/env bash
# CI performance comparison: libjxl-rs (the Rust port, package libjxl-rs, lib
# `jxl`) and jxl-oxide versus the pinned libjxl it translates
# (codecs/libjxl-rs/upstream/libjxl, v0.12-snapshot-50-ga16d32d0, "v0.13.0").
#
#   INPUTS=dir [SIZES="12 24"] [WORK=dir] [OUT=dir] [REPS=7] [WARMUPS=1] [JOBS=n] \
#     [JXL_ENCODE_SIZES=12] [JXL_ENCODE_REPS=3] [JXL_SCALAR_REF=1] [OPS=decode,encode] \
#     bash scripts/bench/ci-perf/codec_jxl.sh
#
# Candidates follow decision D2 (three columns against the original):
#   c-scalar:t1  (a) build-scalar (HWY_COMPILE_ONLY_SCALAR), --num_threads=0
#   c:t1         (b) build-simd restricted to the D6 scope, --num_threads=0
#   c:tN         (c) build-simd, default runner (one worker per CPU)
#   port:t1      the port, serial
#   oxide:t1     context: jxl-oxide 0.12.6 without rayon (decode only)
#
# Operations (all candidates of one operation and size in ONE interleaved plan):
#   decode-vardct    photoN-e7d1.jxl (made by the original `cjxl -e 7 -d 1`)
#   decode-lossless  photoN-d0e7.jxl (made by the original `cjxl -d 0 -e 7`)
#     C: djxl --disable_output [--num_threads=0]
#     port:t1  jxl_decode_compare <in> own full 1  (crates/ferrisma-jxl-decoder,
#              the safe bridge to libjxl-rs; the port decodes serially)
#     oxide:t1 jxl_decode_compare <in> oxide full 1
#   encode-lossy     photoN.ppm, effort 7, distance 1, bare codestream
#     C: cjxl -e 7 -d 1 [--num_threads=0]
#     port:t1  codecs/libjxl-rs/examples/encode_lossy.rs (public lossy VarDCT
#              encoder, cjxl's PPM call sequence)
#   The port encoder is serial and much slower than the original, so the
#   encode runs only for JXL_ENCODE_SIZES (default photo12; photo24 would take
#   several minutes per sample on a 4-core runner) with JXL_ENCODE_REPS
#   repetitions (default 3) and no warm-up.
#   encode-lossless  photoN.ppm, effort 2, distance 0 (the engine's lossless
#                    profile), every size, normal repetitions
#     C: cjxl -d 0 -e 2 --codestream_level=10 --buffering=0 [--num_threads=0]
#     port:t1  codecs/libjxl-rs/examples/encode_lossless.rs
#              (LosslessThunderConfig, the same fixed configuration)
#   OPS selects decode and/or encode operations (both by default).
#
# Timers. djxl and cjxl print only a throughput line on stderr
# ("W x H, X MP/s ..."); a bash wrapper (`exec` keeps the tool as the measured
# process, so wall time and max RSS are the tool's) turns it into
# `elapsed_ms=W*H/X`, millisecond-exact to the printed 3 decimals. djxl's timer
# covers DecompressJxlToPackedPixelFile (decode to interleaved float, since
# --disable_output offers only float formats); the port harness times inspect
# plus decode_into into an F32 buffer, and jxl-oxide times the render plus
# streaming every row into F32. File reading is outside all three timers.
# cjxl's timer covers EncodeImageJXL (PPM parsing and file writing excluded);
# encode_lossy times encoder creation to the last output byte, the same API
# sequence. djxl/cjxl t1 still run their thread-pool runner with zero workers
# (all work on the calling thread), the port has no runner at all.
#
# Builds (in $WORK):
#   build-simd    upstream CMake Release, Highway dynamic dispatch restricted
#                 to D6 (HWY_V1_SCOPE below), columns (b) and (c). Only tool/dependency selection differs from
#                 the defaults (no tests, benchmark, docs, JNI, sjpeg, OpenEXR,
#                 tcmalloc, GIF/JPEG/PNG; static libraries). Clang when present.
#   build-scalar  the differential-oracle configuration of
#                 codecs/libjxl-rs/docs/ENCODER_EXPANSION.md "Reference
#                 configuration": Clang, HWY_COMPILE_ONLY_SCALAR=1, upstream's
#                 own -fno-slp-vectorize -fno-vectorize, libc++ on Linux. Timed
#                 as column (a) and used for the byte-exact checks. Skipped
#                 (and recorded) without Clang, or with JXL_SCALAR_REF=0;
#                 built with libstdc++ (recorded) when libc++ is missing.
#                 tools/oracle/CMakeLists.txt itself builds an adapter
#                 executable, not cjxl; this mirrors its global flags only (not
#                 its per-file oracle observation flags).
#
# Outputs in $OUT: jxl-build.json, jxl-inputs.json, jxl-<op>-<size>.json
# (run_interleaved.py) and jxl-checks.json (encoder byte identity against the
# SIMD and the scalar reference cjxl, decoder agreement port vs jxl-oxide, and
# the original's lossless round trip). Mismatches are recorded, never fatal;
# the script exits non-zero only when a build fails.
#
# System packages: cmake, ninja (optional), clang (the SIMD build falls back
#   to the default C/C++ compiler without it), libc++ for the Linux scalar
#   reference, python3, GNU time on Linux, Rust (the repository toolchain).
#   apt: cmake ninja-build clang libc++-dev libc++abi-dev python3 time
#   brew: cmake ninja python (Apple Clang and libc++ come with Xcode)
# Submodules: codecs/libjxl-rs/upstream/libjxl and its third_party/highway,
#   third_party/brotli and third_party/skcms (initialized here).
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${INPUTS:?set INPUTS}"
SIZES=${SIZES:-"12 24"}
WORK=${WORK:-$ROOT/target/ci-perf/jxl-work}
OUT=${OUT:-$ROOT/target/ci-perf/jxl-out}
REPS=${REPS:-7}
WARMUPS=${WARMUPS:-1}
JOBS=${JOBS:-$(getconf _NPROCESSORS_ONLN)}
ENCODE_SIZES=${JXL_ENCODE_SIZES:-12}
ENCODE_REPS=${JXL_ENCODE_REPS:-3}
SCALAR_REF=${JXL_SCALAR_REF:-1}
OPS=${OPS:-decode,encode}
mkdir -p "$WORK" "$OUT"
WORK=$(cd "$WORK" && pwd)
OUT=$(cd "$OUT" && pwd)
INPUTS=$(cd "$INPUTS" && pwd)

UPSTREAM=$ROOT/codecs/libjxl-rs/upstream/libjxl
if [[ ! -f "$UPSTREAM/CMakeLists.txt" ]]; then
  git -C "$ROOT" submodule update --init codecs/libjxl-rs/upstream/libjxl
fi
for dep in highway brotli skcms; do
  if [[ -z "$(ls -A "$UPSTREAM/third_party/$dep" 2>/dev/null)" ]]; then
    git -C "$UPSTREAM" submodule update --init "third_party/$dep"
  fi
done

# 1. The original.
if command -v clang >/dev/null && command -v clang++ >/dev/null; then
  CC_JXL=$(command -v clang); CXX_JXL=$(command -v clang++)
else
  CC_JXL=$(command -v cc); CXX_JXL=$(command -v c++)
fi
GEN=()
command -v ninja >/dev/null && GEN=(-G Ninja)
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

build_libjxl() { # variant extra-cmake-flags...
  local variant=$1 start=$SECONDS
  shift
  cmake -S "$UPSTREAM" -B "$WORK/build-$variant" ${GEN[@]+"${GEN[@]}"} \
    -DCMAKE_C_COMPILER="$CC_JXL" -DCMAKE_CXX_COMPILER="$CXX_JXL" \
    "${CMAKE_FLAGS[@]}" "$@" >"$WORK/$variant-configure.log"
  cmake --build "$WORK/build-$variant" --parallel "$JOBS" --target cjxl djxl \
    >"$WORK/$variant-build.log"
  echo $((SECONDS - start)) >"$WORK/$variant-build-seconds"
}

# Column (b) of D2: the release build restricted to the v1.0 SIMD scope (D6)
# with libjxl's own Highway target switches: no AVX-512 (AVX3*) on x86-64; no
# SVE/SVE2 or NEON_BF16 on AArch64, so NEON is the highest target there.
# AVX3, AVX3_ZEN4 and AVX3_SPR are already off by upstream default; AVX3_DL
# and NEON_BF16 are not, and Highway picks SVE2_128 on Neoverse-N2.
HWY_V1_SCOPE=(-DJPEGXL_ENABLE_HWY_AVX3=OFF -DJPEGXL_ENABLE_HWY_AVX3_DL=OFF
  -DJPEGXL_ENABLE_HWY_AVX3_ZEN4=OFF -DJPEGXL_ENABLE_HWY_AVX3_SPR=OFF
  -DJPEGXL_ENABLE_HWY_SVE=OFF -DJPEGXL_ENABLE_HWY_SVE2=OFF -DJPEGXL_ENABLE_HWY_SVE_256=OFF
  -DJPEGXL_ENABLE_HWY_SVE2_128=OFF -DJPEGXL_ENABLE_HWY_NEON_BF16=OFF)
build_libjxl simd "${HWY_V1_SCOPE[@]}"
CJXL=$WORK/build-simd/tools/cjxl
DJXL=$WORK/build-simd/tools/djxl
python3 -I "$ROOT/scripts/bench/ci-perf/cpu_features.py" "$OUT/cpu-features.json"

SCALAR_FLAGS=()
SCALAR_STATUS=skipped
if [[ "$SCALAR_REF" == 1 && "$(basename "$CC_JXL")" == clang* ]]; then
  defs=-DHWY_COMPILE_ONLY_SCALAR=1
  cxx=$defs ld= status=built
  if [[ "$(uname -s)" == Linux ]]; then
    if echo 'int main(){}' | "$CXX_JXL" -stdlib=libc++ -x c++ - -o "$WORK/libcxx-probe" 2>/dev/null; then
      cxx="$defs -stdlib=libc++" ld=-stdlib=libc++
    else
      # libc++ matters for the modular palette sort order, not for this
      # lossy VarDCT check; build anyway and record the deviation.
      status="built (libstdc++: libc++ unavailable for $CXX_JXL)"
    fi
  fi
  SCALAR_FLAGS=(-DCMAKE_C_FLAGS="$defs" -DCMAKE_CXX_FLAGS="$cxx"
    -DCMAKE_EXE_LINKER_FLAGS="$ld")
  build_libjxl scalar "${SCALAR_FLAGS[@]}"
  SCALAR_STATUS=$status
elif [[ "$SCALAR_REF" == 1 ]]; then
  SCALAR_STATUS="skipped: the reference configuration needs Clang"
fi
CJXL_SCALAR=$WORK/build-scalar/tools/cjxl
DJXL_SCALAR=$WORK/build-scalar/tools/djxl

# 2. The port side: the repository release profile, no RUSTFLAGS.
(cd "$ROOT" && cargo build --release --locked -p libjxl-rs --example encode_lossy --example encode_lossless)
(cd "$ROOT" && cargo build --release --locked -p ferrisma-jxl-decoder --example jxl_decode_compare)
TARGET=${CARGO_TARGET_DIR:-$ROOT/target}
[[ "$TARGET" = /* ]] || TARGET=$ROOT/$TARGET
PORT_ENC=$TARGET/release/examples/encode_lossy
PORT_ENC_LOSSLESS=$TARGET/release/examples/encode_lossless
PORT_DEC=$TARGET/release/examples/jxl_decode_compare

python3 -I - "$OUT/jxl-build.json" "$WORK" "$UPSTREAM" "$CC_JXL" "$CXX_JXL" "$CJXL" \
  "$SCALAR_STATUS" "$PORT_ENC" "$PORT_DEC" "$OUT/cpu-features.json" "${CMAKE_FLAGS[@]}" \
  "${HWY_V1_SCOPE[@]}" -- ${SCALAR_FLAGS[@]+"${SCALAR_FLAGS[@]}"} <<'EOF'
import json, re, subprocess, sys
from pathlib import Path
out, work, upstream, cc, cxx, cjxl, scalar_status, port_enc, port_dec, cpu_path, *rest = sys.argv[1:]
split = rest.index("--")
flags, scalar_flags = rest[:split], rest[split + 1:]
cpu = json.loads(Path(cpu_path).read_text())
def run(*cmd):
    try:
        r = subprocess.run(cmd, capture_output=True, text=True)
        return (r.stdout + r.stderr).strip()
    except OSError as error:
        return f"unavailable: {error}"
def cache(variant, key):
    path = Path(work, f"build-{variant}", "CMakeCache.txt")
    if not path.exists():
        return None
    m = re.search(rf"^{key}:[A-Z]*=(.*)$", path.read_text(), re.M)
    return m.group(1) if m else None
def seconds(variant):
    p = Path(work, f"{variant}-build-seconds")
    return int(p.read_text()) if p.exists() else None
hwy = {k: cache("simd", f"JPEGXL_ENABLE_HWY_{k}") for k in
       ("SSE2", "SSE4", "AVX2", "AVX3", "AVX3_DL", "AVX3_ZEN4", "AVX3_SPR",
        "NEON", "NEON_BF16", "SVE", "SVE2", "SVE_256", "SVE2_128", "EMU128", "SCALAR")}
version = run(cjxl, "--version").splitlines()[:1]
# cjxl --version lists the Highway targets this CPU supports and the build
# compiled, best first, with the selected one in underscores ("[_AVX2_,...]").
targets = re.search(r"\[([^\]]*)\]", version[0]) if version else None
selected = None
if targets:
    names = [t.strip() for t in targets.group(1).split(",")]
    marked = [t.strip("_") for t in names if t.startswith("_")]
    selected = marked[0] if marked else names[0]
scalar_version = (run(str(Path(work, "build-scalar/tools/cjxl")), "--version").splitlines()[:1]
                  if scalar_status.startswith("built") else None)
record = {
    "codec": "jxl",
    "columns": {
        "a": {"id": "c-scalar:t1", "build": "HWY_COMPILE_ONLY_SCALAR, -fno-slp-vectorize "
              "-fno-vectorize (the differential-oracle configuration), --num_threads=0",
              "simd_level": "none (Highway SCALAR target)", "status": scalar_status,
              "cjxl_version": scalar_version},
        "b": {"id": "c:t1", "build": "CMake Release, Highway dynamic dispatch restricted to "
              "the D6 scope (no AVX3*, SVE*, NEON_BF16), --num_threads=0",
              "simd_level": selected, "cjxl_version": version,
              "cpu_v1_scope_level": cpu["v1_scope_level"],
              "cpu_above_v1_scope": cpu["above_v1_scope"]},
        "c": {"id": "c:tN", "threads": "all (default runner: one worker per CPU)"},
        "context": {"oxide:t1": "jxl-oxide 0.12.6 without rayon (decode only)"},
    },
    "original": {
        "project": "libjxl",
        "describe": run("git", "-C", upstream, "describe", "--tags", "--always"),
        "commit": run("git", "-C", upstream, "rev-parse", "HEAD"),
        "submodules": run("git", "-C", upstream, "submodule", "status",
                          "third_party/highway", "third_party/brotli",
                          "third_party/skcms").splitlines(),
        "cjxl_version": run(cjxl, "--version").splitlines()[:1],
        "cmake_flags": flags,
        "c_compiler": cc, "cxx_compiler": cxx,
        "compiler_version": run(cxx, "--version").splitlines()[:1],
        "cmake_version": run("cmake", "--version").splitlines()[:1],
        "cmake_cxx_flags": cache("simd", "CMAKE_CXX_FLAGS"),
        "cmake_cxx_flags_release": cache("simd", "CMAKE_CXX_FLAGS_RELEASE"),
        "hwy_targets_enabled": hwy,
        "build_seconds": seconds("simd"),
        "notes": "Upstream CMake Release, Highway dynamic dispatch restricted "
                 "to the v1.0 SIMD scope (D6) with the JPEGXL_ENABLE_HWY_* "
                 "switches in cmake_flags (runtime target selection; cjxl "
                 "--version names the selected target), static libraries, "
                 "tools only. Not the repository oracle, which is scalar.",
    },
    "scalar_reference": {
        "status": scalar_status,
        "extra_cmake_flags": scalar_flags,
        "cjxl_version": run(str(Path(work, "build-scalar/tools/cjxl")), "--version").splitlines()[:1]
            if scalar_status.startswith("built") else None,
        "build_seconds": seconds("scalar"),
        "notes": "Differential-oracle configuration (ENCODER_EXPANSION.md "
                 "'Reference configuration'): HWY_COMPILE_ONLY_SCALAR, upstream's "
                 "Clang -fno-slp-vectorize -fno-vectorize, libc++ on Linux. "
                 "Timed as column (a) of D2 and used for the byte-exact "
                 "encoder checks.",
    },
    "port": {
        "encoder": {"package": "libjxl-rs", "example": "encode_lossy", "binary": port_enc,
                    "cargo": "cargo build --release --locked -p libjxl-rs --example encode_lossy"},
        "decoder": {"package": "ferrisma-jxl-decoder", "example": "jxl_decode_compare",
                    "binary": port_dec,
                    "cargo": "cargo build --release --locked -p ferrisma-jxl-decoder "
                             "--example jxl_decode_compare",
                    "notes": "modes own (libjxl-rs) and oxide (jxl-oxide 0.12.6, "
                             "default-features = false: no rayon)"},
        "rustc": run("rustc", "-vV").splitlines(),
    },
}
Path(out).write_text(json.dumps(record, indent=2) + "\n")
EOF

# 3. Reference inputs made with the original (SIMD build, default threads).
for size in $SIZES; do
  master=$INPUTS/photo$size.ppm
  [[ -f "$master" ]] || { echo "missing $master" >&2; exit 1; }
  [[ -s "$WORK/photo$size-e7d1.jxl" ]] ||
    "$CJXL" "$master" "$WORK/photo$size-e7d1.jxl" -e 7 -d 1 --quiet
  [[ -s "$WORK/photo$size-d0e7.jxl" ]] ||
    "$CJXL" "$master" "$WORK/photo$size-d0e7.jxl" -e 7 -d 0 --quiet
done
python3 -I - "$OUT/jxl-inputs.json" "$WORK" $SIZES <<'EOF'
import hashlib, json, sys
from pathlib import Path
out, work, *sizes = sys.argv[1:]
rows = {}
for size in sizes:
    for name, how in ((f"photo{size}-e7d1.jxl", "cjxl -e 7 -d 1 (VarDCT)"),
                      (f"photo{size}-d0e7.jxl", "cjxl -e 7 -d 0 (lossless modular)")):
        data = Path(work, name).read_bytes()
        rows[name] = {"made_by": how, "bytes": len(data),
                      "sha256": hashlib.sha256(data).hexdigest()}
Path(out).write_text(json.dumps(rows, indent=2) + "\n")
EOF

# 4. Interleaved runs.
# Runs djxl/cjxl and turns the "W x H, X MP/s" stderr line into elapsed_ms=;
# all other stderr lines pass through.
WRAP='exec "$0" "$@" 2> >(awk '"'"'/^[0-9]+ x [0-9]+, / { h = $3; sub(/,$/, "", h); for (i = 4; i < NF; i++) if ($(i + 1) ~ /^MP\/s/) { printf "elapsed_ms=%.3f\n", $1 * h / 1e6 / $i * 1000; break } } { print > "/dev/stderr" }'"'"')'

run_plan() { # op size reps warmups candidates-json
  local op=$1 size=$2 plan=$WORK/plan-$1-$2.json
  python3 -I - "$plan" "$3" "$4" "jxl/$op/photo$size" "$5" <<'EOF'
import json, sys
from pathlib import Path
out, reps, warmups, group, candidates = sys.argv[1:]
plan = {"reps": int(reps), "warmups": int(warmups), "group": group,
        "candidates": json.loads(candidates)}
Path(out).write_text(json.dumps(plan, indent=2) + "\n")
EOF
  python3 -I "$ROOT/scripts/bench/ci-perf/run_interleaved.py" "$plan" "$OUT/jxl-$op-$size.json"
}

cand() { # id impl column threads op input notes output cmd...
  python3 -I - "$@" <<'EOF'
import json, sys
id_, impl, column, threads, op, input_, notes, output, *cmd = sys.argv[1:]
print(json.dumps({"id": id_, "cmd": cmd, "env": {}, "output": output or None,
                  "meta": {"codec": "jxl", "op": op, "input": input_, "impl": impl,
                           "column": column, "threads": int(threads), "notes": notes}}))
EOF
}
HAVE_SCALAR=0
[[ "$SCALAR_STATUS" == built* ]] && HAVE_SCALAR=1

NCPU=$(getconf _NPROCESSORS_ONLN)
if [[ "$OPS" == *decode* ]]; then
for size in $SIZES; do
  for op in decode-vardct decode-lossless; do
    case $op in
      decode-vardct) input=photo$size-e7d1.jxl ;;
      decode-lossless) input=photo$size-d0e7.jxl ;;
    esac
    in=$WORK/$input
    c_note="djxl --disable_output: decode to interleaved F32; elapsed_ms from djxl's MP/s line (DecompressJxlToPackedPixelFile, file read excluded)"
    scalar=""
    if [[ $HAVE_SCALAR == 1 ]]; then
      scalar="$(cand c-scalar:t1 c a 1 "$op" "$input" "$c_note; scalar build (HWY_COMPILE_ONLY_SCALAR); --num_threads=0" "" \
        bash -c "$WRAP" "$DJXL_SCALAR" "$in" --disable_output --num_threads=0),"
    fi
    cands="[$scalar
      $(cand c:t1 c b 1 "$op" "$input" "$c_note; --num_threads=0" "" \
        bash -c "$WRAP" "$DJXL" "$in" --disable_output --num_threads=0),
      $(cand c:tN c c "$NCPU" "$op" "$input" "$c_note; default runner" "" \
        bash -c "$WRAP" "$DJXL" "$in" --disable_output),
      $(cand port:t1 port port 1 "$op" "$input" "libjxl-rs via ferrisma-jxl-decoder: inspect + decode_into F32 (file read excluded); serial" "" \
        "$PORT_DEC" "$in" own full 1),
      $(cand oxide:t1 oxide context 1 "$op" "$input" "jxl-oxide 0.12.6 (no rayon): render + stream rows to F32 (file read excluded)" "" \
        "$PORT_DEC" "$in" oxide full 1)]"
    run_plan "$op" "$size" "$REPS" "$WARMUPS" "$cands"
  done
done
fi

if [[ "$OPS" == *encode* ]]; then
for size in $ENCODE_SIZES; do
  [[ " $SIZES " == *" $size "* ]] || continue
  master=$INPUTS/photo$size.ppm
  op=encode-lossy
  c_note="cjxl -e 7 -d 1; elapsed_ms from cjxl's MP/s line (EncodeImageJXL; PPM parse and file write excluded)"
  scalar=""
  if [[ $HAVE_SCALAR == 1 ]]; then
    scalar="$(cand c-scalar:t1 c a 1 "$op" "photo$size.ppm" "$c_note; scalar build (HWY_COMPILE_ONLY_SCALAR); --num_threads=0" "$WORK/enc-scalar-$size.jxl" \
      bash -c "$WRAP" "$CJXL_SCALAR" "$master" "$WORK/enc-scalar-$size.jxl" -e 7 -d 1 --num_threads=0),"
  fi
  cands="[$scalar
    $(cand c:t1 c b 1 "$op" "photo$size.ppm" "$c_note; --num_threads=0" "$WORK/enc-c-t1-$size.jxl" \
      bash -c "$WRAP" "$CJXL" "$master" "$WORK/enc-c-t1-$size.jxl" -e 7 -d 1 --num_threads=0),
    $(cand c:tN c c "$NCPU" "$op" "photo$size.ppm" "$c_note; default runner" "$WORK/enc-c-tN-$size.jxl" \
      bash -c "$WRAP" "$CJXL" "$master" "$WORK/enc-c-tN-$size.jxl" -e 7 -d 1),
    $(cand port:t1 port port 1 "$op" "photo$size.ppm" "encode_lossy: public lossy VarDCT encoder, cjxl's PPM call sequence without the explicit PROGRESSIVE_AC=0 (outside the admitted profile); timer from JxlEncoderCreate to the last output byte; serial" "$WORK/enc-port-$size.jxl" \
      "$PORT_ENC" "$master" "$WORK/enc-port-$size.jxl" 7 1)]"
  run_plan "$op" "$size" "$ENCODE_REPS" 0 "$cands"
done

# Lossless effort 2 (the engine's lossless JPEG XL profile, ADR-0039): cheap,
# so every size with the normal repetitions.
LOSSLESS_FLAGS=(-d 0 -e 2 --codestream_level=10 --buffering=0)
for size in $SIZES; do
  master=$INPUTS/photo$size.ppm
  op=encode-lossless
  c_note="cjxl -d 0 -e 2 --codestream_level=10 --buffering=0; elapsed_ms from cjxl's MP/s line (EncodeImageJXL; PPM parse and file write excluded)"
  scalar=""
  if [[ $HAVE_SCALAR == 1 ]]; then
    scalar="$(cand c-scalar:t1 c a 1 "$op" "photo$size.ppm" "$c_note; scalar build (HWY_COMPILE_ONLY_SCALAR); --num_threads=0" "$WORK/encll-scalar-$size.jxl" \
      bash -c "$WRAP" "$CJXL_SCALAR" "$master" "$WORK/encll-scalar-$size.jxl" "${LOSSLESS_FLAGS[@]}" --num_threads=0),"
  fi
  cands="[$scalar
    $(cand c:t1 c b 1 "$op" "photo$size.ppm" "$c_note; --num_threads=0" "$WORK/encll-c-t1-$size.jxl" \
      bash -c "$WRAP" "$CJXL" "$master" "$WORK/encll-c-t1-$size.jxl" "${LOSSLESS_FLAGS[@]}" --num_threads=0),
    $(cand c:tN c c "$NCPU" "$op" "photo$size.ppm" "$c_note; default runner" "$WORK/encll-c-tN-$size.jxl" \
      bash -c "$WRAP" "$CJXL" "$master" "$WORK/encll-c-tN-$size.jxl" "${LOSSLESS_FLAGS[@]}"),
    $(cand port:t1 port port 1 "$op" "photo$size.ppm" "encode_lossless: LosslessThunderConfig (SetLossless, Thunder/effort 2, level 10, buffering 0, sRGB), the engine's lossless profile; timer around configuration + encode_pixels; serial" "$WORK/encll-port-$size.jxl" \
      "$PORT_ENC_LOSSLESS" "$master" "$WORK/encll-port-$size.jxl")]"
  run_plan "$op" "$size" "$REPS" "$WARMUPS" "$cands"
done
fi

# 5. Correctness side checks (recorded, never fatal).
for size in $SIZES; do
  "$PORT_DEC" "$WORK/photo$size-e7d1.jxl" compare full 1 >"$WORK/compare-vardct-$size.txt" 2>&1 || true
  "$PORT_DEC" "$WORK/photo$size-d0e7.jxl" compare full 1 >"$WORK/compare-lossless-$size.txt" 2>&1 || true
  "$DJXL" "$WORK/photo$size-d0e7.jxl" "$WORK/roundtrip-$size.ppm" --quiet >/dev/null 2>&1 || true
done
python3 -I - "$OUT" "$WORK" "$INPUTS" "$SCALAR_STATUS" "$SIZES" "$ENCODE_SIZES" <<'EOF'
import hashlib, json, sys
from pathlib import Path
out, work, inputs, scalar_status, sizes, encode_sizes = sys.argv[1:]
work = Path(work)
def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest() if path.exists() else None
def ppm_raster(path):
    data = path.read_bytes()
    fields, pos = [], 0
    while len(fields) < 4:
        while data[pos:pos + 1].isspace():
            pos += 1
        start = pos
        while not data[pos:pos + 1].isspace():
            pos += 1
        fields.append(data[start:pos])
    return fields[1:3], data[pos + 1:]
checks = {"scalar_reference": scalar_status, "encode": {}, "decode": {}}
for size in encode_sizes.split():
    if size not in sizes.split():
        continue
    files = {k: work / f"enc-{k}-{size}.jxl" for k in ("port", "c-t1", "c-tN", "scalar")}
    hashes = {k: sha(p) for k, p in files.items()}
    sizes_b = {k: (p.stat().st_size if p.exists() else None) for k, p in files.items()}
    checks["encode"][f"photo{size}"] = {
        "settings": "effort 7, distance 1, 8-bit sRGB RGB PPM, bare codestream",
        "sha256": hashes, "bytes": sizes_b,
        "port_equals_scalar_reference": (hashes["port"] == hashes["scalar"]
                                         if hashes["port"] and hashes["scalar"] else None),
        "port_equals_simd_release": (hashes["port"] == hashes["c-t1"]
                                     if hashes["port"] and hashes["c-t1"] else None),
        "simd_t1_equals_simd_tN": (hashes["c-t1"] == hashes["c-tN"]
                                   if hashes["c-t1"] and hashes["c-tN"] else None),
        "notes": "The byte-exact gate is the scalar reference (the repository's "
                 "differential-oracle configuration); SIMD builds may choose "
                 "differently in float-heavy heuristics (ENCODER_EXPANSION.md).",
    }
checks["encode_lossless"] = {}
for size in sizes.split():
    files = {k: work / f"encll-{k}-{size}.jxl" for k in ("port", "c-t1", "c-tN", "scalar")}
    hashes = {k: sha(p) for k, p in files.items()}
    if not any(hashes.values()):
        continue
    checks["encode_lossless"][f"photo{size}"] = {
        "settings": "effort 2, distance 0, codestream level 10, buffering 0, 8-bit sRGB RGB PPM",
        "sha256": hashes,
        "bytes": {k: (p.stat().st_size if p.exists() else None) for k, p in files.items()},
        "port_equals_scalar_reference": (hashes["port"] == hashes["scalar"]
                                         if hashes["port"] and hashes["scalar"] else None),
        "port_equals_simd_release": (hashes["port"] == hashes["c-t1"]
                                     if hashes["port"] and hashes["c-t1"] else None),
        "simd_t1_equals_simd_tN": (hashes["c-t1"] == hashes["c-tN"]
                                   if hashes["c-t1"] and hashes["c-tN"] else None),
        "notes": "Recorded, not a gate: whether cjxl's flags reach the same "
                 "one-shot path as LosslessThunderConfig is what this compares.",
    }
for size in sizes.split():
    entry = {}
    for kind in ("vardct", "lossless"):
        text = (work / f"compare-{kind}-{size}.txt")
        line = text.read_text().strip().splitlines()[-1:] if text.exists() else []
        fields = dict(t.split("=", 1) for t in line[0].split() if "=" in t) if line else {}
        entry[f"port_vs_oxide_{kind}"] = {
            "raw": line[0] if line else None,
            "max_abs_diff": float(fields["max_abs_diff"]) if "max_abs_diff" in fields else None,
            "mean_abs_diff": float(fields["mean_abs_diff"]) if "mean_abs_diff" in fields else None,
            "notes": "F32 samples, jxl_decode_compare compare mode",
        }
    rt = work / f"roundtrip-{size}.ppm"
    master = Path(inputs, f"photo{size}.ppm")
    if rt.exists() and master.exists():
        (d1, r1), (d2, r2) = ppm_raster(rt), ppm_raster(master)
        entry["c_lossless_roundtrip_identical"] = d1 == d2 and r1 == r2
    else:
        entry["c_lossless_roundtrip_identical"] = None
    checks["decode"][f"photo{size}"] = entry
Path(out, "jxl-checks.json").write_text(json.dumps(checks, indent=2) + "\n")
print(json.dumps(checks, indent=2))
EOF
