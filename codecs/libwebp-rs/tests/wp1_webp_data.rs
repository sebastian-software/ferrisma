//! WP1 differential test: `WebPDataInit`, `WebPDataCopy` and `WebPDataClear`
//! (`src/webp/mux_types.h`). The probe `tools/oracle/probes/wp1/webp_data.c` runs
//! the C helpers on the same inputs, and this file compares the observable state:
//! the size, the bytes, the return value of the copy and the size after the clear.
//!
//! Not compared: whether C's destination holds a NULL pointer for empty data, which
//! a `Cow` slice cannot express, and the failure of the copy for a size beyond the
//! allocation limit, which a Rust slice cannot have. The refusal decision itself is
//! compared by `tests/wp1_alloc.rs`, because `webp_data_copy` uses the same test.

#[path = "support/oracle.rs"]
mod oracle;

use std::borrow::Cow;

use oracle::{Fields, Oracle};
use webp::webp::mux_types::{WebPData, webp_data_clear, webp_data_copy};

fn hex(bytes: &[u8]) -> String {
    bytes.iter().map(|byte| format!("{byte:02x}")).collect()
}

/// xorshift64*, a small deterministic generator for the inputs.
fn next_byte(state: &mut u64) -> u8 {
    *state ^= *state >> 12;
    *state ^= *state << 25;
    *state ^= *state >> 27;
    (state.wrapping_mul(0x2545_f491_4f6c_dd1d) >> 56) as u8
}

#[test]
fn webp_data_helpers_match_the_c_helpers() {
    let Some(oracle) = Oracle::locate("webp_data_helpers_match_the_c_helpers") else {
        return;
    };
    let probe = oracle.probe("wp1", "webp_data");

    let mut stdin = String::new();
    let mut expected = Fields::new();

    // WebPDataInit: empty data.
    stdin.push_str("init\n");
    let initialized = WebPData::default();
    expected = expected
        .with("size", initialized.bytes.len())
        .hex("bytes", &initialized.bytes);

    // WebPDataCopy of borrowed data of several lengths, then WebPDataClear.
    let mut state = 0x9e37_79b9_7f4a_7c15_u64;
    for length in [1_usize, 2, 7, 64, 255, 256, 1000, 65_537] {
        let bytes: Vec<u8> = (0..length).map(|_| next_byte(&mut state)).collect();
        stdin.push_str(&format!("copy {length} {}\n", hex(&bytes)));
        let source = WebPData {
            bytes: Cow::Borrowed(&bytes),
        };
        let mut copy = webp_data_copy(&source).expect("a small copy is accepted");
        expected = expected
            .with("copied", 1)
            .with("size", copy.bytes.len())
            .hex("bytes", &copy.bytes);
        webp_data_clear(&mut copy);
        expected = expected.with("cleared_size", copy.bytes.len());
    }

    // Empty sources: a NULL pointer and a non-NULL pointer with size 0 both copy to
    // empty data, and the copy succeeds.
    for command in ["copy0", "copyempty"] {
        stdin.push_str(&format!("{command}\n"));
        let mut copy = webp_data_copy(&WebPData::default()).expect("empty data copies");
        expected = expected
            .with("copied", 1)
            .with("size", copy.bytes.len())
            .hex("bytes", &copy.bytes);
        webp_data_clear(&mut copy);
        expected = expected.with("cleared_size", copy.bytes.len());
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

/// Clearing owned data must not touch borrowed input, and clearing borrowed data
/// must not free it. This needs no oracle.
#[test]
fn clearing_leaves_borrowed_input_alone() {
    let input = [1_u8, 2, 3];
    let mut view = WebPData {
        bytes: Cow::Borrowed(&input),
    };
    webp_data_clear(&mut view);
    assert!(view.bytes.is_empty());
    assert_eq!(input, [1, 2, 3]);
}
