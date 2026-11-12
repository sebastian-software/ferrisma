"""Port vs C JPEG decode (examples/bench.rs vs tools/oracle/probes/wp9/bench.c), interleaved.

C side: Homebrew libjpeg-turbo 3.2.1 (same tag as the port's pin, NEON build).
Indicative; loaded host. Usage: python3 -I jpegcmp.py <port> <c> <out.json> [reps]
"""
import json
import os
import re
import subprocess
import sys

S = os.path.dirname(os.path.abspath(__file__))
port, cbin, out = sys.argv[1:4]
extra = os.environ.get("EXTRA")
impls = ("port", "c") + (("inline",) if extra else ())
modes_sel = os.environ.get("MODES")
reps = int(sys.argv[4]) if len(sys.argv) > 4 else 5
R = os.path.dirname(S)
inputs = {
    "mountain-24MP-420": f"{R}/in19/jpeg24-o1.jpg",
    "dog-12MP-420": f"{R}/enc-default/corpus/dog.jpg",
    "elcito-42MP-422": f"{R}/enc-elcito/corpus/elcito.jpg",
    "beach-60MP-444": f"{R}/enc-beach/corpus/beach.jpg",
}
modes = {"full": [], "scale1/2": ["-scale", "1", "2"], "scale1/8": ["-scale", "1", "8"]}
res = {}
for r in range(reps):
    order = [(i, m, impl) for i in inputs for m in modes for impl in impls if not modes_sel or m in modes_sel.split(",")]
    if r % 2:
        order.reverse()
    for i, m, impl in order:
        b = {"port": port, "c": cbin, "inline": extra}[impl]
        load = os.getloadavg()[0]
        p = subprocess.run([b, "-repeats", "3", *modes[m], inputs[i]], capture_output=True, text=True)
        ns = int(re.search(r"nanoseconds=(\d+)", p.stdout).group(1))
        ck = re.search(r"checksum=(\w+)", p.stdout).group(1)
        res.setdefault(f"{i}|{m}|{impl}", []).append({"ms_per_decode": ns / 3e6, "checksum": ck, "load": load})
        print(i, m, impl, round(ns / 3e6, 1), flush=True)
json.dump(res, open(out, "w"), indent=1)
