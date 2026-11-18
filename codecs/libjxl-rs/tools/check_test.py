#!/usr/bin/env python3
"""Unit tests of the test tiers and partitions in tools/check.py.

The tiers must fail closed (ADR-0027, amendment 2026-11-17): every test runs
in at least one tier on each platform and oracle, a name that DEEP_TESTS or
HEAVY_TESTS gives must exist, and the hosted records of a tier must cover it.
`python3 tools/check.py --stage static` runs these tests.
"""
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))
import check  # noqa: E402

# A synthetic suite: every named test plus ordinary ones, some of which sort
# between the named ones.
NAMES = sorted({*check.HEAVY_TESTS, *check.DEEP_TESTS, "a::first", "m::middle", "z::last", "decoder::one",
                "encoder_public::zz"})


class Tiers(unittest.TestCase):
    def test_nightly_and_deep_are_the_whole_suite(self):
        nightly = check.tier_names(NAMES, "nightly")
        deep = check.tier_names(NAMES, "deep")
        self.assertEqual(sorted(nightly + deep), NAMES)
        self.assertFalse(set(nightly) & set(deep))
        self.assertEqual(sorted(deep), sorted(check.DEEP_TESTS))
        self.assertEqual(check.tier_names(NAMES, "all"), NAMES)

    def test_deep_holds_the_decided_parts(self):
        # D2: the 2048x1536 parts; the 4000x3000 lossy parts of the streaming
        # and output-mode-2 gates; D3: the photographs.
        deep = set(check.DEEP_TESTS)
        self.assertEqual(len(deep), len(check.DEEP_TESTS))
        self.assertEqual(sum("2048x1536" in name for name in deep), 8)
        self.assertEqual(sum("4000x3000" in name for name in deep), 12)
        self.assertIn("enc_public_photographs_match_the_original_target", deep)
        self.assertFalse(any("lossless" in name or name.startswith("enc_public_streaming") for name in deep))

    def test_named_tests_must_exist(self):
        check.check_named_tests(NAMES)
        for missing in (check.DEEP_TESTS[0], check.HEAVY_TESTS[-1]):
            with self.assertRaises(SystemExit):
                check.check_named_tests([name for name in NAMES if name != missing])

    def test_unknown_tier_fails(self):
        with self.assertRaises(SystemExit):
            check.tier_names(NAMES, "weekly")

    def test_partitions_cover_each_tier_once(self):
        for tier in check.TIERS:
            chosen = check.tier_names(NAMES, tier)
            for count in range(1, 13):
                parts = [check.partition_names(chosen, index, count) for index in range(1, count + 1)]
                ran = [name for part in parts for name in part]
                self.assertEqual(sorted(ran), sorted(chosen), (tier, count))
                self.assertEqual(len(ran), len(set(ran)), (tier, count))

    def test_heavy_tests_are_dealt_out_first(self):
        chosen = check.tier_names(NAMES, "nightly")
        heavy = [name for name in check.HEAVY_TESTS if name in chosen]
        count = 4
        for index in range(1, count + 1):
            part = check.partition_names(chosen, index, count)
            self.assertTrue(set(part) & set(heavy[:count]), index)

    def test_result_lines(self):
        line = "test result: ok. 17 passed; 0 failed; 2 ignored; 0 measured; 583 filtered out; finished in 1.0s"
        self.assertEqual(check.RESULT_LINE.search(line).groups(), ("17", "2"))
        self.assertIsNone(check.RESULT_LINE.search("test foo ... ok"))


