// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "common/skcms_parser.rs"]
pub mod cases;
use jxl::skcms::{
    curve::eval_curve,
    profile::{self, IccProfile},
};
use std::mem::{MaybeUninit, size_of};

#[test]
fn native_original_parser_captures_retain_every_observed_word() {
    let captures = std::fs::read_to_string(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/tests/data/skcms-parser-native.tsv"
    ))
    .expect("independent original captures");
    let rows: Vec<_> = captures.lines().collect();
    assert_eq!(rows.len(), 12, "native source capture inventory");
    for row in rows {
        let (request, expected) = row.split_once('\t').unwrap();
        assert_eq!(
            cases::observe(request),
            expected,
            "native original request: {request}"
        );
    }
}
#[test]
fn short_parse_initializes_every_output_byte_before_returning() {
    for len in [0, 1, 131] {
        let mut storage = MaybeUninit::<IccProfile>::uninit();
        // SAFETY: this initializes untyped object storage, without constructing
        // invalid bools or pointers. Source Parse overwrites the complete object
        // before its short-length return; no prior value is read or dropped.
        unsafe {
            std::ptr::write_bytes(
                storage.as_mut_ptr().cast::<u8>(),
                0xa5,
                size_of::<IccProfile>(),
            );
            assert!(!profile::parse(std::ptr::null(), len, storage.as_mut_ptr()));
            let bytes =
                std::slice::from_raw_parts(storage.as_ptr().cast::<u8>(), size_of::<IccProfile>());
            assert!(bytes.iter().all(|&byte| byte == 0));
        }
    }
}
#[test]
fn retained_curve_pointers_observe_the_same_moved_and_mutated_icc_owner() {
    for mut data in cases::synthetic_profiles() {
        let mut parsed = IccProfile::zeroed();
        // SAFETY: the entire fixture is live, output is disjoint, and source
        // guards check all tag and table extents before returning this profile.
        if !unsafe { profile::parse(data.as_ptr(), data.len(), &raw mut parsed) } {
            continue;
        }
        // SAFETY: Parse initialized every union byte. The count is its common
        // prefix, and the selected two-entry 16-bit table has four live bytes.
        unsafe {
            if parsed.trc[0].table.table_entries != 2 || parsed.trc[0].table.table_16.is_null() {
                continue;
            }
            let table = parsed.trc[0].table.table_16;
            let offset = (table as usize) - (data.as_ptr() as usize);
            assert!(offset + 4 <= data.len());
            assert_eq!(parsed.buffer, data.as_ptr());
            let before = eval_curve(std::ptr::addr_of!(parsed.trc[0]), 0.0);
            data[offset..offset + 2].copy_from_slice(&u16::MAX.to_be_bytes());
            let moved_owner = data;
            let moved_profile = parsed;
            assert_eq!(moved_profile.buffer, moved_owner.as_ptr());
            assert_eq!(moved_profile.trc[0].table.table_16, table);
            assert_eq!(
                eval_curve(std::ptr::addr_of!(moved_profile.trc[0]), 0.0),
                1.0
            );
            assert_ne!(before, 1.0);
            return;
        }
    }
    panic!("source fixture census did not reach the borrowed two-entry table");
}
