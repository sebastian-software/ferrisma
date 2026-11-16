//! Narrow, owned safe boundary to the workspace libjxl-rs encoder (ADR-0039).
//!
//! The port's public encoder API (`jxl::jxl::encode`) is a translation of
//! libjxl's C API: an encoder object and its frame settings are raw pointers
//! with lifetime contracts. [`Encoder`] owns one encoder and one frame
//! settings object, copies the caller's pixels during
//! [`Encoder::add_frame`], and drains the codestream into a caller-owned
//! `Vec<u8>` with an output-byte limit. Every non-success status becomes an
//! [`Error`] that separates requests the port does not admit yet
//! ([`Error::NotAdmitted`]) from API misuse and real failures.
//!
//! The admitted lossy profile is the port's (see its `ENCODER_EXPANSION.md`,
//! steps 1 and 4, and `PORT_STATUS.md`, "Lossy VarDCT with extra channels and
//! patches off"): effort 7, VarDCT with XYB, distance in
//! [`MIN_DISTANCE`]`..`[`MAX_ADMITTED_DISTANCE`], gray or RGB, optionally with
//! one straight alpha channel coded losslessly, enum sRGB or linear sRGB,
//! one frame, serial, default memory manager. Samples are 8-bit, or (since
//! 2026-11-15) 9 to 16 bits as `uint16` with `JXL_BIT_DEPTH_FROM_CODESTREAM`,
//! alpha at the color depth, as `cjxl` passes a PNM of that `maxval`. Patches
//! are off by default ([`FrameOptions::patches`]); with them off the port
//! encodes every image. This bridge exposes nothing outside the profile
//! except the distance and the patches switch, so that the port's admission
//! checkpoint can be inferred (see [`Admission`]).
//!
//! The admitted lossless profile ([`encode_lossless`], issue #223) is the
//! port's lossless Modular profile at effort 2 (`ENCODER_EXPANSION.md` and
//! `PORT_STATUS.md`, "Encoder lossless streaming"): libjxl's own lossless
//! setting (`JxlEncoderSetFrameLossless`, `uses_original_profile`), gray or
//! RGB of 8 to 16 bits with optional straight alpha at the same depth, enum
//! sRGB or linear sRGB, one frame, serial, default memory manager and the default
//! buffering, so that more than eight 256x256 groups take the original's
//! chunked (streaming) encode, as `cjxl -d 0 -e 2` does by default.
//!
//! Output is a raw codestream, except where libjxl requires codestream level
//! 10 (`VerifyLevelSettings`: lossless color above 12 bits, or alpha above 12
//! bits): the original and the port then write the ISOBMFF container with a
//! `jxll` box, and so does this bridge (the level stays automatic).
//!
//! Internal to `ferrisma`: this crate follows no SemVer of its own,
//! `ferrisma` requires exactly the version released with it (an `=`
//! requirement), and any release may change this API. It is not for
//! direct use; enable the matching `ferrisma` feature instead (ADR-0040,
//! problem 7).
#![deny(clippy::undocumented_unsafe_blocks)]

use std::fmt;
use std::ptr::{self, NonNull};

use jxl::jxl::codestream_header::{JxlBasicInfo, JxlOrientation};
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::encode::{
    self as api, JxlEncoder, JxlEncoderError, JxlEncoderFrameSettingId as Id,
    JxlEncoderFrameSettings, JxlEncoderStatus as Status,
};
use jxl::jxl::types::{JxlBitDepth, JxlBitDepthType, JxlDataType, JxlEndianness, JxlPixelFormat};

/// libjxl's `kMinButteraugliDistance`; smaller positive distances are
/// clamped to it by `JxlEncoderSetFrameDistance`.
pub const MIN_DISTANCE: f32 = 0.05;
/// Exclusive upper bound of the admitted lossy profile: libjxl's
/// `kMinButteraugliForDots`. At 3.0 and above the original runs dot
/// detection, which the port does not translate yet.
pub const MAX_ADMITTED_DISTANCE: f32 = 3.0;
/// libjxl's default distance ("visually lossless").
pub const DEFAULT_DISTANCE: f32 = 1.0;
/// The admitted effort (libjxl "squirrel"), set explicitly.
const EFFORT: i64 = 7;
/// The admitted lossless effort (libjxl "thunder"), set explicitly.
pub const LOSSLESS_EFFORT: i64 = 2;
const GROUP_DIM: usize = 256;
const BLOCK_DIM: usize = 8;
/// `CanDoStreamingEncoding` selects the streaming frame encode above this
/// many groups (with default buffering); patches are searched only below it.
const MAX_ONE_SHOT_GROUPS: usize = 8;

