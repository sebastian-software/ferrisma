// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Observation driver for the Rust public encoder. It mirrors the epub-run
// and epub-wrap protocols of tools/oracle/encoder_public.cc token for token;
// it calls the translated public entry points and implements no encoder
// algorithm. The 'S' checkpoint reads the same stored state through the
// encoder's read-only observation accessors and serializes the metadata with
// the translated WriteCodestreamHeaders, as the oracle does with the
// original.
#![allow(dead_code)]
use jxl::jxl::cms::jxl_cms::get_default_cms;
use jxl::jxl::codestream_header::{
    JxlBasicInfo, JxlBlendInfo, JxlBlendMode, JxlExtraChannelInfo, JxlExtraChannelType,
    JxlFrameHeader, JxlOrientation,
};
use jxl::jxl::color_encoding::{
    ColorSpace, JxlColorEncoding, Primaries, RenderingIntent, TransferFunction, WhitePoint,
};
use jxl::jxl::enc_aux_out::LayerType;
use jxl::jxl::enc_bit_writer::BitWriter;
use jxl::jxl::enc_fields::write_codestream_headers;
use jxl::jxl::encode::{self as api, JxlEncoder, JxlEncoderFrameSettings, JxlEncoderStatus};
use jxl::jxl::encode_internal::{
    JxlEncoderChunkedFrameAdapter, JxlEncoderFrameSettingsValues, JxlEncoderOutputProcessorWrapper,
    JxlEncoderQueuedFrame, append_data,
};
use jxl::jxl::memory_manager::{JxlMemoryManager, memory_manager_init};
use jxl::jxl::types::{JxlBitDepth, JxlBitDepthType, JxlDataType, JxlEndianness, JxlPixelFormat};
use std::alloc::{Layout, alloc, dealloc};
use std::collections::{BTreeMap, BTreeSet};
use std::ffi::{CString, c_void};
use std::fmt::Write as _;

pub use api::JxlEncoderAdmission;

pub fn hex(data: &[u8]) -> String {
    if data.is_empty() {
        return "-".into();
    }
    let mut result = String::with_capacity(data.len() * 2);
    for byte in data {
        write!(result, "{byte:02x}").unwrap();
    }
    result
}

pub fn unhex(data: &str) -> Vec<u8> {
    if data == "-" {
        return Vec::new();
    }
    assert!(data.len().is_multiple_of(2), "odd hex length");
    (0..data.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&data[i..i + 2], 16).unwrap())
        .collect()
}

fn f32_bits(field: &str) -> f32 {
    f32::from_bits(field.parse().unwrap())
}

fn digest(data: &[u8]) -> u64 {
    let mut h: u64 = 14695981039346656037;
    for &byte in data {
        h ^= u64::from(byte);
        h = h.wrapping_mul(1099511628211);
    }
    h
}

struct Allocator {
    calls: usize,
    fail: usize,
    next: usize,
    live: BTreeMap<usize, (usize, Layout)>,
    events: Vec<String>,
}

unsafe extern "C" fn allocate(opaque: *mut c_void, size: usize) -> *mut c_void {
    // SAFETY: The driver retains this allocator for the encoder's lifetime and
    // callbacks never run concurrently.
    let a = unsafe { &mut *opaque.cast::<Allocator>() };
    a.calls += 1;
    // Every manager allocation in these histories is an object whose size is
    // a property of the language layout, so only order and identity are kept.
    a.events.push("A".into());
    if a.calls == a.fail {
        return std::ptr::null_mut();
    }
    let layout = Layout::from_size_align(size.max(1), 16).unwrap();
    // SAFETY: Nonzero checked layout, retained for the matching free.
    let p = unsafe { alloc(layout) };
    if !p.is_null() {
        a.next += 1;
        a.live.insert(p as usize, (a.next, layout));
    }
    p.cast()
}

unsafe extern "C" fn release(opaque: *mut c_void, p: *mut c_void) {
    // SAFETY: As in allocate.
    let a = unsafe { &mut *opaque.cast::<Allocator>() };
    if p.is_null() {
        a.events.push("F0".into());
        return;
    }
    let (id, layout) = a.live.remove(&(p as usize)).unwrap();
    a.events.push(format!("F{id}"));
    // SAFETY: Unique callback-returned pointer and its saved layout.
    unsafe { dealloc(p.cast(), layout) };
}

/// cs.wp.prim.tf.gamma-bits.ri with fixed custom coordinates, or the full
/// x: form, as decoder_public_driver.h's ParseEncoding.
fn parse_encoding(token: &str) -> JxlColorEncoding {
    if let Some(full) = token.strip_prefix("x:") {
        let f: Vec<u64> = full.split(':').map(|v| v.parse().unwrap()).collect();
        assert_eq!(f.len(), 14, "color encoding token");
        return JxlColorEncoding {
            color_space: ColorSpace(f[0] as u32),
            white_point: WhitePoint(f[1] as u32),
            white_point_xy: [f64::from_bits(f[2]), f64::from_bits(f[3])],
            primaries: Primaries(f[4] as u32),
            primaries_red_xy: [f64::from_bits(f[5]), f64::from_bits(f[6])],
            primaries_green_xy: [f64::from_bits(f[7]), f64::from_bits(f[8])],
            primaries_blue_xy: [f64::from_bits(f[9]), f64::from_bits(f[10])],
            transfer_function: TransferFunction(f[11] as u32),
            gamma: f64::from_bits(f[12]),
            rendering_intent: RenderingIntent(f[13] as u32),
        };
    }
    let fields: Vec<u64> = token.split('.').map(|v| v.parse().unwrap()).collect();
    JxlColorEncoding {
        color_space: ColorSpace(fields[0] as u32),
        white_point: WhitePoint(fields[1] as u32),
        white_point_xy: [0.3127, 0.329],
        primaries: Primaries(fields[2] as u32),
        primaries_red_xy: [0.68, 0.32],
        primaries_green_xy: [0.265, 0.69],
        primaries_blue_xy: [0.15, 0.06],
        transfer_function: TransferFunction(fields[3] as u32),
        gamma: f64::from_bits(fields[4]),
        rendering_intent: RenderingIntent(fields[5] as u32),
    }
}

fn blend(b: &JxlBlendInfo) -> String {
    format!("{}.{}.{}.{}", b.blendmode.0, b.source, b.alpha, b.clamp)
}

fn b(v: bool) -> u8 {
    u8::from(v)
}

