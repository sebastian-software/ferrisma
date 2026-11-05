// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! The exported C ABI of `lib/include/jxl/decode.h`: every `JXL_EXPORT`
//! function that `lib/jxl/decode.cc` defines, under its original unmangled
//! name, forwarding to the Rust port in [`super::decode`].
//!
//! The wrappers translate only representation: raw C pointers become the Rust
//! references and options of the ported functions, which perform all of the
//! original's argument checks in its order. Structs crossing the boundary are
//! the `#[repr(C)]` header types; their layouts are asserted below and
//! compared with the pinned headers by the test oracle.
//!
//! Language-required deviations, each local to its representation:
//! - The source dereferences `dec` without a check. A null decoder therefore
//!   panics, which aborts at the C boundary, instead of being undefined. The
//!   same holds for every other pointer that the source dereferences: the
//!   ported function receives it as an option and panics only where the
//!   source dereferences it, so paths that return before that point keep
//!   their original status for a null pointer.
//! - An exported call borrows its decoder exclusively, or shared for a `const
//!   JxlDecoder*` getter, for the whole call. Reentering the same decoder from
//!   one of its callbacks on the calling thread would alias that borrow, so it
//!   aborts. The original permits some reentrant getter calls; Rust's
//!   aliasing rules cannot express them for a decoder that is being mutated.
//! - A panic, which the source does not have, cannot unwind into C: the
//!   `extern "C"` boundary aborts the process.
//!
//! The runner functions of `thread_parallel_runner.h` and
//! `resizable_parallel_runner.h`, which the original exports from its
//! separate `jxl_threads` library, are exported by `crate::threads::abi`.

use super::base::data_parallel::JxlParallelRunner;
use super::cms_interface::JxlCmsInterface;
use super::codestream_header::{
    JxlAnimationHeader, JxlBasicInfo, JxlBlendInfo, JxlExtraChannelInfo, JxlFrameHeader,
    JxlHeaderExtensions, JxlLayerInfo, JxlPreviewHeader,
};
use super::color_encoding::JxlColorEncoding;
use super::dec_cache::{ImageOutDestroyCallback, ImageOutInitCallback, ImageOutRunCallback};
use super::decode::{
    self, JxlColorProfileTarget, JxlDecoder, JxlDecoderStatus, JxlImageOutCallback,
    JxlProgressiveDetail, JxlSignature,
};
use super::memory_manager::JxlMemoryManager;
use super::types::{JxlBitDepth, JxlPixelFormat};
use std::cell::RefCell;
use std::ffi::{c_char, c_int, c_void};
use std::mem::{align_of, offset_of, size_of};

// ---------------------------------------------------------------------------
// Layouts of the public structs on the supported 64-bit targets, as measured
// from the pinned headers by the test oracle.

macro_rules! assert_c_layout {
    ($ty:ty, $size:expr, $align:expr $(, $($field:ident).+ = $offset:expr)* $(,)?) => {
        const _: () = {
            assert!(size_of::<$ty>() == $size);
            assert!(align_of::<$ty>() == $align);
            $(assert!(offset_of!($ty, $($field).+) == $offset);)*
        };
    };
}

#[cfg(target_pointer_width = "64")]
mod layouts {
    use super::*;

