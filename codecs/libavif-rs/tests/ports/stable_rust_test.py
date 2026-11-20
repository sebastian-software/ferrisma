"""Regression checks for the stable-Rust rewrite the generators apply (ADR 0060)."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
from stable_rust import stable_text
from transpile_dav1d import empty_register_barrier

ORDER = '{core::intrinsics::AtomicOrdering::SeqCst}'
GENERATED = f'''extern "C" {{
    pub type __sFILEX;
    fn fflush(_: *mut FILE) -> core::ffi::c_int;
}}
pub unsafe fn f(p: *mut core::ffi::c_int) -> core::ffi::c_int {{
    ::core::intrinsics::atomic_store::<_, {ORDER}, false>(p, 1 as core::ffi::c_int);
    let (v, ok) = ::core::intrinsics::atomic_cxchg::<_, {ORDER}, {ORDER}>(p, 1, 2);
    ::core::intrinsics::atomic_xadd::<_, _, {{core::intrinsics::AtomicOrdering::Relaxed}}>(p, 1)
}}
'''
COMMITTED = '''extern "C" {
    fn fflush(_: *mut FILE) -> core::ffi::c_int;
}
#[repr(C)]
pub struct __sFILEX([u8; 0], core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>);
pub unsafe fn f(p: *mut core::ffi::c_int) -> core::ffi::c_int {
    crate::platform::atomic_store_seqcst(p, 1 as core::ffi::c_int);
    let (v, ok) = crate::platform::atomic_cxchg_seqcst_seqcst(p, 1, 2);
    crate::platform::atomic_xadd_relaxed(p, 1)
}
'''

# The translated xgetbv() of aom_ports/x86.h and libwebp's cpu.c (ADR-0044).
XGETBV_GENERATED = """use core::arch::asm;
#[inline]
unsafe extern "C-unwind" fn xgetbv() -> uint64_t {
    let ecx: uint32_t = 0 as uint32_t;
    let mut eax: uint32_t = 0;
    let mut edx: uint32_t = 0;
    asm!(
        ".byte 0x0f, 0x01, 0xd0\\n", "\\n", lateout("ax") eax, lateout("dx") edx,
        inlateout("cx") ecx => _, options(preserves_flags, att_syntax)
    );
    return (edx as uint64_t) << 32 as core::ffi::c_int | eax as uint64_t;
}
"""

# dav1d's empty x86-64 register barrier in get_dc_sign_ctx (ADR-0044): C2Rust's
# form and the assembler-comment form the port carried before 2026-11-16.
BARRIER_C2RUST = """use core::arch::asm;
    let mut s: core::ffi::c_int = 0;
    asm!(
        "\\n", inlateout(reg) mask, inlateout(reg) mul, options(preserves_flags, pure,
        readonly, att_syntax)
    );
    let mut current_block_63: u64;
"""
BARRIER_COMMENT = BARRIER_C2RUST.replace('"\\n"', '"/* {0} {1} */"')
BARRIER_DELETED = """    let mut s: core::ffi::c_int = 0;
    // dav1d's empty x86-64 register barrier on `mask` and `mul` emits no
    // instruction and is not translated (engine ADR-0044).
    let mut current_block_63: u64;
"""


class StableTextTest(unittest.TestCase):
    def test_the_register_barrier_is_deleted(self):
        rel = 'crates/dav1d/src/linux/b8/recon_tmpl.rs'
        for generated in (BARRIER_C2RUST, BARRIER_COMMENT):
            self.assertEqual(stable_text(rel, generated), BARRIER_DELETED)
        self.assertEqual(stable_text(rel, BARRIER_DELETED), BARRIER_DELETED)
        text, count = empty_register_barrier(BARRIER_C2RUST)
        self.assertEqual(count, 1)
        self.assertNotIn('asm!', text)

    def test_xgetbv_becomes_the_intrinsic(self):
        rel = 'crates/aom/src/libaom/aom_scale/aom_scale_rtcd.rs'
        committed = stable_text(rel, XGETBV_GENERATED)
        self.assertNotIn('asm', committed.replace('_xgetbv', ''))
        self.assertIn('#[target_feature(enable = "xsave")]', committed)
        self.assertIn('core::arch::x86_64::_xgetbv(xcr)', committed)
        self.assertIn('    return read_xcr(ecx);\n}\n', committed)
        self.assertEqual(stable_text(rel, committed), committed)

    def test_generator_output_becomes_the_committed_form(self):
        self.assertEqual(stable_text('crates/dav1d/src/common/example.rs', GENERATED), COMMITTED)

    def test_the_rewrite_is_idempotent(self):
        self.assertEqual(stable_text('crates/dav1d/src/common/example.rs', COMMITTED), COMMITTED)


if __name__ == '__main__':
    unittest.main()
