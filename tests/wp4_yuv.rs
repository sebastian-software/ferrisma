//! WP4 differential test: the YUV to RGB and RGB to YUV conversions
//! (`src/dsp/yuv.c` and `src/dsp/yuv.h`), the row samplers `WebPSamplers`, the
//! plane driver `WebPSamplerProcessPlane` and the ARGB, RGB and RGBA converters
//! to Y and UV. One script of commands drives both the C probe
//! `tools/oracle/probes/wp4/yuv.c` and the Rust translation. Every sampler mode
//! runs over row lengths from 0 to 65, the padded planes run over odd sizes and
//! strides, and the converters run over seeded inputs of every width.
//!
//! `WebPConvertRGBA32ToUV` runs on full-range `uint16_t` samples, where
//! upstream's SSE2 and SSE4.1 converters differ from the C function from 32768
//! up (`_mm_madd_epi16` reads them as negative); its callers pass at most 1020.
//! The reference oracle has no SIMD code, so that comparison selects the C
//! translation through `VP8GetCPUInfo = NULL`, as `-noasm` does, and restores
//! the pointer after each case (ADR 0008; `src/dsp/yuv_sse2.rs`).

#[path = "support/oracle.rs"]
mod oracle;

use core::ptr;
use std::sync::atomic::Ordering;

use oracle::Oracle;
use webp::abi::decode::VP8_GET_CPU_INFO;
use webp::dsp::yuv::{
    YUV_HALF, vp8_yuv_to_argb, vp8_yuv_to_bgr, vp8_yuv_to_bgra, vp8_yuv_to_rgb, vp8_yuv_to_rgb565,
    vp8_yuv_to_rgba, vp8_yuv_to_rgba4444, vp8rgb_to_u, vp8rgb_to_v, vp8rgb_to_y, vp8yuv_to_b,
    vp8yuv_to_g, vp8yuv_to_r, webp_convert_argb_to_uv, webp_convert_argb_to_y,
    webp_convert_bgr24_to_y, webp_convert_rgb24_to_y, webp_convert_rgba32_to_uv,
    webp_sampler_process_plane, webp_samplers,
};
use webp::webp::decode::WEBP_CSP_MODE;

/// The sampler modes, RGB through `MODE_rgbA_4444`; the YUV modes have no sampler.
const SAMPLER_MODES: u32 = 11;
/// The sentinel that the probe fills destinations with.
const SENTINEL: u8 = 0xaa;

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

    fn words32(&mut self, count: usize) -> Vec<u32> {
        (0..count).map(|_| self.next_u64() as u32).collect()
    }

    fn words16(&mut self, count: usize) -> Vec<u16> {
        (0..count).map(|_| self.next_u64() as u16).collect()
    }
}

/// The probe's input and the text that it must print, command by command.
#[derive(Default)]
struct Script {
    commands: String,
    expected: String,
}

