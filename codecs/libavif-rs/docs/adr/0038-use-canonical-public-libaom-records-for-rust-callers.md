# Use canonical public libaom records for Rust callers

Status: Accepted (finalized 2026-11-26 as a living record, owner decision)

C2Rust preserves translation units, giving equivalent C records distinct
nominal Rust types in different modules. Generate a raw API from upstream's
selected export lists and translated signatures, using the encoder interface
unit's ABI-checked public record representations and original C-unwind symbols;
obtain ABI-version and flag constants from the original C headers. This avoids
requiring pointer casts between equivalent records without consolidating or
redesigning the mechanically translated implementation modules.
