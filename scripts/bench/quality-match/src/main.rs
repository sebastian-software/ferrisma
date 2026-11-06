//! Common-reference SSIMULACRA2 scoring and quality bisection (M2 work
//! package 0). Used by `scripts/bench/compare-sharp.py`; never shipped.
//!
//! The reference is computed here, independently of every tool under test:
//! the decoded, oriented source is resized with a separable Lanczos3 filter
//! (support 3 × scale, pixel-center aligned, weights normalized) in linear
//! light, using the exact sRGB transfer function in `f64`, and kept as
//! unquantized `f32` sRGB values. Every candidate (AVIF or PNG) is decoded
//! and scored against that one reference, so tools that resize differently
//! pay for it in score; their score ceiling is measured separately.
//!
//! Commands:
//!   ref SOURCE[#ORIENT] WIDTH HEIGHT OUT.ref   build a reference
//!   topng SOURCE[#ORIENT] OUT.png              decode a source to 8-bit PNG
//!   score REF FILE...                          print bytes and score per file
//!   bisect REF TARGET LO HI STEP DIR -- CMD... find the setting that reaches TARGET
//!
//! SOURCE is `.png` (8 or 16 bit), `.jpg` (decoded with zune-jpeg, ICC and
//! EXIF ignored, so pass the EXIF orientation as `#6` and so on) or an 8-bit
//! 4:2:0 full-range `.y4m`. In `bisect`, CMD is run once per setting with
//! `{q}` replaced by the setting and `{out}` by the output path; STEP is the
//! knob's granularity (1 for integer knobs, 0 for continuous ones).

use std::fmt::Write as _;
use std::io::{BufReader, BufWriter, Read as _, Write as _};
use std::path::Path;
use std::process::Command;

/// Interleaved RGB, sRGB-encoded, nominally in [0, 1].
#[derive(Clone)]
struct Frame {
    width: usize,
    height: usize,
    data: Vec<f32>,
}

fn from_u8(width: usize, height: usize, rgb: &[u8]) -> Frame {
    Frame {
        width,
        height,
        data: rgb.iter().map(|&c| f32::from(c) / 255.0).collect(),
    }
}

fn load_png(path: &str) -> Frame {
    let file = std::fs::File::open(path).expect("open png");
    let mut decoder = png::Decoder::new(BufReader::new(file));
    decoder.set_transformations(png::Transformations::EXPAND);
    let mut reader = decoder.read_info().expect("png header");
    let mut buf = vec![0; reader.output_buffer_size().expect("png size")];
    let info = reader.next_frame(&mut buf).expect("png frame");
    let (width, height) = (info.width as usize, info.height as usize);
    let channels = match info.color_type {
        png::ColorType::Rgb => 3,
        png::ColorType::Rgba => 4,
        other => panic!("unsupported png color type {other:?}"),
    };
    let wide = info.bit_depth == png::BitDepth::Sixteen;
    let sample = |i: usize| {
        if wide {
            f32::from(u16::from_be_bytes([buf[2 * i], buf[2 * i + 1]])) / 65535.0
        } else {
            f32::from(buf[i]) / 255.0
        }
    };
    let mut data = Vec::with_capacity(width * height * 3);
    for pixel in 0..width * height {
        if channels == 4 {
            let alpha = sample(pixel * 4 + 3);
            assert!(alpha > 0.999, "{path}: alpha is not supported");
        }
        for c in 0..3 {
            data.push(sample(pixel * channels + c));
        }
    }
    Frame {
        width,
        height,
        data,
    }
}

fn load_jpeg(path: &str) -> Frame {
    use zune_jpeg::zune_core::{
        bytestream::ZCursor, colorspace::ColorSpace, options::DecoderOptions,
    };
    let bytes = std::fs::read(path).expect("read jpeg");
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = zune_jpeg::JpegDecoder::new_with_options(ZCursor::new(&bytes[..]), options);
    let pixels = decoder.decode().expect("decode jpeg");
    let (width, height) = decoder.dimensions().expect("dimensions");
    from_u8(width, height, &pixels)
}

/// 8-bit 4:2:0, full range, BT.601 (the Kodak files in libavif's test data).
fn load_y4m(path: &str) -> Frame {
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
    let frame = header_end
        + 1
        + bytes[header_end + 1..]
            .iter()
            .position(|&b| b == b'\n')
            .unwrap()
        + 1;
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
    from_u8(width, height, &rgb)
}

