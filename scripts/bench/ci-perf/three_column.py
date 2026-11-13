#!/usr/bin/env python3
"""Three-column port-versus-original tables (decision D2) from CI artifacts.

Usage:
  python3 -I three_column.py ARTIFACTS_DIR [ARTIFACTS_DIR...] > tables.md
  python3 -I three_column.py --compact ARTIFACTS_DIR... > summary.md

ARTIFACTS_DIR holds one directory per workflow artifact (`gh run download`);
only the codec-<codec>-<runner>/ directories are read. Each codec script
(codec_<codec>.sh) puts the D2 candidates of one operation and input into one
interleaved plan, named by convention (or by meta.column):

  port:t1      the port, one thread
  c-scalar:t1  (a) the original built scalar, one thread (translation loss)
  c:t1         (b) the original's release build with SIMD restricted to the
               v1.0 scope (D6), one thread
  c:tN         (c) the same build with all threads it can use
  anything else in the plan is a context column (jxl-oxide, dav1d).

Per platform the full mode prints one table: rows are operations, cells are
internal-timer medians. "port" gives the port's median with min-max; each
reference column gives port/reference (above 1: the port is slower) and the
reference median in ms. "n/a" marks a reference the original does not have
(the plan's column_c_na says why); "–" marks one that was not measured.
The compact mode prints one table over all platforms with the three ratios
(a / b / c) per cell.
"""

import json
import re
import sys
from pathlib import Path

PLATFORMS = [("ubuntu-24.04", "GH x86-64"), ("blacksmith-4vcpu-ubuntu-2404", "BS x86-64"),
             ("ubuntu-24.04-arm", "GH ARM64"), ("blacksmith-4vcpu-ubuntu-2404-arm", "BS ARM64"),
             ("macos-15", "GH macOS ARM64")]
COLUMNS = {"c-scalar:t1": "a", "c:t1": "b", "c:tN": "c", "port:t1": "port"}
OPS = [("jpeg", "decode", "JPEG decode"),
       ("webp", "decode-lossy", "WebP lossy decode"),
       ("webp", "decode-lossless", "WebP lossless decode"),
       ("webp", "encode-lossy", "WebP lossy encode q75"),
       ("webp", "encode-lossless", "WebP lossless encode"),
       ("jxl", "decode-vardct", "JPEG XL VarDCT decode"),
       ("jxl", "decode-lossless", "JPEG XL lossless decode"),
       ("jxl", "encode-lossy", "JPEG XL lossy encode e7 d1"),
       ("jxl", "encode-lossless", "JPEG XL lossless encode e2"),
       ("avif", "decode", "AVIF decode (libaom)"),
       ("avif", "encode", "AVIF encode 10-bit 4:4:4 q75 s6")]
CONTEXT_NAMES = {"oxide:t1": "jxl-oxide, 1 thread", "c-dav1d:t1": "C dav1d, 1 thread",
                 "c-dav1d:tN": "C dav1d, all threads"}


def platform_of(name):
    for key, label in sorted(PLATFORMS, key=lambda p: -len(p[0])):
        if name.endswith(key):
            return label
    return None


def host(path):
    text = path.read_text(errors="replace") if path.exists() else ""
    model = re.search(r"Model name:\s*(.+)", text) or re.search(r"^(Apple M\S*.*)$", text, re.M)
    cpus = re.search(r"cpus: (\d+)", text)
    return {"model": model.group(1).strip() if model else "?",
            "cpus": cpus.group(1) if cpus else "?"}


def ms(value):
    if value is None:
        return "–"
    return f"{value:,.0f}" if value >= 100 else f"{value:.1f}"


def input_label(op, group):
    name = group.rsplit("/", 1)[-1]
    size = re.search(r"photo(\d+)", name)
    mp = f"{size.group(1)} MP" if size else name
    if name.endswith("-original"):
        return f"{mp} camera file"
    if name.endswith("-q90"):
        return f"{mp} q90"
    return mp


def column_of(cid, candidate):
    meta = candidate.get("meta", {})
    return meta.get("column") or COLUMNS.get(cid) or "context"


def load(roots):
    """{platform: {"hosts": {...}, "levels": {...}, "rows": [...]}}"""
    data = {}
    for root in roots:
        for d in sorted(Path(root).iterdir()):
            if not d.is_dir() or not d.name.startswith("codec-"):
                continue
            plat = platform_of(d.name)
            if not plat:
                continue
            entry = data.setdefault(plat, {"hosts": {}, "levels": {}, "rows": [], "loads": []})
            codec = d.name[len("codec-"):].split("-")[0]
            entry["hosts"][codec] = host(d / "host-before.txt")
            build = next(iter(d.glob(f"{codec}-build.json")), None)
            if build:
                columns = json.loads(build.read_text()).get("columns", {})
                entry["levels"][codec] = columns.get("b", {}).get("simd_level")
            for f in sorted(d.glob("*.json")):
                record = json.loads(f.read_text())
                if "plan" not in record:
                    continue
                plan, summary = record["plan"], record["summary"]
                group = plan.get("group", f.stem)
                gcodec, op = group.split("/")[:2]
                cells = {"context": {}}
                for c in plan["candidates"]:
                    s = summary.get(c["id"], {})
                    col = column_of(c["id"], c)
                    value = {"median": s.get("internal_median_ms"), "min": s.get("internal_min_ms"),
                             "max": s.get("internal_max_ms"), "n": s.get("n", 0),
                             "failed": s.get("failed", 0)}
                    if col == "context":
                        cells["context"][c["id"]] = value
                    else:
                        cells[col] = value
                    if s.get("load_max") is not None:
                        entry["loads"].append((s["load_min"], s["load_max"]))
                entry["rows"].append({"codec": gcodec, "op": op, "group": group,
                                      "input": input_label(op, group), "cells": cells,
                                      "c_na": plan.get("column_c_na"),
                                      "complete": record.get("complete", True)})
    return data


