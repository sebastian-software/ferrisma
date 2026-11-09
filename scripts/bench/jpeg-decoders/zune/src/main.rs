//! Times `zune-jpeg` (Ferrixel's `jpeg-read` decoder) on one JPEG held in
//! memory, with the same output line and `.ref` format as `jpegdec.c`.
//!
//! usage: zune FILE REPS [OUT.ref]
//!
//! `zune-jpeg` has no scaled decode, so there is no DENOM argument.

use std::io::{BufWriter, Write as _};
use std::time::Instant;

use zune_jpeg::JpegDecoder;
use zune_jpeg::zune_core::{bytestream::ZCursor, colorspace::ColorSpace, options::DecoderOptions};

fn decode(bytes: &[u8]) -> (Vec<u8>, usize, usize) {
    let options = DecoderOptions::default().jpeg_set_out_colorspace(ColorSpace::RGB);
    let mut decoder = JpegDecoder::new_with_options(ZCursor::new(bytes), options);
    let pixels = decoder.decode().expect("decode jpeg");
    let (width, height) = decoder.dimensions().expect("dimensions");
    (pixels, width, height)
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    assert!(args.len() >= 2, "usage: zune FILE REPS [OUT.ref]");
    let bytes = std::fs::read(&args[0]).expect("read jpeg");
    let reps: usize = args[1].parse().expect("reps");
    let mut times = Vec::with_capacity(reps);
    let mut last = None;
    for _ in 0..reps {
        drop(last.take());
        let start = Instant::now();
        let out = decode(&bytes);
        times.push(start.elapsed().as_secs_f64() * 1e3);
        last = Some(out);
    }
    times.sort_by(f64::total_cmp);
    let (pixels, width, height) = last.expect("at least one repetition");
    println!(
        "zune-jpeg\t{}\t1/1\t{width}x{height}\tu8\tbest={:.2}\tmedian={:.2}",
        args[0],
        times[0],
        times[reps / 2]
    );
    if let Some(path) = args.get(2) {
        let mut out = BufWriter::new(std::fs::File::create(path).expect("create ref"));
        out.write_all(b"QMREF1\0\0").unwrap();
        for dim in [width, height] {
            out.write_all(&u32::try_from(dim).unwrap().to_le_bytes())
                .unwrap();
        }
        for &v in &pixels {
            out.write_all(&(f32::from(v) / 255.0).to_le_bytes())
                .unwrap();
        }
    }
}
