//! The x86 SIMD levels of the WP18 and WP19 cases (ADR 0008): a case selects
//! its level through the C ABI's `VP8GetCPUInfo` pointer, which
//! `webp::dsp::cpu::features()` follows, as the tests of `tests/wp18_*.rs`
//! do. The C level installs `NULL` (`-noasm`); a SIMD level installs a query
//! that answers the running CPU's flags up to that level, so that the dispatch
//! functions select exactly the variants that upstream's `Init` functions
//! assign there. Each run stores the pointer again before the call; the store
//! is a plain write, and `features()` stays cached while the pointer is the
//! same.

use core::ffi::{c_int, c_void};
use std::sync::atomic::Ordering;

use webp::abi::decode::VP8_GET_CPU_INFO;
use webp::dsp::cpu::{CPUFeature, vp8_get_cpu_info};

use crate::Case;

/// A selection level: the variant name of the table and the highest feature
/// its query answers (`None` for the C translation).
#[derive(Clone, Copy)]
pub struct Level {
    pub variant: &'static str,
    highest: Option<CPUFeature>,
}

/// The C translation (`VP8GetCPUInfo = NULL`).
pub const C: Level = Level {
    variant: "C",
    highest: None,
};
/// The SSE2 variants.
pub const SSE2: Level = Level {
    variant: "SSE2",
    highest: Some(CPUFeature::kSSE2),
};
/// The SSE2 variants with the SSE4.1 overrides.
pub const SSE41: Level = Level {
    variant: "SSE41",
    highest: Some(CPUFeature::kSSE4_1),
};
/// Every x86 variant, AVX2 included.
pub const AVX2: Level = Level {
    variant: "AVX2",
    highest: Some(CPUFeature::kAVX2),
};

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

impl Level {
    fn pointer(self) -> *mut c_void {
        match self.highest {
            None => core::ptr::null_mut(),
            Some(CPUFeature::kSSE2) => query_sse2 as *mut c_void,
            Some(CPUFeature::kSSE4_1) => query_sse41 as *mut c_void,
            Some(_) => query_avx2 as *mut c_void,
        }
    }

    /// Installs the level for the next dispatch calls.
    pub fn select(self) {
        VP8_GET_CPU_INFO.store(self.pointer(), Ordering::Relaxed);
    }

    /// A case of `function` at this level: `run` after the selection.
    pub fn case(
        self,
        module: &'static str,
        function: &'static str,
        mut run: impl FnMut() + 'static,
    ) -> Case {
        Case {
            module,
            function,
            variant: self.variant,
            run: Box::new(move || {
                self.select();
                run();
            }),
        }
    }
}

/// The C level and those of `simd` that this build and host have: none
/// without the `simd` feature or off x86, where no x86 variant is compiled.
pub fn levels(simd: &[Level]) -> Vec<Level> {
    let mut levels = vec![C];
    if cfg!(all(
        feature = "simd",
        any(target_arch = "x86", target_arch = "x86_64")
    )) {
        levels.extend(
            simd.iter()
                .copied()
                .filter(|level| level.highest.is_some_and(vp8_get_cpu_info)),
        );
    }
    levels
}
