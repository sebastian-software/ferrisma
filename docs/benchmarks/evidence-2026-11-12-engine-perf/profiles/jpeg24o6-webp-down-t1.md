# prof/jpeg24o6-webp-down-t1.json.gz: total CPU weight 533306 (thread CPU delta units, us)

## Self (top 30)
| % | function |
| ---: | --- |
| 15.7 | `ferrisma::orientation::gather_row::<u8>` |
| 13.7 | `jpeg::jdhuff::decode_mcu` |
| 11.0 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 6.1 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 5.2 | `webp::dsp::enc_neon::load4x4_neon` |
| 4.1 | `webp::dsp::cost_neon::get_residual_cost` |
| 3.7 | `webp::enc::token_enc::vp8_emit_tokens` |
| 3.7 | `ferrisma::fast_execute::source_row_to_linear::<3>` |
| 3.0 | `webp::dsp::enc_neon::disto4x4_neon` |
| 2.4 | `ferrisma_kernels::dispatch::neon::axpy` |
| 2.3 | `webp::enc::quant_enc::vp8_decimate` |
| 2.3 | `ferrisma::fast_execute::quantize_row::<3>` |
| 2.0 | `webp::dsp::enc_neon::f_transform_neon` |
| 2.0 | `jpeg::jidctint_neon::jsimd_idct_islow_neon` |
| 1.9 | `webp::dsp::enc_neon::i_transform_one_neon` |
| 1.7 | `webp::dsp::cpu::features` |
| 1.5 | `webp::enc::token_enc::vp8_record_coeff_tokens` |
| 1.4 | `__bzero` |
| 1.2 | `webp::dsp::enc::vp8_collect_histogram` |
| 1.2 | `webp::enc::picture_csp_enc::sum4` |
| 1.1 | `webp::dsp::enc_neon::quantize_block_neon` |
| 1.1 | `std::sys::env::unix::env_read_lock` |
| 0.9 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_regular` |
| 0.9 | `jpeg::jdsample_neon::jsimd_h2v2_fancy_upsample_neon` |
| 0.9 | `jpeg::jdcolext_neon::jsimd_ycc_rgb_convert_neon::<0, 1, 2, -1, 3>` |
| 0.8 | `webp::enc::quant_enc::reconstruct_intra4` |
| 0.7 | `_platform_memmove` |
| 0.6 | `jpeg::jidctint_neon::jsimd_idct_islow_pass2_sparse` |
| 0.5 | `_platform_memset` |
| 0.4 | `webp::enc::picture_csp_enc::convert_row_to_y` |

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
| 99.9 | `ferrisma::pipeline::run` |
| 63.6 | `ferrisma::fast_execute::execute_fast_srgb8::<ferrisma::pipeline::run::{closure#1}>` |
| 63.0 | `<ferrisma::orientation::OrientedSource as ferrisma::decoder::DecodeSource>::read_region` |
| 35.1 | `webp::enc::webp_enc::webp_encode` |
| 35.1 | `<webp::api::encode::Picture>::encode_with` |
| 35.1 | `<ferrisma::webp_sink::WebpSink>::finish` |
| 35.1 | `<ferrisma::pipeline::Sink>::finish` |
| 29.4 | `webp::enc::frame_enc::vp8_enc_token_loop` |
| 26.5 | `<ferrisma::codecs::jpeg::JpegSource as ferrisma::decoder::DecodeSource>::read_region` |
| 25.8 | `<jpeg::decode::Scanlines>::read_rows` |
| 25.8 | `jpeg::jdmainct::process_data_context_main` |
| 25.8 | `jpeg::jdapistd::jpeg_read_scanlines` |
| 24.0 | `jpeg::jdcoefct::decompress_onepass` |
| 23.5 | `webp::enc::quant_enc::vp8_decimate` |
| 19.8 | `ferrisma::fast_execute::run_sequential::<3, ferrisma::pipeline::run::{closure#1}>::{closure#0}` |
| 19.8 | `jpeg::jdhuff::decode_mcu` |
| 15.7 | `ferrisma::orientation::gather_row::<u8>` |
| 11.0 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` |
| 11.0 | `<ferrisma_kernels::Kernels>::horizontal_rgba` |
| 6.2 | `webp::enc::quant_enc::reconstruct_intra4` |
| 6.1 | `jpeg::jdhuff::HUFF_DECODE_FAST` |
| 5.2 | `webp::dsp::enc_neon::load4x4_neon` |
| 4.6 | `webp::dsp::enc_neon::f_transform_neon` |

## Stage shares (leaf-to-root first match)
| stage | % CPU | top self functions in stage |
| --- | ---: | --- |
| webp-encode | 31.0 | `webp::dsp::enc_neon::load4x4_neon` 5.2; `webp::dsp::cost_neon::get_residual_cost` 4.1; `webp::enc::token_enc::vp8_emit_tokens` 3.7; `webp::dsp::enc_neon::disto4x4_neon` 3.0 |
| jpeg-decode | 27.0 | `jpeg::jdhuff::decode_mcu` 13.7; `jpeg::jdhuff::HUFF_DECODE_FAST` 6.1; `jpeg::jidctint_neon::jsimd_idct_islow_neon` 2.0; `std::sys::env::unix::env_read_lock` 1.1 |
| orientation | 16.3 | `ferrisma::orientation::gather_row::<u8>` 15.7; `_platform_memmove` 0.6 |
| resize | 13.4 | `ferrisma_kernels::dispatch::neon::horizontal_rgba` 11.0; `ferrisma_kernels::dispatch::neon::axpy` 2.4; `<ferrisma_kernels::Kernels>::axpy` 0.0 |
| linearize | 3.7 | `ferrisma::fast_execute::source_row_to_linear::<3>` 3.7 |
| rgb2yuv | 2.5 | `webp::enc::picture_csp_enc::sum4` 1.2; `webp::enc::picture_csp_enc::convert_row_to_y` 0.4; `webp::enc::picture_csp_enc::linear_to_gamma` 0.4; `webp::enc::picture_csp_enc::accumulate_rgb` 0.2 |
| engine-pipeline | 2.3 | `__bzero` 1.2; `<ferrisma::pipeline::Sink>::write_row` 0.2; `libsystem_m.dylib+?` 0.2; `mach_absolute_time` 0.1 |
| quantize | 2.3 | `ferrisma::fast_execute::quantize_row::<3>` 2.3 |
| webp-decode | 1.7 | `webp::dsp::cpu::features` 1.7; `webp::utils::bit_writer_utils::flush` 0.0 |
