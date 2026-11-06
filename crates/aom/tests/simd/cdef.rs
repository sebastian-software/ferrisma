// Supplementary transcription of upstream/aom/test/cdef_test.cc for the Neon
// instantiations: CDEFBlockTest, CDEFBlockHighbdTest, CDEFFindDirTest,
// CDEFFindDirDualTest, CDEFCopyRect8to16Test, and CDEFCopyRect16to16Test.
// Seeds, loop bounds, block sizes, boundaries, strengths, damping, bit depths,
// and the reference function tables follow the original (the high-bitdepth C
// table is cdef_filter_16_0_c for every strength index, as upstream). The
// speed tests are not transcribed. Each test stops at the first mismatch, as
// the original's `!error` loop guards do, and reports the failing parameters.
use core::ffi::c_void;

use super::acm_random::ACMRandom;

// av1/common/cdef_block.h
const CDEF_VBORDER: usize = 2;
const CDEF_HBORDER: usize = 8;
const CDEF_BSTRIDE: usize = 144;
const CDEF_VERY_LARGE: u16 = 0x4000;

// av1/common/enums.h
const BLOCK_4X4: i32 = 0;
const BLOCK_4X8: i32 = 1;
const BLOCK_8X4: i32 = 2;
const BLOCK_8X8: i32 = 3;

const MAX_CDEF_BLOCK: usize = 256;
const K_ITERATIONS: i32 = 100;

type CdefFilterBlockFunc = unsafe extern "C-unwind" fn(
    *mut c_void, i32, *const u16, i32, i32, i32, i32, i32, i32, i32, i32);
type CdefFilterBlockFunctions = [CdefFilterBlockFunc; 4];
type FindDirFunc = unsafe extern "C-unwind" fn(*const u16, i32, *mut i32, i32) -> i32;
type FindDirDualFunc = unsafe extern "C-unwind" fn(
    *const u16, *const u16, i32, *mut i32, *mut i32, i32, *mut i32, *mut i32);
type CopyRect8To16 = unsafe extern "C-unwind" fn(*mut u16, i32, *const u8, i32, i32, i32);
type CopyRect16To16 = unsafe extern "C-unwind" fn(*mut u16, i32, *const u16, i32, i32, i32);

#[repr(C, align(16))]
struct Aligned16<T>(T);

fn clamp(value: i32, low: i32, high: i32) -> i32 {
    if value < low { low } else if value > high { high } else { value }
}

