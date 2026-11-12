// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Storage lifetime, bounds, and independent-copy regression tests.

use jxl::jxl::base::status::StatusCode;
use jxl::jxl::image::{Image3, Plane};
use jxl::jxl::image_ops::{
    copy_image_to, copy_image3_to, downsample_image, pad_image_to_block_multiple_in_place,
};
use jxl::jxl::modular::modular_image::{Channel, Image};
use jxl::jxl::modular::transform::squeeze_params::SqueezeParams;
use jxl::jxl::modular::transform::transform::{Transform, TransformId};

#[test]
fn block_padding_repeats_edge_samples_without_allocating_or_overwriting_row_padding() {
    let mut image = Image3::<f32>::create(8, 8).unwrap();
    let addresses: Vec<_> = (0..3)
        .map(|c| image.plane(c).bytes().unwrap().as_ptr())
        .collect();
    for c in 0..3 {
        image.plane_mut(c).bytes_mut().unwrap().fill(0xa5);
        for y in 0..3 {
            for x in 0..4 {
                image.plane_row_mut(c, y)[x] = (x + y * 10 + c * 100) as f32;
            }
        }
    }
    image.shrink_to(4, 3).unwrap();
    pad_image_to_block_multiple_in_place(&mut image, 8).unwrap();
    assert_eq!((image.xsize(), image.ysize()), (8, 8));
    for (c, &address) in addresses.iter().enumerate() {
        assert_eq!(image.plane(c).bytes().unwrap().as_ptr(), address);
        for y in 0..8 {
            for x in 0..8 {
                assert_eq!(
                    image.plane_row(c, y)[x],
                    (x.min(3) + y.min(2) * 10 + c * 100) as f32
                );
            }
            let bytes = image.plane(c).bytes().unwrap();
            let stride = image.plane(c).bytes_per_row();
            assert!(
                bytes[y * stride + 8 * 4..(y + 1) * stride]
                    .iter()
                    .all(|&v| v == 0xa5)
            );
        }
    }
}

#[test]
fn downsampling_averages_each_partial_edge_block_with_its_actual_sample_count() {
    let mut input = Plane::<f32>::create(4, 3).unwrap();
    for y in 0..3 {
        for x in 0..4 {
            input.row_mut(y)[x] = (y * 4 + x) as f32;
        }
    }
    let output = downsample_image(&input, 2).unwrap();
    assert_eq!((output.xsize(), output.ysize()), (2, 2));
    assert_eq!(&output.row(0)[..2], &[2.5, 4.5]);
    assert_eq!(&output.row(1)[..2], &[8.5, 10.5]);
    assert_eq!(
        downsample_image(&input, 1).err(),
        Some(StatusCode::GenericError)
    );
}

#[test]
fn downsampling_empty_images_preserves_the_source_allocation_owner() {
    let empty = Plane::<f32>::create(0, 0).unwrap();
    assert_eq!(
        downsample_image(&empty, 2).err(),
        Some(StatusCode::GenericError)
    );
    let mut allocated = Plane::<f32>::create(8, 8).unwrap();
    allocated.shrink_to(0, 0).unwrap();
    let output = downsample_image(&allocated, 2).unwrap();
    assert_eq!((output.xsize(), output.ysize()), (0, 0));
    assert!(output.bytes().is_some());
}

