// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::Image3;
use jxl::jxl::image_metadata::CustomTransformData;
use jxl::jxl::render_pipeline::render_pipeline_stage::WriteToImage3FStage;
use jxl::jxl::render_pipeline::simple_render_pipeline::SimpleRenderPipelineBuilder;
use jxl::jxl::render_pipeline::stage_upsampling::get_upsampling_stage;

#[test]
fn scalar_upsampling_retains_original_separate_multiply_add_rounding() {
    let mut dim = FrameDimensions::default();
    dim.set(3, 2, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        builder
            .add_stage(get_upsampling_stage(&CustomTransformData::default(), 0, 1))
            .unwrap();
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut image))))
            .unwrap();
        let mut pipeline = builder.finalize(dim).unwrap();
        pipeline.prepare_for_threads(1, false).unwrap();
        let mut input = pipeline.get_input_buffers(0, 0);
        for c in 0..3 {
            let (plane, r) = input.get_buffer_mut(c);
            for y in 0..r.ysize() {
                for x in 0..r.xsize() {
                    plane.row_mut(r.y0() + y)[r.x0() + x] =
                        (((c * 31 + x * 17 + y * 9) % 257) as i32 - 128) as f32 / 16.0;
                }
            }
        }
        input.done().unwrap();
    }
    // Captured independently from original GetUpsamplingStage inside its simple
    // pipeline for the mirrored 3x2 image. Fusing MulAdd changes the last word.
    let expected = [
        3238002688, 3237559997, 3236217154, 3238002688, 3237559997, 3236217155,
    ];
    let actual: Vec<_> = (0..2)
        .flat_map(|y| image.plane_row(0, y)[..3].iter().map(|v| v.to_bits()))
        .collect();
    assert_eq!(actual, expected);
}
