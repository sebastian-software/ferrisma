#!/usr/bin/env python3
"""Retain native original matrix observations and actual source assembly.

This is diagnostic evidence: Rust or ordinary/instrumented source differences
are recorded in full and never relabeled as parity. No source graph is changed.
"""
import argparse
import hashlib
import json
from pathlib import Path
import platform
import subprocess
from skcms_reference import configuration, retained_matrix_requests

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def capture(binary, payload, output):
    with output.open("wb") as stream:
        result = subprocess.run([str(binary)], input=payload, stdout=stream, stderr=subprocess.PIPE)
    assert result.returncode == 0 and not result.stderr, (binary, result.returncode, result.stderr[:1500])
    return output.read_text().splitlines()


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    requests = [q for q in retained_matrix_requests() if q.startswith("skcms-concat ")]
    payload = ("\n".join(requests) + "\n").encode()
    (output / "requests.txt").write_bytes(payload)
    report = {"purpose": "diagnostic evidence; differences are not acceptance", "requests": len(requests),
              "request_sha256": sha(output / "requests.txt"), "platform": platform.platform(), "graphs": {}, "differences": {}}
    original = None
    for graph in ["oracle", "oracle-asan"]:
        directory = output / graph
        directory.mkdir()
        binding = configuration(ROOT / "target" / graph, directory)
        path = directory / "observations.txt"
        rows = capture(ROOT / "target" / graph / "jxl_oracle", payload, path)
        assert len(rows) == len(requests)
        report["graphs"][graph] = {"binding": binding, "observations_sha256": sha(path)}
        if original is None:
            original = rows
        else:
            differences = [{"index": i, "request": q, "ordinary": a, "instrumented": b}
                           for i, (q, a, b) in enumerate(zip(requests, original, rows)) if a != b]
            (output / "ordinary-instrumented-differences.json").write_text(json.dumps(differences, indent=2) + "\n")
            report["differences"]["ordinary_vs_instrumented"] = len(differences)
    for name, chain, release in [("debug", [], False), ("release", [], True), ("msrv", ["+1.85"], False)]:
        command = ["cargo", *chain, "build", "--locked", "--offline", "--lib"] + (["--release"] if release else [])
        with (output / (name + "-library-build.log")).open("w") as log:
            subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=True)
        profile = "release" if release else "debug"
        binary = output / ("rust-" + name)
        command = ["rustc", *chain, "--edition=2024", str(ROOT / "tools/skcms_matrix_probe.rs"),
                   "--extern", "jxl=" + str(ROOT / "target" / profile / "libjxl.rlib"),
                   "-L", "dependency=" + str(ROOT / "target" / profile / "deps"), "-o", str(binary)]
        with (output / (name + "-probe-build.log")).open("w") as log:
            subprocess.run(command, cwd=ROOT, stdout=log, stderr=subprocess.STDOUT, check=True)
        path = output / (name + ".txt")
        rows = capture(binary, payload, path)
        assert len(rows) == len(requests)
        differences = [{"index": i, "request": q, "original": a, "rust": b}
                       for i, (q, a, b) in enumerate(zip(requests, original, rows)) if a != b]
        (output / (name + "-differences.json")).write_text(json.dumps(differences, indent=2) + "\n")
        report["differences"][name] = len(differences)
        report["graphs"][name] = {"observations_sha256": sha(path), "binary_sha256": sha(binary)}
    for graph in ["oracle", "oracle-asan"]:
        for relative, expected in report["graphs"][graph]["binding"]["active_hashes"].items():
            assert sha(ROOT / relative) == expected, relative
    (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"requests": len(requests), "differences": report["differences"],
                      "diagnostic_only": True}), flush=True)


if __name__ == "__main__":
    main()
