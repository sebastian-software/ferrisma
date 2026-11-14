// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The public encoder object and its lifecycle, option and input functions
//! from `lib/jxl/encode.cc`, the JxlEncoder and JxlEncoderFrameSettings
//! structs from `lib/jxl/encode_internal.h`, and the public values of
//! `lib/include/jxl/encode.h`.
//!
//! Translated: JxlEncoderVersion, VerifyLevelSettings, CheckValidBitdepth,
//! VerifyInputBitDepth, QueueFrame, QueueBox, JxlEncoderSetColorEncoding,
//! JxlEncoderSetICCProfile, JxlEncoderInitBasicInfo, JxlEncoderInitFrameHeader,
//! JxlEncoderInitBlendInfo, JxlEncoderSetBasicInfo,
//! JxlEncoderInitExtraChannelInfo, JxlEncoderSetUpsamplingMode,
//! JxlEncoderSetExtraChannelInfo, JxlEncoderSetExtraChannelName,
//! JxlEncoderFrameSettingsCreate, JxlEncoderSetFrameLossless,
//! JxlEncoderSetFrameDistance, JxlEncoderSetExtraChannelDistance,
//! JxlEncoderDistanceFromQuality, JxlEncoderFrameSettingsSetOption,
//! JxlEncoderFrameSettingsSetFloatOption, JxlEncoderCreate, JxlEncoderReset,
//! JxlEncoderDestroy, JxlEncoderGetError, JxlEncoderUseContainer,
//! JxlEncoderStoreJPEGMetadata, JxlEncoderSetCodestreamLevel,
//! JxlEncoderGetRequiredCodestreamLevel, JxlEncoderSetCms,
//! GetCurrentDimensions, CanDoFastLossless, JxlEncoderAddImageFrameInternal,
//! JxlEncoderAddImageFrame, JxlEncoderUseBoxes, JxlEncoderAddBox,
//! JxlEncoderSetExtraChannelBuffer, JxlEncoderCloseFrames,
//! JxlEncoderCloseBoxes, JxlEncoderCloseInput, JxlEncoderProcessOutput, the
//! raw-codestream branch of JxlEncoder::ProcessOneEnqueuedInput with
//! MustUseContainer, the alpha stripping decision with IsAlphaBufferOpaque,
//! and the frame index's AddFrame and StoreFrameIndexBox,
//! JxlEncoderSetFrameHeader, JxlEncoderSetExtraChannelBlendInfo,
//! JxlEncoderSetFrameName, JxlEncoderSetFrameBitDepth,
//! JxlColorEncodingSetToSRGB, JxlColorEncodingSetToLinearSRGB and
//! JxlEncoderAllowExpertOptions.
//!
//! Admission (ADR 0001): the encoder runs before the frame encoder below it is
//! complete. A call that would enter an untranslated branch returns
//! JXL_ENC_ERROR with JXL_ENC_ERR_NOT_SUPPORTED at that source checkpoint,
//! after every check the original performs before it. The checkpoints are
//! [`JxlEncoderAdmission`]: the fast-lossless branch of AddImageFrameInternal
//! (enc_fast_lossless.cc); ProcessOneEnqueuedInput's ICC, container and box
//! branches; and the frame encoder's checkpoints (streaming, the admitted
//! frame profile and patches), see
//! `enc_frame_vardct.rs`.
//! Untranslated public functions are absent: AddJPEGFrame, AddChunkedFrame,
//! FlushInput, SetOutputProcessor, SetParallelRunner, SetDebugImageCallback
//! and the statistics functions. The C ABI is not exported (ADR 0001).
//!
//! Language-required deviations, each local to its representation:
//! - The encoder object is allocated through the caller's memory manager and
//!   constructed in place, as in JxlEncoderCreate; its size is the Rust
//!   layout's. The memory manager lives in a stable heap cell beside the
//!   object because frame settings and queued owners retain its address.
//!   Frame settings, queued frames and queued boxes are MemoryManagerUniquePtr
//!   owners allocated through that manager, as in the source.
//! - Destruction and Reset release the queued inputs and then the frame
//!   settings, each vector from its last element, as the source's reverse
//!   member order and libc++'s vector clear do.
//! - Owners that only untranslated functions install (thread pool,
//!   fast-lossless frame, external output processor, statistics) are
//!   uninhabited `Option<Infallible>` fields at their source positions.
//! - JXL_API_ERROR and JXL_API_ERROR_NOSET return JXL_ENC_ERROR; the release
//!   reference's debug messages are not translated, and VerifyLevelSettings
//!   therefore takes no debug string.
//! - Frame-settings functions take the raw settings pointer that
//!   JxlEncoderFrameSettingsCreate returned and reach the encoder through its
//!   retained `enc` pointer; their safety contracts state the lifetime rules.
//!   Caller pixel and box buffers are slices, or options of slices where the
//!   source distinguishes a null pointer; output pointers of ProcessOutput
//!   stay raw because the output wrapper retains them.

use super::base::byte_order::swap_endianness;
use super::base::status::StatusCode;
use super::cms::jxl_cms::get_default_cms;
use super::cms_interface::JxlCmsInterface;
use super::codestream_header::{
    JxlBasicInfo, JxlBlendInfo, JxlBlendMode, JxlExtraChannelInfo, JxlExtraChannelType,
    JxlFrameHeader, JxlOrientation,
};
use super::color_encoding::{ColorSpace, JxlColorEncoding};
use super::color_encoding_internal::ColorEncoding;
use super::common::SpeedTier;
use super::decode::{Aliased, JPEGXL_MAJOR_VERSION, JPEGXL_MINOR_VERSION, JPEGXL_PATCH_VERSION};
use super::enc_aux_out::LayerType;
use super::enc_bit_writer::BitWriter;
use super::enc_fields::write_codestream_headers;
use super::enc_frame::FrameInfo;
use super::enc_frame_vardct::{EncodeFrameError, FrameAdmission, encode_frame};
use super::enc_params::{CompressParams, MIN_BUTTERAUGLI_DISTANCE};
use super::encode_internal::JxlEncoderFrameIndexBoxEntry;
use super::encode_internal::{
    JxlEncoderChunkedFrameAdapter, JxlEncoderFrameIndexBox, JxlEncoderFrameSettingsValues,
    JxlEncoderOutputProcessorWrapper, JxlEncoderQueuedBox, JxlEncoderQueuedFrame,
    JxlEncoderQueuedInput, append_data, make_box_type,
};
use super::frame_header::{BlendMode, BlendingInfo, ColorTransform};
use super::image_metadata::{CodecMetadata, ExtraChannel, ExtraChannelInfo};
use super::luminance::set_intensity_target;
use super::memory_manager::{
    JxlMemoryManager, default_alloc, default_free, memory_manager_alloc, memory_manager_free,
    memory_manager_init,
};
use super::memory_manager_internal::MemoryManagerUniquePtr;
use super::modular::options::Predictor;
use super::types::{JxlBitDepth, JxlBitDepthType, JxlDataType, JxlPixelFormat};
use std::convert::Infallible;
use std::ffi::CStr;
use std::ptr;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlEncoderStatus(pub u32);
impl JxlEncoderStatus {
    pub const SUCCESS: Self = Self(0);
    pub const ERROR: Self = Self(1);
    pub const NEED_MORE_OUTPUT: Self = Self(2);
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlEncoderError(pub u32);
impl JxlEncoderError {
    pub const OK: Self = Self(0);
    pub const GENERIC: Self = Self(1);
    pub const OOM: Self = Self(2);
    pub const JBRD: Self = Self(3);
    pub const BAD_INPUT: Self = Self(4);
    pub const NOT_SUPPORTED: Self = Self(0x80);
    pub const API_USAGE: Self = Self(0x81);
}

/// JxlEncoderFrameSettingId, a transparent carrier for any stored value.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(transparent)]
pub struct JxlEncoderFrameSettingId(pub u32);
impl JxlEncoderFrameSettingId {
    pub const EFFORT: Self = Self(0);
    pub const DECODING_SPEED: Self = Self(1);
    pub const RESAMPLING: Self = Self(2);
    pub const EXTRA_CHANNEL_RESAMPLING: Self = Self(3);
    pub const ALREADY_DOWNSAMPLED: Self = Self(4);
    pub const PHOTON_NOISE: Self = Self(5);
    pub const NOISE: Self = Self(6);
    pub const DOTS: Self = Self(7);
    pub const PATCHES: Self = Self(8);
    pub const EPF: Self = Self(9);
    pub const GABORISH: Self = Self(10);
    pub const MODULAR: Self = Self(11);
    pub const KEEP_INVISIBLE: Self = Self(12);
    pub const GROUP_ORDER: Self = Self(13);
    pub const GROUP_ORDER_CENTER_X: Self = Self(14);
    pub const GROUP_ORDER_CENTER_Y: Self = Self(15);
    pub const RESPONSIVE: Self = Self(16);
    pub const PROGRESSIVE_AC: Self = Self(17);
    pub const QPROGRESSIVE_AC: Self = Self(18);
    pub const PROGRESSIVE_DC: Self = Self(19);
    pub const CHANNEL_COLORS_GLOBAL_PERCENT: Self = Self(20);
    pub const CHANNEL_COLORS_GROUP_PERCENT: Self = Self(21);
    pub const PALETTE_COLORS: Self = Self(22);
    pub const LOSSY_PALETTE: Self = Self(23);
    pub const COLOR_TRANSFORM: Self = Self(24);
    pub const MODULAR_COLOR_SPACE: Self = Self(25);
    pub const MODULAR_GROUP_SIZE: Self = Self(26);
    pub const MODULAR_PREDICTOR: Self = Self(27);
    pub const MODULAR_MA_TREE_LEARNING_PERCENT: Self = Self(28);
    pub const MODULAR_NB_PREV_CHANNELS: Self = Self(29);
    pub const JPEG_RECON_CFL: Self = Self(30);
    pub const INDEX_BOX: Self = Self(31);
    pub const BROTLI_EFFORT: Self = Self(32);
    pub const JPEG_COMPRESS_BOXES: Self = Self(33);
    pub const BUFFERING: Self = Self(34);
    pub const JPEG_KEEP_EXIF: Self = Self(35);
    pub const JPEG_KEEP_XMP: Self = Self(36);
    pub const JPEG_KEEP_JUMBF: Self = Self(37);
    pub const USE_FULL_IMAGE_HEURISTICS: Self = Self(38);
    pub const DISABLE_PERCEPTUAL_HEURISTICS: Self = Self(39);
    pub const OUTPUT_MODE: Self = Self(40);
    pub const STRIP_ALPHA: Self = Self(41);
    pub const JPEG_RECON_LFS: Self = Self(42);
    pub const FILL_ENUM: Self = Self(65535);
}

/// The source checkpoints at which the encoder returns an admission result.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum JxlEncoderAdmission {
    /// AddImageFrameInternal when CanDoFastLossless selects enc_fast_lossless.
    FastLosslessFrame,
    /// ProcessOneEnqueuedInput at a branch outside raw codestream output:
    /// an ICC profile, a container (including the level 10 that alpha or
    /// extra channels deeper than 12 bits require) or a box.
    EnqueuedInput,
    /// EncodeFrame when CanDoStreamingEncoding selects the streaming encode.
    StreamingFrame,
    /// EncodeFrame outside the admitted one-shot VarDCT profile.
    FrameProfile,
    /// FindBestPatchDictionary when FindTextLikePatches finds patches.
    PatchDictionary,
}

