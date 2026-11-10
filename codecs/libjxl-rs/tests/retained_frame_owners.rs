// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::{
    color_encoding_internal::ColorEncoding,
    dec_cache::PassesDecoderState,
    frame_dimensions::FrameDimensions,
    image::{Image3, Plane},
    image_bundle::ImageBundle,
    image_metadata::ImageMetadata,
    passes_state::{DcSource, PassesSharedState},
    render_pipeline::{
        render_pipeline_stage::{WriteToImage3FStage, WriteToImageBundleStage},
        simple_render_pipeline::SimpleRenderPipelineBuilder,
    },
};
use std::rc::Rc;

#[test]
fn decoder_owned_pipeline_writes_the_selected_live_dc_slot_after_owner_move() {
    for slot in 0..4 {
        let mut state = PassesDecoderState::new();
        state.shared_storage.dc_source = DcSource::Frame(slot);
        let owner = state.shared_storage.dc_frames.clone();
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::from_dc_frame(
                owner.clone(),
                slot,
            ))))
            .unwrap();
        let mut dimensions = FrameDimensions::default();
        dimensions.set(9, 3, 0, 0, 0, true, 1);
        state.render_pipeline = Some(builder.finalize(dimensions).unwrap());
        let mut state = std::convert::identity(state);
        assert!(Rc::ptr_eq(&owner, &state.shared_storage.dc_frames));
        // Replacing the selected allocation after stage construction must be
        // visible through both the retained stage and the source DC selector.
        owner.borrow_mut()[slot] = Image3::create(9, 3).unwrap();
        let pipeline = state.render_pipeline.as_mut().unwrap();
        pipeline.prepare_for_threads(1, false).unwrap();
        let mut input = pipeline.get_input_buffers(0, 0);
        for c in 0..3 {
            let (plane, rect) = input.get_buffer_mut(c);
            for y in 0..rect.ysize() {
                for x in 0..rect.xsize() {
                    plane.row_mut(rect.y0() + y)[rect.x0() + x] =
                        f32::from_bits(0x7fa12345 + c as u32);
                }
            }
        }
        input.done().unwrap();
        assert!(std::ptr::eq(
            &*state.shared_storage.dc(),
            &owner.borrow()[slot]
        ));
        for c in 0..3 {
            for y in 0..3 {
                assert!(
                    owner.borrow()[slot].plane_row(c, y)[..9]
                        .iter()
                        .all(|v| v.to_bits() == 0x7fa12345 + c as u32)
                );
            }
        }
        for other in 0..4 {
            if other != slot {
                assert_eq!(owner.borrow()[other].xsize(), 0);
            }
        }
    }
}

#[test]
fn reference_write_stage_and_final_move_keep_existing_reference_bundle_identity() {
    let metadata = ImageMetadata {
        color_encoding: ColorEncoding::srgb(false).clone(),
        ..ImageMetadata::default()
    };
    let shared = PassesSharedState::new();
    let references = shared.reference_frames.clone();
    for slot in 0..4 {
        *references.borrow_mut()[slot].frame.as_deref_mut().unwrap() =
            ImageBundle::new(Some(&metadata));
        let address = references.borrow()[slot].frame.as_deref().unwrap() as *const _;
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        builder
            .add_stage(Some(Box::new(WriteToImageBundleStage::from_reference(
                references.clone(),
                slot,
                &metadata.color_encoding,
            ))))
            .unwrap();
        let mut dimensions = FrameDimensions::default();
        dimensions.set(2, 1, 0, 0, 0, true, 1);
        let mut pipeline = builder.finalize(dimensions).unwrap();
        pipeline.prepare_for_threads(1, false).unwrap();
        let mut input = pipeline.get_input_buffers(0, 0);
        for c in 0..3 {
            let (plane, rect) = input.get_buffer_mut(c);
            plane.row_mut(rect.y0())[rect.x0()..rect.x0() + 2].fill(c as f32 + 1.0);
        }
        input.done().unwrap();
        let mut frame = ImageBundle::new(Some(&metadata));
        let mut color = Some(Image3::create(2, 1).unwrap());
        frame
            .set_from_image(&mut color, &metadata.color_encoding)
            .unwrap();
        frame.color_mut().plane_row_mut(0, 0)[0] = -7.0;
        frame.name = b"retained frame".to_vec();
        references.borrow_mut()[slot]
            .frame
            .as_deref_mut()
            .unwrap()
            .move_from(&mut frame);
        let reference = references.borrow();
        let bundle = reference[slot].frame.as_deref().unwrap();
        assert!(std::ptr::eq(bundle, address));
        assert_eq!(bundle.name, b"retained frame");
        assert_eq!(bundle.color().plane_row(0, 0)[0], -7.0);
    }
    assert!(Rc::ptr_eq(&references, &shared.reference_frames));
}

