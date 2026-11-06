//! WP2 differential test: `VP8BitReader` (`src/utils/bit_reader_utils.c` and
//! `src/utils/bit_reader_inl_utils.h`). Seeded and truncated streams are read by
//! the probe `tools/oracle/probes/wp2/vp8_bit_reader.c` and by the Rust functions
//! with the same commands. After every command both sides report the result and
//! the reader state: the register, the range, the bit count, the byte offsets of
//! the next byte and of the packed-read limit, and the end-of-input flag. The bit
//! position of the stream is `8 * buf - bits`, which the state already implies.
//!
//! The streams cover every length from zero to 40 bytes, the lengths around the
//! seven-byte loads of a 56-bit `BITS` (a 64-bit refill), and longer seeded
//! streams. Each stream is read past its end, so `eof` and the zero shifts after
//! it are compared too.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::bit_reader_inl_utils::{vp8_get_bit, vp8_get_bit_alt, vp8_get_signed};
use webp::utils::bit_reader_utils::{
    VP8BitReader, vp8_get, vp8_get_signed_value, vp8_get_value, vp8_init_bit_reader,
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

    fn byte(&mut self) -> u8 {
        self.next_u64() as u8
    }
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// One reader command, as the probe reads it.
#[derive(Clone, Copy, Debug)]
enum Op {
    Bit(i32),
    BitAlt(i32),
    Value(i32),
    Get,
    SignedValue(i32),
    Signed(i32),
}

impl Op {
    fn text(self) -> String {
        match self {
            Op::Bit(prob) => format!("bit {prob}"),
            Op::BitAlt(prob) => format!("bit_alt {prob}"),
            Op::Value(bits) => format!("value {bits}"),
            Op::Get => "get".to_owned(),
            Op::SignedValue(bits) => format!("signed_value {bits}"),
            Op::Signed(value) => format!("signed {value}"),
        }
    }

    fn name(self) -> &'static str {
        match self {
            Op::Bit(_) => "bit",
            Op::BitAlt(_) => "bit_alt",
            Op::Value(_) => "value",
            Op::Get => "get",
            Op::SignedValue(_) => "signed_value",
            Op::Signed(_) => "signed",
        }
    }

    /// The probe prints `result=%u`, so the signed results are their unsigned
    /// images.
    fn execute(self, br: &mut VP8BitReader<'_>) -> u32 {
        match self {
            Op::Bit(prob) => vp8_get_bit(br, prob) as u32,
            Op::BitAlt(prob) => vp8_get_bit_alt(br, prob) as u32,
            Op::Value(bits) => vp8_get_value(br, bits),
            Op::Get => vp8_get(br),
            Op::SignedValue(bits) => vp8_get_signed_value(br, bits) as u32,
            Op::Signed(value) => vp8_get_signed(br, value) as u32,
        }
    }
}

/// The reader state, as the probe prints it after each command.
fn state(fields: Fields, br: &VP8BitReader<'_>) -> Fields {
    fields
        .with("value", br.value)
        .with("range", br.range)
        .with("bits", br.bits)
        .with("buf", br.buf)
        .with("buf_max", br.buf_max)
        .with("eof", u8::from(br.eof))
}

/// Probabilities as the decoder uses them: mostly any value in `0..=255`, with
/// the edges and the common values over-represented.
fn probability(rng: &mut Rng) -> i32 {
    match rng.below(8) {
        0 => 0,
        1 => 1,
        2 => 128,
        3 => 255,
        4 => 254,
        _ => rng.below(256) as i32,
    }
}

fn random_op(rng: &mut Rng) -> Op {
    match rng.below(20) {
        0..=6 => Op::Bit(probability(rng)),
        7..=8 => Op::BitAlt(probability(rng)),
        9..=10 => Op::Value(rng.below(25) as i32),
        11 => Op::Get,
        12..=13 => Op::SignedValue(rng.below(17) as i32),
        _ => {
            let value = match rng.below(4) {
                0 => rng.below(3) as i32 - 1,
                1 => rng.below(41) as i32 - 20,
                _ => rng.below(601) as i32 - 300,
            };
            Op::Signed(value)
        }
    }
}

