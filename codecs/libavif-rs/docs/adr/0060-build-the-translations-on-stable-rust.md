# Build the translations on stable Rust

This amends [ADR 0059](0059-freeze-the-libaom-translation-and-gate-owned-code-on-golden-outputs.md)
(the frozen translations are owned code and change through committed tools)
and [ADR 0040](0040-use-rust-va-list-for-array-decayed-libaom-control-callbacks.md)
(its last sentence: the compiler is no longer pinned for this port). Engine
context: the owner removed rav1e on 2026-11-15, so this port is the engine's
only AVIF reader and writer, and the owner decided the same day that it must
build on stable Rust while keeping its four translation profiles (step (a) of
the consolidation plan; merging profiles and platforms are separate steps).

## Context

The libavif, dav1d and libaom translations declared three unstable features:

| Feature | Where | Uses (all profiles) |
| --- | --- | --- |
| `core_intrinsics` | dav1d; libaom declared it without a use | 550 `core::intrinsics::atomic_*` calls in dav1d (C11 atomics) |
| `extern_types` | all three | 1,555 `pub type X;` declarations of incomplete C structs: 1,180 in libaom (241 per macOS profile, 349 per Linux profile), 52 in dav1d, 323 in the libavif profiles and variants; 30 distinct names |
| `c_variadic` | libaom (declared); used without a declaration in dav1d and libavif | C-variadic definitions (`aom_internal_error`, `aom_set_error`, `aom_codec_control`, `dav1d_log`, `avifDiagnosticsPrintf`) and `core::ffi::VaList` in callbacks and control handlers |

`c_variadic` is stable since Rust 1.99.0: the pinned `nightly-2026-09-07`
reports "the feature `c_variadic` has been stable since 1.99.0", and the local
`nightly-2026-08-04` (`1.99.0-nightly`, before the 1.99 beta branch) accepts
C-variadic definitions, `VaList`, `next_arg` and `clone` with
`-Zallow-features=` (no feature may be enabled), while
`nightly-2026-07-20` and stable 1.98.1 reject them. The current stable
release is 1.100.0 (2026-11-09).

## Decision

### One committed, re-runnable transformation

`tools/stable_rust.py` (Python standard library, `python3 -I`) rewrites every
translated unit listed in a translation manifest, in all profiles of the three
crates, and records the new digests: through
`tools/enable_aom_dispatch.py --rehash` for the libaom manifests (ADR 0059's
owned-code route) and with the same digest definition
(`tools/check_translation.py`) for the others. It is idempotent;
`--check` fails if the committed tree differs from its output. Rewrites that
are not one of its patterns would be listed in `tools/stable_rust_edits.json`
and replayed by every run; there are none. Another tool that changes a
translated unit afterwards (for example the RTCD dispatch of ADR 0056/0058) is
followed by a rerun: on a new base, take the base's units and manifests and
run the tool again.

**Atomics.** Each `::core::intrinsics::atomic_<op>::<_, {Ordering}…>(…)` call
becomes `crate::platform::atomic_<op>_<ordering>(…)`, an owned helper in
`crates/dav1d/src/platform.rs` that performs the same single access through
`AtomicI32::from_ptr`/`AtomicU32::from_ptr` (stable since 1.75) with the
ordering the intrinsic named: `load` SeqCst, Relaxed and Acquire, `store`
SeqCst, `or`, `xchg`, `xsub` SeqCst, `xadd` SeqCst and Relaxed, and the strong
`cxchg` with SeqCst on success and failure (`compare_exchange`, result
reshaped to the intrinsic's `(previous, success)`). No ordering changed; the
port keeps dav1d's orderings. `from_ptr`'s preconditions are the intrinsic's:
the object is a live, writable C `_Atomic int`/`_Atomic unsigned` (context,
frame, task and progress records, all heap or static writable storage),
aligned to its size (C's `int` alignment equals `align_of::<AtomicI32>()` on
both targets), and all concurrent accesses are atomic and of the same size.
A scratch crate with every shape on `i32` and `u32` compiles to identical
machine code for the intrinsic and the helper at `opt-level=3` on
`aarch64-apple-darwin` and `x86_64-unknown-linux-gnu`.

**Extern types.** Each `pub type X;` in an `extern` block is removed and
`X` is defined after that block as

```rust
#[repr(C)]
pub struct X {
    _opaque: [u8; 0],
    _marker: core::marker::PhantomData<(*mut u8, core::marker::PhantomPinned)>,
}
```

zero-sized, not constructible outside its module, and neither `Send`, `Sync`
nor `Unpin`. The compiler accepted the extern types only behind pointers
(they are unsized), so every existing use is a pointer to `X`; a pointer to a
zero-sized struct is thin, so layouts and the C ABI do not change. Replacing
the opaque types with the real definitions of other units (rav1d's route) is
left to the profile consolidation.

**C-variadic functions stay as translated.** The port sets
`rust-version = "1.99"` in its packages and the two engine bridges, the first
release where they compile without a feature. (Working decision 2026-11-15,
delegated, revisable by the owner.)

### Rejected: typed error and control entry points

The alternative was to make the variadic entry points non-variadic for Rust
callers (typed error functions formatting their message in Rust, a typed
control argument), and to keep the C-ABI variadic symbols behind a
nightly-only Cargo feature for C consumers and the oracles. It would rewrite
about 6,300 call sites instead of none: `avifDiagnosticsPrintf` alone has
5,757 calls across the libavif profiles with many conversions,
`aom_internal_error` 464, `dav1d_log` 69; `dav1d_log` hands a `va_list` to
the caller's logger (libavif's `avifDav1dLogCallback` passes it to
`vsnprintf`), so a typed path would still need to build a platform `va_list`;
`AV1D_GET_MI_INFO` reads three arguments; and C callers that call these
symbols variadically need the original signature under the original name.
Since Rust 1.99 none of this is needed.

### rav1d precedent

rav1d (verified from its repository by the 2026-11-15 analysis) started from
one C2Rust transpile and reached stable 1.79 (#1309) by deduplicating types to
remove extern types (#593), moving to native atomics (#688), removing
`core_intrinsics` (#745) and removing `c_variadic` (#620), then moved its
atomics to relaxed orderings for speed. This port takes the same two first
steps by rewrite, keeps dav1d's orderings, and keeps its variadics because
they are stable now.

## What still needs nightly

- `rust-toolchain.toml` keeps the pinned nightly as the development default
  (engine ADR-0026); `rustfmt.toml` uses nightly-only options.
- AddressSanitizer builds (`cfg(libavif_rs_asan)` enables `feature(sanitize)`,
  plus `-Zsanitizer`).
- Fuzzing (`cargo +nightly fuzz`) in the other ports.

## Consequences

- `cargo +stable` (1.99 or later) builds the three crates and the engine's
  AVIF features in every profile; the gates in the amendment below compare
  the result with the frozen behavior.
- Reproducing the frozen translation from C2Rust now ends with
  `tools/stable_rust.py`.
- The C ABI is unchanged; C consumers and the nightly C oracles link the same
  symbols.
