#!/usr/bin/env python3
"""Map floating-point contraction per encoder unit (issue #102).

The encoder plan requires a contraction map for every ported unit before a
step is accepted (docs/ENCODER_EXPANSION.md, "Cross-cutting risks"). The
per-map audit `encoder_transforms_contraction.py` checks one recorded map
against the oracle objects. This tool takes the inventory view across all
encoder translation units at once:

* Reference build flags. For each unit it extracts, from the oracle's
  compile_commands.json, the options that decide contraction and lowering:
  optimization level, any -ffp-contract/-ffp-model/-ffast-math override
  (absent means Clang's default -ffp-contract=on), vectorizer switches,
  HWY_COMPILE_ONLY_SCALAR, target and sanitizer options.
* Reference contraction sites. It reruns each unit's exact compile command
  to emit front-end IR (`-Xclang -disable-llvm-passes`, the method and code of
  encoder_transforms_contraction.py) and lists every llvm.fmuladd source site,
  grouped by the file the debug location names (inlined headers included).
  Only the front end decides contraction; optimized IR can fold or move
  sites. This step needs a configured oracle build directory, not a built one:
  `cmake -S tools/oracle -B <dir> -DCMAKE_BUILD_TYPE=Release` is enough.
* Recorded maps. Every tools/*_contraction.json is loaded. A unit whose IR
  sites equal its recorded sites is `recorded`; any difference is a
  `mismatch`; a unit with sites but no map is `unrecorded`; a unit whose
  front end emits no site is `none`.
* Rust side. For the unit's Rust translation (the same stem under src/jxl),
  it counts per-target contraction helpers (`mul_add_f32`, `mul_add_f64`),
  raw `.mul_add(` calls and `#[cfg(target_arch = "aarch64")]` items, and it
  checks that every recorded site's `rust` reference names an existing Rust
  file and, unless it says "unported", an identifier present in that file.
* Lexical candidates (heuristic). Without a build directory it still scans
  the upstream unit and its `-inl.h` for statements with a multiplication
  followed by an addition or subtraction. This is an upper-bound triage
  count for planning, never evidence; the IR map is the evidence.

With `--objects` it also counts fused instructions in already built objects
(FMADD/FMSUB/FNMADD/FMLA on AArch64; VFMADD and friends on x86-64), as the
per-map audit does. Nothing is compiled into an object, replaced or edited.

Usage:
    cmake -S tools/oracle -B /tmp/jxl-oracle -DCMAKE_BUILD_TYPE=Release
    python3 tools/contraction_inventory.py --build /tmp/jxl-oracle \\
        --output /tmp/contraction-inventory [--markdown docs/report.md] [--check]

`--check` exits with status 1 on a recorded-map mismatch or a broken Rust
reference, so a step's gate can run it. Without `--build` only the flags
column and IR sites are missing; the rest of the report is produced.
"""
import argparse
import json
from pathlib import Path
import platform
import re
import sys

sys.path.insert(0, str(Path(__file__).resolve().parent))
import encoder_transforms_contraction as audit  # noqa: E402

ROOT = Path(__file__).resolve().parent.parent
UPSTREAM = ROOT / "upstream/libjxl"
RUST = ROOT / "src/jxl"
UNIT = re.compile(r"^lib/jxl/(?:.*/)?(enc_[a-z0-9_]+|encode)\.cc$")
FLAG = re.compile(
    r"^(-O\d|-Os|-Oz|-ffp-contract=.*|-ffp-model=.*|-f(no-)?fast-math|-f(no-)?finite-math-only"
    r"|-f(no-)?(slp-)?vectorize|-f(no-)?unroll-loops|-m(arch|cpu|tune|fma).*|-arch|arm64|x86_64"
    r"|--target=.*|-target|-fsanitize=.*|-fmath-errno|-fno-math-errno"
    r"|-DHWY_COMPILE_ONLY_SCALAR.*|-DHWY_DISABLED_TARGETS.*)$")
MULADD = re.compile(r"[\w)\]]\s*\*\s*[\w(][^;]*?[^+\-=<>!&|*/]\s*[+-]\s*[\w(.]|"
                    r"[\w)\]]\s*[+-]\s*[\w(][^;]*?[\w)\]]\s*\*\s*[\w(]")
SKIP_LINE = re.compile(r"^\s*(//|#|\*|/\*)")


def compile_entries(build):
    if build is None:
        return {}
    units = {}
    for relative, directory, args in audit.entries(build):
        if UNIT.match(relative) or relative in recorded_sources():
            units.setdefault(relative, (directory, args))
    return units


_RECORDED = None


