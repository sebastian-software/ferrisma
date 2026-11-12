#!/usr/bin/env python3
"""Write decode comparison plans. usage: mkplans.py <scratch> <probe-binary-name> [extra-probe-name]"""
import json, sys

S = sys.argv[1]
P = S + "/jxlperf"
probe = sys.argv[2]
extra = sys.argv[3] if len(sys.argv) > 3 else None
inputs = [
    ("flower-recon", S + "/corpus/flower-jpegrecon.jxl", 2268 * 1512, 3),
    ("flower-d1e7", P + "/flower-d1e7.jxl", 2268 * 1512, 3),
    ("flower-lossless", S + "/corpus/flower-lossless.jxl", 2268 * 1512, 3),
    ("mountain-d1e7", P + "/mountain-d1e7.jxl", 24e6, 1),
    ("mountain-lossless", S + "/corpus/mountain-lossless.jxl", 24e6, 1),
]
for name, f, px, k in inputs:
    djxl = lambda b, t: {
        "name": f"cxx-{b}" + ("" if t == 0 else f"-{t}thr"),
        "argv": [S + f"/cxx-{b}/tools/djxl", f, "--output_format", "pfm", "--disable_output", "-j",
                 f"--num_threads={t}", f"--num_reps={k}"],
        "parse": "mps", "pixels": px}
    c = [{"name": "port", "argv": [P + "/bin/" + probe, "decode", f, str(k)], "parse": "ms", "per": k}]
    if extra:
        c.append({"name": "port-" + extra, "argv": [P + "/bin/" + extra, "decode", f, str(k)], "parse": "ms", "per": k})
    else:
        c += [{"name": "oxide", "argv": [P + "/bin/compare-base", f, "oxide", "full", str(k)], "parse": "ms", "per": k},
              djxl("scalar", 0), djxl("simd", 0), djxl("simd", 8)]
    json.dump({"reps": 5, "candidates": c}, open(f"{P}/plan-dec-{name}{'-ab' if extra else ''}.json", "w"), indent=1)
