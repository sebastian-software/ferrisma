//! WP2 differential test: `VP8LBitWriter` (`src/utils/bit_writer_utils.c` and the
//! inline functions of its header). Two writers receive seeded sequences of
//! `VP8LPutBits` calls with widths from zero to 32, and the lifecycle calls
//! `Init`, `Clone`, `Reset`, `Swap`, `Finish` and `WipeOut`. The probe
//! `tools/oracle/probes/wp2/vp8l_bit_writer.c` runs the same commands in its own
//! process. After every command both sides report the written bytes or the
//! finished stream when the command returns them, and the state of both writers:
//! the accumulator, the number of used bits, the write offset, the allocated size
//! and the error flag. The byte output, the growth policy and the error behavior
//! are compared exactly.
//!
//! The generator runs the Rust writers in lockstep, because some calls are
//! undefined in C, and the test must not issue them (ADR 0002 excludes such
//! inputs explicitly). The exclusions, each found by the `asan-ubsan` oracle:
//!
//! - `VP8LPutBits` with 32 bits at `used == 0` on the 16-bit accumulator path: C
//!   shifts a `uint32_t` by 32. The generator narrows such a call to 31 bits.
//! - `VP8LBitWriterFinish` leaves the cursor at a byte offset, and the next write
//!   word is then stored at an odd address (`bit_writer_utils.c`, the
//!   `*(vp8l_wtype_t*)` store). The sequences finish and wipe the writers only at
//!   their end, as the encoder does.
//! - A write into a writer without a buffer, which a refused allocation leaves
//!   behind. C adds the cursor offset to a null pointer. The refused-allocation
//!   test issues only writes that do not flush, then a finish that allocates.
//! - `VP8LBitWriterReset` to a cursor whose bytes the destination no longer holds:
//!   a resize copies only the bytes before the cursor, and C then reads memory it
//!   never wrote. The reset follows a clone of the same source, as the encoder does.
//! - `VP8LBitWriterClone` of a writer without a buffer, which passes a null
//!   pointer to `memcpy`. No sequence wipes a writer before it is cloned.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::bit_writer_utils::{
    VP8L_WRITER_BITS, VP8L_WRITER_BYTES, VP8LBitWriter, vp8l_bit_writer_clone,
    vp8l_bit_writer_finish, vp8l_bit_writer_init, vp8l_bit_writer_num_bytes, vp8l_bit_writer_reset,
    vp8l_bit_writer_swap, vp8l_bit_writer_wipe_out, vp8l_put_bits,
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
}

/// `2^34 + 1` bytes: above `WEBP_MAX_ALLOCABLE_MEMORY` on 64-bit hosts.
const REFUSED_SIZE: usize = (1 << 34) + 1;

/// One command. Writers are named by index, `0` or `1`.
#[derive(Clone, Copy, Debug)]
enum Op {
    Init(usize, usize),
    Put(usize, u32, i32),
    Written(usize),
    NumBytes(usize),
    Finish(usize),
    Clone(usize, usize),
    Reset(usize, usize),
    Swap,
    Wipe(usize),
}

/// The state of one writer, as the probe prints it.
fn writer_state(fields: Fields, prefix: &str, bw: &VP8LBitWriter) -> Fields {
    fields
        .with(&format!("{prefix}_bits"), bw.bits)
        .with(&format!("{prefix}_used"), bw.used)
        .with(&format!("{prefix}_cur"), bw.cur)
        .with(&format!("{prefix}_alloc"), bw.buf.len())
        .with(&format!("{prefix}_error"), u8::from(bw.error))
}

