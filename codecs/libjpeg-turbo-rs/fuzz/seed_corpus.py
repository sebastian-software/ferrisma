#!/usr/bin/env python3
"""Write the starting corpus of the `decode` fuzz target (ADR 0009).

Usage: fuzz/seed_corpus.py OUT [--oracle DIR]

Copies the JPEG inputs of the port's existing tests into OUT, unchanged:

  - every JPEG file in upstream/libjpeg-turbo/testimages, the 12-bit
    monkey12.jpg included as an error path;
  - with --oracle, the in-scope djpeg inputs of tools/bittests.json that
    upstream's CTest suite writes into the oracle's build tree
    (build/oracle/<variant>/libjpeg-turbo): progressive, arithmetic, 4:4:0,
    3x2 sampling, grayscale, optimized Huffman and ICC files; run
    `tools/build_oracle.py --variant release` and
    `tools/bittests.py --run --variant release` first;
  - the JPEG fixtures of the Ferrisma engine in tests/fixtures (CMYK, EXIF
    orientation).

The T.83 compliance data is not used: ITU grants no license to redistribute
it, and campaign artifacts upload the corpus (ADR 0002).

Prints the SHA-256 of every seed. Fails if an expected input is missing.
"""
import argparse
import json
import shutil
import sys
from hashlib import sha256
from pathlib import Path

CODEC = Path(__file__).resolve().parents[1]
ROOT = CODEC.parents[1]
TESTIMAGES = CODEC / "upstream" / "libjpeg-turbo" / "testimages"


def main():
    parser = argparse.ArgumentParser(description=__doc__,
                                     formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("out", type=Path, help="corpus directory to fill")
    parser.add_argument("--oracle", type=Path,
                        help="oracle variant directory whose CTest suite has run")
    args = parser.parse_args()

    seeds = {}
    images = sorted(TESTIMAGES.glob("*.jpg"))
    if not images:
        sys.exit(f"no test images in {TESTIMAGES}; initialize the upstream submodule")
    for path in images:
        seeds[f"testimages-{path.name}"] = path

    if args.oracle:
        build = args.oracle.resolve() / "libjpeg-turbo"
        manifest = json.loads((CODEC / "tools" / "bittests.json").read_text())
        inputs = sorted({case["input"] for case in manifest["cases"]
                         if case["scope"] == "in" and not case["input"].startswith("testimages/")})
        for name in inputs:
            path = build / name
            if not path.is_file():
                sys.exit(f"{path} is missing; run tools/bittests.py --run for this oracle")
            seeds[f"bittest-{name}"] = path

    fixtures = sorted((ROOT / "tests" / "fixtures").rglob("*.jpg"))
    for path in fixtures:
        relative = path.relative_to(ROOT / "tests" / "fixtures")
        seeds["fixture-" + "-".join(relative.parts)] = path

    args.out.mkdir(parents=True, exist_ok=True)
    for name, path in sorted(seeds.items()):
        shutil.copyfile(path, args.out / name)
        print(f"{sha256(path.read_bytes()).hexdigest()}  {name}")


if __name__ == "__main__":
    main()
