#!/usr/bin/env python3
"""Run upstream's conformance scripts against an oracle build.

Runs test_dwebp.sh and test_lossless.sh from the pinned libwebp-test-data,
unmodified, with the oracle's own dwebp, and test_cwebp.sh with its own cwebp,
and checks the exact number of passing comparisons. The scripts write decoded files next to the vectors, so every run
uses a fresh copy under build/oracle/<variant>/baseline/ and upstream/ stays
untouched. The record is written to build/oracle/<variant>/baseline/baseline.json.

test_dwebp.sh decodes every vector named in libwebp_tests.md5 to bmp, pam, pgm,
ppm and tiff and verifies each output's MD5 twice, by default and with -noasm.
test_lossless.sh decodes the 32 lossless_vec_* vectors and
lossless_color_transform.webp to the same five formats, with and without
-noasm, and compares each output byte for byte with a reference image.

test_cwebp.sh encodes each input file with cwebp and with cwebp -noasm and
compares the two outputs; it ships no reference checksums. The inputs are the
test-data files that the oracle's cwebp reads (the WebP vectors, PAM, PGM and
PPM, and PNG or TIFF when the build found libpng or libtiff), in three option
sets (CWEBP_OPTION_SETS), the same that tests/wp13_tools.rs runs with the
Rust-linked cwebp.

--diagnose-post-v1.6.0 also compares the oracle with test-data 06ddd96, whose
last two commits regenerated seven PGM checksums and the grid, peak and
lossless_color_transform PGM references for libwebp 0d14d84b, a change made
after v1.6.0. That is why the test data is pinned at 260804d (docs/WORKPLAN.md,
findings from WP0). The diagnostic expects exactly those entries to differ.
"""

import argparse
import datetime
import hashlib
import json
import os
from pathlib import Path
import platform
import re
import shutil
import subprocess

ROOT = Path(__file__).resolve().parent.parent
TEST_DATA = ROOT / "upstream/libwebp-test-data"
FORMATS = ["bmp", "pam", "pgm", "ppm", "tiff"]
# Test-data revision whose references were regenerated after libwebp v1.6.0.
POST_V1_6_0_REFERENCES = "06ddd96e276c2c638a72d39d3c0f340afd61978c"
UPDATED_AFTER_V1_6_0 = ["grid.pgm", "libwebp_tests.md5", "lossless_color_transform.pgm", "peak.pgm"]


def git(*args):
    return subprocess.run(["git", "-C", str(TEST_DATA), *args], check=True,
                          stdout=subprocess.PIPE).stdout


def md5_tool():
    """Return an md5sum-compatible program that accepts `-c -`."""
    for name in ("md5sum", "gmd5sum"):
        path = shutil.which(name)
        if path:
            return path
    raise SystemExit("test_dwebp.sh needs md5sum (GNU coreutils, or macOS 14 or later); "
                     "install coreutils for gmd5sum")


def run_script(script, args, cwd, log):
    command = ["sh", script, *args]
    print("+ " + " ".join(command), flush=True)
    result = subprocess.run(command, cwd=cwd, stdout=subprocess.PIPE, stderr=subprocess.STDOUT,
                            text=True)
    log.write_text(result.stdout)
    return result.returncode, result.stdout


def checksum_list(text):
    return [tuple(line.split()) for line in text.splitlines() if line.strip()]


# test_cwebp.sh option sets: the script's own flags.
CWEBP_OPTION_SETS = {
    "default": [],
    "lossless": ["--lossless"],
    "mt_noalpha_q30_m6": ["--mt", "--noalpha", "--extra_args=-q 30 -m 6 -sharp_yuv -af"],
}


