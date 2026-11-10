//! WP6 differential test: the decoder DSP of `src/dsp/dec.c` (inverse
//! transforms, intra predictors, simple and complex loop filters, dithering) and
//! the clip tables of `src/dsp/dec_clip_tables.c`, against the C probe
//! `tools/oracle/probes/wp6/dsp.c`.
//!
//! The test generates seeded inputs, runs each function in Rust and in the
//! probe, and compares every byte of the updated buffers. The transform inputs
//! stay within ±15000, below which the C `int` arithmetic of `TransformOne`
//! cannot overflow (PORTING rule 8; `docs/status/wp6.md`).

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dsp::dec::{
    h_filter8_c, h_filter8i_c, h_filter16_c, h_filter16i_c, simple_h_filter16_c,
    simple_h_filter16i_c, simple_v_filter16_c, simple_v_filter16i_c, transform_ac3_c,
    transform_two_c, v_filter8_c, v_filter8i_c, v_filter16_c, v_filter16i_c, vp8_dither_combine8x8,
    vp8_pred_chroma8, vp8_pred_luma4, vp8_pred_luma16, vp8_transform_dc, vp8_transform_dcuv,
    vp8_transform_wht,
};
use webp::dsp::dec_clip_tables::{VP8KABS0, VP8KCLIP1, VP8KSCLIP1, VP8KSCLIP2};
use wp6_support::{Rng, compare, hex, int16_bytes, token};

const BPS: usize = 32;

/// The script and the expected output, built together.
struct Script {
    commands: String,
    expected: String,
    cases: usize,
}

impl Script {
    fn new() -> Self {
        Self {
            commands: String::new(),
            expected: String::new(),
            cases: 0,
        }
    }

    fn line(&mut self, key: &str, bytes: &[u8]) {
        writeln!(self.expected, "{key}={}", hex(bytes)).unwrap();
    }
}

/// A coefficient: mostly small or zero, sometimes up to the bound.
fn coefficient(rng: &mut Rng) -> i16 {
    match rng.below(8) {
        0..=2 => 0,
        3..=4 => rng.range(-16, 16) as i16,
        5..=6 => rng.range(-2048, 2047) as i16,
        _ => rng.range(-15000, 15000) as i16,
    }
}

fn tables(script: &mut Script) {
    script.commands.push_str("tables\n");
    let sclip1: Vec<u8> = (-1020..=1020).map(|i| VP8KSCLIP1[i] as u8).collect();
    let sclip2: Vec<u8> = (-112..=112).map(|i| VP8KSCLIP2[i] as u8).collect();
    let clip1: Vec<u8> = (-255..=511).map(|i| VP8KCLIP1[i]).collect();
    let abs0: Vec<u8> = (-255..=255).map(|i| VP8KABS0[i]).collect();
    script.line("sclip1", &sclip1);
    script.line("sclip2", &sclip2);
    script.line("clip1", &clip1);
    script.line("abs0", &abs0);
    script.cases += 1;
}

fn transforms(script: &mut Script, rng: &mut Rng) {
    for kind in ["two", "one", "ac3", "uv", "dc", "dcuv"] {
        for _ in 0..400 {
            let mut coeffs = [0i16; 64];
            for c in &mut coeffs {
                *c = coefficient(rng);
            }
            if kind == "ac3" {
                // AC3 assumes only in[0], in[1] and in[4] are non-zero; the
                // other coefficients do not matter to it.
                for (i, c) in coeffs.iter_mut().enumerate() {
                    if i >= 16 || ![0, 1, 4].contains(&i) {
                        *c = 0;
                    }
                }
            }
            let mut dst = rng.bytes(8 * BPS);
            writeln!(
                script.commands,
                "transform {kind} {} {}",
                hex(&int16_bytes(&coeffs)),
                hex(&dst)
            )
            .unwrap();
            // The inverse transforms that call `TransformOne` run the C
            // translations, which these records verify against the C oracle:
            // with `simd` on 64-bit ARM the dispatch selects upstream's NEON
            // transforms, which saturate in 16-bit lanes and differ from C
            // outside `[-2048, 2047]`, the input range that `TransformOne_C`'s
            // own comment assumes and that encoded streams stay in, while
            // these records reach ±15000. `uv` is `TransformUV_C`: two
            // `VP8Transform` calls of two blocks each. `tests/wp20_dec.rs`
            // covers the NEON transforms (WP20, decided on issue #28).
            match kind {
                "two" => transform_two_c(&coeffs, &mut dst, true),
                "one" => transform_two_c(&coeffs, &mut dst, false),
                "ac3" => transform_ac3_c(&coeffs, &mut dst),
                "uv" => {
                    transform_two_c(&coeffs, &mut dst, true);
                    transform_two_c(&coeffs[2 * 16..], &mut dst[4 * BPS..], true);
                }
                "dc" => vp8_transform_dc(&coeffs, &mut dst),
                _ => vp8_transform_dcuv(&coeffs, &mut dst),
            }
            script.line("dst", &dst);
            script.cases += 1;
        }
    }
    for _ in 0..400 {
        let input: Vec<i16> = (0..16)
            .map(|_| {
                if rng.chance(1, 4) {
                    rng.next_u64() as i16
                } else {
                    coefficient(rng)
                }
            })
            .collect();
        let mut out: Vec<i16> = (0..256).map(|_| rng.next_u64() as i16).collect();
        writeln!(
            script.commands,
            "wht {} {}",
            hex(&int16_bytes(&input)),
            hex(&int16_bytes(&out))
        )
        .unwrap();
        vp8_transform_wht(&input, &mut out);
        script.line("out", &int16_bytes(&out));
        script.cases += 1;
    }
}

