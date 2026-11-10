"""The migration comparison admits only checkout paths and unused assertion labels."""
import sys
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(ROOT / 'tools'))
from transpile_root import comparable_profile


class RootProfileComparisonTest(unittest.TestCase):
    def test_assertion_path_and_unused_label(self):
        source = ROOT / 'upstream/libavif/src/alpha.c'
        old = '/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/alpha.c'
        def text(path, label):
            return f'''fn check() {{ '{label}: {{ if ok {{}} else {{ __assert_fail(b"x\\0", b"{path}\\0" as *const u8 as *const core::ffi::c_char, 127, f); }} }} }}'''
        expected = comparable_profile(source, text(old, 'c_12'))
        self.assertEqual(expected, comparable_profile(source, text(source.as_posix(), 'c_13')))
        self.assertNotEqual(expected, comparable_profile(source, text(source.as_posix(), 'c_13').replace('127', '128')))
        self.assertNotEqual(expected, comparable_profile(source, text('/other/alpha.c', 'c_13')))

    def test_path_outside_assertion_argument_is_not_normalized(self):
        source = ROOT / 'upstream/libavif/src/alpha.c'
        old = '/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/alpha.c'
        def text(path):
            return f'fn check() {{ let data = b"{path}\\0"; }}'
        self.assertNotEqual(comparable_profile(source, text(old)),
                            comparable_profile(source, text(source.as_posix())))

    def test_wrapped_path_is_not_normalized(self):
        source = ROOT / 'upstream/libavif/src/alpha.c'
        old = '/home/runner/work/libavif-rs/libavif-rs/upstream/libavif/src/alpha.c'
        def text(path):
            return f'fn check() {{ __assert_fail(b"x\\0", map_path(b"{path}\\0"), 1, f); }}'
        self.assertNotEqual(comparable_profile(source, text(old)),
                            comparable_profile(source, text(source.as_posix())))

    def test_referenced_or_nonassertion_label_is_not_normalized(self):
        source = ROOT / 'upstream/libavif/src/alpha.c'
        for body in ("'{n}: {{ break '{n}; __assert_fail(); }}", "'{n}: {{ do_work(); }}", "'{n}: {{ do_work(); if ok {{ __assert_fail(); }} }}"):
            self.assertNotEqual(comparable_profile(source, body.format(n='c_1')),
                                comparable_profile(source, body.format(n='c_2')))


if __name__ == '__main__':
    unittest.main()
