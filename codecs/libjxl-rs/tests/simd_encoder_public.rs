// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
//! Whole encodes with the encoder's SIMD target forced against the original
//! built for the same single Highway target (port ADR 0004, "Encoder path"
//! condition (c), "Encoder selection"): the encoder's counterpart of
//! `tests/simd_public.rs`.
//!
//! Lossy: the public-encoder histories of the scalar `cjxl` gate
//! (`tests/parity/encoder_public.rs`: the admitted lossy families of encoder
//! steps 2 and 4, with alpha and extra channels, one-shot, streaming, output
//! processor and chunked input, and the step-2 gate sizes up to 4000x3000),
//! plus the engine's sequence (effort 7, patches off, d1.0 and d2.0, gray and
//! RGB, with and without alpha). Each history runs through the shared driver
//! (`tests/support/encoder_public.rs`) with
//! `hwy::set_encoder_simd_for_test(Some(true))` and through `epub-run` of the
//! SIMD oracle (`JXL_CPP_ORACLE_SIMD`, NEON or the AVX2 dispatch build); the
//! observations (every call's result and every output byte) must be equal.
//!
//! Lossless: the engine's `LosslessThunderConfig::encode_pixels` (effort 2)
//! over the corpus of the scalar whole-image gate (`tests/parity/whole_image.rs`)
//! and larger images, against the original's `ss-image`/`ss-image-alpha`
//! (WriteCodestreamHeaders and the one-shot Thunder frame) built for the same
//! target; the codestreams must be byte-identical.
//!
//! Every encode also runs with the scalar encoder path forced, to count the
//! encodes in which the paths differ (so the gate can tell them apart), and
//! records the call sites at which the encoder selected the SIMD target
//! (`hwy::encoder_trace_start`): the export dispatch points, and the decoder
//! code and plane padding the encoder runs inside its own scope. A family
//! fails if a dispatch point that its profile reaches (ADR 0004, "Encoder
//! exports the engine's profiles reach") never selected the SIMD target.
//!
//! The decoder's override is never set here: inside an encoder entry point
//! the decoder code follows the encoder's selection.
#![cfg(feature = "cpp-oracle-simd")]

mod simd_common;

#[path = "support/encoder_public.rs"]
mod driver;

use jxl::jxl::enc_frame::{LosslessSampleDepth, LosslessThunderConfig};
use jxl::jxl::hwy;
use simd_common::target;
use std::collections::{BTreeMap, BTreeSet};
use std::sync::Mutex;

/// The encoder override is process-wide; the tests of this binary run one at
/// a time.
static OVERRIDE: Mutex<()> = Mutex::new(());

fn lock() -> std::sync::MutexGuard<'static, ()> {
    OVERRIDE
        .lock()
        .unwrap_or_else(std::sync::PoisonError::into_inner)
}

fn bits(value: f32) -> u32 {
    value.to_bits()
}

fn run(mm: u32, fail: usize, script: &str) -> String {
    format!("epub-run {mm} {fail} {script}")
}

// ---------------------------------------------------------------------------
// The scalar gate's history generators, copied from
// tests/parity/encoder_public.rs (keep them identical in meaning).

/// A ProcessOutput schedule: 0, 1, 7 and 64 bytes, then doubling, until the
/// sum exceeds `total`.
fn output_schedule(total: usize) -> String {
    let mut ops = vec!["p0".to_string(), "p1".into(), "p7".into()];
    let mut sum = 8;
    let mut avail = 64;
    while sum <= total {
        ops.push(format!("p{avail}"));
        sum += avail;
        avail *= 2;
    }
    ops.join(" ")
}

const LOSSY_TYPES: [(u32, usize, &str); 4] = [
    (2, 1, ""),
    (3, 2, ",bits:16"),
    (5, 2, ",bits:16,exp:5"),
    (0, 4, ",bits:32,exp:8"),
];

const LOSSY_DISTANCES: [f32; 11] = [0.5, 1.0, 0.1, 1.5, 2.9, 0.69, 0.7, 2.0, 0.05, 0.3, 0.001];

#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn lossy_history(
    xsize: usize,
    ysize: usize,
    gray: bool,
    type_index: usize,
    content: usize,
    distance: f32,
    variant: usize,
    extra_info: &str,
    extra_settings: &str,
) -> String {
    let (data_type, bytes, depth) = LOSSY_TYPES[type_index];
    let channels = if gray { 1 } else { 3 };
    let endianness = variant % 3;
    let align = [0, 1, 4, 16, 3][variant % 5];
    let row = xsize * channels * bytes;
    let stride = if align < 2 || row.is_multiple_of(align) {
        row
    } else {
        row + align - row % align
    };
    let size = stride * (ysize - 1) + row + usize::from(variant % 4 == 3) * 5;
    let mut info = format!("Bw:{xsize},h:{ysize}{depth}{extra_info}");
    if gray {
        info.push_str(",c:1");
    }
    if variant % 7 == 2 {
        info.push_str(&format!(",it:{}", bits(100.0)));
    }
    let color = match variant % 3 {
        0 => String::new(),
        1 => format!(" G{}", u8::from(gray)),
        _ => format!(" g{}", u8::from(gray)),
    };
    let encoded_bound = xsize * ysize * channels * 4 + 2048;
    format!(
        "{info}{color} s- d0,{}{extra_settings} \
         a0,{channels},{data_type},{endianness},{align},{size},{content},{xsize},{ysize} Z {}",
        bits(distance),
        output_schedule(encoded_bound)
    )
}

const ALPHA_DEPTHS: [&str; 4] = [",ab:8", ",ab:12", ",ab:8", ",ab:10"];

#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn alpha_history(
    xsize: usize,
    ysize: usize,
    gray: bool,
    type_index: usize,
    content: usize,
    alpha_mode: usize,
    distance: f32,
    variant: usize,
    separate: bool,
    extra_info: &str,
    extra_settings: &str,
) -> String {
    let (data_type, bytes, depth) = LOSSY_TYPES[type_index];
    let color_channels = if gray { 1 } else { 3 };
    let channels = if separate {
        color_channels
    } else {
        color_channels + 1
    };
    let endianness = variant % 3;
    let align = [0, 1, 4, 16, 3][variant % 5];
    let stride = |channels: usize| {
        let row = xsize * channels * bytes;
        let stride = if align < 2 || row.is_multiple_of(align) {
            row
        } else {
            row + align - row % align
        };
        stride * (ysize - 1) + row
    };
    let size = stride(channels) + usize::from(variant % 4 == 3) * 5;
    let mut info = format!(
        "Bw:{xsize},h:{ysize}{depth},ec:1{}{extra_info}",
        ALPHA_DEPTHS[type_index]
    );
    if gray {
        info.push_str(",c:1");
    }
    if variant % 7 == 2 {
        info.push_str(&format!(",it:{}", bits(100.0)));
    }
    let color = match variant % 3 {
        0 => String::new(),
        1 => format!(" G{}", u8::from(gray)),
        _ => format!(" g{}", u8::from(gray)),
    };
    let color = if separate && color.is_empty() {
        format!(" G{}", u8::from(gray))
    } else {
        color
    };
    let content = content + 16 * alpha_mode;
    let mut frame =
        format!("a0,{channels},{data_type},{endianness},{align},{size},{content},{xsize},{ysize}");
    if separate {
        frame.push_str(&format!(
            " E0,1,{data_type},{endianness},{align},{},0,{content},{xsize},{ysize}",
            stride(1)
        ));
    }
    let encoded_bound = xsize * ysize * (channels + 1) * 4 + 2048;
    format!(
        "{info}{color} s- d0,{}{extra_settings} {frame} Z {}",
        bits(distance),
        output_schedule(encoded_bound)
    )
}

#[expect(clippy::too_many_arguments, reason = "one argument per history field")]
fn extra_channels_history(
    xsize: usize,
    ysize: usize,
    gray: bool,
    interleaved_alpha: bool,
    infos: &[(u32, u32)],
    distance: f32,
    variant: usize,
    extra_settings: &str,
) -> String {
    let color_channels = if gray { 1 } else { 3 };
    let channels = color_channels + usize::from(interleaved_alpha);
    let alpha = usize::from(interleaved_alpha);
    let num_ec = alpha + infos.len();
    let mut script = format!("Bw:{xsize},h:{ysize},ec:{num_ec}");
    if interleaved_alpha {
        script.push_str(",ab:8");
    }
    if gray {
        script.push_str(",c:1");
    }
    script.push_str(&format!(" G{}", u8::from(gray)));
    let spot = [bits(1.0), bits(0.25), 0, bits(0.5)];
    for (k, &(ec_type, ec_bits)) in infos.iter().enumerate() {
        script.push_str(&format!(
            " K{},{ec_type},{ec_bits},0,0,0,0,{},{},{},{}",
            alpha + k,
            spot[0],
            spot[1],
            spot[2],
            spot[3]
        ));
    }
    script.push_str(&format!(" s- d0,{}{extra_settings}", bits(distance)));
    script.push_str(&format!(
        " a0,{channels},2,0,0,{},{},{xsize},{ysize}",
        xsize * ysize * channels,
        1 + variant % 4 + 16 * (2 + variant % 2)
    ));
    for k in 0..infos.len() {
        script.push_str(&format!(
            " E0,1,2,0,0,{},{},{},{xsize},{ysize}",
            xsize * ysize,
            alpha + k,
            1 + (variant + k) % 4 + 16 * (2 + (variant + k) % 2)
        ));
    }
    format!(
        "{script} Z {}",
        output_schedule(xsize * ysize * (channels + infos.len()) * 4 + 2048)
    )
}

