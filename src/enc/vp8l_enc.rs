// libwebp v1.6.0, src/enc/vp8l_enc.c:
//
// Copyright 2012 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// main entry for the lossless encoder.
//
// Author: Vikas Arora (vikaas.arora@gmail.com)
//

//! Translation of `src/enc/vp8l_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP11 (VP8L encoder).
//!
//! `VP8LEncoderNew` keeps no pointer to the configuration and the picture:
//! the functions take them per call. The side context of `VP8LEncodeStream`
//! runs on a scoped thread (see `vp8l_encode_stream`).
//!
//! # Deviations from upstream
//!
//! Changes of representation that the ADRs allow, or behavior that the Rust
//! side cannot reproduce exactly. Each records the reason.
//!
//! 1. **Threads (ADR 0003, threads: a worker whose life lies inside one
//!    call).** The side context runs on a `std::thread::scope` thread, the main
//!    context on the calling thread. `WebPPicture` holds the writer and
//!    progress closures, which are not `Send`, so the side picture, a view of
//!    the whole picture as in C, crosses the thread as `PictureViewParts` (its
//!    fields without the four that a view has as `None`) and is rebuilt on the
//!    thread. While the side thread borrows the planes, the main context works
//!    on a second view of the picture whose progress hook forwards to the
//!    picture's hook; the picture's `stats` and progress hook leave the picture
//!    for that time and come back afterwards, and the view's error code is
//!    recorded on the picture after the join, before C's test of
//!    `picture->error_code`. With one context the main context works on the
//!    picture itself. A thread that cannot be created is C's failed `Reset` and
//!    records `VP8_ENC_ERROR_OUT_OF_MEMORY`. A panicking side context counts as
//!    a failed hook; its error code is then unknown, which C asserts cannot
//!    happen: the Rust code debug-asserts and otherwise behaves as C does when
//!    the side's error code is `VP8_ENC_OK`.
//! 2. **`ApplyPalette`** returns the error code that C records on the picture
//!    after its row allocation fails, and `MapImageFromPalette` records it,
//!    because the source pixels are borrowed from the picture. The outcome is
//!    the same. `APPLY_PALETTE_FOR` is a function over the color-to-index
//!    closure.
//! 3. **Return types (ADR 0003, errors).** Functions whose every failure
//!    records an error return `Result` with the recorded code; those that end
//!    with `return pic->error_code == VP8_ENC_OK` return that state as a
//!    `Result` (`VP8LGetHistoImageSymbols`, `EncodeImageNoHuffman`,
//!    `EncodeImageInternal`, `VP8LEncodeStream`, `VP8LEncodeImage`). Functions
//!    with a failure that records no error stay `bool`: `VP8LResidualImage` and
//!    `ApplyPredictFilter` (a failed modes allocation), `VP8ApplyNearLossless`,
//!    `EncoderAnalyze`, `EncoderInit`, `GetHuffBitLengthsAndCodes`, the write
//!    helpers and `EncodeStreamHook`.
//! 4. **`VP8LEncodeImage`** takes `&mut WebPPicture`, because C records the
//!    error and writes `stats` and `extra_info` through its `const` pointer. A
//!    missing writer panics where C calls `NULL` (see
//!    `# Excluded undefined behavior`, item 1).
//! 5. **`use_delta_palette`.** v1.6.0 declares the field "reserved" in
//!    `encode.h` and reads it nowhere, so the Rust encoder does not read it
//!    either; the probe (`tools/oracle/probes/wp11/encode.c`) shows identical
//!    bytes for both values. `WebPConfigInit` does not write it either
//!    (`src/webp/encode.rs`).
//!
//! # Excluded undefined behavior
//!
//! Inputs that are undefined behavior (or assertion failures) in C and are
//! therefore excluded from the oracle comparison (PORTING rule 8):
//!
//! 1. A picture without a writer: C calls the `NULL` pointer; Rust panics.
//! 2. `thread_level > 0` with `use_argb == 0` but an ARGB plane: the side
//!    picture, a view, gets no ARGB plane and C reads through `NULL`.
//! 3. Configurations and pictures that `WebPValidateConfig` and
//!    `WebPValidatePicture` reject (quality outside 0 to 100, where the `int`
//!    conversion is undefined for NaN and infinities; method outside 0 to 6;
//!    dimensions outside 1 to 16,383), and an `extra_info` shorter than the
//!    macroblock count, which C overruns.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **`WEBP_DISABLE_STATS`** is unset in the reference configuration, so the
//!    statistics code is translated and compared.
//! 2. **Thread creation failure** (C's failed `WebPWorker` `Reset`) and a
//!    panicking side context cannot be provoked; `# Deviations from upstream`,
//!    item 1, says what the Rust code does.
//! 3. **Uninitialized C memory.** The transform buffer, the histograms, the
//!    queue and the block storage are `malloc`ed in C and zeroed in Rust
//!    (ADR 0003). Every compared output is written before it is read; the
//!    predictor probe fills its scratch rows with 0xa5 bytes against the Rust
//!    zeros and still matches (`tests/wp11_predictor.rs`).

use crate::dsp::lossless_common::{
    vp8l_fast_log2, vp8l_prefix_encode, vp8l_sub_pixels, vp8l_sub_sample_size,
};
use crate::dsp::lossless_enc::{vp8l_bundle_color_map, vp8l_subtract_green_from_blue_and_red};
use crate::enc::backward_references_enc::{
    MAX_COLOR_CACHE_BITS, MAX_REFS_BLOCK_PER_IMAGE, VP8LBackwardRefs, VP8LHashChain, VP8LLZ77Type,
    pix_or_copy_cache_idx, pix_or_copy_distance, pix_or_copy_is_cache_idx, pix_or_copy_is_literal,
    pix_or_copy_length, pix_or_copy_literal, safe_malloc_test, to_index, try_filled,
    vp8l_backward_refs_clear, vp8l_backward_refs_init, vp8l_get_backward_references,
    vp8l_hash_chain_clear, vp8l_hash_chain_fill, vp8l_hash_chain_init, vp8l_refs_cursor_init,
    vp8l_refs_cursor_next, vp8l_refs_cursor_ok,
};
use crate::enc::histogram_enc::{
    VP8LHistogramSet, vp8l_allocate_histogram, vp8l_allocate_histogram_set, vp8l_bits_entropy,
    vp8l_get_histo_image_symbols, vp8l_histogram_num_codes, vp8l_histogram_set_clear,
    vp8l_histogram_store_refs,
};
use crate::enc::near_lossless_enc::vp8_apply_near_lossless;
use crate::enc::picture_csp_enc::webp_picture_has_transparency;
use crate::enc::picture_enc::argb_samples;
use crate::enc::picture_rescale_enc::webp_picture_view;
use crate::enc::predictor_enc::{
    vp8l_color_space_transform, vp8l_optimize_sampling, vp8l_residual_image,
};
use crate::enc::vp8li_enc::{MAX_TRANSFORM_BITS, VP8LEncoder, VP8LEncoderARGBContent};
use crate::enc::webp_enc::{webp_encoding_set_error, webp_report_progress};
use crate::utils::bit_writer_utils::{
    VP8LBitWriter, vp8l_bit_writer_clone, vp8l_bit_writer_finish, vp8l_bit_writer_init,
    vp8l_bit_writer_num_bytes, vp8l_bit_writer_reset, vp8l_bit_writer_swap,
    vp8l_bit_writer_wipe_out, vp8l_put_bits,
};
use crate::utils::huffman_encode_utils::{
    HuffmanTree, HuffmanTreeCode, HuffmanTreeToken, vp8l_create_compressed_huffman_tree,
    vp8l_create_huffman_tree,
};
use crate::utils::palette::{
    PaletteSorting, get_color_palette, palette_sort, prepare_map_to_palette, search_color_no_idx,
};
use crate::utils::utils::{bits_log2_floor, put_le32, webp_safe_malloc};
use crate::webp::encode::{
    WEBP_MAX_DIMENSION, WebPAuxStats, WebPConfig, WebPEncCSP, WebPEncodingError, WebPImageHint,
    WebPPicture,
};
use crate::webp::format_constants::{
    CHUNK_HEADER_SIZE, CODE_LENGTH_CODES, MAX_PALETTE_SIZE, MIN_HUFFMAN_BITS, MIN_TRANSFORM_BITS,
    NUM_DISTANCE_CODES, NUM_HUFFMAN_BITS, NUM_LENGTH_CODES, NUM_TRANSFORM_BITS, RIFF_HEADER_SIZE,
    TAG_SIZE, TRANSFORM_PRESENT, VP8L_IMAGE_SIZE_BITS, VP8L_MAGIC_BYTE, VP8L_SIGNATURE_SIZE,
    VP8L_VERSION, VP8L_VERSION_BITS, VP8LImageTransformType,
};
use crate::webp::types::{PlaneOffset, PlaneStorage};

// upstream: src/enc/vp8l_enc.c MAX_HUFF_IMAGE_SIZE
/// Maximum number of histogram images (sub-blocks).
const MAX_HUFF_IMAGE_SIZE: i32 = 2600;
// upstream: src/enc/vp8l_enc.c MAX_HUFFMAN_BITS
const MAX_HUFFMAN_BITS: i32 = MIN_HUFFMAN_BITS + (1 << NUM_HUFFMAN_BITS) - 1;
// upstream: src/enc/vp8l_enc.c MAX_PREDICTOR_IMAGE_SIZE
/// Empirical value for which it becomes too computationally expensive to
/// compute the best predictor image.
const MAX_PREDICTOR_IMAGE_SIZE: i32 = 1 << 14;

/// The C `int` value of a `bool` that C keeps in an `int` field and writes to
/// the bitstream.
#[inline]
fn bit(value: bool) -> u32 {
    u32::from(value)
}

/// `pic.error_code == VP8_ENC_OK` as a `Result`, which the functions that end
/// with `return (pic->error_code == VP8_ENC_OK);` return.
fn picture_status(pic: &WebPPicture<'_>) -> Result<(), WebPEncodingError> {
    if pic.error_code == WebPEncodingError::VP8_ENC_OK {
        Ok(())
    } else {
        Err(pic.error_code)
    }
}

/// The ARGB samples of `pic` from its plane's first sample.
fn picture_argb<'p>(pic: &'p WebPPicture<'_>) -> &'p [u32] {
    argb_samples(
        &pic.argb_storage,
        pic.argb.expect("the picture has an ARGB plane"),
    )
}

/// `VP8LSubSampleSize` on the C `int` values that the encoder passes.
#[inline]
fn sub_sample_size(size: i32, sampling_bits: i32) -> i32 {
    vp8l_sub_sample_size(size.cast_unsigned(), sampling_bits.cast_unsigned()).cast_signed()
}

// -----------------------------------------------------------------------------
// Palette

// upstream: src/enc/vp8l_enc.c EntropyIx
/// These five modes are evaluated and their respective entropy is computed.
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EntropyIx(u32);

#[expect(non_upper_case_globals, reason = "C enumerator names")]
impl EntropyIx {
    // upstream: src/enc/vp8l_enc.c kDirect
    const kDirect: Self = Self(0);
    // upstream: src/enc/vp8l_enc.c kSpatial
    const kSpatial: Self = Self(1);
    // upstream: src/enc/vp8l_enc.c kSubGreen
    const kSubGreen: Self = Self(2);
    // upstream: src/enc/vp8l_enc.c kSpatialSubGreen
    const kSpatialSubGreen: Self = Self(3);
    // upstream: src/enc/vp8l_enc.c kPalette
    const kPalette: Self = Self(4);
    // upstream: src/enc/vp8l_enc.c kPaletteAndSpatial
    const kPaletteAndSpatial: Self = Self(5);
    // upstream: src/enc/vp8l_enc.c kNumEntropyIx
    const kNumEntropyIx: Self = Self(6);

    /// The C `int` value of the enumerator.
    #[expect(clippy::cast_possible_wrap, reason = "the enumerators are below 7")]
    const fn int(self) -> i32 {
        self.0 as i32
    }
}

// upstream: src/enc/vp8l_enc.c HistoIx
#[repr(transparent)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct HistoIx(u32);

#[expect(non_upper_case_globals, reason = "C enumerator names")]
impl HistoIx {
    // upstream: src/enc/vp8l_enc.c kHistoAlpha
    const kHistoAlpha: Self = Self(0);
    // upstream: src/enc/vp8l_enc.c kHistoAlphaPred
    const kHistoAlphaPred: Self = Self(1);
    // upstream: src/enc/vp8l_enc.c kHistoGreen
    const kHistoGreen: Self = Self(2);
    // upstream: src/enc/vp8l_enc.c kHistoGreenPred
    const kHistoGreenPred: Self = Self(3);
    // upstream: src/enc/vp8l_enc.c kHistoRed
    const kHistoRed: Self = Self(4);
    // upstream: src/enc/vp8l_enc.c kHistoRedPred
    const kHistoRedPred: Self = Self(5);
    // upstream: src/enc/vp8l_enc.c kHistoBlue
    const kHistoBlue: Self = Self(6);
    // upstream: src/enc/vp8l_enc.c kHistoBluePred
    const kHistoBluePred: Self = Self(7);
    // upstream: src/enc/vp8l_enc.c kHistoRedSubGreen
    const kHistoRedSubGreen: Self = Self(8);
    // upstream: src/enc/vp8l_enc.c kHistoRedPredSubGreen
    const kHistoRedPredSubGreen: Self = Self(9);
    // upstream: src/enc/vp8l_enc.c kHistoBlueSubGreen
    const kHistoBlueSubGreen: Self = Self(10);
    // upstream: src/enc/vp8l_enc.c kHistoBluePredSubGreen
    const kHistoBluePredSubGreen: Self = Self(11);
    // upstream: src/enc/vp8l_enc.c kHistoPalette
    const kHistoPalette: Self = Self(12);
    // upstream: src/enc/vp8l_enc.c kHistoTotal
    /// Must be last.
    const kHistoTotal: Self = Self(13);

    /// The array index of the enumerator.
    const fn at(self) -> usize {
        self.0 as usize
    }
}

// upstream: src/enc/vp8l_enc.c NUM_BUCKETS
const NUM_BUCKETS: i32 = 256;

