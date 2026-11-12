#!/usr/bin/env python3
"""Fetch and verify the pinned external JPEG XL conformance corpus, or
summarize a results file written by its official runner.

  corpus.py fetch <workdir>          download, verify, prepare <workdir>/corpus
  corpus.py summarize <results.json> print a per-case Markdown table

The corpus is `libjxl/conformance` at REVISION. Inputs, descriptors, corpus
lists and the runner scripts must match MANIFEST.sha256; reference images and
profiles are content-addressed objects verified against each test.json
`sha256sums` entry. Run with `python3 -I` so nothing from the download
directory is imported.
"""

import hashlib
import io
import json
import shutil
import sys
import tarfile
import urllib.request
from pathlib import Path

REVISION = "b1d0f990b03e57bf6d137c365cd5dc8b470b9191"
ARCHIVE = f"https://codeload.github.com/libjxl/conformance/tar.gz/{REVISION}"
OBJECTS = "https://storage.googleapis.com/jxl-conformance/objects/"
HERE = Path(__file__).resolve().parent


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def download(url):
    with urllib.request.urlopen(url) as response:
        return response.read()


def fetch(workdir):
    workdir = Path(workdir).resolve()
    corpus = workdir / "corpus"
    if corpus.exists():
        shutil.rmtree(corpus)
    corpus.mkdir(parents=True)
    prefix = f"conformance-{REVISION}/"
    with tarfile.open(fileobj=io.BytesIO(download(ARCHIVE)), mode="r:gz") as archive:
        members = {m.name[len(prefix):]: m for m in archive.getmembers()
                   if m.name.startswith(prefix)}
        for name, member in members.items():
            if not name or not (member.isfile() or member.issym()):
                continue
            target = corpus / name
            target.parent.mkdir(parents=True, exist_ok=True)
            if member.issym():
                # Resolve sibling links (`../case/input.jxl`) to file copies.
                source = members.get(_normalize(Path(name).parent, member.linkname))
                if source is None or not source.isfile():
                    continue  # content-addressed object, fetched below
                data = archive.extractfile(source).read()
            else:
                data = archive.extractfile(member).read()
            target.write_bytes(data)
    failures = 0
    for line in (HERE / "MANIFEST.sha256").read_text().splitlines():
        if not line or line.startswith("#"):
            continue
        digest, name = line.split("  ", 1)
        path = corpus / name
        if not path.is_file() or sha256(path.read_bytes()) != digest:
            print(f"MANIFEST MISMATCH {name}")
            failures += 1
    objects = workdir / "objects"
    objects.mkdir(exist_ok=True)
    for descriptor in sorted((corpus / "testcases").glob("*/test.json")):
        for name, digest in json.loads(descriptor.read_text()).get("sha256sums", {}).items():
            cached = objects / digest
            if not cached.exists():
                cached.write_bytes(download(OBJECTS + digest))
            data = cached.read_bytes()
            if sha256(data) != digest:
                print(f"OBJECT MISMATCH {descriptor.parent.name}/{name}")
                failures += 1
                continue
            (descriptor.parent / name).write_bytes(data)
    if failures:
        sys.exit(f"{failures} verification failures")
    print(f"verified corpus {REVISION} in {corpus}")


def _normalize(parent, link):
    parts = list(parent.parts)
    for part in Path(link).parts:
        if part == "..":
            parts.pop()
        elif part != ".":
            parts.append(part)
    return "/".join(parts)


def summarize(path):
    rows = json.loads(Path(path).read_text())
    print("| Case | Result | Frames | Max RMSE / limit | Max peak / limit | Exact files |")
    print("| --- | --- | ---: | --- | --- | --- |")
    for row in rows:
        frames = [row[f"frame{i}_compare_npy"] for i in range(row.get("num_frames", 0))]
        if frames and all("actual_rmse" in f for f in frames):
            rmse = f"{max(f['actual_rmse'] for f in frames):.3g} / {frames[0]['rmse_limit']:g}"
            peak = f"{max(f['actual_peak_error'] for f in frames):.3g} / {frames[0]['peak_error']:g}"
        else:
            message = row.get("message", "-")
            if message.startswith("Running the decoder"):
                message = "decoder exited with an error"
            rmse = peak = message
        exact = ", ".join(
            f"{name} {'identical' if row[f'compare_binary_{name}']['success'] else 'DIFFERS'}"
            for name in row.get("exact_tests", [])) or "-"
        result = "pass" if row.get("success") else "FAIL"
        print(f"| `{row['test_id']}` | {result} | {len(frames)} | {rmse} | {peak} | {exact} |")
    passed = sum(1 for row in rows if row.get("success"))
    print(f"\n{passed} of {len(rows)} passed")
    return passed == len(rows)


if __name__ == "__main__":
    if len(sys.argv) == 3 and sys.argv[1] == "fetch":
        fetch(sys.argv[2])
    elif len(sys.argv) == 3 and sys.argv[1] == "summarize":
        sys.exit(0 if summarize(sys.argv[2]) else 1)
    else:
        sys.exit(__doc__)