#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn processor_history(
    xsize: usize,
    ysize: usize,
    gray: bool,
    type_index: usize,
    content: usize,
    distance: f32,
    variant: usize,
    extra_settings: &str,
    processor: &str,
    early: bool,
) -> String {
    let history = lossy_history(
        xsize,
        ysize,
        gray,
        type_index,
        content,
        distance,
        variant,
        "",
        extra_settings,
    );
    let (input, _) = history.split_once(" Z ").unwrap();
    if early {
        let (setup, frame) = input.rsplit_once(" a0,").unwrap();
        format!("{setup} {processor} a0,{frame} Z i r S")
    } else {
        format!("{input} Z {processor} i r S")
    }
}

#[expect(clippy::too_many_arguments, reason = "One history's fields")]
fn chunked_history(
    xsize: usize,
    ysize: usize,
    gray: bool,
    alpha: bool,
    thermal: usize,
    type_index: usize,
    content: usize,
    distance: f32,
    variant: usize,
    settings: &str,
    processor: Option<&str>,
) -> String {
    let (data_type, _, depth) = LOSSY_TYPES[type_index];
    let color_channels = if gray { 1 } else { 3 };
    let channels = color_channels + usize::from(alpha);
    let num_ec = usize::from(alpha) + thermal;
    let mut info = format!("Bw:{xsize},h:{ysize}{depth}");
    if num_ec > 0 {
        info.push_str(&format!(",ec:{num_ec}"));
    }
    if alpha {
        info.push_str(ALPHA_DEPTHS[type_index]);
    }
    if gray {
        info.push_str(",c:1");
    }
    let mut script = format!("{info} G{}", u8::from(gray));
    for k in 0..thermal {
        script.push_str(&format!(" K{},6,8,0,0,0,0,0,0,0,0", usize::from(alpha) + k));
    }
    script.push_str(&format!(" s- d0,{}{settings}", bits(distance)));
    if let Some(processor) = processor {
        script.push_str(&format!(" {processor}"));
    }
    let endianness = variant % 3;
    let align = [0, 1, 4, 16, 128][variant % 5];
    let pad = [0, 3, 0, 8, 1][variant % 5];
    let content = content + 16 * usize::from(alpha) * (1 + variant % 3);
    script.push_str(&format!(
        " c0,1,{channels},{data_type},{endianness},{align},{content},{xsize},{ysize},{pad},0,2,{num_ec}"
    ));
    let encoded_bound = xsize * ysize * (channels + thermal) * 4 + 2048;
    match processor {
        Some(_) => format!("{script} Z r S"),
        None => format!("{script} Z {} S", output_schedule(encoded_bound)),
    }
}

fn gate_histories(xsize: usize, ysize: usize, seed: usize) -> Vec<String> {
    let mut requests = Vec::new();
    let mut k = seed;
    for distance in [1.0f32, 2.0] {
        for gray in [false, true] {
            for processor in [false, true] {
                let buffering = [-1, 0, 1, 2, 3][k % 5];
                let mut settings = if buffering == -1 {
                    String::new()
                } else {
                    format!(" o0,34,{buffering}")
                };
                settings.push_str(&format!(" o0,40,{}", [-1, 1, 0][k % 3]));
                let history =
                    lossy_history(xsize, ysize, gray, k % 4, 4, distance, k, "", &settings);
                let (input, _) = history.split_once(" Z ").unwrap();
                let chunk = 4 << 20;
                let count = (xsize * ysize * if gray { 1 } else { 3 }).div_ceil(chunk) + 1;
                let output = if processor {
                    format!("w{},0,0,0 i r", if k.is_multiple_of(2) { 0 } else { 4 })
                } else {
                    vec![format!("p{chunk}"); count].join(" ")
                };
                requests.push(run(0, 0, &format!("{input} Z {output}")));
                k += 1;
            }
        }
    }
    requests
}

// ---------------------------------------------------------------------------
// The families: the request lists of the scalar gate's admitted tests.

fn lossy_small() -> Vec<String> {
    let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
    sizes.extend((41..=48).map(|x| (x, 15)));
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for type_index in 0..4 {
                for content in 1..=4 {
                    let distance = LOSSY_DISTANCES[variant % LOSSY_DISTANCES.len()];
                    requests.push(run(
                        0,
                        0,
                        &lossy_history(
                            xsize, ysize, gray, type_index, content, distance, variant, "", "",
                        ),
                    ));
                    variant += 1;
                }
            }
        }
    }
    requests
}

fn lossy_large() -> Vec<String> {
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &[(63, 129), (256, 256), (257, 255), (512, 1024)] {
        for gray in [false, true] {
            for content in 1..=4 {
                let distance = [1.0, 2.9, 0.5, 1.5][variant % 4];
                requests.push(run(
                    0,
                    0,
                    &lossy_history(
                        xsize,
                        ysize,
                        gray,
                        variant % 4,
                        content,
                        distance,
                        variant,
                        "",
                        "",
                    ),
                ));
                variant += 1;
            }
        }
    }
    requests
}

fn lossy_settings() -> Vec<String> {
    let sizes = [
        (1, 1),
        (7, 7),
        (8, 8),
        (9, 17),
        (45, 15),
        (63, 129),
        (256, 256),
        (257, 255),
        (512, 1024),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize)) in sizes.iter().enumerate() {
        for distance in [1.0, 0.5] {
            for base in [0, 2] {
                let variant = base + index * 7;
                requests.push(run(
                    0,
                    0,
                    &lossy_history(xsize, ysize, false, 0, 4, distance, variant, "", ""),
                ));
                requests.push(run(
                    0,
                    0,
                    &lossy_history(xsize, ysize, true, 3, 4, distance, variant, "", ""),
                ));
            }
        }
    }
    for buffering in [-1, 0, 1, 2, 3] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                512,
                1024,
                false,
                0,
                4,
                0.3,
                5,
                "",
                &format!(" o0,34,{buffering}"),
            ),
        ));
    }
    requests.push(run(
        0,
        0,
        &lossy_history(64, 64, false, 1, 2, 0.5, 6, "", " o0,0,7"),
    ));
    requests.push(run(
        0,
        0,
        &lossy_history(1024, 1024, false, 0, 4, 0.5, 0, "", " o0,34,0"),
    ));
    for (info_bits, depth) in [(",bits:12", "1,0,0"), (",bits:16", "2,10,0")] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                19,
                23,
                false,
                1,
                4,
                0.5,
                4,
                info_bits,
                &format!(" W0,{depth}"),
            ),
        ));
    }
    for orientation in [1, 5, 8] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                21,
                13,
                false,
                0,
                2,
                0.5,
                0,
                &format!(",or:{orientation}"),
                "",
            ),
        ));
    }
    // The admitted histories of the admissions family: distances from 0.7
    // (the AR heuristics) and the streaming sizes.
    for (index, distance) in [0.7f32, 1.0, 1.5, 2.9].into_iter().enumerate() {
        for (xsize, ysize) in [(1, 1), (9, 17), (256, 256)] {
            requests.push(run(
                0,
                0,
                &lossy_history(
                    xsize,
                    ysize,
                    index % 2 == 1,
                    index,
                    4,
                    distance,
                    index,
                    "",
                    "",
                ),
            ));
        }
    }
    for (xsize, ysize, buffering) in [(768, 768, -1), (2049, 300, 2), (768, 769, 3)] {
        requests.push(run(
            0,
            0,
            &lossy_history(
                xsize,
                ysize,
                false,
                0,
                2,
                0.5,
                0,
                "",
                &format!(" o0,34,{buffering}"),
            ),
        ));
    }
    let mut smoke = Vec::new();
    for (w, h) in [(8usize, 8usize), (1, 1), (17, 9)] {
        for d in [0.5f32, 0.1, 1.0] {
            let size = w * h * 3;
            smoke.push(run(
                0,
                0,
                &format!(
                    "Bw:{w},h:{h} s- d0,{} a0,3,2,0,0,{size} Z {}",
                    bits(d),
                    output_schedule(4 * size + 1024)
                ),
            ));
        }
    }
    requests.extend(smoke);
    requests
}

/// The streaming cases of the scalar gate with and without alpha.
fn streaming_cases(alpha: bool) -> Vec<String> {
    let cases = [
        (768, 768, -1),
        (768, 769, 0),
        (1000, 700, -1),
        (2049, 300, 1),
        (2049, 300, 2),
        (768, 768, 3),
        (1031, 517, 3),
        (1024, 1024, -1),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize, buffering)) in cases.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 2 + usize::from(gray);
            let settings = if buffering == -1 {
                String::new()
            } else {
                format!(" o0,34,{buffering}")
            };
            let history = if alpha {
                alpha_history(
                    xsize,
                    ysize,
                    gray,
                    variant % 2,
                    1 + variant % 4,
                    2 + variant % 2,
                    [1.0, 0.5, 2.9, 1.5][variant % 4],
                    variant,
                    false,
                    "",
                    &settings,
                )
            } else {
                lossy_history(
                    xsize,
                    ysize,
                    gray,
                    variant % 4,
                    1 + variant % 4,
                    [1.0, 0.5, 2.9, 1.5][variant % 4],
                    variant,
                    "",
                    &settings,
                )
            };
            requests.push(run(0, 0, &history));
        }
    }
    requests
}

