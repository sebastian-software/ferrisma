// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use super::*;
#[path = "../common/output_stage.rs"]
mod rust_observer;
pub(super) fn handles(kind: &str) -> bool {
    rust_observer::handles(kind)
}
pub(super) fn observe(kind: &str, args: &mut std::str::SplitWhitespace<'_>) -> String {
    rust_observer::observe(kind, args)
}
const FINITE: [u32; 9] = [
    0, 0x80000000, 0x3f000000, 0x3f800000, 0xbf800000, 0x3fa00000, 0x3e800000, 0x32800000,
    0x3f400000,
];

#[derive(Clone, Copy)]
struct Descriptor {
    kind: u32,
    bits: usize,
    channels: usize,
    mode: usize,
    padding: usize,
    endian: u32,
}
fn descriptor(kind: u32, channels: usize, mode: usize, endian: u32) -> Descriptor {
    Descriptor {
        kind,
        bits: if kind == 2 { 8 } else { 16 },
        channels,
        mode,
        padding: 3,
        endian,
    }
}
#[expect(
    clippy::too_many_arguments,
    reason = "Data-only original stage fixture controls."
)]
fn request(
    width: usize,
    height: usize,
    has_alpha: bool,
    unpremul: bool,
    alpha: usize,
    orientation: u32,
    manager: usize,
    outputs: &[Descriptor],
    actions: &[Vec<usize>],
    words: &[u32; 9],
) -> String {
    let mut text = format!(
        "wo-run {width} {height} {} {} {alpha} {orientation} {} {manager}",
        u8::from(has_alpha),
        u8::from(unpremul),
        outputs.len() - 1
    );
    for d in outputs {
        write!(
            text,
            " {} {} {} {} {} {}",
            d.kind, d.bits, d.channels, d.mode, d.padding, d.endian
        )
        .unwrap();
    }
    write!(text, " {}", actions.len()).unwrap();
    for action in actions {
        for word in action {
            write!(text, " {word}").unwrap();
        }
    }
    for word in words {
        write!(text, " {word}").unwrap();
    }
    text
}
fn exact(cases: Vec<String>) {
    if let Some(path) = std::env::var_os("JXL_OUTPUT_STAGE_CAPTURE") {
        static LOCK: std::sync::Mutex<()> = std::sync::Mutex::new(());
        let _lock = LOCK.lock().unwrap();
        let mut file = std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(path)
            .unwrap();
        file.write_all((cases.join("\n") + "\n").as_bytes())
            .unwrap();
    }
    for chunk in cases.chunks(4) {
        for (request, source) in chunk.iter().zip(original_observations(chunk)) {
            let rust = super::observe(request);
            if rust != source {
                let a: Vec<_> = rust.split_whitespace().collect();
                let b: Vec<_> = source.split_whitespace().collect();
                let differences: Vec<_> = a
                    .iter()
                    .zip(&b)
                    .enumerate()
                    .filter(|(_, (a, b))| a != b)
                    .take(8)
                    .collect();
                panic!(
                    "{request}: fields {}/{}; differences {differences:?}",
                    a.len(),
                    b.len()
                );
            }
        }
    }
    println!("{} C++/Rust differential cases matched", cases.len());
}

#[test]
fn actual_output_factory_buffer_conversions_preserve_all_orientations_alpha_extra_channels_and_retry_owners()
 {
    let mut cases = Vec::new();
    for orientation in 1..=8 {
        for kind in [0, 2, 3, 5] {
            for channels in 1..=4 {
                for has_alpha in [false, true] {
                    for unpremul in [false, true] {
                        let outputs = [
                            descriptor(kind, channels, 0, orientation % 4),
                            descriptor(5, 1, 0, (orientation + 1) % 4),
                            descriptor(0, 1, 1, (orientation + 2) % 4),
                        ];
                        let mut actions = vec![vec![0, 2]];
                        for y in 0..7 {
                            actions.push(vec![1, 0, 0, 14, 0, y, y % 2]);
                        }
                        actions.extend([vec![0, 1], vec![1, 0, 0, 12, 2, 1, 0], vec![4, 2]]);
                        cases.push(request(
                            9,
                            7,
                            has_alpha,
                            unpremul,
                            3,
                            orientation,
                            0,
                            &outputs,
                            &actions,
                            &FINITE,
                        ));
                    }
                }
            }
        }
    }
    exact(cases);
}