/// EXIF orientations 1, 3, 6 and 8 (the rotations); mirrored ones panic.
fn orient(frame: Frame, orientation: u8) -> Frame {
    let (w, h) = (frame.width, frame.height);
    let (nw, nh) = match orientation {
        1 => return frame,
        3 => (w, h),
        6 | 8 => (h, w),
        other => panic!("orientation {other} is not supported"),
    };
    let mut data = vec![0.0; frame.data.len()];
    for y in 0..nh {
        for x in 0..nw {
            let (sx, sy) = match orientation {
                3 => (w - 1 - x, h - 1 - y),
                6 => (y, h - 1 - x),
                _ => (w - 1 - y, x),
            };
            let (from, to) = ((sy * w + sx) * 3, (y * nw + x) * 3);
            data[to..to + 3].copy_from_slice(&frame.data[from..from + 3]);
        }
    }
    Frame {
        width: nw,
        height: nh,
        data,
    }
}

fn load_source(spec: &str) -> Frame {
    let (path, orientation) = match spec.rsplit_once('#') {
        Some((path, o)) => (path, o.parse::<u8>().expect("orientation")),
        None => (spec, 1),
    };
    let frame = if path.ends_with(".png") {
        load_png(path)
    } else if path.ends_with(".y4m") {
        load_y4m(path)
    } else {
        load_jpeg(path)
    };
    orient(frame, orientation)
}

fn srgb_to_linear(v: f64) -> f64 {
    if v <= 0.040_45 {
        v / 12.92
    } else {
        ((v + 0.055) / 1.055).powf(2.4)
    }
}

fn linear_to_srgb(v: f64) -> f64 {
    let v = v.clamp(0.0, 1.0);
    if v <= 0.003_130_8 {
        v * 12.92
    } else {
        1.055 * v.powf(1.0 / 2.4) - 0.055
    }
}

fn sinc(x: f64) -> f64 {
    if x.abs() < 1e-12 {
        1.0
    } else {
        let p = x * std::f64::consts::PI;
        p.sin() / p
    }
}

/// Normalized Lanczos3 taps for every output position along one axis.
fn lanczos_taps(input: usize, output: usize) -> Vec<(usize, Vec<f64>)> {
    let ratio = input as f64 / output as f64;
    let scale = ratio.max(1.0);
    let support = 3.0 * scale;
    (0..output)
        .map(|at| {
            let center = (at as f64 + 0.5) * ratio - 0.5;
            let first = (center - support).floor().max(0.0) as usize;
            let last = ((center + support).ceil() as usize).min(input - 1);
            let mut weights: Vec<f64> = (first..=last)
                .map(|i| {
                    let d = (i as f64 - center) / scale;
                    if d.abs() < 3.0 {
                        sinc(d) * sinc(d / 3.0)
                    } else {
                        0.0
                    }
                })
                .collect();
            let sum: f64 = weights.iter().sum();
            weights.iter_mut().for_each(|w| *w /= sum);
            (first, weights)
        })
        .collect()
}

fn reference(source: &Frame, width: usize, height: usize) -> Frame {
    let (sw, sh) = (source.width, source.height);
    let linear: Vec<f64> = source
        .data
        .iter()
        .map(|&v| srgb_to_linear(f64::from(v)))
        .collect();
    let columns = lanczos_taps(sw, width);
    let mut horizontal = vec![0.0_f64; width * sh * 3];
    for y in 0..sh {
        let row = &linear[y * sw * 3..][..sw * 3];
        for (x, (first, weights)) in columns.iter().enumerate() {
            let mut acc = [0.0; 3];
            for (k, w) in weights.iter().enumerate() {
                for c in 0..3 {
                    acc[c] += w * row[(first + k) * 3 + c];
                }
            }
            horizontal[(y * width + x) * 3..][..3].copy_from_slice(&acc);
        }
    }
    let rows = lanczos_taps(sh, height);
    let mut data = vec![0.0_f32; width * height * 3];
    for (y, (first, weights)) in rows.iter().enumerate() {
        let mut acc = vec![0.0_f64; width * 3];
        for (k, w) in weights.iter().enumerate() {
            let src = &horizontal[(first + k) * width * 3..][..width * 3];
            acc.iter_mut().zip(src).for_each(|(a, s)| *a += w * s);
        }
        for (out, a) in data[y * width * 3..][..width * 3].iter_mut().zip(acc) {
            *out = linear_to_srgb(a) as f32;
        }
    }
    Frame {
        width,
        height,
        data,
    }
}

const MAGIC: &[u8; 8] = b"QMREF1\0\0";