// upstream: src/enc/vp8l_enc.c HistogramBuckets
type HistogramBuckets = [u32; NUM_BUCKETS as usize];

// upstream: src/enc/vp8l_enc.c Histograms
/// Keeping track of histograms, indexed by `HistoIx`.
/// Ideally, this would just be a struct with meaningful fields, but the
/// calculation of `entropy_comp` uses the index. One refactoring at a time :)
pub struct Histograms {
    category: Vec<HistogramBuckets>,
}

impl Histograms {
    /// The C `sizeof(Histograms)`.
    pub const C_SIZE: usize = 13 * 256 * 4;

    /// Two different categories, mutably.
    fn two(&mut self, a: HistoIx, b: HistoIx) -> (&mut HistogramBuckets, &mut HistogramBuckets) {
        crate::enc::backward_references_enc::split_two(&mut self.category, a.at(), b.at())
    }
}

// upstream: src/enc/vp8l_enc.c AddSingleSubGreen
/// The C code shifts the pixel as an `int`; the shifts are arithmetic, as in
/// Clang and GCC, and the mask keeps the low byte.
#[expect(clippy::cast_sign_loss, reason = "the masked byte is not negative")]
fn add_single_sub_green(p: u32, r: &mut HistogramBuckets, b: &mut HistogramBuckets) {
    let green = p.cast_signed() >> 8; // The upper bits are masked away later.
    r[(((p.cast_signed() >> 16) - green) & 0xff) as usize] += 1;
    b[((p.cast_signed() - green) & 0xff) as usize] += 1;
}

// upstream: src/enc/vp8l_enc.c AddSingle
#[expect(
    clippy::many_single_char_names,
    reason = "the C names p, a, r, g and b"
)]
fn add_single(p: u32, histo: &mut Histograms, a: HistoIx, r: HistoIx, g: HistoIx, b: HistoIx) {
    histo.category[a.at()][((p >> 24) & 0xff) as usize] += 1;
    histo.category[r.at()][((p >> 16) & 0xff) as usize] += 1;
    histo.category[g.at()][((p >> 8) & 0xff) as usize] += 1;
    histo.category[b.at()][(p & 0xff) as usize] += 1;
}

// upstream: src/enc/vp8l_enc.c HashPix
#[inline]
fn hash_pix(pix: u32) -> u8 {
    // Note that masking with 0xffffffffu is for preventing an
    // 'unsigned int overflow' warning. Doesn't impact the compiled code.
    (((u64::from(pix) + u64::from(pix >> 19)).wrapping_mul(0x39c5_fba7) & 0xffff_ffff) >> 24) as u8
}

// upstream: src/enc/vp8l_enc.c AnalyzeEntropy
/// `argb` starts at the picture's first pixel. Returns `false` when the
/// histograms cannot be allocated.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of AnalyzeEntropy has nine parameters"
)]
#[expect(
    clippy::too_many_lines,
    reason = "the C function AnalyzeEntropy is translated as one function"
)]
fn analyze_entropy(
    argb: &[u32],
    width: i32,
    height: i32,
    argb_stride: i32,
    use_palette: bool,
    palette_size: i32,
    transform_bits: i32,
    min_entropy_ix: &mut EntropyIx,
    red_and_blue_always_zero: &mut bool,
) -> bool {
    if use_palette && palette_size <= 16 {
        // In the case of small palettes, we pack 2, 4 or 8 pixels together. In
        // practice, small palettes are better than any other transform.
        *min_entropy_ix = EntropyIx::kPalette;
        *red_and_blue_always_zero = true;
        return true;
    }

    if !safe_malloc_test(1, Histograms::C_SIZE) {
        return false;
    }
    let Some(category) = try_filled(HistoIx::kHistoTotal.at(), [0_u32; NUM_BUCKETS as usize])
    else {
        return false;
    };
    let mut histo = Histograms { category };
    {
        let mut prev_row: Option<usize> = None;
        let mut curr_row: usize = 0;
        let mut pix_prev = argb[0]; // Skip the first pixel.
        for _y in 0..height {
            for x in 0..to_index(width) {
                let pix = argb[curr_row + x];
                let pix_diff = vp8l_sub_pixels(pix, pix_prev);
                pix_prev = pix;
                if pix_diff == 0 || prev_row.is_some_and(|prev| pix == argb[prev + x]) {
                    continue;
                }
                add_single(
                    pix,
                    &mut histo,
                    HistoIx::kHistoAlpha,
                    HistoIx::kHistoRed,
                    HistoIx::kHistoGreen,
                    HistoIx::kHistoBlue,
                );
                add_single(
                    pix_diff,
                    &mut histo,
                    HistoIx::kHistoAlphaPred,
                    HistoIx::kHistoRedPred,
                    HistoIx::kHistoGreenPred,
                    HistoIx::kHistoBluePred,
                );
                let (r, b) = histo.two(HistoIx::kHistoRedSubGreen, HistoIx::kHistoBlueSubGreen);
                add_single_sub_green(pix, r, b);
                let (r, b) = histo.two(
                    HistoIx::kHistoRedPredSubGreen,
                    HistoIx::kHistoBluePredSubGreen,
                );
                add_single_sub_green(pix_diff, r, b);
                {
                    // Approximate the palette by the entropy of the multiplicative hash.
                    let hash = hash_pix(pix);
                    histo.category[HistoIx::kHistoPalette.at()][usize::from(hash)] += 1;
                }
            }
            prev_row = Some(curr_row);
            curr_row += to_index(argb_stride);
        }
        {
            const TOTAL: usize = HistoIx::kHistoTotal.at();
            const NUM_ENTROPY: usize = EntropyIx::kNumEntropyIx.0 as usize;
            let mut entropy_comp = [0_u64; TOTAL];
            let mut entropy = [0_u64; NUM_ENTROPY];
            let last_mode_to_analyze = if use_palette {
                EntropyIx::kPalette
            } else {
                EntropyIx::kSpatialSubGreen
            };
            // Let's add one zero to the predicted histograms. The zeros are removed
            // too efficiently by the pix_diff == 0 comparison, at least one of the
            // zeros is likely to exist.
            histo.category[HistoIx::kHistoRedPredSubGreen.at()][0] += 1;
            histo.category[HistoIx::kHistoBluePredSubGreen.at()][0] += 1;
            histo.category[HistoIx::kHistoRedPred.at()][0] += 1;
            histo.category[HistoIx::kHistoGreenPred.at()][0] += 1;
            histo.category[HistoIx::kHistoBluePred.at()][0] += 1;
            histo.category[HistoIx::kHistoAlphaPred.at()][0] += 1;

            for (comp, category) in entropy_comp.iter_mut().zip(&histo.category) {
                *comp = vp8l_bits_entropy(category, NUM_BUCKETS);
            }
            let e = |ix: HistoIx| entropy_comp[ix.at()];
            entropy[EntropyIx::kDirect.0 as usize] = e(HistoIx::kHistoAlpha)
                .wrapping_add(e(HistoIx::kHistoRed))
                .wrapping_add(e(HistoIx::kHistoGreen))
                .wrapping_add(e(HistoIx::kHistoBlue));
            entropy[EntropyIx::kSpatial.0 as usize] = e(HistoIx::kHistoAlphaPred)
                .wrapping_add(e(HistoIx::kHistoRedPred))
                .wrapping_add(e(HistoIx::kHistoGreenPred))
                .wrapping_add(e(HistoIx::kHistoBluePred));
            entropy[EntropyIx::kSubGreen.0 as usize] = e(HistoIx::kHistoAlpha)
                .wrapping_add(e(HistoIx::kHistoRedSubGreen))
                .wrapping_add(e(HistoIx::kHistoGreen))
                .wrapping_add(e(HistoIx::kHistoBlueSubGreen));
            entropy[EntropyIx::kSpatialSubGreen.0 as usize] = e(HistoIx::kHistoAlphaPred)
                .wrapping_add(e(HistoIx::kHistoRedPredSubGreen))
                .wrapping_add(e(HistoIx::kHistoGreenPred))
                .wrapping_add(e(HistoIx::kHistoBluePredSubGreen));
            entropy[EntropyIx::kPalette.0 as usize] = e(HistoIx::kHistoPalette);

            // When including transforms, there is an overhead in bits from
            // storing them. This overhead is small but matters for small images.
            // For spatial, there are 14 transformations.
            let tiles = u64::from(sub_sample_size(width, transform_bits).cast_unsigned())
                .wrapping_mul(u64::from(
                    sub_sample_size(height, transform_bits).cast_unsigned(),
                ));
            entropy[EntropyIx::kSpatial.0 as usize] = entropy[EntropyIx::kSpatial.0 as usize]
                .wrapping_add(tiles.wrapping_mul(u64::from(vp8l_fast_log2(14))));
            // For color transforms: 24 as only 3 channels are considered in a
            // ColorTransformElement.
            entropy[EntropyIx::kSpatialSubGreen.0 as usize] = entropy
                [EntropyIx::kSpatialSubGreen.0 as usize]
                .wrapping_add(tiles.wrapping_mul(u64::from(vp8l_fast_log2(24))));
            // For palettes, add the cost of storing the palette.
            // We empirically estimate the cost of a compressed entry as 8 bits.
            // The palette is differential-coded when compressed hence a much
            // lower cost than sizeof(uint32_t)*8.
            entropy[EntropyIx::kPalette.0 as usize] = entropy[EntropyIx::kPalette.0 as usize]
                .wrapping_add(i64::from(palette_size).cast_unsigned().wrapping_mul(8) << 23);

            *min_entropy_ix = EntropyIx::kDirect;
            for k in (EntropyIx::kDirect.0 + 1)..=last_mode_to_analyze.0 {
                if entropy[min_entropy_ix.0 as usize] > entropy[k as usize] {
                    *min_entropy_ix = EntropyIx(k);
                }
            }
            debug_assert!(min_entropy_ix.0 <= last_mode_to_analyze.0);
            *red_and_blue_always_zero = true;
            // Let's check if the histogram of the chosen entropy mode has
            // non-zero red and blue values. If all are zero, we can later skip
            // the cross color optimization.
            {
                const HISTO_PAIRS: [[HistoIx; 2]; 5] = [
                    [HistoIx::kHistoRed, HistoIx::kHistoBlue],
                    [HistoIx::kHistoRedPred, HistoIx::kHistoBluePred],
                    [HistoIx::kHistoRedSubGreen, HistoIx::kHistoBlueSubGreen],
                    [
                        HistoIx::kHistoRedPredSubGreen,
                        HistoIx::kHistoBluePredSubGreen,
                    ],
                    [HistoIx::kHistoRed, HistoIx::kHistoBlue],
                ];
                let pair = HISTO_PAIRS[min_entropy_ix.0 as usize];
                let red_histo = &histo.category[pair[0].at()];
                let blue_histo = &histo.category[pair[1].at()];
                for i in 1..to_index(NUM_BUCKETS) {
                    if (red_histo[i] | blue_histo[i]) != 0 {
                        *red_and_blue_always_zero = false;
                        break;
                    }
                }
            }
        }
    }
    true
}

// upstream: src/enc/vp8l_enc.c ClampBits
/// Clamp histogram and transform bits.
fn clamp_bits(
    width: i32,
    height: i32,
    mut bits: i32,
    min_bits: i32,
    max_bits: i32,
    image_size_max: i32,
) -> i32 {
    let image_size_of = |b: i32| {
        (sub_sample_size(width, b)
            .cast_unsigned()
            .wrapping_mul(sub_sample_size(height, b).cast_unsigned()))
        .cast_signed()
    };
    bits = if bits < min_bits {
        min_bits
    } else if bits > max_bits {
        max_bits
    } else {
        bits
    };
    let mut image_size = image_size_of(bits);
    while bits < max_bits && image_size > image_size_max {
        bits += 1;
        image_size = image_size_of(bits);
    }
    // In case the bits reduce the image too much, choose the smallest value
    // setting the histogram image size to 1.
    while bits > min_bits && image_size == 1 {
        image_size = image_size_of(bits - 1);
        if image_size != 1 {
            break;
        }
        bits -= 1;
    }
    bits
}

// upstream: src/enc/vp8l_enc.c GetHistoBits
fn get_histo_bits(method: i32, use_palette: bool, width: i32, height: i32) -> i32 {
    // Make tile size a function of encoding method (Range: 0 to 6).
    let histo_bits = (if use_palette { 9 } else { 7 }) - method;
    clamp_bits(
        width,
        height,
        histo_bits,
        MIN_HUFFMAN_BITS,
        MAX_HUFFMAN_BITS,
        MAX_HUFF_IMAGE_SIZE,
    )
}

// upstream: src/enc/vp8l_enc.c GetTransformBits
#[expect(clippy::comparison_chain, reason = "the C conditional expression")]
fn get_transform_bits(method: i32, histo_bits: i32) -> i32 {
    let max_transform_bits = if method < 4 {
        6
    } else if method > 4 {
        4
    } else {
        5
    };
    let res = if histo_bits > max_transform_bits {
        max_transform_bits
    } else {
        histo_bits
    };
    debug_assert!(res <= MAX_TRANSFORM_BITS);
    res
}

// upstream: src/enc/vp8l_enc.c CRUNCH_SUBCONFIGS_MAX
/// Set of parameters to be used in each iteration of the cruncher.
const CRUNCH_SUBCONFIGS_MAX: usize = 2;

// upstream: src/enc/vp8l_enc.c CrunchSubConfig
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct CrunchSubConfig {
    lz77: u32,
    do_no_cache: bool,
}

// upstream: src/enc/vp8l_enc.c CrunchConfig
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct CrunchConfig {
    entropy_idx: i32,
    palette_sorting_type: PaletteSorting,
    sub_configs: [CrunchSubConfig; CRUNCH_SUBCONFIGS_MAX],
    sub_configs_size: i32,
}

// upstream: src/enc/vp8l_enc.c CRUNCH_CONFIGS_MAX
/// +2 because we add a palette sorting configuration for kPalette and
/// kPaletteAndSpatial.
const CRUNCH_CONFIGS_MAX: usize =
    EntropyIx::kNumEntropyIx.0 as usize + 2 * PaletteSorting::kPaletteSortingNum.0 as usize;

