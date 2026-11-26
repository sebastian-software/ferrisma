"""Run the opt-in Sol/Astra review workflow for one pull request commit."""

import json
import os
from pathlib import Path
import re
import subprocess
import sys


LABEL = "codex-review"
CONTEXT = "Astra Review (pilot)"
BOT = "github-actions[bot]"
MARKER = re.compile(r"^<!-- ferrisma-codex-review:(sol|astra):([0-9a-f]{40}) -->\n")
MAX_DIFF_BYTES = 400_000


class GitHubError(RuntimeError):
    pass


def github(path, *, payload=None, paginate=False):
    args = ["gh", "api", f"repos/{os.environ['GITHUB_REPOSITORY']}/{path}"]
    if paginate:
        args.extend(["--paginate", "--slurp"])
    if payload is not None:
        args.extend(["--method", "POST", "--input", "-"])
    response = subprocess.run(
        args, input=json.dumps(payload) if payload is not None else None,
        text=True, capture_output=True, check=False,
    )
    if response.returncode:
        raise GitHubError(response.stderr.strip())
    return json.loads(response.stdout)


def output(name, value):
    with Path(os.environ["GITHUB_OUTPUT"]).open("a", encoding="utf-8") as stream:
        stream.write(f"{name}={value}\n")


def load_findings(path):
    result = json.loads(path.read_text(encoding="utf-8"))
    if not isinstance(result, dict) or result.get("complete") is not True:
        raise ValueError("Codex did not return a complete review")
    findings = result.get("findings")
    if not isinstance(findings, list) or len(findings) > 25:
        raise ValueError("Codex returned an invalid findings list")
    for finding in findings:
        if (
            not isinstance(finding, dict)
            or not isinstance(finding.get("path"), str)
            or not finding["path"].strip()
            or type(finding.get("line")) is not int
            or finding["line"] < 0
            or not isinstance(finding.get("body"), str)
            or not 0 < len(finding["body"].strip()) <= 1_500
        ):
            raise ValueError("Codex returned an invalid finding")
    return findings


def review_payload(stage, base, head, findings):
    model = {"sol": "GPT-6.2 Sol", "astra": "GPT-6.2 Astra"}[stage]
    marker = f"<!-- ferrisma-codex-review:{stage}:{base} -->"
    if not findings:
        if stage != "astra":
            raise ValueError("Only Astra can approve")
        return {
            "commit_id": head, "event": "APPROVE",
            "body": f"{marker}\n{model} completed the review and found no blockers.",
        }

    lines = [marker, f"{model} found {len(findings)} blocking issue(s).", ""]
    comments = []
    for finding in findings:
        path, line = finding["path"], finding["line"]
        body = finding["body"].strip().replace("@", "@\u200b")
        location = f"{path}:{line}" if line else path
        location = " ".join(location.split()).replace("`", "'").replace("@", "@\u200b")
        lines.append(f"- `{location}` {body}")
        if line:
            comments.append({"path": path, "line": line, "side": "RIGHT", "body": body})
    return {
        "commit_id": head, "event": "REQUEST_CHANGES",
        "body": "\n".join(lines), "comments": comments,
    }


