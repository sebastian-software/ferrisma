//! Shared support of the WP13 tests: the records of the encode probe
//! (`tools/oracle/probes/wp13/encode.c`), the seeded pictures they carry, and
//! the replay of a record with the Rust translation (`webp_encode`), which
//! prints what the probe prints.
//!
//! Include with `#[path = "support/wp13_cases.rs"] mod wp13_cases;` next to
//! `support/oracle.rs`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::cell::RefCell;
use std::fmt::Write as _;

use super::oracle::Fields;
use webp::enc::config_enc::{webp_config_lossless_preset, webp_validate_config};
use webp::enc::picture_csp_enc::{webp_picture_import_rgb, webp_picture_import_rgba};
use webp::enc::picture_enc::webp_memory_write;
use webp::enc::picture_rescale_enc::webp_picture_view;
use webp::enc::webp_enc::webp_encode;
use webp::webp::encode::{
    WebPAuxStats, WebPConfig, WebPImageHint, WebPMemoryWriter, WebPPicture, WebPPreset,
    webp_config_init, webp_config_preset, webp_picture_init,
};
use webp::webp::types::{PlaneOffset, PlaneStorage};

/// A configuration field the case leaves at the preset's value.
pub const KEEP: i32 = i32::MIN;

/// The configuration fields of a record, in the probe's order.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(usize)]
pub enum F {
    Method,
    Segments,
    Partitions,
    Sns,
    FilterStrength,
    FilterSharpness,
    FilterType,
    Autofilter,
    Pass,
    TargetSize,
    TargetPsnr,
    EmulateJpegSize,
    LowMemory,
    ThreadLevel,
    UseSharpYuv,
    Preprocessing,
    AlphaQuality,
    AlphaFiltering,
    AlphaCompression,
    Exact,
    PartitionLimit,
    Qmin,
    Qmax,
    ShowCompressed,
    Lossless,
    NearLossless,
    UseDeltaPalette,
    ImageHint,
}

pub const NUM_FIELDS: usize = 28;

const FIELD_NAMES: [&str; NUM_FIELDS] = [
    "method", "segments", "parts", "sns", "fstr", "sharp", "ftype", "auto", "pass", "tsize",
    "tpsnr", "jpeg", "lowmem", "threads", "sharpyuv", "pre", "aq", "afilt", "acomp", "exact",
    "plimit", "qmin", "qmax", "show", "lossless", "near", "delta", "hint",
];

/// How the probe builds the picture it encodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Layout {
    /// The imported picture, which owns its planes.
    Library,
    /// The planes copied into caller memory with rows `pad` samples longer.
    Caller { pad: i32 },
    /// A view of a rectangle of the imported picture.
    View {
        left: i32,
        top: i32,
        width: i32,
        height: i32,
    },
}

/// One encoding: a picture, the configuration, and the failure injections.
#[derive(Clone, Debug)]
pub struct Case {
    pub width: i32,
    pub height: i32,
    /// 0: RGB into YUV, 1: RGBA into YUV(A), 2: RGBA into ARGB, 3: RGB into ARGB.
    pub source: i32,
    pub layout: Layout,
    /// `None` starts from `WebPConfigInit`.
    pub preset: Option<u32>,
    pub quality: f32,
    /// `WebPConfigLosslessPreset` level, applied after the preset.
    pub lossless_level: Option<i32>,
    pub fields: [i32; NUM_FIELDS],
    /// Below -1: no progress hook; -1: never abort; else the failing call.
    pub abort_at: i32,
    pub fail_write_at: i32,
    pub with_stats: bool,
    pub extra_info_type: i32,
    /// Encode into a `WebPMemoryWriter` instead of the custom writer.
    pub memory_writer: bool,
    /// RGBA words, R in the low byte.
    pub rgba: Vec<u32>,
}

