// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Distribution assertions adapted from lib/jxl/ans_common_test.cc.

use jxl::jxl::ans_common::{AliasTable, Entry, init_alias_table};

#[test]
fn alias_distribution_smoke_preserves_frequency_and_all_offsets() {
    for distribution in [vec![2048, 2048], vec![4096], vec![0, 0, 0, 4096, 0]] {
        let mut table = vec![Entry::default(); 256];
        init_alias_table(distribution.clone(), 12, 8, &mut table).unwrap();
        let mut offsets = vec![Vec::new(); distribution.len()];
        for i in 0..4096 {
            let symbol = AliasTable::lookup(&table, i, 4, 15);
            offsets[symbol.value].push(symbol.offset);
            assert_eq!(symbol.freq, distribution[symbol.value] as usize);
        }
        for (i, offsets) in offsets.iter_mut().enumerate() {
            assert_eq!(offsets.len(), distribution[i] as usize);
            offsets.sort_unstable();
            for (j, offset) in offsets.iter().enumerate() {
                assert_eq!(*offset, j);
            }
        }
    }
}

#[test]
fn alias_entry_layout_matches_the_eight_byte_packed_source() {
    assert_eq!(std::mem::size_of::<Entry>(), 8);
    assert_eq!(std::mem::align_of::<Entry>(), 1);
}
