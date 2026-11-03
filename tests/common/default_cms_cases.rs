// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Deterministic data-only inputs. Every expected result comes from actual C++.
use jxl::jxl::cms::color_encoding_cms::{
    ColorEncoding, ColorSpace, Primaries, TransferFunction, WhitePoint,
};
use jxl::jxl::cms::icc::maybe_create_profile;
use std::collections::BTreeSet;
use std::io::Write;

pub fn hex(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".into();
    }
    bytes.iter().map(|x| format!("{x:02x}")).collect()
}
pub fn capture(family: &str, requests: &[String]) {
    if let Some(path) = std::env::var_os("JXL_DEFAULT_CMS_CAPTURE") {
        let path = std::path::PathBuf::from(path);
        std::fs::create_dir_all(&path).unwrap();
        let mut output = std::io::BufWriter::new(
            std::fs::File::create(path.join(format!("{family}.requests"))).unwrap(),
        );
        for request in requests {
            writeln!(output, "{request}").unwrap();
        }
    }
}
pub fn guards(original: &[Vec<u8>]) -> Vec<String> {
    let mut out = vec!["default-cms-interface".into()];
    for n in [0, 1, 8, 63, 127, 128, 129, 131, 132] {
        for flags in [0, 1, 2, 3, 5, 6, 7] {
            out.push(format!("default-cms-fields {flags} {}", hex(&vec![0; n])));
        }
    }
    for n in [0, 1, 8, 63, 127] {
        out.push(format!("default-cms-fields 4 {}", hex(&vec![0; n])));
    }
    out.extend(
        original
            .iter()
            .map(|bytes| format!("default-cms-fields 0 {}", hex(bytes))),
    );
    out
}
pub fn matrix() -> Vec<String> {
    include_bytes!("../data/default-cms-matrix-inputs.bin")
        .chunks_exact(52)
        .map(|row| {
            let mut line = "default-cms-matrix-product".to_owned();
            for bytes in row.chunks_exact(4) {
                line.push(' ');
                line.push_str(&u32::from_le_bytes(bytes.try_into().unwrap()).to_string());
            }
            line
        })
        .collect()
}
pub fn matrix_exceptional() -> Vec<String> {
    let values: [u32; 16] = [
        0, 0x80000000, 1, 0x80000001, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc00000,
        0xffc00000, 0x7f800001, 0xff800001, 0x7fc12345, 0xffc54321, 0x7f7fffff, 0xff7fffff,
    ];
    let matrices: [[f32; 9]; 4] = [
        [1.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 1.0],
        [1.0; 9],
        [0.7, 0.2, 0.1, -0.1, 0.8, 0.3, 0.2, 0.1, 0.9],
        [1e20, -1e-20, -0.0, 1e-10, 1e10, 1.0, -1.0, 0.333333, 0.0001],
    ];
    let mut out = Vec::new();
    let mut push = |matrix: [u32; 9], rgb: [u32; 3], alias: u32| {
        let words: Vec<_> = matrix
            .into_iter()
            .chain(rgb)
            .chain([alias])
            .map(|x| x.to_string())
            .collect();
        out.push(format!("default-cms-matrix-product {}", words.join(" ")));
    };
    for matrix in matrices.map(|m| m.map(f32::to_bits)) {
        for at in 0..9 {
            for m in values {
                let mut mat = matrix;
                mat[at] = m;
                for chan in 0..3 {
                    for word in values {
                        let mut rgb = [0.2_f32, 0.7, 1.2].map(f32::to_bits);
                        rgb[chan] = word;
                        for alias in [0, 1] {
                            push(mat, rgb, alias);
                        }
                    }
                }
            }
        }
        for x in values {
            for y in values {
                for z in values {
                    for alias in [0, 1] {
                        push(matrix, [x, y, z], alias);
                    }
                }
            }
        }
    }
    out
}
pub fn detail(original: &[Vec<u8>]) -> Vec<String> {
    original
        .iter()
        .map(|bytes| format!("default-cms-profile-detail {}", hex(bytes)))
        .collect()
}
pub fn generated_profiles() -> Vec<(String, Vec<u8>)> {
    let mut out = Vec::new();
    for cs in [ColorSpace::RGB, ColorSpace::GRAY] {
        for prim in [Primaries::SRGB, Primaries::BT2100, Primaries::P3] {
            if cs == ColorSpace::GRAY && prim != Primaries::SRGB {
                continue;
            }
            for white in [WhitePoint::D65, WhitePoint::DCI, WhitePoint::E] {
                for tf in [
                    TransferFunction::BT709,
                    TransferFunction::LINEAR,
                    TransferFunction::SRGB,
                    TransferFunction::PQ,
                    TransferFunction::DCI,
                    TransferFunction::HLG,
                    TransferFunction::GAMMA,
                ] {
                    let mut c = ColorEncoding {
                        color_space: cs,
                        primaries: prim,
                        white_point: white,
                        ..ColorEncoding::default()
                    };
                    if tf == TransferFunction::GAMMA {
                        c.tf.set_gamma(1.0 / 2.2).unwrap();
                    } else {
                        c.tf.set_transfer_function(tf);
                    }
                    let mut bytes = Vec::new();
                    maybe_create_profile(&c.to_external(), &mut bytes).unwrap();
                    out.push((format!("{}-{}-{}-{}", cs.0, prim.0, white.0, tf.0), bytes));
                }
            }
        }
    }
    out
}
fn profile<'a>(all: &'a [(String, Vec<u8>)], name: &str) -> &'a [u8] {
    &all.iter().find(|(key, _)| key == name).unwrap().1
}
#[expect(
    clippy::too_many_arguments,
    reason = "Retain explicit data-only row fixture fields."
)]
fn row(
    out: &mut Vec<String>,
    src: &[u8],
    dst: &[u8],
    width: usize,
    mode: u32,
    intensity: f32,
    threads: usize,
    channels: usize,
) {
    let values = [0.0_f32, -0.0, 0.001, 0.02, 0.1, 0.5, 1.0, 2.0, -0.25];
    let count = width * channels;
    let mut line = format!(
        "default-cms-row {mode} {threads} {width} {} {} {} {} {} {} {count}",
        threads.saturating_sub(1),
        intensity.to_bits(),
        width * if channels == 4 { 4 } else { 3 },
        width * 3,
        hex(src),
        hex(dst)
    );
    for i in 0..count {
        line.push(' ');
        line.push_str(&values[i % values.len()].to_bits().to_string());
    }
    out.push(line);
}
pub fn rows(original: &[Vec<u8>]) -> Vec<String> {
    let all = generated_profiles();
    let srgb = profile(&all, "0-1-1-13");
    let linear = profile(&all, "0-1-1-8");
    let mut out = Vec::new();
    for (name, bytes) in &all {
        let channels = if name.starts_with("1-") { 1 } else { 3 };
        for width in [0, 1, 5] {
            row(&mut out, bytes, linear, width, 0, 255.0, 2, channels);
            row(&mut out, linear, bytes, width, 0, 255.0, 2, 3);
        }
    }
    for cs in [0, 1] {
        for tf in [1, 8, 13, 16, 17, 18, 65535] {
            let bytes = profile(&all, &format!("{cs}-1-1-{tf}"));
            for width in [0, 1, 3, 4, 7] {
                for mode in [1, 3, 4, 5, 7] {
                    for intensity in [80.0, 300.0, 1000.0] {
                        row(
                            &mut out,
                            bytes,
                            linear,
                            width,
                            mode,
                            intensity,
                            2,
                            if cs == 1 { 1 } else { 3 },
                        );
                        row(&mut out, linear, bytes, width, mode, intensity, 2, 3);
                    }
                }
            }
        }
    }
    for bytes in original {
        let channels = match &bytes[16..20] {
            b"CMYK" => 4,
            b"GRAY" => 1,
            _ => 3,
        };
        for width in [0, 2] {
            row(&mut out, bytes, srgb, width, 0, 255.0, 2, channels);
            row(&mut out, srgb, bytes, width, 0, 255.0, 2, 3);
        }
    }
    for threads in [0, 1, 3] {
        for width in [0, 1, 5] {
            row(&mut out, srgb, srgb, width, 0, 255.0, threads, 3);
        }
    }
    for width in [0, 1] {
        row(&mut out, &[], &[], width, 8, 255.0, 2, 3);
    }
    out
}
pub fn cicp() -> Vec<String> {
    let mut pairs = BTreeSet::new();
    for prim in 0..=u8::MAX {
        for tf in 0..=u8::MAX {
            pairs.insert((prim, tf, 0, 1));
        }
    }
    for matrix in 0..=u8::MAX {
        for range in 0..=u8::MAX {
            pairs.insert((1, 13, matrix, range));
        }
    }
    for prim in [1, 9, 11, 12] {
        for tf in [1, 8, 13, 16, 17, 18] {
            for value in 0..=u8::MAX {
                pairs.insert((prim, tf, value, 1));
                pairs.insert((prim, tf, 0, value));
            }
        }
    }
    let mut out = Vec::new();
    for (prim, tf, matrix, range) in pairs {
        for mode in [0, 1, 2] {
            if mode == 1
                && [1, 9, 11, 12].contains(&prim)
                && [1, 8, 13, 16, 17, 18].contains(&tf)
                && matrix == 0
                && range == 1
            {
                continue;
            }
            out.push(format!(
                "default-cms-cicp {prim} {tf} {matrix} {range} {mode}"
            ));
        }
    }
    out
}
#[expect(
    clippy::too_many_arguments,
    reason = "Retain exact data-only TF context field order."
)]
fn tf(
    out: &mut Vec<String>,
    op: u32,
    code: i32,
    mask: u32,
    intensity: f32,
    apply: u32,
    channels: usize,
    forward: u32,
    size: usize,
    alias: u32,
    nulls: u32,
    values: &[u32],
    lums: [f32; 3],
) {
    let mut line = format!(
        "default-cms-tf {op} {code} {mask} {} {apply} {channels} {forward} {size} {alias} {nulls} {} {} {} {}",
        intensity.to_bits(),
        values.len(),
        lums[0].to_bits(),
        lums[1].to_bits(),
        lums[2].to_bits()
    );
    for value in values {
        line.push(' ');
        line.push_str(&value.to_string());
    }
    out.push(line);
}
pub fn exceptional() -> Vec<String> {
    let values = [
        0, 0x80000000, 1, 0x80000001, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc00000,
        0xffc00000, 0x7f800001, 0xff800001, 0x7fc12345, 0xffc54321, 0x7f7fffff, 0xff7fffff,
    ];
    let mut out = Vec::new();
    for op in [0, 1, 2] {
        for code in if op < 2 { &[1, 2, 3][..] } else { &[0][..] } {
            for channels in if op < 2 { &[1][..] } else { &[1, 3][..] } {
                for value in values {
                    tf(
                        &mut out,
                        op,
                        *code,
                        15,
                        f32::from_bits(1132396544),
                        0,
                        *channels,
                        1,
                        3,
                        0,
                        0,
                        &[value; 3],
                        [
                            f32::from_bits(1046068150),
                            f32::from_bits(1060571709),
                            f32::from_bits(1033091004),
                        ],
                    );
                }
            }
        }
    }
    out
}
pub fn tf_domain() -> Vec<String> {
    let mut out = Vec::new();
    let standard = [0.2126, 0.7152, 0.0722];
    for start in (0..65536).step_by(16) {
        let values: Vec<_> = (start..start + 16)
            .map(|value| jxl::jxl::base::float::load_float16(value as u16).to_bits())
            .collect();
        for op in [0, 1] {
            for code in [1, 2, 3] {
                for intensity in if code == 1 {
                    &[255.0, 10000.0][..]
                } else {
                    &[255.0][..]
                } {
                    tf(
                        &mut out,
                        op,
                        code,
                        if code == 1 {
                            1
                        } else if code == 2 {
                            2
                        } else {
                            0
                        },
                        *intensity,
                        0,
                        1,
                        1,
                        16,
                        0,
                        0,
                        &values,
                        standard,
                    );
                }
            }
        }
        for intensity in [80.0, 255.0, 1000.0, 10000.0] {
            for forward in [0, 1] {
                tf(
                    &mut out, 2, 0, 5, intensity, 0, 1, forward, 16, 0, 0, &values, standard,
                );
            }
        }
    }
    let mut seed = 0xdefa0175ca11_u64;
    let weights = [
        standard,
        [0.2627, 0.6780, 0.0593],
        [1.0 / 3.0; 3],
        [1.0, 0.0, 0.0],
        [0.0; 3],
        [-1.0, 2.0, 0.0001],
        [f32::NAN, 0.5, 0.5],
        [f32::INFINITY, -0.0, 0.5],
    ];
    for i in 0..8192 {
        let mut values = [0; 3];
        for value in &mut values {
            seed = seed
                .wrapping_mul(6364136223846793005)
                .wrapping_add(1442695040888963407);
            *value = 0x3f000000 | ((seed >> 32) as u32 & 0x7fffff);
            if i % 4 == 0 && seed & 1 != 0 {
                *value |= 0x80000000;
            }
        }
        for forward in [0, 1] {
            tf(
                &mut out,
                2,
                0,
                13,
                [80.0, 255.0, 1000.0, 10000.0][i % 4],
                0,
                3,
                forward,
                3,
                0,
                0,
                &values,
                weights[(i / 4) % weights.len()],
            );
        }
    }
    for op in [0, 1] {
        for code in [i32::MIN, -1, 0, 4, 17, i32::MAX] {
            for size in [0, 3, 17] {
                for nulls in [0, 3] {
                    tf(
                        &mut out,
                        op,
                        code,
                        0,
                        255.0,
                        0,
                        1,
                        1,
                        size,
                        0,
                        nulls,
                        &vec![0.2_f32.to_bits(); size],
                        standard,
                    );
                }
            }
        }
        for code in [1, 2, 3] {
            tf(
                &mut out,
                op,
                code,
                if code == 1 {
                    1
                } else if code == 2 {
                    2
                } else {
                    0
                },
                255.0,
                0,
                1,
                1,
                0,
                0,
                3,
                &[],
                standard,
            );
        }
        for intensity in [295.0, 300.0, 305.0] {
            tf(
                &mut out,
                op,
                2,
                3,
                intensity,
                1,
                0,
                1,
                0,
                0,
                3,
                &[],
                standard,
            );
        }
    }
    for intensity in [295.0, 300.0, 305.0] {
        for size in [0, 1, 17] {
            for forward in [0, 1] {
                tf(
                    &mut out,
                    2,
                    0,
                    1,
                    intensity,
                    0,
                    0,
                    forward,
                    size,
                    0,
                    3,
                    &vec![0.2_f32.to_bits(); size],
                    standard,
                );
            }
        }
    }
    for channels in [0, 2, 4, usize::MAX] {
        for size in [0, 17] {
            tf(
                &mut out,
                2,
                0,
                5,
                255.0,
                0,
                channels,
                1,
                size,
                0,
                3,
                &vec![0.2_f32.to_bits(); size],
                standard,
            );
            tf(
                &mut out,
                1,
                2,
                7,
                255.0,
                1,
                channels,
                1,
                size,
                0,
                3,
                &vec![0.2_f32.to_bits(); size],
                standard,
            );
        }
        tf(
            &mut out,
            0,
            2,
            7,
            255.0,
            1,
            channels,
            1,
            0,
            0,
            3,
            &[],
            standard,
        );
    }
    let values = [0.2_f32, 0.7, 1.2, -0.1, 0.6, 0.3, 0.8].map(f32::to_bits);
    for op in [0, 1, 2] {
        for size in [0, 1, 2, 3, 4, 5, 7] {
            for alias in [0, 1, 2, 3] {
                tf(
                    &mut out, op, 2, 15, 80.0, 1, 3, 1, size, alias, 0, &values, standard,
                );
            }
        }
    }
    out
}
pub fn prefixes() -> Vec<String> {
    let all = generated_profiles();
    let srgb = profile(&all, "0-1-1-13");
    let mut out = Vec::new();
    for (which, count) in [(0, 12), (1, 8), (2, 8), (3, 4), (4, 8), (5, 8), (6, 4)] {
        for mode in 0..count {
            out.push(format!(
                "default-cms-prefix {which} {mode} {}",
                if (which == 4 || which == 5) && mode >= 2 {
                    hex(srgb)
                } else {
                    "-".into()
                }
            ));
        }
    }
    out
}
pub fn init_prefixes() -> Vec<String> {
    (0..8)
        .map(|mode| format!("default-cms-init-prefix {mode}"))
        .collect()
}
pub fn callbacks() -> Vec<String> {
    let all = generated_profiles();
    let names = [
        "0-1-1-13",
        "0-1-1-8",
        "0-1-1-16",
        "0-1-1-18",
        "0-1-1-65535",
        "1-1-1-13",
        "1-1-1-18",
        "0-11-11-17",
    ];
    let mut out = Vec::new();
    for (index, name) in names.iter().enumerate() {
        for mode in 0..256 {
            out.push(format!(
                "default-cms-callback {mode} {} {} {} {}",
                hex(profile(&all, name)),
                hex(profile(&all, names[(index + 1) % names.len()])),
                hex(profile(&all, names[(index + 2) % names.len()])),
                hex(profile(&all, names[(index + 3) % names.len()]))
            ));
        }
    }
    let srgb = hex(profile(&all, names[0]));
    for replacement in ["-", "00", "0000000000000000"] {
        for mode in [8, 16, 24, 12, 20, 28, 136, 144, 152, 140, 148, 156] {
            out.push(format!(
                "default-cms-callback {mode} {srgb} {srgb} {replacement} {replacement}"
            ));
        }
    }
    out
}
pub fn numeric() -> Vec<String> {
    let mut out = Vec::new();
    for half in 0..=u16::MAX {
        let values = [
            half,
            half.wrapping_mul(37).wrapping_add(13),
            half.wrapping_mul(103).wrapping_add(7),
        ]
        .map(|v| jxl::jxl::base::float::load_float16(v).to_bits());
        out.push(format!(
            "default-cms-numeric 0 {} {} {}",
            values[0], values[1], values[2]
        ));
    }
    let exceptional: [u32; 16] = [
        0, 0x80000000, 1, 0x80000001, 0x3f800000, 0xbf800000, 0x7f800000, 0xff800000, 0x7fc00000,
        0xffc00000, 0x7f800001, 0xff800001, 0x7fc12345, 0xffc54321, 0x7f7fffff, 0xff7fffff,
    ];
    for x in exceptional {
        for y in exceptional {
            for z in exceptional {
                out.push(format!("default-cms-numeric 0 {x} {y} {z}"));
            }
        }
    }
    for row in include_bytes!("../data/default-cms-matrix-inputs.bin").chunks_exact(52) {
        let mut line = "default-cms-numeric 1".to_owned();
        for bytes in row[..36].chunks_exact(4) {
            line.push(' ');
            line.push_str(&u32::from_le_bytes(bytes.try_into().unwrap()).to_string());
        }
        out.push(line);
    }
    for tf in (0..256).chain([256, 65535, 65536, 0x7fffffff, 0x80000000, u32::MAX]) {
        for primaries in 0..256 {
            out.push(format!("default-cms-numeric 2 {tf} {primaries}"));
        }
    }
    let mut signatures = vec![
        0,
        1,
        u32::MAX,
        0x80000000,
        0x52474220,
        0x434d594b,
        0x47524159,
        0x58595a20,
        0x4c616220,
    ];
    let mut seed = 0xdefac001_u32;
    for _ in 0..4096 {
        seed = seed.wrapping_mul(1664525).wrapping_add(1013904223);
        signatures.push(seed);
    }
    out.extend(
        signatures
            .into_iter()
            .map(|signature| format!("default-cms-numeric 3 {signature}")),
    );
    out
}
pub fn allocation() -> Vec<String> {
    let mut out = Vec::new();
    for initial in [0, 1, 31, 32, 33, 63, 64, 65, 129] {
        for length in [
            0, 1, 2, 3, 7, 15, 31, 32, 33, 63, 64, 65, 95, 127, 128, 129, 255,
        ] {
            for threads in [0, 1, 2, 3, 7] {
                out.push(format!("default-cms-allocate {initial} 4 {length} {threads} 0 {threads} 33 {threads} {length} {threads}"));
            }
        }
    }
    for length in [
        usize::MAX,
        usize::MAX - 1,
        usize::MAX - 30,
        usize::MAX - 31,
        usize::MAX / 2,
    ] {
        out.push(format!("default-cms-allocate 17 2 {length} 0 0 0"));
    }
    out
}
pub fn luminances(original: &[Vec<u8>]) -> Vec<String> {
    let mut profiles: Vec<_> = generated_profiles()
        .into_iter()
        .map(|(_, bytes)| bytes)
        .collect();
    profiles.extend_from_slice(original);
    let mut out = Vec::new();
    for bytes in profiles {
        out.push(format!("default-cms-luminances {} 0", hex(&bytes)));
        if &bytes[16..20] == b"GRAY" {
            out.push(format!("default-cms-luminances {} 1", hex(&bytes)));
        }
    }
    out
}
pub fn profile_equality(original: &[Vec<u8>]) -> Vec<String> {
    let all = generated_profiles();
    let mut out = Vec::new();
    for (index, (_, a)) in all.iter().enumerate() {
        for delta in [0, 1, 2, 3, 7, 21, 63] {
            out.push(format!(
                "default-cms-profile-equal {} {}",
                hex(a),
                hex(&all[(index + delta) % all.len()].1)
            ));
        }
    }
    for a in original {
        for (_, b) in all.iter().step_by(7) {
            out.push(format!("default-cms-profile-equal {} {}", hex(a), hex(b)));
            out.push(format!("default-cms-profile-equal {} {}", hex(b), hex(a)));
        }
    }
    out
}
pub fn fields_for_profiles(profiles: Vec<Vec<u8>>) -> Vec<String> {
    profiles
        .iter()
        .map(|bytes| format!("default-cms-fields 0 {}", hex(bytes)))
        .collect()
}
pub fn fields_for_parser_requests(requests: Vec<String>) -> Vec<String> {
    requests
        .iter()
        .map(|request| {
            format!(
                "default-cms-fields 0 {}",
                request.split_whitespace().last().unwrap()
            )
        })
        .collect()
}
