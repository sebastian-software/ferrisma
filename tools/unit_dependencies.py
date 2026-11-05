#!/usr/bin/env python3
"""Compare the work plan's package dependencies with libwebp's actual ones.

Reads the object files of an oracle build (tools/build_oracle.py) with `nm`
and the `#include` lines of the pinned upstream sources, maps every upstream
unit to its owning package through tools/units.json, and lists each
cross-package reference that the declared dependencies (transitively) do not
cover, classified by the waves of the two packages:

  same wave            the referenced package runs in parallel
  later wave           the referenced package is scheduled after the referrer
  earlier, undeclared  merged earlier, but not named as a dependency

`calls` are references to non-static functions or data defined in another
package's unit; `includes` are headers of another package's unit, which may
only need types, constants or inline helpers. The report is evidence for the
integrator; docs/WORKPLAN.md records the findings and what was decided.
"""

import argparse
import collections
import json
import platform
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = ROOT / "upstream/libwebp"


def closure(packages, package):
    seen = set()
    stack = list(packages[package]["depends"])
    while stack:
        current = stack.pop()
        if current not in seen:
            seen.add(current)
            stack.extend(packages[current]["depends"])
    return seen


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--variant", default="release", help="oracle variant to read (default: release)")
    parser.add_argument("--json", action="store_true", help="print the findings as JSON")
    args = parser.parse_args()
    manifest = json.loads((ROOT / "tools/units.json").read_text())
    packages = manifest["packages"]
    owner = {path: entry["package"] for entry in manifest["modules"] for path in entry["upstream"]}
    objects = ROOT / "build/oracle" / args.variant / "libwebp/CMakeFiles"
    if not objects.is_dir():
        raise SystemExit(f"{objects} is missing; run python3 tools/build_oracle.py --variant {args.variant}")

    defined, referenced = {}, collections.defaultdict(set)
    for path in sorted(objects.glob("*.dir/**/*.c.o")):
        unit = str(path.relative_to(objects)).split(".dir/", 1)[1][:-2]
        if unit not in owner:
            continue
        symbols = subprocess.run(["nm", "-g", str(path)], check=True, stdout=subprocess.PIPE,
                                 text=True).stdout
        for line in symbols.splitlines():
            fields = line.split()
            if len(fields) == 2 and fields[0] == "U":
                referenced[unit].add(fields[1])
            elif len(fields) == 3 and fields[1] in "TDBSRC":
                defined[fields[2]] = unit

    edges = collections.defaultdict(lambda: collections.defaultdict(lambda: {"calls": set(), "includes": set()}))
    for unit, symbols in referenced.items():
        for symbol in symbols:
            target = defined.get(symbol)
            if target and owner[target] != owner[unit]:
                # Mach-O prefixes C symbols with an underscore; ELF does not.
                name = symbol[1:] if platform.system() == "Darwin" else symbol
                edges[owner[unit]][owner[target]]["calls"].add(f"{unit} -> {name} ({target})")
    for unit in owner:
        for header in re.findall(r'#include "((?:src|sharpyuv)/[^"]+)"', (UPSTREAM / unit).read_text()):
            if header in owner and owner[header] != owner[unit]:
                edges[owner[unit]][owner[header]]["includes"].add(f"{unit} -> {header}")

    findings = []
    for source in sorted(edges, key=lambda name: int(name[2:])):
        allowed = closure(packages, source)
        for target in sorted(edges[source], key=lambda name: int(name[2:])):
            if target in allowed:
                continue
            source_wave, target_wave = packages[source]["wave"], packages[target]["wave"]
            kind = ("same wave" if source_wave == target_wave else
                    "later wave" if target_wave > source_wave else "earlier, undeclared")
            findings.append({"package": source, "references": target, "kind": kind,
                             "calls": sorted(edges[source][target]["calls"]),
                             "includes": sorted(edges[source][target]["includes"])})
    if args.json:
        print(json.dumps(findings, indent=2))
        return
    for finding in findings:
        print(f"{finding['package']} -> {finding['references']} [{finding['kind']}]")
        for call in finding["calls"]:
            print(f"  call     {call}")
        for include in finding["includes"]:
            print(f"  include  {include}")
    print(f"{len(findings)} undeclared package reference(s)")


if __name__ == "__main__":
    main()