/// The port's admission checkpoint. The public API reports every one of them
/// as `JXL_ENC_ERR_NOT_SUPPORTED`; the bridge infers which from the stage
/// and from its own request, which it keeps inside the admitted profile:
///
/// - the port stops at `EnqueuedInput` ([`Admission::AlphaOrIcc`]) only for
///   an ICC profile (encoder step 6; alpha deeper than 12 bits is encoded
///   since the port's container block). The bridge offers no ICC, so it
///   never infers this variant; it is kept for that step;
/// - an in-profile request with patches on ([`FrameOptions::patches`]) on
///   the one-shot path (at most 8 groups) can only stop at
///   `PatchDictionary`: FindTextLikePatches found text-like patches, whose
///   reference-frame encode is not translated yet. With patches off (the
///   default) the port skips the patch search, so this is unreachable;
/// - everything else is [`Admission::FrameProfile`].
///
/// The crate's tests check this inference against the port's own
/// observation (`take_admissions`, test builds only).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Admission {
    /// The content needs the encoder's patch path (text-like repeated
    /// shapes), which the port does not encode yet.
    PatchDictionary,
    /// An ICC profile. Not produced by this bridge, which offers none.
    AlphaOrIcc,
    /// Any other request outside the admitted profile.
    FrameProfile,
}

/// The API call that failed.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Stage {
    Create,
    BasicInfo,
    ColorEncoding,
    FrameSettings,
    AddFrame,
    ProcessOutput,
}

#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub enum Error {
    /// The port does not encode this request yet; nothing is wrong with it.
    NotAdmitted(Admission),
    /// The port rejected the call order or a value (`JXL_ENC_ERR_API_USAGE`).
    ApiUsage(Stage),
    /// The bridge rejected the call before reaching the port.
    Misuse(&'static str),
    /// The codestream is longer than the caller's limit; the output vector
    /// is restored to its length before the call.
    OutputLimit { limit: usize },
    /// An encoder allocation or the output vector's reservation failed.
    OutOfMemory,
    /// Any other encoder failure (`JXL_ENC_ERR_GENERIC`, `BAD_INPUT`, or an
    /// error status without a code).
    Failed(Stage),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NotAdmitted(Admission::PatchDictionary) => write!(
                f,
                "the content needs the JPEG XL encoder's patch path, which is not available yet"
            ),
            Self::NotAdmitted(Admission::AlphaOrIcc) => {
                write!(f, "ICC profiles are not admitted by the encoder yet")
            }
            Self::NotAdmitted(Admission::FrameProfile) => {
                write!(f, "the request is outside the encoder's admitted profile")
            }
            Self::ApiUsage(stage) => write!(f, "encoder API usage error at {stage:?}"),
            Self::Misuse(reason) => write!(f, "encoder bridge misuse: {reason}"),
            Self::OutputLimit { limit } => {
                write!(f, "encoded output exceeds the {limit}-byte limit")
            }
            Self::OutOfMemory => write!(f, "encoder allocation failed"),
            Self::Failed(stage) => write!(f, "encoder failed at {stage:?}"),
        }
    }
}

impl std::error::Error for Error {}

/// Color channels of the interleaved input.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum ColorChannels {
    Gray,
    Rgb,
}

