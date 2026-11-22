//! The dav1d default against the libaom reference, on every AVIF file in the
//! repository (owner decision of 2026-11-22, the recorded exception to
//! ADR-0025 point 1: the 1.0 evidence for the dav1d default is bit identity
//! with the libaom reference and with stock dav1d; Argon conformance is a
//! follow-up). Each file is parsed with both decoders: the outcome must be
//! the same, a still image must decode to the same YUV and alpha planes and
//! to the same RGB samples. The corpus is every committed `.avif` file under
//! `codecs/libavif-rs/upstream/libavif` and `tests/fixtures`; nothing is
//! skipped, and the test asserts how many files decoded so a shrinking
//! corpus cannot pass silently.

use super::{Codec, Decoder, ImageInfo, Samples, read};
use std::path::{Path, PathBuf};

const MAX_PIXELS: u64 = 100_000_000;

fn avif_files(directory: &Path, files: &mut Vec<PathBuf>) {
    let mut entries: Vec<_> = std::fs::read_dir(directory)
        .unwrap_or_else(|error| panic!("{}: {error}", directory.display()))
        .map(|entry| entry.unwrap().path())
        .collect();
    entries.sort();
    for path in entries {
        if path.is_dir() {
            avif_files(&path, files);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "avif")
        {
            files.push(path);
        }
    }
}

pub(crate) fn corpus() -> Vec<PathBuf> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut files = Vec::new();
    for directory in ["codecs/libavif-rs/upstream/libavif", "tests/fixtures"] {
        avif_files(&root.join(directory), &mut files);
    }
    files
}

/// Every plane of the decoded image as bytes, in Y, U, V, alpha order, with
/// the image's format and depth.
fn planes(bytes: &[u8], codec: Codec) -> (u32, u32, u32, u32, Vec<Vec<u8>>) {
    let (mut decoder, _) = Decoder::parse_with(bytes, MAX_PIXELS, codec).unwrap();
    let image = decoder.next_image().unwrap();
    // SAFETY: `next_image` returned the decoder's non-null decoded image,
    // which stays valid until the decoder is dropped at the end of this
    // function; every plane pointer libavif sets covers `row_bytes` times
    // the plane's rows.
    unsafe {
        let image = &*image;
        let sample_bytes = if image.depth > 8 { 2 } else { 1 };
        let (shift_x, shift_y, chroma) = match image.yuvFormat {
            read::AVIF_PIXEL_FORMAT_YUV444 => (0, 0, true),
            read::AVIF_PIXEL_FORMAT_YUV422 => (1, 0, true),
            read::AVIF_PIXEL_FORMAT_YUV420 => (1, 1, true),
            read::AVIF_PIXEL_FORMAT_YUV400 => (0, 0, false),
            other => panic!("unexpected YUV format {other}"),
        };
        let plane = |data: *mut u8, row_bytes: u32, width: u32, height: u32| -> Vec<u8> {
            assert!(!data.is_null());
            let row = width as usize * sample_bytes;
            let mut out = Vec::with_capacity(row * height as usize);
            for y in 0..height as usize {
                out.extend_from_slice(std::slice::from_raw_parts(
                    data.add(y * row_bytes as usize),
                    row,
                ));
            }
            out
        };
        let mut planes = vec![plane(
            image.yuvPlanes[0],
            image.yuvRowBytes[0],
            image.width,
            image.height,
        )];
        if chroma {
            let width = (image.width + shift_x) >> shift_x;
            let height = (image.height + shift_y) >> shift_y;
            for index in 1..3 {
                planes.push(plane(
                    image.yuvPlanes[index],
                    image.yuvRowBytes[index],
                    width,
                    height,
                ));
            }
        }
        if !image.alphaPlane.is_null() {
            planes.push(plane(
                image.alphaPlane,
                image.alphaRowBytes,
                image.width,
                image.height,
            ));
        }
        (
            image.width,
            image.height,
            image.depth,
            image.yuvFormat,
            planes,
        )
    }
}

/// The reader's interleaved output at the source depth: 8-bit samples, or
/// 16-bit ones as native-endian bytes.
pub(crate) fn rgb(bytes: &[u8], codec: Codec) -> (ImageInfo, Vec<u8>) {
    let (mut decoder, info) = Decoder::parse_with(bytes, MAX_PIXELS, codec).unwrap();
    let samples = Samples::rgb(info.alpha);
    let count = info.width as usize * info.height as usize * samples.channels();
    let out = if info.depth == 8 {
        let mut out = vec![0_u8; count];
        decoder
            .decode_u8_samples(&mut out, info.width, info.height, samples)
            .unwrap();
        out
    } else {
        let mut out = vec![0_u16; count];
        decoder
            .decode_u16_samples(&mut out, info.width, info.height, samples)
            .unwrap();
        out.iter().flat_map(|sample| sample.to_ne_bytes()).collect()
    };
    (info, out)
}

