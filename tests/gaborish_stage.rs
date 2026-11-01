// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::Image3;
use jxl::jxl::loop_filter::LoopFilter;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineChannelMode as Mode, Settings,
    WriteToImage3FStage,
};
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_gaborish::get_gaborish_stage;

#[test]
fn original_scalar_gaborish_mirrored_3x2_words_are_preserved_without_cpp() {
    let lf = LoopFilter::default();
    assert!(
        get_gaborish_stage(&LoopFilter {
            gab: false,
            ..lf.clone()
        })
        .is_none()
    );
    let stage = get_gaborish_stage(&lf).unwrap();
    assert_eq!(stage.settings(), Settings::symmetric_border_only(1));
    assert_eq!(stage.get_name(), "Gab");
    for c in 0..5 {
        assert_eq!(
            stage.get_channel_mode(c),
            if c < 3 { Mode::InOut } else { Mode::Ignored }
        );
    }
    let mut dim = FrameDimensions::default();
    dim.set(3, 2, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        builder.add_stage(Some(stage)).unwrap();
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut image))))
            .unwrap();
        let mut pipeline = builder.finalize(dim).unwrap();
        pipeline.prepare_for_threads(1, true).unwrap();
        let mut input = pipeline.get_input_buffers(0, 0);
        for c in 0..3 {
            let (plane, rect) = input.get_buffer_mut(c);
            for y in 0..rect.ysize() {
                for x in 0..rect.xsize() {
                    let ix = rect.x0() + x - OFFSET;
                    let iy = rect.y0() + y - OFFSET;
                    plane.row_mut(rect.y0() + y)[rect.x0() + x] =
                        ((c * 31 + ix * 17 + iy * 9) as i32 - 128) as f32 / 16.0;
                }
            }
        }
        input.done().unwrap();
        assert_eq!(pipeline.passes_with_all_input(), 1);
    }
    // Captured from the pinned original scalar GetGaborishStage inside its
    // real simple pipeline: gab-simple, default weights, 3x2, mode 1, pass 0.
    // These are source observations, not values recomputed by a Rust filter.
    let expected = [
        [
            3237527838, 3235610093, 3233692348, 3236676932, 3234759187, 3232841442,
        ],
        [
            3233464607, 3231546860, 3229629115, 3232613701, 3230695955, 3227942339,
        ],
        [
            3229188669, 3225353178, 3221517687, 3227486857, 3223651366, 3218406278,
        ],
    ];
    for (c, words) in expected.iter().enumerate() {
        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(image.plane_row(c, y)[x].to_bits(), words[y * 3 + x]);
            }
        }
    }
}
