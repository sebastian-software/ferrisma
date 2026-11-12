"""Render markdown tables from aomcmp.json, jpegcmp.json and stages JSON files."""
import json
import os
import statistics as st
import sys

S = os.path.dirname(os.path.abspath(__file__))
what = sys.argv[1]


def mm(v):
    return f"{st.median(v):.0f} ({min(v):.0f}–{max(v):.0f})"


if what == "aom":
    r = json.load(open(os.path.join(S, sys.argv[2] if len(sys.argv) > 2 else "aomcmp.json")))
    keys = sorted({k.rsplit("-", 1)[0] for k in r})
    print("| case | port ms | C ms | port/C | bytes port / C | max RSS MB port / C | load |")
    print("| --- | ---: | ---: | ---: | ---: | ---: | ---: |")
    for k in keys:
        p = [s["us"] / 1000 for s in r[k + "-port"]]
        c = [s["us"] / 1000 for s in r[k + "-c"]]
        bp, bc = r[k + "-port"][0]["bytes"], r[k + "-c"][0]["bytes"]
        rp = max(s["rss"] for s in r[k + "-port"]) / 2**20
        rc = max(s["rss"] for s in r[k + "-c"]) / 2**20
        loads = [s["load"] for s in r[k + "-port"] + r[k + "-c"]]
        print(f"| {k} | {mm(p)} | {mm(c)} | {st.median(p)/st.median(c):.2f} | {bp:,} / {bc:,} | {rp:.0f} / {rc:.0f} | {min(loads):.1f}–{max(loads):.1f} |")
elif what == "jpeg":
    r = json.load(open(os.path.join(S, "jpegcmp.json")))
    keys = sorted({k.rsplit("|", 1)[0] for k in r})
    print("| input | mode | port ms/decode | C ms/decode | port/C | same checksum | load |")
    print("| --- | --- | ---: | ---: | ---: | --- | ---: |")
    for k in keys:
        p = [s["ms_per_decode"] for s in r[k + "|port"]]
        c = [s["ms_per_decode"] for s in r[k + "|c"]]
        same = {s["checksum"] for s in r[k + "|port"]} == {s["checksum"] for s in r[k + "|c"]}
        loads = [s["load"] for s in r[k + "|port"] + r[k + "|c"]]
        i, m = k.split("|")
        print(f"| {i} | {m} | {mm(p)} | {mm(c)} | {st.median(p)/st.median(c):.2f} | {same} | {min(loads):.1f}–{max(loads):.1f} |")
elif what == "stages":
    r = json.load(open(os.path.join(S, sys.argv[2])))
    print("| config | wall total ms | decode % | transform % | encode % | open % | managed MB | encoder frame MB | max RSS MB | load |")
    print("| --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |")
    for cid, v in r.items():
        ok = [s for s in v["samples"] if "error" not in s]
        if not ok:
            print(f"| {cid} | rejected: WebP output requires --encoder-threads=1 | | | | | | | | |")
            continue
        tot = st.median(int(s["total_us"]) for s in ok) / 1000
        sh = {}
        for f in ("decode_us", "transform_us", "encode_us", "open_us"):
            sh[f] = 100 * st.median(int(s[f]) / int(s["total_us"]) for s in ok)
        man = max(int(s["peak_managed_bytes"]) for s in ok) / 1e6
        enc = max(int(s["encoder_frame_bytes"]) for s in ok) / 1e6
        rss = max(s["rss"] for s in ok) / 1e6
        loads = [s["load"] for s in ok]
        print(f"| {cid} | {tot:.0f} | {sh['decode_us']:.0f} | {sh['transform_us']:.0f} | {sh['encode_us']:.0f} | {sh['open_us']:.1f} | {man:.1f} | {enc:.1f} | {rss:.0f} | {min(loads):.0f}–{max(loads):.0f} |")
