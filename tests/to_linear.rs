// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::color_encoding::TransferFunction;
use jxl::jxl::dec_xyb::OutputEncodingInfo;
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{
    RENDER_PIPELINE_X_OFFSET as OFFSET, Row, RowBuffers, RowStorage,
};
use jxl::jxl::render_pipeline::stage_to_linear::get_to_linear_stage;
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
fn inverse_factory_copies_pq_hlg_and_reciprocal_gamma_constructor_fields() {
    // Independently captured pristine GetToLinearStage/ProcessRow results.
    for (tf, custom, expected) in [
        (
            TransferFunction::SRGB,
            false,
            [
                [1022386271, 1032039833, 1038981598],
                [1041012212, 1046162841, 1050632903],
                [1051881455, 1056130963, 1059065644],
            ],
        ),
        (
            TransferFunction::PQ,
            false,
            [
                [989955847, 1006708581, 1019822361],
                [1024347190, 1035791000, 1047229702],
                [1050555267, 1060957088, 1071594638],
            ],
        ),
        (
            TransferFunction::HLG,
            false,
            [
                [1003617203, 1015106351, 1023056062],
                [1022783562, 1028411619, 1034307665],
                [1033952651, 1040888682, 1047822625],
            ],
        ),
        (
            TransferFunction::DCI,
            false,
            [
                [1019644130, 1031039272, 1038588083],
                [1040865147, 1046180908, 1050722710],
                [1051994718, 1056295049, 1059152175],
            ],
        ),
        (
            TransferFunction::LINEAR,
            true,
            [
                [1019644130, 1031039272, 1038588083],
                [1040865147, 1046180908, 1050722710],
                [1051994718, 1056295049, 1059152175],
            ],
        ),
    ] {
        let mut info = info(tf);
        info.color_encoding.tf_mut().have_gamma = custom;
        info.color_encoding.tf_mut().gamma = 1;
        let mut stage = get_to_linear_stage(&info);
        info.orig_intensity_target = 7.0;
        info.desired_intensity_target = 3.0;
        info.inverse_gamma = 2.0;
        info.luminances = [0.0; 3];
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
fn unknown_target_is_an_initialized_identity_including_signaling_nan_words() {
    let mut stage = get_to_linear_stage(&info(TransferFunction::UNKNOWN));
    assert!(stage.is_initialized().is_ok());
    let (mut p, ri) = rows();
    let words = [0x80000000u32, 0x7fa12345, 0xffa23456];
    for row in &mut p {
        for (x, word) in words.iter().enumerate() {
            row.row_mut(0)[OFFSET + x] = f32::from_bits(*word);
        }
    }
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
    for row in p {
        assert_eq!(
            row.row(0)[OFFSET..OFFSET + 3]
                .iter()
                .map(|x| x.to_bits())
                .collect::<Vec<_>>(),
            words
        );
    }
}
#[test]
fn inverse_horizontal_guards_run_before_any_color_or_extra_mutation() {
    let mut stage = get_to_linear_stage(&info(TransferFunction::HLG));
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
