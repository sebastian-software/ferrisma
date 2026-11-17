#!/usr/bin/env python3
"""Open or update one GitHub issue per failed job of a nightly or deep run.

Engine ADR-0027, amendment 2026-11-17 (owner decision D5): a red job of the
nightly or the deep tier opens an issue titled `<Tier>: <job name> failed`,
or comments on the open issue of exactly that title, with the failing test
names the job's log shows and the link to the job and the run.

- Jobs are those of the run attempt (`gh api .../runs/ID/attempts/N/jobs`)
  whose conclusion is failure, cancelled or timed_out. An aggregating job
  (a gate, "Checks passed", this report) fails whenever a job it waits for
  fails, so it is reported only when no other job failed: then its own
  failure is the finding.
- Failing tests are read from the job log: libtest's `test NAME ... FAILED`,
  cargo's `error: test failed, to rerun pass ...`, unittest's
  `FAIL: test (case)` / `ERROR: ...`, and a partition's `N tests selected`
  problems (tools/partition.py). Without them, the issue quotes the last
  lines before the failing step's `##[error]` (a tool's own message, such
  as a partition record problem). A log that cannot be fetched (a cancelled
  job may have none) is said so in the issue.
- Open issues are matched by exact title through the REST API (all pages),
  not by search, so a new issue is not opened beside an unindexed one.
- `--ignore-job REGEX` leaves out jobs that report and never block (the
  Clippy canary); `--strip-prefix` removes the calling job's name from the
  titles when the run is a caller of nightly.yml (deep.yml).

--dry-run prints what would be opened or commented and writes nothing; it is
how the reporter is tested against a past failed run:

    python3 scripts/ci/report_failed_jobs.py --repo OWNER/NAME --run-id ID --tier Nightly --dry-run
"""

import argparse
import json
import re
import subprocess
import sys

FAILED = {"failure", "cancelled", "timed_out"}
AGGREGATORS = re.compile(r"(^|/ )(gate|.*Checks passed|Report failed jobs|.*Report the nightly failure)$")
TIMESTAMP = re.compile(r"^\d{4}-\d\d-\d\dT[0-9:.]+Z ")
ANSI = re.compile(r"\x1b\[[0-9;]*[A-Za-z]")
PATTERNS = [
    re.compile(r"^test (?P<name>\S.*?) \.\.\. FAILED$"),
    re.compile(r"error: test failed, to rerun pass `(?P<name>[^`]+)`"),
    re.compile(r"^(?:FAIL|ERROR): (?P<name>\S+ \([^)]*\))$"),
    re.compile(r"^(?P<name>(?:lib|doc|(?:bin|test|example|bench):\S+): \d+ tests selected, \d+ reported)$"),
]
LIMIT = 50


def gh(*args, text=True):
    return subprocess.run(["gh", *args], stdout=subprocess.PIPE, text=text, check=True).stdout


def failing_tests(log):
    """The failing test names of a job log, in order, without repeats."""
    found = []
    for line in log.splitlines():
        line = ANSI.sub("", TIMESTAMP.sub("", line)).rstrip()
        for pattern in PATTERNS:
            match = pattern.search(line)
            if match and match["name"] not in found:
                found.append(match["name"])
    return found


def error_context(log, lines=6):
    """The last lines before the first failing step's `##[error]` line."""
    seen = []
    for line in log.splitlines():
        line = ANSI.sub("", TIMESTAMP.sub("", line)).rstrip()
        if line.startswith("##[error]"):
            return seen[-lines:]
        if line and not line.startswith("##[") and not line.startswith("[command]"):
            seen.append(line)
    return []


def failed_jobs(jobs, ignore=None):
    """The jobs to report: failed ones, aggregators only if nothing else failed."""
    failed = [job for job in jobs if job.get("conclusion") in FAILED and not (ignore and ignore.search(job["name"]))]
    real = [job for job in failed if not AGGREGATORS.search(job["name"])]
    return real or failed


def title(tier, job, strip=""):
    name = job["name"]
    if strip and name.startswith(strip):
        name = name[len(strip):]
    return f"{tier}: {name} failed"