def cwebp_inputs(cwebp, directory):
    """The test-data files that `cwebp` reads, sorted by name."""
    inputs = []
    for path in sorted(directory.iterdir()):
        if path.suffix in (".sh", ".md5", ".txt") or not path.is_file():
            continue
        result = subprocess.run([str(cwebp), "-quiet", "-o", os.devnull, str(path)],
                                stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if result.returncode == 0:
            inputs.append(path.name)
    return inputs


def cwebp_baseline(work, cwebp, md5):
    """test_cwebp.sh with the oracle's cwebp, per option set."""
    copy = fresh_copy(work / "cwebp")
    inputs = cwebp_inputs(cwebp, copy)
    results = {"inputs": inputs}
    for name, options in CWEBP_OPTION_SETS.items():
        code, log = run_script("test_cwebp.sh", [f"--exec={cwebp}", f"--md5exec={md5}", *options,
                                                  *inputs], copy, work / f"test_cwebp-{name}.log")
        results[name] = {"exit_status": code, "done": log.rstrip().endswith("DONE (1 of 1)"),
                         "files_ok": len(re.findall(r"^\S+:\tOK$", log, re.M)),
                         "failures": log.count("FAILED")}
    return results


def lossless_vectors():
    names = sorted(path.name for path in TEST_DATA.glob("lossless_vec_*.webp"))
    return names + ["lossless_color_transform.webp"]


def lossless_reference(vector):
    if vector.startswith("lossless_vec_1_"):
        return "grid"
    if vector.startswith("lossless_vec_2_"):
        return "peak"
    return "lossless_color_transform"


def fresh_copy(destination):
    if destination.exists():
        shutil.rmtree(destination)
    shutil.copytree(TEST_DATA, destination, ignore=shutil.ignore_patterns(".git"))
    return destination


def pinned_baseline(work, dwebp, md5):
    copy = fresh_copy(work / "pinned")
    code, log = run_script("test_dwebp.sh", [f"--exec={dwebp}", f"--md5exec={md5}",
                                              str(copy / "libwebp_tests.md5")], copy,
                           work / "test_dwebp.log")
    verified = re.findall(r"^(\S+): OK$", log, re.M)
    dwebp_result = {"exit_status": code, "done": log.rstrip().endswith("DONE"),
                    "verifications_ok": len(verified), "distinct_checksums_ok": len(set(verified)),
                    "failures": log.count("FAILED")}
    code, log = run_script("test_lossless.sh", [f"--exec={dwebp}"], copy, work / "test_lossless.log")
    compared = re.findall(r"^(\S+): OK$", log, re.M)
    lossless_result = {"exit_status": code, "all_tests_ok": log.rstrip().endswith("ALL TESTS OK"),
                       "comparisons_ok": len(compared),
                       "distinct_vectors_ok": len({name.rsplit(".", 1)[0] for name in compared})}
    return {"test_dwebp": dwebp_result, "test_lossless": lossless_result}


def post_v1_6_0_diagnostic(work, dwebp, md5):
    """Compare the oracle with the references regenerated after v1.6.0."""
    try:
        changed = sorted(git("diff", "--name-only", "HEAD", POST_V1_6_0_REFERENCES).decode().split())
    except subprocess.CalledProcessError:
        raise SystemExit(f"test-data revision {POST_V1_6_0_REFERENCES} is unavailable; "
                         "fetch the full upstream/libwebp-test-data history")
    if changed != UPDATED_AFTER_V1_6_0:
        raise SystemExit(f"unexpected test-data changes up to {POST_V1_6_0_REFERENCES}: {changed}")
    copy = fresh_copy(work / "post-v1.6.0")
    for name in UPDATED_AFTER_V1_6_0:
        (copy / name).write_bytes(git("show", f"{POST_V1_6_0_REFERENCES}:{name}"))
    regenerated = sorted(
        name for _, name in set(checksum_list((copy / "libwebp_tests.md5").read_text()))
        - set(checksum_list((TEST_DATA / "libwebp_tests.md5").read_text())))
    references = dict((name, digest) for digest, name in checksum_list(
        (copy / "libwebp_tests.md5").read_text()))
    code, log = run_script("test_dwebp.sh", [f"--exec={dwebp}", f"--md5exec={md5}", "--dump-md5s",
                                              str(copy / "libwebp_tests.md5")], copy,
                           work / "post-v1.6.0-test_dwebp-dump.log")
    dumped = dict((name, digest) for digest, name in checksum_list(
        "\n".join(line for line in log.splitlines() if re.fullmatch(r"[0-9a-f]{32}  \S+", line))))
    differing = sorted(name for name, digest in references.items() if dumped.get(name) != digest)
    pgm_differing = []
    for vector in lossless_vectors():
        output = copy / f"{vector}.pgm"
        subprocess.run([str(dwebp), str(copy / vector), "-pgm", "-o", str(output)], check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        if output.read_bytes() != (copy / f"{lossless_reference(vector)}.pgm").read_bytes():
            pgm_differing.append(vector)
        output.unlink()
    result = {"revision": POST_V1_6_0_REFERENCES, "regenerated_checksums": regenerated,
              "dump_exit_status": code, "checksums_matching": len(references) - len(differing),
              "checksums_differing": differing,
              "lossless_pgm_vectors_differing": len(pgm_differing)}
    problems = []
    if code != 0 or len(dumped) != len(references) or differing != regenerated:
        problems.append("post-v1.6.0 diagnostic: unexpected checksum differences")
    if len(pgm_differing) != len(lossless_vectors()):
        problems.append("post-v1.6.0 diagnostic: a lossless PGM output matches a regenerated reference")
    return result, problems


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--variant", default="release", help="oracle variant (default: release)")
    parser.add_argument("--diagnose-post-v1.6.0", dest="diagnose", action="store_true",
                        help="also compare with the references of test-data 06ddd96")
    args = parser.parse_args()
    oracle = ROOT / "build/oracle" / args.variant
    manifest = json.loads((oracle / "oracle.json").read_text())
    dwebp = Path(manifest["tools"]["dwebp"])
    cwebp = Path(manifest["tools"]["cwebp"])
    for tool in (dwebp, cwebp):
        if not tool.is_file():
            raise SystemExit(f"{tool} is missing; run python3 tools/build_oracle.py --variant {args.variant}")
    md5 = md5_tool()

    work = oracle / "baseline"
    if work.exists():
        shutil.rmtree(work)
    work.mkdir(parents=True)
    checksums = checksum_list((TEST_DATA / "libwebp_tests.md5").read_text())
    vectors = lossless_vectors()
    expected = {
        "dwebp_checksums": len(checksums),
        "dwebp_vectors": len({name.rsplit(".", 1)[0] for _, name in checksums}),
        "dwebp_verifications": 2 * len(checksums),
        "lossless_vectors": len(vectors),
        "lossless_comparisons": 2 * len(FORMATS) * len(vectors),
    }
    pinned = pinned_baseline(work, dwebp, md5)
    pinned["test_cwebp"] = cwebp_baseline(work, cwebp, md5)
    expected["cwebp_inputs"] = len(pinned["test_cwebp"]["inputs"])
    record = {
        "date": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "host": f"{platform.system()} {platform.release()} {platform.machine()}",
        "variant": args.variant,
        "c_compiler_version": manifest["c_compiler_version"],
        "dwebp": str(dwebp),
        "dwebp_sha256": hashlib.sha256(dwebp.read_bytes()).hexdigest(),
        "cwebp": str(cwebp),
        "cwebp_sha256": hashlib.sha256(cwebp.read_bytes()).hexdigest(),
        "md5_tool": md5,
        "md5_tool_resolved": os.path.realpath(md5),
        "test_data_revision": git("rev-parse", "HEAD").decode().strip(),
        "expected": expected,
        "baseline": pinned,
    }
    problems = []
    if args.diagnose:
        record["post_v1_6_0_diagnostic"], problems = post_v1_6_0_diagnostic(work, dwebp, md5)
    (work / "baseline.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps(record, indent=2))

    dwebp_result, lossless_result = pinned["test_dwebp"], pinned["test_lossless"]
    if dwebp_result["exit_status"] != 0 or not dwebp_result["done"] or dwebp_result["failures"]:
        problems.append("test_dwebp.sh failed; see test_dwebp.log")
    if (dwebp_result["verifications_ok"], dwebp_result["distinct_checksums_ok"]) != (
            expected["dwebp_verifications"], expected["dwebp_checksums"]):
        problems.append("test_dwebp.sh verified an unexpected number of checksums")
    if lossless_result["exit_status"] != 0 or not lossless_result["all_tests_ok"]:
        problems.append("test_lossless.sh failed; see test_lossless.log")
    if (lossless_result["comparisons_ok"], lossless_result["distinct_vectors_ok"]) != (
            expected["lossless_comparisons"], expected["lossless_vectors"]):
        problems.append("test_lossless.sh ran an unexpected number of comparisons")
    cwebp_result = pinned["test_cwebp"]
    for name in CWEBP_OPTION_SETS:
        result = cwebp_result[name]
        if result["exit_status"] != 0 or not result["done"] or result["failures"]:
            problems.append(f"test_cwebp.sh ({name}) failed; see test_cwebp-{name}.log")
        if result["files_ok"] != expected["cwebp_inputs"]:
            problems.append(f"test_cwebp.sh ({name}) checked an unexpected number of files")
    if problems:
        raise SystemExit("\n".join(problems))
    print(f"Baseline passed: {dwebp_result['distinct_checksums_ok']} checksums verified "
          f"{dwebp_result['verifications_ok']} times, {lossless_result['distinct_vectors_ok']} "
          f"lossless vectors in {lossless_result['comparisons_ok']} comparisons, test_cwebp.sh "
          f"{expected['cwebp_inputs']} inputs in {len(CWEBP_OPTION_SETS)} option sets", flush=True)


if __name__ == "__main__":
    main()
