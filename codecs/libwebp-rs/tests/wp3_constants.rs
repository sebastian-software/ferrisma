//! WP3 differential test: the constants, enumerators and C type sizes of
//! `huffman_utils.h`, `huffman_encode_utils.h`, `color_cache_utils.h`,
//! `palette.h` and `lossless_common.h`, printed by the C probe
//! `tools/oracle/probes/wp3/constants.c`. The sizes are the `size` arguments of
//! the allocations of these types (ADR 0003, allocation limits).
//!
//! Type sizes on other hosts are not compared. The `C_SIZE` constants follow
//! the pointer width; an arm64 host checks the 64-bit values, and the x86-64 CI
//! host checks them again. 32-bit targets are not built.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::dsp::lossless_common::{
    APPROX_LOG_MAX, APPROX_LOG_WITH_CORRECTION_MAX, LOG_2_PRECISION_BITS, LOG_2_RECIPROCAL,
    LOG_2_RECIPROCAL_FIXED, LOG_2_RECIPROCAL_FIXED_DOUBLE, LOG_LOOKUP_IDX_MAX,
    PREFIX_LOOKUP_IDX_MAX, WEBP_INT64_MAX, WEBP_UINT64_MAX,
};
use webp::utils::color_cache_utils::HASH_MUL;
use webp::utils::huffman_encode_utils::{HuffmanTree, HuffmanTreeCode, HuffmanTreeToken};
use webp::utils::huffman_utils::{
    HTreeGroup, HUFFMAN_PACKED_BITS, HUFFMAN_PACKED_TABLE_SIZE, HUFFMAN_TABLE_BITS,
    HUFFMAN_TABLE_MASK, HuffmanCode, HuffmanTablesSegment, LENGTHS_TABLE_BITS, LENGTHS_TABLE_MASK,
};
use webp::utils::palette::PaletteSorting;

fn rust_values() -> Fields {
    Fields::new()
        .with("sizeof_HuffmanCode", HuffmanCode::C_SIZE)
        .with("sizeof_HuffmanTablesSegment", HuffmanTablesSegment::C_SIZE)
        .with("sizeof_HTreeGroup", HTreeGroup::C_SIZE)
        .with("sizeof_HuffmanTreeToken", HuffmanTreeToken::C_SIZE)
        .with("sizeof_HuffmanTreeCode", HuffmanTreeCode::C_SIZE)
        .with("sizeof_HuffmanTree", HuffmanTree::C_SIZE)
        .with("HUFFMAN_TABLE_BITS", HUFFMAN_TABLE_BITS)
        .with("HUFFMAN_TABLE_MASK", HUFFMAN_TABLE_MASK)
        .with("LENGTHS_TABLE_BITS", LENGTHS_TABLE_BITS)
        .with("LENGTHS_TABLE_MASK", LENGTHS_TABLE_MASK)
        .with("HUFFMAN_PACKED_BITS", HUFFMAN_PACKED_BITS)
        .with("HUFFMAN_PACKED_TABLE_SIZE", HUFFMAN_PACKED_TABLE_SIZE)
        .with("kHashMul", HASH_MUL)
        .with("kSortedDefault", PaletteSorting::kSortedDefault.0)
        .with("kMinimizeDelta", PaletteSorting::kMinimizeDelta.0)
        .with("kModifiedZeng", PaletteSorting::kModifiedZeng.0)
        .with("kUnusedPalette", PaletteSorting::kUnusedPalette.0)
        .with("kPaletteSortingNum", PaletteSorting::kPaletteSortingNum.0)
        .with(
            "APPROX_LOG_WITH_CORRECTION_MAX",
            APPROX_LOG_WITH_CORRECTION_MAX,
        )
        .with("APPROX_LOG_MAX", APPROX_LOG_MAX)
        .with("LOG_2_PRECISION_BITS", LOG_2_PRECISION_BITS)
        .with("LOG_2_RECIPROCAL", LOG_2_RECIPROCAL.to_bits())
        .with(
            "LOG_2_RECIPROCAL_FIXED_DOUBLE",
            LOG_2_RECIPROCAL_FIXED_DOUBLE.to_bits(),
        )
        .with("LOG_2_RECIPROCAL_FIXED", LOG_2_RECIPROCAL_FIXED)
        .with("LOG_LOOKUP_IDX_MAX", LOG_LOOKUP_IDX_MAX)
        .with("WEBP_INT64_MAX", WEBP_INT64_MAX)
        .with("WEBP_UINT64_MAX", WEBP_UINT64_MAX)
        .with("PREFIX_LOOKUP_IDX_MAX", PREFIX_LOOKUP_IDX_MAX)
}

#[test]
fn constants_and_sizes_match_the_c_headers() {
    let Some(oracle) = Oracle::locate("constants_and_sizes_match_the_c_headers") else {
        return;
    };
    let output = oracle.probe("wp3", "constants").run(&[] as &[&str], b"");
    let actual = output.success().fields();
    let expected = rust_values();
    assert_eq!(
        actual.keys(),
        expected.keys(),
        "the probe prints other items"
    );
    for ((name, c_value), (_, rust_value)) in actual.0.iter().zip(&expected.0) {
        assert_eq!(rust_value, c_value, "{name} differs from the C header");
    }
}
