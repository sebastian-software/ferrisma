use crate::quality::{Filter, footprint, taps, taps_into};

#[test]
fn exact_area_average() {
    let samples = taps(4, 1, 0, Filter::Area).unwrap();
    assert_eq!(samples.len(), 4);
    for (x, tap) in samples.iter().enumerate() {
        assert_eq!(tap.index as usize, x);
        assert!((tap.weight - 0.25).abs() < 1.0e-12);
    }
}

#[test]
fn lanczos_weights_preserve_flat_color_at_edges() {
    for input in [1, 2, 7, 73, 512] {
        for output in [1, 3, 16, 91] {
            for y in 0..output {
                let samples = taps(input, output, y, Filter::Lanczos3).unwrap();
                let sum: f64 = samples.iter().map(|tap| tap.weight).sum();
                assert!((sum - 1.0).abs() < 1.0e-10);
                assert!(samples.iter().all(|tap| tap.index < input));
            }
        }
    }
}

#[test]
fn planner_footprint_covers_antialiasing_halo() {
    let (left, width) = footprint(4_000, 40, 10, 2, Filter::Lanczos3).unwrap();
    assert!(left < 1_000);
    assert!(left + width > 1_200);
    assert!(left + width <= 4_000);
    assert_eq!(footprint(7, 7, 0, 7, Filter::Area).unwrap(), (0, 7));
}

#[test]
fn geometry_fails_closed() {
    assert!(taps(0, 2, 0, Filter::Area).is_err());
    assert!(taps(10, 0, 0, Filter::Area).is_err());
    assert!(taps(10, 2, 2, Filter::Area).is_err());
    assert!(footprint(10, 2, 1, 2, Filter::Lanczos3).is_err());
}

#[test]
fn lanczos_zero_crossings_do_not_hide_interior_tap_indices() {
    // Input height 3 and output height 7 has a Lanczos zero at output
    // position 3. Position 4 then references input row zero again, so
    // endpoint-only footprint planning could underflow or underfetch.
    assert_eq!(footprint(3, 7, 3, 1, Filter::Lanczos3).unwrap(), (1, 1));
    assert_eq!(footprint(3, 7, 3, 2, Filter::Lanczos3).unwrap(), (0, 3));
    assert_eq!(footprint(3, 7, 2, 3, Filter::Lanczos3).unwrap(), (0, 3));

    for input in [1, 2, 3, 7, 23] {
        for output in [1, 3, 7, 11] {
            for start in 0..output {
                for count in 1..=output - start {
                    let (first, width) =
                        footprint(input, output, start, count, Filter::Lanczos3).unwrap();
                    for destination in start..start + count {
                        for tap in taps(input, output, destination, Filter::Lanczos3).unwrap() {
                            assert!(tap.index >= first);
                            assert!(tap.index < first + width);
                        }
                    }
                }
            }
        }
    }
}

/// Brute-force reference footprint computed by scanning `taps()` for every
/// output position in `start..start + count`.
fn reference_footprint(
    input: u32,
    output: u32,
    start: u32,
    count: u32,
    filter: Filter,
) -> (u32, u32) {
    let mut first = u32::MAX;
    let mut last = 0_u32;
    for at in start..start + count {
        let weights = taps(input, output, at, filter).unwrap();
        first = first.min(weights.first().unwrap().index);
        last = last.max(weights.last().unwrap().index);
    }
    (first, last - first + 1)
}

#[test]
fn footprint_matches_exhaustive_taps_scan_for_both_filters() {
    for filter in [Filter::Area, Filter::Lanczos3] {
        for input in [1, 2, 3, 4, 5, 7, 8, 9, 13, 17, 23, 31] {
            for output in [1, 2, 3, 4, 5, 7, 8, 11, 16, 29] {
                for start in 0..output {
                    for count in 1..=output - start {
                        let actual = footprint(input, output, start, count, filter).unwrap();
                        let expected = reference_footprint(input, output, start, count, filter);
                        assert_eq!(
                            actual, expected,
                            "filter={filter:?} input={input} output={output} start={start} count={count}"
                        );
                    }
                }
            }
        }
        // Larger ratios and interior ROIs, including exact 1:1.
        for (input, output, start, count) in [
            (6000, 6000, 0, 6000),
            (6000, 6000, 123, 456),
            (6000, 1600, 0, 1600),
            (6000, 1600, 317, 211),
            (1600, 6000, 0, 6000),
            (1600, 6000, 1500, 900),
            (4000, 3, 1, 2),
            (3, 4000, 1998, 5),
        ] {
            let actual = footprint(input, output, start, count, filter).unwrap();
            let expected = reference_footprint(input, output, start, count, filter);
            assert_eq!(
                actual, expected,
                "filter={filter:?} input={input} output={output} start={start} count={count}"
            );
        }
    }
}

#[test]
fn taps_into_reuses_buffer_and_matches_taps() {
    let mut scratch = Vec::new();
    for filter in [Filter::Area, Filter::Lanczos3] {
        for (input, output) in [(1, 1), (7, 7), (23, 7), (7, 23), (100, 37)] {
            for at in 0..output {
                taps_into(input, output, at, filter, &mut scratch).unwrap();
                let expected = taps(input, output, at, filter).unwrap();
                assert_eq!(
                    scratch, expected,
                    "filter={filter:?} input={input} output={output} at={at}"
                );
            }
        }
    }
    assert!(taps_into(0, 4, 0, Filter::Area, &mut scratch).is_err());
    assert!(taps_into(4, 0, 0, Filter::Area, &mut scratch).is_err());
    assert!(taps_into(4, 4, 4, Filter::Area, &mut scratch).is_err());
}

#[test]
fn footprint_handles_u32_max_bounds_in_constant_time() {
    assert_eq!(
        footprint(u32::MAX, u32::MAX, 100, 50, Filter::Lanczos3).unwrap(),
        (100, 50)
    );
    assert_eq!(
        footprint(1, u32::MAX, 1_000_000, 1_000, Filter::Lanczos3).unwrap(),
        (0, 1)
    );
    let (start, width) = footprint(u32::MAX, 10, 5, 2, Filter::Lanczos3).unwrap();
    assert!(width > 0);
    assert!(start.checked_add(width).is_some());
    assert!(footprint(0, 10, 0, 1, Filter::Area).is_err());
}