// upstream: src/enc/vp8l_enc.c EncoderAnalyze
/// `pic` and `config` are C's `enc->pic` and `enc->config`. Returns `false`
/// when the entropy analysis cannot allocate its histograms; C records no error
/// then, the caller does.
#[expect(
    clippy::too_many_lines,
    reason = "the C function EncoderAnalyze is translated as one function"
)]
#[expect(clippy::float_cmp, reason = "the C comparison config->quality == 100")]
fn encoder_analyze(
    enc: &mut VP8LEncoder,
    pic: &WebPPicture<'_>,
    config: &WebPConfig,
    crunch_configs: &mut [CrunchConfig; CRUNCH_CONFIGS_MAX],
    crunch_configs_size: &mut i32,
    red_and_blue_always_zero: &mut bool,
) -> bool {
    let width = pic.width;
    let height = pic.height;
    let method = config.method;
    let low_effort = config.method == 0;
    let n_lz77s: usize;
    // If set to 0, analyze the cache with the computed cache value. If 1, also
    // analyze with no-cache.
    let mut do_no_cache = false;
    debug_assert!(pic.argb.is_some());

    // Check whether a palette is possible.
    enc.palette_size = get_color_palette(pic, Some(&mut enc.palette_sorted));
    let use_palette = enc.palette_size <= MAX_PALETTE_SIZE;
    if !use_palette {
        enc.palette_size = 0;
    }

    // Empirical bit sizes.
    enc.histo_bits = get_histo_bits(method, use_palette, pic.width, pic.height);
    let transform_bits = get_transform_bits(method, enc.histo_bits);
    enc.predictor_transform_bits = transform_bits;
    enc.cross_color_transform_bits = transform_bits;

    if low_effort {
        // AnalyzeEntropy is somewhat slow.
        crunch_configs[0].entropy_idx = if use_palette {
            EntropyIx::kPalette.int()
        } else {
            EntropyIx::kSpatialSubGreen.int()
        };
        crunch_configs[0].palette_sorting_type = if use_palette {
            PaletteSorting::kSortedDefault
        } else {
            PaletteSorting::kUnusedPalette
        };
        n_lz77s = 1;
        *crunch_configs_size = 1;
    } else {
        let mut min_entropy_ix = EntropyIx::kDirect;
        // Try out multiple LZ77 on images with few colors.
        n_lz77s = if enc.palette_size > 0 && enc.palette_size <= 16 {
            2
        } else {
            1
        };
        if !analyze_entropy(
            picture_argb(pic),
            width,
            height,
            pic.argb_stride,
            use_palette,
            enc.palette_size,
            transform_bits,
            &mut min_entropy_ix,
            red_and_blue_always_zero,
        ) {
            return false;
        }
        if method == 6 && config.quality == 100.0 {
            do_no_cache = true;
            // Go brute force on all transforms.
            *crunch_configs_size = 0;
            for i in 0..EntropyIx::kNumEntropyIx.int() {
                // We can only apply kPalette or kPaletteAndSpatial if we can indeed use
                // a palette.
                let is_palette =
                    i == EntropyIx::kPalette.int() || i == EntropyIx::kPaletteAndSpatial.int();
                if !is_palette || use_palette {
                    debug_assert!(to_index(*crunch_configs_size) < CRUNCH_CONFIGS_MAX);
                    if use_palette && is_palette {
                        for sorting_method in 0..PaletteSorting::kPaletteSortingNum.0 {
                            let typed_sorting_method = PaletteSorting(sorting_method);
                            // TODO(vrabaud) kSortedDefault should be tested. It is omitted
                            // for now for backward compatibility.
                            if typed_sorting_method == PaletteSorting::kUnusedPalette
                                || typed_sorting_method == PaletteSorting::kSortedDefault
                            {
                                continue;
                            }
                            let config = &mut crunch_configs[to_index(*crunch_configs_size)];
                            config.entropy_idx = i;
                            config.palette_sorting_type = typed_sorting_method;
                            *crunch_configs_size += 1;
                        }
                    } else {
                        let config = &mut crunch_configs[to_index(*crunch_configs_size)];
                        config.entropy_idx = i;
                        config.palette_sorting_type = PaletteSorting::kUnusedPalette;
                        *crunch_configs_size += 1;
                    }
                }
            }
        } else {
            // Only choose the guessed best transform.
            *crunch_configs_size = 1;
            crunch_configs[0].entropy_idx = min_entropy_ix.int();
            crunch_configs[0].palette_sorting_type = if use_palette {
                PaletteSorting::kMinimizeDelta
            } else {
                PaletteSorting::kUnusedPalette
            };
            if config.quality >= 75.0 && method == 5 {
                // Test with and without color cache.
                do_no_cache = true;
                // If we have a palette, also check in combination with spatial.
                if min_entropy_ix == EntropyIx::kPalette {
                    *crunch_configs_size = 2;
                    crunch_configs[1].entropy_idx = EntropyIx::kPaletteAndSpatial.int();
                    crunch_configs[1].palette_sorting_type = PaletteSorting::kMinimizeDelta;
                }
            }
        }
    }
    // Fill in the different LZ77s.
    debug_assert!(n_lz77s <= CRUNCH_SUBCONFIGS_MAX);
    for config in &mut crunch_configs[..to_index(*crunch_configs_size)] {
        for j in 0..n_lz77s {
            debug_assert!(j < CRUNCH_SUBCONFIGS_MAX);
            config.sub_configs[j].lz77 = if j == 0 {
                VP8LLZ77Type::kLZ77Standard.0 | VP8LLZ77Type::kLZ77RLE.0
            } else {
                VP8LLZ77Type::kLZ77Box.0
            };
            config.sub_configs[j].do_no_cache = do_no_cache;
        }
        config.sub_configs_size = i32::try_from(n_lz77s).expect("one or two LZ77 types");
    }
    true
}

// upstream: src/enc/vp8l_enc.c EncoderInit
/// `pic` is C's `enc->pic`.
fn encoder_init(enc: &mut VP8LEncoder, pic: &WebPPicture<'_>) -> bool {
    let width = pic.width;
    let height = pic.height;
    let pix_cnt = width * height;
    // we round the block size up, so we're guaranteed to have
    // at most MAX_REFS_BLOCK_PER_IMAGE blocks used:
    let refs_block_size = (pix_cnt - 1) / MAX_REFS_BLOCK_PER_IMAGE + 1;
    if !vp8l_hash_chain_init(&mut enc.hash_chain, pix_cnt) {
        return false;
    }

    for refs in &mut enc.refs {
        vp8l_backward_refs_init(refs, refs_block_size);
    }

    true
}

// upstream: src/enc/vp8l_enc.c GetHuffBitLengthsAndCodes
/// Returns false in case of memory error. The C code carves the code lengths
/// and codes of every code from one allocation; the Rust code tests that total
/// and gives each `HuffmanTreeCode` its own arrays (see `HuffmanTreeCode` in
/// `src/utils/huffman_encode_utils.rs`). The histograms are modified by
/// `VP8LCreateHuffmanTree`, as in C.
fn get_huff_bit_lengths_and_codes(
    histogram_image: &mut VP8LHistogramSet,
    huffman_codes: &mut [HuffmanTreeCode],
) -> bool {
    let mut total_length_size: u64 = 0;
    let histogram_image_size = to_index(histogram_image.size);
    let mut max_num_symbols: i32 = 0;

    // Iterate over all histograms and get the aggregate number of codes used.
    for i in 0..histogram_image_size {
        let histo = histogram_image.histogram(i);
        let codes = &mut huffman_codes[5 * i..5 * i + 5];
        for (k, code) in codes.iter_mut().enumerate() {
            let num_symbols = match k {
                0 => vp8l_histogram_num_codes(histo.palette_code_bits),
                4 => NUM_DISTANCE_CODES,
                _ => 256,
            };
            code.num_symbols = num_symbols;
            total_length_size += i64::from(num_symbols).cast_unsigned();
        }
    }

    let ok = 'end: {
        // Allocate and Set Huffman codes.
        if !safe_malloc_test(total_length_size, 1 + 2) {
            break 'end false;
        }
        for code in &mut huffman_codes[..5 * histogram_image_size] {
            let bit_length = to_index(code.num_symbols);
            let (Some(code_lengths), Some(codes)) =
                (try_filled(bit_length, 0_u8), try_filled(bit_length, 0_u16))
            else {
                break 'end false;
            };
            code.code_lengths = code_lengths;
            code.codes = codes;
            if max_num_symbols < code.num_symbols {
                max_num_symbols = code.num_symbols;
            }
        }

        let max = to_index(max_num_symbols);
        let buf_rle = if safe_malloc_test(1, max) {
            try_filled(max, 0_u8)
        } else {
            None
        };
        let huff_tree =
            webp_safe_malloc::<HuffmanTree>(3 * max as u64, HuffmanTree::C_SIZE).map(|mut tree| {
                tree.resize(3 * max, HuffmanTree::default());
                tree
            });
        let (Some(mut buf_rle), Some(mut huff_tree)) = (buf_rle, huff_tree) else {
            break 'end false;
        };

        // Create Huffman trees.
        for i in 0..histogram_image_size {
            let codes = &mut huffman_codes[5 * i..5 * i + 5];
            let histo = histogram_image.histogram_mut(i);
            vp8l_create_huffman_tree(
                &mut histo.literal,
                15,
                &mut buf_rle,
                &mut huff_tree,
                &mut codes[0],
            );
            vp8l_create_huffman_tree(
                &mut histo.red,
                15,
                &mut buf_rle,
                &mut huff_tree,
                &mut codes[1],
            );
            vp8l_create_huffman_tree(
                &mut histo.blue,
                15,
                &mut buf_rle,
                &mut huff_tree,
                &mut codes[2],
            );
            vp8l_create_huffman_tree(
                &mut histo.alpha,
                15,
                &mut buf_rle,
                &mut huff_tree,
                &mut codes[3],
            );
            vp8l_create_huffman_tree(
                &mut histo.distance,
                15,
                &mut buf_rle,
                &mut huff_tree,
                &mut codes[4],
            );
        }
        true
    };
    if !ok {
        for code in &mut huffman_codes[..5 * histogram_image_size] {
            *code = HuffmanTreeCode::default();
        }
    }
    ok
}

// upstream: src/enc/vp8l_enc.c StoreHuffmanTreeOfHuffmanTreeToBitMask
fn store_huffman_tree_of_huffman_tree_to_bit_mask(
    bw: &mut VP8LBitWriter,
    code_length_bitdepth: &[u8],
) {
    // RFC 1951 will calm you down if you are worried about this funny sequence.
    // This sequence is tuned from that, but more weighted for lower symbol count,
    // and more spiking histograms.
    const STORAGE_ORDER: [u8; CODE_LENGTH_CODES as usize] = [
        17, 18, 0, 1, 2, 3, 4, 5, 16, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15,
    ];
    // Throw away trailing zeros:
    let mut codes_to_store = CODE_LENGTH_CODES;
    while codes_to_store > 4 {
        if code_length_bitdepth[usize::from(STORAGE_ORDER[to_index(codes_to_store - 1)])] != 0 {
            break;
        }
        codes_to_store -= 1;
    }
    vp8l_put_bits(bw, (codes_to_store - 4).cast_unsigned(), 4);
    for &order in &STORAGE_ORDER[..to_index(codes_to_store)] {
        vp8l_put_bits(bw, u32::from(code_length_bitdepth[usize::from(order)]), 3);
    }
}

// upstream: src/enc/vp8l_enc.c ClearHuffmanTreeIfOnlyOneSymbol
fn clear_huffman_tree_if_only_one_symbol(huffman_code: &mut HuffmanTreeCode) {
    let n = to_index(huffman_code.num_symbols);
    let mut count = 0;
    for &length in &huffman_code.code_lengths[..n] {
        if length != 0 {
            count += 1;
            if count > 1 {
                return;
            }
        }
    }
    huffman_code.code_lengths[..n].fill(0);
    huffman_code.codes[..n].fill(0);
}

// upstream: src/enc/vp8l_enc.c StoreHuffmanTreeToBitMask
fn store_huffman_tree_to_bit_mask(
    bw: &mut VP8LBitWriter,
    tokens: &[HuffmanTreeToken],
    num_tokens: i32,
    huffman_code: &HuffmanTreeCode,
) {
    for token in &tokens[..to_index(num_tokens)] {
        let ix = usize::from(token.code);
        let extra_bits = u32::from(token.extra_bits);
        vp8l_put_bits(
            bw,
            u32::from(huffman_code.codes[ix]),
            i32::from(huffman_code.code_lengths[ix]),
        );
        match ix {
            16 => vp8l_put_bits(bw, extra_bits, 2),
            17 => vp8l_put_bits(bw, extra_bits, 3),
            18 => vp8l_put_bits(bw, extra_bits, 7),
            _ => {}
        }
    }
}

