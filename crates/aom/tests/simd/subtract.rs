// Supplementary transcription of upstream/aom/test/subtract_test.cc
// (AV1SubtractBlockTest and AV1HBDSubtractBlockTest CheckResult) for the Neon
// instantiations. The original compares against src - pred; this also
// compares against the translated `_c` function.
use super::acm_random::ACMRandom;

type SubtractFunc = unsafe extern "C-unwind" fn(i32, i32, *mut i16, isize, *const u8, isize,
                                                 *const u8, isize);

extern "C-unwind" {
    fn aom_subtract_block_c(rows: i32, cols: i32, diff: *mut i16, diff_stride: isize,
                            src: *const u8, src_stride: isize, pred: *const u8, pred_stride: isize);
    fn aom_highbd_subtract_block_c(rows: i32, cols: i32, diff: *mut i16, diff_stride: isize,
                                   src: *const u8, src_stride: isize, pred: *const u8,
                                   pred_stride: isize);
}

// kValidBlockSize as (width, height).
const K_VALID_BLOCK_SIZE: [(i32, i32); 16] = [
    (4, 4), (4, 8), (8, 4), (8, 8), (8, 16), (16, 8), (16, 16), (16, 32), (32, 16), (32, 32),
    (32, 64), (64, 32), (64, 64), (64, 128), (128, 64), (128, 128)];

const MAX_BLOCK_SIZE: usize = 128 * 128;

fn check_result(width: i32, height: i32, bit_depth: Option<i32>, func: SubtractFunc,
                ref_func: SubtractFunc) {
    let mut rnd = ACMRandom::new();
    let mut src8 = vec![0u8; MAX_BLOCK_SIZE];
    let mut pred8 = vec![0u8; MAX_BLOCK_SIZE];
    let mut src16 = vec![0u16; MAX_BLOCK_SIZE];
    let mut pred16 = vec![0u16; MAX_BLOCK_SIZE];
    let mut diff = vec![0i16; MAX_BLOCK_SIZE];
    let mut diff_ref = vec![0i16; MAX_BLOCK_SIZE];
    let (w, h) = (width as usize, height as usize);
    for i in 0..100 {
        // FillInputs
        if let Some(bd) = bit_depth {
            let mask = (1u16 << bd).wrapping_sub(1);
            for k in 0..MAX_BLOCK_SIZE {
                src16[k] = rnd.rand16() & mask;
                pred16[k] = rnd.rand16() & mask;
            }
        } else {
            for k in 0..MAX_BLOCK_SIZE {
                src8[k] = rnd.rand8();
                pred8[k] = rnd.rand8();
            }
        }
        let (src, pred) = if bit_depth.is_some() {
            // CONVERT_TO_BYTEPTR
            (((src16.as_ptr() as usize) >> 1) as *const u8, ((pred16.as_ptr() as usize) >> 1) as *const u8)
        } else {
            (src8.as_ptr(), pred8.as_ptr())
        };
        // SAFETY: every buffer holds 128x128 samples and the strides equal the width.
        unsafe {
            func(height, width, diff.as_mut_ptr(), width as isize, src, width as isize, pred,
                 width as isize);
            ref_func(height, width, diff_ref.as_mut_ptr(), width as isize, src, width as isize,
                     pred, width as isize);
        }
        for r in 0..h {
            for c in 0..w {
                let k = r * w + c;
                let expected = if bit_depth.is_some() {
                    (src16[k] as i32 - pred16[k] as i32) as i16
                } else {
                    (src8[k] as i32 - pred8[k] as i32) as i16
                };
                assert_eq!(diff[k], expected, "r = {r}, c = {c}, test: {i}");
                assert_eq!(diff[k], diff_ref[k], "C mismatch r = {r}, c = {c}, test: {i}");
            }
        }
    }
}

#[cfg(target_arch = "aarch64")]
mod neon {
    use super::*;
    use aom::simd::aom_dsp__arm__subtract_neon::*;

    #[test]
    fn subtract_block() {
        for (w, h) in K_VALID_BLOCK_SIZE {
            check_result(w, h, None, aom_subtract_block_neon, aom_subtract_block_c);
        }
    }

    #[test]
    fn highbd_subtract_block() {
        for (w, h) in K_VALID_BLOCK_SIZE {
            check_result(w, h, Some(12), aom_highbd_subtract_block_neon,
                         aom_highbd_subtract_block_c);
        }
    }
}
