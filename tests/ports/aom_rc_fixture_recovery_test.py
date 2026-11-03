#!/usr/bin/env python3
"""Do not mistake a codec failure or changed original case for absent test data."""
import copy
import json
from pathlib import Path
import sys
import tempfile
import unittest
import xml.etree.ElementTree as ET

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools'))
from check_aom_rc_fixture_recovery import validate_failure, verify_repaired_pair


class MissingVideoRecovery(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.directory = Path(self.temporary.name)
        failed = json.loads((ROOT / 'docs/aom-rate-control-fixture-failure.json').read_text())['failed_case_names']
        self.names = failed + [f'Original.Other/{i}' for i in range(22)]
        self.identity = dict(program='test_aom_rc', profile='release', shards=1, filter='*',
                             c_executable_sha256='frozen-c', rust_executable_sha256='frozen-rust')
        self.record = dict(identity=copy.deepcopy(self.identity), index=0, valid=False, results={})
        for side in ('c', 'rust'):
            document = ET.Element('testsuites', tests='38', failures='16', errors='0')
            suite = ET.SubElement(document, 'testsuite')
            cases = []
            for i, name in enumerate(self.names):
                classname, method = name.split('.', 1)
                case = ET.SubElement(suite, 'testcase', classname=classname, name=method,
                                     status='run', result='completed')
                if i < 16:
                    video = 'niklas_640_480_30.yuv' if i < 14 else 'hantro_collage_w352h288.yuv'
                    for message in [
                        'upstream/aom/test/yuv_video_source.h:47\nExpected: (input_file_) != (nullptr), actual: NULL vs (nullptr)\nInput file open failed. Filename: ' + video + '\n',
                        "upstream/aom/test/encode_test_driver.cc:194\nExpected: video->Begin() doesn't generate new fatal failures in the current thread.\n  Actual: it does.\n",
                        "upstream/aom/test/ratectrl_rtc_test.cc:210\nExpected: RunLoop(&video) doesn't generate new fatal failures in the current thread.\n  Actual: it does.\n"]:
                        ET.SubElement(case, 'failure', message=message)
                cases.append(dict(name=name, result='completed', failed=i < 16, skipped=False))
            ET.ElementTree(document).write(self.directory / f'shard-0-{side}.xml')
            self.record['results'][side] = dict(exit_code=1, valid_xml=False, xml_errors=[], cases=cases)
        self.write_report()

    def write_report(self):
        (self.directory / 'shard-0.json').write_text(json.dumps(self.record))

    def reject(self):
        with self.assertRaises((ValueError, FileNotFoundError)):
            validate_failure(self.directory, self.names, self.identity)

    def test_exact_missing_video_failures_are_classified(self):
        self.assertEqual(validate_failure(self.directory, self.names, self.identity), self.record)

    def test_additional_codec_assertion_is_not_hidden(self):
        p = self.directory / 'shard-0-rust.xml'
        tree = ET.parse(p)
        ET.SubElement(next(tree.getroot().iter('testcase')), 'failure', message='Reconstruction checksum differs')
        tree.write(p)
        self.reject()

    def test_another_file_failure_is_not_accepted(self):
        p = self.directory / 'shard-0-c.xml'
        p.write_text(p.read_text().replace('niklas_640_480_30.yuv', 'different.yuv'))
        self.reject()

    def test_missing_c_oracle_is_rejected(self):
        (self.directory / 'shard-0-c.xml').unlink()
        self.reject()

    def test_changed_frozen_identity_is_rejected(self):
        self.record['identity']['rust_executable_sha256'] = 'rebuilt'
        self.write_report()
        self.reject()

    def test_changed_original_case_order_is_rejected(self):
        self.names[0], self.names[1] = self.names[1], self.names[0]
        self.reject()

    def test_report_must_match_actual_xml(self):
        self.record['results']['rust']['cases'][0]['failed'] = False
        self.write_report()
        self.reject()

    def test_crash_is_not_a_missing_video_failure(self):
        self.record['results']['rust']['exit_code'] = -6
        self.write_report()
        self.reject()

    def test_passing_report_cannot_masquerade_as_failed_evidence(self):
        self.record['valid'] = True
        self.write_report()
        self.reject()

    def make_passing_pair(self):
        self.record['valid'] = True
        for side in ('c', 'rust'):
            result = self.record['results'][side]
            result.update(exit_code=0, valid_xml=True)
            for case in result['cases']:
                case['failed'] = False
            p = self.directory / f'shard-0-{side}.xml'
            tree = ET.parse(p)
            tree.getroot().set('failures', '0')
            for case in tree.getroot().iter('testcase'):
                for failure in case.findall('failure'):
                    case.remove(failure)
            tree.write(p)
        self.write_report()

    def test_repaired_pair_requires_actual_unchanged_c_and_rust_xml(self):
        self.make_passing_pair()
        self.assertEqual(verify_repaired_pair(self.directory, self.names, self.identity), self.record)

    def test_repaired_pair_rejects_changed_binary_identity(self):
        self.make_passing_pair()
        self.record['identity']['rust_executable_sha256'] = 'other'
        self.write_report()
        with self.assertRaises(ValueError):
            verify_repaired_pair(self.directory, self.names, self.identity)

    def test_repaired_pair_rejects_changed_actual_xml(self):
        self.make_passing_pair()
        p = self.directory / 'shard-0-rust.xml'
        p.write_text(p.read_text().replace('OneLayer/0', 'Different/0'))
        with self.assertRaises(ValueError):
            verify_repaired_pair(self.directory, self.names, self.identity)


if __name__ == '__main__':
    unittest.main()
