//! WP17: misuse of the safe encoder API (ADR 0006) returns the documented
//! errors and never panics. Rust only: these inputs are refused before the
//! translation is called, so there is no C behavior to compare; where C has
//! one, `tests/wp17_encode.rs` compares it.
//!
//! - `documented_errors_for_each_misuse`: zero and too large dimensions, short
//!   rows and planes, zero strides, chroma strides below `(width + 1) / 2` at
//!   odd widths, strides beyond `i32::MAX`, non-finite and out-of-range
//!   qualities, a non-finite target PSNR, a lossless level above 9, and a
//!   configuration that libwebp's validation rejects. Negative dimensions and
//!   strides cannot be expressed (`u32`, `usize`).
//! - `seeded_misuse_never_panics`: a seeded property test over random
//!   geometries, strides, buffer lengths and configurations for every
//!   constructor, the encoder and the simple functions; each call runs under
//!   `catch_unwind`, and each result is the one the rules predict.
//! - `a_panicking_callback_leaves_a_picture_without_planes`: after a writer or
//!   progress hook panics, the picture encodes to `NullParameter`.

use std::panic::{self, AssertUnwindSafe};

use webp::api::encode::{
    AlphaFilter, Config, EncodeError, Encoder, ImageHint, Picture, PixelLayout, PixelRows, Plane,
    SampleForm, YuvaPlanes, encode_lossless, encode_lossy,
};
use webp::enc::config_enc::webp_validate_config;

const LAYOUTS: [PixelLayout; 6] = [
    PixelLayout::Rgb,
    PixelLayout::Rgba,
    PixelLayout::Rgbx,
    PixelLayout::Bgr,
    PixelLayout::Bgra,
    PixelLayout::Bgrx,
];

fn rows(layout: PixelLayout, data: &[u8], width: u32, height: u32, stride: usize) -> PixelRows<'_> {
    PixelRows {
        layout,
        data,
        width,
        height,
        stride,
    }
}

