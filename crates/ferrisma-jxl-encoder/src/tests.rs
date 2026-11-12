use jxl::jxl::encode::{JxlEncoderAdmission, take_admissions};
use jxl_oxide::color::{ColourEncoding, ColourSpace, Primaries, TransferFunction, WhitePoint};
use jxl_oxide::image::BitDepth;
use jxl_oxide::{ExtraChannelType, JxlImage};

use super::{
    Admission, Alpha, Color, ColorChannels, Encoder, Error, FrameOptions, ImageInfo, Stage, encode,
    estimated_codec_bytes,
};

const LIMIT: usize = 1 << 26;

/// Photo-like 8-bit content: gradients plus a hashed texture.
fn photo(width: usize, height: usize, channels: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * channels);
    for y in 0..height {
        for x in 0..width {
            let hash = (x as u32).wrapping_mul(2_654_435_761) ^ (y as u32).wrapping_mul(40_503);
            let noise = (hash >> 13) as u8 & 0x0f;
            let rgb = [
                ((x * 255) / width.max(2)) as u8 ^ noise,
                ((y * 255) / height.max(2)) as u8,
                ((x + y) * 3) as u8 ^ (noise << 2),
            ];
            data.extend_from_slice(&rgb[..channels]);
        }
    }
    data
}

fn decode(bytes: &[u8]) -> JxlImage {
    JxlImage::builder()
        .read(std::io::Cursor::new(bytes))
        .unwrap()
}

#[test]
fn encodes_xyb_with_enum_srgb_signaling() {
    let (width, height) = (61, 37);
    let mut out = Vec::new();
    let written = encode(
        ImageInfo::srgb8(61, 37),
        FrameOptions::default(),
        &photo(width, height, 3),
        &mut out,
        LIMIT,
    )
    .unwrap();
    assert_eq!(written, out.len());
    assert_eq!(&out[..2], &[0xff, 0x0a], "raw codestream");
    let image = decode(&out);
    assert_eq!((image.width(), image.height()), (61, 37));
    let metadata = &image.image_header().metadata;
    assert!(metadata.xyb_encoded);
    assert_eq!(metadata.orientation, 1);
    assert!(metadata.ec_info.is_empty());
    let ColourEncoding::Enum(color) = &metadata.colour_encoding else {
        panic!("expected enum color");
    };
    assert_eq!(color.colour_space, ColourSpace::Rgb);
    assert_eq!(color.white_point, WhitePoint::D65);
    assert_eq!(color.primaries, Primaries::Srgb);
    assert_eq!(color.tf, TransferFunction::Srgb);
}

#[test]
fn encodes_gray_and_linear_srgb() {
    let mut out = Vec::new();
    let image = ImageInfo {
        width: 20,
        height: 9,
        channels: ColorChannels::Gray,
        color: Color::LinearSrgb,
        alpha: Alpha::None,
    };
    encode(
        image,
        FrameOptions::with_distance(0.5),
        &photo(20, 9, 1),
        &mut out,
        LIMIT,
    )
    .unwrap();
    let decoded = decode(&out);
    let ColourEncoding::Enum(color) = &decoded.image_header().metadata.colour_encoding else {
        panic!("expected enum color");
    };
    assert_eq!(color.colour_space, ColourSpace::Grey);
    assert_eq!(color.tf, TransferFunction::Linear);
}