fn alpha_sizes() -> Vec<String> {
    let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
    sizes.extend((41..=48).map(|x| (x, 15)));
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for type_index in 0..2 {
                for alpha_mode in [2, 3] {
                    let distance = LOSSY_DISTANCES[variant % LOSSY_DISTANCES.len()];
                    requests.push(run(
                        0,
                        0,
                        &alpha_history(
                            xsize,
                            ysize,
                            gray,
                            type_index,
                            1 + variant % 4,
                            alpha_mode,
                            distance,
                            variant,
                            false,
                            "",
                            "",
                        ),
                    ));
                    variant += 1;
                }
            }
        }
    }
    let mut variant = 0;
    for &(xsize, ysize) in &[(63, 129), (256, 256), (257, 255), (512, 1024)] {
        for gray in [false, true] {
            for alpha_mode in [2, 3] {
                let distance = [1.0, 2.9, 0.5, 1.5, 0.7][variant % 5];
                requests.push(run(
                    0,
                    0,
                    &alpha_history(
                        xsize,
                        ysize,
                        gray,
                        variant % 2,
                        4,
                        alpha_mode,
                        distance,
                        variant,
                        false,
                        "",
                        "",
                    ),
                ));
                variant += 1;
            }
        }
    }
    requests
}

fn alpha_variants() -> Vec<String> {
    let mut requests = Vec::new();
    let mut push = |history: String| requests.push(run(0, 0, &history));
    for (index, &(xsize, ysize)) in [(1, 1), (9, 17), (45, 15), (256, 256), (768, 768)]
        .iter()
        .enumerate()
    {
        for type_index in 0..4 {
            let variant = index * 4 + type_index;
            let gray = variant % 2 == 1;
            let d = [1.0, 0.5, 2.9, 1.5][variant % 4];
            for settings in ["", " o0,41,2"] {
                push(alpha_history(
                    xsize, ysize, gray, type_index, 4, 1, d, variant, false, "", settings,
                ));
            }
            push(alpha_history(
                xsize, ysize, gray, type_index, 2, 2, d, variant, false, "", "",
            ));
            push(alpha_history(
                xsize,
                ysize,
                gray,
                type_index,
                4,
                2 + variant % 2,
                d,
                variant,
                true,
                "",
                "",
            ));
            push(alpha_history(
                xsize, ysize, gray, type_index, 4, 2, d, variant, false, ",apre:1", "",
            ));
        }
    }
    for (xsize, ysize) in [(7, 7), (63, 129), (257, 255)] {
        for (variant, ab) in [1usize, 4, 7, 12].into_iter().enumerate() {
            let gray = variant % 2 == 0;
            push(alpha_history(
                xsize,
                ysize,
                gray,
                0,
                4,
                2 + variant % 2,
                1.0,
                variant,
                false,
                &format!(",ab:{ab}"),
                "",
            ));
        }
        for settings in [" o0,41,1", " o0,41,0"] {
            push(alpha_history(
                xsize, ysize, false, 0, 4, 1, 1.0, 1, false, "", settings,
            ));
        }
        push(alpha_history(
            xsize, ysize, false, 0, 5, 3, 1.0, 2, false, "", " o0,8,0",
        ));
    }
    for (index, &(xsize, ysize)) in [(1, 1), (9, 17), (45, 15), (63, 129), (257, 255), (768, 768)]
        .iter()
        .enumerate()
    {
        for (k, settings) in [
            " o0,12,1".to_string(),
            " o0,12,0".to_string(),
            format!(" D0,0,{}", bits(1.0)),
            format!(" D0,0,{}", bits(0.5)),
            format!(" D0,0,{}", bits(3.0)),
        ]
        .iter()
        .enumerate()
        {
            let variant = index * 5 + k;
            push(alpha_history(
                xsize,
                ysize,
                variant % 2 == 1,
                variant % 2,
                1 + variant % 4,
                2 + variant % 2,
                [1.0, 0.5, 2.9, 1.5][variant % 4],
                variant,
                false,
                "",
                settings,
            ));
        }
    }
    requests
}

fn patches_off_and_extra_channels() -> Vec<String> {
    const PATCHES_OFF: &str = " o0,8,0";
    let mut requests = Vec::new();
    let mut variant = 0;
    let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
    sizes.extend((41..=48).step_by(3).map(|x| (x, 15)));
    for &(xsize, ysize) in &sizes {
        for gray in [false, true] {
            for type_index in 0..4 {
                for content in [1 + variant % 4, 5] {
                    let distance = LOSSY_DISTANCES[variant % LOSSY_DISTANCES.len()];
                    requests.push(run(
                        0,
                        0,
                        &lossy_history(
                            xsize,
                            ysize,
                            gray,
                            type_index,
                            content,
                            distance,
                            variant,
                            "",
                            PATCHES_OFF,
                        ),
                    ));
                    variant += 1;
                }
            }
        }
    }
    for &(xsize, ysize) in &[(63, 129), (96, 64), (256, 256), (257, 255), (512, 1024)] {
        for gray in [false, true] {
            for content in [4, 5] {
                let distance = [1.0, 2.9, 0.5, 1.5, 0.7][variant % 5];
                requests.push(run(
                    0,
                    0,
                    &lossy_history(
                        xsize,
                        ysize,
                        gray,
                        variant % 4,
                        content,
                        distance,
                        variant,
                        "",
                        PATCHES_OFF,
                    ),
                ));
                variant += 1;
            }
        }
    }
    for (xsize, ysize, buffering) in [(768, 768, -1), (1031, 517, 3), (2049, 300, 2)] {
        let settings = if buffering == -1 {
            PATCHES_OFF.to_string()
        } else {
            format!("{PATCHES_OFF} o0,34,{buffering}")
        };
        requests.push(run(
            0,
            0,
            &lossy_history(xsize, ysize, false, 0, 5, 1.0, variant, "", &settings),
        ));
        variant += 1;
    }
    // Text-like content with the patches option at 0 (the scalar gate's
    // text-like family).
    for (index, &(xsize, ysize)) in [(64, 64), (96, 64), (63, 129), (256, 256)]
        .iter()
        .enumerate()
    {
        for distance in [0.5f32, 1.0, 2.9] {
            requests.push(
                run(
                    0,
                    0,
                    &lossy_history(xsize, ysize, index % 2 == 1, 0, 5, distance, index, "", ""),
                )
                .replacen(" s- ", " s- o0,8,0 ", 1),
            );
        }
    }
    let sets: [(bool, &[(u32, u32)]); 6] = [
        (true, &[(1, 8), (2, 8)]),
        (false, &[(6, 10)]),
        (false, &[(0, 8), (3, 8)]),
        (true, &[(0, 8)]),
        (true, &[(5, 8), (6, 8), (1, 12)]),
        (false, &[(2, 1)]),
    ];
    for (index, &(xsize, ysize)) in [(1, 1), (9, 17), (63, 129), (257, 255), (768, 768)]
        .iter()
        .enumerate()
    {
        for (k, &(interleaved, infos)) in sets.iter().enumerate() {
            let variant = index * 6 + k;
            let settings = match variant % 5 {
                3 => format!(" D0,0,{}", bits(1.0)),
                _ => String::new(),
            };
            requests.push(run(
                0,
                0,
                &extra_channels_history(
                    xsize,
                    ysize,
                    variant % 2 == 1,
                    interleaved,
                    infos,
                    [1.0, 0.5, 2.9, 1.5][variant % 4],
                    variant,
                    &settings,
                ),
            ));
        }
    }
    requests
}

fn processor_and_chunked() -> Vec<String> {
    let cases = [
        (63, 129, -1, -1),
        (512, 1024, 3, -1),
        (768, 768, -1, -1),
        (768, 768, -1, 1),
        (768, 769, 0, -1),
        (1031, 517, 3, -1),
        (1031, 517, 2, 0),
        (2049, 300, 1, 1),
        (2049, 300, 2, -1),
        (1024, 1024, -1, 0),
        (1024, 1024, -1, 1),
        (1000, 700, 3, 0),
    ];
    let mut requests = Vec::new();
    for (index, &(xsize, ysize, buffering, output_mode)) in cases.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 2 + usize::from(gray);
            let mut settings = String::new();
            if buffering != -1 {
                settings.push_str(&format!(" o0,34,{buffering}"));
            }
            if output_mode != -1 {
                settings.push_str(&format!(" o0,40,{output_mode}"));
            }
            let mask = if variant % 3 == 1 { 4 } else { 0 };
            let schedule = variant % 4;
            let schedule = if schedule == 1 && xsize * ysize > 600_000 {
                3
            } else {
                schedule
            };
            requests.push(run(
                0,
                0,
                &processor_history(
                    xsize,
                    ysize,
                    gray,
                    variant % 4,
                    1 + variant % 4,
                    [1.0, 2.0, 0.5, 1.5][variant % 4],
                    variant,
                    &settings,
                    &format!("w{mask},{schedule},0,0"),
                    variant % 5 == 2,
                ),
            ));
        }
    }
    let cases: [(usize, usize, i32, bool, usize); 12] = [
        (1, 1, -1, false, 0),
        (17, 19, -1, true, 0),
        (63, 129, 0, false, 2),
        (257, 259, 2, true, 3),
        (257, 259, -1, false, 4),
        (512, 1024, 3, false, 0),
        (768, 768, -1, false, 0),
        (768, 768, 1, true, 1),
        (1031, 517, 3, false, 0),
        (1031, 517, 2, false, 2),
        (2049, 300, 0, false, 0),
        (1024, 1024, 3, true, 0),
    ];
    for (index, &(xsize, ysize, buffering, alpha, thermal)) in cases.iter().enumerate() {
        for gray in [false, true] {
            let variant = index * 2 + usize::from(gray);
            let type_index = if alpha { variant % 2 } else { variant % 4 };
            let settings = if buffering == -1 {
                String::new()
            } else {
                format!(" o0,34,{buffering}")
            };
            for streaming in [false, true] {
                let mask = if variant % 3 == 1 { 4 } else { 0 };
                let schedule = if xsize * ysize > 300_000 {
                    [0, 2, 3][variant % 3]
                } else {
                    variant % 4
                };
                let processor = format!("w{mask},{schedule},0,0");
                requests.push(run(
                    0,
                    0,
                    &chunked_history(
                        xsize,
                        ysize,
                        gray,
                        alpha,
                        thermal,
                        type_index,
                        1 + variant % 4,
                        [1.0, 2.0, 0.5, 1.5][variant % 4],
                        variant,
                        &settings,
                        streaming.then_some(processor.as_str()),
                    ),
                ));
            }
        }
    }
    requests
}

