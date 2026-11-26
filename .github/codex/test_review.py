"""Validate the watcher without GitHub writes or model calls."""

import copy
from datetime import datetime, timedelta, timezone
from io import BytesIO
import json
import os
from pathlib import Path
import subprocess
from tempfile import TemporaryDirectory
import unittest
from unittest.mock import patch

import review
from github_app import GitHubApp, GitHubError, api
from watch import Watcher


REPO = "sebastian-software/ferrisma"
HEAD = "b" * 40
BASE = "a" * 40
FINDING = {"path": "src/lib.rs", "line": 42, "body": "This crop loses the alpha plane."}


class FakeApp:
    login = "ferrisma-review[bot]"

    def __init__(self):
        self.pull = {
            "number": 17, "state": "open", "draft": False,
            "labels": [{"name": "codex-review"}],
            "base": {"sha": BASE, "ref": "main", "repo": {"default_branch": "main"}},
            "head": {"sha": HEAD}, "html_url": "https://github.com/sebastian-software/ferrisma/pull/17",
        }
        self.calls = []
        self.history = []
        self.reject_inline = False

    def request(self, repo, path, *, payload=None, paginate=False, raw=False):
        self.calls.append((repo, path, copy.deepcopy(payload)))
        if path == "pulls?state=open&per_page=100":
            return [copy.deepcopy(self.pull)]
        if path == "pulls/17":
            return "diff --git a/src/lib.rs b/src/lib.rs" if raw else copy.deepcopy(self.pull)
        if path == "pulls/17/reviews?per_page=100":
            return self.history
        if path.startswith("contents/AGENTS.md?ref="):
            return {"content": "UmV2aWV3IGd1aWRhbmNlLg=="}
        if path == "pulls/17/reviews" and self.reject_inline and payload.get("comments"):
            raise GitHubError("Invalid anchor (HTTP 422)")
        return {}

    def events(self):
        return [body["event"] for _, path, body in self.calls if path == "pulls/17/reviews"]

    def statuses(self):
        return [body["state"] for _, path, body in self.calls if path.startswith("statuses/")]


class WatcherTest(unittest.TestCase):
    def setUp(self):
        directory = TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.state = Path(directory.name) / "state.json"
        self.app = FakeApp()
        self.watcher = Watcher(self.app, [REPO], "codex-review", self.state)
        runner = patch("watch.run_model")
        self.run_model = runner.start()
        self.addCleanup(runner.stop)

    def test_sol_requests_changes_then_astra_approves_the_new_commit(self):
        self.run_model.side_effect = [[FINDING], [], []]
        self.assertTrue(self.watcher.scan())
        self.assertEqual(self.app.events(), ["REQUEST_CHANGES"])
        self.assertEqual(self.app.statuses(), ["pending", "failure"])
        self.app.pull["head"]["sha"] = "c" * 40
        self.assertTrue(self.watcher.scan())
        self.assertEqual(self.app.events(), ["REQUEST_CHANGES", "APPROVE"])
        self.assertEqual(self.app.statuses()[-3:], ["pending", "pending", "success"])
        self.assertEqual([call.args[0] for call in self.run_model.call_args_list], ["sol", "sol", "astra"])

    def test_unchanged_head_is_deduplicated_across_restarts(self):
        self.run_model.return_value = []
        self.watcher.scan()
        restarted = Watcher(self.app, [REPO], "codex-review", self.state)
        restarted.scan()
        self.assertEqual(self.run_model.call_count, 2)
        self.assertEqual(self.app.events(), ["APPROVE"])

    def test_existing_review_repairs_status_without_another_model_call(self):
        self.app.history = [{
            "body": f"<!-- ferrisma-codex-review:astra:{BASE} -->\nNo blockers.",
            "user": {"login": self.app.login}, "state": "APPROVED", "commit_id": HEAD,
        }]
        self.watcher.scan()
        self.run_model.assert_not_called()
        self.assertEqual(self.app.statuses(), ["success"])

    def test_astra_rechecks_previous_findings_and_retries_invalid_anchors(self):
        previous = f"<!-- ferrisma-codex-review:astra:{BASE} -->\nOpen finding."
        self.app.history = [{
            "body": previous, "user": {"login": self.app.login},
            "state": "CHANGES_REQUESTED", "commit_id": "c" * 40,
        }]
        self.app.reject_inline = True
        self.run_model.side_effect = [[], [FINDING]]
        self.watcher.scan()
        self.assertEqual(self.run_model.call_args.args[-1], previous)
        self.assertEqual(self.app.events(), ["REQUEST_CHANGES", "REQUEST_CHANGES"])
        self.assertEqual(self.app.statuses()[-1], "failure")

    def test_new_push_during_sol_prevents_publishing_on_the_old_head(self):
        def new_commit(*args):
            self.app.pull["head"]["sha"] = "d" * 40
            return [FINDING]

        self.run_model.side_effect = new_commit
        self.watcher.scan()
        self.assertEqual(self.app.events(), [])
        self.assertFalse(self.state.exists())

    def test_model_error_fails_status_and_backs_off(self):
        self.run_model.side_effect = RuntimeError("quota exhausted")
        self.assertFalse(self.watcher.scan())
        self.assertEqual(self.app.statuses(), ["pending", "error"])
        self.watcher.scan()
        self.run_model.assert_called_once()
        self.assertEqual(self.app.events(), [])

    def test_one_scan_covers_every_configured_repository(self):
        self.watcher.repos = [REPO, "sebastian-software/another"]
        self.app.pull["labels"] = []
        self.watcher.scan()
        self.assertEqual([repo for repo, _, _ in self.app.calls], self.watcher.repos)
        self.run_model.assert_not_called()


