// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// FastLog2's error scenario adapts fast_math_test.cc with a local bounded RNG.
// The prefix-clustering regression is checked against the pinned C++ oracle.

use jxl::jxl::base::fast_math::fast_log2f;
use jxl::jxl::enc_ans_params::{ClusteringType, Histogram, HistogramParams};
use jxl::jxl::enc_cluster::cluster_histograms;

#[test]
fn fast_log_keeps_the_upstream_error_bound_in_its_test_range() {
    let mut seed = 1u64;
    for _ in 0..65536 {
        seed ^= seed << 13;
        seed ^= seed >> 7;
        seed ^= seed << 17;
        let value = 1e-7_f32 + (seed as u32 as f32 / u32::MAX as f32) * (1e3_f32 - 1e-7_f32);
        assert!((value.log2() - fast_log2f(value)).abs() < 3.1e-6, "{value}");
    }
}

#[test]
fn prefix_clustering_keeps_the_large_singleton_separate() {
    for prefix in [false, true] {
        let mut input = vec![
            Histogram {
                counts: vec![100000],
                total_count: 100000,
                entropy: 0.0,
            },
            Histogram {
                counts: vec![10000, 1],
                total_count: 10001,
                entropy: 0.0,
            },
        ];
        let params = HistogramParams {
            clustering: ClusteringType::Fast,
            force_huffman: prefix,
            ..HistogramParams::default()
        };
        let mut out = Vec::new();
        let mut symbols = Vec::new();
        assert_eq!(
            cluster_histograms(&params, &mut input, 2, &mut out, &mut symbols),
            Ok(())
        );
        if prefix {
            assert_eq!(symbols, [0, 1]);
            assert_eq!(out.len(), 2);
        } else {
            assert_eq!(symbols, [0, 0]);
            assert_eq!(out[0].counts, [110000, 1]);
        }
    }
}