#[test]
fn documented_errors_for_each_misuse() {
    let data = vec![7_u8; 4096];
    let config = Config::new();

    // Dimensions: zero and beyond 16383, for every constructor.
    for (width, height) in [
        (0, 1),
        (1, 0),
        (0, 0),
        (16384, 1),
        (1, 16384),
        (u32::MAX, 2),
    ] {
        let invalid = Err(EncodeError::InvalidDimensions { width, height });
        let r = rows(PixelLayout::Rgba, &data, width, height, 64);
        assert_eq!(Picture::import(r, SampleForm::Yuva).map(|_| ()), invalid);
        assert_eq!(encode_lossy(r, 75.0).map(|_| ()), invalid);
        assert_eq!(encode_lossless(r).map(|_| ()), invalid);
        let mut argb = vec![0_u32; 16];
        assert_eq!(
            Picture::from_argb(&mut argb, width, height, 4).map(|_| ()),
            invalid
        );
        let (mut y, mut u, mut v) = (vec![0_u8; 16], vec![0_u8; 4], vec![0_u8; 4]);
        let planes = YuvaPlanes {
            y: &mut y,
            y_stride: 4,
            u: &mut u,
            v: &mut v,
            uv_stride: 2,
            a: None,
            a_stride: 0,
        };
        assert_eq!(
            Picture::from_yuva(width, height, planes).map(|_| ()),
            invalid
        );
    }

    // Rows: zero, short and huge strides; short buffers.
    for layout in LAYOUTS {
        let bpp = layout.bytes_per_pixel();
        let stride_error = |stride| {
            Err(EncodeError::InvalidStride {
                plane: Plane::Pixels,
                stride,
            })
        };
        for stride in [0, 5 * bpp - 1, i32::MAX as usize + 1, usize::MAX] {
            let r = rows(layout, &data, 5, 3, stride);
            assert_eq!(
                Picture::import(r, SampleForm::Argb).map(|_| ()),
                stride_error(stride)
            );
        }
        // Exactly long enough, and one byte short.
        let needed = 2 * (5 * bpp + 3) + 5 * bpp;
        let r = rows(layout, &data[..needed], 5, 3, 5 * bpp + 3);
        assert!(Picture::import(r, SampleForm::Yuva).is_ok());
        let r = rows(layout, &data[..needed - 1], 5, 3, 5 * bpp + 3);
        assert_eq!(
            Picture::import(r, SampleForm::Yuva).map(|_| ()),
            Err(EncodeError::BufferTooShort {
                plane: Plane::Pixels,
                len: needed - 1,
                needed
            })
        );
    }

    // ARGB: the stride counts pixels.
    let mut argb = vec![0xff00_0000_u32; 3 * 4];
    assert_eq!(
        Picture::from_argb(&mut argb, 4, 3, 3).map(|_| ()),
        Err(EncodeError::InvalidStride {
            plane: Plane::Argb,
            stride: 3
        })
    );
    assert_eq!(
        Picture::from_argb(&mut argb, 4, 3, 0).map(|_| ()),
        Err(EncodeError::InvalidStride {
            plane: Plane::Argb,
            stride: 0
        })
    );
    assert_eq!(
        Picture::from_argb(&mut argb[..11], 4, 3, 4).map(|_| ()),
        Err(EncodeError::BufferTooShort {
            plane: Plane::Argb,
            len: 11,
            needed: 12
        })
    );
    assert!(Picture::from_argb(&mut argb, 4, 3, 4).is_ok());

    // YUVA at an odd width: chroma rows have (width + 1) / 2 samples.
    let yuva = |y_len: usize,
                y_stride: usize,
                uv_len: usize,
                uv_stride: usize,
                a: Option<(usize, usize)>| {
        let (mut y, mut u, mut v) = (
            vec![128_u8; y_len],
            vec![128_u8; uv_len],
            vec![128_u8; uv_len],
        );
        let mut a_plane = a.map(|(len, _)| vec![255_u8; len]);
        let planes = YuvaPlanes {
            y: &mut y,
            y_stride,
            u: &mut u,
            v: &mut v,
            uv_stride,
            a: a_plane.as_deref_mut(),
            a_stride: a.map_or(0, |(_, stride)| stride),
        };
        Picture::from_yuva(5, 3, planes).map(|picture| picture.owned_sample_bytes())
    };
    assert_eq!(yuva(15, 5, 6, 3, None), Ok(0));
    assert_eq!(yuva(15, 5, 6, 3, Some((15, 5))), Ok(0));
    assert_eq!(
        yuva(15, 5, 6, 2, None),
        Err(EncodeError::InvalidStride {
            plane: Plane::U,
            stride: 2
        })
    );
    assert_eq!(
        yuva(15, 4, 6, 3, None),
        Err(EncodeError::InvalidStride {
            plane: Plane::Y,
            stride: 4
        })
    );
    assert_eq!(
        yuva(15, 0, 6, 3, None),
        Err(EncodeError::InvalidStride {
            plane: Plane::Y,
            stride: 0
        })
    );
    assert_eq!(
        yuva(14, 5, 6, 3, None),
        Err(EncodeError::BufferTooShort {
            plane: Plane::Y,
            len: 14,
            needed: 15
        })
    );
    assert_eq!(
        yuva(15, 5, 5, 3, None),
        Err(EncodeError::BufferTooShort {
            plane: Plane::U,
            len: 5,
            needed: 6
        })
    );
    assert_eq!(
        yuva(15, 5, 6, 3, Some((15, 4))),
        Err(EncodeError::InvalidStride {
            plane: Plane::A,
            stride: 4
        })
    );
    assert_eq!(
        yuva(15, 5, 6, 3, Some((10, 5))),
        Err(EncodeError::BufferTooShort {
            plane: Plane::A,
            len: 10,
            needed: 15
        })
    );
    // Mismatched chroma planes: V shorter than U.
    let (mut y, mut u, mut v) = (vec![0_u8; 15], vec![0_u8; 6], vec![0_u8; 5]);
    let planes = YuvaPlanes {
        y: &mut y,
        y_stride: 5,
        u: &mut u,
        v: &mut v,
        uv_stride: 3,
        a: None,
        a_stride: 0,
    };
    assert_eq!(
        Picture::from_yuva(5, 3, planes).map(|_| ()),
        Err(EncodeError::BufferTooShort {
            plane: Plane::V,
            len: 5,
            needed: 6
        })
    );

    // Configuration.
    let quality = Err(EncodeError::InvalidParameter { name: "quality" });
    for q in [f32::NAN, f32::INFINITY, f32::NEG_INFINITY, -0.5, 100.5] {
        assert_eq!(Config::new().quality(q).validate(), quality);
        assert_eq!(Config::preset(Default::default(), q).map(|_| ()), quality);
        let r = rows(PixelLayout::Rgb, &data, 2, 2, 6);
        assert_eq!(encode_lossy(r, q).map(|_| ()), quality);
    }
    for psnr in [f32::NAN, f32::INFINITY] {
        assert_eq!(
            Config::new().target_psnr(psnr).validate(),
            Err(EncodeError::InvalidParameter {
                name: "target_psnr"
            })
        );
    }
    assert_eq!(
        Config::new().lossless_level(10),
        Err(EncodeError::InvalidParameter {
            name: "lossless_level"
        })
    );
    for invalid in [
        Config::new().method(7),
        Config::new().near_lossless(101),
        Config::new().alpha_quality(101),
        Config::new().target_psnr(-1.0),
    ] {
        assert_eq!(invalid.validate(), Err(EncodeError::InvalidConfiguration));
    }
    let mut raw = Config::new();
    raw.raw_mut().segments = 5;
    assert_eq!(raw.validate(), Err(EncodeError::InvalidConfiguration));

    // An encoding with an invalid configuration calls no callback.
    let mut argb = vec![0xff00_0000_u32; 4];
    let mut picture = Picture::from_argb(&mut argb, 2, 2, 2).unwrap();
    let mut called = false;
    let bad = Config::new().method(9);
    assert_eq!(
        Encoder::new(&bad)
            .progress(|_| {
                called = true;
                true
            })
            .encode_to(&mut picture, |_| unreachable!("no write")),
        Err(EncodeError::InvalidConfiguration)
    );
    assert!(!called);
    assert!(Encoder::new(&config).encode(&mut picture).is_ok());
}

