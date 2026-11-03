// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under MIT license. See licenses/BROTLI-MIT.txt or
// https://opensource.org/licenses/MIT.
// Mechanical translation of pinned Brotli c/common/transform.{c,h}.
// Table pointers become immutable borrowed views. Destination storage remains
// a direct, exclusive caller borrow; this module allocates no storage.

use super::tables::{PREFIX_SUFFIX, PREFIX_SUFFIX_MAP, TRANSFORMS_DATA};

pub const BROTLI_TRANSFORM_IDENTITY: u8 = 0;
pub const BROTLI_TRANSFORM_OMIT_LAST_1: u8 = 1;
pub const BROTLI_TRANSFORM_OMIT_LAST_2: u8 = 2;
pub const BROTLI_TRANSFORM_OMIT_LAST_3: u8 = 3;
pub const BROTLI_TRANSFORM_OMIT_LAST_4: u8 = 4;
pub const BROTLI_TRANSFORM_OMIT_LAST_5: u8 = 5;
pub const BROTLI_TRANSFORM_OMIT_LAST_6: u8 = 6;
pub const BROTLI_TRANSFORM_OMIT_LAST_7: u8 = 7;
pub const BROTLI_TRANSFORM_OMIT_LAST_8: u8 = 8;
pub const BROTLI_TRANSFORM_OMIT_LAST_9: u8 = 9;
pub const BROTLI_TRANSFORM_UPPERCASE_FIRST: u8 = 10;
pub const BROTLI_TRANSFORM_UPPERCASE_ALL: u8 = 11;
pub const BROTLI_TRANSFORM_OMIT_FIRST_1: u8 = 12;
pub const BROTLI_TRANSFORM_OMIT_FIRST_2: u8 = 13;
pub const BROTLI_TRANSFORM_OMIT_FIRST_3: u8 = 14;
pub const BROTLI_TRANSFORM_OMIT_FIRST_4: u8 = 15;
pub const BROTLI_TRANSFORM_OMIT_FIRST_5: u8 = 16;
pub const BROTLI_TRANSFORM_OMIT_FIRST_6: u8 = 17;
pub const BROTLI_TRANSFORM_OMIT_FIRST_7: u8 = 18;
pub const BROTLI_TRANSFORM_OMIT_FIRST_8: u8 = 19;
pub const BROTLI_TRANSFORM_OMIT_FIRST_9: u8 = 20;
pub const BROTLI_TRANSFORM_SHIFT_FIRST: u8 = 21;
pub const BROTLI_TRANSFORM_SHIFT_ALL: u8 = 22;
pub const BROTLI_NUM_TRANSFORM_TYPES: u8 = 23;
pub const BROTLI_TRANSFORMS_MAX_CUT_OFF: usize = BROTLI_TRANSFORM_OMIT_LAST_9 as usize;

pub struct BrotliTransforms<'a> {
    pub prefix_suffix_size: u16,
    pub prefix_suffix: &'a [u8],
    pub prefix_suffix_map: &'a [u16],
    pub num_transforms: u32,
    pub transforms: &'a [u8],
    pub params: Option<&'a [u8]>,
    pub cut_off_transforms: [i16; BROTLI_TRANSFORMS_MAX_CUT_OFF + 1],
}

impl BrotliTransforms<'_> {
    pub fn prefix_id(&self, index: i32) -> u8 {
        self.transforms[(index * 3) as usize]
    }

    pub fn transform_type(&self, index: i32) -> u8 {
        self.transforms[(index * 3 + 1) as usize]
    }

    pub fn suffix_id(&self, index: i32) -> u8 {
        self.transforms[(index * 3 + 2) as usize]
    }

    pub fn prefix(&self, index: i32) -> &[u8] {
        &self.prefix_suffix
            [usize::from(self.prefix_suffix_map[usize::from(self.prefix_id(index))])..]
    }

    pub fn suffix(&self, index: i32) -> &[u8] {
        &self.prefix_suffix
            [usize::from(self.prefix_suffix_map[usize::from(self.suffix_id(index))])..]
    }
}

static TRANSFORMS: BrotliTransforms<'static> = BrotliTransforms {
    prefix_suffix_size: PREFIX_SUFFIX.len() as u16,
    prefix_suffix: &PREFIX_SUFFIX,
    prefix_suffix_map: &PREFIX_SUFFIX_MAP,
    num_transforms: (TRANSFORMS_DATA.len() / 3) as u32,
    transforms: &TRANSFORMS_DATA,
    params: None,
    cut_off_transforms: [0, 12, 27, 23, 42, 63, 56, 48, 59, 64],
};

