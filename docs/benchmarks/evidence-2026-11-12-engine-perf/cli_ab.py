"""Interleaved before/after CLI runs; compares output hashes and report fields.

Usage: python3 -I cli_ab.py plan.json out.json [reps]
plan.json: {"binaries": {"before": path, "after": path},
            "cases": {name: [args after the binary, with {out} for the output]}}
Indicative; loaded host. Order alternates per repetition.
"""
import hashlib
import json
import os
import re
import statistics
import subprocess
import sys
import tempfile

plan = json.load(open(sys.argv[1]))
out = sys.argv[2]
reps = int(sys.argv[3]) if len(sys.argv) > 3 else 7
bins = plan["binaries"]
res = {}
tmp = tempfile.mkdtemp()
for r in range(reps):
    order = [(c, b) for c in plan["cases"] for b in bins]
    if r % 2:
        order.reverse()
    for case, b in order:
        index = list(plan["cases"]).index(case)
        target = os.path.join(tmp, f"{b}-{index}.{plan['ext']}")
        args = [a.replace("{out}", target) for a in plan["cases"][case]]
        load = os.getloadavg()[0]
        p = subprocess.run([bins[b], *args], capture_output=True, text=True, check=True)
        rep = dict(re.findall(r"(\w+)=(\S+)", p.stderr + p.stdout))
        digest = hashlib.sha256(open(target, "rb").read()).hexdigest()
        res.setdefault(case, {}).setdefault(b, []).append({**rep, "sha256": digest, "load": load})
print("| case | before decode ms | after decode ms | change | before total ms | after total ms | peak managed bytes before / after | output |")
print("| --- | ---: | ---: | ---: | ---: | ---: | --- | --- |")
loads = []
for case, d in res.items():
    m = {b: statistics.median(int(x["decode_us"]) for x in d[b]) / 1e3 for b in d}
    t = {b: statistics.median(int(x["total_us"]) for x in d[b]) / 1e3 for b in d}
    peaks = {b: sorted({x["peak_managed_bytes"] for x in d[b]}) for b in d}
    hashes = {x["sha256"] for b in d for x in d[b]}
    loads += [x["load"] for b in d for x in d[b]]
    print(f"| {case} | {m['before']:.1f} | {m['after']:.1f} | {m['after'] - m['before']:+.1f} | "
          f"{t['before']:.1f} | {t['after']:.1f} | {'/'.join(peaks['before'])} / {'/'.join(peaks['after'])} | "
          f"{'identical' if len(hashes) == 1 else 'DIFFERENT'} |")
print(f"load average (1 min) {min(loads):.1f}-{max(loads):.1f}")
json.dump(res, open(out, "w"), indent=0)
