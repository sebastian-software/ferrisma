//! WP4 differential test: the alpha processing of `src/dsp/alpha_processing.c`.
//! One script of commands drives both the C probe
//! `tools/oracle/probes/wp4/alpha_processing.c` and the Rust translation: the
//! premultiplication and un-premultiplication of ARGB rows, single-value rows,
//! planes and RGBA4444 planes with edge alpha values; the dispatch and
//! extraction of alpha planes, including the all-opaque results; the packing of
//! RGB planes; and the alpha tests and replacement, over seeded inputs.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use webp::dsp::alpha_processing::{
    webp_alpha_replace, webp_apply_alpha_multiply, webp_apply_alpha_multiply4444,
    webp_dispatch_alpha, webp_dispatch_alpha_to_green, webp_extract_alpha, webp_extract_green,
    webp_has_alpha8b, webp_has_alpha32b, webp_mult_argb_row, webp_mult_argb_rows, webp_mult_row,
    webp_mult_rows, webp_pack_rgb,
};

/// The sentinel that the probe fills destinations with.
const SENTINEL: u8 = 0xaa;
/// Alpha values with special behavior: transparent, nearly so, and opaque.
const EDGE_ALPHA: [u8; 9] = [0, 1, 2, 3, 127, 128, 200, 254, 255];

/// SplitMix64, a small deterministic generator for the script.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn bytes(&mut self, count: usize) -> Vec<u8> {
        (0..count).map(|_| self.next_u64() as u8).collect()
    }

    /// Bytes drawn from the edge alpha values half the time.
    fn alphas(&mut self, count: usize) -> Vec<u8> {
        (0..count)
            .map(|_| {
                if self.next_u64() & 1 == 0 {
                    EDGE_ALPHA[self.next_u64() as usize % EDGE_ALPHA.len()]
                } else {
                    self.next_u64() as u8
                }
            })
            .collect()
    }

    /// ARGB words whose alpha byte comes from the edge values.
    fn argb(&mut self, count: usize) -> Vec<u32> {
        (0..count)
            .map(|_| {
                let alpha = if self.next_u64() & 1 == 0 {
                    EDGE_ALPHA[self.next_u64() as usize % EDGE_ALPHA.len()]
                } else {
                    self.next_u64() as u8
                };
                (u32::from(alpha) << 24) | (self.next_u64() as u32 & 0x00ff_ffff)
            })
            .collect()
    }

    fn words(&mut self, count: usize) -> Vec<u32> {
        (0..count).map(|_| self.next_u64() as u32).collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

fn words_hex(words: &[u32]) -> String {
    words.iter().map(|word| format!("{word:08x}")).collect()
}

/// A byte array as the probe reads it: its hex digits, or `-` when empty.
fn hex_arg(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        hex(bytes)
    }
}

fn words_arg(words: &[u32]) -> String {
    if words.is_empty() {
        "-".to_owned()
    } else {
        words_hex(words)
    }
}

fn field(key: &str, bytes: &[u8]) -> String {
    format!("{key}={}\n", hex(bytes))
}

fn word_field(key: &str, words: &[u32]) -> String {
    format!("{key}={}\n", words_hex(words))
}

fn int(value: usize) -> i32 {
    i32::try_from(value).expect("the value fits in int")
}