pub fn brotli_get_transforms() -> &'static BrotliTransforms<'static> {
    &TRANSFORMS
}

/// Direct borrowed destination operations at the original byte checkpoints.
///
/// A destination implementation must retain writes/XORs on the same owner.
/// `read` returns the actual initialized byte; `xor` performs the in-place
/// operation without exposing a prior tail byte as a returned scalar.
pub trait ByteDestination {
    fn read(&self, index: usize) -> u8;
    fn write(&mut self, index: usize, value: u8);
    fn xor(&mut self, index: usize, mask: u8);
}

impl ByteDestination for [u8] {
    fn read(&self, index: usize) -> u8 {
        self[index]
    }
    fn write(&mut self, index: usize, value: u8) {
        self[index] = value;
    }
    fn xor(&mut self, index: usize, mask: u8) {
        self[index] ^= mask;
    }
}

fn to_upper_case<D: ByteDestination + ?Sized>(dst: &mut D, pos: usize) -> i32 {
    if dst.read(pos) < 0xc0 {
        if dst.read(pos) >= b'a' && dst.read(pos) <= b'z' {
            dst.xor(pos, 32);
        }
        return 1;
    }
    if dst.read(pos) < 0xe0 {
        dst.xor(pos + 1, 32);
        return 2;
    }
    dst.xor(pos + 2, 5);
    3
}

fn shift<D: ByteDestination + ?Sized>(
    dst: &mut D,
    pos: usize,
    word_len: i32,
    parameter: u16,
) -> i32 {
    let parameter = u32::from(parameter);
    let mut scalar = (parameter & 0x7fff).wrapping_add(0x1000000u32 - (parameter & 0x8000));
    if dst.read(pos) < 0x80 {
        scalar = scalar.wrapping_add(u32::from(dst.read(pos)));
        dst.write(pos, (scalar & 0x7f) as u8);
        return 1;
    } else if dst.read(pos) < 0xc0 {
        return 1;
    } else if dst.read(pos) < 0xe0 {
        if word_len < 2 {
            return 1;
        }
        scalar = scalar.wrapping_add(
            u32::from(dst.read(pos + 1) & 0x3f) | (u32::from(dst.read(pos) & 0x1f) << 6),
        );
        dst.write(pos, (0xc0 | ((scalar >> 6) & 0x1f)) as u8);
        dst.write(
            pos + 1,
            (u32::from(dst.read(pos + 1) & 0xc0) | (scalar & 0x3f)) as u8,
        );
        return 2;
    } else if dst.read(pos) < 0xf0 {
        if word_len < 3 {
            return word_len;
        }
        scalar = scalar.wrapping_add(
            u32::from(dst.read(pos + 2) & 0x3f)
                | (u32::from(dst.read(pos + 1) & 0x3f) << 6)
                | (u32::from(dst.read(pos) & 0x0f) << 12),
        );
        dst.write(pos, (0xe0 | ((scalar >> 12) & 0x0f)) as u8);
        dst.write(
            pos + 1,
            (u32::from(dst.read(pos + 1) & 0xc0) | ((scalar >> 6) & 0x3f)) as u8,
        );
        dst.write(
            pos + 2,
            (u32::from(dst.read(pos + 2) & 0xc0) | (scalar & 0x3f)) as u8,
        );
        return 3;
    } else if dst.read(pos) < 0xf8 {
        if word_len < 4 {
            return word_len;
        }
        scalar = scalar.wrapping_add(
            u32::from(dst.read(pos + 3) & 0x3f)
                | (u32::from(dst.read(pos + 2) & 0x3f) << 6)
                | (u32::from(dst.read(pos + 1) & 0x3f) << 12)
                | (u32::from(dst.read(pos) & 0x07) << 18),
        );
        dst.write(pos, (0xf0 | ((scalar >> 18) & 0x07)) as u8);
        dst.write(
            pos + 1,
            (u32::from(dst.read(pos + 1) & 0xc0) | ((scalar >> 12) & 0x3f)) as u8,
        );
        dst.write(
            pos + 2,
            (u32::from(dst.read(pos + 2) & 0xc0) | ((scalar >> 6) & 0x3f)) as u8,
        );
        dst.write(
            pos + 3,
            (u32::from(dst.read(pos + 3) & 0xc0) | (scalar & 0x3f)) as u8,
        );
        return 4;
    }
    1
}

