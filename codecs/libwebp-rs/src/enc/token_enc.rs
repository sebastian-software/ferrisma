// libwebp v1.6.0, src/enc/token_enc.c:
//
// Copyright 2011 Google Inc. All Rights Reserved.
//
// Use of this source code is governed by a BSD-style license
// that can be found in the COPYING file in the root of the source
// tree. An additional intellectual property rights grant can be found
// in the file PATENTS. All contributing project authors may
// be found in the AUTHORS file in the root of the source tree.
// -----------------------------------------------------------------------------
//
// Paginated token buffer
//
//  A 'token' is a bit value associated with a probability, either fixed
// or a later-to-be-determined after statistics have been collected.
// For dynamic probability, we just record the slot id (idx) for the probability
// value in the final probability array (uint8_t* probas in VP8EmitTokens).
//
// Author: Skal (pascal.massimino@gmail.com)

//! Translation of `src/enc/token_enc.c` from libwebp v1.6.0.
//!
//! Work package: WP10 (VP8 encoder core).
//!
//! # Upstream behavior preserved
//!
//! Behavior of the C code that a reader might take for a defect, which the
//! translation keeps:
//!
//! 1. **`VP8RecordCoeffTokens` records the second token of category 5 with the
//!    probability index `base_id + 10` and the statistics slot 9.**
//! 2. **`VP8EstimateTokenSize` passes the bit as `token & (1 << 15)`** to
//!    `VP8BitCost`, which only tests it for zero.
//!
//! # Checks not applicable
//!
//! These checks do not apply, or cannot be made, with the reason:
//!
//! 1. **Compile-time switches.** `DISABLE_TOKEN_BUFFER` is undefined; see
//!    `src/enc/frame_enc.rs`, `# Checks not applicable`, item 1.

use crate::dec::common_dec::{NUM_BANDS, NUM_CTX, NUM_PROBAS};
use crate::dsp::cost::VP8_ENC_BANDS;
use crate::enc::cost_enc::{VP8Residual, vp8_bit_cost, vp8_record_stats};
use crate::enc::frame_enc::{VP8_CAT3, VP8_CAT4, VP8_CAT5, VP8_CAT6};
use crate::enc::vp8i_enc::{VP8TBuffer, proba_t};
use crate::utils::bit_writer_utils::{VP8BitWriter, vp8_put_bit};
use crate::utils::utils::webp_safe_malloc;

// upstream: src/enc/token_enc.c MIN_PAGE_SIZE
/// we use pages to reduce the number of `memcpy()`; minimum number of token per
/// page.
const MIN_PAGE_SIZE: i32 = 8192;
// upstream: src/enc/token_enc.c FIXED_PROBA_BIT
const FIXED_PROBA_BIT: u32 = 1 << 14;

// upstream: src/enc/token_enc.c token_t
/// bit #15: bit value
/// bit #14: flags for constant proba or idx
/// bits #0..13: slot or constant proba
#[expect(non_camel_case_types, reason = "C type name")]
type token_t = u16;

// upstream: src/enc/token_enc.c VP8Tokens
// C: a page is a `VP8Tokens` header with the `next` pointer, followed by the
// tokens (TOKEN_DATA). Here a page is the vector of its tokens, and the pages of
// a buffer are a vector in the order of the C chain (see `VP8TBuffer`).

/// The C size of the page header, `sizeof(VP8Tokens)`: one pointer.
const VP8_TOKENS_C_SIZE: usize = size_of::<usize>();

//------------------------------------------------------------------------------

// upstream: src/enc/token_enc.c VP8TBufferInit
/// Initialize an empty buffer.
pub fn vp8t_buffer_init(b: &mut VP8TBuffer, page_size: i32) {
    b.pages = Vec::new();
    b.left = 0;
    b.page_size = if page_size < MIN_PAGE_SIZE {
        MIN_PAGE_SIZE
    } else {
        page_size
    };
    b.error = false;
}

// upstream: src/enc/token_enc.c VP8TBufferClear
/// De-allocate pages memory.
pub fn vp8t_buffer_clear(b: &mut VP8TBuffer) {
    b.pages.clear();
    let page_size = b.page_size;
    vp8t_buffer_init(b, page_size);
}

// upstream: src/enc/token_enc.c TBufferNewPage
/// Appends a page, with the size test of `WebPSafeMalloc(1, size)` on the C page
/// size (header and tokens).
#[expect(clippy::cast_sign_loss, reason = "page_size is at least MIN_PAGE_SIZE")]
fn t_buffer_new_page(b: &mut VP8TBuffer) -> bool {
    let mut page: Option<Vec<token_t>> = None;
    if !b.error {
        let size = VP8_TOKENS_C_SIZE as u64 + b.page_size as u64 * size_of::<token_t>() as u64;
        page = usize::try_from(size)
            .ok()
            .and_then(|size| webp_safe_malloc::<()>(1, size))
            .and_then(|_| {
                let mut tokens = Vec::new();
                tokens.try_reserve_exact(b.page_size as usize).ok()?;
                tokens.resize(b.page_size as usize, 0);
                Some(tokens)
            });
    }
    let Some(page) = page else {
        b.error = true;
        return false;
    };

    b.pages.push(page);
    b.left = b.page_size;
    true
}

