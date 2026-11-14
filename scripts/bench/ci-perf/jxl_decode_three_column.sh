#!/usr/bin/env bash
# JPEG XL decode, port against the pinned libjxl, in the three columns of
# ADR-0041 section 6, through the same public-decoder call sequence on both
# sides (jxl_decode_bench/: a C harness linked against the original's two
# builds and its Rust translation against the port's public API).
#
#   WORK=dir OUT=dir [CONFORMANCE=dir] [CASES="bike ..."] [SIZES="12 24"] \
#     [REPS=9] [WARMUPS=1] [THREADS=n] bash scripts/bench/ci-perf/jxl_decode_three_column.sh
#
# Run codec_jxl.sh first with the same WORK (OPS=none is enough): it builds
# build-scalar (column (a)) and build-simd (columns (b), (c)) of the original
# and makes the photoN-e7d1.jxl / photoN-d0e7.jxl inputs from the masters.
# CONFORMANCE is a checkout of the libjxl conformance repository; each name
# in CASES is testcases/<name>/input.jxl.
#
# Tested on macOS AArch64 (Apple Clang, libc++); on Linux the scalar build
# links against libc++ (codec_jxl.sh), so set CC=clang CXX="clang++ -stdlib=libc++".
#
# Candidates per input (one interleaved plan, fresh process per sample):
#   c-scalar:t1   (a) original, scalar reference build, 0 workers
#   port-scalar:t1 (a) port, scalar path forced (set_decoder_simd_for_test)
#   c:t1          (b) original, SIMD build (NEON on AArch64), 0 workers
#   port:t1       (b) port, default selection (NEON on AArch64), 0 workers
#   c:tN          (c) original, SIMD build, THREADS workers
#   port:tN       (c) port, default selection, THREADS workers
#   djxl:t1       context: djxl --disable_output --num_threads=0 (SIMD
#                 build), the tool the earlier records timed
# Timer: both harnesses time JxlDecoderCreate to the final JXL_DEC_SUCCESS
# (runner creation, output allocation included; file read and output write
# excluded). Before timing, every harness candidate decodes each input once
# more and writes its F32 output; the SHA-256 of those files is the
# equivalence record (jxl-decode-equivalence.json).
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${WORK:?set WORK}" "${OUT:?set OUT}"
WORK=$(cd "$WORK" && pwd)
mkdir -p "$OUT"
OUT=$(cd "$OUT" && pwd)
SIZES=${SIZES:-"12 24"}
CASES=${CASES:-}
REPS=${REPS:-9}
WARMUPS=${WARMUPS:-1}
THREADS=${THREADS:-$(getconf _NPROCESSORS_ONLN)}
UPSTREAM=$ROOT/codecs/libjxl-rs/upstream/libjxl
HARNESS=$ROOT/scripts/bench/ci-perf/jxl_decode_bench

# 1. Harnesses.
for v in scalar simd; do
  B=$WORK/build-$v
  [[ -f $B/lib/libjxl.a ]] || { echo "missing $B (run codec_jxl.sh first)" >&2; exit 1; }
  ${CC:-cc} -O2 -DJXL_STATIC_DEFINE -DJXL_THREADS_STATIC_DEFINE -DJXL_CMS_STATIC_DEFINE \
    -I"$UPSTREAM/lib/include" -I"$B/lib/include" -c "$HARNESS/jxl_decode_bench.c" \
    -o "$WORK/jxl_decode_bench-$v.o"
  ${CXX:-c++} "$WORK/jxl_decode_bench-$v.o" "$B/lib/libjxl.a" "$B/lib/libjxl_threads.a" \
    "$B/lib/libjxl_cms.a" "$B/third_party/highway/libhwy.a" \
    "$B/third_party/brotli/libbrotlidec.a" "$B/third_party/brotli/libbrotlicommon.a" \
    -o "$WORK/jxl_decode_bench-$v"
done
C_SCALAR=$WORK/jxl_decode_bench-scalar
C_SIMD=$WORK/jxl_decode_bench-simd
DJXL=$WORK/build-simd/tools/djxl
PORT_TARGET=${PORT_TARGET_DIR:-$WORK/port-bench-target}
(cd "$HARNESS" && CARGO_TARGET_DIR=$PORT_TARGET cargo build --release --locked)
PORT=$PORT_TARGET/release/jxl-decode-bench

# 2. Inputs.
inputs=()
for size in $SIZES; do
  inputs+=("$WORK/photo$size-e7d1.jxl" "$WORK/photo$size-d0e7.jxl")
done
for name in $CASES; do
  inputs+=("${CONFORMANCE:?set CONFORMANCE}/testcases/$name/input.jxl")
