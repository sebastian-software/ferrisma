#!/usr/bin/env python3
"""Build/audit the separate pristine skcms private-helper observation graph."""
import argparse
from collections import Counter
import gzip
import hashlib
import os
import platform
import re
import tempfile
import json
from pathlib import Path
import shlex
import subprocess
from skcms_binding import live_bindings

ROOT = Path(__file__).resolve().parent.parent


def build_private(build):
    commands = json.loads((build / "compile_commands.json").read_text())
    original = ROOT / "upstream/libjxl/third_party/skcms/skcms.cc"
    matches = [entry for entry in commands if Path(entry["file"]).resolve() == original]
    if len(matches) != 1:
        raise SystemExit("Expected exactly one original skcms.cc compile command")
    entry = matches[0]
    command = shlex.split(entry["command"])
    output = build / "skcms_parser_private.o"
    command[command.index("-o") + 1] = str(output)
    command[command.index(str(original))] = str(ROOT / "tools/oracle/skcms_parser_private.cc")
    subprocess.run(command, cwd=entry["directory"], check=True)
    archive = build / "upstream/lib/libjxl_cms.a"
    compiler = command[0]
    link = [compiler, str(output), str(archive), "-o", str(build / "skcms_parser_private")]
    # Compiler, standard library, target and instrumentation are inherited from
    # the exact actual pristine source command, not from a guessed flag set.
    for flag in command[1:]:
        if flag.startswith(("-fsanitize=", "-stdlib=", "--target=", "-mmacosx-version-min=")):
            link.append(flag)
    if "-target" in command:
        i = command.index("-target")
        link.extend(command[i:i + 2])
    import sys
    if sys.platform == "darwin":
        link.append(f"-Wl,-map,{build / 'skcms_parser_private.map'}")
    else:
        link.extend(["-lm", f"-Wl,-Map={build / 'skcms_parser_private.map'}"])
    subprocess.run(link, cwd=entry["directory"], check=True)
    (build / "skcms_parser_private_commands.json").write_text(json.dumps({
        "original": entry, "private_compile": command, "private_link": link,
        "source_sha256": sha(original),
        "observer_sha256": sha(ROOT / "tools/oracle/skcms_parser.cc"),
        "private_observer_sha256": sha(ROOT / "tools/oracle/skcms_parser_private.cc"),
        "object_sha256": sha(output),
        "compiler": subprocess.check_output([compiler,"--version"],text=True),
    }, indent=2) + "\n")


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def observe(binary, requests):
    output = subprocess.run([str(binary)], input="\n".join(requests)+"\n",
                            text=True, capture_output=True, check=False)
    if output.returncode:
        raise RuntimeError(f"Original observer failed ({output.returncode}): {output.stderr}")
    result = output.stdout.splitlines()
    if len(result) != len(requests):
        raise RuntimeError("Original response count changed")
    return result