impl Case {
    pub fn new(width: i32, height: i32, rgba: Vec<u32>) -> Case {
        assert_eq!(rgba.len(), (width * height) as usize);
        Case {
            width,
            height,
            source: 1,
            layout: Layout::Library,
            preset: None,
            quality: 75.0,
            lossless_level: None,
            fields: [KEEP; NUM_FIELDS],
            abort_at: -1,
            fail_write_at: -1,
            with_stats: true,
            extra_info_type: 0,
            memory_writer: false,
            rgba,
        }
    }

    pub fn set(&mut self, field: F, value: i32) -> &mut Self {
        self.fields[field as usize] = value;
        self
    }

    pub fn with(mut self, field: F, value: i32) -> Self {
        self.fields[field as usize] = value;
        self
    }

    pub fn get(&self, field: F) -> i32 {
        self.fields[field as usize]
    }

    /// The probe's binary record.
    pub fn record(&self, out: &mut Vec<u8>) {
        let (layout, left, top, view_width, view_height, pad) = match self.layout {
            Layout::Library => (0, 0, 0, 0, 0, 0),
            Layout::Caller { pad } => (1, 0, 0, 0, 0, pad),
            Layout::View {
                left,
                top,
                width,
                height,
            } => (2, left, top, width, height, 0),
        };
        let mut values = vec![
            self.width,
            self.height,
            self.source,
            layout,
            left,
            top,
            view_width,
            view_height,
            pad,
            self.preset.map_or(-1, |preset| preset as i32),
            self.quality.to_bits() as i32,
            self.lossless_level.unwrap_or(-1),
        ];
        values.extend_from_slice(&self.fields);
        values.extend_from_slice(&[
            self.abort_at,
            self.fail_write_at,
            i32::from(self.with_stats),
            self.extra_info_type,
            i32::from(self.memory_writer),
        ]);
        for value in values {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for &pixel in &self.rgba {
            out.extend_from_slice(&pixel.to_le_bytes());
        }
    }

    /// The configuration the probe builds.
    pub fn config(&self) -> WebPConfig {
        let mut config = WebPConfig::default();
        match self.preset {
            None => assert!(webp_config_init(&mut config)),
            Some(preset) => {
                // An invalid quality fails the preset's validation; the probe
                // keeps the fields.
                let _ = webp_config_preset(&mut config, WebPPreset(preset), self.quality);
            }
        }
        config.quality = self.quality;
        if let Some(level) = self.lossless_level {
            assert!(webp_config_lossless_preset(&mut config, level));
        }
        let set = |field: &mut i32, value: i32| {
            if value != KEEP {
                *field = value;
            }
        };
        set(&mut config.method, self.get(F::Method));
        set(&mut config.segments, self.get(F::Segments));
        set(&mut config.partitions, self.get(F::Partitions));
        set(&mut config.sns_strength, self.get(F::Sns));
        set(&mut config.filter_strength, self.get(F::FilterStrength));
        set(&mut config.filter_sharpness, self.get(F::FilterSharpness));
        set(&mut config.filter_type, self.get(F::FilterType));
        set(&mut config.autofilter, self.get(F::Autofilter));
        set(&mut config.pass, self.get(F::Pass));
        set(&mut config.target_size, self.get(F::TargetSize));
        if self.get(F::TargetPsnr) != KEEP {
            config.target_psnr = f32::from_bits(self.get(F::TargetPsnr) as u32);
        }
        set(&mut config.emulate_jpeg_size, self.get(F::EmulateJpegSize));
        set(&mut config.low_memory, self.get(F::LowMemory));
        set(&mut config.thread_level, self.get(F::ThreadLevel));
        set(&mut config.use_sharp_yuv, self.get(F::UseSharpYuv));
        set(&mut config.preprocessing, self.get(F::Preprocessing));
        set(&mut config.alpha_quality, self.get(F::AlphaQuality));
        set(&mut config.alpha_filtering, self.get(F::AlphaFiltering));
        set(&mut config.alpha_compression, self.get(F::AlphaCompression));
        set(&mut config.exact, self.get(F::Exact));
        set(&mut config.partition_limit, self.get(F::PartitionLimit));
        set(&mut config.qmin, self.get(F::Qmin));
        set(&mut config.qmax, self.get(F::Qmax));
        set(&mut config.show_compressed, self.get(F::ShowCompressed));
        set(&mut config.lossless, self.get(F::Lossless));
        set(&mut config.near_lossless, self.get(F::NearLossless));
        set(&mut config.use_delta_palette, self.get(F::UseDeltaPalette));
        if self.get(F::ImageHint) != KEEP {
            config.image_hint = WebPImageHint(self.get(F::ImageHint) as u32);
        }
        config
    }

    pub fn describe(&self) -> String {
        let mut text = format!(
            "{}x{} source {} {:?} preset {:?} q {} level {:?} abort {} fail {} extra {} mem {}",
            self.width,
            self.height,
            self.source,
            self.layout,
            self.preset,
            self.quality,
            self.lossless_level,
            self.abort_at,
            self.fail_write_at,
            self.extra_info_type,
            self.memory_writer
        );
        for (name, &value) in FIELD_NAMES.iter().zip(&self.fields) {
            if value != KEEP {
                write!(text, " {name} {value}").unwrap();
            }
        }
        text
    }
}

/// The probe input for `cases`.
pub fn records(cases: &[Case]) -> Vec<u8> {
    let mut input = Vec::new();
    for case in cases {
        case.record(&mut input);
    }
    input
}

/// The probe's output split into one `Fields` per record; a record starts
/// with `valid`.
pub fn split_records(all: Fields) -> Vec<Fields> {
    let mut out: Vec<Fields> = Vec::new();
    for (key, value) in all.0 {
        if key == "valid" {
            out.push(Fields::new());
        }
        out.last_mut()
            .expect("the output starts with valid=")
            .0
            .push((key, value));
    }
    out
}

//------------------------------------------------------------------------------
// Seeded pictures

/// SplitMix64, a small deterministic generator for the cases.
pub struct Rng(pub u64);

impl Rng {
    pub fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    /// A value in `0..bound`.
    pub fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }

