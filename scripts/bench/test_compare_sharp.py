"""Contract tests for the existing comparator; no encoders or timing runs."""
import importlib.util
import json
from pathlib import Path
import shlex
import tempfile
import unittest
from unittest.mock import patch

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


if __name__ == "__main__":
    unittest.main()