def body(tier, job, run, tests, log_error, context=()):
    lines = [
        f"The {tier.lower()} job **{job['name']}** ended with `{job['conclusion']}`.",
        "",
        f"- Job: {job['html_url']}",
        f"- Run: {run['html_url']} (attempt {run['run_attempt']}, `{run['event']}`)",
        f"- Commit: `{run['head_sha']}` on `{run['head_branch']}`",
        "",
    ]
    if log_error:
        lines.append(f"The job log could not be read ({log_error}); see the job link.")
    elif tests:
        shown = tests[:LIMIT]
        lines.append(f"Failing tests in the log ({len(tests)}" + (f", first {LIMIT}" if len(tests) > LIMIT else "") + "):")
        lines.append("")
        lines += [f"- `{name}`" for name in shown]
    else:
        lines.append("The log names no failing test (a build, setup or tool step failed, or the job was "
                     "cancelled); see the job link.")
        if context:
            lines += ["", "The last lines before the error:", "", "```", *context, "```"]
    lines += ["", "Opened by the failure report of engine ADR-0027 (amendment 2026-11-17, decision D5)."]
    return "\n".join(lines) + "\n"


class GitHub:
    def __init__(self, repo):
        self.repo = repo

    def run(self, run_id):
        return json.loads(gh("api", f"repos/{self.repo}/actions/runs/{run_id}"))

    def jobs(self, run_id, attempt):
        pages = gh("api", "--paginate", f"repos/{self.repo}/actions/runs/{run_id}/attempts/{attempt}/jobs?per_page=100",
                   "--jq", ".jobs[]")
        return [json.loads(line) for line in pages.splitlines() if line.strip()]

    def log(self, job_id):
        # Recent gh versions refuse to print a response with terminal escape
        # sequences (colored cargo output) without --allow-escape-sequences;
        # older ones do not know the flag.
        path = f"repos/{self.repo}/actions/jobs/{job_id}/logs"
        try:
            raw = gh("api", "--allow-escape-sequences", path, text=False)
        except subprocess.CalledProcessError:
            raw = gh("api", path, text=False)
        return raw.decode("utf-8", "replace")

    def open_issues(self):
        pages = gh("api", "--paginate", f"repos/{self.repo}/issues?state=open&per_page=100",
                   "--jq", ".[] | select(.pull_request == null) | {number, title}")
        return [json.loads(line) for line in pages.splitlines() if line.strip()]

    def comment(self, number, text):
        gh("issue", "comment", str(number), "--repo", self.repo, "--body", text)

    def create(self, issue_title, text):
        gh("issue", "create", "--repo", self.repo, "--title", issue_title, "--body", text)


def report(api, run_id, attempt, tier, dry_run, out=sys.stdout, ignore=None, strip=""):
    """Report the failed jobs of one run attempt; return the actions taken."""
    run = api.run(run_id)
    attempt = attempt or run["run_attempt"]
    run["run_attempt"] = attempt
    jobs = failed_jobs(api.jobs(run_id, attempt), re.compile(ignore) if ignore else None)
    if not jobs:
        print(f"No failed job in run {run_id}, attempt {attempt}.", file=out)
        return []
    issues = {}
    for issue in api.open_issues():
        issues.setdefault(issue["title"], issue["number"])
    actions = []
    for job in jobs:
        log_error, context = None, []
        try:
            log = api.log(job["id"])
            tests, context = failing_tests(log), error_context(log)
        except subprocess.CalledProcessError as error:
            tests, log_error = [], f"exit {error.returncode}"
        issue_title = title(tier, job, strip)
        text = body(tier, job, run, tests, log_error, context)
        number = issues.get(issue_title)
        action = ("comment", number) if number else ("create", None)
        actions.append((action[0], issue_title, number, tests))
        verb = f"comment on #{number}" if number else "open"
        print(f"{'Would ' if dry_run else ''}{verb}: {issue_title} ({len(tests)} failing tests)", file=out)
        if dry_run:
            print("".join(f"    {line}\n" for line in text.splitlines()), file=out)
        elif number:
            api.comment(number, text)
        else:
            api.create(issue_title, text)
    return actions


def main():
    parser = argparse.ArgumentParser(description=__doc__, formatter_class=argparse.RawDescriptionHelpFormatter)
    parser.add_argument("--repo", required=True, help="OWNER/NAME")
    parser.add_argument("--run-id", required=True, type=int)
    parser.add_argument("--attempt", type=int, help="run attempt (default: the run's latest)")
    parser.add_argument("--tier", required=True, choices=["Nightly", "Deep"], help="issue title prefix")
    parser.add_argument("--ignore-job", metavar="REGEX", help="leave out the jobs whose names match")
    parser.add_argument("--strip-prefix", default="", help="remove this prefix of job names from the titles")
    parser.add_argument("--dry-run", action="store_true", help="print the issues and comments, write nothing")
    args = parser.parse_args()
    report(GitHub(args.repo), args.run_id, args.attempt, args.tier, args.dry_run, ignore=args.ignore_job,
           strip=args.strip_prefix)


if __name__ == "__main__":
    main()
