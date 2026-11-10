//! Shared helpers of the WP12 differential tests: the commands of the probe
//! `tools/oracle/probes/wp12/decode.c` and the output that the Rust translation
//! produces for them, in the probe's format.
//!
//! Include it with `#[path = "wp12_support/mod.rs"] mod wp12_support;`.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Uninitialized library memory.** C allocates output buffers with
//!    `malloc`; Rust initializes every allocation (ADR 0003). The tests digest
//!    library memory only after a complete decoding, and during incremental
//!    decoding only the luma (and RGB) rows up to `last_y`: with rescaling, the
//!    U, V and alpha rows lag behind and are uninitialized in C. Caller memory
//!    is filled with a pattern and digested whole, including guard bytes.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use webp::dec::buffer_dec::is_null_plane;
use webp::dec::idec_dec::{
    PlaneRows, WebPIDecoder, webp_i_append, webp_i_dec_get_rgb, webp_i_dec_get_yuva, webp_i_decode,
    webp_i_decoded_area, webp_i_new_decoder, webp_i_new_rgb, webp_i_new_yuv, webp_i_new_yuva,
    webp_i_update,
};
use webp::dec::webp_dec::{
    webp_decode, webp_decode_argb, webp_decode_argb_into, webp_decode_bgr, webp_decode_bgr_into,
    webp_decode_bgra, webp_decode_bgra_into, webp_decode_rgb, webp_decode_rgb_into,
    webp_decode_rgba, webp_decode_rgba_into, webp_decode_yuv, webp_decode_yuv_into, webp_get_info,
    webp_validate_decoder_config,
};
use webp::webp::decode::{
    VP8StatusCode, WEBP_CSP_MODE, WebPBitstreamFeatures, WebPDecBuffer, WebPDecBufferPlanes,
    WebPDecoderConfig, WebPDecoderOptions, WebPRGBABuffer, WebPYUVABuffer, webp_get_features,
    webp_init_decoder_config, webp_is_rgb_mode,
};
use webp::webp::types::{PlaneOffset, PlaneStorage};

/// Guard bytes on both sides of every caller buffer.
pub const GUARD: usize = 64;

/// Bytes per pixel of each colorspace (`kModeBpp`).
pub const MODE_BPP: [usize; 13] = [3, 4, 3, 4, 4, 2, 2, 4, 4, 4, 2, 1, 1];

/// SplitMix64, a small deterministic generator for the scripts.
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

    /// A value in `low..=high`.
    pub fn range(&mut self, low: i64, high: i64) -> i64 {
        low + (self.next_u64() % ((high - low + 1) as u64)) as i64
    }
}

/// The 64-bit FNV-1a digest of the probe.
pub fn digest(bytes: &[u8]) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for &byte in bytes {
        hash ^= u64::from(byte);
        hash = hash.wrapping_mul(0x0100_0000_01b3);
    }
    hash
}

/// `PrintRowsDigest`: `height` rows of `width` bytes, `stride` apart from
/// position `first` of `region`.
pub fn rows_digest(region: &[u8], first: usize, stride: i32, width: i32, height: i32) -> u64 {
    let mut hash = 0xcbf2_9ce4_8422_2325_u64;
    for y in 0..height.max(0) {
        let row = first
            .checked_add_signed((i64::from(y) * i64::from(stride)) as isize)
            .expect("the row is inside the region");
        for &byte in &region[row..row + width.max(0) as usize] {
            hash ^= u64::from(byte);
            hash = hash.wrapping_mul(0x0100_0000_01b3);
        }
    }
    hash
}

/// Lowercase hex, `-` for no bytes.
pub fn token(bytes: &[u8]) -> String {
    if bytes.is_empty() {
        return "-".to_owned();
    }
    let mut text = String::with_capacity(2 * bytes.len());
    for byte in bytes {
        write!(text, "{byte:02x}").unwrap();
    }
    text
}

/// The decoding options of a command, in the probe's order.
#[derive(Clone, Copy, Debug, Default)]
pub struct Options(pub [i32; 14]);

impl Options {
    pub const BYPASS: usize = 0;
    pub const NOFANCY: usize = 1;
    pub const CROP: usize = 2;
    pub const CROP_LEFT: usize = 3;
    pub const CROP_TOP: usize = 4;
    pub const CROP_WIDTH: usize = 5;
    pub const CROP_HEIGHT: usize = 6;
    pub const SCALE: usize = 7;
    pub const SCALED_WIDTH: usize = 8;
    pub const SCALED_HEIGHT: usize = 9;
    pub const THREADS: usize = 10;
    pub const DITHER: usize = 11;
    pub const FLIP: usize = 12;
    pub const ALPHA_DITHER: usize = 13;