fn streaming_suite() -> Vec<String> {
    let mut requests = Vec::new();
    for bitmask in 0..256usize {
        let (container, large, multiple, fast_lossless) = (
            bitmask & 1 != 0,
            bitmask & 2 != 0,
            bitmask & 4 != 0,
            bitmask & 8 != 0,
        );
        let (seek, extra, alpha, onegroup) = (
            bitmask & 16 != 0,
            bitmask & 32 != 0,
            bitmask & 64 != 0,
            bitmask & 128 != 0,
        );
        if container || multiple || fast_lossless {
            continue;
        }
        for mode in [0, 1] {
            let (xsize, ysize): (usize, usize) = if onegroup { (17, 19) } else { (257, 259) };
            let align: usize = if onegroup { 128 } else { 0 };
            let channels = if alpha { 4 } else { 3 };
            let thermal = if extra { 5 } else { 0 };
            let num_ec = thermal + usize::from(alpha);
            let mut info = format!("Bw:{xsize},h:{ysize}");
            if num_ec > 0 {
                info.push_str(&format!(",ec:{num_ec}"));
            }
            if alpha {
                info.push_str(",ab:8");
            }
            let mut setup = format!("{info} G0");
            for i in 0..thermal {
                setup.push_str(&format!(" K{i},6,8,0,0,0,0,0,0,0,0"));
            }
            let settings = |streaming: bool| {
                format!(
                    " s- o0,34,{} o0,40,{mode} o0,38,0",
                    if streaming { 2 } else { 0 }
                )
            };
            let row = xsize * channels;
            let stride = if align > 1 && !row.is_multiple_of(align) {
                row + align - row % align
            } else {
                row
            };
            let size = stride * (ysize - 1) + row;
            let ec_row = xsize;
            let ec_stride = if align > 1 && !ec_row.is_multiple_of(align) {
                ec_row + align - ec_row % align
            } else {
                ec_row
            };
            let ec_size = ec_stride * (ysize - 1) + ec_row;
            let mut frame = format!("a0,{channels},2,0,{align},{size},3,{xsize},{ysize}");
            for ec in 0..thermal {
                frame.push_str(&format!(
                    " E0,1,2,0,{align},{ec_size},{ec},3,{xsize},{ysize}"
                ));
            }
            let chunked = format!(
                "c0,1,{channels},2,0,{align},3,{xsize},{ysize},{},0,2,{num_ec}",
                stride - row
            );
            let processor = format!(
                "w{},{},0,0",
                if seek { 0 } else { 4 },
                if large { 0 } else { 1 }
            );
            let bound = output_schedule(xsize * ysize * (channels + thermal) * 4 + 2048);
            requests.push(run(
                0,
                0,
                &format!("{setup}{} {frame} Z {bound}", settings(false)),
            ));
            requests.push(run(
                0,
                0,
                &format!("{setup}{} {frame} Z {processor} i r", settings(true)),
            ));
            requests.push(run(
                0,
                0,
                &format!("{setup}{} {chunked} {bound}", settings(true)),
            ));
            requests.push(run(
                0,
                0,
                &format!("{setup}{} {processor} {chunked} r", settings(true)),
            ));
        }
    }
    requests
}

/// The engine's sequence (`crates/ferrisma-jxl-encoder`): effort 7 set
/// explicitly, patches off, 8-bit input, d1.0 and d2.0, gray and RGB, with
/// and without one straight 8-bit alpha channel, one-shot and streaming
/// sizes, the four content kinds.
fn engine_profile() -> Vec<String> {
    const ENGINE: &str = " o0,0,7 o0,8,0";
    let sizes = [
        (1, 1),
        (8, 8),
        (64, 64),
        (255, 257),
        (512, 1024),
        (768, 768),
        (1031, 517),
        (2048, 1536),
    ];
    let mut requests = Vec::new();
    let mut variant = 0;
    for &(xsize, ysize) in &sizes {
        for distance in [1.0f32, 2.0] {
            for gray in [false, true] {
                // 8-bit only (type 0), the engine's alignment and byte order
                // (variant 0) and the gate's rotation.
                let content = 1 + variant % 4;
                requests.push(run(
                    0,
                    0,
                    &lossy_history(xsize, ysize, gray, 0, content, distance, 0, "", ENGINE),
                ));
                requests.push(run(
                    0,
                    0,
                    &alpha_history(
                        xsize,
                        ysize,
                        gray,
                        0,
                        content,
                        2 + variant % 2,
                        distance,
                        0,
                        false,
                        "",
                        ENGINE,
                    ),
                ));
                variant += 1;
            }
        }
    }
    requests
}

// ---------------------------------------------------------------------------
// Running and comparing.

/// One port run of a history with the encoder's selection forced.
struct PortRun {
    text: String,
    admission: Option<usize>,
    trace: BTreeMap<String, u64>,
}

fn port_history(request: &str, simd: bool) -> PortRun {
    hwy::set_encoder_simd_for_test(Some(simd));
    let mut args = request.split_whitespace();
    assert_eq!(args.next(), Some("epub-run"));
    hwy::encoder_trace_start();
    let observation = driver::observe(&mut args);
    let trace = hwy::encoder_trace_stop();
    hwy::set_encoder_simd_for_test(None);
    PortRun {
        text: observation.text,
        admission: observation.admission.map(|(index, _)| index),
        trace,
    }
}

/// The codestream bytes of a history's ProcessOutput results and processor
/// writes (an observation line of either implementation).
fn output_bytes(text: &str) -> Vec<u8> {
    let mut bytes = Vec::new();
    for field in text.split(' ') {
        let Some(result) = field.strip_prefix("p:") else {
            continue;
        };
        let parts: Vec<&str> = result.split(',').collect();
        let Ok(written) = parts[1].parse::<usize>() else {
            continue;
        };
        let hex = parts[3].split('/').next().unwrap();
        for i in 0..written.min(hex.len() / 2) {
            bytes.push(u8::from_str_radix(&hex[2 * i..2 * i + 2], 16).unwrap());
        }
    }
    bytes
}

/// A short description of where two observations first differ.
fn divergence(port: &str, original: &str) -> String {
    let p: Vec<&str> = port.split(' ').collect();
    let o: Vec<&str> = original.split(' ').collect();
    let call = p.iter().zip(&o).position(|(a, b)| a != b);
    let (pb, ob) = (output_bytes(port), output_bytes(original));
    let byte = pb.iter().zip(&ob).position(|(a, b)| a != b);
    let differing = pb.iter().zip(&ob).filter(|(a, b)| a != b).count();
    format!(
        "first differing field {call:?} ({:?} vs {:?}), first differing output byte {byte:?}, \
         {differing} differing bytes, sizes {}/{}",
        call.map(|i| &p[i][..p[i].len().min(24)]),
        call.map(|i| &o[i][..o[i].len().min(24)]),
        pb.len(),
        ob.len()
    )
}

#[derive(Default)]
struct FamilyResult {
    histories: usize,
    exact: usize,
    admissions: usize,
    scalar_differs: usize,
    differing: Vec<String>,
    sites: BTreeMap<String, u64>,
}

/// Runs `requests` through the SIMD oracle (on another thread) and through
/// the port with the SIMD and the scalar encoder path forced.
fn compare_family(family: &str, requests: &[String]) -> FamilyResult {
    let originals = {
        let requests = requests.to_vec();
        std::thread::spawn(move || simd_common::oracle(&requests))
    };
    let mut result = FamilyResult::default();
    let mut port = Vec::with_capacity(requests.len());
    for request in requests {
        let simd = port_history(request, true);
        let scalar = port_history(request, false);
        if scalar.text != simd.text {
            result.scalar_differs += 1;
        }
        for (site, count) in &simd.trace {
            *result.sites.entry(site.clone()).or_default() += count;
        }
        port.push(simd);
    }
    let originals = originals.join().expect("the SIMD oracle thread");
    for ((request, simd), original) in requests.iter().zip(&port).zip(&originals) {
        result.histories += 1;
        let equal = match simd.admission {
            None => simd.text == *original,
            Some(index) => {
                // An admission result: compared up to the admission call, as
                // the scalar gate does (not parity of a whole encode).
                result.admissions += 1;
                let p: Vec<&str> = simd.text.split(' ').collect();
                let o: Vec<&str> = original.split(' ').collect();
                p.len() > index && o.len() > index && p[..=index] == o[..=index]
            }
        };
        if equal {
            if simd.admission.is_none() {
                result.exact += 1;
            }
        } else {
            result.differing.push(format!(
                "{}: {}",
                &request[..request.len().min(200)],
                divergence(&simd.text, original)
            ));
        }
    }
    println!(
        "{family} on {}: {} histories, {} whole encodes byte-identical with the original's {} \
         build, {} admission prefixes, {} differ; the scalar encoder path differs from the SIMD \
         path in {} of {}",
        target().name(),
        result.histories,
        result.exact,
        target().name(),
        result.admissions,
        result.differing.len(),
        result.scalar_differs,
        result.histories
    );
    for line in &result.differing {
        println!("  DIFFERS {line}");
    }
    result
}

