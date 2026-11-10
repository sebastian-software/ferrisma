//! WP10 differential test: the VP8 encoder core. The C probe
//! `tools/oracle/probes/wp10/encode.c` imports seeded RGB(A) pictures with the
//! oracle (`WebPPictureImportRGB`, `WebPPictureImportRGBA`, and the conversion and
//! transparent-area cleanup of `WebPEncode`), prints the YUV(A) planes that
//! `WebPEncode` encodes, and encodes them with `WebPEncode`. The Rust test runs
//! the encoder steps on those planes in the order `WebPEncode` calls them
//! (`InitVP8Encoder`, `VP8EncAnalyze`, `VP8EncStartAlpha`, `VP8EncLoop` or
//! `VP8EncTokenLoop`, `VP8EncFinishAlpha`, `VP8EncWrite`, `DeleteVP8Encoder`);
//! this driver and the statistics that `StoreStats` copies are test code, not a
//! library function (`WebPEncode` and `StoreStats` belong to WP13).
//!
//! Compared per case: the written bytes and the sizes of each writer call, the
//! return value, the error code, every progress report, the statistics, the
//! `extra_info` map and, with `show_compressed`, the planes after the call.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Allocation failures.** The size tests of `WebPSafeMalloc` are
//!    translated with the C totals, but no generated input exceeds
//!    `WEBP_MAX_ALLOCABLE_MEMORY` (that needs dimensions above
//!    `WEBP_MAX_DIMENSION`), and host allocation failures are not injected.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::cell::RefCell;
use std::fmt::Write as _;

use oracle::{Fields, Oracle};
use webp::enc::alpha_enc::{vp8_enc_finish_alpha, vp8_enc_start_alpha};
use webp::enc::analysis_enc::vp8_enc_analyze;
use webp::enc::config_enc::webp_validate_config;
use webp::enc::frame_enc::{vp8_enc_loop, vp8_enc_token_loop};
use webp::enc::picture_enc::webp_picture_alloc;
use webp::enc::syntax_enc::{vp8_enc_free_bit_writers, vp8_enc_write};
use webp::enc::vp8i_enc::VP8Encoder;
use webp::enc::webp_enc::{delete_vp8_encoder, init_vp8_encoder};
use webp::webp::encode::{
    WebPAuxStats, WebPConfig, WebPEncCSP, WebPEncodingError, WebPPicture, WebPPreset,
    webp_config_init, webp_config_preset, webp_picture_init,
};
use webp::webp::types::PlaneStorage;
use wp11_images::{Kind, Rng, image};

/// A configuration field the case leaves at the preset's value.
const KEEP: i32 = i32::MIN;

/// The configuration fields of a record, in the probe's order.
#[derive(Clone, Copy, Debug)]
#[repr(usize)]
enum F {
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
}

const NUM_FIELDS: usize = 24;

/// One encoding: a picture, the configuration, and the failure injections.
#[derive(Clone, Debug)]
struct Case {
    width: usize,
    height: usize,
    /// 0: RGB into YUV, 1: RGBA into YUV(A), 2: RGBA into ARGB.
    import: i32,
    /// `None` starts from `WebPConfigInit`.
    preset: Option<u32>,
    quality: f32,
    fields: [i32; NUM_FIELDS],
    /// Below -1: no progress hook; -1: never abort; else the failing call.
    abort_at: i32,
    fail_write_at: i32,
    with_stats: bool,
    extra_info_type: i32,
    /// RGBA words, R in the low byte.
    rgba: Vec<u32>,
}

impl Case {
    fn new(width: usize, height: usize, rgba: Vec<u32>) -> Case {
        Case {
            width,
            height,
            import: 1,
            preset: None,
            quality: 75.0,
            fields: [KEEP; NUM_FIELDS],
            abort_at: -1,
            fail_write_at: -1,
            with_stats: true,
            extra_info_type: 0,
            rgba,
        }
    }

    fn set(&mut self, field: F, value: i32) -> &mut Self {
        self.fields[field as usize] = value;
        self
    }

    fn get(&self, field: F) -> i32 {
        self.fields[field as usize]
    }