#[cfg(feature = "cpp-oracle")]
thread_local! {
    static ADMISSIONS: std::cell::RefCell<Vec<JxlEncoderAdmission>> =
        const { std::cell::RefCell::new(Vec::new()) };
}

/// Test observation for the differential oracle: the admission results this
/// thread returned since the last call, in order. Observation only; it
/// changes no encoder behavior.
#[cfg(feature = "cpp-oracle")]
pub fn take_admissions() -> Vec<JxlEncoderAdmission> {
    ADMISSIONS.with(|admissions| std::mem::take(&mut *admissions.borrow_mut()))
}

const STRIP_ALPHA_FORCE: i32 = 1;

pub fn jxl_encoder_version() -> u32 {
    JPEGXL_MAJOR_VERSION * 1_000_000 + JPEGXL_MINOR_VERSION * 1000 + JPEGXL_PATCH_VERSION
}

/// The original JxlEncoder object, with every field in declaration order.
pub struct JxlEncoder {
    memory_manager: Aliased<JxlMemoryManager>,
    thread_pool: Option<Infallible>,
    encoder_options: Vec<MemoryManagerUniquePtr<JxlEncoderFrameSettings>>,
    num_queued_frames: usize,
    num_queued_boxes: usize,
    input_queue: Vec<JxlEncoderQueuedInput>,
    output_processor: JxlEncoderOutputProcessorWrapper,
    codestream_bytes_written_end_of_frame: usize,
    frame_index_box: JxlEncoderFrameIndexBox,
    cms: JxlCmsInterface,
    cms_set: bool,
    use_container: bool,
    use_boxes: bool,
    container_ftyp_version: i32,
    store_jpeg_metadata: bool,
    codestream_level: i32,
    metadata: CodecMetadata,
    jpeg_metadata: Vec<u8>,
    last_used_cparams: CompressParams<'static>,
    basic_info: JxlBasicInfo,
    error: JxlEncoderError,
    jxlp_counter: usize,
    wrote_bytes: bool,
    frames_closed: bool,
    boxes_closed: bool,
    basic_info_set: bool,
    color_encoding_set: bool,
    intensity_target_set: bool,
    allow_expert_options: bool,
    brotli_effort: i32,
}

/// The original JxlEncoderFrameSettings: its encoder and option values.
pub struct JxlEncoderFrameSettings {
    enc: *mut JxlEncoder,
    values: JxlEncoderFrameSettingsValues,
}
impl JxlEncoderFrameSettings {
    /// Read-only observations of the stored values and encoder pointer.
    pub fn values(&self) -> &JxlEncoderFrameSettingsValues {
        &self.values
    }
    pub fn encoder(&self) -> *const JxlEncoder {
        self.enc
    }
}

/// Read-only copies of the encoder's scalar fields.
#[derive(Clone, Copy, Debug)]
pub struct JxlEncoderScalars {
    pub thread_pool_set: bool,
    pub num_queued_frames: usize,
    pub num_queued_boxes: usize,
    pub codestream_bytes_written_end_of_frame: usize,
    pub cms_set: bool,
    pub use_container: bool,
    pub use_boxes: bool,
    pub container_ftyp_version: i32,
    pub store_jpeg_metadata: bool,
    pub codestream_level: i32,
    pub error: JxlEncoderError,
    pub jxlp_counter: usize,
    pub wrote_bytes: bool,
    pub frames_closed: bool,
    pub boxes_closed: bool,
    pub basic_info_set: bool,
    pub color_encoding_set: bool,
    pub intensity_target_set: bool,
    pub allow_expert_options: bool,
    pub brotli_effort: i32,
}

impl JxlEncoder {
    /// The constructor with the default member initializers and the output
    /// wrapper bound to the manager cell. The source's user-provided
    /// constructor default-initializes the remaining members; they receive
    /// safe zero values with no meaning before Create and Reset assign them.
    /// The basic info's `padding`, which InitBasicInfo leaves unset, stays
    /// indeterminate in the source until SetBasicInfo copies a caller's
    /// struct; here it is zero.
    fn construct(memory_manager_copy: JxlMemoryManager) -> Self {
        let memory_manager = Aliased::new(memory_manager_copy);
        // SAFETY: The manager cell outlives the wrapper, which the encoder
        // owns and drops before the cell.
        let output_processor =
            unsafe { JxlEncoderOutputProcessorWrapper::new(memory_manager.as_ptr()) };
        Self {
            memory_manager,
            thread_pool: None,
            encoder_options: Vec::new(),
            num_queued_frames: 0,
            num_queued_boxes: 0,
            input_queue: Vec::new(),
            output_processor,
            codestream_bytes_written_end_of_frame: 0,
            frame_index_box: JxlEncoderFrameIndexBox::default(),
            cms: JxlCmsInterface::default(),
            cms_set: false,
            use_container: false,
            use_boxes: false,
            container_ftyp_version: -1,
            store_jpeg_metadata: false,
            codestream_level: 0,
            metadata: CodecMetadata::default(),
            jpeg_metadata: Vec::new(),
            last_used_cparams: CompressParams::default(),
            basic_info: JxlBasicInfo::zeroed(),
            error: JxlEncoderError::OK,
            jxlp_counter: 0,
            wrote_bytes: false,
            frames_closed: false,
            boxes_closed: false,
            basic_info_set: false,
            color_encoding_set: false,
            intensity_target_set: false,
            allow_expert_options: false,
            brotli_effort: -1,
        }
    }

    fn manager(&self) -> *const JxlMemoryManager {
        self.memory_manager.as_ptr()
    }

    /// Read-only observations of the stored state.
    pub fn metadata(&self) -> &CodecMetadata {
        &self.metadata
    }
    pub fn basic_info(&self) -> &JxlBasicInfo {
        &self.basic_info
    }
    pub fn frame_index_box(&self) -> &JxlEncoderFrameIndexBox {
        &self.frame_index_box
    }
    pub fn jpeg_metadata(&self) -> &[u8] {
        &self.jpeg_metadata
    }
    pub fn output_processor(&self) -> &JxlEncoderOutputProcessorWrapper {
        &self.output_processor
    }
    pub fn input_queue(&self) -> &[JxlEncoderQueuedInput] {
        &self.input_queue
    }
    pub fn frame_settings(&self) -> impl Iterator<Item = &JxlEncoderFrameSettings> {
        self.encoder_options.iter().map(|settings| &**settings)
    }
    pub fn scalars(&self) -> JxlEncoderScalars {
        JxlEncoderScalars {
            thread_pool_set: self.thread_pool.is_some(),
            num_queued_frames: self.num_queued_frames,
            num_queued_boxes: self.num_queued_boxes,
            codestream_bytes_written_end_of_frame: self.codestream_bytes_written_end_of_frame,
            cms_set: self.cms_set,
            use_container: self.use_container,
            use_boxes: self.use_boxes,
            container_ftyp_version: self.container_ftyp_version,
            store_jpeg_metadata: self.store_jpeg_metadata,
            codestream_level: self.codestream_level,
            error: self.error,
            jxlp_counter: self.jxlp_counter,
            wrote_bytes: self.wrote_bytes,
            frames_closed: self.frames_closed,
            boxes_closed: self.boxes_closed,
            basic_info_set: self.basic_info_set,
            color_encoding_set: self.color_encoding_set,
            intensity_target_set: self.intensity_target_set,
            allow_expert_options: self.allow_expert_options,
            brotli_effort: self.brotli_effort,
        }
    }
}

/// Destroys a vector's elements from the last, as libc++'s clear does.
fn destroy_from_back<T>(values: &mut Vec<T>) {
    while let Some(value) = values.pop() {
        drop(value);
    }
}

impl Drop for JxlEncoder {
    /// The implicit destructor: members in reverse declaration order. Only
    /// the queued inputs, the output wrapper's buffers and the frame settings
    /// own memory-manager allocations; the remaining members follow.
    fn drop(&mut self) {
        destroy_from_back(&mut self.input_queue);
        // SAFETY: The replacement owns no buffer and is dropped with the
        // encoder before the manager cell.
        drop(std::mem::replace(&mut self.output_processor, unsafe {
            JxlEncoderOutputProcessorWrapper::new(self.memory_manager.as_ptr())
        }));
        destroy_from_back(&mut self.encoder_options);
    }
}

fn api_error(enc: &mut JxlEncoder, error_code: JxlEncoderError) -> JxlEncoderStatus {
    enc.error = error_code;
    JxlEncoderStatus::ERROR
}

/// An admission result at the given checkpoint; see the module header.
fn admission(enc: &mut JxlEncoder, checkpoint: JxlEncoderAdmission) -> JxlEncoderStatus {
    #[cfg(feature = "cpp-oracle")]
    ADMISSIONS.with(|admissions| admissions.borrow_mut().push(checkpoint));
    #[cfg(not(feature = "cpp-oracle"))]
    let _ = checkpoint;
    api_error(enc, JxlEncoderError::NOT_SUPPORTED)
}

fn queue_frame(
    frame_settings: &JxlEncoderFrameSettings,
    enc: &mut JxlEncoder,
    mut frame: MemoryManagerUniquePtr<JxlEncoderQueuedFrame>,
) {
    if frame_settings.values.lossless {
        frame.option_values.cparams.set_lossless();
    }
    let mut queued_input = JxlEncoderQueuedInput::new();
    let mut om = frame.option_values.cparams.output_mode;
    if om < 0 {
        om = if frame.option_values.cparams.buffering == 3 {
            1
        } else {
            0
        };
    }
    queued_input.output_mode = om;
    queued_input.frame = Some(frame);
    enc.input_queue.push(queued_input);
    enc.num_queued_frames += 1;
}

fn queue_box(enc: &mut JxlEncoder, queued_box: MemoryManagerUniquePtr<JxlEncoderQueuedBox>) {
    let mut queued_input = JxlEncoderQueuedInput::new();
    queued_input.queued_box = Some(queued_box);
    enc.input_queue.push(queued_input);
    enc.num_queued_boxes += 1;
}

// The JXL codestream can have level 5 or level 10. Levels have certain
// restrictions such as max allowed image dimensions. This function checks the
// level required to support the current encoder settings.
fn verify_level_settings(enc: &JxlEncoder) -> i32 {
    let m = &enc.metadata.m;
    let xsize = enc.metadata.xsize() as u64;
    let ysize = enc.metadata.ysize() as u64;
    // The uncompressed ICC size, if it is used.
    let mut icc_size = 0usize;
    if m.color_encoding.want_icc() {
        icc_size = m.color_encoding.icc().len();
    }
    // Level 10 checks
    if xsize > (1u64 << 30) || ysize > (1u64 << 30) || xsize.wrapping_mul(ysize) > (1u64 << 40) {
        return -1;
    }
    if icc_size as u64 > (1u64 << 28) {
        return -1;
    }
    if m.num_extra_channels > 256 {
        return -1;
    }
    // Level 5 checks
    if !m.modular_16_bit_buffer_sufficient {
        return 10;
    }
    if xsize > (1u64 << 18) || ysize > (1u64 << 18) || xsize.wrapping_mul(ysize) > (1u64 << 28) {
        return 10;
    }
    if icc_size as u64 > (1u64 << 22) {
        return 10;
    }
    if m.num_extra_channels > 4 {
        return 10;
    }
    for eci in &m.extra_channel_info {
        if eci.channel_type == ExtraChannel::BLACK {
            return 10;
        }
    }
    // All level 5 checks passes, so can return the more compatible level 5
    5
}

