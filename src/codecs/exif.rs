//! Minimal, bounds-checked TIFF IFD0 orientation reader shared by the JPEG
//! and WebP adapters.

/// Read the IFD0 `Orientation` tag (`0x0112`) from EXIF data starting at its
/// TIFF header (`II` or `MM`). `Ok(None)` means the tag is absent; `Err` means
/// the TIFF structure needed to find it is malformed.
pub(crate) fn orientation(tiff: &[u8]) -> Result<Option<u16>, ()> {
    let (header, _) = tiff.split_first_chunk::<8>().ok_or(())?;
    let little = match &header[..2] {
        b"II" => true,
        b"MM" => false,
        _ => return Err(()),
    };
    let to_u16 = |bytes: [u8; 2]| {
        if little {
            u16::from_le_bytes(bytes)
        } else {
            u16::from_be_bytes(bytes)
        }
    };
    let to_u32 = |bytes: [u8; 4]| {
        if little {
            u32::from_le_bytes(bytes)
        } else {
            u32::from_be_bytes(bytes)
        }
    };
    if to_u16([header[2], header[3]]) != 42 {
        return Err(());
    }
    let ifd =
        usize::try_from(to_u32([header[4], header[5], header[6], header[7]])).map_err(|_| ())?;
    let (count_bytes, mut rest) = tiff
        .get(ifd..)
        .and_then(<[u8]>::split_first_chunk::<2>)
        .ok_or(())?;
    let entries = usize::from(to_u16(*count_bytes));
    for _ in 0..entries {
        let Some((entry, tail)) = rest.split_first_chunk::<12>() else {
            return Err(());
        };
        rest = tail;
        if to_u16([entry[0], entry[1]]) != 0x0112 {
            continue;
        }
        // One value, stored inline. EXIF specifies SHORT (type 3); real
        // camera files also store it as LONG (type 4), which TIFF readers
        // accept for integer fields. Any other type or count is malformed.
        if to_u32([entry[4], entry[5], entry[6], entry[7]]) != 1 {
            return Err(());
        }
        return match to_u16([entry[2], entry[3]]) {
            3 => Ok(Some(to_u16([entry[8], entry[9]]))),
            4 => u16::try_from(to_u32([entry[8], entry[9], entry[10], entry[11]]))
                .map(Some)
                .map_err(|_| ()),
            _ => Err(()),
        };
    }
    Ok(None)
}

#[cfg(test)]
mod tests {
    use super::orientation;

    fn put16(data: &mut Vec<u8>, little: bool, value: u16) {
        data.extend_from_slice(&if little {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        });
    }

    fn put32(data: &mut Vec<u8>, little: bool, value: u32) {
        data.extend_from_slice(&if little {
            value.to_le_bytes()
        } else {
            value.to_be_bytes()
        });
    }

    /// Little- or big-endian TIFF with one IFD0 entry.
    fn tiff(little: bool, tag: u16, kind: u16, count: u32, value: u16) -> Vec<u8> {
        let mut data = if little {
            b"II".to_vec()
        } else {
            b"MM".to_vec()
        };
        put16(&mut data, little, 42);
        put32(&mut data, little, 8);
        put16(&mut data, little, 1);
        put16(&mut data, little, tag);
        put16(&mut data, little, kind);
        put32(&mut data, little, count);
        put16(&mut data, little, value);
        data.extend_from_slice(&[0, 0, 0, 0, 0, 0]);
        data
    }

    #[test]
    fn reads_orientation_in_both_byte_orders() {
        assert_eq!(orientation(&tiff(true, 0x0112, 3, 1, 6)), Ok(Some(6)));
        assert_eq!(orientation(&tiff(false, 0x0112, 3, 1, 8)), Ok(Some(8)));
        assert_eq!(orientation(&tiff(true, 0x010f, 2, 4, 0)), Ok(None));
    }

    #[test]
    fn reads_an_orientation_stored_as_long_and_after_earlier_tags() {
        // Some cameras write the tag as LONG; such photos must not fail closed.
        assert_eq!(orientation(&tiff(true, 0x0112, 4, 1, 6)), Ok(Some(6)));
        // Big-endian LONG: the value sits in the last two of the four bytes.
        let mut big = tiff(false, 0x0112, 4, 1, 0);
        big[21] = 8;
        assert_eq!(orientation(&big), Ok(Some(8)));

        // Orientation as the second entry in IFD0.
        let mut multi = b"II".to_vec();
        put16(&mut multi, true, 42);
        put32(&mut multi, true, 8);
        put16(&mut multi, true, 2);
        // Entry 0: unrelated tag 0x010f
        put16(&mut multi, true, 0x010f);
        put16(&mut multi, true, 2);
        put32(&mut multi, true, 4);
        put32(&mut multi, true, 0);
        // Entry 1: Orientation = 5
        put16(&mut multi, true, 0x0112);
        put16(&mut multi, true, 3);
        put32(&mut multi, true, 1);
        put16(&mut multi, true, 5);
        put16(&mut multi, true, 0);
        assert_eq!(orientation(&multi), Ok(Some(5)));

        // Empty IFD0 reports absent orientation.
        let mut empty = b"MM".to_vec();
        put16(&mut empty, false, 42);
        put32(&mut empty, false, 8);
        put16(&mut empty, false, 0);
        assert_eq!(orientation(&empty), Ok(None));
    }

    #[test]
    fn rejects_malformed_tiff_structures() {
        assert_eq!(orientation(b""), Err(()));
        assert_eq!(orientation(b"XX*\0"), Err(()));
        assert_eq!(orientation(b"II*\0\x08\0\0"), Err(()));
        let mut magic = tiff(true, 0x0112, 3, 1, 1);
        magic[2] = 41;
        assert_eq!(orientation(&magic), Err(()));
        // A RATIONAL or a second value is not an orientation.
        assert_eq!(orientation(&tiff(true, 0x0112, 5, 1, 1)), Err(()));
        assert_eq!(orientation(&tiff(true, 0x0112, 4, 2, 1)), Err(()));
        // A LONG that does not fit an orientation code is malformed.
        let mut wide = tiff(true, 0x0112, 4, 1, 1);
        wide[20] = 1;
        assert_eq!(orientation(&wide), Err(()));
        let truncated = tiff(true, 0x0112, 3, 1, 1);
        assert_eq!(orientation(&truncated[..12]), Err(()));
        // Truncated non-orientation entry is also malformed.
        let truncated_other = tiff(true, 0x010f, 2, 4, 0);
        assert_eq!(orientation(&truncated_other[..18]), Err(()));
    }

    #[test]
    fn rejects_huge_ifd_offsets_and_entry_counts_without_overflow() {
        for offset in [u32::MAX, u32::MAX - 1, u32::MAX - 11, 1024] {
            let mut data = b"II".to_vec();
            put16(&mut data, true, 42);
            put32(&mut data, true, offset);
            assert_eq!(orientation(&data), Err(()), "offset={offset:#x}");
        }

        // Large entry count on a short TIFF buffer must fail cleanly without overflow.
        let mut huge_count = b"II".to_vec();
        put16(&mut huge_count, true, 42);
        put32(&mut huge_count, true, 8);
        put16(&mut huge_count, true, u16::MAX);
        // One unrelated 12-byte entry, far fewer than u16::MAX.
        huge_count.extend_from_slice(&[0; 12]);
        assert_eq!(orientation(&huge_count), Err(()));
    }
}