/// Transforms a dictionary word into the caller's initialized physical backing.
///
/// `len` is the original signed logical length, independently of the word's
/// physical backing. Reached source word/prefix/suffix/parameter indices and
/// pointer additions must be valid, and signed arithmetic must be representable.
/// The destination includes every reached byte, including up to two bytes past
/// the copied word for the source's simplified uppercase operation. Those tail
/// bytes can be read and changed even beyond the returned logical length.
/// This initialized-slice boundary does not initialize an unfilled ringbuffer.
pub fn brotli_transform_dictionary_word(
    dst: &mut [u8],
    word: &[u8],
    len: i32,
    transforms: &BrotliTransforms<'_>,
    transform_idx: i32,
) -> i32 {
    brotli_transform_dictionary_word_to(dst, word, len, transforms, transform_idx)
}

/// The same source algorithm over a monomorphized borrowed destination view.
/// The word/table/index/arithmetic preconditions above remain in force. Every
/// `read` byte must be initialized; `xor` is a distinct original destination
/// operation. This interface supplies no raw or uninitialized implementation.
pub fn brotli_transform_dictionary_word_to<D: ByteDestination + ?Sized>(
    dst: &mut D,
    mut word: &[u8],
    mut len: i32,
    transforms: &BrotliTransforms<'_>,
    transform_idx: i32,
) -> i32 {
    let mut idx = 0i32;
    let prefix = transforms.prefix(transform_idx);
    let transform_type = transforms.transform_type(transform_idx);
    let suffix = transforms.suffix(transform_idx);
    {
        let mut prefix_pos = 1usize;
        let mut prefix_len = i32::from(prefix[0]);
        while prefix_len != 0 {
            prefix_len -= 1;
            dst.write(idx as usize, prefix[prefix_pos]);
            idx += 1;
            prefix_pos += 1;
        }
    }
    {
        let t = i32::from(transform_type);
        let mut i = 0i32;
        if t <= i32::from(BROTLI_TRANSFORM_OMIT_LAST_9) {
            len -= t;
        } else if t >= i32::from(BROTLI_TRANSFORM_OMIT_FIRST_1)
            && t <= i32::from(BROTLI_TRANSFORM_OMIT_FIRST_9)
        {
            let skip = t - (i32::from(BROTLI_TRANSFORM_OMIT_FIRST_1) - 1);
            word = &word[skip as usize..];
            len -= skip;
        }
        while i < len {
            dst.write(idx as usize, word[i as usize]);
            idx += 1;
            i += 1;
        }
        if t == i32::from(BROTLI_TRANSFORM_UPPERCASE_FIRST) {
            to_upper_case(dst, (idx - len) as usize);
        } else if t == i32::from(BROTLI_TRANSFORM_UPPERCASE_ALL) {
            let mut uppercase = idx - len;
            while len > 0 {
                let step = to_upper_case(dst, uppercase as usize);
                uppercase += step;
                len -= step;
            }
        } else if t == i32::from(BROTLI_TRANSFORM_SHIFT_FIRST) {
            let params = transforms.params.unwrap();
            let param = u16::from(params[(transform_idx * 2) as usize])
                + (u16::from(params[(transform_idx * 2 + 1) as usize]) << 8);
            shift(dst, (idx - len) as usize, len, param);
        } else if t == i32::from(BROTLI_TRANSFORM_SHIFT_ALL) {
            let params = transforms.params.unwrap();
            let param = u16::from(params[(transform_idx * 2) as usize])
                + (u16::from(params[(transform_idx * 2 + 1) as usize]) << 8);
            let mut shift_pos = idx - len;
            while len > 0 {
                let step = shift(dst, shift_pos as usize, len, param);
                shift_pos += step;
                len -= step;
            }
        }
    }
    {
        let mut suffix_pos = 1usize;
        let mut suffix_len = i32::from(suffix[0]);
        while suffix_len != 0 {
            suffix_len -= 1;
            dst.write(idx as usize, suffix[suffix_pos]);
            idx += 1;
            suffix_pos += 1;
        }
        idx
    }
}