    pub fn token(&self) -> String {
        self.0
            .iter()
            .map(i32::to_string)
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn decoder_options(&self) -> WebPDecoderOptions {
        let v = self.0;
        WebPDecoderOptions {
            bypass_filtering: v[0],
            no_fancy_upsampling: v[1],
            use_cropping: v[2],
            crop_left: v[3],
            crop_top: v[4],
            crop_width: v[5],
            crop_height: v[6],
            use_scaling: v[7],
            scaled_width: v[8],
            scaled_height: v[9],
            use_threads: v[10],
            dithering_strength: v[11],
            flip: v[12],
            alpha_dithering_strength: v[13],
            pad: [0; 5],
        }
    }
}

/// The caller memory of a command: (stride, size) per plane, and whether it is
/// slow external memory (C: `is_external_memory` 2).
#[derive(Clone, Debug, Default)]
pub struct Geometry(pub Vec<(i32, usize)>, pub bool);

impl Geometry {
    pub fn token(&self) -> String {
        if self.0.is_empty() {
            return "-".to_owned();
        }
        let planes = self
            .0
            .iter()
            .map(|(stride, size)| format!("{stride},{size}"))
            .collect::<Vec<_>>()
            .join(",");
        if self.1 { format!("S{planes}") } else { planes }
    }

    /// The same memory as slow external memory.
    pub fn slow(mut self) -> Self {
        self.1 = true;
        self
    }

