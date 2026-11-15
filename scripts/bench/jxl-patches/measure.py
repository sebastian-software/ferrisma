#!/usr/bin/env python3
"""Size and quality cost of disabling JPEG XL patches (issue #236).

Usage: measure.py CORPUS_DIR BUILD_TOOLS_DIR OUT_JSON [WORK_DIR]

CORPUS_DIR       the directory make_corpus.py wrote (PPM files and corpus.json)
BUILD_TOOLS_DIR  the tools/ directory of the pinned original libjxl build
                 (cjxl, djxl, ssimulacra2); see build_libjxl.sh
OUT_JSON         raw results
WORK_DIR         scratch directory for encoded and decoded files (default: a
                 fresh temporary directory, removed afterwards)

For every image, at effort 7 and distances 1.0 and 2.0, it encodes three times
with `cjxl --num_threads=0`: with default settings, with `--patches=0` (what
the engine's port writes) and with `--patches=1` (patch search forced on; this also turns
streaming off) and with `--buffering=0 --patches=0` (streaming off, patches
off: separates the effect of patches from the effect of leaving streaming),
decodes each with `djxl`, and scores each decode against the source PPM with
SSIMULACRA2. Patches are reported as used in the default encode when it
differs from the `--patches=0` file (the tool prints no flag for it; with
patches off the frame header flag is false, so equal files mean none were
written). Also records the number of 256x256 groups: at effort 7 and
distance below 3 libjxl encodes in streaming mode, which skips the patch
search, when the frame has more than 8 groups. Standard library only. No
timing is recorded: this is a size and quality measurement.
"""
import hashlib
import json
import statistics
import subprocess
import sys
import tempfile
from pathlib import Path

DISTANCES = ["1.0", "2.0"]
EFFORT = "7"


def run(*args):
    return subprocess.run([str(a) for a in args], check=True, capture_output=True, text=True)


def version(tool):
    return (run(tool, "--version").stdout + run(tool, "--version").stderr).splitlines()[0].strip()


def score(ssim, ref, dec):
    out = run(ssim, ref, dec)
    return float((out.stdout + out.stderr).split()[-1])


def main():
    corpus, tools, out_json = Path(sys.argv[1]), Path(sys.argv[2]), Path(sys.argv[3])
    cjxl, djxl, ssim = tools / "cjxl", tools / "djxl", tools / "ssimulacra2"
    meta = json.loads((corpus / "corpus.json").read_text())
    tmp = None
    if len(sys.argv) > 4:
        work = Path(sys.argv[4])
        work.mkdir(parents=True, exist_ok=True)
    else:
        tmp = tempfile.TemporaryDirectory()
        work = Path(tmp.name)
    rows = []
    for img in meta["images"]:
        src = corpus / f"{img['name']}.ppm"
        for d in DISTANCES:
            row = dict(name=img["name"], kind=img["kind"], width=img["width"], height=img["height"],
                       distance=float(d), effort=int(EFFORT))
            files = {}
            row["groups"] = -(-img["width"] // 256) * -(-img["height"] // 256)
            for mode, extra in (("default", []), ("patches0", ["--patches=0"]), ("patches1", ["--patches=1"]),
                                ("nostream0", ["--buffering=0", "--patches=0"])):
                jxl = work / f"{img['name']}-d{d}-{mode}.jxl"
                dec = work / f"{img['name']}-d{d}-{mode}.ppm"
                run(cjxl, src, jxl, "-e", EFFORT, "-d", d, "--num_threads=0", *extra)
                run(djxl, jxl, dec, "--num_threads=0")
                data = jxl.read_bytes()
                files[mode] = data
                row[mode] = dict(bytes=len(data), sha256=hashlib.sha256(data).hexdigest(),
                                 ssimulacra2=score(ssim, src, dec))
                dec.unlink()
                jxl.unlink()
            row["patches_used"] = files["default"] != files["patches0"]
            row["size_ratio"] = row["patches0"]["bytes"] / row["default"]["bytes"]
            row["ssimulacra2_delta"] = row["patches0"]["ssimulacra2"] - row["default"]["ssimulacra2"]
            row["size_ratio_streaming_off"] = row["nostream0"]["bytes"] / row["patches1"]["bytes"]
            row["ssimulacra2_delta_streaming_off"] = row["nostream0"]["ssimulacra2"] - row["patches1"]["ssimulacra2"]
            row["forced_used"] = files["patches1"] != files["patches0"]
            row["size_ratio_vs_forced"] = row["patches0"]["bytes"] / row["patches1"]["bytes"]
            row["ssimulacra2_delta_vs_forced"] = row["patches0"]["ssimulacra2"] - row["patches1"]["ssimulacra2"]
            rows.append(row)
            print(f"{img['name']:22} d{d} default {row['default']['bytes']:>9} patches0 {row['patches0']['bytes']:>9} "
                  f"ratio {row['size_ratio']:.4f} used={row['patches_used']} forced {row['patches1']['bytes']:>9} ({row['size_ratio_vs_forced']:.3f}) "
                  f"ssim2 {row['default']['ssimulacra2']:.2f} -> {row['patches0']['ssimulacra2']:.2f}", flush=True)
    result = dict(
        tools={t.name: version(t) for t in (cjxl, djxl)},
        ssimulacra2="tools/ssimulacra2 from the same build (prints a score only, no version string)",
        settings=dict(effort=int(EFFORT), distances=[float(x) for x in DISTANCES], threads="--num_threads=0"),
        corpus=dict(pillow=meta["pillow"], numpy=meta["numpy"]),
        results=rows,
    )
    out_json.write_text(json.dumps(result, indent=1) + "\n")
    print("\nsummary: size ratio = patches-off / other, SSIMULACRA2 delta = patches-off minus other")
    classes = (("<=8 groups", lambda r: r["groups"] <= 8), (">8 groups", lambda r: r["groups"] > 8))
    for d in DISTANCES:
        for cname, cf in classes:
            for kind in dict.fromkeys(r["kind"] for r in rows):
                sel = [r for r in rows if r["kind"] == kind and r["distance"] == float(d) and cf(r)]
                if not sel:
                    continue
                a_ = [r["size_ratio"] for r in sel]
                f_ = [r["size_ratio_vs_forced"] for r in sel]
                q = [r["ssimulacra2_delta"] for r in sel]
                qf = [r["ssimulacra2_delta_vs_forced"] for r in sel]
                iso = [r["size_ratio_streaming_off"] for r in sel]
                qiso = [r["ssimulacra2_delta_streaming_off"] for r in sel]
                used = sum(r["patches_used"] for r in sel)
                fused = sum(r["forced_used"] for r in sel)
                print(f"d{d} {cname:10} {kind:14} n={len(sel)} default-used={used} forced-used={fused} "
                      f"vs default: med {statistics.median(a_):.4f} min {min(a_):.4f} max {max(a_):.4f} dQ {statistics.median(q):+.2f} | "
                      f"vs forced: med {statistics.median(f_):.3f} min {min(f_):.3f} max {max(f_):.3f} dQ {statistics.median(qf):+.2f} | "
                      f"patches only: med {statistics.median(iso):.3f} min {min(iso):.3f} max {max(iso):.3f} dQ {statistics.median(qiso):+.2f}")
    if tmp:
        tmp.cleanup()


if __name__ == "__main__":
    main()
