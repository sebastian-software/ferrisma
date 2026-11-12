#!/usr/bin/env python3
"""Interleaved A/B process timing for the 2026-11-12 v1.0 baseline.

Usage: python3 -I run_interleaved.py PLAN.json OUT.json

PLAN.json: {"reps": 7, "warmups": 1, "candidates": [
  {"id": "...", "cmd": [...], "env": {...}, "output": "path or null"}]}

Every repetition runs each candidate once in a fresh process. The candidate
order alternates (forward on even repetitions, reversed on odd ones). Each
sample records process wall time (time.monotonic around /usr/bin/time -l),
maximum resident set size from /usr/bin/time -l, the 1-minute load average
before and after, the candidate's stdout lines that carry its own timer
(`elapsed_ms=` or the CLI `report:` line), and the output size and SHA-256.
Warm-up samples are retained with "warmup": true and excluded from summaries.
This script never claims an idle host; the load averages are recorded so a
reader can judge contention.
"""

import hashlib
import json
import os
import platform
import statistics
import subprocess
import sys
import time
from pathlib import Path


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def sample(candidate):
    env = dict(os.environ, **candidate.get("env", {}))
    before = os.getloadavg()
    start = time.monotonic()
    result = subprocess.run(["/usr/bin/time", "-l", *candidate["cmd"]], env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    wall = time.monotonic() - start
    after = os.getloadavg()
    if result.returncode != 0:
        raise SystemExit(f"{candidate['id']} failed ({result.returncode}): {result.stderr[-2000:]}")
    rss = int(next(line.split()[0] for line in result.stderr.splitlines()
                   if "maximum resident set size" in line))
    lines = [line for line in result.stdout.splitlines()
             if "elapsed_ms=" in line or line.startswith(("report:", "decoder_work:"))]
    internal_ms = None
    for line in lines:
        if "elapsed_ms=" in line:
            internal_ms = float(line.split("elapsed_ms=")[1].split()[0])
        elif line.startswith("report:") and "total_us=" in line:
            internal_ms = int(line.split("total_us=")[1].split()[0]) / 1000
    row = {"id": candidate["id"], "wall_s": wall, "max_rss_bytes": rss,
           "internal_ms": internal_ms, "stdout": lines,
           "load_before": before[0], "load_after": after[0]}
    output = candidate.get("output")
    if output:
        row["output_bytes"] = Path(output).stat().st_size
        row["output_sha256"] = sha256(output)
    return row


def summarize(rows):
    walls = [r["wall_s"] for r in rows]
    rss = [r["max_rss_bytes"] for r in rows]
    internal = [r["internal_ms"] for r in rows if r["internal_ms"] is not None]
    out = {"n": len(rows), "wall_median_s": statistics.median(walls), "wall_min_s": min(walls),
           "wall_max_s": max(walls), "rss_median_bytes": statistics.median(rss),
           "rss_min_bytes": min(rss), "rss_max_bytes": max(rss),
           "load_min": min(min(r["load_before"], r["load_after"]) for r in rows),
           "load_max": max(max(r["load_before"], r["load_after"]) for r in rows)}
    if internal:
        out.update(internal_median_ms=statistics.median(internal), internal_min_ms=min(internal),
                   internal_max_ms=max(internal))
    hashes = {r.get("output_sha256") for r in rows if r.get("output_sha256")}
    if hashes:
        out["output_sha256"] = sorted(hashes)
        out["output_bytes"] = sorted({r["output_bytes"] for r in rows})
    return out


def main():
    plan = json.loads(Path(sys.argv[1]).read_text())
    out = Path(sys.argv[2])
    candidates = plan["candidates"]
    reps, warmups = int(plan.get("reps", 7)), int(plan.get("warmups", 1))
    uptime_before = subprocess.run(["uptime"], capture_output=True, text=True).stdout.strip()
    samples = []
    for rep in range(-warmups, reps):
        order = candidates if rep % 2 == 0 else list(reversed(candidates))
        for candidate in order:
            row = sample(candidate)
            row.update(rep=rep, warmup=rep < 0)
            samples.append(row)
            print(f"rep {rep} {row['id']}: wall {row['wall_s']:.3f}s rss {row['max_rss_bytes'] / 1e6:.1f}MB "
                  f"internal {row['internal_ms']} load {row['load_before']:.1f}", file=sys.stderr)
    uptime_after = subprocess.run(["uptime"], capture_output=True, text=True).stdout.strip()
    summary = {c["id"]: summarize([s for s in samples if s["id"] == c["id"] and not s["warmup"]])
               for c in candidates}
    record = {"plan": plan, "platform": platform.platform(), "machine": platform.machine(),
              "uptime_before": uptime_before, "uptime_after": uptime_after,
              "order": "alternating forward/reverse per repetition, fresh process per sample",
              "summary": summary, "samples": samples}
    out.write_text(json.dumps(record, indent=2) + "\n")


if __name__ == "__main__":
    main()