fn write_ref(frame: &Frame, path: &str) {
    let mut out = BufWriter::new(std::fs::File::create(path).expect("create ref"));
    out.write_all(MAGIC).unwrap();
    out.write_all(&u32::try_from(frame.width).unwrap().to_le_bytes())
        .unwrap();
    out.write_all(&u32::try_from(frame.height).unwrap().to_le_bytes())
        .unwrap();
    for v in &frame.data {
        out.write_all(&v.to_le_bytes()).unwrap();
    }
}

fn read_ref(path: &str) -> Frame {
    let mut bytes = Vec::new();
    std::fs::File::open(path)
        .expect("open ref")
        .read_to_end(&mut bytes)
        .unwrap();
    assert_eq!(&bytes[..8], MAGIC, "{path} is not a reference file");
    let width = u32::from_le_bytes(bytes[8..12].try_into().unwrap()) as usize;
    let height = u32::from_le_bytes(bytes[12..16].try_into().unwrap()) as usize;
    let (words, rest) = bytes[16..].as_chunks::<4>();
    assert!(rest.is_empty(), "{path}: truncated reference");
    let data = words.iter().map(|&b| f32::from_le_bytes(b)).collect();
    Frame {
        width,
        height,
        data,
    }
}

fn decode_avif(path: &str) -> Frame {
    let bytes = std::fs::read(path).expect("read avif");
    let image = avif_decode::Decoder::from_avif(&bytes)
        .expect("parse avif")
        .to_image()
        .expect("decode avif");
    let (width, height, data): (usize, usize, Vec<f32>) = match image {
        avif_decode::Image::Rgb8(img) => (
            img.width(),
            img.height(),
            img.pixels()
                .flat_map(|p| [p.r, p.g, p.b].map(|c| f32::from(c) / 255.0))
                .collect(),
        ),
        avif_decode::Image::Rgb16(img) => (
            img.width(),
            img.height(),
            img.pixels()
                .flat_map(|p| [p.r, p.g, p.b].map(|c| f32::from(c) / 65535.0))
                .collect(),
        ),
        _ => panic!("{path}: unexpected decoded layout (alpha or gray)"),
    };
    Frame {
        width,
        height,
        data,
    }
}

fn load_candidate(path: &str) -> Frame {
    if path.ends_with(".avif") {
        decode_avif(path)
    } else if path.ends_with(".ref") {
        read_ref(path)
    } else {
        load_source(path)
    }
}

fn score(reference: &Frame, candidate: &Frame) -> f64 {
    use ssimulacra2::{
        ColorPrimaries as P, Rgb, TransferCharacteristic as T, compute_frame_ssimulacra2,
    };
    assert_eq!(
        (reference.width, reference.height),
        (candidate.width, candidate.height),
        "candidate geometry differs from the reference"
    );
    let pixels = |f: &Frame| {
        f.data
            .as_chunks::<3>()
            .0
            .iter()
            .map(|p| p.map(|c| c.clamp(0.0, 1.0)))
            .collect::<Vec<_>>()
    };
    compute_frame_ssimulacra2(
        Rgb::new(
            pixels(reference),
            reference.width,
            reference.height,
            T::SRGB,
            P::BT709,
        )
        .unwrap(),
        Rgb::new(
            pixels(candidate),
            candidate.width,
            candidate.height,
            T::SRGB,
            P::BT709,
        )
        .unwrap(),
    )
    .expect("ssimulacra2")
}

#[derive(Clone, Copy)]
struct Point {
    quality: f64,
    score: f64,
    bytes: u64,
}

fn format_quality(quality: f64, step: f64) -> String {
    if step >= 1.0 {
        format!("{quality:.0}")
    } else {
        format!("{quality:.2}")
    }
}

fn run_setting(reference: &Frame, quality: f64, step: f64, dir: &str, command: &[String]) -> Point {
    let q = format_quality(quality, step);
    let out = format!("{dir}/q{q}.avif");
    let args: Vec<String> = command
        .iter()
        .map(|a| a.replace("{q}", &q).replace("{out}", &out))
        .collect();
    let status = Command::new(&args[0])
        .args(&args[1..])
        .stdout(std::process::Stdio::null())
        .status()
        .expect("spawn");
    assert!(status.success(), "command failed: {args:?}");
    let bytes = std::fs::metadata(&out).expect("output").len();
    let point = Point {
        quality,
        score: score(reference, &decode_avif(&out)),
        bytes,
    };
    eprintln!("  q={q} score={:.3} bytes={bytes}", point.score);
    point
}

