#!/usr/bin/env python3
"""Interleaved process timing for the v1.0 CI performance runs.

Usage: python3 -I run_interleaved.py PLAN.json OUT.json

PLAN.json: {"reps": 7, "warmups": 1, "group": "...", "candidates": [
  {"id": "...", "cmd": [...], "env": {...}, "output": "path or null",
   "meta": {...}}]}

Derived from docs/benchmarks/evidence-2026-11-12-baseline/run_interleaved.py
and portable between Linux (GNU time -v) and macOS (time -l). Every
repetition runs each candidate once in a fresh process; the candidate order
alternates (forward on even repetitions, reversed on odd ones). Each sample
records the process wall time (time.monotonic around /usr/bin/time), the
maximum resident set size, the 1-minute load average before and after, and
the candidate's own timer when it prints one:

- `elapsed_ms=<float>` (decode harnesses),
- `report: ... total_us=<int>` (the ferrisma CLI; the whole line is kept),
- `nanoseconds=<int>` with `decodes=<int>` (libjpeg-turbo bench probes;
  internal_ms is per decode),
- `encode_us=<int>` (libavif-rs encode_still; the median of the printed
  repetitions is used).

Warm-up samples are kept with "warmup": true and excluded from summaries.
A candidate whose command fails is recorded with its stderr tail and marked
"failed"; the run continues with the other candidates.
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

LINUX = sys.platform.startswith("linux")
TIME = ["/usr/bin/time", "-v"] if LINUX else ["/usr/bin/time", "-l"]


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1 << 20), b""):
            digest.update(block)
    return digest.hexdigest()


def max_rss_bytes(stderr):
    for line in stderr.splitlines():
        if LINUX and "Maximum resident set size (kbytes)" in line:
            return int(line.rsplit(":", 1)[1]) * 1024
        if not LINUX and "maximum resident set size" in line:
            return int(line.split()[0])
    return None


def internal(stdout):
    lines, value = [], None
    fields = {}
    encode_us = []
    for line in stdout.splitlines():
        if "elapsed_ms=" in line:
            value = float(line.split("elapsed_ms=")[1].split()[0])
            lines.append(line)
        elif line.startswith("report:"):
            if "total_us=" in line:
                value = int(line.split("total_us=")[1].split()[0]) / 1000
            lines.append(line)
        elif line.startswith("decoder_work:"):
            lines.append(line)
        elif "encode_us=" in line:
            encode_us.append(int(line.split("encode_us=")[1].split()[0].rstrip(",")))
            lines.append(line)
        else:
            for token in line.split():
                if "=" in token:
                    key, _, val = token.partition("=")
                    if key in ("nanoseconds", "decodes", "dims", "checksum"):
                        fields[key] = val
    if "nanoseconds" in fields and "decodes" in fields:
        value = int(fields["nanoseconds"]) / int(fields["decodes"]) / 1e6
        lines.append(" ".join(f"{k}={v}" for k, v in sorted(fields.items())))
    if encode_us and value is None:
        value = statistics.median(encode_us) / 1000
    return value, lines


def sample(candidate):
    env = dict(os.environ, **candidate.get("env", {}))
    before = os.getloadavg()
    start = time.monotonic()
    result = subprocess.run([*TIME, *candidate["cmd"]], env=env,
                            stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True,
                            errors="replace")
    wall = time.monotonic() - start
    after = os.getloadavg()
    row = {"id": candidate["id"], "wall_s": wall, "load_before": before[0],
           "load_after": after[0], "returncode": result.returncode}
    if result.returncode != 0:
        row["failed"] = True
        row["stderr_tail"] = result.stderr[-1500:]
        row["stdout_tail"] = result.stdout[-500:]
        return row
    row["max_rss_bytes"] = max_rss_bytes(result.stderr)
    row["internal_ms"], row["stdout"] = internal(result.stdout)
    output = candidate.get("output")
    if output and Path(output).exists():
        row["output_bytes"] = Path(output).stat().st_size
        row["output_sha256"] = sha256(output)
    return row


def summarize(rows):
    ok = [r for r in rows if not r.get("failed")]
    out = {"n": len(ok), "failed": len(rows) - len(ok)}
    if not ok:
        return out
    walls = [r["wall_s"] for r in ok]
    out.update(wall_median_s=statistics.median(walls), wall_min_s=min(walls),
               wall_max_s=max(walls),
               load_min=min(min(r["load_before"], r["load_after"]) for r in ok),
               load_max=max(max(r["load_before"], r["load_after"]) for r in ok))
    rss = [r["max_rss_bytes"] for r in ok if r.get("max_rss_bytes") is not None]
    if rss:
        out.update(rss_median_bytes=statistics.median(rss), rss_max_bytes=max(rss))
    values = [r["internal_ms"] for r in ok if r.get("internal_ms") is not None]
    if values:
        out.update(internal_median_ms=statistics.median(values), internal_min_ms=min(values),
                   internal_max_ms=max(values))
    # CLI stage medians from the report line.
    stages = {}
    for r in ok:
        for line in r.get("stdout", []):
            if line.startswith("report:"):
                for token in line.split()[1:]:
                    key, _, val = token.partition("=")
                    if val.isdigit():
                        stages.setdefault(key, []).append(int(val))
    if stages:
        out["report_median"] = {k: statistics.median(v) for k, v in stages.items()}
    hashes = {r.get("output_sha256") for r in ok if r.get("output_sha256")}
    if hashes:
        out["output_sha256"] = sorted(hashes)
        out["output_bytes"] = sorted({r["output_bytes"] for r in ok})
    return out


def main():
    plan = json.loads(Path(sys.argv[1]).read_text())
    out = Path(sys.argv[2])
    candidates = plan["candidates"]
    reps, warmups = int(plan.get("reps", 7)), int(plan.get("warmups", 1))
    uptime_before = subprocess.run(["uptime"], capture_output=True, text=True).stdout.strip()
    samples = []
    disabled = set()
    for rep in range(-warmups, reps):
        order = candidates if rep % 2 == 0 else list(reversed(candidates))
        for candidate in order:
            if candidate["id"] in disabled:
                continue
            row = sample(candidate)
            row.update(rep=rep, warmup=rep < 0)
            samples.append(row)
            if row.get("failed"):
                # Do not repeat a failing command; keep its evidence.
                disabled.add(candidate["id"])
                print(f"FAILED {row['id']}: {row['stderr_tail'][-400:]}", file=sys.stderr)
                continue
            rss = row.get("max_rss_bytes") or 0
            print(f"rep {rep} {row['id']}: wall {row['wall_s']:.3f}s rss {rss / 1e6:.1f}MB "
                  f"internal {row['internal_ms']} load {row['load_before']:.1f}", file=sys.stderr)
        # Keep a complete record after every repetition, so a job timeout
        # still leaves the finished repetitions as evidence.
        write(out, plan, candidates, samples, uptime_before, complete=rep == reps - 1)


def write(out, plan, candidates, samples, uptime_before, complete):
    uptime_after = subprocess.run(["uptime"], capture_output=True, text=True).stdout.strip()
    summary = {c["id"]: summarize([s for s in samples if s["id"] == c["id"] and not s["warmup"]])
               for c in candidates}
    record = {"plan": plan, "platform": platform.platform(), "machine": platform.machine(),
              "cpus": os.cpu_count(), "uptime_before": uptime_before, "uptime_after": uptime_after,
              "order": "alternating forward/reverse per repetition, fresh process per sample",
              "complete": complete, "summary": summary, "samples": samples}
    out.parent.mkdir(parents=True, exist_ok=True)
    out.write_text(json.dumps(record, indent=2) + "\n")


if __name__ == "__main__":
    main()
