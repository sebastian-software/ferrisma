#!/usr/bin/env python3
"""ranges.py LABEL=DIR...: summary ranges per size group from bench_threads.sh records.

Per group (web = 0.34 + 1.4 MP, 12 MP, 24 MP): min–max speedup of new-dav1d:tT over
new-dav1d:t1 per T, min–max RSS ratio new-dav1d:tmax / t1, min–max speedup of
new-dav1d:tmax over main-dav1d:tmax, and the load range.
"""
import json, sys
from pathlib import Path

for arg in sys.argv[1:]:
    label, d = arg.split("=", 1)
    groups = {}
    loads = []
    for f in Path(d).glob("threads-*.json"):
        r = json.loads(f.read_text())
        s = r["summary"]
        base = f.stem.split("-")[1]
        g = {"photoweb03": "web", "photoweb14": "web", "photo12": "12 MP", "photo24": "24 MP"}[base]
        ts = sorted(int(c["meta"]["threads"]) for c in r["plan"]["candidates"] if c["id"].startswith("new-dav1d"))
        t1 = s["new-dav1d:t1"]["internal_median_ms"]
        e = groups.setdefault(g, {})
        for t in ts[1:]:
            e.setdefault(f"t{t}", []).append(t1 / s[f"new-dav1d:t{t}"]["internal_median_ms"])
        tm = ts[-1]
        e.setdefault("rss", []).append(s[f"new-dav1d:t{tm}"]["rss_median_bytes"] / s["new-dav1d:t1"]["rss_median_bytes"])
        e.setdefault("vs_main", []).append(s[f"main-dav1d:t{tm}"]["internal_median_ms"] / s[f"new-dav1d:t{tm}"]["internal_median_ms"])
        for c in s.values():
            loads += [c["load_min"], c["load_max"]]
    print(f"{label}: load {min(loads):.1f}-{max(loads):.1f}")
    for g in ("web", "12 MP", "24 MP"):
        e = groups[g]
        print("  " + g + ": " + "; ".join(f"{k} {min(v):.2f}-{max(v):.2f}" for k, v in e.items()))
