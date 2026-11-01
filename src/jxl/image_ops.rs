// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Copy/fill, padded copy, and border mirroring from lib/jxl/image_ops.h.
//! Explicit names distinguish source overloads. Numeric image operations and
//! image_ops.cc remain separate, unported boundaries.

use super::base::rect::Rect;
use super::base::status::{Status, StatusCode};
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
