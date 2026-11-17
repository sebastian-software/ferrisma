#!/usr/bin/env python3
"""Unit tests of tools/partition.py: python3 tools/partition_test.py"""

import json
from pathlib import Path
import random
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parent))

import partition  # noqa: E402


def keys(count):
    return ["lib", "doc"] + [f"test:wp{number}" for number in range(count - 2)]


def items(count, per_target=3):
    return [(key, f"t{number}") for key in keys(count) for number in range(per_target)]


def records(all_items, count, heavy=(), commit="c0", group="g", label="dev"):
    result = []
    for index in range(1, count + 1):
        dealt = set(partition.deal({key for key, _ in all_items}, list(heavy), index, count))
        result.append({"group": group, "partition": [index, count], "commit": commit,
                       "runs": {label: {"items": sorted(map(list, all_items)),
                                        "selected": sorted(list(entry) for entry in all_items if entry[0] in dealt)}}})
    return result


class Parse(unittest.TestCase):
    def test_accepts_k_of_n(self):
        self.assertEqual(partition.parse("2/5"), (2, 5))
        self.assertEqual(partition.parse("1/1"), (1, 1))

    def test_rejects_others(self):
        for value in ("0/3", "4/3", "1/0", "a/b", "1", "1/2/3", " 1/2"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                partition.parse(value)

    def test_targets(self):
        for value in ("lib", "doc", "test:wp13_tools", "bin:cwebp-rs"):
            self.assertEqual(partition.target(value), value)
        for value in ("wp13_tools", "test:", "test:a b", "test:wp13::x", "lib::x"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                partition.target(value)


class Deal(unittest.TestCase):
    def test_every_target_in_exactly_one_partition(self):
        generator = random.Random(7)
        for count in range(1, 8):
            for size in (2, 3, count, count + 1, 3 * count + 1, 40):
                listed = keys(size)
                heavy = generator.sample(listed, min(len(listed), 5))
                for rotation in (0, 1, count + 2):
                    dealt = [partition.deal(listed, heavy, index, count, rotation) for index in range(1, count + 1)]
                    flat = [entry for share in dealt for entry in share]
                    with self.subTest(count=count, size=size, rotation=rotation):
                        self.assertEqual(sorted(flat), sorted(listed))
                        self.assertEqual(len(flat), len(set(flat)))

    def test_heavy_first_in_alternating_direction(self):
        listed = keys(14)
        heavy = ["test:wp11", "test:wp10", "test:wp9", "test:wp8", "test:wp7", "test:wp6"]
        shares = [partition.deal(listed, heavy, index, 3) for index in (1, 2, 3)]
        # Heavy positions 0, 1, 2 go to 1, 2, 3; positions 3, 4, 5 to 3, 2, 1.
        self.assertTrue({heavy[0], heavy[5]} <= set(shares[0]))
        self.assertTrue({heavy[1], heavy[4]} <= set(shares[1]))
        self.assertTrue({heavy[2], heavy[3]} <= set(shares[2]))

    def test_each_further_run_starts_one_partition_later(self):
        listed = keys(20)
        owners = [[index for index in (1, 2, 3, 4) if "test:wp13" in partition.deal(listed, ["test:wp13"], index, 4,
                                                                                     rotation)]
                  for rotation in range(6)]
        self.assertEqual(owners, [[1], [2], [3], [4], [1], [2]])

    def test_cargo_order_and_independence_of_listing_order(self):
        listed = keys(12)
        shuffled = list(listed)
        random.Random(3).shuffle(shuffled)
        for index in (1, 2, 3):
            share = partition.deal(listed, ["test:wp5"], index, 3)
            self.assertEqual(share, partition.deal(shuffled, ["test:wp5"], index, 3))
            self.assertEqual(share, sorted(share, key=partition.order_key))

    def test_unlisted_heavy_targets_are_not_dealt(self):
        self.assertNotIn("test:gone", partition.deal(keys(6), ["test:gone"], 1, 1))


class Listing(unittest.TestCase):
    def test_names_of_a_terse_listing(self):
        listing = "a::b: test\nsrc/lib.rs - f (line 3): test\nbench_x: bench\n\n2 tests, 1 benchmark\n"
        self.assertEqual(partition.names(listing), ["a::b", "bench_x", "src/lib.rs - f (line 3)"])

    def test_targets_of_cargo_messages(self):
        def artifact(kind, name, executable, test=True):
            return json.dumps({"reason": "compiler-artifact", "executable": executable,
                               "profile": {"test": test}, "target": {"kind": kind, "name": name}})
        messages = "\n".join([
            artifact(["test"], "wp2", "/t/wp2"),
            artifact(["lib", "staticlib", "cdylib"], "webp", "/t/webp"),
            artifact(["example"], "bench", "/t/bench", test=False),
            artifact(["lib"], "webp", None),
            artifact(["test"], "wp10", "/t/wp10"),
            json.dumps({"reason": "build-finished", "success": True}),
            "Compiling not json",
        ])
        self.assertEqual(partition.targets(messages),
                         [("lib", "/t/webp"), ("test:wp10", "/t/wp10"), ("test:wp2", "/t/wp2")])

    def test_selectors(self):
        self.assertEqual(partition.selector("lib"), ["--lib"])
        self.assertEqual(partition.selector("doc"), ["--doc"])
        self.assertEqual(partition.selector("test:wp2_header"), ["--test", "wp2_header"])

    def test_result_lines_count_every_reported_test(self):
        ok = "test result: ok. 5 passed; 0 failed; 1 ignored; 0 measured; 12 filtered out; finished in 0.1s"
        failed = "test result: FAILED. 3 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out; finished in 1s"
        self.assertEqual(sum(map(int, partition.RESULT.search(ok).groups())), 6)
        self.assertEqual(sum(map(int, partition.RESULT.search(failed).groups())), 5)


class Running(unittest.TestCase):
    def run_partitions(self, listed, reported):
        commands = []
        saved = partition.listing, partition.counted, partition.Partition.write
        problems, shares = [], []
        try:
            partition.listing = lambda cargo_args, root, env: listed
            partition.counted = lambda command, root, env: (commands.append(command), (0, reported(command)))[1]
            partition.Partition.write = lambda self: None
            for index in (1, 2, 3):
                shard = partition.Partition(index, 3, ["test:wp3"], ".", "g")
                problems += shard.test("dev", ["--locked"], {})
                shares.append([tuple(entry) for entry in shard.runs["dev"]["selected"]])
        finally:
            partition.listing, partition.counted, partition.Partition.write = saved
        return commands, problems, shares

    def test_targets_run_whole_and_their_counts_are_checked(self):
        listed = items(6)
        commands, problems, shares = self.run_partitions(listed, lambda command: 3)
        self.assertEqual(problems, [])
        self.assertEqual(sorted(entry for share in shares for entry in share), sorted(listed))
        self.assertEqual(sorted(command[3:] for command in commands),
                         sorted([["--doc"], ["--lib"]] + [["--test", f"wp{number}"] for number in range(4)]))
        self.assertTrue(all("--exact" not in command and "--" not in command for command in commands))

    def test_a_count_that_differs_fails(self):
        listed = items(4)
        _, problems, _ = self.run_partitions(listed, lambda command: 2 if "--doc" in command else 3)
        self.assertEqual(problems, ["doc: 3 tests listed, 2 reported"])


class Verify(unittest.TestCase):
    def test_complete_records_pass(self):
        self.assertEqual(partition.verify(records(items(12), 4, heavy=["test:wp1"])), [])

    def test_a_missing_partition_fails(self):
        problems = partition.verify(records(items(12), 4)[:3])
        self.assertTrue(any("partitions [1, 2, 3] of 4" in problem for problem in problems), problems)

    def test_a_partition_twice_fails(self):
        complete = records(items(12), 3)
        self.assertTrue(partition.verify(complete + complete[:1]))

    def test_an_item_selected_twice_or_never_fails(self):
        complete = records(items(6), 2)
        complete[1]["runs"]["dev"]["selected"].append(complete[0]["runs"]["dev"]["selected"][0])
        moved = complete[0]["runs"]["dev"]["selected"].pop(1)
        problems = partition.verify(complete)
        self.assertTrue(any("several partitions" in problem for problem in problems), problems)
        self.assertTrue(any("no partition" in problem and moved[1] in problem for problem in problems), problems)

    def test_different_listings_fail(self):
        complete = records(items(6), 2)
        complete[1]["runs"]["dev"]["items"].pop()
        self.assertTrue(any("listed different tests" in problem for problem in partition.verify(complete)))

    def test_different_commits_and_runs_fail(self):
        complete = records(items(6), 2)
        complete[1]["commit"] = "c1"
        self.assertTrue(any("different commits" in problem for problem in partition.verify(complete)))
        complete = records(items(6), 2)
        complete[1]["runs"]["release"] = complete[1]["runs"].pop("dev")
        self.assertTrue(any("different invocations" in problem for problem in partition.verify(complete)))

    def test_expected_groups(self):
        both = records(items(6), 2, group="linux") + records(items(8), 3, group="darwin")
        self.assertEqual(partition.verify(both, ["darwin", "linux"]), [])
        self.assertTrue(any("no records of the groups ['windows']" in problem
                            for problem in partition.verify(both, ["darwin", "linux", "windows"])))
        self.assertTrue(any("unexpected groups ['linux']" in problem for problem in partition.verify(both, ["darwin"])))

    def test_the_command_line_fails_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            for number, record in enumerate(records(items(6), 2, group="linux")):
                (Path(directory) / f"{number + 1}-of-2.json").write_text(json.dumps(record))
            partition.main(["verify", "--expect", " linux ,", directory])
            with self.assertRaises(SystemExit):
                partition.main(["verify", "--expect", "linux,darwin", directory])
            (Path(directory) / "2-of-2.json").unlink()
            with self.assertRaises(SystemExit):
                partition.main(["verify", directory])

    def test_groups_are_checked_separately(self):
        both = records(items(6), 2, group="linux") + records(items(8), 3, group="darwin")
        self.assertEqual(partition.verify(both), [])
        self.assertTrue(partition.verify(both[:-1]))
        self.assertTrue(partition.verify([]))


if __name__ == "__main__":
    unittest.main()