/// The dispatch points of the encoder exports and the decoder code a profile
/// reaches (ADR 0004, "Encoder exports the engine's profiles reach"), by
/// source file of the call site.
/// (`enc_transforms` is called inside the target namespace of the other
/// exports, as in the original, not through its own dispatch point.)
const LOSSY_SITES: &[&str] = &[
    "src/jxl/enc_xyb.rs",
    "src/jxl/enc_group.rs",
    "src/jxl/enc_chroma_from_luma.rs",
    "src/jxl/enc_ac_strategy.rs",
    "src/jxl/enc_adaptive_quantization.rs",
    "src/jxl/enc_cluster.rs",
    "src/jxl/enc_entropy_coder.rs",
    "src/jxl/enc_entropy.rs",
    "src/jxl/convolve_symmetric5.rs",
    // The decoder code inside the encoder and its planes (condition (b)).
    "src/jxl/quant_weights.rs",
    "src/jxl/dec_group_render.rs",
    "src/jxl/enc_cache.rs",
    "src/jxl/memory_manager_internal.rs",
];
const ALPHA_SITES: &[&str] = &[
    "src/jxl/modular/encoding/enc_ma.rs",
    "src/jxl/enc_modular_simd.rs",
];
const LOSSLESS_SITES: &[&str] = &[
    "src/jxl/modular/transform/enc_rct.rs",
    "src/jxl/enc_cluster.rs",
    "src/jxl/memory_manager_internal.rs",
];

fn files(sites: &BTreeMap<String, u64>) -> BTreeSet<String> {
    sites
        .keys()
        .map(|site| {
            let file = site.rsplit_once(':').map_or(site.as_str(), |(f, _)| f);
            // Call sites in the crate are relative to the package root.
            file.trim_start_matches("codecs/libjxl-rs/").to_string()
        })
        .collect()
}

fn assert_reached(family: &str, result: &FamilyResult, expected: &[&str]) {
    let reached = files(&result.sites);
    println!(
        "{family}: SIMD selections at {} call sites in {} files: {}",
        result.sites.len(),
        reached.len(),
        reached.iter().cloned().collect::<Vec<_>>().join(", ")
    );
    let missing: Vec<_> = expected
        .iter()
        .filter(|file| !reached.contains(**file))
        .collect();
    assert!(
        missing.is_empty(),
        "{family}: these dispatch points never selected the SIMD target: {missing:?}"
    );
}

fn assert_family(family: &str, result: &FamilyResult) {
    assert!(
        result.differing.is_empty(),
        "{family}: {} whole encodes with the {} target differ from the original's {} build",
        result.differing.len(),
        target().name(),
        target().name()
    );
}

fn lossy_gate(family: &str, requests: Vec<String>, expected: &[&str]) {
    let _guard = lock();
    let result = compare_family(family, &requests);
    assert_reached(family, &result, expected);
    assert_family(family, &result);
}

/// Diagnostics: the histories of the file that JXL_ENCODER_PUBLIC_REQUESTS
/// names, compared as a family (the first divergence of each).
#[test]
#[ignore = "diagnostics for a request file"]
fn enc_public_first_divergence() {
    let Some(path) = std::env::var_os("JXL_ENCODER_PUBLIC_REQUESTS") else {
        return;
    };
    let requests: Vec<String> = std::fs::read_to_string(path)
        .unwrap()
        .lines()
        .filter(|line| !line.is_empty())
        .map(str::to_owned)
        .collect();
    let _guard = lock();
    compare_family("requests", &requests);
}

#[test]
fn enc_public_lossy_small_large_and_settings_match_the_original_target() {
    let mut requests = lossy_small();
    requests.extend(lossy_large());
    requests.extend(lossy_settings());
    lossy_gate("lossy", requests, LOSSY_SITES);
}

#[test]
fn enc_public_lossy_streaming_matches_the_original_target() {
    lossy_gate("lossy-streaming", streaming_cases(false), LOSSY_SITES);
}

#[test]
fn enc_public_lossy_alpha_matches_the_original_target() {
    let mut requests = alpha_sizes();
    requests.extend(streaming_cases(true));
    requests.extend(alpha_variants());
    let expected: Vec<&str> = LOSSY_SITES.iter().chain(ALPHA_SITES).copied().collect();
    lossy_gate("lossy-alpha", requests, &expected);
}

#[test]
fn enc_public_lossy_patches_off_and_extra_channels_match_the_original_target() {
    let expected: Vec<&str> = LOSSY_SITES.iter().chain(ALPHA_SITES).copied().collect();
    lossy_gate(
        "lossy-patches-off-extra-channels",
        patches_off_and_extra_channels(),
        &expected,
    );
}

#[test]
fn enc_public_output_processor_and_chunked_input_match_the_original_target() {
    let mut requests = processor_and_chunked();
    requests.extend(streaming_suite());
    lossy_gate("processor-chunked", requests, LOSSY_SITES);
}

#[test]
fn enc_public_engine_profile_d1_d2_matches_the_original_target() {
    let expected: Vec<&str> = LOSSY_SITES.iter().chain(ALPHA_SITES).copied().collect();
    lossy_gate("engine-e7-d1-d2", engine_profile(), &expected);
}

#[test]
fn enc_public_streaming_gate_sizes_match_the_original_target() {
    let mut requests = Vec::new();
    for (index, (xsize, ysize)) in [(768, 768), (1024, 1024), (2048, 1536)]
        .into_iter()
        .enumerate()
    {
        requests.extend(gate_histories(xsize, ysize, 3 * index));
    }
    requests.extend(gate_histories(2049, 2049, 1));
    lossy_gate("streaming-gate", requests, LOSSY_SITES);
}

#[test]
fn enc_public_streaming_gate_4000x3000_matches_the_original_target() {
    lossy_gate(
        "streaming-gate-4000x3000",
        gate_histories(4000, 3000, 2),
        LOSSY_SITES,
    );
}

// The scalar gate's thin shapes on the SIMD path (ADR-0027, amendment
// 2026-11-17): the gate's eight histories (seed 2) at 2049x257 and 257x2049
// (one DC-group boundary in one direction) and 4097x9 and 9x4097 (three DC
// groups in a row, a side above 2^12). Every encode must match
// (assert_family); unlike the large gates, they do not also require each
// SIMD call site of the lossy profile (assert_reached), which those reach.
fn thin_streaming_gate(family: &str, (xsize, ysize): (usize, usize)) {
    let _guard = lock();
    let result = compare_family(family, &gate_histories(xsize, ysize, 2));
    assert_family(family, &result);
}

#[test]
fn enc_public_streaming_gate_2049x257_matches_the_original_target() {
    thin_streaming_gate("streaming-gate-2049x257", (2049, 257));
}

#[test]
fn enc_public_streaming_gate_257x2049_matches_the_original_target() {
    thin_streaming_gate("streaming-gate-257x2049", (257, 2049));
}

#[test]
fn enc_public_streaming_gate_4097x9_matches_the_original_target() {
    thin_streaming_gate("streaming-gate-4097x9", (4097, 9));
}

#[test]
fn enc_public_streaming_gate_9x4097_matches_the_original_target() {
    thin_streaming_gate("streaming-gate-9x4097", (9, 4097));
}

// ---------------------------------------------------------------------------
// Lossless effort 2 (`LosslessThunderConfig::encode_pixels`).

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

/// tests/parity/whole_image.rs's `pixels`, plus a photo-like mode (5).
fn pixels(w: usize, h: usize, gray: bool, depth: u32, mode: usize) -> Vec<u8> {
    let channels = if gray { 1 } else { 3 };
    let mut seed = 0xc0ffee77119955u64;
    (0..w * h * channels)
        .flat_map(|i| {
            let v = match mode {
                0 => 0,
                1 => {
                    if i % 3 == 0 {
                        65535
                    } else {
                        317
                    }
                }
                2 => (i * 173 + i * i * 29 + 61) as u16,
                3 => ((i * 7) % 5) as u16,
                5 => {
                    let (p, c) = (i / channels, i % channels);
                    let (x, y) = (p % w, p / w);
                    let smooth = (x * 3 + y * 2 + c * 50) % 512;
                    let tri = if smooth < 256 { smooth } else { 511 - smooth };
                    let noise = (next(&mut seed) % 24) as usize;
                    let v = tri + noise + if (x / 13 + y / 9) % 5 == 0 { 60 } else { 0 };
                    if depth == 8 {
                        v.min(255) as u16
                    } else {
                        (v * 257).min(65535) as u16
                    }
                }
                _ => next(&mut seed) as u16,
            };
            if depth == 8 {
                vec![v as u8]
            } else {
                v.to_ne_bytes().to_vec()
            }
        })
        .collect()
}

/// tests/parity/whole_image.rs's `alpha_pixels`.
fn alpha_pixels(w: usize, h: usize, gray: bool, depth: u32, mode: usize) -> Vec<u8> {
    let channels = if gray { 2 } else { 4 };
    let mut seed = 0xaa558811cdef0023;
    (0..w * h * channels)
        .flat_map(|i| {
            let value = if i % channels == channels - 1 {
                match mode {
                    0 => 0,
                    1 => u16::MAX,
                    2 => [0, u16::MAX, 1, 32768, 129][(i / channels) % 5],
                    _ => next(&mut seed) as u16,
                }
            } else {
                match mode {
                    0 => (i * 173 + i * i * 29 + 61) as u16,
                    1 => 0,
                    2 => [1, 65534, 257, 32767][i % 4],
                    3 => ((i / channels) % 5) as u16,
                    _ => next(&mut seed) as u16,
                }
            };
            if depth == 8 {
                vec![value as u8]
            } else {
                value.to_ne_bytes().to_vec()
            }
        })
        .collect()
}

