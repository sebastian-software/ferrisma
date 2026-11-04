#!/usr/bin/env python3
"""Write the standalone public decoder captures from the original oracle.

Each row of tools/captures/decoder_public.tsv is a dpub-run request and the
observation of the original public C API. Inputs come from the original public
encoder. The ordinary Rust test replays them without C++; the public decoder
audit regenerates them from both original graphs and requires equality.
"""
import argparse
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parent.parent
CAPTURES = [
    ("w=8 h=8 c=3", "s320 L1,7,0,100,1"),
    ("w=13 h=7 c=1 bits=16", "s64 k1 L3,1,0,100,1 b"),
    ("w=8 h=8 c=3 icc=1", "s256 L2,5,0,100,1 z0 Z0,0 E0 E1"),
    ("w=8 h=8 c=3 container=1 box=Exif:20:0 box=xml:30:1 box=jumb:12:0 icc=1",
     "s82240 d1 L3,13,8,1000,1"),
    ("w=16 h=12 c=3 a=1 ec=2 names=1 orient=6 intrinsic=1 container=1 box=Exif:40:1",
     "s16448 d0 L5,64,3,1000,1"),
    ("w=12 h=10 c=3 frames=3 duration=5 timecodes=1 container=1 index=1",
     "s16384 L1,1000000,0,1000,1"),
    ("w=8 h=8 c=3 container=1 level=10", "s320 O4 t0 y L1,1,0,1000,0 w L2,3,0,1000,1"),
    ("w=8 h=8 c=3 container=1", "s1 s64 I0,4 I0,4 i c I0,1 p p"),
    ("w=8 h=8 c=3 container=1", "s16448 L1,1,2,1000,1"),
]


def oracle_lines(oracle, lines):
    result = subprocess.run([str(oracle)], input="\n".join(lines) + "\n", capture_output=True,
                            text=True, check=True)
    if result.stderr:
        raise RuntimeError(result.stderr)
    return result.stdout.splitlines()


def generate(oracle):
    specs = sorted({spec for spec, _ in CAPTURES}, key=[s for s, _ in CAPTURES].index)
    files = dict(zip(specs, oracle_lines(oracle, [f"dpub-encode {s}" for s in specs])))
    requests = []
    for index, (spec, script) in enumerate(CAPTURES):
        mm = "1 0" if "icc=1" not in spec else "2 0"
        if index == len(CAPTURES) - 1:
            mm = "1 3"
        requests.append(f"dpub-run {mm} {files[spec]} {script}")
    observations = oracle_lines(oracle, requests)
    return "".join(f"{r}\t{o}\n" for r, o in zip(requests, observations))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--oracle", type=Path,
                        default=Path(os.environ.get("JXL_CPP_ORACLE", ROOT / "target/oracle/jxl_oracle")))
    parser.add_argument("--check", action="store_true", help="Compare with the retained captures")
    args = parser.parse_args()
    text = generate(args.oracle)
    target = ROOT / "tools/captures/decoder_public.tsv"
    if args.check:
        if target.read_text() != text:
            raise SystemExit("Retained public decoder captures differ from the original")
        print(f"{len(CAPTURES)} retained public decoder captures match the original")
    else:
        target.write_text(text)


if __name__ == "__main__":
    main()
