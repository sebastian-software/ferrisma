//! WP2 differential test: `VP8LBitReader` (`src/utils/bit_reader_utils.c` and the
//! inline functions of its header). Seeded streams, streams that end at every
//! length from zero to 40 bytes, and reads that cross the 64-bit prefetch window
//! are driven through the probe `tools/oracle/probes/wp2/vp8l_bit_reader.c` and
//! through the Rust functions with the same commands. After each command both
//! sides report the result, when the command has one, and the state: the window,
//! the byte position, the length, the bit position and the end-of-stream flag.
//!
//! Some calls are issued only where upstream allows them. `VP8LDoFillBitWindow`
//! requires `bit_pos >= VP8L_WBITS`, and `VP8LBitReaderSetBuffer` treats a buffer
//! shorter than the position as a parameter error. The driver consults the Rust
//! reader's state and substitutes `fill` for either call. The two sides agree on
//! every step, so the substitution is the same on both.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::bit_reader_utils::{
    VP8LBitReader, vp8l_bit_reader_set_buffer, vp8l_do_fill_bit_window, vp8l_fill_bit_window,
    vp8l_init_bit_reader, vp8l_is_end_of_stream, vp8l_prefetch_bits, vp8l_read_bits,
    vp8l_set_bit_pos,
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

/// One reader command. `Init` and `Set` name a buffer by its index.
#[derive(Clone, Copy, Debug)]
enum Op {
    Init(usize),
    Set(usize),
    Read(i32),
    Prefetch,
    SetBitPos(i32),
    Fill,
    DoFill,
    Eos,
}

fn random_op(rng: &mut Rng, buffers: usize) -> Op {
    match rng.below(40) {
        0 => Op::Set(rng.below(buffers as u64) as usize),
        1..=12 => Op::Read(if rng.below(10) == 0 {
            25 + rng.below(8) as i32 // past VP8L_MAX_NUM_BIT_READ: sets eos
        } else {
            rng.below(25) as i32
        }),
        13..=14 => Op::Prefetch,
        15..=17 => Op::SetBitPos(match rng.below(4) {
            0 => 32 + rng.below(9) as i32,
            _ => rng.below(64) as i32,
        }),
        18..=22 => Op::Fill,
        23..=28 => Op::DoFill,
        _ => Op::Eos,
    }
}

/// Applies one command to the Rust reader, appends the probe line and the
/// expected fields.
fn apply<'a>(
    br: &mut VP8LBitReader<'a>,
    buffers: &'a [Vec<u8>],
    op: Op,
    script: &mut String,
    expected: &mut Fields,
) {
    let (text, name, result): (String, &str, Option<u32>) = match op {
        Op::Init(index) => {
            vp8l_init_bit_reader(br, buffers[index].as_slice());
            (format!("init {}", hex(&buffers[index])), "init", None)
        }
        Op::Set(index) => {
            vp8l_bit_reader_set_buffer(br, buffers[index].as_slice());
            (format!("set {}", hex(&buffers[index])), "set", None)
        }
        Op::Read(bits) => {
            let value = vp8l_read_bits(br, bits);
            (format!("read {bits}"), "read", Some(value))
        }
        Op::Prefetch => (
            "prefetch".to_owned(),
            "prefetch",
            Some(vp8l_prefetch_bits(br)),
        ),
        Op::SetBitPos(value) => {
            vp8l_set_bit_pos(br, value);
            (format!("set_bit_pos {value}"), "set_bit_pos", None)
        }
        Op::Fill => {
            vp8l_fill_bit_window(br);
            ("fill".to_owned(), "fill", None)
        }
        Op::DoFill => {
            vp8l_do_fill_bit_window(br);
            ("do_fill".to_owned(), "do_fill", None)
        }
        Op::Eos => (
            "eos".to_owned(),
            "eos",
            Some(u32::from(vp8l_is_end_of_stream(br))),
        ),
    };
    script.push_str(&text);
    script.push('\n');
    let mut fields = std::mem::take(expected).with("op", name);
    if let Some(value) = result {
        fields = fields.with("result", value);
    }
    *expected = fields
        .with("val", br.val)
        .with("pos", br.pos)
        .with("len", br.buf.len())
        .with("bit_pos", br.bit_pos)
        .with("eos", u8::from(br.eos));
}

