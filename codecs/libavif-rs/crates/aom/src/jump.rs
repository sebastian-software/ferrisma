//! Typed mapping of the original setjmp targets; never call libc setjmp in Rust.
use std::panic::{catch_unwind, resume_unwind, AssertUnwindSafe};

#[derive(Debug)]
struct Jump {
    target: usize,
    code: i32,
}

pub(crate) enum Scope<R> {
    Returned(R),
    Done,
}

pub(crate) fn scope<R>(target: *mut core::ffi::c_void,
                       body: impl FnOnce() -> Scope<R>) -> Option<Scope<R>> {
    value_scope(target, body)
}

pub(crate) fn value_scope<R>(target: *mut core::ffi::c_void,
                             body: impl FnOnce() -> R) -> Option<R> {
    // The generated closure captures only original C state. C's existing
    // recovery arm owns cleanup and synchronization after a nonlocal error.
    // Rust borrows end when catch_unwind returns, before that arm executes.
    match catch_unwind(AssertUnwindSafe(body)) {
        Ok(value) => Some(value),
        Err(payload) => match payload.downcast_ref::<Jump>() {
            Some(jump) if jump.target == target as usize => {
                debug_assert_ne!(jump.code, 0);
                None
            }
            _ => resume_unwind(payload),
        },
    }
}

pub(crate) fn raise<T>(target: *mut T, code: i32) -> ! {
    // The original call follows va_end and records diagnostics first. As with
    // C longjmp, bypass the process panic hook. Only its matching live scope
    // consumes this payload; nested unrelated targets propagate it unchanged.
    resume_unwind(Box::new(Jump { target: target as usize, code: if code == 0 { 1 } else { code } }))
}

#[cfg(test)]
mod tests {
    use super::*;

    // These are mapping tests, not additional claimed upstream codec cases.
    unsafe extern "C-unwind" fn transfer(target: *mut i32, code: i32) {
        raise(target, code);
    }

    #[test]
    fn return_and_normal_completion_stay_distinct() {
        let mut target = 0;
        let pointer = (&raw mut target).cast();
        assert!(matches!(scope(pointer, || Scope::Returned(73)), Some(Scope::Returned(73))));
        assert!(matches!(scope::<()>(pointer, || Scope::Done), Some(Scope::Done)));
    }

    #[test]
    fn matching_error_crosses_a_c_unwind_frame() {
        let mut target = 0;
        let mut observed = 0;
        let result = scope::<()>((&raw mut target).cast(), || {
            observed = 42;
            // SAFETY: Both target storage and its corresponding catch scope
            // remain live; the frame explicitly permits Rust unwinding.
            unsafe { transfer(&raw mut target, 9) };
            Scope::Done
        });
        assert!(result.is_none());
        assert_eq!(observed, 42);
    }

    #[test]
    fn unrelated_nested_target_propagates_to_its_owner() {
        let mut outer = 0;
        let mut inner = 0;
        let mut wrong_handler = false;
        let result = scope::<()>((&raw mut outer).cast(), || {
            let inner_result = scope::<()>((&raw mut inner).cast(), || raise(&raw mut outer, 4));
            wrong_handler = inner_result.is_none();
            Scope::Done
        });
        assert!(result.is_none());
        assert!(!wrong_handler);
    }

    #[test]
    fn handled_inner_error_leaves_outer_scope_live() {
        let mut outer = 0;
        let mut inner = 0;
        let mut inner_handled = false;
        let result = scope::<()>((&raw mut outer).cast(), || {
            inner_handled = scope::<()>((&raw mut inner).cast(), || raise(&raw mut inner, 2)).is_none();
            raise(&raw mut outer, 3)
        });
        assert!(result.is_none());
        assert!(inner_handled);
    }

    #[test]
    fn unrelated_rust_payload_is_not_a_codec_error() {
        let mut target = 0;
        let result = catch_unwind(AssertUnwindSafe(|| {
            scope::<()>((&raw mut target).cast(), || resume_unwind(Box::new(1234_u64)))
        }));
        assert_eq!(result.err().unwrap().downcast_ref::<u64>(), Some(&1234));
    }
}
