#!/usr/bin/env python3
"""Actual source state bytes at allocation/free checkpoints, plus callable defaults.

Custom fixtures seed every observed byte and quarantine allocations after each
logical free, preserving unambiguous real owner identities. Only an observation
copy has process-relative pointer addresses normalized; source objects, source
algorithms, source callback arguments and sanitizer flags remain unchanged.
"""
import argparse
import hashlib
import json
from itertools import zip_longest
from pathlib import Path
import re
import shlex
import subprocess

ROOT = Path(__file__).resolve().parent.parent
PIN = "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d"


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def corpus():
    requests = [f"bs-state 0 {seed} 0 0 256 256 1" for seed in range(256)]
    for seed in [0, 1, 90, 165, 255]:
        for fail in [1, 2, 3]:
            requests.append(f"bs-state {fail} {seed} 0 0 256 256 1")
        for action in [1, 3]:
            for fail in [0, 1, 2]:
                requests.append(f"bs-state {fail} {seed} {action} 0 256 256 1")
    for group in range(3):
        for maximum, limit in [(1, 1), (256, 256), (704, 704), (65536, 65536)]:
            for ntrees in [0, 1, 2]:
                for fail in [0, 3]:
                    for seed in [0, 165]:
                        requests.append(f"bs-state {fail} {seed} 2 {group} {maximum} {limit} {ntrees}")
    # Unsigned source size arithmetic remains defined, even at wrapping
    # values. Fail the actual corresponding allocation before pointer/codes
    # construction could reach an invalid backing extent. No stream/case is
    # declared successful, filtered out, or given a fabricated source status.
    maximum = (1 << 64) - 1
    for group in range(3):
        for limit, ntrees in [(maximum, 1), (maximum, maximum), (0, maximum),
                              (1, maximum), (maximum - 375, maximum), (65536, 2)]:
            requests.append(f"bs-state 3 165 2 {group} {maximum} {limit} {ntrees}")
    for stored in [0, 1]:
        for size in [0, 1, 2, 7, 8, 15, 16, 31, 64, 511, 4096]:
            for fill in [0, 165, 255]:
                requests.append(f"bs-default {size} {fill} {stored}")
    # Preserve all preceding529 requests byte-for-byte; append actual native
    # descriptor bytes/static padding and max_align_t alignment observation.
    assert len(requests) == 529
    requests.append("bs-layout")
    return requests


def capture(binary, requests, path):
    with path.open("wb") as output:
        p = subprocess.run([str(binary)], input=("\n".join(requests) + "\n").encode(),
                           stdout=output, stderr=subprocess.PIPE)
    assert p.returncode == 0 and not p.stderr, (binary, p.returncode, p.stderr)
    with path.open("rb") as output:
        count = sum(1 for _ in output)
    assert count == len(requests), (binary, count)


def exact(left, right):
    with left.open("rb") as a, right.open("rb") as b:
        for index, (x, y) in enumerate(zip_longest(a, b, fillvalue=b"")):
            assert x == y, (index, x[:80], y[:80])


def compile_options(command, sanitized):
    """Check actual tokens, including contradictory options after required ones."""
    flags = command.get("arguments") or shlex.split(command["command"])
    assert [f for f in flags if f.startswith("-O")] == ["-O3"], command["file"]
    assert "-DNDEBUG" in flags, command["file"]
    sanitizer_flags = [f for f in flags if "sanitize" in f]
    expected = ["-fsanitize=address,undefined", "-fno-sanitize-recover=all"] if sanitized else []
    assert sanitizer_flags == expected, (command["file"], sanitizer_flags)
    if sanitized:
        assert "-fno-omit-frame-pointer" in flags, command["file"]
    forbidden = ["BROTLI_TEST", "BROTLI_EXPERIMENTAL", "BROTLI_EXTERNAL_DICTIONARY_DATA",
                 "BROTLI_STATIC_INIT", "-ffast-math", "-funsafe-math-optimizations",
                 "-ffinite-math-only", "-fassociative-math", "-freciprocal-math",
                 "-fno-signed-zeros", "-fno-trapping-math", "-ffp-contract=fast",
                 "-ffp-model=fast", "-fomit-frame-pointer"]
    for flag in flags:
        assert not any(f in flag for f in forbidden), (command["file"], flag)
        # Do not allow indirect options to hide a changed reference build.
        assert not flag.startswith("@") and flag not in ["-Xclang", "-mllvm", "-Xpreprocessor"], flag
    return flags


