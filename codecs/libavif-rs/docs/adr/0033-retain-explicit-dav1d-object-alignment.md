# Retain explicit dav1d object alignment

C2Rust 0.21.0 drops alignment attributes on local and static variables as well
as records and fields. Restore each original Clang `VarDecl` alignment with a
single-field C-layout storage wrapper and keep all existing value operations
on its offset-zero field. This preserves the original allocation alignment and
typed loads/stores rather than weakening Rust's pointer checks or changing
the original algorithm to tolerate misaligned storage. The complete Linux
Release corpus passed before this correction, but 1,182 Debug cases exposed
the missing alignment of `mask_edges_inter`'s `txa` buffer.

The original corpus reproducer is `av1-1-b8-23-film_grain-50` in
the native comparison run (retired libavif-rs repository, run 41352172727).
Keep that original test and its expected output unchanged.
