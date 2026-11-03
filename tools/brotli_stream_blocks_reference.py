#!/usr/bin/env python3
"""Actual source metadata callbacks and uncompressed partial-output checkpoints.

This is a component proof on actual decoder owners, not a complete compressed
stream claim. Input/storage loans, real callback replacement, local unknown
stack tails, initialized internal-buffer prefixes and original NULL pointers
are exercised without source-body/configuration changes.
"""
import argparse
import json
import os
from pathlib import Path
import subprocess

from brotli_decoder_binding import verify
from brotli_state_reference import ROOT, sha


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    parser.add_argument("--build-root", type=Path,
                        help="optional cache; configure/build/every check still execute")
    args = parser.parse_args()
    out = args.output.resolve()
    out.mkdir(parents=True, exist_ok=False)
    builds = args.build_root.resolve() if args.build_root else out

    def execute(label, command, env=None):
        with (out / f"{label}.log").open("w") as log:
            subprocess.run(list(map(str, command)), cwd=ROOT, env=env,
                           stdout=log, stderr=subprocess.STDOUT, check=True)

    original = {}
    report = {"source": {}, "gates": {}, "complete_streaming_decoder": False}
    expected_owners = {
        **{name: "brotli_stream_blocks.c.o" for name in ["BrotliDecoderCreateInstance",
            "BrotliDecoderDestroyInstance", "BrotliDecoderSetMetadataCallbacks"]},
        "BrotliDecoderStateInit": "libbrotlidec.a(state.c.o)",
        "BrotliSharedDictionaryCreateInstance": "libbrotlicommon.a(shared_dictionary.c.o)"}
    for graph in ["ordinary", "sanitized"]:
        build = builds / f"source-{graph}"
        execute(f"{graph}-configure", ["cmake", "-S", ROOT / "tools/oracle/brotli_stream_blocks", "-B", build,
            "-DCMAKE_BUILD_TYPE=Release", "-DCMAKE_C_COMPILER=clang",
            f"-DORACLE_SANITIZERS={'ON' if graph == 'sanitized' else 'OFF'}"])
        execute(f"{graph}-build", ["cmake", "--build", build, "--target", "brotli_stream_blocks_oracle", "--parallel", "2"])
        binary = build / "brotli_stream_blocks_oracle"
        original[graph] = binary
        report["source"][graph] = verify(binary, graph == "sanitized", "brotli_stream_blocks.map",
                                           ["brotli_stream_blocks.c"], expected_owners)
        result = subprocess.run([binary], input=b"2 0 0 0 0 0 0\n3 0 0 0 6 0 0\n", capture_output=True, check=True)
        assert not result.stderr and result.stdout == b"N1 0 0 0 0 1 1 1\nM1 91 0 0 2 0 6\n"
        (out / f"{graph}-native-input.txt").write_bytes(result.stdout)
        aliases=b"".join(f"4 {mode} 0 0 0 0 0\n".encode() for mode in range(4))
        result=subprocess.run([binary],input=aliases,capture_output=True,check=True)
        assert not result.stderr
        expected=(b"A0 R3 AO3 T3 P1 S3 POS8 O010203cccccccccc\n"
                  b"A1 R1 AO12 T8 P1 S8 POS8 Occcccccccccccccc\n"
                  b"A2 R1 AO0 T8 P1 S8 POS8 Occcccccccccccccc\n"
                  b"A3 R1 AO0 T8 P1 S8 POS8 Occcccccccccccccc\n")
        assert result.stdout==expected
        (out/f"{graph}-cursor-aliases.txt").write_bytes(result.stdout)

    for name, toolchain, level in [("debug", "stable", "0"), ("release", "stable", "3"), ("msrv", "1.85", "0")]:
        compiler = ["rustup", "run", toolchain, "rustc", "--edition", "2024", "-D", "warnings", "-C", f"opt-level={level}"]
        for component in ["stream_input", "stream_blocks", "stream_output"]:
            binary = out / f"{component}-{name}-tests"
            execute(f"{component}-{name}-build", [*compiler, "--test", "--cfg", 'feature="cpp-oracle"',
                ROOT / f"tests/brotli_{component}.rs", "-o", binary])
            for graph, source in original.items():
                execute(f"{component}-{name}-{graph}", [binary], dict(os.environ, RUST_TEST_THREADS="2",
                    BROTLI_STREAM_BLOCKS_ORACLE=str(source), BROTLI_STREAM_BLOCKS_CAPTURE=str(out / f"{name}-{graph}")))
        for graph in original:
            capture = out / f"{name}-{graph}"
            assert (capture / "counts.txt").read_text() == "9405\n"
            assert sha(capture / "source.txt") == sha(capture / "rust.txt")
            report["gates"][f"{name}-{graph}"] = {
                "input_tests": 6, "block_tests": 2, "output_tests": 2, "cases": 9405,
                "observations_sha256": sha(capture / "source.txt"),
                "observation_bytes": (capture / "source.txt").stat().st_size,
                "requests_sha256": sha(capture / "requests.txt")}
    for key in ["observations_sha256", "observation_bytes", "requests_sha256"]:
        assert len({row[key] for row in report["gates"].values()}) == 1
    (out / "clippy.toml").write_text('msrv = "1.85"\n')
    for component in ["stream_input", "stream_blocks", "stream_output"]:
        execute(f"{component}-clippy", ["clippy-driver", "--edition=2024", "--test", "--cfg", 'feature="cpp-oracle"',
            ROOT / f"tests/brotli_{component}.rs", "-D", "warnings", "-o", out / f"{component}-clippy"],
            dict(os.environ, CLIPPY_CONF_DIR=str(out)))
    (out / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(out / "report.json")


if __name__ == "__main__":
    main()
