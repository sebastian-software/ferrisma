"""Summarize interleaved encoder runs, rescore outputs, attach match status.

Usage: python3 encsum.py OUT.json RUN.json...
"""
import json
import subprocess
import sys

S = "/private/tmp/claude-501/-Users-sebastian-Workspace-ferrixel--claude-worktrees-validate-issues-labels-v1-7aadd3/7d1a4515-4dbd-40f9-9bc7-0df0d4838873/scratchpad"
QM = "/Users/sebastian/Workspace/ferrixel/.claude/worktrees/agent-a5b9537dab7d91c10/scripts/bench/quality-match/target/release/ferrisma-quality-match"
DIRS = {"elcito": "enc-elcito", "beach": "enc-beach"}

rows = []
for path in sys.argv[2:]:
    rec = json.load(open(path))
    for cand in rec["plan"]["candidates"]:
        cid = cand["id"]
        image, level, threads, lane = cid.split("-", 3)
        level, threads = int(level[1:]), int(threads[1:])
        d = f"{S}/{DIRS.get(image, 'enc-default')}"
        matched = json.load(open(f"{d}/encoders/matched.json"))
        s = rec["summary"][cid]
        if len(s["output_sha256"]) != 1:
            raise SystemExit(f"non-deterministic output: {cid}")
        out = subprocess.run([QM, "score", f"{d}/refs/{image}.ref", cand["output"]], capture_output=True,
                             text=True, check=True).stdout.split("\t")
        score = float(out[2])
        row = {"image": image, "anchor_quality": level, "threads": threads, "lane": lane,
               "quality": int(next(a for a in cand["cmd"] if a.startswith("--quality=")).split("=")[1]),
               "bytes": s["output_bytes"][0], "output_sha256": s["output_sha256"][0], "score": score,
               "wall_median_s": s["wall_median_s"], "wall_min_s": s["wall_min_s"], "wall_max_s": s["wall_max_s"],
               "rss_median_bytes": s["rss_median_bytes"], "rss_max_bytes": s["rss_max_bytes"],
               "load_min": s["load_min"], "load_max": s["load_max"], "n": s["n"],
               "uptime_before": rec["uptime_before"], "uptime_after": rec["uptime_after"]}
        if lane.startswith("aom"):
            m = next(r for r in matched if r["image"] == image and r["anchor_quality"] == level
                     and r["threads"] == threads and r["speed"] == int(lane[-1]))
            gap = float(m["score_nearest"]) - float(m["target_score"])
            row.update(target_score=float(m["target_score"]), match_gap=gap,
                       match_status="rejected (|gap| > 0.5)" if abs(gap) > 0.5 else "accepted",
                       rescore_vs_match=score - float(m["score_nearest"]),
                       bytes_at_target_estimate=float(m["bytes_at_target"]))
        else:
            m = next(r for r in matched if r["image"] == image and r["anchor_quality"] == level
                     and r["threads"] == threads)
            row.update(target_score=float(m["anchor_score"]), rescore_vs_match=score - float(m["anchor_score"]),
                       anchor_sha256_matches=s["output_sha256"][0] == m["anchor_sha256"])
        rows.append(row)
json.dump(rows, open(sys.argv[1], "w"), indent=1)
for r in rows:
    print(f"| {r['image']} | {r['anchor_quality']} | t{r['threads']} | {r['lane']} q{r['quality']} | "
          f"{r['wall_median_s']:.2f} ({r['wall_min_s']:.2f}–{r['wall_max_s']:.2f}) | {r['rss_max_bytes'] / 1e6:.0f} | "
          f"{r['bytes']} | {r['score']:.2f} | {r.get('match_status', 'anchor')} {r.get('match_gap', 0):+.2f} | "
          f"load {r['load_min']:.1f}–{r['load_max']:.1f} | resc {r['rescore_vs_match']:+.3f} {r.get('anchor_sha256_matches', '')}")
