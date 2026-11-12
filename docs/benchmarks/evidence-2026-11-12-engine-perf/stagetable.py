"""Merge the stage-share sections of sum/<profile>.md into one table."""
import os
import re
import sys

S = os.path.dirname(os.path.abspath(__file__))
names = sys.argv[1:]
cols = ["jpeg-decode", "webp-decode", "av1-decode", "yuv2rgb", "icc", "orientation", "linearize", "resize",
        "quantize", "engine-pipeline", "rgb2yuv", "aom-encode", "rav1e-encode", "webp-encode", "sync/wait", "other"]
rows = []
for n in names:
    txt = open(os.path.join(S, "sum", n + ".md")).read()
    sec = txt.split("## Stage shares")[1]
    vals = dict(re.findall(r"\| ([\w/-]+) \| ([\d.]+) \|", sec))
    rows.append((n, vals))
used = [c for c in cols if any(float(v.get(c, 0)) >= 0.5 for _, v in rows)]
print("| profile | " + " | ".join(used) + " |")
print("| --- | " + " | ".join("---:" for _ in used) + " |")
for n, v in rows:
    print(f"| {n} | " + " | ".join(v.get(c, "–") for c in used) + " |")
