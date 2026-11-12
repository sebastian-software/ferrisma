//! M2 work package 2: AVIF encoder throughput at matched SSIMULACRA2 quality.
//!
//! Inputs are real photographs, decoded and area-downscaled to the width the
//! benchmark harness uses (1600 pixels) in the sRGB domain. The resampling is
//! only there to produce realistic encoder input; it is not Ferrisma's
//! color-managed path. Every encoded file is decoded with `avif-decode` and
//! scored with SSIMULACRA2 against that input frame.
//!
//! Anchor: what Ferrisma ships today, `ravif` at speed 6, one thread,
//! quality 80. Every other variant is quality-matched: its quality knob is
//! bisected until the SSIMULACRA2 score brackets the anchor's score, and
//! bytes and time are interpolated at the anchor's score.
//!
//! Build twice to separate the compile-time features:
//!   cargo run --release -- anchor IMAGES...                       (scalar)
//!   cargo run --release --features asm,threading -- matrix IMAGES...
//! IMAGES are `name=path` or `name=path@width`; `.jpg` and 8-bit 4:2:0
//! full-range `.y4m` are accepted.

use std::fmt::Write as _;
use std::time::{Duration, Instant};

use rav1e::prelude::*;
use ravif::{Img, RGB8};

#[derive(Clone)]
struct Image {
    name: String,
    width: usize,
    height: usize,
    rgb: Vec<RGB8>,
}

fn load_jpeg(path: &str) -> (usize, usize, Vec<u8>) {
    use zune_jpeg::zune_core::{bytestream::ZCursor, colorspace::ColorSpace, options::DecoderOptions};
    let bytes = std::fs::read(path).expect("read jpeg");
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = zune_jpeg::JpegDecoder::new_with_options(ZCursor::new(&bytes[..]), options);
    let pixels = decoder.decode().expect("decode jpeg");
    let (width, height) = decoder.dimensions().expect("dimensions");
    (width, height, pixels)
}

/// 8-bit 4:2:0, full range, BT.601 (the Kodak files in libavif's test data).
fn load_y4m(path: &str) -> (usize, usize, Vec<u8>) {
    let bytes = std::fs::read(path).expect("read y4m");
    let header_end = bytes.iter().position(|&b| b == b'\n').expect("header");
    let header = std::str::from_utf8(&bytes[..header_end]).expect("ascii header");
    let field = |tag: char| {
        header
            .split(' ')
            .find_map(|part| part.strip_prefix(tag))
            .unwrap_or_else(|| panic!("y4m field {tag}"))
    };
    let width: usize = field('W').parse().unwrap();
    let height: usize = field('H').parse().unwrap();
    assert!(field('C').starts_with("420"), "only 4:2:0 y4m");
    assert!(header.contains("XCOLORRANGE=FULL"), "only full-range y4m");
    let frame = header_end + 1 + bytes[header_end + 1..].iter().position(|&b| b == b'\n').unwrap() + 1;
    let (cw, ch) = (width.div_ceil(2), height.div_ceil(2));
    let y = &bytes[frame..frame + width * height];
    let u = &bytes[frame + width * height..][..cw * ch];
    let v = &bytes[frame + width * height + cw * ch..][..cw * ch];
    let mut rgb = Vec::with_capacity(width * height * 3);
    for row in 0..height {
        for col in 0..width {
            let luma = f32::from(y[row * width + col]);
            let cb = f32::from(u[(row / 2) * cw + col / 2]) - 128.0;
            let cr = f32::from(v[(row / 2) * cw + col / 2]) - 128.0;
            let r = luma + 1.402 * cr;
            let g = luma - 0.344_136 * cb - 0.714_136 * cr;
            let b = luma + 1.772 * cb;
            rgb.extend([r, g, b].map(|c| c.round().clamp(0.0, 255.0) as u8));
        }
    }
    (width, height, rgb)
}

