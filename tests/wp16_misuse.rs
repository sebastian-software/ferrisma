//! WP16: misuse of the safe decoder API (ADR 0007) returns the documented
//! errors and never panics. Rust only: the API refuses these inputs before the
//! translation is called, or the translation returns libwebp's status for
//! them, which `tests/wp16_decode.rs` and `tests/wp16_incremental.rs` compare
//! with C where C has one.
//!
//! - `documented_errors_for_each_misuse`: every error of the API with an input
//!   that produces it: crops of zero size, at odd offsets, beyond `i32::MAX`
//!   and outside the image; scales to 0 and beyond 16383; dithering above
//!   100; zero, narrow, wrapping and overflowing strides; short and empty
//!   buffers and planes; outputs that do not match the color mode; an
//!   incomplete or caller-memory incremental decoding asked for an image;
//!   prefixes, garbage, animations and a corrupt animation frame. Negative
//!   strides and offsets cannot be expressed (`u32`, `usize`).
//! - `the_translation_panics_where_the_api_refuses`: the inputs whose raw
//!   form makes the translation panic (a buffer description larger than its
//!   slice, a negative stride, a short chroma plane): the translation panics,
//!   and the API returns its error.
//! - `seeded_misuse_never_panics`: a seeded property test over random
//!   configurations, outputs, data and call sequences of every entry point;
//!   each call runs under `catch_unwind`, and each refusal of the API is the
//!   one that the rules of ADR 0007 predict.
//! - `public_types_are_send_and_sync`.

use std::panic::{self, AssertUnwindSafe};

use webp::api::decode::{
    AnimationDecoder, AnimationInfo, AnimationMode, AnimationOptions, ColorMode, Config, Container,
    DecodeError, Features, Format, Image, IncrementalDecoder, Output, Plane, Status, YuvaPlanes,
    decode, decode_into,
};
use webp::api::encode::{PixelLayout, PixelRows, encode_lossless, encode_lossy};
use webp::dec::webp_dec::webp_decode;
use webp::webp::decode::{
    WEBP_CSP_MODE, WebPDecBufferPlanes, WebPDecoderConfig, WebPRGBABuffer, WebPYUVABuffer,
    webp_init_decoder_config,
};
use webp::webp::types::{PlaneOffset, PlaneStorage};

const MODES: [ColorMode; 13] = [
    ColorMode::Rgb,
    ColorMode::Rgba,
    ColorMode::Bgr,
    ColorMode::Bgra,
    ColorMode::Argb,
    ColorMode::Rgba4444,
    ColorMode::Rgb565,
    ColorMode::PremultipliedRgba,
    ColorMode::PremultipliedBgra,
    ColorMode::PremultipliedArgb,
    ColorMode::PremultipliedRgba4444,
    ColorMode::Yuv,
    ColorMode::Yuva,
];

/// A picture of `width` by `height` with varied samples, and alpha below 255
/// in places when `alpha` is set.
fn pixels(width: u32, height: u32, alpha: bool) -> Vec<u8> {
    let mut rgba = Vec::new();
    for y in 0..height {
        for x in 0..width {
            let a = if alpha && (x + y) % 3 == 0 { 90 } else { 255 };
            rgba.extend([
                (x * 37 % 256) as u8,
                (y * 53 % 256) as u8,
                ((x + y) * 11 % 256) as u8,
                a,
            ]);
        }
    }
    rgba
}

/// A lossless or lossy WebP file of `width` by `height`.
fn file(width: u32, height: u32, alpha: bool, lossless: bool) -> Vec<u8> {
    let rgba = pixels(width, height, alpha);
    let rows = PixelRows {
        layout: PixelLayout::Rgba,
        data: &rgba,
        width,
        height,
        stride: width as usize * 4,
    };
    if lossless {
        encode_lossless(rows).unwrap()
    } else {
        encode_lossy(rows, 75.0).unwrap()
    }
}

fn chunk(tag: &[u8], payload: &[u8]) -> Vec<u8> {
    let mut bytes = tag.to_vec();
    bytes.extend_from_slice(&(payload.len() as u32).to_le_bytes());
    bytes.extend_from_slice(payload);
    if payload.len() % 2 == 1 {
        bytes.push(0);
    }
    bytes
}

fn riff(body: &[u8]) -> Vec<u8> {
    let mut file = b"RIFF".to_vec();
    file.extend_from_slice(&(body.len() as u32 + 4).to_le_bytes());
    file.extend_from_slice(b"WEBP");
    file.extend_from_slice(body);
    file
}

