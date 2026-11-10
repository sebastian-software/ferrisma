//! Compare the two imported SharpYUV implementations before consolidation.
//! Each port's independent C oracle remains the acceptance reference.

use webp::sharpyuv::sharpyuv::{
    sharp_yuv_convert_with_options, SharpYuvOptions, SharpYuvTransferFunctionType,
};
use webp::sharpyuv::sharpyuv_csp::{sharp_yuv_get_conversion_matrix, SharpYuvMatrixType};

fn bytes(words: &[u16]) -> Vec<u8> {
    words.iter().flat_map(|word| word.to_ne_bytes()).collect()
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