/// Applies one command to the Rust writers, appends the probe line and the
/// expected fields.
fn apply(writers: &mut [VP8LBitWriter; 2], op: Op, script: &mut String, expected: &mut Fields) {
    let mut fields = Fields::new();
    let text = match op {
        Op::Init(w, size) => {
            let ok = vp8l_bit_writer_init(&mut writers[w], size);
            fields = fields.with("op", "init").with("result", u8::from(ok));
            format!("init {w} {size}")
        }
        Op::Put(w, value, bits) => {
            vp8l_put_bits(&mut writers[w], value, bits);
            fields = fields.with("op", "put");
            format!("put {w} {value} {bits}")
        }
        Op::Written(w) => {
            let cur = writers[w].cur;
            fields = fields
                .with("op", "written")
                .hex("written", &writers[w].buf[..cur]);
            format!("written {w}")
        }
        Op::NumBytes(w) => {
            let n = vp8l_bit_writer_num_bytes(&writers[w]);
            fields = fields.with("op", "num_bytes").with("result", n);
            format!("num_bytes {w}")
        }
        Op::Finish(w) => {
            vp8l_bit_writer_finish(&mut writers[w]);
            let n = vp8l_bit_writer_num_bytes(&writers[w]);
            fields = fields
                .with("op", "finish")
                .hex("finished", &writers[w].buf[..n]);
            format!("finish {w}")
        }
        Op::Clone(src, dst) => {
            let (a, b) = pair(writers, src, dst);
            let ok = vp8l_bit_writer_clone(a, b);
            fields = fields.with("op", "clone").with("result", u8::from(ok));
            format!("clone {src} {dst}")
        }
        Op::Reset(src, dst) => {
            let (a, b) = pair(writers, src, dst);
            vp8l_bit_writer_reset(a, b);
            fields = fields.with("op", "reset");
            format!("reset {src} {dst}")
        }
        Op::Swap => {
            let (a, b) = writers.split_at_mut(1);
            vp8l_bit_writer_swap(&mut a[0], &mut b[0]);
            fields = fields.with("op", "swap");
            "swap 0 1".to_owned()
        }
        Op::Wipe(w) => {
            vp8l_bit_writer_wipe_out(&mut writers[w]);
            fields = fields.with("op", "wipe");
            format!("wipe {w}")
        }
    };
    script.push_str(&text);
    script.push('\n');
    let mut all = std::mem::take(expected);
    all.0.extend(fields.0);
    let all = writer_state(all, "w0", &writers[0]);
    *expected = writer_state(all, "w1", &writers[1]);
}

/// The source as a shared borrow and the destination as a mutable one.
fn pair(
    writers: &mut [VP8LBitWriter; 2],
    src: usize,
    dst: usize,
) -> (&VP8LBitWriter, &mut VP8LBitWriter) {
    assert_ne!(src, dst);
    let (first, second) = writers.split_at_mut(1);
    if src == 0 {
        (&first[0], &mut second[0])
    } else {
        (&second[0], &mut first[0])
    }
}

/// A random value of `bits` bits, from every bit pattern of the width.
fn value(rng: &mut Rng, bits: i32) -> u32 {
    let raw = rng.next_u64() as u32;
    if bits == 32 {
        raw
    } else {
        raw & ((1_u32 << bits) - 1)
    }
}

/// Chooses the next command against the current state of the Rust writers.
fn put(rng: &mut Rng, writers: &[VP8LBitWriter; 2], w: usize) -> Op {
    let mut bits = match rng.below(4) {
        0 => rng.below(33) as i32,
        1 => rng.below(17) as i32,
        _ => rng.below(25) as i32,
    };
    // C shifts a uint32_t by 32 when it writes 32 bits at used == 0 on the
    // 16-bit accumulator path; that input is undefined and excluded.
    if bits == 32 && writers[w].used == 0 && VP8L_WRITER_BYTES == 2 {
        bits = 31;
    }
    Op::Put(w, value(rng, bits), bits)
}

/// Chooses the next command. `Finish` and `WipeOut` are not chosen here: the
/// encoder finishes a writer once, at the end, and then wipes it, and the module
/// documentation records why the sequences keep them at their end.
fn choose(rng: &mut Rng, writers: &[VP8LBitWriter; 2]) -> Op {
    let w = rng.below(2) as usize;
    match rng.below(100) {
        0..=79 => put(rng, writers, w),
        80..=82 => Op::Written(w),
        83..=87 => Op::NumBytes(w),
        88..=93 => Op::Clone(w, 1 - w),
        94..=96 => Op::Swap,
        _ => {
            let sizes = [0_usize, 1, 5, 1000, 1023, 1024, 1025, 40_000];
            Op::Init(w, sizes[rng.below(sizes.len() as u64) as usize])
        }
    }
}

/// Clones `src` into `dst`, grows `dst` only, and resets `dst` to the cursor of
/// `src`. `VP8LBitWriterReset` copies no bytes, so the destination must still hold
/// the source's bytes; this is the pattern of the encoder (`vp8l_enc.c`). Without
/// the clone, a resize may have dropped the bytes before the reset point, which C
/// then reads uninitialized.
fn reset_round(
    rng: &mut Rng,
    writers: &mut [VP8LBitWriter; 2],
    script: &mut String,
    expected: &mut Fields,
) {
    let src = rng.below(2) as usize;
    let dst = 1 - src;
    apply(writers, Op::Clone(src, dst), script, expected);
    for _ in 0..(1 + rng.below(40)) {
        let op = put(rng, writers, dst);
        apply(writers, op, script, expected);
    }
    apply(writers, Op::Reset(src, dst), script, expected);
}

