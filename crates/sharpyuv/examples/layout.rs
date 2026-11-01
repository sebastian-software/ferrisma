use core::mem::{align_of, offset_of, size_of};
use ::sharpyuv::*;

fn main() {
    let rgb = [127u8, 64, 201];
    let (mut y, mut u, mut v) = ([0u8; 1], [0u8; 1], [0u8; 1]);
    // SAFETY: The three-channel input and disjoint output arrays remain live,
    // have byte alignment and exactly the declared one-pixel dimensions. The
    // original static matrix is valid and options are initialized before use.
    let code = unsafe {
        let matrix = SharpYuvGetConversionMatrix(kSharpYuvMatrixWebp);
        let mut options = SharpYuvOptions { yuv_matrix: core::ptr::null(), transfer_type: 0 };
        assert_eq!(SharpYuvOptionsInit(matrix, &mut options), 1);
        SharpYuvConvertWithOptions(rgb.as_ptr().cast(), rgb.as_ptr().add(1).cast(), rgb.as_ptr().add(2).cast(),
            3, 3, 8, y.as_mut_ptr().cast(), 1, u.as_mut_ptr().cast(), 1, v.as_mut_ptr().cast(), 1,
            8, 1, 1, &options)
    };
    assert_eq!(code, 1);
    println!("{{\"matrix\":{:?},\"options\":{:?},\"space\":{:?},\"mutex\":{:?},\"callback\":{:?},\"pixel\":{:?}}}",
        [size_of::<SharpYuvConversionMatrix>(), align_of::<SharpYuvConversionMatrix>(), offset_of!(SharpYuvConversionMatrix, rgb_to_y), offset_of!(SharpYuvConversionMatrix, rgb_to_u), offset_of!(SharpYuvConversionMatrix, rgb_to_v)],
        [size_of::<SharpYuvOptions>(), align_of::<SharpYuvOptions>(), offset_of!(SharpYuvOptions, yuv_matrix), offset_of!(SharpYuvOptions, transfer_type)],
        [size_of::<SharpYuvColorSpace>(), align_of::<SharpYuvColorSpace>(), offset_of!(SharpYuvColorSpace, kr), offset_of!(SharpYuvColorSpace, kb), offset_of!(SharpYuvColorSpace, bit_depth), offset_of!(SharpYuvColorSpace, range)],
        [size_of::<libc::pthread_mutex_t>(), align_of::<libc::pthread_mutex_t>()],
        [size_of::<Option<unsafe extern "C" fn()>>(), size_of::<*const ()>()],
        [code, y[0] as i32, u[0] as i32, v[0] as i32]);
}
