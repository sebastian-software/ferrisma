#!/usr/bin/env python3
"""Audit/capture actual C decoder status exports and pinned numeric declarations."""

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


def unknown_values():
    values = set(range(-131072, 131073))
    values.update([-2147483648, -2147483647, 2147483646, 2147483647])
    for bit in range(31):
        value = 1 << bit
        values.update([value - 1, value, value + 1, -value - 1, -value, -value + 1])
    seed = 78191
    for _ in range(32768):
        seed = (seed * 1664525 + 1013904223) & 0xFFFFFFFF
        values.add(seed if seed < (1 << 31) else seed - (1 << 32))
    return sorted(values)


def audit_build(build, sanitized):
    entries = json.loads((build / "compile_commands.json").read_text())
    entries = [entry for entry in entries if "/brotli/c/" in entry["file"] or entry["file"].endswith("/brotli_status.c")]
    assert any(entry["file"].endswith("/brotli_status.c") for entry in entries)
    for entry in entries:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command, entry
        assert entry["file"].endswith(".c")
        assert not re.search(r"(?:-x\s*c\+\+|-O0|-fno-sanitize=|-fshort-enums)", command), entry
        assert ("-fsanitize=address,undefined" in command) == sanitized, entry
        if sanitized:
            assert "-fno-sanitize-recover=all" in command, entry
    link_map = (build / "brotli_status.map").read_text()
    bindings = {}
    for function in ["BrotliDecoderErrorString", "BrotliDecoderVersion"]:
        if "# Object files:" in link_map:
            owner = re.search(r"\[\s*(\d+)\]\s+brotli/libbrotlidec\.a\(decode\.c\.o\)", link_map).group(1)
            bindings[function] = next(line for line in link_map.splitlines() if re.search(rf"\[\s*{owner}\]\s+_{function}$", line))
        else:
            assert function in link_map and "libbrotlidec.a(decode.c.o)" in link_map
            bindings[function] = "brotli/libbrotlidec.a(decode.c.o)"
    return {"source_compile_commands": entries, "bindings": bindings, "map_sha256": digest(link_map.encode()), "executable_sha256": digest((build / "brotli_status_oracle").read_bytes())}


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
    source = ROOT / "upstream/libjxl/third_party/brotli/c"
    header = (source / "include/brotli/decode.h").read_text()
    state = (source / "dec/state.h").read_text()
    codes = re.findall(r"BROTLI_ERROR_CODE\((_[A-Z_]*|_), ([A-Z_0-9]+), (-?\d+)\)", header)
    groups = re.findall(r"typedef enum \{([^}]+)\}\s+(BrotliRunning\w+);", state)
    assert len(codes) == 31 and len(groups) == 8
    values = unknown_values()
    assert len(values) == 294992
    payload = "".join(f"{value}\n" for value in values).encode()
    (args.output / "requests.txt").write_bytes(payload)
    report = {"pins": PINS, "requests": len(values), "request_sha256": digest(payload), "source_hashes": {str(path.relative_to(ROOT)): digest(path.read_bytes()) for path in [source / "include/brotli/decode.h", source / "dec/state.h", source / "dec/decode.c", source / "common/version.h"]}, "builds": {}}
    captures = {}
    for name, build, sanitized in [("plain", args.plain, False), ("asan", args.asan, True)]:
        record = audit_build(build, sanitized)
        captures[name] = {}
        for mode, data in [("declarations", None), ("strings", payload)]:
            process = subprocess.run([str(build / "brotli_status_oracle"), mode], input=data, capture_output=True, check=True)
            assert not process.stderr, process.stderr
            (args.output / f"{name}-{mode}.txt").write_bytes(process.stdout)
            captures[name][mode] = process.stdout
            record[mode + "_sha256"] = digest(process.stdout)
        declarations = captures[name]["declarations"].decode().splitlines()
        assert len(declarations) == 102
        for prefix, code_name, code in codes:
            assert f"code BROTLI_DECODER{prefix}{code_name} {code} {prefix}{code_name}" in declarations
        for body, kind in groups:
            assert f"layout {kind} 4 4 0" in declarations
            for value, constant in enumerate(item.strip() for item in body.strip().split(",")):
                assert f"running {kind} {constant} {value}" in declarations
        assert len(captures[name]["strings"].splitlines()) == len(values)
        report["builds"][name] = record
    assert captures["plain"] == captures["asan"]
    report.update({"declaration_observations": 102, "declared_error_codes": 31, "running_constants": 55, "ordinary_asan_exact": True, "unknown_enum_boundary": "int32_t converted to representable signed32 C enum in a C translation unit; no C++ enum cast"})
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["requests", "declaration_observations", "ordinary_asan_exact"]}))


if __name__ == "__main__":
    main()
