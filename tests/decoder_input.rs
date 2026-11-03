// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
#[path = "support/decoder_input.rs"]
mod driver;
fn capture(index: usize) {
    let row = include_str!("../tools/captures/decoder_input.tsv")
        .lines()
        .nth(index)
        .unwrap();
    let (request, expected) = row.split_once('\t').unwrap();
    let mut args = request.split_whitespace();
    let kind = args.next().unwrap();
    let actual = if kind == "di-short" {
        driver::observe_short(&mut args)
    } else {
        driver::observe(&mut args)
    };
    assert_eq!(actual, expected, "{request}");
}
#[test]
fn unknown_closed_field_is_not_read_after_nonnull_input() {
    capture(0);
}
#[test]
fn closed_input_does_not_read_unknown_available_count() {
    capture(1);
}
#[test]
fn release_does_not_read_unknown_pointer_or_flag() {
    capture(2);
}
#[test]
fn close_initializes_only_its_unknown_flag() {
    capture(3);
}
#[test]
fn nonnull_empty_input_keeps_its_identity() {
    capture(4);
}
#[test]
fn allocation_callback_mutates_input_before_original_copy() {
    capture(5);
}
#[test]
fn allocation_failure_retains_input_and_partial_state() {
    capture(6);
}
#[test]
fn retained_copy_spans_replacement_and_release() {
    capture(7);
}
#[test]
fn zero_length_input_preserves_copy_lifecycle() {
    capture(8);
}
#[test]
fn partial_consumption_survives_fragment_replacement() {
    capture(9);
}
#[test]
fn repeated_get_appends_only_new_available_bytes() {
    capture(10);
}
#[test]
fn retained_buffer_allocation_callback_ownership_matches_original() {
    capture(11);
}
#[test]
fn out_of_order_injection_and_last_flag_match_original() {
    capture(12);
}
#[test]
fn empty_bounded_box_retains_its_original_input_checkpoints() {
    capture(13);
}
#[test]
fn bounded_box_skip_wrap_retains_input_checkpoints() {
    capture(14);
}

#[test]
fn skip_debt_crosses_multiple_input_fragments() {
    capture(15);
}
#[test]
fn invalid_copy_position_leaves_prior_span_unchanged() {
    capture(16);
}
#[test]
fn empty_success_span_retains_owned_end_pointer() {
    capture(17);
}
#[test]
fn copy_growth_retains_source_allocation_and_free_order() {
    capture(18);
}
#[test]
fn first_copy_allocation_failure_retains_external_input() {
    capture(19);
}
#[test]
fn second_copy_growth_failure_preserves_first_allocation() {
    capture(20);
}
#[test]
fn later_copy_growth_preserves_initialized_prefix() {
    capture(21);
}

#[test]
fn internal_error_preserves_a_prior_nonempty_output_span() {
    capture(22);
}
#[test]
fn injection_retains_original_u32_counter_wrap() {
    capture(23);
}
