//! WP11 differential test: the constants, enumerators and C type sizes of the
//! lossless encoder. The sizes are the `size` arguments of the allocations of
//! these types (ADR 0003); the C probe `tools/oracle/probes/wp11/constants.c`
//! lets the oracle's compiler lay them out.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Type sizes on other hosts.** The `C_SIZE` constants follow the 64-bit
//!    layout; the arm64 development host checks them, and the x86-64 CI host
//!    checks them again. 32-bit targets are not built.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::dsp::lossless_common::PREFIX_LOOKUP_IDX_MAX;
use webp::enc::backward_references_cost_enc::{CostCacheInterval, CostInterval, CostModel};
use webp::enc::backward_references_enc::{
    HASH_BITS, HASH_SIZE, MAX_COLOR_CACHE_BITS, MAX_LENGTH, MAX_LENGTH_BITS,
    MAX_REFS_BLOCK_PER_IMAGE, Mode, PixOrCopy, PixOrCopyBlock, VP8LLZ77Type, WINDOW_SIZE_BITS,
};
use webp::enc::histogram_enc::{
    HistogramPair, VP8L_NON_TRIVIAL_SYM, VP8LHistogram, VP8LHistogramSet,
};
use webp::enc::vp8l_enc::Histograms;
use webp::enc::vp8li_enc::{MAX_TRANSFORM_BITS, VP8LEncoderARGBContent, WEBP_NEAR_LOSSLESS};

#[test]
fn constants_and_sizes_match_the_c_headers() {
    let Some(oracle) = Oracle::locate("constants_and_sizes_match_the_c_headers") else {
        return;
    };
    let output = oracle.probe("wp11", "constants").run::<&str>(&[], b"");
    let actual = output.success().fields();
    let expected = Fields::new()
        .with("sizeof_VP8LHistogram", VP8LHistogram::C_SIZE)
        .with("sizeof_VP8LHistogramSet", VP8LHistogramSet::C_SIZE)
        .with("sizeof_PixOrCopy", PixOrCopy::C_SIZE)
        .with("sizeof_PixOrCopyBlock", PixOrCopyBlock::C_SIZE)
        .with("sizeof_HistogramPair", HistogramPair::C_SIZE)
        .with("sizeof_CostModel", CostModel::C_SIZE)
        .with("sizeof_CostInterval", CostInterval::C_SIZE)
        .with("sizeof_CostCacheInterval", CostCacheInterval::C_SIZE)
        .with("sizeof_Histograms", Histograms::C_SIZE)
        .with("MAX_COLOR_CACHE_BITS", MAX_COLOR_CACHE_BITS)
        .with("HASH_BITS", HASH_BITS)
        .with("HASH_SIZE", HASH_SIZE)
        .with("MAX_LENGTH_BITS", MAX_LENGTH_BITS)
        .with("WINDOW_SIZE_BITS", WINDOW_SIZE_BITS)
        .with("MAX_LENGTH", MAX_LENGTH)
        .with("MAX_REFS_BLOCK_PER_IMAGE", MAX_REFS_BLOCK_PER_IMAGE)
        .with("kLiteral", Mode::kLiteral.0)
        .with("kCacheIdx", Mode::kCacheIdx.0)
        .with("kCopy", Mode::kCopy.0)
        .with("kNone", Mode::kNone.0)
        .with("kLZ77Standard", VP8LLZ77Type::kLZ77Standard.0)
        .with("kLZ77RLE", VP8LLZ77Type::kLZ77RLE.0)
        .with("kLZ77Box", VP8LLZ77Type::kLZ77Box.0)
        .with("VP8L_NON_TRIVIAL_SYM", VP8L_NON_TRIVIAL_SYM)
        .with("MAX_TRANSFORM_BITS", MAX_TRANSFORM_BITS)
        .with("WEBP_NEAR_LOSSLESS", WEBP_NEAR_LOSSLESS)
        .with("kEncoderNone", VP8LEncoderARGBContent::kEncoderNone.0)
        .with("kEncoderARGB", VP8LEncoderARGBContent::kEncoderARGB.0)
        .with(
            "kEncoderNearLossless",
            VP8LEncoderARGBContent::kEncoderNearLossless.0,
        )
        .with("kEncoderPalette", VP8LEncoderARGBContent::kEncoderPalette.0)
        .with("PREFIX_LOOKUP_IDX_MAX", PREFIX_LOOKUP_IDX_MAX);
    assert_eq!(actual, expected);
}
