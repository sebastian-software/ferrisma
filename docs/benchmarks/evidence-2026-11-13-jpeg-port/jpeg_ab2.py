"""Interleaved runs of libjpeg-turbo-rs tools/bench.py cases: the C oracle's
bench probe against any number of port binaries.

Usage: python3 -I jpeg_ab2.py <codec dir> <oracle variant dir> <out.json> <runs>
           [--env NAME=VALUE ...] [--cases SUBSTR,...] name=binary [name=binary ...]

Indicative; loaded host. Order alternates per run (forward, reversed). Every
implementation must produce the same dims and checksum as C for every case.
The first named port binary is the "before" column of the ratios.
"""
import json
import os
import statistics
import subprocess
import sys

args = sys.argv[1:]
codec, oracle, out, runs = args[:4]
runs = int(runs)
rest = args[4:]
extra_env, cases_filter, impls = {}, None, {}
i = 0
while i < len(rest):
    if rest[i] == "--env":
        name, value = rest[i + 1].split("=", 1)
        extra_env[name] = value
        i += 2
    elif rest[i] == "--cases":
        cases_filter = rest[i + 1].split(",")
        i += 2
    else:
        name, binary = rest[i].split("=", 1)
        impls[name] = binary
        i += 1
sys.path.insert(0, os.path.join(codec, "tools"))
import bench  # noqa: E402

probe = os.path.join(oracle, "probes/wp9/bench")
impls = {"c": probe, **impls}
env = {k: v for k, v in os.environ.items() if not k.startswith("JSIMD_")}
env.update(extra_env)
cases = [c for c in bench.CASES
         if cases_filter is None or any(s in c[0] for s in cases_filter)]
res = {}
for r in range(runs):
    order = [(case, impl) for case in cases for impl in impls]
    if r % 2:
        order.reverse()
    for (label, name, cargs), impl in order:
        path = bench.OUT / name
        cmd = [impls[impl], "-repeats", "5",
               *(bench.probe_args(cargs) if impl == "c" else cargs), path]
        o = subprocess.run(list(map(str, cmd)), check=True, env=env,
                           stdout=subprocess.PIPE, universal_newlines=True).stdout
        f = dict(line.split("=", 1) for line in o.split())
        res.setdefault(label, {}).setdefault(impl, []).append(
            {"ms": int(f["nanoseconds"]) / int(f["decodes"]) / 1e6,
             "checksum": f["checksum"], "dims": f["dims"], "load": os.getloadavg()[0]})
names = [n for n in impls if n != "c"]
head = "| case | C ms | " + " | ".join(f"{n} ms" for n in names) + " | " + \
    " | ".join(f"{n} / C" for n in names)
if len(names) > 1:
    head += " | " + " | ".join(f"{n} / {names[0]}" for n in names[1:])
head += " |"
print(head)
print("| --- " + "| ---: " * (head.count("|") - 2) + "|")
for label, d in res.items():
    sums = {i: {(x["dims"], x["checksum"]) for x in d[i]} for i in d}
    assert len(set().union(*sums.values())) == 1, (label, sums)
    m = {i: statistics.median(x["ms"] for x in d[i]) for i in d}
    row = f"| {label} | {m['c']:.2f} | " + " | ".join(f"{m[n]:.2f}" for n in names) + " | " + \
        " | ".join(f"{m[n] / m['c']:.2f}" for n in names)
    if len(names) > 1:
        row += " | " + " | ".join(f"{m[n] / m[names[0]]:.2f}" for n in names[1:])
    print(row + " |")
loads = [x["load"] for d in res.values() for i in d.values() for x in i]
print(f"load average (1 min) {min(loads):.1f}-{max(loads):.1f}; env {extra_env}; "
      f"{runs} runs x 5 decodes; all checksums equal across C and the port binaries")
json.dump({"oracle": oracle, "env": extra_env, "impls": impls, "runs": res},
          open(out, "w"), indent=1)
