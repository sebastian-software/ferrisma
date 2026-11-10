//! WP9 differential test: picture initialization, validation, allocation and freeing
//! (`src/enc/picture_enc.c`, `WebPPictureIsView` in `src/enc/picture_rescale_enc.c`),
//! the `WebPPictureImport*` functions (`src/enc/picture_csp_enc.c`), the ARGB to YUVA
//! conversions with and without dithering and sharp YUV, and YUVA to ARGB. The probes
//! `tools/oracle/probes/wp9/picture_alloc.c` and `picture_csp.c` run the C functions
//! in separate processes; this test runs the same calls on the Rust translation and
//! compares the return values, the error codes, the strides, the null flags and the
//! visible samples of the planes.
//!
//! Not applicable: samples that the library allocates and has not written yet. The
//! import and conversion tests compare only the visible samples of the planes, never
//! the padding, and the probes print the caller's buffers whole only where the caller
//! supplied them.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::enc::picture_csp_enc::{
    webp_picture_argb_to_yuva, webp_picture_argb_to_yuva_dithered, webp_picture_import_bgr,
    webp_picture_import_bgra, webp_picture_import_bgrx, webp_picture_import_rgb,
    webp_picture_import_rgba, webp_picture_import_rgbx, webp_picture_sharp_argb_to_yuva,
    webp_picture_yuva_to_argb,
};
use webp::enc::picture_enc::{
    webp_picture_alloc, webp_picture_free, webp_picture_init_internal, webp_validate_picture,
};
use webp::enc::picture_rescale_enc::webp_picture_is_view;
use webp::webp::encode::{WEBP_ENCODER_ABI_VERSION, WebPEncCSP, WebPPicture, webp_picture_init};
use webp::webp::types::{PlaneOffset, PlaneStorage};

/// SplitMix64, a small deterministic generator for the cases.
struct Rng(u64);

impl Rng {
    fn next_u64(&mut self) -> u64 {
        self.0 = self.0.wrapping_add(0x9e37_79b9_7f4a_7c15);
        let mut z = self.0;
        z = (z ^ (z >> 30)).wrapping_mul(0xbf58_476d_1ce4_e5b9);
        z = (z ^ (z >> 27)).wrapping_mul(0x94d0_49bb_1331_11eb);
        z ^ (z >> 31)
    }

