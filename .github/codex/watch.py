"""Poll GitHub pull requests and review new commits with the local Codex CLI."""

import argparse
import base64
import fcntl
import json
import os
from pathlib import Path
import re
import subprocess
import sys
from tempfile import NamedTemporaryFile
import time

from github_app import GitHubApp, GitHubError
from review import require_chatgpt_login, review_payload, run_model


CONTEXT = "Astra Review (pilot)"
MARKER = re.compile(r"^<!-- ferrisma-codex-review:(sol|astra):([0-9a-f]{40}) -->\n")
MAX_DIFF_BYTES = 400_000
RETRY_SECONDS = 300


class Watcher:
    def __init__(self, app, repos, label, state_path):
        self.app = app
        self.repos = repos
        self.label = label
        self.state_path = state_path
        self.state = json.loads(state_path.read_text(encoding="utf-8")) if state_path.exists() else {}

    def request(self, repo, path, **kwargs):
        return self.app.request(repo, path, **kwargs)

    def eligible(self, pull):
        return (
            pull["state"] == "open" and not pull["draft"]
            and pull["base"]["ref"] == pull["base"]["repo"]["default_branch"]
            and any(label["name"] == self.label for label in pull["labels"])
        )

    def current(self, repo, pull):
        latest = self.request(repo, f"pulls/{pull['number']}")
        return self.eligible(latest) and latest["head"]["sha"] == pull["head"]["sha"]

    def status(self, repo, pull, state, description):
        self.request(repo, f"statuses/{pull['head']['sha']}", payload={
            "context": CONTEXT, "state": state, "description": description,
            "target_url": pull["html_url"],
        })

    def reviews(self, repo, number):
        for review in self.request(repo, f"pulls/{number}/reviews?per_page=100", paginate=True):
            marker = MARKER.match(review.get("body") or "")
            if marker and (review.get("user") or {}).get("login") == self.app.login:
                yield marker.group(1), marker.group(2), review

    def remember(self, key, head, *, retry_at=None):
        self.state[key] = {"head": head, "retry_at": retry_at}
        with NamedTemporaryFile(mode="w", encoding="utf-8", dir=self.state_path.parent, delete=False) as stream:
            temporary = Path(stream.name)
            json.dump(self.state, stream)
        try:
            os.replace(temporary, self.state_path)
        finally:
            temporary.unlink(missing_ok=True)

    def guidance(self, repo, base):
        try:
            content = self.request(repo, f"contents/AGENTS.md?ref={base}")
        except GitHubError as error:
            if error.has_status(404):
                return ""
            raise
        return base64.b64decode(content["content"]).decode("utf-8")[:32_000]

    def publish(self, repo, pull, stage, findings):
        if not self.current(repo, pull):
            return False
        endpoint = f"pulls/{pull['number']}/reviews"
        payload = review_payload(stage, pull["base"]["sha"], pull["head"]["sha"], findings)
        try:
            self.request(repo, endpoint, payload=payload)
        except GitHubError as error:
            if not payload.get("comments") or not error.has_status(422):
                raise
            if not self.current(repo, pull):
                return False
            # Preserve the findings in the review body if GitHub rejects an inline anchor.
            del payload["comments"]
            self.request(repo, endpoint, payload=payload)
        if not self.current(repo, pull):
            return False
        if findings:
            self.status(repo, pull, "failure", f"{stage.title()} requested changes")
        else:
            self.status(repo, pull, "success", "Astra approved this commit")
        print(f"{repo}#{pull['number']}: {payload['event']} ({stage})", flush=True)
        return True

    def review(self, repo, pull):
        if not self.current(repo, pull):
            return False
        number, base, head = pull["number"], pull["base"]["sha"], pull["head"]["sha"]
        reviews = list(self.reviews(repo, number))
        # A previous process may have posted the review before it saved local state.
        for stage, reviewed_base, review in reversed(reviews):
            if reviewed_base != base or review["commit_id"] != head:
                continue
            if review["state"] == "CHANGES_REQUESTED":
                self.status(repo, pull, "failure", f"{stage.title()} requested changes")
                return True
            if stage == "astra" and review["state"] == "APPROVED":
                self.status(repo, pull, "success", "Astra approved this commit")
                return True

        diff = self.request(repo, f"pulls/{number}", raw=True)
        if not diff.startswith("diff --git ") or len(diff.encode("utf-8")) > MAX_DIFF_BYTES:
            raise RuntimeError("The PR has no reviewable text diff or exceeds the pilot's diff limit")
        if not self.current(repo, pull):
            return False
        guidance = self.guidance(repo, base)
        self.status(repo, pull, "pending", "Sol is reviewing this commit")
        findings = run_model("sol", diff, guidance)
        if findings:
            return self.publish(repo, pull, "sol", findings)

        if not self.current(repo, pull):
            return False
        previous = next((review for stage, _, review in reversed(reviews) if stage == "astra"), None)
        open_findings = previous["body"] if previous and previous["state"] == "CHANGES_REQUESTED" else ""
        self.status(repo, pull, "pending", "Sol found no blockers. Waiting for Astra.")
        findings = run_model("astra", diff, guidance, open_findings)
        return self.publish(repo, pull, "astra", findings)

    def scan(self):
        complete = True
        for repo in self.repos:
            try:
                pulls = self.request(repo, "pulls?state=open&per_page=100", paginate=True)
            except (RuntimeError, OSError, subprocess.SubprocessError) as error:
                print(f"{repo}: {error}", file=sys.stderr, flush=True)
                complete = False
                continue
            for pull in filter(self.eligible, pulls):
                key, head = f"{repo}#{pull['number']}", pull["head"]["sha"]
                saved = self.state.get(key, {})
                retry_at = saved.get("retry_at")
                if saved.get("head") == head and (retry_at is None or retry_at > time.time()):
                    continue
                try:
                    if self.review(repo, pull):
                        self.remember(key, head)
                except (RuntimeError, ValueError, OSError, subprocess.SubprocessError) as error:
                    print(f"{key}: {error}", file=sys.stderr, flush=True)
                    complete = False
                    self.remember(key, head, retry_at=time.time() + RETRY_SECONDS)
                    try:
                        if self.current(repo, pull):
                            self.status(repo, pull, "error", "Codex review did not finish. See watcher logs.")
                    except (RuntimeError, OSError, subprocess.SubprocessError) as status_error:
                        print(f"{key}: could not record failed status: {status_error}", file=sys.stderr, flush=True)
        return complete


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--repo", required=True, action="append", help="GitHub owner/repo. Repeat for multiple repos.")
    parser.add_argument("--app-id", required=True, help="GitHub App client ID or numeric app ID")
    parser.add_argument("--private-key", required=True, type=Path, help="GitHub App PEM file")
    parser.add_argument("--label", default="codex-review", help="Opt-in label on ready PRs")
    parser.add_argument("--state-dir", type=Path, default=Path.home() / ".local/state/ferrisma-codex-review")
    parser.add_argument("--interval", type=int, default=60, help="Minimum seconds between scans (default: 60)")
    parser.add_argument("--once", action="store_true", help="Run a single scan for a scheduler or smoke test")
    args = parser.parse_args()
    if args.interval < 1 or any(not re.fullmatch(r"[\w.-]+/[\w.-]+", repo) for repo in args.repo):
        parser.error("Provide an interval of at least one second and use owner/repo format")

    args.state_dir.mkdir(parents=True, mode=0o700, exist_ok=True)
    with (args.state_dir / "watch.lock").open("w") as lock:
        try:
            fcntl.flock(lock, fcntl.LOCK_EX | fcntl.LOCK_NB)
        except BlockingIOError:
            print("The watcher is already running for this state directory.", file=sys.stderr)
            return
        require_chatgpt_login()
        app = GitHubApp(args.app_id, args.private_key.expanduser())
        watcher = Watcher(app, dict.fromkeys(args.repo), args.label, args.state_dir / "state.json")
        while True:
            started = time.monotonic()
            complete = watcher.scan()
            if args.once:
                return 0 if complete else 1
            time.sleep(max(0, args.interval - (time.monotonic() - started)))


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (RuntimeError, OSError, subprocess.SubprocessError) as error:
        sys.exit(str(error))
