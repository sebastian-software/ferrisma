// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "../src/brotli/bit_reader.rs"]
pub mod bit_reader;
#[path = "../src/brotli/layout.rs"]
pub mod layout;
#[path = "../src/brotli/memory.rs"]
pub mod memory;
#[path = "../src/brotli/mtf.rs"]
pub mod mtf;

use bit_reader::Reg;
use layout::DecoderStateStorage;
use mtf::inverse_move_to_front_transform;
use std::mem::size_of;

unsafe extern "C" fn allocate(_: *mut std::ffi::c_void, size: usize) -> *mut std::ffi::c_void {
    assert_eq!(size, size_of::<DecoderStateStorage>());
    let layout = std::alloc::Layout::from_size_align(size, 16).unwrap();
    // SAFETY: fixed nonzero source state extent and native-compatible alignment.
    unsafe { std::alloc::alloc(layout).cast() }
}
unsafe extern "C" fn free(_: *mut std::ffi::c_void, ptr: *mut std::ffi::c_void) {
    if ptr.is_null() {
        return;
    }
    let layout = std::alloc::Layout::from_size_align(size_of::<DecoderStateStorage>(), 16).unwrap();
    // SAFETY: exact producing callback pair, once, after field borrows end.
    unsafe { std::alloc::dealloc(ptr.cast(), layout) };
}
fn owner(seed: Option<u8>) -> memory::Allocation {
    // SAFETY: static matched callbacks return exclusive source-compatible
    // blocks, need no opaque context and never inspect/reenter borrowed data.
    let allocator = unsafe {
        memory::Allocator::from_callbacks(Some(allocate), Some(free), std::ptr::null_mut())
    }
    .unwrap();
    let mut allocation = allocator
        .allocate(size_of::<DecoderStateStorage>())
        .unwrap();
    if let Some(seed) = seed {
        allocation.fill(seed);
    }
    allocation
}
fn state(owner: &mut memory::Allocation) -> &mut DecoderStateStorage {
    // SAFETY: actual source-sized/aligned callback block; all storage fields
    // are MaybeUninit, so this exclusive view reads/initializes no payload.
    unsafe {
        &mut *owner
            .storage_mut()
            .as_mut_ptr()
            .cast::<DecoderStateStorage>()
    }
}

#[test]
fn unknown_owner_first_call_initializes_the_full_list_before_any_read() {
    let mut allocation = owner(None);
    let s = state(&mut allocation);
    s.mtf_upper_bound.write(63);
    let mut v = [255];
    // SAFETY: actual StateInit upper bound 63 initializes list bytes 0..255;
    // scratch physical byte 3 is written before its -1 read. Other bytes unknown.
    unsafe { inverse_move_to_front_transform(&mut v, 1, &mut s.mtf_upper_bound, &mut s.mtf) };
    assert_eq!(v, [255]);
    // SAFETY: output checkpoint and all list bytes were written, excluding
    // word 0 bytes 0..2, which this fixture never observes or assumes initialized.
    unsafe {
        assert_eq!(s.mtf_upper_bound.assume_init(), 63);
        let bytes = std::slice::from_raw_parts(s.mtf.as_ptr().cast::<u8>().add(3), 257);
        assert_eq!(&bytes[..2], &[255, 255]);
        assert_eq!(&bytes[2..], &(0..255).map(|i| i as u8).collect::<Vec<_>>());
    }
}

#[test]
fn unknown_owner_small_bound_needs_only_the_fresh_eight_byte_prefix() {
    for v in [vec![], vec![0], vec![7], vec![7, 0, 6, 1, 5, 2, 4, 3]] {
        let mut allocation = owner(None);
        let s = state(&mut allocation);
        s.mtf_upper_bound.write(0);
        let mut actual = v.clone();
        // SAFETY: bound 0 source do-while initializes eight list bytes. Every
        // input index is <=7, so no unknown retained tail is reached.
        unsafe {
            inverse_move_to_front_transform(
                &mut actual,
                v.len() as Reg,
                &mut s.mtf_upper_bound,
                &mut s.mtf,
            )
        };
        let expected_bound = v.iter().fold(0, |bound, &index| bound | Reg::from(index)) >> 2;
        // SAFETY: each call always writes the upper bound and first eight
        // list bytes. Empty input need not initialize scratch byte 3.
        unsafe {
            assert_eq!(s.mtf_upper_bound.assume_init(), expected_bound);
            let prefix = std::slice::from_raw_parts(s.mtf.as_ptr().cast::<u8>().add(4), 8);
            let mut sorted = prefix.to_vec();
            sorted.sort_unstable();
            assert_eq!(sorted, [0, 1, 2, 3, 4, 5, 6, 7]);
        }
    }
}

#[test]
fn empty_call_preserves_word_zero_and_tail_but_resets_two_native_words() {
    for seed in [165, 90] {
        let mut allocation = owner(Some(seed));
        let s = state(&mut allocation);
        s.mtf_upper_bound.write(0);
        // SAFETY: initialized bound 0 and empty output; all bytes physically
        // fixture-seeded for complete byte observations after the call.
        unsafe { inverse_move_to_front_transform(&mut [], 0, &mut s.mtf_upper_bound, &mut s.mtf) };
        // SAFETY: allocator fixture initialized all 260 bytes, with no padding.
        let bytes = unsafe { std::slice::from_raw_parts(s.mtf.as_ptr().cast::<u8>(), 260) };
        assert_eq!(&bytes[..4], &[seed; 4]);
        assert_eq!(&bytes[4..12], &[0, 1, 2, 3, 4, 5, 6, 7]);
        assert!(bytes[12..].iter().all(|&byte| byte == seed));
    }
}

