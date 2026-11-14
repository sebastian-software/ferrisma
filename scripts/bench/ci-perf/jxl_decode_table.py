#!/usr/bin/env python3
"""Tables for jxl_decode_three_column.sh.

Usage: python3 -I jxl_decode_table.py OUT_DIR [input ...]

Reads OUT_DIR/jxl-decode-<input>.json (run_interleaved.py) and
OUT_DIR/jxl-decode-equivalence.json and prints Markdown: the three-column
ratio table (port / original, medians of the harnesses' own timers) and the
raw medians with min-max per candidate.
"""

import json
import sys
from pathlib import Path

out = Path(sys.argv[1])
equiv = json.loads((out / "jxl-decode-equivalence.json").read_text())
names = sys.argv[2:] or list(equiv)
PAIRS = (("a", "c-scalar:t1", "port-scalar:t1", "a_scalar_port_equals_original"),
         ("b", "c:t1", "port:t1", "b_neon_port_equals_original"),
         ("c", "c:tN", "port:tN", "c_threads_port_equals_original"))


def ms(summary, cid):
    s = summary[cid]
    return s["internal_median_ms"], s["internal_min_ms"], s["internal_max_ms"], s["n"]


def fmt(v):
    return f"{v:,.0f}" if v >= 100 else f"{v:,.1f}"


print("| Input | Port ÷ original (a) scalar, 1 thread | (b) NEON, 1 thread | (c) NEON, all threads |")
print("| --- | ---: | ---: | ---: |")
for name in names:
    rec = json.loads((out / f"jxl-decode-{name}.json").read_text())
    s = rec["summary"]
    cells = []
    for _, c, p, key in PAIRS:
        ratio = ms(s, p)[0] / ms(s, c)[0]
        cells.append(f"**{ratio:.2f}×** ({'identical' if equiv[name][key] else 'DIFFERS'})")
    print(f"| {name} | " + " | ".join(cells) + " |")
print()
print("| Input | Candidate | Median ms | Min–max ms | n | Load (1 min) |")
print("| --- | --- | ---: | ---: | ---: | --- |")
for name in names:
    rec = json.loads((out / f"jxl-decode-{name}.json").read_text())
    s = rec["summary"]
    for c in rec["plan"]["candidates"]:
        cid = c["id"]
        med, lo, hi, n = ms(s, cid)
        print(f"| {name} | {cid} | {fmt(med)} | {fmt(lo)}–{fmt(hi)} | {n} | "
              f"{s[cid]['load_min']:.1f}–{s[cid]['load_max']:.1f} |")
