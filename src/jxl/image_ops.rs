// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Copy/fill, padded copy, and border mirroring from lib/jxl/image_ops.h, plus
//! block padding and both downsampling overloads from image_ops.cc.
//! Explicit names distinguish source overloads. Other header-only numeric image
//! operations remain unported. Slices replace row pointers and memcpy without
//! changing float operation order or observing uninitialized allocation padding.

use super::base::rect::Rect;
use super::base::status::{Status, StatusCode, StatusOr};
use super::frame_dimensions::BLOCK_DIM;
use super::image::{Image3, Plane, Sample};

pub fn copy_image_to<T: Sample>(from: &Plane<T>, to: &mut Plane<T>) -> Status {
    if from.xsize() != to.xsize() || from.ysize() != to.ysize() {
        return Err(StatusCode::GenericError);
    }
    if from.ysize() == 0 || from.xsize() == 0 {
        return Ok(());
    }
    for y in 0..from.ysize() {
        to.row_mut(y)[..from.xsize()].copy_from_slice(&from.row(y)[..from.xsize()]);
    }
    Ok(())
}
pub fn copy_rect_to<T: Sample>(rf: Rect, from: &Plane<T>, rt: Rect, to: &mut Plane<T>) -> Status {
    if (rf.xsize(), rf.ysize()) != (rt.xsize(), rt.ysize()) {
        return Err(StatusCode::GenericError);
    }
    if !rf.is_inside(Rect::new(0, 0, from.xsize(), from.ysize())) {
        return Err(StatusCode::GenericError);
    }
    if !rt.is_inside(Rect::new(0, 0, to.xsize(), to.ysize())) {
        return Err(StatusCode::GenericError);
    }
    if rf.xsize() == 0 {
        return Ok(());
    }
    for y in 0..rf.ysize() {
        to.row_mut(rt.y0() + y)[rt.x0()..rt.x1()]
            .copy_from_slice(&from.row(rf.y0() + y)[rf.x0()..rf.x1()]);
    }
    Ok(())
}
pub fn copy_rect3_to<T: Sample>(
    rf: Rect,
    from: &Image3<T>,
    rt: Rect,
    to: &mut Image3<T>,
) -> Status {
    if (rf.xsize(), rf.ysize()) != (rt.xsize(), rt.ysize()) {
        return Err(StatusCode::GenericError);
    }
    for c in 0..3 {
        copy_rect_to(rf, from.plane(c), rt, to.plane_mut(c))?;
    }
    Ok(())
}
pub fn copy_image3_to<T: Sample>(from: &Image3<T>, to: &mut Image3<T>) -> Status {
    copy_rect3_to(
        Rect::new(0, 0, from.xsize(), from.ysize()),
        from,
        Rect::new(0, 0, to.xsize(), to.ysize()),
        to,
    )
}
pub fn copy_image_to_with_padding<T: Sample>(
    rf: Rect,
    from: &Plane<T>,
    padding: usize,
    rt: Rect,
    to: &mut Plane<T>,
) -> Status {
    let x0 = padding.min(rf.x0());
    let x1 = padding.min(from.xsize().wrapping_sub(rf.x0()).wrapping_sub(rf.xsize()));
    let y0 = padding.min(rf.y0());
    let y1 = padding.min(from.ysize().wrapping_sub(rf.y0()).wrapping_sub(rf.ysize()));
    if rt.x0() < x0 || rt.y0() < y0 {
        return Err(StatusCode::GenericError);
    }
    copy_rect_to(
        Rect::new(
            rf.x0() - x0,
            rf.y0() - y0,
            rf.xsize().wrapping_add(x0).wrapping_add(x1),
            rf.ysize().wrapping_add(y0).wrapping_add(y1),
        ),
        from,
        Rect::new(
            rt.x0() - x0,
            rt.y0() - y0,
            rt.xsize().wrapping_add(x0).wrapping_add(x1),
            rt.ysize().wrapping_add(y0).wrapping_add(y1),
        ),
        to,
    )
}
pub fn fill_image<T: Sample>(value: T, image: &mut Plane<T>) {
    let width = image.xsize();
    for y in 0..image.ysize() {
        let row = image.row_mut(y);
        for sample in &mut row[..width] {
            *sample = value;
        }
    }
}
pub fn zero_fill_image<T: Sample>(image: &mut Plane<T>) {
    if image.xsize() == 0 {
        return;
    }
    let width = image.xsize();
    for y in 0..image.ysize() {
        image.row_mut(y)[..width].fill(T::default());
    }
}
pub fn fill_image3<T: Sample>(value: T, image: &mut Image3<T>) {
    let width = image.xsize();
    let height = image.ysize();
    for c in 0..3 {
        for y in 0..height {
            let row = image.plane_row_mut(c, y);
            for sample in &mut row[..width] {
                *sample = value;
            }
        }
    }
}
pub fn zero_fill_image3<T: Sample>(image: &mut Image3<T>) {
    let width = image.xsize();
    let height = image.ysize();
    for c in 0..3 {
        for y in 0..height {
            if width != 0 {
                image.plane_row_mut(c, y)[..width].fill(T::default());
            }
        }
    }
}
pub fn fill_plane<T: Sample>(value: T, image: &mut Plane<T>, rect: Rect) {
    for y in 0..rect.ysize() {
        let row = image.row_mut(rect.y0() + y);
        for sample in &mut row[rect.x0()..rect.x1()] {
            *sample = value;
        }
    }
}
pub fn mirror(mut x: i64, size: i64) -> i64 {
    assert_ne!(size, 0);
    while x < 0 || x >= size {
        if x < 0 {
            x = -x - 1;
        } else {
            x = 2 * size - 1 - x;
        }
    }
    x
}

