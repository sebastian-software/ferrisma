"""usage: stage_table.py <dir> name=total_ms ... ; reads <name>.stages.txt; prints a ms table."""
import sys

P = sys.argv[1]
tot = dict((a.split("=")[0], float(a.split("=")[1])) for a in sys.argv[2:])
names = list(tot)
d = {}
for f, t in tot.items():
    for line in open(f"{P}/{f}.stages.txt"):
        pct, name = line.strip().split("%", 1)
        d.setdefault(name.strip(), {})[f] = float(pct) * t / 100
print("| stage | " + " | ".join(names) + " |")
print("|---" * (len(names) + 1) + "|")
for name, v in sorted(d.items(), key=lambda kv: -kv[1].get(names[0], 0)):
    print(f"| {name} | " + " | ".join(f"{v.get(n, 0):.1f}" for n in names) + " |")
