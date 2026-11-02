// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Complete WriteToOutputStage/GetWriteToOutputStage from stage_write.{h,cc},
//! using the pinned Highway one-lane configuration. Bundle and DC writers live
//! in render_pipeline_stage.rs. Raw output/callback descriptors retain their
//! externally owned buffers; allocation callbacks back scratch storage directly.
use super::render_pipeline_stage::*;
use crate::jxl::base::byte_order::swap_endianness;
use crate::jxl::base::hwy_scalar::{clamp, max, nearest_int};
use crate::jxl::base::status::{Status, StatusCode};
use crate::jxl::dec_cache::{ImageOutput, PixelCallback};
use crate::jxl::image_metadata::Orientation;
use crate::jxl::memory_manager::JxlMemoryManager;
use crate::jxl::memory_manager_internal::CallbackAlignedMemory;
use crate::jxl::types::JxlDataType;
use std::ffi::c_void;

const CHUNK_SIZE: usize = 1024;

#[derive(Clone, Copy)]
struct Output {
    pixel_callback: PixelCallback,
    run_opaque: *mut c_void,
    buffer: *mut c_void,
    buffer_size: usize,
    stride: usize,
    num_channels: usize,
    swap_endianness: bool,
    data_type: JxlDataType,
    bits_per_sample: usize,
    // Source leaves main.channel_index_ indeterminate; it is read only for
    // extras, whose constructor loop assigns the index before insertion.
    channel_index: Option<usize>,
}
impl Output {
    fn new(image: &ImageOutput) -> Self {
        Self {
            pixel_callback: image.callback,
            run_opaque: std::ptr::null_mut(),
            buffer: image.buffer,
            buffer_size: image.buffer_size,
            stride: image.stride,
            num_channels: image.format.num_channels as usize,
            swap_endianness: swap_endianness(image.format.endianness),
            data_type: image.format.data_type,
            bits_per_sample: image.bits_per_sample,
            channel_index: None,
        }
    }
    fn prepare_for_threads(&mut self, threads: usize) -> Status {
        if self.pixel_callback.is_present() {
            // SAFETY: The unsafe stage constructor establishes callback
            // validity and lifetime. No scratch borrow exists during init.
            self.run_opaque = unsafe { self.pixel_callback.init(threads, CHUNK_SIZE) };
            if self.run_opaque.is_null() {
                return Err(StatusCode::GenericError);
            }
        } else if self.buffer.is_null() {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }
    fn destroy(&mut self) {
        if !self.run_opaque.is_null() {
            // SAFETY: This is the latest successful source init result. The
            // callback contract keeps its matching destroy live through Drop.
            unsafe {
                self.pixel_callback
                    .destroy
                    .expect("source destroy callback")(self.run_opaque);
            }
        }
    }
}

// Short owner-relative row resolution produces a raw read span. No Rust slice
// of scratch/output storage remains live while a user callback is invoked.
#[derive(Clone, Copy)]
struct InputSpan {
    address: *const f32,
    initialized: usize,
}
impl InputSpan {
    fn value(self, x: usize) -> f32 {
        assert!(x < self.initialized);
        // SAFETY: Every InputSpan comes from a live initialized input row,
        // opaque-alpha vector, or this call's reached initialized scratch span.
        unsafe { self.address.add(x).read() }
    }
    fn word(self, x: usize) -> u32 {
        assert!(x < self.initialized);
        // SAFETY: The same initialized f32 extent/alignment is valid for u32;
        // reading its bits avoids arithmetic on source signaling NaNs.
        unsafe { self.address.cast::<u32>().add(x).read() }
    }
}

pub struct WriteToOutputStage {
    width: usize,
    height: usize,
    main: Output,
    num_color: usize,
    want_alpha: bool,
    has_alpha: bool,
    unpremul_alpha: bool,
    alpha_c: usize,
    flip_x: bool,
    flip_y: bool,
    transpose: bool,
    extra_channels: Vec<Output>,
    opaque_alpha: Vec<f32>,
    memory_manager: *const JxlMemoryManager,
    temp_in: Vec<Option<CallbackAlignedMemory>>,
    temp_out: Vec<Option<CallbackAlignedMemory>>,
}
impl WriteToOutputStage {
    /// Construct the original output stage with externally owned descriptors.
    ///
    /// # Safety
    /// The initialized manager and each reached callback/opaque value must
    /// remain live through stage destruction. Successful allocations must own
    /// their requested writable extent and be accepted by the later source
    /// free callback. Every reached output-buffer span must belong to its
    /// actual caller-owned live allocation with exclusive access during writes.
    /// Callbacks must obey the source pixel-pointer lifetime, must not unwind,
    /// and must not reenter this stage or access its currently borrowed inputs.
    /// Source shape/format preconditions apply at each reached conversion:
    /// initialized selected input rows, valid channel/bit-depth counts and
    /// allocated thread slots. Invalid programmer ranges panic instead of
    /// performing a source-invalid memory access.
    #[expect(
        clippy::too_many_arguments,
        reason = "Original output-stage factory boundary."
    )]
    pub unsafe fn new(
        main_output: &ImageOutput,
        width: usize,
        height: usize,
        has_alpha: bool,
        unpremul_alpha: bool,
        alpha_c: usize,
        orientation: Orientation,
        extra_output: &[ImageOutput],
        memory_manager: *const JxlMemoryManager,
    ) -> Self {
        let main = Output::new(main_output);
        let mut stage = Self {
            width,
            height,
            num_color: if main.num_channels < 3 { 1 } else { 3 },
            want_alpha: main.num_channels == 2 || main.num_channels == 4,
            main,
            has_alpha,
            unpremul_alpha,
            alpha_c,
            flip_x: matches!(
                orientation,
                Orientation::FLIP_HORIZONTAL
                    | Orientation::ROTATE_180
                    | Orientation::ROTATE_270
                    | Orientation::ANTI_TRANSPOSE
            ),
            flip_y: matches!(
                orientation,
                Orientation::FLIP_VERTICAL
                    | Orientation::ROTATE_180
                    | Orientation::ROTATE_90
                    | Orientation::ANTI_TRANSPOSE
            ),
            transpose: matches!(
                orientation,
                Orientation::TRANSPOSE
                    | Orientation::ROTATE_90
                    | Orientation::ROTATE_270
                    | Orientation::ANTI_TRANSPOSE
            ),
            extra_channels: Vec::new(),
            opaque_alpha: vec![1.0; CHUNK_SIZE],
            memory_manager,
            temp_in: Vec::new(),
            temp_out: Vec::new(),
        };
        for (ec, output) in extra_output.iter().enumerate() {
            if output.callback.is_present() || !output.buffer.is_null() {
                let mut extra = Output::new(output);
                extra.channel_index = Some(3usize.wrapping_add(ec));
                stage.extra_channels.push(extra);
            }
        }
        stage
    }

    fn resize_storage(storage: &mut Vec<Option<CallbackAlignedMemory>>, size: usize) {
        // libc++ vector::__destruct_at_end destroys a shrinking suffix in
        // reverse order. Reallocation moves source owners without freeing them.
        while storage.len() > size {
            drop(storage.pop());
        }
        storage.resize_with(size, || None);
    }
    fn output_memory(&self, thread: usize) -> &CallbackAlignedMemory {
        self.temp_out[thread]
            .as_ref()
            .expect("source prepared output scratch")
    }
    fn input_memory(&self, thread: usize, c: usize) -> &CallbackAlignedMemory {
        self.temp_in[thread.wrapping_mul(self.main.num_channels).wrapping_add(c)]
            .as_ref()
            .expect("source prepared input scratch")
    }
    fn unpremul(&mut self, thread: usize, len: usize, lines: &mut [Option<InputSpan>; 4]) {
        for (c, line) in lines.iter().enumerate().take(self.main.num_channels) {
            let source = line.expect("source initialized line pointer");
            assert!(len <= source.initialized);
            let target = self.input_memory(thread, c);
            assert!(len * 4 <= target.size());
            // SAFETY: Live source input/scratch spans are initialized and
            // disjoint from this private target. No callback runs in the copy.
            unsafe {
                std::ptr::copy_nonoverlapping(source.address, target.address().cast::<f32>(), len);
            }
        }
        for x in 0..len {
            let alpha_ptr = self
                .input_memory(thread, self.num_color)
                .address()
                .cast::<f32>();
            // SAFETY: The preceding copy initialized this alpha element.
            let alpha = unsafe { alpha_ptr.add(x).read() };
            let mul = 1.0 / max(crate::jxl::alpha::SMALL_ALPHA, alpha);
            for c in 0..self.num_color {
                let ptr = self.input_memory(thread, c).address().cast::<f32>();
                // SAFETY: These copied color words are initialized and the
                // stage exclusively owns their live aligned scratch block.
                unsafe {
                    ptr.add(x).write(ptr.add(x).read() * mul);
                }
            }
        }
        for (c, line) in lines.iter_mut().enumerate().take(self.main.num_channels) {
            *line = Some(InputSpan {
                address: self.input_memory(thread, c).address().cast(),
                initialized: len,
            });
        }
    }
    fn flip(
        &mut self,
        out: Output,
        thread: usize,
        len: usize,
        start: &mut usize,
        lines: &mut [Option<InputSpan>; 4],
    ) {
        for (c, line) in lines.iter().enumerate().take(out.num_channels) {
            let source = line.expect("source initialized flip pointer");
            let target = self.input_memory(thread, c);
            assert!(len <= source.initialized && len * 4 <= target.size());
            if target.address().cast_const().cast::<f32>() != source.address {
                // SAFETY: Source pointer inequality is the original identity
                // branch. These full spans are disjoint and initialized.
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        source.address,
                        target.address().cast::<f32>(),
                        len,
                    );
                }
            }
        }
        let last = len.wrapping_sub(1);
        for i in 0..len / 2 {
            for c in 0..out.num_channels {
                let target = self.input_memory(thread, c).address().cast::<u32>();
                // SAFETY: The preceding copy (or identity branch) proves both
                // initialized in-bounds words. Swapping bits preserves sNaNs.
                unsafe {
                    std::ptr::swap(target.add(i), target.add(last - i));
                }
            }
        }
        for (c, line) in lines.iter_mut().enumerate().take(out.num_channels) {
            *line = Some(InputSpan {
                address: self.input_memory(thread, c).address().cast(),
                initialized: len,
            });
        }
        *start = self.width.wrapping_sub(*start).wrapping_sub(len);
    }
    fn make_unsigned(value: f32, bits: usize, byte: bool, x: usize, y: usize, c: usize) -> u16 {
        assert!(bits < 32, "source 1u shift precondition");
        let mul = ((1u32 << bits) - 1) as f32;
        let mut v = value * mul;
        if byte {
            let x_off = x.wrapping_add(c.wrapping_mul(23)) % 32;
            let y_off = y.wrapping_add(c.wrapping_mul(13)) % 32;
            v += super::stage_write_dither::DITHER[y_off * 48 + x_off];
        }
        v = clamp(0.0, v, mul);
        let nu = nearest_int(v) as u32;
        // Scalar DemoteTo saturates the unsigned value, rather than narrowing.
        nu.min(if byte {
            u8::MAX as u32
        } else {
            u16::MAX as u32
        }) as u16
    }

    fn output_buffers(
        &mut self,
        out: Output,
        thread: usize,
        y: usize,
        mut start: usize,
        len: usize,
        lines: &mut [Option<InputSpan>; 4],
    ) {
        if self.flip_x {
            self.flip(out, thread, len, &mut start, lines);
        }
        let sample_size = match out.data_type {
            JxlDataType::UINT8 => 1,
            JxlDataType::UINT16 | JxlDataType::FLOAT16 => 2,
            JxlDataType::FLOAT => 4,
            _ => return,
        };
        assert!(out.num_channels <= 4, "source channel-array precondition");
        let memory = self.output_memory(thread);
        let bytes = len
            .checked_mul(out.num_channels)
            .and_then(|n| n.checked_mul(sample_size))
            .unwrap();
        assert!(bytes <= memory.size());
        let destination = memory.address();
        for x in 0..len {
            for (c, line) in lines.iter().enumerate().take(out.num_channels) {
                let input = line.expect("source initialized conversion pointer");
                let at = (x * out.num_channels + c) * sample_size;
                // SAFETY: This private callback-backed aligned block covers
                // `bytes`; all selected input words are initialized. Each
                // reached write initializes precisely its source sample bytes.
                unsafe {
                    match out.data_type {
                        JxlDataType::UINT8 => destination.add(at).write(Self::make_unsigned(
                            input.value(x),
                            out.bits_per_sample,
                            true,
                            start.wrapping_add(x),
                            y,
                            c,
                        )
                            as u8),
                        JxlDataType::UINT16 => {
                            destination.add(at).cast::<u16>().write(Self::make_unsigned(
                                input.value(x),
                                out.bits_per_sample,
                                false,
                                start.wrapping_add(x),
                                y,
                                c,
                            ))
                        }
                        JxlDataType::FLOAT16 => {
                            let mut half = [0u16];
                            crate::jxl::dec_external_image::float_to_f16(
                                &[input.value(x)],
                                &mut half,
                                1,
                            );
                            destination.add(at).cast::<u16>().write(half[0]);
                        }
                        JxlDataType::FLOAT => {
                            destination.add(at).cast::<u32>().write(input.word(x))
                        }
                        _ => unreachable!(),
                    }
                }
            }
        }
        if out.swap_endianness && sample_size != 1 {
            for i in 0..len * out.num_channels {
                // SAFETY: All scalar output words in this exact span were
                // initialized above; alignment is the source 128-byte bound.
                unsafe {
                    if sample_size == 2 {
                        let ptr = destination.cast::<u16>().add(i);
                        ptr.write(ptr.read().swap_bytes());
                    } else {
                        let ptr = destination.cast::<u32>().add(i);
                        ptr.write(ptr.read().swap_bytes());
                    }
                }
            }
        }
        self.write_to_output(out, thread, y, start, len, sample_size, destination);
    }

    #[expect(
        clippy::too_many_arguments,
        reason = "Original typed output write boundary."
    )]
    fn write_to_output(
        &self,
        out: Output,
        thread: usize,
        y: usize,
        start: usize,
        len: usize,
        size: usize,
        source: *const u8,
    ) {
        let pixel_stride = out.num_channels.wrapping_mul(size);
        if self.transpose {
            if !out.run_opaque.is_null() {
                for i in 0..len {
                    // SAFETY: The constructor establishes callback validity;
                    // this initialized sample span lives throughout the call.
                    unsafe {
                        out.pixel_callback.run.expect("source run callback")(
                            out.run_opaque,
                            thread,
                            y,
                            start.wrapping_add(i),
                            1,
                            source.add(i * pixel_stride).cast(),
                        );
                    }
                }
            } else {
                assert!(!out.buffer.is_null(), "source output buffer must be live");
                let offset = start
                    .wrapping_mul(out.stride)
                    .wrapping_add(y.wrapping_mul(pixel_stride));
                for i in 0..len {
                    let ix = offset.wrapping_add(i.wrapping_mul(out.stride));
                    assert!(
                        ix.checked_add(pixel_stride)
                            .is_some_and(|end| end <= out.buffer_size)
                    );
                    // SAFETY: The caller retains the actual writable buffer,
                    // the source range is initialized, and the ranges do not
                    // overlap as required by original memcpy.
                    unsafe {
                        std::ptr::copy_nonoverlapping(
                            source.add(i * pixel_stride),
                            out.buffer.cast::<u8>().add(ix),
                            pixel_stride,
                        );
                    }
                }
            }
        } else if !out.run_opaque.is_null() {
            // SAFETY: No scratch slice/reference is live across the callback;
            // its initialized byte span and opaque lifetime are established.
            unsafe {
                out.pixel_callback.run.expect("source run callback")(
                    out.run_opaque,
                    thread,
                    start,
                    y,
                    len,
                    source.cast(),
                );
            }
        } else {
            assert!(!out.buffer.is_null(), "source output buffer must be live");
            let offset = y
                .wrapping_mul(out.stride)
                .wrapping_add(start.wrapping_mul(pixel_stride));
            let bytes = len.wrapping_mul(pixel_stride);
            assert!(
                offset
                    .checked_add(bytes)
                    .is_some_and(|end| end <= out.buffer_size)
            );
            // SAFETY: Same actual caller-buffer/source-initialization and
            // disjoint memcpy protocol as the transposed branch above.
            unsafe {
                std::ptr::copy_nonoverlapping(source, out.buffer.cast::<u8>().add(offset), bytes);
            }
        }
    }
}

