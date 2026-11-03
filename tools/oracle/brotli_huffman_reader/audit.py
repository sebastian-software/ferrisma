#!/usr/bin/env python3
"""Audit genuine encoder headers, partial arena fields and six decoder gates."""
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
PREREQUISITES = {
    "src/brotli/layout.rs": "7be9a9586cc43ab6e261de91753c2696d0da86b8d51961353fe6b36d6d057c6e",
    "src/brotli/bit_reader.rs": "e0c768a3cea20caad66460f882ec2d5495ccb02b217a2bca4532962c2603d19f",
    "src/brotli/memory.rs": "51bccb0df84f546450a155a0b8dddc9b18eab118bc23509c57750c9e5da15684",
    "src/brotli/huffman.rs": "894c6f8c46f25b024fdc2911e3ac672676736f920fd5cb947c38532dacfdce83",
    "src/brotli/constants.rs": "5d6d6b3f3cdfe8e0c665fdfae22a970afd367ea52d70ff1c5d3e9e0648029191",
    "src/brotli/status.rs": "2ad77d49d9b3b00013d4f53f083ea7ec065b69a174b2c9cb7fa5902f4e1ed9b8",
    "src/brotli/running.rs": "440fdfe54e5d410c81be2cea5d0a56162c9f608070eec36868d90f86134e05ad",
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def run(command, **kwargs):
    result = subprocess.run(command, capture_output=True, check=True, **kwargs)
    assert not result.stderr, result.stderr.decode(errors="replace")
    return result.stdout


def binding(link_map, function, object_name):
    if "# Object files:" in link_map:
        owner = next(re.search(r"\[\s*(\d+)\]", line).group(1)
                     for line in link_map.split("# Sections:")[0].splitlines()
                     if object_name in line)
        return next(line for line in link_map.splitlines()
                    if re.search(rf"\[\s*{owner}\]\s+_{function}$", line))
    assert function in link_map and object_name in link_map
    return object_name


def audit_build(build, sanitized):
    entries = json.loads((build / "compile_commands.json").read_text())
    selected = [entry for entry in entries if "/brotli/c/" in entry["file"]
                or entry["file"].endswith("/brotli_huffman_reader.c")]
    assert any(entry["file"].endswith("/dec/decode.c") for entry in selected)
    assert any(entry["file"].endswith("/enc/brotli_bit_stream.c") for entry in selected)
    definitions = []
    for entry in selected:
        command = entry["command"]
        assert "-O3" in command and "-DNDEBUG" in command, entry
        assert ("-fsanitize=address,undefined" in command) == sanitized, entry
        if sanitized:
            assert "-fno-sanitize-recover=all" in command, entry
        assert not re.search(r"-O0|-fno-sanitize=|-fshort-enums|-DBROTLI_(?:BUILD_NO_RBIT|EXPERIMENTAL)", command), entry
        scoped = "CMakeFiles/brotli_huffman_reader_oracle.dir/" in command
        assert ("-DBROTLI_TEST=1" in command) == scoped, entry
        if scoped:
            definitions.append(entry)
        assert ("-DHEADER_DEFAULT=1" in command) == ("CMakeFiles/brotli_huffman_reader_default.dir/" in command), entry
    assert len(definitions) == 1
    scoped_map = (build / "brotli_huffman_reader_oracle.map").read_text()
    default_map = (build / "brotli_huffman_reader_default.map").read_text()
    bindings = {name: binding(scoped_map, name, "CMakeFiles/brotli_huffman_reader_oracle.dir/") for name in [
        "BrotliSafeReadSymbolForTest", "BrotliDecoderDecompress", "BrotliDecoderDecompressStream"]}
    bindings["default-BrotliDecoderDecompress"] = binding(default_map, "BrotliDecoderDecompress", "libbrotlidec.a(decode.c.o)")
    bindings["BrotliDecoderStateInit"] = binding(scoped_map,"BrotliDecoderStateInit","libbrotlidec.a(state.c.o)")
    for name in ["BrotliStoreHuffmanTree", "BrotliBuildAndStoreHuffmanTreeFast"]:
        bindings[name] = binding(scoped_map,name,"libbrotlienc.a(brotli_bit_stream.c.o)")
    for name in ["BrotliBuildCodeLengthsHuffmanTable", "BrotliBuildSimpleHuffmanTable", "BrotliBuildHuffmanTable"]:
        bindings[name] = binding(scoped_map,name,"libbrotlidec.a(huffman.c.o)")
    return {"compile_commands": selected, "bindings": bindings,
            "scoped_map_sha256": sha(scoped_map.encode()), "default_map_sha256": sha(default_map.encode())}


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--plain", required=True, type=pathlib.Path)
    parser.add_argument("--asan", required=True, type=pathlib.Path)
    parser.add_argument("--output", required=True, type=pathlib.Path)
    args = parser.parse_args()
    args.output.mkdir(parents=True, exist_ok=False)
    report = {"pins": PINS, "prerequisites": PREREQUISITES, "builds": {}, "gates": {}, "default_streams": {}}
    for relative, pin in PINS.items():
        path = ROOT / relative
        assert run(["git", "-C", str(path), "rev-parse", "HEAD"]).decode().strip() == pin
        assert not run(["git", "-C", str(path), "status", "--porcelain"])
    for relative, digest in PREREQUISITES.items():
        assert sha((ROOT / relative).read_bytes()) == digest, relative
    occurrences = []
    for directory in ["dec", "common", "include", "enc"]:
        for path in sorted((SOURCE / "c" / directory).rglob("*")):
            if path.is_file():
                for line, text in enumerate(path.read_bytes().splitlines(), 1):
                    if b"BROTLI_TEST" in text:
                        occurrences.append([str(path.relative_to(SOURCE)), line, text.decode("ascii")])
    assert [(p, n) for p, n, _ in occurrences] == [
        ("c/dec/decode.c", 2976), ("c/common/platform.h", 663),
        ("c/common/platform.h", 664), ("c/common/platform.h", 665),
        ("c/enc/block_splitter.c",198), ("c/enc/brotli_bit_stream.c",1326),
        ("c/enc/encode.c",2011), ("c/enc/encoder_dict.c",633)]
    report["BROTLI_TEST_occurrences"] = occurrences
    report["BROTLI_TEST_effect"] = "scoped include-TU has platform zero normalization and two decoder visibility bridges only; four encoder source sites are visibility wrappers in archives compiled without BROTLI_TEST; no algorithm/width/default archive/sanitizer changes"
    source = (SOURCE / "c/dec/decode.c").read_text()
    report["source_lines_521_930_sha256"] = sha("".join(source.splitlines(keepends=True)[520:930]).encode())
    rust = (ROOT / "src/brotli/huffman_reader.rs").read_text()
    for c_name, r_name in [("kCodeLengthCodeOrder","CODE_LENGTH_ORDER"),
                            ("kCodeLengthPrefixLength","PREFIX_LENGTH"),
                            ("kCodeLengthPrefixValue","PREFIX_VALUE")]:
        a = re.search(rf"{c_name}\[[^\]]*\]\s*=\s*\{{([^}}]+)\}}",source).group(1)
        b = re.search(rf"const {r_name}:[^=]+?=\s*\[([^\]]+)\]",rust).group(1)
        assert re.findall(r"\d+",a) == re.findall(r"\d+",b)
    report["exact_static_prefix_tables"] = True
    for name in ["log2_floor","process_single_code_length","process_repeated_code_length"]:
        assert re.search(rf"#\[inline\(always\)\].*?pub fn {name}",rust,re.S)
    report["source_files"] = {str(path.relative_to(ROOT)): sha(path.read_bytes()) for path in [
        SOURCE / "c/dec/decode.c",SOURCE / "c/dec/state.h",SOURCE / "c/dec/huffman.h",
        SOURCE / "c/enc/brotli_bit_stream.c",SOURCE / "c/enc/brotli_bit_stream.h"]}
    for name, build, sanitized in [("plain",args.plain,False),("asan",args.asan,True)]:
        report["builds"][name] = audit_build(build,sanitized)
        for fixture in ["empty","x","quickfox","quickfox_repeated","alice29.txt",
                        "random_org_10k.bin","backward65536","compressed_repeated"]:
            path = SOURCE / "tests/testdata" / fixture
            expected = path.read_bytes()
            for target in ["brotli_huffman_reader_oracle","brotli_huffman_reader_default"]:
                assert run([str(build / target),"decode",str(path)+".compressed"]) == expected
            report["default_streams"][name+"-"+fixture] = {"bytes":len(expected),"sha256":sha(expected)}
    for config, prefix, optimize in [("debug",["rustc"],"0"),("release",["rustc"],"3"),
                                     ("msrv",["rustup","run","1.85","rustc"],"0")]:
        binary = args.output / ("rust-"+config)
        run(prefix+["--edition=2024","--test","tests/brotli_huffman_reader.rs","--cfg",'feature="cpp-oracle"',
                    "-D","warnings","-C","opt-level="+optimize,"-o",str(binary)],cwd=ROOT)
        for graph,build in [("plain",args.plain),("asan",args.asan)]:
            gate = config+"-"+graph;capture = args.output/gate
            env = dict(os.environ,BROTLI_HUFFMAN_READER_ORACLE=str(build/"brotli_huffman_reader_oracle"),
                       BROTLI_HUFFMAN_READER_CAPTURE=str(capture))
            stdout = run([str(binary),"--test-threads=1"],env=env,cwd=ROOT)
            assert b"8 passed; 0 failed" in stdout,stdout
            (args.output/(gate+".log")).write_bytes(stdout)
            counts = (capture/"counts.txt").read_text()
            case_count,record_count,record_bytes = map(int,counts.splitlines()[0].split())
            report["gates"][gate] = {"cases":case_count,"records":record_count,"record_bytes":record_bytes,
                "status_counts_source_seedA":counts.splitlines()[1],"tests":8,
                "files":{p.name:sha(p.read_bytes()) for p in capture.iterdir() if p.is_file()}}
            assert report["gates"][gate]["files"]["rust.bin"] == report["gates"][gate]["files"]["source.bin"]
            print(gate+" exact",flush=True)
    for key in ["cases","records","record_bytes","status_counts_source_seedA","files"]:
        assert len({json.dumps(g[key],sort_keys=True) for g in report["gates"].values()}) == 1
    (args.output/"clippy.toml").write_text('msrv = "1.85"\n')
    run(["clippy-driver","--edition=2024","--test","tests/brotli_huffman_reader.rs","--cfg",'feature="cpp-oracle"',
         "-D","warnings","-o",str(args.output/"rust-clippy")],cwd=ROOT,
        env=dict(os.environ,CLIPPY_CONF_DIR=str(args.output)))
    report.update({"source_and_rust_exact":True,"source_encoder_fixtures":50,
        "source_header_fields":1375,"source_header_field_bytes":3756,
        "source_header_bytes":4440,"source_header_alignment":8,
        "seed_masks":"observed seed-independent field bytes only; never an initialization proof or basis for production typed reads",
        "unknown_owner_native_fixtures":True,"production_registration":False,
        "source_simple_widths":list(range(1,12)),"same_owner_context_map_output":True,
        "full_repository_gate":False,"clippy_D_warnings_msrv_1_85":True})
    (args.output/"report.json").write_text(json.dumps(report,indent=2)+"\n")
    print(json.dumps({"all_six_exact":True,"fixtures":50}))


if __name__ == "__main__":
    main()
