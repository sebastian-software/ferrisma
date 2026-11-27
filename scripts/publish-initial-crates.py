#!/usr/bin/env python3
"""Rehearse or manually publish the initial functional Ferrisma release."""

from __future__ import annotations

import argparse
import hashlib
import io
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tarfile
import tempfile
from urllib.error import HTTPError
from urllib.request import Request, urlopen


ROOT = Path(__file__).resolve().parent.parent
VERSION = "0.1.0"
CRATES = (
    "ferrisma-sharpyuv",
    "ferrisma-kernels",
    "ferrisma-libjpeg-turbo",
    "ferrisma-libjxl",
    "ferrisma-libaom",
    "ferrisma-dav1d-kernels",
    "ferrisma-dav1d",
    "ferrisma-libwebp",
    "ferrisma-libavif-sharpyuv",
    "ferrisma-libavif",
    "ferrisma-avif-read",
    "ferrisma-avif-write",
    "ferrisma-jxl-decoder",
    "ferrisma-jxl-encoder",
    "ferrisma",
)
MAX_CRATE_BYTES = 10 * 1024 * 1024
REGISTRY_ARGS = ("--locked", "--registry", "crates-io")


def run(*args: str, cwd: Path = ROOT, capture: bool = False) -> str:
    result = subprocess.run(
        args, cwd=cwd, check=True, text=True,
        stdout=subprocess.PIPE if capture else None,
    )
    return result.stdout or ""


def validate_workspace() -> None:
    data = json.loads(run(
        "cargo", "metadata", "--offline", "--locked", "--no-deps",
        "--format-version", "1", capture=True,
    ))
    packages = {package["name"]: package for package in data["packages"]}
    missing = sorted(set(CRATES) - packages.keys())
    if missing:
        raise RuntimeError(f"Release crates missing from the workspace: {', '.join(missing)}")
    wrong_registry = [name for name in CRATES if packages[name]["publish"] != ["crates-io"]]
    if wrong_registry:
        raise RuntimeError(
            'Release manifests must set publish = ["crates-io"]. '
            f"Check out the Ferrisma {VERSION} release commit. Affected crates: {', '.join(wrong_registry)}"
        )
    unexpected = sorted(
        name for name, package in packages.items() if name not in CRATES and package["publish"] != []
    )
    if unexpected:
        raise RuntimeError(f"Crates outside the release set allow publication: {', '.join(unexpected)}")

    earlier = set()
    for name in CRATES:
        package = packages[name]
        if package["version"] != VERSION:
            raise RuntimeError(f"{name} is {package['version']}; the release version must be {VERSION}")
        for dependency in package["dependencies"]:
            if not dependency.get("path") or (dependency["kind"] == "dev" and dependency["req"] == "*"):
                continue
            if dependency["name"] not in earlier:
                raise RuntimeError(f"{name} needs {dependency['name']} earlier in the publish order")
        earlier.add(name)


def verify_packages(publish: bool) -> None:
    with tempfile.TemporaryDirectory(prefix="ferrisma-release-") as temp:
        staging = Path(temp)
        selection = [arg for name in CRATES for arg in ("-p", name)]
        # Cargo's temporary registry can fail verification with "no hash listed"
        # for unpublished dependencies. Build the actual archives in isolation.
        run(
            "cargo", "package", *REGISTRY_ARGS, "--no-verify",
            "--target-dir", temp, *([] if publish else ["--allow-dirty"]), *selection,
        )
        for name in CRATES:
            archive = staging / "package" / f"{name}-{VERSION}.crate"
            size = archive.stat().st_size
            if size > MAX_CRATE_BYTES:
                raise RuntimeError(f"{name} is {size:,} bytes, above the crates.io upload limit")
            print(f"{name} {VERSION}: {size:,} bytes", flush=True)
            # These archives were generated from this checkout by Cargo.
            run("tar", "-xzf", str(archive), cwd=staging)

        shutil.copyfile(ROOT / "rust-toolchain.toml", staging / "rust-toolchain.toml")
        engine = staging / f"ferrisma-{VERSION}"
        # Every internal crate must resolve to its packaged sources.
        with (engine / "Cargo.toml").open("a", encoding="utf-8") as manifest:
            manifest.write("\n[patch.crates-io]\n")
            for name in CRATES[:-1]:
                manifest.write(f'{name} = {{ path = "../{name}-{VERSION}" }}\n')
        run(
            "cargo", "run", "--features", "cli", "--bin", "ferrisma",
            "--target-dir", str(ROOT / "target" / "initial-release-build"), "--", "--version",
            cwd=engine,
        )


