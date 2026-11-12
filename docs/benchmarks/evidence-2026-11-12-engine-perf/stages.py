"""Stage-share runner: CLI processing report + max RSS per configuration.

Usage: python3 -I stages.py <ferrisma-binary> <out.json> [reps]
Indicative only (loaded host); records 1-minute load average per sample.
"""
import json
import os
import re
import subprocess
import sys

S = os.path.dirname(os.path.abspath(__file__))
IN = os.path.join(os.path.dirname(S), "in19")
F = sys.argv[1]
OUT = sys.argv[2]
REPS = int(sys.argv[3]) if len(sys.argv) > 3 else 3
ONLY = os.environ.get("ONLY")

INPUTS = {
    "jpeg12": ("jpeg12-o1.jpg", (4032, 3024), (1600, 1200), "--use-rendered-icc"),
    "jpeg24": ("jpeg24-o1.jpg", (6000, 4000), (1600, 1067), "--assume-srgb"),
    "jpeg24icc": ("jpeg24-o1-icc.jpg", (6000, 4000), (1600, 1067), "--use-rendered-icc"),
    "webp12": ("webp12.webp", (4032, 3024), (1600, 1200), "--use-rendered-icc"),
    "webp24": ("webp24.webp", (6000, 4000), (1600, 1067), "--assume-srgb"),
    "avif12": ("avif12.avif", (4032, 3024), (1600, 1200), "--use-rendered-icc"),
    "avif24": ("avif24.avif", (6000, 4000), (1600, 1067), "--assume-srgb"),
}


def configs():
    for name, (f, full, small, prof) in INPUTS.items():
        for fmt in ("avif", "webp"):
            for resize in ("down", "none"):
                for t in ("1", "0"):
                    cid = f"{name}-{fmt}-{resize}-t{t}"
                    if ONLY and not re.search(ONLY, cid):
                        continue
                    w, h = small if resize == "down" else full
                    yield cid, [F, os.path.join(IN, f), os.path.join(S, "out", f"{cid}.{fmt}"),
                                str(w), str(h), prof, "--budget-mb=4096", "--max-output-mb=1024",
                                f"--transform-threads={t}", f"--encoder-threads={t}"]


def run(cmd):
    load = os.getloadavg()[0]
    p = subprocess.run(["/usr/bin/time", "-l", *cmd], capture_output=True, text=True)
    if p.returncode != 0:
        return {"error": p.stderr[-400:]}
    rep = dict(re.findall(r"(\w+)=(\S+)", re.search(r"^report: (.*)$", p.stdout + p.stderr, re.M).group(1)))
    rss = int(re.search(r"(\d+)\s+maximum resident set size", p.stderr).group(1))
    return {"load": load, "rss": rss, **rep}


results = {}
cfgs = list(configs())
for rep in range(REPS):
    for cid, cmd in (cfgs if rep % 2 == 0 else reversed(cfgs)):
        results.setdefault(cid, {"cmd": cmd, "samples": []})["samples"].append(run(cmd))
        print(cid, results[cid]["samples"][-1].get("total_us"), flush=True)
json.dump(results, open(OUT, "w"), indent=1)
