//! WP15 differential test: size, alignment and field offsets of the `repr(C)`
//! mirror `WebPMuxFrameInfo` in `src/abi/mux.rs` and of the structs that the C
//! ABI shares with `src/webp/mux.rs` (`WebPMuxAnimParams`,
//! `WebPAnimEncoderOptions`), the sizes and signedness of the enums it passes
//! (`WebPMuxError`, `WebPChunkId`), and the values of `WEBP_MUX_ABI_VERSION`
//! and the enumerators, against the C compiler, through the probe
//! `tools/oracle/probes/wp15/layout.c` (ADR 0004).

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;
use std::mem::{align_of, offset_of, size_of};

use oracle::Oracle;
use webp::abi::mux::WebPMuxFrameInfo;
use webp::webp::mux::{
    WEBP_MUX_ABI_VERSION, WebPAnimEncoderOptions, WebPChunkId, WebPMuxAnimParams, WebPMuxError,
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
        ($t:ty, $name:literal, $f:ident) => {
            writeln!(
                expected,
                "{}.{}={}",
                $name,
                stringify!($f),
                offset_of!($t, $f)
            )
            .unwrap()
        };
    }
    macro_rules! value {
        ($name:literal, $value:expr) => {
            writeln!(expected, "{}={}", $name, $value).unwrap()
        };
    }
    structure!(WebPMuxFrameInfo, "WebPMuxFrameInfo");
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", bitstream);
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", x_offset);
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", y_offset);
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", duration);
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", id);
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", dispose_method);
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", blend_method);
    field!(WebPMuxFrameInfo, "WebPMuxFrameInfo", pad);
    structure!(WebPMuxAnimParams, "WebPMuxAnimParams");
    field!(WebPMuxAnimParams, "WebPMuxAnimParams", bgcolor);
    field!(WebPMuxAnimParams, "WebPMuxAnimParams", loop_count);
    structure!(WebPAnimEncoderOptions, "WebPAnimEncoderOptions");
    field!(
        WebPAnimEncoderOptions,
        "WebPAnimEncoderOptions",
        anim_params
    );
    field!(
        WebPAnimEncoderOptions,
        "WebPAnimEncoderOptions",
        minimize_size
    );
    field!(WebPAnimEncoderOptions, "WebPAnimEncoderOptions", kmin);
    field!(WebPAnimEncoderOptions, "WebPAnimEncoderOptions", kmax);
    field!(
        WebPAnimEncoderOptions,
        "WebPAnimEncoderOptions",
        allow_mixed
    );
    field!(WebPAnimEncoderOptions, "WebPAnimEncoderOptions", verbose);
    field!(WebPAnimEncoderOptions, "WebPAnimEncoderOptions", padding);
    structure!(WebPMuxError, "WebPMuxError");
    structure!(WebPChunkId, "WebPChunkId");
    value!("WEBP_MUX_ABI_VERSION", WEBP_MUX_ABI_VERSION);
    value!("WEBP_MUX_OK", WebPMuxError::WEBP_MUX_OK.0);
    value!("WEBP_MUX_NOT_FOUND", WebPMuxError::WEBP_MUX_NOT_FOUND.0);
    value!(
        "WEBP_MUX_INVALID_ARGUMENT",
        WebPMuxError::WEBP_MUX_INVALID_ARGUMENT.0
    );
    value!("WEBP_MUX_BAD_DATA", WebPMuxError::WEBP_MUX_BAD_DATA.0);
    value!(
        "WEBP_MUX_MEMORY_ERROR",
        WebPMuxError::WEBP_MUX_MEMORY_ERROR.0
    );
    value!(
        "WEBP_MUX_NOT_ENOUGH_DATA",
        WebPMuxError::WEBP_MUX_NOT_ENOUGH_DATA.0
    );
    value!("WEBP_CHUNK_VP8X", WebPChunkId::WEBP_CHUNK_VP8X.0);
    value!("WEBP_CHUNK_ICCP", WebPChunkId::WEBP_CHUNK_ICCP.0);
    value!("WEBP_CHUNK_ANIM", WebPChunkId::WEBP_CHUNK_ANIM.0);
    value!("WEBP_CHUNK_ANMF", WebPChunkId::WEBP_CHUNK_ANMF.0);
    value!(
        "WEBP_CHUNK_DEPRECATED",
        WebPChunkId::WEBP_CHUNK_DEPRECATED.0
    );
    value!("WEBP_CHUNK_ALPHA", WebPChunkId::WEBP_CHUNK_ALPHA.0);
    value!("WEBP_CHUNK_IMAGE", WebPChunkId::WEBP_CHUNK_IMAGE.0);
    value!("WEBP_CHUNK_EXIF", WebPChunkId::WEBP_CHUNK_EXIF.0);
    value!("WEBP_CHUNK_XMP", WebPChunkId::WEBP_CHUNK_XMP.0);
    value!("WEBP_CHUNK_UNKNOWN", WebPChunkId::WEBP_CHUNK_UNKNOWN.0);
    value!("WEBP_CHUNK_NIL", WebPChunkId::WEBP_CHUNK_NIL.0);
    // The signedness of the enums' types: WebPChunkId is unsigned, WebPMuxError
    // signed (ADR 0003, "Enums").
    value!(
        "(WebPChunkId)-1 > 0",
        i32::from(WebPChunkId(u32::MAX).0 > 0)
    );
    value!("(WebPMuxError)-1 > 0", i32::from(WebPMuxError(-1).0 > 0));
    let output = oracle.probe("wp15", "layout").run::<&str>(&[], b"");
    output.success();
    assert_eq!(String::from_utf8(output.stdout.clone()).unwrap(), expected);
}
