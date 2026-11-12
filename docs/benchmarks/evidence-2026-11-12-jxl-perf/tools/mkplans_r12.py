#!/usr/bin/env python3
"""Write the R1/R2 A/B plans. usage: mkplans_r12.py <scratch> <final-probe-name>"""
import json
import os
import sys

S = sys.argv[1]
final = sys.argv[2]
P = S + "/jxlperf"
B = P + "/bin/"
out = S + "/r12/plans"
os.makedirs(out, exist_ok=True)


def dec(name, f, k, probes):
    c = [{"name": p, "argv": [B + p, "decode", f, str(k)], "parse": "ms", "per": k} for p in probes]
    json.dump({"reps": 7, "candidates": c}, open(f"{out}/dec-{name}.json", "w"), indent=1)


dec("flower-d1e7", P + "/flower-d1e7.jxl", 3, ["probe-r12base", "probe-r12-xg", "probe-r12-epf", final])
dec("mountain-d1e7", P + "/mountain-d1e7.jxl", 1, ["probe-r12base", final])
dec("mountain-lossless", S + "/corpus/mountain-lossless.jxl", 1, ["probe-r12base", final])
dec("flower-lossless", S + "/corpus/flower-lossless.jxl", 3, ["probe-r12base", final])
dec("flower-recon", S + "/corpus/flower-jpegrecon.jxl", 3, ["probe-r12base", final])
c = [{"name": p, "argv": [B + p, "lossy", P + "/flower.ppm", "7", "1.0", "1", f"{S}/r12/enc-{p}.jxl"],
      "parse": "ms"} for p in ["probe-r12base", final]]
json.dump({"reps": 7, "candidates": c}, open(f"{out}/enc-flower-e7d1.json", "w"), indent=1)
