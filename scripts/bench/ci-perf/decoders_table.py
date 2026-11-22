#!/usr/bin/env python3
"""Markdown tables for scripts/bench/ci-perf/decoders_avif.sh outputs.

Usage: python3 -I decoders_table.py LABEL=OUTDIR [LABEL=OUTDIR ...]

Per OUTDIR: a ratio table (port-dav1d / port-aom and each port against
stock C, one thread and all threads) and a full table (median, min-max,
spread, peak RSS) per input. Ratios are of the internal-timer medians.
"""

import json
import sys
from pathlib import Path

SIZES = ["web03", "web14", "12", "24"]
SIZE_NAME = {"web03": "0.34 MP", "web14": "1.4 MP", "12": "12 MP", "24": "24 MP"}
CLASSES = ["8-420", "8-444", "10-420", "10-444"]
CLASS_NAME = {"8-420": "8-bit 4:2:0", "8-444": "8-bit 4:4:4", "10-420": "10-bit 4:2:0",
              "10-444": "10-bit 4:4:4"}
ORDER = ["port-aom-reader:t1", "port-aom:t1", "port-dav1d:t1", "c-aom:t1", "c-dav1d-v1:t1",
         "c-dav1d:t1", "port-aom:tN", "port-dav1d:tN", "c-aom:tN", "c-dav1d-v1:tN", "c-dav1d:tN"]


def fmt(ms):
    return f"{ms:,.0f}" if ms >= 100 else f"{ms:.1f}" if ms >= 10 else f"{ms:.2f}"


def load(outdir):
    records = {}
    for size in SIZES:
        for cls in CLASSES:
            path = Path(outdir) / f"decoders-{size}-{cls}.json"
            if path.exists():
                records[(size, cls)] = json.loads(path.read_text())
    return records


def med(record, cid):
    s = record["summary"].get(cid) or {}
    return s.get("internal_median_ms")


def ratio(record, a, b):
    x, y = med(record, a), med(record, b)
    return f"{x / y:.2f}" if x and y else "–"


def main():
    for arg in sys.argv[1:]:
        label, _, outdir = arg.partition("=")
        records = load(outdir)
        checks_path = Path(outdir) / "decoders-checks.json"
        checks = json.loads(checks_path.read_text()) if checks_path.exists() else {}
        any_record = next(iter(records.values()))
        loads = [s["load_before"] for r in records.values() for s in r["samples"]]
        print(f"### {label}\n")
        print(f"- Platform: {any_record['platform']}, {any_record['cpus']} CPUs (tN = {any_record['cpus']} threads)")
        print(f"- 1-minute load average before samples: {min(loads):.1f}–{max(loads):.1f}")
        same = sum(1 for c in checks.values() if c["all_identical"])
        print(f"- Decoded RGB identical across all candidates: {same} of {len(checks)} inputs\n")
        print("Ratios of internal-timer medians (above 1: the first is slower).\n")
        print("| Input | dav1d port / aom port, t1 | same, tN | aom port / C aom, t1 | same, tN "
              "| dav1d port / C dav1d v1, t1 | same, tN | aom port / C dav1d v1, t1 | same, tN |")
        print("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
        for size in SIZES:
            for cls in CLASSES:
                r = records.get((size, cls))
                if not r:
                    continue
                cells = [ratio(r, "port-dav1d:t1", "port-aom:t1"), ratio(r, "port-dav1d:tN", "port-aom:tN"),
                         ratio(r, "port-aom:t1", "c-aom:t1"), ratio(r, "port-aom:tN", "c-aom:tN"),
                         ratio(r, "port-dav1d:t1", "c-dav1d-v1:t1"), ratio(r, "port-dav1d:tN", "c-dav1d-v1:tN"),
                         ratio(r, "port-aom:t1", "c-dav1d-v1:t1"), ratio(r, "port-aom:tN", "c-dav1d-v1:tN")]
                print(f"| {SIZE_NAME[size]} {CLASS_NAME[cls]} | " + " | ".join(cells) + " |")
        print("\nInternal-timer median ms (min–max; spread = (max − min) / median; peak RSS median MB), "
              f"{any_record['plan']['reps']} repetitions after {any_record['plan']['warmups']} warm-up.\n")
        print("| Input | " + " | ".join(ORDER) + " |")
        print("| --- |" + " ---: |" * len(ORDER))
        for size in SIZES:
            for cls in CLASSES:
                r = records.get((size, cls))
                if not r:
                    continue
                cells = []
                for cid in ORDER:
                    s = r["summary"].get(cid)
                    if not s or not s.get("internal_median_ms"):
                        cells.append("–")
                        continue
                    m, lo, hi = s["internal_median_ms"], s["internal_min_ms"], s["internal_max_ms"]
                    rss = s.get("rss_median_bytes", 0) / 1e6
                    cells.append(f"{fmt(m)} ({fmt(lo)}–{fmt(hi)}; {100 * (hi - lo) / m:.0f} %; {rss:.0f} MB)")
                print(f"| {SIZE_NAME[size]} {CLASS_NAME[cls]} | " + " | ".join(cells) + " |")
        print()


if __name__ == "__main__":
    main()
