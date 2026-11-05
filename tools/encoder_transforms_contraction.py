#!/usr/bin/env python3
"""Audit the floating-point contraction map of the encoder transform units.

Clang's default -ffp-contract=on fuses a*b+c within one source expression.
The front end records each such site as an llvm.fmuladd call; AArch64 lowers
every one to a fused instruction, while baseline x86-64 has no FMA and lowers
it to a separate multiply and add. The Rust port reproduces each recorded site
per target (see the module headers of convolve_slow.rs and enc_transforms.rs).

For every oracle graph given, this re-runs the actual compile command of each
original archive object from its compile_commands.json, unchanged except for
emitting IR with line tables into the new output directory, and compares the
fmuladd source sites with tools/encoder_transforms_contraction.json. It also
disassembles the archive objects that the oracle links: on AArch64 an object
must contain fused instructions exactly when it has recorded sites; on x86-64
none may contain any. No source, flag, object or binary is replaced.
"""
import argparse
import json
from pathlib import Path
import platform
import re
import shlex
import subprocess

ROOT = Path(__file__).resolve().parent.parent
MAP = ROOT / "tools/encoder_transforms_contraction.json"
FUSED = {
    "aarch64": re.compile(r"\s(fn?m(add|sub)|fml[as])\s"),
    "x86_64": re.compile(r"\s(v?fn?m(add|sub)\d+[sp][sd])\s"),
}
DEPENDENCY_FLAGS = {"-MD", "-MMD"}
DEPENDENCY_OPTIONS = {"-MT", "-MF", "-MQ"}


def arch():
    machine = platform.machine().lower()
    if machine in ("arm64", "aarch64"):
        return "aarch64"
    if machine in ("x86_64", "amd64"):
        return "x86_64"
    raise SystemExit(f"No recorded contraction lowering for {machine}")


def entries(build):
    commands = json.loads((build / "compile_commands.json").read_text())
    for entry in commands:
        source = Path(entry["file"]).resolve()
        try:
            relative = source.relative_to(ROOT / "upstream/libjxl").as_posix()
        except ValueError:
            continue
        args = entry.get("arguments") or shlex.split(entry["command"])
        yield relative, entry["directory"], args


def emit_ir(directory, args, output):
    out = []
    skip = False
    for i, arg in enumerate(args):
        if skip:
            skip = False
            continue
        if arg in DEPENDENCY_FLAGS or arg == "-c":
            continue
        if arg in DEPENDENCY_OPTIONS or arg == "-o":
            skip = True
            continue
        out.append(arg)
    out += ["-S", "-emit-llvm", "-gline-tables-only", "-o", str(output)]
    subprocess.run(out, cwd=directory, check=True)
    return output.read_text()


def fmuladd_sites(ir):
    metadata = {}
    for match in re.finditer(r"^!(\d+) = (?:distinct )?!(\w+)\((.*)\)$", ir, re.M):
        metadata[match[1]] = (match[2], match[3])

    def field(body, key):
        match = re.search(key + r': (!\d+|"[^"]*"|\d+)', body)
        return match[1] if match else None

    def filename(node):
        kind, body = metadata[node]
        if kind == "DILocation":
            return filename(field(body, "scope")[1:])
        if kind in ("DILexicalBlock", "DILexicalBlockFile", "DISubprogram"):
            name = re.search(r'filename: "([^"]*)"', metadata[field(body, "file")[1:]][1])[1]
            return name.split("upstream/libjxl/")[-1]
        raise RuntimeError(f"Unexpected debug scope {kind}")

    sites = set()
    for kind, node in re.findall(r"call [^\n]*@llvm\.fmuladd\.(f32|f64)\([^\n]*!dbg !(\d+)", ir):
        body = metadata[node][1]
        sites.add((filename(node), int(field(body, "line")), int(field(body, "column")), kind))
    return sites


def fused_instructions(objdump, target, obj):
    text = subprocess.run([objdump, "-d", "--no-show-raw-insn", str(obj)], capture_output=True,
                          text=True, check=True).stdout
    return sum(1 for line in text.splitlines() if FUSED[target].search(line + " "))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path, action="append",
                        help="Oracle build directory (default: target/oracle and, if built, target/oracle-asan)")
    parser.add_argument("--objdump", default="objdump")
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    builds = args.build or [p for p in (ROOT / "target/oracle", ROOT / "target/oracle-asan")
                            if (p / "compile_commands.json").is_file()]
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    recorded = json.loads(MAP.read_text())
    target = arch()
    report = {"target": target, "lowering": recorded["lowering"][target], "graphs": {}}
    for build in builds:
        build = build.resolve()
        graph = {}
        found = {name: False for name in recorded["sources"]}
        for relative, directory, command in entries(build):
            if relative not in recorded["sources"]:
                continue
            found[relative] = True
            ir = emit_ir(directory, command, output / f"{build.name}-{Path(relative).name}.ll")
            sites = sorted(fmuladd_sites(ir))
            expected = sorted((s["file"], s["line"], s["column"], s["type"])
                              for s in recorded["sources"][relative])
            if sites != expected:
                raise SystemExit(f"{build.name} {relative}: fmuladd sites {sites} != recorded {expected}")
            index = command.index("-o")
            obj = Path(directory) / command[index + 1]
            fused = fused_instructions(args.objdump, target, obj)
            if target == "x86_64" and fused != 0:
                raise SystemExit(f"{build.name} {relative}: {fused} fused instructions on x86-64")
            if target == "aarch64" and (fused > 0) != bool(sites):
                raise SystemExit(f"{build.name} {relative}: {fused} fused instructions for {len(sites)} sites")
            graph[relative] = {"sites": [list(s) for s in sites], "fused_instructions": fused}
        missing = [name for name, seen in found.items() if not seen]
        if missing:
            raise SystemExit(f"{build.name}: no compile command for {missing}")
        report["graphs"][build.name] = graph
        print(f"{build.name}: {sum(len(g['sites']) for g in graph.values())} recorded fmuladd sites, "
              f"{sum(g['fused_instructions'] for g in graph.values())} fused {target} instructions",
              flush=True)
    (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(output / "report.json")


if __name__ == "__main__":
    main()
