//! WP9 differential test: `WebPCleanupTransparentArea`, `WebPBlendAlpha`,
//! `WebPPictureHasTransparency` and `WebPReplaceTransparentPixels`
//! (`src/enc/picture_tools_enc.c`, `src/enc/picture_csp_enc.c`). The probe
//! `tools/oracle/probes/wp9/picture_tools.c` runs the C functions on caller-owned
//! planes, padding included, and prints the planes whole. The test runs the same
//! calls on buffers of the same contents and compares the bytes.
//!
//! The alpha planes have transparent 8 by 8 blocks, so the cleanup and the blending
//! change samples, and every geometry includes odd sizes and padded strides.

#[path = "support/oracle.rs"]
mod oracle;

#[path = "support/wp9_pictures.rs"]
mod pictures;

use pictures::{Case, Rng, hex, with_picture};

use oracle::{Fields, Oracle};
use webp::enc::picture_csp_enc::webp_picture_has_transparency;
use webp::enc::picture_tools_enc::{
    webp_blend_alpha, webp_cleanup_transparent_area, webp_replace_transparent_pixels,
};

/// The probe's output: every buffer, whole.
fn expected_buffers(case: &Case) -> Fields {
    if case.use_argb != 0 {
        return Fields::new().hex("argb", &case.argb_bytes());
    }
    let mut fields = Fields::new()
        .hex("y", &case.y)
        .hex("u", &case.u)
        .hex("v", &case.v);
    if case.has_alpha() {
        fields = fields.hex("a", &case.a);
    }
    fields
}

#[test]
fn transparent_area_cleanup_matches_the_c_cleanup() {
    let Some(oracle) = Oracle::locate("transparent_area_cleanup_matches_the_c_cleanup") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_tools");

    let mut rng = Rng(0x5eed_0020);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    let mut changed = 0;
    for index in 0..60 {
        let use_argb = i32::from(index % 3 == 0);
        let colorspace = if index % 2 == 0 { 0 } else { 4 };
        let mut case = Case::random(&mut rng, use_argb, colorspace);
        stdin.push_str(&format!(
            "cleanup {} {}\n",
            case.command_fields(),
            case.plane_arguments()
        ));
        let before = expected_buffers(&case);
        with_picture(&mut case, webp_cleanup_transparent_area);
        changed += usize::from(before != expected_buffers(&case));
        expected.0.extend(expected_buffers(&case).0);
    }
    // The cases must exercise the cleanup, or the comparison proves nothing.
    assert!(changed > 0, "no case changed a sample");
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn alpha_blending_matches_the_c_blending() {
    let Some(oracle) = Oracle::locate("alpha_blending_matches_the_c_blending") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_tools");

    let mut rng = Rng(0x5eed_0021);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    let mut changed = 0;
    for index in 0..60 {
        let use_argb = i32::from(index % 3 == 0);
        let colorspace = if index % 4 == 1 { 0 } else { 4 };
        let background = rng.below(1 << 24) as u32;
        let mut case = Case::random(&mut rng, use_argb, colorspace);
        stdin.push_str(&format!(
            "blend {} {background} {}\n",
            case.command_fields(),
            case.plane_arguments()
        ));
        let before = expected_buffers(&case);
        with_picture(&mut case, |picture| webp_blend_alpha(picture, background));
        changed += usize::from(before != expected_buffers(&case));
        expected.0.extend(expected_buffers(&case).0);
    }
    assert!(changed > 0, "no case changed a sample");
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn transparency_detection_matches_the_c_detection() {
    let Some(oracle) = Oracle::locate("transparency_detection_matches_the_c_detection") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_tools");

    let mut rng = Rng(0x5eed_0022);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    for index in 0..60 {
        let use_argb = i32::from(index % 2 == 0);
        let colorspace = if index % 5 == 0 { 0 } else { 4 };
        let mut case = Case::random(&mut rng, use_argb, colorspace);
        stdin.push_str(&format!(
            "hastransp {} {}\n",
            case.command_fields(),
            case.plane_arguments()
        ));
        let transparent = with_picture(&mut case, |picture| webp_picture_has_transparency(picture));
        expected = expected.with("ret", i32::from(transparent));
    }
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn transparent_pixel_replacement_matches_the_c_replacement() {
    let Some(oracle) = Oracle::locate("transparent_pixel_replacement_matches_the_c_replacement")
    else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_tools");

    let mut rng = Rng(0x5eed_0023);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    for _ in 0..40 {
        let mut case = Case::random(&mut rng, 1, 0);
        let color = rng.next_u64() as u32;
        stdin.push_str(&format!(
            "replace {} {} {} {color} {}\n",
            case.width,
            case.height,
            case.argb_stride,
            hex(&case.argb_bytes())
        ));
        with_picture(&mut case, |picture| {
            webp_replace_transparent_pixels(picture, color);
        });
        expected.0.extend(expected_buffers(&case).0);
    }
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}
