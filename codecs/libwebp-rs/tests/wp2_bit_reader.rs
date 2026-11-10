//! WP2 differential test: `VP8BitReader` (`src/utils/bit_reader_utils.c` and
//! `src/utils/bit_reader_inl_utils.h`). Seeded and truncated streams are read by
//! the probe `tools/oracle/probes/wp2/vp8_bit_reader.c` and by the Rust functions
//! with the same commands. The reader holds positions, not references, so the
//! test plays the owner: it keeps the bytes in a buffer, passes the buffer to each
//! read, and moves the input inside the buffer with `VP8RemapBitReader`.
//!
//! After every command both sides report the result and the reader state: the
//! register, the range, the bit count, whether the reader has a buffer (C
//! `buf != NULL`), the indexes of the next byte, the end and the packed-read limit,
//! and the end-of-input flag. Positions are indexes into the owner's buffer on both
//! sides.
//!
//! The streams cover every length from zero to 40 bytes, the lengths around the
//! seven-byte loads of a 56-bit `BITS`, longer seeded streams, and constant
//! streams. Each stream is read past its end, and it is moved inside its buffer at
//! random points, so `VP8RemapBitReader` is compared with the C pointer arithmetic.
//! A reader that has no buffer receives remaps that must not change it.
//!
//! The test moves the input inside one allocation only, so that the C pointer
//! difference and the index delta are the same number (see
//! `src/utils/bit_reader_utils.rs`, `# Deviations from upstream`, item 2).

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::bit_reader_inl_utils::{vp8_get_bit, vp8_get_bit_alt, vp8_get_signed};
use webp::utils::bit_reader_utils::{
    VP8BitReader, vp8_get, vp8_get_signed_value, vp8_get_value, vp8_init_bit_reader,
    vp8_remap_bit_reader,
};

/// Bytes after the input in the owner's buffer, so that the input can move.
/// Matches `PAD` of the probe.
const PAD: usize = 64;

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

/// One reader command that reads the input.
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
    fn execute(self, br: &mut VP8BitReader, data: &[u8]) -> u32 {
        match self {
            Op::Bit(prob) => vp8_get_bit(br, data, prob) as u32,
            Op::BitAlt(prob) => vp8_get_bit_alt(br, data, prob) as u32,
            Op::Value(bits) => vp8_get_value(br, data, bits),
            Op::Get => vp8_get(br, data),
            Op::SignedValue(bits) => vp8_get_signed_value(br, data, bits) as u32,
            Op::Signed(value) => vp8_get_signed(br, data, value) as u32,
        }
    }
}

/// The owner of a reader: the reader, its buffer, and the index of the input in
/// the buffer. It builds the probe script and the expected output together.
#[derive(Default)]
struct Owner {
    br: VP8BitReader,
    storage: Vec<u8>,
    base: usize,
    size: usize,
    script: String,
    expected: Fields,
}

impl Owner {
    /// The reader state, as the probe prints it after each command.
    fn state(&self, fields: Fields) -> Fields {
        let br = &self.br;
        let index = |value: usize| if br.has_buf { value } else { 0 };
        fields
            .with("value", br.value)
            .with("range", br.range)
            .with("bits", br.bits)
            .with("attached", u8::from(br.has_buf))
            .with("buf", index(br.buf))
            .with("buf_end", index(br.buf_end))
            .with("buf_max", index(br.buf_max))
            .with("eof", u8::from(br.eof))
    }

    /// Appends one probe line and the expected fields of its command.
    fn record(&mut self, text: &str, name: &str, result: Option<u32>) {
        self.script.push_str(text);
        self.script.push('\n');
        let mut fields = std::mem::take(&mut self.expected).with("op", name);
        if let Some(value) = result {
            fields = fields.with("result", value);
        }
        self.expected = self.state(fields);
    }

    /// `VP8RemapBitReader` on a reader without a buffer, which changes nothing.
    fn remap_null(&mut self, offset: isize) {
        assert!(!self.br.has_buf, "the reader must not have a buffer");
        vp8_remap_bit_reader(&mut self.br, offset);
        self.record(&format!("remap_null {offset}"), "remap_null", None);
    }

    fn reset(&mut self) {
        self.br = VP8BitReader::default();
        self.storage.clear();
        self.base = 0;
        self.size = 0;
        self.record("reset", "reset", None);
    }

    fn init(&mut self, stream: &[u8]) {
        self.storage = stream.to_vec();
        self.storage.resize(stream.len() + PAD, 0);
        self.base = 0;
        self.size = stream.len();
        vp8_init_bit_reader(&mut self.br, &self.storage[..self.size]);
        self.record(&format!("init {}", hex(stream)), "init", None);
    }

    /// Moves the input to index `target` of the buffer, and remaps the reader by
    /// the index delta, as the owner does when its bytes move.
    fn move_to(&mut self, target: usize) {
        assert!(
            self.br.has_buf,
            "only a reader with a buffer moves its input"
        );
        self.storage
            .copy_within(self.base..self.base + self.size, target);
        let offset = target as isize - self.base as isize;
        vp8_remap_bit_reader(&mut self.br, offset);
        self.base = target;
        self.record(&format!("move {target}"), "move", None);
    }

