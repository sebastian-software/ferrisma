#!/usr/bin/env bash
# CI performance comparison: libwebp-rs (the Rust port, package libwebp-rs,
# lib `webp`) versus the pinned libwebp v1.6.0 it translates.
#
#   INPUTS=dir [SIZES="12 24"] [WORK=dir] [OUT=dir] [REPS=7] [WARMUPS=1] [JOBS=n] \
#     bash scripts/bench/ci-perf/codec_webp.sh
#
# Operations (each with the C tool and the port in ONE interleaved plan):
#   decode-lossy     dwebp -ppm <photoN-q75.webp>      vs cli_bench decode
#   decode-lossless  dwebp -ppm <photoN-lossless.webp> vs cli_bench decode
#   encode-lossy     cwebp -q 75 <photoN.ppm>          vs cli_bench encode lossy
#   encode-lossless  cwebp -lossless <photoN.ppm>      vs cli_bench encode lossless
# Candidates (decision D2, three columns against the original):
#   c-scalar:t1  (a) the original built with WEBP_ENABLE_SIMD=OFF, one thread
#   c:t1         (b) the release build with SIMD (libwebp has nothing above
#                AVX2/NEON, so no restriction is needed), one thread
#   c:tN         (c) the release build with `-mt`: libwebp's only threading,
#                ONE extra worker thread (at most two threads, not all
#                cores); not for decode-lossless, whose decoder has none
#   port:t1      the port, one thread
#
# Timers: both sides print the same region. The C tools run with `-v`, and a
# bash wrapper (`exec` keeps the tool as the measured process, so wall time
# and max RSS are the tool's) turns "Time to decode picture" / "Time to
# encode picture" (stderr, millisecond resolution) into `elapsed_ms=`. The
# port example (codecs/libwebp-rs/examples/cli_bench.rs) times the same calls
# with std::time::Instant. Both read the input before the timer and write a
# file of the same kind (PPM from decode, WebP from encode).
#
# Inputs: the masters photoN.ppm of make_masters.sh. The WebP inputs are made
# with the original cwebp (`-q 75` and `-lossless`), so they are reference
# bitstreams. PPM is used (not PNG) so cwebp needs no libpng.
#
# Outputs in $OUT: webp-build.json (cmake flags, compiler, upstream commit,
# SIMD defines), webp-inputs.json, webp-<op>-<size>.json (run_interleaved.py)
# and webp-checks.json (encoder byte identity, decoded pixel identity).
#
# System packages: cmake, a C compiler, python3, bash, GNU time on Linux
#   (apt: cmake build-essential python3 time; brew: cmake python), Rust (the
#   repository toolchain).
# Submodules: codecs/libwebp-rs/upstream/libwebp (initialized here if absent).
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${INPUTS:?set INPUTS}"
SIZES=${SIZES:-"12 24"}
WORK=${WORK:-$ROOT/target/ci-perf/webp-work}
OUT=${OUT:-$ROOT/target/ci-perf/webp-out}
REPS=${REPS:-7}
WARMUPS=${WARMUPS:-1}
JOBS=${JOBS:-$(getconf _NPROCESSORS_ONLN)}
mkdir -p "$WORK" "$OUT"
WORK=$(cd "$WORK" && pwd)
OUT=$(cd "$OUT" && pwd)
INPUTS=$(cd "$INPUTS" && pwd)

UPSTREAM=$ROOT/codecs/libwebp-rs/upstream/libwebp
if [[ ! -f "$UPSTREAM/CMakeLists.txt" ]]; then
  git -C "$ROOT" submodule update --init codecs/libwebp-rs/upstream/libwebp
fi

# 1. The original: upstream CMake Release, upstream default SIMD and threads.
# Only tool selection differs from the defaults (fewer targets, no viewers).
BUILD=$WORK/libwebp-release
CMAKE_FLAGS=(-DCMAKE_BUILD_TYPE=Release -DWEBP_ENABLE_SIMD=ON -DWEBP_USE_THREAD=ON
  -DWEBP_BUILD_CWEBP=ON -DWEBP_BUILD_DWEBP=ON -DWEBP_BUILD_ANIM_UTILS=OFF
  -DWEBP_BUILD_GIF2WEBP=OFF -DWEBP_BUILD_IMG2WEBP=OFF -DWEBP_BUILD_VWEBP=OFF
  -DWEBP_BUILD_WEBPINFO=OFF -DWEBP_BUILD_WEBPMUX=OFF -DWEBP_BUILD_EXTRAS=OFF)
