"""Interleaved before/after/C run of libjpeg-turbo-rs tools/bench.py cases.

Usage: python3 -I jpeg_ab.py <codec dir> <before> <after> <out.json> [runs]
Indicative; loaded host. Order alternates per run (forward, reversed).
"""
import json
import os
import statistics
import subprocess
import sys

codec, before, after, out = sys.argv[1:5]
runs = int(sys.argv[5]) if len(sys.argv) > 5 else 7
sys.path.insert(0, os.path.join(codec, "tools"))
import bench  # noqa: E402
import build_oracle  # noqa: E402

oracle = build_oracle.build("simd", "4", fresh=False)
bench.make_inputs(oracle)
probe = oracle / "probes/wp9/bench"
env = {k: v for k, v in os.environ.items() if not k.startswith("JSIMD_")}
impls = {"c": probe, "before": before, "after": after}
res = {}
for r in range(runs):
    order = [(case, impl) for case in bench.CASES for impl in impls]
    if r % 2:
        order.reverse()
    for (label, name, args), impl in order:
        path = bench.OUT / name
        cmd = [impls[impl], "-repeats", "5",
               *(bench.probe_args(args) if impl == "c" else args), path]
        o = subprocess.run(list(map(str, cmd)), check=True, env=env,
                           stdout=subprocess.PIPE, universal_newlines=True).stdout
        f = dict(line.split("=", 1) for line in o.split())
        res.setdefault(label, {}).setdefault(impl, []).append(
            {"ms": int(f["nanoseconds"]) / int(f["decodes"]) / 1e6,
             "checksum": f["checksum"], "dims": f["dims"], "load": os.getloadavg()[0]})
print("| case | before ms | after ms | C ms | after/before | before/C | after/C |")
print("| --- | ---: | ---: | ---: | ---: | ---: | ---: |")
for label, d in res.items():
    sums = {i: {(x["dims"], x["checksum"]) for x in d[i]} for i in d}
    assert len(set().union(*sums.values())) == 1, (label, sums)
    m = {i: statistics.median(x["ms"] for x in d[i]) for i in d}
    print(f"| {label} | {m['before']:.2f} | {m['after']:.2f} | {m['c']:.2f} | "
          f"{m['after'] / m['before']:.2f} | {m['before'] / m['c']:.2f} | {m['after'] / m['c']:.2f} |")
loads = [x["load"] for d in res.values() for i in d.values() for x in i]
print(f"load average (1 min) {min(loads):.1f}-{max(loads):.1f}; all checksums equal across C, before, after")
json.dump(res, open(out, "w"), indent=1)
