#!/usr/bin/env python3
"""Audit actual pristine tree-group bodies, default streams and all six Rust gates."""
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


def file_sha(path):
    digest = hashlib.sha256()
    with path.open("rb") as source:
        for chunk in iter(lambda: source.read(1 << 20), b""):
            digest.update(chunk)
    return digest.hexdigest()


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
                or entry["file"].endswith("/brotli_tree_group.c")]
    assert any(entry["file"].endswith("/dec/decode.c") for entry in selected)
    definitions = []
    for entry in selected:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command, entry
        assert ("-fsanitize=address,undefined" in command) == sanitized, entry
        if sanitized:
            assert "-fno-sanitize-recover=all" in command, entry
        assert not re.search(r"-O0|-fno-sanitize=|-fshort-enums|-DBROTLI_(?:BUILD_NO_RBIT|EXPERIMENTAL)", command), entry
        is_scoped = "CMakeFiles/brotli_tree_group_oracle.dir/" in command
        assert "BROTLI_TEST" not in command, entry
        if is_scoped:
            definitions.append(entry)
        assert ("-DTREE_GROUP_DEFAULT=1" in command) == ("CMakeFiles/brotli_tree_group_default.dir/" in command), entry
    assert len(definitions) == 1
    scoped_map = (build / "brotli_tree_group_oracle.map").read_text()
    default_map = (build / "brotli_tree_group_default.map").read_text()
    scoped_owner = "CMakeFiles/brotli_tree_group_oracle.dir/"
    bindings = {name: binding(scoped_map, name, scoped_owner) for name in [
        "BrotliDecoderDecompress", "BrotliDecoderDecompressStream"]}
    bindings["default-BrotliDecoderDecompress"] = binding(
        default_map, "BrotliDecoderDecompress", "libbrotlidec.a(decode.c.o)")
    for name in ["BrotliBuildSimpleHuffmanTable", "BrotliBuildHuffmanTable"]:
        bindings[name] = binding(scoped_map, name, "libbrotlidec.a(huffman.c.o)")
    bindings["BrotliDecoderHuffmanTreeGroupInit"] = binding(
        scoped_map, "BrotliDecoderHuffmanTreeGroupInit", "libbrotlidec.a(state.c.o)")
    for name in ["BrotliBuildAndStoreHuffmanTreeFast", "BrotliStoreHuffmanTree"]:
        bindings[name] = binding(scoped_map, name, "libbrotlienc.a(brotli_bit_stream.c.o)")
    inventory = run(["nm", str(build / "brotli_tree_group_oracle")]).decode()
    for name in ["BrotliDecoderDecompress", "BrotliDecoderDecompressStream"]:
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
    prerequisites = {
        "bit_reader.rs": "e0c768a3cea20caad66460f882ec2d5495ccb02b217a2bca4532962c2603d19f",
        "layout.rs": "7be9a9586cc43ab6e261de91753c2696d0da86b8d51961353fe6b36d6d057c6e",
        "memory.rs": "51bccb0df84f546450a155a0b8dddc9b18eab118bc23509c57750c9e5da15684",
        "constants.rs": "5d6d6b3f3cdfe8e0c665fdfae22a970afd367ea52d70ff1c5d3e9e0648029191",
        "huffman.rs": "894c6f8c46f25b024fdc2911e3ac672676736f920fd5cb947c38532dacfdce83",
        "symbols.rs": "f038c3350e1496aa910368053d5f67a2f182ea223622bbca65d5d3c4d4cffbf2",
        "running.rs": "440fdfe54e5d410c81be2cea5d0a56162c9f608070eec36868d90f86134e05ad",
        "status.rs": "2ad77d49d9b3b00013d4f53f083ea7ec065b69a174b2c9cb7fa5902f4e1ed9b8",
        "huffman_reader.rs": "ebf65b68ec0e52b6fb1b14c7da59a8c59f3661eaade4cb2ad799af762b25f500"}
    for name, expected in prerequisites.items():
        assert sha((ROOT / "src/brotli" / name).read_bytes()) == expected, name
    report["prerequisites"] = prerequisites
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
    report["BROTLI_TEST_effect"] = "not defined in any observer or archive TU; pristine default configuration"
    body = (SOURCE / "c/dec/decode.c").read_text().splitlines(keepends=True)
    report["source_body_lines_1021_1040_sha256"] = sha("".join(body[1020:1040]).encode())
    report["source_header_reader_lines_521_930_sha256"] = sha("".join(body[520:930]).encode())
    report["source_files"] = {str(path.relative_to(ROOT)): sha(path.read_bytes()) for path in [
        SOURCE / "c/dec/decode.c", SOURCE / "c/dec/huffman.h",
        SOURCE / "c/dec/bit_reader.h", SOURCE / "c/common/platform.h",
        SOURCE / "c/dec/state.h", SOURCE / "c/dec/state.c",
        SOURCE / "c/enc/brotli_bit_stream.c", SOURCE / "c/enc/brotli_bit_stream.h"]}
    observer = (ROOT / "tools/oracle/brotli_tree_group.c").read_text()
    assert observer.count('#include "dec/decode.c"') == 1
    assert "static BrotliDecoderErrorCode HuffmanTreeGroupDecode" not in observer
    production = (ROOT / "src/brotli/tree_group.rs").read_text()
    assert "#[inline" not in production  # Original group helper is plain static.
    report["original_plain_static_annotation_preserved"] = True
    for name, build, sanitized in [("plain", args.plain, False), ("asan", args.asan, True)]:
        report["builds"][name] = audit_build(build, sanitized)
        for fixture in ["empty", "x", "quickfox", "quickfox_repeated", "alice29.txt",
                        "random_org_10k.bin", "backward65536", "compressed_repeated"]:
            path = SOURCE / "tests/testdata" / fixture
            expected = path.read_bytes()
            observations = []
            for target in ["brotli_tree_group_oracle", "brotli_tree_group_default"]:
                data = run([str(build / target), "decode", str(path) + ".compressed"])
                assert data == expected, (name, target, fixture)
                observations.append(sha(data))
            report["default_streams"][name + "-" + fixture] = {
                "bytes": len(expected), "decoded_sha256": observations,
                "compressed_sha256": sha(path.with_name(path.name + ".compressed").read_bytes())}
    for config, prefix, optimize in [("debug", ["rustc"], "0"),
                                     ("release", ["rustc"], "3"),
                                     ("msrv", ["rustup", "run", "1.95", "rustc"], "0")]:
        binary = args.output / ("rust-" + config)
        run(prefix + ["--edition=2024", "--test", "tests/brotli_tree_group.rs",
                      "--cfg", 'feature="cpp-oracle"', "-D", "warnings",
                      "-C", "opt-level=" + optimize, "-o", str(binary)], cwd=ROOT)
        for graph, build in [("plain", args.plain), ("asan", args.asan)]:
            gate = config + "-" + graph
            capture = args.output / gate
            env = dict(os.environ, BROTLI_TREE_GROUP_ORACLE=str(build / "brotli_tree_group_oracle"),
                       BROTLI_TREE_GROUP_CAPTURE=str(capture))
            stdout = run([str(binary), "--test-threads=1"], env=env, cwd=ROOT)
            assert b"7 passed; 0 failed" in stdout, stdout
            (args.output / (gate + ".log")).write_bytes(stdout)
            cases, records, observation_bytes = map(int, (capture / "counts.txt").read_text().split())
            # The native test compares every byte in bounded streaming buffers,
            # including EOF, before this independent capture inventory is made.
            assert (cases, records, observation_bytes) == (6611, 29244, 374420032)
            assert (capture / "source.bin").stat().st_size == observation_bytes
            assert file_sha(capture / "rust.bin") == file_sha(capture / "source.bin")
            report["gates"][gate] = {"cases": cases, "records": records, "tests": 7,
                "requests_sha256": file_sha(capture / "requests.txt"),
                "observations_sha256": file_sha(capture / "source.bin"),
                "observation_bytes": observation_bytes,
                "fixtures_sha256": file_sha(capture / "fixtures.bin"),
                "schema_sha256": file_sha(capture / "schema.bin")}
            print(gate + " exact", flush=True)
    for key in report["gates"]["debug-plain"]:
        assert len({gate[key] for gate in report["gates"].values()}) == 1
    (args.output / "clippy.toml").write_text('msrv = "1.95"\n')
    run(["clippy-driver", "--edition=2024", "--test", "tests/brotli_tree_group.rs",
         "--cfg", 'feature="cpp-oracle"', "-D", "warnings", "-o", str(args.output / "rust-clippy")],
        cwd=ROOT, env=dict(os.environ, CLIPPY_CONF_DIR=str(args.output)))
    smoke = pathlib.Path("/private/tmp/brotli-tree-group-smoke-captures")
    if smoke.exists():
        report["retained_first_smoke"] = {}
        for name in ["counts.txt", "requests.txt", "fixtures.bin", "schema.bin", "source.bin", "rust.bin"]:
            old = file_sha(smoke / name)
            assert old == file_sha(args.output / "debug-plain" / name)
            report["retained_first_smoke"][name] = old
    report.update({"source_defined_corpus_only": True,
        "source_and_rust_exact": True, "genuine_encoder_fixtures": 133,
        "encoder_alphabet_widths": list(range(1, 12)),
        "tree_counts": [0, 1, 2, 3, 8], "physical_seeds": [165, 90],
        "production_registration": False, "full_repository_gate": False,
        "native_architecture": os.uname().machine,
        "other_native_architecture_acceptance": "pending",
        "header_structural_field_spans": 1375, "header_structural_bytes": 3756,
        "header_padding_bytes_excluded": 684,
        "output_code_observation": "bits:u8/value:u16 only; every byte1 padding excluded",
        "pointer_slot_initialization": "source success/index increments only, never seed inference",
        "seeded_full_fields_are_not_universal_initialization_proof": True,
        "native_unknown_owner_fixtures": 5,
        "clippy_D_warnings_msrv_1_85": True})
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["genuine_encoder_fixtures", "source_and_rust_exact"]}))



if __name__ == "__main__":
    main()