cmake -S "$UPSTREAM" -B "$BUILD" "${CMAKE_FLAGS[@]}" >"$WORK/cmake-configure.log"
cmake --build "$BUILD" --parallel "$JOBS" --target cwebp dwebp >"$WORK/cmake-build.log"
CWEBP=$BUILD/cwebp
DWEBP=$BUILD/dwebp
# Column (a) of D2: the same configuration with WEBP_ENABLE_SIMD=OFF. CMake
# then defines no WEBP_HAVE_<SIMD> in config.h, so cpu.h selects no SIMD
# code at all (the `-noasm` switch is not a scalar build: on x86-64 and
# AArch64 libwebp omits the C versions of kernels whose SSE2/NEON variant is
# baseline). On x86-64 CMake also adds -mno-sse2/-mno-sse4.1/-mno-avx2.
BUILD_SCALAR=$WORK/libwebp-scalar
SCALAR_FLAGS=("${CMAKE_FLAGS[@]/-DWEBP_ENABLE_SIMD=ON/-DWEBP_ENABLE_SIMD=OFF}")
cmake -S "$UPSTREAM" -B "$BUILD_SCALAR" "${SCALAR_FLAGS[@]}" >"$WORK/cmake-scalar-configure.log"
cmake --build "$BUILD_SCALAR" --parallel "$JOBS" --target cwebp dwebp >"$WORK/cmake-scalar-build.log"
CWEBP_SCALAR=$BUILD_SCALAR/cwebp
DWEBP_SCALAR=$BUILD_SCALAR/dwebp
python3 -I "$ROOT/scripts/bench/ci-perf/cpu_features.py" "$OUT/cpu-features.json"

