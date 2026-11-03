#!/usr/bin/env python3
"""Build independent actual-archive and pristine-include default CMS observers."""
import argparse
import hashlib
import gzip
import json
import os
from pathlib import Path
import platform
import re
import shlex
import subprocess
import itertools
from skcms_binding import live_bindings

ROOT = Path(__file__).resolve().parent.parent


def sha(path):
    return hashlib.sha256(Path(path).read_bytes()).hexdigest()


def source_command(build):
    source = ROOT / "upstream/libjxl/lib/jxl/cms/jxl_cms.cc"
    entries = json.loads((build / "compile_commands.json").read_text())
    entries = [entry for entry in entries if Path(entry["file"]).resolve() == source]
    assert len(entries) == 1, entries
    entry = entries[0]
    command = entry.get("arguments") or shlex.split(entry["command"])
    assert "-O3" in command and "-DNDEBUG" in command
    assert "-DJPEGXL_ENABLE_SKCMS=1" in command
    assert "-DHWY_COMPILE_ONLY_SCALAR=1" in command
    assert "-fno-slp-vectorize" in command and "-fno-vectorize" in command
    return entry, command


def build_contexts(build):
    build = Path(build).resolve()
    entry, original = source_command(build)
    archive = build / "upstream/lib/libjxl_cms.a"
    original_object = Path(original[original.index("-o") + 1])
    if not original_object.is_absolute():
        original_object = Path(entry["directory"]) / original_object
    before = {str(path): sha(path) for path in [archive, original_object, Path(entry["file"])]}
    receipts = []
    for kind in ["public", "private"]:
        source = ROOT / f"tools/oracle/default_cms_{kind}.cc"
        obj = build / f"default_cms_{kind}.o"
        binary = build / f"default_cms_{kind}"
        command = original.copy()
        command[command.index("-o") + 1] = str(obj)
        command[command.index("-c") + 1] = str(source)
        with (build / f"default_cms_{kind}-compile.log").open("w") as log:
            subprocess.run(command, cwd=entry["directory"], stdout=log, stderr=subprocess.STDOUT, check=True)
        link = [command[0], str(obj), str(archive), "-o", str(binary)]
        for flag in command[1:]:
            if flag.startswith(("-fsanitize=", "-fno-sanitize-recover=", "-stdlib=", "--target=", "-mmacosx-version-min=")):
                link.append(flag)
        for pair in ["-target", "-isysroot"]:
            if pair in command:
                index = command.index(pair)
                link += command[index:index + 2]
        mapping = build / f"default_cms_{kind}.map"
        link += [("-Wl,-map," if platform.system() == "Darwin" else "-Wl,-Map,") + str(mapping)]
        if platform.system() == "Linux":
            link += ["-lm"]
        with (build / f"default_cms_{kind}-link.log").open("w") as log:
            subprocess.run(link, cwd=build, stdout=log, stderr=subprocess.STDOUT, check=True)
        files = [source, ROOT / "tools/oracle/default_cms_observe.h", obj, binary, mapping]
        receipts.append({"kind": kind, "original": entry, "compile": command, "link": link,
                         "sha256": {str(path): sha(path) for path in files}})
    assert before == {path: sha(path) for path in before}, "Actual original source/archive/object changed"
    receipt = {"contexts": receipts, "original_graph": before,
               "compiler": subprocess.check_output([original[0], "--version"], text=True)}
    (build / "default_cms_contexts.json").write_text(json.dumps(receipt, indent=2) + "\n")
    return build / "default_cms_public", build / "default_cms_private"