/// test_cdef(); returns the number of SIMD-versus-C block comparisons.
fn test_cdef(bsize: i32, iterations: i32, cdef: CdefFilterBlockFunctions,
             ref_cdef: CdefFilterBlockFunctions, boundary: i32, depth: i32) -> u64 {
    const SIZE: usize = 8;
    const YSIZE: usize = SIZE + 2 * CDEF_VBORDER;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut s = Box::new(Aligned16([0u16; YSIZE * CDEF_BSTRIDE]));
    let mut d = Aligned16([0u16; SIZE * SIZE]);
    let mut ref_d = Aligned16([0u16; SIZE * SIZE]);
    let mut compared = 0u64;

    let block_width = if bsize == BLOCK_8X8 || bsize == BLOCK_8X4 { 8 } else { 4 };
    let block_height = if bsize == BLOCK_8X8 || bsize == BLOCK_4X8 { 8 } else { 4 };
    assert!(depth >= 8);
    let max_pos = (SIZE * SIZE) >> (depth == 8) as usize;
    let b = (boundary != 0) as i32;
    let mut pridamping = 3 + depth - 8;
    while pridamping < 7 - 3 * b + depth - 8 {
        let mut secdamping = 3 + depth - 8;
        while secdamping < 7 - 3 * b + depth - 8 {
            for _count in 0..iterations {
                let mut level = 0;
                while level < (1 << depth) {
                    let mut bits = 1;
                    while bits <= depth {
                        for v in s.0.iter_mut() {
                            *v = clamp((rnd.rand16() as i32 & ((1 << bits) - 1)) + level, 0,
                                       (1 << depth) - 1) as u16;
                        }
                        if boundary != 0 {
                            if boundary & 1 != 0 {
                                // Left
                                for i in 0..YSIZE {
                                    for j in 0..CDEF_HBORDER {
                                        s.0[i * CDEF_BSTRIDE + j] = CDEF_VERY_LARGE;
                                    }
                                }
                            }
                            if boundary & 2 != 0 {
                                // Right
                                for i in 0..YSIZE {
                                    for j in CDEF_HBORDER + SIZE..CDEF_BSTRIDE {
                                        s.0[i * CDEF_BSTRIDE + j] = CDEF_VERY_LARGE;
                                    }
                                }
                            }
                            if boundary & 4 != 0 {
                                // Above
                                for i in 0..CDEF_VBORDER {
                                    for j in 0..CDEF_BSTRIDE {
                                        s.0[i * CDEF_BSTRIDE + j] = CDEF_VERY_LARGE;
                                    }
                                }
                            }
                            if boundary & 8 != 0 {
                                // Below
                                for i in CDEF_VBORDER + SIZE..YSIZE {
                                    for j in 0..CDEF_BSTRIDE {
                                        s.0[i * CDEF_BSTRIDE + j] = CDEF_VERY_LARGE;
                                    }
                                }
                            }
                        }
                        let input = s.0[CDEF_HBORDER + CDEF_VBORDER * CDEF_BSTRIDE..].as_ptr();
                        for dir in 0..8 {
                            let mut pristrength = 0;
                            while pristrength <= 19 << (depth - 8) {
                                if pristrength == 16 {
                                    pristrength = 19;
                                }
                                let mut secstrength = 0;
                                while secstrength <= 4 << (depth - 8) {
                                    if secstrength == 3 << (depth - 8) {
                                        secstrength += 1 << (depth - 8);
                                        continue;
                                    }

                                    let strength_index = ((secstrength == 0) as usize)
                                        | (((pristrength == 0) as usize) << 1);

                                    // SAFETY: `input` points at the 8x8 block inside the
                                    // padded 12-row source with CDEF_BSTRIDE columns, which
                                    // covers every tap offset; both outputs hold 8x8 u16.
                                    unsafe {
                                        ref_cdef[strength_index](
                                            ref_d.0.as_mut_ptr().cast(), SIZE as i32, input,
                                            pristrength, secstrength, dir, pridamping, secdamping,
                                            depth - 8, block_width, block_height);
                                        cdef[strength_index](
                                            d.0.as_mut_ptr().cast(), SIZE as i32, input,
                                            pristrength, secstrength, dir, pridamping, secdamping,
                                            depth - 8, block_width, block_height);
                                    }
                                    compared += 1;
                                    if let Some(pos) = (0..max_pos).find(|&p| ref_d.0[p] != d.0[p]) {
                                        panic!("Error: CDEFBlockTest, SIMD and C mismatch.\n\
                                                First error at {},{} ({} : {})\n\
                                                pristrength: {pristrength}\n\
                                                pridamping: {pridamping}\n\
                                                secstrength: {secstrength}\n\
                                                secdamping: {secdamping}\n\
                                                depth: {depth}\nsize: {bsize}\n\
                                                boundary: {boundary}\ndir: {dir}",
                                               pos % SIZE, pos / SIZE, ref_d.0[pos] as i16,
                                               d.0[pos] as i16);
                                    }
                                    secstrength += 1 << (depth - 8);
                                }
                                pristrength += (1 + 4 * b) << (depth - 8);
                            }
                        }
                        bits += 1 + 3 * b;
                    }
                    level += (2 + 6 * b) << (depth - 8);
                }
            }
            secdamping += 1;
        }
        pridamping += 1;
    }
    compared
}

/// test_finddir(); returns the number of comparisons.
fn test_finddir(finddir: FindDirFunc, ref_finddir: FindDirFunc) -> u64 {
    const SIZE: usize = 8;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut s = Aligned16([0u16; SIZE * SIZE]);
    let mut compared = 0u64;
    let mut ref_var = 0i32;
    let mut var = 0i32;

    let mut depth = 8;
    while depth <= 12 {
        for _count in 0..512 {
            let mut level = 0;
            while level < (1 << depth) {
                for bits in 1..=depth {
                    for v in s.0.iter_mut() {
                        *v = clamp((rnd.rand16() as i32 & ((1 << bits) - 1)) + level, 0,
                                   (1 << depth) - 1) as u16;
                    }
                    // SAFETY: `s` is an 8x8 block with stride 8.
                    let (ref_res, res) = unsafe {
                        (ref_finddir(s.0.as_ptr(), SIZE as i32, &mut ref_var, depth - 8),
                         finddir(s.0.as_ptr(), SIZE as i32, &mut var, depth - 8))
                    };
                    compared += 1;
                    assert!(res == ref_res && var == ref_var,
                            "Error: CDEFFindDirTest, SIMD and C mismatch.\nreturn: {res} : \
                             {ref_res}\nvar: {var} : {ref_var}\ndepth: {depth}");
                }
                level += 1 << (depth - 8);
            }
        }
        depth += 2;
    }
    compared
}