fn values(v: &JxlEncoderFrameSettingsValues) -> String {
    let c = &v.cparams;
    let mut o = String::new();
    write!(
        o,
        "{},{},{}",
        b(v.lossless),
        c.butteraugli_distance.to_bits(),
        c.ec_distance.len()
    )
    .unwrap();
    for d in &c.ec_distance {
        write!(o, ":{}", d.to_bits()).unwrap();
    }
    let fields: Vec<String> = vec![
        b(c.max_error_mode).to_string(),
        b(c.disable_perceptual_optimizations).to_string(),
        (c.speed_tier as i32).to_string(),
        c.brotli_effort.to_string(),
        c.decoding_speed_tier.to_string(),
        c.color_transform.0.to_string(),
        b(c.modular_mode).to_string(),
        c.modular_group_size_shift.to_string(),
        c.preview.0.to_string(),
        c.noise.0.to_string(),
        c.dots.0.to_string(),
        c.patches.0.to_string(),
        c.gaborish.0.to_string(),
        c.epf.to_string(),
        c.progressive_mode.0.to_string(),
        c.qprogressive_mode.0.to_string(),
        b(c.centerfirst).to_string(),
        c.center_x.to_string(),
        c.center_y.to_string(),
        c.progressive_dc.to_string(),
        c.keep_invisible.0.to_string(),
        b(c.cms_set).to_string(),
        b(c.force_cfl_jpeg_recompression).to_string(),
        c.force_lfs_jpeg_recompression.to_string(),
        b(c.jpeg_compress_boxes).to_string(),
        b(c.jpeg_keep_exif).to_string(),
        b(c.jpeg_keep_xmp).to_string(),
        b(c.jpeg_keep_jumbf).to_string(),
        c.photon_noise_iso.to_bits().to_string(),
        c.options.nb_repeats.to_bits().to_string(),
        c.options.max_properties.to_string(),
        c.options.predictor.0.to_string(),
        c.responsive.to_string(),
        c.colorspace.to_string(),
        c.channel_colors_pre_transform_percent.to_bits().to_string(),
        c.channel_colors_percent.to_bits().to_string(),
        c.palette_colors.to_string(),
        b(c.lossy_palette).to_string(),
        c.resampling.to_string(),
        c.ec_resampling.to_string(),
        b(c.already_downsampled).to_string(),
        c.level.to_string(),
        c.buffering.to_string(),
        c.output_mode.to_string(),
        b(c.use_full_image_heuristics).to_string(),
        b(c.debug_image.is_some()).to_string(),
    ];
    write!(o, ",{}", fields.join(",")).unwrap();
    let h = &v.header;
    let l = &h.layer_info;
    write!(
        o,
        ",h{},{},{},{},{},{},{},{},{},{},{}",
        h.duration,
        h.timecode,
        h.name_length,
        h.is_last,
        l.have_crop,
        l.crop_x0,
        l.crop_y0,
        l.xsize,
        l.ysize,
        blend(&l.blend_info),
        l.save_as_reference
    )
    .unwrap();
    write!(o, ",b{}", v.extra_channel_blend_info.len()).unwrap();
    for info in &v.extra_channel_blend_info {
        write!(o, ":{}", blend(info)).unwrap();
    }
    write!(
        o,
        ",n{},d{}:{}:{},i{},x{},a{}",
        hex(&v.frame_name),
        v.image_bit_depth.depth_type.0,
        v.image_bit_depth.bits_per_sample,
        v.image_bit_depth.exponent_bits_per_sample,
        b(v.frame_index_box),
        b(v.aux_out.is_some()),
        v.strip_alpha
    )
    .unwrap();
    o
}

/// As the oracle's BasicInfo: the padding bytes, indeterminate in the
/// original until SetBasicInfo copies a caller's struct, are printed only
/// while basic_info_set holds.
fn basic_info(info: &JxlBasicInfo, padding_defined: bool) -> String {
    format!(
        "{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
        info.have_container,
        info.xsize,
        info.ysize,
        info.bits_per_sample,
        info.exponent_bits_per_sample,
        info.intensity_target.to_bits(),
        info.min_nits.to_bits(),
        info.relative_to_max_display,
        info.linear_below.to_bits(),
        info.uses_original_profile,
        info.have_preview,
        info.have_animation,
        info.orientation.0,
        info.num_color_channels,
        info.num_extra_channels,
        info.alpha_bits,
        info.alpha_exponent_bits,
        info.alpha_premultiplied,
        info.preview.xsize,
        info.preview.ysize,
        info.animation.tps_numerator,
        info.animation.tps_denominator,
        info.animation.num_loops,
        info.animation.have_timecodes,
        info.intrinsic_xsize,
        info.intrinsic_ysize,
        if padding_defined {
            hex(&info.padding)
        } else {
            "~".into()
        }
    )
}

fn encoding(c: &JxlColorEncoding) -> String {
    format!(
        "{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}",
        c.color_space.0,
        c.white_point.0,
        c.white_point_xy[0].to_bits(),
        c.white_point_xy[1].to_bits(),
        c.primaries.0,
        c.primaries_red_xy[0].to_bits(),
        c.primaries_red_xy[1].to_bits(),
        c.primaries_green_xy[0].to_bits(),
        c.primaries_green_xy[1].to_bits(),
        c.primaries_blue_xy[0].to_bits(),
        c.primaries_blue_xy[1].to_bits(),
        c.transfer_function.0,
        c.gamma.to_bits(),
        c.rendering_intent.0
    )
}

fn format(f: &[&str], at: usize) -> JxlPixelFormat {
    JxlPixelFormat {
        num_channels: f[at].parse().unwrap(),
        data_type: JxlDataType(f[at + 1].parse().unwrap()),
        endianness: JxlEndianness(f[at + 2].parse().unwrap()),
        align: f[at + 3].parse().unwrap(),
    }
}

/// Caller sample bytes, as the oracle's Pattern: one extra byte, and for
/// float and float16 formats only bytes 0x30-0x3f, which form finite samples
/// below 2 at any offset and byte order.
fn pattern(size: usize, seed: usize, floats: bool) -> Vec<u8> {
    (0..size + 1)
        .map(|i| {
            let v = ((i * 13 + seed * 29 + 7) & 0xff) as u8;
            if floats { 0x30 | (v & 0x0f) } else { v }
        })
        .collect()
}

/// Image content for the lossy encoder gate, as the oracle's Content: each
/// sample is q/1024 for an integer q in [0, 1024] stored exactly in the
/// requested type, byte order and row alignment.
fn content_hash(x: u32, y: u32, c: u32) -> u32 {
    let mut h = x.wrapping_mul(0x9E37_79B1)
        ^ y.wrapping_add(0x7F4A_7C15).wrapping_mul(0x85EB_CA77)
        ^ (c + 1).wrapping_mul(0xC2B2_AE3D);
    h ^= h >> 15;
    h = h.wrapping_mul(0x2C1B_3C6D);
    h ^= h >> 12;
    h
}

fn content_q(content: u32, x: u32, y: u32, c: u32, xsize: u32, ysize: u32) -> u32 {
    let tri = |t: u32| {
        let m = t % 512;
        if m < 256 { m } else { 511 - m }
    };
    match content {
        1 => 300 + 200 * c,
        2 => (x * 700) / xsize.max(1) + (y * 300) / ysize.max(1) + c * 10,
        3 => content_hash(x, y, c) % 1025,
        5 => {
            let (gx, gy) = (x % 10, y % 14);
            if x < 4 || y < 4 || gx >= 7 || gy >= 10 {
                return 960;
            }
            let glyph = (x / 10 * 7 + y / 14 * 3) % 5;
            if !content_hash(gx, gy, glyph + 100).is_multiple_of(3) {
                80 + 5 * c
            } else {
                960
            }
        }
        _ => {
            let mut q = 150 + tri(x * 3 + y * 2 + c * 40) + tri(x * 11 / 7 + 5 * c) / 2;
            if (x / 13 + y / 9).is_multiple_of(5) {
                q += 120;
            }
            q += content_hash(x, y, c) % 48;
            q.min(1024)
        }
    }
}

/// The oracle's AlphaQ: alpha mode 1 opaque, 2 a ramp with fully transparent
/// stripes, 3 a ramp without zeros.
fn alpha_q(mode: u32, x: u32, y: u32, xsize: u32) -> u32 {
    match mode {
        1 => 1024,
        2 => {
            if (x + 2 * y) % 23 < 6 {
                return 0;
            }
            (40 + (x * 1024) / xsize.max(1) + content_hash(x, y, 9) % 32).min(1024)
        }
        _ => 200 + (x * 800) / xsize.max(1) + (y % 7) * 3,
    }
}

