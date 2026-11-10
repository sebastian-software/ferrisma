// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/decode.c lifecycle, configuration and scalar observation APIs.
//! The actual decoder state allocation is stable; this inline owner only
//! retains its producing allocator and the lifetimes of borrowed dictionaries.
//! This component covers lifecycle/configuration. Streaming and output porting
//! remains a separate boundary on the same state.

use super::default_memory;
use super::layout::DecoderStateStorage;
use super::memory::{AllocFunc, Allocator, FreeFunc};
use super::running::BROTLI_STATE_UNINITED;
use super::state::StateOwner;
use std::ffi::c_void;
use std::mem::{MaybeUninit, size_of};
use std::ptr::{NonNull, addr_of_mut};

pub const BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION: u32 = 0;
pub const BROTLI_DECODER_PARAM_LARGE_WINDOW: u32 = 1;

pub type MetadataStartFunc = unsafe extern "C" fn(*mut c_void, usize);
pub type MetadataChunkFunc = unsafe extern "C" fn(*mut c_void, *const u8, usize);

/// A nonowning token for the actual stable source allocation, not the movable
/// Rust ownership sidecar. It grants access only to the three callback fields.
/// Copying it neither retains an allocation nor extends the decoder lifetime.
#[derive(Clone, Copy)]
pub struct MetadataCallbacksHandle {
    state: NonNull<DecoderStateStorage>,
}

impl MetadataCallbacksHandle {
    /// BrotliDecoderSetMetadataCallbacks at the actual source field addresses.
    /// This can be called from a metadata callback when the decoder has ended
    /// every source-storage reference before invoking the external function.
    /// Later source checkpoints must reload these fields, retaining no snapshot.
    ///
    /// # Safety
    /// The producing decoder allocation must still be live. There may be no
    /// live reference to these fields, no concurrent decoder access, and no
    /// destructor in progress. The new callbacks and opaque must remain valid
    /// until replaced or destruction. Callbacks cannot unwind; chunk contents
    /// are initialized only for their advertised size and lifetime.
    pub unsafe fn set(
        self,
        start: Option<MetadataStartFunc>,
        chunk: Option<MetadataChunkFunc>,
        opaque: *mut c_void,
    ) {
        let state = self.state.as_ptr();
        // SAFETY: The caller establishes the live source allocation and the
        // absence of conflicting field references. addr_of_mut! projects each
        // actual field without constructing a reference to the complete state
        // or reading any other partially initialized member. Exactly these
        // three source assignments execute in their declaration's API order.
        unsafe {
            addr_of_mut!((*state).metadata_start_func).write(MaybeUninit::new(
                start.map_or(0, |function| function as usize),
            ));
            addr_of_mut!((*state).metadata_chunk_func).write(MaybeUninit::new(
                chunk.map_or(0, |function| function as usize),
            ));
            addr_of_mut!((*state).metadata_callback_opaque)
                .write(MaybeUninit::new(opaque as usize));
        }
    }
}

pub struct DecoderInstance<'a> {
    pub(crate) state: StateOwner<'a>,
}

impl<'a> DecoderInstance<'a> {
    /// BrotliDecoderCreateInstance with the source default callback pair.
    pub fn new() -> Option<Self> {
        // SAFETY: An absent pair selects the immutable default functions;
        // null opaque has no external lifetime or callback obligations.
        unsafe { Self::create(None, None, std::ptr::null_mut()) }
    }