#[test]
fn dav1d_and_libaom_decode_the_avif_corpus_identically() {
    let files = corpus();
    let mut decoded = 0;
    for path in &files {
        let name = path.display();
        let bytes = std::fs::read(path).unwrap();
        let dav1d = Decoder::parse_with(&bytes, MAX_PIXELS, Codec::Dav1d).map(|(_, info)| info);
        let libaom = Decoder::parse_with(&bytes, MAX_PIXELS, Codec::Libaom).map(|(_, info)| info);
        match (dav1d, libaom) {
            (Err(dav1d), Err(libaom)) => {
                assert_eq!(dav1d, libaom, "{name}: refusals differ");
                continue;
            }
            (Ok(dav1d), Ok(libaom)) => {
                assert_eq!(
                    format!("{dav1d:?}"),
                    format!("{libaom:?}"),
                    "{name}: parsed information differs"
                );
            }
            (dav1d, libaom) => panic!("{name}: dav1d {dav1d:?}, libaom {libaom:?}"),
        }
        let (dav1d_width, dav1d_height, dav1d_depth, dav1d_format, dav1d_planes) =
            planes(&bytes, Codec::Dav1d);
        let (width, height, depth, format, libaom_planes) = planes(&bytes, Codec::Libaom);
        assert_eq!(
            (dav1d_width, dav1d_height, dav1d_depth, dav1d_format),
            (width, height, depth, format),
            "{name}: decoded image differs"
        );
        assert_eq!(
            dav1d_planes.len(),
            libaom_planes.len(),
            "{name}: plane count differs"
        );
        for (index, (dav1d, libaom)) in dav1d_planes.iter().zip(&libaom_planes).enumerate() {
            assert!(dav1d == libaom, "{name}: plane {index} differs");
        }
        let (_, dav1d_rgb) = rgb(&bytes, Codec::Dav1d);
        let (_, libaom_rgb) = rgb(&bytes, Codec::Libaom);
        assert!(dav1d_rgb == libaom_rgb, "{name}: RGB samples differ");
        decoded += 1;
    }
    eprintln!(
        "dav1d == libaom: {decoded} of {} files decoded identically, the rest refused alike",
        files.len()
    );
    // The committed corpus on 2026-11-22: 95 files, 81 of them still images
    // the reader decodes (the others are animations, malformed or
    // unsupported, and both decoders refuse them alike).
    assert!(files.len() >= 95, "corpus shrank: {} files", files.len());
    assert!(decoded >= 81, "only {decoded} files decoded");
}

/// SHA-256 (FIPS 180-4) of `data`, so the test needs no hash dependency.
fn sha256(data: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a_2f98,
        0x7137_4491,
        0xb5c0_fbcf,
        0xe9b5_dba5,
        0x3956_c25b,
        0x59f1_11f1,
        0x923f_82a4,
        0xab1c_5ed5,
        0xd807_aa98,
        0x1283_5b01,
        0x2431_85be,
        0x550c_7dc3,
        0x72be_5d74,
        0x80de_b1fe,
        0x9bdc_06a7,
        0xc19b_f174,
        0xe49b_69c1,
        0xefbe_4786,
        0x0fc1_9dc6,
        0x240c_a1cc,
        0x2de9_2c6f,
        0x4a74_84aa,
        0x5cb0_a9dc,
        0x76f9_88da,
        0x983e_5152,
        0xa831_c66d,
        0xb003_27c8,
        0xbf59_7fc7,
        0xc6e0_0bf3,
        0xd5a7_9147,
        0x06ca_6351,
        0x1429_2967,
        0x27b7_0a85,
        0x2e1b_2138,
        0x4d2c_6dfc,
        0x5338_0d13,
        0x650a_7354,
        0x766a_0abb,
        0x81c2_c92e,
        0x9272_2c85,
        0xa2bf_e8a1,
        0xa81a_664b,
        0xc24b_8b70,
        0xc76c_51a3,
        0xd192_e819,
        0xd699_0624,
        0xf40e_3585,
        0x106a_a070,
        0x19a4_c116,
        0x1e37_6c08,
        0x2748_774c,
        0x34b0_bcb5,
        0x391c_0cb3,
        0x4ed8_aa4a,
        0x5b9c_ca4f,
        0x682e_6ff3,
        0x748f_82ee,
        0x78a5_636f,
        0x84c8_7814,
        0x8cc7_0208,
        0x90be_fffa,
        0xa450_6ceb,
        0xbef9_a3f7,
        0xc671_78f2,
    ];
    let mut state: [u32; 8] = [
        0x6a09_e667,
        0xbb67_ae85,
        0x3c6e_f372,
        0xa54f_f53a,
        0x510e_527f,
        0x9b05_688c,
        0x1f83_d9ab,
        0x5be0_cd19,
    ];
    let mut message = data.to_vec();
    message.push(0x80);
    while message.len() % 64 != 56 {
        message.push(0);
    }
    message.extend_from_slice(&(data.len() as u64 * 8).to_be_bytes());
    for block in message.as_chunks::<64>().0 {
        let mut w = [0_u32; 64];
        for (word, bytes) in w.iter_mut().zip(block.as_chunks::<4>().0) {
            *word = u32::from_be_bytes(*bytes);
        }
        for i in 16..64 {
            let s0 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
            let s1 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
            w[i] = w[i - 16]
                .wrapping_add(s0)
                .wrapping_add(w[i - 7])
                .wrapping_add(s1);
        }
        let [mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut h] = state;
        for i in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choice = (e & f) ^ (!e & g);
            let t1 = h
                .wrapping_add(s1)
                .wrapping_add(choice)
                .wrapping_add(K[i])
                .wrapping_add(w[i]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let majority = (a & b) ^ (a & c) ^ (b & c);
            let t2 = s0.wrapping_add(majority);
            h = g;
            g = f;
            f = e;
            e = d.wrapping_add(t1);
            d = c;
            c = b;
            b = a;
            a = t1.wrapping_add(t2);
        }
        for (word, value) in state.iter_mut().zip([a, b, c, d, e, f, g, h]) {
            *word = word.wrapping_add(value);
        }
    }
    state.iter().map(|word| format!("{word:08x}")).collect()
}

