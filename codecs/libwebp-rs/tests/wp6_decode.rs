//! WP6 differential test: the VP8 decoder of `src/dec/vp8_dec.c` and
//! `src/dec/frame_dec.c` through `VP8GetHeaders` and `VP8Decode`, with `VP8Io`
//! hooks that print every row that `put()` receives, and the alpha decoding of
//! `src/dec/alpha_dec.c` through `VP8DecompressAlphaRows`, against the C probe
//! `tools/oracle/probes/wp6/decode.c`.
//!
//! Every lossy test vector (the 56 `vp80-*` vectors and the others with a
//! `VP8 ` chunk, with their ALPH chunk where they have one) is decoded on the
//! calling thread (`mt_method` 0) and with the worker (`mt_method` 2, and 1
//! without dithering), with and without in-loop filtering, with dithering and
//! alpha dithering strengths (`VP8InitDithering`), with crop windows, with a
//! failing setup hook and with a `put()` that fails at a given row, and
//! truncated at and around every partition boundary. The test compares the
//! header state, the io, every `put()` (position, size and the visible Y, U, V
//! and alpha rows), the teardown, the result, the status and its message.
//!
//! Mutation check: one mutation of the Rust code is equivalent on the vectors,
//! `data.dither >= MIN_DITHER_AMP` as `>`. A block dithered with amplitude 4
//! changes no sample (the deltas are -2 to 1, and `(delta + 8) >> 4` is 0) and
//! only advances the generator, which shows only when a block with a larger
//! amplitude follows, and no vector has such a pair.
//!
//! On x86, streams with mutated bytes select the C translation through
//! `VP8GetCPUInfo = NULL`: their coefficients can leave the SSE2 transform
//! domain. On AArch64, that pointer does not disable the unconditional NEON
//! transforms, so `transform_matches_c` bounds the comparison domain. The
//! scalar build still covers all defined mutated streams. Other streams run
//! the host-selected SIMD variants (ADR 0008).

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp6_support/mod.rs"]
mod wp6_support;

use core::ptr;
use std::collections::{BTreeMap, BTreeSet};
use std::fmt::Write as _;
use std::sync::atomic::Ordering;

use oracle::Oracle;
use webp::abi::decode::VP8_GET_CPU_INFO;
use webp::dec::alpha_dec::vp8_decompress_alpha_rows;
use webp::dec::frame_dec::{vp8_enter_critical, vp8_init_dithering, vp8_init_frame};
use webp::dec::tree_dec::vp8_parse_intra_mode_row;
use webp::dec::vp8_dec::{
    VP8Io, VP8IoHooks, VP8IoRows, vp8_decode, vp8_decode_mb, vp8_get_headers, vp8_init_io,
    vp8_init_scanline, vp8_new,
};
use webp::dec::vp8i_dec::VP8AlphaPlane;
use webp::dec::webp_dec::webp_io_init_from_options;
use webp::webp::decode::{WEBP_CSP_MODE, WebPDecoderOptions};
use wp6_support::{
    Rng, compare, find_chunk, hex, lossy_vectors, print_digest, print_headers, print_io,
    read_vector, token,
};

/// One `decode` command.
#[derive(Clone)]
struct Decode {
    path: String,
    offset: usize,
    size: usize,
    alpha: Option<(usize, usize)>,
    mt: i32,
    dither: i32,
    alpha_dither: i32,
    bypass: bool,
    nofancy: bool,
    crop: Option<[i32; 4]>,
    setup_fail: bool,
    put_fail: i32,
    dump: bool,
    /// Mutated VP8 data, passed in hex instead of the file's bytes.
    mutated: Option<Vec<u8>>,
}

impl Decode {
    fn command(&self, file: &[u8]) -> String {
        let (alpha_offset, alpha_size) = self
            .alpha
            .map_or((0, -1), |(offset, size)| (offset, size as i64));
        let crop = self.crop.unwrap_or([0; 4]);
        let source = match &self.mutated {
            None => format!(
                "decode {} {} {} {alpha_offset} {alpha_size}",
                self.path, self.offset, self.size
            ),
            Some(data) => {
                let alpha = self.alpha.map_or("-".to_owned(), |(offset, size)| {
                    token(&file[offset..offset + size])
                });
                format!("decodehex {} {alpha} {alpha_size}", token(data))
            }
        };
        format!(
            "{source} {} {} {} {} {} {} {} {} {} {} {} {} {}\n",
            self.mt,
            self.dither,
            self.alpha_dither,
            i32::from(self.bypass),
            i32::from(self.nofancy),
            i32::from(self.crop.is_some()),
            crop[0],
            crop[1],
            crop[2],
            crop[3],
            i32::from(self.setup_fail),
            self.put_fail,
            i32::from(self.dump)
        )
    }

