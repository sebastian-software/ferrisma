//! WP11: the ISO/IEC 10918-2 (ITU-T T.83) compliance data for the processes
//! in scope (ADR 0001, ADR 0002): baseline, extended 8-bit Huffman and
//! arithmetic, and progressive Huffman and arithmetic coding.
//!
//! The nine image streams A1, C1, C2, D1, D2, G1, H1, K1 and L1 code the same
//! 255x257 image with four components of different sampling factors. B1
//! holds tables only, and B2 is an abbreviated image with 255 components,
//! more than libjpeg's `MAX_COMPONENTS`. Two files per component describe the
//! coefficients (ISO/IEC 10918-2:1995):
//!
//! - `EREF_X8.DCT`, the encoder reference test data (3.1.35), the quantized
//!   coefficients of the source image, which every stream codes exactly;
//! - `DREF_X8.DCT`, the decoder reference test data (3.1.34): the reference
//!   FDCT and quantizer (3.1.41, 3.1.43) applied to the output of the
//!   reference decoder (3.1.39: entropy decoding, dequantization and the
//!   reference IDCT of 3.1.42, all `f64` after T.81 A.3.3 and A.3.4). DREF
//!   differs from EREF by one in 24 coefficients of component A and 52 of B.
//!
//! `tools/fetch_t83.py` fetches the data, which is not redistributed;
//! `support/t83.rs` locates and reads it and holds the reference DCTs. The
//! tolerance of T.83 clause 7 was not available, so the tests record the
//! measured values instead of judging them:
//!
//! 1. `decoded_coefficients_match_the_encoder_reference`: every image stream,
//!    decoded as raw data with the islow IDCT, equals `jidctint.rs`'s
//!    `jpeg_idct_islow` applied to the EREF blocks with the stream's
//!    quantization tables, bit for bit, over every block the component
//!    holds, the padded ones included. The tables, and the islow
//!    multipliers that dequantize with them, equal an independent parse of
//!    the stream's DQT segments. So the entropy decoders and
//!    dequantization reproduce the coded coefficients exactly.
//! 2. `decoder_reference_test_is_recorded`: T.83's decoder test. The
//!    reference FDCT and quantizer turn the port's raw output for the islow,
//!    ifast and float IDCTs back into coefficients, which are compared with
//!    DREF; `reference_path_reproduces_the_decoder_reference` checks that the
//!    test's own reference decoder turns EREF into DREF exactly.
//! 3. `accuracy_against_a_true_idct_is_recorded`: the raw output of the three
//!    IDCTs against the reference IDCT of the dequantized EREF coefficients.
//! 4. `streams_match_the_oracle` and `abbreviated_streams_match_the_oracle`:
//!    every stream through the WP6 API probe (raw data with each IDCT, the
//!    default output and, for the progressive streams, buffered-image mode)
//!    and B1 and B2 through the WP2 header probe as well, compared exactly
//!    with the port.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/t83.rs"]
mod t83;
#[path = "support/wp2.rs"]
mod wp2;
#[path = "support/wp4.rs"]
mod wp4;
#[path = "support/wp6.rs"]
mod wp6;

use jpeg::jdapimin::{
    jpeg_create_decompress, jpeg_destroy_decompress, jpeg_finish_decompress, jpeg_read_header,
};
use jpeg::jdapistd::{jpeg_read_raw_data, jpeg_start_decompress};
use jpeg::jdatasrc::jpeg_mem_src;
use jpeg::jerror::{ErrorExit, J_MESSAGE_CODE, jpeg_std_error};
use jpeg::jidctint::jpeg_idct_islow;
use jpeg::jmorecfg::JDIMENSION;
use jpeg::jpeglib::{
    J_DCT_METHOD, JBLOCK, JPEG_HEADER_OK, JPEG_HEADER_TABLES_ONLY, RowListMut, jpeg_component_info,
    jpeg_decompress_struct, jpeg_error_mgr,
};
use oracle::{Fields, Oracle};
use t83::{COMPONENTS, IMAGE_STREAMS, PROGRESSIVE_STREAMS, ReferenceDct, T83};

/// The IDCT methods of checks 2 and 3.
const METHODS: [&str; 3] = ["islow", "ifast", "float"];

/// One component of a raw-data decode. `rows` are `width_in_blocks * 8`
/// samples long, and there are at least `height_in_blocks * 8`; `width` and
/// `height` are the component's downsampled size, the image samples.
struct Plane {
    width: usize,
    height: usize,
    width_in_blocks: usize,
    height_in_blocks: usize,
    rows: Vec<Vec<u8>>,
}

