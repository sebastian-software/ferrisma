"""Checks for the single C va_list spelling of the one-source libaom translation."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from aom_va_list import normalize

MACOS = '''// va_list in a comment stays
pub type __builtin_va_list = *mut core::ffi::c_char;
pub type va_list<'a> = core::ffi::VaList<'a>;
pub type size_t = usize;
pub type control_fn_t = Option<
    unsafe extern "C-unwind" fn(*mut aom_codec_alg_priv_t, va_list) -> aom_codec_err_t,
>;
extern "C" {
    fn vsnprintf(
        buffer: *mut core::ffi::c_char,
        size: size_t,
        format: *const core::ffi::c_char,
        arguments: va_list,
    ) -> core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_error(mut fmt: *const core::ffi::c_char, mut ap: va_list) {
    let mut buffer: [core::ffi::c_char; 8] = [0; 8];
    vsnprintf(buffer.as_mut_ptr(), 8 as size_t, fmt, ap);
}
pub unsafe extern "C-unwind" fn aom_set_error(mut fmt: *const core::ffi::c_char, mut args: ...) {
    let ap = args.clone();
    set_error(fmt, ap);
    let name = b"va_list\\0";
}
'''

LINUX = '''// va_list in a comment stays
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
    pub fp_offset: core::ffi::c_uint,
    pub overflow_arg_area: *mut core::ffi::c_void,
    pub reg_save_area: *mut core::ffi::c_void,
}
pub type size_t = usize;
pub type control_fn_t = Option<
    unsafe extern "C-unwind" fn(
        *mut aom_codec_alg_priv_t,
        core::ffi::VaList<'_>,
    ) -> aom_codec_err_t,
>;
extern "C" {
    fn vsnprintf(
        buffer: *mut core::ffi::c_char,
        size: size_t,
        format: *const core::ffi::c_char,
        arguments: ::core::ffi::VaList,
    ) -> core::ffi::c_int;
}
unsafe extern "C-unwind" fn set_error(
    mut fmt: *const core::ffi::c_char,
    mut ap: ::core::ffi::VaList,
) {
    let mut buffer: [core::ffi::c_char; 8] = [0; 8];
    vsnprintf(
        buffer.as_mut_ptr(),
        8 as size_t,
        fmt,
        ap,
    );
}
pub unsafe extern "C-unwind" fn aom_set_error(mut fmt: *const core::ffi::c_char, mut args: ...) {
    let mut ap: ::core::ffi::VaList;
    ap = args.clone();
    set_error(fmt, ap);
    let name = b"va_list\\0";
}
'''


class VaListSpellingTest(unittest.TestCase):
    def test_both_targets_normalize_to_one_text(self):
        macos, macos_changes = normalize(MACOS)
        linux, linux_changes = normalize(LINUX)
        self.assertEqual(macos, linux)
        self.assertGreater(macos_changes, 0)
        self.assertGreater(linux_changes, 0)

    def test_spelling_records_and_copy_point(self):
        text, _ = normalize(LINUX)
        for gone in ['__builtin_va_list', '__va_list_tag', 'pub type va_list', '::core::ffi::VaList']:
            self.assertNotIn(gone, text)
        self.assertIn("let mut ap: core::ffi::VaList<'_> = args.clone();\n    set_error(fmt, ap);", text)
        self.assertIn("arguments: core::ffi::VaList<'_>,", text)
        self.assertIn("vsnprintf(buffer.as_mut_ptr(), 8 as size_t, fmt, ap);", text)
        # comments and literals are not code
        self.assertIn('// va_list in a comment stays', text)
        self.assertIn('b"va_list\\0"', text)

    def test_idempotent_and_deterministic(self):
        for source in [MACOS, LINUX]:
            once, _ = normalize(source)
            self.assertEqual(normalize(once), (once, 0))
            self.assertEqual(normalize(source)[0], once)

    def test_referenced_records_are_kept(self):
        source = LINUX + 'pub static mut LIST: __builtin_va_list = [];\n'
        text, _ = normalize(source)
        self.assertIn('pub type __builtin_va_list = [__va_list_tag; 1];', text)
        self.assertIn('pub struct __va_list_tag {', text)

    def test_split_start_with_code_between_is_not_merged(self):
        source = '''unsafe extern "C-unwind" fn f(mut args: ...) {
    let mut ap: ::core::ffi::VaList;
    g();
    ap = args.clone();
}
'''
        text, _ = normalize(source)
        self.assertIn("    let mut ap: core::ffi::VaList<'_>;\n    g();\n    ap = args.clone();\n", text)

    def test_unit_without_va_list_is_unchanged(self):
        source = 'pub type size_t = usize;\nfn f(\n    a: size_t,\n    b: size_t,\n) {}\n'
        self.assertEqual(normalize(source), (source, 0))


if __name__ == '__main__':
    unittest.main()
