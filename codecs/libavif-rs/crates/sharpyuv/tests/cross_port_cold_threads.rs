//! Exercise both SharpYUV public boundaries at their first concurrent convert.
//! This separate test binary has one test and makes no sequential warmup call.

use std::sync::{Arc, Barrier};

use webp::sharpyuv::sharpyuv::{
    SharpYuvOptions, SharpYuvTransferFunctionType, sharp_yuv_convert_with_options,
};
use webp::sharpyuv::sharpyuv_csp::{SharpYuvMatrixType, sharp_yuv_get_conversion_matrix};

const C_HASH: u64 = 0xa42a_4a0c_61e4_d1de;

#[derive(Clone, Copy)]
enum Port {
    Avif,
    Webp,
}

fn bytes(words: &[u16]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_ne_bytes()).collect()
}

fn fnv64(parts: &[&[u8]]) -> u64 {
    let mut hash = 14_695_981_039_346_656_037_u64;
    for part in parts {
        for &byte in *part {
            hash = (hash ^ u64::from(byte)).wrapping_mul(1_099_511_628_211);
        }
    }
    hash
}

fn convert(port: Port) -> [Vec<u8>; 3] {
    // This is case zero of cross_port's signed-layout inventory. Its exact
    // whole-allocation hash comes from the pinned SharpYUV 0.4.2 C reference.
    const WIDTH: usize = 3;
    const HEIGHT: usize = 5;
    const INPUT_GUARD: usize = 64;
    const OUTPUT_GUARD: usize = 32;
    let mut input = vec![0xa5a5_u16; 128 + INPUT_GUARD];
    for (index, word) in input[INPUT_GUARD / 2..INPUT_GUARD / 2 + 128]
        .iter_mut()
        .enumerate()
    {
        *word = (index as u16).wrapping_mul(1009).wrapping_add(0x8001);
    }
    let input_bytes = bytes(&input);
    let origins = [INPUT_GUARD, INPUT_GUARD + 2, INPUT_GUARD + 4];
    let y_origin = OUTPUT_GUARD;
    let uv_origin = OUTPUT_GUARD;
    let mut y = vec![0xa5_u8; (64 + OUTPUT_GUARD) * 2];
    let mut u = vec![0xa5_u8; (32 + OUTPUT_GUARD) * 2];
    let mut v = u.clone();

    match port {
        Port::Avif => {
            // SAFETY: Matrix zero is a live static predefined matrix.
            let matrix = unsafe { sharpyuv::SharpYuvGetConversionMatrix(0) };
            assert!(!matrix.is_null());
            let options = sharpyuv::SharpYuvOptions {
                yuv_matrix: matrix,
                transfer_type: 13,
            };
            // SAFETY: All 16-bit input samples are aligned and initialized;
            // input includes the original C loop's two-row pointer guard.
            // Distinct output planes have their full strides and final guard.
            let result = unsafe {
                let base = input.as_ptr().cast::<u8>();
                sharpyuv::SharpYuvConvertWithOptions(
                    base.add(origins[0]).cast(),
                    base.add(origins[1]).cast(),
                    base.add(origins[2]).cast(),
                    6,
                    24,
                    16,
                    y.as_mut_ptr().add(y_origin).cast(),
                    8,
                    u.as_mut_ptr().add(uv_origin).cast(),
                    6,
                    v.as_mut_ptr().add(uv_origin).cast(),
                    6,
                    8,
                    WIDTH as i32,
                    HEIGHT as i32,
                    &options,
                )
            };
            assert_eq!(result, 1);
        }
        Port::Webp => {
            let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(0)).unwrap();
            let options = SharpYuvOptions {
                yuv_matrix: matrix,
                transfer_type: SharpYuvTransferFunctionType(13),
            };
            let result = sharp_yuv_convert_with_options(
                &input_bytes,
                origins[0],
                &input_bytes,
                origins[1],
                &input_bytes,
                origins[2],
                6,
                24,
                16,
                &mut y,
                y_origin,
                8,
                &mut u,
                uv_origin,
                6,
                &mut v,
                uv_origin,
                6,
                8,
                WIDTH as i32,
                HEIGHT as i32,
                &options,
            );
            assert!(result);
        }
    }
    assert_eq!(bytes(&input), input_bytes, "conversion changed the input");
    let output = [y, u, v];
    assert_eq!(fnv64(&[&output[0], &output[1], &output[2]]), C_HASH);
    output
}

#[test]
fn first_concurrent_conversions_match_c_and_repeat() {
    const THREADS: usize = 8;
    let start = Arc::new(Barrier::new(THREADS));
    let threads: Vec<_> = (0..THREADS)
        .map(|index| {
            let start = Arc::clone(&start);
            std::thread::spawn(move || {
                // Half the threads enter each boundary first. This binary has
                // no earlier conversion, so both initializers face the barrier.
                let (first, second) = if index % 2 == 0 {
                    (Port::Avif, Port::Webp)
                } else {
                    (Port::Webp, Port::Avif)
                };
                start.wait();
                let first_output = convert(first);
                assert_eq!(convert(first), first_output, "repeated first boundary");
                let second_output = convert(second);
                assert_eq!(convert(second), second_output, "repeated second boundary");
                assert_eq!(first_output, second_output, "cross-port output");
            })
        })
        .collect();
    for thread in threads {
        thread.join().expect("cold-start conversion thread panicked");
    }
}
