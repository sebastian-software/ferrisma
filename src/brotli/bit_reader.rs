// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/bit_reader.{h,c}, with borrowed input and source-relative byte
//! positions. Const-generic entry points retain source literal refill sites;
//! dynamic entry points retain the other BROTLI_IS_CONSTANT branch.

#[cfg(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64",
    target_arch = "riscv64",
    target_arch = "loongarch64",
    target_arch = "mips64"
))]
pub type Reg = u64;
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "powerpc64",
    target_arch = "riscv64",
    target_arch = "loongarch64",
    target_arch = "mips64"
)))]
pub type Reg = u32;

pub const FAST_INPUT_SLACK: usize = 28;
pub const SHORT_FILL_READ: usize = std::mem::size_of::<Reg>() >> 1;
const UNALIGNED_READ_FAST: bool = cfg!(any(
    target_arch = "x86",
    target_arch = "x86_64",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "loongarch64",
    all(target_arch = "arm", target_feature = "v7")
));

pub fn bit_mask(n: Reg) -> Reg {
    assert!(n <= 32);
    // The source table and expression agree on the reached [0,32] domain.
    ((1u64 << n) - 1) as Reg
}

#[derive(Clone, Copy)]
pub struct Checkpoint<'a> {
    val: Reg,
    bit_pos: Reg,
    input: &'a [u8],
    next: usize,
    avail: usize,
}

/// Initialized fields of the original source reader at an actual loan boundary.
/// Positions are relative to the SAME live initialized input anchor, not a new
/// remaining-input tail. A guard can remain behind next after source reads.
/// Source NULL/empty pointer identity stays in the adapter; this value neither
/// converts NULL to a slice pointer nor claims pointer fields initialized by Init.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SourceReaderState {
    pub val: Reg,
    pub bit_pos: Reg,
    pub next: usize,
    pub guard: usize,
    pub last: usize,
}

pub struct BitReader<'a> {
    pub val: Reg,
    pub bit_pos: Reg,
    input: &'a [u8],
    next: usize,
    guard: usize,
    last: usize,
}