impl Plane {
    /// The samples of the block in block row `row` and block column `col`,
    /// in natural order.
    fn block(&self, row: usize, col: usize) -> [u8; 64] {
        let mut samples = [0; 64];
        for y in 0..8 {
            samples[y * 8..y * 8 + 8]
                .copy_from_slice(&self.rows[row * 8 + y][col * 8..col * 8 + 8]);
        }
        samples
    }
}

/// Unwraps a result of the port, or fails with the object's error.
fn ok<T>(result: Result<T, ErrorExit>, cinfo: &jpeg_decompress_struct<'_>, what: &str) -> T {
    result.unwrap_or_else(|_| panic!("{what} failed with message code {}", cinfo.err.msg_code))
}

/// Decodes `data` as raw data (no upsampling, no color conversion) with
/// `method`, and calls `inspect` with the object and the planes before
/// `jpeg_finish_decompress`. Requires a decode without warnings.
fn decode_raw<R>(
    data: &[u8],
    method: J_DCT_METHOD,
    inspect: impl FnOnce(&jpeg_decompress_struct<'_>, Vec<Plane>) -> R,
) -> R {
    let mut err = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut err);
    let mut cinfo = jpeg_decompress_struct::new(err);
    ok(jpeg_create_decompress(&mut cinfo), &cinfo, "create");
    ok(jpeg_mem_src(&mut cinfo, data), &cinfo, "jpeg_mem_src");
    let status = ok(jpeg_read_header(&mut cinfo, true), &cinfo, "read_header");
    assert_eq!(status, JPEG_HEADER_OK);
    cinfo.raw_data_out = true;
    cinfo.dct_method = method;
    assert!(ok(jpeg_start_decompress(&mut cinfo), &cinfo, "start"));
    assert_eq!((cinfo.output_width, cinfo.output_height), (255, 257));
    assert_eq!(cinfo.min_DCT_scaled_size, 8);

    let lines = JDIMENSION::try_from(cinfo.max_v_samp_factor * cinfo.min_DCT_scaled_size).unwrap();
    let shapes: Vec<(usize, usize)> = cinfo
        .comp_info()
        .iter()
        .map(|compptr| {
            (
                usize::try_from(compptr.v_samp_factor * compptr.DCT_scaled_size).unwrap(),
                compptr.width_in_blocks as usize * 8,
            )
        })
        .collect();
    let mut planes: Vec<Plane> = cinfo
        .comp_info()
        .iter()
        .map(|compptr| Plane {
            width: compptr.downsampled_width as usize,
            height: compptr.downsampled_height as usize,
            width_in_blocks: compptr.width_in_blocks as usize,
            height_in_blocks: compptr.height_in_blocks as usize,
            rows: Vec::new(),
        })
        .collect();
    while cinfo.output_scanline < cinfo.output_height {
        let mut buffers: Vec<Vec<Vec<u8>>> = shapes
            .iter()
            .map(|&(rows, width)| vec![vec![0; width]; rows])
            .collect();
        let n = {
            let mut lists: Vec<Vec<&mut [u8]>> = buffers
                .iter_mut()
                .map(|buffer| buffer.iter_mut().map(Vec::as_mut_slice).collect())
                .collect();
            let mut image: Vec<RowListMut<'_, '_, u8>> = lists
                .iter_mut()
                .map(|list| RowListMut::Separate(list.as_mut_slice()))
                .collect();
            ok(
                jpeg_read_raw_data(&mut cinfo, &mut image, lines),
                &cinfo,
                "read_raw_data",
            )
        };
        assert_eq!(n, lines, "a whole iMCU row per call");
        for (plane, buffer) in planes.iter_mut().zip(buffers) {
            plane.rows.extend(buffer);
        }
    }
    for plane in &planes {
        assert!(plane.rows.len() >= plane.height_in_blocks * 8);
    }
    let result = inspect(&cinfo, planes);
    assert!(ok(jpeg_finish_decompress(&mut cinfo), &cinfo, "finish"));
    assert_eq!(cinfo.err.num_warnings, 0, "warnings");
    ok(jpeg_destroy_decompress(&mut cinfo), &cinfo, "destroy");
    result
}