    assert_c_layout!(JxlPreviewHeader, 8, 4, xsize = 0, ysize = 4);
    assert_c_layout!(
        JxlAnimationHeader,
        16,
        4,
        tps_numerator = 0,
        tps_denominator = 4,
        num_loops = 8,
        have_timecodes = 12,
    );
    assert_c_layout!(
        JxlBasicInfo,
        204,
        4,
        have_container = 0,
        xsize = 4,
        ysize = 8,
        bits_per_sample = 12,
        exponent_bits_per_sample = 16,
        intensity_target = 20,
        min_nits = 24,
        relative_to_max_display = 28,
        linear_below = 32,
        uses_original_profile = 36,
        have_preview = 40,
        have_animation = 44,
        orientation = 48,
        num_color_channels = 52,
        num_extra_channels = 56,
        alpha_bits = 60,
        alpha_exponent_bits = 64,
        alpha_premultiplied = 68,
        preview = 72,
        animation = 80,
        intrinsic_xsize = 96,
        intrinsic_ysize = 100,
        padding = 104,
    );
    assert_c_layout!(
        JxlExtraChannelInfo,
        44,
        4,
        channel_type = 0,
        bits_per_sample = 4,
        exponent_bits_per_sample = 8,
        dim_shift = 12,
        name_length = 16,
        alpha_premultiplied = 20,
        spot_color = 24,
        cfa_channel = 40,
    );
    assert_c_layout!(JxlHeaderExtensions, 8, 8, extensions = 0);
    assert_c_layout!(
        JxlBlendInfo,
        16,
        4,
        blendmode = 0,
        source = 4,
        alpha = 8,
        clamp = 12
    );
    assert_c_layout!(
        JxlLayerInfo,
        40,
        4,
        have_crop = 0,
        crop_x0 = 4,
        crop_y0 = 8,
        xsize = 12,
        ysize = 16,
        blend_info = 20,
        save_as_reference = 36,
    );
    assert_c_layout!(
        JxlFrameHeader,
        56,
        4,
        duration = 0,
        timecode = 4,
        name_length = 8,
        is_last = 12,
        layer_info = 16,
    );
    assert_c_layout!(
        JxlColorEncoding,
        104,
        8,
        color_space = 0,
        white_point = 4,
        white_point_xy = 8,
        primaries = 24,
        primaries_red_xy = 32,
        primaries_green_xy = 48,
        primaries_blue_xy = 64,
        transfer_function = 80,
        gamma = 88,
        rendering_intent = 96,
    );
    assert_c_layout!(
        JxlPixelFormat,
        24,
        8,
        num_channels = 0,
        data_type = 4,
        endianness = 8,
        align = 16,
    );
    assert_c_layout!(
        JxlBitDepth,
        12,
        4,
        depth_type = 0,
        bits_per_sample = 4,
        exponent_bits_per_sample = 8,
    );
    assert_c_layout!(JxlMemoryManager, 24, 8, opaque = 0, alloc = 8, free = 16);
    assert_c_layout!(
        super::super::cms_interface::JxlColorProfile,
        128,
        8,
        icc.data = 0,
        icc.size = 8,
        color_encoding = 16,
        num_channels = 120,
    );
    assert_c_layout!(
        JxlCmsInterface,
        64,
        8,
        set_fields_data = 0,
        set_fields_from_icc = 8,
        init_data = 16,
        init = 24,
        get_src_buf = 32,
        get_dst_buf = 40,
        run = 48,
        destroy = 56,
    );
    assert_c_layout!(super::super::decode::JxlBoxType, 4, 1);
    // Every public enum is a 32-bit integer carrier.
    assert_c_layout!(JxlDecoderStatus, 4, 4);
    assert_c_layout!(JxlSignature, 4, 4);
    assert_c_layout!(JxlColorProfileTarget, 4, 4);
    assert_c_layout!(JxlProgressiveDetail, 4, 4);
}

// ---------------------------------------------------------------------------
// Decoder borrows.

thread_local! {
    /// The decoders whose exported calls are active on this thread.
    static ACTIVE_DECODERS: RefCell<Vec<usize>> = const { RefCell::new(Vec::new()) };
}

/// Marks a decoder as borrowed by an exported call on this thread.
struct ActiveDecoder(usize);

impl ActiveDecoder {
    fn enter(dec: *const JxlDecoder) -> Self {
        assert!(!dec.is_null(), "the source dereferences dec");
        let address = dec as usize;
        ACTIVE_DECODERS.with(|active| {
            let mut active = active.borrow_mut();
            assert!(
                !active.contains(&address),
                "a callback reentered the decoder that invoked it"
            );
            active.push(address);
        });
        Self(address)
    }
}

impl Drop for ActiveDecoder {
    fn drop(&mut self) {
        ACTIVE_DECODERS.with(|active| {
            let mut active = active.borrow_mut();
            let last = active.pop();
            debug_assert_eq!(last, Some(self.0));
        });
    }
}

