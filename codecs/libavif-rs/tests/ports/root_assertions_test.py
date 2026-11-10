#!/usr/bin/env python3
"""Check single evaluation and retained failure paths in generated glibc assertions."""
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools'))
from root_assertions import single_evaluation


ASSERTION = '''if evaluate() != 0 {} else {
    __assert_fail(b"evaluate()\\0".as_ptr(), b"fixture.c\\0".as_ptr(), 7, b"probe\\0".as_ptr());
}'''


def duplicate(first=ASSERTION, second=ASSERTION):
    return first + "\n'c_123: {\n" + second + '\n};\n'


class OriginalAssertionEvaluation(unittest.TestCase):
    def test_side_effect_runs_once_and_original_failure_still_executes(self):
        fixed, count, _ = single_evaluation(duplicate())
        self.assertEqual(count, 1)
        with tempfile.TemporaryDirectory() as directory:
            work = Path(directory)
            source = '''use std::sync::atomic::{AtomicU32, Ordering};
static CALLS: AtomicU32 = AtomicU32::new(0);
fn evaluate() -> i32 { CALLS.fetch_add(1, Ordering::SeqCst); std::env::args().nth(1).is_none() as i32 }
fn __assert_fail(_: *const u8, _: *const u8, _: u32, _: *const u8) -> ! {
    assert_eq!(CALLS.load(Ordering::SeqCst), 1);
    std::process::exit(73);
}
fn main() {
''' + fixed + '\nassert_eq!(CALLS.load(Ordering::SeqCst), 1);\n}\n'
            (work / 'probe.rs').write_text(source)
            subprocess.run(['rustc', '--edition=2021', work / 'probe.rs', '-o', work / 'probe'],
                           check=True, stdout=subprocess.PIPE, stderr=subprocess.PIPE)
            subprocess.run([work / 'probe'], check=True)
            failed = subprocess.run([work / 'probe', 'false'])
            self.assertEqual(failed.returncode, 73)

    def test_different_conditions_are_rejected(self):
        with self.assertRaises(SystemExit):
            single_evaluation(duplicate(second=ASSERTION.replace('evaluate() != 0', 'evaluate() == 0')))

    def test_different_original_diagnostics_are_rejected(self):
        with self.assertRaises(SystemExit):
            single_evaluation(duplicate(second=ASSERTION.replace('fixture.c', 'different.c')))

    def test_two_real_assertions_remain_two_runtime_evaluations(self):
        fixed, count, _ = single_evaluation(duplicate() + duplicate().replace('c_123', 'c_456'))
        self.assertEqual(count, 2)
        self.assertEqual(fixed.count('if evaluate()'), 2)

    def test_unpaired_failure_call_is_not_silently_changed(self):
        with self.assertRaises(SystemExit):
            single_evaluation(ASSERTION)

    def test_unconditional_original_assertion_keeps_one_failure_call(self):
        body = '__assert_fail(b"0\\0".as_ptr(), b"fixture.c\\0".as_ptr(), 9, b"probe\\0".as_ptr());'
        fixed, count, _ = single_evaluation(duplicate(body, body))
        self.assertEqual(count, 1)
        self.assertEqual(fixed.count('__assert_fail('), 1)

    def test_optional_function_call_comma_does_not_change_the_mapping(self):
        fixed, count, _ = single_evaluation(duplicate(second=ASSERTION.replace('b"probe\\0".as_ptr());', 'b"probe\\0".as_ptr(),);')))
        self.assertEqual(count, 1)
        self.assertEqual(fixed.count('__assert_fail('), 1)

    def test_character_and_string_delimiters_do_not_hide_an_assertion(self):
        text = 'let bracket = \'}\'; let message = "not a } delimiter";\n' + duplicate()
        _, count, _ = single_evaluation(text)
        self.assertEqual(count, 1)


if __name__ == '__main__':
    unittest.main()