/// Checks a component's layout against the reference data.
fn check_component(compptr: &jpeg_component_info, ci: usize, blocks: usize, label: &str) {
    let (letter, id, h, v) = COMPONENTS[ci];
    assert_eq!(
        (
            compptr.component_id,
            compptr.h_samp_factor,
            compptr.v_samp_factor
        ),
        (id, h, v),
        "{label}: component {letter}"
    );
    assert_eq!(
        compptr.width_in_blocks as usize * compptr.height_in_blocks as usize,
        blocks,
        "{label}: the reference data holds every block of component {letter}"
    );
}

/// `jpeg_idct_islow` of the port applied to every reference block of a
/// component, with the component's multiplier table and the object's
/// range-limit table: the rows of `width_in_blocks * 8` samples.
fn islow_of_the_reference(
    cinfo: &jpeg_decompress_struct<'_>,
    compptr: &jpeg_component_info,
    blocks: &[[i16; 64]],
) -> Vec<Vec<u8>> {
    let width_in_blocks = compptr.width_in_blocks as usize;
    let mut rows = vec![vec![0; width_in_blocks * 8]; compptr.height_in_blocks as usize * 8];
    for (index, block) in blocks.iter().enumerate() {
        let (block_row, block_col) = (index / width_in_blocks, index % width_in_blocks);
        let coef_block: JBLOCK = *block;
        let mut list: Vec<&mut [u8]> = rows[block_row * 8..block_row * 8 + 8]
            .iter_mut()
            .map(Vec::as_mut_slice)
            .collect();
        jpeg_idct_islow(
            cinfo,
            compptr,
            &coef_block,
            RowListMut::Separate(&mut list),
            JDIMENSION::try_from(block_col * 8).unwrap(),
        );
    }
    rows
}

/// Check 1: the decoded samples equal the islow IDCT of the EREF
/// coefficients over every block, so the coefficients the decoder
/// dequantized are the coded ones.
#[test]
fn decoded_coefficients_match_the_encoder_reference() {
    let Some(t83) = T83::locate("decoded_coefficients_match_the_encoder_reference") else {
        return;
    };
    let references: Vec<Vec<[i16; 64]>> = COMPONENTS
        .iter()
        .map(|&(letter, ..)| t83.encoder_reference(letter))
        .collect();
    for (name, process, coder) in IMAGE_STREAMS {
        let data = t83.stream(name);
        let tables = t83::quantization_tables(&data);
        let blocks = decode_raw(&data, J_DCT_METHOD::JDCT_ISLOW, |cinfo, planes| {
            assert_eq!(cinfo.num_components, 4, "{name}");
            for (n, table) in tables.iter().enumerate() {
                let stored = cinfo.quant_tbl_ptrs[n].as_deref().map(|qtbl| qtbl.quantval);
                assert_eq!(stored, *table, "{name}: quantization table {n}");
            }
            let mut blocks = 0;
            for (ci, plane) in planes.iter().enumerate() {
                let compptr = &cinfo.comp_info()[ci];
                let letter = COMPONENTS[ci].0;
                check_component(compptr, ci, references[ci].len(), name);
                let table = tables[usize::try_from(compptr.quant_tbl_no).unwrap()];
                assert_eq!(
                    compptr.quant_table.as_deref().map(|qtbl| qtbl.quantval),
                    table,
                    "{name}: the quantization table latched for component {letter}"
                );
                // jddctmgr.c: the islow multipliers are the quantizers, so
                // the reference blocks below are dequantized with the
                // stream's tables.
                let multipliers: Vec<i64> = compptr
                    .dct_table
                    .as_deref()
                    .expect("a multiplier table")
                    .islow_array()
                    .iter()
                    .map(|&multiplier| i64::from(multiplier))
                    .collect();
                let quantizers: Vec<i64> = table.unwrap().iter().map(|&q| i64::from(q)).collect();
                assert_eq!(
                    multipliers, quantizers,
                    "{name}: the islow multipliers of component {letter}"
                );
                let expected = islow_of_the_reference(cinfo, compptr, &references[ci]);
                for (y, expected) in expected.iter().enumerate() {
                    let actual = &plane.rows[y];
                    if let Some(x) = actual.iter().zip(expected).position(|(a, e)| a != e) {
                        panic!(
                            "{name} ({process}, {coder}): component {letter} differs from the \
                             islow IDCT of EREF_{letter}8.DCT at x={x}, y={y}: {} instead of {}",
                            actual[x], expected[x]
                        );
                    }
                }
                blocks += references[ci].len();
            }
            blocks
        });
        eprintln!("{name} ({process}, {coder}): {blocks} blocks equal");
    }
}

