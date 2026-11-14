// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::dec_cache::PassesDecoderState;
use jxl::jxl::frame_header::{BlendMode, BlendingInfo, FrameHeader, FrameOrigin};
use jxl::jxl::image::{Image3F, Plane};
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::{CodecMetadata, ExtraChannelInfo};
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
};
use jxl::jxl::render_pipeline::stage_blending::get_blending_stage_shared;
use std::sync::Arc;

fn metadata() -> CodecMetadata {
    let mut m = CodecMetadata::default();
    m.size.set(4, 2).unwrap();
    m.m.num_extra_channels = 1;
    m.m.extra_channel_info = vec![ExtraChannelInfo::default()];
    m
}
fn state<'a>(m: &'a CodecMetadata) -> PassesDecoderState<'a, 'a> {
    let mut state = PassesDecoderState::new();
    state.shared_storage.metadata = Some(m);
    state.output_encoding_info.color_encoding_is_original = true;
    for (i, slot) in state
        .shared_storage
        .reference_frames
        .borrow_mut()
        .iter_mut()
        .enumerate()
    {
        let mut bg = ImageBundle::new(Some(&m.m));
        let mut color = Image3F::create(4, 2).unwrap();
        for c in 0..3 {
            for y in 0..2 {
                color.plane_row_mut(c, y)[..4].fill(i as f32 + c as f32);
            }
        }
        bg.set_from_image(&mut Some(color), &m.m.color_encoding)
            .unwrap();
        let mut ec = Plane::create(4, 2).unwrap();
        for y in 0..2 {
            ec.row_mut(y)[..4].fill(20.0 + i as f32);
        }
        bg.extra_channels_mut().push(ec);
        slot.frame = Some(Box::new(bg));
    }
    state
}
fn header<'a>(m: &'a CodecMetadata) -> FrameHeader<'a> {
    let mut h = FrameHeader::new(Some(m));
    h.blending_info = BlendingInfo {
        mode: BlendMode::ADD,
        source: 3,
        ..BlendingInfo::default()
    };
    h.extra_channel_blending_info = vec![BlendingInfo {
        mode: BlendMode::REPLACE,
        source: 2,
        ..BlendingInfo::default()
    }];
    h
}
fn rows() -> (Vec<Plane<f32>>, Vec<Vec<Row>>) {
    let planes = (0..4)
        .map(|_| {
            let mut p = Plane::create(OFFSET + 12, 1).unwrap();
            p.row_mut(0)[..OFFSET + 12].fill(0.5);
            p
        })
        .collect();
    let descriptors = (0..4)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Input,
                channel: c,
                x: 0,
                y: 0,
            }]
        })
        .collect();
    (planes, descriptors)
}
#[test]
fn retained_stage_observes_same_reference_bundles_and_signed_crop() {
    let m = metadata();
    let state = state(&m);
    let mut h = header(&m);
    h.frame_origin = FrameOrigin { x0: -2, y0: 0 };
    let stage = get_blending_stage_shared(Arc::new(h), &state, &m.m.color_encoding);
    let identity = {
        let refs = state.shared_storage.reference_frames.borrow();
        refs[3].frame.as_deref().unwrap() as *const _
    };
    {
        let mut refs = state.shared_storage.reference_frames.borrow_mut();
        let bg = refs[3].frame.as_deref_mut().unwrap();
        bg.color_mut().plane_row_mut(0, 0)[..4].fill(8.0);
    }
    assert_eq!(identity, {
        let refs = state.shared_storage.reference_frames.borrow();
        refs[3].frame.as_deref().unwrap() as *const _
    });
    let (mut planes, descriptors) = rows();
    stage
        .process_row(
            &descriptors,
            &vec![],
            &mut RowBuffers {
                input: &mut planes,
                output: &mut [],
            },
            0,
            0,
            4,
            0,
            0,
            0,
        )
        .unwrap();
    assert_eq!(&planes[0].row(0)[OFFSET..OFFSET + 4], &[0.5, 0.5, 8.5, 8.5]);
    assert_eq!(&planes[3].row(0)[OFFSET..OFFSET + 4], &[0.5; 4]);
    stage.process_padding_row(
        &descriptors,
        &mut RowBuffers {
            input: &mut planes,
            output: &mut [],
        },
        4,
        0,
        0,
    );
    assert_eq!(&planes[0].row(0)[OFFSET..OFFSET + 4], &[8.0; 4]);
    assert_eq!(&planes[3].row(0)[OFFSET..OFFSET + 4], &[22.0; 4]);
}
#[test]
fn constructor_keeps_xyb_guard_before_empty_extra_reference_allocation() {
    let m = metadata();
    let state = state(&m);
    {
        let mut refs = state.shared_storage.reference_frames.borrow_mut();
        refs[3].ib_is_in_xyb = true;
        refs[2].frame = Some(Box::new(ImageBundle::new(Some(&m.m))));
    }
    let stage = get_blending_stage_shared(Arc::new(header(&m)), &state, &m.m.color_encoding);
    assert!(stage.is_initialized().is_err());
    assert_eq!(stage.constructor_sizes(), [4, 2, 0, 0]);
    {
        let mut refs = state.shared_storage.reference_frames.borrow_mut();
        refs[3].frame = Some(Box::new(ImageBundle::new(Some(&m.m))));
    }
    let stage = get_blending_stage_shared(Arc::new(header(&m)), &state, &m.m.color_encoding);
    assert!(stage.is_initialized().is_ok());
    assert_eq!(stage.constructor_sizes(), [4, 2, 4, 2]);
}
#[test]
fn extra_horizontal_input_is_rejected_before_any_foreground_write() {
    let m = metadata();
    let state = state(&m);
    let stage = get_blending_stage_shared(Arc::new(header(&m)), &state, &m.m.color_encoding);
    let (mut planes, descriptors) = rows();
    for (left, right) in [(1, 0), (0, 1)] {
        assert!(
            stage
                .process_row(
                    &descriptors,
                    &vec![],
                    &mut RowBuffers {
                        input: &mut planes,
                        output: &mut []
                    },
                    left,
                    right,
                    4,
                    0,
                    0,
                    0
                )
                .is_err()
        );
    }
    for p in planes {
        assert!(
            p.row(0)[..OFFSET + 12]
                .iter()
                .all(|v| v.to_bits() == 0.5f32.to_bits())
        );
    }
}