/// Alpha channel of the interleaved input. Only what the port admits and the
/// bridge has verified is representable, so an unadmitted alpha request
/// (float, premultiplied, a separate buffer, another depth than the color's)
/// cannot be expressed.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
#[non_exhaustive]
pub enum Alpha {
    /// No alpha: 1 (gray) or 3 (RGB) interleaved channels.
    #[default]
    None,
    /// One straight (unassociated) alpha channel at the image's
    /// [`ImageInfo::bits_per_sample`] interleaved after the color channels:
    /// gray+alpha (2 channels) or RGBA (4 channels); 8-bit until 2026-11-15,
    /// when the name was kept. The port
    /// codes it losslessly (extra-channel distance 0). At the port's default
    /// `STRIP_ALPHA` setting a fully opaque alpha channel is removed from
    /// the codestream; callers that need an explicit decision make it before
    /// the call.
    Straight8,
}

/// Enum color encoding of the input samples, signaled in the codestream.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum Color {
    Srgb,
    LinearSrgb,
}

/// The image the encoder receives: interleaved integer samples of
/// `bits_per_sample` bits, `u8` at 8 bits, native-endian `u16` scaled to
/// `0..=2^bits - 1` above (`JXL_BIT_DEPTH_FROM_CODESTREAM`).
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct ImageInfo {
    pub width: u32,
    pub height: u32,
    pub channels: ColorChannels,
    pub color: Color,
    pub alpha: Alpha,
    /// 8 to 16 (since 2026-11-15; before, always 8). Alpha has the same.
    pub bits_per_sample: u8,
}

impl ImageInfo {
    /// 8-bit RGB with enum sRGB color.
    #[must_use]
    pub fn srgb8(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            channels: ColorChannels::Rgb,
            color: Color::Srgb,
            alpha: Alpha::None,
            bits_per_sample: 8,
        }
    }

    /// The same image at `bits` bits per sample (8 to 16).
    #[must_use]
    pub fn with_bits(self, bits: u8) -> Self {
        Self {
            bits_per_sample: bits,
            ..self
        }
    }

    /// Bytes of one sample in the interleaved buffer: 1 at 8 bits, 2 above.
    fn sample_bytes(&self) -> usize {
        if self.bits_per_sample > 8 { 2 } else { 1 }
    }

    /// 8-bit RGB with straight 8-bit alpha (RGBA) and enum sRGB color.
    #[must_use]
    pub fn srgba8(width: u32, height: u32) -> Self {
        Self {
            alpha: Alpha::Straight8,
            ..Self::srgb8(width, height)
        }
    }

    fn color_channel_count(&self) -> u32 {
        match self.channels {
            ColorChannels::Gray => 1,
            ColorChannels::Rgb => 3,
        }
    }

    fn extra_channel_count(&self) -> u32 {
        match self.alpha {
            Alpha::None => 0,
            Alpha::Straight8 => 1,
        }
    }

    /// Interleaved channels per pixel, color then alpha.
    fn channel_count(&self) -> u32 {
        self.color_channel_count() + self.extra_channel_count()
    }

    fn dims(&self) -> Option<(usize, usize)> {
        Some((
            usize::try_from(self.width).ok()?,
            usize::try_from(self.height).ok()?,
        ))
    }

    /// Bytes of one interleaved frame.
    #[must_use]
    pub fn frame_bytes(&self) -> Option<usize> {
        let (width, height) = self.dims()?;
        width
            .checked_mul(height)?
            .checked_mul(self.channel_count() as usize)?
            .checked_mul(self.sample_bytes())
    }

    fn num_groups(&self) -> Option<usize> {
        let (width, height) = self.dims()?;
        width
            .div_ceil(GROUP_DIM)
            .checked_mul(height.div_ceil(GROUP_DIM))
    }

    /// Whether the port encodes this image on its streaming path (more than
    /// eight 256x256 groups), where it searches no patches.
    #[must_use]
    pub fn streaming(&self) -> bool {
        self.num_groups()
            .is_none_or(|groups| groups > MAX_ONE_SHOT_GROUPS)
    }
}

/// Per-frame settings. Every value the bridge sets on the port is applied in
/// [`FrameOptions::apply`]; a new option is one field and one entry there.
#[derive(Clone, Copy, Debug, PartialEq)]
#[non_exhaustive]
pub struct FrameOptions {
    /// Butteraugli distance: 1.0 is libjxl's default ("visually lossless"),
    /// lower is closer to the input. Admitted:
    /// [`MIN_DISTANCE`]`..`[`MAX_ADMITTED_DISTANCE`].
    pub distance: f32,
    /// libjxl's patch search (`JXL_ENC_FRAME_SETTING_PATCHES`). `false` (the
    /// bridge's default) sets the option to 0: the port skips the search and
    /// encodes every image, as `cjxl --patches=0` does. `true` leaves
    /// libjxl's default (-1): text-like content on the one-shot path then
    /// returns [`Admission::PatchDictionary`], because the patch
    /// reference-frame encode is not translated yet.
    pub patches: bool,
}