// upstream: src/enc/vp8l_enc.c StoreFullHuffmanCode
/// `huff_tree` and `tokens` are pre-alloacted buffers.
fn store_full_huffman_code(
    bw: &mut VP8LBitWriter,
    huff_tree: &mut [HuffmanTree],
    tokens: &mut [HuffmanTreeToken],
    tree: &HuffmanTreeCode,
) {
    let max_tokens = to_index(tree.num_symbols);
    let mut huffman_code = HuffmanTreeCode {
        num_symbols: CODE_LENGTH_CODES,
        code_lengths: vec![0; CODE_LENGTH_CODES as usize],
        codes: vec![0; CODE_LENGTH_CODES as usize],
    };

    vp8l_put_bits(bw, 0, 1);
    let num_tokens = vp8l_create_compressed_huffman_tree(tree, &mut tokens[..max_tokens]);
    {
        let mut histogram = [0_u32; CODE_LENGTH_CODES as usize];
        let mut buf_rle = [0_u8; CODE_LENGTH_CODES as usize];
        for token in &tokens[..to_index(num_tokens)] {
            histogram[usize::from(token.code)] += 1;
        }

        vp8l_create_huffman_tree(
            &mut histogram,
            7,
            &mut buf_rle,
            huff_tree,
            &mut huffman_code,
        );
    }

    store_huffman_tree_of_huffman_tree_to_bit_mask(bw, &huffman_code.code_lengths);
    clear_huffman_tree_if_only_one_symbol(&mut huffman_code);
    {
        let code_length_bitdepth = &huffman_code.code_lengths;
        let mut trailing_zero_bits: i32 = 0;
        let mut trimmed_length = num_tokens;
        let mut i = num_tokens;
        while i > 0 {
            i -= 1;
            let ix = tokens[to_index(i)].code;
            if ix == 0 || ix == 17 || ix == 18 {
                trimmed_length -= 1; // discount trailing zeros
                trailing_zero_bits += i32::from(code_length_bitdepth[usize::from(ix)]);
                if ix == 17 {
                    trailing_zero_bits += 3;
                } else if ix == 18 {
                    trailing_zero_bits += 7;
                }
            } else {
                break;
            }
        }
        let write_trimmed_length = trimmed_length > 1 && trailing_zero_bits > 12;
        let length = if write_trimmed_length {
            trimmed_length
        } else {
            num_tokens
        };
        vp8l_put_bits(bw, bit(write_trimmed_length), 1);
        if write_trimmed_length {
            if trimmed_length == 2 {
                vp8l_put_bits(bw, 0, 3 + 2); // nbitpairs=1, trimmed_length=2
            } else {
                let nbits = bits_log2_floor((trimmed_length - 2).cast_unsigned());
                let nbitpairs = nbits / 2 + 1;
                debug_assert!(trimmed_length > 2);
                debug_assert!(nbitpairs - 1 < 8);
                vp8l_put_bits(bw, (nbitpairs - 1).cast_unsigned(), 3);
                vp8l_put_bits(bw, (trimmed_length - 2).cast_unsigned(), nbitpairs * 2);
            }
        }
        store_huffman_tree_to_bit_mask(bw, tokens, length, &huffman_code);
    }
}

// upstream: src/enc/vp8l_enc.c StoreHuffmanCode
/// `huff_tree` and `tokens` are pre-alloacted buffers.
fn store_huffman_code(
    bw: &mut VP8LBitWriter,
    huff_tree: &mut [HuffmanTree],
    tokens: &mut [HuffmanTreeToken],
    huffman_code: &HuffmanTreeCode,
) {
    const MAX_BITS: i32 = 8;
    const MAX_SYMBOL: i32 = 1 << MAX_BITS;
    let mut count = 0;
    let mut symbols: [i32; 2] = [0, 0];

    // Check whether it's a small tree.
    let mut i: i32 = 0;
    while i < huffman_code.num_symbols && count < 3 {
        if huffman_code.code_lengths[to_index(i)] != 0 {
            if count < 2 {
                symbols[count] = i;
            }
            count += 1;
        }
        i += 1;
    }

    if count == 0 {
        // emit minimal tree for empty cases
        // bits: small tree marker: 1, count-1: 0, large 8-bit code: 0, code: 0
        vp8l_put_bits(bw, 0x01, 4);
    } else if count <= 2 && symbols[0] < MAX_SYMBOL && symbols[1] < MAX_SYMBOL {
        vp8l_put_bits(bw, 1, 1); // Small tree marker to encode 1 or 2 symbols.
        vp8l_put_bits(bw, u32::try_from(count - 1).expect("one or two symbols"), 1);
        if symbols[0] <= 1 {
            vp8l_put_bits(bw, 0, 1); // Code bit for small (1 bit) symbol value.
            vp8l_put_bits(bw, symbols[0].cast_unsigned(), 1);
        } else {
            vp8l_put_bits(bw, 1, 1);
            vp8l_put_bits(bw, symbols[0].cast_unsigned(), 8);
        }
        if count == 2 {
            vp8l_put_bits(bw, symbols[1].cast_unsigned(), 8);
        }
    } else {
        store_full_huffman_code(bw, huff_tree, tokens, huffman_code);
    }
}

// upstream: src/enc/vp8l_enc.c WriteHuffmanCode
#[inline]
fn write_huffman_code(bw: &mut VP8LBitWriter, code: &HuffmanTreeCode, code_index: usize) {
    let depth = i32::from(code.code_lengths[code_index]);
    let symbol = u32::from(code.codes[code_index]);
    vp8l_put_bits(bw, symbol, depth);
}

// upstream: src/enc/vp8l_enc.c WriteHuffmanCodeWithExtraBits
#[inline]
fn write_huffman_code_with_extra_bits(
    bw: &mut VP8LBitWriter,
    code: &HuffmanTreeCode,
    code_index: usize,
    bits: i32,
    n_bits: i32,
) {
    let depth = i32::from(code.code_lengths[code_index]);
    let symbol = i32::from(code.codes[code_index]);
    vp8l_put_bits(
        bw,
        ((bits << depth) | symbol).cast_unsigned(),
        depth + n_bits,
    );
}

// upstream: src/enc/vp8l_enc.c StoreImageToBitMask
/// # Errors
///
/// Returns the error that `pic` records when the bit writer failed to grow.
#[expect(
    clippy::cast_possible_wrap,
    reason = "the C conversions of the stored lengths and distances to int"
)]
fn store_image_to_bit_mask(
    bw: &mut VP8LBitWriter,
    width: i32,
    histo_bits: i32,
    refs: &VP8LBackwardRefs,
    histogram_symbols: &[u32],
    huffman_codes: &[HuffmanTreeCode],
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let histo_xsize = if histo_bits != 0 {
        sub_sample_size(width, histo_bits)
    } else {
        1
    };
    let tile_mask = if histo_bits == 0 {
        0
    } else {
        -(1 << histo_bits)
    };
    // x and y trace the position in the image.
    let mut x: i32 = 0;
    let mut y: i32 = 0;
    let mut tile_x = x & tile_mask;
    let mut tile_y = y & tile_mask;
    let mut histogram_ix = ((histogram_symbols[0] >> 8) & 0xffff) as usize;
    let mut codes = &huffman_codes[5 * histogram_ix..];
    let mut c = vp8l_refs_cursor_init(refs);
    while vp8l_refs_cursor_ok(&c) {
        let v = c.get(refs);
        if tile_x != (x & tile_mask) || tile_y != (y & tile_mask) {
            tile_x = x & tile_mask;
            tile_y = y & tile_mask;
            histogram_ix = ((histogram_symbols
                [to_index((y >> histo_bits) * histo_xsize + (x >> histo_bits))]
                >> 8)
                & 0xffff) as usize;
            codes = &huffman_codes[5 * histogram_ix..];
        }
        if pix_or_copy_is_literal(v) {
            const ORDER: [i32; 4] = [1, 2, 0, 3];
            for (k, &component) in ORDER.iter().enumerate() {
                let code = pix_or_copy_literal(v, component) as usize;
                write_huffman_code(bw, &codes[k], code);
            }
        } else if pix_or_copy_is_cache_idx(v) {
            let code = pix_or_copy_cache_idx(v) as usize;
            let literal_ix = 256 + to_index(NUM_LENGTH_CODES) + code;
            write_huffman_code(bw, &codes[0], literal_ix);
        } else {
            let distance = pix_or_copy_distance(v) as i32;
            let (code, n_bits, bits) = vp8l_prefix_encode(i32::from(v.len));
            write_huffman_code_with_extra_bits(bw, &codes[0], 256 + to_index(code), bits, n_bits);

            // Don't write the distance with the extra bits code since
            // the distance can be up to 18 bits of extra bits, and the prefix
            // 15 bits, totaling to 33, and our PutBits only supports up to 32 bits.
            let (code, n_bits, bits) = vp8l_prefix_encode(distance);
            write_huffman_code(bw, &codes[4], to_index(code));
            vp8l_put_bits(bw, bits.cast_unsigned(), n_bits);
        }
        x += pix_or_copy_length(v) as i32;
        while x >= width {
            x -= width;
            y += 1;
        }
        vp8l_refs_cursor_next(&mut c, refs);
    }
    if bw.error {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }
    Ok(())
}

/// A buffer of `count` default elements after the C size test of
/// `WebPSafeMalloc(count, size)`.
fn safe_buffer<T: Clone + Default>(count: u64, size: usize) -> Option<Vec<T>> {
    let mut buffer = webp_safe_malloc::<T>(count, size)?;
    buffer.resize(usize::try_from(count).ok()?, T::default());
    Some(buffer)
}

// upstream: src/enc/vp8l_enc.c EncodeImageNoHuffman
/// Special case of `EncodeImageInternal()` for `cache_bits=0`, `histo_bits=31`.
/// `pic` and `percent` are for progress. `refs_array` holds at least the two
/// references that `VP8LGetBackwardReferences` uses.
///
/// # Errors
///
/// Returns the error that `pic` holds at the end, as C returns
/// `pic->error_code == VP8_ENC_OK`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of EncodeImageNoHuffman has eleven parameters"
)]
fn encode_image_no_huffman(
    bw: &mut VP8LBitWriter,
    argb: &[u32],
    hash_chain: &mut VP8LHashChain,
    refs_array: &mut [VP8LBackwardRefs],
    width: i32,
    height: i32,
    quality: i32,
    low_effort: bool,
    pic: &mut WebPPicture<'_>,
    percent_range: i32,
    percent: &mut i32,
) -> Result<(), WebPEncodingError> {
    let mut max_tokens: i32 = 0;
    let mut huffman_codes: [HuffmanTreeCode; 5] = Default::default();
    let histogram_symbols: [u32; 1] = [0]; // only one tree, one symbol
    let mut cache_bits: i32 = 0;
    let huff_tree = safe_buffer::<HuffmanTree>(3 * CODE_LENGTH_CODES as u64, HuffmanTree::C_SIZE);
    'error: {
        let Some(mut huff_tree) = huff_tree else {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        };

        // Calculate backward references from ARGB image.
        if vp8l_hash_chain_fill(
            hash_chain,
            quality,
            argb,
            width,
            height,
            low_effort,
            pic,
            percent_range / 2,
            percent,
        )
        .is_err()
        {
            break 'error;
        }
        if vp8l_get_backward_references(
            width,
            height,
            argb,
            quality,
            /*low_effort=*/ false,
            VP8LLZ77Type::kLZ77Standard.0 | VP8LLZ77Type::kLZ77RLE.0,
            cache_bits,
            /*do_no_cache=*/ false,
            hash_chain,
            refs_array,
            &mut cache_bits,
            pic,
            percent_range - percent_range / 2,
            percent,
        )
        .is_err()
        {
            break 'error;
        }
        let refs = &refs_array[0];
        let Some(mut histogram_image) = vp8l_allocate_histogram_set(1, cache_bits) else {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        };
        vp8l_histogram_set_clear(&mut histogram_image);

        // Build histogram image and symbols from backward references.
        vp8l_histogram_store_refs(
            refs,
            /*distance_modifier=*/ None,
            /*distance_modifier_arg0=*/ 0,
            histogram_image.histogram_mut(0),
        );

        // Create Huffman bit lengths and codes for each histogram image.
        debug_assert_eq!(histogram_image.size, 1);
        if !get_huff_bit_lengths_and_codes(&mut histogram_image, &mut huffman_codes) {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        // No color cache, no Huffman image.
        vp8l_put_bits(bw, 0, 1);

        // Find maximum number of symbols for the huffman tree-set.
        for codes in &huffman_codes {
            if max_tokens < codes.num_symbols {
                max_tokens = codes.num_symbols;
            }
        }

        let Some(mut tokens) = safe_buffer::<HuffmanTreeToken>(
            i64::from(max_tokens).cast_unsigned(),
            HuffmanTreeToken::C_SIZE,
        ) else {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        };

        // Store Huffman codes.
        for codes in &mut huffman_codes {
            store_huffman_code(bw, &mut huff_tree, &mut tokens, codes);
            clear_huffman_tree_if_only_one_symbol(codes);
        }

        // Store actual literals.
        let _ =
            store_image_to_bit_mask(bw, width, 0, refs, &histogram_symbols, &huffman_codes, pic);
    }

    picture_status(pic)
}

/// The copy `VP8LBitWriter bw_init = *bw` of C, which the encoder reads only
/// through `VP8LBitWriterReset`: the pending bits, the cursor and the error
/// flag. C's copy of the buffer pointer is never read, so the snapshot holds
/// no buffer (Rust-only).
fn bit_writer_snapshot(bw: &VP8LBitWriter) -> VP8LBitWriter {
    VP8LBitWriter {
        bits: bw.bits,
        used: bw.used,
        buf: Vec::new(),
        cur: bw.cur,
        error: bw.error,
    }
}

