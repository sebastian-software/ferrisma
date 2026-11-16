// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// These exact words were captured from both actual pristine source contexts;
// this architecture-scoped test executes Rust without invoking a C++ process.
#![cfg(all(feature = "cpp-oracle", target_arch = "aarch64", target_os = "macos"))]
#[path = "common/default_cms.rs"]
mod observation;

/// A decimal token that is a binary32 NaN word (it parses as `u32`) or a
/// binary64 NaN word (it parses as `u64` but not as `u32`), as `nan_word` in
/// `tests/parity.rs`.
fn nan_word(token: &str) -> bool {
    match token.parse::<u32>() {
        Ok(bits) => f32::from_bits(bits).is_nan(),
        Err(_) => token
            .parse::<u64>()
            .is_ok_and(|bits| f64::from_bits(bits).is_nan()),
    }
}

/// ADR-0041 section 3 (D8): the same tokens, each identical, except that two
/// NaN words of the same width are equal whatever their sign and payload
/// (which payload survives a product of two NaNs depends on the operand order
/// the compiler picks). `JXL_PARITY_STRICT=1` restores exact text equality.
fn matches(port: &str, original: &str) -> bool {
    if port == original {
        return true;
    }
    if std::env::var_os("JXL_PARITY_STRICT").is_some_and(|v| v == "1") {
        return false;
    }
    let a: Vec<_> = port.split_whitespace().collect();
    let b: Vec<_> = original.split_whitespace().collect();
    a.len() == b.len()
        && a.iter().zip(&b).all(|(x, y)| {
            x == y
                || (nan_word(x)
                    && nan_word(y)
                    && x.parse::<u32>().is_ok() == y.parse::<u32>().is_ok())
        })
}

#[test]
fn default_cms_independent_original_arm64_observations() {
    let fixture = include_str!("data/default-cms-native-arm64.tsv");
    let mut count = 0;
    for (index, line) in fixture.lines().enumerate() {
        let (request, expected) = line.split_once('\t').unwrap();
        let observed = observation::observe(request);
        assert!(
            matches(&observed, expected),
            "native source case {index}\n  left (Rust): {observed}\n right (C++): {expected}"
        );
        count += 1;
    }
    assert_eq!(count, 16);
}

#[test]
fn the_comparison_equalizes_only_nan_words_of_the_same_width() {
    if std::env::var_os("JXL_PARITY_STRICT").is_some_and(|v| v == "1") {
        return;
    }
    assert!(matches(
        "9221397313971290112 9221353333506179072",
        "9221397313971290112 9221397313971290112"
    ));
    assert!(matches("2143289344 7", "4290772992 7"));
    assert!(!matches(
        "9221397313971290112 4607182418800017408",
        "9221397313971290112 9221397313971290112"
    ));
    assert!(!matches("2143289344", "9221397313971290112"));
    assert!(!matches("2139095040", "2143289344"));
    assert!(!matches("1 2", "1 3"));
    assert!(!matches("1", "1 2"));
}
