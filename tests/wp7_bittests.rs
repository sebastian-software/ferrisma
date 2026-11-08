//! WP7: upstream's in-scope `djpeg` bit tests on the port (ADR 0002).
//!
//! Upstream's CMake suite decodes test images with `djpeg` and compares each
//! output with an MD5 sum (`add_bittest`). `tools/bittests.py --run` runs the
//! in-scope 8-bit cases against the oracle, which also produces their inputs
//! with the oracle's `cjpeg` and checks those inputs' MD5 sums, and writes
//! `bittests-port.txt` next to the oracle's `oracle.json`: each case's input,
//! switches, output file and MD5 checks. `support/djpeg.rs` runs `djpeg.c`'s
//! library calls and `wrppm.c`'s output on the port, and each output must
//! have upstream's MD5 sum. A second test compares the driver's output with
//! the oracle's `djpeg` byte for byte on more inputs and switches, which
//! checks the translated driver itself.

#[path = "support/djpeg.rs"]
mod djpeg;
#[path = "support/inputs.rs"]
mod inputs;
#[path = "support/md5.rs"]
mod md5;
#[path = "support/oracle.rs"]
mod oracle;

use std::process::Command;

use oracle::Oracle;

/// A case of `bittests-port.txt`.
#[derive(Default)]
struct Case {
    test: String,
    input: String,
    args: Vec<String>,
    outfile: String,
    checks: Vec<(String, String)>,
}

fn manifest(oracle: &Oracle) -> Vec<Case> {
    let path = oracle.directory().join("bittests-port.txt");
    let text = std::fs::read_to_string(&path).unwrap_or_else(|_| {
        panic!(
            "missing {}; run python3 tools/bittests.py --run",
            path.display()
        )
    });
    let mut cases = Vec::new();
    let mut case = Case::default();
    for line in text.lines().filter(|line| !line.starts_with('#')) {
        let (key, value) = line.split_once(' ').unwrap_or((line, ""));
        match key {
            "case" => case.test = value.to_owned(),
            "input" => case.input = value.to_owned(),
            "arg" => case.args.push(value.to_owned()),
            "outfile" => case.outfile = value.to_owned(),
            "check" => {
                let (file, md5) = value.split_once(' ').expect("check FILE MD5");
                case.checks.push((file.to_owned(), md5.to_owned()));
            }
            "end" => cases.push(std::mem::take(&mut case)),
            other => panic!("unknown manifest line {other}"),
        }
    }
    cases
}

#[test]
fn upstream_bit_tests_pass_on_the_port() {
    let Some(oracle) = Oracle::locate("upstream_bit_tests_pass_on_the_port") else {
        return;
    };
    let cases = manifest(&oracle);
    assert_eq!(cases.len(), 30, "the in-scope cases of tools/bittests.json");
    for case in &cases {
        let data = std::fs::read(&case.input).unwrap();
        let args: Vec<&str> = case.args.iter().map(String::as_str).collect();
        let output =
            djpeg::djpeg(&args, &data).unwrap_or_else(|message| panic!("{}: {message}", case.test));
        /* djpeg exits with EXIT_WARNING after a warning, which fails the run test */
        assert_eq!(output.num_warnings, 0, "{}: warnings", case.test);
        assert!(!case.checks.is_empty(), "{}: no MD5 check", case.test);
        for (file, expected) in &case.checks {
            let bytes = if *file == case.outfile {
                &output.image
            } else {
                match &output.icc {
                    Some((name, profile)) if name == file => profile,
                    _ => panic!("{}: no output {file}", case.test),
                }
            };
            assert_eq!(&md5::md5_hex(bytes), expected, "{}: {file}", case.test);
        }
    }
}

/// Runs the oracle's `djpeg` in a directory of its own and returns its
/// output file and the ICC profile file `x.icc`, if it wrote one.
fn oracle_djpeg(
    oracle: &Oracle,
    args: &[&str],
    data: &[u8],
) -> Result<(Vec<u8>, Option<Vec<u8>>), String> {
    static CALLS: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
    let call = CALLS.fetch_add(1, std::sync::atomic::Ordering::Relaxed);
    let directory =
        std::env::temp_dir().join(format!("libjpeg-rs-wp7-{}-{call}", std::process::id()));
    std::fs::create_dir_all(&directory).unwrap();
    std::fs::write(directory.join("input.jpg"), data).unwrap();
    let result = Command::new(oracle.tool("djpeg"))
        .args(args)
        .args(["-outfile", "output.ppm", "input.jpg"])
        .current_dir(&directory)
        .output()
        .unwrap();
    let image = std::fs::read(directory.join("output.ppm"));
    let icc = std::fs::read(directory.join("x.icc")).ok();
    let _ = std::fs::remove_dir_all(&directory);
    if result.status.success() {
        Ok((image.unwrap(), icc))
    } else {
        Err(String::from_utf8_lossy(&result.stderr).into_owned())
    }
}

#[test]
fn driver_matches_the_oracle_djpeg() {
    let Some(oracle) = Oracle::locate("driver_matches_the_oracle_djpeg") else {
        return;
    };
    let mut inputs: Vec<(String, Vec<u8>)> = ["testorig.jpg", "testimgari.jpg", "testimgint.jpg"]
        .iter()
        .map(|name| ((*name).to_owned(), inputs::image(name)))
        .collect();
    inputs.push((
        "cmyk".to_owned(),
        inputs::four_component(&oracle, &["cmyk"]),
    ));
    inputs.push((
        "ycck".to_owned(),
        inputs::four_component(&oracle, &["ycck"]),
    ));
    inputs.push((
        "progressive".to_owned(),
        inputs::encode(&oracle, &["-progressive"]),
    ));
    inputs.push((
        "grayscale".to_owned(),
        inputs::encode(&oracle, &["-grayscale"]),
    ));
    let (mut compared, mut failed) = (0, 0);
    for (label, data) in &inputs {
        for args in [
            &[][..],
            &["-ppm"],
            &["-pnm", "-grayscale"],
            &["-rgb", "-nosmooth"],
            &["-fast", "-scale", "3/8"],
            &["-dct", "float", "-scale", "15/8"],
            &["-skip", "3,40"],
            &["-skip", "0,0", "-nosmooth"],
            &["-crop", "50x30+17+9"],
            &["-crop", "1x1+0+0", "-scale", "1/2"],
            &["-crop", "227x149+0+0"],
            &["-crop", "500x10+0+0"],
            &["-skip", "10,400"],
            &["-icc", "x.icc"],
        ] {
            let expected = oracle_djpeg(&oracle, args, data);
            let actual = djpeg::djpeg(args, data);
            match (&expected, &actual) {
                (Ok((image, icc)), Ok(output)) => {
                    assert!(*image == output.image, "{label} {args:?}: outputs differ");
                    let port_icc = output.icc.as_ref().map(|(_, profile)| profile);
                    assert_eq!(
                        icc.as_ref(),
                        port_icc,
                        "{label} {args:?}: ICC profiles differ"
                    );
                    compared += 1;
                }
                (Err(_), Err(_)) => failed += 1,
                _ => panic!(
                    "{label} {args:?}: the oracle {} and the port {}",
                    if expected.is_ok() { "decodes" } else { "fails" },
                    if actual.is_ok() { "decodes" } else { "fails" }
                ),
            }
        }
    }
    /* The failures are crops and skips beyond the image and conversions
     * that four-component input does not support, in both. */
    assert_eq!(
        (compared, failed),
        (80, 18),
        "byte comparisons and shared failures"
    );
}
