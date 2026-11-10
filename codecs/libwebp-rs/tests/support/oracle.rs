//! Runs the C oracle probes for differential tests (WP0, ADR 0002).
//!
//! `tools/build_oracle.py` builds the pinned libwebp in the reference
//! configuration and compiles every `tools/oracle/probes/<wp>/<name>.c` into
//! `build/oracle/<variant>/probes/<wp>/<name>`. `LIBWEBP_ORACLE_DIR` names that
//! variant directory, for example `build/oracle/release`; a relative path is
//! resolved against the repository root.
//!
//! A test that needs the oracle starts with [`Oracle::locate`]. When the
//! variable is unset the test prints why it is skipped and returns, unless the
//! `oracle` Cargo feature is enabled (as `tools/check.py --oracle` does), in
//! which case it fails. Probes run in separate processes; they print one
//! `key=value` line per observation, integers in decimal and byte strings in
//! lowercase hex, and [`Output::fields`] parses those lines for comparison
//! with the Rust translation's results.
//!
//! Include this file from a test crate with
//! `#[path = "support/oracle.rs"] mod oracle;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use std::ffi::OsStr;
use std::fmt::Display;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, ExitStatus, Stdio};

/// The environment variable that names the oracle variant directory.
pub const ORACLE_DIR: &str = "LIBWEBP_ORACLE_DIR";

/// The repository root.
pub fn repository() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The pinned `libwebp-test-data` checkout.
pub fn test_data() -> PathBuf {
    repository().join("upstream/libwebp-test-data")
}

/// Every `.webp` file of the pinned test data, sorted by file name.
pub fn test_vectors() -> Vec<PathBuf> {
    let directory = test_data();
    let entries = std::fs::read_dir(&directory).unwrap_or_else(|error| {
        panic!(
            "cannot read {}: {error}; run git submodule update --init",
            directory.display()
        )
    });
    let mut vectors: Vec<PathBuf> = entries
        .map(|entry| entry.expect("test data directory entry").path())
        .filter(|path| path.extension() == Some(OsStr::new("webp")))
        .collect();
    vectors.sort();
    vectors
}

/// A built oracle variant.
pub struct Oracle {
    directory: PathBuf,
}

impl Oracle {
    /// Returns the oracle named by `LIBWEBP_ORACLE_DIR`.
    ///
    /// Without the variable, returns `None` after printing that `test` is
    /// skipped, or panics when the `oracle` feature requires the oracle.
    /// Panics when the variable names a directory that holds no oracle build.
    pub fn locate(test: &str) -> Option<Oracle> {
        let Some(value) = std::env::var_os(ORACLE_DIR) else {
            let reason = format!(
                "{ORACLE_DIR} is unset; build the oracle with `python3 tools/build_oracle.py` \
                 and set {ORACLE_DIR}=build/oracle/release"
            );
            if cfg!(feature = "oracle") {
                panic!("{test}: the oracle feature requires the oracle: {reason}");
            }
            eprintln!("skipped {test}: {reason}");
            return None;
        };
        let directory = repository().join(value);
        assert!(
            directory.join("oracle.json").is_file(),
            "{ORACLE_DIR}={} holds no oracle build; run python3 tools/build_oracle.py",
            directory.display()
        );
        Some(Oracle { directory })
    }

    /// The variant directory.
    pub fn directory(&self) -> &Path {
        &self.directory
    }

    /// The probe built from `tools/oracle/probes/<package>/<name>.c`.
    ///
    /// Panics when the probe is missing, which means the oracle predates the
    /// probe and must be rebuilt.
    pub fn probe(&self, package: &str, name: &str) -> Probe {
        let path = self.directory.join("probes").join(package).join(name);
        assert!(
            path.is_file(),
            "missing probe {}; rebuild the oracle with python3 tools/build_oracle.py",
            path.display()
        );
        Probe { path }
    }