PRIVATE_NAMES = [
    "CIExyFromXYZ", "DecodeProfile", "ColorSpaceFromProfile", "MatrixProduct",
    "IsAcceptableCHAD", "HasICCVersion4DisplayClass", "HasTag", "HasLUTTags",
    "HasParametricTRCs", "IsApproximatelyD50", "GetNonconformingMatrixProfileWhitePoint",
    "RecoverMatrixProfileWhitePoint", "UnadaptedWhitePoint", "IdentifyPrimaries",
    "IsApproximatelyEqual", "DetectTransferFunction", "GetPrimariesLuminances",
    "ApplyHlgOotf", "IsKnownTransferFunction", "IsKnownColorPrimaries", "ApplyCICP",
    "AllocateBuffer", "BeforeTransform", "AfterTransform",
]
PUBLIC_NAMES = ["JxlGetDefaultCms", "JxlCmsSetFieldsFromICC", "JxlCmsInit",
                "JxlCmsGetSrcBuf", "JxlCmsGetDstBuf", "DoColorSpaceTransform", "JxlCmsDestroy"]


def cms_symbol(name):
    if name == "JxlGetDefaultCms":
        return r"_?JxlGetDefaultCms"
    namespace = "8N_SCALAR" if name in ["BeforeTransform", "AfterTransform"] else "12_GLOBAL__N_1"
    return r"_+ZN3jxl" + namespace + str(len(name)) + name + r"[^.]*"


def bindings(build):
    build = Path(build).resolve()
    entry, original = source_command(build)
    receipt = json.loads((build / "default_cms_contexts.json").read_text())
    for path, expected in receipt["original_graph"].items():
        assert sha(path) == expected, path
    instrumented = "-fsanitize=address,undefined" in original
    wanted = ["-fsanitize=address,undefined", "-fno-sanitize-recover=all"] if instrumented else []
    assert [flag for flag in original if "sanitize" in flag] == wanted
    commands = json.loads((build / "compile_commands.json").read_text())
    dependency_commands = []
    for name in ["skcms.cc", "skcms_TransformBaseline.cc"]:
        selected = [value for value in commands if Path(value["file"]).name == name]
        assert len(selected) == 1, selected
        argv = selected[0].get("arguments") or shlex.split(selected[0]["command"])
        assert [flag for flag in argv if "sanitize" in flag] == wanted
        assert "-DSKCMS_DISABLE_HSW" in argv and "-DSKCMS_DISABLE_SKX" in argv
        assert not any("SKCMS_PORTABLE" in flag for flag in argv)
        dependency_commands.append(selected[0])
    results = {}
    for context in receipt["contexts"]:
        kind = context["kind"]
        expected = original.copy()
        expected[expected.index("-o") + 1] = str(build / f"default_cms_{kind}.o")
        expected[expected.index("-c") + 1] = str(ROOT / f"tools/oracle/default_cms_{kind}.cc")
        assert context["original"] == entry and context["compile"] == expected
        for path, digest in context["sha256"].items():
            assert sha(path) == digest, path
        binary = build / f"default_cms_{kind}"
        mapping = build / f"default_cms_{kind}.map"
        owner = (r"libjxl_cms\.a(?:\[\d+\])?\((?:.*/)?jxl_cms\.cc\.o\)$" if kind == "public"
                 else r"(?:^|/)default_cms_private\.o$")
        specs = {name: (cms_symbol(name), owner) for name in PUBLIC_NAMES}
        if kind == "private":
            specs.update({name: (cms_symbol(name), owner) for name in PRIVATE_NAMES})
            assert not re.search(r"libjxl_cms\.a[^\n]*\bjxl_cms\.cc\.o", mapping.read_text())
        bound = live_bindings(binary, mapping, specs)
        skcms_owner = r"libjxl_cms\.a(?:\[\d+\])?\((?:.*/)?skcms\.cc\.o\)$"
        dependencies = {name: (r"_?" + name, skcms_owner) for name in [
            "skcms_ParseWithA2BPriority", "skcms_GetCHAD", "skcms_GetWTPT", "skcms_Transform",
            "skcms_MakeUsableAsDestination", "skcms_ApproximateCurve", "skcms_ApproximatelyEqualProfiles"]}
        dependencies["baseline::run_program"] = (r"_?_*ZN13skcms_private8baseline11run_programE.*",
            r"libjxl_cms\.a(?:\[\d+\])?\((?:.*/)?skcms_TransformBaseline\.cc\.o\)$")
        dependencies = live_bindings(binary, mapping, dependencies)
        query = "default-cms-bindings\n" + ("default-cms-private-bindings\n" if kind == "private" else "")
        runtime = subprocess.run([str(binary)], input=query, text=True, capture_output=True, check=True)
        assert not runtime.stderr
        rows = runtime.stdout.splitlines()
        public = list(map(int, rows[0].split()))
        assert len(public) == len(PUBLIC_NAMES)
        slide = public[0] - bound["JxlGetDefaultCms"]["address"]
        for name, address in zip(PUBLIC_NAMES, public):
            assert address - slide == bound[name]["address"], (kind, name)
        if kind == "private":
            private = list(map(int, rows[1].split()))
            assert len(private) == len(PRIVATE_NAMES)
            for name, address in zip(PRIVATE_NAMES, private):
                assert address - slide == bound[name]["address"], name
        results[kind] = {"functions": bound, "dependencies": dependencies,
                         "runtime_addresses": runtime.stdout, "runtime_slide": slide}
    return {"instrumented": instrumented, "contexts": results,
            "source_commands": [entry, *dependency_commands], "build_receipt": receipt}


