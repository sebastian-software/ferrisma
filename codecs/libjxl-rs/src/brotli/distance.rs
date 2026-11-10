// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.
//! Mechanical translation of decode.c:1698-1743 and 1812-1893.

use super::bit_reader::{BitReader, Reg};
use super::layout::BodyArenaStorage;
use super::symbols::{HuffmanTable, read_symbol, safe_read_symbol};
use std::mem::MaybeUninit;

/// Disjoint fields of the actual source distance history. Only the reached
/// history slot must be initialized; no whole-array typed borrow is formed.
pub struct DistanceRingFields<'a> {
    pub distance_code: &'a mut MaybeUninit<i32>,
    pub distance_context: &'a mut MaybeUninit<i32>,
    pub dist_rb_idx: &'a mut MaybeUninit<i32>,
    pub dist_rb: &'a MaybeUninit<[i32; 4]>,
}

/// # Safety
/// Source code is 0..18, index arithmetic and selected history+delta fit int,
/// and the reached ring slot is initialized. Code0..3 initializes context;
/// code4..15 preserves it without requiring its previous value initialized.
#[inline(always)]
pub unsafe fn take_distance_from_ring_buffer(fields: &mut DistanceRingFields<'_>) {
    // SAFETY: source caller initializes the short distance code before entry.
    let offset = unsafe { fields.distance_code.assume_init_read() } - 3;
    // SAFETY: same initialized source code, at its original condition load.
    if unsafe { fields.distance_code.assume_init_read() } <= 3 {
        // SAFETY: original shift reads the still-initialized short code.
        fields
            .distance_context
            .write(1 >> unsafe { fields.distance_code.assume_init_read() });
        // SAFETY: source StateInit/history initialized the index; this read
        // occurs after the original context write, not eagerly at entry.
        let rb_index = unsafe { fields.dist_rb_idx.assume_init_read() };
        // SAFETY: source signed arithmetic fits and mask selects the caller's
        // initialized actual history slot. No neighboring slots are read.
        let distance = unsafe {
            fields
                .dist_rb
                .as_ptr()
                .cast::<i32>()
                .add(((rb_index - offset) & 3) as usize)
                .read()
        };
        fields.distance_code.write(distance);
        // SAFETY: the preceding source write initialized context on this path.
        let rb_index = unsafe { fields.dist_rb_idx.assume_init_read() };
        // SAFETY: context was written before the actual distance assignment.
        let context = unsafe { fields.distance_context.assume_init_read() };
        fields.dist_rb_idx.write(rb_index - context);
    } else {
        let mut index_delta = 3;
        // SAFETY: these source code loads precede the code assignment below.
        let mut base = unsafe { fields.distance_code.assume_init_read() } - 10;
        // SAFETY: initialized code at the original second branch condition.
        if unsafe { fields.distance_code.assume_init_read() } < 10 {
            // SAFETY: this source expression still reaches the original code.
            base = unsafe { fields.distance_code.assume_init_read() } - 4;
        } else {
            index_delta = 2;
        }
        let delta = ((0x605142 >> (4 * base)) & 0xf) - 3;
        // SAFETY: source StateInit/history initialized the index at this load.
        let rb_index = unsafe { fields.dist_rb_idx.assume_init_read() };
        // SAFETY: exact masked source slot, initialized by the caller's history.
        let previous = unsafe {
            fields
                .dist_rb
                .as_ptr()
                .cast::<i32>()
                .add(((rb_index + index_delta) & 3) as usize)
                .read()
        };
        fields.distance_code.write(previous + delta);
        // SAFETY: the immediately preceding source write initialized code.
        if unsafe { fields.distance_code.assume_init_read() } <= 0 {
            fields.distance_code.write(0x7fffffff);
        }
    }
}

/// Source SafeReadBits, including its no-reader-access zero-width branch.
/// Failure never reads or writes the potentially unknown output.
#[inline(always)]
pub fn safe_read_bits(br: &mut BitReader<'_>, n_bits: Reg, val: &mut MaybeUninit<Reg>) -> bool {
    if n_bits != 0 {
        let mut local = 0;
        if !br.safe_read_bits(n_bits, &mut local) {
            return false;
        }
        val.write(local);
        true
    } else {
        val.write(0);
        true
    }
}