    fn options(&self) -> WebPDecoderOptions {
        let crop = self.crop.unwrap_or([0; 4]);
        WebPDecoderOptions {
            dithering_strength: self.dither,
            alpha_dithering_strength: self.alpha_dither,
            bypass_filtering: i32::from(self.bypass),
            no_fancy_upsampling: i32::from(self.nofancy),
            use_cropping: i32::from(self.crop.is_some()),
            crop_left: crop[0],
            crop_top: crop[1],
            crop_width: crop[2],
            crop_height: crop[3],
            ..WebPDecoderOptions::default()
        }
    }
}

/// The hooks of the probe: they print what they receive.
struct Hooks {
    options: WebPDecoderOptions,
    setup_fail: bool,
    put_fail: i32,
    put_count: i32,
    dump: bool,
    out: String,
}

fn print_rows(
    out: &mut String,
    key: &str,
    rows: &[u8],
    stride: i32,
    width: i32,
    height: i32,
    dump: bool,
) {
    let (stride, width) = (stride as usize, width as usize);
    let mut visible = Vec::with_capacity(width * height as usize);
    for j in 0..height as usize {
        visible.extend_from_slice(&rows[j * stride..j * stride + width]);
    }
    if dump {
        writeln!(out, "{key}={}", hex(&visible)).unwrap();
    } else {
        print_digest(out, key, &visible);
    }
}

impl VP8IoHooks for Hooks {
    fn setup(&mut self, io: &mut VP8Io) -> bool {
        let ok = webp_io_init_from_options(Some(&self.options), io, WEBP_CSP_MODE::MODE_YUV);
        writeln!(self.out, "setup={}", i32::from(ok)).unwrap();
        print_io(&mut self.out, io);
        ok && !self.setup_fail
    }

    fn put(&mut self, io: &VP8Io, rows: &mut VP8IoRows<'_>) -> bool {
        let uv_w = (io.mb_w + 1) / 2;
        let uv_h = (io.mb_h + 1) / 2;
        let ok = self.put_count != self.put_fail;
        writeln!(
            self.out,
            "put={} mb={},{},{} a={}",
            self.put_count,
            io.mb_y,
            io.mb_w,
            io.mb_h,
            i32::from(rows.a.is_some())
        )
        .unwrap();
        print_rows(
            &mut self.out,
            "y",
            rows.y,
            io.y_stride,
            io.mb_w,
            io.mb_h,
            self.dump,
        );
        print_rows(
            &mut self.out,
            "u",
            rows.u,
            io.uv_stride,
            uv_w,
            uv_h,
            self.dump,
        );
        print_rows(
            &mut self.out,
            "v",
            rows.v,
            io.uv_stride,
            uv_w,
            uv_h,
            self.dump,
        );
        if let Some(a) = rows.a {
            print_rows(
                &mut self.out,
                "a",
                &a[rows.a_offset..],
                io.width,
                io.mb_w,
                io.mb_h,
                self.dump,
            );
        }
        self.put_count += 1;
        ok
    }

    fn teardown(&mut self, _io: &VP8Io) {
        writeln!(self.out, "teardown puts={}", self.put_count).unwrap();
    }
}

/// Runs a `decode` command through the Rust translation.
fn run_decode(case: &Decode, file: &[u8]) -> String {
    let mut out = String::new();
    let data = case
        .mutated
        .as_deref()
        .unwrap_or(&file[case.offset..case.offset + case.size]);
    let alpha = case
        .alpha
        .map(|(offset, size)| &file[offset..offset + size]);
    let mut dec = vp8_new();
    let mut io = VP8Io::default();
    assert!(vp8_init_io(&mut io));
    let mut hooks = Hooks {
        options: case.options(),
        setup_fail: case.setup_fail,
        put_fail: case.put_fail,
        put_count: 0,
        dump: case.dump,
        out: String::new(),
    };
    let ok = vp8_get_headers(&mut dec, &mut io, data).is_ok();
    writeln!(out, "headers={}", i32::from(ok)).unwrap();
    print_headers(&mut out, &dec);
    print_io(&mut out, &io);
    if ok {
        dec.mt_method = case.mt;
        let options = case.options();
        vp8_init_dithering(Some(&options), &mut dec);
        writeln!(
            out,
            "dither={} alpha_dithering={}",
            i32::from(dec.dither),
            dec.alpha_dithering
        )
        .unwrap();
        let ok = vp8_decode(&mut dec, &mut io, data, alpha, Some(&mut hooks));
        out.push_str(&hooks.out);
        writeln!(
            out,
            "decode={} status={} error={}",
            i32::from(ok),
            dec.status.0,
            dec.error_msg.unwrap_or("(null)")
        )
        .unwrap();
        writeln!(
            out,
            "ready={} is_alpha_decoded={}",
            i32::from(dec.ready),
            i32::from(dec.is_alpha_decoded)
        )
        .unwrap();
        print_io(&mut out, &io);
    }
    out
}