/// Runs `f` with the exclusively borrowed decoder.
///
/// # Safety
/// `dec` is null or a live decoder from JxlDecoderCreate that no other
/// thread uses during the call.
unsafe fn with_decoder<R>(dec: *mut JxlDecoder, f: impl FnOnce(&mut JxlDecoder) -> R) -> R {
    let _active = ActiveDecoder::enter(dec);
    // SAFETY: The caller passes a live, non-null (checked above) decoder that
    // no other thread uses. ActiveDecoder excludes a reentrant call on this
    // thread, so no other reference to the decoder exists while `f` runs.
    f(unsafe { &mut *dec })
}

/// Runs `f` with the shared decoder of a `const JxlDecoder*` getter.
///
/// # Safety
/// As for [`with_decoder`].
unsafe fn with_decoder_ref<R>(dec: *const JxlDecoder, f: impl FnOnce(&JxlDecoder) -> R) -> R {
    let _active = ActiveDecoder::enter(dec);
    // SAFETY: As in with_decoder: the decoder is live and non-null, and no
    // exclusive borrow of it is active on this or another thread.
    f(unsafe { &*dec })
}

// ---------------------------------------------------------------------------
// Exported functions, in the order of decode.h.

/// JxlDecoderVersion.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub extern "C" fn JxlDecoderVersion() -> u32 {
    decode::jxl_decoder_version()
}

/// JxlSignatureCheck.
///
/// # Safety
/// When `len` is nonzero, `buf` is readable for `len` initialized bytes.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlSignatureCheck(buf: *const u8, len: usize) -> JxlSignature {
    // ReadSignature returns before reading buf when len is zero.
    let buf = if len == 0 {
        &[][..]
    } else {
        assert!(!buf.is_null(), "the source dereferences buf");
        // SAFETY: The caller supplies len readable initialized bytes at the
        // non-null buf; the call only reads them.
        unsafe { std::slice::from_raw_parts(buf, len) }
    };
    decode::jxl_signature_check(buf)
}

/// JxlDecoderCreate.
///
/// # Safety
/// A non-null `memory_manager` is readable and its callbacks obey
/// memory_manager.h for the decoder's lifetime, never unwind and never
/// reenter the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderCreate(
    memory_manager: *const JxlMemoryManager,
) -> *mut JxlDecoder {
    // SAFETY: Forwards the caller's memory manager contract.
    unsafe { decode::jxl_decoder_create(memory_manager) }
}

/// JxlDecoderReset.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderReset(dec: *mut JxlDecoder) {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder(dec, decode::jxl_decoder_reset) }
}

/// JxlDecoderDestroy.
///
/// # Safety
/// `dec` is null or a live decoder from JxlDecoderCreate that is not used
/// again.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderDestroy(dec: *mut JxlDecoder) {
    if dec.is_null() {
        return;
    }
    // The memory manager's free callback runs during destruction and must not
    // reenter the decoder either.
    let _active = ActiveDecoder::enter(dec);
    // SAFETY: The caller transfers the live decoder, which no other reference
    // aliases (see ActiveDecoder).
    unsafe { decode::jxl_decoder_destroy(dec) }
}

/// JxlDecoderRewind.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderRewind(dec: *mut JxlDecoder) {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder(dec, decode::jxl_decoder_rewind) }
}

/// JxlDecoderSkipFrames.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSkipFrames(dec: *mut JxlDecoder, amount: usize) {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder(dec, |dec| decode::jxl_decoder_skip_frames(dec, amount)) }
}

/// JxlDecoderSkipCurrentFrame.
///
/// # Safety
/// `dec` is a live decoder whose current input obeys JxlDecoderSetInput.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSkipCurrentFrame(dec: *mut JxlDecoder) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and input contracts.
    unsafe { with_decoder(dec, |dec| decode::jxl_decoder_skip_current_frame(dec)) }
}

/// JxlDecoderSetParallelRunner.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate. A non-null
/// `parallel_runner` and `parallel_runner_opaque` obey parallel_runner.h until
/// the decoder is reset, destroyed or given another runner: the runner calls
/// the supplied init and data functions only with the supplied opaque value,
/// only during that runner call, and never unwinds.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetParallelRunner(
    dec: *mut JxlDecoder,
    parallel_runner: Option<JxlParallelRunner>,
    parallel_runner_opaque: *mut c_void,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and runner contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_parallel_runner(dec, parallel_runner, parallel_runner_opaque)
        })
    }
}