// upstream: src/enc/vp8l_enc.c EncodeImageInternal
/// `pic` and `percent` are for progress.
///
/// # Errors
///
/// Returns the error that `pic` holds at the end, as C returns
/// `pic->error_code == VP8_ENC_OK`.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of EncodeImageInternal has seventeen parameters"
)]
#[expect(
    clippy::too_many_lines,
    reason = "the C function EncodeImageInternal is translated as one function"
)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the C conversions of the byte counts to int and back"
)]
fn encode_image_internal(
    bw: &mut VP8LBitWriter,
    argb: &[u32],
    hash_chain: &mut VP8LHashChain,
    refs_array: &mut [VP8LBackwardRefs; 4],
    width: i32,
    height: i32,
    quality: i32,
    low_effort: bool,
    config: &CrunchConfig,
    cache_bits: &mut i32,
    histogram_bits_in: i32,
    init_byte_position: usize,
    hdr_size: &mut i32,
    data_size: &mut i32,
    pic: &mut WebPPicture<'_>,
    mut percent_range: i32,
    percent: &mut i32,
) -> Result<(), WebPEncodingError> {
    let histogram_image_xysize = sub_sample_size(width, histogram_bits_in)
        .cast_unsigned()
        .wrapping_mul(sub_sample_size(height, histogram_bits_in).cast_unsigned());
    let mut remaining_percent = percent_range;
    let mut percent_start = *percent;
    let huff_tree = safe_buffer::<HuffmanTree>(3 * CODE_LENGTH_CODES as u64, HuffmanTree::C_SIZE);
    let histogram_argb = safe_buffer::<u32>(u64::from(histogram_image_xysize), 4);
    let bw_init = bit_writer_snapshot(bw);
    let mut bw_best = VP8LBitWriter::default();
    // histogram image hash chain
    let mut hash_chain_histogram = VP8LHashChain::default();
    let mut bw_size_best: usize = !0;
    debug_assert!(histogram_bits_in >= MIN_HUFFMAN_BITS);
    debug_assert!(histogram_bits_in <= MAX_HUFFMAN_BITS);

    'error: {
        if !vp8l_bit_writer_init(&mut bw_best, 0) {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        // Make sure we can allocate the different objects.
        let (Some(mut huff_tree), Some(mut histogram_argb)) = (huff_tree, histogram_argb) else {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        };
        if !vp8l_hash_chain_init(&mut hash_chain_histogram, histogram_image_xysize as i32) {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        percent_range = remaining_percent / 5;
        if vp8l_hash_chain_fill(
            hash_chain,
            quality,
            argb,
            width,
            height,
            low_effort,
            pic,
            percent_range,
            percent,
        )
        .is_err()
        {
            break 'error;
        }
        percent_start += percent_range;
        remaining_percent -= percent_range;

        // If the value is different from zero, it has been set during the palette
        // analysis.
        let cache_bits_init = if *cache_bits == 0 {
            MAX_COLOR_CACHE_BITS
        } else {
            *cache_bits
        };
        // If several iterations will happen, clone into bw_best.
        if (config.sub_configs_size > 1 || config.sub_configs[0].do_no_cache)
            && !vp8l_bit_writer_clone(bw, &mut bw_best)
        {
            webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        for sub_config in &config.sub_configs[..to_index(config.sub_configs_size)] {
            let mut cache_bits_best: i32 = 0;
            let mut i_remaining_percent = remaining_percent / config.sub_configs_size;
            let mut i_percent_range = i_remaining_percent / 4;
            i_remaining_percent -= i_percent_range;

            if vp8l_get_backward_references(
                width,
                height,
                argb,
                quality,
                low_effort,
                sub_config.lz77,
                cache_bits_init,
                sub_config.do_no_cache,
                hash_chain,
                refs_array,
                &mut cache_bits_best,
                pic,
                i_percent_range,
                percent,
            )
            .is_err()
            {
                break 'error;
            }

            for i_cache in 0..(if sub_config.do_no_cache { 2 } else { 1 }) {
                let cache_bits_tmp = if i_cache == 0 { cache_bits_best } else { 0 };
                let mut histogram_bits = histogram_bits_in;
                // Speed-up: no need to study the no-cache case if it was already studied
                // in i_cache == 0.
                if i_cache == 1 && cache_bits_best == 0 {
                    break;
                }

                // Reset the bit writer for this iteration.
                vp8l_bit_writer_reset(&bw_init, bw);

                // Build histogram image and symbols from backward references.
                let histogram_image =
                    vp8l_allocate_histogram_set(histogram_image_xysize as i32, cache_bits_tmp);
                let tmp_histo = vp8l_allocate_histogram(cache_bits_tmp);
                let (Some(mut histogram_image), Some(mut tmp_histo)) = (histogram_image, tmp_histo)
                else {
                    webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
                    break 'error;
                };

                i_percent_range = i_remaining_percent / 3;
                i_remaining_percent -= i_percent_range;
                if vp8l_get_histo_image_symbols(
                    width,
                    height,
                    &refs_array[i_cache],
                    quality,
                    low_effort,
                    histogram_bits,
                    cache_bits_tmp,
                    &mut histogram_image,
                    &mut tmp_histo,
                    &mut histogram_argb,
                    pic,
                    i_percent_range,
                    percent,
                )
                .is_err()
                {
                    break 'error;
                }
                // Create Huffman bit lengths and codes for each histogram image.
                let histogram_image_size = histogram_image.size;
                let bit_array_size = 5 * to_index(histogram_image_size);
                let huffman_codes =
                    if safe_malloc_test(bit_array_size as u64, HuffmanTreeCode::C_SIZE) {
                        try_filled(bit_array_size, HuffmanTreeCode::default())
                    } else {
                        None
                    };
                // Note: some histogram_image entries may point to tmp_histos[], so the
                // latter need to outlive the following call to
                // GetHuffBitLengthsAndCodes().
                let Some(mut huffman_codes) = huffman_codes.filter(|_| true) else {
                    webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
                    break 'error;
                };
                if !get_huff_bit_lengths_and_codes(&mut histogram_image, &mut huffman_codes) {
                    webp_encoding_set_error(pic, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
                    break 'error;
                }
                // Free combined histograms.
                drop(histogram_image);

                // Free scratch histograms.
                drop(tmp_histo);

                // Color Cache parameters.
                if cache_bits_tmp > 0 {
                    vp8l_put_bits(bw, 1, 1);
                    vp8l_put_bits(bw, cache_bits_tmp.cast_unsigned(), 4);
                } else {
                    vp8l_put_bits(bw, 0, 1);
                }

                // Huffman image + meta huffman.
                let mut histogram_image_size: u32 = 0;
                for symbol in &mut histogram_argb[..histogram_image_xysize as usize] {
                    if *symbol >= histogram_image_size {
                        histogram_image_size = *symbol + 1;
                    }
                    *symbol <<= 8;
                }

                let write_histogram_image = histogram_image_size > 1;
                vp8l_put_bits(bw, bit(write_histogram_image), 1);
                if write_histogram_image {
                    vp8l_optimize_sampling(
                        &mut histogram_argb,
                        width,
                        height,
                        histogram_bits_in,
                        MAX_HUFFMAN_BITS,
                        &mut histogram_bits,
                    );
                    vp8l_put_bits(bw, (histogram_bits - 2).cast_unsigned(), 3);
                    i_percent_range = i_remaining_percent / 2;
                    i_remaining_percent -= i_percent_range;
                    if encode_image_no_huffman(
                        bw,
                        &histogram_argb,
                        &mut hash_chain_histogram,
                        &mut refs_array[2..],
                        sub_sample_size(width, histogram_bits),
                        sub_sample_size(height, histogram_bits),
                        quality,
                        low_effort,
                        pic,
                        i_percent_range,
                        percent,
                    )
                    .is_err()
                    {
                        break 'error;
                    }
                }

                // Store Huffman codes.
                {
                    let mut max_tokens: i32 = 0;
                    let count = 5 * histogram_image_size as usize;
                    // Find maximum number of symbols for the huffman tree-set.
                    for codes in &huffman_codes[..count] {
                        if max_tokens < codes.num_symbols {
                            max_tokens = codes.num_symbols;
                        }
                    }
                    let Some(mut tokens) = safe_buffer::<HuffmanTreeToken>(
                        i64::from(max_tokens).cast_unsigned(),
                        HuffmanTreeToken::C_SIZE,
                    ) else {
                        webp_encoding_set_error(
                            pic,
                            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
                        );
                        break 'error;
                    };
                    for codes in &mut huffman_codes[..count] {
                        store_huffman_code(bw, &mut huff_tree, &mut tokens, codes);
                        clear_huffman_tree_if_only_one_symbol(codes);
                    }
                }
                // Store actual literals.
                let hdr_size_tmp = (vp8l_bit_writer_num_bytes(bw) - init_byte_position) as i32;
                if store_image_to_bit_mask(
                    bw,
                    width,
                    histogram_bits,
                    &refs_array[i_cache],
                    &histogram_argb,
                    &huffman_codes,
                    pic,
                )
                .is_err()
                {
                    break 'error;
                }
                // Keep track of the smallest image so far.
                if vp8l_bit_writer_num_bytes(bw) < bw_size_best {
                    bw_size_best = vp8l_bit_writer_num_bytes(bw);
                    *cache_bits = cache_bits_tmp;
                    *hdr_size = hdr_size_tmp;
                    *data_size = (vp8l_bit_writer_num_bytes(bw)
                        - init_byte_position
                        - *hdr_size as usize) as i32;
                    vp8l_bit_writer_swap(bw, &mut bw_best);
                }
            }
        }
        vp8l_bit_writer_swap(bw, &mut bw_best);

        let _ = webp_report_progress(pic, percent_start + remaining_percent, Some(percent));
    }

    vp8l_hash_chain_clear(&mut hash_chain_histogram);
    vp8l_bit_writer_wipe_out(&mut bw_best);
    picture_status(pic)
}

// -----------------------------------------------------------------------------
// Transforms

// upstream: src/enc/vp8l_enc.c ApplySubtractGreen
fn apply_subtract_green(enc: &mut VP8LEncoder, width: i32, height: i32, bw: &mut VP8LBitWriter) {
    vp8l_put_bits(bw, TRANSFORM_PRESENT.cast_unsigned(), 1);
    vp8l_put_bits(bw, VP8LImageTransformType::SUBTRACT_GREEN_TRANSFORM.0, 2);
    let (argb, _, _) = enc.transform_regions();
    vp8l_subtract_green_from_blue_and_red(argb, width * height);
}

// upstream: src/enc/vp8l_enc.c ApplyPredictFilter
/// `config` and `pic` are C's `enc->config` and `enc->pic`. Returns `false`
/// where C returns 0, which happens without a recorded error when
/// `VP8LResidualImage` cannot allocate its modes.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of ApplyPredictFilter has ten parameters"
)]
fn apply_predict_filter(
    enc: &mut VP8LEncoder,
    config: &WebPConfig,
    pic: &mut WebPPicture<'_>,
    width: i32,
    height: i32,
    quality: i32,
    low_effort: bool,
    used_subtract_green: bool,
    bw: &mut VP8LBitWriter,
    percent_range: i32,
    percent: &mut i32,
    best_bits: &mut i32,
) -> bool {
    let near_lossless_strength = if enc.use_palette {
        100
    } else {
        config.near_lossless
    };
    let max_bits = clamp_bits(
        width,
        height,
        enc.predictor_transform_bits,
        MIN_TRANSFORM_BITS,
        MAX_TRANSFORM_BITS,
        MAX_PREDICTOR_IMAGE_SIZE,
    );
    let min_bits = clamp_bits(
        width,
        height,
        max_bits
            - 2 * (if config.method > 4 {
                config.method - 4
            } else {
                0
            }),
        MIN_TRANSFORM_BITS,
        MAX_TRANSFORM_BITS,
        MAX_PREDICTOR_IMAGE_SIZE,
    );

    {
        let (argb, argb_scratch, transform_data) = enc.transform_regions();
        if !vp8l_residual_image(
            width,
            height,
            min_bits,
            max_bits,
            low_effort,
            argb,
            argb_scratch,
            transform_data,
            near_lossless_strength,
            config.exact != 0,
            used_subtract_green,
            pic,
            percent_range / 2,
            percent,
            best_bits,
        ) {
            return false;
        }
    }
    vp8l_put_bits(bw, TRANSFORM_PRESENT.cast_unsigned(), 1);
    vp8l_put_bits(bw, VP8LImageTransformType::PREDICTOR_TRANSFORM.0, 2);
    debug_assert!(*best_bits >= MIN_TRANSFORM_BITS && *best_bits <= MAX_TRANSFORM_BITS);
    vp8l_put_bits(
        bw,
        (*best_bits - MIN_TRANSFORM_BITS).cast_unsigned(),
        NUM_TRANSFORM_BITS,
    );
    let VP8LEncoder {
        transform_mem,
        transform_data,
        hash_chain,
        refs,
        ..
    } = enc;
    encode_image_no_huffman(
        bw,
        &transform_mem[*transform_data..],
        hash_chain,
        refs,
        sub_sample_size(width, *best_bits),
        sub_sample_size(height, *best_bits),
        quality,
        low_effort,
        pic,
        percent_range - percent_range / 2,
        percent,
    )
    .is_ok()
}

// upstream: src/enc/vp8l_enc.c ApplyCrossColorFilter
/// `pic` is C's `enc->pic`.
///
/// # Errors
///
/// Returns the error that `pic` records.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of ApplyCrossColorFilter has nine parameters"
)]
fn apply_cross_color_filter(
    enc: &mut VP8LEncoder,
    pic: &mut WebPPicture<'_>,
    width: i32,
    height: i32,
    quality: i32,
    low_effort: bool,
    bw: &mut VP8LBitWriter,
    percent_range: i32,
    percent: &mut i32,
    best_bits: &mut i32,
) -> Result<(), WebPEncodingError> {
    let min_bits = enc.cross_color_transform_bits;

    {
        let (argb, _, transform_data) = enc.transform_regions();
        vp8l_color_space_transform(
            width,
            height,
            min_bits,
            quality,
            argb,
            transform_data,
            pic,
            percent_range / 2,
            percent,
            best_bits,
        )?;
    }
    vp8l_put_bits(bw, TRANSFORM_PRESENT.cast_unsigned(), 1);
    vp8l_put_bits(bw, VP8LImageTransformType::CROSS_COLOR_TRANSFORM.0, 2);
    debug_assert!(*best_bits >= MIN_TRANSFORM_BITS && *best_bits <= MAX_TRANSFORM_BITS);
    vp8l_put_bits(
        bw,
        (*best_bits - MIN_TRANSFORM_BITS).cast_unsigned(),
        NUM_TRANSFORM_BITS,
    );
    let VP8LEncoder {
        transform_mem,
        transform_data,
        hash_chain,
        refs,
        ..
    } = enc;
    encode_image_no_huffman(
        bw,
        &transform_mem[*transform_data..],
        hash_chain,
        refs,
        sub_sample_size(width, *best_bits),
        sub_sample_size(height, *best_bits),
        quality,
        low_effort,
        pic,
        percent_range - percent_range / 2,
        percent,
    )
}

// -----------------------------------------------------------------------------

/// Calls the picture's writer, which C calls through `pic->writer`.
///
/// # Panics
///
/// Panics when the picture has no writer, where C calls a `NULL` pointer.
fn write(pic: &mut WebPPicture<'_>, data: &[u8]) -> bool {
    let writer = pic
        .writer
        .as_mut()
        .expect("the picture has a writer, which C calls unconditionally");
    writer(data)
}

