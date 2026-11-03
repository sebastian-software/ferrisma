// Copyright 2018 Google Inc.
// Use of this source code is governed by the BSD-style license in
// licenses/SKCMS-BSD-3-Clause.txt.
use jxl::skcms::{curve::Curve, fitting, transfer_function::TransferFunction as Tf};
use std::mem::MaybeUninit;
use std::ptr;

fn words(tf: Tf) -> [u32; 7] {
    [tf.g, tf.a, tf.b, tf.c, tf.d, tf.e, tf.f].map(f32::to_bits)
}

#[test]
fn rejected_counts_leave_both_outputs_untouched_without_reading_the_union_tail() {
    for count in [0, 1, 0x8000_0000, u32::MAX] {
        let mut curve = MaybeUninit::<Curve>::uninit();
        let mut tf = Tf {
            g: 1.5,
            a: 0.75,
            b: 0.125,
            c: 2.0,
            d: 0.25,
            e: -0.125,
            f: 0.0625,
        };
        let expected = words(tf);
        let mut error = -123.5_f32;
        // SAFETY: only the common count prefix is initialized. The original
        // guard returns before reaching any other union member or output write.
        unsafe {
            curve.as_mut_ptr().cast::<u32>().write(count);
            assert!(!fitting::approximate_curve(
                curve.as_ptr(),
                &raw mut tf,
                &raw mut error
            ));
        }
        assert_eq!(words(tf), expected);
        assert_eq!(error.to_bits(), (-123.5_f32).to_bits());
    }
    // SAFETY: null arguments reach the original guard before any dereference.
    unsafe {
        assert!(!fitting::approximate_curve(
            ptr::null(),
            ptr::null_mut(),
            ptr::null_mut()
        ));
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn original_gradient_does_not_read_the_unused_e_field() {
    let table = [0, 255];
    let mut curve = Curve::zeroed();
    let mut tf = MaybeUninit::<Tf>::uninit();
    let mut gradient = [0.0; 3];
    // SAFETY: exactly the six original loaded TF members are initialized.
    // The unused e field stays genuinely unknown and is never observed.
    unsafe {
        ptr::addr_of_mut!(curve.table.table_entries).write(2);
        ptr::addr_of_mut!(curve.table.table_8).write(table.as_ptr());
        ptr::addr_of_mut!(curve.table.table_16).write(ptr::null());
        let p = tf.as_mut_ptr();
        ptr::addr_of_mut!((*p).g).write(f32::from_bits(1_074_580_685));
        ptr::addr_of_mut!((*p).a).write(1.0);
        ptr::addr_of_mut!((*p).b).write(0.125);
        ptr::addr_of_mut!((*p).c).write(1.0);
        ptr::addr_of_mut!((*p).d).write(0.0625);
        ptr::addr_of_mut!((*p).f).write(0.0);
        let residual = fitting::rg_nonlinear(0.0, &raw const curve, p, gradient.as_mut_ptr());
        // Original ordinary and ASan/UBSan private-helper request 274 in the
        // retained preflight inventory; its result is independent of e.
        assert_eq!(residual.to_bits(), 3_175_297_680);
        assert_eq!(
            gradient.map(f32::to_bits),
            [1_017_729_820, 3_164_019_213, 3_186_153_024]
        );
    }
}

#[cfg(target_arch = "aarch64")]
#[test]
fn original_degenerate_segment_keeps_its_contracted_subtraction_and_output_alias() {
    let table = include_bytes!("data/skcms-fitting-degenerate.table8");
    let mut curve = Curve::zeroed();
    // SAFETY: initialized union fields retain this immutable 255-byte table
    // through both calls. Padding and the inactive branch are never inspected.
    unsafe {
        ptr::addr_of_mut!(curve.table.table_entries).write(table.len() as u32);
        ptr::addr_of_mut!(curve.table.table_8).write(table.as_ptr());
        ptr::addr_of_mut!(curve.table.table_16).write(ptr::null());
    }
    let mut tf = Tf::default();
    let mut error = -123.5;
    // SAFETY: all source samples are in the live table; outputs are writable.
    assert!(unsafe { fitting::approximate_curve(&raw const curve, &raw mut tf, &raw mut error) });
    // Actual unchanged ordinary and ASan/UBSan source words, retained with
    // input and archive hashes in data/skcms-fitting-requests.json.
    assert_eq!(error.to_bits(), 989_789_184);
    assert_eq!(
        words(tf),
        [
            1_065_353_216,
            1_065_287_168,
            998_309_896,
            1_065_353_347,
            1_065_287_164,
            0,
            0
        ]
    );
    // SAFETY: max_error aliases exactly one actual float field. The source
    // writes error before its full TF assignment and rereads the live slot.
    let alias = ptr::addr_of_mut!(tf.g);
    assert!(unsafe { fitting::approximate_curve(&raw const curve, &raw mut tf, alias) });
    assert_eq!(
        words(tf),
        [
            1_065_353_216,
            1_065_287_168,
            998_309_896,
            1_065_353_347,
            1_065_287_164,
            0,
            0
        ]
    );
}