    /// BrotliDecoderCreateInstance. Mixed callback pairs fail before calling
    /// either function. No source payload is allocated on the Rust heap apart
    /// from the actual blocks returned by the selected allocator.
    ///
    /// # Safety
    /// A custom pair and opaque must satisfy Allocator::from_callbacks for
    /// this instance's complete lifetime. In particular callbacks cannot
    /// unwind, reenter this decoder or access blocks during Rust borrows.
    pub unsafe fn create(
        allocate: Option<AllocFunc>,
        free: Option<FreeFunc>,
        opaque: *mut c_void,
    ) -> Option<Self> {
        // The pinned default BROTLI_STATIC_INIT_NONE branch of
        // BrotliDecoderEnsureStaticInit returns true. The exact command
        // lookup data is initialized statically, rather than at runtime.
        let (allocator, shared_allocator, allocate, free, state_opaque) = match (allocate, free) {
            (None, None) => {
                let (allocate, free) = default_memory::callbacks();
                (
                    default_memory::allocator(),
                    default_memory::allocator_with_opaque(opaque),
                    allocate,
                    free,
                    std::ptr::null_mut(),
                )
            }
            (Some(allocate), Some(free)) => {
                // SAFETY: This caller supplies the complete source pair
                // and establishes its allocator/lifetime contract.
                let allocator =
                    unsafe { Allocator::from_callbacks(Some(allocate), Some(free), opaque) }?;
                (allocator, allocator, allocate, free, opaque)
            }
            _ => return None,
        };
        let allocation = allocator.allocate(size_of::<DecoderStateStorage>())?;
        let state = StateOwner::initialize(
            allocation,
            allocator,
            shared_allocator,
            allocate as usize,
            free as usize,
            state_opaque as usize,
            opaque as usize,
        )?;
        Some(Self { state })
    }

    /// BrotliDecoderSetParameter: only an unstarted source state accepts it.
    pub fn set_parameter(&mut self, parameter: u32, value: u32) -> bool {
        // SAFETY: Successful StateInit initializes state, and every source
        // streaming transition writes it before a later API observation.
        if unsafe { self.state.storage().state.assume_init() } != BROTLI_STATE_UNINITED {
            return false;
        }
        match parameter {
            BROTLI_DECODER_PARAM_DISABLE_RING_BUFFER_REALLOCATION => {
                self.state.write_flag(4, 1, u32::from(value == 0));
                true
            }
            BROTLI_DECODER_PARAM_LARGE_WINDOW => {
                self.state.write_flag(5, 1, u32::from(value != 0));
                true
            }
            _ => false,
        }
    }

    pub fn is_used(&self) -> bool {
        // SAFETY: Successful source StateInit initializes state and the
        // nested bit-reader bit count. The short-circuit order matches C and
        // never touches its still-unknown input pointer fields.
        unsafe {
            self.state.storage().state.assume_init() != BROTLI_STATE_UNINITED
                || (*self.state.storage().br.as_ptr()).bit_pos.assume_init() != 0
        }
    }

    pub fn get_error_code(&self) -> i32 {
        // SAFETY: Source Init initializes this field; SaveErrorCode writes
        // the same native integer before every streaming status return.
        unsafe { self.state.storage().error_code.assume_init() }
    }

    /// Store the actual callable source metadata functions and opaque value.
    ///
    /// # Safety
    /// The functions and opaque must remain valid until replaced or until
    /// this instance is destroyed. Callbacks must not unwind; chunk data is a
    /// borrowed source buffer valid only in the call. A callback can replace
    /// these fields through metadata_callbacks_handle after storage loans end.
    pub unsafe fn set_metadata_callbacks(
        &mut self,
        start: Option<MetadataStartFunc>,
        chunk: Option<MetadataChunkFunc>,
        opaque: *mut c_void,
    ) {
        // SAFETY: The exclusive instance borrow establishes a live state
        // with no conflicting field references. The caller establishes the
        // replacement callbacks' lifetime and no-unwind contract.
        unsafe { self.metadata_callbacks_handle().set(start, chunk, opaque) };
    }

    /// Retain the original allocation-relative callback setter token. Its
    /// mutation is unsafe and requires the producing allocation to remain
    /// live; this operation creates no source-storage reference or extra owner.
    pub fn metadata_callbacks_handle(&self) -> MetadataCallbacksHandle {
        MetadataCallbacksHandle {
            state: NonNull::new(self.state.as_ptr().cast_mut().cast())
                .expect("the live source decoder allocation"),
        }
    }

    /// The original opaque state's actual stable allocation identity. The
    /// pointer is not an independent owner and grants no dereference access.
    pub fn as_ptr(&self) -> *const u8 {
        self.state.as_ptr()
    }
}

/// BrotliDecoderDestroyInstance, including its null-state no-op. StateOwner's
/// source-ordered cleanup releases every actual block with its producing pair.
pub fn destroy_instance(instance: Option<DecoderInstance<'_>>) {
    drop(instance);
}
