#!/usr/bin/env python3
"""Summarize a samply profile (+ .syms.json sidecar) into function and stage shares.

usage: prof_summary.py <profile.json.gz> [top] [--stages <stages.json>]
Prints top-N exclusive and inclusive outer (non-inlined) symbols and stage shares.
Stage attribution: each sample goes to the first stage (in file order) whose regex
matches any frame of the stack, scanning from the LEAF outward (innermost wins).
"""
import bisect, gzip, json, re, sys
from collections import Counter

path = sys.argv[1]
top = int(sys.argv[2]) if len(sys.argv) > 2 and sys.argv[2].isdigit() else 15
stages = []
if "--stages" in sys.argv:
    stages = [(n, re.compile(r)) for n, r in json.load(open(sys.argv[sys.argv.index("--stages") + 1]))]
prof = json.load(gzip.open(path))
syms = json.load(open(path.replace(".json.gz", ".json.syms.json")))
st = syms["string_table"]
bylib = {}
for d in syms["data"]:
    tab = sorted(d["symbol_table"], key=lambda e: e["rva"])
    bylib[d["debug_name"]] = ([e["rva"] for e in tab], tab)


def clean(name):
    name = re.sub(r"::h[0-9a-f]{16}$", "", name)
    return name


cache = {}


def symname(libname, addr):
    key = (libname, addr)
    if key in cache:
        return cache[key]
    name = f"{libname}+0x{addr:x}"
    if libname in bylib:
        rvas, tab = bylib[libname]
        i = bisect.bisect_right(rvas, addr) - 1
        if i >= 0 and addr < tab[i]["rva"] + tab[i]["size"]:
            name = clean(st[tab[i]["symbol"]])
    cache[key] = name
    return name


excl, incl, stage_count = Counter(), Counter(), Counter()
total = 0
for t in prof["threads"]:
    strings = t["stringArray"]
    ft, fn, rt, stt = t["frameTable"], t["funcTable"], t["resourceTable"], t["stackTable"]
    libs = prof["libs"]

    def frame_name(f):
        func = ft["func"][f]
        res = fn["resource"][func]
        addr = ft["address"][f]
        if res is None or res < 0 or addr is None or addr < 0:
            return strings[fn["name"][func]]
        lib = libs[rt["lib"][res]]["debugName"]
        return symname(lib, addr)

    names = {}
    for s, w in zip(t["samples"]["stack"], t["samples"].get("weight") or [1] * t["samples"]["length"]):
        if s is None:
            continue
        if s not in names:
            chain = []
            x = s
            while x is not None:
                chain.append(frame_name(stt["frame"][x]))
                x = stt["prefix"][x]
            names[s] = chain  # leaf first
        chain = names[s]
        total += w
        excl[chain[0]] += w
        for n in set(chain):
            incl[n] += w
        label = "other"
        for n in chain:
            hit = next((sn for sn, rx in stages if rx.search(n)), None)
            if hit:
                label = hit
                break
        stage_count[label] += w

print(f"samples={total}")
print(f"\n## Top {top} exclusive (self) symbols")
for n, c in excl.most_common(top):
    print(f"{100*c/total:6.1f}%  {n[:150]}")
print(f"\n## Top {top} inclusive symbols")
for n, c in incl.most_common(top + 10)[: top + 10]:
    print(f"{100*c/total:6.1f}%  {n[:150]}")
if stages:
    print("\n## Stage shares (innermost matching frame)")
    for n, c in stage_count.most_common():
        print(f"{100*c/total:6.1f}%  {n}")

if "--callers" in sys.argv:
    pat = re.compile(sys.argv[sys.argv.index("--callers") + 1])
    callers = Counter()
    sub = 0
    for t in prof["threads"]:
        pass
    # recompute using cached chains of the last thread
    for s, chain in names.items():
        pass
    for t in prof["threads"]:
        for s in t["samples"]["stack"]:
            if s is None:
                continue
            chain = names[s]
            if pat.search(chain[0]):
                sub += 1
                callers[chain[1] if len(chain) > 1 else "-"] += 1
    print(f"\n## Direct callers of leaf /{pat.pattern}/ ({100*sub/total:.1f}% of samples)")
    for n, c in callers.most_common(12):
        print(f"{100*c/total:6.1f}%  {n[:150]}")
