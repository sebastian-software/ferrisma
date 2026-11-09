//! WP15 differential test of `WebPAnimEncoderRefineRect` (declared in
//! `src/mux/animi.h`, translated in `src/mux/anim_encode.rs`) against the C
//! oracle, through the probe `tools/oracle/probes/wp15/refine_rect.c`: pairs of
//! generated pictures that differ in rectangles, with initial rectangles inside,
//! across and outside the canvas, lossless and lossy at several qualities, and
//! a YUV previous picture, which the function refuses.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp14_support/mod.rs"]
mod wp14_support;
#[path = "wp8_support/mod.rs"]
mod wp8_support;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::enc::picture_csp_enc::webp_picture_import_rgba;
use webp::mux::anim_encode::webp_anim_encoder_refine_rect;
use webp::webp::encode::{WebPPicture, webp_picture_init};
use wp8_support::Rng;
use wp14_support::work_dir;

fn picture(rgba: &[u8], width: i32, height: i32, argb: i32) -> WebPPicture<'static> {
    let mut pic = WebPPicture::default();
    assert!(webp_picture_init(&mut pic));
    pic.width = width;
    pic.height = height;
    pic.use_argb = argb;
    assert!(webp_picture_import_rgba(&mut pic, rgba, width * 4));
    pic
}

#[test]
fn refine_rect_matches_the_c_encoder() {
    let Some(oracle) = Oracle::locate("refine_rect_matches_the_c_encoder") else {
        return;
    };
    let directory = work_dir("wp15_refine_rect");
    let mut script = String::new();
    let mut expected = String::new();
    let mut rng = Rng(0x15_2e);
    for case in 0..160 {
        let width = 1 + rng.below(40) as i32;
        let height = 1 + rng.below(30) as i32;
        let size = (width * height * 4) as usize;
        let prev: Vec<u8> = (0..size).map(|_| rng.byte() & 0xf8).collect();
        let mut curr = prev.clone();
        // Change a rectangle, by a little (within the lossy tolerance) or a lot.
        let (x0, y0) = (
            rng.below(width as u64) as i32,
            rng.below(height as u64) as i32,
        );
        let (x1, y1) = (
            x0 + rng.below((width - x0) as u64) as i32,
            y0 + rng.below((height - y0) as u64) as i32,
        );
        let delta = [1u8, 3, 8, 40][rng.below(4) as usize];
        if rng.below(8) != 0 {
            for y in y0..=y1 {
                for x in x0..=x1 {
                    let at = ((y * width + x) * 4) as usize;
                    let channel = rng.below(4) as usize;
                    curr[at + channel] = curr[at + channel].wrapping_add(delta);
                }
            }
        }
        let prev_path = directory.join(format!("prev{case}.rgba"));
        let curr_path = directory.join(format!("curr{case}.rgba"));
        std::fs::write(&prev_path, &prev).unwrap();
        std::fs::write(&curr_path, &curr).unwrap();
        let argb = i32::from(case % 20 != 7);
        let lossless = (case % 3 == 0) as i32;
        let quality = [0.0f32, 25.5, 75.0, 100.0][case % 4];
        let (x, y, w, h) = match case % 5 {
            0 => (0, 0, width, height),
            1 => (-5, -3, width + 10, height + 6),
            2 => (width / 3, height / 3, width / 2 + 1, height / 2 + 1),
            3 => (width + 2, height + 2, 4, 4),
            _ => (
                rng.below(width as u64) as i32,
                rng.below(height as u64) as i32,
                1 + rng.below(width as u64) as i32,
                1 + rng.below(height as u64) as i32,
            ),
        };
        // C asserts a non-empty rectangle in MinimizeChangeRectangle; an
        // initial rectangle that clips to nothing is excluded
        // (docs/status/wp15.md).
        let right = (x + w).clamp(0, width);
        let left = x.clamp(0, width - 1);
        let bottom = (y + h).clamp(0, height);
        let top = y.clamp(0, height - 1);
        if (right - left).clamp(0, width - left) == 0 || (bottom - top).clamp(0, height - top) == 0
        {
            continue;
        }
        writeln!(
            script,
            "refine {} {} {width} {height} {argb} {lossless} {quality} {x} {y} {w} {h}",
            prev_path.display(),
            curr_path.display()
        )
        .unwrap();
        let prev_pic = picture(&prev, width, height, argb);
        let curr_pic = picture(&curr, width, height, 1);
        let (mut rx, mut ry, mut rw, mut rh) = (x, y, w, h);
        let ok = webp_anim_encoder_refine_rect(
            &prev_pic, &curr_pic, lossless, quality, &mut rx, &mut ry, &mut rw, &mut rh,
        );
        writeln!(expected, "refine={},{rx},{ry},{rw},{rh}", i32::from(ok)).unwrap();
    }
    let output = oracle
        .probe("wp15", "refine_rect")
        .run::<&str>(&[], script.as_bytes());
    output.success();
    let c = String::from_utf8(output.stdout).unwrap();
    assert_eq!(c, expected);
    eprintln!(
        "refine_rect_matches_the_c_encoder: {} rectangles",
        expected.lines().count()
    );
}
