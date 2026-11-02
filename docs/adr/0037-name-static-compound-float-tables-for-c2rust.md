# Name static compound float tables for C2Rust

C2Rust 0.21.0's AST exporter crashes when it treats the selected libaom
transform-search table's floating ConstantExpr nodes as integers. For the
translation input only, give the nine file-scope compound float arrays names
and keep their writable float elements, literal spellings, static storage
duration, and pointer-table positions; retain original line mappings and the
immutable source. Do not remove the table or change its thresholds to get a
successful translation.