/// A 4 by 4 animation of two lossless frames; `corrupt` damages the second
/// frame's bitstream after its header.
fn animation(corrupt: bool) -> Vec<u8> {
    let frame = |red: u8, duration: u8| {
        let rgba = [red, 7, 9, 255].repeat(16);
        let image = encode_lossless(PixelRows {
            layout: PixelLayout::Rgba,
            data: &rgba,
            width: 4,
            height: 4,
            stride: 16,
        })
        .unwrap();
        let mut payload = vec![0, 0, 0, 0, 0, 0, 3, 0, 0, 3, 0, 0, duration, 0, 0, 0];
        payload.extend_from_slice(&image[12..]);
        payload
    };
    let mut second = frame(200, 20);
    if corrupt {
        let len = second.len();
        for byte in &mut second[16 + 8 + 5..len] {
            *byte = 0xff;
        }
    }
    let body = [
        chunk(b"VP8X", &[0x02, 0, 0, 0, 3, 0, 0, 3, 0, 0]),
        chunk(b"ANIM", &[1, 2, 3, 4, 0, 0]),
        chunk(b"ANMF", &frame(100, 10)),
        chunk(b"ANMF", &second),
    ]
    .concat();
    riff(&body)
}

fn rows_output(data: &mut [u8], stride: usize) -> Output<'_> {
    Output::Rows { data, stride }
}