/// One `alpha` command: the ALPH payload, the io size and crop window, the
/// alpha dithering and the (row, num_rows) calls.
struct Alpha {
    path: String,
    alpha: (usize, usize),
    width: i32,
    height: i32,
    crop: (bool, [i32; 4]),
    alpha_dithering: i32,
    rows: Vec<(i32, i32)>,
}

impl Alpha {
    fn command(&self) -> String {
        let rows: Vec<String> = self
            .rows
            .iter()
            .flat_map(|(row, num)| [row.to_string(), num.to_string()])
            .collect();
        format!(
            "alpha {} {} {} {} {} {} {} {} {} {} {} {}\n",
            self.path,
            self.alpha.0,
            self.alpha.1,
            self.width,
            self.height,
            i32::from(self.crop.0),
            self.crop.1[0],
            self.crop.1[1],
            self.crop.1[2],
            self.crop.1[3],
            self.alpha_dithering,
            if rows.is_empty() {
                "-".to_owned()
            } else {
                rows.join(",")
            }
        )
    }
}

fn run_alpha(case: &Alpha, file: &[u8]) -> String {
    let mut out = String::new();
    let alpha_data = &file[case.alpha.0..case.alpha.0 + case.alpha.1];
    let mut io = VP8Io::default();
    assert!(vp8_init_io(&mut io));
    io.width = case.width;
    io.height = case.height;
    io.use_cropping = case.crop.0;
    io.crop_left = case.crop.1[0];
    io.crop_right = case.crop.1[1];
    io.crop_top = case.crop.1[2];
    io.crop_bottom = case.crop.1[3];
    let mut dec = vp8_new();
    dec.alpha_dithering = case.alpha_dithering;
    for &(row, num_rows) in &case.rows {
        let result = vp8_decompress_alpha_rows(&mut dec, &io, alpha_data, row, num_rows);
        writeln!(
            out,
            "rows={row},{num_rows} out={} status={} decoded={} dithering={}",
            result.map_or(-1, |pos| pos as i64),
            dec.status.0,
            i32::from(dec.is_alpha_decoded),
            dec.alpha_dithering
        )
        .unwrap();
        let end = if dec.is_alpha_decoded {
            io.crop_bottom
        } else {
            row + num_rows
        };
        let start = io.crop_top.min(end);
        let rows = (start * io.width) as usize..(end * io.width) as usize;
        match dec.alpha_plane {
            VP8AlphaPlane::Memory => print_digest(&mut out, "plane", &dec.alpha_plane_mem[rows]),
            VP8AlphaPlane::Frame => print_digest(&mut out, "plane", &dec.frame_alpha[rows]),
            VP8AlphaPlane::None => writeln!(out, "plane=null").unwrap(),
        }
        if result.is_none() {
            break;
        }
    }
    out
}

/// Whether C's `TransformOne` stays within `int` on the 16 coefficients of a
/// block: its first pass cannot overflow for `int16_t` inputs, and its second
/// pass multiplies the first-pass values at positions 4 and 12 by 20091 and
/// 35468 (`WEBP_TRANSFORM_AC3_MUL1` and `_MUL2`).
fn transform_one_fits(block: &[i16]) -> bool {
    let mul1 = |a: i64| ((a * 20091) >> 16) + a;
    let mul2 = |a: i64| (a * 35468) >> 16;
    let mut tmp = [0i64; 16];
    for i in 0..4 {
        let x = |k: usize| i64::from(block[i + k]);
        let a = x(0) + x(8);
        let b = x(0) - x(8);
        let c = mul2(x(4)) - mul1(x(12));
        let d = mul1(x(4)) + mul2(x(12));
        tmp[4 * i] = a + d;
        tmp[4 * i + 1] = b + c;
        tmp[4 * i + 2] = b - c;
        tmp[4 * i + 3] = a - d;
    }
    let fits = |v: i64| i32::try_from(v * 35468).is_ok() && i32::try_from(v * 20091).is_ok();
    (0..4).all(|i| fits(tmp[i + 4]) && fits(tmp[i + 12]))
}

