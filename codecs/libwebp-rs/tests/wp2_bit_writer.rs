//! WP2 differential test: `VP8BitWriter` (`src/utils/bit_writer_utils.c` and the
//! inline functions of its header). Seeded sequences of boolean-coded bits,
//! uniform bits, signed values, appends and finishes are run through the probe
//! `tools/oracle/probes/wp2/vp8_bit_writer.c` and through the Rust functions. The
//! probe prints the results, the encoder state and the allocated size after every
//! command, and the bytes whenever the test asks for them. The output bytes, the
//! carries, the growth policy and the first-error state are compared exactly.
//!
//! A `2^34 + 1` expected size exceeds `WEBP_MAX_ALLOCABLE_MEMORY`, so the
//! allocation is refused before any memory is reserved. The encoder then keeps
//! writing with its error flag set, as upstream does.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::bit_writer_utils::{
    VP8BitWriter, vp8_bit_writer_append, vp8_bit_writer_buf, vp8_bit_writer_finish,
    vp8_bit_writer_init, vp8_bit_writer_pos, vp8_bit_writer_size, vp8_bit_writer_wipe_out,
    vp8_put_bit, vp8_put_bit_uniform, vp8_put_bits, vp8_put_signed_bits,
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

/// `2^34 + 1` bytes: above `WEBP_MAX_ALLOCABLE_MEMORY` on 64-bit hosts.
const REFUSED_SIZE: usize = (1 << 34) + 1;

/// One encoder command.
#[derive(Clone, Debug)]
enum Op {
    Init(usize),
    Bit(bool, i32),
    Uniform(bool),
    Bits(u32, i32),
    Signed(i32, i32),
    Append(Vec<u8>),
    Finish,
    Bytes,
    Wipe,
}

/// The encoder state, as the probe prints it after each command.
fn state(fields: Fields, bw: &VP8BitWriter) -> Fields {
    fields
        .with("range", bw.range)
        .with("value", bw.value)
        .with("run", bw.run)
        .with("nb_bits", bw.nb_bits)
        .with("pos", bw.pos)
        .with("max_pos", bw.buf.len())
        .with("error", u8::from(bw.error))
        .with("pos_bits", vp8_bit_writer_pos(bw))
}

/// Applies one command to the Rust encoder, appends the probe line and the
/// expected fields.
fn apply(bw: &mut VP8BitWriter, op: &Op, script: &mut String, expected: &mut Fields) {
    let mut fields = Fields::new();
    let text = match op {
        Op::Init(size) => {
            let ok = vp8_bit_writer_init(bw, *size);
            fields = fields.with("op", "init").with("result", u8::from(ok));
            format!("init {size}")
        }
        Op::Bit(bit, prob) => {
            let result = vp8_put_bit(bw, *bit, *prob);
            fields = fields.with("op", "bit").with("result", u8::from(result));
            format!("bit {} {prob}", u8::from(*bit))
        }
        Op::Uniform(bit) => {
            let result = vp8_put_bit_uniform(bw, *bit);
            fields = fields
                .with("op", "uniform")
                .with("result", u8::from(result));
            format!("uniform {}", u8::from(*bit))
        }
        Op::Bits(value, bits) => {
            vp8_put_bits(bw, *value, *bits);
            fields = fields.with("op", "bits");
            format!("bits {value} {bits}")
        }
        Op::Signed(value, bits) => {
            vp8_put_signed_bits(bw, *value, *bits);
            fields = fields.with("op", "signed");
            format!("signed {value} {bits}")
        }
        Op::Append(data) => {
            let result = vp8_bit_writer_append(bw, data);
            fields = fields.with("op", "append").with("result", u8::from(result));
            format!("append {}", hex(data))
        }
        Op::Finish => {
            let _ = vp8_bit_writer_finish(bw);
            fields = fields
                .with("op", "finish")
                .with("size", vp8_bit_writer_size(bw));
            "finish".to_owned()
        }
        Op::Bytes => {
            let size = vp8_bit_writer_size(bw);
            fields = fields
                .with("op", "bytes")
                .hex("bytes", &vp8_bit_writer_buf(bw)[..size]);
            "bytes".to_owned()
        }
        Op::Wipe => {
            vp8_bit_writer_wipe_out(bw);
            fields = fields.with("op", "wipe");
            "wipe".to_owned()
        }
    };
    script.push_str(&text);
    script.push('\n');
    let mut all = std::mem::take(expected);
    all.0.extend(fields.0);
    *expected = state(all, bw);
}

