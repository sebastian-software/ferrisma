// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
//! Independently captured native original concatenation operand regressions.

#[cfg(any(
    target_arch = "aarch64",
    all(target_arch = "x86_64", target_os = "linux")
))]
#[test]
fn concatenation_retains_each_native_competing_nan_product() {
    use jxl::skcms::types::Matrix3x3;
    let requests = include_str!("data/skcms-concat-paired-nan.requests");
    #[cfg(target_arch = "aarch64")]
    let captures = include_bytes!("data/skcms-concat-paired-nan-arm.words");
    #[cfg(all(target_arch = "x86_64", target_os = "linux"))]
    let captures = include_bytes!("data/skcms-concat-paired-nan-x86.words");
    assert_eq!(requests.lines().count(), 432);
    assert_eq!(captures.len(), 432 * 9 * 4);
    for (index, (request, captured)) in requests
        .lines()
        .zip(captures.as_chunks::<36>().0)
        .enumerate()
    {
        let mut args = request.split_whitespace();
        assert_eq!(args.next(), Some("skcms-concat"));
        assert_eq!(args.next(), Some("0"));
        let mut a = Matrix3x3::default();
        let mut b = Matrix3x3::default();
        for matrix in [&mut a, &mut b] {
            for row in &mut matrix.vals {
                for word in row {
                    *word = f32::from_bits(args.next().unwrap().parse().unwrap());
                }
            }
        }
        assert!(args.next().is_none());
        let actual = a.concat(&b);
        for (value, expected) in actual
            .vals
            .iter()
            .flatten()
            .zip(captured.as_chunks::<4>().0)
        {
            assert_eq!(
                value.to_bits(),
                u32::from_le_bytes(*expected),
                "native request {index}"
            );
        }
    }
}
