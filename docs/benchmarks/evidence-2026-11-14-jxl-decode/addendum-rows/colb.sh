#!/bin/bash
# colb.sh LABEL PORTBIN... : column (b) for photo24-e7d1 and bike, original NEON vs given port binaries, interleaved.
set -euo pipefail
LABEL=$1; shift
WT=/Users/sebastian/Workspace/ferrisma/.claude/worktrees/prisma-release-review-419bce/.wt/jxl-modular
WORK=$WT/target/ci-perf/rows-work
S=/private/tmp/claude-501/-Users-sebastian-Workspace-ferramenta--claude-worktrees-prisma-release-review-0f93b7/88d9f815-1b98-4162-a337-0d4852da4fb0/scratchpad
OUT=$S/rows/out-$LABEL; mkdir -p $OUT
C=$WORK/jxl_decode_bench-simd
for in in $WORK/photo24-e7d1.jxl $S/conformance/corpus/testcases/bike/input.jxl; do
  case $in in */bike/*) name=bike;; *) name=photo24-e7d1;; esac
  python3 -I - $OUT/plan-$name.json "$C" "$in" "$name" "$@" <<'PY'
import json, sys
out, c, inp, name, *ports = sys.argv[1:]
cands = [{"id": "c:t1", "cmd": [c, inp, "0"], "env": {}, "output": None, "meta": {"input": name}}]
for i, p in enumerate(ports):
    cands.append({"id": f"port{i}:t1", "cmd": [p, inp, "0", "default"], "env": {}, "output": None, "meta": {"input": name, "bin": p}})
json.dump({"reps": 9, "warmups": 1, "group": f"rows/{name}", "candidates": cands}, open(out, "w"), indent=2)
PY
  python3 -I $WT/scripts/bench/ci-perf/run_interleaved.py $OUT/plan-$name.json $OUT/$name.json >/dev/null 2>&1
  python3 -I - $OUT/$name.json <<'PY'
import json, sys
d = json.load(open(sys.argv[1]))
s = d["summary"]; c = s["c:t1"]["internal_median_ms"]
for k, v in s.items():
    print(f"{sys.argv[1].split("/")[-1]:20} {k:10} {v['internal_median_ms']:9.1f} ms ({v['internal_min_ms']:.1f}-{v.get('internal_max_ms', 0):.1f})  ratio {v['internal_median_ms']/c:.2f}")
PY
done
