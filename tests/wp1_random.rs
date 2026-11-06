//! WP1 differential test: the dithering generator `VP8Random`
//! (`src/utils/random_utils.c` and `.h`). One script of commands drives both the
//! C probe `tools/oracle/probes/wp1/random.c` and the Rust translation, value
//! by value: initializations with dithering amplitudes from below zero to above
//! one, long sequences of `VP8RandomBits`, and seeded states with arbitrary
//! table words, indices and amplitudes that run `VP8RandomBits2` with every
//! amplitude that the encoder uses. After each run the full state is compared.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::random_utils::{
    VP8_RANDOM_TABLE_SIZE, VP8Random, vp8_init_random, vp8_random_bits, vp8_random_bits2,
};

/// Amplitudes in VP8_RANDOM_DITHER_FIX fixed point; larger values overflow the
/// C product `diff * amp` for the widest `num_bits`, which the C code leaves
/// undefined.
const AMPLITUDES: [i32; 6] = [0, 1, 7, 128, 255, 256];

enum Command {
    /// The bits of the float dithering amplitude.
    Init(u32),
    Set {
        index1: usize,
        index2: usize,
        amp: i32,
        tab: Box<[u32; VP8_RANDOM_TABLE_SIZE]>,
    },
    Bits2 {
        num_bits: i32,
        amp: i32,
    },
    Bits {
        num_bits: i32,
    },
    Dump,
}

/// SplitMix64, a small deterministic generator for the script.
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

fn script() -> Vec<Command> {
    let mut commands = Vec::new();
    // Initializations, including clamped values and both signed zeros.
    let amplitudes = [
        -1.0_f32,
        -0.0,
        0.0,
        0.001,
        0.25,
        0.5,
        0.749,
        0.999,
        1.0,
        1.5,
        f32::INFINITY,
        0.123_456,
    ];
    for amplitude in amplitudes {
        commands.push(Command::Init(amplitude.to_bits()));
        commands.push(Command::Dump);
        for step in 0..250 {
            commands.push(Command::Bits {
                num_bits: 1 + step % 23,
            });
        }
        commands.push(Command::Dump);
    }
    // Seeded states with arbitrary table words and indices.
    let mut rng = Rng(0x5eed_0001);
    for _ in 0..8 {
        let mut tab = [0_u32; VP8_RANDOM_TABLE_SIZE];
        for word in &mut tab {
            *word = rng.next_u64() as u32;
        }
        let index1 = rng.below(VP8_RANDOM_TABLE_SIZE as u64) as usize;
        let index2 = rng.below(VP8_RANDOM_TABLE_SIZE as u64) as usize;
        let amp = AMPLITUDES[rng.below(AMPLITUDES.len() as u64) as usize];
        commands.push(Command::Set {
            index1,
            index2,
            amp,
            tab: Box::new(tab),
        });
        for _ in 0..150 {
            let num_bits = 1 + rng.below(23) as i32;
            let amp = AMPLITUDES[rng.below(AMPLITUDES.len() as u64) as usize];
            commands.push(Command::Bits2 { num_bits, amp });
        }
        commands.push(Command::Dump);
    }
    commands
}

/// Runs the script on the Rust generator, returning the probe's standard input
/// and the output that the probe must print.
fn run_rust(commands: &[Command]) -> (String, Fields) {
    let mut rg = VP8Random {
        index1: 0,
        index2: 0,
        tab: [0; VP8_RANDOM_TABLE_SIZE],
        amp: 0,
    };
    let mut stdin = String::new();
    let mut expected = Fields::new();
    for command in commands {
        match command {
            Command::Init(bits) => {
                stdin.push_str(&format!("init {bits}\n"));
                vp8_init_random(&mut rg, f32::from_bits(*bits));
            }
            Command::Set {
                index1,
                index2,
                amp,
                tab,
            } => {
                let bytes: Vec<u8> = tab.iter().flat_map(|word| word.to_le_bytes()).collect();
                stdin.push_str(&format!("set {index1} {index2} {amp} {}\n", hex(&bytes)));
                rg.index1 = *index1;
                rg.index2 = *index2;
                rg.amp = *amp;
                rg.tab = **tab;
            }
            Command::Bits2 { num_bits, amp } => {
                stdin.push_str(&format!("bits2 {num_bits} {amp}\n"));
                expected = expected.with("value", vp8_random_bits2(&mut rg, *num_bits, *amp));
            }
            Command::Bits { num_bits } => {
                stdin.push_str(&format!("bits {num_bits}\n"));
                expected = expected.with("value", vp8_random_bits(&mut rg, *num_bits));
            }
            Command::Dump => {
                stdin.push_str("dump\n");
                let bytes: Vec<u8> = rg.tab.iter().flat_map(|word| word.to_le_bytes()).collect();
                expected = expected
                    .with("index1", rg.index1)
                    .with("index2", rg.index2)
                    .with("amp", rg.amp)
                    .hex("tab", &bytes);
            }
        }
    }
    (stdin, expected)
}

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

#[test]
fn vp8_random_sequences_match_the_c_generator() {
    let Some(oracle) = Oracle::locate("vp8_random_sequences_match_the_c_generator") else {
        return;
    };
    let probe = oracle.probe("wp1", "random");
    let (stdin, expected) = run_rust(&script());
    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    // Name the first mismatching value when the sequences differ.
    if c_output != expected {
        let first = c_output
            .0
            .iter()
            .zip(expected.0.iter())
            .position(|(c, rust)| c != rust);
        panic!(
            "VP8Random differs from the C generator; first difference at field {first:?}: \
             C {:?}, Rust {:?}",
            first.map(|i| &c_output.0[i]),
            first.map(|i| &expected.0[i]),
        );
    }
}