    /// The guarded blocks, filled with the probe's pattern.
    pub fn blocks(&self) -> Vec<Vec<u8>> {
        self.0
            .iter()
            .map(|&(_, size)| (0..size + 2 * GUARD).map(|j| (j * 31 + 7) as u8).collect())
            .collect()
    }
}

/// The plane slices of guarded blocks.
pub fn plane_slices(blocks: &mut [Vec<u8>]) -> Vec<&mut [u8]> {
    blocks
        .iter_mut()
        .map(|block| {
            let size = block.len() - 2 * GUARD;
            &mut block[GUARD..GUARD + size]
        })
        .collect()
}

/// `PrintGeometry`.
pub fn print_blocks(out: &mut String, blocks: &[Vec<u8>]) {
    for (i, block) in blocks.iter().enumerate() {
        writeln!(out, "ext{i}={:016x}", digest(block)).unwrap();
    }
}

/// Reads the first `len` bytes of a file (-1: all).
pub fn read_prefix(path: &Path, len: i64) -> Vec<u8> {
    let mut data = std::fs::read(path).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
    if len >= 0 && (len as usize) < data.len() {
        data.truncate(len as usize);
    }
    data
}

pub fn code(status: Result<(), VP8StatusCode>) -> u32 {
    status.err().unwrap_or(VP8StatusCode::VP8_STATUS_OK).0
}

fn print_features(out: &mut String, key: &str, f: &WebPBitstreamFeatures) {
    writeln!(
        out,
        "{key}={},{},{},{},{}",
        f.width, f.height, f.has_alpha, f.has_animation, f.format
    )
    .unwrap();
}

/// `Features` of the probe.
pub fn features(out: &mut String, data: &[u8]) {
    let mut features = WebPBitstreamFeatures {
        width: 0x5a5a_5a5a,
        height: 0x5a5a_5a5a,
        has_alpha: 0x5a5a_5a5a,
        has_animation: 0x5a5a_5a5a,
        format: 0x5a5a_5a5a,
        pad: [0x5a5a_5a5a; 5],
    };
    writeln!(
        out,
        "status={}",
        code(webp_get_features(data, &mut features))
    )
    .unwrap();
    print_features(out, "features", &features);
    let (mut width, mut height) = (-1, -1);
    let ok = webp_get_info(data, Some(&mut width), Some(&mut height));
    writeln!(out, "info={},{width},{height}", i32::from(ok)).unwrap();
}

/// The offset of a plane in its region, -1 for C's `NULL` (or a plane in freed
/// memory, for which the probe prints -1 too).
fn plane_offset(storage: &PlaneStorage<'_, u8>, plane: Option<PlaneOffset>) -> i64 {
    match plane {
        Some(plane) if !is_null_plane(storage, plane) => plane.offset as i64,
        _ => -1,
    }
}

/// `PrintBuffer` without the memory digests.
fn print_buffer_fields(out: &mut String, buf: &WebPDecBuffer<'_>) {
    let external = match buf.storage {
        PlaneStorage::External(_) if buf.slow_memory => 2,
        PlaneStorage::External(_) => 1,
        _ => 0,
    };
    writeln!(
        out,
        "out={},{},{},{external}",
        buf.colorspace.0, buf.width, buf.height
    )
    .unwrap();
    // A buffer that was never allocated holds the zeroed member of the other
    // view, which C reads as zeros.
    if webp_is_rgb_mode(buf.colorspace) {
        let rgba = match buf.u {
            WebPDecBufferPlanes::RGBA(rgba) => rgba,
            WebPDecBufferPlanes::YUVA(yuva) => {
                assert_eq!(yuva, WebPYUVABuffer::default(), "zeroed member");
                WebPRGBABuffer::default()
            }
        };
        writeln!(
            out,
            "rgba={},{},{}",
            plane_offset(&buf.storage, Some(rgba.rgba)),
            rgba.stride,
            rgba.size
        )
        .unwrap();
    } else {
        let yuva = match buf.u {
            WebPDecBufferPlanes::YUVA(yuva) => yuva,
            WebPDecBufferPlanes::RGBA(rgba) => {
                assert_eq!(rgba, WebPRGBABuffer::default(), "zeroed member");
                WebPYUVABuffer {
                    y: PlaneOffset {
                        region: usize::MAX,
                        offset: 0,
                    },
                    u: PlaneOffset {
                        region: usize::MAX,
                        offset: 0,
                    },
                    v: PlaneOffset {
                        region: usize::MAX,
                        offset: 0,
                    },
                    ..WebPYUVABuffer::default()
                }
            }
        };
        writeln!(
            out,
            "yuva={},{},{},{},{},{},{},{},{},{},{},{}",
            plane_offset(&buf.storage, Some(yuva.y)),
            plane_offset(&buf.storage, Some(yuva.u)),
            plane_offset(&buf.storage, Some(yuva.v)),
            plane_offset(&buf.storage, yuva.a),
            yuva.y_stride,
            yuva.u_stride,
            yuva.v_stride,
            yuva.a_stride,
            yuva.y_size,
            yuva.u_size,
            yuva.v_size,
            yuva.a_size
        )
        .unwrap();
    }
}

/// `PrintBuffer` for a buffer with library memory; `partial` for an
/// incremental decoding that did not finish, whose undecoded rows are
/// uninitialized in C.
fn print_internal_memory(out: &mut String, buf: &WebPDecBuffer<'_>, partial: bool) {
    match &buf.storage {
        PlaneStorage::Internal(memory) if !memory.is_empty() && partial => {
            out.push_str("mem=partial\n");
        }
        PlaneStorage::Internal(memory) if !memory.is_empty() => {
            writeln!(out, "mem={:016x}", digest(memory)).unwrap();
        }
        _ => out.push_str("mem=null\n"),
    }
}

/// Sets the caller memory of `buf` as `SetGeometry` does.
fn set_geometry<'a>(buf: &mut WebPDecBuffer<'a>, geometry: &Geometry, slices: Vec<&'a mut [u8]>) {
    if geometry.0.is_empty() {
        return;
    }
    let plane = |i: usize| PlaneOffset {
        region: i,
        offset: 0,
    };
    let g = |i: usize| geometry.0.get(i).copied().unwrap_or((0, 0));
    if webp_is_rgb_mode(buf.colorspace) {
        buf.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
            rgba: plane(0),
            stride: g(0).0,
            size: g(0).1,
        });
    } else {
        // A plane that the geometry does not give is NULL: a region that does
        // not exist.
        let p = |i: usize| PlaneOffset {
            region: if i < geometry.0.len() { i } else { usize::MAX },
            offset: 0,
        };
        buf.u = WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
            y: p(0),
            y_stride: g(0).0,
            y_size: g(0).1,
            u: p(1),
            u_stride: g(1).0,
            u_size: g(1).1,
            v: p(2),
            v_stride: g(2).0,
            v_size: g(2).1,
            a: (geometry.0.len() > 3).then(|| p(3)),
            a_stride: g(3).0,
            a_size: g(3).1,
        });
    }
    buf.storage = PlaneStorage::External(slices);
    buf.slow_memory = geometry.1;
}