/// A raw-data decode of one stream with one IDCT method.
struct Decode {
    stream: &'static str,
    method: &'static str,
    /// Whether a SIMD kernel ran the IDCT.
    simd: bool,
    /// The quantization table of each component.
    tables: Vec<[u16; 64]>,
    planes: Vec<Plane>,
}

/// Whether the decode runs a SIMD kernel for the 8x8 IDCT.
#[cfg(with_simd)]
fn simd_idct(cinfo: &jpeg_decompress_struct<'_>) -> bool {
    cinfo.idct().idct_simd.is_some()
}

/// Without `WITH_SIMD`, every IDCT is the C routine.
#[cfg(not(with_simd))]
fn simd_idct(_cinfo: &jpeg_decompress_struct<'_>) -> bool {
    false
}

/// Every image stream decoded with every method of `METHODS`.
fn decode_streams(t83: &T83) -> Vec<Decode> {
    let mut decodes = Vec::new();
    for (stream, ..) in IMAGE_STREAMS {
        let data = t83.stream(stream);
        let tables = t83::quantization_tables(&data);
        for method in METHODS {
            decodes.push(decode_raw(
                &data,
                wp4::parse_method(method),
                |cinfo, planes| {
                    let tables = cinfo
                        .comp_info()
                        .iter()
                        .map(|compptr| {
                            tables[usize::try_from(compptr.quant_tbl_no).unwrap()]
                                .expect("a quantization table")
                        })
                        .collect();
                    Decode {
                        stream,
                        method,
                        simd: simd_idct(cinfo),
                        tables,
                        planes,
                    }
                },
            ));
        }
    }
    decodes
}

