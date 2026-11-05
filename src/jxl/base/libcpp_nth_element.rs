// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// Rust translation; see licenses/LLVM-Apache-2.0.txt and LLVM-exception.txt.

//! The reference libc++ `std::nth_element(first, nth, last)` for a contiguous
//! `float` range with the default `__less<>` comparator, from
//! `__algorithm/nth_element.h`. Its `__sort3` is the branchless arithmetic
//! specialization of `__algorithm/sort.h` (`__cond_swap`,
//! `__partially_sorted_swap`), and `__selection_sort` uses `__min_element`
//! from `__algorithm/min_element.h`. Source snapshots and hashes are in
//! tools/reference/libcxx. Debug range randomization is disabled in the
//! reference configuration. The comparator keeps the source's strict weak
//! ordering precondition, which NaN operands violate.

fn less(a: f32, b: f32) -> bool {
    a < b
}

// Ensures that less(v[x], v[y]) by swapping if necessary; returns !less.
fn cond_swap(v: &mut [f32], x: usize, y: usize) -> bool {
    let r = less(v[x], v[y]);
    let tmp = if r { v[x] } else { v[y] };
    v[y] = if r { v[y] } else { v[x] };
    v[x] = tmp;
    !r
}

// Orders v[x], v[y], v[z] under the assumption that v[y] <= v[z].
fn partially_sorted_swap(v: &mut [f32], x: usize, y: usize, z: usize) -> bool {
    let r1 = less(v[z], v[x]);
    let tmp = if r1 { v[z] } else { v[x] };
    v[z] = if r1 { v[x] } else { v[z] };
    let r2 = less(tmp, v[y]);
    v[x] = if r2 { v[x] } else { v[y] };
    v[y] = if r2 { v[y] } else { tmp };
    !r1 || !r2
}

fn sort3(v: &mut [f32], x1: usize, x2: usize, x3: usize) -> bool {
    let swapped1 = cond_swap(v, x2, x3);
    let swapped2 = partially_sorted_swap(v, x1, x2, x3);
    swapped1 || swapped2
}

fn min_element(v: &[f32], mut first: usize, last: usize) -> usize {
    if first == last {
        return first;
    }
    let mut i = first;
    loop {
        i += 1;
        if i == last {
            break;
        }
        if less(v[i], v[first]) {
            first = i;
        }
    }
    first
}

fn selection_sort(v: &mut [f32], mut first: usize, last: usize) {
    let lm1 = last - 1;
    while first != lm1 {
        let i = min_element(v, first, last);
        if i != first {
            v.swap(first, i);
        }
        first += 1;
    }
}

// Manually guard downward moving j against i.
fn find_guard(v: &[f32], i: usize, j: &mut usize, m: usize) -> bool {
    loop {
        *j -= 1;
        if i == *j {
            return false;
        }
        if less(v[*j], v[m]) {
            return true;
        }
    }
}

/// `std::nth_element(v.begin(), v.begin() + nth, v.end())`.
pub fn nth_element(v: &mut [f32], nth: usize) {
    let mut first = 0usize;
    let mut last = v.len();
    if nth == last {
        return;
    }
    const LIMIT: usize = 7;
    loop {
        if nth == last {
            return;
        }
        let len = last - first;
        match len {
            0 | 1 => return,
            2 => {
                last -= 1;
                if less(v[last], v[first]) {
                    v.swap(first, last);
                }
                return;
            }
            3 => {
                sort3(v, first, first + 1, last - 1);
                return;
            }
            _ => {}
        }
        if len <= LIMIT {
            selection_sort(v, first, last);
            return;
        }
        // len > LIMIT >= 3
        let mut m = first + len / 2;
        let lm1 = last - 1;
        let mut n_swaps = u32::from(sort3(v, first, m, lm1));
        // *m is median; partition [first, m) < *m and *m <= [m, last).
        let mut i = first;
        let mut j = lm1;
        // j points beyond range to be tested, *lm1 is known to be <= *m.
        // The search going up is known to be guarded but the search coming
        // down isn't. Prime the downward search with a guard.
        if !less(v[i], v[m]) {
            // *first == *m, *first doesn't go in first part.
            if find_guard(v, i, &mut j, m) {
                v.swap(i, j);
                n_swaps += 1;
            } else {
                // *first == *m, *m <= all other elements. Partition instead
                // into [first, i) == *first and *first < [i, last).
                i += 1;
                j = last;
                j -= 1;
                if !less(v[first], v[j]) {
                    // We need a guard if *first == *(last - 1).
                    loop {
                        if i == j {
                            return; // [first, last) all equivalent elements
                        } else if less(v[first], v[i]) {
                            v.swap(i, j);
                            n_swaps += 1;
                            i += 1;
                            break;
                        }
                        i += 1;
                    }
                }
                // [first, i) == *first and *first < [j, last), j == last - 1.
                if i == j {
                    return;
                }
                loop {
                    while !less(v[first], v[i]) {
                        i += 1;
                    }
                    loop {
                        j -= 1;
                        if !less(v[first], v[j]) {
                            break;
                        }
                    }
                    if i >= j {
                        break;
                    }
                    v.swap(i, j);
                    n_swaps += 1;
                    i += 1;
                }
                // [first, i) == *first and *first < [i, last). The first
                // part is sorted.
                if nth < i {
                    return;
                }
                // nth_element the second part.
                first = i;
                continue;
            }
        }
        i += 1;
        // j points beyond range to be tested, *lm1 is known to be <= *m.
        // If not yet partitioned...
        if i < j {
            // Known that *(i - 1) < *m.
            loop {
                // m still guards upward moving i.
                while less(v[i], v[m]) {
                    i += 1;
                }
                // It is now known that a guard exists for downward moving j.
                loop {
                    j -= 1;
                    if less(v[j], v[m]) {
                        break;
                    }
                }
                if i >= j {
                    break;
                }
                v.swap(i, j);
                n_swaps += 1;
                // It is known that m != j. If m just moved, follow it.
                if m == i {
                    m = j;
                }
                i += 1;
            }
        }
        // [first, i) < *m and *m <= [i, last)
        if i != m && less(v[m], v[i]) {
            v.swap(i, m);
            n_swaps += 1;
        }
        // [first, i) < *i and *i <= [i + 1, last)
        if nth == i {
            return;
        }
        if n_swaps == 0 {
            // We were given a perfectly partitioned sequence. Coincidence?
            if nth < i {
                // Check for [first, i) already sorted.
                j = first;
                m = first;
                loop {
                    j += 1;
                    if j == i {
                        // [first, i) sorted
                        return;
                    }
                    if less(v[j], v[m]) {
                        // Not yet sorted, so sort.
                        break;
                    }
                    m = j;
                }
            } else {
                // Check for [i, last) already sorted.
                j = i;
                m = i;
                loop {
                    j += 1;
                    if j == last {
                        // [i, last) sorted
                        return;
                    }
                    if less(v[j], v[m]) {
                        // Not yet sorted, so sort.
                        break;
                    }
                    m = j;
                }
            }
        }
        // nth_element on range containing nth.
        if nth < i {
            last = i;
        } else {
            i += 1;
            first = i;
        }
    }
}