    pub fn u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn pick<T: Copy>(&mut self, values: &[T]) -> T {
        values[self.below(values.len() as u64) as usize]
    }
}

/// The alpha patterns of the generated pictures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Alpha {
    Opaque,
    /// 0 or 255 in blocks.
    Binary,
    Gradient,
    Noise,
}

pub const ALPHAS: [Alpha; 4] = [Alpha::Opaque, Alpha::Binary, Alpha::Gradient, Alpha::Noise];

/// The color content of the generated pictures.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Content {
    /// Smooth gradients with a little noise.
    Photo,
    /// Random samples.
    Noise,
    /// Runs of a few colors.
    Palette,
    /// One color with rectangles.
    Flat,
}

pub const CONTENTS: [Content; 4] = [
    Content::Photo,
    Content::Noise,
    Content::Palette,
    Content::Flat,
];

/// A seeded RGBA picture (R in the low byte of each word).
pub fn picture(rng: &mut Rng, content: Content, alpha: Alpha, width: i32, height: i32) -> Vec<u32> {
    let (w, h) = (width as u64, height as u64);
    let (r0, g0, b0) = (rng.below(256), rng.below(256), rng.below(256));
    let (dx, dy) = (1 + rng.below(7), 1 + rng.below(7));
    let palette: Vec<u64> = (0..1 + rng.below(6)).map(|_| rng.next_u64()).collect();
    let background = rng.next_u64();
    let rects: Vec<(u64, u64, u64, u64, u64)> = (0..rng.below(4))
        .map(|_| {
            let x0 = rng.below(w);
            let y0 = rng.below(h);
            (
                x0,
                y0,
                1 + rng.below(w - x0),
                1 + rng.below(h - y0),
                rng.next_u64(),
            )
        })
        .collect();
    let mut run = 0;
    let mut color = palette[0];
    let mut out = Vec::with_capacity((w * h) as usize);
    for y in 0..h {
        for x in 0..w {
            let rgb = match content {
                Content::Photo => {
                    let r = (r0 + x * dx + rng.below(4)) & 0xff;
                    let g = (g0 + y * dy + rng.below(4)) & 0xff;
                    let b = (b0 + (x + y) * 2 + rng.below(4)) & 0xff;
                    r | g << 8 | b << 16
                }
                Content::Noise => rng.next_u64() & 0xff_ffff,
                Content::Palette => {
                    if run == 0 {
                        color = palette[rng.below(palette.len() as u64) as usize];
                        run = 1 + rng.below(9);
                    }
                    run -= 1;
                    color & 0xff_ffff
                }
                Content::Flat => {
                    let mut value = background;
                    for &(rx, ry, rw, rh, c) in &rects {
                        if x >= rx && x < rx + rw && y >= ry && y < ry + rh {
                            value = c;
                        }
                    }
                    value & 0xff_ffff
                }
            };
            let a = match alpha {
                Alpha::Opaque => 0xff,
                Alpha::Binary => {
                    if ((x / 8) + (y / 8)) % 3 == 0 {
                        0
                    } else {
                        0xff
                    }
                }
                Alpha::Gradient => {
                    ((x * 255) / w.max(2).saturating_sub(1).max(1)).min(255) ^ (y & 3)
                }
                Alpha::Noise => rng.below(256),
            };
            out.push((rgb | a << 24) as u32);
        }
    }
    out
}

