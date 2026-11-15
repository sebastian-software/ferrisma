#!/usr/bin/env python3
"""Markdown tables for the record from measure.py's JSON.

Usage: tables.py results.json
"""
import json
import statistics
import sys

d = json.load(open(sys.argv[1]))
rows = d["results"]


def f(x, n=3):
    return f"{x:.{n}f}"


def stats(vals, n=3):
    return f"{f(statistics.median(vals), n)} ({f(min(vals), n)}-{f(max(vals), n)})"


print("## Per image\n")
for dist in (1.0, 2.0):
    print(f"### Distance {dist}\n")
    print("| Image | Pixels | Groups | Default bytes | `--patches=0` bytes | Patches used by default | Off / default | Streaming off, patches off bytes | `--patches=1` bytes | Off / forced | SSIMULACRA2 default | off | forced |")
    print("| --- | ---: | ---: | ---: | ---: | :---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    for r in rows:
        if r["distance"] != dist:
            continue
        print(f"| {r['name']} | {r['width']}x{r['height']} | {r['groups']} | {r['default']['bytes']:,} | {r['patches0']['bytes']:,} | "
              f"{'yes' if r['patches_used'] else 'no'} | {f(r['size_ratio'])} | {r['nostream0']['bytes']:,} | {r['patches1']['bytes']:,} | "
              f"{f(r['size_ratio_vs_forced'])} | {f(r['default']['ssimulacra2'], 2)} | {f(r['patches0']['ssimulacra2'], 2)} | {f(r['patches1']['ssimulacra2'], 2)} |")
    print()

print("## Per kind\n")
classes = (("at most 8 groups", lambda r: r["groups"] <= 8), ("more than 8 groups", lambda r: r["groups"] > 8))
for dist in (1.0, 2.0):
    print(f"### Distance {dist}\n")
    print("| Kind | Size class | n | Patches used by default | Off / default, median (min-max) | SSIMULACRA2 off minus default, median | Off / forced, median (min-max) | SSIMULACRA2 off minus forced, median | Streaming off / forced, median (min-max) |")
    print("| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    for kind in dict.fromkeys(r["kind"] for r in rows):
        for cname, cf in classes:
            sel = [r for r in rows if r["kind"] == kind and r["distance"] == dist and cf(r)]
            if not sel:
                continue
            print(f"| {kind} | {cname} | {len(sel)} | {sum(r['patches_used'] for r in sel)} | {stats([r['size_ratio'] for r in sel], 3)} | "
                  f"{statistics.median([r['ssimulacra2_delta'] for r in sel]):+.2f} | {stats([r['size_ratio_vs_forced'] for r in sel], 2)} | "
                  f"{statistics.median([r['ssimulacra2_delta_vs_forced'] for r in sel]):+.2f} | {stats([r['size_ratio_streaming_off'] for r in sel], 2)} |")
    print()