class HostedPlan(unittest.TestCase):
    def test_every_test_runs_on_every_platform_and_oracle(self):
        # Over the nightly tier (with its macOS jobs) and the deep tier, each
        # runner and oracle runs the whole suite.
        covered = {}
        for tier in ("nightly", "deep"):
            for job in check.ci_plan(tier, macos=True)["parity"]:
                covered.setdefault((job["os"], job["oracle"]), set()).add(job["tier"])
        for runner in ("ubuntu-latest", "macos-latest"):
            for oracle in ("ordinary", "sanitized"):
                tiers = covered.get((runner, oracle), set())
                names = set()
                for tier in tiers:
                    names |= set(check.tier_names(NAMES, tier))
                self.assertEqual(names, set(NAMES), (runner, oracle, tiers))

    def test_macos_sanitized_runs_in_the_deep_tier_only(self):
        # D4: the macOS sanitized matrix is the deep tier's.
        nightly = check.ci_plan("nightly", macos=True)["parity"]
        self.assertFalse([job for job in nightly if job["os"] == "macos-latest" and job["oracle"] == "sanitized"])
        deep = check.ci_plan("deep", macos=False)["parity"]
        self.assertTrue([job for job in deep if job["os"] == "macos-latest" and job["tier"] == "all"])

    def test_nightly_without_macos_runs_linux_only(self):
        plan = check.ci_plan("nightly", macos=False)
        self.assertTrue(plan["parity"])
        self.assertFalse([job for job in plan["parity"] + plan["audit"] if job["os"] != "ubuntu-latest"])

    def test_every_partition_is_planned(self):
        for tier in ("nightly", "deep"):
            jobs = check.ci_plan(tier, macos=True)["parity"]
            groups = {}
            for job in jobs:
                groups.setdefault(job["group"], []).append((job["shard"], job["shards"]))
            for group, shards in groups.items():
                count = shards[0][1]
                self.assertEqual(sorted(shards), [(index, count) for index in range(1, count + 1)], group)

    def test_every_audit_group_runs(self):
        # Linux runs every audit group nightly; macOS every group but the
        # Linux-only diagnostics, nightly or deep.
        nightly = check.ci_plan("nightly", macos=True)["audit"]
        deep = check.ci_plan("deep", macos=True)["audit"]
        self.assertEqual({job["group"] for job in nightly if job["os"] == "ubuntu-latest"}, set(check.AUDIT_GROUPS))
        macos = {job["group"] for job in nightly + deep if job["os"] == "macos-latest"}
        self.assertEqual(macos, set(check.AUDIT_GROUPS) - {"diagnostics"})

    def test_each_replaying_audit_has_both_captures(self):
        for tier in ("nightly", "deep"):
            for macos in (False, True):
                plan = check.ci_plan(tier, macos)
                for group in ("decoder", "public-decoder"):
                    self.assertTrue([job for job in plan["audit"] if job["group"] == group], (tier, macos, group))
                audits = sorted({job["os"] for job in plan["audit"] if job["group"] in ("decoder", "public-decoder")})
                captures = sorted((job["os"], job["oracle"]) for job in plan["capture"])
                self.assertEqual(captures, sorted((runner, oracle) for runner in audits
                                                  for oracle in ("ordinary", "sanitized")))

    def test_unknown_hosted_tier_fails(self):
        with self.assertRaises(SystemExit):
            check.ci_plan("all", macos=False)


class Records(unittest.TestCase):
    def write(self, directory, tier, macos, drop=None, extra=None):
        for job in check.ci_plan(tier, macos)["parity"]:
            if drop == (job["group"], job["shard"]):
                continue
            chosen = check.tier_names(NAMES, job["tier"])
            selected = check.partition_names(chosen, job["shard"], job["shards"])
            if extra == (job["group"], job["shard"]):
                selected = selected[1:]
            check.write_record(Path(directory) / f"{job['group']}-{job['shard']}.json", job["group"], job["tier"],
                               (job["shard"], job["shards"]), chosen, selected, len(selected), 0, len(selected))

    def test_complete_records_pass(self):
        for tier in ("nightly", "deep"):
            with tempfile.TemporaryDirectory() as directory:
                self.write(directory, tier, True)
                check.verify_records(directory, tier, True)

    def test_a_missing_partition_fails(self):
        with tempfile.TemporaryDirectory() as directory:
            self.write(directory, "nightly", False, drop=("ubuntu-latest-sanitized", 2))
            with self.assertRaises(SystemExit):
                check.verify_records(directory, "nightly", False)

    def test_a_filtered_test_fails(self):
        with tempfile.TemporaryDirectory() as directory:
            self.write(directory, "deep", False, extra=("ubuntu-latest-ordinary", 1))
            with self.assertRaises(SystemExit):
                check.verify_records(directory, "deep", False)

    def test_an_unaccounted_test_fails(self):
        with tempfile.TemporaryDirectory() as directory:
            self.write(directory, "deep", False)
            path = Path(directory) / "macos-latest-sanitized-1.json"
            record = json.loads(path.read_text())
            record["passed"] -= 1
            path.write_text(json.dumps(record))
            with self.assertRaises(SystemExit):
                check.verify_records(directory, "deep", False)

    def test_records_outside_the_plan_fail(self):
        with tempfile.TemporaryDirectory() as directory:
            self.write(directory, "deep", True)
            # The nightly plan without macOS has no macOS groups.
            with self.assertRaises(SystemExit):
                check.verify_records(directory, "nightly", False)


if __name__ == "__main__":
    unittest.main()