/// The RGBA words of a PAM (`P7`, `RGB_ALPHA` or `RGB`) or PPM (`P6`) file.
pub fn read_pnm(bytes: &[u8]) -> (i32, i32, Vec<u32>) {
    let mut pos = 0;
    let mut token = || -> String {
        loop {
            while bytes[pos].is_ascii_whitespace() {
                pos += 1;
            }
            if bytes[pos] == b'#' {
                while bytes[pos] != b'\n' {
                    pos += 1;
                }
                continue;
            }
            break;
        }
        let start = pos;
        while !bytes[pos].is_ascii_whitespace() {
            pos += 1;
        }
        String::from_utf8(bytes[start..pos].to_vec()).unwrap()
    };
    let magic = token();
    let (width, height, depth);
    if magic == "P7" {
        let mut fields = std::collections::HashMap::new();
        loop {
            let key = token();
            if key == "ENDHDR" {
                break;
            }
            let value = token();
            fields.insert(key, value);
        }
        width = fields["WIDTH"].parse::<i32>().unwrap();
        height = fields["HEIGHT"].parse::<i32>().unwrap();
        depth = fields["DEPTH"].parse::<usize>().unwrap();
    } else {
        assert_eq!(magic, "P6", "PAM or PPM");
        width = token().parse::<i32>().unwrap();
        height = token().parse::<i32>().unwrap();
        let _max = token();
        depth = 3;
    }
    pos += 1; // the single whitespace before the samples
    let samples = &bytes[pos..];
    let rgba = samples
        .chunks(depth)
        .take((width * height) as usize)
        .map(|p| {
            let a = if depth == 4 { u32::from(p[3]) } else { 0xff };
            u32::from(p[0]) | u32::from(p[1]) << 8 | u32::from(p[2]) << 16 | a << 24
        })
        .collect();
    (width, height, rgba)
}

//------------------------------------------------------------------------------
// The Rust replay