/// Area (box) downscale in the sRGB domain; content only, not color-managed.
fn area_downscale(width: usize, height: usize, rgb: &[u8], target_width: usize) -> (usize, usize, Vec<u8>) {
    if target_width >= width {
        return (width, height, rgb.to_vec());
    }
    let target_height = (height * target_width + width / 2) / width;
    let sx = width as f64 / target_width as f64;
    let sy = height as f64 / target_height as f64;
    let weights = |out: usize, scale: f64, len: usize| {
        let (start, end) = (out as f64 * scale, (out as f64 + 1.0) * scale);
        let mut taps = Vec::new();
        let mut index = start.floor() as usize;
        while (index as f64) < end && index < len {
            let w = (end.min(index as f64 + 1.0) - start.max(index as f64)).max(0.0);
            if w > 0.0 {
                taps.push((index, w / scale));
            }
            index += 1;
        }
        taps
    };
    let columns: Vec<_> = (0..target_width).map(|x| weights(x, sx, width)).collect();
    let mut horizontal = vec![0.0_f64; target_width * height * 3];
    for row in 0..height {
        for (x, taps) in columns.iter().enumerate() {
            for &(index, w) in taps {
                for c in 0..3 {
                    horizontal[(row * target_width + x) * 3 + c] += w * f64::from(rgb[(row * width + index) * 3 + c]);
                }
            }
        }
    }
    let mut out = vec![0_u8; target_width * target_height * 3];
    for y in 0..target_height {
        let mut acc = vec![0.0_f64; target_width * 3];
        for (index, w) in weights(y, sy, height) {
            for (a, h) in acc.iter_mut().zip(&horizontal[index * target_width * 3..][..target_width * 3]) {
                *a += w * h;
            }
        }
        for (o, a) in out[y * target_width * 3..][..target_width * 3].iter_mut().zip(acc) {
            *o = a.round().clamp(0.0, 255.0) as u8;
        }
    }
    (target_width, target_height, out)
}