/// JxlDecoderSizeHintBasicInfo.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSizeHintBasicInfo(dec: *const JxlDecoder) -> usize {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder_ref(dec, decode::jxl_decoder_size_hint_basic_info) }
}

/// JxlDecoderSubscribeEvents.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSubscribeEvents(
    dec: *mut JxlDecoder,
    events_wanted: c_int,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_subscribe_events(dec, events_wanted)
        })
    }
}

/// JxlDecoderSetKeepOrientation.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetKeepOrientation(
    dec: *mut JxlDecoder,
    skip_reorientation: c_int,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_keep_orientation(dec, skip_reorientation)
        })
    }
}

/// JxlDecoderSetUnpremultiplyAlpha.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetUnpremultiplyAlpha(
    dec: *mut JxlDecoder,
    unpremul_alpha: c_int,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_unpremultiply_alpha(dec, unpremul_alpha)
        })
    }
}

/// JxlDecoderSetRenderSpotcolors.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetRenderSpotcolors(
    dec: *mut JxlDecoder,
    render_spotcolors: c_int,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_render_spotcolors(dec, render_spotcolors)
        })
    }
}

/// JxlDecoderSetCoalescing.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetCoalescing(
    dec: *mut JxlDecoder,
    coalescing: c_int,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_coalescing(dec, coalescing)
        })
    }
}

/// JxlDecoderProcessInput.
///
/// # Safety
/// `dec` is a live decoder. Its current input obeys JxlDecoderSetInput, and
/// every output buffer, callback and CMS registered with it remains valid for
/// its documented use. Callbacks never unwind or reenter the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderProcessInput(dec: *mut JxlDecoder) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, input, buffer and callback contracts.
    unsafe { with_decoder(dec, |dec| decode::jxl_decoder_process_input(dec)) }
}

/// JxlDecoderSetInput.
///
/// # Safety
/// `dec` is a live decoder. `data` and `size` describe a live initialized
/// input range that stays valid and unmodified until JxlDecoderReleaseInput.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetInput(
    dec: *mut JxlDecoder,
    data: *const u8,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and retained input contracts.
    unsafe { with_decoder(dec, |dec| decode::jxl_decoder_set_input(dec, data, size)) }
}

/// JxlDecoderReleaseInput.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderReleaseInput(dec: *mut JxlDecoder) -> usize {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder(dec, decode::jxl_decoder_release_input) }
}

/// JxlDecoderCloseInput.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderCloseInput(dec: *mut JxlDecoder) {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder(dec, decode::jxl_decoder_close_input) }
}

/// JxlDecoderGetBasicInfo.
///
/// # Safety
/// `dec` is a live decoder. A non-null `info` is writable for a JxlBasicInfo
/// and disjoint from the decoder. Only writes reach it, so its prior contents
/// may be uninitialized.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetBasicInfo(
    dec: *const JxlDecoder,
    info: *mut JxlBasicInfo,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_basic_info(dec, info.as_mut())
        })
    }
}

/// JxlDecoderGetExtraChannelInfo.
///
/// # Safety
/// As for JxlDecoderGetBasicInfo, with `info` a JxlExtraChannelInfo.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetExtraChannelInfo(
    dec: *const JxlDecoder,
    index: usize,
    info: *mut JxlExtraChannelInfo,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null, which the ported function rejects only where the source
    // dereferences it.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_extra_channel_info(dec, index, info.as_mut())
        })
    }
}

/// JxlDecoderGetExtraChannelName.
///
/// # Safety
/// `dec` is a live decoder. `name` is writable for `size` bytes whenever the
/// name and its terminator fit, and disjoint from the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetExtraChannelName(
    dec: *const JxlDecoder,
    index: usize,
    name: *mut c_char,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output buffer contracts.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_extra_channel_name(dec, index, name.cast(), size)
        })
    }
}

/// JxlDecoderGetColorAsEncodedProfile.
///
/// # Safety
/// As for JxlDecoderGetBasicInfo, with `color_encoding` a JxlColorEncoding.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetColorAsEncodedProfile(
    dec: *const JxlDecoder,
    target: JxlColorProfileTarget,
    color_encoding: *mut JxlColorEncoding,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_color_as_encoded_profile(dec, target, color_encoding.as_mut())
        })
    }
}

