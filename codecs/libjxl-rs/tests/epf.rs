// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::render_pipeline::{
    render_pipeline_stage::{RENDER_PIPELINE_X_OFFSET as OFFSET, WriteToImage3FStage},
    simple_render_pipeline::SimpleRenderPipelineBuilder,
    stage_epf::{EpfStage, get_epf_stage, get_epf_stage_shared},
};
use jxl::jxl::{
    frame_dimensions::FrameDimensions,
    image::{Image3, Plane},
    loop_filter::LoopFilter,
};
use std::{cell::RefCell, rc::Rc};
#[test]
fn original_epf_three_pass_words_and_late_sigma_owner_are_preserved() {
    // Captured from the pinned original scalar archive in its real simple
    // pipeline: 3x2, mode 1, pass 0, with the filter and sigma values below.
    let expected = [
        [
            3237338086, 3235432077, 3233344040, 3237025243, 3235036334, 3232802691, 3233274853,
            3231368844, 3228947534, 3232962010, 3230973101, 3227864837, 3228809160, 3224997144,
            3220416669, 3228183476, 3224205659, 3218251269,
        ],
        [
            3237647265, 3235489434, 3233407603, 3236961677, 3234866644, 3232598844, 3233584033,
            3231426202, 3229074660, 3232898445, 3230803412, 3227457143, 3229427521, 3225111858,
            3220670921, 3228056347, 3223866280, 3217435885,
        ],
        [
            3237904477, 3235633036, 3233448030, 3236921250, 3234594816, 3232366592, 3233841246,
            3231569804, 3229155516, 3232858017, 3230531584, 3226992640, 3229778014, 3225399064,
            3220832631, 3227975492, 3223322624, 3216506880,
        ],
    ];
    let lf = LoopFilter {
        epf_iters: 3,
        epf_pass0_sigma_scale: 0.9,
        epf_pass2_sigma_scale: 6.5,
        epf_border_sad_mul: 0.67,
        epf_channel_scale: [0.01, 0.02, 0.03],
        epf_pass1_zeroflush: 0.62,
        epf_pass2_zeroflush: 0.7,
        ..LoopFilter::default()
    };
    for (i, pass) in [EpfStage::Zero, EpfStage::One, EpfStage::Two]
        .into_iter()
        .enumerate()
    {
        let sigma = Rc::new(RefCell::new(Plane::<f32>::create(64, 64).unwrap()));
        let stage = get_epf_stage_shared(&lf, sigma.clone(), pass);
        // A stage holds the same sigma owner, not a snapshot captured at creation.
        for y in 0..64 {
            for x in 0..64 {
                sigma.borrow_mut().row_mut(y)[x] = match (x + y) % 3 {
                    0 => -10.,
                    1 => -3.,
                    _ => -0.1,
                };
            }
        }
        assert!(
            get_epf_stage(
                &LoopFilter {
                    epf_iters: 0,
                    ..lf.clone()
                },
                &sigma.borrow(),
                pass
            )
            .is_none()
        );
        let mut dim = FrameDimensions::default();
        dim.set(3, 2, 0, 0, 0, true, 1);
        let mut image = Image3::default();
        {
            let mut builder = SimpleRenderPipelineBuilder::new(3);
            builder.add_stage(stage).unwrap();
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
        for c in 0..3 {
            for y in 0..2 {
                for x in 0..3 {
                    assert_eq!(
                        image.plane_row(c, y)[x].to_bits(),
                        expected[i][c * 6 + y * 3 + x],
                        "pass {i}, channel {c}, {x},{y}"
                    );
                }
            }
        }
    }
}

#[test]
fn original_epf_nan_payloads_and_skipped_sigma_keep_exact_words() {
    use jxl::jxl::render_pipeline::render_pipeline_stage::{Row, RowBuffers, RowStorage};
    let values = [
        0x7fa12345, 0xffa54321, 0x7fc98765, 0xffc43210, 0x7f800000, 0xff800000, 0x3f000000,
        0x3f800000, 0x80000000,
    ]
    .map(f32::from_bits);
    // Keep the logical nine-word stencil identical across target padding.
    let origin = (9 - OFFSET % 9) % 9;
    let len = origin + OFFSET + 7;
    let mut sigma = Plane::<f32>::create(128, 64).unwrap();
    for y in 0..64 {
        for x in 0..128 {
            sigma.row_mut(y)[x] = [-10., -4., -3.][(x + y) % 3];
        }
    }
    let mut input: Vec<_> = (0..5)
        .map(|_| Plane::<f32>::create(len, 9).unwrap())
        .collect();
    let mut output: Vec<_> = (0..5)
        .map(|_| Plane::<f32>::create(len, 3).unwrap())
        .collect();
    for c in 0..5 {
        for y in 0..9 {
            for x in 0..len {
                input[c].row_mut(y)[x] = values[(c * 7 + y * 3 + x) % 9];
            }
        }
        for y in 0..3 {
            output[c].row_mut(y)[..len].fill(f32::from_bits(0x7fa0dead));
        }
    }
    let ir = (0..5)
        .map(|c| {
            (0..5)
                .map(|y| Row {
                    storage: RowStorage::Input,
                    channel: c,
                    y: y + 1,
                    x: origin,
                })
                .collect()
        })
        .collect();
    let or = (0..5)
        .map(|c| {
            vec![Row {
                storage: RowStorage::Output,
                channel: c,
                y: 1,
                x: origin,
            }]
        })
        .collect();
    get_epf_stage(&LoopFilter::default(), &sigma, EpfStage::One)
        .unwrap()
        .process_row(
            &ir,
            &or,
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
    // Captured from the pristine selected ordinary and fully instrumented original
    // stage on native AArch64. The negative signaling word in column 2 is copied
    // by the original sigma skip and must remain unquieted.
    let expected = [
        [0x7fe54321, 0x7fc98765, 0xffa54321],
        [0x7fe54321, 0x7fc98765, 0x80000000],
        [0x7fe54321, 0x7fc98765, 0x3f000000],
    ];
    for (c, plane) in output.iter().enumerate() {
        for y in 0..3 {
            for x in 0..len {
                let wanted =
                    if c < 3 && y == 1 && (origin + OFFSET - 1..origin + OFFSET + 2).contains(&x) {
                        expected[c][x - (origin + OFFSET - 1)]
                    } else {
                        0x7fa0dead
                    };
                assert_eq!(plane.row(y)[x].to_bits(), wanted, "channel{c}, {x},{y}");
            }
        }
    }
}
