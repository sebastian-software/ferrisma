#!/usr/bin/env python3
"""Verify the pool inventory tools/pools.json against the pinned upstream and src/.

ADR 0005 ("Pools") makes free_pool reset every field of the decompression
object that upstream fills from the pool being freed. tools/pools.json lists
every allocation call in the upstream files that tools/units.json maps to a
module: the memory-manager method or table allocator it calls, its pool, what
it fills, and the field of the object whose reset releases the storage (a
module slot holds its module's sub-allocations; the memory manager holds the
virtual arrays; `alloc_sarray` and `alloc_barray` fill the array they return).
It also lists, per pool, the object fields those allocations fill and the work
package that adds each field. This script checks that:

- every allocation call upstream is listed once, at its line, with its pool,
  and every listed call exists;
- every object field an in-scope allocation names is listed for its pool;
- every listed field that jpeg_decompress_struct (src/jpeglib.rs) already has
  is reset in release_pool_fields (src/jmemmgr.rs), under its pool, and every
  field it does not have names a later work package.
"""

import json
from pathlib import Path
import re
import sys

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = ROOT / "upstream/libjpeg-turbo"
MANIFEST = ROOT / "tools/pools.json"
UNITS = ROOT / "tools/units.json"

CALL = re.compile(
    r"\(\*cinfo->mem->(alloc_small|alloc_large|alloc_sarray|alloc_barray|"
    r"request_virt_barray|request_virt_sarray)\)"
    r"|\b(jpeg_alloc_quant_table|jpeg_alloc_huff_table)\s*\("
    r"|^\s+[^/*]*\b(alloc_small|alloc_large|alloc_sarray|alloc_barray)\s*\(cinfo")
DEFINITION = re.compile(r"^(METHODDEF|LOCAL|GLOBAL)\(")


def upstream_calls():
    """(site, call, pool) of every allocation call in the mapped C files."""
    units = json.loads(UNITS.read_text())
    calls = []
    for entry in units["modules"]:
        for path in entry["upstream"]:
            if not path.endswith(".c"):
                continue
            lines = (UPSTREAM / path).read_text().splitlines()
            for number, line in enumerate(lines, 1):
                previous = lines[number - 2] if number > 1 else ""
                if DEFINITION.match(previous):
                    continue  # the definition of jpeg_alloc_*_table itself
                match = CALL.search(line)
                if not match:
                    continue
                name = next(group for group in match.groups() if group)
                pool = re.search(r"JPOOL_(PERMANENT|IMAGE)", " ".join(lines[number - 1:number + 1]))
                calls.append((f"{path}:{number}", name, pool.group(0) if pool else None))
    return calls


def struct_fields():
    text = (ROOT / "src/jpeglib.rs").read_text()
    body = text[text.index("pub struct jpeg_decompress_struct<'src> {"):]
    body = body[:body.index("\n}\n")]
    return set(re.findall(r"^\s+pub (\w+):", body, re.M))


def released_fields():
    """The fields release_pool_fields resets, per pool."""
    text = (ROOT / "src/jmemmgr.rs").read_text()
    body = text[text.index("pub fn release_pool_fields("):]
    body = body[:body.index("\n}\n")]
    released = {}
    for pool, block in re.findall(r"if pool_id == (JPOOL_\w+) \{(.*?)\n    \}", body, re.S):
        released[pool] = set(re.findall(r"cinfo\.(\w+) =", block))
    return released


def verify():
    manifest = json.loads(MANIFEST.read_text())
    problems = []
    listed = {}
    for entry in manifest["allocations"]:
        key = entry["site"]
        if key in listed:
            problems.append(f"{key}: listed twice")
        listed[key] = entry
    found = upstream_calls()
    for site, call, pool in found:
        entry = listed.get(site)
        if entry is None:
            problems.append(f"{site}: {call} is not listed in tools/pools.json")
            continue
        if entry["call"] != call:
            problems.append(f"{site}: listed as {entry['call']}, upstream calls {call}")
        if pool is not None and entry["pool"] != pool:
            problems.append(f"{site}: listed in {entry['pool']}, upstream uses {pool}")
    for site in sorted(set(listed) - {site for site, _, _ in found}):
        problems.append(f"{site}: listed, but upstream has no allocation call there")

    fields = manifest["fields"]
    for entry in manifest["allocations"]:
        field = entry["field"]
        if entry.get("out_of_scope") or not field.startswith("cinfo."):
            continue
        name = field[len("cinfo."):]
        if name not in fields.get(entry["pool"], {}):
            problems.append(f"{entry['site']}: field {name} is not listed for {entry['pool']}")

    present = struct_fields()
    released = released_fields()
    for pool, entries in fields.items():
        for name, package in entries.items():
            if name in present:
                if name not in released.get(pool, set()):
                    problems.append(f"release_pool_fields does not reset {name} for {pool}")
            elif package == "1":
                problems.append(f"{name}: work package 1, but jpeg_decompress_struct lacks it")
    for pool, names in released.items():
        for name in sorted(names - set(fields.get(pool, {}))):
            problems.append(f"release_pool_fields resets {name} for {pool}, "
                            "which tools/pools.json does not list")
    return problems, len(found)


def main():
    problems, count = verify()
    for problem in problems:
        print(problem)
    if problems:
        raise SystemExit(f"{len(problems)} pool inventory problem(s)")
    print(f"Pool inventory verified: {count} allocation calls", flush=True)


if __name__ == "__main__":
    sys.exit(main())
