#!/usr/bin/env python3
"""Inclusive sample share of stacks containing a frame matching a regex.

usage: incl_share.py <profile.json.gz> <regex>...
Robust against identical-code folding of leaf helpers: it counts every sample
whose stack passes through a matching (outer, non-inlined) frame.
"""
import bisect, gzip, json, re, sys

path = sys.argv[1]
pats = [re.compile(p) for p in sys.argv[2:]]
prof = json.load(gzip.open(path))
syms = json.load(open(path.replace(".json.gz", ".json.syms.json")))
st = syms["string_table"]
bylib = {}
for d in syms["data"]:
    tab = sorted(d["symbol_table"], key=lambda e: e["rva"])
    bylib[d["debug_name"]] = ([e["rva"] for e in tab], tab)


def symname(lib, addr):
    if lib in bylib:
        rvas, tab = bylib[lib]
        i = bisect.bisect_right(rvas, addr) - 1
        if i >= 0 and addr < tab[i]["rva"] + tab[i]["size"]:
            return st[tab[i]["symbol"]]
    return f"{lib}+0x{addr:x}"


total = 0
hits = [0] * len(pats)
for t in prof["threads"]:
    strings = t["stringArray"]
    ft, fn, rt, stt = t["frameTable"], t["funcTable"], t["resourceTable"], t["stackTable"]
    memo = {}
    for s in t["samples"]["stack"]:
        if s is None:
            continue
        if s not in memo:
            names = []
            x = s
            while x is not None:
                f = stt["frame"][x]
                func = ft["func"][f]
                res = fn["resource"][func]
                addr = ft["address"][f]
                if res is None or res < 0 or addr is None or addr < 0:
                    names.append(strings[fn["name"][func]])
                else:
                    names.append(symname(prof["libs"][rt["lib"][res]]["debugName"], addr))
                x = stt["prefix"][x]
            memo[s] = [any(p.search(n) for n in names) for p in pats]
        total += 1
        for i, h in enumerate(memo[s]):
            hits[i] += h
print(f"samples={total}")
for p, h in zip(pats, hits):
    print(f"{100 * h / total:6.1f}%  /{p.pattern}/")
