#!/usr/bin/env python3
"""Protect reuse of actual original Release evidence during policy changes."""
import copy
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from plan_aom_release import source_pair, verify_reused
from project_aom_source_evidence import project_source


class OriginalReleaseReuse(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        self.directory = self.root / 'build/aom-paired-release-test_libaom'
        self.directory.mkdir(parents=True)
        cases = [dict(name='Original.Case', result='completed', failed=False, skipped=False)]
        self.record = dict(identity=dict(profile='release', program='test_libaom', shards=120, filter='*'),
            index=4, valid=True, results={side: dict(exit_code=0, valid_xml=True,
                xml_errors=[], cases=copy.deepcopy(cases)) for side in ('c', 'rust')})
        self.report = self.directory / 'shard-4.json'
        self.write_report()
        for side in ('c', 'rust'):
            (self.directory / f'shard-4-{side}.xml').write_text('<testsuites failures="0" errors="0"><testsuite>'
                '<testcase classname="Original" name="Case" status="run" result="completed"/>'
                '</testsuite></testsuites>')

    def write_report(self):
        self.report.write_text(json.dumps(self.record))

    def test_complete_original_pair_is_reusable(self):
        self.assertEqual(source_pair(self.directory, 'test_libaom', 4), self.record)

    def test_incomplete_cancellation_is_scheduled_again(self):
        self.report.unlink()
        (self.directory / 'shard-4-rust.xml').unlink()
        self.assertIsNone(source_pair(self.directory, 'test_libaom', 4))

    def test_partial_explicit_failure_is_not_hidden_by_rerun(self):
        self.report.unlink()
        (self.directory / 'shard-4-c.xml').write_text('<testsuites failures="1" errors="0"/>')
        with self.assertRaises(ValueError):
            source_pair(self.directory, 'test_libaom', 4)

    def test_partial_log_failure_is_not_hidden_by_rerun(self):
        self.report.unlink()
        (self.directory / 'shard-4-c.log').write_text('[  FAILED  ] Original.Case\n')
        with self.assertRaises(ValueError):
            source_pair(self.directory, 'test_libaom', 4)

    def test_changed_xml_rejects_a_green_report(self):
        path = self.directory / 'shard-4-rust.xml'
        path.write_text(path.read_text().replace('name="Case"', 'name="Different"'))
        with self.assertRaises(ValueError):
            source_pair(self.directory, 'test_libaom', 4)

    def test_debug_pair_cannot_replace_release(self):
        self.record['identity']['profile'] = 'debug'
        self.write_report()
        with self.assertRaises(ValueError):
            source_pair(self.directory, 'test_libaom', 4)

    def test_aborted_original_process_is_not_treated_as_cancellation(self):
        self.record['valid'] = False
        self.record['results']['rust']['exit_code'] = -6
        self.write_report()
        with self.assertRaises(ValueError):
            source_pair(self.directory, 'test_libaom', 4)

    def test_changed_skip_identity_is_rejected(self):
        self.record['results']['rust']['cases'][0]['skipped'] = True
        self.write_report()
        with self.assertRaises(ValueError):
            source_pair(self.directory, 'test_libaom', 4)

    def test_reused_files_must_remain_identical_after_composition(self):
        relative = str(self.report.relative_to(self.root))
        plan = dict(reused_files_sha256={relative: hashlib.sha256(self.report.read_bytes()).hexdigest()})
        verify_reused(plan, self.root)
        self.report.write_text('{}')
        with self.assertRaises(ValueError):
            verify_reused(plan, self.root)

    def test_reused_paths_must_stay_inside_collection(self):
        with self.assertRaises(ValueError):
            verify_reused(dict(reused_files_sha256={'../outside': '0' * 64}), self.root)

    def source_plan(self):
        fixture = dict(files={'original.yuv': dict(sha1='original', size=10)})
        self.record['identity']['verified_fixtures_sha256'] = hashlib.sha256(
            json.dumps(fixture['files'], sort_keys=True, separators=(',', ':')).encode()).hexdigest()
        self.write_report()
        (self.directory / 'fixtures.json').write_text(json.dumps(fixture))
        for side in ('c', 'rust'):
            (self.directory / f'shard-4-{side}.log').write_text('Original passing test log\n')
        return dict(source_run=100, reused_partitions=[dict(program='test_libaom', index=4)],
            reused_files_sha256={str(p.relative_to(self.root)): hashlib.sha256(p.read_bytes()).hexdigest()
                for p in self.directory.glob('shard-4*')})

    def test_partial_originals_remain_separate_from_a_complete_passing_projection(self):
        plan = self.source_plan()
        partial = self.directory / 'shard-5-c.xml'
        partial.write_bytes((self.directory / 'shard-4-c.xml').read_bytes())
        destination = self.root / 'composition'
        report = project_source(self.root / 'build', destination, plan)
        self.assertEqual(report['passing_source_partitions'], 1)
        self.assertTrue(partial.exists())
        self.assertFalse((destination / 'build/aom-paired-release-test_libaom/shard-5-c.xml').exists())
        verify_reused(plan, destination)

    def test_projection_rejects_changed_actual_source_evidence(self):
        plan = self.source_plan()
        (self.directory / 'shard-4-c.log').write_text('Changed log')
        destination = self.root / 'composition'
        with self.assertRaises(ValueError):
            project_source(self.root / 'build', destination, plan)
        self.assertFalse(destination.exists())

    def test_projection_does_not_hide_an_explicit_failure_in_an_unselected_original(self):
        plan = self.source_plan()
        (self.directory / 'shard-5-c.xml').write_text('<testsuites failures="1" errors="0"/>')
        with self.assertRaises(ValueError):
            project_source(self.root / 'build', self.root / 'composition', plan)

    def test_projection_rejects_changed_fixture_inventory(self):
        plan = self.source_plan()
        (self.directory / 'fixtures.json').write_text('{"files": {}}')
        with self.assertRaises(ValueError):
            project_source(self.root / 'build', self.root / 'composition', plan)

    def test_projection_cannot_replace_a_differing_passing_record(self):
        plan = self.source_plan()
        destination = self.root / 'composition'
        target = destination / 'build/aom-paired-release-test_libaom/shard-4.json'
        target.parent.mkdir(parents=True)
        target.write_text('Existing passing evidence')
        with self.assertRaises(ValueError):
            project_source(self.root / 'build', destination, plan)
        self.assertEqual(target.read_text(), 'Existing passing evidence')


if __name__ == '__main__':
    unittest.main()
