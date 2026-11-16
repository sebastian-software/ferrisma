# Retain the empty x86 register barrier

> **2026-11-16: superseded.** Engine [ADR-0044](../../../../docs/adr/0044-no-assembly-acceleration-through-rust-intrinsics.md)
> allows no inline assembly, and the barrier is deleted. See the
> [amendment](#amendment-the-barrier-is-deleted-2026-11-16).

The original scalar reconstruction code contains an empty GNU asm statement
with read/write constraints on the 64-bit `mask` and `mul` registers, even when
codec assembly is disabled. Keep both operands and their constraints. Reference
them in an assembler comment because Rust rejects unused asm operands. The
comment emits no machine instructions; it preserves the original opaque
register flow without adding or removing an optimization.

## Amendment: the barrier is deleted (2026-11-16)

The statement is upstream's `__asm__("" : "+r"(mask), "+r"(mul))` in
`get_dc_sign_ctx` (`src/recon_tmpl.c`), compiled only for x86-64 GNU
compilers, with the comment that it coerces compilers into better code for
64-bit constants. It emits no instruction, and its read-write register
operands hand the compiler back the values it put in: `mask` and `mul` keep
their constants, so no value of the function can change by deleting it; what
can change is only how the compiler materializes the two constants. Engine
ADR-0044 allows no inline assembly, so the four Linux units
(`crates/dav1d/src/linux/{b8,b16,release/b8,release/b16}/recon_tmpl.rs`) no
longer carry it, and a comment at the site says so. The deletion is
`remove_register_barrier` in `tools/stable_rust.py` (checked in CI by its
`--check` mode), which `empty_register_barrier` in `tools/transpile_dav1d.py`
and `tools/import_dav1d_linux.py` call for a regeneration; it deletes C2Rust's
form and the assembler-comment form above. The manifests keep the correction
name `empty-readwrite-register-barrier`.
