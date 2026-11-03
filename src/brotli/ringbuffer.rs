// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/decode.c ring allocation, sizing, output and output-observation APIs.
//! Every field loan ends before allocation/free callbacks. Actual allocation
//! owners retain source pointer identity; copying uses MaybeUninit bytes so
//! source-unwritten ring capacity is never read as initialized Rust integers.

use super::bitfield::BitfieldWord;
use super::layout::DecoderStateStorage;
use super::running::BROTLI_STATE_DONE;
use super::state::StateOwner;
use super::status::{
    BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1, BROTLI_DECODER_NEEDS_MORE_OUTPUT,
    BROTLI_DECODER_SUCCESS,
};
use std::mem::{MaybeUninit, offset_of};
use std::ops::Range;

const RING_BUFFER_WRITE_AHEAD_SLACK: usize = 542;

macro_rules! read {
    ($owner:expr, $field:ident) => {{
        // SAFETY: Each enclosing internal helper requires exactly the source
        // initialization checkpoints for its reached field reads. A single
        // field borrow ends here, before any external callback can execute.
        unsafe { $owner.storage().$field.assume_init() }
    }};
}
macro_rules! write {
    ($owner:expr, $field:ident, $value:expr) => {{
        let value = $value;
        $owner.storage_mut().$field.write(value);
    }};
}

/// The source's nullable next_out and nullable *next_out cases. Reference
/// records an owner-relative address without borrowing or reading the bytes.
/// Copy writes only the reached prefix of potentially uninitialized output.
pub enum OutputTarget<'a> {
    Discard,
    Reference(&'a mut Option<Range<usize>>),
    Copy {
        destination: &'a mut [MaybeUninit<u8>],
        position: &'a mut usize,
    },
}

