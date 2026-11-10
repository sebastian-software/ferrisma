//! WP4 differential tests of the inverse DCTs and their manager:
//! `jddctmgr.c`, `jidctint.c`, `jidctfst.c`, `jidctflt.c` and `jidctred.c`.
//!
//! The probe `tools/oracle/probes/wp4/idct.c` sets up one component with a
//! random quantization table, lets `jinit_inverse_dct` and `start_pass`
//! select the routine for a DCT method and scaled size, and hashes the
//! output of every transformed block: image-like, DC-only, single and dense
//! coefficient blocks for every method and size, and blocks of arbitrary
//! coefficients for the methods whose arithmetic is defined for them in C.
//! It also prints the memory pools, which shows the C size of the module and
//! the multiplier table. `support/wp4.rs` runs the same sequence on the
//! port; each case compares both outputs exactly.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp2.rs"]
mod wp2;
#[path = "support/wp4.rs"]
mod wp4;

use oracle::{Fields, Oracle};

/// Runs `args` with the probe and the port and compares the output.
fn compare(oracle: &Oracle, args: &[&str]) -> Fields {
    let expected = oracle
        .probe("wp4", "idct")
        .run(args, &[])
        .success()
        .fields();
    let actual = wp4::run_idct(args);
    if actual != expected {
        let first = actual
            .0
            .iter()
            .zip(&expected.0)
            .position(|(a, e)| a != e)
            .unwrap_or(actual.0.len().min(expected.0.len()));
        let (port, c) = (actual.0.get(first), expected.0.get(first));
        if let (Some((key, port)), Some((_, c))) = (port, c)
            && key == "out"
        {
            let block = port.split(',').zip(c.split(',')).position(|(a, e)| a != e);
            panic!("{args:?}: block {block:?} differs");
        }
        panic!("{args:?}: outputs differ at field {first}\n  port:   {port:?}\n  oracle: {c:?}");
    }
    expected
}

/// Every scaled size with the accurate integer method, which every size
/// other than 8 uses, including blocks of arbitrary coefficients and 16-bit
/// quantization tables.
#[test]
fn scaled_sizes() {
    let Some(oracle) = Oracle::locate("scaled_sizes") else {
        return;
    };
    for size in 1..=16 {
        let size = size.to_string();
        for seed in ["1", "2", "3"] {
            compare(
                &oracle,
                &[
                    "-method", "islow", "-size", &size, "-blocks", "400", "-seed", seed,
                ],
            );
        }
        let fields = compare(
            &oracle,
            &[
                "-method", "islow", "-size", &size, "-blocks", "400", "-seed", "4", "-qmax",
                "65535", "-extreme",
            ],
        );
        assert_eq!(fields.get("error"), Some("0"), "size {size}");
    }
}

/// The three methods of the full-size IDCT. The fast integer method does
/// its arithmetic in `int`, which arbitrary coefficients overflow in C, and
/// the float method converts to `int`, which is undefined out of range, so
/// those get image-like blocks only.
#[test]
fn methods() {
    let Some(oracle) = Oracle::locate("methods") else {
        return;
    };
    for method in ["islow", "ifast", "float"] {
        for seed in ["1", "2", "3", "4", "5"] {
            for qmax in ["1", "16", "255"] {
                let fields = compare(
                    &oracle,
                    &[
                        "-method", method, "-blocks", "1000", "-seed", seed, "-qmax", qmax,
                    ],
                );
                assert_eq!(fields.get("error"), Some("0"), "{method}");
            }
        }
    }
}

/// The errors of `start_pass`: an unknown method at full size and an
/// unsupported scaled size.
#[test]
fn start_pass_errors() {
    let Some(oracle) = Oracle::locate("start_pass_errors") else {
        return;
    };
    let fields = compare(&oracle, &["-method", "3", "-blocks", "1"]);
    assert_ne!(fields.get("error"), Some("0"));
    for size in ["0", "17", "-1"] {
        let fields = compare(&oracle, &["-size", size, "-blocks", "1"]);
        assert_ne!(fields.get("error"), Some("0"));
    }
    /* An unknown method is ignored by the scaled sizes. */
    let fields = compare(&oracle, &["-method", "3", "-size", "4", "-blocks", "50"]);
    assert_eq!(fields.get("error"), Some("0"));
}
