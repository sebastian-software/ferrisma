//! WP12 differential test: size, alignment and field offsets of the `repr(C)`
//! mirrors in `src/abi/decode.rs` against the C compiler, through the probe
//! `tools/oracle/probes/wp12/layout.c` (ADR 0004).

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;
use std::mem::{align_of, offset_of, size_of};

use oracle::Oracle;
use webp::abi::decode::{
    WebPDecBuffer, WebPDecBufferUnion, WebPDecoderConfig, WebPRGBABuffer, WebPYUVABuffer,
};
use webp::webp::decode::{VP8StatusCode, WEBP_CSP_MODE, WebPBitstreamFeatures, WebPDecoderOptions};

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
        ($t:ty, $name:literal, $($f:tt).+) => {
            writeln!(expected, "{}.{}={}", $name, stringify!($($f).+), offset_of!($t, $($f).+)).unwrap()
        };
    }
    structure!(WebPRGBABuffer, "WebPRGBABuffer");
    field!(WebPRGBABuffer, "WebPRGBABuffer", rgba);
    field!(WebPRGBABuffer, "WebPRGBABuffer", stride);
    field!(WebPRGBABuffer, "WebPRGBABuffer", size);
    structure!(WebPYUVABuffer, "WebPYUVABuffer");
    field!(WebPYUVABuffer, "WebPYUVABuffer", y);
    field!(WebPYUVABuffer, "WebPYUVABuffer", u);
    field!(WebPYUVABuffer, "WebPYUVABuffer", v);
    field!(WebPYUVABuffer, "WebPYUVABuffer", a);
    field!(WebPYUVABuffer, "WebPYUVABuffer", y_stride);
    field!(WebPYUVABuffer, "WebPYUVABuffer", u_stride);
    field!(WebPYUVABuffer, "WebPYUVABuffer", v_stride);
    field!(WebPYUVABuffer, "WebPYUVABuffer", a_stride);
    field!(WebPYUVABuffer, "WebPYUVABuffer", y_size);
    field!(WebPYUVABuffer, "WebPYUVABuffer", u_size);
    field!(WebPYUVABuffer, "WebPYUVABuffer", v_size);
    field!(WebPYUVABuffer, "WebPYUVABuffer", a_size);
    structure!(WebPDecBuffer, "WebPDecBuffer");
    field!(WebPDecBuffer, "WebPDecBuffer", colorspace);
    field!(WebPDecBuffer, "WebPDecBuffer", width);
    field!(WebPDecBuffer, "WebPDecBuffer", height);
    field!(WebPDecBuffer, "WebPDecBuffer", is_external_memory);
    field!(WebPDecBuffer, "WebPDecBuffer", u);
    writeln!(
        expected,
        "WebPDecBuffer.u.RGBA={}",
        offset_of!(WebPDecBuffer, u) + offset_of!(WebPDecBufferUnion, RGBA)
    )
    .unwrap();
    writeln!(
        expected,
        "WebPDecBuffer.u.YUVA={}",
        offset_of!(WebPDecBuffer, u) + offset_of!(WebPDecBufferUnion, YUVA)
    )
    .unwrap();
    field!(WebPDecBuffer, "WebPDecBuffer", pad);
    field!(WebPDecBuffer, "WebPDecBuffer", private_memory);
    writeln!(
        expected,
        "WebPDecBuffer.u={},{}",
        size_of::<WebPDecBufferUnion>(),
        align_of::<WebPDecBufferUnion>()
    )
    .unwrap();
    structure!(WebPDecoderConfig, "WebPDecoderConfig");
    field!(WebPDecoderConfig, "WebPDecoderConfig", input);
    field!(WebPDecoderConfig, "WebPDecoderConfig", output);
    field!(WebPDecoderConfig, "WebPDecoderConfig", options);
    structure!(WebPBitstreamFeatures, "WebPBitstreamFeatures");
    structure!(WebPDecoderOptions, "WebPDecoderOptions");
    structure!(WEBP_CSP_MODE, "WEBP_CSP_MODE");
    structure!(VP8StatusCode, "VP8StatusCode");

    let output = oracle.probe("wp12", "layout").run::<&str>(&[], b"");
    output.success();
    assert_eq!(String::from_utf8_lossy(&output.stdout), expected);
}
