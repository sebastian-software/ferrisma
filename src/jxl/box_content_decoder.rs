// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Complete `lib/jxl/box_content_decoder.{h,cc}` at the pinned source revision.
//! The original nullable Brotli field retains its actual pointer value, with a
//! unique retained Rust owner beside the source fields.
//! Scalar fields stay uninitialized until the source StartBox checkpoint. The
//! fixed empty dictionary binding is nonowning, allocation-free sidecar data.
use super::decode::JxlDecoderStatus;
use crate::brotli::compound_dictionary::CompoundDictionaryBindings;
use crate::brotli::instance::DecoderInstance;
use crate::brotli::status::{
    BROTLI_DECODER_RESULT_ERROR, BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT,
    BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT, BROTLI_DECODER_RESULT_SUCCESS,
};
use std::mem::MaybeUninit;

pub struct JxlBoxContentDecoder {
    brotli_dec: usize,
    header_done: MaybeUninit<bool>,
    brob_decode: MaybeUninit<bool>,
    box_until_eof: MaybeUninit<bool>,
    remaining: MaybeUninit<usize>,
    pos: MaybeUninit<usize>,
    bindings: CompoundDictionaryBindings<'static>,
    brotli_owner: Option<DecoderInstance<'static>>,
}

impl Drop for JxlBoxContentDecoder {
    fn drop(&mut self) {
        if self.brotli_dec != 0 {
            // The original destructor does not clear its pointer field. The
            // source value stays present throughout exact decoder cleanup;
            // taking only the hidden unique owner cannot reorder that field.
            drop(self.brotli_owner.take());
        }
    }
}

impl Default for JxlBoxContentDecoder {
    fn default() -> Self {
        Self::new()
    }
}

impl JxlBoxContentDecoder {
    /// The source default constructor initializes only brotli_dec=nullptr.
    /// The owner sidecar adds no source payload, allocation or pointer alias.
    pub fn new() -> Self {
        Self {
            brotli_dec: 0,
            header_done: MaybeUninit::uninit(),
            brob_decode: MaybeUninit::uninit(),
            box_until_eof: MaybeUninit::uninit(),
            remaining: MaybeUninit::uninit(),
            pos: MaybeUninit::uninit(),
            bindings: CompoundDictionaryBindings::default(),
            brotli_owner: None,
        }
    }

    pub fn start_box(&mut self, brob_decode: bool, box_until_eof: bool, contents_size: usize) {
        if self.brotli_dec != 0 {
            // Source destruction precedes clearing the pointer and every
            // scalar assignment. No dictionary/output storage is cloned.
            drop(self.brotli_owner.take());
            self.brotli_dec = 0;
        }
        self.header_done.write(false);
        self.brob_decode.write(brob_decode);
        self.box_until_eof.write(box_until_eof);
        self.remaining
            .write(if box_until_eof { 0 } else { contents_size });
        self.pos.write(0);
    }

