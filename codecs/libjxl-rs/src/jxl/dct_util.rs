// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! ACImage/ACImageT storage and virtual operations from lib/jxl/dct_util.h.
//! A closed enum expresses the two scalar types admitted by the source static
//! assertion. Typed row borrows replace its pointer unions and virtual dispatch.
use super::base::status::StatusOr;
use super::image::Image3;
use super::image_ops::{zero_fill_image, zero_fill_image3};
use super::memory_manager_internal::MemoryManagerPtr;
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
        Self::make_in(MemoryManagerPtr::NATIVE, kind, xsize, ysize)
    }
    /// The source ACImageT<T>::Make(memory_manager, xsize, ysize).
    pub fn make_in(
        memory_manager: MemoryManagerPtr,
        kind: AcType,
        xsize: usize,
        ysize: usize,
    ) -> StatusOr<Box<Self>> {
        let mut result = Box::new(Self::empty(kind));
        match result.as_mut() {
            Self::I16(img) => img.assign(Image3::create_in(xsize, ysize, memory_manager)?),
            Self::I32(img) => img.assign(Image3::create_in(xsize, ysize, memory_manager)?),
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
    /// Row `group` of the three planes: the group's accumulated coefficients
    /// (`PlaneRow(c, group_idx, 0)`), or Empty for an empty image.
    pub fn group_rows(&mut self, group: usize) -> GroupCoefficients<'_> {
        if self.is_empty() {
            return GroupCoefficients::Empty(self.image_type());
        }
        match self {
            Self::I16(image) => GroupCoefficients::Rows(
                image
                    .planes_mut()
                    .each_mut()
                    .map(|plane| AcPtrMut::I16(plane.row_mut(group))),
            ),
            Self::I32(image) => GroupCoefficients::Rows(
                image
                    .planes_mut()
                    .each_mut()
                    .map(|plane| AcPtrMut::I32(plane.row_mut(group))),
            ),
        }
    }
    /// Every row of the three planes, by group: disjoint views that let each
    /// group task own its group's coefficients (port ADR 0003, "Concurrent
    /// decoder tasks"); empty for an empty image.
    pub fn split_group_rows(&mut self) -> Vec<[AcPtrMut<'_>; 3]> {
        fn rows<T: super::image::Sample>(
            plane: &mut super::image::Plane<T>,
        ) -> std::slice::ChunksMut<'_, T> {
            let stride = plane.pixels_per_row() as usize;
            plane
                .pixels_mut()
                .expect("a nonempty AC image is allocated")
                .chunks_mut(stride)
        }
        if self.is_empty() {
            return Vec::new();
        }
        let ysize = match self {
            Self::I16(image) => image.ysize(),
            Self::I32(image) => image.ysize(),
        };
        let mut result = Vec::with_capacity(ysize);
        match self {
            Self::I16(image) => {
                let [a, b, c] = image.planes_mut().each_mut();
                for ((a, b), c) in rows(a).zip(rows(b)).zip(rows(c)).take(ysize) {
                    result.push([AcPtrMut::I16(a), AcPtrMut::I16(b), AcPtrMut::I16(c)]);
                }
            }
            Self::I32(image) => {
                let [a, b, c] = image.planes_mut().each_mut();
                for ((a, b), c) in rows(a).zip(rows(b)).zip(rows(c)).take(ysize) {
                    result.push([AcPtrMut::I32(a), AcPtrMut::I32(b), AcPtrMut::I32(c)]);
                }
            }
        }
        result
    }
}

/// One group's accumulated AC coefficients: its row of each plane of the AC
/// image, or Empty when the frame keeps none (one pass, no JPEG).
pub enum GroupCoefficients<'a> {
    Empty(AcType),
    Rows([AcPtrMut<'a>; 3]),
}
impl GroupCoefficients<'_> {
    pub fn image_type(&self) -> AcType {
        match self {
            Self::Empty(kind) => *kind,
            Self::Rows([AcPtrMut::I16(_), ..]) => AcType::I16,
            Self::Rows(_) => AcType::I32,
        }
    }
    pub fn is_empty(&self) -> bool {
        matches!(self, Self::Empty(_))
    }
    /// The rows from sample `offset`, as `PlaneRow(c, group_idx, 0) + offset`.
    pub fn from_offset(&mut self, offset: usize) -> [AcPtrMut<'_>; 3] {
        let Self::Rows(rows) = self else {
            panic!("the source reads accumulated coefficients only when it keeps them");
        };
        rows.each_mut().map(|row| match row {
            AcPtrMut::I16(values) => AcPtrMut::I16(&mut values[offset..]),
            AcPtrMut::I32(values) => AcPtrMut::I32(&mut values[offset..]),
        })
    }
}
