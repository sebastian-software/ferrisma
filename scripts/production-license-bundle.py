#!/usr/bin/env python3
"""Collect a target-specific Cargo dependency and notice review bundle.

This is evidence for review, not a determination of distribution obligations.
"""
from __future__ import annotations

import argparse
import hashlib
import importlib.util
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SPEC = importlib.util.spec_from_file_location("audit", ROOT / "scripts/audit-licenses.py")
assert SPEC and SPEC.loader
AUDIT = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(AUDIT)

NOTICE = re.compile(r"^(license|licence|copying|notice|patents?|authors|copyright)([._-].*)?$", re.I)
TREE_LINE = re.compile(r"^([^ ]+) v([^ ]+)(?: .*)?$")
EXTRA = {
    "ferrisma": ["src/av1_tuning.rs", "THIRD-PARTY-NOTICES.md", "assets/icc/README.md", "assets/icc/sRGB2014.icc"],
    "libjpeg-turbo-rs": ["README.ijg", "src/jdcolext_neon.rs", "src/jsimdcpu.rs"],
}


def command(args: list[str]) -> str:
    return subprocess.run(args, cwd=ROOT, text=True, stdout=subprocess.PIPE, check=True).stdout


def parse_tree(output: str) -> set[tuple[str, str]]:
    result = set()
    for line in output.splitlines():
        match = TREE_LINE.fullmatch(line)
        if not match:
            raise ValueError(f"Unrecognized cargo tree output: {line!r}")
        result.add((match.group(1), match.group(2)))
    if not result:
        raise ValueError("Empty cargo tree")
    return result


def select_packages(metadata: dict, normal: set, all_edges: set) -> list[tuple[dict, str]]:
    if not normal <= all_edges:
        raise ValueError("Normal dependency graph is not contained in normal/build graph")
    by_key = {}
    for package in metadata["packages"]:
        key = (package["name"], package["version"])
        by_key.setdefault(key, []).append(package)
    result = []
    for key in sorted(all_edges):
        matches = by_key.get(key, [])
        if len(matches) != 1:
            raise ValueError(f"Cannot uniquely identify {key}: {len(matches)} metadata matches")
        result.append((matches[0], "normal" if key in normal else "build-only"))
    return result


def digest(path: Path) -> str:
    return hashlib.sha256(path.read_bytes()).hexdigest()


def notices(directory: Path, recursive: bool) -> list[Path]:
    paths = directory.rglob("*") if recursive else directory.iterdir()
    return sorted(p for p in paths if p.is_file() and
                  (NOTICE.match(p.name) or "LICENSES" in p.parts or
                   ("licenses" in p.parts and p.suffix.lower() == ".txt")))


def workspace_hash(directory: Path) -> str:
    relative = directory.relative_to(ROOT).as_posix()
    files = subprocess.run(["git", "ls-files", "--stage", "-z", "--", relative], cwd=ROOT,
                           stdout=subprocess.PIPE, check=True).stdout.split(b"\0")
    state = hashlib.sha256()
    for entry in sorted(filter(None, files)):
        header, name = entry.split(b"\t", 1)
        mode, object_id, stage = header.split(b" ")
        if stage != b"0":
            raise ValueError(f"Unmerged source entry: {name!r}")
        if mode == b"160000":
            state.update(name + b"\0gitlink\0" + object_id)
            continue
        path = ROOT / name.decode("utf-8")
        if not path.is_file():
            raise ValueError(f"Tracked source is missing: {path}")
        state.update(name + b"\0" + bytes.fromhex(digest(path)))
    return state.hexdigest()


def lock_checksums(text: str) -> dict[tuple[str, str, str], str]:
    result = {}
    for block in text.split("[[package]]")[1:]:
        values = dict(re.findall(r'^(name|version|source|checksum) = ("[^"\\]*(?:\\.[^"\\]*)*")$', block, re.M))
        values = {key: json.loads(value) for key, value in values.items()}
        if "source" in values and "checksum" in values:
            key = (values["name"], values["version"], values["source"])
            if key in result:
                raise ValueError(f"Duplicate lockfile package: {key}")
            result[key] = values["checksum"]
    return result


def git_blob(data: bytes) -> str:
    return hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()