def live_map_bindings(mapped):
    """Resolve live function addresses to archive members, not map mentions.

    Mach-O names the contributing object on each live symbol row. GNU ld
    names it on the containing live text section; require the symbol address
    to lie in that section. Archive extraction and discarded-section lists
    are outside both parsers' live regions and cannot establish a binding.
    """
    found = {}
    if "# Object files:" in mapped:
        objects_text = mapped.split("# Object files:\n", 1)[1].split("# Sections:", 1)[0]
        objects = dict(re.findall(r"^\[\s*(\d+)\]\s+(.+)$", objects_text, re.M))
        live = mapped.split("# Symbols:\n", 1)[1].split("# Dead Stripped Symbols:", 1)[0]
        for address, size, index, symbol in re.findall(
                r"^(0x[0-9A-Fa-f]+)\s+(0x[0-9A-Fa-f]+)\s+\[\s*(\d+)\]\s+_?(Brotli\w+)\s*$", live, re.M):
            assert symbol not in found and int(size, 16) > 0, symbol
            found[symbol] = {"address": int(address, 16), "object": objects[index]}
    else:
        assert "Linker script and memory map\n" in mapped
        live = mapped.split("Linker script and memory map\n", 1)[1]
        section, owner = None, None
        for line in live.splitlines():
            # Both the one-line and wrapped per-function GNU section forms.
            start = re.match(r"^ (\.\S+)(?:\s+(.*))?$", line)
            if start:
                section, owner = start[1], None
            extent = re.match(r"^\s+(?:\.\S+\s+)?(0x[0-9a-fA-F]+)\s+(0x[0-9a-fA-F]+)\s+(\S+\.a\([^()]+\))\s*$", line)
            if extent and section and section.startswith(".text"):
                owner = (int(extent[1], 16), int(extent[2], 16), extent[3])
            symbol = re.match(r"^\s+(0x[0-9a-fA-F]+)\s+(Brotli\w+)\s*$", line)
            if symbol and owner:
                address = int(symbol[1], 16)
                assert owner[0] <= address < owner[0] + owner[1], (symbol[2], owner)
                assert symbol[2] not in found, symbol[2]
                found[symbol[2]] = {"address": address, "object": owner[2]}
            # Top-level sections and non-text input sections end attribution.
            if line and not line[0].isspace():
                section, owner = None, None
    assert found, "No live Brotli symbol bindings"
    return found


