// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
use jxl::skcms::profile::{self, IccProfile, SIGNATURE_RGB};
use jxl::skcms::transform as t;
use jxl::skcms::transform_types::{AlphaFormat as A, PixelFormat as P};
use std::mem::{MaybeUninit, size_of};
use std::ptr;
#[cfg(all(feature = "cpp-oracle", target_arch = "aarch64", target_os = "macos"))]
#[path = "common/skcms_transform.rs"]
pub mod native_cases;
#[cfg(all(feature = "cpp-oracle", target_arch = "aarch64", target_os = "macos"))]
#[path = "common/skcms_profile_observation.rs"]
mod skcms_profile_observation;

#[cfg(all(feature = "cpp-oracle", target_arch = "aarch64", target_os = "macos"))]
#[test]
fn native_original_transform_captures_retain_every_observed_word() {
    let captures = include_str!("data/skcms-transform-native-arm64.tsv");
    let rows: Vec<_> = captures.lines().collect();
    assert_eq!(rows.len(), 16, "native original capture inventory");
    for row in rows {
        let (request, expected) = row.split_once('\t').unwrap();
        assert_eq!(native_cases::observe(request), expected, "{request}");
    }
}

#[test]
fn invalid_gamma_prefix_does_not_materialize_unknown_function_words() {
    use jxl::skcms::transfer_function::TransferFunction;
    let mut profile = MaybeUninit::<IccProfile>::uninit();
    let mut inverse = MaybeUninit::<TransferFunction>::uninit();
    // SAFETY: the inverse predicate reads has_trc and the first curve's count,
    // then returns false after classifying the g-only invalid inverse marker.
    // Neither function's a..f words nor the profile's other fields are reached.
    unsafe {
        let p = profile.as_mut_ptr();
        ptr::addr_of_mut!((*p).has_trc).write(true);
        ptr::addr_of_mut!((*p).trc[0]).cast::<u32>().write(0);
        ptr::addr_of_mut!((*p).trc[0].parametric.function.g).write(-129.0);
        ptr::addr_of_mut!((*inverse.as_mut_ptr()).g).write(-129.0);
        assert!(!t::trcs_are_approximate_inverse(p, inverse.as_ptr()));
    }
}

#[test]
fn zero_pixels_retain_signed_channel_count_no_access_paths() {
    for destination in [false, true] {
        for count in [u32::MAX, u32::MAX - 1, 0xffff_ff80, 0x8000_0001] {
            let mut value = MaybeUninit::<IccProfile>::uninit();
            // SAFETY: original signed channel conversion yields a negative
            // non-INT_MIN count, so no curve is read. Zero pixels execute no
            // CLUT context, leaving grid pointers/points genuinely unknown.
            unsafe {
                let p = value.as_mut_ptr();
                ptr::addr_of_mut!((*p).data_color_space).write(SIGNATURE_RGB);
                ptr::addr_of_mut!((*p).pcs).write(profile::SIGNATURE_XYZ);
                ptr::addr_of_mut!((*p).has_cicp).write(false);
                if destination {
                    ptr::addr_of_mut!((*p).has_b2a).write(true);
                    ptr::addr_of_mut!((*p).b2a.input_channels).write(0);
                    ptr::addr_of_mut!((*p).b2a.matrix_channels).write(0);
                    ptr::addr_of_mut!((*p).b2a.output_channels).write(count);
                } else {
                    ptr::addr_of_mut!((*p).has_a2b).write(true);
                    ptr::addr_of_mut!((*p).a2b.input_channels).write(count);
                    ptr::addr_of_mut!((*p).a2b.matrix_channels).write(0);
                    ptr::addr_of_mut!((*p).a2b.output_channels).write(0);
                }
                let xyz = profile::xyz_d50_profile();
                assert!(t::transform(
                    ptr::null(),
                    P::RGB_fff,
                    A::Opaque,
                    if destination { xyz } else { p },
                    ptr::null_mut(),
                    P::RGB_fff,
                    A::Opaque,
                    if destination { p } else { xyz },
                    0,
                ));
            }
        }
    }
}