#[test]
fn documented_errors_for_each_misuse() {
    let image = file(10, 6, true, true); // lossless 10 by 6 with alpha
    let lossy = file(9, 5, false, false); // lossy 9 by 5
    let rgba = Config::new(ColorMode::Rgba);

    // Crops: zero size, odd offsets, values beyond i32::MAX.
    for (left, top, width, height) in [
        (0, 0, 0, 1),
        (0, 0, 1, 0),
        (1, 0, 2, 2),
        (0, 3, 2, 2),
        (0x8000_0000, 0, 2, 2),
        (0, 0, 0x8000_0000, 2),
        (u32::MAX - 1, 0, 2, 2),
    ] {
        let config = rgba.crop(left, top, width, height);
        let invalid = Err(DecodeError::InvalidCrop {
            left,
            top,
            width,
            height,
        });
        assert_eq!(config.validate(), invalid);
        assert_eq!(decode(&image, &config).map(|_| ()), invalid);
        assert_eq!(IncrementalDecoder::new(&config).map(|_| ()), invalid);
    }
    // Crops outside the image: refused against the features; the incremental
    // decoder learns the size from the header, where libwebp refuses it.
    for (left, top, width, height) in [(10, 0, 1, 1), (0, 6, 1, 1), (2, 0, 9, 6), (0, 2, 10, 5)] {
        let config = rgba.crop(left, top, width, height);
        assert_eq!(config.validate(), Ok(()));
        let invalid = Err(DecodeError::InvalidCrop {
            left,
            top,
            width,
            height,
        });
        assert_eq!(
            config
                .output_size(&Features::parse(&image).unwrap())
                .map(|_| ()),
            invalid
        );
        assert_eq!(decode(&image, &config).map(|_| ()), invalid);
        let mut decoder = IncrementalDecoder::new(&config).unwrap();
        assert_eq!(decoder.append(&image), Err(DecodeError::InvalidParam));
    }
    // Scales: zero and beyond 16383.
    for (width, height) in [
        (0, 4),
        (4, 0),
        (0, 0),
        (16384, 4),
        (4, 16384),
        (u32::MAX, u32::MAX),
    ] {
        let config = rgba.scale(width, height);
        let invalid = Err(DecodeError::InvalidScale { width, height });
        assert_eq!(config.validate(), invalid);
        assert_eq!(decode(&image, &config).map(|_| ()), invalid);
    }
    assert_eq!(rgba.scale(16383, 1).validate(), Ok(()));
    // Dithering strengths above 100.
    assert_eq!(
        rgba.dithering(101).validate(),
        Err(DecodeError::InvalidParameter { name: "dithering" })
    );
    assert_eq!(
        decode(&lossy, &rgba.alpha_dithering(255)).map(|_| ()),
        Err(DecodeError::InvalidParameter {
            name: "alpha_dithering"
        })
    );

    // Rows: zero, narrow, beyond i32::MAX and overflowing strides; short rows.
    let mut rows = vec![0_u8; 40 * 6];
    for stride in [0, 39, 0x8000_0000, usize::MAX] {
        assert_eq!(
            decode_into(&image, &rgba, rows_output(&mut rows, stride)),
            Err(DecodeError::InvalidStride {
                plane: Plane::Rows,
                stride
            })
        );
    }
    assert_eq!(
        decode_into(&image, &rgba, rows_output(&mut rows[..40 * 6 - 1], 40)),
        Err(DecodeError::BufferTooShort {
            plane: Plane::Rows,
            len: 239,
            needed: 240
        })
    );
    assert_eq!(
        decode_into(&image, &rgba, rows_output(&mut rows, 40)),
        Ok((10, 6))
    );
    assert!(rows.iter().any(|&byte| byte != 0));

    // Planes: strides and lengths of each plane (9 by 5: chroma 5 by 3).
    let yuv = Config::new(ColorMode::Yuv);
    let yuva = Config::new(ColorMode::Yuva);
    let (mut y, mut u, mut v, mut a) = (
        vec![0_u8; 45],
        vec![0_u8; 15],
        vec![0_u8; 15],
        vec![0_u8; 45],
    );
    let mut check = |config: &Config,
                     lens: [usize; 4],
                     strides: [usize; 3],
                     alpha: bool,
                     expected: Result<(u32, u32), DecodeError>| {
        let output = Output::Planes(YuvaPlanes {
            y: &mut y[..lens[0]],
            y_stride: strides[0],
            u: &mut u[..lens[1]],
            v: &mut v[..lens[2]],
            uv_stride: strides[1],
            a: alpha.then(|| &mut a[..lens[3]]),
            a_stride: strides[2],
        });
        assert_eq!(
            decode_into(&lossy, config, output),
            expected,
            "{lens:?} {strides:?}"
        );
    };
    check(&yuv, [45, 15, 15, 0], [9, 5, 0], false, Ok((9, 5)));
    check(&yuva, [45, 15, 15, 45], [9, 5, 9], true, Ok((9, 5)));
    check(
        &yuv,
        [45, 15, 15, 0],
        [8, 5, 0],
        false,
        Err(DecodeError::InvalidStride {
            plane: Plane::Y,
            stride: 8,
        }),
    );
    // An odd width: chroma rows of (9 + 1) / 2 = 5 samples.
    check(
        &yuv,
        [45, 15, 15, 0],
        [9, 4, 0],
        false,
        Err(DecodeError::InvalidStride {
            plane: Plane::U,
            stride: 4,
        }),
    );
    check(
        &yuva,
        [45, 15, 15, 45],
        [9, 5, 0],
        true,
        Err(DecodeError::InvalidStride {
            plane: Plane::A,
            stride: 0,
        }),
    );
    check(
        &yuv,
        [44, 15, 15, 0],
        [9, 5, 0],
        false,
        Err(DecodeError::BufferTooShort {
            plane: Plane::Y,
            len: 44,
            needed: 45,
        }),
    );
    check(
        &yuv,
        [45, 14, 15, 0],
        [9, 5, 0],
        false,
        Err(DecodeError::BufferTooShort {
            plane: Plane::U,
            len: 14,
            needed: 15,
        }),
    );
    check(
        &yuv,
        [45, 15, 14, 0],
        [9, 5, 0],
        false,
        Err(DecodeError::BufferTooShort {
            plane: Plane::V,
            len: 14,
            needed: 15,
        }),
    );
    check(
        &yuva,
        [45, 15, 15, 44],
        [9, 5, 9],
        true,
        Err(DecodeError::BufferTooShort {
            plane: Plane::A,
            len: 44,
            needed: 45,
        }),
    );
    // Outputs that do not match the mode.
    check(
        &yuva,
        [45, 15, 15, 0],
        [9, 5, 0],
        false,
        Err(DecodeError::OutputMismatch {
            mode: ColorMode::Yuva,
        }),
    );
    check(
        &yuv,
        [45, 15, 15, 45],
        [9, 5, 9],
        true,
        Err(DecodeError::OutputMismatch {
            mode: ColorMode::Yuv,
        }),
    );
    check(
        &rgba,
        [45, 15, 15, 0],
        [9, 5, 0],
        false,
        Err(DecodeError::OutputMismatch {
            mode: ColorMode::Rgba,
        }),
    );
    assert_eq!(
        decode_into(&lossy, &yuv, rows_output(&mut rows, 40)),
        Err(DecodeError::OutputMismatch {
            mode: ColorMode::Yuv
        })
    );

    // The incremental decoder over caller memory: what does not depend on the
    // image is checked at once, the rest when the header arrives.
    let mut small = vec![0_u8; 16];
    assert_eq!(
        IncrementalDecoder::with_output(&rgba, rows_output(&mut small, 0)).map(|_| ()),
        Err(DecodeError::InvalidStride {
            plane: Plane::Rows,
            stride: 0
        })
    );
    assert_eq!(
        IncrementalDecoder::with_output(&rgba, rows_output(&mut small, 0x8000_0000)).map(|_| ()),
        Err(DecodeError::InvalidStride {
            plane: Plane::Rows,
            stride: 0x8000_0000
        })
    );
    assert_eq!(
        IncrementalDecoder::with_output(&rgba, rows_output(&mut [], 40)).map(|_| ()),
        Err(DecodeError::BufferTooShort {
            plane: Plane::Rows,
            len: 0,
            needed: 1
        })
    );
    assert_eq!(
        IncrementalDecoder::with_output(&yuv, rows_output(&mut small, 4)).map(|_| ()),
        Err(DecodeError::OutputMismatch {
            mode: ColorMode::Yuv
        })
    );
    {
        let mut decoder =
            IncrementalDecoder::with_output(&rgba, rows_output(&mut small, 40)).unwrap();
        assert_eq!(decoder.append(&image), Err(DecodeError::InvalidParam));
        // Every later step reports the failure as a bitstream error, as C.
        assert_eq!(decoder.append(&image), Err(DecodeError::BitstreamError));
        assert_eq!(
            decoder.into_image().map(|_| ()),
            Err(DecodeError::OutputMismatch {
                mode: ColorMode::Rgba
            })
        );
    }
    // Mixed append and update, a shorter update, an image before the end.
    let mut decoder = IncrementalDecoder::new(&rgba).unwrap();
    assert_eq!(decoder.append(&image[..20]), Ok(Status::NeedsMoreData));
    assert_eq!(decoder.update(&image), Err(DecodeError::InvalidParam));
    let mut decoder = IncrementalDecoder::new(&rgba).unwrap();
    assert_eq!(decoder.update(&image[..30]), Ok(Status::NeedsMoreData));
    assert_eq!(decoder.update(&image[..20]), Err(DecodeError::InvalidParam));
    // An empty append before any data, which libwebp asserts against, and
    // empty pieces later.
    let mut decoder = IncrementalDecoder::new(&rgba).unwrap();
    assert_eq!(decoder.append(&[]), Ok(Status::NeedsMoreData));
    assert_eq!(decoder.append(&image[..5]), Ok(Status::NeedsMoreData));
    assert_eq!(decoder.append(&[]), Ok(Status::NeedsMoreData));
    let mut updated = IncrementalDecoder::new(&rgba).unwrap();
    assert_eq!(updated.update(&[]), Ok(Status::NeedsMoreData));
    assert_eq!(updated.append(&[]), Err(DecodeError::InvalidParam));
    assert_eq!(
        decoder.append(&image[5..image.len() / 2]),
        Ok(Status::NeedsMoreData)
    );
    assert_eq!(
        decoder.into_image().map(|_| ()),
        Err(DecodeError::Suspended)
    );
    let mut decoder = IncrementalDecoder::new(&rgba).unwrap();
    assert_eq!(decoder.append(&image), Ok(Status::Complete));
    assert_eq!(decoder.append(&image), Ok(Status::Complete));
    let whole = decoder.into_image().unwrap();
    assert_eq!(whole, decode(&image, &rgba).unwrap());

    // Data: prefixes, garbage, an animation given to the still decoder.
    assert_eq!(
        Features::parse(&image[..12]),
        Err(DecodeError::NotEnoughData)
    );
    assert_eq!(
        Features::parse(b"not a WebP file"),
        Err(DecodeError::BitstreamError)
    );
    assert_eq!(
        decode(&image[..12], &rgba).map(|_| ()),
        Err(DecodeError::BitstreamError)
    );
    assert_eq!(
        Container::new(&image[..12]).map(|_| ()),
        Err(DecodeError::NotEnoughData)
    );
    assert_eq!(
        Container::new(b"not a WebP file").map(|_| ()),
        Err(DecodeError::BitstreamError)
    );
    assert_eq!(
        Container::new(&[]).map(|_| ()),
        Err(DecodeError::NotEnoughData)
    );
    let anim = animation(false);
    let features = Features::parse(&anim).unwrap();
    assert!(features.has_animation && features.format.is_none());
    assert_eq!(
        decode(&anim, &rgba).map(|_| ()),
        Err(DecodeError::UnsupportedFeature)
    );
    let mut decoder = IncrementalDecoder::new(&rgba).unwrap();
    assert_eq!(decoder.append(&anim), Err(DecodeError::UnsupportedFeature));
    assert_eq!(
        AnimationDecoder::new(&anim[..anim.len() - 10], AnimationOptions::default()).map(|_| ()),
        Err(DecodeError::NotEnoughData)
    );
    assert_eq!(
        AnimationDecoder::new(b"RIFF", AnimationOptions::default()).map(|_| ()),
        Err(DecodeError::NotEnoughData)
    );
    // A corrupt frame: the first decodes, the second fails and stays.
    let corrupt = animation(true);
    let mut decoder = AnimationDecoder::new(&corrupt, AnimationOptions::default()).unwrap();
    assert_eq!(
        decoder
            .next_frame()
            .unwrap()
            .map(|frame| frame.timestamp_ms),
        Some(10)
    );
    assert_eq!(
        decoder.next_frame().map(|_| ()),
        Err(DecodeError::FrameFailed)
    );
    assert_eq!(
        decoder.next_frame().map(|_| ()),
        Err(DecodeError::FrameFailed)
    );
    assert!(decoder.has_more_frames());
    decoder.reset();
    assert!(decoder.next_frame().unwrap().is_some());
    // The error codes.
    assert_eq!(
        DecodeError::InvalidCrop {
            left: 1,
            top: 0,
            width: 1,
            height: 1
        }
        .code(),
        None
    );
    assert_eq!(DecodeError::UserAbort.code().map(|code| code.0), Some(6));
    assert_eq!(
        Features::parse(&image).unwrap().format,
        Some(Format::Lossless)
    );
}

