#!/usr/bin/env python3
"""Unit tests of scripts/ci/report_failed_jobs.py: python3 -I scripts/ci/test_report_failed_jobs.py"""

import io
from pathlib import Path
import subprocess
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))

import report_failed_jobs as report  # noqa: E402

LOG = """\
2026-11-17T10:47:00.3175290Z running 3 tests
2026-11-17T10:47:00.3175290Z test wp2::a_matches ... ok
2026-11-17T10:47:00.3175290Z test wp2::b_matches ... FAILED
2026-11-17T10:47:00.3175290Z test src/lib.rs - f (line 3) ... FAILED
2026-11-17T10:47:00.3175290Z failures:
2026-11-17T10:47:00.3175290Z     wp2::b_matches
2026-11-17T10:47:00.3175290Z test wp2::b_matches ... FAILED
2026-11-17T10:47:00.3175290Z \x1b[1m\x1b[91merror\x1b[0m: test failed, to rerun pass `--test wp2_header`
2026-11-17T10:47:00.3175290Z FAIL: test_deal (__main__.Deal)
2026-11-17T10:47:00.3175290Z ERROR: test_verify (__main__.Verify)
2026-11-17T10:47:00.3175290Z test:wp13_tools: 4 tests selected, 3 reported
2026-11-17T10:47:00.3175290Z test result: FAILED. 1 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out
"""


def job(number, name, conclusion):
    return {"id": number, "name": name, "conclusion": conclusion,
            "html_url": f"https://github.com/o/r/actions/runs/1/job/{number}"}


class FakeApi:
    def __init__(self, jobs, issues=(), logs=None):
        self._jobs, self._issues, self._logs = jobs, list(issues), logs or {}
        self.created, self.comments = [], []

    def run(self, run_id):
        return {"html_url": f"https://github.com/o/r/actions/runs/{run_id}", "run_attempt": 2,
                "event": "schedule", "head_sha": "abc123", "head_branch": "main"}

    def jobs(self, run_id, attempt):
        self.attempt = attempt
        return self._jobs

    def log(self, job_id):
        if job_id not in self._logs:
            raise subprocess.CalledProcessError(1, ["gh"])
        return self._logs[job_id]

    def open_issues(self):
        return self._issues

    def comment(self, number, text):
        self.comments.append((number, text))

    def create(self, title, text):
        self.created.append((title, text))


class Extraction(unittest.TestCase):
    def test_failing_tests_of_a_log(self):
        self.assertEqual(report.failing_tests(LOG), [
            "wp2::b_matches", "src/lib.rs - f (line 3)", "--test wp2_header", "test_deal (__main__.Deal)",
            "test_verify (__main__.Verify)", "test:wp13_tools: 4 tests selected, 3 reported"])

    def test_the_lines_before_the_error(self):
        log = ("2026-11-17T10:00:00.0000000Z ##[group]Run python3 tools/partition.py verify\n"
               "2026-11-17T10:00:00.0000000Z python3 tools/partition.py verify\n"
               "2026-11-17T10:00:00.0000000Z ##[endgroup]\n"
               "2026-11-17T10:00:01.0000000Z darwin-arm64-asan-ubsan: partitions [3] of 3\n"
               "2026-11-17T10:00:01.0000000Z ##[error]Process completed with exit code 1.\n"
               "2026-11-17T10:00:02.0000000Z later\n")
        self.assertEqual(report.error_context(log), ["python3 tools/partition.py verify",
                                                     "darwin-arm64-asan-ubsan: partitions [3] of 3"])
        self.assertEqual(report.error_context("no error here\n"), [])
        api = FakeApi([job(1, "x / Oracle partitions complete", "failure")], logs={1: log})
        actions = report.report(api, 5, None, "Nightly", dry_run=False, out=io.StringIO())
        self.assertEqual(actions[0][3], [])
        self.assertIn("darwin-arm64-asan-ubsan: partitions [3] of 3\n```", api.created[0][1])

    def test_a_green_log_names_nothing(self):
        self.assertEqual(report.failing_tests("test a ... ok\ntest result: ok. 1 passed; 0 failed;\n"), [])