// upstream: src/enc/vp8l_enc.c WriteRiffHeader
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversions of the sizes to uint32_t and of the magic byte"
)]
fn write_riff_header(pic: &mut WebPPicture<'_>, riff_size: usize, vp8l_size: usize) -> bool {
    const LEN: usize = (RIFF_HEADER_SIZE + CHUNK_HEADER_SIZE + VP8L_SIGNATURE_SIZE) as usize;
    let mut riff: [u8; LEN] = [
        b'R',
        b'I',
        b'F',
        b'F',
        0,
        0,
        0,
        0,
        b'W',
        b'E',
        b'B',
        b'P',
        b'V',
        b'P',
        b'8',
        b'L',
        0,
        0,
        0,
        0,
        VP8L_MAGIC_BYTE as u8,
    ];
    put_le32(&mut riff[TAG_SIZE as usize..], riff_size as u32);
    put_le32(
        &mut riff[(RIFF_HEADER_SIZE + TAG_SIZE) as usize..],
        vp8l_size as u32,
    );
    write(pic, &riff)
}

// upstream: src/enc/vp8l_enc.c WriteImageSize
fn write_image_size(pic: &WebPPicture<'_>, bw: &mut VP8LBitWriter) -> bool {
    let width = pic.width - 1;
    let height = pic.height - 1;
    debug_assert!(width < WEBP_MAX_DIMENSION && height < WEBP_MAX_DIMENSION);

    vp8l_put_bits(bw, width.cast_unsigned(), VP8L_IMAGE_SIZE_BITS);
    vp8l_put_bits(bw, height.cast_unsigned(), VP8L_IMAGE_SIZE_BITS);
    !bw.error
}

// upstream: src/enc/vp8l_enc.c WriteRealAlphaAndVersion
fn write_real_alpha_and_version(bw: &mut VP8LBitWriter, has_alpha: bool) -> bool {
    vp8l_put_bits(bw, bit(has_alpha), 1);
    vp8l_put_bits(bw, VP8L_VERSION.cast_unsigned(), VP8L_VERSION_BITS);
    !bw.error
}

// upstream: src/enc/vp8l_enc.c WriteImage
/// # Errors
///
/// Returns the error that `pic` records: a failed bit writer or writer.
fn write_image(
    pic: &mut WebPPicture<'_>,
    bw: &mut VP8LBitWriter,
    coded_size: &mut usize,
) -> Result<(), WebPEncodingError> {
    let _ = vp8l_bit_writer_finish(bw);
    let webpll_size = vp8l_bit_writer_num_bytes(bw);
    let vp8l_size = VP8L_SIGNATURE_SIZE as usize + webpll_size;
    let pad = vp8l_size & 1;
    let riff_size = TAG_SIZE as usize + CHUNK_HEADER_SIZE as usize + vp8l_size + pad;
    *coded_size = 0;

    if bw.error {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }

    if !write_riff_header(pic, riff_size, vp8l_size) || !write(pic, &bw.buf[..webpll_size]) {
        return Err(webp_encoding_set_error(
            pic,
            WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE,
        ));
    }

    if pad != 0 {
        let pad_byte: [u8; 1] = [0];
        if !write(pic, &pad_byte) {
            return Err(webp_encoding_set_error(
                pic,
                WebPEncodingError::VP8_ENC_ERROR_BAD_WRITE,
            ));
        }
    }
    *coded_size = CHUNK_HEADER_SIZE as usize + riff_size;
    Ok(())
}

// -----------------------------------------------------------------------------

// upstream: src/enc/vp8l_enc.c ClearTransformBuffer
fn clear_transform_buffer(enc: &mut VP8LEncoder) {
    enc.transform_mem = Vec::new();
    enc.transform_mem_size = 0;
}

/// `WEBP_ALIGN` of a position in words of an aligned allocation: the next
/// multiple of 32 bytes. The C code aligns addresses; the positions differ from
/// C's only by the alignment of the allocation, which is not observable.
fn align_words(position: u64) -> u64 {
    let words = ((WEBP_ALIGN_WORDS) - 1) as u64;
    (position + words) & !words
}

/// The words of `WEBP_ALIGN_CST + 1` bytes.
const WEBP_ALIGN_WORDS: usize = (crate::utils::utils::WEBP_ALIGN_CST + 1) / 4;

// upstream: src/enc/vp8l_enc.c AllocateTransformBuffer
/// Allocates the memory for argb (W x H) buffer, 2 rows of context for
/// prediction and transform data.
/// Flags influencing the memory allocated:
///  `enc.transform_bits`
///  `enc.use_predict`, `enc.use_cross_color`
///
/// # Errors
///
/// Returns the error that `pic` records after a failed allocation.
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversions of the sizes to size_t, which the allocation checked"
)]
fn allocate_transform_buffer(
    enc: &mut VP8LEncoder,
    pic: &mut WebPPicture<'_>,
    width: i32,
    height: i32,
) -> Result<(), WebPEncodingError> {
    let image_size = i64::from(width)
        .cast_unsigned()
        .wrapping_mul(i64::from(height).cast_unsigned());
    // VP8LResidualImage needs room for 2 scanlines of uint32 pixels with an extra
    // pixel in each, plus 2 regular scanlines of bytes.
    // TODO(skal): Clean up by using arithmetic in bytes instead of words.
    let argb_scratch_size: u64 = if enc.use_predict {
        i64::from((width + 1) * 2).cast_unsigned().wrapping_add(
            (i64::from(width * 2).cast_unsigned() + size_of::<u32>() as u64 - 1)
                / size_of::<u32>() as u64,
        )
    } else {
        0
    };
    let transform_data_size: u64 = if enc.use_predict || enc.use_cross_color {
        u64::from(sub_sample_size(width, MIN_TRANSFORM_BITS).cast_unsigned())
            * u64::from(sub_sample_size(height, MIN_TRANSFORM_BITS).cast_unsigned())
    } else {
        0
    };
    let max_alignment_in_words: u64 =
        (crate::utils::utils::WEBP_ALIGN_CST as u64 + size_of::<u32>() as u64 - 1)
            / size_of::<u32>() as u64;
    let mem_size = image_size
        .wrapping_add(max_alignment_in_words)
        .wrapping_add(argb_scratch_size)
        .wrapping_add(max_alignment_in_words)
        .wrapping_add(transform_data_size);
    if enc.transform_mem.is_empty() || mem_size > enc.transform_mem_size as u64 {
        clear_transform_buffer(enc);
        let Some(mem) = safe_buffer::<u32>(mem_size, size_of::<u32>()) else {
            return Err(webp_encoding_set_error(
                pic,
                WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
            ));
        };
        enc.transform_mem = mem;
        enc.transform_mem_size = mem_size as usize;
        enc.argb_content = VP8LEncoderARGBContent::kEncoderNone;
    }
    enc.argb = 0;
    let argb_scratch = align_words(image_size);
    enc.argb_scratch = argb_scratch as usize;
    enc.transform_data = align_words(argb_scratch + argb_scratch_size) as usize;

    enc.current_width = width;
    Ok(())
}

// upstream: src/enc/vp8l_enc.c MakeInputImageCopy
/// # Errors
///
/// Returns the error that `pic` records after a failed allocation.
fn make_input_image_copy(
    enc: &mut VP8LEncoder,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let width = pic.width;
    let height = pic.height;

    allocate_transform_buffer(enc, pic, width, height)?;
    if enc.argb_content == VP8LEncoderARGBContent::kEncoderARGB {
        return Ok(());
    }

    {
        let src = picture_argb(pic);
        let (dst, _, _) = enc.transform_regions();
        let w = to_index(width);
        let stride = to_index(pic.argb_stride);
        for y in 0..to_index(height) {
            dst[y * w..(y + 1) * w].copy_from_slice(&src[y * stride..y * stride + w]);
        }
    }
    enc.argb_content = VP8LEncoderARGBContent::kEncoderARGB;
    debug_assert_eq!(enc.current_width, width);
    Ok(())
}

// -----------------------------------------------------------------------------

// upstream: src/enc/vp8l_enc.c APPLY_PALETTE_GREEDY_MAX
const APPLY_PALETTE_GREEDY_MAX: i32 = 4;

// upstream: src/enc/vp8l_enc.c SearchColorGreedy
#[inline]
fn search_color_greedy(palette: &[u32], palette_size: i32, color: u32) -> u32 {
    const _: () = assert!(3 == APPLY_PALETTE_GREEDY_MAX - 1);
    let _ = palette_size;
    debug_assert!(palette_size < APPLY_PALETTE_GREEDY_MAX);
    if color == palette[0] {
        return 0;
    }
    if color == palette[1] {
        return 1;
    }
    if color == palette[2] {
        return 2;
    }
    3
}

// upstream: src/enc/vp8l_enc.c ApplyPaletteHash0
#[inline]
fn apply_palette_hash0(color: u32) -> u32 {
    // Focus on the green color.
    (color >> 8) & 0xff
}

// upstream: src/enc/vp8l_enc.c PALETTE_INV_SIZE_BITS
const PALETTE_INV_SIZE_BITS: i32 = 11;
// upstream: src/enc/vp8l_enc.c PALETTE_INV_SIZE
const PALETTE_INV_SIZE: i32 = 1 << PALETTE_INV_SIZE_BITS;

// upstream: src/enc/vp8l_enc.c ApplyPaletteHash1
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C (uint32_t) cast of the 64-bit product"
)]
fn apply_palette_hash1(color: u32) -> u32 {
    // Forget about alpha.
    ((u64::from(color & 0x00ff_ffff) * 4_222_244_071) as u32) >> (32 - PALETTE_INV_SIZE_BITS)
}

// upstream: src/enc/vp8l_enc.c ApplyPaletteHash2
#[inline]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C (uint32_t) cast of the 64-bit product"
)]
fn apply_palette_hash2(color: u32) -> u32 {
    // Forget about alpha.
    ((u64::from(color & 0x00ff_ffff) * ((1_u64 << 31) - 1)) as u32) >> (32 - PALETTE_INV_SIZE_BITS)
}

// upstream: src/enc/vp8l_enc.c APPLY_PALETTE_FOR
/// Use 1 pixel cache for ARGB pixels. The C macro becomes a function over the
/// color-to-index expression.
#[expect(
    clippy::too_many_arguments,
    reason = "the C macro reads nine variables of ApplyPalette"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion of the palette index to uint8_t"
)]
fn apply_palette_for(
    src: &[u32],
    src_stride: u32,
    dst: &mut [u32],
    dst_stride: u32,
    first_color: u32,
    width: i32,
    height: i32,
    xbits: i32,
    tmp_row: &mut [u8],
    color_index: impl Fn(u32) -> u32,
) {
    let mut prev_pix = first_color;
    let mut prev_idx: u32 = 0;
    let mut src_pos: usize = 0;
    let mut dst_pos: usize = 0;
    for _y in 0..height {
        for x in 0..to_index(width) {
            let pix = src[src_pos + x];
            if pix != prev_pix {
                prev_idx = color_index(pix);
                prev_pix = pix;
            }
            tmp_row[x] = prev_idx as u8;
        }
        vp8l_bundle_color_map(tmp_row, width, xbits, &mut dst[dst_pos..]);
        src_pos += src_stride as usize;
        dst_pos += dst_stride as usize;
    }
}

// upstream: src/enc/vp8l_enc.c ApplyPalette
/// Remap argb values in `src[]` to packed palettes entries in `dst[]`
/// using `row` as a temporary buffer of size `width`.
/// We assume that all `src[]` values have a corresponding entry in the palette.
/// `src` and `dst` are distinct here, as at the only call. C records
/// `VP8_ENC_ERROR_OUT_OF_MEMORY` on the picture when the row cannot be
/// allocated; this returns that code, and the caller records it.
#[expect(
    clippy::too_many_arguments,
    reason = "the C signature of ApplyPalette has ten parameters"
)]
#[expect(
    clippy::cast_possible_truncation,
    reason = "the C conversion of the palette position to uint16_t"
)]
fn apply_palette(
    src: &[u32],
    src_stride: u32,
    dst: &mut [u32],
    dst_stride: u32,
    palette: &[u32],
    palette_size: i32,
    width: i32,
    height: i32,
    xbits: i32,
) -> Result<(), WebPEncodingError> {
    // TODO(skal): this tmp buffer is not needed if VP8LBundleColorMap() can be
    // made to work in-place.
    let Some(mut tmp_row) = safe_buffer::<u8>(i64::from(width).cast_unsigned(), 1) else {
        return Err(WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
    };

    if palette_size < APPLY_PALETTE_GREEDY_MAX {
        apply_palette_for(
            src,
            src_stride,
            dst,
            dst_stride,
            palette[0],
            width,
            height,
            xbits,
            &mut tmp_row,
            |pix| search_color_greedy(palette, palette_size, pix),
        );
    } else {
        let mut buffer = [0_u16; PALETTE_INV_SIZE as usize];
        let hash_functions: [fn(u32) -> u32; 3] = [
            apply_palette_hash0,
            apply_palette_hash1,
            apply_palette_hash2,
        ];

        // Try to find a perfect hash function able to go from a color to an index
        // within 1 << PALETTE_INV_SIZE_BITS in order to build a hash map to go
        // from color to index in palette.
        let mut i = 0;
        while i < 3 {
            let mut use_lut = true;
            // Set each element in buffer to max uint16_t.
            buffer.fill(0xffff);
            for (j, &color) in palette[..to_index(palette_size)].iter().enumerate() {
                let ind = hash_functions[i](color) as usize;
                if buffer[ind] != 0xffff {
                    use_lut = false;
                    break;
                }
                buffer[ind] = j as u16;
            }
            if use_lut {
                break;
            }
            i += 1;
        }

        let first = palette[0];
        let mut apply = |color_index: &dyn Fn(u32) -> u32| {
            apply_palette_for(
                src,
                src_stride,
                dst,
                dst_stride,
                first,
                width,
                height,
                xbits,
                &mut tmp_row,
                color_index,
            );
        };
        if i == 0 {
            apply(&|pix| u32::from(buffer[apply_palette_hash0(pix) as usize]));
        } else if i == 1 {
            apply(&|pix| u32::from(buffer[apply_palette_hash1(pix) as usize]));
        } else if i == 2 {
            apply(&|pix| u32::from(buffer[apply_palette_hash2(pix) as usize]));
        } else {
            let mut idx_map = [0_u32; MAX_PALETTE_SIZE as usize];
            let mut palette_sorted = [0_u32; MAX_PALETTE_SIZE as usize];
            prepare_map_to_palette(
                palette,
                palette_size.cast_unsigned(),
                &mut palette_sorted,
                &mut idx_map,
            );
            apply(&|pix| {
                idx_map[to_index(search_color_no_idx(&palette_sorted, pix, palette_size))]
            });
        }
    }
    Ok(())
}

// upstream: src/enc/vp8l_enc.c MapImageFromPalette
/// Note: Expects `enc.palette` to be set properly. `pic` is C's `enc->pic`.
///
/// # Errors
///
/// Returns the error that `pic` records after a failed allocation.
#[expect(
    clippy::bool_to_int_with_if,
    reason = "the C conditional expression of xbits"
)]
fn map_image_from_palette(
    enc: &mut VP8LEncoder,
    pic: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let width = pic.width;
    let height = pic.height;
    let palette_size = enc.palette_size;

    // Replace each input pixel by corresponding palette index.
    // This is done line by line.
    let xbits = if palette_size <= 4 {
        if palette_size <= 2 { 3 } else { 2 }
    } else if palette_size <= 16 {
        1
    } else {
        0
    };

    allocate_transform_buffer(enc, pic, sub_sample_size(width, xbits), height)?;
    let palette = enc.palette;
    let current_width = enc.current_width.cast_unsigned();
    let result = {
        let (argb, _, _) = enc.transform_regions();
        apply_palette(
            picture_argb(pic),
            pic.argb_stride.cast_unsigned(),
            argb,
            current_width,
            &palette,
            palette_size,
            width,
            height,
            xbits,
        )
    };
    if let Err(error) = result {
        return Err(webp_encoding_set_error(pic, error));
    }
    enc.argb_content = VP8LEncoderARGBContent::kEncoderPalette;
    Ok(())
}

