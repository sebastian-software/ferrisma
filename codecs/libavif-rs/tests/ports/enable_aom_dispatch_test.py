"""The check mode of tools/enable_aom_dispatch.py (ADR 0059, amendment of 2026-11-15)."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
import enable_aom_dispatch as dispatch  # noqa: E402

FTYPE = 'unsafe extern "C-unwind" fn(*const uint8_t, core::ffi::c_int) -> int64_t'
HEADER = '''RTCD_EXTERN int64_t (*foo)(const uint8_t *a, int n);

static void setup_rtcd_internal(void)
{
    int flags = x86_simd_caps();

    (void)flags;

    foo = foo_c;
    if (flags & HAS_SSE2) foo = foo_sse2;
    if (flags & HAS_AVX2) foo = foo_avx2;
}
'''
UNIT = f'''extern "C-unwind" {{
    fn foo_c(a: *const uint8_t, n: core::ffi::c_int) -> int64_t;
    fn foo_sse2(a: *const uint8_t, n: core::ffi::c_int) -> int64_t;
    fn foo_avx2(a: *const uint8_t, n: core::ffi::c_int) -> int64_t;
}}
#[no_mangle]
pub static mut foo: Option<{FTYPE}> = None;
unsafe extern "C-unwind" fn setup_rtcd_internal() {{
    let mut flags: core::ffi::c_int = x86_simd_caps();
    foo = Some(foo_c as {FTYPE}) as Option<{FTYPE}>;
    if flags & HAS_SSE2 != 0 {{
        foo = Some(foo_sse2 as {FTYPE}) as Option<{FTYPE}>;
    }}
    if flags & HAS_AVX2 != 0 {{
        foo = Some(foo_avx2 as {FTYPE}) as Option<{FTYPE}>;
    }}
}}
'''
# The rustfmt-wrapped form C2Rust emitted for the functions dispatched at the freeze.
WRAPPED = '''    foo = Some(
        foo_c
            as unsafe extern "C-unwind" fn(
                *const uint8_t,
                core::ffi::c_int,
            ) -> int64_t,
    )
        as Option<
            unsafe extern "C-unwind" fn(
                *const uint8_t,
                core::ffi::c_int,
            ) -> int64_t,
        >;
'''


def problems(unit, header=HEADER):
    return dispatch.rtcd_unit_problems('unit.rs', unit, header, lambda name: FTYPE)


class RtcdUnitCheckTest(unittest.TestCase):
    def test_the_tool_output_passes(self):
        self.assertEqual(problems(UNIT), [])

    def test_the_wrapped_c2rust_form_passes(self):
        line = f'    foo = Some(foo_c as {FTYPE}) as Option<{FTYPE}>;\n'
        self.assertEqual(problems(UNIT.replace(line, WRAPPED)), [])

    def test_a_missing_variant_assignment_fails(self):
        block = f'    if flags & HAS_AVX2 != 0 {{\n        foo = Some(foo_avx2 as {FTYPE}) as Option<{FTYPE}>;\n    }}\n'
        self.assertIn(block, UNIT)
        self.assertTrue(problems(UNIT.replace(block, '')))

    def test_a_variant_the_header_does_not_select_fails(self):
        header = HEADER.replace('    if (flags & HAS_AVX2) foo = foo_avx2;\n', '')
        self.assertTrue(problems(UNIT, header))

    def test_an_extra_pointer_global_fails(self):
        extra = UNIT.replace('unsafe extern "C-unwind" fn setup_rtcd_internal',
                             f'#[no_mangle]\npub static mut bar: Option<{FTYPE}> = None;\n'
                             'unsafe extern "C-unwind" fn setup_rtcd_internal')
        self.assertTrue(any('bar' in p for p in problems(extra)))

    def test_a_pointer_global_of_another_type_fails(self):
        other = UNIT.replace(f'pub static mut foo: Option<{FTYPE}>', 'pub static mut foo: Option<unsafe extern "C-unwind" fn() -> ()>')
        self.assertTrue(any('another type' in p for p in problems(other)))

    def test_a_missing_declaration_fails(self):
        missing = UNIT.replace('    fn foo_sse2(a: *const uint8_t, n: core::ffi::c_int) -> int64_t;\n', '')
        self.assertTrue(any('foo_sse2' in p for p in problems(missing)))

    def test_a_missing_variant_is_shown_as_a_diff(self):
        block = f'    if flags & HAS_AVX2 != 0 {{\n        foo = Some(foo_avx2 as {FTYPE}) as Option<{FTYPE}>;\n    }}\n'
        report = '\n'.join(problems(UNIT.replace(block, '')))
        self.assertIn(f'+if flags & HAS_AVX2 != 0 {{ foo = Some(foo_avx2 as {FTYPE}) as Option<{FTYPE}>; }}', report)
        self.assertIn('(tool output)', report)

    def test_the_wrapped_c2rust_form_shows_no_diff_lines(self):
        line = f'    foo = Some(foo_c as {FTYPE}) as Option<{FTYPE}>;\n'
        body = UNIT.split('fn setup_rtcd_internal() {\n', 1)[1].rsplit('}\n', 1)[0]
        expected = dispatch.expected_setup(HEADER, lambda name: FTYPE)
        self.assertEqual(dispatch.setup_diff('unit.rs', body.replace(line, WRAPPED), expected), '')

    def test_a_pointer_global_of_another_type_shows_both_types(self):
        other = UNIT.replace(f'pub static mut foo: Option<{FTYPE}>', 'pub static mut foo: Option<unsafe extern "C-unwind" fn() -> ()>')
        report = '\n'.join(problems(other))
        self.assertIn('committed:   Option<unsafe extern "C-unwind" fn() -> ()>', report)
        self.assertIn(f'tool output: Option<{FTYPE}>', report)

    def test_an_unknown_header_statement_is_refused(self):
        with self.assertRaises(SystemExit):
            dispatch.header_setup(HEADER.replace('    (void)flags;\n', '    flags = 0;\n'))


class CallerCheckTest(unittest.TestCase):
    """The check flags a caller exactly when the tool would rewrite it."""
    PARAMS, RET = 'a: *const uint8_t, n: core::ffi::c_int', '-> int64_t'

    def rewrite(self, text):
        return dispatch.edit_caller(text, 'int64_t g(void) { return foo(0, 0); }', 'foo',
                                    self.PARAMS, self.RET, [], 'caller.rs')

    def test_a_dispatched_caller_is_left_alone(self):
        text = ('extern "C-unwind" {\n    static mut foo: Option<' + FTYPE + '>;\n}\n'
                'unsafe fn g() -> int64_t { foo.expect("non-null function pointer")(0 as *const uint8_t, 0) }\n')
        self.assertEqual(self.rewrite(text), text)

    def test_a_direct_call_of_the_c_function_is_found(self):
        text = ('extern "C-unwind" {\n    fn foo_c(a: *const uint8_t, n: core::ffi::c_int) -> int64_t;\n}\n'
                'unsafe fn g() -> int64_t { foo_c(0 as *const uint8_t, 0) }\n')
        self.assertNotEqual(self.rewrite(text), text)


if __name__ == '__main__':
    unittest.main()
