#!/usr/bin/env python3
"""Exact original/Rust dictionary transforms over live raw destination blocks."""
import argparse
import hashlib
import json
from pathlib import Path
import re
import struct
import subprocess

ROOT = Path(__file__).resolve().parent.parent
PIN = "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"
COUNT = 13504 * 121


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def capture(binary, mode, path):
    with path.open("wb") as output:
        result = subprocess.run([str(binary), mode], stdout=output, stderr=subprocess.PIPE, check=False)
    assert result.returncode == 0 and not result.stderr, (binary, mode, result.returncode, result.stderr)


def exact_files(left, right):
    with left.open("rb") as a, right.open("rb") as b:
        offset = 0
        while True:
            x, y = a.read(1 << 20), b.read(1 << 20)
            assert x == y, (left, right, offset)
            if not x:
                break
            offset += len(x)


def initialized_prefix_proof(seeded, unknown):
    # Inspect every record, rather than comparing digests or a few samples.
    # The source defines all64 seeded bytes and only the returned prefix in
    # unknown mode. Never request/observe a still-unknown tail as an integer.
    with seeded.open("rb") as a, unknown.open("rb") as b:
        for index in range(COUNT):
            known_len = a.read(4)
            raw_len = b.read(4)
            assert len(known_len) == len(raw_len) == 4, index
            assert known_len == raw_len, index
            length = struct.unpack("<i", known_len)[0]
            assert 0 <= length <= 64, (index, length)
            full = a.read(64)
            prefix = b.read(length)
            assert len(full) == 64 and len(prefix) == length, index
            assert full[:length] == prefix, index
        assert not a.read(1) and not b.read(1)


def source_binding(binary, sanitized):
    commands_path = binary.parent / "compile_commands.json"
    commands = json.loads(commands_path.read_text())
    dictionary_units = [entry for entry in commands
                        if entry["file"].endswith("/c/common/dictionary.c")]
    assert len(dictionary_units) == 1
    compiled_source = Path(dictionary_units[0]["file"]).resolve().parents[2]
    assert subprocess.check_output(["git", "-C", str(compiled_source), "rev-parse", "HEAD"], text=True).strip() == PIN
    assert not subprocess.check_output(["git", "-C", str(compiled_source), "status", "--porcelain", "--untracked-files=no"], text=True).strip()
    # A private pristine clone may differ from this audit driver's clone.
    units = [entry for entry in commands if str(compiled_source) in entry["file"]]
    assert units
    for entry in units:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command
        assert ("-fsanitize=address,undefined" in command) == sanitized
        if sanitized:
            assert "-fno-sanitize-recover=all" in command
    for name in ["dictionary.c", "transform.c"]:
        assert any(entry["file"].endswith("/c/common/" + name) for entry in units)
    symbol_text = subprocess.check_output(["nm", str(binary)], text=True)
    for name in ["BrotliGetDictionary", "BrotliGetTransforms", "BrotliTransformDictionaryWord"]:
        assert len(re.findall(rf"\b[Tt] _?{name}$", symbol_text, re.M)) == 1
    map_path = binary.parent / "brotli_raw_transforms.map"
    link_map = map_path.read_text()
    for name in ["dictionary.c.o", "transform.c.o"]:
        assert name in link_map
    return {"binary_sha256": sha(binary), "compile_commands_sha256": sha(commands_path),
            "map_sha256": sha(map_path), "pristine_c_units": len(units),
            "compiled_source": str(compiled_source)}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--ordinary", required=True, type=Path)
    parser.add_argument("--sanitized", required=True, type=Path)
    parser.add_argument("--rust", required=True, action="append", help="configuration=binary")
    parser.add_argument("--frozen-seeded", type=Path, help="Optional prior exact builtin source stream")
    parser.add_argument("--output", required=True, type=Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    source = ROOT / "upstream/libjxl/third_party/brotli"
    assert subprocess.check_output(["git", "-C", str(source), "rev-parse", "HEAD"], text=True).strip() == PIN
    assert not subprocess.check_output(["git", "-C", str(source), "status", "--porcelain", "--untracked-files=no"], text=True).strip()
    binaries = [("ordinary", args.ordinary.resolve()), ("sanitized", args.sanitized.resolve())]
    binaries += [(name, Path(path).resolve()) for name, path in (x.split("=", 1) for x in args.rust)]
    assert len({name for name, _ in binaries}) == len(binaries)
    report = {"source_pin": PIN, "records_per_mode": COUNT, "modes": ["seeded", "unknown"],
              "source_binding": {}, "configurations": {}, "native_core_admission": False}
    for name, binary in binaries:
        if name in ["ordinary", "sanitized"]:
            report["source_binding"][name] = source_binding(binary, name == "sanitized")
        files = {}
        for mode in ["seeded", "unknown"]:
            path = args.output / f"{name}-{mode}.bin"
            capture(binary, mode, path)
            files[mode] = {"bytes": path.stat().st_size, "sha256": sha(path)}
            if name != "ordinary":
                exact_files(args.output / f"ordinary-{mode}.bin", path)
        initialized_prefix_proof(args.output / f"{name}-seeded.bin", args.output / f"{name}-unknown.bin")
        report["configurations"][name] = {"binary_sha256": sha(binary), "files": files}
    if args.frozen_seeded:
        exact_files(args.frozen_seeded, args.output / "ordinary-seeded.bin")
        report["frozen_initialized_stream_sha256"] = sha(args.frozen_seeded)
    report["all_exact"] = True
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"records_per_mode": COUNT, "configurations": list(report["configurations"]), "all_exact": True}))


if __name__ == "__main__":
    main()