    /// An upstream tool of the oracle build: `dwebp`, `cwebp` or `webpinfo`.
    pub fn tool(&self, name: &str) -> PathBuf {
        let path = self.directory.join("libwebp").join(name);
        assert!(path.is_file(), "missing oracle tool {}", path.display());
        path
    }
}

/// One probe executable.
pub struct Probe {
    path: PathBuf,
}

impl Probe {
    /// Runs the probe with `args` and `stdin` and collects its output.
    pub fn run<A: AsRef<OsStr>>(&self, args: &[A], stdin: &[u8]) -> Output {
        let mut child = Command::new(&self.path)
            .args(args)
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap_or_else(|error| panic!("cannot run {}: {error}", self.path.display()));
        let mut input = child.stdin.take().expect("piped stdin");
        // Feed stdin from a second thread so that a probe that writes before it
        // has read everything cannot block on a full pipe.
        let output = std::thread::scope(|scope| {
            scope.spawn(move || {
                // A probe may exit without reading its input; that is not an error here.
                let _ = input.write_all(stdin);
            });
            child.wait_with_output()
        })
        .unwrap_or_else(|error| panic!("cannot wait for {}: {error}", self.path.display()));
        Output {
            probe: self.path.clone(),
            stdout: output.stdout,
            stderr: output.stderr,
            status: output.status,
        }
    }
}

/// The observable result of one probe run.
#[derive(Debug)]
pub struct Output {
    pub probe: PathBuf,
    pub stdout: Vec<u8>,
    pub stderr: Vec<u8>,
    pub status: ExitStatus,
}

impl Output {
    /// Asserts that the probe exited with status 0 and returns `self`.
    pub fn success(&self) -> &Self {
        assert!(
            self.status.success(),
            "{} failed with {}: {}",
            self.probe.display(),
            self.status,
            String::from_utf8_lossy(&self.stderr)
        );
        self
    }

    /// Parses standard output as `key=value` lines, in order.
    pub fn fields(&self) -> Fields {
        let text = std::str::from_utf8(&self.stdout)
            .unwrap_or_else(|_| panic!("{} printed non-UTF-8 output", self.probe.display()));
        let mut fields = Fields::new();
        for line in text.lines() {
            let (key, value) = line.split_once('=').unwrap_or_else(|| {
                panic!(
                    "{} printed a line without '=': {line:?}",
                    self.probe.display()
                )
            });
            fields.0.push((key.to_owned(), value.to_owned()));
        }
        fields
    }
}

/// Ordered `key=value` observations, as printed by a probe or built from the
/// Rust translation's results for comparison.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Fields(pub Vec<(String, String)>);

impl Fields {
    pub fn new() -> Self {
        Fields(Vec::new())
    }

    /// Appends a field whose value is printed with `Display`, as probes print integers.
    #[must_use]
    pub fn with(mut self, key: &str, value: impl Display) -> Self {
        self.0.push((key.to_owned(), value.to_string()));
        self
    }

    /// Appends a byte string in lowercase hex, as `ProbePrintHex` prints it.
    #[must_use]
    pub fn hex(mut self, key: &str, bytes: &[u8]) -> Self {
        let value = bytes.iter().map(|byte| format!("{byte:02x}")).collect();
        self.0.push((key.to_owned(), value));
        self
    }

    /// The value of the first field named `key`.
    pub fn get(&self, key: &str) -> Option<&str> {
        self.0
            .iter()
            .find(|(name, _)| name == key)
            .map(|(_, value)| value.as_str())
    }

    /// The value of the first field named `key`, parsed as an integer.
    pub fn int(&self, key: &str) -> i64 {
        let value = self
            .get(key)
            .unwrap_or_else(|| panic!("missing field {key} in {self:?}"));
        value
            .parse()
            .unwrap_or_else(|_| panic!("field {key}={value} is not an integer"))
    }

    /// The field names, in order.
    pub fn keys(&self) -> Vec<&str> {
        self.0.iter().map(|(name, _)| name.as_str()).collect()
    }
}
