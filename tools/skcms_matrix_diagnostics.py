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
    payload = payload.replace(b"skcms-concat ", b"skcms-concat-archive ")
    with output.open("wb") as stream:
        result = subprocess.run([str(binary)], input=payload, stdout=stream, stderr=subprocess.PIPE)
    assert result.returncode == 0 and not result.stderr, (binary, result.returncode, result.stderr[:1500])
    return output.read_text().splitlines()


def paired_nan_product_requests():
    """Exercise both input NaNs at each of the 27 concatenation products.

    These are defined source requests, not UB diagnostics or expected results.
    Keep each output cell and product visible independently: all other input
    entries are zero. The original graph chooses the actual output words.
    """
    words = [0x7fc12345, 0xffc23456, 0x7f812345, 0xff823456]
    requests = []
    for row in range(3):
        for column in range(3):
            for term in range(3):
                for first in words:
                    for second in words:
                        a, b = [0] * 9, [0] * 9
                        a[3 * row + term] = first
                        b[3 * term + column] = second
                        requests.append("skcms-concat 0 " + " ".join(str(x) for x in a + b))
    assert len(requests) == 432
    return requests


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    output = args.output.resolve()
    output.mkdir(parents=True, exist_ok=False)
    retained = [q for q in retained_matrix_requests() if q.startswith("skcms-concat ")]
    assert len(retained) == 24324
    paired = paired_nan_product_requests()
    fixture = ROOT / "tests/data/skcms-concat-paired-nan.requests"
    assert fixture.read_text().splitlines() == paired
    requests = retained + paired
    payload = ("\n".join(requests) + "\n").encode()
    (output / "requests.txt").write_bytes(payload)
    report = {"purpose": "diagnostic evidence; differences are not acceptance", "requests": len(requests),
              "request_sha256": sha(output / "requests.txt"), "platform": platform.platform(),
              "retained_requests": len(retained), "paired_nan_product_requests": len(paired),
              "retained_request_sha256": hashlib.sha256(("\n".join(retained) + "\n").encode()).hexdigest(),
              "graphs": {}, "differences": {}}
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
            report["differences"]["retained_ordinary_vs_instrumented"] = sum(
                d["index"] < len(retained) for d in differences)
            report["differences"]["paired_nan_ordinary_vs_instrumented"] = sum(
                d["index"] >= len(retained) for d in differences)
    for name, chain, release in [("debug", [], False), ("release", [], True), ("msrv", ["+1.95"], False)]:
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