#[test]
fn the_frame_is_copied_and_owned_objects_drop_at_every_step() {
    let pixels = photo(16, 16, 3);
    let mut reference = Vec::new();
    encode(
        ImageInfo::srgb8(16, 16),
        FrameOptions::default(),
        &pixels,
        &mut reference,
        LIMIT,
    )
    .unwrap();
    // The caller's buffer can change after add_frame: the port copied it.
    let mut changing = pixels.clone();
    let mut encoder = Encoder::new().unwrap();
    encoder.set_image(ImageInfo::srgb8(16, 16)).unwrap();
    encoder.set_frame_options(FrameOptions::default()).unwrap();
    encoder.add_frame(&changing).unwrap();
    changing.fill(0);
    drop(changing);
    encoder.close_input();
    let mut out = vec![1, 2, 3];
    encoder.process_output_into(&mut out, LIMIT).unwrap();
    assert_eq!(&out[..3], &[1, 2, 3], "existing bytes are kept");
    assert_eq!(&out[3..], &reference[..]);
    // A finished encoder accepts no further work.
    assert!(matches!(
        encoder.process_output_into(&mut out, LIMIT),
        Err(Error::Misuse(_))
    ));
    // Drop at every intermediate state releases the encoder, its frame
    // settings and the queued frame copy (checked under Miri/ASan by the
    // port; here it must simply not fail).
    drop(Encoder::new().unwrap());
    let mut partial = Encoder::new().unwrap();
    partial.set_image(ImageInfo::srgb8(16, 16)).unwrap();
    drop(partial);
    let mut queued = Encoder::new().unwrap();
    queued.set_image(ImageInfo::srgb8(16, 16)).unwrap();
    queued.add_frame(&pixels).unwrap();
    drop(queued);
}

#[test]
fn bridge_misuse_is_rejected_before_the_port() {
    let mut encoder = Encoder::new().unwrap();
    assert_eq!(
        encoder.add_frame(&[0; 3]),
        Err(Error::Misuse("set the image before adding a frame"))
    );
    assert!(matches!(
        encoder.set_image(ImageInfo::srgb8(0, 4)),
        Err(Error::Misuse(_))
    ));
    encoder.set_image(ImageInfo::srgb8(2, 2)).unwrap();
    assert!(matches!(
        encoder.set_image(ImageInfo::srgb8(2, 2)),
        Err(Error::Misuse(_))
    ));
    for distance in [f32::NAN, f32::INFINITY, 0.0, -1.0] {
        assert!(matches!(
            encoder.set_frame_options(FrameOptions::with_distance(distance)),
            Err(Error::Misuse(_))
        ));
    }
    assert!(matches!(encoder.add_frame(&[0; 11]), Err(Error::Misuse(_))));
    encoder.add_frame(&[0; 12]).unwrap();
    assert!(matches!(encoder.add_frame(&[0; 12]), Err(Error::Misuse(_))));
    let mut out = Vec::new();
    assert!(matches!(
        encoder.process_output_into(&mut out, LIMIT),
        Err(Error::Misuse(_))
    ));
}

#[test]
fn port_api_usage_errors_are_typed_and_poison_the_encoder() {
    let mut encoder = Encoder::new().unwrap();
    encoder.set_image(ImageInfo::srgb8(4, 4)).unwrap();
    assert_eq!(
        encoder.set_frame_options(FrameOptions::with_distance(25.5)),
        Err(Error::ApiUsage(Stage::FrameSettings))
    );
    assert_eq!(
        encoder.set_frame_options(FrameOptions::default()),
        Err(Error::Misuse("the encoder already failed"))
    );
    // Level 10 limits: more than 2^30 pixels per side.
    let mut encoder = Encoder::new().unwrap();
    assert_eq!(
        encoder.set_image(ImageInfo::srgb8((1 << 30) + 1, 1)),
        Err(Error::ApiUsage(Stage::BasicInfo))
    );
}

#[test]
fn distances_outside_the_profile_are_frame_profile_admissions() {
    for distance in [3.0, 4.5] {
        let _ = take_admissions();
        let mut out = vec![9];
        let error = encode(
            ImageInfo::srgb8(32, 32),
            FrameOptions::with_distance(distance),
            &photo(32, 32, 3),
            &mut out,
            LIMIT,
        )
        .unwrap_err();
        assert_eq!(error, Error::NotAdmitted(Admission::FrameProfile));
        assert_eq!(take_admissions(), [JxlEncoderAdmission::FrameProfile]);
        assert_eq!(out, [9], "output restored");
    }
}