    fn bytes(&mut self, len: usize) -> Vec<u8> {
        (0..len).map(|_| self.next_u64() as u8).collect()
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// The visible samples of a plane: `height` rows of `width` bytes, `stride` apart.
fn visible(samples: &[u8], width: usize, height: usize, stride: usize) -> Vec<u8> {
    (0..height)
        .flat_map(|row| samples[row * stride..row * stride + width].iter().copied())
        .collect()
}

/// The samples of a plane, from its offset to the end of its region.
fn plane_samples<'a>(storage: &'a PlaneStorage<'_, u8>, plane: PlaneOffset) -> &'a [u8] {
    &storage.region(plane.region).expect("the region exists")[plane.offset..]
}

/// The visible ARGB words of a plane as native-order bytes.
fn visible_argb(
    storage: &PlaneStorage<'_, u32>,
    plane: PlaneOffset,
    width: usize,
    height: usize,
    stride: usize,
) -> Vec<u8> {
    let words = &storage.region(plane.region).expect("the region exists")[plane.offset..];
    (0..height)
        .flat_map(|row| {
            words[row * stride..row * stride + width]
                .iter()
                .flat_map(|word| word.to_ne_bytes())
        })
        .collect()
}

/// The state of a picture as the probe prints it, under `prefix`.
fn picture_fields(prefix: &str, pic: &WebPPicture<'_>) -> Fields {
    Fields::new()
        .with(&format!("{prefix}use_argb"), pic.use_argb)
        .with(&format!("{prefix}colorspace"), pic.colorspace.0)
        .with(&format!("{prefix}width"), pic.width)
        .with(&format!("{prefix}height"), pic.height)
        .with(&format!("{prefix}y_stride"), pic.y_stride)
        .with(&format!("{prefix}uv_stride"), pic.uv_stride)
        .with(&format!("{prefix}a_stride"), pic.a_stride)
        .with(&format!("{prefix}argb_stride"), pic.argb_stride)
        .with(&format!("{prefix}y_null"), i32::from(pic.y.is_none()))
        .with(&format!("{prefix}u_null"), i32::from(pic.u.is_none()))
        .with(&format!("{prefix}v_null"), i32::from(pic.v.is_none()))
        .with(&format!("{prefix}a_null"), i32::from(pic.a.is_none()))
        .with(&format!("{prefix}argb_null"), i32::from(pic.argb.is_none()))
        .with(&format!("{prefix}error"), pic.error_code.0)
        .with(
            &format!("{prefix}is_view"),
            i32::from(webp_picture_is_view(pic)),
        )
}

#[test]
fn alloc_and_free_match_the_c_picture() {
    let Some(oracle) = Oracle::locate("alloc_and_free_match_the_c_picture") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_alloc");

    let cases: [(i32, u32, i32, i32); 14] = [
        (0, 0, 16, 8),
        (0, 4, 16, 8),
        (0, 0, 1, 1),
        (0, 4, 3, 5),
        (0, 0, 0, 4),
        (0, 0, -1, 4),
        (0, 1, 4, 4),
        (1, 0, 4, 3),
        (1, 4, 4, 3),
        (1, 4, 7, 1),
        (1, 0, 1, 9),
        (0, 0, 5, 0),
        (1, 2, 4, 4),
        (0, 4, 33, 17),
    ];
    let mut stdin = String::new();
    let mut expected = Fields::new();
    for (use_argb, colorspace, width, height) in cases {
        stdin.push_str(&format!("alloc {use_argb} {colorspace} {width} {height}\n"));
        let mut pic = WebPPicture::default();
        webp_picture_init(&mut pic);
        pic.use_argb = use_argb;
        pic.colorspace = WebPEncCSP(colorspace);
        pic.width = width;
        pic.height = height;
        let ret = webp_picture_alloc(&mut pic);
        expected = expected.with("ret", i32::from(ret.is_ok()));
        expected.0.extend(picture_fields("", &pic).0);
        webp_picture_free(&mut pic);
        expected.0.extend(picture_fields("freed_", &pic).0);
    }

    // Allocation limits: these sizes fail before any memory is taken (ADR 0003).
    for (use_argb, colorspace, width, height) in [(1, 0, 65_536, 65_536), (0, 4, 100_000, 100_000)]
    {
        stdin.push_str(&format!("alloc {use_argb} {colorspace} {width} {height}\n"));
        let mut pic = WebPPicture::default();
        webp_picture_init(&mut pic);
        pic.use_argb = use_argb;
        pic.colorspace = WebPEncCSP(colorspace);
        pic.width = width;
        pic.height = height;
        let ret = webp_picture_alloc(&mut pic);
        expected = expected.with("ret", i32::from(ret.is_ok()));
        expected.0.extend(picture_fields("", &pic).0);
        webp_picture_free(&mut pic);
        expected.0.extend(picture_fields("freed_", &pic).0);
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn validation_and_initialization_match_the_c_picture() {
    let Some(oracle) = Oracle::locate("validation_and_initialization_match_the_c_picture") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_alloc");

    let mut stdin = String::new();
    let mut expected = Fields::new();

    for (use_argb, colorspace, width, height) in [
        (0, 0, 1, 1),
        (0, 4, 1, 1),
        (1, 0, 1, 1),
        (1, 1, 1, 1),
        (0, 2, 3, 3),
        (0, 0, 0, 1),
        (0, 0, 1, 0),
        (0, 0, -3, 2),
        (1, 4, 2, -2),
        (0, 0, 16_384, 16_384),
    ] {
        stdin.push_str(&format!(
            "validate {use_argb} {colorspace} {width} {height}\n"
        ));
        let mut pic = WebPPicture::default();
        webp_picture_init(&mut pic);
        pic.use_argb = use_argb;
        pic.colorspace = WebPEncCSP(colorspace);
        pic.width = width;
        pic.height = height;
        let ret = webp_validate_picture(&mut pic);
        expected = expected
            .with("ret", i32::from(ret.is_ok()))
            .with("error", pic.error_code.0);
    }

    for version in [WEBP_ENCODER_ABI_VERSION, 0x0211, 0x0100] {
        stdin.push_str(&format!("init {version}\n"));
        let mut pic = WebPPicture::default();
        let ret = webp_picture_init_internal(&mut pic, version);
        expected = expected.with("ret", i32::from(ret));
        if ret {
            expected.0.extend(picture_fields("", &pic).0);
        }
    }

    stdin.push_str("init_wrapper\n");
    let mut pic = WebPPicture::default();
    let ret = webp_picture_init(&mut pic);
    expected = expected.with("ret", i32::from(ret));
    expected.0.extend(picture_fields("", &pic).0);

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

/// The output of the probe's `PrintYuva` for a picture that the import or conversion
/// left: the colorspace, the ARGB samples when the picture is ARGB, or the YUVA planes.
fn yuva_output(pic: &WebPPicture<'_>, width: usize, height: usize) -> Fields {
    let uv_width = width.div_ceil(2);
    let uv_height = height.div_ceil(2);
    let mut fields = Fields::new()
        .with("colorspace", pic.colorspace.0)
        .with("use_argb", pic.use_argb);
    if pic.use_argb != 0 {
        let plane = pic.argb.expect("the ARGB plane exists");
        let bytes = visible_argb(
            &pic.argb_storage,
            plane,
            width,
            height,
            usize::try_from(pic.argb_stride).expect("the stride is positive"),
        );
        return fields.hex("argb", &bytes);
    }
    let y = pic.y.expect("the luma plane exists");
    let u = pic.u.expect("the U plane exists");
    let v = pic.v.expect("the V plane exists");
    fields = fields
        .hex(
            "y",
            &visible(
                plane_samples(&pic.yuva, y),
                width,
                height,
                usize::try_from(pic.y_stride).expect("positive"),
            ),
        )
        .hex(
            "u",
            &visible(
                plane_samples(&pic.yuva, u),
                uv_width,
                uv_height,
                usize::try_from(pic.uv_stride).expect("positive"),
            ),
        )
        .hex(
            "v",
            &visible(
                plane_samples(&pic.yuva, v),
                uv_width,
                uv_height,
                usize::try_from(pic.uv_stride).expect("positive"),
            ),
        );
    if let Some(a) = pic.a {
        fields = fields.hex(
            "a",
            &visible(
                plane_samples(&pic.yuva, a),
                width,
                height,
                usize::try_from(pic.a_stride).expect("positive"),
            ),
        );
    }
    fields
}

#[test]
fn importers_match_the_c_importers_on_padded_strides() {
    let Some(oracle) = Oracle::locate("importers_match_the_c_importers_on_padded_strides") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_csp");

    let kinds = ["rgb", "rgba", "bgr", "bgra", "rgbx", "bgrx"];
    let dims = [(1, 1), (2, 3), (3, 2), (5, 4), (7, 5), (16, 3)];
    let mut rng = Rng(0x5eed_0009);
    let mut stdin = String::new();
    let mut expected = Fields::new();

    for kind in kinds {
        let bytes_per_pixel = match kind {
            "rgb" | "bgr" => 3,
            _ => 4,
        };
        for (width, height) in dims {
            for pad in [0_usize, 3] {
                for use_argb in [0, 1] {
                    let stride = bytes_per_pixel * width + pad;

                    // Upstream reads RGBA rows through a `uint32_t*` cast, which is undefined when a
                    // row starts off a 4-byte boundary (UBSan reports it). These inputs are excluded
                    // from the oracle comparison (`src/enc/picture_csp_enc.rs`,
                    // `# Excluded undefined behavior`).
                    if kind == "rgba" && use_argb == 1 && stride % 4 != 0 && height > 1 {
                        continue;
                    }
                    let input = rng.bytes(stride * height);
                    stdin.push_str(&format!(
                        "import {kind} {use_argb} {width} {height} {stride} {}\n",
                        hex(&input)
                    ));
                    let mut pic = WebPPicture {
                        use_argb,
                        width: i32::try_from(width).unwrap(),
                        height: i32::try_from(height).unwrap(),
                        ..WebPPicture::default()
                    };
                    let stride_i32 = i32::try_from(stride).unwrap();
                    let ok = match kind {
                        "rgb" => webp_picture_import_rgb(&mut pic, &input, stride_i32),
                        "rgba" => webp_picture_import_rgba(&mut pic, &input, stride_i32),
                        "bgr" => webp_picture_import_bgr(&mut pic, &input, stride_i32),
                        "bgra" => webp_picture_import_bgra(&mut pic, &input, stride_i32),
                        "rgbx" => webp_picture_import_rgbx(&mut pic, &input, stride_i32),
                        _ => webp_picture_import_bgrx(&mut pic, &input, stride_i32),
                    };
                    expected = expected
                        .with("ret", i32::from(ok))
                        .with("error", pic.error_code.0);
                    expected.0.extend(yuva_output(&pic, width, height).0);
                    webp_picture_free(&mut pic);
                }
            }
        }
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn argb_to_yuva_conversions_match_the_c_conversions() {
    let Some(oracle) = Oracle::locate("argb_to_yuva_conversions_match_the_c_conversions") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_csp");

    let mut rng = Rng(0x5eed_0010);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    // mode, colorspace, dithering, width, height, stride
    let cases: [(u32, u32, f32, usize, usize, usize); 12] = [
        (0, 0, 0.0, 4, 4, 4),
        (0, 4, 0.0, 5, 3, 6),
        (0, 0, 0.0, 1, 1, 1),
        (0, 1, 0.0, 3, 3, 3),
        (0, 2, 0.0, 3, 3, 3),
        (1, 0, 0.5, 6, 5, 6),
        (1, 4, 1.0, 7, 7, 8),
        (1, 0, 0.0, 4, 4, 4),
        (2, 0, 0.0, 4, 4, 4),
        (2, 4, 0.0, 9, 6, 9),
        (2, 0, 0.0, 2, 2, 2),
        (2, 0, 0.0, 3, 1, 3),
    ];
    for (mode, colorspace, dithering, width, height, stride) in cases {
        let argb_bytes = rng.bytes(4 * stride * height);
        stdin.push_str(&format!(
            "argb2yuva {mode} {colorspace} {:08x} {width} {height} {stride} {}\n",
            dithering.to_bits(),
            hex(&argb_bytes)
        ));
        let mut words: Vec<u32> = argb_bytes
            .as_chunks::<4>()
            .0
            .iter()
            .map(|chunk| u32::from_ne_bytes(*chunk))
            .collect();
        let mut pic = WebPPicture {
            use_argb: 1,
            width: i32::try_from(width).unwrap(),
            height: i32::try_from(height).unwrap(),
            argb_stride: i32::try_from(stride).unwrap(),
            argb: Some(PlaneOffset {
                region: 0,
                offset: 0,
            }),
            argb_storage: PlaneStorage::External(vec![words.as_mut_slice()]),
            ..WebPPicture::default()
        };
        let result = match mode {
            0 => webp_picture_argb_to_yuva(&mut pic, WebPEncCSP(colorspace)),
            1 => webp_picture_argb_to_yuva_dithered(&mut pic, WebPEncCSP(colorspace), dithering),
            _ => webp_picture_sharp_argb_to_yuva(&mut pic),
        };
        expected = expected
            .with("ret", i32::from(result.is_ok()))
            .with("error", pic.error_code.0);
        expected.0.extend(yuva_output(&pic, width, height).0);
        webp_picture_free(&mut pic);
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn yuva_to_argb_conversion_matches_the_c_conversion() {
    let Some(oracle) = Oracle::locate("yuva_to_argb_conversion_matches_the_c_conversion") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_csp");

    let mut rng = Rng(0x5eed_0011);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    // colorspace, width, height, y stride, uv stride, a stride
    let cases: [(u32, usize, usize, usize, usize, usize); 8] = [
        (0, 1, 1, 1, 1, 0),
        (0, 2, 2, 2, 1, 0),
        (0, 4, 4, 5, 3, 0),
        (4, 4, 4, 4, 2, 4),
        (4, 5, 3, 6, 3, 7),
        (4, 7, 6, 7, 4, 7),
        (0, 9, 8, 9, 5, 0),
        (4, 3, 5, 3, 2, 3),
    ];
    for (colorspace, width, height, y_stride, uv_stride, a_stride) in cases {
        let uv_height = height.div_ceil(2);
        let has_alpha = colorspace & 4 != 0;
        let y_bytes = rng.bytes(y_stride * height);
        let u_bytes = rng.bytes(uv_stride * uv_height);
        let v_bytes = rng.bytes(uv_stride * uv_height);
        let a_bytes = if has_alpha {
            rng.bytes(a_stride * height)
        } else {
            Vec::new()
        };
        let a_hex = if has_alpha {
            hex(&a_bytes)
        } else {
            "-".to_owned()
        };
        stdin.push_str(&format!(
            "yuva2argb {colorspace} {width} {height} {y_stride} {uv_stride} {a_stride} {} {} {} {a_hex}\n",
            hex(&y_bytes),
            hex(&u_bytes),
            hex(&v_bytes),
        ));
        let mut y = y_bytes;
        let mut u = u_bytes;
        let mut v = v_bytes;
        let mut a = a_bytes;
        let mut planes = vec![y.as_mut_slice(), u.as_mut_slice(), v.as_mut_slice()];
        let mut pic = WebPPicture {
            colorspace: WebPEncCSP(colorspace),
            width: i32::try_from(width).unwrap(),
            height: i32::try_from(height).unwrap(),
            y: Some(PlaneOffset {
                region: 0,
                offset: 0,
            }),
            u: Some(PlaneOffset {
                region: 1,
                offset: 0,
            }),
            v: Some(PlaneOffset {
                region: 2,
                offset: 0,
            }),
            y_stride: i32::try_from(y_stride).unwrap(),
            uv_stride: i32::try_from(uv_stride).unwrap(),
            a_stride: i32::try_from(a_stride).unwrap(),
            ..WebPPicture::default()
        };
        if has_alpha {
            planes.push(a.as_mut_slice());
            pic.a = Some(PlaneOffset {
                region: 3,
                offset: 0,
            });
        }
        pic.yuva = PlaneStorage::External(planes);
        let result = webp_picture_yuva_to_argb(&mut pic);
        expected = expected
            .with("ret", i32::from(result.is_ok()))
            .with("error", pic.error_code.0);
        expected.0.extend(yuva_output(&pic, width, height).0);
        webp_picture_free(&mut pic);
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}
