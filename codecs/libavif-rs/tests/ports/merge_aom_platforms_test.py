"""Checks for the per-target merge of the one-source libaom trees."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from merge_aom_platforms import merge_unit, normalized, project, split_unit, verify_unit

HEAD = '''/*
 * License header.
 */

// Mechanically translated from example.c.
'''

MACOS = HEAD + '''extern "C-unwind" {
    fn shared_decl(x: i32) -> i32;
    fn kernel_neon(x: i32) -> i32;
    fn changed(x: i32) -> i32;
}
pub type __builtin_va_list = *mut core::ffi::c_char;
/// A record on both sides.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Shared {
    pub a: i32,
}
#[repr(C)]
pub struct MacOnly([u8; 0]);
pub static mut TABLE: [i32; 2] = [1, 2]; // trailing comment
pub unsafe extern "C-unwind" fn wrapped(a: i32, b: i32) -> i32 {
    return shared_decl(a) + b;
}
pub unsafe extern "C-unwind" fn caller() -> i32 {
    return kernel_neon(1);
}
'''

LINUX = HEAD + '''extern "C-unwind" {
    fn shared_decl(x: i32) -> i32;
    fn changed(
        x: i32,
        y: i32,
    ) -> i32;
    fn kernel_sse2(x: i32) -> i32;
}
pub type __builtin_va_list = [__va_list_tag; 1];
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __va_list_tag {
    pub gp_offset: core::ffi::c_uint,
}
/// A record on both sides.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct Shared {
    pub a: i32,
}
pub static mut TABLE: [i32; 2] = [1, 2]; // trailing comment
pub unsafe extern "C-unwind" fn wrapped(
    a: i32,
    b: i32,
) -> i32 {
    return shared_decl(a) + b;
}
pub unsafe extern "C-unwind" fn caller() -> i32 {
    return kernel_sse2(1);
}
'''


class SplitTest(unittest.TestCase):
    def test_items_keep_source_order_and_cover_the_text(self):
        items, tail = split_unit(MACOS)
        self.assertEqual([i.kind for i in items],
                         ['head', 'extern', 'type', 'struct', 'struct', 'static', 'fn', 'fn'])
        self.assertEqual(''.join(i.text for i in items) + tail, MACOS)
        block = items[1]
        self.assertEqual([c.name for c in block.children], ['shared_decl', 'kernel_neon', 'changed'])
        self.assertTrue(items[3].text.startswith('/// A record'))
        self.assertTrue(items[5].text.endswith('// trailing comment\n'))

    def test_statics_with_braces_end_at_the_semicolon(self):
        text = 'static X: S = S { a: 1 };\nfn f() {}\nconst C: i32 = { 3 };\nmacro_rules! m { () => {}; }\n'
        items, tail = split_unit(text)
        self.assertEqual([i.key for i in items], [('static', 'X'), ('fn', 'f'), ('const', 'C'), ('macro', 'm')])
        self.assertEqual(tail, '')

    def test_wrapping_is_whitespace_but_a_one_element_tuple_is_not(self):
        self.assertEqual(normalized('fn f(a: i32, b: i32) {}'), normalized('fn f(\n    a: i32,\n    b: i32,\n) {}'))
        self.assertNotEqual(normalized('type T = (i32,);'), normalized('type T = (i32);'))


class MergeTest(unittest.TestCase):
    def setUp(self):
        self.merged, self.stats = merge_unit(MACOS, LINUX)

    def test_shared_pair_and_single_items(self):
        merged = self.merged
        self.assertEqual(merged.count('License header'), 1)
        self.assertEqual(merged.count('pub struct Shared'), 1)
        self.assertEqual(merged.count('fn shared_decl'), 1)
        self.assertIn('    #[cfg(target_arch = "aarch64")]\n    fn kernel_neon', merged)
        self.assertIn('    #[cfg(target_arch = "x86_64")]\n    fn kernel_sse2', merged)
        self.assertIn('#[cfg(target_arch = "aarch64")]\npub type __builtin_va_list = *mut', merged)
        self.assertIn('#[cfg(target_arch = "x86_64")]\npub type __builtin_va_list = [', merged)
        self.assertIn('#[cfg(target_arch = "x86_64")]\n#[derive(Copy, Clone)]\n#[repr(C)]\npub struct __va_list_tag', merged)
        self.assertIn('#[cfg(target_arch = "aarch64")]\n#[repr(C)]\npub struct MacOnly', merged)
        # Whitespace-only difference: one copy, macOS text.
        self.assertIn('fn wrapped(a: i32, b: i32) -> i32 {', merged)
        self.assertEqual(merged.count('fn wrapped'), 1)
        items = self.stats.items
        self.assertEqual(items['pairs'], 3)  # changed, __builtin_va_list, caller
        self.assertEqual(items['shared_whitespace'], 1)

    def test_extern_block_split_keeps_one_block(self):
        self.assertEqual(self.merged.count('extern "C-unwind" {'), 1)
        block = self.merged[self.merged.index('extern "C-unwind" {'):self.merged.index('\n}\n') + 3]
        self.assertIn('fn shared_decl(x: i32) -> i32;\n', block)
        self.assertIn('    #[cfg(target_arch = "aarch64")]\n    fn changed(x: i32) -> i32;', block)

    def test_projection_round_trip(self):
        self.assertEqual(project(self.merged, 'aarch64'), MACOS)
        self.assertEqual(verify_unit(MACOS, project(self.merged, 'aarch64')), ('exact', 0))
        self.assertEqual(verify_unit(LINUX, project(self.merged, 'x86_64')), ('whitespace', 1))
        self.assertIsNone(verify_unit(LINUX.replace('b: i32,\n)', 'c: i32,\n)'), project(self.merged, 'x86_64')))

    def test_one_sided_extern_block_is_attributed_whole(self):
        macos = 'extern "C" {\n    fn only_mac();\n}\npub fn f() {}\n'
        merged, stats = merge_unit(macos, 'pub fn f() {}\n')
        self.assertTrue(merged.startswith('#[cfg(target_arch = "aarch64")]\nextern "C" {'))
        self.assertEqual(project(merged, 'x86_64'), 'pub fn f() {}\n')
        self.assertEqual(project(merged, 'aarch64'), macos)

    def test_identical_units_pass_through(self):
        merged, stats = merge_unit(MACOS, MACOS)
        self.assertEqual(merged, MACOS)
        self.assertEqual(stats.items['pairs'] + stats.items['aarch64_only'] + stats.items['x86_64_only'], 0)


if __name__ == '__main__':
    unittest.main()