fn predictors(script: &mut Script, rng: &mut Rng) {
    for (family, modes) in [("luma4", 10), ("luma16", 7), ("chroma8", 7)] {
        for mode in 0..modes {
            for _ in 0..60 {
                let mut buf = if rng.chance(1, 4) {
                    // Flat or nearly flat neighbors, as in smooth pictures.
                    let base = rng.byte();
                    (0..BPS * 20)
                        .map(|_| base.wrapping_add(rng.range(-2, 2) as u8))
                        .collect()
                } else {
                    rng.bytes(BPS * 20)
                };
                let pos = BPS * rng.range(1, 2) as usize + rng.range(1, 8) as usize;
                writeln!(script.commands, "pred {family} {mode} {pos} {}", hex(&buf)).unwrap();
                match family {
                    "luma4" => vp8_pred_luma4(mode, &mut buf, pos),
                    "luma16" => vp8_pred_luma16(mode, &mut buf, pos),
                    _ => vp8_pred_chroma8(mode, &mut buf, pos),
                }
                script.line("buf", &buf);
                script.cases += 1;
            }
        }
    }
}

/// A plane with smooth areas, edges and noise, so that the filters take every
/// branch.
fn filter_plane(rng: &mut Rng, size: usize) -> Vec<u8> {
    match rng.below(4) {
        0 => rng.bytes(size),
        1 => {
            let base = rng.range(0, 255);
            let spread = rng.range(0, 8);
            (0..size)
                .map(|_| (base + rng.range(-spread, spread)).clamp(0, 255) as u8)
                .collect()
        }
        2 => {
            // Steps between areas, with noise.
            let a = rng.range(0, 255);
            let b = rng.range(0, 255);
            let step = rng.range(1, 31) as usize;
            (0..size)
                .map(|i| {
                    let base = if (i % 32) < step { a } else { b };
                    (base + rng.range(-3, 3)).clamp(0, 255) as u8
                })
                .collect()
        }
        _ => {
            let a = rng.range(0, 255);
            let b = rng.range(0, 255);
            let row = rng.range(1, 24) as usize;
            let stride = 64;
            (0..size)
                .map(|i| {
                    let base = if i / stride < row { a } else { b };
                    (base + rng.range(-2, 2)).clamp(0, 255) as u8
                })
                .collect()
        }
    }
}

fn threshold(rng: &mut Rng) -> i32 {
    if rng.chance(1, 10) {
        rng.range(0, 1200) as i32
    } else {
        rng.range(0, 200) as i32
    }
}

