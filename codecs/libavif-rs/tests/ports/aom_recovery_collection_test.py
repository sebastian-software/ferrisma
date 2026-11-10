#!/usr/bin/env python3
"""Verify artifact import preserves original evidence and rejects conflicts."""
import copy
import hashlib
import json
from pathlib import Path
import sys
import tempfile
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from aom_subshards import merge_subshards
from collect_aom_recovery import install_recovery, verify_original_xml


class RecoveryArtifactImport(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.base = Path(self.temporary.name)
        self.staging, self.root = self.base / 'staging', self.base / 'root'
        self.root.mkdir()
        identity = dict(profile='debug', program='test_libaom', shards=120, filter='*',
                        c_executable_sha256='c' * 64, rust_executable_sha256='d' * 64)
        parts, evidence = [], []
        for segment in range(2):
            index = 4 + 120 * segment
            relative = Path(f'build/aom-recovery-debug-test_libaom/original-4/segment-{segment}')
            directory = self.staging / relative
            directory.mkdir(parents=True)
            cases = [dict(name=f'Original.Case{segment}', result='completed', failed=False, skipped=False)]
            part = dict(identity=dict(identity, shards=240), index=index, valid=True,
                        results={side: dict(exit_code=0, valid_xml=True, xml_errors=[],
                                           cases=copy.deepcopy(cases)) for side in ('c', 'rust')})
            report = directory / f'shard-{index}.json'
            report.write_text(json.dumps(part, indent=2) + '\n')
            proof = dict(report_path=str(report.relative_to(self.staging)),
                         report_sha256=hashlib.sha256(report.read_bytes()).hexdigest(), xml_paths={}, xml_sha256={})
            for side in ('c', 'rust'):
                xml = directory / f'shard-{index}-{side}.xml'
                xml.write_text(f'<testsuites failures="0" errors="0"><testsuite><testcase classname="Original" name="Case{segment}" status="run" result="completed"/></testsuite></testsuites>')
                proof['xml_paths'][side] = str(xml.relative_to(self.staging))
                proof['xml_sha256'][side] = hashlib.sha256(xml.read_bytes()).hexdigest()
            parts.append(part)
            evidence.append(proof)
        merged = merge_subshards(identity, 4, parts)
        merged['subshard_provenance'] = evidence
        directory = self.staging / 'build/aom-paired-debug-test_libaom'
        directory.mkdir()
        (directory / 'shard-4.json').write_text(json.dumps(merged, indent=2) + '\n')
        contract = dict(source_run=100, profile='debug', indices={'4': dict(job=1,
            annotations=[dict(message='The job has exceeded the maximum execution time of 6h0m0s')])})
        (self.staging / 'build/aom-recovery-source-contract.json').write_text(json.dumps(contract))

    def test_complete_artifact_import_and_actual_xml(self):
        seen = set()
        result = install_recovery(self.staging, self.root, 100, seen)
        self.assertEqual(result['original_indices'], [4])
        self.assertEqual(seen, {('debug', 4)})
        self.assertEqual(verify_original_xml(self.root), 1)

    def test_other_source_run(self):
        with self.assertRaises(ValueError):
            install_recovery(self.staging, self.root, 101, set())
        self.assertFalse((self.root / 'build').exists())

    def test_overlapping_recovery(self):
        with self.assertRaises(ValueError):
            install_recovery(self.staging, self.root, 100, {('debug', 4)})
        self.assertFalse((self.root / 'build').exists())

    def test_passing_original_is_preserved(self):
        target = self.root / 'build/aom-paired-debug-test_libaom/shard-4.json'
        target.parent.mkdir(parents=True)
        target.write_text('{"valid": true}')
        with self.assertRaises(ValueError):
            install_recovery(self.staging, self.root, 100, set())
        self.assertEqual(target.read_text(), '{"valid": true}')

    def test_changed_original_xml_is_rejected_before_import(self):
        xml = next(self.staging.rglob('shard-4-c.xml'))
        xml.write_text(xml.read_text().replace('Case0', 'DifferentCase'))
        with self.assertRaises(ValueError):
            install_recovery(self.staging, self.root, 100, set())
        self.assertFalse((self.root / 'build').exists())

    def test_conflicting_original_reference_is_preserved(self):
        relative = Path('build/aom-native-debug/reference-profile.json')
        for base, value in ((self.staging, 'changed'), (self.root, 'original')):
            target = base / relative
            target.parent.mkdir(parents=True)
            target.write_text(value)
        with self.assertRaises(ValueError):
            install_recovery(self.staging, self.root, 100, set())
        self.assertEqual((self.root / relative).read_text(), 'original')
        self.assertFalse((self.root / 'build/aom-paired-debug-test_libaom/shard-4.json').exists())

    def test_unexpected_artifact_paths_are_rejected(self):
        (self.staging / 'build/unrelated-file').write_text('outside evidence')
        with self.assertRaises(ValueError):
            install_recovery(self.staging, self.root, 100, set())
        self.assertFalse((self.root / 'build').exists())


if __name__ == '__main__':
    unittest.main()
