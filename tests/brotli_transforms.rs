// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.

#[path = "../src/brotli/dictionary.rs"]
pub mod dictionary;
#[path = "../src/brotli/tables.rs"]
pub mod tables;
#[path = "../src/brotli/transform.rs"]
pub mod transform;

use dictionary::*;
use transform::*;

#[cfg(feature = "cpp-oracle")]
fn destination(seed: u32) -> [u8; 64] {
    std::array::from_fn(|i| seed.wrapping_add(i as u32 * 37) as u8)
}

#[cfg(feature = "cpp-oracle")]
struct BorrowedDestination<'a>(&'a mut [u8]);

#[cfg(feature = "cpp-oracle")]
impl ByteDestination for BorrowedDestination<'_> {
    fn read(&self, index: usize) -> u8 {
        self.0[index]
    }
    fn write(&mut self, index: usize, value: u8) {
        self.0[index] = value;
    }
    fn xor(&mut self, index: usize, mask: u8) {
        self.0[index] ^= mask;
    }
}

fn custom_tables<'a>(
    prefix_suffix: &'a [u8],
    map: &'a [u16],
    triplet: &'a [u8],
    params: &'a [u8],
) -> BrotliTransforms<'a> {
    BrotliTransforms {
        prefix_suffix_size: prefix_suffix.len() as u16,
        prefix_suffix,
        prefix_suffix_map: map,
        num_transforms: 1,
        transforms: triplet,
        params: if [BROTLI_TRANSFORM_SHIFT_FIRST, BROTLI_TRANSFORM_SHIFT_ALL].contains(&triplet[1])
        {
            Some(params)
        } else {
            None
        },
        cut_off_transforms: [-1; 10],
    }
}

#[test]
fn embedded_dictionary_setter_preserves_the_static_owner_and_data() {
    let dictionary = brotli_get_dictionary();
    brotli_set_dictionary_data(None);
    brotli_set_dictionary_data(Some(b"alternate"));
    assert!(std::ptr::eq(dictionary, brotli_get_dictionary()));
    assert_eq!(dictionary.data_size, 122784);
    assert_eq!(dictionary.data.len(), dictionary.data_size);
    assert_eq!(&dictionary.data[..4], b"time");
    assert_eq!(brotli_get_transforms().num_transforms, 121);
}

#[test]
fn malformed_builtin_word_preserves_uppercase_writes_beyond_returned_length() {
    let dictionary = brotli_get_dictionary();
    let start = dictionary.offsets_by_length[4] as usize + 436 * 4;
    assert_eq!(
        &dictionary.data[start..start + 4],
        &[0x7a, 0x68, 0x3a, 0xe5]
    );
    let mut dst = [0xa5; 64];
    let length = brotli_transform_dictionary_word(
        &mut dst,
        &dictionary.data[start..],
        4,
        brotli_get_transforms(),
        44,
    );
    assert_eq!(length, 4);
    assert_eq!(&dst[..7], &[b'Z', b'H', b':', 0xe5, 0xa5, 0xa0, 0xa5]);
}

#[derive(Debug, Eq, PartialEq)]
enum Access {
    Read(usize),
    Write(usize, u8),
    Xor(usize, u8),
}

struct TraceDestination {
    bytes: [u8; 64],
    accesses: std::cell::RefCell<Vec<Access>>,
}

impl ByteDestination for TraceDestination {
    fn read(&self, index: usize) -> u8 {
        self.accesses.borrow_mut().push(Access::Read(index));
        self.bytes[index]
    }
    fn write(&mut self, index: usize, value: u8) {
        self.accesses.get_mut().push(Access::Write(index, value));
        self.bytes[index] = value;
    }
    fn xor(&mut self, index: usize, mask: u8) {
        self.accesses.get_mut().push(Access::Xor(index, mask));
        self.bytes[index] ^= mask;
    }
}

