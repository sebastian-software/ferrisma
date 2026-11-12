//! Conformance front end for the safe wrapper, for the official
//! `libjxl/conformance` runner in `--lax` (application) mode:
//!
//! ```text
//! jxl_conformance_wrapper input.jxl out.npy --icc_out f --metadata_out f [ignored flags]
//! ```
//!
//! It emits exactly what the engine receives: the upright F32 frame through
//! `decode_region_into` (the engine's image-out callback path) with the full
//! region, and the DATA ICC from `inspect`. The runner transforms these
//! samples from that profile to the reference profile, so a profile that
//! misdescribes the samples fails the tolerance check. Inputs the wrapper
//! rejects by design (animation, non-alpha extra channels, float samples)
//! exit with an error. The ORIGINAL profile and JPEG reconstruction are not
//! part of the wrapper; the codec-level run in `codecs/libjxl-rs` checks them.
use ferrisma_jxl_decoder::{Region, decode_region_into, inspect};
use std::process::ExitCode;

const MAX_PIXELS: u64 = 100_000_000;

fn main() -> ExitCode {
    let mut positional = Vec::new();
    let mut icc_out = None;
    let mut metadata_out = None;
    let mut args = std::env::args().skip(1);
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--icc_out" => icc_out = args.next(),
            "--metadata_out" => metadata_out = args.next(),
            "--orig_icc_out" | "--preview_out" => {
                args.next();
            }
            "--norender_spotcolors" => {}
            _ => positional.push(arg),
        }
    }
    let [input, output] = positional.as_slice() else {
        eprintln!("usage: jxl_conformance_wrapper input.jxl out.npy [options]");
        return ExitCode::FAILURE;
    };
    if !output.ends_with(".npy") {
        eprintln!("the wrapper emits pixels only");
        return ExitCode::FAILURE;
    }
    let bytes = std::fs::read(input).expect("read input");
    let info = match inspect(&bytes, MAX_PIXELS) {
        Ok(info) => info,
        Err(error) => {
            eprintln!("inspect: {error:?}");
            return ExitCode::FAILURE;
        }
    };
    let mut samples = vec![0f32; info.sample_count().expect("sample count")];
    if let Err(error) =
        decode_region_into(&bytes, &info, Region::full(&info), &mut samples, MAX_PIXELS)
    {
        eprintln!("decode_region_into: {error:?}");
        return ExitCode::FAILURE;
    }
    let header = format!(
        "{{'descr': '<f4', 'fortran_order': False, 'shape': (1, {}, {}, {}), }}\n",
        info.height, info.width, info.channels
    );
    let mut npy = b"\x93NUMPY\x01\x00".to_vec();
    npy.extend_from_slice(&u16::try_from(header.len()).unwrap().to_le_bytes());
    npy.extend_from_slice(header.as_bytes());
    for sample in &samples {
        npy.extend_from_slice(&sample.to_le_bytes());
    }
    std::fs::write(output, npy).expect("write npy");
    if let Some(path) = icc_out {
        std::fs::write(path, &info.rendered_icc).expect("write icc");
    }
    if let Some(path) = metadata_out {
        std::fs::write(path, "{\"frames\": [{\"name\": \"\"}]}\n").expect("write metadata");
    }
    ExitCode::SUCCESS
}