def bindings(build, directory):
    commands = json.loads((build / "compile_commands.json").read_text())
    source = ROOT / "upstream/libjxl/third_party/skcms/skcms.cc"
    entry, = [row for row in commands if Path(row["file"]).resolve() == source]
    original = shlex.split(entry["command"])
    receipt = json.loads((build / "skcms_parser_private_commands.json").read_text())
    expected = original.copy()
    expected[expected.index("-o")+1] = str(build / "skcms_parser_private.o")
    expected[expected.index(str(source))] = str(ROOT / "tools/oracle/skcms_parser_private.cc")
    if expected != receipt["private_compile"] or receipt["original"] != entry:
        raise RuntimeError("Private helper compilation does not match actual original command")
    for name, path in {"source_sha256":source,
                       "observer_sha256":ROOT/"tools/oracle/skcms_parser.cc",
                       "private_observer_sha256":ROOT/"tools/oracle/skcms_parser_private.cc",
                       "object_sha256":build/"skcms_parser_private.o"}.items():
        if receipt[name] != sha(path):
            raise RuntimeError(f"Stale private source binding: {name}")
    if [flag for flag in original if flag.startswith("-O")] != ["-O3"] or "-DNDEBUG" not in original:
        raise RuntimeError("Original skcms numeric/optimization configuration changed")
    if any(flag.startswith(("-ffast-math", "-ffp-contract", "-ffp-model", "-funsafe-math", "-fno-signed-zeros")) for flag in original):
        raise RuntimeError("Original skcms floating-point configuration changed")
    sanitized = build.name == "oracle-asan"
    wanted = ["-fsanitize=address,undefined", "-fno-sanitize-recover=all"] if sanitized else []
    if [flag for flag in original if "sanitize" in flag] != wanted:
        raise RuntimeError("Original nonrecovering sanitizer coverage changed")
    link = shlex.split((build / "CMakeFiles/jxl_oracle.dir/link.txt").read_text())
    link[link.index("-o")+1] = str(directory / "public-bound")
    mapping = directory / "public.map"
    link.append(("-Wl,-map," if platform.system()=="Darwin" else "-Wl,-Map,")+str(mapping))
    subprocess.run(link,cwd=build,check=True)
    text = mapping.read_text()
    names=["skcms_ParseWithA2BPriority","skcms_GetTagByIndex","skcms_GetTagBySignature",
           "skcms_GetCHAD","skcms_GetWTPT","skcms_GetInputChannelCount",
           "skcms_sRGB_profile","skcms_XYZD50_profile","skcms_sRGB_TransferFunction",
           "skcms_sRGB_Inverse_TransferFunction","skcms_Identity_TransferFunction"]
    public = live_bindings(directory / "public-bound", mapping, {
        name: (r"_?" + name, r"libjxl_cms\.a(?:\[\d+\])?\((?:.*/)?skcms\.cc\.o\)$") for name in names})
    private = (build / "skcms_parser_private.map").read_text()
    if re.search(r"libjxl_cms\.a[^\n]*skcms\.cc\.o", private):
        raise RuntimeError("Private helper pulled duplicate public skcms definitions")
    private_symbols = live_bindings(build / "skcms_parser_private", build / "skcms_parser_private.map", {
        name: (r"_+ZL" + str(len(name)) + name + r".*", r"(?:^|/)skcms_parser_private\.o$")
        for name in ["eval_curve", "fit_linear", "canonicalize_identity", "read_curve"]})
    return {"original_compile":original,"private_receipt":receipt,"public_link":link,
            "public_bindings":public,"private_symbols":private_symbols,
            "private_map_sha256":sha(build/"skcms_parser_private.map")}


def source_probes(output):
    probes=[]
    for count in [1073741824,1073741825,1610612736,2147483647]:
        probes.append((f"skcms-curve-source-large 0 {count} 0",1245,"signed integer overflow"))
    for count in [2147483647,2147483648,4294967295]:
        probes.append((f"skcms-curve-source-large 1 {count} 1065353216",308,"outside the range of representable values"))
    for count in [1073741824,1073741825]:
        probes.append((f"skcms-curve-source-large 2 {count} 1065353216",318,"signed integer overflow"))
    rows=[]
    for index,(request,line,diagnostic) in enumerate(probes):
        result=subprocess.run([str(ROOT/"target/oracle-asan/skcms_parser_private")],input=request+"\n",text=True,capture_output=True)
        (output/f"source-only-{index}.stderr").write_text(result.stderr)
        (output/f"source-only-{index}.stdout").write_text(result.stdout)
        if not result.returncode or diagnostic not in result.stderr or f"skcms.cc:{line}:" not in result.stderr or "SUMMARY: UndefinedBehaviorSanitizer" not in result.stderr:
            raise RuntimeError("Missing exact positive source UB diagnosis: "+request+"\n"+result.stderr)
        if "AddressSanitizer: heap-buffer-overflow" in result.stderr or "AddressSanitizer: stack-buffer-overflow" in result.stderr:
            raise RuntimeError("Probe reached a payload bounds error before its arithmetic diagnosis")
        rows.append({"request":request,"classification":"source-undefined","line":line,
                     "diagnostic":result.stderr,"exit":result.returncode})
    return rows


