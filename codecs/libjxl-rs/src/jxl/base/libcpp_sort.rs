// Part of the LLVM Project, under the Apache License v2.0 with LLVM Exceptions.
// SPDX-License-Identifier: Apache-2.0 WITH LLVM-exception
// Rust translation; see licenses/LLVM-Apache-2.0.txt and LLVM-exception.txt.

//! The reference libc++ non-arithmetic std::sort specialization, including its
//! heap fallback. Source snapshots and hashes are in tools/reference/libcxx.
//! Preserve equivalent-element order for libjxl's palette comparators. This is
//! not a replacement for other std::sort specializations. Comparators retain
//! the source's strict-weak-ordering precondition. Default values occupy only
//! moved-from holes, which are never compared or exposed to callers.

use std::mem::take;

fn sort3<T, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    x: usize,
    y: usize,
    z: usize,
    c: &mut F,
) -> bool {
    if !c(&v[y], &v[x]) {
        if !c(&v[z], &v[y]) {
            return false;
        }
        v.swap(y, z);
        if c(&v[y], &v[x]) {
            v.swap(x, y);
        }
        return true;
    }
    if c(&v[z], &v[y]) {
        v.swap(x, z);
        return true;
    }
    v.swap(x, y);
    if c(&v[z], &v[y]) {
        v.swap(y, z);
    }
    true
}
fn sort4<T, F: FnMut(&T, &T) -> bool>(v: &mut [T], a: [usize; 4], c: &mut F) {
    let [x1, x2, x3, x4] = a;
    sort3(v, x1, x2, x3, c);
    if c(&v[x4], &v[x3]) {
        v.swap(x3, x4);
        if c(&v[x3], &v[x2]) {
            v.swap(x2, x3);
            if c(&v[x2], &v[x1]) {
                v.swap(x1, x2);
            }
        }
    }
}
fn sort5<T, F: FnMut(&T, &T) -> bool>(v: &mut [T], a: [usize; 5], c: &mut F) {
    let [x1, x2, x3, x4, x5] = a;
    sort4(v, [x1, x2, x3, x4], c);
    if c(&v[x5], &v[x4]) {
        v.swap(x4, x5);
        if c(&v[x4], &v[x3]) {
            v.swap(x3, x4);
            if c(&v[x3], &v[x2]) {
                v.swap(x2, x3);
                if c(&v[x2], &v[x1]) {
                    v.swap(x1, x2);
                }
            }
        }
    }
}
fn small<T, F: FnMut(&T, &T) -> bool>(v: &mut [T], first: usize, last: usize, c: &mut F) -> bool {
    match last - first {
        0 | 1 => (),
        2 => {
            if c(&v[last - 1], &v[first]) {
                v.swap(first, last - 1);
            }
        }
        3 => {
            sort3(v, first, first + 1, last - 1, c);
        }
        4 => sort4(v, [first, first + 1, first + 2, last - 1], c),
        5 => sort5(v, [first, first + 1, first + 2, first + 3, last - 1], c),
        _ => return false,
    }
    true
}
fn insertion_sort<T: Default, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    first: usize,
    last: usize,
    c: &mut F,
    guarded: bool,
) {
    if first == last {
        return;
    }
    for i in first + 1..last {
        let mut j = i - 1;
        if c(&v[i], &v[j]) {
            let t = take(&mut v[i]);
            let mut k = j;
            j = i;
            loop {
                v[j] = take(&mut v[k]);
                j = k;
                if guarded && j == first {
                    break;
                }
                k -= 1;
                if !c(&t, &v[k]) {
                    break;
                }
            }
            v[j] = t;
        }
    }
}
fn insertion_sort_incomplete<T: Default, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    first: usize,
    last: usize,
    c: &mut F,
) -> bool {
    if small(v, first, last, c) {
        return true;
    }
    let mut j = first + 2;
    sort3(v, first, first + 1, j, c);
    let mut count = 0;
    for i in j + 1..last {
        if c(&v[i], &v[j]) {
            let t = take(&mut v[i]);
            let mut k = j;
            j = i;
            loop {
                v[j] = take(&mut v[k]);
                j = k;
                if j == first {
                    break;
                }
                k -= 1;
                if !c(&t, &v[k]) {
                    break;
                }
            }
            v[j] = t;
            count += 1;
            if count == 8 {
                return i + 1 == last;
            }
        }
        j = i;
    }
    true
}
fn partition_right<T: Default, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    mut first: usize,
    mut last: usize,
    c: &mut F,
) -> (usize, bool) {
    let begin = first;
    let pivot = take(&mut v[first]);
    loop {
        first += 1;
        if !c(&v[first], &pivot) {
            break;
        }
    }
    if begin == first - 1 {
        while first < last {
            last -= 1;
            if c(&v[last], &pivot) {
                break;
            }
        }
    } else {
        loop {
            last -= 1;
            if c(&v[last], &pivot) {
                break;
            }
        }
    }
    let already_partitioned = first >= last;
    while first < last {
        v.swap(first, last);
        loop {
            first += 1;
            if !c(&v[first], &pivot) {
                break;
            }
        }
        loop {
            last -= 1;
            if c(&v[last], &pivot) {
                break;
            }
        }
    }
    let pos = first - 1;
    if begin != pos {
        v[begin] = take(&mut v[pos]);
    }
    v[pos] = pivot;
    (pos, already_partitioned)
}
fn partition_left<T: Default, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    mut first: usize,
    mut last: usize,
    c: &mut F,
) -> usize {
    let begin = first;
    let pivot = take(&mut v[first]);
    if c(&pivot, &v[last - 1]) {
        loop {
            first += 1;
            if c(&pivot, &v[first]) {
                break;
            }
        }
    } else {
        loop {
            first += 1;
            if first >= last || c(&pivot, &v[first]) {
                break;
            }
        }
    }
    if first < last {
        loop {
            last -= 1;
            if !c(&pivot, &v[last]) {
                break;
            }
        }
    }
    while first < last {
        v.swap(first, last);
        loop {
            first += 1;
            if c(&pivot, &v[first]) {
                break;
            }
        }
        loop {
            last -= 1;
            if !c(&pivot, &v[last]) {
                break;
            }
        }
    }
    let pos = first - 1;
    if begin != pos {
        v[begin] = take(&mut v[pos]);
    }
    v[pos] = pivot;
    first
}
fn sift_down<T: Default, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    len: usize,
    mut start: usize,
    c: &mut F,
) {
    if len < 2 || (len - 2) / 2 < start {
        return;
    }
    let mut child = 2 * start + 1;
    if child + 1 < len && c(&v[child], &v[child + 1]) {
        child += 1;
    }
    if c(&v[child], &v[start]) {
        return;
    }
    let top = take(&mut v[start]);
    loop {
        v[start] = take(&mut v[child]);
        start = child;
        if (len - 2) / 2 < child {
            break;
        }
        child = 2 * child + 1;
        if child + 1 < len && c(&v[child], &v[child + 1]) {
            child += 1;
        }
        if c(&v[child], &top) {
            break;
        }
    }
    v[start] = top;
}
fn sift_up<T: Default, F: FnMut(&T, &T) -> bool>(v: &mut [T], mut len: usize, c: &mut F) {
    if len <= 1 {
        return;
    }
    let mut last = len - 1;
    len = (len - 2) / 2;
    if c(&v[len], &v[last]) {
        let t = take(&mut v[last]);
        loop {
            v[last] = take(&mut v[len]);
            last = len;
            if len == 0 {
                break;
            }
            len = (len - 1) / 2;
            if !c(&v[len], &t) {
                break;
            }
        }
        v[last] = t;
    }
}
fn heap_sort<T: Default, F: FnMut(&T, &T) -> bool>(v: &mut [T], c: &mut F) {
    let n = v.len();
    if n > 1 {
        for start in (0..=(n - 2) / 2).rev() {
            sift_down(v, n, start, c);
        }
    }
    for len in (2..=n).rev() {
        let top = take(&mut v[0]);
        let mut hole = 0;
        let mut child = 0;
        // Original Floyd sift-down always descends to a leaf.
        loop {
            child = 2 * child + 1;
            if child + 1 < len && c(&v[child], &v[child + 1]) {
                child += 1;
            }
            v[hole] = take(&mut v[child]);
            hole = child;
            if child > (len - 2) / 2 {
                break;
            }
        }
        let last = len - 1;
        if hole == last {
            v[hole] = top;
        } else {
            v[hole] = take(&mut v[last]);
            v[last] = top;
            sift_up(v, hole + 1, c);
        }
    }
}
pub fn introsort<T: Default, F: FnMut(&T, &T) -> bool>(
    v: &mut [T],
    mut first: usize,
    mut last: usize,
    c: &mut F,
    mut depth: usize,
    mut leftmost: bool,
) {
    loop {
        let len = last - first;
        if small(v, first, last, c) {
            return;
        }
        if len < 24 {
            insertion_sort(v, first, last, c, leftmost);
            return;
        }
        if depth == 0 {
            heap_sort(&mut v[first..last], c);
            return;
        }
        depth -= 1;
        let half = len / 2;
        if len > 128 {
            sort3(v, first, first + half, last - 1, c);
            sort3(v, first + 1, first + half - 1, last - 2, c);
            sort3(v, first + 2, first + half + 1, last - 3, c);
            sort3(v, first + half - 1, first + half, first + half + 1, c);
            v.swap(first, first + half);
        } else {
            sort3(v, first + half, first, last - 1, c);
        }
        if !leftmost && !c(&v[first - 1], &v[first]) {
            first = partition_left(v, first, last, c);
            continue;
        }
        let (i, already_partitioned) = partition_right(v, first, last, c);
        if already_partitioned {
            let fs = insertion_sort_incomplete(v, first, i, c);
            if insertion_sort_incomplete(v, i + 1, last, c) {
                if fs {
                    return;
                }
                last = i;
                continue;
            } else if fs {
                first = i + 1;
                continue;
            }
        }
        introsort(v, first, i, c, depth, leftmost);
        leftmost = false;
        first = i + 1;
    }
}
pub fn sort_by<T: Default, F: FnMut(&T, &T) -> bool>(v: &mut [T], mut c: F) {
    if v.is_empty() {
        return;
    }
    let depth = 2 * (usize::BITS - 1 - v.len().leading_zeros()) as usize;
    introsort(v, 0, v.len(), &mut c, depth, true);
}
