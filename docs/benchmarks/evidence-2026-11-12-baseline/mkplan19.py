import json
import sys

S = "/private/tmp/claude-501/-Users-sebastian-Workspace-ferrixel--claude-worktrees-validate-issues-labels-v1-7aadd3/7d1a4515-4dbd-40f9-9bc7-0df0d4838873/scratchpad"
F = "/Users/sebastian/Workspace/ferrixel/.claude/worktrees/agent-a5b9537dab7d91c10/target/release/ferrisma"
size = sys.argv[1]
if size == "12":
    rows = [("jpeg12-o1", "jpeg12-o1.jpg", 1600, 1200, "--use-rendered-icc", []),
            ("jpeg12-o6", "jpeg12-o6.jpg", 1200, 1600, "--use-rendered-icc", []),
            ("jpeg12-o2", "jpeg12-o2.jpg", 1600, 1200, "--use-rendered-icc", []),
            ("webp12", "webp12.webp", 1600, 1200, "--use-rendered-icc", []),
            ("avif12", "avif12.avif", 1600, 1200, "--use-rendered-icc", []),
            ("jxl12-libjxl-rs", "jxl12.jxl", 1200, 1600, "--use-rendered-icc", ["--jxl-decoder=libjxl-rs"]),
            ("jxl12-jxl-oxide", "jxl12.jxl", 1200, 1600, "--use-rendered-icc", ["--jxl-decoder=jxl-oxide"])]
else:
    rows = [("jpeg24-o1", "jpeg24-o1.jpg", 1600, 1067, "--assume-srgb", []),
            ("jpeg24-o6", "jpeg24-o6.jpg", 1067, 1600, "--assume-srgb", []),
            ("jpeg24-o2", "jpeg24-o2.jpg", 1600, 1067, "--assume-srgb", []),
            ("webp24", "webp24.webp", 1600, 1067, "--assume-srgb", []),
            ("avif24", "avif24.avif", 1600, 1067, "--assume-srgb", []),
            ("jxl24-libjxl-rs", "jxl24.jxl", 1600, 1067, "--use-rendered-icc", ["--jxl-decoder=libjxl-rs"]),
            ("jxl24-jxl-oxide", "jxl24.jxl", 1600, 1067, "--use-rendered-icc", ["--jxl-decoder=jxl-oxide"])]
cands = []
for threads in ("1", "0"):
    for cid, f, w, h, prof, extra in rows:
        if threads == "0" and not cid.startswith("jpeg"):
            continue
        out = f"{S}/runs/out19-{cid}-t{threads}.webp"
        cands.append({"id": f"{cid}-t{threads}", "output": out,
                      "cmd": [F, f"{S}/in19/{f}", out, str(w), str(h), prof, *extra, "--budget-mb=2048",
                              f"--transform-threads={threads}", "--encoder-threads=1"]})
json.dump({"reps": 7, "warmups": 1, "candidates": cands}, open(f"{S}/runs/r19-{size}.plan.json", "w"), indent=1)
