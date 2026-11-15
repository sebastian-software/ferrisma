pub mod context_predict;
pub mod dec_ma;
pub mod enc_encoding;
pub mod enc_ma;
#[expect(clippy::module_inception, reason = "Preserve the upstream source path")]
pub mod encoding;
pub mod ma_common;
// encoder entropy: the encoder's SIMD FindBestSplit (port ADR 0004,
// "Encoder path").
pub mod enc_ma_simd;
// end encoder entropy