#[test]
fn aligned_rows_retain_samples_and_stride_through_shrink_and_restore() {
    let mut p = Plane::<i32>::create_with_pre_padding(513, 3, 2).unwrap();
    let stride = p.bytes_per_row();
    let storage = p.bytes().unwrap().len();
    assert_eq!(p.bytes().unwrap().as_ptr() as usize % 128, 0);
    for y in 0..3 {
        for x in 0..513 {
            p.row_mut(y)[x] = (1000 * y + x) as i32;
        }
    }
    p.shrink_to(7, 1).unwrap();
    assert_eq!(p.bytes_per_row(), stride);
    assert_eq!(p.bytes().unwrap().len(), storage);
    assert_eq!(p.shrink_to(514, 1), Err(StatusCode::GenericError));
    assert_eq!((p.xsize(), p.ysize()), (7, 1));
    p.shrink_to(513, 3).unwrap();
    for y in 0..3 {
        for x in 0..513 {
            assert_eq!(p.row(y)[x], (1000 * y + x) as i32);
        }
    }
    let moved = p;
    let mut empty = Plane::default();
    let mut moved = moved;
    moved.swap(&mut empty);
    assert!(moved.bytes().is_none());
    assert_eq!(empty.row(2)[512], 2512);
}
#[test]
fn copying_float_samples_keeps_bits_and_leaves_destination_padding_untouched() {
    let mut from = Plane::<f32>::create(3, 2).unwrap();
    let mut to = Plane::<f32>::create(3, 2).unwrap();
    to.bytes_mut().unwrap().fill(0xa5);
    let patterns = [0x80000000, 0x7fc12345, 0x00000001];
    for y in 0..2 {
        for (x, &bits) in patterns.iter().enumerate() {
            from.row_mut(y)[x] = f32::from_bits(bits);
        }
    }
    copy_image_to(&from, &mut to).unwrap();
    for y in 0..2 {
        for (x, &bits) in patterns.iter().enumerate() {
            assert_eq!(to.row(y)[x].to_bits(), bits);
        }
        let start = y * to.bytes_per_row() + 3 * 4;
        let end = (y + 1) * to.bytes_per_row();
        assert!(to.bytes().unwrap()[start..end].iter().all(|&b| b == 0xa5));
    }
}
#[test]
#[allow(
    clippy::needless_range_loop,
    reason = "Keep the index loops; stable Clippy versions differ."
)]
fn modular_clone_owns_distinct_samples_and_transform_storage() {
    let mut image = Image::create(19, 3, 8, 3).unwrap();
    for c in 0..3 {
        for y in 0..3 {
            for x in 0..19 {
                image.channel[c].row_mut(y)[x] = (100 * c + 20 * y + x) as i32;
            }
        }
    }
    let mut t = Transform::new(TransformId::SQUEEZE);
    t.squeezes = vec![SqueezeParams::default()];
    image.transform.push(t);
    let mut clone = Image::clone_image(&image).unwrap();
    clone.channel[1].row_mut(2)[18] = -7;
    clone.transform[0].squeezes[0].horizontal = true;
    assert_eq!(image.channel[1].row(2)[18], 158);
    assert!(!image.transform[0].squeezes[0].horizontal);
    assert!(!clone.empty());
    assert_eq!(clone.channel[2].component, 2);
    let mut three = Image3::<i32>::create(19, 3).unwrap();
    let mut target = Image3::create(19, 3).unwrap();
    for c in 0..3 {
        for y in 0..3 {
            three.plane_row_mut(c, y)[..19].copy_from_slice(&image.channel[c].row(y)[..19]);
        }
    }
    copy_image3_to(&three, &mut target).unwrap();
    assert_eq!(target.plane_row(2, 2)[18], 258);
}
#[test]
fn channel_resize_keeps_metadata_mutations_and_the_previous_plane_on_failure() {
    let mut c = Channel::create(7, 3).unwrap();
    c.row_mut(0)[0] = 9;
    assert_eq!(c.shrink_to(-1, 2), Err(StatusCode::GenericError));
    assert_eq!((c.w, c.h), (usize::MAX, 2));
    assert_eq!((c.plane.xsize(), c.plane.ysize()), (7, 3));
    assert_eq!(c.row(0)[0], 9);
    // The source plane has no memory-manager pointer after an empty create.
    // Growing it to positive dimensions reaches that null allocator and fails.
    let mut empty = Channel::create(0, 3).unwrap();
    assert_eq!(empty.shrink_to(7, 3), Err(StatusCode::GenericError));
    assert_eq!((empty.w, empty.h), (7, 3));
    assert_eq!(empty.plane.xsize(), 0);
    empty.shrink_to(0, 2).unwrap();
    assert!(empty.plane.bytes().is_none());
}
