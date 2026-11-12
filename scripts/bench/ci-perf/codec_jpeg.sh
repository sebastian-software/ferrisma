#!/usr/bin/env bash
# JPEG decode: the libjpeg-turbo-rs port against pinned C libjpeg-turbo.
#
#   INPUTS=dir WORK=dir OUT=dir [SIZES="12 24"] [REPS=7] [WARMUPS=1] [JOBS=n] \
#     bash scripts/bench/ci-perf/codec_jpeg.sh
#
# Original: codecs/libjpeg-turbo-rs/tools/build_oracle.py variant "simd"
# (upstream CMake Release, WITH_SIMD=1/REQUIRE_SIMD=1, static; it keeps the
# oracle's -ffp-contract=off, which only affects the float IDCT, not the islow
# path timed here) and its bench probe tools/oracle/probes/wp9/bench.c, the C
# counterpart of the port's examples/bench.rs (ADR 0004). Both decode the file
# from memory REPEATS times through the full-image scanline API to RGB and print
# nanoseconds, decodes and an FNV-1a checksum; the checksums must agree.
#
# Inputs: photoN-q90.jpg encoded by the oracle's cjpeg -quality 90 (4:2:0
# baseline) from the masters, and photoN-original.jpg (the camera file).
# Packages: apt cmake nasm build-essential time; brew cmake. Submodule:
# codecs/libjpeg-turbo-rs/upstream/libjpeg-turbo.
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${INPUTS:?}" "${WORK:?}" "${OUT:?}"
SIZES=${SIZES:-"12 24"}
REPS=${REPS:-7}
WARMUPS=${WARMUPS:-1}
JOBS=${JOBS:-$(getconf _NPROCESSORS_ONLN)}
REPEATS=${JPEG_REPEATS:-3}
mkdir -p "$WORK" "$OUT"
PORT="$ROOT/codecs/libjpeg-turbo-rs"
if [[ ! -f "$PORT/upstream/libjpeg-turbo/src/jpeglib.h" ]]; then
  git -C "$ROOT" submodule update --init codecs/libjpeg-turbo-rs/upstream/libjpeg-turbo
fi

python3 - "$PORT" "$JOBS" "$OUT/jpeg-build.json" <<'EOF'
import json, sys
from pathlib import Path
sys.path.insert(0, sys.argv[1] + "/tools")
import build_oracle
directory = build_oracle.build("simd", sys.argv[2], fresh=False)
record = {"variant": "simd", "directory": str(directory),
          "oracle": json.loads((directory / "oracle.json").read_text())}
Path(sys.argv[3]).write_text(json.dumps(record, indent=2) + "\n")
EOF
ORACLE=$(python3 -c 'import json,sys; print(json.load(open(sys.argv[1]))["directory"])' "$OUT/jpeg-build.json")
PROBE="$ORACLE/probes/wp9/bench"
CJPEG="$ORACLE/libjpeg-turbo/cjpeg-static"
[[ -x "$PROBE" && -x "$CJPEG" ]] || { echo "oracle probe or cjpeg missing" >&2; exit 1; }

cargo build --release --locked -p libjpeg-turbo-rs --example bench
TARGET=${CARGO_TARGET_DIR:-$ROOT/target}
BENCH="$TARGET/release/examples/bench"

printf '[\n' > "$OUT/jpeg-inputs.json.tmp"
first=1
for size in $SIZES; do
  "$CJPEG" -quality 90 -outfile "$WORK/photo$size-q90.jpg" "$INPUTS/photo$size.ppm"
  for input in "$WORK/photo$size-q90.jpg" "$INPUTS/photo$size-original.jpg"; do
    name=$(basename "$input" .jpg)
    sha=$(python3 -c 'import hashlib,sys; print(hashlib.sha256(open(sys.argv[1],"rb").read()).hexdigest())' "$input")
    bytes=$(wc -c < "$input" | tr -d ' ')
    [[ $first == 1 ]] || printf ',\n' >> "$OUT/jpeg-inputs.json.tmp"
    first=0
    printf '  {"input": "%s", "bytes": %s, "sha256": "%s"}' "$name" "$bytes" "$sha" >> "$OUT/jpeg-inputs.json.tmp"
    python3 - "$name" "$input" "$PROBE" "$BENCH" "$REPEATS" "$REPS" "$WARMUPS" > "$WORK/plan-$name.json" <<'EOF'
import json, sys
name, path, probe, bench, repeats, reps, warmups = sys.argv[1:8]
meta = {"codec": "jpeg", "op": "decode", "input": name, "threads": 1,
        "notes": f"in-process: {repeats} decodes from memory to RGB per process; internal_ms per decode"}
print(json.dumps({"reps": int(reps), "warmups": int(warmups), "group": f"jpeg/decode/{name}",
    "candidates": [
        {"id": "c:t1", "cmd": [probe, "-repeats", repeats, path], "output": None,
         "meta": {**meta, "impl": "c"}},
        {"id": "port:t1", "cmd": [bench, "-repeats", repeats, path], "output": None,
         "meta": {**meta, "impl": "port"}}]}))
EOF
    python3 -I "$ROOT/scripts/bench/ci-perf/run_interleaved.py" "$WORK/plan-$name.json" "$OUT/jpeg-decode-$name.json"
  done
done
printf '\n]\n' >> "$OUT/jpeg-inputs.json.tmp"
mv "$OUT/jpeg-inputs.json.tmp" "$OUT/jpeg-inputs.json"

# Checksums must agree between C and port.
python3 - "$OUT" > "$OUT/jpeg-checks.json" <<'EOF'
import json, sys
from pathlib import Path
checks = []
for path in sorted(Path(sys.argv[1]).glob("jpeg-decode-*.json")):
    record = json.loads(path.read_text())
    sums = {}
    for s in record["samples"]:
        for line in s.get("stdout", []):
            for token in line.split():
                if token.startswith("checksum="):
                    sums.setdefault(s["id"], set()).add(token[9:])
    checks.append({"file": path.name, "checksums": {k: sorted(v) for k, v in sums.items()},
                   "identical": len({frozenset(v) for v in sums.values()}) == 1})
print(json.dumps(checks, indent=2))
EOF
cat "$OUT/jpeg-checks.json"
