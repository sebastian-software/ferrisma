#!/usr/bin/env python3
"""Quality-matched Ferrixel versus Sharp comparison (M2 work package 0).

Usage: python3 -I scripts/bench/compare-sharp.py PHASE OUT_DIR
PHASE is prepare, ceiling, match, time, batch, stages, sweepmatch, sweep or all. See
docs/benchmarks/M2-sharp-comparison.md for the method and the results.

Environment:
  BENCH_SHARP_NODE_MODULES  node_modules directory containing sharp (required)
  COMPARE_LIBAVIF_DATA      libavif tests/data (default: imported local fixtures)
  COMPARE_LIBJXL_DATA       libjxl decode-test (default: imported local fixtures)
  COMPARE_REPO              Ferrixel checkout to build and measure (default:
                            the checkout containing this script)
  COMPARE_RUNS              timed runs per measurement (default 10)
  COMPARE_MAX_LOAD          wait until the 1-minute load average is below
                            this before every timing run (default 4)

Only `time`, `batch`, `stages` and `sweep` measure time; `prepare`, `ceiling` and `match`
produce bytes and SSIMULACRA2 scores, which do not depend on machine load.
"""

import csv
import json
import os
import statistics
import subprocess
import sys
import time
from pathlib import Path

BENCH = Path(__file__).resolve().parent
# The Ferrixel checkout whose CLI is measured; defaults to this script's.
REPO = Path(os.environ.get("COMPARE_REPO", BENCH.parents[1])).resolve()
FERRIXEL = REPO / "target" / "release" / "ferrixel"
BATCH = REPO / "target" / "release" / "examples" / "batch"
QM_DIR = BENCH / "quality-match"
QM = QM_DIR / "target" / "release" / "ferrixel-quality-match"
SHARP_AVIF = BENCH / "sharp-avif.cjs"
SHARP_BATCH = BENCH / "sharp-batch.cjs"

# name, source file in the corpus, Ferrixel profile flag, Sharp ICC mode,
# output width, height, EXIF orientation (for the zune-jpeg reference).
IMAGES = [
    ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6),
    ("flower", "flower.jpg", "--assume-srgb", "convert", 1600, 1067, 1),
    ("kodim03", "kodim03.jpg", "--assume-srgb", "convert", 512, 341, 1),
    ("kodim23", "kodim23.jpg", "--assume-srgb", "convert", 512, 341, 1),
]
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
    if sys.platform == "darwin":
        return float(run(["sysctl", "-n", "vm.loadavg"]).split()[1])
    return float(Path("/proc/loadavg").read_text().split()[0])


def wait_for_idle():
    limit = float(os.environ.get("COMPARE_MAX_LOAD", "4"))
    while (load := load1()) >= limit:
        print(f"  load {load:.1f} >= {limit}, waiting 60 s", file=sys.stderr)
        time.sleep(60)
    return load


def ferrixel_cmd(src, out, w, h, flag, quality, threads, chroma="444"):
    return [FERRIXEL, src, out, w, h, flag, f"--quality={quality}",
            f"--encoder-threads={threads}", f"--transform-threads={threads}", f"--chroma={chroma}"]


def sharp_cmd(src, out, w, h, icc, quality, threads, chroma="444", extra=()):
    return ["node", SHARP_AVIF, src, out, w, h, f"quality={quality}", f"icc={icc}",
            f"threads={threads}", f"chroma={chroma}", *extra]


# Matched configurations: tool, label, threads (0 = all cores), chroma.
CONFIGS = [
    ("ferrixel", "ferrixel-all-444", 0, "444"),
    ("sharp", "sharp-all-444", 0, "444"),
    ("ferrixel", "ferrixel-one-444", 1, "444"),
    ("sharp", "sharp-one-444", 1, "444"),
    ("ferrixel", "ferrixel-all-420", 0, "420"),
    ("sharp", "sharp-all-420", 0, "420"),
]


def config_cmd(tool, threads, chroma, image, corpus, quality, out):
    name, file, flag, icc, w, h, _ = image
    if tool == "ferrixel":
        return ferrixel_cmd(corpus / file, out, w, h, flag, quality, threads, chroma)
    return sharp_cmd(corpus / file, out, w, h, icc, quality, threads, chroma)