fn lossless_config(
    w: usize,
    h: usize,
    gray: bool,
    depth: u32,
    alpha: bool,
) -> LosslessThunderConfig {
    let depth = if depth == 8 {
        LosslessSampleDepth::Uint8
    } else {
        LosslessSampleDepth::Uint16
    };
    if alpha {
        LosslessThunderConfig::new_with_alpha(w, h, gray, depth).unwrap()
    } else {
        LosslessThunderConfig::new(w, h, gray, depth).unwrap()
    }
}

/// One lossless encode with the encoder's selection forced, and its trace.
fn port_lossless(
    (w, h, gray, depth, alpha): (usize, usize, bool, u32, bool),
    input: &[u8],
    simd: bool,
) -> (Vec<u8>, BTreeMap<String, u64>) {
    hwy::set_encoder_simd_for_test(Some(simd));
    hwy::encoder_trace_start();
    let bytes = lossless_config(w, h, gray, depth, alpha)
        .encode_pixels(input, None)
        .unwrap()
        .as_slice()
        .to_vec();
    let trace = hwy::encoder_trace_stop();
    hwy::set_encoder_simd_for_test(None);
    (bytes, trace)
}

#[test]
fn enc_public_lossless_effort2_matches_the_original_target() {
    let _guard = lock();
    let mut cases = Vec::new();
    for (w, h) in [
        (1, 1),
        (3, 2),
        (17, 9),
        (65, 33),
        (257, 3),
        (513, 3),
        (2049, 2),
        (3, 257),
        (3, 2049),
        (257, 257),
        (300, 260),
        (640, 480),
    ] {
        for gray in [false, true] {
            for depth in [8, 16] {
                for mode in 0..6 {
                    cases.push(((w, h, gray, depth, false), mode));
                    if mode < 5 {
                        cases.push(((w, h, gray, depth, true), mode));
                    }
                }
            }
        }
    }
    // The engine's 8-bit RGB and RGBA at photograph-like sizes.
    for (w, h) in [(1024, 768), (2048, 1536)] {
        cases.push(((w, h, false, 8, false), 5));
        cases.push(((w, h, false, 8, true), 4));
    }
    let mut requests = Vec::new();
    let mut simd_bytes = Vec::new();
    let mut scalar_differs = 0;
    let mut sites = BTreeMap::new();
    for &(shape, mode) in &cases {
        let (w, h, gray, depth, alpha) = shape;
        let input = if alpha {
            alpha_pixels(w, h, gray, depth, mode)
        } else {
            pixels(w, h, gray, depth, mode)
        };
        let (simd, trace) = port_lossless(shape, &input, true);
        let (scalar, _) = port_lossless(shape, &input, false);
        if scalar != simd {
            scalar_differs += 1;
        }
        for (site, count) in trace {
            *sites.entry(site).or_default() += count;
        }
        requests.push(format!(
            "ss-image{} {w} {h} {} {depth} 0 {} {}",
            if alpha { "-alpha" } else { "" },
            u32::from(gray),
            driver::hex(&input),
            driver::hex(&simd)
        ));
        simd_bytes.push(simd);
    }
    let originals = simd_common::oracle(&requests);
    let mut differing = Vec::new();
    for (((shape, mode), simd), original) in cases.iter().zip(&simd_bytes).zip(&originals) {
        let fields: Vec<&str> = original.split_whitespace().collect();
        assert_eq!(fields[0], "0", "the original encodes {shape:?} mode {mode}");
        let original = driver::unhex(fields[1]);
        if original != *simd {
            let byte = simd.iter().zip(&original).position(|(a, b)| a != b);
            differing.push(format!(
                "{shape:?} mode {mode}: first differing byte {byte:?}, sizes {}/{}",
                simd.len(),
                original.len()
            ));
        }
    }
    let result = FamilyResult {
        histories: cases.len(),
        exact: cases.len() - differing.len(),
        scalar_differs,
        differing,
        sites,
        ..FamilyResult::default()
    };
    println!(
        "lossless-e2 on {}: {} encodes, {} byte-identical with the original's {} build, {} \
         differ; the scalar encoder path differs from the SIMD path in {} of {}",
        target().name(),
        result.histories,
        result.exact,
        target().name(),
        result.differing.len(),
        result.scalar_differs,
        result.histories
    );
    for line in &result.differing {
        println!("  DIFFERS {line}");
    }
    assert_reached("lossless-e2", &result, LOSSLESS_SITES);
    assert_family("lossless-e2", &result);
}

// ---------------------------------------------------------------------------
// Photographs (optional): `JXL_ENCODER_GATE_PHOTOS` names binary 8-bit RGB
// PPM files, separated by `:`. Real images contain no NaN or infinite
// sample, so every difference here is a finding.

/// A PPM's size and RGB samples.
fn read_ppm(path: &str) -> (usize, usize, Vec<u8>) {
    let data = std::fs::read(path).unwrap_or_else(|e| panic!("read {path}: {e}"));
    let mut fields = Vec::new();
    let mut at = 0;
    while fields.len() < 4 {
        while data[at].is_ascii_whitespace() {
            at += 1;
        }
        let start = at;
        while !data[at].is_ascii_whitespace() {
            at += 1;
        }
        fields.push(String::from_utf8(data[start..at].to_vec()).unwrap());
    }
    assert_eq!(
        (fields[0].as_str(), fields[3].as_str()),
        ("P6", "255"),
        "{path}: 8-bit binary PPM"
    );
    let (w, h): (usize, usize) = (fields[1].parse().unwrap(), fields[2].parse().unwrap());
    (w, h, data[at + 1..at + 1 + w * h * 3].to_vec())
}

/// tools/oracle/encoder_selection.cc's EselAlpha.
fn esel_alpha(x: usize, y: usize) -> u8 {
    if (x + 2 * y) % 23 < 5 {
        return 0;
    }
    ((x * 7 + y * 3) % 256) as u8
}

/// The rectangle of an RGB image as interleaved RGB or RGBA (esel-ppm's input).
fn crop(
    image: &(usize, usize, Vec<u8>),
    (x0, y0, xsize, ysize): (usize, usize, usize, usize),
    alpha: bool,
) -> Vec<u8> {
    let channels = if alpha { 4 } else { 3 };
    let mut out = Vec::with_capacity(xsize * ysize * channels);
    for y in 0..ysize {
        for x in 0..xsize {
            let at = ((y0 + y) * image.0 + x0 + x) * 3;
            out.extend_from_slice(&image.2[at..at + 3]);
            if alpha {
                out.push(esel_alpha(x, y));
            }
        }
    }
    out
}

