#!/usr/bin/env python3
"""Verify that the upstream submodules sit at the commits pinned in upstream.json.

For each pin it checks that the Git index records the submodule (mode 160000)
at the pinned commit, that the checkout's HEAD is that commit, that the tag
(where one is pinned and fetched) points at it, and that the checkout has no modified,
staged or untracked files. It also checks that LICENSE, PATENTS and AUTHORS are
unchanged copies of upstream COPYING, PATENTS and AUTHORS.
"""

import json
from pathlib import Path
import subprocess

ROOT = Path(__file__).resolve().parent.parent
NOTICES = {"LICENSE": "COPYING", "PATENTS": "PATENTS", "AUTHORS": "AUTHORS"}


def git(*args, cwd=ROOT):
    return subprocess.check_output(["git", *args], cwd=cwd, text=True).strip()


def verify():
    problems = []
    pins = json.loads((ROOT / "upstream.json").read_text())
    workspace = ROOT.parents[1]
    gitmodules = git("config", "--file", str(workspace / ".gitmodules"),
                     "--get-regexp", r"submodule\..*\.(path|url)")
    for name, pin in pins.items():
        path, revision = pin["path"], pin["revision"]
        workspace_path = str((ROOT / path).relative_to(workspace))
        if f"submodule.{workspace_path}.path {workspace_path}" not in gitmodules.splitlines():
            problems.append(f"{name}: .gitmodules has no submodule at {path}")
        if f"submodule.{workspace_path}.url {pin['repository']}" not in gitmodules.splitlines():
            problems.append(f"{name}: .gitmodules URL differs from {pin['repository']}")
        entry = git("ls-files", "--stage", "--", path).split()
        if len(entry) < 2 or entry[0] != "160000" or entry[1] != revision:
            problems.append(f"{name}: the Git index does not record {path} at {revision}")
        checkout = ROOT / path
        if not (checkout / ".git").exists():
            problems.append(f"{name}: {path} is not initialized; run "
                            f"git submodule update --init {path}")
            continue
        head = git("rev-parse", "HEAD", cwd=checkout)
        if head != revision:
            problems.append(f"{name}: {path} is at {head}, expected {revision}")
        if "tag" in pin:
            try:
                tagged = git("rev-parse", f"refs/tags/{pin['tag']}^{{commit}}", cwd=checkout)
            except subprocess.CalledProcessError:
                tagged = None
            if tagged is None:
                # A shallow clone may lack tags; the commit check above decides.
                print(f"Note: tag {pin['tag']} is not available in {path}; checked the commit only")
            elif tagged != revision:
                problems.append(f"{name}: tag {pin['tag']} points at {tagged}, expected {revision}")
        dirty = git("status", "--porcelain", "--untracked-files=all", "--ignored=no", cwd=checkout)
        if dirty:
            problems.append(f"{name}: {path} must stay unmodified:\n{dirty}")
    libwebp = ROOT / pins["libwebp"]["path"]
    for copy, original in NOTICES.items():
        if (ROOT / copy).read_bytes() != (libwebp / original).read_bytes():
            problems.append(f"{copy} differs from upstream {original}")
    return problems


def main():
    problems = verify()
    for problem in problems:
        print(problem)
    if problems:
        raise SystemExit(f"{len(problems)} upstream provenance problem(s)")
    pins = json.loads((ROOT / "upstream.json").read_text())
    summary = ", ".join(f"{pin['path']} at {pin['revision'][:12]}" for pin in pins.values())
    print(f"Upstream pins and notices verified: {summary}", flush=True)


if __name__ == "__main__":
    main()
