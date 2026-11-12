#!/usr/bin/env python3
"""Quality-matched Ferrisma versus Sharp comparison (M2 work package 0).

Usage: python3 -I scripts/bench/compare-sharp.py PHASE OUT_DIR
PHASE is prepare, ceiling, match, time, batch, stages, sweepmatch, sweep,
encoder-match, encoder-check, encoder-time or all. See docs/benchmarks/M2-sharp-comparison.md
and docs/benchmarks/encoder-adoption.md for the respective methods.

Environment:
  BENCH_SHARP_NODE_MODULES  node_modules directory containing sharp (required)
  COMPARE_LIBAVIF_DATA      libavif tests/data (default: imported local fixtures)
  COMPARE_LIBJXL_DATA       libjxl decode-test (default: imported local fixtures)
  COMPARE_REPO              Ferrisma checkout to build and measure (default:
                            the checkout containing this script)
  COMPARE_RUNS              timed runs per measurement (default 10)
  COMPARE_MAX_LOAD          wait until the 1-minute load average is below
                            this before every timing run (default 4)
  COMPARE_ENCODER_LEVELS    comma-separated rav1e quality anchors (default 80)
  COMPARE_ENCODER_RUN_ID    unique identifier for each encoder timing run
  COMPARE_IDLE_TIMEOUT      bounded encoder idle wait in seconds (default 900)

Only `time`, `batch`, `stages`, `sweep` and `encoder-time` measure time;
matching produces bytes and SSIMULACRA2 scores without timing them.
"""

import csv
import hashlib
import math
import json
import platform
import re
import shlex
import os
import statistics
import subprocess
import sys
import time
from pathlib import Path

BENCH = Path(__file__).resolve().parent
# The Ferrisma checkout whose CLI is measured; defaults to this script's.
REPO = Path(os.environ.get("COMPARE_REPO", BENCH.parents[1])).resolve()


def resolve_target(path, repo):
    path = Path(path)
    return (path if path.is_absolute() else repo / path).resolve()


TARGET_PATH = Path(os.environ.get("CARGO_TARGET_DIR", REPO / "target"))
TARGET = resolve_target(TARGET_PATH, REPO)
if "CARGO_TARGET_DIR" in os.environ:
    os.environ["CARGO_TARGET_DIR"] = str(TARGET)
FERRISMA = TARGET / "release" / "ferrisma"
BATCH = TARGET / "release" / "examples" / "batch"
QM_DIR = BENCH / "quality-match"
QM_TARGET = TARGET if "CARGO_TARGET_DIR" in os.environ else QM_DIR / "target"
QM = QM_TARGET / "release" / "ferrisma-quality-match"
LIBAVIF_ENCODER = TARGET / "release" / "ferrisma-libavif"
SHARP_AVIF = BENCH / "sharp-avif.cjs"
SHARP_BATCH = BENCH / "sharp-batch.cjs"

# name, source file in the corpus, Ferrisma profile flag, Sharp ICC mode,
# output width, height, EXIF orientation (for the zune-jpeg reference).
IMAGES = [
    ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6),
    ("flower", "flower.jpg", "--assume-srgb", "convert", 1600, 1067, 1),
    ("kodim03", "kodim03.jpg", "--assume-srgb", "convert", 512, 341, 1),
    ("kodim23", "kodim23.jpg", "--assume-srgb", "convert", 512, 341, 1),
]
IMAGE_SET = os.environ.get("COMPARE_IMAGE_SET", "default")
LARGE_CROPS = {"beach-24": "1752,1168,6000,4000", "beach-48": "752,168,8000,6000"}
if IMAGE_SET in ("dog", "flower", "kodim03", "kodim23"):
    IMAGES = [image for image in IMAGES if image[0] == IMAGE_SET]
elif IMAGE_SET in ("mountain", "deer", "elcito"):
    flag = "--assume-srgb" if IMAGE_SET == "elcito" else "--use-rendered-icc"
    IMAGES = [(IMAGE_SET, f"{IMAGE_SET}.jpg", flag, "ignore", 1600, 1067, 1)]
elif IMAGE_SET != "default":
    if IMAGE_SET not in ("beach", *LARGE_CROPS):
        sys.exit(f"unknown COMPARE_IMAGE_SET: {IMAGE_SET}")
    height = 1200 if IMAGE_SET == "beach-48" else 1067
    IMAGES = [(IMAGE_SET, "beach.jpg", "--use-rendered-icc", "ignore", 1600, height, 1)]
SMALL = (320, 213)
QUALITY_RANGE = (30, 95)


def run(args, env=None, capture=True):
    merged = dict(os.environ, **(env or {}))
    result = subprocess.run([str(a) for a in args], env=merged, check=True,
                            stdout=subprocess.PIPE if capture else subprocess.DEVNULL, text=True)
    return result.stdout if capture else ""


def node_env(extra=None):
    modules = os.environ.get("BENCH_SHARP_NODE_MODULES")
    if not modules:
        sys.exit("set BENCH_SHARP_NODE_MODULES to a node_modules directory containing sharp")
    env = {"NODE_PATH": modules}
    env.update(extra or {})
    return env


def load1():
    return os.getloadavg()[0]


def wait_for_idle():
    limit = float(os.environ.get("COMPARE_MAX_LOAD", "4"))
    while (load := load1()) >= limit:
        print(f"  load {load:.1f} >= {limit}, waiting 60 s", file=sys.stderr)
        time.sleep(60)
    return load


def ferrisma_cmd(src, out, w, h, flag, quality, threads, chroma="444", speed=6, crop=None):
    # The recorded comparisons calibrate rav1e's quality scale; since the
    # workspace libavif writer became the `cli` default, select rav1e explicitly.
    cmd = [FERRISMA, src, out, w, h, flag, "--avif-encoder=rav1e", f"--quality={quality}",
           f"--encoder-threads={threads}", f"--transform-threads={threads}", f"--chroma={chroma}", f"--speed={speed}"]
    return [*cmd, f"--crop={crop}"] if crop else cmd


def sharp_cmd(src, out, w, h, icc, quality, threads, chroma="444", extra=()):
    return ["node", SHARP_AVIF, src, out, w, h, f"quality={quality}", f"icc={icc}",
            f"threads={threads}", f"chroma={chroma}", *extra]


# Matched configurations: tool, label, threads (0 = all cores), chroma.
CONFIGS = [
    ("ferrisma", "ferrisma-all-444", 0, "444"),
    ("sharp", "sharp-all-444", 0, "444"),
    ("ferrisma", "ferrisma-one-444", 1, "444"),
    ("sharp", "sharp-one-444", 1, "444"),
    ("ferrisma", "ferrisma-all-420", 0, "420"),
    ("sharp", "sharp-all-420", 0, "420"),
]

if os.environ.get("COMPARE_CONFIGS", "all") == "all-444":
    CONFIGS = [config for config in CONFIGS if config[1] in ("ferrisma-all-444", "sharp-all-444")]
elif os.environ.get("COMPARE_CONFIGS", "all") != "all":
    sys.exit("COMPARE_CONFIGS must be all or all-444")