    /// The probe's binary record.
    fn record(&self, out: &mut Vec<u8>) {
        let mut values = vec![
            self.width as i32,
            self.height as i32,
            self.import,
            self.preset.map_or(-1, |preset| preset as i32),
            self.quality.to_bits() as i32,
        ];
        values.extend_from_slice(&self.fields);
        values.extend_from_slice(&[
            self.abort_at,
            self.fail_write_at,
            i32::from(self.with_stats),
            self.extra_info_type,
        ]);
        for value in values {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for &pixel in &self.rgba {
            out.extend_from_slice(&pixel.to_le_bytes());
        }
    }

    /// The configuration the probe builds.
    fn config(&self) -> WebPConfig {
        let mut config = WebPConfig::default();
        match self.preset {
            None => assert!(webp_config_init(&mut config)),
            Some(preset) => assert!(webp_config_preset(
                &mut config,
                WebPPreset(preset),
                self.quality
            )),
        }
        config.quality = self.quality;
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
        config
    }

    fn describe(&self) -> String {
        let names = [
            "method", "segments", "parts", "sns", "fstr", "sharp", "ftype", "auto", "pass",
            "tsize", "tpsnr", "jpeg", "lowmem", "threads", "sharpyuv", "pre", "aq", "afilt",
            "acomp", "exact", "plimit", "qmin", "qmax", "show",
        ];
        let mut text = format!(
            "{}x{} import {} preset {:?} q {} abort {} fail {} extra {}",
            self.width,
            self.height,
            self.import,
            self.preset,
            self.quality,
            self.abort_at,
            self.fail_write_at,
            self.extra_info_type
        );
        for (name, &value) in names.iter().zip(&self.fields) {
            if value != KEEP {
                write!(text, " {name} {value}").unwrap();
            }
        }
        text
    }
}

fn join(values: &[impl std::fmt::Display]) -> String {
    if values.is_empty() {
        return "-".to_owned();
    }
    values
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",")
}

fn unhex(text: &str) -> Vec<u8> {
    (0..text.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&text[i..i + 2], 16).expect("hex digits"))
        .collect()
}

