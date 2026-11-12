#!/usr/bin/env python3
"""Compact cross-platform tables for docs/benchmarks/v1.0-ci-performance-*.md.

Usage: python3 -I report_tables.py ARTIFACTS_DIR [ARTIFACTS_DIR...] > tables.md

Engine cells: "wall (decode / transform / encode)" medians in ms from the CLI
report and process wall. Codec cells: port / C ratio of the internal timer
medians, with both medians.
"""

import json
import re
import sys
from pathlib import Path

PLATFORMS = [("ubuntu-24.04", "GH x86-64"), ("ubuntu-24.04-arm", "GH ARM64"),
             ("macos-15", "GH macOS ARM64"), ("blacksmith-4vcpu-ubuntu-2404", "BS x86-64"),
             ("blacksmith-4vcpu-ubuntu-2404-arm", "BS ARM64")]


def platform_of(name):
    for key, label in sorted(PLATFORMS, key=lambda p: -len(p[0])):
        if name.endswith(key):
            return label
    return None


def fmt(v):
    return f"{v:,.0f}"


def load(roots):
    engine, codec = {}, {}
    for root in roots:
        for d in Path(root).iterdir():
            plat = platform_of(d.name)
            if not plat:
                continue
            for f in d.glob("*.json"):
                if re.search(r"-(build|inputs|checks|skipped)(-\d+)?\.json$", f.name):
                    continue
                r = json.loads(f.read_text())
                if "plan" not in r:
                    continue
                target = engine if d.name.startswith("engine") else codec
                target.setdefault(plat, []).append(r)
    return engine, codec


def engine_tables(engine):
    out = []
    plats = [p for _, p in PLATFORMS if p in engine]
    for part in ("decode", "encode"):
        for size in ("12", "24"):
            for resize in ("2048w", "full"):
                for threads in ("t1", "tN"):
                    rows = {}
                    for plat in plats:
                        for r in engine[plat]:
                            if r["plan"]["group"] != f"engine/{part}/photo{size}":
                                continue
                            for c in r["plan"]["candidates"]:
                                cid = c["id"]
                                if not cid.endswith(f":{resize}:{threads}"):
                                    continue
                                key = cid.rsplit(":", 2)[0]
                                s = r["summary"][cid]
                                rep = s.get("report_median")
                                if not rep:
                                    cell = "failed"
                                else:
                                    cell = (f"{fmt(s['wall_median_s'] * 1e3)} ({fmt(rep['decode_us'] / 1e3)} / "
                                            f"{fmt(rep['transform_us'] / 1e3)} / {fmt(rep['encode_us'] / 1e3)})")
                                rows.setdefault(key, {})[plat] = cell
                    if not rows:
                        continue
                    label = "resize to 2048 px wide" if resize == "2048w" else "no resize"
                    tlabel = "one thread" if threads == "t1" else "all cores"
                    out.append(f"\n#### {part} part, {size} MP, {label}, {tlabel}\n")
                    out.append("| Case | " + " | ".join(plats) + " |")
                    out.append("| --- | " + " | ".join("---:" for _ in plats) + " |")
                    for key, cells in rows.items():
                        out.append(f"| {key} | " + " | ".join(cells.get(p, "–") for p in plats) + " |")
    return out


def codec_tables(codec):
    out = []
    plats = [p for _, p in PLATFORMS if p in codec]
    rows = {}
    for plat in plats:
        for r in codec[plat]:
            group = r["plan"]["group"]
            summary = r["summary"]
            refs = {}
            for cid, s in summary.items():
                impl, _, th = cid.partition(":")
                if impl in ("c", "c-aom"):
                    refs[th] = s
            for cid, s in summary.items():
                impl, _, th = cid.partition(":")
                if impl.startswith("c"):
                    continue
                ref = refs.get(th) or refs.get("t1")
                if not ref or not s.get("internal_median_ms") or not ref.get("internal_median_ms"):
                    cell = "–"
                else:
                    ratio = s["internal_median_ms"] / ref["internal_median_ms"]
                    cell = (f"**{ratio:.2f}×** ({fmt(s['internal_median_ms'])} / "
                            f"{fmt(ref['internal_median_ms'])})")
                key = re.sub(r"/t[1N]$", "", group) + f" {impl}:{th}"
                rows.setdefault(key, {})[plat] = cell
    out.append("| Operation, input, candidate | " + " | ".join(plats) + " |")
    out.append("| --- | " + " | ".join("---:" for _ in plats) + " |")
    for key in sorted(rows):
        out.append(f"| {key} | " + " | ".join(rows[key].get(p, "–") for p in plats) + " |")
    return out


def main():
    engine, codec = load(sys.argv[1:])
    print("### Engine stages: wall ms (decode / transform / encode ms), medians\n")
    print("\n".join(engine_tables(engine)))
    print("\n### Port / original, internal timer medians (port ms / C ms)\n")
    print("\n".join(codec_tables(codec)))


if __name__ == "__main__":
    main()