fn load(spec: &str) -> Image {
    let (name, rest) = spec.split_once('=').expect("name=path[@width]");
    let (path, target) = match rest.rsplit_once('@') {
        Some((path, width)) => (path, Some(width.parse::<usize>().expect("width"))),
        None => (rest, None),
    };
    let (w, h, rgb) = if path.ends_with(".y4m") { load_y4m(path) } else { load_jpeg(path) };
    let (width, height, rgb) = match target {
        Some(target) => area_downscale(w, h, &rgb, target),
        None => (w, h, rgb),
    };
    Image {
        name: name.to_owned(),
        width,
        height,
        rgb: rgb.chunks_exact(3).map(|p| RGB8::new(p[0], p[1], p[2])).collect(),
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Chroma {
    Cs444,
    Cs420,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Variant {
    /// `ravif` as Ferrisma calls it (4:4:4, 10-bit, BT.601 full range).
    Ravif { speed: u8, threads: usize },
    /// `rav1e` driven directly with `ravif`'s speed tweaks, so that chroma
    /// subsampling and bit depth are the only differences.
    Direct { speed: u8, threads: usize, chroma: Chroma, depth: u8 },
    /// `rav1e` with its own preset plus the four still-image settings only
    /// (no `ravif` tweaks), and a tile count fixed by the image size instead
    /// of the thread count: the candidate for Ferrisma's own sink.
    Preset { speed: u8, threads: usize, chroma: Chroma, size_tiles: bool },
}

impl Variant {
    fn label(self) -> String {
        match self {
            Self::Ravif { speed, threads } => format!("ravif 4:4:4 10-bit s{speed} t{threads}"),
            Self::Direct { speed, threads, chroma, depth } => {
                let chroma = if chroma == Chroma::Cs420 { "4:2:0" } else { "4:4:4" };
                format!("rav1e {chroma} {depth}-bit s{speed} t{threads}")
            }
            Self::Preset { speed, threads, chroma, size_tiles } => {
                let chroma = if chroma == Chroma::Cs420 { "4:2:0" } else { "4:4:4" };
                let tiles = if size_tiles { "size tiles" } else { "1 tile" };
                format!("rav1e preset {chroma} 10-bit s{speed} t{threads} {tiles}")
            }
        }
    }
}

/// `ravif` 0.13.0 `quality_to_quantizer` (BSD-3-Clause), so that the quality
/// knob means the same quantizer in both encoder paths.
fn quality_to_quantizer(quality: f32) -> u8 {
    let q = quality / 100.;
    let x = if q >= 0.82 {
        (1. - q) * 2.6
    } else if q > 0.25 {
        q.mul_add(-0.5, 1. - 0.125)
    } else {
        1. - q
    };
    (x * 255.).round() as u8
}

/// `ravif` 0.13.0 `SpeedTweaks::from_my_preset` plus `speed_settings`
/// (BSD-3-Clause), condensed; returns the settings and the minimum tile size.
fn speed_settings(speed: u8, quantizer: u8) -> (SpeedSettings, usize) {
    let low_quality = quantizer < quality_to_quantizer(55.);
    let high_quality = quantizer > quality_to_quantizer(80.);
    let max_block_size = if high_quality { 16 } else { 64 };
    let mut s = SpeedSettings::from_preset(speed);
    s.multiref = false;
    s.rdo_lookahead_frames = 1;
    s.scene_detection_mode = SceneDetectionSpeed::None;
    s.motion.include_near_mvs = false;
    let (min, max) = match speed {
        0 => (4, 64.min(max_block_size)),
        1 if low_quality => (4, 64.min(max_block_size)),
        2 if low_quality => (4, 32.min(max_block_size)),
        1..=4 => (4, 16),
        5..=8 => (8, 16),
        _ => (16, 16),
    };
    let size = |v: u8| match v {
        4 => BlockSize::BLOCK_4X4,
        8 => BlockSize::BLOCK_8X8,
        16 => BlockSize::BLOCK_16X16,
        32 => BlockSize::BLOCK_32X32,
        _ => BlockSize::BLOCK_64X64,
    };
    s.partition.partition_range = PartitionRange::new(size(min), size(max));
    s.prediction.prediction_modes = if speed <= 1 {
        PredictionModesSetting::ComplexAll
    } else {
        PredictionModesSetting::Simple
    };
    s.sgr_complexity = if speed <= 2 { SGRComplexityLevel::Full } else { SGRComplexityLevel::Reduced };
    s.partition.encode_bottomup = speed <= 2;
    s.transform.rdo_tx_decision = speed <= 4 && !high_quality;
    s.transform.reduced_tx_set = speed == 4 || speed >= 9;
    s.prediction.fine_directional_intra = speed <= 6;
    s.fast_deblock = speed >= 7 && !high_quality;
    s.lrf = low_quality && speed <= 8;
    s.cdef = low_quality && speed <= 9;
    s.transform.enable_inter_tx_split = speed >= 9;
    s.transform.tx_domain_rate = speed >= 10;
    s.motion.use_satd_subpel = false;
    let min_tile = match speed {
        0 => 4096,
        1 => 2048,
        2 => 1024,
        3 => 512,
        4 => 256,
        _ => 128,
    } * if high_quality { 2 } else { 1 };
    (s, min_tile)
}

const BT601: [f32; 3] = [0.299, 0.587, 0.114];

/// Unrounded BT.601 full-range YCbCr at `depth` bits (`ravif`'s formula).
fn ycbcr(px: RGB8, depth: u8) -> [f32; 3] {
    let max_value = ((1_u32 << depth) - 1) as f32;
    let scale = max_value / 255.;
    let shift = (max_value * 0.5).round();
    let m = BT601;
    let y = (scale * m[2]).mul_add(f32::from(px.b), (scale * m[0]).mul_add(f32::from(px.r), scale * m[1] * f32::from(px.g)));
    let cb = f32::from(px.b).mul_add(scale, -y).mul_add(0.5 / (1. - m[2]), shift);
    let cr = f32::from(px.r).mul_add(scale, -y).mul_add(0.5 / (1. - m[0]), shift);
    [y, cb, cr]
}

/// Tile count from the image size alone: one tile per 512×512 pixels of
/// area, at most 64. Independent of the thread count by construction.
fn size_tiles(width: usize, height: usize) -> usize {
    ((width * height) / (512 * 512)).clamp(1, 64)
}

/// `rav1e`'s own preset with the settings every still image needs.
fn preset_settings(speed: u8) -> SpeedSettings {
    let mut s = SpeedSettings::from_preset(speed);
    s.multiref = false;
    s.rdo_lookahead_frames = 1;
    s.scene_detection_mode = SceneDetectionSpeed::None;
    s.motion.include_near_mvs = false;
    s
}

#[allow(clippy::too_many_arguments)]
fn encode_direct<P: Pixel>(
    image: &Image,
    speed: u8,
    threads: usize,
    chroma: Chroma,
    depth: u8,
    quality: f32,
    preset: Option<bool>,
) -> Vec<u8> {
    let (w, h) = (image.width, image.height);
    let quantizer = quality_to_quantizer(quality);
    let (speed_settings, tiles) = match preset {
        None => {
            let (settings, min_tile) = speed_settings(speed, quantizer);
            (settings, threads.min((w * h) / (min_tile * min_tile)))
        }
        Some(by_size) => (preset_settings(speed), if by_size { size_tiles(w, h) } else { 1 }),
    };
    let config = Config::new()
        .with_encoder_config(EncoderConfig {
            width: w,
            height: h,
            bit_depth: usize::from(depth),
            chroma_sampling: if chroma == Chroma::Cs420 { ChromaSampling::Cs420 } else { ChromaSampling::Cs444 },
            chroma_sample_position: ChromaSamplePosition::Unknown,
            pixel_range: PixelRange::Full,
            color_description: Some(ColorDescription {
                transfer_characteristics: TransferCharacteristics::SRGB,
                color_primaries: ColorPrimaries::BT709,
                matrix_coefficients: MatrixCoefficients::BT601,
            }),
            still_picture: true,
            quantizer: usize::from(quantizer),
            min_quantizer: quantizer,
            bitrate: 0,
            tune: Tune::Psychovisual,
            tiles,
            speed_settings,
            ..EncoderConfig::default()
        })
        .with_threads(threads);
    let mut context: Context<P> = config.new_context().expect("rav1e config");
    let mut frame = context.new_frame();
    let max = f32::from(u16::try_from((1_u32 << depth) - 1).unwrap());
    let to_pixel = |v: f32| P::cast_from(v.round().clamp(0.0, max) as u16);
    {
        let mut planes = frame.planes.iter_mut();
        let mut y_plane = planes.next().unwrap().mut_slice(Default::default());
        for (row, out) in y_plane.rows_iter_mut().take(h).enumerate() {
            for (col, px) in out[..w].iter_mut().enumerate() {
                *px = to_pixel(ycbcr(image.rgb[row * w + col], depth)[0]);
            }
        }
        let (cw, ch, step) = match chroma {
            Chroma::Cs444 => (w, h, 1),
            Chroma::Cs420 => (w.div_ceil(2), h.div_ceil(2), 2),
        };
        for (component, plane) in [1_usize, 2].into_iter().zip(planes) {
            let mut slice = plane.mut_slice(Default::default());
            for (row, out) in slice.rows_iter_mut().take(ch).enumerate() {
                for (col, px) in out[..cw].iter_mut().enumerate() {
                    // Average the covered luma positions (center siting).
                    let (mut sum, mut count) = (0.0, 0.0);
                    for dy in 0..step {
                        for dx in 0..step {
                            let (y, x) = (row * step + dy, col * step + dx);
                            if y < h && x < w {
                                sum += ycbcr(image.rgb[y * w + x], depth)[component];
                                count += 1.0;
                            }
                        }
                    }
                    *px = to_pixel(sum / count);
                }
            }
        }
    }
    context.send_frame(frame).expect("send frame");
    context.flush();
    let mut av1 = Vec::new();
    loop {
        match context.receive_packet() {
            Ok(mut packet) => {
                if packet.frame_type == FrameType::KEY {
                    av1.append(&mut packet.data);
                }
            }
            Err(EncoderStatus::Encoded) => {}
            Err(EncoderStatus::LimitReached) => break,
            Err(error) => panic!("rav1e: {error:?}"),
        }
    }
    let mut container = avif_serialize::Aviffy::new();
    container
        .set_matrix_coefficients(avif_serialize::constants::MatrixCoefficients::Bt601)
        .set_full_color_range(true)
        .set_chroma_subsampling(if chroma == Chroma::Cs420 { (true, true) } else { (false, false) })
        .set_seq_profile(if chroma == Chroma::Cs420 { 0 } else { 1 });
    container.to_vec(&av1, None, u32::try_from(w).unwrap(), u32::try_from(h).unwrap(), depth)
}

fn encode(image: &Image, variant: Variant, quality: f32) -> (Vec<u8>, Duration) {
    let started = Instant::now();
    let bytes = match variant {
        Variant::Ravif { speed, threads } => {
            ravif::Encoder::new()
                .with_quality(quality)
                .with_speed(speed)
                .with_num_threads(Some(threads))
                .encode_rgb(Img::new(&image.rgb[..], image.width, image.height))
                .expect("ravif")
                .avif_file
        }
        Variant::Direct { speed, threads, chroma, depth } => {
            if depth > 8 {
                encode_direct::<u16>(image, speed, threads, chroma, depth, quality, None)
            } else {
                encode_direct::<u8>(image, speed, threads, chroma, depth, quality, None)
            }
        }
        Variant::Preset { speed, threads, chroma, size_tiles } => {
            encode_direct::<u16>(image, speed, threads, chroma, 10, quality, Some(size_tiles))
        }
    };
    (bytes, started.elapsed())
}

fn score(image: &Image, avif: &[u8]) -> f64 {
    use ssimulacra2::{ColorPrimaries as P, Rgb, TransferCharacteristic as T, compute_frame_ssimulacra2};
    let decoded: Vec<[f32; 3]> = match avif_decode::Decoder::from_avif(avif).expect("parse").to_image().expect("decode") {
        avif_decode::Image::Rgb8(img) => img.pixels().map(|p| [p.r, p.g, p.b].map(|c| f32::from(c) / 255.0)).collect(),
        avif_decode::Image::Rgb16(img) => img.pixels().map(|p| [p.r, p.g, p.b].map(|c| f32::from(c) / 65535.0)).collect(),
        _ => panic!("unexpected decoded layout"),
    };
    let source: Vec<[f32; 3]> = image.rgb.iter().map(|p| [p.r, p.g, p.b].map(|c| f32::from(c) / 255.0)).collect();
    let (w, h) = (image.width, image.height);
    compute_frame_ssimulacra2(
        Rgb::new(source, w, h, T::SRGB, P::BT709).unwrap(),
        Rgb::new(decoded, w, h, T::SRGB, P::BT709).unwrap(),
    )
    .expect("ssimulacra2")
}

fn median(mut times: Vec<Duration>) -> Duration {
    times.sort();
    times[times.len() / 2]
}

fn hash(bytes: &[u8]) -> u64 {
    // FNV-1a, enough to compare outputs of two builds.
    bytes.iter().fold(0xcbf2_9ce4_8422_2325, |h, &b| (h ^ u64::from(b)).wrapping_mul(0x100_0000_01b3))
}

struct Point {
    quality: f32,
    score: f64,
    bytes: usize,
}

struct Matched {
    bytes: f64,
    score_low: f64,
    score_high: f64,
    quality: f32,
    time: Duration,
    encodes: usize,
}

/// Bisect the quality knob until the score brackets `target`, then
/// interpolate bytes linearly in score between the bracketing points.
fn match_quality(image: &Image, variant: Variant, target: f64) -> Matched {
    let mut points: Vec<Point> = Vec::new();
    let measure = |quality: f32, points: &mut Vec<Point>| {
        let (bytes, _) = encode(image, variant, quality);
        let point = Point { quality, score: score(image, &bytes), bytes: bytes.len() };
        points.push(point);
    };
    let (mut low, mut high) = (40.0_f32, 98.0_f32);
    measure(low, &mut points);
    measure(high, &mut points);
    for _ in 0..7 {
        let middle = (low + high) / 2.0;
        measure(middle, &mut points);
        if points.last().unwrap().score < target {
            low = middle;
        } else {
            high = middle;
        }
        if (points.last().unwrap().score - target).abs() < 0.15 {
            break;
        }
    }
    let below = points.iter().filter(|p| p.score <= target).max_by(|a, b| a.score.total_cmp(&b.score));
    let above = points.iter().filter(|p| p.score >= target).min_by(|a, b| a.score.total_cmp(&b.score));
    let (bytes, score_low, score_high, quality) = match (below, above) {
        (Some(b), Some(a)) if (a.score - b.score).abs() > 1e-9 => {
            let t = (target - b.score) / (a.score - b.score);
            (b.bytes as f64 + t * (a.bytes as f64 - b.bytes as f64), b.score, a.score, if t < 0.5 { b.quality } else { a.quality })
        }
        (Some(p), _) | (_, Some(p)) => (p.bytes as f64, p.score, p.score, p.quality),
        _ => unreachable!(),
    };
    let encodes = points.len();
    let time = median((0..3).map(|_| encode(image, variant, quality).1).collect());
    Matched { bytes, score_low, score_high, quality, time, encodes }
}

fn variants() -> Vec<Variant> {
    use Chroma::{Cs420, Cs444};
    let mut list = vec![Variant::Ravif { speed: 6, threads: 1 }];
    for threads in [2, 4, 8, 20] {
        list.push(Variant::Ravif { speed: 6, threads });
    }
    for speed in [7, 8, 9, 10] {
        list.push(Variant::Ravif { speed, threads: 20 });
    }
    list.extend([
        Variant::Direct { speed: 6, threads: 20, chroma: Cs444, depth: 10 },
        Variant::Direct { speed: 6, threads: 20, chroma: Cs420, depth: 10 },
        Variant::Direct { speed: 6, threads: 20, chroma: Cs420, depth: 8 },
        Variant::Direct { speed: 8, threads: 20, chroma: Cs420, depth: 10 },
        Variant::Direct { speed: 6, threads: 1, chroma: Cs420, depth: 10 },
    ]);
    list
}

fn sink_variants() -> Vec<Variant> {
    use Chroma::{Cs420, Cs444};
    vec![
        Variant::Ravif { speed: 6, threads: 1 },
        Variant::Direct { speed: 6, threads: 1, chroma: Cs444, depth: 10 },
        Variant::Preset { speed: 6, threads: 1, chroma: Cs444, size_tiles: false },
        Variant::Preset { speed: 6, threads: 1, chroma: Cs420, size_tiles: false },
        Variant::Preset { speed: 6, threads: 1, chroma: Cs444, size_tiles: true },
        Variant::Preset { speed: 6, threads: 20, chroma: Cs444, size_tiles: true },
        Variant::Preset { speed: 6, threads: 20, chroma: Cs420, size_tiles: true },
        Variant::Preset { speed: 7, threads: 20, chroma: Cs420, size_tiles: true },
    ]
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let mode = args.first().map(String::as_str).unwrap_or_default();
    let images: Vec<Image> = args.iter().skip(1).map(|spec| load(spec)).collect();
    assert!(!images.is_empty(), "usage: (anchor|matrix) name=path[@width]...");
    let features = format!(
        "asm={} threading={}",
        cfg!(feature = "asm"),
        cfg!(feature = "threading")
    );
    let mut out = String::new();
    let anchor = Variant::Ravif { speed: 6, threads: 1 };
    match mode {
        "anchor" => {
            writeln!(out, "## Anchor ({features})\n").unwrap();
            writeln!(out, "| image | size | bytes | FNV-1a | SSIMULACRA2 | median encode ms (3 runs) |\n| --- | --- | ---: | --- | ---: | ---: |").unwrap();
            for image in &images {
                let (bytes, _) = encode(image, anchor, 80.0);
                let time = median((0..3).map(|_| encode(image, anchor, 80.0).1).collect());
                writeln!(
                    out,
                    "| {} | {}×{} | {} | {:016x} | {:.2} | {:.0} |",
                    image.name, image.width, image.height, bytes.len(), hash(&bytes), score(image, &bytes), time.as_secs_f64() * 1e3
                )
                .unwrap();
            }
        }
        "sink" => {
            writeln!(out, "## Sink candidates ({features})\n").unwrap();
            for image in &images {
                let (anchor_bytes, _) = encode(image, anchor, 80.0);
                let target = score(image, &anchor_bytes);
                let anchor_time = median((0..3).map(|_| encode(image, anchor, 80.0).1).collect());
                // Same quality, tiles from size: 1 and 20 threads must agree.
                let one = Variant::Preset { speed: 6, threads: 1, chroma: Chroma::Cs444, size_tiles: true };
                let many = Variant::Preset { speed: 6, threads: 20, chroma: Chroma::Cs444, size_tiles: true };
                let (a, _) = encode(image, one, 80.0);
                let (b, _) = encode(image, many, 80.0);
                writeln!(
                    out,
                    "### {} ({}×{}, {} size tiles); anchor {} bytes, SSIMULACRA2 {:.2}, {:.0} ms; size-tile output t1 {:016x} vs t20 {:016x}: {}\n",
                    image.name, image.width, image.height, size_tiles(image.width, image.height),
                    anchor_bytes.len(), target, anchor_time.as_secs_f64() * 1e3,
                    hash(&a), hash(&b), if a == b { "identical" } else { "DIFFERENT" }
                )
                .unwrap();
                writeln!(out, "| variant | quality used | score bracket | bytes at anchor score | bytes vs anchor | median encode ms | speedup vs anchor | encodes |\n| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |").unwrap();
                for variant in sink_variants() {
                    let m = match_quality(image, variant, target);
                    writeln!(
                        out,
                        "| {} | {:.1} | {:.2}–{:.2} | {:.0} | {:+.1}% | {:.0} | {:.1}× | {} |",
                        variant.label(), m.quality, m.score_low, m.score_high, m.bytes,
                        (m.bytes / anchor_bytes.len() as f64 - 1.0) * 100.0,
                        m.time.as_secs_f64() * 1e3,
                        anchor_time.as_secs_f64() / m.time.as_secs_f64(), m.encodes
                    )
                    .unwrap();
                    eprintln!("{} {} done", image.name, variant.label());
                }
                writeln!(out).unwrap();
            }
        }
        "matrix" => {
            writeln!(out, "## Quality-matched matrix ({features})\n").unwrap();
            for image in &images {
                let (anchor_bytes, _) = encode(image, anchor, 80.0);
                let target = score(image, &anchor_bytes);
                let anchor_time = median((0..3).map(|_| encode(image, anchor, 80.0).1).collect());
                writeln!(
                    out,
                    "### {} ({}×{}); anchor ravif s6 t1 q80: {} bytes, FNV-1a {:016x}, SSIMULACRA2 {:.2}, {:.0} ms\n",
                    image.name, image.width, image.height, anchor_bytes.len(), hash(&anchor_bytes), target, anchor_time.as_secs_f64() * 1e3
                )
                .unwrap();
                writeln!(out, "| variant | quality used | score bracket | bytes at anchor score | bytes vs anchor | median encode ms | speedup vs anchor | encodes |\n| --- | ---: | --- | ---: | ---: | ---: | ---: | ---: |").unwrap();
                for variant in variants() {
                    let m = match_quality(image, variant, target);
                    writeln!(
                        out,
                        "| {} | {:.1} | {:.2}–{:.2} | {:.0} | {:+.1}% | {:.0} | {:.1}× | {} |",
                        variant.label(),
                        m.quality,
                        m.score_low,
                        m.score_high,
                        m.bytes,
                        (m.bytes / anchor_bytes.len() as f64 - 1.0) * 100.0,
                        m.time.as_secs_f64() * 1e3,
                        anchor_time.as_secs_f64() / m.time.as_secs_f64(),
                        m.encodes
                    )
                    .unwrap();
                    eprintln!("{} {} done", image.name, variant.label());
                }
                writeln!(out).unwrap();
            }
        }
        _ => panic!("mode must be anchor, matrix or sink"),
    }
    print!("{out}");
}
