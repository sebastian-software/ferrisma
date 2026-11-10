"""Regression checks for the generated libaom nonlocal-error mapping."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from aom_jumps import NONLOCAL_CALL, map_scopes
from import_aom_linux import merge_jump_corrections


class JumpSpellingTest(unittest.TestCase):
    def test_glibc_and_canonical_spellings_produce_the_same_scope(self):
        for comparison in ['==', '!=']:
            with self.subTest(comparison=comparison):
                fixture = '''extern "C" {
    fn setjmp(target: *mut Buffer) -> core::ffi::c_int;
    fn longjmp(target: *mut Buffer, value: core::ffi::c_int) -> !;
}
pub unsafe extern "C" fn example(target: *mut Buffer) -> core::ffi::c_int {
    if setjmp(target) COMPARISON 0 as core::ffi::c_int {
        return 7;
    }
    longjmp(target, 1);
    return 9;
}
'''.replace('COMPARISON', comparison)
                canonical, changes = map_scopes(fixture)
                glibc, glibc_changes = map_scopes(fixture.replace('setjmp(', '_setjmp('))
                self.assertEqual(canonical, glibc)
                self.assertIsNone(NONLOCAL_CALL.search(glibc))
                self.assertEqual(changes['scoped-nonlocal-error-targets'], 1)
                self.assertEqual(changes['typed-nonlocal-error-transfer'], 1)
                self.assertEqual(glibc_changes.pop('glibc-setjmp-aliases'), 2)
                self.assertEqual(changes, glibc_changes)

    def test_unreviewed_signal_saving_variant_is_rejected(self):
        with self.assertRaisesRegex(SystemExit, 'Unmapped libc nonlocal error transfer'):
            map_scopes('pub unsafe fn example() { __sigsetjmp(target, 1); }')

    def test_import_preserves_generation_stage_order_before_compatibility(self):
        original = {'raw-c-mutable-address': 1, 'unwinding-codec-and-callback-abi': 2,
                    'system-and-vmaf-c-abi-declarations': 4,
                    'current-rust-variadic-argument-method': 1}
        changes = merge_jump_corrections(original, {'glibc-setjmp-aliases': 2,
                                         'scoped-nonlocal-error-targets': 1}, 1)
        self.assertEqual(list(changes), ['raw-c-mutable-address', 'glibc-setjmp-aliases',
            'scoped-nonlocal-error-targets', 'unwinding-codec-and-callback-abi',
            'system-and-vmaf-c-abi-declarations', 'current-rust-variadic-argument-method'])
        self.assertEqual(changes['system-and-vmaf-c-abi-declarations'], 3)
        self.assertEqual(original['system-and-vmaf-c-abi-declarations'], 4)


if __name__ == '__main__':
    unittest.main()
