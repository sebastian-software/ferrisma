//! WP4: the unfilter form with the previous row as the output row (`prev ==
//! out` in C) is unreachable through the port's API.
//!
//! C allows it (`WebPUnfilterFunc` has no `restrict`), and there upstream's
//! `GradientUnfilter_SSE2` and `GradientUnfilter_C` give different bytes;
//! upstream's own callers never pass it. The owner decided on 2026-11-22 to
//! prove the form unreachable instead of granting an exception to O7 (ADR
//! 0008, amendment log), which lets `tests/wp4_filters.rs` leave it out of the
//! `release-simd` comparison on x86. The proof has three parts:
//!
//! 1. The signature: `webp_unfilters` takes `prev` as a shared and `out` as an
//!    exclusive borrow, so safe Rust cannot pass one row as both. The
//!    `compile_fail` doctest of `webp_unfilters` shows the call rejected.
//! 2. The callers: `src/dec/alpha_dec.rs` and `src/dec/vp8l_dec.rs` are the
//!    only callers in the crate, and each takes `prev` and `out` from the two
//!    halves of one `split_at_mut` of the alpha plane, which the decoder owns.
//!    The source guards below fail when a caller, the dispatch to the SIMD
//!    entry points, or the `unsafe` around them changes, so that the argument
//!    is made again.
//! 3. The run time: debug builds of `webp_unfilters` assert that `prev` and
//!    `out` do not overlap, and the decodes below reach both callers with
//!    every filter and a previous row through the public decoding API.

use std::fs;
use std::path::{Path, PathBuf};

use webp::api::decode::{ColorMode, Config as DecodeConfig, decode};
use webp::api::encode::{
    AlphaFilter, Config as EncodeConfig, Encoder, Picture, PixelLayout, PixelRows, SampleForm,
};
use webp::dsp::dsp::WEBP_FILTER_TYPE;
use webp::dsp::filters::webp_filters;

const WIDTH: usize = 37;
const HEIGHT: usize = 11;

fn manifest_dir() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

/// The source of `path` (relative to the package) without its comment lines
/// and trailing comments, so that documentation does not count as code.
fn code(path: &str) -> String {
    let text = fs::read_to_string(manifest_dir().join(path))
        .unwrap_or_else(|error| panic!("reading {path}: {error}"));
    text.lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .map(|line| line.split(" // ").next().unwrap_or(line))
        .collect::<Vec<_>>()
        .join("\n")
}

