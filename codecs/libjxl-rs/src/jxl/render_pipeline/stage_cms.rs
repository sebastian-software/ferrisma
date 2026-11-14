// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete CmsStage/factory from render_pipeline/stage_cms.cc. Exclusive row
//! borrows replace restrict pointers. Unsafe construction establishes the CMS
//! implementation/data lifetime contract; methods check source programmer
//! preconditions before foreign buffer access. No default CMS is substituted.
use super::render_pipeline_stage::{
    RenderPipelineChannelMode as Mode, RenderPipelineStage, RowBuffers, RowInfo, Settings,
};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::color_encoding_internal::{ColorEncoding, ColorSpaceTransform};
use crate::jxl::dec_xyb::OutputEncodingInfo;

struct CmsStage {
    output_encoding_info: OutputEncodingInfo,
    xsize: Option<usize>,
    // Cells: PrepareForThreads and rows run through `&self`. The transform's
    // Run needs exclusive access, so CMS rows are not shared between threads
    // (port ADR 0003, "Concurrent decoder tasks").
    color_space_transform: std::cell::RefCell<Option<Box<ColorSpaceTransform>>>,
    c_src: Option<ColorEncoding>,
    // Derived native safety state. Source callers must set input sizes before
    // preparation and honor the CMS initialization's thread/pixel bounds.
    initialized: std::cell::Cell<Option<(usize, usize)>>,
}
impl CmsStage {
    fn new(output_encoding_info: OutputEncodingInfo, linear: bool) -> Self {
        let mut stage = Self {
            output_encoding_info,
            xsize: None,
            color_space_transform: std::cell::RefCell::new(None),
            c_src: Some(ColorEncoding::default()),
            initialized: std::cell::Cell::new(None),
        };
        stage.c_src = Some(if linear {
            stage.output_encoding_info.linear_color_encoding.clone()
        } else {
            stage.output_encoding_info.orig_color_encoding.clone()
        });
        stage
    }
    fn source(&self) -> &ColorEncoding {
        self.c_src.as_ref().expect("live source profile")
    }
    fn is_needed(&self) -> bool {
        let src = self.source();
        let channels_src = if src.is_cmyk() { 4 } else { src.channels() };
        let channels_dst = self.output_encoding_info.color_encoding.channels();
        let not_mixing = channels_src == channels_dst || (channels_src == 4 && channels_dst == 3);
        self.output_encoding_info.cms_set
            && !src.same_color_encoding(&self.output_encoding_info.color_encoding)
            && not_mixing
    }
}
impl RenderPipelineStage for CmsStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_channel_mode(&self, c: usize) -> Mode {
        if c < if self.source().is_cmyk() { 4 } else { 3 } {
            Mode::InPlace
        } else {
            Mode::Ignored
        }
    }
    fn get_name(&self) -> &str {
        "Cms"
    }
    fn set_input_sizes(&mut self, sizes: &[(usize, usize)]) -> Status {
        if sizes.len() < 3 {
            return Err(StatusCode::GenericError);
        }
        for size in &sizes[1..] {
            if size.0 != sizes[0].0 || size.1 != sizes[0].1 {
                return Err(StatusCode::GenericError);
            }
        }
        self.xsize = Some(sizes[0].0);
        Ok(())
    }
    fn prepare_for_threads(&self, threads: usize) -> Status {
        *self.color_space_transform.borrow_mut() = Some(Box::new(ColorSpaceTransform::new(
            &self.output_encoding_info.color_management_system,
        )));
        self.initialized.set(None);
        let width = self
            .xsize
            .expect("source requires SetInputSizes before CMS preparation");
        // SAFETY: Only unsafe get_cms_stage constructs this type. Its caller
        // keeps the callback interface/data valid through stage drop and
        // guarantees cms_interface.h behavior. The borrowed profiles live for
        // init; ColorSpaceTransform retains their copied ICC owners afterward.
        let status = unsafe {
            self.color_space_transform
                .borrow_mut()
                .as_mut()
                .unwrap()
                .init(
                    self.c_src.as_ref().unwrap(),
                    &self.output_encoding_info.color_encoding,
                    self.output_encoding_info.desired_intensity_target,
                    width,
                    threads,
                )
        };
        if status.is_ok() {
            self.initialized.set(Some((threads, width)));
        }
        status
    }
    fn process_row(
        &self,
        input: &RowInfo,
        _output: &RowInfo,
        buffers: &mut RowBuffers<'_>,
        left: usize,
        right: usize,
        width: usize,
        _x: usize,
        _y: usize,
        thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        if width > self.xsize.expect("source requires initialized input sizes") {
            return Err(StatusCode::GenericError);
        }
        let gray_src = self.source().channels() == 1;
        let gray_dst = self.output_encoding_info.color_encoding.channels() == 1;
        let (threads, allocated_width) = self
            .initialized
            .get()
            .expect("source requires successful CMS preparation");
        assert!(
            thread < threads && width <= allocated_width,
            "source CMS initialization bounds"
        );
        let mut transform_guard = self.color_space_transform.borrow_mut();
        let transform = transform_guard.as_mut().unwrap();
        assert!(
            transform.has_context(),
            "source requires a valid CMS context"
        );
        // SAFETY: Successful preparation established the context and callback
        // bounds checked above. Unsafe factory construction guarantees valid
        // aligned buffers of the initialized sizes, disjoint from row planes.
        // Source/destination CMS buffers may alias only per cms_interface.h;
        // raw pointers avoid creating conflicting Rust references to them.
        let (src, dst) = unsafe { (transform.buf_src(thread), transform.buf_dst(thread)) };
        assert!(
            width == 0 || (!src.is_null() && !dst.is_null()),
            "source requires CMS buffers"
        );
        let channels = if self.source().is_cmyk() {
            4
        } else if gray_src {
            1
        } else {
            3
        };
        let mut rows = [None; 4];
        for (c, row) in rows.iter_mut().enumerate().take(channels) {
            *row = Some(self.get_input_row(input, c, 0));
        }
        if gray_src && !self.source().is_cmyk() {
            let row = &buffers.row(rows[0].unwrap())[..width];
            if width != 0 {
                // SAFETY: The row is initialized/in-bounds. The factory's CMS
                // contract guarantees writable, aligned, disjoint foreign
                // storage for width source samples; the copy preserves bits.
                unsafe { std::ptr::copy_nonoverlapping(row.as_ptr(), src, width) };
            }
        } else {
            for x in 0..width {
                for (c, row) in rows.iter().enumerate().take(channels) {
                    let value = buffers.row(row.unwrap())[x];
                    // SAFETY: Interleaved offset is below width*source_channels,
                    // within the CMS buffer guaranteed at unsafe construction.
                    unsafe { src.add(channels * x + c).write(value) };
                }
            }
        }
        // SAFETY: The input has just been initialized, the output covers its
        // profile's channel count, and the context/thread/bounds are valid.
        // The CMS implementation guarantees initialized output on success.
        unsafe { transform.run(thread, src, dst, width) }?;
        if gray_dst {
            for c in 0..3 {
                let row = self.get_input_row(input, c, 0);
                let output = &mut buffers.row_mut(row)[..width];
                if width != 0 {
                    // SAFETY: Run initialized width gray samples. Destination
                    // is an exclusive in-bounds plane borrow disjoint from the
                    // CMS buffer, as required by the original memcpy contract.
                    unsafe { std::ptr::copy_nonoverlapping(dst, output.as_mut_ptr(), width) };
                }
            }
        } else {
            let rows: [_; 3] = std::array::from_fn(|c| self.get_input_row(input, c, 0));
            for x in 0..width {
                for (c, row) in rows.iter().enumerate() {
                    // SAFETY: Successful Run initialized width*3 RGB samples;
                    // this scalar read is in the live aligned CMS output buffer.
                    let value = unsafe { dst.add(3 * x + c).read() };
                    buffers.row_mut(*row)[x] = value;
                }
            }
        }
        Ok(())
    }
}
impl Drop for CmsStage {
    fn drop(&mut self) {
        // Match original reverse member destruction. The transform retains its
        // independent ICC caches through destroy; output_encoding_info drops
        // automatically after these owned members have been released.
        drop(self.c_src.take());
        drop(self.color_space_transform.get_mut().take());
    }
}
/// Construct the original CMS stage, retaining its IsNeeded early return.
///
/// # Safety
/// When a stage is needed, all invoked CMS functions and opaque data must obey
/// cms_interface.h and remain valid through stage destruction. Calls must not
/// unwind or reenter the stage/transform. Buffers must be aligned, initialized
/// as promised by the CMS, and disjoint from pipeline planes; input/output CMS
/// buffers may overlap only as permitted by that interface. No foreign calls
/// occur when IsNeeded is false, so no callback lifetime is required then.
pub unsafe fn get_cms_stage(
    info: &OutputEncodingInfo,
    linear: bool,
) -> Option<Box<dyn RenderPipelineStage>> {
    let stage = Box::new(CmsStage::new(info.clone(), linear));
    if !stage.is_needed() {
        None
    } else {
        Some(stage)
    }
}
