// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::chroma_from_luma::ColorCorrelation;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::image::Plane;
use jxl::jxl::render_pipeline::render_pipeline_stage::{Row, RowBuffers, RowStorage};
use jxl::jxl::splines::{QuantizedSpline, Spline, SplineDataView, SplinePoint, Splines};

#[test]
fn original_encoded_spline_decodes_without_the_cpp_oracle() {
    // Captured from the pinned original EncodeSplines, default HistogramParams,
    // scalar ordinary graph on AArch64. Three leading bits precede the spline.
    let bytes = [
        0x17, 0x00, 0x05, 0x24, 0x91, 0x02, 0x63, 0x44, 0xf3, 0x08, 0x46, 0x18, 0x8f, 0x10, 0x6c,
        0x8a, 0x41, 0x58, 0x7a, 0x88, 0x83, 0x97, 0x00,
    ];
    let mut br = BitReader::new(&bytes);
    br.skip_bits(3);
    let mut splines = Splines::new();
    splines.decode(&mut br, 64).unwrap();
    assert_eq!(br.total_bits_consumed(), 181);
    br.close().unwrap();
    assert!(splines.has_any());
    assert_eq!(splines.quantization_adjustment(), -1);
    assert_eq!(splines.starting_points(), &[SplinePoint::new(3.0, 4.0)]);
    let q = &splines.quantized_splines()[0];
    assert_eq!(q.control_points(), &[(2, 1), (-1, 2)]);
    for (c, first) in [10, 20, 30].into_iter().enumerate() {
        assert_eq!(q.color_dct()[c][0], first);
        assert!(q.color_dct()[c][1..].iter().all(|&v| v == 0));
    }
    assert_eq!(q.sigma_dct()[0], 4);
    assert!(q.sigma_dct()[1..].iter().all(|&v| v == 0));
    let mut result = Spline::default();
    q.dequantize(
        splines.starting_points()[0],
        -1,
        0.0,
        1.0,
        64,
        &mut 0,
        &mut result,
    )
    .unwrap();
    assert_eq!(
        result.control_points,
        &[
            SplinePoint::new(3.0, 4.0),
            SplinePoint::new(5.0, 5.0),
            SplinePoint::new(6.0, 8.0)
        ]
    );
    splines
        .initialize_draw_cache(8, 8, &ColorCorrelation::default())
        .unwrap();
    assert!(!splines.segments().is_empty());
    splines.clear();
    assert!(!splines.has_any());
    assert!(!splines.have_segment_indices());
    assert!(!splines.have_segment_y_start());
}
#[test]
fn cache_retains_original_duplicate_control_point_failure_and_borrowed_data() {
    let original = Spline {
        control_points: vec![SplinePoint::new(2.0, 3.0), SplinePoint::new(2.0, 3.0)],
        ..Spline::default()
    };
    let qs = [QuantizedSpline::create(&original, 0, 0.0, 1.0).unwrap()];
    let points = [original.control_points[0]];
    let mut s = Splines::new();
    s.set_data(SplineDataView {
        splines: &qs,
        starting_points: &points,
    });
    assert_eq!(s.quantized_splines().as_ptr(), qs.as_ptr());
    assert_eq!(
        s.initialize_draw_cache(8, 8, &ColorCorrelation::default()),
        Err(StatusCode::GenericError)
    );
    assert!(s.segments().is_empty());
    assert!(!s.have_segment_indices());
    assert!(!s.have_segment_y_start());
    assert!(s.has_any());
}
#[test]
fn three_row_borrows_preserve_descriptor_order_across_owners_rows_and_spans() {
    let mut input = vec![
        Plane::<f32>::create(12, 3).unwrap(),
        Plane::create(12, 3).unwrap(),
    ];
    let mut output = vec![Plane::<f32>::create(12, 3).unwrap()];
    {
        let mut b = RowBuffers {
            input: &mut input,
            output: &mut output,
        };
        let rows = [
            Row {
                storage: RowStorage::Output,
                channel: 0,
                y: 2,
                x: 3,
            },
            Row {
                storage: RowStorage::Input,
                channel: 1,
                y: 0,
                x: 4,
            },
            Row {
                storage: RowStorage::Input,
                channel: 0,
                y: 1,
                x: 2,
            },
        ];
        let [r, g, b] = b.three_rows_mut(rows, 4);
        r.fill(1.0);
        g.fill(2.0);
        b.fill(3.0);
    }
    assert_eq!(&output[0].row(2)[3..7], &[1.0; 4]);
    assert_eq!(&input[1].row(0)[4..8], &[2.0; 4]);
    assert_eq!(&input[0].row(1)[2..6], &[3.0; 4]);
    {
        let mut b = RowBuffers {
            input: &mut input,
            output: &mut output,
        };
        let rows = [
            Row {
                storage: RowStorage::Input,
                channel: 0,
                y: 2,
                x: 8,
            },
            Row {
                storage: RowStorage::Input,
                channel: 0,
                y: 2,
                x: 0,
            },
            Row {
                storage: RowStorage::Input,
                channel: 0,
                y: 2,
                x: 4,
            },
        ];
        let [r, g, b] = b.three_rows_mut(rows, 4);
        r.fill(4.0);
        g.fill(5.0);
        b.fill(6.0);
    }
    assert_eq!(
        &input[0].row(2)[..12],
        &[5.0, 5.0, 5.0, 5.0, 6.0, 6.0, 6.0, 6.0, 4.0, 4.0, 4.0, 4.0]
    );
}
#[test]
#[should_panic(expected = "restricted row spans must not overlap")]
fn three_row_borrows_reject_overlapping_restricted_spans() {
    let mut input = vec![Plane::<f32>::create(12, 1).unwrap()];
    let mut b = RowBuffers {
        input: &mut input,
        output: &mut [],
    };
    b.three_rows_mut(
        [
            Row {
                storage: RowStorage::Input,
                channel: 0,
                y: 0,
                x: 2,
            },
            Row {
                storage: RowStorage::Input,
                channel: 0,
                y: 0,
                x: 0,
            },
            Row {
                storage: RowStorage::Input,
                channel: 0,
                y: 0,
                x: 8,
            },
        ],
        4,
    );
}
