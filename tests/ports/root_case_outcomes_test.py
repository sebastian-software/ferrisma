#!/usr/bin/env python3
"""Reject hidden skip differences and incomplete results in retained CTest logs."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools'))
from compare_results import case_outcomes, results


class CompleteCaseOutcomes(unittest.TestCase):
    def parse(self, body, count=2):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'LastTest.log'
            path.write_bytes(b'1/1 Testing: original\n[==========] Running ' + str(count).encode()
                             + b' tests from 1 test suite.\n' + body)
            return case_outcomes(path, dict(original=dict(gtest_cases=[count])))['original']

    body = (b'[ RUN      ] Original.First\n[       OK ] Original.First (3 ms)\n'
            b'[ RUN      ] Original.Second\n[  SKIPPED ] Original.Second (0 ms)\n'
            b'[  SKIPPED ] 1 test, listed below:\n[  SKIPPED ] Original.Second\n')

    def test_reads_each_case_status_without_counting_summary_lines(self):
        self.assertEqual(self.parse(self.body), [dict(name='Original.First', status='OK'),
                                                dict(name='Original.Second', status='SKIPPED')])

    def test_a_skip_change_hidden_by_truncated_xml_is_detectable(self):
        changed = self.body.replace(b'[  SKIPPED ] Original.Second (0 ms)', b'[       OK ] Original.Second (8 ms)')
        self.assertNotEqual(self.parse(self.body), self.parse(changed))

    def test_incomplete_case_result_is_rejected(self):
        with self.assertRaises(SystemExit):
            self.parse(self.body.replace(b'[  SKIPPED ] Original.Second (0 ms)\n', b''))

    def test_next_case_cannot_hide_a_missing_result(self):
        with self.assertRaises(SystemExit):
            self.parse(self.body.replace(b'[       OK ] Original.First (3 ms)\n', b''))

    def test_failed_case_is_rejected_even_with_successful_xml(self):
        with self.assertRaises(SystemExit):
            self.parse(self.body.replace(b'[       OK ]', b'[  FAILED ]'))

    def test_changed_or_duplicate_terminal_names_are_rejected(self):
        for changed in [self.body.replace(b'[       OK ] Original.First', b'[       OK ] Original.Other'),
                        self.body + b'[       OK ] Original.Second (0 ms)\n']:
            with self.subTest(changed=changed), self.assertRaises(SystemExit):
                self.parse(changed)

    def test_non_google_test_registration_has_no_invented_cases(self):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'LastTest.log'
            path.write_bytes(b'1/1 Testing: original\nOriginal shell integration output\n')
            self.assertEqual(case_outcomes(path, dict(original=dict(gtest_cases=[]))), dict(original=[]))

    def xml_records(self, output):
        with tempfile.TemporaryDirectory() as directory:
            path = Path(directory) / 'results.xml'
            path.write_text('<testsuite><testcase name="original" status="run"><system-out>'
                            + output + '</system-out></testcase></testsuite>')
            return results(path)['original']

    xml_output = ('[==========] Running 2 tests from 1 test suite.\n'
                  '[ RUN      ] Original.Second\n[  SKIPPED ] Original.Second (2 ms)\n'
                  '[  SKIPPED ] 1 test, listed below:\n[  SKIPPED ] Original.Second\n')

    def test_xml_skip_comparison_ignores_elapsed_time(self):
        record = self.xml_records(self.xml_output)
        self.assertEqual(record['gtest_skipped'],
                         ['Original.Second', '1 test, listed below:', 'Original.Second'])
        self.assertEqual(record, self.xml_records(self.xml_output.replace('(2 ms)', '(1 ms)')))

    def test_xml_skip_comparison_still_detects_changed_skips(self):
        changed = self.xml_output.replace('Original.Second', 'Original.Other')
        self.assertNotEqual(self.xml_records(self.xml_output), self.xml_records(changed))
        removed = self.xml_output.replace('[  SKIPPED ] Original.Second (2 ms)\n', '')
        self.assertNotEqual(self.xml_records(self.xml_output), self.xml_records(removed))

    def test_passing_truncated_xml_cannot_hide_different_skips_from_cli(self):
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            xml = work / 'same.xml'
            xml.write_text('<testsuite><testcase name="original" status="run"><system-out>'
                           '[==========] Running 2 tests from 1 test suite.\n[truncated output]'
                           '</system-out></testcase></testsuite>')
            for implementation, body in [('c', self.body), ('rust', self.body.replace(
                    b'[  SKIPPED ] Original.Second (0 ms)', b'[       OK ] Original.Second (8 ms)'))]:
                (work / implementation).write_bytes(b'1/1 Testing: original\n'
                    b'[==========] Running 2 tests from 1 test suite.\n' + body)
            result = subprocess.run([sys.executable, ROOT / 'tools/compare_results.py',
                '--baseline', xml, '--port', xml, '--baseline-log', work / 'c',
                '--port-log', work / 'rust', '--report', work / 'rejected.json'],
                text=True, capture_output=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('dynamic skips differ', result.stderr)
            self.assertFalse((work / 'rejected.json').exists())


if __name__ == '__main__':
    unittest.main()
