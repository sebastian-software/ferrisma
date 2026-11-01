// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ACImage/ACImageT storage and virtual operations from lib/jxl/dct_util.h.
//! A closed enum expresses the two scalar types admitted by the source static
//! assertion. Typed row borrows replace its pointer unions and virtual dispatch.
use super::base::status::StatusOr;
use super::image::Image3;
use super::image_ops::{zero_fill_image, zero_fill_image3};
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum AcType {
    I16 = 0,
    I32 = 1,
}
pub enum AcPtrMut<'a> {
    I16(&'a mut [i16]),
    I32(&'a mut [i32]),
}
pub enum ConstAcPtr<'a> {
    I16(&'a [i16]),
    I32(&'a [i32]),
}
pub enum AcImage {
    I16(Image3<i16>),
    I32(Image3<i32>),
}
impl AcImage {
    pub fn empty(kind: AcType) -> Self {
        match kind {
            AcType::I16 => Self::I16(Image3::default()),
            AcType::I32 => Self::I32(Image3::default()),
        }
    }
    pub fn make(kind: AcType, xsize: usize, ysize: usize) -> StatusOr<Box<Self>> {
        let mut result = Box::new(Self::empty(kind));
        match result.as_mut() {
            Self::I16(img) => *img = Image3::create(xsize, ysize)?,
            Self::I32(img) => *img = Image3::create(xsize, ysize)?,
        }
        Ok(result)
    }
    pub fn image_type(&self) -> AcType {
        match self {
            Self::I16(_) => AcType::I16,
            Self::I32(_) => AcType::I32,
        }
    }
    pub fn plane_row(&self, c: usize, y: usize, xbase: usize) -> ConstAcPtr<'_> {
        match self {
            Self::I16(img) => ConstAcPtr::I16(&img.plane_row(c, y)[xbase..]),
            Self::I32(img) => ConstAcPtr::I32(&img.plane_row(c, y)[xbase..]),
        }
    }
    pub fn plane_row_mut(&mut self, c: usize, y: usize, xbase: usize) -> AcPtrMut<'_> {
        match self {
            Self::I16(img) => AcPtrMut::I16(&mut img.plane_row_mut(c, y)[xbase..]),
            Self::I32(img) => AcPtrMut::I32(&mut img.plane_row_mut(c, y)[xbase..]),
        }
    }
    pub fn pixels_per_row(&self) -> usize {
        match self {
            Self::I16(img) => img.plane(0).pixels_per_row() as usize,
            Self::I32(img) => img.plane(0).pixels_per_row() as usize,
        }
    }
    pub fn zero_fill(&mut self) {
        match self {
            Self::I16(img) => zero_fill_image3(img),
            Self::I32(img) => zero_fill_image3(img),
        }
    }
    pub fn zero_fill_plane(&mut self, c: usize) {
        match self {
            Self::I16(img) => zero_fill_image(img.plane_mut(c)),
            Self::I32(img) => zero_fill_image(img.plane_mut(c)),
        }
    }
    pub fn is_empty(&self) -> bool {
        match self {
            Self::I16(img) => img.xsize() == 0 || img.ysize() == 0,
            Self::I32(img) => img.xsize() == 0 || img.ysize() == 0,
        }
    }
}