// upstream: src/enc/vp8l_enc.c EncodePalette
/// Save `palette[]` to bitstream. `pic` is C's `enc->pic`.
///
/// # Errors
///
/// Returns the error that `pic` holds after encoding the palette.
fn encode_palette(
    bw: &mut VP8LBitWriter,
    low_effort: bool,
    enc: &mut VP8LEncoder,
    pic: &mut WebPPicture<'_>,
    percent_range: i32,
    percent: &mut i32,
) -> Result<(), WebPEncodingError> {
    let mut tmp_palette = [0_u32; MAX_PALETTE_SIZE as usize];
    let palette_size = enc.palette_size;
    let palette = &enc.palette;
    // If the last element is 0, do not store it and count on automatic palette
    // 0-filling. This can only happen if there is no pixel packing, hence if
    // there are strictly more than 16 colors (after 0 is removed).
    let encoded_palette_size: u32 = if palette[to_index(palette_size - 1)] == 0 && palette_size > 17
    {
        (palette_size - 1).cast_unsigned()
    } else {
        palette_size.cast_unsigned()
    };
    vp8l_put_bits(bw, TRANSFORM_PRESENT.cast_unsigned(), 1);
    vp8l_put_bits(bw, VP8LImageTransformType::COLOR_INDEXING_TRANSFORM.0, 2);
    debug_assert!((1..=MAX_PALETTE_SIZE).contains(&palette_size));
    vp8l_put_bits(bw, encoded_palette_size - 1, 8);
    for i in (1..encoded_palette_size as usize).rev() {
        tmp_palette[i] = vp8l_sub_pixels(palette[i], palette[i - 1]);
    }
    tmp_palette[0] = palette[0];
    encode_image_no_huffman(
        bw,
        &tmp_palette,
        &mut enc.hash_chain,
        &mut enc.refs,
        encoded_palette_size.cast_signed(),
        1,
        /*quality=*/ 20,
        low_effort,
        pic,
        percent_range,
        percent,
    )
}

// -----------------------------------------------------------------------------
// VP8LEncoder

// upstream: src/enc/vp8l_enc.c VP8LEncoderNew
/// A new encoder with no ARGB content. C also stores the configuration and
/// the picture, which the Rust encoder takes per call, and calls
/// `VP8LEncDspInit`, which only assigns portable pointers (ADR 0003). C's
/// failed allocation of the fixed-size encoder records
/// `VP8_ENC_ERROR_OUT_OF_MEMORY`; the Rust encoder is a value.
fn vp8l_encoder_new() -> VP8LEncoder {
    VP8LEncoder {
        argb_content: VP8LEncoderARGBContent::kEncoderNone,
        ..VP8LEncoder::default()
    }
}

// -----------------------------------------------------------------------------
// Main call

// upstream: src/enc/vp8l_enc.c StreamEncodeContext
/// The parameters of one encoding context. C's `picture`, `bw`, `enc` and
/// `stats` pointers are borrows.
struct StreamEncodeContext<'c, 'p> {
    config: &'c WebPConfig,
    picture: &'c mut WebPPicture<'p>,
    bw: &'c mut VP8LBitWriter,
    enc: &'c mut VP8LEncoder,
    crunch_configs: [CrunchConfig; CRUNCH_CONFIGS_MAX],
    num_crunch_configs: i32,
    red_and_blue_always_zero: bool,
    stats: Option<&'c mut WebPAuxStats>,
}

// upstream: src/enc/vp8l_enc.c EncodeStreamHook
/// The worker hook of an encoding context. The hook should return false in
/// case of error.
#[expect(
    clippy::too_many_lines,
    reason = "the C function EncodeStreamHook is translated as one function"
)]
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "the C conversions of the quality and the sizes to int"
)]
fn encode_stream_hook(params: &mut StreamEncodeContext<'_, '_>) -> bool {
    let config = params.config;
    let picture = &mut *params.picture;
    let bw = &mut *params.bw;
    let enc = &mut *params.enc;
    let crunch_configs = &params.crunch_configs;
    let num_crunch_configs = params.num_crunch_configs;
    let red_and_blue_always_zero = params.red_and_blue_always_zero;
    let stats = &mut params.stats;
    let quality = config.quality as i32;
    let low_effort = config.method == 0;
    let width = picture.width;
    let height = picture.height;
    let byte_position = vp8l_bit_writer_num_bytes(bw);
    let mut percent: i32 = 2; // for WebPProgressHook
    let mut hdr_size: i32 = 0;
    let mut data_size: i32 = 0;
    let mut best_size: usize = !0;
    let bw_init = bit_writer_snapshot(bw);
    let mut bw_best = VP8LBitWriter::default();

    'error: {
        if !vp8l_bit_writer_init(&mut bw_best, 0)
            || (num_crunch_configs > 1 && !vp8l_bit_writer_clone(bw, &mut bw_best))
        {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        for crunch_config in &crunch_configs[..to_index(num_crunch_configs)] {
            let entropy_idx = crunch_config.entropy_idx;
            let mut remaining_percent = 97 / num_crunch_configs;
            let mut percent_range;
            let mut predictor_transform_bits: i32 = 0;
            let mut cross_color_transform_bits: i32 = 0;
            enc.use_palette = entropy_idx == EntropyIx::kPalette.int()
                || entropy_idx == EntropyIx::kPaletteAndSpatial.int();
            enc.use_subtract_green = entropy_idx == EntropyIx::kSubGreen.int()
                || entropy_idx == EntropyIx::kSpatialSubGreen.int();
            enc.use_predict = entropy_idx == EntropyIx::kSpatial.int()
                || entropy_idx == EntropyIx::kSpatialSubGreen.int()
                || entropy_idx == EntropyIx::kPaletteAndSpatial.int();
            // When using a palette, R/B==0, hence no need to test for cross-color.
            if low_effort || enc.use_palette {
                enc.use_cross_color = false;
            } else {
                enc.use_cross_color = if red_and_blue_always_zero {
                    false
                } else {
                    enc.use_predict
                };
            }
            // Reset any parameter in the encoder that is set in the previous iteration.
            enc.cache_bits = 0;
            vp8l_backward_refs_clear(&mut enc.refs[0]);
            vp8l_backward_refs_clear(&mut enc.refs[1]);

            // Apply near-lossless preprocessing.
            let use_near_lossless =
                (config.near_lossless < 100) && !enc.use_palette && !enc.use_predict;
            if use_near_lossless {
                if allocate_transform_buffer(enc, picture, width, height).is_err() {
                    break 'error;
                }
                if enc.argb_content != VP8LEncoderARGBContent::kEncoderNearLossless {
                    let (argb, _, _) = enc.transform_regions();
                    if !vp8_apply_near_lossless(picture, config.near_lossless, argb) {
                        webp_encoding_set_error(
                            picture,
                            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
                        );
                        break 'error;
                    }
                }
                enc.argb_content = VP8LEncoderARGBContent::kEncoderNearLossless;
            } else {
                enc.argb_content = VP8LEncoderARGBContent::kEncoderNone;
            }

            // Encode palette
            if enc.use_palette {
                if !palette_sort(
                    crunch_config.palette_sorting_type,
                    picture,
                    &enc.palette_sorted,
                    enc.palette_size.cast_unsigned(),
                    &mut enc.palette,
                ) {
                    webp_encoding_set_error(
                        picture,
                        WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
                    );
                    break 'error;
                }
                percent_range = remaining_percent / 4;
                if encode_palette(bw, low_effort, enc, picture, percent_range, &mut percent)
                    .is_err()
                {
                    break 'error;
                }
                remaining_percent -= percent_range;
                if map_image_from_palette(enc, picture).is_err() {
                    break 'error;
                }
                // If using a color cache, do not have it bigger than the number of
                // colors.
                if enc.palette_size < (1 << MAX_COLOR_CACHE_BITS) {
                    enc.cache_bits = bits_log2_floor(enc.palette_size.cast_unsigned()) + 1;
                }
            }
            // In case image is not packed.
            if enc.argb_content != VP8LEncoderARGBContent::kEncoderNearLossless
                && enc.argb_content != VP8LEncoderARGBContent::kEncoderPalette
                && make_input_image_copy(enc, picture).is_err()
            {
                break 'error;
            }

            // -------------------------------------------------------------------------
            // Apply transforms and write transform data.

            if enc.use_subtract_green {
                let current_width = enc.current_width;
                apply_subtract_green(enc, current_width, height, bw);
            }

            if enc.use_predict {
                percent_range = remaining_percent / 3;
                let current_width = enc.current_width;
                let used_subtract_green = enc.use_subtract_green;
                if !apply_predict_filter(
                    enc,
                    config,
                    picture,
                    current_width,
                    height,
                    quality,
                    low_effort,
                    used_subtract_green,
                    bw,
                    percent_range,
                    &mut percent,
                    &mut predictor_transform_bits,
                ) {
                    break 'error;
                }
                remaining_percent -= percent_range;
            }

            if enc.use_cross_color {
                percent_range = remaining_percent / 2;
                let current_width = enc.current_width;
                if apply_cross_color_filter(
                    enc,
                    picture,
                    current_width,
                    height,
                    quality,
                    low_effort,
                    bw,
                    percent_range,
                    &mut percent,
                    &mut cross_color_transform_bits,
                )
                .is_err()
                {
                    break 'error;
                }
                remaining_percent -= percent_range;
            }

            vp8l_put_bits(bw, bit(TRANSFORM_PRESENT == 0), 1); // No more transforms.

            // -------------------------------------------------------------------------
            // Encode and write the transformed image.
            let result = {
                let VP8LEncoder {
                    transform_mem,
                    argb,
                    hash_chain,
                    refs,
                    current_width,
                    cache_bits,
                    histo_bits,
                    ..
                } = &mut *enc;
                encode_image_internal(
                    bw,
                    &transform_mem[*argb..],
                    hash_chain,
                    refs,
                    *current_width,
                    height,
                    quality,
                    low_effort,
                    crunch_config,
                    cache_bits,
                    *histo_bits,
                    byte_position,
                    &mut hdr_size,
                    &mut data_size,
                    picture,
                    remaining_percent,
                    &mut percent,
                )
            };
            if result.is_err() {
                break 'error;
            }

            // If we are better than what we already have.
            if vp8l_bit_writer_num_bytes(bw) < best_size {
                best_size = vp8l_bit_writer_num_bytes(bw);
                // Store the BitWriter.
                vp8l_bit_writer_swap(bw, &mut bw_best);
                // Update the stats.
                if let Some(stats) = stats.as_deref_mut() {
                    stats.lossless_features = 0;
                    if enc.use_predict {
                        stats.lossless_features |= 1;
                    }
                    if enc.use_cross_color {
                        stats.lossless_features |= 2;
                    }
                    if enc.use_subtract_green {
                        stats.lossless_features |= 4;
                    }
                    if enc.use_palette {
                        stats.lossless_features |= 8;
                    }
                    stats.histogram_bits = enc.histo_bits;
                    stats.transform_bits = predictor_transform_bits;
                    stats.cross_color_transform_bits = cross_color_transform_bits;
                    stats.cache_bits = enc.cache_bits;
                    stats.palette_size = enc.palette_size;
                    stats.lossless_size = (best_size - byte_position) as i32;
                    stats.lossless_hdr_size = hdr_size;
                    stats.lossless_data_size = data_size;
                }
            }
            // Reset the bit writer for the following iteration if any.
            if num_crunch_configs > 1 {
                vp8l_bit_writer_reset(&bw_init, bw);
            }
        }
        vp8l_bit_writer_swap(&mut bw_best, bw);
    }

    vp8l_bit_writer_wipe_out(&mut bw_best);
    // The hook should return false in case of error.
    picture.error_code == WebPEncodingError::VP8_ENC_OK
}

