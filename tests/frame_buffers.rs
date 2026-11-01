// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::rect::Rect;
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::encode_internal::JxlEncoderChunkedFrameAdapter;
use jxl::jxl::image::Image3;
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
fn format(channels: u32, align: usize) -> JxlPixelFormat {
    JxlPixelFormat {
        num_channels: channels,
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::NATIVE,
        align,
    }
}

#[test]
fn rebinding_keeps_the_previous_copy_until_copy_buffers_and_null_does_not_clear_it() {
    let first = [1, 2];
    let second = [7, 8];
    let mut input = JxlEncoderChunkedFrameAdapter::new(2, 1, 1);
    assert!(input.set_from_buffer(0, Some(&first), 2, format(1, 0)));
    assert!(input.set_from_buffer(1, Some(&first), 2, format(1, 0)));
    assert!(input.channels()[1].borrowed_buffer().is_none());
    assert_eq!(input.channels()[1].copied_buffer(), first);
    input.copy_buffers();
    assert!(input.set_from_buffer(0, Some(&second), 2, format(1, 0)));
    let mut row = 999;
    assert_eq!(
        input.get_color_channel_data_at(0, 0, 2, 1, &mut row),
        Some(first.as_slice())
    );
    assert_eq!(row, 2);
    assert_eq!(
        input.channels()[0].borrowed_buffer(),
        Some(second.as_slice())
    );
    input.copy_buffers();
    assert!(input.set_from_buffer(0, None, 2, format(1, 0)));
    assert_eq!(
        input.get_color_channel_data_at(0, 0, 2, 1, &mut row),
        Some(second.as_slice())
    );
    row = 999;
    assert!(
        input
            .get_color_channel_data_at(3, 0, 0, 0, &mut row)
            .is_none()
    );
    assert_eq!(row, 999);
}

#[test]
fn failed_stride_rounding_keeps_the_wrapped_stride_new_format_and_old_input_pointer() {
    let first = [1];
    let second = [2];
    let mut input = JxlEncoderChunkedFrameAdapter::new(usize::MAX, 1, 0);
    assert!(!input.set_from_buffer(0, Some(&first), 1, format(1, 0)));
    assert_eq!(input.channels()[0].stride(), usize::MAX);
    assert!(!input.set_from_buffer(0, Some(&second), 1, format(1, 2)));
    assert_eq!(input.channels()[0].stride(), 0);
    assert_eq!(input.channels()[0].format().align, 2);
    assert_eq!(
        input.channels()[0].borrowed_buffer(),
        Some(first.as_slice())
    );
    assert_eq!(input.channels()[0].buffer_size(), 1);
}

#[test]
fn gray_replication_error_keeps_converted_first_plane_untouched_later_planes_and_alpha_flag() {
    let config = LosslessThunderConfig::new(2, 1, true, LosslessSampleDepth::Uint8).unwrap();
    let data = [7, 9];
    let mut input = JxlEncoderChunkedFrameAdapter::new(2, 1, 0);
    assert!(input.set_from_buffer(0, Some(&data), 2, format(1, 0)));
    let mut color = Image3::<f32>::create(2, 1).unwrap();
    for c in 0..3 {
        color.plane_row_mut(c, 0).fill(f32::from_bits(0x7fa12345));
    }
    color.plane_mut(1).shrink_to(1, 1).unwrap();
    let mut alpha = true;
    assert_eq!(
        config.copy_color_channels(&input, Rect::new(0, 0, 2, 1), &mut color, &mut alpha),
        Err(StatusCode::GenericError)
    );
    assert!(!alpha);
    assert_eq!(
        &color.plane_row(0, 0)[..2],
        &[7.0 * (1.0f32 / 255.0), 9.0 * (1.0f32 / 255.0)]
    );
    assert_eq!(color.plane_row(1, 0)[0].to_bits(), 0x7fa12345);
    assert_eq!(color.plane_row(2, 0)[0].to_bits(), 0x7fa12345);
    assert!(input.set_from_buffer(0, None, 8, format(4, 0)));
    assert_eq!(
        config.copy_color_channels(&input, Rect::new(0, 0, 2, 1), &mut color, &mut alpha),
        Err(StatusCode::GenericError)
    );
    assert!(alpha);
}

#[test]
fn buffer_backed_zero_gray_components_retain_original_finalized_frame_bytes() {
    let config = LosslessThunderConfig::new(1, 1, true, LosslessSampleDepth::Uint8).unwrap();
    let mut frame = config.create_frame().unwrap();
    frame.allocate_frame_fields().unwrap();
    let data = [0];
    let mut input = JxlEncoderChunkedFrameAdapter::new(1, 1, 0);
    assert!(input.set_from_buffer(0, Some(&data), 1, format(1, 0)));
    let mut color = Image3::<f32>::create(8, 8).unwrap();
    color.shrink_to(1, 1).unwrap();
    let mut alpha = false;
    config
        .copy_color_channels(&input, Rect::new(0, 0, 1, 1), &mut color, &mut alpha)
        .unwrap();
    jxl::jxl::image_ops::pad_image_to_block_multiple_in_place(&mut color, 8).unwrap();
    frame.prepare_modular(&color).unwrap();
    frame.compute_modular_tokens().unwrap();
    let mut groups = Vec::new();
    frame.encode_groups(&mut groups, None).unwrap();
    let writer = frame.finalize_groups(&groups, None).unwrap();
    // Original group/header/TOC bytes; this is a frame, not a full codestream.
    assert_eq!(
        writer.get_span(),
        &[
            0x08, 0x00, 0x01, 0x00, 0x1c, 0x00, 0x4b, 0x12, 0xa5, 0x42, 0x85, 0x24, 0x0c
        ]
    );
}
