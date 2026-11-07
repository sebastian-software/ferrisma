//! WP7 differential test: the lossless decoder of `src/dec/vp8l_dec.c` through
//! `VP8LDecodeHeader` and `VP8LDecodeImage`, as `src/dec/webp_dec.c` and the
//! incremental decoder drive it, against the C probe
//! `tools/oracle/probes/wp7/decode.c`.
//!
//! The inputs are the VP8L chunks of the test vectors: the 32 `lossless_vec_*`
//! files, `lossless_color_transform.webp`, `bad_palette_index.webp`,
//! `color_cache_bits_11.webp` and the other lossless files. Each is decoded in
//! every colorspace, with padded and flipped strides, cropped, scaled, with
//! invalid options, truncated at many points, incrementally in steps, and with
//! mutated bytes. After the header and each step the test compares the status,
//! the decoder state (bit reader, transforms, color cache, Huffman image, tree
//! groups and tables), every `VP8Io` field and the whole output buffer,
//! including the bytes a decoding must not touch. VP8L writes the rows into the
//! output buffer itself and calls no `put` hook, so the rows are compared there.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp7_support/mod.rs"]
mod wp7_support;

use std::fmt::Write as _;
use std::path::Path;

use oracle::Oracle;
use webp::dec::vp8_dec::VP8Io;
use webp::dec::vp8l_dec::{vp8l_decode_header, vp8l_decode_image, vp8l_get_info, vp8l_new};
use webp::dec::webpi_dec::WebPDecParams;
use webp::utils::bit_reader_utils::vp8l_bit_reader_set_buffer;
use webp::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions;
use webp::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPDecBuffer, WebPDecBufferPlanes, WebPDecoderOptions,
    WebPRGBABuffer, WebPYUVABuffer, webp_is_rgb_mode,
};
use webp::webp::types::{PlaneOffset, PlaneStorage};
use wp7_support::{
    Rng, compare, find_chunk, hex, print_decoder, print_decoder_full, print_digest, print_io,
};

/// One `decode` command of the probe.
#[derive(Clone)]
struct Decode {
    header_size: usize,
    mode: u32,
    use_options: bool,
    options: WebPDecoderOptions,
    out_w: i32,
    out_h: i32,
    strides: [i32; 4],
    flip: bool,
    steps: Vec<(usize, bool)>,
    dump: bool,
}

impl Decode {
    fn command(&self) -> String {
        let o = &self.options;
        let steps = if self.steps.is_empty() {
            "-".to_owned()
        } else {
            self.steps
                .iter()
                .map(|(size, incremental)| format!("{size},{}", i32::from(*incremental)))
                .collect::<Vec<_>>()
                .join(",")
        };
        format!(
            "decode {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}\n",
            self.header_size,
            self.mode,
            i32::from(self.use_options),
            o.use_cropping,
            o.crop_left,
            o.crop_top,
            o.crop_width,
            o.crop_height,
            o.use_scaling,
            o.scaled_width,
            o.scaled_height,
            o.bypass_filtering,
            o.no_fancy_upsampling,
            self.out_w,
            self.out_h,
            self.strides[0],
            self.strides[1],
            self.strides[2],
            self.strides[3],
            i32::from(self.flip),
            steps,
            i32::from(self.dump)
        )
    }
}