#[test]
fn actual_output_callbacks_keep_chunk_1024_coordinates_opaque_tokens_and_physical_payloads() {
    let mut cases = Vec::new();
    for orientation in 1..=8 {
        for width in [0, 1, 2, 31, 32, 33, 1023, 1024, 1025, 1031, 2049] {
            for kind in [0, 2, 3, 5] {
                for mode in [1, 2] {
                    let outputs = [
                        descriptor(kind, 4, mode, orientation % 4),
                        descriptor(3, 1, 1, (orientation + 1) % 4),
                    ];
                    let actions = vec![
                        vec![0, 2],
                        vec![1, 0, 0, width + 7, 0, 0, 0],
                        vec![1, 0, 0, width + 7, 0, 2, 1],
                        vec![1, 0, 0, width + 7, width, 0, 0],
                        vec![5, 0],
                        vec![0, 1],
                        vec![1, 0, 0, 17, 1, 1, 0],
                        vec![4, 2],
                    ];
                    cases.push(request(
                        width,
                        3,
                        true,
                        true,
                        3,
                        orientation,
                        0,
                        &outputs,
                        &actions,
                        &FINITE,
                    ));
                }
            }
        }
    }
    exact(cases);
}

#[test]
fn unsigned_dithering_keeps_every_period_cell_rounding_threshold_bit_depth_and_exceptional_input() {
    let mut cases = Vec::new();
    for step in -512..=512 {
        let value = (0.5 + step as f32 / 262144.0).to_bits();
        let outputs = [descriptor(2, 4, 0, 0)];
        let mut actions = vec![vec![0, 1]];
        for y in 0..32 {
            actions.push(vec![1, 0, 0, 33, 31, y, 0]);
        }
        cases.push(request(
            96,
            32,
            false,
            false,
            usize::MAX,
            1,
            0,
            &outputs,
            &actions,
            &[value; 9],
        ));
    }
    let patterns = [
        FINITE,
        [
            0x7f800000, 0xff800000, 0x7fc12345, 0x7fa23456, 0xffc98765, 0xffa12345, 1, 0x80000001,
            0x3f000001,
        ],
    ];
    for kind in [2, 3] {
        for bits in [0, 1, 2, 4, 7, 8, 9, 10, 12, 15, 16, 17, 24, 30] {
            for endian in 0..=3 {
                for words in patterns {
                    let mut output = descriptor(kind, 4, 0, endian);
                    output.bits = bits;
                    cases.push(request(
                        37,
                        3,
                        true,
                        false,
                        3,
                        1,
                        0,
                        &[output, descriptor(3, 1, 3, 0)],
                        &[
                            vec![0, 1],
                            vec![1, 0, 0, 37, 0, 0, 0],
                            vec![1, 0, 0, 37, 0, 2, 0],
                        ],
                        &words,
                    ));
                }
            }
        }
    }
    exact(cases);
}

