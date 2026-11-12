"""Port vs C AOM still-image encode (encode_still.rs vs encode_still.c), interleaved.

C side: upstream libavif 1.4.2 (vendored source) + Homebrew libaom 3.15.3 (NEON).
Port side: libavif-rs encode_still example (libaom 3.15.2 translation, NEON RTCD).
Indicative only; loaded host. Usage: python3 -I aomcmp.py <port-bin> <c-bin> <out.json> [reps]
"""
import json
import os
import re
import subprocess
import sys

S = os.path.dirname(os.path.abspath(__file__))
port, cbin, out = sys.argv[1:4]
reps = int(sys.argv[4]) if len(sys.argv) > 4 else 3
cases = []
for img in os.environ.get("IMGS", "dog,mountain,elcito").split(","):
    for q in os.environ.get("QS", "56,82").split(","):
        for chroma in os.environ.get("CHROMA", "444").split(","):
            cases.append((img, int(q), chroma))
res = {}
for r in range(reps):
    order = [(c, impl) for c in cases for impl in ("port", "c")]
    if r % 2:
        order.reverse()
    for (img, q, chroma), impl in order:
        b = port if impl == "port" else cbin
        o = os.path.join(S, "out", f"aom-{impl}-{img}-q{q}-{chroma}.avif")
        cmd = [b, os.path.join(S, "bmp", f"{img}.bmp"), o, f"--quality={q}", "--speed=6", "--threads=1",
               "--depth=10", "--repeat=1"] + (["--yuv444"] if chroma == "444" else [])
        load = os.getloadavg()[0]
        p = subprocess.run(["/usr/bin/time", "-l", *cmd], capture_output=True, text=True)
        us = re.findall(r"encode_us=(\d+)", p.stdout + p.stderr)
        rss = re.search(r"(\d+)\s+maximum resident", p.stderr)
        key = f"{img}-q{q}-{chroma}-{impl}"
        res.setdefault(key, []).append({"us": int(us[-1]) if us else None, "rss": int(rss.group(1)) if rss else None,
                                        "bytes": os.path.getsize(o) if os.path.exists(o) else None, "load": load,
                                        "rc": p.returncode, "err": p.stderr[-200:] if p.returncode else ""})
        print(key, res[key][-1]["us"], res[key][-1]["bytes"], flush=True)
json.dump(res, open(out, "w"), indent=1)