/// Runs a `decode` command on the Rust translation and prints what the probe
/// prints.
fn run(input: &[u8], case: &Decode, out: &mut String) {
    let mode = WEBP_CSP_MODE(case.mode);
    let is_rgb = webp_is_rgb_mode(mode);
    let uv_h = (case.out_h + 1) / 2;
    let rows = [case.out_h, uv_h, uv_h, case.out_h];
    let count = if is_rgb {
        1
    } else if case.strides[3] > 0 {
        4
    } else {
        3
    };
    let mut planes: Vec<Vec<u8>> = (0..count)
        .map(|i| vec![0xa5; case.strides[i] as usize * rows[i] as usize])
        .collect();
    let sign = if case.flip { -1 } else { 1 };
    let start = |i: usize| {
        let offset = if case.flip {
            (rows[i] - 1) * case.strides[i]
        } else {
            0
        };
        PlaneOffset {
            region: i,
            offset: offset as usize,
        }
    };
    let u = if is_rgb {
        WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
            rgba: start(0),
            stride: sign * case.strides[0],
            size: planes[0].len(),
        })
    } else {
        WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
            y: start(0),
            u: start(1),
            v: start(2),
            a: (count == 4).then(|| start(3)),
            y_stride: sign * case.strides[0],
            u_stride: sign * case.strides[1],
            v_stride: sign * case.strides[2],
            a_stride: if count == 4 {
                sign * case.strides[3]
            } else {
                0
            },
            y_size: planes[0].len(),
            u_size: planes[1].len(),
            v_size: planes[2].len(),
            a_size: if count == 4 { planes[3].len() } else { 0 },
        })
    };
    let mut output = WebPDecBuffer {
        colorspace: mode,
        width: case.out_w,
        height: case.out_h,
        slow_memory: false,
        u,
        pad: [0; 4],
        storage: PlaneStorage::External(planes.iter_mut().map(Vec::as_mut_slice).collect()),
    };

    let mut dec = vp8l_new();
    let mut io = VP8Io::default();
    let mut params = WebPDecParams {
        options: case.use_options.then_some(&case.options),
        ..WebPDecParams::default()
    };

    let mut ok = vp8l_decode_header(&mut dec, &mut io, &input[..case.header_size]).is_ok();
    writeln!(out, "header={}", i32::from(ok)).unwrap();
    print_io(out, &io);
    print_decoder_full(out, &dec);

    for &(size, incremental) in &case.steps {
        if !ok {
            break;
        }
        vp8l_bit_reader_set_buffer(&mut dec.br, size);
        dec.incremental = incremental;
        ok = vp8l_decode_image(&mut dec, &mut io, &input[..size], &mut params, &mut output).is_ok();
        writeln!(out, "step={size}").unwrap();
        writeln!(out, "image={}", i32::from(ok)).unwrap();
        writeln!(out, "last_y={}", params.last_y).unwrap();
        print_io(out, &io);
        print_decoder(out, &dec);
        for i in 0..count {
            let plane = output.storage.region(i).unwrap();
            let key = format!("plane{i}");
            if case.dump {
                writeln!(out, "{key}={}", hex(plane)).unwrap();
            } else {
                print_digest(out, &key, plane);
            }
        }
        if dec.status != VP8StatusCode::VP8_STATUS_SUSPENDED {
            break;
        }
    }
}

fn bytes_per_pixel(mode: u32) -> i32 {
    match WEBP_CSP_MODE(mode) {
        WEBP_CSP_MODE::MODE_RGB | WEBP_CSP_MODE::MODE_BGR => 3,
        WEBP_CSP_MODE::MODE_RGBA_4444
        | WEBP_CSP_MODE::MODE_rgbA_4444
        | WEBP_CSP_MODE::MODE_RGB_565 => 2,
        _ => 4,
    }
}

/// A decode of all `size` bytes in `mode` with `options`, into a buffer of the
/// size that `WebPAllocateDecBuffer` would choose, with `pad` extra bytes per
/// row.
fn case(
    size: usize,
    width: i32,
    height: i32,
    mode: u32,
    options: Option<WebPDecoderOptions>,
    pad: i32,
    flip: bool,
) -> Decode {
    let mut out_w = width;
    let mut out_h = height;
    let mut valid = true;
    if let Some(o) = options {
        if o.use_cropping != 0 {
            out_w = o.crop_width;
            out_h = o.crop_height;
        }
        if o.use_scaling != 0 {
            let mut scaled_w = o.scaled_width;
            let mut scaled_h = o.scaled_height;
            valid = webp_rescaler_get_scaled_dimensions(out_w, out_h, &mut scaled_w, &mut scaled_h);
            out_w = scaled_w;
            out_h = scaled_h;
        }
    }
    if !valid || out_w <= 0 || out_h <= 0 {
        // Invalid options: the decoding fails before writing; any buffer will do.
        out_w = width;
        out_h = height;
    }
    let strides = if webp_is_rgb_mode(WEBP_CSP_MODE(mode)) {
        [out_w * bytes_per_pixel(mode) + pad, 0, 0, 0]
    } else {
        let uv_w = (out_w + 1) / 2;
        let a_stride = if mode == WEBP_CSP_MODE::MODE_YUVA.0 {
            out_w + pad
        } else {
            0
        };
        [out_w + pad, uv_w + pad, uv_w + pad, a_stride]
    };
    Decode {
        header_size: size,
        mode,
        use_options: options.is_some(),
        options: options.unwrap_or_default(),
        out_w,
        out_h,
        strides,
        flip,
        steps: vec![(size, false)],
        dump: false,
    }
}

fn crop_options(rng: &mut Rng, width: i32, height: i32) -> WebPDecoderOptions {
    let crop_width = rng.range(1, i64::from(width)) as i32;
    let crop_height = rng.range(1, i64::from(height)) as i32;
    WebPDecoderOptions {
        use_cropping: 1,
        crop_left: rng.range(0, i64::from(width - crop_width)) as i32,
        crop_top: rng.range(0, i64::from(height - crop_height)) as i32,
        crop_width,
        crop_height,
        ..WebPDecoderOptions::default()
    }
}

