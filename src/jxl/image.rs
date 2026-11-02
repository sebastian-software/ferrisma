// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Planar storage from lib/jxl/image.{h,cc}. Rust owns initialized aligned
//! memory; sealed numeric samples allow safe typed views of those bytes.
//! Rows expose physical padding, while xsize remains the valid sample count.
//! C allocator callbacks and MSan-only padding instrumentation remain unported.

use super::base::status::{Status, StatusCode, StatusOr};
use super::memory_manager_internal::{AlignedMemory, bytes_per_row};
use std::marker::PhantomData;
use std::mem::{align_of, size_of, swap};

mod sealed {
    pub trait Sample {}
    impl Sample for u8 {}
    impl Sample for i8 {}
    impl Sample for u16 {}
    impl Sample for i16 {}
    impl Sample for u32 {}
    impl Sample for i32 {}
    impl Sample for u64 {}
    impl Sample for i64 {}
    impl Sample for f32 {}
    impl Sample for f64 {}
}
/// Source POD numeric sample instantiations. All bit patterns are valid,
/// alignment is at most eight, and every sample occupies 1, 2, 4, or 8 bytes.
pub trait Sample: sealed::Sample + Copy + Default {}
impl Sample for u8 {}
impl Sample for i8 {}
impl Sample for u16 {}
impl Sample for i16 {}
impl Sample for u32 {}
impl Sample for i32 {}
impl Sample for u64 {}
impl Sample for i64 {}
impl Sample for f32 {}
impl Sample for f64 {}