fn stats_line(stats: &WebPAuxStats) -> String {
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

/// C's `GetPSNR` of `src/enc/webp_enc.c` (WP13), for the statistics.
fn get_psnr(err: u64, size: u64) -> f64 {
    if err > 0 && size > 0 {
        10. * (255. * 255. * size as f64 / err as f64).log10()
    } else {
        99.
    }
}

/// What C's `StoreStats` (WP13) copies from the encoder into the statistics.
fn store_stats(enc: &VP8Encoder, pic: &mut WebPPicture<'_>) {
    if let Some(stats) = pic.stats.as_deref_mut() {
        for i in 0..4 {
            stats.segment_level[i] = enc.dqm[i].fstrength;
            stats.segment_quant[i] = enc.dqm[i].quant;
            for s in 0..=2 {
                stats.residual_bytes[s][i] = enc.residual_bytes[s][i];
            }
        }
        let size = enc.sse_count;
        let sse = &enc.sse;
        stats.psnr[0] = get_psnr(sse[0], size) as f32;
        stats.psnr[1] = get_psnr(sse[1], size / 4) as f32;
        stats.psnr[2] = get_psnr(sse[2], size / 4) as f32;
        stats.psnr[3] = get_psnr(sse[0] + sse[1] + sse[2], size * 3 / 2) as f32;
        stats.psnr[4] = get_psnr(sse[3], size) as f32;
        stats.coded_size = enc.coded_size;
        stats.block_count = enc.block_count;
    }
}

/// Copies rows of `width` samples into a plane of the picture's storage.
fn fill_plane(
    storage: &mut PlaneStorage<'_, u8>,
    offset: usize,
    stride: i32,
    width: usize,
    rows: &[u8],
) {
    let PlaneStorage::Internal(samples) = storage else {
        panic!("the allocated picture owns its planes");
    };
    for (y, row) in rows.chunks(width).enumerate() {
        let start = offset + y * stride as usize;
        samples[start..start + width].copy_from_slice(row);
    }
}

/// The rows of `width` samples of a plane of the picture.
fn plane_rows(
    storage: &PlaneStorage<'_, u8>,
    offset: usize,
    stride: i32,
    width: usize,
    height: usize,
) -> Vec<u8> {
    let samples = storage.region(0).expect("an owned storage");
    let mut rows = Vec::new();
    for y in 0..height {
        let start = offset + y * stride as usize;
        rows.extend_from_slice(&samples[start..start + width]);
    }
    rows
}

/// Encodes `case` with the Rust translation on the planes that the probe
/// printed in `c`, and prints what the probe prints.
fn encode(case: &Case, c: &Fields) -> Fields {
    let config = case.config();
    let valid = webp_validate_config(&config);
    let mut fields = Fields::new().with("valid", i32::from(valid));
    if !valid {
        return fields;
    }
    let (width, height) = (case.width, case.height);
    let (uv_width, uv_height) = (width.div_ceil(2), height.div_ceil(2));
    let y = unhex(c.get("y").expect("y plane"));
    let u = unhex(c.get("u").expect("u plane"));
    let v = unhex(c.get("v").expect("v plane"));
    let a = match c.get("a").expect("a plane") {
        "-" => None,
        text => Some(unhex(text)),
    };

    let out = RefCell::new(Vec::<u8>::new());
    let sizes = RefCell::new(Vec::<usize>::new());
    let progress = RefCell::new(Vec::<i32>::new());
    let mut stats = WebPAuxStats::default();
    let mb_count = width.div_ceil(16) * height.div_ceil(16);
    let mut extra = vec![0xaa_u8; mb_count + 1];
    let mut shown = None;

    let (ret, error) = {
        let mut pic = WebPPicture::default();
        assert!(webp_picture_init(&mut pic));
        pic.use_argb = 0;
        pic.width = width as i32;
        pic.height = height as i32;
        pic.colorspace = if a.is_some() {
            WebPEncCSP::WEBP_YUV420A
        } else {
            WebPEncCSP::WEBP_YUV420
        };
        webp_picture_alloc(&mut pic).expect("the picture allocates");
        let (y_off, u_off, v_off) = (
            pic.y.unwrap().offset,
            pic.u.unwrap().offset,
            pic.v.unwrap().offset,
        );
        fill_plane(&mut pic.yuva, y_off, pic.y_stride, width, &y);
        fill_plane(&mut pic.yuva, u_off, pic.uv_stride, uv_width, &u);
        fill_plane(&mut pic.yuva, v_off, pic.uv_stride, uv_width, &v);
        if let Some(a) = &a {
            let a_off = pic.a.unwrap().offset;
            fill_plane(&mut pic.yuva, a_off, pic.a_stride, width, a);
        }

        let fail_write_at = case.fail_write_at;
        let sizes = &sizes;
        let out = &out;
        pic.writer = Some(Box::new(move |data: &[u8]| {
            let call = sizes.borrow().len();
            sizes.borrow_mut().push(data.len());
            if call as i32 == fail_write_at {
                return false;
            }
            out.borrow_mut().extend_from_slice(data);
            true
        }));
        if case.abort_at >= -1 {
            let abort_at = case.abort_at;
            let progress = &progress;
            pic.progress_hook = Some(Box::new(move |percent| {
                let call = progress.borrow().len();
                progress.borrow_mut().push(percent);
                call as i32 != abort_at
            }));
        }
        if case.with_stats {
            pic.stats = Some(&mut stats);
        }
        if case.extra_info_type > 0 {
            pic.extra_info = Some(extra.as_mut_slice());
            pic.extra_info_type = case.extra_info_type;
        }

        // WebPEncode: the lossy branch after the colorspace conversion.
        pic.error_code = WebPEncodingError::VP8_ENC_OK;
        if let Some(stats) = pic.stats.as_deref_mut() {
            *stats = WebPAuxStats::default();
        }
        let ok = match init_vp8_encoder(&config, &mut pic) {
            Err(_) => false,
            Ok(mut enc) => {
                let mut ok = vp8_enc_analyze(&mut enc, &config, &mut pic).is_ok();
                ok = ok && vp8_enc_start_alpha(&mut enc, &config, &mut pic);
                if enc.use_tokens {
                    ok = ok && vp8_enc_token_loop(&mut enc, &config, &mut pic).is_ok();
                } else {
                    ok = ok && vp8_enc_loop(&mut enc, &config, &mut pic).is_ok();
                }
                ok = ok && vp8_enc_finish_alpha(&mut enc, &mut pic);
                ok = ok && vp8_enc_write(&mut enc, &mut pic).is_ok();
                store_stats(&enc, &mut pic);
                if !ok {
                    vp8_enc_free_bit_writers(&mut enc);
                }
                ok &= delete_vp8_encoder(enc, &mut pic);
                ok
            }
        };
        if config.show_compressed != 0 {
            shown = Some((
                plane_rows(
                    &pic.yuva,
                    pic.y.unwrap().offset,
                    pic.y_stride,
                    width,
                    height,
                ),
                plane_rows(
                    &pic.yuva,
                    pic.u.unwrap().offset,
                    pic.uv_stride,
                    uv_width,
                    uv_height,
                ),
                plane_rows(
                    &pic.yuva,
                    pic.v.unwrap().offset,
                    pic.uv_stride,
                    uv_width,
                    uv_height,
                ),
            ));
        }
        (ok, pic.error_code.0)
    };

    fields = fields.hex("y", &y).hex("u", &u).hex("v", &v);
    fields = match &a {
        None => fields.with("a", "-"),
        Some(a) => fields.hex("a", a),
    };
    fields = fields
        .with("ret", i32::from(ret))
        .with("error", error)
        .with("progress", join(&progress.borrow()))
        .with("writes", join(&sizes.borrow()))
        .hex("out", &out.borrow());
    if case.with_stats {
        fields = fields.with("stats", stats_line(&stats));
    }
    if case.extra_info_type > 0 {
        fields = fields.hex("extra", &extra);
    }
    if let Some((y, u, v)) = shown {
        fields = fields
            .hex("shown_y", &y)
            .hex("shown_u", &u)
            .hex("shown_v", &v);
    }
    fields
}

/// An RGBA word from an ARGB word of the WP11 generators.
fn rgba_of(argb: u32) -> u32 {
    let a = argb >> 24;
    let r = (argb >> 16) & 0xff;
    let g = (argb >> 8) & 0xff;
    let b = argb & 0xff;
    r | (g << 8) | (b << 16) | (a << 24)
}

/// A seeded RGBA picture of `kind`; `opaque` forces alpha to 255.
fn picture(rng: &mut Rng, kind: Kind, width: usize, height: usize, opaque: bool) -> Vec<u32> {
    image(rng, kind, width, height, width)
        .into_iter()
        .map(|argb| {
            let rgba = rgba_of(argb);
            if opaque { rgba | 0xff00_0000 } else { rgba }
        })
        .collect()
}

/// Runs the cases through the probe and the Rust translation and compares them.
fn compare(test: &str, cases: &[Case]) {
    let Some(oracle) = Oracle::locate(test) else {
        return;
    };
    let mut stdin = Vec::new();
    for case in cases {
        case.record(&mut stdin);
    }
    let output = oracle.probe("wp10", "encode").run::<&str>(&[], &stdin);
    let actual = output.success().fields();
    let mut groups: Vec<Fields> = Vec::new();
    for (key, value) in actual.0 {
        if key == "valid" {
            groups.push(Fields::new());
        }
        groups
            .last_mut()
            .expect("valid= first")
            .0
            .push((key, value));
    }
    assert_eq!(groups.len(), cases.len(), "number of cases");
    let mut mismatches = 0;
    for (index, (case, c)) in cases.iter().zip(&groups).enumerate() {
        let rust = encode(case, c);
        if &rust != c {
            mismatches += 1;
            if mismatches <= 6 {
                eprintln!("case {index}: {}", case.describe());
                for ((key, value_c), (_, value_rust)) in c.0.iter().zip(&rust.0) {
                    if value_c != value_rust {
                        let cut = |v: &str| v.chars().take(200).collect::<String>();
                        eprintln!(
                            "  {key}: C    {}\n  {key}: Rust {}",
                            cut(value_c),
                            cut(value_rust)
                        );
                    }
                }
                if c.0.len() != rust.0.len() {
                    eprintln!("  keys: C {:?} Rust {:?}", c.keys(), rust.keys());
                }
            }
        }
    }
    assert_eq!(
        mismatches,
        0,
        "{mismatches} of {} cases differ",
        cases.len()
    );
}

const QUALITIES: [f32; 10] = [0.0, 5.0, 25.0, 50.0, 70.0, 75.0, 90.0, 98.0, 99.0, 100.0];

#[test]
fn methods_and_qualities_match_the_c_encoder() {
    let mut rng = Rng(0x0010_e0c0_de10);
    let mut cases = Vec::new();
    let pictures = [
        (Kind::Photo, 48, 40),
        (Kind::Noise, 33, 17),
        (Kind::Flat, 40, 24),
        (Kind::Stripes, 17, 35),
    ];
    for &(kind, width, height) in &pictures {
        let rgba = picture(&mut rng, kind, width, height, true);
        for method in 0..=6 {
            for &quality in &QUALITIES {
                if (method + quality as i32) % 2 == 1 && kind != Kind::Photo {
                    continue;
                }
                let mut case = Case::new(width, height, rgba.clone());
                case.quality = quality;
                case.set(F::Method, method);
                case.import = method % 2;
                cases.push(case);
            }
        }
    }
    // Tiny pictures, every method.
    for &(width, height) in &[(1, 1), (2, 3), (5, 1), (16, 16), (17, 1), (1, 18)] {
        let rgba = picture(&mut rng, Kind::Photo, width, height, true);
        for method in 0..=6 {
            let mut case = Case::new(width, height, rgba.clone());
            case.set(F::Method, method);
            case.quality = QUALITIES[rng.below(10) as usize];
            cases.push(case);
        }
    }
    // The fast analysis of methods 0 and 1 compares with 8 + 9 * quality / 100,
    // which changes between the qualities 11 and 12, 22 and 23, and so on.
    for &kind in &[Kind::Photo, Kind::Stripes, Kind::Flat, Kind::Gray] {
        let rgba = picture(&mut rng, kind, 64, 48, true);
        for method in 0..=1 {
            for &quality in &[11.0, 12.0, 22.0, 23.0, 44.0, 45.0, 99.0] {
                let mut case = Case::new(64, 48, rgba.clone());
                case.quality = quality;
                case.set(F::Method, method);
                cases.push(case);
            }
        }
    }
    compare("methods_and_qualities_match_the_c_encoder", &cases);
}

#[test]
fn presets_and_tools_match_the_c_encoder() {
    let mut rng = Rng(0x0010_7001_5eed);
    let mut cases = Vec::new();
    let photo = picture(&mut rng, Kind::Photo, 40, 36, true);
    // The presets at several qualities.
    for preset in 0..=5 {
        for &quality in &[10.0, 60.0, 95.0] {
            for method in [1, 4] {
                let mut case = Case::new(40, 36, photo.clone());
                case.preset = Some(preset);
                case.quality = quality;
                case.set(F::Method, method);
                cases.push(case);
            }
        }
    }
    // Segments and partitions.
    for segments in 1..=4 {
        for partitions in 0..=3 {
            for method in [0, 2, 4, 6] {
                let mut case = Case::new(40, 36, photo.clone());
                case.set(F::Segments, segments)
                    .set(F::Partitions, partitions)
                    .set(F::Method, method)
                    .set(F::LowMemory, i32::from(partitions % 2 == 1));
                case.quality = 30.0 + 20.0 * segments as f32;
                cases.push(case);
            }
        }
    }
    // SNS, filter strength, sharpness, type, autofilter.
    for sns in [0, 25, 80, 100] {
        for strength in [0, 30, 100] {
            let mut case = Case::new(40, 36, photo.clone());
            case.set(F::Sns, sns)
                .set(F::FilterStrength, strength)
                .set(F::FilterSharpness, rng.below(8) as i32)
                .set(F::FilterType, rng.below(2) as i32)
                .set(F::Method, rng.below(7) as i32);
            cases.push(case);
        }
    }
    for autofilter in 0..=1 {
        for sharpness in [0, 3, 7] {
            for filter_type in 0..=1 {
                let mut case = Case::new(40, 36, photo.clone());
                case.set(F::Autofilter, autofilter)
                    .set(F::FilterSharpness, sharpness)
                    .set(F::FilterType, filter_type)
                    .set(F::Method, 2 + 2 * filter_type);
                cases.push(case);
            }
        }
    }
    // Passes with size and PSNR targets, in both loops.
    for &(target_size, target_psnr) in &[
        (0, 0.0_f32),
        (1500, 0.0),
        (400, 0.0),
        (100_000, 0.0),
        (0, 30.0),
        (0, 42.0),
        (0, 60.0),
    ] {
        for pass in [1, 3, 10] {
            for low_memory in 0..=1 {
                let mut case = Case::new(40, 36, photo.clone());
                case.set(F::TargetSize, target_size)
                    .set(F::TargetPsnr, target_psnr.to_bits() as i32)
                    .set(F::Pass, pass)
                    .set(F::LowMemory, low_memory)
                    .set(F::Method, if low_memory == 0 { 4 } else { 2 });
                cases.push(case);
            }
        }
    }
    // qmin, qmax and the partition limit.
    for &(qmin, qmax) in &[(0, 100), (20, 60), (70, 70)] {
        for limit in [0, 50, 100] {
            let mut case = Case::new(40, 36, photo.clone());
            case.set(F::Qmin, qmin)
                .set(F::Qmax, qmax)
                .set(F::PartitionLimit, limit)
                .set(F::Pass, 2)
                .set(F::TargetSize, 800)
                .set(F::Method, 3);
            cases.push(case);
        }
    }
    // emulate_jpeg_size, preprocessing and sharp YUV, from YUV and ARGB input.
    for jpeg in 0..=1 {
        for preprocessing in 0..=7 {
            for import in [1, 2] {
                let mut case = Case::new(40, 36, photo.clone());
                case.import = import;
                case.set(F::EmulateJpegSize, jpeg)
                    .set(F::Preprocessing, preprocessing)
                    .set(F::Method, rng.below(7) as i32)
                    .set(F::Segments, 1 + rng.below(4) as i32);
                cases.push(case);
            }
        }
    }
    for use_sharp_yuv in 0..=1 {
        let mut case = Case::new(40, 36, photo.clone());
        case.import = 2;
        case.set(F::UseSharpYuv, use_sharp_yuv);
        cases.push(case);
    }
    compare("presets_and_tools_match_the_c_encoder", &cases);
}

#[test]
fn threads_and_side_outputs_match_the_c_encoder() {
    let mut rng = Rng(0x0010_7412_ead5);
    let mut cases = Vec::new();
    // thread_level 1 splits the analysis from two macroblock rows on.
    for &(width, height) in &[(40, 16), (40, 32), (50, 70), (100, 60)] {
        for &kind in &[Kind::Photo, Kind::Noise] {
            let rgba = picture(&mut rng, kind, width, height, true);
            for method in [0, 1, 3, 5] {
                for thread_level in 0..=1 {
                    let mut case = Case::new(width, height, rgba.clone());
                    case.set(F::ThreadLevel, thread_level)
                        .set(F::Method, method)
                        .set(F::Segments, 4)
                        .set(F::Preprocessing, 1);
                    cases.push(case);
                }
            }
        }
    }
    // show_compressed and every extra_info type. Type 6 reads the bit counts
    // that only VP8EncLoop sets; the token loop leaves them uninitialized in C.
    let noise = picture(&mut rng, Kind::Noise, 35, 33, true);
    let photo = picture(&mut rng, Kind::Photo, 35, 33, true);
    for extra_info_type in 1..=8 {
        for method in [1, 4] {
            if extra_info_type == 6 && method >= 3 {
                continue;
            }
            let rgba = if extra_info_type == 6 { &noise } else { &photo };
            let mut case = Case::new(35, 33, rgba.clone());
            case.extra_info_type = extra_info_type;
            case.set(F::Method, method)
                .set(F::ShowCompressed, i32::from(method == 4));
            cases.push(case);
        }
    }
    compare("threads_and_side_outputs_match_the_c_encoder", &cases);
}

#[test]
fn alpha_matches_the_c_encoder() {
    let mut rng = Rng(0x0010_a1fa_0000);
    let mut cases = Vec::new();
    let kinds = [
        Kind::Transparent,
        Kind::Palette { n: 4, zero: true },
        Kind::Palette {
            n: 200,
            zero: false,
        },
        Kind::Noise,
    ];
    for (k, &kind) in kinds.iter().enumerate() {
        let (width, height) = [(40, 36), (17, 33), (64, 40), (23, 9)][k];
        let rgba = picture(&mut rng, kind, width, height, false);
        for alpha_compression in 0..=1 {
            for alpha_filtering in 0..=2 {
                for &alpha_quality in &[0, 30, 70, 90, 100] {
                    let mut case = Case::new(width, height, rgba.clone());
                    case.set(F::AlphaCompression, alpha_compression)
                        .set(F::AlphaFiltering, alpha_filtering)
                        .set(F::AlphaQuality, alpha_quality)
                        .set(F::Exact, rng.below(2) as i32)
                        .set(F::ThreadLevel, rng.below(2) as i32)
                        .set(F::Method, [0, 3, 4, 6][rng.below(4) as usize]);
                    case.import = 1 + rng.below(2) as i32;
                    // Without compression C copies uninitialized lossless
                    // statistics into the picture (see `src/enc/alpha_enc.rs`,
                    // `# Excluded undefined behavior`, item 1).
                    case.with_stats = alpha_compression == 1;
                    cases.push(case);
                }
            }
        }
    }
    // A smooth alpha gradient with every level, at the qualities above 70 that
    // keep 16 + (quality - 70) * 8 levels.
    let (width, height) = (64, 48);
    let mut rgba = picture(&mut rng, Kind::Photo, width, height, true);
    for (i, pixel) in rgba.iter_mut().enumerate() {
        let (x, y) = (i % width, i / width);
        let level = ((x * 4 + y * 3) % 256) as u32;
        *pixel = (*pixel & 0x00ff_ffff) | (level << 24);
    }
    for &alpha_quality in &[71, 80, 90, 95, 99] {
        for alpha_filtering in 0..=2 {
            let mut case = Case::new(width, height, rgba.clone());
            case.set(F::AlphaQuality, alpha_quality)
                .set(F::AlphaFiltering, alpha_filtering)
                .set(F::AlphaCompression, 1)
                .set(F::Method, 4);
            cases.push(case);
        }
    }
    compare("alpha_matches_the_c_encoder", &cases);
}

#[test]
fn aborts_and_write_failures_match_the_c_encoder() {
    let mut rng = Rng(0x0010_ab0e_7001);
    let mut cases = Vec::new();
    let opaque = picture(&mut rng, Kind::Photo, 40, 34, true);
    let transparent = picture(&mut rng, Kind::Transparent, 40, 34, false);
    // The progress calls of an encoding: abort at each of them.
    for (rgba, method, low_memory) in [
        (&opaque, 4, 0),
        (&opaque, 2, 0),
        (&transparent, 4, 0),
        (&transparent, 6, 1),
    ] {
        for abort_at in 0..14 {
            for thread_level in 0..=1 {
                let mut case = Case::new(40, 34, rgba.clone());
                case.abort_at = abort_at;
                case.set(F::Method, method)
                    .set(F::LowMemory, low_memory)
                    .set(F::ThreadLevel, thread_level)
                    .set(F::Pass, 2)
                    .set(F::Partitions, 2);
                // A failed threaded encoding with alpha deletes the encoder
                // while C's worker may still write the statistics.
                case.with_stats = thread_level == 0 || std::ptr::eq(rgba, &opaque);
                cases.push(case);
            }
        }
    }
    // A writer failure at each call, and no progress hook.
    for (rgba, partitions) in [(&opaque, 0), (&opaque, 3), (&transparent, 1)] {
        for fail_write_at in 0..10 {
            let mut case = Case::new(40, 34, rgba.clone());
            case.fail_write_at = fail_write_at;
            case.abort_at = -2;
            case.set(F::Partitions, partitions)
                .set(F::LowMemory, 1)
                .set(F::Method, 3);
            cases.push(case);
        }
    }
    compare("aborts_and_write_failures_match_the_c_encoder", &cases);
}

#[test]
fn random_configurations_match_the_c_encoder() {
    let mut rng = Rng(0x0010_4a4d_0c0f);
    let mut cases = Vec::new();
    let sizes = [
        (1, 1),
        (3, 2),
        (9, 30),
        (16, 16),
        (31, 17),
        (32, 32),
        (47, 33),
        (64, 48),
        (80, 20),
        (21, 70),
    ];
    for index in 0..240 {
        let (width, height) = sizes[rng.below(sizes.len() as u64) as usize];
        let kind = Kind::ALL[rng.below(Kind::ALL.len() as u64) as usize];
        let opaque = rng.below(3) != 0;
        let rgba = picture(&mut rng, kind, width, height, opaque);
        let mut case = Case::new(width, height, rgba);
        case.import = rng.below(3) as i32;
        case.preset = (rng.below(3) == 0).then(|| rng.below(6) as u32);
        case.quality = if rng.below(2) == 0 {
            QUALITIES[rng.below(10) as usize]
        } else {
            rng.below(10_001) as f32 / 100.0
        };
        let pick = |rng: &mut Rng, values: &[i32]| values[rng.below(values.len() as u64) as usize];
        case.set(F::Method, rng.below(7) as i32);
        if rng.below(2) == 0 {
            case.set(F::Segments, 1 + rng.below(4) as i32);
        }
        if rng.below(2) == 0 {
            case.set(F::Partitions, rng.below(4) as i32);
        }
        if rng.below(2) == 0 {
            case.set(F::Sns, pick(&mut rng, &[0, 1, 30, 50, 99, 100]));
        }
        if rng.below(2) == 0 {
            case.set(F::FilterStrength, pick(&mut rng, &[0, 5, 40, 100]));
            case.set(F::FilterSharpness, rng.below(8) as i32);
            case.set(F::FilterType, rng.below(2) as i32);
        }
        if rng.below(3) == 0 {
            case.set(F::Autofilter, 1);
        }
        if rng.below(3) == 0 {
            case.set(F::Pass, 1 + rng.below(10) as i32);
            match rng.below(3) {
                0 => {
                    case.set(F::TargetSize, pick(&mut rng, &[50, 200, 1000, 5000]));
                }
                1 => {
                    let psnr = pick(&mut rng, &[25, 35, 40, 50]) as f32;
                    case.set(F::TargetPsnr, psnr.to_bits() as i32);
                }
                _ => {}
            }
        }
        if rng.below(4) == 0 {
            case.set(F::EmulateJpegSize, 1);
        }
        if rng.below(4) == 0 {
            case.set(F::LowMemory, 1);
        }
        case.set(F::ThreadLevel, rng.below(2) as i32);
        if rng.below(4) == 0 {
            case.set(F::UseSharpYuv, 1);
        }
        if rng.below(3) == 0 {
            case.set(F::Preprocessing, rng.below(8) as i32);
        }
        if !opaque {
            case.set(
                F::AlphaQuality,
                pick(&mut rng, &[0, 10, 50, 70, 71, 95, 100]),
            );
            case.set(F::AlphaFiltering, rng.below(3) as i32);
            case.set(F::AlphaCompression, rng.below(2) as i32);
        }
        case.set(F::Exact, rng.below(2) as i32);
        if rng.below(4) == 0 {
            case.set(F::PartitionLimit, rng.below(101) as i32);
        }
        if rng.below(5) == 0 {
            let qmin = rng.below(60) as i32;
            case.set(F::Qmin, qmin)
                .set(F::Qmax, qmin + rng.below(41) as i32);
        }
        case.set(F::ShowCompressed, i32::from(rng.below(5) == 0));
        case.extra_info_type = if rng.below(4) == 0 {
            [1, 2, 3, 4, 5, 7][rng.below(6) as usize]
        } else {
            0
        };
        // See alpha_matches_the_c_encoder for the uninitialized statistics.
        let alpha_uncompressed = !opaque && case.get(F::AlphaCompression) == 0;
        case.with_stats = !alpha_uncompressed && index % 3 != 0;
        cases.push(case);
    }
    compare("random_configurations_match_the_c_encoder", &cases);
}

#[test]
fn large_and_edge_pictures_match_the_c_encoder() {
    let mut rng = Rng(0x0010_1a29_e000);
    let mut cases = Vec::new();
    // More than 768 macroblocks: the token loop refreshes its probabilities
    // every mb_count / 8 macroblocks instead of every MIN_COUNT.
    for &(kind, width, height, method) in &[
        (Kind::Photo, 512, 400, 4),
        (Kind::Stripes, 480, 448, 5),
        (Kind::Noise, 640, 32, 6),
    ] {
        let rgba = picture(&mut rng, kind, width, height, true);
        let mut case = Case::new(width, height, rgba);
        case.set(F::Method, method).set(F::Segments, 4);
        case.quality = 60.0;
        cases.push(case);
    }
    // Alpha planes with exactly 15, 16 and 17 levels: FAST filtering skips the
    // filter estimate up to 16 levels.
    for levels in [15_u32, 16, 17] {
        let (width, height) = (40, 30);
        let mut rgba = picture(&mut rng, Kind::Photo, width, height, true);
        for (i, pixel) in rgba.iter_mut().enumerate() {
            let level = ((i as u32 * 7 + (i as u32 / width as u32) * 3) % levels) * 255 / levels;
            *pixel = (*pixel & 0x00ff_ffff) | (level << 24);
        }
        for effort in [2, 5] {
            let mut case = Case::new(width, height, rgba.clone());
            case.set(F::AlphaFiltering, 1)
                .set(F::AlphaQuality, 100)
                .set(F::AlphaCompression, 1)
                .set(F::Method, effort);
            cases.push(case);
        }
    }
    // Noise with one flat macroblock among 43, 42 and 44: one skipped block
    // gives a skip probability just below or at the threshold of 250.
    for count in [42_usize, 43, 44, 51, 52] {
        let (width, height) = (16 * count, 16);
        let mut rgba = picture(&mut rng, Kind::Noise, width, height, true);
        for y in 0..16 {
            for x in 0..16 {
                // RGB 130 converts to Y = U = V = 128, the DC prediction of the
                // first macroblock, so the block has no residual.
                rgba[y * width + x] = 0xff82_8282;
            }
        }
        // Methods 0 to 2 with one segment leave the skip probability unset:
        // their header cost is 0, and StatLoop stops early (see the status
        // page). Methods 3 and up with low_memory, and two segments, reach it.
        for &(method, segments, low_memory) in &[(0, 1, 0), (2, 2, 0), (3, 1, 1), (5, 1, 1)] {
            for &quality in &[20.0, 50.0, 80.0] {
                let mut case = Case::new(width, height, rgba.clone());
                case.set(F::Method, method)
                    .set(F::Segments, segments)
                    .set(F::LowMemory, low_memory);
                case.quality = quality;
                cases.push(case);
            }
        }
    }
    // Mostly smooth pictures with a few noisy macroblocks: the susceptibilities
    // crowd at one end, and the k-means centers move for several iterations.
    for &(width, height, noisy) in &[(256, 256, 9_usize), (320, 160, 23), (192, 192, 50)] {
        let mut rgba = picture(&mut rng, Kind::Photo, width, height, true);
        let mb_w = width / 16;
        for k in 0..noisy {
            let mb = (k * 7919) % (mb_w * (height / 16));
            let (mx, my) = (mb % mb_w, mb / mb_w);
            for y in 0..16 {
                for x in 0..16 {
                    rgba[(my * 16 + y) * width + mx * 16 + x] = rng.u32() | 0xff00_0000;
                }
            }
        }
        for method in [2, 4, 6] {
            let mut case = Case::new(width, height, rgba.clone());
            case.set(F::Method, method)
                .set(F::Segments, 4)
                .set(F::Sns, 100);
            cases.push(case);
        }
    }
    compare("large_and_edge_pictures_match_the_c_encoder", &cases);
}
