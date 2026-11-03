#!/usr/bin/env python3
"""Prepare the actual pinned-original observers required by all-feature tests.

One dedicated graph shares pristine archives while preserving each existing
adapter translation unit and its scoped visibility-only BROTLI_TEST setting.
It never changes the libjxl oracle or a frozen standalone reference graph.
"""
import argparse
import importlib.util
import json
import os
from pathlib import Path
import re
import shlex
import shutil
import subprocess

from brotli_decoder_binding import requested_bindings
from brotli_state_reference import PIN, ROOT, compile_options, sha

# Test environment name, unchanged observer TUs, and live source bindings.
ARCHIVE = "libbrotlidec.a(decode.c.o)"
STATE = "libbrotlidec.a(state.c.o)"
SHARED = "libbrotlicommon.a(shared_dictionary.c.o)"
STATE_BINDINGS = {
    **{f"BrotliDecoderState{name}": STATE for name in
       ["Init", "MetablockBegin", "CleanupAfterMetablock", "Cleanup"]},
    "BrotliDecoderHuffmanTreeGroupInit": STATE,
    "BrotliDecoderCreateInstance": ARCHIVE,
    "BrotliDecoderDestroyInstance": ARCHIVE,
    "BrotliDefaultAllocFunc": "libbrotlicommon.a(platform.c.o)",
    "BrotliDefaultFreeFunc": "libbrotlicommon.a(platform.c.o)",
    "BrotliSharedDictionaryCreateInstance": SHARED,
    "BrotliSharedDictionaryDestroyInstance": SHARED,
    "BrotliGetDictionary": "libbrotlicommon.a(dictionary.c.o)",
    "BrotliGetTransforms": "libbrotlicommon.a(transform.c.o)",
}
OBSERVERS = {
    "bit_reader_storage": (["brotli_bit_reader_storage.c"], {
        "BrotliSafeReadBits32Slow": "libbrotlidec.a(bit_reader.c.o)",
        "BrotliWarmupBitReader": "libbrotlidec.a(bit_reader.c.o)"}),
    "state": (["brotli_state.cc"], STATE_BINDINGS),
    "instance": (["brotli_instance.cc", "brotli_instance_settings.c"], {
        "BrotliDecoderCreateInstance": ARCHIVE, "BrotliDecoderDestroyInstance": ARCHIVE,
        "BrotliDecoderSetParameter": ARCHIVE, "BrotliDecoderSetMetadataCallbacks": ARCHIVE}),
    "status": (["brotli_status.c"], {
        "BrotliDecoderVersion": ARCHIVE, "BrotliDecoderErrorString": ARCHIVE}),
    "transforms": (["brotli_transforms.cc"], {
        "BrotliGetDictionary": "libbrotlicommon.a(dictionary.c.o)",
        "BrotliGetTransforms": "libbrotlicommon.a(transform.c.o)",
        "BrotliTransformDictionaryWord": "libbrotlicommon.a(transform.c.o)"}),
    "huffman": (["brotli_huffman.cc"], {
        name: "libbrotlidec.a(huffman.c.o)" for name in
        ["BrotliBuildSimpleHuffmanTable", "BrotliBuildHuffmanTable", "BrotliBuildCodeLengthsHuffmanTable"]}),
}
PRIVATE = ["metablock", "compound_dictionary", "ringbuffer", "stream_blocks",
           "symbols", "huffman_reader", "block_mtf", "distance_command", "tree_group", "context_map"]
TEST_BRIDGES = {"symbols", "huffman_reader", "block_mtf"}
for name in PRIVATE:
    filename = f"brotli_{name}.c"
    expected = {function: filename + ".o" for function in
                ["BrotliDecoderCreateInstance", "BrotliDecoderDestroyInstance", "BrotliDecoderDecompressStream"]}
    expected.update({"BrotliDecoderStateInit": STATE,
                     "BrotliSharedDictionaryCreateInstance": SHARED})
    if name in TEST_BRIDGES:
        expected.update({function: filename + ".o" for function in
                         ["BrotliSafeReadSymbolForTest", "BrotliInverseMoveToFrontTransformForTest"]})
    OBSERVERS[name] = ([filename] + (["brotli_context_map_fixtures.c"] if name == "context_map" else []), expected)


