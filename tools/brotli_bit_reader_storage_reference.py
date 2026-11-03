#!/usr/bin/env python3
"""Source-storage reader loans preserve fields and all prior reader observations.

The optional build cache is reconfigured/rebuilt; no check is skipped. The
original reader observers supplied below remain unchanged, including binaries,
source configuration and sanitizer policy.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess
import sys

from brotli_decoder_binding import verify
from brotli_state_reference import ROOT, sha


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--ordinary", type=Path, required=True,
                        help="unchanged original reader observer")
    parser.add_argument("--sanitized", type=Path, required=True,
                        help="unchanged fully instrumented original reader observer")
    parser.add_argument("--build-root", type=Path)
    args = parser.parse_args()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    builds = args.build_root.resolve() if args.build_root else out

    def execute(label, command, env=None):
        with (out / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, command)), cwd=ROOT, env=env,
                           stdout=log, stderr=subprocess.STDOUT, check=True)

    requests = "".join(f"{skip} {width}\n" for skip in range(33) for width in range(33)).encode()
    (out / "requests.txt").write_bytes(requests)
    report = {"loan_requests": 1089, "source": {}, "rust": {},
              "pointer_identity": "native NULL/non-NULL token stays adapter-owned; no slice pointer synthesis"}
    original = {}
    for graph in ["ordinary", "sanitized"]:
        build = builds / f"source-{graph}"
        execute(f"{graph}-configure", ["cmake", "-S", ROOT / "tools/oracle/brotli_bit_reader_storage",
            "-B", build, "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_COMPILER=clang",
            f"-DORACLE_SANITIZERS={'ON' if graph == 'sanitized' else 'OFF'}"])
        execute(f"{graph}-build", ["cmake", "--build", build, "--target",
                                 "brotli_bit_reader_storage_oracle", "--parallel", "2"])
        binary = build / "brotli_bit_reader_storage_oracle"
        original[graph] = binary
        result = subprocess.run([binary], input=requests, capture_output=True, check=True)
        assert not result.stderr and len(result.stdout.splitlines()) == 1089
        (out / f"{graph}.txt").write_bytes(result.stdout)
        if graph == "sanitized":
            assert result.stdout == (out / "ordinary.txt").read_bytes()
        report["source"][graph] = verify(binary, graph == "sanitized",
            "brotli_bit_reader_storage.map", ["brotli_bit_reader_storage.c"], {
                "BrotliSafeReadBits32Slow": "libbrotlidec.a(bit_reader.c.o)",
                "BrotliWarmupBitReader": "libbrotlidec.a(bit_reader.c.o)"})
        report["source"][graph]["output_sha256"] = sha(out / f"{graph}.txt")
    rust = []
    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"),
                                    ("msrv", "1.95", "0")]:
        compiler = ["rustup", "run", toolchain, "rustc", "--edition", "2024", "-D", "warnings",
                    "-C", f"opt-level={level}"]
        binary = out / name
        execute(f"{name}-probe-build", [*compiler, ROOT / "tools/brotli_bit_reader_probe.rs", "-o", binary])
        rust += ["--rust", f"{name}={binary}"]
        for component in ["bit_reader", "bit_reader_storage"]:
            tests = out / f"{component}-{name}-tests"
            execute(f"{component}-{name}-tests-build", [*compiler, "--test", "--cfg", 'feature="cpp-oracle"',
                ROOT / f"tests/brotli_{component}.rs", "-o", tests])
            for graph in original:
                execute(f"{component}-{name}-{graph}", [tests], dict(os.environ, RUST_TEST_THREADS="2",
                    BROTLI_BIT_READER_STORAGE_ORACLE=str(original[graph])))
        report["rust"][name] = {"old_tests_each_graph": 6, "new_tests_each_graph": 3,
                               "binary_sha256": sha(binary)}
    execute("retained-reader-audit", [sys.executable, ROOT / "tools/brotli_bit_reader_extended_reference.py",
        "--ordinary", args.ordinary.resolve(), "--sanitized", args.sanitized.resolve(),
        *rust, "--output", out / "retained-reader-audit"])
    (out / "clippy.toml").write_text('msrv = "1.95"\n')
    execute("clippy", ["clippy-driver", "--edition=2024", "--test", "--cfg", 'feature="cpp-oracle"',
        ROOT / "tests/brotli_bit_reader_storage.rs", "-D", "warnings", "-o", out / "clippy"],
        dict(os.environ, CLIPPY_CONF_DIR=str(out)))
    report["retained_reader_audit_sha256"] = sha(out / "retained-reader-audit/report.json")
    (out / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(out / "report.json")


if __name__ == "__main__":
    main()
