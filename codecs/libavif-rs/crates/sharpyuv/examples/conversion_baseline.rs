//! A bounded, separate-process baseline for the two existing public boundaries.
//! Run the release binary directly under the host's peak-RSS measurement tool.

use std::hint::black_box;
use std::time::Instant;

use webp::sharpyuv::sharpyuv::{
    sharp_yuv_convert_with_options, SharpYuvOptions, SharpYuvTransferFunctionType,
};
use webp::sharpyuv::sharpyuv_csp::{sharp_yuv_get_conversion_matrix, SharpYuvMatrixType};

fn main() {
    let port = std::env::args().nth(1).expect("pass avif or webp");
    assert!(port == "avif" || port == "webp", "pass avif or webp");
    const WIDTH: usize = 512;
    const HEIGHT: usize = 512;
    const WARMUP: usize = 3;
    const MEASURED: usize = 30;
    let mut input = vec![0_u8; WIDTH * HEIGHT * 3];
    let mut seed = 0x4156_4946_u32;
    for sample in &mut input {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        *sample = (seed >> 24) as u8;
    }
    let mut y = vec![0_u8; WIDTH * HEIGHT];
    let mut u = vec![0_u8; WIDTH.div_ceil(2) * HEIGHT.div_ceil(2)];
    let mut v = u.clone();
    let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(0)).unwrap();
    let safe_options = SharpYuvOptions {
        yuv_matrix: matrix,
        transfer_type: SharpYuvTransferFunctionType(13),
    };
    // SAFETY: Matrix 0 is a live static predefined matrix.
    let raw_matrix = unsafe { sharpyuv::SharpYuvGetConversionMatrix(0) };
    let raw_options = sharpyuv::SharpYuvOptions {
        yuv_matrix: raw_matrix,
        transfer_type: 13,
    };
    let mut elapsed = std::time::Duration::ZERO;
    for iteration in 0..(WARMUP + MEASURED) {
        let started = Instant::now();
        if port == "avif" {
            // SAFETY: The input contains all interleaved RGB rows, the output
            // planes are distinct and large enough, and the matrix is static.
            let success = unsafe {
                let rgb = input.as_ptr();
                sharpyuv::SharpYuvConvertWithOptions(
                    rgb.cast(), rgb.add(1).cast(), rgb.add(2).cast(),
                    3, (WIDTH * 3) as i32, 8,
                    y.as_mut_ptr().cast(), WIDTH as i32,
                    u.as_mut_ptr().cast(), WIDTH.div_ceil(2) as i32,
                    v.as_mut_ptr().cast(), WIDTH.div_ceil(2) as i32,
                    8, WIDTH as i32, HEIGHT as i32, &raw_options,
                )
            };
            assert_eq!(success, 1);
        } else {
            let success = sharp_yuv_convert_with_options(
                &input, 0, &input, 1, &input, 2,
                3, (WIDTH * 3) as i32, 8,
                &mut y, 0, WIDTH as i32,
                &mut u, 0, WIDTH.div_ceil(2) as i32,
                &mut v, 0, WIDTH.div_ceil(2) as i32,
                8, WIDTH as i32, HEIGHT as i32, &safe_options,
            );
            assert!(success);
        }
        black_box((&y, &u, &v));
        if iteration >= WARMUP {
            elapsed += started.elapsed();
        }
    }
    let checksum: u64 = y.iter().chain(&u).chain(&v).map(|&sample| sample as u64).sum();
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    // SAFETY: `getrusage` initializes the supplied rusage on a zero return.
    let result = unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) };
    assert_eq!(result, 0, "getrusage failed");
    // SAFETY: The successful call above initialized the whole structure.
    let usage = unsafe { usage.assume_init() };
    let peak_rss_bytes = if cfg!(target_os = "macos") {
        usage.ru_maxrss as u64
    } else {
        (usage.ru_maxrss as u64) * 1024
    };
    println!("port={port} size={WIDTH}x{HEIGHT} RGB8/YUV420P8 matrix=WebP transfer=sRGB warmup={WARMUP} measured={MEASURED} total_ms={:.3} mean_ms={:.3} peak_rss_bytes={peak_rss_bytes} checksum={checksum}",
        elapsed.as_secs_f64() * 1000.0,
        elapsed.as_secs_f64() * 1000.0 / MEASURED as f64);
}