#[test]
fn destination_view_keeps_tail_xor_separate_from_initialized_reads() {
    let triplet = [0, BROTLI_TRANSFORM_UPPERCASE_ALL, 0];
    let transforms = custom_tables(&[0], &[0], &triplet, &[]);
    let mut destination = TraceDestination {
        bytes: [0xa5; 64],
        accesses: Default::default(),
    };
    let len = brotli_transform_dictionary_word_to(&mut destination, &[0xe5], 1, &transforms, 0);
    assert_eq!(len, 1);
    assert_eq!(
        destination.accesses.into_inner(),
        [
            Access::Write(0, 0xe5),
            Access::Read(0),
            Access::Read(0),
            Access::Xor(2, 5)
        ]
    );
    assert_eq!(&destination.bytes[..4], &[0xe5, 0xa5, 0xa0, 0xa5]);
}

#[test]
fn guarded_multibyte_shift_does_not_read_past_the_logical_length() {
    let triplet = [0, BROTLI_TRANSFORM_SHIFT_ALL, 0];
    let transforms = custom_tables(&[0], &[0], &triplet, &[0xff, 0xff]);
    let mut destination = TraceDestination {
        bytes: [0xa5; 64],
        accesses: Default::default(),
    };
    let len =
        brotli_transform_dictionary_word_to(&mut destination, &[0xf0, 0x80], 2, &transforms, 0);
    assert_eq!(len, 2);
    assert_eq!(
        destination.accesses.into_inner(),
        [
            Access::Write(0, 0xf0),
            Access::Write(1, 0x80),
            Access::Read(0),
            Access::Read(0),
            Access::Read(0),
            Access::Read(0),
            Access::Read(0)
        ]
    );
}

#[cfg(feature = "cpp-oracle")]
mod parity {
    use super::*;
    use std::io::{BufReader, Read as _};
    use std::process::{Child, Command, Stdio};

    fn oracle(mode: &str) -> Child {
        Command::new(
            std::env::var_os("BROTLI_TRANSFORMS_ORACLE").expect(
                "build the original Brotli transform oracle and set BROTLI_TRANSFORMS_ORACLE",
            ),
        )
        .arg(mode)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()
        .unwrap()
    }

    fn compare_record<const N: usize>(
        reader: &mut impl std::io::Read,
        len: i32,
        dst: &[u8; N],
        context: &str,
    ) {
        let mut header = [0u8; 4];
        reader.read_exact(&mut header).unwrap();
        let mut actual = [0u8; N];
        reader.read_exact(&mut actual).unwrap();
        assert_eq!(i32::from_le_bytes(header), len, "{context}");
        assert_eq!(&actual, dst, "{context}");
    }

    fn finish(mut child: Child, mut reader: impl std::io::Read) {
        let mut extra = Vec::new();
        reader.read_to_end(&mut extra).unwrap();
        assert!(extra.is_empty());
        assert!(child.wait().unwrap().success());
    }

    #[test]
    fn all_builtin_dictionary_words_and_transforms_preserve_every_destination_byte() {
        let mut child = oracle("builtin");
        drop(child.stdin.take());
        let mut reader = BufReader::new(child.stdout.take().unwrap());
        let dictionary = brotli_get_dictionary();
        let transforms = brotli_get_transforms();
        let mut count = 0;
        for length in 4..=24 {
            for index in 0..1u32 << dictionary.size_bits_by_length[length] {
                let offset =
                    dictionary.offsets_by_length[length] as usize + index as usize * length;
                for transform in 0..transforms.num_transforms {
                    let seed = length as u32 * 11 + index * 3 + transform * 17;
                    let mut dst = destination(seed);
                    let mut generic = destination(seed);
                    let len = brotli_transform_dictionary_word(
                        &mut dst,
                        &dictionary.data[offset..],
                        length as i32,
                        transforms,
                        transform as i32,
                    );
                    let generic_len = brotli_transform_dictionary_word_to(
                        &mut BorrowedDestination(&mut generic),
                        &dictionary.data[offset..],
                        length as i32,
                        transforms,
                        transform as i32,
                    );
                    assert_eq!(generic_len, len);
                    assert_eq!(generic, dst);
                    compare_record(&mut reader, len, &dst, "builtin tuple");
                    count += 1;
                }
            }
        }
        assert_eq!(count, 1_633_984);
        finish(child, reader);
        eprintln!("Brotli builtin transform tuples: {count}");
    }