/// The fields of a picture view that a thread can receive (Rust-only). A view
/// holds no writer, progress hook, `extra_info` or `stats` (ADR 0003, shared
/// views), so the two callback fields, which are not `Send`, are left out, and
/// every field that remains is `Send`.
struct PictureViewParts<'s> {
    use_argb: i32,
    colorspace: WebPEncCSP,
    width: i32,
    height: i32,
    y: Option<PlaneOffset>,
    u: Option<PlaneOffset>,
    v: Option<PlaneOffset>,
    y_stride: i32,
    uv_stride: i32,
    a: Option<PlaneOffset>,
    a_stride: i32,
    pad1: [u32; 2],
    argb: Option<PlaneOffset>,
    argb_stride: i32,
    pad2: [u32; 3],
    extra_info_type: i32,
    error_code: WebPEncodingError,
    pad3: [u32; 3],
    pad6: [u32; 8],
    yuva: PlaneStorage<'s, u8>,
    argb_storage: PlaneStorage<'s, u32>,
}

impl<'s> PictureViewParts<'s> {
    /// Takes the fields of `view`.
    fn from_view(view: WebPPicture<'s>) -> Self {
        debug_assert!(view.writer.is_none() && view.progress_hook.is_none());
        debug_assert!(view.extra_info.is_none() && view.stats.is_none());
        Self {
            use_argb: view.use_argb,
            colorspace: view.colorspace,
            width: view.width,
            height: view.height,
            y: view.y,
            u: view.u,
            v: view.v,
            y_stride: view.y_stride,
            uv_stride: view.uv_stride,
            a: view.a,
            a_stride: view.a_stride,
            pad1: view.pad1,
            argb: view.argb,
            argb_stride: view.argb_stride,
            pad2: view.pad2,
            extra_info_type: view.extra_info_type,
            error_code: view.error_code,
            pad3: view.pad3,
            pad6: view.pad6,
            yuva: view.yuva,
            argb_storage: view.argb_storage,
        }
    }

    /// The view again, without callbacks, `extra_info` or `stats`.
    fn into_picture(self) -> WebPPicture<'s> {
        WebPPicture {
            use_argb: self.use_argb,
            colorspace: self.colorspace,
            width: self.width,
            height: self.height,
            y: self.y,
            u: self.u,
            v: self.v,
            y_stride: self.y_stride,
            uv_stride: self.uv_stride,
            a: self.a,
            a_stride: self.a_stride,
            pad1: self.pad1,
            argb: self.argb,
            argb_stride: self.argb_stride,
            pad2: self.pad2,
            writer: None,
            extra_info_type: self.extra_info_type,
            extra_info: None,
            stats: None,
            error_code: self.error_code,
            progress_hook: None,
            pad3: self.pad3,
            pad6: self.pad6,
            yuva: self.yuva,
            argb_storage: self.argb_storage,
        }
    }
}

/// What the two contexts of `VP8LEncodeStream` report after their `Sync`
/// (Rust-only): the hook results and the error code of the main context's
/// picture and of the side picture.
struct ContextOutcome {
    ok_main: bool,
    main_error: WebPEncodingError,
    ok_side: bool,
    side_error: WebPEncodingError,
}

// upstream: src/enc/vp8l_enc.c VP8LEncodeStream
/// Encodes the main image stream using the supplied bit writer.
/// Returns false in case of error (stored in `picture.error_code`).
///
/// Threads (ADR 0003, threads; see the `# Deviations from upstream` section of
/// this module, item 1): when `thread_level > 0` splits the configurations, the
/// side context runs on a `std::thread::scope` thread and the main context on
/// the calling thread, as C's `Launch` and `Execute` do; the two `Sync` calls
/// are the join of the scope. Thread creation failure is C's failed `Reset`,
/// and a panicking side context counts as a failed hook. While the side thread
/// borrows the picture's planes through a view, the main context works on a
/// second view of the picture, which forwards to the picture's progress hook;
/// its error code is recorded on the picture afterwards. The contexts share
/// nothing they write, so the bytes do not depend on scheduling.
///
/// # Panics
///
/// Panics when a view of the whole picture cannot be made, where C asserts.
///
/// # Errors
///
/// Returns the error that `picture` holds at the end, as C returns
/// `picture->error_code == VP8_ENC_OK`.
#[expect(
    clippy::too_many_lines,
    reason = "the C function VP8LEncodeStream is translated as one function"
)]
pub fn vp8l_encode_stream(
    config: &WebPConfig,
    picture: &mut WebPPicture<'_>,
    bw_main: &mut VP8LBitWriter,
) -> Result<(), WebPEncodingError> {
    let mut enc_main = vp8l_encoder_new();
    let mut crunch_configs = [CrunchConfig::default(); CRUNCH_CONFIGS_MAX];
    let mut num_crunch_configs_main: i32 = 0;
    let mut num_crunch_configs_side: i32 = 0;
    let mut red_and_blue_always_zero = false;
    let mut params_main_configs = [CrunchConfig::default(); CRUNCH_CONFIGS_MAX];
    let mut params_side_configs = [CrunchConfig::default(); CRUNCH_CONFIGS_MAX];
    // The main thread uses picture->stats, the side thread uses stats_side.
    let mut stats_side = WebPAuxStats::default();
    let mut bw_side = VP8LBitWriter::default();

    if !vp8l_bit_writer_init(&mut bw_side, 0) {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY,
        ));
    }

    // C initializes picture_side with WebPPictureInit here, which succeeds; the
    // side picture is the view built below.
    'error: {
        // Analyze image (entropy, num_palettes etc)
        if !encoder_analyze(
            &mut enc_main,
            picture,
            config,
            &mut crunch_configs,
            &mut num_crunch_configs_main,
            &mut red_and_blue_always_zero,
        ) || !encoder_init(&mut enc_main, picture)
        {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        // Split the configs between the main and side threads (if any).
        if config.thread_level > 0 {
            num_crunch_configs_side = num_crunch_configs_main / 2;
            for idx in 0..to_index(num_crunch_configs_side) {
                params_side_configs[idx] = crunch_configs
                    [to_index(num_crunch_configs_main - num_crunch_configs_side) + idx];
            }
        }
        num_crunch_configs_main -= num_crunch_configs_side;
        params_main_configs[..to_index(num_crunch_configs_main)]
            .copy_from_slice(&crunch_configs[..to_index(num_crunch_configs_main)]);

        if num_crunch_configs_side == 0 {
            // One context, on the picture itself; its stats are params->stats.
            let mut stats_main = picture.stats.take();
            let mut params_main = StreamEncodeContext {
                config,
                picture: &mut *picture,
                bw: &mut *bw_main,
                enc: &mut enc_main,
                crunch_configs: params_main_configs,
                num_crunch_configs: num_crunch_configs_main,
                red_and_blue_always_zero,
                stats: stats_main.as_deref_mut(),
            };
            // Execute the main thread.
            let _ok_main = encode_stream_hook(&mut params_main);
            picture.stats = stats_main;
            break 'error;
        }

        // Create a side bit writer.
        if !vp8l_bit_writer_clone(bw_main, &mut bw_side) {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }
        // Create a side encoder.
        let mut enc_side = vp8l_encoder_new();
        if !encoder_init(&mut enc_side, picture) {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }
        // Copy the values that were computed for the main encoder.
        enc_side.histo_bits = enc_main.histo_bits;
        enc_side.predictor_transform_bits = enc_main.predictor_transform_bits;
        enc_side.cross_color_transform_bits = enc_main.cross_color_transform_bits;
        enc_side.palette_size = enc_main.palette_size;
        enc_side.palette = enc_main.palette;
        enc_side.palette_sorted = enc_main.palette_sorted;

        // The callbacks and the stats leave the picture while the views borrow it.
        let mut hook = picture.progress_hook.take();
        let mut stats_main = picture.stats.take();
        if let Some(stats) = stats_main.as_deref() {
            stats_side = *stats;
        }
        let width = picture.width;
        let height = picture.height;
        let outcome = {
            // Create a side picture (error_code is not thread-safe).
            let view_side =
                webp_picture_view(picture, /*left=*/ 0, /*top=*/ 0, width, height)
                    .expect("a view of the whole picture");
            // Progress hook is not thread-safe: the view has none.
            let side_parts = PictureViewParts::from_view(view_side);
            let mut picture_main = webp_picture_view(picture, 0, 0, width, height)
                .expect("a view of the whole picture");
            if let Some(hook) = hook.as_mut() {
                picture_main.progress_hook = Some(Box::new(hook));
            }
            let side_stats = if stats_main.is_none() {
                None
            } else {
                Some(&mut stats_side)
            };
            let bw_side = &mut bw_side;
            let enc_side = &mut enc_side;
            std::thread::scope(|scope| {
                // Start the second thread if needed.
                let side = std::thread::Builder::new().spawn_scoped(scope, move || {
                    let mut picture_side = side_parts.into_picture();
                    let mut params_side = StreamEncodeContext {
                        config,
                        picture: &mut picture_side,
                        bw: bw_side,
                        enc: enc_side,
                        crunch_configs: params_side_configs,
                        num_crunch_configs: num_crunch_configs_side,
                        red_and_blue_always_zero,
                        stats: side_stats,
                    };
                    let ok = encode_stream_hook(&mut params_side);
                    (ok, picture_side.error_code)
                });
                let Ok(side) = side else {
                    return None;
                };
                // Execute the main thread.
                let mut params_main = StreamEncodeContext {
                    config,
                    picture: &mut picture_main,
                    bw: &mut *bw_main,
                    enc: &mut enc_main,
                    crunch_configs: params_main_configs,
                    num_crunch_configs: num_crunch_configs_main,
                    red_and_blue_always_zero,
                    stats: stats_main.as_deref_mut(),
                };
                let ok_main = encode_stream_hook(&mut params_main);
                let main_error = picture_main.error_code;
                // Wait for the second thread.
                let (ok_side, side_error) = side
                    .join()
                    .unwrap_or((false, WebPEncodingError::VP8_ENC_OK));
                Some(ContextOutcome {
                    ok_main,
                    main_error,
                    ok_side,
                    side_error,
                })
            })
        };
        picture.progress_hook = hook;
        let Some(outcome) = outcome else {
            picture.stats = stats_main;
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        };
        // The main context recorded its errors on its view of the picture.
        webp_encoding_set_error(picture, outcome.main_error);
        if !outcome.ok_main || !outcome.ok_side {
            if picture.error_code == WebPEncodingError::VP8_ENC_OK {
                debug_assert!(outcome.side_error != WebPEncodingError::VP8_ENC_OK);
                webp_encoding_set_error(picture, outcome.side_error);
            }
            picture.stats = stats_main;
            break 'error;
        }
        if vp8l_bit_writer_num_bytes(&bw_side) < vp8l_bit_writer_num_bytes(bw_main) {
            vp8l_bit_writer_swap(bw_main, &mut bw_side);
            if let Some(stats) = stats_main.as_deref_mut() {
                *stats = stats_side;
            }
        }
        picture.stats = stats_main;
    }

    vp8l_bit_writer_wipe_out(&mut bw_side);
    picture_status(picture)
}

// upstream: src/enc/vp8l_enc.c VP8LEncodeImage
/// Encodes the picture and writes the complete RIFF file through the
/// picture's writer. C returns 0 for a `NULL` picture, which only the C ABI can
/// pass.
///
/// # Errors
///
/// Returns the error that `picture` holds at the end, as C returns
/// `picture->error_code == VP8_ENC_OK`: `VP8_ENC_ERROR_NULL_PARAMETER` without
/// an ARGB plane, and the errors of the encoding.
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    reason = "the C conversions of the coded size to int"
)]
pub fn vp8l_encode_image(
    config: &WebPConfig,
    picture: &mut WebPPicture<'_>,
) -> Result<(), WebPEncodingError> {
    let mut coded_size: usize = 0;
    let mut percent: i32 = 0;
    let mut bw = VP8LBitWriter::default();

    if picture.argb.is_none() {
        return Err(webp_encoding_set_error(
            picture,
            WebPEncodingError::VP8_ENC_ERROR_NULL_PARAMETER,
        ));
    }

    let width = picture.width;
    let height = picture.height;
    // Initialize BitWriter with size corresponding to 16 bpp to photo images and
    // 8 bpp for graphical images.
    let initial_size = if config.image_hint == WebPImageHint::WEBP_HINT_GRAPH {
        width * height
    } else {
        width * height * 2
    };
    'error: {
        if !vp8l_bit_writer_init(&mut bw, to_index(initial_size)) {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        let user_abort = |picture: &mut WebPPicture<'_>| {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_USER_ABORT);
        };
        if webp_report_progress(picture, 1, Some(&mut percent)).is_err() {
            user_abort(picture);
            break 'error;
        }
        // Reset stats (for pure lossless coding)
        if let Some(stats) = picture.stats.as_deref_mut() {
            *stats = WebPAuxStats::default();
            stats.psnr = [99.0; 5];
        }

        // Write image size.
        if !write_image_size(picture, &mut bw) {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        let has_alpha = webp_picture_has_transparency(picture);
        // Write the non-trivial Alpha flag and lossless version.
        if !write_real_alpha_and_version(&mut bw, has_alpha) {
            webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
            break 'error;
        }

        if webp_report_progress(picture, 2, Some(&mut percent)).is_err() {
            user_abort(picture);
            break 'error;
        }

        // Encode main image stream.
        if vp8l_encode_stream(config, picture, &mut bw).is_err() {
            break 'error;
        }

        if webp_report_progress(picture, 99, Some(&mut percent)).is_err() {
            user_abort(picture);
            break 'error;
        }

        // Finish the RIFF chunk.
        if write_image(picture, &mut bw, &mut coded_size).is_err() {
            break 'error;
        }

        if webp_report_progress(picture, 100, Some(&mut percent)).is_err() {
            user_abort(picture);
            break 'error;
        }

        // Save size.
        if let Some(stats) = picture.stats.as_deref_mut() {
            stats.coded_size += coded_size as i32;
            stats.lossless_size = coded_size as i32;
        }

        if let Some(extra_info) = picture.extra_info.as_deref_mut() {
            let mb_w = (width + 15) >> 4;
            let mb_h = (height + 15) >> 4;
            extra_info[..to_index(mb_w * mb_h)].fill(0);
        }
    }

    if bw.error {
        webp_encoding_set_error(picture, WebPEncodingError::VP8_ENC_ERROR_OUT_OF_MEMORY);
    }
    vp8l_bit_writer_wipe_out(&mut bw);
    picture_status(picture)
}