class CodexTest(unittest.TestCase):
    def test_model_invocation_uses_chatgpt_and_excludes_keys(self):
        def execute(args, **kwargs):
            result = Path(args[args.index("--output-last-message") + 1])
            result.write_text(json.dumps({"complete": True, "findings": []}), encoding="utf-8")
            return subprocess.CompletedProcess(args, 0)

        keys = {"OPENAI_API_KEY": "test-api-key", "CODEX_API_KEY": "test-codex-key", "GH_TOKEN": "test-gh-key"}
        with patch.dict(os.environ, keys), patch("review.subprocess.run", side_effect=execute) as command:
            self.assertEqual(review.run_model("sol", "a diff", "trusted guidance"), [])
        args = command.call_args.args[0]
        env = command.call_args.kwargs["env"]
        self.assertTrue(keys.keys().isdisjoint(env))
        self.assertIn('forced_login_method="chatgpt"', args)
        self.assertIn("shell_tool", args)
        self.assertIn("gpt-6.2-sol", args)

    def test_incomplete_result_cannot_approve(self):
        with TemporaryDirectory() as directory:
            result = Path(directory) / "result.json"
            result.write_text('{"complete":false,"findings":[]}', encoding="utf-8")
            with self.assertRaisesRegex(ValueError, "complete review"):
                review.load_findings(result)


class GitHubAppTest(unittest.TestCase):
    def test_multiple_repos_share_one_installation_token(self):
        expires = (datetime.now(timezone.utc) + timedelta(hours=1)).isoformat()
        with patch.object(GitHubApp, "jwt", return_value="test-app-jwt"), patch(
            "github_app.api", side_effect=[
                {"slug": "test-review"}, {"id": 9}, {"token": "installation", "expires_at": expires}, {"id": 9},
            ],
        ) as request:
            app = GitHubApp("test-app-id", Path("test-key.pem"))
            self.assertEqual(app.token(REPO), "installation")
            self.assertEqual(app.token("sebastian-software/another"), "installation")
        self.assertEqual([call.args[1] for call in request.call_args_list], [
            "app", f"repos/{REPO}/installation", "app/installations/9/access_tokens",
            "repos/sebastian-software/another/installation",
        ])

    def test_paginated_requests_use_bearer_auth_on_github(self):
        first = BytesIO(b"[1]")
        first.headers = {"Link": '<https://api.github.com/repos/owner/repo/pulls?page=2>; rel="next"'}
        second = BytesIO(b"[2]")
        second.headers = {}
        with patch("github_app.OPENER.open", side_effect=[first, second]) as request:
            self.assertEqual(api("test-token", "repos/owner/repo/pulls", paginate=True), [1, 2])
        self.assertEqual(request.call_count, 2)
        self.assertEqual(request.call_args.args[0].get_header("Authorization"), "Bearer test-token")


if __name__ == "__main__":
    unittest.main()