/// Whether `call` panics (the default hook prints its message).
fn panics(call: impl FnOnce()) -> bool {
    panic::catch_unwind(AssertUnwindSafe(call)).is_err()
}

#[test]
fn the_translation_panics_where_the_api_refuses() {
    let image = file(8, 4, false, true);
    let rgba = Config::new(ColorMode::Rgba);

    // Two RGBA rows described as the four rows of the image: the translation
    // addresses rows past the slice. The API describes every slice by its
    // length.
    let mut rows = vec![0_u8; 32 * 2];
    assert!(panics(|| {
        let mut config = WebPDecoderConfig::default();
        assert!(webp_init_decoder_config(&mut config));
        config.output.colorspace = WEBP_CSP_MODE::MODE_RGBA;
        config.output.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
            rgba: PlaneOffset::default(),
            stride: 32,
            size: 32 * 4,
        });
        config.output.storage = PlaneStorage::External(vec![&mut rows]);
        let _ = webp_decode(&image, &mut config);
    }));
    assert_eq!(
        decode_into(
            &image,
            &rgba,
            Output::Rows {
                data: &mut rows,
                stride: 32
            }
        ),
        Err(DecodeError::BufferTooShort {
            plane: Plane::Rows,
            len: 64,
            needed: 128
        })
    );

    // A negative stride, which C takes as bottom-up rows: the translation's
    // rows start before the slice. Its bits as a `usize` exceed `i32::MAX`.
    let mut rows = vec![0_u8; 32 * 4];
    assert!(panics(|| {
        let mut config = WebPDecoderConfig::default();
        assert!(webp_init_decoder_config(&mut config));
        config.output.colorspace = WEBP_CSP_MODE::MODE_RGBA;
        config.output.u = WebPDecBufferPlanes::RGBA(WebPRGBABuffer {
            rgba: PlaneOffset::default(),
            stride: -32,
            size: 32 * 4,
        });
        config.output.storage = PlaneStorage::External(vec![&mut rows]);
        let _ = webp_decode(&image, &mut config);
    }));
    let wrapped = usize::try_from((-32_i32).cast_unsigned()).unwrap();
    assert_eq!(
        decode_into(
            &image,
            &rgba,
            Output::Rows {
                data: &mut rows,
                stride: wrapped
            }
        ),
        Err(DecodeError::InvalidStride {
            plane: Plane::Rows,
            stride: wrapped
        })
    );

    // A U plane shorter than its description.
    let (mut y, mut u, mut v) = (vec![0_u8; 32], vec![0_u8; 7], vec![0_u8; 8]);
    assert!(panics(|| {
        let mut config = WebPDecoderConfig::default();
        assert!(webp_init_decoder_config(&mut config));
        config.output.colorspace = WEBP_CSP_MODE::MODE_YUV;
        let plane = |region| PlaneOffset { region, offset: 0 };
        config.output.u = WebPDecBufferPlanes::YUVA(WebPYUVABuffer {
            y: plane(0),
            u: plane(1),
            v: plane(2),
            a: None,
            y_stride: 8,
            u_stride: 4,
            v_stride: 4,
            a_stride: 0,
            y_size: 32,
            u_size: 8,
            v_size: 8,
            a_size: 0,
        });
        config.output.storage = PlaneStorage::External(vec![&mut y, &mut u, &mut v]);
        let _ = webp_decode(&image, &mut config);
    }));
    let planes = YuvaPlanes {
        y: &mut y,
        y_stride: 8,
        u: &mut u,
        v: &mut v,
        uv_stride: 4,
        a: None,
        a_stride: 0,
    };
    assert_eq!(
        decode_into(&image, &Config::new(ColorMode::Yuv), Output::Planes(planes)),
        Err(DecodeError::BufferTooShort {
            plane: Plane::U,
            len: 7,
            needed: 8
        })
    );
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
}

