// Copyright 2013 Google Inc. All Rights Reserved.
// Distributed under the MIT license in licenses/BROTLI-MIT.txt.

//! c/dec/bit_reader.h:375-390, with the original unknown destination storage.
//! The initialized-slice BitReader API remains unchanged. This counterpart
//! writes the exact source prefix without initializing the destination tail.
use super::bit_reader::BitReader;
use super::stream_input::SourceInput;
use std::mem::MaybeUninit;

/// # Safety
/// Input is the reader's same live initialized anchor; destination is a live
/// nonoverlapping source memcpy extent. Reader scalar/shift/count domains
/// match the original. No source input/destination mutation or callback may
/// run during this loan. Only written destination bytes become initialized.
pub unsafe fn copy_reader_bytes(
    destination: &mut [MaybeUninit<u8>],
    reader: &mut BitReader<'_>,
    input: SourceInput<'_>,
    mut num: usize,
) {
    assert!(num <= destination.len() && num <= reader.remaining_bytes());
    let mut out = 0;
    while reader.available_bits() >= 8 && num > 0 {
        destination[out].write(reader.bits_unmasked() as u8);
        reader.drop_bits(8);
        out += 1;
        num -= 1;
    }
    reader.normalize();
    if num > 0 {
        // SAFETY: The caller proves same-anchor identity/nonoverlap. Checked
        // source/destination extents plus the actual initialized input cover
        // exactly the source memcpy, retaining all unwritten tail bytes.
        unsafe {
            std::ptr::copy_nonoverlapping(
                input
                    .pointer_at(reader.position())
                    .cast::<MaybeUninit<u8>>(),
                destination.as_mut_ptr().add(out),
                num,
            )
        };
        reader.drop_bytes(num);
    }
}
