# Retain the empty x86 register barrier

The original scalar reconstruction code contains an empty GNU asm statement
with read/write constraints on the 64-bit `mask` and `mul` registers, even when
codec assembly is disabled. Keep both operands and their constraints. Reference
them in an assembler comment because Rust rejects unused asm operands. The
comment emits no machine instructions; it preserves the original opaque
register flow without adding or removing an optimization.
