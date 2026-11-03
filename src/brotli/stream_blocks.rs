// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/decode.c:1423-1524 metadata and uncompressed-block transitions.
//! Reader/storage loans end before callbacks; callback replacement is read
//! from the same actual native fields at every original call checkpoint.
use super::bitfield::BitfieldWord;
use super::instance::{DecoderInstance, MetadataChunkFunc};
use super::layout::{BitReaderStorage, DecoderStateStorage};
use super::reader_copy::copy_reader_bytes;
use super::running::*;
use super::status::*;
use super::stream_input::SourceInput;
use std::ffi::c_void;
use std::mem::MaybeUninit;

macro_rules! read {
    ($owner:expr, $field:ident) => {{
        // SAFETY: Internal source helper caller proves each reached initialized
        // field; this single loan ends before reader/callback operations.
        unsafe { $owner.state.storage().$field.assume_init() }
    }};
}
macro_rules! write {
    ($owner:expr, $field:ident, $value:expr) => {{
        let value = $value;
        $owner.state.storage_mut().$field.write(value);
    }};
}
impl DecoderInstance<'_> {
    /// Source br=&s->br without borrowing the complete native object. This
    /// address stays tied to the same stable actual decoder allocation.
    pub(crate) fn source_reader_pointer(&self) -> *mut BitReaderStorage {
        let state = self.state.as_ptr().cast_mut().cast::<DecoderStateStorage>();
        // SAFETY: The live actual native owner contains this field. Only its
        // address is formed; no pointer slot, field value or payload is read.
        unsafe { std::ptr::addr_of_mut!((*state).br).cast() }
    }

    /// # Safety
    /// Exactly the selected source flag bits were initialized. No other
    /// native field/input loan or callback is active at this checkpoint.
    unsafe fn block_flag(&mut self, shift: u32, width: u32) -> u32 {
        let slot = &mut self.state.storage_mut().flags;
        // SAFETY: The actual native aligned4-byte word is exclusively lent as
        // MaybeUninit storage. Only initialized selected bits become a scalar.
        let bytes = unsafe {
            std::slice::from_raw_parts_mut(slot.as_mut_ptr().cast::<MaybeUninit<u8>>(), 4)
        };
        unsafe { BitfieldWord::new(bytes.try_into().unwrap()).read(shift, width) }
    }

    /// The source's actual conditional call. Arguments are constructed only
    /// when a callback is present; no source-storage reference survives it.
    ///
    /// # Safety
    /// A nonzero native callback value comes from the actual source setter,
    /// has the MetadataChunkFunc ABI, and satisfies its lifetime/no-unwind
    /// contract. The produced data extent is initialized for size bytes.
    unsafe fn metadata_chunk_call(&mut self, data: impl FnOnce(&Self) -> *const u8, size: usize) {
        if read!(self, metadata_chunk_func) != 0 {
            // SAFETY: The source callback setter stored this exact callable
            // function address/signature; no alternative owner/token is used.
            let address = read!(self, metadata_chunk_func);
            let callback: MetadataChunkFunc = unsafe { std::mem::transmute(address) };
            let opaque = read!(self, metadata_callback_opaque) as *mut c_void;
            let data = data(self);
            // SAFETY: All field/input/reader loans have ended. The callback
            // may replace these same three native callback fields through the
            // allocation-relative handle. Subsequent calls reload them.
            unsafe { callback(opaque, data, size) };
        }
    }

    /// Complete original SkipMetadataBlock.
    ///
    /// # Safety
    /// The original reached metadata/reader fields are initialized. Input is
    /// the current same native reader binding; bytes and arithmetic have their
    /// source-defined domains. Callback pointers have the setter's contract.
    #[inline(never)]
    pub unsafe fn skip_metadata_block(&mut self, input: SourceInput<'_>) -> i32 {
        if read!(self, meta_block_remaining_len) == 0 {
            return BROTLI_DECODER_SUCCESS;
        }
        let br = self.source_reader_pointer();
        debug_assert_eq!(unsafe { (*br).bit_pos.assume_init() } & 7, 0);
        if unsafe { (*br).bit_pos.assume_init() } >= 8 {
            let mut buffer = [MaybeUninit::<u8>::uninit(); 8];
            let mut nbytes = (unsafe { (*br).bit_pos.assume_init() } as i32) >> 3;
            debug_assert!(nbytes <= 8);
            if nbytes > read!(self, meta_block_remaining_len) {
                nbytes = read!(self, meta_block_remaining_len);
            }
            // SAFETY: br is the separately selected native field, not a whole
            // state reference overlapping an internal input buffer. Source
            // CopyBytes initializes only nbytes of the local actual8 bytes.
            let br = unsafe { &mut *self.source_reader_pointer() };
            unsafe {
                input.with_reader(br, |reader| {
                    copy_reader_bytes(&mut buffer, reader, input, nbytes as usize)
                });
            }
            unsafe {
                self.metadata_chunk_call(|_| buffer.as_ptr().cast::<u8>(), nbytes as usize);
            }
            write!(
                self,
                meta_block_remaining_len,
                read!(self, meta_block_remaining_len) - nbytes
            );
            if read!(self, meta_block_remaining_len) == 0 {
                return BROTLI_DECODER_SUCCESS;
            }
        }
        // SAFETY: Same initialized native field and input binding. No reader
        // or input reference is retained beyond this immediate field loan.
        let br = unsafe { &mut *self.source_reader_pointer() };
        let mut nbytes = unsafe { input.with_reader(br, |reader| reader.remaining_bytes()) } as i32;
        if nbytes > read!(self, meta_block_remaining_len) {
            nbytes = read!(self, meta_block_remaining_len);
        }
        if nbytes > 0 {
            unsafe {
                self.metadata_chunk_call(
                    |s| input.next_pointer(&*s.state.storage().br.as_ptr()),
                    nbytes as usize,
                );
            }
            let br = unsafe { &mut *self.source_reader_pointer() };
            unsafe {
                input.with_reader(br, |reader| reader.drop_bytes(nbytes as usize));
            }
            write!(
                self,
                meta_block_remaining_len,
                read!(self, meta_block_remaining_len) - nbytes
            );
            if read!(self, meta_block_remaining_len) == 0 {
                return BROTLI_DECODER_SUCCESS;
            }
        }
        let br = unsafe { &mut *self.source_reader_pointer() };
        debug_assert_eq!(
            unsafe { input.with_reader(br, |reader| reader.remaining_bytes()) },
            0
        );
        BROTLI_DECODER_NEEDS_MORE_INPUT
    }

    /// Complete original CopyUncompressedBlockToOutput.
    ///
    /// # Safety
    /// Actual uncompressed/reader/ring fields and reached output extents have
    /// their source initialization and signed arithmetic domains. Raw caller
    /// cursors obey write_ringbuffer_raw's contracts, including defined aliases.
    /// No input/field/byte/cursor reference survives allocation/free callbacks.
    #[inline(never)]
    pub unsafe fn copy_uncompressed_block_to_output_raw(
        &mut self,
        input: SourceInput<'_>,
        available_out: *mut usize,
        next_out: *mut *mut u8,
        total_out: *mut usize,
    ) -> i32 {
        if !unsafe { self.state.ensure_ringbuffer() } {
            return BROTLI_DECODER_ERROR_ALLOC_RING_BUFFER_1;
        }
        loop {
            let mut substate = read!(self, substate_uncompressed);
            if substate == BROTLI_STATE_UNCOMPRESSED_NONE {
                let br = unsafe { &mut *self.source_reader_pointer() };
                let mut nbytes =
                    unsafe { input.with_reader(br, |reader| reader.remaining_bytes()) } as i32;
                if nbytes > read!(self, meta_block_remaining_len) {
                    nbytes = read!(self, meta_block_remaining_len);
                }
                if read!(self, pos) + nbytes > read!(self, ringbuffer_size) {
                    nbytes = read!(self, ringbuffer_size) - read!(self, pos);
                }
                let pos = read!(self, pos) as usize;
                let br = unsafe { &mut *self.source_reader_pointer() };
                let ring = self
                    .state
                    .ringbuffer
                    .as_mut()
                    .expect("source actual ring owner");
                let destination = &mut ring.storage_mut()[pos..];
                unsafe {
                    input.with_reader(br, |reader| {
                        copy_reader_bytes(destination, reader, input, nbytes as usize)
                    });
                }
                write!(self, pos, read!(self, pos) + nbytes);
                write!(
                    self,
                    meta_block_remaining_len,
                    read!(self, meta_block_remaining_len) - nbytes
                );
                if read!(self, pos) < 1 << unsafe { self.block_flag(6, 6) } {
                    if read!(self, meta_block_remaining_len) == 0 {
                        return BROTLI_DECODER_SUCCESS;
                    }
                    return BROTLI_DECODER_NEEDS_MORE_INPUT;
                }
                write!(self, substate_uncompressed, BROTLI_STATE_UNCOMPRESSED_WRITE);
                substate = BROTLI_STATE_UNCOMPRESSED_WRITE;
            }
            if substate == BROTLI_STATE_UNCOMPRESSED_WRITE {
                let result =
                    unsafe { self.write_ringbuffer_raw(available_out, next_out, total_out, false) };
                if result != BROTLI_DECODER_SUCCESS {
                    return result;
                }
                if read!(self, ringbuffer_size) == 1 << unsafe { self.block_flag(6, 6) } {
                    write!(self, max_distance, read!(self, max_backward_distance));
                }
                write!(self, substate_uncompressed, BROTLI_STATE_UNCOMPRESSED_NONE);
            }
        }
    }
}
