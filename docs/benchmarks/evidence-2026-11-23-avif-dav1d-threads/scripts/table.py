#!/usr/bin/env python3
"""table.py PASS_DIR: Markdown tables from bench_threads.sh output.

Per input: median internal ms (min-max) and median peak RSS MB per candidate,
speedup of new-dav1d:tT over new-dav1d:t1, and RGB8 identity of every
candidate's output against new-dav1d:t1 (all samples). Then plane identity
(planes.txt or qemu_planes.txt) per input against dav1d t1 and aom t1.
"""
import json, sys, re
from pathlib import Path

d = Path(sys.argv[1])
order = ["photoweb03", "photoweb14", "photo12", "photo24"]
size = {"photoweb03": "0.34 MP", "photoweb14": "1.4 MP", "photo12": "12 MP", "photo24": "24 MP"}
files = sorted(d.glob("threads-*.json"), key=lambda p: (
    order.index(p.stem.split("-")[1]), int(p.stem.split("-")[2]), p.stem.split("-")[3]))
ids = None
rows, ident_bad, loads = [], [], []
for f in files:
    r = json.loads(f.read_text())
    s = r["summary"]
    if ids is None:
        ids = [c["id"] for c in r["plan"]["candidates"]]
    _, base, depth, chroma = f.stem.split("-")
    label = f"{size[base]} {depth}-bit 4:{chroma[1]}:{chroma[2]}"
    ref = s["new-dav1d:t1"].get("output_sha256")
    t1 = s["new-dav1d:t1"]["internal_median_ms"]
    cells = []
    for i in ids:
        c = s[i]
        if c.get("failed"):
            ident_bad.append(f"{label} {i} FAILED")
        h = c.get("output_sha256")
        if h != ref or (h and len(h) != 1):
            ident_bad.append(f"{label} {i}: {h} vs {ref}")
        loads += [c["load_min"], c["load_max"]]
        cells.append(f"{c['internal_median_ms']:.1f} ({c['internal_min_ms']:.1f}–{c['internal_max_ms']:.1f}; {c['rss_median_bytes']/1e6:.0f} MB)")
    sp = [f"{t1 / s[i]['internal_median_ms']:.2f}" for i in ids if i.startswith("new-dav1d:") and i != "new-dav1d:t1"]
    rows.append((label, cells, sp))
newd = [i for i in ids if i.startswith("new-dav1d:") and i != "new-dav1d:t1"]
print(f"Load (1-min) during samples: {min(loads):.1f}–{max(loads):.1f}\n")
print("Speedup of the dav1d decode, this branch, against its own one-thread run:\n")
print("| Input | " + " | ".join(i.split(":")[1] for i in newd) + " |")
print("| --- |" + " ---: |" * len(newd))
for label, _, sp in rows:
    print(f"| {label} | " + " | ".join(sp) + " |")
print("\nInternal-timer median ms (min–max; peak RSS median):\n")
print("| Input | " + " | ".join(ids) + " |")
print("| --- |" + " ---: |" * len(ids))
for label, cells, _ in rows:
    print(f"| {label} | " + " | ".join(cells) + " |")
print(f"\nRGB8 identity against new-dav1d:t1, every sample of every candidate: "
      f"{'all identical' if not ident_bad else 'DIFFERENCES'} ({len(rows)} inputs × {len(ids)} candidates)")
for b in ident_bad:
    print("-", b)
for pf in ("planes.txt", "qemu_planes.txt"):
    p = d / pf if (d / pf).exists() else d.parent / pf
    if not p.exists():
        continue
    by = {}
    for line in p.read_text().split("\n"):
        if not line.strip():
            continue
        parts = line.split()
        kv = dict(x.split("=", 1) for x in parts[3:])
        by.setdefault(parts[0], {})[(parts[1], parts[2])] = kv
    bad, n = [], 0
    for name, m in by.items():
        ref = m[("dav1d", "t1")]
        for k, v in m.items():
            n += 1
            if k[0] == "dav1d" and (v["planes"], v["rgb"]) != (ref["planes"], ref["rgb"]):
                bad.append(f"{name} {k}")
            if k[0] == "aom" and v["planes"] != ref["planes"]:
                bad.append(f"{name} aom planes differ")
            if v.get("rc", "0") != "0":
                bad.append(f"{name} {k} rc={v['rc']}")
    print(f"\n{pf}: {len(by)} inputs, {n} decodes; YUV planes and full-depth RGB of dav1d at every thread count "
          f"vs dav1d t1 (and aom t1 planes): {'all identical' if not bad else 'DIFFERENCES'}")
    for b in bad:
        print("-", b)
