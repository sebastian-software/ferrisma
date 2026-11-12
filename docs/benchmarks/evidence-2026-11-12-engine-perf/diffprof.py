"""Compare self CPU per function between a port profile and a C profile.

Usage: python3 -I diffprof.py <port.json.gz> <c.json.gz> <port_ms> <c_ms> [top]
Self shares are scaled to the measured per-run milliseconds (port_ms, c_ms)
so the table shows estimated ms per function and the port-minus-C delta.
Names are normalized: module path dropped, `_c`/`_neon`/`_facade` suffixes and
`aom::...::` prefixes removed, so the port's `_c` fallback and C's `_neon`
kernel land in one row; the variant each side ran is shown.
"""
import re
import subprocess
import sys
from collections import Counter, defaultdict

port, cprof, port_ms, c_ms = sys.argv[1], sys.argv[2], float(sys.argv[3]), float(sys.argv[4])
top = int(sys.argv[5]) if len(sys.argv) > 5 else 25


def selfs(path):
    out = subprocess.run([sys.executable, "-I", __file__.replace("diffprof.py", "prof.py"), path, "400"],
                         capture_output=True, text=True).stdout
    sec = out.split("## Self")[1].split("## Inclusive")[0]
    rows = re.findall(r"\| ([\d.]+) \| `(.*)` \|", sec)
    return [(float(p), n) for p, n in rows]


def norm(n):
    n = re.sub(r"<[^>]*>", "", n)
    base = n.split("::")[-1] if "::" in n else n
    base = re.sub(r"^(simd_|_)", "", base)
    variant = "neon" if re.search(r"_neon", base) else ("c" if re.search(r"_c$|_c_", base) else "")
    key = re.sub(r"_(neon|c|facade)(_|$)", r"\2", base).rstrip("_").lower()
    key = key.replace("helper", "").strip("_")
    return key, variant


agg = {"port": defaultdict(float), "c": defaultdict(float)}
var = {"port": defaultdict(set), "c": defaultdict(set)}
for side, path, ms in (("port", port, port_ms), ("c", cprof, c_ms)):
    for share, name in selfs(path):
        k, v = norm(name)
        agg[side][k] += share * ms / 100
        if v:
            var[side][k].add(v)
keys = set(agg["port"]) | set(agg["c"])
rows = sorted(keys, key=lambda k: -(agg["port"][k] - agg["c"][k]))
print(f"| function (normalized) | port ms | port variant | C ms | C variant | port − C ms |\n| --- | ---: | --- | ---: | --- | ---: |")
for k in rows[:top]:
    print(f"| `{k}` | {agg['port'][k]:.0f} | {'/'.join(sorted(var['port'][k])) or '-'} | {agg['c'][k]:.0f} | "
          f"{'/'.join(sorted(var['c'][k])) or '-'} | {agg['port'][k]-agg['c'][k]:+.0f} |")
print(f"\n(total port {port_ms:.0f} ms, C {c_ms:.0f} ms; gap {port_ms-c_ms:+.0f} ms)")