fn div_ceil(what: usize, align: usize) -> usize {
    assert_ne!(align, 0, "source division requires a nonzero divisor");
    what.wrapping_add(align - 1) / align
}

/// The source assumes a nonempty image with sufficient original allocation.
/// ShrinkTo errors retain its earlier plane-size changes before pixel access.
pub fn pad_image_to_block_multiple_in_place(image: &mut Image3<f32>, block_dim: usize) -> Status {
    let xsize_orig = image.xsize();
    let ysize_orig = image.ysize();
    let xsize = div_ceil(xsize_orig, block_dim).wrapping_mul(block_dim);
    let ysize = div_ceil(ysize_orig, block_dim).wrapping_mul(block_dim);
    image.shrink_to(xsize, ysize)?;
    assert!(
        xsize_orig != 0 && ysize_orig != 0,
        "source padding reads the last input pixel/row"
    );
    // Image3::PlaneRow uses plane zero's stride for every component.
    let stride = image.plane(0).bytes_per_row();
    for c in 0..3 {
        for y in 0..ysize_orig {
            // An empty fill range need not construct a checked row view. The
            // source's unsigned rounding can shrink a huge block size to zero.
            if xsize_orig < xsize {
                let row = image.plane_row_mut(c, y);
                for x in xsize_orig..xsize {
                    row[x] = row[xsize_orig - 1];
                }
            }
        }
        // This byte copy is the source memcpy, including exact float payloads.
        // Its ranges lie in different rows of this same owning allocation.
        let bytes = image
            .plane_mut(c)
            .bytes_mut()
            .expect("nonempty source allocation");
        let source = (ysize_orig - 1) * stride;
        for y in ysize_orig..ysize {
            bytes.copy_within(source..source + xsize * size_of::<f32>(), y * stride);
        }
    }
    Ok(())
}

fn do_downsample_image(input: &Plane<f32>, factor: usize, output: &mut Plane<f32>) -> Status {
    if factor == 1 {
        return Err(StatusCode::GenericError);
    }
    output.shrink_to(
        div_ceil(input.xsize(), factor),
        div_ceil(input.ysize(), factor),
    )?;
    let xsize = output.xsize();
    for y in 0..output.ysize() {
        let row_out = output.row_mut(y);
        for (x, value) in row_out[..xsize].iter_mut().enumerate() {
            let mut cnt = 0usize;
            let mut sum = 0.0f32;
            for iy in 0..factor {
                if iy + factor * y >= input.ysize() {
                    break;
                }
                // Selecting each source row replaces strided pointer access.
                let row_in = input.row(factor * y + iy);
                for ix in 0..factor {
                    if ix + factor * x >= input.xsize() {
                        break;
                    }
                    sum += row_in[x * factor + ix];
                    cnt += 1;
                }
            }
            *value = sum / cnt as f32;
        }
    }
    Ok(())
}

pub fn downsample_image(image: &Plane<f32>, factor: usize) -> StatusOr<Plane<f32>> {
    let mut downsampled = Plane::create_in(
        div_ceil(image.xsize(), factor).wrapping_add(BLOCK_DIM),
        div_ceil(image.ysize(), factor).wrapping_add(BLOCK_DIM),
        0,
        image.have_memory_manager(),
    )?;
    do_downsample_image(image, factor, &mut downsampled)?;
    Ok(downsampled)
}

pub fn downsample_image3(image: &Image3<f32>, factor: usize) -> StatusOr<Image3<f32>> {
    if factor == 1 {
        return Err(StatusCode::GenericError);
    }
    let mut downsampled = Image3::create_in(
        div_ceil(image.xsize(), factor).wrapping_add(BLOCK_DIM),
        div_ceil(image.ysize(), factor).wrapping_add(BLOCK_DIM),
        image.plane(0).have_memory_manager(),
    )?;
    downsampled.shrink_to(
        downsampled.xsize() - BLOCK_DIM,
        downsampled.ysize() - BLOCK_DIM,
    )?;
    for c in 0..3 {
        do_downsample_image(image.plane(c), factor, downsampled.plane_mut(c))?;
    }
    Ok(downsampled)
}