/// `Decode` of the probe.
pub fn decode(out: &mut String, data: &[u8], mode: u32, options: Options, geometry: &Geometry) {
    let mut blocks = geometry.blocks();
    {
        let mut config = WebPDecoderConfig::default();
        assert!(webp_init_decoder_config(&mut config));
        config.options = options.decoder_options();
        config.output.colorspace = WEBP_CSP_MODE(mode);
        set_geometry(&mut config.output, geometry, plane_slices(&mut blocks));
        writeln!(
            out,
            "valid={}",
            i32::from(webp_validate_decoder_config(&config))
        )
        .unwrap();
        let status = webp_decode(data, &mut config);
        writeln!(out, "status={}", code(status)).unwrap();
        print_features(out, "input", &config.input);
        print_buffer_fields(out, &config.output);
        if geometry.0.is_empty() {
            print_internal_memory(out, &config.output, false);
        }
    }
    if !geometry.0.is_empty() {
        print_blocks(out, &blocks);
    }
}

/// `Validate` of the probe.
pub fn validate(out: &mut String, mode: u32, options: Options, width: i32, height: i32) {
    let mut config = WebPDecoderConfig::default();
    assert!(webp_init_decoder_config(&mut config));
    config.options = options.decoder_options();
    config.output.colorspace = WEBP_CSP_MODE(mode);
    config.input.width = width;
    config.input.height = height;
    writeln!(
        out,
        "valid={}",
        i32::from(webp_validate_decoder_config(&config))
    )
    .unwrap();
}

/// `Simple` of the probe.
pub fn simple(out: &mut String, data: &[u8], func: &str) {
    let (mut width, mut height) = (-1, -1);
    if func == "YUV" {
        let buf = webp_decode_yuv(data, Some(&mut width), Some(&mut height));
        writeln!(out, "dims={width},{height}").unwrap();
        let Some(buf) = buf else {
            out.push_str("out=null\n");
            return;
        };
        let WebPDecBufferPlanes::YUVA(yuva) = buf.u else {
            panic!("YUVA member")
        };
        let PlaneStorage::Internal(memory) = &buf.storage else {
            panic!("library memory")
        };
        let uv_size = yuva.u_stride as usize * (height as usize).div_ceil(2);
        writeln!(
            out,
            "yuv={},{},{},{}",
            yuva.u.offset, yuva.v.offset, yuva.y_stride, yuva.u_stride
        )
        .unwrap();
        let y_size = yuva.y_stride as usize * height as usize;
        writeln!(out, "y={:016x}", digest(&memory[..y_size])).unwrap();
        writeln!(
            out,
            "u={:016x}",
            digest(&memory[yuva.u.offset..yuva.u.offset + uv_size])
        )
        .unwrap();
        writeln!(
            out,
            "v={:016x}",
            digest(&memory[yuva.v.offset..yuva.v.offset + uv_size])
        )
        .unwrap();
        return;
    }
    let (decoded, bpp) = match func {
        "RGB" => (
            webp_decode_rgb(data, Some(&mut width), Some(&mut height)),
            3,
        ),
        "RGBA" => (
            webp_decode_rgba(data, Some(&mut width), Some(&mut height)),
            4,
        ),
        "ARGB" => (
            webp_decode_argb(data, Some(&mut width), Some(&mut height)),
            4,
        ),
        "BGR" => (
            webp_decode_bgr(data, Some(&mut width), Some(&mut height)),
            3,
        ),
        "BGRA" => (
            webp_decode_bgra(data, Some(&mut width), Some(&mut height)),
            4,
        ),
        _ => panic!("unknown function {func}"),
    };
    writeln!(out, "dims={width},{height}").unwrap();
    match decoded {
        None => out.push_str("out=null\n"),
        Some(samples) => {
            let size = width as usize * height as usize * bpp;
            writeln!(out, "out={:016x}", digest(&samples[..size])).unwrap();
        }
    }
}