#[derive(Default)]
struct PlaneBase {
    xsize: u32,
    ysize: u32,
    orig_xsize: u32,
    orig_ysize: u32,
    bytes_per_row: usize,
    bytes: Option<AlignedMemory>,
    sizeof_t: usize,
}
impl PlaneBase {
    fn new(xsize: u32, ysize: u32, sizeof_t: usize) -> Self {
        Self {
            xsize,
            ysize,
            orig_xsize: xsize,
            orig_ysize: ysize,
            sizeof_t,
            ..Self::default()
        }
    }
    fn allocate(&mut self, pre_padding: usize, have_memory_manager: bool) -> Status {
        if self.bytes.is_some() || self.bytes_per_row != 0 {
            return Err(StatusCode::GenericError);
        }
        self.bytes_per_row = bytes_per_row(self.xsize as usize, self.sizeof_t)?;
        if self.xsize == 0 || self.ysize == 0 {
            return Ok(());
        }
        let total = (self.ysize as usize)
            .checked_mul(self.bytes_per_row)
            .ok_or(StatusCode::GenericError)?;
        // A null source memory manager fails only when allocation is reached.
        // This preserves reallocating a channel whose previous plane was empty.
        if !have_memory_manager {
            return Err(StatusCode::GenericError);
        }
        self.bytes = Some(AlignedMemory::create(
            total,
            pre_padding.wrapping_mul(self.sizeof_t),
        )?);
        Ok(())
    }
    fn shrink_to(&mut self, xsize: usize, ysize: usize) -> Status {
        if xsize > self.orig_xsize as usize || ysize > self.orig_ysize as usize {
            return Err(StatusCode::GenericError);
        }
        self.xsize = xsize as u32;
        self.ysize = ysize as u32;
        Ok(())
    }
    fn swap(&mut self, other: &mut Self) {
        swap(&mut self.xsize, &mut other.xsize);
        swap(&mut self.ysize, &mut other.ysize);
        swap(&mut self.orig_xsize, &mut other.orig_xsize);
        swap(&mut self.orig_ysize, &mut other.orig_ysize);
        swap(&mut self.bytes_per_row, &mut other.bytes_per_row);
        swap(&mut self.bytes, &mut other.bytes);
        // The source does not swap sizeof_t.
    }
}
pub struct Plane<T: Sample> {
    base: PlaneBase,
    sample: PhantomData<T>,
}
impl<T: Sample> Default for Plane<T> {
    fn default() -> Self {
        Self {
            base: PlaneBase::default(),
            sample: PhantomData,
        }
    }
}
impl<T: Sample> Plane<T> {
    pub const NUM_PLANES: usize = 1;
    pub fn create(xsize: usize, ysize: usize) -> StatusOr<Self> {
        Self::create_in(xsize, ysize, 0, true)
    }
    pub fn create_with_pre_padding(
        xsize: usize,
        ysize: usize,
        pre_padding: usize,
    ) -> StatusOr<Self> {
        Self::create_in(xsize, ysize, pre_padding, true)
    }
    pub(crate) fn create_in(
        xsize: usize,
        ysize: usize,
        pre_padding: usize,
        have_memory_manager: bool,
    ) -> StatusOr<Self> {
        let x = xsize as u32;
        let y = ysize as u32;
        if x as usize != xsize || y as usize != ysize {
            return Err(StatusCode::GenericError);
        }
        let mut value = Self {
            base: PlaneBase::new(x, y, size_of::<T>()),
            sample: PhantomData,
        };
        value.base.allocate(pre_padding, have_memory_manager)?;
        Ok(value)
    }
    pub fn xsize(&self) -> usize {
        self.base.xsize as usize
    }
    pub fn ysize(&self) -> usize {
        self.base.ysize as usize
    }
    pub fn bytes_per_row(&self) -> usize {
        self.base.bytes_per_row
    }
    pub fn pixels_per_row(&self) -> isize {
        (self.bytes_per_row() / size_of::<T>()) as isize
    }
    pub fn bytes(&self) -> Option<&[u8]> {
        self.base.bytes.as_ref().map(AlignedMemory::bytes)
    }
    pub fn bytes_mut(&mut self) -> Option<&mut [u8]> {
        self.base.bytes.as_mut().map(AlignedMemory::bytes_mut)
    }
    /// Complete initialized sample storage, including physical row padding.
    /// Used to represent source pointers with relative row/sample offsets.
    pub fn pixels(&self) -> Option<&[T]> {
        self.base
            .bytes
            .as_ref()
            .map(|memory| self.row_at(0, memory.bytes().len()))
    }
    /// Complete initialized sample storage under an exclusive owner borrow.
    pub fn pixels_mut(&mut self) -> Option<&mut [T]> {
        let length = self.base.bytes.as_ref()?.bytes().len();
        Some(self.row_at_mut(0, length))
    }
    pub(crate) fn have_memory_manager(&self) -> bool {
        self.base.bytes.is_some()
    }
    fn row_at(&self, offset: usize, length: usize) -> &[T] {
        let Some(memory) = self.base.bytes.as_ref() else {
            assert_eq!(length, 0);
            return &[];
        };
        assert_eq!(offset % align_of::<T>(), 0);
        assert_eq!(length % size_of::<T>(), 0);
        let end = offset.checked_add(length).expect("row extent overflow");
        let bytes = &memory.bytes()[offset..end];
        // SAFETY: AlignedMemory owns initialized storage aligned to 128 bytes.
        // The checked slice bounds and aligned offset cover whole sealed Sample
        // elements, whose alignments are <=8 and whose bit patterns are all
        // valid. Its shared owner borrow keeps storage live and prevents mutation.
        unsafe { std::slice::from_raw_parts(bytes.as_ptr().cast::<T>(), length / size_of::<T>()) }
    }
    fn row_at_mut(&mut self, offset: usize, length: usize) -> &mut [T] {
        let Some(memory) = self.base.bytes.as_mut() else {
            assert_eq!(length, 0);
            return &mut [];
        };
        assert_eq!(offset % align_of::<T>(), 0);
        assert_eq!(length % size_of::<T>(), 0);
        let end = offset.checked_add(length).expect("row extent overflow");
        let bytes = &mut memory.bytes_mut()[offset..end];
        // SAFETY: Same allocation, initialization, sample-validity, alignment,
        // and checked-extent proof as row_at. The exclusive owner borrow rules
        // out other byte or typed views for the returned slice's lifetime.
        unsafe {
            std::slice::from_raw_parts_mut(bytes.as_mut_ptr().cast::<T>(), length / size_of::<T>())
        }
    }
    pub fn row(&self, y: usize) -> &[T] {
        assert!(y < self.ysize());
        self.row_at(y.wrapping_mul(self.bytes_per_row()), self.bytes_per_row())
    }
    pub fn row_mut(&mut self, y: usize) -> &mut [T] {
        assert!(y < self.ysize());
        self.row_at_mut(y.wrapping_mul(self.bytes_per_row()), self.bytes_per_row())
    }
    pub fn shrink_to(&mut self, xsize: usize, ysize: usize) -> Status {
        self.base.shrink_to(xsize, ysize)
    }
    pub fn swap(&mut self, other: &mut Self) {
        self.base.swap(&mut other.base);
    }
}
pub type ImageSB = Plane<i8>;
pub type ImageB = Plane<u8>;
pub type ImageS = Plane<i16>;
pub type ImageU = Plane<u16>;
pub type ImageI = Plane<i32>;
pub type ImageF = Plane<f32>;

