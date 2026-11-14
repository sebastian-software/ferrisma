// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! RenderPipelineStage/settings/default methods from render_pipeline/
//! render_pipeline_stage.{h,cc}. Owner-relative row descriptors replace float
//! pointers. RowBuffers resolves them under one exclusive storage borrow.
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::frame_header::FrameOrigin;
use crate::jxl::image::Plane;
use crate::jxl::memory_manager_internal::MemoryManagerPtr;

#[cfg(any(target_arch = "arm", target_arch = "aarch64"))]
pub const RENDER_PIPELINE_X_OFFSET: usize = 24;
#[cfg(not(any(target_arch = "arm", target_arch = "aarch64")))]
pub const RENDER_PIPELINE_X_OFFSET: usize = 32;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(i32)]
pub enum RenderPipelineChannelMode {
    Ignored = 0,
    InPlace = 1,
    InOut = 2,
    Input = 3,
}
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct Settings {
    pub border_x: usize,
    pub border_y: usize,
    pub shift_x: usize,
    pub shift_y: usize,
}
impl Settings {
    pub fn shift_x(shift: usize, border: usize) -> Self {
        Self {
            border_x: border,
            shift_x: shift,
            ..Self::default()
        }
    }
    pub fn shift_y(shift: usize, border: usize) -> Self {
        Self {
            border_y: border,
            shift_y: shift,
            ..Self::default()
        }
    }
    pub fn symmetric(shift: usize, border: usize) -> Self {
        Self {
            border_x: border,
            border_y: border,
            shift_x: shift,
            shift_y: shift,
        }
    }
    pub fn symmetric_border_only(border: usize) -> Self {
        Self::symmetric(0, border)
    }
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum RowStorage {
    Input,
    Output,
}
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Row {
    pub storage: RowStorage,
    pub channel: usize,
    pub y: usize,
    pub x: usize,
}
impl Row {
    pub fn offset(self, x: isize) -> Self {
        Self {
            x: self.x.wrapping_add_signed(x),
            ..self
        }
    }
}
pub type RowInfo = Vec<Vec<Row>>;
pub struct RowBuffers<'a> {
    pub input: &'a mut [Plane<f32>],
    pub output: &'a mut [Plane<f32>],
}
impl RowBuffers<'_> {
    pub fn row(&self, row: Row) -> &[f32] {
        let planes = match row.storage {
            RowStorage::Input => &*self.input,
            RowStorage::Output => &*self.output,
        };
        &planes[row.channel].row(row.y)[row.x..]
    }
    /// Borrow three nonoverlapping source row spans at the same time. Their
    /// storage, channel, and physical row offsets may differ. As for the
    /// source restricted pointers, all three spans must be disjoint.
    pub fn three_rows_mut(&mut self, rows: [Row; 3], length: usize) -> [&mut [f32]; 3] {
        let requests = rows.map(|r| (r.storage, r.channel, r.y, r.x));
        let mut order = [0, 1, 2];
        order.sort_by_key(|&i| {
            let (storage, channel, y, x) = requests[i];
            (usize::from(storage == RowStorage::Output), channel, y, x)
        });
        let mut result: [Option<&mut [f32]>; 3] = std::array::from_fn(|_| None);
        for (storage, planes) in [
            (RowStorage::Input, &mut *self.input),
            (RowStorage::Output, &mut *self.output),
        ] {
            for (channel, plane) in planes.iter_mut().enumerate() {
                let stride = plane.pixels_per_row() as usize;
                let height = plane.ysize();
                let mut tail = plane.pixels_mut().unwrap_or(&mut []);
                let mut previous_end = 0;
                for &i in &order {
                    let (s, c, y, x) = requests[i];
                    if s != storage || c != channel {
                        continue;
                    }
                    assert!(y < height && x.checked_add(length).is_some_and(|end| end <= stride));
                    let start = y
                        .checked_mul(stride)
                        .and_then(|v| v.checked_add(x))
                        .unwrap();
                    assert!(
                        start >= previous_end,
                        "restricted row spans must not overlap"
                    );
                    let (_, rest) = tail.split_at_mut(start - previous_end);
                    let (span, rest) = rest.split_at_mut(length);
                    result[i] = Some(span);
                    tail = rest;
                    previous_end = start + length;
                }
            }
        }
        result.map(|r| r.expect("row owner must exist"))
    }
    pub fn row_mut(&mut self, row: Row) -> &mut [f32] {
        let planes = match row.storage {
            RowStorage::Input => &mut *self.input,
            RowStorage::Output => &mut *self.output,
        };
        &mut planes[row.channel].row_mut(row.y)[row.x..]
    }
    /// The SIMD stages' row pointers (port ADR 0003, "SIMD row spans"):
    /// `R` read spans and `W` write spans resolved in place, at the same
    /// time, as slices of their planes' whole sample storage. A span's first
    /// sample is the source's `row + from`: its index is `y * stride +
    /// row.x + from`, so a negative `from` reaches into the preceding row's
    /// padding exactly as the source's pointer arithmetic does, and a span
    /// may extend into the row's padding (`BytesPerRow`) on the right.
    ///
    /// Write spans must be pairwise disjoint (the source's restricted output
    /// pointers), and a plane with a write span is not read through a read
    /// span (InOut stages read and write different buffers; in-place stages
    /// read through their write spans). Returns `None`, resolving nothing,
    /// when a span reaches outside its plane's storage (before the first
    /// row or past the last), when a plane is both read and written, or when
    /// two write spans overlap; [`super::simd_rows::with_spans`] then copies.
    pub fn spans<const R: usize, const W: usize>(
        &mut self,
        reads: &[Span; R],
        writes: &[Span; W],
    ) -> Option<SpanViews<'_, R, W>> {
        let mut read_out: [Option<&[f32]>; R] = [None; R];
        let mut write_out: [Option<&mut [f32]>; W] = std::array::from_fn(|_| None);
        for (storage, planes) in [
            (RowStorage::Input, &mut *self.input),
            (RowStorage::Output, &mut *self.output),
        ] {
            for (channel, plane) in planes.iter_mut().enumerate() {
                let on = |s: &Span| s.row.storage == storage && s.row.channel == channel;
                let written = writes.iter().any(on);
                if !written && !reads.iter().any(on) {
                    continue;
                }
                let stride = plane.pixels_per_row();
                let height = plane.ysize();
                // The index of the span's first sample in the plane storage.
                let start = |s: &Span, storage_len: usize| -> Option<usize> {
                    if s.row.y >= height {
                        return None;
                    }
                    let first = (s.row.y as isize)
                        .checked_mul(stride)?
                        .checked_add(s.row.x as isize)?
                        .checked_add(s.from)?;
                    let first = usize::try_from(first).ok()?;
                    (first.checked_add(s.len)? <= storage_len).then_some(first)
                };
                if written {
                    if reads.iter().any(on) {
                        return None;
                    }
                    let mut tail = plane.pixels_mut()?;
                    let total = tail.len();
                    // This plane's write spans in storage order, as in
                    // three_rows_mut, so that each is split off once.
                    let mut order: [(usize, usize); W] = [(usize::MAX, 0); W];
                    for (i, s) in writes.iter().enumerate() {
                        if on(s) {
                            order[i] = (start(s, total)?, i);
                        }
                    }
                    order.sort_unstable();
                    let mut previous_end = 0;
                    for &(first, i) in &order {
                        if first == usize::MAX {
                            break;
                        }
                        if first < previous_end {
                            return None;
                        }
                        let (_, rest) =
                            std::mem::take(&mut tail).split_at_mut(first - previous_end);
                        let (span, rest) = rest.split_at_mut(writes[i].len);
                        write_out[i] = Some(span);
                        tail = rest;
                        previous_end = first + writes[i].len;
                    }
                } else {
                    let pixels: &[f32] = plane.pixels()?;
                    for (i, s) in reads.iter().enumerate() {
                        if on(s) {
                            let first = start(s, pixels.len())?;
                            read_out[i] = Some(&pixels[first..first + s.len]);
                        }
                    }
                }
            }
        }
        if read_out.iter().any(Option::is_none) || write_out.iter().any(Option::is_none) {
            return None;
        }
        Some((read_out.map(Option::unwrap), write_out.map(Option::unwrap)))
    }
}
/// The read and write slices of [`RowBuffers::spans`].
pub type SpanViews<'a, const R: usize, const W: usize> = ([&'a [f32]; R], [&'a mut [f32]; W]);
/// One SIMD row span for [`RowBuffers::spans`]: `len` samples of `row`
/// from sample offset `from`, relative to the row descriptor.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct Span {
    pub row: Row,
    pub from: isize,
    pub len: usize,
}
pub trait RenderPipelineStage {
    fn settings(&self) -> Settings;
    #[expect(
        clippy::too_many_arguments,
        reason = "Retain the original stage row boundary, with explicit borrowed row storage."
    )]
    fn process_row(
        &mut self,
        input_rows: &RowInfo,
        output_rows: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        xextra_left: usize,
        xextra_right: usize,
        xsize: usize,
        xpos: usize,
        ypos: usize,
        thread_id: usize,
    ) -> Status;
    fn get_channel_mode(&self, c: usize) -> RenderPipelineChannelMode;
    fn get_name(&self) -> &str;
    // These are the complete original base implementations, not substitutes
    // for an unported concrete stage. Concrete overrides retain their own work.
    fn is_initialized(&self) -> Status {
        Ok(())
    }
    fn set_input_sizes(&mut self, _input_sizes: &[(usize, usize)]) -> Status {
        Ok(())
    }
    fn prepare_for_threads(&mut self, _num_threads: usize) -> Status {
        Ok(())
    }
    fn switch_to_image_dimensions(&self) -> bool {
        false
    }
    fn get_image_dimensions(
        &self,
        _xsize: &mut usize,
        _ysize: &mut usize,
        _origin: &mut FrameOrigin,
    ) {
    }
    fn process_padding_row(
        &mut self,
        _output_rows: &RowInfo,
        _buffers: &mut RowBuffers<'_>,
        _xsize: usize,
        _xpos: usize,
        _ypos: usize,
    ) {
    }
    fn get_input_row(&self, rows: &RowInfo, c: usize, offset: i32) -> Row {
        assert_ne!(self.get_channel_mode(c), RenderPipelineChannelMode::Ignored);
        let border = self.settings().border_y;
        assert!((offset as i64).abs() <= border as i64);
        rows[c][border.wrapping_add_signed(offset as isize)]
            .offset(RENDER_PIPELINE_X_OFFSET as isize)
    }
    fn get_output_row(&self, rows: &RowInfo, c: usize, offset: usize) -> Row {
        assert_eq!(self.get_channel_mode(c), RenderPipelineChannelMode::InOut);
        assert!(offset <= 1usize << self.settings().shift_y);
        rows[c][offset].offset(RENDER_PIPELINE_X_OFFSET as isize)
    }
}
/// Complete original WriteToImage3FStage and factory from stage_write.cc.
pub struct WriteToImage3FStage<'a> {
    memory_manager: MemoryManagerPtr,
    image: Image3WriteTarget<'a>,
}
enum Image3WriteTarget<'a> {
    Borrowed(&'a mut crate::jxl::image::Image3<f32>),
    Owned(std::rc::Rc<std::cell::RefCell<crate::jxl::image::Image3<f32>>>),
    DcFrame(crate::jxl::passes_state::SharedDcFrames, usize),
}
impl Image3WriteTarget<'_> {
    fn with_storage(
        &mut self,
        f: impl FnOnce(&mut crate::jxl::image::Image3<f32>) -> Status,
    ) -> Status {
        match self {
            Self::Borrowed(image) => f(image),
            Self::Owned(owner) => f(&mut owner.borrow_mut()),
            Self::DcFrame(owner, index) => f(&mut owner.borrow_mut()[*index]),
        }
    }
}
impl<'a> WriteToImage3FStage<'a> {
    pub fn new(image: &'a mut crate::jxl::image::Image3<f32>) -> Self {
        Self {
            memory_manager: MemoryManagerPtr::NATIVE,
            image: Image3WriteTarget::Borrowed(image),
        }
    }
    pub fn from_owned(
        owner: std::rc::Rc<std::cell::RefCell<crate::jxl::image::Image3<f32>>>,
    ) -> Self {
        Self {
            memory_manager: MemoryManagerPtr::NATIVE,
            image: Image3WriteTarget::Owned(owner),
        }
    }
    /// Retain the decoder's actual DC array, resolving its slot for every call.
    pub fn from_dc_frame(owner: crate::jxl::passes_state::SharedDcFrames, index: usize) -> Self {
        assert!(index < 4);
        Self {
            memory_manager: MemoryManagerPtr::NATIVE,
            image: Image3WriteTarget::DcFrame(owner, index),
        }
    }
    /// The source factory's memory_manager argument; the constructors above
    /// select the native allocator boundary.
    pub fn with_memory_manager(mut self, memory_manager: MemoryManagerPtr) -> Self {
        self.memory_manager = memory_manager;
        self
    }
}
impl RenderPipelineStage for WriteToImage3FStage<'_> {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn set_input_sizes(&mut self, sizes: &[(usize, usize)]) -> Status {
        if sizes.len() < 3 {
            return Err(StatusCode::GenericError);
        }
        for c in 1..3 {
            if sizes[c].0 != sizes[0].0 || sizes[c].1 != sizes[0].1 {
                return Err(StatusCode::GenericError);
            }
        }
        let image =
            crate::jxl::image::Image3::create_in(sizes[0].0, sizes[0].1, self.memory_manager)?;
        self.image.with_storage(|storage| {
            storage.assign(image);
            Ok(())
        })
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
        self.image.with_storage(|image| {
            for (c, from) in rows.into_iter().enumerate() {
                image.plane_row_mut(c, y)[x..x + width]
                    .copy_from_slice(&buffers.row(from)[..width]);
            }
            Ok(())
        })
    }
    fn get_channel_mode(&self, c: usize) -> RenderPipelineChannelMode {
        if c < 3 {
            RenderPipelineChannelMode::Input
        } else {
            RenderPipelineChannelMode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "WriteI3F"
    }
}