/// `Into` of the probe.
pub fn into(out: &mut String, data: &[u8], func: &str, geometry: &Geometry) {
    let mut blocks = geometry.blocks();
    let ok = {
        let mut slices = plane_slices(&mut blocks).into_iter();
        if func == "YUV" {
            let luma = slices.next().unwrap();
            let u = slices.next().unwrap();
            let v = slices.next().unwrap();
            let s = |i: usize| geometry.0[i].0;
            webp_decode_yuv_into(data, luma, s(0), u, s(1), v, s(2)).is_some()
        } else {
            let buf = slices.next().unwrap();
            let stride = geometry.0[0].0;
            match func {
                "RGB" => webp_decode_rgb_into(data, buf, stride),
                "RGBA" => webp_decode_rgba_into(data, buf, stride),
                "ARGB" => webp_decode_argb_into(data, buf, stride),
                "BGR" => webp_decode_bgr_into(data, buf, stride),
                "BGRA" => webp_decode_bgra_into(data, buf, stride),
                _ => panic!("unknown function {func}"),
            }
            .is_some()
        }
    };
    writeln!(out, "result={}", if ok { "buffer" } else { "null" }).unwrap();
    print_blocks(out, &blocks);
}

/// The offset of a plane that the incremental getters return.
fn rows_offset(rows: PlaneRows<'_>) -> i64 {
    rows.map_or(-1, |(_, offset)| offset as i64)
}

/// `PrintIncremental` of the probe.
fn print_incremental(
    out: &mut String,
    idec: &WebPIDecoder<'_, '_>,
    yuv: bool,
    external: bool,
    prev_last_y: &mut i32,
) {
    let (mut left, mut top, mut width, mut height) = (-1, -1, -1, -1);
    let area = webp_i_decoded_area(
        idec,
        Some(&mut left),
        Some(&mut top),
        Some(&mut width),
        Some(&mut height),
    );
    let colorspace = area.map(|area| area.colorspace);
    writeln!(
        out,
        "area={},{left},{top},{width},{height}",
        i32::from(area.is_some())
    )
    .unwrap();
    if yuv {
        let (mut last_y, mut w, mut h, mut stride, mut uv_stride, mut a_stride) =
            (-1, -1, -1, 0, 0, 0);
        let (mut u, mut v, mut a): (PlaneRows<'_>, PlaneRows<'_>, PlaneRows<'_>) =
            (None, None, None);
        let y = webp_i_dec_get_yuva(
            idec,
            Some(&mut last_y),
            Some(&mut u),
            Some(&mut v),
            Some(&mut a),
            Some(&mut w),
            Some(&mut h),
            Some(&mut stride),
            Some(&mut uv_stride),
            Some(&mut a_stride),
        );
        match y {
            None => writeln!(out, "yuva=null,{last_y},{w},{h}").unwrap(),
            Some((y_region, y_offset)) => {
                writeln!(
                    out,
                    "yuva={},{},{},{},{last_y},{w},{h},{stride},{uv_stride},{a_stride}",
                    y_offset,
                    rows_offset(u),
                    rows_offset(v),
                    rows_offset(a)
                )
                .unwrap();
                let print = |out: &mut String, key: &str, rows: PlaneRows<'_>, s, w, h| {
                    let (region, first) = rows.expect("the plane exists");
                    writeln!(out, "{key}={:016x}", rows_digest(region, first, s, w, h)).unwrap();
                };
                if last_y != *prev_last_y {
                    print(out, "y", Some((y_region, y_offset)), stride, w, last_y);
                    if external {
                        print(out, "u", u, uv_stride, (w + 1) / 2, (last_y + 1) / 2);
                        print(out, "v", v, uv_stride, (w + 1) / 2, (last_y + 1) / 2);
                        if a.is_some() {
                            print(out, "a", a, a_stride, w, last_y);
                        }
                    }
                    *prev_last_y = last_y;
                }
            }
        }
    } else {
        let (mut last_y, mut w, mut h, mut stride) = (-1, -1, -1, 0);
        match webp_i_dec_get_rgb(
            idec,
            Some(&mut last_y),
            Some(&mut w),
            Some(&mut h),
            Some(&mut stride),
        ) {
            None => writeln!(out, "rgb=null,{last_y},{w},{h},{stride}").unwrap(),
            Some((region, offset)) => {
                writeln!(out, "rgb={offset},{last_y},{w},{h},{stride}").unwrap();
                if last_y != *prev_last_y {
                    let bpp = MODE_BPP[colorspace.expect("the area exists").0 as usize] as i32;
                    writeln!(
                        out,
                        "rows={:016x}",
                        rows_digest(region, offset, stride, w * bpp, last_y)
                    )
                    .unwrap();
                    *prev_last_y = last_y;
                }
            }
        }
    }
}

