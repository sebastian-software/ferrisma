#!/usr/bin/env python3
"""Run one of N disjoint partitions of a `cargo test` invocation's tests.

The nightly tier gives a hosted job at most 60 minutes (engine ADR-0027,
amendment 2026-11-17), and this port's asan-ubsan oracle stage takes longer
on a hosted macOS runner. `tools/check.py --partition K/N` therefore runs,
of every `cargo test` invocation, only the test targets (the library's unit
tests, each integration test, the doctests) dealt to partition K. The N partitions
together run every test exactly once:

- `cargo test <args> --no-run --message-format=json-render-diagnostics`
  builds what `cargo test <args>` builds and names each test executable;
  `<executable> --list --format terse` and
  `cargo test <args> --doc -- --list --format terse` list their tests.
- Whole targets are dealt, not single tests: libtest runs a target's tests
  in parallel, so a target takes about as long as its slowest test, and
  splitting a target's tests over partitions makes every partition wait for
  one of its slow tests (hosted run 44177110508: the dev profile took as
  long in each of three partitions as unpartitioned). The caller's HEAVY
  targets, the slowest in the hosted logs, are dealt first in their order,
  then the others sorted; the deal alternates direction (1..N, then N..1,
  ...), as libjxl-rs's tools/check.py deals its test names, and starts one
  partition later for each further run of the check (the next profile or
  SIMD mode), so that the slowest target of every run lands in a different
  partition. HEAVY only orders the deal: a HEAVY target that no run of the
  invocation lists fails, so the list stays current.
- Each dealt target runs whole, as `cargo test <args> --test NAME` (`--lib`,
  `--doc`, `--bin NAME`), and the test count it reports must equal the
  number of tests it listed, or the partition fails. A failing target does
  not stop the later ones; the partition fails at the end.
- Every partition writes the listed and the selected (target, test) items of
  every run under build/partitions/, and
  `python3 tools/partition.py verify <record files or directories>` fails
  unless the records of each group come from one commit, cover the
  partitions 1..N exactly once, list the same items and select every item
  exactly once (the hosted completeness job runs it over all shards);
  `--expect G1,G2,...` also requires exactly these groups.

`python3 tools/partition.py plan K/N [HEAVY...] -- <cargo test args>` prints a
partition's targets without running a test.
"""

import json
import os
from pathlib import Path
import platform
import re
import subprocess
import sys

# The order in which cargo runs the kinds of test targets.
KIND_ORDER = {"lib": 0, "bin": 1, "test": 2, "example": 3, "bench": 4, "doc": 5}
RESULT = re.compile(r"test result: (?:ok|FAILED)\. (\d+) passed; (\d+) failed; (\d+) ignored; (\d+) measured;")
TARGET = re.compile(r"lib|doc|(bin|test|example|bench):[A-Za-z0-9_-]+")


def parse(value):
    """K/N with 1 <= K <= N."""
    match = re.fullmatch(r"([1-9][0-9]*)/([1-9][0-9]*)", value)
    if not match or int(match[1]) > int(match[2]):
        raise ValueError(f"expected K/N with 1 <= K <= N, not {value!r}")
    return int(match[1]), int(match[2])


def target(text):
    """A HEAVY entry: 'lib', 'doc' or 'KIND:NAME' such as 'test:wp13_tools'."""
    if not TARGET.fullmatch(text):
        raise ValueError(f"not a test target 'lib', 'doc' or 'test:NAME': {text!r}")
    return text


def order_key(key):
    return KIND_ORDER[key.split(":")[0]], key


