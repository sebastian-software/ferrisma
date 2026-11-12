#!/usr/bin/env python3
"""Interleaved A/B runner: fresh process per sample, alternating order.

usage: ab.py plan.json out.json
plan: {"reps": 5, "candidates": [{"name":..., "argv":[...], "parse": "ms"|"mps", "pixels": N, "per": k}]}
"ms": parse elapsed_ms=X (total over `per` iterations); "mps": djxl/cjxl geomean MP/s.
Records per-iteration ms, max RSS (bytes, /usr/bin/time -l) and 1-min load before/after.
"""
import json, os, re, statistics, subprocess, sys

plan = json.load(open(sys.argv[1]))
reps = plan.get("reps", 5)
cands = plan["candidates"]
results = {c["name"]: [] for c in cands}


def load():
    return os.getloadavg()[0]


def run(c):
    l0 = load()
    p = subprocess.run(["/usr/bin/time", "-l"] + c["argv"], capture_output=True, text=True)
    l1 = load()
    out = p.stdout + p.stderr
    if p.returncode != 0:
        raise SystemExit(f"{c['name']} failed: {out[-2000:]}")
    rss = int(re.search(r"(\d+)\s+maximum resident set size", out).group(1))
    if c["parse"] == "ms":
        ms = float(re.search(r"elapsed_ms=([0-9.]+)", out).group(1)) / c.get("per", 1)
    else:
        m = re.search(r"([0-9.]+) MP/s", out)
        ms = c["pixels"] / 1e6 / float(m.group(1)) * 1000.0
    return {"ms": ms, "rss": rss, "load": [l0, l1]}


for c in cands:  # warm-up
    run(c)
for r in range(reps):
    order = cands if r % 2 == 0 else list(reversed(cands))
    for c in order:
        results[c["name"]].append(run(c))
summary = {}
for name, xs in results.items():
    ms = [x["ms"] for x in xs]
    rss = [x["rss"] for x in xs]
    loads = [l for x in xs for l in x["load"]]
    summary[name] = {
        "ms_median": statistics.median(ms), "ms_min": min(ms), "ms_max": max(ms),
        "rss_mb_median": statistics.median(rss) / 1e6,
        "load_min": min(loads), "load_max": max(loads),
    }
    print(f"{name:40s} {summary[name]['ms_median']:9.1f} ms ({min(ms):.1f}-{max(ms):.1f})  "
          f"RSS {summary[name]['rss_mb_median']:7.1f} MB  load {min(loads):.1f}-{max(loads):.1f}")
json.dump({"plan": plan, "samples": results, "summary": summary}, open(sys.argv[2], "w"), indent=1)
