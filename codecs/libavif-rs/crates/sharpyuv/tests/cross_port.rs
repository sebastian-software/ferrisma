//! Compare the two imported SharpYUV implementations before consolidation.
//! Each port's independent C oracle remains the acceptance reference.

use webp::sharpyuv::sharpyuv::{
    sharp_yuv_convert_with_options, SharpYuvOptions, SharpYuvTransferFunctionType,
};
use webp::sharpyuv::sharpyuv_csp::{sharp_yuv_get_conversion_matrix, SharpYuvMatrixType};

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

#[test]
fn conversions_match_across_ports() {
    let mut cases = 0;
    for rgb_depth in [8, 10, 12, 16] {
        for yuv_depth in [8, 10, 12] {
            for transfer in [1, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18] {
                for matrix in 0..5 {
                    for (width, height) in [(1_usize, 1_usize), (3, 5), (17, 9)] {
                        let input_unit = if rgb_depth == 8 { 1 } else { 2 };
                        let input_stride = (3 * width + 6) * input_unit;
                        let mut input = vec![0_u16; input_stride * height / 2 + 1];
                        let mut seed = 0x4156_4946_u32;
                        let max = (1_u32 << rgb_depth) - 1;
                        let mut encoded = vec![0_u8; input.len() * 2];
                        for y in 0..height {
                            for x in 0..width {
                                for channel in 0..3 {
                                    seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
                                    let value = ((seed >> 8) & max) as u16;
                                    let offset = y * input_stride + (3 * x + channel) * input_unit;
                                    encoded[offset..offset + input_unit]
                                        .copy_from_slice(&value.to_ne_bytes()[..input_unit]);
                                }
                            }
                        }
                        for (word, pair) in input.iter_mut().zip(encoded.chunks_exact(2)) {
                            *word = u16::from_ne_bytes([pair[0], pair[1]]);
                        }
                        let output_unit = if yuv_depth == 8 { 1 } else { 2 };
                        let strides = [
                            (width + 5) * output_unit,
                            (width.div_ceil(2) + 5) * output_unit,
                        ];
                        let heights = [height, height.div_ceil(2)];
                        let mut y = vec![0xa5a5_u16; (strides[0] * heights[0]).div_ceil(2) + 8];
                        let mut u = vec![0xa5a5_u16; (strides[1] * heights[1]).div_ceil(2) + 8];
                        let mut v = u.clone();
                        let (mut safe_y, mut safe_u, mut safe_v) =
                            (bytes(&y), bytes(&u), bytes(&v));
                        let safe_matrix =
                            sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(matrix)).unwrap();
                        // SAFETY: 0..5 names the five immutable predefined matrices.
                        let raw_matrix = unsafe { sharpyuv::SharpYuvGetConversionMatrix(matrix) };
                        assert!(!raw_matrix.is_null());
                        // SAFETY: The preceding lookup returned live static matrix storage.
                        let raw_values = unsafe { &*raw_matrix };
                        assert_eq!(raw_values.rgb_to_y, safe_matrix.rgb_to_y);
                        assert_eq!(raw_values.rgb_to_u, safe_matrix.rgb_to_u);
                        assert_eq!(raw_values.rgb_to_v, safe_matrix.rgb_to_v);
                        let raw_options = sharpyuv::SharpYuvOptions {
                            yuv_matrix: raw_matrix.cast(),
                            transfer_type: transfer,
                        };
                        // SAFETY: The input is u16-aligned, initialized, and includes every
                        // declared row. Channel addresses lie within this one read-only
                        // allocation. Output planes are distinct aligned allocations with
                        // padded strides and complete luma/chroma extents. All storage and
                        // the static matrix outlive this synchronous call; no pointer escapes.
                        let raw_ok = unsafe {
                            let start = input.as_ptr().cast::<u8>();
                            sharpyuv::SharpYuvConvertWithOptions(
                                start.cast(),
                                start.add(input_unit).cast(),
                                start.add(2 * input_unit).cast(),
                                (3 * input_unit) as i32,
                                input_stride as i32,
                                rgb_depth,
                                y.as_mut_ptr().cast(),
                                strides[0] as i32,
                                u.as_mut_ptr().cast(),
                                strides[1] as i32,
                                v.as_mut_ptr().cast(),
                                strides[1] as i32,
                                yuv_depth,
                                width as i32,
                                height as i32,
                                &raw_options,
                            )
                        };
                        let safe_ok = sharp_yuv_convert_with_options(
                            &encoded,
                            0,
                            &encoded,
                            input_unit,
                            &encoded,
                            2 * input_unit,
                            (3 * input_unit) as i32,
                            input_stride as i32,
                            rgb_depth,
                            &mut safe_y,
                            0,
                            strides[0] as i32,
                            &mut safe_u,
                            0,
                            strides[1] as i32,
                            &mut safe_v,
                            0,
                            strides[1] as i32,
                            yuv_depth,
                            width as i32,
                            height as i32,
                            &SharpYuvOptions {
                                yuv_matrix: safe_matrix,
                                transfer_type: SharpYuvTransferFunctionType(transfer),
                            },
                        );
                        let context = format!("RGB{rgb_depth}/YUV{yuv_depth}, transfer={transfer}, matrix={matrix}, {width}x{height}");
                        assert_eq!(raw_ok, 1, "{context}");
                        assert!(safe_ok, "{context}");
                        assert_eq!(safe_y, bytes(&y), "Y: {context}");
                        assert_eq!(safe_u, bytes(&u), "U: {context}");
                        assert_eq!(safe_v, bytes(&v), "V: {context}");
                        assert_eq!(bytes(&input), encoded, "input changed: {context}");
                        cases += 1;
                    }
                }
            }
        }
    }
    assert_eq!(cases, 2880);
}

