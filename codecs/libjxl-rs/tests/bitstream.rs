// Copyright (c) the JPEG XL Project Authors. All rights reserved.
// Use of this source code is governed by a BSD-style license in LICENSE.
// Adapted from bit_reader_test.cc and enc_bit_writer_test.cc. Generated cases
// use a local deterministic generator rather than the upstream C++ RNGs.

use jxl::jxl::base::status::{Status, StatusCode};
use jxl::jxl::dec_bit_reader::{BitReader, BitReaderScopedCloser};
use jxl::jxl::enc_aux_out::{AuxOut, LayerType};
use jxl::jxl::enc_bit_writer::BitWriter;

fn write_patches(patches: &[(usize, u64)]) -> BitWriter {
    let mut writer = BitWriter::new();
    let budget: usize = patches.iter().map(|p| p.0).sum();
    writer
        .with_max_bits(
            (budget + 7) & !7,
            LayerType::Header,
            None,
            |writer, _| {
                for &(width, value) in patches {
                    writer.write(width, value);
                }
                writer.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    writer
}

fn next(seed: &mut u64) -> u64 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    *seed
}

#[test]
fn extends_with_zeroes() {
    for size in 4..32 {
        let data = vec![0xFF; size];
        for n_bytes in 0..size {
            let mut reader = BitReader::new(&data[..n_bytes]);
            for _ in 0..n_bytes * 8 {
                assert_eq!(reader.read_bits(1), 1);
            }
            for width in 0..BitReader::MAX_BITS_PER_CALL {
                assert_eq!(reader.peek_bits(width), 0);
            }
            assert_eq!(reader.close(), Ok(()));
        }
    }
}

#[test]
fn order() {
    let mut single_bits = vec![(1, 1); 5];
    single_bits.extend(vec![(1, 0); 5]);
    single_bits.extend(vec![(1, 1); 6]);
    let cases = [
        (single_bits, [0x1F, 0xFC]),
        (vec![(8, 0xF8), (8, 0x3F)], [0xF8, 0x3F]),
        (vec![(16, 0xF83F)], [0x3F, 0xF8]),
        (vec![(1, 1), (3, 6), (8, 0xDB), (4, 8)], [0xBD, 0x8D]),
    ];
    for (patches, expected) in cases {
        let writer = write_patches(&patches);
        assert_eq!(writer.get_span(), expected);
        let mut reader = BitReader::new(writer.get_span());
        assert_eq!(reader.read_fixed_bits::<8>(), u64::from(expected[0]));
        assert_eq!(reader.read_fixed_bits::<8>(), u64::from(expected[1]));
        reader.close().unwrap();
    }
}

#[test]
fn total_counters() {
    let bytes = [1, 2, 3, 4, 0, 0, 0, 0];
    let mut reader = BitReader::new(&bytes);
    assert_eq!(reader.total_bytes(), bytes.len());
    assert_eq!(reader.total_bits_consumed(), 0);
    for (width, total) in [(1, 1), (10, 11), (4, 15), (1, 16), (16, 32)] {
        reader.read_bits(width);
        assert_eq!(reader.total_bits_consumed(), total);
    }
    reader.close().unwrap();
}

#[test]
fn move_transfers_close_obligation() {
    let bytes = [1, 2, 3, 4, 0, 0, 0, 0];
    let mut reader2 = {
        let mut reader1 = BitReader::new(&bytes);
        assert_eq!(reader1.total_bits_consumed(), 0);
        reader1.read_fixed_bits::<16>();
        assert_eq!(reader1.total_bits_consumed(), 16);
        reader1
    };
    assert_eq!(reader2.total_bits_consumed(), 16);
    assert_eq!(reader2.read_fixed_bits::<8>(), 3);
    assert_eq!(reader2.total_bits_consumed(), 24);
    reader2.close().unwrap();
}

#[test]
fn skip_matches_individual_reads() {
    for task in 0..96 {
        for skip in 0..128 {
            let mut patches = vec![(1, 1); task];
            patches.extend(vec![(1, 0); skip]);
            patches.push((3, 5));
            let writer = write_patches(&patches);
            let mut reader1 = BitReader::new(writer.get_span());
            let mut reader2 = BitReader::new(writer.get_span());
            for _ in 0..task {
                assert_eq!(reader1.read_bits(1), 1);
                assert_eq!(reader2.read_bits(1), 1);
            }
            reader1.skip_bits(skip);
            for _ in 0..skip {
                assert_eq!(reader2.read_bits(1), 0);
            }
            assert_eq!(reader1.total_bits_consumed(), reader2.total_bits_consumed());
            assert_eq!(reader1.read_bits(3), 5);
            assert_eq!(reader2.read_bits(3), 5);
            reader1.close().unwrap();
            reader2.close().unwrap();
        }
    }
}

#[test]
fn round_trips_thousand_streams() {
    for task in 0..1000 {
        let mut seed = 55537 + 129 * task;
        let mut total = 0;
        let mut patches = Vec::new();
        loop {
            let width = 1 + next(&mut seed) as usize % 32;
            if total + width > 8000 {
                break;
            }
            let value = next(&mut seed) & ((1 << width) - 1);
            patches.push((width, value));
            total += width;
        }
        let writer = write_patches(&patches);
        let mut reader = BitReader::new(writer.get_span());
        for (width, value) in patches {
            assert_eq!(reader.read_bits(width), value);
        }
        reader.close().unwrap();
    }
}

#[test]
fn random_sequence_million_patches() {
    let mut seed = 42;
    let patches: Vec<_> = (0..1024 * 1024)
        .map(|_| {
            let width = 1 + next(&mut seed) as usize % BitWriter::MAX_BITS_PER_CALL;
            (width, next(&mut seed) & ((1 << width) - 1))
        })
        .collect();
    let writer = write_patches(&patches);
    let mut reader = BitReader::new(writer.get_span());
    for (width, value) in patches {
        assert_eq!(reader.read_bits(width), value);
    }
    reader.jump_to_byte_boundary().unwrap();
    reader.close().unwrap();
}

#[test]
fn bounds_checks_defer_and_acknowledge_failure() {
    let mut reader = BitReader::new(&[0xFF]);
    assert_eq!(reader.read_bits(9), 0xFF);
    assert_eq!(reader.close(), Err(StatusCode::GenericError));

    let mut reader = BitReader::new(&[0xFF]);
    reader.read_bits(9);
    assert_eq!(
        reader.all_reads_within_bounds(),
        Err(StatusCode::GenericError)
    );
    // The caller has handled this condition; Close does not report it again.
    assert_eq!(reader.close(), Ok(()));

    let mut reader = BitReader::new(&[0xFF]);
    reader.skip_bits(usize::MAX);
    assert!(reader.total_bits_consumed() > 8);
    assert_eq!(reader.close(), Err(StatusCode::GenericError));
}

#[test]
fn scoped_close_propagates_failure() {
    for (read, expected) in [(8, Ok(())), (9, Err(StatusCode::GenericError))] {
        let mut reader = BitReader::new(&[0xFF]);
        let mut status: Status = Ok(());
        {
            let mut closer = BitReaderScopedCloser::new(&mut reader, &mut status);
            closer.read_bits(read);
        }
        assert_eq!(status, expected);
    }
    let mut reader = BitReader::new(&[]);
    let mut status = Err(StatusCode::Unsupported);
    drop(BitReaderScopedCloser::new(&mut reader, &mut status));
    assert_eq!(status, Err(StatusCode::Unsupported));
}

#[test]
fn padding_and_corruption_checkpoints() {
    let mut reader = BitReader::new(&[0x81]);
    assert_eq!(reader.read_bits(1), 1);
    assert_eq!(
        reader.jump_to_byte_boundary(),
        Err(StatusCode::GenericError)
    );
    assert_eq!(reader.total_bits_consumed(), 8);
    reader.close().unwrap();

    let mut reader = BitReader::new(&[0xFF; 10]);
    reader.mark_unhealthy();
    assert_eq!(
        reader.all_reads_within_bounds(),
        Err(StatusCode::GenericError)
    );
    reader.read_bits(56);
    assert_eq!(reader.close(), Err(StatusCode::GenericError));
}

#[test]
fn nested_allotments_charge_once_and_reclaim_on_error() {
    for fail in [false, true] {
        let mut writer = BitWriter::new();
        let mut aux = AuxOut::default();
        let result = writer.with_max_bits(
            16,
            LayerType::Header,
            Some(&mut aux),
            |writer, aux| {
                writer.write(3, 7);
                writer.with_max_bits(
                    64,
                    LayerType::Ac,
                    aux,
                    |writer, _| {
                        writer.write(13, 4097);
                        if fail {
                            Err(StatusCode::GenericError)
                        } else {
                            Ok(())
                        }
                    },
                    true,
                )?;
                writer.write(3, 5);
                Ok(())
            },
            true,
        );
        assert_eq!(
            result,
            if fail {
                Err(StatusCode::GenericError)
            } else {
                Ok(())
            }
        );
        assert_eq!(
            aux.layer(LayerType::Header).total_bits,
            if fail { 3 } else { 6 }
        );
        assert_eq!(aux.layer(LayerType::Ac).total_bits, 13);
        assert_eq!(
            aux.layer(LayerType::Ac).histogram_bits,
            if fail { 0 } else { 13 }
        );
        assert_eq!(writer.bits_written(), if fail { 16 } else { 19 });
        writer
            .with_max_bits(
                8,
                LayerType::Header,
                None,
                |w, _| {
                    w.zero_pad_to_byte();
                    Ok(())
                },
                false,
            )
            .unwrap();
        writer.append_byte_aligned(&[0xA5, 0x5A]).unwrap();
        writer.shrink().unwrap();
        assert!(writer.get_span().ends_with(&[0xA5, 0x5A]));
    }
}

#[test]
fn append_aligned_and_unaligned_preserves_next_byte_initialization() {
    let mut writer = write_patches(&[(3, 5)]);
    let mut unaligned = BitWriter::new();
    unaligned
        .with_max_bits(
            13,
            LayerType::Header,
            None,
            |w, _| {
                w.write(13, 0x1ABC);
                Ok(())
            },
            false,
        )
        .unwrap();
    writer.append_unaligned(&unaligned).unwrap();
    writer
        .with_max_bits(
            7,
            LayerType::Header,
            None,
            |w, _| {
                w.zero_pad_to_byte();
                Ok(())
            },
            false,
        )
        .unwrap();
    let other = write_patches(&[(8, 0xEF)]);
    let empty = BitWriter::new();
    writer
        .append_byte_aligned_writers(&[&empty, &other])
        .unwrap();
    writer
        .with_max_bits(
            8,
            LayerType::Header,
            None,
            |w, _| {
                w.write(8, 0x42);
                Ok(())
            },
            false,
        )
        .unwrap();
    assert_eq!(
        &writer.get_span()[writer.get_span().len() - 2..],
        &[0xEF, 0x42]
    );
    let mut reader = BitReader::new(writer.get_span());
    assert_eq!(reader.read_bits(8), 5); // write_patches padded the first writer.
    assert_eq!(reader.read_bits(13), 0x1ABC);
    reader.close().unwrap();
}

#[test]
fn writer_storage_can_move_across_threads() {
    let writer = write_patches(&[(8, 0x12)]);
    let writer = std::thread::spawn(move || {
        let mut writer = writer;
        writer.append_byte_aligned(&[0x34]).unwrap();
        writer
    })
    .join()
    .unwrap();
    assert_eq!(writer.get_span(), [0x12, 0x34]);
}
