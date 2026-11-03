#!/usr/bin/env python3
"""Capture real pinned Brotli allocation/layout/common-table observations.

This focused source audit neither builds nor substitutes a decoder. Both
arguments must be the observer linked to the pristine CMake Brotli targets.
"""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re
import subprocess

ROOT = Path(__file__).resolve().parent.parent
PIN = "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def run(binary, requests):
    process = subprocess.run([str(binary)], input="\n".join(requests)+"\n",
                             text=True, capture_output=True, check=True)
    assert not process.stderr, process.stderr
    lines = process.stdout.splitlines()
    assert len(lines) == len(requests), (len(lines), len(requests))
    return lines


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ordinary", required=True, type=Path)
    parser.add_argument("--sanitized", required=True, type=Path)
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    source = ROOT / "upstream/libjxl/third_party/brotli"
    git = lambda *a: subprocess.check_output(["git", "-C", str(source), *a], text=True).strip()
    assert git("rev-parse", "HEAD") == PIN
    assert git("status", "--porcelain", "--untracked-files=no") == ""
    configurations = {}
    for name, binary, sanitized in [("ordinary", args.ordinary, False),
                                    ("sanitized", args.sanitized, True)]:
        binary = binary.resolve()
        commands = json.loads((binary.parent / "compile_commands.json").read_text())
        originals = [c for c in commands if str(source) in c["file"]]
        assert originals and any(c["file"].endswith("/c/dec/decode.c") for c in originals)
        assert any(c["file"].endswith("/c/dec/state.c") for c in originals)
        assert any(c["file"].endswith("/c/common/shared_dictionary.c") for c in originals)
        for command in originals:
            flags = command["command"]
            assert ("-fsanitize=address,undefined" in flags) == sanitized, flags
            if sanitized:
                assert "-fno-sanitize-recover=all" in flags, flags
            assert "BROTLI_EXPERIMENTAL" not in flags, flags
            assert "BROTLI_EXTERNAL_DICTIONARY_DATA" not in flags, flags
            assert "BROTLI_STATIC_INIT" not in flags, flags
            assert "-O3" in flags and "-DNDEBUG" in flags, flags
        symbols = subprocess.check_output(["nm", "-g", str(binary)], text=True)
        for symbol in ["BrotliDecoderCreateInstance", "BrotliDecoderDestroyInstance",
                       "BrotliDecoderDecompressStream", "BrotliDecoderSetParameter",
                       "BrotliDecoderAttachDictionary", "BrotliEncoderCompress",
                       "BrotliGetDictionary", "BrotliGetTransforms"]:
            assert len(re.findall(rf"\b[Tt] _?{symbol}$", symbols, re.M)) == 1, symbol
        (args.output / f"{name}.symbols.txt").write_text(symbols)
        configurations[name] = {"binary": str(binary), "binary_sha256": sha(binary),
                                "compile_commands_sha256": sha(binary.parent / "compile_commands.json"),
                                "source_units": len(originals)}
    encoder_requests = []
    payloads = [b"", b"a", b"hello world\n", bytes(range(256)),
                b"time down life left back code data show only site city open just like free work text year over body love form book play live line\n" * 200,
                bytes((i * 97 + i // 13) & 255 for i in range(8192)), b"x" * 65536]
    for quality in [0, 1, 4, 9, 11]:
        for payload in payloads:
            encoder_requests.append(f"br-compress {quality} 16 0 {payload.hex() or '-'}")
    encoded = run(args.ordinary, encoder_requests)
    assert run(args.sanitized, encoder_requests) == encoded
    assert all(line.startswith("1 ") for line in encoded)
    streams = [line.split()[1] for line in encoded]
    requests = ["br-layout", "br-tables"]
    requests += [f"br-alloc {fail} {pair} 0 0 -" for pair in range(4) for fail in range(5)]
    requests += [f"br-alloc {fail} 3 1 0 00010203040506070809" for fail in range(7)]
    for stream in streams:
        for fail in range(13):
            requests.append(f"br-alloc {fail} 3 2 70000 {stream}")
        for capacity in [0, 1, 7, 64, 511, 8192]:
            requests.append(f"br-alloc 0 3 2 {capacity} {stream}")
        raw = bytes.fromhex(stream)
        for length in sorted({0, 1, len(raw)//2, len(raw)-1} - {-1}):
            requests.append(f"br-alloc 0 3 2 17 {raw[:length].hex() or '-'}")
    requests += [f"br-alloc 0 3 2 17 {byte:02x}" for byte in range(256)]
    # Keep every old request in its original position and append the actual
    # constructor's partially initialized packed-word observations.
    assert len(requests) == 1080
    requests += [f"br-flags-create {seed}" for seed in range(256)]
    ordinary = run(args.ordinary, requests)
    sanitized = run(args.sanitized, requests)
    (args.output / "requests.txt").write_text("\n".join(requests)+"\n")
    (args.output / "encoder-requests.txt").write_text("\n".join(encoder_requests)+"\n")
    (args.output / "encoder-outputs.txt").write_text("\n".join(encoded)+"\n")
    (args.output / "ordinary.txt").write_text("\n".join(ordinary)+"\n")
    (args.output / "sanitized.txt").write_text("\n".join(sanitized)+"\n")
    assert ordinary == sanitized, [i for i, pair in enumerate(zip(ordinary, sanitized)) if pair[0] != pair[1]]
    # The generated literals must agree with actual original exported objects.
    table_words = ordinary[1].split()
    import generate_brotli_tables as tables
    generated = tables.outputs()
    assert generated[ROOT / "src/brotli/dictionary.bin"].hex() == table_words[0]
    rust = generated[ROOT / "src/brotli/tables.rs"].decode()
    def rust_array(name):
        body = re.search(rf"pub static {name}: \[[^;]*; \d+\] = \[(.*?)\];", rust, re.S)[1]
        return tables.numbers(body)
    for name, word in zip(["CONTEXT_LOOKUP", "PREFIX_SUFFIX", "TRANSFORMS_DATA"], table_words[1:4]):
        assert bytes(rust_array(name)).hex() == word, name
    assert rust_array("PREFIX_SUFFIX_MAP") == list(map(int, table_words[4:54]))
    entries = [tables.numbers(e) for e in re.findall(r"(?m)^    CmdLutElement \{([^{}]*)\}", rust, re.S)]
    assert entries == [list(map(int, e.split(':'))) for e in table_words[54:]]
    report = {"pin": PIN, "configurations": configurations,
              "request_count": len(requests), "encoder_requests": len(encoder_requests),
              "ordinary_sanitized_mismatches": 0, "layout": ordinary[0].split(),
              "result_histogram": dict(Counter(re.search(r"\bR([^:]+)", line)[1]
                  for line in ordinary if re.search(r"\bR([^:]+)", line))),
              "source_tables_exact": True,
              "source_initial_flags": {str(seed): ordinary[1080+seed].split()[0]
                                       for seed in range(256)},
              "files": {p.name: sha(p) for p in args.output.iterdir() if p.is_file()}}
    (args.output / "report.json").write_text(json.dumps(report, indent=2)+"\n")
    print(json.dumps({"requests": len(requests), "encodes": len(encoded), "mismatches": 0}))


if __name__ == "__main__":
    main()
