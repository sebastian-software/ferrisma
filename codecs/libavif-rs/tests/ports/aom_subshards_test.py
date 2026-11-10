#!/usr/bin/env python3
"""Reject incomplete or incompatible evidence when recovering original shards."""
import copy
import hashlib
import json
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from aom_subshards import merge_subshards, passing_subshards


class OriginalShardRecovery(unittest.TestCase):
    def setUp(self):
        self.identity = dict(shards=2, profile='debug', filter='*',
                             c_executable_sha256='original', rust_executable_sha256='ported')
        self.cases = [dict(name=f'Original.Case{i}', result='skipped' if i == 3 else 'completed', failed=False,
                           skipped=i == 3) for i in range(7)]
        self.parts = []
        for segment in range(3):
            results = {side: dict(exit_code=0, valid_xml=True, xml_errors=[],
                                 cases=copy.deepcopy(self.cases[segment::3]))
                       for side in ('c', 'rust')}
            self.parts.append(dict(identity=dict(self.identity, shards=6),
                index=1 + 2 * segment, valid=True, results=results))

    def test_exact_partition_and_matching_original_skip(self):
        merged = merge_subshards(self.identity, 1, self.parts)
        merged['subshard_provenance'] = self.proof()
        self.assertEqual(merged['results']['c']['cases'], self.cases)
        self.assertTrue(passing_subshards(merged))

    def test_missing_part(self):
        with self.assertRaises(ValueError):
            merge_subshards(self.identity, 1, self.parts[:-1])

    def test_changed_binary(self):
        self.parts[1]['identity']['rust_executable_sha256'] = 'different'
        with self.assertRaises(ValueError):
            merge_subshards(self.identity, 1, self.parts)

    def test_changed_original_partition(self):
        self.parts[1]['index'] += 1
        with self.assertRaises(ValueError):
            merge_subshards(self.identity, 1, self.parts)

    def test_mismatched_dynamic_skip(self):
        self.parts[0]['results']['rust']['cases'][1]['skipped'] = False
        with self.assertRaises(ValueError):
            merge_subshards(self.identity, 1, self.parts)

    def test_failed_or_aborted_process(self):
        for field, value in [('exit_code', -6), ('valid_xml', False)]:
            parts = copy.deepcopy(self.parts)
            parts[0]['results']['rust'][field] = value
            with self.assertRaises(ValueError):
                merge_subshards(self.identity, 1, parts)

    def test_duplicate_original_case(self):
        for side in ('c', 'rust'):
            self.parts[1]['results'][side]['cases'][0]['name'] = 'Original.Case0'
        with self.assertRaises(ValueError):
            merge_subshards(self.identity, 1, self.parts)

    def test_modified_aggregate(self):
        merged = merge_subshards(self.identity, 1, self.parts)
        merged['subshard_provenance'] = self.proof()
        merged['results']['c']['cases'] = []
        self.assertFalse(passing_subshards(merged))

    def proof(self):
        return [dict(report_sha256=hashlib.sha256((json.dumps(p, indent=2) + '\n').encode()).hexdigest(),
                     xml_sha256=dict(c='a' * 64, rust='b' * 64)) for p in self.parts]

    def test_missing_xml_evidence(self):
        merged = merge_subshards(self.identity, 1, self.parts)
        self.assertFalse(passing_subshards(merged))

    def test_modified_embedded_report(self):
        merged = merge_subshards(self.identity, 1, self.parts)
        merged['subshard_provenance'] = self.proof()
        merged['subshards'][0]['extra_unverified_field'] = 'changed'
        self.assertFalse(passing_subshards(merged))


if __name__ == '__main__':
    unittest.main()
