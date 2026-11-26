"""Exercise the review transitions without GitHub writes or model calls."""

from contextlib import ExitStack
import importlib.util
import json
import os
from pathlib import Path
from tempfile import TemporaryDirectory
import unittest
from unittest.mock import patch


spec = importlib.util.spec_from_file_location("review", Path(__file__).with_name("review.py"))
review = importlib.util.module_from_spec(spec)
spec.loader.exec_module(review)


class ReviewTest(unittest.TestCase):
    def setUp(self):
        contexts = ExitStack()
        self.addCleanup(contexts.close)
        temp = contexts.enter_context(TemporaryDirectory())
        contexts.enter_context(patch.dict(os.environ, {"RUNNER_TEMP": temp}))
        self.pull = review.PullRequest({
            "pull_request": {"number": 17, "base": {"sha": "a" * 40}, "head": {"sha": "b" * 40}},
            "repository": {"default_branch": "main"},
        })
        self.status = contexts.enter_context(patch.object(self.pull, "status"))
        self.current = contexts.enter_context(patch.object(self.pull, "is_current", return_value=True))
        self.github = contexts.enter_context(patch.object(review, "github"))
        self.output = contexts.enter_context(patch.object(review, "output"))

    def write_result(self, stage, findings, *, complete=True):
        (self.pull.temp / f"ferrisma-{stage}.json").write_text(
            json.dumps({"complete": complete, "findings": findings}), encoding="utf-8",
        )

    def test_sol_blocker_requests_changes_without_escalating(self):
        self.write_result("sol", [{"path": "src/lib.rs", "line": 42, "body": "The crop loses alpha."}])
        self.pull.publish("sol")
        payload = self.github.call_args.kwargs["payload"]
        self.assertEqual(payload["event"], "REQUEST_CHANGES")
        self.assertEqual(payload["commit_id"], self.pull.head)
        self.assertEqual(payload["comments"][0]["line"], 42)
        self.status.assert_called_once_with("failure", "Sol requested changes")
        self.output.assert_not_called()

    def test_clean_sol_only_escalates(self):
        self.write_result("sol", [])
        self.pull.publish("sol")
        self.github.assert_not_called()
        self.status.assert_called_once_with("pending", "Sol found no blockers; waiting for Astra")
        self.output.assert_called_once_with("escalate", "true")

    def test_clean_astra_approves_before_setting_success(self):
        self.write_result("astra", [])
        self.github.side_effect = lambda *args, **kwargs: self.status.assert_not_called()
        self.pull.publish("astra")
        payload = self.github.call_args.kwargs["payload"]
        self.assertEqual(payload["event"], "APPROVE")
        self.assertEqual(payload["commit_id"], self.pull.head)
        self.status.assert_called_once_with("success", "Astra approved this commit")

    def test_astra_review_retains_blockers_if_inline_anchor_is_rejected(self):
        self.write_result("astra", [{"path": "src/lib.rs", "line": 42, "body": "The crop loses alpha."}])
        self.github.side_effect = [review.GitHubError("Validation Failed (HTTP 422)"), None]
        self.pull.publish("astra")
        self.assertEqual(self.github.call_count, 2)
        payload = self.github.call_args.kwargs["payload"]
        self.assertNotIn("comments", payload)
        self.assertIn("src/lib.rs:42", payload["body"])
        self.assertEqual(payload["event"], "REQUEST_CHANGES")
        self.status.assert_called_once_with("failure", "Astra requested changes")

    def test_rerun_recovers_success_without_new_review(self):
        with patch.object(self.pull, "reviews", return_value=[(
            "astra", self.pull.base, {"commit_id": self.pull.head, "state": "APPROVED"},
        )]):
            self.pull.prepare()
        self.status.assert_called_once_with("success", "Astra approved this commit")
        self.output.assert_not_called()

    def test_incomplete_review_cannot_approve(self):
        self.write_result("astra", [], complete=False)
        with self.assertRaisesRegex(ValueError, "complete review"):
            self.pull.publish("astra")
        self.github.assert_not_called()
        self.status.assert_not_called()

    def test_stale_result_is_discarded(self):
        self.write_result("astra", [])
        self.current.return_value = False
        self.pull.publish("astra")
        self.github.assert_not_called()
        self.status.assert_not_called()


if __name__ == "__main__":
    unittest.main()