def deal(targets, heavy, index, count, rotation=0):
    """The targets of partition `index` of `count`: the HEAVY targets that
    are listed, in their order, then the others sorted, dealt out in
    alternating direction, starting `rotation` partitions later."""
    listed = set(targets)
    order = [entry for entry in heavy if entry in listed]
    order += sorted(entry for entry in listed if entry not in set(heavy))
    selected = []
    for position, entry in enumerate(order):
        slot = position % count
        if (position // count) % 2:
            slot = count - 1 - slot
        if (slot + rotation) % count == index - 1:
            selected.append(entry)
    return sorted(selected, key=order_key)


def names(listing):
    """The test names of a libtest `--list --format terse` listing."""
    return sorted({line.rsplit(": ", 1)[0] for line in listing.splitlines()
                   if line.endswith(": test") or line.endswith(": bench")})


def targets(messages):
    """The test executables of `cargo test --no-run --message-format=json`
    output, as (target, executable) sorted in cargo's order of kinds."""
    found = {}
    for line in messages.splitlines():
        if not line.startswith("{"):
            continue
        message = json.loads(line)
        if message.get("reason") != "compiler-artifact" or not message.get("executable"):
            continue
        if not message.get("profile", {}).get("test"):
            continue
        kinds = message["target"]["kind"]
        if any(kind in ("lib", "rlib", "staticlib", "cdylib", "dylib", "proc-macro") for kind in kinds):
            key = "lib"
        elif kinds[0] in ("bin", "test", "example", "bench"):
            key = f"{kinds[0]}:{message['target']['name']}"
        else:
            raise SystemExit(f"unknown test target kind {kinds} of {message['target']['name']}")
        found[key] = message["executable"]
    return sorted(found.items(), key=lambda entry: order_key(entry[0]))


def selector(key):
    """The cargo arguments that select one test target."""
    if key in ("lib", "doc"):
        return [f"--{key}"]
    kind, name = key.split(":", 1)
    return [f"--{kind}", name]


def show(command):
    print("+ " + " ".join(map(str, command)), flush=True)


def listing(cargo_args, root, env):
    """Build the invocation's tests and list them: every (target, test name)."""
    command = ["cargo", "test", *cargo_args, "--no-run", "--message-format=json-render-diagnostics"]
    show(command)
    built = subprocess.run(command, cwd=root, env=env, stdout=subprocess.PIPE, text=True, errors="replace",
                           check=True).stdout
    items = []
    executables = targets(built)
    for key, executable in executables:
        listed = subprocess.run([executable, "--list", "--format", "terse"], cwd=root, env=env,
                                stdout=subprocess.PIPE, text=True, check=True).stdout
        items += [(key, name) for name in names(listed)]
    if any(key == "lib" for key, _ in executables):
        command = ["cargo", "test", *cargo_args, "--doc", "--", "--list", "--format", "terse"]
        show(command)
        listed = subprocess.run(command, cwd=root, env=env, stdout=subprocess.PIPE, text=True, check=True).stdout
        items += [("doc", name) for name in names(listed)]
    return items


def counted(command, root, env):
    """Run a cargo test command, echo its output, and return its exit status
    and the number of tests the test results report."""
    show(command)
    process = subprocess.Popen(command, cwd=root, env=env, stdout=subprocess.PIPE, text=True, errors="replace")
    total = 0
    for line in process.stdout:
        sys.stdout.write(line)
        match = RESULT.search(line)
        if match:
            total += sum(int(value) for value in match.groups())
    sys.stdout.flush()
    return process.wait(), total


def run(cargo_args, root, env, selected, test_args=()):
    """Run the targets of the selected items whole; return the problems."""
    problems = []
    expected = {}
    for key, _ in selected:
        expected[key] = expected.get(key, 0) + 1
    for key in sorted(expected, key=order_key):
        command = ["cargo", "test", *cargo_args, *selector(key)]
        if test_args:
            command += ["--", *test_args]
        status, total = counted(command, root, env)
        if status:
            problems.append(f"{key}: cargo test exited with {status}")
        elif total != expected[key]:
            problems.append(f"{key}: {expected[key]} tests listed, {total} reported")
    return problems


class Partition:
    """One partition K/N of every `cargo test` invocation of a check.py run,
    with the record of what it listed and selected."""

    def __init__(self, index, count, heavy, root, group):
        self.index, self.count = index, count
        self.heavy = [target(entry) for entry in heavy]
        self.root = Path(root)
        self.group = f"{sys.platform}-{platform.machine()}-{group}"
        self.runs = {}

    def test(self, label, cargo_args, env, test_args=()):
        """List, deal and run one invocation; return its problems."""
        if label in self.runs:
            raise SystemExit(f"partition run {label!r} appears twice")
        items = listing(cargo_args, self.root, env)
        dealt = set(deal({key for key, _ in items}, self.heavy, self.index, self.count, rotation=len(self.runs)))
        selected = [entry for entry in items if entry[0] in dealt]
        self.runs[label] = {"items": sorted(map(list, items)), "selected": sorted(map(list, selected))}
        print(f"Partition {self.index}/{self.count} of {label}: {len(dealt)} targets, "
              f"{len(selected)} of {len(items)} tests: {', '.join(sorted(dealt, key=order_key))}", flush=True)
        self.write()
        return run(cargo_args, self.root, env, selected, test_args)

    def check_heavy(self):
        """Fail if a HEAVY target is listed by none of the runs."""
        listed = {entry[0] for run_record in self.runs.values() for entry in run_record["items"]}
        missing = [entry for entry in self.heavy if entry not in listed]
        if self.runs and missing:
            raise SystemExit(f"HEAVY names test targets that no run lists: {missing}")

    def path(self):
        return self.root / "build" / "partitions" / self.group / f"{self.index}-of-{self.count}.json"

    def write(self):
        commit = subprocess.run(["git", "rev-parse", "HEAD"], cwd=self.root, stdout=subprocess.PIPE, text=True,
                                check=True).stdout.strip()
        path = self.path()
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps({"group": self.group, "partition": [self.index, self.count], "commit": commit,
                                    "runs": self.runs}, indent=1, sort_keys=True) + "\n")


