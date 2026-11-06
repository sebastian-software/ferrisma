#!/usr/bin/env python3
"""ADR-0025 JPEG decoder comparison: libjpeg-turbo, jpegli and zune-jpeg.

usage: run.py WORK_DIR CORPUS_DIR SYNTH_DIR OUT_DIR

WORK_DIR    output of build.sh (bin/jpegdec-turbo, bin/jpegdec-jpegli, bin/zune,
            bin/cjpeg-turbo)
CORPUS_DIR  compare-sharp.py's corpus (dog.jpg, flower.jpg, kodim03-y4m.png,
            kodim23-y4m.png)
SYNTH_DIR   make-corpus.sh's single/ directory (24 and 48 Mpx plasma JPEGs)
OUT_DIR     receives speed.tsv, quality.tsv, scaled.tsv and the work files

Three measurements, all single-threaded (none of the decoders threads):
1. speed: best and median of N decodes from memory, full size and, for the
   two libjpeg-API decoders, at scale 1/2, 1/4 and 1/8.
2. quality: lossless originals (two Kodak images, and the dog and flower
   photos downscaled 4x and 3x in linear light so JPEG artifacts vanish) are
   encoded by libjpeg-turbo's cjpeg at q50/75/90 4:2:0 and q90 4:4:4, then
   each decoder's output is scored with SSIMULACRA2 against the original.
3. scaled: shrink-on-load loss per libjpeg-API decoder (ADR-0023). For each
   target size, the decoder's own full decode resized in linear light is the
   reference; the candidate is its scaled decode resized the same way.
Needs ImageMagick (`magick`) to write PPM input for cjpeg.
"""

import subprocess
import sys
from pathlib import Path

HERE = Path(__file__).resolve().parent
QM = HERE.parent / "quality-match" / "target" / "release" / "ferrixel-quality-match"


def run(cmd):
    return subprocess.run([str(c) for c in cmd], check=True, capture_output=True, text=True).stdout


def load():
    return run(["sysctl", "-n", "vm.loadavg"]).split()[1]


def main():
    work, corpus, synth, out = (Path(a).resolve() for a in sys.argv[1:5])
    bin_ = work / "bin"
    turbo, jpegli, zune, cjpeg = (bin_ / n for n in ("jpegdec-turbo", "jpegdec-jpegli", "zune", "cjpeg-turbo"))
    out.mkdir(parents=True, exist_ok=True)
    tmp = out / "work"
    tmp.mkdir(exist_ok=True)

    # 1. speed
    speed_inputs = [
        (corpus / "kodim03.jpg", 60),
        (corpus / "flower.jpg", 30),
        (corpus / "dog.jpg", 15),
        (synth / "synthetic-24mpx-6000x4000.jpg", 9),
        (synth / "synthetic-48mpx-8000x6000.jpg", 7),
    ]
    with open(out / "speed.tsv", "w") as f:
        f.write("decoder\tfile\tscale\tsize\ttype\tbest_ms\tmedian_ms\tload\n")
        for path, reps in speed_inputs:
            rows = [run([zune, path, reps])]
            for denom in (1, 2, 4, 8):
                rows.append(run([turbo, path, denom, reps]))
                rows.append(run([jpegli, path, denom, reps]))
                rows.append(run([jpegli, path, denom, reps, tmp / "x.ref", "f32"]))
            for row in rows:
                d, p, s, size, t, best, med = row.strip().split("\t")
                f.write(f"{d}\t{Path(p).name}\t{s}\t{size}\t{t}\t{best[5:]}\t{med[7:]}\t{load()}\n")
            f.flush()
            print(f"speed: {path.name}", file=sys.stderr)

    # 2. quality
    originals = {
        "kodim03": corpus / "kodim03-y4m.png",
        "kodim23": corpus / "kodim23-y4m.png",
    }
    for name, src, w, h in (("dog", "dog.jpg", 1008, 756), ("flower", "flower.jpg", 756, 504)):
        ref = tmp / f"{name}-clean.ref"
        run([QM, "ref", corpus / src, w, h, ref])
        run([QM, "topng", ref, tmp / f"{name}-clean.png"])
        originals[name] = tmp / f"{name}-clean.png"
    settings = [("q50-420", ["-quality", "50"]), ("q75-420", ["-quality", "75"]),
                ("q90-420", ["-quality", "90"]), ("q90-444", ["-quality", "90", "-sample", "1x1"])]
    with open(out / "quality.tsv", "w") as f:
        f.write("image\tsetting\tbytes\tlibjpeg-turbo\tjpegli-u8\tjpegli-f32\tzune-jpeg\n")
        for name, png in originals.items():
            ppm = tmp / f"{name}.ppm"
            run(["magick", png, ppm])
            for label, args in settings:
                jpg = tmp / f"{name}-{label}.jpg"
                run([cjpeg, *args, "-outfile", jpg, ppm])
                cands = [tmp / f"{name}-{label}-{d}.ref" for d in ("turbo", "jpegli8", "jpeglif", "zune")]
                run([turbo, jpg, 1, 1, cands[0]])
                run([jpegli, jpg, 1, 1, cands[1]])
                run([jpegli, jpg, 1, 1, cands[2], "f32"])
                run([zune, jpg, 1, cands[3]])
                scores = [line.split("\t")[2] for line in run([QM, "score", png, *cands]).strip().split("\n")]
                f.write(f"{name}\t{label}\t{jpg.stat().st_size}\t" + "\t".join(scores) + "\n")
            print(f"quality: {name}", file=sys.stderr)

    # 3. scaled decode (shrink-on-load)
    with open(out / "scaled.tsv", "w") as f:
        f.write("image\ttarget\tdecoder\tscale\tdecoded\tscore\n")
        for name, src, sw, sh in (("dog", "dog.jpg", 4032, 3024), ("flower", "flower.jpg", 2268, 1512)):
            for tw in (400, 1000):
                th = round(sh * tw / sw)
                for dec, cmd, extra in (("libjpeg-turbo", turbo, []), ("jpegli-f32", jpegli, ["f32"])):
                    full = tmp / f"{name}-{dec}-full.ref"
                    run([cmd, corpus / src, 1, 1, full, *extra])
                    reference = tmp / f"{name}-{tw}-{dec}-reference.ref"
                    run([QM, "ref", full, tw, th, reference])
                    for denom in (2, 4, 8):
                        if sw / denom < tw:
                            continue
                        scaled = tmp / f"{name}-{dec}-{denom}.ref"
                        line = run([cmd, corpus / src, denom, 1, scaled, *extra]).split("\t")
                        resized = tmp / f"{name}-{tw}-{dec}-{denom}.ref"
                        run([QM, "ref", scaled, tw, th, resized])
                        score = run([QM, "score", reference, resized]).split("\t")[2].strip()
                        f.write(f"{name}\t{tw}x{th}\t{dec}\t1/{denom}\t{line[3]}\t{score}\n")
            print(f"scaled: {name}", file=sys.stderr)


if __name__ == "__main__":
    main()
