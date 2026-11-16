"""tools/enable_dav1d_dispatch.py: the edits and their check mode (ADR 0062)."""
from pathlib import Path
import sys
import unittest

sys.path.insert(0, str(Path(__file__).resolve().parents[2] / 'tools'))
import enable_dav1d_dispatch as dispatch  # noqa: E402

CPU = '''#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_init_cpu() {}
#[no_mangle]
pub unsafe extern "C" fn dav1d_set_cpu_flags_mask(mask: core::ffi::c_uint) {
    dav1d_cpu_flags_mask = mask;
}
'''
ITX = '''#[no_mangle]
#[cold]
pub unsafe extern "C" fn dav1d_itx_dsp_init_8bpc(
    c: *mut Dav1dInvTxfmDSPContext,
    mut bpc: core::ffi::c_int,
) {
    (*c).itxfm_add[0][0] = Some(inv_txfm_add_dct_dct_4x4_c as unsafe extern "C" fn()) as itxfm_fn;
    let mut all_simd: core::ffi::c_int = 0 as core::ffi::c_int;
    if all_simd == 0 {
        dav1d_init_last_nonzero_col_from_eob_tables();
    }
}
'''
MAC_ITX = 'crates/dav1d/src/release/b8/itx_tmpl.rs'
LINUX_ITX = 'crates/dav1d/src/linux/release/b8/itx_tmpl.rs'
MAC_CPU = 'crates/dav1d/src/common/cpu.rs'


class DispatchText(unittest.TestCase):
    def test_init_cpu_reads_the_native_flags_in_every_profile(self):
        for rel in [MAC_CPU, 'crates/dav1d/src/linux/release/common/cpu.rs']:
            out = dispatch.dispatch_text(rel, CPU, [])
            self.assertIn('dav1d_cpu_flags = crate::simd::cpu::dav1d_get_cpu_flags_native(); // dav1d-dispatch', out)
            self.assertNotIn('dav1d_init_cpu() {}', out)

    def test_entry_goes_into_its_family_bitdepth_and_target_only(self):
        out = dispatch.dispatch_text(MAC_ITX, ITX, ['itx_8bpc_arm'])
        call = ('    crate::simd::arm::itx::itx_dsp_init_arm_8bpc(c.cast(), bpc, &raw mut all_simd); '
                '// dav1d-dispatch\n')
        self.assertIn('= 0 as core::ffi::c_int;\n' + call + '    if all_simd == 0 {', out)
        # Not into the x86-64 profiles, not into other bit depths.
        self.assertEqual(dispatch.dispatch_text(LINUX_ITX, ITX, ['itx_8bpc_arm']), ITX)
        self.assertEqual(dispatch.dispatch_text('crates/dav1d/src/release/b16/itx_tmpl.rs', ITX,
                                                ['itx_8bpc_arm']), ITX)
        linux = dispatch.dispatch_text(LINUX_ITX, ITX, ['itx_8bpc_x86'])
        self.assertIn('crate::simd::x86::itx::itx_dsp_init_x86_8bpc(', linux)

    def test_idempotent_and_reversible(self):
        once = dispatch.dispatch_text(MAC_ITX, ITX, ['itx_8bpc_arm'])
        self.assertEqual(dispatch.dispatch_text(MAC_ITX, once, ['itx_8bpc_arm']), once)
        # Removing the entry restores the translation exactly.
        self.assertEqual(dispatch.dispatch_text(MAC_ITX, once, []), ITX)
        cpu = dispatch.dispatch_text(MAC_CPU, CPU, [])
        self.assertEqual(dispatch.strip(cpu), CPU)

    def test_a_hand_edit_is_not_reproduced(self):
        once = dispatch.dispatch_text(MAC_ITX, ITX, ['itx_8bpc_arm'])
        edited = once.replace('&raw mut all_simd); // dav1d-dispatch', '&raw mut all_simd);')
        self.assertNotEqual(dispatch.dispatch_text(MAC_ITX, edited, ['itx_8bpc_arm']), edited)

    def test_unknown_entries_and_missing_anchors_fail(self):
        with self.assertRaises(SystemExit):
            dispatch.parse_entry('mc_8bpc_arm')
        with self.assertRaises(SystemExit):
            dispatch.parse_entry('itx_10bpc_arm')
        with self.assertRaises(SystemExit):
            dispatch.dispatch_text(MAC_ITX, ITX.replace('let mut all_simd', 'let all_simd'), ['itx_8bpc_arm'])

    def test_committed_tree_matches(self):
        # The same comparison as --check, without the digest part.
        for rel, _ in dispatch.units():
            text = (dispatch.ROOT / rel).read_text()
            self.assertEqual(dispatch.dispatch_text(rel, text), text, rel)


if __name__ == '__main__':
    unittest.main()