    fn u32_bytes(out: &mut Vec<u8>, value: u32) {
        out.extend(value.to_le_bytes());
    }

    #[test]
    fn embedded_dictionary_and_transform_metadata_match_original_exports() {
        let mut child = oracle("metadata");
        drop(child.stdin.take());
        let mut actual = Vec::new();
        child
            .stdout
            .take()
            .unwrap()
            .read_to_end(&mut actual)
            .unwrap();
        assert!(child.wait().unwrap().success());
        let dictionary = brotli_get_dictionary();
        let transforms = brotli_get_transforms();
        let mut expected = Vec::new();
        u32_bytes(&mut expected, 1);
        u32_bytes(&mut expected, 1);
        for bits in dictionary.size_bits_by_length {
            u32_bytes(&mut expected, u32::from(bits));
        }
        for offset in dictionary.offsets_by_length {
            u32_bytes(&mut expected, offset);
        }
        u32_bytes(&mut expected, dictionary.data_size as u32);
        expected.extend(dictionary.data);
        u32_bytes(&mut expected, u32::from(transforms.prefix_suffix_size));
        u32_bytes(&mut expected, transforms.num_transforms);
        u32_bytes(&mut expected, 1);
        for cutoff in transforms.cut_off_transforms {
            u32_bytes(&mut expected, cutoff as u32);
        }
        expected.extend(transforms.prefix_suffix);
        for &map in transforms.prefix_suffix_map {
            u32_bytes(&mut expected, u32::from(map));
        }
        expected.extend(transforms.transforms);
        assert_eq!(actual, expected);
    }

    struct Case {
        transform: u8,
        parameter: u16,
        len: i32,
        seed: u32,
        prefix: Vec<u8>,
        suffix: Vec<u8>,
        word: Vec<u8>,
    }

    fn custom_cases(mut visit: impl FnMut(Case)) -> usize {
        let mut count = 0;
        let mut emit = |case| {
            visit(case);
            count += 1;
        };
        for transform in 0..=255u8 {
            for len in [-3, -1, 0, 1, 2, 3, 4, 5, 8, 12] {
                emit(Case {
                    transform,
                    parameter: 0x9234,
                    len,
                    seed: u32::from(transform),
                    prefix: if transform % 2 == 0 {
                        b"pre".to_vec()
                    } else {
                        vec![]
                    },
                    suffix: if transform % 3 == 0 {
                        b"suffix".to_vec()
                    } else {
                        vec![]
                    },
                    word: b"aBcDxyZ12345".to_vec(),
                });
            }
        }
        for first in 0..=255u8 {
            for len in 0..=8 {
                for transform in [10, 11, 21, 22] {
                    for &parameter in if transform >= 21 {
                        &[0, 1, 0x7fff, 0x8000, 0xffff][..]
                    } else {
                        &[0][..]
                    } {
                        emit(Case {
                            transform,
                            parameter,
                            len,
                            seed: u32::from(first),
                            prefix: vec![b'<'],
                            suffix: vec![b'>'],
                            word: vec![first, 0x81, 0xb2, 0xc3, 0xe5, 0xa9, b'z', b'q'],
                        });
                    }
                }
            }
        }
        for parameter in 0..=u16::MAX {
            for word in [
                &b"a"[..],
                &[0xc3, 0xa9],
                &[0xe2, 0x82, 0xac],
                &[0xf0, 0x90, 0x8d, 0x88],
            ] {
                for transform in [21, 22] {
                    emit(Case {
                        transform,
                        parameter,
                        len: word.len() as i32,
                        seed: u32::from(parameter),
                        prefix: vec![],
                        suffix: vec![],
                        word: word.to_vec(),
                    });
                }
            }
        }
        count
    }