fn check_valid_bitdepth(bits_per_sample: u32, exponent_bits_per_sample: u32) -> JxlEncoderStatus {
    if exponent_bits_per_sample == 0 {
        // The spec allows up to 31 for bits_per_sample here, but the code does
        // not (yet) support it.
        if !(bits_per_sample > 0 && bits_per_sample <= 24) {
            return JxlEncoderStatus::ERROR;
        }
    } else if exponent_bits_per_sample > 8
        || bits_per_sample > 24u32.wrapping_add(exponent_bits_per_sample)
        || bits_per_sample < 3u32.wrapping_add(exponent_bits_per_sample)
    {
        return JxlEncoderStatus::ERROR;
    }
    JxlEncoderStatus::SUCCESS
}

fn verify_input_bit_depth(_bit_depth: JxlBitDepth, _format: JxlPixelFormat) -> JxlEncoderStatus {
    JxlEncoderStatus::SUCCESS
}

/// The shared level check at the end of SetBasicInfo and
/// SetExtraChannelInfo.
fn level_verification(enc: &mut JxlEncoder) -> JxlEncoderStatus {
    let required_level = verify_level_settings(enc);
    if required_level == -1 || (enc.codestream_level < required_level && enc.codestream_level != -1)
    {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_encoder_set_color_encoding(
    enc: &mut JxlEncoder,
    color: &JxlColorEncoding,
) -> JxlEncoderStatus {
    if !enc.basic_info_set {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if enc.color_encoding_set {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if enc.metadata.m.color_encoding.from_external(color).is_err() {
        return api_error(enc, JxlEncoderError::GENERIC);
    }
    if enc.metadata.m.color_encoding.get_color_space() == ColorSpace::GRAY {
        if enc.basic_info.num_color_channels != 1 {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
    } else if enc.basic_info.num_color_channels != 3 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    enc.color_encoding_set = true;
    if !enc.intensity_target_set {
        set_intensity_target(&mut enc.metadata.m);
    }
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetICCProfile with the profile as a slice; an empty slice is the
/// source's zero size.
pub fn jxl_encoder_set_icc_profile(enc: &mut JxlEncoder, icc_profile: &[u8]) -> JxlEncoderStatus {
    if !enc.basic_info_set {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if enc.color_encoding_set {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if icc_profile.is_empty() {
        return api_error(enc, JxlEncoderError::BAD_INPUT);
    }
    let mut icc = icc_profile.to_vec();
    if enc.cms_set {
        // SAFETY: The encoder's CMS is the default CMS or a caller CMS that
        // jxl_encoder_set_cms admitted under its contract.
        if unsafe {
            enc.metadata
                .m
                .color_encoding
                .set_icc(&mut icc, Some(&enc.cms))
        }
        .is_err()
        {
            return api_error(enc, JxlEncoderError::BAD_INPUT);
        }
    } else {
        enc.metadata.m.color_encoding.set_icc_raw(icc);
    }
    if enc.metadata.m.color_encoding.get_color_space() == ColorSpace::GRAY {
        if enc.basic_info.num_color_channels != 1 {
            return api_error(enc, JxlEncoderError::BAD_INPUT);
        }
    } else if enc.basic_info.num_color_channels != 3 {
        return api_error(enc, JxlEncoderError::BAD_INPUT);
    }
    enc.color_encoding_set = true;
    if !enc.intensity_target_set {
        set_intensity_target(&mut enc.metadata.m);
    }
    if enc.basic_info.uses_original_profile == 0 && enc.cms_set {
        // SAFETY: As above for the encoder's CMS.
        unsafe { enc.metadata.m.color_encoding.decide_if_want_icc(&enc.cms) };
    }
    JxlEncoderStatus::SUCCESS
}

/// Leaves `padding` unchanged, as the source does.
pub fn jxl_encoder_init_basic_info(info: &mut JxlBasicInfo) {
    info.have_container = 0;
    info.xsize = 0;
    info.ysize = 0;
    info.bits_per_sample = 8;
    info.exponent_bits_per_sample = 0;
    info.intensity_target = 0.0;
    info.min_nits = 0.0;
    info.relative_to_max_display = 0;
    info.linear_below = 0.0;
    info.uses_original_profile = 0;
    info.have_preview = 0;
    info.have_animation = 0;
    info.orientation = JxlOrientation::IDENTITY;
    info.num_color_channels = 3;
    info.num_extra_channels = 0;
    info.alpha_bits = 0;
    info.alpha_exponent_bits = 0;
    info.alpha_premultiplied = 0;
    info.preview.xsize = 0;
    info.preview.ysize = 0;
    info.intrinsic_xsize = 0;
    info.intrinsic_ysize = 0;
    info.animation.tps_numerator = 10;
    info.animation.tps_denominator = 1;
    info.animation.num_loops = 0;
    info.animation.have_timecodes = 0;
}

pub fn jxl_encoder_init_frame_header(frame_header: &mut JxlFrameHeader) {
    frame_header.duration = 0;
    frame_header.timecode = 0;
    frame_header.name_length = 0;
    frame_header.is_last = 1;
    frame_header.layer_info.have_crop = 0;
    frame_header.layer_info.crop_x0 = 0;
    frame_header.layer_info.crop_y0 = 0;
    frame_header.layer_info.xsize = 0;
    frame_header.layer_info.ysize = 0;
    jxl_encoder_init_blend_info(&mut frame_header.layer_info.blend_info);
    frame_header.layer_info.save_as_reference = 0;
}

pub fn jxl_encoder_init_blend_info(blend_info: &mut JxlBlendInfo) {
    blend_info.blendmode = JxlBlendMode::REPLACE;
    blend_info.source = 0;
    blend_info.alpha = 0;
    blend_info.clamp = 0;
}

pub fn jxl_encoder_set_basic_info(enc: &mut JxlEncoder, info: &JxlBasicInfo) -> JxlEncoderStatus {
    if enc
        .metadata
        .size
        .set(info.xsize as usize, info.ysize as usize)
        .is_err()
    {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if JxlEncoderStatus::SUCCESS
        != check_valid_bitdepth(info.bits_per_sample, info.exponent_bits_per_sample)
    {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    let m = &mut enc.metadata.m;
    m.bit_depth.bits_per_sample = info.bits_per_sample;
    m.bit_depth.exponent_bits_per_sample = info.exponent_bits_per_sample;
    m.bit_depth.floating_point_sample = info.exponent_bits_per_sample != 0;
    m.modular_16_bit_buffer_sufficient =
        (info.uses_original_profile == 0 || info.bits_per_sample <= 12) && info.alpha_bits <= 12;
    if (info.intrinsic_xsize > 0 || info.intrinsic_ysize > 0)
        && (info.intrinsic_xsize != info.xsize || info.intrinsic_ysize != info.ysize)
    {
        if info.intrinsic_xsize > (1 << 30)
            || info.intrinsic_ysize > (1 << 30)
            || m.intrinsic_size
                .set(info.intrinsic_xsize as usize, info.intrinsic_ysize as usize)
                .is_err()
        {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
        m.have_intrinsic_size = true;
    }
    // The number of extra channels includes the alpha channel, so for example
    // and RGBA with no other extra channels, has exactly num_extra_channels == 1
    m.num_extra_channels = info.num_extra_channels;
    m.extra_channel_info
        .resize_with(m.num_extra_channels as usize, ExtraChannelInfo::default);
    if info.num_extra_channels == 0 && info.alpha_bits != 0 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    // If the user provides non-zero alpha_bits, we make the channel info at
    // index zero the appropriate alpha channel.
    if info.alpha_bits != 0 {
        let mut channel_info = JxlExtraChannelInfo::default();
        jxl_encoder_init_extra_channel_info(JxlExtraChannelType::ALPHA, &mut channel_info);
        channel_info.bits_per_sample = info.alpha_bits;
        channel_info.exponent_bits_per_sample = info.alpha_exponent_bits;
        channel_info.alpha_premultiplied = info.alpha_premultiplied;
        if jxl_encoder_set_extra_channel_info(enc, 0, &channel_info) != JxlEncoderStatus::SUCCESS {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
    }
    let m = &mut enc.metadata.m;
    m.xyb_encoded = info.uses_original_profile == 0;
    if info.orientation.0 > 0 && info.orientation.0 <= 8 {
        m.orientation = info.orientation.0;
    } else {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if info.num_color_channels != 1 && info.num_color_channels != 3 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if info.intensity_target != 0.0 {
        enc.metadata.m.set_intensity_target(info.intensity_target);
        enc.intensity_target_set = true;
    } else if enc.color_encoding_set {
        // If this is false, JxlEncoderSetColorEncoding will be called later
        // and we will get one more chance to call jxl::SetIntensityTarget,
        // after the color encoding is indeed set.
        set_intensity_target(&mut enc.metadata.m);
        enc.intensity_target_set = true;
    }
    let m = &mut enc.metadata.m;
    m.tone_mapping.min_nits = info.min_nits;
    m.tone_mapping.relative_to_max_display = info.relative_to_max_display != 0;
    m.tone_mapping.linear_below = info.linear_below;
    enc.basic_info = *info;
    enc.basic_info_set = true;

    enc.metadata.m.have_animation = info.have_animation != 0;
    if info.have_animation != 0 {
        if info.animation.tps_denominator < 1 {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
        if info.animation.tps_numerator < 1 {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
        let animation = &mut enc.metadata.m.animation;
        animation.tps_numerator = info.animation.tps_numerator;
        animation.tps_denominator = info.animation.tps_denominator;
        animation.num_loops = info.animation.num_loops;
        animation.have_timecodes = info.animation.have_timecodes != 0;
    }
    level_verification(enc)
}

pub fn jxl_encoder_init_extra_channel_info(
    channel_type: JxlExtraChannelType,
    info: &mut JxlExtraChannelInfo,
) {
    info.channel_type = channel_type;
    info.bits_per_sample = 8;
    info.exponent_bits_per_sample = 0;
    info.dim_shift = 0;
    info.name_length = 0;
    info.alpha_premultiplied = 0;
    info.spot_color = [0.0; 4];
    info.cfa_channel = 0;
}

pub fn jxl_encoder_set_upsampling_mode(
    enc: &mut JxlEncoder,
    factor: i64,
    mode: i64,
) -> JxlEncoderStatus {
    // for convenience, allow calling this with factor 1 and just make it a
    // no-op
    if factor == 1 {
        return JxlEncoderStatus::SUCCESS;
    }
    if factor != 2 && factor != 4 && factor != 8 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if mode < -1 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if mode > 1 {
        return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
    }
    let count = if factor == 2 {
        15
    } else if factor == 4 {
        55
    } else {
        210
    };
    if mode == -1 {
        // Default fancy upsampling: don't signal custom weights
        enc.metadata.transform_data.custom_weights_mask &= !((factor >> 1) as u32);
    } else if mode == 0 {
        // Nearest neighbor upsampling
        let td = &mut enc.metadata.transform_data;
        td.custom_weights_mask |= (factor >> 1) as u32;
        let weights: &mut [f32] = match factor {
            2 => &mut td.upsampling2_weights,
            4 => &mut td.upsampling4_weights,
            _ => &mut td.upsampling8_weights,
        };
        weights[..count].fill(0.0);
        if factor == 2 {
            weights[9] = 1.0;
        } else if factor == 4 {
            for i in [19, 24, 49] {
                weights[i] = 1.0;
            }
        } else if factor == 8 {
            for i in [39, 44, 49, 54, 119, 124, 129, 174, 179, 204] {
                weights[i] = 1.0;
            }
        }
    } else if mode == 1 {
        // 'Pixel dots' upsampling (nearest-neighbor with cut corners)
        jxl_encoder_set_upsampling_mode(enc, factor, 0);
        let td = &mut enc.metadata.transform_data;
        if factor == 4 {
            td.upsampling4_weights[19] = 0.0;
            td.upsampling4_weights[24] = 0.5;
        } else if factor == 8 {
            for i in [39, 44, 49, 119] {
                td.upsampling8_weights[i] = 0.0;
            }
            for i in [54, 124] {
                td.upsampling8_weights[i] = 0.5;
            }
        }
    }
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_encoder_set_extra_channel_info(
    enc: &mut JxlEncoder,
    index: usize,
    info: &JxlExtraChannelInfo,
) -> JxlEncoderStatus {
    if index >= enc.metadata.m.num_extra_channels as usize {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if JxlEncoderStatus::SUCCESS
        != check_valid_bitdepth(info.bits_per_sample, info.exponent_bits_per_sample)
    {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    let m = &mut enc.metadata.m;
    let channel = &mut m.extra_channel_info[index];
    channel.channel_type = ExtraChannel(info.channel_type.0);
    channel.bit_depth.bits_per_sample = info.bits_per_sample;
    m.modular_16_bit_buffer_sufficient &= info.bits_per_sample <= 12;
    let channel = &mut m.extra_channel_info[index];
    channel.bit_depth.exponent_bits_per_sample = info.exponent_bits_per_sample;
    channel.bit_depth.floating_point_sample = info.exponent_bits_per_sample != 0;
    channel.dim_shift = info.dim_shift;
    channel.name.clear();
    channel.alpha_associated = info.alpha_premultiplied != 0;
    channel.cfa_channel = info.cfa_channel;
    channel.spot_color = info.spot_color;
    level_verification(enc)
}

/// JxlEncoderSetExtraChannelName with the name bytes as a slice.
pub fn jxl_encoder_set_extra_channel_name(
    enc: &mut JxlEncoder,
    index: usize,
    name: &[u8],
) -> JxlEncoderStatus {
    if index >= enc.metadata.m.num_extra_channels as usize {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    enc.metadata.m.extra_channel_info[index].name = name.to_vec();
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderFrameSettingsCreate. Returns null when the allocation fails.
///
/// # Safety
/// `enc` is a live encoder from [`jxl_encoder_create`] with no Rust reference
/// to it live during the call; the returned settings retain this pointer. A
/// nonnull `source` is a live settings object of a live encoder. The returned
/// object is owned by the encoder and released by its Reset or Destroy.
pub unsafe fn jxl_encoder_frame_settings_create(
    enc: *mut JxlEncoder,
    source: *const JxlEncoderFrameSettings,
) -> *mut JxlEncoderFrameSettings {
    // SAFETY: The caller supplies the live encoder and source; the settings
    // allocation is distinct from both, and the encoder's manager cell
    // outlives every settings object it owns.
    unsafe {
        let Some(mut opts) = MemoryManagerUniquePtr::make(
            (*enc).manager(),
            JxlEncoderFrameSettings {
                enc: ptr::null_mut(),
                values: JxlEncoderFrameSettingsValues::default(),
            },
        ) else {
            return ptr::null_mut();
        };
        opts.enc = enc;
        if !source.is_null() {
            opts.values = (*source).values.clone();
        } else {
            opts.values.lossless = false;
        }
        opts.values.cparams.level = (*enc).codestream_level;
        let num_extra_channels = (*enc).metadata.m.num_extra_channels as usize;
        opts.values
            .cparams
            .ec_distance
            .resize(num_extra_channels, 0.0);
        let ret = opts.as_ptr();
        (*enc).encoder_options.push(opts);
        ret
    }
}

/// The settings object and its encoder, as the source reaches them.
///
/// # Safety
/// `frame_settings` is a live settings object from
/// [`jxl_encoder_frame_settings_create`] whose encoder is live; no other Rust
/// reference to either is live for `'a`.
unsafe fn settings_and_encoder<'a>(
    frame_settings: *mut JxlEncoderFrameSettings,
) -> (&'a mut JxlEncoderFrameSettings, &'a mut JxlEncoder) {
    // SAFETY: The caller supplies both live objects. The settings object is a
    // separate memory-manager allocation, so the two borrows do not overlap.
    unsafe {
        let settings = &mut *frame_settings;
        let enc = &mut *settings.enc;
        (settings, enc)
    }
}

/// JxlEncoderSetFrameLossless.
///
/// # Safety
/// As for every frame-settings function: `frame_settings` is a live settings
/// object from [`jxl_encoder_frame_settings_create`] whose encoder is live,
/// and no Rust reference to either is live during the call.
pub unsafe fn jxl_encoder_set_frame_lossless(
    frame_settings: *mut JxlEncoderFrameSettings,
    lossless: i32,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    if lossless != 0 && enc.basic_info_set && enc.metadata.m.xyb_encoded {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    settings.values.lossless = lossless != 0;
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetFrameDistance.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_set_frame_distance(
    frame_settings: *mut JxlEncoderFrameSettings,
    mut distance: f32,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    if distance < 0.0 || distance > 25.0 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if distance > 0.0 && distance < MIN_BUTTERAUGLI_DISTANCE {
        distance = MIN_BUTTERAUGLI_DISTANCE;
    }
    settings.values.cparams.butteraugli_distance = distance;
    if distance == 0.0 {
        // SAFETY: The same settings object; the borrows above end here.
        return unsafe { jxl_encoder_set_frame_lossless(frame_settings, 1) };
    }
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetExtraChannelDistance.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_set_extra_channel_distance(
    frame_settings: *mut JxlEncoderFrameSettings,
    index: usize,
    mut distance: f32,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    let num_extra_channels = enc.metadata.m.num_extra_channels as usize;
    if index >= num_extra_channels {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if distance != -1.0 && (distance < 0.0 || distance > 25.0) {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if distance > 0.0 && distance < 0.01 {
        distance = 0.01;
    }
    if index >= settings.values.cparams.ec_distance.len() {
        // This can only happen if JxlEncoderFrameSettingsCreate() was called
        // before JxlEncoderSetBasicInfo().
        settings
            .values
            .cparams
            .ec_distance
            .resize(num_extra_channels, 0.0);
    }
    settings.values.cparams.ec_distance[index] = distance;
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderDistanceFromQuality. The double expressions keep the source's
/// float/double promotion; `mul_add_f64` follows Clang's AArch64 contraction
/// of the original object (FMADD in both branches) and separate operations on
/// x86-64.
pub fn jxl_encoder_distance_from_quality(quality: f32) -> f32 {
    use super::base::float::mul_add_f64;
    if quality >= 100.0 {
        0.0
    } else if quality >= 30.0 {
        mul_add_f64(f64::from(100.0 - quality), 0.09, 0.1) as f32
    } else {
        let q = f64::from(quality);
        let square = 53.0 / 3000.0 * q;
        let linear = q * -(23.0 / 20.0);
        (mul_add_f64(square, q, linear) + 25.0) as f32
    }
}

/// JxlEncoderFrameSettingsSetOption.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_frame_settings_set_option(
    frame_settings: *mut JxlEncoderFrameSettings,
    option: JxlEncoderFrameSettingId,
    value: i64,
) -> JxlEncoderStatus {
    use super::base::override_option::Override;
    use JxlEncoderFrameSettingId as Id;
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    // Tri-state to bool convertors.
    let default_to_true = |v: i64| v != 0;
    let default_to_false = |v: i64| v == 1;
    let to_override = |v: i64| Override(v as i8);

    // check if value is -1, 0 or 1 for Override-type options
    match option {
        Id::NOISE
        | Id::DOTS
        | Id::PATCHES
        | Id::GABORISH
        | Id::MODULAR
        | Id::KEEP_INVISIBLE
        | Id::GROUP_ORDER
        | Id::RESPONSIVE
        | Id::PROGRESSIVE_AC
        | Id::QPROGRESSIVE_AC
        | Id::LOSSY_PALETTE
        | Id::JPEG_RECON_CFL
        | Id::JPEG_RECON_LFS
        | Id::JPEG_COMPRESS_BOXES
        | Id::JPEG_KEEP_EXIF
        | Id::JPEG_KEEP_XMP
        | Id::JPEG_KEEP_JUMBF
            if !(-1..=1).contains(&value) =>
        {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
        _ => {}
    }

    let cparams = &mut settings.values.cparams;
    match option {
        Id::EFFORT => {
            if enc.allow_expert_options {
                if !(1..=11).contains(&value) {
                    return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
                }
            } else if !(1..=10).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            cparams.speed_tier = speed_tier_from(10 - value);
        }
        Id::BROTLI_EFFORT => {
            if !(-1..=11).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            // set cparams for brotli use in JPEG frames
            cparams.brotli_effort = value as i32;
            // set enc option for brotli use in brob boxes
            enc.brotli_effort = value as i32;
        }
        Id::DECODING_SPEED => {
            if !(0..=4).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            cparams.decoding_speed_tier = value as usize;
        }
        Id::RESAMPLING => {
            if value != -1 && value != 1 && value != 2 && value != 4 && value != 8 {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.resampling = value as i32;
        }
        Id::EXTRA_CHANNEL_RESAMPLING => {
            if value != -1 && value != 1 && value != 2 && value != 4 && value != 8 {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.ec_resampling = value as i32;
        }
        Id::ALREADY_DOWNSAMPLED => {
            if !(0..=1).contains(&value) {
                return JxlEncoderStatus::ERROR;
            }
            cparams.already_downsampled = value == 1;
        }
        Id::NOISE => cparams.noise = to_override(value),
        Id::DOTS => cparams.dots = to_override(value),
        Id::PATCHES => cparams.patches = to_override(value),
        Id::EPF => {
            if !(-1..=3).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.epf = value as i32;
        }
        Id::GABORISH => cparams.gaborish = to_override(value),
        Id::MODULAR => cparams.modular_mode = value == 1,
        Id::KEEP_INVISIBLE => cparams.keep_invisible = to_override(value),
        Id::GROUP_ORDER => cparams.centerfirst = value == 1,
        Id::GROUP_ORDER_CENTER_X => {
            if value < -1 {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.center_x = value as usize;
        }
        Id::GROUP_ORDER_CENTER_Y => {
            if value < -1 {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.center_y = value as usize;
        }
        Id::RESPONSIVE => cparams.responsive = value as i32,
        Id::PROGRESSIVE_AC => cparams.progressive_mode = to_override(value),
        Id::QPROGRESSIVE_AC => cparams.qprogressive_mode = to_override(value),
        Id::PROGRESSIVE_DC => {
            if !(-1..=2).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.progressive_dc = value as i32;
        }
        Id::PALETTE_COLORS => {
            if !(-1..=70913).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            if value == -1 {
                cparams.palette_colors = 1 << 10;
            } else {
                cparams.palette_colors = value as i32;
            }
        }
        Id::LOSSY_PALETTE => cparams.lossy_palette = default_to_false(value),
        Id::COLOR_TRANSFORM => {
            if !(-1..=2).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            if value == -1 {
                cparams.color_transform = ColorTransform::XYB;
            } else {
                cparams.color_transform = ColorTransform(value as u32);
            }
        }
        Id::MODULAR_COLOR_SPACE => {
            if !(-1..=41).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.colorspace = value as i32;
        }
        Id::MODULAR_GROUP_SIZE => {
            if !(-1..=3).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.modular_group_size_shift = value as i32;
        }
        Id::MODULAR_PREDICTOR => {
            if !(-1..=15).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            cparams.options.predictor = Predictor(value as u32);
        }
        Id::MODULAR_NB_PREV_CHANNELS => {
            // The max allowed value can in theory be higher. However, it
            // depends on the effort setting. 11 is the highest safe value that
            // doesn't cause tree_samples to be >= 64 in the encoder.
            if !(-1..=11).contains(&value) {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            if value == -1 {
                cparams.options.max_properties = 0;
            } else {
                cparams.options.max_properties = value as i32;
            }
        }
        Id::JPEG_RECON_CFL => cparams.force_cfl_jpeg_recompression = default_to_true(value),
        Id::INDEX_BOX => {
            if !(0..=1).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            settings.values.frame_index_box = true;
        }
        Id::PHOTON_NOISE => return api_error(enc, JxlEncoderError::NOT_SUPPORTED),
        Id::JPEG_COMPRESS_BOXES => cparams.jpeg_compress_boxes = default_to_true(value),
        Id::BUFFERING => {
            if !(-1..=3).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            cparams.buffering = value as i32;
        }
        Id::JPEG_KEEP_EXIF => cparams.jpeg_keep_exif = default_to_true(value),
        Id::JPEG_KEEP_XMP => cparams.jpeg_keep_xmp = default_to_true(value),
        Id::JPEG_KEEP_JUMBF => cparams.jpeg_keep_jumbf = default_to_true(value),
        Id::USE_FULL_IMAGE_HEURISTICS => {
            if !(0..=1).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            cparams.use_full_image_heuristics = default_to_false(value);
        }
        Id::DISABLE_PERCEPTUAL_HEURISTICS => {
            if !(0..=1).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            cparams.disable_perceptual_optimizations = default_to_false(value);
            if cparams.disable_perceptual_optimizations
                && enc.basic_info_set
                && enc.metadata.m.xyb_encoded
            {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
        }
        Id::OUTPUT_MODE => {
            if !(-1..=2).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            cparams.output_mode = value as i32;
        }
        Id::JPEG_RECON_LFS => cparams.force_lfs_jpeg_recompression = value as i32,
        Id::STRIP_ALPHA => {
            if !(-1..=2).contains(&value) {
                return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
            }
            settings.values.strip_alpha = value as i32;
        }
        _ => return api_error(enc, JxlEncoderError::NOT_SUPPORTED),
    }
    JxlEncoderStatus::SUCCESS
}

/// `static_cast<SpeedTier>(10 - value)` for the checked effort range.
fn speed_tier_from(value: i64) -> SpeedTier {
    match value {
        -1 => SpeedTier::TectonicPlate,
        0 => SpeedTier::Glacier,
        1 => SpeedTier::Tortoise,
        2 => SpeedTier::Kitten,
        3 => SpeedTier::Squirrel,
        4 => SpeedTier::Wombat,
        5 => SpeedTier::Hare,
        6 => SpeedTier::Cheetah,
        7 => SpeedTier::Falcon,
        8 => SpeedTier::Thunder,
        9 => SpeedTier::Lightning,
        _ => unreachable!("the effort range was checked"),
    }
}

/// JxlEncoderFrameSettingsSetFloatOption.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_frame_settings_set_float_option(
    frame_settings: *mut JxlEncoderFrameSettings,
    option: JxlEncoderFrameSettingId,
    value: f32,
) -> JxlEncoderStatus {
    use JxlEncoderFrameSettingId as Id;
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    let cparams = &mut settings.values.cparams;
    match option {
        Id::PHOTON_NOISE => {
            if value < 0.0 {
                return JxlEncoderStatus::ERROR;
            }
            cparams.photon_noise_iso = value;
            JxlEncoderStatus::SUCCESS
        }
        Id::MODULAR_MA_TREE_LEARNING_PERCENT => {
            if value < -1.0 || value > 100.0 {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            // Convert from floating point percentage to floating point
            // fraction here.
            if value < -0.5 {
                cparams.options.nb_repeats = 0.5;
            } else {
                cparams.options.nb_repeats = value * 0.01;
            }
            JxlEncoderStatus::SUCCESS
        }
        Id::CHANNEL_COLORS_GLOBAL_PERCENT => {
            if value < -1.0 || value > 100.0 {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            if value < -0.5 {
                cparams.channel_colors_pre_transform_percent = 95.0;
            } else {
                cparams.channel_colors_pre_transform_percent = value;
            }
            JxlEncoderStatus::SUCCESS
        }
        Id::CHANNEL_COLORS_GROUP_PERCENT => {
            if value < -1.0 || value > 100.0 {
                return api_error(enc, JxlEncoderError::API_USAGE);
            }
            if value < -0.5 {
                cparams.channel_colors_percent = 80.0;
            } else {
                cparams.channel_colors_percent = value;
            }
            JxlEncoderStatus::SUCCESS
        }
        Id::EFFORT
        | Id::DECODING_SPEED
        | Id::RESAMPLING
        | Id::EXTRA_CHANNEL_RESAMPLING
        | Id::ALREADY_DOWNSAMPLED
        | Id::NOISE
        | Id::DOTS
        | Id::PATCHES
        | Id::EPF
        | Id::GABORISH
        | Id::MODULAR
        | Id::KEEP_INVISIBLE
        | Id::GROUP_ORDER
        | Id::GROUP_ORDER_CENTER_X
        | Id::GROUP_ORDER_CENTER_Y
        | Id::RESPONSIVE
        | Id::PROGRESSIVE_AC
        | Id::QPROGRESSIVE_AC
        | Id::PROGRESSIVE_DC
        | Id::PALETTE_COLORS
        | Id::LOSSY_PALETTE
        | Id::COLOR_TRANSFORM
        | Id::MODULAR_COLOR_SPACE
        | Id::MODULAR_GROUP_SIZE
        | Id::MODULAR_PREDICTOR
        | Id::MODULAR_NB_PREV_CHANNELS
        | Id::JPEG_RECON_CFL
        | Id::INDEX_BOX
        | Id::BROTLI_EFFORT
        | Id::FILL_ENUM
        | Id::JPEG_COMPRESS_BOXES
        | Id::BUFFERING
        | Id::JPEG_KEEP_EXIF
        | Id::JPEG_KEEP_XMP
        | Id::JPEG_KEEP_JUMBF
        | Id::USE_FULL_IMAGE_HEURISTICS
        | Id::STRIP_ALPHA => api_error(enc, JxlEncoderError::NOT_SUPPORTED),
        _ => api_error(enc, JxlEncoderError::NOT_SUPPORTED),
    }
}

/// JxlEncoderCreate.
///
/// # Safety
/// A non-null `memory_manager` is readable and initialized. Its callbacks
/// obey memory_manager.h for the encoder's whole lifetime, never unwind and
/// never reenter the encoder. An allocation returned for the encoder object
/// or a frame-settings, queued-frame or queued-box owner is aligned for that
/// Rust type, as the source's placement new requires.
pub unsafe fn jxl_encoder_create(memory_manager: *const JxlMemoryManager) -> *mut JxlEncoder {
    let mut local_memory_manager = JxlMemoryManager::default();
    // SAFETY: The caller supplies the readable manager; the local is writable.
    if unsafe { memory_manager_init(&mut local_memory_manager, memory_manager) }.is_err() {
        return ptr::null_mut();
    }
    // SAFETY: memory_manager_init produced valid callbacks for this request.
    let alloc =
        unsafe { memory_manager_alloc(&local_memory_manager, std::mem::size_of::<JxlEncoder>()) };
    if alloc.is_null() {
        return ptr::null_mut();
    }
    let enc = alloc.cast::<JxlEncoder>();
    assert!(
        enc.is_aligned(),
        "the source's placement new requires an aligned encoder allocation"
    );
    // SAFETY: The callback returned a writable allocation of the requested
    // size, checked aligned above. Construct in place, then initialize as the
    // source does.
    unsafe {
        enc.write(JxlEncoder::construct(local_memory_manager));
        (*enc).cms = *get_default_cms();
        (*enc).cms_set = true;
        // Initialize all the field values.
        jxl_encoder_reset(&mut *enc);
    }
    enc
}

/// JxlEncoderReset. Frame settings created earlier are released; their
/// pointers must not be used again. `error`, `allow_expert_options`,
/// `brotli_effort`, the frame index box, the CMS and the JPEG metadata keep
/// their values, as in the source.
pub fn jxl_encoder_reset(enc: &mut JxlEncoder) {
    enc.thread_pool = None;
    destroy_from_back(&mut enc.input_queue);
    enc.num_queued_frames = 0;
    enc.num_queued_boxes = 0;
    destroy_from_back(&mut enc.encoder_options);
    enc.codestream_bytes_written_end_of_frame = 0;
    enc.wrote_bytes = false;
    enc.jxlp_counter = 0;
    enc.metadata = CodecMetadata::default();
    enc.last_used_cparams = CompressParams::default();
    enc.frames_closed = false;
    enc.boxes_closed = false;
    enc.basic_info_set = false;
    enc.color_encoding_set = false;
    enc.intensity_target_set = false;
    enc.use_container = false;
    enc.use_boxes = false;
    enc.container_ftyp_version = -1;
    enc.store_jpeg_metadata = false;
    enc.codestream_level = -1;
    // SAFETY: The encoder's manager cell outlives its wrapper.
    enc.output_processor = unsafe { JxlEncoderOutputProcessorWrapper::new(enc.manager()) };
    jxl_encoder_init_basic_info(&mut enc.basic_info);
}

/// JxlEncoderDestroy.
///
/// # Safety
/// `enc` is null or an encoder from [`jxl_encoder_create`] that is not used
/// again, nor are its frame settings.
pub unsafe fn jxl_encoder_destroy(enc: *mut JxlEncoder) {
    if !enc.is_null() {
        // SAFETY: The caller transfers the live encoder; copy its manager
        // before destruction, then free the object storage with it.
        unsafe {
            let local_memory_manager = *(*enc).memory_manager.get();
            // Call destructor directly since custom free function is used.
            ptr::drop_in_place(enc);
            memory_manager_free(&local_memory_manager, enc.cast());
        }
    }
}

pub fn jxl_encoder_get_error(enc: &JxlEncoder) -> JxlEncoderError {
    enc.error
}

pub fn jxl_encoder_use_container(enc: &mut JxlEncoder, use_container: i32) -> JxlEncoderStatus {
    if enc.wrote_bytes {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    enc.use_container = use_container != 0;
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_encoder_store_jpeg_metadata(
    enc: &mut JxlEncoder,
    store_jpeg_metadata: i32,
) -> JxlEncoderStatus {
    if enc.wrote_bytes {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    enc.store_jpeg_metadata = store_jpeg_metadata != 0;
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_encoder_set_codestream_level(enc: &mut JxlEncoder, level: i32) -> JxlEncoderStatus {
    if level != -1 && level != 5 && level != 10 {
        return api_error(enc, JxlEncoderError::NOT_SUPPORTED);
    }
    if enc.wrote_bytes {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    enc.codestream_level = level;
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_encoder_get_required_codestream_level(enc: &JxlEncoder) -> i32 {
    verify_level_settings(enc)
}

/// JxlEncoderSetCms.
///
/// # Safety
/// The interface's callbacks and data stay valid while the encoder can call
/// them, obey cms_interface.h, and neither unwind nor reenter the encoder.
pub unsafe fn jxl_encoder_set_cms(enc: &mut JxlEncoder, cms: JxlCmsInterface) {
    enc.cms = cms;
    enc.cms_set = true;
}

fn get_current_dimensions(
    frame_settings: &JxlEncoderFrameSettings,
    enc: &mut JxlEncoder,
    xsize: &mut usize,
    ysize: &mut usize,
) -> JxlEncoderStatus {
    *xsize = enc.metadata.xsize();
    *ysize = enc.metadata.ysize();
    let layer_info = &frame_settings.values.header.layer_info;
    if layer_info.have_crop != 0 {
        *xsize = layer_info.xsize as usize;
        *ysize = layer_info.ysize as usize;
    }
    if frame_settings.values.cparams.already_downsampled {
        // size_t from int, and DivCeil's size_t arithmetic.
        let factor = frame_settings.values.cparams.resampling as usize;
        *xsize = xsize.wrapping_add(factor).wrapping_sub(1) / factor;
        *ysize = ysize.wrapping_add(factor).wrapping_sub(1) / factor;
    }
    if *xsize == 0 || *ysize == 0 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    JxlEncoderStatus::SUCCESS
}

#[expect(
    clippy::manual_is_multiple_of,
    reason = "Mirror the source modulo test."
)]
#[allow(
    clippy::needless_bool,
    reason = "Keep the source's early-return shape."
)]
fn can_do_fast_lossless(
    frame_settings: &JxlEncoderFrameSettings,
    enc: &JxlEncoder,
    pixel_format: &JxlPixelFormat,
    has_alpha: bool,
) -> bool {
    let values = &frame_settings.values;
    let m = &enc.metadata.m;
    if !values.lossless {
        return false;
    }
    if has_alpha && values.strip_alpha > 0 {
        return false;
    }
    if values.frame_index_box {
        return false;
    }
    if values.header.layer_info.have_crop != 0 {
        return false;
    }
    if m.have_animation {
        return false;
    }
    if values.cparams.speed_tier != SpeedTier::Lightning {
        return false;
    }
    if values.image_bit_depth.depth_type == JxlBitDepthType::CUSTOM
        && values.image_bit_depth.bits_per_sample != m.bit_depth.bits_per_sample
    {
        return false;
    }
    if values.image_bit_depth.depth_type == JxlBitDepthType::FROM_PIXEL_FORMAT
        && m.bit_depth.bits_per_sample % 8 != 0
    {
        return false;
    }
    if !values.frame_name.is_empty() {
        return false;
    }
    // No extra channels other than alpha.
    if !(has_alpha && m.num_extra_channels == 1) && m.num_extra_channels != 0 {
        return false;
    }
    if m.bit_depth.bits_per_sample > 16 {
        return false;
    }
    let data_type = pixel_format.data_type;
    if data_type != JxlDataType::FLOAT16
        && data_type != JxlDataType::UINT16
        && data_type != JxlDataType::UINT8
    {
        return false;
    }
    if (m.bit_depth.bits_per_sample > 8)
        != (data_type == JxlDataType::UINT16 || data_type == JxlDataType::FLOAT16)
    {
        return false;
    }
    let num_channels = pixel_format.num_channels;
    if !((num_channels == 1 || num_channels == 3) && !has_alpha)
        && !((num_channels == 2 || num_channels == 4) && has_alpha)
    {
        return false;
    }
    true
}

fn add_image_frame_internal(
    frame_settings: &JxlEncoderFrameSettings,
    enc: &mut JxlEncoder,
    streaming: bool,
    frame_data: JxlEncoderChunkedFrameAdapter<'_>,
) -> JxlEncoderStatus {
    // enable: encoder work runs outside the decoder's dispatch scope, also
    // when a decoder callback calls it (port ADR 0004, "Scope of the
    // selection"): the port's encoder is the scalar translation.
    super::hwy::outside_decoding(|| {
        add_image_frame_outside_decoding(frame_settings, enc, streaming, frame_data)
    })
}
fn add_image_frame_outside_decoding(
    frame_settings: &JxlEncoderFrameSettings,
    enc: &mut JxlEncoder,
    streaming: bool,
    mut frame_data: JxlEncoderChunkedFrameAdapter<'_>,
) -> JxlEncoderStatus {
    let pixel_format = frame_data.get_color_channels_pixel_format();
    let num_channels = pixel_format.num_channels;
    let mut has_interleaved_alpha = usize::from(num_channels == 2 || num_channels == 4);

    if !enc.basic_info_set {
        // Basic Info must be set. Otherwise, this is an API misuse.
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if enc.frames_closed {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if num_channels < 3 {
        if enc.basic_info.num_color_channels != 1 {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
    } else if enc.basic_info.num_color_channels != 3 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if frame_settings.values.lossless && enc.metadata.m.xyb_encoded {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if frame_settings
        .values
        .cparams
        .disable_perceptual_optimizations
        && enc.metadata.m.xyb_encoded
    {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if JxlEncoderStatus::SUCCESS
        != verify_input_bit_depth(frame_settings.values.image_bit_depth, pixel_format)
    {
        return JxlEncoderStatus::ERROR;
    }
    if has_interleaved_alpha > enc.metadata.m.num_extra_channels as usize {
        if frame_settings.values.strip_alpha == STRIP_ALPHA_FORCE {
            has_interleaved_alpha = 0;
        } else {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
    }

    let has_alpha = enc.metadata.m.has_alpha();

    // All required conditions to do fast-lossless.
    if can_do_fast_lossless(frame_settings, enc, &pixel_format, has_alpha) {
        return admission(enc, JxlEncoderAdmission::FastLosslessFrame);
    }

    if !streaming {
        // The input callbacks are only guaranteed to be available during frame
        // encoding when both the input and the output is streaming. In all
        // other cases we need to create an internal copy of the frame data.
        if !frame_data.copy_buffers() {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
    }

    if !enc.color_encoding_set {
        let c_current: &ColorEncoding = if pixel_format.data_type == JxlDataType::FLOAT
            || pixel_format.data_type == JxlDataType::FLOAT16
        {
            ColorEncoding::linear_srgb(num_channels < 3)
        } else {
            ColorEncoding::srgb(num_channels < 3)
        };
        enc.metadata.m.color_encoding = c_current.clone();
    }
    let value = JxlEncoderQueuedFrame {
        option_values: frame_settings.values.clone(),
        frame_data: frame_data.into_detached(),
        ec_initialized: Vec::new(),
    };
    // SAFETY: The encoder's manager cell outlives its queued inputs, and the
    // create contract makes its callbacks valid for this allocation.
    let Some(mut queued_frame) = (unsafe { MemoryManagerUniquePtr::make(enc.manager(), value) })
    else {
        return api_error(enc, JxlEncoderError::OOM);
    };

    for ec_info in &enc.metadata.m.extra_channel_info {
        if has_interleaved_alpha != 0 && ec_info.channel_type == ExtraChannel::ALPHA {
            queued_frame.ec_initialized.push(1);
            has_interleaved_alpha = 0; // only first Alpha is initialized
        } else {
            queued_frame.ec_initialized.push(0);
        }
    }
    queued_frame.option_values.cparams.level = enc.codestream_level;

    queue_frame(frame_settings, enc, queued_frame);
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderAddImageFrame. `None` is the source's null buffer with size 0;
/// the slice length is the source's `size`.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_add_image_frame(
    frame_settings: *const JxlEncoderFrameSettings,
    pixel_format: &JxlPixelFormat,
    buffer: Option<&[u8]>,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract; the source
    // accesses the settings as const.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings.cast_mut()) };
    let settings: &JxlEncoderFrameSettings = settings;
    let mut xsize = 0;
    let mut ysize = 0;
    if get_current_dimensions(settings, enc, &mut xsize, &mut ysize) != JxlEncoderStatus::SUCCESS {
        return api_error(enc, JxlEncoderError::GENERIC);
    }
    let mut frame_data = JxlEncoderChunkedFrameAdapter::new(
        xsize,
        ysize,
        enc.metadata.m.num_extra_channels as usize,
    );
    let size = buffer.map_or(0, <[u8]>::len);
    if !frame_data.set_from_buffer(0, buffer, size, *pixel_format) {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    add_image_frame_internal(settings, enc, false, frame_data)
}

pub fn jxl_encoder_use_boxes(enc: &mut JxlEncoder) -> JxlEncoderStatus {
    if enc.wrote_bytes {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    enc.use_boxes = true;
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_encoder_add_box(
    enc: &mut JxlEncoder,
    box_type: &[u8; 4],
    contents: &[u8],
    compress_box: i32,
) -> JxlEncoderStatus {
    if !enc.use_boxes {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if enc.boxes_closed {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if compress_box != 0 {
        if &box_type[..3] == b"jxl" {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
        if box_type == b"jbrd" {
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
        if box_type == b"brob" {
            // The compress_box will compress an existing non-brob box into a
            // brob box. If already giving a valid brotli-compressed brob box,
            // set compress_box to false since it is already compressed.
            return api_error(enc, JxlEncoderError::API_USAGE);
        }
    }
    let value = JxlEncoderQueuedBox {
        box_type: make_box_type(box_type),
        contents: contents.to_vec(),
        compress_box: compress_box != 0,
    };
    // SAFETY: As for queued frames.
    let Some(queued_box) = (unsafe { MemoryManagerUniquePtr::make(enc.manager(), value) }) else {
        return api_error(enc, JxlEncoderError::OOM);
    };
    queue_box(enc, queued_box);
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetExtraChannelBuffer. `None` is the source's null buffer with
/// size 0.
///
/// The source dereferences the last queued input's frame without a check;
/// when that input is a box, the original's behavior is undefined and this
/// function panics there.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_set_extra_channel_buffer(
    frame_settings: *const JxlEncoderFrameSettings,
    pixel_format: &JxlPixelFormat,
    buffer: Option<&[u8]>,
    index: u32,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings.cast_mut()) };
    let settings: &JxlEncoderFrameSettings = settings;
    if index >= enc.metadata.m.num_extra_channels {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if !enc.basic_info_set || !enc.color_encoding_set {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if enc.input_queue.is_empty() {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if enc.frames_closed {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    let mut ec_format = *pixel_format;
    ec_format.num_channels = 1;
    if JxlEncoderStatus::SUCCESS
        != verify_input_bit_depth(settings.values.image_bit_depth, ec_format)
    {
        return JxlEncoderStatus::ERROR;
    }
    let queued_frame = enc
        .input_queue
        .last_mut()
        .expect("checked above")
        .frame
        .as_mut()
        .expect("the source dereferences the last queued frame without a check");
    let size = buffer.map_or(0, <[u8]>::len);
    if !queued_frame
        .frame_data
        .set_extra_from_buffer(1 + index as usize, buffer, size, ec_format)
    {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    queued_frame.ec_initialized[index as usize] = 1;
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_encoder_close_frames(enc: &mut JxlEncoder) {
    enc.frames_closed = true;
}

pub fn jxl_encoder_close_boxes(enc: &mut JxlEncoder) {
    enc.boxes_closed = true;
}

pub fn jxl_encoder_close_input(enc: &mut JxlEncoder) {
    jxl_encoder_close_frames(enc);
    jxl_encoder_close_boxes(enc);
}

/// How ProcessOneEnqueuedInput failed: with the status the source returns
/// (JXL_API_ERROR has set `error`), or with an admission result.
enum EnqueuedInputFailure {
    Failed,
    Admission(JxlEncoderAdmission),
}
impl From<StatusCode> for EnqueuedInputFailure {
    fn from(_: StatusCode) -> Self {
        Self::Failed
    }
}

fn enqueued_api_error(enc: &mut JxlEncoder, error_code: JxlEncoderError) -> EnqueuedInputFailure {
    api_error(enc, error_code);
    EnqueuedInputFailure::Failed
}

/// The default memory manager after MemoryManagerInit with a null manager.
fn uses_default_memory_manager(enc: &JxlEncoder) -> bool {
    let manager = enc.memory_manager.get();
    manager.alloc.map(|f| f as usize) == Some(default_alloc as *const () as usize)
        && manager.free.map(|f| f as usize) == Some(default_free as *const () as usize)
}

/// JxlEncoderFrameIndexBox::AddFrame.
fn frame_index_box_add_frame(
    frame_index_box: &mut JxlEncoderFrameIndexBox,
    offi: u64,
    duration: u32,
    to_be_indexed: bool,
) {
    if frame_index_box.entries.len() == 1 && offi == frame_index_box.entries[0].offi {
        // API use for the first frame, let's clear the already recorded first
        // frame.
        frame_index_box.entries.clear();
    }
    frame_index_box.entries.push(JxlEncoderFrameIndexBoxEntry {
        to_be_indexed,
        duration,
        offi,
    });
}

/// JxlEncoderFrameIndexBox::StoreFrameIndexBox.
fn store_frame_index_box(frame_index_box: &JxlEncoderFrameIndexBox) -> bool {
    frame_index_box.entries.iter().any(|e| e.to_be_indexed)
}

/// JxlEncoder::MustUseContainer.
fn must_use_container(enc: &JxlEncoder, output_mode: i32) -> bool {
    enc.container_ftyp_version >= 0
        || enc.use_container
        || (enc.codestream_level != 5 && enc.codestream_level != -1)
        || enc.store_jpeg_metadata
        || enc.use_boxes
        || output_mode == 2
}

/// IsAlphaBufferOpaque.
fn is_alpha_buffer_opaque(
    buffer: Option<&[u8]>,
    format: &JxlPixelFormat,
    xsize: usize,
    ysize: usize,
    row_offset: usize,
    alpha_c: usize,
    alpha_bits: usize,
) -> bool {
    let Some(p) = buffer else {
        return false;
    };
    if format.num_channels as usize <= alpha_c {
        return false;
    }
    let bytes_per_pixel = super::encode_internal::bytes_per_pixel(*format);
    if bytes_per_pixel == 0 {
        return false;
    }
    let swap = swap_endianness(format.endianness);
    if format.data_type == JxlDataType::UINT8 {
        let offset = alpha_c;
        let expected: u8 = if alpha_bits > 0 && alpha_bits <= 8 {
            ((1u32 << alpha_bits) - 1) as u8
        } else {
            255
        };
        for y in 0..ysize {
            let row = &p[y * row_offset..];
            for x in 0..xsize {
                if row[x * bytes_per_pixel + offset] != expected {
                    return false;
                }
            }
        }
        true
    } else if format.data_type == JxlDataType::UINT16 {
        let offset = alpha_c * 2;
        let expected: u16 = if alpha_bits > 0 && alpha_bits <= 16 {
            ((1u32 << alpha_bits) - 1) as u16
        } else {
            65535
        };
        for y in 0..ysize {
            let row = &p[y * row_offset..];
            for x in 0..xsize {
                let at = x * bytes_per_pixel + offset;
                let mut val = u16::from_ne_bytes([row[at], row[at + 1]]);
                if swap {
                    val = val.swap_bytes();
                }
                if val != expected {
                    return false;
                }
            }
        }
        true
    } else if format.data_type == JxlDataType::FLOAT {
        let offset = alpha_c * 4;
        for y in 0..ysize {
            let row = &p[y * row_offset..];
            for x in 0..xsize {
                let at = x * bytes_per_pixel + offset;
                let mut bits = u32::from_ne_bytes(row[at..at + 4].try_into().unwrap());
                if swap {
                    bits = bits.swap_bytes();
                }
                if bits != 0x3F80_0000 {
                    return false;
                }
            }
        }
        true
    } else if format.data_type == JxlDataType::FLOAT16 {
        let offset = alpha_c * 2;
        for y in 0..ysize {
            let row = &p[y * row_offset..];
            for x in 0..xsize {
                let at = x * bytes_per_pixel + offset;
                let mut bits = u16::from_ne_bytes([row[at], row[at + 1]]);
                if swap {
                    bits = bits.swap_bytes();
                }
                if bits != 0x3C00 {
                    return false;
                }
            }
        }
        true
    } else {
        false
    }
}

/// The alpha stripping decision of ProcessOneEnqueuedInput before the first
/// output: strip alpha when it is forced, or when the single extra channel of
/// a single-frame still is fully opaque (IsAlphaBufferOpaque over chunks).
fn strip_opaque_alpha(enc: &mut JxlEncoder) {
    const STRIP_ALPHA_AUTO: i32 = -1;
    const STRIP_ALPHA_IF_OPAQUE: i32 = 2;
    let first_frame = enc.input_queue.iter().position(|qi| qi.frame.is_some());

    if !enc.metadata.m.has_alpha() {
        return;
    }
    let alpha_ec_idx = enc
        .metadata
        .m
        .extra_channel_info
        .iter()
        .position(|eci| eci.channel_type == ExtraChannel::ALPHA)
        .expect("HasAlpha");
    let alpha_bits = enc.metadata.m.extra_channel_info[alpha_ec_idx]
        .bit_depth
        .bits_per_sample as usize;
    let frame = first_frame.map(|i| enc.input_queue[i].frame.as_deref().expect("frame"));
    let strip_alpha = frame.map_or(STRIP_ALPHA_AUTO, |f| f.option_values.strip_alpha);
    let is_lossless = frame.is_some_and(|f| f.option_values.lossless);

    // Only auto-strip alpha for single-frame images with exactly one extra
    // channel.
    let is_multiframe_or_animation = enc.metadata.m.have_animation
        || !enc.frames_closed
        || enc.num_queued_frames != 1
        || enc.metadata.m.num_extra_channels != 1;

    let mut should_strip =
        strip_alpha == STRIP_ALPHA_FORCE && enc.metadata.m.num_extra_channels == 1;

    // Check if alpha can be stripped because it is fully opaque.
    if let Some(first_frame) = frame
        && !should_strip
        && !is_multiframe_or_animation
        && alpha_ec_idx < first_frame.ec_initialized.len()
        && first_frame.ec_initialized[alpha_ec_idx] != 0
        && (strip_alpha == STRIP_ALPHA_IF_OPAQUE
            || (strip_alpha == STRIP_ALPHA_AUTO && !is_lossless))
    {
        // The buffer-backed input source sets every callback.
        let src = &first_frame.frame_data;
        let color_fmt = src.get_color_channels_pixel_format();
        let has_interleaved = color_fmt.num_channels == 2 || color_fmt.num_channels == 4;
        let has_extra_channel = !has_interleaved;
        if has_interleaved || has_extra_channel {
            let ec_fmt = if has_extra_channel {
                src.get_extra_channel_pixel_format(alpha_ec_idx)
            } else {
                color_fmt
            };
            let fmt = if has_interleaved { color_fmt } else { ec_fmt };
            let alpha_c = if has_interleaved {
                color_fmt.num_channels as usize - 1
            } else {
                0
            };
            let is_chunk_opaque = |x: usize, y: usize, xs: usize, ys: usize| -> bool {
                let mut row_offset = 0;
                let buf = if has_interleaved {
                    src.get_color_channel_data_at(x, y, xs, ys, &mut row_offset)
                } else {
                    src.get_extra_channel_data_at(alpha_ec_idx, x, y, xs, ys, &mut row_offset)
                };
                buf.is_some()
                    && is_alpha_buffer_opaque(buf, &fmt, xs, ys, row_offset, alpha_c, alpha_bits)
            };

            const CHUNK_DIM_Y: usize = 256;
            const CHUNK_DIM_X: usize = 2048;
            let xsize = enc.metadata.xsize();
            let ysize = enc.metadata.ysize();
            let mut all_opaque = true;

            let mut y = 0;
            while y < ysize && all_opaque {
                let chunk_ysize = CHUNK_DIM_Y.min(ysize - y);
                let mut x = 0;
                while x < xsize {
                    let chunk_xsize = CHUNK_DIM_X.min(xsize - x);
                    if !is_chunk_opaque(x, y, chunk_xsize, chunk_ysize) {
                        all_opaque = false;
                        break;
                    }
                    x += CHUNK_DIM_X;
                }
                y += CHUNK_DIM_Y;
            }
            should_strip = all_opaque;
        }
    }

    if should_strip {
        enc.metadata.m.extra_channel_info.remove(alpha_ec_idx);
        enc.metadata.m.num_extra_channels -= 1;
        enc.basic_info.num_extra_channels = enc.metadata.m.num_extra_channels;
        enc.basic_info.alpha_bits = 0;
        enc.basic_info.alpha_exponent_bits = 0;
        enc.basic_info.alpha_premultiplied = 0;

        for qi in &mut enc.input_queue {
            if let Some(frame) = qi.frame.as_deref_mut() {
                if alpha_ec_idx < frame.ec_initialized.len() {
                    frame.ec_initialized.remove(alpha_ec_idx);
                }
                if alpha_ec_idx < frame.option_values.extra_channel_blend_info.len() {
                    frame
                        .option_values
                        .extra_channel_blend_info
                        .remove(alpha_ec_idx);
                }
                if alpha_ec_idx < frame.option_values.cparams.ec_distance.len() {
                    frame.option_values.cparams.ec_distance.remove(alpha_ec_idx);
                }
            }
        }
    }
}

/// JxlEncoder::ProcessOneEnqueuedInput for raw codestream output: the
/// metadata header and one frame through EncodeFrame. The branches that
/// enter untranslated units are admission results at their checkpoints:
/// ICC profiles (WriteICC), containers and boxes (step 3), and the frame encoder's own
/// checkpoints ([`FrameAdmission`]).
#[allow(clippy::needless_range_loop, reason = "Keep the source index loops.")]
fn process_one_enqueued_input(enc: &mut JxlEncoder) -> Result<(), EnqueuedInputFailure> {
    let output_mode = enc.input_queue[0].output_mode; // resolved at queue time

    // Mode 2 requires that the container was started with ftyp version 1.
    if output_mode == 2 && enc.wrote_bytes && enc.container_ftyp_version != 1 {
        return Err(enqueued_api_error(enc, JxlEncoderError::API_USAGE));
    }

    let mut header_bytes: Vec<u8> = Vec::new();
    if !enc.wrote_bytes {
        strip_opaque_alpha(enc);

        // First time encoding any data, verify the level 5 vs level 10 settings
        let required_level = verify_level_settings(enc);
        // Only level 5 and 10 are defined, and the function can return -1 to
        // indicate full incompatibility.
        if !(required_level == -1 || required_level == 5 || required_level == 10) {
            return Err(EnqueuedInputFailure::Failed);
        }
        // codestream_level == -1 means auto-set to the required level
        if enc.codestream_level == -1 {
            enc.codestream_level = required_level;
        }
        if enc.codestream_level == 5 && required_level != 5 {
            return Err(enqueued_api_error(enc, JxlEncoderError::API_USAGE));
        }
        if required_level == -1 {
            return Err(enqueued_api_error(enc, JxlEncoderError::API_USAGE));
        }
        let mut writer = BitWriter::new();
        if write_codestream_headers(&mut enc.metadata, &mut writer, None).is_err() {
            return Err(enqueued_api_error(enc, JxlEncoderError::GENERIC));
        }
        // Only send ICC (at least several hundred bytes) if fields aren't enough.
        if enc.metadata.m.color_encoding.want_icc() {
            // WriteICC (enc_icc_codec.cc) is step 6.
            return Err(EnqueuedInputFailure::Admission(
                JxlEncoderAdmission::EnqueuedInput,
            ));
        }
        writer.with_max_bits(
            8,
            LayerType::Header,
            None,
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )?;
        header_bytes = writer.take_bytes().as_slice().to_vec();

        // Not actually the end of frame, but the end of metadata/ICC, but helps
        // the next frame to start here for indexing purposes.
        enc.codestream_bytes_written_end_of_frame += header_bytes.len();

        if must_use_container(enc, output_mode) {
            // The container header, the jxll box and the partial-header jxlp
            // box are step 3.
            return Err(EnqueuedInputFailure::Admission(
                JxlEncoderAdmission::EnqueuedInput,
            ));
        }
        enc.wrote_bytes = true;
    }

    enc.output_processor.set_finalized_position()?;

    // Choose frame or box processing: exactly one of the two owners (box or
    // frame) in the input queue item is non-null.
    if enc.input_queue[0].frame.is_none() {
        // Box output (uncompressed or brob) needs a container, so it is
        // reached only after the container admission above.
        return Err(EnqueuedInputFailure::Admission(
            JxlEncoderAdmission::EnqueuedInput,
        ));
    }
    let mut input = enc.input_queue.remove(0);
    let mut input_frame = input.frame.take().expect("checked above");
    enc.num_queued_frames -= 1;
    for idx in 0..input_frame.ec_initialized.len() {
        if input_frame.ec_initialized[idx] == 0 {
            return Err(enqueued_api_error(enc, JxlEncoderError::API_USAGE));
        }
    }
    if enc.metadata.m.xyb_encoded {
        input_frame.option_values.cparams.color_transform = ColorTransform::XYB;
    } else {
        input_frame.option_values.cparams.color_transform = ColorTransform::NONE;
    }

    let (duration, timecode) = if enc.metadata.m.have_animation {
        (
            input_frame.option_values.header.duration,
            input_frame.option_values.header.timecode,
        )
    } else {
        // If have_animation is false, the encoder should ignore the duration
        // and timecode values.
        (0, 0)
    };

    let last_frame = enc.frames_closed && enc.num_queued_frames == 0;

    // The box-size estimate of the source (max_bits_per_sample, the upper
    // bound and use_large_box) serves only the container branches.
    let frame_start_pos = enc.output_processor.current_position();
    let n0 = enc.jxlp_counter;
    let content_start = enc.output_processor.current_position();

    if !header_bytes.is_empty() {
        append_data(&mut enc.output_processor, &header_bytes)?;
    }
    let _ = frame_start_pos;

    frame_index_box_add_frame(
        &mut enc.frame_index_box,
        enc.codestream_bytes_written_end_of_frame as u64,
        duration,
        input_frame.option_values.frame_index_box,
    );

    let layer_info = input_frame.option_values.header.layer_info;
    let save_as_reference = layer_info.save_as_reference as usize;
    if save_as_reference >= 3 {
        return Err(enqueued_api_error(enc, JxlEncoderError::API_USAGE));
    }

    let mut frame_info = FrameInfo {
        is_last: last_frame,
        save_as_reference,
        source: layer_info.blend_info.source as usize,
        clamp: layer_info.blend_info.clamp != 0,
        alpha_channel: layer_info.blend_info.alpha as i32,
        ..FrameInfo::default()
    };
    let num_extra_channels = enc.metadata.m.num_extra_channels as usize;
    frame_info
        .extra_channel_blending_info
        .resize_with(num_extra_channels, BlendingInfo::default);
    // If extra channel blend info has not been set, use the blend mode from
    // the layer_info.
    let default_blend_info = layer_info.blend_info;
    for i in 0..num_extra_channels {
        let from = input_frame
            .option_values
            .extra_channel_blend_info
            .get(i)
            .copied()
            .unwrap_or(default_blend_info);
        let to = &mut frame_info.extra_channel_blending_info[i];
        to.mode = BlendMode(from.blendmode.0);
        to.source = from.source;
        to.alpha_channel = from.alpha;
        to.clamp = from.clamp != 0;
    }
    frame_info.origin.x0 = layer_info.crop_x0;
    frame_info.origin.y0 = layer_info.crop_y0;
    frame_info.blendmode = BlendMode(layer_info.blend_info.blendmode.0);
    frame_info.blend = layer_info.blend_info.blendmode != JxlBlendMode::REPLACE;
    frame_info.image_bit_depth = input_frame.option_values.image_bit_depth;
    frame_info.duration = duration;
    frame_info.timecode = timecode;
    frame_info.name = input_frame.option_values.frame_name.clone();
    let mut frame_cparams = input_frame.option_values.cparams.clone();
    frame_cparams.output_mode = output_mode;
    let default_memory_manager = uses_default_memory_manager(enc);
    match encode_frame(
        &frame_cparams,
        &frame_info,
        &enc.metadata,
        &input_frame.frame_data,
        &mut enc.output_processor,
        default_memory_manager,
    ) {
        Ok(()) => {}
        Err(EncodeFrameError::Admission(checkpoint)) => {
            return Err(EnqueuedInputFailure::Admission(match checkpoint {
                FrameAdmission::StreamingFrame => JxlEncoderAdmission::StreamingFrame,
                FrameAdmission::FrameProfile => JxlEncoderAdmission::FrameProfile,
                FrameAdmission::PatchDictionary => JxlEncoderAdmission::PatchDictionary,
            }));
        }
        Err(EncodeFrameError::Status(_)) => {
            return Err(enqueued_api_error(enc, JxlEncoderError::GENERIC));
        }
    }
    enc.last_used_cparams = input_frame.option_values.cparams.clone();

    let content_size = enc.output_processor.current_position() - content_start;
    enc.codestream_bytes_written_end_of_frame = enc
        .codestream_bytes_written_end_of_frame
        .wrapping_add(content_size)
        .wrapping_sub(header_bytes.len())
        .wrapping_sub((enc.jxlp_counter - n0) * 12);

    enc.output_processor.set_finalized_position()?;
    if last_frame && store_frame_index_box(&enc.frame_index_box) {
        // The jxli box is container output (step 3).
        return Err(EnqueuedInputFailure::Admission(
            JxlEncoderAdmission::EnqueuedInput,
        ));
    }
    drop(input_frame);
    drop(input);
    Ok(())
}

/// JxlEncoderProcessOutput. ProcessOneEnqueuedInput returns an admission
/// result at its untranslated checkpoints (see the module header).
///
/// # Safety
/// `next_out` and `avail_out` are valid, `*next_out` points to `*avail_out`
/// writable bytes, and no Rust reference to them is live during the call. The
/// encoder retains both pointers, as the source does; they are written only
/// during a later ProcessOutput call, which replaces them first.
pub unsafe fn jxl_encoder_process_output(
    enc: &mut JxlEncoder,
    next_out: *mut *mut u8,
    avail_out: *mut usize,
) -> JxlEncoderStatus {
    if enc.output_processor.output_processor_set() {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    // SAFETY: The caller supplies the output pointer contract.
    if unsafe { enc.output_processor.set_avail_out(next_out, avail_out) }.is_err() {
        return JxlEncoderStatus::ERROR;
    }
    // SAFETY: As above; the output wrapper writes through the retained
    // pointers only inside the calls below, and no reference is held across
    // them.
    while unsafe { *avail_out } != 0 && !enc.input_queue.is_empty() {
        // enable: encoder work runs outside the decoder's dispatch scope, also
        // when a decoder callback calls it (port ADR 0004, "Scope of the
        // selection"): the port's encoder is the scalar translation.
        match super::hwy::outside_decoding(|| process_one_enqueued_input(enc)) {
            Ok(()) => {}
            Err(EnqueuedInputFailure::Failed) => return JxlEncoderStatus::ERROR,
            Err(EnqueuedInputFailure::Admission(checkpoint)) => {
                return admission(enc, checkpoint);
            }
        }
    }
    if !enc.input_queue.is_empty() || enc.output_processor.has_output_to_write() {
        return JxlEncoderStatus::NEED_MORE_OUTPUT;
    }
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetFrameHeader.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_set_frame_header(
    frame_settings: *mut JxlEncoderFrameSettings,
    frame_header: &JxlFrameHeader,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    if frame_header.layer_info.blend_info.source > 3 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    // If there are no extra channels, it's ok for the value to be 0.
    if frame_header.layer_info.blend_info.alpha != 0
        && frame_header.layer_info.blend_info.alpha as usize
            >= enc.metadata.m.extra_channel_info.len()
    {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    settings.values.header = *frame_header;
    // Setting the frame header resets the frame name, it must be set again
    // with JxlEncoderSetFrameName if desired.
    settings.values.frame_name.clear();
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetExtraChannelBlendInfo.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_set_extra_channel_blend_info(
    frame_settings: *mut JxlEncoderFrameSettings,
    index: usize,
    blend_info: &JxlBlendInfo,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    let num_extra_channels = enc.metadata.m.num_extra_channels as usize;
    if index >= num_extra_channels {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    if settings.values.extra_channel_blend_info.len() != num_extra_channels {
        let mut default_blend_info = JxlBlendInfo::default();
        jxl_encoder_init_blend_info(&mut default_blend_info);
        settings
            .values
            .extra_channel_blend_info
            .resize(num_extra_channels, default_blend_info);
    }
    settings.values.extra_channel_blend_info[index] = *blend_info;
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetFrameName; `None` is the source's null name.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_set_frame_name(
    frame_settings: *mut JxlEncoderFrameSettings,
    frame_name: Option<&CStr>,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, enc) = unsafe { settings_and_encoder(frame_settings) };
    let str = frame_name.map_or(&[][..], CStr::to_bytes);
    if str.len() > 1071 {
        return api_error(enc, JxlEncoderError::API_USAGE);
    }
    settings.values.frame_name = str.to_vec();
    settings.values.header.name_length = str.len() as u32;
    JxlEncoderStatus::SUCCESS
}

/// JxlEncoderSetFrameBitDepth.
///
/// # Safety
/// As for [`jxl_encoder_set_frame_lossless`].
pub unsafe fn jxl_encoder_set_frame_bit_depth(
    frame_settings: *mut JxlEncoderFrameSettings,
    bit_depth: &JxlBitDepth,
) -> JxlEncoderStatus {
    // SAFETY: The caller supplies the frame-settings contract.
    let (settings, _enc) = unsafe { settings_and_encoder(frame_settings) };
    if bit_depth.depth_type != JxlBitDepthType::FROM_PIXEL_FORMAT
        && bit_depth.depth_type != JxlBitDepthType::FROM_CODESTREAM
    {
        return JxlEncoderStatus::ERROR;
    }
    settings.values.image_bit_depth = *bit_depth;
    JxlEncoderStatus::SUCCESS
}

pub fn jxl_color_encoding_set_to_srgb(color_encoding: &mut JxlColorEncoding, is_gray: i32) {
    *color_encoding = ColorEncoding::srgb(is_gray != 0).to_external();
}

pub fn jxl_color_encoding_set_to_linear_srgb(color_encoding: &mut JxlColorEncoding, is_gray: i32) {
    *color_encoding = ColorEncoding::linear_srgb(is_gray != 0).to_external();
}

pub fn jxl_encoder_allow_expert_options(enc: &mut JxlEncoder) {
    enc.allow_expert_options = true;
}
