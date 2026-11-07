//! WP4 differential test: the constants, enumerators, macros and struct layouts
//! of `src/dsp/dsp.h`, `src/dsp/yuv.h`, `src/dsp/cpu.h` and
//! `src/utils/rescaler_utils.h`. The probe `tools/oracle/probes/wp4/constants.c`
//! prints each value under its C name; this test compares them with the Rust
//! constants, and the layouts of the `repr(C)` structs with the C compiler's.
//! `WebPRescaler` is not `repr(C)`, so only its C size, `WebPRescaler::C_SIZE`,
//! is compared.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::Oracle;
use std::mem::{offset_of, size_of};
use webp::dsp::cpu::CPUFeature;
use webp::dsp::dsp::{
    BPS, MAX_COEFF_THRESH, VP8_DITHER_AMP_BITS, VP8_DITHER_AMP_CENTER, VP8_DITHER_DESCALE,
    VP8_DITHER_DESCALE_ROUNDER, VP8_SSIM_KERNEL, VP8DistoStats, VP8Histogram, WEBP_FILTER_TYPE,
    WEBP_TRANSFORM_AC3_C1, WEBP_TRANSFORM_AC3_C2, webp_transform_ac3_mul1, webp_transform_ac3_mul2,
};
use webp::dsp::yuv::{YUV_FIX, YUV_FIX2, YUV_HALF, YUV_MASK2};
use webp::utils::rescaler_utils::{
    WEBP_RESCALER_ONE, WEBP_RESCALER_RFIX, WebPRescaler, webp_rescaler_frac,
};

#[test]
fn header_constants_match_the_c_headers() {
    let Some(oracle) = Oracle::locate("header_constants_match_the_c_headers") else {
        return;
    };
    let no_args: [&str; 0] = [];
    let output = oracle.probe("wp4", "constants").run(&no_args, b"");
    let fields = output.success().fields();
    let expected: Vec<(&str, i64)> = vec![
        ("BPS", i64::from(BPS)),
        ("MAX_COEFF_THRESH", i64::from(MAX_COEFF_THRESH)),
        ("VP8_SSIM_KERNEL", i64::from(VP8_SSIM_KERNEL)),
        ("VP8_DITHER_DESCALE", i64::from(VP8_DITHER_DESCALE)),
        (
            "VP8_DITHER_DESCALE_ROUNDER",
            i64::from(VP8_DITHER_DESCALE_ROUNDER),
        ),
        ("VP8_DITHER_AMP_BITS", i64::from(VP8_DITHER_AMP_BITS)),
        ("VP8_DITHER_AMP_CENTER", i64::from(VP8_DITHER_AMP_CENTER)),
        ("WEBP_TRANSFORM_AC3_C1", i64::from(WEBP_TRANSFORM_AC3_C1)),
        ("WEBP_TRANSFORM_AC3_C2", i64::from(WEBP_TRANSFORM_AC3_C2)),
        (
            "WEBP_TRANSFORM_AC3_MUL1_100",
            i64::from(webp_transform_ac3_mul1(100)),
        ),
        (
            "WEBP_TRANSFORM_AC3_MUL2_100",
            i64::from(webp_transform_ac3_mul2(100)),
        ),
        (
            "WEBP_TRANSFORM_AC3_MUL1_m7",
            i64::from(webp_transform_ac3_mul1(-7)),
        ),
        (
            "WEBP_TRANSFORM_AC3_MUL2_m7",
            i64::from(webp_transform_ac3_mul2(-7)),
        ),
        (
            "WEBP_FILTER_NONE",
            i64::from(WEBP_FILTER_TYPE::WEBP_FILTER_NONE.0),
        ),
        (
            "WEBP_FILTER_HORIZONTAL",
            i64::from(WEBP_FILTER_TYPE::WEBP_FILTER_HORIZONTAL.0),
        ),
        (
            "WEBP_FILTER_VERTICAL",
            i64::from(WEBP_FILTER_TYPE::WEBP_FILTER_VERTICAL.0),
        ),
        (
            "WEBP_FILTER_GRADIENT",
            i64::from(WEBP_FILTER_TYPE::WEBP_FILTER_GRADIENT.0),
        ),
        (
            "WEBP_FILTER_LAST",
            i64::from(WEBP_FILTER_TYPE::WEBP_FILTER_LAST.0),
        ),
        (
            "WEBP_FILTER_BEST",
            i64::from(WEBP_FILTER_TYPE::WEBP_FILTER_BEST.0),
        ),
        (
            "WEBP_FILTER_FAST",
            i64::from(WEBP_FILTER_TYPE::WEBP_FILTER_FAST.0),
        ),
        ("YUV_FIX", i64::from(YUV_FIX)),
        ("YUV_HALF", i64::from(YUV_HALF)),
        ("YUV_FIX2", i64::from(YUV_FIX2)),
        ("YUV_MASK2", i64::from(YUV_MASK2)),
        ("WEBP_RESCALER_RFIX", i64::from(WEBP_RESCALER_RFIX)),
        ("kSSE2", i64::from(CPUFeature::kSSE2.0)),
        ("kSSE3", i64::from(CPUFeature::kSSE3.0)),
        ("kSlowSSSE3", i64::from(CPUFeature::kSlowSSSE3.0)),
        ("kSSE4_1", i64::from(CPUFeature::kSSE4_1.0)),
        ("kAVX", i64::from(CPUFeature::kAVX.0)),
        ("kAVX2", i64::from(CPUFeature::kAVX2.0)),
        ("kNEON", i64::from(CPUFeature::kNEON.0)),
        ("kMIPS32", i64::from(CPUFeature::kMIPS32.0)),
        ("kMIPSdspR2", i64::from(CPUFeature::kMIPSdspR2.0)),
        ("kMSA", i64::from(CPUFeature::kMSA.0)),
        ("sizeof_VP8Histogram", size_of::<VP8Histogram>() as i64),
        ("sizeof_VP8DistoStats", size_of::<VP8DistoStats>() as i64),
        ("sizeof_WebPRescaler", WebPRescaler::C_SIZE as i64),
        (
            "offsetof_VP8DistoStats_w",
            offset_of!(VP8DistoStats, w) as i64,
        ),
        (
            "offsetof_VP8DistoStats_xm",
            offset_of!(VP8DistoStats, xm) as i64,
        ),
        (
            "offsetof_VP8DistoStats_ym",
            offset_of!(VP8DistoStats, ym) as i64,
        ),
        (
            "offsetof_VP8DistoStats_xxm",
            offset_of!(VP8DistoStats, xxm) as i64,
        ),
        (
            "offsetof_VP8DistoStats_xym",
            offset_of!(VP8DistoStats, xym) as i64,
        ),
        (
            "offsetof_VP8DistoStats_yym",
            offset_of!(VP8DistoStats, yym) as i64,
        ),
        (
            "offsetof_VP8Histogram_max",
            offset_of!(VP8Histogram, max_value) as i64,
        ),
        (
            "offsetof_VP8Histogram_last",
            offset_of!(VP8Histogram, last_non_zero) as i64,
        ),
    ];
    for (name, value) in &expected {
        assert_eq!(fields.int(name), *value, "{name}");
    }
    assert_eq!(fields.int("WEBP_RESCALER_ONE") as u64, WEBP_RESCALER_ONE);
    assert_eq!(
        fields.int("WEBP_RESCALER_FRAC_1_3") as u32,
        webp_rescaler_frac(1, 3)
    );
    assert_eq!(
        fields.int("WEBP_RESCALER_FRAC_m5_7") as u32,
        webp_rescaler_frac(-5, 7)
    );
}