/// The oracle's Content; `content + 16 * m` selects alpha mode `m` for the
/// channel `alpha_c`.
fn content(
    size: usize,
    format: &JxlPixelFormat,
    content: u32,
    xsize: u32,
    ysize: u32,
    alpha_c: Option<u32>,
) -> Vec<u8> {
    let alpha_mode = content / 16;
    let content = content % 16;
    let mut data = vec![0u8; size + 1];
    let bytes = match format.data_type {
        JxlDataType::UINT8 => 1,
        JxlDataType::FLOAT => 4,
        _ => 2,
    };
    let row = xsize as usize * format.num_channels as usize * bytes;
    let stride = if format.align > 1 && !row.is_multiple_of(format.align) {
        row + format.align - row % format.align
    } else {
        row
    };
    let big = format.endianness == JxlEndianness::BIG;
    for y in 0..ysize {
        for x in 0..xsize {
            for c in 0..format.num_channels {
                let at = y as usize * stride
                    + (x as usize * format.num_channels as usize + c as usize) * bytes;
                if at + bytes > size {
                    continue;
                }
                let q = if alpha_mode != 0 && Some(c) == alpha_c {
                    alpha_q(alpha_mode, x, y, xsize)
                } else {
                    content_q(content, x, y, c, xsize, ysize)
                };
                let word = match format.data_type {
                    JxlDataType::UINT8 => (q * 255 + 512) / 1024,
                    JxlDataType::UINT16 => (q * 65535 + 512) / 1024,
                    JxlDataType::FLOAT => (q as f32 / 1024.0).to_bits(),
                    _ => {
                        if q == 0 {
                            0
                        } else {
                            let e = 31 - q.leading_zeros();
                            ((e + 5) << 10) | (((q << 10) >> e) & 0x3ff)
                        }
                    }
                };
                for b in 0..bytes {
                    let shift = if big { 8 * (bytes - 1 - b) } else { 8 * b };
                    data[at + b] = (word >> shift) as u8;
                }
            }
        }
    }
    data
}

fn float_format(format: &JxlPixelFormat) -> bool {
    format.data_type == JxlDataType::FLOAT || format.data_type == JxlDataType::FLOAT16
}

fn channel(adapter: &JxlEncoderChunkedFrameAdapter<'_>, index: usize) -> String {
    let c = &adapter.channels()[index];
    let format = c.format();
    format!(
        "c{index}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}.{}",
        format.num_channels,
        format.data_type.0,
        format.endianness.0,
        format.align,
        c.xsize(),
        c.ysize(),
        c.bytes_per_pixel(),
        c.stride(),
        c.buffer_size(),
        c.copied_buffer().len(),
        digest(c.copied_buffer())
    )
}

/// One history's observation and, if the encoder returned an admission
/// result, the index of that operation and its checkpoint.
pub struct Observation {
    pub text: String,
    pub admission: Option<(usize, JxlEncoderAdmission)>,
}

pub struct Driver {
    out: String,
    allocator: Box<Allocator>,
    manager: Box<JxlMemoryManager>,
    enc: *mut JxlEncoder,
    print_events: bool,
    settings: Vec<*mut JxlEncoderFrameSettings>,
    ec_set: BTreeMap<usize, BTreeSet<usize>>,
    // ProcessOutput's caller variables; the encoder retains their addresses.
    output: Vec<u8>,
    next_out: Box<*mut u8>,
    avail_out: Box<usize>,
    // encoder streaming
    processor: Box<Processor>,
    // Boxed: the encoder retains each source's address as its opaque value.
    #[allow(clippy::vec_box, reason = "Stable addresses for the callbacks")]
    sources: Vec<Box<Source>>,
    // end encoder streaming
}

impl Drop for Driver {
    fn drop(&mut self) {
        // SAFETY: The driver owns the encoder; null after Destroy.
        unsafe { api::jxl_encoder_destroy(self.enc) };
    }
}

impl Driver {
    pub fn new() -> Self {
        let mut allocator = Box::new(Allocator {
            calls: 0,
            fail: 0,
            next: 0,
            live: BTreeMap::new(),
            events: Vec::new(),
        });
        let manager = Box::new(JxlMemoryManager {
            opaque: (&mut *allocator as *mut Allocator).cast(),
            alloc: Some(allocate),
            free: Some(release),
        });
        Self {
            out: String::new(),
            allocator,
            manager,
            enc: std::ptr::null_mut(),
            print_events: false,
            settings: Vec::new(),
            ec_set: BTreeMap::new(),
            output: Vec::new(),
            next_out: Box::new(std::ptr::null_mut()),
            avail_out: Box::new(0),
            processor: Box::default(),
            sources: Vec::new(),
        }
    }

    fn enc(&mut self) -> &mut JxlEncoder {
        // SAFETY: A live encoder between Create and Destroy; no other
        // reference to it is live while the driver calls into it.
        unsafe { &mut *self.enc }
    }

    pub fn create(&mut self, mm: i32, fail: usize) -> bool {
        self.allocator.fail = fail;
        self.print_events = mm == 1;
        let manager: *const JxlMemoryManager = if mm != 0 {
            &*self.manager
        } else {
            std::ptr::null()
        };
        // SAFETY: The boxed manager and allocator outlive the encoder, and
        // the allocator returns 16-byte aligned blocks.
        self.enc = unsafe { api::jxl_encoder_create(manager) };
        write!(self.out, "create:{}", u8::from(!self.enc.is_null())).unwrap();
        !self.enc.is_null()
    }

    pub fn destroy(&mut self) -> String {
        // SAFETY: The live encoder is not used again.
        unsafe { api::jxl_encoder_destroy(self.enc) };
        self.enc = std::ptr::null_mut();
        self.out.push_str(" E");
        if self.print_events {
            for e in &self.allocator.events {
                write!(self.out, " {e}").unwrap();
            }
        }
        std::mem::take(&mut self.out)
    }

    fn settings(&self, index: &str) -> *mut JxlEncoderFrameSettings {
        self.settings[index.parse::<usize>().unwrap()]
    }

    pub fn op(&mut self, op: &str) {
        let kind = op.chars().next().unwrap();
        let arg = &op[1..];
        let f: Vec<&str> = if arg.is_empty() {
            Vec::new()
        } else {
            arg.split(',').collect()
        };
        let result = self.result(kind, arg, &f);
        let error = api::jxl_encoder_get_error(self.enc()).0;
        write!(self.out, " {kind}:{result}/{error}").unwrap();
    }

