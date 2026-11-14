// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! The decoder's dispatch selection is scoped to decoding (port ADR 0004,
//! "Scope of the selection").
//!
//! The original has one process-wide `HWY_DYNAMIC_DISPATCH`; the port's
//! encoder is the scalar translation only (its Highway exports are not
//! ported), and its lossy encoder runs decoder code in its own loop (the AR
//! heuristics' reconstruction with `DecodeGroupForRoundtrip` and the render
//! pipeline, `DequantMatrices`, the modular inverse transforms). These tests
//! prove both halves of the requirement in one process:
//!
//! - every encoder entry point produces exactly the bytes it produces with
//!   the scalar path forced process-wide (today's selection), before, after,
//!   during (on another thread) and inside (from a decoder callback) a
//!   decode;
//! - every decoder entry point (the public decoder, with and without a
//!   thread runner, and the internal whole-decode helpers) runs the target
//!   the decoder selects (`decoder_target_for(DECODER_SELECTS_SIMD)`: NEON on
//!   AArch64 once the default is on, the scalar path under the scalar gate's
//!   `JXL_DECODER_TARGET=scalar`), with that target's row padding, and gives
//!   exactly the output of a decode with that target forced.
//!
//! Outside every entry point the selection is the scalar path.

use jxl::jxl::codestream_header::{JxlBasicInfo, JxlOrientation};
use jxl::jxl::color_encoding::JxlColorEncoding;
use jxl::jxl::decode::{self as dapi, JxlDecoderStatus as DStatus};
use jxl::jxl::encode::{self as eapi, JxlEncoderFrameSettingId as Id, JxlEncoderStatus as EStatus};
use jxl::jxl::hwy::{
    DECODER_SELECTS_SIMD, Target, decoder_target, decoder_target_for, max_vector_size,
    scalar_forced_by_environment, set_decoder_simd_for_test, simd_selections_on_this_thread,
};
use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
use std::ffi::c_void;
use std::ptr;
use std::sync::Mutex;

/// The override is process-wide; the tests of this binary run one at a time.
static OVERRIDE: Mutex<()> = Mutex::new(());

const WIDTH: u32 = 136;
const HEIGHT: u32 = 72;

/// A deterministic RGB image with edges, gradients and texture, so that the
/// encoder's AR heuristics, Gaborish and EPF have work to do.
fn pixels() -> Vec<u8> {
    let mut state = 0x2545_f491_u32;
    let mut out = Vec::with_capacity((WIDTH * HEIGHT * 3) as usize);
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            state ^= state << 13;
            state ^= state >> 17;
            state ^= state << 5;
            let noise = (state >> 27) as u8;
            let edge: u8 = if (x / 17 + y / 11) % 2 == 0 { 40 } else { 200 };
            out.push((x * 255 / WIDTH) as u8 ^ noise);
            out.push(edge.wrapping_add(noise));
            out.push(((x + 2 * y) % 256) as u8);
        }
    }
    out
}

/// The engine's lossy sequence (`crates/ferrisma-jxl-encoder`): effort 7,
/// patches off, VarDCT with XYB at `distance`, 8-bit sRGB.
fn encode(rgb: &[u8], distance: f32) -> Vec<u8> {
    let (bytes, selections) = encode_counted(rgb, distance);
    assert_eq!(
        selections, 0,
        "the encoder selected a SIMD target {selections} times"
    );
    bytes
}

/// The encoded bytes and how often the encode selected a SIMD target on
/// this thread (the port's encoder has no runner of its own).
fn encode_counted(rgb: &[u8], distance: f32) -> (Vec<u8>, u64) {
    let before = simd_selections_on_this_thread();
    let bytes = encode_sized(rgb, WIDTH, HEIGHT, distance);
    (bytes, simd_selections_on_this_thread() - before)
}