/// Runs one sequence from fresh writers and returns the probe output, which the
/// caller compares.
fn sequence(rng: &mut Rng, count: usize, script: &mut String, expected: &mut Fields) {
    let mut writers: [VP8LBitWriter; 2] = [VP8LBitWriter::default(), VP8LBitWriter::default()];
    let sizes = [0_usize, 1, 64, 1000, 1024, 3000];
    apply(
        &mut writers,
        Op::Init(0, sizes[rng.below(sizes.len() as u64) as usize]),
        script,
        expected,
    );
    apply(
        &mut writers,
        Op::Init(1, sizes[rng.below(sizes.len() as u64) as usize]),
        script,
        expected,
    );
    for _ in 0..count {
        if rng.below(60) == 0 {
            reset_round(rng, &mut writers, script, expected);
        } else {
            let op = choose(rng, &writers);
            apply(&mut writers, op, script, expected);
        }
    }
    for w in 0..2 {
        apply(&mut writers, Op::Finish(w), script, expected);
        apply(&mut writers, Op::Written(w), script, expected);
        apply(&mut writers, Op::Wipe(w), script, expected);
    }
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
        "the probe and the Rust writer differ at field {index}:\n\
         probe:  {:?}\nrust:   {:?}\nprobe context: {:?}",
        actual.0.get(index),
        expected.0.get(index),
        &actual.0[lo..hi],
    );
}

#[test]
fn lossless_writer_states_and_bytes_match_the_c_writer() {
    let Some(oracle) = Oracle::locate("lossless_writer_states_and_bytes_match_the_c_writer") else {
        return;
    };
    let mut rng = Rng(0x5eed_5001);
    let mut script = String::new();
    let mut expected = Fields::new();
    for _ in 0..6 {
        sequence(&mut rng, 6000, &mut script, &mut expected);
    }
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8l_bit_writer")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
}

#[test]
fn refused_allocation_sets_the_error_and_writing_continues() {
    let Some(oracle) = Oracle::locate("refused_allocation_sets_the_error_and_writing_continues")
    else {
        return;
    };
    let mut rng = Rng(0x5eed_5002);
    let mut script = String::new();
    let mut expected = Fields::new();
    let mut writers: [VP8LBitWriter; 2] = [VP8LBitWriter::default(), VP8LBitWriter::default()];
    apply(
        &mut writers,
        Op::Init(0, REFUSED_SIZE),
        &mut script,
        &mut expected,
    );
    apply(&mut writers, Op::Init(1, 0), &mut script, &mut expected);
    // Writer 0 has no buffer. A flush would write into it, which C leaves
    // undefined (it adds an offset to a null pointer), so only puts that keep
    // the accumulator below one write word are issued.
    for _ in 0..40 {
        let bits = rng.below(VP8L_WRITER_BITS as u64) as i32;
        if writers[0].used + bits < VP8L_WRITER_BITS {
            let op = Op::Put(0, value(&mut rng, bits), bits);
            apply(&mut writers, op, &mut script, &mut expected);
        }
    }
    apply(&mut writers, Op::NumBytes(0), &mut script, &mut expected);
    apply(&mut writers, Op::Finish(0), &mut script, &mut expected);
    apply(&mut writers, Op::Written(0), &mut script, &mut expected);
    // Writer 1 carries on with ordinary writes; the clone copies the error flag.
    for _ in 0..2000 {
        let op = if rng.below(10) == 0 {
            Op::NumBytes(1)
        } else {
            put(&mut rng, &writers, 1)
        };
        apply(&mut writers, op, &mut script, &mut expected);
    }
    apply(&mut writers, Op::Clone(0, 1), &mut script, &mut expected);
    apply(&mut writers, Op::NumBytes(1), &mut script, &mut expected);
    apply(&mut writers, Op::Written(1), &mut script, &mut expected);
    let args: [&str; 0] = [];
    let output = oracle
        .probe("wp2", "vp8l_bit_writer")
        .run(&args, script.as_bytes());
    assert_same(&output.success().fields(), &expected);
}
