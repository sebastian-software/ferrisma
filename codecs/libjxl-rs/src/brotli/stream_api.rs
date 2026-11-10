// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! Source API bindings on the same retained decoder. The raw TakeOutput
//! boundary returns its actual ring pointer without asserting initialized
//! capacity or creating a Rust reference to a potentially aliased size cell.
use super::compound_dictionary::CompoundDictionaryBindings;
use super::instance::DecoderInstance;
use super::layout::{BitReaderStorage, DecoderStateStorage};
use super::running::{BROTLI_STATE_DONE, BROTLI_STATE_UNINITED};
use super::shared_dictionary::RawDictionaryView;
use super::status::{BROTLI_DECODER_NEEDS_MORE_OUTPUT, BROTLI_DECODER_SUCCESS};
use std::ptr::NonNull;

/// Read-only source API token for the actual stable decoder block. It is not
/// an allocation owner, a Rust sidecar reference, or a lifetime extension.
#[derive(Clone, Copy)]
pub struct DecoderStateHandle {
    state: NonNull<DecoderStateStorage>,
}
impl DecoderStateHandle {
    /// # Safety
    /// The producing constructor succeeded and allocation remains live. No
    /// concurrent mutation/conflicting field loan or destruction may overlap
    /// this read. Metadata callbacks can use this after source loans end.
    pub unsafe fn get_error_code(self) -> i32 {
        // SAFETY: Source StateInit and every status return initialize this
        // exact scalar, without requiring a whole native-state reference.
        unsafe { (*self.state.as_ptr()).error_code.assume_init_read() }
    }
    /// # Safety
    /// Same live allocation/field-loan contract as get_error_code.
    pub unsafe fn is_used(self) -> bool {
        let p = self.state.as_ptr();
        // SAFETY: Constructor initializes state. Preserve the source's first
        // short circuit, reading no bit-reader input pointer on either path.
        if unsafe { (*p).state.assume_init_read() } != BROTLI_STATE_UNINITED {
            return true;
        }
        // SAFETY: Source Init initializes only the reader accumulator fields;
        // this exact selected count is initialized even before SetInput.
        unsafe {
            (*std::ptr::addr_of!((*p).br).cast::<BitReaderStorage>())
                .bit_pos
                .assume_init_read()
                != 0
        }
    }
    /// # Safety
    /// Same live allocation/field-loan contract as get_error_code. The source
    /// decoder transition history initializes the counters when ring!=NULL.
    pub unsafe fn has_more_output(self) -> bool {
        let p = self.state.as_ptr();
        // SAFETY: Same initialized scalar; source error short circuit first.
        if unsafe { self.get_error_code() } < 0 {
            return false;
        }
        // SAFETY: Source Init/EnsureRingBuffer initialize the pointer slot;
        // the value is compared only, never dereferenced as an integer token.
        if unsafe { (*p).ringbuffer.assume_init_read() } == 0 {
            return false;
        }
        // SAFETY: Same source UnwrittenBytes(false) checkpoints and exact
        // initialized scalar fields. All arithmetic is original unsigned
        // size_t, including pos's cast; no ring payload/reference is read.
        let pos = unsafe { (*p).pos.assume_init_read() } as usize;
        let partial = unsafe { (*p).rb_roundtrips.assume_init_read() }
            .wrapping_mul(unsafe { (*p).ringbuffer_size.assume_init_read() } as usize)
            .wrapping_add(pos);
        partial.wrapping_sub(unsafe { (*p).partial_pos_out.assume_init_read() }) != 0
    }
    /// # Safety
    /// Same live allocation/source transition contract as has_more_output.
    pub unsafe fn is_finished(self) -> bool {
        // SAFETY: StateInit/transitions initialize this scalar. Preserve the
        // source state short circuit before evaluating HasMoreOutput.
        unsafe {
            (*self.state.as_ptr()).state.assume_init_read() == BROTLI_STATE_DONE
                && !self.has_more_output()
        }
    }
}

impl<'a> DecoderInstance<'a> {
    /// Retain a nonowning token for source getter calls during callbacks. Each
    /// use is unsafe and cannot outlive the producing decoder allocation.
    pub fn state_handle(&self) -> DecoderStateHandle {
        DecoderStateHandle {
            state: NonNull::new(self.as_ptr().cast_mut().cast()).expect("live source state"),
        }
    }

    /// Source DecodeJPEGData/stream callers retain one decoder and its fixed
    /// borrowed dictionary bindings; no dictionary payload is cloned.
    ///
    /// # Safety
    /// The bindings belong to this live decoder. All previous attachments and
    /// the dictionary's source lifetime/arithmetic requirements hold. The
    /// immutable slice remains live and immutable for the decoder lifetime.
    pub unsafe fn attach_dictionary(
        &mut self,
        bindings: &mut CompoundDictionaryBindings<'a>,
        dictionary_type: u32,
        data: Option<&'a [u8]>,
    ) -> bool {
        // SAFETY: The caller supplies the unchanged same-state source helper
        // contract; no field or payload reference is created by this binding.
        unsafe {
            self.state
                .attach_dictionary(bindings, dictionary_type, data)
        }
    }

