"""Contract tests for the existing comparator; no encoders or timing runs."""
import importlib.util
import json
from pathlib import Path
import shlex
import tempfile
import unittest
from unittest.mock import patch
import os

spec = importlib.util.spec_from_file_location("compare_sharp", Path(__file__).with_name("compare-sharp.py"))
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)


class ComparisonContract(unittest.TestCase):
    def test_hyperfine_p95_uses_nearest_rank_and_preserves_arguments(self):
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / "raw.json"
            command = ["encoder", "a file's name.jpg"]

            def fake_run(args, env=None):
                self.assertEqual(shlex.split(args[-1]), command)
                output.write_text(json.dumps({"results": [{"times": list(range(1, 21)), "user": 1, "system": 2}]}))

            with patch.object(bench, "run", side_effect=fake_run):
                result = bench.hyperfine(command, {}, "20", output)
            self.assertEqual(result["median_s"], 10.5)
            self.assertEqual(result["p95_s"], 19)

    def test_timing_rejects_incomplete_and_stale_matching(self):
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            provenance = out / "match-provenance.json"
            provenance.write_text('{"status":"incomplete"}')
            with self.assertRaisesRegex(ValueError, "incomplete"):
                bench.timing(out)
            provenance.write_text(json.dumps({"status": "complete", "fingerprint": {"old": True}}))
            with patch.object(bench, "match_fingerprint", return_value={"new": True}):
                with self.assertRaisesRegex(ValueError, "no longer matches"):
                    bench.timing(out)

    def test_matching_rejects_missing_and_unbracketed_lanes(self):
        image = ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6)
        configs = [("ferrixel", "ferrixel-all-444", 0, "444")]
        with patch.object(bench, "IMAGES", [image]), patch.object(bench, "CONFIGS", configs):
            with self.assertRaisesRegex(ValueError, "missing quality match"):
                bench.validate_matched({})
            key = ("dog", "A", "ferrixel-all-444")
            with self.assertRaisesRegex(ValueError, "not bracketed"):
                bench.validate_matched({key: {"status": "unreachable"}})
            row = {"status": "ok", "target_score": "70", "score_below": "69", "score_above": "71", "score_nearest": "71"}
            bench.validate_matched({key: row})
            for field in ("target_score", "score_below", "score_above", "score_nearest"):
                with self.assertRaisesRegex(ValueError, "non-finite"):
                    bench.validate_matched({key: {**row, field: "nan"}})

    def test_final_fingerprint_rejects_changed_inputs(self):
        with tempfile.TemporaryDirectory() as tmp:
            out = Path(tmp)
            (out / "matched.tsv").write_text("match")
            matching = {"status": "complete", "fingerprint": {"before": 1},
                        "matched_sha256": bench.sha256(out / "matched.tsv")}
            with patch.object(bench, "match_fingerprint", return_value={"before": 1}):
                bench.require_current_match(out, matching)
            with patch.object(bench, "match_fingerprint", return_value={"after": 2}):
                with self.assertRaisesRegex(ValueError, "no longer matches"):
                    bench.require_current_match(out, matching)

    def test_output_must_be_repeatable_and_near_shared_target(self):
        row = {"target_score": "70", "score_nearest": "70.4"}
        self.assertAlmostEqual(bench.validate_output_score(row, 70.4), 0.4)
        with self.assertRaisesRegex(ValueError, "matching pass"):
            bench.validate_output_score(row, 70.6)
        with self.assertRaisesRegex(ValueError, "more than 0.5"):
            bench.validate_output_score({"target_score": "70", "score_nearest": "71"}, 71.0)
        with self.assertRaisesRegex(ValueError, "non-finite"):
            bench.validate_output_score(row, float("nan"))

    def test_incomplete_timing_never_marks_provenance_complete(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            provenance_path = root / "time-provenance.json"
            provenance_path.write_text('{"status":"incomplete"}')
            data = root / "time.csv"
            data.write_text("header\nrow\n")
            with self.assertRaisesRegex(ValueError, "every expected lane"):
                bench.write_timing_provenance(provenance_path, {}, data, 2, 1)
            self.assertEqual(json.loads(provenance_path.read_text())["status"], "incomplete")
            bench.write_timing_provenance(provenance_path, {}, data, 1, 1)
            result = json.loads(provenance_path.read_text())
            self.assertEqual(result["status"], "complete")
            self.assertEqual(result["csv_sha256"], bench.sha256(data))

    def test_nonfinite_scores_and_timing_samples_fail(self):
        with patch.object(bench, "run", return_value="out.avif\t42\tnan\n"):
            with self.assertRaisesRegex(ValueError, "non-finite"):
                bench.score("ref", ["out.avif"])
        with tempfile.TemporaryDirectory() as tmp:
            output = Path(tmp) / "raw.json"
            def fake_run(args, env=None):
                output.write_text(json.dumps({"results": [{"times": [float("nan")], "user": 1, "system": 2}]}))
            with patch.object(bench, "run", side_effect=fake_run):
                with self.assertRaisesRegex(ValueError, "wall-time"):
                    bench.hyperfine(["encoder"], {}, "1", output)

    def test_large_crop_matches_both_commands_and_keeps_full_source(self):
        image = ("beach-24", "beach.jpg", "--use-rendered-icc", "ignore", 1600, 1067, 1)
        ferrixel = bench.config_cmd("ferrixel", 0, "444", image, Path("corpus"), 80, "out.avif")
        sharp = bench.config_cmd("sharp", 0, "444", image, Path("corpus"), 50, "out.avif")
        self.assertIn("--crop=1752,1168,6000,4000", ferrixel)
        self.assertIn("crop=1752,1168,6000,4000", sharp)
        self.assertIn(Path("corpus/beach.jpg"), ferrixel)
        self.assertIn("--budget-mb=2048", ferrixel)

    def test_encoder_speed_is_explicit_and_sweep_overrides_it_once(self):
        image = ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6)
        cmd = bench.sweep_cmd(image, Path("corpus"), "ferrixel", 8, 80, "out.avif")
        self.assertEqual([str(x) for x in cmd if str(x).startswith("--speed=")], ["--speed=8"])

    def test_encoder_lanes_preserve_crop_geometry_and_threads(self):
        image = ("beach-24", "beach.jpg", "--use-rendered-icc", "ignore", 1600, 1067, 1)
        for threads in (0, 1):
            cmd = bench.encoder_cmd(image, Path("corpus"), "encoder", 73, 7, threads, "out.avif")
            self.assertIn(f"--encoder-threads={threads}", cmd)
            self.assertIn(f"--transform-threads={threads}", cmd)
            self.assertIn("--crop=1752,1168,6000,4000", cmd)
            self.assertIn(Path("corpus/beach.jpg"), cmd)
            self.assertIn("--budget-mb=2048", cmd)
            self.assertIn("--chroma=444", cmd)

    def test_encoder_quality_levels_are_bounded_and_unique(self):
        with patch.dict(os.environ, {"COMPARE_ENCODER_LEVELS": "60,80,90"}):
            self.assertEqual(bench.encoder_levels(), [60, 80, 90])
        for levels in ("", "60,60", "0,80", "101", "nan"):
            with patch.dict(os.environ, {"COMPARE_ENCODER_LEVELS": levels}):
                with self.assertRaises(ValueError):
                    bench.encoder_levels()

    def test_encoder_builds_every_binary_from_one_unchanged_receipt(self):
        commands = []
        def fake_run(args, env=None, capture=True):
            commands.append(args)
            name, path = [("ferrixel", bench.FERRIXEL), ("ferrixel-libavif", bench.LIBAVIF_ENCODER),
                          ("ferrixel-quality-match", bench.QM)][len(commands) - 1]
            return json.dumps({"reason": "compiler-artifact", "target": {"name": name}, "executable": str(path)})
        with patch.object(bench, "encoder_source_receipt", return_value={"source": "stable"}), patch.object(bench, "run", side_effect=fake_run):
            self.assertEqual(bench.encoder_build(), {"source": "stable"})
        self.assertEqual(len(commands), 3)
        self.assertTrue(all("--locked" in command for command in commands))
        self.assertIn("ferrixel", commands[0])
        self.assertIn("ferrixel-libavif", commands[1])
        self.assertIn(bench.QM_DIR / "Cargo.toml", commands[2])
        self.assertTrue(all("--message-format=json-render-diagnostics" in command for command in commands))
        commands.clear()
        with patch.object(bench, "encoder_source_receipt", side_effect=[{"source": "before"}, {"source": "after"}]), patch.object(bench, "run", side_effect=fake_run):
            with self.assertRaisesRegex(ValueError, "changed while building"):
                bench.encoder_build()
        wrong = json.dumps({"reason": "compiler-artifact", "target": {"name": "ferrixel"},
                            "executable": "/tmp/stale/other/ferrixel"})
        with patch.object(bench, "encoder_source_receipt", return_value={}), patch.object(bench, "run", return_value=wrong):
            with self.assertRaisesRegex(ValueError, "check target configuration"):
                bench.encoder_build()

    def test_encoder_comparison_rejects_a_different_checkout(self):
        with tempfile.TemporaryDirectory() as tmp, patch.object(bench, "REPO", Path(tmp)):
            with self.assertRaisesRegex(ValueError, "harness checkout"):
                bench.encoder_source_receipt()

    def test_relative_target_is_resolved_against_the_comparison_checkout(self):
        with tempfile.TemporaryDirectory() as tmp:
            repo = Path(tmp) / "repo"
            self.assertEqual(bench.resolve_target("other-target", repo), (repo / "other-target").resolve())
            self.assertEqual(bench.resolve_target(Path(tmp) / "shared-target", repo), (Path(tmp) / "shared-target").resolve())

    def test_encoder_match_rejects_stale_missing_and_unbracketed_evidence(self):
        image = ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            work = root / "encoders"
            work.mkdir()
            (work / "anchor.avif").write_bytes(b"anchor")
            records = [{"image": "dog", "anchor_quality": 80, "threads": threads, "speed": speed,
                        "status": "ok", "target_score": "70", "score_below": "69",
                        "score_above": "71", "score_nearest": "70"}
                       for threads in (0, 1) for speed in (6, 7)]
            match_file = work / "matched.json"
            match_file.write_text(json.dumps(records))
            matching = {"status": "complete", "fingerprint": {"x": 1}, "matched_sha256": bench.sha256(match_file),
                        "outputs_sha256": {"anchor.avif": bench.sha256(work / "anchor.avif")}}
            with patch.object(bench, "IMAGES", [image]), patch.object(bench, "encoder_fingerprint", return_value={"x": 1}), patch.dict(os.environ, {"COMPARE_ENCODER_LEVELS": "80"}):
                self.assertEqual(len(bench.validate_encoder_match(root, matching)), 4)
                (work / "anchor.avif").write_bytes(b"changed")
                with self.assertRaisesRegex(ValueError, "missing or changed"):
                    bench.validate_encoder_match(root, matching)
                (work / "anchor.avif").write_bytes(b"anchor")
                records[0]["status"] = "unreachable"
                match_file.write_text(json.dumps(records))
                matching["matched_sha256"] = bench.sha256(match_file)
                with self.assertRaisesRegex(ValueError, "not bracketed"):
                    bench.validate_encoder_match(root, matching)
                records[0]["status"] = "ok"
                records.pop()
                match_file.write_text(json.dumps(records))
                matching["matched_sha256"] = bench.sha256(match_file)
                with self.assertRaisesRegex(ValueError, "matrix"):
                    bench.validate_encoder_match(root, matching)

    def test_encoder_phases_keep_per_thread_matches_and_each_sample(self):
        image = ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            rav, aom = root / "rav", root / "aom"
            rav.write_bytes(b"rav")
            aom.write_bytes(b"aom")
            def fake_run(args, env=None, capture=True):
                if len(args) > 1 and str(args[1]) == "bisect":
                    path = Path(args[7])
                    path.mkdir(parents=True, exist_ok=True)
                    (path / "q70.avif").write_bytes(b"matched")
                    return "ok\t70.000\t69\t69.000\t90\t70\t71.000\t100\t95\t70\t70.000\t4\n"
                if args[0] == rav:
                    Path(args[2]).write_bytes(b"anchor")
                    return ""
                return "version"
            def fake_sample(cmd):
                Path(cmd[2]).write_bytes(b"sample")
                return 0.2, 123, {"total_us": "100"}
            def fake_score(ref, files):
                return [(Path(path).name, Path(path).stat().st_size, 70.0) for path in files]
            env = {"COMPARE_ENCODER_LEVELS": "80", "COMPARE_RUNS": "2", "COMPARE_ENCODER_RUN_ID": "test"}
            empty_sha = "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
            with (patch.object(bench, "IMAGES", [image]),
                  patch.object(bench, "FERRIXEL", rav),
                  patch.object(bench, "LIBAVIF_ENCODER", aom),
                  patch.object(bench, "encoder_build", return_value={"tracked_diff_sha256": empty_sha}),
                  patch.object(bench, "encoder_fingerprint", return_value={"tracked_diff_sha256": empty_sha}),
                  patch.object(bench, "tracked_source_diff", return_value=b""),
                  patch.object(bench, "run", side_effect=fake_run),
                  patch.object(bench, "score", side_effect=fake_score),
                  patch.object(bench, "encoder_sample", side_effect=fake_sample),
                  patch.object(bench, "encoder_wait_for_idle"),
                  patch.object(bench, "load1", return_value=0.1), patch.dict(os.environ, env)):
                bench.encoder_match(root)
                bench.encoder_timing(root)
                work = root / "encoders"
                matched = json.loads((work / "matched.json").read_text())
                self.assertEqual({row["threads"] for row in matched}, {0, 1})
                result = work / "time-test"
                provenance = json.loads((result / "provenance.json").read_text())
                samples = json.loads((result / "samples.json").read_text())
                self.assertEqual(provenance["status"], "complete")
                self.assertEqual(provenance["samples_sha256"], bench.sha256(result / "samples.json"))
                self.assertEqual(len(samples), 12)
                self.assertEqual([row["lane"] for row in samples[:3]], ["rav1e", "aom-s6", "aom-s7"])
                self.assertEqual([row["lane"] for row in samples[3:6]], ["aom-s7", "aom-s6", "rav1e"])


if __name__ == "__main__":
    unittest.main()