/// Runs `ops` from a fresh reader that starts on `buffers[first]`, substituting
/// `Fill` for a `DoFill` that the upstream precondition forbids.
fn drive(
    buffers: &[Vec<u8>],
    first: usize,
    ops: &[Op],
    script: &mut String,
    expected: &mut Fields,
) {
    let mut br = VP8LBitReader::default();
    apply(&mut br, buffers, Op::Init(first), script, expected);
    for &op in ops {
        let op = match op {
            Op::DoFill if br.bit_pos < 32 => Op::Fill,
            // A buffer shorter than the position is a parameter error upstream
            // (`VP8LBitReaderSetBuffer` says so, and its debug assertion fires).
            Op::Set(index) if buffers[index].len() < br.pos => Op::Fill,
            other => other,
        };
        apply(&mut br, buffers, op, script, expected);
    }
}

fn buffers(rng: &mut Rng) -> Vec<Vec<u8>> {
    let mut buffers: Vec<Vec<u8>> = (0..=40)
        .map(|len| (0..len).map(|_| rng.byte()).collect())
        .collect();
    for len in [63, 64, 65, 66, 100, 255, 1000] {
        buffers.push((0..len).map(|_| rng.byte()).collect());
    }
    buffers
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
fn lossless_reader_states_match_the_c_reader() {
    let Some(oracle) = Oracle::locate("lossless_reader_states_match_the_c_reader") else {
        return;
    };
    let mut rng = Rng(0x5eed_3001);
    let buffers = buffers(&mut rng);
    let mut script = String::new();
    let mut expected = Fields::new();
    let mut count = 0_usize;
    for round in 0..(4 * buffers.len()) {
        let first = round % buffers.len();
        let steps = 40 + buffers[first].len() * 2;
        let ops: Vec<Op> = (0..steps)
            .map(|_| random_op(&mut rng, buffers.len()))
            .collect();
        count += ops.len();
        drive(&buffers, first, &ops, &mut script, &mut expected);
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8l_bit_reader")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
    assert!(count > 10_000, "the test must exercise many commands");
}

#[test]
fn prefetch_window_refills_match_the_c_reader() {
    let Some(oracle) = Oracle::locate("prefetch_window_refills_match_the_c_reader") else {
        return;
    };
    // Every stream length around the eight-byte window and its four-byte refill,
    // read with the widest reads and a fill after each one.
    let buffers: Vec<Vec<u8>> = (0..=40_u8)
        .map(|len| (0..len).map(|i| i ^ 0xa5).collect())
        .collect();
    let cycle = [
        Op::Read(24),
        Op::Read(24),
        Op::Fill,
        Op::Read(5),
        Op::DoFill,
        Op::Read(13),
        Op::Eos,
        Op::SetBitPos(40),
        Op::DoFill,
        Op::Prefetch,
        Op::Read(24),
    ];
    let mut script = String::new();
    let mut expected = Fields::new();
    for first in 0..buffers.len() {
        let ops: Vec<Op> = std::iter::repeat_n(cycle, 6).flatten().collect();
        drive(&buffers, first, &ops, &mut script, &mut expected);
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8l_bit_reader")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
}

#[test]
fn set_buffer_keeps_the_position_and_recomputes_eos() {
    let Some(oracle) = Oracle::locate("set_buffer_keeps_the_position_and_recomputes_eos") else {
        return;
    };
    let mut rng = Rng(0x5eed_3002);
    let buffers = buffers(&mut rng);
    let mut script = String::new();
    let mut expected = Fields::new();
    for round in 0..buffers.len() {
        // Read to the end of one buffer, then move to a shorter or longer one.
        let mut ops = vec![Op::Set(round), Op::Read(24), Op::Read(24), Op::Read(24)];
        ops.extend((0..8).map(|_| random_op(&mut rng, buffers.len())));
        drive(&buffers, round, &ops, &mut script, &mut expected);
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8l_bit_reader")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
}
