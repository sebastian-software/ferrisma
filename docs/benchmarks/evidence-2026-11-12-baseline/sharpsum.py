import json
import subprocess
import sys

S = "/private/tmp/claude-501/-Users-sebastian-Workspace-ferrixel--claude-worktrees-validate-issues-labels-v1-7aadd3/7d1a4515-4dbd-40f9-9bc7-0df0d4838873/scratchpad"
QM = "/Users/sebastian/Workspace/ferrixel/.claude/worktrees/agent-a5b9537dab7d91c10/scripts/bench/quality-match/target/release/ferrisma-quality-match"
TARGET = {"dog": 73.467, "flower": 76.508, "kodim03": 72.678, "kodim23": 76.223}
rows = []
for t in (1, 0):
    for image in ("dog", "flower", "kodim03", "kodim23"):
        rec = json.load(open(f"{S}/runs/sharp-{image}-t{t}.json"))
        for cand in rec["plan"]["candidates"]:
            s = rec["summary"][cand["id"]]
            assert len(s["output_sha256"]) == 1, cand["id"]
            score = float(subprocess.run([QM, "score", f"{S}/enc-default/refs/{image}.ref", cand["output"]],
                                         capture_output=True, text=True, check=True).stdout.split("\t")[2])
            q = next(a for a in cand["cmd"] if "quality=" in a).split("=")[1]
            delta = score - TARGET[image]
            rows.append({"id": cand["id"], "image": image, "threads": t, "quality": q, "score": score,
                         "target_A": TARGET[image], "delta": delta,
                         "status": "rejected (|delta| > 0.5)" if abs(delta) > 0.5 else "within 0.5",
                         "bytes": s["output_bytes"][0], "output_sha256": s["output_sha256"][0],
                         "wall_median_s": s["wall_median_s"], "wall_min_s": s["wall_min_s"],
                         "wall_max_s": s["wall_max_s"], "rss_max_bytes": s["rss_max_bytes"],
                         "load_min": s["load_min"], "load_max": s["load_max"]})
json.dump(rows, open(sys.argv[1], "w"), indent=1)
for r in rows:
    print(f"| {r['image']} | t{r['threads']} | {r['id'].split('-', 2)[2]} q{r['quality']} | {r['wall_median_s']:.2f} "
          f"({r['wall_min_s']:.2f}–{r['wall_max_s']:.2f}) | {r['rss_max_bytes'] / 1e6:.0f} | {r['bytes']:,} | "
          f"{r['score']:.2f} ({r['delta']:+.2f}) {r['status']} | {r['load_min']:.1f}–{r['load_max']:.1f} |")