// The independent C oracles cover these contracts separately. Compare the two
// Rust public boundaries on the same signed layout before extracting a core.
#[test]
fn signed_layouts_and_repeated_calls_match_across_ports() {
    // Exact outputs from tests/sharpyuv/extraction_boundaries.c linked against
    // the pinned SharpYUV 0.4.2 C reference with -ffp-contract=off. The C
    // program and reproduction command are recorded in the evidence note.
    const C_HASHES: [u64; 16] = [
        0x6115_d527_c3f8_a95e, 0xe7c6_59af_2ecb_6ee9,
        0x4b42_4b22_f2a8_3d02, 0x01c5_17a5_2b57_4ba9,
        0x6115_d527_c3f8_a95e, 0x5535_dadb_0411_e06c,
        0x4b42_4b22_f2a8_3d02, 0x7bc4_036b_5303_9e29,
        0x9787_3e7a_20d8_c3b5, 0xf7ea_3fa1_cbfd_65f2,
        0x6b86_e2f6_0f18_cf60, 0xc905_8676_cd2b_d032,
        0x6541_1d97_24e1_b448, 0x3959_4bb7_6cd0_e671,
        0x0431_4105_77ac_9038, 0x38e1_1d20_da46_aa92,
    ];
    let width = 3_usize;
    let height = 5_usize;
    let mut case_index = 0;
    for planar in [false, true] {
        for reverse_rows in [false, true] {
            for reverse_pixels in [false, true] {
                for yuv_depth in [8, 10] {
                    let step = if planar { 2 } else { 6 };
                    let row = if planar { 16 } else { 24 };
                    let channel = if planar { [0, 64, 128] } else { [0, 2, 4] };
                    let input: Vec<u16> = (0..128)
                        .map(|i| (i as u16).wrapping_mul(1009).wrapping_add(0x8001))
                        .collect();
                    let input_bytes = bytes(&input);
                    let origins = channel.map(|base| {
                        base + if reverse_rows { (height - 1) * row } else { 0 }
                            + if reverse_pixels { (width - 1) * step } else { 0 }
                    });
                    let signed_step = if reverse_pixels { -(step as i32) } else { step as i32 };
                    let signed_row = if reverse_rows { -(row as i32) } else { row as i32 };
                    let output_unit = if yuv_depth == 8 { 1 } else { 2 };
                    let y_row = 8 * output_unit;
                    let uv_row = 6 * output_unit;
                    let y_origin = if reverse_rows { (height - 1) * y_row } else { 0 };
                    let uv_origin = if reverse_rows { (height.div_ceil(2) - 1) * uv_row } else { 0 };
                    let y_stride = if reverse_rows { -(y_row as i32) } else { y_row as i32 };
                    let uv_stride = if reverse_rows { -(uv_row as i32) } else { uv_row as i32 };
                    let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(0)).unwrap();
                    // SAFETY: Matrix 0 is a live static predefined matrix.
                    let raw_matrix = unsafe { sharpyuv::SharpYuvGetConversionMatrix(0) };
                    let raw_options = sharpyuv::SharpYuvOptions {
                        yuv_matrix: raw_matrix,
                        transfer_type: 13,
                    };
                    let safe_options = SharpYuvOptions {
                        yuv_matrix: matrix,
                        transfer_type: SharpYuvTransferFunctionType(13),
                    };
                    for repetition in 0..3 {
                        let mut raw_y = vec![0xa5a5_u16; 64];
                        let mut raw_u = vec![0xa5a5_u16; 32];
                        let mut raw_v = vec![0xa5a5_u16; 32];
                        let mut safe_y = bytes(&raw_y);
                        let mut safe_u = bytes(&raw_u);
                        let mut safe_v = bytes(&raw_v);
                        // SAFETY: All channel pointers are aligned within the initialized
                        // input allocation. Signed steps and strides stay inside it for
                        // every pixel; output planes are distinct, aligned, and large
                        // enough for luma and chroma rows in either direction.
                        let raw_ok = unsafe {
                            let base = input.as_ptr().cast::<u8>();
                            sharpyuv::SharpYuvConvertWithOptions(
                                base.add(origins[0]).cast(),
                                base.add(origins[1]).cast(),
                                base.add(origins[2]).cast(),
                                signed_step,
                                signed_row,
                                16,
                                raw_y.as_mut_ptr().cast::<u8>().add(y_origin).cast(),
                                y_stride,
                                raw_u.as_mut_ptr().cast::<u8>().add(uv_origin).cast(),
                                uv_stride,
                                raw_v.as_mut_ptr().cast::<u8>().add(uv_origin).cast(),
                                uv_stride,
                                yuv_depth,
                                width as i32,
                                height as i32,
                                &raw_options,
                            )
                        };
                        let safe_ok = sharp_yuv_convert_with_options(
                            &input_bytes, origins[0], &input_bytes, origins[1],
                            &input_bytes, origins[2], signed_step, signed_row, 16,
                            &mut safe_y, y_origin, y_stride,
                            &mut safe_u, uv_origin, uv_stride,
                            &mut safe_v, uv_origin, uv_stride,
                            yuv_depth, width as i32, height as i32, &safe_options,
                        );
                        let context = format!("planar={planar}, rows={reverse_rows}, pixels={reverse_pixels}, YUV{yuv_depth}, call={repetition}");
                        assert_eq!(raw_ok, 1, "{context}");
                        assert!(safe_ok, "{context}");
                        assert_eq!(safe_y, bytes(&raw_y), "Y: {context}");
                        assert_eq!(safe_u, bytes(&raw_u), "U: {context}");
                        assert_eq!(safe_v, bytes(&raw_v), "V: {context}");
                        assert_eq!(input_bytes, bytes(&input), "input: {context}");
                        assert_eq!(
                            fnv64(&[&safe_y, &safe_u, &safe_v]),
                            C_HASHES[case_index],
                            "pinned C oracle: {context}"
                        );
                    }
                    case_index += 1;
                }
            }
        }
    }
    assert_eq!(case_index, C_HASHES.len());
}

