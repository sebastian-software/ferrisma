# Interleaved timing of jxl_decode_compare builds: 5 repetitions, 1 warm-up.
# usage: timing.py OUT.json INPUT
import json, os, re, statistics, subprocess, sys
V = os.path.dirname(os.path.abspath(__file__))
out, inp = sys.argv[1], sys.argv[2]
cases = {
  "x86-before-avx2": (f"{V}/jdc-x86-before", {"ROSETTA_ADVERTISE_AVX": "1"}),
  "x86-before-scalar": (f"{V}/jdc-x86-before", {}),
  "x86-after-avx2": (f"{V}/jdc-x86-after", {"ROSETTA_ADVERTISE_AVX": "1"}),
  "x86-after-scalar": (f"{V}/jdc-x86-after", {}),
  "arm-before-neon": (f"{V}/jdc-arm-before", {}),
  "arm-after-neon": (f"{V}/jdc-arm-after", {}),
}
res = {k: [] for k in cases}; sums = {k: set() for k in cases}; loads = []
for rep in range(6):
    for k, (b, e) in cases.items():
        loads.append(os.getloadavg()[0])
        r = subprocess.run([b, inp, "own", "full", "1"], env=dict(os.environ, **e), capture_output=True, text=True, check=True).stdout
        ms = float(re.search(r"elapsed_ms=([0-9.]+)", r).group(1)); sums[k].add(re.search(r"checksum=(\S+)", r).group(1))
        if rep: res[k].append(ms)
        print(rep, k, ms, flush=True)
summary = {k: {"median_ms": statistics.median(v), "min_ms": min(v), "max_ms": max(v), "runs": v, "checksums": sorted(sums[k])} for k, v in res.items()}
json.dump({"input": inp, "load_1min": {"min": min(loads), "max": max(loads), "median": statistics.median(loads)}, "cases": summary}, open(out, "w"), indent=1)
for k, v in summary.items(): print(k, round(v["median_ms"], 1), v["checksums"])
print("load", min(loads), max(loads))