def order(row):
    for i, (codec, op, _) in enumerate(OPS):
        if row["codec"] == codec and row["op"] == op:
            size = re.search(r"(\d+) MP", row["input"])
            return (i, int(size.group(1)) if size else 0, row["input"])
    return (len(OPS), 0, row["group"])


def op_label(row):
    for codec, op, label in OPS:
        if row["codec"] == codec and row["op"] == op:
            return label
    return f"{row['codec']} {row['op']}"


def ratio(port, ref):
    if not port or not ref or port.get("median") is None or ref.get("median") is None:
        return None
    return port["median"] / ref["median"]


def ref_cell(row, col):
    port, ref = row["cells"].get("port"), row["cells"].get(col)
    if col == "c" and ref is None and row["c_na"]:
        return "n/a"
    r = ratio(port, ref)
    if r is None:
        return "failed" if ref and ref.get("failed") else "–"
    return f"**{r:.2f}×** ({ms(ref['median'])})"


def port_cell(row):
    p = row["cells"].get("port")
    if not p or p.get("median") is None:
        return "failed" if p and p.get("failed") else "–"
    star = "" if row["complete"] else " †"
    return f"{ms(p['median'])} ({ms(p['min'])}–{ms(p['max'])}){star}"


def full(data):
    out = []
    for _, plat in PLATFORMS:
        if plat not in data:
            continue
        entry = data[plat]
        rows = sorted(entry["rows"], key=order)
        models = sorted({f"{h['model']} ({h['cpus']} vCPUs)" for h in entry["hosts"].values()})
        loads = entry["loads"]
        out.append(f"\n### {plat}\n")
        out.append("- CPU per job: " + "; ".join(
            f"{codec}: {h['model']}, {h['cpus']} vCPUs" for codec, h in sorted(entry["hosts"].items())))
        out.append("- Column (b) SIMD level: " + "; ".join(
            f"{codec}: {lvl or '?'}" for codec, lvl in sorted(entry["levels"].items())))
        if loads:
            out.append(f"- 1-minute load average during samples: {min(l[0] for l in loads):.1f}–"
                       f"{max(l[1] for l in loads):.1f}")
        if len(models) > 1:
            out.append("- Codec jobs ran on different CPU models; compare ratios, not "
                       "absolute times, across codecs.")
        contexts = sorted({cid for r in rows for cid in r["cells"]["context"]})
        head = ["Operation", "Input", "Port ms (min–max)", "× (a) scalar, 1 thread",
                "× (b) SIMD, 1 thread", "× (c) SIMD, all threads"]
        head += [f"× {CONTEXT_NAMES.get(c, c)}" for c in contexts]
        out.append("")
        out.append("| " + " | ".join(head) + " |")
        out.append("| --- | --- | " + " | ".join("---:" for _ in head[2:]) + " |")
        for row in rows:
            cells = [op_label(row), row["input"], port_cell(row),
                     ref_cell(row, "a"), ref_cell(row, "b"), ref_cell(row, "c")]
            for c in contexts:
                ctx = row["cells"]["context"].get(c)
                r = ratio(row["cells"].get("port"), ctx)
                cells.append("" if ctx is None else
                             (f"{r:.2f}× ({ms(ctx['median'])})" if r is not None else "–"))
            out.append("| " + " | ".join(cells) + " |")
    out.append("\nCells: port/reference ratio of the internal-timer medians (above 1: the "
               "port is slower), reference median in ms in parentheses. n/a: the original "
               "has no threading for this operation. † partial record (job limit).")
    return out


def compact(data):
    plats = [p for _, p in PLATFORMS if p in data]
    rows = {}
    for plat in plats:
        for row in data[plat]["rows"]:
            key = (order(row), op_label(row), row["input"])
            cell = []
            for col in ("a", "b", "c"):
                c = ref_cell(row, col)
                cell.append(c.split("×")[0].strip("* ") if "×" in c else c)
            rows.setdefault(key, {})[plat] = " / ".join(cell)
    out = ["| Operation | Input | " + " | ".join(plats) + " |",
           "| --- | --- | " + " | ".join("---:" for _ in plats) + " |"]
    for key in sorted(rows):
        out.append(f"| {key[1]} | {key[2]} | " +
                   " | ".join(rows[key].get(p, "–") for p in plats) + " |")
    out.append("\nCells: port time divided by (a) the scalar original, one thread / (b) the "
               "SIMD original at the D6 level, one thread / (c) the SIMD original with all "
               "threads; above 1 the port is slower.")
    return out


def main():
    args = sys.argv[1:]
    mode = full
    if args and args[0] == "--compact":
        mode, args = compact, args[1:]
    if not args:
        raise SystemExit(__doc__)
    print("\n".join(mode(load(args))))


if __name__ == "__main__":
    main()