impl Default for FrameOptions {
    fn default() -> Self {
        Self {
            distance: DEFAULT_DISTANCE,
            patches: false,
        }
    }
}

impl FrameOptions {
    /// The given distance with patches off.
    #[must_use]
    pub fn with_distance(distance: f32) -> Self {
        Self {
            distance,
            ..Self::default()
        }
    }

    /// The same options with the patch search on (`true`, libjxl's default)
    /// or off (`false`).
    #[must_use]
    pub fn with_patches(self, patches: bool) -> Self {
        Self { patches, ..self }
    }

    /// Whether every value is inside the port's admitted profile.
    #[must_use]
    pub fn admitted(&self) -> bool {
        (MIN_DISTANCE..MAX_ADMITTED_DISTANCE).contains(&self.distance)
    }

    /// Set every option on the port's frame settings, in order.
    ///
    /// # Safety
    /// `settings` is a live frame-settings object of a live encoder, and no
    /// Rust reference to either is live during the call.
    unsafe fn apply(&self, settings: *mut JxlEncoderFrameSettings) -> Status {
        // Integer options: pinned so that a change of the port's defaults
        // cannot silently leave the admitted profile.
        // Patches: 0 is `Override::kOff`, -1 libjxl's default; the port's
        // FrameProfile gate admits exactly these two values.
        let patches = if self.patches { -1 } else { 0 };
        let integer_options: [(Id, i64); 2] = [(Id::EFFORT, EFFORT), (Id::PATCHES, patches)];
        for (id, value) in integer_options {
            // SAFETY: The caller's contract.
            let status = unsafe { api::jxl_encoder_frame_settings_set_option(settings, id, value) };
            if status != Status::SUCCESS {
                return status;
            }
        }
        // SAFETY: The caller's contract.
        unsafe { api::jxl_encoder_set_frame_distance(settings, self.distance) }
    }
}

/// Sole owner of an encoder object; destroys it on drop.
struct Raw(NonNull<JxlEncoder>);

impl Drop for Raw {
    fn drop(&mut self) {
        // SAFETY: This is the sole owner of a pointer returned by create.
        // Destruction also releases the frame settings, the queued frame copy
        // and the output wrapper's buffers; no settings pointer is used after.
        unsafe { api::jxl_encoder_destroy(self.0.as_ptr()) };
    }
}

/// One encoder with one frame settings object (the port's default memory
/// manager, serial, raw codestream output in buffer mode).
pub struct Encoder {
    raw: Raw,
    /// Owned by the encoder; valid until `raw` is dropped.
    settings: NonNull<JxlEncoderFrameSettings>,
    image: Option<ImageInfo>,
    options: Option<FrameOptions>,
    lossless: bool,
    frame_added: bool,
    input_closed: bool,
    finished: bool,
    failed: bool,
}

impl Encoder {
    /// Create an encoder and its frame settings.
    ///
    /// # Errors
    /// [`Error::OutOfMemory`] when either allocation fails.
    pub fn new() -> Result<Self, Error> {
        // SAFETY: A null manager selects the default allocator; the returned
        // pointer is uniquely owned by `Raw` and destroyed in its Drop.
        let raw = Raw(
            NonNull::new(unsafe { api::jxl_encoder_create(ptr::null()) })
                .ok_or(Error::OutOfMemory)?,
        );
        // SAFETY: The encoder is live and no reference to it exists; a null
        // source selects default values. The settings object is owned by the
        // encoder and released with it.
        let settings =
            unsafe { api::jxl_encoder_frame_settings_create(raw.0.as_ptr(), ptr::null()) };
        let settings = NonNull::new(settings).ok_or(Error::OutOfMemory)?;
        Ok(Self {
            raw,
            settings,
            image: None,
            options: None,
            lossless: false,
            frame_added: false,
            input_closed: false,
            finished: false,
            failed: false,
        })
    }

