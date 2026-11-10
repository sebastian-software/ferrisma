//! WP2 differential test: the target-dependent configuration of the bit I/O
//! units. The reader's `BITS`, the register and load types, and the writer's
//! accumulator and write word depend on the target architecture (ADR 0002). The
//! probe `tools/oracle/probes/wp2/config.c` reports the values that the C build
//! on this host uses, and the test compares them with the Rust constants. The
//! fast-load path of `VP8LDoFillBitWindow` is observed, not read from the source.
//!
//! The consistency checks at the end need no oracle.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::utils::bit_reader_inl_utils::{LBIT_BYTES, lbit_t};
use webp::utils::bit_reader_utils::{
    BITS, VP8L_LBITS, VP8L_MAX_NUM_BIT_READ, VP8L_USE_FAST_LOAD, VP8L_WBITS, bit_t,
};
use webp::utils::bit_writer_utils::{
    VP8L_WRITER_BITS, VP8L_WRITER_BYTES, VP8L_WRITER_MAX_BITS, vp8l_atype_t, vp8l_wtype_t,
};

#[test]
fn target_configuration_matches_the_c_build() {
    let Some(oracle) = Oracle::locate("target_configuration_matches_the_c_build") else {
        return;
    };
    let args: [&str; 0] = [];
    let output = oracle.probe("wp2", "config").run(&args, &[]);
    let fields = output.success().fields();
    let expected = Fields::new()
        .with("bits", BITS)
        .with("bit_t_bytes", size_of::<bit_t>())
        .with("lbit_t_bytes", size_of::<lbit_t>())
        .with("vp8l_atype_bytes", size_of::<vp8l_atype_t>())
        .with("vp8l_wtype_bytes", size_of::<vp8l_wtype_t>())
        .with("vp8l_writer_bytes", VP8L_WRITER_BYTES)
        .with("vp8l_writer_bits", VP8L_WRITER_BITS)
        .with("vp8l_writer_max_bits", VP8L_WRITER_MAX_BITS)
        .with("fast_load", u8::from(VP8L_USE_FAST_LOAD));
    assert_eq!(
        fields, expected,
        "the Rust configuration differs from the C build on this host"
    );
}

// upstream selects only 24 and 56 bits, as the header's target list states.
const _: () = assert!(BITS == 24 || BITS == 56);

#[test]
fn widths_follow_the_target_choice() {
    // upstream: BITS > 24 selects the 64-bit register and load types.
    assert_eq!(BITS > 24, size_of::<bit_t>() == 8);
    assert_eq!(BITS > 32, size_of::<lbit_t>() == 8);
    assert_eq!(LBIT_BYTES, size_of::<lbit_t>());
    // The 64-bit accumulator is the x86-64 choice of the lossless writer.
    assert_eq!(VP8L_WRITER_MAX_BITS, 2 * VP8L_WRITER_BITS);
    assert_eq!(VP8L_WRITER_BITS as usize, 8 * VP8L_WRITER_BYTES);
    assert_eq!(size_of::<vp8l_atype_t>(), 2 * size_of::<vp8l_wtype_t>());
}

#[test]
fn lossless_reader_constants_match_the_header() {
    assert_eq!(VP8L_LBITS, 64);
    assert_eq!(VP8L_WBITS, 32);
    assert_eq!(VP8L_MAX_NUM_BIT_READ, 24);
    assert_eq!(size_of::<webp::utils::bit_reader_utils::vp8l_val_t>(), 8);
}
