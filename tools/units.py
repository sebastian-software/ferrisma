#!/usr/bin/env python3
"""Verify the module skeleton against tools/units.json and the pinned upstream.

tools/units.json maps every file under upstream/libjpeg-turbo/src and under
the SIMD directories it lists in `expanded_directories` to a Rust module and
its owning work package, or names the category of reason it is not
translated. A module is named after an upstream file it translates, with `-`
as `_` (`jidctint-neon.c` is `jidctint_neon.rs`). This script checks that:

- every upstream file (and every subdirectory) is mapped or excluded, once;
- every mapped module file exists and starts with its provenance header: for
  each upstream file it translates, a line naming the file and that file's
  leading comment block, verbatim as `//` lines, followed by a module doc comment that names
  the files, the owning work package and any items left untranslated;
- every module of `own_modules`, which translates no upstream file, exists
  and starts with a module doc comment;
- src/lib.rs declares exactly the mapped modules and `own_modules`, sorted,
  and denies unsafe code, and src/ holds no other module file.

Work packages append their translation below the header and never edit it.
--create-missing writes the header-only file for a manifest entry whose module
file does not exist yet; it never overwrites a file. --print-declarations
prints the module declarations that src/lib.rs must contain.
"""

import argparse
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = ROOT / "upstream/libjpeg-turbo"
MANIFEST = ROOT / "tools/units.json"


def load():
    return json.loads(MANIFEST.read_text())


def leading_comment(path):
    """Return the upstream file's leading /* ... */ block as `//` lines.

    Each line of the block, delimiters included, follows `// ` unchanged.
    Rust line comments keep the text byte for byte; rustfmt would re-indent a
    block comment whose lines do not all start with ` *`.
    """
    text = (UPSTREAM / path).read_text()
    if not text.startswith("/*") or "*/" not in text:
        raise SystemExit(f"{path}: expected a leading /* ... */ comment block")
    block = text[:text.index("*/") + 2]
    return "\n".join(f"// {line}".rstrip() if line.strip() else "//"
                     for line in block.splitlines())


def code_list(names):
    quoted = [f"`{name}`" for name in names]
    if len(quoted) == 1:
        return quoted[0]
    return ", ".join(quoted[:-1]) + " and " + quoted[-1]


def module_header(manifest, entry):
    version = manifest["upstream_version"]
    lines = []
    for path in entry["upstream"]:
        lines.append(f"// {version}, {path}:")
        lines.append("//")
        lines.append(leading_comment(path))
    package = entry["package"]
    lines.append(f"//! Translation of {code_list(entry['upstream'])} from {version}.")
    lines.append("//!")
    lines.append(f"//! Work package: {package} ({manifest['packages'][package]}).")
    skipped = entry.get("not_translated")
    if skipped:
        lines.append("//!")
        lines.append(f"//! Not translated: {code_list(skipped['items'])}, because "
                     f"{skipped['reason']}.")
    return "\n".join(lines) + "\n"


def declarations(manifest):
    entries = manifest["modules"] + manifest["own_modules"]
    return [f"pub mod {Path(entry['module']).stem};"
            for entry in sorted(entries, key=lambda entry: entry["module"])]


def stem(path):
    """The module name that an upstream file gives: its name up to the first
    dot, with `-` as `_`."""
    return Path(path).name.split(".")[0].replace("-", "_")


def upstream_entries(manifest):
    """Every file and subdirectory directly under src/ and under each expanded
    directory, except the expanded directories themselves."""
    expanded = manifest["expanded_directories"]
    entries = set()
    for directory in ["src/", *expanded]:
        for path in (UPSTREAM / directory).iterdir():
            relative = str(path.relative_to(UPSTREAM))
            entries.add(relative + "/" if path.is_dir() else relative)
    return entries - set(expanded)


