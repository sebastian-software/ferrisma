// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/headers.{h,cc}`.
//! Keeps the visitor field order, aspect-ratio selection, validation order, and
//! integer truncation. Structs start initialized before Bundle::Init visits
//! every conditional branch, as required by Rust.

use super::base::status::{Status, StatusCode};
use super::dec_bit_reader::BitReader;
use super::field_encodings::{Fields, U32Enc, bits, bits_offset, val};
use super::fields::{Visitor, bundle};

pub const CODESTREAM_MARKER: u8 = 0x0A;

#[derive(Clone, Copy)]
struct Rational {
    num: u32,
    den: u32,
}

impl Rational {
    const fn mul_truncate(self, multiplicand: u32) -> u32 {
        (multiplicand as u64 * self.num as u64 / self.den as u64) as u32
    }
}

fn fixed_aspect_ratios(ratio: u32) -> Rational {
    debug_assert!(ratio != 0 && ratio < 8);
    const RATIOS: [Rational; 7] = [
        Rational { num: 1, den: 1 },
        Rational { num: 12, den: 10 },
        Rational { num: 4, den: 3 },
        Rational { num: 3, den: 2 },
        Rational { num: 16, den: 9 },
        Rational { num: 5, den: 4 },
        Rational { num: 2, den: 1 },
    ];
    RATIOS[(ratio - 1) as usize]
}

fn find_aspect_ratio(xsize: u32, ysize: u32) -> u32 {
    for r in 1..8 {
        if xsize == fixed_aspect_ratios(r).mul_truncate(ysize) {
            return r;
        }
    }
    0
}

pub struct SizeHeader {
    small: bool,
    ysize_div8_minus_1: u32,
    ysize: u32,
    ratio: u32,
    xsize_div8_minus_1: u32,
    xsize: u32,
}

impl Default for SizeHeader {
    fn default() -> Self {
        let mut header = Self {
            small: false,
            ysize_div8_minus_1: 0,
            ysize: 0,
            ratio: 0,
            xsize_div8_minus_1: 0,
            xsize: 0,
        };
        bundle::init(&mut header);
        header
    }
}

impl SizeHeader {
    pub fn set(&mut self, xsize64: usize, ysize64: usize) -> Status {
        if xsize64 > u32::MAX as usize || ysize64 > u32::MAX as usize {
            return Err(StatusCode::GenericError);
        }
        let xsize32 = xsize64 as u32;
        let ysize32 = ysize64 as u32;
        if xsize64 == 0 || ysize64 == 0 {
            return Err(StatusCode::GenericError);
        }
        self.ratio = find_aspect_ratio(xsize32, ysize32);
        self.small = ysize64 <= 256
            && ysize64 % 8 == 0
            && (self.ratio != 0 || (xsize64 <= 256 && xsize64 % 8 == 0));
        if self.small {
            self.ysize_div8_minus_1 = ysize32 / 8 - 1;
        } else {
            self.ysize = ysize32;
        }
        if self.ratio == 0 {
            if self.small {
                self.xsize_div8_minus_1 = xsize32 / 8 - 1;
            } else {
                self.xsize = xsize32;
            }
        }
        if self.xsize() != xsize64 || self.ysize() != ysize64 {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }

    pub fn xsize(&self) -> usize {
        if self.ratio != 0 {
            return fixed_aspect_ratios(self.ratio).mul_truncate(self.ysize() as u32) as usize;
        }
        if self.small {
            ((self.xsize_div8_minus_1 + 1) * 8) as usize
        } else {
            self.xsize as usize
        }
    }

    pub fn ysize(&self) -> usize {
        if self.small {
            ((self.ysize_div8_minus_1 + 1) * 8) as usize
        } else {
            self.ysize as usize
        }
    }
}

impl Fields for SizeHeader {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.bool(false, &mut self.small)?;
        if visitor.conditional(self.small) {
            visitor.bits(5, 0, &mut self.ysize_div8_minus_1)?;
        }
        if visitor.conditional(!self.small) {
            visitor.u32(size_encoding(), 1, &mut self.ysize)?;
        }
        visitor.bits(3, 0, &mut self.ratio)?;
        if visitor.conditional(self.ratio == 0 && self.small) {
            visitor.bits(5, 0, &mut self.xsize_div8_minus_1)?;
        }
        if visitor.conditional(self.ratio == 0 && !self.small) {
            visitor.u32(size_encoding(), 1, &mut self.xsize)?;
        }
        Ok(())
    }
}

