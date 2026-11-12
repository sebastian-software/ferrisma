//! Run each mode in a fresh release process and measure RSS externally.
//! Example: /usr/bin/time -l cargo run --release -p ferrisma-jxl-decoder
//!   --example jxl_decode_compare -- fixture.jxl own crop 10
use ferrisma_jxl_decoder::{decode_into, inspect};
use jxl_oxide::{CropInfo, JxlImage};
use std::hint::black_box;
use std::io::Cursor;
use std::time::Instant;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    assert_eq!(
        args.len(),
        5,
        "path mode(own|oxide|compare) region(full|crop) iterations"
    );
    let bytes = std::fs::read(&args[1]).unwrap();
    if args[2] == "compare" {
        let info = inspect(&bytes, 100_000_000).unwrap();
        let mut own = vec![0f32; info.sample_count().unwrap()];
        decode_into(&bytes, &info, &mut own, 100_000_000).unwrap();
        let image = JxlImage::builder().read(Cursor::new(&bytes)).unwrap();
        let rendered = image.render_frame_cropped(0).unwrap();
        let mut stream = rendered.stream();
        assert_eq!(stream.channels(), info.channels);
        let row_len = info.width as usize * info.channels as usize;
        let mut row = vec![0f32; row_len];
        let mut max_diff = 0f32;
        let mut sum_diff = 0f64;
        for y in 0..info.height as usize {
            assert_eq!(stream.write_to_buffer(&mut row), row_len);
            for (a, b) in own[y * row_len..(y + 1) * row_len].iter().zip(&row) {
                assert!(a.is_finite() && b.is_finite());
                let diff = (a - b).abs();
                max_diff = max_diff.max(diff);
                sum_diff += f64::from(diff);
            }
        }
        println!(
            "samples={} max_abs_diff={max_diff:.9} mean_abs_diff={:.9}",
            own.len(),
            sum_diff / own.len() as f64
        );
        return;
    }
    let iterations: usize = args[4].parse().unwrap();
    assert!(iterations > 0);
    let dimensions = if args[2] == "own" {
        let info = inspect(&bytes, 100_000_000).unwrap();
        (info.width, info.height)
    } else {
        let initial = JxlImage::builder().read(Cursor::new(&bytes)).unwrap();
        (initial.width(), initial.height())
    };
    let begin = Instant::now();
    let mut checksum = 0f64;
    for _ in 0..iterations {
        let (width, height) = dimensions;
        let (x, y, w, h) = if args[3] == "crop" {
            (
                width / 4,
                height / 4,
                (width / 2).max(1),
                (height / 2).max(1),
            )
        } else {
            (0, 0, width, height)
        };
        match args[2].as_str() {
            "oxide" => {
                let mut image = JxlImage::builder().read(Cursor::new(&bytes)).unwrap();
                image.set_image_region(CropInfo {
                    left: x,
                    top: y,
                    width: w,
                    height: h,
                });
                let rendered = image.render_frame_cropped(0).unwrap();
                let mut stream = rendered.stream();
                let mut row = vec![0f32; w as usize * stream.channels() as usize];
                for _ in 0..h {
                    assert_eq!(stream.write_to_buffer(&mut row), row.len());
                    checksum += f64::from(row[0]);
                }
            }
            "own" => {
                let info = inspect(&bytes, 100_000_000).unwrap();
                assert_eq!((width, height), (info.width, info.height));
                let mut output = vec![0f32; info.sample_count().unwrap()];
                decode_into(&bytes, &info, &mut output, 100_000_000).unwrap();
                let channels = info.channels as usize;
                for row in y..y + h {
                    checksum +=
                        f64::from(output[(row as usize * width as usize + x as usize) * channels]);
                }
            }
            _ => panic!("unknown mode"),
        }
    }
    black_box(checksum);
    println!(
        "mode={} region={} iterations={} dimensions={}x{} elapsed_ms={:.3} checksum={checksum:.8}",
        args[2],
        args[3],
        iterations,
        dimensions.0,
        dimensions.1,
        begin.elapsed().as_secs_f64() * 1000.0
    );
}