def verify(build, name, sanitized):
    """Require exact flags and actual nm-address/live-owner agreement per API."""
    binary = build / f"brotli_{name}_oracle"
    commands = json.loads((build / "compile_commands.json").read_text())
    source = (ROOT / "upstream/libjxl/third_party/brotli").resolve()
    units = [c for c in commands if Path(c["file"]).resolve().is_relative_to(source)]
    assert len(units) == 35
    for unit in units:
        compile_options(unit, sanitized)
    adapters = []
    for filename in OBSERVERS[name][0]:
        matches = [c for c in commands if Path(c["file"]).resolve() == ROOT / "tools/oracle" / filename
                   and any(f"CMakeFiles/{binary.name}.dir/" in arg
                           for arg in c.get("arguments", shlex.split(c.get("command", ""))))]
        assert len(matches) == 1, (filename, matches)
        entry = matches[0]
        flags = entry.get("arguments") or shlex.split(entry["command"])
        bridges = [flag for flag in flags if "BROTLI_TEST" in flag]
        assert bridges == (["-DBROTLI_TEST=1"] if name in TEST_BRIDGES else []), (filename, bridges)
        # Normalize exactly the pre-existing visibility-only adapter definition;
        # every archive unit above is checked untouched, with no test setting.
        normalized = dict(entry, arguments=[f for f in flags if f != "-DBROTLI_TEST=1"])
        compile_options(normalized, sanitized)
        adapters.append(entry)
    mapped = build / f"{binary.name}.map"
    expected = OBSERVERS[name][1]
    live = requested_bindings(mapped.read_text(), expected)
    symbols = subprocess.check_output(["nm", binary], text=True)
    bindings = {}
    for symbol, owner in expected.items():
        addresses = re.findall(rf"^([0-9a-fA-F]+) [Tt] _?{symbol}$", symbols, re.M)
        assert len(addresses) == 1, symbol
        actual = live[symbol]
        assert Path(actual["object"]).name == owner, (symbol, actual, owner)
        assert actual["address"] == int(addresses[0], 16), (symbol, actual)
        bindings[symbol] = actual
    return {"binary_sha256": sha(binary), "map_sha256": sha(mapped),
            "compile_commands_sha256": sha(build / "compile_commands.json"),
            "configured_pristine_c_units": len(units), "checked_adapter_units": len(adapters),
            "live_symbol_bindings": bindings}


def prepare(sanitized):
    graph = "sanitized" if sanitized else "ordinary"
    build = ROOT / "target/brotli-support" / graph
    build.mkdir(parents=True, exist_ok=True)
    source = ROOT / "upstream/libjxl/third_party/brotli"
    assert subprocess.check_output(["git", "-C", source, "rev-parse", "HEAD"], text=True).strip() == PIN
    assert not subprocess.check_output(["git", "-C", source, "status", "--porcelain"])
    configure = ["cmake", "-S", ROOT / "tools/oracle/brotli_support", "-B", build,
                 "-DCMAKE_BUILD_TYPE=Release", f"-DCMAKE_C_COMPILER={shutil.which('clang')}",
                 f"-DCMAKE_CXX_COMPILER={shutil.which('clang++')}",
                 f"-DORACLE_SANITIZERS={'ON' if sanitized else 'OFF'}"]
    def run(command):
        print("+ " + " ".join(map(str, command)), flush=True)
        subprocess.run(list(map(str, command)), cwd=ROOT, check=True)
    # A compiler-cache change can reset options; the identical confirmation
    # retains required settings before the strict actual-command verification.
    run(configure)
    run(configure)
    run(["cmake", "--build", build, "--target", *[f"brotli_{name}_oracle" for name in OBSERVERS], "--parallel", "2"])
    report = {"source_pin": PIN, "graph": graph,
              "observers": {name: verify(build, name, sanitized) for name in OBSERVERS}}
    env = {f"BROTLI_{name.upper()}_ORACLE": str(build / f"brotli_{name}_oracle") for name in OBSERVERS}
    captures = build / "captures"
    captures.mkdir(exist_ok=True)
    for name in OBSERVERS:
        env[f"BROTLI_{name.upper()}_CAPTURE"] = str(captures / name)
    fixtures = subprocess.run([env["BROTLI_CONTEXT_MAP_ORACLE"], "fixtures"], capture_output=True, check=True)
    assert not fixtures.stderr
    (build / "context-map-fixtures.bin").write_bytes(fixtures.stdout)
    spec = importlib.util.spec_from_file_location("brotli_context_corpus", ROOT / "tools/oracle/brotli_context_map/corpus.py")
    corpus = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(corpus)
    requests = corpus.generate(fixtures.stdout)
    assert len(requests.splitlines()) == 38187
    request_path = build / "context-map-requests.txt"
    request_path.write_bytes(requests)
    env["BROTLI_CONTEXT_MAP_REQUESTS"] = str(request_path)
    report.update({"context_map_requests": 38187, "context_map_requests_sha256": sha(request_path),
                   "context_map_fixtures_sha256": sha(build / "context-map-fixtures.bin"), "environment": env})
    (build / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    return env


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--sanitizers", action="store_true")
    args = parser.parse_args()
    print(json.dumps(prepare(args.sanitizers), indent=2))


if __name__ == "__main__":
    main()