/// Complete original WriteToImageBundleStage/factory from stage_write.cc.
pub struct WriteToImageBundleStage<'b, 'm> {
    image_bundle: ImageBundleTarget<'b, 'm>,
    color_encoding: crate::jxl::color_encoding_internal::ColorEncoding,
}
enum ImageBundleTarget<'b, 'm> {
    Borrowed(&'b mut crate::jxl::image_bundle::ImageBundle<'m>),
    Shared(&'b std::cell::RefCell<crate::jxl::image_bundle::ImageBundle<'m>>),
    Owned(std::rc::Rc<std::cell::RefCell<crate::jxl::image_bundle::ImageBundle<'m>>>),
    Pointer(std::rc::Weak<std::cell::RefCell<crate::jxl::image_bundle::ImageBundle<'m>>>),
    Reference(crate::jxl::passes_state::SharedReferenceFrames<'m>, usize),
}
impl ImageBundleTarget<'_, '_> {
    fn with_storage(
        &mut self,
        f: impl FnOnce(&mut crate::jxl::image_bundle::ImageBundle<'_>) -> Status,
    ) -> Status {
        match self {
            Self::Borrowed(storage) => f(storage),
            Self::Shared(owner) => f(&mut owner.borrow_mut()),
            Self::Owned(owner) => f(&mut owner.borrow_mut()),
            Self::Pointer(pointer) => f(&mut pointer
                .upgrade()
                .expect("the source's ImageBundle pointer is live while the pipeline runs")
                .borrow_mut()),
            Self::Reference(owner, index) => f(owner.borrow_mut()[*index]
                .frame
                .as_deref_mut()
                .expect("source requires an existing reference bundle")),
        }
    }
}
impl<'b, 'm> WriteToImageBundleStage<'b, 'm> {
    pub fn new(
        image_bundle: &'b mut crate::jxl::image_bundle::ImageBundle<'m>,
        color_encoding: &crate::jxl::color_encoding_internal::ColorEncoding,
    ) -> Self {
        Self {
            image_bundle: ImageBundleTarget::Borrowed(image_bundle),
            color_encoding: color_encoding.clone(),
        }
    }
    /// The sequential frame decoder and stage retain the same external owner.
    /// Each source call borrows it only for that call; no callback runs while
    /// this write stage holds its dynamic exclusive borrow.
    pub fn from_shared(
        owner: &'b std::cell::RefCell<crate::jxl::image_bundle::ImageBundle<'m>>,
        color_encoding: &crate::jxl::color_encoding_internal::ColorEncoding,
    ) -> Self {
        Self {
            image_bundle: ImageBundleTarget::Shared(owner),
            color_encoding: color_encoding.clone(),
        }
    }
    /// Retain an external bundle owner without borrowing the containing decoder.
    pub fn from_owned(
        owner: std::rc::Rc<std::cell::RefCell<crate::jxl::image_bundle::ImageBundle<'m>>>,
        color_encoding: &crate::jxl::color_encoding_internal::ColorEncoding,
    ) -> Self {
        Self {
            image_bundle: ImageBundleTarget::Owned(owner),
            color_encoding: color_encoding.clone(),
        }
    }
    /// The source's non-owning ImageBundle pointer to a bundle owned
    /// elsewhere. The stage does not extend the bundle's lifetime, so its
    /// release stays at the owner's checkpoint.
    pub fn from_pointer(
        owner: &std::rc::Rc<std::cell::RefCell<crate::jxl::image_bundle::ImageBundle<'m>>>,
        color_encoding: &crate::jxl::color_encoding_internal::ColorEncoding,
    ) -> Self {
        Self {
            image_bundle: ImageBundleTarget::Pointer(std::rc::Rc::downgrade(owner)),
            color_encoding: color_encoding.clone(),
        }
    }
    /// Resolve the existing reference slot under a short exclusive borrow.
    pub fn from_reference(
        owner: crate::jxl::passes_state::SharedReferenceFrames<'m>,
        index: usize,
        color_encoding: &crate::jxl::color_encoding_internal::ColorEncoding,
    ) -> Self {
        assert!(index < 4);
        Self {
            image_bundle: ImageBundleTarget::Reference(owner, index),
            color_encoding: color_encoding.clone(),
        }
    }
}
impl RenderPipelineStage for WriteToImageBundleStage<'_, '_> {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn set_input_sizes(&mut self, sizes: &[(usize, usize)]) -> Status {
        if sizes.len() < 3 {
            return Err(StatusCode::GenericError);
        }
        for c in 1..sizes.len() {
            if sizes[c].0 != sizes[0].0 {
                return Err(StatusCode::GenericError);
            }
            if sizes[c].1 != sizes[0].1 {
                return Err(StatusCode::GenericError);
            }
        }
        self.image_bundle.with_storage(|storage| {
            let memory_manager = storage.memory_manager();
            let mut tmp = Some(crate::jxl::image::Image3::<f32>::create_in(
                sizes[0].0,
                sizes[0].1,
                memory_manager,
            )?);
            storage.set_from_image(&mut tmp, &self.color_encoding)?;
            crate::jxl::base::libcpp_vector::clear(storage.extra_channels_mut());
            for &(w, h) in sizes.iter().skip(3) {
                let ch = Plane::create_in(w, h, 0, memory_manager)?;
                storage.extra_channels_mut().push(ch);
            }
            Ok(())
        })
    }
    fn process_row(
        &mut self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        x: usize,
        y: usize,
        _thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        // Settings are permanently default and all channels have kInput mode:
        // this is the base GetInputRow calculation under those source values.
        let row = |c: usize| input[c][0].offset(RENDER_PIPELINE_X_OFFSET as isize);
        self.image_bundle.with_storage(|storage| {
            for c in 0..3 {
                storage.color_mut().plane_row_mut(c, y)[x..x + width]
                    .copy_from_slice(&buffers.row(row(c))[..width]);
            }
            for ec in 0..storage.extra_channels().len() {
                if storage.extra_channels()[ec].xsize() < x.wrapping_add(width) {
                    return Err(StatusCode::GenericError);
                }
                storage.extra_channels_mut()[ec].row_mut(y)
                    [x.wrapping_sub(left)..x.wrapping_sub(left).wrapping_add(width)]
                    .copy_from_slice(&buffers.row(row(3 + ec))[..width]);
            }
            Ok(())
        })
    }
    fn get_channel_mode(&self, _c: usize) -> RenderPipelineChannelMode {
        RenderPipelineChannelMode::Input
    }
    fn get_name(&self) -> &str {
        "WriteIB"
    }
}
