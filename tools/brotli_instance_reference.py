#!/usr/bin/env python3
"""Untouched original lifecycle/configuration calls and exact Rust observations."""
import argparse
import hashlib
import json
from pathlib import Path
import subprocess

from brotli_decoder_binding import verify

PIN = "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"


def corpus():
    requests = []
    for seed in range(256):
        requests.append(f"bi-create 3 0 {seed} 3 4 0 0 1 0 0 1 1 1")
    for pair in range(4):
        for fail in range(4):
            for seed in [0, 165, 255]:
                for metadata in range(4):
                    requests.append(f"bi-create {pair} {fail} {seed} {metadata} 0")
    for pair in [0, 3]:
        for parameter in [0, 1, 2, 3, 0x7fffffff, 0xffffffff]:
            for value in [0, 1, 2, 255, 0x7fffffff, 0xffffffff]:
                requests.append(f"bi-create {pair} 0 165 3 5 0 0 1 0 {parameter} {value} 0 1 1 1")
    return requests


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def source_binding(binary, sanitized):
    expected = {name: "libbrotlidec.a(decode.c.o)" for name in [
        "BrotliDecoderCreateInstance", "BrotliDecoderDestroyInstance",
        "BrotliDecoderSetParameter", "BrotliDecoderIsUsed",
        "BrotliDecoderGetErrorCode", "BrotliDecoderSetMetadataCallbacks"]}
    expected.update({"BrotliDecoderStateInit": "libbrotlidec.a(state.c.o)",
                     "BrotliDecoderEnsureStaticInit": "libbrotlidec.a(static_init.c.o)",
                     "BrotliSharedDictionaryCreateInstance": "libbrotlicommon.a(shared_dictionary.c.o)",
                     "BrotliSharedDictionaryDestroyInstance": "libbrotlicommon.a(shared_dictionary.c.o)"})
    return verify(binary, sanitized, "brotli_instance.map",
                  ["brotli_instance.cc", "brotli_instance_settings.c"], expected)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ordinary", type=Path, required=True)
    parser.add_argument("--sanitized", type=Path, required=True)
    parser.add_argument("--rust", action="append", required=True)
    parser.add_argument("--output", type=Path, required=True)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    requests = corpus()
    payload = ("\n".join(requests) + "\n").encode()
    (args.output / "requests.txt").write_bytes(payload)
    binaries = [("ordinary", args.ordinary.resolve()), ("sanitized", args.sanitized.resolve())]
    binaries += [(name, Path(path).resolve()) for name, path in (x.split("=", 1) for x in args.rust)]
    assert len({name for name, _ in binaries}) == len(binaries)
    expected = None
    report = {"source_pin": PIN, "requests": len(requests), "configurations": {}}
    for name, binary in binaries:
        p = subprocess.run([binary], input=payload, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
        assert p.returncode == 0 and not p.stderr, (name, p.returncode, p.stderr)
        assert len(p.stdout.splitlines()) == len(requests)
        if expected is None:
            expected = p.stdout
        assert p.stdout == expected, name
        output = args.output / f"{name}.txt"
        output.write_bytes(p.stdout)
        report["configurations"][name] = {"binary_sha256": sha(binary), "output_sha256": sha(output),
            "source_binding": source_binding(binary, name == "sanitized") if name in ["ordinary", "sanitized"] else None}
    report["all_exact"] = True
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"requests": len(requests), "all_exact": True}))


if __name__ == "__main__":
    main()
