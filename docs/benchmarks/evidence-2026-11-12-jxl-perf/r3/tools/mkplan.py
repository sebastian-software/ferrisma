#!/usr/bin/env python3
"""Write an interleaved A/B plan for ab.py.

usage: mkplan.py <case> <reps> <out.json> <binname>...
case: recon | d1e7 | mountain | enc. Binaries live in <scratch>/r3/bin.
"""
import json
import os
import sys

S = os.environ["S"]  # scratch directory with the inputs and r3/bin
case, reps, out, bins = sys.argv[1], int(sys.argv[2]), sys.argv[3], sys.argv[4:]
files = {
    "recon": (S + "/corpus/flower-jpegrecon.jxl", 3),
    "d1e7": (S + "/jxlperf/flower-d1e7.jxl", 3),
    "mountain": (S + "/jxlperf/mountain-d1e7.jxl", 1),
}
c = []
for b in bins:
    exe = S + "/r3/bin/" + b
    if case == "enc":
        c.append({"name": b, "argv": [exe, "lossy", S + "/jxlperf/flower.ppm", "7", "1.0", "1"], "parse": "ms"})
    else:
        f, k = files[case]
        c.append({"name": b, "argv": [exe, "decode", f, str(k)], "parse": "ms", "per": k})
json.dump({"reps": reps, "candidates": c}, open(out, "w"), indent=1)