    fn result(&mut self, kind: char, arg: &str, f: &[&str]) -> String {
        let s = |status: JxlEncoderStatus| status.0.to_string();
        // SAFETY (for every frame-settings call below): each index names a
        // live settings object of the live encoder, created by 's' and cleared
        // on Reset, and the driver holds no other reference during the call.
        match kind {
            'V' => api::jxl_encoder_version().to_string(),
            'x' => {
                api::jxl_encoder_reset(self.enc());
                self.settings.clear();
                self.ec_set.clear();
                "0".into()
            }
            'e' => api::jxl_encoder_get_error(self.enc()).0.to_string(),
            'B' => {
                let mut info = JxlBasicInfo::zeroed();
                api::jxl_encoder_init_basic_info(&mut info);
                for field in f {
                    let (key, value) = field.split_once(':').unwrap();
                    let v: u32 = value.parse().unwrap();
                    let fv = f32::from_bits(v);
                    match key {
                        "w" => info.xsize = v,
                        "h" => info.ysize = v,
                        "bits" => info.bits_per_sample = v,
                        "exp" => info.exponent_bits_per_sample = v,
                        "it" => info.intensity_target = fv,
                        "mn" => info.min_nits = fv,
                        "rel" => info.relative_to_max_display = v as i32,
                        "lb" => info.linear_below = fv,
                        "orig" => info.uses_original_profile = v as i32,
                        "prev" => info.have_preview = v as i32,
                        "anim" => info.have_animation = v as i32,
                        "or" => info.orientation = JxlOrientation(v),
                        "c" => info.num_color_channels = v,
                        "ec" => info.num_extra_channels = v,
                        "ab" => info.alpha_bits = v,
                        "aexp" => info.alpha_exponent_bits = v,
                        "apre" => info.alpha_premultiplied = v as i32,
                        "px" => info.preview.xsize = v,
                        "py" => info.preview.ysize = v,
                        "ix" => info.intrinsic_xsize = v,
                        "iy" => info.intrinsic_ysize = v,
                        "tn" => info.animation.tps_numerator = v,
                        "td" => info.animation.tps_denominator = v,
                        "nl" => info.animation.num_loops = v,
                        "tc" => info.animation.have_timecodes = v as i32,
                        "cont" => info.have_container = v as i32,
                        "pad" => info.padding[v as usize % 100] = (v >> 8) as u8,
                        _ => panic!("basic info key"),
                    }
                }
                s(api::jxl_encoder_set_basic_info(self.enc(), &info))
            }
            'I' => {
                let mut info = JxlBasicInfo::zeroed();
                info.padding = [0xA5; 100];
                api::jxl_encoder_init_basic_info(&mut info);
                let mut header = JxlFrameHeader::default();
                api::jxl_encoder_init_frame_header(&mut header);
                let mut blend_info = JxlBlendInfo::default();
                api::jxl_encoder_init_blend_info(&mut blend_info);
                let mut ec = JxlExtraChannelInfo::default();
                api::jxl_encoder_init_extra_channel_info(
                    JxlExtraChannelType(arg.parse().unwrap()),
                    &mut ec,
                );
                let l = &header.layer_info;
                let mut o = format!(
                    "0;{};{},{},{},{},{},{},{},{},{},{},{};{};{},{},{},{},{},{}",
                    basic_info(&info, true),
                    header.duration,
                    header.timecode,
                    header.name_length,
                    header.is_last,
                    l.have_crop,
                    l.crop_x0,
                    l.crop_y0,
                    l.xsize,
                    l.ysize,
                    blend(&l.blend_info),
                    l.save_as_reference,
                    blend(&blend_info),
                    ec.channel_type.0,
                    ec.bits_per_sample,
                    ec.exponent_bits_per_sample,
                    ec.dim_shift,
                    ec.name_length,
                    ec.alpha_premultiplied
                );
                for v in ec.spot_color {
                    write!(o, ",{}", v.to_bits()).unwrap();
                }
                write!(o, ",{}", ec.cfa_channel).unwrap();
                o
            }
            'K' => {
                let ec = JxlExtraChannelInfo {
                    channel_type: JxlExtraChannelType(f[1].parse().unwrap()),
                    bits_per_sample: f[2].parse().unwrap(),
                    exponent_bits_per_sample: f[3].parse().unwrap(),
                    dim_shift: f[4].parse().unwrap(),
                    name_length: 0,
                    alpha_premultiplied: f[5].parse().unwrap(),
                    spot_color: [
                        f32_bits(f[7]),
                        f32_bits(f[8]),
                        f32_bits(f[9]),
                        f32_bits(f[10]),
                    ],
                    cfa_channel: f[6].parse().unwrap(),
                };
                s(api::jxl_encoder_set_extra_channel_info(
                    self.enc(),
                    f[0].parse().unwrap(),
                    &ec,
                ))
            }
            'n' => {
                let name = unhex(f[1]);
                s(api::jxl_encoder_set_extra_channel_name(
                    self.enc(),
                    f[0].parse().unwrap(),
                    &name,
                ))
            }
            'C' => {
                let color = parse_encoding(arg);
                s(api::jxl_encoder_set_color_encoding(self.enc(), &color))
            }
            'G' | 'g' => {
                let mut color = JxlColorEncoding::default();
                if kind == 'G' {
                    api::jxl_color_encoding_set_to_srgb(&mut color, arg.parse().unwrap());
                } else {
                    api::jxl_color_encoding_set_to_linear_srgb(&mut color, arg.parse().unwrap());
                }
                s(api::jxl_encoder_set_color_encoding(self.enc(), &color))
            }
            'T' => {
                let mut color = JxlColorEncoding::default();
                api::jxl_color_encoding_set_to_srgb(&mut color, arg.parse().unwrap());
                let srgb = encoding(&color);
                api::jxl_color_encoding_set_to_linear_srgb(&mut color, arg.parse().unwrap());
                format!("0;{srgb};{}", encoding(&color))
            }
            'Q' => {
                let icc = unhex(arg);
                s(api::jxl_encoder_set_icc_profile(self.enc(), &icc))
            }
            'L' => s(api::jxl_encoder_set_codestream_level(
                self.enc(),
                arg.parse().unwrap(),
            )),
            'l' => api::jxl_encoder_get_required_codestream_level(self.enc()).to_string(),
            'U' => s(api::jxl_encoder_use_container(
                self.enc(),
                arg.parse().unwrap(),
            )),
            'J' => s(api::jxl_encoder_store_jpeg_metadata(
                self.enc(),
                arg.parse().unwrap(),
            )),
            'X' => s(api::jxl_encoder_use_boxes(self.enc())),
            'A' => {
                api::jxl_encoder_allow_expert_options(self.enc());
                "0".into()
            }
            'M' => {
                // SAFETY: The translated default CMS obeys cms_interface.h.
                unsafe { api::jxl_encoder_set_cms(self.enc(), *get_default_cms()) };
                "0".into()
            }
            'u' => s(api::jxl_encoder_set_upsampling_mode(
                self.enc(),
                f[0].parse().unwrap(),
                f[1].parse().unwrap(),
            )),
            's' => {
                let source = if arg == "-" {
                    std::ptr::null()
                } else {
                    self.settings(arg).cast_const()
                };
                // SAFETY: The live encoder and a live or null source.
                let created = unsafe { api::jxl_encoder_frame_settings_create(self.enc, source) };
                if created.is_null() {
                    "-".into()
                } else {
                    self.settings.push(created);
                    (self.settings.len() - 1).to_string()
                }
            }
            'o' => s(unsafe {
                api::jxl_encoder_frame_settings_set_option(
                    self.settings(f[0]),
                    api::JxlEncoderFrameSettingId(f[1].parse().unwrap()),
                    f[2].parse().unwrap(),
                )
            }),
            'f' => s(unsafe {
                api::jxl_encoder_frame_settings_set_float_option(
                    self.settings(f[0]),
                    api::JxlEncoderFrameSettingId(f[1].parse().unwrap()),
                    f32_bits(f[2]),
                )
            }),
            'd' => s(unsafe {
                api::jxl_encoder_set_frame_distance(self.settings(f[0]), f32_bits(f[1]))
            }),
            'D' => s(unsafe {
                api::jxl_encoder_set_extra_channel_distance(
                    self.settings(f[0]),
                    f[1].parse().unwrap(),
                    f32_bits(f[2]),
                )
            }),
            'q' => api::jxl_encoder_distance_from_quality(f32_bits(arg))
                .to_bits()
                .to_string(),
            'y' => s(unsafe {
                api::jxl_encoder_set_frame_lossless(self.settings(f[0]), f[1].parse().unwrap())
            }),
            'h' => {
                let v: Vec<i64> = f[1..].iter().map(|x| x.parse().unwrap()).collect();
                assert_eq!(v.len(), 14, "frame header fields");
                let mut h = JxlFrameHeader::default();
                api::jxl_encoder_init_frame_header(&mut h);
                h.duration = v[0] as u32;
                h.timecode = v[1] as u32;
                h.name_length = v[2] as u32;
                h.is_last = v[3] as i32;
                h.layer_info.have_crop = v[4] as i32;
                h.layer_info.crop_x0 = v[5] as i32;
                h.layer_info.crop_y0 = v[6] as i32;
                h.layer_info.xsize = v[7] as u32;
                h.layer_info.ysize = v[8] as u32;
                h.layer_info.blend_info.blendmode = JxlBlendMode(v[9] as u32);
                h.layer_info.blend_info.source = v[10] as u32;
                h.layer_info.blend_info.alpha = v[11] as u32;
                h.layer_info.blend_info.clamp = v[12] as i32;
                h.layer_info.save_as_reference = v[13] as u32;
                s(unsafe { api::jxl_encoder_set_frame_header(self.settings(f[0]), &h) })
            }
            't' => {
                let info = JxlBlendInfo {
                    blendmode: JxlBlendMode(f[2].parse().unwrap()),
                    source: f[3].parse().unwrap(),
                    alpha: f[4].parse().unwrap(),
                    clamp: f[5].parse().unwrap(),
                };
                s(unsafe {
                    api::jxl_encoder_set_extra_channel_blend_info(
                        self.settings(f[0]),
                        f[1].parse().unwrap(),
                        &info,
                    )
                })
            }
            'N' => {
                let name = (f[1] != "~").then(|| {
                    let mut bytes = unhex(f[1]);
                    // The C string ends at its first NUL byte.
                    if let Some(end) = bytes.iter().position(|&b| b == 0) {
                        bytes.truncate(end);
                    }
                    CString::new(bytes).unwrap()
                });
                s(
                    unsafe {
                        api::jxl_encoder_set_frame_name(self.settings(f[0]), name.as_deref())
                    },
                )
            }
            'W' => {
                let depth = JxlBitDepth {
                    depth_type: JxlBitDepthType(f[1].parse().unwrap()),
                    bits_per_sample: f[2].parse().unwrap(),
                    exponent_bits_per_sample: f[3].parse().unwrap(),
                };
                s(unsafe { api::jxl_encoder_set_frame_bit_depth(self.settings(f[0]), &depth) })
            }
            'a' => {
                let format = format(f, 1);
                let null = f[5] == "~";
                let size = if null { 0 } else { f[5].parse().unwrap() };
                // Optional image content: a7,...,size,content,xsize,ysize.
                let alpha_c = matches!(format.num_channels, 2 | 4).then(|| format.num_channels - 1);
                let pixels = if f.len() > 6 {
                    content(
                        size,
                        &format,
                        f[6].parse().unwrap(),
                        f[7].parse().unwrap(),
                        f[8].parse().unwrap(),
                        alpha_c,
                    )
                } else {
                    pattern(size, 7, float_format(&format))
                };
                let buffer = (!null).then(|| &pixels[..size]);
                s(
                    unsafe {
                        api::jxl_encoder_add_image_frame(self.settings(f[0]), &format, buffer)
                    },
                )
            }
            'E' => {
                let format = format(f, 1);
                let null = f[5] == "~";
                let size = if null { 0 } else { f[5].parse().unwrap() };
                let index: u32 = f[6].parse().unwrap();
                // Optional content: E0,...,size,index,content,xsize,ysize.
                let pixels = if f.len() > 7 {
                    content(
                        size,
                        &format,
                        f[7].parse().unwrap(),
                        f[8].parse().unwrap(),
                        f[9].parse().unwrap(),
                        Some(0),
                    )
                } else {
                    pattern(size, 11 + index as usize, float_format(&format))
                };
                let buffer = (!null).then(|| &pixels[..size]);
                let status = unsafe {
                    api::jxl_encoder_set_extra_channel_buffer(
                        self.settings(f[0]),
                        &format,
                        buffer,
                        index,
                    )
                };
                if status == JxlEncoderStatus::SUCCESS {
                    let frame = self.enc().input_queue().last().unwrap().frame_address();
                    self.ec_set
                        .entry(frame.unwrap() as usize)
                        .or_default()
                        .insert(index as usize);
                }
                s(status)
            }
            'O' => {
                let box_type: [u8; 4] = unhex(f[0]).try_into().expect("box type");
                let size: usize = f[1].parse().unwrap();
                let contents = pattern(size, 5, false);
                s(api::jxl_encoder_add_box(
                    self.enc(),
                    &box_type,
                    &contents[..size],
                    f[2].parse().unwrap(),
                ))
            }
            'F' => {
                api::jxl_encoder_close_frames(self.enc());
                "0".into()
            }
            'Y' => {
                api::jxl_encoder_close_boxes(self.enc());
                "0".into()
            }
            'Z' => {
                api::jxl_encoder_close_input(self.enc());
                "0".into()
            }
            'p' => {
                let avail: usize = f[0].parse().unwrap();
                self.output = vec![0xA5; avail + 1];
                *self.next_out = self.output.as_mut_ptr();
                *self.avail_out = avail;
                let next_out: *mut *mut u8 = &mut *self.next_out;
                let avail_out: *mut usize = &mut *self.avail_out;
                // SAFETY: Both boxed caller variables outlive the encoder, and
                // the output vector holds `avail` writable bytes.
                let status =
                    unsafe { api::jxl_encoder_process_output(&mut *self.enc, next_out, avail_out) };
                // SAFETY: The encoder call has returned; read the caller
                // variables it may have advanced.
                let written = unsafe { (*next_out).offset_from(self.output.as_ptr()) };
                format!(
                    "{},{},{},{}",
                    status.0,
                    written,
                    *self.avail_out,
                    hex(&self.output[..avail])
                )
            }
            'S' => format!("0;{}", self.state()),
            // encoder streaming
            'w' => {
                self.processor.schedule = f[1].parse().unwrap();
                self.processor.stop_at = f[2].parse().unwrap();
                self.processor.stop_kind = f[3].parse().unwrap();
                let callbacks = self.processor.callbacks(f[0].parse().unwrap());
                // SAFETY: The boxed processor outlives the encoder, which the
                // driver destroys first; its callbacks follow encode.h.
                s(unsafe { api::jxl_encoder_set_output_processor(self.enc(), callbacks) })
            }
            'c' => {
                // The oracle's 'c': AddChunkedFrame over a recording source.
                let mut source = Box::<Source>::default();
                let color_format = format(f, 2);
                source.color_format = Some(color_format);
                let (xs, ys, pad): (usize, usize, usize) = (
                    f[7].parse().unwrap(),
                    f[8].parse().unwrap(),
                    f[9].parse().unwrap(),
                );
                source.xsize = xs;
                source.ysize = ys;
                source.fail_at = f[10].parse().unwrap();
                let ec_format = JxlPixelFormat {
                    num_channels: 1,
                    data_type: JxlDataType(f[11].parse().unwrap()),
                    endianness: color_format.endianness,
                    align: 0,
                };
                source.ec_format = Some(ec_format);
                let content_id: u32 = f[6].parse().unwrap();
                let mut packed = color_format;
                packed.align = 0;
                let row = xs * source_bytes_per_pixel(&packed);
                let alpha_c = matches!(packed.num_channels, 2 | 4).then(|| packed.num_channels - 1);
                source.color = padded(
                    &content(row * ys, &packed, content_id, xs as u32, ys as u32, alpha_c),
                    row,
                    ys,
                    pad,
                );
                source.color_stride = row + pad;
                let ec_row = xs * source_bytes_per_pixel(&ec_format);
                for _ in 0..f[12].parse::<usize>().unwrap() {
                    let data = content(
                        ec_row * ys,
                        &ec_format,
                        content_id,
                        xs as u32,
                        ys as u32,
                        Some(0),
                    );
                    source.ec.push(padded(&data, ec_row, ys, pad));
                }
                source.ec_stride = ec_row + pad;
                let callbacks = source.callbacks();
                let is_last: i32 = f[1].parse().unwrap();
                // SAFETY: The boxed source outlives the encoder (the driver
                // destroys the encoder first) and follows encode.h.
                let status = unsafe {
                    api::jxl_encoder_add_chunked_frame(self.settings(f[0]), is_last, callbacks)
                };
                let events = compress(&source.events);
                self.sources.push(source);
                format!("{};{}", status.0, events)
            }
            'i' => s(api::jxl_encoder_flush_input(self.enc())),
            'r' => format!("0;{}", self.processor.report()),
            // end encoder streaming
            _ => panic!("encoder public op"),
        }
    }