#[test]
fn size_guard_does_not_access_unreached_profile_storage() {
    let p = ptr::without_provenance::<IccProfile>(8);
    // SAFETY: this request returns at the original size guard, before any
    // pixel/profile access. Non-null pointers represent no readable storage.
    let ok = unsafe {
        t::transform(
            ptr::null(),
            P::RGBA_ffff,
            A::Unpremul,
            p,
            ptr::null_mut(),
            P::RGBA_ffff,
            A::Unpremul,
            p,
            i32::MAX as usize,
        )
    };
    assert!(!ok);
}
#[test]
fn in_place_size_rejection_precedes_profile_fields() {
    let p = ptr::without_provenance::<IccProfile>(8);
    let mut pixel = [0u8; 16];
    // SAFETY: original same-address/different-width rejection returns before
    // dereferencing either non-null profile or reading/writing pixel bytes.
    let ok = unsafe {
        t::transform(
            pixel.as_ptr(),
            P::RGBA_ffff,
            A::Unpremul,
            p,
            pixel.as_mut_ptr(),
            P::RGBA_8888,
            A::Unpremul,
            p,
            0,
        )
    };
    assert!(!ok);
    assert_eq!(pixel, [0; 16]);
}
#[test]
fn b2a_usability_does_not_read_unknown_inactive_profile_fields() {
    let mut p = MaybeUninit::<IccProfile>::uninit();
    // SAFETY: the selected source branch reads only has_B2A, which is initialized.
    unsafe {
        ptr::addr_of_mut!((*p.as_mut_ptr()).has_b2a).write(true);
        assert!(t::make_usable_as_destination(p.as_mut_ptr()));
    }
}
#[test]
fn failed_usability_does_not_read_unknown_later_flags() {
    let mut p = MaybeUninit::<IccProfile>::uninit();
    // SAFETY: has_B2A=false and has_trc=false select the original early return;
    // every later flag, matrix and curve remains genuinely uninitialized.
    unsafe {
        ptr::addr_of_mut!((*p.as_mut_ptr()).has_b2a).write(false);
        ptr::addr_of_mut!((*p.as_mut_ptr()).has_trc).write(false);
        assert!(!t::make_usable_as_destination(p.as_mut_ptr()));
    }
}
#[test]
fn identical_profile_pointer_skips_unknown_color_transform_fields() {
    let mut p = MaybeUninit::<IccProfile>::uninit();
    let mut pixels: [f32; 16] = std::array::from_fn(|i| i as f32 / 7.0);
    let before = pixels.map(f32::to_bits);
    // SAFETY: identical profile pointers skip color transformation. Only the
    // color-space field is reached; float pixels are aligned and in-place safe.
    unsafe {
        ptr::addr_of_mut!((*p.as_mut_ptr()).data_color_space).write(SIGNATURE_RGB);
        assert!(t::transform(
            pixels.as_ptr().cast(),
            P::RGBA_ffff,
            A::Unpremul,
            p.as_ptr(),
            pixels.as_mut_ptr().cast(),
            P::RGBA_ffff,
            A::Unpremul,
            p.as_ptr(),
            4
        ));
    }
    assert_eq!(pixels.map(f32::to_bits), before);
}
#[test]
fn original_null_short_circuits_read_no_profile_or_function() {
    // SAFETY: null profile short-circuits the inverse predicate; equal null
    // pointers short-circuit profile equality before its original memcmp.
    unsafe {
        assert!(!t::trcs_are_approximate_inverse(
            ptr::null(),
            ptr::without_provenance(4)
        ));
        assert!(t::approximately_equal_profiles(ptr::null(), ptr::null()));
    }
}
#[test]
fn a_late_curve_rejection_keeps_every_original_profile_byte() {
    // SAFETY: immutable static profile is fully initialized, including inactive
    // bytes; this copy creates independent writable source-layout storage.
    let mut p = unsafe { profile::srgb_profile().read() };
    // SAFETY: the prefix is shared by both union branches. The count-one guard
    // rejects before reading either table pointer from that selected branch.
    unsafe {
        ptr::addr_of_mut!(p.trc[2]).cast::<u32>().write(1);
    }
    // SAFETY: this fixture's whole representation remains initialized. A failed
    // conversion must not commit either of the two earlier successful curves.
    unsafe {
        let before =
            std::slice::from_raw_parts((&raw const p).cast::<u8>(), size_of::<IccProfile>())
                .to_vec();
        assert!(!t::make_usable_as_destination(&mut p));
        let after =
            std::slice::from_raw_parts((&raw const p).cast::<u8>(), size_of::<IccProfile>());
        assert_eq!(after, before);
    }
}
