// Copyright 2013, 2016 Google Inc. All Rights Reserved.
// Distributed under MIT license. See licenses/BROTLI-MIT.txt or
// https://opensource.org/licenses/MIT.
// Mechanical translation of pinned Brotli c/common/constants.{c,h}.

pub const BROTLI_CONTEXT_MAP_MAX_RLE: u32 = 16;
pub const BROTLI_MAX_NUMBER_OF_BLOCK_TYPES: u32 = 256;
pub const BROTLI_NUM_LITERAL_SYMBOLS: u32 = 256;
pub const BROTLI_NUM_COMMAND_SYMBOLS: u32 = 704;
pub const BROTLI_NUM_BLOCK_LEN_SYMBOLS: u32 = 26;
pub const BROTLI_MAX_CONTEXT_MAP_SYMBOLS: u32 =
    BROTLI_MAX_NUMBER_OF_BLOCK_TYPES + BROTLI_CONTEXT_MAP_MAX_RLE;
pub const BROTLI_MAX_BLOCK_TYPE_SYMBOLS: u32 = BROTLI_MAX_NUMBER_OF_BLOCK_TYPES + 2;
pub const BROTLI_REPEAT_PREVIOUS_CODE_LENGTH: u32 = 16;
pub const BROTLI_REPEAT_ZERO_CODE_LENGTH: u32 = 17;
pub const BROTLI_CODE_LENGTH_CODES: u32 = BROTLI_REPEAT_ZERO_CODE_LENGTH + 1;
pub const BROTLI_INITIAL_REPEATED_CODE_LENGTH: u32 = 8;
pub const BROTLI_LARGE_MAX_DISTANCE_BITS: u32 = 62;
pub const BROTLI_LARGE_MIN_WBITS: u32 = 10;
pub const BROTLI_LARGE_MAX_WBITS: u32 = 30;
pub const BROTLI_NUM_DISTANCE_SHORT_CODES: u32 = 16;
pub const BROTLI_MAX_NPOSTFIX: u32 = 3;
pub const BROTLI_MAX_NDIRECT: u32 = 120;
pub const BROTLI_MAX_DISTANCE_BITS: u32 = 24;
pub const BROTLI_MAX_DISTANCE: u32 = 0x3fffffc;
pub const BROTLI_MAX_ALLOWED_DISTANCE: u32 = 0x7ffffffc;
pub const BROTLI_NUM_INS_COPY_CODES: u32 = 24;
pub const BROTLI_LITERAL_CONTEXT_BITS: u32 = 6;
pub const BROTLI_DISTANCE_CONTEXT_BITS: u32 = 2;
pub const BROTLI_WINDOW_GAP: usize = 16;

/// The macro's reached arguments keep its original unsigned shift domain.
pub const fn brotli_distance_alphabet_size(npostfix: u32, ndirect: u32, maxnbits: u32) -> u32 {
    BROTLI_NUM_DISTANCE_SHORT_CODES
        .wrapping_add(ndirect)
        .wrapping_add(maxnbits << npostfix.wrapping_add(1))
}

// The source's unused BROTLI_NUM_DISTANCE_SYMBOLS macro passes MAX_NDIRECT
// before MAX_NPOSTFIX. Expanding it would shift by 121. No Rust value is
// invented for that unreached, undefined C expression.

