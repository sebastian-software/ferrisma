//! WP9 differential test: `WebPConfigInitInternal`, the `WebPConfigInit` and
//! `WebPConfigPreset` wrappers, `WebPConfigLosslessPreset` and `WebPValidateConfig`
//! (`src/enc/config_enc.c`, `src/webp/encode.rs`). The probe
//! `tools/oracle/probes/wp9/config.c` runs the C functions on a config filled with
//! `0xa5` bytes, as the test fills the Rust config, and prints the return values and
//! every field. Floats are compared as their bits.
//!
//! The cases cover every preset, a preset outside the enumeration, quality values at
//! and beyond the range (including NaN and infinities), an incompatible ABI version,
//! every lossless level and one level outside the range, and one field at a time at
//! its boundary values for `WebPValidateConfig`.

#[path = "support/oracle.rs"]
mod oracle;

use oracle::{Fields, Oracle};
use webp::enc::config_enc::{
    webp_config_init_internal, webp_config_lossless_preset, webp_validate_config,
};
use webp::webp::encode::{
    WEBP_ENCODER_ABI_VERSION, WebPConfig, WebPImageHint, WebPPreset, webp_config_init,
    webp_config_preset,
};

/// A config whose every field holds the bytes `0xa5`, as the probe's `memset` leaves it.
fn filled() -> WebPConfig {
    let word = 0xa5a5_a5a5_u32 as i32;
    let float = f32::from_bits(0xa5a5_a5a5);
    WebPConfig {
        lossless: word,
        quality: float,
        method: word,
        image_hint: WebPImageHint(0xa5a5_a5a5),
        target_size: word,
        target_psnr: float,
        segments: word,
        sns_strength: word,
        filter_strength: word,
        filter_sharpness: word,
        filter_type: word,
        autofilter: word,
        alpha_compression: word,
        alpha_filtering: word,
        alpha_quality: word,
        pass: word,
        show_compressed: word,
        preprocessing: word,
        partitions: word,
        partition_limit: word,
        emulate_jpeg_size: word,
        thread_level: word,
        low_memory: word,
        near_lossless: word,
        exact: word,
        use_delta_palette: word,
        use_sharp_yuv: word,
        qmin: word,
        qmax: word,
    }
}

/// The output of the probe for a config, after the return value `ret`.
fn fields_of(ret: bool, config: &WebPConfig) -> Fields {
    Fields::new()
        .with("ret", i32::from(ret))
        .with("lossless", config.lossless)
        .with("quality", format!("{:08x}", config.quality.to_bits()))
        .with("method", config.method)
        .with("image_hint", config.image_hint.0)
        .with("target_size", config.target_size)
        .with(
            "target_psnr",
            format!("{:08x}", config.target_psnr.to_bits()),
        )
        .with("segments", config.segments)
        .with("sns_strength", config.sns_strength)
        .with("filter_strength", config.filter_strength)
        .with("filter_sharpness", config.filter_sharpness)
        .with("filter_type", config.filter_type)
        .with("autofilter", config.autofilter)
        .with("alpha_compression", config.alpha_compression)
        .with("alpha_filtering", config.alpha_filtering)
        .with("alpha_quality", config.alpha_quality)
        .with("pass", config.pass)
        .with("show_compressed", config.show_compressed)
        .with("preprocessing", config.preprocessing)
        .with("partitions", config.partitions)
        .with("partition_limit", config.partition_limit)
        .with("emulate_jpeg_size", config.emulate_jpeg_size)
        .with("thread_level", config.thread_level)
        .with("low_memory", config.low_memory)
        .with("near_lossless", config.near_lossless)
        .with("exact", config.exact)
        .with("use_delta_palette", config.use_delta_palette)
        .with("use_sharp_yuv", config.use_sharp_yuv)
        .with("qmin", config.qmin)
        .with("qmax", config.qmax)
}

/// Appends the expected output of one probe block to `expected`.
fn push(expected: Fields, block: Fields) -> Fields {
    let mut all = expected;
    all.0.extend(block.0);
    all
}

