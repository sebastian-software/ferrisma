// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::jxl::color_encoding_internal::ColorEncoding;
use jxl::jxl::dec_bit_reader::BitReader;
use jxl::jxl::dec_patch_dictionary::{PatchDictionary, ReferenceFrame};
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_bundle::ImageBundle;
use jxl::jxl::image_metadata::ImageMetadata;
use jxl::jxl::render_pipeline::render_pipeline_stage::{Row, RowBuffers, RowStorage};

#[test]
fn repeated_dictionary_decode_replaces_previous_frame_blend_modes() {
    // Captured from the actual pinned BuildAndEncodeHistograms/WriteTokens:
    // one 1x1 patch at (0,0) using reference 0; add, then replace. This is the
    // source patch_dictionary_test.cc regression without a C++ runtime.
    let add = [0x12, 0x86, 0x24, 0x03, 0x18];
    let replace = [0x12, 0x42, 0x19, 0x20];
    let metadata = ImageMetadata::default();
    let references: [ReferenceFrame; 4] = std::array::from_fn(|r| {
        let mut frame = ImageBundle::new(Some(&metadata));
        if r == 0 {
            let mut color = Image3::create(1, 1).unwrap();
            for c in 0..3 {
                color.plane_row_mut(c, 0)[0] = 1.0;
            }
            frame
                .set_from_image(&mut Some(color), ColorEncoding::srgb(false))
                .unwrap();
        }
        ReferenceFrame {
            frame: Some(Box::new(frame)),
            ib_is_in_xyb: r == 0,
        }
    });
    let mut pd = PatchDictionary::new();
    pd.bind_own_reference_frames();
    for (bytes, expected) in [(add.as_slice(), 1.5f32), (replace.as_slice(), 1.0f32)] {
        let mut br = BitReader::new(bytes);
        let mut uses = false;
        pd.decode(
            jxl::jxl::memory_manager_internal::MemoryManagerPtr::NATIVE,
            &mut br,
            1,
            1,
            0,
            &mut uses,
            &references,
        )
        .unwrap();
        br.close().unwrap();
        assert!(!uses);
        assert_eq!(pd.get_references(), 1);
        assert_eq!(pd.get_patches_for_row(0), vec![0]);
        assert!(pd.get_patches_for_row(1).is_empty());
        let mut storage: Vec<_> = (0..3)
            .map(|_| {
                let mut p = Plane::create(1, 1).unwrap();
                p.row_mut(0)[0] = 0.5;
                p
            })
            .collect();
        let rows: Vec<_> = (0..3)
            .map(|c| Row {
                storage: RowStorage::Input,
                channel: c,
                x: 0,
                y: 0,
            })
            .collect();
        pd.add_one_row(
            &rows,
            &mut RowBuffers {
                input: &mut storage,
                output: &mut [],
            },
            0,
            0,
            1,
            &[],
            &references,
        )
        .unwrap();
        for p in storage {
            assert_eq!(p.row(0)[0].to_bits(), expected.to_bits());
        }
    }
    pd.clear();
    assert!(!pd.has_any());
    assert_eq!(pd.storage_sizes(), [0; 7]);
    assert_eq!(pd.blendings_stride(), Some(1));
    assert_eq!(pd.get_references(), 0);
}

#[test]
fn selected_original_alpha_max_retains_comparison_result() {
    // The pristine no-inline original TU retains std::max's comparison and
    // chooses kSmallAlpha for a signaling NaN in ordinary and ASan graphs.
    // The dedicated audit also compares all finite requests to both untouched
    // optimized original objects; see tools/alpha_reference.py.
    let mut r = [0.0f32];
    let mut g = [0.5f32];
    let mut b = [1.0f32];
    jxl::jxl::alpha::premultiply_alpha(
        &mut [&mut r, &mut g, &mut b],
        &[f32::from_bits(0xffa98765)],
        1,
    );
    assert_eq!(r[0].to_bits(), 0);
    assert_eq!(g[0].to_bits(), 0x32000000);
    assert_eq!(b[0].to_bits(), 0x32800000);
}

#[test]
fn independent_background_alpha_retains_first_nan_product_operand() {
    // Actual original BlendingStage green sample: color background reference0,
    // alpha background reference1, foreground Pixel8 at native ARM offset24.
    // The pristine scalar alpha TU observes +0x7fc12345 for the green result.
    // The differential corpus retains all four output words and both offsets.
    let bg = [0x80000000, 0x7fc12345, 0xff800000, 0xffc23456].map(f32::from_bits);
    let fg = [0x80000000, 0x7fc12345, 0xff800000, 0].map(f32::from_bits);
    let mut r = [0.0];
    let mut g = [0.0];
    let mut b = [0.0];
    let mut a = [0.0];
    jxl::jxl::alpha::perform_alpha_blending_rgba(
        [&bg[0..1], &bg[1..2], &bg[2..3], &bg[3..4]],
        [&fg[0..1], &fg[1..2], &fg[2..3], &fg[3..4]],
        &mut [&mut r, &mut g, &mut b, &mut a],
        1,
        false,
        true,
    );
    assert_eq!(g[0].to_bits(), 0x7fc12345);
}