/// Every `.rs` file below `dir`, relative to the package, sorted.
fn rust_files(dir: &Path, out: &mut Vec<String>) {
    let mut entries: Vec<_> = fs::read_dir(manifest_dir().join(dir))
        .expect("a source directory")
        .map(|entry| entry.expect("a directory entry").path())
        .collect();
    entries.sort();
    for path in entries {
        let relative = path
            .strip_prefix(manifest_dir())
            .expect("below the package")
            .to_path_buf();
        if path.is_dir() {
            rust_files(&relative, out);
        } else if path.extension().is_some_and(|ext| ext == "rs") {
            out.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

fn source_files() -> Vec<String> {
    let mut files = Vec::new();
    rust_files(Path::new("src"), &mut files);
    files
}

/// The text of the function `name` in `code`: from `fn name(` to the first
/// line that closes an item at the top level.
fn function<'c>(code: &'c str, name: &str) -> &'c str {
    let start = code
        .find(&format!("fn {name}("))
        .unwrap_or_else(|| panic!("the function {name} is gone"));
    let end = code[start..]
        .find("\n}")
        .map_or(code.len(), |offset| start + offset + 2);
    &code[start..end]
}

/// The arguments of the first call `callee(` in `code`, split at the commas
/// outside brackets.
fn call_arguments(code: &str, callee: &str) -> Vec<String> {
    let start = code
        .find(&format!("{callee}("))
        .unwrap_or_else(|| panic!("no call of {callee}"))
        + callee.len()
        + 1;
    let mut depth = 0_u32;
    let mut arguments = vec![String::new()];
    for c in code[start..].chars() {
        match c {
            '(' | '[' | '{' => depth += 1,
            ')' | ']' | '}' if depth == 0 => break,
            ')' | ']' | '}' => depth -= 1,
            ',' if depth == 0 => {
                arguments.push(String::new());
                continue;
            }
            _ => {}
        }
        arguments.last_mut().expect("an argument").push(c);
    }
    arguments
        .into_iter()
        .map(|argument| argument.split_whitespace().collect::<Vec<_>>().join(" "))
        .filter(|argument| !argument.is_empty())
        .collect()
}

/// A caller of `webp_unfilters`: the file, the function, and the names of the
/// halves of its `split_at_mut`, the one before the output row and the one
/// that starts with it.
struct Caller {
    file: &'static str,
    function: &'static str,
    split: &'static str,
    head: &'static str,
    tail: &'static str,
}

const CALLERS: [Caller; 2] = [
    // C: ALPHDecode, WebPUnfilters[filter](prev_line, deltas, dst, width).
    Caller {
        file: "src/dec/alpha_dec.rs",
        function: "alph_decode",
        split: "let (before, after) = plane.split_at_mut(dst);",
        head: "before",
        tail: "after",
    },
    // C: AlphaApplyFilter, WebPUnfilters[filter](prev_line, out, out, stride).
    Caller {
        file: "src/dec/vp8l_dec.rs",
        function: "alpha_apply_filter",
        split: "let (head, tail) = alph.output.split_at_mut(out);",
        head: "head",
        tail: "tail",
    },
];

/// What to do when a guard fails: the argument of this file has to be made
/// again for the changed code before the guard is updated.
const REVIEW: &str = "the proof that `prev == out` is unreachable (tests/wp4_unfilter_aliasing.rs, \
     libwebp-rs ADR 0008, amendment of 2026-11-22) must be re-established for the changed code";

#[test]
fn the_alpha_decoder_is_the_only_caller() {
    let mut calls = Vec::new();
    for file in source_files() {
        if file == "src/dsp/filters.rs" {
            continue;
        }
        let count = code(&file).matches("webp_unfilters(").count();
        if count > 0 {
            calls.push((file, count));
        }
    }
    let expected: Vec<_> = CALLERS
        .iter()
        .map(|caller| (caller.file.to_owned(), 1))
        .collect();
    assert_eq!(
        calls, expected,
        "the callers of webp_unfilters changed: {REVIEW}"
    );
}

#[test]
fn the_unfilters_are_reached_only_through_webp_unfilters() {
    // The entry points that take a previous row and an output row; outside the
    // filter modules, only `webp_unfilters` may name them.
    let names = [
        "horizontal_unfilter",
        "vertical_unfilter",
        "gradient_unfilter",
        "WEBP_UNFILTERS",
        "WebPUnfilterFunc",
        "filters_neon::unfilters",
    ];
    let own = [
        "src/dsp/filters.rs",
        "src/dsp/filters_sse2.rs",
        "src/dsp/filters_neon.rs",
    ];
    for file in source_files() {
        if own.contains(&file.as_str()) {
            continue;
        }
        let code = code(&file);
        for name in names {
            assert!(
                !code.contains(name),
                "{file} names {name} outside the filter modules: {REVIEW}"
            );
        }
    }
}

#[test]
fn each_caller_takes_the_rows_from_the_two_halves_of_one_split() {
    for caller in &CALLERS {
        let code = code(caller.file);
        let body = function(&code, caller.function);
        assert!(
            body.contains(caller.split),
            "{}::{} no longer splits the alpha plane with `{}`: {REVIEW}",
            caller.file,
            caller.function,
            caller.split
        );
        let arguments = call_arguments(body, "webp_unfilters");
        assert_eq!(arguments.len(), 5, "{}: {arguments:?}", caller.file);
        let (mut prev, out) = (arguments[1].clone(), &arguments[3]);
        if prev.chars().all(|c| c == '_' || c.is_ascii_alphanumeric()) {
            // A local binding: its initializer is the previous row.
            let binding = format!("let {prev} = ");
            let start = body
                .find(&binding)
                .unwrap_or_else(|| panic!("{}: no binding of {prev}: {REVIEW}", caller.file));
            let end = body[start..].find(';').expect("a statement") + start;
            prev = body[start + binding.len()..end].to_owned();
        }
        assert!(
            prev.contains(&format!("&{}[", caller.head)) && !prev.contains(caller.tail),
            "{}: the previous row `{prev}` is not taken from `{}` alone: {REVIEW}",
            caller.file,
            caller.head
        );
        assert!(
            out.starts_with(&format!("&mut {}[", caller.tail)) && !out.contains(caller.head),
            "{}: the output row `{out}` is not taken from `{}` alone: {REVIEW}",
            caller.file,
            caller.tail
        );
    }
}

#[test]
fn the_unfilter_path_has_no_unchecked_aliasing() {
    // The crate denies `unsafe` outside the modules that allow it, and the
    // decoder and the portable filters are not among them.
    assert!(
        code("src/lib.rs").contains("#![deny(unsafe_code)]"),
        "{REVIEW}"
    );
    for file in source_files() {
        let code = code(&file);
        if file.starts_with("src/dec/") || file == "src/dsp/filters.rs" {
            assert!(!code.contains("unsafe"), "{file} contains unsafe: {REVIEW}");
        }
    }
    let modules = code("src/dsp/mod.rs");
    let filters = modules
        .find("pub mod filters;")
        .expect("the filters module");
    let attributes = &modules[modules[..filters].rfind(';').map_or(0, |end| end + 1)..filters];
    assert!(
        !attributes.contains("unsafe_code"),
        "src/dsp/filters.rs may contain unsafe: {REVIEW}"
    );

    // No pointer from which a second view of a row could be made, in the
    // modules of the path and the vector load and store helpers they use.
    let path = [
        "src/dsp/filters.rs",
        "src/dsp/filters_sse2.rs",
        "src/dsp/filters_neon.rs",
        "src/dsp/common_sse2.rs",
        "src/dsp/neon.rs",
        "src/dec/alpha_dec.rs",
        "src/dec/vp8l_dec.rs",
    ];
    for file in path {
        let code = code(file);
        for construct in ["from_raw_parts", "transmute", "ptr::", "*mut ", "*const "] {
            assert!(
                !code.contains(construct),
                "{file} contains `{construct}`: {REVIEW}"
            );
        }
    }

    // The `unsafe` of the SIMD filter modules only calls the
    // `#[target_feature]` function with the entry point's own slices.
    for (file, suffix) in [
        ("src/dsp/filters_sse2.rs", "_sse2"),
        ("src/dsp/filters_neon.rs", "_neon"),
    ] {
        let code = code(file);
        assert!(
            !code.contains("as_ptr") && !code.contains("as_mut_ptr"),
            "{file} takes a raw pointer: {REVIEW}"
        );
        for line in code.lines().filter(|line| line.contains("unsafe")) {
            let line = line.trim();
            let call = line
                .strip_prefix("unsafe { ")
                .and_then(|rest| rest.strip_suffix(" }"))
                .unwrap_or_else(|| panic!("{file}: unexpected unsafe `{line}`: {REVIEW}"));
            let (callee, arguments) = call.split_once('(').expect("a call");
            assert!(
                callee.ends_with(suffix)
                    && callee
                        .chars()
                        .all(|c| c == '_' || c.is_ascii_lowercase() || c.is_ascii_digit())
                    && [
                        "prev, input, out, width)",
                        "data, width, height, stride, filtered_data)",
                    ]
                    .contains(&arguments),
                "{file}: unexpected unsafe `{line}`: {REVIEW}"
            );
        }
    }
}

/// An alpha plane `f(x) + g(y)` with seeded `f` and `g`, which the gradient
/// predictor reconstructs exactly and the fast filter estimate picks.
fn alpha_plane() -> Vec<u8> {
    let mut state = 0x5eed_0004_u32;
    let mut next = || {
        state = state.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        (state >> 24) as u8 % 61
    };
    let f: Vec<u8> = (0..WIDTH).map(|_| next()).collect();
    let g: Vec<u8> = (0..HEIGHT).map(|_| next()).collect();
    (0..HEIGHT)
        .flat_map(|y| f.iter().map(|&fx| fx + g[y]).collect::<Vec<_>>())
        .collect()
}

/// A lossy WebP of a colored picture with `alpha`, encoded with `config`.
fn encode_with_alpha(alpha: &[u8], config: &EncodeConfig) -> Vec<u8> {
    let rgba: Vec<u8> = alpha
        .iter()
        .enumerate()
        .flat_map(|(i, &a)| [(i * 7) as u8, (i * 3) as u8, 90, a])
        .collect();
    let rows = PixelRows {
        layout: PixelLayout::Rgba,
        data: &rgba,
        width: WIDTH as u32,
        height: HEIGHT as u32,
        stride: WIDTH * 4,
    };
    let mut picture = Picture::import(rows, SampleForm::Yuva).expect("a picture");
    Encoder::new(config)
        .encode(&mut picture)
        .expect("an encoding")
}

/// The position and length of the payload of the `ALPH` chunk of `webp`.
fn alph_chunk(webp: &[u8]) -> (usize, usize) {
    let mut pos = 12;
    while pos + 8 <= webp.len() {
        let size = u32::from_le_bytes(webp[pos + 4..pos + 8].try_into().expect("4 bytes")) as usize;
        if &webp[pos..pos + 4] == b"ALPH" {
            return (pos + 8, size);
        }
        pos += 8 + size + (size & 1);
    }
    panic!("the file has no ALPH chunk")
}

/// The alpha channel of `webp`, decoded through the public API.
fn decoded_alpha(webp: &[u8]) -> Vec<u8> {
    let image = decode(webp, &DecodeConfig::new(ColorMode::Rgba)).expect("a decoding");
    let rows = image.rows().expect("packed rows");
    rows.iter()
        .flat_map(|row| {
            row[..WIDTH * 4]
                .chunks(4)
                .map(|pixel| pixel[3])
                .collect::<Vec<_>>()
        })
        .collect()
}

#[test]
fn uncompressed_alpha_unfilters_with_distinct_rows() {
    // ALPHDecode: an uncompressed alpha plane, filtered here with each filter
    // and written into the ALPH chunk of an encoded file.
    let alpha = alpha_plane();
    let config = EncodeConfig::new()
        .alpha_compression(false)
        .alpha_filter(AlphaFilter::None);
    let webp = encode_with_alpha(&alpha, &config);
    let (payload, size) = alph_chunk(&webp);
    assert_eq!(size, 1 + WIDTH * HEIGHT, "an uncompressed alpha plane");
    assert_eq!(
        webp[payload], 0,
        "no compression, no filter, no preprocessing"
    );
    for filter in 1..=3_u8 {
        let mut file = webp.clone();
        file[payload] = filter << 2;
        webp_filters(
            WEBP_FILTER_TYPE(u32::from(filter)),
            &alpha,
            WIDTH as i32,
            HEIGHT as i32,
            WIDTH as i32,
            &mut file[payload + 1..payload + size],
        );
        assert_eq!(decoded_alpha(&file), alpha, "filter {filter}");
    }
}

#[test]
fn lossless_alpha_unfilters_with_distinct_rows() {
    // AlphaApplyFilter: a losslessly compressed alpha plane that the encoder's
    // fast estimate filters with the gradient filter (method 3 tries no
    // other filter for 17 to 192 levels).
    let alpha = alpha_plane();
    let config = EncodeConfig::new()
        .method(3)
        .alpha_compression(true)
        .alpha_filter(AlphaFilter::Fast);
    let webp = encode_with_alpha(&alpha, &config);
    let (payload, _) = alph_chunk(&webp);
    let header = webp[payload];
    assert_eq!(header & 3, 1, "lossless compression");
    assert_eq!((header >> 2) & 3, 3, "the gradient filter");
    assert_eq!(decoded_alpha(&webp), alpha);
}