/// The streams of the test: each length from zero to 40, the lengths around the
/// seven-byte and eight-byte loads, longer seeded streams and constant streams.
fn streams(rng: &mut Rng) -> Vec<Vec<u8>> {
    let mut streams: Vec<Vec<u8>> = Vec::new();
    for len in 0..=40 {
        streams.push((0..len).map(|_| rng.byte()).collect());
    }
    for len in [55, 56, 57, 63, 64, 65, 100, 255, 256, 1000, 4096] {
        streams.push((0..len).map(|_| rng.byte()).collect());
    }
    streams.push(vec![0x00; 33]);
    streams.push(vec![0xff; 33]);
    streams.push(
        (0..64)
            .map(|i| if i % 2 == 0 { 0xaa } else { 0x55 })
            .collect(),
    );
    streams
}

/// Runs the commands on the Rust reader and returns the expected probe output.
///
/// The command list and the expected fields are built together, so both sides
/// read the same stream with the same operations and the same count.
fn run_stream(data: &[u8], ops: &[Op], expected: &mut Fields) -> String {
    let mut script = format!("init {}\n", hex(data));
    let mut br = VP8BitReader::default();
    vp8_init_bit_reader(&mut br, data);
    *expected = state(std::mem::take(expected).with("op", "init"), &br);
    for &op in ops {
        script.push_str(&op.text());
        script.push('\n');
        let result = op.execute(&mut br);
        let fields = std::mem::take(expected)
            .with("op", op.name())
            .with("result", result);
        *expected = state(fields, &br);
    }
    script
}

/// Compares the probe output with the expected fields, reporting the first
/// difference with its neighbors.
fn assert_same(actual: &Fields, expected: &Fields) {
    if actual == expected {
        return;
    }
    let index = actual
        .0
        .iter()
        .zip(expected.0.iter())
        .position(|(a, e)| a != e)
        .unwrap_or_else(|| actual.0.len().min(expected.0.len()));
    let lo = index.saturating_sub(10);
    let hi = (index + 3).min(actual.0.len()).min(expected.0.len());
    panic!(
        "the probe and the Rust reader differ at field {index}:\n\
         probe:  {:?}\nrust:   {:?}\nprobe context: {:?}",
        actual.0.get(index),
        expected.0.get(index),
        &actual.0[lo..hi.max(lo)],
    );
}

#[test]
fn bit_reader_states_match_the_c_reader() {
    let Some(oracle) = Oracle::locate("bit_reader_states_match_the_c_reader") else {
        return;
    };
    let mut rng = Rng(0x5eed_2001);
    let mut script = String::new();
    let mut expected = Fields::new();
    let mut count = 0_usize;
    for data in streams(&mut rng) {
        // Enough operations to read past the end of the stream: each command
        // consumes at most 24 bits, and the input has `8 * len` bits.
        let ops: Vec<Op> = (0..(data.len() * 8 / 2 + 48))
            .map(|_| random_op(&mut rng))
            .collect();
        count += ops.len();
        script.push_str(&run_stream(&data, &ops, &mut expected));
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_reader")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
    assert!(count > 10_000, "the test must exercise many commands");
}

#[test]
fn truncated_streams_set_eof_and_shift_zeros() {
    let Some(oracle) = Oracle::locate("truncated_streams_set_eof_and_shift_zeros") else {
        return;
    };
    let mut rng = Rng(0x5eed_2002);
    let mut script = String::new();
    let mut expected = Fields::new();
    for len in [0_usize, 1, 2, 7, 8, 9] {
        let data: Vec<u8> = (0..len).map(|_| rng.byte()).collect();
        // Read 300 bits with probability one half: far past the input.
        let ops = vec![Op::Bit(128); 300];
        script.push_str(&run_stream(&data, &ops, &mut expected));
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_reader")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
}

#[test]
fn signed_and_value_reads_match_the_c_reader() {
    let Some(oracle) = Oracle::locate("signed_and_value_reads_match_the_c_reader") else {
        return;
    };
    let mut rng = Rng(0x5eed_2003);
    let mut script = String::new();
    let mut expected = Fields::new();
    for _ in 0..24 {
        let len = rng.below(200) as usize;
        let data: Vec<u8> = (0..len).map(|_| rng.byte()).collect();
        let ops: Vec<Op> = (0..(len * 4 + 16))
            .map(|i| match i % 4 {
                0 => Op::Value(rng.below(25) as i32),
                1 => Op::SignedValue(rng.below(17) as i32),
                2 => Op::Get,
                _ => Op::Signed(rng.below(511) as i32 - 255),
            })
            .collect();
        script.push_str(&run_stream(&data, &ops, &mut expected));
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_reader")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
}
