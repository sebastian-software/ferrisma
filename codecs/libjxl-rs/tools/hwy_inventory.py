#!/usr/bin/env python3
"""Inventory the pinned original's Highway exports for the SIMD plan (ADR 0004).

For every lib/ source with HWY_EXPORT, print its exports, the size of its
HWY_NAMESPACE code (including the lib/ -inl.h headers it includes,
transitively) and the Highway operations it uses. With --ops, print how many
export files use each operation. Heuristic text scan; it reads the pinned
sources only.
"""
import argparse
import collections
import re
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = ROOT / "upstream/libjxl"
HWY = UPSTREAM / "third_party/highway/hwy"


def highway_api():
    names = {"Set", "Zero", "Load", "LoadU", "Store", "StoreU", "Lanes", "Undefined", "Iota",
             "Stream", "FirstN", "MaxLanes"}
    for name in ("arm_neon-inl.h", "x86_256-inl.h", "x86_128-inl.h", "generic_ops-inl.h",
                 "shared-inl.h"):
        text = (HWY / "ops" / name).read_text()
        names.update(re.findall(r"HWY_API\s+[^;{(]*?\b([A-Z][A-Za-z0-9]+)\s*\(", text))
    return names


def includes(path, seen):
    if path in seen or not path.exists():
        return
    seen.add(path)
    for header in re.findall(r'#include "(lib/[^"]+-inl\.h)"', path.read_text(errors="ignore")):
        includes(UPSTREAM / header, seen)


def namespace_lines(path):
    text = path.read_text(errors="ignore")
    if path.name.endswith("-inl.h"):
        return text.count("\n")
    total = 0
    for block in re.findall(r"HWY_BEFORE_NAMESPACE\(\);(.*?)HWY_AFTER_NAMESPACE\(\);", text, re.S):
        total += block.count("\n")
    return total


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ops", action="store_true", help="Print operation usage counts")
    args = parser.parse_args()
    api = highway_api()
    usage = collections.Counter()
    exports = 0
    for source in sorted((UPSTREAM / "lib").rglob("*.cc")):
        names = re.findall(r"HWY_EXPORT\((\w+)\)", source.read_text(errors="ignore"))
        if not names:
            continue
        exports += len(names)
        files = set()
        includes(source, files)
        used = set()
        for path in files:
            text = path.read_text(errors="ignore")
            used.update(name for name in api if re.search(r"\b%s\s*[<(]" % name, text))
        usage.update(used)
        size = sum(namespace_lines(path) for path in files)
        if not args.ops:
            print(f"{source.relative_to(UPSTREAM)}\t{len(names)}\t{size}\t{','.join(names)}"
                  f"\t{' '.join(sorted(used))}")
    if args.ops:
        for name, count in sorted(usage.items(), key=lambda item: (-item[1], item[0])):
            print(f"{name}\t{count}")
    print(f"# {exports} HWY_EXPORTs")


if __name__ == "__main__":
    main()
