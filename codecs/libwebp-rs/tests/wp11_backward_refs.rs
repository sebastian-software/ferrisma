//! WP11 differential test: hash chains, backward references and the histogram
//! image (`VP8LHashChainFill`, `VP8LGetBackwardReferences` with
//! `VP8LBackwardReferencesTraceBackwards`, and `VP8LGetHistoImageSymbols`, from
//! `src/enc/backward_references_enc.c`, `backward_references_cost_enc.c` and
//! `histogram_enc.c`). The C probe `tools/oracle/probes/wp11/backward_refs.c`
//! and the Rust translation run on the same seeded pixels; the hash chain, the
//! return values, the error code, the progress, the chosen cache bits, every
//! reference, the histogram symbols and the costs of every clustered histogram
//! are compared. The inputs cover every quality band of the window and
//! iteration heuristics, low effort, the RLE, standard and box LZ77 and their
//! combinations, every maximum cache size, both no-cache modes, histogram bits
//! from 2 to 9, runs longer than `MAX_LENGTH` and images up to 6,000 pixels.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::fmt::Write as _;

use oracle::{Fields, Oracle};
use webp::dsp::lossless_common::vp8l_sub_sample_size;
use webp::enc::backward_references_enc::{
    MAX_REFS_BLOCK_PER_IMAGE, VP8LBackwardRefs, VP8LHashChain, vp8l_backward_refs_init,
    vp8l_get_backward_references, vp8l_hash_chain_fill, vp8l_hash_chain_init,
    vp8l_refs_cursor_init, vp8l_refs_cursor_next, vp8l_refs_cursor_ok,
};
use webp::enc::histogram_enc::{
    vp8l_allocate_histogram, vp8l_allocate_histogram_set, vp8l_get_histo_image_symbols,
    vp8l_histogram_estimate_bits,
};
use webp::webp::encode::{WebPPicture, webp_picture_init};
use wp11_images::{Kind, Rng, image};

struct Command {
    width: i32,
    height: i32,
    quality: i32,
    low_effort: bool,
    lz77: u32,
    cache_bits: i32,
    do_no_cache: bool,
    histo_bits: i32,
    argb: Vec<u32>,
}

fn hex32(values: &[u32]) -> String {
    if values.is_empty() {
        return "-".to_owned();
    }
    values.iter().map(|value| format!("{value:08x}")).collect()
}

fn refs_line(refs: &VP8LBackwardRefs) -> String {
    let mut line = String::new();
    let mut c = vp8l_refs_cursor_init(refs);
    while vp8l_refs_cursor_ok(&c) {
        let v = c.get(refs);
        write!(
            line,
            "{:02x}{:04x}{:08x}",
            v.mode, v.len, v.argb_or_distance
        )
        .unwrap();
        vp8l_refs_cursor_next(&mut c, refs);
    }
    if line.is_empty() {
        line.push('-');
    }
    line
}

fn run(command: &Command) -> Fields {
    let size = command.width * command.height;
    let mut picture = WebPPicture::default();
    assert!(webp_picture_init(&mut picture));
    let mut chain = VP8LHashChain::default();
    assert!(vp8l_hash_chain_init(&mut chain, size));
    let mut refs: [VP8LBackwardRefs; 3] = Default::default();
    for r in &mut refs {
        vp8l_backward_refs_init(r, (size - 1) / MAX_REFS_BLOCK_PER_IMAGE + 1);
    }
    let mut percent = 0;
    let fill = vp8l_hash_chain_fill(
        &mut chain,
        command.quality,
        &command.argb,
        command.width,
        command.height,
        command.low_effort,
        &mut picture,
        30,
        &mut percent,
    );
    let mut fields = Fields::new()
        .with("fill", i32::from(fill.is_ok()))
        .with("chain", hex32(&chain.offset_length));
    let mut cache_bits_best = -1;
    let ret = vp8l_get_backward_references(
        command.width,
        command.height,
        &command.argb,
        command.quality,
        command.low_effort,
        command.lz77,
        command.cache_bits,
        command.do_no_cache,
        &chain,
        &mut refs,
        &mut cache_bits_best,
        &mut picture,
        30,
        &mut percent,
    );
    fields = fields
        .with("refs_ret", i32::from(ret.is_ok()))
        .with("error", picture.error_code.0)
        .with("cache_bits", cache_bits_best)
        .with("percent", percent)
        .with("refs0", refs_line(&refs[0]));
    if command.do_no_cache {
        fields = fields.with("refs1", refs_line(&refs[1]));
    }
    let histo_xsize = vp8l_sub_sample_size(
        command.width.cast_unsigned(),
        command.histo_bits.cast_unsigned(),
    );
    let histo_ysize = vp8l_sub_sample_size(
        command.height.cast_unsigned(),
        command.histo_bits.cast_unsigned(),
    );
    let xysize = (histo_xsize * histo_ysize) as i32;
    let mut set = vp8l_allocate_histogram_set(xysize, cache_bits_best).unwrap();
    let mut tmp = vp8l_allocate_histogram(cache_bits_best).unwrap();
    let mut symbols = vec![0_u32; xysize as usize];
    let ret = vp8l_get_histo_image_symbols(
        command.width,
        command.height,
        &refs[0],
        command.quality,
        command.low_effort,
        command.histo_bits,
        cache_bits_best,
        &mut set,
        &mut tmp,
        &mut symbols,
        &mut picture,
        30,
        &mut percent,
    );
    fields = fields
        .with("histo_ret", i32::from(ret.is_ok()))
        .with("histo_size", set.size)
        .with("symbols", hex32(&symbols));
    for i in 0..set.size as usize {
        let h = set.histogram(i);
        let mut costs = format!("{:016x}", h.bit_cost);
        for cost in h.costs {
            write!(costs, "{cost:016x}").unwrap();
        }
        fields = fields.with("costs", costs).with(
            "estimate",
            format!("{:016x}", vp8l_histogram_estimate_bits(h)),
        );
    }
    fields
}

