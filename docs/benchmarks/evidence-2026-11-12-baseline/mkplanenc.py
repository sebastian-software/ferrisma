"""Interleaved encoder plans from a compare-encoders matched.json.

Usage: python3 mkplanenc.py OUT_DIR IMAGE THREADS PLAN.json
"""
import json
import sys

F = "/Users/sebastian/Workspace/ferrixel/.claude/worktrees/agent-a5b9537dab7d91c10/target/release/ferrisma"
IMAGES = {
    "dog": ("dog.jpg", "--use-rendered-icc", 1600, 2133, []),
    "flower": ("flower.jpg", "--assume-srgb", 1600, 1067, []),
    "kodim03": ("kodim03.jpg", "--assume-srgb", 512, 341, []),
    "kodim23": ("kodim23.jpg", "--assume-srgb", 512, 341, []),
    "elcito": ("elcito.jpg", "--assume-srgb", 1600, 1067, ["--budget-mb=2048"]),
    "beach": ("beach.jpg", "--use-rendered-icc", 1600, 1067, ["--budget-mb=2048"]),
}
out, image, threads, plan_path = sys.argv[1], sys.argv[2], int(sys.argv[3]), sys.argv[4]
file, flag, w, h, extra = IMAGES[image]
matched = json.load(open(f"{out}/encoders/matched.json"))
cands = []
for level in sorted({r["anchor_quality"] for r in matched}):
    lanes = [("rav1e-s6", "rav1e", level, 6)]
    for speed in (6, 7):
        row = next(r for r in matched if r["image"] == image and r["anchor_quality"] == level
                   and r["threads"] == threads and r["speed"] == speed)
        lanes.append((f"aom-s{speed}", "libavif", int(row["q_nearest"]), speed))
    for lane, encoder, quality, speed in lanes:
        path = f"{out}/timing/{image}-q{level}-t{threads}-{lane}.avif"
        cands.append({"id": f"{image}-q{level}-t{threads}-{lane}", "output": path,
                      "cmd": [F, f"{out}/corpus/{file}", path, str(w), str(h), flag,
                              f"--avif-encoder={encoder}", f"--quality={quality}", f"--speed={speed}",
                              f"--encoder-threads={threads}", f"--transform-threads={threads}",
                              "--chroma=444", *extra]})
json.dump({"reps": 7, "warmups": 1, "candidates": cands}, open(plan_path, "w"), indent=1)