//------------------------------------------------------------------------------

// upstream: src/enc/token_enc.c TOKEN_ID
const fn token_id(t: i32, b: i32, ctx: i32) -> u32 {
    #[expect(clippy::cast_sign_loss, reason = "the token id is non-negative")]
    {
        (NUM_PROBAS * (ctx + NUM_CTX * (b + NUM_BANDS * t))) as u32
    }
}

// upstream: src/enc/token_enc.c AddToken
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_possible_wrap,
    clippy::cast_sign_loss,
    reason = "the token fits 16 bits and the slot is non-negative"
)]
fn add_token(b: &mut VP8TBuffer, bit: u32, proba_idx: u32, stats: &mut proba_t) -> u32 {
    debug_assert!(proba_idx < FIXED_PROBA_BIT);
    debug_assert!(bit <= 1);
    if b.left > 0 || t_buffer_new_page(b) {
        b.left -= 1;
        let slot = b.left as usize;
        let tokens = b.pages.last_mut().expect("a page exists");
        tokens[slot] = ((bit << 15) | proba_idx) as token_t;
    }
    vp8_record_stats(bit as i32, stats);
    bit
}

// upstream: src/enc/token_enc.c AddConstantToken
#[expect(
    clippy::cast_possible_truncation,
    clippy::cast_sign_loss,
    reason = "the token fits 16 bits and the slot is non-negative"
)]
fn add_constant_token(b: &mut VP8TBuffer, bit: u32, proba: u32) {
    debug_assert!(proba < 256);
    debug_assert!(bit <= 1);
    if b.left > 0 || t_buffer_new_page(b) {
        b.left -= 1;
        let slot = b.left as usize;
        let tokens = b.pages.last_mut().expect("a page exists");
        tokens[slot] = ((bit << 15) | FIXED_PROBA_BIT | proba) as token_t;
    }
}

// upstream: src/enc/token_enc.c VP8RecordCoeffTokens
/// Record the coding of coefficients without knowing the probabilities yet.
#[expect(clippy::too_many_lines, reason = "the C function translated whole")]
#[expect(
    clippy::cast_sign_loss,
    reason = "positions and contexts are non-negative indexes; C converts the level to uint32_t"
)]
#[expect(clippy::identity_op, reason = "the C constants")]
pub fn vp8_record_coeff_tokens(
    ctx: i32,
    res: &mut VP8Residual<'_>,
    tokens: &mut VP8TBuffer,
) -> i32 {
    let coeffs = res.coeffs;
    let coeff_type = res.coeff_type;
    let last = res.last;
    let mut n = res.first;
    let mut base_id = token_id(coeff_type, n, ctx);
    // should be stats[VP8EncBands[n]], but it's equivalent for n=0 or 1
    let mut s: (usize, usize) = (n as usize, ctx as usize);
    if add_token(
        tokens,
        u32::from(last >= 0),
        base_id,
        &mut res.stats[s.0][s.1][0],
    ) == 0
    {
        return 0;
    }

    while n < 16 {
        let c = i32::from(coeffs[n as usize]);
        n += 1;
        let sign = c < 0;
        let v: u32 = if sign { (-c) as u32 } else { c as u32 };
        if add_token(
            tokens,
            u32::from(v != 0),
            base_id + 1,
            &mut res.stats[s.0][s.1][1],
        ) == 0
        {
            let band = i32::from(VP8_ENC_BANDS[n as usize]);
            base_id = token_id(coeff_type, band, 0); // ctx=0
            s = (band as usize, 0);
            continue;
        }
        if add_token(
            tokens,
            u32::from(v > 1),
            base_id + 2,
            &mut res.stats[s.0][s.1][2],
        ) == 0
        {
            let band = i32::from(VP8_ENC_BANDS[n as usize]);
            base_id = token_id(coeff_type, band, 1); // ctx=1
            s = (band as usize, 1);
        } else {
            if add_token(
                tokens,
                u32::from(v > 4),
                base_id + 3,
                &mut res.stats[s.0][s.1][3],
            ) == 0
            {
                if add_token(
                    tokens,
                    u32::from(v != 2),
                    base_id + 4,
                    &mut res.stats[s.0][s.1][4],
                ) != 0
                {
                    add_token(
                        tokens,
                        u32::from(v == 4),
                        base_id + 5,
                        &mut res.stats[s.0][s.1][5],
                    );
                }
            } else if add_token(
                tokens,
                u32::from(v > 10),
                base_id + 6,
                &mut res.stats[s.0][s.1][6],
            ) == 0
            {
                if add_token(
                    tokens,
                    u32::from(v > 6),
                    base_id + 7,
                    &mut res.stats[s.0][s.1][7],
                ) == 0
                {
                    add_constant_token(tokens, u32::from(v == 6), 159);
                } else {
                    add_constant_token(tokens, u32::from(v >= 9), 165);
                    add_constant_token(tokens, u32::from(v & 1 == 0), 145);
                }
            } else {
                let mut mask: u32;
                let tab: &[u8];
                let mut residue = v - 3;
                if residue < (8 << 1) {
                    // VP8Cat3  (3b)
                    add_token(tokens, 0, base_id + 8, &mut res.stats[s.0][s.1][8]);
                    add_token(tokens, 0, base_id + 9, &mut res.stats[s.0][s.1][9]);
                    residue -= 8 << 0;
                    mask = 1 << 2;
                    tab = &VP8_CAT3;
                } else if residue < (8 << 2) {
                    // VP8Cat4  (4b)
                    add_token(tokens, 0, base_id + 8, &mut res.stats[s.0][s.1][8]);
                    add_token(tokens, 1, base_id + 9, &mut res.stats[s.0][s.1][9]);
                    residue -= 8 << 1;
                    mask = 1 << 3;
                    tab = &VP8_CAT4;
                } else if residue < (8 << 3) {
                    // VP8Cat5  (5b)
                    add_token(tokens, 1, base_id + 8, &mut res.stats[s.0][s.1][8]);
                    add_token(tokens, 0, base_id + 10, &mut res.stats[s.0][s.1][9]);
                    residue -= 8 << 2;
                    mask = 1 << 4;
                    tab = &VP8_CAT5;
                } else {
                    // VP8Cat6 (11b)
                    add_token(tokens, 1, base_id + 8, &mut res.stats[s.0][s.1][8]);
                    add_token(tokens, 1, base_id + 10, &mut res.stats[s.0][s.1][9]);
                    residue -= 8 << 3;
                    mask = 1 << 10;
                    tab = &VP8_CAT6;
                }
                let mut tab_pos = 0;
                while mask != 0 {
                    add_constant_token(
                        tokens,
                        u32::from(residue & mask != 0),
                        u32::from(tab[tab_pos]),
                    );
                    tab_pos += 1;
                    mask >>= 1;
                }
            }
            let band = i32::from(VP8_ENC_BANDS[n as usize]);
            base_id = token_id(coeff_type, band, 2); // ctx=2
            s = (band as usize, 2);
        }
        add_constant_token(tokens, u32::from(sign), 128);
        if n == 16
            || add_token(
                tokens,
                u32::from(n <= last),
                base_id,
                &mut res.stats[s.0][s.1][0],
            ) == 0
        {
            return 1; // EOB
        }
    }
    1
}