#[test]
fn text_like_content_is_a_patch_dictionary_admission_only_with_patches_on() {
    // Repeated glyphs: with the patch search on (libjxl's default),
    // FindTextLikePatches finds patches.
    for (width, height, glyph, copies, seed) in [(64, 64, 5, 6, 1), (128, 128, 9, 2, 6)] {
        let pixels = glyphs(width, height, glyph, copies, seed);
        let image = ImageInfo::srgb8(width as u32, height as u32);
        let _ = take_admissions();
        let mut out = Vec::new();
        let error = encode(
            image,
            FrameOptions::default().with_patches(true),
            &pixels,
            &mut out,
            LIMIT,
        )
        .unwrap_err();
        assert_eq!(error, Error::NotAdmitted(Admission::PatchDictionary));
        assert_eq!(take_admissions(), [JxlEncoderAdmission::PatchDictionary]);
        assert!(out.is_empty());
        assert!(error.to_string().contains("patch"));
        // The bridge's default is patches off: the port skips the search and
        // the same content encodes, with no admission result.
        assert!(!FrameOptions::default().patches);
        let mut out = Vec::new();
        encode(image, FrameOptions::default(), &pixels, &mut out, LIMIT).unwrap();
        assert!(take_admissions().is_empty());
        let decoded = decode(&out);
        assert_eq!(
            (decoded.width(), decoded.height()),
            (width as u32, height as u32)
        );
    }
    // A single glyph repeats nothing: no patches, so the frame encodes even
    // with the search on.
    let _ = take_admissions();
    let mut out = Vec::new();
    encode(
        ImageInfo::srgb8(128, 128),
        FrameOptions::default().with_patches(true),
        &glyphs(128, 128, 5, 1, 7),
        &mut out,
        LIMIT,
    )
    .unwrap();
    assert!(take_admissions().is_empty());
}

/// Photo-like color with an alpha ramp that has fully transparent stripes,
/// interleaved after `color_channels` color samples.
fn with_alpha(width: usize, height: usize, color_channels: usize) -> Vec<u8> {
    let color = photo(width, height, color_channels);
    let mut data = Vec::with_capacity(width * height * (color_channels + 1));
    for (i, pixel) in color.chunks_exact(color_channels).enumerate() {
        let (x, y) = (i % width, i / width);
        let alpha = if (x / 3) % 4 == 0 {
            0
        } else {
            ((x * 7 + y * 3) % 256) as u8
        };
        data.extend_from_slice(pixel);
        data.push(alpha);
    }
    data
}

/// Decode and return the interleaved 8-bit samples, checking that alpha is
/// one straight 8-bit channel.
fn decode_samples(bytes: &[u8], channels: usize) -> Vec<u8> {
    let image = decode(bytes);
    let metadata = &image.image_header().metadata;
    assert!(metadata.xyb_encoded);
    assert_eq!(metadata.ec_info.len(), 1);
    assert_eq!(
        metadata.ec_info[0].ty,
        ExtraChannelType::Alpha {
            alpha_associated: false
        }
    );
    assert_eq!(
        metadata.ec_info[0].bit_depth,
        BitDepth::IntegerSample { bits_per_sample: 8 }
    );
    let render = image.render_frame(0).unwrap();
    let mut stream = render.stream();
    assert_eq!(stream.channels() as usize, channels);
    let mut samples = vec![0_u8; (stream.width() * stream.height()) as usize * channels];
    stream.write_to_buffer(&mut samples);
    samples
}

