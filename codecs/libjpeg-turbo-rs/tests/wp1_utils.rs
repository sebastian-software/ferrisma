//! WP1 differential tests of the C sizes behind the memory accounting (ADR
//! 0005, "Exact accounting") and of `jutils.c`.
//!
//! The probe `tools/oracle/probes/wp1/sizes.c` prints `sizeof` of the public
//! structs and pointer types that allocation requests name, `ALIGN_SIZE` and
//! `MAX_ALLOC_CHUNK`, `jpeg_natural_order`, `jdiv_round_up` and `jround_up`
//! for a set of operands, and the effect of `jcopy_sample_rows`,
//! `jcopy_block_row` and `jzero_far` on small arrays. The private sizes
//! (`my_memory_mgr`, `jvirt_barray_control`) are checked by
//! `tests/wp1_memory.rs`, against the space the C memory manager accounts.

#[path = "support/oracle.rs"]
mod oracle;

use jpeg::jmemmgr::ALIGN_SIZE;
use jpeg::jmemsys::MAX_ALLOC_CHUNK;
use jpeg::jpeglib::{
    JBLOCK, RowListMut, SIZEOF_JBLOCK, SIZEOF_JBLOCKROW, SIZEOF_JHUFF_TBL, SIZEOF_JQUANT_TBL,
    SIZEOF_JSAMPROW, SIZEOF_jpeg_component_info, SIZEOF_jpeg_error_mgr, SampleArray,
};
use jpeg::jutils::{
    jcopy_block_row, jcopy_sample_rows, jdiv_round_up, jpeg_natural_order, jround_up, jzero_far,
};
use oracle::{Fields, Oracle};

/// The probe's operands for `jdiv_round_up` and `jround_up`.
const OPERANDS: [(i64, i64); 13] = [
    (0, 1),
    (1, 1),
    (1, 8),
    (7, 8),
    (8, 8),
    (9, 8),
    (227, 16),
    (149, 16),
    (65500, 8),
    (65500, 7),
    (1_000_000_000, 3),
    (4_294_967_295, 2),
    (123_456_789_012, 1000),
];

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// What `sizes.c` prints, computed by the port.
fn rust_fields() -> Fields {
    let mut fields = Fields::new()
        .with("sizeof_JQUANT_TBL", SIZEOF_JQUANT_TBL)
        .with("sizeof_JHUFF_TBL", SIZEOF_JHUFF_TBL)
        .with("sizeof_jpeg_component_info", SIZEOF_jpeg_component_info)
        .with("sizeof_jpeg_error_mgr", SIZEOF_jpeg_error_mgr)
        .with("sizeof_JBLOCK", SIZEOF_JBLOCK)
        .with("sizeof_JSAMPROW", SIZEOF_JSAMPROW)
        .with("sizeof_JBLOCKROW", SIZEOF_JBLOCKROW)
        .with("ALIGN_SIZE", ALIGN_SIZE)
        .with("MAX_ALLOC_CHUNK", MAX_ALLOC_CHUNK);
    let natural: Vec<String> = jpeg_natural_order.iter().map(i32::to_string).collect();
    fields = fields.with("jpeg_natural_order", natural.join(","));
    for (index, (a, b)) in OPERANDS.iter().enumerate() {
        fields = fields.with(
            &format!("round{index}"),
            format!("{},{}", jdiv_round_up(*a, *b), jround_up(*a, *b)),
        );
    }

    // Four rows of 16 samples, numbered 0..64; rows 0 and 1 onto rows 2 and
    // 3, five samples each. Upstream copies within one array; the port takes
    // a copy of the input rows, which the rows it writes do not overlap.
    let mut storage = SampleArray {
        buffer: (0..64).collect(),
        rows: vec![0, 16, 32, 48],
    };
    let input = storage.clone();
    jcopy_sample_rows(input.as_rows(), 0, storage.as_rows_mut(), 2, 2, 5);
    fields = fields.hex("copy_sample_rows", &storage.buffer);

    let mut blocks: [[JBLOCK; 3]; 2] = [[[0; 64]; 3]; 2];
    for (i, block) in blocks[0].iter_mut().enumerate() {
        for (j, coef) in block.iter_mut().enumerate() {
            *coef = i16::try_from(i * 100 + j).unwrap();
        }
    }
    blocks[1] = [[-1; 64]; 3];
    let [source, dest] = &mut blocks;
    jcopy_block_row(source, dest, 2);
    let copied: Vec<String> = blocks[1]
        .iter()
        .map(|block| format!("{},{}", block[0], block[63]))
        .collect();
    fields = fields.with("copy_block_row", copied.join(";"));

    jzero_far(&mut blocks[0], SIZEOF_JBLOCK);
    fields = fields.with(
        "zero_far_blocks",
        format!(
            "{},{},{}",
            blocks[0][0][0], blocks[0][0][63], blocks[0][1][0]
        ),
    );
    jzero_far(storage.row_mut(1), 5);
    fields.with("zero_far_samples", hex(&storage.buffer[16..32]))
}

#[test]
fn sizes_and_utilities_match_the_oracle() {
    let Some(oracle) = Oracle::locate("sizes_and_utilities_match_the_oracle") else {
        return;
    };
    let expected = oracle
        .probe("wp1", "sizes")
        .run::<&str>(&[], &[])
        .success()
        .fields();
    assert_eq!(rust_fields(), expected);
}

#[test]
fn row_copies_within_one_array() {
    // ADR 0005, "Caller rows": a copy within one array, such as
    // jcopy_sample_rows(output_data, outrow, output_data, outrow + 1, ...),
    // is RowListMut::copy_row, for both forms of rows.
    let mut shared = SampleArray {
        buffer: (0..32).collect(),
        rows: vec![0, 8, 16, 24],
    };
    let mut rows = shared.as_rows_mut();
    rows.copy_row(1, 2, 8);
    rows.copy_row(3, 0, 4);
    assert_eq!(
        shared.buffer,
        [
            24, 25, 26, 27, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 8, 9, 10, 11, 12, 13, 14, 15,
            24, 25, 26, 27, 28, 29, 30, 31
        ]
    );

    let mut row0 = [0u8; 4];
    let mut row1 = [1u8; 4];
    let mut row2 = [2u8; 4];
    let mut separate: [&mut [u8]; 3] = [&mut row0, &mut row1, &mut row2];
    let mut rows = RowListMut::Separate(&mut separate);
    rows.copy_row(2, 0, 3);
    rows.copy_row(1, 2, 2);
    rows.copy_row(1, 1, 4);
    assert_eq!((row0, row1, row2), ([2, 2, 2, 0], [1; 4], [1, 1, 2, 2]));
}
