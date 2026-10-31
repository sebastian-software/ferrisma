// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

//! Ported from `lib/jxl/base/byte_order.h`.
//! Byte slices replace pointers; native byte conversion methods implement the
//! same unaligned word loads, stores, and swaps without creating references to
//! unaligned integer values. Store16 intentionally truncates its u32 argument.

use crate::jxl::types::JxlEndianness;

pub const fn is_little_endian() -> bool {
    cfg!(target_endian = "little")
}

pub const fn swap_endianness(endianness: JxlEndianness) -> bool {
    matches!(endianness, JxlEndianness::Big) && is_little_endian()
        || matches!(endianness, JxlEndianness::Little) && !is_little_endian()
}

pub fn load_be16(p: &[u8]) -> u32 {
    u32::from(u16::from_be_bytes(p[..2].try_into().unwrap()))
}
pub fn load_le16(p: &[u8]) -> u32 {
    u32::from(u16::from_le_bytes(p[..2].try_into().unwrap()))
}
pub fn load_be32(p: &[u8]) -> u32 {
    u32::from_be_bytes(p[..4].try_into().unwrap())
}
pub fn load_le32(p: &[u8]) -> u32 {
    u32::from_le_bytes(p[..4].try_into().unwrap())
}
pub fn load_be64(p: &[u8]) -> u64 {
    u64::from_be_bytes(p[..8].try_into().unwrap())
}
pub fn load_le64(p: &[u8]) -> u64 {
    u64::from_le_bytes(p[..8].try_into().unwrap())
}
pub fn load_be_float(p: &[u8]) -> f32 {
    f32::from_bits(load_be32(p))
}
pub fn load_le_float(p: &[u8]) -> f32 {
    f32::from_bits(load_le32(p))
}

pub fn store_be16(native: u32, p: &mut [u8]) {
    p[..2].copy_from_slice(&(native as u16).to_be_bytes());
}
pub fn store_le16(native: u32, p: &mut [u8]) {
    p[..2].copy_from_slice(&(native as u16).to_le_bytes());
}
pub fn store_be32(native: u32, p: &mut [u8]) {
    p[..4].copy_from_slice(&native.to_be_bytes());
}
pub fn store_le32(native: u32, p: &mut [u8]) {
    p[..4].copy_from_slice(&native.to_le_bytes());
}
pub fn store_be64(native: u64, p: &mut [u8]) {
    p[..8].copy_from_slice(&native.to_be_bytes());
}
pub fn store_le64(native: u64, p: &mut [u8]) {
    p[..8].copy_from_slice(&native.to_le_bytes());
}

pub fn bswap_float(x: f32) -> f32 {
    f32::from_bits(x.to_bits().swap_bytes())
}

pub struct OrderBE;
pub struct OrderLE;

pub trait ByteOrder {
    fn store16(native: u32, p: &mut [u8]);
    fn store32(native: u32, p: &mut [u8]);
    fn load16(p: &[u8]) -> u32;
    fn load32(p: &[u8]) -> u32;
}

impl ByteOrder for OrderBE {
    fn store16(native: u32, p: &mut [u8]) {
        store_be16(native, p);
    }
    fn store32(native: u32, p: &mut [u8]) {
        store_be32(native, p);
    }
    fn load16(p: &[u8]) -> u32 {
        load_be16(p)
    }
    fn load32(p: &[u8]) -> u32 {
        load_be32(p)
    }
}

impl ByteOrder for OrderLE {
    fn store16(native: u32, p: &mut [u8]) {
        store_le16(native, p);
    }
    fn store32(native: u32, p: &mut [u8]) {
        store_le32(native, p);
    }
    fn load16(p: &[u8]) -> u32 {
        load_le16(p)
    }
    fn load32(p: &[u8]) -> u32 {
        load_le32(p)
    }
}

pub fn store16<O: ByteOrder>(_: O, native: u32, p: &mut [u8]) {
    O::store16(native, p);
}
pub fn store32<O: ByteOrder>(_: O, native: u32, p: &mut [u8]) {
    O::store32(native, p);
}
pub fn load16<O: ByteOrder>(_: O, p: &[u8]) -> u32 {
    O::load16(p)
}
pub fn load32<O: ByteOrder>(_: O, p: &[u8]) -> u32 {
    O::load32(p)
}