impl StateOwner<'_> {
    /// SaveErrorCode and its initialized-input-pointer short circuit.
    ///
    /// # Safety
    /// used_input/buffer_length have their source initialization. Nonzero
    /// buffer_length additionally requires initialized bit-reader input slots.
    pub unsafe fn save_error_code(&mut self, error: i32, consumed_input: usize) -> u32 {
        write!(self, error_code, error);
        write!(
            self,
            used_input,
            read!(self, used_input).wrapping_add(consumed_input as u64)
        );
        if read!(self, buffer_length) != 0 {
            // SAFETY: The caller proves the source's initialized br storage
            // and input fields only on this nonempty-internal-buffer path.
            let depleted = unsafe {
                let br = &*self.storage().br.as_ptr();
                br.next_in.assume_init() == br.last_in.assume_init()
            };
            if depleted {
                write!(self, buffer_length, 0);
            }
        }
        match error {
            BROTLI_DECODER_SUCCESS => 1,
            super::status::BROTLI_DECODER_NEEDS_MORE_INPUT => 2,
            BROTLI_DECODER_NEEDS_MORE_OUTPUT => 3,
            _ => 0,
        }
    }

    unsafe fn ring_flag(&mut self, shift: u32, width: u32) -> u32 {
        let flags = &mut self.storage_mut().flags;
        // SAFETY: The live native flag slot has checked state extent and
        // alignment. This exclusive loan forms no initialized whole-word
        // value; only the caller-proven initialized source field is read.
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(flags.as_mut_ptr().cast::<MaybeUninit<u8>>(), 4)
        };
        debug_assert_eq!(offset_of!(DecoderStateStorage, flags) % 4, 0);
        // SAFETY: The caller establishes initialization of these bits only.
        unsafe { BitfieldWord::new(bytes.try_into().expect("native flag word")).read(shift, width) }
    }

    /// UnwrittenBytes, including the original unsigned arithmetic.
    ///
    /// # Safety
    /// The reached pos, ringbuffer_size, rb_roundtrips and partial_pos_out
    /// fields must have their source initialization. This helper reads no ring
    /// payload and does not require the entire allocation to be initialized.
    pub unsafe fn unwritten_bytes(&self, wrap: bool) -> usize {
        let pos = if wrap && read!(self, pos) > read!(self, ringbuffer_size) {
            read!(self, ringbuffer_size) as usize
        } else {
            read!(self, pos) as usize
        };
        let partial_pos_rb = read!(self, rb_roundtrips)
            .wrapping_mul(read!(self, ringbuffer_size) as usize)
            .wrapping_add(pos);
        partial_pos_rb.wrapping_sub(read!(self, partial_pos_out))
    }

    /// BrotliDecoderHasMoreOutput with the original error/null short circuits.
    ///
    /// # Safety
    /// Fields reached after the error and pointer checks must be initialized
    /// as in the original decoder. A negative error needs only error_code.
    pub unsafe fn has_more_output(&self) -> bool {
        if read!(self, error_code) < 0 {
            return false;
        }
        // SAFETY: The same source checks dominate UnwrittenBytes.
        read!(self, ringbuffer) != 0 && unsafe { self.unwritten_bytes(false) } != 0
    }

    /// BrotliDecoderIsFinished, retaining its DONE short circuit.
    ///
    /// # Safety
    /// State and the fields reached by HasMoreOutput on DONE must have their
    /// source initialization. Non-DONE reads no other source field.
    pub unsafe fn is_finished(&self) -> bool {
        // SAFETY: The source's same DONE condition dominates the second call.
        read!(self, state) == BROTLI_STATE_DONE && !unsafe { self.has_more_output() }
    }

    /// BrotliCalculateRingBufferSize.
    ///
    /// # Safety
    /// The source has decoded metablock size/flags. Every reached integer
    /// operation and shift must be in its defined source domain (in particular
    /// window_bits<31 and signed output_size addition without overflow).
    pub unsafe fn calculate_ringbuffer_size(&mut self) {
        // SAFETY: The decoded window field is initialized on this source path.
        let window_size = 1i32 << unsafe { self.ring_flag(6, 6) };
        let mut new_ringbuffer_size = window_size;
        let mut min_size = if read!(self, ringbuffer_size) != 0 {
            read!(self, ringbuffer_size)
        } else {
            1024
        };
        if read!(self, ringbuffer_size) == window_size {
            return;
        }
        // SAFETY: StateInit and the decoded header initialize metadata.
        if unsafe { self.ring_flag(2, 1) } != 0 {
            return;
        }
        let mut output_size = if read!(self, ringbuffer) == 0 {
            0
        } else {
            read!(self, pos)
        };
        output_size += read!(self, meta_block_remaining_len);
        min_size = if min_size < output_size {
            output_size
        } else {
            min_size
        };
        // SAFETY: StateInit/SetParameter initialize canny allocation.
        if unsafe { self.ring_flag(4, 1) } != 0 {
            while (new_ringbuffer_size >> 1) >= min_size {
                new_ringbuffer_size >>= 1;
            }
        }
        write!(self, new_ringbuffer_size, new_ringbuffer_size);
    }

    /// BrotliEnsureRingBuffer, on the actual callback-owned allocation.
    ///
    /// # Safety
    /// If sizes differ, CalculateRingBufferSize selected the source size>=2. Existing ring
    /// pointer/owner identity and the reached old-copy extent must be valid;
    /// source pos bytes fit both allocations. Copying unknown representation
    /// bytes remains permitted and does not claim they are initialized output.
    pub unsafe fn ensure_ringbuffer(&mut self) -> bool {
        let old_ringbuffer = read!(self, ringbuffer);
        if read!(self, ringbuffer_size) == read!(self, new_ringbuffer_size) {
            return true;
        }
        let size =
            (read!(self, new_ringbuffer_size) as usize).wrapping_add(RING_BUFFER_WRITE_AHEAD_SLACK);
        // No source-storage loan survives this actual callback. The old
        // source pointer is still present while its allocation RHS executes.
        let allocation = self.allocator.allocate(size);
        let address = allocation
            .as_ref()
            .map_or(0, |owner| owner.as_ptr() as usize);
        write!(self, ringbuffer, address);
        let Some(mut allocation) = allocation else {
            write!(self, ringbuffer, old_ringbuffer);
            return false;
        };
        let new_size = read!(self, new_ringbuffer_size) as usize;
        allocation.storage_mut()[new_size - 2].write(0);
        allocation.storage_mut()[new_size - 1].write(0);
        if old_ringbuffer != 0 {
            let count = read!(self, pos) as usize;
            let old = self
                .ringbuffer
                .take()
                .expect("source live old ring allocation");
            assert_eq!(old.as_ptr() as usize, old_ringbuffer);
            assert!(count <= old.len() && count <= allocation.len());
            // SAFETY: Distinct live callback allocations own checked extents.
            // Copy MaybeUninit representations, never an unknown u8 value.
            // Every allocation/field loan ends before the original free call.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    old.as_ptr().cast::<MaybeUninit<u8>>(),
                    allocation.storage_mut().as_mut_ptr(),
                    count,
                )
            };
            self.allocator.release(Some(old));
        } else {
            assert!(self.ringbuffer.is_none());
        }
        self.ringbuffer = Some(allocation);
        write!(self, ringbuffer_size, read!(self, new_ringbuffer_size));
        write!(self, ringbuffer_mask, read!(self, new_ringbuffer_size) - 1);
        write!(
            self,
            ringbuffer_end,
            read!(self, ringbuffer).wrapping_add(read!(self, ringbuffer_size) as usize)
        );
        true
    }

    /// WrapRingBuffer; no payload/position read when the flag is clear.
    ///
    /// # Safety
    /// The selected flag is initialized. If set, source ring/end/pos describe
    /// two valid nonoverlapping extents in this same allocation, and the actual
    /// owner is live. Unknown representation bytes may be copied as in memcpy.
    pub unsafe fn wrap_ringbuffer(&mut self) {
        // SAFETY: Source initializes this field before the reached call.
        if unsafe { self.ring_flag(3, 1) } != 0 {
            let count = read!(self, pos) as usize;
            let end = read!(self, ringbuffer_end);
            let base = read!(self, ringbuffer);
            let owner = self
                .ringbuffer
                .as_mut()
                .expect("source live ring allocation");
            assert_eq!(owner.as_ptr() as usize, base);
            let offset = end.checked_sub(base).expect("source interior ring end");
            assert!(count <= offset && offset <= owner.len() && count <= owner.len() - offset);
            let storage = owner.storage_mut();
            // SAFETY: Checked same-owner extents are disjoint. MaybeUninit
            // preserves the source memcpy representation without scalar reads.
            unsafe {
                std::ptr::copy_nonoverlapping(
                    storage.as_ptr().add(offset),
                    storage.as_mut_ptr(),
                    count,
                )
            };
            self.write_flag(3, 1, 0);
        }
    }

    /// WriteRingBuffer. OutputTarget distinguishes both source null cases.
    ///
    /// # Safety
    /// Every reached source field and pointer arithmetic has its initialized,
    /// defined source domain. Copy extents fit the actual ring allocation and
    /// output; this does not assert other ring capacity is initialized. Output
    /// cannot alias the ring. A returned reference range is an address, not
    /// yet a claim of initialized bytes.
    pub unsafe fn write_ringbuffer(
        &mut self,
        available_out: &mut usize,
        target: OutputTarget<'_>,
        total_out: Option<&mut usize>,
        force: bool,
    ) -> i32 {
        let ringbuffer = read!(self, ringbuffer);
        let start = read!(self, partial_pos_out) & read!(self, ringbuffer_mask) as usize;
        // SAFETY: Same reached scalar source initialization.
        let to_write = unsafe { self.unwritten_bytes(true) };
        let mut num_written = *available_out;
        if num_written > to_write {
            num_written = to_write;
        }
        if read!(self, meta_block_remaining_len) < 0 {
            return BROTLI_DECODER_ERROR_FORMAT_BLOCK_LENGTH_1;
        }
        match target {
            OutputTarget::Reference(range) => {
                *range = Some(start..start + num_written);
            }
            OutputTarget::Copy {
                destination,
                position,
            } => {
                let owner = self
                    .ringbuffer
                    .as_ref()
                    .expect("source live ring allocation");
                assert_eq!(owner.as_ptr() as usize, ringbuffer);
                assert!(start <= owner.len() && num_written <= owner.len() - start);
                assert!(
                    *position <= destination.len() && num_written <= destination.len() - *position
                );
                // SAFETY: Real distinct input/output extents checked above.
                // No uninitialized ring byte becomes an initialized scalar.
                unsafe {
                    std::ptr::copy_nonoverlapping(
                        owner.as_ptr().add(start).cast::<MaybeUninit<u8>>(),
                        destination.as_mut_ptr().add(*position),
                        num_written,
                    )
                };
                *position += num_written;
            }
            OutputTarget::Discard => {}
        }
        *available_out -= num_written;
        write!(
            self,
            partial_pos_out,
            read!(self, partial_pos_out).wrapping_add(num_written)
        );
        if let Some(total_out) = total_out {
            *total_out = read!(self, partial_pos_out);
        }
        if num_written < to_write {
            // SAFETY: Decoded initialized window bits, defined signed shift.
            if read!(self, ringbuffer_size) == 1i32 << unsafe { self.ring_flag(6, 6) } || force {
                return BROTLI_DECODER_NEEDS_MORE_OUTPUT;
            } else {
                return BROTLI_DECODER_SUCCESS;
            }
        }
        // SAFETY: The same decoded window field is initialized.
        if read!(self, ringbuffer_size) == 1i32 << unsafe { self.ring_flag(6, 6) }
            && read!(self, pos) >= read!(self, ringbuffer_size)
        {
            write!(self, pos, read!(self, pos) - read!(self, ringbuffer_size));
            write!(
                self,
                rb_roundtrips,
                read!(self, rb_roundtrips).wrapping_add(1)
            );
            self.write_flag(3, 1, u32::from(read!(self, pos) != 0));
        }
        BROTLI_DECODER_SUCCESS
    }

    /// BrotliDecoderTakeOutput. The slice borrows the original live ring
    /// allocation, preventing a later decoder mutation until it is released.
    ///
    /// # Safety
    /// Source field/extent requirements of WrapRingBuffer/WriteRingBuffer
    /// must hold. Every byte of the returned range must have been initialized
    /// by reached decoder writes (or the allocator); capacity alone does not
    /// establish initialization. Error-only early exits need only their
    /// reached scalar fields, retaining the original short circuit.
    pub unsafe fn take_output(&mut self, size: &mut usize) -> Option<&[u8]> {
        let mut available_out = if *size != 0 { *size } else { 1usize << 24 };
        let requested_out = available_out;
        if read!(self, ringbuffer) == 0 || read!(self, error_code) < 0 {
            *size = 0;
            return None;
        }
        // SAFETY: The caller establishes the original ring checkpoints.
        unsafe { self.wrap_ringbuffer() };
        let mut range = None;
        // SAFETY: This is the source's null *next_out case: it publishes an
        // address, performs no memcpy, and advances the same output fields.
        let status = unsafe {
            self.write_ringbuffer(
                &mut available_out,
                OutputTarget::Reference(&mut range),
                None,
                true,
            )
        };
        if status == BROTLI_DECODER_SUCCESS || status == BROTLI_DECODER_NEEDS_MORE_OUTPUT {
            *size = requested_out - available_out;
        } else {
            if status < 0 {
                // SAFETY: The same source failure path has initialized its
                // status/accounting fields; no extra consumed input is added.
                unsafe { self.save_error_code(status, 0) };
            }
            *size = 0;
            range = None;
        }
        range.map(|range| {
            let owner = self
                .ringbuffer
                .as_ref()
                .expect("source live ring allocation");
            // SAFETY: The caller establishes the reached initialized output
            // prefix. The actual owner checks extent and retains its lifetime.
            unsafe { owner.initialized_bytes(range) }
        })
    }
}