def validate_recovered(entry: dict, source_hash: str, directory: Path, recovered_root: Path) -> list[Path]:
    if source_hash != entry["package_sha256"]:
        raise ValueError(f"Recovered license package checksum mismatch: {entry['name']}")
    vcs = json.loads((directory / ".cargo_vcs_info.json").read_text())
    if vcs["git"]["sha1"] != entry["upstream_revision"]:
        raise ValueError(f"Recovered license revision mismatch: {entry['name']}")
    if git_blob((directory / "Cargo.toml.orig").read_bytes()) != entry["upstream_manifest_blob_sha1"]:
        raise ValueError(f"Recovered license manifest mismatch: {entry['name']}")
    paths = []
    for item in entry["files"]:
        path = recovered_root / item["path"]
        if not path.resolve().is_relative_to(recovered_root.resolve()):
            raise ValueError(f"Invalid recovered license path: {path}")
        data = path.read_bytes()
        blob = git_blob(data)
        if digest(path) != item["sha256"] or blob != item["git_blob_sha1"]:
            raise ValueError(f"Recovered license file changed: {path}")
        paths.append(path)
    if not paths:
        raise ValueError(f"Recovered license has no files: {entry['name']}")
    return paths


def jpeg_source_review(package: dict, source_hash: str) -> str | None:
    if (package["name"], package["version"], package.get("license"), source_hash) == (
        "libjpeg-turbo-rs", "0.1.0", "IJG AND Zlib",
        "f5ba795a3d447b0adcaeef7e239cf45fb5495665529c5f11c9d7de599c0afca2"):
        return "docs/license-audit.md#jpeg-source-bound-evidence"
    return None


def build(target: str, features: str, output: Path) -> int:
    common = ["--locked", "--target", target, "--no-default-features", "--features", features]
    tree = ["cargo", "tree", "-p", "ferrisma", *common, "--prefix", "none", "--format", "{p}"]
    normal = parse_tree(command([*tree, "-e", "normal"]))
    all_edges = parse_tree(command([*tree, "-e", "normal,build"]))
    metadata = json.loads(command(["cargo", "metadata", "--format-version", "1", "--locked",
                                   "--filter-platform", target, "--no-default-features",
                                   "--features", features]))
    packages = select_packages(metadata, normal, all_edges)
    feature_tree = command(["cargo", "tree", "-p", "ferrisma", *common, "-e", "normal,build",
                            "--prefix", "none", "--format", "{p} | {f}"]).replace(str(ROOT), "<workspace>")
    if output.exists():
        raise ValueError(f"Output already exists: {output}")
    output.mkdir(parents=True)
    (output / "cargo-tree-features.txt").write_text(feature_tree)
    inventory = []
    checksums = lock_checksums((ROOT / "Cargo.lock").read_text())
    recovered = json.loads((ROOT / "licenses/recovered/manifest.json").read_text())
    for package, scope in packages:
        directory = Path(package["manifest_path"]).parent
        workspace = package["source"] is None
        source_hash = (workspace_hash(directory) if workspace else
                       checksums.get((package["name"], package["version"], package["source"])))
        if not source_hash:
            raise ValueError(f"Missing source checksum: {package['name']} {package['version']}")
        selected = notices(directory, recursive=not workspace)
        if workspace and package["name"] in {"ferrisma-kernels", "ferrisma-avif-read", "ferrisma-avif-write", "ferrisma-jxl-decoder"}:
            # The project-owned workspace crates use the root's dual license;
            # packaged crates carry linked copies of the same texts.
            selected.extend(ROOT / name for name in ("LICENSE-MIT", "LICENSE-APACHE")
                            if not (directory / name).is_file())
        if workspace and directory != ROOT:
            # Retained source terms for imported ports, including nested LICENSES.
            for subdir in ("LICENSES", "licenses"):
                if (directory / subdir).is_dir():
                    selected.extend(p for p in (directory / subdir).rglob("*") if p.is_file())
            if package["name"] == "libavif-rs":
                selected.extend(notices(directory / "upstream/libavif", recursive=False))
        for extra in EXTRA.get(package["name"], []):
            selected.append(directory / extra)
        selected = sorted(set(selected))
        copied = []
        for path in selected:
            if not path.is_file():
                raise ValueError(f"Missing expected notice: {path}")
            try:
                relative = path.relative_to(directory)
            except ValueError:
                relative = Path("project-license") / path.name
            destination = output / "notices" / f'{package["name"]}-{package["version"]}' / relative
            destination.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(path, destination)
            copied.append({"path": destination.relative_to(output).as_posix(), "sha256": digest(path)})
        provenance = None
        matches = [r for r in recovered if r["name"] == package["name"] and r["version"] == package["version"]]
        if matches:
            if len(matches) != 1 or workspace:
                raise ValueError(f"Ambiguous recovered license: {package['name']}")
            provenance = matches[0]
            if package["source"] != provenance["package_source"]:
                raise ValueError(f"Recovered license source mismatch: {package['name']}")
            for path in validate_recovered(provenance, source_hash, directory, ROOT / "licenses/recovered"):
                destination = output / "notices" / f'{package["name"]}-{package["version"]}' / "recovered" / path.name
                destination.parent.mkdir(parents=True, exist_ok=True)
                shutil.copyfile(path, destination)
                copied.append({"path": destination.relative_to(output).as_posix(), "sha256": digest(path)})
        source_review = jpeg_source_review(package, source_hash)
        status, note = AUDIT.classify(package.get("license"))
        inventory.append({"name": package["name"], "version": package["version"], "scope": scope,
                          "source": ("workspace/" + directory.relative_to(ROOT).as_posix()) if workspace else package["source"],
                          "source_sha256": source_hash, "source_hash_kind": "tracked-worktree-files" if workspace else "cargo-package",
                          "license": package.get("license"), "license_file": (Path(package["license_file"]).name if package.get("license_file") else None),
                          "metadata_status": status, "metadata_note": note, "source_review": source_review, "recovered_provenance": provenance, "notice_review": "FOUND_NOT_REVIEWED" if copied else "MISSING", "notices": copied})
    report = {"schema": 1, "target": target, "features": features, "default_features": False,
              "cargo_version": command(["cargo", "--version"]).strip(),
              "lockfile_sha256": digest(ROOT / "Cargo.lock"),
              "cargo_tree_features_sha256": digest(output / "cargo-tree-features.txt"),
              "git_head": command(["git", "rev-parse", "HEAD"]).strip(),
              "git_status": command(["git", "status", "--porcelain", "--untracked-files=all"]).splitlines(),
              "scope_note": "Cargo normal/build dependency graph, not a linked-binary inventory. Normal includes proc macros; build-only may execute on the host. Dev dependencies and standalone benchmark projects are excluded. Source and legal review remain open.",
              "packages": inventory}
    (output / "inventory.json").write_text(json.dumps(report, indent=2, sort_keys=True) + "\n")
    print(f"Collected {len(inventory)} packages in {output}; review all source and notice obligations.")
    issues = [p for p in inventory if p["metadata_status"] != "METADATA_OK" or p["notice_review"] == "MISSING"]
    for package in issues:
        print(f"REVIEW: {package['name']} {package['version']}: metadata={package['metadata_status']}, notices={package['notice_review']}", file=sys.stderr)
    return len(issues)