/// Bisect the knob in [lo, hi] for the lowest setting whose score reaches
/// `target`, assuming the score grows with the knob; bytes are interpolated
/// linearly in score between the closest points below and above the target.
fn bisect(
    reference: &Frame,
    target: f64,
    mut lo: f64,
    mut hi: f64,
    step: f64,
    dir: &str,
    command: &[String],
) -> String {
    let mut points = Vec::new();
    let top = run_setting(reference, hi, step, dir, command);
    points.push(top);
    if top.score >= target {
        let bottom = run_setting(reference, lo, step, dir, command);
        points.push(bottom);
        if bottom.score < target {
            for _ in 0..14 {
                let mid = if step >= 1.0 {
                    if hi - lo <= step {
                        break;
                    }
                    lo + (((hi - lo) / step / 2.0).floor().max(1.0)) * step
                } else {
                    if hi - lo < 0.02 {
                        break;
                    }
                    (lo + hi) / 2.0
                };
                let p = run_setting(reference, mid, step, dir, command);
                points.push(p);
                if p.score >= target {
                    hi = mid;
                } else {
                    lo = mid;
                }
                if step < 1.0 && (p.score - target).abs() < 0.05 {
                    break;
                }
            }
        }
    }
    let below = points
        .iter()
        .filter(|p| p.score < target)
        .max_by(|a, b| a.score.total_cmp(&b.score))
        .copied();
    let above = points
        .iter()
        .filter(|p| p.score >= target)
        .min_by(|a, b| a.score.total_cmp(&b.score))
        .copied();
    let (status, bytes, nearest) = match (below, above) {
        (Some(b), Some(a)) => {
            let t = (target - b.score) / (a.score - b.score);
            let nearest = if t < 0.5 { b } else { a };
            (
                "ok",
                b.bytes as f64 + t * (a.bytes as f64 - b.bytes as f64),
                nearest,
            )
        }
        (None, Some(a)) => ("below-range", a.bytes as f64, a),
        (Some(b), None) => ("unreachable", b.bytes as f64, b),
        (None, None) => unreachable!(),
    };
    let fmt = |p: Option<Point>| match p {
        Some(p) => format!(
            "{}\t{:.3}\t{}",
            format_quality(p.quality, step),
            p.score,
            p.bytes
        ),
        None => "-\t-\t-".to_owned(),
    };
    format!(
        "{status}\t{target:.3}\t{}\t{}\t{bytes:.0}\t{}\t{:.3}\t{}",
        fmt(below),
        fmt(above),
        format_quality(nearest.quality, step),
        nearest.score,
        points.len()
    )
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let usage = "usage: quality-match (ref SOURCE W H OUT.ref | topng SOURCE OUT.png | score REF FILE... | bisect REF TARGET LO HI STEP DIR -- CMD...)";
    match args.first().map(String::as_str) {
        Some("ref") if args.len() == 5 => {
            let source = load_source(&args[1]);
            let (w, h) = (
                args[2].parse().expect("width"),
                args[3].parse().expect("height"),
            );
            let frame = reference(&source, w, h);
            write_ref(&frame, &args[4]);
            println!(
                "{} {}x{} -> {}x{}",
                args[4], source.width, source.height, w, h
            );
        }
        Some("topng") if args.len() == 3 => {
            let frame = load_source(&args[1]);
            let file = std::fs::File::create(&args[2]).expect("create png");
            let mut encoder = png::Encoder::new(
                BufWriter::new(file),
                u32::try_from(frame.width).unwrap(),
                u32::try_from(frame.height).unwrap(),
            );
            encoder.set_color(png::ColorType::Rgb);
            encoder.set_depth(png::BitDepth::Eight);
            let mut writer = encoder.write_header().expect("png header");
            let bytes: Vec<u8> = frame
                .data
                .iter()
                .map(|&v| (v * 255.0).round().clamp(0.0, 255.0) as u8)
                .collect();
            writer.write_image_data(&bytes).expect("png data");
        }
        Some("score") if args.len() >= 3 => {
            let reference = load_candidate(&args[1]);
            let mut out = String::new();
            for path in &args[2..] {
                let bytes = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);
                let value = score(&reference, &load_candidate(path));
                writeln!(out, "{}\t{bytes}\t{value:.3}", Path::new(path).display()).unwrap();
            }
            print!("{out}");
        }
        Some("bisect") if args.len() >= 9 && args[7] == "--" => {
            let reference = read_ref(&args[1]);
            let num = |i: usize| args[i].parse::<f64>().expect("number");
            std::fs::create_dir_all(&args[6]).expect("create dir");
            println!(
                "{}",
                bisect(
                    &reference,
                    num(2),
                    num(3),
                    num(4),
                    num(5),
                    &args[6],
                    &args[8..]
                )
            );
        }
        _ => {
            eprintln!("{usage}");
            std::process::exit(2);
        }
    }
}