/// `Incremental` of the probe.
#[expect(clippy::too_many_arguments, reason = "the fields of the command")]
pub fn incremental(
    out: &mut String,
    data: &[u8],
    chunk: usize,
    append: u32,
    kind: &str,
    mode: u32,
    options: Options,
    geometry: &Geometry,
) {
    let mut blocks = geometry.blocks();
    let mut config = WebPDecoderConfig::default();
    assert!(webp_init_decoder_config(&mut config));
    config.options = options.decoder_options();
    let mut library_memory_printed = false;
    let mut partial;
    {
        let slices = plane_slices(&mut blocks);
        let mut mode = mode;
        let idec = match kind {
            "config" | "fconfig" => {
                config.output.colorspace = WEBP_CSP_MODE(mode);
                set_geometry(&mut config.output, geometry, slices);
                let input: &[u8] = if kind == "fconfig" { data } else { &[] };
                webp_i_decode(input, Some(&mut config))
            }
            "new" => {
                mode = WEBP_CSP_MODE::MODE_RGB.0;
                webp_i_new_decoder(None)
            }
            "rgb" => {
                let stride = geometry.0.first().map_or(0, |g| g.0);
                webp_i_new_rgb(WEBP_CSP_MODE(mode), slices.into_iter().next(), stride)
            }
            "yuva" | "yuv" => {
                mode = WEBP_CSP_MODE::MODE_YUVA.0;
                let mut slices = slices.into_iter();
                let s = |i: usize| geometry.0.get(i).map_or(0, |g| g.0);
                let (y, u, v, a) = (slices.next(), slices.next(), slices.next(), slices.next());
                if kind == "yuv" {
                    webp_i_new_yuv(y, s(0), u, s(1), v, s(2))
                } else {
                    webp_i_new_yuva(y, s(0), u, s(1), v, s(2), a, s(3))
                }
            }
            _ => panic!("unknown kind {kind}"),
        };
        writeln!(out, "idec={}", i32::from(idec.is_some())).unwrap();
        let Some(mut idec) = idec else {
            return;
        };
        let yuv = !webp_is_rgb_mode(WEBP_CSP_MODE(mode));
        let mut pos = 0;
        let mut prev_last_y = -1;
        loop {
            let next = if data.len() - pos < chunk {
                data.len()
            } else {
                pos + chunk
            };
            let status = if append == 1 || (append == 2 && pos == 0) {
                webp_i_append(&mut idec, &data[pos..next])
            } else {
                let fresh = data[..next].to_vec();
                webp_i_update(&mut idec, &fresh)
            };
            pos = next;
            writeln!(out, "step={pos},{}", code(status)).unwrap();
            print_incremental(out, &idec, yuv, !geometry.0.is_empty(), &mut prev_last_y);
            partial = status.is_err();
            if status != Err(VP8StatusCode::VP8_STATUS_SUSPENDED) || pos == data.len() {
                break;
            }
        }
        if kind != "config" && kind != "fconfig" {
            match webp_i_decoded_area(&idec, None, None, None, None) {
                Some(area) => {
                    print_buffer_fields(out, area);
                    if geometry.0.is_empty() {
                        print_internal_memory(out, area, partial);
                        library_memory_printed = true;
                    }
                }
                None => library_memory_printed = geometry.0.is_empty(),
            }
        }
    }
    if kind == "config" || kind == "fconfig" {
        print_buffer_fields(out, &config.output);
        if geometry.0.is_empty() {
            print_internal_memory(out, &config.output, partial);
            return;
        }
    }
    drop(config);
    if !library_memory_printed {
        print_blocks(out, &blocks);
    }
}

/// The expected output dimensions of a decoding with `options`, or `None` when
/// the options are invalid for the image.
pub fn output_dimensions(width: i32, height: i32, options: &Options) -> Option<(i32, i32)> {
    let v = options.0;
    let (mut w, mut h) = (width, height);
    if v[Options::CROP] != 0 {
        w = v[Options::CROP_WIDTH];
        h = v[Options::CROP_HEIGHT];
    }
    if v[Options::SCALE] != 0 {
        let (mut sw, mut sh) = (v[Options::SCALED_WIDTH], v[Options::SCALED_HEIGHT]);
        if !webp::utils::rescaler_utils::webp_rescaler_get_scaled_dimensions(w, h, &mut sw, &mut sh)
        {
            return None;
        }
        w = sw;
        h = sh;
    }
    (w > 0 && h > 0).then_some((w, h))
}

