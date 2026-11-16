#!/usr/bin/env python3
"""Build and verify native raw dictionary-transform destination representation.

The actual original CMake targets and all sanitizers remain unchanged. This
standalone proof does not enable the raw helper in decoder state. Native ARM64
and x86-64 acceptance is required before that separate integration checkpoint.
"""
import argparse
import json
from pathlib import Path
import platform
import re
import subprocess
import sys
from rust_version import MSRV

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    assert platform.machine().lower() in ["aarch64", "arm64", "x86_64", "amd64"]
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)

    def execute(label, command):
        with (output / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, command)), cwd=ROOT, stdout=log,
                           stderr=subprocess.STDOUT, check=True)

    execute("tables-check", [sys.executable, ROOT / "tools/generate_brotli_tables.py", "--check"])
    original = {}
    for sanitized in [False, True]:
        name = "sanitized" if sanitized else "ordinary"
        directory = output / f"source-{name}"
        execute(f"source-{name}-configure", ["cmake", "-S", ROOT / "tools/oracle/brotli_raw_transforms",
            "-B", directory, "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_COMPILER=clang",
            "-DCMAKE_CXX_COMPILER=clang++", f"-DORACLE_SANITIZERS={'ON' if sanitized else 'OFF'}"])
        execute(f"source-{name}-build", ["cmake", "--build", directory, "--target",
            "brotli_raw_transforms_oracle", "--parallel", "2"])
        original[name] = directory / "brotli_raw_transforms_oracle"
        with (output / f"source-{name}.s").open("w") as assembly:
            command = ["otool", "-tvV", original[name]] if sys.platform == "darwin" else ["objdump", "-d", original[name]]
            subprocess.run(list(map(str, command)), stdout=assembly, check=True)
    rust = []
    asm_checks = {}
    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"), ("msrv", MSRV, "0")]:
        compiler = ["rustup", "run", toolchain, "rustc", "--edition", "2024", "-D", "warnings", "-C", f"opt-level={level}"]
        binary, assembly = output / f"rust-{name}", output / f"rust-{name}.s"
        execute(f"rust-{name}-build", [*compiler, f"--emit=link={binary},asm={assembly}", ROOT / "tools/brotli_raw_transform_probe.rs"])
        tests = output / f"tests-{name}"
        execute(f"tests-{name}-build", [*compiler, "--test", ROOT / "tests/brotli_raw_transform.rs", "-o", tests])
        execute(f"tests-{name}-run", [tests])
        # Since 2026-11-16 the raw XOR is plain Rust on initialized bytes
        # (owner decision; ADR-0044, port ADR 0003): the probe must contain no
        # inline assembly. Rust's standard runtime may emit an empty
        # compiler-fence block, which has no instructions.
        blocks = re.findall(r"(?:; InlineAsm Start|#APP)(.*?)(?:; InlineAsm End|#NO_APP)", assembly.read_text(), re.S)
        blocks = [block for block in blocks if block.strip()]
        assert not blocks, (name, "inline assembly in the raw XOR probe", blocks)
        asm_checks[name] = len(blocks)
        rust += ["--rust", f"{name}={binary}"]
    execute("raw-transform-audit", [sys.executable, ROOT / "tools/brotli_raw_transform_reference.py",
        "--ordinary", original["ordinary"], "--sanitized", original["sanitized"],
        *rust, "--output", output / "audit"])
    (output / "assembly-checks.json").write_text(json.dumps(asm_checks, indent=2) + "\n")
    print(output / "audit/report.json")


if __name__ == "__main__":
    main()
