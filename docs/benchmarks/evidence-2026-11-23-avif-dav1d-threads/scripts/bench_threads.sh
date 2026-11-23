#!/usr/bin/env bash
# bench_threads.sh OUT BIN_DIR INPUTS "THREADS..." [REPS]
# Times the engine's safe reader (decode_still) on the 16 inputs of the
# 2026-11-22 decoder record:
#   new-dav1d:tT   this branch (dav1d decode on T threads, RGB rows on T)
#   main-dav1d:tT  origin/main reader (dav1d decode on 1 thread, RGB rows on T)
#   new-libaom:tT  this branch, libaom (decode on 1 thread, RGB rows on T; unchanged path)
# with run_interleaved.py (fresh process per sample, alternating order,
# 1 warm-up). Each decode_still writes RGB8 PPM; run_interleaved hashes it.
# Then a plane pass: zz_planes_threads (scratch decode_planes with codec and
# thread arguments) writes YUV planes and full-depth RGB per thread count.
set -euo pipefail
OUT=$1; BIN=$2; INPUTS=$3; THREADS=$4; REPS=${5:-7}
WT=${WT:-$(cd "$(dirname "$0")/../../../../.wt/avif-dav1d-threads" 2>/dev/null && pwd || echo .)}
RUNNER=${RUNNER:-$WT/scripts/bench/ci-perf/run_interleaved.py}
mkdir -p "$OUT/work"
h() { if command -v sha256sum >/dev/null; then sha256sum "$1" | cut -c1-64; else shasum -a 256 "$1" | cut -c1-64; fi; }
{ uname -a; date -u +%FT%TZ; uptime; nproc 2>/dev/null || sysctl -n hw.ncpu; } > "$OUT/host.txt"
for f in "$INPUTS"/photo*.avif; do
  name=$(basename "$f" .avif)
  python3 -I - "$f" "$name" "$BIN" "$OUT" "$THREADS" "$REPS" > "$OUT/work/plan-$name.json" <<'EOF'
import json, sys
f, name, bin_, out, threads, reps = sys.argv[1:]
cands = []
for t in threads.split():
    for cid, exe, codec in (("new-dav1d", "decode_still-new", "dav1d"),
                            ("main-dav1d", "decode_still-main", "dav1d"),
                            ("new-libaom", "decode_still-new", "libaom")):
        if cid != "new-dav1d" and t not in (threads.split()[0], threads.split()[-1]):
            continue
        o = f"{out}/work/{name}-{cid}-t{t}.ppm"
        cands.append({"id": f"{cid}:t{t}", "cmd": [f"{bin_}/{exe}", f, o, codec, t],
                      "output": o, "meta": {"threads": int(t), "codec": codec}})
print(json.dumps({"reps": int(reps), "warmups": 1, "group": name, "candidates": cands}))
EOF
  python3 -I "$RUNNER" "$OUT/work/plan-$name.json" "$OUT/threads-$name.json" 2> "$OUT/work/log-$name.txt"
  rm -f "$OUT"/work/*.ppm
done
# Plane identity: YUV planes and full-depth RGB per thread count.
: > "$OUT/planes.txt"
for f in "$INPUTS"/photo*.avif; do
  name=$(basename "$f" .avif)
  for codec in dav1d; do
    for t in $THREADS; do
      "$BIN/zz_planes_threads" "$f" "$OUT/work/p.raw" "$OUT/work/r.raw" "$codec" "$t" > /dev/null
      echo "$name $codec t$t planes=$(h "$OUT/work/p.raw") rgb=$(h "$OUT/work/r.raw")" >> "$OUT/planes.txt"
    done
  done
  "$BIN/zz_planes_threads" "$f" "$OUT/work/p.raw" "$OUT/work/r.raw" aom 1 > /dev/null
  echo "$name aom t1 planes=$(h "$OUT/work/p.raw") rgb=$(h "$OUT/work/r.raw")" >> "$OUT/planes.txt"
done
rm -f "$OUT/work/p.raw" "$OUT/work/r.raw"
{ date -u +%FT%TZ; uptime; } >> "$OUT/host.txt"
echo done >> "$OUT/host.txt"
