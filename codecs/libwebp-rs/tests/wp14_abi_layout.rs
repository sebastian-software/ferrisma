//! WP14 differential test: size, alignment and field offsets of the `repr(C)`
//! mirrors in `src/abi/demux.rs` (`WebPData`, `WebPIterator`,
//! `WebPChunkIterator`) and of the structs that the C ABI shares with
//! `src/webp/demux.rs` (`WebPAnimDecoderOptions`, `WebPAnimInfo`), and the
//! sizes of the enums it passes, against the C compiler, through the probe
//! `tools/oracle/probes/wp14/layout.c` (ADR 0004).

#[path = "support/oracle.rs"]
mod oracle;

use std::fmt::Write as _;
use std::mem::{align_of, offset_of, size_of};

use oracle::Oracle;
use webp::abi::demux::{WebPChunkIterator, WebPData, WebPIterator};
use webp::webp::decode::WEBP_CSP_MODE;
use webp::webp::demux::{WebPAnimDecoderOptions, WebPAnimInfo, WebPDemuxState, WebPFormatFeature};
use webp::webp::mux_types::{WebPMuxAnimBlend, WebPMuxAnimDispose};

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
    structure!(WebPData, "WebPData");
    field!(WebPData, "WebPData", bytes);
    field!(WebPData, "WebPData", size);
    structure!(WebPIterator, "WebPIterator");
    field!(WebPIterator, "WebPIterator", frame_num);
    field!(WebPIterator, "WebPIterator", num_frames);
    field!(WebPIterator, "WebPIterator", x_offset);
    field!(WebPIterator, "WebPIterator", y_offset);
    field!(WebPIterator, "WebPIterator", width);
    field!(WebPIterator, "WebPIterator", height);
    field!(WebPIterator, "WebPIterator", duration);
    field!(WebPIterator, "WebPIterator", dispose_method);
    field!(WebPIterator, "WebPIterator", complete);
    field!(WebPIterator, "WebPIterator", fragment);
    field!(WebPIterator, "WebPIterator", has_alpha);
    field!(WebPIterator, "WebPIterator", blend_method);
    field!(WebPIterator, "WebPIterator", pad);
    field!(WebPIterator, "WebPIterator", private_);
    structure!(WebPChunkIterator, "WebPChunkIterator");
    field!(WebPChunkIterator, "WebPChunkIterator", chunk_num);
    field!(WebPChunkIterator, "WebPChunkIterator", num_chunks);
    field!(WebPChunkIterator, "WebPChunkIterator", chunk);
    field!(WebPChunkIterator, "WebPChunkIterator", pad);
    field!(WebPChunkIterator, "WebPChunkIterator", private_);
    structure!(WebPAnimDecoderOptions, "WebPAnimDecoderOptions");
    field!(WebPAnimDecoderOptions, "WebPAnimDecoderOptions", color_mode);
    field!(
        WebPAnimDecoderOptions,
        "WebPAnimDecoderOptions",
        use_threads
    );
    field!(WebPAnimDecoderOptions, "WebPAnimDecoderOptions", padding);
    structure!(WebPAnimInfo, "WebPAnimInfo");
    field!(WebPAnimInfo, "WebPAnimInfo", canvas_width);
    field!(WebPAnimInfo, "WebPAnimInfo", canvas_height);
    field!(WebPAnimInfo, "WebPAnimInfo", loop_count);
    field!(WebPAnimInfo, "WebPAnimInfo", bgcolor);
    field!(WebPAnimInfo, "WebPAnimInfo", frame_count);
    field!(WebPAnimInfo, "WebPAnimInfo", pad);
    structure!(WebPDemuxState, "WebPDemuxState");
    structure!(WebPFormatFeature, "WebPFormatFeature");
    structure!(WebPMuxAnimDispose, "WebPMuxAnimDispose");
    structure!(WebPMuxAnimBlend, "WebPMuxAnimBlend");
    structure!(WEBP_CSP_MODE, "WEBP_CSP_MODE");
    let output = oracle.probe("wp14", "layout").run::<&str>(&[], b"");
    output.success();
    assert_eq!(String::from_utf8(output.stdout.clone()).unwrap(), expected);
}
