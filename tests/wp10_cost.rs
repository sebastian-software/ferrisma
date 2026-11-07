//! WP10 differential test: the level costs, the residual costs and statistics,
//! and the token buffer of the VP8 encoder (`src/enc/cost_enc.c`,
//! `src/dsp/cost.c`, `src/enc/token_enc.c`), against
//! `tools/oracle/probes/wp10/cost.c`. Scripts of seeded coefficient blocks run
//! with the default or random probabilities and with statistics that start
//! near the overflow at which `VP8RecordStats` halves them; the token buffers
//! span several pages.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::fmt::Write as _;

use oracle::{Fields, Oracle};
use webp::dsp::cost::{vp8_get_residual_cost, vp8_set_residual_coeffs};
use webp::enc::cost_enc::{vp8_calculate_level_costs, vp8_init_residual, vp8_record_coeffs};
use webp::enc::picture_enc::webp_picture_alloc;
use webp::enc::token_enc::{
    vp8_emit_tokens, vp8_estimate_token_size, vp8_record_coeff_tokens, vp8t_buffer_clear,
    vp8t_buffer_init,
};
use webp::enc::vp8i_enc::{MAX_VARIABLE_LEVEL, VP8Encoder, VP8TBuffer};
use webp::enc::webp_enc::init_vp8_encoder;
use webp::utils::bit_writer_utils::{
    VP8BitWriter, vp8_bit_writer_finish, vp8_bit_writer_init, vp8_bit_writer_size,
};
use webp::webp::encode::{WebPConfig, WebPPicture, webp_config_init, webp_picture_init};
use wp11_images::Rng;

struct Block {
    coeff_type: i32,
    first: i32,
    ctx: i32,
    coeffs: [i16; 16],
}

struct Script {
    probas: Option<Vec<u8>>,
    stats_init: u32,
    page_size: i32,
    blocks: Vec<Block>,
}

impl Script {
    fn write(&self, out: &mut Vec<u8>) {
        let header = [
            i32::from(self.probas.is_some()),
            self.stats_init as i32,
            self.page_size,
            self.blocks.len() as i32,
        ];
        for value in header {
            out.extend_from_slice(&value.to_le_bytes());
        }
        if let Some(probas) = &self.probas {
            out.extend_from_slice(probas);
        }
        for block in &self.blocks {
            for value in [block.coeff_type, block.first, block.ctx] {
                out.extend_from_slice(&value.to_le_bytes());
            }
            for &value in &block.coeffs {
                out.extend_from_slice(&i32::from(value).to_le_bytes());
            }
        }
    }
}

/// An encoder from `InitVP8Encoder`, which calls `VP8DefaultProbas`.
fn encoder() -> Box<VP8Encoder> {
    let mut config = WebPConfig::default();
    assert!(webp_config_init(&mut config));
    let mut pic = WebPPicture::default();
    assert!(webp_picture_init(&mut pic));
    pic.width = 16;
    pic.height = 16;
    webp_picture_alloc(&mut pic).expect("the picture allocates");
    init_vp8_encoder(&config, &mut pic).expect("the encoder initializes")
}

fn stats_hex(enc: &VP8Encoder) -> String {
    let mut text = String::new();
    for value in enc.proba.stats.as_flattened().as_flattened().as_flattened() {
        write!(text, "{value:08x}").unwrap();
    }
    text
}

