//! WP13 differential test: the simple encoding API (`Encode`, `WebPEncodeRGB`,
//! `WebPEncodeBGR`, `WebPEncodeRGBA`, `WebPEncodeBGRA` and the four
//! `WebPEncodeLossless*` functions of `src/enc/picture_enc.c`) through the Rust
//! translation against the C library, with the `simple` command of the probe
//! `tools/oracle/probes/wp13/api.c`. The rows come from the probe's xorshift32
//! generator, which `fill` reproduces. Compared: the returned size, whether the
//! output was written (C writes `NULL` on failure and leaves it untouched when
//! the configuration fails), and the bytes. Negative strides and `NULL`
//! arguments exist only at the C ABI (`tests/wp13_abi.rs`).

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::enc::picture_enc::{
    webp_encode_bgr, webp_encode_bgra, webp_encode_lossless_bgr, webp_encode_lossless_bgra,
    webp_encode_lossless_rgb, webp_encode_lossless_rgba, webp_encode_rgb, webp_encode_rgba,
};
use webp::webp::encode::{WebPConfig, WebPPreset, webp_config_preset};

/// The probe's `Fill`: xorshift32 bytes, with the fourth byte of each pixel
/// forced by `amode`.
fn fill(size: usize, seed: u32, amode: i32) -> Vec<u8> {
    let mut s = seed | 1;
    (0..size)
        .map(|i| {
            s ^= s << 13;
            s ^= s >> 17;
            s ^= s << 5;
            let mut byte = s as u8;
            if i & 3 == 3 {
                if amode == 1 {
                    byte = 0xff;
                }
                if amode == 2 {
                    byte = if byte & 0x10 != 0 { 0xff } else { 0 };
                }
            }
            byte
        })
        .collect()
}

type Lossy = fn(&[u8], i32, i32, i32, f32) -> Option<Vec<u8>>;
type Lossless = fn(&[u8], i32, i32, i32) -> Option<Vec<u8>>;

const LOSSY: [Lossy; 4] = [
    webp_encode_rgb,
    webp_encode_bgr,
    webp_encode_rgba,
    webp_encode_bgra,
];
const LOSSLESS: [Lossless; 4] = [
    webp_encode_lossless_rgb,
    webp_encode_lossless_bgr,
    webp_encode_lossless_rgba,
    webp_encode_lossless_bgra,
];

/// What the probe prints for one `simple` command, from the translation.
fn rust_simple(
    f: usize,
    w: i32,
    h: i32,
    stride: i32,
    quality: f32,
    seed: u32,
    amode: i32,
) -> Fields {
    let rows = h.max(1) as usize;
    let input = fill(rows * stride as usize + 64, seed, amode);
    let quality_used = if f < 4 { quality } else { 70.0 };
    let mut config = WebPConfig::default();
    let configured = webp_config_preset(&mut config, WebPPreset::WEBP_PRESET_DEFAULT, quality_used);
    let result = if f < 4 {
        LOSSY[f](&input, w, h, stride, quality)
    } else {
        LOSSLESS[f - 4](&input, w, h, stride)
    };
    let fields = Fields::new().with("cmd", "simple");
    match result {
        Some(bytes) => fields
            .with("size", bytes.len())
            .with("output", "set")
            .hex("out", &bytes),
        None => fields
            .with("size", 0)
            .with("output", if configured { "null" } else { "untouched" }),
    }
}

#[test]
fn simple_api_matches_the_c_functions() {
    let test = "simple_api_matches_the_c_functions";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let mut commands = String::new();
    let mut expected = Fields::new();
    let mut seed = 0x1357_u32;
    for f in 0..8_usize {
        let step = if f % 4 < 2 { 3 } else { 4 };
        for (w, h) in [
            (1, 1),
            (2, 3),
            (7, 5),
            (16, 16),
            (33, 17),
            (64, 9),
            (0, 4),
            (5, 0),
        ] {
            let tight = step * w.max(1);
            for stride in [tight, tight + 3, tight - 1, 0] {
                for quality in [0.0_f32, 30.0, 75.0, 100.0, -0.5, 100.5] {
                    if f >= 4 && quality != 75.0 {
                        continue;
                    }
                    seed = seed.wrapping_mul(2_654_435_761).wrapping_add(17);
                    let amode = (seed >> 7) as i32 % 3;
                    let seed = seed >> 2;
                    commands.push_str(&format!(
                        "simple {f} {w} {h} {stride} {} {seed} {amode} 0\n",
                        quality.to_bits() as i32
                    ));
                    for (key, value) in rust_simple(f, w, h, stride, quality, seed, amode).0 {
                        expected.0.push((key, value));
                    }
                }
            }
        }
    }
    let output = oracle
        .probe("wp13", "api")
        .run::<&str>(&[], commands.as_bytes());
    output.success();
    let c = output.fields();
    for (index, (a, b)) in c.0.iter().zip(&expected.0).enumerate() {
        assert_eq!(a, b, "{test}: line {index} differs");
    }
    assert_eq!(c.0.len(), expected.0.len(), "{test}: output lengths differ");
    let encoded =
        c.0.iter()
            .filter(|(k, v)| k == "output" && v == "set")
            .count();
    eprintln!(
        "{test}: {} calls, {encoded} encoded",
        commands.lines().count()
    );
}
