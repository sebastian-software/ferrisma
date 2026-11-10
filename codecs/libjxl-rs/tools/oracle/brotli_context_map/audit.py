#!/usr/bin/env python3
"""Audit actual pristine context-map bodies, default streams and all six Rust gates."""
import argparse
import hashlib
import json
import os
import pathlib
import re
import subprocess
import sys
import struct
sys.dont_write_bytecode = True
from corpus import generate, fixtures
from history import capture_inventory

ROOT = pathlib.Path(__file__).resolve().parents[3]
SOURCE = ROOT / "upstream/libjxl/third_party/brotli"
sys.path.insert(0, str(ROOT / "tools"))
from brotli_decoder_binding import verify
from brotli_state_reference import compile_options
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


def audit_build(build, sanitized):
    entries = json.loads((build / "compile_commands.json").read_text())
    selected = [entry for entry in entries if "/brotli/c/" in entry["file"]
                or entry["file"].endswith("/brotli_context_map.c")
                or entry["file"].endswith("/brotli_context_map_fixtures.c")]
    assert any(entry["file"].endswith("/dec/decode.c") for entry in selected)
    definitions = []
    for entry in selected:
        flags = compile_options(entry, sanitized)
        is_scoped = (entry["file"].endswith("/brotli_context_map.c") and
                     any("CMakeFiles/brotli_context_map_oracle.dir/" in f for f in flags))
        if is_scoped:
            definitions.append(entry)
        assert "BROTLI_TEST" not in " ".join(flags), entry
        assert ("-DCONTEXT_MAP_DEFAULT=1" in flags) == any(
            "CMakeFiles/brotli_context_map_default.dir/" in f for f in flags), entry
    assert len(definitions) == 1
    scoped = {
        **{name: "brotli_context_map.c.o" for name in [
            "BrotliDecoderDecompress", "BrotliDecoderDecompressStream"]},
        **{name: "libbrotlidec.a(huffman.c.o)" for name in [
            "BrotliBuildSimpleHuffmanTable", "BrotliBuildHuffmanTable"]},
        **{name: "brotli_context_map_fixtures.c.o" for name in [
            "BrotliBuildAndStoreHuffmanTreeFast", "BrotliStoreHuffmanTree"]}}
    default = {
        "BrotliDecoderDecompress": "libbrotlidec.a(decode.c.o)",
        "BrotliEncoderCompress": "libbrotlienc.a(encode.c.o)",
        "BrotliStoreMetaBlock": "libbrotlienc.a(brotli_bit_stream.c.o)"}
    proofs = {}
    for target, observers, expected in [
            ("brotli_context_map_oracle", ["brotli_context_map.c", "brotli_context_map_fixtures.c"], scoped),
            ("brotli_context_map_default", ["brotli_context_map.c"], default)]:
        proofs[target] = verify(build / target, sanitized, target + ".map", observers, expected)
    return {"compile_commands": selected, "live_bindings": proofs,
            "strict_actual_compile_tokens_checked": True,
            "scoped_map_sha256": file_sha(build / "brotli_context_map_oracle.map"),
            "default_map_sha256": file_sha(build / "brotli_context_map_default.map")}


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
        "bit_reader.rs": "b7add31eb19d3fb231687dd3277def05ec9597c1dda1b3b68ceac7625d6cd8db",
        "layout.rs": "7be9a9586cc43ab6e261de91753c2696d0da86b8d51961353fe6b36d6d057c6e",
        "memory.rs": "51bccb0df84f546450a155a0b8dddc9b18eab118bc23509c57750c9e5da15684",
        "constants.rs": "5d6d6b3f3cdfe8e0c665fdfae22a970afd367ea52d70ff1c5d3e9e0648029191",
        "huffman.rs": "894c6f8c46f25b024fdc2911e3ac672676736f920fd5cb947c38532dacfdce83",
        "symbols.rs": "f038c3350e1496aa910368053d5f67a2f182ea223622bbca65d5d3c4d4cffbf2",
        "running.rs": "440fdfe54e5d410c81be2cea5d0a56162c9f608070eec36868d90f86134e05ad",
        "status.rs": "2ad77d49d9b3b00013d4f53f083ea7ec065b69a174b2c9cb7fa5902f4e1ed9b8",
        "huffman_reader.rs": "ebf65b68ec0e52b6fb1b14c7da59a8c59f3661eaade4cb2ad799af762b25f500",
        "mtf.rs": "16801c28f80c5d542a690ce3c412f588947a8ddced2a3a07378675f028caf3e8",
        "metablock.rs": "74ff8ccd24c0f0d7d95b550b737fcbc939323bdae447234ffd9930f50897a570",
        "bitfield.rs": "ec888eca107c2ceb5f5c02f728145439e3a43d3fff4abb43f69c1afc507203fb"}
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
    report["source_body_lines_1050_1175_sha256"] = sha("".join(body[1049:1175]).encode())
    report["source_header_reader_lines_521_930_sha256"] = sha("".join(body[520:930]).encode())
    report["source_files"] = {str(path.relative_to(ROOT)): sha(path.read_bytes()) for path in [
        SOURCE / "c/dec/decode.c", SOURCE / "c/dec/huffman.h",
        SOURCE / "c/dec/bit_reader.h", SOURCE / "c/common/platform.h",
        SOURCE / "c/dec/state.h", SOURCE / "c/dec/state.c",
        SOURCE / "c/enc/brotli_bit_stream.c", SOURCE / "c/enc/brotli_bit_stream.h"]}
    observer = (ROOT / "tools/oracle/brotli_context_map.c").read_text()
    fixture_tu = (ROOT / "tools/oracle/brotli_context_map_fixtures.c").read_text()
    assert fixture_tu.count('#include "enc/brotli_bit_stream.c"') == 1
    assert "static void EncodeContextMap(" not in fixture_tu
    assert "static void StoreTrivialContextMap(" not in fixture_tu
    assert observer.count("Bytes((const uint8_t*)s+offsetof(BrotliDecoderState,mtf),sizeof(s->mtf))") == 2

    assert observer.count('#include "dec/decode.c"') == 1
    assert "static BrotliDecoderErrorCode DecodeContextMap" not in observer
    production = (ROOT / "src/brotli/context_map.rs").read_text()
    assert "#[inline" not in production  # Original group helper is plain static.
    report["original_plain_static_annotation_preserved"] = True
    for name, build, sanitized in [("plain", args.plain, False), ("asan", args.asan, True)]:
        report["builds"][name] = audit_build(build, sanitized)
        for fixture in ["empty", "x", "quickfox", "quickfox_repeated", "alice29.txt",
                        "random_org_10k.bin", "backward65536", "compressed_repeated"]:
            path = SOURCE / "tests/testdata" / fixture
            expected = path.read_bytes()
            observations = []
            for target in ["brotli_context_map_oracle", "brotli_context_map_default"]:
                data = run([str(build / target), "decode", str(path) + ".compressed"])
                assert data == expected, (name, target, fixture)
                observations.append(sha(data))
            compressed = run([str(build / "brotli_context_map_default"), "encode", str(path)])
            roundtrip = args.output / (name + "-" + fixture + ".br")
            roundtrip.write_bytes(compressed)
            for target in ["brotli_context_map_oracle", "brotli_context_map_default"]:
                assert run([str(build / target), "decode", str(roundtrip)]) == expected
            report["default_streams"][name + "-" + fixture] = {
                "bytes": len(expected), "decoded_sha256": observations,
                "compressed_sha256": sha(path.with_name(path.name + ".compressed").read_bytes())}
    fixture_bytes = run([str(args.plain / "brotli_context_map_oracle"), "fixtures"])
    assert fixture_bytes == run([str(args.asan / "brotli_context_map_oracle"), "fixtures"])
    assert len(fixtures(fixture_bytes)) == 265
    (args.output / "fixtures.bin").write_bytes(fixture_bytes)
    requests = args.output / "requests.txt"
    requests.write_bytes(generate(fixture_bytes))
    original_inventory = generate(fixture_bytes)
    assert requests.read_bytes() == original_inventory
    assert len(original_inventory.splitlines()) == 38187
    valid, excluded = [], []
    for row in original_inventory.splitlines(keepends=True):
        words = row.split()
        mode = int(words[2])
        width = int(words[5]) if mode == 4 else int(words[3]) if mode == 8 else None
        (excluded if width is not None and not 1 <= width <= 24 else valid).append(row)
    assert b"".join(valid) == generate(fixture_bytes, include_synthetic_width_boundaries=False)
    (args.output / "initial-request-inventory.txt").write_bytes(original_inventory)
    (args.output / "historical-width-boundary-requests.txt").write_bytes(b"".join(excluded))
    report["historical_width_boundary_inventory"] = {
        "initial_requests": len(valid) + len(excluded), "previously_compared_requests": len(valid),
        "previously_source_only_requests": len(excluded),
        "reason": "Historical split retained as evidence only. Comments or compiled-out DCHECK do not classify arithmetic UB; all rows, including synthetic zero/wider states, now receive full Rust parity. Per-row source-stage/caller invariants are a separate census.",
        "initial_inventory_sha256": sha(original_inventory),
        "excluded_inventory_sha256": sha(b"".join(excluded)),
        "classification": "no filtering; all38187 exact cases retained; native64 guard extension is an approved prerequisite boundary amendment",
        "expected_source_words_removed": False}

    report["source_only_diagnostics"] = {}
    diagnostic_hashes = []
    for graph, build in [("plain", args.plain), ("asan", args.asan)]:
        destination = args.output / (graph + "-historical-width-boundaries.bin")
        with (args.output / "historical-width-boundary-requests.txt").open("rb") as source, destination.open("wb") as sink:
            result = subprocess.run([str(build / "brotli_context_map_oracle"), "run-diagnostics"], stdin=source, stdout=sink, stderr=subprocess.PIPE)
        (args.output / (graph + "-historical-width-boundaries.stderr")).write_bytes(result.stderr)
        assert result.returncode == 0 and not result.stderr, result.stderr.decode(errors="replace")
        statuses = {}
        records = 0
        with destination.open("rb") as stream:
            while True:
                head = stream.read(96)
                if not head:
                    break
                assert len(head) == 96
                values = struct.unpack("<12Q", head)
                status = values[0] if values[0] < 1 << 63 else values[0] - (1 << 64)
                statuses[str(status)] = statuses.get(str(status), 0) + 1
                remainder = values[11] + 3756 + 4 + 8 + 260 + values[8]
                assert len(stream.read(remainder)) == remainder
                records += 1
        assert records == 2 * len(excluded)
        diagnostic_hashes.append(file_sha(destination))
        report["source_only_diagnostics"][graph] = {"records": records, "requests": len(excluded),
            "statuses": statuses, "bytes": destination.stat().st_size,
            "sha256": diagnostic_hashes[-1], "exit_code": result.returncode, "stderr": ""}
    with (args.output / "plain-historical-width-boundaries.bin").open("rb") as a, (args.output / "asan-historical-width-boundaries.bin").open("rb") as b:
        while True:
            chunk = a.read(65536)
            assert b.read(len(chunk)) == chunk
            if not chunk:
                assert not b.read(1)
                break
    assert len(set(diagnostic_hashes)) == 1
    report["source_only_diagnostic_full_bytes_equal"] = True
    report["requests"] = {"bytes": requests.stat().st_size, "cases": len(requests.read_bytes().splitlines()), "sha256": file_sha(requests)}
    # Execute the complete original sanitized corpus before Rust comparison;
    # any undefined original request fails here with full UBSan/ASan diagnostics.
    with requests.open("rb") as source, (args.output / "original-domain-preflight.bin").open("wb") as sink:
        completed = subprocess.run([str(args.asan / "brotli_context_map_oracle"), "run"], stdin=source, stdout=sink, stderr=subprocess.PIPE)
    assert completed.returncode == 0 and not completed.stderr, completed.stderr.decode(errors="replace")
    report["original_domain_preflight"] = {"bytes": (args.output / "original-domain-preflight.bin").stat().st_size, "sha256": file_sha(args.output / "original-domain-preflight.bin")}
    for config, prefix, optimize in [("debug", ["rustc"], "0"),
                                     ("release", ["rustc"], "3"),
                                     ("msrv", ["rustup", "run", "1.95", "rustc"], "0")]:
        binary = args.output / ("rust-" + config)
        run(prefix + ["--edition=2024", "--test", "tests/brotli_context_map.rs",
                      "--cfg", 'feature="cpp-oracle"', "-D", "warnings",
                      "-C", "opt-level=" + optimize, "-o", str(binary)], cwd=ROOT)
        for graph, build in [("plain", args.plain), ("asan", args.asan)]:
            gate = config + "-" + graph
            capture = args.output / gate
            env = dict(os.environ, BROTLI_CONTEXT_MAP_ORACLE=str(build / "brotli_context_map_oracle"),
                       BROTLI_CONTEXT_MAP_CAPTURE=str(capture), BROTLI_CONTEXT_MAP_REQUESTS=str(requests))
            stdout = run([str(binary), "--test-threads=1"], env=env, cwd=ROOT)
            assert b"12 passed; 0 failed" in stdout, stdout
            (args.output / (gate + ".log")).write_bytes(stdout)
            cases, records, observation_bytes = map(int, (capture / "counts.txt").read_text().split())
            # The native test compares every byte in bounded streaming buffers,
            # including EOF, before this independent capture inventory is made.
            assert cases == report["requests"]["cases"]
            assert file_sha(capture / "source.bin") == report["original_domain_preflight"]["sha256"]
            assert (capture / "source.bin").stat().st_size == observation_bytes
            assert file_sha(capture / "rust.bin") == file_sha(capture / "source.bin")
            report["gates"][gate] = {"cases": cases, "records": records, "tests": 12,
                "requests_sha256": file_sha(capture / "requests.txt"),
                "observations_sha256": file_sha(capture / "source.bin"),
                "observation_bytes": observation_bytes,
                "fixtures_sha256": file_sha(args.output / "fixtures.bin"),
                "schema_sha256": file_sha(capture / "schema.bin")}
            print(gate + " exact", flush=True)
    for key in report["gates"]["debug-plain"]:
        assert len({gate[key] for gate in report["gates"].values()}) == 1
    (args.output / "clippy.toml").write_text('msrv = "1.95"\n')
    run(["clippy-driver", "--edition=2024", "--test", "tests/brotli_context_map.rs",
         "--cfg", 'feature="cpp-oracle"', "-D", "warnings", "-o", str(args.output / "rust-clippy")],
        cwd=ROOT, env=dict(os.environ, CLIPPY_CONF_DIR=str(args.output)))
    smoke = pathlib.Path("/private/tmp/brotli-context-map-smoke-captures")
    report["per_row_source_history"] = capture_inventory(
        requests, args.output / "debug-plain/source.bin", args.output / "entry-history.jsonl",
        smoke / "source.bin" if smoke.exists() else None)
    assert report["per_row_source_history"]["requests"] == 38187
    assert report["per_row_source_history"]["records"] == 89364
    if smoke.exists():
        report["retained_previous_rust_bytes"] = capture_inventory(
            requests, args.output / "debug-plain/rust.bin", args.output / "rust-entry-history.jsonl", smoke / "rust.bin")
        assert report["per_row_source_history"]["retained_previous_records"] == 75108
        assert report["per_row_source_history"]["retained_previous_bytes"] == 453521836
        assert report["retained_previous_rust_bytes"]["retained_previous_bytes"] == 453521836
        assert file_sha(smoke / "schema.bin") == file_sha(args.output / "debug-plain/schema.bin")
    report["per_row_history_sha256"] = file_sha(args.output / "entry-history.jsonl")
    report.update({"source_defined_corpus_only": True,
        "source_and_rust_exact": True, "genuine_encoder_fixtures": 265,
        "both_outer_map_slots": True, "physical_seeds": [165, 90],
        "production_registration": False, "full_repository_gate": False,
        "native_architecture": os.uname().machine,
        "other_native_architecture_acceptance": "pending",
        "header_structural_field_spans": 1375, "header_structural_bytes": 3756,
        "header_padding_bytes_excluded": 684,
        "output_code_observation": "bits:u8/value:u16 only; every byte1 padding excluded",
        "map_prefix_initialization": "success/TRANSFORM complete independently of stale index; shortage DECODE saved index; error retains conservative proved prefix",
        "seeded_full_fields_are_not_universal_initialization_proof": True,
        "native_unknown_owner_fixtures": 10,
        "clippy_D_warnings_msrv_1_85": True})
    (args.output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({key: report[key] for key in ["genuine_encoder_fixtures", "source_and_rust_exact"]}))



if __name__ == "__main__":
    main()
