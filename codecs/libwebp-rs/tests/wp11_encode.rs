//! WP11 differential test: `VP8LEncodeImage` (`src/enc/vp8l_enc.c`), which
//! writes the complete RIFF file through the picture's writer. The C probe
//! `tools/oracle/probes/wp11/encode.c` and the Rust translation encode the same
//! seeded ARGB pictures under the same configurations; the written bytes, the
//! return value, the error code, every progress report, every writer call, the
//! lossless statistics and the cleared `extra_info` are compared.
//!
//! The configurations cover `quality`, `method` 0 to 6, `exact`,
//! `near_lossless`, `use_delta_palette`, `thread_level` 0 and 1 and the image
//! hint, on pictures of every generated kind, from 1 by 1 to 160 by 90 pixels
//! with padded rows. Further cases abort from the progress hook at each reported
//! percent, fail the writer at each call, and pass a picture without an ARGB
//! plane.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::cell::RefCell;
use std::fmt::Write as _;

use oracle::{Fields, Oracle};
use webp::enc::vp8l_enc::vp8l_encode_image;
use webp::webp::encode::{
    WebPAuxStats, WebPConfig, WebPImageHint, WebPPicture, webp_config_init, webp_picture_init,
};
use webp::webp::types::{PlaneOffset, PlaneStorage};
use wp11_images::{Kind, Rng, image};

/// One encoding: a picture, the configuration fields the probe sets, and the
/// failure injections.
#[derive(Clone, Debug)]
struct Case {
    width: usize,
    height: usize,
    stride: usize,
    argb: Vec<u32>,
    quality: f32,
    method: i32,
    exact: i32,
    near_lossless: i32,
    use_delta_palette: i32,
    thread_level: i32,
    image_hint: u32,
    abort_at: i32,
    fail_write_at: i32,
    with_stats: bool,
    with_extra_info: bool,
    null_argb: bool,
    /// Cases of one group must produce identical files and statistics.
    group: Option<usize>,
}

impl Case {
    fn new(width: usize, height: usize, stride: usize, argb: Vec<u32>) -> Case {
        Case {
            width,
            height,
            stride,
            argb,
            quality: 75.0,
            method: 4,
            exact: 0,
            near_lossless: 100,
            use_delta_palette: 0,
            thread_level: 0,
            image_hint: 0,
            abort_at: -1,
            fail_write_at: -1,
            with_stats: false,
            with_extra_info: false,
            null_argb: false,
            group: None,
        }
    }

