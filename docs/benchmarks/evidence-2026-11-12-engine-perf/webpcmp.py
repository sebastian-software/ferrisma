"""Port (through the engine CLI stage report) vs Homebrew libwebp 1.6.0 cwebp/dwebp, interleaved.

Decode: engine decode_us (webp24/webp12 -> 1600 px WebP, one thread; adapter decode excluding the
transform callback) vs `dwebp -v` "Time to decode picture" (RGB output, fancy upsampling).
Encode: engine encode_us for a 1600x1067 lossless-WebP input re-encoded lossy q75 speed 6 (method 3),
no resize (exact pixel roundtrip verified), vs `cwebp -v -preset default -q 75 -m 3` "Time to encode picture"
from the same pixels as PNG. Indicative; loaded host.
"""
import json
import os
import re
import subprocess
import statistics as st

S = os.path.dirname(os.path.abspath(__file__))
I = os.path.join(os.path.dirname(S), "in19")
F = os.path.join(S, "ferrisma-base")
cases = {
    "decode-webp24|port": [F, f"{I}/webp24.webp", f"{S}/out/w1.webp", "1600", "1067", "--assume-srgb", "--transform-threads=1"],
    "decode-webp24|c": ["dwebp", "-v", f"{I}/webp24.webp", "-ppm", "-o", f"{S}/out/w.ppm"],
    "decode-webp12|port": [F, f"{I}/webp12.webp", f"{S}/out/w2.webp", "1600", "1200", "--use-rendered-icc", "--transform-threads=1"],
    "decode-webp12|c": ["dwebp", "-v", f"{I}/webp12.webp", "-ppm", "-o", f"{S}/out/w.ppm"],
    "encode-1600|port": [F, f"{S}/bmp/mountain1600-ll.webp", f"{S}/out/w3.webp", "1600", "1067", "--assume-srgb", "--transform-threads=1"],
    "encode-1600|c": ["cwebp", "-v", "-preset", "default", "-q", "75", "-m", "3", f"{S}/bmp/mountain1600.png", "-o", f"{S}/out/w4.webp"],
}
res = {}
for r in range(7):
    order = list(cases) if r % 2 == 0 else list(reversed(cases))
    for k in order:
        load = os.getloadavg()[0]
        p = subprocess.run(cases[k], capture_output=True, text=True)
        txt = p.stdout + p.stderr
        if k.endswith("|port"):
            field = "decode_us" if k.startswith("decode") else "encode_us"
            ms = int(re.search(field + r"=(\d+)", txt).group(1)) / 1000
        else:
            pat = r"Time to decode picture: ([\d.]+)s" if k.startswith("decode") else r"Time to encode picture: ([\d.]+)s"
            ms = float(re.search(pat, txt).group(1)) * 1000
        res.setdefault(k, []).append({"ms": ms, "load": load})
json.dump(res, open(os.path.join(S, "webpcmp.json"), "w"), indent=1)
print("| case | port ms | C ms | port/C | load |\n| --- | ---: | ---: | ---: | ---: |")
for k in sorted({k.split("|")[0] for k in res}):
    p = [s["ms"] for s in res[k + "|port"]]
    c = [s["ms"] for s in res[k + "|c"]]
    loads = [s["load"] for s in res[k + "|port"] + res[k + "|c"]]
    print(f"| {k} | {st.median(p):.0f} ({min(p):.0f}–{max(p):.0f}) | {st.median(c):.0f} ({min(c):.0f}–{max(c):.0f}) | {st.median(p)/st.median(c):.2f} | {min(loads):.1f}–{max(loads):.1f} |")