/// test_finddir_dual(); returns the number of comparisons.
fn test_finddir_dual(finddir: FindDirDualFunc, ref_finddir: FindDirDualFunc) -> u64 {
    const SIZE_WD: usize = 16;
    const SIZE_HT: usize = 8;
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut s = Aligned16([0u16; SIZE_HT * SIZE_WD]);
    let mut compared = 0u64;
    let mut ref_var = [0i32; 2];
    let mut ref_dir = [0i32; 2];
    let mut var = [0i32; 2];
    let mut dir = [0i32; 2];

    let mut depth = 8;
    while depth <= 12 {
        for _count in 0..512 {
            let mut level = 0;
            while level < (1 << depth) {
                for bits in 1..=depth {
                    for v in s.0.iter_mut() {
                        *v = clamp((rnd.rand16() as i32 & ((1 << bits) - 1)) + level, 0,
                                   (1 << depth) - 1) as u16;
                    }
                    // SAFETY: two 8x8 blocks side by side in a 16-wide, 8-row buffer.
                    unsafe {
                        let p = s.0.as_ptr();
                        ref_finddir(p, p.add(8), SIZE_WD as i32, &mut ref_var[0], &mut ref_var[1],
                                    depth - 8, &mut ref_dir[0], &mut ref_dir[1]);
                        finddir(p, p.add(8), SIZE_WD as i32, &mut var[0], &mut var[1], depth - 8,
                                &mut dir[0], &mut dir[1]);
                    }
                    compared += 1;
                    for j in 0..2 {
                        assert!(ref_dir[j] == dir[j] && ref_var[j] == var[j],
                                "Error: CDEFFindDirTest, SIMD and C mismatch.\ndirection: {} : \
                                 {}\nvariance: {} : {}\ndepth: {depth}",
                                dir[j], ref_dir[j], var[j], ref_var[j]);
                    }
                }
                level += 1 << (depth - 8);
            }
        }
        depth += 2;
    }
    compared
}

/// CDEFCopyRect8to16Test::test_copy_rect_8_to_16(); returns the number of
/// compared rectangles. `test_func` / `ref_func` are GET_PARAM(0) / (1), which
/// the Neon instantiation binds to the C and Neon functions respectively.
fn test_copy_rect_8_to_16(test_func: CopyRect8To16, ref_func: CopyRect8To16) -> u64 {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut src = vec![0u8; MAX_CDEF_BLOCK * MAX_CDEF_BLOCK];
    let mut ref_dst = vec![0u16; MAX_CDEF_BLOCK * MAX_CDEF_BLOCK];
    let mut test_dst = vec![0u16; MAX_CDEF_BLOCK * MAX_CDEF_BLOCK];
    let stride = MAX_CDEF_BLOCK;
    for k in 0..K_ITERATIONS {
        // This function operates on values of width that are either 4 or a
        // multiple of 8. For height, generate a random value between 1 and 256,
        // making sure it is even.
        let width = if k == 0 { 4 } else { (rnd.rand8() as usize % 32 + 1) * 8 };
        let height = if k == 0 { 4 } else { (rnd.rand8() as usize % 128 + 1) * 2 };
        for i in 0..height {
            for j in 0..width {
                src[i * stride + j] = rnd.rand8();
            }
        }

        // SAFETY: all three buffers are MAX_CDEF_BLOCK x MAX_CDEF_BLOCK.
        unsafe {
            ref_func(ref_dst.as_mut_ptr(), stride as i32, src.as_ptr(), stride as i32,
                     width as i32, height as i32);
            test_func(test_dst.as_mut_ptr(), stride as i32, src.as_ptr(), stride as i32,
                      width as i32, height as i32);
        }

        for i in 0..height {
            for j in 0..width {
                assert_eq!(test_dst[i * stride + j], ref_dst[i * stride + j],
                           "Error: CDEFCopyRect8to16Test, SIMD and C mismatch.\nFirst error at \
                            {i},{j}\nwidth: {width}\nheight: {height}");
            }
        }
    }
    K_ITERATIONS as u64
}