/// JxlDecoderGetICCProfileSize.
///
/// # Safety
/// As for JxlDecoderGetBasicInfo, with `size` a size_t.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetICCProfileSize(
    dec: *const JxlDecoder,
    target: JxlColorProfileTarget,
    size: *mut usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_icc_profile_size(dec, target, size.as_mut())
        })
    }
}

/// JxlDecoderGetColorAsICCProfile.
///
/// # Safety
/// `dec` is a live decoder. `icc_profile` is writable for `size` bytes and
/// disjoint from the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetColorAsICCProfile(
    dec: *const JxlDecoder,
    target: JxlColorProfileTarget,
    icc_profile: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output buffer contracts.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_color_as_icc_profile(dec, target, icc_profile, size)
        })
    }
}

/// JxlDecoderSetPreferredColorProfile.
///
/// # Safety
/// `dec` is a live decoder. A non-null `color_encoding` is a readable,
/// initialized JxlColorEncoding.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetPreferredColorProfile(
    dec: *mut JxlDecoder,
    color_encoding: *const JxlColorEncoding,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and input contracts; as_ref yields None
    // for null.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_preferred_color_profile(dec, color_encoding.as_ref())
        })
    }
}

/// JxlDecoderSetDesiredIntensityTarget.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetDesiredIntensityTarget(
    dec: *mut JxlDecoder,
    desired_intensity_target: f32,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_desired_intensity_target(dec, desired_intensity_target)
        })
    }
}

/// JxlDecoderSetOutputColorProfile.
///
/// # Safety
/// As for JxlDecoderSetPreferredColorProfile. A non-null `icc_data` is
/// readable for `icc_size` initialized bytes, and the CMS set with
/// JxlDecoderSetCms obeys cms_interface.h when it is invoked.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetOutputColorProfile(
    dec: *mut JxlDecoder,
    color_encoding: *const JxlColorEncoding,
    icc_data: *const u8,
    icc_size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, input and CMS contracts; as_ref yields
    // None for null.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_output_color_profile(
                dec,
                color_encoding.as_ref(),
                icc_data,
                icc_size,
            )
        })
    }
}

/// JxlDecoderSetCms.
///
/// # Safety
/// `dec` is a live decoder. Every callback and opaque pointer of `cms` obeys
/// cms_interface.h whenever the decoder invokes it.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetCms(
    dec: *mut JxlDecoder,
    cms: JxlCmsInterface,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and CMS contracts.
    unsafe { with_decoder(dec, |dec| decode::jxl_decoder_set_cms(dec, cms)) }
}

/// JxlDecoderPreviewOutBufferSize.
///
/// # Safety
/// `dec` is a live decoder. A non-null `format` is a readable, initialized
/// JxlPixelFormat; a non-null `size` is writable for a size_t.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderPreviewOutBufferSize(
    dec: *const JxlDecoder,
    format: *const JxlPixelFormat,
    size: *mut usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, input and output contracts; null pointers
    // become None and are rejected only where the source dereferences them.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_preview_out_buffer_size(dec, format.as_ref(), size.as_mut())
        })
    }
}

/// JxlDecoderSetPreviewOutBuffer.
///
/// # Safety
/// `dec` is a live decoder. A non-null `format` is a readable, initialized
/// JxlPixelFormat. `buffer` is writable for `size` bytes until the preview
/// event or until the decoder releases it, and disjoint from the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetPreviewOutBuffer(
    dec: *mut JxlDecoder,
    format: *const JxlPixelFormat,
    buffer: *mut c_void,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, format and output buffer contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_preview_out_buffer(dec, format.as_ref(), buffer, size)
        })
    }
}

/// JxlDecoderGetFrameHeader.
///
/// # Safety
/// As for JxlDecoderGetBasicInfo, with `header` a JxlFrameHeader.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetFrameHeader(
    dec: *const JxlDecoder,
    header: *mut JxlFrameHeader,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_frame_header(dec, header.as_mut())
        })
    }
}