/// The engine's lossy encode (esel-ppm's sequence) through the port's public
/// encoder with the encoder's selection forced.
fn port_photo(
    pixels: &[u8],
    xsize: usize,
    ysize: usize,
    alpha: bool,
    distance: f32,
    simd: bool,
) -> (Vec<u8>, BTreeMap<String, u64>) {
    use jxl::jxl::codestream_header::{JxlBasicInfo, JxlOrientation};
    use jxl::jxl::color_encoding::JxlColorEncoding;
    use jxl::jxl::encode::{
        self as api, JxlEncoderFrameSettingId as Id, JxlEncoderStatus as Status,
    };
    use jxl::jxl::types::{JxlDataType, JxlEndianness, JxlPixelFormat};
    hwy::set_encoder_simd_for_test(Some(simd));
    hwy::encoder_trace_start();
    // SAFETY: A null manager selects the default allocator; the encoder is
    // destroyed below and no reference to it outlives that.
    let raw = unsafe { api::jxl_encoder_create(std::ptr::null()) };
    assert!(!raw.is_null());
    // SAFETY: `raw` is live and uniquely owned until destroyed below.
    let enc = unsafe { &mut *raw };
    let mut info = JxlBasicInfo::zeroed();
    api::jxl_encoder_init_basic_info(&mut info);
    info.xsize = xsize as u32;
    info.ysize = ysize as u32;
    info.bits_per_sample = 8;
    info.num_color_channels = 3;
    info.uses_original_profile = 0;
    info.orientation = JxlOrientation::IDENTITY;
    if alpha {
        info.num_extra_channels = 1;
        info.alpha_bits = 8;
    }
    assert_eq!(api::jxl_encoder_set_basic_info(enc, &info), Status::SUCCESS);
    let mut color = JxlColorEncoding::default();
    api::jxl_color_encoding_set_to_srgb(&mut color, 0);
    assert_eq!(
        api::jxl_encoder_set_color_encoding(enc, &color),
        Status::SUCCESS
    );
    // SAFETY: The settings belong to the live encoder and are released with
    // it; no reference is held across these calls.
    let settings = unsafe { api::jxl_encoder_frame_settings_create(raw, std::ptr::null()) };
    assert!(!settings.is_null());
    // SAFETY: As above.
    unsafe {
        assert_eq!(
            api::jxl_encoder_frame_settings_set_option(settings, Id::EFFORT, 7),
            Status::SUCCESS
        );
        assert_eq!(
            api::jxl_encoder_frame_settings_set_option(settings, Id::PATCHES, 0),
            Status::SUCCESS
        );
        assert_eq!(
            api::jxl_encoder_set_frame_distance(settings, distance),
            Status::SUCCESS
        );
    }
    let format = JxlPixelFormat {
        num_channels: if alpha { 4 } else { 3 },
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    // SAFETY: As above; the buffer is copied before the call returns.
    let status = unsafe { api::jxl_encoder_add_image_frame(settings, &format, Some(pixels)) };
    assert_eq!(status, Status::SUCCESS);
    // SAFETY: `raw` is live.
    api::jxl_encoder_close_input(unsafe { &mut *raw });
    let mut out = vec![0u8; pixels.len() + 65536];
    let mut next_out = out.as_mut_ptr();
    let mut avail_out = out.len();
    // SAFETY: `next_out` points to `avail_out` writable bytes of `out`, which
    // is not otherwise referenced during the call.
    let status = unsafe {
        api::jxl_encoder_process_output(&mut *raw, &raw mut next_out, &raw mut avail_out)
    };
    assert_eq!(status, Status::SUCCESS);
    out.truncate(out.len() - avail_out);
    // SAFETY: `raw` came from jxl_encoder_create and is not used again.
    unsafe { api::jxl_encoder_destroy(raw) };
    let trace = hwy::encoder_trace_stop();
    hwy::set_encoder_simd_for_test(None);
    (out, trace)
}

#[test]
fn enc_public_photographs_match_the_original_target() {
    let Ok(paths) = std::env::var("JXL_ENCODER_GATE_PHOTOS") else {
        println!("JXL_ENCODER_GATE_PHOTOS is not set: no photograph is compared");
        return;
    };
    let _guard = lock();
    let mut lossy = Vec::new();
    let mut lossless = Vec::new();
    for path in paths.split(':').filter(|p| !p.is_empty()) {
        let image = read_ppm(path);
        let (w, h) = (image.0, image.1);
        let mut rects = vec![(0, 0, w, h)];
        for (cw, ch) in [(1024, 768), (1031, 517), (257, 255)] {
            if cw <= w && ch <= h {
                rects.push(((w - cw) / 2, (h - ch) / 3, cw, ch));
            }
        }
        for (k, &rect) in rects.iter().enumerate() {
            for distance in [1.0f32, 2.0] {
                // Alpha on the crops (the whole image keeps the engine's RGB).
                for alpha in [false, k > 0] {
                    lossy.push((path.to_string(), rect, alpha, distance));
                }
            }
            if k > 0 || w * h <= 4_000_000 {
                lossless.push((path.to_string(), rect, false));
                lossless.push((path.to_string(), rect, true));
            }
        }
        lossy.dedup();
        let mut requests = Vec::new();
        let mut port = Vec::new();
        let mut scalar_differs = 0;
        let mut sites = BTreeMap::new();
        for (p, rect, alpha, distance) in lossy.drain(..) {
            requests.push(format!(
                "esel-ppm {p} {} {} {} {} {} {}",
                rect.0,
                rect.1,
                rect.2,
                rect.3,
                bits(distance),
                u8::from(alpha)
            ));
            let pixels = crop(&image, rect, alpha);
            let (simd, trace) = port_photo(&pixels, rect.2, rect.3, alpha, distance, true);
            let (scalar, _) = port_photo(&pixels, rect.2, rect.3, alpha, distance, false);
            scalar_differs += usize::from(scalar != simd);
            for (site, count) in trace {
                *sites.entry(site).or_default() += count;
            }
            port.push(((rect, alpha, distance), simd));
        }
        let originals = simd_common::oracle(&requests);
        let mut differing = Vec::new();
        for ((case, simd), original) in port.iter().zip(&originals) {
            let (status, hex) = original.split_once(' ').unwrap();
            assert_eq!(status, "0", "the original encodes {path} {case:?}");
            let original = driver::unhex(hex);
            if original != *simd {
                let byte = simd.iter().zip(&original).position(|(a, b)| a != b);
                differing.push(format!(
                    "{path} {case:?}: first differing byte {byte:?}, sizes {}/{}",
                    simd.len(),
                    original.len()
                ));
            }
        }
        let result = FamilyResult {
            histories: port.len(),
            exact: port.len() - differing.len(),
            scalar_differs,
            differing,
            sites,
            ..FamilyResult::default()
        };
        println!(
            "photograph {path} ({w}x{h}) lossy on {}: {} encodes, {} byte-identical with the \
             original's {} build; the scalar encoder path differs in {} of {}",
            target().name(),
            result.histories,
            result.exact,
            target().name(),
            result.scalar_differs,
            result.histories
        );
        for line in &result.differing {
            println!("  DIFFERS {line}");
        }
        let expected: Vec<&str> = LOSSY_SITES.iter().chain(ALPHA_SITES).copied().collect();
        assert_reached("photographs", &result, &expected);
        assert_family("photographs", &result);
        // Lossless effort 2 on the same pixels.
        let mut requests = Vec::new();
        let mut port = Vec::new();
        let mut scalar_differs = 0;
        for (_, rect, alpha) in lossless.drain(..) {
            let input = crop(&image, rect, alpha);
            let shape = (rect.2, rect.3, false, 8, alpha);
            let (simd, _) = port_lossless(shape, &input, true);
            let (scalar, _) = port_lossless(shape, &input, false);
            scalar_differs += usize::from(scalar != simd);
            requests.push(format!(
                "ss-image{} {} {} 0 8 0 {} {}",
                if alpha { "-alpha" } else { "" },
                rect.2,
                rect.3,
                driver::hex(&input),
                driver::hex(&simd)
            ));
            port.push(((rect, alpha), simd));
        }
        let originals = simd_common::oracle(&requests);
        let mut differing = 0;
        for ((case, simd), original) in port.iter().zip(&originals) {
            let fields: Vec<&str> = original.split_whitespace().collect();
            assert_eq!(
                fields[0], "0",
                "the original encodes {path} {case:?} losslessly"
            );
            if driver::unhex(fields[1]) != *simd {
                differing += 1;
                println!("  DIFFERS lossless {path} {case:?}");
            }
        }
        println!(
            "photograph {path} lossless e2 on {}: {} encodes, {} byte-identical; the scalar \
             encoder path differs in {scalar_differs} of {}",
            target().name(),
            port.len(),
            port.len() - differing,
            port.len()
        );
        assert_eq!(
            differing, 0,
            "lossless photograph encodes differ from the original"
        );
    }
}

// ---------------------------------------------------------------------------
// Encoder step 1 checkpoints: the state after LossyFrameHeuristics,
// InitializePassesEncoder and ComputeARHeuristics (`es1-ckpt`, through the
// read-only observer `jxl::jxl::enc_checkpoints`), word for word against the
// original's SIMD build. The rendering and the cases are copied from
// tests/parity/encoder_step1_checkpoints.rs (keep them identical in meaning).
mod checkpoints {
    use super::{driver, hwy, simd_common, target};
    use jxl::jxl::ac_context::BlockCtxMap;
    use jxl::jxl::ac_strategy::AcStrategyImage;
    use jxl::jxl::dct_util::AcImage;
    use jxl::jxl::enc_cache::PassesEncoderState;
    use jxl::jxl::enc_checkpoints::{Checkpoint, set_checkpoint_observer};
    use jxl::jxl::frame_header::FrameHeader;
    use jxl::jxl::image::{Image3F, ImageF, Plane};
    use jxl::jxl::quantizer::Quantizer;
    use std::cell::RefCell;
    use std::fmt::Write as _;
    use std::rc::Rc;

    fn plane(out: &mut String, plane: &ImageF) {
        write!(out, " P {} {}", plane.xsize(), plane.ysize()).unwrap();
        for y in 0..plane.ysize() {
            for v in &plane.row(y)[..plane.xsize()] {
                write!(out, " {}", v.to_bits()).unwrap();
            }
        }
    }

    fn ints<T: jxl::jxl::image::Sample + Into<i64>>(out: &mut String, plane: &Plane<T>) {
        write!(out, " I {} {}", plane.xsize(), plane.ysize()).unwrap();
        for y in 0..plane.ysize() {
            for &v in &plane.row(y)[..plane.xsize()] {
                write!(out, " {}", v.into()).unwrap();
            }
        }
    }

    fn strategy(out: &mut String, acs: &AcStrategyImage) {
        out.push_str(" S");
        for y in 0..acs.ysize() {
            let row = acs.const_row(y, 0);
            for x in 0..acs.xsize() {
                let a = row.get(x);
                write!(
                    out,
                    " {}:{}",
                    a.raw_strategy(),
                    u8::from(a.is_first_block())
                )
                .unwrap();
            }
        }
    }

    fn quantizer(out: &mut String, q: &Quantizer) {
        let params = q.get_params();
        write!(
            out,
            " Q {} {} {} {} {}",
            params.global_scale,
            params.quant_dc,
            q.scale().to_bits(),
            q.inv_global_scale().to_bits(),
            q.inv_quant_dc().to_bits()
        )
        .unwrap();
        for c in 0..3 {
            write!(out, " {}", q.mul_dc()[c].to_bits()).unwrap();
        }
        for c in 0..3 {
            write!(out, " {}", q.inv_mul_dc()[c].to_bits()).unwrap();
        }
    }

    fn block_ctx_map(out: &mut String, map: &BlockCtxMap) {
        write!(out, " K {} {}", map.num_ctxs, map.num_dc_ctxs).unwrap();
        for t in &map.dc_thresholds {
            write!(out, " {}", t.len()).unwrap();
            for v in t {
                write!(out, " {v}").unwrap();
            }
        }
        write!(out, " {}", map.qf_thresholds.len()).unwrap();
        for v in &map.qf_thresholds {
            write!(out, " {v}").unwrap();
        }
        write!(out, " {}", map.ctx_map.len()).unwrap();
        for v in &map.ctx_map {
            write!(out, " {v}").unwrap();
        }
    }

    /// The oracle's rendering of one checkpoint.
    fn checkpoint(
        out: &mut String,
        checkpoint: Checkpoint,
        header: &FrameHeader<'_>,
        state: &PassesEncoderState<'_>,
        image: &Image3F,
    ) {
        let shared = &state.shared;
        match checkpoint {
            Checkpoint::LossyFrameHeuristics => {
                write!(
                    out,
                    " H 0 F {} {} {} {}",
                    u8::from(header.loop_filter.gab),
                    header.loop_filter.epf_iters,
                    header.x_qm_scale,
                    header.b_qm_scale
                )
                .unwrap();
                for c in 0..3 {
                    plane(out, image.plane(c));
                }
                plane(out, &state.initial_quant_masking1x1);
                strategy(out, &shared.ac_strategy);
                ints(out, &shared.raw_quant_field);
                quantizer(out, &shared.quantizer);
                {
                    let base = shared.cmap.base();
                    write!(
                        out,
                        " B {} {} {} {} {} {}",
                        base.y_to_x_dc(),
                        base.y_to_b_dc(),
                        base.raw_color_factor(),
                        base.color_scale().to_bits(),
                        base.base_correlation_x().to_bits(),
                        base.base_correlation_b().to_bits()
                    )
                    .unwrap();
                    for v in base.dc_factors() {
                        write!(out, " {}", v.to_bits()).unwrap();
                    }
                }
                ints(out, &shared.cmap.ytox_map);
                ints(out, &shared.cmap.ytob_map);
                ints(out, &shared.epf_sharpness);
                block_ctx_map(out, &shared.block_ctx_map);
                out.push_str(" M");
                for e in shared.matrices.encodings() {
                    write!(out, " {},{}", e.mode() as i32, e.predefined).unwrap();
                }
                for c in 0..3 {
                    write!(out, " {}", shared.matrices.dc_quant(c).to_bits()).unwrap();
                }
                write!(
                    out,
                    " X {} {}",
                    u8::from(shared.image_features.splines.borrow().has_any()),
                    u8::from(shared.image_features.patches.borrow().has_any())
                )
                .unwrap();
            }
            Checkpoint::InitializePassesEncoder => {
                out.push_str(" I 0");
                for c in 0..3 {
                    plane(out, shared.dc_storage.plane(c));
                }
                ints(out, &shared.quant_dc);
                // One row per group; the first 64 words per block are written.
                let frame_dim = &shared.frame_dim;
                write!(out, " C {}", state.coeffs.len()).unwrap();
                for coeffs in &state.coeffs {
                    let Some(AcImage::I32(image)) = coeffs.as_deref() else {
                        panic!("the encoder's coefficients are 32-bit");
                    };
                    for g in 0..frame_dim.num_groups {
                        let r = frame_dim.block_group_rect(g);
                        let n = r.xsize() * r.ysize() * 64;
                        write!(out, " G {g} {n}").unwrap();
                        for c in 0..3 {
                            for v in &image.plane(c).row(g)[..n] {
                                write!(out, " {v}").unwrap();
                            }
                        }
                    }
                }
            }
            Checkpoint::ArHeuristics => {
                out.push_str(" A 0");
                ints(out, &shared.epf_sharpness);
            }
        }
    }

    /// One checkpoint history: float32 samples of `content`, enum sRGB or linear
    /// sRGB, one frame, CloseInput and enough output.
    struct Case {
        xsize: usize,
        ysize: usize,
        gray: bool,
        linear: bool,
        content: u32,
        distance: f32,
        intensity: f32,
    }

    impl Case {
        fn request(&self) -> String {
            format!(
                "es1-ckpt {} {} {} {} {} {} {}",
                self.xsize,
                self.ysize,
                u8::from(self.gray),
                u8::from(self.linear),
                self.content,
                self.distance.to_bits(),
                self.intensity.to_bits()
            )
        }

        /// The same frame as a public encoder history (tests/parity/encoder_public.rs
        /// protocol): bits 32, exponent 8, the color encoding, the distance, an
        /// interleaved native float32 frame with the generated content.
        fn history(&self) -> String {
            let channels = if self.gray { 1 } else { 3 };
            let mut info = format!("Bw:{},h:{},bits:32,exp:8", self.xsize, self.ysize);
            if self.gray {
                info.push_str(",c:1");
            }
            if self.intensity != 0.0 {
                write!(info, ",it:{}", self.intensity.to_bits()).unwrap();
            }
            let color = if self.linear { 'g' } else { 'G' };
            let size = self.xsize * self.ysize * channels * 4;
            // ProcessOutput with 0, 1, 7 and 64 bytes, then doubling.
            let mut schedule = String::from("p0 p1 p7");
            let (mut sum, mut avail) = (8usize, 64usize);
            while sum <= size + 2048 {
                write!(schedule, " p{avail}").unwrap();
                sum += avail;
                avail *= 2;
            }
            format!(
                "epub-run 0 0 {info} {color}{} s- d0,{} a0,{channels},0,0,0,{size},{},{},{} Z {schedule}",
                u8::from(self.gray),
                self.distance.to_bits(),
                self.content,
                self.xsize,
                self.ysize
            )
        }
    }

    /// The codestream that one epub-run observation's ProcessOutput calls wrote.
    fn public_bytes(observation: &str) -> String {
        let mut bytes = String::new();
        let mut last = "";
        for field in observation.split(' ') {
            let Some(result) = field.strip_prefix("p:") else {
                continue;
            };
            let parts: Vec<&str> = result.split(',').collect();
            let written: usize = parts[1].parse().unwrap();
            let hex = parts[3].split('/').next().unwrap();
            bytes.push_str(&hex[..2 * written]);
            last = parts[0];
        }
        assert_eq!(last, "0", "the encoder finishes");
        bytes
    }

    /// The port's checkpoints of one history with the encoder's selection
    /// forced, and its public observation.
    fn port(history: &str, simd: bool) -> (String, String) {
        hwy::set_encoder_simd_for_test(Some(simd));
        let text = Rc::new(RefCell::new(String::from("E 0")));
        let sink = Rc::clone(&text);
        let previous = set_checkpoint_observer(Some(Box::new(move |c, h, s, i| {
            checkpoint(&mut sink.borrow_mut(), c, h, s, i);
        })));
        assert!(previous.is_none());
        let mut args = history.split_whitespace();
        assert_eq!(args.next(), Some("epub-run"));
        let rust = driver::observe(&mut args);
        set_checkpoint_observer(None);
        hwy::set_encoder_simd_for_test(None);
        assert!(rust.admission.is_none(), "admitted history: {history}");
        let checkpoints = text.borrow().clone();
        (checkpoints, rust.text)
    }

    #[test]
    fn enc_public_step1_checkpoints_match_the_original_target() {
        let _guard = super::lock();
        let mut sizes = vec![(1, 1), (7, 7), (8, 8), (9, 17)];
        sizes.extend((41..=48).map(|x| (x, 15)));
        sizes.extend([(63, 129), (256, 256), (257, 255)]);
        let distances = [1.0f32, 0.5, 1.5, 2.9];
        let mut cases = Vec::new();
        let mut variant = 0usize;
        for &(xsize, ysize) in &sizes {
            for gray in [false, true] {
                cases.push(Case {
                    xsize,
                    ysize,
                    gray,
                    linear: variant % 3 == 1,
                    content: 1 + (variant % 4) as u32,
                    distance: distances[variant % 4],
                    intensity: if variant % 5 == 2 { 100.0 } else { 0.0 },
                });
                variant += 1;
            }
        }
        // Eight groups at d1.0 and d2.9.
        for (gray, distance) in [(false, 1.0f32), (true, 2.9)] {
            cases.push(Case {
                xsize: 512,
                ysize: 1024,
                gray,
                linear: false,
                content: 4,
                distance,
                intensity: 0.0,
            });
        }
        let requests: Vec<String> = cases.iter().map(Case::request).collect();
        let histories: Vec<String> = cases.iter().map(Case::history).collect();
        let originals = simd_common::oracle(&requests);
        let public = simd_common::oracle(&histories);
        let (mut words, mut scalar_differs, mut differing) = (0usize, 0usize, Vec::new());
        for ((request, history), (cpp, public)) in requests
            .iter()
            .zip(&histories)
            .zip(originals.iter().zip(&public))
        {
            let (expected, bytes) = cpp.rsplit_once(" W ").expect("checkpoint bytes");
            assert_eq!(
                bytes,
                public_bytes(public),
                "the oracle run equals the public encode: {request}"
            );
            let (simd, simd_public) = port(history, true);
            let (scalar, _) = port(history, false);
            scalar_differs += usize::from(scalar != simd);
            if simd != expected || simd_public != *public {
                let r: Vec<&str> = simd.split(' ').collect();
                let c: Vec<&str> = expected.split(' ').collect();
                let at = r.iter().zip(&c).position(|(a, b)| a != b);
                differing.push(format!(
                    "{request}: first differing checkpoint word {at:?}, public observation equal: {}",
                    simd_public == *public
                ));
            }
            words += expected.split(' ').count();
        }
        println!(
            "step1-checkpoints on {}: {} histories ({words} words), {} match the original's {} \
             build; the scalar encoder path differs in {scalar_differs} of {}",
            target().name(),
            cases.len(),
            cases.len() - differing.len(),
            target().name(),
            cases.len()
        );
        for line in &differing {
            println!("  DIFFERS {line}");
        }
        assert!(
            differing.is_empty(),
            "checkpoint histories differ from the original's {} build",
            target().name()
        );
    }
}