/// A random value for a crop or scale field: small, at the limits, or huge.
fn dimension(rng: &mut Rng) -> u32 {
    match rng.below(6) {
        0 => 0,
        1..=3 => rng.below(24) as u32,
        4 => rng.pick(&[16383, 16384, 0x7fff_ffff, 0x8000_0000, u32::MAX]),
        _ => rng.below(70) as u32,
    }
}

/// A random stride: zero, small, at the limits or huge.
fn stride(rng: &mut Rng, row: usize) -> usize {
    match rng.below(8) {
        0 => 0,
        1 => row.saturating_sub(1),
        2 => rng.pick(&[0x7fff_ffff, 0x8000_0000, usize::MAX]),
        _ => row + rng.below(5) as usize,
    }
}

/// The API's own refusal of `config` before any data, by the rules of
/// ADR 0007.
fn config_rule(
    crop: Option<[u32; 4]>,
    scale: Option<[u32; 2]>,
    dither: [u8; 2],
) -> Option<DecodeError> {
    if let Some([left, top, width, height]) = crop
        && (width == 0
            || height == 0
            || left % 2 == 1
            || top % 2 == 1
            || [left, top, width, height].iter().any(|&v| v > 0x7fff_ffff))
    {
        return Some(DecodeError::InvalidCrop {
            left,
            top,
            width,
            height,
        });
    }
    if let Some([width, height]) = scale
        && (!(1..=16383).contains(&width) || !(1..=16383).contains(&height))
    {
        return Some(DecodeError::InvalidScale { width, height });
    }
    if dither[0] > 100 {
        return Some(DecodeError::InvalidParameter { name: "dithering" });
    }
    if dither[1] > 100 {
        return Some(DecodeError::InvalidParameter {
            name: "alpha_dithering",
        });
    }
    None
}

