#!/usr/bin/env python3
"""Run an upstream CTest suite with a per-build temporary directory and JUnit output."""
import argparse
import os
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--jobs", type=int, default=4)
    args = parser.parse_args()
    build = args.build.resolve()
    temporary = build / "test-tmp"
    temporary.mkdir(exist_ok=True)
    environment = os.environ.copy()
    # The original metadata tests use fixed file names in the process-wide temp
    # directory. Keep suites isolated without modifying their source or cases.
    environment.update(TMPDIR=str(temporary), TMP=str(temporary), TEMP=str(temporary))
    command = ["ctest", "--test-dir", str(build), "--output-on-failure", "-j", str(args.jobs),
               "--output-junit", str(ROOT / "build" / f"{build.name}-results.xml")]
    subprocess.run(command, cwd=ROOT, check=True, env=environment)


if __name__ == "__main__":
    main()