def recorded_maps():
    global _RECORDED
    if _RECORDED is None:
        _RECORDED = {}
        for path in sorted((ROOT / "tools").glob("*_contraction.json")):
            data = json.loads(path.read_text())
            for source, sites in data["sources"].items():
                _RECORDED[source] = (path.name, sites)
    return _RECORDED


def recorded_sources():
    return set(recorded_maps())


def upstream_units():
    units = set(recorded_sources())
    if UPSTREAM.is_dir():
        for path in (UPSTREAM / "lib/jxl").rglob("*.cc"):
            relative = path.relative_to(UPSTREAM).as_posix()
            if UNIT.match(relative) and not relative.endswith(("_test.cc", "_gbench.cc")):
                units.add(relative)
    return sorted(units)


def rust_file(relative):
    stem = relative[len("lib/jxl/"):-len(".cc")]
    path = RUST / f"{stem}.rs"
    return path if path.is_file() else None


def rust_counts(path):
    if path is None:
        return None
    text = path.read_text()
    return {
        "file": path.relative_to(ROOT).as_posix(),
        "mul_add_f32": len(re.findall(r"\bmul_add_f32\(", text)),
        "mul_add_f64": len(re.findall(r"\bmul_add_f64\(", text)),
        "raw_mul_add": len(re.findall(r"\.mul_add\(", text)),
        "aarch64_cfg_items": len(re.findall(r'#\[cfg\(target_arch = "aarch64"\)\]', text)),
    }


def check_rust_reference(site):
    """Return a problem string for a recorded site's `rust` field, if any."""
    reference = site.get("rust", "")
    if reference.startswith("unported"):
        return None
    match = re.match(r"([\w/]+\.rs)\b\s*(.*)", reference)
    if not match:
        return f"no Rust file named in {reference!r}"
    candidates = list(RUST.rglob(Path(match[1]).name))
    if not candidates:
        return f"{match[1]} does not exist"
    rest = match[2]
    if rest.startswith("unported"):
        return None
    ident = re.match(r"(?:\()?([A-Za-z_][A-Za-z0-9_]*)", rest)
    if ident and ident[1] not in ("constant", "the", "no"):
        if not any(re.search(rf"\b{re.escape(ident[1])}\b", c.read_text()) for c in candidates):
            return f"{ident[1]} not found in {match[1]}"
    return None


def lexical_candidates(relative):
    sources = [UPSTREAM / relative]
    inl = UPSTREAM / relative.replace(".cc", "-inl.h")
    if inl.is_file():
        sources.append(inl)
    count = 0
    for source in sources:
        if not source.is_file():
            continue
        statement = ""
        for line in source.read_text(errors="replace").splitlines():
            if SKIP_LINE.match(line):
                continue
            statement += " " + line.split("//")[0]
            if ";" in line or "{" in line or "}" in line:
                count += len(MULADD.findall(statement))
                statement = ""
    return count


def flags(args):
    selected = []
    for i, arg in enumerate(args):
        if FLAG.match(arg) and arg not in ("arm64", "x86_64"):
            selected.append(arg)
        elif arg in ("arm64", "x86_64") and i and args[i - 1] in ("-arch", "-target"):
            selected.append(arg)
    if not any(a.startswith(("-ffp-contract", "-ffp-model", "-ffast-math")) for a in selected):
        selected.append("(default -ffp-contract=on)")
    return selected


def inventory(build, output, objects, objdump):
    entries = compile_entries(build)
    maps = recorded_maps()
    target = audit.arch()
    problems = []
    report = {"target": target, "host": platform.platform(), "build": str(build) if build else None,
              "units": []}
    for relative in upstream_units():
        rust = rust_file(relative)
        unit = {"source": relative, "rust": rust_counts(rust),
                "lexical_candidates": lexical_candidates(relative)}
        recorded = maps.get(relative)
        unit["recorded_map"] = recorded[0] if recorded else None
        if recorded:
            unit["recorded_sites"] = len(recorded[1])
            unit["recorded_folded"] = sum(1 for s in recorded[1] if s.get("folded"))
            for site in recorded[1]:
                problem = check_rust_reference(site)
                if problem:
                    problems.append(f"{relative}:{site['line']}: {problem}")
        if relative in entries:
            directory, args = entries[relative]
            unit["flags"] = flags(args)
            ir_path = output / (relative.replace("/", "_") + ".ll")
            sites = sorted(audit.fmuladd_sites(audit.emit_ir(directory, args, ir_path)))
            ir_path.unlink()
            by_file = {}
            for file, line, column, kind in sites:
                by_file.setdefault(file, []).append([line, column, kind])
            unit["ir_sites"] = len(sites)
            unit["ir_sites_by_file"] = by_file
            if recorded:
                expected = sorted((s["file"], s["line"], s["column"], s["type"])
                                  for s in recorded[1])
                unit["status"] = "recorded" if sites == expected else "mismatch"
                if sites != expected:
                    problems.append(f"{relative}: IR sites differ from {recorded[0]}")
            else:
                unit["status"] = "unrecorded" if sites else "none"
            if objects:
                index = args.index("-o")
                obj = Path(directory) / args[index + 1]
                unit["fused_instructions"] = (audit.fused_instructions(objdump, target, obj)
                                              if obj.is_file() else None)
        else:
            unit["status"] = "recorded (not audited)" if recorded else "not in build"
        report["units"].append(unit)
    report["problems"] = problems
    return report


