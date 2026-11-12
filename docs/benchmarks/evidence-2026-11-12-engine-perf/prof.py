"""Summarize a samply profile (+ .syms.json sidecar) as self/inclusive tables.

Usage: python3 -I prof.py <profile.json.gz> [top] [--group]
Weights samples by thread CPU delta (so idle/waiting threads count ~0).
"""
import bisect
import gzip
import json
import re
import sys
from collections import Counter

path = sys.argv[1]
top = int(sys.argv[2]) if len(sys.argv) > 2 and sys.argv[2].isdigit() else 30
p = json.load(gzip.open(path))
syms = json.load(open(re.sub(r"\.gz$", "", path).replace(".json", ".json.syms.json")))
strings = syms["string_table"]
tables = {}
for lib in syms["data"]:
    st = lib["symbol_table"]
    if isinstance(st, str):
        st = json.loads(st.replace("'", '"'))
    st = sorted(st, key=lambda e: e["rva"])
    tables[lib["debug_name"]] = ([e["rva"] for e in st], st)
libs = p["libs"]


def short(name):
    # drop closures' generic arg lists only beyond depth 2
    for _ in range(3):
        name = re.sub(r"<([^<>]{0,60})<[^<>]*<[^<>]*>[^<>]*>", r"<\1<..>", name)
    name = name.replace("core::", "").replace("alloc::", "")
    name = re.sub(r"::h[0-9a-f]{16}$", "", name)
    return name[:200]


def lookup(libname, addr):
    t = tables.get(libname)
    if not t:
        return f"{libname}+?"
    i = bisect.bisect_right(t[0], addr) - 1
    if i < 0:
        return f"{libname}+?"
    e = t[1][i]
    if addr >= e["rva"] + e.get("size", 1 << 30):
        return f"{libname}+?"
    return short(strings[e["symbol"]])


CATS = [
    ("icc", r"moxcms|ferrisma::cms::"),
    ("orientation", r"ferrisma::orientation"),
    ("resize", r"horizontal_rgba|neon::axpy|scalar::axpy|ferrisma_kernels"),
    ("quantize", r"quantize_row"),
    ("linearize", r"source_row_to_linear"),
    ("rgb2yuv", r"avifImageRGBToYUV|RGBToYUV|sharpyuv|rgb_to_yuv|import_rgb|picture_csp|WebPPictureImport|import_yuva|ImportYUVA"),
    ("aom-encode", r"aom_codec_encode|av1_encode|encoder_encode|av1_get_compressed"),
    ("rav1e-encode", r"^rav1e|<rav1e|rav1e::"),
    ("webp-encode", r"webp_encode|WebPEncode|vp8_enc|webp::enc::|webp::dsp::enc|cost_neon"),
    ("av1-decode", r"aom_codec_decode|decoder_decode|av1_receive_compressed|aomCodecGetNextImage|av1__decoder|dav1d"),
    ("yuv2rgb", r"avifImageYUVToRGB|YUVToRGB|reformat"),
    ("jpeg-decode", r"^jpeg::|JpegSource"),
    ("webp-decode", r"webp::dec|WebPDecode|vp8_decode|vp8l|VP8|webp::dsp::|webp::utils|WebpSource|codecs::webp"),
    ("avif-decode-other", r"AvifSource|avifDecoder|ferrisma_avif_read"),
    ("sync/wait", r"psynch|pthread_cond|mpsc|thread::park|semaphore|__ulock|futex|Condvar|recv"),
    ("engine-pipeline", r"parallel_execute|fast_execute|ferrisma::pipeline|ferrisma::"),
]
import re as _re
CATS = [(c, _re.compile(r)) for c, r in CATS]
stage_c = Counter()
stage_fn = {}
self_c, incl_c = Counter(), Counter()
total = 0.0
for th in p["threads"]:
    ft, fn, rt, sa = th["frameTable"], th["funcTable"], th["resourceTable"], th["stringArray"]
    stk = th["stackTable"]
    names = {}

    def frame_name(f):
        if f in names:
            return names[f]
        func = ft["func"][f]
        res = fn["resource"][func]
        if res is None or res < 0:
            n = sa[fn["name"][func]]
        else:
            lib = libs[rt["lib"][res]]["debugName"]
            n = lookup(lib, ft["address"][f])
        names[f] = n
        return n

    s = th["samples"]
    w = s.get("threadCPUDelta") or [1] * s["length"]
    for i in range(s["length"]):
        st = s["stack"][i]
        if st is None:
            continue
        wt = w[i] or 0
        if wt <= 0:
            continue
        total += wt
        seen = set()
        leaf = True
        cat = None
        leafname = None
        while st is not None:
            n = frame_name(stk["frame"][st])
            if leaf:
                self_c[n] += wt
                leafname = n
                leaf = False
            if cat is None:
                for c, rx in CATS:
                    if rx.search(n):
                        cat = c
                        break
            if n not in seen:
                incl_c[n] += wt
                seen.add(n)
            st = stk["prefix"][st]
        cat = cat or "other"
        stage_c[cat] += wt
        stage_fn.setdefault(cat, Counter())[leafname] += wt

print(f"# {path}: total CPU weight {total:.0f} (thread CPU delta units, us)")
print(f"\n## Self (top {top})\n| % | function |\n| ---: | --- |")
for n, c in self_c.most_common(top):
    print(f"| {100*c/total:.1f} | `{n}` |")
print(f"\n## Inclusive (top {top})\n| % | function |\n| ---: | --- |")
for n, c in incl_c.most_common(top):
    print(f"| {100*c/total:.1f} | `{n}` |")

print("\n## Stage shares (leaf-to-root first match)\n| stage | % CPU | top self functions in stage |\n| --- | ---: | --- |")
for c, v in stage_c.most_common():
    tops = "; ".join(f"`{n[:70]}` {100*k/total:.1f}" for n, k in stage_fn[c].most_common(4))
    print(f"| {c} | {100*v/total:.1f} | {tops} |")

if "--threads" in sys.argv:
    print("\n## Per-thread CPU (top 6 threads)\n| thread | % CPU of process | top self functions |\n| --- | ---: | --- |")
    rows = []
    for th in p["threads"]:
        ft, fn, rt, sa = th["frameTable"], th["funcTable"], th["resourceTable"], th["stringArray"]
        stk = th["stackTable"]
        s = th["samples"]
        w = s.get("threadCPUDelta") or [1] * s["length"]
        c = Counter()
        tot = 0
        for i in range(s["length"]):
            st = s["stack"][i]
            if st is None or not w[i]:
                continue
            f = stk["frame"][st]
            func = ft["func"][f]
            res = fn["resource"][func]
            if res is None or res < 0:
                n = sa[fn["name"][func]]
            else:
                n = lookup(libs[rt["lib"][res]]["debugName"], ft["address"][f])
            c[n] += w[i]
            tot += w[i]
        rows.append((tot, th.get("name"), c))
    rows.sort(key=lambda r: -r[0])
    for tot, name, c in rows[:6]:
        tops = "; ".join(f"`{n[:60]}` {100*k/tot:.0f}%" for n, k in c.most_common(5))
        print(f"| {name} | {100*tot/total:.1f} | {tops} |")
    print(f"\nthreads with samples: {sum(1 for r in rows if r[0] > 0)}")
