#!/usr/bin/env python3
"""Audit pristine selected alpha symbols against the original default objects.

The selected wrapper changes only inlining. Removing that one object from an
otherwise identical temporary link makes the unchanged original archive supply
all alpha operations. No active object, dependency file, or binary is replaced.
"""
import argparse
import hashlib
import json
from pathlib import Path
import re
import shlex
import subprocess
import sys
import tempfile

ROOT = Path(__file__).resolve().parent.parent


def requests():
    # The same 384 exceptional requests as the actual Rust parity generator.
    words = [0, 0x80000000, 0x3e800000, 0x3f400000, 0x3f800000, 0x40000000,
             0xc0800000, 0x00800000, 1, 0x7f7fffff, 0x7f800000, 0xff800000,
             0x7fc12345, 0xffc98765, 0x7fa12345, 0xffa98765]
    finite_words = [0, 0x80000000, 0x3e800000, 0x3f400000, 0x3f800000,
                    0x40000000, 0xc0800000, 0x00800000, 1, 0x7f7fffff,
                    0xff7fffff, 0x3eaaaaab, 0xbecccccd, 0x80800000,
                    0x00800001, 0x80000001]
    cases, finite = [], []
    for op in range(6):
        for premul in range(2):
            for clamp in range(2):
                for alias in range(4):
                    bi, bai, fi, fai = [(0, 1, 2, 3), (0, 0, 2, 2),
                                        (0, 1, 2, 2), (0, 0, 2, 3)][alias]
                    for selected_words, variants in [(words, range(4)),
                                                      (finite_words, range(16))]:
                        for variant in variants:
                            n = 0 if selected_words is words and variant == 0 else (
                                17 if selected_words is words else 19)
                            values = ["patch-alpha", op, premul, clamp, n,
                                      bi, bai, fi, fai]
                            values.extend(selected_words[(x * 3 + c * 5 + variant) %
                                                         len(selected_words)]
                                          for c in range(5) for x in range(n))
                            cases.append(" ".join(map(str, values)))
                            finite.append(n == 0 or selected_words is finite_words)
    if (len(cases), sum(finite)) != (1920, 1632):
        raise RuntimeError("Alpha reference corpus coverage changed")
    return cases, finite


def verify_rust_requests(cases, temporary):
    source = (ROOT / "tests/parity/patches.rs").read_text()
    name = "patches_alpha_kernels_aliases_rounding_and_exceptional_values"
    match = re.search(r"fn " + name + r"\(\) \{.*?^\}", source, re.M | re.S)
    if match is None:
        raise RuntimeError("Actual Rust alpha request generator not found")
    program = ("use std::fmt::Write as _;\n"
               "fn chunks(cases:Vec<String>){for case in cases {println!(\"{}\",case);}}\n"
               + match.group(0) + "\nfn main(){" + name + "();}\n")
    code, binary = temporary / "alpha_requests.rs", temporary / "alpha_requests"
    code.write_text(program)
    subprocess.run(["rustc", "--edition=2024", str(code), "-o", str(binary)], check=True)
    actual = subprocess.run([str(binary)], capture_output=True, text=True, check=True).stdout.splitlines()
    if actual != cases:
        raise RuntimeError("Alpha audit corpus differs from actual Rust parity generator")
    print(f"All {len(cases)} audit inputs match the actual Rust request generator", flush=True)


def observe(binary, cases):
    result = subprocess.run([str(binary)], input="\n".join(cases) + "\n",
                            text=True, capture_output=True, check=True)
    lines = result.stdout.splitlines()
    if len(lines) != len(cases):
        raise RuntimeError("Alpha reference response count changed")
    return lines


def link(build, temporary, selected):
    command = shlex.split((build / "CMakeFiles/jxl_oracle.dir/link.txt").read_text())
    obj = "CMakeFiles/jxl_oracle.dir/alpha.cc.o"
    if command.count(obj) != 1:
        raise RuntimeError("Expected exactly one selected alpha object")
    binary = temporary / ("selected" if selected else "default")
    command[command.index("-o") + 1] = str(binary)
    if selected:
        map_file = temporary / "selected.map"
        command.append("-Wl,-map," + str(map_file) if sys.platform == "darwin"
                       else "-Wl,-Map," + str(map_file))
    else:
        # The archive and all its original alpha compile flags remain intact.
        command.remove(obj)
    subprocess.run(command, cwd=build, check=True)
    if selected:
        verify_binding(map_file.read_text())
    return binary


