#!/usr/bin/env python3
"""Fetch the ISO/IEC 10918-2 (ITU-T T.83) compliance data for work package 11.

ITU publishes the T.83 test data as one zip file. Its Readme says
"(c) ITU & ISO/IEC 1994" and grants no license to redistribute it, so the
repository does not contain it (ADR 0002). This script downloads the zip from
ITU, or takes a local copy with --zip, verifies its size and SHA-256 and
extracts the files in scope into build/t83/:

  A1 B1 B2 C1 C2 D1 D2 G1 H1 K1 L1 .JPG  the 8-bit DCT streams (baseline,
                                         extended and progressive, Huffman
                                         and arithmetic coding)
  DREF_A8 ... DREF_D8 .DCT               the 8-bit decoder reference test data
                                         (ISO/IEC 10918-2 3.1.34)
  EREF_A8 ... EREF_D8 .DCT               the 8-bit encoder reference test data
                                         (3.1.35): the quantized coefficients
                                         that the streams code
  T83_data_files.zip                     the verified zip, reused next time
  t83.json                               source, hashes and the T.83 folders
                                         that hold a copy of each file

The zip keeps identical copies of a stream in several process folders. Every
copy must equal the pinned SHA-256 of its file; any other copy stops the
script. 12-bit and lossless streams, the encoder tests (the source images)
and the streams T.83 never published are out of scope (ADR 0001,
docs/PORT_STATUS.md); EREF serves as the exact coefficient reference.

tests/wp11_conformance.rs reads the directory named by LIBJPEG_T83_DIR,
default build/t83. Only the Python standard library is used.
"""

import argparse
import hashlib
import json
from pathlib import Path, PurePosixPath
import shutil
import sys
import tempfile
import urllib.request
import zipfile

ROOT = Path(__file__).resolve().parent.parent
DEFAULT_DIR = ROOT / "build/t83"
URL = "https://www.itu.int/wftp3/Public/t/testsignal/SpeImage/T83/v1994_11/T83_data_files.zip"
ZIP_NAME = "T83_data_files.zip"
ZIP_SIZE = 3_924_283
ZIP_SHA256 = "9f5ed0a74a3087c6c93f4cbb0bd5128c2a87b128e4f59f5b6649e32941da7069"
MANIFEST = "t83.json"

# The files in scope: name, size and SHA-256.
FILES = {
    "A1.JPG": (32108, "f893c20d82e35701ed96c28e4857f1f42b709e3fd592c1102fe97a6e54cc8e40"),
    "B1.JPG": (971, "48d3dc5026b4e8ca751e2cf9fb6c576364f92bafd339c19997c12c470946950c"),
    "B2.JPG": (854774, "b07e02e8ff185462285685ff6768d7e80133adc4fc9793bc2b35e3b23a48a739"),
    "C1.JPG": (27773, "382463b80d3a9f552cb1820a09c0bfc9482113099c466252f1ee26af856aefd8"),
    "C2.JPG": (27923, "b965fef07a5d731e409491fba9a423ea6d2e6078f6adde7339c53e2534ed85eb"),
    "D1.JPG": (28297, "e340bd5014555136dffd2f1461bc9e63c4844830d02c8bc9d03f1ad1c69465dd"),
    "D2.JPG": (30324, "48e06aa5406ee331aab569374d76929381df022b71a18f48aa89b98d6e76c5c4"),
    "G1.JPG": (28160, "6784b3037fb6c19f35c7b2c2f9b0072555132521d8cda6b5192c1f923350e8f6"),
    "H1.JPG": (30590, "e729dd0f2b43ef6dd4d0ca70184b9e40d25cdcb5e224b07fa04d67bf419878e9"),
    "K1.JPG": (28863, "a393d8fb87caf52d1072e32e98786024e1baf6273443e10430138c9cdfbc724b"),
    "L1.JPG": (31220, "d8233b544fadfa3695b0dcf88320d1a949d0191cee80ea59e19b908fb663bee1"),
    "DREF_A8.DCT": (12672, "fe10f8945ad965a84de12c36b19133689a34b852f8999348eac013b6a4db0647"),
    "DREF_B8.DCT": (23936, "de58bd2921caf2da59e7e5632c447bff932db5cc4ef70aa13d916439ffce2f0c"),
    "DREF_C8.DCT": (36864, "24ce45330fe27b3b3d272b551c5b4bbe0624c94fbea11e589a935c1f96f1a9f9"),
    "DREF_D8.DCT": (46464, "f1466dbf049a31b81d57cf13fdc745502c69c8ceaca4857c850fbec2908a99c9"),
    "EREF_A8.DCT": (12672, "9c774394d8592d6d73940301579b0caab5f034c156868041322f68c9cd8930c1"),
    "EREF_B8.DCT": (23936, "cb2903872e09160362bd60381783f1473e1f615c9f2eabf9838d738b3f774324"),
    "EREF_C8.DCT": (36864, "24ce45330fe27b3b3d272b551c5b4bbe0624c94fbea11e589a935c1f96f1a9f9"),
    "EREF_D8.DCT": (46464, "f1466dbf049a31b81d57cf13fdc745502c69c8ceaca4857c850fbec2908a99c9"),
}


