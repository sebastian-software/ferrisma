//! WP7 differential test: the alpha path of the lossless decoder,
//! `VP8LDecodeAlphaHeader` and `VP8LDecodeAlphaImageStream` of
//! `src/dec/vp8l_dec.c`, on the lossless ALPH chunks of the test vectors
//! (`alpha_*`, `lossy_alpha*`, `alpha_color_cache.webp`,
//! `big_endian_bug_393.webp`), against the C probe
//! `tools/oracle/probes/wp7/alpha.c`.
//!
//! Each chunk is decoded in row steps as the VP8 decoder requests them, with and
//! without a crop window, truncated at many points and with mutated bytes. The
//! test compares the decoder state, the choice of the 8-bit path, `prev_line`,
//! the io and the whole alpha plane after the header and each step.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp7_support/mod.rs"]
mod wp7_support;

use std::fmt::Write as _;
use std::path::PathBuf;

use oracle::Oracle;
use webp::dec::alphai_dec::ALPHDecoder;
use webp::dec::vp8_dec::VP8Io;
use webp::dec::vp8l_dec::{vp8l_decode_alpha_header, vp8l_decode_alpha_image_stream};
use webp::dsp::dsp::WEBP_FILTER_TYPE;
use wp7_support::{
    Rng, canvas_size, compare, find_chunk, hex, print_decoder, print_decoder_full, print_digest,
    print_io,
};

/// One `alpha` command of the probe.
#[derive(Clone)]
struct Alpha {
    size: usize,
    width: i32,
    height: i32,
    use_cropping: bool,
    crop: [i32; 4],
    rows: Vec<i32>,
    dump: bool,
}

impl Alpha {
    fn command(&self) -> String {
        let rows = if self.rows.is_empty() {
            "-".to_owned()
        } else {
            self.rows
                .iter()
                .map(i32::to_string)
                .collect::<Vec<_>>()
                .join(",")
        };
        format!(
            "alpha {} {} {} {} {} {} {} {} {} {}\n",
            self.size,
            self.width,
            self.height,
            i32::from(self.use_cropping),
            self.crop[0],
            self.crop[1],
            self.crop[2],
            self.crop[3],
            rows,
            i32::from(self.dump)
        )
    }
}

fn print_alpha(out: &mut String, alph_dec: &ALPHDecoder, plane: &[u8], dump: bool) {
    writeln!(out, "use_8b_decode={}", i32::from(alph_dec.use_8b_decode)).unwrap();
    let prev_line = alph_dec.prev_line.map_or(-1, |line| line as i64);
    writeln!(out, "prev_line={prev_line}").unwrap();
    print_io(out, &alph_dec.io);
    if dump {
        writeln!(out, "plane={}", hex(plane)).unwrap();
    } else {
        print_digest(out, "plane", plane);
    }
}

/// Runs an `alpha` command on the Rust translation and prints what the probe
/// prints.
fn run(input: &[u8], case: &Alpha, out: &mut String) {
    let mut plane = vec![0xa5_u8; case.width as usize * case.height as usize];
    let mut alph_dec = ALPHDecoder {
        width: case.width,
        height: case.height,
        method: i32::from(input[0] & 0x03),
        filter: WEBP_FILTER_TYPE(u32::from((input[0] >> 2) & 0x03)),
        pre_processing: i32::from((input[0] >> 4) & 0x03),
        ..ALPHDecoder::default()
    };
    alph_dec.io = VP8Io {
        width: case.width,
        height: case.height,
        use_cropping: case.use_cropping,
        crop_left: case.crop[0],
        crop_right: case.crop[1],
        crop_top: case.crop[2],
        crop_bottom: case.crop[3],
        ..VP8Io::default()
    };
    let data = &input[1..case.size];

    let mut ok = vp8l_decode_alpha_header(&mut alph_dec, data);
    writeln!(out, "header={}", i32::from(ok)).unwrap();
    writeln!(
        out,
        "method={}\nfilter={}",
        alph_dec.method, alph_dec.filter.0
    )
    .unwrap();
    writeln!(
        out,
        "has_decoder={}",
        i32::from(alph_dec.vp8l_dec.is_some())
    )
    .unwrap();
    if let Some(dec) = &alph_dec.vp8l_dec {
        print_decoder_full(out, dec);
    }
    print_alpha(out, &alph_dec, &plane, case.dump);

    for &rows in &case.rows {
        if !ok {
            break;
        }
        ok = vp8l_decode_alpha_image_stream(&mut alph_dec, rows, data, &mut plane).is_ok();
        writeln!(out, "rows={rows}").unwrap();
        writeln!(out, "image={}", i32::from(ok)).unwrap();
        print_decoder(out, alph_dec.vp8l_dec.as_ref().unwrap());
        print_alpha(out, &alph_dec, &plane, case.dump);
    }
}