fn scale_options(
    rng: &mut Rng,
    mut options: WebPDecoderOptions,
    width: i32,
    height: i32,
) -> WebPDecoderOptions {
    options.use_scaling = 1;
    let (w, h) = if options.use_cropping != 0 {
        (options.crop_width, options.crop_height)
    } else {
        (width, height)
    };
    match rng.below(5) {
        0 => {
            // Derive the height from the width.
            options.scaled_width = rng.range(1, i64::from(2 * w)) as i32;
            options.scaled_height = 0;
        }
        1 => {
            options.scaled_width = 0;
            options.scaled_height = rng.range(1, i64::from(2 * h)) as i32;
        }
        2 => {
            // Upscaling.
            options.scaled_width = rng.range(i64::from(w), i64::from(3 * w)) as i32;
            options.scaled_height = rng.range(i64::from(h), i64::from(3 * h)) as i32;
        }
        _ => {
            // Downscaling.
            options.scaled_width = rng.range(1, i64::from(w)) as i32;
            options.scaled_height = rng.range(1, i64::from(h)) as i32;
        }
    }
    options
}

/// The configurations of one vector of `size` bytes and `width` by `height`
/// pixels: every colorspace, strides, crops, scales and invalid options.
fn configurations(rng: &mut Rng, size: usize, width: i32, height: i32) -> Vec<Decode> {
    let mut cases = Vec::new();
    let small = i64::from(width) * i64::from(height) * 4 <= 256 * 1024;
    let mut reference = case(
        size,
        width,
        height,
        WEBP_CSP_MODE::MODE_BGRA.0,
        None,
        0,
        false,
    );
    reference.dump = small;
    cases.push(reference);
    for mode in 0..WEBP_CSP_MODE::MODE_LAST.0 {
        cases.push(case(size, width, height, mode, None, 0, false));
        let pad = rng.range(1, 7) as i32;
        cases.push(case(
            size,
            width,
            height,
            mode,
            None,
            pad,
            rng.below(2) == 0,
        ));
    }
    // Options without cropping or scaling: only the io flags change.
    let plain = WebPDecoderOptions {
        bypass_filtering: 1,
        no_fancy_upsampling: 1,
        ..WebPDecoderOptions::default()
    };
    cases.push(case(
        size,
        width,
        height,
        WEBP_CSP_MODE::MODE_RGBA.0,
        Some(plain),
        0,
        false,
    ));
    // Cropping in RGB and YUV modes, with the whole picture and single pixels.
    let full = WebPDecoderOptions {
        use_cropping: 1,
        crop_width: width,
        crop_height: height,
        ..WebPDecoderOptions::default()
    };
    cases.push(case(
        size,
        width,
        height,
        WEBP_CSP_MODE::MODE_BGRA.0,
        Some(full),
        0,
        false,
    ));
    let corner = WebPDecoderOptions {
        use_cropping: 1,
        crop_left: width - 1,
        crop_top: height - 1,
        crop_width: 1,
        crop_height: 1,
        ..WebPDecoderOptions::default()
    };
    cases.push(case(
        size,
        width,
        height,
        WEBP_CSP_MODE::MODE_ARGB.0,
        Some(corner),
        0,
        false,
    ));
    for _ in 0..6 {
        let options = crop_options(rng, width, height);
        let mode = rng.below(u64::from(WEBP_CSP_MODE::MODE_LAST.0)) as u32;
        cases.push(case(
            size,
            width,
            height,
            mode,
            Some(options),
            rng.range(0, 3) as i32,
            rng.below(3) == 0,
        ));
    }
    // Scaling, alone and after cropping, in RGB and YUV modes.
    for index in 0..8 {
        let base = if index % 2 == 0 {
            WebPDecoderOptions::default()
        } else {
            crop_options(rng, width, height)
        };
        let options = scale_options(rng, base, width, height);
        let mode = if index < 4 {
            [
                WEBP_CSP_MODE::MODE_RGBA.0,
                WEBP_CSP_MODE::MODE_rgbA.0,
                WEBP_CSP_MODE::MODE_BGR.0,
                WEBP_CSP_MODE::MODE_RGBA_4444.0,
            ][index]
        } else {
            [
                WEBP_CSP_MODE::MODE_YUVA.0,
                WEBP_CSP_MODE::MODE_YUV.0,
                WEBP_CSP_MODE::MODE_Argb.0,
                WEBP_CSP_MODE::MODE_RGB_565.0,
            ][index - 4]
        };
        cases.push(case(
            size,
            width,
            height,
            mode,
            Some(options),
            rng.range(0, 3) as i32,
            rng.below(3) == 0,
        ));
    }
    // Invalid options: the crop leaves the picture, or the scaled size is negative.
    let outside = WebPDecoderOptions {
        use_cropping: 1,
        crop_left: width - 1,
        crop_width: 2,
        crop_height: 1,
        ..WebPDecoderOptions::default()
    };
    cases.push(case(
        size,
        width,
        height,
        WEBP_CSP_MODE::MODE_RGBA.0,
        Some(outside),
        0,
        false,
    ));
    let negative = WebPDecoderOptions {
        use_scaling: 1,
        scaled_width: -4,
        scaled_height: 10,
        ..WebPDecoderOptions::default()
    };
    cases.push(case(
        size,
        width,
        height,
        WEBP_CSP_MODE::MODE_RGBA.0,
        Some(negative),
        0,
        false,
    ));
    cases
}