impl Script {
    fn push(&mut self, command: &str, expected: String) {
        self.commands.push_str(command);
        self.commands.push('\n');
        self.expected.push_str(&expected);
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// A byte array as the probe reads it: its hex digits, or `-` when empty.
fn hex_arg(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        "-".to_owned()
    } else {
        hex(bytes)
    }
}

fn words32_arg(words: &[u32]) -> String {
    if words.is_empty() {
        return "-".to_owned();
    }
    words.iter().map(|word| format!("{word:08x}")).collect()
}

fn words16_arg(words: &[u16]) -> String {
    if words.is_empty() {
        return "-".to_owned();
    }
    words.iter().map(|word| format!("{word:04x}")).collect()
}

/// The probe's `key=value` line for a byte buffer.
fn field(key: &str, bytes: &[u8]) -> String {
    format!("{key}={}\n", hex(bytes))
}

/// Bytes per pixel of a sampler mode, as `WebPSamplers` writes them.
fn bytes_per_pixel(mode: u32) -> usize {
    match mode {
        0 | 2 => 3,
        5 | 6 | 10 => 2,
        _ => 4,
    }
}

fn row_case(script: &mut Script, rng: &mut Rng, mode: u32, len: usize) {
    let y = rng.bytes(len);
    let u = rng.bytes(len.div_ceil(2));
    let v = rng.bytes(len.div_ceil(2));
    let command = format!(
        "row {mode} {len} {} {} {}",
        hex_arg(&y),
        hex_arg(&u),
        hex_arg(&v)
    );
    let mut dst = vec![SENTINEL; 4 * len + 8];
    webp_samplers(
        WEBP_CSP_MODE(mode),
        &y,
        &u,
        &v,
        &mut dst,
        i32::try_from(len).expect("the length fits in int"),
    );
    script.push(&command, field("dst", &dst));
}

fn plane_case(
    script: &mut Script,
    rng: &mut Rng,
    mode: u32,
    width: usize,
    height: usize,
    strides: (usize, usize, usize),
) {
    let (y_stride, uv_stride, dst_stride) = strides;
    let uv_rows = height.div_ceil(2);
    let y = rng.bytes(y_stride * height);
    let u = rng.bytes(uv_stride * uv_rows);
    let v = rng.bytes(uv_stride * uv_rows);
    let command = format!(
        "plane {mode} {width} {height} {y_stride} {uv_stride} {dst_stride} {} {} {}",
        hex_arg(&y),
        hex_arg(&u),
        hex_arg(&v)
    );
    let mut dst = vec![SENTINEL; dst_stride * height];
    webp_sampler_process_plane(
        &y,
        i32::try_from(y_stride).expect("the stride fits in int"),
        &u,
        &v,
        i32::try_from(uv_stride).expect("the stride fits in int"),
        &mut dst,
        i32::try_from(dst_stride).expect("the stride fits in int"),
        i32::try_from(width).expect("the width fits in int"),
        i32::try_from(height).expect("the height fits in int"),
        |y, u, v, dst, len| webp_samplers(WEBP_CSP_MODE(mode), y, u, v, dst, len),
    );
    script.push(&command, field("dst", &dst));
}

fn argb_y_case(script: &mut Script, rng: &mut Rng, width: usize) {
    let argb = rng.words32(width);
    let command = format!("argb_y {width} {}", words32_arg(&argb));
    let mut y = vec![SENTINEL; width];
    webp_convert_argb_to_y(&argb, &mut y, i32::try_from(width).expect("int"));
    script.push(&command, field("y", &y));
}

fn argb_uv_case(script: &mut Script, rng: &mut Rng, width: usize, do_store: i32) {
    let argb = rng.words32(width);
    let uv = width.div_ceil(2);
    let u0 = rng.bytes(uv);
    let v0 = rng.bytes(uv);
    let command = format!(
        "argb_uv {width} {do_store} {} {} {}",
        words32_arg(&argb),
        hex_arg(&u0),
        hex_arg(&v0)
    );
    let (mut u, mut v) = (u0, v0);
    webp_convert_argb_to_uv(
        &argb,
        &mut u,
        &mut v,
        i32::try_from(width).expect("int"),
        do_store,
    );
    let expected = field("u", &u) + &field("v", &v);
    script.push(&command, expected);
}

fn rgb_y_case(script: &mut Script, rng: &mut Rng, width: usize, bgr: bool) {
    let rgb = rng.bytes(3 * width);
    let command = format!(
        "{} {width} {}",
        if bgr { "bgr_y" } else { "rgb_y" },
        hex_arg(&rgb)
    );
    let mut y = vec![SENTINEL; width];
    let width_int = i32::try_from(width).expect("int");
    if bgr {
        webp_convert_bgr24_to_y(&rgb, &mut y, width_int);
    } else {
        webp_convert_rgb24_to_y(&rgb, &mut y, width_int);
    }
    script.push(&command, field("y", &y));
}

fn rgba_uv_case(script: &mut Script, rng: &mut Rng, width: usize) {
    let rgb = rng.words16(4 * width);
    let command = format!("rgba_uv {width} {}", words16_arg(&rgb));
    let mut u = vec![SENTINEL; width];
    let mut v = vec![SENTINEL; width];
    // The C translation, as `-noasm` selects it: the samples leave the domain of
    // upstream's SIMD converters (see the module documentation).
    let previous = VP8_GET_CPU_INFO.swap(ptr::null_mut(), Ordering::AcqRel);
    webp_convert_rgba32_to_uv(&rgb, &mut u, &mut v, i32::try_from(width).expect("int"));
    VP8_GET_CPU_INFO.store(previous, Ordering::Release);
    let expected = field("u", &u) + &field("v", &v);
    script.push(&command, expected);
}

fn yuv_case(script: &mut Script, y: u8, u: u8, v: u8) {
    let command = format!("yuv {y} {u} {v}");
    let (y32, u32_, v32) = (i32::from(y), i32::from(u), i32::from(v));
    let mut rgb = [SENTINEL; 4];
    let mut bgr = [SENTINEL; 4];
    let mut rgb565 = [SENTINEL; 2];
    let mut rgba4444 = [SENTINEL; 2];
    let mut argb = [SENTINEL; 4];
    let mut bgra = [SENTINEL; 4];
    let mut rgba = [SENTINEL; 4];
    vp8_yuv_to_rgb(y32, u32_, v32, &mut rgb);
    vp8_yuv_to_bgr(y32, u32_, v32, &mut bgr);
    vp8_yuv_to_rgb565(y32, u32_, v32, &mut rgb565);
    vp8_yuv_to_rgba4444(y32, u32_, v32, &mut rgba4444);
    vp8_yuv_to_argb(y, u, v, &mut argb);
    vp8_yuv_to_bgra(y, u, v, &mut bgra);
    vp8_yuv_to_rgba(y, u, v, &mut rgba);
    let expected = field("rgb", &rgb[..3])
        + &field("bgr", &bgr[..3])
        + &field("rgb565", &rgb565)
        + &field("rgba4444", &rgba4444)
        + &field("argb", &argb)
        + &field("bgra", &bgra)
        + &field("rgba", &rgba)
        + &format!("r={}\n", vp8yuv_to_r(y32, v32))
        + &format!("g={}\n", vp8yuv_to_g(y32, u32_, v32))
        + &format!("b={}\n", vp8yuv_to_b(y32, u32_));
    script.push(&command, expected);
}

fn rgb_yuv_case(script: &mut Script, r: i32, g: i32, b: i32, rounding: i32) {
    let command = format!("rgb_yuv {r} {g} {b} {rounding}");
    let expected = format!(
        "y={}\nu={}\nv={}\n",
        vp8rgb_to_y(r, g, b, rounding),
        vp8rgb_to_u(r, g, b, rounding),
        vp8rgb_to_v(r, g, b, rounding)
    );
    script.push(&command, expected);
}

fn script() -> Script {
    let mut rng = Rng(0x5eed_0004);
    let mut script = Script::default();
    script.commands.push_str("init\n");

    // Every sampler mode over lengths around the unrolled pairs, and larger rows.
    let lengths: Vec<usize> = (0..=40).chain([63, 64, 65]).collect();
    for mode in 0..SAMPLER_MODES {
        for &len in &lengths {
            row_case(&mut script, &mut rng, mode, len);
        }
    }

    // Padded planes with odd sizes, strides larger than the rows, and both
    // directions of chroma subsampling at the last odd row.
    let shapes: [(usize, usize); 8] = [
        (1, 1),
        (3, 2),
        (5, 3),
        (7, 5),
        (16, 9),
        (17, 8),
        (33, 7),
        (40, 1),
    ];
    for mode in 0..SAMPLER_MODES {
        for &(width, height) in &shapes {
            let pixel = bytes_per_pixel(mode);
            let strides = (width + 3, width.div_ceil(2) + 2, width * pixel + 5);
            plane_case(&mut script, &mut rng, mode, width, height, strides);
            // Tight strides, with no padding at all.
            let tight = (width, width.div_ceil(2), width * pixel);
            plane_case(&mut script, &mut rng, mode, width, height, tight);
        }
    }

    // ARGB, RGB and RGBA to Y and UV for every width up to 33, both store modes.
    for width in 0..=33 {
        argb_y_case(&mut script, &mut rng, width);
        argb_uv_case(&mut script, &mut rng, width, 1);
        argb_uv_case(&mut script, &mut rng, width, 0);
        rgb_y_case(&mut script, &mut rng, width, false);
        rgb_y_case(&mut script, &mut rng, width, true);
        rgba_uv_case(&mut script, &mut rng, width);
    }

    // The inline conversions: every corner, then seeded triples.
    for &y in &[0_u8, 1, 16, 128, 235, 254, 255] {
        for &u in &[0_u8, 16, 128, 240, 255] {
            for &v in &[0_u8, 16, 128, 240, 255] {
                yuv_case(&mut script, y, u, v);
            }
        }
    }
    for _ in 0..1500 {
        let (y, u, v) = (rng.bytes(1)[0], rng.bytes(1)[0], rng.bytes(1)[0]);
        yuv_case(&mut script, y, u, v);
    }

    // RGB to YUV with every rounding the encoder uses, and seeded samples.
    for &rounding in &[0, YUV_HALF, YUV_HALF << 2, 1, 1 << 17] {
        for &(r, g, b) in &[
            (0, 0, 0),
            (255, 255, 255),
            (255, 0, 0),
            (0, 255, 0),
            (0, 0, 255),
        ] {
            rgb_yuv_case(&mut script, r, g, b, rounding);
        }
        for _ in 0..200 {
            let triple = rng.bytes(3);
            rgb_yuv_case(
                &mut script,
                i32::from(triple[0]),
                i32::from(triple[1]),
                i32::from(triple[2]),
                rounding,
            );
        }
    }
    script
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
fn yuv_conversions_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("yuv_conversions_match_the_c_functions") else {
        return;
    };
    let script = script();
    let observations = script.expected.lines().count();
    assert!(
        observations > 3000,
        "the script has {observations} observations"
    );
    let probe = oracle.probe("wp4", "yuv");
    let output = probe.run(&[] as &[&str], script.commands.as_bytes());
    let output = output.success();
    let actual = String::from_utf8(output.stdout.clone()).expect("UTF-8 output");
    assert!(
        actual == script.expected,
        "the YUV conversions differ from the C functions: {}",
        first_difference(&script.expected, &actual)
    );
}