#[test]
fn straight_8_bit_alpha_is_coded_losslessly_without_an_admission_result() {
    // RGBA and gray+alpha, one-shot (up to 2x2 groups) and streaming (18
    // groups); text-like content with alpha encodes too with patches off.
    for (width, height, channels) in [
        (1, 1, ColorChannels::Rgb),
        (61, 37, ColorChannels::Rgb),
        (300, 260, ColorChannels::Rgb),
        (2049, 300, ColorChannels::Rgb),
        (61, 37, ColorChannels::Gray),
        (300, 260, ColorChannels::Gray),
    ] {
        let color_channels = if channels == ColorChannels::Gray {
            1
        } else {
            3
        };
        let image = ImageInfo {
            channels,
            ..ImageInfo::srgba8(width as u32, height as u32)
        };
        let pixels = with_alpha(width, height, color_channels);
        let _ = take_admissions();
        let mut out = Vec::new();
        encode(image, FrameOptions::default(), &pixels, &mut out, LIMIT).unwrap();
        assert!(take_admissions().is_empty());
        let samples = decode_samples(&out, color_channels + 1);
        let stride = color_channels + 1;
        let alpha =
            |data: &[u8]| -> Vec<u8> { data.chunks_exact(stride).map(|p| p[stride - 1]).collect() };
        assert_eq!(
            alpha(&samples),
            alpha(&pixels),
            "{width}x{height} {channels:?}: alpha is exact"
        );
    }
    let mut text = Vec::new();
    for rgb in glyphs(64, 64, 5, 6, 1).as_chunks::<3>().0 {
        text.extend_from_slice(rgb);
        text.push(if rgb[0] < 128 { 255 } else { 96 });
    }
    let mut out = Vec::new();
    encode(
        ImageInfo::srgba8(64, 64),
        FrameOptions::default(),
        &text,
        &mut out,
        LIMIT,
    )
    .unwrap();
    let samples = decode_samples(&out, 4);
    assert!(
        samples
            .as_chunks::<4>()
            .0
            .iter()
            .zip(text.as_chunks::<4>().0)
            .all(|(a, b)| a[3] == b[3])
    );
}

#[test]
fn fully_opaque_alpha_is_stripped_by_the_port() {
    // The port's default STRIP_ALPHA removes an opaque alpha channel; the
    // codestream then equals the RGB encode of the same color.
    let rgb = photo(40, 30, 3);
    let rgba: Vec<u8> = rgb
        .as_chunks::<3>()
        .0
        .iter()
        .flat_map(|p| [p[0], p[1], p[2], 255])
        .collect();
    let mut with_alpha = Vec::new();
    encode(
        ImageInfo::srgba8(40, 30),
        FrameOptions::default(),
        &rgba,
        &mut with_alpha,
        LIMIT,
    )
    .unwrap();
    let mut without = Vec::new();
    encode(
        ImageInfo::srgb8(40, 30),
        FrameOptions::default(),
        &rgb,
        &mut without,
        LIMIT,
    )
    .unwrap();
    assert!(
        decode(&with_alpha)
            .image_header()
            .metadata
            .ec_info
            .is_empty()
    );
    assert_eq!(with_alpha, without);
}

#[test]
fn alpha_buffers_must_hold_the_alpha_samples() {
    let mut encoder = Encoder::new().unwrap();
    encoder.set_image(ImageInfo::srgba8(2, 2)).unwrap();
    // An RGB-sized buffer is not one RGBA frame.
    assert!(matches!(encoder.add_frame(&[0; 12]), Err(Error::Misuse(_))));
    encoder.add_frame(&[0; 16]).unwrap();
    let mut encoder = Encoder::new().unwrap();
    encoder
        .set_image(ImageInfo {
            channels: ColorChannels::Gray,
            ..ImageInfo::srgba8(3, 2)
        })
        .unwrap();
    assert!(matches!(encoder.add_frame(&[0; 6]), Err(Error::Misuse(_))));
    encoder.add_frame(&[0; 12]).unwrap();
}

