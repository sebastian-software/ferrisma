#!/usr/bin/env python3
"""Capture complete initialized destinations from actual pinned Brotli exports."""

import argparse
import hashlib
import json
import pathlib
import re
import subprocess


ROOT = pathlib.Path(__file__).resolve().parents[3]
PINS = {
    "upstream/libjxl": "a16d32d0ac113f5e54125e49bb571281c5d0b704",
    "upstream/libjxl/third_party/highway": "457c891775a7397bdb0376bb1031e6e027af1c48",
    "upstream/libjxl/third_party/brotli": "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d",
    "upstream/libjxl/third_party/skcms": "96d9171c94b937a1b5f0293de7309ac16311b722",
}
RECORD_SIZE = 68


def digest(path):
    result = hashlib.sha256()
    with path.open("rb") as stream:
        for chunk in iter(lambda: stream.read(1 << 20), b""):
            result.update(chunk)
    return result.hexdigest()


def assert_exact(first, second):
    with first.open("rb") as a, second.open("rb") as b:
        while True:
            av, bv = a.read(1 << 20), b.read(1 << 20)
            assert av == bv, (first, second)
            if not av:
                return


def audit_build(build, sanitized):
    commands = json.loads((build / "compile_commands.json").read_text())
    entries = [entry for entry in commands if "/brotli/c/common/" in entry["file"] or entry["file"].endswith("brotli_transforms.cc")]
    assert len(entries) == 7
    for entry in entries:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command, entry
        assert ("-fsanitize=address,undefined" in command) == sanitized, entry
        if sanitized:
            assert "-fno-sanitize-recover=all" in command, entry
        assert not re.search(r"(?:-O0|-fno-sanitize=|-DBROTLI_EXTERNAL_DICTIONARY_DATA|-DBROTLI_EXPERIMENTAL)", command), entry
    link_map = (build / "brotli_transforms.map").read_text()
    bindings = {}
    for function, source in [("BrotliGetDictionary", "dictionary"), ("BrotliSetDictionaryData", "dictionary"), ("BrotliGetTransforms", "transform"), ("BrotliTransformDictionaryWord", "transform")]:
        if "# Object files:" in link_map:
            owner = re.search(rf"\[\s*(\d+)\]\s+brotli/libbrotlicommon\.a\({source}\.c\.o\)", link_map).group(1)
            bindings[function] = next(line for line in link_map.splitlines() if re.search(rf"\[\s*{owner}\]\s+_{function}$", line))
        else:
            assert function in link_map and f"libbrotlicommon.a({source}.c.o)" in link_map
            bindings[function] = f"brotli/libbrotlicommon.a({source}.c.o)"
    return {"source_compile_commands": entries, "bindings": bindings, "map_sha256": digest(build / "brotli_transforms.map"), "executable_sha256": digest(build / "brotli_transforms_oracle")}


def capture(build, mode, target, data=None):
    with target.open("wb") as output:
        result = subprocess.run([str(build / "brotli_transforms_oracle"), mode], input=data, stdout=output, stderr=subprocess.PIPE, check=True)
    assert not result.stderr, result.stderr
    return {"bytes": target.stat().st_size, "sha256": digest(target)}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--plain", required=True, type=pathlib.Path)
    parser.add_argument("--asan", required=True, type=pathlib.Path)
    parser.add_argument("--output", required=True, type=pathlib.Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    for relative, pin in PINS.items():
        checkout = ROOT / relative
        assert subprocess.check_output(["git", "-C", str(checkout), "rev-parse", "HEAD"], text=True).strip() == pin
        assert not subprocess.check_output(["git", "-C", str(checkout), "status", "--porcelain"])
    report = {"pins": PINS, "record_size": RECORD_SIZE, "record_layout": "little-endian signed i32 returned length, followed by all 64 initialized destination bytes", "builds": {}}
    for name, build, sanitized in [("plain", args.plain, False), ("asan", args.asan, True)]:
        record = audit_build(build, sanitized)
        for mode, records in [("builtin", 1633984), ("custom-corpus", 554496), ("wide", 4320), ("metadata", None)]:
            target = args.output / f"{name}-{mode}.bin"
            record[mode] = capture(build, mode, target)
            if records is not None:
                assert target.stat().st_size == records * (1028 if mode == "wide" else RECORD_SIZE)
                record[mode]["records"] = records
                record[mode]["record_size"] = 1028 if mode == "wide" else RECORD_SIZE
        report["builds"][name] = record
    for mode in ["builtin", "custom-corpus", "wide", "metadata"]:
        assert_exact(args.output / f"plain-{mode}.bin", args.output / f"asan-{mode}.bin")
    metadata = (args.output / "plain-metadata.bin").read_bytes()
    assert metadata[:8] == b"\1\0\0\0\1\0\0\0"
    bits = [int.from_bytes(metadata[8 + i * 4:12 + i * 4], "little") for i in range(32)]
    offsets_start = 8 + 32 * 4
    offsets = [int.from_bytes(metadata[offsets_start + i * 4:offsets_start + 4 + i * 4], "little") for i in range(32)]
    data_start = offsets_start + 32 * 4 + 4
    dictionary_data = metadata[data_start:data_start + 122784]
    assert dictionary_data == (ROOT / "src/brotli/dictionary.bin").read_bytes()
    bad = []
    word_count = 0
    for length in range(4, 25):
        for index in range(1 << bits[length]):
            word_count += 1
            start = offsets[length] + index * length
            word = dictionary_data[start:start + length]
            try:
                word.decode("utf-8", errors="strict")
            except UnicodeDecodeError:
                bad.append({"length": length, "index": index, "word_hex": word.hex()})
    assert word_count == 13504 and len(bad) == 13
    payload = "".join(f"{item['length']} {item['index']}\n" for item in bad).encode()
    (args.output / "malformed-word-requests.txt").write_bytes(payload)
    for name, build in [("plain", args.plain), ("asan", args.asan)]:
        record = capture(build, "words", args.output / f"{name}-malformed-words.bin", payload)
        assert record["bytes"] == 13 * 121 * RECORD_SIZE
        report["builds"][name]["malformed_words"] = record
    assert_exact(args.output / "plain-malformed-words.bin", args.output / "asan-malformed-words.bin")
    malformed = (args.output / "plain-malformed-words.bin").read_bytes()
    for i, word in enumerate(bad):
        offset = (i * 121 + 44) * RECORD_SIZE
        record = malformed[offset:offset + RECORD_SIZE]
        returned_length = int.from_bytes(record[:4], "little", signed=True)
        destination = record[4:]
        seed = word["length"] * 11 + word["index"] * 3 + 44 * 17
        initialized = bytes((seed + j * 37) & 255 for j in range(64))
        word.update({"transform": 44, "returned_length": returned_length, "complete_destination_hex": destination.hex(), "initialized_destination_hex": initialized.hex(), "tail_changes": [{"index": j, "before": initialized[j], "after": destination[j]} for j in range(returned_length, 64) if initialized[j] != destination[j]]})
    report.update({"word_count": word_count, "builtin_tuples": 1633984, "custom_cases": 554496, "wide_cases": 4320, "malformed_words": bad, "ordinary_asan_exact": True})
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["word_count", "builtin_tuples", "custom_cases", "ordinary_asan_exact"]}))


if __name__ == "__main__":
    main()
