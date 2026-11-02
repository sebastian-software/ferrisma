// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::render_pipeline::{
    render_pipeline_stage::{RENDER_PIPELINE_X_OFFSET as OFFSET, WriteToImage3FStage},
    simple_render_pipeline::SimpleRenderPipelineBuilder,
    stage_chroma_upsampling::get_chroma_upsampling_stage,
    stage_ycbcr::get_ycbcr_stage,
};
use jxl::jxl::{frame_dimensions::FrameDimensions, image::Image3};
#[test]
fn original_subsampled_chroma_then_ycbcr_pipeline_preserves_captured_pixels() {
    let mut dim = FrameDimensions::default();
    dim.set(3, 2, 0, 0, 0, true, 1);
    let mut image = Image3::default();
    {
        let mut builder = SimpleRenderPipelineBuilder::new(3);
        for c in [0, 2] {
            builder
                .add_stage(Some(get_chroma_upsampling_stage(c, true)))
                .unwrap();
            builder
                .add_stage(Some(get_chroma_upsampling_stage(c, false)))
                .unwrap();
        }
        builder.add_stage(Some(get_ycbcr_stage())).unwrap();
        builder
            .add_stage(Some(Box::new(WriteToImage3FStage::new(&mut image))))
            .unwrap();
        let mut pipeline = builder.finalize(dim).unwrap();
        pipeline.prepare_for_threads(1, true).unwrap();
        let mut input = pipeline.get_input_buffers(0, 0);
        for c in 0..3 {
            let (plane, r) = input.get_buffer_mut(c);
            for y in 0..r.ysize() {
                for x in 0..r.xsize() {
                    let ix = r.x0() + x - OFFSET;
                    let iy = r.y0() + y - OFFSET;
                    plane.row_mut(r.y0() + y)[r.x0() + x] =
                        ((c * 31 + ix * 17 + iy * 9) as i32 - 128) as f32 / 16.;
                }
            }
        }
        input.done().unwrap();
    }
    // Captured from the pinned original scalar stages in their real simple
    // pipeline: cs-simple mask109, 3x2, mode1, pass0. Chroma channels have both
    // source shifts while channel1 remains full resolution; odd edges are mirrored.
    let expected: [u32; 18] = [
        3241508905, 3240004296, 3238109192, 3240919081, 3239414472, 3237036048, 1041084160,
        1064007000, 1068876868, 1060334528, 1069398700, 1073595460, 3248350318, 3247546486,
        3246495879, 3248055406, 3247251574, 3246010638,
    ];
    for c in 0..3 {
        for y in 0..2 {
            for x in 0..3 {
                assert_eq!(
                    image.plane_row(c, y)[x].to_bits(),
                    expected[c * 6 + y * 3 + x]
                );
            }
        }
    }
}

#[test]
fn original_chroma_row_words_and_ycbcr_guard_preserve_mutation_order() {
    use jxl::jxl::image::Plane;
    use jxl::jxl::render_pipeline::render_pipeline_stage::{Row, RowBuffers, RowStorage};
    let values = [
        0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210, 0x3e800000, 0xbf000000, 0x3f000000,
        0x3f800000, 0x80000000,
    ]
    .map(f32::from_bits);
    let origin = (9 - OFFSET % 9) % 9;
    let len = origin + OFFSET + 11;
    for t in 0..3 {
        let mut input: Vec<_> = (0..5)
            .map(|_| Plane::<f32>::create(len, 7).unwrap())
            .collect();
        let mut output: Vec<_> = (0..5)
            .map(|_| Plane::<f32>::create(len, 5).unwrap())
            .collect();
        for c in 0..5 {
            for y in 0..7 {
                for x in 0..len {
                    input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
                }
            }
            for y in 0..5 {
                output[c].row_mut(y)[..len].fill(f32::from_bits(0x7fa0dead));
            }
        }
        let ir = (0..5)
            .map(|c| {
                if t == 2 { vec![1, 3, 5] } else { vec![3] }
                    .into_iter()
                    .map(|y| Row {
                        storage: RowStorage::Input,
                        channel: c,
                        y,
                        x: origin,
                    })
                    .collect()
            })
            .collect();
        let or = (0..5)
            .map(|c| {
                if t == 2 { vec![1, 3] } else { vec![2] }
                    .into_iter()
                    .map(|y| Row {
                        storage: RowStorage::Output,
                        channel: c,
                        y,
                        x: origin,
                    })
                    .collect()
            })
            .collect();
        let mut stage = if t == 0 {
            get_ycbcr_stage()
        } else {
            get_chroma_upsampling_stage(2, t == 1)
        };
        if t == 0 {
            assert!(
                stage
                    .process_row(
                        &ir,
                        &or,
                        &mut RowBuffers {
                            input: &mut input,
                            output: &mut output
                        },
                        1,
                        0,
                        3,
                        23,
                        41,
                        17
                    )
                    .is_err()
            );
            for (c, p) in input.iter().enumerate() {
                for y in 0..7 {
                    for x in 0..len {
                        assert_eq!(
                            p.row(y)[x].to_bits(),
                            values[(c * 7 + y * 3 + x) % 9].to_bits()
                        );
                    }
                }
            }
        }
        stage
            .process_row(
                &ir,
                &or,
                &mut RowBuffers {
                    input: &mut input,
                    output: &mut output,
                },
                0,
                0,
                3,
                23,
                41,
                17,
            )
            .unwrap();
        // Captured independently from both complete native ARM original graphs.
        // Input origin normalizes the source's target-dependent row prefix so the
        // same logical nine-word stencil is checked on every native target.
        let yc = [
            [0x3f4d0bc4, 0x3f99fa9e, 0x7fe12345],
            [0x7fe12345, 0xffe54321, 0x7fe12345],
            [0x7fe12345, 0xffe54321, 0x7fe12345],
        ];
        let horizontal = [
            0xbea00000, 0xbe800000, 0x3e800000, 0x3f200000, 0x3f600000, 0x3f400000,
        ];
        let vertical = [
            [0xbec00000, 0x7fe12345, 0xffe54321],
            [0x7fc98765, 0xffc43210, 0x3f500000],
        ];
        for (c, p) in input.iter().enumerate() {
            for y in 0..7 {
                for x in 0..len {
                    let expected = if t == 0
                        && c < 3
                        && y == 3
                        && (origin + OFFSET..origin + OFFSET + 3).contains(&x)
                    {
                        yc[c][x - origin - OFFSET]
                    } else {
                        values[(c * 7 + y * 3 + x) % 9].to_bits()
                    };
                    assert_eq!(
                        p.row(y)[x].to_bits(),
                        expected,
                        "input type{t},channel{c},{x},{y}"
                    );
                }
            }
        }
        for (c, p) in output.iter().enumerate() {
            for y in 0..5 {
                for x in 0..len {
                    let expected = if t == 1
                        && c == 2
                        && y == 2
                        && (origin + OFFSET..origin + OFFSET + 6).contains(&x)
                    {
                        horizontal[x - origin - OFFSET]
                    } else if t == 2
                        && c == 2
                        && [1, 3].contains(&y)
                        && (origin + OFFSET..origin + OFFSET + 3).contains(&x)
                    {
                        vertical[(y - 1) / 2][x - origin - OFFSET]
                    } else {
                        0x7fa0dead
                    };
                    assert_eq!(
                        p.row(y)[x].to_bits(),
                        expected,
                        "output type{t},channel{c},{x},{y}"
                    );
                }
            }
        }
    }
}