impl Default for BitReader<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> BitReader<'a> {
    pub fn new() -> Self {
        // Source Init initializes only accumulator fields. The empty binding
        // gives safe storage until the required SetInput checkpoint; source
        // pointer observations start only after that checkpoint.
        Self {
            val: 0,
            bit_pos: 0,
            input: &[],
            next: 0,
            guard: 0,
            last: 0,
        }
    }

    /// Lend the already initialized original reader fields over their same
    /// caller input allocation. This is a field/borrow mapping, not SetInput:
    /// every saved guard and endpoint is preserved without recomputation.
    /// Only the supplied input slice is claimed initialized; capacity outside
    /// that slice must not be exposed merely to represent native storage.
    pub fn from_source_state(input: &'a [u8], state: SourceReaderState) -> Self {
        assert!(state.next <= state.last && state.last <= input.len());
        assert!(state.guard <= input.len());
        Self {
            val: state.val,
            bit_pos: state.bit_pos,
            input,
            next: state.next,
            guard: state.guard,
            last: state.last,
        }
    }

    /// Return initialized field values for the adapter to write back to its
    /// actual native storage before ending the input/reader loan. Input anchor
    /// identity is unchanged; no pointer is reconstructed from an empty slice.
    pub fn source_state(&self) -> SourceReaderState {
        SourceReaderState {
            val: self.val,
            bit_pos: self.bit_pos,
            next: self.next,
            guard: self.guard,
            last: self.last,
        }
    }

    pub fn available_in(&self) -> usize {
        self.last - self.next
    }
    pub fn position(&self) -> usize {
        self.next
    }
    pub fn guard_position(&self) -> usize {
        self.guard
    }
    pub fn available_bits(&self) -> Reg {
        self.bit_pos
    }
    pub fn bits_unmasked(&self) -> Reg {
        self.val
    }

    pub fn save(&self) -> Checkpoint<'a> {
        Checkpoint {
            val: self.val,
            bit_pos: self.bit_pos,
            input: self.input,
            next: self.next,
            avail: self.available_in(),
        }
    }

    pub fn set_input(&mut self, input: &'a [u8]) {
        self.set_input_range(input, 0, input.len());
    }

    fn set_input_range(&mut self, input: &'a [u8], next: usize, avail: usize) {
        assert!(next <= input.len() && avail <= input.len() - next);
        self.input = input;
        self.next = next;
        self.last = if avail == 0 { next } else { next + avail };
        self.guard = if avail + 1 > FAST_INPUT_SLACK {
            next + avail + 1 - FAST_INPUT_SLACK
        } else {
            next
        };
    }

    pub fn restore(&mut self, saved: Checkpoint<'a>) {
        self.val = saved.val;
        self.bit_pos = saved.bit_pos;
        self.set_input_range(saved.input, saved.next, saved.avail);
    }

    pub fn remaining_bytes(&self) -> usize {
        let cap = 1usize << 30;
        let avail = self.available_in();
        if avail > cap {
            cap
        } else {
            avail + (self.bit_pos >> 3) as usize
        }
    }

    pub fn check_input_amount(&self) -> bool {
        self.next < self.guard
    }

    fn load_bits(val: Reg, new_bits: Reg, offset: Reg) -> Reg {
        assert!(offset < Reg::BITS as Reg);
        val | (new_bits << offset)
    }

    fn load<const N: usize>(&self) -> [u8; N] {
        self.input[self.next..self.next + N].try_into().unwrap()
    }

    fn fill(&mut self, n: Reg, literal: bool) {
        if Reg::BITS == 64 {
            if UNALIGNED_READ_FAST && literal && n <= 8 {
                if self.bit_pos <= 8 {
                    self.val = Self::load_bits(
                        self.val,
                        u64::from_le_bytes(self.load()) as Reg,
                        self.bit_pos,
                    );
                    self.bit_pos += 56;
                    self.next += 7;
                }
            } else if UNALIGNED_READ_FAST && literal && n <= 16 {
                if self.bit_pos <= 16 {
                    self.val = Self::load_bits(
                        self.val,
                        u64::from_le_bytes(self.load()) as Reg,
                        self.bit_pos,
                    );
                    self.bit_pos += 48;
                    self.next += 6;
                }
            } else if self.bit_pos <= 32 {
                self.val = Self::load_bits(
                    self.val,
                    u32::from_le_bytes(self.load()) as Reg,
                    self.bit_pos,
                );
                self.bit_pos += 32;
                self.next += SHORT_FILL_READ;
            }
        } else if UNALIGNED_READ_FAST && literal && n <= 8 {
            if self.bit_pos <= 8 {
                self.val = Self::load_bits(
                    self.val,
                    u32::from_le_bytes(self.load()) as Reg,
                    self.bit_pos,
                );
                self.bit_pos += 24;
                self.next += 3;
            }
        } else if self.bit_pos <= 16 {
            self.val = Self::load_bits(
                self.val,
                u16::from_le_bytes(self.load()) as Reg,
                self.bit_pos,
            );
            self.bit_pos += 16;
            self.next += SHORT_FILL_READ;
        }
    }

    pub fn fill_bit_window(&mut self, n: Reg) {
        self.fill(n, false);
    }
    pub fn fill_bit_window_const<const N: u32>(&mut self) {
        self.fill(N as Reg, true);
    }
    pub fn fill_bit_window16(&mut self) {
        self.fill(17, true);
    }
    pub fn get16_bits_unmasked(&mut self) -> Reg {
        self.fill(16, true);
        self.val
    }

    pub fn pull_byte(&mut self) -> bool {
        if self.next == self.last {
            return false;
        }
        self.val = Self::load_bits(self.val, self.input[self.next] as Reg, self.bit_pos);
        self.bit_pos += 8;
        self.next += 1;
        true
    }

    pub fn warmup(&mut self) -> bool {
        let mut mask = (std::mem::size_of::<Reg>() >> 1) - 1;
        if UNALIGNED_READ_FAST {
            mask = 0;
        }
        if self.bit_pos == 0 {
            self.val = 0;
            if !self.pull_byte() {
                return false;
            }
        }
        while (self.input.as_ptr().wrapping_add(self.next) as usize & mask) != 0 {
            if !self.pull_byte() {
                return true;
            }
        }
        true
    }

    pub fn get_bits(&mut self, n: Reg) -> Reg {
        self.fill(n, false);
        self.val & bit_mask(n)
    }
    pub fn get_bits_const<const N: u32>(&mut self) -> Reg {
        self.fill(N as Reg, true);
        self.val & bit_mask(N as Reg)
    }
    pub fn safe_get_bits(&mut self, n: Reg, value: &mut Reg) -> bool {
        while self.bit_pos < n {
            if !self.pull_byte() {
                return false;
            }
        }
        *value = self.val & bit_mask(n);
        true
    }

    pub fn drop_bits(&mut self, n: Reg) {
        self.bit_pos -= n;
        self.val >>= n;
    }

    pub fn normalize(&mut self) {
        if self.bit_pos < Reg::BITS as Reg {
            self.val &= ((1 as Reg) << self.bit_pos) - 1;
        }
    }

    pub fn unload(&mut self) {
        let unused_bytes = self.bit_pos >> 3;
        let unused_bits = unused_bytes << 3;
        if unused_bytes != 0 {
            self.next -= unused_bytes as usize;
        }
        self.bit_pos -= unused_bits;
        self.normalize();
    }

    pub fn take_bits(&mut self, n: Reg) -> Reg {
        let value = self.val & bit_mask(n);
        self.drop_bits(n);
        value
    }

    pub fn read_bits24(&mut self, n: Reg) -> Reg {
        assert!(n <= 24);
        if Reg::BITS == 64 || n <= 16 {
            self.fill(n, false);
            self.take_bits(n)
        } else {
            self.fill(16, true);
            let low = self.take_bits(16);
            self.fill(8, true);
            let high = self.take_bits(n - 16);
            low | (high << 16)
        }
    }

    pub fn read_bits32(&mut self, n: Reg) -> Reg {
        assert!(n <= 32);
        if Reg::BITS == 64 || n <= 16 {
            self.fill(n, false);
            self.take_bits(n)
        } else {
            self.fill(16, true);
            let low = self.take_bits(16);
            self.fill(16, true);
            let high = self.take_bits(n - 16);
            low | (high << 16)
        }
    }

    pub fn read_bits24_const<const N: u32>(&mut self) -> Reg {
        assert!(N <= 24);
        if Reg::BITS == 64 || N <= 16 {
            self.fill(N as Reg, true);
            self.take_bits(N as Reg)
        } else {
            self.fill(16, true);
            let low = self.take_bits(16);
            self.fill(8, true);
            let high = self.take_bits((N - 16) as Reg);
            low | (high << 16)
        }
    }

    pub fn read_bits32_const<const N: u32>(&mut self) -> Reg {
        assert!(N <= 32);
        if Reg::BITS == 64 || N <= 16 {
            self.fill(N as Reg, true);
            self.take_bits(N as Reg)
        } else {
            self.fill(16, true);
            let low = self.take_bits(16);
            self.fill(16, true);
            let high = self.take_bits((N - 16) as Reg);
            low | (high << 16)
        }
    }

    pub fn safe_read_bits(&mut self, n: Reg, value: &mut Reg) -> bool {
        // The pinned NDEBUG graph omits C's <=24 DCHECK. Its unchanged
        // while/take body is defined through32 on the proved native64 matrix;
        // retain that observable domain without changing other targets.
        assert!(
            n <= if cfg!(any(target_arch = "aarch64", target_arch = "x86_64")) {
                32
            } else {
                24
            }
        );
        while self.bit_pos < n {
            if !self.pull_byte() {
                return false;
            }
        }
        *value = self.take_bits(n);
        true
    }

    pub fn safe_read_bits32(&mut self, n: Reg, value: &mut Reg) -> bool {
        assert!(n <= 32);
        if Reg::BITS == 64 || n <= 24 {
            while self.bit_pos < n {
                if !self.pull_byte() {
                    return false;
                }
            }
            *value = self.take_bits(n);
            true
        } else {
            self.safe_read_bits32_slow(n, value)
        }
    }

    pub fn safe_read_bits32_slow(&mut self, n: Reg, value: &mut Reg) -> bool {
        assert!(n > 24 && n <= 32);
        let saved = self.save();
        let mut low = 0;
        let mut high = 0;
        if !self.safe_read_bits(16, &mut low) || !self.safe_read_bits(n - 16, &mut high) {
            self.restore(saved);
            return false;
        }
        *value = low | (high << 16);
        true
    }

    pub fn jump_to_byte_boundary(&mut self) -> bool {
        let count = self.bit_pos & 7;
        let mut padding = 0;
        if count != 0 {
            padding = self.take_bits(count);
        }
        self.normalize();
        padding == 0
    }

    pub fn drop_bytes(&mut self, n: usize) {
        debug_assert_eq!(self.bit_pos, 0);
        debug_assert_eq!(self.val, 0);
        assert!(n <= self.available_in());
        self.next += n;
    }

    pub fn copy_bytes(&mut self, dest: &mut [u8], mut n: usize) {
        assert!(n <= dest.len() && n <= self.remaining_bytes());
        let mut out = 0;
        while self.bit_pos >= 8 && n > 0 {
            dest[out] = self.val as u8;
            self.drop_bits(8);
            out += 1;
            n -= 1;
        }
        self.normalize();
        if n > 0 {
            dest[out..out + n].copy_from_slice(&self.input[self.next..self.next + n]);
            self.drop_bytes(n);
        }
    }
}
