// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Unsigned coordinate operations from `lib/jxl/base/rect.h`'s Rect alias.
//! Signed RectT instantiations and image/row accessors remain unported.

use super::status::{StatusCode, StatusOr};
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Rect {
    x0: usize,
    y0: usize,
    xsize: usize,
    ysize: usize,
}
const fn clamped_size(begin: usize, size_max: usize, end: usize) -> usize {
    if begin.wrapping_add(size_max) <= end {
        size_max
    } else {
        end.saturating_sub(begin)
    }
}
impl Rect {
    pub const fn new(x0: usize, y0: usize, xsize: usize, ysize: usize) -> Self {
        Self {
            x0,
            y0,
            xsize,
            ysize,
        }
    }
    pub const fn new_clamped(
        x0: usize,
        y0: usize,
        xsize: usize,
        ysize: usize,
        xend: usize,
        yend: usize,
    ) -> Self {
        Self::new(
            x0,
            y0,
            clamped_size(x0, xsize, xend),
            clamped_size(y0, ysize, yend),
        )
    }
    pub fn crop(self, xsize: usize, ysize: usize) -> Self {
        self.intersection(Self::new(0, 0, xsize, ysize))
    }
    pub fn intersection(self, other: Self) -> Self {
        Self::new_clamped(
            self.x0.max(other.x0),
            self.y0.max(other.y0),
            self.xsize,
            self.ysize,
            self.x1().min(other.x1()),
            self.y1().min(other.y1()),
        )
    }
    pub fn translate(self, x: i64, y: i64) -> Self {
        Self::new(
            self.x0.wrapping_add(x as usize),
            self.y0.wrapping_add(y as usize),
            self.xsize,
            self.ysize,
        )
    }
    pub fn is_inside(self, other: Self) -> bool {
        self.x0 >= other.x0
            && self.x1() <= other.x1()
            && self.y0 >= other.y0
            && self.y1() <= other.y1()
    }
    pub fn is_same(self, other: Self) -> bool {
        self == other
    }
    pub const fn x0(self) -> usize {
        self.x0
    }
    pub const fn y0(self) -> usize {
        self.y0
    }
    pub const fn xsize(self) -> usize {
        self.xsize
    }
    pub const fn ysize(self) -> usize {
        self.ysize
    }
    pub const fn x1(self) -> usize {
        self.x0.wrapping_add(self.xsize)
    }
    pub const fn y1(self) -> usize {
        self.y0.wrapping_add(self.ysize)
    }
    pub fn shift_left(self, x: usize, y: usize) -> Self {
        Self::new(
            self.x0.wrapping_mul(1usize << x),
            self.y0.wrapping_mul(1usize << y),
            self.xsize.wrapping_shl(x as u32),
            self.ysize.wrapping_shl(y as u32),
        )
    }
    pub fn ceil_shift_right(self, shift: (usize, usize)) -> StatusOr<Self> {
        let (x, y) = shift;
        let dx = 1usize << x;
        let dy = 1usize << y;
        if self.x0 % dx != 0 || self.y0 % dy != 0 {
            return Err(StatusCode::GenericError);
        }
        Ok(Self::new(
            self.x0 / dx,
            self.y0 / dy,
            self.xsize.wrapping_add(dx - 1) / dx,
            self.ysize.wrapping_add(dy - 1) / dy,
        ))
    }
    pub fn extend(self, border: usize, parent: Self) -> Self {
        let x0 = if self.x0 > parent.x0.wrapping_add(border) {
            self.x0.wrapping_sub(border)
        } else {
            parent.x0
        };
        let y0 = if self.y0 > parent.y0.wrapping_add(border) {
            self.y0.wrapping_sub(border)
        } else {
            parent.y0
        };
        let x1 = if self.x1().wrapping_add(border) > parent.x1() {
            parent.x1()
        } else {
            self.x1().wrapping_add(border)
        };
        let y1 = if self.y1().wrapping_add(border) > parent.y1() {
            parent.y1()
        } else {
            self.y1().wrapping_add(border)
        };
        Self::new(x0, y0, x1.wrapping_sub(x0), y1.wrapping_sub(y0))
    }
}