/// JxlDecoderGetFrameName.
///
/// # Safety
/// `dec` is a live decoder. `name` is writable for `size` bytes whenever the
/// name and its terminator fit, and disjoint from the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetFrameName(
    dec: *const JxlDecoder,
    name: *mut c_char,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output buffer contracts.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_frame_name(dec, name.cast(), size)
        })
    }
}

/// JxlDecoderGetExtraChannelBlendInfo.
///
/// # Safety
/// As for JxlDecoderGetBasicInfo, with `blend_info` a JxlBlendInfo.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetExtraChannelBlendInfo(
    dec: *const JxlDecoder,
    index: usize,
    blend_info: *mut JxlBlendInfo,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_extra_channel_blend_info(dec, index, blend_info.as_mut())
        })
    }
}

/// JxlDecoderImageOutBufferSize.
///
/// # Safety
/// As for JxlDecoderPreviewOutBufferSize.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderImageOutBufferSize(
    dec: *const JxlDecoder,
    format: *const JxlPixelFormat,
    size: *mut usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, input and output contracts; null pointers
    // become None and are rejected only where the source dereferences them.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_image_out_buffer_size(dec, format.as_ref(), size.as_mut())
        })
    }
}

/// JxlDecoderSetImageOutBuffer.
///
/// # Safety
/// As for JxlDecoderSetPreviewOutBuffer, until the full-image event.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetImageOutBuffer(
    dec: *mut JxlDecoder,
    format: *const JxlPixelFormat,
    buffer: *mut c_void,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, format and output buffer contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_image_out_buffer(dec, format.as_ref(), buffer, size)
        })
    }
}

/// JxlDecoderSetImageOutCallback.
///
/// # Safety
/// `dec` is a live decoder and a non-null `format` is a readable, initialized
/// JxlPixelFormat. The callback and `opaque` obey decode.h until the
/// full-image event and never unwind or reenter the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetImageOutCallback(
    dec: *mut JxlDecoder,
    format: *const JxlPixelFormat,
    callback: Option<JxlImageOutCallback>,
    opaque: *mut c_void,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, format and callback contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_image_out_callback(dec, format.as_ref(), callback, opaque)
        })
    }
}

/// JxlDecoderSetMultithreadedImageOutCallback.
///
/// # Safety
/// As for JxlDecoderSetImageOutCallback, for all three callbacks and
/// `init_opaque`.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetMultithreadedImageOutCallback(
    dec: *mut JxlDecoder,
    format: *const JxlPixelFormat,
    init_callback: Option<ImageOutInitCallback>,
    run_callback: Option<ImageOutRunCallback>,
    destroy_callback: Option<ImageOutDestroyCallback>,
    init_opaque: *mut c_void,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, format and callback contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_multithreaded_image_out_callback(
                dec,
                format.as_ref(),
                init_callback,
                run_callback,
                destroy_callback,
                init_opaque,
            )
        })
    }
}

/// JxlDecoderExtraChannelBufferSize.
///
/// # Safety
/// As for JxlDecoderPreviewOutBufferSize.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderExtraChannelBufferSize(
    dec: *const JxlDecoder,
    format: *const JxlPixelFormat,
    size: *mut usize,
    index: u32,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, input and output contracts; null pointers
    // become None and are rejected only where the source dereferences them.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_extra_channel_buffer_size(
                dec,
                format.as_ref(),
                size.as_mut(),
                index,
            )
        })
    }
}

/// JxlDecoderSetExtraChannelBuffer.
///
/// # Safety
/// As for JxlDecoderSetImageOutBuffer.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetExtraChannelBuffer(
    dec: *mut JxlDecoder,
    format: *const JxlPixelFormat,
    buffer: *mut c_void,
    size: usize,
    index: u32,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, format and output buffer contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_extra_channel_buffer(dec, format.as_ref(), buffer, size, index)
        })
    }
}

/// JxlDecoderSetJPEGBuffer.
///
/// # Safety
/// `dec` is a live decoder. When `size` is nonzero, `data` is writable for
/// `size` bytes until JxlDecoderReleaseJPEGBuffer and disjoint from the
/// decoder and its input.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetJPEGBuffer(
    dec: *mut JxlDecoder,
    data: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output buffer contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_jpeg_buffer(dec, data, size)
        })
    }
}

