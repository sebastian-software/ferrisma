//! Target runtime for the original source assertion macro.
#[cfg(target_os = "macos")]
use core::ffi::{c_char, c_int};

// Zero-length fields with these types retain C member alignment without
// changing the member's value, indexing, or size.
#[derive(Copy, Clone)]
#[repr(align(2))]
pub struct Align2;
#[derive(Copy, Clone)]
#[repr(align(4))]
pub struct Align4;
#[derive(Copy, Clone)]
#[repr(align(8))]
pub struct Align8;
#[derive(Copy, Clone)]
#[repr(align(16))]
pub struct Align16;
#[derive(Copy, Clone)]
#[repr(align(32))]
pub struct Align32;
#[derive(Copy, Clone)]
#[repr(align(64))]
pub struct Align64;

// The single field stays at offset zero. These storage wrappers preserve
// explicit alignment on original C local and static objects; all generated
// value operations continue to address the field itself.
#[repr(C, align(2))]
pub struct Aligned2<T>(pub T);
#[repr(C, align(4))]
pub struct Aligned4<T>(pub T);
#[repr(C, align(8))]
pub struct Aligned8<T>(pub T);
#[repr(C, align(16))]
pub struct Aligned16<T>(pub T);
#[repr(C, align(32))]
pub struct Aligned32<T>(pub T);
#[repr(C, align(64))]
pub struct Aligned64<T>(pub T);

#[cfg(target_os = "macos")]
pub(crate) unsafe fn assertion(function: *const c_char, file: *const c_char,
                               line: c_int, expression: *const c_char) {
    #[cfg(debug_assertions)]
    {
        extern "C" {
            fn __assert_rtn(function: *const c_char, file: *const c_char,
                            line: c_int, expression: *const c_char) -> !;
        }
        // SAFETY: Translation supplies the original static NUL-terminated
        // strings. The C assertion runtime terminates without unwinding.
        __assert_rtn(function, file, line, expression);
    }
    #[cfg(not(debug_assertions))]
    let _ = (function, file, line, expression);
}

// C11 atomics of the translation on stable Rust (tools/stable_rust.py, port
// ADR 0060). C2Rust lowered each C `atomic_*` call to a
// `core::intrinsics::atomic_*` call; the tool rewrites it to the helper below
// that names the same operation and ordering, for example
// `atomic_load::<_, {SeqCst}, false>` (a non-volatile load) to
// `atomic_load_seqcst`. Each helper performs exactly one access through
// `Atomic*::from_ptr`, which compiles to the same LLVM atomic instruction.
//
// SAFETY contract of every helper, identical to the intrinsic it replaces:
// `p` points to a live, writable C `atomic_int`/`atomic_uint` object (dav1d
// declares every such object `_Atomic`, in context, frame, task and
// picture-progress records allocated writable), aligned to its size (C's
// alignment of `int`/`unsigned`, which equals `align_of::<AtomicI32>()` on
// both targets), and every concurrent access to it is atomic and of the same
// size (C11 data-race rules, which the original keeps).
use core::sync::atomic::Ordering::{self, Acquire, Relaxed, SeqCst};
use core::sync::atomic::{AtomicI32, AtomicU32};

pub(crate) trait AtomicWord: Copy {
    unsafe fn load(p: *mut Self, order: Ordering) -> Self;
    unsafe fn store(p: *mut Self, value: Self, order: Ordering);
    unsafe fn swap(p: *mut Self, value: Self, order: Ordering) -> Self;
    unsafe fn fetch_add(p: *mut Self, value: Self, order: Ordering) -> Self;
    unsafe fn fetch_sub(p: *mut Self, value: Self, order: Ordering) -> Self;
    unsafe fn fetch_or(p: *mut Self, value: Self, order: Ordering) -> Self;
    unsafe fn compare_exchange(p: *mut Self, current: Self, new: Self,
                               success: Ordering, failure: Ordering) -> (Self, bool);
}

macro_rules! atomic_word {
    ($word:ty, $atomic:ty) => {
        impl AtomicWord for $word {
            #[inline(always)]
            unsafe fn load(p: *mut Self, order: Ordering) -> Self {
                <$atomic>::from_ptr(p).load(order)
            }
            #[inline(always)]
            unsafe fn store(p: *mut Self, value: Self, order: Ordering) {
                <$atomic>::from_ptr(p).store(value, order)
            }
            #[inline(always)]
            unsafe fn swap(p: *mut Self, value: Self, order: Ordering) -> Self {
                <$atomic>::from_ptr(p).swap(value, order)
            }
            #[inline(always)]
            unsafe fn fetch_add(p: *mut Self, value: Self, order: Ordering) -> Self {
                <$atomic>::from_ptr(p).fetch_add(value, order)
            }
            #[inline(always)]
            unsafe fn fetch_sub(p: *mut Self, value: Self, order: Ordering) -> Self {
                <$atomic>::from_ptr(p).fetch_sub(value, order)
            }
            #[inline(always)]
            unsafe fn fetch_or(p: *mut Self, value: Self, order: Ordering) -> Self {
                <$atomic>::from_ptr(p).fetch_or(value, order)
            }
            #[inline(always)]
            unsafe fn compare_exchange(p: *mut Self, current: Self, new: Self,
                                       success: Ordering, failure: Ordering) -> (Self, bool) {
                // The intrinsic is the strong exchange and returns the
                // previous value with the success flag.
                match <$atomic>::from_ptr(p).compare_exchange(current, new, success, failure) {
                    Ok(previous) => (previous, true),
                    Err(previous) => (previous, false),
                }
            }
        }
    };
}
atomic_word!(i32, AtomicI32);
atomic_word!(u32, AtomicU32);

macro_rules! atomic_helpers {
    ($($load:ident $order:ident),* ; $($store:ident $sorder:ident),* ;
     $($rmw:ident $method:ident $rorder:ident),*) => {
        $(#[inline(always)]
        pub(crate) unsafe fn $load<T: AtomicWord>(p: *const T) -> T {
            T::load(p as *mut T, $order)
        })*
        $(#[inline(always)]
        pub(crate) unsafe fn $store<T: AtomicWord>(p: *mut T, value: T) {
            T::store(p, value, $sorder)
        })*
        $(#[inline(always)]
        pub(crate) unsafe fn $rmw<T: AtomicWord>(p: *mut T, value: T) -> T {
            T::$method(p, value, $rorder)
        })*
    };
}
atomic_helpers!(atomic_load_seqcst SeqCst, atomic_load_relaxed Relaxed,
                atomic_load_acquire Acquire;
                atomic_store_seqcst SeqCst;
                atomic_xchg_seqcst swap SeqCst, atomic_xadd_seqcst fetch_add SeqCst,
                atomic_xadd_relaxed fetch_add Relaxed, atomic_xsub_seqcst fetch_sub SeqCst,
                atomic_or_seqcst fetch_or SeqCst);

#[inline(always)]
pub(crate) unsafe fn atomic_cxchg_seqcst_seqcst<T: AtomicWord>(p: *mut T, current: T, new: T)
                                                              -> (T, bool) {
    T::compare_exchange(p, current, new, SeqCst, SeqCst)
}
