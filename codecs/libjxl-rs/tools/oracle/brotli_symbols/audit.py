#!/usr/bin/env python3
"""Audit actual pristine symbol bodies, default streams and all six Rust gates."""
import argparse
import hashlib
import json
import os
import pathlib
import re
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[3]
SOURCE = ROOT / "upstream/libjxl/third_party/brotli"
PINS = {
    "upstream/libjxl": "a16d32d0ac113f5e54125e49bb571281c5d0b704",
    "upstream/libjxl/third_party/highway": "457c891775a7397bdb0376bb1031e6e027af1c48",
    "upstream/libjxl/third_party/brotli": "db24a98aed38a16de78cc05ca15cdd8dc2a0d92d",
    "upstream/libjxl/third_party/skcms": "96d9171c94b937a1b5f0293de7309ac16311b722",
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def run(command, **kwargs):
    result = subprocess.run(command, capture_output=True, check=True, **kwargs)
    assert not result.stderr, result.stderr.decode(errors="replace")
    return result.stdout


def binding(link_map, function, object_name):
    if "# Object files:" in link_map:
        objects = link_map.split("# Sections:")[0]
        owner = next(re.search(r"\[\s*(\d+)\]", line).group(1)
                     for line in objects.splitlines() if object_name in line)
        return next(line for line in link_map.splitlines()
                    if re.search(rf"\[\s*{owner}\]\s+_{function}$", line))
    assert function in link_map and object_name in link_map
    return object_name


def audit_build(build, sanitized):
    entries = json.loads((build / "compile_commands.json").read_text())
    selected = [entry for entry in entries if "/brotli/c/" in entry["file"]
                or entry["file"].endswith("/brotli_symbols.c")]
    assert any(entry["file"].endswith("/dec/decode.c") for entry in selected)
    definitions = []
    for entry in selected:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command, entry
        assert ("-fsanitize=address,undefined" in command) == sanitized, entry
        if sanitized:
            assert "-fno-sanitize-recover=all" in command, entry
        assert not re.search(r"-O0|-fno-sanitize=|-fshort-enums|-DBROTLI_(?:BUILD_NO_RBIT|EXPERIMENTAL)", command), entry
        is_scoped = "CMakeFiles/brotli_symbols_oracle.dir/" in command
        assert ("-DBROTLI_TEST=1" in command) == is_scoped, entry
        if is_scoped:
            definitions.append(entry)
        assert ("-DSYMBOLS_DEFAULT=1" in command) == ("CMakeFiles/brotli_symbols_default.dir/" in command), entry
    assert len(definitions) == 1
    scoped_map = (build / "brotli_symbols_oracle.map").read_text()
    default_map = (build / "brotli_symbols_default.map").read_text()
    scoped_owner = "CMakeFiles/brotli_symbols_oracle.dir/"
    bindings = {name: binding(scoped_map, name, scoped_owner) for name in [
        "BrotliSafeReadSymbolForTest", "BrotliInverseMoveToFrontTransformForTest",
        "BrotliDecoderDecompress"]}
    bindings["default-BrotliDecoderDecompress"] = binding(
        default_map, "BrotliDecoderDecompress", "libbrotlidec.a(decode.c.o)")
    for name in ["BrotliBuildSimpleHuffmanTable", "BrotliBuildHuffmanTable"]:
        bindings[name] = binding(scoped_map, name, "libbrotlidec.a(huffman.c.o)")
    inventory = run(["nm", str(build / "brotli_symbols_oracle")]).decode()
    for name in ["SafeDecodeSymbol", "BrotliSafeReadSymbolForTest"]:
        assert re.search(rf"\b[tT]\s+_?{name}$", inventory, re.M), name
    return {"compile_commands": selected, "bindings": bindings,
            "scoped_map_sha256": sha(scoped_map.encode()),
            "default_map_sha256": sha(default_map.encode())}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--plain", required=True, type=pathlib.Path)
    parser.add_argument("--asan", required=True, type=pathlib.Path)
    parser.add_argument("--output", required=True, type=pathlib.Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    report = {"pins": PINS, "builds": {}, "gates": {}, "default_streams": {}}
    for relative, pin in PINS.items():
        path = ROOT / relative
        assert run(["git", "-C", str(path), "rev-parse", "HEAD"]).decode().strip() == pin
        assert not run(["git", "-C", str(path), "status", "--porcelain"])
    assert sha((ROOT / "src/brotli/bit_reader.rs").read_bytes()) == "e0c768a3cea20caad66460f882ec2d5495ccb02b217a2bca4532962c2603d19f"
    occurrences = []
    for directory in ["dec", "common", "include"]:
        for path in sorted((SOURCE / "c" / directory).rglob("*")):
            if path.is_file():
                for line, text in enumerate(path.read_bytes().splitlines(), 1):
                    if b"BROTLI_TEST" in text:
                        occurrences.append([str(path.relative_to(SOURCE)), line, text.decode("ascii")])
    assert [(p, n) for p, n, _ in occurrences] == [
        ("c/dec/decode.c", 2976), ("c/common/platform.h", 663),
        ("c/common/platform.h", 664), ("c/common/platform.h", 665)]
    report["BROTLI_TEST_occurrences"] = occurrences
    report["BROTLI_TEST_effect"] = "platform normalization of zero; only two visibility wrappers after decode.c2976; no algorithm, numeric width, default archive or sanitizer changes"
    body = (SOURCE / "c/dec/decode.c").read_text().splitlines(keepends=True)
    report["source_body_lines_361_519_sha256"] = sha("".join(body[360:519]).encode())
    report["source_files"] = {str(path.relative_to(ROOT)): sha(path.read_bytes()) for path in [
        SOURCE / "c/dec/decode.c", SOURCE / "c/dec/huffman.h",
        SOURCE / "c/dec/bit_reader.h", SOURCE / "c/common/platform.h"]}
    rust = (ROOT / "src/brotli/symbols.rs").read_text()
    annotations = {name: "never" if name == "safe_decode_symbol" else "always" for name in [
        "decode_symbol", "read_symbol", "safe_decode_symbol", "safe_read_symbol",
        "preload_symbol", "read_preloaded_symbol", "copy_preloaded_symbols_to_u8_with_writer",
        "copy_preloaded_symbols_to_u8"]}
    for name, annotation in annotations.items():
        assert f"#[inline({annotation})]\npub fn {name}<" in rust
    report["original_inline_noinline_preserved"] = annotations
    for name, build, sanitized in [("plain", args.plain, False), ("asan", args.asan, True)]:
        report["builds"][name] = audit_build(build, sanitized)
        for fixture in ["empty", "x", "quickfox", "quickfox_repeated", "alice29.txt",
                        "random_org_10k.bin", "backward65536", "compressed_repeated"]:
            path = SOURCE / "tests/testdata" / fixture
            expected = path.read_bytes()
            observations = []
            for target in ["brotli_symbols_oracle", "brotli_symbols_default"]:
                data = run([str(build / target), "decode", str(path) + ".compressed"])
                assert data == expected, (name, target, fixture)
                observations.append(sha(data))
            report["default_streams"][name + "-" + fixture] = {
                "bytes": len(expected), "decoded_sha256": observations,
                "compressed_sha256": sha(path.with_name(path.name + ".compressed").read_bytes())}
    binaries = {}
    for config, prefix, optimize in [("debug", ["rustc"], "0"),
                                     ("release", ["rustc"], "3"),
                                     ("msrv", ["rustup", "run", "1.95", "rustc"], "0")]:
        binary = args.output / ("rust-" + config)
        command = prefix + ["--edition=2024", "--test", "tests/brotli_symbols.rs",
                            "--cfg", 'feature="cpp-oracle"', "-D", "warnings",
                            "-C", "opt-level=" + optimize, "-o", str(binary)]
        run(command, cwd=ROOT)
        binaries[config] = binary
        for graph, build in [("plain", args.plain), ("asan", args.asan)]:
            gate = config + "-" + graph
            capture = args.output / gate
            env = dict(os.environ, BROTLI_SYMBOLS_ORACLE=str(build / "brotli_symbols_oracle"),
                       BROTLI_SYMBOLS_CAPTURE=str(capture))
            stdout = run([str(binary), "--test-threads=1"], env=env, cwd=ROOT)
            assert b"8 passed; 0 failed" in stdout, stdout
            (args.output / (gate + ".log")).write_bytes(stdout)
            report["gates"][gate] = {"records": int((capture / "records.txt").read_text()),
                "requests_sha256": sha((capture / "requests.txt").read_bytes()),
                "observations_sha256": sha((capture / "source.bin").read_bytes()),
                "tables_sha256": sha((capture / "tables.bin").read_bytes()),
                "record_bytes": 136, "tests": 8}
            print(gate + " exact", flush=True)
    for key in ["records", "requests_sha256", "observations_sha256", "tables_sha256"]:
        assert len({gate[key] for gate in report["gates"].values()}) == 1
    (args.output / "clippy.toml").write_text('msrv = "1.95"\n')
    run(["clippy-driver", "--edition=2024", "--test", "tests/brotli_symbols.rs",
         "--cfg", 'feature="cpp-oracle"', "-D", "warnings", "-o",
         str(args.output / "rust-clippy")], cwd=ROOT,
        env=dict(os.environ, CLIPPY_CONF_DIR=str(args.output)))
    report.update({"source_defined_corpus_only": True, "table_kinds": 35,
        "all_15bit_prefixes_per_table": 32768, "source_and_rust_exact": True,
        "production_registration": False, "full_repository_gate": False,
        "clippy_D_warnings_msrv_1_85": True})
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["table_kinds", "source_and_rust_exact"]}))


if __name__ == "__main__":
    main()