# 2. The port: the repository release profile, no RUSTFLAGS.
(cd "$ROOT" && cargo build --release --locked -p libwebp-rs --example cli_bench)
PORT=${CARGO_TARGET_DIR:-$ROOT/target}/release/examples/cli_bench
[[ "$PORT" = /* ]] || PORT=$ROOT/$PORT

CC_PATH=$(sed -n 's/^CMAKE_C_COMPILER:[A-Z]*=//p' "$BUILD/CMakeCache.txt")
python3 -I - "$OUT/webp-build.json" "$BUILD" "$UPSTREAM" "$CC_PATH" "$PORT" "$BUILD_SCALAR" \
  "$OUT/cpu-features.json" "${CMAKE_FLAGS[@]}" <<'EOF'
import json, re, subprocess, sys
from pathlib import Path
out, build, upstream, cc, port, build_scalar, cpu_path, *flags = sys.argv[1:]
cpu = json.loads(Path(cpu_path).read_text())
def run(*cmd):
    try:
        return subprocess.run(cmd, capture_output=True, text=True).stdout.strip()
    except OSError as error:
        return f"unavailable: {error}"
cache = Path(build, "CMakeCache.txt").read_text()
def cache_value(key):
    m = re.search(rf"^{key}:[A-Z]*=(.*)$", cache, re.M)
    return m.group(1) if m else None
def simd_defines(directory):
    config = Path(directory, "src/webp/config.h")
    return (re.findall(r"#define (WEBP_HAVE_(?:SSE\w*|AVX\w*|NEON\w*|MIPS\w*|MSA))\b",
                       config.read_text()) if config.exists() else [])
simd = simd_defines(build)
scalar_cache = Path(build_scalar, "CMakeCache.txt").read_text()
m = re.search(r"^CMAKE_C_FLAGS:[A-Z]*=(.*)$", scalar_cache, re.M)
level = cpu["v1_scope_level"]
if level == "AVX2":
    level = "AVX2 (SSE4.1/SSE2 for kernels without an AVX2 variant)"
record = {
    "codec": "webp",
    "columns": {
        "a": {"id": "c-scalar:t1", "build": "CMake Release, WEBP_ENABLE_SIMD=OFF",
              "simd_level": "none (C code only)", "simd_defines": simd_defines(build_scalar),
              "cmake_c_flags": m.group(1) if m else None},
        "b": {"id": "c:t1", "build": "CMake Release, WEBP_ENABLE_SIMD=ON",
              "simd_level": level,
              "restriction": "none needed: libwebp 1.6.0 has no SIMD above AVX2 or NEON"},
        "c": {"id": "c:tN", "threads": 2,
              "notes": "-mt: libwebp's only threading, one extra worker thread; "
                       "the lossless decoder has none (n/a)"},
    },
    "original": {
        "project": "libwebp", "tag": "v1.6.0",
        "commit": run("git", "-C", upstream, "rev-parse", "HEAD"),
        "cmake_flags": flags,
        "cmake_c_flags": cache_value("CMAKE_C_FLAGS"),
        "cmake_c_flags_release": cache_value("CMAKE_C_FLAGS_RELEASE"),
        "c_compiler": cc, "c_compiler_version": run(cc, "--version").splitlines()[:1],
        "cmake_version": run("cmake", "--version").splitlines()[:1],
        "simd_defines": simd,
        "notes": "Upstream CMake Release with default SIMD and threads; not the "
                 "repository oracle (tools/build_oracle.py builds probes with "
                 "-ffp-contract=off and SIMD off, or release-simd with oracle flags).",
    },
    "port": {
        "package": "libwebp-rs", "example": "cli_bench", "binary": port,
        "cargo": "cargo build --release --locked -p libwebp-rs --example cli_bench",
        "features": "default (simd: runtime-dispatched SSE2/SSE4.1/AVX2/NEON)",
        "rustc": run("rustc", "-vV").splitlines(),
    },
}
Path(out).write_text(json.dumps(record, indent=2) + "\n")
EOF

# 3. Reference inputs made with the original cwebp.
for size in $SIZES; do
  master=$INPUTS/photo$size.ppm
  [[ -f "$master" ]] || { echo "missing $master" >&2; exit 1; }
  [[ -f "$WORK/photo$size-q75.webp" ]] || "$CWEBP" -quiet -q 75 "$master" -o "$WORK/photo$size-q75.webp"
  [[ -f "$WORK/photo$size-lossless.webp" ]] || "$CWEBP" -quiet -lossless "$master" -o "$WORK/photo$size-lossless.webp"
done

# 4. Interleaved runs.
RESULTS=$WORK/results
mkdir -p "$RESULTS"
# Runs a C tool with -v and prints its "Time to ... picture" as elapsed_ms=;
# other stderr lines pass through to stderr.
WRAP='exec "$0" "$@" -v 2> >(awk '"'"'/^Time to (de|en)code picture:/ { t = $5; sub(/s$/, "", t); printf "elapsed_ms=%.3f\n", t * 1000; next } { print > "/dev/stderr" }'"'"')'

run_op() { # op size
  local op=$1 size=$2 input kind
  case "$op" in
    decode-lossy) input=$WORK/photo$size-q75.webp; kind=ppm ;;
    decode-lossless) input=$WORK/photo$size-lossless.webp; kind=ppm ;;
    encode-lossy | encode-lossless) input=$INPUTS/photo$size.ppm; kind=webp ;;
  esac
  python3 -I - "$WORK/plan-$op-$size.json" "$op" "$size" "$input" "$kind" "$RESULTS" \
      "$CWEBP" "$DWEBP" "$PORT" "$WRAP" "$REPS" "$WARMUPS" "$CWEBP_SCALAR" "$DWEBP_SCALAR" <<'EOF'
import json, sys
(plan_path, op, size, input_, kind, results, cwebp, dwebp, port, wrap, reps, warmups,
 cwebp_scalar, dwebp_scalar) = sys.argv[1:]
# D2 columns: (a) scalar build, one thread; (b) SIMD build, one thread;
# (c) SIMD build with -mt (libwebp's only threading: one extra worker), not
# for the lossless decoder, which has no threading. The port runs one thread.
rows = [("c-scalar:t1", "c", "a", False), ("c:t1", "c", "b", False)]
if op != "decode-lossless":
    rows.append(("c:tN", "c", "c", True))
rows.append(("port:t1", "port", "port", False))
candidates = []
for cid, impl, column, mt in rows:
    output = f"{results}/{op}-{size}-{cid.replace(':', '-')}.{kind}"
    scalar = column == "a"
    if op.startswith("decode"):
        if impl == "c":
            cmd = ["bash", "-c", wrap, dwebp_scalar if scalar else dwebp, "-ppm",
                   *(["-mt"] if mt else []), input_, "-o", output]
        else:
            cmd = [port, "decode", input_, output]
    else:
        lossless = op == "encode-lossless"
        if impl == "c":
            cmd = ["bash", "-c", wrap, cwebp_scalar if scalar else cwebp,
                   *(["-lossless"] if lossless else ["-q", "75"]),
                   *(["-mt"] if mt else []), input_, "-o", output]
        else:
            cmd = [port, "encode", input_, output, "lossless" if lossless else "lossy"]
    notes = ("internal timer: the -v 'Time to ... picture' region (C: DecodeWebP / "
             "WebPEncode with the file writer; ms resolution) and the same calls in the "
             "port example; wall time includes reading the input and writing the "
             f"{kind.upper()} output on both sides")
    if scalar:
        notes += "; WEBP_ENABLE_SIMD=OFF build"
    if mt:
        notes += "; -mt = libwebp's one extra worker thread"
    candidates.append({
        "id": cid, "cmd": cmd, "env": {}, "output": output,
        "meta": {"codec": "webp", "op": op, "column": column,
                 "input": input_.rsplit("/", 1)[-1], "impl": impl,
                 "threads": 2 if mt else 1, "notes": notes}})
plan = {"reps": int(reps), "warmups": int(warmups), "group": f"webp/{op}/photo{size}",
        "candidates": candidates}
if op == "decode-lossless":
    plan["column_c_na"] = "libwebp's lossless decoder has no threading"
with open(plan_path, "w") as f:
    json.dump(plan, f, indent=2)
EOF
  python3 -I "$ROOT/scripts/bench/ci-perf/run_interleaved.py" \
    "$WORK/plan-$op-$size.json" "$OUT/webp-$op-$size.json"
}

for size in $SIZES; do
  for op in decode-lossy decode-lossless encode-lossy encode-lossless; do
    run_op "$op" "$size"
  done
done

# 5. Correctness side-checks (recorded, never fatal).
python3 -I - "$OUT/webp-checks.json" "$OUT/webp-inputs.json" "$WORK" "$RESULTS" "$INPUTS" $SIZES <<'EOF'
import hashlib, json, sys
from pathlib import Path
checks_path, inputs_path, work, results, inputs, *sizes = sys.argv[1:]
def sha(p):
    return hashlib.sha256(Path(p).read_bytes()).hexdigest()
def pixels(p):
    # Payload after a P6 header without comments (dwebp, djpeg and the port).
    data = Path(p).read_bytes()
    fields, pos = 0, 0
    while fields < 4:
        while data[pos:pos + 1].isspace():
            pos += 1
        while not data[pos:pos + 1].isspace():
            pos += 1
        fields += 1
    return data[pos + 1:]
inputs_record, checks = {}, []
for size in sizes:
    for name in (f"photo{size}.ppm",):
        p = Path(inputs, name)
        inputs_record[name] = {"bytes": p.stat().st_size, "sha256": sha(p)}
    for name, cmd in ((f"photo{size}-q75.webp", "cwebp -q 75"),
                      (f"photo{size}-lossless.webp", "cwebp -lossless")):
        p = Path(work, name)
        inputs_record[name] = {"bytes": p.stat().st_size, "sha256": sha(p),
                               "made_with": f"original {cmd} photo{size}.ppm"}
    for op, kind, others in (("decode-lossy", "ppm", ("port-t1", "c-scalar-t1", "c-tN")),
                             ("decode-lossless", "ppm", ("port-t1", "c-scalar-t1")),
                             ("encode-lossy", "webp", ("port-t1", "c-scalar-t1", "c-tN")),
                             ("encode-lossless", "webp", ("port-t1", "c-scalar-t1", "c-tN"))):
        for other in others:
            c = Path(results, f"{op}-{size}-c-t1.{kind}")
            r = Path(results, f"{op}-{size}-{other}.{kind}")
            row = {"check": f"{op}/photo{size}: {other} output identical to c-t1 (SIMD, one thread)",
                   "c": str(c), "port": str(r)}
            if not (c.exists() and r.exists()):
                row.update(result="missing output")
            else:
                row.update(c_sha256=sha(c), port_sha256=sha(r), c_bytes=c.stat().st_size,
                           port_bytes=r.stat().st_size)
                row["result"] = "identical" if row["c_sha256"] == row["port_sha256"] else "DIFFERENT"
                if kind == "ppm" and row["result"] != "identical":
                    a, b = pixels(c), pixels(r)
                    row["max_abs_diff"] = max((abs(x - y) for x, y in zip(a, b)), default=None)
                    row["differing_samples"] = sum(x != y for x, y in zip(a, b))
            checks.append(row)
        if op == "encode-lossy" or op == "encode-lossless":
            ref = Path(work, f"photo{size}-{'q75' if op == 'encode-lossy' else 'lossless'}.webp")
            c = Path(results, f"{op}-{size}-c-t1.webp")
            if c.exists():
                checks.append({"check": f"{op}/photo{size}: C run reproduces the reference input",
                               "result": "identical" if sha(c) == sha(ref) else "DIFFERENT"})
    out = Path(results, f"decode-lossless-{size}-port-t1.ppm")  # cid port:t1
    if out.exists():
        same = pixels(out) == pixels(Path(inputs, f"photo{size}.ppm"))
        checks.append({"check": f"decode-lossless/photo{size}: port decode equals the master pixels",
                       "result": "identical" if same else "DIFFERENT"})
Path(inputs_path).write_text(json.dumps(inputs_record, indent=2) + "\n")
Path(checks_path).write_text(json.dumps({"codec": "webp", "checks": checks}, indent=2) + "\n")
bad = [c["check"] for c in checks if c["result"] != "identical"]
print(f"webp checks: {len(checks) - len(bad)}/{len(checks)} identical", file=sys.stderr)
for b in bad:
    print(f"  not identical: {b}", file=sys.stderr)
EOF
# Large decoded outputs are not needed after the checks.
rm -f "$RESULTS"/*.ppm