/// Whether the block's coefficients stay within `[-2048, 2047]`, where the
/// inverse transforms that the dispatch selects equal C's: with `simd` on
/// 64-bit ARM these are upstream's NEON transforms, which saturate in 16-bit
/// lanes and differ from `TransformOne_C` outside that range, the input range
/// that `TransformOne_C`'s own comment assumes ("assuming an input in
/// [-2048, 2047] interval") and that encoded streams stay in (WP20, decided on
/// issue #28). The scalar build accepts every block, so it still runs every
/// mutated stream against the reference oracle.
fn transform_matches_c(block: &[i16]) -> bool {
    !cfg!(all(feature = "simd", target_arch = "aarch64"))
        || block.iter().all(|&c| (-2048..=2047).contains(&c))
}

/// Whether C's `TransformOne` stays within `int` on every block that the stream
/// codes, and, with `simd` on 64-bit ARM, whether every block stays within the
/// range where the selected transforms equal C's (`transform_matches_c`). A
/// mutated stream can code coefficients for which its second pass overflows,
/// which C does not define (PORTING rule 8); the test does not issue those. The
/// check parses every macroblock with the translation's public parser, which
/// needs no reconstruction, and checks every block, whichever transform its
/// coefficients select.
fn coefficients_in_range(data: &[u8]) -> bool {
    let mut dec = vp8_new();
    let mut io = VP8Io::default();
    assert!(vp8_init_io(&mut io));
    if vp8_get_headers(&mut dec, &mut io, data).is_err() {
        return true;
    }
    if vp8_enter_critical(&mut dec, &mut io, None).is_err()
        || !vp8_init_frame(&mut dec, &mut io, None)
    {
        return true;
    }
    for mb_y in 0..dec.mb_h {
        dec.mb_y = mb_y;
        let token_br = (mb_y.cast_unsigned() & dec.num_parts_minus_one) as usize;
        if !vp8_parse_intra_mode_row(&mut dec, data) {
            return true;
        }
        while dec.mb_x < dec.mb_w {
            let ok = vp8_decode_mb(&mut dec, token_br, data);
            let block = &dec.mb_data[dec.mb_x as usize];
            if !block
                .coeffs
                .chunks(16)
                .all(|block| transform_one_fits(block) && transform_matches_c(block))
            {
                return false;
            }
            if !ok {
                return true;
            }
            dec.mb_x += 1;
        }
        vp8_init_scanline(&mut dec);
    }
    true
}

/// The partition boundaries of a VP8 payload: the end of the frame header, of
/// the first partition, of the partition size table and of each token
/// partition, as the header parser of the translation finds them.
fn boundaries(payload: &[u8]) -> Vec<usize> {
    let mut points = vec![3, 10];
    let mut dec = vp8_new();
    let mut io = VP8Io::default();
    assert!(vp8_init_io(&mut io));
    if vp8_get_headers(&mut dec, &mut io, payload).is_ok() {
        let part0_end = dec.br.buf_end;
        points.push(part0_end);
        points.push(part0_end + 3 * dec.num_parts_minus_one as usize);
        for part in &dec.parts[..=dec.num_parts_minus_one as usize] {
            points.push(part.buf_end);
        }
    }
    points
}