def audit(output, record_native):
    output.mkdir(parents=True,exist_ok=False)
    pins={}
    source=ROOT/"upstream/libjxl"
    for name in ["","third_party/highway","third_party/brotli","third_party/skcms"]:
        path=source/name
        revision=subprocess.check_output(["git","-C",str(path),"rev-parse","HEAD"],text=True).strip()
        expected=json.loads((ROOT/"upstream.json").read_text())["revision"] if not name else subprocess.check_output(["git","-C",str(source),"ls-tree","HEAD",name],text=True).split()[2]
        dirty=subprocess.check_output(["git","-C",str(path),"status","--porcelain","--ignore-submodules=none"],text=True)
        if revision!=expected or dirty:raise RuntimeError("Pristine source pin changed: "+name)
        pins[name or "libjxl"]=revision
    if (ROOT/"licenses/SKCMS-BSD-3-Clause.txt").read_bytes() != (source/"third_party/skcms/LICENSE").read_bytes():
        raise RuntimeError("skcms source notice changed")
    capture=output/"requests.txt"
    env=dict(os.environ,JXL_DECODER_TARGET="scalar",JXL_SKCMS_PARSER_CAPTURE=str(capture),JXL_CPP_ORACLE=str(ROOT/"target/oracle/jxl_oracle"),CARGO_BUILD_JOBS="2",RUST_TEST_THREADS="2")
    with (output/"capture.log").open("w") as log:
        subprocess.run(["cargo","test","--locked","--offline","--all-features","--test","parity","skcms_parser::","--","--nocapture"],cwd=ROOT,env=env,stdout=log,stderr=subprocess.STDOUT,check=True)
    requests=capture.read_text().splitlines()
    counts=Counter(request.split()[0] for request in requests)
    report={"pins":pins,"requests":len(requests),"counts":dict(counts),"request_sha256":sha(capture),"platform":platform.platform(),"graphs":{}}
    previous={}
    native=[]
    for graph in ["oracle","oracle-asan"]:
        build=ROOT/"target"/graph;directory=output/graph;directory.mkdir()
        stable=[build/"jxl_oracle",build/"skcms_parser_private",build/"skcms_parser_private.o",build/"upstream/lib/libjxl_cms.a"]
        before={str(path):sha(path) for path in stable}
        binding=bindings(build,directory)
        for kind in ["public","private"]:
            selected=[request for request in requests if request.startswith("skcms-profile-")== (kind=="public")]
            binary=build/("jxl_oracle" if kind=="public" else "skcms_parser_private")
            observations=directory/(kind+".txt.gz")
            with gzip.open(observations,"wt",compresslevel=1) as stream:
                for at in range(0,len(selected),128):
                    batch=selected[at:at+128];rows=observe(binary,batch)
                    if kind=="public":
                        independently=observe(build/"skcms_parser_private",batch)
                        if rows!=independently:raise RuntimeError("Whole public Parse differs in pristine private-helper compilation context")
                    stream.write("\n".join(rows)+"\n")
            if kind in previous:
                with gzip.open(previous[kind],"rt") as left,gzip.open(observations,"rt") as right:
                    for index,request in enumerate(selected):
                        if next(left)!=next(right):raise RuntimeError(f"Original ordinary/ASAN drift at {index}: {request}")
                    if next(left,None) is not None or next(right,None) is not None:raise RuntimeError("Trailing source observations")
            previous[kind]=observations
        after={str(path):sha(path) for path in stable}
        if before!=after:raise RuntimeError("Active source graph changed during independent audit")
        report["graphs"][graph]={"bindings":binding,"stable_sha256":after}
        candidates=[q for q in requests if q.startswith("skcms-profile-static") or q=="skcms-profile-layout"]
        candidates.extend(q for q in requests if q.startswith("skcms-profile-parse 0 0 ") and 300<len(q)<1200)
        chosen=list(dict.fromkeys(candidates))[:12]
        captures=list(zip(chosen,observe(build/"jxl_oracle",chosen)))
        if native and native!=captures:raise RuntimeError("Independent native source captures drifted")
        native=captures
    report["source_only"]=source_probes(output)
    if record_native:
        (ROOT/"tests/data/skcms-parser-native.tsv").write_text("\n".join(request+"\t"+row for request,row in native)+"\n")
        (ROOT/"tests/data/skcms-parser-native.json").write_text(json.dumps({"pins":pins,"platform":platform.platform(),"compiler":report["graphs"]["oracle"]["bindings"]["private_receipt"]["compiler"],"captures":len(native),"tsv_sha256":sha(ROOT/"tests/data/skcms-parser-native.tsv")},indent=2)+"\n")
    report["native_captures"]=len(native)
    (output/"report.json").write_text(json.dumps(report,indent=2)+"\n")
    print(json.dumps({"requests":len(requests),"source_only":len(report["source_only"]),"native_captures":len(native),"report_sha256":sha(output/"report.json")}),flush=True)


def main():
    parser=argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--build-private",type=Path)
    parser.add_argument("--output",type=Path)
    parser.add_argument("--record-native",action="store_true")
    args=parser.parse_args()
    if args.build_private:
        if args.output:parser.error("Choose build or audit")
        build_private(args.build_private.resolve())
    elif args.output:audit(args.output.resolve(),args.record_native)
    else:parser.error("Choose --build-private or --output")


if __name__=="__main__":main()
