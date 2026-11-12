# prof/webpenc-port.json.gz: total CPU weight 3277350 (thread CPU delta units, us)

## Self (top 30)
| % | function |
| ---: | --- |
| 14.2 | `webp::dsp::enc_neon::load4x4_neon` |
| 9.4 | `webp::dec::vp8l_dec::decode_image_data` |
| 8.0 | `webp::dsp::cost_neon::get_residual_cost` |
| 7.4 | `webp::dsp::enc_neon::disto4x4_neon` |
| 5.0 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 4.8 | `webp::enc::token_enc::vp8_emit_tokens` |
| 4.8 | `webp::enc::quant_enc::vp8_decimate` |
| 4.6 | `webp::dec::vp8l_dec::read_symbol` |
| 4.1 | `webp::dsp::enc_neon::f_transform_neon` |
| 3.9 | `ferrisma::fast_execute::quantize_row::<3>` |
| 3.2 | `webp::dsp::cpu::features` |
| 3.1 | `webp::dsp::lossless_neon::predictor_add11` |
| 2.9 | `webp::dsp::enc_neon::quantize_block_neon` |
| 2.6 | `webp::enc::picture_csp_enc::sum4` |
| 2.3 | `webp::dsp::enc::vp8_collect_histogram` |
| 1.8 | `webp::enc::token_enc::vp8_record_coeff_tokens` |
| 1.5 | `webp::enc::quant_enc::reconstruct_intra4` |
| 1.5 | `webp::dsp::enc::vp8_enc_quantize2_blocks` |
| 1.1 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 1.1 | `webp::enc::picture_csp_enc::convert_row_to_y` |
| 0.9 | `__bzero` |
| 0.8 | `webp::dsp::enc::vp8_set_histogram_data` |
| 0.7 | `webp::enc::picture_csp_enc::linear_to_gamma` |
| 0.6 | `webp::enc::tree_enc::vp8_code_intra_modes` |
| 0.6 | `_platform_memmove` |
| 0.6 | `webp::dsp::enc::true_motion` |
| 0.6 | `webp::utils::bit_reader_utils::vp8l_read_bits` |
| 0.6 | `webp::enc::quant_enc::reconstruct_uv` |
| 0.6 | `webp::dsp::lossless_neon::convert_bgra_to_rgb` |
| 0.5 | `webp::enc::cost_enc::vp8_get_cost_luma16` |

## Inclusive (top 30)
| % | function |
| ---: | --- |
| 100.0 | `ferrisma::cli::run` |
| 100.0 | `ferrisma::main` |
| 100.0 | `std::sys::backtrace::__rust_begin_short_backtrace::<fn() -> result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>, result::Result<std::process::ExitCode, boxed::Box<dyn error::Error>>>` |
| 100.0 | `std::rt::lang_start::<result::Result<..>>::{closure#0}` |
| 100.0 | `std::rt::lang_start_internal` |
| 100.0 | `main` |
| 100.0 | `start` |
| 99.8 | `ferrisma::pipeline::run` |
| 73.7 | `<ferrisma::webp_sink::WebpSink>::finish` |
| 73.7 | `<ferrisma::pipeline::Sink>::finish` |
| 73.7 | `webp::enc::webp_enc::webp_encode` |
| 73.7 | `<webp::api::encode::Picture>::encode_with` |
| 61.2 | `webp::enc::frame_enc::vp8_enc_token_loop` |
| 53.5 | `webp::enc::quant_enc::vp8_decimate` |
| 25.8 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 25.7 | `<ferrisma::codecs::webp::WebpSource as ferrisma::decoder::DecodeSource>::read_region` |
| 19.4 | `webp::dec::vp8l_dec::vp8l_decode_image` |
| 19.4 | `webp::dec::webp_dec::decode_into` |
| 19.4 | `webp::dec::webp_dec::webp_decode` |
| 19.4 | `webp::api::decode::decode_into` |
| 19.1 | `webp::dec::vp8l_dec::decode_image_data` |
| 15.9 | `webp::enc::quant_enc::reconstruct_intra4` |
| 14.2 | `webp::dsp::enc_neon::load4x4_neon` |
| 12.3 | `webp::dsp::enc_neon::f_transform_neon` |
| 8.0 | `webp::dsp::cost_neon::get_residual_cost` |
| 7.4 | `webp::dsp::enc_neon::disto4x4_neon` |
| 6.5 | `webp::enc::analysis_enc::do_segments_job` |
| 6.5 | `webp::enc::analysis_enc::vp8_enc_analyze` |
| 6.2 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |
| 5.5 | `webp::dsp::enc::vp8_collect_histogram` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| webp-encode | 64.8 | `webp::dsp::enc_neon::load4x4_neon` 14.2; `webp::dsp::cost_neon::get_residual_cost` 8.0; `webp::dsp::enc_neon::disto4x4_neon` 7.4; `webp::dsp::enc_neon::i_transform_one_neon` 5.0 |
| webp-decode | 23.0 | `webp::dec::vp8l_dec::decode_image_data` 9.4; `webp::dec::vp8l_dec::read_symbol` 4.6; `webp::dsp::cpu::features` 3.2; `webp::dsp::lossless_neon::predictor_add11` 3.1 |
| rgb2yuv | 5.3 | `webp::enc::picture_csp_enc::sum4` 2.6; `webp::enc::picture_csp_enc::convert_row_to_y` 1.1; `webp::enc::picture_csp_enc::linear_to_gamma` 0.7; `webp::enc::picture_csp_enc::accumulate_rgb` 0.4 |
| quantize | 3.9 | `ferrisma::fast_execute::quantize_row::<3>` 3.9 |
| resize | 1.5 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 1.1; `ferrisma_kernels::dispatch::neon::axpy` 0.3; `__sysctl` 0.0 |
| engine-pipeline | 1.1 | `__bzero` 0.5; `<ferrisma::pipeline::Sink>::write_row` 0.3; `read` 0.2; `mach_absolute_time` 0.0 |
| linearize | 0.5 | `ferrisma::fast_execute::source_row_to_linear::<3>` 0.5 |
