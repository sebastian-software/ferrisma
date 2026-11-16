"""Measured against reserved memory for the AVIF and WebP writers
(2026-11-16). python3 -I other_table.py <run_other.sh results>

The reservation is what the engine charges the budget for the request
(printed by the probe: AVIF `required_frame_bytes`, WebP the ARGB frame plus
the YUV420(A) planes of lossy output). The measurement is the larger RSS of
the runs. Prints a Markdown table.
"""
import re
import sys

best, res = {}, {}
for line in open(sys.argv[1]):
    p = line.split()
    if len(p) < 6 or not p[3].isdigit():
        continue
    k = (p[1], p[0])
    best[k] = max(best.get(k, 0), int(p[3]))
    res[k] = int(p[5])
print("| Writer, setting | Input | Pixels | Measured MB | Measured B/px | Reserved B/px | Measured / reserved |")
print("| --- | --- | ---: | ---: | ---: | ---: | ---: |")
for (label, name), rss in sorted(best.items(), key=lambda kv: (kv[0][0], kv[0][1].split("-")[0], eval(re.search(r"(\d+)x(\d+)", kv[0][1]).group(0).replace("x", "*")), kv[0][1])):
    w, h = map(int, re.search(r"(\d+)x(\d+)", name).groups())
    px = w * h
    r = res[(label, name)]
    print(f"| {label} | {name} | {px:,} | {rss / 1e6:,.1f} | {rss / px:.1f} | {r / px:.1f} | {rss / r:.1f}x |")
