#!/usr/bin/env python3
"""Summarize downloaded v1.0 CI performance artifacts as Markdown tables.

Usage: python3 -I summarize.py ARTIFACTS_DIR > tables.md

ARTIFACTS_DIR holds one directory per artifact (`gh run download`):
engine-<part>-<size>-<runner>/ and codec-<codec>-<runner>/, each with the
run_interleaved.py records, host-before.txt and host-after.txt.
"""

import json
import re
import sys
from pathlib import Path


def cpu(host):
    text = host.read_text(errors="replace") if host.exists() else ""
    model = re.search(r"Model name:\s*(.+)", text) or re.search(r"machdep.cpu.brand_string.*?\n?(Apple.+)", text)
    if not model:
        model = re.search(r"^(Apple M\S*.*)$", text, re.M)
    cpus = re.search(r"cpus: (\d+)", text)
    load = re.search(r"load averages?: ([\d., ]+)", text)
    return {"model": model.group(1).strip() if model else "?", "cpus": cpus.group(1) if cpus else "?",
            "load": load.group(1).strip() if load else "?"}


def ms(value):
    return "–" if value is None else (f"{value:,.0f}" if value >= 100 else f"{value:.1f}")


def spread(summary, key="internal"):
    med = summary.get(f"{key}_median_ms")
    if med is None:
        return "–"
    return f"{ms(med)} ({ms(summary[f'{key}_min_ms'])}–{ms(summary[f'{key}_max_ms'])})"


def wall(summary):
    if "wall_median_s" not in summary:
        return "failed"
    return (f"{ms(summary['wall_median_s'] * 1e3)} ({ms(summary['wall_min_s'] * 1e3)}–"
            f"{ms(summary['wall_max_s'] * 1e3)})")


def engine_tables(root):
    out = []
    for directory in sorted(root.glob("engine-*")):
        before, after = cpu(directory / "host-before.txt"), cpu(directory / "host-after.txt")
        for record_path in sorted(directory.glob("engine-*-*.json")):
            if "inputs" in record_path.name:
                continue
            record = json.loads(record_path.read_text())
            out.append(f"\n#### {directory.name} — {before['model']}, {before['cpus']} CPUs, "
                       f"load before {before['load']} / after {after['load']}\n")
            out.append("| Case | wall ms median (min–max) | open | decode | transform | encode | "
                       "decode % | transform % | encode % | managed MB | max RSS MB |")
            out.append("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
            for candidate in record["plan"]["candidates"]:
                s = record["summary"][candidate["id"]]
                r = s.get("report_median")
                if not r:
                    out.append(f"| {candidate['id']} | failed |  |  |  |  |  |  |  |  |  |")
                    continue
                stages = [r.get(k, 0) / 1e3 for k in ("open_us", "decode_us", "transform_us", "encode_us")]
                total = r.get("total_us", 0) / 1e3 or 1
                shares = [f"{100 * v / total:.0f}" for v in stages[1:]]
                out.append(
                    f"| {candidate['id']} | {wall(s)} | {ms(stages[0])} | {ms(stages[1])} | {ms(stages[2])} | "
                    f"{ms(stages[3])} | {shares[0]} | {shares[1]} | {shares[2]} | "
                    f"{r.get('peak_managed_bytes', 0) / 1e6:.1f} | {s.get('rss_median_bytes', 0) / 1e6:.1f} |")
    return out


def codec_tables(root):
    out = []
    for directory in sorted(root.glob("codec-*")):
        before, after = cpu(directory / "host-before.txt"), cpu(directory / "host-after.txt")
        records = [p for p in sorted(directory.glob("*.json"))
                   if not re.search(r"-(build|inputs|checks|skipped)\.json$", p.name)]
        if not records:
            continue
        out.append(f"\n#### {directory.name} — {before['model']}, {before['cpus']} CPUs, "
                   f"load before {before['load']} / after {after['load']}\n")
        out.append("| Group | Candidate | internal ms median (min–max) | wall ms median (min–max) | "
                   "max RSS MB | internal / C | wall / C |")
        out.append("| --- | --- | ---: | ---: | ---: | ---: | ---: |")
        for path in records:
            record = json.loads(path.read_text())
            if "plan" not in record:
                continue
            summary = record["summary"]
            group = record["plan"].get("group", path.stem)
            refs = {}
            for cid, s in summary.items():
                impl, _, threads = cid.partition(":")
                # The reference is the C build of the library the port
                # translates (for AVIF decode: libavif with libaom).
                if impl in ("c", "c-aom"):
                    refs[threads] = s
            for cid, s in summary.items():
                impl, _, threads = cid.partition(":")
                ref = refs.get(threads) or refs.get("t1")
                ratio_i = ratio_w = "–"
                if ref and not impl.startswith("c"):
                    if s.get("internal_median_ms") and ref.get("internal_median_ms"):
                        ratio_i = f"{s['internal_median_ms'] / ref['internal_median_ms']:.2f}×"
                    if s.get("wall_median_s") and ref.get("wall_median_s"):
                        ratio_w = f"{s['wall_median_s'] / ref['wall_median_s']:.2f}×"
                rss = s.get("rss_median_bytes")
                out.append(f"| {group} | {cid} | {spread(s)} | {wall(s)} | "
                           f"{'–' if rss is None else f'{rss / 1e6:.1f}'} | {ratio_i} | {ratio_w} |")
    return out


def main():
    root = Path(sys.argv[1])
    print("### Engine stage breakdown\n")
    print("\n".join(engine_tables(root)))
    print("\n### Port versus original\n")
    print("\n".join(codec_tables(root)))


if __name__ == "__main__":
    main()
