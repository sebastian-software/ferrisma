//! Matched RGB16/YUV10 signed-layout benchmark for the two public boundaries.
//! Run one boundary per fresh process; all pointer advances remain allocated.

use std::hint::black_box;
use std::time::Instant;

use webp::sharpyuv::sharpyuv::{
    SharpYuvOptions, SharpYuvTransferFunctionType, sharp_yuv_convert_with_options,
};
use webp::sharpyuv::sharpyuv_csp::{SharpYuvMatrixType, sharp_yuv_get_conversion_matrix};

fn bytes(words: &[u16]) -> &[u8] {
    // SAFETY: A byte view has the same lifetime and covers the initialized words.
    unsafe { std::slice::from_raw_parts(words.as_ptr().cast(), std::mem::size_of_val(words)) }
}

fn bytes_mut(words: &mut [u16]) -> &mut [u8] {
    // SAFETY: The exclusive byte view covers the initialized mutable words.
    unsafe { std::slice::from_raw_parts_mut(words.as_mut_ptr().cast(), std::mem::size_of_val(words)) }
}

fn main() {
    let port = std::env::args().nth(1).expect("pass avif or webp");
    assert!(port == "avif" || port == "webp");
    const WIDTH: usize = 256;
    const HEIGHT: usize = 256;
    const WARMUP: usize = 3;
    const MEASURED: usize = 30;
    let rgb_row = WIDTH * 6;
    let input_guard = rgb_row * 3;
    let mut input = vec![0_u16; (input_guard + rgb_row * (HEIGHT + 3)) / 2];
    let mut seed = 0x5359_5556_u32;
    for sample in &mut input {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        *sample = (seed >> 8) as u16;
    }
    let rgb_origin = input_guard + (HEIGHT - 1) * rgb_row + (WIDTH - 1) * 6;
    let input_origins = [rgb_origin, rgb_origin + 2, rgb_origin + 4];
    let y_row = WIDTH * 2;
    let uv_row = WIDTH.div_ceil(2) * 2;
    let y_guard = y_row * 3;
    let uv_guard = uv_row * 3;
    let mut y = vec![0xa5a5_u16; (y_guard + y_row * (HEIGHT + 3)) / 2];
    let mut u = vec![0xa5a5_u16; (uv_guard + uv_row * (HEIGHT.div_ceil(2) + 3)) / 2];
    let mut v = u.clone();
    let y_origin = y_guard + (HEIGHT - 1) * y_row;
    let uv_origin = uv_guard + (HEIGHT.div_ceil(2) - 1) * uv_row;
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
            // SAFETY: All 16-bit samples are aligned; the distinct output planes
            // and input include three guard rows at each formed pointer boundary.
            let result = unsafe {
                let base = input.as_ptr().cast::<u8>();
                sharpyuv::SharpYuvConvertWithOptions(
                    base.add(input_origins[0]).cast(),
                    base.add(input_origins[1]).cast(),
                    base.add(input_origins[2]).cast(),
                    -6, -(rgb_row as i32), 16,
                    y.as_mut_ptr().cast::<u8>().add(y_origin).cast(), -(y_row as i32),
                    u.as_mut_ptr().cast::<u8>().add(uv_origin).cast(), -(uv_row as i32),
                    v.as_mut_ptr().cast::<u8>().add(uv_origin).cast(), -(uv_row as i32),
                    10, WIDTH as i32, HEIGHT as i32, &raw_options,
                )
            };
            assert_eq!(result, 1);
        } else {
            let input_bytes = bytes(&input);
            let result = sharp_yuv_convert_with_options(
                input_bytes, input_origins[0], input_bytes, input_origins[1],
                input_bytes, input_origins[2], -6, -(rgb_row as i32), 16,
                bytes_mut(&mut y), y_origin, -(y_row as i32),
                bytes_mut(&mut u), uv_origin, -(uv_row as i32),
                bytes_mut(&mut v), uv_origin, -(uv_row as i32),
                10, WIDTH as i32, HEIGHT as i32, &safe_options,
            );
            assert!(result);
        }
        black_box((&y, &u, &v));
        if iteration >= WARMUP {
            elapsed += started.elapsed();
        }
    }
    let checksum: u64 = y.iter().chain(&u).chain(&v).map(|&sample| u64::from(sample)).sum();
    let mut usage = std::mem::MaybeUninit::<libc::rusage>::uninit();
    // SAFETY: A zero result initializes the complete structure.
    assert_eq!(unsafe { libc::getrusage(libc::RUSAGE_SELF, usage.as_mut_ptr()) }, 0);
    // SAFETY: The successful call above initialized the structure.
    let usage = unsafe { usage.assume_init() };
    let peak_rss_bytes = if cfg!(target_os = "macos") {
        usage.ru_maxrss as u64
    } else {
        (usage.ru_maxrss as u64) * 1024
    };
    println!("port={port} size={WIDTH}x{HEIGHT} RGB16/YUV420P10 reverse-rows reverse-pixels matrix=WebP transfer=sRGB warmup={WARMUP} measured={MEASURED} total_ms={:.3} mean_ms={:.3} peak_rss_bytes={peak_rss_bytes} checksum={checksum}",
        elapsed.as_secs_f64() * 1000.0,
        elapsed.as_secs_f64() * 1000.0 / MEASURED as f64);
}