//------------------------------------------------------------------------------
// Final coding pass, with known probabilities

// upstream: src/enc/token_enc.c VP8EmitTokens
/// Finalizes bitstream when probabilities are known. Deletes the allocated
/// token memory if `final_pass` is true. `probas` is the flat array of the
/// coefficient probabilities (C: `(const uint8_t*)proba->coeffs`).
#[expect(clippy::cast_sign_loss, reason = "left and page_size are non-negative")]
pub fn vp8_emit_tokens(
    b: &mut VP8TBuffer,
    bw: &mut VP8BitWriter,
    probas: &[u8],
    final_pass: bool,
) -> bool {
    debug_assert!(!b.error);
    let num_pages = b.pages.len();
    for (index, tokens) in b.pages.iter().enumerate() {
        let last_page = index + 1 == num_pages;
        let n_end = if last_page { b.left } else { 0 };
        let mut n = b.page_size;
        while n > n_end {
            n -= 1;
            let token = tokens[n as usize];
            let bit = (token >> 15) & 1 != 0;
            if u32::from(token) & FIXED_PROBA_BIT != 0 {
                vp8_put_bit(bw, bit, i32::from(token & 0xff)); // constant proba
            } else {
                vp8_put_bit(bw, bit, i32::from(probas[usize::from(token & 0x3fff)]));
            }
        }
    }
    if final_pass {
        b.pages = Vec::new();
    }
    true
}

// upstream: src/enc/token_enc.c VP8EstimateTokenSize
/// Estimate the final coded size given a set of 'probas'.
#[must_use]
#[expect(clippy::cast_sign_loss, reason = "left and page_size are non-negative")]
pub fn vp8_estimate_token_size(b: &VP8TBuffer, probas: &[u8]) -> usize {
    let mut size: usize = 0;
    debug_assert!(!b.error);
    let num_pages = b.pages.len();
    for (index, tokens) in b.pages.iter().enumerate() {
        let last_page = index + 1 == num_pages;
        let n_end = if last_page { b.left } else { 0 };
        let mut n = b.page_size;
        while n > n_end {
            n -= 1;
            let token = tokens[n as usize];
            let bit = token & (1 << 15) != 0;
            if u32::from(token) & FIXED_PROBA_BIT != 0 {
                size = size.wrapping_add(vp8_bit_cost(bit, (token & 0xff) as u8) as usize);
            } else {
                size = size
                    .wrapping_add(vp8_bit_cost(bit, probas[usize::from(token & 0x3fff)]) as usize);
            }
        }
    }
    size
}
