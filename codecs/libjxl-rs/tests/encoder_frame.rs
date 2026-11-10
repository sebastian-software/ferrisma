// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Native ownership, partial-state, and header integration checks. No image
// codestream is supplied by these header-only tests.

use jxl::jxl::base::override_option::Override;
use jxl::jxl::base::status::StatusCode;
use jxl::jxl::cms_interface::{JxlCmsInterface, JxlColorProfile};
use jxl::jxl::color_encoding::{ColorSpace, RenderingIntent};
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_fields::write_frame_header;
use jxl::jxl::enc_frame::{FrameInfo, make_frame_header, params_post_init, set_progressive_mode};
use jxl::jxl::enc_params::CompressParams;
use jxl::jxl::enc_progressive_split::{PassDefinition, ProgressiveMode, ProgressiveSplitter};
use jxl::jxl::frame_header::{
    ColorTransform, FrameEncoding, FrameHeader, FrameOrigin, FrameSize, Passes, read_frame_header,
};
use jxl::jxl::image_metadata::CodecMetadata;
use jxl::jxl::splines::{QuantizedSpline, SplineDataView, SplinePoint};
use std::ffi::c_void;
use std::sync::atomic::{AtomicUsize, Ordering};

static CMS_CALLS: AtomicUsize = AtomicUsize::new(0);
// Marker for the descriptor-copy test. SetCms/Clone/Drop must never invoke it.
unsafe extern "C" fn cms_init_marker(
    data: *mut c_void,
    _: usize,
    _: usize,
    _: *const JxlColorProfile,
    _: *const JxlColorProfile,
    _: f32,
) -> *mut c_void {
    CMS_CALLS.fetch_add(1, Ordering::Relaxed);
    data
}

#[test]
fn encoder_parameters_copy_owners_borrow_views_and_keep_callback_descriptors_inactive() {
    let splines = vec![QuantizedSpline::default(), QuantizedSpline::default()];
    let points = [SplinePoint::new(1.0, 2.0), SplinePoint::new(3.0, 4.0)];
    let mode = ProgressiveMode::default();
    let mut token = 7u8;
    let cms = JxlCmsInterface {
        init_data: std::ptr::from_mut(&mut token).cast(),
        set_fields_data: std::ptr::from_mut(&mut token).cast(),
        init: Some(cms_init_marker),
        ..Default::default()
    };
    let mut p = CompressParams {
        ec_distance: vec![-0.0, f32::from_bits(0x7fc01234), -1.0],
        custom_splines: SplineDataView {
            splines: &splines,
            starting_points: &points,
        },
        custom_progressive_mode: Some(&mode),
        ..Default::default()
    };
    assert!(!p.cms_set);
    p.set_cms(&cms);
    let copy = p.clone();
    assert!(p.cms_set && copy.cms_set);
    assert_eq!(copy.cms.init_data, cms.init_data);
    assert_eq!(copy.cms.set_fields_data, cms.set_fields_data);
    assert!(copy.cms.init.is_some());
    assert_ne!(p.ec_distance.as_ptr(), copy.ec_distance.as_ptr());
    assert!(std::ptr::eq(
        copy.custom_splines.splines.as_ptr(),
        splines.as_ptr()
    ));
    assert!(std::ptr::eq(
        copy.custom_splines.starting_points.as_ptr(),
        points.as_ptr()
    ));
    assert!(std::ptr::eq(copy.custom_progressive_mode.unwrap(), &mode));
    p.set_lossless();
    assert!(p.is_lossless());
    assert_eq!(p.color_transform, ColorTransform::NONE);
    assert!(p.ec_distance.iter().all(|v| v.to_bits() == 0));
    assert_eq!(copy.ec_distance[0].to_bits(), (-0.0f32).to_bits());
    assert_eq!(copy.ec_distance[1].to_bits(), 0x7fc01234);
    p.set_cms(&JxlCmsInterface::default());
    assert!(p.cms.init_data.is_null() && p.cms.init.is_none());
    drop(p);
    drop(copy);
    assert_eq!(CMS_CALLS.load(Ordering::Relaxed), 0);
}

#[test]
fn failed_parameter_validation_and_header_preparation_keep_completed_source_mutations() {
    let mut p = CompressParams {
        butteraugli_distance: 0.0,
        manual_noise: vec![0.0],
        ..Default::default()
    };
    assert_eq!(params_post_init(&mut p), Err(StatusCode::GenericError));
    assert_eq!(p.butteraugli_distance.to_bits(), 0);
    assert_eq!(p.original_butteraugli_distance, -1.0);
    assert_eq!(p.resampling, -1);
    p.manual_noise.clear();
    p.set_lossless();
    params_post_init(&mut p).unwrap();
    let mut metadata = CodecMetadata::default();
    metadata.size.set(513, 257).unwrap();
    metadata.m.xyb_encoded = false;
    let mut header = FrameHeader::new(Some(&metadata));
    header.save_as_reference = 2;
    header.custom_size_or_origin = true;
    header.frame_size = FrameSize {
        xsize: 400,
        ysize: 200,
    };
    header.frame_origin = FrameOrigin { x0: -2, y0: 3 };
    header.flags = 2;
    let info = FrameInfo {
        dc_level: 3,
        is_last: false,
        name: vec![b'f', 0, 255],
        ..Default::default()
    };
    assert_eq!(
        make_frame_header(
            513,
            257,
            &p,
            &ProgressiveSplitter::default(),
            &info,
            None,
            false,
            &mut header
        ),
        Err(StatusCode::GenericError)
    );
    assert_eq!(header.name, info.name);
    assert_eq!(header.encoding, FrameEncoding::MODULAR);
    assert_eq!(header.color_transform, ColorTransform::NONE);
    assert_eq!(header.dc_level, 3);
    assert!(!header.is_last);
    assert_eq!(header.flags, 0);
    assert_eq!(header.passes.num_passes, 1);
    assert!(!header.loop_filter.gab);
    assert_eq!(header.loop_filter.epf_iters, 0);
    assert_eq!(header.save_as_reference, 2);
    assert!(header.custom_size_or_origin);
    assert_eq!(
        (header.frame_size.xsize, header.frame_size.ysize),
        (400, 200)
    );
    assert_eq!((header.frame_origin.x0, header.frame_origin.y0), (-2, 3));
}