TARGETS = os.environ.get("COMPARE_TARGETS", "AB")
if TARGETS not in ("A", "AB"):
    sys.exit("COMPARE_TARGETS must be A or AB")


def config_cmd(tool, threads, chroma, image, corpus, quality, out):
    name, file, flag, icc, w, h, _ = image
    crop = LARGE_CROPS.get(name)
    if tool == "ferrisma":
        cmd = ferrisma_cmd(corpus / file, out, w, h, flag, quality, threads, chroma, crop=crop)
        return [*cmd, "--budget-mb=2048"] if name.startswith("beach") or name in ("mountain", "deer", "elcito") else cmd
    return sharp_cmd(corpus / file, out, w, h, icc, quality, threads, chroma,
                     extra=(f"crop={crop}",) if crop else ())


def sha256(path):
    digest = hashlib.sha256()
    with open(path, "rb") as f:
        for block in iter(lambda: f.read(1024 * 1024), b""):
            digest.update(block)
    return digest.hexdigest()


def verify_corpus_file(path, image_id, manifest_name="corpus.json"):
    manifest = json.loads((BENCH / manifest_name).read_text())
    entry = next(image for image in manifest["images"] if image["id"] == image_id)
    if sha256(path) != entry["sha256"]:
        raise ValueError(f"corpus SHA-256 mismatch: {image_id}: {path}")


def prepare(out):
    corpus = out / "corpus"
    refs = out / "refs"
    small = corpus / "small"
    for d in (corpus, refs, small):
        d.mkdir(parents=True, exist_ok=True)
    if IMAGE_SET.startswith("beach") or IMAGE_SET in ("mountain", "deer", "elcito"):
        prepare_large(corpus, refs)
        return
    avif = Path(os.environ.get("COMPARE_LIBAVIF_DATA", REPO / "codecs/libavif-rs/upstream/libavif/tests/data"))
    jxl = Path(os.environ.get("COMPARE_LIBJXL_DATA", REPO / "codecs/libjxl-rs/tests/data/decode-test"))
    verify_corpus_file(avif / "dog_exif_extended_xmp_icc.jpg", "dog")
    verify_corpus_file(jxl / "flower.png.im_q85_420.jpg", "flower")
    print("building ferrisma CLI, batch example and quality-match", file=sys.stderr)
    build = ["cargo", "build", "--release", "--features", "cli", "--bin", "ferrisma"]
    if IMAGE_SET == "default":
        build += ["--example", "batch"]
    run(build, capture=False)
    run(["cargo", "build", "--release", "--manifest-path", QM_DIR / "Cargo.toml"], capture=False)
    (corpus / "dog.jpg").write_bytes((avif / "dog_exif_extended_xmp_icc.jpg").read_bytes())
    (corpus / "flower.jpg").write_bytes((jxl / "flower.png.im_q85_420.jpg").read_bytes())
    verify_corpus_file(corpus / "dog.jpg", "dog")
    verify_corpus_file(corpus / "flower.jpg", "flower")
    env = node_env()
    for kodak in ("kodim03", "kodim23"):
        # y4m -> 8-bit PNG (BT.601 full range) -> JPEG q95 4:2:0 (mozjpeg):
        # the only lossy step before the pipelines; both read the same JPEG.
        run([QM, "topng", avif / f"{kodak}_yuv420_8bpc.y4m", corpus / f"{kodak}-y4m.png"])
        run(["node", "-e", 'require("sharp")(process.argv[1]).jpeg({quality:95,chromaSubsampling:"4:2:0"})'
             '.toFile(process.argv[2])', corpus / f"{kodak}-y4m.png", corpus / f"{kodak}.jpg"], env=env)
    for name, file, _, icc, w, h, orientation in IMAGES:
        # Main reference: libjpeg-turbo-family decode (Sharp's mozjpeg),
        # oriented, sRGB. Second reference: zune-jpeg (Ferrisma's decoder).
        src = refs / f"{name}-src.png"
        run(["node", "-e", 'const [i,o,icc]=process.argv.slice(1);require("sharp")(i,{ignoreIcc:icc==="ignore"})'
             '.autoOrient().toColourspace("srgb").png({compressionLevel:1}).toFile(o)', corpus / file, src, icc], env=env)
        run([QM, "ref", src, w, h, refs / f"{name}.ref"])
        run([QM, "ref", f"{corpus / file}#{orientation}", w, h, refs / f"{name}-zune.ref"])
    if IMAGE_SET != "default":
        return
    # Many small images: 100 deterministic 640x427 crops of the four photos,
    # JPEG q90 4:2:0, no metadata (so --assume-srgb applies).
    script = r"""
const sharp = require("sharp");
const [corpus, outdir] = process.argv.slice(1);
const plan = [["dog.jpg", 50], ["flower.jpg", 40], ["kodim03.jpg", 5], ["kodim23.jpg", 5]];
(async () => {
  let n = 0;
  for (const [file, count] of plan) {
    const oriented = await sharp(`${corpus}/${file}`).autoOrient().toColourspace("srgb").raw()
      .toBuffer({ resolveWithObject: true });
    const { width, height, channels } = oriented.info;
    for (let i = 0; i < count; i++) {
      const fx = (i * 0.6180339887) % 1, fy = (i * 0.4142135624 + 0.5) % 1;
      const left = Math.floor((width - 640) * fx), top = Math.floor((height - 427) * fy);
      n++;
      await sharp(oriented.data, { raw: { width, height, channels } })
        .extract({ left, top, width: 640, height: 427 })
        .jpeg({ quality: 90, chromaSubsampling: "4:2:0" })
        .toFile(`${outdir}/small-${String(n).padStart(3, "0")}.jpg`);
    }
  }
})();
"""
    run(["node", "-e", script, corpus, small], env=env)
    for jpg in sorted(small.glob("*.jpg")):
        png = refs / "small" / (jpg.stem + ".png")
        png.parent.mkdir(exist_ok=True)
        run([QM, "topng", jpg, png])
        run([QM, "ref", png, *SMALL, refs / "small" / (jpg.stem + ".ref")])
    print(f"corpus in {corpus}", file=sys.stderr)


