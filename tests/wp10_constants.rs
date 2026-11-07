//! WP10 differential test: the C sizes and exported tables of the VP8 encoder
//! core, the encoder version, and the filter strengths of
//! `VP8FilterStrengthFromDelta`, against `tools/oracle/probes/wp10/constants.c`.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::dsp::cost::{VP8_ENC_BANDS, VP8_ENTROPY_COST, VP8_LEVEL_FIXED_COSTS};
use webp::dsp::enc::VP8_DSP_SCAN;
use webp::enc::cost_enc::{VP8_FIXED_COSTS_I4, VP8_FIXED_COSTS_I16, VP8_FIXED_COSTS_UV};
use webp::enc::filter_enc::vp8_filter_strength_from_delta;
use webp::enc::frame_enc::{VP8_CAT3, VP8_CAT4, VP8_CAT5, VP8_CAT6};
use webp::enc::quant_enc::{VP8_SCAN, VP8I16_MODE_OFFSETS, VP8UV_MODE_OFFSETS};
use webp::enc::tree_enc::{VP8_COEFFS_PROBA0, VP8_COEFFS_UPDATE_PROBA};
use webp::enc::vp8i_enc::{
    C8DC8, C8HE8, C8TM8, C8VE8, DERROR_C_SIZE, ERROR_DIFFUSION_QUALITY, I4DC4, I4HD4, I4HE4, I4HU4,
    I4LD4, I4RD4, I4TM4, I4TMP, I4VE4, I4VL4, I4VR4, I16DC16, I16HE16, I16TM16, I16VE16,
    LF_STATS_C_SIZE, MAX_COST, MAX_LEVEL, MAX_LF_LEVELS, MAX_VARIABLE_LEVEL, PRED_SIZE_ENC, QFIX,
    U_OFF_ENC, V_OFF_ENC, VP8Encoder, VP8MBInfo, Y_OFF_ENC, YUV_SIZE_ENC,
};
use webp::enc::webp_enc::webp_get_encoder_version;

fn list<T: std::fmt::Display>(values: impl IntoIterator<Item = T>) -> String {
    values
        .into_iter()
        .map(|value| value.to_string())
        .collect::<Vec<_>>()
        .join(",")
}

#[test]
fn constants_and_tables_match_the_c_headers() {
    let Some(oracle) = Oracle::locate("constants_and_tables_match_the_c_headers") else {
        return;
    };
    let actual = oracle
        .probe("wp10", "constants")
        .run::<&str>(&[], &[])
        .success()
        .fields();
    let mut strengths = Vec::new();
    for sharpness in 0..8 {
        for delta in 0..80 {
            strengths.push(vp8_filter_strength_from_delta(sharpness, delta));
        }
    }
    let expected = Fields::new()
        .with("sizeof_VP8Encoder", VP8Encoder::C_SIZE)
        .with("sizeof_VP8MBInfo", VP8MBInfo::C_SIZE)
        .with("sizeof_DError", DERROR_C_SIZE)
        .with("sizeof_LFStats", LF_STATS_C_SIZE)
        .with("sizeof_VP8Tokens_header", size_of::<usize>())
        .with("version", webp_get_encoder_version())
        .with("MAX_LF_LEVELS", MAX_LF_LEVELS)
        .with("MAX_VARIABLE_LEVEL", MAX_VARIABLE_LEVEL)
        .with("MAX_LEVEL", MAX_LEVEL)
        .with("YUV_SIZE_ENC", YUV_SIZE_ENC)
        .with("PRED_SIZE_ENC", PRED_SIZE_ENC)
        .with("Y_OFF_ENC", Y_OFF_ENC)
        .with("U_OFF_ENC", U_OFF_ENC)
        .with("V_OFF_ENC", V_OFF_ENC)
        .with("I16", list([I16DC16, I16TM16, I16VE16, I16HE16]))
        .with("C8", list([C8DC8, C8TM8, C8VE8, C8HE8]))
        .with(
            "I4",
            list([
                I4DC4, I4TM4, I4VE4, I4HE4, I4RD4, I4VR4, I4LD4, I4VL4, I4HD4, I4HU4, I4TMP,
            ]),
        )
        .with("MAX_COST", MAX_COST)
        .with("QFIX", QFIX)
        .with("ERROR_DIFFUSION_QUALITY", ERROR_DIFFUSION_QUALITY)
        .with("VP8EntropyCost", list(VP8_ENTROPY_COST))
        .with("VP8LevelFixedCosts", list(VP8_LEVEL_FIXED_COSTS))
        .with("VP8EncBands", list(VP8_ENC_BANDS))
        .with("VP8FixedCostsUV", list(VP8_FIXED_COSTS_UV))
        .with("VP8FixedCostsI16", list(VP8_FIXED_COSTS_I16))
        .with(
            "VP8FixedCostsI4",
            list(VP8_FIXED_COSTS_I4.as_flattened().as_flattened().iter()),
        )
        .with(
            "VP8CoeffsProba0",
            list(
                VP8_COEFFS_PROBA0
                    .as_flattened()
                    .as_flattened()
                    .as_flattened()
                    .iter(),
            ),
        )
        .with(
            "VP8CoeffsUpdateProba",
            list(
                VP8_COEFFS_UPDATE_PROBA
                    .as_flattened()
                    .as_flattened()
                    .as_flattened()
                    .iter(),
            ),
        )
        .with("VP8Cat3", list(VP8_CAT3))
        .with("VP8Cat4", list(VP8_CAT4))
        .with("VP8Cat5", list(VP8_CAT5))
        .with("VP8Cat6", list(VP8_CAT6))
        .with("VP8Scan", list(VP8_SCAN))
        .with("VP8I16ModeOffsets", list(VP8I16_MODE_OFFSETS))
        .with("VP8UVModeOffsets", list(VP8UV_MODE_OFFSETS))
        .with("VP8DspScan", list(VP8_DSP_SCAN))
        .with("VP8FilterStrengthFromDelta", list(strengths));
    assert_eq!(actual.keys(), expected.keys());
    for ((key, c), (_, rust)) in actual.0.iter().zip(&expected.0) {
        assert_eq!(c, rust, "{key}");
    }
}
