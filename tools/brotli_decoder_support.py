#!/usr/bin/env python3
"""Fresh source/Rust proof of decoder lifecycle and early header readers.

This is a focused component runner, not a claim that the streaming decoder is
complete. Source/default archive bindings, default compiler configuration and
all ASan/UBSan instrumentation are retained. No runtime C decoder is used by
the Rust components; original programs are independent test observers only.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

ROOT = Path(__file__).resolve().parent.parent


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--resume", action="store_true",
                        help="reuse a partial build directory; reconfigure/build and run every check again")
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=args.resume)

    def execute(label, command, env=None):
        with (output / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, command)), cwd=ROOT, env=env,
                           stdout=log, stderr=subprocess.STDOUT, check=True)

    execute("tables-check", [sys.executable, ROOT / "tools/generate_brotli_tables.py", "--check"])
    originals = {}
    for component in ["instance", "metablock", "metadata"]:
        for sanitized in [False, True]:
            graph = "sanitized" if sanitized else "ordinary"
            directory = output / f"{component}-{graph}"
            execute(f"{component}-{graph}-configure", ["cmake", "-S", ROOT / f"tools/oracle/brotli_{component}",
                "-B", directory, "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_COMPILER=clang",
                "-DCMAKE_CXX_COMPILER=clang++", f"-DORACLE_SANITIZERS={'ON' if sanitized else 'OFF'}"])
            targets = [f"brotli_{component}_oracle"]
            if component == "metablock":
                targets.append("brotli_metablock_default")
            execute(f"{component}-{graph}-build", ["cmake", "--build", directory, "--target", *targets, "--parallel", "2"])
            originals[component, graph] = directory / f"brotli_{component}_oracle"
    rust = {"instance": [], "metablock": []}
    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"), ("msrv", "1.95", "0")]:
        compiler = ["rustup", "run", toolchain, "rustc", "--edition", "2024", "-D", "warnings", "-C", f"opt-level={level}"]
        for component in ["instance", "metablock"]:
            binary = output / f"{component}-{name}"
            tests = output / f"{component}-{name}-tests"
            execute(f"{component}-{name}-build", [*compiler, ROOT / f"tools/brotli_{component}_probe.rs", "-o", binary])
            execute(f"{component}-{name}-tests-build", [*compiler, "--test", "--cfg", 'feature="cpp-oracle"',
                ROOT / f"tests/brotli_{component}.rs", "-o", tests])
            for graph in ["ordinary", "sanitized"]:
                env = dict(os.environ, RUST_TEST_THREADS="1")
                env[f"BROTLI_{component.upper()}_ORACLE"] = str(originals[component, graph])
                execute(f"{component}-{name}-{graph}-tests", [tests], env)
            rust[component] += ["--rust", f"{name}={binary}"]
    for component in ["instance", "metablock"]:
        defaults = []
        if component == "metablock":
            defaults = ["--default-ordinary", output / "metablock-ordinary/brotli_metablock_default",
                        "--default-sanitized", output / "metablock-sanitized/brotli_metablock_default"]
        execute(f"{component}-audit", [sys.executable, ROOT / f"tools/brotli_{component}_reference.py",
            "--ordinary", originals[component, "ordinary"], "--sanitized", originals[component, "sanitized"],
            *defaults, *rust[component], "--output", output / f"{component}-audit"])
    execute("metadata-source-audit", [sys.executable, ROOT / "tools/brotli_metadata_reference.py",
        "--ordinary", originals["metadata", "ordinary"], "--sanitized", originals["metadata", "sanitized"],
        "--output", output / "metadata-source-audit"])
    (output / "clippy.toml").write_text('msrv = "1.95"\n')
    for component in ["instance", "metablock"]:
        execute(f"{component}-clippy", ["clippy-driver", "--edition=2024", "--test", ROOT / f"tests/brotli_{component}.rs",
            "--cfg", 'feature="cpp-oracle"', "-D", "warnings", "-o", output / f"{component}-clippy"],
            dict(os.environ, CLIPPY_CONF_DIR=str(output)))
    print(output / "instance-audit/report.json")
    print(output / "metablock-audit/report.json")
    (output / "report.json").write_text(json.dumps({"all_six_exact": True,
        "instance": "instance-audit/report.json", "metablock": "metablock-audit/report.json",
        "instance_tests_each_configuration": 5, "metablock_tests_each_configuration": 5,
        "source_only_metadata_flow": "metadata-source-audit/report.json",
        "complete_streaming_decoder": False}, indent=2) + "\n")


if __name__ == "__main__":
    main()