/// CDEFCopyRect16to16Test::test_copy_rect_16_to_16(); see test_copy_rect_8_to_16.
fn test_copy_rect_16_to_16(test_func: CopyRect16To16, ref_func: CopyRect16To16) -> u64 {
    let mut rnd = ACMRandom::with_seed(ACMRandom::deterministic_seed());
    let mut src = vec![0u16; MAX_CDEF_BLOCK * MAX_CDEF_BLOCK];
    let mut ref_dst = vec![0u16; MAX_CDEF_BLOCK * MAX_CDEF_BLOCK];
    let mut test_dst = vec![0u16; MAX_CDEF_BLOCK * MAX_CDEF_BLOCK];
    let stride = MAX_CDEF_BLOCK;
    for k in 0..K_ITERATIONS {
        let width = if k == 0 { 4 } else { (rnd.rand8() as usize % 32 + 1) * 8 };
        let height = if k == 0 { 4 } else { (rnd.rand8() as usize % 128 + 1) * 2 };
        for i in 0..height {
            for j in 0..width {
                src[i * stride + j] = rnd.rand16();
            }
        }

        // SAFETY: all three buffers are MAX_CDEF_BLOCK x MAX_CDEF_BLOCK.
        unsafe {
            ref_func(ref_dst.as_mut_ptr(), stride as i32, src.as_ptr(), stride as i32,
                     width as i32, height as i32);
            test_func(test_dst.as_mut_ptr(), stride as i32, src.as_ptr(), stride as i32,
                      width as i32, height as i32);
        }

        for i in 0..height {
            for j in 0..width {
                assert_eq!(test_dst[i * stride + j], ref_dst[i * stride + j],
                           "Error: CDEFCopyRect16to16Test, SIMD and C mismatch.\nFirst error at \
                            {i},{j}\nwidth: {width}\nheight: {height}");
            }
        }
    }
    K_ITERATIONS as u64
}

extern "C-unwind" {
    fn cdef_filter_8_0_c(dest: *mut c_void, dstride: i32, in_: *const u16, pri_strength: i32,
                         sec_strength: i32, dir: i32, pri_damping: i32, sec_damping: i32,
                         coeff_shift: i32, block_width: i32, block_height: i32);
    fn cdef_filter_8_1_c(dest: *mut c_void, dstride: i32, in_: *const u16, pri_strength: i32,
                         sec_strength: i32, dir: i32, pri_damping: i32, sec_damping: i32,
                         coeff_shift: i32, block_width: i32, block_height: i32);
    fn cdef_filter_8_2_c(dest: *mut c_void, dstride: i32, in_: *const u16, pri_strength: i32,
                         sec_strength: i32, dir: i32, pri_damping: i32, sec_damping: i32,
                         coeff_shift: i32, block_width: i32, block_height: i32);
    fn cdef_filter_8_3_c(dest: *mut c_void, dstride: i32, in_: *const u16, pri_strength: i32,
                         sec_strength: i32, dir: i32, pri_damping: i32, sec_damping: i32,
                         coeff_shift: i32, block_width: i32, block_height: i32);
    fn cdef_filter_16_0_c(dest: *mut c_void, dstride: i32, in_: *const u16, pri_strength: i32,
                          sec_strength: i32, dir: i32, pri_damping: i32, sec_damping: i32,
                          coeff_shift: i32, block_width: i32, block_height: i32);
    fn cdef_find_dir_c(img: *const u16, stride: i32, var: *mut i32, coeff_shift: i32) -> i32;
    fn cdef_find_dir_dual_c(img1: *const u16, img2: *const u16, stride: i32, var1: *mut i32,
                            var2: *mut i32, coeff_shift: i32, out1: *mut i32, out2: *mut i32);
    fn cdef_copy_rect8_8bit_to_16bit_c(dst: *mut u16, dstride: i32, src: *const u8,
                                       sstride: i32, width: i32, height: i32);
    fn cdef_copy_rect8_16bit_to_16bit_c(dst: *mut u16, dstride: i32, src: *const u16,
                                        sstride: i32, width: i32, height: i32);
}

const K_CDEF_FILTER_FUNC_C: CdefFilterBlockFunctions =
    [cdef_filter_8_0_c, cdef_filter_8_1_c, cdef_filter_8_2_c, cdef_filter_8_3_c];