fn encode_sized(rgb: &[u8], width: u32, height: u32, distance: f32) -> Vec<u8> {
    // SAFETY: A null manager selects the default allocator; the encoder is
    // destroyed below and no reference to it outlives that.
    let raw = unsafe { eapi::jxl_encoder_create(ptr::null()) };
    assert!(!raw.is_null());
    // SAFETY: `raw` is live and uniquely owned until destroyed below.
    let enc = unsafe { &mut *raw };
    let mut basic = JxlBasicInfo::zeroed();
    eapi::jxl_encoder_init_basic_info(&mut basic);
    basic.xsize = width;
    basic.ysize = height;
    basic.bits_per_sample = 8;
    basic.uses_original_profile = 0;
    basic.num_color_channels = 3;
    basic.orientation = JxlOrientation::IDENTITY;
    assert_eq!(
        eapi::jxl_encoder_set_basic_info(enc, &basic),
        EStatus::SUCCESS
    );
    let mut color = JxlColorEncoding::default();
    eapi::jxl_color_encoding_set_to_srgb(&mut color, 0);
    assert_eq!(
        eapi::jxl_encoder_set_color_encoding(enc, &color),
        EStatus::SUCCESS
    );
    // SAFETY: The settings belong to the live encoder and are released with it.
    let settings = unsafe { eapi::jxl_encoder_frame_settings_create(raw, ptr::null()) };
    assert!(!settings.is_null());
    // SAFETY: `settings` and its encoder are live; no reference is held across
    // these calls.
    unsafe {
        assert_eq!(
            eapi::jxl_encoder_frame_settings_set_option(settings, Id::EFFORT, 7),
            EStatus::SUCCESS
        );
        assert_eq!(
            eapi::jxl_encoder_frame_settings_set_option(settings, Id::PATCHES, 0),
            EStatus::SUCCESS
        );
        assert_eq!(
            eapi::jxl_encoder_set_frame_distance(settings, distance),
            EStatus::SUCCESS
        );
    }
    let format = JxlPixelFormat {
        num_channels: 3,
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    // SAFETY: As above; the buffer is copied before the call returns.
    let status = unsafe { eapi::jxl_encoder_add_image_frame(settings, &format, Some(rgb)) };
    assert_eq!(status, EStatus::SUCCESS);
    // SAFETY: `raw` is live.
    eapi::jxl_encoder_close_input(unsafe { &mut *raw });
    let mut out = vec![0u8; 1 << 24];
    let mut next_out = out.as_mut_ptr();
    let mut avail_out = out.len();
    // SAFETY: `next_out` points to `avail_out` writable bytes of `out`, which
    // is not otherwise referenced during the call.
    let status = unsafe {
        eapi::jxl_encoder_process_output(&mut *raw, &raw mut next_out, &raw mut avail_out)
    };
    assert_eq!(
        status,
        EStatus::SUCCESS,
        "the admitted profile encodes in one call"
    );
    out.truncate(out.len() - avail_out);
    // SAFETY: `raw` came from jxl_encoder_create and is not used again.
    unsafe { eapi::jxl_encoder_destroy(raw) };
    out
}

/// What the image-out callback saw on every call.
#[derive(Default)]
struct Observed {
    /// `decoder_target()` and `max_vector_size()` inside the decode.
    targets: Vec<(Option<&'static str>, usize)>,
    threads: Vec<std::thread::ThreadId>,
    /// An encode run from inside the first callback.
    /// With the SIMD selections it made on the callback thread.
    nested_encode: Option<(Vec<u8>, u64)>,
    nested_encode_input: Option<Vec<u8>>,
}

struct Sink {
    observed: Mutex<Observed>,
    float: Mutex<Vec<f32>>,
}

unsafe extern "C" fn on_pixels(
    opaque: *mut c_void,
    x: usize,
    y: usize,
    num_pixels: usize,
    pixels: *const c_void,
) {
    // SAFETY: `opaque` is the `Sink` the decode below registered and keeps
    // alive; the decoder passes `num_pixels` RGB float samples at `pixels`.
    let sink = unsafe { &*opaque.cast::<Sink>() };
    let row = unsafe { std::slice::from_raw_parts(pixels.cast::<f32>(), 3 * num_pixels) };
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        let mut observed = sink.observed.lock().unwrap();
        observed
            .targets
            .push((decoder_target().map(Target::name), max_vector_size()));
        observed.threads.push(std::thread::current().id());
        if let Some(input) = observed.nested_encode_input.take() {
            observed.nested_encode = Some(encode_counted(&input, 1.0));
        }
        drop(observed);
        let mut float = sink.float.lock().unwrap();
        let start = 3 * (y * WIDTH as usize + x);
        float[start..start + row.len()].copy_from_slice(row);
    }));
    if result.is_err() {
        // Callbacks never unwind into the decoder.
        std::process::abort();
    }
}

