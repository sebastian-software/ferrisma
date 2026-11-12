import json
import os

P = os.path.dirname(os.path.abspath(__file__))
I = os.path.join(os.path.dirname(P), "in19")
cands = []
rows = [
    ("avif24-webp", "avif24.avif", "webp", 1600, 1067, "--assume-srgb", []),
    ("avif24-avif", "avif24.avif", "avif", 1600, 1067, "--assume-srgb", ["--encoder-threads=0"]),
    ("avif12-webp", "avif12.avif", "webp", 1600, 1200, "--use-rendered-icc", []),
]
for t in ("0", "1"):
    for cid, f, fmt, w, h, prof, extra in rows:
        for b in ("base", "yuvmt"):
            out = os.path.join(P, "out", f"pl-{cid}-t{t}-{b}.{fmt}")
            ex = [e for e in extra if t == "0"] or (["--encoder-threads=1"] if fmt == "avif" else [])
            cands.append({"id": f"{cid}-t{t}-{b}", "output": out,
                          "cmd": [os.path.join(P, f"ferrisma-{b}"), os.path.join(I, f), out, str(w), str(h), prof,
                                  "--budget-mb=2048", f"--transform-threads={t}", *ex]})
json.dump({"reps": 7, "warmups": 1, "candidates": cands}, open(os.path.join(P, "plan-yuvmt.json"), "w"), indent=1)