fn script() -> (String, String) {
    let mut rng = Rng(0x5eed_0006);
    let mut commands = String::from("init\n");
    let mut expected = String::new();
    let mut push = |command: String, output: String| {
        commands.push_str(&command);
        commands.push('\n');
        expected.push_str(&output);
    };

    // ARGB premultiplication and un-premultiplication, every width and both directions.
    for width in (0..=24).chain([63, 64]) {
        for inverse in [false, true] {
            let mut words = rng.argb(width);
            let command = format!(
                "mult_argb_row {width} {} {}",
                u8::from(inverse),
                words_arg(&words)
            );
            webp_mult_argb_row(&mut words, int(width), inverse);
            push(command, word_field("ptr", &words));
        }
    }

    // Single-value rows with edge alpha values.
    for width in 0..=24 {
        for inverse in [false, true] {
            let alpha = rng.alphas(width);
            let mut ptr = rng.bytes(width);
            let command = format!(
                "mult_row {width} {} {} {}",
                u8::from(inverse),
                hex_arg(&alpha),
                hex_arg(&ptr)
            );
            webp_mult_row(&mut ptr, &alpha, int(width), inverse);
            push(command, field("ptr", &ptr));
        }
    }

    // ARGB rows with a stride in words, and single-value rows with two strides.
    for &(width, stride, rows) in &[(3, 4, 2), (5, 6, 3), (8, 8, 4), (1, 2, 5)] {
        for inverse in [false, true] {
            let mut words = rng.argb(stride * rows);
            let command = format!(
                "mult_argb_rows {width} {stride} {rows} {} {}",
                u8::from(inverse),
                words_arg(&words)
            );
            webp_mult_argb_rows(&mut words, int(stride), int(width), int(rows), inverse);
            push(command, word_field("ptr", &words));
        }
    }
    for &(width, stride, astride, rows) in &[(3, 5, 4, 2), (7, 8, 7, 3), (6, 6, 9, 4)] {
        for inverse in [false, true] {
            let mut ptr = rng.bytes(stride * rows);
            let alpha = rng.alphas(astride * rows);
            let command = format!(
                "mult_rows {width} {stride} {astride} {rows} {} {} {}",
                u8::from(inverse),
                hex_arg(&ptr),
                hex_arg(&alpha)
            );
            webp_mult_rows(
                &mut ptr,
                int(stride),
                &alpha,
                int(astride),
                int(width),
                int(rows),
                inverse,
            );
            push(command, field("ptr", &ptr));
        }
    }

    // Premultiplied planes: RGBA, BGRA and ARGB, and RGBA4444.
    for &(width, height) in &[(1, 1), (3, 2), (5, 4), (9, 3), (16, 1)] {
        for alpha_first in [false, true] {
            let stride = 4 * width + 3;
            let mut rgba = rng.bytes(stride * height);
            for row in 0..height {
                for i in 0..width {
                    let alpha_index = row * stride + if alpha_first { 4 * i } else { 4 * i + 3 };
                    rgba[alpha_index] = EDGE_ALPHA[(row + i) % EDGE_ALPHA.len()];
                }
            }
            let command = format!(
                "apply {width} {height} {stride} {} {}",
                u8::from(alpha_first),
                hex_arg(&rgba)
            );
            webp_apply_alpha_multiply(&mut rgba, alpha_first, int(width), int(height), int(stride));
            push(command, field("rgba", &rgba));
        }
        let stride = 2 * width + 1;
        let mut rgba4444 = rng.bytes(stride * height);
        for row in 0..height {
            for i in 0..width {
                // The alpha nibble is the low nibble of the second byte.
                let index = row * stride + 2 * i + 1;
                rgba4444[index] = (rgba4444[index] & 0xf0) | (i as u8 % 16);
            }
        }
        let command = format!("apply4444 {width} {height} {stride} {}", hex_arg(&rgba4444));
        webp_apply_alpha_multiply4444(&mut rgba4444, int(width), int(height), int(stride));
        push(command, field("rgba", &rgba4444));
    }

    // Dispatch of an alpha plane into ARGB bytes, with opaque and non-opaque planes.
    for &(width, height) in &[(1, 1), (4, 2), (7, 3), (12, 5)] {
        for opaque in [false, true] {
            let astride = width + 2;
            let dstride = 4 * width + 3;
            let mut alpha = rng.bytes(astride * height);
            if opaque {
                alpha.iter_mut().for_each(|value| *value = 0xff);
            }
            let command = format!(
                "dispatch {width} {height} {astride} {dstride} {}",
                hex_arg(&alpha)
            );
            let mut dst = vec![SENTINEL; dstride * height];
            let ret = webp_dispatch_alpha(
                &alpha,
                int(astride),
                int(width),
                int(height),
                &mut dst,
                int(dstride),
            );
            push(
                command,
                format!("ret={}\n{}", u8::from(ret), field("dst", &dst)),
            );
        }
        // Dispatch into the green channel of 32-bit pixels.
        let astride = width + 1;
        let dstride = width + 2;
        let alpha = rng.bytes(astride * height);
        let command = format!(
            "dispatch_green {width} {height} {astride} {dstride} {}",
            hex_arg(&alpha)
        );
        let mut dst = vec![0xaaaa_aaaa_u32; dstride * height];
        webp_dispatch_alpha_to_green(
            &alpha,
            int(astride),
            int(width),
            int(height),
            &mut dst,
            int(dstride),
        );
        push(command, word_field("dst", &dst));
    }

    // Extraction of the alpha bytes of ARGB planes, with and without opaque alpha.
    for &(width, height) in &[(1, 1), (4, 2), (7, 3), (12, 5)] {
        for opaque in [false, true] {
            let argb_stride = 4 * width + 1;
            let astride = width + 3;
            let mut argb = rng.bytes(argb_stride * height);
            for row in 0..height {
                for i in 0..width {
                    argb[row * argb_stride + 4 * i] = if opaque {
                        0xff
                    } else {
                        EDGE_ALPHA[(row + i) % EDGE_ALPHA.len()]
                    };
                }
            }
            let command = format!(
                "extract {width} {height} {astride} {argb_stride} {}",
                hex_arg(&argb)
            );
            let mut alpha = vec![SENTINEL; astride * height];
            let ret = webp_extract_alpha(
                &argb,
                int(argb_stride),
                int(width),
                int(height),
                &mut alpha,
                int(astride),
            );
            push(
                command,
                format!("ret={}\n{}", u8::from(ret), field("alpha", &alpha)),
            );
        }
    }
    for size in [0_usize, 1, 2, 5, 17] {
        let words = rng.words(size);
        let command = format!("extract_green {size} {}", words_arg(&words));
        let mut alpha = vec![SENTINEL; size];
        webp_extract_green(&words, &mut alpha, int(size));
        push(command, field("alpha", &alpha));
    }

    // Packing of RGB and BGR planes into opaque ARGB pixels.
    for &step in &[3_usize, 4] {
        for len in 0..=12 {
            let r = rng.bytes(step * len);
            let g = rng.bytes(step * len);
            let b = rng.bytes(step * len);
            let command = format!(
                "pack_rgb {len} {step} {} {} {}",
                hex_arg(&r),
                hex_arg(&g),
                hex_arg(&b)
            );
            let mut out = vec![0xaaaa_aaaa_u32; len];
            webp_pack_rgb(&r, &g, &b, int(len), int(step), &mut out);
            push(command, word_field("out", &out));
        }
    }

    // The alpha tests over opaque and non-opaque bytes, at every length to 40.
    for len in 0..=40 {
        let mut bytes = vec![0xff_u8; len];
        let plain = rng.bytes(len);
        for (index, value) in bytes.iter_mut().enumerate() {
            if index % 2 == 1 {
                *value = plain[index];
            }
        }
        let command = format!("has8 {len} {}", hex_arg(&bytes));
        let ret = webp_has_alpha8b(&bytes, int(len));
        push(command, format!("ret={}\n", u8::from(ret)));

        let rgba = rng.bytes(4 * len);
        let mut rgba_opaque = rgba.clone();
        for index in 0..len {
            rgba_opaque[4 * index + 3] = if index % 3 == 0 {
                0xff
            } else {
                rgba[4 * index + 3]
            };
        }
        let command = format!("has32 {len} {}", hex_arg(&rgba_opaque));
        let ret = webp_has_alpha32b(&rgba_opaque, int(len));
        push(command, format!("ret={}\n", u8::from(ret)));
    }

    // Replacement of fully transparent pixels.
    for len in [0_usize, 1, 3, 8, 17] {
        let mut words = rng.argb(len);
        let color = rng.words(1)[0];
        let command = format!("replace {len} {color} {}", words_arg(&words));
        webp_alpha_replace(&mut words, int(len), color);
        push(command, word_field("src", &words));
    }
    (commands, expected)
}

/// The first line where the two outputs differ, for the failure message.
fn first_difference(expected: &str, actual: &str) -> String {
    let mut expected_lines = expected.lines();
    let mut actual_lines = actual.lines();
    for index in 0.. {
        match (expected_lines.next(), actual_lines.next()) {
            (None, None) => return "the outputs are equal".to_owned(),
            (Some(e), Some(a)) if e == a => continue,
            (e, a) => {
                return format!("line {index}: expected {e:?}, the C probe printed {a:?}");
            }
        }
    }
    unreachable!()
}

#[test]
fn alpha_processing_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("alpha_processing_matches_the_c_functions") else {
        return;
    };
    let (commands, expected) = script();
    let observations = expected.lines().count();
    assert!(
        observations > 250,
        "the script has {observations} observations"
    );
    let probe = oracle.probe("wp4", "alpha_processing");
    let output = probe.run(&[] as &[&str], commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    assert!(
        actual == expected,
        "the alpha processing differs from the C functions: {}",
        first_difference(&expected, &actual)
    );
}