#[test]
fn output_limit_applies_at_the_exact_byte() {
    let pixels = photo(40, 30, 3);
    let mut full = Vec::new();
    let length = encode(
        ImageInfo::srgb8(40, 30),
        FrameOptions::default(),
        &pixels,
        &mut full,
        LIMIT,
    )
    .unwrap();
    let mut exact = Vec::new();
    encode(
        ImageInfo::srgb8(40, 30),
        FrameOptions::default(),
        &pixels,
        &mut exact,
        length,
    )
    .unwrap();
    assert_eq!(exact, full);
    let mut short = vec![7];
    assert_eq!(
        encode(
            ImageInfo::srgb8(40, 30),
            FrameOptions::default(),
            &pixels,
            &mut short,
            length - 1,
        ),
        Err(Error::OutputLimit { limit: length - 1 })
    );
    assert_eq!(short, [7]);
}

#[test]
fn large_images_take_the_streaming_path() {
    // 9 x 2 groups of 256x256: more than 8, so CanDoStreamingEncoding holds.
    let (width, height) = (2049, 300);
    let image = ImageInfo::srgb8(2049, 300);
    assert!(image.streaming());
    assert!(!ImageInfo::srgb8(1024, 512).streaming());
    let mut out = Vec::new();
    encode(
        image,
        FrameOptions::default(),
        &photo(width, height, 3),
        &mut out,
        LIMIT,
    )
    .unwrap();
    let decoded = decode(&out);
    assert_eq!((decoded.width(), decoded.height()), (2049, 300));
}

#[test]
fn the_memory_estimate_counts_the_input_copy_and_codec_planes() {
    let small = ImageInfo::srgb8(9, 5);
    // 135 input bytes, 16x8 padded x 7 f32 planes, one group of coefficients.
    assert_eq!(
        estimated_codec_bytes(&small),
        Some(9 * 5 * 3 + 16 * 8 * 28 + 3 * 256 * 256 * 4)
    );
    let large = ImageInfo::srgb8(4096, 4096);
    let per_pixel = estimated_codec_bytes(&large).unwrap() / (4096 * 4096);
    // Streaming keeps the whole input copy but only one group of planes.
    assert!((3..8).contains(&per_pixel), "{per_pixel}");
    assert_eq!(
        estimated_codec_bytes(&ImageInfo::srgb8(u32::MAX, u32::MAX)),
        None
    );
    // Alpha: one more input byte per pixel, and an f32 plane plus two i32
    // Modular copies of the encoded area (the frame one-shot, one bordered
    // group streaming).
    assert_eq!(
        estimated_codec_bytes(&ImageInfo::srgba8(9, 5)),
        Some(9 * 5 * 4 + 16 * 8 * 28 + 3 * 256 * 256 * 4 + 9 * 5 * 12)
    );
    let rgb = estimated_codec_bytes(&ImageInfo::srgb8(4096, 4096)).unwrap();
    assert_eq!(
        estimated_codec_bytes(&ImageInfo::srgba8(4096, 4096)),
        Some(rgb + 4096 * 4096 + 272 * 272 * 12)
    );
}

/// Text-like content: a flat light background with `copies` of one small
/// dark glyph, as in the port's patch-detection parity corpus (`glyph_image`
/// in `tests/parity/encoder_coefficients.rs`, quantized to 8 bits).
fn glyphs(xsize: usize, ysize: usize, glyph: usize, copies: usize, seed: u64) -> Vec<u8> {
    let mut s = seed.wrapping_mul(0x9e37_79b9_7f4a_7c15) | 1;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut data = [242u8, 237, 230].repeat(xsize * ysize);
    let shape: Vec<bool> = (0..glyph * glyph).map(|_| rnd() % 3 != 0).collect();
    let span = xsize.saturating_sub(glyph + 8).max(1);
    for k in 0..copies {
        let gx = 4 + (k * (glyph + 6)) % span;
        let gy = 4 + ((k * (glyph + 6)) / span) * (glyph + 6);
        if gy + glyph + 4 >= ysize {
            break;
        }
        for y in 0..glyph {
            for x in 0..glyph {
                if shape[y * glyph + x] {
                    let p = ((gy + y) * xsize + gx + x) * 3;
                    data[p..p + 3].copy_from_slice(&[26, 38, 51]);
                }
            }
        }
    }
    data
}
