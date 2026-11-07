//! WP10 differential test: `VP8SetSegmentParams` (`src/enc/quant_enc.c`), the
//! quantizers of the segments with their matrices and lambdas, the filter
//! strengths and the merging of equivalent segments, against
//! `tools/oracle/probes/wp10/segments.c`, over seeded susceptibilities,
//! qualities, SNS strengths, the JPEG emulation, the filter settings and the
//! methods.

#[path = "support/oracle.rs"]
mod oracle;
#[path = "support/wp11_images.rs"]
mod wp11_images;

use std::fmt::Write as _;

use oracle::{Fields, Oracle};
use webp::enc::picture_enc::webp_picture_alloc;
use webp::enc::quant_enc::vp8_set_segment_params;
use webp::enc::vp8i_enc::{VP8Matrix, VP8SegmentInfo};
use webp::enc::webp_enc::init_vp8_encoder;
use webp::webp::encode::{WebPConfig, WebPPicture, webp_config_init, webp_picture_init};
use wp11_images::Rng;

struct Record {
    mb_w: i32,
    mb_h: i32,
    method: i32,
    num_segments: i32,
    quality: f32,
    sns_strength: i32,
    emulate_jpeg_size: i32,
    filter_strength: i32,
    filter_type: i32,
    filter_sharpness: i32,
    hdr_sharpness: i32,
    alpha: i32,
    uv_alpha: i32,
    dqm_alpha: [i32; 4],
    dqm_beta: [i32; 4],
    segments: Vec<i32>,
}

impl Record {
    fn write(&self, out: &mut Vec<u8>) {
        let mut values = vec![
            self.mb_w,
            self.mb_h,
            self.method,
            self.num_segments,
            self.quality.to_bits() as i32,
            self.sns_strength,
            self.emulate_jpeg_size,
            self.filter_strength,
            self.filter_type,
            self.filter_sharpness,
            self.hdr_sharpness,
            self.alpha,
            self.uv_alpha,
        ];
        values.extend_from_slice(&self.dqm_alpha);
        values.extend_from_slice(&self.dqm_beta);
        values.extend_from_slice(&self.segments);
        for value in values {
            out.extend_from_slice(&value.to_le_bytes());
        }
    }

    /// Fills an encoder as the probe fills its zeroed `VP8Encoder`, calls the
    /// translation and prints what the probe prints.
    fn run(&self) -> Fields {
        let mut config = WebPConfig::default();
        assert!(webp_config_init(&mut config));
        config.sns_strength = self.sns_strength;
        config.emulate_jpeg_size = self.emulate_jpeg_size;
        config.filter_strength = self.filter_strength;
        config.filter_type = self.filter_type;
        config.filter_sharpness = self.filter_sharpness;
        let mut pic = WebPPicture::default();
        assert!(webp_picture_init(&mut pic));
        pic.width = 16 * self.mb_w;
        pic.height = 16 * self.mb_h;
        webp_picture_alloc(&mut pic).expect("the picture allocates");
        let mut enc = init_vp8_encoder(&config, &mut pic).expect("the encoder initializes");
        // The fields the probe sets; the others of the zeroed C encoder are
        // not read by VP8SetSegmentParams.
        enc.method = self.method;
        enc.segment_hdr.num_segments = self.num_segments;
        enc.filter_hdr.sharpness = self.hdr_sharpness;
        enc.alpha = self.alpha;
        enc.uv_alpha = self.uv_alpha;
        enc.dq_y1_dc = 0;
        enc.dq_y2_dc = 0;
        enc.dq_y2_ac = 0;
        enc.dq_uv_dc = 0;
        enc.dq_uv_ac = 0;
        for i in 0..4 {
            enc.dqm[i] = VP8SegmentInfo {
                alpha: self.dqm_alpha[i],
                beta: self.dqm_beta[i],
                ..VP8SegmentInfo::default()
            };
        }
        for (mb, &segment) in enc.mb_info.iter_mut().zip(&self.segments) {
            mb.segment = segment as u8;
        }

        vp8_set_segment_params(&mut enc, &config, self.quality);

        let e = &enc;
        let encoder = format!(
            "{},{},{},{},{},{},{},{},{},{},{}",
            e.base_quant,
            e.dq_y1_dc,
            e.dq_y2_dc,
            e.dq_y2_ac,
            e.dq_uv_dc,
            e.dq_uv_ac,
            e.segment_hdr.num_segments,
            e.filter_hdr.level,
            e.filter_hdr.simple,
            e.filter_hdr.sharpness,
            e.filter_hdr.i4x4_lf_delta
        );
        let matrix = |text: &mut String, m: &VP8Matrix| {
            for i in 0..16 {
                write!(
                    text,
                    ",{},{},{},{},{}",
                    m.q[i], m.iq[i], m.bias[i], m.zthresh[i], m.sharpen[i]
                )
                .unwrap();
            }
        };
        let mut dqm = String::new();
        for (i, m) in e.dqm.iter().enumerate() {
            write!(
                dqm,
                "{}{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{},{}",
                if i == 0 { "" } else { ";" },
                m.alpha,
                m.beta,
                m.quant,
                m.fstrength,
                m.max_edge,
                m.min_disto,
                m.lambda_i16,
                m.lambda_i4,
                m.lambda_uv,
                m.lambda_mode,
                m.lambda_trellis,
                m.tlambda,
                m.lambda_trellis_i16,
                m.lambda_trellis_i4,
                m.lambda_trellis_uv,
                0,
                m.i4_penalty
            )
            .unwrap();
            matrix(&mut dqm, &m.y1);
            matrix(&mut dqm, &m.y2);
            matrix(&mut dqm, &m.uv);
        }
        let segments = e
            .mb_info
            .iter()
            .map(|mb| mb.segment.to_string())
            .collect::<Vec<_>>()
            .join(",");
        Fields::new()
            .with("encoder", encoder)
            .with("dqm", dqm)
            .with("segments", segments)
    }
}