/// A recorded value: the method, the component, the maximum absolute
/// difference, the number of differing values and, for check 2, the number
/// of quantizer ties (0 in check 3, which does not quantize).
type Record = (&'static str, char, u32, usize, usize);

/// Measured values of check 2 or 3, one per decode and component.
struct Measured {
    stream: &'static str,
    method: &'static str,
    simd: bool,
    letter: char,
    max: u32,
    count: usize,
    ties: usize,
    total: usize,
}

/// Prints `measured` and asserts it against `records`, or against
/// `float_sse2` for the float IDCT's SSE2 kernel.
fn assert_records(measured: &[Measured], records: &[Record], float_sse2: &[Record], unit: &str) {
    eprintln!("stream method simd component max_diff differing_{unit} ties {unit}");
    for m in measured {
        eprintln!(
            "{} {} {} {} {} {} {} {}",
            m.stream, m.method, m.simd, m.letter, m.max, m.count, m.ties, m.total
        );
    }
    for m in measured {
        let sse2 = m.method == "float" && m.simd;
        let table = if sse2 { float_sse2 } else { records };
        let &(_, _, max, count, ties) = table
            .iter()
            .find(|entry| entry.0 == m.method && entry.1 == m.letter)
            .unwrap();
        assert_eq!(
            (m.max, m.count, m.ties),
            (max, count, ties),
            "{} {}{}: component {}, (max |diff|, differing {unit}, ties)",
            m.stream,
            m.method,
            if sse2 { " (SSE2)" } else { "" },
            m.letter
        );
    }
}

/// The test's own reference decoder turns EREF into DREF with the stream's
/// tables: reference IDCT, then reference FDCT and quantizer (ISO/IEC
/// 10918-2 3.1.34), exactly, where a tie matches if DREF holds either of its
/// two nearest integers. This checks `ReferenceDct`; every stream has the
/// same tables.
#[test]
fn reference_path_reproduces_the_decoder_reference() {
    let Some(t83) = T83::locate("reference_path_reproduces_the_decoder_reference") else {
        return;
    };
    let tables = t83::quantization_tables(&t83.stream("A1"));
    for (name, ..) in IMAGE_STREAMS {
        assert_eq!(
            t83::quantization_tables(&t83.stream(name)),
            tables,
            "{name}"
        );
    }
    let dct = ReferenceDct::new();
    // A1's SOF: component A uses table 0, B 1, C 2 and D 3.
    for (ci, &(letter, ..)) in COMPONENTS.iter().enumerate() {
        let table = tables[ci].unwrap();
        let (encoder, decoder) = (t83.encoder_reference(letter), t83.decoder_reference(letter));
        assert_eq!(encoder.len(), decoder.len());
        let (mut differ, mut ties) = (0, 0);
        for (index, (eref, dref)) in encoder.iter().zip(&decoder).enumerate() {
            let samples = dct.idct(&ReferenceDct::dequantize(eref, &table));
            let quantized = ReferenceDct::quantize(&dct.fdct(&samples), &table);
            for (k, (q, d)) in quantized.iter().zip(dref).enumerate() {
                assert_eq!(
                    q.distance(*d),
                    0,
                    "component {letter}, block {index}, coefficient {k}: {q:?} instead of {d}"
                );
                ties += usize::from(q.is_tie());
            }
            differ += eref.iter().zip(dref).filter(|(e, d)| e != d).count();
        }
        eprintln!(
            "component {letter}: {} blocks reproduced, {ties} ties; DREF differs from EREF in \
             {differ} coefficients",
            decoder.len()
        );
    }
}

/// Check 2, T.83's decoder test: the reference FDCT and quantizer applied to
/// every block of the port's raw output, compared with DREF, per stream,
/// component and IDCT method: `(method, component, max |diff|, differing
/// coefficients, ties)`. A tie differs only if DREF holds neither of its
/// two nearest integers; the ties are counted where the rule applied. Every
/// stream gives the same values (check 1 shows that
/// they all decode to the EREF coefficients, and their tables are equal).
///
/// DREF holds `width_in_blocks * height_in_blocks` blocks per component, the
/// blocks that complete its grid of 8x8 blocks, and the raw output covers
/// every one of them, so all are compared. The dummy blocks that complete
/// the interleaved MCUs beyond them (component B's 18th block row, C's 33rd
/// block column, D's block rows 34 to 36) are in neither.
///
/// The float IDCT's values depend on the implementation: upstream's C
/// routine (`jidctflt.c`, every configuration on AArch64 and the portable
/// one everywhere) and the SSE2 kernel on x86-64 (`jidctflt-sse2.asm`)
/// round differently, as upstream's `FLOATTEST8` hashes show.
const DECODER_TEST: [Record; 12] = [
    ("islow", 'A', 1, 4, 0),
    ("islow", 'B', 1, 3, 1),
    ("islow", 'C', 0, 0, 0),
    ("islow", 'D', 0, 0, 0),
    ("ifast", 'A', 1, 53, 10),
    ("ifast", 'B', 1, 66, 4),
    ("ifast", 'C', 0, 0, 0),
    ("ifast", 'D', 1, 5, 0),
    ("float", 'A', 0, 0, 0),
    ("float", 'B', 0, 0, 1),
    ("float", 'C', 0, 0, 0),
    ("float", 'D', 0, 0, 0),
];

/// The float IDCT's values of check 2 with the SSE2 kernel.
const DECODER_TEST_FLOAT_SSE2: [Record; 4] = [
    ("float", 'A', 0, 0, 0),
    ("float", 'B', 0, 0, 1),
    ("float", 'C', 0, 0, 0),
    ("float", 'D', 0, 0, 0),
];

/// Check 2: T.83's decoder test on the port's raw output.
#[test]
fn decoder_reference_test_is_recorded() {
    let Some(t83) = T83::locate("decoder_reference_test_is_recorded") else {
        return;
    };
    let references: Vec<Vec<[i16; 64]>> = COMPONENTS
        .iter()
        .map(|&(letter, ..)| t83.decoder_reference(letter))
        .collect();
    let dct = ReferenceDct::new();
    let mut measured = Vec::new();
    for decode in decode_streams(&t83) {
        for (ci, plane) in decode.planes.iter().enumerate() {
            let reference = &references[ci];
            assert_eq!(
                reference.len(),
                plane.width_in_blocks * plane.height_in_blocks
            );
            let (mut max, mut count, mut ties) = (0, 0, 0);
            for (index, dref) in reference.iter().enumerate() {
                let samples =
                    plane.block(index / plane.width_in_blocks, index % plane.width_in_blocks);
                let quantized = ReferenceDct::quantize(&dct.fdct(&samples), &decode.tables[ci]);
                for (q, d) in quantized.iter().zip(dref) {
                    let difference = q.distance(*d);
                    max = max.max(u32::from(difference));
                    count += usize::from(difference != 0);
                    ties += usize::from(q.is_tie());
                }
            }
            measured.push(Measured {
                stream: decode.stream,
                method: decode.method,
                simd: decode.simd,
                letter: COMPONENTS[ci].0,
                max,
                count,
                ties,
                total: reference.len() * 64,
            });
        }
    }
    assert_records(
        &measured,
        &DECODER_TEST,
        &DECODER_TEST_FLOAT_SSE2,
        "coefficients",
    );
}

/// Check 3: the port's raw output against the reference IDCT of the
/// dequantized EREF coefficients, over the image samples of each component
/// (the downsampled size), as `DECODER_TEST` records it: `(method,
/// component, max |diff|, differing samples)`.
const ACCURACY: [Record; 12] = [
    ("islow", 'A', 1, 105, 0),
    ("islow", 'B', 1, 230, 0),
    ("islow", 'C', 1, 366, 0),
    ("islow", 'D', 1, 440, 0),
    ("ifast", 'A', 2, 2701, 0),
    ("ifast", 'B', 2, 5223, 0),
    ("ifast", 'C', 3, 8526, 0),
    ("ifast", 'D', 2, 10529, 0),
    ("float", 'A', 0, 0, 0),
    ("float", 'B', 0, 0, 0),
    ("float", 'C', 1, 2, 0),
    ("float", 'D', 1, 1, 0),
];

/// The float IDCT's values of check 3 with the SSE2 kernel.
const ACCURACY_FLOAT_SSE2: [Record; 4] = [
    ("float", 'A', 0, 0, 0),
    ("float", 'B', 0, 0, 0),
    ("float", 'C', 0, 0, 0),
    ("float", 'D', 1, 1, 0),
];

/// Check 3: the port's raw output against a true IDCT.
#[test]
fn accuracy_against_a_true_idct_is_recorded() {
    let Some(t83) = T83::locate("accuracy_against_a_true_idct_is_recorded") else {
        return;
    };
    let references: Vec<Vec<[i16; 64]>> = COMPONENTS
        .iter()
        .map(|&(letter, ..)| t83.encoder_reference(letter))
        .collect();
    let dct = ReferenceDct::new();
    let mut measured = Vec::new();
    for decode in decode_streams(&t83) {
        for (ci, plane) in decode.planes.iter().enumerate() {
            let reference = &references[ci];
            assert_eq!(
                reference.len(),
                plane.width_in_blocks * plane.height_in_blocks
            );
            let (mut max, mut count) = (0, 0);
            for (index, eref) in reference.iter().enumerate() {
                let (row, col) = (index / plane.width_in_blocks, index % plane.width_in_blocks);
                let expected = dct.idct(&ReferenceDct::dequantize(eref, &decode.tables[ci]));
                let actual = plane.block(row, col);
                for k in 0..64 {
                    let (y, x) = (row * 8 + k / 8, col * 8 + k % 8);
                    if y < plane.height && x < plane.width {
                        let difference = actual[k].abs_diff(expected[k]);
                        max = max.max(u32::from(difference));
                        count += usize::from(difference != 0);
                    }
                }
            }
            measured.push(Measured {
                stream: decode.stream,
                method: decode.method,
                simd: decode.simd,
                letter: COMPONENTS[ci].0,
                max,
                count,
                ties: 0,
                total: plane.width * plane.height,
            });
        }
    }
    assert_records(&measured, &ACCURACY, &ACCURACY_FLOAT_SSE2, "samples");
}

/// Runs `args` on `data` with the WP6 probe and the port, compares the
/// output and requires a decode without errors and warnings.
fn compare_api(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let expected = oracle
        .probe("wp6", "api")
        .run(args, data)
        .success()
        .fields();
    let actual = wp6::run_rust(args, data);
    assert_fields(&actual, &expected, args, label);
    expected
}

/// Runs `args` on `data` with the WP2 probe and the port and compares the output.
fn compare_header(oracle: &Oracle, args: &[&str], data: &[u8], label: &str) -> Fields {
    let expected = wp2::without_two_pass_quantize(
        oracle
            .probe("wp2", "header")
            .run(args, data)
            .success()
            .fields(),
    );
    let actual = wp2::run_rust(args, data);
    assert_fields(&actual, &expected, args, label);
    expected
}

fn assert_fields(actual: &Fields, expected: &Fields, args: &[&str], label: &str) {
    if actual != expected {
        let first = actual
            .0
            .iter()
            .zip(&expected.0)
            .position(|(a, e)| a != e)
            .unwrap_or(actual.0.len().min(expected.0.len()));
        let (port, c) = (actual.0.get(first), expected.0.get(first));
        if let (Some((key, port)), Some((_, c))) = (port, c)
            && key == "rows"
        {
            let row = port.split(',').zip(c.split(',')).position(|(a, e)| a != e);
            panic!("{label} {args:?}: output row {row:?} differs");
        }
        panic!(
            "{label} {args:?}: outputs differ at field {first}\n  port:   {port:?}\n  oracle: {c:?}"
        );
    }
}

/// Check 4: every image stream through the WP6 probe and the port.
#[test]
fn streams_match_the_oracle() {
    let Some(oracle) = Oracle::locate("streams_match_the_oracle") else {
        return;
    };
    let Some(t83) = T83::locate("streams_match_the_oracle") else {
        return;
    };
    for (name, ..) in IMAGE_STREAMS {
        let data = t83.stream(name);
        let mut modes: Vec<&[&str]> = vec![
            &["-raw", "-dct", "islow"],
            &["-raw", "-dct", "ifast"],
            &["-raw", "-dct", "float"],
            &[],
        ];
        if PROGRESSIVE_STREAMS.contains(&name) {
            modes.push(&["-buffered"]);
        }
        for args in modes {
            let fields = compare_api(&oracle, args, &data, name);
            assert_eq!(fields.get("error"), Some("0"), "{name} {args:?}");
            assert_eq!(fields.get("num_warnings"), Some("0"), "{name} {args:?}");
        }
    }
}

/// Check 4 for the abbreviated streams: B1, tables only, and B2, whose 255
/// components exceed `MAX_COMPONENTS` (10), an upstream limit.
#[test]
fn abbreviated_streams_match_the_oracle() {
    let Some(oracle) = Oracle::locate("abbreviated_streams_match_the_oracle") else {
        return;
    };
    let Some(t83) = T83::locate("abbreviated_streams_match_the_oracle") else {
        return;
    };
    let tables = t83.stream("B1");
    let image = t83.stream("B2");
    let no_image = J_MESSAGE_CODE::JERR_NO_IMAGE.0.to_string();
    let component_count = J_MESSAGE_CODE::JERR_COMPONENT_COUNT.0.to_string();

    // With require_image false, jpeg_read_header reads B1's tables.
    let fields = compare_header(&oracle, &["-noimage"], &tables, "B1");
    assert_eq!(
        fields.get("status"),
        Some(JPEG_HEADER_TABLES_ONLY.to_string().as_str())
    );
    assert_eq!(fields.get("error"), Some("0"));
    // With require_image true, it fails with JERR_NO_IMAGE.
    for fields in [
        compare_header(&oracle, &[], &tables, "B1"),
        compare_api(&oracle, &[], &tables, "B1"),
    ] {
        assert_eq!(fields.get("error"), Some(no_image.as_str()));
    }
    // B2 fails with JERR_COMPONENT_COUNT at its first scan.
    for fields in [
        compare_header(&oracle, &[], &image, "B2"),
        compare_api(&oracle, &[], &image, "B2"),
    ] {
        assert_eq!(fields.get("error"), Some(component_count.as_str()));
    }

    // The port alone: B2 after B1's tables, in one object, as an application
    // reads the pair, fails with the same error.
    let mut err = Box::new(jpeg_error_mgr::default());
    jpeg_std_error(&mut err);
    let mut cinfo = jpeg_decompress_struct::new(err);
    ok(jpeg_create_decompress(&mut cinfo), &cinfo, "create");
    ok(jpeg_mem_src(&mut cinfo, &tables), &cinfo, "jpeg_mem_src");
    let status = ok(jpeg_read_header(&mut cinfo, false), &cinfo, "B1");
    assert_eq!(status, JPEG_HEADER_TABLES_ONLY);
    assert!(cinfo.quant_tbl_ptrs.iter().all(Option::is_some));
    ok(jpeg_mem_src(&mut cinfo, &image), &cinfo, "jpeg_mem_src");
    assert!(jpeg_read_header(&mut cinfo, true).is_err());
    assert_eq!(
        cinfo.err.msg_code,
        J_MESSAGE_CODE::JERR_COMPONENT_COUNT.0.cast_signed()
    );
    ok(jpeg_destroy_decompress(&mut cinfo), &cinfo, "destroy");
}