/// FNV-1a over `data`, from `hash`.
pub fn fnv(mut hash: u64, data: &[u8]) -> u64 {
    for &byte in data {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

pub const FNV_START: u64 = 0xcbf2_9ce4_8422_2325;

/// The digest of the rows of a plane, as the probe's `PrintPlane`.
fn plane_digest(
    storage: &PlaneStorage<'_, u8>,
    plane: Option<PlaneOffset>,
    stride: i32,
    width: i32,
    height: i32,
) -> String {
    let Some(plane) = plane else {
        return "-".to_owned();
    };
    let region = storage.region(plane.region).expect("the plane's region");
    let mut hash = FNV_START;
    for y in 0..height as i64 {
        let start = (plane.offset as i64 + y * i64::from(stride)) as usize;
        hash = fnv(hash, &region[start..start + width as usize]);
    }
    format!("{hash:016x}")
}

fn argb_digest(
    storage: &PlaneStorage<'_, u32>,
    plane: Option<PlaneOffset>,
    stride: i32,
    width: i32,
    height: i32,
) -> String {
    let Some(plane) = plane else {
        return "-".to_owned();
    };
    let region = storage.region(plane.region).expect("the plane's region");
    let mut hash = FNV_START;
    for y in 0..height as i64 {
        let start = (plane.offset as i64 + y * i64::from(stride)) as usize;
        for &v in &region[start..start + width as usize] {
            hash = fnv(hash, &v.to_le_bytes());
        }
    }
    format!("{hash:016x}")
}

/// What the probe's `PrintPicture` prints for `pic`.
pub fn picture_line(pic: &WebPPicture<'_>) -> String {
    let (uv_w, uv_h) = ((pic.width + 1) >> 1, (pic.height + 1) >> 1);
    format!(
        "{},{},{},{},{},{},{},{},{}",
        pic.use_argb,
        pic.colorspace.0,
        pic.width,
        pic.height,
        plane_digest(&pic.yuva, pic.y, pic.y_stride, pic.width, pic.height),
        plane_digest(&pic.yuva, pic.u, pic.uv_stride, uv_w, uv_h),
        plane_digest(&pic.yuva, pic.v, pic.uv_stride, uv_w, uv_h),
        plane_digest(&pic.yuva, pic.a, pic.a_stride, pic.width, pic.height),
        argb_digest(
            &pic.argb_storage,
            pic.argb,
            pic.argb_stride,
            pic.width,
            pic.height
        ),
    )
}

/// `WebPAuxStats` filled with the probe's 0xa5 bytes.
pub fn poisoned_stats() -> WebPAuxStats {
    let int = 0xa5a5_a5a5_u32 as i32;
    let float = f32::from_bits(0xa5a5_a5a5);
    WebPAuxStats {
        coded_size: int,
        psnr: [float; 5],
        block_count: [int; 3],
        header_bytes: [int; 2],
        residual_bytes: [[int; 4]; 3],
        segment_size: [int; 4],
        segment_quant: [int; 4],
        segment_level: [int; 4],
        alpha_data_size: int,
        layer_data_size: int,
        lossless_features: 0xa5a5_a5a5,
        histogram_bits: int,
        transform_bits: int,
        cache_bits: int,
        palette_size: int,
        lossless_size: int,
        lossless_hdr_size: int,
        lossless_data_size: int,
        cross_color_transform_bits: int,
        pad: [0xa5a5_a5a5],
    }
}

pub fn stats_line(stats: &WebPAuxStats) -> String {
    let mut line = format!("{}", stats.coded_size);
    for value in stats.psnr {
        write!(line, ",{:08x}", value.to_bits()).unwrap();
    }
    for value in stats
        .block_count
        .iter()
        .chain(&stats.header_bytes)
        .chain(stats.residual_bytes.as_flattened())
        .chain(&stats.segment_size)
        .chain(&stats.segment_quant)
        .chain(&stats.segment_level)
    {
        write!(line, ",{value}").unwrap();
    }
    write!(
        line,
        ",{},{},{},{},{},{},{},{},{},{},{},{}",
        stats.alpha_data_size,
        stats.layer_data_size,
        stats.lossless_features,
        stats.histogram_bits,
        stats.transform_bits,
        stats.cache_bits,
        stats.palette_size,
        stats.lossless_size,
        stats.lossless_hdr_size,
        stats.lossless_data_size,
        stats.cross_color_transform_bits,
        stats.pad[0]
    )
    .unwrap();
    line
}

pub fn join(values: &[impl std::fmt::Display]) -> String {
    if values.is_empty() {
        return "-".to_owned();
    }
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

/// What the callbacks of one encoding saw.
#[derive(Default)]
struct Sinks {
    out: RefCell<Vec<u8>>,
    sizes: RefCell<Vec<usize>>,
    progress: RefCell<Vec<i32>>,
    memory: RefCell<WebPMemoryWriter>,
}

/// Installs the case's writer, progress hook, statistics and `extra_info` on
/// `pic`, encodes it, and returns the return value and the error code.
fn encode_with<'a>(
    case: &Case,
    config: &WebPConfig,
    pic: &mut WebPPicture<'a>,
    sinks: &'a Sinks,
    stats: &'a mut WebPAuxStats,
    extra: Option<&'a mut [u8]>,
) -> (bool, u32) {
    if case.memory_writer {
        pic.writer = Some(Box::new(move |data: &[u8]| {
            webp_memory_write(&mut sinks.memory.borrow_mut(), data)
        }));
    } else {
        let fail_write_at = case.fail_write_at;
        pic.writer = Some(Box::new(move |data: &[u8]| {
            let call = sinks.sizes.borrow().len();
            sinks.sizes.borrow_mut().push(data.len());
            if call as i32 == fail_write_at {
                return false;
            }
            sinks.out.borrow_mut().extend_from_slice(data);
            true
        }));
    }
    if case.abort_at >= -1 {
        let abort_at = case.abort_at;
        pic.progress_hook = Some(Box::new(move |percent| {
            let call = sinks.progress.borrow().len();
            sinks.progress.borrow_mut().push(percent);
            call as i32 != abort_at
        }));
    } else {
        pic.progress_hook = None;
    }
    if case.with_stats {
        pic.stats = Some(stats);
    }
    if let Some(extra) = extra {
        pic.extra_info = Some(extra);
        pic.extra_info_type = case.extra_info_type;
    }
    let ret = webp_encode(config, pic);
    pic.writer = None;
    pic.progress_hook = None;
    (ret, pic.error_code.0)
}

/// The caller memory of `Layout::Caller`: the planes copied into rows `pad`
/// samples longer, filled with 0x5a, with one guard sample.
struct CallerPlanes {
    yuva: Vec<Vec<u8>>,
    argb: Option<Vec<u32>>,
    y_stride: i32,
    uv_stride: i32,
    a_stride: i32,
    argb_stride: i32,
}

fn copy_rows(
    storage: &PlaneStorage<'_, u8>,
    plane: PlaneOffset,
    src_stride: i32,
    width: i32,
    height: i32,
    stride: i32,
) -> Vec<u8> {
    let region = storage.region(plane.region).unwrap();
    let mut out = vec![0x5a_u8; (stride * height) as usize + 1];
    for y in 0..height as usize {
        let from = plane.offset + y * src_stride as usize;
        let to = y * stride as usize;
        out[to..to + width as usize].copy_from_slice(&region[from..from + width as usize]);
    }
    out
}

fn caller_planes(source: &WebPPicture<'_>, pad: i32) -> CallerPlanes {
    let (w, h) = (source.width, source.height);
    let (uv_w, uv_h) = ((w + 1) >> 1, (h + 1) >> 1);
    if source.use_argb != 0 {
        let stride = w + pad;
        let region = source
            .argb_storage
            .region(source.argb.unwrap().region)
            .unwrap();
        let mut argb = vec![0x5a5a_5a5a_u32; (stride * h) as usize + 1];
        for y in 0..h as usize {
            let from = source.argb.unwrap().offset + y * source.argb_stride as usize;
            let to = y * stride as usize;
            argb[to..to + w as usize].copy_from_slice(&region[from..from + w as usize]);
        }
        return CallerPlanes {
            yuva: Vec::new(),
            argb: Some(argb),
            y_stride: 0,
            uv_stride: 0,
            a_stride: 0,
            argb_stride: stride,
        };
    }
    let mut yuva = vec![
        copy_rows(
            &source.yuva,
            source.y.unwrap(),
            source.y_stride,
            w,
            h,
            w + pad,
        ),
        copy_rows(
            &source.yuva,
            source.u.unwrap(),
            source.uv_stride,
            uv_w,
            uv_h,
            uv_w + pad,
        ),
        copy_rows(
            &source.yuva,
            source.v.unwrap(),
            source.uv_stride,
            uv_w,
            uv_h,
            uv_w + pad,
        ),
    ];
    if let Some(a) = source.a {
        yuva.push(copy_rows(&source.yuva, a, source.a_stride, w, h, w + pad));
    }
    CallerPlanes {
        a_stride: if source.a.is_some() { w + pad } else { 0 },
        yuva,
        argb: None,
        y_stride: w + pad,
        uv_stride: uv_w + pad,
        argb_stride: 0,
    }
}

/// What the probe's `PrintCaller` prints.
fn caller_line(caller: &CallerPlanes) -> String {
    let mut line = String::new();
    for i in 0..4 {
        match caller.yuva.get(i) {
            Some(plane) => write!(line, "{:016x},", fnv(FNV_START, plane)).unwrap(),
            None => line.push_str("-,"),
        }
    }
    match &caller.argb {
        None => line.push('-'),
        Some(argb) => {
            let mut hash = FNV_START;
            for v in argb {
                hash = fnv(hash, &v.to_le_bytes());
            }
            write!(line, "{hash:016x}").unwrap();
        }
    }
    line
}

/// Replays `case` with the Rust translation and prints what the probe prints.
/// The second value is, for a view, the source picture before and after the
/// call, as `pic=` prints it.
pub fn replay(case: &Case) -> (Fields, Option<(String, String)>) {
    let config = case.config();
    let mut fields = Fields::new().with("valid", i32::from(webp_validate_config(&config)));

    let mut source = WebPPicture::default();
    assert!(webp_picture_init(&mut source));
    source.width = case.width;
    source.height = case.height;
    source.use_argb = i32::from(case.source >= 2);
    let mut rgba = Vec::with_capacity(case.rgba.len() * 4);
    let mut rgb = Vec::with_capacity(case.rgba.len() * 3);
    for word in &case.rgba {
        let bytes = word.to_le_bytes();
        rgba.extend_from_slice(&bytes);
        rgb.extend_from_slice(&bytes[..3]);
    }
    let imported = if case.source == 0 || case.source == 3 {
        webp_picture_import_rgb(&mut source, &rgb, 3 * case.width)
    } else {
        webp_picture_import_rgba(&mut source, &rgba, 4 * case.width)
    };
    assert!(imported, "import failed: {}", case.describe());

    let sinks = Sinks::default();
    let mut stats = poisoned_stats();
    let mut extra: Option<Vec<u8>> = None;
    let mut source_lines = None;

    let (ret, error, pic_line, caller) = match case.layout {
        Layout::Library => {
            let mut pic = source;
            if case.extra_info_type > 0 {
                extra = Some(zeroed_map(mb_count(pic.width, pic.height)));
            }
            let extra_slice = extra
                .as_mut()
                .map(|extra| &mut extra[..mb_count(pic.width, pic.height)]);
            let (ret, error) =
                encode_with(case, &config, &mut pic, &sinks, &mut stats, extra_slice);
            (ret, error, picture_line(&pic), None)
        }
        Layout::Caller { pad } => {
            let mut caller = caller_planes(&source, pad);
            let mut pic = WebPPicture::default();
            assert!(webp_picture_init(&mut pic));
            pic.use_argb = source.use_argb;
            pic.colorspace = source.colorspace;
            pic.width = source.width;
            pic.height = source.height;
            drop(source);
            if case.extra_info_type > 0 {
                extra = Some(zeroed_map(mb_count(pic.width, pic.height)));
            }
            let (ret, error, line) = {
                let mut pic = pic;
                if let Some(argb) = caller.argb.as_mut() {
                    pic.argb_storage = PlaneStorage::External(vec![argb.as_mut_slice()]);
                    pic.argb = Some(PlaneOffset {
                        region: 0,
                        offset: 0,
                    });
                    pic.argb_stride = caller.argb_stride;
                } else {
                    let has_alpha = caller.yuva.len() == 4;
                    pic.yuva = PlaneStorage::External(
                        caller.yuva.iter_mut().map(Vec::as_mut_slice).collect(),
                    );
                    let plane = |region| Some(PlaneOffset { region, offset: 0 });
                    pic.y = plane(0);
                    pic.u = plane(1);
                    pic.v = plane(2);
                    pic.a = if has_alpha { plane(3) } else { None };
                    pic.y_stride = caller.y_stride;
                    pic.uv_stride = caller.uv_stride;
                    pic.a_stride = caller.a_stride;
                }
                let extra_slice = extra
                    .as_mut()
                    .map(|extra| &mut extra[..mb_count(pic.width, pic.height)]);
                let (ret, error) =
                    encode_with(case, &config, &mut pic, &sinks, &mut stats, extra_slice);
                (ret, error, picture_line(&pic))
            };
            (ret, error, line, Some(caller_line(&caller)))
        }
        Layout::View {
            left,
            top,
            width,
            height,
        } => {
            let before = picture_line(&source);
            let view = webp_picture_view(&source, left, top, width, height);
            fields = fields.with("view", i32::from(view.is_some()));
            let Some(mut pic) = view else {
                return (fields, None);
            };
            if case.extra_info_type > 0 {
                extra = Some(zeroed_map(mb_count(pic.width, pic.height)));
            }
            let extra_slice = extra
                .as_mut()
                .map(|extra| &mut extra[..mb_count(pic.width, pic.height)]);
            let (ret, error) =
                encode_with(case, &config, &mut pic, &sinks, &mut stats, extra_slice);
            let line = picture_line(&pic);
            drop(pic);
            source_lines = Some((before, picture_line(&source)));
            (ret, error, line, None)
        }
    };

    fields = fields
        .with("ret", i32::from(ret))
        .with("error", error)
        .with("progress", join(&sinks.progress.borrow()));
    if case.memory_writer {
        let memory = sinks.memory.borrow();
        fields = fields
            .with("mem", format!("{},{}", memory.size, memory.max_size))
            .hex("out", &memory.mem[..memory.size]);
    } else {
        fields = fields
            .with("writes", join(&sinks.sizes.borrow()))
            .hex("out", &sinks.out.borrow());
    }
    if case.with_stats {
        fields = fields.with("stats", stats_line(&stats));
    }
    if let Some(extra) = &extra {
        fields = fields.hex("extra", extra);
    }
    fields = fields.with("pic", pic_line);
    if let Some(caller) = caller {
        fields = fields.with("caller", caller);
    }
    (fields, source_lines)
}

/// The probe's `extra_info` map: zeros and one guard byte (0xaa).
fn zeroed_map(mb_count: usize) -> Vec<u8> {
    let mut map = vec![0_u8; mb_count + 1];
    map[mb_count] = 0xaa;
    map
}

pub fn mb_count(width: i32, height: i32) -> usize {
    (((width + 15) >> 4) * ((height + 15) >> 4)) as usize
}