/// The API's refusal of one plane of caller memory.
fn plane_rule(
    plane: Plane,
    len: usize,
    stride: usize,
    row: usize,
    rows: usize,
) -> Option<DecodeError> {
    if stride < row || stride > 0x7fff_ffff {
        return Some(DecodeError::InvalidStride { plane, stride });
    }
    let needed = (rows - 1).checked_mul(stride)?.checked_add(row);
    match needed {
        None => Some(DecodeError::InvalidStride { plane, stride }),
        Some(needed) if len < needed => Some(DecodeError::BufferTooShort { plane, len, needed }),
        Some(_) => None,
    }
}

/// One random round: a configuration, data, an output and the calls.
fn round(rng: &mut Rng, files: &[Vec<u8>], stats: &mut [usize; 4]) {
    let mode = rng.pick(&MODES);
    let mut config = Config::new(mode);
    let crop = (rng.below(3) == 0).then(|| {
        [
            dimension(rng),
            dimension(rng),
            dimension(rng),
            dimension(rng),
        ]
    });
    if let Some([left, top, width, height]) = crop {
        config = config.crop(left, top, width, height);
    }
    let scale = (rng.below(3) == 0).then(|| [dimension(rng), dimension(rng)]);
    if let Some([width, height]) = scale {
        config = config.scale(width, height);
    }
    let dither = [rng.pick(&[0, 50, 100, 101, 255]), rng.pick(&[0, 100, 101])];
    config = config
        .dithering(dither[0])
        .alpha_dithering(dither[1])
        .flip(rng.below(2) == 0)
        .bypass_filtering(rng.below(4) == 0)
        .fancy_upsampling(rng.below(4) != 0)
        .multithreaded(rng.below(4) == 0);
    let rule = config_rule(crop, scale, dither);
    assert_eq!(config.validate().err(), rule, "{config:?}");

    let base = &files[rng.below(files.len() as u64) as usize];
    let data: Vec<u8> = match rng.below(6) {
        0 => base[..rng.below(base.len() as u64 + 1) as usize].to_vec(),
        1 => {
            let mut bytes = base.clone();
            let at = rng.below(bytes.len() as u64) as usize;
            bytes[at] ^= 1 << rng.below(8);
            bytes
        }
        _ => base.clone(),
    };
    let features = Features::parse(&data);
    let size = features.as_ref().ok().map(|f| config.output_size(f));

    // Library memory. Large scales would allocate up to 1 GiB per call;
    // those are checked through caller memory below, which is refused first.
    let small = scale.is_none_or(|[w, h]| u64::from(w) * u64::from(h) <= 4096);
    if small {
        let result = decode(&data, &config);
        if let Some(rule) = rule {
            assert_eq!(result.as_ref().err(), Some(&rule));
        }
        if let (Ok(image), Some(Ok((w, h)))) = (&result, &size) {
            assert_eq!((image.width(), image.height()), (*w, *h));
            stats[0] += 1;
        }
    }

    // Caller memory with random geometry.
    let (w, h) = match &size {
        Some(Ok((w, h))) => (*w as usize, *h as usize),
        _ => (1 + rng.below(40) as usize, 1 + rng.below(40) as usize),
    };
    let mut expected = None;
    let result = if mode.is_yuv() {
        let (uw, uh) = (w.div_ceil(2), h.div_ceil(2));
        let strides = [stride(rng, w), stride(rng, uw), stride(rng, w)];
        let lens = [
            (w * h + 8)
                .saturating_sub(rng.below(16) as usize)
                .min(1 << 16),
            (uw * uh + 4)
                .saturating_sub(rng.below(8) as usize)
                .min(1 << 16),
            (uw * uh + 4)
                .saturating_sub(rng.below(8) as usize)
                .min(1 << 16),
            (w * h + 8)
                .saturating_sub(rng.below(16) as usize)
                .min(1 << 16),
        ];
        let alpha = match rng.below(5) {
            0 => mode != ColorMode::Yuva,
            _ => mode == ColorMode::Yuva,
        };
        let mut memory: Vec<Vec<u8>> = lens.iter().map(|&len| vec![0x5a; len]).collect();
        let [y, u, v, a] = &mut memory[..] else {
            unreachable!()
        };
        if size.as_ref().is_some_and(Result::is_ok)
            && rule.is_none()
            && features.as_ref().is_ok_and(|f| !f.has_animation)
        {
            expected = if alpha != (mode == ColorMode::Yuva) {
                Some(DecodeError::OutputMismatch { mode })
            } else {
                plane_rule(Plane::Y, lens[0], strides[0], w, h)
                    .or_else(|| plane_rule(Plane::U, lens[1], strides[1], uw, uh))
                    .or_else(|| plane_rule(Plane::V, lens[2], strides[1], uw, uh))
                    .or_else(|| {
                        if alpha {
                            plane_rule(Plane::A, lens[3], strides[2], w, h)
                        } else {
                            None
                        }
                    })
            };
        }
        let output = Output::Planes(YuvaPlanes {
            y,
            y_stride: strides[0],
            u,
            v,
            uv_stride: strides[1],
            a: alpha.then_some(&mut a[..]),
            a_stride: strides[2],
        });
        decode_into(&data, &config, output)
    } else {
        let bpp = mode.bytes_per_pixel().unwrap();
        let row = w * bpp;
        let stride = stride(rng, row);
        let short = if rng.below(2) == 0 {
            0
        } else {
            rng.below(40) as usize
        };
        let len = (row * h + 16).saturating_sub(short).min(1 << 18);
        let mut memory = vec![0x5a_u8; len];
        if size.as_ref().is_some_and(Result::is_ok)
            && rule.is_none()
            && features.as_ref().is_ok_and(|f| !f.has_animation)
        {
            expected = plane_rule(Plane::Rows, len, stride, row, h);
        }
        decode_into(
            &data,
            &config,
            Output::Rows {
                data: &mut memory,
                stride,
            },
        )
    };
    if let Some(rule) = rule {
        assert_eq!(result, Err(rule));
    } else if let Some(expected) = expected {
        assert_eq!(result, Err(expected), "{config:?}");
        stats[1] += 1;
    } else if result.is_ok() {
        stats[2] += 1;
    }

    // An incremental decoding in random pieces, with append or update.
    if small {
        let decoder = IncrementalDecoder::new(&config);
        if let Some(rule) = rule {
            assert_eq!(decoder.as_ref().err(), Some(&rule));
        }
        if let Ok(mut decoder) = decoder {
            let update = rng.below(2) == 0;
            let mut pos = 0;
            while pos < data.len() {
                // Empty pieces now and then, the first one included.
                let piece = if rng.below(8) == 0 {
                    0
                } else {
                    1 + rng.below(300) as usize
                };
                let next = (pos + piece).min(data.len());
                let status = if update {
                    decoder.update(&data[..next])
                } else {
                    decoder.append(&data[pos..next])
                };
                pos = next;
                if let Some(rows) = decoder.rows() {
                    assert_eq!(rows.len(), decoder.decoded_rows());
                    assert_eq!(rows.iter().count(), rows.len() as usize);
                }
                if let Some(planes) = decoder.planes() {
                    assert_eq!(planes.y.iter().count(), planes.y.len() as usize);
                }
                if status != Ok(Status::NeedsMoreData) {
                    break;
                }
            }
            if decoder.into_image().is_ok() {
                stats[3] += 1;
            }
        }
    }

    // The views.
    if let Ok(container) = Container::partial(&data) {
        let _ = (
            container.icc_profile(),
            container.exif(),
            container.xmp(),
            container.chunks(*b"ICCP").count(),
        );
    }
    let options = AnimationOptions {
        mode: rng.pick(&[
            AnimationMode::Rgba,
            AnimationMode::Bgra,
            AnimationMode::PremultipliedRgba,
            AnimationMode::PremultipliedBgra,
        ]),
        multithreaded: rng.below(2) == 0,
    };
    if let Ok(mut decoder) = AnimationDecoder::new(&data, options) {
        let info: AnimationInfo = decoder.info();
        for _ in 0..=info.frame_count {
            match decoder.next_frame() {
                Ok(Some(frame)) => assert_eq!(
                    frame.canvas.len(),
                    frame.stride * info.canvas_height as usize
                ),
                Ok(None) | Err(_) => break,
            }
        }
        decoder.reset();
    }
}

