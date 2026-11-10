// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
//! Mechanical translation of decode.c:1895-1949.

use super::bit_reader::{BitReader, Reg};
use super::distance::safe_read_bits;
use super::symbols::{HuffmanTable, read_symbol, safe_read_symbol};
use super::tables::CMD_LUT;
use std::mem::MaybeUninit;

pub struct CommandFields<'a> {
    pub distance_code: &'a mut MaybeUninit<i32>,
    pub distance_context: &'a mut MaybeUninit<i32>,
    pub dist_htree_index: &'a mut MaybeUninit<u8>,
    pub copy_length: &'a mut MaybeUninit<i32>,
    /// Precisely source block_length[1], not an initialized whole-array view.
    pub block_length: &'a mut MaybeUninit<Reg>,
}

/// # Safety
/// table_at resolves the actual borrowed table at the source read call, after
/// save in safe mode. Its getters retain the frozen symbol contract and decode
/// only indices0..704.
/// map_at resolves only the actual initialized selected context-map byte in
/// source order. All fields, output, table/map and input are disjoint. Only
/// block_length must be initialized before success; other outputs can start
/// unknown. Fast mode needs original input slack. Source copy/insert signed
/// additions fit int, as for the pinned command LUT and its valid extra bits.
// Keep the source's local declaration and branch assignment checkpoints.
#[allow(clippy::needless_late_init)]
#[inline(always)]
pub unsafe fn read_command_internal<T: HuffmanTable>(
    safe: i32,
    fields: &mut CommandFields<'_>,
    br: &mut BitReader<'_>,
    insert_length: &mut MaybeUninit<i32>,
    table_at: impl FnOnce() -> T,
    map_at: impl FnOnce(usize) -> u8,
) -> bool {
    let cmd_code;
    let mut insert_len_extra = MaybeUninit::new(0);
    let copy_length;
    let memento;
    if safe == 0 {
        let table = table_at();
        cmd_code = read_symbol(&table, br);
        memento = None;
    } else {
        memento = Some(br.save());
        let mut local = 0;
        let table = table_at();
        if !safe_read_symbol(&table, br, &mut local) {
            return false;
        }
        cmd_code = local;
    }
    let v = CMD_LUT[cmd_code as usize];
    fields.distance_code.write(i32::from(v.distance_code));
    fields.distance_context.write(i32::from(v.context));
    // SAFETY: preceding source assignment initialized this exact context field.
    let context = unsafe { fields.distance_context.assume_init_read() };
    fields.dist_htree_index.write(map_at(context as usize));
    insert_length.write(i32::from(v.insert_len_offset));
    if safe == 0 {
        if v.insert_len_extra_bits != 0 {
            insert_len_extra.write(br.read_bits24(Reg::from(v.insert_len_extra_bits)));
        }
        copy_length = br.read_bits24(Reg::from(v.copy_len_extra_bits));
    } else {
        let mut local = MaybeUninit::uninit();
        if !safe_read_bits(
            br,
            Reg::from(v.insert_len_extra_bits),
            &mut insert_len_extra,
        ) || !safe_read_bits(br, Reg::from(v.copy_len_extra_bits), &mut local)
        {
            br.restore(memento.unwrap());
            return false;
        }
        // SAFETY: both safe suffix reads succeeded and initialized this local.
        copy_length = unsafe { local.assume_init() };
    }
    fields
        .copy_length
        .write(copy_length as i32 + i32::from(v.copy_len_offset));
    // SAFETY: successful source command entry requires initialized block count;
    // the source counter is unsigned, so decrement preserves wrapping behavior.
    let count = unsafe { fields.block_length.assume_init_read() };
    fields.block_length.write(count.wrapping_sub(1));
    // SAFETY: source wrote insert offset before the suffix reads, while the
    // local extra is initialized to0 or by a successful source suffix read.
    let offset = unsafe { insert_length.assume_init_read() };
    // SAFETY: initialized local on every successful path, including width0.
    let extra = unsafe { insert_len_extra.assume_init() };
    insert_length.write(offset + extra as i32);
    true
}

/// # Safety
/// Same source domains as read_command_internal with safe==0.
#[inline(always)]
pub unsafe fn read_command<T: HuffmanTable>(
    fields: &mut CommandFields<'_>,
    br: &mut BitReader<'_>,
    insert_length: &mut MaybeUninit<i32>,
    table_at: impl FnOnce() -> T,
    map_at: impl FnOnce(usize) -> u8,
) {
    // SAFETY: unchanged source fast-call contract is forwarded.
    unsafe { read_command_internal(0, fields, br, insert_length, table_at, map_at) };
}
/// # Safety
/// Same source domains as read_command_internal with safe!=0.
#[inline(always)]
pub unsafe fn safe_read_command<T: HuffmanTable>(
    fields: &mut CommandFields<'_>,
    br: &mut BitReader<'_>,
    insert_length: &mut MaybeUninit<i32>,
    table_at: impl FnOnce() -> T,
    map_at: impl FnOnce(usize) -> u8,
) -> bool {
    // SAFETY: unchanged source safe-call contract is forwarded.
    unsafe { read_command_internal(1, fields, br, insert_length, table_at, map_at) }
}

#[inline(always)]
pub fn check_input_amount(safe: i32, br: &BitReader<'_>) -> bool {
    if safe != 0 {
        return true;
    }
    br.check_input_amount()
}
