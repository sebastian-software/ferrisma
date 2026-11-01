# Retain C field alignment with zero-sized markers

C2Rust 0.21.0 omits some C field alignment attributes, changing internal decoder
layouts and causing valid frame headers to fail. Read the original compiler's
AST and retain each explicit field alignment with an immediately preceding
zero-sized aligned field. Retain record alignment on the Rust record itself.
This preserves member values, array indexing, original field offsets, and total
layout without wrapping or redesigning the algorithms' data types.

Resolve anonymous records through their containing original record and field
sequence; reject ambiguous mappings. Initialize synthetic zero-sized fields
mechanically in struct literals. Verify sizes, alignment, and member offsets
against original C for every named translated record instance, including the
two bit-depth instantiations. The importer records each correction; original
source and test expectations remain unchanged.

The original packed `refmvs_refpair` and `refmvs_block` typedefs additionally
require alignment of two and four bytes. Rust forbids combining `packed` and
`align`. Give the two-byte ref-pair union its natural alignment. Represent the
twelve-byte block with `packed(4)` to cap its eight-byte union member's alignment
while retaining the required four-byte record alignment. Every member offset
and total size must still match the independent C probes. Keep the genuinely
packed five-byte temporal motion record packed.