fn size_encoding() -> U32Enc {
    U32Enc::new(
        bits_offset(9, 1),
        bits_offset(13, 1),
        bits_offset(18, 1),
        bits_offset(30, 1),
    )
}

pub struct PreviewHeader {
    div8: bool,
    ysize_div8: u32,
    ysize: u32,
    ratio: u32,
    xsize_div8: u32,
    xsize: u32,
}

impl Default for PreviewHeader {
    fn default() -> Self {
        let mut header = Self {
            div8: false,
            ysize_div8: 0,
            ysize: 0,
            ratio: 0,
            xsize_div8: 0,
            xsize: 0,
        };
        bundle::init(&mut header);
        header
    }
}

impl PreviewHeader {
    pub fn set(&mut self, xsize64: usize, ysize64: usize) -> Status {
        let xsize32 = xsize64 as u32;
        let ysize32 = ysize64 as u32;
        if xsize64 == 0 || ysize64 == 0 {
            return Err(StatusCode::GenericError);
        }
        self.div8 = xsize64 % 8 == 0 && ysize64 % 8 == 0;
        if self.div8 {
            self.ysize_div8 = ysize32 / 8;
        } else {
            self.ysize = ysize32;
        }
        self.ratio = find_aspect_ratio(xsize32, ysize32);
        if self.ratio == 0 {
            if self.div8 {
                self.xsize_div8 = xsize32 / 8;
            } else {
                self.xsize = xsize32;
            }
        }
        if self.xsize() != xsize64 || self.ysize() != ysize64 {
            return Err(StatusCode::GenericError);
        }
        Ok(())
    }

    pub fn xsize(&self) -> usize {
        if self.ratio != 0 {
            return fixed_aspect_ratios(self.ratio).mul_truncate(self.ysize() as u32) as usize;
        }
        if self.div8 {
            (self.xsize_div8 * 8) as usize
        } else {
            self.xsize as usize
        }
    }

    pub fn ysize(&self) -> usize {
        if self.div8 {
            (self.ysize_div8 * 8) as usize
        } else {
            self.ysize as usize
        }
    }
}

impl Fields for PreviewHeader {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.bool(false, &mut self.div8)?;
        let div8_encoding = U32Enc::new(val(16), val(32), bits_offset(5, 1), bits_offset(9, 33));
        let encoding = U32Enc::new(
            bits_offset(6, 1),
            bits_offset(8, 65),
            bits_offset(10, 321),
            bits_offset(12, 1345),
        );
        if visitor.conditional(self.div8) {
            visitor.u32(div8_encoding, 1, &mut self.ysize_div8)?;
        }
        if visitor.conditional(!self.div8) {
            visitor.u32(encoding, 1, &mut self.ysize)?;
        }
        visitor.bits(3, 0, &mut self.ratio)?;
        if visitor.conditional(self.ratio == 0 && self.div8) {
            visitor.u32(div8_encoding, 1, &mut self.xsize_div8)?;
        }
        if visitor.conditional(self.ratio == 0 && !self.div8) {
            visitor.u32(encoding, 1, &mut self.xsize)?;
        }
        Ok(())
    }
}

pub struct AnimationHeader {
    pub tps_numerator: u32,
    pub tps_denominator: u32,
    pub num_loops: u32,
    pub have_timecodes: bool,
}

impl Default for AnimationHeader {
    fn default() -> Self {
        let mut header = Self {
            tps_numerator: 0,
            tps_denominator: 0,
            num_loops: 0,
            have_timecodes: false,
        };
        bundle::init(&mut header);
        header
    }
}

impl Fields for AnimationHeader {
    fn visit_fields(&mut self, visitor: &mut dyn Visitor) -> Status {
        visitor.u32(
            U32Enc::new(val(100), val(1000), bits_offset(10, 1), bits_offset(30, 1)),
            1,
            &mut self.tps_numerator,
        )?;
        visitor.u32(
            U32Enc::new(val(1), val(1001), bits_offset(8, 1), bits_offset(10, 1)),
            1,
            &mut self.tps_denominator,
        )?;
        visitor.u32(
            U32Enc::new(val(0), bits(3), bits(16), bits(32)),
            0,
            &mut self.num_loops,
        )?;
        visitor.bool(false, &mut self.have_timecodes)?;
        Ok(())
    }
}

pub fn read_size_header(reader: &mut BitReader<'_>, size: &mut SizeHeader) -> Status {
    bundle::read(reader, size)
}