/// Source SafeReadBits32; the underlying reader retains its native32/64 branch.
#[inline(always)]
pub fn safe_read_bits32(br: &mut BitReader<'_>, n_bits: Reg, val: &mut MaybeUninit<Reg>) -> bool {
    if n_bits != 0 {
        let mut local = 0;
        if !br.safe_read_bits32(n_bits, &mut local) {
            return false;
        }
        val.write(local);
        true
    } else {
        val.write(0);
        true
    }
}

/// # Safety
/// The three scalar inputs are initialized actual source fields. Source shift
/// counts are below Reg::BITS, and all reached direct/complete-group stores
/// fit the actual544-entry arrays. Only source-written slots become initialized;
/// short slots0..16 and the unreached suffix remain untouched/possibly unknown.
pub unsafe fn calculate_distance_lut(
    body: &mut BodyArenaStorage,
    distance_postfix_bits: &MaybeUninit<Reg>,
    num_direct_distance_codes: &MaybeUninit<Reg>,
    alphabet_size_limit: &MaybeUninit<u16>,
) {
    // SAFETY: source header initialized these fields before the body checkpoint.
    let npostfix = unsafe { distance_postfix_bits.assume_init_read() };
    // SAFETY: same source checkpoint initializes direct count and group limit.
    let ndirect = unsafe { num_direct_distance_codes.assume_init_read() };
    // SAFETY: source Huffman group initialization wrote this limit.
    let alphabet_size_limit = Reg::from(unsafe { alphabet_size_limit.assume_init_read() });
    let postfix: Reg = 1 << npostfix;
    let mut bits: Reg = 1;
    let mut half: Reg = 0;
    let mut i: Reg = 16;
    let mut j = 0;
    while j < ndirect {
        // SAFETY: source caller's backing domain includes every direct slot;
        // complete field writes initialize only the reached actual array entries.
        unsafe {
            body.dist_extra_bits
                .as_mut_ptr()
                .cast::<u8>()
                .add(i as usize)
                .write(0);
            body.dist_offset
                .as_mut_ptr()
                .cast::<Reg>()
                .add(i as usize)
                .write(j + 1);
        }
        i += 1;
        j += 1;
    }
    while i < alphabet_size_limit {
        let base = ndirect
            .wrapping_add(((2 + half) << bits).wrapping_sub(4) << npostfix)
            .wrapping_add(1);
        j = 0;
        while j < postfix {
            // SAFETY: original always fills a complete group, including slots
            // beyond the logical limit, within the caller-proved544 backing.
            unsafe {
                body.dist_extra_bits
                    .as_mut_ptr()
                    .cast::<u8>()
                    .add(i as usize)
                    .write(bits as u8);
                body.dist_offset
                    .as_mut_ptr()
                    .cast::<Reg>()
                    .add(i as usize)
                    .write(base.wrapping_add(j));
            }
            i += 1;
            j += 1;
        }
        bits += half;
        half ^= 1;
    }
}

/// Actual field loans; block_length is precisely source block_length[2].
pub struct DistanceFields<'a> {
    pub ring: DistanceRingFields<'a>,
    pub block_length: &'a mut MaybeUninit<Reg>,
    pub distance_postfix_bits: &'a MaybeUninit<Reg>,
    pub dist_htree_index: &'a MaybeUninit<u8>,
    pub body: &'a BodyArenaStorage,
}