FAMILIES = {
    "guards": (False, 157), "matrix": (True, 4096), "matrix-exceptional": (True, 88064), "detail": (True, 88),
    "rows": (False, 2967), "cicp": (True, 428379), "exceptional": (True, 128),
    "tf-domain": (True, 82126), "prefixes": (True, 52), "init-prefixes": (False, 8), "callbacks": (False, 2084),
    "numeric": (True, 144905), "allocation": (True, 770), "luminances": (True, 205),
    "profile-equality": (True, 2700), "synthetic-fields": (False, 5001),
    "malformed-fields": (False, 184467),
}


def audit(output, record_native=False):
    """Compare both complete original graphs, retaining every generated case."""
    output = Path(output).resolve()
    output.mkdir(parents=True, exist_ok=True)
    manifest = json.loads((ROOT / "tests/data/default-cms-preflight.json").read_text())
    matrix = ROOT / "tests/data/default-cms-matrix-inputs.bin"
    assert sha(matrix) == manifest["sha256"] and matrix.stat().st_size == 4096 * 52
    for key, count, family in [
        ("initial_matrix_failures", 483, "matrix"),
        ("initial_profile_failures", 51, "guards"),
        ("initial_ootf_failures", 284, "tf-domain"),
        ("initial_hlg_panic_indices", 12, "exceptional"),
        ("initial_callback_failures", 8, "callbacks"),
        ("initial_xy_failures", 854, "numeric"),
        ("initial_null_zero_abort_indices", 8, "callbacks"),
    ]:
        indices = manifest[key]
        assert len(indices) == count and len(set(indices)) == count
        assert all(0 <= index < FAMILIES[family][1] for index in indices)
    report = {"platform": platform.platform(), "machine": platform.machine(), "families": {}, "graphs": {}}
    pins = {}
    for path in ["upstream/libjxl", "upstream/libjxl/third_party/highway",
                 "upstream/libjxl/third_party/brotli", "upstream/libjxl/third_party/skcms"]:
        pins[path] = subprocess.check_output(["git", "-C", str(ROOT / path), "rev-parse", "HEAD"], text=True).strip()
    report["pins"] = pins
    probe = ROOT / "target/default_cms_audit_probe"
    subprocess.run(["cargo", "build", "--locked", "--offline", "--all-features"], cwd=ROOT, check=True)
    with (output / "rust-probe-build.log").open("w") as log:
        subprocess.run(["rustc", "--edition=2024", str(ROOT / "tools/default_cms_probe.rs"),
                        "--extern", f"jxl={ROOT / 'target/debug/libjxl.rlib'}", "-L",
                        f"dependency={ROOT / 'target/debug/deps'}", "-o", str(probe)], cwd=ROOT,
                       env=dict(os.environ, CARGO_MANIFEST_DIR=str(ROOT)), stdout=log, stderr=subprocess.STDOUT, check=True)
    report["rust_probe_sha256"] = sha(probe)
    source_platform = (platform.system(), platform.machine()) == (manifest["reference_platform"]["system"], manifest["reference_platform"]["machine"])
    native = {}
    selection = {"guards": [0, 64], "matrix": [2], "rows": [2, 420], "cicp": [0],
                 "exceptional": [24], "tf-domain": [65551], "prefixes": [24],
                 "callbacks": [280], "numeric": [7944], "allocation": [1],
                 "luminances": [0], "profile-equality": [0], "synthetic-fields": [0],
                 "malformed-fields": [0]}
    for graph in ["oracle", "oracle-asan"]:
        build = ROOT / "target" / graph
        directory = output / graph
        directory.mkdir(exist_ok=True)
        binding = bindings(build)
        stable_paths = [probe, build / "upstream/lib/libjxl_cms.a"]
        stable_paths.extend(Path(path) for path in binding["build_receipt"]["original_graph"])
        for entry in binding["source_commands"]:
            argv = entry.get("arguments") or shlex.split(entry["command"])
            obj = Path(argv[argv.index("-o") + 1])
            stable_paths.append(obj if obj.is_absolute() else Path(entry["directory"]) / obj)
        for context in binding["build_receipt"]["contexts"]:
            stable_paths.extend(Path(path) for path in context["sha256"])
        stable = {str(path): sha(path) for path in stable_paths}
        disassembly = {}
        for name, entry in [("cms", binding["source_commands"][0]), ("baseline", binding["source_commands"][2])]:
            argv = entry.get("arguments") or shlex.split(entry["command"])
            obj = Path(argv[argv.index("-o") + 1])
            if not obj.is_absolute(): obj = Path(entry["directory"]) / obj
            destination = directory / f"{name}-object-disassembly.txt"
            command = ["otool", "-tvV", str(obj)] if platform.system() == "Darwin" else ["objdump", "-dr", str(obj)]
            with destination.open("w") as log: subprocess.run(command, stdout=log, stderr=subprocess.STDOUT, check=True)
            disassembly[name] = {"command": command, "object_sha256": sha(obj), "disassembly_sha256": sha(destination)}
        report["graphs"][graph] = {"bindings": binding, "stable_sha256": stable, "disassembly": disassembly}
        for family, (private, expected_count) in FAMILIES.items():
            requests = build / "default-cms-corpus" / f"{family}.requests"
            assert requests.is_file(), ("Run the complete check.py gates first", requests)
            request_hash = sha(requests)
            with requests.open("rb") as stream: count = sum(1 for _ in stream)
            assert count == expected_count, (family, count, expected_count)
            previous = report["families"].setdefault(family, {"count": count, "requests_sha256": request_hash, "graphs": {}})
            assert previous["requests_sha256"] == request_hash, (family, "input corpus changed between graphs")
            if family in manifest["retained_families"]:
                retained = manifest["retained_families"][family]
                assert count == retained["count"] and request_hash == retained["requests_sha256"], family
            binaries = [("private", build / "default_cms_private")]
            if not private: binaries.insert(0, ("public", build / "default_cms_public"))
            binaries.append(("rust", probe))
            observed = []
            family_graph = {}
            for kind, binary in binaries:
                raw = directory / f"{family}-{kind}.stdout"
                stderr = directory / f"{family}-{kind}.stderr"
                with requests.open("rb") as source, raw.open("wb") as dest, stderr.open("wb") as errors:
                    result = subprocess.run([str(binary)], stdin=source, stdout=dest, stderr=errors)
                assert result.returncode == 0 and stderr.stat().st_size == 0, (graph, family, kind, result.returncode, stderr)
                digest = sha(raw)
                selected = {}
                with raw.open("rb") as stream:
                    for index, row in enumerate(stream):
                        if index in selection.get(family, []): selected[index] = row.rstrip().decode()
                    assert index + 1 == count, (family, kind, "observation count")
                if kind != "rust":
                    if source_platform and family in manifest["retained_families"]:
                        assert digest == manifest["retained_families"][family]["initial_source_observations_sha256"], (family, "initial source words changed")
                    with requests.open() as stream:
                        for index, request in enumerate(stream):
                            if index in selected:
                                key = (family, index)
                                value = (request.rstrip(), selected[index])
                                assert key not in native or native[key] == value, (graph, family, index, "independent source capture mismatch")
                                native[key] = value
                compressed = raw.with_suffix(".stdout.gz")
                with raw.open("rb") as source, gzip.open(compressed, "wb", compresslevel=1) as dest:
                    while chunk := source.read(1024 * 1024): dest.write(chunk)
                raw.unlink()
                family_graph[kind] = {"raw_sha256": digest, "gzip_sha256": sha(compressed), "binary_sha256": sha(binary)}
                observed.append(compressed)
            with gzip.open(observed[0], "rb") as source:
                references = [gzip.open(path, "rb") for path in observed[1:]]
                try:
                    for index, rows in enumerate(itertools.zip_longest(source, *references)):
                        assert all(row is not None and row.rstrip() == rows[0].rstrip() for row in rows), (graph, family, index, "exact observation mismatch")
                    assert index + 1 == count
                finally:
                    for reference in references: reference.close()
            if graph == "oracle-asan":
                ordinary = output / "oracle" / observed[0].name
                with gzip.open(ordinary, "rb") as left, gzip.open(observed[0], "rb") as right:
                    for index, (a, b) in enumerate(itertools.zip_longest(left, right)):
                        assert a == b, (family, index, "ordinary/instrumented source discrepancy")
                    assert index + 1 == count
            previous["graphs"][graph] = family_graph
            print(graph, family, count, "exact", flush=True)
        assert stable == {path: sha(path) for path in stable}, "Frozen source/helper graph changed"
    assert len(native) == 16
    captures = output / "native.tsv"
    captures.write_text("".join(request + "\t" + row + "\n" for request, row in native.values()))
    report["native"] = {"count": len(native), "sha256": sha(captures)}
    report["cases"] = sum(count for _, count in FAMILIES.values())
    assert report["cases"] == 946197
    if record_native:
        assert platform.system() == "Darwin" and platform.machine() == "arm64"
        fixture = ROOT / "tests/data/default-cms-native-arm64.tsv"
        fixture.write_bytes(captures.read_bytes())
        (ROOT / "tests/data/default-cms-native-arm64.json").write_text(json.dumps({"pins": pins, "platform": report["platform"], "compiler": report["graphs"]["oracle"]["bindings"]["build_receipt"]["compiler"], "cases": len(native), "tsv_sha256": sha(fixture)}, indent=2) + "\n")
    elif platform.system() == "Darwin" and platform.machine() == "arm64":
        fixture = ROOT / "tests/data/default-cms-native-arm64.tsv"
        assert fixture.read_bytes() == captures.read_bytes(), "Independent native fixture changed"
    (output / "report.json").write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"cases": report["cases"], "report_sha256": sha(output / "report.json")}), flush=True)


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build", type=Path)
    parser.add_argument("--verify-bindings", action="store_true")
    parser.add_argument("--output", type=Path)
    parser.add_argument("--record-native", action="store_true")
    args = parser.parse_args()
    if args.output:
        audit(args.output, args.record_native)
    elif args.build:
        for binary in build_contexts(args.build): print(binary)
    else:
        parser.error("Choose --build or --output")
    if args.build and args.verify_bindings:
        result = bindings(args.build)
        path = args.build / "default_cms_bindings.json"
        path.write_text(json.dumps(result, indent=2) + "\n")
        print(path)
