#!/usr/bin/env python3
"""Audit actual pristine block-length/MTF bodies, default streams and all six Rust gates."""
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
                or entry["file"].endswith("/brotli_block_mtf.c")]
    assert any(entry["file"].endswith("/dec/decode.c") for entry in selected)
    definitions = []
    for entry in selected:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command, entry
        assert ("-fsanitize=address,undefined" in command) == sanitized, entry
        if sanitized:
            assert "-fno-sanitize-recover=all" in command, entry
        assert not re.search(r"-O0|-fno-sanitize=|-fshort-enums|-DBROTLI_(?:BUILD_NO_RBIT|EXPERIMENTAL)", command), entry
        is_scoped = "CMakeFiles/brotli_block_mtf_oracle.dir/" in command
        assert ("-DBROTLI_TEST=1" in command) == is_scoped, entry
        if is_scoped:
            definitions.append(entry)
        assert ("-DBLOCK_MTF_DEFAULT=1" in command) == ("CMakeFiles/brotli_block_mtf_default.dir/" in command), entry
    assert len(definitions) == 1
    scoped_map = (build / "brotli_block_mtf_oracle.map").read_text()
    default_map = (build / "brotli_block_mtf_default.map").read_text()
    scoped_owner = "CMakeFiles/brotli_block_mtf_oracle.dir/"
    bindings = {name: binding(scoped_map, name, scoped_owner) for name in [
        "BrotliSafeReadSymbolForTest", "BrotliInverseMoveToFrontTransformForTest",
        "BrotliDecoderDecompress"]}
    bindings["default-BrotliDecoderDecompress"] = binding(
        default_map, "BrotliDecoderDecompress", "libbrotlidec.a(decode.c.o)")
    for name in ["BrotliBuildSimpleHuffmanTable", "BrotliBuildHuffmanTable"]:
        bindings[name] = binding(scoped_map, name, "libbrotlidec.a(huffman.c.o)")
    inventory = run(["nm", str(build / "brotli_block_mtf_oracle")]).decode()
    for name in ["InverseMoveToFrontTransform", "BrotliSafeReadSymbolForTest"]:
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
        "running.rs": "440fdfe54e5d410c81be2cea5d0a56162c9f608070eec36868d90f86134e05ad"}
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
    report["BROTLI_TEST_effect"] = "platform normalization of zero; only two visibility wrappers after decode.c2976; no algorithm, numeric width, default archive or sanitizer changes"
    body = (SOURCE / "c/dec/decode.c").read_text().splitlines(keepends=True)
    report["source_body_lines_930_1023_sha256"] = sha("".join(body[929:1023]).encode())
    report["source_files"] = {str(path.relative_to(ROOT)): sha(path.read_bytes()) for path in [
        SOURCE / "c/dec/decode.c", SOURCE / "c/dec/huffman.h",
        SOURCE / "c/dec/bit_reader.h", SOURCE / "c/common/platform.h",
        SOURCE / "c/dec/state.h", SOURCE / "c/dec/state.c", SOURCE / "c/dec/prefix.h"]}
    for path, names, annotation in [
        ("block_length.rs", ["read_block_length", "safe_read_block_length"], "always"),
        ("mtf.rs", ["inverse_move_to_front_transform"], "never")]:
        rust = (ROOT / "src/brotli" / path).read_text()
        for name in names:
            assert re.search(rf"#\[inline\({annotation}\)\]\npub (?:unsafe )?fn {name}", rust)
    report["original_inline_noinline_preserved"] = True
    for name, build, sanitized in [("plain", args.plain, False), ("asan", args.asan, True)]:
        report["builds"][name] = audit_build(build, sanitized)
        for fixture in ["empty", "x", "quickfox", "quickfox_repeated", "alice29.txt",
                        "random_org_10k.bin", "backward65536", "compressed_repeated"]:
            path = SOURCE / "tests/testdata" / fixture
            expected = path.read_bytes()
            observations = []
            for target in ["brotli_block_mtf_oracle", "brotli_block_mtf_default"]:
                data = run([str(build / target), "decode", str(path) + ".compressed"])
                assert data == expected, (name, target, fixture)
                observations.append(sha(data))
            report["default_streams"][name + "-" + fixture] = {
                "bytes": len(expected), "decoded_sha256": observations,
                "compressed_sha256": sha(path.with_name(path.name + ".compressed").read_bytes())}
    for kind, tests in [("block_length", 5), ("mtf", 5)]:
        for config, prefix, optimize in [("debug", ["rustc"], "0"),
                                         ("release", ["rustc"], "3"),
                                         ("msrv", ["rustup", "run", "1.85", "rustc"], "0")]:
            binary = args.output / ("rust-" + kind + "-" + config)
            run(prefix + ["--edition=2024", "--test", "tests/brotli_" + kind + ".rs",
                          "--cfg", 'feature="cpp-oracle"', "-D", "warnings",
                          "-C", "opt-level=" + optimize, "-o", str(binary)], cwd=ROOT)
            for graph, build in [("plain", args.plain), ("asan", args.asan)]:
                gate = config + "-" + graph
                capture = args.output / gate
                env = dict(os.environ, BROTLI_BLOCK_MTF_ORACLE=str(build / "brotli_block_mtf_oracle"),
                           BROTLI_BLOCK_MTF_CAPTURE=str(capture))
                stdout = run([str(binary), "--test-threads=1"], env=env, cwd=ROOT)
                assert f"{tests} passed; 0 failed".encode() in stdout, stdout
                (args.output / (kind + "-" + gate + ".log")).write_bytes(stdout)
                part = capture / ("block" if kind == "block_length" else "mtf")
                cases, records = map(int, (part / "counts.txt").read_text().split())
                assert (part / "rust.bin").read_bytes() == (part / "source.bin").read_bytes()
                report["gates"][kind + "-" + gate] = {
                    "cases": cases, "records": records, "tests": tests,
                    "requests_sha256": sha((part / "requests.txt").read_bytes()),
                    "observations_sha256": sha((part / "source.bin").read_bytes()),
                    "observation_bytes": (part / "source.bin").stat().st_size}
                if kind == "block_length":
                    report["gates"][kind + "-" + gate]["tables_sha256"] = sha((part / "tables.bin").read_bytes())
                print(kind + "-" + gate + " exact", flush=True)
        gates = [gate for key, gate in report["gates"].items() if key.startswith(kind + "-")]
        for key in gates[0]:
            assert len({gate[key] for gate in gates}) == 1
    (args.output / "clippy.toml").write_text('msrv = "1.85"\n')
    for kind in ["block_length", "mtf"]:
        run(["clippy-driver", "--edition=2024", "--test", "tests/brotli_" + kind + ".rs",
             "--cfg", 'feature="cpp-oracle"', "-D", "warnings", "-o",
             str(args.output / ("rust-clippy-" + kind))], cwd=ROOT,
            env=dict(os.environ, CLIPPY_CONF_DIR=str(args.output)))
    report.update({"source_defined_corpus_only": True, "table_families": 4,
        "all_15bit_prefixes_per_general_table": 32768, "source_and_rust_exact": True,
        "production_registration": False, "full_repository_gate": False,
        "mtf_seeded_full_owner_bytes": 260, "mtf_word0_bytes0_2_never_read_in_unknown_proof": True,
        "seeded_tail_fixtures_are_not_universal_initialization_proof": True,
        "clippy_D_warnings_msrv_1_85": True})
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["table_families", "source_and_rust_exact"]}))


if __name__ == "__main__":
    main()