#[test]
fn vp8_decoder_matches_the_c_decoder() {
    let Some(oracle) = Oracle::locate("vp8_decoder_matches_the_c_decoder") else {
        return;
    };
    let probe = oracle.probe("wp6", "decode");
    let dir = oracle::test_data();
    let mut rng = Rng(0x5750_3644_4543);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut decodes = 0;
    let mut statuses: BTreeMap<u32, usize> = BTreeMap::new();
    let mut puts = 0usize;
    let mut excluded = 0usize;

    let names = lossy_vectors(&dir);
    for name in &names {
        let file = read_vector(&dir, name);
        let (offset, size) = find_chunk(&file, b"VP8 ").expect("a VP8 chunk");
        let size = size.min(file.len() - offset);
        let alpha = find_chunk(&file, b"ALPH");
        let path = dir.join(name).to_str().expect("a UTF-8 path").to_owned();
        let payload = &file[offset..offset + size];
        let width = i32::from(payload[6]) | (i32::from(payload[7] & 0x3f) << 8);
        let height = i32::from(payload[8]) | (i32::from(payload[9] & 0x3f) << 8);
        let big = width * height > 1_000_000;
        let base = Decode {
            path: path.clone(),
            offset,
            size,
            alpha,
            mt: 0,
            dither: 0,
            alpha_dither: 0,
            bypass: false,
            nofancy: false,
            crop: None,
            setup_fail: false,
            put_fail: -1,
            dump: width * height <= 256,
            mutated: None,
        };
        let mut cases = Vec::new();
        if big {
            // One threaded decode of the large picture, cropped.
            cases.push(Decode {
                mt: 2,
                crop: Some([(width / 3) & !1, (height / 4) & !1, 600, 300]),
                ..base.clone()
            });
        } else {
            for mt in [0, 1, 2] {
                cases.push(Decode { mt, ..base.clone() });
            }
            cases.push(Decode {
                bypass: true,
                mt: rng.range(0, 2) as i32 * 2,
                ..base.clone()
            });
            for mt in [0, 2] {
                cases.push(Decode {
                    mt,
                    dither: [50, 100, 101, 37, 13, -5][rng.below(6) as usize],
                    alpha_dither: [0, 50, 100, 120][rng.below(4) as usize],
                    ..base.clone()
                });
                let cw = rng.range(1, i64::from(width)) as i32;
                let ch = rng.range(1, i64::from(height)) as i32;
                let left = rng.range(0, i64::from(width - cw)) as i32;
                let top = rng.range(0, i64::from(height - ch)) as i32;
                cases.push(Decode {
                    mt,
                    crop: Some([left, top, cw, ch]),
                    bypass: rng.chance(1, 3),
                    ..base.clone()
                });
                cases.push(Decode {
                    mt,
                    put_fail: rng.range(0, i64::from((height + 15) / 16)) as i32,
                    ..base.clone()
                });
            }
            // Strengths 7 and 14 give the dithering amplitude 4, the smallest
            // that dithers (MIN_DITHER_AMP), for the quantizers 5 to 7 and 10.
            for dither in [7, 14] {
                cases.push(Decode {
                    dither,
                    mt: rng.range(0, 1) as i32 * 2,
                    ..base.clone()
                });
            }
            cases.push(Decode {
                setup_fail: true,
                mt: 2,
                ..base.clone()
            });
            // A put() that fails on the last row: in the threaded path C
            // reports it at the final Sync without recording a status.
            for mt in [0, 2] {
                cases.push(Decode {
                    mt,
                    put_fail: (height + 15) / 16 - 1,
                    ..base.clone()
                });
            }
            // Truncated alpha data, which fails in the put() branch.
            if let Some((alpha_offset, alpha_size)) = alpha {
                for mt in [0, 2] {
                    cases.push(Decode {
                        mt,
                        alpha: Some((alpha_offset, rng.below(alpha_size as u64) as usize)),
                        ..base.clone()
                    });
                }
            }
            // Mutated bytes in the first partition and in the token partitions.
            for m in 0..6 {
                let mut data = payload.to_vec();
                for _ in 0..rng.range(1, 4) {
                    let pos = rng.range(10, size as i64 - 1) as usize;
                    data[pos] = if m % 2 == 0 {
                        data[pos] ^ (1 << rng.below(8))
                    } else {
                        rng.byte()
                    };
                }
                if !coefficients_in_range(&data) {
                    excluded += 1;
                    // WP6_EXCLUDED names a file that collects the excluded
                    // commands, to check them against the asan-ubsan oracle.
                    if let Some(path) = std::env::var_os("WP6_EXCLUDED") {
                        let case = Decode {
                            mt: 0,
                            mutated: Some(data),
                            ..base.clone()
                        };
                        let mut text = std::fs::read_to_string(&path).unwrap_or_default();
                        text.push_str(&case.command(&file));
                        std::fs::write(&path, text).unwrap();
                    }
                    continue;
                }
                cases.push(Decode {
                    mt: rng.range(0, 1) as i32 * 2,
                    mutated: Some(data),
                    ..base.clone()
                });
            }
            // Truncations at and around the partition boundaries.
            let mut sizes = BTreeSet::new();
            for point in boundaries(payload) {
                for delta in [-2i64, -1, 0, 1, 7] {
                    let cut = point as i64 + delta;
                    if cut >= 0 && (cut as usize) < size {
                        sizes.insert(cut as usize);
                    }
                }
            }
            for _ in 0..3 {
                sizes.insert(rng.below(size as u64) as usize);
            }
            for cut in sizes {
                cases.push(Decode {
                    size: cut,
                    mt: rng.range(0, 1) as i32 * 2,
                    ..base.clone()
                });
            }
        }
        for case in &cases {
            commands.push_str(&case.command(&file));
            let out = if case.mutated.is_some() {
                // The C translation, as `-noasm` selects it, for a stream whose
                // coefficients can leave the domain of upstream's SSE2
                // transform; the pointer comes back for the next case.
                let previous = VP8_GET_CPU_INFO.swap(ptr::null_mut(), Ordering::AcqRel);
                let out = run_decode(case, &file);
                VP8_GET_CPU_INFO.store(previous, Ordering::Release);
                out
            } else {
                run_decode(case, &file)
            };
            for line in out.lines() {
                if let Some(rest) = line.strip_prefix("decode=") {
                    let status: u32 = rest
                        .split_whitespace()
                        .nth(1)
                        .and_then(|s| s.strip_prefix("status="))
                        .and_then(|s| s.parse().ok())
                        .unwrap();
                    *statuses.entry(status).or_default() += 1;
                }
                if line.starts_with("put=") {
                    puts += 1;
                }
            }
            expected.push_str(&out);
            decodes += 1;
        }
    }

    let output = probe.run::<&str>(&[], commands.as_bytes());
    output.success();
    compare("decode", &output.stdout, &expected);
    eprintln!(
        "decode: {decodes} decodes over {} vectors, {puts} puts, statuses {statuses:?}, \
         {excluded} mutations excluded",
        names.len()
    );
}