const K_CDEF_FILTER_HIGHBD_FUNC_C: CdefFilterBlockFunctions =
    [cdef_filter_16_0_c, cdef_filter_16_0_c, cdef_filter_16_0_c, cdef_filter_16_0_c];

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::av1__common__arm__cdef_block_neon::*;

    const K_CDEF_FILTER_FUNC_NEON: CdefFilterBlockFunctions =
        [cdef_filter_8_0_neon, cdef_filter_8_1_neon, cdef_filter_8_2_neon, cdef_filter_8_3_neon];

    const K_CDEF_FILTER_HIGHBD_FUNC_NEON: CdefFilterBlockFunctions = [
        cdef_filter_16_0_neon, cdef_filter_16_1_neon, cdef_filter_16_2_neon,
        cdef_filter_16_3_neon,
    ];

    /// One TEST_P(CDEFBlockTest / CDEFBlockHighbdTest, ...) per boundary
    /// (::testing::Range(0, 16)) and depth for a given block size.
    fn block_test(cdef: CdefFilterBlockFunctions, ref_cdef: CdefFilterBlockFunctions,
                  bsize: i32, depths: &[i32]) {
        let mut compared = 0;
        for &depth in depths {
            for boundary in 0..16 {
                compared += test_cdef(bsize, 1, cdef, ref_cdef, boundary, depth);
            }
        }
        eprintln!("bsize {bsize} depths {depths:?}: {compared} block comparisons");
    }

    #[test]
    fn cdef_block_4x4() {
        block_test(K_CDEF_FILTER_FUNC_NEON, K_CDEF_FILTER_FUNC_C, BLOCK_4X4, &[8]);
    }

    #[test]
    fn cdef_block_4x8() {
        block_test(K_CDEF_FILTER_FUNC_NEON, K_CDEF_FILTER_FUNC_C, BLOCK_4X8, &[8]);
    }

    #[test]
    fn cdef_block_8x4() {
        block_test(K_CDEF_FILTER_FUNC_NEON, K_CDEF_FILTER_FUNC_C, BLOCK_8X4, &[8]);
    }

    #[test]
    fn cdef_block_8x8() {
        block_test(K_CDEF_FILTER_FUNC_NEON, K_CDEF_FILTER_FUNC_C, BLOCK_8X8, &[8]);
    }

    #[test]
    fn cdef_block_highbd_4x4() {
        block_test(K_CDEF_FILTER_HIGHBD_FUNC_NEON, K_CDEF_FILTER_HIGHBD_FUNC_C, BLOCK_4X4,
                   &[10, 12]);
    }

    #[test]
    fn cdef_block_highbd_4x8() {
        block_test(K_CDEF_FILTER_HIGHBD_FUNC_NEON, K_CDEF_FILTER_HIGHBD_FUNC_C, BLOCK_4X8,
                   &[10, 12]);
    }

    #[test]
    fn cdef_block_highbd_8x4() {
        block_test(K_CDEF_FILTER_HIGHBD_FUNC_NEON, K_CDEF_FILTER_HIGHBD_FUNC_C, BLOCK_8X4,
                   &[10, 12]);
    }

    #[test]
    fn cdef_block_highbd_8x8() {
        block_test(K_CDEF_FILTER_HIGHBD_FUNC_NEON, K_CDEF_FILTER_HIGHBD_FUNC_C, BLOCK_8X8,
                   &[10, 12]);
    }

    #[test]
    fn cdef_find_dir() {
        let n = test_finddir(cdef_find_dir_neon, cdef_find_dir_c);
        eprintln!("cdef_find_dir: {n} comparisons");
    }

    #[test]
    fn cdef_find_dir_dual() {
        let n = test_finddir_dual(cdef_find_dir_dual_neon, cdef_find_dir_dual_c);
        eprintln!("cdef_find_dir_dual: {n} comparisons");
    }

    #[test]
    fn cdef_copy_rect8_8bit_to_16bit() {
        test_copy_rect_8_to_16(cdef_copy_rect8_8bit_to_16bit_c,
                               cdef_copy_rect8_8bit_to_16bit_neon);
    }

    #[test]
    fn cdef_copy_rect8_16bit_to_16bit() {
        test_copy_rect_16_to_16(cdef_copy_rect8_16bit_to_16bit_c,
                                cdef_copy_rect8_16bit_to_16bit_neon);
    }
}
