#!/usr/bin/env python3
"""Compare the declared package dependencies with libwebp's actual ones.

Reads the object files of an oracle build (tools/build_oracle.py) with `nm`
and the `#include` lines of the pinned upstream sources, maps every upstream
unit to its owning package through tools/units.json, and lists each
cross-package reference that the declared dependencies (transitively) do not
cover, classified by the sub-waves of the two packages:

  same wave            the referenced package runs in the same sub-wave
  later wave           the referenced package is scheduled after the referrer
  earlier, undeclared  scheduled earlier, but not named as a dependency

`calls` are references to non-static functions or data defined in another
package's unit; `includes` are headers of another package's unit, which may
only need types, constants or inline helpers.

The function- and type-level exceptions of tools/units.json count: a function
listed there belongs to the exception's package, and a unit's references and
includes are accepted when the file's owner or one of the unit's exception
packages may make them. Object files do not say which function inside a unit
makes a reference, so that acceptance is per unit, not per function. Includes
that use nothing of the other package's code (macros, target constants or
nothing at all) are listed under "reviewed_includes" with a reason.

With --check the script exits with status 1 when any reference is left;
tools/check.py --oracle runs it that way.
"""

import argparse
import collections
import json
from pathlib import Path
import platform
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = ROOT / "upstream/libwebp"


def wave_key(wave):
    match = re.fullmatch(r"(\d+)([a-z]?)", wave)
    return int(match[1]), match[2]


def package_key(name):
    return int(name[2:])


def closure(packages, package):
    seen = {package}
    stack = list(packages[package]["depends"])
    while stack:
        current = stack.pop()
        if current not in seen:
            seen.add(current)
            stack.extend(packages[current]["depends"])
    return seen


def analyze(manifest, variant):
    packages = manifest["packages"]
    owner = {path: entry["package"] for entry in manifest["modules"] for path in entry["upstream"]}
    unit_packages = collections.defaultdict(set)
    function_owner = {}
    for exception in manifest.get("exceptions", []):
        unit_packages[exception["upstream"]].add(exception["package"])
        for name in exception.get("functions", []):
            function_owner[(exception["upstream"], name)] = exception["package"]
    reviewed = {(entry["unit"], entry["header"]) for entry in manifest.get("reviewed_includes", [])}

    def allowed(unit):
        result = set()
        for package in {owner[unit]} | unit_packages[unit]:
            result |= closure(packages, package)
        return result

    objects = ROOT / "build/oracle" / variant / "libwebp/CMakeFiles"
    if not objects.is_dir():
        raise SystemExit(f"{objects} is missing; run python3 tools/build_oracle.py --variant {variant}")
    darwin = platform.system() == "Darwin"
    defined, referenced = {}, collections.defaultdict(set)
    for path in sorted(objects.glob("*.dir/**/*.c.o")):
        unit = str(path.relative_to(objects)).split(".dir/", 1)[1][:-2]
        if unit not in owner:
            continue
        symbols = subprocess.run(["nm", "-g", str(path)], check=True, stdout=subprocess.PIPE,
                                 text=True).stdout
        for line in symbols.splitlines():
            fields = line.split()
            # Mach-O prefixes C symbols with an underscore; ELF does not.
            name = fields[-1][1:] if darwin and fields and fields[-1].startswith("_") else fields[-1] if fields else ""
            if len(fields) == 2 and fields[0] == "U":
                referenced[unit].add(name)
            elif len(fields) == 3 and fields[1] in "TDBSRCWV":
                defined[name] = unit

    edges = collections.defaultdict(lambda: collections.defaultdict(lambda: {"calls": set(), "includes": set()}))
    for unit, names in referenced.items():
        permitted = allowed(unit)
        for name in names:
            target_unit = defined.get(name)
            if target_unit is None:
                continue
            target = function_owner.get((target_unit, name), owner[target_unit])
            if target not in permitted:
                edges[owner[unit]][target]["calls"].add(f"{unit} -> {name} ({target_unit})")
    for unit in owner:
        permitted = allowed(unit)
        for header in re.findall(r'#include "((?:src|sharpyuv)/[^"]+)"', (UPSTREAM / unit).read_text()):
            if header not in owner or (unit, header) in reviewed:
                continue
            if not ({owner[header]} | unit_packages[header]) & permitted:
                edges[owner[unit]][owner[header]]["includes"].add(f"{unit} -> {header}")

    findings = []
    for source in sorted(edges, key=package_key):
        for target in sorted(edges[source], key=package_key):
            source_wave = wave_key(packages[source]["wave"])
            target_wave = wave_key(packages[target]["wave"])
            kind = ("same wave" if source_wave == target_wave else
                    "later wave" if target_wave > source_wave else "earlier, undeclared")
            findings.append({"package": source, "references": target, "kind": kind,
                             "calls": sorted(edges[source][target]["calls"]),
                             "includes": sorted(edges[source][target]["includes"])})
    return findings


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--variant", default="release", help="oracle variant to read (default: release)")
    parser.add_argument("--json", action="store_true", help="print the findings as JSON")
    parser.add_argument("--check", action="store_true", help="exit with status 1 if any reference is left")
    args = parser.parse_args()
    manifest = json.loads((ROOT / "tools/units.json").read_text())
    findings = analyze(manifest, args.variant)
    if args.json:
        print(json.dumps(findings, indent=2))
    else:
        for finding in findings:
            print(f"{finding['package']} -> {finding['references']} [{finding['kind']}]")
            for call in finding["calls"]:
                print(f"  call     {call}")
            for include in finding["includes"]:
                print(f"  include  {include}")
        print(f"{len(findings)} undeclared package reference(s)", flush=True)
    if args.check and findings:
        raise SystemExit("Declared package dependencies do not cover the references above; "
                         "see tools/units.json and docs/adr/0009-work-packages-and-integration.md")


if __name__ == "__main__":
    main()