def prepare_large(corpus, refs):
    source_dir = os.environ.get("COMPARE_LARGE_CORPUS")
    if not source_dir:
        sys.exit("set COMPARE_LARGE_CORPUS to the directory containing the verified large photo")
    is_beach = IMAGE_SET.startswith("beach")
    filename = "ferrisma-beach-volleyball-53458847235.jpg" if is_beach else f"ferrisma-{IMAGE_SET}.jpg"
    image_id = "beach-volleyball-sunset" if is_beach else IMAGE_SET
    manifest = "corpus-large.json" if is_beach else "corpus-photos.json"
    source = Path(source_dir) / filename
    verify_corpus_file(source, image_id, manifest)
    run(["cargo", "build", "--release", "--features", "cli", "--bin", "ferrisma"], capture=False)
    run(["cargo", "build", "--release", "--manifest-path", QM_DIR / "Cargo.toml"], capture=False)
    input_file = corpus / IMAGES[0][1]
    input_file.write_bytes(source.read_bytes())
    verify_corpus_file(input_file, image_id, manifest)
    name, _, _, icc, w, h, orientation = IMAGES[0]
    crop = LARGE_CROPS.get(name, "")
    # These sources have sRGB profiles or, for Elcito, an explicit sRGB
    # assumption. Ignoring arbitrary embedded profiles would not be valid.
    script = ("const [i,o,c]=process.argv.slice(1);let p=require('sharp')(i,{ignoreIcc:true}).autoOrient();"
              "if(c){const [left,top,width,height]=c.split(',').map(Number);p=p.extract({left,top,width,height});}"
              "p.toColourspace('srgb').png({compressionLevel:1}).toFile(o)")
    for label, src in (("", input_file), ("-zune", refs / f"{name}-zune-source.png")):
        if label:
            run([QM, "topng", f"{input_file}#{orientation}", src])
        decoded = refs / f"{name}{label}-src.png"
        run(["node", "-e", script, src, decoded, crop], env=node_env())
        run([QM, "ref", decoded, w, h, refs / f"{name}{label}.ref"])


def score(ref, files):
    rows = run([QM, "score", ref, *files]).strip().splitlines()
    scored = [(Path(p).name, int(b), float(s)) for p, b, s in (r.split("\t") for r in rows)]
    if len(scored) != len(files) or any(not math.isfinite(s) for _, _, s in scored):
        raise ValueError("scorer returned missing or non-finite scores")
    return scored


def ceiling(out):
    """Resize-only and near-lossless scores: what each pipeline can reach."""
    corpus, refs, work = out / "corpus", out / "refs", out / "ceiling"
    work.mkdir(parents=True, exist_ok=True)
    env = node_env()
    lines = ["image\tcandidate\tbytes\tscore_vs_ref\tscore_vs_zune_ref"]
    for image in IMAGES:
        name, file, flag, icc, w, h, _ = image
        files = []
        for space in ("linear", "srgb", "gamma22"):
            f = work / f"{name}-sharp-{space}.png"
            run(["node", SHARP_AVIF, corpus / file, f, w, h, f"space={space}", f"icc={icc}"], env=env)
            files.append(f)
        f = work / f"{name}-sharp-q100.avif"
        run(sharp_cmd(corpus / file, f, w, h, icc, 100, 0), env=env)
        files.append(f)
        f = work / f"{name}-ferrisma-q100.avif"
        run(ferrisma_cmd(corpus / file, f, w, h, flag, 100, 0))
        files.append(f)
        main = score(refs / f"{name}.ref", [refs / f"{name}-zune.ref", *files])
        zune = score(refs / f"{name}-zune.ref", files)
        lines.append(f"{name}\tzune-ref vs ref\t-\t{main[0][2]:.3f}\t-")
        for (n, b, s), (_, _, z) in zip(main[1:], zune):
            lines.append(f"{name}\t{n}\t{b}\t{s:.3f}\t{z:.3f}")
    (out / "ceiling.tsv").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


def match_fingerprint(out):
    corpus, refs = out / "corpus", out / "refs"
    if not IMAGE_SET.startswith("beach"):
        for name in (image[0] for image in IMAGES if image[0] in ("dog", "flower")):
            verify_corpus_file(corpus / f"{name}.jpg", name)
    else:
        verify_corpus_file(corpus / "beach.jpg", "beach-volleyball-sunset", "corpus-large.json")
    if IMAGE_SET in ("mountain", "deer", "elcito"):
        verify_corpus_file(corpus / f"{IMAGE_SET}.jpg", IMAGE_SET, "corpus-photos.json")
    return {
        "image_set": IMAGE_SET, "inputs": {image[0]: sha256(corpus / image[1]) for image in IMAGES},
        "references": {image[0]: sha256(refs / f"{image[0]}.ref") for image in IMAGES},
        "ferrisma_sha256": sha256(FERRISMA), "quality_match_sha256": sha256(QM),
        "sharp_script_sha256": sha256(SHARP_AVIF), "harness_sha256": sha256(Path(__file__)),
        "sharp_versions": json.loads(run(["node", "-e", "console.log(JSON.stringify(require('sharp').versions))"], env=node_env())),
    }


def match(out):
    corpus, refs, work = out / "corpus", out / "refs", out / "match"
    work.mkdir(parents=True, exist_ok=True)
    env = node_env()
    provenance_path = out / "match-provenance.json"
    provenance_path.write_text(json.dumps({"status": "incomplete"}) + "\n")
    fingerprint = match_fingerprint(out)
    rows = ["image\ttarget\tconfig\tstatus\ttarget_score\tq_below\tscore_below\tbytes_below\tq_above\tscore_above"
            "\tbytes_above\tbytes_at_target\tq_nearest\tscore_nearest\tencodes"]
    anchors = ["image\tanchor\tbytes\tscore"]
    for image in IMAGES:
        name, _, _, _, _, _, _ = image
        ref = refs / f"{name}.ref"
        # A: Ferrisma's benchmark setting (quality 80, speed 6, all cores).
        # B: Sharp's defaults (quality 50, effort 4, all cores).
        a = work / f"{name}-anchor-ferrisma-q80.avif"
        run(config_cmd("ferrisma", 0, "444", image, corpus, 80, a))
        b = work / f"{name}-anchor-sharp-q50.avif"
        run(config_cmd("sharp", 0, "444", image, corpus, 50, b), env=env)
        (_, a_bytes, a_score), (_, b_bytes, b_score) = score(ref, [a, b])
        anchors += [f"{name}\tferrisma-q80\t{a_bytes}\t{a_score:.3f}", f"{name}\tsharp-q50\t{b_bytes}\t{b_score:.3f}"]
        for target_name, target in (("A", a_score), ("B", b_score))[:len(TARGETS)]:
            for tool, label, threads, chroma in CONFIGS:
                print(f"{name} target {target_name} {target:.3f} {label}", file=sys.stderr)
                cmd = config_cmd(tool, threads, chroma, image, corpus, "{q}", "{out}")
                step = "1" if tool == "sharp" else "0"
                result = run([QM, "bisect", ref, f"{target:.3f}", *QUALITY_RANGE, step,
                              work / f"{name}-{target_name}-{label}", "--", *cmd], env=env).strip()
                rows.append(f"{name}\t{target_name}\t{label}\t{result}")
                (out / "matched.tsv").write_text("\n".join(rows) + "\n")
    (out / "anchors.tsv").write_text("\n".join(anchors) + "\n")
    if fingerprint != match_fingerprint(out):
        raise ValueError("matching inputs or tools changed during the run")
    provenance_path.write_text(json.dumps({"status": "complete", "fingerprint": fingerprint,
                                           "matched_sha256": sha256(out / "matched.tsv")}, indent=2) + "\n")
    print("\n".join(anchors + rows))


def read_tsv(path):
    with open(path) as f:
        return list(csv.DictReader(f, delimiter="\t"))