/// The filter records call the C translations (`SimpleVFilter16_C` and the
/// others), which they verify against the C oracle, rather than the dispatch
/// functions: with `simd` on 64-bit ARM the dispatch selects upstream's NEON
/// filters, which convert the thresholds to `uint8_t` and differ from the C
/// functions for the thresholds above 254 that `threshold` draws and the
/// decoder never passes (at most 193). `tests/wp20_dec.rs` covers the NEON
/// filters over the thresholds 0 to 254 (WP20, decided on issue #28).
fn filters(script: &mut Script, rng: &mut Rng) {
    for name in ["v16", "h16", "v16i", "h16i"] {
        for _ in 0..300 {
            let stride = rng.range(32, 64) as usize;
            let mut buf = filter_plane(rng, stride * 26);
            let pos = 5 * stride + 5;
            let thresh = threshold(rng);
            writeln!(
                script.commands,
                "simple {name} {pos} {stride} {thresh} {}",
                hex(&buf)
            )
            .unwrap();
            let s = stride as i32;
            match name {
                "v16" => simple_v_filter16_c(&mut buf, pos, s, thresh),
                "h16" => simple_h_filter16_c(&mut buf, pos, s, thresh),
                "v16i" => simple_v_filter16i_c(&mut buf, pos, s, thresh),
                _ => simple_h_filter16i_c(&mut buf, pos, s, thresh),
            }
            script.line("buf", &buf);
            script.cases += 1;
        }
        for _ in 0..300 {
            let stride = rng.range(32, 64) as usize;
            let mut buf = filter_plane(rng, stride * 26);
            let pos = 5 * stride + 5;
            let thresh = threshold(rng);
            let ithresh = rng.range(0, 70) as i32;
            let hev = rng.range(0, 3) as i32;
            writeln!(
                script.commands,
                "luma {name} {pos} {stride} {thresh} {ithresh} {hev} {}",
                hex(&buf)
            )
            .unwrap();
            let s = stride as i32;
            match name {
                "v16" => v_filter16_c(&mut buf, pos, s, thresh, ithresh, hev),
                "h16" => h_filter16_c(&mut buf, pos, s, thresh, ithresh, hev),
                "v16i" => v_filter16i_c(&mut buf, pos, s, thresh, ithresh, hev),
                _ => h_filter16i_c(&mut buf, pos, s, thresh, ithresh, hev),
            }
            script.line("buf", &buf);
            script.cases += 1;
        }
    }
    for name in ["v8", "h8", "v8i", "h8i"] {
        for _ in 0..300 {
            let stride = rng.range(16, 40) as usize;
            let mut u = filter_plane(rng, stride * 18);
            let mut v = filter_plane(rng, stride * 18);
            let upos = 5 * stride + 5;
            let vpos = 5 * stride + rng.range(4, 6) as usize;
            let thresh = threshold(rng);
            let ithresh = rng.range(0, 70) as i32;
            let hev = rng.range(0, 3) as i32;
            writeln!(
                script.commands,
                "chroma {name} {upos} {vpos} {stride} {thresh} {ithresh} {hev} {} {}",
                hex(&u),
                hex(&v)
            )
            .unwrap();
            let s = stride as i32;
            match name {
                "v8" => v_filter8_c(&mut u, upos, &mut v, vpos, s, thresh, ithresh, hev),
                "h8" => h_filter8_c(&mut u, upos, &mut v, vpos, s, thresh, ithresh, hev),
                "v8i" => v_filter8i_c(&mut u, upos, &mut v, vpos, s, thresh, ithresh, hev),
                _ => h_filter8i_c(&mut u, upos, &mut v, vpos, s, thresh, ithresh, hev),
            }
            script.line("u", &u);
            script.line("v", &v);
            script.cases += 1;
        }
    }
}

fn dithering(script: &mut Script, rng: &mut Rng) {
    for _ in 0..400 {
        let amp = rng.range(0, 255);
        let dither: Vec<u8> = (0..64)
            .map(|_| (128 + rng.range(-amp, amp) / 2).clamp(0, 255) as u8)
            .collect();
        let stride = rng.range(8, 40) as usize;
        let mut dst = rng.bytes(stride * 8);
        writeln!(
            script.commands,
            "dither {} {stride} {}",
            hex(&dither),
            token(&dst)
        )
        .unwrap();
        vp8_dither_combine8x8(&dither, &mut dst, stride as i32);
        script.line("dst", &dst);
        script.cases += 1;
    }
}

#[test]
fn decoder_dsp_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("decoder_dsp_matches_the_c_functions") else {
        return;
    };
    let probe = oracle.probe("wp6", "dsp");
    let mut rng = Rng(0x5750_3644_5350);
    let mut script = Script::new();
    tables(&mut script);
    transforms(&mut script, &mut rng);
    predictors(&mut script, &mut rng);
    filters(&mut script, &mut rng);
    dithering(&mut script, &mut rng);
    let output = probe.run::<&str>(&[], script.commands.as_bytes());
    output.success();
    compare("dsp", &output.stdout, &script.expected);
    eprintln!("dsp: {} cases", script.cases);
}