fn commands() -> Vec<Command> {
    let mut rng = Rng(0xbac4_ef11);
    let mut commands = Vec::new();
    let sizes: [(i32, i32); 12] = [
        (1, 1),
        (2, 1),
        (1, 3),
        (3, 3),
        (7, 5),
        (16, 9),
        (23, 31),
        (40, 40),
        (64, 32),
        (100, 60),
        (5, 900),
        (77, 50),
    ];
    let qualities = [0, 10, 25, 26, 40, 50, 51, 60, 75, 76, 90, 100];
    for (index, &(width, height)) in sizes.iter().enumerate() {
        for (k, &kind) in Kind::ALL.iter().enumerate() {
            let repeats = if width * height > 3000 { 1 } else { 2 };
            for repeat in 0..repeats {
                let argb = image(
                    &mut rng,
                    kind,
                    width as usize,
                    height as usize,
                    width as usize,
                );
                let lz77 = [1, 2, 3, 4, 5, 6, 7][rng.below(7) as usize];
                commands.push(Command {
                    width,
                    height,
                    quality: qualities[(index + k + repeat * 5) % qualities.len()],
                    low_effort: rng.below(4) == 0,
                    lz77,
                    cache_bits: rng.below(11) as i32,
                    do_no_cache: rng.below(2) == 0,
                    histo_bits: 2 + rng.below(8) as i32,
                    argb,
                });
            }
        }
    }
    commands
}

#[test]
fn backward_references_and_histogram_image_match_the_c_functions() {
    let Some(oracle) =
        Oracle::locate("backward_references_and_histogram_image_match_the_c_functions")
    else {
        return;
    };
    let commands = commands();
    let mut script = String::new();
    for c in &commands {
        writeln!(
            script,
            "refs {} {} {} {} {} {} {} {} {}",
            c.width,
            c.height,
            c.quality,
            i32::from(c.low_effort),
            c.lz77,
            c.cache_bits,
            i32::from(c.do_no_cache),
            c.histo_bits,
            hex32(&c.argb)
        )
        .unwrap();
    }
    let output = oracle
        .probe("wp11", "backward_refs")
        .run::<&str>(&[], script.as_bytes());
    let actual = output.success().fields();
    let mut offset = 0;
    for (index, command) in commands.iter().enumerate() {
        let expected = run(command);
        let got = &actual.0[offset..offset + expected.0.len()];
        for ((key, want), (got_key, got_value)) in expected.0.iter().zip(got) {
            assert_eq!(key, got_key, "command {index}");
            assert!(
                want == got_value,
                "command {index} ({}x{} q{} low {} lz77 {} cache {} no_cache {} histo {}): \
                 field {key}\n  C    {}\n  Rust {}",
                command.width,
                command.height,
                command.quality,
                command.low_effort,
                command.lz77,
                command.cache_bits,
                command.do_no_cache,
                command.histo_bits,
                got_value.chars().take(200).collect::<String>(),
                want.chars().take(200).collect::<String>()
            );
        }
        offset += expected.0.len();
    }
    assert_eq!(offset, actual.0.len(), "the probe printed more fields");
}