#[test]
fn output_prepare_failures_retries_and_manager_mutation_keep_partial_owners_and_callback_order() {
    let mut cases = Vec::new();
    for main_mode in 0..=3 {
        for orientation in [1, 2, 5, 8] {
            let outputs = [
                descriptor(0, 4, main_mode, 0),
                descriptor(5, 1, 1, 1),
                descriptor(2, 1, 0, 2),
                descriptor(3, 1, 1, 0),
            ];
            for failure in 11..=26 {
                let actions = [
                    vec![0, 2],
                    vec![1, 0, 0, 13, 1, 1, 0],
                    vec![2, failure],
                    vec![0, 3],
                    vec![1, 0, 0, 20, 2, 1, 0],
                    vec![4, 2],
                    vec![2, 0],
                    vec![0, 1],
                    vec![1, 0, 0, 17, 0, 0, 0],
                ];
                cases.push(request(
                    17,
                    3,
                    true,
                    true,
                    3,
                    orientation,
                    0,
                    &outputs,
                    &actions,
                    &FINITE,
                ));
            }
            let base_inits = if main_mode == 1 || main_mode == 2 {
                3
            } else if main_mode == 0 {
                2
            } else {
                0
            };
            for failure in base_inits + 1..=base_inits + 4 {
                let actions = [
                    vec![0, 2],
                    vec![3, failure],
                    vec![0, 3],
                    vec![1, 0, 0, 17, 17, 0, 0],
                    vec![4, 2],
                    vec![3, 0],
                    vec![0, 1],
                    vec![1, 0, 0, 17, 0, 0, 0],
                ];
                cases.push(request(
                    17,
                    3,
                    true,
                    true,
                    3,
                    orientation,
                    0,
                    &outputs,
                    &actions,
                    &FINITE,
                ));
            }
            for manager in 0..=2 {
                let actions = [
                    vec![0, 0],
                    vec![1, 0, 0, 0, 0, 0, 0],
                    vec![0, 2],
                    vec![1, 1, 0, 17, 0, 0, 0],
                    vec![1, 0, 1, 17, 0, 0, 0],
                    vec![1, 0, 0, 17, 0, 3, 0],
                    vec![1, 0, 0, 17, 17, 0, 0],
                ];
                cases.push(request(
                    17,
                    3,
                    true,
                    true,
                    3,
                    orientation,
                    manager,
                    &outputs,
                    &actions,
                    &FINITE,
                ));
            }
        }
    }
    exact(cases);
}

#[test]
fn raw_float_and_half_output_keeps_original_signaling_words_and_unpremul_checkpoints() {
    let mut cases = Vec::new();
    let words = [
        0x7fc12345, 0xffc98765, 0x7fa23456, 0xffa12345, 0x7f800000, 0xff800000, 0, 0x80000000, 1,
    ];
    for orientation in 1..=8 {
        for kind in [0, 5] {
            for channels in 1..=4 {
                for unpremul in [false, true] {
                    for mode in [0, 1, 2] {
                        let outputs = [
                            descriptor(kind, channels, mode, orientation % 4),
                            descriptor(kind, 1, 1, (orientation + 1) % 4),
                        ];
                        cases.push(request(
                            35,
                            3,
                            true,
                            unpremul,
                            3,
                            orientation,
                            0,
                            &outputs,
                            &[
                                vec![0, 1],
                                vec![1, 0, 0, 35, 0, 0, 0],
                                vec![1, 0, 0, 35, 0, 2, 0],
                            ],
                            &words,
                        ));
                    }
                }
            }
        }
    }
    exact(cases);
}

#[test]
fn jxl_memory_manager_keeps_its_own_null_default_pair_alias_and_partial_failure_rules() {
    let mut cases = Vec::new();
    for null in 0..=1 {
        for alias in 0..=1 {
            for opaque in 0..=1 {
                for alloc in 0..=1 {
                    for free in 0..=1 {
                        for size in [0, 1, 3, 16, 128, 4096] {
                            cases.push(format!(
                                "wo-mm {null} {alias} {opaque} {alloc} {free} {size}"
                            ));
                        }
                    }
                }
            }
        }
    }
    exact(cases);
}

#[test]
fn stage_uses_the_retained_manager_pointer_current_function_and_opaque_fields() {
    let mut cases = Vec::new();
    for orientation in 1..=8 {
        for mode in [0, 1, 2] {
            let outputs = [descriptor(0, 4, mode, 1), descriptor(5, 1, 1, 2)];
            let actions = [
                vec![0, 2],
                vec![1, 0, 0, 35, 0, 0, 0],
                vec![4, 2],
                vec![6, 2],
                vec![0, 3],
                vec![1, 0, 0, 35, 0, 2, 2],
                vec![4, 1],
                vec![0, 1],
                vec![1, 0, 0, 17, 2, 1, 0],
                vec![6, 1],
                vec![4, 2],
            ];
            cases.push(request(
                35,
                3,
                true,
                true,
                3,
                orientation,
                0,
                &outputs,
                &actions,
                &FINITE,
            ));
        }
    }
    exact(cases);
}

