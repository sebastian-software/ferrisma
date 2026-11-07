//! WP9 differential test: `WebPPictureView` and `WebPPictureCopy`
//! (`src/enc/picture_rescale_enc.c`). The probe
//! `tools/oracle/probes/wp9/picture_view.c` runs the C calls on caller-owned planes
//! and prints what the view or the copy exposes: the return value, the size, the
//! colorspace, the error code and the visible samples. The test runs the same calls on
//! the Rust translation and compares them.
//!
//! The views cover rectangles inside and outside the picture, with odd corners (the
//! chroma position is snapped), in both storage forms. The copies cover both forms
//! with padded strides. The C copy also shares the callback and reference pointers of
//! its source; the Rust copy and view hold none of them (WP9 deviation), so the test
//! records the C sharing as the expected output and checks the Rust fields separately.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp9_pictures.rs"]
mod pictures;
#[path = "support/wp9_visible.rs"]
mod visible;

use oracle::{Fields, Oracle};
use pictures::{Case, Rng, with_picture};
use visible::visible_picture;
use webp::enc::picture_enc::webp_picture_alloc;
use webp::enc::picture_rescale_enc::{webp_picture_copy, webp_picture_view};
use webp::enc::picture_tools_enc::webp_cleanup_transparent_area;
use webp::webp::encode::{WebPAuxStats, WebPEncCSP, WebPPicture, webp_picture_init};
use webp::webp::types::PlaneStorage;

#[test]
fn view_matches_the_c_view() {
    let Some(oracle) = Oracle::locate("view_matches_the_c_view") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_view");

    let mut rng = Rng(0x5eed_0060);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    let mut succeeded = 0;
    let mut refused = 0;
    for index in 0..80 {
        let use_argb = i32::from(index % 3 == 0);
        let colorspace = if index % 2 == 0 { 0 } else { 4 };
        let mut case = Case::random(&mut rng, use_argb, colorspace);
        let width = i32::try_from(case.width).unwrap();
        let height = i32::try_from(case.height).unwrap();
        let left = rng.below(u64::try_from(width + 2).unwrap()) as i32 - 1;
        let top = rng.below(u64::try_from(height + 2).unwrap()) as i32 - 1;
        let view_width = rng.below(u64::try_from(width + 2).unwrap()) as i32;
        let view_height = rng.below(u64::try_from(height + 2).unwrap()) as i32;
        stdin.push_str(&format!(
            "view {} {left} {top} {view_width} {view_height} {}\n",
            case.command_fields(),
            case.plane_arguments()
        ));
        let block = with_picture(&mut case, |picture| {
            match webp_picture_view(picture, left, top, view_width, view_height) {
                None => Fields::new().with("ret", 0),
                Some(view) => {
                    let mut fields = Fields::new().with("ret", 1);
                    fields.0.extend(visible_picture(&view).0);
                    fields
                }
            }
        });
        if block.get("ret") == Some("1") {
            succeeded += 1;
        } else {
            refused += 1;
        }
        expected.0.extend(block.0);
    }
    // The cases must include views that succeed and views that the rectangle refuses.
    assert!(succeeded > 0, "no view succeeded");
    assert!(refused > 0, "no view was refused");
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn copy_matches_the_c_copy() {
    let Some(oracle) = Oracle::locate("copy_matches_the_c_copy") else {
        return;
    };
    let probe = oracle.probe("wp9", "picture_view");

    let mut rng = Rng(0x5eed_0061);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    for index in 0..40 {
        let use_argb = i32::from(index % 3 == 0);
        let colorspace = if index % 2 == 0 { 0 } else { 4 };
        let mut case = Case::random(&mut rng, use_argb, colorspace);
        stdin.push_str(&format!(
            "copy {} {}\n",
            case.command_fields(),
            case.plane_arguments()
        ));
        let block = with_picture(&mut case, |picture| {
            let mut copy = WebPPicture::default();
            let result = webp_picture_copy(picture, &mut copy);
            let mut fields = Fields::new().with("ret", i32::from(result.is_ok()));
            if result.is_ok() {
                fields.0.extend(visible_picture(&copy).0);
            }
            // The C copy shares these pointers with its source (WP9 deviation).
            fields
                .with("writer_copied", 1)
                .with("progress_copied", 1)
                .with("extra_info_copied", 1)
                .with("stats_copied", 1)
        });
        expected.0.extend(block.0);
    }
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn views_and_copies_hold_no_callbacks() {
    // A source with every callback and reference set, so that the copy and the view
    // would carry them if they held them.
    // Declared before the picture, which borrows them and drops after them.
    let mut extra = vec![0_u8; 24];
    let mut stats = WebPAuxStats::default();
    let mut source = WebPPicture::default();
    assert!(webp_picture_init(&mut source));
    source.width = 6;
    source.height = 4;
    source.colorspace = WebPEncCSP::WEBP_YUV420A;
    source.use_argb = 0;
    webp_picture_alloc(&mut source).expect("the source allocates");
    source.writer = Some(Box::new(|_: &[u8]| true));
    source.progress_hook = Some(Box::new(|_: i32| true));
    source.extra_info = Some(extra.as_mut_slice());
    source.stats = Some(&mut stats);

    let view = webp_picture_view(&source, 0, 0, 4, 2).expect("the view is inside the picture");
    assert!(view.writer.is_none());
    assert!(view.progress_hook.is_none());
    assert!(view.extra_info.is_none());
    assert!(view.stats.is_none());
    assert!(matches!(view.yuva, PlaneStorage::Shared(_)));

    let mut copy = WebPPicture::default();
    webp_picture_copy(&source, &mut copy).expect("the copy allocates");
    assert!(copy.writer.is_none());
    assert!(copy.progress_hook.is_none());
    assert!(copy.extra_info.is_none());
    assert!(copy.stats.is_none());
    assert!(matches!(copy.yuva, PlaneStorage::Internal(_)));
}

#[test]
#[should_panic(expected = "shared view of a picture cannot be written")]
fn writing_into_a_yuva_view_panics() {
    let mut rng = Rng(0x5eed_0062);
    let mut case = Case::random(&mut rng, 0, 4);
    with_picture(&mut case, |picture| {
        let mut view =
            webp_picture_view(picture, 0, 0, 1, 1).expect("the view is inside the picture");
        webp_cleanup_transparent_area(&mut view);
    });
}

#[test]
#[should_panic(expected = "shared view of a picture cannot be written")]
fn writing_into_an_argb_view_panics() {
    let mut rng = Rng(0x5eed_0063);
    let mut case = Case::random(&mut rng, 1, 0);
    with_picture(&mut case, |picture| {
        let mut view =
            webp_picture_view(picture, 0, 0, 1, 1).expect("the view is inside the picture");
        webp_cleanup_transparent_area(&mut view);
    });
}