    fn enc(&mut self) -> &mut JxlEncoder {
        // SAFETY: The encoder is uniquely owned and live until Drop; the
        // returned borrow ends before any frame-settings call, which reaches
        // the encoder through its own retained pointer.
        unsafe { self.raw.0.as_mut() }
    }

    fn ready(&self) -> Result<(), Error> {
        if self.failed {
            return Err(Error::Misuse("the encoder already failed"));
        }
        if self.finished {
            return Err(Error::Misuse("the codestream is already complete"));
        }
        Ok(())
    }

    /// Record a failed call and classify the port's error code.
    fn fail(&mut self, stage: Stage) -> Error {
        self.failed = true;
        match api::jxl_encoder_get_error(self.enc()) {
            JxlEncoderError::NOT_SUPPORTED => Error::NotAdmitted(self.admission(stage)),
            JxlEncoderError::API_USAGE => Error::ApiUsage(stage),
            JxlEncoderError::OOM => Error::OutOfMemory,
            _ => Error::Failed(stage),
        }
    }

    /// The admission checkpoint behind `JXL_ENC_ERR_NOT_SUPPORTED`; see
    /// [`Admission`].
    fn admission(&self, stage: Stage) -> Admission {
        let patch_search = self
            .options
            .is_some_and(|options| options.admitted() && options.patches);
        let one_shot = self.image.is_some_and(|image| !image.streaming());
        if stage == Stage::ProcessOutput && patch_search && one_shot {
            Admission::PatchDictionary
        } else {
            Admission::FrameProfile
        }
    }

    /// Set the basic info (8-bit samples, XYB, orientation 1, and for
    /// [`Alpha::Straight8`] one straight 8-bit alpha extra channel) and the
    /// enum color encoding.
    ///
    /// # Errors
    /// [`Error::Misuse`] for a second call or an empty image;
    /// [`Error::ApiUsage`] when the port rejects the size or level.
    pub fn set_image(&mut self, image: ImageInfo) -> Result<(), Error> {
        self.set_image_with(image, false)
    }

    /// [`Encoder::set_image`] for a lossless frame: the same basic info with
    /// `uses_original_profile` (no XYB), which `JxlEncoderSetFrameLossless`
    /// requires.
    ///
    /// # Errors
    /// As for [`Encoder::set_image`].
    pub fn set_lossless_image(&mut self, image: ImageInfo) -> Result<(), Error> {
        self.set_image_with(image, true)
    }

    fn set_image_with(&mut self, image: ImageInfo, original_profile: bool) -> Result<(), Error> {
        self.ready()?;
        if self.image.is_some() {
            return Err(Error::Misuse("the image is already set"));
        }
        if image.width == 0 || image.height == 0 {
            return Err(Error::Misuse("the image must not be empty"));
        }
        if !(8..=16).contains(&image.bits_per_sample) {
            return Err(Error::Misuse("samples must have 8 to 16 bits"));
        }
        let mut basic = JxlBasicInfo::zeroed();
        api::jxl_encoder_init_basic_info(&mut basic);
        basic.xsize = image.width;
        basic.ysize = image.height;
        basic.bits_per_sample = u32::from(image.bits_per_sample);
        basic.exponent_bits_per_sample = 0;
        basic.uses_original_profile = i32::from(original_profile);
        basic.num_color_channels = image.color_channel_count();
        // SetBasicInfo derives the alpha extra channel's info (type alpha,
        // the color's bits, unassociated) from these fields, as libjxl does;
        // `cjxl` gives a PAM's alpha the color depth too.
        basic.num_extra_channels = image.extra_channel_count();
        basic.alpha_bits = u32::from(image.bits_per_sample) * image.extra_channel_count();
        basic.alpha_exponent_bits = 0;
        basic.alpha_premultiplied = 0;
        basic.orientation = JxlOrientation::IDENTITY;
        if api::jxl_encoder_set_basic_info(self.enc(), &basic) != Status::SUCCESS {
            return Err(self.fail(Stage::BasicInfo));
        }
        self.image = Some(image);
        let gray = i32::from(image.channels == ColorChannels::Gray);
        let mut color = JxlColorEncoding::default();
        match image.color {
            Color::Srgb => api::jxl_color_encoding_set_to_srgb(&mut color, gray),
            Color::LinearSrgb => api::jxl_color_encoding_set_to_linear_srgb(&mut color, gray),
        }
        if api::jxl_encoder_set_color_encoding(self.enc(), &color) != Status::SUCCESS {
            return Err(self.fail(Stage::ColorEncoding));
        }
        Ok(())
    }