    #[test]
    fn maximum_byte_prefix_suffix_lengths_keep_the_whole_physical_destination() {
        let mut child = oracle("wide");
        drop(child.stdin.take());
        let mut reader = BufReader::new(child.stdout.take().unwrap());
        let mut count = 0;
        for prefix_len in [0usize, 1, 127, 128, 254, 255] {
            for suffix_len in [0usize, 1, 127, 128, 254, 255] {
                let prefix = vec![b'p'; prefix_len];
                let suffix = vec![b's'; suffix_len];
                let mut prefix_suffix = vec![prefix_len as u8];
                prefix_suffix.extend(&prefix);
                prefix_suffix.push(suffix_len as u8);
                prefix_suffix.extend(&suffix);
                prefix_suffix.push(0);
                let map = [0, prefix_len as u16 + 1];
                for transform in 0..=23u8 {
                    let triplet = [0, transform, 1];
                    let transforms = custom_tables(&prefix_suffix, &map, &triplet, &[0, 0x80]);
                    for logical_len in [-3, 0, 1, 4, 24] {
                        let seed =
                            prefix_len as u32 + suffix_len as u32 * 3 + u32::from(transform) * 7;
                        let mut dst: [u8; 1024] =
                            std::array::from_fn(|i| seed.wrapping_add(i as u32 * 37) as u8);
                        let mut generic = dst;
                        let word = b"aBcDxyZ12345qRsTuVwXyZ01";
                        let len = brotli_transform_dictionary_word(
                            &mut dst,
                            word,
                            logical_len,
                            &transforms,
                            0,
                        );
                        let generic_len = brotli_transform_dictionary_word_to(
                            &mut BorrowedDestination(&mut generic),
                            word,
                            logical_len,
                            &transforms,
                            0,
                        );
                        assert_eq!(generic_len, len);
                        assert_eq!(generic, dst);
                        compare_record(&mut reader, len, &dst, "wide affix");
                        count += 1;
                    }
                }
            }
        }
        assert_eq!(count, 4320);
        finish(child, reader);
        eprintln!("Brotli wide transform cases: {count}");
    }

    #[test]
    fn custom_types_all_shift_parameters_and_utf8_branches_match_every_source_byte() {
        let mut child = oracle("custom-corpus");
        drop(child.stdin.take());
        let mut reader = BufReader::new(child.stdout.take().unwrap());
        let count = custom_cases(|case| {
            let mut prefix_suffix = vec![case.prefix.len() as u8];
            prefix_suffix.extend(&case.prefix);
            prefix_suffix.push(case.suffix.len() as u8);
            prefix_suffix.extend(&case.suffix);
            prefix_suffix.push(0);
            let map = [0, case.prefix.len() as u16 + 1];
            let triplet = [0, case.transform, 1];
            let params = case.parameter.to_le_bytes();
            let transforms = custom_tables(&prefix_suffix, &map, &triplet, &params);
            let mut word = case.word;
            word.resize(word.len() + 16, 0xd3);
            let mut dst = destination(case.seed);
            let mut generic = destination(case.seed);
            let len = brotli_transform_dictionary_word(&mut dst, &word, case.len, &transforms, 0);
            let generic_len = brotli_transform_dictionary_word_to(
                &mut BorrowedDestination(&mut generic),
                &word,
                case.len,
                &transforms,
                0,
            );
            assert_eq!(generic_len, len);
            assert_eq!(generic, dst);
            compare_record(&mut reader, len, &dst, "custom transform");
        });
        assert_eq!(count, 554_496);
        finish(child, reader);
        eprintln!("Brotli custom transform cases: {count}");
    }
}
