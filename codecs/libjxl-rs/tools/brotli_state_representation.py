#!/usr/bin/env python3
"""Fresh original/Rust state allocation, callable-default and partial-byte proof.

This focused runner preserves source algorithms/default flags/all sanitizers.
The SIZE_MAX default allocator policy is a separate explicit source probe:
ordinary malloc failure and the ASan allocator interceptor abort are both kept,
without changing allocator_may_return_null or classifying the request as UB.
"""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)

    def execute(label, command, env=None):
        with (out / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, command)), cwd=ROOT, env=env,
                           stdout=log, stderr=subprocess.STDOUT, check=True)

    execute("tables-check", [sys.executable, ROOT / "tools/generate_brotli_tables.py", "--check"])
    original = {}
    for sanitized in [False, True]:
        graph = "sanitized" if sanitized else "ordinary"
        directory = out / f"source-{graph}"
        execute(f"source-{graph}-configure", ["cmake", "-S", ROOT / "tools/oracle/brotli_state", "-B", directory,
            "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_COMPILER=clang", "-DCMAKE_CXX_COMPILER=clang++",
            f"-DORACLE_SANITIZERS={'ON' if sanitized else 'OFF'}"])
        execute(f"source-{graph}-build", ["cmake", "--build", directory, "--target", "brotli_state_oracle", "--parallel", "2"])
        original[graph] = directory / "brotli_state_oracle"
    rust = []
    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"), ("msrv", "1.95", "0")]:
        compiler = ["rustup", "run", toolchain, "rustc", "--edition", "2024", "-D", "warnings", "-C", f"opt-level={level}"]
        binary, tests = out / name, out / f"{name}-tests"
        execute(f"{name}-build", [*compiler, ROOT / "tools/brotli_state_probe.rs", "-o", binary])
        execute(f"{name}-tests-build", [*compiler, "--test", "--cfg", 'feature="cpp-oracle"', ROOT / "tests/brotli_state.rs", "-o", tests])
        for graph in ["ordinary", "sanitized"]:
            import os
            env = os.environ.copy()
            env["BROTLI_STATE_ORACLE"] = str(original[graph])
            execute(f"{name}-{graph}-tests", [tests, "--nocapture"], env)
        rust += ["--rust", f"{name}={binary}"]
    execute("state-audit", [sys.executable, ROOT / "tools/brotli_state_reference.py",
        "--ordinary", original["ordinary"], "--sanitized", original["sanitized"],
        *rust, "--output", out / "audit"])
    probe = out / "default-size-max"
    probe.mkdir()
    request = b"bs-default 18446744073709551615 0 0\n"
    (probe / "request.txt").write_bytes(request)
    report = {"classification": "instrumentation allocator policy; not codec undefined behavior", "configurations": {}}
    for graph, binary in original.items():
        result = subprocess.run([binary], input=request, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        if graph == "ordinary":
            assert result.returncode == 0 and result.stdout == b"0\n" and not result.stderr
        else:
            assert result.returncode != 0 and not result.stdout
            assert b"ERROR: AddressSanitizer: requested allocation size 0xffffffffffffffff" in result.stderr
            assert b"exceeds maximum supported size" in result.stderr
            assert b"runtime error:" not in result.stderr
        for stream, data in [("stdout", result.stdout), ("stderr", result.stderr)]:
            (probe / f"{graph}.{stream}").write_bytes(data)
        report["configurations"][graph] = {"exit": result.returncode,
            "stdout_sha256": hashlib.sha256(result.stdout).hexdigest(), "stderr_sha256": hashlib.sha256(result.stderr).hexdigest()}
    (probe / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(out / "audit/report.json")


if __name__ == "__main__":
    main()
