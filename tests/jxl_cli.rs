//! CLI coverage for lossy JPEG XL output (ADR-0039): transparent and
//! text-like input encode to `.jxl` at the default options (lossy d1.0,
//! patches off) and read back through the engine.
#![cfg(feature = "cli")]
// Test images are small; their coordinates and samples fit every cast.
#![allow(clippy::cast_possible_truncation)]

use std::path::{Path, PathBuf};
use std::process::Command;

use jxl_oxide::{ExtraChannelType, JxlImage};
use webp::api::encode::{PixelLayout, PixelRows, encode_lossless};

fn temp(name: &str) -> PathBuf {
    std::env::temp_dir().join(format!("ferrisma-jxl-cli-{}-{name}", std::process::id()))
}

fn write_webp(path: &Path, width: usize, height: usize, rgba: &[u8]) {
    let bytes = encode_lossless(PixelRows {
        layout: PixelLayout::Rgba,
        data: rgba,
        width: width as u32,
        height: height as u32,
        stride: width * 4,
    })
    .unwrap();
    std::fs::write(path, bytes).unwrap();
}

fn ferrisma(args: &[&str]) {
    let result = Command::new(env!("CARGO_BIN_EXE_ferrisma"))
        .args(args)
        .output()
        .unwrap();
    assert!(
        result.status.success(),
        "{args:?}: {}",
        String::from_utf8_lossy(&result.stderr)
    );
}

/// A gradient with an alpha ramp and fully transparent stripes.
fn translucent(width: usize, height: usize) -> Vec<u8> {
    let mut data = Vec::with_capacity(width * height * 4);
    for y in 0..height {
        for x in 0..width {
            data.extend_from_slice(&[
                ((x * 255) / width) as u8,
                ((y * 255) / height) as u8,
                (((x + y) * 255) / (width + height)) as u8,
                if (x / 3) % 5 == 0 {
                    0
                } else {
                    (1 + (x * 7 + y * 3) % 255) as u8
                },
            ]);
        }
    }
    data
}

/// Text-like content: copies of one small dark glyph on a flat light page.
/// The same generator as `glyphs` in `src/jxl_lossy_tests.rs` and the
/// bridge's tests, where libjxl's default patch search finds patches in the
/// 64x64 six-glyph image (the bridge checks that admission).
fn glyphs(width: usize, height: usize, glyph: usize, copies: usize) -> Vec<u8> {
    let mut s = 0x9e37_79b9_7f4a_7c15_u64 | 1;
    let mut rnd = || {
        s ^= s << 13;
        s ^= s >> 7;
        s ^= s << 17;
        s
    };
    let mut data = [242_u8, 237, 230, 255].repeat(width * height);
    let shape: Vec<bool> = (0..glyph * glyph).map(|_| rnd() % 3 != 0).collect();
    let span = width.saturating_sub(glyph + 8).max(1);
    for k in 0..copies {
        let gx = 4 + (k * (glyph + 6)) % span;
        let gy = 4 + ((k * (glyph + 6)) / span) * (glyph + 6);
        if gy + glyph + 4 >= height {
            break;
        }
        for y in 0..glyph {
            for x in 0..glyph {
                if shape[y * glyph + x] {
                    let p = ((gy + y) * width + gx + x) * 4;
                    data[p..p + 3].copy_from_slice(&[26, 38, 51]);
                }
            }
        }
    }
    data
}

#[test]
fn transparent_and_text_like_input_encode_to_lossy_jpeg_xl_by_default() {
    for (name, width, height, rgba) in [
        ("alpha", 96, 64, translucent(96, 64)),
        ("text", 64, 64, glyphs(64, 64, 5, 6)),
        ("page", 300, 120, glyphs(300, 120, 5, 200)),
    ] {
        let input = temp(&format!("{name}.webp"));
        let output = temp(&format!("{name}.jxl"));
        let readback = temp(&format!("{name}-readback.webp"));
        write_webp(&input, width, height, &rgba);
        let (w, h) = (width.to_string(), height.to_string());
        // Default JPEG XL options: lossy d1.0 with patches off.
        ferrisma(&[
            input.to_str().unwrap(),
            output.to_str().unwrap(),
            &w,
            &h,
            "--assume-srgb",
        ]);
        let bytes = std::fs::read(&output).unwrap();
        let image = JxlImage::builder()
            .read(std::io::Cursor::new(&bytes))
            .unwrap();
        let metadata = &image.image_header().metadata;
        assert!(metadata.xyb_encoded, "{name}: lossy");
        assert_eq!(
            (image.width(), image.height()),
            (width as u32, height as u32)
        );
        let transparent = rgba.as_chunks::<4>().0.iter().any(|p| p[3] != 255);
        assert_eq!(metadata.ec_info.len(), usize::from(transparent), "{name}");
        if transparent {
            assert_eq!(
                metadata.ec_info[0].ty,
                ExtraChannelType::Alpha {
                    alpha_associated: false
                }
            );
            // Same-size output: the finished rows equal the input, and alpha
            // is coded losslessly.
            let render = image.render_frame(0).unwrap();
            let mut stream = render.stream();
            let mut samples = vec![0_u8; width * height * 4];
            stream.write_to_buffer(&mut samples);
            let alpha = |data: &[u8]| {
                data.as_chunks::<4>()
                    .0
                    .iter()
                    .map(|p| p[3])
                    .collect::<Vec<_>>()
            };
            assert_eq!(alpha(&samples), alpha(&rgba), "{name}: alpha is exact");
        }
        // The engine reads opaque output back through jxl-oxide and the
        // rendered-ICC stage. Transparent lossy (XYB) JPEG XL is not yet
        // readable by the engine: the rendered-ICC stage accepts only opaque
        // RGB, and without it XYB samples have no verifiable color; the
        // independent decode above checks that output instead.
        if !transparent {
            ferrisma(&[
                output.to_str().unwrap(),
                readback.to_str().unwrap(),
                &w,
                &h,
                "--jxl-decoder=jxl-oxide",
                "--use-rendered-icc",
                "--webp-lossless",
            ]);
            assert_eq!(&std::fs::read(&readback).unwrap()[..4], b"RIFF");
        }
        for path in [input, output, readback] {
            let _ = std::fs::remove_file(path);
        }
    }
}
