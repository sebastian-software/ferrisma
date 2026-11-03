// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Source WriteRingBuffer with actual raw caller cursors. Scalar cursor aliases
//! retain their original load/store checkpoints. No Rust reference to caller
//! cursor/output storage survives any operation or callback boundary.
use super::bitfield::BitfieldWord;
use super::instance::DecoderInstance;
use super::status::*;
use std::mem::MaybeUninit;

macro_rules! read {
    ($owner:expr,$field:ident) => {{
        // SAFETY: Internal source entry history initializes reached fields.
        unsafe { $owner.state.storage().$field.assume_init() }
    }};
}
macro_rules! write {
    ($owner:expr,$field:ident,$value:expr) => {{
        let value = $value;
        $owner.state.storage_mut().$field.write(value);
    }};
}
impl DecoderInstance<'_> {
    /// # Safety
    /// The selected source bits were initialized; no field loan or callback
    /// overlaps this exclusive four-byte raw-word loan.
    unsafe fn output_window_bits(&mut self) -> u32 {
        let slot = &mut self.state.storage_mut().flags;
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(slot.as_mut_ptr().cast::<MaybeUninit<u8>>(), 4)
        };
        unsafe { BitfieldWord::new(bytes.try_into().unwrap()).read(6, 6) }
    }
    /// Complete source WriteRingBuffer, including both next_out NULL cases.
    ///
    /// # Safety
    /// Reached native state/ring fields and cursor loads are initialized and
    /// satisfy original arithmetic/pointer/memcpy domains. available_out is a
    /// live readable/writable size_t; non-NULL next_out/total_out denote their
    /// exact writable caller cells. Non-NULL *next_out is a live source memcpy
    /// destination. Source-defined cell aliases are permitted; every raw
    /// checkpoint accesses actual caller storage, never a shadow cursor.
    #[inline(never)]
    pub unsafe fn write_ringbuffer_raw(
        &mut self,
        available_out: *mut usize,
        next_out: *mut *mut u8,
        total_out: *mut usize,
        force: bool,
    ) -> i32 {
        let token = read!(self, ringbuffer);
        let offset = read!(self, partial_pos_out) & read!(self, ringbuffer_mask) as usize;
        let owner = self
            .state
            .ringbuffer
            .as_ref()
            .expect("source live ring owner");
        assert_eq!(owner.as_ptr() as usize, token);
        assert!(offset <= owner.len());
        // SAFETY: Same actual ring owner and checked reached pointer offset.
        // This forms an address only; it reads no potentially unknown bytes.
        let start = unsafe { owner.as_ptr().add(offset) };
        let to_write = unsafe { self.state.unwritten_bytes(true) };
        // SAFETY: The caller supplies this exact initialized cursor cell.
        let mut num_written = unsafe { available_out.read() };
        if num_written > to_write {
            num_written = to_write;
        }
        if read!(self, meta_block_remaining_len) < 0 {
            return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1;
        }
        if !next_out.is_null() && unsafe { next_out.read() }.is_null() {
            // SAFETY: This actual nullable destination-pointer cell is live.
            // C assigns the ring address and does not advance it in this arm.
            unsafe {
                next_out.write(start.cast_mut());
            }
        } else if !next_out.is_null() {
            let destination = unsafe { next_out.read() };
            // SAFETY: Original caller proves live nonoverlapping memcpy
            // ranges. MaybeUninit copies object representations without eager
            // integer reads or byte references, including unknown source tails.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    start.cast::<MaybeUninit<u8>>(),
                    destination.cast::<MaybeUninit<u8>>(),
                    num_written,
                );
            }
            // Reload after memcpy: the output may have changed the pointer
            // cell itself, where its newly stored pointer/arithmetic is defined.
            let current = unsafe { next_out.read() };
            unsafe {
                next_out.write(current.add(num_written));
            }
        }
        // Read actual caller cells at these exact source checkpoints. In
        // particular total_out may alias available_out; copying them into
        // independent Rust cursor variables would change the defined result.
        unsafe {
            available_out.write(available_out.read().wrapping_sub(num_written));
        }
        write!(
            self,
            partial_pos_out,
            read!(self, partial_pos_out).wrapping_add(num_written)
        );
        if !total_out.is_null() {
            let current = read!(self, partial_pos_out);
            unsafe {
                total_out.write(current);
            }
        }
        if num_written < to_write {
            if read!(self, ringbuffer_size) == 1i32 << unsafe { self.output_window_bits() } || force
            {
                return BROTLI_DECODER_NEEDS_MORE_OUTPUT;
            } else {
                return BROTLI_DECODER_SUCCESS;
            }
        }
        if read!(self, ringbuffer_size) == 1i32 << unsafe { self.output_window_bits() }
            && read!(self, pos) >= read!(self, ringbuffer_size)
        {
            write!(self, pos, read!(self, pos) - read!(self, ringbuffer_size));
            write!(
                self,
                rb_roundtrips,
                read!(self, rb_roundtrips).wrapping_add(1)
            );
            self.state
                .write_flag(3, 1, u32::from(read!(self, pos) != 0));
        }
        BROTLI_DECODER_SUCCESS
    }
}
