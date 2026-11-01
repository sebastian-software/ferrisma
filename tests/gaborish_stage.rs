// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::frame_dimensions::FrameDimensions;
use jxl::jxl::image::Image3;
use jxl::jxl::loop_filter::LoopFilter;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineChannelMode as Mode, Row, RowBuffers,
    RowStorage, Settings, WriteToImage3FStage,
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

#[test]
fn original_scalar_gaborish_competing_nan_rows_preserve_operator_boundaries() {
    use jxl::jxl::image::Plane;
    let values = [
        0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210, 0x7f800000, 0xff800000, 0x3f000000,
        0x3f800000, 0x80000000,
    ]
    .map(f32::from_bits);
    // Keep the logical stencil identical across the source's target-dependent
    // row padding. origin+OFFSET is a multiple of the nine-value input period.
    let origin = (9 - OFFSET % 9) % 9;
    let len = origin + OFFSET + 5;
    let mut input: Vec<_> = (0..5).map(|_| Plane::create(len, 7).unwrap()).collect();
    let mut output: Vec<_> = (0..5).map(|_| Plane::create(len, 4).unwrap()).collect();
    for c in 0..5 {
        for y in 0..7 {
            for x in 0..len {
                input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
            }
        }
        for y in 0..4 {
            output[c].row_mut(y)[..len].fill(f32::from_bits(0x7fa0dead));
        }
    }
    let rows = (0..5)
        .map(|c| {
            [1, 3, 5]
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
    let out_rows = (0..5)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Output,
                channel: c,
                y: 2,
                x: origin,
            }]
        })
        .collect();
    get_gaborish_stage(&LoopFilter::default())
        .unwrap()
        .process_row(
            &rows,
            &out_rows,
            &mut RowBuffers {
                input: &mut input,
                output: &mut output,
            },
            1,
            2,
            0,
            23,
            41,
            17,
        )
        .unwrap();
    // Captured from the unchanged original stage in the dedicated scalar TU:
    // ordinary and ASan/UBSan on ARM, ordinary and UBSan on x86. These are
    // source observations, not words recomputed by the Rust filter or its test.
    // The architectures have different NaN preference, even with identical
    // input stencils and source operator order. Each table was captured from
    // that target's actual original ordinary/instrumented scalar stage.
    let expected = if cfg!(target_arch = "x86_64") {
        [
            [0x7fe12345, 0xffe54321, 0xffe54321],
            [0xffe54321, 0xffe54321, 0x7fe12345],
            [0xffe54321, 0x7fe12345, 0xffe54321],
        ]
    } else {
        [
            [0xffe54321, 0x7fc98765, 0xffc43210],
            [0xffe54321, 0x7fe12345, 0xffe54321],
            [0x7fe12345, 0x7fe12345, 0xffe54321],
        ]
    };
    for (c, plane) in output.iter().enumerate() {
        for y in 0..4 {
            for x in 0..len {
                let expected =
                    if c < 3 && y == 2 && (origin + OFFSET - 1..origin + OFFSET + 2).contains(&x) {
                        expected[c][x - (origin + OFFSET - 1)]
                    } else {
                        0x7fa0dead
                    };
                assert_eq!(
                    plane.row(y)[x].to_bits(),
                    expected,
                    "channel {c}, ({x},{y})"
                );
            }
        }
    }
}
