#!/usr/bin/env python3
"""Tests for scoped license inventory identity and failure behavior."""
import hashlib
import importlib.util
import json
import tempfile
import unittest
from pathlib import Path
from unittest import mock

SCRIPT = Path(__file__).resolve().with_name("production-license-bundle.py")
SPEC = importlib.util.spec_from_file_location("production_license_bundle", SCRIPT)
assert SPEC and SPEC.loader
BUNDLE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(BUNDLE)


class ProductionLicenseBundleTests(unittest.TestCase):
    def test_tree_selection_excludes_dev_and_labels_build_only(self):
        normal = BUNDLE.parse_tree("root v1.0.0 (/work)\nshared v2.0.0 (*)\n")
        all_edges = BUNDLE.parse_tree("root v1.0.0 (/work)\nshared v2.0.0\ntool v3.0.0 (proc-macro)\n")
        metadata = {"packages": [{"name": n, "version": v} for n, v in
                                 [("root", "1.0.0"), ("shared", "2.0.0"),
                                  ("tool", "3.0.0"), ("dev", "4.0.0")]]}
        self.assertEqual([(p["name"], scope) for p, scope in BUNDLE.select_packages(metadata, normal, all_edges)],
                         [("root", "normal"), ("shared", "normal"), ("tool", "build-only")])

    def test_unknown_or_ambiguous_tree_entries_fail(self):
        with self.assertRaises(ValueError):
            BUNDLE.parse_tree("unexpected format")
        with self.assertRaises(ValueError):
            BUNDLE.select_packages({"packages": [{"name": "a", "version": "1"}] * 2},
                                   {("a", "1")}, {("a", "1")})
        with self.assertRaises(ValueError):
            BUNDLE.select_packages({"packages": []}, {("a", "1")}, set())

    def test_lockfile_checksums_are_keyed_by_full_source(self):
        text = '''version = 4
[[package]]
name = "a"
version = "1.0.0"
source = "registry+first"
checksum = "abc"
[[package]]
name = "a"
version = "1.0.0"
source = "registry+second"
checksum = "def"
'''
        checksums = BUNDLE.lock_checksums(text)
        self.assertEqual(checksums[("a", "1.0.0", "registry+first")], "abc")
        self.assertEqual(checksums[("a", "1.0.0", "registry+second")], "def")
        with self.assertRaises(ValueError):
            BUNDLE.lock_checksums(text + text.split("[[package]]")[1].join(["[[package]]", ""]))

    def test_recovered_license_requires_package_revision_and_exact_bytes(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / ".cargo_vcs_info.json").write_text(json.dumps({"git": {"sha1": "revision"}}))
            (root / "Cargo.toml.orig").write_bytes(b"manifest\n")
            data = b"upstream license\n"
            (root / "LICENSE").write_bytes(data)
            entry = {"name": "fixture", "package_sha256": "package", "upstream_revision": "revision",
                     "upstream_manifest_blob_sha1": BUNDLE.git_blob(b"manifest\n"),
                     "files": [{"path": "LICENSE", "sha256": hashlib.sha256(data).hexdigest(),
                                "git_blob_sha1": hashlib.sha1(b"blob " + str(len(data)).encode() + b"\0" + data).hexdigest()}]}
            self.assertEqual(BUNDLE.validate_recovered(entry, "package", root, root), [root / "LICENSE"])
            with self.assertRaises(ValueError):
                BUNDLE.validate_recovered(entry, "changed", root, root)
            (root / ".cargo_vcs_info.json").write_text(json.dumps({"git": {"sha1": "changed"}}))
            with self.assertRaises(ValueError):
                BUNDLE.validate_recovered(entry, "package", root, root)
            (root / ".cargo_vcs_info.json").write_text(json.dumps({"git": {"sha1": "revision"}}))
            (root / "LICENSE").write_text("altered")
            with self.assertRaises(ValueError):
                BUNDLE.validate_recovered(entry, "package", root, root)

    def test_source_bound_jpeg_review_does_not_cover_changes(self):
        package = {"name": "libjpeg-turbo-rs", "version": "0.1.0", "license": "IJG AND Zlib"}
        source = "f5ba795a3d447b0adcaeef7e239cf45fb5495665529c5f11c9d7de599c0afca2"
        self.assertIsNotNone(BUNDLE.jpeg_source_review(package, source))
        self.assertIsNone(BUNDLE.jpeg_source_review(package, "changed"))
        self.assertIsNone(BUNDLE.jpeg_source_review({**package, "license": "IJG"}, source))

    def test_verify_rejects_a_tampered_notice_before_rebuilding(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "cargo-tree-features.txt").write_text("tree")
            (root / "notice").write_text("changed")
            report = {"cargo_tree_features_sha256": BUNDLE.digest(root / "cargo-tree-features.txt"),
                      "packages": [{"notices": [{"path": "notice", "sha256": "original"}]}]}
            (root / "inventory.json").write_text(json.dumps(report))
            with self.assertRaisesRegex(ValueError, "altered bundled notice"):
                BUNDLE.verify("target", "features", root)

    def test_strict_verify_propagates_rebuilt_review_items(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "cargo-tree-features.txt").write_text("tree")
            (root / "notice").write_text("notice")
            report = {"cargo_tree_features_sha256": BUNDLE.digest(root / "cargo-tree-features.txt"),
                      "packages": [{"metadata_status": "REVIEW", "notice_review": "MISSING",
                                    "notices": [{"path": "notice", "sha256": BUNDLE.digest(root / "notice")}]}],
                      "git_status": []}
            (root / "inventory.json").write_text(json.dumps(report))

            def rebuild(_target, _features, candidate):
                candidate.mkdir()
                (candidate / "inventory.json").write_text(json.dumps(report))
                return 1

            args = ["bundle", "--target", "target", "--features", "cli", "--output", str(root), "--verify"]
            with mock.patch.object(BUNDLE, "build", side_effect=rebuild), mock.patch("sys.argv", args):
                self.assertEqual(BUNDLE.main(), 0)
            with mock.patch.object(BUNDLE, "build", side_effect=rebuild), mock.patch("sys.argv", args + ["--strict"]):
                self.assertEqual(BUNDLE.main(), 1)

    def test_bundle_rejects_external_file_and_directory_symlinks(self):
        with tempfile.TemporaryDirectory() as temp:
            outer = Path(temp)
            root = outer / "bundle"
            root.mkdir()
            outside = outer / "external"
            outside.mkdir()
            (outside / "LICENSE").write_text("same bytes")
            (root / "link").symlink_to(outside / "LICENSE")
            (root / "dir-link").symlink_to(outside, target_is_directory=True)
            for relative in ("link", "dir-link/LICENSE", "../external/LICENSE"):
                with self.subTest(relative=relative), self.assertRaises(ValueError):
                    BUNDLE.bundle_file(root, relative)
            (root / "actual").write_text("same bytes")
            self.assertEqual(BUNDLE.bundle_file(root, "actual"), root / "actual")

    def test_notice_collection_includes_nested_patents(self):
        with tempfile.TemporaryDirectory() as temp:
            root = Path(temp)
            (root / "LICENSE").write_text("license")
            (root / "third_party").mkdir()
            (root / "third_party" / "PATENTS").write_text("patent")
            (root / "third_party" / "code.rs").write_text("source")
            self.assertEqual([p.name for p in BUNDLE.notices(root, True)], ["LICENSE", "PATENTS"])
            before = BUNDLE.digest(root / "LICENSE")
            (root / "LICENSE").write_text("changed")
            self.assertNotEqual(before, BUNDLE.digest(root / "LICENSE"))


if __name__ == "__main__":
    unittest.main()