    /// Apply the frame options.
    ///
    /// # Errors
    /// [`Error::Misuse`] for a non-finite or non-positive distance (zero
    /// would select lossless, which XYB excludes); [`Error::ApiUsage`] when
    /// the port rejects a value (a distance above 25). A distance outside
    /// the admitted range is accepted here and reported as
    /// [`Admission::FrameProfile`] when the frame is encoded.
    pub fn set_frame_options(&mut self, options: FrameOptions) -> Result<(), Error> {
        self.ready()?;
        if self.lossless {
            return Err(Error::Misuse("the lossless profile is already set"));
        }
        if !options.distance.is_finite() || options.distance <= 0.0 {
            return Err(Error::Misuse("the distance must be finite and positive"));
        }
        // SAFETY: The settings object and its encoder are live (owned by
        // `raw`), and `&mut self` guarantees no other reference to either.
        let status = unsafe { options.apply(self.settings.as_ptr()) };
        if status != Status::SUCCESS {
            return Err(self.fail(Stage::FrameSettings));
        }
        self.options = Some(options);
        Ok(())
    }

    /// Select the lossless profile: effort 2 ([`LOSSLESS_EFFORT`]) and
    /// `JxlEncoderSetFrameLossless`. Buffering stays at its default, so the
    /// port encodes more than eight groups on its streaming path, as the
    /// original does. Call after [`Encoder::set_lossless_image`].
    ///
    /// # Errors
    /// [`Error::Misuse`] before [`Encoder::set_lossless_image`] or after
    /// frame options; [`Error::ApiUsage`] when the port rejects a value (an
    /// XYB image from [`Encoder::set_image`]).
    pub fn set_lossless(&mut self) -> Result<(), Error> {
        self.ready()?;
        if self.image.is_none() {
            return Err(Error::Misuse("set the image before the lossless profile"));
        }
        if self.options.is_some() || self.lossless {
            return Err(Error::Misuse("frame options are already set"));
        }
        let settings = self.settings.as_ptr();
        // SAFETY: The settings object and its encoder are live (owned by
        // `raw`), and `&mut self` guarantees no other reference to either.
        let status = unsafe {
            api::jxl_encoder_frame_settings_set_option(settings, Id::EFFORT, LOSSLESS_EFFORT)
        };
        if status != Status::SUCCESS {
            return Err(self.fail(Stage::FrameSettings));
        }
        // SAFETY: As above.
        if unsafe { api::jxl_encoder_set_frame_lossless(settings, 1) } != Status::SUCCESS {
            return Err(self.fail(Stage::FrameSettings));
        }
        self.lossless = true;
        Ok(())
    }

