//! WP10: the input generator and the hashes of the kernel tests
//! (`tests/support/kernels.rs`) against the probe support they mirror
//! (`tools/oracle/probes/wp10/support/wp10_support.c`, through the probe
//! `tools/oracle/probes/wp10/rng.c`). Every kernel test of work package 10
//! relies on both sides drawing the same inputs and hashing the same way.

#[path = "support/kernels.rs"]
mod kernels;
#[path = "support/oracle.rs"]
mod oracle;

use kernels::{Rng, array_rows, fill_rows, row_hashes, rows, stride};
use oracle::Oracle;

#[test]
fn generator_and_hashes_match_the_probe_support() {
    let Some(oracle) = Oracle::locate("generator_and_hashes_match_the_probe_support") else {
        return;
    };
    for (seed, count, numrows, width) in [(0, 5, 1, 1), (1, 40, 3, 64), (12345, 200, 5, 227)] {
        let args = [
            "-seed".to_owned(),
            seed.to_string(),
            "-count".to_owned(),
            count.to_string(),
            "-rows".to_owned(),
            numrows.to_string(),
            "-width".to_owned(),
            width.to_string(),
        ];
        let fields = oracle
            .probe("wp10", "rng")
            .run(&args, &[])
            .success()
            .fields();

        let mut rng = Rng::new(seed);
        let next: Vec<String> = (0..count).map(|_| rng.next().to_string()).collect();
        assert_eq!(
            fields.get("next"),
            Some(next.join(",").as_str()),
            "seed {seed}"
        );

        let mut array = rows(numrows, width);
        fill_rows(&mut rng, &mut array);
        assert_eq!(fields.int("stride"), i64::try_from(stride(width)).unwrap());
        let expected = row_hashes(&array_rows(&array, stride(width)), stride(width));
        assert_eq!(fields.get("rows"), Some(expected.as_str()), "seed {seed}");
    }
}