    /// Source Process, preserving actual caller cursor loads/stores.
    ///
    /// # Safety
    /// StartBox has initialized every source scalar. Each reached input pointer
    /// addition and memcpy has its original live extent/provenance/nonoverlap
    /// domain, including non-NULL arguments for reached zero-byte memcpy calls.
    /// The output pointer/count cells are initialized, aligned, live and valid
    /// for their reached reads/writes. Defined aliases among caller cells retain
    /// source checkpoint order; no conflicting Rust loans or concurrent decoder
    /// access may overlap. Unclaimed private-state aliases are not a new API.
    /// If a compressed box reaches CreateInstance, its default state allocation
    /// succeeds before the original unconditional decoder dereference. This is
    /// the source caller domain, not an invented allocation-error status.
    pub unsafe fn process_raw(
        &mut self,
        mut next_in: *const u8,
        mut avail_in: usize,
        box_pos: usize,
        next_out: *mut *mut u8,
        avail_out: *mut usize,
    ) -> JxlDecoderStatus {
        // SAFETY: Actual StartBox initializes pos before the first source read.
        let pos = unsafe { self.pos.assume_init() };
        if box_pos > pos {
            return JxlDecoderStatus::ERROR;
        }
        let offset = pos - box_pos;
        if offset > avail_in {
            return JxlDecoderStatus::ERROR;
        }
        // SAFETY: The caller supplies the original pointer arithmetic extent;
        // the two source guards precede both pointer and count adjustments.
        next_in = unsafe { next_in.add(offset) };
        avail_in -= offset;

        // SAFETY: Same actual StartBox scalar initialization history.
        if unsafe { self.brob_decode.assume_init() } {
            if !unsafe { self.header_done.assume_init() } {
                if avail_in < 4 {
                    return JxlDecoderStatus::NEED_MORE_INPUT;
                }
                if !unsafe { self.box_until_eof.assume_init() } {
                    let remaining = unsafe { self.remaining.assume_init() };
                    if remaining < 4 {
                        return JxlDecoderStatus::ERROR;
                    }
                    self.remaining.write(remaining - 4);
                }
                // SAFETY: The reached four initialized header bytes belong
                // to this original input allocation. Their values are skipped.
                next_in = unsafe { next_in.add(4) };
                avail_in -= 4;
                self.pos
                    .write(unsafe { self.pos.assume_init() }.wrapping_add(4));
                self.header_done.write(true);
            }
            if self.brotli_dec == 0 {
                self.brotli_owner = DecoderInstance::new();
                // The source pointer assignment follows the entire constructor
                // call. Hidden owner metadata adds no source-visible write or
                // allocation; the field is the same actual returned pointer.
                self.brotli_dec = self
                    .brotli_owner
                    .as_ref()
                    .map_or(0, |d| d.as_ptr() as usize);
            }
            let mut avail_in_brotli = avail_in;
            if !unsafe { self.box_until_eof.assume_init() } {
                avail_in_brotli = avail_in_brotli.min(unsafe { self.remaining.assume_init() });
            }
            let next_in_before = next_in;
            // SAFETY: The original initialized caller cell is read at this
            // checkpoint; no reference to it survives the actual decode call.
            let next_out_before = unsafe { next_out.read() };
            // Source invokes DecompressStream unconditionally after creation.
            // A failed allocation violates that source dereference domain;
            // this assertion adds no codec error or successful fallback path.
            let decoder = self
                .brotli_owner
                .as_mut()
                .expect("source non-NULL Brotli decoder");
            // SAFETY: Original live input/output/cursor contracts forward to
            // the same retained source decoder and its fixed empty bindings.
            let result = unsafe {
                decoder.decompress_stream_raw(
                    &self.bindings,
                    &mut avail_in_brotli,
                    &mut next_in,
                    avail_out,
                    next_out,
                    std::ptr::null_mut(),
                )
            };
            // SAFETY: Original output/input pointer differences are within
            // their respective actual caller allocations. Preserve both reached
            // reads before the source error return, including mutated cursors.
            let consumed = unsafe { next_in.offset_from(next_in_before) } as usize;
            let _produced = unsafe { next_out.read().offset_from(next_out_before) } as usize;
            if result == BROTLI_DECODER_RESULT_ERROR {
                return JxlDecoderStatus::ERROR;
            }
            // The source msan unpoison marker has no ordinary Rust data read;
            // the decoder itself initializes exactly its produced prefix.
            self.pos
                .write(unsafe { self.pos.assume_init() }.wrapping_add(consumed));
            if !unsafe { self.box_until_eof.assume_init() } {
                self.remaining
                    .write(unsafe { self.remaining.assume_init() }.wrapping_sub(consumed));
                if result == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT
                    && unsafe { self.remaining.assume_init() } == 0
                {
                    return JxlDecoderStatus::ERROR;
                }
            }
            if result == BROTLI_DECODER_RESULT_NEEDS_MORE_INPUT {
                return JxlDecoderStatus::NEED_MORE_INPUT;
            }
            if result == BROTLI_DECODER_RESULT_NEEDS_MORE_OUTPUT {
                return JxlDecoderStatus::BOX_NEED_MORE_OUTPUT;
            }
            if result == BROTLI_DECODER_RESULT_SUCCESS {
                if !unsafe { self.box_until_eof.assume_init() }
                    && unsafe { self.remaining.assume_init() } != 0
                {
                    return JxlDecoderStatus::ERROR;
                }
                return JxlDecoderStatus::BOX_COMPLETE;
            }
            JxlDecoderStatus::ERROR
        } else {
            let mut can_read = avail_in;
            if !unsafe { self.box_until_eof.assume_init() } {
                can_read = can_read.min(unsafe { self.remaining.assume_init() });
            }
            // SAFETY: Exact original caller count/pointer reads; raw storage
            // retains their actual aliases and no whole output slice is formed.
            let to_write = can_read.min(unsafe { avail_out.read() });
            unsafe { std::ptr::copy_nonoverlapping(next_in, next_out.read(), to_write) };
            // SAFETY: Each source reload occurs after memcpy, not before it.
            unsafe {
                next_out.write(next_out.read().add(to_write));
                avail_out.write(avail_out.read().wrapping_sub(to_write));
            }
            if !unsafe { self.box_until_eof.assume_init() } {
                self.remaining
                    .write(unsafe { self.remaining.assume_init() }.wrapping_sub(to_write));
            }
            self.pos
                .write(unsafe { self.pos.assume_init() }.wrapping_add(to_write));
            if to_write < can_read {
                return JxlDecoderStatus::BOX_NEED_MORE_OUTPUT;
            }
            if !unsafe { self.box_until_eof.assume_init() }
                && unsafe { self.remaining.assume_init() } > 0
            {
                return JxlDecoderStatus::NEED_MORE_INPUT;
            }
            JxlDecoderStatus::BOX_COMPLETE
        }
    }

    /// Read-only private-field observer for independent original comparisons.
    ///
    /// # Safety
    /// StartBox has initialized all source scalars; no conflicting field loans
    /// or concurrent decoder operations overlap this observation.
    #[cfg(feature = "cpp-oracle")]
    pub unsafe fn observation(&self) -> BoxContentObservation {
        BoxContentObservation {
            // SAFETY: Exact caller-proven initialized source scalars; no raw
            // state padding, inactive capacity or unknown source field is read.
            header_done: unsafe { self.header_done.assume_init() },
            brob_decode: unsafe { self.brob_decode.assume_init() },
            box_until_eof: unsafe { self.box_until_eof.assume_init() },
            remaining: unsafe { self.remaining.assume_init() },
            pos: unsafe { self.pos.assume_init() },
            brotli: self.brotli_owner.as_ref().map(|d| {
                (
                    d.get_error_code(),
                    d.is_used(),
                    d.is_finished(),
                    d.has_more_output(),
                )
            }),
        }
    }
}

#[cfg(feature = "cpp-oracle")]
pub struct BoxContentObservation {
    pub header_done: bool,
    pub brob_decode: bool,
    pub box_until_eof: bool,
    pub remaining: usize,
    pub pos: usize,
    pub brotli: Option<(i32, bool, bool, bool)>,
}
