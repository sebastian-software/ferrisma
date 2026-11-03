#!/usr/bin/env python3
"""Build and audit native pinned Brotli allocation/packed-bit representation.

This is a focused standalone diagnostic, not the repository-wide check gate.
All build products, observations and original/generated assembly stay under the
new output directory. No original source, flags or archive objects are replaced.
"""
import argparse
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)

    def execute(label, cmd):
        with (output / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, cmd)), cwd=ROOT, stdout=log,
                           stderr=subprocess.STDOUT, check=True)

    execute("tables-check", [sys.executable, ROOT / "tools/generate_brotli_tables.py", "--check"])
    binaries = {}
    for sanitized in [False, True]:
        graph = "sanitized" if sanitized else "ordinary"
        for component in ["brotli", "brotli_bitfield"]:
            directory = output / f"{component}-{graph}"
            target = "brotli_source_observer" if component == "brotli" else "brotli_bitfield_observer"
            execute(f"{component}-{graph}-configure", ["cmake", "-S", ROOT / f"tools/oracle/{component}",
                "-B", directory, "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_COMPILER=clang",
                *(["-DCMAKE_CXX_COMPILER=clang++"] if component == "brotli" else []),
                f"-DORACLE_SANITIZERS={'ON' if sanitized else 'OFF'}"])
            execute(f"{component}-{graph}-build", ["cmake", "--build", directory, "--target", target, "--parallel", "2"])
            binaries[component, graph] = directory / target
    rust = []
    assembly = []
    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"), ("msrv", "1.85", "0")]:
        binary = output / f"rust-{name}"
        asm = output / f"rust-{name}.s"
        execute(f"rust-{name}-build", ["rustup", "run", toolchain, "rustc", "--edition", "2024",
            "-D", "warnings", "-C", f"opt-level={level}",
            f"--emit=link={binary},asm={asm}", ROOT / "tools/brotli_bitfield_probe.rs"])
        rust += ["--rust", f"{name}={binary}"]
        assembly += ["--rust-asm", f"{name}={asm}"]
    execute("allocation-audit", [sys.executable, ROOT / "tools/brotli_reference.py",
        "--ordinary", binaries["brotli", "ordinary"], "--sanitized", binaries["brotli", "sanitized"],
        "--output", output / "allocation"])
    execute("bitfield-audit", [sys.executable, ROOT / "tools/brotli_bitfield_reference.py",
        "--ordinary", binaries["brotli_bitfield", "ordinary"], "--sanitized", binaries["brotli_bitfield", "sanitized"],
        *rust, *assembly, "--decoder-report", output / "allocation/report.json", "--output", output / "bitfield"])
    print(output / "bitfield/report.json")


if __name__ == "__main__":
    main()