    fn read(&mut self, op: Op) {
        let result = op.execute(&mut self.br, &self.storage);
        self.record(&op.text(), op.name(), Some(result));
    }
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

/// Reads `stream` with `count` random commands. About one command in 64 moves the
/// input inside the buffer, to a random index up to `PAD`.
fn read_stream(owner: &mut Owner, rng: &mut Rng, stream: &[u8], count: usize) {
    owner.reset();
    let offset = rng.below(17) as isize - 8;
    owner.remap_null(offset);
    owner.init(stream);
    for _ in 0..count {
        if rng.below(64) == 0 {
            owner.move_to(rng.below(PAD as u64 + 1) as usize);
        } else {
            owner.read(random_op(rng));
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
    let hi = (index + 3).min(actual.0.len()).max(lo);
    panic!(
        "the probe and the Rust reader differ at field {index}:\n\
         probe:  {:?}\nrust:   {:?}\nprobe context: {:?}",
        actual.0.get(index),
        expected.0.get(index),
        &actual.0[lo..hi],
    );
}

#[test]
fn bit_reader_states_match_the_c_reader() {
    let Some(oracle) = Oracle::locate("bit_reader_states_match_the_c_reader") else {
        return;
    };
    let mut rng = Rng(0x5eed_2001);
    let mut owner = Owner::default();
    let mut count = 0_usize;
    for stream in streams(&mut rng) {
        // Enough commands to read past the end of the stream: each consumes at
        // most 24 bits, and the input has `8 * len` bits.
        let commands = stream.len() * 8 / 2 + 48;
        count += commands;
        read_stream(&mut owner, &mut rng, &stream, commands);
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_reader")
        .run(&args, owner.script.as_bytes());
    assert_same(&output.success().fields(), &owner.expected);
    assert!(count > 10_000, "the test must exercise many commands");
}

#[test]
fn truncated_streams_set_eof_and_shift_zeros() {
    let Some(oracle) = Oracle::locate("truncated_streams_set_eof_and_shift_zeros") else {
        return;
    };
    let mut rng = Rng(0x5eed_2002);
    let mut owner = Owner::default();
    for len in [0_usize, 1, 2, 7, 8, 9] {
        let stream: Vec<u8> = (0..len).map(|_| rng.byte()).collect();
        // Read 300 bits with probability one half: far past the input.
        owner.reset();
        owner.init(&stream);
        for _ in 0..300 {
            owner.read(Op::Bit(128));
        }
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_reader")
        .run(&args, owner.script.as_bytes());
    assert_same(&output.success().fields(), &owner.expected);
}

#[test]
fn signed_and_value_reads_match_the_c_reader() {
    let Some(oracle) = Oracle::locate("signed_and_value_reads_match_the_c_reader") else {
        return;
    };
    let mut rng = Rng(0x5eed_2003);
    let mut owner = Owner::default();
    for _ in 0..24 {
        let len = rng.below(200) as usize;
        let stream: Vec<u8> = (0..len).map(|_| rng.byte()).collect();
        owner.reset();
        owner.init(&stream);
        for i in 0..(len * 4 + 16) {
            let op = match i % 4 {
                0 => Op::Value(rng.below(25) as i32),
                1 => Op::SignedValue(rng.below(17) as i32),
                2 => Op::Get,
                _ => Op::Signed(rng.below(511) as i32 - 255),
            };
            owner.read(op);
        }
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_reader")
        .run(&args, owner.script.as_bytes());
    assert_same(&output.success().fields(), &owner.expected);
}

#[test]
fn remap_moves_the_positions_and_leaves_a_reader_without_buffer_alone() {
    let Some(oracle) =
        Oracle::locate("remap_moves_the_positions_and_leaves_a_reader_without_buffer_alone")
    else {
        return;
    };
    let mut rng = Rng(0x5eed_2004);
    let mut owner = Owner::default();
    // A reader without a buffer, remapped by several offsets, changes nothing.
    owner.reset();
    for offset in [-8_isize, -1, 0, 1, 7, 64, 1000] {
        owner.remap_null(offset);
    }
    // Then the input is moved in and out of its buffer, to the first and the last
    // index, and read between the moves.
    for len in [0_usize, 1, 7, 8, 9, 40, 300] {
        let stream: Vec<u8> = (0..len).map(|_| rng.byte()).collect();
        owner.reset();
        owner.init(&stream);
        for target in [PAD, 0, 1, PAD / 2, 0, PAD] {
            for _ in 0..12 {
                owner.read(random_op(&mut rng));
            }
            owner.move_to(target);
        }
        for _ in 0..40 {
            owner.read(random_op(&mut rng));
        }
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_reader")
        .run(&args, owner.script.as_bytes());
    assert_same(&output.success().fields(), &owner.expected);
}