    fn state(&mut self) -> String {
        let enc_ptr = self.enc.cast_const();
        // SAFETY: A live encoder; shared observation only.
        let e = unsafe { &*self.enc };
        let v = e.scalars();
        let out = e.output_processor();
        let index = e.frame_index_box();
        let mut o = format!(
            "{}{}{}{}{}{}{}{}{}{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
            b(v.cms_set),
            b(v.use_container),
            b(v.use_boxes),
            b(v.store_jpeg_metadata),
            b(v.wrote_bytes),
            b(v.frames_closed),
            b(v.boxes_closed),
            b(v.basic_info_set),
            b(v.color_encoding_set),
            b(v.intensity_target_set),
            b(v.allow_expert_options),
            v.codestream_level,
            v.brotli_effort,
            v.container_ftyp_version,
            v.num_queued_frames,
            v.num_queued_boxes,
            v.codestream_bytes_written_end_of_frame,
            v.jxlp_counter,
            v.error.0,
            b(index.index_box_requested_through_api),
            index.tnum,
            index.tden,
            index.entries.len(),
            e.jpeg_metadata().len(),
            b(v.thread_pool_set),
            b(out.output_processor_set()),
            b(out.has_avail_out()),
            out.current_position(),
            b(out.has_output_to_write())
        );
        // The stored metadata, serialized by the translated writer on a copy.
        let mut copy = e.metadata().clone();
        let mut writer = BitWriter::new();
        let status = write_codestream_headers(&mut copy, &mut writer, None).and_then(|()| {
            writer.with_max_bits(
                8,
                LayerType::Header,
                None,
                |w, _| {
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
        });
        match status {
            Ok(()) => write!(o, ";H0:{}", hex(writer.get_span())).unwrap(),
            Err(_) => o.push_str(";H1:-"),
        }
        let color = &e.metadata().m.color_encoding;
        write!(o, ";I{}:{}", b(color.want_icc()), hex(color.icc())).unwrap();
        write!(o, ";B{}", basic_info(e.basic_info(), v.basic_info_set)).unwrap();
        for (i, settings) in e.frame_settings().enumerate() {
            write!(
                o,
                ";s{i}:{},{}",
                values(settings.values()),
                b(settings.encoder() == enc_ptr)
            )
            .unwrap();
        }
        write!(o, ";q{}", e.input_queue().len()).unwrap();
        for input in e.input_queue() {
            if let Some(frame) = input.frame() {
                let frame: &JxlEncoderQueuedFrame = frame;
                write!(
                    o,
                    ";F{}:{}:ec",
                    input.output_mode,
                    values(&frame.option_values)
                )
                .unwrap();
                for v in &frame.ec_initialized {
                    write!(o, "{v}").unwrap();
                }
                let data = &frame.frame_data;
                write!(
                    o,
                    ":{}.{}.{}:{}",
                    data.xsize(),
                    data.ysize(),
                    data.channels().len(),
                    channel(data, 0)
                )
                .unwrap();
                let address = input.frame_address().unwrap() as usize;
                if let Some(set) = self.ec_set.get(&address) {
                    for &index in set {
                        write!(o, ":{}", channel(data, 1 + index)).unwrap();
                    }
                }
            } else if let Some(queued_box) = input.queued_box() {
                write!(
                    o,
                    ";X{}:{}:{}:{}:{}",
                    input.output_mode,
                    hex(&queued_box.box_type),
                    b(queued_box.compress_box),
                    queued_box.contents.len(),
                    digest(&queued_box.contents)
                )
                .unwrap();
            } else {
                write!(o, ";L{}", input.output_mode).unwrap();
            }
        }
        o
    }
}

/// Replays one epub-run history. After an admission result, the remaining
/// operations are outside the admitted profile and are not executed.
pub fn observe(args: &mut std::str::SplitWhitespace<'_>) -> Observation {
    let mm: i32 = args.next().unwrap().parse().unwrap();
    let fail: usize = args.next().unwrap().parse().unwrap();
    let mut driver = Driver::new();
    let mut admission = None;
    api::take_admissions();
    if driver.create(mm, fail) {
        for (index, op) in args.enumerate() {
            driver.op(op);
            let admitted = api::take_admissions();
            assert!(admitted.len() <= 1, "one admission result per call");
            if let Some(&checkpoint) = admitted.first() {
                admission = Some((index, checkpoint));
                break;
            }
        }
        return Observation {
            text: driver.destroy(),
            admission,
        };
    }
    Observation {
        text: std::mem::take(&mut driver.out),
        admission,
    }
}

/// Replays one epub-wrap history on the translated buffer-mode wrapper.
pub fn observe_wrapper(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut manager = JxlMemoryManager::default();
    // SAFETY: Both pointers address the same local manager, as the oracle's
    // MemoryManagerInit with a null source.
    unsafe { memory_manager_init(&mut manager, std::ptr::null()) }.unwrap();
    // SAFETY: The local manager outlives the wrapper.
    let mut wrapper = unsafe { JxlEncoderOutputProcessorWrapper::new(&manager) };
    let mut processor: Box<Processor> = Box::default(); // encoder streaming
    let mut caller: Vec<u8> = Vec::new();
    let mut next: Box<*mut u8> = Box::new(std::ptr::null_mut());
    let mut avail: Box<usize> = Box::new(0);
    let mut caller_size = 0usize;
    let mut out = String::new();
    let pattern = |count: usize, seed: usize| -> Vec<u8> {
        (0..count)
            .map(|i| ((i * 7 + seed * 31 + 3) & 0xff) as u8)
            .collect()
    };
    for (position, op) in args.enumerate() {
        let kind = op.chars().next().unwrap();
        let f: Vec<usize> = if op.len() > 1 {
            op[1..].split(',').map(|v| v.parse().unwrap()).collect()
        } else {
            Vec::new()
        };
        if position > 0 {
            out.push(' ');
        }
        write!(out, "{kind}:").unwrap();
        let st = |status: Result<(), jxl::jxl::base::status::StatusCode>| u8::from(status.is_err());
        match kind {
            'A' => {
                caller_size = f[0];
                caller = vec![0xA5; caller_size + 1];
                *next = caller.as_mut_ptr();
                *avail = caller_size;
                let next_out: *mut *mut u8 = &mut *next;
                let avail_out: *mut usize = &mut *avail;
                // SAFETY: The boxed caller variables and buffer outlive every
                // later wrapper call; the buffer holds `caller_size` bytes.
                write!(
                    out,
                    "{}",
                    st(unsafe { wrapper.set_avail_out(next_out, avail_out) })
                )
                .unwrap();
            }
            'G' | 'R' => match wrapper.get_buffer(f[0], f[1]) {
                Err(code) => write!(out, "1,{}", code as i32).unwrap(),
                Ok(mut buffer) => {
                    // Appends and advances stay within the window.
                    let count = f[2].min(buffer.size());
                    write!(out, "0,{}", buffer.size()).unwrap();
                    let data = pattern(count, f[3]);
                    write!(out, ",{}", st(buffer.append(&data))).unwrap();
                    if kind == 'R' {
                        write!(out, ",{}", st(buffer.release())).unwrap();
                    } else {
                        // The oracle's buffer destructor releases and ignores
                        // the status (its JXL_DASSERT is off in the release
                        // reference); release here so that the checked
                        // profile's debug assertion in Drop sees no buffer.
                        let _ = buffer.release(); // encoder streaming
                    }
                }
            },
            'K' => write!(out, "{}", st(wrapper.seek(f[0]))).unwrap(),
            'k' => {
                let pos = wrapper.current_position() + f[0];
                write!(out, "{}", st(wrapper.seek(pos))).unwrap();
            }
            'H' => {
                // The AppendBox order: reserve the header, write the contents,
                // write the header into the reserved bytes and return to the end.
                let header = f[0];
                let start = wrapper.current_position();
                write!(out, "{}", st(wrapper.seek(start + header))).unwrap();
                let contents = pattern(f[1], f[2]);
                write!(out, "{}", st(append_data(&mut wrapper, &contents))).unwrap();
                let end = wrapper.current_position();
                write!(out, "{}", st(wrapper.seek(start))).unwrap();
                let bytes = pattern(header, f[2] + 1);
                write!(out, "{}", st(append_data(&mut wrapper, &bytes))).unwrap();
                write!(out, "{}", st(wrapper.seek(end))).unwrap();
            }
            'F' => write!(out, "{}", st(wrapper.set_finalized_position())).unwrap(),
            'D' => {
                let data = pattern(f[0], f[1]);
                write!(out, "{}", st(append_data(&mut wrapper, &data))).unwrap();
            }
            // encoder streaming
            'E' => {
                // The wrapper with an external output processor, as
                // JxlEncoderSetOutputProcessor makes it.
                processor.schedule = f[1] as u32;
                processor.stop_at = f[2];
                processor.stop_kind = f[3] as u32;
                let callbacks = processor.callbacks(f[0] as u32);
                // SAFETY: The local manager and the boxed processor outlive
                // the wrapper.
                wrapper = unsafe {
                    JxlEncoderOutputProcessorWrapper::with_processor(&manager, callbacks)
                };
                out.push('0');
            }
            'r' => out.push_str(&processor.report()),
            // end encoder streaming
            'P' => write!(
                out,
                "{},{},{},{},{}",
                wrapper.current_position(),
                b(wrapper.has_output_to_write()),
                b(wrapper.was_stop_requested()),
                b(wrapper.has_avail_out()),
                b(wrapper.output_processor_set())
            )
            .unwrap(),
            _ => panic!("output wrapper op"),
        }
        if !caller.is_empty() {
            // SAFETY: The wrapper writes only during its calls, which ended.
            let offset = unsafe { (*next).offset_from(caller.as_ptr()) };
            write!(
                out,
                "|{},{},{}",
                offset,
                *avail,
                hex(&caller[..caller_size])
            )
            .unwrap();
        }
    }
    out
}

// encoder streaming
/// The run-length notation of a callback log (encoder_streaming.h Compress).
pub fn compress(events: &[String]) -> String {
    let mut out = String::new();
    let mut i = 0;
    while i < events.len() {
        let (mut best_len, mut best_k) = (1, 1);
        for len in 1..=3 {
            if i + len > events.len() {
                break;
            }
            let mut k = 1;
            while i + (k + 1) * len <= events.len()
                && events[i..i + len] == events[i + k * len..i + (k + 1) * len]
            {
                k += 1;
            }
            if k >= 2 && k * len > best_k * best_len {
                best_len = len;
                best_k = k;
            }
        }
        if !out.is_empty() {
            out.push('.');
        }
        if best_k == 1 {
            out.push_str(&events[i]);
        } else if best_len == 1 {
            write!(out, "{}*{best_k}", events[i]).unwrap();
        } else {
            write!(out, "[{}]*{best_k}", events[i..i + best_len].join(",")).unwrap();
        }
        i += best_len * best_k;
    }
    if out.is_empty() { "-".into() } else { out }
}

/// The recording output processor of encoder_streaming.h, schedule for
/// schedule.
#[derive(Default)]
pub struct Processor {
    file: Vec<u8>,
    position: usize,
    finalized: usize,
    schedule: u32,
    stop_at: usize,
    stop_kind: u32,
    calls: usize,
    last_suggested: usize,
    last_given: usize,
    events: Vec<String>,
}

unsafe extern "C" fn processor_get_buffer(opaque: *mut c_void, size: *mut usize) -> *mut c_void {
    // SAFETY: The opaque value is the driver's boxed processor, and `size`
    // is the encoder's valid local; no other reference is live.
    let (p, size) = unsafe { (&mut *opaque.cast::<Processor>(), &mut *size) };
    p.calls += 1;
    let e = if p.calls > 1 && *size + p.last_given == p.last_suggested {
        "g=>".to_string()
    } else {
        format!("g{size}>")
    };
    p.last_suggested = *size;
    p.last_given = 0;
    if p.stop_at != 0 && p.calls == p.stop_at {
        if p.stop_kind == 0 {
            p.events.push(format!("{e}~"));
            return std::ptr::null_mut();
        }
        *size = 0;
        p.events.push(format!("{e}0"));
        if p.file.len() < p.position + 1 {
            p.file.resize(p.position + 1, 0xDA);
        }
        return p.file[p.position..].as_mut_ptr().cast();
    }
    if p.schedule == 1 {
        *size = 1;
    } else if p.schedule == 3 {
        const CYCLE: [usize; 3] = [3, 64, 4096];
        *size = (*size).min(CYCLE[p.calls % 3]);
    }
    if p.position + *size > p.file.len() {
        p.file.resize(p.position + *size, 0xDA);
    }
    if p.schedule == 0 {
        *size = p.file.len() - p.position;
    }
    p.last_given = *size;
    p.events.push(format!("{e}{size}"));
    p.file[p.position..].as_mut_ptr().cast()
}

unsafe extern "C" fn processor_release_buffer(opaque: *mut c_void, written_bytes: usize) {
    // SAFETY: As for processor_get_buffer.
    let p = unsafe { &mut *opaque.cast::<Processor>() };
    p.events.push(format!("r{written_bytes}"));
    p.position += written_bytes;
}

unsafe extern "C" fn processor_seek(opaque: *mut c_void, position: u64) {
    // SAFETY: As for processor_get_buffer.
    let p = unsafe { &mut *opaque.cast::<Processor>() };
    p.events.push(format!("k{position}"));
    p.position = position as usize;
}

unsafe extern "C" fn processor_set_finalized_position(opaque: *mut c_void, finalized: u64) {
    // SAFETY: As for processor_get_buffer.
    let p = unsafe { &mut *opaque.cast::<Processor>() };
    p.events.push(if finalized as usize == p.position {
        "f=".to_string()
    } else {
        format!("f{finalized}")
    });
    p.finalized = finalized as usize;
}

impl Processor {
    fn callbacks(&mut self, mask: u32) -> api::JxlEncoderOutputProcessor {
        api::JxlEncoderOutputProcessor {
            opaque: (self as *mut Processor).cast(),
            get_buffer: (mask & 1 == 0).then_some(processor_get_buffer as _),
            release_buffer: (mask & 2 == 0).then_some(processor_release_buffer as _),
            seek: (mask & 4 == 0).then_some(processor_seek as _),
            set_finalized_position: (mask & 8 == 0)
                .then_some(processor_set_finalized_position as _),
        }
    }
    fn report(&mut self) -> String {
        let out = format!(
            "{};{},{},{};{}",
            compress(&self.events),
            self.position,
            self.finalized,
            self.file.len(),
            hex(&self.file)
        );
        self.events.clear();
        out
    }
}

/// The recording input source of encoder_streaming.h.
#[derive(Default)]
pub struct Source {
    color_format: Option<JxlPixelFormat>,
    ec_format: Option<JxlPixelFormat>,
    xsize: usize,
    ysize: usize,
    color_stride: usize,
    ec_stride: usize,
    color: Vec<u8>,
    ec: Vec<Vec<u8>>,
    fail_at: usize,
    requests: usize,
    events: Vec<String>,
}

fn source_bytes_per_pixel(f: &JxlPixelFormat) -> usize {
    let bytes = match f.data_type {
        JxlDataType::UINT8 => 1,
        JxlDataType::FLOAT => 4,
        _ => 2,
    };
    f.num_channels as usize * bytes
}

fn padded(data: &[u8], row: usize, ysize: usize, pad: usize) -> Vec<u8> {
    let mut out = vec![0xEE; (row + pad) * ysize + 1];
    for y in 0..ysize {
        out[y * (row + pad)..y * (row + pad) + row].copy_from_slice(&data[y * row..(y + 1) * row]);
    }
    out
}

impl Source {
    fn inside(&self, x: usize, y: usize, xs: usize, ys: usize) -> bool {
        x <= self.xsize && xs <= self.xsize - x && y <= self.ysize && ys <= self.ysize - y
    }
    #[allow(clippy::too_many_arguments, reason = "The oracle's Request")]
    fn request(
        &mut self,
        e: String,
        buffer: Option<*const u8>,
        stride: usize,
        bpp: usize,
        (x, y, xs, ys): (usize, usize, usize, usize),
        row_offset: *mut usize,
    ) -> *const c_void {
        self.requests += 1;
        if self.fail_at != 0 && self.requests == self.fail_at {
            self.events.push(format!("{e}~"));
            return std::ptr::null();
        }
        let Some(buffer) = buffer.filter(|_| self.inside(x, y, xs, ys)) else {
            self.events.push(format!("{e}!"));
            return std::ptr::null();
        };
        self.events.push(e);
        // SAFETY: The encoder's valid local; the offset is inside the
        // buffer because the rectangle is inside the image.
        unsafe {
            *row_offset = stride;
            buffer.add(y * stride + x * bpp).cast()
        }
    }
    fn callbacks(&mut self) -> api::JxlChunkedFrameInputSource {
        api::JxlChunkedFrameInputSource {
            opaque: (self as *mut Source).cast(),
            get_color_channels_pixel_format: Some(source_color_format),
            get_color_channel_data_at: Some(source_color_at),
            get_extra_channel_pixel_format: Some(source_extra_format),
            get_extra_channel_data_at: Some(source_extra_at),
            release_buffer: Some(source_release),
        }
    }
}

unsafe extern "C" fn source_color_format(opaque: *mut c_void, pixel_format: *mut JxlPixelFormat) {
    // SAFETY: The opaque value is the driver's boxed source; the format is
    // the encoder's valid local.
    let s = unsafe { &mut *opaque.cast::<Source>() };
    s.events.push("F".into());
    unsafe { *pixel_format = s.color_format.unwrap() };
}

unsafe extern "C" fn source_color_at(
    opaque: *mut c_void,
    x: usize,
    y: usize,
    xs: usize,
    ys: usize,
    row_offset: *mut usize,
) -> *const c_void {
    // SAFETY: As for source_color_format.
    let s = unsafe { &mut *opaque.cast::<Source>() };
    let bpp = source_bytes_per_pixel(&s.color_format.unwrap());
    let buffer = Some(s.color.as_ptr());
    let stride = s.color_stride;
    s.request(
        format!("c{x},{y},{xs},{ys}"),
        buffer,
        stride,
        bpp,
        (x, y, xs, ys),
        row_offset,
    )
}

unsafe extern "C" fn source_extra_format(
    opaque: *mut c_void,
    ec_index: usize,
    pixel_format: *mut JxlPixelFormat,
) {
    // SAFETY: As for source_color_format.
    let s = unsafe { &mut *opaque.cast::<Source>() };
    s.events.push(format!("E{ec_index}"));
    unsafe { *pixel_format = s.ec_format.unwrap() };
}

unsafe extern "C" fn source_extra_at(
    opaque: *mut c_void,
    ec_index: usize,
    x: usize,
    y: usize,
    xs: usize,
    ys: usize,
    row_offset: *mut usize,
) -> *const c_void {
    // SAFETY: As for source_color_format.
    let s = unsafe { &mut *opaque.cast::<Source>() };
    let bpp = source_bytes_per_pixel(&s.ec_format.unwrap());
    let buffer = s.ec.get(ec_index).map(|b| b.as_ptr());
    let stride = s.ec_stride;
    s.request(
        format!("e{ec_index},{x},{y},{xs},{ys}"),
        buffer,
        stride,
        bpp,
        (x, y, xs, ys),
        row_offset,
    )
}

unsafe extern "C" fn source_release(opaque: *mut c_void, buf: *const c_void) {
    // SAFETY: As for source_color_format.
    let s = unsafe { &mut *opaque.cast::<Source>() };
    let p = buf as usize;
    let mut e = "R?".to_string();
    let base = s.color.as_ptr() as usize;
    if p >= base && p < base + s.color.len() {
        e = format!("Rc{}", p - base);
    }
    for (i, ec) in s.ec.iter().enumerate() {
        let base = ec.as_ptr() as usize;
        if p >= base && p < base + ec.len() {
            e = format!("Re{i}:{}", p - base);
        }
    }
    s.events.push(e);
}

/// Replays one epub-rect request on the translated buffer-backed adapter.
pub fn observe_rect(args: &mut std::str::SplitWhitespace<'_>) -> String {
    let mut n = || args.next().unwrap().parse::<usize>().unwrap();
    let (xsize, ysize, channels, extra) = (n(), n(), n(), n());
    let color_format = JxlPixelFormat {
        num_channels: channels as u32,
        data_type: JxlDataType::UINT8,
        endianness: JxlEndianness::NATIVE,
        align: 0,
    };
    let alpha_format = JxlPixelFormat {
        num_channels: 1,
        ..color_format
    };
    let color = vec![0u8; xsize * ysize * channels];
    let alpha = vec![0u8; xsize * ysize];
    let mut adapter = JxlEncoderChunkedFrameAdapter::new(xsize, ysize, extra);
    let mut out = String::new();
    write!(
        out,
        "{}",
        u8::from(adapter.set_from_buffer(0, Some(&color), color.len(), color_format))
    )
    .unwrap();
    for ec in 0..extra {
        write!(
            out,
            "{}",
            u8::from(adapter.set_from_buffer(1 + ec, Some(&alpha), alpha.len(), alpha_format))
        )
        .unwrap();
    }
    let input = adapter.get_input_source();
    for op in args {
        let f: Vec<usize> = op[1..].split(',').map(|v| v.parse().unwrap()).collect();
        let mut row_offset = 0;
        let kind = op.as_bytes()[0];
        let buffer = if kind == b'c' {
            input.get_color_buffer(f[0], f[1], f[2], f[3], &mut row_offset)
        } else {
            input.get_extra_channel_buffer(f[0], f[1], f[2], f[3], f[4], &mut row_offset)
        };
        write!(out, " {}:", kind as char).unwrap();
        if buffer.is_null() {
            out.push('-');
        } else if kind == b'c' {
            write!(
                out,
                "{},{row_offset}",
                buffer.as_ptr() as usize - color.as_ptr() as usize
            )
            .unwrap();
        } else {
            write!(out, "+,{row_offset}").unwrap();
        }
    }
    out
}
// end encoder streaming
