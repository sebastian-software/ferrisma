"""Contract tests for the existing comparator; no encoders or timing runs."""
import importlib.util
import json
from pathlib import Path
import shlex
import tempfile
import unittest
from unittest.mock import patch
import os
import subprocess
import sys
import textwrap

spec = importlib.util.spec_from_file_location("compare_sharp", Path(__file__).with_name("compare-sharp.py"))
bench = importlib.util.module_from_spec(spec)
spec.loader.exec_module(bench)


class ComparisonContract(unittest.TestCase):
    def run_calibration_workflow(self, root, accept_nearby, invalid_evidence=False):
        """Exercise the real workflow shell with a bounded encoder-command fake."""
        workflow = (bench.REPO / ".github/workflows/m2-encoder-corpus.yml").read_text()

        def step(name):
            block = workflow.split(f"      - name: {name}\n", 1)[1].split("\n      - ", 1)[0]
            return textwrap.dedent(block.split("        run: |\n", 1)[1])

        output = root / "bench-out"
        for folder in ("corpus", "refs"):
            (output / "prepared/dog" / folder).mkdir(parents=True)
        (output / "images.txt").write_text("dog\n")
        (output / "diagnostic.tsv").write_text("dog\t60\trejected\n")
        command = root / "scripts/bench/compare-encoders.sh"
        command.parent.mkdir(parents=True)
        command.write_text('''#!/usr/bin/env bash
printf '%s\\t%s\\n' "$COMPARE_ENCODER_LEVELS" "$2" >> "$OUT/calls.tsv"
if [[ "$2" == check ]]; then
  if [[ "$INVALID_EVIDENCE" == 1 ]]; then exit 1; fi
  if [[ "$ACCEPT_NEARBY" == 1 && "$COMPARE_ENCODER_LEVELS" =~ ^(69|79|89)$ ]]; then exit 0; fi
  exit 3
fi
''')
        command.chmod(0o755)
        env = {**os.environ, "OUT": str(output), "SCALAR": "false",
               "ACCEPT_NEARBY": str(int(accept_nearby)),
               "INVALID_EVIDENCE": str(int(invalid_evidence))}
        for name in ("Calibrate primary anchors and retain every attempted match",
                     "Time every primary cell with complete matching"):
            subprocess.run(["bash", "-e", "-o", "pipefail", "-c", step(name)],
                           cwd=root, env=env, check=True, capture_output=True, text=True)
        final = subprocess.run(["bash", "-e", "-o", "pipefail", "-c",
                                step("Require every primary cell to complete")],
                               cwd=root, env=env, capture_output=True, text=True)
        return output, final

    def test_workflow_calibrates_in_declared_order_and_times_only_first_pass(self):
        with tempfile.TemporaryDirectory() as tmp:
            output, final = self.run_calibration_workflow(Path(tmp), True)
            self.assertEqual(final.returncode, 0, final.stderr)
            calls = (output / "calls.tsv").read_text().splitlines()
            self.assertEqual([line for line in calls if line.endswith("\tcheck")],
                             [f"{q}\tcheck" for q in (70, 69, 80, 79, 90, 89)])
            self.assertEqual([line for line in calls if line.endswith("\ttime")],
                             [f"{q}\ttime" for q in (69, 79, 89)])
            attempts = (output / "attempts.tsv").read_text()
            self.assertIn("dog\t70\t70\trejected\n", attempts)
            self.assertIn("dog\t70\t69\tcomplete\n", attempts)
            self.assertEqual((output / "cells/dog-a70-q69/refs").resolve(),
                             (output / "prepared/dog/refs").resolve())

    def test_workflow_exhaustion_keeps_gate_red_without_timing_failed_matches(self):
        with tempfile.TemporaryDirectory() as tmp:
            output, final = self.run_calibration_workflow(Path(tmp), False)
            self.assertNotEqual(final.returncode, 0)
            calls = (output / "calls.tsv").read_text().splitlines()
            self.assertFalse(any(line.endswith("\ttime") for line in calls))
            self.assertEqual([line for line in calls if line.endswith("\tcheck")][:5],
                             [f"{q}\tcheck" for q in (70, 69, 71, 68, 72)])
            self.assertEqual(len((output / "attempts.tsv").read_text().splitlines()), 16)

    def test_workflow_does_not_search_past_invalid_matching_evidence(self):
        with tempfile.TemporaryDirectory() as tmp:
            output, final = self.run_calibration_workflow(Path(tmp), True, invalid_evidence=True)
            self.assertNotEqual(final.returncode, 0)
            calls = (output / "calls.tsv").read_text().splitlines()
            self.assertEqual([line for line in calls if line.endswith("\tcheck")],
                             [f"{q}\tcheck" for q in (70, 80, 90)])
            self.assertFalse(any(line.endswith("\ttime") for line in calls))

    def test_encoder_check_reserves_exit_three_for_quality_mismatches(self):
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / "encoders").mkdir()
            (root / "encoders/match-provenance.json").write_text('{}')
            with patch.object(bench, "validate_encoder_match",
                              side_effect=bench.EncoderQualityMismatch("quality gap")):
                with self.assertRaises(SystemExit) as result:
                    bench.encoder_check(root)
                self.assertEqual(result.exception.code, 3)
            with patch.object(bench, "validate_encoder_match", side_effect=ValueError("stale")):
                with self.assertRaisesRegex(ValueError, "stale"):
                    bench.encoder_check(root)

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
        configs = [("ferrisma", "ferrisma-all-444", 0, "444")]
        with patch.object(bench, "IMAGES", [image]), patch.object(bench, "CONFIGS", configs):
            with self.assertRaisesRegex(ValueError, "missing quality match"):
                bench.validate_matched({})
            key = ("dog", "A", "ferrisma-all-444")
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
        ferrisma = bench.config_cmd("ferrisma", 0, "444", image, Path("corpus"), 80, "out.avif")
        sharp = bench.config_cmd("sharp", 0, "444", image, Path("corpus"), 50, "out.avif")
        self.assertIn("--crop=1752,1168,6000,4000", ferrisma)
        self.assertIn("crop=1752,1168,6000,4000", sharp)
        self.assertIn(Path("corpus/beach.jpg"), ferrisma)
        self.assertIn("--budget-mb=2048", ferrisma)

    def test_independent_photo_preparation_checks_source_and_copy(self):
        image = ("deer", "deer.jpg", "--use-rendered-icc", "ignore", 1600, 1067, 1)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            source, corpus, refs = root / "source", root / "corpus", root / "refs"
            for path in (source, corpus, refs):
                path.mkdir()
            (source / "ferrisma-deer.jpg").write_bytes(b"photo")
            checks = []
            commands = []
            def verify(path, image_id, manifest):
                checks.append((path, image_id, manifest))
            def fake_run(args, env=None, capture=True):
                commands.append(args)
                return ""
            with (patch.object(bench, "IMAGE_SET", "deer"), patch.object(bench, "IMAGES", [image]),
                  patch.object(bench, "verify_corpus_file", side_effect=verify),
                  patch.object(bench, "run", side_effect=fake_run),
                  patch.dict(os.environ, {"COMPARE_LARGE_CORPUS": str(source), "BENCH_SHARP_NODE_MODULES": "test"})):
                bench.prepare_large(corpus, refs)
            self.assertEqual((corpus / "deer.jpg").read_bytes(), b"photo")
            self.assertEqual(checks, [(source / "ferrisma-deer.jpg", "deer", "corpus-photos.json"),
                                      (corpus / "deer.jpg", "deer", "corpus-photos.json")])
            self.assertTrue(any(f"{corpus / 'deer.jpg'}#1" in command for command in commands))
            cmd = bench.encoder_cmd(image, corpus, "encoder", 80, 6, 1, "out.avif")
            self.assertIn("--budget-mb=2048", cmd)
            self.assertFalse(any(str(arg).startswith("--crop=") for arg in cmd))

    def test_elcito_selection_uses_explicit_srgb_assumption(self):
        script = ("import runpy; x=runpy.run_path('scripts/bench/compare-sharp.py'); "
                  "print(x['IMAGES'][0][2])")
        env = {**os.environ, "COMPARE_IMAGE_SET": "elcito"}
        output = subprocess.check_output([sys.executable, "-I", "-c", script], env=env,
                                         cwd=bench.REPO, text=True)
        self.assertEqual(output.strip(), "--assume-srgb")

    def test_encoder_speed_is_explicit_and_sweep_overrides_it_once(self):
        image = ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6)
        cmd = bench.sweep_cmd(image, Path("corpus"), "ferrisma", 8, 80, "out.avif")
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
        with patch.dict(os.environ, {}, clear=True):
            self.assertEqual(bench.encoder_levels(), [80])
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
            name, path = [("ferrisma", bench.FERRISMA), ("ferrisma-libavif", bench.LIBAVIF_ENCODER),
                          ("ferrisma-quality-match", bench.QM)][len(commands) - 1]
            return json.dumps({"reason": "compiler-artifact", "target": {"name": name}, "executable": str(path)})
        with patch.object(bench, "encoder_source_receipt", return_value={"source": "stable"}), patch.object(bench, "run", side_effect=fake_run):
            self.assertEqual(bench.encoder_build(), {"source": "stable"})
        self.assertEqual(len(commands), 3)
        self.assertTrue(all("--locked" in command for command in commands))
        self.assertIn("ferrisma", commands[0])
        self.assertIn("ferrisma-libavif", commands[1])
        self.assertIn(bench.QM_DIR / "Cargo.toml", commands[2])
        self.assertTrue(all("--message-format=json-render-diagnostics" in command for command in commands))
        commands.clear()
        with patch.object(bench, "encoder_source_receipt", side_effect=[{"source": "before"}, {"source": "after"}]), patch.object(bench, "run", side_effect=fake_run):
            with self.assertRaisesRegex(ValueError, "changed while building"):
                bench.encoder_build()
        wrong = json.dumps({"reason": "compiler-artifact", "target": {"name": "ferrisma"},
                            "executable": "/tmp/stale/other/ferrisma"})
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
                for nearest in ("69.5", "70.5"):
                    records[0]["score_nearest"] = nearest
                    match_file.write_text(json.dumps(records))
                    matching["matched_sha256"] = bench.sha256(match_file)
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

    def test_integer_quality_gap_fails_before_any_warmup_or_sample(self):
        image = ("dog", "dog.jpg", "--use-rendered-icc", "ignore", 1600, 2133, 6)
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            work = root / "encoders"
            work.mkdir()
            (work / "anchor.avif").write_bytes(b"anchor")
            records = [{"image": "dog", "anchor_quality": 60, "threads": threads, "speed": speed,
                        "status": "ok", "target_score": "70", "score_below": "68.415",
                        "score_above": "71.5", "score_nearest": "70"}
                       for threads in (0, 1) for speed in (6, 7)]
            records[0]["score_nearest"] = "68.415"
            match_file = work / "matched.json"
            match_file.write_text(json.dumps(records))
            matching = {"status": "complete", "fingerprint": {"x": 1},
                        "matched_sha256": bench.sha256(match_file),
                        "outputs_sha256": {"anchor.avif": bench.sha256(work / "anchor.avif")}}
            (work / "match-provenance.json").write_text(json.dumps(matching))
            with (patch.object(bench, "IMAGES", [image]),
                  patch.object(bench, "encoder_fingerprint", return_value={"x": 1}),
                  patch.object(bench, "encoder_sample") as sample,
                  patch.object(bench, "encoder_wait_for_idle") as idle,
                  patch.dict(os.environ, {"COMPARE_ENCODER_LEVELS": "60", "COMPARE_ENCODER_RUN_ID": "gap"})):
                with self.assertRaises(SystemExit) as result:
                    bench.encoder_check(root)
                self.assertEqual(result.exception.code, 3)
                with self.assertRaisesRegex(ValueError, r"dog.*60.*0.*6.*delta=-1\.585"):
                    bench.encoder_timing(root)
                sample.assert_not_called()
                idle.assert_not_called()
                timed = work / "time-gap"
                self.assertEqual(json.loads((timed / "provenance.json").read_text())["status"], "incomplete")
                self.assertFalse((timed / "samples.json").exists())

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
                  patch.object(bench, "FERRISMA", rav),
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