def hyperfine(cmd, env, runs, json_path):
    run(["hyperfine", "--warmup", "1", "--runs", runs, "--shell=none", "--export-json", json_path,
         shlex.join(map(str, cmd))], env=env)
    result = json.loads(Path(json_path).read_text())["results"][0]
    if "measurements" in result:  # hyperfine 2 schema
        walls = [m["time_wall_clock"]["value"] for m in result["measurements"]]
        cpu = statistics.mean(m["time_cpu"]["value"] for m in result["measurements"])
    else:
        walls = result["times"]
        cpu = result["user"] + result["system"]
    if len(walls) != int(runs) or any(not math.isfinite(w) or w <= 0 for w in walls):
        raise ValueError("invalid or missing Hyperfine wall-time samples")
    if not math.isfinite(cpu) or cpu < 0:
        raise ValueError("invalid Hyperfine CPU time")
    return {"mean_s": statistics.mean(walls), "median_s": statistics.median(walls),
            "p95_s": sorted(walls)[math.ceil(0.95 * len(walls)) - 1],
            "stddev_s": statistics.stdev(walls) if len(walls) > 1 else 0.0,
            "min_s": min(walls), "cpu_s": cpu}


def peak_rss_kib(cmd, env):
    merged = dict(os.environ, **env)
    if sys.platform == "darwin":
        r = subprocess.run(["/usr/bin/time", "-l", *map(str, cmd)], env=merged, check=True,
                           stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
        line = next(l for l in r.stderr.splitlines() if "maximum resident set size" in l)
        return int(line.split()[0]) // 1024
    r = subprocess.run(["/usr/bin/time", "-f", "%M", *map(str, cmd)], env=merged, check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.PIPE, text=True)
    return int(r.stderr.strip().splitlines()[-1])


def in_process_ms(tool, cmd, env, runs):
    """Pipeline time without process and runtime start-up."""
    if tool == "sharp":
        out = run([*cmd, f"repeat={runs}"], env=env)
        return float(out.split("median_ms=")[1].split()[0])
    totals = []
    for _ in range(int(runs)):
        out = run(cmd)
        report = next(l for l in out.splitlines() if l.startswith("report:"))
        totals.append(int(report.split("total_us=")[1].split()[0]) / 1000)
    return statistics.median(totals)


def validate_matched(matched):
    for image in IMAGES:
        for _, label, _, _ in CONFIGS:
            key = (image[0], "A", label)
            if key not in matched:
                raise ValueError(f"missing quality match: {key}")
            row = matched[key]
            if row["status"] != "ok":
                raise ValueError(f"quality target not bracketed for {key}: {row['status']}")
            for field in ("target_score", "score_below", "score_above", "score_nearest"):
                try:
                    number = float(row[field])
                except (ValueError, KeyError) as error:
                    raise ValueError(f"invalid {field} for {key}") from error
                if not math.isfinite(number):
                    raise ValueError(f"non-finite {field} for {key}")


def validate_output_score(row, actual):
    target, nearest = float(row["target_score"]), float(row["score_nearest"])
    if not all(math.isfinite(value) for value in (actual, target, nearest)):
        raise ValueError("non-finite output or matching score")
    if abs(actual - nearest) > 0.1:
        raise ValueError("timed output score differs from matching pass")
    delta = actual - target
    if abs(delta) > 0.5:
        raise ValueError("timed output is more than 0.5 SSIMULACRA2 from the target")
    return delta


def write_timing_provenance(path, provenance, csv_path, expected, actual):
    if actual != expected:
        raise ValueError("timing did not complete every expected lane")
    provenance.update(status="complete", expected_lanes=expected, actual_lanes=actual,
                      csv_sha256=sha256(csv_path))
    path.write_text(json.dumps(provenance, indent=2) + "\n")


def require_current_match(out, matching):
    if (matching.get("status") != "complete" or matching.get("fingerprint") != match_fingerprint(out)
            or matching.get("matched_sha256") != sha256(out / "matched.tsv")):
        raise ValueError("quality matching evidence is incomplete or no longer matches the inputs and tools")


def timing(out):
    corpus, work = out / "corpus", out / "time"
    work.mkdir(parents=True, exist_ok=True)
    provenance_path = out / "time-provenance.json"
    provenance_path.write_text(json.dumps({"status": "incomplete"}) + "\n")
    runs = os.environ.get("COMPARE_RUNS", "10")
    matching = json.loads((out / "match-provenance.json").read_text())
    require_current_match(out, matching)
    matched = {(r["image"], r["target"], r["config"]): r for r in read_tsv(out / "matched.tsv")}
    validate_matched(matched)
    fields = ["image", "target", "config", "quality", "target_score", "score", "score_delta", "bytes", "input_sha256", "reference_sha256",
              "output_sha256", "command_json", "load_before", "load_after", "runs", "mean_s", "median_s", "p95_s",
              "stddev_s", "min_s", "cpu_s", "cpu_per_wall", "in_process_ms", "peak_rss_kib"]
    provenance = {
        "status": "incomplete",
        "repository_commit": run(["git", "rev-parse", "HEAD"]).strip(),
        "repository_status": run(["git", "status", "--short"]).strip(),
        "platform": platform.platform(), "machine": platform.machine(),
        "rustc": run(["rustc", "--version"]).strip(),
        "node": run(["node", "--version"]).strip(),
        "hyperfine": run(["hyperfine", "--version"]).strip(),
        "sharp_versions": json.loads(run(["node", "-e", "console.log(JSON.stringify(require('sharp').versions))"], env=node_env())),
        "ferrisma_sha256": sha256(FERRISMA), "quality_match_sha256": sha256(QM),
        "warmup_runs": 1, "runs": int(runs), "order": "fixed, as listed in CONFIGS",
        "peak_rss": "separate process run", "p95": "nearest rank",
    }
    provenance_path.write_text(json.dumps(provenance, indent=2) + "\n")
    path = out / "time.csv"
    expected = len(IMAGES) * len(CONFIGS)
    actual = 0
    with open(path, "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        for image in IMAGES:
            name = image[0]
            for tool, label, threads, chroma in CONFIGS:
                m = matched[(name, "A", label)]
                quality = m["q_nearest"]
                avif = work / f"{name}-{label}.avif"
                env = node_env({"UV_THREADPOOL_SIZE": "1"} if tool == "sharp" and threads == 1 else None)
                cmd = config_cmd(tool, threads, chroma, image, corpus, quality, avif)
                before = wait_for_idle()
                stats = hyperfine(cmd, env, runs, work / f"{name}-{label}.json")
                rss = peak_rss_kib(cmd, env)
                inproc = in_process_ms(tool, cmd, env, runs)
                after = load1()
                ref = out / "refs" / f"{name}.ref"
                (_, output_bytes, actual_score), = score(ref, [avif])
                delta = validate_output_score(m, actual_score)
                row = {"image": name, "target": "A", "config": label, "quality": quality,
                       "target_score": m["target_score"], "score": actual_score, "score_delta": delta, "bytes": output_bytes,
                       "input_sha256": sha256(corpus / image[1]), "reference_sha256": sha256(ref),
                       "output_sha256": sha256(avif), "command_json": json.dumps([str(c) for c in cmd]),
                       "load_before": f"{before:.2f}", "load_after": f"{after:.2f}", "runs": runs,
                       **{k: f"{v:.4f}" for k, v in stats.items()},
                       "cpu_per_wall": f"{stats['cpu_s'] / stats['mean_s']:.2f}",
                       "in_process_ms": f"{inproc:.1f}", "peak_rss_kib": rss}
                writer.writerow(row)
                actual += 1
                f.flush()
                print(row, file=sys.stderr)
    require_current_match(out, matching)
    write_timing_provenance(provenance_path, provenance, path, expected, actual)


def small_scores(refs, outputs):
    scores = []
    for avif in sorted(outputs.glob("*.avif")):
        (_, b, s), = score(refs / "small" / (avif.stem + ".ref"), [avif])
        scores.append((b, s))
    return scores


def batch(out):
    corpus, refs, work = out / "corpus", out / "refs", out / "batch"
    small = corpus / "small"
    work.mkdir(parents=True, exist_ok=True)
    w, h = SMALL
    env = node_env()
    lines = []
    # Target: Ferrisma's batch setting (quality 80, one thread per image, as
    # Engine::transcode_batch runs it), mean score over the set.
    fdir = work / "ferrisma-q80"
    fdir.mkdir(exist_ok=True)
    for jpg in sorted(small.glob("*.jpg")):
        run(ferrisma_cmd(jpg, fdir / (jpg.stem + ".avif"), w, h, "--assume-srgb", 80, 1))
    fs = small_scores(refs, fdir)
    target = statistics.mean(s for _, s in fs)
    lines.append(f"ferrisma q80: images {len(fs)} mean score {target:.3f} total bytes {sum(b for b, _ in fs)}")
    # Bisect Sharp's quality on the mean score (one thread per image, 20 in
    # flight; the defaults are scored at the matched quality too).
    def sharp_mean(quality, parallel=20, threads=1):
        d = work / f"sharp-q{quality}-p{parallel}-t{threads}"
        run(["node", SHARP_BATCH, small, w, h, f"quality={quality}", f"parallel={parallel}",
             f"threads={threads}", f"out={d}"], env=env)
        s = small_scores(refs, d)
        return statistics.mean(x for _, x in s), sum(b for b, _ in s)
    lo, hi = QUALITY_RANGE
    points = {}
    for q in (lo, hi):
        points[q] = sharp_mean(q)
    while hi - lo > 1:
        mid = (lo + hi) // 2
        points[mid] = sharp_mean(mid)
        if points[mid][0] >= target:
            hi = mid
        else:
            lo = mid
    (s_lo, b_lo), (s_hi, b_hi) = points[lo], points[hi]
    t = (target - s_lo) / (s_hi - s_lo)
    lines.append(f"sharp bracket q{lo} {s_lo:.3f} {b_lo} B, q{hi} {s_hi:.3f} {b_hi} B; "
                 f"interpolated bytes at target {b_lo + t * (b_hi - b_lo):.0f}")
    s_def, b_def = sharp_mean(hi, parallel=4, threads=0)
    lines.append(f"sharp q{hi} with defaults (4 in flight, all threads): mean score {s_def:.3f} total bytes {b_def}")
    quality = hi
    # Throughput, three runs each, after the load check.
    runs = 3
    for label, cmd, nenv in [
        ("ferrisma Engine (20 workers) and sequential", [BATCH, small, w, 0, h], {}),
        ("sharp defaults: 4 in flight, all threads", ["node", SHARP_BATCH, small, w, h, f"quality={quality}"], env),
        ("sharp 20 in flight, 1 thread each", ["node", SHARP_BATCH, small, w, h, f"quality={quality}",
                                                "parallel=20", "threads=1"], env),
        ("sharp 10 in flight, 2 threads each", ["node", SHARP_BATCH, small, w, h, f"quality={quality}",
                                                 "parallel=10", "threads=2"], env),
        ("sharp 1 in flight, 1 thread (sequential)", ["node", SHARP_BATCH, small, w, h, f"quality={quality}",
                                                       "parallel=1", "threads=1"], env),
    ]:
        for i in range(runs):
            before = wait_for_idle()
            started = time.monotonic()
            text = run(cmd, env=nenv).strip().replace("\n", " | ")
            rss = peak_rss_kib(cmd, nenv)
            lines.append(f"{label} run {i + 1}: load {before:.2f}->{load1():.2f}, wall {time.monotonic() - started:.2f} s, "
                         f"peak RSS {rss / 1024:.1f} MiB (separate run): {text}")
            print(lines[-1], file=sys.stderr)
    (out / "batch.txt").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


def stages(out):
    """Ferrisma's own stage report at the matched setting (target A)."""
    corpus, work = out / "corpus", out / "stages"
    work.mkdir(parents=True, exist_ok=True)
    runs = int(os.environ.get("COMPARE_RUNS", "10"))
    matched = {(r["image"], r["config"]): r for r in read_tsv(out / "matched.tsv") if r["target"] == "A"}
    keys = ["open_us", "decode_us", "transform_us", "encode_us", "total_us"]
    lines = ["image\tconfig\tload_before\tload_after\t" + "\t".join(k.replace("_us", "_ms") for k in keys)]
    for image in IMAGES:
        for tool, label, threads, chroma in CONFIGS:
            if tool != "ferrisma":
                continue
            cmd = config_cmd(tool, threads, chroma, image, corpus, matched[(image[0], label)]["q_nearest"],
                             work / f"{image[0]}-{label}.avif")
            before = wait_for_idle()
            samples = {k: [] for k in keys}
            for _ in range(runs):
                report = next(l for l in run(cmd).splitlines() if l.startswith("report:"))
                fields = dict(kv.split("=") for kv in report.split()[1:])
                for k in keys:
                    samples[k].append(int(fields[k]) / 1000)
            lines.append(f"{image[0]}\t{label}\t{before:.2f}\t{load1():.2f}\t"
                         + "\t".join(f"{statistics.median(samples[k]):.1f}" for k in keys))
            print(lines[-1], file=sys.stderr)
    (out / "stages.tsv").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


# Encoder presets around each tool's default (rav1e speed 6, aom effort 4),
# all cores, 4:4:4, matched to target A: the bytes-versus-time trade-off.
SWEEP_IMAGES = ("flower", "kodim23")
SWEEP = [("ferrisma", 4), ("ferrisma", 8), ("ferrisma", 10), ("sharp", 0), ("sharp", 2), ("sharp", 6)]


def sweep_cmd(image, corpus, tool, preset, quality, path):
    name, file, flag, icc, w, h, _ = image
    if tool == "ferrisma":
        return ferrisma_cmd(corpus / file, path, w, h, flag, quality, 0, speed=preset)
    return sharp_cmd(corpus / file, path, w, h, icc, quality, 0, extra=(f"effort={preset}",))


def sweep_match(out):
    """Bytes and scores of the preset sweep (no timing; safe under load)."""
    corpus, refs, work = out / "corpus", out / "refs", out / "sweep"
    work.mkdir(parents=True, exist_ok=True)
    targets = {r["image"]: float(r["score"]) for r in read_tsv(out / "anchors.tsv") if r["anchor"] == "ferrisma-q80"}
    rows = ["image\ttool\tpreset\tstatus\ttarget_score\tq_below\tscore_below\tbytes_below\tq_above\tscore_above"
            "\tbytes_above\tbytes_at_target\tq_nearest\tscore_nearest\tencodes"]
    env = node_env()
    for image in (i for i in IMAGES if i[0] in SWEEP_IMAGES):
        name = image[0]
        for tool, preset in SWEEP:
            print(f"{name} {tool} preset {preset}", file=sys.stderr)
            step = "1" if tool == "sharp" else "0"
            result = run([QM, "bisect", refs / f"{name}.ref", f"{targets[name]:.3f}", *QUALITY_RANGE, step,
                          work / f"{name}-{tool}-{preset}", "--",
                          *sweep_cmd(image, corpus, tool, preset, "{q}", "{out}")], env=env).strip()
            rows.append(f"{name}\t{tool}\t{preset}\t{result}")
            (out / "sweep-matched.tsv.partial").write_text("\n".join(rows) + "\n")
    (out / "sweep-matched.tsv.partial").replace(out / "sweep-matched.tsv")
    print("\n".join(rows))


def sweep(out):
    """Load-gated timing of the preset sweep at its matched settings."""
    corpus, work = out / "corpus", out / "sweep"
    if not (out / "sweep-matched.tsv").exists():
        sweep_match(out)
    runs = os.environ.get("COMPARE_SWEEP_RUNS", "5")
    images = {i[0]: i for i in IMAGES}
    fields = ["image", "tool", "preset", "status", "target_score", "bytes_at_target", "q_nearest", "score_nearest",
              "load_before", "load_after", "runs", "median_s", "stddev_s", "cpu_per_wall", "in_process_ms"]
    env = node_env()
    with open(out / "sweep.csv", "w", newline="") as f:
        writer = csv.DictWriter(f, fieldnames=fields)
        writer.writeheader()
        for m in read_tsv(out / "sweep-matched.tsv"):
            image, tool, preset = images[m["image"]], m["tool"], m["preset"]
            timed = sweep_cmd(image, corpus, tool, preset, m["q_nearest"], work / f"{m['image']}-{tool}-{preset}.avif")
            before = wait_for_idle()
            stats = hyperfine(timed, env, runs, work / f"{m['image']}-{tool}-{preset}.json")
            inproc = in_process_ms(tool, timed, env, runs)
            row = {k: m[k] for k in ("image", "tool", "preset", "status", "target_score", "bytes_at_target",
                                     "q_nearest", "score_nearest")}
            row.update({"load_before": f"{before:.2f}", "load_after": f"{load1():.2f}", "runs": runs,
                        "median_s": f"{stats['median_s']:.4f}", "stddev_s": f"{stats['stddev_s']:.4f}",
                        "cpu_per_wall": f"{stats['cpu_s'] / stats['mean_s']:.2f}", "in_process_ms": f"{inproc:.1f}"})
            writer.writerow(row)
            f.flush()
            print(row, file=sys.stderr)


# The encoder comparison intentionally uses the same prepared references as
# the Sharp harness, but matches each thread configuration independently.
def encoder_levels():
    raw = os.environ.get("COMPARE_ENCODER_LEVELS", "80")
    try:
        values = [int(value) for value in raw.split(",")]
    except ValueError as error:
        raise ValueError("COMPARE_ENCODER_LEVELS must contain integer qualities") from error
    if not values or len(set(values)) != len(values) or any(v < 1 or v > 100 for v in values):
        raise ValueError("encoder anchor qualities must be distinct integers from 1 to 100")
    return values


def encoder_cmd(image, corpus, binary, quality, speed, threads, output):
    name, file, flag, _, w, h, _ = image
    cmd = [binary, corpus / file, output, w, h, flag, f"--quality={quality}",
           f"--speed={speed}", f"--encoder-threads={threads}",
           f"--transform-threads={threads}", "--chroma=444"]
    if binary == FERRISMA:
        # The "rav1e" lane; the libavif lane runs the separate adapter binary.
        cmd.append("--avif-encoder=rav1e")
    if name in LARGE_CROPS:
        cmd.append(f"--crop={LARGE_CROPS[name]}")
    if name.startswith("beach") or name in ("mountain", "deer", "elcito"):
        cmd.append("--budget-mb=2048")
    return cmd


def tracked_source_diff():
    return subprocess.run(["git", "-C", str(REPO), "diff", "--binary", "HEAD"],
                          check=True, stdout=subprocess.PIPE).stdout


def encoder_source_receipt():
    if REPO != BENCH.parents[1]:
        raise ValueError("encoder comparison requires COMPARE_REPO to be the harness checkout")
    return {
        "encoder_script_sha256": sha256(BENCH / "compare-encoders.sh"),
        "harness_sha256": sha256(Path(__file__)),
        "lockfile_sha256": sha256(REPO / "Cargo.lock"),
        "scorer_sources_sha256": {name: sha256(QM_DIR / name)
                                  for name in ("Cargo.toml", "Cargo.lock", "src/main.rs")},
        "corpus_manifests_sha256": {name: sha256(BENCH / name) for name in ("corpus.json", "corpus-large.json", "corpus-photos.json")},
        "repository_commit": run(["git", "-C", REPO, "rev-parse", "HEAD"]).strip(),
        "rustc": run(["rustc", "--version", "--verbose"]).strip(),
        "cargo": run(["cargo", "--version"]).strip(),
        "build_env": {key: os.environ.get(key) for key in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "CARGO_TARGET_DIR")},
        "tracked_status": run(["git", "-C", REPO, "status", "--short", "--untracked-files=no"]).strip(),
        "tracked_diff_sha256": hashlib.sha256(tracked_source_diff()).hexdigest(),
        "levels": encoder_levels(),
    }


def encoder_build():
    """Build all measured executables from one unchanged source receipt."""
    before = encoder_source_receipt()
    commands = [
        ("ferrisma", FERRISMA, ["cargo", "build", "--locked", "--release", "-p", "ferrisma", "--features", "cli", "--bin", "ferrisma"]),
        ("ferrisma-libavif", LIBAVIF_ENCODER, ["cargo", "build", "--locked", "--release", "-p", "ferrisma-libavif", "--features", "cli", "--bin", "ferrisma-libavif"]),
        ("ferrisma-quality-match", QM, ["cargo", "build", "--locked", "--release", "--manifest-path", QM_DIR / "Cargo.toml"]),
    ]
    for name, expected, command in commands:
        messages = run([*command, "--message-format=json-render-diagnostics"]).splitlines()
        built = [Path(message["executable"]).resolve() for line in messages
                 for message in [json.loads(line)]
                 if message.get("reason") == "compiler-artifact"
                 and message.get("target", {}).get("name") == name
                 and message.get("executable")]
        if built != [expected.resolve()]:
            raise ValueError(f"Cargo built {name} at {built}, expected {[expected.resolve()]}; check target configuration")
    if before != encoder_source_receipt():
        raise ValueError("encoder or scorer source changed while building")
    return before


def encoder_fingerprint(out):
    # Reuse the existing source hash checks, references and tool provenance.
    source = encoder_source_receipt()
    base = match_fingerprint(out)
    base.update(source)
    base["libavif_sha256"] = sha256(LIBAVIF_ENCODER)
    return base


def encoder_match(out):
    work = out / "encoders"
    work.mkdir(parents=True, exist_ok=True)
    provenance = work / "match-provenance.json"
    if provenance.exists() or (work / "matched.json").exists():
        raise ValueError("encoder matching output already exists; use a fresh output directory")
    provenance.write_text('{"status":"incomplete"}\n')
    built_from = encoder_build()
    fingerprint = encoder_fingerprint(out)
    if any(fingerprint.get(key) != value for key, value in built_from.items()):
        raise ValueError("encoder or scorer source changed after building")
    (work / "source.patch").write_bytes(tracked_source_diff())
    if sha256(work / "source.patch") != fingerprint["tracked_diff_sha256"]:
        raise ValueError("tracked source changed while recording provenance")
    records = []
    for image in IMAGES:
        name = image[0]
        ref = out / "refs" / f"{name}.ref"
        for threads in (0, 1):
            for level in encoder_levels():
                anchor = work / f"{name}-q{level}-t{threads}-rav1e.avif"
                anchor_cmd = encoder_cmd(image, out / "corpus", FERRISMA, level, 6, threads, anchor)
                run(anchor_cmd)
                (_, size, target), = score(ref, [anchor])
                for speed in (6, 7):
                    path = work / f"{name}-q{level}-t{threads}-aom-s{speed}"
                    cmd = encoder_cmd(image, out / "corpus", LIBAVIF_ENCODER, "{q}", speed, threads, "{out}")
                    columns = run([QM, "bisect", ref, f"{target:.3f}", "0", "100", "1", path, "--", *cmd]).strip().split("\t")
                    names = ("status", "target_score", "q_below", "score_below", "bytes_below", "q_above",
                             "score_above", "bytes_above", "bytes_at_target", "q_nearest", "score_nearest", "encodes")
                    if len(columns) != len(names):
                        raise ValueError("invalid encoder bisection result")
                    result = dict(zip(names, columns))
                    records.append({"image": name, "anchor_quality": level, "threads": threads,
                                    "speed": speed, "anchor_bytes": size, "anchor_score": target,
                                    "anchor_sha256": sha256(anchor), **result})
                    (work / "matched.json").write_text(json.dumps(records, indent=2) + "\n")
    if fingerprint != encoder_fingerprint(out):
        raise ValueError("matching inputs or tools changed during the run")
    files = {str(path.relative_to(work)): sha256(path) for path in sorted(work.glob("*-rav1e.avif"))}
    files.update({str(path.relative_to(work)): sha256(path) for path in sorted(work.glob("*-aom-s*/q*.avif"))})
    files["source.patch"] = sha256(work / "source.patch")
    provenance.write_text(json.dumps({"status": "complete", "fingerprint": fingerprint,
                                      "matched_sha256": sha256(work / "matched.json"),
                                      "match_aom_simd_caps_mask": os.environ.get("AOM_SIMD_CAPS_MASK"),
                                      "outputs_sha256": files}, indent=2) + "\n")


class EncoderQualityMismatch(ValueError):
    """Valid matching evidence cannot meet the declared quality tolerance."""


def validate_encoder_match(out, matching):
    work = out / "encoders"
    if (matching.get("status") != "complete" or matching.get("fingerprint") != encoder_fingerprint(out)
            or matching.get("matched_sha256") != sha256(work / "matched.json")):
        raise ValueError("encoder matching evidence is incomplete or stale")
    if not matching.get("outputs_sha256") or any(
            not (work / name).is_file() or sha256(work / name) != digest
            for name, digest in matching["outputs_sha256"].items()):
        raise ValueError("matched encoder outputs are missing or changed")
    records = json.loads((work / "matched.json").read_text())
    indexed = {}
    for row in records:
        key = (row["image"], row["anchor_quality"], row["threads"], row["speed"])
        if key in indexed:
            raise ValueError(f"duplicate quality match: {key}")
        if row["status"] in ("below-range", "unreachable"):
            raise EncoderQualityMismatch(f"quality target not bracketed for {key}: {row['status']}")
        if row["status"] != "ok":
            raise ValueError(f"unknown quality match status for {key}: {row['status']}")
        for field in ("target_score", "score_below", "score_above", "score_nearest"):
            if not math.isfinite(float(row[field])):
                raise ValueError(f"invalid quality score for {key}")
        if not float(row["score_below"]) < float(row["target_score"]) <= float(row["score_above"]):
            raise ValueError(f"invalid quality bracket for {key}")
        delta = float(row["score_nearest"]) - float(row["target_score"])
        if abs(delta) > 0.5:
            raise EncoderQualityMismatch(f"nearest integer quality is more than 0.5 SSIMULACRA2 from target for {key}: delta={delta:+.3f}")
        indexed[key] = row
    expected = {(i[0], q, t, s) for i in IMAGES for q in encoder_levels() for t in (0, 1) for s in (6, 7)}
    if set(indexed) != expected:
        raise ValueError("encoder quality matches do not cover the requested matrix")
    return indexed


def encoder_check(out):
    """Check every matched cell without starting a timing run."""
    matching = json.loads((out / "encoders" / "match-provenance.json").read_text())
    try:
        validate_encoder_match(out, matching)
    except EncoderQualityMismatch as error:
        print(error, file=sys.stderr)
        raise SystemExit(3) from error


def encoder_sample(cmd):
    """One process wall-time and peak-RSS observation, including startup and I/O."""
    if sys.platform == "darwin":
        timed = ["/usr/bin/time", "-l", *map(str, cmd)]
    else:
        timed = ["/usr/bin/time", "-f", "FERRISMA_RSS_KIB=%M", *map(str, cmd)]
    start = time.monotonic()
    result = subprocess.run(timed, check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    wall = time.monotonic() - start
    if sys.platform == "darwin":
        rss = int(next(line.split()[0] for line in result.stderr.splitlines()
                       if "maximum resident set size" in line)) // 1024
    else:
        rss = int(next(line.split("=", 1)[1] for line in result.stderr.splitlines()
                       if line.startswith("FERRISMA_RSS_KIB=")))
    report = next(line for line in result.stdout.splitlines() if line.startswith("report:"))
    fields = dict(field.split("=", 1) for field in report.split()[1:])
    if wall <= 0 or rss <= 0:
        raise ValueError("invalid timing or RSS sample")
    return wall, rss, fields


def encoder_wait_for_idle():
    limit = float(os.environ.get("COMPARE_MAX_LOAD", "4"))
    timeout = int(os.environ.get("COMPARE_IDLE_TIMEOUT", "900"))
    if not math.isfinite(limit) or limit <= 0 or not 1 <= timeout <= 7200:
        raise ValueError("invalid encoder load limit or idle timeout")
    deadline = time.monotonic() + timeout
    while load1() >= limit:
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise TimeoutError("encoder timing could not obtain an idle window")
        time.sleep(min(30, remaining))


def encoder_hardware():
    model = platform.processor()
    if sys.platform.startswith("linux"):
        for line in Path("/proc/cpuinfo").read_text().splitlines():
            if line.startswith("model name"):
                model = line.split(":", 1)[1].strip()
                break
    elif sys.platform == "darwin":
        result = subprocess.run(["sysctl", "-n", "machdep.cpu.brand_string"], capture_output=True, text=True)
        if result.returncode == 0:
            model = result.stdout.strip()
    affinity = len(os.sched_getaffinity(0)) if hasattr(os, "sched_getaffinity") else None
    return {"model": model, "logical_cpus": os.cpu_count(), "affinity_cpus": affinity}


def encoder_timing(out):
    work = out / "encoders"
    run_id = os.environ.get("COMPARE_ENCODER_RUN_ID", "default")
    if not re.fullmatch(r"[A-Za-z0-9_-]+", run_id):
        raise ValueError("COMPARE_ENCODER_RUN_ID must contain only letters, numbers, _ or -")
    timedir = work / f"time-{run_id}"
    timedir.mkdir(parents=True, exist_ok=False)
    output = timedir / "provenance.json"
    output.write_text('{"status":"incomplete"}\n')
    matching = json.loads((work / "match-provenance.json").read_text())
    matches = validate_encoder_match(out, matching)
    runs = int(os.environ.get("COMPARE_RUNS", "5"))
    if not 1 <= runs <= 100:
        raise ValueError("COMPARE_RUNS must be between 1 and 100")
    samples_path = timedir / "samples.json"
    stamp = {"status": "incomplete", "fingerprint": matching["fingerprint"],
             "matched_sha256": sha256(work / "matched.json"), "platform": platform.platform(),
             "machine": platform.machine(), "rustc": run(["rustc", "--version", "--verbose"]).strip(),
             "cargo": run(["cargo", "--version"]).strip(), "node": run(["node", "--version"]).strip(),
             "hardware": encoder_hardware(),
             "binary_sizes": {"rav1e": FERRISMA.stat().st_size, "libavif": LIBAVIF_ENCODER.stat().st_size},
             "env": {key: os.environ.get(key) for key in ("RUSTFLAGS", "CARGO_ENCODED_RUSTFLAGS", "AOM_SIMD_CAPS_MASK", "CARGO_TARGET_DIR")},
             "match_aom_simd_caps_mask": matching.get("match_aom_simd_caps_mask"),
             "runs": runs, "warmups": 1, "order": "alternate candidate order per sample"}
    samples = []
    samples_path.write_text("[]\n")
    output.write_text(json.dumps(stamp, indent=2) + "\n")
    for image in IMAGES:
        name = image[0]
        for threads in (0, 1):
            for level in encoder_levels():
                lanes = [("rav1e", FERRISMA, level, 6, None)]
                for speed in (6, 7):
                    row = matches[(name, level, threads, speed)]
                    lanes.append((f"aom-s{speed}", LIBAVIF_ENCODER, row["q_nearest"], speed, row))
                for sample in range(-1, runs):
                    for label, binary, quality, speed, row in (lanes if sample % 2 == 0 else list(reversed(lanes))):
                        encoder_wait_for_idle()
                        avif = timedir / f"{name}-q{level}-t{threads}-{label}.avif"
                        cmd = encoder_cmd(image, out / "corpus", binary, quality, speed, threads, avif)
                        before = load1()
                        wall, rss, report = encoder_sample(cmd)
                        if sample < 0:
                            continue
                        (_, size, actual), = score(out / "refs" / f"{name}.ref", [avif])
                        if row:
                            validate_output_score(row, actual)
                        else:
                            anchor = matches[(name, level, threads, 6)]
                            if abs(actual - float(anchor["anchor_score"])) > 0.1:
                                raise ValueError("timed anchor differs from matching score")
                        samples.append({"image": name, "anchor_quality": level, "threads": threads,
                                        "lane": label, "quality": quality, "speed": speed,
                                        "run": sample + 1, "wall_s": wall, "peak_rss_kib": rss,
                                        "stage_report": report, "score": actual, "bytes": size,
                                        "output_sha256": sha256(avif), "load_before": before,
                                        "load_after": load1(), "output_file": avif.name,
                                        "command": list(map(str, cmd))})
                        samples_path.write_text(json.dumps(samples, indent=2) + "\n")
    validate_encoder_match(out, matching)
    expected = len(IMAGES) * len(encoder_levels()) * 2 * 3 * runs
    if len(samples) != expected:
        raise ValueError("encoder timing did not complete every lane")
    groups = {}
    for row in samples:
        key = (row["image"], row["anchor_quality"], row["threads"], row["lane"])
        groups.setdefault(key, []).append(row)
    summary = []
    for (name, quality, threads, lane), group in groups.items():
        walls = sorted(row["wall_s"] for row in group)
        summary.append({"image": name, "anchor_quality": quality, "threads": threads, "lane": lane,
                        "median_s": statistics.median(walls), "p95_s": walls[math.ceil(0.95 * len(walls)) - 1],
                        "min_s": walls[0], "max_s": walls[-1],
                        "peak_rss_kib_max": max(row["peak_rss_kib"] for row in group)})
    summary_path = timedir / "summary.json"
    summary_path.write_text(json.dumps(summary, indent=2) + "\n")
    stamp.update(status="complete", expected_samples=expected, actual_samples=len(samples),
                 samples_sha256=sha256(samples_path),
                 summary_sha256=sha256(summary_path),
                 final_outputs_sha256={path.name: sha256(path) for path in sorted(timedir.glob("*.avif"))})
    output.write_text(json.dumps(stamp, indent=2) + "\n")


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    phase, out = sys.argv[1], Path(sys.argv[2]).resolve()
    out.mkdir(parents=True, exist_ok=True)
    os.chdir(REPO)
    if IMAGE_SET != "default" and phase not in ("prepare", "match", "time", "encoder-match", "encoder-check", "encoder-time"):
        sys.exit("selected image sets support prepare, match and time only")
    phases = {"prepare": prepare, "ceiling": ceiling, "match": match, "time": timing, "batch": batch,
              "stages": stages, "sweepmatch": sweep_match, "sweep": sweep,
              "encoder-match": encoder_match, "encoder-check": encoder_check, "encoder-time": encoder_timing}
    legacy_all = ("prepare", "ceiling", "match", "time", "batch", "stages", "sweepmatch", "sweep")
    for name in (legacy_all if phase == "all" else [phase]):
        if name not in phases:
            sys.exit(__doc__)
        phases[name](out)


if __name__ == "__main__":
    main()
