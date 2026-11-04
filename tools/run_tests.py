#!/usr/bin/env python3
"""Run an upstream CTest suite with a per-build temporary directory and JUnit output."""
import argparse
import os
from pathlib import Path
import subprocess
import shutil

ROOT = Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("build", type=Path)
    parser.add_argument("--jobs", type=int, default=4)
    parser.add_argument('--tests-regex', help='Explicit bounded CTest selection; omitted for the complete original suite')
    parser.add_argument('--evidence-dir', type=Path, help='Preserve this actual XML and complete log in a separate immutable directory')
    args = parser.parse_args()
    build = args.build.resolve()
    directory = args.evidence_dir.resolve() if args.evidence_dir else None
    if directory and not directory.is_relative_to((ROOT / 'build').resolve()):
        parser.error('--evidence-dir must remain inside the checkout build directory')
    temporary = build / "test-tmp"
    temporary.mkdir(exist_ok=True)
    environment = os.environ.copy()
    # The original metadata tests use fixed file names in the process-wide temp
    # directory. Keep suites isolated without modifying their source or cases.
    environment.update(TMPDIR=str(temporary), TMP=str(temporary), TEMP=str(temporary))
    if "-asan" in build.name:
        # Match upstream's sanitized CI: allocation-failure tests deliberately
        # request impossible allocations and expect NULL, rather than ASan abort.
        environment.setdefault("ASAN_OPTIONS", "allocator_may_return_null=1")
    command = ["ctest", "--test-dir", str(build), "--output-on-failure", "-j", str(args.jobs),
               "--output-junit", str(ROOT / "build" / f"{build.name}-results.xml")]
    if args.tests_regex:
        command += ['--tests-regex', args.tests_regex, '--no-tests=error']
    result = subprocess.run(command, cwd=ROOT, env=environment)
    if directory:
        directory.mkdir(parents=True, exist_ok=True)
        for source, name in [(ROOT / 'build' / f'{build.name}-results.xml', 'results.xml'),
                             (build / 'Testing/Temporary/LastTest.log', 'LastTest.log')]:
            if source.is_file():
                destination = directory / name
                if destination.exists() and destination.read_bytes() != source.read_bytes():
                    raise SystemExit('Refusing to overwrite different preserved CTest execution evidence')
                if not destination.exists():
                    shutil.copyfile(source, destination)
    result.check_returncode()


if __name__ == "__main__":
    main()