fn random_op(rng: &mut Rng) -> Op {
    match rng.below(100) {
        0..=39 => Op::Bit(rng.below(2) == 1, rng.below(256) as i32),
        40..=54 => Op::Uniform(rng.below(2) == 1),
        55..=64 => {
            let bits = 1 + rng.below(31) as i32;
            Op::Bits(rng.next_u64() as u32, bits)
        }
        65..=76 => {
            let value = rng.below(401) as i32 - 200;
            Op::Signed(value, rng.below(21) as i32)
        }
        77..=82 => {
            let len = rng.below(6) as usize;
            Op::Append((0..len).map(|_| rng.byte()).collect())
        }
        83..=86 => Op::Bytes,
        _ => Op::Uniform(rng.below(2) == 1),
    }
}

/// A sequence: an initialization, random commands, then the final bytes.
fn sequence(rng: &mut Rng, size: usize, count: usize) -> Vec<Op> {
    let mut ops = vec![Op::Init(size)];
    ops.extend((0..count).map(|_| random_op(rng)));
    ops.extend([Op::Finish, Op::Bytes, Op::Wipe]);
    ops
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
        "the probe and the Rust encoder differ at field {index}:\n\
         probe:  {:?}\nrust:   {:?}\nprobe context: {:?}",
        actual.0.get(index),
        expected.0.get(index),
        &actual.0[lo..hi],
    );
}

fn run_all(oracle: &Oracle, sequences: &[Vec<Op>]) {
    let mut script = String::new();
    let mut expected = Fields::new();
    for ops in sequences {
        let mut bw = VP8BitWriter::default();
        for op in ops {
            apply(&mut bw, op, &mut script, &mut expected);
        }
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8_bit_writer")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
}

#[test]
fn bit_writer_states_and_bytes_match_the_c_writer() {
    let Some(oracle) = Oracle::locate("bit_writer_states_and_bytes_match_the_c_writer") else {
        return;
    };
    let mut rng = Rng(0x5eed_4001);
    let sizes = [0_usize, 1, 2, 5, 100, 1023, 1024, 1025, 4000, 70_000];
    let mut sequences = Vec::new();
    for (index, &size) in sizes.iter().cycle().take(20).enumerate() {
        let count = if index % 5 == 0 {
            12_000
        } else {
            600 + rng.below(1500) as usize
        };
        sequences.push(sequence(&mut rng, size, count));
    }
    run_all(&oracle, &sequences);
}

#[test]
fn refused_allocation_sets_the_error_and_writing_continues() {
    let Some(oracle) = Oracle::locate("refused_allocation_sets_the_error_and_writing_continues")
    else {
        return;
    };
    let mut rng = Rng(0x5eed_4002);
    let mut sequences = Vec::new();
    for _ in 0..4 {
        let mut ops = vec![Op::Init(REFUSED_SIZE)];
        ops.extend((0..800).map(|_| random_op(&mut rng)));
        ops.extend([Op::Finish, Op::Bytes, Op::Wipe]);
        sequences.push(ops);
    }
    // An unaligned append is refused; an aligned one is accepted after a flush.
    sequences.push(vec![
        Op::Init(0),
        Op::Uniform(true),
        Op::Append(vec![1, 2, 3]),
        Op::Finish,
        Op::Append(vec![1, 2, 3]),
        Op::Bytes,
        Op::Wipe,
    ]);
    run_all(&oracle, &sequences);
}
