//! Dump one AVIF file through the safe reader and the engine for an external
//! reference comparison (issue #54, ADR-0031). It prints one JSON line with
//! the wrapper's parsed metadata and the engine outcome, and writes:
//!
//! - `<prefix>.wrapper.raw`: unoriented interleaved RGB(A) from
//!   `ferrisma-avif-read`, 8-bit or native-endian 16-bit by source depth;
//! - `<prefix>.engine.rgba`: the engine's upright straight-alpha sRGB8 RGBA
//!   rows at source size (no resampling);
//! - `<prefix>.icc`: the ICC profile libavif retained, if any.
//!
//! `scripts/avif_reference_crosscheck.py` compares both with libheif and
//! FFmpeg/dav1d. Usage: `avif_reference_crosscheck <input.avif> <prefix>`.

use ferrisma::pipeline::{
    EncodeError, EncodeRequest, EncodedImage, EncoderBackend, ImageFormat, InputProfilePolicy,
    OutputFormat, ResizeFit, ResizeOptions, RowEncoder, TransformOptions, transcode_resized_with,
};
use ferrisma_avif_read::Decoder;
use std::{env, fs};

struct Capture;
struct CaptureRows(Vec<u8>);

impl EncoderBackend for Capture {
    fn format(&self) -> OutputFormat {
        OutputFormat::Avif
    }
    fn name(&self) -> &'static str {
        "capture"
    }
    fn begin(&self, _: &EncodeRequest) -> Result<Box<dyn RowEncoder + '_>, EncodeError> {
        Ok(Box::new(CaptureRows(Vec::new())))
    }
}

impl RowEncoder for CaptureRows {
    fn write_row(&mut self, _: u32, rgba: &[u8]) -> Result<(), EncodeError> {
        self.0.extend_from_slice(rgba);
        Ok(())
    }
    fn finish(self: Box<Self>) -> Result<EncodedImage, EncodeError> {
        Ok(EncodedImage::new(OutputFormat::Avif, self.0, 0, false))
    }
}

fn json_string(text: &str) -> String {
    format!("\"{}\"", text.replace('\\', "\\\\").replace('"', "\\\""))
}

fn wrapper(bytes: &[u8], prefix: &str) -> String {
    let (mut decoder, info) = match Decoder::parse(bytes, 100_000_000) {
        Ok(parsed) => parsed,
        Err(error) => return format!("{{\"error\":{}}}", json_string(&format!("{error:?}"))),
    };
    if let Some(icc) = &info.icc {
        fs::write(format!("{prefix}.icc"), icc).unwrap();
    }
    let channels = if info.alpha { 4 } else { 3 };
    let samples = info.width as usize * info.height as usize * channels;
    let decoded = if info.depth == 8 {
        let mut pixels = vec![0_u8; samples];
        decoder
            .decode_u8(&mut pixels, info.width, info.height, info.alpha)
            .map(|()| pixels)
    } else {
        let mut pixels = vec![0_u16; samples];
        decoder
            .decode_u16(&mut pixels, info.width, info.height, info.alpha)
            .map(|()| pixels.iter().flat_map(|v| v.to_ne_bytes()).collect())
    };
    let decode = match decoded {
        Ok(raw) => {
            fs::write(format!("{prefix}.wrapper.raw"), raw).unwrap();
            "\"ok\"".to_owned()
        }
        Err(error) => json_string(&format!("{error:?}")),
    };
    let s = info.sequence;
    format!(
        "{{\"width\":{},\"height\":{},\"depth\":{},\"alpha\":{},\"alpha_premultiplied\":{},\
         \"container_cicp\":[{},{},{},{}],\"sequence\":{{\"depth\":{},\"monochrome\":{},\
         \"cicp\":[{},{},{},{}]}},\"transform_flags\":{},\"icc_bytes\":{},\"decode\":{decode}}}",
        info.width,
        info.height,
        info.depth,
        info.alpha,
        info.alpha_premultiplied,
        info.primaries,
        info.transfer,
        info.matrix,
        u8::from(info.full_range),
        s.depth,
        s.monochrome,
        s.primaries,
        s.transfer,
        s.matrix,
        u8::from(s.full_range),
        info.transform_flags,
        info.icc.as_ref().map_or(0, Vec::len),
    )
}

fn engine(bytes: &[u8], prefix: &str) -> String {
    let options = TransformOptions {
        transform_threads: 1,
        profile_policy: InputProfilePolicy::AssumeSrgbIfMissing,
        ..TransformOptions::default()
    };
    let upright = ResizeOptions {
        width: Some(u32::MAX),
        height: Some(u32::MAX),
        fit: ResizeFit::Inside,
        without_enlargement: true,
    };
    match transcode_resized_with(bytes, ImageFormat::Avif, options, upright, &Capture) {
        Ok(out) => {
            fs::write(format!("{prefix}.engine.rgba"), &out.bytes).unwrap();
            format!(
                "{{\"width\":{},\"height\":{},\"decoder_full_frame_buffers\":{},\
                 \"peak_managed_bytes\":{},\"opaque_decoder_memory\":{}}}",
                out.width,
                out.height,
                out.report.decoder_full_frame_buffers,
                out.report.peak_managed_bytes,
                out.report.opaque_decoder_memory
            )
        }
        Err(error) => format!(
            "{{\"error\":{},\"kind\":{}}}",
            json_string(&error.to_string()),
            json_string(&format!("{:?}", error.kind()))
        ),
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    assert_eq!(
        args.len(),
        3,
        "usage: avif_reference_crosscheck <input.avif> <prefix>"
    );
    let bytes = fs::read(&args[1]).unwrap();
    println!(
        "{{\"wrapper\":{},\"engine\":{}}}",
        wrapper(&bytes, &args[2]),
        engine(&bytes, &args[2])
    );
}