def binding(binary, sanitized):
    commands = json.loads((binary.parent / "compile_commands.json").read_text())
    state_units = [c for c in commands if c["file"].endswith("/c/dec/state.c")]
    assert len(state_units) == 1
    source = Path(state_units[0]["file"]).resolve().parents[2]
    assert subprocess.check_output(["git", "-C", source, "rev-parse", "HEAD"], text=True).strip() == PIN
    assert not subprocess.check_output(["git", "-C", source, "status", "--porcelain", "--untracked-files=no"], text=True).strip()
    units = [c for c in commands if Path(c["file"]).resolve().is_relative_to(source)]
    assert len(units) == 35, len(units)
    observer = [c for c in commands if Path(c["file"]).resolve() == ROOT / "tools/oracle/brotli_state.cc"]
    assert len(observer) == 1
    for c in [*units, *observer]:
        compile_options(c, sanitized)
    expected = {
        **{f"BrotliDecoderState{name}": "libbrotlidec.a(state.c.o)" for name in
           ["Init", "MetablockBegin", "CleanupAfterMetablock", "Cleanup"]},
        "BrotliDecoderHuffmanTreeGroupInit": "libbrotlidec.a(state.c.o)",
        "BrotliDecoderCreateInstance": "libbrotlidec.a(decode.c.o)",
        "BrotliDecoderDestroyInstance": "libbrotlidec.a(decode.c.o)",
        "BrotliDefaultAllocFunc": "libbrotlicommon.a(platform.c.o)",
        "BrotliDefaultFreeFunc": "libbrotlicommon.a(platform.c.o)",
        "BrotliSharedDictionaryCreateInstance": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliSharedDictionaryDestroyInstance": "libbrotlicommon.a(shared_dictionary.c.o)",
        "BrotliGetDictionary": "libbrotlicommon.a(dictionary.c.o)",
        "BrotliGetTransforms": "libbrotlicommon.a(transform.c.o)",
    }
    symbols = subprocess.check_output(["nm", str(binary)], text=True)
    link_map = binary.parent / "brotli_state.map"
    live = live_map_bindings(link_map.read_text())
    bindings = {}
    for symbol, object_file in expected.items():
        addresses = re.findall(rf"^([0-9a-fA-F]+) [Tt] _?{symbol}$", symbols, re.M)
        assert len(addresses) == 1, symbol
        actual = live[symbol]
        assert Path(actual["object"]).name == object_file, (symbol, actual, object_file)
        assert actual["address"] == int(addresses[0], 16), (symbol, actual, addresses)
        bindings[symbol] = actual
    return {"source": str(source), "configured_pristine_c_units": len(units),
            "actual_observer_compile_options_checked": True, "live_symbol_bindings": bindings,
            "binary_sha256": sha(binary), "compile_commands_sha256": sha(binary.parent / "compile_commands.json"),
            "map_sha256": sha(link_map)}


def main():
    p = argparse.ArgumentParser(description=__doc__)
    p.add_argument("--ordinary", required=True, type=Path)
    p.add_argument("--sanitized", required=True, type=Path)
    p.add_argument("--rust", required=True, action="append")
    p.add_argument("--output", required=True, type=Path)
    args = p.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    requests = corpus()
    (args.output / "requests.txt").write_text("\n".join(requests) + "\n")
    binaries = [("ordinary", args.ordinary.resolve()), ("sanitized", args.sanitized.resolve())]
    binaries += [(name, Path(path).resolve()) for name, path in (entry.split("=", 1) for entry in args.rust)]
    assert len(set(name for name, _ in binaries)) == len(binaries)
    report = {"source_pin": PIN, "requests_per_configuration": len(requests), "configurations": {},
              "pointer_normalization": "observation copies only; actual allocation/static/function/opaque identities and interior offsets", "default_payload_initialized_by_observer_only": True}
    for name, binary in binaries:
        path = args.output / f"{name}.txt"
        source = binding(binary, name == "sanitized") if name in ["ordinary", "sanitized"] else None
        capture(binary, requests, path)
        if name != "ordinary":
            exact(args.output / "ordinary.txt", path)
        report["configurations"][name] = {"source_binding": source, "binary_sha256": sha(binary),
                                           "output_sha256": sha(path), "output_bytes": path.stat().st_size}
        if name not in ["ordinary", "sanitized"]:
            private = subprocess.run([binary], input=b"bs-private-layout\n", stdout=subprocess.PIPE, stderr=subprocess.PIPE, check=True)
            assert not private.stderr
            required, payload, header, residue = map(int, private.stdout.split())
            # The actual original max_align_t row is preserved in the530th
            # record; private Rust alignment is observed separately, not
            # relabeled as C ABI alignment or imposed on source metadata.
            with (args.output / "ordinary.txt").open("rb") as original:
                last = None
                for last in original:
                    pass
            assert last is not None and required == int(last.split()[0])
            assert payload >= required and payload % required == 0
            assert header % payload == 0 and residue == 0
            report["configurations"][name]["private_default_alignment"] = {
                "actual_source_required": required, "rust_payload": payload,
                "private_header": header, "observed_payload_residue": residue}
    report["all_exact"] = True
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"requests": len(requests), "configurations": list(report["configurations"]), "all_exact": True}))


if __name__ == "__main__":
    main()