def bundle_file(root: Path, relative: str) -> Path:
    """Resolve a self-contained regular file without following bundle symlinks."""
    path = Path(relative)
    if path.is_absolute() or not path.parts or any(part in {".", ".."} for part in path.parts):
        raise ValueError(f"Invalid bundled file path: {relative}")
    current = root
    for part in path.parts:
        current = current / part
        if current.is_symlink():
            raise ValueError(f"Bundled file traverses a symlink: {current}")
    if not current.is_file():
        raise ValueError(f"Missing bundled file: {current}")
    return current


def verify(target: str, features: str, output: Path) -> int:
    recorded = json.loads(bundle_file(output, "inventory.json").read_text())
    feature_tree = bundle_file(output, "cargo-tree-features.txt")
    if digest(feature_tree) != recorded["cargo_tree_features_sha256"]:
        raise ValueError("Missing or altered Cargo feature tree")
    for package in recorded["packages"]:
        for notice in package["notices"]:
            path = bundle_file(output, notice["path"])
            if digest(path) != notice["sha256"]:
                raise ValueError(f"Missing or altered bundled notice: {path}")
    with tempfile.TemporaryDirectory(prefix="ferrisma-license-verify-") as temporary:
        candidate = Path(temporary) / "bundle"
        issues = build(target, features, candidate)
        current = json.loads((candidate / "inventory.json").read_text())
    # The worktree status is contextual evidence, not part of the source hash.
    recorded.pop("git_status")
    current.pop("git_status")
    if recorded != current:
        raise ValueError("Inventory does not match the current graph, sources, lockfile, or toolchain")
    print("Verified graph, source identities, and bundled notice hashes; legal review remains open.")
    return issues


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--target", required=True, help="Explicit Rust target triple")
    parser.add_argument("--features", required=True, help="Explicit comma-separated Ferrisma features")
    parser.add_argument("--output", required=True, type=Path, help="New output directory")
    parser.add_argument("--verify", action="store_true", help="Verify an existing bundle against the current checkout")
    parser.add_argument("--strict", action="store_true", help="Fail for review metadata or missing notice files")
    args = parser.parse_args()
    try:
        if args.verify:
            issues = verify(args.target, args.features, args.output.resolve())
        else:
            issues = build(args.target, args.features, args.output.resolve())
        return int(args.strict and issues > 0)
    except (OSError, ValueError, subprocess.CalledProcessError) as error:
        print(f"License bundle failed: {error}", file=sys.stderr)
        return 1


if __name__ == "__main__":
    raise SystemExit(main())