fn run(script: &Script) -> Fields {
    let mut enc = encoder();
    let mut enc2 = encoder();
    if let Some(probas) = &script.probas {
        let flat = enc
            .proba
            .coeffs
            .as_flattened_mut()
            .as_flattened_mut()
            .as_flattened_mut();
        flat.copy_from_slice(probas);
        enc2.proba.coeffs = enc.proba.coeffs;
    }
    vp8_calculate_level_costs(&mut enc.proba);
    vp8_calculate_level_costs(&mut enc2.proba);
    for value in enc
        .proba
        .stats
        .as_flattened_mut()
        .as_flattened_mut()
        .as_flattened_mut()
    {
        *value = script.stats_init;
    }
    enc2.proba.stats = enc.proba.stats;
    let mut tokens = VP8TBuffer::default();
    vp8t_buffer_init(&mut tokens, script.page_size);

    let mut fields = Fields::new();
    for block in &script.blocks {
        let mut res = vp8_init_residual(block.first, block.coeff_type, &mut enc.proba);
        vp8_set_residual_coeffs(&block.coeffs, &mut res);
        let cost = vp8_get_residual_cost(block.ctx, &res);
        let recorded = vp8_record_coeffs(block.ctx, &mut res);
        let last = res.last;
        let mut res2 = vp8_init_residual(block.first, block.coeff_type, &mut enc2.proba);
        vp8_set_residual_coeffs(&block.coeffs, &mut res2);
        let tokened = vp8_record_coeff_tokens(block.ctx, &mut res2, &mut tokens);
        fields = fields.with("block", format!("{last},{cost},{recorded},{tokened}"));
    }

    let mut level_cost = String::new();
    for value in enc
        .proba
        .level_cost
        .as_flattened()
        .as_flattened()
        .as_flattened()
    {
        write!(level_cost, "{value:04x}").unwrap();
    }
    assert_eq!(
        level_cost.len(),
        4 * 4 * 8 * 3 * (MAX_VARIABLE_LEVEL as usize + 1)
    );
    let remapped = enc
        .proba
        .remapped_costs
        .as_flattened()
        .as_flattened()
        .iter()
        .map(ToString::to_string)
        .collect::<Vec<_>>()
        .join(",");
    let probas = enc2
        .proba
        .coeffs
        .as_flattened()
        .as_flattened()
        .as_flattened()
        .to_vec();
    let estimate = vp8_estimate_token_size(&tokens, &probas);
    let mut bw = VP8BitWriter::default();
    assert!(vp8_bit_writer_init(&mut bw, 0));
    let emit = vp8_emit_tokens(&mut tokens, &mut bw, &probas, true);
    vp8_bit_writer_finish(&mut bw);
    let size = vp8_bit_writer_size(&bw);
    vp8t_buffer_clear(&mut tokens);
    fields
        .with("level_cost", level_cost)
        .with("remapped", remapped)
        .with("stats", stats_hex(&enc))
        .with("token_stats", stats_hex(&enc2))
        .with("estimate", estimate)
        .with("emit", i32::from(emit))
        .hex("emitted", &bw.buf[..size])
}

/// A block of levels: mostly small, sometimes up to the largest category.
fn block(rng: &mut Rng) -> Block {
    let coeff_type = rng.below(4) as i32;
    let first = i32::from(coeff_type == 0);
    let mut coeffs = [0_i16; 16];
    let density = rng.below(17);
    for (n, value) in coeffs.iter_mut().enumerate() {
        if (n as i32) < first || rng.below(16) >= density {
            continue;
        }
        let magnitude = match rng.below(10) {
            0..=5 => 1 + rng.below(2),
            6 | 7 => 3 + rng.below(8),
            8 => 11 + rng.below(56),
            _ => 67 + rng.below(1981),
        } as i16;
        *value = if rng.below(2) == 0 {
            magnitude
        } else {
            -magnitude
        };
    }
    Block {
        coeff_type,
        first,
        ctx: rng.below(3) as i32,
        coeffs,
    }
}

#[test]
fn residual_costs_and_tokens_match_the_c_functions() {
    let Some(oracle) = Oracle::locate("residual_costs_and_tokens_match_the_c_functions") else {
        return;
    };
    let mut rng = Rng(0x0010_c057_0001);
    let mut scripts = Vec::new();
    for &(random, stats_init, page_size, count) in &[
        (false, 0_u32, 0, 40),
        (true, 0, 9000, 300),
        (false, 0xfffd_fff0, 8192, 200),
        (true, 0xfffe_0000, 8192, 120),
        (true, 0x7fff_7fff, 20_000, 1500),
        (false, 0, 8192, 2000),
    ] {
        let probas = random.then(|| (0..1056).map(|_| rng.below(256) as u8).collect());
        let blocks = (0..count).map(|_| block(&mut rng)).collect();
        scripts.push(Script {
            probas,
            stats_init,
            page_size,
            blocks,
        });
    }
    let mut stdin = Vec::new();
    for script in &scripts {
        script.write(&mut stdin);
    }
    let actual = oracle
        .probe("wp10", "cost")
        .run::<&str>(&[], &stdin)
        .success()
        .fields();
    let mut expected = Fields::new();
    for script in &scripts {
        expected.0.extend(run(script).0);
    }
    assert_eq!(actual.keys(), expected.keys());
    for (index, ((key, c), (_, rust))) in actual.0.iter().zip(&expected.0).enumerate() {
        assert!(
            c == rust,
            "field {index} {key}:\n  C    {}\n  Rust {}",
            &c[..c.len().min(200)],
            &rust[..rust.len().min(200)]
        );
    }
}