#[test]
fn seeded_misuse_never_panics() {
    let mut files = Vec::new();
    for (width, height, alpha, lossless) in [
        (1, 1, false, true),
        (7, 5, true, true),
        (16, 9, false, false),
        (33, 17, true, false),
        (4, 30, true, true),
    ] {
        files.push(file(width, height, alpha, lossless));
    }
    files.push(animation(false));
    files.push(animation(true));
    let mut rng = Rng(0x0016_5eed);
    let mut stats = [0; 4];
    for index in 0..2000 {
        let result = panic::catch_unwind(AssertUnwindSafe(|| round(&mut rng, &files, &mut stats)));
        assert!(result.is_ok(), "round {index} panicked");
    }
    eprintln!(
        "{} library decodings, {} refusals of caller memory by the rules, {} caller decodings, {} incremental images",
        stats[0], stats[1], stats[2], stats[3]
    );
    assert!(stats.iter().all(|&count| count > 20), "{stats:?}");
}

#[test]
fn public_types_are_send_and_sync() {
    fn send_sync<T: Send + Sync>() {}
    send_sync::<Config>();
    send_sync::<Features>();
    send_sync::<Image>();
    send_sync::<DecodeError>();
    send_sync::<Container<'static>>();
    send_sync::<AnimationDecoder<'static>>();
    send_sync::<Output<'static>>();
}