/// The smallest prefix from which the header decodes, as the incremental
/// decoder finds it by trying again with more data.
fn header_prefix(input: &[u8]) -> Option<usize> {
    (1..=input.len()).find(|&size| {
        let mut dec = vp8l_new();
        let mut io = VP8Io::default();
        vp8l_decode_header(&mut dec, &mut io, &input[..size]).is_ok()
    })
}

/// Truncations: the header and the image from the first `size` bytes only.
fn truncations(rng: &mut Rng, input: &[u8], width: i32, height: i32) -> Vec<Decode> {
    let mut sizes: Vec<usize> = (0..input.len().min(24)).collect();
    for k in 1..24 {
        sizes.push(input.len() * k / 24);
    }
    for _ in 0..8 {
        sizes.push(rng.below(input.len() as u64) as usize);
    }
    if let Some(header) = header_prefix(input) {
        sizes.extend(
            [header - 1, header, header + 1]
                .into_iter()
                .filter(|&s| s < input.len()),
        );
    }
    sizes.sort_unstable();
    sizes.dedup();
    sizes
        .into_iter()
        .map(|size| {
            let mode = rng.below(u64::from(WEBP_CSP_MODE::MODE_LAST.0)) as u32;
            case(size, width, height, mode, None, 0, false)
        })
        .collect()
}

/// Incremental decodings: the header from the smallest prefix that holds it,
/// then the image in steps of growing size, as `src/dec/idec_dec.c` feeds it.
fn incremental(rng: &mut Rng, input: &[u8], width: i32, height: i32) -> Vec<Decode> {
    let Some(header) = header_prefix(input) else {
        return Vec::new();
    };
    let mut cases = Vec::new();
    for round in 0..4 {
        let mut steps = Vec::new();
        let mut size = header;
        let count = [1, 7, 23, 64][round];
        let increment = ((input.len() - header) / count).max(1);
        while size < input.len() {
            steps.push((size, true));
            size += rng.range(1, increment as i64 * 2) as usize;
        }
        steps.push((input.len(), false));
        let options = match round {
            2 => Some(crop_options(rng, width, height)),
            3 => Some(scale_options(
                rng,
                WebPDecoderOptions::default(),
                width,
                height,
            )),
            _ => None,
        };
        let mode = [
            WEBP_CSP_MODE::MODE_BGRA.0,
            WEBP_CSP_MODE::MODE_YUVA.0,
            WEBP_CSP_MODE::MODE_RGB.0,
            WEBP_CSP_MODE::MODE_rgbA.0,
        ][round];
        let mut decode = case(input.len(), width, height, mode, options, 0, round == 1);
        decode.header_size = header;
        decode.steps = steps;
        cases.push(decode);
    }
    // The data ends early: suspended steps, then a last step without the
    // incremental flag, which fails after the suspension (the first-error
    // precedence of VP8LSetError lets the bitstream error replace
    // VP8_STATUS_SUSPENDED).
    if input.len() > header + 2 {
        let end = rng.range(header as i64 + 1, input.len() as i64 - 1) as usize;
        let mut steps = steps_between(rng, header, end, 5);
        steps.push((end, false));
        let mut decode = case(
            input.len(),
            width,
            height,
            WEBP_CSP_MODE::MODE_BGRA.0,
            None,
            0,
            false,
        );
        decode.header_size = header;
        decode.steps = steps;
        cases.push(decode);
    }
    cases
}

/// Incremental steps from `start` up to, but not including, `end`.
fn steps_between(rng: &mut Rng, start: usize, end: usize, count: usize) -> Vec<(usize, bool)> {
    let increment = ((end - start) / count).max(1);
    let mut steps = Vec::new();
    let mut size = start;
    while size < end {
        steps.push((size, true));
        size += rng.range(1, increment as i64 * 2) as usize;
    }
    steps
}