    /// Queue the one frame. The port copies `pixels` before returning; the
    /// slice is not retained.
    ///
    /// # Errors
    /// [`Error::Misuse`] before [`Encoder::set_image`], for a second frame or
    /// a buffer that is not exactly one interleaved frame;
    /// [`Error::ApiUsage`] or [`Error::NotAdmitted`] from the port.
    pub fn add_frame(&mut self, pixels: &[u8]) -> Result<(), Error> {
        self.ready()?;
        let Some(image) = self.image else {
            return Err(Error::Misuse("set the image before adding a frame"));
        };
        if self.frame_added {
            return Err(Error::Misuse("only one frame is admitted"));
        }
        if Some(pixels.len()) != image.frame_bytes() {
            return Err(Error::Misuse(
                "the pixel buffer is not one interleaved frame",
            ));
        }
        let deep = image.bits_per_sample > 8;
        let format = JxlPixelFormat {
            num_channels: image.channel_count(),
            data_type: if deep {
                JxlDataType::UINT16
            } else {
                JxlDataType::UINT8
            },
            endianness: JxlEndianness::NATIVE,
            align: 0,
        };
        if deep {
            // The samples are codes of the codestream depth, as `cjxl` passes
            // a PNM (`JXL_BIT_DEPTH_FROM_CODESTREAM`); 8-bit input keeps the
            // default, the same values.
            let bit_depth = JxlBitDepth {
                depth_type: JxlBitDepthType::FROM_CODESTREAM,
                bits_per_sample: 0,
                exponent_bits_per_sample: 0,
            };
            // SAFETY: The settings object and its encoder are live and not
            // otherwise referenced.
            let status =
                unsafe { api::jxl_encoder_set_frame_bit_depth(self.settings.as_ptr(), &bit_depth) };
            if status != Status::SUCCESS {
                return Err(self.fail(Stage::AddFrame));
            }
        }
        // SAFETY: The settings object and its encoder are live and not
        // otherwise referenced. AddImageFrame copies the buffer
        // (CopyBuffers) before it returns, so `pixels` is not retained.
        let status = unsafe {
            api::jxl_encoder_add_image_frame(self.settings.as_ptr(), &format, Some(pixels))
        };
        if status != Status::SUCCESS {
            return Err(self.fail(Stage::AddFrame));
        }
        self.frame_added = true;
        Ok(())
    }

    /// Close frames and boxes; the queued frame becomes the last one.
    pub fn close_input(&mut self) {
        api::jxl_encoder_close_input(self.enc());
        self.input_closed = true;
    }

    /// Encode and append the complete codestream to `out`, at most `limit`
    /// bytes. Returns the number of bytes appended. The port keeps the
    /// encoded frame in its own buffer until it is copied out, so the limit
    /// bounds what reaches `out`, not the encoder's own output buffer.
    ///
    /// # Errors
    /// [`Error::Misuse`] before [`Encoder::close_input`];
    /// [`Error::OutputLimit`] when the codestream is longer than `limit`;
    /// [`Error::NotAdmitted`], [`Error::ApiUsage`], [`Error::Failed`] from
    /// the port; [`Error::OutOfMemory`]. On error `out` has its length at
    /// entry again and the encoder accepts no further calls.
    pub fn process_output_into(&mut self, out: &mut Vec<u8>, limit: usize) -> Result<usize, Error> {
        self.ready()?;
        if !self.input_closed {
            return Err(Error::Misuse("close the input before processing output"));
        }
        let start = out.len();
        let result = self.drain(out, start, limit);
        if result.is_err() {
            out.truncate(start);
        } else {
            self.finished = true;
        }
        result
    }

    fn drain(&mut self, out: &mut Vec<u8>, start: usize, limit: usize) -> Result<usize, Error> {
        const FIRST_CHUNK: usize = 64 * 1024;
        loop {
            let written = out.len() - start;
            let remaining = limit - written;
            // Grow geometrically but never past the limit; a zero-sized
            // window lets the port report completion at exactly the limit.
            let room = remaining.min(written.max(FIRST_CHUNK));
            if out.try_reserve(room).is_err() {
                self.failed = true;
                return Err(Error::OutOfMemory);
            }
            let mut next_out = out.spare_capacity_mut().as_mut_ptr().cast::<u8>();
            let mut avail_out = room;
            // SAFETY: `next_out` points to at least `room` writable bytes of
            // spare capacity, and no Rust reference to them or to the two
            // locals is live during the call. The encoder retains both
            // pointers but writes through them only during a later
            // ProcessOutput call, which replaces them first.
            let status = unsafe {
                api::jxl_encoder_process_output(self.enc(), &raw mut next_out, &raw mut avail_out)
            };
            let produced = room - avail_out;
            // SAFETY: The encoder initialized exactly the first `produced`
            // bytes of the spare capacity (it advanced `next_out` by them).
            unsafe { out.set_len(out.len() + produced) };
            match status {
                Status::SUCCESS => return Ok(out.len() - start),
                Status::NEED_MORE_OUTPUT if room == 0 => {
                    self.failed = true;
                    return Err(Error::OutputLimit { limit });
                }
                Status::NEED_MORE_OUTPUT => {}
                _ => return Err(self.fail(Stage::ProcessOutput)),
            }
        }
    }
}