/// # Safety
/// Source entry has distance_code<0 and initialized tree index. Every reached
/// block-length read is initialized; symbol shortage does not read that field.
/// tree_at resolves the exact actual tree owner once, before save/read. Its
/// separate bits/value getters follow the frozen symbol helper's reached-read
/// contract. Short codes require the ring helper's domain; regular codes need
/// their initialized LUT slots, source-valid extra width0..32 and postfix shift,
/// and actual fast input slack if safe==0. Other field loans/input are disjoint.
// Keep the source's local declaration and branch assignment checkpoints.
#[allow(clippy::needless_late_init)]
#[inline(always)]
pub unsafe fn read_distance_internal<T: HuffmanTable>(
    safe: i32,
    fields: &mut DistanceFields<'_>,
    br: &mut BitReader<'_>,
    tree_at: impl FnOnce(u8) -> T,
) -> bool {
    // SAFETY: actual source group selection initializes this byte before entry.
    let index = unsafe { fields.dist_htree_index.assume_init_read() };
    let distance_tree = tree_at(index);
    let code;
    let memento;
    if safe == 0 {
        code = read_symbol(&distance_tree, br);
        memento = None;
    } else {
        memento = Some(br.save());
        let mut local = 0;
        if !safe_read_symbol(&distance_tree, br, &mut local) {
            return false;
        }
        code = local;
    }
    // SAFETY: source entry initialized this counter. Unsigned decrement wraps.
    let length = unsafe { fields.block_length.assume_init_read() };
    fields.block_length.write(length.wrapping_sub(1));
    fields.ring.distance_context.write(0);
    if code & !0xf == 0 {
        fields.ring.distance_code.write(code as i32);
        // SAFETY: exact short-code path has just initialized code/context;
        // source history/index preconditions are part of this entry contract.
        unsafe { take_distance_from_ring_buffer(&mut fields.ring) };
        return true;
    }
    let bits;
    if safe == 0 {
        // SAFETY: source LUT construction initialized the exact regular slot;
        // this field is read at the original fast-call argument checkpoint.
        let nbits = Reg::from(unsafe {
            fields
                .body
                .dist_extra_bits
                .as_ptr()
                .cast::<u8>()
                .add(code as usize)
                .read()
        });
        bits = br.read_bits32(nbits);
    } else {
        let mut local = MaybeUninit::uninit();
        // SAFETY: same initialized slot at the original safe-call checkpoint.
        let nbits = Reg::from(unsafe {
            fields
                .body
                .dist_extra_bits
                .as_ptr()
                .cast::<u8>()
                .add(code as usize)
                .read()
        });
        if !safe_read_bits32(br, nbits, &mut local) {
            // SAFETY: the earlier unsigned decrement initialized this field.
            let length = unsafe { fields.block_length.assume_init_read() };
            fields.block_length.write(length.wrapping_add(1));
            br.restore(memento.unwrap());
            return false;
        }
        // SAFETY: successful safe suffix read initialized this local only.
        bits = unsafe { local.assume_init() };
    }
    // SAFETY: only suffix success reaches offset/postfix reads. The caller
    // proves initialization of exactly this offset slot and postfix field.
    let offset = unsafe {
        fields
            .body
            .dist_offset
            .as_ptr()
            .cast::<Reg>()
            .add(code as usize)
            .read()
    };
    // SAFETY: source header checkpoint initialized this shift count.
    let postfix = unsafe { fields.distance_postfix_bits.assume_init_read() };
    fields
        .ring
        .distance_code
        .write(offset.wrapping_add(bits << postfix) as i32);
    true
}

/// # Safety
/// Same source domains as read_distance_internal with safe==0.
#[inline(always)]
pub unsafe fn read_distance<T: HuffmanTable>(
    fields: &mut DistanceFields<'_>,
    br: &mut BitReader<'_>,
    tree_at: impl FnOnce(u8) -> T,
) {
    // SAFETY: forwarded unchanged source fast-call contract.
    unsafe { read_distance_internal(0, fields, br, tree_at) };
}
/// # Safety
/// Same source domains as read_distance_internal with safe!=0.
#[inline(always)]
pub unsafe fn safe_read_distance<T: HuffmanTable>(
    fields: &mut DistanceFields<'_>,
    br: &mut BitReader<'_>,
    tree_at: impl FnOnce(u8) -> T,
) -> bool {
    // SAFETY: forwarded unchanged source safe-call contract.
    unsafe { read_distance_internal(1, fields, br, tree_at) }
}