#[derive(Clone, Debug)]
struct Case {
    bound: i32,
    calls: Vec<(Vec<u8>, usize)>,
}
fn cases() -> Vec<Case> {
    let mut out = Vec::new();
    for index in 0..=255 {
        for length in [0, 1, 2, 17, 256] {
            out.push(Case {
                bound: -1,
                calls: vec![(vec![index; length + 11], length)],
            });
        }
    }
    for bound in -1..64 {
        // For overridden small bounds the allocator-seeded tail is defined
        // initialized arbitrary storage, not claimed to be a reachable list
        // history. Every byte of that retained storage is compared separately.
        out.push(Case {
            bound,
            calls: vec![
                (vec![0x71; 13], 0),
                ((0..256).map(|i| i as u8).collect(), 256),
                ((0..256).rev().map(|i| i as u8).collect(), 256),
                (vec![255, 0, 1, 254, 2, 3, 4, 5, 6, 7, 91], 10),
                (vec![1; 19], 0),
                (vec![0; 13], 13),
                (vec![255, 255, 0, 1, 2, 3, 4, 5, 0x71], 8),
            ],
        });
    }
    let mut rng = 0x123456789abcdef0u64;
    for length in [
        0, 1, 2, 3, 7, 8, 15, 16, 31, 32, 63, 64, 255, 256, 1024, 4096, 8192,
    ] {
        for prefix in [0, length / 2, length] {
            for mode in 0..4 {
                let data = (0..length)
                    .map(|i| {
                        rng ^= rng << 13;
                        rng ^= rng >> 7;
                        rng ^= rng << 17;
                        match mode {
                            0 => rng as u8,
                            1 => (rng as u8) & 7,
                            2 => i as u8,
                            _ => 255 - (i as u8),
                        }
                    })
                    .collect();
                out.push(Case {
                    bound: -1,
                    calls: vec![(data, prefix)],
                });
            }
        }
    }
    out
}
fn observe(c: &Case, seed: u8) -> Vec<u8> {
    let mut allocation = owner(Some(seed));
    let s = state(&mut allocation);
    s.mtf_upper_bound
        .write(if c.bound < 0 { 63 } else { c.bound as Reg });
    let mut out = Vec::new();
    for (data, length) in &c.calls {
        let mut v = data.clone();
        // SAFETY: source StateInit bound or explicitly initialized arbitrary
        // tail fixture; all reached list/payload bytes are initialized, fields
        // are disjoint, and the same actual owner is retained across calls.
        unsafe {
            inverse_move_to_front_transform(
                &mut v,
                *length as Reg,
                &mut s.mtf_upper_bound,
                &mut s.mtf,
            )
        };
        // SAFETY: bound written at every return; all 260 raw bytes physically
        // fixture-seeded (u32 array has no padding), no logical-init inference.
        unsafe {
            out.extend_from_slice(&s.mtf_upper_bound.assume_init().to_le_bytes());
            out.extend_from_slice(std::slice::from_raw_parts(s.mtf.as_ptr().cast::<u8>(), 260));
        }
        out.extend(v);
    }
    out
}

#[test]
fn retained_owner_history_and_all_input_indices_run_natively() {
    for c in cases() {
        for seed in [165, 90] {
            observe(&c, seed);
        }
    }
}

#[cfg(feature = "cpp-oracle")]
#[test]
fn actual_original_mtf_full_owner_and_payload_bytes_match() {
    use std::io::Write;
    use std::process::{Command, Stdio};
    let capture =
        std::path::PathBuf::from(std::env::var_os("BROTLI_BLOCK_MTF_CAPTURE").unwrap()).join("mtf");
    std::fs::create_dir_all(&capture).unwrap();
    let request = capture.join("requests.txt");
    let mut file = std::io::BufWriter::new(std::fs::File::create(&request).unwrap());
    let mut expected = Vec::new();
    let cases = cases();
    let mut records = 0;
    for c in &cases {
        writeln!(file, "{} {}", c.bound, c.calls.len()).unwrap();
        for (data, length) in &c.calls {
            write!(file, "{} {} ", data.len(), length).unwrap();
            for byte in data {
                write!(file, "{byte:02x}").unwrap();
            }
            writeln!(file).unwrap();
        }
        for seed in [165, 90] {
            expected.extend(observe(c, seed));
            records += c.calls.len();
        }
    }
    file.flush().unwrap();
    drop(file);
    let source = Command::new(std::env::var_os("BROTLI_BLOCK_MTF_ORACLE").unwrap())
        .arg("mtf")
        .stdin(Stdio::from(std::fs::File::open(&request).unwrap()))
        .output()
        .unwrap();
    assert!(
        source.status.success(),
        "{}",
        String::from_utf8_lossy(&source.stderr)
    );
    assert!(source.stderr.is_empty());
    std::fs::write(capture.join("source.bin"), &source.stdout).unwrap();
    std::fs::write(capture.join("rust.bin"), &expected).unwrap();
    assert_eq!(source.stdout.len(), expected.len());
    for (i, (&source, &rust)) in source.stdout.iter().zip(&expected).enumerate() {
        assert_eq!(source, rust, "byte {i}");
    }
    std::fs::write(
        capture.join("counts.txt"),
        format!("{} {}\n", cases.len(), records),
    )
    .unwrap();
}
