//! WP1 differential test: the accept-or-reject decisions of `WebPSafeMalloc` and
//! `WebPSafeCalloc` (`src/utils/utils.c`), on boundary sizes around
//! `WEBP_MAX_ALLOCABLE_MEMORY` and on products that overflow 64 bits. The probe
//! `tools/oracle/probes/wp1/safe_alloc.c` makes the same calls in C.
//!
//! The Rust helpers reserve `nmemb` elements of their type. The cases therefore
//! use element types whose size equals the C `size` argument (`[u8; 1]`,
//! `[u8; 2]`, `[u8; 4]`, `[u8; 8]`, and zero-sized `[u8; 0]`), so that an accepted
//! case reserves the same number of bytes in both languages. Calloc cases stay
//! small when accepted: the Rust calloc writes its zeros, while C's calloc of a
//! large block is lazily zeroed, and writing 16 GiB would not be a test.
//!
//! The C functions assert `nmemb * size > 0` on the accepted path (`utils.c`,
//! lines 200 and 210). A zero count or a zero size therefore has no C decision to
//! compare: the debug oracle aborts on it, and the other variants return whatever
//! `malloc(0)` returns. Those requests are excluded from the comparison in every
//! variant, and `zero_size_requests_are_recorded_for_rust_only` records the Rust
//! decision for them.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::utils::{WEBP_MAX_ALLOCABLE_MEMORY, webp_safe_calloc, webp_safe_malloc};

const MAX: u64 = WEBP_MAX_ALLOCABLE_MEMORY;

/// Accept-or-reject of `WebPSafeMalloc(nmemb, size)`, reserving the same bytes
/// as the C allocation for the sizes used by the cases.
fn malloc_accepts(nmemb: u64, size: u64) -> bool {
    match size {
        0 => webp_safe_malloc::<[u8; 0]>(nmemb, 0).is_some(),
        1 => webp_safe_malloc::<[u8; 1]>(nmemb, 1).is_some(),
        2 => webp_safe_malloc::<[u8; 2]>(nmemb, 2).is_some(),
        4 => webp_safe_malloc::<[u8; 4]>(nmemb, 4).is_some(),
        8 => webp_safe_malloc::<[u8; 8]>(nmemb, 8).is_some(),
        _ => webp_safe_malloc::<u8>(nmemb, usize::try_from(size).unwrap()).is_some(),
    }
}

/// Accept-or-reject of `WebPSafeCalloc(nmemb, size)`; see the module comment for
/// the accepted cases that the probe may run.
fn calloc_accepts(nmemb: u64, size: u64) -> bool {
    match size {
        0 => webp_safe_calloc::<[u8; 0]>(nmemb, 0).is_some(),
        1 => webp_safe_calloc::<u8>(nmemb, 1).is_some(),
        2 => webp_safe_calloc::<[u8; 2]>(nmemb, 2).is_some(),
        4 => webp_safe_calloc::<[u8; 4]>(nmemb, 4).is_some(),
        8 => webp_safe_calloc::<[u8; 8]>(nmemb, 8).is_some(),
        _ => webp_safe_calloc::<u8>(nmemb, usize::try_from(size).unwrap()).is_some(),
    }
}

/// Whether the C functions reach their `nmemb * size > 0` assertion with a nonzero
/// product, so that the comparison has a C decision to check. A zero count or a
/// zero size reaches the assertion with a zero product, which C leaves undefined.
/// A product that wraps in 64 bits fails the size test first, so it stays
/// compared.
fn has_c_decision(nmemb: u64, size: u64) -> bool {
    nmemb != 0 && size != 0
}

/// The malloc cases, which may reserve up to `WEBP_MAX_ALLOCABLE_MEMORY` bytes
/// lazily in both languages. The zero-size cases are listed here too, and the
/// tests below split them with `has_c_decision`.
fn malloc_cases() -> Vec<(u64, u64)> {
    vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (5, 0),
        (0, 4096),
        (3, 4),
        (1000, 8),
        (4096, 16),
        // Size 1 around the limit.
        (MAX - 1, 1),
        (MAX, 1),
        (MAX + 1, 1),
        // Sizes 2, 4 and 8 around the limit.
        (MAX / 2, 2),
        (MAX / 2 + 1, 2),
        (1 << 33, 2),
        (MAX / 4, 4),
        (MAX / 4 + 1, 4),
        (MAX / 8, 8),
        (MAX / 8 + 1, 8),
        // Sizes that fail the limit for a single element.
        (1, MAX + 1),
        (1, u64::MAX),
        (1 << 33, 3),
        (MAX / 3 + 1, 3),
        // Products that would wrap in 64 bits.
        (1 << 63, 2),
        (1 << 33 | 1, 2),
        (u64::MAX, 1),
        (1 << 63, 0),
    ]
}