/// The engine's decode sequence (`crates/ferrisma-jxl-decoder`), float RGB
/// through the image-out callback, optionally with a thread runner and an
/// encode from inside the first callback.
fn decode(
    file: &[u8],
    workers: usize,
    nested_encode_input: Option<Vec<u8>>,
) -> (Vec<f32>, Observed) {
    let sink = Sink {
        observed: Mutex::new(Observed {
            nested_encode_input,
            ..Observed::default()
        }),
        float: Mutex::new(vec![0.0; (WIDTH * HEIGHT * 3) as usize]),
    };
    // SAFETY: A null manager selects the default allocator.
    let raw = unsafe { dapi::jxl_decoder_create(ptr::null()) };
    assert!(!raw.is_null());
    // SAFETY: `raw` is live and uniquely owned until destroyed below.
    let dec = unsafe { &mut *raw };
    let runner = if workers > 0 {
        // SAFETY: A null manager selects the default allocator.
        let runner = unsafe {
            jxl::threads::thread_parallel_runner::jxl_thread_parallel_runner_create(
                ptr::null(),
                workers,
            )
        };
        assert!(!runner.is_null());
        // SAFETY: The runner obeys parallel_runner.h and outlives the decoder.
        let status = unsafe {
            dapi::jxl_decoder_set_parallel_runner(
                dec,
                Some(jxl::threads::thread_parallel_runner::jxl_thread_parallel_runner),
                runner,
            )
        };
        assert_eq!(status, DStatus::SUCCESS);
        runner
    } else {
        ptr::null_mut()
    };
    let events = DStatus::BASIC_INFO.0 | DStatus::FULL_IMAGE.0;
    assert_eq!(
        dapi::jxl_decoder_subscribe_events(dec, events as i32),
        DStatus::SUCCESS
    );
    // SAFETY: `file` outlives the decoder.
    assert_eq!(
        unsafe { dapi::jxl_decoder_set_input(dec, file.as_ptr(), file.len()) },
        DStatus::SUCCESS
    );
    dapi::jxl_decoder_close_input(dec);
    let format = JxlPixelFormat {
        num_channels: 3,
        data_type: JxlDataType::FLOAT,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    loop {
        // SAFETY: The input and the callback's sink stay live until the
        // decoder is destroyed.
        let status = unsafe { dapi::jxl_decoder_process_input(dec) };
        if status == DStatus::BASIC_INFO {
            // SAFETY: `sink` outlives the decoder; the callback never unwinds
            // and never reenters the decoder.
            let s = unsafe {
                dapi::jxl_decoder_set_image_out_callback(
                    dec,
                    Some(&format),
                    Some(on_pixels),
                    ptr::from_ref(&sink).cast_mut().cast(),
                )
            };
            assert_eq!(s, DStatus::SUCCESS);
        } else if status == DStatus::FULL_IMAGE {
            continue;
        } else {
            assert_eq!(status, DStatus::SUCCESS);
            break;
        }
    }
    // SAFETY: `raw` came from jxl_decoder_create and is not used again.
    unsafe { dapi::jxl_decoder_destroy(raw) };
    if !runner.is_null() {
        // SAFETY: The decoder that used the runner is destroyed.
        unsafe { jxl::threads::thread_parallel_runner::jxl_thread_parallel_runner_destroy(runner) };
    }
    (
        sink.float.into_inner().unwrap(),
        sink.observed.into_inner().unwrap(),
    )
}

fn bits(v: &[f32]) -> Vec<u32> {
    v.iter().map(|x| x.to_bits()).collect()
}

/// Runs `body` with the decoder's selection forced (`Some`) or with the
/// default restored (`None`).
fn forced<R>(simd: Option<bool>, body: impl FnOnce() -> R) -> R {
    set_decoder_simd_for_test(simd);
    let result = body();
    set_decoder_simd_for_test(None);
    result
}

/// The selection a decoder entry point makes with the default restored.
fn expected_decoder_target() -> Option<Target> {
    set_decoder_simd_for_test(None);
    decoder_target_for(DECODER_SELECTS_SIMD)
}

#[test]
fn outside_decoding_the_selection_is_the_scalar_path() {
    let _guard = OVERRIDE.lock().unwrap_or_else(|e| e.into_inner());
    set_decoder_simd_for_test(None);
    assert!(decoder_target().is_none());
    assert_eq!(max_vector_size(), 4);
    // The selection a decoder makes: NEON (or AVX2) only when the default is
    // on for this target and the scalar gate does not force the scalar path.
    let expected = expected_decoder_target();
    assert_eq!(
        expected.is_some(),
        DECODER_SELECTS_SIMD && !scalar_forced_by_environment() && Target::detect().is_some()
    );
    println!(
        "DECODER_SELECTS_SIMD: {DECODER_SELECTS_SIMD}, scalar forced by the environment: {}, decoder selection: {:?}",
        scalar_forced_by_environment(),
        expected.map(Target::name)
    );
}

#[test]
fn encoder_bytes_and_decoder_target_do_not_depend_on_each_other() {
    let _guard = OVERRIDE.lock().unwrap_or_else(|e| e.into_inner());
    let rgb = pixels();
    let expected = expected_decoder_target();
    let expected_name = expected.map(Target::name);
    let expected_vector = expected.map_or(4, |t| 4 * t.lanes());

    // Today's encoder: the scalar path forced process-wide.
    let reference = forced(Some(false), || encode(&rgb, 1.0));
    let reference_far = forced(Some(false), || encode(&rgb, 2.5));
    // Decodes with each path forced, to tell which one ran.
    let scalar_decode = forced(Some(false), || decode(&reference, 0, None).0);
    let simd_decode =
        Target::detect().map(|_| forced(Some(true), || decode(&reference, 0, None).0));
    let expected_decode = if expected.is_some() {
        simd_decode
            .clone()
            .expect("a SIMD selection implies a detected target")
    } else {
        scalar_decode.clone()
    };
    if let Some(simd) = &simd_decode {
        assert_ne!(
            bits(simd),
            bits(&scalar_decode),
            "this input must tell the SIMD decode from the scalar one"
        );
    }

    // The encoder runs decoder code in its own loop: a process-wide SIMD
    // selection (what a global switch would do) reaches it. The counter
    // therefore sees an encoder that is not scoped out of decoding, even
    // where its bytes happen not to change.
    if Target::detect().is_some() {
        let (bytes, selections) = forced(Some(true), || encode_counted(&rgb, 1.0));
        assert!(
            selections > 0,
            "a process-wide SIMD selection reaches the encoder's decoder code"
        );
        println!(
            "process-wide SIMD: the encoder selected SIMD {selections} times, bytes equal to the scalar encoder's: {}",
            bytes == reference
        );
    }

    // Encode first, then decode.
    assert_eq!(encode(&rgb, 1.0), reference, "encode before any decode");
    let before = simd_selections_on_this_thread();
    let (decoded, observed) = decode(&reference, 0, None);
    assert_eq!(
        simd_selections_on_this_thread() > before,
        expected.is_some(),
        "the decode (no runner) selects SIMD exactly when the decoder's selection is SIMD"
    );
    assert!(!observed.targets.is_empty());
    for target in &observed.targets {
        assert_eq!(
            *target,
            (expected_name, expected_vector),
            "inside the public decoder"
        );
    }
    assert_eq!(
        bits(&decoded),
        bits(&expected_decode),
        "public decode, default selection"
    );
    assert!(
        decoder_target().is_none(),
        "the scope ends with the entry point"
    );
    assert_eq!(max_vector_size(), 4);
    // Decode first (above), then encode.
    assert_eq!(encode(&rgb, 1.0), reference, "encode after a decode");
    assert_eq!(
        encode(&rgb, 2.5),
        reference_far,
        "encode after a decode, distance 2.5"
    );

    // A thread runner: the scope travels with every task.
    let (decoded, observed) = decode(&reference, 3, None);
    for target in &observed.targets {
        assert_eq!(
            *target,
            (expected_name, expected_vector),
            "on the runner's threads"
        );
    }
    assert_eq!(
        bits(&decoded),
        bits(&expected_decode),
        "public decode with a runner"
    );
    let main = std::thread::current().id();
    println!(
        "runner decode: {} callbacks, {} on other threads",
        observed.threads.len(),
        observed.threads.iter().filter(|t| **t != main).count()
    );

    // An encode from inside the decoder's own callback.
    let (decoded, observed) = decode(&reference, 0, Some(rgb.clone()));
    assert_eq!(
        observed
            .nested_encode
            .as_ref()
            .map(|(bytes, selections)| (bytes.as_slice(), *selections)),
        Some((reference.as_slice(), 0)),
        "an encode called from a decoder callback: same bytes, no SIMD selection"
    );
    assert_eq!(bits(&decoded), bits(&expected_decode));
    for target in &observed.targets {
        assert_eq!(
            *target,
            (expected_name, expected_vector),
            "around the nested encode"
        );
    }

    // An encode and a decode on two threads at the same time.
    let (encoded, decoded) = std::thread::scope(|s| {
        let encoder = s.spawn(|| encode(&rgb, 1.0));
        let decoder = s.spawn(|| decode(&reference, 2, None));
        (encoder.join().unwrap(), decoder.join().unwrap())
    });
    assert_eq!(encoded, reference, "an encode concurrent with a decode");
    assert_eq!(
        bits(&decoded.0),
        bits(&expected_decode),
        "a decode concurrent with an encode"
    );

    // The internal whole-decode helper is a decoder entry point as well.
    let helper = jxl::jxl::dec_frame::decode_vardct(&reference);
    assert!(
        helper.status.is_ok(),
        "the helper admits the engine's lossy profile"
    );
    let helper_expected = forced(Some(expected.is_some()), || {
        jxl::jxl::dec_frame::decode_vardct(&reference)
    });
    assert_eq!(
        bits(&helper.float_pixels),
        bits(&helper_expected.float_pixels)
    );
    assert_eq!(helper.pixels, helper_expected.pixels);
}