class PullRequest:
    def __init__(self, event):
        pull = event["pull_request"]
        self.number = pull["number"]
        self.base = pull["base"]["sha"]
        self.head = pull["head"]["sha"]
        self.branch = event["repository"]["default_branch"]
        self.endpoint = f"pulls/{self.number}"
        self.temp = Path(os.environ["RUNNER_TEMP"])

    def is_current(self):
        pull = github(self.endpoint)
        return (
            pull["head"]["sha"] == self.head
            and pull["base"]["ref"] == self.branch
            and pull["state"] == "open"
            and not pull["draft"]
            and any(label["name"] == LABEL for label in pull["labels"])
        )

    def status(self, state, description):
        url = (
            f"{os.environ['GITHUB_SERVER_URL']}/{os.environ['GITHUB_REPOSITORY']}"
            f"/actions/runs/{os.environ['GITHUB_RUN_ID']}"
        )
        github(f"statuses/{self.head}", payload={
            "context": CONTEXT, "state": state, "description": description, "target_url": url,
        })

    def reviews(self):
        for page in github(f"{self.endpoint}/reviews?per_page=100", paginate=True):
            for review in page:
                marker = MARKER.match(review.get("body") or "")
                if marker and (review.get("user") or {}).get("login") == BOT:
                    yield marker.group(1), marker.group(2), review

    def prepare(self):
        if not self.is_current():
            return
        reviews = list(self.reviews())
        for stage, base, review in reversed(reviews):
            if base != self.base or review["commit_id"] != self.head:
                continue
            if stage == "astra" and review["state"] == "APPROVED":
                self.status("success", "Astra approved this commit")
                return
            if review["state"] == "CHANGES_REQUESTED":
                self.status("failure", f"{stage.title()} requested changes")
                return

        if os.environ.get("OPENAI_CONFIGURED") != "true":
            raise RuntimeError("Add the OPENAI_API_KEY Actions secret before labeling a pilot PR")
        self.status("pending", "Waiting for Sol and Astra")
        output("pending", "true")
        subprocess.run(
            ["git", "fetch", "--no-tags", "origin", f"refs/pull/{self.number}/head"], check=True,
        )
        fetched = subprocess.check_output(["git", "rev-parse", "FETCH_HEAD"], text=True).strip()
        if fetched != self.head or not self.is_current():
            return

        # Only the trusted checkout is executed. The fetched head is used as Git data.
        diff = self.temp / "ferrisma-review.diff"
        with diff.open("wb") as stream:
            subprocess.run([
                "git", "diff", "--no-color", "--no-ext-diff", "--no-textconv",
                f"{self.base}...{self.head}", "--",
            ], stdout=stream, check=True)
        if diff.stat().st_size > MAX_DIFF_BYTES:
            raise RuntimeError("The PR diff exceeds the pilot's review limit")

        prompt = Path(__file__).with_name("review-prompt.md").read_text(encoding="utf-8")
        prompt = prompt.replace("{diff}", str(diff))
        (self.temp / "ferrisma-sol.md").write_text(prompt, encoding="utf-8")
        previous = next((review for stage, _, review in reversed(reviews) if stage == "astra"), None)
        if previous and previous["state"] == "CHANGES_REQUESTED":
            history = self.temp / "ferrisma-astra-history.md"
            history.write_text(previous["body"], encoding="utf-8")
            prompt += f"\nRecheck the previous Astra findings in `{history}` against this patch.\n"
        (self.temp / "ferrisma-astra.md").write_text(prompt, encoding="utf-8")
        output("review", "true")

    def publish(self, stage):
        findings = load_findings(self.temp / f"ferrisma-{stage}.json")
        if not self.is_current():
            return
        if stage == "sol" and not findings:
            self.status("pending", "Sol found no blockers; waiting for Astra")
            output("escalate", "true")
            return

        payload = review_payload(stage, self.base, self.head, findings)
        endpoint = f"{self.endpoint}/reviews"
        try:
            github(endpoint, payload=payload)
        except GitHubError as error:
            if not payload.get("comments") or "HTTP 422" not in str(error):
                raise
            if not self.is_current():
                return
            # Review still has file and line references when GitHub rejects an anchor.
            del payload["comments"]
            github(endpoint, payload=payload)

        if self.is_current():
            if findings:
                self.status("failure", f"{stage.title()} requested changes")
            else:
                self.status("success", "Astra approved this commit")


def main():
    event = json.loads(Path(os.environ["GITHUB_EVENT_PATH"]).read_text(encoding="utf-8"))
    pull = PullRequest(event)
    stage = sys.argv[1]
    if stage == "prepare":
        pull.prepare()
    elif stage in {"sol", "astra"}:
        pull.publish(stage)
    elif stage == "error":
        if pull.is_current():
            pull.status("error", "Codex review could not finish; see workflow logs")
    else:
        raise ValueError(f"Unknown review stage: {stage}")


if __name__ == "__main__":
    main()