#[test]
fn moved_bundle_color_view_retains_source_headers_and_borrows_destination_allocation() {
    let metadata = ImageMetadata {
        color_encoding: ColorEncoding::srgb(false).clone(),
        ..ImageMetadata::default()
    };
    let mut source = ImageBundle::new(Some(&metadata));
    let mut color = Some(Image3::create(3, 2).unwrap());
    let address = color.as_ref().unwrap().plane(0).bytes().unwrap().as_ptr();
    source
        .set_from_image(&mut color, &metadata.color_encoding)
        .unwrap();
    source.color_mut().plane_row_mut(0, 0)[0] = f32::from_bits(0xffa12345);
    source
        .extra_channels_mut()
        .push(Plane::create(3, 2).unwrap());
    let mut destination = ImageBundle::new(None);
    let alias = destination.move_from(&mut source);
    assert_eq!(alias.color().plane(0).bytes().unwrap().as_ptr(), address);
    assert_eq!(alias.color().plane_row(0, 0)[0].to_bits(), 0xffa12345);
    assert_eq!((source.color().xsize(), source.color().ysize()), (3, 2));
    assert!(source.color().plane(0).bytes().is_none());
    assert!(source.extra_channels().is_empty());
    destination.remove_color();
    assert!(!destination.has_color());
    assert_eq!((destination.xsize(), destination.ysize()), (3, 2));
    assert_eq!(destination.extra_channels().len(), 1);
}

#[test]
fn decoder_retains_the_reference_temporary_until_final_move_into_existing_slot() {
    let metadata = ImageMetadata {
        color_encoding: ColorEncoding::srgb(false).clone(),
        ..ImageMetadata::default()
    };
    let mut state = PassesDecoderState::new();
    *state.frame_storage_for_referencing.borrow_mut() = ImageBundle::new(Some(&metadata));
    let temporary = state.frame_storage_for_referencing.clone();
    let references = state.shared_storage.reference_frames.clone();
    let identity = references.borrow()[2].frame.as_deref().unwrap() as *const _;
    let mut builder = SimpleRenderPipelineBuilder::new(3);
    builder
        .add_stage(Some(Box::new(WriteToImageBundleStage::from_owned(
            temporary.clone(),
            &metadata.color_encoding,
        ))))
        .unwrap();
    let mut dimensions = FrameDimensions::default();
    dimensions.set(1, 1, 0, 0, 0, true, 1);
    state.render_pipeline = Some(builder.finalize(dimensions).unwrap());
    let mut state = std::convert::identity(state);
    assert!(Rc::ptr_eq(&temporary, &state.frame_storage_for_referencing));
    let pipeline = state.render_pipeline.as_mut().unwrap();
    pipeline.prepare_for_threads(1, false).unwrap();
    let mut input = pipeline.get_input_buffers(0, 0);
    for c in 0..3 {
        let (plane, rect) = input.get_buffer_mut(c);
        plane.row_mut(rect.y0())[rect.x0()] = f32::from_bits(0xffc12345 + c as u32);
    }
    input.done().unwrap();
    let allocation = temporary
        .borrow()
        .color()
        .plane(0)
        .bytes()
        .unwrap()
        .as_ptr();
    references.borrow_mut()[2]
        .frame
        .as_deref_mut()
        .unwrap()
        .move_from(&mut temporary.borrow_mut());
    let reference = references.borrow();
    let destination = reference[2].frame.as_deref().unwrap();
    assert!(std::ptr::eq(destination, identity));
    assert_eq!(
        destination.color().plane(0).bytes().unwrap().as_ptr(),
        allocation
    );
    for c in 0..3 {
        assert_eq!(
            destination.color().plane_row(c, 0)[0].to_bits(),
            0xffc12345 + c as u32
        );
    }
    assert_eq!(temporary.borrow().color().xsize(), 1);
    assert!(temporary.borrow().color().plane(0).bytes().is_none());
}