def verify(records, expected=None):
    """The problems of a set of partition records (see the module docstring)."""
    problems = []
    groups = {}
    for record in records:
        groups.setdefault(record["group"], []).append(record)
    if not groups:
        problems.append("no partition records")
    if expected is not None:
        missing = sorted(set(expected) - set(groups))
        unexpected = sorted(set(groups) - set(expected))
        if missing:
            problems.append(f"no records of the groups {missing}")
        if unexpected:
            problems.append(f"records of unexpected groups {unexpected}")
    for group, members in sorted(groups.items()):
        counts = {member["partition"][1] for member in members}
        commits = {member["commit"] for member in members}
        if len(counts) != 1:
            problems.append(f"{group}: records of different partition counts {sorted(counts)}")
            continue
        if len(commits) != 1:
            problems.append(f"{group}: records of different commits {sorted(commits)}")
        count = counts.pop()
        indices = sorted(member["partition"][0] for member in members)
        if indices != list(range(1, count + 1)):
            problems.append(f"{group}: partitions {indices} of {count}, expected each of 1..{count} once")
            continue
        labels = {tuple(sorted(member["runs"])) for member in members}
        if len(labels) != 1:
            problems.append(f"{group}: the partitions ran different invocations")
            continue
        for label in labels.pop():
            listed = {tuple(map(tuple, member["runs"][label]["items"])) for member in members}
            if len(listed) != 1:
                problems.append(f"{group}, {label}: the partitions listed different tests")
                continue
            items = set(listed.pop())
            seen = {}
            for member in members:
                for entry in map(tuple, member["runs"][label]["selected"]):
                    if entry not in items:
                        problems.append(f"{group}, {label}: {'::'.join(entry)} selected but not listed")
                    seen[entry] = seen.get(entry, 0) + 1
            twice = sorted("::".join(entry) for entry, times in seen.items() if times > 1)
            never = sorted("::".join(entry) for entry in items - set(seen))
            if twice:
                problems.append(f"{group}, {label}: selected by several partitions: {twice}")
            if never:
                problems.append(f"{group}, {label}: selected by no partition: {never}")
    return problems


def load(paths):
    records = []
    for path in map(Path, paths):
        files = sorted(path.rglob("*-of-*.json")) if path.is_dir() else [path]
        records += [json.loads(file.read_text()) for file in files]
    return records


def main(argv):
    if argv[:1] == ["verify"] and len(argv) > 1:
        expected = None
        if argv[1] == "--expect" and len(argv) > 3:
            expected = [group.strip() for group in argv[2].split(",") if group.strip()]
            argv = argv[2:]
        records = load(argv[1:])
        problems = verify(records, expected)
        if problems:
            raise SystemExit("\n".join(problems))
        groups = sorted({record["group"] for record in records})
        print(f"Partition records complete: {len(records)} records, groups {', '.join(groups)}")
        return
    if argv[:1] == ["plan"] and "--" in argv:
        split = argv.index("--")
        index, count = parse(argv[1])
        heavy = [target(entry) for entry in argv[2:split]]
        items = listing(argv[split + 1:], Path.cwd(), dict(os.environ))
        for key in deal({key for key, _ in items}, heavy, index, count):
            print(key)
        return
    raise SystemExit("usage: partition.py verify [--expect G1,G2,...] RECORD|DIR... | "
                     "plan K/N [HEAVY...] -- CARGO_TEST_ARGS...")


if __name__ == "__main__":
    main(sys.argv[1:])