def markdown(report):
    lines = [
        "# Encoder floating-point contraction inventory",
        "",
        "Generated by `tools/contraction_inventory.py` (issue #102); do not edit by hand.",
        f"Target: `{report['target']}`. Host: `{report['host']}`.",
        "",
        "Status: `recorded` = front-end `llvm.fmuladd` sites equal the recorded map; "
        "`unrecorded` = sites without a map; `none` = no site; `mismatch` = map differs. "
        "IR sites include sites in headers inlined into the unit, also in unported functions. "
        "Rust columns count `mul_add_f32`/`mul_add_f64` helper calls, raw `.mul_add(` calls and "
        "`aarch64` cfg items in the same-stem Rust file. Lexical is a heuristic upper-bound "
        "count of `a*b+c` statement shapes in the C++ unit and its `-inl.h`, not evidence.",
        "",
    ]
    lines += [
        "| Unit | Rust | Status | IR sites | Recorded (folded) | Rust helpers / raw / aarch64 cfg | Lexical |",
        "| --- | --- | --- | ---: | ---: | --- | ---: |",
    ]
    for unit in report["units"]:
        rust = unit["rust"]
        helpers = (f"{rust['mul_add_f32'] + rust['mul_add_f64']} / {rust['raw_mul_add']} / "
                   f"{rust['aarch64_cfg_items']}") if rust else "–"
        recorded = (f"{unit['recorded_sites']} ({unit['recorded_folded']})"
                    if unit["recorded_map"] else "–")
        lines.append(
            f"| `{unit['source']}` | {'`' + Path(rust['file']).name + '`' if rust else 'unported'} "
            f"| {unit['status']} | {unit.get('ir_sites', '–')} | {recorded} | {helpers} "
            f"| {unit['lexical_candidates']} |")
    open_units = [u for u in report["units"] if u["rust"] and u["status"] in ("unrecorded", "mismatch")]
    if open_units:
        lines += ["", "Ported units without a matching map, by the file of each fmuladd debug "
                  "location (`line:column`):", ""]
        for unit in open_units:
            for file, sites in sorted(unit["ir_sites_by_file"].items()):
                where = ", ".join(f"{line}:{column}" for line, column, _ in sites)
                lines.append(f"- `{unit['source']}` → `{file}`: {where}")
    flags_seen = sorted({" ".join(u["flags"]) for u in report["units"] if "flags" in u})
    lines += ["", "Reference flags (deciding contraction and lowering) per distinct set:", ""]
    lines += [f"- `{f}`" for f in flags_seen]
    if report["problems"]:
        lines += ["", "Problems:", ""] + [f"- {p}" for p in report["problems"]]
    return "\n".join(lines) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--build", type=Path,
                        help="Configured oracle build directory with compile_commands.json")
    parser.add_argument("--output", type=Path, required=True,
                        help="New directory for report.json and scratch IR")
    parser.add_argument("--markdown", type=Path, help="Also write the Markdown table here")
    parser.add_argument("--objects", action="store_true",
                        help="Count fused instructions in already built objects")
    parser.add_argument("--objdump", default="objdump")
    parser.add_argument("--check", action="store_true",
                        help="Exit 1 on a recorded-map mismatch or broken Rust reference")
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    build = args.build.resolve() if args.build else None
    if build and not (build / "compile_commands.json").is_file():
        raise SystemExit(f"{build}: no compile_commands.json; configure tools/oracle first")
    report = inventory(build, output, args.objects, args.objdump)
    (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    table = markdown(report)
    if args.markdown:
        args.markdown.write_text(table)
    counts = {}
    for unit in report["units"]:
        counts[unit["status"]] = counts.get(unit["status"], 0) + 1
    print(f"{len(report['units'])} units: " + ", ".join(f"{v} {k}" for k, v in sorted(counts.items())))
    for problem in report["problems"]:
        print(f"problem: {problem}")
    print(output / "report.json")
    if args.check and report["problems"]:
        raise SystemExit(1)


if __name__ == "__main__":
    main()