def prepare(out):
    corpus = out / "corpus"
    refs = out / "refs"
    small = corpus / "small"
    for d in (corpus, refs, small):
        d.mkdir(parents=True, exist_ok=True)
    print("building ferrixel CLI, batch example and quality-match", file=sys.stderr)
    run(["cargo", "build", "--release", "--features", "cli", "--bin", "ferrixel", "--example", "batch"], capture=False)
    run(["cargo", "build", "--release", "--manifest-path", QM_DIR / "Cargo.toml"], capture=False)
    avif = Path(os.environ.get("COMPARE_LIBAVIF_DATA", REPO / "codecs/libavif-rs/upstream/libavif/tests/data"))
    jxl = Path(os.environ.get("COMPARE_LIBJXL_DATA", REPO / "codecs/libjxl-rs/tests/data/decode-test"))
    (corpus / "dog.jpg").write_bytes((avif / "dog_exif_extended_xmp_icc.jpg").read_bytes())
    (corpus / "flower.jpg").write_bytes((jxl / "flower.png.im_q85_420.jpg").read_bytes())
    env = node_env()
    for kodak in ("kodim03", "kodim23"):
        # y4m -> 8-bit PNG (BT.601 full range) -> JPEG q95 4:2:0 (mozjpeg):
        # the only lossy step before the pipelines; both read the same JPEG.
        run([QM, "topng", avif / f"{kodak}_yuv420_8bpc.y4m", corpus / f"{kodak}-y4m.png"])
        run(["node", "-e", 'require("sharp")(process.argv[1]).jpeg({quality:95,chromaSubsampling:"4:2:0"})'
             '.toFile(process.argv[2])', corpus / f"{kodak}-y4m.png", corpus / f"{kodak}.jpg"], env=env)
    for name, file, _, icc, w, h, orientation in IMAGES:
        # Main reference: libjpeg-turbo-family decode (Sharp's mozjpeg),
        # oriented, sRGB. Second reference: zune-jpeg (Ferrixel's decoder).
        src = refs / f"{name}-src.png"
        run(["node", "-e", 'const [i,o,icc]=process.argv.slice(1);require("sharp")(i,{ignoreIcc:icc==="ignore"})'
             '.autoOrient().toColourspace("srgb").png({compressionLevel:1}).toFile(o)', corpus / file, src, icc], env=env)
        run([QM, "ref", src, w, h, refs / f"{name}.ref"])
        run([QM, "ref", f"{corpus / file}#{orientation}", w, h, refs / f"{name}-zune.ref"])
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


def score(ref, files):
    rows = run([QM, "score", ref, *files]).strip().splitlines()
    return [(Path(p).name, int(b), float(s)) for p, b, s in (r.split("\t") for r in rows)]


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
        f = work / f"{name}-ferrixel-q100.avif"
        run(ferrixel_cmd(corpus / file, f, w, h, flag, 100, 0))
        files.append(f)
        main = score(refs / f"{name}.ref", [refs / f"{name}-zune.ref", *files])
        zune = score(refs / f"{name}-zune.ref", files)
        lines.append(f"{name}\tzune-ref vs ref\t-\t{main[0][2]:.3f}\t-")
        for (n, b, s), (_, _, z) in zip(main[1:], zune):
            lines.append(f"{name}\t{n}\t{b}\t{s:.3f}\t{z:.3f}")
    (out / "ceiling.tsv").write_text("\n".join(lines) + "\n")
    print("\n".join(lines))


def match(out):
    corpus, refs, work = out / "corpus", out / "refs", out / "match"
    work.mkdir(parents=True, exist_ok=True)
    env = node_env()
    rows = ["image\ttarget\tconfig\tstatus\ttarget_score\tq_below\tscore_below\tbytes_below\tq_above\tscore_above"
            "\tbytes_above\tbytes_at_target\tq_nearest\tscore_nearest\tencodes"]
    anchors = ["image\tanchor\tbytes\tscore"]
    for image in IMAGES:
        name, _, _, _, _, _, _ = image
        ref = refs / f"{name}.ref"
        # A: Ferrixel's benchmark setting (quality 80, speed 6, all cores).
        # B: Sharp's defaults (quality 50, effort 4, all cores).
        a = work / f"{name}-anchor-ferrixel-q80.avif"
        run(config_cmd("ferrixel", 0, "444", image, corpus, 80, a))
        b = work / f"{name}-anchor-sharp-q50.avif"
        run(config_cmd("sharp", 0, "444", image, corpus, 50, b), env=env)
        (_, a_bytes, a_score), (_, b_bytes, b_score) = score(ref, [a, b])
        anchors += [f"{name}\tferrixel-q80\t{a_bytes}\t{a_score:.3f}", f"{name}\tsharp-q50\t{b_bytes}\t{b_score:.3f}"]
        for target_name, target in (("A", a_score), ("B", b_score)):
            for tool, label, threads, chroma in CONFIGS:
                print(f"{name} target {target_name} {target:.3f} {label}", file=sys.stderr)
                cmd = config_cmd(tool, threads, chroma, image, corpus, "{q}", "{out}")
                step = "1" if tool == "sharp" else "0"
                result = run([QM, "bisect", ref, f"{target:.3f}", *QUALITY_RANGE, step,
                              work / f"{name}-{target_name}-{label}", "--", *cmd], env=env).strip()
                rows.append(f"{name}\t{target_name}\t{label}\t{result}")
                (out / "matched.tsv").write_text("\n".join(rows) + "\n")
    (out / "anchors.tsv").write_text("\n".join(anchors) + "\n")
    print("\n".join(anchors + rows))


def read_tsv(path):
    with open(path) as f:
        return list(csv.DictReader(f, delimiter="\t"))