#[test]
fn sha256_matches_the_standard_vectors() {
    assert_eq!(
        sha256(b""),
        "e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855"
    );
    assert_eq!(
        sha256(b"abc"),
        "ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad"
    );
    assert_eq!(
        sha256(b"abcdbcdecdefdefgefghfghighijhijkijkljklmklmnlmnomnopnopq"),
        "248d6a61d20638b8e5c026930c3e6039a33ce45964ff2167f6ecedd419db06c1"
    );
}

/// The reader's output with the dav1d default, and with the libaom
/// reference, equals the pinned C libavif with portable C libaom on the
/// seven committed fixtures of the 2026-11-11 parity record
/// (`docs/benchmarks/M2-avif-input.md`, "Independent reference and output
/// contract"): the SHA-256 of the raw interleaved samples, 8-bit or
/// native-endian 16-bit, as the C consumer `tests/ports/avif_input_rgb.c`
/// wrote them. The hashes are that record's, never refreshed from the port;
/// its eighth case, a temporary photo, is not committed. 16-bit samples are
/// compared as little-endian bytes, as recorded on macOS ARM64.
#[test]
fn both_decoders_reproduce_the_pinned_c_oracle() {
    const LIBAVIF: &str = "codecs/libavif-rs/upstream/libavif/";
    const ANDROID: &str = "codecs/libavif-rs/upstream/libavif/android_jni/avifandroidjni/src/androidTest/assets/avif/";
    let cases = [
        (
            format!("{LIBAVIF}tests/data/io/kodim03_yuv420_8bpc.avif"),
            "87435c26f9b6604eb3c77e0a93b504f32e4673bc651cf19e038650e24ee02b03",
        ),
        (
            format!("{ANDROID}fox.profile1.10bpc.yuv444.avif"),
            "d05e2a4c1704fb16554adcd9ac2d93ce743838c87f1cc5513170fb3171c7c885",
        ),
        (
            format!("{ANDROID}fox.profile2.12bpc.yuv444.avif"),
            "07ddf6e578be0854464c48e991e77b64883371f28966bc75a4d24441fd6aa69c",
        ),
        (
            format!("{LIBAVIF}tests/data/sofa_grid1x5_420.avif"),
            "dac5f75d9d5a530b989b75898bdff5558ef048a575e7a8a6d349d7a73c5d59b1",
        ),
        (
            format!("{LIBAVIF}tests/data/abc_color_irot_alpha_irot.avif"),
            "5102863ca73f618c60944e490aa3982e7a1afd6975f4d0edf12b40ac85c88f82",
        ),
        (
            format!("{LIBAVIF}tests/data/color_grid_alpha_nogrid.avif"),
            "816eeaa9e291fc6b7fa5bb2735e165de7de66bb5512cb15ab8f714b72ad2c77d",
        ),
        (
            format!("{LIBAVIF}tests/data/color_nogrid_alpha_nogrid_gainmap_grid.avif"),
            "5c86e460c3b35c3ec7c88a2ac863164b4b5a67fb8da85b04418fa8e68a40e262",
        ),
    ];
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    for (file, expected) in cases {
        let bytes = std::fs::read(root.join(&file)).unwrap();
        for codec in [Codec::Dav1d, Codec::Libaom] {
            let (info, mut samples) = rgb(&bytes, codec);
            if info.depth > 8 && cfg!(target_endian = "big") {
                for pair in samples.as_chunks_mut::<2>().0 {
                    pair.swap(0, 1);
                }
            }
            assert_eq!(sha256(&samples), expected, "{file} with {codec:?}");
        }
    }
}
