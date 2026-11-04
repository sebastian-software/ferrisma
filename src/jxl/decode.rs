// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The public incremental decoder from `lib/jxl/decode.cc`, with its public
//! values from `lib/include/jxl/decode.h`.
//!
//! Language-required deviations, each local to its representation:
//! - The decoder object is allocated through the caller's memory manager, as
//!   in JxlDecoderCreate, and constructed in place. Its size is that of the
//!   Rust layout.
//! - Fields that the source references through retained raw pointers from
//!   other owners (the memory manager, codec metadata and output image
//!   metadata) live in stable Rust-owned heap cells beside the object, so that
//!   borrowing the decoder mutably does not invalidate those references. They
//!   are global-allocator storage and not memory-manager allocations.
//! - The persistent input fields live in the existing `DecoderInput` owner at
//!   the position of `codestream_copy`. Explicit destruction releases every
//!   owner in the source's reverse declaration order.
//! - A bit reader borrows its span until its scope guard ends. Where the
//!   source requests more input while its reader is still alive, the reader's
//!   result-discarding bounds/close checkpoint runs first, because the request
//!   may reallocate the borrowed copy. The guard has no observable effect.
//! - The image bundle is an `Rc<RefCell>` shared with the frame decoder's
//!   retained `decoded_` pointer, and ProcessSections borrows a copy of the
//!   unchanged TOC; see ADR 0003. The frame decoder receives the decoder's
//!   PassesDecoderState per call and always selects the low-memory pipeline.
//! - JXL_API_ERROR/JXL_INPUT_ERROR return JXL_DEC_ERROR; the release
//!   reference's debug diagnostics are not translated.
//! - Caller buffers and callback opaque values are raw pointers whose
//!   lifetime contracts are stated on the unsafe entry points. The C ABI
//!   exports are a separate boundary.
//! - The frame decoder retains a pointer to the boxed thread pool, as the
//!   source's `pool_`; the box is replaced only while no frame decoder
//!   exists. RunOnPool tasks run through the caller's runner, serialized as
//!   described in `base::data_parallel::run_on_pool_exclusive`.