/// Decodes of copies of `input` with changed bytes after the 5-byte header.
fn mutations(
    rng: &mut Rng,
    input: &[u8],
    width: i32,
    height: i32,
    count: usize,
) -> Vec<(Vec<u8>, Decode)> {
    let mut cases = Vec::new();
    for _ in 0..count {
        let mut data = input.to_vec();
        let changes = rng.range(1, 4);
        for _ in 0..changes {
            let position = rng.range(5, data.len() as i64 - 1) as usize;
            data[position] ^= 1 << rng.below(8);
        }
        let mut decode = case(
            data.len(),
            width,
            height,
            WEBP_CSP_MODE::MODE_BGRA.0,
            None,
            0,
            false,
        );
        // Every other mutation is decoded incrementally, so that an error can
        // follow a suspension.
        if rng.below(2) == 0
            && let Some(header) = header_prefix(&data)
        {
            let mut steps = steps_between(rng, header, data.len(), 6);
            steps.push((data.len(), false));
            decode.header_size = header;
            decode.steps = steps;
        }
        cases.push((data, decode));
    }
    cases
}

/// Runs the probe on a list of inputs and their commands, and compares.
fn check(oracle: &Oracle, test: &str, runs: &[(Input<'_>, Vec<Decode>)]) -> usize {
    let mut commands = String::new();
    let mut expected = String::new();
    let mut count = 0;
    for (input, cases) in runs {
        match input {
            Input::File { path, offset, data } => {
                writeln!(commands, "file {} {offset} {}", path.display(), data.len()).unwrap();
            }
            Input::Bytes(data) => writeln!(commands, "data {}", hex(data)).unwrap(),
        }
        for decode in cases {
            commands.push_str(&decode.command());
            run(input.data(), decode, &mut expected);
            count += 1;
        }
    }
    let output = oracle
        .probe("wp7", "decode")
        .run::<&str>(&[], commands.as_bytes());
    output.success();
    compare(test, &output.stdout, &expected);
    eprintln!("{test}: {}", outcomes(&expected));
    count
}

/// Counts the outcomes in the expected output: failed and successful headers,
/// and the status after each step.
fn outcomes(expected: &str) -> String {
    let mut headers = [0; 2];
    let mut statuses = std::collections::BTreeMap::new();
    let mut in_step = false;
    for line in expected.lines() {
        if let Some(value) = line.strip_prefix("header=") {
            headers[usize::from(value == "1")] += 1;
            in_step = false;
        } else if line.starts_with("step=") {
            in_step = true;
        } else if in_step && let Some(value) = line.strip_prefix("status=") {
            *statuses.entry(value.to_owned()).or_insert(0) += 1;
            in_step = false;
        }
    }
    format!(
        "headers failed {}, decoded {}; step statuses {statuses:?}",
        headers[0], headers[1]
    )
}

enum Input<'d> {
    File {
        path: &'d Path,
        offset: usize,
        data: &'d [u8],
    },
    Bytes(&'d [u8]),
}

impl Input<'_> {
    fn data(&self) -> &[u8] {
        match self {
            Input::File { data, .. } | Input::Bytes(data) => data,
        }
    }
}

/// The VP8L vectors of the test data with their payload offset.
fn vectors(filter: impl Fn(&str) -> bool) -> Vec<(std::path::PathBuf, usize, Vec<u8>)> {
    oracle::test_vectors()
        .into_iter()
        .filter(|path| filter(path.file_name().unwrap().to_str().unwrap()))
        .filter_map(|path| {
            let file = std::fs::read(&path).unwrap();
            let (offset, size) = find_chunk(&file, b"VP8L")?;
            let payload = file[offset..offset + size].to_vec();
            Some((path, offset, payload))
        })
        .collect()
}

fn decode_vectors(test: &str, seed: u64, filter: impl Fn(&str) -> bool, mutation_count: usize) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let mut rng = Rng(seed);
    let mut total = 0;
    let vectors = vectors(filter);
    assert!(!vectors.is_empty(), "{test}: no vectors");
    for (path, offset, payload) in &vectors {
        let (width, height, _) = vp8l_get_info(payload).expect("a valid VP8L header");
        let mut cases = configurations(&mut rng, payload.len(), width, height);
        cases.extend(truncations(&mut rng, payload, width, height));
        cases.extend(incremental(&mut rng, payload, width, height));
        let mut runs = vec![(
            Input::File {
                path,
                offset: *offset,
                data: payload,
            },
            cases,
        )];
        let mutated = mutations(&mut rng, payload, width, height, mutation_count);
        for (data, decode) in &mutated {
            runs.push((Input::Bytes(data), vec![decode.clone()]));
        }
        total += check(&oracle, test, &runs);
    }
    eprintln!(
        "{test}: {} vectors, {total} decodes compared",
        vectors.len()
    );
}