impl RenderPipelineStage for WriteToOutputStage {
    fn settings(&self) -> Settings {
        Settings::default()
    }
    fn get_name(&self) -> &str {
        "WritePixelCB"
    }
    fn get_channel_mode(&self, c: usize) -> RenderPipelineChannelMode {
        if c < self.num_color
            || (self.has_alpha && c == self.alpha_c)
            || self
                .extra_channels
                .iter()
                .any(|out| out.channel_index == Some(c))
        {
            RenderPipelineChannelMode::Input
        } else {
            RenderPipelineChannelMode::Ignored
        }
    }
    fn prepare_for_threads(&mut self, threads: usize) -> Status {
        self.main.prepare_for_threads(threads)?;
        for extra in &mut self.extra_channels {
            extra.prepare_for_threads(threads)?;
        }
        Self::resize_storage(&mut self.temp_out, threads);
        let alloc_size = std::mem::size_of::<f32>() * CHUNK_SIZE;
        for memory in &mut self.temp_out {
            // SAFETY: The constructor's lifelong callback/manager protocol
            // holds. A failed create leaves the previous owner unchanged.
            let next = unsafe {
                CallbackAlignedMemory::create(
                    self.memory_manager,
                    alloc_size.wrapping_mul(self.main.num_channels),
                    0,
                )?
            };
            drop(memory.take());
            *memory = Some(next);
        }
        if (self.has_alpha && self.want_alpha && self.unpremul_alpha) || self.flip_x {
            Self::resize_storage(
                &mut self.temp_in,
                threads.wrapping_mul(self.main.num_channels),
            );
            for memory in &mut self.temp_in {
                // SAFETY: The same original manager owns this actual block.
                let next =
                    unsafe { CallbackAlignedMemory::create(self.memory_manager, alloc_size, 0)? };
                drop(memory.take());
                *memory = Some(next);
            }
        }
        Ok(())
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
        mut y: usize,
        thread: usize,
    ) -> Status {
        if left != 0 || right != 0 {
            return Err(StatusCode::GenericError);
        }
        if self.main.run_opaque.is_null() && self.main.buffer.is_null() {
            return Err(StatusCode::GenericError);
        }
        if y >= self.height {
            return Ok(());
        }
        if x >= self.width {
            return Ok(());
        }
        if self.flip_y {
            y = self.height - 1 - y;
        }
        let limit = width.min(self.width - x);
        for x0 in (0..limit).step_by(CHUNK_SIZE) {
            let start = x + x0;
            let len = CHUNK_SIZE.min(limit - x0);
            let mut lines: [Option<InputSpan>; 4] = [None; 4];
            for (c, line) in lines.iter_mut().enumerate().take(self.num_color) {
                let row = self.get_input_row(input, c, 0).offset(x0 as isize);
                let values = buffers.row(row);
                assert!(len <= values.len());
                *line = Some(InputSpan {
                    address: values.as_ptr(),
                    initialized: values.len(),
                });
            }
            lines[self.num_color] = Some(if self.has_alpha {
                let row = self
                    .get_input_row(input, self.alpha_c, 0)
                    .offset(x0 as isize);
                let values = buffers.row(row);
                assert!(len <= values.len());
                InputSpan {
                    address: values.as_ptr(),
                    initialized: values.len(),
                }
            } else {
                InputSpan {
                    address: self.opaque_alpha.as_ptr(),
                    initialized: self.opaque_alpha.len(),
                }
            });
            if self.has_alpha && self.want_alpha && self.unpremul_alpha {
                self.unpremul(thread, len, &mut lines);
            }
            self.output_buffers(self.main, thread, y, start, len, &mut lines);
            for ec in 0..self.extra_channels.len() {
                let extra = self.extra_channels[ec];
                let row = self
                    .get_input_row(
                        input,
                        extra.channel_index.expect("source extra channel index"),
                        0,
                    )
                    .offset(x0 as isize);
                let values = buffers.row(row);
                assert!(len <= values.len());
                lines[0] = Some(InputSpan {
                    address: values.as_ptr(),
                    initialized: values.len(),
                });
                self.output_buffers(extra, thread, y, start, len, &mut lines);
            }
        }
        Ok(())
    }
}
impl Drop for WriteToOutputStage {
    fn drop(&mut self) {
        self.main.destroy();
        for extra in &mut self.extra_channels {
            extra.destroy();
        }
        // Original destructor body precedes reverse member destruction. libc++
        // destroys each AlignedMemory vector's elements from end to beginning.
        while let Some(memory) = self.temp_out.pop() {
            drop(memory);
        }
        while let Some(memory) = self.temp_in.pop() {
            drop(memory);
        }
    }
}

/// Original public factory; see WriteToOutputStage::new for the raw protocol.
///
/// # Safety
/// All constructor callback, manager, buffer and reached-range obligations
/// apply until the returned stage has been destroyed.
#[expect(
    clippy::too_many_arguments,
    reason = "Original output-stage factory boundary."
)]
pub unsafe fn get_write_to_output_stage(
    main: &ImageOutput,
    width: usize,
    height: usize,
    has_alpha: bool,
    unpremul: bool,
    alpha_c: usize,
    orientation: Orientation,
    extra: &[ImageOutput],
    manager: *const JxlMemoryManager,
) -> Box<dyn RenderPipelineStage> {
    // SAFETY: The caller supplies exactly the constructor's source protocol.
    Box::new(unsafe {
        WriteToOutputStage::new(
            main,
            width,
            height,
            has_alpha,
            unpremul,
            alpha_c,
            orientation,
            extra,
            manager,
        )
    })
}