/// Caller memory for an output of `width` by `height` in `mode`, with `pad`
/// bytes of padding per row; `alpha` adds an alpha plane in YUV.
pub fn geometry_for(mode: u32, width: i32, height: i32, pad: i32, alpha: bool) -> Geometry {
    let (w, h) = (width as usize, height as usize);
    if (mode as usize) < 11 {
        let stride = width * MODE_BPP[mode as usize] as i32 + pad;
        Geometry(
            vec![(
                stride,
                stride as usize * (h - 1) + w * MODE_BPP[mode as usize],
            )],
            false,
        )
    } else {
        let uv_w = w.div_ceil(2);
        let uv_h = h.div_ceil(2);
        let y_stride = width + pad;
        let uv_stride = uv_w as i32 + pad;
        let mut planes = vec![
            (y_stride, y_stride as usize * h),
            (uv_stride, uv_stride as usize * uv_h),
            (uv_stride, uv_stride as usize * uv_h),
        ];
        if alpha {
            planes.push((y_stride, y_stride as usize * h));
        }
        Geometry(planes, false)
    }
}

/// The test vectors without the two whose size makes a debug decode slow.
pub fn vectors() -> Vec<PathBuf> {
    crate::oracle::test_vectors()
        .into_iter()
        .filter(|path| {
            let name = path.file_name().unwrap().to_str().unwrap();
            name != "bryce.webp" && name != "lossless_big_random_alpha.webp"
        })
        .collect()
}

/// Compares the probe's output with the expected lines and panics at the
/// first difference, with the preceding lines for context.
pub fn compare(test: &str, actual: &[u8], expected: &str) {
    let actual = std::str::from_utf8(actual).expect("the probe prints UTF-8");
    if let Ok(dir) = std::env::var("WP12_DUMP") {
        std::fs::write(format!("{dir}/{test}.c.txt"), actual).unwrap();
        std::fs::write(format!("{dir}/{test}.rust.txt"), expected).unwrap();
    }
    let actual_lines: Vec<&str> = actual.lines().collect();
    let expected_lines: Vec<&str> = expected.lines().collect();
    for (index, (a, e)) in actual_lines.iter().zip(&expected_lines).enumerate() {
        if a != e {
            let start = index.saturating_sub(20);
            let context = actual_lines[start..index].join("\n");
            panic!(
                "{test}: line {index} differs\n--- context (C) ---\n{context}\n--- C ---\n{a}\n--- Rust ---\n{e}"
            );
        }
    }
    assert_eq!(
        actual_lines.len(),
        expected_lines.len(),
        "{test}: the C probe printed {} lines and the Rust translation {}",
        actual_lines.len(),
        expected_lines.len()
    );
}

/// A copy of a RIFF WebP file cut to `fraction` of its length with the RIFF size
/// and the size of the last chunk patched to match, so that the headers parse
/// and the decoding fails inside the bitstream, after the output buffer is
/// allocated. Written under `target/wp12-cut/`; returns its path.
pub fn consistent_cut(path: &Path, numerator: usize, denominator: usize) -> PathBuf {
    let data = std::fs::read(path).unwrap();
    let len = data.len() * numerator / denominator;
    let mut cut = data[..len].to_vec();
    let mut pos = 12;
    let mut last = None;
    while pos + 8 <= len {
        last = Some(pos);
        let size = u32::from_le_bytes(cut[pos + 4..pos + 8].try_into().unwrap()) as usize;
        pos += 8 + size + (size & 1);
    }
    if let Some(last) = last {
        let size = (len - last - 8) as u32;
        cut[last + 4..last + 8].copy_from_slice(&size.to_le_bytes());
        cut[4..8].copy_from_slice(&((len - 8) as u32).to_le_bytes());
    }
    let directory = crate::oracle::repository().join("target/wp12-cut");
    std::fs::create_dir_all(&directory).unwrap();
    let name = format!(
        "{}.{numerator}-{denominator}",
        path.file_name().unwrap().to_str().unwrap()
    );
    let output = directory.join(name);
    std::fs::write(&output, &cut).unwrap();
    output
}