#[test]
fn lossless_vec_vectors_match_the_c_decoder() {
    decode_vectors(
        "lossless_vec_vectors_match_the_c_decoder",
        0x5eed_0703,
        |name| name.starts_with("lossless_vec_"),
        24,
    );
}

#[test]
fn color_transform_vector_matches_the_c_decoder() {
    decode_vectors(
        "color_transform_vector_matches_the_c_decoder",
        0x5eed_0704,
        |name| name == "lossless_color_transform.webp",
        24,
    );
}

#[test]
fn other_lossless_vectors_match_the_c_decoder() {
    decode_vectors(
        "other_lossless_vectors_match_the_c_decoder",
        0x5eed_0705,
        |name| {
            matches!(
                name,
                "bad_palette_index.webp"
                    | "color_cache_bits_11.webp"
                    | "dual_transform.webp"
                    | "one_color_no_palette.webp"
                    | "near_lossless_75.webp"
                    | "lossless1.webp"
                    | "lossless2.webp"
                    | "lossless3.webp"
                    | "lossless4.webp"
            )
        },
        24,
    );
}

#[test]
fn large_random_alpha_vector_matches_the_c_decoder() {
    let test = "large_random_alpha_vector_matches_the_c_decoder";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let vectors = vectors(|name| name == "lossless_big_random_alpha.webp");
    let (path, offset, payload) = &vectors[0];
    let (width, height, _) = vp8l_get_info(payload).unwrap();
    let mut rng = Rng(0x5eed_0706);
    let mut cases = vec![case(
        payload.len(),
        width,
        height,
        WEBP_CSP_MODE::MODE_RGBA.0,
        None,
        0,
        false,
    )];
    let cropped = crop_options(&mut rng, width, height);
    let options = scale_options(&mut rng, cropped, width, height);
    cases.push(case(
        payload.len(),
        width,
        height,
        WEBP_CSP_MODE::MODE_YUVA.0,
        Some(options),
        0,
        false,
    ));
    let count = check(
        &oracle,
        test,
        &[(
            Input::File {
                path,
                offset: *offset,
                data: payload,
            },
            cases,
        )],
    );
    eprintln!("{test}: {count} decodes compared");
}

/// Seeded RGBA content for a `width` by `height` picture: few colors (palettes
/// and color indexing with packed pixels), gradients, repeated patterns (long
/// backward references at every distance), noise and sparse alpha.
fn picture(rng: &mut Rng, width: usize, height: usize) -> Vec<u8> {
    let kind = rng.below(5);
    let colors: Vec<[u8; 4]> = (0..rng.range(1, 300))
        .map(|_| {
            [
                rng.byte(),
                rng.byte(),
                rng.byte(),
                if rng.below(3) == 0 { rng.byte() } else { 255 },
            ]
        })
        .collect();
    let period_x = rng.range(1, 9) as usize;
    let period_y = rng.range(1, 5) as usize;
    let (a, b) = (rng.range(0, 9) as usize, rng.range(0, 9) as usize);
    let mut pixels = Vec::with_capacity(4 * width * height);
    for y in 0..height {
        for x in 0..width {
            let pixel = match kind {
                0 => colors[rng.below(colors.len() as u64) as usize],
                1 => {
                    let v = (x * a + y * b) as u8;
                    [v, v.wrapping_mul(3), 255 - v, 255]
                }
                2 => colors[((x % period_x) + 3 * (y % period_y)) % colors.len()],
                3 => [rng.byte(), rng.byte(), rng.byte(), rng.byte()],
                _ => {
                    if rng.below(8) == 0 {
                        colors[rng.below(colors.len() as u64) as usize]
                    } else {
                        colors[0]
                    }
                }
            };
            pixels.extend_from_slice(&pixel);
        }
    }
    pixels
}

