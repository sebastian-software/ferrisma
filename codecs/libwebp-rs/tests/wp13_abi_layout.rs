//! WP13 differential test: size, alignment and field offsets of the `repr(C)`
//! mirrors in `src/abi/encode.rs` (`WebPPicture`, `WebPMemoryWriter`) and of the
//! structs that the C ABI shares with `src/webp/encode.rs` (`WebPConfig`,
//! `WebPAuxStats`), and the sizes of the enums and callback pointers, against
//! the C compiler, through the probe `tools/oracle/probes/wp13/layout.c`
//! (ADR 0004).

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;
use std::mem::{align_of, offset_of, size_of};

use oracle::Oracle;
use webp::abi::encode::{WebPMemoryWriter, WebPPicture, WebPProgressHook, WebPWriterFunction};
use webp::webp::encode::{
    WebPAuxStats, WebPConfig, WebPEncCSP, WebPEncodingError, WebPImageHint, WebPPreset,
};

#[test]
fn abi_mirrors_match_the_c_layout() {
    let Some(oracle) = Oracle::locate("abi_mirrors_match_the_c_layout") else {
        return;
    };
    let mut expected = String::new();
    macro_rules! structure {
        ($t:ty, $name:literal) => {
            writeln!(
                expected,
                "{}={},{}",
                $name,
                size_of::<$t>(),
                align_of::<$t>()
            )
            .unwrap()
        };
    }
    macro_rules! field {
        ($t:ty, $name:literal, $c:literal, $f:ident) => {
            writeln!(expected, "{}.{}={}", $name, $c, offset_of!($t, $f)).unwrap()
        };
    }
    structure!(WebPPicture, "WebPPicture");
    field!(WebPPicture, "WebPPicture", "use_argb", use_argb);
    field!(WebPPicture, "WebPPicture", "colorspace", colorspace);
    field!(WebPPicture, "WebPPicture", "width", width);
    field!(WebPPicture, "WebPPicture", "height", height);
    field!(WebPPicture, "WebPPicture", "y", y);
    field!(WebPPicture, "WebPPicture", "u", u);
    field!(WebPPicture, "WebPPicture", "v", v);
    field!(WebPPicture, "WebPPicture", "y_stride", y_stride);
    field!(WebPPicture, "WebPPicture", "uv_stride", uv_stride);
    field!(WebPPicture, "WebPPicture", "a", a);
    field!(WebPPicture, "WebPPicture", "a_stride", a_stride);
    field!(WebPPicture, "WebPPicture", "pad1", pad1);
    field!(WebPPicture, "WebPPicture", "argb", argb);
    field!(WebPPicture, "WebPPicture", "argb_stride", argb_stride);
    field!(WebPPicture, "WebPPicture", "pad2", pad2);
    field!(WebPPicture, "WebPPicture", "writer", writer);
    field!(WebPPicture, "WebPPicture", "custom_ptr", custom_ptr);
    field!(
        WebPPicture,
        "WebPPicture",
        "extra_info_type",
        extra_info_type
    );
    field!(WebPPicture, "WebPPicture", "extra_info", extra_info);
    field!(WebPPicture, "WebPPicture", "stats", stats);
    field!(WebPPicture, "WebPPicture", "error_code", error_code);
    field!(WebPPicture, "WebPPicture", "progress_hook", progress_hook);
    field!(WebPPicture, "WebPPicture", "user_data", user_data);
    field!(WebPPicture, "WebPPicture", "pad3", pad3);
    field!(WebPPicture, "WebPPicture", "pad4", pad4);
    field!(WebPPicture, "WebPPicture", "pad5", pad5);
    field!(WebPPicture, "WebPPicture", "pad6", pad6);
    field!(WebPPicture, "WebPPicture", "memory_", memory_);
    field!(WebPPicture, "WebPPicture", "memory_argb_", memory_argb_);
    field!(WebPPicture, "WebPPicture", "pad7", pad7);
    structure!(WebPMemoryWriter, "WebPMemoryWriter");
    field!(WebPMemoryWriter, "WebPMemoryWriter", "mem", mem);
    field!(WebPMemoryWriter, "WebPMemoryWriter", "size", size);
    field!(WebPMemoryWriter, "WebPMemoryWriter", "max_size", max_size);
    field!(WebPMemoryWriter, "WebPMemoryWriter", "pad", pad);
    structure!(WebPConfig, "WebPConfig");
    field!(WebPConfig, "WebPConfig", "lossless", lossless);
    field!(WebPConfig, "WebPConfig", "quality", quality);
    field!(WebPConfig, "WebPConfig", "method", method);
    field!(WebPConfig, "WebPConfig", "image_hint", image_hint);
    field!(WebPConfig, "WebPConfig", "target_size", target_size);
    field!(WebPConfig, "WebPConfig", "target_PSNR", target_psnr);
    field!(WebPConfig, "WebPConfig", "segments", segments);
    field!(WebPConfig, "WebPConfig", "sns_strength", sns_strength);
    field!(WebPConfig, "WebPConfig", "filter_strength", filter_strength);
    field!(
        WebPConfig,
        "WebPConfig",
        "filter_sharpness",
        filter_sharpness
    );
    field!(WebPConfig, "WebPConfig", "filter_type", filter_type);
    field!(WebPConfig, "WebPConfig", "autofilter", autofilter);
    field!(
        WebPConfig,
        "WebPConfig",
        "alpha_compression",
        alpha_compression
    );
    field!(WebPConfig, "WebPConfig", "alpha_filtering", alpha_filtering);
    field!(WebPConfig, "WebPConfig", "alpha_quality", alpha_quality);
    field!(WebPConfig, "WebPConfig", "pass", pass);
    field!(WebPConfig, "WebPConfig", "show_compressed", show_compressed);
    field!(WebPConfig, "WebPConfig", "preprocessing", preprocessing);
    field!(WebPConfig, "WebPConfig", "partitions", partitions);
    field!(WebPConfig, "WebPConfig", "partition_limit", partition_limit);
    field!(
        WebPConfig,
        "WebPConfig",
        "emulate_jpeg_size",
        emulate_jpeg_size
    );
    field!(WebPConfig, "WebPConfig", "thread_level", thread_level);
    field!(WebPConfig, "WebPConfig", "low_memory", low_memory);
    field!(WebPConfig, "WebPConfig", "near_lossless", near_lossless);
    field!(WebPConfig, "WebPConfig", "exact", exact);
    field!(
        WebPConfig,
        "WebPConfig",
        "use_delta_palette",
        use_delta_palette
    );
    field!(WebPConfig, "WebPConfig", "use_sharp_yuv", use_sharp_yuv);
    field!(WebPConfig, "WebPConfig", "qmin", qmin);
    field!(WebPConfig, "WebPConfig", "qmax", qmax);
    structure!(WebPAuxStats, "WebPAuxStats");
    field!(WebPAuxStats, "WebPAuxStats", "coded_size", coded_size);
    field!(WebPAuxStats, "WebPAuxStats", "PSNR", psnr);
    field!(WebPAuxStats, "WebPAuxStats", "block_count", block_count);
    field!(WebPAuxStats, "WebPAuxStats", "header_bytes", header_bytes);
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "residual_bytes",
        residual_bytes
    );
    field!(WebPAuxStats, "WebPAuxStats", "segment_size", segment_size);
    field!(WebPAuxStats, "WebPAuxStats", "segment_quant", segment_quant);
    field!(WebPAuxStats, "WebPAuxStats", "segment_level", segment_level);
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "alpha_data_size",
        alpha_data_size
    );
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "layer_data_size",
        layer_data_size
    );
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "lossless_features",
        lossless_features
    );
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "histogram_bits",
        histogram_bits
    );
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "transform_bits",
        transform_bits
    );
    field!(WebPAuxStats, "WebPAuxStats", "cache_bits", cache_bits);
    field!(WebPAuxStats, "WebPAuxStats", "palette_size", palette_size);
    field!(WebPAuxStats, "WebPAuxStats", "lossless_size", lossless_size);
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "lossless_hdr_size",
        lossless_hdr_size
    );
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "lossless_data_size",
        lossless_data_size
    );
    field!(
        WebPAuxStats,
        "WebPAuxStats",
        "cross_color_transform_bits",
        cross_color_transform_bits
    );
    field!(WebPAuxStats, "WebPAuxStats", "pad", pad);
    structure!(WebPEncCSP, "WebPEncCSP");
    structure!(WebPEncodingError, "WebPEncodingError");
    structure!(WebPPreset, "WebPPreset");
    structure!(WebPImageHint, "WebPImageHint");
    structure!(WebPWriterFunction, "WebPWriterFunction");
    structure!(WebPProgressHook, "WebPProgressHook");
    let output = oracle.probe("wp13", "layout").run::<&str>(&[], b"");
    output.success();
    assert_eq!(String::from_utf8(output.stdout.clone()).unwrap(), expected);
}
