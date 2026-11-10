//! Shared helpers of the WP18 parity tests (ADR 0008, parity 1): run a
//! dispatch function at each SIMD level the host has and compare its output
//! with the C level's, on copies of the same seeded input.
//!
//! A level is selected the way the C ABI's `-noasm` switch selects the C code:
//! through the `VP8GetCPUInfo` pointer that `webp::dsp::cpu::features()`
//! follows. The C level installs `NULL`; the SSE2, SSE4.1 and AVX2 levels
//! install a query that answers the running CPU's flags up to that level
//! (`kSSE2`; `kSSE2` to `kSSE4_1`; `kSSE2` to `kAVX2`), so that the dispatch
//! functions select exactly the variants that upstream's `Init` functions
//! assign at that level. The pointer is global, so a lock serializes the
//! tests of one test binary while a level is installed.
//!
//! Include it with `#[path = "wp18_support/mod.rs"] mod wp18_support;`.

#![allow(
    dead_code,
    reason = "each test crate uses a different subset of the helpers"
)]

use core::ffi::{c_int, c_void};
use std::fmt::Debug;
use std::sync::atomic::Ordering;
use std::sync::{Mutex, MutexGuard, PoisonError};

use webp::abi::decode::VP8_GET_CPU_INFO;
use webp::dsp::cpu::{CPUFeature, vp8_get_cpu_info};

/// A selection level of the dispatch functions.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Level {
    /// `VP8GetCPUInfo = NULL`: the C translation.
    C,
    /// The SSE2 variants.
    Sse2,
    /// The SSE2 variants with the SSE4.1 overrides.
    Sse41,
    /// Every x86 variant, AVX2 included.
    Avx2,
}

impl Level {
    /// The highest feature the level's query answers.
    fn highest(self) -> Option<CPUFeature> {
        match self {
            Self::C => None,
            Self::Sse2 => Some(CPUFeature::kSSE2),
            Self::Sse41 => Some(CPUFeature::kSSE4_1),
            Self::Avx2 => Some(CPUFeature::kAVX2),
        }
    }

    /// The level's `VP8GetCPUInfo` pointer.
    fn pointer(self) -> *mut c_void {
        match self {
            Self::C => core::ptr::null_mut(),
            Self::Sse2 => query_sse2 as *mut c_void,
            Self::Sse41 => query_sse41 as *mut c_void,
            Self::Avx2 => query_avx2 as *mut c_void,
        }
    }
}

/// Answers `feature` as the running CPU does, up to `highest`.
fn answer(feature: CPUFeature, highest: CPUFeature) -> c_int {
    c_int::from(feature <= highest && vp8_get_cpu_info(feature))
}

extern "C" fn query_sse2(feature: CPUFeature) -> c_int {
    answer(feature, CPUFeature::kSSE2)
}

extern "C" fn query_sse41(feature: CPUFeature) -> c_int {
    answer(feature, CPUFeature::kSSE4_1)
}

extern "C" fn query_avx2(feature: CPUFeature) -> c_int {
    answer(feature, CPUFeature::kAVX2)
}

/// The SIMD levels the running CPU has, lowest first. AVX2 is reported by
/// the translation's own query (`cpuid` and the OS's register state).
pub fn simd_levels() -> Vec<Level> {
    [Level::Sse2, Level::Sse41, Level::Avx2]
        .into_iter()
        .filter(|level| vp8_get_cpu_info(level.highest().expect("a SIMD level")))
        .collect()
}

/// Serializes the installation of a level within one test binary.
static LOCK: Mutex<()> = Mutex::new(());

/// Holds a level installed until it is dropped, then restores the pointer.
pub struct Installed {
    previous: *mut c_void,
    _guard: MutexGuard<'static, ()>,
}

impl Drop for Installed {
    fn drop(&mut self) {
        VP8_GET_CPU_INFO.store(self.previous, Ordering::Release);
    }
}

/// Installs `level` until the result is dropped.
pub fn install(level: Level) -> Installed {
    let guard = LOCK.lock().unwrap_or_else(PoisonError::into_inner);
    let previous = VP8_GET_CPU_INFO.swap(level.pointer(), Ordering::AcqRel);
    Installed {
        previous,
        _guard: guard,
    }
}

/// Runs `run` at `level`.
pub fn at_level<R>(level: Level, run: impl FnOnce() -> R) -> R {
    let _installed = install(level);
    run()
}

/// Counts the comparisons of one test for its summary line.
#[derive(Default)]
pub struct Tally {
    rows: Vec<(String, usize, Vec<Level>)>,
}

impl Tally {
    /// Records `inputs` inputs of `function`, each compared at `levels`.
    pub fn add(&mut self, function: &str, inputs: usize, levels: &[Level]) {
        self.rows
            .push((function.to_owned(), inputs, levels.to_vec()));
    }

    /// Prints one line per function: inputs and the levels compared with C.
    pub fn print(&self, test: &str) {
        let mut total = 0;
        for (function, inputs, levels) in &self.rows {
            eprintln!("{test}: {function}: {inputs} inputs, compared at {levels:?} with C");
            total += inputs * levels.len();
        }
        eprintln!("{test}: {total} comparisons");
    }
}

/// Runs `run` on a copy of `input` at the C level and at each of `levels`,
/// and asserts that every level leaves the copy as the C level does.
/// `describe` names the case in a failure message.
pub fn assert_parity<T: Clone + PartialEq + Debug>(
    levels: &[Level],
    input: &T,
    describe: impl Fn() -> String,
    run: impl Fn(&mut T),
) {
    let mut expected = input.clone();
    at_level(Level::C, || run(&mut expected));
    for &level in levels {
        let mut actual = input.clone();
        at_level(level, || run(&mut actual));
        if actual != expected {
            panic!(
                "{}: {level:?} differs from C\n{}",
                describe(),
                first_difference(&format!("{expected:?}"), &format!("{actual:?}"))
            );
        }
    }
}

/// The context of the first difference between two debug renderings.
fn first_difference(expected: &str, actual: &str) -> String {
    let index = expected
        .bytes()
        .zip(actual.bytes())
        .position(|(a, b)| a != b)
        .unwrap_or_else(|| expected.len().min(actual.len()));
    let start = index.saturating_sub(120);
    let end_e = (index + 120).min(expected.len());
    let end_a = (index + 120).min(actual.len());
    format!(
        "at character {index}:\n  C:    ...{}...\n  SIMD: ...{}...",
        &expected[start..end_e],
        &actual[start..end_a]
    )
}