/// JxlDecoderReleaseJPEGBuffer.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderReleaseJPEGBuffer(dec: *mut JxlDecoder) -> usize {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder(dec, decode::jxl_decoder_release_jpeg_buffer) }
}

/// JxlDecoderSetBoxBuffer.
///
/// # Safety
/// `dec` is a live decoder. `data` is writable for `size` bytes until
/// JxlDecoderReleaseBoxBuffer and disjoint from the decoder and its input.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetBoxBuffer(
    dec: *mut JxlDecoder,
    data: *mut u8,
    size: usize,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output buffer contracts.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_box_buffer(dec, data, size)
        })
    }
}

/// JxlDecoderReleaseBoxBuffer.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderReleaseBoxBuffer(dec: *mut JxlDecoder) -> usize {
    // SAFETY: The caller's decoder contract.
    unsafe { with_decoder(dec, decode::jxl_decoder_release_box_buffer) }
}

/// JxlDecoderSetDecompressBoxes.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetDecompressBoxes(
    dec: *mut JxlDecoder,
    decompress: c_int,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_decompress_boxes(dec, decompress)
        })
    }
}

/// JxlDecoderGetBoxType. The JxlBoxType array parameter is a pointer to its
/// first byte, as in C.
///
/// # Safety
/// `dec` is a live decoder. A non-null `box_type` is writable for four bytes
/// and disjoint from the decoder.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetBoxType(
    dec: *mut JxlDecoder,
    box_type: *mut c_char,
    decompressed: c_int,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts. JxlBoxType is four
    // bytes with alignment one, so the cast pointer is aligned; as_mut yields
    // None for null.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_get_box_type(
                dec,
                box_type.cast::<decode::JxlBoxType>().as_mut(),
                decompressed,
            )
        })
    }
}

/// JxlDecoderGetBoxSizeRaw.
///
/// # Safety
/// As for JxlDecoderGetBasicInfo, with `size` a uint64_t.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetBoxSizeRaw(
    dec: *const JxlDecoder,
    size: *mut u64,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_box_size_raw(dec, size.as_mut())
        })
    }
}

/// JxlDecoderGetBoxSizeContents.
///
/// # Safety
/// As for JxlDecoderGetBasicInfo, with `size` a uint64_t.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetBoxSizeContents(
    dec: *const JxlDecoder,
    size: *mut u64,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and output contracts; as_mut yields None
    // for null.
    unsafe {
        with_decoder_ref(dec, |dec| {
            decode::jxl_decoder_get_box_size_contents(dec, size.as_mut())
        })
    }
}

/// JxlDecoderSetProgressiveDetail.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetProgressiveDetail(
    dec: *mut JxlDecoder,
    detail: JxlProgressiveDetail,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_progressive_detail(dec, detail)
        })
    }
}

/// JxlDecoderGetIntendedDownsamplingRatio.
///
/// # Safety
/// `dec` is a live decoder from JxlDecoderCreate.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderGetIntendedDownsamplingRatio(dec: *mut JxlDecoder) -> usize {
    // SAFETY: The caller's decoder contract.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_get_intended_downsampling_ratio(dec)
        })
    }
}

/// JxlDecoderFlushImage.
///
/// # Safety
/// `dec` is a live decoder whose registered output buffers and callbacks
/// obey their contracts.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderFlushImage(dec: *mut JxlDecoder) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder, buffer and callback contracts.
    unsafe { with_decoder(dec, |dec| decode::jxl_decoder_flush_image(dec)) }
}

/// JxlDecoderSetImageOutBitDepth.
///
/// # Safety
/// `dec` is a live decoder. A non-null `bit_depth` is a readable, initialized
/// JxlBitDepth.
#[unsafe(no_mangle)] // SAFETY: The only definition of this decode.h symbol.
pub unsafe extern "C" fn JxlDecoderSetImageOutBitDepth(
    dec: *mut JxlDecoder,
    bit_depth: *const JxlBitDepth,
) -> JxlDecoderStatus {
    // SAFETY: The caller's decoder and input contracts; as_ref yields None
    // for null.
    unsafe {
        with_decoder(dec, |dec| {
            decode::jxl_decoder_set_image_out_bit_depth(dec, bit_depth.as_ref())
        })
    }
}