#[test]
fn config_init_presets_and_wrappers_match_the_c_config() {
    let Some(oracle) = Oracle::locate("config_init_presets_and_wrappers_match_the_c_config") else {
        return;
    };
    let probe = oracle.probe("wp9", "config");

    let presets: [u32; 8] = [0, 1, 2, 3, 4, 5, 6, 17];
    let qualities: [f32; 8] = [0.0, 75.0, 100.0, 50.5, -0.5, 100.5, f32::NAN, f32::INFINITY];
    let versions: [i32; 5] = [WEBP_ENCODER_ABI_VERSION, 0x0211, 0x0200, 0x0100, 0x0310];

    let mut stdin = String::new();
    let mut expected = Fields::new();

    for &version in &versions {
        for &preset in &presets {
            for &quality in &qualities {
                stdin.push_str(&format!(
                    "init {version} {preset} {:08x}\n",
                    quality.to_bits()
                ));
                let mut config = filled();
                let ret =
                    webp_config_init_internal(&mut config, WebPPreset(preset), quality, version);
                expected = push(expected, fields_of(ret, &config));
            }
        }
    }

    for &preset in &presets {
        for &quality in &qualities {
            stdin.push_str(&format!("preset {preset} {:08x}\n", quality.to_bits()));
            let mut config = filled();
            let ret = webp_config_preset(&mut config, WebPPreset(preset), quality);
            expected = push(expected, fields_of(ret, &config));
        }
    }

    stdin.push_str("default\n");
    let mut config = filled();
    let ret = webp_config_init(&mut config);
    expected = push(expected, fields_of(ret, &config));

    for level in -1_i32..=10 {
        stdin.push_str(&format!("lossless {level}\n"));
        let mut config = filled();
        let _ = webp_config_init(&mut config);
        let ret = webp_config_lossless_preset(&mut config, level);
        expected = push(expected, fields_of(ret, &config));
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

#[test]
fn config_validation_at_the_boundaries_matches_the_c_validator() {
    let Some(oracle) =
        Oracle::locate("config_validation_at_the_boundaries_matches_the_c_validator")
    else {
        return;
    };
    let probe = oracle.probe("wp9", "config");

    // The 29 fields in struct order; floats are given as their bits.
    let base: Vec<String> = {
        let mut config = filled();
        let _ = webp_config_init(&mut config);
        fields_of(true, &config)
            .0
            .into_iter()
            .skip(1)
            .map(|(_, value)| value)
            .collect()
    };
    // The base values are hex for the two floats, so the candidates follow the same form.
    let float_candidates: [f32; 8] = [-1.0, -0.0, 0.0, 30.0, 100.0, 100.5, f32::NAN, f32::INFINITY];
    let int_candidates: [i64; 10] = [-1, 0, 1, 2, 4, 6, 7, 10, 100, 101];
    let candidates = |index: usize| -> Vec<String> {
        match index {
            1 | 5 => float_candidates
                .iter()
                .map(|value| format!("{:08x}", value.to_bits()))
                .collect(),
            _ => int_candidates
                .iter()
                .map(|value| value.to_string())
                .collect(),
        }
    };

    let mut stdin = String::new();
    let mut expected = Fields::new();
    let emit = |values: &[String], stdin: &mut String, expected: Fields| -> Fields {
        stdin.push_str("validate");
        for value in values {
            stdin.push(' ');
            stdin.push_str(value);
        }
        stdin.push('\n');
        let config = config_from_fields(values);
        push(
            expected,
            Fields::new().with("ret", i32::from(webp_validate_config(&config))),
        )
    };

    expected = emit(&base, &mut stdin, expected);
    for index in 0..29 {
        for candidate in candidates(index) {
            let mut values = base.clone();
            values[index] = candidate;
            expected = emit(&values, &mut stdin, expected);
        }
    }

    let c_output = probe.run::<&str>(&[], stdin.as_bytes()).success().fields();
    assert_eq!(c_output, expected);
}

/// Builds the config that the probe's `validate` command builds from the same fields.
fn config_from_fields(values: &[String]) -> WebPConfig {
    let int = |index: usize| -> i32 { values[index].parse::<i64>().unwrap() as i32 };
    let float =
        |index: usize| -> f32 { f32::from_bits(u32::from_str_radix(&values[index], 16).unwrap()) };
    WebPConfig {
        lossless: int(0),
        quality: float(1),
        method: int(2),
        image_hint: WebPImageHint(int(3) as u32),
        target_size: int(4),
        target_psnr: float(5),
        segments: int(6),
        sns_strength: int(7),
        filter_strength: int(8),
        filter_sharpness: int(9),
        filter_type: int(10),
        autofilter: int(11),
        alpha_compression: int(12),
        alpha_filtering: int(13),
        alpha_quality: int(14),
        pass: int(15),
        show_compressed: int(16),
        preprocessing: int(17),
        partitions: int(18),
        partition_limit: int(19),
        emulate_jpeg_size: int(20),
        thread_level: int(21),
        low_memory: int(22),
        near_lossless: int(23),
        exact: int(24),
        use_delta_palette: int(25),
        use_sharp_yuv: int(26),
        qmin: int(27),
        qmax: int(28),
    }
}
