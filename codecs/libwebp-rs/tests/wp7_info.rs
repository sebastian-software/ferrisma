//! WP7 differential test: `VP8LCheckSignature` and `VP8LGetInfo` of
//! `src/dec/vp8l_dec.c` on the headers of the lossless test vectors, their
//! prefixes, mutated headers and random bytes, against the C probe
//! `tools/oracle/probes/wp7/info.c`.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "wp7_support/mod.rs"]
mod wp7_support;

use std::fmt::Write as _;

use oracle::Oracle;
use webp::dec::vp8l_dec::{vp8l_check_signature, vp8l_get_info};
use wp7_support::{Rng, compare, find_chunk, hex};

fn command(commands: &mut String, expected: &mut String, data: &[u8], size: usize) {
    let bytes = if data.is_empty() {
        "-".to_owned()
    } else {
        hex(data)
    };
    writeln!(commands, "info {bytes} {size}").unwrap();
    let input = &data[..size];
    writeln!(
        expected,
        "signature={}",
        i32::from(vp8l_check_signature(input))
    )
    .unwrap();
    match vp8l_get_info(input) {
        Some((width, height, has_alpha)) => writeln!(
            expected,
            "info=1\nwidth={width}\nheight={height}\nhas_alpha={}",
            i32::from(has_alpha)
        )
        .unwrap(),
        None => writeln!(expected, "info=0\nwidth=-1\nheight=-1\nhas_alpha=-1").unwrap(),
    }
}

#[test]
fn header_info_matches_the_c_functions() {
    let Some(oracle) = Oracle::locate("header_info_matches_the_c_functions") else {
        return;
    };
    let mut rng = Rng(0x5eed_0701);
    let mut commands = String::new();
    let mut expected = String::new();
    let mut count = 0;

    for path in oracle::test_vectors() {
        let file = std::fs::read(&path).unwrap();
        let payload = match find_chunk(&file, b"VP8L") {
            Some((offset, size)) => &file[offset..offset + size.min(64)],
            // Lossy files: the start of their first chunk has no VP8L signature.
            None => &file[12..file.len().min(40)],
        };
        for size in 0..=payload.len().min(12) {
            command(&mut commands, &mut expected, payload, size);
            count += 1;
        }
        command(&mut commands, &mut expected, payload, payload.len());
        count += 1;
        // Mutated headers: the magic byte, the size and alpha bits, the version.
        if payload.len() >= 5 {
            for _ in 0..12 {
                let mut header = payload[..payload.len().min(16)].to_vec();
                let position = rng.below(5) as usize;
                header[position] ^= 1 << rng.below(8);
                command(&mut commands, &mut expected, &header, header.len());
                count += 1;
            }
        }
    }
    // Random headers, most with the signature byte.
    for _ in 0..400 {
        let length = rng.below(10) as usize;
        let mut data: Vec<u8> = (0..length).map(|_| rng.byte()).collect();
        if !data.is_empty() && rng.below(4) != 0 {
            data[0] = 0x2f;
        }
        if data.len() >= 5 && rng.below(2) == 0 {
            data[4] &= 0x1f;
        }
        let size = rng.below(data.len() as u64 + 1) as usize;
        command(&mut commands, &mut expected, &data, size);
        count += 1;
    }

    let output = oracle
        .probe("wp7", "info")
        .run::<&str>(&[], commands.as_bytes());
    output.success();
    compare("header_info", &output.stdout, &expected);
    eprintln!("{count} commands compared");
}