pub struct Image3<T: Sample> {
    planes: [Plane<T>; 3],
}
impl<T: Sample> Default for Image3<T> {
    fn default() -> Self {
        Self {
            planes: std::array::from_fn(|_| Plane::default()),
        }
    }
}
impl<T: Sample> Image3<T> {
    pub const NUM_PLANES: usize = 3;
    pub fn create(xsize: usize, ysize: usize) -> StatusOr<Self> {
        Self::create_in(xsize, ysize, true)
    }
    pub(crate) fn create_in(
        xsize: usize,
        ysize: usize,
        have_memory_manager: bool,
    ) -> StatusOr<Self> {
        let plane0 = Plane::create_in(xsize, ysize, 0, have_memory_manager)?;
        let plane1 = Plane::create_in(xsize, ysize, 0, have_memory_manager)?;
        let plane2 = Plane::create_in(xsize, ysize, 0, have_memory_manager)?;
        Ok(Self {
            planes: [plane0, plane1, plane2],
        })
    }
    pub fn plane(&self, c: usize) -> &Plane<T> {
        &self.planes[c]
    }
    pub fn plane_mut(&mut self, c: usize) -> &mut Plane<T> {
        &mut self.planes[c]
    }
    pub(crate) fn planes_mut(&mut self) -> &mut [Plane<T>; 3] {
        &mut self.planes
    }
    pub fn plane_row(&self, c: usize, y: usize) -> &[T] {
        assert!(c < 3 && y < self.ysize());
        let stride = self.planes[0].bytes_per_row();
        self.planes[c].row_at(y.wrapping_mul(stride), stride)
    }
    pub fn plane_row_mut(&mut self, c: usize, y: usize) -> &mut [T] {
        assert!(c < 3 && y < self.ysize());
        let stride = self.planes[0].bytes_per_row();
        self.planes[c].row_at_mut(y.wrapping_mul(stride), stride)
    }
    pub fn swap(&mut self, other: &mut Self) {
        for c in 0..3 {
            other.planes[c].swap(&mut self.planes[c]);
        }
    }
    pub fn shrink_to(&mut self, xsize: usize, ysize: usize) -> Status {
        for plane in &mut self.planes {
            plane.shrink_to(xsize, ysize)?;
        }
        Ok(())
    }
    pub fn xsize(&self) -> usize {
        self.planes[0].xsize()
    }
    pub fn ysize(&self) -> usize {
        self.planes[0].ysize()
    }
    pub fn bytes_per_row(&self) -> usize {
        self.planes[0].bytes_per_row()
    }
    pub fn pixels_per_row(&self) -> isize {
        self.planes[0].pixels_per_row()
    }
}
pub type Image3B = Image3<u8>;
pub type Image3S = Image3<i16>;
pub type Image3U = Image3<u16>;
pub type Image3I = Image3<i32>;
pub type Image3F = Image3<f32>;
