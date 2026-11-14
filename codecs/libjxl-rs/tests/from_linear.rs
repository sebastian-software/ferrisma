// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::color_encoding::TransferFunction;
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, RenderPipelineStage, Row, RowBuffers, RowStorage,
};
use jxl::jxl::render_pipeline::stage_from_linear::{
    get_from_linear_stage, get_srgb_from_linear_stage,
};
fn info(tf: TransferFunction) -> OutputEncodingInfo {
    let mut info = OutputEncodingInfo {
        orig_intensity_target: 1000.0,
        desired_intensity_target: 203.0,
        inverse_gamma: 0.45,
        luminances: [0.2627, 0.6780, 0.0593],
        ..OutputEncodingInfo::default()
    };
    info.color_encoding.tf_mut().set_transfer_function(tf);
    info
}
fn rows() -> (Vec<Plane<f32>>, Vec<Vec<Row>>) {
    let mut planes: Vec<_> = (0..4)
        .map(|_| {
            let mut p = Plane::create(OFFSET + 8, 1).unwrap();
            p.row_mut(0)[..OFFSET + 8].fill(-0.25);
            p
        })
        .collect();
    for (c, values) in [
        [0.1875, 0.28125, 0.375],
        [0.40625, 0.5, 0.59375],
        [0.625, 0.71875, 0.8125],
    ]
    .iter()
    .enumerate()
    {
        planes[c].row_mut(0)[OFFSET..OFFSET + 3].copy_from_slice(values);
    }
    let ri = (0..4)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Input,
                channel: c,
                y: 0,
                x: 0,
            }]
        })
        .collect();
    (planes, ri)
}
#[test]
fn factory_owns_selected_pq_and_gamma_state_instead_of_caller_encoding() {
    // Captured independently from the pristine GetFromLinearStage/ProcessRow.
    for (tf, custom, expected) in [
        (
            TransferFunction::PQ,
            false,
            [
                [1058179324, 1058894223, 1059408427],
                [1059552355, 1059927207, 1060238849],
                [1060332077, 1060586517, 1060810144],
            ],
        ),
        (
            TransferFunction::DCI,
            false,
            [
                [1055985327, 1058056066, 1059366307],
                [1059762062, 1060857632, 1061845107],
                [1062154951, 1063036377, 1063856609],
            ],
        ),
        (
            TransferFunction::LINEAR,
            true,
            [
                [1055985327, 1058056066, 1059366307],
                [1059762062, 1060857632, 1061845107],
                [1062154951, 1063036377, 1063856609],
            ],
        ),
    ] {
        let mut info = info(tf);
        info.color_encoding.tf_mut().have_gamma = custom;
        info.color_encoding.tf_mut().gamma = 1;
        let stage = get_from_linear_stage(&info).unwrap();
        info.orig_intensity_target = 7.0;
        info.inverse_gamma = 2.0;
        info.color_encoding
            .tf_mut()
            .set_transfer_function(TransferFunction::UNKNOWN);
        for pass in 0..2 {
            let (mut p, ri) = rows();
            stage
                .process_row(
                    &ri,
                    &vec![],
                    &mut RowBuffers {
                        input: &mut p,
                        output: &mut [],
                    },
                    0,
                    0,
                    3,
                    19,
                    23,
                    pass,
                )
                .unwrap();
            for c in 0..3 {
                assert_eq!(
                    p[c].row(0)[OFFSET..OFFSET + 3]
                        .iter()
                        .map(|x| x.to_bits())
                        .collect::<Vec<_>>(),
                    expected[c]
                );
            }
            assert!(
                p[3].row(0)[..OFFSET + 8]
                    .iter()
                    .all(|x| x.to_bits() == (-0.25f32).to_bits())
            );
        }
    }
}
#[test]
fn existing_srgb_factory_keeps_the_original_stage_words() {
    let general = get_from_linear_stage(&info(TransferFunction::SRGB)).unwrap();
    for stage in [
        get_srgb_from_linear_stage(),
        general as Box<dyn RenderPipelineStage>,
    ] {
        let (mut p, ri) = rows();
        stage
            .process_row(
                &ri,
                &vec![],
                &mut RowBuffers {
                    input: &mut p,
                    output: &mut [],
                },
                0,
                0,
                3,
                0,
                0,
                0,
            )
            .unwrap();
        let expected = [
            [1055965156, 1058086695, 1059415369],
            [1059814264, 1060913241, 1061897533],
            [1062205240, 1063077824, 1063886251],
        ];
        for c in 0..3 {
            assert_eq!(
                p[c].row(0)[OFFSET..OFFSET + 3]
                    .iter()
                    .map(|x| x.to_bits())
                    .collect::<Vec<_>>(),
                expected[c]
            );
        }
    }
}
#[test]
fn horizontal_guards_reject_before_any_color_or_extra_write() {
    let stage = get_from_linear_stage(&info(TransferFunction::HLG)).unwrap();
    let (mut p, ri) = rows();
    for row in &mut p {
        row.row_mut(0)[..OFFSET + 8].fill(f32::from_bits(0x7fa12345));
    }
    for (left, right) in [(1, 0), (0, 1)] {
        assert!(
            stage
                .process_row(
                    &ri,
                    &vec![],
                    &mut RowBuffers {
                        input: &mut p,
                        output: &mut []
                    },
                    left,
                    right,
                    3,
                    0,
                    0,
                    0
                )
                .is_err()
        );
    }
    for row in p {
        assert!(
            row.row(0)[..OFFSET + 8]
                .iter()
                .all(|x| x.to_bits() == 0x7fa12345)
        );
    }
}
#[cfg(debug_assertions)]
#[test]
#[should_panic(expected = "Invalid target encoding")]
fn invalid_encoding_is_a_debug_programmer_error() {
    let _ = get_from_linear_stage(&info(TransferFunction::UNKNOWN));
}
#[cfg(not(debug_assertions))]
#[test]
fn invalid_encoding_returns_the_original_release_null() {
    assert!(get_from_linear_stage(&info(TransferFunction::UNKNOWN)).is_none());
}
