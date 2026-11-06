//! WP1 differential test: the byte, bit, endian and plane helpers of
//! `src/utils/utils.h`, `src/utils/utils.c` (`WebPCopyPlane`) and
//! `src/utils/endian_inl_utils.h`. One script of commands drives both the C probe
//! `tools/oracle/probes/wp1/utils_inline.c` and the Rust functions, and the
//! outputs are compared value by value. The plane copies cover positive and
//! negative strides, padded and unpadded buffers, and offsets into the buffers.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::endian_inl_utils::{b_swap16, b_swap32, b_swap64, h_to_le16, h_to_le32};
use webp::utils::utils::{
    bits_ctz, bits_log2_floor, check_size_overflow, get_le16, get_le24, get_le32, put_le16,
    put_le24, put_le32, webp_align, webp_copy_plane, webp_int32_to_mem, webp_mem_to_int32,
    webp_mem_to_uint32, webp_uint32_to_mem,
};

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

    fn bytes<const N: usize>(&mut self) -> [u8; N] {
        let mut out = [0_u8; N];
        for byte in &mut out {
            *byte = self.next_u64() as u8;
        }
        out
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// Builds the probe's input and the expected output side by side.
#[derive(Default)]
struct Script {
    stdin: String,
    expected: Fields,
}

impl Script {
    fn command(&mut self, text: String) {
        self.stdin.push_str(&text);
        self.stdin.push('\n');
    }
}

/// Plane geometry for `WebPCopyPlane`: the stride magnitude is at least the
/// width, the first row starts at `offset`, and the buffer holds every row.
fn plane(rng: &mut Rng, width: i32, height: i32) -> (i32, usize, usize) {
    let magnitude = width.max(1) + rng.below(4) as i32;
    let negative = height > 0 && rng.below(2) == 0;
    let stride = if negative { -magnitude } else { magnitude };
    let span = usize::try_from(height.max(1) - 1).unwrap() * usize::try_from(magnitude).unwrap();
    let offset = if negative { span } else { 0 };
    let len = span + usize::try_from(width).unwrap() + rng.below(4) as usize;
    (stride, offset, len)
}

fn script() -> Script {
    let mut script = Script::default();
    let mut rng = Rng(0x5eed_0002);

    for _ in 0..200 {
        let data: [u8; 4] = rng.bytes();
        script.command(format!("le {}", hex(&data)));
        script.expected = script
            .expected
            .with("get_le16", get_le16(&data))
            .with("get_le24", get_le24(&data))
            .with("get_le32", get_le32(&data));
    }

    for value in [0_i32, 1, 255, 256, 65_535]
        .into_iter()
        .chain((0..200).map(|_| rng.below(65_536) as i32))
    {
        script.command(format!("put16 {value}"));
        let mut data = [0_u8; 4];
        put_le16(&mut data, value);
        script.expected = script.expected.hex("bytes", &data);
    }
    for value in [0_i32, 1, 65_535, 65_536, (1 << 24) - 1]
        .into_iter()
        .chain((0..200).map(|_| rng.below(1 << 24) as i32))
    {
        script.command(format!("put24 {value}"));
        let mut data = [0_u8; 4];
        put_le24(&mut data, value);
        script.expected = script.expected.hex("bytes", &data);
    }
    for value in [0_u32, 1, 65_536, u32::MAX]
        .into_iter()
        .chain((0..200).map(|_| rng.next_u64() as u32))
    {
        script.command(format!("put32 {value}"));
        let mut data = [0_u8; 4];
        put_le32(&mut data, value);
        script.expected = script.expected.hex("bytes", &data);
    }

    for value in [1_u32, 2, 3, 1 << 31, u32::MAX]
        .into_iter()
        .chain((0..200).map(|_| (rng.next_u64() as u32).max(1)))
    {
        script.command(format!("log2 {value}"));
        script.expected = script.expected.with("value", bits_log2_floor(value));
    }
    for value in [1_u32, 2, 1 << 31, u32::MAX]
        .into_iter()
        .chain((0..200).map(|_| (rng.next_u64() as u32).max(1)))
    {
        script.command(format!("ctz {value}"));
        script.expected = script.expected.with("value", bits_ctz(value));
    }

    for _ in 0..200 {
        let data: [u8; 4] = rng.bytes();
        script.command(format!("memu32 {}", hex(&data)));
        script.expected = script.expected.with("value", webp_mem_to_uint32(&data));
        script.command(format!("memi32 {}", hex(&data)));
        script.expected = script.expected.with("value", webp_mem_to_int32(&data));
    }
    for _ in 0..200 {
        let value = rng.next_u64() as u32;
        script.command(format!("wu32 {value}"));
        let mut data = [0_u8; 4];
        webp_uint32_to_mem(&mut data, value);
        script.expected = script.expected.hex("bytes", &data);
        let signed = value.cast_signed();
        script.command(format!("wi32 {signed}"));
        let mut data = [0_u8; 4];
        webp_int32_to_mem(&mut data, signed);
        script.expected = script.expected.hex("bytes", &data);
    }

    for _ in 0..200 {
        let value = rng.below(65_536) as u16;
        script.command(format!("hle16 {value}"));
        script.expected = script.expected.with("value", h_to_le16(value));
        script.command(format!("bs16 {value}"));
        script.expected = script.expected.with("value", b_swap16(value));
        let wide = rng.next_u64() as u32;
        script.command(format!("hle32 {wide}"));
        script.expected = script.expected.with("value", h_to_le32(wide));
        script.command(format!("bs32 {wide}"));
        script.expected = script.expected.with("value", b_swap32(wide));
        let huge = rng.next_u64();
        script.command(format!("bs64 {huge}"));
        script.expected = script.expected.with("value", b_swap64(huge));
    }

    for size in [0_u64, 1, 1 << 32, 1 << 40, u64::MAX]
        .into_iter()
        .chain((0..50).map(|_| rng.next_u64()))
    {
        script.command(format!("csov {size}"));
        script.expected = script
            .expected
            .with("value", i32::from(check_size_overflow(size)));
    }

    for _ in 0..200 {
        let address = rng.below(1 << 48);
        script.command(format!("align {address}"));
        let aligned = webp_align(usize::try_from(address).unwrap());
        script.expected = script.expected.with("value", aligned);
    }

    for _ in 0..200 {
        let width = rng.below(9) as i32;
        let height = rng.below(6) as i32;
        let (src_stride, src_offset, src_len) = plane(&mut rng, width, height);
        let (dst_stride, dst_offset, dst_len) = plane(&mut rng, width, height);
        let src: Vec<u8> = (0..src_len).map(|_| rng.next_u64() as u8).collect();
        let source_text = if src_len == 0 {
            "-".to_owned()
        } else {
            hex(&src)
        };
        script.command(format!(
            "copy_plane {src_stride} {dst_stride} {width} {height} {src_offset} {dst_offset} \
             {src_len} {dst_len} {source_text}"
        ));
        let mut dst = vec![0xa5_u8; dst_len];
        webp_copy_plane(
            &src, src_offset, src_stride, &mut dst, dst_offset, dst_stride, width, height,
        );
        script.expected = script.expected.hex("dst", &dst);
    }
    script
}

#[test]
fn utility_helpers_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("utility_helpers_match_the_c_functions") else {
        return;
    };
    let probe = oracle.probe("wp1", "utils_inline");
    let script = script();
    let c_output = probe
        .run::<&str>(&[], script.stdin.as_bytes())
        .success()
        .fields();
    assert_eq!(c_output, script.expected);
}
