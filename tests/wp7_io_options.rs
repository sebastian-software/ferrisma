//! WP7 differential test: `WebPIoInitFromOptions`, `WebPCheckCropDimensions`
//! and `WebPCheckCropDimensionsBasic` of `src/dec/webp_dec.c` on cropping and
//! scaling options, valid and invalid, against the C probe
//! `tools/oracle/probes/wp7/io_options.c`. Every io field is compared after each
//! call, also after a failure, from a zeroed io and from one whose fields hold
//! another value first, so that the fields a call leaves alone are visible.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp7_support/mod.rs"]
mod wp7_support;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dec::vp8_dec::VP8Io;
use webp::dec::webp_dec::{webp_check_crop_dimensions, webp_io_init_from_options};
use webp::webp::decode::{WEBP_CSP_MODE, WebPDecoderOptions};
use wp7_support::{Rng, compare, print_io};

struct Init {
    fill: i32,
    width: i32,
    height: i32,
    mode: u32,
    use_options: bool,
    options: WebPDecoderOptions,
}

fn init(commands: &mut String, expected: &mut String, case: &Init) {
    let o = &case.options;
    writeln!(
        commands,
        "init {} {} {} {} {} {} {} {} {} {} {} {} {} {} {}",
        case.fill,
        case.width,
        case.height,
        case.mode,
        i32::from(case.use_options),
        o.use_cropping,
        o.crop_left,
        o.crop_top,
        o.crop_width,
        o.crop_height,
        o.use_scaling,
        o.scaled_width,
        o.scaled_height,
        o.bypass_filtering,
        o.no_fancy_upsampling
    )
    .unwrap();
    let flag = case.fill != 0;
    let fill = case.fill;
    let mut io = VP8Io {
        width: case.width,
        height: case.height,
        mb_y: fill,
        mb_w: fill,
        mb_h: fill,
        y_stride: fill,
        uv_stride: fill,
        fancy_upsampling: flag,
        bypass_filtering: flag,
        use_cropping: flag,
        crop_left: fill,
        crop_right: fill,
        crop_top: fill,
        crop_bottom: fill,
        use_scaling: flag,
        scaled_width: fill,
        scaled_height: fill,
    };
    let options = case.use_options.then_some(&case.options);
    let ok = webp_io_init_from_options(options, &mut io, WEBP_CSP_MODE(case.mode));
    writeln!(expected, "init={}", i32::from(ok)).unwrap();
    print_io(expected, &io);
}

fn crop(commands: &mut String, expected: &mut String, values: [i32; 6]) {
    let [w, h, x, y, cw, ch] = values;
    writeln!(commands, "crop {w} {h} {x} {y} {cw} {ch}").unwrap();
    let ok = webp_check_crop_dimensions(w, h, x, y, cw, ch);
    writeln!(expected, "crop={}", i32::from(ok)).unwrap();
}

/// A dimension around the interesting values of `size`.
fn near(rng: &mut Rng, size: i32) -> i32 {
    match rng.below(8) {
        0 => 0,
        1 => -(rng.below(3) as i32) - 1,
        2 => size,
        3 => size + 1 + rng.below(3) as i32,
        4 => size - 1,
        5 => i32::MAX - rng.below(2) as i32,
        6 => i32::MIN + rng.below(2) as i32,
        _ => rng.range(0, i64::from(size.max(1))) as i32,
    }
}

#[test]
fn io_init_from_options_matches_the_c_function() {
    let Some(oracle) = Oracle::locate("io_init_from_options_matches_the_c_function") else {
        return;
    };
    let mut rng = Rng(0x5eed_0702);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut count = 0;

    // Crop checks, including the ones whose subtraction would overflow if the
    // earlier tests did not short-circuit.
    for _ in 0..3000 {
        let w = near(&mut rng, 100);
        let h = near(&mut rng, 60);
        let x = near(&mut rng, w.clamp(0, 100));
        let y = near(&mut rng, h.clamp(0, 60));
        let cw = near(&mut rng, w.clamp(0, 100));
        let ch = near(&mut rng, h.clamp(0, 60));
        crop(&mut commands, &mut expected, [w, h, x, y, cw, ch]);
        count += 1;
    }

    // WebPIoInitFromOptions. The picture dimensions stay below 2^29, so that
    // `W * 3` does not overflow (PORTING rule 8).
    for index in 0..4000 {
        let width = match rng.below(6) {
            0 => rng.range(-3, 3) as i32,
            1 => rng.range(1, 16384) as i32,
            _ => rng.range(1, 300) as i32,
        };
        let height = match rng.below(6) {
            0 => rng.range(-3, 3) as i32,
            1 => rng.range(1, 16384) as i32,
            _ => rng.range(1, 300) as i32,
        };
        let mut options = WebPDecoderOptions {
            use_cropping: i32::from(rng.below(2) == 0),
            ..WebPDecoderOptions::default()
        };
        if rng.below(3) == 0 {
            options.use_cropping = rng.range(-2, 3) as i32;
        }
        options.crop_left = near(&mut rng, width.clamp(0, 300) / 2);
        options.crop_top = near(&mut rng, height.clamp(0, 300) / 2);
        options.crop_width = near(&mut rng, width.clamp(0, 300) / 2);
        options.crop_height = near(&mut rng, height.clamp(0, 300) / 2);
        options.use_scaling = i32::from(rng.below(2) == 0);
        options.scaled_width = match rng.below(6) {
            0 => 0,
            1 => -(rng.below(5) as i32),
            2 => i32::MAX / 2 + rng.below(3) as i32 - 1,
            _ => rng.range(1, 600) as i32,
        };
        options.scaled_height = match rng.below(6) {
            0 => 0,
            1 => -(rng.below(5) as i32),
            2 => i32::MAX / 2 + rng.below(3) as i32 - 1,
            _ => rng.range(1, 600) as i32,
        };
        options.bypass_filtering = rng.range(0, 2) as i32;
        options.no_fancy_upsampling = rng.range(0, 2) as i32;
        let case = Init {
            fill: if index % 2 == 0 { 0 } else { 777 },
            width,
            height,
            mode: rng.below(14) as u32,
            use_options: rng.below(8) != 0,
            options,
        };
        init(&mut commands, &mut expected, &case);
        count += 1;
    }

    let output = oracle
        .probe("wp7", "io_options")
        .run::<&str>(&[], commands.as_bytes());
    output.success();
    compare("io_options", &output.stdout, &expected);
    eprintln!("{count} commands compared");
}
