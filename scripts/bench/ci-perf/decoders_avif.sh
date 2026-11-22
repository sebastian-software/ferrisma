#!/usr/bin/env bash
# AVIF still decode: the port's two decoders against each other and against
# stock C, over bit depth, chroma subsampling, size and thread count
# (docs/benchmarks/v1.0-decoders-2026-11-22.md).
#
#   INPUTS=dir [CLASSES="8-420 8-444 10-420 10-444"] [SIZES="web03 web14 12 24"] \
#     [WORK=dir] [OUT=dir] [REPS=11] [WARMUPS=1] [JOBS=n] [SKIP_BUILD=1] \
#     bash scripts/bench/ci-perf/decoders_avif.sh
#
# INPUTS holds photo12.ppm and photo24.ppm (scripts/bench/ci-perf/make_masters.sh).
# web14 and web03 are photo12 box-filtered by 3 (1344x1008) and 6 (672x504).
#
# Builds: codec_avif.sh with AVIF_BUILD_ONLY=1 (the same C originals, C
# drivers and port examples as the three-column codec job), so every
# candidate here is built exactly as there.
#
# Inputs: made by the ORIGINAL encoder (C encode_still, quality 60, speed 6,
# one thread, so one tile), one per depth x subsampling x size.
#
# Candidates (all in ONE interleaved plan per input, fresh process per sample):
#   c-aom:t1 / c-aom:tN            C libavif + libaom release, D6 scope
#   c-dav1d-v1:t1 / c-dav1d-v1:tN  C libavif + dav1d, asm masked to the v1.0 scope
#   c-dav1d:t1 / c-dav1d:tN        C libavif + dav1d, unrestricted asm
#   port-aom-reader:t1             ferrisma-avif-read (the engine's path; 8-bit only)
#   port-aom:t1 / port-aom:tN      libavif-rs decode_codec aom
#   port-dav1d:t1 / port-dav1d:tN  libavif-rs decode_codec dav1d
# tN = maxThreads = online CPUs. Timer: elapsed_ms = decoder create + parse +
# decode + YUV->RGB8 (nearest), the same region in every driver.
#
# Outputs in $OUT: avif-build.json, cpu-features.json (from codec_avif.sh),
# decoders-inputs.json, decoders-<size>-<class>.json (run_interleaved.py) and
# decoders-checks.json (decoded RGB identity per input across candidates).
set -euo pipefail
ROOT=$(cd "$(dirname "$0")/../../.." && pwd)
: "${INPUTS:?set INPUTS}"
CLASSES=${CLASSES:-"8-420 8-444 10-420 10-444"}
SIZES=${SIZES:-"web03 web14 12 24"}
WORK=${WORK:-$ROOT/target/ci-perf/decoders-work}
OUT=${OUT:-$ROOT/target/ci-perf/decoders-out}
REPS=${REPS:-11}
WARMUPS=${WARMUPS:-1}
NCPU=$(getconf _NPROCESSORS_ONLN)
mkdir -p "$WORK" "$OUT"
WORK=$(cd "$WORK" && pwd)
OUT=$(cd "$OUT" && pwd)
INPUTS=$(cd "$INPUTS" && pwd)

if [[ "${SKIP_BUILD:-0}" != 1 ]]; then
  AVIF_BUILD_ONLY=1 INPUTS="$INPUTS" WORK="$WORK/build" OUT="$OUT" \
    bash "$ROOT/scripts/bench/ci-perf/codec_avif.sh"
