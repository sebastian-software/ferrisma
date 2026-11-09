#!/usr/bin/env python3
"""Verify the module skeleton against tools/units.json and the pinned upstream.

Every Rust module file starts with a fixed provenance header: the leading
comment block of each upstream file it translates, copied verbatim, followed by
a module doc comment that names those files and the owning work package. Work
packages append their translation below that header and never edit it. The safe
API modules under src/api translate no upstream file; their header names the
part of the translation they wrap and the owning work package. A manifest entry
with an `unsafe` field (the SIMD modules, ADR 0008) gets `#[allow(unsafe_code)]`
with that reason and the ABI module's unsafe lints on its `pub mod` line.

--create-missing writes the header-only file for a manifest entry whose module
file does not exist yet. It never overwrites an existing file.
"""

import argparse
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = ROOT / "upstream/libwebp"
MANIFEST = ROOT / "tools/units.json"
VERSION = "libwebp v1.6.0"
TOP_LEVEL = ["webp", "utils", "dsp", "dec", "enc", "demux", "mux", "sharpyuv", "abi", "api"]


def load():
    return json.loads(MANIFEST.read_text())


def leading_comment(path):
    """Return the upstream file's leading `//` comment block, unchanged."""
    lines = []
    for line in (UPSTREAM / path).read_text().splitlines():
        if not line.startswith("//"):
            break
        lines.append(line)
    if not lines or not lines[0].startswith("// Copyright"):
        raise SystemExit(f"{path}: expected a leading copyright comment")
    return lines


def code_list(paths):
    quoted = [f"`{path}`" for path in paths]
    if len(quoted) == 1:
        return quoted[0]
    return ", ".join(quoted[:-1]) + " and " + quoted[-1]


def notices(paths):
    lines = []
    for path in paths:
        lines.append(f"// {VERSION}, {path}:")
        lines.append("//")
        lines.extend(leading_comment(path))
        lines.append("")
    return lines


def item_list(names):
    quoted = [f"`{name}`" for name in names]
    if len(quoted) == 1:
        return quoted[0]
    return ", ".join(quoted[:-1]) + " and " + quoted[-1]


def owner(manifest, entry):
    """Name the owning package and, one per line, the exceptions inside the module."""
    lines = [f"//! Work package: {entry['package']} "
             f"({manifest['packages'][entry['package']]['name']})"
             + (f"; {entry['note']}." if "note" in entry else ".")]
    for exception in manifest.get("exceptions", []):
        if exception["module"] == entry["module"]:
            items = exception.get("functions", []) + exception.get("types", [])
            lines.append("//!")
            lines.append(f"//! Exception: {exception['package']} translates {item_list(items)}"
                         f" from `{exception['upstream']}`, because {exception['reason']}.")
    return lines


def module_header(manifest, entry):
    lines = notices(entry["upstream"])
    lines.append(f"//! Translation of {code_list(entry['upstream'])} from {VERSION}.")
    lines.append("//!")
    lines.extend(owner(manifest, entry))
    return "\n".join(lines) + "\n"


def abi_header(manifest, entry):
    lines = notices([entry["exports"]])
    lines.append(f"//! C ABI export of `{entry['exports']}` from {VERSION}.")
    lines.append("//!")
    lines.append("//! Mirrors the header's structs with `repr(C)` and converts them to the")
    lines.append("//! safe representations at the boundary; see")
    lines.append("//! `docs/adr/0004-c-abi-boundary.md`.")
    lines.append("//!")
    lines.extend(owner(manifest, entry))
    return "\n".join(lines) + "\n"


def api_header(manifest, entry):
    lines = [f"//! Safe Rust API over the {entry['subject']} of the translation; no upstream unit.",
             "//!",
             "//! Wraps the translated types and functions in validated Rust types and adds",
             "//! no codec behavior of its own; see `docs/adr/`.",
             "//!"]
    lines.extend(owner(manifest, entry))
    return "\n".join(lines) + "\n"


def entries(manifest):
    for entry in manifest["modules"]:
        yield entry, module_header(manifest, entry)
    for entry in manifest["abi"]:
        yield entry, abi_header(manifest, entry)
    for entry in manifest.get("api", []):
        yield entry, api_header(manifest, entry)


def declarations(manifest, directory):
    """Return the expected `mod` declarations of src/<directory>/mod.rs."""
    found = []
    for entry, _ in entries(manifest):
        path = Path(entry["module"])
        if path.parent == Path("src") / directory:
            found.append((path.stem, entry.get("cfg"), entry.get("unsafe")))
    lines = []
    for name, cfg, unsafe in sorted(found):
        if cfg:
            lines.append(f"#[cfg({cfg})]")
        if unsafe:
            # ADR 0008: the SIMD modules may contain unsafe code of two kinds,
            # under the ABI module's lints.
            lines.append(f'#[allow(unsafe_code, reason = "{unsafe}")]')
            lines.append("#[deny(clippy::undocumented_unsafe_blocks)]")
            lines.append("#[deny(clippy::multiple_unsafe_ops_per_block)]")
        if name == directory:
            lines.append('#[expect(clippy::module_inception, reason = "keeps the upstream file name")]')
        lines.append(f"pub mod {name};")
    return lines


