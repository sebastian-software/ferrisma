//! Encode a binary PPM (`P6`) or PAM (`P7`, `RGB` or `RGB_ALPHA`) of any
//! `maxval` `2^n - 1` (8 to 16 bits) through this bridge, as the engine's
//! JPEG XL sink does: the byte comparison with `cjxl` on the same file and
//! the memory probe of the 2026-11-15 depth package
//! (`docs/benchmarks/v1.0-jxl-deep-memory-2026-11-15.md`).
//!
//! ```sh
//! cargo run --release -p ferrisma-jxl-encoder --example encode_pnm -- in.pam out.jxl lossless
//! cargo run --release -p ferrisma-jxl-encoder --example encode_pnm -- in.ppm out.jxl 1.0
//! ```
use ferrisma_jxl_encoder::{Alpha, FrameOptions, ImageInfo, encode, encode_lossless};

fn header_fields(data: &[u8]) -> (Vec<String>, usize) {
    // P6: four whitespace-separated tokens; P7: key/value lines to ENDHDR.
    if data.starts_with(b"P7") {
        let end = data
            .windows(7)
            .position(|w| w == b"ENDHDR\n")
            .expect("ENDHDR")
            + 7;
        let text = std::str::from_utf8(&data[..end]).unwrap();
        (text.split_whitespace().map(str::to_owned).collect(), end)
    } else {
        let mut tokens = Vec::new();
        let mut i = 0;
        while tokens.len() < 4 {
            while data[i].is_ascii_whitespace() {
                i += 1;
            }
            let start = i;
            while !data[i].is_ascii_whitespace() {
                i += 1;
            }
            tokens.push(String::from_utf8_lossy(&data[start..i]).into_owned());
        }
        (tokens, i + 1)
    }
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let data = std::fs::read(&args[1]).unwrap();
    let (tokens, start) = header_fields(&data);
    let field = |key: &str| -> usize {
        let i = tokens.iter().position(|t| t == key).unwrap();
        tokens[i + 1].parse().unwrap()
    };
    let (width, height, maxval, alpha) = if tokens[0] == "P7" {
        let alpha = tokens.iter().any(|t| t == "RGB_ALPHA");
        (field("WIDTH"), field("HEIGHT"), field("MAXVAL"), alpha)
    } else {
        (
            tokens[1].parse().unwrap(),
            tokens[2].parse().unwrap(),
            tokens[3].parse().unwrap(),
            false,
        )
    };
    let bits = u8::try_from(usize::BITS - maxval.leading_zeros()).unwrap();
    assert_eq!((1_usize << bits) - 1, maxval, "maxval must be 2^n - 1");
    let raw = &data[start..];
    let pixels: Vec<u8> = if bits > 8 {
        raw.as_chunks::<2>()
            .0
            .iter()
            .flat_map(|pair| u16::from_be_bytes(*pair).to_ne_bytes())
            .collect()
    } else {
        raw.to_vec()
    };
    let base = if alpha {
        ImageInfo::srgba8(
            u32::try_from(width).unwrap(),
            u32::try_from(height).unwrap(),
        )
    } else {
        ImageInfo::srgb8(
            u32::try_from(width).unwrap(),
            u32::try_from(height).unwrap(),
        )
    };
    let image = base.with_bits(bits);
    assert_eq!(image.alpha == Alpha::None, !alpha);
    let mut out = Vec::new();
    if args[3] == "lossless" {
        encode_lossless(image, &pixels, &mut out, usize::MAX).unwrap();
    } else {
        let distance: f32 = args[3].parse().unwrap();
        encode(
            image,
            FrameOptions::with_distance(distance),
            &pixels,
            &mut out,
            usize::MAX,
        )
        .unwrap();
    }
    std::fs::write(&args[2], &out).unwrap();
    println!("{} bytes", out.len());
}
