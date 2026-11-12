"""Interleaved Sharp/Ferrisma plan from compare-sharp.py matched.tsv (target A).

Usage: python3 mkplansharp.py IMAGE THREADS PLAN.json
"""
import csv
import json
import sys

S = "/private/tmp/claude-501/-Users-sebastian-Workspace-ferrixel--claude-worktrees-validate-issues-labels-v1-7aadd3/7d1a4515-4dbd-40f9-9bc7-0df0d4838873/scratchpad"
R = "/Users/sebastian/Workspace/ferrixel/.claude/worktrees/agent-a5b9537dab7d91c10"
F = f"{R}/target/release/ferrisma"
SHARP = f"{R}/scripts/bench/sharp-avif.cjs"
NODE = "/private/tmp/ferramenta300-registry-consumer/node_modules"
IMAGES = {"dog": ("dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133),
          "flower": ("flower.jpg", "--assume-srgb", "convert", 1600, 1067),
          "kodim03": ("kodim03.jpg", "--assume-srgb", "convert", 512, 341),
          "kodim23": ("kodim23.jpg", "--assume-srgb", "convert", 512, 341)}
image, threads, plan_path = sys.argv[1], int(sys.argv[2]), sys.argv[3]
file, flag, icc, w, h = IMAGES[image]
label = "all" if threads == 0 else "one"
matched = {(r["image"], r["config"]): r for r in csv.DictReader(open(f"{S}/enc-default/matched.tsv"), delimiter="\t")
           if r["target"] == "A"}
enc = json.load(open(f"{S}/enc-default/encoders/matched.json"))
src = f"{S}/enc-default/corpus/{file}"
fq = matched[(image, f"ferrisma-{label}-444")]["q_nearest"]
sq = matched[(image, f"sharp-{label}-444")]["q_nearest"]
aq = next(r for r in enc if r["image"] == image and r["anchor_quality"] == 80 and r["threads"] == threads
          and r["speed"] == 6)["q_nearest"]
o = f"{S}/enc-default/sharp-timing"
sharp_env = {"NODE_PATH": NODE, **({"UV_THREADPOOL_SIZE": "1"} if threads == 1 else {})}
cands = [
    {"id": f"{image}-{label}-ferrisma-rav1e-s6", "output": f"{o}/{image}-{label}-rav1e.avif",
     "cmd": [F, src, f"{o}/{image}-{label}-rav1e.avif", str(w), str(h), flag, "--avif-encoder=rav1e",
             f"--quality={fq}", f"--encoder-threads={threads}", f"--transform-threads={threads}", "--chroma=444",
             "--speed=6"]},
    {"id": f"{image}-{label}-sharp", "output": f"{o}/{image}-{label}-sharp.avif", "env": sharp_env,
     "cmd": ["node", SHARP, src, f"{o}/{image}-{label}-sharp.avif", str(w), str(h), f"quality={sq}", f"icc={icc}",
             f"threads={threads}", "chroma=444"]},
    {"id": f"{image}-{label}-ferrisma-aom-s6", "output": f"{o}/{image}-{label}-aom.avif",
     "cmd": [F, src, f"{o}/{image}-{label}-aom.avif", str(w), str(h), flag, "--avif-encoder=libavif",
             f"--quality={aq}", f"--encoder-threads={threads}", f"--transform-threads={threads}", "--chroma=444",
             "--speed=6"]},
]
json.dump({"reps": 7, "warmups": 1, "candidates": cands}, open(plan_path, "w"), indent=1)