def verify_binding(contents):
    names = ["PerformAlphaBlending", "PerformAlphaWeightedAdd",
             "PerformMulBlending", "PremultiplyAlpha", "UnpremultiplyAlpha"]
    if sys.platform == "darwin":
        matches = re.findall(r"^\[\s*(\d+)\]\s+.*CMakeFiles/jxl_oracle.dir/alpha\.cc\.o$",
                             contents, re.M)
        if len(matches) != 1:
            raise RuntimeError("Selected alpha object absent from link map")
        source = matches[0]
        bound = [line for line in contents.splitlines()
                 if re.search(r"\[\s*" + source + r"\]\s+", line)
                 and "FDE for:" not in line
                 and any(name in line for name in names)]
    else:
        bound = []
        current = ""
        for line in contents.splitlines():
            if "CMakeFiles/jxl_oracle.dir/" in line and ".o" in line:
                current = line
            if re.search(r"^\s+0x[0-9a-f]+\s+", line) and any(
                    "jxl::" + name + "(" in line for name in names):
                if "CMakeFiles/jxl_oracle.dir/alpha.cc.o" not in current:
                    raise RuntimeError("Alpha symbol bound outside selected object: " + line)
                bound.append(line)
    if len(bound) != 6 or any(not any(name in line for line in bound) for name in names):
        raise RuntimeError("Missing or duplicate original alpha public symbol bindings")
    print("All 6 original alpha public symbols bind to the pristine selected TU", flush=True)


def audit(ordinary, sanitized):
    cases, finite = requests()
    # Direct inclusion must continue to use the exact source at the pinned tree.
    source = ROOT / "upstream/libjxl/lib/jxl/alpha.cc"
    digest = hashlib.sha256(source.read_bytes()).hexdigest()
    if digest != "0935796743e0f4524bf0f506883d0bbc605f344bd8c939c6ba501b10c7c85b52":
        raise RuntimeError("Pinned original alpha source changed")
    names = ("ordinary", "ASan")
    with tempfile.TemporaryDirectory(prefix="jxl-alpha-reference-") as directory:
        temporary = Path(directory)
        verify_rust_requests(cases, temporary)
        chosen, defaults = [], []
        for name, build in zip(names, (ordinary, sanitized)):
            entries = json.loads((build / "compile_commands.json").read_text())
            entry, = [e for e in entries if Path(e["file"]).resolve() ==
                      ROOT / "tools/oracle/alpha.cc"]
            command = shlex.split(entry["command"])
            if command.count("-fno-inline-functions") != 1 or "-O3" not in command:
                raise RuntimeError("Unexpected scoped alpha compile configuration")
            if (name == "ASan") != any("-fsanitize=" in arg for arg in command):
                raise RuntimeError("Alpha sanitizer configuration changed")
            local = temporary / name
            local.mkdir()
            chosen.append(link(build, local, True))
            defaults.append(link(build, local, False))
        minimal = "patch-alpha 4 0 0 1 0 1 2 3 0 1056964608 1065353216 4289300325 0"
        for name, selected, default in zip(names, chosen, defaults):
            print(f"{name}: signaling-alpha default={observe(default, [minimal])[0]}; "
                  f"selected={observe(selected, [minimal])[0]}", flush=True)
        for start in range(0, len(cases), 16):
            chunk = cases[start:start + 16]
            previous = None
            for name, selected, default in zip(names, chosen, defaults):
                actual, baseline = observe(selected, chunk), observe(default, chunk)
                for index, (a, b) in enumerate(zip(actual, baseline), start):
                    if finite[index] and a != b:
                        raise RuntimeError(f"Finite alpha source drift in {name}, case {index}")
                if previous is not None and actual != previous:
                    raise RuntimeError(f"Selected alpha ordinary/ASan drift at case {start}")
                previous = actual
    for name in names:
        print(f"{name}: {sum(finite)} finite alpha requests match default source", flush=True)
    print(f"Selected ordinary/ASan alpha objects agree on {len(cases)} requests", flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ordinary-build", type=Path, default=ROOT / "target/oracle")
    parser.add_argument("--sanitized-build", type=Path, default=ROOT / "target/oracle-asan")
    args = parser.parse_args()
    audit(args.ordinary_build.resolve(), args.sanitized_build.resolve())