def verify(manifest):
    problems = []
    if not (UPSTREAM / "src/jpeglib.h").is_file():
        return ["upstream/libjpeg-turbo is missing; run git submodule update --init"]
    covered = [path for entry in manifest["modules"] for path in entry["upstream"]]
    covered += [entry["upstream"] for entry in manifest["excluded"]]
    upstream = upstream_entries(manifest)
    for path in sorted(upstream - set(covered)):
        problems.append(f"upstream file without module or exclusion: {path}")
    for path in sorted(set(covered) - upstream):
        problems.append(f"tools/units.json names a missing upstream file: {path}")
    for path in sorted({path for path in covered if covered.count(path) > 1}):
        problems.append(f"upstream file assigned twice: {path}")
    for entry in manifest["excluded"]:
        if entry["category"] not in manifest["categories"]:
            problems.append(f"{entry['upstream']}: unknown category {entry['category']}")

    expected_files = {ROOT / "src/lib.rs"}
    names = [Path(entry["module"]).stem for entry in manifest["modules"]]
    for entry in manifest["modules"]:
        module = ROOT / entry["module"]
        expected_files.add(module)
        if Path(entry["module"]).parent != Path("src"):
            problems.append(f"{entry['module']}: modules live directly in src/")
        if entry["package"] not in manifest["packages"]:
            problems.append(f"{entry['module']}: unknown package {entry['package']}")
        stems = {stem(path) for path in entry["upstream"]}
        if Path(entry["module"]).stem not in stems:
            problems.append(f"{entry['module']}: not named after an upstream file it translates")
        skipped = entry.get("not_translated")
        if skipped:
            source = "".join((UPSTREAM / path).read_text() for path in entry["upstream"])
            for item in skipped["items"]:
                if not re.search(rf"\b{re.escape(item)}\b", source):
                    problems.append(f"{entry['module']}: {item} does not occur upstream")
        if not module.is_file():
            problems.append(f"missing module file: {entry['module']}")
        elif not module.read_text().startswith(module_header(manifest, entry)):
            problems.append(f"{entry['module']}: provenance header differs from tools/units.json")

    upstream_stems = {stem(path) for path in upstream}
    for entry in manifest["own_modules"]:
        module = ROOT / entry["module"]
        expected_files.add(module)
        names.append(module.stem)
        if Path(entry["module"]).parent != Path("src"):
            problems.append(f"{entry['module']}: modules live directly in src/")
        if entry["package"] not in manifest["packages"]:
            problems.append(f"{entry['module']}: unknown package {entry['package']}")
        if module.stem in upstream_stems:
            problems.append(f"{entry['module']}: named after an upstream file it does not translate")
        if not module.is_file():
            problems.append(f"missing module file: {entry['module']}")
        elif not module.read_text().startswith("//! "):
            problems.append(f"{entry['module']}: does not start with a module doc comment")
    for name in sorted({name for name in names if names.count(name) > 1}):
        problems.append(f"module declared twice: {name}")

    for path in sorted((ROOT / "src").rglob("*.rs")):
        if path not in expected_files:
            problems.append(f"module file outside tools/units.json: {path.relative_to(ROOT)}")
    lib = (ROOT / "src/lib.rs").read_text()
    actual = [line for line in lib.splitlines() if re.match(r"^\s*(pub(\(\w+\))? )?mod \w+", line)]
    if actual != declarations(manifest):
        problems.append("src/lib.rs: module declarations differ from tools/units.json "
                        "(see tools/units.py --print-declarations)")
    if "#![deny(unsafe_code)]" not in lib:
        problems.append("src/lib.rs: #![deny(unsafe_code)] is missing")
    return problems


def create_missing(manifest):
    for entry in manifest["modules"]:
        module = ROOT / entry["module"]
        if not module.exists():
            module.write_text(module_header(manifest, entry))
            print(f"created {entry['module']}")


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--create-missing", action="store_true",
                        help="write header-only module files that do not exist yet")
    parser.add_argument("--print-declarations", action="store_true",
                        help="print the module declarations of src/lib.rs")
    args = parser.parse_args()
    manifest = load()
    if args.print_declarations:
        print("\n".join(declarations(manifest)))
        return
    if args.create_missing:
        create_missing(manifest)
    problems = verify(manifest)
    for problem in problems:
        print(problem)
    if problems:
        raise SystemExit(f"{len(problems)} module skeleton problem(s)")
    print(f"Module skeleton verified: {len(manifest['modules'])} translated and "
          f"{len(manifest['own_modules'])} own modules, "
          f"{len(manifest['excluded'])} excluded upstream entries", flush=True)


if __name__ == "__main__":
    main()