fi
C_ENC=$WORK/build/c-bin/encode_still
C_DEC=$WORK/build/c-bin/decode_still
EXAMPLES=${CARGO_TARGET_DIR:-$ROOT/target}/release/examples
[[ "$EXAMPLES" = /* ]] || EXAMPLES=$ROOT/$EXAMPLES
P_READER=$EXAMPLES/decode_still
P_CODEC=$EXAMPLES/decode_codec
[[ "$(uname -m)" == x86_64 ]] && DAV1D_V1_MASK=0x2f || DAV1D_V1_MASK=0x1

# Masters as BMP; the web sizes box-filtered from photo12.
for size in $SIZES; do
  case "$size" in
    web03 | web14)
      factor=$([[ "$size" == web14 ]] && echo 3 || echo 6)
      python3 -I - "$INPUTS/photo12.ppm" "$WORK/photo$size.ppm" "$factor" <<'EOF'
import sys
from pathlib import Path
src, dst, k = sys.argv[1], sys.argv[2], int(sys.argv[3])
data = Path(src).read_bytes()
fields, pos = [], 0
while len(fields) < 4:
    while data[pos:pos + 1].isspace():
        pos += 1
    end = pos
    while not data[end:end + 1].isspace():
        end += 1
    fields.append(data[pos:end])
    pos = end
assert fields[0] == b"P6" and fields[3] == b"255"
w, h = int(fields[1]), int(fields[2])
px = memoryview(data)[pos + 1:pos + 1 + w * h * 3]
ow, oh = w // k, h // k
out = bytearray(ow * oh * 3)
area = k * k
for oy in range(oh):
    acc = [0] * (ow * 3)
    for dy in range(k):
        row = px[(oy * k + dy) * w * 3:(oy * k + dy + 1) * w * 3]
        for c in range(3):
            for dx in range(k):
                vals = row[dx * 3 + c:ow * k * 3:k * 3]
                for ox, v in enumerate(vals):
                    acc[ox * 3 + c] += v
    out[oy * ow * 3:(oy + 1) * ow * 3] = bytes((a + area // 2) // area for a in acc)
Path(dst).write_bytes(b"P6\n%d %d\n255\n" % (ow, oh) + bytes(out))
EOF
      ppm=$WORK/photo$size.ppm ;;
    *) ppm=$INPUTS/photo$size.ppm ;;
  esac
  python3 -I "$ROOT/scripts/bench/ci-perf/ppm_to.py" bmp "$ppm" "$WORK/photo$size.bmp"
  for class in $CLASSES; do
    depth=${class%-*}
    extra=--threads=1
    [[ "${class#*-}" == 444 ]] && extra=--yuv444
    "$C_ENC" "$WORK/photo$size.bmp" "$WORK/photo$size-$class.avif" --quality=60 --speed=6 \
      --depth="$depth" "$extra" --threads=1 >/dev/null
  done
done
python3 -I - "$OUT/decoders-inputs.json" "$WORK" "$SIZES" "$CLASSES" <<'EOF'
import hashlib, json, sys
from pathlib import Path
out, work, sizes, classes = sys.argv[1], Path(sys.argv[2]), sys.argv[3].split(), sys.argv[4].split()
record = {"made_with": "C libavif+libaom release, encode_still.c --quality=60 --speed=6 --depth=D "
                       "[--yuv444] --threads=1 (one tile, BT.709/sRGB/BT.601 full range); web14/web03: "
                       "photo12 box-filtered by 3 and 6", "files": {}}
for size in sizes:
    for name in [f"photo{size}.bmp"] + [f"photo{size}-{c}.avif" for c in classes]:
        data = (work / name).read_bytes()
        record["files"][name] = {"bytes": len(data), "sha256": hashlib.sha256(data).hexdigest()}
Path(out).write_text(json.dumps(record, indent=2) + "\n")
EOF

candidate() { # candidate ID OUTPUT ENV_JSON CMD...
  python3 -I -c 'import json, sys; print(json.dumps({"id": sys.argv[1], "cmd": sys.argv[4:], "env": json.loads(sys.argv[3]), "output": sys.argv[2], "meta": {"codec": "avif", "op": "decode"}}))' "$@"
}
for size in $SIZES; do
  for class in $CLASSES; do
    in=$WORK/photo$size-$class.avif
    o=$WORK/dec-$size-$class
    mask="{\"DAV1D_CPU_MASK\": \"$DAV1D_V1_MASK\"}"
    cands=(
      "$(candidate c-aom:t1 "$o-c-aom-t1.ppm" '{}' "$C_DEC" aom 1 "$in" "$o-c-aom-t1.ppm")"
      "$(candidate c-aom:tN "$o-c-aom-tN.ppm" '{}' "$C_DEC" aom "$NCPU" "$in" "$o-c-aom-tN.ppm")"
      "$(candidate c-dav1d-v1:t1 "$o-c-dav1d-v1-t1.ppm" "$mask" "$C_DEC" dav1d 1 "$in" "$o-c-dav1d-v1-t1.ppm")"
      "$(candidate c-dav1d-v1:tN "$o-c-dav1d-v1-tN.ppm" "$mask" "$C_DEC" dav1d "$NCPU" "$in" "$o-c-dav1d-v1-tN.ppm")"
      "$(candidate c-dav1d:t1 "$o-c-dav1d-t1.ppm" '{}' "$C_DEC" dav1d 1 "$in" "$o-c-dav1d-t1.ppm")"
      "$(candidate c-dav1d:tN "$o-c-dav1d-tN.ppm" '{}' "$C_DEC" dav1d "$NCPU" "$in" "$o-c-dav1d-tN.ppm")"
      "$(candidate port-aom:t1 "$o-port-aom-t1.ppm" '{}' "$P_CODEC" aom 1 "$in" "$o-port-aom-t1.ppm")"
      "$(candidate port-aom:tN "$o-port-aom-tN.ppm" '{}' "$P_CODEC" aom "$NCPU" "$in" "$o-port-aom-tN.ppm")"
      "$(candidate port-dav1d:t1 "$o-port-dav1d-t1.ppm" '{}' "$P_CODEC" dav1d 1 "$in" "$o-port-dav1d-t1.ppm")"
      "$(candidate port-dav1d:tN "$o-port-dav1d-tN.ppm" '{}' "$P_CODEC" dav1d "$NCPU" "$in" "$o-port-dav1d-tN.ppm")"
    )
    [[ "${class%-*}" == 8 ]] && cands+=("$(candidate port-aom-reader:t1 "$o-port-aom-reader-t1.ppm" '{}' "$P_READER" "$in" "$o-port-aom-reader-t1.ppm")")
    python3 -I - "$WORK/plan-$size-$class.json" "$REPS" "$WARMUPS" "avif/decode/photo$size-$class" "${cands[@]}" <<'EOF'
import json, sys
out, reps, warmups, group, *candidates = sys.argv[1:]
open(out, "w").write(json.dumps({"reps": int(reps), "warmups": int(warmups), "group": group,
                                 "candidates": [json.loads(c) for c in candidates]}, indent=2) + "\n")
EOF
    echo "== $size $class: $(uptime)" >&2
    python3 -I "$ROOT/scripts/bench/ci-perf/run_interleaved.py" "$WORK/plan-$size-$class.json" \
      "$OUT/decoders-$size-$class.json"
  done
done

python3 -I - "$OUT" <<'EOF'
import json, sys
from pathlib import Path
out = Path(sys.argv[1])
checks = {}
for path in sorted(out.glob("decoders-*-*.json")):
    if path.name in ("decoders-inputs.json", "decoders-checks.json"):
        continue
    record = json.loads(path.read_text())
    hashes = {cid: s.get("output_sha256") for cid, s in record["summary"].items()}
    flat = {h for v in hashes.values() if v for h in v}
    checks[record["plan"]["group"]] = {
        "sha256": hashes, "all_identical": len(flat) == 1 and all(hashes.values())}
(out / "decoders-checks.json").write_text(json.dumps(checks, indent=2) + "\n")
bad = [g for g, c in checks.items() if not c["all_identical"]]
print(f"decoded RGB identity: {len(checks) - len(bad)} of {len(checks)} inputs identical across all candidates")
for g in bad:
    print(f"  DIFFERENT: {g}: {checks[g]['sha256']}")
EOF