def verify(manifest):
    problems = []
    upstream = {str(p.relative_to(UPSTREAM)) for d in ("src", "sharpyuv")
                for p in (UPSTREAM / d).rglob("*") if p.suffix in (".c", ".h")}
    covered = [f for m in manifest["modules"] for f in m["upstream"]]
    covered += [e["upstream"] for e in manifest["excluded"]]
    for path in sorted(upstream - set(covered)):
        problems.append(f"upstream unit without module or exclusion: {path}")
    for path in sorted(set(covered) - upstream):
        problems.append(f"manifest names a missing upstream file: {path}")
    for path in sorted({f for f in covered if covered.count(f) > 1}):
        problems.append(f"upstream unit assigned twice: {path}")
    expected_files = set()
    for entry, header in entries(manifest):
        module = ROOT / entry["module"]
        expected_files.add(module)
        if entry["package"] not in manifest["packages"]:
            problems.append(f"{entry['module']}: unknown package {entry['package']}")
        if not module.is_file():
            problems.append(f"missing module file: {entry['module']}")
        elif not module.read_text().startswith(header):
            problems.append(f"{entry['module']}: provenance header differs from tools/units.json")
    modules = {entry["module"]: entry for entry in manifest["modules"]}
    for exception in manifest.get("exceptions", []):
        target = modules.get(exception["module"])
        if target is None:
            problems.append(f"exception names an unknown module: {exception['module']}")
        elif exception["upstream"] not in target["upstream"]:
            problems.append(f"exception names {exception['upstream']}, which "
                            f"{exception['module']} does not translate")
        if exception["package"] not in manifest["packages"]:
            problems.append(f"exception names an unknown package: {exception['package']}")
        if not exception.get("functions") and not exception.get("types"):
            problems.append(f"exception for {exception['module']} lists no functions or types")
        source = (UPSTREAM / exception["upstream"]).read_text()
        for name in exception.get("functions", []) + exception.get("types", []):
            if not re.search(rf"\b{re.escape(name)}\b", source):
                problems.append(f"{name} does not occur in {exception['upstream']}")
    for entry in manifest.get("reviewed_includes", []):
        unit = UPSTREAM / entry["unit"]
        if not unit.is_file() or f'#include "{entry["header"]}"' not in unit.read_text():
            problems.append(f"reviewed include not found: {entry['unit']} -> {entry['header']}")
    for directory in TOP_LEVEL:
        mod_rs = ROOT / "src" / directory / "mod.rs"
        expected_files.add(mod_rs)
        if not mod_rs.is_file():
            problems.append(f"missing module file: src/{directory}/mod.rs")
            continue
        actual = [line for line in mod_rs.read_text().splitlines()
                  if line.startswith(("pub mod ", "mod ", "#[cfg(", "#[expect(", "#[allow(", "#[deny("))]
        if actual != declarations(manifest, directory):
            problems.append(f"src/{directory}/mod.rs: declarations differ from tools/units.json")
    for path in sorted((ROOT / "src").rglob("*.rs")):
        if path != ROOT / "src/lib.rs" and path not in expected_files:
            problems.append(f"module file outside tools/units.json: {path.relative_to(ROOT)}")
    lib = (ROOT / "src/lib.rs").read_text()
    mods = re.findall(r"^pub mod (\w+);$", lib, re.M)
    if sorted(mods) != sorted(TOP_LEVEL):
        problems.append("src/lib.rs: top-level modules differ from the fixed layout")
    if "#![deny(unsafe_code)]" not in lib:
        problems.append("src/lib.rs: #![deny(unsafe_code)] is missing")
    return problems


def create_missing(manifest):
    for entry, header in entries(manifest):
        module = ROOT / entry["module"]
        if not module.exists():
            module.parent.mkdir(parents=True, exist_ok=True)
            module.write_text(header)
            print(f"created {entry['module']}")


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--create-missing", action="store_true",
                        help="write header-only module files that do not exist yet")
    parser.add_argument("--print-declarations", metavar="DIR",
                        help="print the expected declarations of src/DIR/mod.rs")
    args = parser.parse_args()
    manifest = load()
    if args.print_declarations:
        print("\n".join(declarations(manifest, args.print_declarations)))
        return
    if args.create_missing:
        create_missing(manifest)
    problems = verify(manifest)
    for problem in problems:
        print(problem)
    if problems:
        raise SystemExit(f"{len(problems)} module skeleton problem(s)")
    count = len(manifest["modules"]) + len(manifest["abi"]) + len(manifest.get("api", []))
    print(f"Module skeleton verified: {count} module files, "
          f"{len(manifest['excluded'])} excluded upstream units", flush=True)


if __name__ == "__main__":
    main()
