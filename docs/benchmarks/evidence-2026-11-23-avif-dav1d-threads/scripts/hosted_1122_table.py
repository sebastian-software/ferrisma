#!/usr/bin/env python3
"""Tabulate the hosted 2026-11-22 dav1d/aom thread-scaling records.

Reads run_interleaved.py records (decoders-<size>-<depth>-<chroma>.json) under
each run/runner directory and prints one Markdown row per run x runner x input.

Usage: python3 -I hosted_1122_table.py [SOURCE_DIR]
"""

import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
DEFAULT_SOURCE = os.path.normpath(
    os.path.join(HERE, "..", "..", "bench-decoders-2026-11-22")
)
SIZE_ORDER = {"web03": 0, "web14": 1, "12": 2, "24": 3}
NAME_RE = re.compile(r"^decoders-(web03|web14|12|24)-(\d+)-(\d+)\.json$")
CANDIDATES = {
    "dav1d_t1": "port-dav1d:t1",
    "dav1d_tN": "port-dav1d:tN",
    "aom_t1": "port-aom:t1",
    "aom_tN": "port-aom:tN",
}
HEADER = (
    "| run | runner | input | cpus | port-dav1d t1 ms | port-dav1d tN ms | "
    "speedup t1/tN | dav1d RSS t1 MB | dav1d RSS tN MB | port-aom t1 ms | "
    "port-aom tN ms | aom RSS t1 MB | aom RSS tN MB |"
)
RULE = "|" + "---|" * 13


def fail(message):
    print(message, file=sys.stderr)
    sys.exit(1)


def load_cell(path, data):
    summary = data.get("summary")
    if not isinstance(summary, dict):
        fail(f"{path}: no 'summary' object; top-level keys: {sorted(data)}")
    missing = [key for key in CANDIDATES.values() if key not in summary]
    if missing:
        fail(
            f"{path}: missing candidate keys {missing}; "
            f"existing summary keys: {sorted(summary)}"
        )
    cells = {}
    for name, key in CANDIDATES.items():
        entry = summary[key]
        for field in ("internal_median_ms", "rss_median_bytes"):
            if entry.get(field) is None:
                fail(f"{path}: {key} has no '{field}'; keys: {sorted(entry)}")
        cells[name] = (entry["internal_median_ms"], entry["rss_median_bytes"])
    return cells


def collect(source):
    rows = []
    for run in sorted(os.listdir(source)):
        run_dir = os.path.join(source, run)
        if not (run.startswith("hosted-") and os.path.isdir(run_dir)):
            continue
        for runner in sorted(os.listdir(run_dir)):
            runner_dir = os.path.join(run_dir, runner)
            if not os.path.isdir(runner_dir):
                continue
            files = []
            for name in os.listdir(runner_dir):
                match = NAME_RE.match(name)
                if match:
                    size, depth, chroma = match.groups()
                    files.append(
                        (
                            SIZE_ORDER[size],
                            int(depth),
                            int(chroma),
                            name,
                        )
                    )
            for size_rank, depth, chroma, name in sorted(files):
                path = os.path.join(runner_dir, name)
                with open(path, encoding="utf-8") as handle:
                    data = json.load(handle)
                cells = load_cell(path, data)
                cpus = data.get("cpus")
                rows.append((run, runner, name, cpus, cells))
    return rows


def format_row(run, runner, name, cpus, cells):
    input_label = name[len("decoders-") : -len(".json")]
    d1, d1_rss = cells["dav1d_t1"]
    dn, dn_rss = cells["dav1d_tN"]
    a1, a1_rss = cells["aom_t1"]
    an, an_rss = cells["aom_tN"]
    if dn == 0:
        fail(f"{run}/{runner}/{name}: port-dav1d tN median is zero")
    speedup = d1 / dn
    return (
        f"| {run} | {runner} | {input_label} | "
        f"{cpus if cpus is not None else 'missing'} | "
        f"{d1:.1f} | {dn:.1f} | {speedup:.2f} | "
        f"{d1_rss / 1e6:.1f} | {dn_rss / 1e6:.1f} | "
        f"{a1:.1f} | {an:.1f} | "
        f"{a1_rss / 1e6:.1f} | {an_rss / 1e6:.1f} |"
    )


def main():
    source = os.path.abspath(sys.argv[1] if len(sys.argv) > 1 else DEFAULT_SOURCE)
    if not os.path.isdir(source):
        fail(f"source directory not found: {source}")
    rows = collect(source)
    if not rows:
        fail(f"no decoders-<size>-<depth>-<chroma>.json records under {source}")
    print(f"Source directory: {source}")
    print()
    print(HEADER)
    print(RULE)
    for run, runner, name, cpus, cells in rows:
        print(format_row(run, runner, name, cpus, cells))


if __name__ == "__main__":
    main()