/// SplitMix64.
struct Rng(u64);

impl Rng {
    fn next(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn below(&mut self, bound: u64) -> u64 {
        self.next() % bound
    }

    fn pick<T: Copy>(&mut self, values: &[T]) -> T {
        values[self.below(values.len() as u64) as usize]
    }

    fn chance(&mut self, one_in: u64) -> bool {
        self.below(one_in) == 0
    }
}

/// The rules of ADR 0006 for a plane, as the test states them.
fn plane_rule(
    plane: Plane,
    len: usize,
    columns: usize,
    rows: usize,
    stride: usize,
) -> Result<(), EncodeError> {
    let invalid = Err(EncodeError::InvalidStride { plane, stride });
    if stride < columns || stride > i32::MAX as usize {
        return invalid;
    }
    let Some(needed) = (rows - 1)
        .checked_mul(stride)
        .and_then(|start| start.checked_add(columns))
    else {
        return invalid;
    };
    if len < needed {
        return Err(EncodeError::BufferTooShort { plane, len, needed });
    }
    Ok(())
}

fn dimension_rule(width: u32, height: u32) -> Result<(), EncodeError> {
    if width == 0 || height == 0 || width > 16383 || height > 16383 {
        Err(EncodeError::InvalidDimensions { width, height })
    } else {
        Ok(())
    }
}

fn config_rule(config: &Config) -> Result<(), EncodeError> {
    let raw = config.as_raw();
    if !raw.quality.is_finite() || !(0.0..=100.0).contains(&raw.quality) {
        return Err(EncodeError::InvalidParameter { name: "quality" });
    }
    if !raw.target_psnr.is_finite() {
        return Err(EncodeError::InvalidParameter {
            name: "target_psnr",
        });
    }
    if !webp_validate_config(raw) {
        return Err(EncodeError::InvalidConfiguration);
    }
    Ok(())
}

/// A dimension: mostly small, sometimes 0 or beyond the limit.
fn dimension(rng: &mut Rng) -> u32 {
    match rng.below(12) {
        0 => 0,
        1 => rng.pick(&[16383, 16384, 100_000, u32::MAX]),
        _ => 1 + rng.below(23) as u32,
    }
}

/// A stride for rows of `columns` elements.
fn stride(rng: &mut Rng, columns: usize) -> usize {
    match rng.below(20) {
        0 => 0,
        1 => columns.saturating_sub(1),
        2 => rng.pick(&[i32::MAX as usize, i32::MAX as usize + 1, usize::MAX]),
        3..=7 => columns + 1 + rng.below(9) as usize,
        _ => columns,
    }
}

/// A buffer for `rows` rows of `columns` elements at `stride`: mostly the
/// needed length, sometimes shorter or longer, never more than 64 KiB.
fn length(rng: &mut Rng, columns: usize, rows: usize, stride: usize) -> usize {
    let needed = rows
        .saturating_sub(1)
        .saturating_mul(stride)
        .saturating_add(columns);
    let needed = needed.min(1 << 16);
    match rng.below(12) {
        0 => needed.saturating_sub(1 + rng.below(4) as usize),
        1 => rng.below(64) as usize,
        2 | 3 => needed + rng.below(8) as usize,
        _ => needed,
    }
}

fn random_config(rng: &mut Rng) -> Config {
    let mut config = if rng.chance(2) {
        Config::new()
    } else {
        Config::preset(Default::default(), rng.below(101) as f32).unwrap()
    };
    if rng.chance(4) {
        config = config.lossless_level(rng.below(10) as u8).unwrap();
    }
    config = config
        .lossless(rng.chance(3))
        .method(rng.pick(&[0, 2, 4, 6, 6, 7]))
        .exact(rng.chance(2))
        .near_lossless(rng.pick(&[100, 60, 0, 101]))
        .alpha_quality(rng.pick(&[100, 50, 0, 101]))
        .alpha_filter(rng.pick(&[AlphaFilter::None, AlphaFilter::Fast, AlphaFilter::Best]))
        .alpha_compression(!rng.chance(4))
        .sharp_yuv(rng.chance(3))
        .multithreaded(rng.chance(2))
        .image_hint(rng.pick(&[
            ImageHint::Default,
            ImageHint::Picture,
            ImageHint::Photo,
            ImageHint::Graph,
        ]));
    match rng.below(12) {
        0 => config = config.quality(rng.pick(&[f32::NAN, f32::INFINITY, -1.0, 100.5])),
        1 => config = config.target_psnr(rng.pick(&[f32::NAN, f32::NEG_INFINITY, 35.0])),
        2 => config = config.target_size(rng.pick(&[0, 500, u32::MAX])),
        3 => config.raw_mut().segments = rng.pick(&[0, 1, 5]),
        4 => config.raw_mut().pass = rng.pick(&[0, 3, 11]),
        5 => config.raw_mut().preprocessing = rng.pick(&[2, 4, 8]),
        _ => {}
    }
    config
}

/// Runs `f` and fails the test if it panics.
fn no_panic<T>(what: &str, f: impl FnOnce() -> T) -> T {
    panic::catch_unwind(AssertUnwindSafe(f)).unwrap_or_else(|_| panic!("{what} panicked"))
}

/// Checks an encoding of valid input: the configuration's rule decides, and
/// an encoder failure carries a libwebp code.
fn check_encoding(what: &str, config: &Config, result: Result<Vec<u8>, EncodeError>) -> bool {
    match config_rule(config) {
        Err(expected) => {
            assert_eq!(result, Err(expected), "{what}");
            false
        }
        Ok(()) => match result {
            Ok(bytes) => {
                assert_eq!(&bytes[..4], b"RIFF", "{what}");
                true
            }
            Err(error) => {
                assert!(error.code().is_some(), "{what}: {error:?}");
                false
            }
        },
    }
}

#[test]
fn seeded_misuse_never_panics() {
    let mut rng = Rng(0x1717_0006);
    let (mut refused, mut encoded) = (0, 0);
    for round in 0..2000 {
        let (width, height) = (dimension(&mut rng), dimension(&mut rng));
        let config = random_config(&mut rng);
        let what = format!("round {round}: {width} by {height}, {config:?}");
        let dims = dimension_rule(width, height);
        let (w, h) = (width as usize, height as usize);
        match round % 4 {
            // Rows into a picture, then an encoding.
            0 | 1 => {
                let layout = rng.pick(&LAYOUTS);
                let columns = w.saturating_mul(layout.bytes_per_pixel());
                let stride = stride(&mut rng, columns);
                let len = length(&mut rng, columns, h, stride);
                let data: Vec<u8> = (0..len).map(|_| rng.next() as u8).collect();
                let r = rows(layout, &data, width, height, stride);
                let expected =
                    dims.and_then(|()| plane_rule(Plane::Pixels, len, columns, h, stride));
                if round % 4 == 0 {
                    let form = rng.pick(&[SampleForm::Yuva, SampleForm::Argb]);
                    let picture = no_panic(&what, || Picture::import(r, form));
                    match expected {
                        Err(error) => {
                            assert_eq!(picture.map(|_| ()), Err(error), "{what}");
                            refused += 1;
                        }
                        Ok(()) => {
                            let mut picture = picture.unwrap();
                            let result =
                                no_panic(&what, || Encoder::new(&config).encode(&mut picture));
                            encoded += usize::from(check_encoding(&what, &config, result));
                        }
                    }
                } else {
                    // The simple functions check the quality, then the rows.
                    let quality = rng.pick(&[75.0, 0.0, 100.0, f32::NAN, 101.0]);
                    let lossy = no_panic(&what, || encode_lossy(r, quality));
                    let lossless = no_panic(&what, || encode_lossless(r));
                    let quality_rule = if quality.is_finite() && (0.0..=100.0).contains(&quality) {
                        Ok(())
                    } else {
                        Err(EncodeError::InvalidParameter { name: "quality" })
                    };
                    match quality_rule.and(expected) {
                        Err(error) => {
                            assert_eq!(lossy.map(|_| ()), Err(error), "{what}");
                            refused += 1;
                        }
                        Ok(()) => encoded += usize::from(lossy.is_ok()),
                    }
                    match expected {
                        Err(error) => assert_eq!(lossless.map(|_| ()), Err(error), "{what}"),
                        Ok(()) => encoded += usize::from(lossless.is_ok()),
                    }
                }
            }
            // Caller ARGB pixels.
            2 => {
                let stride = stride(&mut rng, w);
                let len = length(&mut rng, w, h, stride);
                let mut argb: Vec<u32> = (0..len).map(|_| rng.next() as u32).collect();
                let expected = dims.and_then(|()| plane_rule(Plane::Argb, len, w, h, stride));
                let picture = no_panic(&what, || {
                    Picture::from_argb(&mut argb, width, height, stride)
                });
                match expected {
                    Err(error) => {
                        assert_eq!(picture.map(|_| ()), Err(error), "{what}");
                        refused += 1;
                    }
                    Ok(()) => {
                        let mut picture = picture.unwrap();
                        let result = no_panic(&what, || Encoder::new(&config).encode(&mut picture));
                        encoded += usize::from(check_encoding(&what, &config, result));
                    }
                }
            }
            // Caller YUVA planes.
            _ => {
                let (uv_w, uv_h) = (w.div_ceil(2), h.div_ceil(2));
                let y_stride = stride(&mut rng, w);
                let uv_stride = stride(&mut rng, uv_w);
                let with_alpha = rng.chance(2);
                let a_stride = stride(&mut rng, w);
                let lens = [
                    length(&mut rng, w, h, y_stride),
                    length(&mut rng, uv_w, uv_h, uv_stride),
                    length(&mut rng, uv_w, uv_h, uv_stride),
                    length(&mut rng, w, h, a_stride),
                ];
                let mut planes: Vec<Vec<u8>> = lens
                    .iter()
                    .map(|&len| (0..len).map(|_| rng.next() as u8).collect())
                    .collect();
                let expected = dims
                    .and_then(|()| plane_rule(Plane::Y, lens[0], w, h, y_stride))
                    .and_then(|()| plane_rule(Plane::U, lens[1], uv_w, uv_h, uv_stride))
                    .and_then(|()| plane_rule(Plane::V, lens[2], uv_w, uv_h, uv_stride))
                    .and_then(|()| {
                        if with_alpha {
                            plane_rule(Plane::A, lens[3], w, h, a_stride)
                        } else {
                            Ok(())
                        }
                    });
                let [y, u, v, a] = planes.as_mut_slice() else {
                    unreachable!()
                };
                let yuva = YuvaPlanes {
                    y,
                    y_stride,
                    u,
                    v,
                    uv_stride,
                    a: with_alpha.then_some(a.as_mut_slice()),
                    a_stride,
                };
                let picture = no_panic(&what, || Picture::from_yuva(width, height, yuva));
                match expected {
                    Err(error) => {
                        assert_eq!(picture.map(|_| ()), Err(error), "{what}");
                        refused += 1;
                    }
                    Ok(()) => {
                        let mut picture = picture.unwrap();
                        let result = no_panic(&what, || Encoder::new(&config).encode(&mut picture));
                        encoded += usize::from(check_encoding(&what, &config, result));
                    }
                }
            }
        }
    }
    eprintln!(
        "seeded_misuse_never_panics: 2000 rounds, {refused} refused inputs, {encoded} encodings"
    );
    assert!(
        refused > 1000 && encoded > 500,
        "{refused} refused, {encoded} encoded"
    );
}

/// A writer or progress hook that panics leaves the picture without planes;
/// a later encoding returns `NullParameter` instead of panicking.
#[test]
fn a_panicking_callback_leaves_a_picture_without_planes() {
    let rgba: Vec<u8> = (0..24 * 20 * 4).map(|i| (i * 37 % 251) as u8).collect();
    let rows = rows(PixelLayout::Rgba, &rgba, 24, 20, 96);
    let mut checked = 0;
    for form in [SampleForm::Yuva, SampleForm::Argb] {
        for lossless in [false, true] {
            for writer_panics in [false, true] {
                let mut argb: Vec<u32> = rgba
                    .chunks(4)
                    .map(|p| u32::from_be_bytes([p[3], p[0], p[1], p[2]]))
                    .collect();
                let mut pictures = vec![
                    Picture::import(rows, form).unwrap(),
                    Picture::from_argb(&mut argb, 24, 20, 24).unwrap(),
                ];
                for picture in &mut pictures {
                    let config = Config::new().lossless(lossless);
                    let caught = panic::catch_unwind(AssertUnwindSafe(|| {
                        if writer_panics {
                            Encoder::new(&config)
                                .encode_to(picture, |_| panic!("the writer panics"))
                        } else {
                            Encoder::new(&config)
                                .progress(|_| panic!("the hook panics"))
                                .encode(picture)
                                .map(|_| ())
                        }
                    }));
                    assert!(caught.is_err(), "the callback's panic propagates");
                    assert_eq!(picture.owned_sample_bytes(), 0);
                    for again in [false, true] {
                        let config = Config::new().lossless(again);
                        let result = panic::catch_unwind(AssertUnwindSafe(|| {
                            Encoder::new(&config).encode(picture)
                        }))
                        .expect("a later encoding does not panic");
                        assert_eq!(result, Err(EncodeError::NullParameter));
                        checked += 1;
                    }
                }
            }
        }
    }
    assert_eq!(checked, 32);
}

#[test]
fn pictures_are_send_and_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Picture<'static>>();
    send_sync::<Picture<'_>>();
    send_sync::<Config>();
    send_sync::<EncodeError>();
}