#[test]
fn segment_parameters_match_the_c_function() {
    let Some(oracle) = Oracle::locate("segment_parameters_match_the_c_function") else {
        return;
    };
    let mut rng = Rng(0x0010_5e60_0001);
    let mut records = Vec::new();
    for index in 0..600 {
        let (mb_w, mb_h) = (1 + rng.below(6) as i32, 1 + rng.below(5) as i32);
        let num_segments = 1 + rng.below(4) as i32;
        let quality = match index % 4 {
            0 => [0.0, 0.5, 25.0, 50.0, 74.9, 75.0, 99.0, 100.0][rng.below(8) as usize],
            _ => rng.below(100_001) as f32 / 1000.0,
        };
        let mut dqm_alpha = [0; 4];
        let mut dqm_beta = [0; 4];
        for i in 0..4 {
            // Equal susceptibilities make equivalent segments, which merge.
            dqm_alpha[i] = if rng.below(4) == 0 && i > 0 {
                dqm_alpha[i - 1]
            } else {
                rng.below(255) as i32 - 127
            };
            dqm_beta[i] = rng.below(256) as i32;
        }
        let segments = (0..mb_w * mb_h)
            .map(|_| rng.below(num_segments as u64) as i32)
            .collect();
        records.push(Record {
            mb_w,
            mb_h,
            method: rng.below(7) as i32,
            num_segments,
            quality,
            sns_strength: [0, 1, 25, 50, 80, 100][rng.below(6) as usize],
            emulate_jpeg_size: rng.below(2) as i32,
            filter_strength: [0, 1, 20, 60, 100][rng.below(5) as usize],
            filter_type: rng.below(2) as i32,
            filter_sharpness: rng.below(8) as i32,
            hdr_sharpness: rng.below(8) as i32,
            alpha: rng.below(256) as i32,
            uv_alpha: rng.below(160) as i32,
            dqm_alpha,
            dqm_beta,
            segments,
        });
    }
    let mut stdin = Vec::new();
    for record in &records {
        record.write(&mut stdin);
    }
    let actual = oracle
        .probe("wp10", "segments")
        .run::<&str>(&[], &stdin)
        .success()
        .fields();
    let mut expected = Fields::new();
    for record in &records {
        expected.0.extend(record.run().0);
    }
    assert_eq!(actual.keys(), expected.keys());
    for (index, ((key, c), (_, rust))) in actual.0.iter().zip(&expected.0).enumerate() {
        assert!(
            c == rust,
            "record {} {key}:\n  C    {}\n  Rust {}",
            index / 3,
            &c[..c.len().min(240)],
            &rust[..rust.len().min(240)]
        );
    }
}
