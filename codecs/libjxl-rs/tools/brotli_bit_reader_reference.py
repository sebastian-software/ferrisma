#!/usr/bin/env python3
"""Differential checkpoint/refill audit of actual pinned Brotli bit readers."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent


def corpus():
    result = []
    def add(op, val, bits, n, data):
        result.append(f"{op} {val} {bits} {n} {data.hex() or '-'}")
    positions = [0, 1, 3, 7, 8, 15, 16, 24, 31, 32, 33, 48, 55, 56, 63, 64]
    for size in [0, 1, 2, 3, 4, 7, 8, 16, 27, 28, 29, 32, 64]:
        data = bytes((i*97+size*13)&255 for i in range(size))
        for bits in positions:
            val = 0xd3a5c197eb024f68 & ((1<<bits)-1)
            add(0, val, bits, 0, data)
            if bits <= 56: add(1, val, bits, 0, data)
            for n in [1, 3, 8, 16, 17, 24, 25, 31, 32]:
                for op in [2, 4, 15, 21]: add(op, val, bits, n, data)
                if n <= 24: add(3, val, bits, n, data)
                if n > 24: add(5, val, bits, n, data)
                if n <= bits:
                    add(18, val, bits, n, data); add(19, val, bits, n, data)
            # Normalize/padding may observe existing spectre bits; no refill
            # overlaps them in these original operations.
            add(11, 0xd3a5c197eb024f68, bits, 0, data)
            add(13, 0xd3a5c197eb024f68, bits, 0, data)
            remaining = size + bits//8
            for n in sorted({0, 1, 2, bits//8, remaining//2, remaining}):
                if n <= remaining and (n <= bits//8 or bits%8 == 0):
                    add(14, val, bits, n, data)
            if size >= 8 and bits > 0:
                for n in [1, 8, 16, 24, 32]:
                    for op in [6, 7, 9, 10, 17, 24]: add(op, val, bits, n, data)
                    if n <= 24:
                        add(16, val, bits, n, data); add(23, val, bits, n, data)
                add(8, val, bits, 16, data); add(20, val, bits, 17, data)
        for n in [0, 1, 7, 8, 15, 16, 24, 25, 31, 32]: add(12, 0, 0, n, data)
    for byte in range(256):
        data = bytes([byte])*40
        add(0, 0, 0, 0, data)
        add(4, 0, 0, 32, data)
        add(14, 0, 0, 40, data)
    return result


def run(binary, requests):
    p = subprocess.run([str(binary)], input="\n".join(requests)+"\n", text=True,
                       capture_output=True, check=True)
    assert not p.stderr, p.stderr
    result = p.stdout.splitlines()
    assert len(result) == len(requests)
    return result


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--ordinary", required=True, type=Path)
    p.add_argument("--sanitized", required=True, type=Path)
    p.add_argument("--rust", action="append", required=True)
    p.add_argument("--output", required=True, type=Path)
    args = p.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    source = ROOT / "upstream/libjxl/third_party/brotli"
    assert subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip() == "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"
    assert not subprocess.check_output(["git", "-C", str(source), "status", "--porcelain", "--untracked-files=no"], text=True).strip()
    requests = corpus()
    (args.output / "requests.txt").write_text("\n".join(requests)+"\n")
    original = {}
    configs = {}
    sha = lambda path: hashlib.sha256(Path(path).read_bytes()).hexdigest()
    for name, binary, sanitized in [("ordinary", args.ordinary.resolve(), False), ("sanitized", args.sanitized.resolve(), True)]:
        commands = json.loads((binary.parent / "compile_commands.json").read_text())
        units = [c for c in commands if str(source) in c["file"]]
        assert any(c["file"].endswith("/c/dec/bit_reader.c") for c in units)
        for command in units:
            flags = command["command"]
            assert ("-fsanitize=address,undefined" in flags) == sanitized
            if sanitized: assert "-fno-sanitize-recover=all" in flags
            assert "-O3" in flags and "-DNDEBUG" in flags
            assert "BROTLI_BUILD_" not in flags
        # Original internal helpers are private-extern local t symbols after
        # Mach-O linking; -g would hide the actual pristine definitions.
        symbols = subprocess.check_output(["nm", str(binary)], text=True)
        for symbol in ["BrotliInitBitReader", "BrotliWarmupBitReader", "BrotliSafeReadBits32Slow"]:
            assert len(re.findall(rf"\b[Tt] _?{symbol}$", symbols, re.M)) == 1
        result = run(binary, requests)
        (args.output / f"{name}.txt").write_text("\n".join(result)+"\n")
        original[name] = result
        configs[name] = {"binary_sha256": sha(binary), "compile_commands_sha256": sha(binary.parent / "compile_commands.json")}
    assert original["ordinary"] == original["sanitized"]
    for name, binary in (entry.split("=", 1) for entry in args.rust):
        result = run(Path(binary), requests)
        (args.output / f"{name}.txt").write_text("\n".join(result)+"\n")
        if result != original["ordinary"]:
            mismatches = [i for i, pair in enumerate(zip(result, original["ordinary"])) if pair[0] != pair[1]]
            raise AssertionError([(i, requests[i], original["ordinary"][i], result[i]) for i in mismatches[:20]])
        configs[name] = {"binary_sha256": sha(binary), "requests_per_source_graph": len(requests)}
    report = {"requests": len(requests), "mismatches": 0, "configurations": configs,
              "files": {f.name: sha(f) for f in args.output.iterdir() if f.is_file()}}
    (args.output / "report.json").write_text(json.dumps(report, indent=2)+"\n")
    print(json.dumps({"requests": len(requests), "mismatches": 0}))


if __name__ == "__main__":
    main()