    /// The probe's binary record.
    fn record(&self, out: &mut Vec<u8>) {
        let fields = [
            self.width as i32,
            self.height as i32,
            self.stride as i32,
            self.quality.to_bits() as i32,
            self.method,
            self.exact,
            self.near_lossless,
            self.use_delta_palette,
            self.thread_level,
            self.image_hint as i32,
            self.abort_at,
            self.fail_write_at,
            i32::from(self.with_stats),
            i32::from(self.with_extra_info),
            i32::from(self.null_argb),
        ];
        for value in fields {
            out.extend_from_slice(&value.to_le_bytes());
        }
        for &pixel in &self.argb {
            out.extend_from_slice(&pixel.to_le_bytes());
        }
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

/// The statistics with every byte 0xa5, as the probe's `memset` leaves them.
fn poisoned_stats() -> WebPAuxStats {
    let i = 0xa5a5_a5a5_u32 as i32;
    let f = f32::from_bits(0xa5a5_a5a5);
    WebPAuxStats {
        coded_size: i,
        psnr: [f; 5],
        block_count: [i; 3],
        header_bytes: [i; 2],
        residual_bytes: [[i; 4]; 3],
        segment_size: [i; 4],
        segment_quant: [i; 4],
        segment_level: [i; 4],
        alpha_data_size: i,
        layer_data_size: i,
        lossless_features: 0xa5a5_a5a5,
        histogram_bits: i,
        transform_bits: i,
        cache_bits: i,
        palette_size: i,
        lossless_size: i,
        lossless_hdr_size: i,
        lossless_data_size: i,
        cross_color_transform_bits: i,
        pad: [0xa5a5_a5a5],
    }
}

fn stats_line(stats: &WebPAuxStats) -> String {
    let mut line = format!("{}", stats.coded_size);
    for value in stats.psnr {
        write!(line, ",{:08x}", value.to_bits()).unwrap();
    }
    for value in stats.block_count {
        write!(line, ",{value}").unwrap();
    }
    for value in stats.header_bytes {
        write!(line, ",{value}").unwrap();
    }
    for row in stats.residual_bytes {
        for value in row {
            write!(line, ",{value}").unwrap();
        }
    }
    for value in stats
        .segment_size
        .iter()
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

/// Encodes `case` with the Rust translation and prints what the probe prints.
fn encode(case: &Case) -> Fields {
    let mut pixels = case.argb.clone();
    let out = RefCell::new(Vec::<u8>::new());
    let sizes = RefCell::new(Vec::<usize>::new());
    let progress = RefCell::new(Vec::<i32>::new());
    let mut stats = poisoned_stats();
    let extra_size = ((case.width + 15) >> 4) * ((case.height + 15) >> 4);
    let mut extra = vec![0xaa_u8; extra_size + 1];

    let mut config = WebPConfig::default();
    assert!(webp_config_init(&mut config));
    config.lossless = 1;
    config.quality = case.quality;
    config.method = case.method;
    config.exact = case.exact;
    config.near_lossless = case.near_lossless;
    config.use_delta_palette = case.use_delta_palette;
    config.thread_level = case.thread_level;
    config.image_hint = WebPImageHint(case.image_hint);

    let result = {
        let mut picture = WebPPicture::default();
        assert!(webp_picture_init(&mut picture));
        picture.use_argb = 1;
        picture.width = case.width as i32;
        picture.height = case.height as i32;
        picture.argb = (!case.null_argb).then_some(PlaneOffset {
            region: 0,
            offset: 0,
        });
        picture.argb_stride = case.stride as i32;
        picture.argb_storage = PlaneStorage::External(vec![pixels.as_mut_slice()]);
        let fail_write_at = case.fail_write_at;
        let sizes = &sizes;
        let out = &out;
        picture.writer = Some(Box::new(move |data: &[u8]| {
            let call = sizes.borrow().len();
            sizes.borrow_mut().push(data.len());
            if call as i32 == fail_write_at {
                return false;
            }
            out.borrow_mut().extend_from_slice(data);
            true
        }));
        if case.abort_at >= 0 {
            let abort_at = case.abort_at;
            let progress = &progress;
            picture.progress_hook = Some(Box::new(move |percent| {
                progress.borrow_mut().push(percent);
                percent < abort_at
            }));
        }
        if case.with_stats {
            picture.stats = Some(&mut stats);
        }
        if case.with_extra_info {
            picture.extra_info = Some(extra.as_mut_slice());
        }
        let result = vp8l_encode_image(&config, &mut picture);
        (result.is_ok(), picture.error_code.0)
    };

    let mut fields = Fields::new()
        .with("ret", i32::from(result.0))
        .with("error", result.1)
        .with("progress", join(&progress.borrow()))
        .with("writes", join(&sizes.borrow()))
        .hex("out", &out.borrow());
    if case.with_stats {
        fields = fields.with("stats", stats_line(&stats));
    }
    if case.with_extra_info {
        fields = fields.hex("extra", &extra);
    }
    fields
}

/// The seeded cases.
fn cases() -> Vec<Case> {
    let mut rng = Rng(0x5711_e0c0_de11);
    let mut cases = Vec::new();
    let sizes: [(usize, usize); 14] = [
        (1, 1),
        (1, 9),
        (13, 1),
        (2, 2),
        (3, 5),
        (8, 8),
        (17, 13),
        (31, 33),
        (40, 40),
        (64, 66),
        (70, 64),
        (100, 20),
        (160, 90),
        (90, 128),
    ];
    // Every kind at every size, with the default configuration and a random
    // method, quality and flags.
    for (index, &(width, height)) in sizes.iter().enumerate() {
        for (k, &kind) in Kind::ALL.iter().enumerate() {
            if width * height > 6000 && (index + k) % 3 != 0 {
                continue;
            }
            let stride = width + rng.below(3) as usize;
            let argb = image(&mut rng, kind, width, height, stride);
            let mut case = Case::new(width, height, stride, argb);
            case.method = rng.below(7) as i32;
            case.quality = [
                0.0, 10.0, 25.0, 26.0, 50.0, 51.0, 74.0, 75.0, 76.0, 90.0, 99.0, 100.0,
            ][rng.below(12) as usize];
            case.exact = rng.below(2) as i32;
            case.near_lossless = [100, 100, 99, 80, 60, 40, 20, 0][rng.below(8) as usize];
            case.use_delta_palette = rng.below(2) as i32;
            case.thread_level = rng.below(2) as i32;
            case.image_hint = rng.below(5) as u32;
            case.with_stats = rng.below(2) == 0;
            case.with_extra_info = rng.below(4) == 0;
            cases.push(case);
        }
    }
    // The method and quality matrix on a few pictures: every method with the
    // qualities that select the brute-force (method 6, quality 100) and the
    // no-cache (method 5, quality 75 or more) configurations, threaded or not.
    let pictures = [
        (Kind::Photo, 48, 40),
        (Kind::Palette { n: 9, zero: false }, 37, 29),
        (Kind::Palette { n: 120, zero: true }, 44, 41),
        (Kind::Transparent, 66, 64),
    ];
    for &(kind, width, height) in &pictures {
        let stride = width + 1;
        let argb = image(&mut rng, kind, width, height, stride);
        for method in 0..=6 {
            for &quality in &[0.0, 50.0, 75.0, 100.0] {
                let mut case = Case::new(width, height, stride, argb.clone());
                case.method = method;
                case.quality = quality;
                case.thread_level = i32::from(method >= 5);
                case.with_stats = true;
                case.near_lossless = if kind == Kind::Transparent { 60 } else { 100 };
                case.exact = i32::from(method % 2 == 0);
                cases.push(case);
            }
        }
    }
    // use_delta_palette and thread_level have no effect on the bytes of the
    // reference encoder; both values on the same pictures show it.
    for (group, &(kind, width, height)) in [
        (Kind::Palette { n: 30, zero: false }, 33, 33),
        (Kind::Photo, 40, 31),
    ]
    .iter()
    .enumerate()
    {
        let argb = image(&mut rng, kind, width, height, width);
        for use_delta_palette in 0..2 {
            for thread_level in 0..2 {
                let mut case = Case::new(width, height, width, argb.clone());
                case.method = 6;
                case.quality = 100.0;
                case.use_delta_palette = use_delta_palette;
                case.thread_level = thread_level;
                case.with_stats = true;
                case.group = Some(group);
                cases.push(case);
            }
        }
    }
    // Palettes of exactly 17 colors with the color 0: the modified Zeng sort of
    // the brute-force configurations can put 0 last, which `EncodePalette`
    // drops only above 17 colors.
    for _ in 0..8 {
        let (width, height) = (24 + rng.below(9) as usize, 20 + rng.below(9) as usize);
        let argb = image(
            &mut rng,
            Kind::Palette { n: 17, zero: true },
            width,
            height,
            width,
        );
        let mut case = Case::new(width, height, width, argb);
        case.method = 6;
        case.quality = 100.0;
        case.with_stats = true;
        cases.push(case);
    }
    // A user abort at each percent that the encoder reports, threaded or not,
    // and a writer failure at each of the three writer calls.
    let argb = image(&mut rng, Kind::Photo, 30, 22, 30);
    for abort_at in [0, 1, 2, 3, 5, 10, 20, 40, 60, 80, 97, 98, 99, 100, 101] {
        for thread_level in 0..2 {
            let mut case = Case::new(30, 22, 30, argb.clone());
            case.method = 6;
            case.quality = 100.0;
            case.abort_at = abort_at;
            case.thread_level = thread_level;
            case.with_stats = true;
            case.with_extra_info = true;
            cases.push(case);
        }
    }
    let argb = image(&mut rng, Kind::Palette { n: 5, zero: false }, 9, 7, 9);
    for fail_write_at in 0..4 {
        let mut case = Case::new(9, 7, 9, argb.clone());
        case.fail_write_at = fail_write_at;
        case.with_stats = true;
        cases.push(case);
    }
    // A picture without an ARGB plane.
    let mut case = Case::new(4, 4, 4, vec![0; 16]);
    case.null_argb = true;
    case.with_stats = true;
    cases.push(case);
    cases
}

#[test]
fn lossless_files_match_the_c_encoder() {
    let Some(oracle) = Oracle::locate("lossless_files_match_the_c_encoder") else {
        return;
    };
    let cases = cases();
    let mut stdin = Vec::new();
    for case in &cases {
        case.record(&mut stdin);
    }
    let output = oracle.probe("wp11", "encode").run::<&str>(&[], &stdin);
    let actual = output.success().fields();
    let mut expected = Fields::new();
    for case in &cases {
        expected.0.extend(encode(case).0);
    }
    assert_eq!(actual.0.len(), expected.0.len(), "number of observations");
    let per_case = |fields: &Fields| -> Vec<Vec<(String, String)>> {
        let mut groups: Vec<Vec<(String, String)>> = Vec::new();
        for (key, value) in &fields.0 {
            if key == "ret" {
                groups.push(Vec::new());
            }
            groups
                .last_mut()
                .unwrap()
                .push((key.clone(), value.clone()));
        }
        groups
    };
    let (actual, expected) = (per_case(&actual), per_case(&expected));
    let mut mismatches = 0;
    for (index, (a, e)) in actual.iter().zip(&expected).enumerate() {
        if a != e {
            mismatches += 1;
            if mismatches <= 5 {
                let case = &cases[index];
                eprintln!(
                    "case {index}: {}x{} stride {} q {} m {} exact {} nl {} tl {} hint {} abort {} fail {}",
                    case.width,
                    case.height,
                    case.stride,
                    case.quality,
                    case.method,
                    case.exact,
                    case.near_lossless,
                    case.thread_level,
                    case.image_hint,
                    case.abort_at,
                    case.fail_write_at
                );
                for ((ka, va), (_, ve)) in a.iter().zip(e) {
                    if va != ve {
                        let cut = |v: &str| v.chars().take(160).collect::<String>();
                        eprintln!("  {ka}: C    {}\n  {ka}: Rust {}", cut(va), cut(ve));
                    }
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
    // use_delta_palette and thread_level leave the file and the statistics as
    // they are.
    for group in 0..2 {
        let outputs: Vec<&Vec<(String, String)>> = cases
            .iter()
            .zip(&actual)
            .filter(|(case, _)| case.group == Some(group))
            .map(|(_, output)| output)
            .collect();
        assert_eq!(outputs.len(), 4);
        assert!(
            outputs.windows(2).all(|pair| pair[0] == pair[1]),
            "group {group} differs between use_delta_palette and thread_level"
        );
    }
    // The cases reach the paths they were made for.
    let errors: Vec<&str> = actual.iter().map(|case| case[1].1.as_str()).collect();
    for code in ["0", "3", "8", "10"] {
        assert!(errors.contains(&code), "no case ends with error {code}");
    }
}
