//! WP1 differential test: `WebPCopyPixels` (`src/utils/utils.c`) on ARGB pictures.
//! The probe `tools/oracle/probes/wp1/copy_pixels.c` initializes two pictures with
//! the library's `WebPPictureInit`, copies with `WebPCopyPixels` and prints the
//! destination. The Rust side copies the same pixels with `webp_copy_pixels` and
//! compares the bytes of the destination.
//!
//! The cases cover random dimensions (including empty ones), strides that are
//! at least the width in both directions, offsets into the buffers, and both
//! storages: the source owns its pixels, the destination borrows the caller's.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::utils::webp_copy_pixels;
use webp::webp::encode::WebPPicture;
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

    fn below(&mut self, bound: u64) -> u64 {
        self.next_u64() % bound
    }
}

/// Geometry of one plane in pixels: the stride (at least the width in magnitude;
/// negative walks upward), the offset of the first pixel and the buffer length.
fn plane(rng: &mut Rng, width: i32, height: i32) -> (i32, usize, usize) {
    let magnitude = width.max(1) + rng.below(4) as i32;
    let negative = height > 0 && rng.below(2) == 0;
    let stride = if negative { -magnitude } else { magnitude };
    let span = usize::try_from(height.max(1) - 1).unwrap() * usize::try_from(magnitude).unwrap();
    let offset = if negative { span } else { 0 };
    let len = span + usize::try_from(width).unwrap() + rng.below(3) as usize;
    (stride, offset, len)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn copy_pixels_matches_the_c_function() {
    let Some(oracle) = Oracle::locate("copy_pixels_matches_the_c_function") else {
        return;
    };
    let probe = oracle.probe("wp1", "copy_pixels");

    let mut rng = Rng(0x5eed_0003);
    let mut stdin = String::new();
    let mut expected = Fields::new();
    for _ in 0..200 {
        let width = rng.below(7) as i32;
        let height = rng.below(5) as i32;
        let (src_stride, src_offset, src_len) = plane(&mut rng, width, height);
        let (dst_stride, dst_offset, dst_len) = plane(&mut rng, width, height);
        let src: Vec<u32> = (0..src_len).map(|_| rng.next_u64() as u32).collect();
        let source_hex = if src_len == 0 {
            "-".to_owned()
        } else {
            hex(&src
                .iter()
                .flat_map(|pixel| pixel.to_le_bytes())
                .collect::<Vec<u8>>())
        };
        stdin.push_str(&format!(
            "copy {src_stride} {dst_stride} {width} {height} {src_offset} {dst_offset} \
             {src_len} {dst_len} {source_hex}\n"
        ));

        // The destination borrows the caller's buffer; the source owns its pixels.
        let mut destination = vec![0xa5a5_a5a5_u32; dst_len];
        {
            let source = WebPPicture {
                use_argb: 1,
                width,
                height,
                argb_stride: src_stride,
                argb: Some(PlaneOffset {
                    region: 0,
                    offset: src_offset,
                }),
                argb_storage: PlaneStorage::Internal(src),
                ..WebPPicture::default()
            };
            let mut target = WebPPicture {
                use_argb: 1,
                width,
                height,
                argb_stride: dst_stride,
                argb: Some(PlaneOffset {
                    region: 0,
                    offset: dst_offset,
                }),
                argb_storage: PlaneStorage::External(vec![destination.as_mut_slice()]),
                ..WebPPicture::default()
            };
            webp_copy_pixels(&source, &mut target);
        }
        let bytes: Vec<u8> = destination
            .iter()
            .flat_map(|pixel| pixel.to_le_bytes())
            .collect();
        expected = expected.hex("dst", &bytes);
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}
