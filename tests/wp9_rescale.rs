//! WP9 differential test: `WebPPictureRescale` and `WebPPictureCrop`
//! (`src/enc/picture_rescale_enc.c`). The probe
//! `tools/oracle/probes/wp9/picture_rescale.c` runs the C calls on caller-owned planes;
//! on success the picture then holds library-owned planes, so both sides print the
//! return value, the new dimensions, the error code and the visible samples only.
//!
//! The rescale cases keep the aspect ratio with a zero dimension, scale down and up,
//! use odd sizes, and include an invalid dimension. The crop cases place the
//! rectangle at odd corners (the chroma position is snapped), inside the picture and
//! outside it.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp9_pictures.rs"]
mod pictures;
#[path = "support/wp9_visible.rs"]
mod visible;

use oracle::{Fields, Oracle};
use pictures::{Case, Rng, with_picture};
use visible::visible_picture;
use webp::enc::picture_rescale_enc::{webp_picture_crop, webp_picture_rescale};

#[test]
fn rescale_matches_the_c_rescale() {
    let Some(oracle) = Oracle::locate("rescale_matches_the_c_rescale") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_rescale");

    let targets: [(i32, i32); 10] = [
        (0, 0),
        (1, 1),
        (2, 3),
        (5, 7),
        (16, 9),
        (3, 0),
        (0, 4),
        (-1, 4),
        (7, 2),
        (40, 30),
    ];
    let mut rng = Rng(0x5eed_0030);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    let mut succeeded = 0;
    for index in 0..60 {
        let use_argb = i32::from(index % 3 == 0);
        let colorspace = if index % 2 == 0 { 0 } else { 4 };
        let (new_width, new_height) = targets[index % targets.len()];
        let mut case = Case::random(&mut rng, use_argb, colorspace);
        stdin.push_str(&format!(
            "rescale {} {new_width} {new_height} {}\n",
            case.command_fields(),
            case.plane_arguments()
        ));
        let fields = with_picture(&mut case, |picture| {
            let result = webp_picture_rescale(picture, new_width, new_height);
            let mut fields = Fields::new().with("ret", i32::from(result.is_ok()));
            fields.0.extend(visible_picture(picture).0);
            fields
        });
        succeeded += usize::from(fields.get("ret") == Some("1"));
        expected.0.extend(fields.0);
    }
    // The cases must include successful rescales, or the comparison proves little.
    assert!(succeeded > 0, "no rescale succeeded");
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn crop_matches_the_c_crop() {
    let Some(oracle) = Oracle::locate("crop_matches_the_c_crop") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_rescale");

    let mut rng = Rng(0x5eed_0031);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    let mut succeeded = 0;
    for index in 0..80 {
        let use_argb = i32::from(index % 3 == 0);
        let colorspace = if index % 2 == 0 { 0 } else { 4 };
        let mut case = Case::random(&mut rng, use_argb, colorspace);
        let width = i32::try_from(case.width).unwrap();
        let height = i32::try_from(case.height).unwrap();
        let left = rng.below(u64::try_from(width + 2).unwrap()) as i32 - 1;
        let top = rng.below(u64::try_from(height + 2).unwrap()) as i32 - 1;
        let crop_width = rng.below(u64::try_from(width + 2).unwrap()) as i32;
        let crop_height = rng.below(u64::try_from(height + 2).unwrap()) as i32;
        stdin.push_str(&format!(
            "crop {} {left} {top} {crop_width} {crop_height} {}\n",
            case.command_fields(),
            case.plane_arguments()
        ));
        let (ret, fields) = with_picture(&mut case, |picture| {
            let ret = webp_picture_crop(picture, left, top, crop_width, crop_height);
            (ret, visible_picture(picture))
        });
        succeeded += usize::from(ret);
        let mut block = Fields::new().with("ret", i32::from(ret));
        block.0.extend(fields.0);
        expected.0.extend(block.0);
    }
    assert!(succeeded > 0, "no crop succeeded");
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}