class Selection(unittest.TestCase):
    def test_aggregators_only_when_nothing_else_failed(self):
        jobs = [job(1, "families (webp) / webp / WEBP: Oracle (macos-latest, asan-ubsan, 2/4)", "failure"),
                job(2, "families (webp) / webp / WEBP: Checks passed", "failure"),
                job(3, "families (webp) / gate", "failure"), job(4, "gate", "failure"),
                job(5, "families (jxl) / jxl / JXL: Tests (ubuntu-latest, ordinary oracle, 3/4)", "cancelled"),
                job(6, "kernel-bench (ubuntu-24.04)", "success"), job(7, "report", None),
                job(8, "families (avif) / avif / dav1d / AVIF dav1d: parity (macos-15, arm64)", "timed_out")]
        self.assertEqual([entry["id"] for entry in report.failed_jobs(jobs)], [1, 5, 8])
        self.assertEqual([entry["id"] for entry in report.failed_jobs(jobs[1:4] + jobs[5:7])], [2, 3, 4])
        self.assertEqual(report.failed_jobs(jobs[5:7]), [])

    def test_ignored_jobs(self):
        jobs = [job(1, "clippy-canary", "failure"), job(2, "deep / clippy-canary", "failure"),
                job(3, "gate", "failure")]
        ignore = report.re.compile(r"(^|/ )clippy-canary$")
        self.assertEqual([entry["id"] for entry in report.failed_jobs(jobs, ignore)], [3])

    def test_partition_completeness_is_not_an_aggregator(self):
        jobs = [job(1, "families (webp) / webp / WEBP: Oracle partitions complete", "failure"),
                job(2, "families (webp) / gate", "failure")]
        self.assertEqual([entry["id"] for entry in report.failed_jobs(jobs)], [1])


class Reporting(unittest.TestCase):
    def jobs(self):
        return [job(11, "families (jpeg) / jpeg / JPEG: Oracle (macos-latest, 2/3)", "failure"),
                job(12, "fuzz-campaign / jpeg-port decode (macos-15, ASan)", "cancelled"),
                job(13, "gate", "failure")]

    def test_opens_new_and_comments_on_existing_issues_by_exact_title(self):
        issues = [{"number": 7, "title": "Nightly: fuzz-campaign / jpeg-port decode (macos-15, ASan) failed"},
                  {"number": 8, "title": "Nightly: families (jpeg) / jpeg / JPEG: Oracle (macos-latest, 2/3)"},
                  {"number": 9, "title": "Deep: families (jpeg) / jpeg / JPEG: Oracle (macos-latest, 2/3) failed"}]
        api = FakeApi(self.jobs(), issues, {11: LOG})
        actions = report.report(api, 5, None, "Nightly", dry_run=False, out=io.StringIO())
        self.assertEqual(api.attempt, 2)
        self.assertEqual([(kind, number) for kind, _, number, _ in actions], [("create", None), ("comment", 7)])
        (created_title, created_body), = api.created
        self.assertEqual(created_title, "Nightly: families (jpeg) / jpeg / JPEG: Oracle (macos-latest, 2/3) failed")
        self.assertIn("- `wp2::b_matches`", created_body)
        self.assertIn("https://github.com/o/r/actions/runs/1/job/11", created_body)
        self.assertIn("https://github.com/o/r/actions/runs/5 (attempt 2, `schedule`)", created_body)
        (number, comment), = api.comments
        self.assertEqual(number, 7)
        self.assertIn("could not be read (exit 1)", comment)

    def test_dry_run_writes_nothing(self):
        api = FakeApi(self.jobs(), [], {11: LOG})
        out = io.StringIO()
        actions = report.report(api, 5, 1, "Deep", dry_run=True, out=out)
        self.assertEqual(api.attempt, 1)
        self.assertEqual((api.created, api.comments), ([], []))
        self.assertEqual(len(actions), 2)
        self.assertIn("Would open: Deep: families (jpeg) / jpeg / JPEG: Oracle (macos-latest, 2/3) failed", out.getvalue())

    def test_titles_without_the_calling_job(self):
        api = FakeApi([job(1, "deep / families (jxl) / jxl / JXL: Tests (macos-latest, sanitized oracle, 1/6)",
                           "failure")])
        actions = report.report(api, 5, None, "Deep", dry_run=True, out=io.StringIO(), strip="deep / ")
        self.assertEqual(actions[0][1], "Deep: families (jxl) / jxl / JXL: Tests (macos-latest, sanitized oracle, 1/6) failed")

    def test_a_green_run_reports_nothing(self):
        api = FakeApi([job(1, "gate", "success")])
        out = io.StringIO()
        self.assertEqual(report.report(api, 5, None, "Nightly", dry_run=False, out=out), [])
        self.assertIn("No failed job", out.getvalue())

    def test_long_test_lists_are_cut(self):
        tests = [f"t{number}" for number in range(report.LIMIT + 5)]
        text = report.body("Nightly", job(1, "x", "failure"), FakeApi([]).run(3) | {"run_attempt": 1}, tests, None)
        self.assertIn(f"({report.LIMIT + 5}, first {report.LIMIT})", text)
        self.assertNotIn(f"`t{report.LIMIT}`", text)


if __name__ == "__main__":
    unittest.main()