done
label() { # path -> short input name
  case $1 in
    */testcases/*/input.jxl) basename "$(dirname "$1")" ;;
    *) basename "$1" .jxl ;;
  esac
}

cand() { # id column threads input cmd...
  python3 -I - "$@" <<'EOF'
import json, sys
id_, column, threads, input_, *cmd = sys.argv[1:]
print(json.dumps({"id": id_, "cmd": cmd, "env": {}, "output": None,
                  "meta": {"codec": "jxl", "op": "decode", "input": input_,
                           "column": column, "threads": int(threads)}}))
EOF
}
WRAP='exec "$0" "$@" 2> >(awk '"'"'/^[0-9]+ x [0-9]+, / { h = $3; sub(/,$/, "", h); for (i = 4; i < NF; i++) if ($(i + 1) ~ /^MP\/s/) { printf "elapsed_ms=%.3f\n", $1 * h / 1e6 / $i * 1000; break } } { print > "/dev/stderr" }'"'"')'

# 3. Equivalence: one untimed decode per harness candidate, F32 output hashed.
python3 -I - "$OUT/jxl-decode-equivalence.json" "$WORK" "$THREADS" "$C_SCALAR" "$C_SIMD" "$PORT" \
  "${inputs[@]}" <<'EOF'
import hashlib, json, subprocess, sys
from pathlib import Path
out, work, threads, c_scalar, c_simd, port, *inputs = sys.argv[1:]
runs = {
    "c-scalar:t1": [c_scalar, "{in}", "0", "{out}"],
    "port-scalar:t1": [port, "{in}", "0", "scalar", "{out}"],
    "c:t1": [c_simd, "{in}", "0", "{out}"],
    "port:t1": [port, "{in}", "0", "default", "{out}"],
    "c:tN": [c_simd, "{in}", threads, "{out}"],
    "port:tN": [port, "{in}", threads, "default", "{out}"],
}
record = {}
for path in inputs:
    name = Path(path).parent.name if Path(path).name == "input.jxl" else Path(path).stem
    rows = {}
    for cid, cmd in runs.items():
        dump = Path(work, "equiv.f32")
        r = subprocess.run([a.format(**{"in": path, "out": dump}) for a in cmd],
                           capture_output=True, text=True)
        line = r.stdout.strip().splitlines()[-1] if r.stdout.strip() else r.stderr[-300:]
        rows[cid] = {"returncode": r.returncode, "line": line,
                     "sha256": hashlib.sha256(dump.read_bytes()).hexdigest() if r.returncode == 0 else None,
                     "bytes": dump.stat().st_size if r.returncode == 0 else None}
        dump.unlink(missing_ok=True)
    data = Path(path).read_bytes()
    record[name] = {
        "file": path, "bytes": len(data), "sha256": hashlib.sha256(data).hexdigest(),
        "runs": rows,
        "a_scalar_port_equals_original": rows["port-scalar:t1"]["sha256"] == rows["c-scalar:t1"]["sha256"],
        "b_neon_port_equals_original": rows["port:t1"]["sha256"] == rows["c:t1"]["sha256"],
        "c_threads_port_equals_original": rows["port:tN"]["sha256"] == rows["c:tN"]["sha256"],
        "original_tN_equals_t1": rows["c:tN"]["sha256"] == rows["c:t1"]["sha256"],
        "port_tN_equals_t1": rows["port:tN"]["sha256"] == rows["port:t1"]["sha256"],
    }
    print(name, {k: v for k, v in record[name].items() if k.endswith(("original", "t1"))})
Path(out).write_text(json.dumps(record, indent=2) + "\n")
EOF

# 4. Timed, interleaved.
for in in "${inputs[@]}"; do
  name=$(label "$in")
  cands="[
    $(cand c-scalar:t1 a 1 "$name" "$C_SCALAR" "$in" 0),
    $(cand port-scalar:t1 a 1 "$name" "$PORT" "$in" 0 scalar),
    $(cand c:t1 b 1 "$name" "$C_SIMD" "$in" 0),
    $(cand port:t1 b 1 "$name" "$PORT" "$in" 0 default),
    $(cand c:tN c "$THREADS" "$name" "$C_SIMD" "$in" "$THREADS"),
    $(cand port:tN c "$THREADS" "$name" "$PORT" "$in" "$THREADS" default),
    $(cand djxl:t1 context 1 "$name" bash -c "$WRAP" "$DJXL" "$in" --disable_output --num_threads=0)]"
  plan=$WORK/plan-decode-$name.json
  python3 -I - "$plan" "$REPS" "$WARMUPS" "jxl/decode/$name" "$cands" <<'EOF'
import json, sys
from pathlib import Path
out, reps, warmups, group, candidates = sys.argv[1:]
Path(out).write_text(json.dumps({"reps": int(reps), "warmups": int(warmups), "group": group,
                                 "candidates": json.loads(candidates)}, indent=2) + "\n")
EOF
  python3 -I "$ROOT/scripts/bench/ci-perf/run_interleaved.py" "$plan" "$OUT/jxl-decode-$name.json"
done