#[test]
fn alpha_rows_match_the_c_decoder() {
    let Some(oracle) = Oracle::locate("alpha_rows_match_the_c_decoder") else {
        return;
    };
    let probe = oracle.probe("wp6", "decode");
    let dir = oracle::test_data();
    let mut rng = Rng(0x5750_3641_4c50);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut calls = 0;
    for name in lossy_vectors(&dir) {
        let file = read_vector(&dir, &name);
        let Some(alpha) = find_chunk(&file, b"ALPH") else {
            continue;
        };
        let (offset, _) = find_chunk(&file, b"VP8 ").expect("a VP8 chunk");
        let width = i32::from(file[offset + 6]) | (i32::from(file[offset + 7] & 0x3f) << 8);
        let height = i32::from(file[offset + 8]) | (i32::from(file[offset + 9] & 0x3f) << 8);
        let path = dir.join(&name).to_str().expect("a UTF-8 path").to_owned();
        let mut cases = Vec::new();
        for variant in 0..12 {
            let crop = if variant % 3 == 1 {
                let left = rng.range(0, i64::from(width - 1)) as i32 & !1;
                let top = rng.range(0, i64::from(height - 1)) as i32 & !1;
                let right = rng.range(i64::from(left + 1), i64::from(width)) as i32;
                let bottom = rng.range(i64::from(top + 1), i64::from(height)) as i32;
                (true, [left, right, top, bottom])
            } else {
                (false, [0, width, 0, height])
            };
            let bottom = crop.1[3];
            let step = [16, 1, 7, 64][variant % 4];
            let mut rows = Vec::new();
            let mut row = 0;
            while row < bottom {
                let num = step.min(bottom - row);
                rows.push((row, num));
                row += num;
            }
            if variant == 5 {
                // An out-of-range request fails without decoding.
                rows.push((bottom, 1));
            }
            if variant == 8 {
                rows.insert(0, (0, 0));
            }
            let size = if variant >= 9 {
                // Truncated alpha data.
                rng.range(0, alpha.1 as i64 - 1) as usize
            } else {
                alpha.1
            };
            cases.push(Alpha {
                path: path.clone(),
                alpha: (alpha.0, size),
                width,
                height,
                crop,
                alpha_dithering: [0, 30, 100][variant % 3],
                rows,
            });
        }
        for case in &cases {
            commands.push_str(&case.command());
            expected.push_str(&run_alpha(case, &file));
            calls += case.rows.len();
        }
    }
    let output = probe.run::<&str>(&[], commands.as_bytes());
    output.success();
    compare("alpha", &output.stdout, &expected);
    eprintln!("alpha: {calls} calls");
}
