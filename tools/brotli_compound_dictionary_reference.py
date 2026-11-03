#!/usr/bin/env python3
"""Actual raw attachment caller-domain proof against both pristine source graphs.

This covers attachment/copy with mutation after attachment, late initialization,
mutation between copies, and NULL/nonzero-length attachment without a reached
copy. It is a component proof, not a complete Brotli streaming decoder claim.
"""
import argparse
import hashlib
import json
import os
from pathlib import Path
import subprocess

from brotli_decoder_binding import verify
from brotli_state_reference import ROOT

EXPECTED = (b"M0 A1 I1 C6 P1:1 5b0203040542\n"
            b"M1 A1 I1 C6 P1:1 0b0c0d0e0f10\n"
            b"M2 A1 I1 C6 P1:1 010203040506 I1 C6 5b0203040542\n"
            b"M3 A1 S6 P1:1\n")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--build-root", type=Path,
                        help="optional cache; configure/build and every check still execute")
    args = parser.parse_args()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    builds = args.build_root.resolve() if args.build_root else out

    def execute(label, command, env=None):
        with (out / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, command)), cwd=ROOT, env=env,
                           stdout=log, stderr=subprocess.STDOUT, check=True)

    expected_owners = {
        **{name: "brotli_compound_dictionary.c.o" for name in [
            "BrotliDecoderCreateInstance", "BrotliDecoderAttachDictionary",
            "BrotliDecoderDestroyInstance"]},
        "BrotliSharedDictionaryAttach": "libbrotlicommon.a(shared_dictionary.c.o)"}
    original = {}
    report = {"source": {}, "rust": {}, "complete_streaming_decoder": False}
    for graph in ["ordinary", "sanitized"]:
        build = builds / f"source-{graph}"
        execute(f"{graph}-configure", ["cmake", "-S", ROOT / "tools/oracle/brotli_compound_dictionary",
            "-B", build, "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_COMPILER=clang",
            f"-DORACLE_SANITIZERS={'ON' if graph == 'sanitized' else 'OFF'}"])
        execute(f"{graph}-build", ["cmake", "--build", build, "--target",
                                 "brotli_compound_dictionary_oracle", "--parallel", "2"])
        binary = build / "brotli_compound_dictionary_oracle"
        original[graph] = binary
        result = subprocess.run([binary], capture_output=True, check=True)
        assert not result.stderr and result.stdout == EXPECTED, result
        (out / f"{graph}.stdout").write_bytes(result.stdout)
        report["source"][graph] = verify(binary, graph == "sanitized",
            "brotli_compound_dictionary.map", ["brotli_compound_dictionary.c"], expected_owners)
    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"),
                                    ("msrv", "1.85", "0")]:
        binary = out / f"{name}-tests"
        execute(f"{name}-build", ["rustup", "run", toolchain, "rustc", "--edition", "2024",
            "--test", "--cfg", 'feature="cpp-oracle"', "-D", "warnings", "-C", f"opt-level={level}",
            ROOT / "tests/brotli_compound_dictionary.rs", "-o", binary])
        for graph, source in original.items():
            execute(f"{name}-{graph}", [binary], dict(os.environ, RUST_TEST_THREADS="2",
                BROTLI_COMPOUND_DICTIONARY_ORACLE=str(source)))
        report["rust"][name] = {"tests_each_graph": 8,
                               "binary_sha256": hashlib.sha256(binary.read_bytes()).hexdigest()}
    (out / "clippy.toml").write_text('msrv = "1.85"\n')
    execute("clippy", ["clippy-driver", "--edition=2024", "--test", "--cfg", 'feature="cpp-oracle"',
        ROOT / "tests/brotli_compound_dictionary.rs", "-D", "warnings", "-o", out / "clippy"],
        dict(os.environ, CLIPPY_CONF_DIR=str(out)))
    (out / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(out / "report.json")


if __name__ == "__main__":
    main()