    /// BrotliDecoderAttachDictionary with the actual caller pointer/length.
    /// Attachment itself does not read the data, including NULL/nonzero size.
    ///
    /// # Safety
    /// The same-state bindings/arithmetic contract above holds. Every reached
    /// later dictionary operation has the actual live caller extent for `'a`,
    /// with original memcpy/pointer domains and no conflicting loans. Scalar
    /// reads require their selected bytes initialized; representation copies
    /// can retain unknown bytes without constructing an initialized slice.
    /// Caller mutation/late initialization occurs only between ended decoder
    /// loans, as in RawDictionaryView. No caller allocation is retained/copied
    /// by a second owner and no source allocation checkpoint is added.
    pub unsafe fn attach_dictionary_raw(
        &mut self,
        bindings: &mut CompoundDictionaryBindings<'a>,
        dictionary_type: u32,
        size: usize,
        data: *const u8,
    ) -> bool {
        // SAFETY: Retaining this raw value reads no data or initialized slice;
        // the caller supplies lifetime/provenance for every later reached use.
        let view = unsafe { RawDictionaryView::from_raw_parts(data, size) };
        // SAFETY: Same-state caller contract forwards to the exact existing
        // source helper, preserving shared-prefix/addon partial failure order.
        unsafe {
            self.state
                .attach_dictionary_raw(bindings, dictionary_type, view)
        }
    }

    pub fn has_more_output(&self) -> bool {
        // SAFETY: Constructor and source stream transitions initialize every
        // reached scalar field. This reads no pixel/capacity or caller data.
        unsafe { self.state.has_more_output() }
    }

    pub fn is_finished(&self) -> bool {
        // SAFETY: Same retained constructor/source-transition scalar history;
        // the original state short circuit precedes output counter accesses.
        unsafe { self.state.is_finished() }
    }

    /// Complete source decode.c:2911-2934 TakeOutput. NULL is distinct from a
    /// non-NULL zero-length ring view. The caller owns no returned allocation.
    ///
    /// # Safety
    /// `size` is its actual initialized readable/writable size_t cell. Defined
    /// aliases with source scalar fields are permitted; no reference to this
    /// caller cell is held across native state operations. Decoder/ring source
    /// history and arithmetic must be valid. A returned pointer remains tied
    /// to the actual ring allocation and source mutation/destruction lifetime;
    /// each caller byte read separately requires that byte initialized. This
    /// method never exposes an unknown byte as a Rust scalar or slice.
    pub unsafe fn take_output_raw(&mut self, size: *mut usize) -> *const u8 {
        let mut result = std::ptr::null_mut();
        // SAFETY: The caller provides this exact live initialized scalar cell;
        // preserve the original conditional's two reached loads.
        let mut available_out = if unsafe { size.read() } != 0 {
            unsafe { size.read() }
        } else {
            1usize << 24
        };
        let requested_out = available_out;
        // SAFETY: Successful constructor initializes these selected fields.
        let ring = unsafe { self.state.storage().ringbuffer.assume_init() };
        if ring == 0 || self.get_error_code() < 0 {
            // SAFETY: Only the reached size cell is written on this exit.
            unsafe { size.write(0) };
            return std::ptr::null();
        }
        // SAFETY: Source stream/ring history is the caller's contract. No
        // caller size/state/payload reference is retained during this helper.
        unsafe { self.state.wrap_ringbuffer() };
        // SAFETY: Actual source null result pointer cell selects WriteRing's
        // address-publication arm. Available count/result are disjoint locals;
        // NULL total_out retains the original no-total-store behavior.
        let status = unsafe {
            self.write_ringbuffer_raw(&mut available_out, &mut result, std::ptr::null_mut(), true)
        };
        if status == BROTLI_DECODER_SUCCESS || status == BROTLI_DECODER_NEEDS_MORE_OUTPUT {
            // SAFETY: Actual caller cell store follows every source write to
            // native output counters, preserving its possible scalar aliases.
            unsafe { size.write(requested_out.wrapping_sub(available_out)) };
        } else {
            if status < 0 {
                // SAFETY: Same reached failure checkpoint and initialized
                // input/accounting fields, adding no consumed input.
                unsafe { self.state.save_error_code(status, 0) };
            }
            // SAFETY: Exact source error writes, including no dangling slice.
            unsafe { size.write(0) };
            result = std::ptr::null_mut();
        }
        result.cast_const()
    }
}