def download(url: str) -> bytes:
    request = Request(url, headers={
        "User-Agent": "ferrisma-initial-release (https://github.com/sebastian-software/ferrisma)",
        "Cache-Control": "no-cache",
    })
    with urlopen(request, timeout=30) as response:
        return response.read()


def index_versions(name: str) -> dict:
    try:
        content = download(f"https://index.crates.io/{name[:2]}/{name[2:4]}/{name}")
    except HTTPError as error:
        if error.code == 404:
            return {}
        raise
    return {record["vers"]: record for record in map(json.loads, content.splitlines())}


def verify_uploaded(name: str, record: dict, sha: str) -> None:
    label = f"{name}-{VERSION}"
    if record["yanked"]:
        raise RuntimeError(f"{label} is yanked on crates.io")
    archive = download(f"https://static.crates.io/crates/{name}/{label}.crate")
    if hashlib.sha256(archive).hexdigest() != record["cksum"]:
        raise RuntimeError(f"Registry checksum mismatch for {label}")
    with tarfile.open(fileobj=io.BytesIO(archive), mode="r:gz") as package:
        member = package.extractfile(f"{label}/.cargo_vcs_info.json")
        git = json.load(member)["git"] if member is not None else {}
    if git.get("sha1") != sha or git.get("dirty"):
        raise RuntimeError(f"{label} was published from a different checkout than {sha}")


def pending_crates(sha: str) -> list[str]:
    pending = []
    for name in CRATES:
        versions = index_versions(name)
        if VERSION in versions:
            verify_uploaded(name, versions[VERSION], sha)
            print(f"Already published from {sha[:12]}: {name} {VERSION}", flush=True)
        elif not versions or (name == "ferrisma" and set(versions) == {"0.0.0"}):
            pending.append(name)
        else:
            raise RuntimeError(f"{name} already exists on crates.io with unexpected versions")
    return pending


def clean_head() -> str:
    if run("git", "status", "--porcelain", "--untracked-files=all", capture=True):
        raise RuntimeError("Publish from a clean checkout of the reviewed release commit")
    return run("git", "rev-parse", "HEAD", capture=True).strip()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--dry-run", action="store_true", help="Build and check all packages (default)")
    mode.add_argument("--publish", action="store_true", help="Upload 0.1.0 using Cargo credentials")
    args = parser.parse_args()

    sha = clean_head() if args.publish else None
    validate_workspace()
    pending = pending_crates(sha) if sha is not None else []
    if sha is not None and not pending:
        print(f"All {len(CRATES)} crates at {VERSION} are already on crates.io from {sha}.")
        return

    verify_packages(args.publish)
    if sha is None:
        print(f"Dry run passed for all {len(CRATES)} crates at {VERSION}.")
        return

    for name in pending:
        if clean_head() != sha:
            raise RuntimeError("The release checkout changed during publication")
        print(f"Publishing {name} {VERSION} from {sha[:12]}", flush=True)
        try:
            features = ["--features", "cli"] if name == "ferrisma" else []
            run("cargo", "publish", *REGISTRY_ARGS, "-p", name, *features)
        except subprocess.CalledProcessError as error:
            raise RuntimeError(
                f"Cargo stopped at {name}. Resolve the error above and rerun --publish from {sha}."
            ) from error
        record = index_versions(name).get(VERSION)
        if record is None:
            raise RuntimeError(f"{name} is not indexed yet. Rerun --publish from {sha} once it is visible.")
        verify_uploaded(name, record, sha)
    print(f"All {len(CRATES)} crates at {VERSION} are on crates.io from {sha}.")


if __name__ == "__main__":
    try:
        main()
    except (RuntimeError, OSError, subprocess.CalledProcessError, tarfile.TarError, ValueError, KeyError) as error:
        sys.exit(f"Ferrisma release stopped: {error}")