def sha256(data):
    return hashlib.sha256(data).hexdigest()


def verify_zip(path):
    """Raise SystemExit unless `path` is the published zip."""
    size = path.stat().st_size
    if size != ZIP_SIZE:
        raise SystemExit(f"{path}: {size} bytes, expected {ZIP_SIZE}")
    digest = sha256(path.read_bytes())
    if digest != ZIP_SHA256:
        raise SystemExit(f"{path}: SHA-256 {digest}, expected {ZIP_SHA256}")


def download(destination):
    """Download the zip from ITU into `destination` and verify it."""
    print(f"+ download {URL}", flush=True)
    with tempfile.NamedTemporaryFile(dir=destination.parent, delete=False) as part:
        with urllib.request.urlopen(URL, timeout=120) as response:
            shutil.copyfileobj(response, part)
    part_path = Path(part.name)
    try:
        verify_zip(part_path)
    except SystemExit:
        part_path.unlink()
        raise
    part_path.replace(destination)


def extract(archive, directory):
    """Extract the files in scope; return the folders that hold each one."""
    folders = {name: [] for name in FILES}
    contents = {}
    with zipfile.ZipFile(archive) as zipped:
        for member in zipped.infolist():
            path = PurePosixPath(member.filename)
            if member.is_dir() or path.name not in FILES:
                continue
            data = zipped.read(member)
            size, digest = FILES[path.name]
            if len(data) != size or sha256(data) != digest:
                raise SystemExit(f"{member.filename}: differs from the pinned {path.name} "
                                 f"({len(data)} bytes, SHA-256 {sha256(data)})")
            folders[path.name].append(str(path.parent))
            contents[path.name] = data
    missing = sorted(name for name in FILES if name not in contents)
    if missing:
        raise SystemExit(f"{archive}: missing {', '.join(missing)}")
    for name, data in contents.items():
        (directory / name).write_bytes(data)
    return {name: sorted(paths, key=lambda folder: (len(folder), folder))
            for name, paths in folders.items()}


def fetch(directory, zip_path=None):
    directory.mkdir(parents=True, exist_ok=True)
    cached = directory / ZIP_NAME
    if zip_path is not None:
        verify_zip(zip_path)
        if zip_path.resolve() != cached.resolve():
            shutil.copyfile(zip_path, cached)
    elif cached.is_file():
        verify_zip(cached)
        print(f"Using the verified {cached}", flush=True)
    else:
        download(cached)
    folders = extract(cached, directory)
    manifest = {
        "source": URL,
        "zip": {"name": ZIP_NAME, "size": ZIP_SIZE, "sha256": ZIP_SHA256},
        "copyright": "(c) ITU & ISO/IEC 1994; not redistributed",
        "files": {name: {"size": size, "sha256": digest, "folders": folders[name]}
                  for name, (size, digest) in FILES.items()},
    }
    (directory / MANIFEST).write_text(json.dumps(manifest, indent=2) + "\n")
    print(f"T.83 data: {len(FILES)} files in {directory}; LIBJPEG_T83_DIR={directory}",
          flush=True)
    return directory


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--zip", type=Path, help="use this copy of the zip instead of downloading")
    parser.add_argument("--dir", type=Path, default=DEFAULT_DIR,
                        help="output directory (default: build/t83)")
    args = parser.parse_args()
    fetch(args.dir.resolve(), args.zip)


if __name__ == "__main__":
    sys.exit(main())
