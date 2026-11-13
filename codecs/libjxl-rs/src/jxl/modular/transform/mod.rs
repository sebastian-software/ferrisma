pub mod enc_palette;
pub mod enc_rct;
pub mod enc_squeeze;
pub mod enc_transform;
pub mod palette;
pub mod rct;
pub mod squeeze;
pub mod squeeze_params;
#[expect(
    clippy::module_inception,
    reason = "Preserve the upstream modular/transform/transform file boundary."
)]
pub mod transform;
// modular: the SIMD inverse transforms (port ADR 0004, Plan item 2).
pub mod rct_simd;
// end modular
