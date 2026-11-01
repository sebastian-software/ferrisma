// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::color_encoding::ColorSpace;
use jxl::jxl::dec_xyb::{OpsinParams, OutputEncodingInfo};
use jxl::jxl::dec_xyb_scalar::xyb_to_rgb;
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineChannelMode as Mode, Row, RowBuffers,
    RowStorage,
};
use jxl::jxl::render_pipeline::stage_xyb::get_xyb_stage;

#[test]
fn scalar_xyb_preserves_original_float_words_and_nan_operand_order() {
    let mut p = OpsinParams::default();
    p.init(255.0);
    // Independently captured from the pinned original scalar XybToRgb. The
    // first pixel includes cancellation near zero; the last checks which
    // quiet-NaN payload survives the source scalar operand order. ADR 0002
    // records the isolated original-source reference compiler configuration.
    let pixels = [
        ([0.0; 3], [2952790004, 2952790016, 2952790014]),
        ([0.015, 0.5, 0.75], [1058772106, 1029631832, 1065792252]),
        ([-0.25, -1.0, 2.0], [3244326585, 1071069936, 1102954757]),
        (
            [f32::from_bits(0x7fc12345), f32::from_bits(0xffc98765), 0.75],
            [0xffc98765; 3],
        ),
    ];
    for ([x, y, b], expected) in pixels {
        assert_eq!(xyb_to_rgb(x, y, b, &p).map(f32::to_bits), expected);
    }
    p.init(1.0);
    // This original result combines overflow/cancellation with an independent
    // signaling NaN. Commuting the scalar additions changes the retained payload.
    assert_eq!(
        xyb_to_rgb(
            f32::from_bits(1545563933),
            f32::from_bits(1838757507),
            f32::from_bits(4286825518),
            &p,
        )
        .map(f32::to_bits),
        [0xffc3c42e; 3],
    );
}

#[test]
fn scaled_xyb_factory_copies_selection_ignores_extra_channels_and_guards_before_access() {
    let mut info = OutputEncodingInfo::default();
    info.color_encoding.set_color_space(ColorSpace::XYB);
    let mut stage = get_xyb_stage(&info);
    info.color_encoding.set_color_space(ColorSpace::RGB);
    assert_eq!(stage.get_name(), "XYB");
    assert_eq!(stage.get_channel_mode(0), Mode::InPlace);
    assert_eq!(stage.get_channel_mode(3), Mode::Ignored);
    let mut empty = RowBuffers {
        input: &mut [],
        output: &mut [],
    };
    for (left, right) in [(1, 0), (0, 1)] {
        assert_eq!(
            stage.process_row(&vec![], &vec![], &mut empty, left, right, 3, 0, 0, 0),
            Err(StatusCode::GenericError)
        );
    }
    let sentinel = 0x7fa98765;
    let mut planes: Vec<_> = (0..4)
        .map(|_| {
            let mut p = Plane::create(OFFSET + 8, 1).unwrap();
            p.row_mut(0).fill(f32::from_bits(sentinel));
            p
        })
        .collect();
    for (c, values) in [
        [1.4375, 2.5, 3.5625],
        [3.375, 4.4375, 5.5],
        [5.3125, 6.375, 7.4375],
    ]
    .into_iter()
    .enumerate()
    {
        planes[c].row_mut(0)[OFFSET..OFFSET + 3].copy_from_slice(&values);
    }
    let rows = (0..4)
        .map(|channel| {
            vec![Row {
                storage: RowStorage::Input,
                channel,
                y: 0,
                x: 0,
            }]
        })
        .collect();
    stage
        .process_row(
            &rows,
            &vec![],
            &mut RowBuffers {
                input: &mut planes,
                output: &mut [],
            },
            0,
            0,
            3,
            19,
            23,
            1,
        )
        .unwrap();
    // Captured from original GetXYBStage's scaled-output branch, independently
    // of these native rows. Every surrounding word and extra channel survives.
    let expected = [
        [1107665948, 1114070919, 1118080377],
        [1082099500, 1084750955, 1087386944],
        [1079309975, 1079309975, 1079309975],
    ];
    for (c, plane) in planes.iter().enumerate() {
        for (x, value) in plane.row(0).iter().enumerate() {
            assert_eq!(
                value.to_bits(),
                if c < 3 && (OFFSET..OFFSET + 3).contains(&x) {
                    expected[c][x - OFFSET]
                } else {
                    sentinel
                }
            );
        }
    }
}