def hyperfine(cmd, env, runs, json_path):
    run(["hyperfine", "--warmup", "1", "--runs", runs, "--shell=none", "--export-json", json_path,
         " ".join(f"'{c}'" if " " in str(c) else str(c) for c in cmd)], env=env)
    result = json.loads(Path(json_path).read_text())["results"][0]
    if "measurements" in result:  # hyperfine 2 schema
        walls = [m["time_wall_clock"]["value"] for m in result["measurements"]]
        cpu = statistics.mean(m["time_cpu"]["value"] for m in result["measurements"])
    else:
        walls = result["times"]
        cpu = result["user"] + result["system"]
    return {"mean_s": statistics.mean(walls), "median_s": statistics.median(walls),
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


def timing(out):
    corpus, work = out / "corpus", out / "time"
    work.mkdir(parents=True, exist_ok=True)
    runs = os.environ.get("COMPARE_RUNS", "10")
    matched = {(r["image"], r["target"], r["config"]): r for r in read_tsv(out / "matched.tsv")}
    fields = ["image", "target", "config", "quality", "score", "bytes", "load_before", "load_after", "runs",
              "mean_s", "median_s", "stddev_s", "min_s", "cpu_s", "cpu_per_wall", "in_process_ms", "peak_rss_kib"]
    path = out / "time.csv"
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
                row = {"image": name, "target": "A", "config": label, "quality": quality,
                       "score": m["score_nearest"], "bytes": avif.stat().st_size,
                       "load_before": f"{before:.2f}", "load_after": f"{after:.2f}", "runs": runs,
                       **{k: f"{v:.4f}" for k, v in stats.items()},
                       "cpu_per_wall": f"{stats['cpu_s'] / stats['mean_s']:.2f}",
                       "in_process_ms": f"{inproc:.1f}", "peak_rss_kib": rss}
                writer.writerow(row)
                f.flush()
                print(row, file=sys.stderr)


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
    # Target: Ferrixel's batch setting (quality 80, one thread per image, as
    # Engine::transcode_batch runs it), mean score over the set.
    fdir = work / "ferrixel-q80"
    fdir.mkdir(exist_ok=True)
    for jpg in sorted(small.glob("*.jpg")):
        run(ferrixel_cmd(jpg, fdir / (jpg.stem + ".avif"), w, h, "--assume-srgb", 80, 1))
    fs = small_scores(refs, fdir)
    target = statistics.mean(s for _, s in fs)
    lines.append(f"ferrixel q80: images {len(fs)} mean score {target:.3f} total bytes {sum(b for b, _ in fs)}")
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
        ("ferrixel Engine (20 workers) and sequential", [BATCH, small, w, 0, h], {}),
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
    """Ferrixel's own stage report at the matched setting (target A)."""
    corpus, work = out / "corpus", out / "stages"
    work.mkdir(parents=True, exist_ok=True)
    runs = int(os.environ.get("COMPARE_RUNS", "10"))
    matched = {(r["image"], r["config"]): r for r in read_tsv(out / "matched.tsv") if r["target"] == "A"}
    keys = ["open_us", "decode_us", "transform_us", "encode_us", "total_us"]
    lines = ["image\tconfig\tload_before\tload_after\t" + "\t".join(k.replace("_us", "_ms") for k in keys)]
    for image in IMAGES:
        for tool, label, threads, chroma in CONFIGS:
            if tool != "ferrixel":
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
SWEEP = [("ferrixel", 4), ("ferrixel", 8), ("ferrixel", 10), ("sharp", 0), ("sharp", 2), ("sharp", 6)]


def sweep_cmd(image, corpus, tool, preset, quality, path):
    name, file, flag, icc, w, h, _ = image
    if tool == "ferrixel":
        return [*ferrixel_cmd(corpus / file, path, w, h, flag, quality, 0), f"--speed={preset}"]
    return sharp_cmd(corpus / file, path, w, h, icc, quality, 0, extra=(f"effort={preset}",))


def sweep_match(out):
    """Bytes and scores of the preset sweep (no timing; safe under load)."""
    corpus, refs, work = out / "corpus", out / "refs", out / "sweep"
    work.mkdir(parents=True, exist_ok=True)
    targets = {r["image"]: float(r["score"]) for r in read_tsv(out / "anchors.tsv") if r["anchor"] == "ferrixel-q80"}
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


def main():
    if len(sys.argv) != 3:
        sys.exit(__doc__)
    phase, out = sys.argv[1], Path(sys.argv[2]).resolve()
    out.mkdir(parents=True, exist_ok=True)
    os.chdir(REPO)
    phases = {"prepare": prepare, "ceiling": ceiling, "match": match, "time": timing, "batch": batch,
              "stages": stages, "sweepmatch": sweep_match, "sweep": sweep}
    for name in (phases if phase == "all" else [phase]):
        if name not in phases:
            sys.exit(__doc__)
        phases[name](out)


if __name__ == "__main__":
    main()