/// The calloc cases: accepted ones stay small, the rejected ones sit on the
/// same boundaries as the malloc cases.
fn calloc_cases() -> Vec<(u64, u64)> {
    vec![
        (0, 0),
        (0, 1),
        (1, 0),
        (1, 1),
        (10, 4),
        (1000, 8),
        (4096, 16),
        (3, 4),
        (MAX + 1, 1),
        (1, MAX + 1),
        (1, u64::MAX),
        (1 << 33, 3),
        (1 << 33 | 1, 2),
        (MAX / 3 + 1, 3),
        (1 << 63, 2),
        (u64::MAX, 1),
    ]
}

#[test]
fn safe_allocation_decisions_match_the_c_helpers() {
    let Some(oracle) = Oracle::locate("safe_allocation_decisions_match_the_c_helpers") else {
        return;
    };
    let probe = oracle.probe("wp1", "safe_alloc");

    let mut stdin = String::new();
    let mut expected = Fields::new();
    for (nmemb, size) in malloc_cases()
        .into_iter()
        .filter(|&(nmemb, size)| has_c_decision(nmemb, size))
    {
        stdin.push_str(&format!("malloc {nmemb} {size}\n"));
        expected = expected.with("accepted", i32::from(malloc_accepts(nmemb, size)));
    }
    for (nmemb, size) in calloc_cases()
        .into_iter()
        .filter(|&(nmemb, size)| has_c_decision(nmemb, size))
    {
        stdin.push_str(&format!("calloc {nmemb} {size}\n"));
        expected = expected.with("accepted", i32::from(calloc_accepts(nmemb, size)));
    }

    let c_decisions = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_decisions, expected);
}

/// Records the Rust decision for the zero-size requests that the comparison
/// excludes. No C decision is compared here, so this needs no oracle. The size
/// test accepts a zero count or a zero size. `WebPSafeMalloc` reserves capacity
/// only, and `WebPSafeCalloc` fills `nmemb` zeroed elements.
#[test]
fn zero_size_requests_are_recorded_for_rust_only() {
    for (nmemb, size) in malloc_cases()
        .into_iter()
        .filter(|&(nmemb, size)| !has_c_decision(nmemb, size))
    {
        assert!(malloc_accepts(nmemb, size), "malloc({nmemb}, {size})");
    }
    for (nmemb, size) in calloc_cases()
        .into_iter()
        .filter(|&(nmemb, size)| !has_c_decision(nmemb, size))
    {
        assert!(calloc_accepts(nmemb, size), "calloc({nmemb}, {size})");
    }
    let malloc_len = webp_safe_malloc::<[u8; 0]>(5, 0);
    assert_eq!(malloc_len.as_ref().map(Vec::len), Some(0));
    let calloc_len = webp_safe_calloc::<[u8; 0]>(5, 0);
    assert_eq!(calloc_len.as_ref().map(Vec::len), Some(5));
    let empty_len = webp_safe_calloc::<[u8; 0]>(0, 0);
    assert_eq!(empty_len.as_ref().map(Vec::len), Some(0));
}

/// Checks the limit against the arithmetic of `CheckSizeArgumentsOverflow`
/// directly; it needs no oracle, and it reserves nothing, because every case
/// here is rejected before any reservation.
#[test]
fn rejected_sizes_never_reserve_memory() {
    assert_eq!(MAX, 1 << 34, "WEBP_MAX_ALLOCABLE_MEMORY on a 64-bit target");
    assert!(!malloc_accepts(MAX + 1, 1));
    assert!(!malloc_accepts(1, MAX + 1));
    assert!(!malloc_accepts(1 << 63, 2));
    assert!(!malloc_accepts(u64::MAX, 1));
    assert!(!calloc_accepts(MAX + 1, 1));
    assert!(!calloc_accepts(1 << 63, 2));
}