pub const fn brotli_max_backward_limit(window_bits: u32) -> usize {
    (1usize << window_bits).wrapping_sub(BROTLI_WINDOW_GAP)
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct BrotliDistanceCodeLimit {
    pub max_alphabet_size: u32,
    pub max_distance: u32,
}

/// Calculates the last complete distance-code group, in source operation order.
/// Shift counts must be below 32, as in the original C helper.
pub fn brotli_calculate_distance_code_limit(
    max_distance: u32,
    npostfix: u32,
    ndirect: u32,
) -> BrotliDistanceCodeLimit {
    if max_distance <= ndirect {
        return BrotliDistanceCodeLimit {
            max_alphabet_size: max_distance.wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES),
            max_distance,
        };
    }
    let forbidden_distance = max_distance.wrapping_add(1);
    let mut offset = forbidden_distance.wrapping_sub(ndirect).wrapping_sub(1);
    let mut ndistbits = 0u32;
    let postfix = (1u32 << npostfix).wrapping_sub(1);
    offset = (offset >> npostfix).wrapping_add(4);
    let mut tmp = offset / 2;
    while tmp != 0 {
        ndistbits = ndistbits.wrapping_add(1);
        tmp >>= 1;
    }
    ndistbits = ndistbits.wrapping_sub(1);
    let half = (offset >> ndistbits) & 1;
    let mut group = (ndistbits.wrapping_sub(1) << 1) | half;
    if group == 0 {
        return BrotliDistanceCodeLimit {
            max_alphabet_size: ndirect.wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES),
            max_distance: ndirect,
        };
    }
    group = group.wrapping_sub(1);
    ndistbits = (group >> 1).wrapping_add(1);
    let extra = (1u32 << ndistbits).wrapping_sub(1);
    let mut start = (1u32 << (ndistbits + 1)).wrapping_sub(4);
    start = start.wrapping_add((group & 1) << ndistbits);
    BrotliDistanceCodeLimit {
        max_alphabet_size: ((group << npostfix) | postfix)
            .wrapping_add(ndirect)
            .wrapping_add(BROTLI_NUM_DISTANCE_SHORT_CODES)
            .wrapping_add(1),
        max_distance: (start.wrapping_add(extra) << npostfix)
            .wrapping_add(postfix)
            .wrapping_add(ndirect)
            .wrapping_add(1),
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
#[repr(C)]
pub struct BrotliPrefixCodeRange {
    pub offset: u16,
    pub nbits: u8,
}

pub const BROTLI_PREFIX_CODE_RANGES: [BrotliPrefixCodeRange; 26] = [
    BrotliPrefixCodeRange {
        offset: 1,
        nbits: 2,
    },
    BrotliPrefixCodeRange {
        offset: 5,
        nbits: 2,
    },
    BrotliPrefixCodeRange {
        offset: 9,
        nbits: 2,
    },
    BrotliPrefixCodeRange {
        offset: 13,
        nbits: 2,
    },
    BrotliPrefixCodeRange {
        offset: 17,
        nbits: 3,
    },
    BrotliPrefixCodeRange {
        offset: 25,
        nbits: 3,
    },
    BrotliPrefixCodeRange {
        offset: 33,
        nbits: 3,
    },
    BrotliPrefixCodeRange {
        offset: 41,
        nbits: 3,
    },
    BrotliPrefixCodeRange {
        offset: 49,
        nbits: 4,
    },
    BrotliPrefixCodeRange {
        offset: 65,
        nbits: 4,
    },
    BrotliPrefixCodeRange {
        offset: 81,
        nbits: 4,
    },
    BrotliPrefixCodeRange {
        offset: 97,
        nbits: 4,
    },
    BrotliPrefixCodeRange {
        offset: 113,
        nbits: 5,
    },
    BrotliPrefixCodeRange {
        offset: 145,
        nbits: 5,
    },
    BrotliPrefixCodeRange {
        offset: 177,
        nbits: 5,
    },
    BrotliPrefixCodeRange {
        offset: 209,
        nbits: 5,
    },
    BrotliPrefixCodeRange {
        offset: 241,
        nbits: 6,
    },
    BrotliPrefixCodeRange {
        offset: 305,
        nbits: 6,
    },
    BrotliPrefixCodeRange {
        offset: 369,
        nbits: 7,
    },
    BrotliPrefixCodeRange {
        offset: 497,
        nbits: 8,
    },
    BrotliPrefixCodeRange {
        offset: 753,
        nbits: 9,
    },
    BrotliPrefixCodeRange {
        offset: 1265,
        nbits: 10,
    },
    BrotliPrefixCodeRange {
        offset: 2289,
        nbits: 11,
    },
    BrotliPrefixCodeRange {
        offset: 4337,
        nbits: 12,
    },
    BrotliPrefixCodeRange {
        offset: 8433,
        nbits: 13,
    },
    BrotliPrefixCodeRange {
        offset: 16625,
        nbits: 24,
    },
];
