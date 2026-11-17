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


def items(count, targets=("lib", "test:a", "test:b", "doc")):
    return [(targets[n % len(targets)], f"t{n:03}") for n in range(count)]


def records(all_items, count, heavy=(), commit="c0", group="g", label="dev"):
    return [{"group": group, "partition": [index, count], "commit": commit,
             "runs": {label: {"items": sorted(map(list, all_items)),
                              "selected": sorted(map(list, partition.deal(all_items, list(heavy), index, count)))}}}
            for index in range(1, count + 1)]


class Parse(unittest.TestCase):
    def test_accepts_k_of_n(self):
        self.assertEqual(partition.parse("2/5"), (2, 5))
        self.assertEqual(partition.parse("1/1"), (1, 1))

    def test_rejects_others(self):
        for value in ("0/3", "4/3", "1/0", "a/b", "1", "1/2/3", " 1/2"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                partition.parse(value)

    def test_items(self):
        self.assertEqual(partition.item("test:wp13_tools::a::b"), ("test:wp13_tools", "a::b"))
        self.assertEqual(partition.item("doc::src/lib.rs - x (line 3)"), ("doc", "src/lib.rs - x (line 3)"))
        for value in ("wp13_tools::a", "test:wp13::", "test:a b::c", "lib"):
            with self.subTest(value=value), self.assertRaises(ValueError):
                partition.item(value)


class Deal(unittest.TestCase):
    def test_every_item_in_exactly_one_partition(self):
        generator = random.Random(7)
        for count in range(1, 8):
            for size in (0, 1, count - 1, count, 3 * count + 1, 97):
                all_items = items(max(size, 0))
                heavy = generator.sample(all_items, min(len(all_items), 5))
                for rotation in (0, 1, count + 2):
                    dealt = [partition.deal(all_items, heavy, index, count, rotation) for index in range(1, count + 1)]
                    flat = [entry for share in dealt for entry in share]
                    with self.subTest(count=count, size=size, rotation=rotation):
                        self.assertEqual(sorted(flat), sorted(all_items))
                        self.assertEqual(len(flat), len(set(flat)))

    def test_heavy_first_in_alternating_direction(self):
        all_items = items(12)
        heavy = [all_items[11], all_items[10], all_items[9], all_items[8], all_items[7], all_items[6]]
        shares = [partition.deal(all_items, heavy, index, 3) for index in (1, 2, 3)]
        # Heavy positions 0, 1, 2 go to 1, 2, 3; positions 3, 4, 5 to 3, 2, 1.
        self.assertEqual(shares[0][:2], [heavy[0], heavy[5]])
        self.assertEqual(shares[1][:2], [heavy[1], heavy[4]])
        self.assertEqual(shares[2][:2], [heavy[2], heavy[3]])

    def test_each_further_run_starts_one_partition_later(self):
        all_items = items(20)
        heavy = [all_items[13]]
        owners = [[index for index in (1, 2, 3, 4) if heavy[0] in partition.deal(all_items, heavy, index, 4, rotation)]
                  for rotation in range(6)]
        self.assertEqual(owners, [[1], [2], [3], [4], [1], [2]])

    def test_does_not_depend_on_listing_order(self):
        all_items = items(40)
        shuffled = list(all_items)
        random.Random(3).shuffle(shuffled)
        heavy = [all_items[5], all_items[30]]
        for index in (1, 2, 3, 4):
            self.assertEqual(partition.deal(all_items, heavy, index, 4), partition.deal(shuffled, heavy, index, 4))

    def test_unlisted_heavy_items_are_not_dealt(self):
        all_items = items(6)
        self.assertNotIn(("test:x", "gone"), partition.deal(all_items, [("test:x", "gone")], 1, 1))


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


class Verify(unittest.TestCase):
    def test_complete_records_pass(self):
        self.assertEqual(partition.verify(records(items(30), 4, heavy=items(30)[:3])), [])

    def test_a_missing_partition_fails(self):
        problems = partition.verify(records(items(30), 4)[:3])
        self.assertTrue(any("partitions [1, 2, 3] of 4" in problem for problem in problems), problems)

    def test_a_partition_twice_fails(self):
        complete = records(items(30), 3)
        self.assertTrue(partition.verify(complete + complete[:1]))

    def test_an_item_selected_twice_or_never_fails(self):
        complete = records(items(10), 2)
        complete[1]["runs"]["dev"]["selected"].append(complete[0]["runs"]["dev"]["selected"][0])
        moved = complete[0]["runs"]["dev"]["selected"].pop(1)
        problems = partition.verify(complete)
        self.assertTrue(any("several partitions" in problem for problem in problems), problems)
        self.assertTrue(any("no partition" in problem and moved[1] in problem for problem in problems), problems)

    def test_different_listings_fail(self):
        complete = records(items(10), 2)
        complete[1]["runs"]["dev"]["items"].pop()
        self.assertTrue(any("listed different tests" in problem for problem in partition.verify(complete)))

    def test_different_commits_and_runs_fail(self):
        complete = records(items(10), 2)
        complete[1]["commit"] = "c1"
        self.assertTrue(any("different commits" in problem for problem in partition.verify(complete)))
        complete = records(items(10), 2)
        complete[1]["runs"]["release"] = complete[1]["runs"].pop("dev")
        self.assertTrue(any("different invocations" in problem for problem in partition.verify(complete)))

    def test_expected_groups(self):
        both = records(items(10), 2, group="linux") + records(items(12), 3, group="darwin")
        self.assertEqual(partition.verify(both, ["darwin", "linux"]), [])
        self.assertTrue(any("no records of the groups ['windows']" in problem
                            for problem in partition.verify(both, ["darwin", "linux", "windows"])))
        self.assertTrue(any("unexpected groups ['linux']" in problem for problem in partition.verify(both, ["darwin"])))

    def test_the_command_line_fails_closed(self):
        with tempfile.TemporaryDirectory() as directory:
            for number, record in enumerate(records(items(10), 2, group="linux")):
                (Path(directory) / f"{number + 1}-of-2.json").write_text(json.dumps(record))
            partition.main(["verify", "--expect", " linux ,", directory])
            with self.assertRaises(SystemExit):
                partition.main(["verify", "--expect", "linux,darwin", directory])
            (Path(directory) / "2-of-2.json").unlink()
            with self.assertRaises(SystemExit):
                partition.main(["verify", directory])

    def test_groups_are_checked_separately(self):
        both = records(items(10), 2, group="linux") + records(items(12), 3, group="darwin")
        self.assertEqual(partition.verify(both), [])
        self.assertTrue(partition.verify(both[:-1]))
        self.assertTrue(partition.verify([]))


if __name__ == "__main__":
    unittest.main()