#[test]
fn original_parameter_preparation_and_frame_header_io_agree_for_lossless_gray_and_rgb() {
    for gray in [false, true] {
        let mut metadata = CodecMetadata::default();
        metadata.size.set(513, 257).unwrap();
        metadata.m.xyb_encoded = false;
        metadata.m.set_uint_samples(16);
        metadata
            .m
            .color_encoding
            .set_srgb(
                if gray {
                    ColorSpace::GRAY
                } else {
                    ColorSpace::RGB
                },
                RenderingIntent::RELATIVE,
            )
            .unwrap();
        let mut p = CompressParams::default();
        p.set_lossless();
        params_post_init(&mut p).unwrap();
        assert_eq!((p.resampling, p.ec_resampling), (1, 1));
        assert_eq!(p.original_butteraugli_distance.to_bits(), 0);
        let mut splitter = ProgressiveSplitter::default();
        set_progressive_mode(&p, &mut splitter);
        let mut header = FrameHeader::new(Some(&metadata));
        make_frame_header(
            513,
            257,
            &p,
            &splitter,
            &FrameInfo::default(),
            None,
            false,
            &mut header,
        )
        .unwrap();
        assert_eq!(header.encoding, FrameEncoding::MODULAR);
        assert_eq!(header.color_transform, ColorTransform::NONE);
        assert_eq!(header.group_size_shift, 1);
        assert!(!header.custom_size_or_origin);
        let dims = header.to_frame_dimensions();
        assert_eq!(
            (dims.xsize, dims.ysize, dims.num_groups, dims.num_dc_groups),
            (513, 257, 6, 1)
        );
        let mut writer = BitWriter::new();
        write_frame_header(&mut header, &mut writer, None).unwrap();
        let bits = writer.bits_written();
        writer
            .with_max_bits(
                7,
                LayerType::Header,
                None,
                |w, _| {
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
            .unwrap();
        let mut br = BitReader::new(writer.get_span());
        let mut read = FrameHeader::new(Some(&metadata));
        read_frame_header(&mut br, &mut read).unwrap();
        assert_eq!(br.total_bits_consumed(), bits);
        br.close().unwrap();
        assert_eq!(read.to_frame_dimensions().num_groups, 6);
        assert_eq!(read.loop_filter.padding(), 0);
        assert!(read.is_last);
    }
}

#[test]
fn progressive_selection_keeps_unused_array_entries_and_rejects_zero_after_count_updates() {
    let mode = ProgressiveMode::new(&[
        PassDefinition {
            num_coefficients: 2,
            shift: 0,
            suitable_for_downsampling_of_at_least: 4,
        },
        PassDefinition {
            num_coefficients: 3,
            shift: 0,
            suitable_for_downsampling_of_at_least: 2,
        },
        PassDefinition {
            num_coefficients: 8,
            shift: 0,
            suitable_for_downsampling_of_at_least: 0,
        },
    ]);
    let p = CompressParams {
        progressive_mode: Override::OFF,
        qprogressive_mode: Override::ON,
        custom_progressive_mode: Some(&mode),
        ..Default::default()
    };
    let mut splitter = ProgressiveSplitter::default();
    set_progressive_mode(&p, &mut splitter);
    assert_eq!(splitter.get_num_passes(), 3);
    let mut passes = Passes {
        downsample: [91; 11],
        last_pass: [92; 11],
        shift: [93; 11],
        ..Passes::default()
    };
    splitter.init_passes(&mut passes).unwrap();
    assert_eq!(passes.num_downsample, 2);
    assert_eq!(&passes.downsample[..3], &[4, 2, 91]);
    assert_eq!(&passes.last_pass[..3], &[0, 1, 92]);
    assert_eq!(&passes.shift[..4], &[0, 0, 0, 93]);
    splitter.set_progressive_mode(ProgressiveMode {
        num_passes: 0,
        ..mode
    });
    let previous = passes.clone();
    assert_eq!(
        splitter.init_passes(&mut passes),
        Err(StatusCode::GenericError)
    );
    assert_eq!((passes.num_passes, passes.num_downsample), (0, 0));
    assert_eq!(passes.downsample, previous.downsample);
    assert_eq!(passes.last_pass, previous.last_pass);
    assert_eq!(passes.shift, previous.shift);
}