use super::base::data_parallel::{JxlParallelRunner, ThreadPool};
use super::base::status::StatusCode;
use super::box_content_decoder::JxlBoxContentDecoder;
use super::cms_interface::JxlCmsInterface;
use super::codestream_header::{
    JxlBasicInfo, JxlBlendInfo, JxlBlendMode, JxlExtraChannelInfo, JxlExtraChannelType,
    JxlFrameHeader, JxlOrientation,
};
use super::color_encoding::{ColorSpace, JxlColorEncoding};
use super::color_encoding_internal::ColorEncoding;
use super::common::JXL_SIGNATURE_BOX;
use super::dec_bit_reader::BitReader;
use super::dec_cache::PassesDecoderState;
use super::dec_cache::PixelCallback;
use super::dec_cache::{ImageOutDestroyCallback, ImageOutInitCallback, ImageOutRunCallback};
use super::dec_frame::{FrameDecoder, SectionInfo, SectionStatus};
use super::decode_to_jpeg::JxlToJpegDecoder;
use super::decoder_input::{CodestreamSpan, DecoderInput};
use super::field_encodings::Fields;
use super::fields::bundle;
use super::frame_header::FrameHeader;
use super::headers::CODESTREAM_MARKER;
use super::icc_codec::ICCReader;
use super::image_bundle::ImageBundle;
use super::image_metadata::{CodecMetadata, ExtraChannel, ImageMetadata};
use super::memory_manager::{
    JxlMemoryManager, memory_manager_alloc, memory_manager_free, memory_manager_init,
};
use super::padded_bytes::PaddedBytes as GlobalPaddedBytes;
use super::types::{JxlBitDepth, JxlBitDepthType, JxlDataType, JxlPixelFormat};
use std::cell::RefCell;
use std::ffi::c_void;
use std::ptr::{self, NonNull};
use std::rc::Rc;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlSignature(pub u32);
impl JxlSignature {
    pub const NOT_ENOUGH_BYTES: Self = Self(0);
    pub const INVALID: Self = Self(1);
    pub const CODESTREAM: Self = Self(2);
    pub const CONTAINER: Self = Self(3);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlDecoderStatus(pub u32);
impl JxlDecoderStatus {
    pub const SUCCESS: Self = Self(0);
    pub const ERROR: Self = Self(1);
    pub const NEED_MORE_INPUT: Self = Self(2);
    pub const NEED_PREVIEW_OUT_BUFFER: Self = Self(3);
    pub const NEED_IMAGE_OUT_BUFFER: Self = Self(5);
    pub const JPEG_NEED_MORE_OUTPUT: Self = Self(6);
    pub const BOX_NEED_MORE_OUTPUT: Self = Self(7);
    pub const BASIC_INFO: Self = Self(0x40);
    pub const COLOR_ENCODING: Self = Self(0x100);
    pub const PREVIEW_IMAGE: Self = Self(0x200);
    pub const FRAME: Self = Self(0x400);
    pub const FULL_IMAGE: Self = Self(0x1000);
    pub const JPEG_RECONSTRUCTION: Self = Self(0x2000);
    pub const BOX: Self = Self(0x4000);
    pub const FRAME_PROGRESSION: Self = Self(0x8000);
    pub const BOX_COMPLETE: Self = Self(0x10000);
}

/// JxlProgressiveDetail from decode.h, in its original ascending order.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
#[repr(transparent)]
pub struct JxlProgressiveDetail(pub u32);
impl JxlProgressiveDetail {
    pub const FRAMES: Self = Self(0);
    pub const DC: Self = Self(1);
    pub const LAST_PASSES: Self = Self(2);
    pub const PASSES: Self = Self(3);
    pub const DC_PROGRESSIVE: Self = Self(4);
    pub const DC_GROUPS: Self = Self(5);
    pub const GROUPS: Self = Self(6);
}

pub fn out_of_bounds(a: usize, b: usize, size: usize) -> bool {
    let pos = a.wrapping_add(b);
    if pos > size {
        return true;
    }
    if pos < a {
        return true;
    }
    false
}

pub fn initial_basic_info_size_hint() -> usize {
    let container_header_size = 48;
    let max_codestream_basic_info_size = 50;
    container_header_size + max_codestream_basic_info_size
}

pub fn read_signature(buf: &[u8], pos: &mut usize) -> JxlSignature {
    if *pos >= buf.len() {
        return JxlSignature::NOT_ENOUGH_BYTES;
    }
    let buf = &buf[*pos..];
    if buf[0] == 0xff {
        if buf.len() < 2 {
            return JxlSignature::NOT_ENOUGH_BYTES;
        } else if buf[1] == CODESTREAM_MARKER {
            *pos += 2;
            return JxlSignature::CODESTREAM;
        } else {
            return JxlSignature::INVALID;
        }
    }
    if buf[0] == 0 {
        if buf.len() < JXL_SIGNATURE_BOX.len() {
            return JxlSignature::NOT_ENOUGH_BYTES;
        } else if buf[..JXL_SIGNATURE_BOX.len()] == JXL_SIGNATURE_BOX {
            *pos += JXL_SIGNATURE_BOX.len();
            return JxlSignature::CONTAINER;
        } else {
            return JxlSignature::INVALID;
        }
    }
    JxlSignature::INVALID
}

pub fn jxl_signature_check(buf: &[u8]) -> JxlSignature {
    let mut pos = 0;
    read_signature(buf, &mut pos)
}

/// Source output parameters remain unchanged until their original assignments.
/// Incomplete extended headers retain type, size=1, and header_size=16.
pub fn parse_box_header(
    input: &[u8],
    mut pos: usize,
    file_pos: usize,
    box_type: &mut [u8; 4],
    box_size: &mut usize,
    header_size: &mut usize,
) -> JxlDecoderStatus {
    if out_of_bounds(pos, 8, input.len()) {
        *header_size = 8;
        return JxlDecoderStatus::NEED_MORE_INPUT;
    }
    let box_start = pos;
    *box_size = u32::from_be_bytes(input[pos..pos + 4].try_into().unwrap()) as usize;
    pos += 4;
    box_type.copy_from_slice(&input[pos..pos + 4]);
    pos += 4;
    if *box_size == 1 {
        *header_size = 16;
        if out_of_bounds(pos, 8, input.len()) {
            return JxlDecoderStatus::NEED_MORE_INPUT;
        }
        let box_size_64 = u64::from_be_bytes(input[pos..pos + 8].try_into().unwrap());
        pos += 8;
        *box_size = box_size_64 as usize;
        if box_size_64 != *box_size as u64 {
            return JxlDecoderStatus::ERROR;
        }
    }
    *header_size = pos - box_start;
    if *box_size > 0 && *box_size < *header_size {
        return JxlDecoderStatus::ERROR;
    }
    if file_pos.wrapping_add(*box_size) < file_pos {
        return JxlDecoderStatus::ERROR;
    }
    JxlDecoderStatus::SUCCESS
}

// ---------------------------------------------------------------------------
// Public values from decode.h that the decoder object uses.

pub const JPEGXL_MAJOR_VERSION: u32 = 0;
pub const JPEGXL_MINOR_VERSION: u32 = 13;
pub const JPEGXL_PATCH_VERSION: u32 = 0;

pub fn jxl_decoder_version() -> u32 {
    JPEGXL_MAJOR_VERSION * 1_000_000 + JPEGXL_MINOR_VERSION * 1000 + JPEGXL_PATCH_VERSION
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlColorProfileTarget(pub u32);
impl JxlColorProfileTarget {
    pub const ORIGINAL: Self = Self(0);
    pub const DATA: Self = Self(1);
}

pub type JxlBoxType = [u8; 4];
pub type JxlImageOutCallback =
    unsafe extern "C" fn(*mut c_void, usize, usize, usize, *const c_void);

// JXL_API_ERROR and JXL_INPUT_ERROR: the release reference evaluates only to
// JXL_DEC_ERROR. The message documents the original diagnostic.
macro_rules! api_error {
    ($($message:tt)*) => {
        JxlDecoderStatus::ERROR
    };
}
macro_rules! input_error {
    ($($message:tt)*) => {
        JxlDecoderStatus::ERROR
    };
}
// JXL_API_RETURN_IF_ERROR with ConvertStatus for decoder statuses.
macro_rules! api_return_if_error {
    ($status:expr) => {{
        let status_: JxlDecoderStatus = $status;
        if status_ != JxlDecoderStatus::SUCCESS {
            return status_;
        }
    }};
}
// JXL_API_RETURN_IF_ERROR with ConvertStatus for jxl::Status.
macro_rules! api_return_if_status_error {
    ($status:expr) => {{
        if ($status).is_err() {
            return JxlDecoderStatus::ERROR;
        }
    }};
}

// The source's informative event masks, as the int bitfield events_wanted.
const DEC_BASIC_INFO: i32 = JxlDecoderStatus::BASIC_INFO.0 as i32;
const DEC_COLOR_ENCODING: i32 = JxlDecoderStatus::COLOR_ENCODING.0 as i32;
const DEC_PREVIEW_IMAGE: i32 = JxlDecoderStatus::PREVIEW_IMAGE.0 as i32;
const DEC_FRAME: i32 = JxlDecoderStatus::FRAME.0 as i32;
const DEC_FULL_IMAGE: i32 = JxlDecoderStatus::FULL_IMAGE.0 as i32;
const DEC_JPEG_RECONSTRUCTION: i32 = JxlDecoderStatus::JPEG_RECONSTRUCTION.0 as i32;
const DEC_BOX: i32 = JxlDecoderStatus::BOX.0 as i32;
const DEC_FRAME_PROGRESSION: i32 = JxlDecoderStatus::FRAME_PROGRESSION.0 as i32;
const DEC_BOX_COMPLETE: i32 = JxlDecoderStatus::BOX_COMPLETE.0 as i32;

fn bits_per_channel(data_type: JxlDataType) -> usize {
    match data_type {
        JxlDataType::UINT8 => 8,
        JxlDataType::UINT16 => 16,
        JxlDataType::FLOAT => 32,
        JxlDataType::FLOAT16 => 16,
        _ => 0,
    }
}

/// The source's metadata template parameter: ImageMetadata and
/// ExtraChannelInfo both provide bit_depth.bits_per_sample.
trait BitDepthSource {
    fn bits_per_sample(&self) -> u32;
}
impl BitDepthSource for ImageMetadata {
    fn bits_per_sample(&self) -> u32 {
        self.bit_depth.bits_per_sample
    }
}
impl BitDepthSource for super::image_metadata::ExtraChannelInfo {
    fn bits_per_sample(&self) -> u32 {
        self.bit_depth.bits_per_sample
    }
}

fn get_bit_depth<T: BitDepthSource>(
    bit_depth: JxlBitDepth,
    metadata: &T,
    format: JxlPixelFormat,
) -> u32 {
    if bit_depth.depth_type == JxlBitDepthType::FROM_PIXEL_FORMAT {
        return bits_per_channel(format.data_type) as u32;
    } else if bit_depth.depth_type == JxlBitDepthType::FROM_CODESTREAM {
        return metadata.bits_per_sample();
    } else if bit_depth.depth_type == JxlBitDepthType::CUSTOM {
        return bit_depth.bits_per_sample;
    }
    0
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum DecoderStage {
    Inited,
    Started,
    CodestreamFinished,
    Error,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum FrameStage {
    Header,
    Toc,
    Full,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum BoxStage {
    Header,
    Ftyp,
    Skip,
    Codestream,
    PartialCodestream,
    BufferingJxlp,
    JpegRecon,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum JpegReconStage {
    None,
    SettingMetadata,
    Outputting,
}

/// For each internal frame, which storage locations it references and which
/// it is stored in, as in FrameDecoder::References and FrameDecoder::SaveAs.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct FrameRef {
    pub reference: i32,
    pub saved_as: i32,
}

/// GetFrameDependencies: the frames required to decode the frame at `index`
/// and any later frame, in the source's discovery order.
pub fn get_frame_dependencies(index: usize, refs: &[FrameRef]) -> Vec<usize> {
    debug_assert!(index < refs.len());
    let mut result = Vec::new();
    const NUM_STORAGE: usize = 8;
    let invalid = refs.len();
    let mut storage: [Vec<usize>; NUM_STORAGE] = Default::default();
    for (s, slot) in storage.iter_mut().enumerate() {
        slot.resize(refs.len(), 0);
        let mask = 1i32 << s;
        let mut id = invalid;
        for i in 0..refs.len() {
            if refs[i].saved_as & mask != 0 {
                id = i;
            }
            slot[i] = id;
        }
    }
    let mut seen = vec![0u8; index + 1];
    let mut stack = vec![index];
    seen[index] = 1;
    for slot in &storage {
        let frame_ref = slot[index];
        if frame_ref == invalid {
            continue;
        }
        if seen[frame_ref] != 0 {
            continue;
        }
        stack.push(frame_ref);
        seen[frame_ref] = 1;
        result.push(frame_ref);
    }
    while let Some(frame_index) = stack.pop() {
        if frame_index == 0 {
            continue;
        }
        for (s, slot) in storage.iter().enumerate() {
            let mask = 1i32 << s;
            if refs[frame_index].reference & mask == 0 {
                continue;
            }
            let frame_ref = slot[frame_index - 1];
            if frame_ref == invalid {
                continue;
            }
            if seen[frame_ref] != 0 {
                continue;
            }
            stack.push(frame_ref);
            seen[frame_ref] = 1;
            result.push(frame_ref);
        }
    }
    result
}

/// Parameters for user-requested extra channel output.
#[derive(Clone, Copy)]
struct ExtraChannelOutput {
    format: JxlPixelFormat,
    buffer: *mut c_void,
    buffer_size: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct JxlDecoderFrameIndexBoxEntry {
    pub offi: u64,
    pub ti: u32,
    pub fi: u32,
}

/// The frame index box storage the source declares in the decoder object.
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct JxlDecoderFrameIndexBox {
    pub tnum: i32,
    pub tden: i32,
    pub entries: Vec<JxlDecoderFrameIndexBoxEntry>,
}
impl Default for JxlDecoderFrameIndexBox {
    fn default() -> Self {
        Self {
            tnum: 1,
            tden: 1000,
            entries: Vec::new(),
        }
    }
}
impl JxlDecoderFrameIndexBox {
    pub fn nf(&self) -> i64 {
        self.entries.len() as i64
    }
    pub fn add_frame(&mut self, offi: u64, ti: u32, fi: u32) {
        self.entries
            .push(JxlDecoderFrameIndexBoxEntry { offi, ti, fi });
    }
}

/// A stable heap cell for a decoder field that other owners reference through
/// a retained pointer. Borrowing the enclosing decoder does not retag it.
struct Aliased<T> {
    value: NonNull<T>,
}
impl<T> Aliased<T> {
    fn new(value: T) -> Self {
        Self {
            // SAFETY: Box::into_raw yields a unique, non-null, initialized
            // allocation that this cell alone releases.
            value: unsafe { NonNull::new_unchecked(Box::into_raw(Box::new(value))) },
        }
    }
    fn as_ptr(&self) -> *const T {
        self.value.as_ptr()
    }
    fn get(&self) -> &T {
        // SAFETY: The cell owns an initialized value; shared access only.
        unsafe { self.value.as_ref() }
    }
    /// # Safety
    /// No reference previously derived through `retained` may be used after
    /// this exclusive access begins; such references may only be dropped.
    unsafe fn get_mut(&mut self) -> &mut T {
        // SAFETY: The caller excludes later use of retained shared loans.
        unsafe { self.value.as_mut() }
    }
    /// A reference with the decoder's retained-pointer lifetime.
    ///
    /// # Safety
    /// The holder is dropped before this cell, and is not used after a later
    /// `get_mut` on this cell.
    unsafe fn retained(&self) -> &'static T {
        // SAFETY: The caller bounds the reference by the cell's lifetime.
        unsafe { &*self.value.as_ptr() }
    }
}
impl<T> Drop for Aliased<T> {
    fn drop(&mut self) {
        // SAFETY: Unique ownership created in `new`, released exactly once.
        drop(unsafe { Box::from_raw(self.value.as_ptr()) });
    }
}

#[derive(Clone, Copy)]
#[repr(C)]
struct SimpleImageOutCallback {
    callback: Option<JxlImageOutCallback>,
    opaque: *mut c_void,
}

/// The original JxlDecoder object, with every field in declaration order.
/// Fields of the persistent input boundary (last_codestream_seen, file_pos,
/// box_contents_end, box_contents_unbounded, codestream_copy and its counters,
/// next_jxlp_index, the jxlp buffer, next_in, avail_in and input_closed) are
/// stored in `input` at the codestream_copy position.
pub struct JxlDecoder {
    memory_manager: Aliased<JxlMemoryManager>,
    thread_pool: Option<Box<ThreadPool>>,
    stage: DecoderStage,
    got_signature: bool,
    got_codestream_signature: bool,
    got_basic_info: bool,
    got_transform_data: bool,
    got_all_headers: bool,
    post_headers: bool,
    icc_reader: Option<Box<ICCReader>>,
    frame_index_box: JxlDecoderFrameIndexBox,
    got_preview_image: bool,
    preview_frame: bool,
    box_contents_begin: usize,
    box_contents_size: usize,
    box_size: usize,
    header_size: usize,
    box_type: JxlBoxType,
    box_decoded_type: JxlBoxType,
    box_event: bool,
    decompress_boxes: bool,
    box_out_buffer_set: bool,
    box_out_buffer_set_current_box: bool,
    box_out_buffer: *mut u8,
    box_out_buffer_size: usize,
    box_out_buffer_begin: usize,
    box_out_buffer_pos: usize,
    keep_orientation: bool,
    unpremul_alpha: bool,
    render_spotcolors: bool,
    coalescing: bool,
    desired_intensity_target: f32,
    events_wanted: i32,
    orig_events_wanted: i32,
    basic_info_size_hint: usize,
    have_container: bool,
    box_count: usize,
    prog_detail: JxlProgressiveDetail,
    frame_prog_detail: JxlProgressiveDetail,
    downsampling_target: usize,
    image_out_buffer_set: bool,
    image_out_buffer: *mut c_void,
    image_out_init_callback: Option<ImageOutInitCallback>,
    image_out_run_callback: Option<ImageOutRunCallback>,
    image_out_destroy_callback: Option<ImageOutDestroyCallback>,
    image_out_init_opaque: *mut c_void,
    simple_image_out_callback: Aliased<SimpleImageOutCallback>,
    image_out_size: usize,
    image_out_format: JxlPixelFormat,
    image_out_bit_depth: JxlBitDepth,
    extra_channel_output: Vec<ExtraChannelOutput>,
    metadata: Aliased<CodecMetadata>,
    image_metadata: Aliased<ImageMetadata>,
    ib: Option<Rc<RefCell<ImageBundle<'static>>>>,
    passes_state: Option<Box<PassesDecoderState<'static, 'static>>>,
    frame_dec: Option<Box<FrameDecoder<'static, 'static>>>,
    next_section: usize,
    section_processed: Vec<u8>,
    frame_header: Option<Box<FrameHeader<'static>>>,
    remaining_frame_size: usize,
    frame_stage: FrameStage,
    dc_frame_progression_done: bool,
    is_last_of_still: bool,
    is_last_total: bool,
    skip_frames: usize,
    skipping_frame: bool,
    internal_frames: usize,
    external_frames: usize,
    frame_refs: Vec<FrameRef>,
    frame_external_to_internal: Vec<usize>,
    frame_required: Vec<u8>,
    input: DecoderInput<'static>,
    box_stage: BoxStage,
    jxl_file_format_version: u32,
    buffering_jxlp_index: u32,
    buffering_jxlp_is_last: bool,
    box_content_decoder: JxlBoxContentDecoder,
    jpeg_decoder: JxlToJpegDecoder,
    metadata_decoder: JxlBoxContentDecoder,
    exif_metadata: Vec<u8>,
    xmp_metadata: Vec<u8>,
    store_exif: i32,
    store_xmp: i32,
    recon_out_buffer_pos: usize,
    recon_exif_size: usize,
    recon_xmp_size: usize,
    recon_output_jpeg: JpegReconStage,
}

impl JxlDecoder {
    const NUM_BUFFERS_LIMIT: usize = 1 << 20;

    /// The default constructor followed by Initialize. Source-indeterminate
    /// scalars receive safe values that carry no meaning until the original
    /// Reset/Rewind or later assignment. The retained manager pointer refers
    /// to the stable manager cell, so the value may move before its use.
    fn construct(memory_manager_copy: JxlMemoryManager) -> Self {
        let memory_manager = Aliased::new(memory_manager_copy);
        let manager = memory_manager.as_ptr();
        Self {
            memory_manager,
            thread_pool: None,
            stage: DecoderStage::Inited,
            got_signature: false,
            got_codestream_signature: false,
            got_basic_info: false,
            got_transform_data: false,
            got_all_headers: false,
            post_headers: false,
            icc_reader: None,
            frame_index_box: JxlDecoderFrameIndexBox::default(),
            got_preview_image: false,
            preview_frame: false,
            box_contents_begin: 0,
            box_contents_size: 0,
            box_size: 0,
            header_size: 0,
            box_type: [0; 4],
            box_decoded_type: [0; 4],
            box_event: false,
            decompress_boxes: false,
            box_out_buffer_set: false,
            box_out_buffer_set_current_box: false,
            box_out_buffer: ptr::null_mut(),
            box_out_buffer_size: 0,
            box_out_buffer_begin: 0,
            box_out_buffer_pos: 0,
            keep_orientation: false,
            unpremul_alpha: false,
            render_spotcolors: false,
            coalescing: false,
            desired_intensity_target: 0.0,
            events_wanted: 0,
            orig_events_wanted: 0,
            basic_info_size_hint: 0,
            have_container: false,
            box_count: 0,
            prog_detail: JxlProgressiveDetail::DC,
            frame_prog_detail: JxlProgressiveDetail::FRAMES,
            downsampling_target: 0,
            image_out_buffer_set: false,
            image_out_buffer: ptr::null_mut(),
            image_out_init_callback: None,
            image_out_run_callback: None,
            image_out_destroy_callback: None,
            image_out_init_opaque: ptr::null_mut(),
            simple_image_out_callback: Aliased::new(SimpleImageOutCallback {
                callback: None,
                opaque: ptr::null_mut(),
            }),
            image_out_size: 0,
            image_out_format: JxlPixelFormat {
                num_channels: 0,
                data_type: JxlDataType::FLOAT,
                endianness: super::types::JxlEndianness::NATIVE,
                align: 0,
            },
            image_out_bit_depth: JxlBitDepth::default(),
            extra_channel_output: Vec::new(),
            metadata: Aliased::new(CodecMetadata::default()),
            image_metadata: Aliased::new(ImageMetadata::default()),
            ib: None,
            passes_state: None,
            frame_dec: None,
            next_section: 0,
            section_processed: Vec::new(),
            frame_header: None,
            remaining_frame_size: 0,
            frame_stage: FrameStage::Header,
            dc_frame_progression_done: false,
            is_last_of_still: false,
            is_last_total: false,
            skip_frames: 0,
            skipping_frame: false,
            internal_frames: 0,
            external_frames: 0,
            frame_refs: Vec::new(),
            frame_external_to_internal: Vec::new(),
            frame_required: Vec::new(),
            // SAFETY: The manager cell is stable and outlives the input owner,
            // which is destroyed before it. Callbacks cannot reenter the owner.
            input: unsafe { DecoderInput::new(manager) },
            box_stage: BoxStage::Header,
            jxl_file_format_version: 0,
            buffering_jxlp_index: 0,
            buffering_jxlp_is_last: false,
            box_content_decoder: JxlBoxContentDecoder::new(),
            jpeg_decoder: JxlToJpegDecoder::new(),
            metadata_decoder: JxlBoxContentDecoder::new(),
            exif_metadata: Vec::new(),
            xmp_metadata: Vec::new(),
            store_exif: 0,
            store_xmp: 0,
            recon_out_buffer_pos: 0,
            recon_exif_size: 0,
            recon_xmp_size: 0,
            recon_output_jpeg: JpegReconStage::None,
        }
    }

    fn manager_ptr(&self) -> *const JxlMemoryManager {
        self.memory_manager.as_ptr()
    }

    fn metadata(&self) -> &CodecMetadata {
        self.metadata.get()
    }

    /// # Safety
    /// See Aliased::get_mut: retained metadata loans are not used afterwards.
    unsafe fn metadata_mut(&mut self) -> &mut CodecMetadata {
        // SAFETY: Forward the caller's retained-loan exclusion.
        unsafe { self.metadata.get_mut() }
    }

    fn next_in(&self) -> *const u8 {
        // SAFETY: Decoder construction, Rewind and SetInput initialize it.
        unsafe { self.input.input.next_in.assume_init() }
    }

    fn avail_in(&self) -> usize {
        // SAFETY: Decoder construction, Rewind and SetInput initialize it.
        unsafe { self.input.input.avail_in.assume_init() }
    }

    fn input_closed(&self) -> bool {
        // SAFETY: Decoder construction, Rewind and CloseInput initialize it.
        unsafe { self.input.input.input_closed.assume_init() }
    }

    /// The current caller input as bytes.
    ///
    /// # Safety
    /// The SetInput contract holds; no reference survives a mutation of the
    /// input fields or a caller/allocator callback.
    unsafe fn input_bytes<'a>(&self) -> &'a [u8] {
        let (data, size) = (self.next_in(), self.avail_in());
        if size == 0 || data.is_null() {
            return &[];
        }
        // SAFETY: SetInput retains a live initialized range of avail_in bytes;
        // AdvanceInput keeps next_in/avail_in within that same range.
        unsafe { std::slice::from_raw_parts(data, size) }
    }

    fn advance_input(&mut self, size: usize) {
        // SAFETY: Every caller advances within the current SetInput range.
        unsafe { self.input.advance_input(size) }
    }

    fn advance_codestream(&mut self, size: usize) {
        // SAFETY: The original input/history and pointer-advance domain hold;
        // all codestream span loans ended before this call.
        unsafe { self.input.advance_codestream(size) }
    }

    fn request_more_input(&mut self) -> JxlDecoderStatus {
        // SAFETY: The SetInput contract covers the copied extent; no span or
        // input reference is live across the allocation callbacks.
        unsafe { self.input.request_more_input() }
    }

    fn get_codestream_input(&mut self, span: &mut CodestreamSpan) -> JxlDecoderStatus {
        // SAFETY: As for request_more_input. The returned span is only used
        // until the next input or copy mutation.
        unsafe { self.input.get_codestream_input(span) }
    }

    fn inject_next_buffered_jxlp_box(&mut self, success: &mut bool) -> super::base::status::Status {
        // SAFETY: Map payloads are owned by the input owner and disjoint from
        // codestream_copy; no reference into them is live.
        unsafe { self.input.inject_next_buffered_jxlp_box(success) }
    }

    fn can_use_more_codestream_input(&self) -> bool {
        self.stage != DecoderStage::CodestreamFinished
    }

    fn jbrd_need_more_boxes(&self) -> bool {
        if self.store_exif < 2 && self.recon_exif_size > 0 {
            return true;
        }
        if self.store_xmp < 2 && self.recon_xmp_size > 0 {
            return true;
        }
        false
    }
}

impl Drop for JxlDecoder {
    /// The implicit C++ destructor: members in reverse declaration order.
    fn drop(&mut self) {
        drop(std::mem::take(&mut self.xmp_metadata));
        drop(std::mem::take(&mut self.exif_metadata));
        drop(std::mem::take(&mut self.metadata_decoder));
        drop(std::mem::take(&mut self.jpeg_decoder));
        drop(std::mem::take(&mut self.box_content_decoder));
        // The input owner releases the jxlp map before the codestream copy.
        // SAFETY: The empty replacement never allocates, so it never calls
        // through its manager pointer, including when it is dropped after the
        // manager cell below.
        drop(std::mem::replace(&mut self.input, unsafe {
            DecoderInput::new(self.memory_manager.as_ptr())
        }));
        drop(std::mem::take(&mut self.frame_required));
        drop(std::mem::take(&mut self.frame_external_to_internal));
        drop(std::mem::take(&mut self.frame_refs));
        drop(self.frame_header.take());
        drop(std::mem::take(&mut self.section_processed));
        drop(self.frame_dec.take());
        drop(self.passes_state.take());
        drop(self.ib.take());
        // The metadata cells, the simple-callback cell and the manager cell
        // then drop with the remaining fields in declaration order; none owns
        // a memory-manager allocation, so that order is unobservable.
        drop(std::mem::take(&mut self.extra_channel_output));
        drop(std::mem::take(&mut self.frame_index_box.entries));
        drop(self.icc_reader.take());
        drop(self.thread_pool.take());
    }
}

fn check_size_limit(xsize: usize, ysize: usize) -> bool {
    if xsize == 0 || ysize == 0 {
        return true;
    }
    // DivCeil(xsize, 32) * 32 in unsigned size_t arithmetic.
    let padded_xsize = (xsize.wrapping_add(32 - 1) / 32).wrapping_mul(32);
    if padded_xsize < xsize {
        return false;
    }
    let num_pixels = padded_xsize.wrapping_mul(ysize);
    if num_pixels / padded_xsize != ysize {
        return false;
    }
    true
}

/// Resets the state that must be reset for both Rewind and Reset.
pub fn jxl_decoder_rewind_decoding_state(dec: &mut JxlDecoder) {
    dec.stage = DecoderStage::Inited;
    dec.got_signature = false;
    dec.input.last_codestream_seen = false;
    dec.got_codestream_signature = false;
    dec.got_basic_info = false;
    dec.got_transform_data = false;
    dec.got_all_headers = false;
    dec.post_headers = false;
    if let Some(icc_reader) = dec.icc_reader.as_mut() {
        icc_reader.reset();
    }
    dec.got_preview_image = false;
    dec.preview_frame = false;
    dec.input.file_pos = 0;
    dec.box_contents_begin = 0;
    dec.input.box_contents_end = 0;
    dec.box_contents_size = 0;
    dec.box_size = 0;
    dec.header_size = 0;
    dec.input.box_contents_unbounded = false;
    dec.box_type = [0; 4];
    dec.box_decoded_type = [0; 4];
    dec.box_event = false;
    dec.box_stage = BoxStage::Header;
    dec.jxl_file_format_version = 0;
    dec.input.next_jxlp_index = 0;
    dec.input.jxlp_ooo_buffer.clear();
    dec.input.jxlp_ooo_buffer_total = 0;
    dec.buffering_jxlp_index = 0;
    dec.buffering_jxlp_is_last = false;
    dec.box_out_buffer_set = false;
    dec.box_out_buffer_set_current_box = false;
    dec.box_out_buffer = ptr::null_mut();
    dec.box_out_buffer_size = 0;
    dec.box_out_buffer_begin = 0;
    dec.box_out_buffer_pos = 0;

    dec.exif_metadata.clear();
    dec.xmp_metadata.clear();
    dec.store_exif = 0;
    dec.store_xmp = 0;
    dec.recon_out_buffer_pos = 0;
    dec.recon_exif_size = 0;
    dec.recon_xmp_size = 0;
    dec.recon_output_jpeg = JpegReconStage::None;
    dec.jpeg_decoder.reset();

    dec.events_wanted = dec.orig_events_wanted;
    dec.basic_info_size_hint = initial_basic_info_size_hint();
    dec.have_container = false;
    dec.box_count = 0;
    dec.downsampling_target = 8;
    dec.image_out_buffer_set = false;
    dec.image_out_buffer = ptr::null_mut();
    dec.image_out_init_callback = None;
    dec.image_out_run_callback = None;
    dec.image_out_destroy_callback = None;
    dec.image_out_init_opaque = ptr::null_mut();
    dec.image_out_size = 0;
    dec.image_out_bit_depth.depth_type = JxlBitDepthType::FROM_PIXEL_FORMAT;
    dec.extra_channel_output.clear();
    dec.input.input.next_in.write(ptr::null());
    dec.input.input.avail_in.write(0);
    dec.input.input.input_closed.write(false);

    dec.passes_state = None;
    dec.frame_dec = None;
    dec.next_section = 0;
    dec.section_processed.clear();

    dec.ib = None;
    // SAFETY: The image bundle, frame state and codec state that retain the
    // metadata are released above. The remaining frame header is replaced
    // below before any later read of its metadata pointer.
    unsafe {
        *dec.metadata.get_mut() = CodecMetadata::default();
        *dec.image_metadata.get_mut() = dec.metadata.get().m.clone();
    }
    // SAFETY: The header is released before the metadata cell (Drop order).
    let metadata = unsafe { dec.metadata.retained() };
    dec.frame_header = Some(Box::new(FrameHeader::new(Some(metadata))));

    dec.input.codestream_copy.clear();
    dec.input.codestream_unconsumed = 0;
    dec.input.codestream_pos = 0;
    dec.input.codestream_bits_ahead = 0;

    dec.frame_stage = FrameStage::Header;
    dec.remaining_frame_size = 0;
    dec.is_last_of_still = false;
    dec.is_last_total = false;
    dec.skip_frames = 0;
    dec.skipping_frame = false;
    dec.internal_frames = 0;
    dec.external_frames = 0;
}

pub fn jxl_decoder_reset(dec: &mut JxlDecoder) {
    jxl_decoder_rewind_decoding_state(dec);

    dec.thread_pool = None;
    dec.keep_orientation = false;
    dec.unpremul_alpha = false;
    dec.render_spotcolors = true;
    dec.coalescing = true;
    dec.desired_intensity_target = 0.0;
    dec.orig_events_wanted = 0;
    dec.events_wanted = 0;
    dec.frame_refs.clear();
    dec.frame_external_to_internal.clear();
    dec.frame_required.clear();
    dec.decompress_boxes = false;
}

/// JxlDecoderCreate.
///
/// # Safety
/// A non-null `memory_manager` is readable and initialized. Its callbacks
/// obey memory_manager.h for the decoder's whole lifetime, never unwind and
/// never reenter the decoder. An allocation returned for the decoder object is
/// aligned for `JxlDecoder`, as the source's placement new requires.
pub unsafe fn jxl_decoder_create(memory_manager: *const JxlMemoryManager) -> *mut JxlDecoder {
    let mut local_memory_manager = JxlMemoryManager::default();
    // SAFETY: The caller supplies the readable manager; the local is writable.
    if unsafe { memory_manager_init(&mut local_memory_manager, memory_manager) }.is_err() {
        return ptr::null_mut();
    }
    // SAFETY: memory_manager_init produced valid callbacks for this request.
    let alloc =
        unsafe { memory_manager_alloc(&local_memory_manager, std::mem::size_of::<JxlDecoder>()) };
    if alloc.is_null() {
        return ptr::null_mut();
    }
    let dec = alloc.cast::<JxlDecoder>();
    assert!(
        dec.is_aligned(),
        "the source's placement new requires an aligned decoder allocation"
    );
    // SAFETY: The callback returned a writable allocation of the requested
    // size, checked aligned above. Construct in place, then reset, as the
    // source's placement new, Initialize and JxlDecoderReset.
    unsafe {
        dec.write(JxlDecoder::construct(local_memory_manager));
        jxl_decoder_reset(&mut *dec);
    }
    dec
}

/// JxlDecoderDestroy.
///
/// # Safety
/// `dec` is null or a decoder from jxl_decoder_create that is not used again.
pub unsafe fn jxl_decoder_destroy(dec: *mut JxlDecoder) {
    if !dec.is_null() {
        // SAFETY: The caller transfers the live decoder; copy its manager
        // before destruction, then free the object storage with it.
        unsafe {
            let local_memory_manager = *(*dec).memory_manager.get();
            ptr::drop_in_place(dec);
            memory_manager_free(&local_memory_manager, dec.cast());
        }
    }
}

pub fn jxl_decoder_rewind(dec: &mut JxlDecoder) {
    jxl_decoder_rewind_decoding_state(dec);
}

pub fn jxl_decoder_skip_frames(dec: &mut JxlDecoder, amount: usize) {
    dec.skip_frames = dec.skip_frames.wrapping_add(amount);

    dec.frame_required.clear();
    let next_frame = dec.external_frames.wrapping_add(dec.skip_frames);

    if next_frame < dec.frame_external_to_internal.len() {
        let internal_index = dec.frame_external_to_internal[next_frame];
        if internal_index < dec.frame_refs.len() {
            let deps = get_frame_dependencies(internal_index, &dec.frame_refs);

            dec.frame_required.resize(internal_index + 1, 0);
            for idx in deps {
                if idx < dec.frame_required.len() {
                    dec.frame_required[idx] = 1;
                } else {
                    // JXL_DEBUG_ABORT("Unreachable"): release builds continue.
                    debug_assert!(false, "Unreachable");
                }
            }
        }
    }
}

/// JxlDecoderSkipCurrentFrame.
///
/// # Safety
/// The current input satisfies the jxl_decoder_set_input contract.
pub unsafe fn jxl_decoder_skip_current_frame(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    if dec.frame_stage != FrameStage::Full {
        return api_error!("JxlDecoderSkipCurrentFrame called at the wrong time");
    }
    debug_assert!(dec.frame_dec.is_some());
    dec.frame_stage = FrameStage::Header;
    dec.advance_codestream(dec.remaining_frame_size);
    if dec.is_last_of_still {
        // The output buffers only apply to the frame that was skipped.
        dec.image_out_buffer_set = false;
        dec.extra_channel_output.clear();
    }
    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderSetParallelRunner.
///
/// # Safety
/// A present runner and its opaque value obey parallel_runner.h until the
/// decoder is reset, destroyed or given another runner: they call the
/// supplied init/data functions only with the supplied opaque value, only
/// during that runner call, and never unwind.
pub unsafe fn jxl_decoder_set_parallel_runner(
    dec: &mut JxlDecoder,
    parallel_runner: Option<JxlParallelRunner>,
    parallel_runner_opaque: *mut c_void,
) -> JxlDecoderStatus {
    if dec.stage != DecoderStage::Inited {
        return api_error!("JxlDecoderSetParallelRunner must be called before starting");
    }
    // SAFETY: The caller's runner contract. In the Inited stage no frame
    // decoder retains the replaced pool.
    dec.thread_pool = Some(Box::new(unsafe {
        ThreadPool::new(parallel_runner, parallel_runner_opaque)
    }));
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_size_hint_basic_info(dec: &JxlDecoder) -> usize {
    if dec.got_basic_info {
        return 0;
    }
    dec.basic_info_size_hint
}

pub fn jxl_decoder_subscribe_events(dec: &mut JxlDecoder, events_wanted: i32) -> JxlDecoderStatus {
    if dec.stage != DecoderStage::Inited {
        return JxlDecoderStatus::ERROR;
    }
    if events_wanted & 63 != 0 {
        return JxlDecoderStatus::ERROR;
    }
    dec.events_wanted = events_wanted;
    dec.orig_events_wanted = events_wanted;
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_keep_orientation(
    dec: &mut JxlDecoder,
    skip_reorientation: i32,
) -> JxlDecoderStatus {
    if dec.stage != DecoderStage::Inited {
        return api_error!("Must set keep_orientation option before starting");
    }
    dec.keep_orientation = skip_reorientation != 0;
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_unpremultiply_alpha(
    dec: &mut JxlDecoder,
    unpremul_alpha: i32,
) -> JxlDecoderStatus {
    if dec.stage != DecoderStage::Inited {
        return api_error!("Must set unpremul_alpha option before starting");
    }
    dec.unpremul_alpha = unpremul_alpha != 0;
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_render_spotcolors(
    dec: &mut JxlDecoder,
    render_spotcolors: i32,
) -> JxlDecoderStatus {
    if dec.stage != DecoderStage::Inited {
        return api_error!("Must set render_spotcolors option before starting");
    }
    dec.render_spotcolors = render_spotcolors != 0;
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_coalescing(dec: &mut JxlDecoder, coalescing: i32) -> JxlDecoderStatus {
    if dec.stage != DecoderStage::Inited {
        return api_error!("Must set coalescing option before starting");
    }
    dec.coalescing = coalescing != 0;
    JxlDecoderStatus::SUCCESS
}

/// GetCurrentDimensions: the dimensions of the current image buffer.
fn get_current_dimensions(dec: &JxlDecoder, xsize: &mut usize, ysize: &mut usize) {
    let frame_header = dec
        .frame_header
        .as_deref()
        .expect("source dereferences frame_header");
    let metadata = dec.metadata();
    if frame_header.nonserialized_is_preview {
        *xsize = metadata.oriented_preview_xsize(dec.keep_orientation);
        *ysize = metadata.oriented_preview_ysize(dec.keep_orientation);
        return;
    }
    *xsize = metadata.oriented_xsize(dec.keep_orientation);
    *ysize = metadata.oriented_ysize(dec.keep_orientation);
    if !dec.coalescing {
        let frame_dim = frame_header.to_frame_dimensions();
        *xsize = frame_dim.xsize_upsampled;
        *ysize = frame_dim.ysize_upsampled;
        if !dec.keep_orientation && metadata.m.get_orientation().0 as i32 > 4 {
            std::mem::swap(xsize, ysize);
        }
    }
}

/// A codestream bit reader with GetBitReader's deleter: AllReadsWithinBounds
/// and Close, both with discarded results.
struct ScopedReader<'a> {
    reader: BitReader<'a>,
}
impl<'a> ScopedReader<'a> {
    fn new(span: &'a [u8]) -> Self {
        Self {
            reader: BitReader::new(span),
        }
    }
}
impl Drop for ScopedReader<'_> {
    fn drop(&mut self) {
        let _ = self.reader.all_reads_within_bounds();
        let _ = self.reader.close();
    }
}

/// The span bytes of a successful GetCodestreamInput.
///
/// # Safety
/// The span is used only until the next input or copy mutation.
unsafe fn span_bytes<'a>(span: CodestreamSpan) -> &'a [u8] {
    if span.size == 0 || span.data.is_null() {
        return &[];
    }
    // SAFETY: GetCodestreamInput returns caller input or the decoder's own
    // initialized copy, valid until the next mutation by the caller's bound.
    unsafe { std::slice::from_raw_parts(span.data, span.size) }
}

/// ReadBundle's outcome. `NeedMoreInput` stands for the source's
/// `return dec->RequestMoreInput()`, which the caller performs after its
/// reader scope ends (see the module notes).
enum BundleRead {
    Success,
    NeedMoreInput,
    Status(JxlDecoderStatus),
}

fn read_bundle(data: &[u8], reader: &mut BitReader<'_>, t: &mut dyn Fields) -> BundleRead {
    // Use a copy of the bit reader because CanRead advances bits.
    let mut reader2 = BitReader::new(data);
    reader2.skip_bits(reader.total_bits_consumed());
    let can_read = bundle::can_read(&mut reader2, t);
    if reader2.close().is_err() {
        return BundleRead::Status(JxlDecoderStatus::ERROR);
    }

    if !can_read {
        return BundleRead::NeedMoreInput;
    }
    if bundle::read(reader, t).is_err() {
        return BundleRead::Status(JxlDecoderStatus::ERROR);
    }
    BundleRead::Success
}

fn jxl_decoder_read_basic_info(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    if !dec.got_codestream_signature {
        // Check and skip the codestream signature
        let mut span = CodestreamSpan {
            data: ptr::null(),
            size: 0,
        };
        api_return_if_error!(dec.get_codestream_input(&mut span));
        if span.size < 2 {
            return dec.request_more_input();
        }
        // SAFETY: The successful span covers at least these two bytes.
        let bytes = unsafe { span_bytes(span) };
        if bytes[0] != 0xff || bytes[1] != CODESTREAM_MARKER {
            return input_error!("invalid signature");
        }
        dec.got_codestream_signature = true;
        dec.advance_codestream(2);
    }

    let mut span = CodestreamSpan {
        data: ptr::null(),
        size: 0,
    };
    api_return_if_error!(dec.get_codestream_input(&mut span));
    // SAFETY: The span is not used after the input mutations below; the
    // metadata cell is disjoint from the span storage.
    let data = unsafe { span_bytes(span) };
    let mut reader = ScopedReader::new(data);
    // SAFETY: No retained metadata loan is read before its replacement.
    let metadata = unsafe { dec.metadata_mut() };
    let mut outcome = read_bundle(data, &mut reader.reader, &mut metadata.size);
    if matches!(outcome, BundleRead::Success) {
        outcome = read_bundle(data, &mut reader.reader, &mut metadata.m);
    }
    match outcome {
        BundleRead::Success => {}
        BundleRead::NeedMoreInput => {
            drop(reader);
            return dec.request_more_input();
        }
        BundleRead::Status(status) => return status,
    }
    let total_bits = reader.reader.total_bits_consumed();
    dec.advance_codestream(total_bits / 8);
    dec.input.codestream_bits_ahead = total_bits % 8;
    dec.got_basic_info = true;
    dec.basic_info_size_hint = 0;
    // SAFETY: No image bundle retains the output image metadata here.
    unsafe {
        *dec.image_metadata.get_mut() = dec.metadata.get().m.clone();
    }

    if !check_size_limit(dec.metadata().size.xsize(), dec.metadata().size.ysize()) {
        return input_error!("image is too large");
    }

    JxlDecoderStatus::SUCCESS
}

/// Reads all codestream headers (but not frame headers).
fn jxl_decoder_read_all_headers(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    if !dec.got_transform_data {
        let mut span = CodestreamSpan {
            data: ptr::null(),
            size: 0,
        };
        api_return_if_error!(dec.get_codestream_input(&mut span));
        // SAFETY: As in ReadBasicInfo.
        let data = unsafe { span_bytes(span) };
        let mut reader = ScopedReader::new(data);
        reader.reader.skip_bits(dec.input.codestream_bits_ahead);
        // SAFETY: No retained metadata loan is read before its replacement.
        let metadata = unsafe { dec.metadata_mut() };
        metadata.transform_data.nonserialized_xyb_encoded = metadata.m.xyb_encoded;
        match read_bundle(data, &mut reader.reader, &mut metadata.transform_data) {
            BundleRead::Success => {}
            BundleRead::NeedMoreInput => {
                drop(reader);
                return dec.request_more_input();
            }
            BundleRead::Status(status) => return status,
        }
        let total_bits = reader.reader.total_bits_consumed();
        dec.advance_codestream(total_bits / 8);
        dec.input.codestream_bits_ahead = total_bits % 8;
        dec.got_transform_data = true;
    }

    let mut span = CodestreamSpan {
        data: ptr::null(),
        size: 0,
    };
    api_return_if_error!(dec.get_codestream_input(&mut span));
    // SAFETY: As in ReadBasicInfo.
    let data = unsafe { span_bytes(span) };
    let mut reader = ScopedReader::new(data);
    reader.reader.skip_bits(dec.input.codestream_bits_ahead);

    if dec.metadata().m.color_encoding.want_icc() {
        let icc_reader = dec
            .icc_reader
            .as_mut()
            .expect("source allocates icc_reader first");
        let status = icc_reader.init(&mut reader.reader);
        if reader.reader.all_reads_within_bounds().is_err()
            || status == Err(StatusCode::NotEnoughBytes)
        {
            drop(reader);
            return dec.request_more_input();
        }
        if status.is_err() {
            return JxlDecoderStatus::ERROR;
        }
        // The ICC reader's existing output representation uses the global
        // allocator; its memory-manager propagation is a separate boundary.
        let mut decoded_icc = GlobalPaddedBytes::default();
        let icc_reader = dec
            .icc_reader
            .as_mut()
            .expect("source allocates icc_reader first");
        let status = icc_reader.process(&mut reader.reader, &mut decoded_icc);
        if status == Err(StatusCode::NotEnoughBytes) {
            drop(reader);
            return dec.request_more_input();
        }
        if status.is_err() {
            return JxlDecoderStatus::ERROR;
        }
        if decoded_icc.is_empty() {
            return JxlDecoderStatus::ERROR;
        }
        let icc = decoded_icc.as_slice().to_vec();
        // SAFETY: No retained metadata loan is read before its replacement.
        unsafe { dec.metadata_mut() }
            .m
            .color_encoding
            .set_icc_raw(icc);
    }

    dec.got_all_headers = true;
    api_return_if_status_error!(reader.reader.jump_to_byte_boundary());

    let consumed = reader.reader.total_bits_consumed() / 8;
    dec.advance_codestream(consumed);
    dec.input.codestream_bits_ahead = 0;
    drop(reader);

    if dec.passes_state.is_none() {
        dec.passes_state = Some(Box::new(PassesDecoderState::new()));
    }

    let metadata = dec.metadata.get();
    let passes_state = dec.passes_state.as_mut().expect("allocated above");
    api_return_if_status_error!(
        passes_state
            .output_encoding_info
            .set_from_metadata(metadata)
    );
    if dec.desired_intensity_target > 0.0 {
        passes_state.output_encoding_info.desired_intensity_target = dec.desired_intensity_target;
    }
    // SAFETY: No image bundle retains the output image metadata here.
    unsafe {
        *dec.image_metadata.get_mut() = dec.metadata.get().m.clone();
    }

    JxlDecoderStatus::SUCCESS
}

fn jxl_decoder_process_sections(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    let mut span = CodestreamSpan {
        data: ptr::null(),
        size: 0,
    };
    api_return_if_error!(dec.get_codestream_input(&mut span));
    // SAFETY: The section readers end before the input mutation below.
    let data = unsafe { span_bytes(span) };
    // A copy of the frame decoder's TOC, unchanged by ProcessSections, so that
    // the frame decoder can be borrowed mutably meanwhile.
    let toc = dec
        .frame_dec
        .as_ref()
        .expect("the header stage allocates frame_dec")
        .toc()
        .to_vec();
    let mut pos = 0usize;
    let mut readers = Vec::new();
    let mut section_ids = Vec::new();
    for (i, entry) in toc.iter().enumerate().skip(dec.next_section) {
        if dec.section_processed[i] != 0 {
            pos = pos.wrapping_add(entry.size);
            continue;
        }
        let id = entry.id;
        let size = entry.size;
        if out_of_bounds(pos, size, span.size) {
            break;
        }
        readers.push(BitReader::new(&data[pos..pos + size]));
        section_ids.push((id, i));
        pos = pos.wrapping_add(size);
    }
    let mut section_info: Vec<SectionInfo<'_, '_>> = readers
        .iter_mut()
        .zip(&section_ids)
        .map(|(br, &(id, index))| SectionInfo { br, id, index })
        .collect();
    let mut section_status = vec![SectionStatus::default(); section_info.len()];
    let frame_dec = dec
        .frame_dec
        .as_mut()
        .expect("the header stage allocates frame_dec");
    let passes_state = dec
        .passes_state
        .as_mut()
        .expect("ReadAllHeaders allocates passes_state");
    let status = frame_dec.process_sections(passes_state, &mut section_info, &mut section_status);
    drop(section_info);
    let mut out_of_bounds_section = false;
    let mut has_error = false;
    for mut br in readers {
        if br.all_reads_within_bounds().is_err() {
            // Mark out of bounds section, but keep closing and deleting the
            // next ones as well.
            out_of_bounds_section = true;
        }
        if br.close().is_err() {
            has_error = true;
        }
    }
    if has_error {
        return input_error!("internal: bit-reader failed to close");
    }
    if out_of_bounds_section {
        // If any bit reader indicates out of bounds, it's an error, not just
        // needing more input, since we ensure only bit readers containing a
        // complete section are provided to the FrameDecoder.
        return input_error!("frame out of bounds");
    }
    if status.is_err() {
        return input_error!("frame processing failed");
    }
    for (i, s_status) in section_status.iter().enumerate() {
        if *s_status == SectionStatus::Done {
            dec.section_processed[section_ids[i].1] = 1;
        } else if *s_status != SectionStatus::Skipped {
            return input_error!("unexpected section status");
        }
    }
    let mut completed_prefix_bytes = 0usize;
    while dec.next_section < dec.section_processed.len()
        && dec.section_processed[dec.next_section] == 1
    {
        completed_prefix_bytes = completed_prefix_bytes.wrapping_add(toc[dec.next_section].size);
        dec.next_section += 1;
    }
    dec.remaining_frame_size = dec
        .remaining_frame_size
        .wrapping_sub(completed_prefix_bytes);
    dec.advance_codestream(completed_prefix_bytes);
    JxlDecoderStatus::SUCCESS
}

// TODO(eustas): no CodecInOut -> no image size reinforcement -> possible OOM.
fn jxl_decoder_process_codestream(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    // If no parallel runner is set, use the default
    if dec.thread_pool.is_none() {
        // SAFETY: A runner-less pool runs only the supplied local functions.
        dec.thread_pool = Some(Box::new(unsafe { ThreadPool::new(None, ptr::null_mut()) }));
    }

    // No matter what events are wanted, the basic info is always required.
    if !dec.got_basic_info {
        let status = jxl_decoder_read_basic_info(dec);
        if status != JxlDecoderStatus::SUCCESS {
            return status;
        }
    }

    if dec.events_wanted & DEC_BASIC_INFO != 0 {
        dec.events_wanted &= !DEC_BASIC_INFO;
        return JxlDecoderStatus::BASIC_INFO;
    }

    if dec.events_wanted == 0 {
        dec.stage = DecoderStage::CodestreamFinished;
        return JxlDecoderStatus::SUCCESS;
    }

    if dec.icc_reader.is_none() {
        dec.icc_reader = Some(Box::new(ICCReader::new()));
    }

    if !dec.got_all_headers {
        let status = jxl_decoder_read_all_headers(dec);
        if status != JxlDecoderStatus::SUCCESS {
            return status;
        }
    }

    if dec.events_wanted & DEC_COLOR_ENCODING != 0 {
        dec.events_wanted &= !DEC_COLOR_ENCODING;
        return JxlDecoderStatus::COLOR_ENCODING;
    }

    if dec.events_wanted == 0 {
        dec.stage = DecoderStage::CodestreamFinished;
        return JxlDecoderStatus::SUCCESS;
    }

    dec.post_headers = true;

    if !dec.got_preview_image && dec.metadata().m.have_preview {
        dec.preview_frame = true;
    }

    // Handle frames
    loop {
        let parse_frames =
            (dec.events_wanted & (DEC_PREVIEW_IMAGE | DEC_FRAME | DEC_FULL_IMAGE)) != 0;
        if !parse_frames {
            break;
        }
        if dec.frame_stage == FrameStage::Header && dec.is_last_total {
            break;
        }
        if dec.frame_stage == FrameStage::Header {
            if dec.recon_output_jpeg == JpegReconStage::SettingMetadata
                || dec.recon_output_jpeg == JpegReconStage::Outputting
            {
                // The image bundle contains the JPEG reconstruction frame, but
                // the decoder is still waiting to decode an EXIF or XMP box.
                // It's not implemented to decode additional frames during this,
                // and a JPEG reconstruction image should have only one frame.
                return api_error!("cannot decode a next frame after JPEG reconstruction frame");
            }
            if dec.ib.is_none() {
                // SAFETY: Bundles are released before the image metadata cell
                // (Drop and Rewind order), and the cell is not mutated after
                // post_headers while a bundle exists.
                let image_metadata = unsafe { dec.image_metadata.retained() };
                dec.ib = Some(Rc::new(RefCell::new(ImageBundle::new(Some(
                    image_metadata,
                )))));
            }
            // If JPEG reconstruction is wanted and possible, set the jpeg_data
            // of the ImageBundle.
            let ib = dec.ib.clone().expect("allocated above");
            if dec
                .jpeg_decoder
                .set_image_bundle_jpeg_data(&mut ib.borrow_mut())
                .is_err()
            {
                return JxlDecoderStatus::ERROR;
            }
            // SAFETY: The frame decoder and every frame header are released
            // before the metadata cell, which is not mutated after the headers.
            let metadata = unsafe { dec.metadata.retained() };
            // SAFETY: FrameDecoder::new's output and CMS contract is the public
            // decoder's: the image-out buffer, callbacks and extra-channel
            // buffers (jxl_decoder_set_*_buffer/callback), the memory manager
            // (jxl_decoder_create) and the CMS (jxl_decoder_set_cms) stay valid
            // until the pipelines built from them are replaced or the decoder
            // is destroyed. The manager cell outlives the frame decoder. The
            // boxed pool, created above, is replaced only in the Inited stage
            // or by Reset, after this frame decoder is released; destruction
            // releases the frame decoder first.
            let pool = dec
                .thread_pool
                .as_deref()
                .map_or(ptr::null(), ptr::from_ref);
            let frame_dec = unsafe { FrameDecoder::new(metadata, dec.manager_ptr(), pool, false) }
                .expect("parsed metadata has 1..=32 bits per sample and matching extra channels");
            dec.frame_dec = Some(Box::new(frame_dec));
            dec.frame_header = Some(Box::new(FrameHeader::new(Some(metadata))));
            let mut span = CodestreamSpan {
                data: ptr::null(),
                size: 0,
            };
            api_return_if_error!(dec.get_codestream_input(&mut span));
            // SAFETY: The span is not used after the input mutations below.
            let data = unsafe { span_bytes(span) };
            let mut reader = ScopedReader::new(data);
            let frame_dec = dec.frame_dec.as_mut().expect("allocated above");
            let passes_state = dec
                .passes_state
                .as_mut()
                .expect("ReadAllHeaders allocates passes_state");
            let status =
                frame_dec.init_frame(passes_state, &mut reader.reader, ib, dec.preview_frame);
            if reader.reader.all_reads_within_bounds().is_err()
                || status == Err(StatusCode::NotEnoughBytes)
            {
                drop(reader);
                return dec.request_more_input();
            } else if status.is_err() {
                return input_error!("invalid frame header");
            }
            let consumed = reader.reader.total_bits_consumed() / 8;
            drop(reader);
            dec.advance_codestream(consumed);
            let frame_dec = dec.frame_dec.as_mut().expect("allocated above");
            let frame_header = dec.frame_header.as_mut().expect("allocated above");
            **frame_header = frame_dec.get_frame_header().clone();
            let frame_dim = frame_header.to_frame_dimensions();
            if !check_size_limit(
                frame_dim.xsize_upsampled_padded,
                frame_dim.ysize_upsampled_padded,
            ) {
                return input_error!("frame is too large");
            }
            let output_type = if dec.preview_frame {
                DEC_PREVIEW_IMAGE
            } else {
                DEC_FULL_IMAGE
            };
            let output_needed = (dec.events_wanted & output_type) != 0;
            let passes_state = dec
                .passes_state
                .as_mut()
                .expect("ReadAllHeaders allocates passes_state");
            if output_needed {
                api_return_if_status_error!(frame_dec.init_frame_output(passes_state));
            }
            dec.remaining_frame_size = frame_dec.sum_section_sizes() as usize;

            dec.frame_stage = FrameStage::Toc;
            if dec.preview_frame {
                if (dec.events_wanted & DEC_PREVIEW_IMAGE) == 0 {
                    dec.frame_stage = FrameStage::Header;
                    dec.advance_codestream(dec.remaining_frame_size);
                    dec.got_preview_image = true;
                    dec.preview_frame = false;
                }
                continue;
            }

            let frame_header = dec.frame_header.as_deref().expect("allocated above");
            let saved_as = FrameDecoder::saved_as(frame_header);
            // is last in entire codestream
            dec.is_last_total = frame_header.is_last;
            // is last of current still
            dec.is_last_of_still = dec.is_last_total || frame_header.animation_frame.duration > 0;
            // is kRegularFrame and coalescing is disabled
            dec.is_last_of_still |= !dec.coalescing
                && frame_header.frame_type == super::frame_header::FrameType::REGULAR;
            let internal_frame_index = dec.internal_frames;
            let external_frame_index = dec.external_frames;
            if dec.is_last_of_still {
                dec.external_frames = dec.external_frames.wrapping_add(1);
            }
            dec.internal_frames = dec.internal_frames.wrapping_add(1);

            if dec.skip_frames > 0 {
                dec.skipping_frame = true;
                if dec.is_last_of_still {
                    dec.skip_frames -= 1;
                }
            } else {
                dec.skipping_frame = false;
            }

            if external_frame_index >= dec.frame_external_to_internal.len() {
                dec.frame_external_to_internal.push(internal_frame_index);
                if dec.frame_external_to_internal.len() != external_frame_index.wrapping_add(1) {
                    return api_error!("internal");
                }
            }

            if internal_frame_index >= dec.frame_refs.len() {
                // add the value 0xff (which means all references) to new slots:
                // we only know the references of the frame at FinalizeFrame, and
                // fill in the correct values there. As long as this information
                // is not known, the worst case where the frame depends on all
                // storage slots is assumed.
                dec.frame_refs.push(FrameRef {
                    reference: 0xFF,
                    saved_as,
                });
                if dec.frame_refs.len() != internal_frame_index.wrapping_add(1) {
                    return api_error!("internal");
                }
            }

            if dec.skipping_frame {
                // Whether this frame could be referenced by any future frame:
                // either because it's a frame saved for blending or patches, or
                // because it's a DC frame.
                let mut referenceable = frame_header.can_be_referenced()
                    || frame_header.frame_type == super::frame_header::FrameType::DC;
                if internal_frame_index < dec.frame_required.len()
                    && dec.frame_required[internal_frame_index] == 0
                {
                    referenceable = false;
                }
                if !referenceable {
                    // Skip all decoding for this frame, since the user is
                    // skipping this frame and no future frames can reference it.
                    dec.frame_stage = FrameStage::Header;
                    dec.advance_codestream(dec.remaining_frame_size);
                    continue;
                }
            }

            if (dec.events_wanted & DEC_FRAME) != 0 && dec.is_last_of_still {
                // Only return this for the last of a series of stills: patches
                // frames etc... before this one do not contain the correct
                // information such as animation timing, ...
                if !dec.skipping_frame {
                    return JxlDecoderStatus::FRAME;
                }
            }
        }

        if dec.frame_stage == FrameStage::Toc {
            let frame_dec = dec
                .frame_dec
                .as_mut()
                .expect("the header stage allocates frame_dec");
            frame_dec.set_render_spotcolors(dec.render_spotcolors);
            frame_dec.set_coalescing(dec.coalescing);

            if !dec.preview_frame && (dec.events_wanted & DEC_FRAME_PROGRESSION) != 0 {
                dec.frame_prog_detail = frame_dec.set_pause_at_progressive(dec.prog_detail);
            } else {
                dec.frame_prog_detail = JxlProgressiveDetail::FRAMES;
            }
            dec.dc_frame_progression_done = false;

            dec.next_section = 0;
            dec.section_processed.clear();
            dec.section_processed.resize(frame_dec.toc().len(), 0);

            // If we don't need pixels, we can skip actually decoding the frames.
            if dec.preview_frame || (dec.events_wanted & DEC_FULL_IMAGE) != 0 {
                dec.frame_stage = FrameStage::Full;
            } else if !dec.is_last_total {
                dec.frame_stage = FrameStage::Header;
                dec.advance_codestream(dec.remaining_frame_size);
                continue;
            } else {
                break;
            }
        }

        if dec.frame_stage == FrameStage::Full {
            if !dec.image_out_buffer_set {
                if dec.preview_frame {
                    return JxlDecoderStatus::NEED_PREVIEW_OUT_BUFFER;
                }
                let jpeg_output = dec.jpeg_decoder.is_output_set()
                    && dec
                        .ib
                        .as_ref()
                        .expect("the header stage allocates the bundle")
                        .borrow()
                        .jpeg_data
                        .is_some();
                if !jpeg_output && dec.is_last_of_still && !dec.skipping_frame {
                    // TODO(lode): remove the dec->is_last_of_still condition if
                    // the frame decoder needs the image buffer as working space
                    // for decoding non-visible or blending frames too
                    return JxlDecoderStatus::NEED_IMAGE_OUT_BUFFER;
                }
            }

            if dec.image_out_buffer_set {
                let mut xsize = 0usize;
                let mut ysize = 0usize;
                get_current_dimensions(dec, &mut xsize, &mut ysize);
                let bits_per_sample = get_bit_depth(
                    dec.image_out_bit_depth,
                    &dec.metadata().m,
                    dec.image_out_format,
                );
                let frame_dec = dec
                    .frame_dec
                    .as_mut()
                    .expect("the header stage allocates frame_dec");
                let passes_state = dec
                    .passes_state
                    .as_mut()
                    .expect("ReadAllHeaders allocates passes_state");
                api_return_if_status_error!(frame_dec.set_image_output(
                    passes_state,
                    PixelCallback::new(
                        dec.image_out_init_callback,
                        dec.image_out_run_callback,
                        dec.image_out_destroy_callback,
                        dec.image_out_init_opaque,
                    ),
                    dec.image_out_buffer,
                    dec.image_out_size,
                    xsize,
                    ysize,
                    dec.image_out_format,
                    bits_per_sample as usize,
                    dec.unpremul_alpha,
                    !dec.keep_orientation,
                ));
                for i in 0..dec.extra_channel_output.len() {
                    let extra = dec.extra_channel_output[i];
                    let ec_bits_per_sample = get_bit_depth(
                        dec.image_out_bit_depth,
                        &dec.metadata.get().m.extra_channel_info[i],
                        extra.format,
                    );
                    api_return_if_status_error!(frame_dec.add_extra_channel_output(
                        passes_state,
                        extra.buffer,
                        extra.buffer_size,
                        xsize,
                        extra.format,
                        ec_bits_per_sample as usize,
                    ));
                }
            }

            let next_num_passes_to_pause = dec
                .frame_dec
                .as_ref()
                .expect("the header stage allocates frame_dec")
                .next_num_passes_to_pause();

            api_return_if_error!(jxl_decoder_process_sections(dec));

            let frame_dec = dec
                .frame_dec
                .as_mut()
                .expect("the header stage allocates frame_dec");
            let all_sections_done = frame_dec.has_decoded_all();
            let got_dc_only = !all_sections_done && frame_dec.has_decoded_dc();

            if dec.frame_prog_detail >= JxlProgressiveDetail::DC
                && !dec.dc_frame_progression_done
                && got_dc_only
            {
                dec.dc_frame_progression_done = true;
                dec.downsampling_target = 8;
                return JxlDecoderStatus::FRAME_PROGRESSION;
            }

            let new_progression_step_done =
                frame_dec.num_complete_passes() >= next_num_passes_to_pause;

            if !all_sections_done
                && dec.frame_prog_detail >= JxlProgressiveDetail::LAST_PASSES
                && new_progression_step_done
            {
                dec.downsampling_target = dec
                    .frame_header
                    .as_ref()
                    .expect("the header stage replaces frame_header")
                    .passes
                    .get_downsampling_target_for_completed_passes(
                        frame_dec.num_complete_passes() as u32
                    ) as usize;
                return JxlDecoderStatus::FRAME_PROGRESSION;
            }

            if !all_sections_done {
                // Not all sections have been processed yet
                return dec.request_more_input();
            }

            let passes_state = dec
                .passes_state
                .as_mut()
                .expect("ReadAllHeaders allocates passes_state");
            if !dec.preview_frame {
                let internal_index = dec.internal_frames.wrapping_sub(1);
                if dec.frame_refs.len() <= internal_index {
                    return api_error!("internal");
                }
                // Always fill this in, even if it was already written, it could
                // be that this frame was skipped before and set to 255, while
                // only now we know the true value.
                dec.frame_refs[internal_index].reference = frame_dec.references(passes_state);
            }

            if frame_dec.finalize_frame(passes_state).is_err() {
                return input_error!("decoding frame failed");
            }
            // If jpeg output was requested, we merely return the
            // JXL_DEC_FULL_IMAGE status without outputting pixels.
            if dec.jpeg_decoder.is_output_set()
                && dec
                    .ib
                    .as_ref()
                    .expect("the header stage allocates the bundle")
                    .borrow()
                    .jpeg_data
                    .is_some()
            {
                dec.frame_stage = FrameStage::Header;
                dec.recon_output_jpeg = JpegReconStage::SettingMetadata;
                return JxlDecoderStatus::FULL_IMAGE;
            }
            if dec.preview_frame || dec.is_last_of_still {
                dec.image_out_buffer_set = false;
                dec.extra_channel_output.clear();
            }
        }

        dec.frame_stage = FrameStage::Header;

        // The pixels have been output or are not needed, do not keep them in
        // memory here.
        dec.ib = None;
        if dec.preview_frame {
            dec.got_preview_image = true;
            dec.preview_frame = false;
            dec.events_wanted &= !DEC_PREVIEW_IMAGE;
            return JxlDecoderStatus::PREVIEW_IMAGE;
        } else if dec.is_last_of_still
            && (dec.events_wanted & DEC_FULL_IMAGE) != 0
            && !dec.skipping_frame
        {
            return JxlDecoderStatus::FULL_IMAGE;
        }
    }

    dec.stage = DecoderStage::CodestreamFinished;
    // Return success, this means there is nothing more to do.
    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderSetInput.
///
/// # Safety
/// `data` and `size` describe a live initialized input range that stays
/// valid and unmodified by the caller until jxl_decoder_release_input, except
/// as documented for the source. A non-null zero-length input keeps its
/// identity until released.
pub unsafe fn jxl_decoder_set_input(
    dec: &mut JxlDecoder,
    data: *const u8,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller supplies the retained input contract.
    unsafe { dec.input.set_input(data, size) }
}

pub fn jxl_decoder_release_input(dec: &mut JxlDecoder) -> usize {
    dec.input.release_input()
}

pub fn jxl_decoder_close_input(dec: &mut JxlDecoder) {
    dec.input.close_input();
}

/// JxlDecoderSetJPEGBuffer.
///
/// # Safety
/// When `size` is nonzero, `data` is writable for `size` bytes until
/// jxl_decoder_release_jpeg_buffer, and is disjoint from the decoder and its
/// input. A null `data` is permitted only with a zero size.
pub unsafe fn jxl_decoder_set_jpeg_buffer(
    dec: &mut JxlDecoder,
    data: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    // JPEG reconstruction buffer can only set and updated before or during the
    // first frame, the reconstruction box refers to the first frame and in
    // theory multi-frame images should not be used with a jbrd box.
    if dec.internal_frames > 1 {
        return api_error!("JPEG reconstruction only works for the first frame");
    }
    if dec.jpeg_decoder.is_output_set() {
        return api_error!("Already set JPEG buffer");
    }
    dec.jpeg_decoder.set_output_buffer(data, size)
}

pub fn jxl_decoder_release_jpeg_buffer(dec: &mut JxlDecoder) -> usize {
    dec.jpeg_decoder.release_output_buffer()
}

/// HandleBoxes; this includes handling the codestream if it is not a
/// box-based jxl file.
///
/// # Safety
/// The process_input contract holds.
unsafe fn handle_boxes(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    // Box handling loop
    loop {
        if dec.box_stage != BoxStage::Header {
            dec.advance_input(dec.header_size);
            dec.header_size = 0;
            if (dec.events_wanted & DEC_BOX) != 0
                && dec.box_event
                && !dec.box_out_buffer_set_current_box
            {
                // The user did not set an output buffer for this box before
                // continuing decoding past the box header; treat this as opting
                // out of box output for this box and disallow late buffer setup.
                dec.box_event = false;
            }
            if (dec.events_wanted & DEC_BOX) != 0 && dec.box_out_buffer_set_current_box {
                let mut next_out = dec.box_out_buffer.wrapping_add(dec.box_out_buffer_pos);
                let mut avail_out = dec.box_out_buffer_size.wrapping_sub(dec.box_out_buffer_pos);

                let box_pos = dec.input.file_pos.wrapping_sub(dec.box_contents_begin);
                let (next_in, avail_in) = (dec.next_in(), dec.avail_in());
                // SAFETY: StartBox ran at this box's header. Input and output
                // ranges are the caller's retained buffers, disjoint from the
                // decoder; the cursor cells are locals.
                let mut box_result = unsafe {
                    dec.box_content_decoder.process_raw(
                        next_in,
                        avail_in,
                        box_pos,
                        &mut next_out,
                        &mut avail_out,
                    )
                };
                let produced = (next_out as usize)
                    .wrapping_sub(dec.box_out_buffer.wrapping_add(dec.box_out_buffer_pos) as usize);
                dec.box_out_buffer_pos = dec.box_out_buffer_pos.wrapping_add(produced);

                if box_result == JxlDecoderStatus::BOX_COMPLETE
                    && (dec.events_wanted & DEC_BOX_COMPLETE) == 0
                {
                    box_result = JxlDecoderStatus::SUCCESS;
                }

                // Don't return JXL_DEC_NEED_MORE_INPUT: the box stages below,
                // instead, handle the input progression, and the above only
                // outputs the part of the box seen so far.
                if box_result != JxlDecoderStatus::SUCCESS
                    && box_result != JxlDecoderStatus::NEED_MORE_INPUT
                {
                    return box_result;
                }
            }
            if dec.store_exif == 1 || dec.store_xmp == 1 {
                // Just a safeguard to prevent unlimited growth. NB: for JPEG
                // chunks 65533 bytes enough.
                const BLOCK_SIZE_LIMIT: usize = 64 << 20;
                loop {
                    let metadata = if dec.store_exif == 1 {
                        &mut dec.exif_metadata
                    } else {
                        &mut dec.xmp_metadata
                    };
                    if metadata.is_empty() {
                        metadata.resize(64, 0);
                    }
                    let orig_next_out =
                        metadata.as_mut_ptr().wrapping_add(dec.recon_out_buffer_pos);
                    let mut next_out = orig_next_out;
                    let mut avail_out = metadata.len().wrapping_sub(dec.recon_out_buffer_pos);
                    let box_pos = dec.input.file_pos.wrapping_sub(dec.box_contents_begin);
                    let (next_in, avail_in) = (dec.next_in(), dec.avail_in());
                    // SAFETY: StartBox ran for this metadata box; the output is
                    // the decoder's own vector, disjoint from caller input.
                    let box_result = unsafe {
                        dec.metadata_decoder.process_raw(
                            next_in,
                            avail_in,
                            box_pos,
                            &mut next_out,
                            &mut avail_out,
                        )
                    };
                    let produced = (next_out as usize).wrapping_sub(orig_next_out as usize);
                    dec.recon_out_buffer_pos = dec.recon_out_buffer_pos.wrapping_add(produced);
                    let metadata = if dec.store_exif == 1 {
                        &mut dec.exif_metadata
                    } else {
                        &mut dec.xmp_metadata
                    };
                    if box_result == JxlDecoderStatus::BOX_NEED_MORE_OUTPUT {
                        if metadata.len() >= BLOCK_SIZE_LIMIT {
                            return input_error!("EXIF/XMP box is too large");
                        }
                        let doubled = metadata.len() * 2;
                        metadata.resize(doubled, 0);
                    } else if box_result == JxlDecoderStatus::NEED_MORE_INPUT {
                        break; // box stage handling below will handle this instead
                    } else if box_result == JxlDecoderStatus::BOX_COMPLETE {
                        let needed_size = if dec.store_exif == 1 {
                            dec.recon_exif_size
                        } else {
                            dec.recon_xmp_size
                        };
                        if dec.input.box_contents_unbounded
                            && dec.recon_out_buffer_pos < needed_size
                        {
                            // Unbounded box, but we know the expected size due to
                            // the jbrd box's data. Treat this as the
                            // JXL_DEC_NEED_MORE_INPUT case.
                            break;
                        } else {
                            metadata.resize(dec.recon_out_buffer_pos, 0);
                            if dec.store_exif == 1 {
                                dec.store_exif = 2;
                            }
                            if dec.store_xmp == 1 {
                                dec.store_xmp = 2;
                            }
                            break;
                        }
                    } else {
                        // error
                        return box_result;
                    }
                }
            }
        }
        if dec.recon_output_jpeg == JpegReconStage::SettingMetadata && !dec.jbrd_need_more_boxes() {
            // The source dereferences the image bundle's JPEG data.
            let ib = dec
                .ib
                .clone()
                .expect("the source dereferences the reconstruction bundle");
            let mut ib = ib.borrow_mut();
            let jpeg_data = ib
                .jpeg_data
                .as_deref_mut()
                .expect("the source dereferences the reconstruction JPEG data");
            if dec.recon_exif_size != 0 {
                let status = JxlToJpegDecoder::set_exif(&dec.exif_metadata, jpeg_data);
                if status != JxlDecoderStatus::SUCCESS {
                    return status;
                }
            }
            if dec.recon_xmp_size != 0 {
                let status = JxlToJpegDecoder::set_xmp(&dec.xmp_metadata, jpeg_data);
                if status != JxlDecoderStatus::SUCCESS {
                    return status;
                }
            }
            dec.recon_output_jpeg = JpegReconStage::Outputting;
        }

        if dec.recon_output_jpeg == JpegReconStage::Outputting && !dec.jbrd_need_more_boxes() {
            let ib = dec
                .ib
                .clone()
                .expect("the source dereferences the reconstruction bundle");
            let status = {
                let ib = ib.borrow();
                let jpeg_data = ib
                    .jpeg_data
                    .as_deref()
                    .expect("the source dereferences the reconstruction JPEG data");
                // SAFETY: jxl_decoder_set_jpeg_buffer's caller contract covers
                // the stored output buffer, which is disjoint from the JPEG data.
                unsafe { dec.jpeg_decoder.write_output(jpeg_data) }
            };
            drop(ib);
            if status != JxlDecoderStatus::SUCCESS {
                return status;
            }
            dec.recon_output_jpeg = JpegReconStage::None;
            dec.ib = None;
            if (dec.events_wanted & DEC_FULL_IMAGE) != 0 {
                // Return the full image event here now, this may be delayed if
                // this could only be done after decoding an exif or xmp box
                // after the codestream.
                return JxlDecoderStatus::FULL_IMAGE;
            }
        }

        if dec.box_stage == BoxStage::Header {
            if !dec.have_container {
                if dec.stage == DecoderStage::CodestreamFinished {
                    return JxlDecoderStatus::SUCCESS;
                }
                dec.box_stage = BoxStage::Codestream;
                dec.input.box_contents_unbounded = true;
                continue;
            }
            if dec.avail_in() == 0 {
                if dec.stage != DecoderStage::CodestreamFinished {
                    // Not yet seen (all) codestream boxes.
                    return JxlDecoderStatus::NEED_MORE_INPUT;
                }
                if dec.jbrd_need_more_boxes() {
                    return JxlDecoderStatus::NEED_MORE_INPUT;
                }
                if dec.input_closed() {
                    return JxlDecoderStatus::SUCCESS;
                }
                if (dec.events_wanted & DEC_BOX) == 0 {
                    // All codestream and jbrd metadata boxes finished, and no
                    // individual boxes requested by user, so no need to request
                    // any more input. This returns success for backwards
                    // compatibility, when JxlDecoderCloseInput and JXL_DEC_BOX
                    // did not exist, as well as for efficiency.
                    return JxlDecoderStatus::SUCCESS;
                }
                // Even though we are exactly at a box end, there still may be
                // more boxes. The user may call JxlDecoderCloseInput to indicate
                // the input is finished and get success instead.
                return JxlDecoderStatus::NEED_MORE_INPUT;
            }

            let boxed_codestream_done = (dec.events_wanted & DEC_BOX) != 0
                && dec.stage == DecoderStage::CodestreamFinished
                && !dec.jbrd_need_more_boxes()
                && dec.input.last_codestream_seen;
            // SAFETY: avail_in > 0 here; the bytes are the caller's input.
            let input = unsafe { dec.input_bytes() };
            if boxed_codestream_done
                && dec.avail_in() >= 2
                && input[0] == 0xff
                && input[1] == CODESTREAM_MARKER
            {
                // We detected the start of the next naked codestream, so we can
                // return success here.
                return JxlDecoderStatus::SUCCESS;
            }

            let mut box_size = 0usize;
            let mut header_size = 0usize;
            let status = parse_box_header(
                input,
                0,
                dec.input.file_pos,
                &mut dec.box_type,
                &mut box_size,
                &mut header_size,
            );
            if status != JxlDecoderStatus::SUCCESS {
                if status == JxlDecoderStatus::NEED_MORE_INPUT {
                    dec.basic_info_size_hint = initial_basic_info_size_hint()
                        .wrapping_add(header_size)
                        .wrapping_sub(dec.input.file_pos);
                }
                return status;
            }
            if &dec.box_type == b"brob" {
                if dec.avail_in() < header_size + 4 {
                    return JxlDecoderStatus::NEED_MORE_INPUT;
                }
                dec.box_decoded_type
                    .copy_from_slice(&input[header_size..header_size + 4]);
            } else {
                dec.box_decoded_type = dec.box_type;
            }

            // Box order validity checks
            // The signature box at box_count == 1 is not checked here since
            // that's already done at the beginning.
            dec.box_count = dec.box_count.wrapping_add(1);
            if boxed_codestream_done && &dec.box_type == b"JXL " {
                // We detected the start of the next boxed stream, so we can
                // return success here.
                return JxlDecoderStatus::SUCCESS;
            }
            if dec.box_count == 2 && &dec.box_type != b"ftyp" {
                return input_error!("the second box must be the ftyp box");
            }
            if &dec.box_type == b"ftyp" && dec.box_count != 2 {
                return input_error!("the ftyp box must come second");
            }

            dec.input.box_contents_unbounded = box_size == 0;
            dec.box_contents_begin = dec.input.file_pos.wrapping_add(header_size);
            dec.input.box_contents_end = if dec.input.box_contents_unbounded {
                0
            } else {
                dec.input.file_pos.wrapping_add(box_size)
            };
            dec.box_contents_size = if dec.input.box_contents_unbounded {
                0
            } else {
                box_size.wrapping_sub(header_size)
            };
            dec.box_size = box_size;
            dec.header_size = header_size;
            if (dec.orig_events_wanted & DEC_JPEG_RECONSTRUCTION) != 0 {
                // Initiate storing of Exif or XMP data for JPEG reconstruction
                if dec.store_exif == 0 && &dec.box_decoded_type == b"Exif" {
                    dec.store_exif = 1;
                    dec.recon_out_buffer_pos = 0;
                }
                if dec.store_xmp == 0 && &dec.box_decoded_type == b"xml " {
                    dec.store_xmp = 1;
                    dec.recon_out_buffer_pos = 0;
                }
            }
            if (dec.events_wanted & DEC_BOX) != 0 {
                let decompress = dec.decompress_boxes && &dec.box_type == b"brob";
                dec.box_content_decoder.start_box(
                    decompress,
                    dec.input.box_contents_unbounded,
                    dec.box_contents_size,
                );
            }
            if dec.store_exif == 1 || dec.store_xmp == 1 {
                let brob = &dec.box_type == b"brob";
                dec.metadata_decoder.start_box(
                    brob,
                    dec.input.box_contents_unbounded,
                    dec.box_contents_size,
                );
            }
            if &dec.box_type == b"ftyp" {
                dec.box_stage = BoxStage::Ftyp;
            } else if &dec.box_type == b"jxlc" {
                if dec.input.last_codestream_seen {
                    return input_error!("there can only be one jxlc box");
                }
                dec.input.last_codestream_seen = true;
                dec.box_stage = BoxStage::Codestream;
            } else if &dec.box_type == b"jxlp" {
                dec.box_stage = BoxStage::PartialCodestream;
            } else if (dec.orig_events_wanted & DEC_JPEG_RECONSTRUCTION) != 0
                && &dec.box_type == b"jbrd"
            {
                if (dec.events_wanted & DEC_JPEG_RECONSTRUCTION) == 0 {
                    return input_error!("multiple JPEG reconstruction boxes not supported");
                }
                dec.box_stage = BoxStage::JpegRecon;
            } else {
                dec.box_stage = BoxStage::Skip;
            }

            if (dec.events_wanted & DEC_BOX) != 0 {
                dec.box_event = true;
                dec.box_out_buffer_set_current_box = false;
                return JxlDecoderStatus::BOX;
            }
        } else if dec.box_stage == BoxStage::Ftyp {
            if dec.box_contents_size < 12 {
                return input_error!("file type box too small");
            }
            if dec.avail_in() < 8 {
                return JxlDecoderStatus::NEED_MORE_INPUT;
            }
            // SAFETY: At least eight caller input bytes are available.
            let input = unsafe { dec.input_bytes() };
            if &input[..4] != b"jxl " {
                return input_error!("file type box major brand must be \"jxl \"");
            }
            let version = u32::from_be_bytes(input[4..8].try_into().unwrap());
            if version > 1 {
                return input_error!("unknown jxl file format version");
            }
            dec.jxl_file_format_version = version;
            dec.advance_input(8);
            dec.box_stage = BoxStage::Skip;
        } else if dec.box_stage == BoxStage::PartialCodestream {
            if dec.input.last_codestream_seen {
                return input_error!("cannot have jxlp box after last jxlp box");
            }
            // TODO(lode): error if box is unbounded but last bit not set
            if dec.avail_in() < 4 {
                return JxlDecoderStatus::NEED_MORE_INPUT;
            }
            if !dec.input.box_contents_unbounded && dec.box_contents_size < 4 {
                return input_error!("jxlp box too small to contain index");
            }
            // SAFETY: At least four caller input bytes are available.
            let input = unsafe { dec.input_bytes() };
            let jxlp_index = u32::from_be_bytes(input[..4].try_into().unwrap());
            let counter = jxlp_index & 0x7FFF_FFFF;
            let is_last = (jxlp_index & 0x8000_0000) != 0;

            if counter < dec.input.next_jxlp_index {
                return input_error!("jxlp box index is a duplicate (already processed)");
            }
            dec.advance_input(4); // consume the 4-byte jxlp counter header

            if counter == dec.input.next_jxlp_index {
                dec.input.next_jxlp_index = dec.input.next_jxlp_index.wrapping_add(1);
                if is_last {
                    dec.input.last_codestream_seen = true;
                }
                dec.box_stage = BoxStage::Codestream;
            } else if dec.jxl_file_format_version >= 1 {
                // Out-of-order box (version 1+): buffer payload for later
                // injection. Reject a counter that is already buffered.
                if dec.input.jxlp_ooo_buffer.len() >= JxlDecoder::NUM_BUFFERS_LIMIT {
                    return JxlDecoderStatus::ERROR;
                }
                // SAFETY: The stable manager cell outlives the payload, which
                // the input owner releases before the cell.
                let buffer = Box::new(unsafe {
                    super::padded_bytes_callbacks::PaddedBytes::new(dec.manager_ptr())
                });
                let entry = (buffer, is_last);
                match dec.input.jxlp_ooo_buffer.entry(counter) {
                    std::collections::btree_map::Entry::Occupied(_) => {
                        // emplace does not insert; the new pair is destroyed.
                        drop(entry);
                        return input_error!("duplicate jxlp box index");
                    }
                    std::collections::btree_map::Entry::Vacant(slot) => {
                        slot.insert(entry);
                    }
                }
                dec.buffering_jxlp_index = counter;
                dec.buffering_jxlp_is_last = is_last;
                dec.box_stage = BoxStage::BufferingJxlp;
            } else {
                return input_error!(
                    "jxlp box index is out of order; out-of-order jxlp boxes require file format version 1"
                );
            }
        } else if dec.box_stage == BoxStage::Codestream {
            let status = jxl_decoder_process_codestream(dec);
            if status == JxlDecoderStatus::FULL_IMAGE
                && dec.recon_output_jpeg != JpegReconStage::None
            {
                continue;
            }
            if status == JxlDecoderStatus::NEED_MORE_INPUT
                && dec.input.file_pos == dec.input.box_contents_end
                && !dec.input.box_contents_unbounded
            {
                // Physical box exhausted; inject the next buffered OOO box if
                // ready.
                let mut has_more_data = false;
                api_return_if_status_error!(dec.inject_next_buffered_jxlp_box(&mut has_more_data));
                if has_more_data {
                    continue;
                }
                dec.box_stage = BoxStage::Header;
                continue;
            }

            if status == JxlDecoderStatus::SUCCESS {
                if dec.jbrd_need_more_boxes() {
                    dec.box_stage = BoxStage::Skip;
                    continue;
                }
                if dec.input.box_contents_unbounded {
                    // Last box reached and codestream done, nothing more to do.
                    break;
                }
                if (dec.events_wanted & DEC_BOX) != 0 {
                    // Codestream done, but there may be more other boxes.
                    dec.box_stage = BoxStage::Skip;
                    continue;
                }
            }
            return status;
        } else if dec.box_stage == BoxStage::BufferingJxlp {
            let remaining = if dec.input.box_contents_unbounded {
                dec.avail_in()
            } else {
                dec.avail_in()
                    .min(dec.input.box_contents_end.wrapping_sub(dec.input.file_pos))
            };
            if !dec.input.can_add_buffer(remaining) {
                return JxlDecoderStatus::ERROR;
            }
            let next_in = dec.next_in();
            // The partial-codestream stage emplaced this entry, and neither
            // injection nor removal occurs while buffering. A missing entry
            // would make the source dereference a null payload owner.
            let entry = dec
                .input
                .jxlp_ooo_buffer
                .get_mut(&dec.buffering_jxlp_index)
                .expect("the buffered jxlp entry exists");
            // SAFETY: The retained caller range covers `remaining` bytes; it is
            // disjoint from the payload owner and no loan spans the callback.
            api_return_if_status_error!(unsafe {
                entry.0.append_raw(next_in, next_in.wrapping_add(remaining))
            });
            dec.input.jxlp_ooo_buffer_total =
                dec.input.jxlp_ooo_buffer_total.wrapping_add(remaining);
            dec.advance_input(remaining);

            let box_done = !dec.input.box_contents_unbounded
                && dec.input.file_pos >= dec.input.box_contents_end;
            if !box_done {
                return JxlDecoderStatus::NEED_MORE_INPUT;
            }
            // Box fully buffered; parse the next box header.
            dec.box_stage = BoxStage::Header;
        } else if dec.box_stage == BoxStage::JpegRecon {
            if !dec.jpeg_decoder.is_parsing_box() {
                // This is a new JPEG reconstruction metadata box.
                dec.jpeg_decoder
                    .start_box(dec.input.box_contents_unbounded, dec.box_contents_size);
            }
            // SAFETY: The SetInput contract covers the current input range.
            let mut next_in = unsafe { dec.input_bytes() };
            let avail_in = next_in.len();
            let recon_result = dec.jpeg_decoder.process(&mut next_in);
            let consumed = avail_in - next_in.len();
            dec.advance_input(consumed);
            if recon_result == JxlDecoderStatus::JPEG_RECONSTRUCTION {
                let jpeg_data = dec
                    .jpeg_decoder
                    .get_jpeg_data()
                    .expect("the source dereferences the parsed JPEG data");
                let num_exif = JxlToJpegDecoder::num_exif_markers(jpeg_data);
                let num_xmp = JxlToJpegDecoder::num_xmp_markers(jpeg_data);
                if num_exif != 0 {
                    if num_exif > 1 {
                        return input_error!(
                            "multiple exif markers for JPEG reconstruction not supported"
                        );
                    }
                    if JxlToJpegDecoder::exif_box_content_size(jpeg_data, &mut dec.recon_exif_size)
                        != JxlDecoderStatus::SUCCESS
                    {
                        return input_error!("invalid jbrd exif size");
                    }
                }
                if num_xmp != 0 {
                    if num_xmp > 1 {
                        return input_error!(
                            "multiple XMP markers for JPEG reconstruction not supported"
                        );
                    }
                    if JxlToJpegDecoder::xml_box_content_size(jpeg_data, &mut dec.recon_xmp_size)
                        != JxlDecoderStatus::SUCCESS
                    {
                        return input_error!("invalid jbrd XMP size");
                    }
                }

                dec.box_stage = BoxStage::Header;
                // If successful JPEG reconstruction, return the success if the
                // user cares about it, otherwise continue.
                if (dec.events_wanted & DEC_JPEG_RECONSTRUCTION) != 0 {
                    dec.events_wanted &= !DEC_JPEG_RECONSTRUCTION;
                    return JxlDecoderStatus::JPEG_RECONSTRUCTION;
                }
            } else {
                // If anything else, return the result.
                return recon_result;
            }
        } else if dec.box_stage == BoxStage::Skip {
            if dec.input.box_contents_unbounded {
                if dec.input_closed() {
                    return JxlDecoderStatus::SUCCESS;
                }
                if !dec.box_out_buffer_set {
                    // An unbounded box is always the last box. Not requesting
                    // box data, so return success even if JxlDecoderCloseInput
                    // was not called for backwards compatibility as well as
                    // efficiency since this box is being skipped.
                    return JxlDecoderStatus::SUCCESS;
                }
                // Arbitrarily more bytes may follow, only JxlDecoderCloseInput
                // can mark the end.
                dec.advance_input(dec.avail_in());
                return JxlDecoderStatus::NEED_MORE_INPUT;
            }
            // Amount of remaining bytes in the box that is being skipped.
            let remaining = dec.input.box_contents_end.wrapping_sub(dec.input.file_pos);
            if dec.avail_in() < remaining {
                // Indicate how many more bytes needed starting from next_in.
                dec.basic_info_size_hint = initial_basic_info_size_hint()
                    .wrapping_add(dec.input.box_contents_end)
                    .wrapping_sub(dec.input.file_pos);
                // Don't have the full box yet, skip all we have so far
                dec.advance_input(dec.avail_in());
                return JxlDecoderStatus::NEED_MORE_INPUT;
            } else {
                // Full box available, skip all its remaining bytes
                dec.advance_input(remaining);
                dec.box_stage = BoxStage::Header;
            }
        }
    }
    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderProcessInput.
///
/// # Safety
/// The current input satisfies the jxl_decoder_set_input contract, and every
/// caller buffer and callback registered with the decoder remains valid for
/// its documented use. Callbacks never unwind or reenter the decoder.
pub unsafe fn jxl_decoder_process_input(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    if dec.stage == DecoderStage::Inited {
        dec.stage = DecoderStage::Started;
    }
    if dec.stage == DecoderStage::Error {
        return api_error!(
            "Cannot keep using decoder after it encountered an error, use JxlDecoderReset to reset it"
        );
    }

    if !dec.got_signature {
        // SAFETY: The SetInput contract covers the current input range.
        let sig = jxl_signature_check(unsafe { dec.input_bytes() });
        if sig == JxlSignature::INVALID {
            return input_error!("invalid signature");
        }
        if sig == JxlSignature::NOT_ENOUGH_BYTES {
            if dec.input_closed() {
                return input_error!("file too small for signature");
            }
            return JxlDecoderStatus::NEED_MORE_INPUT;
        }

        dec.got_signature = true;

        if sig == JxlSignature::CONTAINER {
            dec.have_container = true;
        } else {
            dec.input.last_codestream_seen = true;
        }
    }

    // SAFETY: Forward the caller's contract.
    let status = unsafe { handle_boxes(dec) };

    if status == JxlDecoderStatus::NEED_MORE_INPUT && dec.input_closed() {
        return input_error!("premature end of input");
    }

    // Even if the box handling returns success, certain types of
    // data may be missing.
    if status == JxlDecoderStatus::SUCCESS {
        if dec.can_use_more_codestream_input() {
            return input_error!("codestream never finished");
        }
        if dec.jbrd_need_more_boxes() {
            return input_error!("missing metadata boxes for jpeg reconstruction");
        }
    }

    status
}

pub fn jxl_decoder_get_basic_info(
    dec: &JxlDecoder,
    info: Option<&mut JxlBasicInfo>,
) -> JxlDecoderStatus {
    if !dec.got_basic_info {
        return JxlDecoderStatus::NEED_MORE_INPUT;
    }

    if let Some(info) = info {
        *info = JxlBasicInfo::zeroed();

        let metadata = dec.metadata();
        let meta = &metadata.m;

        info.have_container = i32::from(dec.have_container);
        info.xsize = metadata.size.xsize() as u32;
        info.ysize = metadata.size.ysize() as u32;
        info.uses_original_profile = i32::from(!meta.xyb_encoded);

        info.bits_per_sample = meta.bit_depth.bits_per_sample;
        info.exponent_bits_per_sample = meta.bit_depth.exponent_bits_per_sample;

        info.have_preview = i32::from(meta.have_preview);
        info.have_animation = i32::from(meta.have_animation);
        info.orientation = JxlOrientation(meta.orientation);

        if !dec.keep_orientation {
            if info.orientation.0 >= JxlOrientation::TRANSPOSE.0 {
                std::mem::swap(&mut info.xsize, &mut info.ysize);
            }
            info.orientation = JxlOrientation::IDENTITY;
        }

        info.intensity_target = meta.intensity_target();
        if dec.desired_intensity_target > 0.0 {
            info.intensity_target = dec.desired_intensity_target;
        }
        info.min_nits = meta.tone_mapping.min_nits;
        info.relative_to_max_display = i32::from(meta.tone_mapping.relative_to_max_display);
        info.linear_below = meta.tone_mapping.linear_below;

        if let Some(alpha) = meta.find(ExtraChannel::ALPHA) {
            info.alpha_bits = alpha.bit_depth.bits_per_sample;
            info.alpha_exponent_bits = alpha.bit_depth.exponent_bits_per_sample;
            info.alpha_premultiplied = i32::from(alpha.alpha_associated);
        } else {
            info.alpha_bits = 0;
            info.alpha_exponent_bits = 0;
            info.alpha_premultiplied = 0;
        }

        info.num_color_channels = if meta.color_encoding.get_color_space() == ColorSpace::GRAY {
            1
        } else {
            3
        };

        info.num_extra_channels = meta.num_extra_channels;

        if info.have_preview != 0 {
            info.preview.xsize = meta.preview_size.xsize() as u32;
            info.preview.ysize = meta.preview_size.ysize() as u32;
        }

        if info.have_animation != 0 {
            info.animation.tps_numerator = meta.animation.tps_numerator;
            info.animation.tps_denominator = meta.animation.tps_denominator;
            info.animation.num_loops = meta.animation.num_loops;
            info.animation.have_timecodes = i32::from(meta.animation.have_timecodes);
        }

        if meta.have_intrinsic_size {
            info.intrinsic_xsize = meta.intrinsic_size.xsize() as u32;
            info.intrinsic_ysize = meta.intrinsic_size.ysize() as u32;
        } else {
            info.intrinsic_xsize = info.xsize;
            info.intrinsic_ysize = info.ysize;
        }
    }

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_extra_channel_info(
    dec: &JxlDecoder,
    index: usize,
    info: &mut JxlExtraChannelInfo,
) -> JxlDecoderStatus {
    if !dec.got_basic_info {
        return JxlDecoderStatus::NEED_MORE_INPUT;
    }

    let channels = &dec.metadata().m.extra_channel_info;

    if index >= channels.len() {
        return JxlDecoderStatus::ERROR; // out of bounds
    }
    let channel = &channels[index];

    info.channel_type = JxlExtraChannelType(channel.channel_type.0);
    info.bits_per_sample = channel.bit_depth.bits_per_sample;
    info.exponent_bits_per_sample = if channel.bit_depth.floating_point_sample {
        channel.bit_depth.exponent_bits_per_sample
    } else {
        0
    };
    info.dim_shift = channel.dim_shift;
    info.name_length = channel.name.len() as u32;
    info.alpha_premultiplied = i32::from(channel.alpha_associated);
    info.spot_color[0] = channel.spot_color[0];
    info.spot_color[1] = channel.spot_color[1];
    info.spot_color[2] = channel.spot_color[2];
    info.spot_color[3] = channel.spot_color[3];
    info.cfa_channel = channel.cfa_channel;

    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderGetExtraChannelName.
///
/// # Safety
/// `name` is writable for `size` bytes whenever the name fits.
pub unsafe fn jxl_decoder_get_extra_channel_name(
    dec: &JxlDecoder,
    index: usize,
    name: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    if !dec.got_basic_info {
        return JxlDecoderStatus::NEED_MORE_INPUT;
    }

    let channels = &dec.metadata().m.extra_channel_info;

    if index >= channels.len() {
        return JxlDecoderStatus::ERROR; // out of bounds
    }
    let channel = &channels[index];

    // Also need null-termination character
    if channel.name.len().wrapping_add(1) > size {
        return JxlDecoderStatus::ERROR;
    }

    // SAFETY: The caller's buffer holds `size` >= name length + 1 bytes and
    // is disjoint from the decoder's metadata.
    unsafe {
        ptr::copy_nonoverlapping(channel.name.as_ptr(), name, channel.name.len());
        name.add(channel.name.len()).write(0);
    }

    JxlDecoderStatus::SUCCESS
}

/// Gets the ColorEncoding for the desired target, and checks errors.
fn get_color_encoding_for_target(
    dec: &JxlDecoder,
    target: JxlColorProfileTarget,
) -> Result<&ColorEncoding, JxlDecoderStatus> {
    if !dec.got_all_headers {
        return Err(JxlDecoderStatus::NEED_MORE_INPUT);
    }
    if target == JxlColorProfileTarget::DATA && dec.metadata().m.xyb_encoded {
        Ok(&dec
            .passes_state
            .as_ref()
            .expect("source dereferences passes_state")
            .output_encoding_info
            .color_encoding)
    } else {
        Ok(&dec.metadata().m.color_encoding)
    }
}

pub fn jxl_decoder_get_color_as_encoded_profile(
    dec: &JxlDecoder,
    target: JxlColorProfileTarget,
    color_encoding: Option<&mut JxlColorEncoding>,
) -> JxlDecoderStatus {
    let jxl_color_encoding = match get_color_encoding_for_target(dec, target) {
        Ok(encoding) => encoding,
        Err(status) => return status,
    };

    if jxl_color_encoding.want_icc() {
        return JxlDecoderStatus::ERROR; // Indicate no encoded profile available.
    }

    if let Some(color_encoding) = color_encoding {
        *color_encoding = jxl_color_encoding.to_external();
    }

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_icc_profile_size(
    dec: &JxlDecoder,
    target: JxlColorProfileTarget,
    size: Option<&mut usize>,
) -> JxlDecoderStatus {
    let jxl_color_encoding = match get_color_encoding_for_target(dec, target) {
        Ok(encoding) => encoding,
        Err(status) => return status,
    };

    if jxl_color_encoding.want_icc() {
        let color_space = dec.metadata().m.color_encoding.get_color_space();
        if color_space == ColorSpace::UNKNOWN || color_space == ColorSpace::XYB {
            // This indicates there's no ICC profile available
            return JxlDecoderStatus::ERROR;
        }
    }

    if let Some(size) = size {
        *size = jxl_color_encoding.icc().len();
    }

    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderGetColorAsICCProfile.
///
/// # Safety
/// `icc_profile` is writable for `size` bytes.
pub unsafe fn jxl_decoder_get_color_as_icc_profile(
    dec: &JxlDecoder,
    target: JxlColorProfileTarget,
    icc_profile: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    let mut wanted_size = 0usize;
    // This also checks the NEED_MORE_INPUT and the unknown/xyb cases
    let status = jxl_decoder_get_icc_profile_size(dec, target, Some(&mut wanted_size));
    if status != JxlDecoderStatus::SUCCESS {
        return status;
    }
    if size < wanted_size {
        return api_error!("ICC profile output too small");
    }

    let jxl_color_encoding = match get_color_encoding_for_target(dec, target) {
        Ok(encoding) => encoding,
        Err(status) => return status,
    };

    let icc = jxl_color_encoding.icc();
    if !icc.is_empty() {
        // SAFETY: The caller's buffer holds `size` >= ICC length bytes and is
        // disjoint from the decoder's encoding storage.
        unsafe {
            ptr::copy_nonoverlapping(icc.as_ptr(), icc_profile, icc.len());
        }
    }

    JxlDecoderStatus::SUCCESS
}

/// PrepareSizeCheck: the bits per channel for buffer size computation.
fn prepare_size_check(
    dec: &JxlDecoder,
    format: &JxlPixelFormat,
    bits: &mut usize,
) -> JxlDecoderStatus {
    if !dec.got_basic_info {
        // Don't know image dimensions yet, cannot check for valid size.
        return JxlDecoderStatus::NEED_MORE_INPUT;
    }
    if !dec.coalescing && (dec.frame_header.is_none() || dec.frame_stage == FrameStage::Header) {
        return api_error!("Don't know frame dimensions yet");
    }
    if format.num_channels > 4 {
        return api_error!("More than 4 channels not supported");
    }

    *bits = bits_per_channel(format.data_type);

    if *bits == 0 {
        return api_error!("Invalid/unsupported data type");
    }

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_intended_downsampling_ratio(dec: &JxlDecoder) -> usize {
    dec.downsampling_target
}

/// JxlDecoderFlushImage.
///
/// # Safety
/// The registered output buffers and callbacks satisfy their contracts.
pub unsafe fn jxl_decoder_flush_image(dec: &mut JxlDecoder) -> JxlDecoderStatus {
    if !dec.image_out_buffer_set {
        return JxlDecoderStatus::ERROR;
    }
    if dec.frame_stage != FrameStage::Full {
        return JxlDecoderStatus::ERROR;
    }
    debug_assert!(dec.frame_dec.is_some());
    let frame_dec = dec
        .frame_dec
        .as_mut()
        .expect("the full stage has a frame decoder");
    if !frame_dec.has_decoded_dc() {
        // FrameDecoder::Flush currently requires DC to have been decoded
        // already to work correctly.
        return JxlDecoderStatus::ERROR;
    }

    let passes_state = dec
        .passes_state
        .as_mut()
        .expect("ReadAllHeaders allocates passes_state");
    if frame_dec.flush(passes_state).is_err() {
        return JxlDecoderStatus::ERROR;
    }

    JxlDecoderStatus::SUCCESS
}

fn safe_mul(a: usize, b: usize, product: &mut usize) -> bool {
    *product = 0;
    if a == 0 || b == 0 {
        return true;
    }
    if b > usize::MAX / a {
        return false;
    }
    *product = a * b;
    true
}

fn safe_add(a: usize, b: usize, sum: &mut usize) -> bool {
    *sum = a.wrapping_add(b);
    *sum >= a
}

fn safe_round_up_to(what: usize, align: usize, result: &mut usize) -> bool {
    if align < 2 {
        *result = what;
        return true;
    }
    let reminder = what % align;
    if reminder == 0 {
        *result = what;
        return true;
    }
    safe_add(what, align - reminder, result)
}

fn get_min_size(
    dec: &JxlDecoder,
    format: &JxlPixelFormat,
    mut num_channels: usize,
    min_size: &mut usize,
    preview: bool,
) -> JxlDecoderStatus {
    let mut bits = 0usize;
    let status = prepare_size_check(dec, format, &mut bits);
    if status != JxlDecoderStatus::SUCCESS {
        return status;
    }
    let mut xsize = 0usize;
    let mut ysize = 0usize;
    if preview {
        xsize = dec.metadata().oriented_preview_xsize(dec.keep_orientation);
        ysize = dec.metadata().oriented_preview_ysize(dec.keep_orientation);
    } else {
        get_current_dimensions(dec, &mut xsize, &mut ysize);
    }
    if num_channels == 0 {
        num_channels = format.num_channels as usize;
    }
    let mut row_bits = 0usize;
    // SafeMul takes its operands by value before assigning the product.
    if !safe_mul(xsize, num_channels, &mut row_bits) || {
        let row_bits_in = row_bits;
        !safe_mul(row_bits_in, bits, &mut row_bits)
    } {
        return api_error!("Image too large for output buffer size calculation");
    }
    let mut row_size = row_bits.wrapping_add(8 - 1) / 8;
    let last_row_size = row_size;
    let what = row_size;
    if !safe_round_up_to(what, format.align, &mut row_size) {
        return api_error!("Image too large for output buffer size calculation");
    }

    let mut total = 0usize;
    if ysize > 1 && !safe_mul(row_size, ysize - 1, &mut total) {
        return api_error!("Image too large for output buffer size calculation");
    }
    if !safe_add(total, last_row_size, min_size) {
        return api_error!("Image too large for output buffer size calculation");
    }
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_preview_out_buffer_size(
    dec: &JxlDecoder,
    format: &JxlPixelFormat,
    size: &mut usize,
) -> JxlDecoderStatus {
    if format.num_channels < 3 && !dec.image_metadata.get().color_encoding.is_gray() {
        return api_error!("Number of channels is too low for color output");
    }
    get_min_size(dec, format, 0, size, true)
}

/// JxlDecoderSetPreviewOutBuffer.
///
/// # Safety
/// `buffer` is writable for `size` bytes until the preview event or until the
/// decoder releases the buffer, and is disjoint from the decoder.
pub unsafe fn jxl_decoder_set_preview_out_buffer(
    dec: &mut JxlDecoder,
    format: &JxlPixelFormat,
    buffer: *mut c_void,
    size: usize,
) -> JxlDecoderStatus {
    if !dec.got_basic_info
        || !dec.metadata().m.have_preview
        || (dec.orig_events_wanted & DEC_PREVIEW_IMAGE) == 0
        || dec.got_preview_image
        || !dec.preview_frame
    {
        return api_error!("No preview out buffer needed at this time");
    }
    if format.num_channels < 3 && !dec.image_metadata.get().color_encoding.is_gray() {
        return api_error!("Number of channels is too low for color output");
    }

    let mut min_size = 0usize;
    // This also checks whether the format is valid and supported and basic
    // info is available.
    let status = jxl_decoder_preview_out_buffer_size(dec, format, &mut min_size);
    if status != JxlDecoderStatus::SUCCESS {
        return status;
    }

    if size < min_size {
        return JxlDecoderStatus::ERROR;
    }

    dec.image_out_buffer_set = true;
    dec.image_out_buffer = buffer;
    dec.image_out_size = size;
    dec.image_out_format = *format;

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_image_out_buffer_size(
    dec: &JxlDecoder,
    format: &JxlPixelFormat,
    size: &mut usize,
) -> JxlDecoderStatus {
    if format.num_channels < 3 && !dec.image_metadata.get().color_encoding.is_gray() {
        return api_error!("Number of channels is too low for color output");
    }

    get_min_size(dec, format, 0, size, false)
}

/// JxlDecoderSetImageOutBuffer.
///
/// # Safety
/// `buffer` is writable for `size` bytes until the full-image event or until
/// the decoder releases the buffer, and is disjoint from the decoder.
pub unsafe fn jxl_decoder_set_image_out_buffer(
    dec: &mut JxlDecoder,
    format: &JxlPixelFormat,
    buffer: *mut c_void,
    size: usize,
) -> JxlDecoderStatus {
    if !dec.got_basic_info || (dec.orig_events_wanted & DEC_FULL_IMAGE) == 0 || dec.preview_frame {
        return api_error!("No image out buffer needed at this time");
    }
    if dec.image_out_buffer_set && dec.image_out_run_callback.is_some() {
        return api_error!("Cannot change from image out callback to image out buffer");
    }
    if format.num_channels < 3 && !dec.image_metadata.get().color_encoding.is_gray() {
        return api_error!("Number of channels is too low for color output");
    }
    let mut min_size = 0usize;
    // This also checks whether the format is valid and supported and basic
    // info is available.
    let status = jxl_decoder_image_out_buffer_size(dec, format, &mut min_size);
    if status != JxlDecoderStatus::SUCCESS {
        return status;
    }

    if size < min_size {
        return JxlDecoderStatus::ERROR;
    }

    dec.image_out_buffer_set = true;
    dec.image_out_buffer = buffer;
    dec.image_out_size = size;
    dec.image_out_format = *format;

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_extra_channel_buffer_size(
    dec: &JxlDecoder,
    format: &JxlPixelFormat,
    size: &mut usize,
    index: u32,
) -> JxlDecoderStatus {
    if !dec.got_basic_info || (dec.orig_events_wanted & DEC_FULL_IMAGE) == 0 {
        return api_error!("No extra channel buffer needed at this time");
    }

    if index >= dec.metadata().m.num_extra_channels {
        return api_error!("Invalid extra channel index");
    }

    get_min_size(dec, format, 1, size, false)
}

/// JxlDecoderSetExtraChannelBuffer.
///
/// # Safety
/// `buffer` is writable for `size` bytes until the full-image event or until
/// the decoder releases the buffer, and is disjoint from the decoder.
pub unsafe fn jxl_decoder_set_extra_channel_buffer(
    dec: &mut JxlDecoder,
    format: &JxlPixelFormat,
    buffer: *mut c_void,
    size: usize,
    index: u32,
) -> JxlDecoderStatus {
    let mut min_size = 0usize;
    // This also checks whether the format and index are valid and supported
    // and basic info is available.
    let status = jxl_decoder_extra_channel_buffer_size(dec, format, &mut min_size, index);
    if status != JxlDecoderStatus::SUCCESS {
        return status;
    }

    if size < min_size {
        return JxlDecoderStatus::ERROR;
    }

    if dec.extra_channel_output.len() <= index as usize {
        let num_extra_channels = dec.metadata().m.num_extra_channels as usize;
        dec.extra_channel_output.resize(
            num_extra_channels,
            ExtraChannelOutput {
                format: JxlPixelFormat {
                    num_channels: 0,
                    data_type: JxlDataType(0),
                    endianness: super::types::JxlEndianness(0),
                    align: 0,
                },
                buffer: ptr::null_mut(),
                buffer_size: 0,
            },
        );
    }
    // Guaranteed correct thanks to check in JxlDecoderExtraChannelBufferSize.
    debug_assert!(dec.extra_channel_output.len() > index as usize);

    let output = &mut dec.extra_channel_output[index as usize];
    output.format = *format;
    output.format.num_channels = 1;
    output.buffer = buffer;
    output.buffer_size = size;

    JxlDecoderStatus::SUCCESS
}

unsafe extern "C" fn simple_init_callback(
    init_opaque: *mut c_void,
    _num_threads: usize,
    _num_pixels_per_thread: usize,
) -> *mut c_void {
    // No initialization to do, just reuse init_opaque as run_opaque.
    init_opaque
}

unsafe extern "C" fn simple_run_callback(
    run_opaque: *mut c_void,
    _thread_id: usize,
    x: usize,
    y: usize,
    num_pixels: usize,
    pixels: *const c_void,
) {
    // SAFETY: run_opaque is the decoder's own SimpleImageOutCallback, set by
    // jxl_decoder_set_image_out_callback and stable with the decoder object.
    let simple_callback = unsafe { &*run_opaque.cast::<SimpleImageOutCallback>() };
    // SAFETY: The caller's simple callback contract.
    unsafe {
        (simple_callback
            .callback
            .expect("the source calls the stored simple callback"))(
            simple_callback.opaque,
            x,
            y,
            num_pixels,
            pixels,
        );
    }
}

unsafe extern "C" fn simple_destroy_callback(_run_opaque: *mut c_void) {}

/// JxlDecoderSetImageOutCallback.
///
/// # Safety
/// The callback and its opaque value obey decode.h until the full-image event
/// and never unwind or reenter the decoder.
pub unsafe fn jxl_decoder_set_image_out_callback(
    dec: &mut JxlDecoder,
    format: &JxlPixelFormat,
    callback: Option<JxlImageOutCallback>,
    opaque: *mut c_void,
) -> JxlDecoderStatus {
    // SAFETY: A run callback reads this cell only during ProcessInput, never
    // concurrently with this exclusive update.
    let simple = unsafe { dec.simple_image_out_callback.get_mut() };
    simple.callback = callback;
    simple.opaque = opaque;
    let init_opaque = dec
        .simple_image_out_callback
        .as_ptr()
        .cast_mut()
        .cast::<c_void>();
    // SAFETY: The wrappers forward to the caller's callback contract.
    unsafe {
        jxl_decoder_set_multithreaded_image_out_callback(
            dec,
            format,
            Some(simple_init_callback),
            Some(simple_run_callback),
            Some(simple_destroy_callback),
            init_opaque,
        )
    }
}

/// JxlDecoderSetMultithreadedImageOutCallback.
///
/// # Safety
/// The callbacks and opaque value obey decode.h until the full-image event and
/// never unwind or reenter the decoder.
pub unsafe fn jxl_decoder_set_multithreaded_image_out_callback(
    dec: &mut JxlDecoder,
    format: &JxlPixelFormat,
    init_callback: Option<ImageOutInitCallback>,
    run_callback: Option<ImageOutRunCallback>,
    destroy_callback: Option<ImageOutDestroyCallback>,
    init_opaque: *mut c_void,
) -> JxlDecoderStatus {
    if dec.image_out_buffer_set && !dec.image_out_buffer.is_null() {
        return api_error!("Cannot change from image out buffer to image out callback");
    }

    if init_callback.is_none() || run_callback.is_none() || destroy_callback.is_none() {
        return api_error!("All callbacks are required");
    }

    // Perform error checking for invalid format.
    let mut bits_sink = 0usize;
    let status = prepare_size_check(dec, format, &mut bits_sink);
    if status != JxlDecoderStatus::SUCCESS {
        return status;
    }

    dec.image_out_buffer_set = true;
    dec.image_out_init_callback = init_callback;
    dec.image_out_run_callback = run_callback;
    dec.image_out_destroy_callback = destroy_callback;
    dec.image_out_init_opaque = init_opaque;
    dec.image_out_format = *format;

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_frame_header(
    dec: &JxlDecoder,
    header: &mut JxlFrameHeader,
) -> JxlDecoderStatus {
    let Some(frame_header) = dec.frame_header.as_deref() else {
        return api_error!("no frame header available");
    };
    if dec.frame_stage == FrameStage::Header {
        return api_error!("no frame header available");
    }
    let metadata = &dec.metadata().m;
    *header = JxlFrameHeader::default();
    if metadata.have_animation {
        header.duration = frame_header.animation_frame.duration;
        if metadata.animation.have_timecodes {
            header.timecode = frame_header.animation_frame.timecode;
        }
    }
    header.name_length = frame_header.name.len() as u32;
    header.is_last = i32::from(frame_header.is_last);
    let mut xsize = 0usize;
    let mut ysize = 0usize;
    get_current_dimensions(dec, &mut xsize, &mut ysize);
    header.layer_info.xsize = xsize as u32;
    header.layer_info.ysize = ysize as u32;
    if !dec.coalescing && frame_header.custom_size_or_origin {
        header.layer_info.crop_x0 = frame_header.frame_origin.x0;
        header.layer_info.crop_y0 = frame_header.frame_origin.y0;
        header.layer_info.have_crop = 1;
    } else {
        header.layer_info.crop_x0 = 0;
        header.layer_info.crop_y0 = 0;
        header.layer_info.have_crop = 0;
    }
    if !dec.keep_orientation && !dec.coalescing {
        // orient the crop offset
        let w = dec.metadata().oriented_xsize(false);
        let h = dec.metadata().oriented_ysize(false);
        if metadata.orientation > 4 {
            std::mem::swap(
                &mut header.layer_info.crop_x0,
                &mut header.layer_info.crop_y0,
            );
        }
        let o = (metadata.orientation.wrapping_sub(1) & 3) as usize;
        if o > 0 && o < 3 {
            header.layer_info.crop_x0 = w
                .wrapping_sub(xsize)
                .wrapping_sub(header.layer_info.crop_x0 as isize as usize)
                as i32;
        }
        if o > 1 {
            header.layer_info.crop_y0 = h
                .wrapping_sub(ysize)
                .wrapping_sub(header.layer_info.crop_y0 as isize as usize)
                as i32;
        }
    }
    if dec.coalescing {
        header.layer_info.blend_info.blendmode = JxlBlendMode::REPLACE;
        header.layer_info.blend_info.source = 0;
        header.layer_info.blend_info.alpha = 0;
        header.layer_info.blend_info.clamp = 0;
        header.layer_info.save_as_reference = 0;
    } else {
        header.layer_info.blend_info.blendmode = JxlBlendMode(frame_header.blending_info.mode.0);
        header.layer_info.blend_info.source = frame_header.blending_info.source;
        header.layer_info.blend_info.alpha = frame_header.blending_info.alpha_channel;
        header.layer_info.blend_info.clamp = i32::from(frame_header.blending_info.clamp);
        header.layer_info.save_as_reference = frame_header.save_as_reference;
    }
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_extra_channel_blend_info(
    dec: &JxlDecoder,
    index: usize,
    blend_info: &mut JxlBlendInfo,
) -> JxlDecoderStatus {
    let Some(frame_header) = dec.frame_header.as_deref() else {
        return api_error!("no frame header available");
    };
    if dec.frame_stage == FrameStage::Header {
        return api_error!("no frame header available");
    }
    let metadata = &dec.metadata().m;
    if index >= metadata.num_extra_channels as usize {
        return api_error!("Invalid extra channel index");
    }
    let info = &frame_header.extra_channel_blending_info[index];
    blend_info.blendmode = JxlBlendMode(info.mode.0);
    blend_info.source = info.source;
    blend_info.alpha = info.alpha_channel;
    blend_info.clamp = i32::from(info.clamp);
    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderGetFrameName.
///
/// # Safety
/// `name` is writable for `size` bytes whenever the name fits.
pub unsafe fn jxl_decoder_get_frame_name(
    dec: &JxlDecoder,
    name: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    let Some(frame_header) = dec.frame_header.as_deref() else {
        return api_error!("no frame header available");
    };
    if dec.frame_stage == FrameStage::Header {
        return api_error!("no frame header available");
    }
    if size < frame_header.name.len().wrapping_add(1) {
        return api_error!("too small frame name output buffer");
    }
    // SAFETY: The caller's buffer holds the name and its terminator.
    unsafe {
        ptr::copy_nonoverlapping(frame_header.name.as_ptr(), name, frame_header.name.len());
        name.add(frame_header.name.len()).write(0);
    }

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_preferred_color_profile(
    dec: &mut JxlDecoder,
    color_encoding: Option<&JxlColorEncoding>,
) -> JxlDecoderStatus {
    // SAFETY: No ICC data is passed.
    unsafe { jxl_decoder_set_output_color_profile(dec, color_encoding, ptr::null(), 0) }
}

/// JxlDecoderSetOutputColorProfile.
///
/// # Safety
/// A non-null `icc_data` is readable for `icc_size` initialized bytes. The CMS
/// set with jxl_decoder_set_cms satisfies its contract when it is invoked.
pub unsafe fn jxl_decoder_set_output_color_profile(
    dec: &mut JxlDecoder,
    color_encoding: Option<&JxlColorEncoding>,
    icc_data: *const u8,
    icc_size: usize,
) -> JxlDecoderStatus {
    if color_encoding.is_some() && !icc_data.is_null() {
        return api_error!("cannot set both color_encoding and icc_data");
    }
    if color_encoding.is_none() && icc_data.is_null() {
        return api_error!("one of color_encoding and icc_data must be set");
    }
    if !dec.got_all_headers {
        return api_error!("color info not yet available");
    }
    if dec.post_headers {
        return api_error!("too late to set the color encoding");
    }
    let output_encoding = &mut dec
        .passes_state
        .as_mut()
        .expect("ReadAllHeaders allocates passes_state")
        .output_encoding_info;
    // SAFETY: No image bundle retains the output image metadata before the
    // frame loop, which post_headers excludes above.
    let image_metadata = unsafe { dec.image_metadata.get_mut() };
    let mut c_out = ColorEncoding::default();
    let mut same_encoding = false;
    if let Some(color_encoding) = color_encoding {
        api_return_if_status_error!(c_out.from_external(color_encoding));
        same_encoding = c_out.same_color_encoding(&output_encoding.color_encoding);
    }
    if !output_encoding.cms_set
        && (!icc_data.is_null() || (!image_metadata.xyb_encoded && !same_encoding))
    {
        return api_error!("must set color management system via JxlDecoderSetCms");
    }
    let orig_encoding = &mut image_metadata.color_encoding;
    if !orig_encoding.have_fields() && output_encoding.cms_set {
        let mut tmp_icc = orig_encoding.icc().to_vec();
        // SAFETY: The caller's CMS contract covers this callback.
        api_return_if_status_error!(unsafe {
            orig_encoding.set_icc(&mut tmp_icc, Some(&output_encoding.color_management_system))
        });
        output_encoding.orig_color_encoding = orig_encoding.clone();
    }
    if let Some(color_encoding) = color_encoding {
        if image_metadata.color_encoding.is_gray()
            && color_encoding.color_space != ColorSpace::GRAY
            && dec.image_out_buffer_set
            && dec.image_out_format.num_channels < 3
        {
            return api_error!("Number of channels is too low for color output");
        }
        if color_encoding.color_space == ColorSpace::UNKNOWN {
            return api_error!("Unknown output colorspace");
        }
        if c_out.icc().is_empty() {
            return JxlDecoderStatus::ERROR;
        }
        api_return_if_status_error!(output_encoding.maybe_set_color_encoding(&c_out));
        image_metadata.color_encoding = output_encoding.color_encoding.clone();
        return JxlDecoderStatus::SUCCESS;
    }
    // icc_data != nullptr
    // TODO(firsching): implement setting output color profile from icc_data.
    let mut c_dst = ColorEncoding::default();
    // SAFETY: The caller supplies icc_size readable bytes at icc_data.
    let mut padded_icc = if icc_size == 0 {
        Vec::new()
    } else {
        unsafe { std::slice::from_raw_parts(icc_data, icc_size) }.to_vec()
    };
    // SAFETY: The caller's CMS contract covers this callback.
    if unsafe {
        c_dst.set_icc(
            &mut padded_icc,
            Some(&output_encoding.color_management_system),
        )
    }
    .is_err()
    {
        return api_error!("setting output color profile from icc_data not yet implemented.");
    }
    api_return_if_status_error!(output_encoding.maybe_set_color_encoding(&c_dst));

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_desired_intensity_target(
    dec: &mut JxlDecoder,
    desired_intensity_target: f32,
) -> JxlDecoderStatus {
    if desired_intensity_target < 0.0 {
        return api_error!("negative intensity target requested");
    }
    dec.desired_intensity_target = desired_intensity_target;
    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderSetCms.
///
/// # Safety
/// Every callback and opaque pointer of `cms` obeys cms_interface.h whenever
/// the decoder invokes it, until the decoder is reset or destroyed.
pub unsafe fn jxl_decoder_set_cms(dec: &mut JxlDecoder, cms: JxlCmsInterface) -> JxlDecoderStatus {
    let passes_state = dec
        .passes_state
        .get_or_insert_with(|| Box::new(PassesDecoderState::new()));
    passes_state.output_encoding_info.color_management_system = cms;
    passes_state.output_encoding_info.cms_set = true;
    JxlDecoderStatus::SUCCESS
}

/// JxlDecoderSetBoxBuffer.
///
/// # Safety
/// `data` is writable for `size` bytes until jxl_decoder_release_box_buffer,
/// and is disjoint from the decoder and its input.
pub unsafe fn jxl_decoder_set_box_buffer(
    dec: &mut JxlDecoder,
    data: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    if dec.box_out_buffer_set {
        return api_error!("must release box buffer before setting it again");
    }
    if !dec.box_event {
        return api_error!("can only set box buffer after box event");
    }

    dec.box_out_buffer_set = true;
    dec.box_out_buffer_set_current_box = true;
    dec.box_out_buffer = data;
    dec.box_out_buffer_size = size;
    dec.box_out_buffer_pos = 0;
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_release_box_buffer(dec: &mut JxlDecoder) -> usize {
    if !dec.box_out_buffer_set {
        return 0;
    }
    let result = dec.box_out_buffer_size.wrapping_sub(dec.box_out_buffer_pos);
    dec.box_out_buffer_set = false;
    dec.box_out_buffer = ptr::null_mut();
    dec.box_out_buffer_size = 0;
    if !dec.box_out_buffer_set_current_box {
        dec.box_out_buffer_begin = 0;
    } else {
        dec.box_out_buffer_begin = dec
            .box_out_buffer_begin
            .wrapping_add(dec.box_out_buffer_pos);
    }
    dec.box_out_buffer_set_current_box = false;
    result
}

pub fn jxl_decoder_set_decompress_boxes(dec: &mut JxlDecoder, decompress: i32) -> JxlDecoderStatus {
    // TODO(lode): return error if libbrotli is not compiled in the jxl
    // decoding library
    dec.decompress_boxes = decompress != 0;
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_box_type(
    dec: &JxlDecoder,
    box_type: &mut JxlBoxType,
    decompressed: i32,
) -> JxlDecoderStatus {
    if !dec.box_event {
        return api_error!("can only get box info after JXL_DEC_BOX event");
    }
    if decompressed != 0 {
        *box_type = dec.box_decoded_type;
    } else {
        *box_type = dec.box_type;
    }

    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_box_size_raw(dec: &JxlDecoder, size: Option<&mut u64>) -> JxlDecoderStatus {
    if !dec.box_event {
        return api_error!("can only get box info after JXL_DEC_BOX event");
    }
    if let Some(size) = size {
        *size = dec.box_size as u64;
    }
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_get_box_size_contents(
    dec: &JxlDecoder,
    size: Option<&mut u64>,
) -> JxlDecoderStatus {
    if !dec.box_event {
        return api_error!("can only get box info after JXL_DEC_BOX event");
    }
    if let Some(size) = size {
        *size = dec.box_contents_size as u64;
    }
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_progressive_detail(
    dec: &mut JxlDecoder,
    detail: JxlProgressiveDetail,
) -> JxlDecoderStatus {
    if detail != JxlProgressiveDetail::DC
        && detail != JxlProgressiveDetail::LAST_PASSES
        && detail != JxlProgressiveDetail::PASSES
    {
        return api_error!("Values other than kDC, kLastPasses and kPasses are not implemented.");
    }
    dec.prog_detail = detail;
    JxlDecoderStatus::SUCCESS
}

fn verify_output_bit_depth<T: BitDepthSource>(
    bit_depth: JxlBitDepth,
    metadata: &T,
    format: JxlPixelFormat,
) -> JxlDecoderStatus {
    let bits_per_sample = get_bit_depth(bit_depth, metadata, format);
    if bits_per_sample == 0 {
        return api_error!("Invalid output bit depth");
    }
    if format.data_type == JxlDataType::UINT8 && bits_per_sample > 8 {
        return api_error!("Invalid bit depth for uint8 output");
    } else if format.data_type == JxlDataType::UINT16 && bits_per_sample > 16 {
        return api_error!("Invalid bit depth for uint16 output");
    }
    JxlDecoderStatus::SUCCESS
}

pub fn jxl_decoder_set_image_out_bit_depth(
    dec: &mut JxlDecoder,
    bit_depth: &JxlBitDepth,
) -> JxlDecoderStatus {
    if !dec.image_out_buffer_set {
        return api_error!("No image out buffer was set.");
    }
    api_return_if_error!(verify_output_bit_depth(
        *bit_depth,
        &dec.metadata().m,
        dec.image_out_format
    ));
    dec.image_out_bit_depth = *bit_depth;
    JxlDecoderStatus::SUCCESS
}