/// Row steps up to `last`, of `step` rows each from `first` on.
fn steps(first: i32, last: i32, step: i32) -> Vec<i32> {
    let mut rows = Vec::new();
    let mut row = first;
    while row < last {
        row = (row + step).min(last);
        rows.push(row);
    }
    rows
}

fn configurations(rng: &mut Rng, size: usize, width: i32, height: i32) -> Vec<Alpha> {
    let full = [0, width, 0, height];
    let base = Alpha {
        size,
        width,
        height,
        use_cropping: false,
        crop: full,
        rows: steps(0, height, 16),
        dump: i64::from(width) * i64::from(height) <= 64 * 1024,
    };
    let mut cases = vec![base.clone()];
    for step in [1, 7, height] {
        cases.push(Alpha {
            rows: steps(0, height, step),
            dump: false,
            ..base.clone()
        });
    }
    // Irregular steps, starting late as a cropped VP8 decoding does.
    for _ in 0..3 {
        let mut rows = Vec::new();
        let mut row = rng.range(0, i64::from(height - 1)) as i32;
        while row < height {
            row = (row + rng.range(1, 40) as i32).min(height);
            rows.push(row);
        }
        cases.push(Alpha {
            rows,
            dump: false,
            ..base.clone()
        });
    }
    // Crop windows: rows above crop_top are skipped by the 8-bit path for the
    // filters that allow it, and decoding stops at crop_bottom.
    for _ in 0..6 {
        let top = rng.range(0, i64::from(height - 1)) as i32;
        let bottom = rng.range(i64::from(top + 1), i64::from(height)) as i32;
        let left = rng.range(0, i64::from(width - 1)) as i32;
        let right = rng.range(i64::from(left + 1), i64::from(width)) as i32;
        let step = rng.range(1, 32) as i32;
        cases.push(Alpha {
            use_cropping: true,
            crop: [left, right, top, bottom],
            rows: steps(0, bottom, step),
            dump: false,
            ..base.clone()
        });
    }
    // Truncations.
    let mut sizes: Vec<usize> = (1..size.min(16)).collect();
    for k in 1..24 {
        sizes.push(1 + (size - 1) * k / 24);
    }
    sizes.sort_unstable();
    sizes.dedup();
    for truncated in sizes {
        cases.push(Alpha {
            size: truncated,
            dump: false,
            ..base.clone()
        });
    }
    cases
}

fn alpha_vectors() -> Vec<(PathBuf, usize, Vec<u8>, i32, i32)> {
    oracle::test_vectors()
        .into_iter()
        .filter_map(|path| {
            let file = std::fs::read(&path).unwrap();
            let (offset, size) = find_chunk(&file, b"ALPH")?;
            let payload = file[offset..offset + size].to_vec();
            // Only the lossless method uses the VP8L decoder.
            if payload[0] & 0x03 != 1 {
                return None;
            }
            let (width, height) = canvas_size(&file).expect("an ALPH chunk needs VP8X");
            Some((path, offset, payload, width, height))
        })
        .collect()
}

#[test]
fn alpha_planes_match_the_c_decoder() {
    let test = "alpha_planes_match_the_c_decoder";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let mut rng = Rng(0x5eed_0707);
    let vectors = alpha_vectors();
    assert!(
        vectors.len() >= 10,
        "{test}: {} lossless ALPH chunks",
        vectors.len()
    );
    let mut total = 0;
    let mut eight_bit = 0;
    for (path, offset, payload, width, height) in &vectors {
        let mut commands = String::new();
        let mut expected = String::new();
        writeln!(
            commands,
            "file {} {offset} {}",
            path.display(),
            payload.len()
        )
        .unwrap();
        for case in configurations(&mut rng, payload.len(), *width, *height) {
            commands.push_str(&case.command());
            run(payload, &case, &mut expected);
            total += 1;
        }
        // Mutated bytes after the alpha header byte.
        for _ in 0..24 {
            let mut data = payload.clone();
            for _ in 0..rng.range(1, 3) {
                let position = rng.range(1, data.len() as i64 - 1) as usize;
                data[position] ^= 1 << rng.below(8);
            }
            writeln!(commands, "data {}", hex(&data)).unwrap();
            let case = Alpha {
                size: data.len(),
                width: *width,
                height: *height,
                use_cropping: false,
                crop: [0, *width, 0, *height],
                rows: steps(0, *height, 16),
                dump: false,
            };
            commands.push_str(&case.command());
            run(&data, &case, &mut expected);
            total += 1;
        }
        if expected.contains("use_8b_decode=1") {
            eight_bit += 1;
        }
        let output = oracle
            .probe("wp7", "alpha")
            .run::<&str>(&[], commands.as_bytes());
        output.success();
        compare(test, &output.stdout, &expected);
    }
    eprintln!(
        "{test}: {} chunks ({eight_bit} with the 8-bit path), {total} decodes compared",
        vectors.len()
    );
}