#[test]
fn rejected_arguments_leave_outputs_unchanged_across_ports() {
    let input = vec![0x1234_u16; 64];
    let input_bytes = bytes(&input);
    // (name, RGB step, RGB stride, RGB depth, Y stride, U stride, V stride,
    // YUV depth, width, height). These are defined early rejections in C.
    let cases = [
        ("zero width", 6, 18, 16, 10, 8, 8, 10, 0, 3),
        ("negative height", 6, 18, 16, 10, 8, 8, 10, 3, -1),
        ("maximum width", 6, 18, 16, 10, 8, 8, 10, i32::MAX, 3),
        ("RGB depth", 6, 18, 9, 10, 8, 8, 10, 3, 3),
        ("YUV depth", 6, 18, 16, 10, 8, 8, 9, 3, 3),
        ("RGB step alignment", 3, 18, 16, 10, 8, 8, 10, 3, 3),
        ("RGB stride alignment", 6, 17, 16, 10, 8, 8, 10, 3, 3),
        ("Y stride alignment", 6, 18, 16, 9, 8, 8, 10, 3, 3),
        ("U stride alignment", 6, 18, 16, 10, 7, 8, 10, 3, 3),
        ("V stride alignment", 6, 18, 16, 10, 8, 7, 10, 3, 3),
    ];
    let matrix = sharp_yuv_get_conversion_matrix(SharpYuvMatrixType(0)).unwrap();
    // SAFETY: Matrix 0 is a live static predefined matrix.
    let raw_matrix = unsafe { sharpyuv::SharpYuvGetConversionMatrix(0) };
    let raw_options = sharpyuv::SharpYuvOptions {
        yuv_matrix: raw_matrix,
        transfer_type: 13,
    };
    let safe_options = SharpYuvOptions {
        yuv_matrix: matrix,
        transfer_type: SharpYuvTransferFunctionType(13),
    };
    for (name, step, row, rgb_depth, y_row, u_row, v_row, yuv_depth, width, height) in cases {
        let mut raw_y = vec![0xa5a5_u16; 32];
        let mut raw_u = vec![0xa5a5_u16; 16];
        let mut raw_v = vec![0xa5a5_u16; 16];
        let mut safe_y = bytes(&raw_y);
        let mut safe_u = bytes(&raw_u);
        let mut safe_v = bytes(&raw_v);
        // SAFETY: All pointers are aligned and live. Each case returns before
        // dereferencing any sample or output pointer, including the cases with
        // dimensions too large for these buffers.
        let raw_ok = unsafe {
            let base = input.as_ptr().cast::<u8>();
            sharpyuv::SharpYuvConvertWithOptions(
                base.cast(), base.add(2).cast(), base.add(4).cast(),
                step, row, rgb_depth,
                raw_y.as_mut_ptr().cast(), y_row,
                raw_u.as_mut_ptr().cast(), u_row,
                raw_v.as_mut_ptr().cast(), v_row,
                yuv_depth, width, height, &raw_options,
            )
        };
        let safe_ok = sharp_yuv_convert_with_options(
            &input_bytes, 0, &input_bytes, 2, &input_bytes, 4,
            step, row, rgb_depth,
            &mut safe_y, 0, y_row,
            &mut safe_u, 0, u_row,
            &mut safe_v, 0, v_row,
            yuv_depth, width, height, &safe_options,
        );
        assert_eq!(raw_ok, 0, "{name}");
        assert!(!safe_ok, "{name}");
        assert_eq!(safe_y, bytes(&raw_y), "Y: {name}");
        assert_eq!(safe_u, bytes(&raw_u), "U: {name}");
        assert_eq!(safe_v, bytes(&raw_v), "V: {name}");
        assert!(safe_y.iter().all(|&byte| byte == 0xa5), "Y modified: {name}");
        assert!(safe_u.iter().all(|&byte| byte == 0xa5), "U modified: {name}");
        assert!(safe_v.iter().all(|&byte| byte == 0xa5), "V modified: {name}");
    }
}