/// Encode one frame: set the image and options, add `pixels`, close the
/// input and append the codestream to `out` (at most `limit` bytes).
///
/// # Errors
/// As for the [`Encoder`] steps.
pub fn encode(
    image: ImageInfo,
    options: FrameOptions,
    pixels: &[u8],
    out: &mut Vec<u8>,
    limit: usize,
) -> Result<usize, Error> {
    let mut encoder = Encoder::new()?;
    encoder.set_image(image)?;
    encoder.set_frame_options(options)?;
    encoder.add_frame(pixels)?;
    encoder.close_input();
    encoder.process_output_into(out, limit)
}

/// Encode one lossless frame (effort 2, the default buffering): set the
/// image, the lossless profile, add `pixels`, close the input and append the
/// codestream to `out` (at most `limit` bytes).
///
/// # Errors
/// As for the [`Encoder`] steps.
pub fn encode_lossless(
    image: ImageInfo,
    pixels: &[u8],
    out: &mut Vec<u8>,
    limit: usize,
) -> Result<usize, Error> {
    let mut encoder = Encoder::new()?;
    encoder.set_lossless_image(image)?;
    encoder.set_lossless()?;
    encoder.add_frame(pixels)?;
    encoder.close_input();
    encoder.process_output_into(out, limit)
}

/// An estimate, not a bound, of the encoder's large buffers alive at the
/// same time for `image` in the admitted profile, from the port's source:
///
/// - the queued frame's copy of the input (AddImageFrame's CopyBuffers),
///   alpha included;
/// - one-shot path: the `Image3<f32>` color frame padded to 8x8 blocks, the
///   pre-Gaborish `orig_opsin` copy of the same size, the 1x1 masking plane
///   of the adaptive quantization, and the `i32` AC coefficients of every
///   group (3 x 256 x 256 per group);
/// - streaming path: the same per-group buffers for one group with its
///   8-pixel border, plus the DC-resolution planes of the whole frame
///   (about seven `f32`/`i32` planes per 8x8 block);
/// - with alpha, 12 bytes per pixel of the encoded area (the whole frame
///   one-shot, one bordered group streaming): the `f32` extra-channel plane
///   of the VarDCT ComputeEncodingData (`extra_channels`), the Modular
///   encoder's global `i32` channel (`compute_encoding_data_extra_channels`)
///   and its `i32` per-group copies (`prepare_stream_params`), which exist
///   at the same time.
///
/// Tokens, histograms, section writers, per-block heuristics state and the
/// encoded output are not counted.
#[must_use]
pub fn estimated_codec_bytes(image: &ImageInfo) -> Option<usize> {
    let (width, height) = image.dims()?;
    let input = image.frame_bytes()?;
    let block = |n: usize| n.checked_next_multiple_of(BLOCK_DIM);
    let coefficients_per_group = 3 * GROUP_DIM * GROUP_DIM * 4;
    let planes = |w: usize, h: usize| -> Option<usize> {
        // Two Image3<f32> and one f32 plane: 7 f32 samples per pixel.
        block(w)?.checked_mul(block(h)?)?.checked_mul(7 * 4)
    };
    // Per pixel of the encoded area: one f32 plane and two i32 copies.
    let extra = image.extra_channel_count() as usize * 3 * 4;
    let codec = if image.streaming() {
        let border = GROUP_DIM + 2 * BLOCK_DIM;
        let blocks = width
            .div_ceil(BLOCK_DIM)
            .checked_mul(height.div_ceil(BLOCK_DIM))?;
        planes(border, border)?
            .checked_add(coefficients_per_group)?
            .checked_add(blocks.checked_mul(7 * 4)?)?
            .checked_add(border * border * extra)?
    } else {
        planes(width, height)?
            .checked_add(image.num_groups()?.checked_mul(coefficients_per_group)?)?
            .checked_add(width.checked_mul(height)?.checked_mul(extra)?)?
    };
    input.checked_add(codec)
}

#[cfg(test)]
mod tests;
