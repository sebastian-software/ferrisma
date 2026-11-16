"""tools/enable_dav1d_dispatch.py: the edits and their check mode (ADR 0062)."""
from pathlib import Path
import re
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
MSAC = '''extern "C" {
    fn dav1d_msac_decode_symbol_adapt_c(s: *mut MsacContext, cdf: *mut uint16_t, n: size_t) -> c_uint;
}
#[no_mangle]
pub unsafe extern "C" fn dav1d_msac_decode_bool_adapt_c(s: *mut MsacContext, cdf: *mut uint16_t) -> c_uint {
    let bit = dav1d_msac_decode_bool_c(s, *cdf as c_uint);
    bit
}
pub unsafe extern "C" fn dav1d_msac_decode_hi_tok_c(s: *mut MsacContext, cdf: *mut uint16_t) -> c_uint {
    let a = dav1d_msac_decode_symbol_adapt_c(s, cdf, 3 as size_t);
    let b = dav1d_msac_decode_symbol_adapt_c(s, cdf, 3 as size_t);
    let c = dav1d_msac_decode_symbol_adapt_c(s, cdf, 3 as size_t);
    let d = dav1d_msac_decode_symbol_adapt_c(s, cdf, 3 as size_t);
    assert(b"dav1d_msac_decode_symbol_adapt_c\\0");
    a + b + c + d + dav1d_msac_decode_bool_equi_c(s)
}
'''
MAC_MSAC = 'crates/dav1d/src/release/common/msac.rs'
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

    def test_msac_renames_calls_only_and_reverses(self):
        out = dispatch.dispatch_text(MAC_MSAC, MSAC, ['msac_arm'])
        self.assertEqual(out.count('crate::simd::arm::msac::dav1d_msac_decode_symbol_adapt4(s, cdf, 3'), 4)
        self.assertIn('crate::simd::arm::msac::dav1d_msac_decode_bool(s, *cdf', out)
        self.assertIn('crate::simd::arm::msac::dav1d_msac_decode_bool_equi(s)', out)
        # Definitions, declarations and strings keep their C names.
        self.assertIn('fn dav1d_msac_decode_symbol_adapt_c(s: *mut', out)
        self.assertIn('fn dav1d_msac_decode_hi_tok_c(', out)
        self.assertIn('b"dav1d_msac_decode_symbol_adapt_c', out)
        self.assertEqual(dispatch.dispatch_text(MAC_MSAC, out, []), MSAC)
        self.assertEqual(dispatch.dispatch_text(MAC_MSAC, out, ['msac_arm']), out)
        # Not into the x86-64 profiles.
        self.assertEqual(dispatch.dispatch_text('crates/dav1d/src/linux/release/common/msac.rs', MSAC,
                                                ['msac_arm']), MSAC)
        # A count that differs from SYMBOL_ADAPT fails.
        with self.assertRaises(SystemExit):
            dispatch.dispatch_text(MAC_MSAC, MSAC.replace('let d = dav1d_msac_decode_symbol_adapt_c', 'let d = ('),
                                   ['msac_arm'])
        with self.assertRaises(SystemExit):
            dispatch.parse_entry('msac_8bpc_arm')
        with self.assertRaises(SystemExit):
            dispatch.parse_entry('itx_arm')

    def test_symbol_adapt_sizes_are_the_original_macros(self):
        source = dispatch.ROOT / 'upstream/dav1d/src'
        if not (source / 'decode.c').exists():
            raise unittest.SkipTest('upstream/dav1d is not checked out')
        call = re.compile(r'dav1d_msac_decode_symbol_adapt(4|8|16|##ns)\(')
        decode = [int(m[1]) for m in call.finditer((source / 'decode.c').read_text())]
        self.assertEqual(decode, dispatch.SYMBOL_ADAPT['common/decode.rs'])
        recon = (source / 'recon_tmpl.c').read_text()
        # case_sz(sz, bin, ns, is_1d) expands once per invocation, and the
        # body of DECODE_COEFS_CLASS once per tx class.
        cases = [int(ns) for ns in re.findall(r'^\s*case_sz\(\s*\d+,\s*\d+,\s*(\d+),', recon, re.M)]
        define = recon.index('#define DECODE_COEFS_CLASS')
        body_end = recon.index('const uint16_t *scan;', define)
        body = [int(m[1]) for m in call.finditer(recon[define:body_end])]
        classes = len(re.findall(r'^\s*DECODE_COEFS_CLASS\(TX_CLASS_\w+\);', recon, re.M))
        expanded = []
        for match in call.finditer(recon):
            if match[1] == '##ns':
                expanded += cases
            elif define <= match.start() < body_end:
                if match.start() == define + recon[define:body_end].index(match[0]):
                    expanded += body * classes  # at the first call of the body
            else:
                expanded.append(int(match[1]))
        for unit in ['b8/recon_tmpl.rs', 'b16/recon_tmpl.rs']:
            self.assertEqual(expanded, dispatch.SYMBOL_ADAPT[unit], unit)
        hi_tok = (source / 'msac.c').read_text().split('dav1d_msac_decode_hi_tok_c(', 1)[1].split('\n}', 1)[0]
        self.assertEqual([int(m[1]) for m in call.finditer(hi_tok)], dispatch.SYMBOL_ADAPT['common/msac.rs'])

    def test_entry_without_anchor_goes_at_the_end_of_the_init(self):
        lpf = ('pub unsafe extern "C" fn dav1d_loop_filter_dsp_init_8bpc(\n'
               '    c: *mut Dav1dLoopFilterDSPContext,\n) {\n'
               '    (*c).loop_filter_sb[1][1] = Some(f) as loopfilter_sb_fn;\n}\n')
        rel = 'crates/dav1d/src/release/b8/loopfilter_tmpl.rs'
        out = dispatch.dispatch_text(rel, lpf, ['loopfilter_8bpc_arm'])
        self.assertTrue(out.endswith(
            'as loopfilter_sb_fn;\n    crate::simd::arm::loopfilter::loop_filter_dsp_init_arm_8bpc(c.cast()); '
            '// dav1d-dispatch\n}\n'))
        self.assertEqual(dispatch.dispatch_text(rel, out, []), lpf)

    def test_committed_tree_matches(self):
        # The same comparison as --check, without the digest part.
        for rel, _ in dispatch.units():
            text = (dispatch.ROOT / rel).read_text()
            self.assertEqual(dispatch.dispatch_text(rel, text), text, rel)


if __name__ == '__main__':
    unittest.main()