/// Encodes seeded pictures with the oracle's `cwebp -lossless` and returns the
/// VP8L payloads: small widths (below 8 pixels a backward reference can map to a
/// distance below 1), odd sizes, every method and quality.
fn generated_payloads(oracle: &Oracle, rng: &mut Rng, count: usize) -> Vec<Vec<u8>> {
    let cwebp = oracle.tool("cwebp");
    let directory = std::env::temp_dir().join(format!("libwebp-rs-wp7-{}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    let mut payloads = Vec::new();
    for index in 0..count {
        let width = if rng.below(4) == 0 {
            rng.range(8, 64)
        } else {
            rng.range(1, 7)
        } as usize;
        let height = rng.range(1, 40) as usize;
        let pixels = picture(rng, width, height);
        let input = directory.join(format!("{index}.pam"));
        let output = directory.join(format!("{index}.webp"));
        let mut pam = format!(
            "P7\nWIDTH {width}\nHEIGHT {height}\nDEPTH 4\nMAXVAL 255\nTUPLTYPE RGB_ALPHA\nENDHDR\n"
        )
        .into_bytes();
        pam.extend_from_slice(&pixels);
        std::fs::write(&input, pam).unwrap();
        let quality = rng.range(0, 100).to_string();
        let method = rng.range(0, 6).to_string();
        let mut args = vec!["-quiet", "-lossless", "-q", &quality, "-m", &method];
        if rng.below(2) == 0 {
            args.push("-exact");
        }
        let status = std::process::Command::new(&cwebp)
            .args(&args)
            .arg(&input)
            .arg("-o")
            .arg(&output)
            .status()
            .expect("cwebp runs");
        assert!(status.success(), "cwebp failed on {}", input.display());
        let file = std::fs::read(&output).unwrap();
        let (offset, size) = find_chunk(&file, b"VP8L").expect("cwebp -lossless writes VP8L");
        payloads.push(file[offset..offset + size].to_vec());
        std::fs::remove_file(&input).unwrap();
        std::fs::remove_file(&output).unwrap();
    }
    std::fs::remove_dir(&directory).unwrap();
    payloads
}

#[test]
fn generated_pictures_match_the_c_decoder() {
    let test = "generated_pictures_match_the_c_decoder";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let mut rng = Rng(0x5eed_0709);
    let payloads = generated_payloads(&oracle, &mut rng, 120);
    let mut total = 0;
    for chunk in payloads.chunks(12) {
        let mut runs = Vec::new();
        let mut mutated_all = Vec::new();
        for payload in chunk {
            let (width, height, _) = vp8l_get_info(payload).expect("a valid VP8L header");
            let mut reference = case(
                payload.len(),
                width,
                height,
                WEBP_CSP_MODE::MODE_BGRA.0,
                None,
                0,
                false,
            );
            reference.dump = true;
            let mut cases = vec![reference];
            for _ in 0..3 {
                let mode = rng.below(u64::from(WEBP_CSP_MODE::MODE_LAST.0)) as u32;
                let options = match rng.below(3) {
                    0 => None,
                    1 => Some(crop_options(&mut rng, width, height)),
                    _ => {
                        let cropped = crop_options(&mut rng, width, height);
                        Some(scale_options(&mut rng, cropped, width, height))
                    }
                };
                cases.push(case(
                    payload.len(),
                    width,
                    height,
                    mode,
                    options,
                    rng.range(0, 3) as i32,
                    rng.below(2) == 0,
                ));
            }
            cases.extend(
                truncations(&mut rng, payload, width, height)
                    .into_iter()
                    .step_by(3),
            );
            cases.extend(incremental(&mut rng, payload, width, height));
            mutated_all.push(mutations(&mut rng, payload, width, height, 16));
            runs.push((Input::Bytes(payload), cases));
        }
        for mutated in &mutated_all {
            for (data, decode) in mutated {
                runs.push((Input::Bytes(data), vec![decode.clone()]));
            }
        }
        total += check(&oracle, test, &runs);
    }
    eprintln!(
        "{test}: {} pictures, {total} decodes compared",
        payloads.len()
    );
}

/// Writes bits least significant first, as the VP8L reader reads them.
#[derive(Default)]
struct Bits {
    bytes: Vec<u8>,
    count: usize,
}

impl Bits {
    fn put(&mut self, value: u32, n_bits: u32) {
        for i in 0..n_bits {
            if self.count.is_multiple_of(8) {
                self.bytes.push(0);
            }
            *self.bytes.last_mut().unwrap() |= (((value >> i) & 1) as u8) << (self.count % 8);
            self.count += 1;
        }
    }
}

/// The extra bits of a length or distance prefix symbol (C: `GetCopyDistance`).
fn extra_bits(symbol: u32) -> u32 {
    if symbol < 4 { 0 } else { (symbol - 2) >> 1 }
}

/// A hand-built VP8L stream of `width` by `height` pixels without transforms:
/// a green code of one literal and one length prefix, single-symbol red, blue,
/// alpha and distance codes, and a random sequence of literals and backward
/// references with random extra bits. Small widths with the plane codes of
/// large x-offsets give distances below 1, which the decoder clamps; long
/// lengths and distances run past the picture, which it rejects.
fn crafted_stream(rng: &mut Rng, width: u32, height: u32) -> Vec<u8> {
    let mut bits = Bits::default();
    bits.put(0x2f, 8);
    bits.put(width - 1, 14);
    bits.put(height - 1, 14);
    bits.put(rng.below(2) as u32, 1);
    bits.put(0, 3);
    bits.put(0, 1); // no transform
    bits.put(0, 1); // no color cache
    bits.put(0, 1); // no meta Huffman image
    let literal = rng.below(256) as u32;
    // Mostly short lengths and the distance symbols of plane codes up to 96,
    // so that most streams stay inside the picture.
    let length_symbol = if rng.below(4) == 0 {
        rng.below(24)
    } else {
        rng.below(5)
    } as u32;
    let distance_symbol = if rng.below(4) == 0 {
        rng.below(40)
    } else {
        rng.below(13)
    } as u32;
    // Green: a code-length code of the symbols 0 and 1 (lengths 1), then 280
    // code lengths of 1 bit each: length 1 for the literal and the prefix.
    bits.put(0, 1);
    bits.put(0, 4); // four code length code lengths: 17, 18, 0, 1
    bits.put(0, 3);
    bits.put(0, 3);
    bits.put(1, 3);
    bits.put(1, 3);
    bits.put(0, 1); // no max_symbol
    for symbol in 0..280 {
        bits.put(
            u32::from(symbol == literal || symbol == 256 + length_symbol),
            1,
        );
    }
    // Red: a simple code of two 8-bit symbols, one bit per literal, so that the
    // literals differ. Blue, alpha and distance: simple codes of one symbol.
    let red = rng.below(255) as u32;
    bits.put(1, 1);
    bits.put(1, 1);
    bits.put(1, 1);
    bits.put(red, 8);
    bits.put(red + 1 + rng.below(u64::from(255 - red)) as u32, 8);
    for symbol in [
        rng.below(256) as u32,
        rng.below(256) as u32,
        distance_symbol,
    ] {
        bits.put(1, 1);
        bits.put(0, 1);
        bits.put(1, 1);
        bits.put(symbol, 8);
    }
    let total = u64::from(width) * u64::from(height);
    let mut pos = 0_u64;
    while pos < total {
        let n = extra_bits(length_symbol);
        let extra = rng.below(1 << n) as u32;
        let length = if length_symbol < 4 {
            u64::from(length_symbol) + 1
        } else {
            (u64::from(2 + (length_symbol & 1)) << n) + u64::from(extra) + 1
        };
        // A reference past the end is kept now and then; otherwise a literal.
        let overshoot = pos + length > total && rng.below(8) != 0;
        // Literals first, so that most references find their distance.
        let lead = (total / 2).min(8 * u64::from(width) + 8).max(1);
        if pos < lead || overshoot || rng.below(3) == 0 {
            bits.put(0, 1); // the literal has code 0
            bits.put(rng.below(2) as u32, 1); // its red symbol
            pos += 1;
        } else {
            bits.put(1, 1); // the length prefix has code 1
            bits.put(extra, n);
            bits.put(
                rng.below(1 << extra_bits(distance_symbol)) as u32,
                extra_bits(distance_symbol),
            );
            pos += length;
        }
    }
    bits.bytes
}

#[test]
fn crafted_distances_match_the_c_decoder() {
    let test = "crafted_distances_match_the_c_decoder";
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let mut rng = Rng(0x5eed_070a);
    let mut streams = Vec::new();
    for index in 0..600 {
        let width = if index % 3 == 0 {
            rng.range(8, 40)
        } else {
            rng.range(1, 7)
        } as u32;
        let height = rng.range(1, 24) as u32;
        streams.push(crafted_stream(&mut rng, width, height));
    }
    let mut total = 0;
    for chunk in streams.chunks(100) {
        let mut runs = Vec::new();
        for stream in chunk {
            let (width, height, _) = vp8l_get_info(stream).expect("a valid VP8L header");
            let mut reference = case(
                stream.len(),
                width,
                height,
                WEBP_CSP_MODE::MODE_BGRA.0,
                None,
                0,
                false,
            );
            reference.dump = true;
            let mut cases = vec![reference];
            cases.extend(
                incremental(&mut rng, stream, width, height)
                    .into_iter()
                    .take(1),
            );
            runs.push((Input::Bytes(stream), cases));
        }
        total += check(&oracle, test, &runs);
    }
    eprintln!(
        "{test}: {} streams, {total} decodes compared",
        streams.len()
    );
}