#[test]
fn alpha_channel_selection_and_source_ignored_format_branches_keep_modes_guards_and_owners() {
    let mut cases = Vec::new();
    for alpha in 0..7 {
        for channels in 1..=4 {
            for orientation in 1..=8 {
                let outputs = [
                    descriptor(0, channels, 2, 1),
                    descriptor(5, 1, 0, 2),
                    descriptor(2, 1, 3, 0),
                    descriptor(3, 1, 1, 1),
                    descriptor(0, 1, 0, 0),
                ];
                cases.push(request(
                    7,
                    3,
                    true,
                    true,
                    alpha,
                    orientation,
                    0,
                    &outputs,
                    &[
                        vec![0, 2],
                        vec![1, 0, 0, 7, 0, 0, 0],
                        vec![1, 0, 0, 7, 0, 2, 1],
                    ],
                    &FINITE,
                ));
            }
        }
    }
    for kind in [1, 4, 6, 7] {
        for channels in 0..=4 {
            for orientation in [0, 1, 2, 5, 8, 9, u32::MAX] {
                let outputs = [descriptor(kind, channels, 2, 0)];
                cases.push(request(
                    7,
                    3,
                    false,
                    false,
                    usize::MAX,
                    orientation,
                    0,
                    &outputs,
                    &[
                        vec![0, 2],
                        vec![1, 0, 0, 7, 0, 0, 0],
                        vec![0, 1],
                        vec![1, 0, 0, 7, 0, 2, 0],
                    ],
                    &FINITE,
                ));
            }
        }
        for channels in [5, 17] {
            cases.push(request(
                7,
                3,
                false,
                false,
                usize::MAX,
                1,
                0,
                &[descriptor(kind, channels, 2, 0)],
                &[vec![0, 1], vec![1, 0, 0, 7, 0, 0, 0]],
                &FINITE,
            ));
        }
    }
    for orientation in [0, 1, 2, 5, 8, 9, u32::MAX] {
        cases.push(request(
            0,
            1,
            true,
            true,
            usize::MAX,
            orientation,
            0,
            &[descriptor(0, 4, 1, 0)],
            &[vec![0, 0], vec![1, 0, 0, 0, 0, 0, 0]],
            &FINITE,
        ));
    }
    exact(cases);
}

// Depth 31's shift is defined. The original Highway conversion is undefined
// only after the reached value rounds to 2^31; the independent original-only
// audit retains positive diagnostics for those requests and oversized shifts.
#[test]
fn unsigned_depth_31_keeps_every_defined_control_below_the_original_cast_boundary() {
    let mut cases = Vec::new();
    for kind in [2, 3] {
        for channels in 1..=4 {
            for orientation in [1, 2, 5, 8] {
                for mode in [0, 1] {
                    for word in [0, 0x3f000000, 0x3f7fffff] {
                        let mut output = descriptor(kind, channels, mode, 0);
                        output.bits = 31;
                        // Without a real alpha input the source synthesizes 1.0,
                        // which crosses the depth-31 conversion boundary. Retain
                        // those original requests in the source-only census and
                        // compare a real, controlled alpha plane here instead.
                        let outputs = if channels % 2 == 0 {
                            vec![output, descriptor(0, 1, 3, 0)]
                        } else {
                            vec![output]
                        };
                        cases.push(request(
                            3,
                            2,
                            channels % 2 == 0,
                            false,
                            if channels % 2 == 0 { 3 } else { usize::MAX },
                            orientation,
                            0,
                            &outputs,
                            &[
                                vec![0, 2],
                                vec![1, 0, 0, 3, 0, 0, 0],
                                vec![1, 0, 0, 3, 0, 1, 1],
                            ],
                            &[word; 9],
                        ));
                    }
                }
            }
        }
    }
    exact(cases);
}
