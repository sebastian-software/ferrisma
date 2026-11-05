#!/usr/bin/env python3
"""Run upstream's dwebp conformance scripts against an oracle build.

Runs test_dwebp.sh and test_lossless.sh from the pinned libwebp-test-data with
the oracle's own dwebp and checks the exact number of passing comparisons. The
scripts write decoded files next to the vectors, so every run uses a fresh copy
under build/oracle/<variant>/baseline/ and upstream/ stays untouched. The
record is written to build/oracle/<variant>/baseline/baseline.json.

test_dwebp.sh decodes every vector named in libwebp_tests.md5 to bmp, pam, pgm,
ppm and tiff and verifies each output's MD5 twice, by default and with -noasm.
test_lossless.sh decodes the 32 lossless_vec_* vectors and
lossless_color_transform.webp to the same five formats, with and without
-noasm, and compares each output byte for byte with a reference image.

The pinned test data postdates libwebp v1.6.0: its last two commits (05cd58b,
06ddd96) regenerate seven PGM checksums and the grid, peak and
lossless_color_transform PGM references for upstream commit 0d14d84b ("Have
lossless use ImportYUVAFromRGB", after v1.6.0). The baseline therefore has two
parts (docs/PORT_STATUS.md):

1. v1.6.0 references: both scripts, unmodified, on a copy whose four updated
   files are taken from test-data revision 260804d, the last one before that
   update. Both scripts must pass completely.
2. Pinned references: on an unmodified copy, every MD5 that test_dwebp.sh
   --dump-md5s produces is compared with the pinned list, test_lossless.sh runs
   for the four non-PGM formats, and each lossless vector's PGM output is
   compared with the pinned reference. The expected differences are exactly
   the regenerated PGM entries.
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
# The last test-data revision whose references match libwebp v1.6.0.
V1_6_0_REFERENCES = "260804d686f0491e0326a50eef491829dae09e9e"
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


def v1_6_0_references(work, dwebp, md5):
    copy = fresh_copy(work / "v1.6.0-references")
    for name in UPDATED_AFTER_V1_6_0:
        (copy / name).write_bytes(git("show", f"{V1_6_0_REFERENCES}:{name}"))
    code, log = run_script("test_dwebp.sh", [f"--exec={dwebp}", f"--md5exec={md5}",
                                              str(copy / "libwebp_tests.md5")], copy,
                           work / "v1.6.0-test_dwebp.log")
    verified = re.findall(r"^(\S+): OK$", log, re.M)
    dwebp_result = {"exit_status": code, "done": log.rstrip().endswith("DONE"),
                    "verifications_ok": len(verified), "distinct_checksums_ok": len(set(verified)),
                    "failures": log.count("FAILED")}
    code, log = run_script("test_lossless.sh", [f"--exec={dwebp}"], copy,
                           work / "v1.6.0-test_lossless.log")
    compared = re.findall(r"^(\S+): OK$", log, re.M)
    lossless_result = {"exit_status": code, "all_tests_ok": log.rstrip().endswith("ALL TESTS OK"),
                       "comparisons_ok": len(compared),
                       "distinct_vectors_ok": len({name.rsplit(".", 1)[0] for name in compared})}
    return {"test_dwebp": dwebp_result, "test_lossless": lossless_result}


def pinned_references(work, dwebp, md5):
    copy = fresh_copy(work / "pinned")
    pinned = dict((name, digest) for digest, name in checksum_list(
        (copy / "libwebp_tests.md5").read_text()))
    code, log = run_script("test_dwebp.sh", [f"--exec={dwebp}", f"--md5exec={md5}", "--dump-md5s",
                                              str(copy / "libwebp_tests.md5")], copy,
                           work / "pinned-test_dwebp-dump.log")
    dumped = dict((name, digest) for digest, name in checksum_list(
        "\n".join(line for line in log.splitlines() if re.fullmatch(r"[0-9a-f]{32}  \S+", line))))
    differing = sorted(name for name, digest in pinned.items() if dumped.get(name) != digest)
    dwebp_result = {"exit_status": code, "dumped": len(dumped),
                    "matching": len(pinned) - len(differing), "differing": differing}

    non_pgm = [fmt for fmt in ("ppm", "pam", "bmp", "tiff")]
    code, log = run_script("test_lossless.sh", [f"--exec={dwebp}", f"--formats={' '.join(non_pgm)}"],
                           copy, work / "pinned-test_lossless-non-pgm.log")
    compared = re.findall(r"^(\S+): OK$", log, re.M)
    pgm_differing = []
    for vector in lossless_vectors():
        output = copy / f"{vector}.pgm"
        subprocess.run([str(dwebp), str(copy / vector), "-pgm", "-o", str(output)], check=True,
                       stdout=subprocess.DEVNULL, stderr=subprocess.DEVNULL)
        reference = copy / f"{lossless_reference(vector)}.pgm"
        if output.read_bytes() != reference.read_bytes():
            pgm_differing.append(vector)
        output.unlink()
    lossless_result = {"exit_status": code, "all_tests_ok": log.rstrip().endswith("ALL TESTS OK"),
                       "non_pgm_comparisons_ok": len(compared),
                       "pgm_vectors_matching": len(lossless_vectors()) - len(pgm_differing),
                       "pgm_vectors_differing": len(pgm_differing)}
    return {"test_dwebp": dwebp_result, "test_lossless": lossless_result}


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--variant", default="release", help="oracle variant (default: release)")
    args = parser.parse_args()
    oracle = ROOT / "build/oracle" / args.variant
    manifest = json.loads((oracle / "oracle.json").read_text())
    dwebp = Path(manifest["tools"]["dwebp"])
    if not dwebp.is_file():
        raise SystemExit(f"{dwebp} is missing; run python3 tools/build_oracle.py --variant {args.variant}")
    md5 = md5_tool()
    try:
        changed = sorted(git("diff", "--name-only", V1_6_0_REFERENCES, "HEAD").decode().split())
    except subprocess.CalledProcessError:
        raise SystemExit(f"test-data revision {V1_6_0_REFERENCES} is unavailable; "
                         "fetch the full upstream/libwebp-test-data history")
    if changed != UPDATED_AFTER_V1_6_0:
        raise SystemExit(f"unexpected test-data changes since {V1_6_0_REFERENCES}: {changed}")
    changed_checksums = sorted(
        name for _, name in set(checksum_list((TEST_DATA / "libwebp_tests.md5").read_text()))
        - set(checksum_list(git("show", f"{V1_6_0_REFERENCES}:libwebp_tests.md5").decode())))

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
        "pinned_checksums_updated_after_v1_6_0": changed_checksums,
    }
    v1_6_0 = v1_6_0_references(work, dwebp, md5)
    pinned = pinned_references(work, dwebp, md5)
    record = {
        "date": datetime.datetime.now(datetime.timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ"),
        "host": f"{platform.system()} {platform.release()} {platform.machine()}",
        "variant": args.variant,
        "c_compiler_version": manifest["c_compiler_version"],
        "dwebp": str(dwebp),
        "dwebp_sha256": hashlib.sha256(dwebp.read_bytes()).hexdigest(),
        "md5_tool": md5,
        "md5_tool_resolved": os.path.realpath(md5),
        "test_data_revision": git("rev-parse", "HEAD").decode().strip(),
        "v1_6_0_reference_revision": V1_6_0_REFERENCES,
        "expected": expected,
        "v1_6_0_references": v1_6_0,
        "pinned_references": pinned,
    }
    (work / "baseline.json").write_text(json.dumps(record, indent=2) + "\n")
    print(json.dumps(record, indent=2))

    problems = []
    dwebp_v1, lossless_v1 = v1_6_0["test_dwebp"], v1_6_0["test_lossless"]
    if dwebp_v1["exit_status"] != 0 or not dwebp_v1["done"] or dwebp_v1["failures"]:
        problems.append("test_dwebp.sh failed on the v1.6.0 references")
    if (dwebp_v1["verifications_ok"], dwebp_v1["distinct_checksums_ok"]) != (
            expected["dwebp_verifications"], expected["dwebp_checksums"]):
        problems.append("test_dwebp.sh verified an unexpected number of checksums")
    if lossless_v1["exit_status"] != 0 or not lossless_v1["all_tests_ok"]:
        problems.append("test_lossless.sh failed on the v1.6.0 references")
    if (lossless_v1["comparisons_ok"], lossless_v1["distinct_vectors_ok"]) != (
            expected["lossless_comparisons"], expected["lossless_vectors"]):
        problems.append("test_lossless.sh ran an unexpected number of comparisons")
    dwebp_pin, lossless_pin = pinned["test_dwebp"], pinned["test_lossless"]
    if dwebp_pin["dumped"] != len(checksums) or dwebp_pin["differing"] != changed_checksums:
        problems.append("the pinned checksums differ from the oracle in unexpected entries")
    if lossless_pin["exit_status"] != 0 or not lossless_pin["all_tests_ok"] or \
            lossless_pin["non_pgm_comparisons_ok"] != 2 * 4 * len(vectors):
        problems.append("test_lossless.sh failed for the non-PGM formats on the pinned references")
    if lossless_pin["pgm_vectors_differing"] != len(vectors):
        problems.append("lossless PGM outputs unexpectedly match the regenerated pinned references")
    if problems:
        raise SystemExit("\n".join(problems))
    print(f"Baseline passed. v1.6.0 references: {dwebp_v1['distinct_checksums_ok']} checksums "
          f"verified {dwebp_v1['verifications_ok']} times, {lossless_v1['distinct_vectors_ok']} "
          f"lossless vectors in {lossless_v1['comparisons_ok']} comparisons. Pinned references: "
          f"{dwebp_pin['matching']} of {len(checksums)} checksums match, "
          f"{lossless_pin['non_pgm_comparisons_ok']} non-PGM lossless comparisons pass, "
          f"{lossless_pin['pgm_vectors_differing']} lossless PGM outputs differ as expected",
          flush=True)


if __name__ == "__main__":
    main()
