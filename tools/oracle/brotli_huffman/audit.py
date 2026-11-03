#!/usr/bin/env python3
"""Capture real pinned Brotli helpers and audit their native source bindings."""

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


def digest(data):
    return hashlib.sha256(data).hexdigest()


def requests():
    result = ["constants"]
    for p in range(4):
        for d in range(0, (15 << p) + 1, 1 << p):
            for distance in [0, 1, 2, 7, 16, 128, 65535, 0x3FFFFFC, 0x7FFFFFFC, 0xFFFFFFFB]:
                result.append(f"distance {distance} {p} {d}")
    for n, minimum in enumerate([0, 1, 2, 2, 3, 0]):
        for root in range(minimum, 9):
            for values in [[0, 0, 0, 0], [65535, 0, 256, 17], [3, 2, 1, 0], [1, 3, 0, 2]]:
                result.append(f"simple {root} {n} " + " ".join(map(str, values)))
    for symbol in range(18):
        for depth in [1, 3, 5]:
            lengths = [0] * 18
            lengths[symbol] = depth
            result.append("lengths " + " ".join(map(str, lengths)))
    for root in [7, 8]:
        for lengths in [[1], [2, 2], [3, 0, 3], [15], [15, 0, 0, 15]]:
            result.append(f"general {root} {len(lengths)} " + " ".join(map(str, lengths)))
        for depth in range(1, 16):
            lengths = [depth] * (1 << depth)
            result.append(f"general {root} {len(lengths)} " + " ".join(map(str, lengths)))
        for depth in range(2, 16):
            lengths = list(range(1, depth)) + [depth, depth]
            result.append(f"general {root} {len(lengths)} " + " ".join(map(str, lengths)))
    for lengths in [[1, 1] + [0] * 16, [1, 2, 3, 4, 5, 5] + [0] * 12, [2, 3, 4, 5, 5] + [0] * 13]:
        result.append("lengths " + " ".join(map(str, lengths)))
    result += [case.replace("general ", "general-accessor ", 1) for case in result if case.startswith("general ")]
    for postfix in list(range(31)) + [0xFFFFFFFF]:
        for direct in [0, 1, 15, 120, 0xFFFFFFEF, 0xFFFFFFF0, 0xFFFFFFFF]:
            for bits in [0, 1, 24, 62, 0xFFFFFFFF]:
                result.append(f"alphabet {postfix} {direct} {bits}")
    return result


def audit_build(build, sanitized):
    commands = json.loads((build / "compile_commands.json").read_text())
    entries = [entry for entry in commands if "/brotli/c/" in entry["file"]]
    assert entries
    for entry in entries:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command, entry
        assert ("-fsanitize=address,undefined" in command) == sanitized, entry
        if sanitized:
            assert "-fno-sanitize-recover=all" in command, entry
        assert not re.search(r"(?:-O0|-fno-sanitize=|-DBROTLI_BUILD_NO_RBIT|-DBROTLI_EXPERIMENTAL)", command), entry
    link_map = (build / "brotli_huffman.map").read_text()
    binding = {}
    if "# Object files:" in link_map:
        owner = re.search(r"\[\s*(\d+)\]\s+brotli/libbrotlidec\.a\(huffman\.c\.o\)", link_map).group(1)
        for function in ["BrotliBuildCodeLengthsHuffmanTable", "BrotliBuildHuffmanTable", "BrotliBuildSimpleHuffmanTable"]:
            line = next(line for line in link_map.splitlines() if re.search(rf"\[\s*{owner}\]\s+_{function}$", line))
            binding[function] = line
        assert "libbrotlicommon.a(constants.c.o)" in link_map
    else:
        for function in ["BrotliBuildCodeLengthsHuffmanTable", "BrotliBuildHuffmanTable", "BrotliBuildSimpleHuffmanTable"]:
            assert function in link_map
            binding[function] = "brotli/libbrotlidec.a(huffman.c.o)"
        assert "libbrotlidec.a(huffman.c.o)" in link_map
        assert "libbrotlicommon.a(constants.c.o)" in link_map
    return {"source_compile_commands": entries, "bindings": binding, "map_sha256": digest(link_map.encode())}


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
    rust = (ROOT / "src/brotli/huffman.rs").read_text()
    start = rust.index("pub fn brotli_build_huffman_table_with_symbols(")
    brace = rust.index("{", start)
    cursor, depth = brace + 1, 1
    while depth:
        depth += (rust[cursor] == "{") - (rust[cursor] == "}")
        cursor += 1
    algorithm_digest = digest(rust[brace + 1:cursor - 1].encode())
    assert algorithm_digest == "fc6bde95fe89df41607a4c9e145c8ed55e2fcee41ecfe4934fd29dfa39aafbce"
    cases = requests()
    payload = ("\n".join(cases) + "\n").encode()
    (args.output / "requests.txt").write_bytes(payload)
    report = {"pins": PINS, "general_algorithm_body_sha256": algorithm_digest, "requests": len(cases), "request_sha256": digest(payload), "builds": {}}
    outputs = []
    for name, build, sanitized in [("plain", args.plain, False), ("asan", args.asan, True)]:
        record = audit_build(build, sanitized)
        process = subprocess.run([str(build / "brotli_huffman_oracle")], input=payload, capture_output=True, check=True)
        assert not process.stderr, process.stderr
        assert len(process.stdout.splitlines()) == len(cases)
        (args.output / f"{name}.txt").write_bytes(process.stdout)
        record["observation_sha256"] = digest(process.stdout)
        record["executable_sha256"] = digest((build / "brotli_huffman_oracle").read_bytes())
        report["builds"][name] = record
        outputs.append(process.stdout)
    assert outputs[0] == outputs[1]
    # This is a source precondition diagnosis, never a passing parity request.
    invalid = b"distance 4294967295 0 0\n"
    process = subprocess.run([str(args.asan / "brotli_huffman_oracle")], input=invalid, capture_output=True)
    assert process.returncode != 0
    assert b"constants.h" in process.stderr and b"shift exponent" in process.stderr, process.stderr
    (args.output / "undefined-distance.stderr").write_bytes(process.stderr)
    report["undefined_source_request"] = {"request": invalid.decode().strip(), "returncode": process.returncode, "diagnostic_sha256": digest(process.stderr)}
    alphabet_undefined = []
    for postfix in [31, 32, 0xFFFFFFFE]:
        request = f"alphabet {postfix} 0 1\n".encode()
        process = subprocess.run([str(args.asan / "brotli_huffman_oracle")], input=request, capture_output=True)
        assert process.returncode != 0
        assert b"shift exponent" in process.stderr and b"32-bit type" in process.stderr, process.stderr
        (args.output / f"undefined-alphabet-{postfix}.stderr").write_bytes(process.stderr)
        alphabet_undefined.append({"request": request.decode().strip(), "returncode": process.returncode, "diagnostic_sha256": digest(process.stderr)})
    report["undefined_alphabet_requests"] = alphabet_undefined
    report["ordinary_asan_exact"] = True
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["requests", "request_sha256", "ordinary_asan_exact"]}))


if __name__ == "__main__":
    main()
